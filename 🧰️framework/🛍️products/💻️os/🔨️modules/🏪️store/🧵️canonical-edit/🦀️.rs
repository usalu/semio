//! 🔏️ Store-owned canonical JSON traversal, byte accounting, and exact one-item sealing.

use semio_framework_os_store_one_item::{ARTIFACT_STORE_ONE_ITEM_ID_BYTES, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, ArtifactStoreOneItemGrant};
use std::sync::Arc;
use semio_framework_value::{DslValue, ErasedSnapshotRetirement, SnapshotRetirementFactory, ValueError};
use semio_framework_os_spr::{Edit, HybridLogicalTimestamp, Mutation, MutationId, MutationMeta, UndoPolicy};
use serde::Serialize;
use semio_framework_value_snapshot_read::{artifact_retirement_box_close_step, artifact_retirement_box_demands};
use semio_framework_value::{RetirementDemand, FactoryAuthority, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneStep, RetainedCloneProgress}};




#[path="🪪️source-identity/🦀️.rs"]
mod source_identity;
pub use source_identity::{ArtifactCanonicalSourceIdentityCursor,ArtifactCanonicalSourceIdentityBirth,ArtifactCanonicalSourceIdentityStep};

#[path = "🌱️value/🦀️.rs"]
pub mod value;
pub use value::{ArtifactCanonicalValue, ArtifactCanonicalValueAdmission, ArtifactCanonicalValueCheckpoint, ArtifactCanonicalValueCloseStep, ArtifactCanonicalValueGrant, ArtifactCanonicalValueLimits, ArtifactCanonicalValueStep};

#[path = "🧵️borrowed/🦀️.rs"]
pub mod borrowed;
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
pub mod reader;
pub use reader::ArtifactCanonicalJsonReader;


//#region 🧬️TypedCanonicalSource
pub const ARTIFACT_CANONICAL_JSON_DEPTH: usize = 64;
pub const ARTIFACT_CANONICAL_JSON_CHUNK_BYTES: usize = 256;
pub const CANONICAL_EDIT_MAXIMUM_BYTES: u64 = 16 * ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES as u64;
pub const CANONICAL_EDIT_MAXIMUM_OVERHEAD_BYTES: u64 = b"semio.artifact.cursor.v2".len() as u64 + 3 * 8 + b"edit".len() as u64 + 4 * ARTIFACT_STORE_ONE_ITEM_ID_BYTES as u64;

/// 🌲️ Views one original mutation as its path-addressed canonical JSON without copying it: the wire of a mutation IS its canonical tree.
#[repr(transparent)]
pub struct ArtifactCanonicalTreeSource<M>(M);

impl<M: ArtifactCanonicalJsonTree> ArtifactCanonicalTreeSource<M> {
    /// 🔗️ Reborrows the original mutation in place as its canonical wire role.
    pub const fn view(mutation: &M) -> &Self {
        unsafe { &*(mutation as *const M).cast::<Self>() }
    }

    fn descend<'a>(&'a self, path: &[usize]) -> Result<&'a dyn ArtifactCanonicalJsonTree, ValueError> {
        let mut node: &dyn ArtifactCanonicalJsonTree = &self.0;
        for ordinal in path {
            node = node.canonical_tree_child(*ordinal)?;
        }
        Ok(node)
    }
}

impl<M: ArtifactCanonicalJsonTree> ArtifactCanonicalJson for ArtifactCanonicalTreeSource<M> {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, ValueError> {
        self.descend(path)?.canonical_tree_node()
    }

    fn canonical_json_key(&self, object_path: &[usize], index: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> {
        self.descend(object_path)?.canonical_tree_key(index)
    }
}

/// ⚠️ Exact initialized output prefix retained even when typed canonical traversal fails.
#[derive(Debug, PartialEq, Eq)]
pub struct ArtifactCanonicalJsonEncodeError {
    pub written_bytes: usize,
    pub reason: ValueError,
}

/// 🧭️ Exact serde field order over an immutable typed owner. Each lookup must perform bounded
/// indexed access; scanning, serialization, cloning, and collection inside these methods are forbidden.
pub trait ArtifactCanonicalJson: Sync {
    /// 🪪️ A temporary borrowed projection names its stable retained original owner without transferring ownership.
    fn canonical_json_original_owner(&self) -> Option<&dyn ArtifactCanonicalJson> {
        None
    }
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

/// 🌲️ Views one original mutation as its path-addressed canonical JSON without copying it: the wire of a mutation IS its canonical tree.
#[repr(transparent)]
pub struct ArtifactCanonicalTreeSource<M>(M);

impl<M: ArtifactCanonicalJsonTree> ArtifactCanonicalTreeSource<M> {
    /// 🔗️ Reborrows the original mutation in place as its canonical wire role.
    pub const fn view(mutation: &M) -> &Self {
        unsafe { &*(mutation as *const M).cast::<Self>() }
    }

