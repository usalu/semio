//! 📡️ Event routes of the coordinator: the warning, breach, scope and event query routes and the publish route. See [`super`].

use super::*;

//#region 🌐️Http
impl Server {
    pub(super) fn warnings(&self, request: &Request) -> Response {
        let ticket = request.query.get("ticket_id").cloned().unwrap_or_default();
        let warnings: Vec<Value> = self.repository.project_warnings(&ticket).iter().map(Warning::to_value).collect();
        Response::json(200, &Value::Array(warnings))
    }

    pub(super) fn breachs(&self, request: &Request) -> Response {
        let ticket = request.query.get("ticket_id").cloned().unwrap_or_default();
        Response::json(200, &Value::Array(self.repository.project_breachs(&ticket)))
    }

    pub(super) fn scopes(&self, request: &Request) -> Response {
        let Some(file) = request.query.get("file").filter(|value| !value.is_empty()) else {
            return Response::error(400, "file query required");
        };
        let scopes: Vec<Value> = self.repository.project_scopes_by_file(file).iter().map(Scope::to_value).collect();
        Response::json(200, &Value::Array(scopes))
    }

    pub(super) fn events(&self, request: &Request) -> Response {
        let Some(payload) = request.json() else {
            return Response::error(400, "invalid JSON body");
        };
        let kind = payload.get("kind").and_then(Value::as_str).unwrap_or_default().to_string();
        if kind.is_empty() {
            return Response::error(400, "kind required");
        }
        let source = payload.get("source").and_then(Value::as_str).unwrap_or_default().to_string();
        let body = payload.get("payload").cloned().unwrap_or(Value::Null);
        match self.publish(&kind, &source, &body) {
            Err(failure) => Response::error(500, &failure.to_string()),
            Ok(()) => {
                let mut object = Map::new();
                object.insert("status".to_string(), Value::String("ok".to_string()));
                Response::json(200, &Value::Object(object))
            }
        }
    }

    /// 📬️ Persists one in-process event and runs the notification handlers that subscribe to it.
    ///
    /// The Go twin routes this through a buffered channel whose `Publish` blocks for the handler
    /// result, so the observable contract is a synchronous dispatch — which is what this is.
    pub fn publish(&self, kind: &str, source: &str, payload: &Value) -> Result<(), StoreError> {
        let mut object = Map::new();
        object.insert("id".to_string(), Value::String(new_id()));
        object.insert("type".to_string(), Value::String(kind.to_string()));
        object.insert("source".to_string(), Value::String(source.to_string()));
        object.insert("payload_json".to_string(), Value::String(to_go_json(payload)));
        object.insert("created_at".to_string(), Value::String(now_rfc3339()));
        let event = Value::Object(object);
        self.repository.record_published_event(&event)?;
        self.notify(kind, payload);
        Ok(())
    }
}
//#endregion 🌐️Http
