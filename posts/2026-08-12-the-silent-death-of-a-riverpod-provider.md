I used to think of Riverpod like a service locator with better manners.

Need a repository? `ref.read(repoProvider)`.

Need an async operation? `ref.read(uploadProvider(...).future)`.

Got the object. Job done. Move on.

That mental model worked for a long time. Then Riverpod 3 (and stricter autoDispose + lifecycle checks) turned that habit into a debugging nightmare. I spent days chasing errors that looked like this:

```
Cannot use the Ref of uploadOnboardingDocumentProvider(...) after it has been disposed.
This typically happens if:
- A provider rebuilt, but the previous "build" was still pending and is still performing operations.
  You should therefore either use `ref.onDispose` to cancel pending work, or
  check `ref.mounted` after async gaps or anything that could invalidate the provider.
- You tried to use Ref inside `onDispose` or other life-cycles.
  This is not supported, as the provider is already being disposed.
```

If you’ve seen that message and felt personally attacked: this post is for you.

The short version:

> **`ref.read` gives you the value. It does not keep the provider alive.**
>
> **`ref.watch` / `listen` keeps the provider alive.**
>
> If your provider is `autoDispose`, and nothing is listening, the box can close while your async work is still running. Then any later use of that provider’s `Ref` explodes.

That distinction sounds small. In a real upload pipeline with family providers, blob URLs, and TaskEither chains, it is not small at all.

---

## The problem I was solving

We were building courier onboarding. One step required uploading documents:

1. Cache the picked file locally
2. Ask the API for an upload session (returns `documentId` + SAS `uploadUrl`)
3. PUT the bytes to Azure Blob Storage (Azurite locally)
4. Call `/complete` so the backend sets `UploadedAtUtc`

Architecturally, this lived in a Riverpod-native flow:

- Screen → Mutation → Controller → `@riverpod` async upload provider → repositories

The upload provider was (effectively) an **autoDispose family**:

```dart
@riverpod
Future<Either<Failure, String>> uploadOnboardingDocument(
  Ref ref,
  UploadOnboardingDocumentParams params,
) async {
  // store file
  // create upload session
  final blobRepo = ref.read(
    supplierApplicationBlobRepositoryProvider(session.uploadUrl),
  );
  // upload bytes
  // complete upload
}
```

From the controller, the “obvious” call looked like:

```dart
final result = await ref.read(
  uploadOnboardingDocumentProvider(params).future,
);
```

That line is the trap.

---

## What I thought `ref.read` meant

My mental model:

> “`read` just fetches the current value / starts the future. I’m not subscribing because I don’t need rebuilds. Perfect for one-off actions.”

And that *is* partly true.

- `ref.read(provider)` returns the current state.
- `ref.read(provider.future)` returns the `Future`.
- For one-off UI actions, people often prefer `read`/`refresh` so widgets don’t rebuild around a subscription.

But there is a second sentence that belongs next to that advice, and I had not internalized it hard enough:

> **Reading does not create a long-lived dependency.**
>
> When the temporary listener created by `read(...future)` is done (or when nothing else is watching), an `autoDispose` provider can dispose.

So yes: you get the object.

Then the box that produced that object can disappear.

If that object (or the provider body itself) still needs `ref` after an await — you’re now holding a ticket to a closed venue.

---

## The lifecycle picture that finally clicked

Think of a provider as a **box**.

```
┌─────────────────────────────┐
│ uploadOnboardingDocument(…) │
│                             │
│  owns a Ref                 │
│  can read other providers   │
│  runs async work            │
└─────────────────────────────┘
```

That box stays open while something cares about it:

- a widget `ref.watch`es it
- another provider `ref.watch`es it
- a `listen` is active
- or you explicitly keep it alive (`keepAlive` / link)

`ref.read` is more like:

> “Open the box, take what I need, close the door behind me unless someone else is still inside.”

For sync values that never use `Ref` again, fine.

For an async provider that:

