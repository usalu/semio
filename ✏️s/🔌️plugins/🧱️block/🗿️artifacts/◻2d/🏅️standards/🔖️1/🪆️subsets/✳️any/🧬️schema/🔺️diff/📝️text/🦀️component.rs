//! 🔺️ Block 2D artifact — sparse field-delta diff codec and apply/absorb.

use crate::artifacts::block2d::schema::diff::*;


use crate::artifacts::block2d::schema::Block2dArtifact;
use crate::artifacts::block2d::{Block2dHandleKind, Block2dHandleTemplate, Block2dSnapshot};
use crate::{BlockAttribute, BlockCompatibilityRule};
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

async fn apply_handle_kinds_delta(items: &[Block2dHandleKind], delta: &Block2dHandleKindsDelta) -> protocol::MutationApplyResult<Vec<Block2dHandleKind>> {
    let patched: Vec<_> = delta.patched.iter().map(|e| (e.id.clone(), e.patch.replacement.clone())).collect();
    apply_identified_delta(items, &delta.removed, &delta.added, &patched, &delta.reordered, |item| item.id.as_str())
}

async fn apply_handles_delta(items: &[Block2dHandleTemplate], delta: &Block2dHandlesDelta) -> protocol::MutationApplyResult<Vec<Block2dHandleTemplate>> {
    let patched: Vec<_> = delta.patched.iter().map(|e| (e.id.clone(), e.patch.replacement.clone())).collect();
    apply_identified_delta(items, &delta.removed, &delta.added, &patched, &delta.reordered, |item| item.id.as_str())
}

async fn apply_compatibility_delta(items: &[BlockCompatibilityRule], delta: &Block2dCompatibilityDelta) -> protocol::MutationApplyResult<Vec<BlockCompatibilityRule>> {
    let patched: Vec<_> = delta.patched.iter().map(|e| (e.id.clone(), e.patch.replacement.clone())).collect();
    apply_identified_delta(items, &delta.removed, &delta.added, &patched, &delta.reordered, |item| item.id.as_str())
}

async fn apply_attributes_delta(items: &[BlockAttribute], delta: &Block2dAttributesDelta) -> protocol::MutationApplyResult<Vec<BlockAttribute>> {
    let patched: Vec<_> = delta.patched.iter().map(|e| (e.id.clone(), e.patch.replacement.clone())).collect();
    apply_identified_delta(items, &delta.removed, &delta.added, &patched, &delta.reordered, |item| item.key.as_str())
}

impl Block2dDiff {
    /// 🧬️ Applies every sparse entry (all state classes) onto a full artifact.
    pub async fn apply_to_artifact(&self, artifact: &Block2dArtifact) -> protocol::MutationApplyResult<Block2dArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema { next.schema = schema.clone(); }
            if let Some(node_kind) = &self.node_kind { next.node_kind = node_kind.clone(); }
            if let Some(presentation) = &self.presentation { next.presentation = presentation.clone(); }
            if let Some(delta) = &self.handle_kinds { next.handle_kinds = apply_handle_kinds_delta(&next.handle_kinds, delta).map_err(|error| error.under(["handleKinds"]))?; }
            if let Some(delta) = &self.handles { next.handles = apply_handles_delta(&next.handles, delta).map_err(|error| error.under(["handles"]))?; }
            if let Some(delta) = &self.compatibility { next.compatibility = apply_compatibility_delta(&next.compatibility, delta).map_err(|error| error.under(["compatibility"]))?; }
            if let Some(delta) = &self.attributes { next.attributes = apply_attributes_delta(&next.attributes, delta).map_err(|error| error.under(["attributes"]))?; }
            if let Some(list) = &self.authors { next.authors = list.values.clone(); }
            if let Some(camera2d) = &self.camera2d { next.camera2d = camera2d.clone(); }
            if let Some(meta) = &self.meta { next.meta = meta.clone(); }
            if let Some(list) = &self.selected_ids { next.selected_ids = list.values.clone(); }
            if let Some(locale) = &self.locale { next.locale = locale.clone(); }
            next
        })
    }
}

