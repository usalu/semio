//! 🔏️ Store-owned canonical JSON traversal, byte accounting, and exact one-item sealing.

use super::*;

#[path = "🧵️borrowed/🦀️.rs"]
mod borrowed;
use borrowed::ArtifactCanonicalEditEncoder;
pub use borrowed::{ArtifactCanonicalJsonArray, ArtifactCanonicalJsonObject, ArtifactCanonicalJsonValue};
#[path = "📖️reader/🦀️.rs"]
mod reader;
pub use reader::ArtifactCanonicalJsonReader;
#[cfg(test)]
#[path = "🧵️borrowed/🧪️tests/🦀️.rs"]
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
    pub reason: String,
}

/// 🧬️ A borrowed typed JSON node; callers cannot supply encoded bytes or a digest.
#[derive(Clone, Copy, Debug)]
pub enum ArtifactCanonicalJsonNode<'a> {
    Null,
    Bool(bool),
    I64(i64),
    U64(u64),
    I128(i128),
    U128(u128),
    F32(f32),
    F64(f64),
    String(&'a str),
    Array(usize),
    Object(usize),
}

/// 🧭️ Exact serde field order over an immutable typed owner. Each lookup must perform bounded
/// indexed access; scanning, serialization, cloning, and collection inside these methods are forbidden.
pub trait ArtifactCanonicalJson: Sync {
    fn canonical_json_node(&self, _path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, String> {
        Err(invalid_path())
    }
    fn canonical_json_key(&self, _object_path: &[usize], _index: usize) -> Result<&str, String> {
        Err(invalid_path())
    }
    fn canonical_json_borrowed_root(&self) -> Result<Option<ArtifactCanonicalJsonValue<'_>>, String> {
        Ok(None)
    }
}

fn invalid_path() -> String {
    "canonical-edit.invalid-typed-path".into()
}

