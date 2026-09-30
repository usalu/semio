//! 🌐 Schema-addressed owner contributions over bounded document HTTP transport.

use crate::OwnedJsonSchemaValidator;
use semio_framework_os_kernel::os_directory::client::{DirectoryClient, DirectoryTransport, DocumentHttpOperationV1, DocumentHttpPortCodeV1, DocumentHttpPortDeclarationV1, DocumentHttpRequestV1, HttpMethod, DOCUMENT_HTTP_PORT_TOPIC, DOCUMENT_HTTP_REQUEST_MAX_BYTES, DOCUMENT_HTTP_RESPONSE_MAX_BYTES};
use semio_framework_os_kernel::{DslValue, os_directory::DocumentScope};
use semio_framework_async::OperationContext;

/// 🔐 Compiled schemas remain bound to one installed owner and one declared service.
#[derive(Clone)]
pub struct CompiledDocumentHttpPortV1 {
    declaration: DocumentHttpPortDeclarationV1,
    schemas: Vec<(OwnedJsonSchemaValidator, OwnedJsonSchemaValidator)>,
}

impl CompiledDocumentHttpPortV1 {
    /// 🧾 Compiles only bounded, closed owner declarations; missing and foreign schemas fail closed.
    pub fn compile(owner: &str, declaration: DocumentHttpPortDeclarationV1) -> Result<Self, DocumentHttpPortCodeV1> {
        if declaration.schema != DOCUMENT_HTTP_PORT_TOPIC || declaration.owner != owner || owner.is_empty() || owner.len() > 128 || declaration.service_id.is_empty() || declaration.service_id.len() > 256 || declaration.operations.is_empty() || declaration.operations.len() > 8 {
            return Err(DocumentHttpPortCodeV1::Invalid);
        }
        let declaration_schema = OwnedJsonSchemaValidator::compile(include_str!("../../../🛍️products/💻️os/🔨️modules/📇️directory/🔌️client/🧬️schema/🔣️.json")).map_err(|_| DocumentHttpPortCodeV1::Invalid)?;
        declaration_schema.validate_json(&pack::json::to_json_string(&declaration)).map_err(|_| DocumentHttpPortCodeV1::Invalid)?;
        let mut schemas = Vec::new();
        let mut actions = std::collections::HashSet::new();
        for operation in &declaration.operations {
            if !actions.insert(operation.action.clone()) || operation.action.is_empty() || operation.action.len() > 64 || !matches!(operation.method.as_str(), "GET" | "POST") || operation.route.is_empty() || operation.route.len() > 8 || operation.route.iter().any(|segment| segment.is_empty() || segment.len() > 256 || matches!(segment.as_str(), "." | "..")) || operation.request_max_bytes == 0 || operation.request_max_bytes > DOCUMENT_HTTP_REQUEST_MAX_BYTES || operation.response_max_bytes == 0 || operation.response_max_bytes > DOCUMENT_HTTP_RESPONSE_MAX_BYTES || (operation.method == "GET" && operation.send_body) {
                return Err(DocumentHttpPortCodeV1::Invalid);
            }
            let mut validators = Vec::new();
            for source in [&operation.input_schema, &operation.output_schema] {
                if source.len() > 32 * 1024 { return Err(DocumentHttpPortCodeV1::Bounds); }
                let schema = pack::json::parse(source).map_err(|_| DocumentHttpPortCodeV1::Invalid)?;
                if !schema.get("$id").and_then(pack::json::Value::as_str).is_some_and(|id| id.starts_with(&format!("{owner}:"))) { return Err(DocumentHttpPortCodeV1::Invalid); }
                validators.push(OwnedJsonSchemaValidator::compile(source).map_err(|_| DocumentHttpPortCodeV1::Invalid)?);
            }
            let output = validators.pop().ok_or(DocumentHttpPortCodeV1::Invalid)?;
            let input = validators.pop().ok_or(DocumentHttpPortCodeV1::Invalid)?;
            schemas.push((input, output));
        }
        Ok(Self { declaration, schemas })
    }

    pub fn service_id(&self) -> &str { &self.declaration.service_id }

    /// 📡 Validates before sending and before exposing any response to a consumer.
    pub async fn call<T: DirectoryTransport>(&self, client: &DirectoryClient<T>, ctx: &OperationContext, scope: &DocumentScope, action: &str, payload: &DslValue) -> Result<DslValue, DocumentHttpPortCodeV1> {
        let index = self.declaration.operations.iter().position(|operation| operation.action == action).ok_or(DocumentHttpPortCodeV1::Unavailable)?;
        let operation = &self.declaration.operations[index];
        let source = pack::json::to_json_string(payload);
        if source.len() > operation.request_max_bytes { return Err(DocumentHttpPortCodeV1::Bounds); }
        self.schemas[index].0.validate_json(&source).map_err(|_| DocumentHttpPortCodeV1::Invalid)?;
        let value = pack::json::parse(&source).map_err(|_| DocumentHttpPortCodeV1::Invalid)?;
        let request = request(operation, &value, source.into_bytes())?;
        client.document_http_call(ctx, scope, &request, |bytes| {
            let source = std::str::from_utf8(bytes).map_err(|_| DocumentHttpPortCodeV1::Invalid)?;
            self.schemas[index].1.validate_json(source).map_err(|_| DocumentHttpPortCodeV1::Invalid)?;
            pack::json::from_json_str(source).map_err(|_| DocumentHttpPortCodeV1::Invalid)
        }).await
    }
}

fn request(operation: &DocumentHttpOperationV1, input: &pack::json::Value, bytes: Vec<u8>) -> Result<DocumentHttpRequestV1, DocumentHttpPortCodeV1> {
    let mut segments = Vec::new();
    for segment in &operation.route {
        let value = if let Some(field) = segment.strip_prefix('{').and_then(|value| value.strip_suffix('}')) {
            input.get(field).and_then(pack::json::Value::as_str).ok_or(DocumentHttpPortCodeV1::Invalid)?.to_string()
        } else { segment.clone() };
        segments.push(value);
    }
    let after = operation.cursor_field.as_ref().map(|field| input.get(field).and_then(pack::json::Value::as_u64).ok_or(DocumentHttpPortCodeV1::Invalid)).transpose()?;
    Ok(DocumentHttpRequestV1 { method: if operation.method == "GET" { HttpMethod::Get } else { HttpMethod::Post }, segments, after, body: operation.send_body.then_some(bytes), request_max_bytes: operation.request_max_bytes, response_max_bytes: operation.response_max_bytes })
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
