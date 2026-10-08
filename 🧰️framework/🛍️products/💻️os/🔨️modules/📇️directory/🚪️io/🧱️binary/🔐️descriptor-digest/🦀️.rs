//! 🔐️ Canonical descriptor binary admission and authority hashing.
use crate::os_directory::schema::{ArtifactHash, DescriptorDigestError, DocumentDescriptor, validate_document_descriptor_v1};

pub const DESCRIPTOR_DIGEST_V1_DOMAIN: &[u8] = b"semio.document-descriptor.digest.v1\0";

fn decode_descriptor_hash(field: &'static str, value: &str) -> Result<[u8; 32], DescriptorDigestError> {
    if value.len() != 64 || value.as_bytes().iter().any(|byte| !matches!(byte, b'0'..=b'9' | b'a'..=b'f')) {
        return Err(DescriptorDigestError::InvalidHash(field));
    }
    let mut output = [0u8; 32];
    for (index, pair) in value.as_bytes().as_chunks::<2>().0.iter().enumerate() {
        let digit = |byte| match byte {
            b'0'..=b'9' => byte - b'0',
            b'a'..=b'f' => byte - b'a' + 10,
            _ => unreachable!(),
        };
        output[index] = digit(pair[0]) << 4 | digit(pair[1]);
    }
    if output == [0; 32] {
        return Err(DescriptorDigestError::InvalidHash(field));
    }
    Ok(output)
}

fn append_descriptor_field(output: &mut Vec<u8>, field: &'static str, bytes: &[u8]) -> Result<(), DescriptorDigestError> {
    let length = u64::try_from(bytes.len()).map_err(|_| DescriptorDigestError::LengthOverflow(field))?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(bytes);
    Ok(())
}

fn append_descriptor_text(output: &mut Vec<u8>, field: &'static str, value: &str) -> Result<(), DescriptorDigestError> {
    if value.is_empty() {
        return Err(DescriptorDigestError::EmptyField(field));
    }
    append_descriptor_field(output, field, value.as_bytes())
}

/// 🧬️ Encodes every immutable descriptor leaf after `DESCRIPTOR_DIGEST_V1_DOMAIN`, in declaration
/// order, as `u64_be(payload byte length) || payload`. Text is UTF-8, unsigned integers are fixed-
/// width big-endian payloads, and the three SHA-256 strings are decoded to their 32 bytes. Owner
/// leaves remain nested-order `plugin_id, package_id, version, package_hash`; frontier leaves remain
/// `head_seq, commit_seq, epoch`. JSON serialization never participates.
pub fn descriptor_digest_encoding_v1(descriptor: &DocumentDescriptor) -> Result<Vec<u8>, DescriptorDigestError> {
    validate_document_descriptor_v1(descriptor)?;
    let mut output = Vec::with_capacity(DESCRIPTOR_DIGEST_V1_DOMAIN.len() + 384);
    output.extend_from_slice(DESCRIPTOR_DIGEST_V1_DOMAIN);
    append_descriptor_text(&mut output, "space_id", &descriptor.space_id)?;
    append_descriptor_text(&mut output, "document_id", &descriptor.document_id)?;
    append_descriptor_text(&mut output, "artifact_kind", &descriptor.artifact_kind)?;
    append_descriptor_text(&mut output, "artifact_schema", &descriptor.artifact_schema)?;
    append_descriptor_text(&mut output, "owner.plugin_id", &descriptor.owner.plugin_id)?;
    append_descriptor_text(&mut output, "owner.package_id", &descriptor.owner.package_id)?;
    append_descriptor_text(&mut output, "owner.version", &descriptor.owner.version)?;
    append_descriptor_field(&mut output, "owner.package_hash", &decode_descriptor_hash("owner.package_hash", &descriptor.owner.package_hash)?)?;
    append_descriptor_field(&mut output, "pack_schema_hash", &decode_descriptor_hash("pack_schema_hash", &descriptor.pack_schema_hash)?)?;
    append_descriptor_field(&mut output, "bootstrap_version", &descriptor.bootstrap_version.to_be_bytes())?;
    append_descriptor_field(&mut output, "bootstrap_frontier.head_seq", &descriptor.bootstrap_frontier.head_seq.to_be_bytes())?;
    append_descriptor_field(&mut output, "bootstrap_frontier.commit_seq", &descriptor.bootstrap_frontier.commit_seq.to_be_bytes())?;
    append_descriptor_field(&mut output, "bootstrap_frontier.epoch", &descriptor.bootstrap_frontier.epoch.to_be_bytes())?;
    append_descriptor_field(&mut output, "bootstrap_snapshot_hash", &decode_descriptor_hash("bootstrap_snapshot_hash", &descriptor.bootstrap_snapshot_hash)?)?;
    Ok(output)
}

/// 🔐️ SHA-256 of [`descriptor_digest_encoding_v1`] through the repository-owned hash primitive.
pub fn descriptor_digest_v1(descriptor: &DocumentDescriptor) -> Result<ArtifactHash, DescriptorDigestError> {
    Ok(ArtifactHash(semio_framework_hash::Sha256::digest(&descriptor_digest_encoding_v1(descriptor)?)))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
