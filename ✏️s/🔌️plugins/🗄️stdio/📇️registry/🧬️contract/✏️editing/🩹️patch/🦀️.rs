//! 🩹️ Path-scoped native snapshot edits: ONE RFC 6901 pointer operation per mutation, its exact inverse, and the input
//! schema that types its value by the snapshot sub-schema the pointer addresses, so a history edit renders the control the
//! snapshot schema declares for that field (design §20.3 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).
//!
//! @see ../🟦️.ts — the TypeScript twin of preparation, application, inversion and location.
//! @see ../../../🧬️schema/🔣️.json — `$defs/SnapshotPatch`, the wire schema of [`SnapshotPatch`].

use super::{decode_pointer, snapshot_from_edit_source, validate_value, values_equivalent, SnapshotEditError, SnapshotEditEvent};
use crate::{kernel, value_derive};
use semio_framework_value::DslValue;
use semio_framework_value::FromValue;
use kernel::OpBinary;
use semio_framework_value::ToValue;
use semio_framework_value::ValueEdit;
use semio_framework_value::ValueShape;
use semio_framework_ui_locale::LocalizedLabel;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

pub const SNAPSHOT_PATCH_MAX_BYTES: usize = 1_048_576;
pub const SNAPSHOT_PATCH_MAX_SEGMENTS: usize = 128;
/// 🧩️ The most parts one exact inverse may take (`x-semio-inverse-rows.bounded` of every `patch-snapshot` leaf): a value from a
/// 16 MiB source projects to at most ~64 MiB of canonical JSON (octets as decimal arrays), i.e. ~64 parts of the patch budget.
pub const SNAPSHOT_PATCH_MAX_INVERSE_PARTS: usize = 128;
const SNAPSHOT_PATCH_MAX_INDEX: u64 = 9_007_199_254_740_991;
const SNAPSHOT_SCHEMA_REF_HOPS: usize = 32;

fn is_false(value: &bool) -> bool {
    !*value
}

//#region 🩹️Patch
/// 🩹️ One path-scoped edit of a decoded snapshot. `path` (and `from` of a move) is an RFC 6901 pointer; `index` positions an
/// object member (only an exact inverse restoring an object member's place carries it); `key` is a rename's new member name.
/// A `splice` replaces `remove` units at `offset` of the container at `path` with the units of `value` (array items, octets,
/// UTF-8 bytes of a string, object members by position); `continued` marks a non-final part of one exact multi-part inverse,
/// whose whole-snapshot invariant the run's final part checks.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
pub enum SnapshotPatch {
    Set {
        path: String,
        value: DslValue,
    },
    Insert {
        path: String,
        value: DslValue,
        #[value(skip_serializing_if = "Option::is_none")]
        index: Option<u64>,
    },
    Remove {
        path: String,
    },
    Move {
        from: String,
        path: String,
        #[value(skip_serializing_if = "Option::is_none")]
        index: Option<u64>,
    },
    Rename {
        path: String,
        key: String,
    },
    Splice {
        path: String,
        offset: u64,
        remove: u64,
        value: DslValue,
        #[value(default, skip_serializing_if = "is_false")]
        continued: bool,
    },
}

impl SnapshotPatch {
    /// 📍️ The pointer this operation writes: a move's destination, every other operation's own path.
    pub fn path(&self) -> &str {
        match self {
            Self::Set { path, .. } | Self::Insert { path, .. } | Self::Remove { path } | Self::Move { path, .. } | Self::Rename { path, .. } | Self::Splice { path, .. } => path,
        }
    }

    /// ⏭️ Whether this operation is a non-final part of one exact multi-part inverse.
    pub fn continued(&self) -> bool {
        matches!(self, Self::Splice { continued: true, .. })
    }

    /// 🎯️ The decoded segments of [`Self::path`] — the conflict target the envelope carries (empty: the whole document).
    pub fn target(&self) -> Vec<String> {
        decode_pointer(self.path()).unwrap_or_default()
    }

    /// 🏷️ The wire name of this operation (`set`, `insert`, `remove`, `move`, `rename`).
    pub fn operation(&self) -> &'static str {
        match self {
            Self::Set { .. } => "set",
            Self::Insert { .. } => "insert",
            Self::Remove { .. } => "remove",
            Self::Move { .. } => "move",
            Self::Rename { .. } => "rename",
            Self::Splice { .. } => "splice",
        }
    }
}

/// 🏷️ The history label of one patch in every shell locale, naming the operation and the pointer it addresses.
pub fn snapshot_patch_label(patch: &SnapshotPatch) -> LocalizedLabel {
    let shown = |path: &str| if path.is_empty() { "/".to_string() } else { path.to_string() };
    match patch {
        SnapshotPatch::Set { path, .. } => LocalizedLabel::native(&format!("Set {}", shown(path)), &format!("{} setzen", shown(path))),
        SnapshotPatch::Insert { path, .. } => LocalizedLabel::native(&format!("Insert {}", shown(path)), &format!("{} einfügen", shown(path))),
        SnapshotPatch::Remove { path } => LocalizedLabel::native(&format!("Remove {}", shown(path)), &format!("{} entfernen", shown(path))),
        SnapshotPatch::Move { from, path, .. } => LocalizedLabel::native(&format!("Move {} to {}", shown(from), shown(path)), &format!("{} nach {} verschieben", shown(from), shown(path))),
        SnapshotPatch::Rename { path, key } => LocalizedLabel::native(&format!("Rename {} to {key}", shown(path)), &format!("{} in {key} umbenennen", shown(path))),
        SnapshotPatch::Splice { path, offset, .. } => LocalizedLabel::native(&format!("Replace part of {} at {offset}", shown(path)), &format!("Teil von {} ab {offset} ersetzen", shown(path))),
    }
}

impl semio_framework_dsl_record::DslField for SnapshotPatch {
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Value
    }

    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Value(<Self as ToValue>::to_value(self))
    }

    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        let semio_framework_dsl_record::FieldValue::Value(value) = value else { return Err("snapshot patch requires a structured value".into()) };
        let patch = <Self as FromValue>::from_value(value.clone()).map_err(|error| error.to_string())?;
        validate_patch(&patch).map_err(|error| error.to_string())?;
        Ok(patch)
    }
}

impl kernel::OpText for SnapshotPatch {
    fn print_op(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }

    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if line.len() > SNAPSHOT_PATCH_MAX_BYTES {
            return Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::WorkLimit, "snapshot patch exceeds the native publication item limit", semio_framework_diagnostic::TextSpan::at(1, 1)));
        }
        let patch: Self =
            semio_framework_pack_json::from_json_str(line, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        validate_patch(&patch).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        Ok(patch)
    }
}

/// 🔡️ The patch as lowercase hexadecimal of its canonical JSON — the one-physical-line payload every hand-written stdio text
/// codec embeds (`patch-snapshot patch=<hex>`, or a `<tag>:<hex>` envelope).
pub fn snapshot_patch_hex(patch: &SnapshotPatch) -> String {
    <SnapshotPatch as kernel::OpText>::print_op(patch).bytes().map(|byte| format!("{byte:02x}")).collect()
}

/// 🔡️ Parses a [`snapshot_patch_hex`] payload back into its validated patch, refusing malformed hexadecimal.
pub fn snapshot_patch_from_hex(hex: &str) -> Result<SnapshotPatch, String> {
    if !hex.len().is_multiple_of(2) || !hex.bytes().all(|byte| byte.is_ascii_hexdigit()) || hex.len() > SNAPSHOT_PATCH_MAX_BYTES * 2 {
        return Err("snapshot patch is not hexadecimal of even length within the patch budget".into());
    }
    let bytes = (0..hex.len()).step_by(2).map(|at| u8::from_str_radix(&hex[at..at + 2], 16).map_err(|error| error.to_string())).collect::<Result<Vec<u8>, String>>()?;
    let json = String::from_utf8(bytes).map_err(|error| error.to_string())?;
    <SnapshotPatch as kernel::OpText>::parse_op(&json).map_err(|error| error.to_string())
}

/// 📝️ The `patch-snapshot` op line of a hand-written aggregate text codec: `patch-snapshot patch=<hex of the patch JSON>` (the
/// grammar rule `patch-snapshot = "patch-snapshot" "patch" "=" hex` every stdio mutation grammar shares).
pub fn snapshot_patch_text(patch: &SnapshotPatch) -> String {
    format!("patch-snapshot patch={}", snapshot_patch_hex(patch))
}

/// 📝️ Parses a [`snapshot_patch_text`] line back into its patch, refusing any other opcode or malformed hex.
pub fn snapshot_patch_from_text(line: &str) -> Result<SnapshotPatch, String> {
    snapshot_patch_from_hex(line.strip_prefix("patch-snapshot patch=").ok_or_else(|| "expected `patch-snapshot patch=<hex>`".to_string())?)
}

