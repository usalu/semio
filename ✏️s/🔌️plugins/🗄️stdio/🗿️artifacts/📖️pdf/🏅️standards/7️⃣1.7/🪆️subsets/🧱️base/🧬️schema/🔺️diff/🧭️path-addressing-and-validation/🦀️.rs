//! 🧭️ Object path addressing and the validators of every diff lane.

use super::*;

//#region 🔖️PathAddressing
/// 🧭️ One step of a `NodePath`-style address into ONE object's `PdfObject` tree -- used by
/// `SetDictEntry`/`RemoveDictEntry` mutations (recipe: "path addresses nesting inside one
/// object's PdfValue tree, same NodePath-style addressing xml/svg use"). Per ISO 32000-1, a raw
/// `Stream` can only ever be an indirect object's OWN top-level value (never nested inside an
/// Array/Dict as a value -- that requires an indirect `Ref`), so only `path == []` can possibly
/// address a `Stream`'s dict; every deeper step is guaranteed `Array`/`Dict`.
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum PdfPathSegment {
    ArrayIndex { index: usize },
    DictKey { key: String },
}

/// 🔍️ Walks `path` from `root`, returning the container reached (an `Array`/`Dict`/`Stream`) --
/// or `None` on an out-of-range/kind-mismatched step (graceful no-op upstream).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn resolve_value<'a>(root: &'a PdfObject, path: &[PdfPathSegment]) -> Option<&'a PdfObject> {
    let mut current = root;
    for seg in path {
        current = match (seg, current) {
            (PdfPathSegment::ArrayIndex { index }, PdfObject::Array(items)) => items.get(*index)?,
            (PdfPathSegment::DictKey { key }, PdfObject::Dict(entries)) => &entries.iter().find(|e| &e.key == key)?.value,
            (PdfPathSegment::DictKey { key }, PdfObject::Stream { dict, .. }) => &dict.iter().find(|e| &e.key == key)?.value,
            _ => return None,
        };
    }
    Some(current)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn dict_entries_of(value: &PdfObject) -> Option<&[PdfDictEntry]> {
    match value {
        PdfObject::Dict(d) => Some(d.as_slice()),
        PdfObject::Stream { dict, .. } => Some(dict.as_slice()),
        _ => None,
    }
}

