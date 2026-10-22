//! 🏷️ The name-keyed group and object diffs and the aggregate `ObjDiff` with its validation, apply, inverse and absorb.

use super::*;

//#region 🔖️GroupDiff
/// 🔺️ Sparse patch for one [`ObjGroup`]/[`ObjObject`] — `faces` is a weak whole-list value
/// (membership set), replaced wholesale, never sub-diffed.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjGroupDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub faces: Option<Vec<u64>>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn group_diff_is_empty(d: &ObjGroupDiff) -> bool {
    d == &ObjGroupDiff::default()
}

/// 🧮 `ObjGroup` and `ObjObject` are structurally identical (`name`/`faces`) but distinct named
/// types per the recipe — this tiny local trait lets `apply_group_diff`/membership helpers work
/// over either without merging the two public types back into one shared type.
trait HasFaces {
    fn faces_mut(&mut self) -> &mut Vec<u64>;
}
impl HasFaces for ObjGroup {
    fn faces_mut(&mut self) -> &mut Vec<u64> {
        &mut self.faces
    }
}
impl HasFaces for ObjObject {
    fn faces_mut(&mut self) -> &mut Vec<u64> {
        &mut self.faces
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_group_diff<T: HasFaces>(item: &mut T, d: &ObjGroupDiff) {
    if let Some(f) = &d.faces {
        *item.faces_mut() = f.clone();
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_group_rows(base: &mut ObjGroupDiff, other: ObjGroupDiff) {
    if other.faces.is_some() {
        base.faces = other.faces;
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjGroupModified {
    pub name: String,
    pub diff: ObjGroupDiff,
}
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjGroupAdded {
    pub index: usize,
    pub group: ObjGroup,
}
/// 🔺️ Name-keyed removed/modified/added triple over `ObjSnapshot::groups` (same shape as
/// `stdio.zip`'s `ZipEntriesDiff` — no rename tracking needed here, `g` statements only ever
/// add/remove named groups, never rename one in place).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjGroupsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ObjGroupModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ObjGroupAdded>,
}
impl ObjGroupsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}

/// ▶️ Applies a name-keyed groups/objects triple in place (shared shape; parameterized over the
/// `name`/`faces` accessor pair since `ObjGroup`/`ObjObject` are structurally identical but
/// distinct named types per the recipe).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_named_membership<T: Clone>(base: &[T], removed: &[String], modified: &[(String, ObjGroupDiff)], added: &[(usize, T)], name_of: impl Fn(&T) -> &str, patch: impl Fn(&mut T, &ObjGroupDiff)) -> Vec<T> {
    let mut items = base.to_vec();
    for (name, d) in modified {
        for item in &mut items {
            if name_of(item) == name {
                patch(item, d);
            }
        }
    }
    let removed_set: HashSet<&str> = removed.iter().map(String::as_str).collect();
    items.retain(|it| !removed_set.contains(name_of(it)));
    let mut adds: Vec<&(usize, T)> = added.iter().collect();
    adds.sort_by_key(|(i, _)| *i);
    for (idx, item) in adds {
        items.insert(*idx, item.clone());
    }
    items
}

/// ➕️ Structural, total, base-free absorb for a name-keyed groups/objects triple — same
/// algorithm as `stdio.zip`'s `absorb_entries` minus rename tracking (φ is the identity on
/// names here since nothing renames a group/object in place).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn absorb_named_membership<T: Clone>(
    d1: ObjGroupsDiff,
    d2: ObjGroupsDiff,
    name_of: impl Fn(&T) -> &str,
    patch: impl Fn(&mut T, &ObjGroupDiff),
    added_item: impl Fn(&ObjGroupAdded) -> T,
    wrap_added: impl Fn(usize, T) -> ObjGroupAdded,
) -> Option<ObjGroupsDiff> {
    let added_names: HashSet<String> = d1.added.iter().map(|a| name_of(&added_item(a)).to_string()).collect();

    let mut merged_removed: Vec<String> = d1.removed;
    let mut annihilated: HashSet<String> = HashSet::new();
    for name in &d2.removed {
        if added_names.contains(name) {
            annihilated.insert(name.clone());
        } else if !merged_removed.contains(name) {
            merged_removed.push(name.clone());
        }
    }
    let mut d1_modified = d1.modified;
    d1_modified.retain(|m| !merged_removed.contains(&m.name));

    let mut merged_added: Vec<ObjGroupAdded> = d1.added.into_iter().filter(|a| !annihilated.contains(name_of(&added_item(a)))).collect();
    let mut merged_modified: Vec<ObjGroupModified> = d1_modified;

    for dm in &d2.modified {
        if added_names.contains(&dm.name) {
            if annihilated.contains(&dm.name) {
                continue;
            }
            if let Some(a) = merged_added.iter_mut().find(|a| name_of(&added_item(a)) == dm.name) {
                let mut item = added_item(a);
                patch(&mut item, &dm.diff);
                *a = wrap_added(a.index, item);
            }
        } else {
            if merged_removed.contains(&dm.name) {
                continue;
            }
            if let Some(existing) = merged_modified.iter_mut().find(|m| m.name == dm.name) {
                absorb_group_rows(&mut existing.diff, dm.diff.clone());
            } else {
                merged_modified.push(ObjGroupModified { name: dm.name.clone(), diff: dm.diff.clone() });
            }
        }
    }

    merged_added.extend(d2.added);
    let merged = ObjGroupsDiff { removed: merged_removed, modified: merged_modified, added: merged_added };
    if merged.is_empty() {
        None
    } else {
        Some(merged)
    }
}
//#endregion 🔖️GroupDiff

//#region 🔖️ObjectsDiff
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjObjectAdded {
    pub index: usize,
    pub object: ObjObject,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct ObjObjectsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<ObjGroupModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<ObjObjectAdded>,
}
impl ObjObjectsDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
}
//#endregion 🔖️ObjectsDiff

//#region 🔖️Diff
/// 🔺️ Diff for `stdio.obj`. `schema` is an identity field and never appears here.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.obj.diff")]
pub struct ObjDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub vertices: Option<ObjVerticesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub texcoords: Option<ObjTexCoordsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub normals: Option<ObjNormalsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub faces: Option<ObjFacesDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub groups: Option<ObjGroupsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<ObjObjectsDiff>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mtllib: Option<Option<String>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub usemtl: Option<Vec<ObjUsemtlRange>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub smoothing_groups: Option<Vec<ObjSmoothingRange>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unknown_statements: Option<Vec<ObjUnknownStatement>>,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_indexed_targets(base_len: usize, removed_indices: &[usize], modified_indices: impl IntoIterator<Item = usize>, added_indices: impl IntoIterator<Item = usize>, target: &str) -> MutationApplyResult<()> {
    let mut removed = BTreeSet::new();
    for &index in removed_indices {
        if index >= base_len || !removed.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-remove-index", "removal target must exist exactly once").at([target, &index.to_string()]));
        }
    }
    let mut modified = BTreeSet::new();
    for index in modified_indices {
        if index >= base_len || removed.contains(&index) || !modified.insert(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-modify-index", "modification target must exist exactly once and remain present").at([target, &index.to_string()]));
        }
    }
    let mut additions: Vec<usize> = added_indices.into_iter().collect();
    additions.sort_unstable();
    let mut previous = None;
    for (length, index) in (base_len - removed.len()..).zip(additions) {
        if index > length || previous == Some(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-add-index", "addition target must be unique and within the evolving sequence").at([target, &index.to_string()]));
        }
        previous = Some(index);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_named_targets<'a>(
    base_names: impl IntoIterator<Item = &'a str>,
    removed_names: impl IntoIterator<Item = &'a str>,
    modified_names: impl IntoIterator<Item = &'a str>,
    added: impl IntoIterator<Item = (usize, &'a str)>,
    target: &str,
) -> MutationApplyResult<()> {
    let mut base = BTreeSet::new();
    for name in base_names {
        if !base.insert(name) {
            return Err(MutationApplyError::new("mutation.apply.duplicate-base-target", "base names must be unique").at([target, name]));
        }
    }
    let mut removed = BTreeSet::new();
    for name in removed_names {
        if !base.contains(name) || !removed.insert(name) {
            return Err(MutationApplyError::new("mutation.apply.invalid-remove-target", "removal target must exist exactly once").at([target, name]));
        }
    }
    let mut modified = BTreeSet::new();
    for name in modified_names {
        if !base.contains(name) || removed.contains(name) || !modified.insert(name) {
            return Err(MutationApplyError::new("mutation.apply.invalid-modify-target", "modification target must exist exactly once and remain present").at([target, name]));
        }
    }
    let mut additions: Vec<(usize, &str)> = added.into_iter().collect();
    additions.sort_by_key(|(index, _)| *index);
    let mut added_names = BTreeSet::new();
    let mut previous = None;
    for (length, (index, name)) in (base.len() - removed.len()..).zip(additions) {
        if base.contains(name) || !added_names.insert(name) || index > length || previous == Some(index) {
            return Err(MutationApplyError::new("mutation.apply.invalid-add-target", "addition name and position must be unique and valid").at([target, name]));
        }
        previous = Some(index);
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn validate_obj_diff(diff: &ObjDiff, base: &ObjSnapshot) -> MutationApplyResult<()> {
    if let Some(value) = &diff.vertices {
        validate_indexed_targets(base.vertices.len(), &value.removed, value.modified.iter().map(|entry| entry.index), value.added.iter().map(|entry| entry.index), "vertices")?;
    }
    if let Some(value) = &diff.texcoords {
        validate_indexed_targets(base.texcoords.len(), &value.removed, value.modified.iter().map(|entry| entry.index), value.added.iter().map(|entry| entry.index), "texcoords")?;
    }
    if let Some(value) = &diff.normals {
        validate_indexed_targets(base.normals.len(), &value.removed, value.modified.iter().map(|entry| entry.index), value.added.iter().map(|entry| entry.index), "normals")?;
    }
    if let Some(value) = &diff.faces {
        validate_indexed_targets(base.faces.len(), &value.removed, value.modified.iter().map(|entry| entry.index), value.added.iter().map(|entry| entry.index), "faces")?;
    }
    if let Some(value) = &diff.groups {
        validate_named_targets(
            base.groups.iter().map(|entry| entry.name.as_str()),
            value.removed.iter().map(String::as_str),
            value.modified.iter().map(|entry| entry.name.as_str()),
            value.added.iter().map(|entry| (entry.index, entry.group.name.as_str())),
            "groups",
        )?;
    }
    if let Some(value) = &diff.objects {
        validate_named_targets(
            base.objects.iter().map(|entry| entry.name.as_str()),
            value.removed.iter().map(String::as_str),
            value.modified.iter().map(|entry| entry.name.as_str()),
            value.added.iter().map(|entry| (entry.index, entry.object.name.as_str())),
            "objects",
        )?;
    }
    Ok(())
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn apply_obj_diff_unchecked(diff: &ObjDiff, base: &ObjSnapshot) -> ObjSnapshot {
    ObjSnapshot {
        schema: base.schema.clone(),
        vertices: match &diff.vertices {
            Some(d) => d.apply(&base.vertices),
            None => base.vertices.clone(),
        },
        texcoords: match &diff.texcoords {
            Some(d) => d.apply(&base.texcoords),
            None => base.texcoords.clone(),
        },
        normals: match &diff.normals {
            Some(d) => d.apply(&base.normals),
            None => base.normals.clone(),
        },
        faces: match &diff.faces {
            Some(d) => d.apply(&base.faces),
            None => base.faces.clone(),
        },
        groups: match &diff.groups {
            Some(d) => {
                let modified: Vec<(String, ObjGroupDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
                let added: Vec<(usize, ObjGroup)> = d.added.iter().map(|a| (a.index, a.group.clone())).collect();
                apply_named_membership(&base.groups, &d.removed, &modified, &added, |g| g.name.as_str(), apply_group_diff)
            }
            None => base.groups.clone(),
        },
        objects: match &diff.objects {
            Some(d) => {
                let modified: Vec<(String, ObjGroupDiff)> = d.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
                let added: Vec<(usize, ObjObject)> = d.added.iter().map(|a| (a.index, a.object.clone())).collect();
                apply_named_membership(&base.objects, &d.removed, &modified, &added, |o| o.name.as_str(), apply_group_diff)
            }
            None => base.objects.clone(),
        },
        mtllib: diff.mtllib.clone().unwrap_or_else(|| base.mtllib.clone()),
        usemtl: diff.usemtl.clone().unwrap_or_else(|| base.usemtl.clone()),
        smoothing_groups: diff.smoothing_groups.clone().unwrap_or_else(|| base.smoothing_groups.clone()),
        unknown_statements: diff.unknown_statements.clone().unwrap_or_else(|| base.unknown_statements.clone()),
    }
}

impl MutationDiff<ObjSnapshot> for ObjDiff {}

impl protocol::ColdMutationDiff<ObjSnapshot> for ObjDiff {
    /// ➕️ Structural, total, base-free sequential-coalesce (`## Absorb` contract). Index-keyed
    /// collections use the label-simulation transport (`generic_absorb_pair`); name-keyed
    /// collections use `absorb_named_membership`; every other field is LWW.
    fn absorb(&mut self, other: Self) {
        self.vertices = match (self.vertices.take(), other.vertices) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => ObjVerticesDiff::absorb(a, b),
        };
        self.texcoords = match (self.texcoords.take(), other.texcoords) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => ObjTexCoordsDiff::absorb(a, b),
        };
        self.normals = match (self.normals.take(), other.normals) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => ObjNormalsDiff::absorb(a, b),
        };
        self.faces = match (self.faces.take(), other.faces) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => ObjFacesDiff::absorb(a, b),
        };
        self.groups = match (self.groups.take(), other.groups) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => absorb_named_membership(a, b, |g: &ObjGroup| g.name.as_str(), apply_group_diff, |added: &ObjGroupAdded| added.group.clone(), |index, group| ObjGroupAdded { index, group }),
        };
        self.objects = match (self.objects.take(), other.objects) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => {
                let a2 = ObjGroupsDiff { removed: a.removed, modified: a.modified, added: a.added.into_iter().map(|x| ObjGroupAdded { index: x.index, group: ObjGroup { name: x.object.name, faces: x.object.faces } }).collect() };
                let b2 = ObjGroupsDiff { removed: b.removed, modified: b.modified, added: b.added.into_iter().map(|x| ObjGroupAdded { index: x.index, group: ObjGroup { name: x.object.name, faces: x.object.faces } }).collect() };
                absorb_named_membership(a2, b2, |g: &ObjGroup| g.name.as_str(), apply_group_diff, |added: &ObjGroupAdded| added.group.clone(), |index, group| ObjGroupAdded { index, group }).map(|merged| ObjObjectsDiff {
                    removed: merged.removed,
                    modified: merged.modified,
                    added: merged.added.into_iter().map(|x| ObjObjectAdded { index: x.index, object: ObjObject { name: x.group.name, faces: x.group.faces } }).collect(),
                })
            }
        };
        if other.mtllib.is_some() {
            self.mtllib = other.mtllib;
        }
        if other.usemtl.is_some() {
            self.usemtl = other.usemtl;
        }
        if other.smoothing_groups.is_some() {
            self.smoothing_groups = other.smoothing_groups;
        }
        if other.unknown_statements.is_some() {
            self.unknown_statements = other.unknown_statements;
        }
    }

