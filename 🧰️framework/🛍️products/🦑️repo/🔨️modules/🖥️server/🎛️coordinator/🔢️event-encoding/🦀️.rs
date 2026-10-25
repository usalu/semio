//! 🔢️ Event encoding of the coordinator: the stage and event schemas, the event envelope and input, the store limits and the canonical Go-compatible JSON encoding with the event checksum. See [`super`].

use super::*;

//#region 🔢️Encoding
/// 📜️ The persisted schema name every stage file carries.
pub const STAGE_SCHEMA: &str = "semio.coordinator.event-stage/1";

/// 🧬️ The persisted schema name of one event envelope.
pub const EVENT_SCHEMA: &str = "semio.coordinator.event/1";

/// 🧾️ One committed record of the append-only log.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventEnvelope {
    pub stream: String,
    pub sequence: u64,
    pub id: String,
    pub generation: u64,
    pub kind: String,
    pub payload: Value,
    pub checksum: String,
}

/// 📨️ A sequence-free event proposed for an append command.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EventInput {
    pub stream: String,
    pub id: String,
    pub generation: u64,
    pub kind: String,
    pub payload: Value,
}

/// 📏️ The bounds that keep one append or replay finite.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StoreLimits {
    pub max_payload_bytes: usize,
    pub max_append_bytes: usize,
    pub max_append_events: usize,
    pub max_replay_events: usize,
    pub max_json_depth: usize,
    pub max_log_bytes: u64,
}

impl Default for StoreLimits {
    fn default() -> Self {
        Self {
            max_payload_bytes: 1 << 20,
            max_append_bytes: 8 << 20,
            max_append_events: 4096,
            max_replay_events: 1_000_000,
            max_json_depth: 64,
            max_log_bytes: 1 << 30,
        }
    }
}

/// ✍️ Encodes one JSON value exactly the way Go's `encoding/json` does.
///
/// The log is compared byte for byte across implementations, so the escaping rules must match Go's
/// and not `serde_json`'s: Go escapes `<`, `>`, `&` and U+2028/U+2029, and spells `\b`/`\f` as
/// ``/``. Object keys are emitted in sorted order, which is what Go's map marshalling
/// produces and what `canonical_payload` has already normalised every payload to.
pub fn encode_go_json(value: &Value, out: &mut String) {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Number(number) => out.push_str(&number.to_string()),
        Value::String(text) => encode_go_string(text, out),
        Value::Array(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                encode_go_json(item, out);
            }
            out.push(']');
        }
        Value::Object(entries) => {
            out.push('{');
            for (index, (key, item)) in entries.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                encode_go_string(key, out);
                out.push(':');
                encode_go_json(item, out);
            }
            out.push('}');
        }
    }
}

fn encode_go_string(text: &str, out: &mut String) {
    out.push('"');
    for character in text.chars() {
        match character {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            '\u{2028}' => out.push_str("\\u2028"),
            '\u{2029}' => out.push_str("\\u2029"),
            other if (other as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", other as u32)),
            other => out.push(other),
        }
    }
    out.push('"');
}

/// 🧮️ The Go-compatible text of one JSON value.
pub fn to_go_json(value: &Value) -> String {
    let mut out = String::new();
    encode_go_json(value, &mut out);
    out
}

/// ♻️ Normalises a payload the way the store persists it: decoded, then re-encoded with sorted keys.
pub fn canonical_payload(payload: &Value, max_depth: usize) -> Result<Value, StoreError> {
    validate_payload(payload, max_depth)?;
    Ok(payload.clone())
}

/// ✅️ Refuses an empty payload and one nested past the configured depth.
pub fn validate_payload(payload: &Value, max_depth: usize) -> Result<(), StoreError> {
    fn depth_of(value: &Value, depth: usize, max_depth: usize) -> Result<(), StoreError> {
        if depth > max_depth {
            return Err(StoreError::Limit(format!("JSON depth {depth} > {max_depth}")));
        }
        match value {
            Value::Array(items) => items.iter().try_for_each(|item| depth_of(item, depth + 1, max_depth)),
            Value::Object(entries) => entries.values().try_for_each(|item| depth_of(item, depth + 1, max_depth)),
            _ => Ok(()),
        }
    }
    match payload {
        Value::Array(_) | Value::Object(_) => depth_of(payload, 1, max_depth),
        Value::Null => Err(StoreError::Corrupt("event payload must be valid JSON".to_string())),
        _ => Ok(()),
    }
}

/// 🔏️ The SHA-256 identity of a record over its NUL-separated header and canonical payload.
///
/// `sha256(stream+nul+sequence+nul+id+nul+generation+nul+type+nul+canonical-payload)`, exactly the
/// formula frozen in the `G3EventLogContract` schema export.
pub fn event_checksum(event: &EventEnvelope) -> String {
    let mut hasher = Sha256::new();
    hasher.update(
        format!(
            "{}\u{0}{}\u{0}{}\u{0}{}\u{0}{}\u{0}",
            event.stream, event.sequence, event.id, event.generation, event.kind
        )
        .as_bytes(),
    );
    hasher.update(to_go_json(&event.payload).as_bytes());
    hasher.finish()
}

/// 🔂️ The canonical single line of one envelope, terminated by the record separator.
pub fn encode_event(event: &EventEnvelope) -> String {
    let mut out = String::from("{\"stream\":");
    encode_go_string(&event.stream, &mut out);
    out.push_str(&format!(",\"sequence\":{},\"id\":", event.sequence));
    encode_go_string(&event.id, &mut out);
    out.push_str(&format!(",\"generation\":{},\"type\":", event.generation));
    encode_go_string(&event.kind, &mut out);
    out.push_str(",\"payload\":");
    encode_go_json(&event.payload, &mut out);
    out.push_str(",\"checksum\":");
    encode_go_string(&event.checksum, &mut out);
    out.push_str("}\n");
    out
}

/// 🔎️ Reads one canonical line back into an envelope, refusing anything the encoder would not emit.
pub fn decode_event(line: &str) -> Result<EventEnvelope, StoreError> {
    let value: Value = serde_json::from_str(line).map_err(|_| StoreError::Corrupt("malformed event".to_string()))?;
    let object = value.as_object().ok_or_else(|| StoreError::Corrupt("malformed event".to_string()))?;
    let text = |key: &str| object.get(key).and_then(Value::as_str).unwrap_or_default().to_string();
    let number = |key: &str| object.get(key).and_then(Value::as_u64).unwrap_or_default();
    Ok(EventEnvelope {
        stream: text("stream"),
        sequence: number("sequence"),
        id: text("id"),
        generation: number("generation"),
        kind: text("type"),
        payload: object.get("payload").cloned().unwrap_or(Value::Null),
        checksum: text("checksum"),
    })
}

pub(super) fn valid_scalar(value: &str) -> bool {
    !value.is_empty() && !value.contains('\u{0}')
}
//#endregion 🔢️Encoding
