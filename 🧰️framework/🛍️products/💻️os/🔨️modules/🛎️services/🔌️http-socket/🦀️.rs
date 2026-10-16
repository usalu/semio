//! 🔌️ The bounded socket HTTP transport: framing, the cancellation handle and the connect, read-line and read-page drivers.

use semio_framework_async::{HostFuture, OperationContext, ScopeHandle};
use std::sync::{Arc, Mutex, OnceLock};
use std::sync::atomic::{AtomicBool, Ordering};
use super::compute_pool::ComputePool;
use super::host_runtime::TokioHostRuntime;
use super::http_contract::{
    AsyncHttpTransport,
    HttpBody,
    HttpBodyCancellationHandle,
    HttpPoolError,
    HttpRequest,
    HttpResponseHead,
    HttpTransportStart,
    HttpTransportTerminalGuard,
    HttpTransportTerminalHandle,
};

//#region 🌐️HttpPool
const SOCKET_HTTP_URL_BYTES: usize = 2_048;
#[doc(hidden)] pub const SOCKET_HTTP_HOST_BYTES: usize = 256;
const SOCKET_HTTP_HEADER_BYTES: usize = 16 * 1024;
const SOCKET_HTTP_HEADER_ITEMS: usize = 64;
#[doc(hidden)] pub const SOCKET_HTTP_BODY_PAGE_BYTES: usize = 16 * 1024;

#[derive(Clone, Copy)]
enum SocketHttpFraming {
    Length(u64),
    Chunked { remaining: usize },
    UntilEof,
    Terminal,
}

struct SocketHttpBodyState {
    pub(super) reader: std::io::BufReader<std::net::TcpStream>,
    framing: SocketHttpFraming,
}

pub struct SocketHttpBody {
    pub(super) state: Arc<Mutex<Option<SocketHttpBodyState>>>,
    pub(super) cancellation: HttpBodyCancellationHandle,
    pub(super) compute: Arc<ComputePool>,
    pub(super) runtime: Arc<TokioHostRuntime>,
    pub(super) scope: ScopeHandle,
    pub(super) ctx: OperationContext,
}

impl HttpBody for SocketHttpBody {
    fn cancellation_handle(&self) -> HttpBodyCancellationHandle {
        self.cancellation.clone()
    }

    fn next_chunk(&mut self, terminal: HttpTransportTerminalGuard) -> HostFuture<Result<Option<Vec<u8>>, HttpPoolError>> {
        let state = self.state.clone();
        let compute = self.compute.clone();
        let runtime = self.runtime.clone();
        let scope = self.scope.clone();
        let ctx = self.ctx.clone();
        Box::pin(async move {
            compute
                .run_io(runtime.as_ref(), &scope, ctx, move || {
                    let _terminal = terminal;
                    let mut slot = state.lock().map_err(|_| HttpPoolError::Transport("socket HTTP body lock poisoned".into()))?;
                    let body = slot.as_mut().ok_or_else(|| HttpPoolError::Transport("socket HTTP body reached terminal ownership".into()))?;
                    socket_http_read_page(body)
                })
                .await
                .map_err(HttpPoolError::Compute)?
        })
    }
}

pub struct SocketHttpTransport {
    pub(super) compute: Arc<ComputePool>,
    pub(super) runtime: Arc<TokioHostRuntime>,
    pub(super) scope: ScopeHandle,
}

#[doc(hidden)] pub struct SocketHttpCancellation {
    requested: AtomicBool,
    pub(super) stream: OnceLock<std::net::TcpStream>,
}

impl SocketHttpCancellation {
    #[doc(hidden)] pub fn new() -> Self {
        Self { requested: AtomicBool::new(false), stream: OnceLock::new() }
    }

    #[doc(hidden)] pub fn handle(owner: &Arc<Self>) -> HttpBodyCancellationHandle {
        let owner = owner.clone();
        HttpBodyCancellationHandle::interrupt(move || owner.interrupt())
    }

    #[doc(hidden)] pub fn bind(&self, stream: &std::net::TcpStream) -> std::io::Result<()> {
        let clone = stream.try_clone()?;
        let _ = self.stream.set(clone);
        if self.requested.load(Ordering::Acquire) {
            self.interrupt()?;
        }
        Ok(())
    }

    pub(super) fn interrupt(&self) -> std::io::Result<()> {
        self.requested.store(true, Ordering::Release);
        if let Some(stream) = self.stream.get() {
            match stream.shutdown(std::net::Shutdown::Both) {
                Ok(()) => {}
                Err(error) if matches!(error.kind(), std::io::ErrorKind::NotConnected) => {}
                Err(error) => return Err(error),
            }
        }
        Ok(())
    }
}