    fn apply(&self, unpublished: &mut ObjSnapshot, _capability: protocol::ApplyCapability) -> MutationApplyResult<()> {
        let base = &*unpublished;
        validate_obj_diff(self, base)?;
        *unpublished = apply_obj_diff_unchecked(self, base);
        Ok(())
    }
}

/// ↩️ Negative rows for the groups triple against its BASE groups; removed groups return at their base index.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_groups(diff: &ObjGroupsDiff, base: &[ObjGroup]) -> ObjGroupsDiff {
    let mut added_final: Vec<&ObjGroupAdded> = diff.added.iter().collect();
    added_final.sort_by_key(|added| added.index);
    let removed = added_final.iter().map(|added| added.group.name.clone()).collect();
    let modified = diff.modified.iter().filter_map(|row| base.iter().find(|group| group.name == row.name).map(|group| ObjGroupModified { name: row.name.clone(), diff: ObjGroupDiff { faces: row.diff.faces.as_ref().map(|_| group.faces.clone()) } })).collect();
    let mut restored: Vec<ObjGroupAdded> = diff.removed.iter().filter_map(|name| base.iter().position(|group| &group.name == name).map(|index| ObjGroupAdded { index, group: base[index].clone() })).collect();
    restored.sort_by_key(|added| added.index);
    ObjGroupsDiff { removed, modified, added: restored }
}

