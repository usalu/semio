//! 🪜️ Lowering a patch to primitive steps, splices and their application and validation.

use super::*;

//#region 🪜️Steps
/// 🪜️ One primitive value edit at decoded segments — what a [`SnapshotPatch`] lowers to (a move and a rename to two).
#[derive(Clone, Debug, PartialEq)]
pub(super) enum StepEdit {
    Set(DslValue),
    Insert(DslValue),
    InsertAt(DslValue, usize),
    Remove,
    Splice { offset: usize, remove: usize, value: DslValue },
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct Step {
    pub(super) path: Vec<String>,
    pub(super) edit: StepEdit,
}

pub(super) fn pointer(path: &[String]) -> String {
    path.iter().map(|segment| format!("/{}", segment.replace('~', "~0").replace('/', "~1"))).collect()
}

pub(super) fn path_error(code: &'static str, path: &[String], error: impl std::fmt::Display) -> SnapshotEditError {
    SnapshotEditError::new(code, pointer(path), error.to_string())
}

pub(super) fn segments(path: &[String]) -> Vec<&str> {
    path.iter().map(String::as_str).collect()
}

pub(super) fn at<S: ToValue>(snapshot: &S, path: &[String]) -> Result<DslValue, SnapshotEditError> {
    snapshot.value_at_path(&segments(path)).map_err(|error| path_error("snapshot-edit.path-missing", path, error))
}

pub(super) fn shape<S: ToValue>(snapshot: &S, path: &[String]) -> Result<ValueShape, SnapshotEditError> {
    snapshot.value_shape_at_path(&segments(path)).map_err(|error| path_error("snapshot-edit.path-missing", path, error))
}

pub(super) fn index(index: u64, path: &[String]) -> Result<usize, SnapshotEditError> {
    usize::try_from(index).ok().filter(|_| index <= SNAPSHOT_PATCH_MAX_INDEX).ok_or_else(|| path_error("snapshot-edit.invalid-index", path, "object insertion index exceeds the portable integer range"))
}

/// 📍️ `path` with an appending `-` of an array parent replaced by that array's length in `snapshot`.
pub(super) fn normalize<S: ToValue>(snapshot: &S, path: &[String]) -> Result<Vec<String>, SnapshotEditError> {
    let mut path = path.to_vec();
    if path.last().is_some_and(|segment| segment == "-") {
        if let ValueShape::Array { len } = shape(snapshot, &path[..path.len() - 1])? {
            *path.last_mut().expect("an appending pointer has a last segment") = len.to_string();
        }
    }
    Ok(path)
}

pub(super) fn object_key_index<S: ToValue>(snapshot: &S, path: &[String]) -> Result<Option<usize>, SnapshotEditError> {
    let Some((key, parent)) = path.split_last() else { return Ok(None) };
    let ValueShape::Object { len } = shape(snapshot, parent)? else { return Ok(None) };
    for position in 0..len {
        if snapshot.value_key_at_path(&segments(parent), position).map_err(|error| path_error("snapshot-edit.path-missing", parent, error))? == *key {
            return Ok(Some(position));
        }
    }
    Err(path_error("snapshot-edit.path-missing", path, "the addressed object key does not exist"))
}

pub(super) fn steps<S: ToValue>(snapshot: &S, patch: &SnapshotPatch) -> Result<Vec<Step>, SnapshotEditError> {
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

pub(super) fn child(path: &[String], segment: impl Into<String>) -> Vec<String> {
    let mut child = path.to_vec();
    child.push(segment.into());
    child
}

pub(super) fn octets(value: &DslValue, path: &[String]) -> Result<Vec<u8>, SnapshotEditError> {
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
pub(super) fn splice_removed<S: ToValue>(snapshot: &S, path: &[String], offset: usize, remove: usize) -> Result<DslValue, SnapshotEditError> {
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
pub(super) fn splice_units(shape: &ValueShape, value: &DslValue, path: &[String]) -> Result<usize, SnapshotEditError> {
    match (shape, value) {
        (ValueShape::Array { .. } | ValueShape::Bytes { .. }, DslValue::Array(items)) => Ok(items.len()),
        (ValueShape::Array { .. } | ValueShape::Bytes { .. }, DslValue::Bytes(bytes)) => Ok(bytes.len()),
        (ValueShape::String, DslValue::String(text)) => Ok(text.len()),
        (ValueShape::Object { .. }, DslValue::Object(members)) => Ok(members.len()),
        _ => Err(splice_mismatch(path)),
    }
}

pub(super) fn edit<S: ToValue + FromValue>(snapshot: &mut S, path: &[String], edit: ValueEdit) -> Result<(), SnapshotEditError> {
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

pub(super) fn apply_step<S: ToValue + FromValue>(snapshot: &mut S, step: &Step) -> Result<(), SnapshotEditError> {
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
                    array_index(key, len, &pointer(&path), true)?;
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

pub(super) fn validate_patch(patch: &SnapshotPatch) -> Result<(), SnapshotEditError> {
    validate_patch_within(patch, SNAPSHOT_PATCH_MAX_BYTES)
}

pub(super) fn validate_patch_within(patch: &SnapshotPatch, budget: usize) -> Result<(), SnapshotEditError> {
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