impl SocketHttpTransport {
    pub fn new(compute: Arc<ComputePool>, runtime: Arc<TokioHostRuntime>, scope: ScopeHandle) -> Self {
        Self { compute, runtime, scope }
    }
}

impl AsyncHttpTransport for SocketHttpTransport {
    fn begin(&self, ctx: &OperationContext, request: HttpRequest) -> HttpTransportStart {
        let compute = self.compute.clone();
        let runtime = self.runtime.clone();
        let scope = self.scope.clone();
        let connect_compute = compute.clone();
        let connect_runtime = runtime.clone();
        let connect_scope = scope.clone();
        let connect_ctx = ctx.clone();
        let body_ctx = ctx.clone();
        let cancellation_owner = Arc::new(SocketHttpCancellation::new());
        let cancellation = SocketHttpCancellation::handle(&cancellation_owner);
        let body_cancellation = cancellation.clone();
        let (terminal, terminal_guard) = HttpTransportTerminalHandle::pair();
        let response = Box::pin(async move {
            let connected = connect_compute
                .run_io(connect_runtime.as_ref(), &connect_scope, connect_ctx, move || {
                    let _terminal = terminal_guard;
                    socket_http_connect(request, cancellation_owner)
                })
                .await
                .map_err(HttpPoolError::Compute)??;
            let (head, state) = connected;
            let body: Box<dyn HttpBody> = Box::new(SocketHttpBody { state: Arc::new(Mutex::new(Some(state))), cancellation, compute, runtime, scope, ctx: body_ctx });
            Ok((head, body))
        });
        HttpTransportStart::new(body_cancellation, terminal, response)
    }
}

