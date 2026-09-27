//! 🔑️ Password credentials for `hub.auth`: the framework's own PBKDF2-HMAC-SHA256
//! ([`semio_framework_hash::pbkdf2_sha256`]), with no external cryptography dependency (AGENTS.md: no
//! runtime dependency on external libraries).
//!
//! Schema authority: [`🔣️.json`](../🧬️schema/🔣️.json) `$defs/CredentialHashV1` — the encoded
//! credential is exactly `pbkdf2-sha256$<iterations>$<32 lower-hex salt>$<64 lower-hex digest>`,
//! the one string `UserRecord::password_hash` ever carries. Verification reads the iteration count
//! and salt out of the stored string, so raising [`DEFAULT_ITERATIONS`] never invalidates a
//! credential minted under the old cost.

use crate::directory::constant_time_digest_eq;
use semio_framework_hash::pbkdf2_sha256;

/// 🏷️ The only credential scheme this hub mints or verifies.
pub const CREDENTIAL_SCHEME: &str = "pbkdf2-sha256";
/// 🐢️ Cost of one freshly minted credential (OWASP's 2023 floor for PBKDF2-HMAC-SHA256).
pub const DEFAULT_ITERATIONS: u32 = 210_000;
/// 🚧️ Admissible cost window; the lower bound keeps test fixtures cheap without admitting `1`.
pub const MIN_ITERATIONS: u32 = 1_000;
pub const MAX_ITERATIONS: u32 = 999_999_999;
pub const SALT_BYTES: usize = 16;
pub const DIGEST_BYTES: usize = 32;
pub const PASSWORD_MIN_BYTES: usize = 8;
pub const PASSWORD_MAX_BYTES: usize = 256;

/// 🛑️ Why a credential could not be derived, parsed, or admitted.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PasswordCredentialError {
    PasswordOutOfBounds,
    IterationsOutOfBounds,
    Malformed,
    EntropyUnavailable,
}

impl std::fmt::Display for PasswordCredentialError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(match self {
            Self::PasswordOutOfBounds => "password is outside 8..=256 bytes",
            Self::IterationsOutOfBounds => "pbkdf2 iteration count is outside 1000..=999999999",
            Self::Malformed => "credential is not a pbkdf2-sha256 encoding",
            Self::EntropyUnavailable => "operating-system credential entropy unavailable",
        })
    }
}

impl std::error::Error for PasswordCredentialError {}

/// 🔐️ One stored password credential. `Debug` never prints the digest or the salt.
#[derive(Clone, PartialEq, Eq)]
pub struct PasswordCredentialV1 {
    iterations: u32,
    salt: [u8; SALT_BYTES],
    digest: [u8; DIGEST_BYTES],
}

impl std::fmt::Debug for PasswordCredentialV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("PasswordCredentialV1").field("iterations", &self.iterations).field("salt", &"[REDACTED]").field("digest", &"[REDACTED]").finish()
    }
}

impl PasswordCredentialV1 {
    /// 🎲️ Mints a credential over fresh operating-system entropy at the given cost.
    pub fn mint(password: &str, iterations: u32) -> Result<Self, PasswordCredentialError> {
        let mut salt = [0u8; SALT_BYTES];
        directory::os_identity::fill_entropy(&mut salt).map_err(|_| PasswordCredentialError::EntropyUnavailable)?;
        Self::derive(password, salt, iterations)
    }

    /// 🧮️ Derives a credential over an exact salt — the deterministic seam every test vector uses.
    pub fn derive(password: &str, salt: [u8; SALT_BYTES], iterations: u32) -> Result<Self, PasswordCredentialError> {
        if !(PASSWORD_MIN_BYTES..=PASSWORD_MAX_BYTES).contains(&password.len()) {
            return Err(PasswordCredentialError::PasswordOutOfBounds);
        }
        if !(MIN_ITERATIONS..=MAX_ITERATIONS).contains(&iterations) {
            return Err(PasswordCredentialError::IterationsOutOfBounds);
        }
        Ok(Self { iterations, salt, digest: pbkdf2_sha256(password.as_bytes(), &salt, iterations) })
    }

