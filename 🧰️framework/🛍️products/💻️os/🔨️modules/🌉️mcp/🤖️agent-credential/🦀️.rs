//! 🤖️ Bounded, owner-neutral delegated credential loading and exchange protocol inventory.
use crate::{GatewayError, GatewayErrorCode};
use std::sync::OnceLock;

/// 📏️ Maximum credential and exchange request bytes.
pub const DELEGATED_CREDENTIAL_MAX_BYTES: u64 = 16 * 1024;

/// 🔐️ A delegated secret whose wire grammar belongs to the selected owner.
pub struct DelegatedCredentialV1 { origin: String, scope_id: String, audience: String, token: String }
impl std::fmt::Debug for DelegatedCredentialV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_struct("DelegatedCredentialV1").field("origin", &self.origin).field("scope_id", &self.scope_id).field("audience", &self.audience).field("token", &"<redacted>").finish() }
}
impl Drop for DelegatedCredentialV1 { fn drop(&mut self) { unsafe { self.token.as_bytes_mut() }.fill(0); } }
impl DelegatedCredentialV1 {
    /// 🛂️ Constructs only bounded nonempty neutral credential fields.
    pub fn new(origin: String, scope_id: String, audience: String, mut token: String) -> Result<Self, GatewayError> {
        if [&origin, &scope_id, &audience, &token].iter().any(|field| field.is_empty() || field.len() > 4096 || field.chars().any(char::is_control)) { unsafe { token.as_bytes_mut() }.fill(0); return Err(invalid("credential fields are invalid")); }
        Ok(Self { origin, scope_id, audience, token })
    }
    pub fn origin(&self) -> &str { &self.origin }
    pub fn scope_id(&self) -> &str { &self.scope_id }
    pub fn audience(&self) -> &str { &self.audience }
    pub fn expose_for_exchange(&self) -> &str { &self.token }
}

/// 📤️ An exact relative exchange route and bounded owner-encoded request body.
pub struct CredentialExchangeRequestV1 { pub path: String, pub body: Vec<u8> }
impl CredentialExchangeRequestV1 {
    pub fn new(path: String, body: Vec<u8>) -> Result<Self, GatewayError> {
        if path.len() < 2 || path.len() > 1024 || !path.starts_with('/') || path[1..].split('/').any(|part| part.is_empty() || part == "." || part == ".." || !part.bytes().all(|byte| byte.is_ascii_alphanumeric() || b"_.~-".contains(&byte))) || body.len() > DELEGATED_CREDENTIAL_MAX_BYTES as usize { return Err(invalid("credential exchange request is invalid")); }
        Ok(Self { path, body })
    }
}

/// 🎁️ A first-party session grant returned by the selected owner decoder.
pub struct DelegatedSessionGrantV1 { pub token: String, pub agent_principal_id: String, pub agent_label: String, pub space_id: String, pub expires_at_ms: i64 }
impl Drop for DelegatedSessionGrantV1 { fn drop(&mut self) { unsafe { self.token.as_bytes_mut() }.fill(0); } }
impl std::fmt::Debug for DelegatedSessionGrantV1 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result { f.debug_struct("DelegatedSessionGrantV1").field("agent_principal_id", &self.agent_principal_id).field("space_id", &self.space_id).field("token", &"<redacted>").finish() }
}

/// 🔌️ A caller-installed credential wire protocol, with no concrete owner type in its API.
#[derive(Clone, Copy)]
pub struct CredentialExchangeProtocolV1 {
    pub id: &'static str,
    pub credential_schema: &'static str,
    pub decode: fn(&[u8]) -> Result<DelegatedCredentialV1, GatewayError>,
    pub request: fn(&DelegatedCredentialV1, &str) -> Result<CredentialExchangeRequestV1, GatewayError>,
    pub decode_grant: fn(&[u8], &DelegatedCredentialV1) -> Result<DelegatedSessionGrantV1, GatewayError>,
    pub decode_error: fn(u16, &[u8]) -> GatewayError,
}
static PROTOCOLS: OnceLock<Vec<CredentialExchangeProtocolV1>> = OnceLock::new();
fn invalid(message: &str) -> GatewayError { GatewayError::new(GatewayErrorCode::InputInvalid, message) }

/// 🛂️ Installs one exact bounded caller inventory and refuses ambiguous schema ownership.
pub fn install_credential_exchange_protocols_v1(protocols: Vec<CredentialExchangeProtocolV1>) -> Result<(), GatewayError> {
    if protocols.len() > 128 { return Err(invalid("credential protocol inventory exceeds its boundary")); }
    let mut ids = std::collections::BTreeSet::new(); let mut schemas = std::collections::BTreeSet::new();
    for protocol in &protocols {
        if [protocol.id, protocol.credential_schema].iter().any(|id| id.is_empty() || id.len() > 512 || id.chars().any(char::is_control)) || !ids.insert(protocol.id) || !schemas.insert(protocol.credential_schema) { return Err(invalid("credential protocol inventory is ambiguous")); }
    }
    PROTOCOLS.set(protocols).map_err(|_| invalid("credential protocol inventory is already installed"))
}

