#[cfg(feature = "ssr")]
use std::{
    collections::HashMap,
    net::IpAddr,
    sync::Mutex,
    time::{Duration, Instant},
};

/// Very small in-memory IP rate limiter.
///
/// Notes:
/// - Per-process only (does not share limits across multiple server instances).
/// - Uses a simple sliding window by storing recent timestamps per IP.
#[cfg(feature = "ssr")]
pub struct IpRateLimiter {
    window: Duration,
    max_requests: usize,
    state: Mutex<HashMap<IpAddr, Vec<Instant>>>,
}

#[cfg(feature = "ssr")]
impl IpRateLimiter {
    pub fn new(window: Duration, max_requests: usize) -> Self {
        Self {
            window,
            max_requests,
            state: Mutex::new(HashMap::new()),
        }
    }

    /// Returns `true` if allowed, `false` if rate-limited.
    pub fn allow(&self, ip: IpAddr) -> bool {
        let now = Instant::now();
        let cutoff = now.checked_sub(self.window).unwrap_or(now);

        let mut state = self
            .state
            .lock()
            .expect("IpRateLimiter mutex poisoned");

        let entries = state.entry(ip).or_default();
        entries.retain(|t| *t >= cutoff);

        if entries.len() >= self.max_requests {
            return false;
        }

        entries.push(now);
        true
    }
}

#[cfg(feature = "ssr")]
pub fn default_contact_rate_limiter() -> IpRateLimiter {
    IpRateLimiter::new(Duration::from_secs(10 * 60), 3)
}