/// ↩️ Negative rows for the objects triple against its BASE objects; removed objects return at their base index.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn inverse_objects(diff: &ObjObjectsDiff, base: &[ObjObject]) -> ObjObjectsDiff {
    let mut added_final: Vec<&ObjObjectAdded> = diff.added.iter().collect();
    added_final.sort_by_key(|added| added.index);
    let removed = added_final.iter().map(|added| added.object.name.clone()).collect();
    let modified = diff.modified.iter().filter_map(|row| base.iter().find(|object| object.name == row.name).map(|object| ObjGroupModified { name: row.name.clone(), diff: ObjGroupDiff { faces: row.diff.faces.as_ref().map(|_| object.faces.clone()) } })).collect();
    let mut restored: Vec<ObjObjectAdded> = diff.removed.iter().filter_map(|name| base.iter().position(|object| &object.name == name).map(|index| ObjObjectAdded { index, object: base[index].clone() })).collect();
    restored.sort_by_key(|added| added.index);
    ObjObjectsDiff { removed, modified, added: restored }
}

impl DiffAlgebra<ObjSnapshot> for ObjDiff {
    fn is_empty(&self) -> bool {
        self.vertices.as_ref().is_none_or(ObjVerticesDiff::is_empty)
            && self.texcoords.as_ref().is_none_or(ObjTexCoordsDiff::is_empty)
            && self.normals.as_ref().is_none_or(ObjNormalsDiff::is_empty)
            && self.faces.as_ref().is_none_or(ObjFacesDiff::is_empty)
            && self.groups.as_ref().is_none_or(ObjGroupsDiff::is_empty)
            && self.objects.as_ref().is_none_or(ObjObjectsDiff::is_empty)
            && self.mtllib.is_none()
            && self.usemtl.is_none()
            && self.smoothing_groups.is_none()
            && self.unknown_statements.is_none()
    }
}

