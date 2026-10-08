//! 🧬️ Puzzle2d diff schema — sparse typed delta over the artifact: per-field entity patches and id-keyed collection deltas.

use crate::standards::v1::subsets::any::schema::Puzzle2dArtifact;
use crate::{Puzzle2dCamera, Puzzle2dCompatSpecificity, Puzzle2dEdge, Puzzle2dHandle, Puzzle2dKindCatalogs, Puzzle2dKindCompatibility, Puzzle2dMeta, Puzzle2dNode, Puzzle2dNodeAnchor, Puzzle2dTargetRegion};
use crate::Puzzle2dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};
use semio_framework_value::{list::PagedList, paged::PagedUtf8};

//#region 🔖️Diff
/// 🔺️ Sparse typed delta for the puzzle2d artifact: per-field entity patches and id-keyed collection deltas.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle2d")]
pub struct Puzzle2dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<Puzzle2dArtifact>>,
    #[state(artifact)]
    pub schema: Option<PagedUtf8<{ usize::MAX }>>,
    #[state(artifact)]
    pub camera: Option<Puzzle2dCamera>,
    #[state(artifact)]
    pub nodes: Option<Puzzle2dNodesDelta>,
    #[state(artifact)]
    pub edges: Option<Puzzle2dEdgesDelta>,
    #[state(artifact)]
    pub target_regions: Option<Puzzle2dTargetRegionsDelta>,
    #[state(artifact)]
    pub meta: Option<Puzzle2dMetaPatch>,
}
//#endregion 🔖️Diff

//#region 🔖️Patches
/// 🔑️ The identity of one kind-compatibility row: the pair of kinds it links.
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dKindCompatibilityKey {
    pub source: PagedUtf8<{ usize::MAX }>,
    pub target: PagedUtf8<{ usize::MAX }>,
}

/// 🩹 Sparse per-field patch over one `Puzzle2dHandle` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dHandlePatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub handle_kind: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub radius: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub color: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub icon_kind: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scale: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub visible: Option<Option<bool>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub locked: Option<Option<bool>>,
}

/// 🩹 Sparse per-field patch over one `Puzzle2dNode` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dNodePatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub node_kind: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub shape: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub radius: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub width: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub height: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub text: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub icon_kind: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub root: Option<Option<bool>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scale: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub visible: Option<Option<bool>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub locked: Option<Option<bool>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<Puzzle2dNodeAnchor>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub handles: Option<Puzzle2dHandlesDelta>,
}

/// 🩹 Sparse per-field patch over one `Puzzle2dEdge` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dEdgePatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<PagedUtf8<{ usize::MAX }>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<PagedUtf8<{ usize::MAX }>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub edge_kind: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub gap: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub shift: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rise: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub turn: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tilt: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub source_tip: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub target_tip: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub visible: Option<Option<bool>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub locked: Option<Option<bool>>,
}

/// 🩹 Sparse per-field patch over one `Puzzle2dTargetRegion` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dTargetRegionPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub label: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// 🩹 Sparse per-field patch over one `Puzzle2dKindCompatibility` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dKindCompatibilityPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bidirectional: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub important: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub specificity: Option<Puzzle2dCompatSpecificity>,
}

/// 🩹 Sparse per-field patch over one `Puzzle2dMeta` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dMetaPatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub manifest_id: Option<Option<PagedUtf8<{ usize::MAX }>>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind_compatibility: Option<Puzzle2dKindCompatibilityDelta>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub kind_catalogs: Option<Option<Puzzle2dKindCatalogs>>,
}

//#endregion 🔖️Patches

//#region 🔖️Deltas
/// 🧩 Identified-collection delta for a node's `handles`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dHandlesDelta {
    pub added: Vec<Puzzle2dHandle>,
    pub removed: Vec<PagedUtf8<{ usize::MAX }>>,
    pub patched: Vec<Puzzle2dHandlePatchEntry>,
    pub reordered: Option<Vec<PagedUtf8<{ usize::MAX }>>>,
}

/// 🩹 One patched `Puzzle2dHandle` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dHandlePatchEntry {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub patch: Puzzle2dHandlePatch,
}

/// 🧩 Identified-collection delta for `nodes`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dNodesDelta {
    pub added: Vec<Puzzle2dNode>,
    pub removed: Vec<PagedUtf8<{ usize::MAX }>>,
    pub patched: Vec<Puzzle2dNodePatchEntry>,
    pub reordered: Option<Vec<PagedUtf8<{ usize::MAX }>>>,
}

/// 🩹 One patched `Puzzle2dNode` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dNodePatchEntry {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub patch: Puzzle2dNodePatch,
}

/// 🧩 Identified-collection delta for `edges`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dEdgesDelta {
    pub added: Vec<Puzzle2dEdge>,
    pub removed: Vec<PagedUtf8<{ usize::MAX }>>,
    pub patched: Vec<Puzzle2dEdgePatchEntry>,
    pub reordered: Option<Vec<PagedUtf8<{ usize::MAX }>>>,
}

/// 🩹 One patched `Puzzle2dEdge` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dEdgePatchEntry {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub patch: Puzzle2dEdgePatch,
}