    fn descend<'a>(&'a self, path: &[usize]) -> Result<&'a dyn ArtifactCanonicalJsonTree, ValueError> {
        let mut node: &dyn ArtifactCanonicalJsonTree = &self.0;
        for ordinal in path {
            node = node.canonical_tree_child(*ordinal)?;
        }
        Ok(node)
    }
}

impl<M: ArtifactCanonicalJsonTree> ArtifactCanonicalJson for ArtifactCanonicalTreeSource<M> {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, ValueError> {
        self.descend(path)?.canonical_tree_node()
    }

    fn canonical_json_key(&self, object_path: &[usize], index: usize) -> Result<ArtifactCanonicalJsonText<'_>, ValueError> {
        self.descend(object_path)?.canonical_tree_key(index)
    }
}

pub fn invalid_path() -> ValueError {
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
    PagedMutations(&'a semio_framework_value_persistent::list::PagedList<M, {usize::MAX}>),
    Metas(&'a [MutationMeta]),
    Meta(&'a MutationMeta),
    Clock(&'a HybridLogicalTimestamp),
    Dependencies(&'a [MutationId]),
    Hash(&'a [u8; 32]),
    Origin(&'a semio_framework_os_spr::MutationOrigin),
    Target(&'a semio_framework_os_spr::ForeignTarget),
    Scalar(ArtifactCanonicalJsonNode<'a>),
}

impl<'a, M: ArtifactCanonicalJson> CanonicalEditNode<'a, M> {
    /// 🔤 camelCase, because `MutationOrigin`'s own `ToValue` is the normative key
    /// spelling and this cursor must produce its bytes exactly. Its sibling
    /// `transaction` variant was already aligned; `contributed` alone was not, so a
    /// contributed edit's canonical form never matched its own value projection.
    pub fn fields(&self) -> [(&'static str, bool); 12] {
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
                semio_framework_os_spr::MutationOrigin::Owner => fields[0] = ("kind", true),
                semio_framework_os_spr::MutationOrigin::Contributed { .. } => fields[..4].copy_from_slice(&[("kind", true), ("pluginId", true), ("mutationId", true), ("payloadHash", true)]),
                semio_framework_os_spr::MutationOrigin::Transaction { .. } => fields[..2].copy_from_slice(&[("kind", true), ("initiator", true)]),
            },
            Self::Target(target) => fields[..3].copy_from_slice(&[("artifactId", true), ("artifactKind", true), ("dialect", target.dialect.is_some())]),
            _ => {}
        }
        fields
    }

    pub fn child(&self, index: usize) -> Result<Self, ValueError> {
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
                semio_framework_os_spr::MutationOrigin::Owner if index == 0 => Self::Scalar(N::String("owner")),
                semio_framework_os_spr::MutationOrigin::Contributed { plugin_id, mutation_id, payload_hash } => match index {
                    0 => Self::Scalar(N::String("contributed")),
                    1 => Self::Scalar(N::String(plugin_id)),
                    2 => Self::Scalar(N::String(&mutation_id.0)),
                    3 => Self::Hash(&payload_hash.0),
                    _ => return Err(invalid_path()),
                },
                semio_framework_os_spr::MutationOrigin::Transaction { initiator } => match index {
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

    pub fn node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'a>, ValueError> {
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

    pub fn key(&self, path: &[usize], index: usize) -> Result<ArtifactCanonicalJsonText<'a>, ValueError> {
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
    pub frames: [JsonFrame; ARTIFACT_CANONICAL_JSON_DEPTH],
    pub path: [usize; ARTIFACT_CANONICAL_JSON_DEPTH],
    pub depth: usize,
    pub maximum_depth: usize,
    pub scalar: ScalarBytes,
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
    /// 🔁️ Quotes only the live root metadata needed after a complete stream.
    pub fn restart_copy_bytes() -> usize { size_of::<JsonFrame>() + 4 * size_of::<usize>() }
    /// 🔁️ Starts another source without clearing or copying the retained frame array.
    pub fn restart_admitted(&mut self, grant: RetainedCloneGrant) -> Result<Option<RetainedCloneProgress>, ValueError> {
        if !self.is_complete() { return Err(ValueError::literal(ValueRefusalKind::InvariantViolated, "canonical JSON restart requires a complete stream")); }
        let bytes = Self::restart_copy_bytes();
        if grant.maximum_items == 0 || grant.maximum_copy_bytes < bytes || grant.maximum_depth == 0 { return Ok(None); }
        self.frames[0] = JsonFrame::default();
        self.depth = 1;
        self.scalar.length = 0;
        self.escape_length = 0;
        self.escape_offset = 0;
        Ok(Some(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }))
    }
    /// 📏️ Starts a fresh encoder whose depth grant is `maximum_depth` frames instead of the full inline capacity.
    pub fn with_maximum_depth(maximum_depth:usize)->Result<Self,ValueError>{if maximum_depth==0||maximum_depth>ARTIFACT_CANONICAL_JSON_DEPTH{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"canonical source depth policy exceeds its retained frame capacity"));}Ok(Self{maximum_depth,..Default::default()})}

    pub fn next_encode_demand(&self) -> RetirementDemand {
        if self.is_complete() { return RetirementDemand::default(); }
        let frame = self.frames[self.depth - 1];
        let pushing = matches!((frame.kind, frame.phase), (2, 1) if frame.index != frame.length) || matches!((frame.kind, frame.phase), (3, 5));
        let copy_bytes = match (frame.kind, frame.phase) {
            (0, _) => 64,
            (1, 1) | (3, 2) => 7,
            (3, 1) if frame.index != frame.length => 2 + 2 * size_of::<usize>(),
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
        if matches!((kind, phase), (1, 1) | (3, 2)) && self.escape_offset == self.escape_length {
            let text = if kind == 1 {
                match source.canonical_json_node(&self.path[..top]).map_err(|reason| ArtifactCanonicalJsonEncodeError { written_bytes: 0, reason })? {
                    ArtifactCanonicalJsonNode::String(text) => ArtifactCanonicalJsonText::Contiguous(text),
                    ArtifactCanonicalJsonNode::Text(text) => text,
                    _ => return Err(ArtifactCanonicalJsonEncodeError { written_bytes: 0, reason: ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical-edit.source-shape-changed") }),
                }
            } else {
                source.canonical_json_key(&self.path[..top], self.frames[top].index).map_err(|reason| ArtifactCanonicalJsonEncodeError { written_bytes: 0, reason })?
            };
            if let ArtifactCanonicalJsonText::Contiguous(text) = text {
                let start = self.frames[top].offset;
                let bytes = text.as_bytes().get(start..).ok_or_else(|| ArtifactCanonicalJsonEncodeError { written_bytes: 0, reason: ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated, "canonical-edit.text-offset") })?;
                let count = bytes.iter().take(output.len().min(grant.maximum_copy_bytes).min(64)).take_while(|byte| **byte >= 0x20 && **byte != b'"' && **byte != b'\\').count();
                if count != 0 {
                    output[..count].copy_from_slice(&bytes[..count]);
                    self.frames[top].offset += count;
                    return Ok(ArtifactCanonicalJsonTreeStep { ownership: RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: count, ..Default::default() }), written_bytes: count });
                }
            }
        }
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
                (3, 1) => { self.frames[top].phase = 2; self.frames[top].offset = 0; self.frames[top].chunk = 0; copied += 1 + 2 * size_of::<usize>(); Ok(Some(b'"')) }
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


















//#endregion 🔏️Sealing
//#region 🧪️CanonicalLaws

//#endregion 🧪️CanonicalLaws

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;
#[path="🔏️borrowed-digest/🦀️.rs"]
mod borrowed_digest;
pub use borrowed_digest::ArtifactCanonicalRecordDigest;