/// 💾️ The patch carried by the rest of a hand-written aggregate binary frame (its canonical JSON bytes, as [`OpBinary::encode_op`]
/// writes them), with the refusal as text for decoders that report strings.
pub fn snapshot_patch_from_bytes(bytes: &[u8]) -> Result<SnapshotPatch, String> {
    SnapshotPatch::decode_op(bytes).map_err(|error| error.to_string())
}

impl OpBinary for SnapshotPatch {
    fn encode_op(&self) -> Result<Vec<u8>, kernel::ProtocolError> {
        Ok(<Self as kernel::OpText>::print_op(self).into_bytes())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, kernel::ProtocolError> {
        let line = std::str::from_utf8(bytes).map_err(|error| kernel::ProtocolError::Malformed { what: "snapshot patch utf8", offset: 0, detail: error.to_string() })?;
        <Self as kernel::OpText>::parse_op(line).map_err(|error| kernel::ProtocolError::Malformed { what: "snapshot patch", offset: 0, detail: error.to_string() })
    }
}
/// ♻️ Incremental retirement of a retained patch: its pointers and its value tree retire through the framework cursors, so
/// a value of up to [`SNAPSHOT_PATCH_MAX_BYTES`] never drops in one turn.
impl semio_framework_value::retirement::RetireOwned for SnapshotPatch {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        match self {
            Self::Set { path, value } | Self::Insert { path, value, .. } | Self::Splice { path, value, .. } => (path, value).retirement(),
            Self::Remove { path } => path.retirement(),
            Self::Move { from, path, .. } => (from, path).retirement(),
            Self::Rename { path, key } => (path, key).retirement(),
        }
    }
}
//#endregion 🩹️Patch

//#region 🪜️Steps
/// 🪜️ One primitive value edit at decoded segments — what a [`SnapshotPatch`] lowers to (a move and a rename to two).
#[derive(Clone, Debug, PartialEq)]
enum StepEdit {
    Set(DslValue),
    Insert(DslValue),
    InsertAt(DslValue, usize),
    Remove,
    Splice { offset: usize, remove: usize, value: DslValue },
}

#[derive(Clone, Debug, PartialEq)]
struct Step {
    path: Vec<String>,
    edit: StepEdit,
}

fn pointer(path: &[String]) -> String {
    path.iter().map(|segment| format!("/{}", segment.replace('~', "~0").replace('/', "~1"))).collect()
}

fn path_error(code: &'static str, path: &[String], error: impl std::fmt::Display) -> SnapshotEditError {
    SnapshotEditError::new(code, pointer(path), error.to_string())
}

fn segments(path: &[String]) -> Vec<&str> {
    path.iter().map(String::as_str).collect()
}

fn at<S: ToValue>(snapshot: &S, path: &[String]) -> Result<DslValue, SnapshotEditError> {
    snapshot.value_at_path(&segments(path)).map_err(|error| path_error("snapshot-edit.path-missing", path, error))
}

fn shape<S: ToValue>(snapshot: &S, path: &[String]) -> Result<ValueShape, SnapshotEditError> {
    snapshot.value_shape_at_path(&segments(path)).map_err(|error| path_error("snapshot-edit.path-missing", path, error))
}

fn index(index: u64, path: &[String]) -> Result<usize, SnapshotEditError> {
    usize::try_from(index).ok().filter(|_| index <= SNAPSHOT_PATCH_MAX_INDEX).ok_or_else(|| path_error("snapshot-edit.invalid-index", path, "object insertion index exceeds the portable integer range"))
}

/// 📍️ `path` with an appending `-` of an array parent replaced by that array's length in `snapshot`.
fn normalize<S: ToValue>(snapshot: &S, path: &[String]) -> Result<Vec<String>, SnapshotEditError> {
    let mut path = path.to_vec();
    if path.last().is_some_and(|segment| segment == "-") {
        if let ValueShape::Array { len } = shape(snapshot, &path[..path.len() - 1])? {
            *path.last_mut().expect("an appending pointer has a last segment") = len.to_string();
        }
    }
    Ok(path)
}

fn object_key_index<S: ToValue>(snapshot: &S, path: &[String]) -> Result<Option<usize>, SnapshotEditError> {
    let Some((key, parent)) = path.split_last() else { return Ok(None) };
    let ValueShape::Object { len } = shape(snapshot, parent)? else { return Ok(None) };
    for position in 0..len {
        if snapshot.value_key_at_path(&segments(parent), position).map_err(|error| path_error("snapshot-edit.path-missing", parent, error))? == *key {
            return Ok(Some(position));
        }
    }
    Err(path_error("snapshot-edit.path-missing", path, "the addressed object key does not exist"))
}

fn steps<S: ToValue>(snapshot: &S, patch: &SnapshotPatch) -> Result<Vec<Step>, SnapshotEditError> {
    Ok(match patch {
        SnapshotPatch::Set { path, value } => vec![Step { path: decode_pointer(path)?, edit: StepEdit::Set(value.clone()) }],
        SnapshotPatch::Insert { path, value, index: None } => vec![Step { path: decode_pointer(path)?, edit: StepEdit::Insert(value.clone()) }],
        SnapshotPatch::Insert { path, value, index: Some(position) } => {
            let path = decode_pointer(path)?;
            let position = index(*position, &path)?;
            vec![Step { path, edit: StepEdit::InsertAt(value.clone(), position) }]
        }
        SnapshotPatch::Remove { path } => vec![Step { path: decode_pointer(path)?, edit: StepEdit::Remove }],
        SnapshotPatch::Move { from, path, index: position } => {
            let from = decode_pointer(from)?;
            let path = decode_pointer(path)?;
            if from.is_empty() || path.is_empty() || (path.len() > from.len() && path.starts_with(&from)) {
                return Err(SnapshotEditError::new("snapshot-edit.invalid-move", pointer(&path), "a move cannot address the root or a descendant of its source"));
            }
            if from == path {
                return Ok(Vec::new());
            }
            let value = at(snapshot, &from)?;
            let edit = match position {
                Some(position) => StepEdit::InsertAt(value, index(*position, &path)?),
                None => StepEdit::Insert(value),
            };
            vec![Step { path: from, edit: StepEdit::Remove }, Step { path, edit }]
        }
        SnapshotPatch::Rename { path, key } => {
            let path = decode_pointer(path)?;
            let Some((old, parent)) = path.split_last() else {
                return Err(SnapshotEditError::new("snapshot-edit.root-operation", "", "cannot rename the document root"));
            };
            if !matches!(shape(snapshot, parent)?, ValueShape::Object { .. }) {
                return Err(SnapshotEditError::new("snapshot-edit.not-object", pointer(&path), "only an object key can be renamed"));
            }
            if old == key {
                return Ok(Vec::new());
            }
            let value = at(snapshot, &path)?;
            let position = object_key_index(snapshot, &path)?.expect("a rename parent is an object");
            let mut destination = parent.to_vec();
            destination.push(key.clone());
            vec![Step { path, edit: StepEdit::Remove }, Step { path: destination, edit: StepEdit::InsertAt(value, position) }]
        }
        SnapshotPatch::Splice { path, offset, remove, value, .. } => {
            let path = decode_pointer(path)?;
            let (offset, remove) = (index(*offset, &path)?, index(*remove, &path)?);
            vec![Step { path, edit: StepEdit::Splice { offset, remove, value: value.clone() } }]
        }
    })
}

/// ✂️ The units `remove` covers at `offset` of the container at `path`, validated against its length.
fn splice_range(offset: usize, remove: usize, len: usize, path: &[String]) -> Result<std::ops::Range<usize>, SnapshotEditError> {
    offset.checked_add(remove).filter(|end| *end <= len).map(|end| offset..end).ok_or_else(|| path_error("snapshot-edit.index-out-of-bounds", path, "the splice range exceeds the container"))
}

fn splice_mismatch(path: &[String]) -> SnapshotEditError {
    path_error("snapshot-edit.splice-mismatch", path, "the splice value is not of the addressed container's kind")
}

fn child(path: &[String], segment: impl Into<String>) -> Vec<String> {
    let mut child = path.to_vec();
    child.push(segment.into());
    child
}

fn octets(value: &DslValue, path: &[String]) -> Result<Vec<u8>, SnapshotEditError> {
    match value {
        DslValue::Bytes(bytes) => Ok(bytes.clone()),
        DslValue::Array(items) => items.iter().map(|item| item.as_u64().and_then(|byte| u8::try_from(byte).ok()).ok_or_else(|| splice_mismatch(path))).collect(),
        _ => Err(splice_mismatch(path)),
    }
}

fn text_at<S: ToValue>(snapshot: &S, path: &[String]) -> Result<String, SnapshotEditError> {
    match at(snapshot, path)? {
        DslValue::String(text) => Ok(text),
        _ => Err(path_error("snapshot-edit.not-container", path, "the addressed value is not text")),
    }
}

fn octets_at<S: ToValue>(snapshot: &S, path: &[String]) -> Result<Vec<u8>, SnapshotEditError> {
    octets(&at(snapshot, path)?, path)
}

