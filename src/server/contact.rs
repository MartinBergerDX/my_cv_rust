//! Contact form server function. Sends email via Resend (configured in configuration/*.yaml and APP_* env).

#[cfg(not(target_arch = "wasm32"))]
use crate::{configuration, email_client};
use leptos::prelude::ServerFnError;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ContactFormInput {
    pub name: String,
    pub email: String,
    pub subject: Option<String>,
    pub message: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContactFormResult {
    pub success: bool,
    pub message: String,
}

#[cfg(not(target_arch = "wasm32"))]
fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[cfg(not(target_arch = "wasm32"))]
fn contact_email_subject(input: &ContactFormInput) -> String {
    input
        .subject
        .as_deref()
        .filter(|s| !s.trim().is_empty())
        .map(|s| s.to_string())
        .unwrap_or_else(|| "Contact form message".to_string())
}

#[cfg(not(target_arch = "wasm32"))]
fn contact_email_html(input: &ContactFormInput) -> String {
    let subject = contact_email_subject(input);
    format!(
        r#"<p><strong>From:</strong> {} &lt;{}&gt;</p>
<p><strong>Subject:</strong> {}</p>
<hr>
<p>{}</p>"#,
        escape_html(&input.name),
        escape_html(&input.email),
        escape_html(&subject),
        escape_html(&input.message).replace('\n', "<br>\n")
    )
}

#[cfg(not(target_arch = "wasm32"))]
fn contact_email_text(input: &ContactFormInput) -> String {
    let subject = contact_email_subject(input);
    format!(
        "From: {} <{}>\nSubject: {}\n\n{}",
        input.name, input.email, subject, input.message
    )
}

#[leptos::server]
pub async fn submit_contact(
    input: ContactFormInput,
) -> Result<ContactFormResult, ServerFnError> {
    if input.name.trim().is_empty() {
        return Ok(ContactFormResult {
            success: false,
            message: "Name is required.".into(),
        });
    }
    if input.email.trim().is_empty() {
        return Ok(ContactFormResult {
            success: false,
            message: "Email is required.".into(),
        });
    }
    if !input.email.contains('@') {
        return Ok(ContactFormResult {
            success: false,
            message: "Please enter a valid email address.".into(),
        });
    }
    if input.message.trim().is_empty() {
        return Ok(ContactFormResult {
            success: false,
            message: "Message is required.".into(),
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        tracing::info!("submit_contact: validation passed, loading config and sending email");
        let conf = configuration::get_configuration().map_err(|e| -> ServerFnError {
            tracing::error!("submit_contact: configuration error: {}", e);
            ServerFnError::ServerError(format!("Configuration error: {}", e))
        })?;
        let recipient = conf.contact_recipient_email.as_str();
        tracing::info!(recipient = %recipient, "submit_contact: sending email via client");
        let client = email_client::get_email_client();
        let subject = format!("[CV Contact] {}", contact_email_subject(&input));
        let html = contact_email_html(&input);
        let text = contact_email_text(&input);

        if let Err(e) = client
            .send_email(recipient, &subject, &html, &text, Some(&input.email))
            .await
        {
            tracing::error!("submit_contact: send_email failed: {}", e);
            return Err(ServerFnError::ServerError(format!("Failed to send email: {}", e)));
        }
        tracing::info!("submit_contact: email sent successfully");
    }

    #[cfg(target_arch = "wasm32")]
    let _ = input;

    Ok(ContactFormResult {
        success: true,
        message: "Thank you! Your message has been sent.".into(),
    })
}
