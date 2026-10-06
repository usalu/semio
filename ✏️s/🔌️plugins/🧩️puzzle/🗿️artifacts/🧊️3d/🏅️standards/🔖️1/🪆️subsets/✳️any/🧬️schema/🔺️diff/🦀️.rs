//! 🧬️ Puzzle3d diff schema — sparse field delta over the artifact.

use crate::standards::v1::subsets::any::schema::Puzzle3dArtifact;
use crate::{Puzzle3dAttraction, Puzzle3dMeta, Puzzle3dObject, Puzzle3dReference, Puzzle3dTargetVolume};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the puzzle3d artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle3d")]
pub struct Puzzle3dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<Puzzle3dArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub domain: Option<String>,
    #[state(artifact)]
    pub meta: Option<Puzzle3dMeta>,
    #[state(artifact)]
    pub objects: Option<Puzzle3dObjectsDelta>,
    #[state(artifact)]
    pub attractions: Option<Puzzle3dAttractionsDelta>,
    #[state(artifact)]
    pub target_volumes: Option<Puzzle3dTargetVolumesDelta>,
    #[state(artifact)]
    pub references: Option<Puzzle3dReferencesDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `objects`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dObjectsDelta {
    pub added: Vec<Puzzle3dObject>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle3dObjectPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle3dObject` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dObjectPatchEntry {
    pub id: String,
    pub patch: Puzzle3dObjectPatch,
}

/// 🩹 Sparse patch over `Puzzle3dObject` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dObjectPatch {
    pub replacement: Option<Puzzle3dObject>,
}

/// 🧩 Identified-collection delta for `attractions`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dAttractionsDelta {
    pub added: Vec<Puzzle3dAttraction>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle3dAttractionPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle3dAttraction` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dAttractionPatchEntry {
    pub id: String,
    pub patch: Puzzle3dAttractionPatch,
}

/// 🩹 Sparse patch over `Puzzle3dAttraction` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dAttractionPatch {
    pub replacement: Option<Puzzle3dAttraction>,
}

/// 🧩 Identified-collection delta for `targetVolumes`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dTargetVolumesDelta {
    pub added: Vec<Puzzle3dTargetVolume>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle3dTargetVolumePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle3dTargetVolume` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dTargetVolumePatchEntry {
    pub id: String,
    pub patch: Puzzle3dTargetVolumePatch,
}

/// 🩹 Sparse patch over `Puzzle3dTargetVolume` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dTargetVolumePatch {
    pub replacement: Option<Puzzle3dTargetVolume>,
}

/// 🧩 Identified-collection delta for `references`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dReferencesDelta {
    pub added: Vec<Puzzle3dReference>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle3dReferencePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle3dReference` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dReferencePatchEntry {
    pub id: String,
    pub patch: Puzzle3dReferencePatch,
}

/// 🩹 Sparse patch over `Puzzle3dReference` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dReferencePatch {
    pub replacement: Option<Puzzle3dReference>,
}

//#endregion 🔖️DeltaHelpers

use crate::Puzzle3dSnapshot;
use protocol::MutationDiff;

fn apply_identified_delta<T: Clone>(items: &[T], removed: &[String], added: &[T], patched: &[(String, Option<T>)], reordered: &Option<Vec<String>>, id_of: impl Fn(&T) -> &str) -> protocol::MutationApplyResult<Vec<T>> {
    let mut next = items.to_vec();
    let mut seen = std::collections::HashSet::new();
    for id in removed {
        if !seen.insert(id.clone()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "item is removed more than once").at(["removed", id.as_str()]));
        }
        let position = next.iter().position(|item| id_of(item) == id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "removed item does not exist").at(["removed", id.as_str()]))?;
        next.remove(position);
    }
    seen.clear();
    for item in added {
        let id = id_of(item);
        if !seen.insert(id.to_string()) || next.iter().any(|entry| id_of(entry) == id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "added item identity already exists").at(["added", id]));
        }
        next.push(item.clone());
    }
    seen.clear();
    for (id, replacement) in patched {
        if !seen.insert(id.clone()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "item is patched more than once").at(["patched", id.as_str()]));
        }
        let position = next.iter().position(|entry| id_of(entry) == id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "patched item does not exist").at(["patched", id.as_str()]))?;
        let value = replacement.as_ref().ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.incomplete-diff", "item patch has no replacement").at(["patched", id.as_str()]))?;
        let replacement_id = id_of(value);
        if replacement_id != id && next.iter().enumerate().any(|(index, entry)| index != position && id_of(entry) == replacement_id) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "patched item identity already exists").at(["patched", replacement_id]));
        }
        next[position] = value.clone();
    }
    if let Some(order) = reordered {
        if order.len() != next.len() {
            return Err(protocol::MutationApplyError::new("mutation.apply.incomplete-diff", format!("order has length {}, expected {}", order.len(), next.len())).at(["reordered"]));
        }
        seen.clear();
        for id in order {
            if !seen.insert(id.clone()) {
                return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "item appears more than once in order").at(["reordered", id.as_str()]));
            }
            if !next.iter().any(|entry| id_of(entry) == id) {
                return Err(protocol::MutationApplyError::new("mutation.apply.missing-target", "ordered item does not exist").at(["reordered", id.as_str()]));
            }
        }
        let mut ordered = Vec::with_capacity(next.len());
        for id in order {
            let position = next.iter().position(|entry| id_of(entry) == id).ok_or_else(|| protocol::MutationApplyError::new("mutation.apply.missing-target", "ordered item does not exist").at(["reordered", id.as_str()]))?;
            ordered.push(next.remove(position));
        }
        next = ordered;
    }
    Ok(next)
}