fn text_range(text: &str, range: &std::ops::Range<usize>, path: &[String]) -> Result<(), SnapshotEditError> {
    if text.is_char_boundary(range.start) && text.is_char_boundary(range.end) {
        Ok(())
    } else {
        Err(path_error("snapshot-edit.char-boundary", path, "a text splice must address UTF-8 character boundaries"))
    }
}

/// ✂️ The units a splice at `offset` covering `remove` removes from the container at `path`, as a value of its kind.
fn splice_removed<S: ToValue>(snapshot: &S, path: &[String], offset: usize, remove: usize) -> Result<DslValue, SnapshotEditError> {
    Ok(match shape(snapshot, path)? {
        ValueShape::Array { len } => DslValue::Array(splice_range(offset, remove, len, path)?.map(|position| at(snapshot, &child(path, position.to_string()))).collect::<Result<_, _>>()?),
        ValueShape::Bytes { len } => DslValue::Bytes(octets_at(snapshot, path)?[splice_range(offset, remove, len, path)?].to_vec()),
        ValueShape::String => {
            let text = text_at(snapshot, path)?;
            let range = splice_range(offset, remove, text.len(), path)?;
            text_range(&text, &range, path)?;
            DslValue::String(text[range].to_string())
        }
        ValueShape::Object { len } => DslValue::Object(
            splice_range(offset, remove, len, path)?
                .map(|position| {
                    let key = snapshot.value_key_at_path(&segments(path), position).map_err(|error| path_error("snapshot-edit.path-missing", path, error))?;
                    let value = at(snapshot, &child(path, key.clone()))?;
                    Ok((key, value))
                })
                .collect::<Result<_, SnapshotEditError>>()?,
        ),
        _ => return Err(path_error("snapshot-edit.not-container", path, "a splice addresses an array, octets, text or an object")),
    })
}

/// 🔢️ The units `value` inserts into a container of `shape` (items, octets, UTF-8 bytes or members).
fn splice_units(shape: &ValueShape, value: &DslValue, path: &[String]) -> Result<usize, SnapshotEditError> {
    match (shape, value) {
        (ValueShape::Array { .. } | ValueShape::Bytes { .. }, DslValue::Array(items)) => Ok(items.len()),
        (ValueShape::Array { .. } | ValueShape::Bytes { .. }, DslValue::Bytes(bytes)) => Ok(bytes.len()),
        (ValueShape::String, DslValue::String(text)) => Ok(text.len()),
        (ValueShape::Object { .. }, DslValue::Object(members)) => Ok(members.len()),
        _ => Err(splice_mismatch(path)),
    }
}

fn edit<S: ToValue + FromValue>(snapshot: &mut S, path: &[String], edit: ValueEdit) -> Result<(), SnapshotEditError> {
    snapshot.edit_value_at_path(&segments(path), edit).map_err(|error| path_error("snapshot-edit.schema-invalid", path, error))
}

fn equivalent_at<S: ToValue>(snapshot: &S, path: &[String], value: &DslValue) -> Result<(), SnapshotEditError> {
    if values_equivalent(&at(snapshot, path)?, value) {
        Ok(())
    } else {
        Err(path_error("snapshot-edit.lossy-conversion", path, "the typed field would normalize or discard part of the edit"))
    }
}

/// ✂️ Applies one splice: the common prefix of removed and inserted units is replaced in place (an object member only when its
/// key is unchanged), the rest removed and inserted, so a typed struct member or array item is never transiently absent.
fn apply_splice<S: ToValue + FromValue>(snapshot: &mut S, path: &[String], offset: usize, remove: usize, value: &DslValue) -> Result<(), SnapshotEditError> {
    match shape(snapshot, path)? {
        ValueShape::Array { len } => {
            splice_range(offset, remove, len, path)?;
            let items = match value {
                DslValue::Array(items) => items.clone(),
                DslValue::Bytes(bytes) => bytes.iter().map(|byte| DslValue::uint(u64::from(*byte))).collect(),
                _ => return Err(splice_mismatch(path)),
            };
            let common = remove.min(items.len());
            for (position, item) in items.iter().enumerate().take(common) {
                edit(snapshot, &child(path, (offset + position).to_string()), ValueEdit::Set(item.clone()))?;
            }
            for _ in common..remove {
                edit(snapshot, &child(path, (offset + common).to_string()), ValueEdit::Remove)?;
            }
            for (position, item) in items.iter().enumerate().skip(common) {
                edit(snapshot, &child(path, (offset + position).to_string()), ValueEdit::Insert(item.clone()))?;
            }
            items.iter().enumerate().try_for_each(|(position, item)| equivalent_at(snapshot, &child(path, (offset + position).to_string()), item))
        }
        ValueShape::Bytes { len } => {
            let range = splice_range(offset, remove, len, path)?;
            let mut bytes = octets_at(snapshot, path)?;
            bytes.splice(range, octets(value, path)?);
            let next = DslValue::Bytes(bytes);
            edit(snapshot, path, ValueEdit::Set(next.clone()))?;
            equivalent_at(snapshot, path, &next)
        }
        ValueShape::String => {
            let DslValue::String(inserted) = value else { return Err(splice_mismatch(path)) };
            let mut text = text_at(snapshot, path)?;
            let range = splice_range(offset, remove, text.len(), path)?;
            text_range(&text, &range, path)?;
            text.replace_range(range, inserted);
            let next = DslValue::String(text);
            edit(snapshot, path, ValueEdit::Set(next.clone()))?;
            equivalent_at(snapshot, path, &next)
        }
        ValueShape::Object { len } => {
            let DslValue::Object(members) = value else { return Err(splice_mismatch(path)) };
            let removed = splice_range(offset, remove, len, path)?.map(|position| snapshot.value_key_at_path(&segments(path), position).map_err(|error| path_error("snapshot-edit.path-missing", path, error))).collect::<Result<Vec<_>, _>>()?;
            let mut keys = std::collections::BTreeSet::new();
            for (key, _) in members {
                if !keys.insert(key.as_str()) || (!removed.contains(key) && object_key_index(snapshot, &child(path, key.clone())).is_ok()) {
                    return Err(path_error("snapshot-edit.key-exists", &child(path, key.clone()), format!("object key '{key}' already exists")));
                }
            }
            let common = removed.iter().zip(members).take_while(|(removed, (key, _))| *removed == key).count();
            for (key, member) in &members[..common] {
                edit(snapshot, &child(path, key.clone()), ValueEdit::Set(member.clone()))?;
            }
            for key in &removed[common..] {
                edit(snapshot, &child(path, key.clone()), ValueEdit::Remove)?;
            }
            for (position, (key, member)) in members.iter().enumerate().skip(common) {
                edit(snapshot, &child(path, key.clone()), ValueEdit::InsertAt { value: member.clone(), index: offset + position })?;
            }
            members.iter().try_for_each(|(key, member)| equivalent_at(snapshot, &child(path, key.clone()), member))
        }
        _ => Err(path_error("snapshot-edit.not-container", path, "a splice addresses an array, octets, text or an object")),
    }
}

fn apply_step<S: ToValue + FromValue>(snapshot: &mut S, step: &Step) -> Result<(), SnapshotEditError> {
    if let StepEdit::Splice { offset, remove, value } = &step.edit {
        return apply_splice(snapshot, &step.path, *offset, *remove, value);
    }
    let path = match step.edit {
        StepEdit::Insert(_) => normalize(snapshot, &step.path)?,
        _ => step.path.clone(),
    };
    let edit = match &step.edit {
        StepEdit::Set(value) => {
            shape(snapshot, &path)?;
            ValueEdit::Set(value.clone())
        }
        StepEdit::Insert(value) => {
            let (key, parent) = path.split_last().ok_or_else(|| SnapshotEditError::new("snapshot-edit.root-operation", "", "cannot insert at the document root"))?;
            match shape(snapshot, parent)? {
                ValueShape::Object { .. } if object_key_index(snapshot, &path).is_ok() => return Err(path_error("snapshot-edit.key-exists", &path, format!("object key '{key}' already exists"))),
                ValueShape::Array { len } => {
                    super::array_index(key, len, &pointer(&path), true)?;
                }
                ValueShape::Object { .. } => {}
                _ => return Err(path_error("snapshot-edit.not-container", &path, "the addressed parent is a scalar")),
            }
            ValueEdit::Insert(value.clone())
        }
        StepEdit::InsertAt(value, position) => {
            let (key, parent) = path.split_last().ok_or_else(|| SnapshotEditError::new("snapshot-edit.root-operation", "", "cannot insert at the document root"))?;
            let ValueShape::Object { len } = shape(snapshot, parent)? else {
                return Err(path_error("snapshot-edit.not-object", &path, "positioned insertion requires an object parent"));
            };
            if object_key_index(snapshot, &path).is_ok() {
                return Err(path_error("snapshot-edit.key-exists", &path, format!("object key '{key}' already exists")));
            }
            if *position > len {
                return Err(path_error("snapshot-edit.index-out-of-bounds", &path, "object insertion index is out of range"));
            }
            ValueEdit::InsertAt { value: value.clone(), index: *position }
        }
        StepEdit::Remove => {
            if path.is_empty() {
                return Err(SnapshotEditError::new("snapshot-edit.root-operation", "", "cannot remove the document root"));
            }
            shape(snapshot, &path)?;
            ValueEdit::Remove
        }
        StepEdit::Splice { .. } => unreachable!("a splice step is applied by apply_splice"),
    };
    snapshot.edit_value_at_path(&segments(&path), edit).map_err(|error| path_error("snapshot-edit.schema-invalid", &path, error))?;
    if let StepEdit::Set(value) | StepEdit::Insert(value) | StepEdit::InsertAt(value, _) = &step.edit {
        if !values_equivalent(&at(snapshot, &path)?, value) {
            return Err(path_error("snapshot-edit.lossy-conversion", &path, "the typed field would normalize or discard part of the edit"));
        }
    }
    Ok(())
}