impl MutationDiff<Block2dSnapshot> for Block2dDiff {
    async fn apply(&self, snapshot: &Block2dSnapshot) -> protocol::MutationApplyResult<Block2dSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema { next.schema = schema.clone(); }
            if let Some(node_kind) = &self.node_kind { next.node_kind = node_kind.clone(); }
            if let Some(presentation) = &self.presentation { next.presentation = presentation.clone(); }
            if let Some(delta) = &self.handle_kinds { next.handle_kinds = apply_handle_kinds_delta(&next.handle_kinds, delta).map_err(|error| error.under(["handleKinds"]))?; }
            if let Some(delta) = &self.handles { next.handles = apply_handles_delta(&next.handles, delta).map_err(|error| error.under(["handles"]))?; }
            if let Some(delta) = &self.compatibility { next.compatibility = apply_compatibility_delta(&next.compatibility, delta).map_err(|error| error.under(["compatibility"]))?; }
            if let Some(delta) = &self.attributes { next.attributes = apply_attributes_delta(&next.attributes, delta).map_err(|error| error.under(["attributes"]))?; }
            if let Some(list) = &self.authors { next.authors = list.values.clone(); }
            if let Some(camera2d) = &self.camera2d { next.camera2d = camera2d.clone(); }
            if let Some(meta) = &self.meta { next.meta = meta.clone(); }
            next
        })
    }
    async fn absorb(&mut self, other: Self) {
        if other.artifact.is_some() {
            *self = other;
            return;
        }
        macro_rules! take {
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(node_kind);
        take!(presentation);
        take!(authors);
        take!(camera2d);
        take!(meta);
        take!(selected_ids);
        take!(locale);
        async fn absorb_delta<D, F>(target: &mut Option<D>, incoming: Option<D>, merge: F)
        where
            F: FnOnce(&mut D, D),
        {
            if let Some(src) = incoming {
                match target {
                    Some(dst) => merge(dst, src),
                    None => *target = Some(src),
                }
            }
        }
        absorb_delta(&mut self.handle_kinds, other.handle_kinds, |dst, src| {
            dst.removed.extend(src.removed);
            dst.added.extend(src.added);
            dst.patched.extend(src.patched);
            if src.reordered.is_some() { dst.reordered = src.reordered; }
        });
        absorb_delta(&mut self.handles, other.handles, |dst, src| {
            dst.removed.extend(src.removed);
            dst.added.extend(src.added);
            dst.patched.extend(src.patched);
            if src.reordered.is_some() { dst.reordered = src.reordered; }
        });
        absorb_delta(&mut self.compatibility, other.compatibility, |dst, src| {
            dst.removed.extend(src.removed);
            dst.added.extend(src.added);
            dst.patched.extend(src.patched);
            if src.reordered.is_some() { dst.reordered = src.reordered; }
        });
        absorb_delta(&mut self.attributes, other.attributes, |dst, src| {
            dst.removed.extend(src.removed);
            dst.added.extend(src.added);
            dst.patched.extend(src.patched);
            if src.reordered.is_some() { dst.reordered = src.reordered; }
        });
    }
}
//#endregion 🔖️Apply

//#region 🔖️DiffHelpers
pub(crate) trait Block2dHasId {
    async fn id(&self) -> &str;
}
impl Block2dHasId for Block2dHandleKind { fn id(&self) -> &str { &self.id } }
impl Block2dHasId for Block2dHandleTemplate { fn id(&self) -> &str { &self.id } }
impl Block2dHasId for BlockCompatibilityRule { fn id(&self) -> &str { &self.id } }
impl Block2dHasId for BlockAttribute { fn id(&self) -> &str { &self.key } }

/// 🔍️ Index of an id-keyed row.
pub(crate) async fn block2d_index_of<T: Block2dHasId>(items: &[T], id: &str) -> Option<usize> {
    items.iter().position(|item| item.id() == id)
}

