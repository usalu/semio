//! 📡️ Fixed query transport with lossless u64 authority, bounded page envelopes, and exact trailing-byte rejection.

use super::{LocalInteractionIdentity, LocalInteractionPage, LocalInteractionQueryToken};

//#region 🧬️Transport
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LocalInteractionQueryCommand {
    Read { request_id: u64 },
    Acknowledge { token: LocalInteractionQueryToken },
    Cancel { token: LocalInteractionQueryToken },
}

/// 🌱️ Hand-written, not derived — same DAG reason `LocalInteractionState`'s hand-written twin in
/// the parent module documents (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS,
/// 26/09/01). Internally tagged on `"kind"`, mirroring `#[serde(tag = "kind", rename_all =
/// "camelCase", deny_unknown_fields)]`; `request_id` mirrors `#[serde(with = "decimal_u64")]`.
impl crate::value::ToValue for LocalInteractionQueryCommand {
    fn to_value(&self) -> crate::value::DslValue {
        match self {
            LocalInteractionQueryCommand::Read { request_id } => {
                crate::value::DslValue::object(vec![("kind".to_string(), crate::value::DslValue::String("read".to_string())), ("requestId".to_string(), super::encode_decimal_u64(*request_id))])
            }
            LocalInteractionQueryCommand::Acknowledge { token } => {
                crate::value::DslValue::object(vec![("kind".to_string(), crate::value::DslValue::String("acknowledge".to_string())), ("token".to_string(), crate::value::ToValue::to_value(token))])
            }
            LocalInteractionQueryCommand::Cancel { token } => {
                crate::value::DslValue::object(vec![("kind".to_string(), crate::value::DslValue::String("cancel".to_string())), ("token".to_string(), crate::value::ToValue::to_value(token))])
            }
        }
    }
}
impl crate::value::FromValue for LocalInteractionQueryCommand {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for LocalInteractionQueryCommand, found {value:?}")));
        };
        let get = |key: &str| fields.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let kind = match get("kind") {
            Some(crate::value::DslValue::String(s)) => s,
            _ => return Err(crate::value::ValueError::new("LocalInteractionQueryCommand missing kind")),
        };
        let known: &[&str] = match kind.as_str() { "read" => &["kind", "requestId"], "acknowledge" | "cancel" => &["kind", "token"], _ => &["kind"] };
        if let Some((unknown, _)) = fields.iter().find(|(k, _)| !known.contains(&k.as_str())) {
            return Err(crate::value::ValueError::new(format!("unknown field `{unknown}` for LocalInteractionQueryCommand")));
        }
        match kind.as_str() {
            "read" => Ok(LocalInteractionQueryCommand::Read {
                request_id: super::decode_decimal_u64(get("requestId").ok_or_else(|| crate::value::ValueError::new("LocalInteractionQueryCommand.read missing requestId"))?).map_err(|e| e.under("requestId"))?,
            }),
            "acknowledge" => Ok(LocalInteractionQueryCommand::Acknowledge {
                token: <LocalInteractionQueryToken as crate::value::FromValue>::from_value(get("token").ok_or_else(|| crate::value::ValueError::new("LocalInteractionQueryCommand.acknowledge missing token"))?).map_err(|e| e.under("token"))?,
            }),
            "cancel" => Ok(LocalInteractionQueryCommand::Cancel {
                token: <LocalInteractionQueryToken as crate::value::FromValue>::from_value(get("token").ok_or_else(|| crate::value::ValueError::new("LocalInteractionQueryCommand.cancel missing token"))?).map_err(|e| e.under("token"))?,
            }),
            other => Err(crate::value::ValueError::new(format!("unknown LocalInteractionQueryCommand kind `{other}`"))),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LocalInteractionQueryRejection { Busy, Closed, GenerationExhausted, SourceFailed }

/// 🌱️ Hand-written, not derived — same reason as `LocalInteractionQueryCommand` above.
impl crate::value::ToValue for LocalInteractionQueryRejection {
    fn to_value(&self) -> crate::value::DslValue {
        crate::value::DslValue::String(
            match self {
                LocalInteractionQueryRejection::Busy => "busy",
                LocalInteractionQueryRejection::Closed => "closed",
                LocalInteractionQueryRejection::GenerationExhausted => "generation-exhausted",
                LocalInteractionQueryRejection::SourceFailed => "source-failed",
            }
            .to_string(),
        )
    }
}
impl crate::value::FromValue for LocalInteractionQueryRejection {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        match value {
            crate::value::DslValue::String(s) => match s.as_str() {
                "busy" => Ok(LocalInteractionQueryRejection::Busy),
                "closed" => Ok(LocalInteractionQueryRejection::Closed),
                "generation-exhausted" => Ok(LocalInteractionQueryRejection::GenerationExhausted),
                "source-failed" => Ok(LocalInteractionQueryRejection::SourceFailed),
                other => Err(crate::value::ValueError::new(format!("unknown LocalInteractionQueryRejection variant `{other}`"))),
            },
            other => Err(crate::value::ValueError::new(format!("expected a string, found {other:?}"))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LocalInteractionQueryReply {
    Started { token: LocalInteractionQueryToken },
    Page { page: LocalInteractionPage },
    Closed { token: LocalInteractionQueryToken, cancelled: bool },
    Rejected { request_id: u64, code: LocalInteractionQueryRejection },
}

/// 🌱️ Hand-written, not derived — same reason as `LocalInteractionQueryCommand` above.
impl crate::value::ToValue for LocalInteractionQueryReply {
    fn to_value(&self) -> crate::value::DslValue {
        match self {
            LocalInteractionQueryReply::Started { token } => {
                crate::value::DslValue::object(vec![("kind".to_string(), crate::value::DslValue::String("started".to_string())), ("token".to_string(), crate::value::ToValue::to_value(token))])
            }
            LocalInteractionQueryReply::Page { page } => {
                crate::value::DslValue::object(vec![("kind".to_string(), crate::value::DslValue::String("page".to_string())), ("page".to_string(), crate::value::ToValue::to_value(page))])
            }
            LocalInteractionQueryReply::Closed { token, cancelled } => crate::value::DslValue::object(vec![
                ("kind".to_string(), crate::value::DslValue::String("closed".to_string())),
                ("token".to_string(), crate::value::ToValue::to_value(token)),
                ("cancelled".to_string(), crate::value::ToValue::to_value(cancelled)),
            ]),
            LocalInteractionQueryReply::Rejected { request_id, code } => crate::value::DslValue::object(vec![
                ("kind".to_string(), crate::value::DslValue::String("rejected".to_string())),
                ("requestId".to_string(), super::encode_decimal_u64(*request_id)),
                ("code".to_string(), crate::value::ToValue::to_value(code)),
            ]),
        }
    }
}
impl crate::value::FromValue for LocalInteractionQueryReply {
    fn from_value(value: crate::value::DslValue) -> Result<Self, crate::value::ValueError> {
        let crate::value::DslValue::Object(fields) = value else {
            return Err(crate::value::ValueError::new(format!("expected an object for LocalInteractionQueryReply, found {value:?}")));
        };
        let get = |key: &str| fields.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let kind = match get("kind") {
            Some(crate::value::DslValue::String(s)) => s,
            _ => return Err(crate::value::ValueError::new("LocalInteractionQueryReply missing kind")),
        };
        let known: &[&str] = match kind.as_str() {
            "started" => &["kind", "token"],
            "page" => &["kind", "page"],
            "closed" => &["kind", "token", "cancelled"],
            "rejected" => &["kind", "requestId", "code"],
            _ => &["kind"],
        };
        if let Some((unknown, _)) = fields.iter().find(|(k, _)| !known.contains(&k.as_str())) {
            return Err(crate::value::ValueError::new(format!("unknown field `{unknown}` for LocalInteractionQueryReply")));
        }
        match kind.as_str() {
            "started" => Ok(LocalInteractionQueryReply::Started {
                token: <LocalInteractionQueryToken as crate::value::FromValue>::from_value(get("token").ok_or_else(|| crate::value::ValueError::new("LocalInteractionQueryReply.started missing token"))?).map_err(|e| e.under("token"))?,
            }),
            "page" => Ok(LocalInteractionQueryReply::Page {
                page: <LocalInteractionPage as crate::value::FromValue>::from_value(get("page").ok_or_else(|| crate::value::ValueError::new("LocalInteractionQueryReply.page missing page"))?).map_err(|e| e.under("page"))?,
            }),
            "closed" => Ok(LocalInteractionQueryReply::Closed {
                token: <LocalInteractionQueryToken as crate::value::FromValue>::from_value(get("token").ok_or_else(|| crate::value::ValueError::new("LocalInteractionQueryReply.closed missing token"))?).map_err(|e| e.under("token"))?,
                cancelled: <bool as crate::value::FromValue>::from_value(get("cancelled").ok_or_else(|| crate::value::ValueError::new("LocalInteractionQueryReply.closed missing cancelled"))?).map_err(|e| e.under("cancelled"))?,
            }),
            "rejected" => Ok(LocalInteractionQueryReply::Rejected {
                request_id: super::decode_decimal_u64(get("requestId").ok_or_else(|| crate::value::ValueError::new("LocalInteractionQueryReply.rejected missing requestId"))?).map_err(|e| e.under("requestId"))?,
                code: <LocalInteractionQueryRejection as crate::value::FromValue>::from_value(get("code").ok_or_else(|| crate::value::ValueError::new("LocalInteractionQueryReply.rejected missing code"))?).map_err(|e| e.under("code"))?,
            }),
            other => Err(crate::value::ValueError::new(format!("unknown LocalInteractionQueryReply kind `{other}`"))),
        }
    }
}
//#endregion 🧬️Transport

//#region 🔢️FixedCodec
const MAXIMUM_QUERY_WIRE_BYTES: usize = 4256;

fn unsigned(out: &mut Vec<u8>, mut value: u64) {
    loop { let byte = (value & 127) as u8; value >>= 7; out.push(byte | if value == 0 { 0 } else { 128 }); if value == 0 { break; } }
}

struct Reader<'a> { bytes: &'a [u8], offset: usize }

impl<'a> Reader<'a> {
    fn new(bytes: &'a [u8]) -> Result<Self, &'static str> { if bytes.len() > MAXIMUM_QUERY_WIRE_BYTES { return Err("local-interaction.wire-envelope"); } Ok(Self { bytes, offset: 0 }) }
    fn byte(&mut self) -> Result<u8, &'static str> { let byte = self.bytes.get(self.offset).copied().ok_or("local-interaction.truncated")?; self.offset += 1; Ok(byte) }
    fn unsigned(&mut self) -> Result<u64, &'static str> {
        let mut value = 0u64;
        for index in 0..10 {
            let byte = self.byte()?;
            if index == 9 && byte > 1 { return Err("local-interaction.invalid-u64"); }
            value |= u64::from(byte & 127) << (index * 7);
            if byte & 128 == 0 { if index != 0 && byte == 0 { return Err("local-interaction.noncanonical-u64"); } return Ok(value); }
        }
        Err("local-interaction.invalid-u64")
    }
    fn hash(&mut self) -> Result<[u8; 32], &'static str> { let mut hash = [0; 32]; for byte in &mut hash { *byte = self.byte()?; } Ok(hash) }
    fn boolean(&mut self) -> Result<bool, &'static str> { match self.byte()? { 0 => Ok(false), 1 => Ok(true), _ => Err("local-interaction.invalid-bool") } }
    fn finish(self) -> Result<(), &'static str> { if self.offset != self.bytes.len() { Err("local-interaction.trailing-bytes") } else { Ok(()) } }
}

fn token(out: &mut Vec<u8>, value: &LocalInteractionQueryToken) {
    unsigned(out, value.request_id); unsigned(out, value.query_generation);
    unsigned(out, u64::from(value.identity.app_instance_id)); unsigned(out, value.identity.generation);
    out.extend_from_slice(&value.identity.revision); out.extend_from_slice(&value.identity.document_revision); out.extend_from_slice(&value.identity.topology_revision);
    unsigned(out, value.ordinal);
}

fn read_token(reader: &mut Reader<'_>) -> Result<LocalInteractionQueryToken, &'static str> {
    let request_id = reader.unsigned()?;
    let query_generation = reader.unsigned()?;
    let app_instance_id = u32::try_from(reader.unsigned()?).map_err(|_| "local-interaction.invalid-instance")?;
    let identity = LocalInteractionIdentity { app_instance_id, generation: reader.unsigned()?, revision: reader.hash()?, document_revision: reader.hash()?, topology_revision: reader.hash()? };
    Ok(LocalInteractionQueryToken { request_id, query_generation, identity, ordinal: reader.unsigned()? })
}
//#endregion 🔢️FixedCodec

//#region 📡️CommandAndReply
pub fn encode_local_interaction_query_command(command: &LocalInteractionQueryCommand) -> Vec<u8> {
    let mut out = Vec::with_capacity(140);
    match command {
        LocalInteractionQueryCommand::Read { request_id } => { out.push(0); unsigned(&mut out, *request_id); },
        LocalInteractionQueryCommand::Acknowledge { token: value } => { out.push(1); token(&mut out, value); },
        LocalInteractionQueryCommand::Cancel { token: value } => { out.push(2); token(&mut out, value); },
    }
    out
}

pub fn decode_local_interaction_query_command(bytes: &[u8]) -> Result<LocalInteractionQueryCommand, &'static str> {
    let mut reader = Reader::new(bytes)?;
    let command = match reader.byte()? {
        0 => LocalInteractionQueryCommand::Read { request_id: reader.unsigned()? },
        1 => LocalInteractionQueryCommand::Acknowledge { token: read_token(&mut reader)? },
        2 => LocalInteractionQueryCommand::Cancel { token: read_token(&mut reader)? },
        _ => return Err("local-interaction.command-kind"),
    };
    reader.finish()?; Ok(command)
}

pub fn encode_local_interaction_query_reply(reply: &LocalInteractionQueryReply) -> Result<Vec<u8>, &'static str> {
    let mut out = Vec::with_capacity(MAXIMUM_QUERY_WIRE_BYTES);
    encode_local_interaction_query_reply_into(reply, &mut out)?;
    Ok(out)
}

/// 📏️ Exact fixed-token and admitted-page byte extent, without allocation or payload traversal.
pub fn local_interaction_query_reply_encoded_len(reply: &LocalInteractionQueryReply) -> Result<usize, &'static str> {
    fn width(value: u64) -> usize { ((64 - value.leading_zeros()).max(1) as usize + 6) / 7 }
    fn token_len(value: &LocalInteractionQueryToken) -> usize { 96 + width(value.request_id) + width(value.query_generation) + width(value.identity.app_instance_id as u64) + width(value.identity.generation) + width(value.ordinal) }
    Ok(match reply {
        LocalInteractionQueryReply::Started { token } => 1 + token_len(token),
        LocalInteractionQueryReply::Closed { token, .. } => 2 + token_len(token),
        LocalInteractionQueryReply::Rejected { request_id, .. } => 2 + width(*request_id),
        LocalInteractionQueryReply::Page { page } => {
            if page.bytes.len() > 4096 { return Err("local-interaction.page-length"); }
            98 + width(page.request_id) + width(page.query_generation) + width(page.identity.app_instance_id as u64) + width(page.identity.generation) + width(page.ordinal) + width(page.bytes.len() as u64) + page.bytes.len()
        },
    })
}

/// 📤️ Writes only into pre-admitted allocation; rejection leaves the caller's bytes untouched.
pub fn encode_local_interaction_query_reply_into(reply: &LocalInteractionQueryReply, out: &mut Vec<u8>) -> Result<(), &'static str> {
    let length = local_interaction_query_reply_encoded_len(reply)?;
    if out.capacity() - out.len() < length { return Err("local-interaction.output-not-admitted"); }
    match reply {
        LocalInteractionQueryReply::Started { token: value } => { out.push(0); token(out, value); },
        LocalInteractionQueryReply::Page { page } => {
            out.push(1); token(out, &LocalInteractionQueryToken { request_id: page.request_id, query_generation: page.query_generation, identity: page.identity.clone(), ordinal: page.ordinal });
            out.push(u8::from(page.terminal)); unsigned(out, page.bytes.len() as u64); out.extend_from_slice(&page.bytes);
        },
        LocalInteractionQueryReply::Closed { token: value, cancelled } => { out.push(2); token(out, value); out.push(u8::from(*cancelled)); },
        LocalInteractionQueryReply::Rejected { request_id, code } => {
            out.push(3); unsigned(out, *request_id); out.push(match code { LocalInteractionQueryRejection::Busy => 0, LocalInteractionQueryRejection::Closed => 1, LocalInteractionQueryRejection::GenerationExhausted => 2, LocalInteractionQueryRejection::SourceFailed => 3 });
        },
    }
    Ok(())
}

