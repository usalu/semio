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
) -> Vec<T> {
    let mut next = items.to_vec();
    for id in removed {
        next.retain(|item| id_of(item) != id);
    }
    for item in added {
        if let Some(pos) = next.iter().position(|entry| id_of(entry) == id_of(item)) {
            next[pos] = item.clone();
        } else {
            next.push(item.clone());
        }
    }
    for (id, replacement) in patched {
        if let (Some(pos), Some(value)) = (next.iter().position(|entry| id_of(entry) == id), replacement) {
            next[pos] = value.clone();
        }
    }
    if let Some(order) = reordered {
        let mut by_id: std::collections::BTreeMap<_, _> =
            next.into_iter().map(|item| (id_of(&item).to_string(), item)).collect();
        let mut ordered = Vec::with_capacity(order.len());
        for id in order {
            if let Some(item) = by_id.remove(id) {
                ordered.push(item);
            }
        }
        ordered.extend(by_id.into_values());
        next = ordered;
    }
    next
}

macro_rules! apply_col {
    ($fn:ident, $ty:ty, $delta:ty, $field:ident) => {
        pub fn $fn(items: &[$ty], delta: &$delta) -> Vec<$ty> {
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
    pub fn apply_to_artifact(&self, artifact: &Puzzle3dArtifact) -> Puzzle3dArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema { next.schema = schema.clone(); }
        if let Some(domain) = &self.domain { next.domain = domain.clone(); }
        if let Some(meta) = &self.meta { next.meta = meta.clone(); }
        if let Some(delta) = &self.objects { next.objects = apply_objects_delta(&next.objects, delta); }
        if let Some(delta) = &self.attractions { next.attractions = apply_attractions_delta(&next.attractions, delta); }
        if let Some(delta) = &self.target_volumes { next.target_volumes = apply_target_volumes_delta(&next.target_volumes, delta); }
        if let Some(delta) = &self.references { next.references = apply_references_delta(&next.references, delta); }
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
    }
}

impl MutationDiff<Puzzle3dSnapshot> for Puzzle3dDiff {
    fn apply(&self, snapshot: &Puzzle3dSnapshot) -> Puzzle3dSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema { next.schema = schema.clone(); }
        if let Some(domain) = &self.domain { next.domain = domain.clone(); }
        if let Some(meta) = &self.meta { next.meta = meta.clone(); }
        if let Some(delta) = &self.objects { next.objects = apply_objects_delta(&next.objects, delta); }
        if let Some(delta) = &self.attractions { next.attractions = apply_attractions_delta(&next.attractions, delta); }
        if let Some(delta) = &self.target_volumes { next.target_volumes = apply_target_volumes_delta(&next.target_volumes, delta); }
        if let Some(delta) = &self.references { next.references = apply_references_delta(&next.references, delta); }
        next
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

