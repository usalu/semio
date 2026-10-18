//! 🌉️ Browser transport of the directory client: the web_sys fetch and WebSocket bridge behind the directory transport seam. See [`super`].

use super::{DirectoryTransport, DirectoryWsConnection, DirectoryWsPoll, HttpMethod, HttpResponse, TransportError};
use semio_framework_async::browser::JsFuture;
use semio_framework_async::OperationContext;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{BinaryType, CloseEvent, MessageEvent, RequestInit, Response, WebSocket};

enum BrowserDirectoryFrame {
    Text(String),
    Closed(Option<u16>),
}

#[derive(Clone)]
pub struct BrowserDirectoryTransport;

impl Default for BrowserDirectoryTransport {
    fn default() -> Self {
        Self
    }
}

impl DirectoryTransport for BrowserDirectoryTransport {
    type Ws = BrowserWsConnection;
    async fn http(&self, ctx: &OperationContext, method: HttpMethod, url: &str, bearer: Option<&str>, body: Option<Vec<u8>>) -> Result<HttpResponse, TransportError> {
        if ctx.cancel.is_cancelled().await {
            return Err(TransportError::Cancelled);
        }
        let window = web_sys::window().ok_or_else(|| TransportError::Io("no window".to_string()))?;
        let init = RequestInit::new();
        init.set_method(match method {
            HttpMethod::Get => "GET",
            HttpMethod::Post => "POST",
        });
        if let Some(bytes) = &body {
            let array = js_sys::Uint8Array::from(bytes.as_slice());
            init.set_body(&array);
        }
        let request = web_sys::Request::new_with_str_and_init(url, &init).map_err(|error| TransportError::Io(format!("{error:?}")))?;
        if let Some(token) = bearer {
            request.headers().set("Authorization", &format!("Bearer {token}")).map_err(|error| TransportError::Io(format!("{error:?}")))?;
        }
        let response_value = JsFuture::from(window.fetch_with_request(&request)).await.map_err(|error| TransportError::Io(format!("{error:?}")))?;
        let response: Response = response_value.dyn_into().map_err(|error| TransportError::Io(format!("{error:?}")))?;
        let status = response.status();
        let array_buffer = JsFuture::from(response.array_buffer().map_err(|error| TransportError::Io(format!("{error:?}")))?).await.map_err(|error| TransportError::Io(format!("{error:?}")))?;
        let bytes = js_sys::Uint8Array::new(&array_buffer).to_vec();
        Ok(HttpResponse { status, body: bytes })
    }

    async fn get_accepting(&self, ctx: &OperationContext, url: &str, bearer: Option<&str>, accept: &str) -> Result<HttpResponse, TransportError> {
        if ctx.cancel.is_cancelled().await {
            return Err(TransportError::Cancelled);
        }
        let window = web_sys::window().ok_or_else(|| TransportError::Io("no window".to_string()))?;
        let init = RequestInit::new();
        init.set_method("GET");
        let request = web_sys::Request::new_with_str_and_init(url, &init).map_err(|error| TransportError::Io(format!("{error:?}")))?;
        request.headers().set("Accept", accept).map_err(|error| TransportError::Io(format!("{error:?}")))?;
        if let Some(token) = bearer {
            request.headers().set("Authorization", &format!("Bearer {token}")).map_err(|error| TransportError::Io(format!("{error:?}")))?;
        }
        let response: Response = JsFuture::from(window.fetch_with_request(&request)).await.map_err(|error| TransportError::Io(format!("{error:?}")))?.dyn_into().map_err(|error| TransportError::Io(format!("{error:?}")))?;
        let status = response.status();
        let array_buffer = JsFuture::from(response.array_buffer().map_err(|error| TransportError::Io(format!("{error:?}")))?).await.map_err(|error| TransportError::Io(format!("{error:?}")))?;
        Ok(HttpResponse { status, body: js_sys::Uint8Array::new(&array_buffer).to_vec() })
    }

    fn issue_socket_grant(&self, _ctx: &OperationContext, _url: &str, _bearer: &str, _body: &[u8], _timeout_ms: u64) -> Result<HttpResponse, TransportError> {
        Err(TransportError::Io("browser directory socket issuance requires the TypeScript credential relay".into()))
    }

    fn open_ws(&self, ctx: &OperationContext, url: &str, protocols: &[String], _timeout_ms: u64) -> Result<Self::Ws, TransportError> {
        if ctx.cancel.is_cancelled_now() {
            return Err(TransportError::Cancelled);
        }
        let protocol_values = js_sys::Array::new();
        for protocol in protocols {
            protocol_values.push(&JsValue::from_str(protocol));
        }
        let socket = WebSocket::new_with_str_sequence(url, &protocol_values).map_err(|error| TransportError::Io(format!("{error:?}")))?;
        socket.set_binary_type(BinaryType::Blob);
        let (incoming_tx, incoming_rx) = tokio::sync::mpsc::unbounded_channel::<BrowserDirectoryFrame>();
        let message_tx = incoming_tx.clone();
        let onmessage = Closure::wrap(Box::new(move |event: MessageEvent| {
            if let Some(text) = event.data().as_string() {
                let _ = message_tx.send(BrowserDirectoryFrame::Text(text));
            }
        }) as Box<dyn FnMut(MessageEvent)>);
        let onclose = Closure::wrap(Box::new(move |event: CloseEvent| {
            let _ = incoming_tx.send(BrowserDirectoryFrame::Closed(Some(event.code())));
        }) as Box<dyn FnMut(CloseEvent)>);
        socket.set_onmessage(Some(onmessage.as_ref().unchecked_ref()));
        socket.set_onclose(Some(onclose.as_ref().unchecked_ref()));
        Ok(BrowserWsConnection { socket, _onmessage: onmessage, _onclose: onclose, incoming_rx })
    }
}

pub struct BrowserWsConnection {
    socket: WebSocket,
    _onmessage: Closure<dyn FnMut(MessageEvent)>,
    _onclose: Closure<dyn FnMut(CloseEvent)>,
    incoming_rx: tokio::sync::mpsc::UnboundedReceiver<BrowserDirectoryFrame>,
}

impl DirectoryWsConnection for BrowserWsConnection {
    fn send_text(&mut self, text: String) -> Result<(), TransportError> {
        self.socket.send_with_str(&text).map_err(|error| TransportError::Io(format!("{error:?}")))
    }

    fn send_binary(&mut self, bytes: Vec<u8>) -> Result<(), TransportError> {
        self.socket.send_with_u8_array(&bytes).map_err(|error| TransportError::Io(format!("{error:?}")))
    }

    fn try_recv_text(&mut self) -> Result<DirectoryWsPoll, TransportError> {
        match self.incoming_rx.try_recv() {
            Ok(BrowserDirectoryFrame::Text(text)) => Ok(DirectoryWsPoll::Text(text)),
            Ok(BrowserDirectoryFrame::Closed(code)) => Ok(DirectoryWsPoll::Closed(code)),
            Err(tokio::sync::mpsc::error::TryRecvError::Empty) => Ok(DirectoryWsPoll::Pending),
            Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => Ok(DirectoryWsPoll::Closed(None)),
        }
    }

    fn close(&mut self) {
        let _ = self.socket.close();
    }
}