pub fn decode_local_interaction_query_reply(bytes: &[u8]) -> Result<LocalInteractionQueryReply, &'static str> {
    let mut reader = Reader::new(bytes)?;
    let reply = match reader.byte()? {
        0 => LocalInteractionQueryReply::Started { token: read_token(&mut reader)? },
        1 => {
            let token = read_token(&mut reader)?;
            let terminal = reader.boolean()?;
            let length = usize::try_from(reader.unsigned()?).map_err(|_| "local-interaction.page-length")?;
            if length > 4096 { return Err("local-interaction.page-length"); }
            let mut payload = Vec::with_capacity(length); for _ in 0..length { payload.push(reader.byte()?); }
            LocalInteractionQueryReply::Page { page: LocalInteractionPage { request_id: token.request_id, query_generation: token.query_generation, identity: token.identity, ordinal: token.ordinal, terminal, bytes: payload } }
        },
        2 => LocalInteractionQueryReply::Closed { token: read_token(&mut reader)?, cancelled: reader.boolean()? },
        3 => { let request_id = reader.unsigned()?; let code = match reader.byte()? { 0 => LocalInteractionQueryRejection::Busy, 1 => LocalInteractionQueryRejection::Closed, 2 => LocalInteractionQueryRejection::GenerationExhausted, 3 => LocalInteractionQueryRejection::SourceFailed, _ => return Err("local-interaction.rejection-code") }; LocalInteractionQueryReply::Rejected { request_id, code } },
        _ => return Err("local-interaction.reply-kind"),
    };
    reader.finish()?; Ok(reply)
}
//#endregion 📡️CommandAndReply

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