/// 🧩️ Wraps a leaf `PdfDictDiff` (a modification to the dict/stream-dict located at `path` inside
/// object `id`) into a full `PdfDiff`, folding `path` from innermost to outermost. Only the
/// OUTERMOST step (`path == []`, i.e. the object's own top-level value) can be a `Stream` --
/// every step beyond that is guaranteed `Dict`/`Array` per `PdfPathSegment`'s own doc comment.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn diff_at_object_path(id: ObjRef, path: &[PdfPathSegment], is_root_stream: bool, leaf: PdfDictDiff) -> PdfDiff {
    let mut node = if is_root_stream { PdfValueDiff::Stream { dict: Some(leaf), data: None, filters: None } } else { PdfValueDiff::Dict { diff: leaf } };
    for seg in path.iter().rev() {
        node = match seg {
            PdfPathSegment::ArrayIndex { index } => PdfValueDiff::Array { diff: PdfArrayDiff { modified: vec![PdfArrayModified { index: *index, diff: node }], ..Default::default() } },
            PdfPathSegment::DictKey { key } => PdfValueDiff::Dict { diff: PdfDictDiff { modified: vec![PdfDictModified { key: key.clone(), diff: node }], ..Default::default() } },
        };
    }
    PdfDiff { objects: Some(PdfObjectsDiff { modified: vec![PdfObjectModified { id, diff: node }], ..Default::default() }), ..Default::default() }
}
//#endregion 🔖️PathAddressing
pub(super) fn validate_index_triple(base_len: usize, removed: &[usize], modified: &[usize], added: &[usize], field: &str) -> MutationApplyResult<()> {
    let mut removed_set = HashSet::new();
    for &index in removed {
        if index >= base_len {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed removal target does not exist").at(vec![field.to_string(), index.to_string()]));
        }
        if !removed_set.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed removal target is repeated").at(vec![field.to_string(), index.to_string()]));
        }
    }
    let mut modified_set = HashSet::new();
    for &index in modified {
        if index >= base_len {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "indexed modification target does not exist").at(vec![field.to_string(), index.to_string()]));
        }
        if removed_set.contains(&index) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "indexed modification targets a removed item").at(vec![field.to_string(), index.to_string()]));
        }
        if !modified_set.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed modification target is repeated").at(vec![field.to_string(), index.to_string()]));
        }
    }
    let final_len = base_len - removed_set.len() + added.len();
    let mut added_set = HashSet::new();
    for &index in added {
        if index >= final_len {
            return Err(MutationApplyError::new("mutation.apply.invalid-index", "indexed addition is outside the final collection").at(vec![field.to_string(), index.to_string()]));
        }
        if !added_set.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "indexed addition occupies a repeated final position").at(vec![field.to_string(), index.to_string()]));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn validate_named_keys<K: Eq + std::hash::Hash + Clone>(base: &[K], removed: &[K], modified: &[K], added: &[K], field: &str) -> MutationApplyResult<()> {
    let base_set: HashSet<K> = base.iter().cloned().collect();
    if base_set.len() != base.len() {
        return Err(MutationApplyError::new("mutation.apply.duplicate-target", "base collection contains duplicate keys").at([field]));
    }
    let mut removed_set = HashSet::new();
    for key in removed {
        if !base_set.contains(key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named removal target does not exist").at([field]));
        }
        if !removed_set.insert(key.clone()) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named removal target is repeated").at([field]));
        }
    }
    let mut modified_set = HashSet::new();
    for key in modified {
        if !base_set.contains(key) {
            return Err(MutationApplyError::new("mutation.apply.missing-target", "named modification target does not exist").at([field]));
        }
        if removed_set.contains(key) {
            return Err(MutationApplyError::new("mutation.apply.conflicting-target", "named modification targets a removed item").at([field]));
        }
        if !modified_set.insert(key.clone()) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named modification target is repeated").at([field]));
        }
    }
    let mut added_set = HashSet::new();
    for key in added {
        if base_set.contains(key) || !added_set.insert(key.clone()) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-target", "named addition target already exists").at([field]));
        }
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn validate_dict_diff(diff: &PdfDictDiff, base: &[PdfDictEntry]) -> MutationApplyResult<()> {
    validate_named_keys(
        &base.iter().map(|entry| entry.key.clone()).collect::<Vec<_>>(),
        &diff.removed,
        &diff.modified.iter().map(|item| item.key.clone()).collect::<Vec<_>>(),
        &diff.added.iter().map(|item| item.key.clone()).collect::<Vec<_>>(),
        "dict",
    )?;
    for modified in &diff.modified {
        let entry = base.iter().find(|entry| entry.key == modified.key).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "dictionary modification target does not exist").at(vec!["dict".to_string(), modified.key.clone()]))?;
        validate_value_diff(&modified.diff, &entry.value).map_err(|error| error.under(vec!["dict".to_string(), modified.key.clone()]))?;
    }
    let removed_indices: Vec<usize> = (0..diff.removed.len()).collect();
    validate_index_triple(base.len(), &removed_indices, &[], &diff.added.iter().map(|item| item.index).collect::<Vec<_>>(), "dict")
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_array_diff(diff: &PdfArrayDiff, base: &[PdfObject]) -> MutationApplyResult<()> {
    validate_index_triple(base.len(), &diff.removed, &diff.modified.iter().map(|item| item.index).collect::<Vec<_>>(), &diff.added.iter().map(|item| item.index).collect::<Vec<_>>(), "array")?;
    for modified in &diff.modified {
        let item = base.get(modified.index).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "array modification target does not exist").at(vec!["array".to_string(), modified.index.to_string()]))?;
        validate_value_diff(&modified.diff, item).map_err(|error| error.under(vec!["array".to_string(), modified.index.to_string()]))?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_value_diff(diff: &PdfValueDiff, base: &PdfObject) -> MutationApplyResult<()> {
    match diff {
        PdfValueDiff::Replace { .. } => Ok(()),
        PdfValueDiff::Bool { .. } if matches!(base, PdfObject::Bool(_)) => Ok(()),
        PdfValueDiff::Int { .. } if matches!(base, PdfObject::Int(_)) => Ok(()),
        PdfValueDiff::Real { .. } if matches!(base, PdfObject::Real(_)) => Ok(()),
        PdfValueDiff::Str { .. } if matches!(base, PdfObject::Str(_)) => Ok(()),
        PdfValueDiff::Name { .. } if matches!(base, PdfObject::Name(_)) => Ok(()),
        PdfValueDiff::Ref { .. } if matches!(base, PdfObject::Ref(_)) => Ok(()),
        PdfValueDiff::Array { diff } => match base {
            PdfObject::Array(items) => validate_array_diff(diff, items),
            _ => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "array diff targets a non-array value")),
        },
        PdfValueDiff::Dict { diff } => match base {
            PdfObject::Dict(entries) => validate_dict_diff(diff, entries),
            _ => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "dictionary diff targets a non-dictionary value")),
        },
        PdfValueDiff::Stream { dict, .. } => match base {
            PdfObject::Stream { dict: entries, .. } => {
                if let Some(dict) = dict {
                    validate_dict_diff(dict, entries)?;
                }
                Ok(())
            }
            _ => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "stream diff targets a non-stream value")),
        },
        _ => Err(MutationApplyError::new("mutation.apply.kind-mismatch", "scalar diff targets a value of another kind")),
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(super) fn validate_objects_diff(diff: &PdfObjectsDiff, base: &[PdfIndirectObject]) -> MutationApplyResult<()> {
    let keys: Vec<ObjRef> = base.iter().map(|object| object.id).collect();
    validate_named_keys(&keys, &diff.removed, &diff.modified.iter().map(|item| item.id).collect::<Vec<_>>(), &[], "objects")?;
    let retained_keys:Vec<_>=keys.into_iter().filter(|id|!diff.removed.contains(id)).collect();
    validate_named_keys(&retained_keys,&[],&[],&diff.added.iter().map(|item|item.id).collect::<Vec<_>>(),"objects")?;
    let removed_indices: Vec<usize> = (0..diff.removed.len()).collect();
    validate_index_triple(base.len(), &removed_indices, &[], &diff.added.iter().map(|item| item.index).collect::<Vec<_>>(), "objects")?;
    for modified in &diff.modified {
        let object = base.iter().find(|object| object.id == modified.id).ok_or_else(|| MutationApplyError::new("mutation.apply.missing-target", "object modification target does not exist").at(["objects"]))?;
        validate_value_diff(&modified.diff, &object.value).map_err(|error| error.under(["objects"]))?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
