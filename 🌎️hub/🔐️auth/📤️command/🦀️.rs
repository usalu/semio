//! 🔑️ The operator credential verb, dispatched before any Hub service is opened.
//!
//! This is how the **first** principal of a fresh `OS_HUB_DATA` gets a password, and the only way a
//! principal that has lost its password gets a new one. It is deliberately not an HTTP route: it
//! requires the ability to read and write the hub's own server-owned data root, which is strictly
//! stronger than any capability reachable over the network, so no bearer, no admin subject and no
//! "first request wins" race can stand in for physical control of the deployment. Self-service
//! change (old password required) lives on `POST /auth/credentials` instead.
//!
//! ```text
//! OS_HUB_DATA=/abs/path os-hub credential set --email a@example.com [--display-name "Ada"]
//! ```
//!
//! The password is read from stdin, never from `argv` (which every process on the machine can read)
//! and never from an environment variable (which children inherit). An absent account is created,
//! an existing account keeps its identity and gets the new credential plus a `credential-changed`
//! fact, and in both cases every live session of that principal is revoked before the verb returns.

use super::HubError;
use semio_hub::auth::password::{DEFAULT_ITERATIONS, MAX_ITERATIONS, MIN_ITERATIONS, PASSWORD_MAX_BYTES, PASSWORD_MIN_BYTES, PasswordCredentialV1};
use semio_hub::directory::HubDirectory;
use std::ffi::OsString;

/// 🧾️ The exact parsed `credential set` invocation.
struct CredentialSet {
    email: String,
    display_name: Option<String>,
}

fn unsafe_configuration(message: &str) -> HubError {
    HubError::UnsafeAuthConfiguration(message.into())
}

/// 🚪️ Recognizes only `credential set …` and leaves every other argument vector to the next
/// dispatcher, so an unknown verb is still reported by exactly one owner.
fn selected(arguments: &[OsString]) -> Result<Option<CredentialSet>, HubError> {
    if arguments.len() < 2 || arguments[0] != OsString::from("credential") {
        return Ok(None);
    }
    if arguments[1] != OsString::from("set") {
        return Err(unsafe_configuration("the only credential verb is `credential set`"));
    }
    let mut email = None;
    let mut display_name = None;
    let mut index = 2;
    while index < arguments.len() {
        let flag = arguments[index].to_str().ok_or_else(|| unsafe_configuration("credential set arguments must be UTF-8"))?;
        let value = arguments.get(index + 1).and_then(|value| value.to_str()).ok_or_else(|| unsafe_configuration("credential set: every flag needs one value"))?;
        match flag {
            "--email" => email = Some(value.to_string()),
            "--display-name" => display_name = Some(value.to_string()),
            _ => return Err(unsafe_configuration("credential set accepts only --email and --display-name")),
        }
        index += 2;
    }
    let email = email.ok_or_else(|| unsafe_configuration("credential set requires --email"))?.trim().to_lowercase();
    if !(3..=254).contains(&email.len()) || email.matches('@').count() != 1 || email.split('@').any(str::is_empty) || email.chars().any(|character| character.is_control() || character == ' ') {
        return Err(unsafe_configuration("credential set: --email is out of bounds"));
    }
    if let Some(name) = display_name.as_deref() {
        if name.is_empty() || name.chars().count() > 128 || name.chars().any(char::is_control) || name.trim_matches(' ') != name {
            return Err(unsafe_configuration("credential set: --display-name is out of bounds"));
        }
    }
    Ok(Some(CredentialSet { email, display_name }))
}

/// 🔒️ Reads exactly one password from stdin. A trailing newline is stripped; nothing else is, so a
/// password may contain spaces. The buffer is zeroed before it is dropped.
fn read_password(input: impl std::io::Read) -> Result<String, HubError> {
    use std::io::Read;
    let mut bytes = Vec::with_capacity(PASSWORD_MAX_BYTES + 2);
    input.take(u64::try_from(PASSWORD_MAX_BYTES + 2).unwrap_or(258)).read_to_end(&mut bytes)?;
    while bytes.last().is_some_and(|byte| *byte == b'\n' || *byte == b'\r') {
        bytes.pop();
    }
    let password = String::from_utf8(bytes).map_err(|error| {
        let mut rejected = error.into_bytes();
        rejected.fill(0);
        unsafe_configuration("credential set: the password on stdin is not UTF-8")
    })?;
    if !(PASSWORD_MIN_BYTES..=PASSWORD_MAX_BYTES).contains(&password.len()) || password.chars().any(char::is_control) {
        return Err(unsafe_configuration("credential set: the password on stdin must be 8..=256 bytes with no control characters"));
    }
    Ok(password)
}

/// 🔢️ `OS_HUB_PASSWORD_ITERATIONS`, read independently of `CredentialSignInPolicyV1` so an operator
/// can provision a credential on a hub that has not enabled credential sign-in yet.
fn configured_iterations() -> Result<u32, HubError> {
    match std::env::var("OS_HUB_PASSWORD_ITERATIONS").ok().filter(|value| !value.is_empty()) {
        None => Ok(DEFAULT_ITERATIONS),
        Some(value) => {
            let iterations = value.parse::<u32>().map_err(|_| unsafe_configuration("OS_HUB_PASSWORD_ITERATIONS must be 1000..=999999999"))?;
            (MIN_ITERATIONS..=MAX_ITERATIONS).contains(&iterations).then_some(iterations).ok_or_else(|| unsafe_configuration("OS_HUB_PASSWORD_ITERATIONS must be 1000..=999999999"))
        }
    }
}

async fn set(request: CredentialSet) -> Result<(), HubError> {
    use std::io::Write;
    let data = std::env::var_os("OS_HUB_DATA")
        .map(std::path::PathBuf::from)
        .filter(|path| path.is_absolute())
        .ok_or_else(|| unsafe_configuration("credential set requires an explicit absolute server-owned OS_HUB_DATA"))?;
    let iterations = configured_iterations()?;
    let mut password = read_password(std::io::stdin().lock())?;
    let credential = PasswordCredentialV1::mint(&password, iterations).map_err(|_| unsafe_configuration("credential set: the password could not be hashed"))?;
    unsafe { password.as_mut_vec() }.fill(0);
    let directory = super::connect_directory(&data).await?;
    let correlation_id = directory::os_identity::time_ordered_id();
    let user_id = match directory.get_user_by_email(&request.email).await? {
        Some(existing) => {
            directory.set_password_credential(&existing.id, &credential.encode(), None, &correlation_id).await?;
            directory.revoke_auth_sessions_for_user(&existing.id, "credential-changed", None, &correlation_id).await?;
            existing.id
        }
        None => directory.create_user(&request.email, request.display_name.as_deref().unwrap_or(&request.email), Some(&credential.encode()), None, None).await?.id,
    };
    let mut output = std::io::stdout().lock();
    writeln!(output, "{user_id}")?;
    output.flush()?;
    Ok(())
}

/// 🚪️ Handles only the exact credential verb and leaves ordinary no-argument startup untouched.
pub(super) async fn dispatch(arguments: &[OsString]) -> Result<bool, HubError> {
    match selected(arguments)? {
        Some(request) => set(request).await.map(|()| true),
        None => Ok(false),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
