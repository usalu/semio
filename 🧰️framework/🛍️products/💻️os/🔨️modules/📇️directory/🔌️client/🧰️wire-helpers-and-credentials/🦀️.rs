//! 🧰️ Wire helpers and credentials of the directory client: the JSON and hex decoders, the URL component encoder, the socket grant probes, the wall clock, the socket hello, the local hub credential with its frame reads and the inherited Windows file. See [`super`].

use super::*;

/// 🔤️ Wire bytes → `FromValue` type, via `pack::json` rather than `serde_json` — the one decode
/// choke point every `request_json` response body passes through.
pub(super) fn decode_json_bytes<R: FromValue>(bytes: &[u8]) -> Result<R, DirectoryClientError> {
    let text = std::str::from_utf8(bytes).map_err(|error| DirectoryClientError::Decode(error.to_string()))?;
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(DirectoryClientError::from)
}

pub(super) fn wipe_string(value: &mut String) -> usize {
    let len = value.len();
    unsafe { value.as_mut_vec().fill(0) };
    len
}

pub(super) struct WipeDocumentPlanSocketGrantIntent(pub(super) DocumentPlanSocketGrantIntentV1);

impl Drop for WipeDocumentPlanSocketGrantIntent {
    fn drop(&mut self) {
        wipe_string(&mut self.0.plan_receipt);
    }
}

fn lower_hex(bytes: &[u8]) -> bool {
    bytes.iter().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(byte))
}

pub(super) fn valid_socket_grant(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 107 && bytes.get(..10) == Some(b"socket.v1.".as_slice()) && bytes.get(10..42).is_some_and(lower_hex) && bytes.get(42) == Some(&b'.') && bytes.get(43..107).is_some_and(lower_hex)
}

pub(super) fn valid_socket_actor(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 71 && bytes.get(..7) == Some(b"hub.v1.".as_slice()) && bytes.get(7..71).is_some_and(lower_hex)
}

/// 🔡️ Decodes one 64-digit lower-hex digest (a pack schema hash on the wire) into its 32 bytes.
pub fn decode_lower_hex_32(value: &str) -> Option<[u8; 32]> {
    if value.len() != 64 || !lower_hex(value.as_bytes()) {
        return None;
    }
    let digit = |byte| match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        _ => None,
    };
    let mut decoded = [0u8; 32];
    for (index, pair) in value.as_bytes().as_chunks::<2>().0.iter().enumerate() {
        decoded[index] = digit(pair[0])? << 4 | digit(pair[1])?;
    }
    Some(decoded)
}

/// 🔗 UTF-8 byte percent-encoding shared by protected HTTP scope and document WebSocket URLs.
pub fn encode_url_component(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(char::from(byte));
        } else {
            encoded.push_str(&format!("%{byte:02X}"));
        }
    }
    encoded
}

#[doc(hidden)] pub const DOCUMENT_ADMISSION_RESPONSE_MAX_BYTES: usize = 64 * 1024;

pub(super) const DIRECTORY_SCOPE_ID_MAX_BYTES: usize = 4 * 1024;

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn emit_socket_grant_probe(path: &str, grant: &str) {
    if std::env::var("SEMIO_DIRECT_CHILD_PROBE").ok().as_deref() != Some("1") {
        return;
    }
    let digest = semio_framework_hash::Sha256::digest(&grant.as_bytes()[10..42]);
    eprintln!("[semio-directory-client] socket-grant-selector-digest {path} {}", digest.iter().map(|byte| format!("{byte:02x}")).collect::<String>());
}

#[cfg(target_arch = "wasm32")]
pub(super) fn emit_socket_grant_probe(_path: &str, _grant: &str) {}

/// 🔢 Direct-child probe: the process-local ordinal of every document socket grant exchanged, so a
/// harness can prove a reconnect ran a fresh open-plan exchange.
#[cfg(not(target_arch = "wasm32"))]
pub(super) fn emit_document_socket_grant_probe(path: &str) {
    static EXCHANGES: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);
    if std::env::var("SEMIO_DIRECT_CHILD_PROBE").ok().as_deref() != Some("1") {
        return;
    }
    eprintln!("[semio-directory-client] document-socket-grant {path} {}", EXCHANGES.fetch_add(1, std::sync::atomic::Ordering::Relaxed) + 1);
}

