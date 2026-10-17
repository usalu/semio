//! 🧾️ The bootstrap wire: signed and unsigned frames, the credential envelope and the HMAC proof helpers.

use super::*;

#[derive(Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct LocalBootstrapProfileWire {
    pub(super) profile_id: String,
    pub(super) subject: String,
    pub(super) display_name: String,
    pub(super) allowed_client_classes: Box<[LocalBootstrapClientClass]>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct InitializeWire {
    pub(super) schema: String,
    pub(super) kind: String,
    pub(super) run_id: String,
    pub(super) channel_key: String,
    pub(super) profiles: Box<[LocalBootstrapProfileWire]>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct KindWire {
    pub(super) kind: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct HelloWire {
    pub(super) schema: String,
    pub(super) kind: String,
    pub(super) run_id: String,
    pub(super) sequence: u64,
    pub(super) exchange_id: String,
    pub(super) issued_at: i64,
    pub(super) expires_at: i64,
    pub(super) launcher_nonce: String,
    pub(super) proof: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct HelloUnsigned<'a> {
    pub(super) schema: &'a str,
    pub(super) kind: &'a str,
    pub(super) run_id: &'a str,
    pub(super) sequence: u64,
    pub(super) exchange_id: &'a str,
    pub(super) issued_at: i64,
    pub(super) expires_at: i64,
    pub(super) launcher_nonce: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct HelloAcceptedUnsigned<'a> {
    pub(super) schema: &'a str,
    pub(super) kind: &'a str,
    pub(super) run_id: &'a str,
    pub(super) sequence: u64,
    pub(super) exchange_id: &'a str,
    pub(super) issued_at: i64,
    pub(super) expires_at: i64,
    pub(super) hub_nonce: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct HelloAcceptedWire<'a> {
    pub(super) schema: &'a str,
    pub(super) kind: &'a str,
    pub(super) run_id: &'a str,
    pub(super) sequence: u64,
    pub(super) exchange_id: &'a str,
    pub(super) issued_at: i64,
    pub(super) expires_at: i64,
    pub(super) hub_nonce: &'a str,
    pub(super) proof: &'a str,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct IssueWire {
    pub(super) schema: String,
    pub(super) kind: String,
    pub(super) run_id: String,
    pub(super) sequence: u64,
    pub(super) exchange_id: String,
    pub(super) issued_at: i64,
    pub(super) expires_at: i64,
    pub(super) profile_id: String,
    pub(super) device_instance_id: String,
    pub(super) client_class: LocalBootstrapClientClass,
    pub(super) proof: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct IssueUnsigned<'a> {
    pub(super) schema: &'a str,
    pub(super) kind: &'a str,
    pub(super) run_id: &'a str,
    pub(super) sequence: u64,
    pub(super) exchange_id: &'a str,
    pub(super) issued_at: i64,
    pub(super) expires_at: i64,
    pub(super) profile_id: &'a str,
    pub(super) device_instance_id: &'a str,
    pub(super) client_class: LocalBootstrapClientClass,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub(super) struct TerminalInputWire {
    pub(super) schema: String,
    pub(super) kind: String,
    pub(super) run_id: String,
    pub(super) sequence: u64,
    pub(super) exchange_id: String,
    pub(super) issued_at: i64,
    pub(super) expires_at: i64,
    pub(super) proof: String,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct TerminalUnsigned<'a> {
    pub(super) schema: &'a str,
    pub(super) kind: &'a str,
    pub(super) run_id: &'a str,
    pub(super) sequence: u64,
    pub(super) exchange_id: &'a str,
    pub(super) issued_at: i64,
    pub(super) expires_at: i64,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RejectUnsigned<'a> {
    pub(super) schema: &'a str,
    pub(super) kind: &'a str,
    pub(super) run_id: &'a str,
    pub(super) sequence: u64,
    pub(super) exchange_id: &'a str,
    pub(super) issued_at: i64,
    pub(super) expires_at: i64,
    pub(super) code: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct RejectWire<'a> {
    pub(super) schema: &'a str,
    pub(super) kind: &'a str,
    pub(super) run_id: &'a str,
    pub(super) sequence: u64,
    pub(super) exchange_id: &'a str,
    pub(super) issued_at: i64,
    pub(super) expires_at: i64,
    pub(super) code: &'a str,
    pub(super) proof: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CredentialUnsigned<'a> {
    pub(super) schema: &'a str,
    pub(super) run_id: &'a str,
    pub(super) sequence: u64,
    pub(super) exchange_id: &'a str,
    pub(super) issued_at: i64,
    pub(super) expires_at: i64,
    pub(super) profile_id: &'a str,
    pub(super) client_class: LocalBootstrapClientClass,
    pub(super) session_id: &'a str,
    pub(super) session_kind: AuthSessionKind,
    pub(super) authorization_generation: u64,
    pub(super) capability: &'a str,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct CredentialWire<'a> {
    pub(super) schema: &'a str,
    pub(super) run_id: &'a str,
    pub(super) sequence: u64,
    pub(super) exchange_id: &'a str,
    pub(super) issued_at: i64,
    pub(super) expires_at: i64,
    pub(super) profile_id: &'a str,
    pub(super) client_class: LocalBootstrapClientClass,
    pub(super) session_id: &'a str,
    pub(super) session_kind: AuthSessionKind,
    pub(super) authorization_generation: u64,
    pub(super) capability: &'a str,
    pub(super) proof: &'a str,
}

pub(super) fn hmac_sha256(key: &[u8; 32], canonical: &[u8]) -> [u8; 32] {
    let mut inner_pad = [0x36; 64];
    let mut outer_pad = [0x5c; 64];
    for index in 0..32 {
        inner_pad[index] ^= key[index];
        outer_pad[index] ^= key[index];
    }
    let mut inner = Sha256::new();
    inner.update(&inner_pad);
    inner.update(LOCAL_BOOTSTRAP_HMAC_DOMAIN);
    inner.update(&(canonical.len() as u32).to_be_bytes());
    inner.update(canonical);
    let inner_digest = inner.finalize();
    let mut outer = Sha256::new();
    outer.update(&outer_pad);
    outer.update(&inner_digest);
    inner_pad.fill(0);
    outer_pad.fill(0);
    outer.finalize()
}

pub(super) fn decode_hex<const N: usize>(encoded: &str) -> DirectoryResult<[u8; N]> {
    if encoded.len() != N * 2 || !encoded.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) {
        return Err(denied());
    }
    let mut bytes = [0; N];
    for (index, output) in bytes.iter_mut().enumerate() {
        let high = hex_nibble(encoded.as_bytes()[index * 2]);
        let low = hex_nibble(encoded.as_bytes()[index * 2 + 1]);
        *output = high << 4 | low;
    }
    Ok(bytes)
}

pub(super) fn hex_nibble(byte: u8) -> u8 {
    match byte {
        b'0'..=b'9' => byte - b'0',
        b'a'..=b'f' => byte - b'a' + 10,
        _ => 0,
    }
}