/// 📍 Builds a handle-kinds set delta, preserving insert index via `reordered` when the id is new.
pub async fn diff_set_handle_kind(index: usize, handle_kind: Block2dHandleKind, base: &Block2dSnapshot) -> Block2dDiff {
    let mut delta = Block2dHandleKindsDelta { added: vec![handle_kind.clone()], ..Default::default() };
    if block2d_index_of(&base.handle_kinds, &handle_kind.id).is_none() {
        let mut order: Vec<String> = base.handle_kinds.iter().map(|e| e.id.clone()).collect();
        let at = index.min(order.len());
        order.insert(at, handle_kind.id.clone());
        delta.reordered = Some(order);
    }
    Block2dDiff { handle_kinds: Some(delta), ..Default::default() }
}

/// ➖ Builds a handle-kinds remove delta.
pub async fn diff_remove_handle_kind(id: String) -> Block2dDiff {
    Block2dDiff { handle_kinds: Some(Block2dHandleKindsDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}

/// 📍 Builds a handles set delta.
pub async fn diff_set_handle(index: usize, handle: Block2dHandleTemplate, base: &Block2dSnapshot) -> Block2dDiff {
    let mut delta = Block2dHandlesDelta { added: vec![handle.clone()], ..Default::default() };
    if block2d_index_of(&base.handles, &handle.id).is_none() {
        let mut order: Vec<String> = base.handles.iter().map(|e| e.id.clone()).collect();
        let at = index.min(order.len());
        order.insert(at, handle.id.clone());
        delta.reordered = Some(order);
    }
    Block2dDiff { handles: Some(delta), ..Default::default() }
}

/// ➖ Builds a handles remove delta.
pub async fn diff_remove_handle(id: String) -> Block2dDiff {
    Block2dDiff { handles: Some(Block2dHandlesDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}

/// 📍 Builds a compatibility set delta.
pub async fn diff_set_compatibility_rule(index: usize, rule: BlockCompatibilityRule, base: &Block2dSnapshot) -> Block2dDiff {
    let mut delta = Block2dCompatibilityDelta { added: vec![rule.clone()], ..Default::default() };
    if block2d_index_of(&base.compatibility, &rule.id).is_none() {
        let mut order: Vec<String> = base.compatibility.iter().map(|e| e.id.clone()).collect();
        let at = index.min(order.len());
        order.insert(at, rule.id.clone());
        delta.reordered = Some(order);
    }
    Block2dDiff { compatibility: Some(delta), ..Default::default() }
}

/// ➖ Builds a compatibility remove delta.
pub async fn diff_remove_compatibility_rule(id: String) -> Block2dDiff {
    Block2dDiff { compatibility: Some(Block2dCompatibilityDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}

/// 📍 Builds an attributes set delta.
pub async fn diff_set_attribute(index: usize, attribute: BlockAttribute, base: &Block2dSnapshot) -> Block2dDiff {
    let mut delta = Block2dAttributesDelta { added: vec![attribute.clone()], ..Default::default() };
    if block2d_index_of(&base.attributes, &attribute.key).is_none() {
        let mut order: Vec<String> = base.attributes.iter().map(|e| e.key.clone()).collect();
        let at = index.min(order.len());
        order.insert(at, attribute.key.clone());
        delta.reordered = Some(order);
    }
    Block2dDiff { attributes: Some(delta), ..Default::default() }
}

/// ➖ Builds an attributes remove delta.
pub async fn diff_remove_attribute(key: String) -> Block2dDiff {
    Block2dDiff { attributes: Some(Block2dAttributesDelta { removed: vec![key], ..Default::default() }), ..Default::default() }
}

/// 🌍️ Builds a whole-artifact replacement delta from a snapshot.
pub async fn diff_set_snapshot(snapshot: Block2dSnapshot) -> Block2dDiff {
    Block2dDiff {
        artifact: Some(Box::new(Block2dArtifact::from_snapshot(snapshot))),
        ..Default::default()
    }
}
//#endregion 🔖️DiffHelpers
