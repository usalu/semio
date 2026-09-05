//! 🔺️ Block 3D artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::block3d::schema::diff::*;

use crate::artifacts::block3d::schema::Block3dArtifact;
use crate::artifacts::block3d::{Block3dSnapshot, Block3dVortexKind, Block3dVortexTemplate};
use crate::{BlockAttribute, BlockCompatibilityRule, BlockRepresentation};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
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

macro_rules! apply_delta {
    ($target:literal, $items:expr, $delta:expr, $id:expr) => {{
        let patched: Vec<_> = $delta.patched.iter().map(|e| (e.id.clone(), e.patch.replacement.clone())).collect();
        apply_identified_delta($items, &$delta.removed, &$delta.added, &patched, &$delta.reordered, $id).map_err(|error| error.under([$target]))?
    }};
}

impl Block3dDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub fn apply_to_artifact(&self, artifact: &Block3dArtifact) -> protocol::MutationApplyResult<Block3dArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(v) = &self.schema {
                next.schema = v.clone();
            }
            if let Some(v) = &self.object_kind {
                next.object_kind = v.clone();
            }
            if let Some(d) = &self.representations {
                next.representations = apply_delta!("representations", &next.representations, d, |i: &BlockRepresentation| i.id.as_str());
            }
            if let Some(d) = &self.vortex_kinds {
                let current = crate::artifacts::block3d::vortex_kinds_of_parts(&next.catalog, &next.vortex_kind_extra);
                let merged = apply_delta!("vortexKinds", &current, d, |i: &Block3dVortexKind| i.id.as_str());
                crate::artifacts::block3d::set_vortex_kinds_parts(&mut next.catalog, &mut next.vortex_kind_extra, merged);
            }
            if let Some(d) = &self.vortices {
                next.vortices = apply_delta!("vortices", &next.vortices, d, |i: &Block3dVortexTemplate| i.id.as_str());
            }
            if let Some(d) = &self.compatibility {
                next.compatibility = apply_delta!("compatibility", &next.compatibility, d, |i: &BlockCompatibilityRule| i.id.as_str());
            }
            if let Some(d) = &self.attributes {
                next.attributes = apply_delta!("attributes", &next.attributes, d, |i: &BlockAttribute| i.key.as_str());
            }
            if let Some(list) = &self.authors {
                next.authors = list.values.clone();
            }
            if let Some(v) = &self.camera3d {
                next.camera3d = v.clone();
            }
            if let Some(v) = &self.meta {
                next.meta = v.clone();
            }
            if let Some(list) = &self.selected_ids {
                next.selected_ids = list.values.clone();
            }
            if let Some(v) = &self.active_representation_id {
                next.active_representation_id = v.clone();
            }
            if let Some(list) = &self.wanted_tags {
                next.wanted_tags = list.values.clone();
            }
            if let Some(v) = &self.locale {
                next.locale = v.clone();
            }
            if let Some(list) = &self.windows {
                next.windows = list.values.clone();
            }
            if let Some(v) = &self.brush_vortex_kind_id {
                next.brush_vortex_kind_id = v.clone();
            }
            if let Some(v) = self.brush_radius {
                next.brush_radius = v;
            }
            if let Some(v) = self.brush_flip {
                next.brush_flip = v;
            }
            if let Some(v) = &self.brush_preview {
                next.brush_preview = v.clone();
            }
            if let Some(v) = &self.camera {
                next.camera = v.clone();
            }
            if let Some(v) = &self.hovered_vortex_full_id {
                next.hovered_vortex_full_id = v.clone();
            }
            next
        })
    }
}

