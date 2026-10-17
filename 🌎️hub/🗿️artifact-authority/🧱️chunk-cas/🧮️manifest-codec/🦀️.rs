//! 🧮️ The canonical manifest and ownership codecs: field cursor, prepare, validate, encode, decode and the manifest locator.

use super::*;

pub(super) fn append_field(output: &mut Vec<u8>, value: &[u8]) -> Result<(), AuthorityError> {
    let length = u64::try_from(value.len()).map_err(|_| AuthorityError::ResourceLimit("artifact CAS manifest byte"))?;
    output.extend_from_slice(&length.to_be_bytes());
    output.extend_from_slice(value);
    Ok(())
}

fn valid_space(space_id: &str) -> bool {
    !space_id.is_empty() && space_id.len() <= ARTIFACT_CAS_MAX_SPACE_BYTES
}

pub(super) fn chunk_id(space_id: &str, bytes: &[u8]) -> Result<ArtifactHash, AuthorityError> {
    if !valid_space(space_id) || bytes.is_empty() || bytes.len() > ARTIFACT_CAS_CHUNK_BYTES {
        return Err(AuthorityError::BlobIntegrity("artifact CAS chunk identity"));
    }
    let mut hash = Sha256::new();
    hash.update(ARTIFACT_CAS_CHUNK_DOMAIN_V1);
    hash.update(&(space_id.len() as u64).to_be_bytes());
    hash.update(space_id.as_bytes());
    hash.update(&8u64.to_be_bytes());
    hash.update(&(bytes.len() as u64).to_be_bytes());
    hash.update(&(bytes.len() as u64).to_be_bytes());
    hash.update(bytes);
    Ok(ArtifactHash(hash.finalize()))
}

pub(super) fn space_digest(space_id: &str) -> Result<ArtifactHash, AuthorityError> {
    if !valid_space(space_id) {
        return Err(AuthorityError::BlobIntegrity("artifact CAS space identity"));
    }
    let mut hash = Sha256::new();
    hash.update(ARTIFACT_CAS_SPACE_DOMAIN_V1);
    hash.update(&(space_id.len() as u64).to_be_bytes());
    hash.update(space_id.as_bytes());
    Ok(ArtifactHash(hash.finalize()))
}

/// 🧬️ Computes the exact domain-separated v1 chunk identity.
pub fn artifact_cas_chunk_id_v1(space_id: &str, bytes: &[u8]) -> Result<ArtifactHash, AuthorityError> {
    chunk_id(space_id, bytes)
}

/// 🧬️ Encodes one manifest in its only canonical byte representation.
pub fn encode_artifact_cas_manifest_v1(manifest: &ArtifactCasManifestV1) -> Result<Vec<u8>, AuthorityError> {
    if !valid_space(&manifest.space_id)
        || manifest.raw_byte_length > AUTHORITY_MAX_PAIR_BYTES
        || manifest.chunk_bytes != ARTIFACT_CAS_CHUNK_BYTES as u32
        || manifest.chunks.len() > ARTIFACT_CAS_MAX_CHUNKS
        || manifest.chunks.len() as u64 != manifest.raw_byte_length.div_ceil(ARTIFACT_CAS_CHUNK_BYTES as u64)
    {
        return Err(AuthorityError::BlobIntegrity("artifact CAS manifest shape"));
    }
    let mut output = Vec::with_capacity(ARTIFACT_CAS_MANIFEST_DOMAIN_V1.len() + 128 + manifest.chunks.len() * 64);
    output.extend_from_slice(ARTIFACT_CAS_MANIFEST_DOMAIN_V1);
    append_field(&mut output, manifest.space_id.as_bytes())?;
    append_field(&mut output, &manifest.raw_sha256.0)?;
    append_field(&mut output, &manifest.raw_byte_length.to_be_bytes())?;
    append_field(&mut output, &manifest.chunk_bytes.to_be_bytes())?;
    append_field(&mut output, &(manifest.chunks.len() as u32).to_be_bytes())?;
    for (index, chunk) in manifest.chunks.iter().enumerate() {
        let expected_length = if index + 1 == manifest.chunks.len() {
            let remainder = manifest.raw_byte_length % ARTIFACT_CAS_CHUNK_BYTES as u64;
            if remainder == 0 {
                ARTIFACT_CAS_CHUNK_BYTES as u64
            } else {
                remainder
            }
        } else {
            ARTIFACT_CAS_CHUNK_BYTES as u64
        };
        if chunk.ordinal as usize != index || chunk.byte_length == 0 || u64::from(chunk.byte_length) != expected_length {
            return Err(AuthorityError::BlobIntegrity("artifact CAS manifest chunk shape"));
        }
        append_field(&mut output, &chunk.ordinal.to_be_bytes())?;
        append_field(&mut output, &chunk.byte_length.to_be_bytes())?;
        append_field(&mut output, &chunk.chunk_id.0)?;
    }
    if output.len() > ARTIFACT_CAS_MAX_MANIFEST_BYTES {
        return Err(AuthorityError::ResourceLimit("artifact CAS manifest byte"));
    }
    Ok(output)
}

