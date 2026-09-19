//! 📇️ The wgpu renderer's directory door — the ONE transport seam `🐚️Shell/🎯️targets/🧊️wgpu`'s
//! identity, directory-command and Space-Administration lanes reach the hub through, on BOTH
//! targets.
//!
//! ⚖️ Why a seam and not a `cfg`: Space Administration, identity and the directory command FIFO used
//! to be `#[cfg(not(target_arch = "wasm32"))]` in their entirety, because the only transport wired
//! was `os_directory::client::native::NativeDirectoryTransport` (ureq + a tokio host runtime). The
//! shell code itself is target-neutral — every one of those regions is pure state machinery over
//! `DirectoryClient<T>` — so the split belongs at `T`, not around the features. [`ShellDirectory`]
//! names the one implementation each target links; everything above it is compiled identically.
//!
//! 🚪️ The browser half reuses the file door's exact shape (`🚪️host-io/🟦️.ts`): the wgpu shell runs
//! inside the dedicated Worker that owns the `OffscreenCanvas`, so `web_sys::window()` is `None`
//! there and the kernel's own `client::browser` transport (which starts from `window()`) cannot
//! answer. The shell instead posts a request/response mailbox message through `semioWgpuHostIo`,
//! which the PAGE services with `fetch` — the same `credentials: "include"` cookie-session call the
//! React shell's TypeScript `DirectoryClient` makes, against byte-identical paths
//! (`GET /auth/sessions/me`, `GET /directory/spaces/{id}[?cursor=]`, `POST /directory/commands`,
//! `GET /directory/event-page/v1?after=`).

use semio_framework_os_kernel::os_directory::client::{HttpMethod, HttpResponse, TransportError};
use serde::{Deserialize, Serialize};

//#region 🔖️Wire
/// 🚪️ The door op every directory HTTP hop carries. One literal, declared beside both halves, so the
/// Rust encoder and `🚪️host-io/🟦️.ts`'s decoder cannot drift into two spellings.
pub const DIRECTORY_DOOR_OP: &str = "directory-http";

/// 📨️ One directory HTTP request handed to the page. `body` is text because every payload on the
/// frozen hub surface is JSON — never opaque bytes — so the door needs no binary channel at all.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryDoorRequestV1 {
    pub op: String,
    pub method: String,
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
}

/// 📬️ One answer the page hands back: either a real HTTP status/body pair or a transport refusal.
/// A refusal is never encoded as a synthetic status — a fetch that never reached the hub and a hub
/// that answered 500 are different outcomes, and the directory client's own retry policy branches
/// on exactly that distinction.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryDoorResponseV1 {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 🔤️ The wire spelling of one verb, matching `fetch`'s own `method` vocabulary exactly.
pub fn directory_door_method(method: HttpMethod) -> &'static str {
    match method {
        HttpMethod::Get => "GET",
        HttpMethod::Post => "POST",
        HttpMethod::Delete => "DELETE",
    }
}

/// 📨️ Seals one request for the door. A non-UTF-8 body is refused here rather than lossily
/// transcoded: the hub surface has no such payload, so one would be a caller defect, not a value.
pub fn encode_directory_door_request(method: HttpMethod, url: &str, bearer: Option<&str>, body: Option<&[u8]>) -> Result<String, TransportError> {
    let body = match body {
        Some(bytes) => Some(String::from_utf8(bytes.to_vec()).map_err(|_| TransportError::Io("directory door body is not utf-8".into()))?),
        None => None,
    };
    let request = DirectoryDoorRequestV1 { op: DIRECTORY_DOOR_OP.to_string(), method: directory_door_method(method).to_string(), url: url.to_string(), bearer: bearer.map(str::to_string), body };
    serde_json::to_string(&request).map_err(|error| TransportError::Io(error.to_string()))
}

/// 📬️ Reads one door answer back into the transport-agnostic response the directory client expects.
pub fn decode_directory_door_response(answer: &str) -> Result<HttpResponse, TransportError> {
    let response: DirectoryDoorResponseV1 = serde_json::from_str(answer).map_err(|error| TransportError::Io(format!("directory door answer unreadable: {error}")))?;
    if let Some(error) = response.error {
        return Err(TransportError::Io(error));
    }
    let status = response.status.ok_or_else(|| TransportError::Io("directory door answer carries no status".into()))?;
    Ok(HttpResponse { status, body: response.body.unwrap_or_default().into_bytes() })
}
//#endregion 🔖️Wire

//#region 🔖️BrowserTransport
/// 🌐️ The browser `DirectoryTransport`: every HTTP hop crosses the page mailbox door, and the
/// live-presence socket crosses the DUPLEX door beside it (`🔌️socket-door/🦀️.rs`, packet W15e).
///
/// 🚧️ `issue_socket_grant` still refuses, and that refusal is now the ONE thing between this target
/// and a live directory stream. Two independent reasons, both real: the trait method is
/// SYNCHRONOUS (a wasm isolate cannot block on the page's `fetch`), and `DirectoryClient`'s own
/// `protected_post` requires a `LocalHubCredential` bearer, which a cookie-session browser shell
/// structurally does not hold — React's TypeScript client has the same shape and takes its receipt
/// from an INJECTED `socketGrantIssuer` rather than from the transport. Closing it is a kernel-side
/// grant route, not a door. Until then `open_stream_ws` stops at the grant, never at the socket.
#[cfg(target_arch = "wasm32")]
#[derive(Clone, Copy, Debug, Default)]
pub struct BrowserDoorDirectoryTransport;

