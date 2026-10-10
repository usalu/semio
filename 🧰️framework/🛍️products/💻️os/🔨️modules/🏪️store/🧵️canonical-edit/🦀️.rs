//! 🔏️ Store-owned canonical JSON traversal, byte accounting, and exact one-item sealing.

use super::*;
use semio_framework_value::{RetirementDemand, FactoryAuthority, retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress}};

#[path = "🧳️source/🦀️.rs"]
mod native_source;
use native_source::{ArtifactStoreCanonicalOwner,ArtifactStoreCanonicalSource};

#[path = "🌱️value/🦀️.rs"]
mod value;
pub use value::{ArtifactCanonicalValue, ArtifactCanonicalValueAdmission, ArtifactCanonicalValueCheckpoint, ArtifactCanonicalValueCloseStep, ArtifactCanonicalValueGrant, ArtifactCanonicalValueLimits, ArtifactCanonicalValueStep};

#[path = "🧵️borrowed/🦀️.rs"]
mod borrowed;
use borrowed::ArtifactCanonicalEditEncoder;
pub use semio_framework_pack_json::ArtifactCanonicalJsonText;
use semio_framework_pack_json::canonical_escape;
#[cfg(test)]
#[path = "🔤️text/🧪️tests/🦀️.rs"]
mod native_text_tests;
pub use semio_framework_pack_json::{ArtifactCanonicalJsonNode, ArtifactCanonicalJsonTree, ArtifactCanonicalJsonTreeCursor, ArtifactCanonicalJsonTreeStep};
use semio_framework_pack_json::ArtifactCanonicalJsonScalarBytes as ScalarBytes;
pub use borrowed::{ArtifactCanonicalJsonArray, ArtifactCanonicalJsonObject, ArtifactCanonicalJsonValue};
#[path = "📖️reader/🦀️.rs"]
mod reader;
pub use reader::ArtifactCanonicalJsonReader;
#[cfg(test)]
#[path = "🧵️borrowed/🧪️tests/🧵️borrowed/🦀️.rs"]
mod borrowed_tests;

//#region 🧬️TypedCanonicalSource
pub const ARTIFACT_CANONICAL_JSON_DEPTH: usize = 64;
pub const ARTIFACT_CANONICAL_JSON_CHUNK_BYTES: usize = 256;
const CANONICAL_EDIT_MAXIMUM_BYTES: u64 = 16 * ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES as u64;
const CANONICAL_EDIT_MAXIMUM_OVERHEAD_BYTES: u64 = b"semio.artifact.cursor.v2".len() as u64 + 3 * 8 + b"edit".len() as u64 + 4 * ARTIFACT_STORE_ONE_ITEM_ID_BYTES as u64;

/// ⚠️ Exact initialized output prefix retained even when typed canonical traversal fails.
#[derive(Debug, PartialEq, Eq)]
pub struct ArtifactCanonicalJsonEncodeError {
    pub written_bytes: usize,
    pub reason: ValueError,
}

/// 🧭️ Exact serde field order over an immutable typed owner. Each lookup must perform bounded
/// indexed access; scanning, serialization, cloning, and collection inside these methods are forbidden.
pub trait ArtifactCanonicalJson: Sync {
    fn canonical_json_node(&self, _path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, ValueError> {
        Err(invalid_path())
    }
    fn canonical_json_key(&self, _object_path: &[usize], _index: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> {
        Err(invalid_path())
    }
    fn canonical_json_borrowed_root(&self) -> Result<Option<ArtifactCanonicalJsonValue<'_>>, ValueError> {
        Ok(None)
    }
}

fn invalid_path() -> ValueError {
    ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"canonical-edit.invalid-typed-path")
}

