//! 📡️ Event log: the hash-chained event log and its replay.
//!
//! Holds the `📡️Event` region of the crate root; the root re-exports its items.

use super::*;

//#region 📡️Event

/// 🏷️ The schema identifier every event record carries.
pub const EVENT_SCHEMA: &str = "semio.mcp.event/1";

/// 🔗️ One hash-chained event record.
#[derive(Clone, Debug, PartialEq)]
pub struct Event {
    pub schema: String,
    pub sequence: u64,
    pub kind: String,
    pub peer: String,
    pub generation: u64,
    pub request_id: String,
    pub payload: String,
    pub previous: String,
    pub hash: String,
}

impl Event {
    /// ✍️ Renders the record exactly as the Go implementation does, so both chains agree bit for bit.
    pub fn text(&self, hash: &str) -> String {
        let mut out = String::new();
        out.push_str("{\"schema\":");
        go_string(&self.schema, &mut out);
        out.push_str(",\"sequence\":");
        out.push_str(&self.sequence.to_string());
        out.push_str(",\"kind\":");
        go_string(&self.kind, &mut out);
        out.push_str(",\"peer\":");
        go_string(&self.peer, &mut out);
        out.push_str(",\"generation\":");
        out.push_str(&self.generation.to_string());
        if !self.request_id.is_empty() {
            out.push_str(",\"requestId\":");
            go_string(&self.request_id, &mut out);
        }
        out.push_str(",\"payload\":");
        out.push_str(&self.payload);
        if !self.previous.is_empty() {
            out.push_str(",\"previous\":");
            go_string(&self.previous, &mut out);
        }
        out.push_str(",\"hash\":");
        go_string(hash, &mut out);
        out.push('}');
        out
    }

    /// 🔏️ The chain digest of this record, taken over its own rendering with an empty `hash`.
    pub fn digest(&self) -> String {
        sha256_hex(self.text("").as_bytes())
    }
}

/// 📥️ What a caller commits into the log.
#[derive(Clone, Debug)]
pub struct EventInput {
    pub kind: String,
    pub peer: String,
    pub generation: u64,
    pub request_id: String,
    pub payload: String,
}

impl EventInput {
    pub fn new(kind: &str, peer: &str, generation: u64, payload: String) -> EventInput {
        EventInput { kind: kind.to_string(), peer: peer.to_string(), generation, request_id: String::new(), payload }
    }
}

/// 📚️ The append-only, hash-chained, bounded event log.
#[derive(Debug, Default)]
pub struct EventLog {
    pub(super) state: Mutex<EventLogState>,
    max_bytes: usize,
    max_count: usize,
}

#[derive(Debug, Default)]
pub(super) struct EventLogState {
    events: Vec<Event>,
    pub(super) data: String,
}

impl EventLog {
    pub fn new(max_bytes: usize, max_count: usize) -> EventLog {
        EventLog { state: Mutex::new(EventLogState::default()), max_bytes, max_count }
    }

    /// ✅️ Appends a whole batch or nothing at all.
    pub fn commit(&self, inputs: &[EventInput]) -> Result<(), Error> {
        if inputs.is_empty() {
            return Ok(());
        }
        let mut state = self.state.lock().unwrap_or_else(|poison| poison.into_inner());
        if self.max_count > 0 && state.events.len() + inputs.len() > self.max_count {
            return Err(Error::Limit);
        }
        let mut previous = state.events.last().map(|event| event.hash.clone()).unwrap_or_default();
        let mut staged_events = Vec::with_capacity(inputs.len());
        let mut staged = String::new();
        for (index, input) in inputs.iter().enumerate() {
            if input.kind.is_empty() || serde_json::from_str::<serde_json::Value>(&input.payload).is_err() {
                return Err(Error::Invalid("invalid event".to_string()));
            }
            let mut event = Event {
                schema: EVENT_SCHEMA.to_string(),
                sequence: (state.events.len() + index + 1) as u64,
                kind: input.kind.clone(),
                peer: input.peer.clone(),
                generation: input.generation,
                request_id: input.request_id.clone(),
                payload: compact(input.payload.as_bytes()),
                previous: previous.clone(),
                hash: String::new(),
            };
            event.hash = event.digest();
            staged.push_str(&event.text(&event.hash));
            staged.push('\n');
            previous = event.hash.clone();
            staged_events.push(event);
        }
        if self.max_bytes > 0 && state.data.len() + staged.len() > self.max_bytes {
            return Err(Error::Limit);
        }
        state.events.extend(staged_events);
        state.data.push_str(&staged);
        Ok(())
    }