fn validate_patch(patch: &SnapshotPatch) -> Result<(), SnapshotEditError> {
    validate_patch_within(patch, SNAPSHOT_PATCH_MAX_BYTES)
}

fn validate_patch_within(patch: &SnapshotPatch, budget: usize) -> Result<(), SnapshotEditError> {
    let pointers = match patch {
        SnapshotPatch::Move { from, path, .. } => vec![from, path],
        SnapshotPatch::Set { path, .. } | SnapshotPatch::Insert { path, .. } | SnapshotPatch::Remove { path } | SnapshotPatch::Rename { path, .. } | SnapshotPatch::Splice { path, .. } => vec![path],
    };
    for path in pointers {
        if decode_pointer(path)?.len() > SNAPSHOT_PATCH_MAX_SEGMENTS {
            return Err(SnapshotEditError::new("snapshot-edit.patch-limit", path.as_str(), "a snapshot patch pointer permits 128 segments"));
        }
    }
    if let SnapshotPatch::Insert { index: Some(position), path, .. } | SnapshotPatch::Move { index: Some(position), path, .. } | SnapshotPatch::Splice { offset: position, path, .. } = patch {
        index(*position, &decode_pointer(path)?)?;
    }
    if let SnapshotPatch::Splice { remove, path, value, .. } = patch {
        index(*remove, &decode_pointer(path)?)?;
        if !matches!(value, DslValue::Array(_) | DslValue::Bytes(_) | DslValue::String(_) | DslValue::Object(_)) {
            return Err(SnapshotEditError::new("snapshot-edit.splice-mismatch", path.as_str(), "a splice inserts array items, octets, text or object members"));
        }
    }
    if let SnapshotPatch::Set { value, path } | SnapshotPatch::Insert { value, path, .. } | SnapshotPatch::Splice { value, path, .. } = patch {
        validate_value(value, path)?;
    }
    if patch.encode_op().map_err(|error| SnapshotEditError::new("snapshot-edit.patch-limit", "", error.to_string()))?.len() > budget.min(SNAPSHOT_PATCH_MAX_BYTES) {
        return Err(SnapshotEditError::new("snapshot-edit.patch-limit", "", "snapshot patch exceeds the native publication item limit"));
    }
    Ok(())
}
//#endregion 🪜️Steps

//#region 🧭️Edits
/// 🧭️ Translates one user edit into ONE canonical path operation: an appending pointer becomes the index it lands on, a
/// source replacement the set of the document root.
pub fn prepare_snapshot_patch<S: ToValue + FromValue + Clone>(snapshot: &S, event: &SnapshotEditEvent) -> Result<SnapshotPatch, SnapshotEditError> {
    let canonical = |path: &str| decode_pointer(path).map(|segments| pointer(&segments));
    let patch = match event {
        SnapshotEditEvent::SetValue { path, value } => SnapshotPatch::Set { path: canonical(path)?, value: value.clone() },
        SnapshotEditEvent::InsertValue { path, value } => SnapshotPatch::Insert { path: pointer(&normalize(snapshot, &decode_pointer(path)?)?), value: value.clone(), index: None },
        SnapshotEditEvent::RemoveValue { path } => SnapshotPatch::Remove { path: canonical(path)? },
        SnapshotEditEvent::ReplaceSource { source } => SnapshotPatch::Set { path: String::new(), value: snapshot_from_edit_source::<DslValue>(source)? },
        SnapshotEditEvent::MoveValue { from, path } => {
            let segments = decode_pointer(from)?;
            let destination = decode_pointer(path)?;
            let mut intermediate = snapshot.clone();
            if !segments.is_empty() && segments != destination && !(destination.len() > segments.len() && destination.starts_with(&segments)) {
                apply_step(&mut intermediate, &Step { path: segments.clone(), edit: StepEdit::Remove })?;
            }
            SnapshotPatch::Move { from: pointer(&segments), path: pointer(&normalize(&intermediate, &destination)?), index: None }
        }
        SnapshotEditEvent::RenameKey { path, key } => SnapshotPatch::Rename { path: canonical(path)?, key: key.clone() },
    };
    validate_patch(&patch)?;
    steps(snapshot, &patch)?;
    Ok(patch)
}

/// 🛡️ Applies one path operation to a detached native snapshot without projecting unrelated fields; a refusal leaves the
/// original untouched.
pub fn apply_snapshot_patch<S: ToValue + FromValue + Clone>(snapshot: &S, patch: &SnapshotPatch) -> Result<S, SnapshotEditError> {
    validate_patch(patch)?;
    let mut next = snapshot.clone();
    for step in steps(snapshot, patch)? {
        apply_step(&mut next, &step)?;
    }
    Ok(next)
}

/// 🛂️ [`apply_snapshot_patch`] followed by the artifact's own whole-snapshot invariant `check` (package authority, cross-part
/// coherence, …); a result the check rejects is refused as `snapshot-edit.schema-invalid` at the patch pointer.
pub fn apply_snapshot_patch_checked<S: ToValue + FromValue + Clone, E: std::fmt::Display>(snapshot: &S, patch: &SnapshotPatch, check: impl FnOnce(&S) -> Result<(), E>) -> Result<S, SnapshotEditError> {
    let next = apply_snapshot_patch(snapshot, patch)?;
    if !patch.continued() {
        check(&next).map_err(|error| SnapshotEditError::new("snapshot-edit.schema-invalid", patch.path(), error.to_string()))?;
    }
    Ok(next)
}

fn schema_path<S: ToValue>(snapshot: &S, path: &[String]) -> Result<Vec<semio_framework_schema::SchemaFragmentPathSegment>, SnapshotEditError> {
    use semio_framework_schema::SchemaFragmentPathSegment;
    let mut result = Vec::with_capacity(path.len());
    for (position, segment) in path.iter().enumerate() {
        let parent = &path[..position];
        result.push(match shape(snapshot, parent)? {
            ValueShape::Array { len } => SchemaFragmentPathSegment::Index(super::array_index(segment, len, &pointer(path), true)?),
            ValueShape::Object { .. } => SchemaFragmentPathSegment::Key(segment.clone()),
            _ => return Err(path_error("snapshot-edit.not-container", parent, "the addressed parent is a scalar")),
        });
    }
    Ok(result)
}

fn fragment_path(path: &[semio_framework_schema::SchemaFragmentPathSegment]) -> Vec<String> {
    use semio_framework_schema::SchemaFragmentPathSegment;
    path.iter()
        .map(|segment| match segment {
            SchemaFragmentPathSegment::Key(key) => key.clone(),
            SchemaFragmentPathSegment::Index(position) => position.to_string(),
            SchemaFragmentPathSegment::Append => "-".into(),
        })
        .collect()
}

fn project_context<S: ToValue>(snapshot: &S, path: &[semio_framework_schema::SchemaFragmentPathSegment]) -> Result<DslValue, semio_framework_schema::SchemaFragmentContextRefusal> {
    use semio_framework_schema::SchemaFragmentContextRefusal;
    fn project<S: ToValue>(snapshot: &S, path: &mut Vec<String>, remaining: &mut usize) -> Result<DslValue, semio_framework_value::ValueError> {
        if *remaining == 0 || path.len() > SNAPSHOT_PATCH_MAX_SEGMENTS {
            return Err(semio_framework_value::ValueError::new(if path.len() > SNAPSHOT_PATCH_MAX_SEGMENTS { semio_framework_value::ValueRefusalKind::DepthLimit } else { semio_framework_value::ValueRefusalKind::WorkLimit }, "validation context exceeds its bounded node budget"));
        }
        *remaining -= 1;
        match snapshot.value_shape_at_path(&segments(path))? {
            ValueShape::Array { len } => {
                if len > *remaining {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit, "validation array exceeds its bounded node budget"));
                }
                let mut values = Vec::with_capacity(len);
                for position in 0..len {
                    path.push(position.to_string());
                    values.push(project(snapshot, path, remaining)?);
                    path.pop();
                }
                Ok(DslValue::Array(values))
            }
            ValueShape::Object { len } => {
                if len > *remaining {
                    return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit, "validation object exceeds its bounded node budget"));
                }
                let mut values = Vec::with_capacity(len);
                for position in 0..len {
                    let key = snapshot.value_key_at_path(&segments(path), position)?;
                    path.push(key.clone());
                    values.push((key, project(snapshot, path, remaining)?));
                    path.pop();
                }
                Ok(DslValue::Object(values))
            }
            _ => snapshot.value_at_path(&segments(path)),
        }
    }
    let mut path = fragment_path(path);
    let mut remaining = super::SNAPSHOT_EDIT_MAXIMUM_VALUE_NODES;
    project(snapshot, &mut path, &mut remaining).map_err(|error| SchemaFragmentContextRefusal::new(error.to_string()))
}

