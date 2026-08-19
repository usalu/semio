//! 🔺️ Block 5D artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::block5d::schema::diff::*;


use crate::artifacts::block5d::schema::Block5dArtifact;
use crate::artifacts::block5d::{Block5dGripKind, Block5dGripTemplate, Block5dSnapshot};
use crate::{BlockAttribute, BlockCompatibilityRule, BlockRepresentation};
use protocol::MutationDiff;

//#region 📖️SemioGrammar
/// 📖️ Normative handcrafted text grammar for this facet (`dialect grammar`).
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️component.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️component.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️Apply
async fn apply_identified_delta<T: Clone>(
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

macro_rules! apply_delta {
    ($target:literal, $items:expr, $delta:expr, $id:expr) => {{
        let patched: Vec<_> = $delta.patched.iter().map(|e| (e.id.clone(), e.patch.replacement.clone())).collect();
        apply_identified_delta($items, &$delta.removed, &$delta.added, &patched, &$delta.reordered, $id).map_err(|error| error.under([$target]))?
    }};
}

impl Block5dDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub async fn apply_to_artifact(&self, artifact: &Block5dArtifact) -> protocol::MutationApplyResult<Block5dArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact { return Ok((**replacement).clone()); }
            let mut next = artifact.clone();
            if let Some(v) = &self.schema { next.schema = v.clone(); }
            if let Some(v) = &self.part_kind { next.part_kind = v.clone(); }
            if let Some(v) = &self.part_2d { next.part_2d = v.clone(); }
            if let Some(v) = &self.part_3d { next.part_3d = v.clone(); }
            if let Some(d) = &self.representations { next.representations = apply_delta!("representations", &next.representations, d, |i: &BlockRepresentation| i.id.as_str()); }
            if let Some(d) = &self.grip_kinds { next.grip_kinds = apply_delta!("gripKinds", &next.grip_kinds, d, |i: &Block5dGripKind| i.id.as_str()); }
            if let Some(d) = &self.grips { next.grips = apply_delta!("grips", &next.grips, d, |i: &Block5dGripTemplate| i.id.as_str()); }
            if let Some(d) = &self.compatibility { next.compatibility = apply_delta!("compatibility", &next.compatibility, d, |i: &BlockCompatibilityRule| i.id.as_str()); }
            if let Some(d) = &self.attributes { next.attributes = apply_delta!("attributes", &next.attributes, d, |i: &BlockAttribute| i.key.as_str()); }
            if let Some(list) = &self.authors { next.authors = list.values.clone(); }
            if let Some(v) = &self.camera2d { next.camera2d = v.clone(); }
            if let Some(v) = &self.camera3d { next.camera3d = v.clone(); }
            if let Some(v) = &self.meta { next.meta = v.clone(); }
            if let Some(list) = &self.selected_ids { next.selected_ids = list.values.clone(); }
            if let Some(v) = &self.locale { next.locale = v.clone(); }
            next
        })
    }
}

impl MutationDiff<Block5dSnapshot> for Block5dDiff {
    async fn apply(&self, snapshot: &Block5dSnapshot) -> protocol::MutationApplyResult<Block5dSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact { return Ok(replacement.to_snapshot()); }
            let mut next = snapshot.clone();
            if let Some(v) = &self.schema { next.schema = v.clone(); }
            if let Some(v) = &self.part_kind { next.part_kind = v.clone(); }
            if let Some(v) = &self.part_2d { next.part_2d = v.clone(); }
            if let Some(v) = &self.part_3d { next.part_3d = v.clone(); }
            if let Some(d) = &self.representations { next.representations = apply_delta!("representations", &next.representations, d, |i: &BlockRepresentation| i.id.as_str()); }
            if let Some(d) = &self.grip_kinds { next.grip_kinds = apply_delta!("gripKinds", &next.grip_kinds, d, |i: &Block5dGripKind| i.id.as_str()); }
            if let Some(d) = &self.grips { next.grips = apply_delta!("grips", &next.grips, d, |i: &Block5dGripTemplate| i.id.as_str()); }
            if let Some(d) = &self.compatibility { next.compatibility = apply_delta!("compatibility", &next.compatibility, d, |i: &BlockCompatibilityRule| i.id.as_str()); }
            if let Some(d) = &self.attributes { next.attributes = apply_delta!("attributes", &next.attributes, d, |i: &BlockAttribute| i.key.as_str()); }
            if let Some(list) = &self.authors { next.authors = list.values.clone(); }
            if let Some(v) = &self.camera2d { next.camera2d = v.clone(); }
            if let Some(v) = &self.camera3d { next.camera3d = v.clone(); }
            if let Some(v) = &self.meta { next.meta = v.clone(); }
            next
        })
    }
    async fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() { *self = other; return; }
        macro_rules! take { ($f:ident) => { if other.$f.is_some() { self.$f = other.$f; } }; }
        take!(schema); take!(part_kind); take!(part_2d); take!(part_3d); take!(authors);
        take!(camera2d); take!(camera3d); take!(meta); take!(selected_ids); take!(locale);
        async fn absorb_col<D>(target: &mut Option<D>, incoming: Option<D>, merge: impl FnOnce(&mut D, D)) {
            if let Some(src) = incoming {
                match target { Some(dst) => merge(dst, src), None => *target = Some(src) }
            }
        }
        macro_rules! merge_delta {
            ($field:ident) => {
                absorb_col(&mut self.$field, other.$field, |dst, src| {
                    dst.removed.extend(src.removed); dst.added.extend(src.added); dst.patched.extend(src.patched);
                    if src.reordered.is_some() { dst.reordered = src.reordered; }
                });
            };
        }
        merge_delta!(representations); merge_delta!(grip_kinds); merge_delta!(grips);
        merge_delta!(compatibility); merge_delta!(attributes);
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffHelpers
pub(crate) trait Block5dHasId { fn id(&self) -> &str; }
impl Block5dHasId for BlockRepresentation { fn id(&self) -> &str { &self.id } }
impl Block5dHasId for Block5dGripKind { fn id(&self) -> &str { &self.id } }
impl Block5dHasId for Block5dGripTemplate { fn id(&self) -> &str { &self.id } }
impl Block5dHasId for BlockCompatibilityRule { fn id(&self) -> &str { &self.id } }
impl Block5dHasId for BlockAttribute { fn id(&self) -> &str { &self.key } }
pub(crate) async fn block5d_index_of<T: Block5dHasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}