fn field_at(fields: &[(&'static str, bool)], index: usize) -> Result<(usize, &'static str), ValueError> {
    fields.iter().enumerate().filter(|(_, (_, present))| *present).nth(index).map(|(ordinal, (name, _))| (ordinal, *name)).ok_or_else(invalid_path)
}

/// 🪺️ Typed cursor over what an edit's revision identity covers (`CursorRevisionAccumulator::revision_value`): every member
/// of the edit but `sequenceNumber`, its position in one replica's ledger.
enum CanonicalEditNode<'a, M> {
    Edit(&'a Edit<M>),
    Mutation(&'a M),
    Mutations(&'a [M]),
    PagedMutations(&'a semio_framework_value::list::PagedList<M, {usize::MAX}>),
    Metas(&'a [MutationMeta]),
    Meta(&'a MutationMeta),
    Clock(&'a HybridLogicalTimestamp),
    Dependencies(&'a [MutationId]),
    Hash(&'a [u8; 32]),
    Origin(&'a crate::os_spr::MutationOrigin),
    Target(&'a crate::os_spr::ForeignTarget),
    Scalar(ArtifactCanonicalJsonNode<'a>),
}

impl<'a, M: ArtifactCanonicalJson> CanonicalEditNode<'a, M> {
    /// 🔤 camelCase, because `MutationOrigin`'s own `ToValue` is the normative key
    /// spelling and this cursor must produce its bytes exactly. Its sibling
    /// `transaction` variant was already aligned; `contributed` alone was not, so a
    /// contributed edit's canonical form never matched its own value projection.
    fn fields(&self) -> [(&'static str, bool); 12] {
        let mut fields = [("", false); 12];
        match self {
            Self::Edit(edit) => {
                fields[..9].copy_from_slice(&[
                    ("id", true),
                    ("actor", edit.actor.is_some()),
                    ("forwards", true),
                    ("inverse", true),
                    ("mutationMeta", !edit.mutation_meta.is_empty()),
                    ("verb", edit.verb.is_some()),
                    ("startedAt", true),
                    ("finishedAt", edit.finished_at.is_some()),
                    ("line", true),
                ]);
            }
            Self::Meta(meta) => {
                fields = [
                    ("mutation_id", meta.mutation_id.is_some()),
                    ("dependencies", !meta.dependencies.is_empty()),
                    ("base_version", true),
                    ("author_id", meta.author_id.is_some()),
                    ("timestamp", true),
                    ("undo_policy", true),
                    ("payload_hash", meta.payload_hash.is_some()),
                    ("semantic_kind", meta.semantic_kind.is_some()),
                    ("label", meta.label.is_some()),
                    ("group_id", meta.group_id.is_some()),
                    ("origin", !meta.origin.is_owner()),
                    ("", false),
                ];
            }
            Self::Clock(_) => fields[..3].copy_from_slice(&[("actor", true), ("physical_ms", true), ("logical", true)]),
            Self::Origin(origin) => match origin {
                crate::os_spr::MutationOrigin::Owner => fields[0] = ("kind", true),
                crate::os_spr::MutationOrigin::Contributed { .. } => fields[..4].copy_from_slice(&[("kind", true), ("pluginId", true), ("mutationId", true), ("payloadHash", true)]),
                crate::os_spr::MutationOrigin::Transaction { .. } => fields[..2].copy_from_slice(&[("kind", true), ("initiator", true)]),
            },
            Self::Target(target) => fields[..3].copy_from_slice(&[("artifactId", true), ("artifactKind", true), ("dialect", target.dialect.is_some())]),
            _ => {}
        }
        fields
    }

    fn child(&self, index: usize) -> Result<Self, ValueError> {
        use ArtifactCanonicalJsonNode as N;
        Ok(match self {
            Self::Edit(edit) => match field_at(&self.fields(), index)?.0 {
                0 => Self::Scalar(N::String(&edit.id)),
                1 => Self::Scalar(N::String(edit.actor.as_deref().ok_or_else(invalid_path)?)),
                2 => Self::Mutations(&edit.forwards),
                3 => Self::PagedMutations(&edit.inverse),
                4 => Self::Metas(&edit.mutation_meta),
                5 => Self::Scalar(N::String(edit.verb.as_deref().ok_or_else(invalid_path)?)),
                6 => Self::Scalar(N::String(&edit.started_at)),
                7 => Self::Scalar(N::String(edit.finished_at.as_deref().ok_or_else(invalid_path)?)),
                8 => Self::Scalar(edit.line.as_deref().map_or(N::Null, N::String)),
                _ => return Err(invalid_path()),
            },
            Self::Mutations(values) => Self::Mutation(values.get(index).ok_or_else(invalid_path)?),
            Self::PagedMutations(values) => Self::Mutation(values.get(index).ok_or_else(invalid_path)?),
            Self::Metas(values) => Self::Meta(values.get(index).ok_or_else(invalid_path)?),
            Self::Meta(meta) => match field_at(&self.fields(), index)?.0 {
                0 => Self::Scalar(N::String(&meta.mutation_id.as_ref().ok_or_else(invalid_path)?.0)),
                1 => Self::Dependencies(&meta.dependencies),
                2 => Self::Scalar(N::U64(meta.base_version)),
                3 => Self::Scalar(N::String(&meta.author_id.as_ref().ok_or_else(invalid_path)?.0)),
                4 => Self::Clock(&meta.timestamp),
                5 => Self::Scalar(N::String(match meta.undo_policy {
                    UndoPolicy::ExactBaseOnly => "ExactBaseOnly",
                    UndoPolicy::TransformAgainstConcurrent => "TransformAgainstConcurrent",
                    UndoPolicy::SemanticUndo => "SemanticUndo",
                    UndoPolicy::CompensatingAction => "CompensatingAction",
                })),
                6 => Self::Hash(&meta.payload_hash.as_ref().ok_or_else(invalid_path)?.0),
                7 => Self::Scalar(N::String(&meta.semantic_kind.as_ref().ok_or_else(invalid_path)?.0)),
                8 => Self::Scalar(N::String(meta.label.as_deref().ok_or_else(invalid_path)?)),
                9 => Self::Scalar(N::String(meta.group_id.as_deref().ok_or_else(invalid_path)?)),
                10 => Self::Origin(&meta.origin),
                _ => return Err(invalid_path()),
            },
            Self::Clock(clock) => Self::Scalar(N::U64(match index {
                0 => clock.actor,
                1 => clock.physical_ms,
                2 => clock.logical,
                _ => return Err(invalid_path()),
            })),
            Self::Dependencies(values) => Self::Scalar(N::String(&values.get(index).ok_or_else(invalid_path)?.0)),
            Self::Hash(value) => Self::Scalar(N::U64(u64::from(*value.get(index).ok_or_else(invalid_path)?))),
            Self::Origin(origin) => match origin {
                crate::os_spr::MutationOrigin::Owner if index == 0 => Self::Scalar(N::String("owner")),
                crate::os_spr::MutationOrigin::Contributed { plugin_id, mutation_id, payload_hash } => match index {
                    0 => Self::Scalar(N::String("contributed")),
                    1 => Self::Scalar(N::String(plugin_id)),
                    2 => Self::Scalar(N::String(&mutation_id.0)),
                    3 => Self::Hash(&payload_hash.0),
                    _ => return Err(invalid_path()),
                },
                crate::os_spr::MutationOrigin::Transaction { initiator } => match index {
                    0 => Self::Scalar(N::String("transaction")),
                    1 => Self::Target(initiator),
                    _ => return Err(invalid_path()),
                },
                _ => return Err(invalid_path()),
            },
            Self::Target(target) => match field_at(&self.fields(), index)?.0 {
                0 => Self::Scalar(N::String(&target.artifact_id)),
                1 => Self::Scalar(N::String(&target.artifact_kind)),
                2 => Self::Scalar(N::String(target.dialect.as_deref().ok_or_else(invalid_path)?)),
                _ => return Err(invalid_path()),
            },
            _ => return Err(invalid_path()),
        })
    }

    fn node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'a>, ValueError> {
        if let Self::Mutation(value) = self {
            return value.canonical_json_node(path);
        }
        if let Some((first, rest)) = path.split_first() {
            return self.child(*first)?.node(rest);
        }
        Ok(match self {
            Self::Mutations(values) => ArtifactCanonicalJsonNode::Array(values.len()),
            Self::PagedMutations(values) => ArtifactCanonicalJsonNode::Array(values.len()),
            Self::Metas(values) => ArtifactCanonicalJsonNode::Array(values.len()),
            Self::Dependencies(values) => ArtifactCanonicalJsonNode::Array(values.len()),
            Self::Hash(_) => ArtifactCanonicalJsonNode::Array(32),
            Self::Scalar(value) => *value,
            _ => ArtifactCanonicalJsonNode::Object(self.fields().iter().filter(|(_, present)| *present).count()),
        })
    }

    fn key(&self, path: &[usize], index: usize) -> Result<ArtifactCanonicalJsonText<'a>, ValueError> {
        if let Self::Mutation(value) = self {
            return value.canonical_json_key(path, index);
        }
        if let Some((first, rest)) = path.split_first() {
            return self.child(*first)?.key(rest, index);
        }
        field_at(&self.fields(), index).map(|(_, name)| name.into())
    }


}

impl<M: ArtifactCanonicalJson> ArtifactCanonicalJson for Edit<M> {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, ValueError> {
        CanonicalEditNode::Edit(self).node(path)
    }
    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> {
        CanonicalEditNode::Edit(self).key(path, index)
    }
}
//#endregion 🧬️TypedCanonicalSource

//#region 🔣️ByteEncoder
#[derive(Clone, Copy, Default)]
struct JsonFrame {
    kind: u8,
    phase: u8,
    index: usize,
    length: usize,
    offset: usize,
    chunk: usize,
}



/// 🔣️ Fixed-state canonical JSON encoder. Each emitted byte is actual work; strings are read
/// one source byte at a time and escape expansion is retained between arbitrarily small grants.
pub struct ArtifactCanonicalJsonCursor {
    frames: [JsonFrame; ARTIFACT_CANONICAL_JSON_DEPTH],
    path: [usize; ARTIFACT_CANONICAL_JSON_DEPTH],
    depth: usize,
    maximum_depth: usize,
    scalar: ScalarBytes,
    escape: [u8; 6],
    escape_length: usize,
    escape_offset: usize,
}

impl Default for ArtifactCanonicalJsonCursor {
    fn default() -> Self {
        Self {
            frames: [JsonFrame::default(); ARTIFACT_CANONICAL_JSON_DEPTH],
            path: [0; ARTIFACT_CANONICAL_JSON_DEPTH],
            depth: 1,
            maximum_depth: ARTIFACT_CANONICAL_JSON_DEPTH,
            scalar: ScalarBytes { bytes: [0; 64], length: 0 },
            escape: [0; 6],
            escape_length: 0,
            escape_offset: 0,
        }
    }
}

impl ArtifactCanonicalJsonCursor {
    pub fn next_encode_demand(&self) -> RetirementDemand {
        if self.is_complete() { return RetirementDemand::default(); }
        let frame = self.frames[self.depth - 1];
        let pushing = matches!((frame.kind, frame.phase), (2, 1) if frame.index != frame.length) || matches!((frame.kind, frame.phase), (3, 5));
        let copy_bytes = match (frame.kind, frame.phase) {
            (0, _) => 64,
            (1, 1) | (3, 2) => 7,
            _ => 1,
        };
        RetirementDemand { copy_bytes, depth: self.depth + usize::from(pushing), ..Default::default() }
    }

    pub fn next_encode_output_bound(&self) -> usize {
        if self.is_complete() { return 0; }
        let frame = self.frames[self.depth - 1];
        usize::from(match (frame.kind, frame.phase) {
            (0, _) | (3, 5) => false,
            (2, 1) => frame.index == frame.length,
            (4, _) => frame.offset < self.scalar.length,
            _ => true,
        })
    }

    pub fn encode_chunk_admitted(&mut self, source: &(impl ArtifactCanonicalJson + ?Sized), output: &mut [u8], grant: RetainedCloneGrant) -> Result<ArtifactCanonicalJsonTreeStep, ArtifactCanonicalJsonEncodeError> {
        if self.is_complete() { return Ok(ArtifactCanonicalJsonTreeStep { ownership: RetainedCloneStep::Complete(Default::default()), written_bytes: 0 }); }
        let demand = self.next_encode_demand();
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < demand.copy_bytes || output.is_empty() { return Ok(ArtifactCanonicalJsonTreeStep { ownership: RetainedCloneStep::Progress(Default::default()), written_bytes: 0 }); }
        if demand.depth > grant.maximum_depth || demand.depth > self.maximum_depth {
            return Err(ArtifactCanonicalJsonEncodeError { written_bytes: 0, reason: ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "canonical-edit.depth-limit") });
        }
        let top = self.depth - 1;
        let kind = self.frames[top].kind;
        let phase = self.frames[top].phase;
        let mut copied = 0;
        let result = (|| -> Result<Option<u8>, ValueError> {
            if kind == 0 {
                let node = source.canonical_json_node(&self.path[..top])?;
                let tag = match node {
                    ArtifactCanonicalJsonNode::String(_) | ArtifactCanonicalJsonNode::Text(_) => 1,
                    ArtifactCanonicalJsonNode::Array(length) => { self.frames[top].length = length; 2 }
                    ArtifactCanonicalJsonNode::Object(length) => { self.frames[top].length = length; 3 }
                    value => {
                        copied += self.scalar.write_node(value)?.copied_bytes;
                        4
                    }
                };
                self.frames[top].kind = tag;
                return Ok(None);
            }
            match (kind, phase) {
                (1, 0) => { self.frames[top].phase = 1; Ok(Some(b'"')) }
                (1, 1) | (3, 2) => {
                    if self.escape_offset < self.escape_length {
                        let byte = self.escape[self.escape_offset]; self.escape_offset += 1; return Ok(Some(byte));
                    }
                    let text = if kind == 1 {
                        match source.canonical_json_node(&self.path[..top])? {
                            ArtifactCanonicalJsonNode::String(text) => text.into(),
                            ArtifactCanonicalJsonNode::Text(text) => text,
                            _ => return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical-edit.source-shape-changed")),
                        }
                    } else { source.canonical_json_key(&self.path[..top], self.frames[top].index)? };
                    let state = &mut self.frames[top];
                    let byte = text.next_byte(&mut state.chunk, &mut state.offset).map_err(|reason| ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, reason));
                    match byte? {
                        Some(byte) => { self.escape_length = canonical_escape(byte, &mut self.escape); self.escape_offset = 1; copied += self.escape_length; Ok(Some(self.escape[0])) }
                        None => { state.phase = if kind == 1 { 2 } else { 3 }; Ok(None) }
                    }
                }
                (1, 2) => { self.depth -= 1; Ok(Some(b'"')) }
                (2, 0) => { self.frames[top].phase = 1; Ok(Some(b'[')) }
                (2, 1) if self.frames[top].index == self.frames[top].length => { self.depth -= 1; Ok(Some(b']')) }
                (2, 1) | (3, 5) => {
                    self.path[top] = self.frames[top].index; self.frames[self.depth] = JsonFrame::default(); self.depth += 1; self.frames[top].phase = if kind == 2 { 2 } else { 6 };
 Ok(None)
                }
                (2, 2) | (3, 6) => {
                    self.frames[top].index += 1; self.frames[top].phase = 1; 
                    Ok((self.frames[top].index < self.frames[top].length).then_some(b','))
                }
                (3, 0) => { self.frames[top].phase = 1; Ok(Some(b'{')) }
                (3, 1) if self.frames[top].index == self.frames[top].length => { self.depth -= 1; Ok(Some(b'}')) }
                (3, 1) => { self.frames[top].phase = 2; self.frames[top].offset = 0; self.frames[top].chunk = 0; copied += 1 + 2 * std::mem::size_of::<usize>(); Ok(Some(b'"')) }
                (3, 3) => { self.frames[top].phase = 4; Ok(Some(b'"')) }
                (3, 4) => { self.frames[top].phase = 5; Ok(Some(b':')) }
                (4, _) if self.frames[top].offset == self.scalar.length => { self.depth -= 1; Ok(None) }
                (4, _) => { let byte = self.scalar.bytes[self.frames[top].offset]; self.frames[top].offset += 1; Ok(Some(byte)) }
                _ => Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical-edit.encoder-state")),
            }
        })();
        let byte = result.map_err(|error| {
            let original = error.retained_progress();
            let progress = RetainedCloneProgress { copied_items: original.copied_items.max(usize::from(copied != 0)), copied_bytes: original.copied_bytes.checked_add(copied).expect("bounded canonical event copy receipt is representable"), ..original };
            ArtifactCanonicalJsonEncodeError { written_bytes: 0, reason: error.with_retained_progress(progress) }
        })?;
        if let Some(byte) = byte { output[0] = byte; copied += 1; }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: copied, ..Default::default() };
        Ok(ArtifactCanonicalJsonTreeStep { ownership: if self.is_complete() { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) }, written_bytes: usize::from(byte.is_some()) })
    }

    pub fn is_complete(&self) -> bool { self.depth == 0 }
}
//#endregion 🔣️ByteEncoder

//#region 🔏️Sealing
#[path="🪪️authority/🦀️.rs"]
mod publication_authority_retirement;

pub(super) struct ArtifactStoreOneItemAuthorityRetirement {
    owner: semio_framework_value::retirement::shared::SharedControlledRetirement<ArtifactStoreOneItemLiveAuthority>,
}

impl ArtifactStoreOneItemAuthorityRetirement {
    pub(super) fn new(authority: Arc<ArtifactStoreOneItemLiveAuthority>) -> Self {
        Self { owner: semio_framework_value::retirement::shared::SharedControlledRetirement::lease(authority) }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreOneItemAuthorityRetirement {
    fn next_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.owner.next_copy_byte_demand() }
    fn next_capacity_byte_demand(&self, maximum_body_bytes: usize) -> Result<usize, semio_framework_value::ValueError> { self.owner.next_capacity_byte_demand(maximum_body_bytes) }
    fn next_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.owner.next_release_byte_demand() }
    fn next_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> { self.owner.next_depth_demand() }
    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_value::retained_clone::RetainedCloneStep, semio_framework_value::ValueError> { self.owner.step(grant) }
    fn terminal_is_empty(&self) -> bool { self.owner.terminal_is_empty() }
}


/// 📍️ Portable replay witness. Restoration re-executes each prior byte and verifies this prefix;
/// no supplied hash state or digest can directly create publication authority.
/// 🔮️ serde stays TEST-ONLY: this file's own round-trip test (`serde_json::to_vec`/
/// `from_slice` against `checkpoint()`) uses it as an independent differential oracle. Production
/// never serializes this type through serde.
#[derive(Clone, Copy, Debug, PartialEq, Eq, ToValue, FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", deny_unknown_fields))]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct ArtifactStoreOneItemSealCheckpoint {
    pub version: u8,
    pub operation: u64,
    pub generation: u64,
    pub base_revision: [u8; 32],
    pub authority_digest: [u8; 32],
    pub phase: u8,
    pub completed_bytes: u64,
    pub canonical_bytes: u64,
    pub prefix_digest: [u8; 32],
}

/// 🔏️ Exact Store-owned edit, post-root, encoding state, and retirement lifecycle.
pub struct ArtifactStoreOneItemSealer<P, M> {
    native: Option<Box<dyn ArtifactStoreCanonicalOwner<M>>>,
    native_frame_bytes: usize,
    native_identities: Option<[semio_framework_value::paged::PagedUtf8<{usize::MAX}>;3]>,
    native_identities_close: Option<semio_framework_value::retirement::controlled::ControlledRetirement<(semio_framework_value::paged::PagedUtf8<{usize::MAX}>,semio_framework_value::paged::PagedUtf8<{usize::MAX}>,semio_framework_value::paged::PagedUtf8<{usize::MAX}>)>>,
    prepared_digest: Option<[u8;32]>,
    ownership: RetainedCloneProgress,
    authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    edit: Option<Box<Edit<M>>>,
    unboxed_edit: Option<Edit<M>>,
    post: Option<Arc<P>>,
    prepared: Option<ArtifactStoreOneItemPrepared<P, M>>,
    hash: semio_framework_hash::Sha256,
    transcript: semio_framework_hash::Sha256,
    phase: u8,
    header_offset: usize,
    canonical_bytes: u64,
    hashed_canonical_bytes: u64,
    completed_bytes: u64,
    turns: u32,
    last_chunk: [u8; ARTIFACT_CANONICAL_JSON_CHUNK_BYTES],
    last_length: usize,
    last_canonical: bool,
    replay: Option<ArtifactStoreOneItemSealCheckpoint>,
    mutation_retirement: Option<Arc<dyn ArtifactOwnedValueRetirementFactory<M>>>,
    snapshot_retirement: Option<Arc<dyn SnapshotRetirementFactory<P>>>,
    active_retirement: Option<Box<dyn ErasedSnapshotRetirement>>,
    retirement_strings: [Option<String>; 3],
    factory_close: [Option<FactoryAuthority>; 2],
    identities: [Vec<u8>; 3],
    identity_index: usize,
    cancelled: bool,
    closing: bool,
}

impl<P, M> ArtifactStoreOneItemSealer<P, M> {
    /// 🎟️ Measures the original edit/source frames before native ownership transfer.
    pub fn constructor_demand() -> semio_framework_value::retained_clone::RetainedCloneBirthDemand where M: semio_framework_value::retirement::RetireOwned+Sync+ArtifactCanonicalJsonTree {
        semio_framework_value::retained_clone::RetainedCloneBirthDemand {
            capacity_bytes: ArtifactStoreCanonicalSource::<M>::source_constructor_demand().capacity_bytes,
            depth: 1,
        }
    }
    pub fn constructor_copy_byte_demand()->usize where M:semio_framework_value::retirement::RetireOwned+Sync+ArtifactCanonicalJsonTree{std::mem::size_of::<Edit<M>>()+ArtifactStoreCanonicalSource::<M>::source_constructor_demand().copy_bytes+std::mem::size_of::<Self>()}

    /// 🧳️ Retains every exact original input when constructor capacity or depth is refused.
    pub fn admit(
        authority: Arc<ArtifactStoreOneItemLiveAuthority>,
        edit: Edit<M>,
        post: Arc<P>,
        mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>,
        snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>,
        grant: RetainedCloneGrant,
    ) -> Result<(Self, RetainedCloneProgress), (ValueError, Arc<ArtifactStoreOneItemLiveAuthority>, Edit<M>, Arc<P>, Arc<dyn ArtifactOwnedValueRetirementFactory<M>>, Arc<dyn SnapshotRetirementFactory<P>>)> where M: semio_framework_value::retirement::RetireOwned+Sync+ArtifactCanonicalJsonTree {
        if grant.maximum_copy_bytes<Self::constructor_copy_byte_demand(){return Err((ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"canonical original source constructor lacks input transfer copy grant"),authority,edit,post,mutation_retirement,snapshot_retirement));}
        let progress = match Self::constructor_demand().admit(grant) {
            Ok(progress) => progress,
            Err(error) => return Err((error, authority, edit, post, mutation_retirement, snapshot_retirement)),
        };
        Ok((Self::new(authority, edit, post, mutation_retirement, snapshot_retirement), RetainedCloneProgress{copied_bytes:Self::constructor_copy_byte_demand(),..progress}))
    }

    pub(super) fn new(authority: Arc<ArtifactStoreOneItemLiveAuthority>, edit: Edit<M>, post: Arc<P>, mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>, snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>) -> Self where M: semio_framework_value::retirement::RetireOwned+Sync+ArtifactCanonicalJsonTree {
        Self {
            native: Some(Box::new(ArtifactStoreCanonicalSource::new(Arc::clone(&authority),Box::new(edit)))),
            native_frame_bytes:std::mem::size_of::<ArtifactStoreCanonicalSource<M>>(),
            native_identities:None,
            native_identities_close:None,
            prepared_digest:None,
            ownership:Default::default(),
            authority: Some(authority),
            edit: None,
            unboxed_edit: None,
            post: Some(post),
            prepared: None,
            hash: semio_framework_hash::Sha256::new(),
            transcript: semio_framework_hash::Sha256::new(),
            phase: 0,
            header_offset: 0,
            canonical_bytes: 0,
            hashed_canonical_bytes: 0,
            completed_bytes: 0,
            turns: 0,
            last_chunk: [0; ARTIFACT_CANONICAL_JSON_CHUNK_BYTES],
            last_length: 0,
            last_canonical: false,
            replay: None,
            mutation_retirement: Some(mutation_retirement),
            snapshot_retirement: Some(snapshot_retirement),
            active_retirement: None,
            retirement_strings: Default::default(),
            factory_close: Default::default(),
            identities: std::array::from_fn(|_| Vec::new()),
            identity_index: 0,
            cancelled: false,
            closing: false,
        }
    }

    /// 🌐️ The original completed domain cursor funds this scalar declaration after sealing.
    pub fn record_foreign_step_presence(&mut self,presence:bool)->bool{if let Some(prepared)=self.prepared.as_mut(){prepared.record_foreign_step_presence(presence);true}else{false}}

    pub fn prepared(&self) -> Option<&ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.as_ref()
    }
    pub fn take_prepared(&mut self) -> Option<ArtifactStoreOneItemPrepared<P, M>> {
        self.prepared.take()
    }
    pub fn cancel(&mut self) {
        self.cancelled = true;
    }
    pub fn begin_close(&mut self) {
        self.closing = true;
        self.replay = None;
        if let Some(owner)=self.native.as_mut(){owner.begin_close();}
    }
    pub fn canonical_chunk(&self) -> &[u8] {
        if self.last_canonical { &self.last_chunk[..self.last_length] } else { &[] }
    }

    pub fn checkpoint(&self) -> ArtifactStoreOneItemSealCheckpoint {
        let authority = self.authority.as_ref().expect("nonterminal sealer retains its authority");
        ArtifactStoreOneItemSealCheckpoint {
            version: 1,
            operation: authority.operation.0,
            generation: authority.generation.0,
            base_revision: authority.base_revision,
            authority_digest: self.authority_digest(),
            phase: self.phase,
            completed_bytes: self.completed_bytes,
            canonical_bytes: self.canonical_bytes,
            prefix_digest: self.transcript.clone().finalize(),
        }
    }

    pub fn restore_checkpoint(&mut self, checkpoint: ArtifactStoreOneItemSealCheckpoint) -> Result<(), String> {
        let authority = self.authority.as_ref().ok_or_else(|| "canonical-edit.authority-missing".to_string())?;
        if self.phase != 0
            || self.completed_bytes != 0
            || self.cancelled
            || self.closing
            || checkpoint.version != 1
            || checkpoint.phase > 6
            || checkpoint.operation != authority.operation.0
            || checkpoint.generation != authority.generation.0
            || checkpoint.base_revision != authority.base_revision
            || checkpoint.authority_digest != self.authority_digest()
            || checkpoint.canonical_bytes > CANONICAL_EDIT_MAXIMUM_BYTES
            || checkpoint.completed_bytes > CANONICAL_EDIT_MAXIMUM_BYTES * 2 + CANONICAL_EDIT_MAXIMUM_OVERHEAD_BYTES
        {
            return Err("canonical-edit.checkpoint-authority".into());
        }
        self.replay = Some(checkpoint);
        Ok(())
    }

    fn progress(&self) -> ArtifactStoreOneItemCheckpoint {
        ArtifactStoreOneItemCheckpoint { cursor: self.turns, completed_items: self.turns, completed_bytes: self.completed_bytes, digest: self.prepared.as_ref().map(ArtifactStoreOneItemPrepared::edit_digest).or(self.prepared_digest).unwrap_or_else(||self.transcript.clone().finalize()) }
    }

    fn verify_replay(&mut self) -> Result<(), String> {
        let Some(target) = self.replay else {
            return Ok(());
        };
        if self.completed_bytes == target.completed_bytes && self.phase == target.phase {
            if self.canonical_bytes != target.canonical_bytes || self.transcript.clone().finalize() != target.prefix_digest {
                return Err("canonical-edit.checkpoint-prefix".into());
            }
            self.replay = None;
        } else if self.completed_bytes > target.completed_bytes || self.phase > target.phase || self.phase == 6 {
            return Err("canonical-edit.checkpoint-position".into());
        }
        Ok(())
    }

    fn header_byte(&self, offset: usize) -> Option<u8> {
        let id = self.edit.as_ref()?.id.as_bytes();
        let fixed = b"semio.artifact.cursor.v2";
        let mut at = offset;
        for segment in [fixed.as_slice(), &4u64.to_be_bytes(), b"edit".as_slice(), &(id.len() as u64).to_be_bytes(), id, &self.canonical_bytes.to_be_bytes()] {
            if at < segment.len() {
                return Some(segment[at]);
            }
            at -= segment.len();
        }
        None
    }

    fn authority_digest(&self) -> [u8; 32] {
        let authority = self.authority.as_ref().expect("nonterminal sealer retains authority");
        CursorRevisionAccumulator::hash_record(
            b"one-item-authority",
            &[
                &authority.operation.0.to_be_bytes(),
                &authority.generation.0.to_be_bytes(),
                &authority.base_revision,
                &authority.next_sequence_number.to_be_bytes(),
                &authority.next_clock.actor.to_be_bytes(),
                &authority.next_clock.physical_ms.to_be_bytes(),
                &authority.next_clock.logical.to_be_bytes(),
                authority.actor.as_bytes(),
                &[u8::from(authority.line.is_some())],
                authority.line.as_deref().unwrap_or("").as_bytes(),
                &[u8::from(authority.group_id.is_some())],
                authority.group_id.as_deref().unwrap_or("").as_bytes(),
            ],
        )
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.native.is_none()
            && self.native_identities.is_none()
            && self.native_identities_close.is_none()
            && self.authority.is_none()
            && self.edit.is_none()
            && self.unboxed_edit.is_none()
            && self.post.is_none()
            && self.prepared.is_none()
            && self.active_retirement.is_none()
            && self.retirement_strings.iter().all(Option::is_none)
            && self.mutation_retirement.is_none()
            && self.snapshot_retirement.is_none()
            && self.identities.iter().all(|value| value.capacity() == 0)
            && self.factory_close.iter().all(Option::is_none)
    }
}

impl<P: Send + Sync + 'static, M: Send + 'static> ArtifactStoreOneItemSealer<P, M> {
    /// 📐️ Quotes the actual selected native child or the separately retained source-frame/publication handoff.
    pub fn preparation_demands(&self) -> Result<RetirementDemand,ValueError> {
        if let Some(owner)=self.native.as_ref(){
            if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<Box<dyn ArtifactStoreCanonicalOwner<M>>>>(),release_bytes:self.native_frame_bytes,depth:1,..Default::default()});}
            let mut demand=owner.next_demand()?;demand.depth=demand.depth.checked_add(1).ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit,"canonical preparation child depth overflow"))?;return Ok(demand);
        }
        if self.prepared.is_some(){return Ok(Default::default());}
        Ok(RetirementDemand{copy_bytes:std::mem::size_of::<ArtifactStoreOneItemPrepared<P,M>>()+std::mem::size_of::<Box<Edit<M>>>()+std::mem::size_of::<Arc<P>>()+std::mem::size_of::<Arc<ArtifactStoreOneItemLiveAuthority>>()+std::mem::size_of::<[semio_framework_value::paged::PagedUtf8<{usize::MAX}>;3]>(),depth:1,..Default::default()})
    }
    /// 🧾️ Returns every actual currency from the preceding canonical preparation turn.
    pub fn ownership_progress(&self)->RetainedCloneProgress{self.ownership}
    pub fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, ValueError> {
        self.ownership=Default::default();self.last_length=0;self.last_canonical=false;
        if !grant.permits_one()||self.cancelled||self.closing{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
        if self.replay.is_some(){self.cancelled=true;return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"canonical-edit.native-checkpoint-unsupported"));}
        if self.prepared.is_some(){return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.progress(),self.ownership));}
        let demand=match self.preparation_demands(){Ok(demand)=>demand,Err(error)=>{self.cancelled=true;self.ownership=error.retained_progress();return Err(error)}};
        if demand.copy_bytes>grant.maximum_copy_bytes||demand.capacity_bytes>grant.maximum_capacity_bytes||demand.release_bytes>grant.maximum_release_bytes||demand.depth>grant.maximum_depth{return Ok(ArtifactStoreOneItemPreparationStep::Blocked);}
        let retained=grant.retained_grant();
        let result=(||->Result<RetainedCloneProgress,ValueError>{
            if let Some(owner)=self.native.as_mut(){
                if owner.terminal_is_empty(){self.native=None;self.phase=5;return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()});}
                let child=RetainedCloneGrant{maximum_items:1,maximum_depth:retained.maximum_depth-1,..retained};
                if owner.ready(){
                    let(edit,identities,digest,receipt)=owner.take(child)?.ok_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"canonical-edit.original-return-refused"))?;
                    self.edit=Some(edit);self.native_identities=Some(identities);self.prepared_digest=Some(digest);return Ok(receipt);
                }
                return Ok(owner.advance(child)?.progress());
            }
            if self.authority.is_none()||self.edit.is_none()||self.post.is_none()||self.native_identities.is_none()||self.prepared_digest.is_none(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"canonical-edit.original-prepared-owners-missing"));}
            let authority=self.authority.as_ref().expect("validated original publication authority");
            let edit=self.edit.take().expect("validated original edit");
            let post=self.post.take().expect("validated original post");
            let identities=self.native_identities.take().expect("validated original identities");
            let digest=self.prepared_digest.take().expect("validated original digest");
            self.prepared=Some(authority.seal_prepared_owned(edit,post,digest,identities));self.phase=6;
            Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})
        })();
        let (progress,error)=match result{Ok(progress)=>(progress,None),Err(error)=>(error.retained_progress(),Some(error))};
        self.ownership=progress;
        if error.is_some()||!progress.fits(retained){self.cancelled=true;}
        let bytes=progress.copied_bytes.checked_add(progress.retained_capacity_bytes).and_then(|bytes|bytes.checked_add(progress.released_bytes)).and_then(|bytes|u64::try_from(bytes).ok());
        if progress!=RetainedCloneProgress::default(){
            let Some((completed,turns))=bytes.and_then(|bytes|self.completed_bytes.checked_add(bytes)).zip(self.turns.checked_add(1))else{self.cancelled=true;return Err(error.unwrap_or_else(||ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"canonical-edit.work-overflow").with_retained_progress(progress)));};
            self.completed_bytes=completed;self.turns=turns;
        }
        if let Some(error)=error{return Err(error);}
        if !progress.fits(retained){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::OwnershipLimit,"canonical-edit.original-child-receipt-exceeds-grant").with_retained_progress(progress));}
        Ok(if self.prepared.is_some(){ArtifactStoreOneItemPreparationStep::Prepared(self.progress(),progress)}else if progress==RetainedCloneProgress::default(){ArtifactStoreOneItemPreparationStep::Blocked}else{ArtifactStoreOneItemPreparationStep::Progress(self.progress(),progress)})
    }
}