/// 📄️ Selects exactly the owner of the declared wire schema before decoding a secret.
pub fn decode_delegated_credential_v1(bytes: &[u8], protocols: &[CredentialExchangeProtocolV1]) -> Result<(CredentialExchangeProtocolV1, DelegatedCredentialV1), GatewayError> {
    if bytes.is_empty() || bytes.len() as u64 > DELEGATED_CREDENTIAL_MAX_BYTES { return Err(invalid("credential document exceeds its boundary")); }
    #[derive(serde::Deserialize)]
    struct SchemaSelection { schema: String }
    let selection: SchemaSelection = serde_json::from_slice(bytes).map_err(|_| invalid("credential document schema is absent or invalid"))?;
    let schema = selection.schema.as_str();
    let matches: Vec<_> = protocols.iter().filter(|protocol| protocol.credential_schema == schema).collect();
    if matches.len() != 1 { return Err(invalid("credential document requires one explicitly installed owner protocol")); }
    let protocol = *matches[0];
    Ok((protocol, (protocol.decode)(bytes)?))
}

/// 📂️ Reads an owner-only bounded regular credential document and wipes the source bytes.
pub fn read_delegated_credential_file_v1(path: &std::path::Path, protocols: &[CredentialExchangeProtocolV1]) -> Result<(CredentialExchangeProtocolV1, DelegatedCredentialV1), GatewayError> {
    let metadata = std::fs::symlink_metadata(path).map_err(|_| invalid("credential file is unreadable"))?;
    if !metadata.is_file() || metadata.is_symlink() || metadata.len() > DELEGATED_CREDENTIAL_MAX_BYTES { return Err(invalid("credential file must be bounded and regular")); }
    crate::owner_only::verify_owner_only(path).map_err(|reason| GatewayError::new(GatewayErrorCode::PermissionDenied, reason))?;
    use std::io::Read;
    let file = std::fs::File::open(path).map_err(|_| invalid("credential file is unreadable"))?;
    let opened = file.metadata().map_err(|_| invalid("credential file is unreadable"))?;
    if !opened.is_file() || opened.len() > DELEGATED_CREDENTIAL_MAX_BYTES { return Err(invalid("credential file must be bounded and regular")); }
    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        if metadata.dev() != opened.dev() || metadata.ino() != opened.ino() || opened.permissions().mode() & 0o077 != 0 { return Err(invalid("credential file changed during admission")); }
    }
    let mut bytes = Vec::new();
    let read = file.take(DELEGATED_CREDENTIAL_MAX_BYTES + 1).read_to_end(&mut bytes);
    let credential = read.map_err(|_| invalid("credential file is unreadable")).and_then(|_| decode_delegated_credential_v1(&bytes, protocols)); bytes.fill(0); credential
}

/// 📥️ Loads a source location through the process caller's admitted protocol inventory.
pub fn load_delegated_credential_v1(source: &crate::AgentCredentialSource) -> Result<(CredentialExchangeProtocolV1, DelegatedCredentialV1), GatewayError> {
    let protocols = PROTOCOLS.get().map(Vec::as_slice).unwrap_or(&[]);
    match source {
        crate::AgentCredentialSource::File(path) => read_delegated_credential_file_v1(std::path::Path::new(path), protocols),
        #[cfg(unix)]
        crate::AgentCredentialSource::Descriptor(fd) => {
            use std::io::Read;
            use std::os::fd::FromRawFd;
            if *fd < 0 { return Err(invalid("credential descriptor must be nonnegative")); }
            let mut file = unsafe { std::fs::File::from_raw_fd(*fd) }; let mut bytes = Vec::new();
            let read = file.by_ref().take(DELEGATED_CREDENTIAL_MAX_BYTES + 1).read_to_end(&mut bytes);
            let outcome = read.map_err(|_| invalid("credential descriptor is unreadable")).and_then(|_| decode_delegated_credential_v1(&bytes, protocols)); bytes.fill(0); outcome
        }
        #[cfg(not(unix))]
        crate::AgentCredentialSource::Descriptor(_) => Err(invalid("credential descriptors are unavailable on this platform; use a file")),
    }
}

#[cfg(test)]
#[path="../🧪️tests/🤖️agent-credential/🦀️.rs"]
mod tests;