macro_rules! apply_col {
    ($fn:ident, $ty:ty, $delta:ty, $field:ident) => {
        pub fn $fn(items: &[$ty], delta: &$delta) -> protocol::MutationApplyResult<Vec<$ty>> {
            let patched: Vec<_> = delta.patched.iter().map(|entry| (entry.id.clone(), entry.patch.replacement.clone())).collect();
            apply_identified_delta(items, &delta.removed, &delta.added, &patched, &delta.reordered, |item| &item.id)
        }
    };
}

apply_col!(apply_objects_delta, Puzzle3dObject, Puzzle3dObjectsDelta, objects);

apply_col!(apply_attractions_delta, Puzzle3dAttraction, Puzzle3dAttractionsDelta, attractions);

apply_col!(apply_target_volumes_delta, Puzzle3dTargetVolume, Puzzle3dTargetVolumesDelta, target_volumes);

apply_col!(apply_references_delta, Puzzle3dReference, Puzzle3dReferencesDelta, references);

impl Puzzle3dDiff {
    /// 🧬️ Applies every sparse entry onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &Puzzle3dArtifact) -> protocol::MutationApplyResult<Puzzle3dArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(domain) = &self.domain {
                next.domain = domain.clone();
            }
            if let Some(meta) = &self.meta {
                next.meta = meta.clone();
            }
            if let Some(delta) = &self.objects {
                next.objects = apply_objects_delta(&next.objects, delta).map_err(|error| error.under(["objects"]))?;
            }
            if let Some(delta) = &self.attractions {
                next.attractions = apply_attractions_delta(&next.attractions, delta).map_err(|error| error.under(["attractions"]))?;
            }
            if let Some(delta) = &self.target_volumes {
                next.target_volumes = apply_target_volumes_delta(&next.target_volumes, delta).map_err(|error| error.under(["targetVolumes"]))?;
            }
            if let Some(delta) = &self.references {
                next.references = apply_references_delta(&next.references, delta).map_err(|error| error.under(["references"]))?;
            }
            next
        })
    }
}

impl MutationDiff<Puzzle3dSnapshot> for Puzzle3dDiff {
    fn apply(&self, snapshot: &Puzzle3dSnapshot) -> protocol::MutationApplyResult<Puzzle3dSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(domain) = &self.domain {
                next.domain = domain.clone();
            }
            if let Some(meta) = &self.meta {
                next.meta = meta.clone();
            }
            if let Some(delta) = &self.objects {
                next.objects = apply_objects_delta(&next.objects, delta).map_err(|error| error.under(["objects"]))?;
            }
            if let Some(delta) = &self.attractions {
                next.attractions = apply_attractions_delta(&next.attractions, delta).map_err(|error| error.under(["attractions"]))?;
            }
            if let Some(delta) = &self.target_volumes {
                next.target_volumes = apply_target_volumes_delta(&next.target_volumes, delta).map_err(|error| error.under(["targetVolumes"]))?;
            }
            if let Some(delta) = &self.references {
                next.references = apply_references_delta(&next.references, delta).map_err(|error| error.under(["references"]))?;
            }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($f:ident) => {
                if other.$f.is_some() {
                    self.$f = other.$f;
                }
            };
        }
        take!(schema);
        take!(domain);
        take!(meta);
        macro_rules! merge_delta {
            ($field:ident) => {
                if let Some(delta) = other.$field {
                    match &mut self.$field {
                        Some(existing) => {
                            existing.removed.extend(delta.removed);
                            existing.added.extend(delta.added);
                            for patch in delta.patched {
                                if let Some(previous) = existing.patched.iter_mut().find(|entry| entry.id == patch.id) {
                                    *previous = patch;
                                } else {
                                    existing.patched.push(patch);
                                }
                            }
                            if delta.reordered.is_some() {
                                existing.reordered = delta.reordered;
                            }
                        }
                        None => self.$field = Some(delta),
                    }
                }
            };
        }
        merge_delta!(objects);
        merge_delta!(attractions);
        merge_delta!(target_volumes);
        merge_delta!(references);
    }
}