impl<P: Send + Sync + 'static, M: Send + 'static> ArtifactStoreOneItemSealer<P, M> {
    pub fn retirement_demands(&self, maximum_body_bytes: usize) -> Result<RetirementDemand, ValueError> {
        let depth_error = || ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "canonical sealer depth overflow");
        if let Some(owner)=self.native.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<Option<Box<dyn ArtifactStoreCanonicalOwner<M>>>>(),release_bytes:self.native_frame_bytes,depth:1,..Default::default()});}let mut demand=owner.next_demand()?;demand.depth=demand.depth.checked_add(1).ok_or_else(depth_error)?;return Ok(demand);}
        if self.native_identities.is_some(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of::<[semio_framework_value::paged::PagedUtf8<{usize::MAX}>;3]>()+std::mem::size_of::<semio_framework_value::retirement::controlled::ControlledRetirement<(semio_framework_value::paged::PagedUtf8<{usize::MAX}>,semio_framework_value::paged::PagedUtf8<{usize::MAX}>,semio_framework_value::paged::PagedUtf8<{usize::MAX}>)>>(),depth:1,..Default::default()});}
        if let Some(owner)=self.native_identities_close.as_ref(){if owner.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:std::mem::size_of_val(&self.native_identities_close),depth:1,..Default::default()});}let body=owner.next_copy_byte_demand()?;return Ok(RetirementDemand{copy_bytes:body,capacity_bytes:owner.next_capacity_byte_demand(body)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?.checked_add(1).ok_or_else(depth_error)?});}
        if let Some(active) = self.active_retirement.as_ref() { let mut demand = super::artifact_retirement_box_demands(active, maximum_body_bytes)?; demand.depth = demand.depth.checked_add(1).ok_or_else(depth_error)?; return Ok(demand); }
        if let Some(bytes) = self.identities.iter().find(|bytes| bytes.capacity() != 0) { return Ok(RetirementDemand { release_bytes: bytes.capacity(), depth: 1, ..Default::default() }); }
        if self.prepared.is_some() { let birth=ArtifactStoreOneItemPrepared::<P,M>::retirement_birth_demand();return Ok(RetirementDemand { capacity_bytes:birth.capacity_bytes,depth:birth.depth+1, ..Default::default() }); }
        if self.edit.is_some() { return Ok(RetirementDemand { release_bytes: std::mem::size_of::<Edit<M>>(), depth: 1, ..Default::default() }); }
        if self.unboxed_edit.is_some() { return Ok(RetirementDemand { capacity_bytes: std::mem::size_of::<ArtifactStoreDecodedEditRetirement<M>>(), depth: 2, ..Default::default() }); }
        if let Some(post) = self.post.as_ref() { return Ok(RetirementDemand { capacity_bytes: self.snapshot_retirement.as_ref().expect("original sealer snapshot factory").retirement_birth_bytes(post), depth: 2, ..Default::default() }); }
        if let Some(value) = self.retirement_strings.iter().find_map(Option::as_ref) { return Ok(RetirementDemand { release_bytes: value.capacity(), depth: 1, ..Default::default() }); }
        if let Some(authority) = self.authority.as_ref() { let birth = authority.retirement_birth_demand(); return Ok(RetirementDemand { capacity_bytes: birth.capacity_bytes, depth: birth.depth.checked_add(1).ok_or_else(depth_error)?, ..Default::default() }); }
        if self.mutation_retirement.is_some() || self.snapshot_retirement.is_some() { return Ok(RetirementDemand { copy_bytes: std::mem::size_of::<Arc<dyn semio_framework_value::FactoryRetirement>>(), depth: 1, ..Default::default() }); }
        if let Some(factory) = self.factory_close.iter().find_map(Option::as_ref) { let mut demand = factory.demands(maximum_body_bytes)?; demand.depth = demand.depth.checked_add(1).ok_or_else(depth_error)?; return Ok(demand); }
        Ok(Default::default())
    }
    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.terminal_is_empty() { return Ok(RetainedCloneStep::Complete(Default::default())); }
        if !self.closing || grant.maximum_items == 0 { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let demand = self.retirement_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_depth < demand.depth { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::DepthLimit, "canonical sealer exceeds admitted depth")); }
        if demand.copy_bytes > grant.maximum_copy_bytes || demand.capacity_bytes > grant.maximum_capacity_bytes || demand.release_bytes > grant.maximum_release_bytes { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if let Some(owner)=self.native.as_mut(){if owner.terminal_is_empty(){self.native=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}));}let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};return owner.advance(child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if let Some([a,b,c])=self.native_identities.take(){match semio_framework_value::retirement::controlled::ControlledRetirement::new((a,b,c)){Ok(owner)=>self.native_identities_close=Some(owner),Err((error,(a,b,c)))=>{self.native_identities=Some([a,b,c]);return Err(error);}}return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}
        if let Some(owner)=self.native_identities_close.as_mut(){if owner.terminal_is_empty(){self.native_identities_close=None;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}));}let child=RetainedCloneGrant{maximum_items:1,maximum_depth:grant.maximum_depth-1,..grant};return owner.step(child).map(|step|RetainedCloneStep::Progress(step.progress()));}
        let child = RetainedCloneGrant { maximum_items: 1, maximum_depth: grant.maximum_depth - 1, ..grant };
        if self.active_retirement.is_some() { return super::artifact_retirement_box_close_step(&mut self.active_retirement, child).map(|step| RetainedCloneStep::Progress(step.progress())); }
        if let Some(bytes) = self.identities.iter_mut().find(|bytes| bytes.capacity() != 0) {
            let released_bytes = bytes.capacity();
            *bytes = Vec::new();
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }));
        }
        if let Some(prepared) = self.prepared.take() {
            let mutation_factory=self.mutation_retirement.take().expect("original sealer mutation issuer");let snapshot_factory=self.snapshot_retirement.take().expect("original sealer root issuer");
            return match prepared.admit_retirement(mutation_factory,snapshot_factory,child){Ok((owner,receipt))=>{self.active_retirement=Some(owner);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original,mutations,snapshots))=>{self.prepared=Some(original);self.mutation_retirement=Some(mutations);self.snapshot_retirement=Some(snapshots);Err(error)}};
        }
        if let Some(edit) = self.edit.take() {
            self.unboxed_edit = Some(*edit);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes: demand.release_bytes, ..Default::default() }));
        }
        if let Some(edit) = self.unboxed_edit.take() {
            let factory = self.mutation_retirement.as_ref().expect("original mutation factory");
            return match super::admit_artifact_retirement(edit, child, |original| ArtifactStoreDecodedEditRetirement::new(original, Arc::clone(factory))) {
                Ok((active, progress)) => { self.active_retirement = Some(active); Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original)) => { self.unboxed_edit = Some(original); Err(error) },
            };
        }
        if let Some(post) = self.post.take() {
            return match self.snapshot_retirement.as_ref().expect("original snapshot factory").retire(post, child) {
                Ok((active, progress)) => { self.active_retirement = Some(active); if !progress.fits(child) || progress.retained_capacity_bytes != demand.capacity_bytes { return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical sealer constructor changed its receipt")); } Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original)) => { self.post = Some(original); Err(error) },
            };
        }
        if let Some(value) = self.retirement_strings.iter_mut().find_map(Option::take) {
            let released_bytes = value.capacity();
            drop(value);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, released_bytes, ..Default::default() }));
        }
        if let Some(authority) = self.authority.take() {
            return match authority.retire(child) {
                Ok((active, progress)) => { self.active_retirement = Some(active); Ok(RetainedCloneStep::Progress(progress)) },
                Err((error, original)) => { self.authority = Some(original); Err(error) },
            };
        }
        if let Some(factory) = self.mutation_retirement.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factory_close[0] = Some(FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if let Some(factory) = self.snapshot_retirement.take() {
            let factory: Arc<dyn semio_framework_value::FactoryRetirement> = factory;
            self.factory_close[1] = Some(FactoryAuthority::new(factory));
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() }));
        }
        if let Some(slot) = self.factory_close.iter_mut().find(|slot| slot.is_some()) {
            let factory = slot.as_mut().unwrap();
            let step = factory.step(child)?;
            let step = semio_framework_value::retained_clone::admit_retained_clone_close(child, step, factory.terminal_is_empty(), "canonical sealer factory")?;
            if factory.terminal_is_empty() { *slot = None; }
            return Ok(if self.terminal_is_empty() { RetainedCloneStep::Complete(step.progress()) } else { RetainedCloneStep::Progress(step.progress()) });
        }
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
}

impl<P, M> Drop for ArtifactStoreOneItemSealer<P, M> {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "canonical edit sealer dropped before exact owners were transferred or retired");
    }
}
//#endregion 🔏️Sealing

//#region 🧪️CanonicalLaws
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️CanonicalLaws