impl MutationDiff<Block3dSnapshot> for Block3dDiff {
    fn apply(&self, snapshot: &Block3dSnapshot) -> protocol::MutationApplyResult<Block3dSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(v) = &self.schema {
                next.schema = v.clone();
            }
            if let Some(v) = &self.object_kind {
                next.object_kind = v.clone();
            }
            if let Some(d) = &self.representations {
                next.representations = apply_delta!("representations", &next.representations, d, |i: &BlockRepresentation| i.id.as_str());
            }
            if let Some(d) = &self.vortex_kinds {
                let current = crate::artifacts::block3d::vortex_kinds_of(&next);
                let merged = apply_delta!("vortexKinds", &current, d, |i: &Block3dVortexKind| i.id.as_str());
                crate::artifacts::block3d::set_vortex_kinds(&mut next, merged);
            }
            if let Some(d) = &self.vortices {
                next.vortices = apply_delta!("vortices", &next.vortices, d, |i: &Block3dVortexTemplate| i.id.as_str());
            }
            if let Some(d) = &self.compatibility {
                next.compatibility = apply_delta!("compatibility", &next.compatibility, d, |i: &BlockCompatibilityRule| i.id.as_str());
            }
            if let Some(d) = &self.attributes {
                next.attributes = apply_delta!("attributes", &next.attributes, d, |i: &BlockAttribute| i.key.as_str());
            }
            if let Some(list) = &self.authors {
                next.authors = list.values.clone();
            }
            if let Some(v) = &self.camera3d {
                next.camera3d = v.clone();
            }
            if let Some(v) = &self.meta {
                next.meta = v.clone();
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
        take!(object_kind);
        take!(authors);
        take!(camera3d);
        take!(meta);
        take!(selected_ids);
        take!(active_representation_id);
        take!(wanted_tags);
        take!(locale);
        take!(windows);
        take!(brush_vortex_kind_id);
        take!(brush_radius);
        take!(brush_flip);
        take!(brush_preview);
        take!(camera);
        take!(hovered_vortex_full_id);
        fn absorb_col<D: Default>(target: &mut Option<D>, incoming: Option<D>, merge: impl FnOnce(&mut D, D)) {
            if let Some(src) = incoming {
                match target {
                    Some(dst) => merge(dst, src),
                    None => *target = Some(src),
                }
            }
        }
        macro_rules! merge_delta {
            ($field:ident) => {
                absorb_col(&mut self.$field, other.$field, |dst, src| {
                    dst.removed.extend(src.removed);
                    dst.added.extend(src.added);
                    dst.patched.extend(src.patched);
                    if src.reordered.is_some() {
                        dst.reordered = src.reordered;
                    }
                });
            };
        }
        merge_delta!(representations);
        merge_delta!(vortex_kinds);
        merge_delta!(vortices);
        merge_delta!(compatibility);
        merge_delta!(attributes);
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffHelpers
pub(crate) trait Block3dHasId {
    fn id(&self) -> &str;
}
impl Block3dHasId for BlockRepresentation {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block3dHasId for Block3dVortexKind {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block3dHasId for Block3dVortexTemplate {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block3dHasId for BlockCompatibilityRule {
    fn id(&self) -> &str {
        &self.id
    }
}
impl Block3dHasId for BlockAttribute {
    fn id(&self) -> &str {
        &self.key
    }
}

pub(crate) fn block3d_index_of<T: Block3dHasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}

pub fn diff_set_representation(index: usize, item: BlockRepresentation, base: &Block3dSnapshot) -> Block3dDiff {
    let mut delta = Block3dRepresentationsDelta { added: vec![item.clone()], ..Default::default() };
    if block3d_index_of(&base.representations, &item.id).is_none() {
        let mut order: Vec<_> = base.representations.iter().map(|e| e.id.clone()).collect();
        order.insert(index.min(order.len()), item.id.clone());
        delta.reordered = Some(order);
    }
    Block3dDiff { representations: Some(delta), ..Default::default() }
}
pub fn diff_remove_representation(id: String) -> Block3dDiff {
    Block3dDiff { representations: Some(Block3dRepresentationsDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub fn diff_set_vortex_kind(index: usize, item: Block3dVortexKind, base: &Block3dSnapshot) -> Block3dDiff {
    let current = crate::artifacts::block3d::vortex_kinds_of(base);
    let mut delta = Block3dVortexKindsDelta { added: vec![item.clone()], ..Default::default() };
    if block3d_index_of(&current, &item.id).is_none() {
        let mut order: Vec<_> = current.iter().map(|e| e.id.clone()).collect();
        order.insert(index.min(order.len()), item.id.clone());
        delta.reordered = Some(order);
    }
    Block3dDiff { vortex_kinds: Some(delta), ..Default::default() }
}
pub fn diff_remove_vortex_kind(id: String) -> Block3dDiff {
    Block3dDiff { vortex_kinds: Some(Block3dVortexKindsDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub fn diff_set_vortex(index: usize, item: Block3dVortexTemplate, base: &Block3dSnapshot) -> Block3dDiff {
    let mut delta = Block3dVorticesDelta { added: vec![item.clone()], ..Default::default() };
    if block3d_index_of(&base.vortices, &item.id).is_none() {
        let mut order: Vec<_> = base.vortices.iter().map(|e| e.id.clone()).collect();
        order.insert(index.min(order.len()), item.id.clone());
        delta.reordered = Some(order);
    }
    Block3dDiff { vortices: Some(delta), ..Default::default() }
}
pub fn diff_remove_vortex(id: String) -> Block3dDiff {
    Block3dDiff { vortices: Some(Block3dVorticesDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub fn diff_set_compatibility_rule(index: usize, rule: BlockCompatibilityRule, base: &Block3dSnapshot) -> Block3dDiff {
    let mut delta = Block3dCompatibilityDelta { added: vec![rule.clone()], ..Default::default() };
    if block3d_index_of(&base.compatibility, &rule.id).is_none() {
        let mut order: Vec<_> = base.compatibility.iter().map(|e| e.id.clone()).collect();
        order.insert(index.min(order.len()), rule.id.clone());
        delta.reordered = Some(order);
    }
    Block3dDiff { compatibility: Some(delta), ..Default::default() }
}
pub fn diff_remove_compatibility_rule(id: String) -> Block3dDiff {
    Block3dDiff { compatibility: Some(Block3dCompatibilityDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub fn diff_set_attribute(index: usize, attribute: BlockAttribute, base: &Block3dSnapshot) -> Block3dDiff {
    let mut delta = Block3dAttributesDelta { added: vec![attribute.clone()], ..Default::default() };
    if block3d_index_of(&base.attributes, &attribute.key).is_none() {
        let mut order: Vec<_> = base.attributes.iter().map(|e| e.key.clone()).collect();
        order.insert(index.min(order.len()), attribute.key.clone());
        delta.reordered = Some(order);
    }
    Block3dDiff { attributes: Some(delta), ..Default::default() }
}
pub fn diff_remove_attribute(key: String) -> Block3dDiff {
    Block3dDiff { attributes: Some(Block3dAttributesDelta { removed: vec![key], ..Default::default() }), ..Default::default() }
}
pub fn diff_set_snapshot(snapshot: Block3dSnapshot) -> Block3dDiff {
    Block3dDiff { artifact: Some(Box::new(Block3dArtifact::from_snapshot(snapshot))), ..Default::default() }
}
//#endregion 🔖️DiffHelpers