impl protocol::ColdDiffAlgebra<ObjSnapshot> for ObjDiff {
    /// ↩️ Concrete diff-level undo: every index-keyed collection turns into its negative rows (removed rows return at their base
    /// index), the name-keyed groups and objects restore their base membership and return at their base index, and the replaced
    /// scalars return to their base values.
    fn inverse(&self, base: &ObjSnapshot) -> Self {
        ObjDiff {
            vertices: self.vertices.as_ref().and_then(|d| d.inverse(&base.vertices)),
            texcoords: self.texcoords.as_ref().and_then(|d| d.inverse(&base.texcoords)),
            normals: self.normals.as_ref().and_then(|d| d.inverse(&base.normals)),
            faces: self.faces.as_ref().and_then(|d| d.inverse(&base.faces)),
            groups: self.groups.as_ref().map(|d| inverse_groups(d, &base.groups)).filter(|d| !d.is_empty()),
            objects: self.objects.as_ref().map(|d| inverse_objects(d, &base.objects)).filter(|d| !d.is_empty()),
            mtllib: self.mtllib.as_ref().map(|_| base.mtllib.clone()),
            usemtl: self.usemtl.as_ref().map(|_| base.usemtl.clone()),
            smoothing_groups: self.smoothing_groups.as_ref().map(|_| base.smoothing_groups.clone()),
            unknown_statements: self.unknown_statements.as_ref().map(|_| base.unknown_statements.clone()),
        }
    }
}

//#endregion 🔖️Diff
