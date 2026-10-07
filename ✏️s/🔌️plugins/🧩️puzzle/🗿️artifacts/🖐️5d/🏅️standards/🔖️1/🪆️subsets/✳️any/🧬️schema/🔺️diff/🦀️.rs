//! 🧬️ Puzzle5d diff schema — sparse field delta over the artifact.

use crate::standards::v1::subsets::any::schema::Puzzle5dArtifact;
use crate::{Puzzle5dFastener, Puzzle5dKindCatalogsExtra, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, Puzzle5dTargetVolume};
use ::semio_framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the puzzle5d artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle5d")]
pub struct Puzzle5dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<Puzzle5dArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub domain: Option<String>,
    #[state(artifact)]
    pub label: Option<Option<String>>,
    #[state(artifact)]
    pub meta: Option<Puzzle5dMeta>,
    #[child(kind = "s.stdio.semio")]
    #[state(artifact)]
    pub kind_catalogs: Option<Option<store::ArtifactChild<SemioKitSnapshot>>>,
    #[state(artifact)]
    pub kind_catalogs_extra: Option<Option<Puzzle5dKindCatalogsExtra>>,
    #[state(artifact)]
    pub kind_compatibility: Option<Puzzle5dKindCompatibilityList>,
    #[state(artifact)]
    pub parts: Option<Puzzle5dPartsDelta>,
    #[state(artifact)]
    pub fasteners: Option<Puzzle5dFastenersDelta>,
    #[state(artifact)]
    pub target_volumes: Option<Puzzle5dTargetVolumesDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers

/// 📋 Kind-compatibility list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dKindCompatibilityList {
    pub values: Vec<Puzzle5dKindCompatibility>,
}

/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `parts`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dPartsDelta {
    pub added: Vec<Puzzle5dPart>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle5dPartPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle5dPart` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPartPatchEntry {
    pub id: String,
    pub patch: Puzzle5dPartPatch,
}

/// 🩹 Sparse patch over `Puzzle5dPart` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dPartPatch {
    pub replacement: Option<Puzzle5dPart>,
}

/// 🧩 Identified-collection delta for `fasteners`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dFastenersDelta {
    pub added: Vec<Puzzle5dFastener>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle5dFastenerPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle5dFastener` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dFastenerPatchEntry {
    pub id: String,
    pub patch: Puzzle5dFastenerPatch,
}

/// 🩹 Sparse patch over `Puzzle5dFastener` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dFastenerPatch {
    pub replacement: Option<Puzzle5dFastener>,
}

/// 🧩 Identified-collection delta for `target_volumes`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dTargetVolumesDelta {
    pub added: Vec<Puzzle5dTargetVolume>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle5dTargetVolumePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle5dTargetVolume` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dTargetVolumePatchEntry {
    pub id: String,
    pub patch: Puzzle5dTargetVolumePatch,
}

/// 🩹 Sparse patch over `Puzzle5dTargetVolume` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dTargetVolumePatch {
    pub replacement: Option<Puzzle5dTargetVolume>,
}

//#endregion 🔖️DeltaHelpers

use crate::Puzzle5dSnapshot;
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

pub fn apply_parts_delta(parts: &[Puzzle5dPart], delta: &Puzzle5dPartsDelta) -> protocol::MutationApplyResult<Vec<Puzzle5dPart>> {
    let patched: Vec<_> = delta.patched.iter().map(|entry| (entry.id.clone(), entry.patch.replacement.clone())).collect();
    apply_identified_delta(parts, &delta.removed, &delta.added, &patched, &delta.reordered, |p| &p.id)
}

pub fn apply_fasteners_delta(fasteners: &[Puzzle5dFastener], delta: &Puzzle5dFastenersDelta) -> protocol::MutationApplyResult<Vec<Puzzle5dFastener>> {
    let patched: Vec<_> = delta.patched.iter().map(|entry| (entry.id.clone(), entry.patch.replacement.clone())).collect();
    apply_identified_delta(fasteners, &delta.removed, &delta.added, &patched, &delta.reordered, |f| &f.id)
}

pub fn apply_target_volumes_delta(target_volumes: &[Puzzle5dTargetVolume], delta: &Puzzle5dTargetVolumesDelta) -> protocol::MutationApplyResult<Vec<Puzzle5dTargetVolume>> {
    let patched: Vec<_> = delta.patched.iter().map(|entry| (entry.id.clone(), entry.patch.replacement.clone())).collect();
    apply_identified_delta(target_volumes, &delta.removed, &delta.added, &patched, &delta.reordered, |v| &v.id)
}

impl Puzzle5dDiff {
    /// 🧬️ Applies every sparse entry onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &Puzzle5dArtifact) -> protocol::MutationApplyResult<Puzzle5dArtifact> {
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
            if let Some(label) = &self.label {
                next.label = label.clone();
            }
            if let Some(meta) = &self.meta {
                next.meta = meta.clone();
            }
            if let Some(catalogs) = &self.kind_catalogs {
                next.kind_catalogs = catalogs.clone();
            }
            if let Some(extra) = &self.kind_catalogs_extra {
                next.kind_catalogs_extra = extra.clone();
            }
            if let Some(list) = &self.kind_compatibility {
                next.kind_compatibility = list.values.clone();
            }
            if let Some(delta) = &self.parts {
                next.parts = apply_parts_delta(&next.parts, delta).map_err(|error| error.under(["parts"]))?;
            }
            if let Some(delta) = &self.fasteners {
                next.fasteners = apply_fasteners_delta(&next.fasteners, delta).map_err(|error| error.under(["fasteners"]))?;
            }
            if let Some(delta) = &self.target_volumes {
                next.target_volumes = apply_target_volumes_delta(&next.target_volumes, delta).map_err(|error| error.under(["targetVolumes"]))?;
            }
            next
        })
    }
}

impl MutationDiff<Puzzle5dSnapshot> for Puzzle5dDiff {
    fn apply(&self, snapshot: &Puzzle5dSnapshot) -> protocol::MutationApplyResult<Puzzle5dSnapshot> {
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
            if let Some(label) = &self.label {
                next.label = label.clone();
            }
            if let Some(meta) = &self.meta {
                next.meta = meta.clone();
            }
            if let Some(catalogs) = &self.kind_catalogs {
                next.kind_catalogs = catalogs.clone();
            }
            if let Some(extra) = &self.kind_catalogs_extra {
                next.kind_catalogs_extra = extra.clone();
            }
            if let Some(list) = &self.kind_compatibility {
                next.kind_compatibility = list.values.clone();
            }
            if let Some(delta) = &self.parts {
                next.parts = apply_parts_delta(&next.parts, delta).map_err(|error| error.under(["parts"]))?;
            }
            if let Some(delta) = &self.fasteners {
                next.fasteners = apply_fasteners_delta(&next.fasteners, delta).map_err(|error| error.under(["fasteners"]))?;
            }
            if let Some(delta) = &self.target_volumes {
                next.target_volumes = apply_target_volumes_delta(&next.target_volumes, delta).map_err(|error| error.under(["targetVolumes"]))?;
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
        take!(label);
        take!(meta);
        take!(kind_catalogs);
        take!(kind_catalogs_extra);
        take!(kind_compatibility);
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
        merge_delta!(parts);
        merge_delta!(fasteners);
        merge_delta!(target_volumes);
    }
}