#[cfg(target_arch = "wasm32")]
impl semio_framework_os_kernel::os_directory::client::DirectoryTransport for BrowserDoorDirectoryTransport {
    type Ws = BrowserDoorWsConnection;

    async fn http(&self, ctx: &semio_framework_async::OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
        if ctx.cancel.is_cancelled().await {
            return Err(TransportError::Cancelled);
        }
        let request = encode_directory_door_request(method, url, bearer, body.as_deref())?;
        let answer = crate::shell::host_io_call(&request, None).await.map_err(TransportError::Io)?;
        if ctx.cancel.is_cancelled().await {
            return Err(TransportError::Cancelled);
        }
        decode_directory_door_response(&answer)
    }

    fn issue_socket_grant(&self, _ctx: &semio_framework_async::OperationContext, _url: &str, _bearer: &str, _body: &[u8], _timeout_ms: u64) -> Result<HttpResponse, TransportError> {
        Err(TransportError::Io("the browser directory door issues no socket grant".into()))
    }

    /// 🔌️ Dials through the DUPLEX door. The dial itself is asynchronous (the page owns the socket),
    /// so this answers immediately with a `Connecting` connection and `try_recv_text` reports
    /// `Pending` until the page says otherwise — which is exactly what that arm of the trait exists
    /// for, not a stand-in for one.
    fn open_ws(&self, ctx: &semio_framework_async::OperationContext, url: &str, protocols: &[String], _timeout_ms: u64) -> Result<Self::Ws, TransportError> {
        if ctx.cancel.is_cancelled_now() {
            return Err(TransportError::Cancelled);
        }
        crate::socket_door::browser::BrowserDoorSocket::open(url, protocols).map(BrowserDoorWsConnection::new).map_err(TransportError::Io)
    }
}

/// 🔌️ One directory socket the page owns, read through the bounded duplex door. Text frames are the
/// directory contract's own wire (`DirectoryStreamMessage` JSON); a binary frame from this endpoint
/// would be a protocol violation and is dropped rather than transcoded into a fake text frame.
#[cfg(target_arch = "wasm32")]
pub struct BrowserDoorWsConnection {
    socket: crate::socket_door::browser::BrowserDoorSocket,
}

#[cfg(target_arch = "wasm32")]
impl BrowserDoorWsConnection {
    fn new(socket: crate::socket_door::browser::BrowserDoorSocket) -> Self {
        Self { socket }
    }
}

#[cfg(target_arch = "wasm32")]
impl semio_framework_os_kernel::os_directory::client::DirectoryWsConnection for BrowserDoorWsConnection {
    fn send_text(&mut self, text: String) -> Result<(), TransportError> {
        self.socket.send(crate::socket_door::SocketMessage::Text(text)).map_err(TransportError::Io)
    }

    fn send_binary(&mut self, bytes: Vec<u8>) -> Result<(), TransportError> {
        self.socket.send(crate::socket_door::SocketMessage::Binary(bytes)).map_err(TransportError::Io)
    }

    fn try_recv_text(&mut self) -> Result<semio_framework_os_kernel::os_directory::client::DirectoryWsPoll, TransportError> {
        use semio_framework_os_kernel::os_directory::client::DirectoryWsPoll;
        loop {
            match self.socket.try_recv() {
                Some(crate::socket_door::SocketMessage::Text(text)) => return Ok(DirectoryWsPoll::Text(text)),
                // 🛟️ A binary frame on the directory endpoint is off-contract; skip it rather than
                // fabricating text, exactly as the native transport skips a `Message::Binary`.
                Some(crate::socket_door::SocketMessage::Binary(_)) => continue,
                None => break,
            }
        }
        let lane = self.socket.lane();
        if lane.is_closed() {
            return Ok(DirectoryWsPoll::Closed(lane.close_code()));
        }
        Ok(DirectoryWsPoll::Pending)
    }

    fn close(&mut self) {
        self.socket.close();
    }
}
//#endregion 🔖️BrowserTransport

//#region 🔖️Seam
/// 🔌️ The ONE transport the shell's directory client is parameterized over on this target.
#[cfg(not(target_arch = "wasm32"))]
pub type ShellDirectoryTransport = semio_framework_os_kernel::os_directory::client::native::NativeDirectoryTransport<semio_framework_os_services::TokioHostRuntime>;

#[cfg(target_arch = "wasm32")]
pub type ShellDirectoryTransport = BrowserDoorDirectoryTransport;

/// 📇️ The hub directory client every shell lane holds, identical on both targets above the seam.
pub type ShellDirectoryClient = semio_framework_os_kernel::os_directory::client::DirectoryClient<ShellDirectoryTransport>;
//#endregion 🔖️Seam

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
