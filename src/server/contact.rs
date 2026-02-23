//! Contact form server function. Mock for now; ready for email API integration.

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

#[leptos::server]
pub async fn submit_contact(input: ContactFormInput) -> Result<ContactFormResult, ServerFnError> {
    // Validation
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

    // TODO: Integrate with your email API (e.g. Resend, SendGrid, Cloudflare Email Workers).
    // Example: send_email(&input).await?;
    let _ = input; // use input when wiring email

    Ok(ContactFormResult {
        success: true,
        message: "Thank you! Your message has been sent.".into(),
    })
}