    /// 📄️ The whole log as JSONL.
    pub fn snapshot(&self) -> String {
        self.state.lock().unwrap_or_else(|poison| poison.into_inner()).data.clone()
    }

    /// 📋️ Every committed record.
    pub fn events(&self) -> Vec<Event> {
        self.state.lock().unwrap_or_else(|poison| poison.into_inner()).events.clone()
    }
}

/// ✂️ Lifts one top-level member out of an already-compact JSON object without re-serialising it, so a
/// replayed payload keeps the exact bytes its digest was taken over.
pub fn raw_member(line: &str, name: &str) -> Option<String> {
    let bytes = line.as_bytes();
    let needle = format!("\"{name}\":");
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut index = 0usize;
    while index < bytes.len() {
        let byte = bytes[index];
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
            }
            index += 1;
            continue;
        }
        match byte {
            b'{' | b'[' => depth += 1,
            b'}' | b']' => depth = depth.saturating_sub(1),
            b'"' => {
                if depth == 1 && line[index..].starts_with(&needle) {
                    let start = index + needle.len();
                    let end = value_end(bytes, start)?;
                    return Some(line[start..end].to_string());
                }
                in_string = true;
            }
            _ => {}
        }
        index += 1;
    }
    None
}

fn value_end(bytes: &[u8], start: usize) -> Option<usize> {
    let mut depth = 0usize;
    let mut in_string = false;
    let mut escaped = false;
    let mut index = start;
    while index < bytes.len() {
        let byte = bytes[index];
        if in_string {
            if escaped {
                escaped = false;
            } else if byte == b'\\' {
                escaped = true;
            } else if byte == b'"' {
                in_string = false;
                if depth == 0 {
                    return Some(index + 1);
                }
            }
            index += 1;
            continue;
        }
        match byte {
            b'"' => in_string = true,
            b'{' | b'[' => depth += 1,
            b'}' | b']' => {
                if depth == 0 {
                    return Some(index);
                }
                depth -= 1;
                if depth == 0 {
                    return Some(index + 1);
                }
            }
            b',' if depth == 0 => return Some(index),
            _ => {}
        }
        index += 1;
    }
    None
}

/// 🔁️ Reloads a JSONL log, rejecting any break in the sequence or in the hash chain.
pub fn replay_events(data: &str, max_bytes: usize, max_count: usize) -> Result<Vec<Event>, Error> {
    if max_bytes > 0 && data.len() > max_bytes {
        return Err(Error::Limit);
    }
    let mut result: Vec<Event> = Vec::new();
    let mut previous = String::new();
    for line in data.lines() {
        if line.is_empty() {
            continue;
        }
        if max_count > 0 && result.len() == max_count {
            return Err(Error::Limit);
        }
        let position = result.len() + 1;
        let corrupt = || Error::Invalid(format!("corrupt event {position}"));
        let parsed: serde_json::Value = serde_json::from_str(line).map_err(|_| corrupt())?;
        let members = parsed.as_object().ok_or_else(corrupt)?;
        let text = |name: &str| members.get(name).and_then(|value| value.as_str()).unwrap_or_default().to_string();
        let event = Event {
            schema: text("schema"),
            sequence: members.get("sequence").and_then(serde_json::Value::as_u64).unwrap_or_default(),
            kind: text("kind"),
            peer: text("peer"),
            generation: members.get("generation").and_then(serde_json::Value::as_u64).unwrap_or_default(),
            request_id: text("requestId"),
            payload: raw_member(line, "payload").ok_or_else(corrupt)?,
            previous: text("previous"),
            hash: text("hash"),
        };
        if event.schema != EVENT_SCHEMA || event.sequence != position as u64 || event.previous != previous || event.kind.is_empty() || event.digest() != event.hash {
            return Err(corrupt());
        }
        previous = event.hash.clone();
        result.push(event);
    }
    Ok(result)
}

//#endregion 📡️Event