fn project_shape<S: ToValue>(snapshot: &S, path: &[semio_framework_schema::SchemaFragmentPathSegment]) -> Result<semio_framework_schema::SchemaFragmentValueShape, semio_framework_schema::SchemaFragmentContextRefusal> {
    use semio_framework_schema::{SchemaFragmentContextRefusal, SchemaFragmentValueShape};
    let path = fragment_path(path);
    snapshot
        .value_shape_at_path(&segments(&path))
        .map(|shape| match shape {
            ValueShape::Null => SchemaFragmentValueShape::Null,
            ValueShape::Bool => SchemaFragmentValueShape::Bool,
            ValueShape::Number => SchemaFragmentValueShape::Number,
            ValueShape::String => SchemaFragmentValueShape::String,
            ValueShape::Bytes { len } | ValueShape::Array { len } => SchemaFragmentValueShape::Array { len },
            ValueShape::Object { len } => SchemaFragmentValueShape::Object { len },
        })
        .map_err(|error| SchemaFragmentContextRefusal::new(error.to_string()))
}

fn apply_validated_snapshot_patch<S: ToValue + FromValue + Clone>(snapshot: &S, patch: &SnapshotPatch, validator: &semio_framework_schema::OwnedJsonSchemaValidator) -> Result<S, SnapshotEditError> {
    use semio_framework_schema::SchemaFragmentOperation;
    validate_patch(patch)?;
    let steps = steps(snapshot, patch)?;
    let mut next = snapshot.clone();
    let mut paths = Vec::with_capacity(steps.len());
    for step in &steps {
        paths.push(schema_path(&next, &step.path)?);
        apply_step(&mut next, step)?;
    }
    for (step, path) in steps.iter().zip(paths) {
        let container = match &step.edit {
            StepEdit::Splice { .. } => Some(at(&next, &step.path)?),
            _ => None,
        };
        let (operation, candidate) = match &step.edit {
            StepEdit::Set(value) => (SchemaFragmentOperation::Set, Some(value)),
            StepEdit::Insert(value) | StepEdit::InsertAt(value, _) => (SchemaFragmentOperation::Insert, Some(value)),
            StepEdit::Remove => (SchemaFragmentOperation::Remove, None),
            StepEdit::Splice { .. } => (SchemaFragmentOperation::Set, container.as_ref()),
        };
        validator.validate_dsl_fragment_with_context(&path, operation, candidate, |context| project_context(&next, context), |context| project_shape(&next, context)).map_err(|error| SnapshotEditError::new(error.code, error.path, error.message))?;
    }
    Ok(next)
}

/// 🧬️ Validates one path operation against the exact editor schema and its bounded post-edit context.
pub fn apply_snapshot_patch_for_dialect<S: ToValue + FromValue + Clone>(snapshot: &S, patch: &SnapshotPatch, dialect: semio_framework_plugin::Dialect, document_schema: &str) -> Result<S, SnapshotEditError> {
    let identity = snapshot.value_at_path(&["schema"]).map_err(|error| path_error("snapshot-edit.path-missing", &["schema".into()], error))?;
    super::validate_snapshot_schema_for_dialect(&DslValue::Object(vec![("schema".into(), identity.clone())]), dialect, document_schema)?;
    let id = super::snapshot_schema_descriptor_for_dialect(dialect, document_schema)?;
    let validators = super::SNAPSHOT_SCHEMA_VALIDATORS.get_or_init(|| std::sync::RwLock::new(HashMap::new()));
    if !validators.read().unwrap_or_else(|poisoned| poisoned.into_inner()).contains_key(&id) {
        let validator = semio_framework_schema::structural_validator_for(&id, "snapshot").map_err(|error| SnapshotEditError::new("snapshot-edit.invalid-schema-contract", "", error.to_string()))?;
        validators.write().unwrap_or_else(|poisoned| poisoned.into_inner()).insert(id.clone(), validator);
    }
    let cached = validators.read().unwrap_or_else(|poisoned| poisoned.into_inner());
    let next = apply_validated_snapshot_patch(snapshot, patch, &cached[&id])?;
    let next_identity = next.value_at_path(&["schema"]).map_err(|error| path_error("snapshot-edit.path-missing", &["schema".into()], error))?;
    if identity != next_identity {
        return Err(SnapshotEditError::new("snapshot-edit.schema-identity", "/schema", "an edit cannot change the registered snapshot schema identity"));
    }
    Ok(next)
}

/// ↩️ The exact inverse of `patch` captured from its publication base as ONE operation: a set restores the prior value, an
/// insert removes the member it added, a removal re-inserts the value (an object member at its former place), a move moves
/// back, a rename renames back in place and a splice splices the removed units back. Refused when that one operation exceeds
/// the patch budget — [`inverse_snapshot_patches`] then answers the same inverse in parts.
pub fn inverse_snapshot_patch<S: ToValue + FromValue + Clone>(snapshot: &S, patch: &SnapshotPatch) -> Result<SnapshotPatch, SnapshotEditError> {
    validate_patch(patch)?;
    let inverse = exact_inverse(snapshot, patch)?;
    validate_patch(&inverse)?;
    Ok(inverse)
}

/// ↩️ The exact inverse of `patch` as the parts a store row may carry: the one operation of [`inverse_snapshot_patch`] when it
/// fits [`SNAPSHOT_PATCH_MAX_BYTES`], else at most [`SNAPSHOT_PATCH_MAX_INVERSE_PARTS`] parts that restore the prior value
/// exactly (see [`inverse_snapshot_patches_within`]). A patch whose prior value exceeds that bound has no exact inverse and is
/// refused (`snapshot-edit.inverse-limit`), so undo is never lossy.
pub fn inverse_snapshot_patches<S: ToValue + FromValue + Clone>(snapshot: &S, patch: &SnapshotPatch) -> Result<Vec<SnapshotPatch>, SnapshotEditError> {
    inverse_snapshot_patches_within(snapshot, patch, SNAPSHOT_PATCH_MAX_BYTES)
}

/// 🧩️ [`inverse_snapshot_patches`] with every part's canonical JSON within `budget` bytes. A prior value too large for one part
/// is placed as its shell — containers emptied, struct-like objects keeping every key with shells of their members, an object
/// too large even as that a prefix of its members — through a splice of its parent (a `set` of the root only when the root
/// changes kind), then refilled by splices appending items, octets, UTF-8 chunks and members in document order, each part
/// packed greedily up to the budget. Every part except the last is `continued`, so whole-snapshot invariants are checked once
/// the prior value is complete; intermediate states keep every struct member and array item present.
pub fn inverse_snapshot_patches_within<S: ToValue + FromValue + Clone>(snapshot: &S, patch: &SnapshotPatch, budget: usize) -> Result<Vec<SnapshotPatch>, SnapshotEditError> {
    validate_patch(patch)?;
    let inverse = exact_inverse(snapshot, patch)?;
    if validate_patch_within(&inverse, budget).is_ok() {
        return Ok(vec![inverse]);
    }
    let mut planner = InversePlanner { budget: budget.min(SNAPSHOT_PATCH_MAX_BYTES), parts: Vec::new() };
    planner.split(snapshot, patch, inverse)?;
    let last = planner.parts.len().saturating_sub(1);
    for (position, part) in planner.parts.iter_mut().enumerate() {
        if let SnapshotPatch::Splice { continued, .. } = part {
            *continued = position < last;
        }
        validate_patch_within(part, planner.budget)?;
    }
    Ok(planner.parts)
}

