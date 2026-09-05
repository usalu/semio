//! ✉️ Bounded canonical protocol bytes, never a GIS execution or current-session capability.

use super::{schema::SAFE_INTEGER_MAX, InferenceErrorV1, InferencePrivateBytesV1};

pub(super) const COMMAND_MAX_BYTES: usize = 8192;
const TEXT_MAX_BYTES: usize = 256;
const DEPENDENCY_MAX_COUNT: usize = 64;
const PAYLOAD_MAX_BYTES: usize = 4096;

/// 🧱️ Exactly the server-derived fields one approval envelope may carry; no client bytes enter it.
pub(super) struct CanonicalInferenceCommandPartsV1<'a> {
    pub mutation_id: &'a str,
    pub document_id: &'a str,
    pub actor: &'a str,
    pub diff_schema: &'a str,
    pub diff_payload: &'a [u8],
    pub inverse_schema: &'a str,
    pub inverse_payload: &'a [u8],
    pub timestamp: protocol::HybridLogicalTimestamp,
}

pub(super) struct CanonicalInferenceCommandV1<'a> {
    mutation_id: &'a str,
    document_id: &'a str,
    actor: &'a str,
    dependencies: [&'a str; DEPENDENCY_MAX_COUNT],
    dependency_count: usize,
    diff_schema: &'a str,
    diff_payload: &'a [u8],
    inverse_schema: &'a str,
    inverse_payload: &'a [u8],
    timestamp: protocol::HybridLogicalTimestamp,
}

impl<'a> CanonicalInferenceCommandV1<'a> {
    pub(super) fn decode(bytes: &'a [u8]) -> Result<Self, InferenceErrorV1> {
        if bytes.is_empty() || bytes.len() > COMMAND_MAX_BYTES {
            return Err(InferenceErrorV1::Bounds);
        }
        let mut cursor = Cursor { bytes, position: 0 };
        let mutation_id = cursor.text()?;
        let document_id = cursor.text()?;
        let actor = cursor.text()?;
        let count = cursor.integer()?;
        if count > DEPENDENCY_MAX_COUNT as u64 {
            return Err(InferenceErrorV1::Bounds);
        }
        let dependency_count = count as usize;
        let mut dependencies = [""; DEPENDENCY_MAX_COUNT];
        for index in 0..dependency_count {
            let value = cursor.text()?;
            if dependencies[..index].contains(&value) {
                return Err(InferenceErrorV1::Invalid);
            }
            dependencies[index] = value;
        }
        let value = Self {
            mutation_id,
            document_id,
            actor,
            dependencies,
            dependency_count,
            diff_schema: cursor.text()?,
            diff_payload: cursor.field(PAYLOAD_MAX_BYTES)?,
            inverse_schema: cursor.text()?,
            inverse_payload: cursor.field(PAYLOAD_MAX_BYTES)?,
            timestamp: protocol::HybridLogicalTimestamp { actor: cursor.integer()?, physical_ms: cursor.integer()?, logical: cursor.integer()? },
        };
        if cursor.position != bytes.len() {
            return Err(InferenceErrorV1::Invalid);
        }
        let mut canonical = InferencePrivateBytesV1::new(Vec::with_capacity(bytes.len()), COMMAND_MAX_BYTES)?;
        value.encode(&mut canonical.0);
        if canonical.as_slice() != bytes {
            return Err(InferenceErrorV1::Invalid);
        }
        Ok(value)
    }

    pub(super) fn matches_identity(&self, mutation_id: &str, document_id: &str, actor: &str) -> bool {
        self.mutation_id == mutation_id && self.document_id == document_id && self.actor == actor
    }

    fn encode(&self, output: &mut Vec<u8>) {
        protocol::write_str(output, self.mutation_id);
        protocol::write_str(output, self.document_id);
        protocol::write_str(output, self.actor);
        protocol::wire::write_varint_u64(output, self.dependency_count as u64);
        for dependency in &self.dependencies[..self.dependency_count] {
            protocol::write_str(output, dependency);
        }
        protocol::write_str(output, self.diff_schema);
        protocol::write_bytes(output, self.diff_payload);
        protocol::write_str(output, self.inverse_schema);
        protocol::write_bytes(output, self.inverse_payload);
        for value in [self.timestamp.actor, self.timestamp.physical_ms, self.timestamp.logical] {
            protocol::wire::write_varint_u64(output, value);
        }
    }
}

/// ✍️ Builds the one canonical dependency-free server-stamped envelope and returns its exact bytes.
pub(super) fn encode_server_stamped_command_v1(parts: &CanonicalInferenceCommandPartsV1<'_>) -> Result<Vec<u8>, InferenceErrorV1> {
    if parts.diff_payload.len() > PAYLOAD_MAX_BYTES || parts.inverse_payload.len() > PAYLOAD_MAX_BYTES {
        return Err(InferenceErrorV1::Bounds);
    }
    let mut bytes = Vec::with_capacity(COMMAND_MAX_BYTES);
    CanonicalInferenceCommandV1 {
        mutation_id: parts.mutation_id,
        document_id: parts.document_id,
        actor: parts.actor,
        dependencies: [""; DEPENDENCY_MAX_COUNT],
        dependency_count: 0,
        diff_schema: parts.diff_schema,
        diff_payload: parts.diff_payload,
        inverse_schema: parts.inverse_schema,
        inverse_payload: parts.inverse_payload,
        timestamp: parts.timestamp,
    }
    .encode(&mut bytes);
    if bytes.is_empty() || bytes.len() > COMMAND_MAX_BYTES {
        return Err(InferenceErrorV1::Bounds);
    }
    CanonicalInferenceCommandV1::decode(&bytes)?;
    Ok(bytes)
}

struct Cursor<'a> {
    bytes: &'a [u8],
    position: usize,
}

impl<'a> Cursor<'a> {
    fn integer(&mut self) -> Result<u64, InferenceErrorV1> {
        let mut value = 0u64;
        for shift in (0..=63).step_by(7) {
            let byte = *self.bytes.get(self.position).ok_or(InferenceErrorV1::Invalid)?;
            self.position += 1;
            if shift == 63 && byte > 1 {
                return Err(InferenceErrorV1::Invalid);
            }
            value |= u64::from(byte & 127) << shift;
            if byte & 128 == 0 {
                if shift != 0 && byte == 0 {
                    return Err(InferenceErrorV1::Invalid);
                }
                if value > SAFE_INTEGER_MAX {
                    return Err(InferenceErrorV1::Bounds);
                }
                return Ok(value);
            }
        }
        Err(InferenceErrorV1::Invalid)
    }

    fn field(&mut self, maximum: usize) -> Result<&'a [u8], InferenceErrorV1> {
        let length = self.integer()?;
        if length > maximum as u64 || length > (self.bytes.len() - self.position) as u64 {
            return Err(InferenceErrorV1::Bounds);
        }
        let end = self.position + length as usize;
        let value = &self.bytes[self.position..end];
        self.position = end;
        Ok(value)
    }

    fn text(&mut self) -> Result<&'a str, InferenceErrorV1> {
        let value = std::str::from_utf8(self.field(TEXT_MAX_BYTES)?).map_err(|_| InferenceErrorV1::Invalid)?;
        if value.is_empty() || value.chars().any(char::is_control) {
            return Err(InferenceErrorV1::Invalid);
        }
        Ok(value)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
