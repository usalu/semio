//! 🚦️ Routing: request decoding, method routing and the notification handlers.
//!
//! Holds the `🚦️Routing` region of the crate root; the root re-exports its items.

use super::*;

//#region 🚦️Routing

/// 🤝️ Settles the handshake latch on every path an `initialize` dispatch can leave by, including the
/// early refusals that never reach the router.
struct HandshakeSettle<'a>(&'a Session);

impl Drop for HandshakeSettle<'_> {
    fn drop(&mut self) {
        self.0.settle_handshake();
    }
}

impl Session {
    /// 📬️ Handles one framed payload and returns the frame to write back, if any.
    pub fn dispatch(&self, payload: &[u8]) -> Result<Option<String>, Error> {
        let request = match self.decode_request(payload) {
            Ok(request) => request,
            Err((id_raw, error)) => return Ok(Some(self.encode_error(&id_raw, &error))),
        };
        let Some(id) = request.id.clone() else {
            self.handle_notification(&request, payload)?;
            return Ok(None);
        };
        let _settle = if request.method == "initialize" {
            Some(HandshakeSettle(self))
        } else {
            self.await_handshake();
            None
        };
        let key = id.key();
        let context = match self.begin(&key) {
            Ok(context) => context,
            Err(error) => {
                let response = self.encode_error(&request.id_raw, &error);
                self.commit_exchange(&key, payload, &response)?;
                return Ok(Some(response));
            }
        };
        let outcome = self.route(&context, &request);
        self.finish(&key);
        let response = match outcome {
            Ok(result) => self.encode_result(&request.id_raw, &result),
            Err(error) => self.encode_error(&request.id_raw, &error),
        };
        self.commit_exchange(&key, payload, &response)?;
        Ok(Some(response))
    }

    fn route(&self, context: &Context, request: &Request) -> Result<J, RpcError> {
        if context.is_cancelled() {
            return Err(RpcError::new(CODE_REQUEST_CANCELLED, "request cancelled"));
        }
        let phase = self.lock().phase;
        if request.method == "initialize" {
            if phase != Phase::Connected {
                return Err(RpcError::new(CODE_INVALID_REQUEST, "already initialized"));
            }
            let invalid = || RpcError::new(CODE_INVALID_PARAMS, "invalid initialize params");
            let members = decode_params(&request.params).map_err(|_| invalid())?;
            let protocol = members.get("protocolVersion").and_then(serde_json::Value::as_str).unwrap_or_default();
            let info = members.get("clientInfo").and_then(serde_json::Value::as_object);
            let name = info.and_then(|info| info.get("name")).and_then(serde_json::Value::as_str).unwrap_or_default();
            let version = info.and_then(|info| info.get("version")).and_then(serde_json::Value::as_str).unwrap_or_default();
            if protocol.is_empty() || name.is_empty() || version.is_empty() {
                return Err(invalid());
            }
            let mut state = self.lock();
            if state.phase != Phase::Connected {
                return Err(RpcError::new(CODE_INVALID_REQUEST, "already initialized"));
            }
            state.phase = if std::mem::take(&mut state.pending_initialized) { Phase::Ready } else { Phase::Initialized };
            drop(state);
            return Ok(J::object(vec![
                ("protocolVersion", Some(J::Str(negotiate_protocol_version(protocol).to_string()))),
                ("capabilities", Some(self.server.capabilities())),
                (
                    "serverInfo",
                    Some(J::Obj(vec![("name".to_string(), J::Str(self.server.config.server_name.clone())), ("version".to_string(), J::Str(self.server.config.server_version.clone()))])),
                ),
                ("instructions", (!self.server.config.instructions.is_empty()).then(|| J::Str(self.server.config.instructions.clone()))),
            ]));
        }
        if phase != Phase::Ready {
            return Err(RpcError::new(CODE_NOT_INITIALIZED, "session not initialized"));
        }
        match request.method.as_str() {
            "ping" => {
                decode_params(&request.params).map_err(|_| RpcError::new(CODE_INVALID_PARAMS, "invalid params"))?;
                Ok(J::Obj(Vec::new()))
            }
            "tools/list" => self.list(&request.params, "tools", |server| server.tools.values().map(|(schema, _)| schema.json()).collect()),
            "tools/call" => self.call_tool(context, request),
            "resources/list" => self.list(&request.params, "resources", |server| server.resources.values().map(|(schema, _)| schema.json()).collect()),
            "resources/templates/list" => self.list(&request.params, "resourceTemplates", |server| server.templates.values().map(ResourceTemplate::json).collect()),
            "resources/read" => self.read_resource(context, request),
            "prompts/list" => self.list(&request.params, "prompts", |server| server.prompts.values().map(|(schema, _)| schema.json()).collect()),
            "prompts/get" => self.get_prompt(context, request),
            _ => Err(RpcError::new(CODE_METHOD_NOT_FOUND, "method not found")),
        }
    }