    /// 🕳️ The credential an account without one is checked against: the cost of a freshly minted
    /// credential and a digest no password derives, so refusing an unknown account takes as long as
    /// refusing a wrong password and the refusal's timing never tells the two apart.
    pub fn absent() -> Self {
        Self { iterations: DEFAULT_ITERATIONS, salt: [0; SALT_BYTES], digest: [0; DIGEST_BYTES] }
    }

    /// 📖️ Parses the exact `CredentialHashV1` encoding; anything else is [`PasswordCredentialError::Malformed`].
    pub fn parse(encoded: &str) -> Result<Self, PasswordCredentialError> {
        let mut parts = encoded.split('$');
        if parts.next() != Some(CREDENTIAL_SCHEME) {
            return Err(PasswordCredentialError::Malformed);
        }
        let iterations = parts.next().ok_or(PasswordCredentialError::Malformed)?;
        let salt = parts.next().ok_or(PasswordCredentialError::Malformed)?;
        let digest = parts.next().ok_or(PasswordCredentialError::Malformed)?;
        if parts.next().is_some() || iterations.starts_with('0') || iterations.is_empty() || !iterations.bytes().all(|byte| byte.is_ascii_digit()) {
            return Err(PasswordCredentialError::Malformed);
        }
        let iterations: u32 = iterations.parse().map_err(|_| PasswordCredentialError::Malformed)?;
        if !(MIN_ITERATIONS..=MAX_ITERATIONS).contains(&iterations) {
            return Err(PasswordCredentialError::Malformed);
        }
        Ok(Self { iterations, salt: decode_lower_hex::<SALT_BYTES>(salt)?, digest: decode_lower_hex::<DIGEST_BYTES>(digest)? })
    }

    /// 🧾️ The exact stored string. Safe to persist; carries no plaintext.
    pub fn encode(&self) -> String {
        format!("{CREDENTIAL_SCHEME}${}${}${}", self.iterations, encode_lower_hex(&self.salt), encode_lower_hex(&self.digest))
    }

    /// ✅️ Constant-time verification at this credential's own stored cost.
    pub fn verify(&self, password: &str) -> bool {
        if !(PASSWORD_MIN_BYTES..=PASSWORD_MAX_BYTES).contains(&password.len()) {
            return false;
        }
        constant_time_digest_eq(&self.digest, &pbkdf2_sha256(password.as_bytes(), &self.salt, self.iterations))
    }

    pub fn iterations(&self) -> u32 {
        self.iterations
    }

    pub fn salt(&self) -> [u8; SALT_BYTES] {
        self.salt
    }

    pub fn digest(&self) -> [u8; DIGEST_BYTES] {
        self.digest
    }
}

fn encode_lower_hex(bytes: &[u8]) -> String {
    let mut encoded = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        encoded.push(char::from_digit(u32::from(byte >> 4), 16).unwrap_or('0'));
        encoded.push(char::from_digit(u32::from(byte & 0x0f), 16).unwrap_or('0'));
    }
    encoded
}

fn decode_lower_hex<const N: usize>(encoded: &str) -> Result<[u8; N], PasswordCredentialError> {
    if encoded.len() != N * 2 {
        return Err(PasswordCredentialError::Malformed);
    }
    let bytes = encoded.as_bytes();
    let mut decoded = [0u8; N];
    for index in 0..N {
        let high = lower_hex_value(bytes[index * 2])?;
        let low = lower_hex_value(bytes[index * 2 + 1])?;
        decoded[index] = (high << 4) | low;
    }
    Ok(decoded)
}

fn lower_hex_value(byte: u8) -> Result<u8, PasswordCredentialError> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        _ => Err(PasswordCredentialError::Malformed),
    }
}
