//! 🔺️ Puzzle 3d artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::puzzle3d::schema::diff::{
    Puzzle3dAttractionsDelta, Puzzle3dDiff, Puzzle3dObjectsDelta, Puzzle3dReferencesDelta,
    Puzzle3dStringList, Puzzle3dTargetVolumesDelta,
};
use crate::artifacts::puzzle3d::schema::Puzzle3dArtifact;
use crate::artifacts::puzzle3d::{
    Puzzle3dAttraction, Puzzle3dObject, Puzzle3dReference, Puzzle3dSnapshot, Puzzle3dTargetVolume,
};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar


//#region 🔖️Apply
fn apply_identified_delta<T: Clone>(
    items: &[T],
    removed: &[String],
    added: &[T],
    patched: &[(String, Option<T>)],
    reordered: &Option<Vec<String>>,
    id_of: impl Fn(&T) -> &str,
) -> protocol::MutationApplyResult<Vec<T>> {
    let mut next = items.to_vec();
    let mut seen = std::collections::HashSet::new();
    for id in removed {
        if !seen.insert(id.clone()) {
            return Err(protocol::MutationApplyError::new("mutation.apply.duplicate-target", "item is removed more than once").at(["removed", id.as_str()]));
        }
        let position = next.iter().position(|item| id_of(item) == id).ok_or_else(|| {
            protocol::MutationApplyError::new("mutation.apply.missing-target", "removed item does not exist").at(["removed", id.as_str()])
        })?;
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
        let position = next.iter().position(|entry| id_of(entry) == id).ok_or_else(|| {
            protocol::MutationApplyError::new("mutation.apply.missing-target", "patched item does not exist").at(["patched", id.as_str()])
        })?;
        let value = replacement.as_ref().ok_or_else(|| {
            protocol::MutationApplyError::new("mutation.apply.incomplete-diff", "item patch has no replacement").at(["patched", id.as_str()])
        })?;
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
            let position = next.iter().position(|entry| id_of(entry) == id).ok_or_else(|| {
                protocol::MutationApplyError::new("mutation.apply.missing-target", "ordered item does not exist").at(["reordered", id.as_str()])
            })?;
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
            if let Some(schema) = &self.schema { next.schema = schema.clone(); }
            if let Some(domain) = &self.domain { next.domain = domain.clone(); }
            if let Some(meta) = &self.meta { next.meta = meta.clone(); }
            if let Some(delta) = &self.objects { next.objects = apply_objects_delta(&next.objects, delta).map_err(|error| error.under(["objects"]))?; }
            if let Some(delta) = &self.attractions { next.attractions = apply_attractions_delta(&next.attractions, delta).map_err(|error| error.under(["attractions"]))?; }
            if let Some(delta) = &self.target_volumes { next.target_volumes = apply_target_volumes_delta(&next.target_volumes, delta).map_err(|error| error.under(["targetVolumes"]))?; }
            if let Some(delta) = &self.references { next.references = apply_references_delta(&next.references, delta).map_err(|error| error.under(["references"]))?; }
            if let Some(list) = &self.selected_object_ids { next.selected_object_ids = list.values.clone(); }
            if let Some(list) = &self.selected_vortex_ids { next.selected_vortex_ids = list.values.clone(); }
            if let Some(list) = &self.selected_attraction_ids { next.selected_attraction_ids = list.values.clone(); }
            if let Some(list) = &self.selected_target_volume_ids { next.selected_target_volume_ids = list.values.clone(); }
            if let Some(list) = &self.selected_reference_ids { next.selected_reference_ids = list.values.clone(); }
            if let Some(value) = &self.active_utility_id { next.active_utility_id = value.clone(); }
            if let Some(value) = self.camera_position_x { next.camera_position_x = value; }
            if let Some(value) = self.camera_position_y { next.camera_position_y = value; }
            if let Some(value) = self.camera_position_z { next.camera_position_z = value; }
            if let Some(value) = self.camera_target_x { next.camera_target_x = value; }
            if let Some(value) = self.camera_target_y { next.camera_target_y = value; }
            if let Some(value) = self.camera_target_z { next.camera_target_z = value; }
            if let Some(value) = self.camera_zoom { next.camera_zoom = value; }
            if let Some(value) = &self.selection_method { next.selection_method = value.clone(); }
            if let Some(value) = &self.selection_mode_default { next.selection_mode_default = value.clone(); }
            if let Some(value) = &self.engagement_input { next.engagement_input = value.clone(); }
            if let Some(value) = self.grid_visible { next.grid_visible = value; }
            if let Some(value) = self.grid_snap_enabled { next.grid_snap_enabled = value; }
            if let Some(value) = self.grid_spacing { next.grid_spacing = value; }
            if let Some(value) = self.overlap_budget { next.overlap_budget = value; }
            if let Some(value) = self.fill_count { next.fill_count = value; }
            if let Some(value) = self.brush_candidate_index { next.brush_candidate_index = value; }
            if let Some(value) = self.lod_automatic { next.lod_automatic = value; }
            if let Some(value) = self.lod_depth_variable { next.lod_depth_variable = value; }
            if let Some(value) = self.lod_manual { next.lod_manual = value; }
            if let Some(value) = self.proximity_radius { next.proximity_radius = value; }
            if let Some(value) = &self.locale { next.locale = value.clone(); }
            if let Some(value) = &self.runtime_extras_json { next.runtime_extras_json = value.clone(); }
            if let Some(value) = &self.hovered_object_id { next.hovered_object_id = value.clone(); }
            if let Some(value) = &self.hovered_vortex_full_id { next.hovered_vortex_full_id = value.clone(); }
            if let Some(value) = &self.hovered_kind_id { next.hovered_kind_id = value.clone(); }
            if let Some(value) = self.preview_seq { next.preview_seq = value; }
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
            if let Some(schema) = &self.schema { next.schema = schema.clone(); }
            if let Some(domain) = &self.domain { next.domain = domain.clone(); }
            if let Some(meta) = &self.meta { next.meta = meta.clone(); }
            if let Some(delta) = &self.objects { next.objects = apply_objects_delta(&next.objects, delta).map_err(|error| error.under(["objects"]))?; }
            if let Some(delta) = &self.attractions { next.attractions = apply_attractions_delta(&next.attractions, delta).map_err(|error| error.under(["attractions"]))?; }
            if let Some(delta) = &self.target_volumes { next.target_volumes = apply_target_volumes_delta(&next.target_volumes, delta).map_err(|error| error.under(["targetVolumes"]))?; }
            if let Some(delta) = &self.references { next.references = apply_references_delta(&next.references, delta).map_err(|error| error.under(["references"]))?; }
            next
        })
    }
    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() { *self = other; return; }
        macro_rules! take { ($f:ident) => { if other.$f.is_some() { self.$f = other.$f; } }; }
        take!(schema); take!(domain); take!(meta);
        take!(selected_object_ids); take!(selected_vortex_ids); take!(selected_attraction_ids);
        take!(selected_target_volume_ids); take!(selected_reference_ids); take!(active_utility_id);
        take!(camera_position_x); take!(camera_position_y); take!(camera_position_z);
        take!(camera_target_x); take!(camera_target_y); take!(camera_target_z); take!(camera_zoom);
        take!(selection_method); take!(selection_mode_default); take!(engagement_input);
        take!(grid_visible); take!(grid_snap_enabled); take!(grid_spacing); take!(overlap_budget);
        take!(fill_count); take!(brush_candidate_index); take!(lod_automatic); take!(lod_depth_variable);
        take!(lod_manual); take!(proximity_radius); take!(locale); take!(runtime_extras_json);
        take!(hovered_object_id); take!(hovered_vortex_full_id); take!(hovered_kind_id); take!(preview_seq);
        macro_rules! merge_delta {
            ($field:ident) => {
                if let Some(delta) = other.$field {
                    match &mut self.$field {
                        Some(existing) => {
                            existing.removed.extend(delta.removed);
                            existing.added.extend(delta.added);
                            existing.patched.extend(delta.patched);
                            if delta.reordered.is_some() { existing.reordered = delta.reordered; }
                        }
                        None => self.$field = Some(delta),
                    }
                }
            };
        }
        merge_delta!(objects); merge_delta!(attractions); merge_delta!(target_volumes); merge_delta!(references);
    }
}
//#endregion 🔖️Apply