#[cfg(target_arch = "wasm32")]
pub(super) fn emit_document_socket_grant_probe(_path: &str) {}

#[cfg(not(all(target_arch = "wasm32", not(target_env = "p2"))))]
#[doc(hidden)] pub fn wall_now_ms() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| i64::try_from(duration.as_millis()).unwrap_or(i64::MAX))
}

/// ⏱️ Browser: `std`'s `SystemTime::now()` PANICS on `wasm32-unknown-unknown` (no implementation,
/// not an error), so every expiry comparison on this target reads `Date.now()` instead.
#[cfg(all(target_arch = "wasm32", not(target_env = "p2")))]
pub(super) fn wall_now_ms() -> i64 {
    let millis = js_sys::Date::now();
    if millis.is_finite() && millis > 0.0 {
        millis as i64
    } else {
        0
    }
}

pub(super) fn directory_socket_hello_v1() -> Vec<u8> {
    let schema = b"semio.directory.events/v1";
    let mut bytes = Vec::with_capacity(5 + schema.len() + 34);
    bytes.extend_from_slice(&[0, 7, 1, 1, schema.len() as u8]);
    bytes.extend_from_slice(schema);
    bytes.extend_from_slice(&[0; 32]);
    bytes.extend_from_slice(&[0, 0]);
    bytes
}

//#region 🔖️Client
/// 📇️ Talks the hub's directory REST/WS surface over an injected `DirectoryTransport`. Holds the
/// protected local credential and socket-grant authority without serializing either into bindings.
pub struct LocalHubCredential {
    pub(super) hub_origin: String,
    pub(super) capability: Box<[u8]>,
}

pub(super) struct WipeBytes<'a> {
    #[doc(hidden)] pub bytes: Vec<u8>,
    pub(super) observer: Option<&'a std::sync::atomic::AtomicUsize>,
}

pub(super) struct WipeSecretText(pub(super) String);

impl Drop for WipeSecretText {
    fn drop(&mut self) {
        wipe_string(&mut self.0);
    }
}

impl Drop for WipeBytes<'_> {
    fn drop(&mut self) {
        let len = self.bytes.len();
        self.bytes.fill(0);
        if let Some(observer) = self.observer {
            observer.fetch_add(self.bytes.iter().filter(|byte| **byte == 0).count().min(len), std::sync::atomic::Ordering::SeqCst);
        }
    }
}

#[doc(hidden)] pub fn read_local_hub_credential_frame<'a, R: std::io::Read>(pipe: &mut R, observer: Option<&'a std::sync::atomic::AtomicUsize>) -> Result<WipeBytes<'a>, DirectoryClientError> {
    let mut length = [0u8; 4];
    pipe.read_exact(&mut length).map_err(|_| DirectoryClientError::Unauthorized)?;
    let length = u32::from_be_bytes(length) as usize;
    if length == 0 || length > 16 * 1024 {
        return Err(DirectoryClientError::Unauthorized);
    }
    let mut bytes = WipeBytes { bytes: vec![0u8; length], observer };
    pipe.read_exact(&mut bytes.bytes).map_err(|_| DirectoryClientError::Unauthorized)?;
    let mut trailing = WipeBytes { bytes: vec![0u8; 1], observer };
    if pipe.read(&mut trailing.bytes).map_err(|_| DirectoryClientError::Unauthorized)? != 0 {
        return Err(DirectoryClientError::Unauthorized);
    }
    Ok(bytes)
}

struct WipeDslValue<'a> {
    pub(super) value: DslValue,
    pub(super) observer: Option<&'a std::sync::atomic::AtomicUsize>,
}

impl Drop for WipeDslValue<'_> {
    fn drop(&mut self) {
        let wiped = wipe_dsl_value(&mut self.value);
        if let Some(observer) = self.observer {
            observer.fetch_add(wiped, std::sync::atomic::Ordering::SeqCst);
        }
    }
}