fn field_at(fields: &[(&'static str, bool)], index: usize) -> Result<(usize, &'static str), String> {
    fields.iter().enumerate().filter(|(_, (_, present))| *present).nth(index).map(|(ordinal, (name, _))| (ordinal, *name)).ok_or_else(invalid_path)
}

enum CanonicalEditNode<'a, M> {
    Edit(&'a Edit<M>),
    Mutation(&'a M),
    Mutations(&'a [M]),
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
    fn fields(&self) -> [(&'static str, bool); 12] {
        let mut fields = [("", false); 12];
        match self {
            Self::Edit(edit) => {
                fields[..10].copy_from_slice(&[
                    ("id", true),
                    ("actor", edit.actor.is_some()),
                    ("forwards", true),
                    ("inverse", true),
                    ("mutationMeta", !edit.mutation_meta.is_empty()),
                    ("description", edit.description.is_some()),
                    ("coalesceKey", edit.coalesce_key.is_some()),
                    ("sequenceNumber", true),
                    ("startedAt", true),
                    ("finishedAt", edit.finished_at.is_some()),
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
                ]
            }
            Self::Clock(_) => fields[..3].copy_from_slice(&[("actor", true), ("physical_ms", true), ("logical", true)]),
            Self::Origin(origin) => match origin {
                crate::os_spr::MutationOrigin::Owner => fields[0] = ("kind", true),
                crate::os_spr::MutationOrigin::Contributed { .. } => fields[..4].copy_from_slice(&[("kind", true), ("plugin_id", true), ("mutation_id", true), ("payload_hash", true)]),
                crate::os_spr::MutationOrigin::Transaction { .. } => fields[..2].copy_from_slice(&[("kind", true), ("initiator", true)]),
            },
            Self::Target(target) => fields[..3].copy_from_slice(&[("artifactId", true), ("artifactKind", true), ("dialect", target.dialect.is_some())]),
            _ => {}
        }
        fields
    }

    fn child(&self, index: usize) -> Result<Self, String> {
        use ArtifactCanonicalJsonNode as N;
        Ok(match self {
            Self::Edit(edit) => match field_at(&self.fields(), index)?.0 {
                0 => Self::Scalar(N::String(&edit.id)),
                1 => Self::Scalar(N::String(edit.actor.as_deref().ok_or_else(invalid_path)?)),
                2 => Self::Mutations(&edit.forwards),
                3 => Self::Mutations(&edit.inverse),
                4 => Self::Metas(&edit.mutation_meta),
                5 => Self::Scalar(N::String(edit.description.as_deref().ok_or_else(invalid_path)?)),
                6 => Self::Scalar(N::String(edit.coalesce_key.as_deref().ok_or_else(invalid_path)?)),
                7 => Self::Scalar(N::I64(i64::from(edit.sequence_number))),
                8 => Self::Scalar(N::String(&edit.started_at)),
                9 => Self::Scalar(N::String(edit.finished_at.as_deref().ok_or_else(invalid_path)?)),
                _ => return Err(invalid_path()),
            },
            Self::Mutations(values) => Self::Mutation(values.get(index).ok_or_else(invalid_path)?),
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

    fn node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'a>, String> {
        if let Self::Mutation(value) = self {
            return value.canonical_json_node(path);
        }
        if let Some((first, rest)) = path.split_first() {
            return self.child(*first)?.node(rest);
        }
        Ok(match self {
            Self::Mutations(values) => ArtifactCanonicalJsonNode::Array(values.len()),
            Self::Metas(values) => ArtifactCanonicalJsonNode::Array(values.len()),
            Self::Dependencies(values) => ArtifactCanonicalJsonNode::Array(values.len()),
            Self::Hash(_) => ArtifactCanonicalJsonNode::Array(32),
            Self::Scalar(value) => *value,
            _ => ArtifactCanonicalJsonNode::Object(self.fields().iter().filter(|(_, present)| *present).count()),
        })
    }

    fn key(&self, path: &[usize], index: usize) -> Result<&'a str, String> {
        if let Self::Mutation(value) = self {
            return value.canonical_json_key(path, index);
        }
        if let Some((first, rest)) = path.split_first() {
            return self.child(*first)?.key(rest, index);
        }
        field_at(&self.fields(), index).map(|(_, name)| name)
    }

    fn borrowed_value(self) -> ArtifactCanonicalJsonValue<'a> {
        use ArtifactCanonicalJsonValue as V;
        match self {
            Self::Mutation(value) => V::Source(value),
            Self::Mutations(values) => V::Array(ArtifactCanonicalJsonArray::new(values.iter().map(|value| V::Source(value)))),
            Self::Metas(values) => V::Array(ArtifactCanonicalJsonArray::new(values.iter().map(|value| Self::Meta(value).borrowed_value()))),
            Self::Dependencies(values) => V::Array(ArtifactCanonicalJsonArray::new(values.iter().map(|value| V::Scalar(ArtifactCanonicalJsonNode::String(&value.0))))),
            Self::Hash(values) => V::Array(ArtifactCanonicalJsonArray::new(values.iter().map(|value| V::Scalar(ArtifactCanonicalJsonNode::U64(u64::from(*value)))))),
            Self::Scalar(value) => V::Scalar(value),
            node => {
                let length = node.fields().iter().filter(|(_, present)| *present).count();
                V::Object(ArtifactCanonicalJsonObject::new((0..length).map(move |index| {
                    let key = field_at(&node.fields(), index).expect("fixed canonical metadata field").1;
                    let value = node.child(index).expect("fixed canonical metadata child").borrowed_value();
                    (key, value)
                })))
            }
        }
    }
}

impl<M: ArtifactCanonicalJson> ArtifactCanonicalJson for Edit<M> {
    fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, String> {
        CanonicalEditNode::Edit(self).node(path)
    }
    fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<&str, String> {
        CanonicalEditNode::Edit(self).key(path, index)
    }
    fn canonical_json_borrowed_root(&self) -> Result<Option<ArtifactCanonicalJsonValue<'_>>, String> {
        Ok(Some(CanonicalEditNode::Edit(self).borrowed_value()))
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
}

fn canonical_escape(byte: u8, escape: &mut [u8; 6]) -> usize {
    match byte {
        b'"' => {
            escape[..2].copy_from_slice(b"\\\"");
            2
        }
        b'\\' => {
            escape[..2].copy_from_slice(b"\\\\");
            2
        }
        b'\n' => {
            escape[..2].copy_from_slice(b"\\n");
            2
        }
        b'\r' => {
            escape[..2].copy_from_slice(b"\\r");
            2
        }
        b'\t' => {
            escape[..2].copy_from_slice(b"\\t");
            2
        }
        8 => {
            escape[..2].copy_from_slice(b"\\b");
            2
        }
        12 => {
            escape[..2].copy_from_slice(b"\\f");
            2
        }
        0..=31 => {
            escape.copy_from_slice(b"\\u0000");
            escape[4] = b"0123456789abcdef"[(byte >> 4) as usize];
            escape[5] = b"0123456789abcdef"[(byte & 15) as usize];
            6
        }
        _ => {
            escape[0] = byte;
            1
        }
    }
}

struct ScalarBytes {
    bytes: [u8; 64],
    length: usize,
}

impl ScalarBytes {
    /// 🔓️ Every arm but `F32` is now serde-free: `null`/`bool`/plain-decimal integers have a
    /// single unambiguous JSON spelling (no shortest-round-trip question the way floats have), so
    /// they are written directly; `F64` routes through `pack::json::format_f64`, proven
    /// byte-identical to `serde_json`'s own `f64` writer for every value (`.🧬semio/🦑️repo/
    /// 🎫️tickets/🎆️26/🌙️09/☀️01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS/
    /// 🔍️research/📓️float-format-parity.md`). `F32` stays on `serde_json` — that proof covers only
    /// `f64`, and `zmij`'s `f32` path uses a materially different threshold/precision budget this
    /// ticket did not verify.
    fn from_node(node: ArtifactCanonicalJsonNode<'_>) -> Result<Self, String> {
        let mut scalar = Self { bytes: [0; 64], length: 0 };
        use std::io::Write as _;
        match node {
            ArtifactCanonicalJsonNode::Null => scalar.write_all(b"null").map_err(|error| error.to_string()),
            ArtifactCanonicalJsonNode::Bool(value) => scalar.write_all(if value { b"true" } else { b"false" }).map_err(|error| error.to_string()),
            ArtifactCanonicalJsonNode::I64(value) => write!(scalar, "{value}").map_err(|error| error.to_string()),
            ArtifactCanonicalJsonNode::U64(value) => write!(scalar, "{value}").map_err(|error| error.to_string()),
            ArtifactCanonicalJsonNode::I128(value) => write!(scalar, "{value}").map_err(|error| error.to_string()),
            ArtifactCanonicalJsonNode::U128(value) => write!(scalar, "{value}").map_err(|error| error.to_string()),
            ArtifactCanonicalJsonNode::F32(value) => serde_json::to_writer(&mut scalar, &value).map_err(|error| error.to_string()),
            ArtifactCanonicalJsonNode::F64(value) => scalar.write_all(crate::os_pack::json::format_f64(value).as_bytes()).map_err(|error| error.to_string()),
            _ => return Err(invalid_path()),
        }?;
        Ok(scalar)
    }
}

impl std::io::Write for ScalarBytes {
    fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
        let end = self.length.checked_add(bytes.len()).filter(|end| *end <= self.bytes.len()).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "canonical scalar exceeds fixed encoding"))?;
        self.bytes[self.length..end].copy_from_slice(bytes);
        self.length = end;
        Ok(bytes.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
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
    fn push(&mut self, index: usize) -> Result<(), String> {
        if self.depth >= self.maximum_depth {
            return Err("canonical-edit.depth-limit".into());
        }
        self.path[self.depth - 1] = index;
        self.frames[self.depth] = JsonFrame::default();
        self.depth += 1;
        Ok(())
    }

    fn escaped_byte(&mut self, text: &str, frame: usize) -> Option<u8> {
        if self.escape_offset < self.escape_length {
            let byte = self.escape[self.escape_offset];
            self.escape_offset += 1;
            return Some(byte);
        }
        let offset = &mut self.frames[frame].offset;
        let byte = *text.as_bytes().get(*offset)?;
        *offset += 1;
        self.escape_offset = 0;
        self.escape_length = canonical_escape(byte, &mut self.escape);
        self.escape_offset = 1;
        Some(self.escape[0])
    }

    fn scalar_node(&mut self, node: ArtifactCanonicalJsonNode<'_>) -> Result<(), String> {
        self.scalar = ScalarBytes::from_node(node)?;
        Ok(())
    }

    fn next_byte(&mut self, source: &(impl ArtifactCanonicalJson + ?Sized)) -> Result<Option<u8>, String> {
        loop {
            if self.depth == 0 {
                return Ok(None);
            }
            let top = self.depth - 1;
            let frame = self.frames[top];
            if frame.kind == 0 {
                let node = source.canonical_json_node(&self.path[..top])?;
                self.frames[top].kind = match node {
                    ArtifactCanonicalJsonNode::String(_) => 1,
                    ArtifactCanonicalJsonNode::Array(length) => {
                        self.frames[top].length = length;
                        2
                    }
                    ArtifactCanonicalJsonNode::Object(length) => {
                        self.frames[top].length = length;
                        3
                    }
                    value => {
                        self.scalar_node(value)?;
                        4
                    }
                };
                continue;
            }
            match (frame.kind, frame.phase) {
                (1, 0) => {
                    self.frames[top].phase = 1;
                    return Ok(Some(b'"'));
                }
                (1, 1) => {
                    let ArtifactCanonicalJsonNode::String(text) = source.canonical_json_node(&self.path[..top])? else {
                        return Err("canonical-edit.source-shape-changed".into());
                    };
                    if let Some(byte) = self.escaped_byte(text, top) {
                        return Ok(Some(byte));
                    }
                    self.frames[top].phase = 2;
                }
                (1, 2) => {
                    self.depth -= 1;
                    return Ok(Some(b'"'));
                }
                (2, 0) => {
                    self.frames[top].phase = 1;
                    return Ok(Some(b'['));
                }
                (2, 1) if frame.index == frame.length => {
                    self.depth -= 1;
                    return Ok(Some(b']'));
                }
                (2, 1) => {
                    self.frames[top].phase = 2;
                    self.push(frame.index)?;
                }
                (2, 2) => {
                    self.frames[top].index += 1;
                    self.frames[top].phase = 1;
                    if self.frames[top].index < frame.length {
                        return Ok(Some(b','));
                    }
                }
                (3, 0) => {
                    self.frames[top].phase = 1;
                    return Ok(Some(b'{'));
                }
                (3, 1) if frame.index == frame.length => {
                    self.depth -= 1;
                    return Ok(Some(b'}'));
                }
                (3, 1) => {
                    self.frames[top].phase = 2;
                    self.frames[top].offset = 0;
                    return Ok(Some(b'"'));
                }
                (3, 2) => {
                    let text = source.canonical_json_key(&self.path[..top], frame.index)?;
                    if let Some(byte) = self.escaped_byte(text, top) {
                        return Ok(Some(byte));
                    }
                    self.frames[top].phase = 3;
                }
                (3, 3) => {
                    self.frames[top].phase = 4;
                    return Ok(Some(b'"'));
                }
                (3, 4) => {
                    self.frames[top].phase = 5;
                    return Ok(Some(b':'));
                }
                (3, 5) => {
                    self.frames[top].phase = 6;
                    self.push(frame.index)?;
                }
                (3, 6) => {
                    self.frames[top].index += 1;
                    self.frames[top].phase = 1;
                    if self.frames[top].index < frame.length {
                        return Ok(Some(b','));
                    }
                }
                (4, _) => {
                    if frame.offset == self.scalar.length {
                        self.depth -= 1;
                        continue;
                    }
                    let byte = self.scalar.bytes[frame.offset];
                    self.frames[top].offset += 1;
                    return Ok(Some(byte));
                }
                _ => return Err("canonical-edit.encoder-state".into()),
            }
        }
    }

    pub fn encode_chunk(&mut self, source: &(impl ArtifactCanonicalJson + ?Sized), output: &mut [u8]) -> Result<usize, ArtifactCanonicalJsonEncodeError> {
        let mut written = 0;
        while written < output.len().min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES) {
            let Some(byte) = self.next_byte(source).map_err(|reason| ArtifactCanonicalJsonEncodeError { written_bytes: written, reason })? else { break };
            output[written] = byte;
            written += 1;
        }
        Ok(written)
    }

    pub fn is_complete(&self) -> bool {
        self.depth == 0
    }
}
//#endregion 🔣️ByteEncoder

//#region 🔏️Sealing
pub(super) struct ArtifactStoreOneItemAuthorityRetirement {
    authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    strings: [Option<String>; 2],
    active: Option<ArtifactStoreStringRetirement>,
}

impl ArtifactStoreOneItemAuthorityRetirement {
    pub(super) fn new(authority: Arc<ArtifactStoreOneItemLiveAuthority>) -> Self {
        Self { authority: Some(authority), strings: Default::default(), active: None }
    }
}

impl ErasedSnapshotRetirement for ArtifactStoreOneItemAuthorityRetirement {
    fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
        if items == 0 || bytes == 0 {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        if let Some(active) = self.active.as_mut() {
            return match active.close_step(1, bytes)? {
                SnapshotRetirementStep::Complete => {
                    assert!(active.terminal_is_empty());
                    self.active = None;
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
                step => Ok(step),
            };
        }
        if let Some(value) = self.strings.iter_mut().find_map(Option::take) {
            self.active = Some(ArtifactStoreStringRetirement::new(value));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.take() {
            if let Some(authority) = Arc::into_inner(authority) {
                self.strings = [Some(authority.actor), authority.group_id];
            }
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
    }
    fn terminal_is_empty(&self) -> bool {
        self.authority.is_none() && self.strings.iter().all(Option::is_none) && self.active.is_none()
    }
}

impl Drop for ArtifactStoreOneItemAuthorityRetirement {
    fn drop(&mut self) {
        assert!(self.terminal_is_empty(), "Store live authority dropped before bounded string retirement completed");
    }
}

/// 📍️ Portable replay witness. Restoration re-executes each prior byte and verifies this prefix;
/// no supplied hash state or digest can directly create publication authority.
/// @emoji 🔮️ serde stays TEST-ONLY: this file's own round-trip test (`serde_json::to_vec`/
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
    encoder: ArtifactCanonicalEditEncoder,
    authority: Option<Arc<ArtifactStoreOneItemLiveAuthority>>,
    edit: Option<Box<Edit<M>>>,
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
    identities: [Vec<u8>; 3],
    identity_index: usize,
    cancelled: bool,
    closing: bool,
}

impl<P, M> ArtifactStoreOneItemSealer<P, M> {
    pub(super) fn new(authority: Arc<ArtifactStoreOneItemLiveAuthority>, edit: Edit<M>, post: Arc<P>, mutation_retirement: Arc<dyn ArtifactOwnedValueRetirementFactory<M>>, snapshot_retirement: Arc<dyn SnapshotRetirementFactory<P>>) -> Self {
        Self {
            authority: Some(authority),
            edit: Some(Box::new(edit)),
            post: Some(post),
            prepared: None,
            encoder: ArtifactCanonicalEditEncoder::default(),
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
            identities: std::array::from_fn(|_| Vec::with_capacity(ARTIFACT_STORE_ONE_ITEM_ID_BYTES)),
            identity_index: 0,
            cancelled: false,
            closing: false,
        }
    }

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
    }
    pub fn canonical_chunk(&self) -> &[u8] {
        if self.last_canonical {
            &self.last_chunk[..self.last_length]
        } else {
            &[]
        }
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
        ArtifactStoreOneItemCheckpoint { cursor: self.turns, completed_items: self.turns, completed_bytes: self.completed_bytes, digest: self.transcript.clone().finalize() }
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
                &[u8::from(authority.group_id.is_some())],
                authority.group_id.as_deref().unwrap_or("").as_bytes(),
            ],
        )
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.closing
            && self.authority.is_none()
            && self.edit.is_none()
            && self.post.is_none()
            && self.prepared.is_none()
            && self.active_retirement.is_none()
            && self.retirement_strings.iter().all(Option::is_none)
            && self.mutation_retirement.is_none()
            && self.snapshot_retirement.is_none()
            && self.identities.iter().all(Vec::is_empty)
            && self.encoder.terminal_is_empty()
    }
}

impl<P: Send + Sync + 'static, M: ArtifactCanonicalJson + Send + 'static> ArtifactStoreOneItemSealer<P, M> {
    pub fn advance(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<ArtifactStoreOneItemPreparationStep, String> {
        self.last_length = 0;
        self.last_canonical = false;
        if !grant.permits_one() || self.cancelled || self.closing {
            return Ok(ArtifactStoreOneItemPreparationStep::Blocked);
        }
        self.verify_replay()?;
        if self.phase == 6 {
            return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.progress()));
        }
        let mut maximum = grant.maximum_bytes.min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES);
        let mut encoding_error = None;
        if let Some(target) = self.replay {
            if target.completed_bytes > self.completed_bytes {
                maximum = maximum.min((target.completed_bytes - self.completed_bytes) as usize);
            }
        }
        match self.phase {
            0 => {
                self.authority.as_ref().ok_or_else(|| "canonical-edit.authority-missing".to_string())?.validate_semantic_edit(self.edit.as_ref().ok_or_else(|| "canonical-edit.owner-missing".to_string())?)?;
                self.phase = 1;
            }
            1 | 3 => {
                let edit = self.edit.as_ref().ok_or_else(|| "canonical-edit.owner-missing".to_string())?;
                self.last_length = match self.encoder.encode_chunk(edit.as_ref(), &mut self.last_chunk[..maximum]) {
                    Ok(written) => written,
                    Err(error) => {
                        self.cancelled = true;
                        encoding_error = Some(error.reason);
                        error.written_bytes
                    }
                };
                self.last_canonical = true;
                if self.phase == 1 {
                    self.canonical_bytes = self.canonical_bytes.checked_add(self.last_length as u64).filter(|bytes| *bytes <= CANONICAL_EDIT_MAXIMUM_BYTES).ok_or_else(|| "canonical-edit.byte-limit".to_string())?;
                } else {
                    self.hash.update(&self.last_chunk[..self.last_length]);
                    self.hashed_canonical_bytes += self.last_length as u64;
                }
                if self.encoder.is_complete() {
                    if self.phase == 1 {
                        self.phase = 2;
                        self.encoder.reset()?;
                    } else {
                        self.phase = 4;
                    }
                }
            }
            2 => {
                while self.last_length < maximum {
                    let Some(byte) = self.header_byte(self.header_offset) else {
                        self.phase = 3;
                        break;
                    };
                    self.last_chunk[self.last_length] = byte;
                    self.last_length += 1;
                    self.header_offset += 1;
                }
                self.hash.update(&self.last_chunk[..self.last_length]);
            }
            4 => {
                if self.hashed_canonical_bytes != self.canonical_bytes {
                    return Err("canonical-edit.source-length-changed".into());
                }
                let authority = self.authority.as_ref().expect("sealer retains authority");
                let edit = self.edit.as_ref().expect("sealer retains edit");
                while self.last_length < maximum && self.identity_index < 3 {
                    let source = if self.identity_index == 0 { authority.actor.as_bytes() } else { edit.id.as_bytes() };
                    let target = &mut self.identities[self.identity_index];
                    if target.len() == source.len() {
                        self.identity_index += 1;
                        continue;
                    }
                    if target.len() == ARTIFACT_STORE_ONE_ITEM_ID_BYTES {
                        return Err("canonical-edit.identity-limit".into());
                    }
                    let byte = source[target.len()];
                    target.push(byte);
                    self.last_chunk[self.last_length] = byte;
                    self.last_length += 1;
                }
                if self.identity_index == 3 {
                    self.phase = 5;
                }
            }
            5 => {
                self.phase = 6;
            }
            _ => return Err("canonical-edit.sealer-state".into()),
        }
        self.transcript.update(&self.last_chunk[..self.last_length]);
        self.completed_bytes = self.completed_bytes.checked_add(self.last_length as u64).ok_or_else(|| "canonical-edit.work-overflow".to_string())?;
        self.turns = self.turns.checked_add(1).ok_or_else(|| "canonical-edit.turn-overflow".to_string())?;
        if let Some(error) = encoding_error {
            return Err(error);
        }
        self.verify_replay()?;
        if self.phase == 6 {
            let authority = self.authority.as_ref().ok_or_else(|| "canonical-edit.authority-missing".to_string())?;
            let edit = self.edit.take().ok_or_else(|| "canonical-edit.owner-missing".to_string())?;
            let post = self.post.take().ok_or_else(|| "canonical-edit.post-owner-missing".to_string())?;
            let identities = std::mem::take(&mut self.identities).map(|bytes| unsafe { String::from_utf8_unchecked(bytes) });
            self.prepared = Some(authority.seal_prepared_owned(edit, post, self.hash.clone().finalize(), identities));
            return Ok(ArtifactStoreOneItemPreparationStep::Prepared(self.progress()));
        }
        Ok(ArtifactStoreOneItemPreparationStep::Progress(self.progress()))
    }

    pub fn close_step(&mut self, grant: ArtifactStoreOneItemGrant) -> Result<SnapshotRetirementStep, String> {
        if !self.closing || !grant.permits_one() {
            return Ok(SnapshotRetirementStep::Blocked);
        }
        if !self.encoder.terminal_is_empty() {
            return self.encoder.close_step();
        }
        if let Some(active) = self.active_retirement.as_mut() {
            return match active.close_step(grant.maximum_items.min(1), grant.maximum_bytes)? {
                SnapshotRetirementStep::Pending { released_items, released_bytes } if released_items <= 1 && released_bytes <= grant.maximum_bytes => Ok(SnapshotRetirementStep::Pending { released_items, released_bytes }),
                SnapshotRetirementStep::Pending { .. } => Err("canonical-edit.retirement-grant".into()),
                SnapshotRetirementStep::Blocked => Ok(SnapshotRetirementStep::Blocked),
                SnapshotRetirementStep::Complete => {
                    if !active.terminal_is_empty() {
                        return Err("canonical-edit.retirement-witness".into());
                    }
                    self.active_retirement = None;
                    Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 })
                }
            };
        }
        if let Some(bytes) = self.identities.iter_mut().find(|bytes| !bytes.is_empty()) {
            let released_bytes = bytes.len().min(grant.maximum_bytes);
            bytes.truncate(bytes.len() - released_bytes);
            return Ok(SnapshotRetirementStep::Pending { released_items: 0, released_bytes });
        }
        if let Some(prepared) = self.prepared.take() {
            self.edit = Some(prepared.edit);
            self.post = Some(prepared.post_snapshot);
            self.retirement_strings = [prepared.local_actor, Some(prepared.applied_edit_id), Some(prepared.tail_edit_id)];
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(edit) = self.edit.take() {
            self.active_retirement = Some(Box::new(ArtifactStoreDecodedEditRetirement::new(*edit, Arc::clone(self.mutation_retirement.as_ref().expect("sealer retains mutation retirement authority")))));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(post) = self.post.take() {
            self.active_retirement = Some(self.snapshot_retirement.as_ref().expect("sealer retains snapshot retirement authority").retire(post));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(value) = self.retirement_strings.iter_mut().find_map(Option::take) {
            self.active_retirement = Some(Box::new(ArtifactStoreStringRetirement::new(value)));
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if let Some(authority) = self.authority.take() {
            self.active_retirement = Some(authority.retire());
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        if self.mutation_retirement.take().is_some() || self.snapshot_retirement.take().is_some() {
            return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(SnapshotRetirementStep::Complete)
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
mod tests {
    use super::*;

    /// 🔬️ `ScalarBytes::from_node`'s serde-free arms (`Null`/`Bool`/`I64`/`U64`/`I128`/`U128`/
    /// `F64`), byte-for-byte against `serde_json` — the direct proof this ticket's own
    /// `float-format-parity.md` calls for on the second real call site (`F32` intentionally
    /// excluded, still routed through `serde_json`, see `from_node`'s own docstring). The `f64`
    /// half reuses the same LCG this crate's other property tests use, not a new dependency.
    #[test]
    fn scalar_bytes_from_node_matches_serde_json_byte_for_byte() {
        fn bytes_of(node: ArtifactCanonicalJsonNode<'_>) -> Vec<u8> {
            let scalar = ScalarBytes::from_node(node).unwrap();
            scalar.bytes[..scalar.length].to_vec()
        }
        assert_eq!(bytes_of(ArtifactCanonicalJsonNode::Null), serde_json::to_vec(&()).unwrap());
        for value in [true, false] {
            assert_eq!(bytes_of(ArtifactCanonicalJsonNode::Bool(value)), serde_json::to_vec(&value).unwrap());
        }
        for value in [0i64, -1, 1, i64::MIN, i64::MAX, -17] {
            assert_eq!(bytes_of(ArtifactCanonicalJsonNode::I64(value)), serde_json::to_vec(&value).unwrap());
        }
        for value in [0u64, 1, u64::MAX] {
            assert_eq!(bytes_of(ArtifactCanonicalJsonNode::U64(value)), serde_json::to_vec(&value).unwrap());
        }
        for value in [0i128, -1, i128::MIN, i128::MAX] {
            assert_eq!(bytes_of(ArtifactCanonicalJsonNode::I128(value)), serde_json::to_vec(&value).unwrap());
        }
        for value in [0u128, u128::MAX] {
            assert_eq!(bytes_of(ArtifactCanonicalJsonNode::U128(value)), serde_json::to_vec(&value).unwrap());
        }
        let mut state: u64 = 0xC0DE_CAFE_1234_5678;
        let mut next_u64 = || {
            state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
            let mut z = state;
            z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
            z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
            z ^ (z >> 31)
        };
        let mut checked = 0usize;
        for value in [0.0, -0.0, 1.0, -1.0, 0.1, 1e21, 1e-7, f64::MIN_POSITIVE, f64::MAX] {
            assert_eq!(bytes_of(ArtifactCanonicalJsonNode::F64(value)), serde_json::to_vec(&value).unwrap(), "mismatch for {value:e}");
            checked += 1;
        }
        for _ in 0..50_000u32 {
            let value = f64::from_bits(next_u64());
            if !value.is_finite() {
                continue;
            }
            assert_eq!(bytes_of(ArtifactCanonicalJsonNode::F64(value)), serde_json::to_vec(&value).unwrap(), "mismatch for {value:e}");
            checked += 1;
        }
        eprintln!("[DEBUG] [canonical-edit] {checked} ScalarBytes f64 values matched serde_json byte-for-byte");
    }

    #[derive(Clone, Debug, Serialize, ToValue, Deserialize, FromValue)]
    enum FixtureMutation {
        Replace { text: String, nested: Vec<String>, enabled: bool, amount: i64 },
    }

    impl ArtifactCanonicalJson for FixtureMutation {
        fn canonical_json_node(&self, path: &[usize]) -> Result<ArtifactCanonicalJsonNode<'_>, String> {
            use ArtifactCanonicalJsonNode as N;
            let Self::Replace { text, nested, enabled, amount } = self;
            Ok(match path {
                [] => N::Object(1),
                [0] => N::Object(4),
                [0, 0] => N::String(text),
                [0, 1] => N::Array(nested.len()),
                [0, 1, index] => N::String(nested.get(*index).ok_or_else(invalid_path)?),
                [0, 2] => N::Bool(*enabled),
                [0, 3] => N::I64(*amount),
                _ => return Err(invalid_path()),
            })
        }
        fn canonical_json_key(&self, path: &[usize], index: usize) -> Result<&str, String> {
            match path {
                [] if index == 0 => Ok("Replace"),
                [0] => ["text", "nested", "enabled", "amount"].get(index).copied().ok_or_else(invalid_path),
                _ => Err(invalid_path()),
            }
        }
    }

    fn fixture() -> (Edit<FixtureMutation>, serde_json::Value) {
        let value: serde_json::Value = serde_json::from_str(include_str!("🧪️fixtures/🔣️canonical-edit-sealer.json")).unwrap();
        (serde_json::from_value(value["edit"].clone()).unwrap(), value)
    }

    #[test]
    fn canonical_edit_large_unicode_bytes_match_serde_and_language_neutral_oracle() {
        let (edit, fixture) = fixture();
        let oracle = serde_json::to_vec(&edit).unwrap();
        assert_eq!(oracle, fixture["expectedJson"].as_str().unwrap().as_bytes());
        for maximum in [1, 2, 7, 256, 4096] {
            let mut encoder = ArtifactCanonicalJsonCursor::default();
            let mut actual = Vec::new();
            let mut output = vec![0; maximum];
            assert_eq!(encoder.encode_chunk(&edit, &mut []).unwrap(), 0);
            while !encoder.is_complete() {
                let count = encoder.encode_chunk(&edit, &mut output).unwrap();
                assert!(count <= maximum.min(ARTIFACT_CANONICAL_JSON_CHUNK_BYTES));
                actual.extend_from_slice(&output[..count]);
            }
            assert_eq!(actual, oracle, "grant {maximum}");
        }
        let digest = CursorRevisionAccumulator::edit_digest(&edit);
        let hex: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
        assert_eq!(hex, fixture["expectedDigest"].as_str().unwrap());
    }

    struct FixtureRetirement {
        text: Option<String>,
        nested: Vec<String>,
        active: Option<ArtifactStoreStringRetirement>,
    }

    impl ErasedSnapshotRetirement for FixtureRetirement {
        fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
            if items == 0 || bytes == 0 {
                return Ok(SnapshotRetirementStep::Blocked);
            }
            if let Some(active) = self.active.as_mut() {
                let step = active.close_step(items, bytes)?;
                if matches!(step, SnapshotRetirementStep::Complete) {
                    assert!(active.terminal_is_empty());
                    self.active = None;
                }
                return Ok(if matches!(step, SnapshotRetirementStep::Complete) { SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 } } else { step });
            }
            if let Some(text) = self.text.take().or_else(|| self.nested.pop()) {
                self.active = Some(ArtifactStoreStringRetirement::new(text));
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            Ok(SnapshotRetirementStep::Complete)
        }
        fn terminal_is_empty(&self) -> bool {
            self.text.is_none() && self.nested.is_empty() && self.active.is_none()
        }
    }

    struct FixtureMutationRetirement;
    impl ArtifactOwnedValueRetirementFactory<FixtureMutation> for FixtureMutationRetirement {
        fn retire_owned(&self, value: FixtureMutation) -> Box<dyn ErasedSnapshotRetirement> {
            let FixtureMutation::Replace { text, nested, .. } = value;
            Box::new(FixtureRetirement { text: Some(text), nested, active: None })
        }
    }

    pub(super) struct FixtureSnapshotRetirement;
    struct FixtureRootRetirement(Option<Arc<u64>>);
    impl SnapshotRetirementFactory<u64> for FixtureSnapshotRetirement {
        fn retire(&self, snapshot: Arc<u64>) -> Box<dyn ErasedSnapshotRetirement> {
            Box::new(FixtureRootRetirement(Some(snapshot)))
        }
    }
    impl ErasedSnapshotRetirement for FixtureRootRetirement {
        fn close_step(&mut self, items: usize, bytes: usize) -> Result<SnapshotRetirementStep, String> {
            if items == 0 || bytes == 0 {
                return Ok(SnapshotRetirementStep::Blocked);
            }
            if self.0.take().is_some() {
                return Ok(SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
            }
            Ok(SnapshotRetirementStep::Complete)
        }
        fn terminal_is_empty(&self) -> bool {
            self.0.is_none()
        }
    }

    pub(super) fn authority() -> Arc<ArtifactStoreOneItemLiveAuthority> {
        Arc::new(ArtifactStoreOneItemLiveAuthority {
            operation: semio_framework_job::OperationId(11),
            generation: semio_framework_job::Generation(7),
            base_revision: [9; 32],
            base_applied_edit_count: 3,
            next_sequence_number: 4,
            next_clock: HybridLogicalTimestamp { actor: 1, physical_ms: 42, logical: 2 },
            actor: "actor-1".into(),
            group_id: Some("group-1".into()),
        })
    }

    fn sealer(authority: &Arc<ArtifactStoreOneItemLiveAuthority>) -> ArtifactStoreOneItemSealer<u64, FixtureMutation> {
        authority.begin_one_item_seal(fixture().0, Arc::new(17), Arc::new(FixtureMutationRetirement), Arc::new(FixtureSnapshotRetirement))
    }

    fn close(sealer: &mut ArtifactStoreOneItemSealer<u64, FixtureMutation>, bytes: usize) {
        sealer.begin_close();
        assert!(matches!(sealer.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 0 }).unwrap(), SnapshotRetirementStep::Blocked));
        for _ in 0..100_000 {
            match sealer.close_step(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).unwrap() {
                SnapshotRetirementStep::Complete => {
                    assert!(sealer.terminal_is_empty());
                    return;
                }
                SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1 && released_bytes <= bytes);
                }
                SnapshotRetirementStep::Blocked => panic!("positive grant failed to retire retained owners"),
            }
        }
        panic!("bounded retirement did not terminate");
    }

    fn finish(sealer: &mut ArtifactStoreOneItemSealer<u64, FixtureMutation>, bytes: usize) -> [u8; 32] {
        let mut previous = sealer.completed_bytes;
        for _ in 0..100_000 {
            let step = sealer.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).unwrap();
            assert!(sealer.completed_bytes - previous <= bytes as u64);
            previous = sealer.completed_bytes;
            if matches!(step, ArtifactStoreOneItemPreparationStep::Prepared(_)) {
                return sealer.prepared().unwrap().edit_digest();
            }
        }
        panic!("positive byte grant failed to seal");
    }

    #[test]
    fn canonical_sealer_tiny_grants_replay_and_cross_worker_transfer_preserve_exact_digest() {
        let oracle = CursorRevisionAccumulator::edit_digest(&fixture().0);
        for bytes in [1, 2, 7, 256, 4096] {
            let authority = authority();
            let mut owner = sealer(&authority);
            assert!(matches!(owner.advance(ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: bytes }).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked));
            owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).unwrap();
            for _ in 0..19 {
                owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: bytes }).unwrap();
            }
            let checkpoint: ArtifactStoreOneItemSealCheckpoint = serde_json::from_slice(&serde_json::to_vec(&owner.checkpoint()).unwrap()).unwrap();
            let mut replay = sealer(&authority);
            replay.restore_checkpoint(checkpoint).unwrap();
            assert_eq!(finish(&mut replay, 7), oracle);
            authority.validate_prepared(replay.prepared().unwrap()).unwrap();
            close(&mut replay, 1);
            let mut moved = std::thread::spawn(move || {
                assert_eq!(finish(&mut owner, bytes), oracle);
                owner
            })
            .join()
            .unwrap();
            assert_eq!(moved.completed_bytes, 2 * moved.canonical_bytes + moved.header_offset as u64 + 7 + 2 * "edit-✓".len() as u64);
            authority.validate_prepared(moved.prepared().unwrap()).unwrap();
            close(&mut moved, 1);
        }
    }

    #[test]
    fn canonical_sealer_rejects_stale_checkpoint_forged_prefix_and_rebound_owners() {
        let authority = authority();
        let mut owner = sealer(&authority);
        for _ in 0..4 {
            owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 7 }).unwrap();
        }
        let checkpoint = owner.checkpoint();
        for hostile in 0..5 {
            let mut altered = checkpoint;
            match hostile {
                0 => altered.operation += 1,
                1 => altered.generation += 1,
                2 => altered.base_revision[0] ^= 1,
                3 => altered.authority_digest[0] ^= 1,
                _ => altered.version += 1,
            }
            let mut replay = sealer(&authority);
            assert!(replay.restore_checkpoint(altered).is_err());
            close(&mut replay, 7);
        }
        let mut altered = checkpoint;
        altered.prefix_digest[0] ^= 1;
        let mut replay = sealer(&authority);
        replay.restore_checkpoint(altered).unwrap();
        let mut rejected = false;
        for _ in 0..100 {
            if replay.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 1 }).is_err() {
                rejected = true;
                break;
            }
        }
        assert!(rejected && replay.prepared().is_none());
        close(&mut replay, 1);
        finish(&mut owner, 4096);
        let prepared = owner.prepared.as_mut().unwrap();
        prepared.edit_digest[0] ^= 1;
        assert!(authority.validate_prepared(prepared).is_err());
        prepared.edit_digest[0] ^= 1;
        let replacement = Box::new(prepared.edit.as_ref().clone());
        let original = std::mem::replace(&mut prepared.edit, replacement);
        assert!(authority.validate_prepared(prepared).is_err());
        prepared.edit = original;
        let original = std::mem::replace(&mut prepared.post_snapshot, Arc::new(17));
        assert!(authority.validate_prepared(prepared).is_err());
        prepared.post_snapshot = original;
        assert!(super::tests::authority().validate_prepared(prepared).is_err());
        authority.validate_prepared(prepared).unwrap();
        close(&mut owner, 1);
    }

    #[test]
    fn canonical_sealer_cancellation_at_every_phase_retires_exact_owners_and_allows_retry() {
        let authority = authority();
        for phase in 0..=6 {
            let mut owner = sealer(&authority);
            while owner.phase < phase {
                owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 }).unwrap();
            }
            owner.cancel();
            let checkpoint = owner.checkpoint();
            assert!(matches!(owner.advance(ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 4096 }).unwrap(), ArtifactStoreOneItemPreparationStep::Blocked));
            assert_eq!(owner.checkpoint(), checkpoint);
            close(&mut owner, 1);
        }
        let mut retry = sealer(&authority);
        assert_eq!(finish(&mut retry, 1), CursorRevisionAccumulator::edit_digest(&fixture().0));
        close(&mut retry, 1);
    }

    #[test]
    fn canonical_sealer_checkpoint_maximum_accepts_exact_framing_and_identity_overhead_only() {
        let authority = authority();
        let mut owner = sealer(&authority);
        let mut checkpoint = owner.checkpoint();
        checkpoint.phase = 6;
        checkpoint.canonical_bytes = CANONICAL_EDIT_MAXIMUM_BYTES;
        checkpoint.completed_bytes = 2 * CANONICAL_EDIT_MAXIMUM_BYTES + CANONICAL_EDIT_MAXIMUM_OVERHEAD_BYTES;
        assert!(CANONICAL_EDIT_MAXIMUM_OVERHEAD_BYTES > 1_024);
        owner.restore_checkpoint(checkpoint).unwrap();
        checkpoint.completed_bytes += 1;
        assert!(owner.restore_checkpoint(checkpoint).is_err());
        close(&mut owner, 1);
    }

    #[test]
    fn canonical_sealer_preserves_large_domains_and_all_wire_metadata_origins() {
        let (base, fixture) = fixture();
        for length in fixture["largeTextBytes"].as_array().unwrap() {
            for origin in fixture["origins"].as_array().unwrap() {
                let mut edit = base.clone();
                let FixtureMutation::Replace { text, .. } = &mut edit.forwards[0];
                *text = "x".repeat(length.as_u64().unwrap() as usize);
                edit.coalesce_key = Some("coalesce-🧵".into());
                edit.finished_at = Some("finished".into());
                edit.mutation_meta[0].origin = serde_json::from_value(origin.clone()).unwrap();
                edit.mutation_meta[0].payload_hash = Some(crate::os_spr::PayloadHash([23; 32]));
                edit.mutation_meta[0].semantic_kind = Some(SchemaId("fixture#replace".into()));
                let expected = serde_json::to_vec(&edit).unwrap();
                let mut actual = Vec::new();
                let mut encoder = ArtifactCanonicalJsonCursor::default();
                let mut chunk = [0; 256];
                while !encoder.is_complete() {
                    let count = encoder.encode_chunk(&edit, &mut chunk).unwrap();
                    actual.extend_from_slice(&chunk[..count]);
                }
                assert_eq!(actual, expected);
                let digest = CursorRevisionAccumulator::edit_digest(&edit);
                let authority = authority();
                let mut owner = authority.begin_one_item_seal(edit, Arc::new(17), Arc::new(FixtureMutationRetirement), Arc::new(FixtureSnapshotRetirement));
                assert_eq!(finish(&mut owner, 4096), digest);
                close(&mut owner, 4096);
            }
        }
    }

    #[test]
    fn canonical_authority_final_unicode_strings_retire_under_single_byte_grants() {
        let mut authority = authority();
        Arc::get_mut(&mut authority).unwrap().actor = "actor-🧵".into();
        Arc::get_mut(&mut authority).unwrap().group_id = Some("group-✓".into());
        let mut retirement = authority.retire();
        let mut released = 0;
        assert!(matches!(retirement.close_step(1, 0).unwrap(), SnapshotRetirementStep::Blocked));
        for _ in 0..100 {
            match retirement.close_step(1, 1).unwrap() {
                SnapshotRetirementStep::Pending { released_items, released_bytes } => {
                    assert!(released_items <= 1 && released_bytes <= 1);
                    released += released_bytes;
                }
                SnapshotRetirementStep::Complete => {
                    assert!(retirement.terminal_is_empty());
                    assert_eq!(released, "actor-🧵".len() + "group-✓".len());
                    return;
                }
                SnapshotRetirementStep::Blocked => panic!("positive retirement grant blocked"),
            }
        }
        panic!("final authority strings did not retire");
    }
}
//#endregion 🧪️CanonicalLaws
