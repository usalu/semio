//! 🩹️ Compact native snapshot edits and exact inverse patches.

use super::{decode_pointer, snapshot_from_edit_source, validate_value, values_equivalent, SnapshotEditError, SnapshotEditEvent};
use crate::{kernel, pack, value_derive};
use kernel::{DslValue, FromValue, OpBinary, ToValue, ValueEdit, ValueShape};

pub const SNAPSHOT_PATCH_MAX_BYTES: usize = 1_048_576;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
pub enum SnapshotPatchEdit {
    Set { value: DslValue },
    Insert { value: DslValue },
    InsertAt { value: DslValue, index: usize },
    Remove,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(deny_unknown_fields)]
pub struct SnapshotValuePatch {
    pub path: Vec<String>,
    pub edit: SnapshotPatchEdit,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(deny_unknown_fields)]
pub struct SnapshotPatch {
    pub edits: Vec<SnapshotValuePatch>,
}

impl kernel::os_dsl::DslField for SnapshotPatch {
    fn shape() -> kernel::os_dsl::Shape {
        kernel::os_dsl::Shape::Value
    }

    fn to_value(&self) -> kernel::os_dsl::FieldValue {
        kernel::os_dsl::FieldValue::Value(<Self as ToValue>::to_value(self))
    }

    fn from_value(value: &kernel::os_dsl::FieldValue) -> Result<Self, String> {
        let kernel::os_dsl::FieldValue::Value(value) = value else { return Err("snapshot patch requires a structured value".into()) };
        let patch = <Self as FromValue>::from_value(value.clone()).map_err(|error| error.to_string())?;
        validate_patch(&patch).map_err(|error| error.to_string())?;
        Ok(patch)
    }
}

impl kernel::OpText for SnapshotPatch {
    fn print_op(&self) -> String {
        pack::json::to_json_string(self)
    }

