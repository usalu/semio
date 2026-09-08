
use super::{DirectoryWsConnection, DirectoryWsPoll, HttpMethod, HttpResponse, TransportError};
use semio_framework_async::OperationContext;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, PartialEq)]
pub struct RecordedRequest {
    pub method: HttpMethod,
    pub url: String,
    pub bearer: Option<String>,
    pub body: Vec<u8>,
}

#[derive(Clone, Default)]
pub struct FakeTransport {
    pub responses: Arc<Mutex<VecDeque<Result<HttpResponse, TransportError>>>>,
    pub requests: Arc<Mutex<Vec<RecordedRequest>>>,
    pub ws_outcomes: Arc<Mutex<VecDeque<Result<VecDeque<Result<Option<String>, TransportError>>, TransportError>>>>,
    pub ws_urls: Arc<Mutex<Vec<String>>>,
    pub ws_closes: Arc<AtomicUsize>,
    pub ws_sends: Arc<AtomicUsize>,
    pub cancel_after_grant: Arc<AtomicBool>,
    pub cancel_after_grant_number: Arc<AtomicUsize>,
    pub cancel_after_open: Arc<AtomicBool>,
    /// 🧪️ Cooperative yield points `http()` passes through (checking `ctx.cancel` at each one)
    /// BEFORE touching `responses`/`requests` — 0 (the default) keeps every existing test's
    /// synchronous-looking behavior unchanged; a cancellation test sets this > 0 so an
    /// interleaved caller has a real window to flip the token between yields (see
    /// `an_in_flight_request_is_cancelled_when_its_context_is_cancelled` below).
    pub yields_before_response: Arc<AtomicU32>,
}

impl FakeTransport {
    pub async fn push_response(&self, response: Result<HttpResponse, TransportError>) {
        self.responses.lock().unwrap().push_back(response);
    }

    pub async fn push_ws(&self, outcome: Result<VecDeque<Result<Option<String>, TransportError>>, TransportError>) {
        self.ws_outcomes.lock().unwrap().push_back(outcome);
    }

    pub async fn json_response(status: u16, body: &serde_json::Value) -> Result<HttpResponse, TransportError> {
        Ok(HttpResponse { status, body: serde_json::to_vec(body).unwrap() })
    }
}

// 🔀️ `pub` (was private): now named in `impl DirectoryTransport for FakeTransport`'s public
// `type Ws = FakeWs;` associated type.
pub struct FakeWs {
    frames: VecDeque<Result<Option<String>, TransportError>>,
    close_frame: Option<Option<u16>>,
    closes: Option<Arc<AtomicUsize>>,
    sends: Option<Arc<AtomicUsize>>,
}

impl FakeWs {
    /// 🧪️ Creates a late-dial socket whose close is observable by the cancellation law.
    pub fn with_close_observer(closes: Arc<AtomicUsize>) -> Self {
        Self { frames: VecDeque::new(), close_frame: None, closes: Some(closes), sends: None }
    }

    /// 🛑️ Creates a socket whose next receive preserves one close code.
    pub fn with_close_code(code: u16) -> Self {
        Self { frames: VecDeque::new(), close_frame: Some(Some(code)), closes: None, sends: None }
    }
}

impl DirectoryWsConnection for FakeWs {
    fn send_text(&mut self, _text: String) -> Result<(), TransportError> {
        Ok(())
    }

    fn send_binary(&mut self, _bytes: Vec<u8>) -> Result<(), TransportError> {
        if let Some(sends) = &self.sends {
            sends.fetch_add(1, Ordering::SeqCst);
        }
        Ok(())
    }

    fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> {
        if let Some(code) = self.close_frame.take() {
            return Ok(DirectoryWsPoll::Closed(code));
        }
        match self.frames.pop_front() {
            Some(Ok(Some(text))) => Ok(DirectoryWsPoll::Text(text)),
            Some(Ok(None)) => Ok(DirectoryWsPoll::Closed(None)),
            Some(Err(error)) => Err(error),
            None => Ok(DirectoryWsPoll::Pending),
        }
    }

    fn close(&mut self) {
        if let Some(closes) = &self.closes {
            closes.fetch_add(1, Ordering::SeqCst);
        }
    }
}

impl super::DirectoryTransport for FakeTransport {
    type Ws = FakeWs;
    async fn http(&self, ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
        for _ in 0..self.yields_before_response.load(Ordering::SeqCst) {
            if ctx.cancel.is_cancelled().await {
                return Err(TransportError::Cancelled);
            }
            semio_framework_async::yield_once().await;
        }
        if ctx.cancel.is_cancelled().await {
            return Err(TransportError::Cancelled);
        }
        self.requests.lock().unwrap().push(RecordedRequest { method, url: url.to_string(), bearer: bearer.map(str::to_string), body: body.unwrap_or_default() });
        self.responses.lock().unwrap().pop_front().unwrap_or_else(|| Err(TransportError::Io("no scripted response".to_string())))
    }

    fn issue_socket_grant(&self, ctx: &OperationContext, url: &str, bearer: &str, body: &[u8], _timeout_ms: u64) -> Result<HttpResponse, TransportError> {
        if ctx.cancel.is_cancelled_now() {
            return Err(TransportError::Cancelled);
        }
        let request_number = {
            let mut requests = self.requests.lock().unwrap();
            requests.push(RecordedRequest { method: HttpMethod::Post, url: url.to_string(), bearer: Some(bearer.to_string()), body: body.to_vec() });
            requests.len()
        };
        let response = self.responses.lock().unwrap().pop_front().unwrap_or_else(|| Err(TransportError::Io("no scripted response".to_string())));
        if self.cancel_after_grant.load(Ordering::SeqCst) || self.cancel_after_grant_number.load(Ordering::SeqCst) == request_number {
            ctx.cancel.cancel_now();
        }
        response
    }

    fn open_ws(&self, ctx: &OperationContext, url: &str, _protocols: &[String], _timeout_ms: u64) -> Result<Self::Ws, TransportError> {
        if ctx.cancel.is_cancelled_now() {
            return Err(TransportError::Cancelled);
        }
        self.ws_urls.lock().unwrap().push(url.to_string());
        let frames = self.ws_outcomes.lock().unwrap().pop_front().unwrap_or_else(|| Err(TransportError::Io("no scripted ws".to_string())))?;
        if self.cancel_after_open.load(Ordering::SeqCst) {
            ctx.cancel.cancel_now();
        }
        Ok(FakeWs { frames, close_frame: None, closes: Some(self.ws_closes.clone()), sends: Some(self.ws_sends.clone()) })
    }
}