1. awaits store
2. awaits create-session
3. **then** `ref.read(familyProvider(uploadUrl))`
4. awaits blob upload
5. awaits complete

…step 3 and beyond are exactly where disposal hurts.

That was my case. The family blob repository depended on `uploadUrl` from the session response. I couldn’t inject it up front. I had to resolve it *after* an async gap — using `ref` again.

If the upload provider had already been disposed because nothing was watching it, boom:

`Cannot use the Ref of uploadOnboardingDocumentProvider(...) after it has been disposed.`

---

## Why this is especially nasty with Tasks / Either chains

We use `fpdart` `TaskEither` a lot. It’s elegant:

```dart
storeFile()
  .flatMap((path) => createSession())
  .flatMap((session) => uploadBlob(session))
  .flatMap((session) => complete(session));
```

It’s also a disposal amplifier.

Why?

Because the “business logic” looks synchronous in structure, but is asynchronous in time. The chain can cross multiple awaits. Each await is a chance for:

- autoDispose to kick in
- a parent provider to rebuild
- a screen to navigate away
- a Mutation listener to drop

And when the failure finally surfaces, the stack trace points at the `ref.read(...)` *inside* the chain — not at the controller line that started everything with `ref.read(...future)`.

So you debug the leaf. You add `ref.mounted`. You add `keepAlive`. You question your architecture. You rewrite repositories. You pass `Ref` deeper. You make it worse.

The root cause was earlier:

> **You started a long-lived async provider as if it were a fire-and-forget function call.**

It isn’t. It’s a stateful box with a `Ref` tied to its lifetime.

---

## The fix that actually worked: `ref.watch` on the future

In the controller, the durable fix was not “more careful null checks.” It was:

```dart
final result = await ref.watch(
  uploadOnboardingDocumentProvider(params).future,
);
```

That feels wrong the first time you write it.

“Watch? I’m not building UI from this. This is a one-off upload.”

But in Riverpod terms, `watch` here means:

> **While this awaited future is part of my dependency graph / active computation, keep that provider alive.**

You’re not watching because you want rebuild spam. You’re watching because you need the provider’s lifetime to cover the whole async operation — including later `ref.read`s inside that provider after awaits.

That matched our team rule of thumb once we wrote it down:

- **Liveness** comes from `watch` / `listen` (or `keepAlive`), not from `read`
- **Re-run** comes from `invalidate` / `refresh`, not from calling `read` again
- **Provider-to-provider async**: prefer `await ref.watch(otherProvider.future)`
- **UI one-offs**: often `await ref.refresh(opProvider.future)` — but only if you understand the lifetime implications

For our autoDispose upload family, `watch(...future)` was the pragmatic, Riverpod-native answer.

---

## Related anti-pattern: passing `Ref` into repositories

While we were deep in this, I also revisited an old Riverpod discussion pattern:

**Option A:** inject concrete dependencies into the repository

**Option B:** pass `Ref` into the repository and `ref.read` inside methods

Option B used to feel modern. In Riverpod 3, with autoDispose and family providers created mid-flow (like “now that I have an upload URL, give me a blob client”), Option B becomes a footgun factory.

Why?

Because a stored `Ref` is not a global service locator token. It is scoped to the provider instance that created it. If that provider disposes while your repository method is still awaiting, later `ref.read` calls fail in spectacular ways.

My rule now:

> **Don’t pass `Ref` into repositories/services.**
>
> Resolve dependencies in the provider factory with `ref.watch`, inject plain objects.
>
> Keep `Ref` inside `@riverpod` notifiers/providers.
>
> After async gaps, respect `ref.mounted` if you must continue.

And don’t paper over disposal with `keepAlive` unless you intentionally want extended lifetime. `keepAlive` is a sharp tool. It can hide the real dependency mistake and create “why is this still alive?” bugs later.

---

## A concrete timeline of the bug

Here’s the failure mode in slow motion:

