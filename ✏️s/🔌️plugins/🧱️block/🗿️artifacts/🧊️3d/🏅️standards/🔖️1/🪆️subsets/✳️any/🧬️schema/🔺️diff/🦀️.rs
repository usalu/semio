//! 🧬️ Block3d diff schema — sparse field delta over the artifact.

use crate::Block3dWindowView;
use crate::{Block3dVortexKind, Block3dVortexTemplate};
use crate::{BlockAttribute, BlockAuthor, BlockCamera3d, BlockCompatibilityRule, BlockKindIdentity, BlockMeta, BlockRepresentation};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the block3d artifact.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.block.block3d")]
pub struct Block3dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::standards::v1::subsets::any::schema::Block3dArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub object_kind: Option<BlockKindIdentity>,
    #[state(artifact)]
    pub representations: Option<Block3dRepresentationsDelta>,
    #[state(artifact)]
    pub vortex_kinds: Option<Block3dVortexKindsDelta>,
    #[state(artifact)]
    pub vortices: Option<Block3dVorticesDelta>,
    #[state(artifact)]
    pub compatibility: Option<Block3dCompatibilityDelta>,
    #[state(artifact)]
    pub attributes: Option<Block3dAttributesDelta>,
    #[state(artifact)]
    pub authors: Option<Block3dAuthorList>,
    #[state(artifact)]
    pub camera3d: Option<BlockCamera3d>,
    #[state(artifact)]
    pub meta: Option<BlockMeta>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Block3dStringList {
    pub values: Vec<String>,
}

/// 👤️ Author-list wrapper.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Block3dAuthorList {
    pub values: Vec<BlockAuthor>,
}

/// 🪟 Windows-list wrapper.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Block3dWindowsList {
    pub values: Vec<Block3dWindowView>,
}

/// 📂 Identified-collection delta for Representations.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Block3dRepresentationsDelta {
    pub added: Vec<BlockRepresentation>,
    pub removed: Vec<String>,
    pub patched: Vec<Block3dRepresentationsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched Representations entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Block3dRepresentationsPatchEntry {
    pub id: String,
    pub patch: Block3dRepresentationsPatch,
}

/// 🩹 Sparse patch over Representations.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Block3dRepresentationsPatch {
    pub replacement: Option<BlockRepresentation>,
}

/// 📂 Identified-collection delta for VortexKinds.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Block3dVortexKindsDelta {
    pub added: Vec<Block3dVortexKind>,
    pub removed: Vec<String>,
    pub patched: Vec<Block3dVortexKindsPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched VortexKinds entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Block3dVortexKindsPatchEntry {
    pub id: String,
    pub patch: Block3dVortexKindsPatch,
}

/// 🩹 Sparse patch over VortexKinds.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Block3dVortexKindsPatch {
    pub replacement: Option<Block3dVortexKind>,
}

/// 📂 Identified-collection delta for Vortices.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Block3dVorticesDelta {
    pub added: Vec<Block3dVortexTemplate>,
    pub removed: Vec<String>,
    pub patched: Vec<Block3dVorticesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched Vortices entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Block3dVorticesPatchEntry {
    pub id: String,
    pub patch: Block3dVorticesPatch,
}

/// 🩹 Sparse patch over Vortices.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Block3dVorticesPatch {
    pub replacement: Option<Block3dVortexTemplate>,
}

/// 📂 Identified-collection delta for Compatibility.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Block3dCompatibilityDelta {
    pub added: Vec<BlockCompatibilityRule>,
    pub removed: Vec<String>,
    pub patched: Vec<Block3dCompatibilityPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched Compatibility entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Block3dCompatibilityPatchEntry {
    pub id: String,
    pub patch: Block3dCompatibilityPatch,
}

/// 🩹 Sparse patch over Compatibility.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Block3dCompatibilityPatch {
    pub replacement: Option<BlockCompatibilityRule>,
}