/// 🧩 Identified-collection delta for `targetRegions`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dTargetRegionsDelta {
    pub added: Vec<Puzzle2dTargetRegion>,
    pub removed: Vec<PagedUtf8<{ usize::MAX }>>,
    pub patched: Vec<Puzzle2dTargetRegionPatchEntry>,
    pub reordered: Option<Vec<PagedUtf8<{ usize::MAX }>>>,
}

/// 🩹 One patched `Puzzle2dTargetRegion` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dTargetRegionPatchEntry {
    pub id: PagedUtf8<{ usize::MAX }>,
    pub patch: Puzzle2dTargetRegionPatch,
}

/// 🧩 Keyed-row delta for `meta.kindCompatibility` (a row is addressed by its source and target kinds).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle2dKindCompatibilityDelta {
    pub added: Vec<Puzzle2dKindCompatibility>,
    pub removed: Vec<Puzzle2dKindCompatibilityKey>,
    pub patched: Vec<Puzzle2dKindCompatibilityPatchEntry>,
    pub reordered: Option<Vec<Puzzle2dKindCompatibilityKey>>,
}

/// 🩹 One patched `Puzzle2dKindCompatibility` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle2dKindCompatibilityPatchEntry {
    pub id: Puzzle2dKindCompatibilityKey,
    pub patch: Puzzle2dKindCompatibilityPatch,
}

//#endregion 🔖️Deltas