pub async fn diff_set_representation(index: usize, item: BlockRepresentation, base: &Block5dSnapshot) -> Block5dDiff {
    let mut delta = Block5dRepresentationsDelta { added: vec![item.clone()], ..Default::default() };
    if block5d_index_of(&base.representations, &item.id).is_none() {
        let mut order: Vec<_> = base.representations.iter().map(|e| e.id.clone()).collect();
        order.insert(index.min(order.len()), item.id.clone());
        delta.reordered = Some(order);
    }
    Block5dDiff { representations: Some(delta), ..Default::default() }
}
pub async fn diff_remove_representation(id: String) -> Block5dDiff {
    Block5dDiff { representations: Some(Block5dRepresentationsDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub async fn diff_set_grip_kind(index: usize, item: Block5dGripKind, base: &Block5dSnapshot) -> Block5dDiff {
    let mut delta = Block5dGripKindsDelta { added: vec![item.clone()], ..Default::default() };
    if block5d_index_of(&base.grip_kinds, &item.id).is_none() {
        let mut order: Vec<_> = base.grip_kinds.iter().map(|e| e.id.clone()).collect();
        order.insert(index.min(order.len()), item.id.clone());
        delta.reordered = Some(order);
    }
    Block5dDiff { grip_kinds: Some(delta), ..Default::default() }
}
pub async fn diff_remove_grip_kind(id: String) -> Block5dDiff {
    Block5dDiff { grip_kinds: Some(Block5dGripKindsDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub async fn diff_set_grip(index: usize, item: Block5dGripTemplate, base: &Block5dSnapshot) -> Block5dDiff {
    let mut delta = Block5dGripsDelta { added: vec![item.clone()], ..Default::default() };
    if block5d_index_of(&base.grips, &item.id).is_none() {
        let mut order: Vec<_> = base.grips.iter().map(|e| e.id.clone()).collect();
        order.insert(index.min(order.len()), item.id.clone());
        delta.reordered = Some(order);
    }
    Block5dDiff { grips: Some(delta), ..Default::default() }
}
pub async fn diff_remove_grip(id: String) -> Block5dDiff {
    Block5dDiff { grips: Some(Block5dGripsDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub async fn diff_set_compatibility_rule(index: usize, rule: BlockCompatibilityRule, base: &Block5dSnapshot) -> Block5dDiff {
    let mut delta = Block5dCompatibilityDelta { added: vec![rule.clone()], ..Default::default() };
    if block5d_index_of(&base.compatibility, &rule.id).is_none() {
        let mut order: Vec<_> = base.compatibility.iter().map(|e| e.id.clone()).collect();
        order.insert(index.min(order.len()), rule.id.clone());
        delta.reordered = Some(order);
    }
    Block5dDiff { compatibility: Some(delta), ..Default::default() }
}
pub async fn diff_remove_compatibility_rule(id: String) -> Block5dDiff {
    Block5dDiff { compatibility: Some(Block5dCompatibilityDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}
pub async fn diff_set_attribute(index: usize, attribute: BlockAttribute, base: &Block5dSnapshot) -> Block5dDiff {
    let mut delta = Block5dAttributesDelta { added: vec![attribute.clone()], ..Default::default() };
    if block5d_index_of(&base.attributes, &attribute.key).is_none() {
        let mut order: Vec<_> = base.attributes.iter().map(|e| e.key.clone()).collect();
        order.insert(index.min(order.len()), attribute.key.clone());
        delta.reordered = Some(order);
    }
    Block5dDiff { attributes: Some(delta), ..Default::default() }
}
pub async fn diff_remove_attribute(key: String) -> Block5dDiff {
    Block5dDiff { attributes: Some(Block5dAttributesDelta { removed: vec![key], ..Default::default() }), ..Default::default() }
}
pub async fn diff_set_snapshot(snapshot: Block5dSnapshot) -> Block5dDiff {
    Block5dDiff { artifact: Some(Box::new(Block5dArtifact::from_snapshot(snapshot))), ..Default::default() }
}
//#endregion 🔖️DiffHelpers