fn socket_http_connect(request: HttpRequest, cancellation: Arc<SocketHttpCancellation>) -> Result<(HttpResponseHead, SocketHttpBodyState), HttpPoolError> {
    use std::io::Write;
    if request.method != "GET" || !request.body.is_empty() {
        return Err(HttpPoolError::Transport("socket HTTP transport admits GET with an empty body only".into()));
    }
    if request.url.len() > SOCKET_HTTP_URL_BYTES || request.headers.len() > SOCKET_HTTP_HEADER_ITEMS {
        return Err(HttpPoolError::Transport("socket HTTP request exceeded fixed credits".into()));
    }
    let (host, port, path) = socket_http_url(&request.url)?;
    let header_bytes = request.headers.iter().try_fold(0usize, |total, (name, value)| {
        if name.bytes().any(|byte| byte <= b' ' || byte == b':') || value.bytes().any(|byte| byte == b'\r' || byte == b'\n') {
            return Err(HttpPoolError::Transport("socket HTTP request contained invalid header bytes".into()));
        }
        total
            .checked_add(name.len())
            .and_then(|total| total.checked_add(value.len()))
            .and_then(|total| total.checked_add(4))
            .filter(|total| *total <= SOCKET_HTTP_HEADER_BYTES)
            .ok_or_else(|| HttpPoolError::Transport("socket HTTP request headers exceeded fixed credits".into()))
    })?;
    let _ = header_bytes;
    let mut stream = std::net::TcpStream::connect((host.as_str(), port)).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
    cancellation.bind(&stream).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
    stream.set_nodelay(true).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
    let mut head = Vec::with_capacity(request.url.len().min(SOCKET_HTTP_URL_BYTES));
    head.extend_from_slice(b"GET ");
    head.extend_from_slice(path.as_bytes());
    head.extend_from_slice(b" HTTP/1.1\r\nHost: ");
    head.extend_from_slice(host.as_bytes());
    head.extend_from_slice(b"\r\nConnection: close\r\nAccept-Encoding: identity\r\n");
    for (name, value) in request.headers {
        head.extend_from_slice(name.as_bytes());
        head.extend_from_slice(b": ");
        head.extend_from_slice(value.as_bytes());
        head.extend_from_slice(b"\r\n");
    }
    head.extend_from_slice(b"\r\n");
    if head.len() > SOCKET_HTTP_HEADER_BYTES {
        return Err(HttpPoolError::Transport("socket HTTP serialized request exceeded fixed credits".into()));
    }
    stream.write_all(&head).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
    let mut reader = std::io::BufReader::with_capacity(SOCKET_HTTP_BODY_PAGE_BYTES, stream);
    let status_line = socket_http_read_line(&mut reader, SOCKET_HTTP_HEADER_BYTES)?;
    let mut status_parts = status_line.split_whitespace();
    if !status_parts.next().is_some_and(|version| version == "HTTP/1.1" || version == "HTTP/1.0") {
        return Err(HttpPoolError::Transport("socket HTTP response used an unsupported protocol".into()));
    }
    let status = status_parts.next().and_then(|status| status.parse::<u16>().ok()).ok_or_else(|| HttpPoolError::Transport("socket HTTP response omitted a valid status".into()))?;
    let mut headers = Vec::with_capacity(SOCKET_HTTP_HEADER_ITEMS);
    let mut header_total = status_line.len().saturating_add(2);
    let mut content_length = None;
    let mut chunked = false;
    loop {
        let line = socket_http_read_line(&mut reader, SOCKET_HTTP_HEADER_BYTES.saturating_sub(header_total))?;
        header_total =
            header_total.checked_add(line.len()).and_then(|total| total.checked_add(2)).filter(|total| *total <= SOCKET_HTTP_HEADER_BYTES).ok_or_else(|| HttpPoolError::Transport("socket HTTP response headers exceeded fixed credits".into()))?;
        if line.is_empty() {
            break;
        }
        if headers.len() == SOCKET_HTTP_HEADER_ITEMS {
            return Err(HttpPoolError::Transport("socket HTTP response header count exceeded fixed credits".into()));
        }
        let (name, value) = line.split_once(':').ok_or_else(|| HttpPoolError::Transport("socket HTTP response header was malformed".into()))?;
        let value = value.trim().to_string();
        if name.eq_ignore_ascii_case("content-length") {
            content_length = Some(value.parse::<u64>().map_err(|_| HttpPoolError::Transport("socket HTTP Content-Length was invalid".into()))?);
        }
        if name.eq_ignore_ascii_case("transfer-encoding") {
            chunked = value.split(',').any(|part| part.trim().eq_ignore_ascii_case("chunked"));
        }
        headers.push((name.to_string(), value));
    }
    let framing = if chunked {
        SocketHttpFraming::Chunked { remaining: 0 }
    } else if let Some(length) = content_length {
        SocketHttpFraming::Length(length)
    } else {
        SocketHttpFraming::UntilEof
    };
    Ok((HttpResponseHead { status, headers }, SocketHttpBodyState { reader, framing }))
}

#[doc(hidden)] pub fn socket_http_url(url: &str) -> Result<(String, u16, String), HttpPoolError> {
    let rest = url.strip_prefix("http://").ok_or_else(|| HttpPoolError::Transport("socket HTTP transport requires an http:// URL; HTTPS has no owned TLS stream yet".into()))?;
    let (authority, path) = rest.split_once('/').map_or((rest, "/".to_string()), |(authority, path)| (authority, format!("/{path}")));
    if authority.is_empty() || authority.len() > SOCKET_HTTP_HOST_BYTES || path.len() > SOCKET_HTTP_URL_BYTES {
        return Err(HttpPoolError::Transport("socket HTTP authority/path exceeded fixed credits".into()));
    }
    let (host, port) =
        authority.rsplit_once(':').map_or_else(|| Ok((authority.to_string(), 80)), |(host, port)| port.parse::<u16>().map(|port| (host.to_string(), port)).map_err(|_| HttpPoolError::Transport("socket HTTP port was invalid".into())))?;
    if host.is_empty() {
        return Err(HttpPoolError::Transport("socket HTTP host was empty".into()));
    }
    Ok((host, port, path))
}