#[doc(hidden)] pub fn wipe_dsl_value(value: &mut DslValue) -> usize {
    match value {
        DslValue::String(value) => wipe_string(value),
        DslValue::Bytes(value) => {
            let len = value.len();
            value.fill(0);
            len
        }
        DslValue::Array(values) => values.iter_mut().map(wipe_dsl_value).sum(),
        DslValue::Object(entries) => entries.iter_mut().map(|(key, value)| wipe_string(key) + wipe_dsl_value(value)).sum(),
        DslValue::Null | DslValue::Bool(_) | DslValue::Number(_) => 0,
    }
}

#[doc(hidden)] pub fn decode_local_hub_credential(bytes: &[u8], expected_class: &str, observer: Option<&std::sync::atomic::AtomicUsize>) -> Result<LocalHubCredential, DirectoryClientError> {
    let mut decoded = WipeDslValue { value: decode_json_bytes::<DslValue>(bytes)?, observer };
    let field = |name: &str| decoded.value.get(name).and_then(|value| value.as_str()).ok_or(DirectoryClientError::Unauthorized);
    let schema = field("schema")?;
    let client_class = field("clientClass")?;
    let hub_origin = field("hubOrigin")?;
    let capability = field("capability")?;
    let expires_at_ms = decoded.value.get("expiresAtMs").and_then(|value| value.as_i64()).ok_or(DirectoryClientError::Unauthorized)?;
    if schema != "semio.local.consumer-credential/v1"
        || client_class != expected_class
        || !hub_origin.strip_prefix("http://127.0.0.1:").is_some_and(|port| port.parse::<u16>().is_ok())
        || expires_at_ms <= wall_now_ms()
        || !valid_session_capability(capability)
    {
        return Err(DirectoryClientError::Unauthorized);
    }
    let hub_origin = hub_origin.to_string();
    let DslValue::Object(entries) = &mut decoded.value else { return Err(DirectoryClientError::Unauthorized) };
    let capability = entries
        .iter_mut()
        .find_map(|(key, value)| (key == "capability").then_some(value))
        .and_then(|value| match value {
            DslValue::String(value) => Some(std::mem::take(value).into_bytes().into_boxed_slice()),
            _ => None,
        })
        .ok_or(DirectoryClientError::Unauthorized)?;
    Ok(LocalHubCredential { hub_origin, capability })
}

impl std::fmt::Debug for LocalHubCredential {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.debug_struct("LocalHubCredential").field("hub_origin", &self.hub_origin).field("capability", &"<redacted>").finish()
    }
}

impl Drop for LocalHubCredential {
    fn drop(&mut self) {
        self.capability.fill(0);
    }
}