struct FieldCursor<'a> {
    pub(super) bytes: &'a [u8],
    pub(super) offset: usize,
}

impl<'a> FieldCursor<'a> {
    fn field(&mut self, exact: Option<usize>) -> Result<&'a [u8], AuthorityError> {
        let length_end = self.offset.checked_add(8).ok_or(AuthorityError::BlobIntegrity("artifact CAS manifest encoding"))?;
        let length_bytes: [u8; 8] = self.bytes.get(self.offset..length_end).ok_or(AuthorityError::BlobIntegrity("artifact CAS manifest encoding"))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS manifest encoding"))?;
        let length = usize::try_from(u64::from_be_bytes(length_bytes)).map_err(|_| AuthorityError::ResourceLimit("artifact CAS manifest byte"))?;
        if exact.is_some_and(|expected| expected != length) {
            return Err(AuthorityError::BlobIntegrity("artifact CAS manifest field width"));
        }
        let end = length_end.checked_add(length).ok_or(AuthorityError::BlobIntegrity("artifact CAS manifest encoding"))?;
        let value = self.bytes.get(length_end..end).ok_or(AuthorityError::BlobIntegrity("artifact CAS manifest encoding"))?;
        self.offset = end;
        Ok(value)
    }
}

fn u32_field(cursor: &mut FieldCursor<'_>) -> Result<u32, AuthorityError> {
    Ok(u32::from_be_bytes(cursor.field(Some(4))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS manifest integer"))?))
}

fn u64_field(cursor: &mut FieldCursor<'_>) -> Result<u64, AuthorityError> {
    Ok(u64::from_be_bytes(cursor.field(Some(8))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS manifest integer"))?))
}

/// 🔍️ Strictly decodes, scope-checks, and canonicality-checks one manifest before use.
pub fn decode_artifact_cas_manifest_v1(bytes: &[u8], expected_space_id: &str, expected_manifest_id: ArtifactHash) -> Result<ArtifactCasManifestV1, AuthorityError> {
    if bytes.len() > ARTIFACT_CAS_MAX_MANIFEST_BYTES || !valid_space(expected_space_id) || !bytes.starts_with(ARTIFACT_CAS_MANIFEST_DOMAIN_V1) || ArtifactHash(Sha256::digest(bytes)) != expected_manifest_id {
        return Err(AuthorityError::BlobIntegrity("artifact CAS manifest identity"));
    }
    let mut cursor = FieldCursor { bytes, offset: ARTIFACT_CAS_MANIFEST_DOMAIN_V1.len() };
    let space_bytes = cursor.field(None)?;
    if space_bytes.len() > ARTIFACT_CAS_MAX_SPACE_BYTES {
        return Err(AuthorityError::ResourceLimit("artifact CAS space byte"));
    }
    let space_id = std::str::from_utf8(space_bytes).map_err(|_| AuthorityError::BlobIntegrity("artifact CAS manifest space"))?.to_string();
    let raw_sha256 = ArtifactHash(cursor.field(Some(32))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS raw hash"))?);
    let raw_byte_length = u64_field(&mut cursor)?;
    let chunk_bytes = u32_field(&mut cursor)?;
    let chunk_count = usize::try_from(u32_field(&mut cursor)?).map_err(|_| AuthorityError::ResourceLimit("artifact CAS chunk count"))?;
    if space_id != expected_space_id
        || raw_byte_length > AUTHORITY_MAX_PAIR_BYTES
        || chunk_bytes != ARTIFACT_CAS_CHUNK_BYTES as u32
        || chunk_count > ARTIFACT_CAS_MAX_CHUNKS
        || chunk_count as u64 != raw_byte_length.div_ceil(ARTIFACT_CAS_CHUNK_BYTES as u64)
    {
        return Err(AuthorityError::BlobIntegrity("artifact CAS manifest shape"));
    }
    let mut chunks = Vec::with_capacity(chunk_count);
    for index in 0..chunk_count {
        let ordinal = u32_field(&mut cursor)?;
        let byte_length = u32_field(&mut cursor)?;
        let chunk_id = ArtifactHash(cursor.field(Some(32))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS chunk hash"))?);
        let expected_length = if index + 1 == chunk_count {
            let remainder = raw_byte_length % ARTIFACT_CAS_CHUNK_BYTES as u64;
            if remainder == 0 {
                ARTIFACT_CAS_CHUNK_BYTES as u64
            } else {
                remainder
            }
        } else {
            ARTIFACT_CAS_CHUNK_BYTES as u64
        };
        if ordinal as usize != index || byte_length == 0 || u64::from(byte_length) != expected_length {
            return Err(AuthorityError::BlobIntegrity("artifact CAS manifest chunk shape"));
        }
        chunks.push(ArtifactCasChunkV1 { ordinal, byte_length, chunk_id });
    }
    if cursor.offset != bytes.len() {
        return Err(AuthorityError::BlobIntegrity("artifact CAS manifest trailing byte"));
    }
    let manifest = ArtifactCasManifestV1 { space_id, raw_sha256, raw_byte_length, chunk_bytes, chunks };
    if encode_artifact_cas_manifest_v1(&manifest)? != bytes {
        return Err(AuthorityError::BlobIntegrity("artifact CAS manifest canonical encoding"));
    }
    Ok(manifest)
}

/// 🧭 Plans exact raw/chunk/manifest identities without retaining a second copy of raw bytes.
pub fn prepare_artifact_cas_manifest_v1(space_id: &str, raw: &[u8]) -> Result<ArtifactCasManifestPlan, AuthorityError> {
    if !valid_space(space_id) || raw.len() as u64 > AUTHORITY_MAX_PAIR_BYTES {
        return Err(AuthorityError::ResourceLimit("artifact CAS raw byte"));
    }
    let mut chunks = Vec::with_capacity(raw.len().div_ceil(ARTIFACT_CAS_CHUNK_BYTES));
    for (ordinal, bytes) in raw.chunks(ARTIFACT_CAS_CHUNK_BYTES).enumerate() {
        chunks.push(ArtifactCasChunkV1 {
            ordinal: u32::try_from(ordinal).map_err(|_| AuthorityError::ResourceLimit("artifact CAS chunk count"))?,
            byte_length: u32::try_from(bytes.len()).map_err(|_| AuthorityError::ResourceLimit("artifact CAS chunk byte"))?,
            chunk_id: chunk_id(space_id, bytes)?,
        });
    }
    let manifest = ArtifactCasManifestV1 { space_id: space_id.to_string(), raw_sha256: ArtifactHash(Sha256::digest(raw)), raw_byte_length: raw.len() as u64, chunk_bytes: ARTIFACT_CAS_CHUNK_BYTES as u32, chunks };
    let manifest_bytes = encode_artifact_cas_manifest_v1(&manifest)?;
    let manifest_id = ArtifactHash(Sha256::digest(&manifest_bytes));
    Ok(ArtifactCasManifestPlan { manifest, manifest_bytes, manifest_id })
}

fn validate_ownership_plan(plan: &ArtifactCasOwnershipPlanV1) -> Result<(), AuthorityError> {
    if !valid_space(&plan.scope.space_id)
        || plan.scope.document_id.is_empty()
        || plan.checkpoint_id.0 == [0; 32]
        || plan.pack_manifest_id.0 == [0; 32]
        || plan.spr_manifest_id.0 == [0; 32]
        || plan.objects.len() < 2
        || plan.objects.len() > ARTIFACT_CAS_OWNERSHIP_MAX_OBJECTS
    {
        return Err(AuthorityError::BlobIntegrity("artifact CAS ownership shape"));
    }
    let mut previous: Option<(ArtifactCasObjectKind, [u8; 32])> = None;
    let mut pack = false;
    let mut spr = false;
    for object in &plan.objects {
        if object.space_id != plan.scope.space_id || object.digest.0 == [0; 32] {
            return Err(AuthorityError::BlobIntegrity("artifact CAS ownership object"));
        }
        let identity = (object.kind, object.digest.0);
        if previous.is_some_and(|value| value >= identity) {
            return Err(AuthorityError::BlobIntegrity("artifact CAS ownership order"));
        }
        previous = Some(identity);
        pack |= object.kind == ArtifactCasObjectKind::Manifest && object.digest == plan.pack_manifest_id;
        spr |= object.kind == ArtifactCasObjectKind::Manifest && object.digest == plan.spr_manifest_id;
    }
    if !pack || !spr {
        return Err(AuthorityError::BlobIntegrity("artifact CAS ownership manifest"));
    }
    Ok(())
}

/// 🧭️ Derives the exact pair ownership set before any physical CAS write.
pub fn prepare_artifact_cas_ownership_v1(checkpoint: &ArtifactCheckpoint, pair: &ArtifactPair) -> Result<ArtifactCasOwnershipPlanV1, AuthorityError> {
    let pack = prepare_artifact_cas_manifest_v1(&checkpoint.scope.space_id, &pair.pack)?;
    let spr = prepare_artifact_cas_manifest_v1(&checkpoint.scope.space_id, &pair.spr)?;
    if checkpoint.pack.sha256 != pack.manifest.raw_sha256 || checkpoint.pack.byte_length != pack.manifest.raw_byte_length || checkpoint.spr.sha256 != spr.manifest.raw_sha256 || checkpoint.spr.byte_length != spr.manifest.raw_byte_length {
        return Err(AuthorityError::BlobIntegrity("artifact CAS ownership raw identity"));
    }
    let mut objects = Vec::with_capacity(pack.manifest.chunks.len() + spr.manifest.chunks.len() + 2);
    objects.extend(pack.manifest.chunks.iter().map(|chunk| ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Chunk, digest: chunk.chunk_id }));
    objects.extend(spr.manifest.chunks.iter().map(|chunk| ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Chunk, digest: chunk.chunk_id }));
    objects.push(ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Manifest, digest: pack.manifest_id });
    objects.push(ArtifactCasObjectKey { space_id: checkpoint.scope.space_id.clone(), kind: ArtifactCasObjectKind::Manifest, digest: spr.manifest_id });
    objects.sort_by_key(|object| (object.kind, object.digest.0));
    objects.dedup();
    let plan = ArtifactCasOwnershipPlanV1 { scope: checkpoint.scope.clone(), checkpoint_id: checkpoint.checkpoint_id, pack_manifest_id: pack.manifest_id, spr_manifest_id: spr.manifest_id, objects };
    validate_ownership_plan(&plan)?;
    Ok(plan)
}

/// 🧬️ Encodes the private ownership journal input canonically.
pub fn encode_artifact_cas_ownership_v1(plan: &ArtifactCasOwnershipPlanV1) -> Result<Vec<u8>, AuthorityError> {
    validate_ownership_plan(plan)?;
    let mut output = Vec::with_capacity(ARTIFACT_CAS_OWNERSHIP_DOMAIN_V1.len() + 256 + plan.objects.len() * 64);
    output.extend_from_slice(ARTIFACT_CAS_OWNERSHIP_DOMAIN_V1);
    append_field(&mut output, plan.scope.space_id.as_bytes())?;
    append_field(&mut output, plan.scope.document_id.as_bytes())?;
    append_field(&mut output, &plan.checkpoint_id.0)?;
    append_field(&mut output, &plan.pack_manifest_id.0)?;
    append_field(&mut output, &plan.spr_manifest_id.0)?;
    append_field(&mut output, &(plan.objects.len() as u32).to_be_bytes())?;
    for object in &plan.objects {
        append_field(&mut output, object.kind.name().as_bytes())?;
        append_field(&mut output, &object.digest.0)?;
    }
    if output.len() > ARTIFACT_CAS_OWNERSHIP_MAX_BYTES {
        return Err(AuthorityError::ResourceLimit("artifact CAS ownership byte"));
    }
    Ok(output)
}

/// 🔍️ Strictly decodes one canonical private ownership journal input.
pub fn decode_artifact_cas_ownership_v1(bytes: &[u8]) -> Result<ArtifactCasOwnershipPlanV1, AuthorityError> {
    if bytes.len() > ARTIFACT_CAS_OWNERSHIP_MAX_BYTES || !bytes.starts_with(ARTIFACT_CAS_OWNERSHIP_DOMAIN_V1) {
        return Err(AuthorityError::BlobIntegrity("artifact CAS ownership identity"));
    }
    let mut cursor = FieldCursor { bytes, offset: ARTIFACT_CAS_OWNERSHIP_DOMAIN_V1.len() };
    let space_id = std::str::from_utf8(cursor.field(None)?).map_err(|_| AuthorityError::BlobIntegrity("artifact CAS ownership space"))?.to_string();
    let document_id = std::str::from_utf8(cursor.field(None)?).map_err(|_| AuthorityError::BlobIntegrity("artifact CAS ownership document"))?.to_string();
    let checkpoint_id = ArtifactHash(cursor.field(Some(32))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS ownership checkpoint"))?);
    let pack_manifest_id = ArtifactHash(cursor.field(Some(32))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS ownership manifest"))?);
    let spr_manifest_id = ArtifactHash(cursor.field(Some(32))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS ownership manifest"))?);
    let count = usize::try_from(u32_field(&mut cursor)?).map_err(|_| AuthorityError::ResourceLimit("artifact CAS ownership count"))?;
    if count > ARTIFACT_CAS_OWNERSHIP_MAX_OBJECTS {
        return Err(AuthorityError::ResourceLimit("artifact CAS ownership count"));
    }
    let mut objects = Vec::with_capacity(count);
    for _ in 0..count {
        let kind = ArtifactCasObjectKind::parse(std::str::from_utf8(cursor.field(None)?).map_err(|_| AuthorityError::BlobIntegrity("artifact CAS ownership kind"))?)?;
        let digest = ArtifactHash(cursor.field(Some(32))?.try_into().map_err(|_| AuthorityError::BlobIntegrity("artifact CAS ownership object"))?);
        objects.push(ArtifactCasObjectKey { space_id: space_id.clone(), kind, digest });
    }
    if cursor.offset != bytes.len() {
        return Err(AuthorityError::BlobIntegrity("artifact CAS ownership trailing byte"));
    }
    let plan = ArtifactCasOwnershipPlanV1 { scope: DocumentScope::new(space_id, document_id), checkpoint_id, pack_manifest_id, spr_manifest_id, objects };
    validate_ownership_plan(&plan)?;
    if encode_artifact_cas_ownership_v1(&plan)? != bytes {
        return Err(AuthorityError::BlobIntegrity("artifact CAS ownership canonical encoding"));
    }
    Ok(plan)
}

/// 🔗️ Verifies that final private locators exactly name the reserved manifests.
pub fn validate_artifact_cas_publication_v1(plan: &ArtifactCasOwnershipPlanV1, checkpoint: &ArtifactCheckpoint) -> Result<(), AuthorityError> {
    validate_ownership_plan(plan)?;
    if plan.scope != checkpoint.scope
        || plan.checkpoint_id != checkpoint.checkpoint_id
        || decode_artifact_cas_manifest_locator_v1(&checkpoint.pack.storage_key)? != plan.pack_manifest_id
        || decode_artifact_cas_manifest_locator_v1(&checkpoint.spr.storage_key)? != plan.spr_manifest_id
    {
        return Err(AuthorityError::BlobIntegrity("artifact CAS reserved publication"));
    }
    Ok(())
}

/// 🪢️ Encodes one private manifest locator without exposing tenant or chunk identities.
pub fn artifact_cas_manifest_locator_v1(manifest_id: ArtifactHash) -> String {
    format!("{ARTIFACT_CAS_MANIFEST_LOCATOR_PREFIX}{}", hex_lower(&manifest_id.0))
}

/// 🔓️ Strictly decodes one private manifest locator.
pub fn decode_artifact_cas_manifest_locator_v1(locator: &str) -> Result<ArtifactHash, AuthorityError> {
    let hexadecimal = locator.strip_prefix(ARTIFACT_CAS_MANIFEST_LOCATOR_PREFIX).ok_or_else(|| AuthorityError::Store("invalid artifact CAS manifest locator".to_string()))?;
    if hexadecimal.len() != 64 || hexadecimal.bytes().any(|byte| !matches!(byte, b'0'..=b'9' | b'a'..=b'f')) {
        return Err(AuthorityError::Store("invalid artifact CAS manifest locator".to_string()));
    }
    let mut output = [0; 32];
    for (index, pair) in hexadecimal.as_bytes().chunks_exact(2).enumerate() {
        let nibble = |byte: u8| if byte <= b'9' { byte - b'0' } else { byte - b'a' + 10 };
        output[index] = nibble(pair[0]) << 4 | nibble(pair[1]);
    }
    Ok(ArtifactHash(output))
}

pub(super) fn validate_object(key: &ArtifactCasObjectKey, bytes: &[u8]) -> Result<(), AuthorityError> {
    if !valid_space(&key.space_id) || bytes.len() > key.kind.maximum_bytes() {
        return Err(AuthorityError::ResourceLimit("artifact CAS object byte"));
    }
    let actual = match key.kind {
        ArtifactCasObjectKind::Chunk => chunk_id(&key.space_id, bytes)?,
        ArtifactCasObjectKind::Manifest => {
            let digest = ArtifactHash(Sha256::digest(bytes));
            let _ = decode_artifact_cas_manifest_v1(bytes, &key.space_id, digest)?;
            digest
        }
    };
    if actual != key.digest {
        return Err(AuthorityError::BlobIntegrity("artifact CAS object key"));
    }
    Ok(())
}