1. Controller calls `ref.read(uploadProvider(params).future)`
2. Upload provider starts
3. Temporary interest in that provider exists just long enough to attach to the future
4. Provider awaits “create upload session”
5. During/after that gap, if nothing else is watching, autoDispose can dispose the provider
6. Session returns with `uploadUrl`
7. Provider does `ref.read(blobRepoProvider(uploadUrl))`
8. 💥 `Ref` is disposed

Meanwhile Azure/Azurite may already have the file (or a partial attempt), the DB may have a pending document row (`UploadedAtUtc = null`), and your admin portal shows an empty Documents list because it only lists completed uploads.

So the product symptom (“files are in storage but portal is empty”) and the framework symptom (“Ref disposed”) were cousins. Same family of “async pipeline completed only halfway because lifetime assumptions were wrong.”

---

## What the docs say vs what production teaches

Riverpod docs are careful about `read` vs `watch`. Community advice often says:

- use `watch` to react to changes
- use `read` in callbacks / one-off actions

That advice is correct for UI event handlers updating a notifier.

It is incomplete for:

- autoDispose async providers
- family providers created after an await
- long Task/Either pipelines
- “get me `.future` and then continue using `ref` inside that provider”

Production teaches a sharper rule:

> **If an async provider needs its own `Ref` after an await, something must keep that provider alive for the duration of the work.**

That “something” is usually:

- `ref.watch(provider.future)` from a parent that itself is alive, or
- a deliberate `keepAlive` strategy with cancellation discipline, or
- redesign so post-await dependencies don’t need `Ref` (inject what you can earlier)

We chose watch. It was the smallest change that matched Riverpod’s model instead of fighting it.

---

## How I debug this class of bug now

When I see disposed `Ref` errors, I ask:

1. **Which provider owns the Ref in the stack trace?**
2. **Is it autoDispose?** (codegen `isAutoDispose`, annotations)
3. **Who is the root listener?** Widget watch? Parent provider watch? Mutation? Container listen?
4. **Did we start it with `read(...future)`?**
5. **Is there an await before another `ref.read/watch`?**
6. **Are we resolving a family provider only after receiving runtime data (URL, id)?**

If yes to 4–6, I don’t start with `keepAlive`. I first try:

```dart
await ref.watch(thatProvider.future);
```

from a scope that remains alive for the operation.

I also log steps aggressively in the pipeline (store → session → blob → complete), because “upload succeeded” in the UI often meant “local filename set” or “blob landed,” not “portal-visible complete.”

---

## The mental model I want tattooed on my eyelids

| API | Gives you the value? | Keeps provider alive? | Good for |
| --- | --- | --- | --- |
| `ref.read` | Yes | No (not as a long-lived dependency) | Sync lookups, event handlers that don’t need lifetime |
| `ref.watch` | Yes | Yes | Dependencies, and async work whose provider must stay alive |
| `ref.listen` | Via callbacks | Yes | Side effects without rebuilds |
| `ref.refresh` / `invalidate` | Triggers recompute | Not the same as “keep alive” | Re-running work |

And the slogan:

> **`read` returns the object. `watch` rents the box.
> Async work that still needs `Ref` needs the box to stay rented.**

---

## Closing

I didn’t lose days because Riverpod is “too magical.” I lost days because I treated providers like functions and `Ref` like a free service locator handle.

They aren’t.

A provider is a lifecycle-bound unit of work. Its `Ref` dies with it. AutoDispose makes that death eager. Family providers created mid-flight make the dependency graph dynamic. Task chains make the awaits longer. Together, they punish the old “just `read` it” reflex.

The fix was almost embarrassingly small: **watch the future so the provider stays alive while the future is the thing you care about.**

If this post saves you one afternoon of staring at a disposed `Ref` stack while Azurite cheerfully shows files that your portal refuses to list — then my suffering was worth something.

And if you take only one line into your next PR:

> **Don’t start long autoDispose async work with `ref.read(...future)` if that work needs `ref` again after an await. Use `ref.watch(...future)` (or an explicit lifetime strategy).**

Your future self — and your Documents tab — will thank you.