//#region 🔖️Algebra
/// 🩹 Sparse per-field patch over one entity (or the whole snapshot): every method is a pure function of the patch and the
/// entity it addresses. Applying needs the central applier's capability; absorbing coalesces same-field entries so the
/// composite of two sequential patches is one patch.
pub trait ItemPatch<T>: Default + PartialEq + Sized {
    /// 🔑️ Writes the patched fields into `item` — callable only with the central applier's capability.
    fn apply_to(&self, item: &mut T, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()>;
    /// ➕️ Composes `self` then `later` into one patch: the later value wins per field, nested patches coalesce.
    fn absorb(&mut self, later: Self);
    /// 🔁️ The negative patch: restores exactly the fields `self` names to their `base` values.
    fn inverse(&self, base: &T) -> Self;
    /// 🧭️ The patch that carries `base` to `other`, naming only the fields that differ.
    fn between(base: &T, other: &T) -> Self;
    /// 🕳️ Whether the patch names no field.
    fn is_empty(&self) -> bool;
}

/// 🗂️ Owned-vec view of an ordered entity container.
pub trait ItemList<T> {
    fn items(&self) -> Vec<T>;
    fn from_items(items: Vec<T>) -> Self;
}

impl<T: Clone> ItemList<T> for Vec<T> {
    fn items(&self) -> Vec<T> {
        self.clone()
    }
    fn from_items(items: Vec<T>) -> Self {
        items
    }
}

impl<T: Clone> ItemList<T> for semio_framework_value::list::PagedList<T, { usize::MAX }> {
    fn items(&self) -> Vec<T> {
        self.iter().cloned().collect()
    }
    fn from_items(items: Vec<T>) -> Self {
        let mut list = Self::new();
        for item in items {
            list.push(item);
        }
        list
    }
}

/// 🕳️ Tri-state decode of every `Option<Option<T>>` patch slot: a missing key is the unchanged slot (`None`) and a PRESENT
/// `null` is the clear `Some(None)`, never the unchanged slot the blanket `Option<T>` decode would fold it into.
fn deserialize_double_option<T: semio_framework_value::FromValue>(value: semio_framework_value::DslValue) -> Result<Option<Option<T>>, semio_framework_value::ValueError> {
    <Option<T> as semio_framework_value::FromValue>::from_value(value).map(Some)
}

/// 🧩 Id-keyed collection algebra for one delta type: apply, absorb (create∘delete → nothing, delete∘create → replace,
/// patch∘patch → one patch), concrete inverse and snapshot-to-snapshot `between`.
macro_rules! keyed_delta {
    ($delta:ident, $entry:ident, $patch:ident, $item:ty, $id:ty, $list:ty, $key:path, $label:path) => {
        impl $delta {
            pub fn is_empty(&self) -> bool {
                self.added.is_empty() && self.removed.is_empty() && self.patched.is_empty() && self.reordered.is_none()
            }
            pub fn patching(id: $id, patch: $patch) -> Self {
                Self { patched: vec![$entry { id, patch }], ..Default::default() }
            }
            pub fn removing(removed: Vec<$id>) -> Self {
                Self { removed, ..Default::default() }
            }
            pub fn adding(item: $item, reordered: Option<Vec<$id>>) -> Self {
                Self { added: vec![item], reordered, ..Default::default() }
            }
            pub fn apply_to(&self, base: &$list, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<$list> {
                let refusal = |code: &str, message: &str, section: &str, id: &$id| protocol::MutationApplyError::new(code, message).at([section.to_owned(), $label(id)]);
                let mut items = <$list as ItemList<$item>>::items(base);
                let mut seen = std::collections::HashSet::<$id>::new();
                let mut index: std::collections::HashMap<$id, usize> = items.iter().enumerate().map(|(at, item)| ($key(item), at)).collect();
                for id in &self.removed {
                    if !seen.insert(id.clone()) {
                        return Err(refusal("mutation.apply.duplicate-target", "item is removed more than once", "removed", id));
                    }
                    if !index.contains_key(id) {
                        return Err(refusal("mutation.apply.missing-target", "removed item does not exist", "removed", id));
                    }
                }
                if !seen.is_empty() {
                    items.retain(|item| !seen.contains(&$key(item)));
                    index = items.iter().enumerate().map(|(at, item)| ($key(item), at)).collect();
                }
                seen.clear();
                for item in &self.added {
                    let id = $key(item);
                    if !seen.insert(id.clone()) || index.contains_key(&id) {
                        return Err(refusal("mutation.apply.duplicate-target", "added item identity already exists", "added", &id));
                    }
                    index.insert(id, items.len());
                    items.push(item.clone());
                }
                seen.clear();
                for entry in &self.patched {
                    if !seen.insert(entry.id.clone()) {
                        return Err(refusal("mutation.apply.duplicate-target", "item is patched more than once", "patched", &entry.id));
                    }
                    let Some(&at) = index.get(&entry.id) else {
                        return Err(refusal("mutation.apply.missing-target", "patched item does not exist", "patched", &entry.id));
                    };
                    entry.patch.apply_to(&mut items[at], capability).map_err(|error| error.under(["patched".to_owned(), $label(&entry.id)]))?;
                }
                if let Some(order) = &self.reordered {
                    if order.len() != items.len() {
                        return Err(protocol::MutationApplyError::new("mutation.apply.incomplete-diff", format!("order has length {}, expected {}", order.len(), items.len())).at(["reordered"]));
                    }
                    seen.clear();
                    let mut pool: Vec<Option<$item>> = items.into_iter().map(Some).collect();
                    let mut ordered = Vec::with_capacity(pool.len());
                    for id in order {
                        if !seen.insert(id.clone()) {
                            return Err(refusal("mutation.apply.duplicate-target", "item appears more than once in order", "reordered", id));
                        }
                        let Some(item) = index.get(id).and_then(|&at| pool[at].take()) else {
                            return Err(refusal("mutation.apply.missing-target", "ordered item does not exist", "reordered", id));
                        };
                        ordered.push(item);
                    }
                    items = ordered;
                }
                Ok(<$list as ItemList<$item>>::from_items(items))
            }
            pub fn absorb(&mut self, later: Self) {
                let Self { added, removed, patched, reordered } = later;
                for id in removed {
                    self.patched.retain(|entry| entry.id != id);
                    if let Some(order) = &mut self.reordered {
                        order.retain(|entry| entry != &id);
                    }
                    match self.added.iter().position(|item| $key(item) == id) {
                        Some(at) => {
                            self.added.remove(at);
                        }
                        None => self.removed.push(id),
                    }
                }
                for item in added {
                    if let Some(order) = &mut self.reordered {
                        order.push($key(&item));
                    }
                    self.added.push(item);
                }
                let mut slots: std::collections::HashMap<$id, usize> = self.patched.iter().enumerate().map(|(at, entry)| (entry.id.clone(), at)).collect();
                for entry in patched {
                    match slots.get(&entry.id) {
                        Some(&at) => self.patched[at].patch.absorb(entry.patch),
                        None => {
                            slots.insert(entry.id.clone(), self.patched.len());
                            self.patched.push(entry);
                        }
                    }
                }
                if reordered.is_some() {
                    self.reordered = reordered;
                }
            }
            pub fn inverse(&self, base: &$list) -> Self {
                let items = <$list as ItemList<$item>>::items(base);
                let by_id: std::collections::HashMap<$id, &$item> = items.iter().map(|item| ($key(item), item)).collect();
                let base_order: Vec<$id> = items.iter().map(|item| $key(item)).collect();
                let added_ids: Vec<$id> = self.added.iter().map(|item| $key(item)).collect();
                let added_set: std::collections::HashSet<&$id> = added_ids.iter().collect();
                let removed_set: std::collections::HashSet<&$id> = self.removed.iter().collect();
                let restored: Vec<$item> = self.removed.iter().filter_map(|id| by_id.get(id).map(|item| (*item).clone())).collect();
                let patched: Vec<$entry> = self.patched.iter().filter_map(|entry| by_id.get(&entry.id).map(|item| $entry { id: entry.id.clone(), patch: <$patch as ItemPatch<$item>>::inverse(&entry.patch, item) })).collect();
                let mut order: Vec<$id> = match &self.reordered {
                    Some(order) => order.clone(),
                    None => base_order.iter().filter(|id| !removed_set.contains(id)).chain(added_ids.iter()).cloned().collect(),
                };
                order.retain(|id| !added_set.contains(id));
                order.extend(restored.iter().map(|item| $key(item)));
                Self { added: restored, removed: added_ids, patched, reordered: (order != base_order).then_some(base_order) }
            }
            pub fn between(base: &$list, other: &$list) -> Self {
                let (from, to) = (<$list as ItemList<$item>>::items(base), <$list as ItemList<$item>>::items(other));
                let from_ids: std::collections::HashSet<$id> = from.iter().map(|item| $key(item)).collect();
                let to_by_id: std::collections::HashMap<$id, &$item> = to.iter().map(|item| ($key(item), item)).collect();
                let removed: Vec<$id> = from.iter().map(|item| $key(item)).filter(|id| !to_by_id.contains_key(id)).collect();
                let added: Vec<$item> = to.iter().filter(|item| !from_ids.contains(&$key(item))).cloned().collect();
                let patched: Vec<$entry> = from
                    .iter()
                    .filter_map(|item| {
                        let id = $key(item);
                        let patch = <$patch as ItemPatch<$item>>::between(item, to_by_id.get(&id)?);
                        (!patch.is_empty()).then(|| $entry { id, patch })
                    })
                    .collect();
                let natural: Vec<$id> = from.iter().map(|item| $key(item)).filter(|id| to_by_id.contains_key(id)).chain(added.iter().map(|item| $key(item))).collect();
                let target: Vec<$id> = to.iter().map(|item| $key(item)).collect();
                Self { added, removed, patched, reordered: (natural != target).then_some(target) }
            }
        }
    };
}
//#endregion 🔖️Algebra

//#region 🔖️Keys
fn handle_key(item: &Puzzle2dHandle) -> PagedUtf8<{ usize::MAX }> {
    item.id.clone()
}

fn node_key(item: &Puzzle2dNode) -> PagedUtf8<{ usize::MAX }> {
    item.id.clone()
}

fn edge_key(item: &Puzzle2dEdge) -> PagedUtf8<{ usize::MAX }> {
    item.id.clone()
}

fn target_region_key(item: &Puzzle2dTargetRegion) -> PagedUtf8<{ usize::MAX }> {
    item.id.clone()
}

fn kind_compatibility_key(item: &Puzzle2dKindCompatibility) -> Puzzle2dKindCompatibilityKey {
    Puzzle2dKindCompatibilityKey { source: item.source.clone(), target: item.target.clone() }
}

fn text_label(id: &PagedUtf8<{ usize::MAX }>) -> String {
    id.to_string_owner()
}

fn kind_compatibility_label(id: &Puzzle2dKindCompatibilityKey) -> String {
    format!("{} -> {}", id.source, id.target)
}

//#endregion 🔖️Keys

keyed_delta!(Puzzle2dHandlesDelta, Puzzle2dHandlePatchEntry, Puzzle2dHandlePatch, Puzzle2dHandle, PagedUtf8<{ usize::MAX }>, PagedList<Puzzle2dHandle, { usize::MAX }>, handle_key, text_label);
keyed_delta!(Puzzle2dNodesDelta, Puzzle2dNodePatchEntry, Puzzle2dNodePatch, Puzzle2dNode, PagedUtf8<{ usize::MAX }>, PagedList<Puzzle2dNode, { usize::MAX }>, node_key, text_label);
keyed_delta!(Puzzle2dEdgesDelta, Puzzle2dEdgePatchEntry, Puzzle2dEdgePatch, Puzzle2dEdge, PagedUtf8<{ usize::MAX }>, PagedList<Puzzle2dEdge, { usize::MAX }>, edge_key, text_label);
keyed_delta!(Puzzle2dTargetRegionsDelta, Puzzle2dTargetRegionPatchEntry, Puzzle2dTargetRegionPatch, Puzzle2dTargetRegion, PagedUtf8<{ usize::MAX }>, PagedList<Puzzle2dTargetRegion, { usize::MAX }>, target_region_key, text_label);
keyed_delta!(Puzzle2dKindCompatibilityDelta, Puzzle2dKindCompatibilityPatchEntry, Puzzle2dKindCompatibilityPatch, Puzzle2dKindCompatibility, Puzzle2dKindCompatibilityKey, PagedList<Puzzle2dKindCompatibility, { usize::MAX }>, kind_compatibility_key, kind_compatibility_label);

impl ItemPatch<Puzzle2dHandle> for Puzzle2dHandlePatch {
    fn apply_to(&self, item: &mut Puzzle2dHandle, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.handle_kind {
            item.handle_kind = value.clone();
        }
        if let Some(value) = &self.angle {
            item.angle = *value;
        }
        if let Some(value) = &self.radius {
            item.radius = *value;
        }
        if let Some(value) = &self.color {
            item.color = value.clone();
        }
        if let Some(value) = &self.icon_kind {
            item.icon_kind = value.clone();
        }
        if let Some(value) = &self.scale {
            item.scale = *value;
        }
        if let Some(value) = &self.visible {
            item.visible = *value;
        }
        if let Some(value) = &self.locked {
            item.locked = *value;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.handle_kind.is_some() {
            self.handle_kind = later.handle_kind;
        }
        if later.angle.is_some() {
            self.angle = later.angle;
        }
        if later.radius.is_some() {
            self.radius = later.radius;
        }
        if later.color.is_some() {
            self.color = later.color;
        }
        if later.icon_kind.is_some() {
            self.icon_kind = later.icon_kind;
        }
        if later.scale.is_some() {
            self.scale = later.scale;
        }
        if later.visible.is_some() {
            self.visible = later.visible;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
    }
    fn inverse(&self, base: &Puzzle2dHandle) -> Self {
        Self {
            handle_kind: self.handle_kind.as_ref().map(|_| base.handle_kind.clone()),
            angle: self.angle.as_ref().map(|_| base.angle),
            radius: self.radius.as_ref().map(|_| base.radius),
            color: self.color.as_ref().map(|_| base.color.clone()),
            icon_kind: self.icon_kind.as_ref().map(|_| base.icon_kind.clone()),
            scale: self.scale.as_ref().map(|_| base.scale),
            visible: self.visible.as_ref().map(|_| base.visible),
            locked: self.locked.as_ref().map(|_| base.locked),
        }
    }
    fn between(base: &Puzzle2dHandle, other: &Puzzle2dHandle) -> Self {
        Self {
            handle_kind: (base.handle_kind != other.handle_kind).then(|| other.handle_kind.clone()),
            angle: (base.angle != other.angle).then(|| other.angle),
            radius: (base.radius != other.radius).then(|| other.radius),
            color: (base.color != other.color).then(|| other.color.clone()),
            icon_kind: (base.icon_kind != other.icon_kind).then(|| other.icon_kind.clone()),
            scale: (base.scale != other.scale).then(|| other.scale),
            visible: (base.visible != other.visible).then(|| other.visible),
            locked: (base.locked != other.locked).then(|| other.locked),
        }
    }
    fn is_empty(&self) -> bool {
        self.handle_kind.is_none() && self.angle.is_none() && self.radius.is_none() && self.color.is_none() && self.icon_kind.is_none() && self.scale.is_none() && self.visible.is_none() && self.locked.is_none()
    }
}

impl ItemPatch<Puzzle2dNode> for Puzzle2dNodePatch {
    fn apply_to(&self, item: &mut Puzzle2dNode, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.node_kind {
            item.node_kind = value.clone();
        }
        if let Some(value) = &self.shape {
            item.shape = value.clone();
        }
        if let Some(value) = &self.x {
            item.x = *value;
        }
        if let Some(value) = &self.y {
            item.y = *value;
        }
        if let Some(value) = &self.radius {
            item.radius = *value;
        }
        if let Some(value) = &self.width {
            item.width = *value;
        }
        if let Some(value) = &self.height {
            item.height = *value;
        }
        if let Some(value) = &self.text {
            item.text = value.clone();
        }
        if let Some(value) = &self.icon_kind {
            item.icon_kind = value.clone();
        }
        if let Some(value) = &self.root {
            item.root = *value;
        }
        if let Some(value) = &self.scale {
            item.scale = *value;
        }
        if let Some(value) = &self.visible {
            item.visible = *value;
        }
        if let Some(value) = &self.locked {
            item.locked = *value;
        }
        if let Some(value) = &self.anchor {
            item.anchor = *value;
        }
        if let Some(delta) = &self.handles {
            item.handles = delta.apply_to(&item.handles, capability).map_err(|error| error.under(["handles"]))?;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.node_kind.is_some() {
            self.node_kind = later.node_kind;
        }
        if later.shape.is_some() {
            self.shape = later.shape;
        }
        if later.x.is_some() {
            self.x = later.x;
        }
        if later.y.is_some() {
            self.y = later.y;
        }
        if later.radius.is_some() {
            self.radius = later.radius;
        }
        if later.width.is_some() {
            self.width = later.width;
        }
        if later.height.is_some() {
            self.height = later.height;
        }
        if later.text.is_some() {
            self.text = later.text;
        }
        if later.icon_kind.is_some() {
            self.icon_kind = later.icon_kind;
        }
        if later.root.is_some() {
            self.root = later.root;
        }
        if later.scale.is_some() {
            self.scale = later.scale;
        }
        if later.visible.is_some() {
            self.visible = later.visible;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
        if later.anchor.is_some() {
            self.anchor = later.anchor;
        }
        match (&mut self.handles, later.handles) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
    }
    fn inverse(&self, base: &Puzzle2dNode) -> Self {
        Self {
            node_kind: self.node_kind.as_ref().map(|_| base.node_kind.clone()),
            shape: self.shape.as_ref().map(|_| base.shape.clone()),
            x: self.x.as_ref().map(|_| base.x),
            y: self.y.as_ref().map(|_| base.y),
            radius: self.radius.as_ref().map(|_| base.radius),
            width: self.width.as_ref().map(|_| base.width),
            height: self.height.as_ref().map(|_| base.height),
            text: self.text.as_ref().map(|_| base.text.clone()),
            icon_kind: self.icon_kind.as_ref().map(|_| base.icon_kind.clone()),
            root: self.root.as_ref().map(|_| base.root),
            scale: self.scale.as_ref().map(|_| base.scale),
            visible: self.visible.as_ref().map(|_| base.visible),
            locked: self.locked.as_ref().map(|_| base.locked),
            anchor: self.anchor.as_ref().map(|_| base.anchor),
            handles: self.handles.as_ref().map(|delta| delta.inverse(&base.handles)),
        }
    }
    fn between(base: &Puzzle2dNode, other: &Puzzle2dNode) -> Self {
        Self {
            node_kind: (base.node_kind != other.node_kind).then(|| other.node_kind.clone()),
            shape: (base.shape != other.shape).then(|| other.shape.clone()),
            x: (base.x != other.x).then(|| other.x),
            y: (base.y != other.y).then(|| other.y),
            radius: (base.radius != other.radius).then(|| other.radius),
            width: (base.width != other.width).then(|| other.width),
            height: (base.height != other.height).then(|| other.height),
            text: (base.text != other.text).then(|| other.text.clone()),
            icon_kind: (base.icon_kind != other.icon_kind).then(|| other.icon_kind.clone()),
            root: (base.root != other.root).then(|| other.root),
            scale: (base.scale != other.scale).then(|| other.scale),
            visible: (base.visible != other.visible).then(|| other.visible),
            locked: (base.locked != other.locked).then(|| other.locked),
            anchor: (base.anchor != other.anchor).then(|| other.anchor),
            handles: Some(Puzzle2dHandlesDelta::between(&base.handles, &other.handles)).filter(|delta| !delta.is_empty()),
        }
    }
    fn is_empty(&self) -> bool {
        self.node_kind.is_none() && self.shape.is_none() && self.x.is_none() && self.y.is_none() && self.radius.is_none() && self.width.is_none() && self.height.is_none() && self.text.is_none() && self.icon_kind.is_none() && self.root.is_none() && self.scale.is_none() && self.visible.is_none() && self.locked.is_none() && self.anchor.is_none() && self.handles.as_ref().is_none_or(Puzzle2dHandlesDelta::is_empty)
    }
}

impl ItemPatch<Puzzle2dEdge> for Puzzle2dEdgePatch {
    fn apply_to(&self, item: &mut Puzzle2dEdge, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.source {
            item.source = value.clone();
        }
        if let Some(value) = &self.target {
            item.target = value.clone();
        }
        if let Some(value) = &self.edge_kind {
            item.edge_kind = value.clone();
        }
        if let Some(value) = &self.gap {
            item.gap = *value;
        }
        if let Some(value) = &self.shift {
            item.shift = *value;
        }
        if let Some(value) = &self.rise {
            item.rise = *value;
        }
        if let Some(value) = &self.rotation {
            item.rotation = *value;
        }
        if let Some(value) = &self.turn {
            item.turn = *value;
        }
        if let Some(value) = &self.tilt {
            item.tilt = *value;
        }
        if let Some(value) = &self.x {
            item.x = *value;
        }
        if let Some(value) = &self.y {
            item.y = *value;
        }
        if let Some(value) = &self.source_tip {
            item.source_tip = value.clone();
        }
        if let Some(value) = &self.target_tip {
            item.target_tip = value.clone();
        }
        if let Some(value) = &self.visible {
            item.visible = *value;
        }
        if let Some(value) = &self.locked {
            item.locked = *value;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.source.is_some() {
            self.source = later.source;
        }
        if later.target.is_some() {
            self.target = later.target;
        }
        if later.edge_kind.is_some() {
            self.edge_kind = later.edge_kind;
        }
        if later.gap.is_some() {
            self.gap = later.gap;
        }
        if later.shift.is_some() {
            self.shift = later.shift;
        }
        if later.rise.is_some() {
            self.rise = later.rise;
        }
        if later.rotation.is_some() {
            self.rotation = later.rotation;
        }
        if later.turn.is_some() {
            self.turn = later.turn;
        }
        if later.tilt.is_some() {
            self.tilt = later.tilt;
        }
        if later.x.is_some() {
            self.x = later.x;
        }
        if later.y.is_some() {
            self.y = later.y;
        }
        if later.source_tip.is_some() {
            self.source_tip = later.source_tip;
        }
        if later.target_tip.is_some() {
            self.target_tip = later.target_tip;
        }
        if later.visible.is_some() {
            self.visible = later.visible;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
    }
    fn inverse(&self, base: &Puzzle2dEdge) -> Self {
        Self {
            source: self.source.as_ref().map(|_| base.source.clone()),
            target: self.target.as_ref().map(|_| base.target.clone()),
            edge_kind: self.edge_kind.as_ref().map(|_| base.edge_kind.clone()),
            gap: self.gap.as_ref().map(|_| base.gap),
            shift: self.shift.as_ref().map(|_| base.shift),
            rise: self.rise.as_ref().map(|_| base.rise),
            rotation: self.rotation.as_ref().map(|_| base.rotation),
            turn: self.turn.as_ref().map(|_| base.turn),
            tilt: self.tilt.as_ref().map(|_| base.tilt),
            x: self.x.as_ref().map(|_| base.x),
            y: self.y.as_ref().map(|_| base.y),
            source_tip: self.source_tip.as_ref().map(|_| base.source_tip.clone()),
            target_tip: self.target_tip.as_ref().map(|_| base.target_tip.clone()),
            visible: self.visible.as_ref().map(|_| base.visible),
            locked: self.locked.as_ref().map(|_| base.locked),
        }
    }
    fn between(base: &Puzzle2dEdge, other: &Puzzle2dEdge) -> Self {
        Self {
            source: (base.source != other.source).then(|| other.source.clone()),
            target: (base.target != other.target).then(|| other.target.clone()),
            edge_kind: (base.edge_kind != other.edge_kind).then(|| other.edge_kind.clone()),
            gap: (base.gap != other.gap).then(|| other.gap),
            shift: (base.shift != other.shift).then(|| other.shift),
            rise: (base.rise != other.rise).then(|| other.rise),
            rotation: (base.rotation != other.rotation).then(|| other.rotation),
            turn: (base.turn != other.turn).then(|| other.turn),
            tilt: (base.tilt != other.tilt).then(|| other.tilt),
            x: (base.x != other.x).then(|| other.x),
            y: (base.y != other.y).then(|| other.y),
            source_tip: (base.source_tip != other.source_tip).then(|| other.source_tip.clone()),
            target_tip: (base.target_tip != other.target_tip).then(|| other.target_tip.clone()),
            visible: (base.visible != other.visible).then(|| other.visible),
            locked: (base.locked != other.locked).then(|| other.locked),
        }
    }
    fn is_empty(&self) -> bool {
        self.source.is_none() && self.target.is_none() && self.edge_kind.is_none() && self.gap.is_none() && self.shift.is_none() && self.rise.is_none() && self.rotation.is_none() && self.turn.is_none() && self.tilt.is_none() && self.x.is_none() && self.y.is_none() && self.source_tip.is_none() && self.target_tip.is_none() && self.visible.is_none() && self.locked.is_none()
    }
}

impl ItemPatch<Puzzle2dTargetRegion> for Puzzle2dTargetRegionPatch {
    fn apply_to(&self, item: &mut Puzzle2dTargetRegion, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.x {
            item.x = *value;
        }
        if let Some(value) = &self.y {
            item.y = *value;
        }
        if let Some(value) = &self.width {
            item.width = *value;
        }
        if let Some(value) = &self.height {
            item.height = *value;
        }
        if let Some(value) = &self.label {
            item.label = value.clone();
        }
        if let Some(value) = &self.hidden {
            item.hidden = *value;
        }
        if let Some(value) = &self.locked {
            item.locked = *value;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.x.is_some() {
            self.x = later.x;
        }
        if later.y.is_some() {
            self.y = later.y;
        }
        if later.width.is_some() {
            self.width = later.width;
        }
        if later.height.is_some() {
            self.height = later.height;
        }
        if later.label.is_some() {
            self.label = later.label;
        }
        if later.hidden.is_some() {
            self.hidden = later.hidden;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
    }
    fn inverse(&self, base: &Puzzle2dTargetRegion) -> Self {
        Self {
            x: self.x.as_ref().map(|_| base.x),
            y: self.y.as_ref().map(|_| base.y),
            width: self.width.as_ref().map(|_| base.width),
            height: self.height.as_ref().map(|_| base.height),
            label: self.label.as_ref().map(|_| base.label.clone()),
            hidden: self.hidden.as_ref().map(|_| base.hidden),
            locked: self.locked.as_ref().map(|_| base.locked),
        }
    }
    fn between(base: &Puzzle2dTargetRegion, other: &Puzzle2dTargetRegion) -> Self {
        Self {
            x: (base.x != other.x).then(|| other.x),
            y: (base.y != other.y).then(|| other.y),
            width: (base.width != other.width).then(|| other.width),
            height: (base.height != other.height).then(|| other.height),
            label: (base.label != other.label).then(|| other.label.clone()),
            hidden: (base.hidden != other.hidden).then(|| other.hidden),
            locked: (base.locked != other.locked).then(|| other.locked),
        }
    }
    fn is_empty(&self) -> bool {
        self.x.is_none() && self.y.is_none() && self.width.is_none() && self.height.is_none() && self.label.is_none() && self.hidden.is_none() && self.locked.is_none()
    }
}

impl ItemPatch<Puzzle2dKindCompatibility> for Puzzle2dKindCompatibilityPatch {
    fn apply_to(&self, item: &mut Puzzle2dKindCompatibility, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.bidirectional {
            item.bidirectional = *value;
        }
        if let Some(value) = &self.important {
            item.important = *value;
        }
        if let Some(value) = &self.specificity {
            item.specificity = *value;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.bidirectional.is_some() {
            self.bidirectional = later.bidirectional;
        }
        if later.important.is_some() {
            self.important = later.important;
        }
        if later.specificity.is_some() {
            self.specificity = later.specificity;
        }
    }
    fn inverse(&self, base: &Puzzle2dKindCompatibility) -> Self {
        Self {
            bidirectional: self.bidirectional.as_ref().map(|_| base.bidirectional),
            important: self.important.as_ref().map(|_| base.important),
            specificity: self.specificity.as_ref().map(|_| base.specificity),
        }
    }
    fn between(base: &Puzzle2dKindCompatibility, other: &Puzzle2dKindCompatibility) -> Self {
        Self {
            bidirectional: (base.bidirectional != other.bidirectional).then(|| other.bidirectional),
            important: (base.important != other.important).then(|| other.important),
            specificity: (base.specificity != other.specificity).then(|| other.specificity),
        }
    }
    fn is_empty(&self) -> bool {
        self.bidirectional.is_none() && self.important.is_none() && self.specificity.is_none()
    }
}

impl ItemPatch<Puzzle2dMeta> for Puzzle2dMetaPatch {
    fn apply_to(&self, item: &mut Puzzle2dMeta, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.manifest_id {
            item.manifest_id = value.clone();
        }
        if let Some(delta) = &self.kind_compatibility {
            item.kind_compatibility = delta.apply_to(&item.kind_compatibility, capability).map_err(|error| error.under(["kindCompatibility"]))?;
        }
        if let Some(value) = &self.kind_catalogs {
            item.kind_catalogs = value.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.manifest_id.is_some() {
            self.manifest_id = later.manifest_id;
        }
        match (&mut self.kind_compatibility, later.kind_compatibility) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        if later.kind_catalogs.is_some() {
            self.kind_catalogs = later.kind_catalogs;
        }
    }
    fn inverse(&self, base: &Puzzle2dMeta) -> Self {
        Self {
            manifest_id: self.manifest_id.as_ref().map(|_| base.manifest_id.clone()),
            kind_compatibility: self.kind_compatibility.as_ref().map(|delta| delta.inverse(&base.kind_compatibility)),
            kind_catalogs: self.kind_catalogs.as_ref().map(|_| base.kind_catalogs.clone()),
        }
    }
    fn between(base: &Puzzle2dMeta, other: &Puzzle2dMeta) -> Self {
        Self {
            manifest_id: (base.manifest_id != other.manifest_id).then(|| other.manifest_id.clone()),
            kind_compatibility: Some(Puzzle2dKindCompatibilityDelta::between(&base.kind_compatibility, &other.kind_compatibility)).filter(|delta| !delta.is_empty()),
            kind_catalogs: (base.kind_catalogs != other.kind_catalogs).then(|| other.kind_catalogs.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.manifest_id.is_none() && self.kind_compatibility.as_ref().is_none_or(Puzzle2dKindCompatibilityDelta::is_empty) && self.kind_catalogs.is_none()
    }
}

impl MutationDiff<Puzzle2dSnapshot> for Puzzle2dDiff {
    fn apply(&self, base: &Puzzle2dSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle2dSnapshot> {
        let mut item = match &self.artifact {
            Some(artifact) => artifact.to_snapshot(),
            None => base.clone(),
        };
        if let Some(value) = &self.schema {
            item.schema = value.clone();
        }
        if let Some(value) = &self.camera {
            item.camera = value.clone();
        }
        if let Some(delta) = &self.nodes {
            item.nodes = delta.apply_to(&item.nodes, capability).map_err(|error| error.under(["nodes"]))?;
        }
        if let Some(delta) = &self.edges {
            item.edges = delta.apply_to(&item.edges, capability).map_err(|error| error.under(["edges"]))?;
        }
        if let Some(delta) = &self.target_regions {
            item.target_regions = delta.apply_to(&item.target_regions, capability).map_err(|error| error.under(["targetRegions"]))?;
        }
        if let Some(patch) = &self.meta {
            patch.apply_to(&mut item.meta, capability).map_err(|error| error.under(["meta"]))?;
        }
        Ok(item)
    }
    fn absorb(&mut self, later: Self) {
        if later.artifact.is_some() {
            *self = later;
            return;
        }
        if later.schema.is_some() {
            self.schema = later.schema;
        }
        if later.camera.is_some() {
            self.camera = later.camera;
        }
        match (&mut self.nodes, later.nodes) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.edges, later.edges) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.target_regions, later.target_regions) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.meta, later.meta) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
    }
}

impl DiffAlgebra<Puzzle2dSnapshot> for Puzzle2dDiff {
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Self {
        if self.artifact.is_some() {
            return Self { artifact: Some(Box::new(Puzzle2dArtifact::from_snapshot(base.clone()))), ..Default::default() };
        }
        Self {
            artifact: None,
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            camera: self.camera.as_ref().map(|_| base.camera.clone()),
            nodes: self.nodes.as_ref().map(|delta| delta.inverse(&base.nodes)),
            edges: self.edges.as_ref().map(|delta| delta.inverse(&base.edges)),
            target_regions: self.target_regions.as_ref().map(|delta| delta.inverse(&base.target_regions)),
            meta: self.meta.as_ref().map(|patch| patch.inverse(&base.meta)),
        }
    }
    fn between(base: &Puzzle2dSnapshot, other: &Puzzle2dSnapshot) -> Self {
        Self {
            artifact: None,
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            camera: (base.camera != other.camera).then(|| other.camera.clone()),
            nodes: Some(Puzzle2dNodesDelta::between(&base.nodes, &other.nodes)).filter(|delta| !delta.is_empty()),
            edges: Some(Puzzle2dEdgesDelta::between(&base.edges, &other.edges)).filter(|delta| !delta.is_empty()),
            target_regions: Some(Puzzle2dTargetRegionsDelta::between(&base.target_regions, &other.target_regions)).filter(|delta| !delta.is_empty()),
            meta: Some(<Puzzle2dMetaPatch as ItemPatch<_>>::between(&base.meta, &other.meta)).filter(|patch| !patch.is_empty()),
        }
    }
    fn is_empty(&self) -> bool {
        self.artifact.is_none() && self.schema.is_none() && self.camera.is_none() && self.nodes.as_ref().is_none_or(Puzzle2dNodesDelta::is_empty) && self.edges.as_ref().is_none_or(Puzzle2dEdgesDelta::is_empty) && self.target_regions.as_ref().is_none_or(Puzzle2dTargetRegionsDelta::is_empty) && self.meta.as_ref().is_none_or(|patch| patch.is_empty())
    }
}
