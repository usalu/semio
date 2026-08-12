//! 🔺️ Puzzle 5d artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::puzzle5d::schema::diff::{
    Puzzle5dDiff, Puzzle5dFastenersDelta, Puzzle5dKindCompatibilityList, Puzzle5dPartsDelta, Puzzle5dStringList,
};
use crate::artifacts::puzzle5d::schema::Puzzle5dArtifact;
use crate::artifacts::puzzle5d::{Puzzle5dFastener, Puzzle5dPart, Puzzle5dSnapshot};
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

pub fn apply_parts_delta(parts: &[Puzzle5dPart], delta: &Puzzle5dPartsDelta) -> Vec<Puzzle5dPart> {
    let patched: Vec<_> = delta.patched.iter().map(|entry| (entry.id.clone(), entry.patch.replacement.clone())).collect();
    apply_identified_delta(parts, &delta.removed, &delta.added, &patched, &delta.reordered, |p| &p.id)
}

pub fn apply_fasteners_delta(fasteners: &[Puzzle5dFastener], delta: &Puzzle5dFastenersDelta) -> Vec<Puzzle5dFastener> {
    let patched: Vec<_> = delta.patched.iter().map(|entry| (entry.id.clone(), entry.patch.replacement.clone())).collect();
    apply_identified_delta(fasteners, &delta.removed, &delta.added, &patched, &delta.reordered, |f| &f.id)
}

impl Puzzle5dDiff {
    /// 🧬️ Applies every sparse entry onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &Puzzle5dArtifact) -> Puzzle5dArtifact {
        if let Some(replacement) = &self.artifact {
            return (**replacement).clone();
        }
        let mut next = artifact.clone();
        if let Some(schema) = &self.schema { next.schema = schema.clone(); }
        if let Some(domain) = &self.domain { next.domain = domain.clone(); }
        if let Some(label) = &self.label { next.label = label.clone(); }
        if let Some(meta) = &self.meta { next.meta = meta.clone(); }
        if let Some(catalogs) = &self.kind_catalogs { next.kind_catalogs = catalogs.clone(); }
        if let Some(list) = &self.kind_compatibility { next.kind_compatibility = list.values.clone(); }
        if let Some(delta) = &self.parts { next.parts = apply_parts_delta(&next.parts, delta); }
        if let Some(delta) = &self.fasteners { next.fasteners = apply_fasteners_delta(&next.fasteners, delta); }
        if let Some(list) = &self.selected_part_ids { next.selected_part_ids = list.values.clone(); }
        if let Some(list) = &self.selected_grip_ids { next.selected_grip_ids = list.values.clone(); }
        if let Some(list) = &self.selected_fastener_ids { next.selected_fastener_ids = list.values.clone(); }
        if let Some(value) = &self.active_utility_id { next.active_utility_id = value.clone(); }
        if let Some(value) = self.camera2d_x { next.camera2d_x = value; }
        if let Some(value) = self.camera2d_y { next.camera2d_y = value; }
        if let Some(value) = self.camera2d_zoom { next.camera2d_zoom = value; }
        if let Some(value) = self.camera3d_position_x { next.camera3d_position_x = value; }
        if let Some(value) = self.camera3d_position_y { next.camera3d_position_y = value; }
        if let Some(value) = self.camera3d_position_z { next.camera3d_position_z = value; }
        if let Some(value) = self.camera3d_target_x { next.camera3d_target_x = value; }
        if let Some(value) = self.camera3d_target_y { next.camera3d_target_y = value; }
        if let Some(value) = self.camera3d_target_z { next.camera3d_target_z = value; }
        if let Some(value) = self.camera3d_zoom { next.camera3d_zoom = value; }
        if let Some(value) = &self.selection_method { next.selection_method = value.clone(); }
        if let Some(value) = self.grid_snap_enabled { next.grid_snap_enabled = value; }
        if let Some(value) = self.grid_factor { next.grid_factor = value; }
        if let Some(value) = self.suggestion_offset { next.suggestion_offset = value; }
        if let Some(value) = self.overlap_budget { next.overlap_budget = value; }
        if let Some(value) = self.fill_count { next.fill_count = value; }
        if let Some(value) = self.brush_candidate_index { next.brush_candidate_index = value; }
        if let Some(value) = &self.lod_mode { next.lod_mode = value.clone(); }
        if let Some(value) = &self.locale { next.locale = value.clone(); }
        if let Some(value) = &self.runtime_extras_json { next.runtime_extras_json = value.clone(); }
        if let Some(value) = &self.hovered_part_id { next.hovered_part_id = value.clone(); }
        if let Some(value) = self.preview_seq { next.preview_seq = value; }
        next
    }
}

impl MutationDiff<Puzzle5dSnapshot> for Puzzle5dDiff {
    fn apply(&self, snapshot: &Puzzle5dSnapshot) -> Puzzle5dSnapshot {
        if let Some(replacement) = &self.artifact {
            return replacement.to_snapshot();
        }
        let mut next = snapshot.clone();
        if let Some(schema) = &self.schema { next.schema = schema.clone(); }
        if let Some(domain) = &self.domain { next.domain = domain.clone(); }
        if let Some(label) = &self.label { next.label = label.clone(); }
        if let Some(meta) = &self.meta { next.meta = meta.clone(); }
        if let Some(catalogs) = &self.kind_catalogs { next.kind_catalogs = catalogs.clone(); }
        if let Some(list) = &self.kind_compatibility { next.kind_compatibility = list.values.clone(); }
        if let Some(delta) = &self.parts { next.parts = apply_parts_delta(&next.parts, delta); }
        if let Some(delta) = &self.fasteners { next.fasteners = apply_fasteners_delta(&next.fasteners, delta); }
        next
    }

    fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() { *self = other; return; }
        macro_rules! take { ($f:ident) => { if other.$f.is_some() { self.$f = other.$f; } }; }
        take!(schema); take!(domain); take!(label); take!(meta); take!(kind_catalogs); take!(kind_compatibility);
        take!(selected_part_ids); take!(selected_grip_ids); take!(selected_fastener_ids); take!(active_utility_id);
        take!(camera2d_x); take!(camera2d_y); take!(camera2d_zoom);
        take!(camera3d_position_x); take!(camera3d_position_y); take!(camera3d_position_z);
        take!(camera3d_target_x); take!(camera3d_target_y); take!(camera3d_target_z); take!(camera3d_zoom);
        take!(selection_method); take!(grid_snap_enabled); take!(grid_factor); take!(suggestion_offset);
        take!(overlap_budget); take!(fill_count); take!(brush_candidate_index); take!(lod_mode); take!(locale);
        take!(runtime_extras_json); take!(hovered_part_id); take!(preview_seq);
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
        merge_delta!(parts); merge_delta!(fasteners);
    }
}
//#endregion 🔖️Apply