    fn list(&self, params: &serde_json::Value, field: &str, collect: impl Fn(&Server) -> Vec<J>) -> Result<J, RpcError> {
        let members = decode_params(params).map_err(|_| RpcError::new(CODE_INVALID_PARAMS, "invalid list params"))?;
        let cursor = members.get("cursor").and_then(serde_json::Value::as_str).unwrap_or_default();
        let offset = if cursor.is_empty() { 0 } else { cursor.parse::<usize>().map_err(|_| RpcError::new(CODE_INVALID_PARAMS, "invalid cursor"))? };
        let items = collect(&self.server);
        let (end, next) = page(offset, items.len(), self.server.config.limits.max_page_items).ok_or_else(|| RpcError::new(CODE_INVALID_PARAMS, "invalid cursor"))?;
        Ok(J::object(vec![(field, Some(J::Arr(items[offset..end].to_vec()))), ("nextCursor", next.map(J::Str))]))
    }

    fn call_tool(&self, context: &Context, request: &Request) -> Result<J, RpcError> {
        let invalid = || RpcError::new(CODE_INVALID_PARAMS, "invalid tool params");
        let members = decode_params(&request.params).map_err(|_| invalid())?;
        let name = members.get("name").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
        if name.is_empty() {
            return Err(invalid());
        }
        let arguments = members.get("arguments").cloned().unwrap_or(serde_json::Value::Object(serde_json::Map::new()));
        if !arguments.is_object() {
            return Err(RpcError::new(CODE_INVALID_PARAMS, "invalid tool arguments"));
        }
        let params = CallToolParams { name: name.clone(), arguments, progress_token: progress_token(members) };
        let Some((_, handler)) = self.server.tools.get(&name) else {
            return Err(RpcError::new(CODE_INVALID_PARAMS, "tool not found"));
        };
        let progress = self.progress(params.progress_token.clone());
        let result = handler(context, &params, &progress).map_err(|error| handler_rpc_error(context, &error))?;
        if context.is_cancelled() {
            return Err(RpcError::new(CODE_REQUEST_CANCELLED, "request cancelled"));
        }
        Ok(result.json())
    }

    fn read_resource(&self, context: &Context, request: &Request) -> Result<J, RpcError> {
        let invalid = || RpcError::new(CODE_INVALID_PARAMS, "invalid resource params");
        let members = decode_params(&request.params).map_err(|_| invalid())?;
        let uri = members.get("uri").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
        if uri.is_empty() {
            return Err(invalid());
        }
        let Some((_, handler)) = self.server.resources.get(&uri) else {
            return Err(RpcError::new(CODE_INVALID_PARAMS, "resource not found"));
        };
        let progress = self.progress(progress_token(members));
        let contents = handler(context, &uri, &progress).map_err(|error| handler_rpc_error(context, &error))?;
        if context.is_cancelled() {
            return Err(RpcError::new(CODE_REQUEST_CANCELLED, "request cancelled"));
        }
        Ok(J::Obj(vec![("contents".to_string(), J::Arr(contents.iter().map(ResourceContent::json).collect()))]))
    }

