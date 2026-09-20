use dsl::{FromValue, ToValue};

pub const DOCUMENT_BACKBONE_BINDING_SCHEMA_V1: &str = "semio.plugin.document-backbone-binding.v1";
pub const DOCUMENT_BACKBONE_BINDING_RECEIPT_SCHEMA_V1: &str = "semio.plugin.document-backbone-binding-receipt.v1";
pub const DOCUMENT_BACKBONE_BINDING_CONTROL_MAXIMUM_BYTES: usize = 4 * 1024;
pub const DOCUMENT_BACKBONE_BINDING_URI_MAXIMUM_BYTES: usize = 1_280;
pub const DOCUMENT_BACKBONE_BINDING_CODE_MAXIMUM_BYTES: usize = 256;
pub const DOCUMENT_BACKBONE_PENDING_MAXIMUM_BYTES: usize = store::BACKBONE_CHANNEL_MAXIMUM_BYTES;
pub const DOCUMENT_BACKBONE_PENDING_MAXIMUM_MESSAGES: usize = store::BACKBONE_CHANNEL_MAXIMUM_MESSAGES;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentBackboneBindingOperationV1 {
    Bind,
    Retire,
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, FromValue, ToValue)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
struct DocumentBackboneBindingWireV1 {
    schema: String,
    operation: String,
    instance_id: u32,
    binding_generation: u64,
    uri: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentBackboneBindingCommandV1 {
    pub operation: DocumentBackboneBindingOperationV1,
    pub instance_id: u32,
    pub binding_generation: u64,
    pub uri: String,
}

impl DocumentBackboneBindingCommandV1 {
    pub fn encode(&self) -> Result<Vec<u8>, String> {
        if self.uri.is_empty() || self.uri.len() > DOCUMENT_BACKBONE_BINDING_URI_MAXIMUM_BYTES {
            return Err("plugin.document-backbone.binding-fields".into());
        }
        let operation = match self.operation {
            DocumentBackboneBindingOperationV1::Bind => "bind",
            DocumentBackboneBindingOperationV1::Retire => "retire",
        };
        let bytes = store::pack_rt::encode_wire_value(
            &DocumentBackboneBindingWireV1 {
                schema: DOCUMENT_BACKBONE_BINDING_SCHEMA_V1.into(),
                operation: operation.into(),
                instance_id: self.instance_id,
                binding_generation: self.binding_generation,
                uri: self.uri.clone(),
            }
            .to_value(),
        );
        if bytes.len() > DOCUMENT_BACKBONE_BINDING_CONTROL_MAXIMUM_BYTES {
            return Err("plugin.document-backbone.binding-capacity".into());
        }
        Ok(bytes)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, serde::Deserialize, serde::Serialize, FromValue, ToValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
struct DocumentBackboneBindingReceiptWireV1 {
    schema: String,
    operation: String,
    instance_id: u32,
    binding_generation: u64,
    uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[value(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DocumentBackboneBindingReceiptV1 {
    pub operation: &'static str,
    pub instance_id: u32,
    pub binding_generation: u64,
    pub uri: String,
    pub code: Option<&'static str>,
}

impl DocumentBackboneBindingReceiptV1 {
    pub fn bound(command: &DocumentBackboneBindingCommandV1) -> Self {
        Self { operation: "bound", instance_id: command.instance_id, binding_generation: command.binding_generation, uri: command.uri.clone(), code: None }
    }

    pub fn retired(command: &DocumentBackboneBindingCommandV1) -> Self {
        Self { operation: "retired", instance_id: command.instance_id, binding_generation: command.binding_generation, uri: command.uri.clone(), code: None }
    }

    pub fn refused(command: &DocumentBackboneBindingCommandV1, code: &'static str) -> Self {
        Self { operation: "refused", instance_id: command.instance_id, binding_generation: command.binding_generation, uri: command.uri.clone(), code: Some(code) }
    }

    pub fn encode(&self) -> Vec<u8> {
        store::pack_rt::encode_wire_value(
            &DocumentBackboneBindingReceiptWireV1 {
                schema: DOCUMENT_BACKBONE_BINDING_RECEIPT_SCHEMA_V1.into(),
                operation: self.operation.into(),
                instance_id: self.instance_id,
                binding_generation: self.binding_generation,
                uri: self.uri.clone(),
                code: self.code.map(str::to_string),
            }
            .to_value(),
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DocumentBackboneBindingStateV1 {
    pub generation: u64,
    pub uri: Option<String>,
}

pub enum DocumentBackboneBindingDecisionV1 {
    Bind(DocumentBackboneBindingReceiptV1),
    Replay(DocumentBackboneBindingReceiptV1),
    Retire(DocumentBackboneBindingReceiptV1),
    Refuse(DocumentBackboneBindingReceiptV1),
}

pub fn decode_document_backbone_binding_command_v1(payload: &[u8]) -> Result<Option<DocumentBackboneBindingCommandV1>, String> {
    if payload.len() > DOCUMENT_BACKBONE_BINDING_CONTROL_MAXIMUM_BYTES {
        return Ok(None);
    }
    let value = match store::pack_rt::decode_wire_value(payload) {
        Ok(value) => value,
        Err(_) => return Ok(None),
    };
    if value.get("schema").and_then(dsl::DslValue::as_str) != Some(DOCUMENT_BACKBONE_BINDING_SCHEMA_V1) {
        return Ok(None);
    }
    if store::pack_rt::encode_wire_value(&value) != payload {
        return Err("plugin.document-backbone.binding-noncanonical".into());
    }
    let wire = DocumentBackboneBindingWireV1::from_value(value).map_err(|error| error.to_string())?;
    let operation = match wire.operation.as_str() {
        "bind" => DocumentBackboneBindingOperationV1::Bind,
        "retire" => DocumentBackboneBindingOperationV1::Retire,
        _ => return Err("plugin.document-backbone.binding-operation".into()),
    };
    if wire.schema != DOCUMENT_BACKBONE_BINDING_SCHEMA_V1 || wire.uri.is_empty() || wire.uri.len() > DOCUMENT_BACKBONE_BINDING_URI_MAXIMUM_BYTES {
        return Err("plugin.document-backbone.binding-fields".into());
    }
    Ok(Some(DocumentBackboneBindingCommandV1 { operation, instance_id: wire.instance_id, binding_generation: wire.binding_generation, uri: wire.uri }))
}

pub fn require_document_backbone_binding_receipt_v1(payload: &[u8], command: &DocumentBackboneBindingCommandV1) -> Result<(), String> {
    if payload.is_empty() || payload.len() > DOCUMENT_BACKBONE_BINDING_CONTROL_MAXIMUM_BYTES {
        return Err("plugin.document-backbone.receipt-bytes".into());
    }
    let value = store::pack_rt::decode_wire_value(payload).map_err(|_| "plugin.document-backbone.receipt-codec".to_string())?;
    if store::pack_rt::encode_wire_value(&value) != payload {
        return Err("plugin.document-backbone.receipt-noncanonical".into());
    }
    let receipt = DocumentBackboneBindingReceiptWireV1::from_value(value).map_err(|error| error.to_string())?;
    if receipt.schema != DOCUMENT_BACKBONE_BINDING_RECEIPT_SCHEMA_V1
        || receipt.instance_id != command.instance_id
        || receipt.binding_generation != command.binding_generation
        || receipt.uri != command.uri
    {
        return Err("plugin.document-backbone.receipt-owner".into());
    }
    let expected = match command.operation {
        DocumentBackboneBindingOperationV1::Bind => "bound",
        DocumentBackboneBindingOperationV1::Retire => "retired",
    };
    match receipt.operation.as_str() {
        operation if operation == expected && receipt.code.is_none() => Ok(()),
        "refused" => Err(receipt
            .code
            .filter(|code| {
                !code.is_empty()
                    && code.len() <= DOCUMENT_BACKBONE_BINDING_CODE_MAXIMUM_BYTES
                    && code.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
                    && code.as_bytes().iter().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'.' | b'-'))
            })
            .ok_or_else(|| "plugin.document-backbone.receipt-code".to_string())?),
        _ => Err("plugin.document-backbone.receipt-operation".into()),
    }
}

pub fn decide_document_backbone_binding_v1(state: &DocumentBackboneBindingStateV1, command: &DocumentBackboneBindingCommandV1) -> DocumentBackboneBindingDecisionV1 {
    match command.operation {
        DocumentBackboneBindingOperationV1::Bind => match state.uri.as_deref() {
            Some(uri) if state.generation == command.binding_generation && uri == command.uri => DocumentBackboneBindingDecisionV1::Replay(DocumentBackboneBindingReceiptV1::bound(command)),
            Some(_) if state.generation == command.binding_generation => DocumentBackboneBindingDecisionV1::Refuse(DocumentBackboneBindingReceiptV1::refused(command, "plugin.document-backbone.binding-collision")),
            Some(_) if command.binding_generation > state.generation => DocumentBackboneBindingDecisionV1::Refuse(DocumentBackboneBindingReceiptV1::refused(command, "plugin.document-backbone.binding-live")),
            Some(_) => DocumentBackboneBindingDecisionV1::Refuse(DocumentBackboneBindingReceiptV1::refused(command, "plugin.document-backbone.stale-generation")),
            None if command.binding_generation > state.generation => DocumentBackboneBindingDecisionV1::Bind(DocumentBackboneBindingReceiptV1::bound(command)),
            None => DocumentBackboneBindingDecisionV1::Refuse(DocumentBackboneBindingReceiptV1::refused(command, "plugin.document-backbone.stale-generation")),
        },
        DocumentBackboneBindingOperationV1::Retire => match state.uri.as_deref() {
            Some(uri) if state.generation == command.binding_generation && uri == command.uri => DocumentBackboneBindingDecisionV1::Retire(DocumentBackboneBindingReceiptV1::retired(command)),
            Some(_) if state.generation == command.binding_generation => DocumentBackboneBindingDecisionV1::Refuse(DocumentBackboneBindingReceiptV1::refused(command, "plugin.document-backbone.binding-collision")),
            _ => DocumentBackboneBindingDecisionV1::Refuse(DocumentBackboneBindingReceiptV1::refused(command, "plugin.document-backbone.stale-generation")),
        },
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit-standalone/🦀️.rs"]
mod tests;