#[doc(hidden)] pub fn socket_http_read_line(reader: &mut impl std::io::Read, remaining: usize) -> Result<String, HttpPoolError> {
    if remaining == 0 {
        return Err(HttpPoolError::Transport("socket HTTP line exceeded fixed credits".into()));
    }
    let mut bytes = Vec::with_capacity(remaining.min(256));
    for _ in 0..remaining {
        let mut byte = [0];
        reader.read_exact(&mut byte).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
        bytes.push(byte[0]);
        if byte[0] == b'\n' {
            break;
        }
    }
    if !bytes.ends_with(b"\n") {
        return Err(HttpPoolError::Transport("socket HTTP response line exceeded fixed credits".into()));
    }
    if bytes.ends_with(b"\n") {
        bytes.pop();
    }
    if bytes.ends_with(b"\r") {
        bytes.pop();
    }
    String::from_utf8(bytes).map_err(|_| HttpPoolError::Transport("socket HTTP response header was not UTF-8".into()))
}

fn socket_http_read_page(body: &mut SocketHttpBodyState) -> Result<Option<Vec<u8>>, HttpPoolError> {
    use std::io::Read;
    match body.framing {
        SocketHttpFraming::Length(0) | SocketHttpFraming::Terminal => {
            body.framing = SocketHttpFraming::Terminal;
            Ok(None)
        }
        SocketHttpFraming::Length(remaining) => {
            let mut bytes = vec![0; usize::try_from(remaining).unwrap_or(usize::MAX).min(SOCKET_HTTP_BODY_PAGE_BYTES)];
            let count = body.reader.read(&mut bytes).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
            if count == 0 {
                return Err(HttpPoolError::Transport("socket HTTP body ended before Content-Length".into()));
            }
            bytes.truncate(count);
            body.framing = SocketHttpFraming::Length(remaining - count as u64);
            Ok(Some(bytes))
        }
        SocketHttpFraming::UntilEof => {
            let mut bytes = vec![0; SOCKET_HTTP_BODY_PAGE_BYTES];
            let count = body.reader.read(&mut bytes).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
            if count == 0 {
                body.framing = SocketHttpFraming::Terminal;
                return Ok(None);
            }
            bytes.truncate(count);
            Ok(Some(bytes))
        }
        SocketHttpFraming::Chunked { mut remaining } => {
            if remaining == 0 {
                let line = socket_http_read_line(&mut body.reader, 128)?;
                let length = line.split(';').next().and_then(|value| usize::from_str_radix(value.trim(), 16).ok()).ok_or_else(|| HttpPoolError::Transport("socket HTTP chunk length was invalid".into()))?;
                if length == 0 {
                    let mut trailer_bytes = 0usize;
                    for _ in 0..SOCKET_HTTP_HEADER_ITEMS {
                        let line = socket_http_read_line(&mut body.reader, SOCKET_HTTP_HEADER_BYTES.saturating_sub(trailer_bytes))?;
                        trailer_bytes = trailer_bytes
                            .checked_add(line.len())
                            .and_then(|bytes| bytes.checked_add(2))
                            .filter(|bytes| *bytes <= SOCKET_HTTP_HEADER_BYTES)
                            .ok_or_else(|| HttpPoolError::Transport("socket HTTP trailers exceeded fixed credits".into()))?;
                        if line.is_empty() {
                            body.framing = SocketHttpFraming::Terminal;
                            return Ok(None);
                        }
                    }
                    return Err(HttpPoolError::Transport("socket HTTP trailer count exceeded fixed credits".into()));
                }
                remaining = length;
            }
            let mut bytes = vec![0; remaining.min(SOCKET_HTTP_BODY_PAGE_BYTES)];
            body.reader.read_exact(&mut bytes).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
            remaining -= bytes.len();
            if remaining == 0 {
                let mut suffix = [0; 2];
                body.reader.read_exact(&mut suffix).map_err(|error| HttpPoolError::Transport(error.to_string()))?;
                if suffix != *b"\r\n" {
                    return Err(HttpPoolError::Transport("socket HTTP chunk omitted its terminator".into()));
                }
            }
            body.framing = SocketHttpFraming::Chunked { remaining };
            Ok(Some(bytes))
        }
    }
}
//#endregion 🌐️HttpPool