fn exact_inverse<S: ToValue + FromValue + Clone>(snapshot: &S, patch: &SnapshotPatch) -> Result<SnapshotPatch, SnapshotEditError> {
    Ok(match patch {
        SnapshotPatch::Set { path, .. } => SnapshotPatch::Set { path: path.clone(), value: at(snapshot, &decode_pointer(path)?)? },
        SnapshotPatch::Insert { path, .. } => SnapshotPatch::Remove { path: pointer(&normalize(snapshot, &decode_pointer(path)?)?) },
        SnapshotPatch::Remove { path } => {
            let segments = decode_pointer(path)?;
            SnapshotPatch::Insert { path: path.clone(), value: at(snapshot, &segments)?, index: object_key_index(snapshot, &segments)?.map(|position| position as u64) }
        }
        SnapshotPatch::Move { from, path, .. } => {
            let segments = decode_pointer(from)?;
            let destination = decode_pointer(path)?;
            if segments == destination {
                patch.clone()
            } else {
                let mut intermediate = snapshot.clone();
                apply_step(&mut intermediate, &Step { path: segments.clone(), edit: StepEdit::Remove })?;
                SnapshotPatch::Move { from: pointer(&normalize(&intermediate, &destination)?), path: from.clone(), index: object_key_index(snapshot, &segments)?.map(|position| position as u64) }
            }
        }
        SnapshotPatch::Rename { path, key } => {
            let segments = decode_pointer(path)?;
            match segments.split_last() {
                Some((old, parent)) if old != key => {
                    let mut renamed = parent.to_vec();
                    renamed.push(key.clone());
                    SnapshotPatch::Rename { path: pointer(&renamed), key: old.clone() }
                }
                _ => patch.clone(),
            }
        }
        SnapshotPatch::Splice { path, offset, remove, value, .. } => {
            let segments = decode_pointer(path)?;
            let inserted = splice_units(&shape(snapshot, &segments)?, value, &segments)?;
            let removed = splice_removed(snapshot, &segments, index(*offset, &segments)?, index(*remove, &segments)?)?;
            SnapshotPatch::Splice { path: path.clone(), offset: *offset, remove: inserted as u64, value: removed, continued: false }
        }
    })
}
//#endregion 🧭️Edits

//#region 🧩️InverseParts
/// 📏️ The length of `text` as a canonical JSON string (the framework writer's escapes).
fn text_json_len(text: &str) -> usize {
    2 + text
        .chars()
        .map(|character| match character {
            '"' | '\\' | '\u{0008}' | '\u{000C}' | '\n' | '\r' | '\t' => 2,
            control if (control as u32) < 0x20 => 6,
            other => other.len_utf8(),
        })
        .sum::<usize>()
}

fn digits(value: u64) -> usize {
    value.checked_ilog10().map_or(1, |exponent| exponent as usize + 1)
}

/// 📏️ The exact length of `value`'s canonical JSON (the text [`SnapshotPatch`] encodes it as), computed without writing it.
fn json_len(value: &DslValue) -> usize {
    let separators = |count: usize| count.saturating_sub(1);
    match value {
        DslValue::Null | DslValue::Bool(true) => 4,
        DslValue::Bool(false) => 5,
        DslValue::Number(semio_framework_value::Number::UInt(number)) => digits(*number),
        DslValue::Number(semio_framework_value::Number::Int(number)) => digits(number.unsigned_abs()) + usize::from(*number < 0),
        DslValue::Number(semio_framework_value::Number::Float(_)) => semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(value)).len(),
        DslValue::String(text) => text_json_len(text),
        DslValue::Bytes(bytes) => 2 + bytes.iter().map(|byte| digits(u64::from(*byte))).sum::<usize>() + separators(bytes.len()),
        DslValue::Array(items) => 2 + items.iter().map(json_len).sum::<usize>() + separators(items.len()),
        DslValue::Object(members) => 2 + members.iter().map(|(key, member)| text_json_len(key) + 1 + json_len(member)).sum::<usize>() + separators(members.len()),
    }
}

/// 🐚️ The smallest stand-in of `value` that keeps its kind and, for objects, every key: containers empty, text empty.
fn minimal_len(value: &DslValue) -> usize {
    match value {
        DslValue::String(_) | DslValue::Bytes(_) | DslValue::Array(_) => 2,
        DslValue::Object(members) => 2 + members.iter().map(|(key, member)| text_json_len(key) + 1 + minimal_len(member)).sum::<usize>() + members.len().saturating_sub(1),
        scalar => json_len(scalar),
    }
}

/// 🐚️ How a placed stand-in differs from the value it stands for: whole, emptied (refilled by appends), or an object whose
/// first `placed` members stand in by their own shells (the rest appended as members).
enum Shell {
    Whole,
    Emptied,
    Object { members: Vec<Shell>, placed: usize },
}

/// 🐚️ The stand-in of `value` within `room` bytes of canonical JSON: the value itself when it fits; otherwise its kind emptied,
/// or an object keeping, in order, each member whole while the rest still fit as their minimal stand-ins, each other member
/// as its own shell, and — when even the minimal stand-ins do not fit — only the members before the first that does not.
fn shell(value: &DslValue, room: usize) -> (DslValue, Shell) {
    if json_len(value) <= room {
        return (value.clone(), Shell::Whole);
    }
    match value {
        DslValue::String(_) => (DslValue::String(String::new()), Shell::Emptied),
        DslValue::Bytes(_) => (DslValue::Bytes(Vec::new()), Shell::Emptied),
        DslValue::Array(_) => (DslValue::Array(Vec::new()), Shell::Emptied),
        DslValue::Object(members) => {
            let entry = |position: usize, key: &str| text_json_len(key) + 1 + usize::from(position > 0);
            let mut rest = members.iter().enumerate().map(|(position, (key, member))| entry(position, key) + minimal_len(member)).sum::<usize>();
            let (mut used, mut placed, mut shells) = (2usize, Vec::new(), Vec::new());
            for (position, (key, member)) in members.iter().enumerate() {
                let minimal = minimal_len(member);
                rest -= entry(position, key) + minimal;
                let Some(available) = room.checked_sub(used + entry(position, key) + rest).filter(|available| *available >= minimal) else { break };
                let (stand_in, member_shell) = shell(member, available);
                used += entry(position, key) + json_len(&stand_in);
                placed.push((key.clone(), stand_in));
                shells.push(member_shell);
            }
            let count = placed.len();
            (DslValue::Object(placed), Shell::Object { members: shells, placed: count })
        }
        scalar => (scalar.clone(), Shell::Whole),
    }
}