/// 📂 Identified-collection delta for Attributes.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Block3dAttributesDelta {
    pub added: Vec<BlockAttribute>,
    pub removed: Vec<String>,
    pub patched: Vec<Block3dAttributesPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched Attributes entry.
#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Block3dAttributesPatchEntry {
    pub id: String,
    pub patch: Block3dAttributesPatch,
}

/// 🩹 Sparse patch over Attributes.
#[derive(Clone, Debug, Default, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Block3dAttributesPatch {
    pub replacement: Option<BlockAttribute>,
}

//#endregion 🔖️DeltaHelpers

use crate::standards::v1::subsets::any::schema::Block3dArtifact;
use crate::Block3dSnapshot;
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

macro_rules! apply_delta {
    ($target:literal, $items:expr, $delta:expr, $id:expr) => {{
        let patched: Vec<_> = $delta.patched.iter().map(|e| (e.id.clone(), e.patch.replacement.clone())).collect();
        apply_identified_delta($items, &$delta.removed, &$delta.added, &patched, &$delta.reordered, $id).map_err(|error| error.under([$target]))?
    }};
}

impl Block3dDiff {
    /// 🧬️ Applies sparse document changes to the artifact.
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
                let current = crate::vortex_kinds_of_parts(&next.catalog, &next.vortex_kind_extra);
                let merged = apply_delta!("vortexKinds", &current, d, |i: &Block3dVortexKind| i.id.as_str());
                crate::set_vortex_kinds_parts(&mut next.catalog, &mut next.vortex_kind_extra, &merged);
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
                let current = crate::vortex_kinds_of(&next);
                let merged = apply_delta!("vortexKinds", &current, d, |i: &Block3dVortexKind| i.id.as_str());
                crate::set_vortex_kinds(&mut next, &merged);
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
        /// 🩹 Two edits touching the same item coalesce into ONE patch entry: `apply` refuses a
        /// collection whose `patched` list names the same id twice
        /// (`mutation.apply.duplicate-target`), so a plain `extend` breaks the diff-absorb law the
        /// moment one gesture patches the same row twice. Every patch here is a whole-item
        /// `replacement`, so the later entry simply supersedes the earlier one.
        fn absorb_patches<E>(target: &mut Vec<E>, incoming: Vec<E>, id_of: impl Fn(&E) -> &str) {
            for entry in incoming {
                match target.iter().position(|existing| id_of(existing) == id_of(&entry)) {
                    Some(position) => target[position] = entry,
                    None => target.push(entry),
                }
            }
        }
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
                    absorb_patches(&mut dst.patched, src.patched, |entry| entry.id.as_str());
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
    let reordered = if block3d_index_of(&base.representations, &item.id).is_none() {
        let mut order: Vec<_> = base.representations.iter().map(|e| e.id.clone()).collect();
        order.insert(index.min(order.len()), item.id.clone());
        Some(order)
    } else {
        None
    };
    let delta = Block3dRepresentationsDelta { added: vec![item], reordered, ..Default::default() };
    Block3dDiff { representations: Some(delta), ..Default::default() }
}

pub fn diff_remove_representation(id: String) -> Block3dDiff {
    Block3dDiff { representations: Some(Block3dRepresentationsDelta { removed: vec![id], ..Default::default() }), ..Default::default() }
}

pub fn diff_set_vortex_kind(index: usize, item: Block3dVortexKind, base: &Block3dSnapshot) -> Block3dDiff {
    let current = crate::vortex_kinds_of(base);
    let mut delta = Block3dVortexKindsDelta { added: vec![item.clone()], ..Default::default() };
    if block3d_index_of(&current, &item.id).is_none() {
        let mut order: Vec<_> = current.iter().map(|e| e.id.clone()).collect();
        order.insert(index.min(order.len()), item.id);
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
        order.insert(index.min(order.len()), item.id);
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
        order.insert(index.min(order.len()), rule.id);
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
        order.insert(index.min(order.len()), attribute.key);
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