    fn get_prompt(&self, context: &Context, request: &Request) -> Result<J, RpcError> {
        let invalid = || RpcError::new(CODE_INVALID_PARAMS, "invalid prompt params");
        let members = decode_params(&request.params).map_err(|_| invalid())?;
        let name = members.get("name").and_then(serde_json::Value::as_str).unwrap_or_default().to_string();
        if name.is_empty() {
            return Err(invalid());
        }
        let mut arguments = BTreeMap::new();
        if let Some(given) = members.get("arguments").and_then(serde_json::Value::as_object) {
            for (key, value) in given {
                arguments.insert(key.clone(), value.as_str().unwrap_or_default().to_string());
            }
        }
        let Some((_, handler)) = self.server.prompts.get(&name) else {
            return Err(RpcError::new(CODE_INVALID_PARAMS, "prompt not found"));
        };
        let result = handler(context, &name, &arguments).map_err(|error| handler_rpc_error(context, &error))?;
        if context.is_cancelled() {
            return Err(RpcError::new(CODE_REQUEST_CANCELLED, "request cancelled"));
        }
        Ok(result.json())
    }

    fn progress(&self, token: Option<String>) -> Progress<'_> {
        Progress { session: self, token }
    }

    fn handle_notification(&self, request: &Request, payload: &[u8]) -> Result<(), Error> {
        match request.method.as_str() {
            "notifications/initialized" => {
                let mut state = self.lock();
                match state.phase {
                    Phase::Initialized => state.phase = Phase::Ready,
                    Phase::Connected => state.pending_initialized = true,
                    _ => {}
                }
            }
            "notifications/cancelled" => {
                if let Ok(members) = decode_params(&request.params) {
                    if let Some(id) = members.get("requestId").and_then(Id::from_value) {
                        let reason = members.get("reason").and_then(serde_json::Value::as_str).unwrap_or("cancelled by peer");
                        self.cancel(&id.key(), reason);
                    }
                }
            }
            _ => {}
        }
        self.server.log.commit(&[EventInput::new("notification.received", &self.peer, self.generation, compact(payload))])
    }

    pub(super) fn decode_request(&self, payload: &[u8]) -> Result<Request, (String, RpcError)> {
        let null = || "null".to_string();
        if payload.len() > self.server.config.limits.max_payload_bytes {
            return Err((null(), RpcError::new(CODE_PAYLOAD_TOO_LARGE, "payload too large")));
        }
        match validate_nesting(payload, self.server.config.limits.max_nesting) {
            Err(Error::NestingTooDeep) => return Err((null(), RpcError::new(CODE_INVALID_REQUEST, "nesting too deep"))),
            Err(_) => return Err((null(), RpcError::new(CODE_PARSE_ERROR, "parse error"))),
            Ok(()) => {}
        }
        let mut stream = serde_json::Deserializer::from_slice(payload).into_iter::<serde_json::Value>();
        let Some(Ok(document)) = stream.next() else {
            return Err((null(), RpcError::new(CODE_PARSE_ERROR, "parse error")));
        };
        if stream.next().is_some() {
            return Err((null(), RpcError::new(CODE_INVALID_REQUEST, "trailing data")));
        }
        let Some(members) = document.as_object() else {
            return Err((null(), RpcError::new(CODE_INVALID_REQUEST, "invalid request")));
        };
        for name in members.keys() {
            if !matches!(name.as_str(), "jsonrpc" | "id" | "method" | "params") {
                return Err((null(), RpcError::new(CODE_INVALID_REQUEST, "invalid request")));
            }
        }
        let id_raw = members.get("id").map_or_else(null, |id| compact(serde_json::to_string(id).unwrap_or_default().as_bytes()));
        let id = match members.get("id") {
            None => None,
            Some(value) => match Id::from_value(value) {
                Some(id) => Some(id),
                None => return Err((null(), RpcError::new(CODE_INVALID_REQUEST, "invalid request id"))),
            },
        };
        let request = Request {
            jsonrpc: members.get("jsonrpc").and_then(serde_json::Value::as_str).unwrap_or_default().to_string(),
            id,
            id_raw: id_raw.clone(),
            method: members.get("method").and_then(serde_json::Value::as_str).unwrap_or_default().to_string(),
            params: members.get("params").cloned().unwrap_or(serde_json::Value::Object(serde_json::Map::new())),
        };
        if request.jsonrpc != JSONRPC_VERSION || request.method.is_empty() {
            return Err((id_raw, RpcError::new(CODE_INVALID_REQUEST, "invalid request")));
        }
        Ok(request)
    }

    fn encode_result(&self, id_raw: &str, result: &J) -> String {
        let encoded = format!("{{\"jsonrpc\":\"2.0\",\"id\":{id_raw},\"result\":{}}}", result.text());
        let limits = self.server.config.limits;
        if encoded.len() > limits.max_payload_bytes || validate_nesting(encoded.as_bytes(), limits.max_nesting).is_err() {
            return self.encode_error(id_raw, &RpcError::new(CODE_PAYLOAD_TOO_LARGE, "response too large"));
        }
        encoded
    }

    fn encode_error(&self, id_raw: &str, error: &RpcError) -> String {
        let encoded = format!("{{\"jsonrpc\":\"2.0\",\"id\":{id_raw},\"error\":{}}}", error.json().text());
        let limits = self.server.config.limits;
        if encoded.len() <= limits.max_payload_bytes && validate_nesting(encoded.as_bytes(), limits.max_nesting).is_ok() {
            return encoded;
        }
        format!("{{\"jsonrpc\":\"2.0\",\"id\":{id_raw},\"error\":{}}}", RpcError::new(CODE_PAYLOAD_TOO_LARGE, "response too large").json().text())
    }

    fn commit_exchange(&self, key: &str, request: &[u8], response: &str) -> Result<(), Error> {
        let mut received = EventInput::new("request.received", &self.peer, self.generation, compact(request));
        received.request_id = key.to_string();
        let mut sent = EventInput::new("response.sent", &self.peer, self.generation, compact(response.as_bytes()));
        sent.request_id = key.to_string();
        self.server.log.commit(&[received, sent])
    }

    pub(super) fn reject(&self, payload: &[u8], rejection: RpcError) -> Result<String, Error> {
        let (id_raw, key, error) = match self.decode_request(payload) {
            Ok(request) => (request.id_raw.clone(), request.id.map(|id| id.key()).unwrap_or_default(), rejection),
            Err((id_raw, protocol)) => (id_raw, String::new(), protocol),
        };
        let response = self.encode_error(&id_raw, &error);
        self.commit_exchange(&key, payload, &response)?;
        Ok(response)
    }
}

