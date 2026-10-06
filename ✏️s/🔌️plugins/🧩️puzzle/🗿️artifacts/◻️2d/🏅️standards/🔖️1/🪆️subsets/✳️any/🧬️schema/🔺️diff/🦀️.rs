//! 🧬️ Puzzle2d diff schema — sparse field delta over the artifact.

use crate::standards::v1::subsets::any::schema::Puzzle2dArtifact;
use crate::{Puzzle2dCamera, Puzzle2dEdge, Puzzle2dMeta, Puzzle2dNode, Puzzle2dTargetRegion};
use ::semio_framework_schema::ArtifactSchema;

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the puzzle2d artifact.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle2d")]
pub struct Puzzle2dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<Puzzle2dArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub camera: Option<Puzzle2dCamera>,
    #[state(artifact)]
    pub nodes: Option<Puzzle2dNodesDelta>,
    #[state(artifact)]
    pub edges: Option<Puzzle2dEdgesDelta>,
    #[state(artifact)]
    pub target_regions: Option<Puzzle2dTargetRegionsDelta>,
    #[state(artifact)]
    pub meta: Option<Puzzle2dMeta>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dStringList {
    pub values: Vec<String>,
}

/// 🧩 Identified-collection delta for `nodes`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dNodesDelta {
    pub added: Vec<Puzzle2dNode>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle2dNodePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle2dNode` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dNodePatchEntry {
    pub id: String,
    pub patch: Puzzle2dNodePatch,
}

/// 🩹 Sparse patch over `Puzzle2dNode` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dNodePatch {
    pub replacement: Option<Puzzle2dNode>,
}

/// 🧩 Identified-collection delta for `edges`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dEdgesDelta {
    pub added: Vec<Puzzle2dEdge>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle2dEdgePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle2dEdge` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dEdgePatchEntry {
    pub id: String,
    pub patch: Puzzle2dEdgePatch,
}

/// 🩹 Sparse patch over `Puzzle2dEdge` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dEdgePatch {
    pub replacement: Option<Puzzle2dEdge>,
}

/// 🧩 Identified-collection delta for `targetRegions`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dTargetRegionsDelta {
    pub added: Vec<Puzzle2dTargetRegion>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle2dTargetRegionPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle2dTargetRegion` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dTargetRegionPatchEntry {
    pub id: String,
    pub patch: Puzzle2dTargetRegionPatch,
}

/// 🩹 Sparse patch over `Puzzle2dTargetRegion` — whole-item replacement via `replacement`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dTargetRegionPatch {
    pub replacement: Option<Puzzle2dTargetRegion>,
}

//#endregion 🔖️DeltaHelpers

use crate::Puzzle2dSnapshot;
use protocol::MutationDiff;

impl Puzzle2dDiff {
    /// 🧬️ Applies sparse document changes to the artifact.
    pub fn apply_to_artifact(&self, artifact: &Puzzle2dArtifact) -> protocol::MutationApplyResult<Puzzle2dArtifact> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok((**replacement).clone());
            }
            let mut next = artifact.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(camera) = &self.camera {
                next.camera = camera.clone();
            }
            if let Some(delta) = &self.nodes {
                next.nodes = apply_nodes_delta(&next.nodes, delta).map_err(|error| error.under(["nodes"]))?;
            }
            if let Some(delta) = &self.edges {
                next.edges = apply_edges_delta(&next.edges, delta).map_err(|error| error.under(["edges"]))?;
            }
            if let Some(delta) = &self.target_regions {
                next.target_regions = apply_target_regions_delta(&next.target_regions, delta).map_err(|error| error.under(["targetRegions"]))?;
            }
            if let Some(meta) = &self.meta {
                next.meta = meta.clone();
            }
            next
        })
    }
}

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

/// 🧩 Applies an identified-collection delta to nodes.
pub fn apply_nodes_delta(nodes: &[Puzzle2dNode], delta: &Puzzle2dNodesDelta) -> protocol::MutationApplyResult<Vec<Puzzle2dNode>> {
    let patched: Vec<_> = delta.patched.iter().map(|entry| (entry.id.clone(), entry.patch.replacement.clone())).collect();
    apply_identified_delta(nodes, &delta.removed, &delta.added, &patched, &delta.reordered, |n| &n.id)
}

/// 🧩 Applies an identified-collection delta to target regions.
pub fn apply_target_regions_delta(regions: &[Puzzle2dTargetRegion], delta: &Puzzle2dTargetRegionsDelta) -> protocol::MutationApplyResult<Vec<Puzzle2dTargetRegion>> {
    let patched: Vec<_> = delta.patched.iter().map(|entry| (entry.id.clone(), entry.patch.replacement.clone())).collect();
    apply_identified_delta(regions, &delta.removed, &delta.added, &patched, &delta.reordered, |r| &r.id)
}

/// 🧩 Applies an identified-collection delta to edges.
pub fn apply_edges_delta(edges: &[Puzzle2dEdge], delta: &Puzzle2dEdgesDelta) -> protocol::MutationApplyResult<Vec<Puzzle2dEdge>> {
    let patched: Vec<_> = delta.patched.iter().map(|entry| (entry.id.clone(), entry.patch.replacement.clone())).collect();
    apply_identified_delta(edges, &delta.removed, &delta.added, &patched, &delta.reordered, |e| &e.id)
}

impl MutationDiff<Puzzle2dSnapshot> for Puzzle2dDiff {
    fn apply(&self, snapshot: &Puzzle2dSnapshot) -> protocol::MutationApplyResult<Puzzle2dSnapshot> {
        Ok({
            if let Some(replacement) = &self.artifact {
                return Ok(replacement.to_snapshot());
            }
            let mut next = snapshot.clone();
            if let Some(schema) = &self.schema {
                next.schema = schema.clone();
            }
            if let Some(camera) = &self.camera {
                next.camera = camera.clone();
            }
            if let Some(delta) = &self.nodes {
                next.nodes = apply_nodes_delta(&next.nodes, delta).map_err(|error| error.under(["nodes"]))?;
            }
            if let Some(delta) = &self.edges {
                next.edges = apply_edges_delta(&next.edges, delta).map_err(|error| error.under(["edges"]))?;
            }
            if let Some(delta) = &self.target_regions {
                next.target_regions = apply_target_regions_delta(&next.target_regions, delta).map_err(|error| error.under(["targetRegions"]))?;
            }
            if let Some(meta) = &self.meta {
                next.meta = meta.clone();
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
            ($field:ident) => {
                if other.$field.is_some() {
                    self.$field = other.$field;
                }
            };
        }
        take!(schema);
        take!(camera);
        take!(meta);
        if let Some(delta) = other.nodes {
            match &mut self.nodes {
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
                None => self.nodes = Some(delta),
            }
        }
        if let Some(delta) = other.edges {
            match &mut self.edges {
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
                None => self.edges = Some(delta),
            }
        }
        if let Some(delta) = other.target_regions {
            match &mut self.target_regions {
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
                None => self.target_regions = Some(delta),
            }
        }
    }
}
