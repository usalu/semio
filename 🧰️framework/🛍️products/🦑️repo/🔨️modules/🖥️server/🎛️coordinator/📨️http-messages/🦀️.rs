//! 📨️ HTTP messages of the coordinator: the request, the response and the reason phrase. See [`super`].

use super::*;

//#region 🌐️Http
/// 🌐️ One parsed HTTP/1.1 request.
#[derive(Debug, Clone, Default)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Request {
    pub(super) fn header(&self, name: &str) -> String {
        self.headers.get(&name.to_ascii_lowercase()).cloned().unwrap_or_default()
    }

    pub(super) fn json(&self) -> Option<Value> {
        serde_json::from_slice(&self.body).ok()
    }
}

/// 📩️ One HTTP/1.1 response.
#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub content_type: Option<String>,
    pub body: Vec<u8>,
}

impl Response {
    pub(super) fn empty(status: u16) -> Self {
        Self { status, content_type: None, body: Vec::new() }
    }

    pub(super) fn text(status: u16, body: &str) -> Self {
        Self { status, content_type: None, body: body.as_bytes().to_vec() }
    }

    pub(super) fn json(status: u16, value: &Value) -> Self {
        Self { status, content_type: Some("application/json".to_string()), body: format!("{}\n", to_go_json(value)).into_bytes() }
    }

    pub(super) fn error(status: u16, message: &str) -> Self {
        let mut object = Map::new();
        object.insert("error".to_string(), Value::String(message.to_string()));
        Self::json(status, &Value::Object(object))
    }
}

pub(super) fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        400 => "Bad Request",
        401 => "Unauthorized",
        404 => "Not Found",
        405 => "Method Not Allowed",
        500 => "Internal Server Error",
        _ => "Status",
    }
}
//#endregion 🌐️Http