pub(super) fn progress_token(members: &serde_json::Map<String, serde_json::Value>) -> Option<String> {
    members.get("_meta").and_then(serde_json::Value::as_object).and_then(|meta| meta.get("progressToken")).map(|token| compact(serde_json::to_string(token).unwrap_or_default().as_bytes()))
}

fn handler_rpc_error(context: &Context, error: &HandlerError) -> RpcError {
    if context.is_cancelled() {
        return RpcError::new(CODE_REQUEST_CANCELLED, "request cancelled");
    }
    if (-32099..=-32000).contains(&error.code) && !error.message.is_empty() {
        return RpcError { code: error.code, message: error.message.clone(), data: error.data.clone() };
    }
    RpcError::new(CODE_INTERNAL_ERROR, "internal error")
}

/// 📄️ Computes the end index and the next cursor of one page.
pub fn page(offset: usize, length: usize, size: usize) -> Option<(usize, Option<String>)> {
    if offset > length {
        return None;
    }
    let end = offset + size;
    if end >= length {
        return Some((length, None));
    }
    Some((end, Some(end.to_string())))
}

/// 🪆️ Rejects a frame whose bracket nesting exceeds the configured depth or is unbalanced.
pub fn validate_nesting(payload: &[u8], maximum: usize) -> Result<(), Error> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    for byte in payload {
        if in_string {
            if escaped {
                escaped = false;
            } else if *byte == b'\\' {
                escaped = true;
            } else if *byte == b'"' {
                in_string = false;
            }
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => {
                depth += 1;
                if depth > maximum {
                    return Err(Error::NestingTooDeep);
                }
            }
            b'}' | b']' => {
                if depth == 0 {
                    return Err(Error::Invalid("invalid nesting".to_string()));
                }
                depth -= 1;
            }
            _ => {}
        }
    }
    if depth != 0 || in_string {
        return Err(Error::Invalid("invalid nesting".to_string()));
    }
    Ok(())
}

//#endregion 🚦️Routing
