use crate::{dao::email::EmailCode, server_config::SERVER_CONFIGURATION};
use lettre::{transport::smtp::authentication::Credentials, Message, SmtpTransport, Transport};
use std::error::Error;

pub fn send_email_code(email_code: EmailCode) -> Result<impl Send, Box<dyn Error>> {
    let email_back = Message::builder()
        .from("泰格论坛 <sls.tech.forum@outlook.com>".parse().unwrap())
        .to(format!("user <{}>", email_code.email.clone())
            .parse()
            .unwrap())
        .subject("泰格论坛邮箱验证")
        .body(format!("验证码: {}", email_code.code))
        .unwrap();

    let email_account = &SERVER_CONFIGURATION.lock().unwrap().email_account;

    let creds = Credentials::new(email_account.email.clone(), email_account.password.clone());

    let emailer = SmtpTransport::starttls_relay(&email_account.smtp_server)
        .unwrap()
        .credentials(creds)
        .port(587)
        .build();

    emailer.send(&email_back).map_err(|err| err.into())
}

#[allow(dead_code)]
pub fn is_email(str: &String) -> bool {
    if let Some(at_idx) = str.rfind('@') {
        if let Some(dot_idx) = str.rfind('.') {
            at_idx < dot_idx
        } else {
            false
        }
    } else {
        false
    }
}