impl LocalHubCredential {
    /// 🎫️ Seals a capability returned by the public session-mint route. The token stays in
    /// this non-serializable, zeroing owner and can only be consumed through [`DirectoryClient`].
    pub fn from_minted_session(hub_origin: &str, capability: &str) -> Result<Self, DirectoryClientError> {
        let absolute = hub_origin.strip_prefix("https://").or_else(|| hub_origin.strip_prefix("http://")).is_some_and(|authority| !authority.is_empty() && !authority.chars().any(|character| matches!(character, '/' | '?' | '#' | '@')));
        let browser_proxy = hub_origin.starts_with('/') && !hub_origin.starts_with("//") && !hub_origin.chars().any(|character| character.is_control() || matches!(character, '?' | '#'));
        if !(absolute || browser_proxy) || !valid_session_capability(capability) {
            return Err(DirectoryClientError::Unauthorized);
        }
        Ok(Self { hub_origin: hub_origin.trim_end_matches('/').to_string(), capability: capability.as_bytes().into() })
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn read_inherited(expected_class: &str) -> Result<Self, DirectoryClientError> {
        #[cfg(unix)]
        let mut pipe = {
            use std::os::fd::FromRawFd;
            unsafe extern "C" {
                fn fcntl(fd: i32, command: i32, ...) -> i32;
            }
            const F_GETFD: i32 = 1;
            const F_SETFD: i32 = 2;
            const FD_CLOEXEC: i32 = 1;
            let flags = unsafe { fcntl(3, F_GETFD) };
            if flags < 0 || unsafe { fcntl(3, F_SETFD, flags | FD_CLOEXEC) } < 0 {
                return Err(DirectoryClientError::Unauthorized);
            }
            unsafe { std::fs::File::from_raw_fd(3) }
        };
        #[cfg(windows)]
        let mut pipe = inherited_windows_file()?;
        let bytes = read_local_hub_credential_frame(&mut pipe, None)?;
        decode_local_hub_credential(&bytes.bytes, expected_class, None)
    }

    /// 🤖️ Adopts a session capability this process obtained over the network rather than from the
    /// inherited fd-3 pipe — the shape `semio-os-mcp --hub` needs after exchanging an agent
    /// delegation at `POST /auth/agent-sessions`. The capability is held under exactly the same
    /// redacting `Debug`, constant-time comparison and zero-on-drop discipline as an inherited one;
    /// what it deliberately does NOT do is pin `hub_origin` to loopback, because an agent session
    /// is issued by whichever hub the delegation names and the caller has already run that origin
    /// through `validate_hub_origin`.
    pub fn adopt_session_capability(hub_origin: &str, capability: &str) -> Result<Self, DirectoryClientError> {
        if hub_origin.is_empty() || !valid_session_capability(capability) {
            return Err(DirectoryClientError::Unauthorized);
        }
        Ok(Self { hub_origin: hub_origin.to_string(), capability: capability.as_bytes().into() })
    }

    #[cfg(any(test, feature = "testkit"))]
    #[doc(hidden)]
    #[doc(hidden)] pub fn test(hub_origin: &str, capability: &str) -> Self {
        Self { hub_origin: hub_origin.to_string(), capability: capability.as_bytes().into() }
    }

    #[doc(hidden)] pub fn capability(&self) -> Result<&str, DirectoryClientError> {
        std::str::from_utf8(&self.capability).map_err(|_| DirectoryClientError::Unauthorized)
    }

    pub fn hub_origin(&self) -> &str {
        &self.hub_origin
    }

    pub fn authorizes_capability(&self, candidate: &str) -> bool {
        if candidate.len() != self.capability.len() {
            return false;
        }
        candidate.as_bytes().iter().zip(self.capability.iter()).fold(0u8, |difference, (left, right)| difference | (left ^ right)) == 0
    }

    pub fn authorizes_bearer_header(&self, header: &str) -> bool {
        header.strip_prefix("Bearer ").is_some_and(|candidate| self.authorizes_capability(candidate))
    }
}

fn valid_session_capability(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 108 && bytes.get(..11) == Some(b"session.v1.".as_slice()) && bytes.get(11..43).is_some_and(lower_hex) && bytes.get(43) == Some(&b'.') && bytes.get(44..108).is_some_and(lower_hex)
}

#[cfg(windows)]
fn inherited_windows_file() -> Result<std::fs::File, DirectoryClientError> {
    use std::ffi::c_void;
    use std::os::windows::io::FromRawHandle;
    type Handle = *mut c_void;
    unsafe extern "C" {
        fn _get_osfhandle(fd: i32) -> isize;
        fn _close(fd: i32) -> i32;
        fn GetCurrentProcess() -> Handle;
        fn DuplicateHandle(source_process: Handle, source: Handle, target_process: Handle, target: *mut Handle, access: u32, inherit: i32, options: u32) -> i32;
        fn CloseHandle(handle: Handle) -> i32;
    }
    let source = unsafe { _get_osfhandle(3) };
    if source == -1 {
        return Err(DirectoryClientError::Unauthorized);
    }
    let process = unsafe { GetCurrentProcess() };
    let mut duplicate: Handle = std::ptr::null_mut();
    let duplicated = unsafe { DuplicateHandle(process, source as Handle, process, &mut duplicate, 0, 0, 2) } != 0 && !duplicate.is_null();
    let closed = unsafe { _close(3) } == 0;
    if !duplicated || !closed {
        if duplicated {
            unsafe { CloseHandle(duplicate) };
        }
        return Err(DirectoryClientError::Unauthorized);
    }
    Ok(unsafe { std::fs::File::from_raw_handle(duplicate) })
}
//#endregion 🔖️Client
