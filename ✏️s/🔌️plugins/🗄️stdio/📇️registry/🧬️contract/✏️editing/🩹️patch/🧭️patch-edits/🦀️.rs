//! 🧭️ Preparing, applying and inverting snapshot patches against schema and dialect.

use super::*;

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
            ValueShape::Array { len } => SchemaFragmentPathSegment::Index(array_index(segment, len, &pointer(path), true)?),
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
    let mut remaining = SNAPSHOT_EDIT_MAXIMUM_VALUE_NODES;
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

/// 🧭️ Applies the caller's explicit snapshot schema to addressed paths and bounded post-edit context.
pub fn apply_validated_snapshot_patch<S: ToValue + FromValue + Clone>(snapshot: &S, patch: &SnapshotPatch, validator: &semio_framework_schema::OwnedJsonSchemaValidator) -> Result<S, SnapshotEditError> {
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
pub fn apply_snapshot_patch_for_dialect<S: ToValue + FromValue + Clone>(snapshot: &S, patch: &SnapshotPatch, dialect: semio_framework_artifact_reference::Dialect, document_schema: &str) -> Result<S, SnapshotEditError> {
    let identity = snapshot.value_at_path(&["schema"]).map_err(|error| path_error("snapshot-edit.path-missing", &["schema".into()], error))?;
    validate_snapshot_schema_for_dialect(&DslValue::Object(vec![("schema".into(), identity.clone())]), dialect, document_schema)?;
    let id = snapshot_schema_descriptor_for_dialect(dialect, document_schema)?;
    let validators = SNAPSHOT_SCHEMA_VALIDATORS.get_or_init(|| std::sync::RwLock::new(HashMap::new()));
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