/// 🧱️ The units a splice appends: items, octets, text or members.
enum Units<'v> {
    Items(&'v [DslValue]),
    Octets(&'v [u8]),
    Text(&'v str),
    Members(&'v [(String, DslValue)]),
}

impl<'v> Units<'v> {
    fn of(value: &'v DslValue) -> Option<Self> {
        Some(match value {
            DslValue::Array(items) => Self::Items(items),
            DslValue::Bytes(bytes) => Self::Octets(bytes),
            DslValue::String(text) => Self::Text(text),
            DslValue::Object(members) => Self::Members(members),
            _ => return None,
        })
    }

    fn len(&self) -> usize {
        match self {
            Self::Items(items) => items.len(),
            Self::Octets(bytes) => bytes.len(),
            Self::Text(text) => text.len(),
            Self::Members(members) => members.len(),
        }
    }
}

struct InversePlanner {
    budget: usize,
    parts: Vec<SnapshotPatch>,
}

impl InversePlanner {
    fn limit() -> SnapshotEditError {
        SnapshotEditError::new("snapshot-edit.inverse-limit", "", "the exact inverse exceeds its bounded number of parts")
    }

    fn push(&mut self, part: SnapshotPatch) -> Result<(), SnapshotEditError> {
        if self.parts.len() >= SNAPSHOT_PATCH_MAX_INVERSE_PARTS {
            return Err(Self::limit());
        }
        self.parts.push(part);
        Ok(())
    }

    /// 📏️ The room a splice part at `path`/`offset`/`remove` leaves for its value (counting a `continued` marker).
    fn room(&self, path: &str, offset: usize, remove: usize) -> Result<usize, SnapshotEditError> {
        let envelope = SnapshotPatch::Splice { path: path.into(), offset: offset as u64, remove: remove as u64, value: DslValue::Null, continued: true };
        let bytes = envelope.encode_op().map_err(|error| SnapshotEditError::new("snapshot-edit.patch-limit", path, error.to_string()))?.len() - 4;
        self.budget.checked_sub(bytes).ok_or_else(Self::limit)
    }

    fn split<S: ToValue + FromValue + Clone>(&mut self, snapshot: &S, patch: &SnapshotPatch, inverse: SnapshotPatch) -> Result<(), SnapshotEditError> {
        match inverse {
            SnapshotPatch::Set { path, value } => {
                let segments = decode_pointer(&path)?;
                match segments.split_last() {
                    Some((key, parent)) => {
                        let position = match shape(snapshot, parent)? {
                            ValueShape::Object { .. } => object_key_index(snapshot, &segments)?.ok_or_else(Self::limit)?,
                            _ => super::array_index(key, usize::MAX, &path, false)?,
                        };
                        self.place(snapshot, parent, key, position, 1, &value)
                    }
                    None => {
                        let SnapshotPatch::Set { value: current, .. } = patch else { return Err(Self::limit()) };
                        self.root(current, &value)
                    }
                }
            }
            SnapshotPatch::Insert { path, value, index: position } => {
                let segments = decode_pointer(&path)?;
                let (key, parent) = segments.split_last().ok_or_else(Self::limit)?;
                let position = match position {
                    Some(position) => index(position, &segments)?,
                    None => super::array_index(key, usize::MAX, &path, false)?,
                };
                self.place(snapshot, parent, key, position, 0, &value)
            }
            SnapshotPatch::Splice { path, offset, remove, value, .. } => {
                let units = Units::of(&value).ok_or_else(Self::limit)?;
                self.append(&decode_pointer(&path)?, offset as usize, remove as usize, units)
            }
            SnapshotPatch::Remove { .. } | SnapshotPatch::Move { .. } | SnapshotPatch::Rename { .. } => Err(SnapshotEditError::new("snapshot-edit.patch-limit", "", "a pointer-only inverse exceeds the patch budget")),
        }
    }

    /// 📍️ Places the shell of `value` as member `key` of `parent` (replacing `remove` units at `position`), then refills it.
    fn place<S: ToValue>(&mut self, snapshot: &S, parent: &[String], key: &str, position: usize, remove: usize, value: &DslValue) -> Result<(), SnapshotEditError> {
        let object = matches!(shape(snapshot, parent)?, ValueShape::Object { .. });
        let wrapper = if object { 2 + text_json_len(key) + 1 } else { 2 };
        let room = self.room(&pointer(parent), position, remove)?.checked_sub(wrapper).ok_or_else(Self::limit)?;
        let (stand_in, placed) = shell(value, room);
        let wrapped = if object { DslValue::Object(vec![(key.to_string(), stand_in)]) } else { DslValue::Array(vec![stand_in]) };
        self.push(SnapshotPatch::Splice { path: pointer(parent), offset: position as u64, remove: remove as u64, value: wrapped, continued: true })?;
        self.fill(&child(parent, key), value, &placed)
    }

    /// 🌳️ Restores a replaced document root: a splice of the whole root when the kind is unchanged, else a root `set` of the shell.
    fn root(&mut self, current: &DslValue, value: &DslValue) -> Result<(), SnapshotEditError> {
        let same_kind = std::mem::discriminant(current) == std::mem::discriminant(value);
        match (Units::of(current), Units::of(value)) {
            (Some(current), Some(Units::Members(members))) if same_kind => {
                let room = self.room("", 0, current.len())?;
                let (stand_in, placed) = shell(value, room);
                self.push(SnapshotPatch::Splice { path: String::new(), offset: 0, remove: current.len() as u64, value: stand_in, continued: true })?;
                self.fill_object(&[], members, &placed)
            }
            (Some(current), Some(units)) if same_kind => self.append(&[], 0, current.len(), units),
            _ => {
                let envelope = SnapshotPatch::Set { path: String::new(), value: DslValue::Null }.encode_op().map_err(|error| SnapshotEditError::new("snapshot-edit.patch-limit", "", error.to_string()))?.len() - 4;
                let (stand_in, placed) = shell(value, self.budget.checked_sub(envelope).ok_or_else(Self::limit)?);
                self.push(SnapshotPatch::Set { path: String::new(), value: stand_in })?;
                self.fill(&[], value, &placed)
            }
        }
    }

    fn fill(&mut self, path: &[String], value: &DslValue, placed: &Shell) -> Result<(), SnapshotEditError> {
        match (placed, value) {
            (Shell::Whole, _) => Ok(()),
            (Shell::Emptied, value) => self.append(path, 0, 0, Units::of(value).ok_or_else(Self::limit)?),
            (Shell::Object { .. }, DslValue::Object(members)) => self.fill_object(path, members, placed),
            _ => Err(Self::limit()),
        }
    }

    fn fill_object(&mut self, path: &[String], members: &[(String, DslValue)], placed: &Shell) -> Result<(), SnapshotEditError> {
        let Shell::Object { members: shells, placed } = placed else { return Ok(()) };
        for ((key, member), member_shell) in members.iter().zip(shells) {
            self.fill(&child(path, key.clone()), member, member_shell)?;
        }
        if *placed < members.len() {
            self.append(path, *placed, 0, Units::Members(&members[*placed..]))?;
        }
        Ok(())
    }

    /// ➕️ Appends `units` at `offset` of the container at `path` (the first part also removing `remove` units), packing each
    /// part greedily up to the budget; an item or member too large for a part of its own goes in as its shell and is refilled
    /// right after its part.
    fn append(&mut self, path: &[String], offset: usize, remove: usize, units: Units<'_>) -> Result<(), SnapshotEditError> {
        let text = pointer(path);
        let (mut start, mut remove, mut next) = (offset, remove, 0usize);
        while next < units.len() || remove > 0 {
            let room = self.room(&text, start, remove)?.checked_sub(2).ok_or_else(Self::limit)?;
            let mut used = 0usize;
            let first = next;
            let (value, refills) = match &units {
                Units::Text(content) => {
                    let mut end = next;
                    for character in content[next..].chars() {
                        let size = text_json_len(character.encode_utf8(&mut [0; 4])) - 2;
                        if used + size > room {
                            break;
                        }
                        used += size;
                        end += character.len_utf8();
                    }
                    next = end;
                    (DslValue::String(content[first..end].to_string()), Vec::new())
                }
                Units::Octets(bytes) => {
                    while next < bytes.len() && used + usize::from(next > first) + digits(u64::from(bytes[next])) <= room {
                        used += usize::from(next > first) + digits(u64::from(bytes[next]));
                        next += 1;
                    }
                    (DslValue::Bytes(bytes[first..next].to_vec()), Vec::new())
                }
                Units::Items(items) => {
                    let (mut batch, mut refills) = (Vec::new(), Vec::new());
                    while next < items.len() {
                        let (stand_in, placed) = shell(&items[next], room);
                        let size = usize::from(!batch.is_empty()) + json_len(&stand_in);
                        if !batch.is_empty() && used + size > room {
                            break;
                        }
                        used += size;
                        refills.push((child(path, (start + batch.len()).to_string()), &items[next], placed));
                        batch.push(stand_in);
                        next += 1;
                    }
                    (DslValue::Array(batch), refills)
                }
                Units::Members(members) => {
                    let (mut batch, mut refills) = (Vec::new(), Vec::new());
                    while next < members.len() {
                        let (key, member) = &members[next];
                        let (stand_in, placed) = shell(member, room.saturating_sub(text_json_len(key) + 1));
                        let size = usize::from(!batch.is_empty()) + text_json_len(key) + 1 + json_len(&stand_in);
                        if !batch.is_empty() && used + size > room {
                            break;
                        }
                        used += size;
                        refills.push((child(path, key.clone()), member, placed));
                        batch.push((key.clone(), stand_in));
                        next += 1;
                    }
                    (DslValue::Object(batch), refills)
                }
            };
            if next == first && remove == 0 {
                return Err(SnapshotEditError::new("snapshot-edit.patch-limit", text.as_str(), "a unit of the inverse cannot be split within the patch budget"));
            }
            self.push(SnapshotPatch::Splice { path: text.clone(), offset: start as u64, remove: remove as u64, value, continued: true })?;
            start += next - first;
            remove = 0;
            for (refill_path, member, placed) in refills {
                self.fill(&refill_path, member, &placed)?;
            }
        }
        Ok(())
    }
}
//#endregion 🧩️InverseParts

//#region 🧬️InputSchema
/// 📚️ Schema documents by `$id`, resolved once through the runtime's input-schema resolver.
fn schema_document(id: &str) -> Option<Arc<DslValue>> {
    static DOCUMENTS: OnceLock<Mutex<HashMap<String, Arc<DslValue>>>> = OnceLock::new();
    let documents = DOCUMENTS.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(document) = documents.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).get(id) {
        return Some(Arc::clone(document));
    }
    let document = Arc::new(semio_framework_plugin::registered_input_schema_document(id)?);
    documents.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).insert(id.to_string(), Arc::clone(&document));
    Some(document)
}

/// 🔍️ Resolves a schema document by its `$id`.
pub type SnapshotSchemaResolver<'r> = &'r dyn Fn(&str) -> Option<Arc<DslValue>>;

/// 🧭️ One place in a schema document: the document's `$id` and the JSON Pointer of the node inside it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SnapshotSchemaLocation {
    pub document: String,
    pub pointer: String,
}

impl SnapshotSchemaLocation {
    /// 🔗️ The `$ref` that names this location.
    pub fn reference(&self) -> String {
        if self.pointer.is_empty() {
            self.document.clone()
        } else {
            format!("{}#{}", self.document, self.pointer)
        }
    }

    fn child(&self, suffix: &str) -> Self {
        Self { document: self.document.clone(), pointer: format!("{}/{suffix}", self.pointer) }
    }
}

fn escape(segment: &str) -> String {
    segment.replace('~', "~0").replace('/', "~1")
}

fn node<'d>(document: &'d DslValue, pointer: &str) -> Option<&'d DslValue> {
    let mut current = document;
    for raw in pointer.split('/').skip(1) {
        let segment = raw.replace("~1", "/").replace("~0", "~");
        current = match current {
            DslValue::Object(_) => current.get(&segment)?,
            DslValue::Array(items) => items.get(segment.parse::<usize>().ok()?)?,
            _ => return None,
        };
    }
    Some(current)
}