    fn parse_op(line: &str) -> Result<Self, kernel::TextError> {
        if line.len() > SNAPSHOT_PATCH_MAX_BYTES {
            return Err(kernel::TextError::new("snapshot patch exceeds the native publication item limit", kernel::TextSpan::at(1, 1)));
        }
        super::validate_source_keys(line).map_err(|error| kernel::TextError::new(error.to_string(), kernel::TextSpan::at(1, 1)))?;
        let patch: Self = pack::json::from_json_str(line).map_err(|error| kernel::TextError::new(error.to_string(), kernel::TextSpan::at(1, 1)))?;
        validate_patch(&patch).map_err(|error| kernel::TextError::new(error.to_string(), kernel::TextSpan::at(1, 1)))?;
        Ok(patch)
    }
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

fn pointer(path: &[String]) -> String {
    path.iter().map(|segment| format!("/{}", segment.replace('~', "~0").replace('/', "~1"))).collect()
}

fn path_error(path: &[String], error: impl std::fmt::Display) -> SnapshotEditError {
    SnapshotEditError::new("snapshot-edit.path-invalid", pointer(path), error.to_string())
}

fn segments(path: &[String]) -> Vec<&str> {
    path.iter().map(String::as_str).collect()
}

fn at<S: ToValue>(snapshot: &S, path: &[String]) -> Result<DslValue, SnapshotEditError> {
    snapshot.value_at_path(&segments(path)).map_err(|error| path_error(path, error))
}

fn normalize_path<S: ToValue>(snapshot: &S, patch: &SnapshotValuePatch) -> Result<Vec<String>, SnapshotEditError> {
    let mut path = patch.path.clone();
    if matches!(patch.edit, SnapshotPatchEdit::Insert { .. }) && path.last().is_some_and(|segment| segment == "-") {
        let parent = &path[..path.len() - 1];
        if let ValueShape::Array { len } = snapshot.value_shape_at_path(&segments(parent)).map_err(|error| path_error(parent, error))? {
            *path.last_mut().unwrap() = len.to_string();
        }
    }
    Ok(path)
}

fn validate_patch(patch: &SnapshotPatch) -> Result<(), SnapshotEditError> {
    if patch.edits.len() > 2 || patch.edits.iter().any(|edit| edit.path.len() > 128) {
        return Err(SnapshotEditError::new("snapshot-edit.patch-limit", "", "a snapshot patch permits two edits and 128 path segments"));
    }
    for edit in &patch.edits {
        if matches!(&edit.edit, SnapshotPatchEdit::InsertAt { index, .. } if *index as u64 > 9_007_199_254_740_991) {
            return Err(path_error(&edit.path, "object insertion index exceeds the portable integer range"));
        }
        if let SnapshotPatchEdit::Set { value } | SnapshotPatchEdit::Insert { value } | SnapshotPatchEdit::InsertAt { value, .. } = &edit.edit {
            validate_value(value, &pointer(&edit.path))?;
        }
    }
    if patch.encode_op().map_err(|error| path_error(&[], error))?.len() > SNAPSHOT_PATCH_MAX_BYTES {
        return Err(SnapshotEditError::new("snapshot-edit.patch-limit", "", "snapshot patch exceeds the native publication item limit"));
    }
    Ok(())
}

fn apply_one<S: ToValue + FromValue>(snapshot: &mut S, patch: &SnapshotValuePatch) -> Result<(), SnapshotEditError> {
    let path = normalize_path(snapshot, patch)?;
    let edit = match &patch.edit {
        SnapshotPatchEdit::Set { value } => {
            snapshot.value_shape_at_path(&segments(&path)).map_err(|error| path_error(&path, error))?;
            ValueEdit::Set(value.clone())
        }
        SnapshotPatchEdit::Insert { value } => ValueEdit::Insert(value.clone()),
        SnapshotPatchEdit::InsertAt { value, index } => {
            let parent = path.split_last().ok_or_else(|| path_error(&path, "cannot insert at the document root"))?.1;
            let ValueShape::Object { len } = snapshot.value_shape_at_path(&segments(parent)).map_err(|error| path_error(parent, error))? else {
                return Err(path_error(&path, "positioned insertion requires an object parent"));
            };
            if *index > len {
                return Err(path_error(&path, "object insertion index is out of range"));
            }
            ValueEdit::InsertAt { value: value.clone(), index: *index }
        }
        SnapshotPatchEdit::Remove => ValueEdit::Remove,
    };
    snapshot.edit_value_at_path(&segments(&path), edit).map_err(|error| path_error(&path, error))?;
    if let SnapshotPatchEdit::Set { value } | SnapshotPatchEdit::Insert { value } | SnapshotPatchEdit::InsertAt { value, .. } = &patch.edit {
        if !values_equivalent(&at(snapshot, &path)?, value) {
            return Err(SnapshotEditError::new("snapshot-edit.lossy-conversion", pointer(&path), "the typed field would normalize or discard part of the edit"));
        }
    }
    Ok(())
}

/// 🧭️ Translates one user edit into at most two native path operations.
pub fn prepare_snapshot_patch<S: ToValue + FromValue + Clone>(snapshot: &S, event: &SnapshotEditEvent) -> Result<SnapshotPatch, SnapshotEditError> {
    let single = |path: &str, edit| -> Result<SnapshotPatch, SnapshotEditError> {
        let edit = SnapshotValuePatch { path: decode_pointer(path)?, edit };
        Ok(SnapshotPatch { edits: vec![SnapshotValuePatch { path: normalize_path(snapshot, &edit)?, ..edit }] })
    };
    let patch = match event {
        SnapshotEditEvent::SetValue { path, value } => single(path, SnapshotPatchEdit::Set { value: value.clone() })?,
        SnapshotEditEvent::InsertValue { path, value } => single(path, SnapshotPatchEdit::Insert { value: value.clone() })?,
        SnapshotEditEvent::RemoveValue { path } => single(path, SnapshotPatchEdit::Remove)?,
        SnapshotEditEvent::ReplaceSource { source } => single("", SnapshotPatchEdit::Set { value: snapshot_from_edit_source::<DslValue>(source)? })?,
        SnapshotEditEvent::MoveValue { from, path } => {
            let from = decode_pointer(from)?;
            let path = decode_pointer(path)?;
            if from.is_empty() || path.is_empty() || (path.len() > from.len() && path.starts_with(&from)) {
                return Err(SnapshotEditError::new("snapshot-edit.invalid-move", pointer(&path), "move cannot address the root or a descendant of its source"));
            }
            let value = at(snapshot, &from)?;
            if from == path {
                SnapshotPatch { edits: Vec::new() }
            } else {
                let remove = SnapshotValuePatch { path: from, edit: SnapshotPatchEdit::Remove };
                let mut intermediate = snapshot.clone();
                apply_one(&mut intermediate, &remove)?;
                let insert = SnapshotValuePatch { path, edit: SnapshotPatchEdit::Insert { value } };
                SnapshotPatch { edits: vec![remove, SnapshotValuePatch { path: normalize_path(&intermediate, &insert)?, ..insert }] }
            }
        }
        SnapshotEditEvent::RenameKey { path, key } => {
            let path = decode_pointer(path)?;
            let Some((old_key, parent)) = path.split_last() else {
                return Err(SnapshotEditError::new("snapshot-edit.root-operation", "", "cannot rename the document root"));
            };
            if !matches!(snapshot.value_shape_at_path(&segments(parent)).map_err(|error| path_error(parent, error))?, ValueShape::Object { .. }) {
                return Err(SnapshotEditError::new("snapshot-edit.not-object", pointer(&path), "only an object key can be renamed"));
            }
            let value = at(snapshot, &path)?;
            if old_key == key {
                SnapshotPatch { edits: Vec::new() }
            } else {
                let mut destination = parent.to_vec();
                destination.push(key.clone());
                let index = object_key_index(snapshot, &path)?.expect("rename parent is an object");
                SnapshotPatch { edits: vec![SnapshotValuePatch { path, edit: SnapshotPatchEdit::Remove }, SnapshotValuePatch { path: destination, edit: SnapshotPatchEdit::InsertAt { value, index } }] }
            }
        }
    };
    validate_patch(&patch)?;
    Ok(patch)
}

/// 🛡️ Applies structural edits to a detached native snapshot without projecting unrelated fields.
pub fn apply_snapshot_patch<S: ToValue + FromValue + Clone>(snapshot: &S, patch: &SnapshotPatch) -> Result<S, SnapshotEditError> {
    validate_patch(patch)?;
    let mut next = snapshot.clone();
    for edit in &patch.edits {
        apply_one(&mut next, edit)?;
    }
    Ok(next)
}

fn schema_path<S: ToValue>(snapshot: &S, path: &[String]) -> Result<Vec<semio_framework_schema::SchemaFragmentPathSegment>, SnapshotEditError> {
    use semio_framework_schema::SchemaFragmentPathSegment;
    let mut result = Vec::with_capacity(path.len());
    for (index, segment) in path.iter().enumerate() {
        let parent = &path[..index];
        result.push(match snapshot.value_shape_at_path(&segments(parent)).map_err(|error| path_error(parent, error))? {
            ValueShape::Array { len } => SchemaFragmentPathSegment::Index(super::array_index(segment, len, &pointer(path), true)?),
            ValueShape::Object { .. } => SchemaFragmentPathSegment::Key(segment.clone()),
            _ => return Err(path_error(parent, "the addressed parent is a scalar")),
        });
    }
    Ok(result)
}

fn project_context<S: ToValue>(snapshot: &S, path: &[semio_framework_schema::SchemaFragmentPathSegment]) -> Result<DslValue, semio_framework_schema::SchemaFragmentContextRefusal> {
    use semio_framework_schema::{SchemaFragmentContextRefusal, SchemaFragmentPathSegment};
    let mut path = path
        .iter()
        .map(|segment| match segment {
            SchemaFragmentPathSegment::Key(key) => key.clone(),
            SchemaFragmentPathSegment::Index(index) => index.to_string(),
            SchemaFragmentPathSegment::Append => "-".into(),
        })
        .collect::<Vec<_>>();
    fn project<S: ToValue>(snapshot: &S, path: &mut Vec<String>, remaining: &mut usize) -> Result<DslValue, kernel::ValueError> {
        if *remaining == 0 || path.len() > 128 {
            return Err(kernel::ValueError::new("validation context exceeds its bounded node budget"));
        }
        *remaining -= 1;
        match snapshot.value_shape_at_path(&segments(path))? {
            ValueShape::Array { len } => {
                if len > *remaining {
                    return Err(kernel::ValueError::new("validation array exceeds its bounded node budget"));
                }
                let mut values = Vec::with_capacity(len);
                for index in 0..len {
                    path.push(index.to_string());
                    values.push(project(snapshot, path, remaining)?);
                    path.pop();
                }
                Ok(DslValue::Array(values))
            }
            ValueShape::Object { len } => {
                if len > *remaining {
                    return Err(kernel::ValueError::new("validation object exceeds its bounded node budget"));
                }
                let mut values = Vec::with_capacity(len);
                for index in 0..len {
                    let key = snapshot.value_key_at_path(&segments(path), index)?;
                    path.push(key.clone());
                    values.push((key, project(snapshot, path, remaining)?));
                    path.pop();
                }
                Ok(DslValue::Object(values))
            }
            _ => snapshot.value_at_path(&segments(path)),
        }
    }
    let mut remaining = super::SNAPSHOT_EDIT_MAXIMUM_VALUE_NODES;
    project(snapshot, &mut path, &mut remaining).map_err(|error| SchemaFragmentContextRefusal::new(error.to_string()))
}

fn project_shape<S: ToValue>(snapshot: &S, path: &[semio_framework_schema::SchemaFragmentPathSegment]) -> Result<semio_framework_schema::SchemaFragmentValueShape, semio_framework_schema::SchemaFragmentContextRefusal> {
    use semio_framework_schema::{SchemaFragmentContextRefusal, SchemaFragmentPathSegment, SchemaFragmentValueShape};
    let path = path
        .iter()
        .map(|segment| match segment {
            SchemaFragmentPathSegment::Key(key) => key.clone(),
            SchemaFragmentPathSegment::Index(index) => index.to_string(),
            SchemaFragmentPathSegment::Append => "-".into(),
        })
        .collect::<Vec<_>>();
    let path = path.iter().map(String::as_str).collect::<Vec<_>>();
    snapshot
        .value_shape_at_path(&path)
        .map(|shape| match shape {
            ValueShape::Null => SchemaFragmentValueShape::Null,
            ValueShape::Bool => SchemaFragmentValueShape::Bool,
            ValueShape::Number => SchemaFragmentValueShape::Number,
            ValueShape::String => SchemaFragmentValueShape::String,
            ValueShape::Bytes { len } => SchemaFragmentValueShape::Array { len },
            ValueShape::Array { len } => SchemaFragmentValueShape::Array { len },
            ValueShape::Object { len } => SchemaFragmentValueShape::Object { len },
        })
        .map_err(|error| SchemaFragmentContextRefusal::new(error.to_string()))
}

fn apply_validated_snapshot_patch<S: ToValue + FromValue + Clone>(snapshot: &S, patch: &SnapshotPatch, validator: &semio_framework_schema::OwnedJsonSchemaValidator) -> Result<S, SnapshotEditError> {
    use semio_framework_schema::SchemaFragmentOperation;
    validate_patch(patch)?;
    let mut next = snapshot.clone();
    let mut paths = Vec::with_capacity(patch.edits.len());
    for edit in &patch.edits {
        paths.push(schema_path(&next, &edit.path)?);
        apply_one(&mut next, edit)?;
    }
    for (edit, path) in patch.edits.iter().zip(paths) {
        let (operation, candidate) = match &edit.edit {
            SnapshotPatchEdit::Set { value } => (SchemaFragmentOperation::Set, Some(value)),
            SnapshotPatchEdit::Insert { value } | SnapshotPatchEdit::InsertAt { value, .. } => (SchemaFragmentOperation::Insert, Some(value)),
            SnapshotPatchEdit::Remove => (SchemaFragmentOperation::Remove, None),
        };
        validator.validate_dsl_fragment_with_context(&path, operation, candidate, |context| project_context(&next, context), |context| project_shape(&next, context)).map_err(|error| SnapshotEditError::new(error.code, error.path, error.message))?;
    }
    Ok(next)
}

/// 🧬️ Validates compact edits against the exact editor schema and bounded post-edit context.
pub fn apply_snapshot_patch_for_dialect<S: ToValue + FromValue + Clone>(snapshot: &S, patch: &SnapshotPatch, dialect: semio_framework_plugin::Dialect, document_schema: &str) -> Result<S, SnapshotEditError> {
    let identity = snapshot.value_at_path(&["schema"]).map_err(|error| path_error(&["schema".into()], error))?;
    super::validate_snapshot_schema_for_dialect(&DslValue::Object(vec![("schema".into(), identity.clone())]), dialect, document_schema)?;
    let id = super::snapshot_schema_descriptor_for_dialect(dialect, document_schema)?;
    let validators = super::SNAPSHOT_SCHEMA_VALIDATORS.get_or_init(|| std::sync::RwLock::new(std::collections::HashMap::new()));
    if !validators.read().unwrap_or_else(|poisoned| poisoned.into_inner()).contains_key(&id) {
        let validator = semio_framework_schema::structural_validator_for(&id, "snapshot").map_err(|error| SnapshotEditError::new("snapshot-edit.invalid-schema-contract", "", error.to_string()))?;
        validators.write().unwrap_or_else(|poisoned| poisoned.into_inner()).insert(id.clone(), validator);
    }
    let cached = validators.read().unwrap_or_else(|poisoned| poisoned.into_inner());
    let next = apply_validated_snapshot_patch(snapshot, patch, &cached[&id])?;
    let next_identity = next.value_at_path(&["schema"]).map_err(|error| path_error(&["schema".into()], error))?;
    if identity != next_identity {
        return Err(SnapshotEditError::new("snapshot-edit.schema-identity", "/schema", "an edit cannot change the registered snapshot schema identity"));
    }
    Ok(next)
}

/// ↩️ Captures an exact compact inverse from the current native publication base.
pub fn inverse_snapshot_patch<S: ToValue + FromValue + Clone>(snapshot: &S, patch: &SnapshotPatch) -> Result<SnapshotPatch, SnapshotEditError> {
    validate_patch(patch)?;
    let mut next = snapshot.clone();
    let mut edits = Vec::with_capacity(patch.edits.len());
    for edit in &patch.edits {
        let path = normalize_path(&next, edit)?;
        let inverse = match &edit.edit {
            SnapshotPatchEdit::Set { .. } => SnapshotPatchEdit::Set { value: at(&next, &path)? },
            SnapshotPatchEdit::Insert { .. } | SnapshotPatchEdit::InsertAt { .. } => SnapshotPatchEdit::Remove,
            SnapshotPatchEdit::Remove => match object_key_index(&next, &path)? {
                Some(index) => SnapshotPatchEdit::InsertAt { value: at(&next, &path)?, index },
                None => SnapshotPatchEdit::Insert { value: at(&next, &path)? },
            },
        };
        apply_one(&mut next, edit)?;
        edits.push(SnapshotValuePatch { path, edit: inverse });
    }
    edits.reverse();
    let inverse = SnapshotPatch { edits };
    validate_patch(&inverse)?;
    Ok(inverse)
}

fn object_key_index<S: ToValue>(snapshot: &S, path: &[String]) -> Result<Option<usize>, SnapshotEditError> {
    let Some((key, parent)) = path.split_last() else {
        return Ok(None);
    };
    let ValueShape::Object { len } = snapshot.value_shape_at_path(&segments(parent)).map_err(|error| path_error(parent, error))? else {
        return Ok(None);
    };
    for index in 0..len {
        if snapshot.value_key_at_path(&segments(parent), index).map_err(|error| path_error(parent, error))? == *key {
            return Ok(Some(index));
        }
    }
    Err(path_error(path, "the addressed object key does not exist"))
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