/// 🔗️ `location` with every `$ref` it starts with followed (bounded), and the document the result lives in.
fn followed(location: &SnapshotSchemaLocation, resolve: SnapshotSchemaResolver<'_>) -> Option<(SnapshotSchemaLocation, Arc<DslValue>)> {
    let mut current = location.clone();
    for _ in 0..SNAPSHOT_SCHEMA_REF_HOPS {
        let document = resolve(&current.document)?;
        let Some(reference) = node(&document, &current.pointer)?.get("$ref").and_then(DslValue::as_str).map(str::to_string) else { return Some((current, document)) };
        let (id, fragment) = reference.split_once('#').unwrap_or((reference.as_str(), ""));
        current = SnapshotSchemaLocation { document: if id.is_empty() { current.document.clone() } else { id.to_string() }, pointer: fragment.to_string() };
    }
    None
}

fn is_index(segment: &str) -> bool {
    segment == "-" || (!segment.is_empty() && segment.bytes().all(|byte| byte.is_ascii_digit()))
}

/// 🪜️ Every location that may describe the instance member `segment` of a value described by `location`: a declared
/// property, the items of an array, the additional properties of a map — through every union branch that admits it.
fn member_locations(location: &SnapshotSchemaLocation, segment: &str, resolve: SnapshotSchemaResolver<'_>, depth: usize) -> Vec<SnapshotSchemaLocation> {
    let Some((resolved, document)) = followed(location, resolve).filter(|_| depth < SNAPSHOT_SCHEMA_REF_HOPS) else { return Vec::new() };
    let Some(schema) = node(&document, &resolved.pointer) else { return Vec::new() };
    let mut found = Vec::new();
    for union in ["oneOf", "anyOf", "allOf"] {
        if let Some(DslValue::Array(members)) = schema.get(union) {
            for position in 0..members.len() {
                found.extend(member_locations(&resolved.child(&format!("{union}/{position}")), segment, resolve, depth + 1));
            }
        }
    }
    if schema.get("properties").and_then(|properties| properties.get(segment)).is_some() {
        found.push(resolved.child(&format!("properties/{}", escape(segment))));
    } else if is_index(segment) && schema.get("items").is_some() {
        match schema.get("items") {
            Some(DslValue::Array(items)) => match segment.parse::<usize>().ok().filter(|position| *position < items.len()) {
                Some(position) => found.push(resolved.child(&format!("items/{position}"))),
                None if matches!(schema.get("additionalItems"), Some(DslValue::Object(_))) => found.push(resolved.child("additionalItems")),
                None => {}
            },
            Some(DslValue::Object(_)) => found.push(resolved.child("items")),
            _ => {}
        }
    } else if matches!(schema.get("additionalProperties"), Some(DslValue::Object(_))) {
        found.push(resolved.child("additionalProperties"));
    }
    found
}

/// 🧭️ The location of the sub-schema of `document` (a snapshot schema `$id`) that describes the instance at `segments`,
/// read from the schema alone: a union passes on the branches that admit the next segment, and a location is answered only
/// when every admitting branch resolves to the same schema. `None` when a document does not resolve, a segment is not
/// admitted, or the location is ambiguous.
pub fn snapshot_schema_location(document: &str, segments: &[String], resolve: SnapshotSchemaResolver<'_>) -> Option<SnapshotSchemaLocation> {
    let mut location = SnapshotSchemaLocation { document: document.to_string(), pointer: String::new() };
    for segment in segments {
        let candidates = member_locations(&location, segment, resolve, 0);
        let (first, document) = followed(candidates.first()?, resolve)?;
        let target = node(&document, &first.pointer)?;
        for candidate in candidates.iter().skip(1) {
            let (other, other_document) = followed(candidate, resolve)?;
            if other != first && node(&other_document, &other.pointer)? != target {
                return None;
            }
        }
        location = candidates.into_iter().next()?;
    }
    Some(location)
}

fn hidden_string() -> DslValue {
    DslValue::Object(vec![("type".into(), DslValue::String("string".into())), ("x-semio-ui".into(), DslValue::Object(vec![("widget".into(), DslValue::String("hidden".into()))]))])
}

fn hidden_index() -> DslValue {
    DslValue::Object(vec![
        ("type".into(), DslValue::String("integer".into())),
        ("minimum".into(), DslValue::Number(semio_framework_value::Number::UInt(0))),
        ("maximum".into(), DslValue::Number(semio_framework_value::Number::UInt(SNAPSHOT_PATCH_MAX_INDEX))),
        ("x-semio-ui".into(), DslValue::Object(vec![("widget".into(), DslValue::String("hidden".into()))])),
    ])
}

/// 🧬️ The input schema of `patch` against the snapshot schema whose `$id` is `snapshot_schema`, resolving documents
/// through `resolve`, as a leaf payload schema (`{patch: …}`): the operation pinned, its pointers and position hidden, a
/// written value typed by the snapshot sub-schema its pointer addresses (a `$ref` to that location, so its own UI facts
/// apply) and a rename's new key as text. `None` when that location does not resolve or is ambiguous.
pub fn snapshot_patch_input_schema_text(snapshot_schema: &str, patch: &SnapshotPatch, resolve: SnapshotSchemaResolver<'_>) -> Option<String> {
    let operation = ("operation".to_string(), DslValue::Object(vec![("const".into(), DslValue::String(patch.operation().into()))]));
    let (mut properties, mut required) = (vec![operation], vec!["operation", "path"]);
    match patch {
        SnapshotPatch::Set { path, .. } | SnapshotPatch::Insert { path, .. } => {
            let location = snapshot_schema_location(snapshot_schema, &decode_pointer(path).ok()?, resolve)?;
            properties.push(("path".into(), hidden_string()));
            properties.push(("value".into(), DslValue::Object(vec![("$ref".into(), DslValue::String(location.reference()))])));
            required.push("value");
            if let SnapshotPatch::Insert { index: Some(_), .. } = patch {
                properties.push(("index".into(), hidden_index()));
            }
        }
        SnapshotPatch::Remove { .. } => properties.push(("path".into(), hidden_string())),
        SnapshotPatch::Move { index, .. } => {
            properties.push(("from".into(), hidden_string()));
            properties.push(("path".into(), hidden_string()));
            required.push("from");
            if index.is_some() {
                properties.push(("index".into(), hidden_index()));
            }
        }
        SnapshotPatch::Rename { .. } => {
            properties.push(("path".into(), hidden_string()));
            properties.push(("key".into(), DslValue::Object(vec![("type".into(), DslValue::String("string".into()))])));
            required.push("key");
        }
        SnapshotPatch::Splice { .. } => return None,
    }
    let strings = |names: &[&str]| DslValue::Array(names.iter().map(|name| DslValue::String((*name).into())).collect());
    let label = DslValue::Object(vec![("en".into(), DslValue::String("Snapshot edit".into())), ("de".into(), DslValue::String("Änderung der Momentaufnahme".into()))]);
    let body = DslValue::Object(vec![
        ("type".into(), DslValue::String("object".into())),
        ("additionalProperties".into(), DslValue::Bool(false)),
        ("required".into(), strings(&required)),
        ("properties".into(), DslValue::Object(properties)),
        ("x-semio-ui".into(), DslValue::Object(vec![("label".into(), label)])),
    ]);
    let schema = DslValue::Object(vec![
        ("$schema".into(), DslValue::String("http://json-schema.org/draft-07/schema#".into())),
        ("title".into(), DslValue::String("PatchSnapshot".into())),
        ("type".into(), DslValue::String("object".into())),
        ("additionalProperties".into(), DslValue::Bool(false)),
        ("required".into(), strings(&["patch"])),
        ("properties".into(), DslValue::Object(vec![("patch".into(), body)])),
    ]);
    Some(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&schema)))
}

/// 🧬️ [`snapshot_patch_input_schema_text`] over the runtime's registered schema documents, admitted by the input reader
/// and interned (one leaked text per distinct schema, bounded by the snapshot schemas' locations). `None` sends the
/// caller to the leaf's own structural payload schema.
pub fn snapshot_patch_input_schema(snapshot_schema: &str, patch: &SnapshotPatch) -> Option<&'static str> {
    static SCHEMAS: OnceLock<Mutex<HashMap<String, Option<&'static str>>>> = OnceLock::new();
    let text = snapshot_patch_input_schema_text(snapshot_schema, patch, &schema_document)?;
    let schemas = SCHEMAS.get_or_init(|| Mutex::new(HashMap::new()));
    if let Some(known) = schemas.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).get(&text) {
        return *known;
    }
    let interned = semio_framework_plugin::mutation_input_defs(&text, &semio_framework_plugin::registered_input_schema_document).is_ok().then(|| &*Box::leak(text.clone().into_boxed_str()));
    schemas.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).insert(text, interned);
    interned
}
//#endregion 🧬️InputSchema

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
