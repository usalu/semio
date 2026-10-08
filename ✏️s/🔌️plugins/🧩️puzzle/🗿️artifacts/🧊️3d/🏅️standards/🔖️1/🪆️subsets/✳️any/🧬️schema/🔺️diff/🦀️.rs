//! 🧬️ Puzzle3d diff schema — sparse typed delta over the artifact: per-field entity patches and id-keyed collection deltas.

use crate::standards::v1::subsets::any::schema::Puzzle3dArtifact;
use crate::{Puzzle3dAttraction, Puzzle3dCompatSpecificity, Puzzle3dKindCatalogs, Puzzle3dKindCompatibility, Puzzle3dMeta, Puzzle3dObject, Puzzle3dObjectAnchor, Puzzle3dReference, Puzzle3dReferenceSource, Puzzle3dScale, Puzzle3dTargetVolume, Puzzle3dVortex};
use crate::Puzzle3dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};

//#region 🔖️Diff
/// 🔺️ Sparse typed delta for the puzzle3d artifact: per-field entity patches and id-keyed collection deltas.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle3d")]
pub struct Puzzle3dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<Puzzle3dArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub domain: Option<String>,
    #[state(artifact)]
    pub meta: Option<Puzzle3dMetaPatch>,
    #[state(artifact)]
    pub objects: Option<Puzzle3dObjectsDelta>,
    #[state(artifact)]
    pub attractions: Option<Puzzle3dAttractionsDelta>,
    #[state(artifact)]
    pub target_volumes: Option<Puzzle3dTargetVolumesDelta>,
    #[state(artifact)]
    pub references: Option<Puzzle3dReferencesDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️Patches
/// 🔑️ The identity of one kind-compatibility row: the pair of kinds it links.
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dKindCompatibilityKey {
    pub source: String,
    pub target: String,
}

/// 🩹 Sparse per-field patch over one `Puzzle3dVortex` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dVortexPatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub vortex_kind: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub label: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub direction: Option<Option<[f64; 3]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub radius: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// 🩹 Sparse per-field patch over one `Puzzle3dObject` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dObjectPatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub label: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub object_kind: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<Puzzle3dObjectAnchor>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub orientation: Option<Option<[f64; 4]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scale: Option<Option<Puzzle3dScale>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub mesh_url: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub vortices: Option<Puzzle3dVorticesDelta>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// 🩹 Sparse per-field patch over one `Puzzle3dAttraction` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dAttractionPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub attracting: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub attracted: Option<String>,
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
}

/// 🩹 Sparse per-field patch over one `Puzzle3dTargetVolume` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dTargetVolumePatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub orientation: Option<Option<[f64; 4]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scale: Option<Option<Puzzle3dScale>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// 🩹 Sparse per-field patch over one `Puzzle3dReference` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dReferencePatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<Puzzle3dReferenceSource>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub width_world: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
}

/// 🩹 Sparse per-field patch over one `Puzzle3dKindCompatibility` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dKindCompatibilityPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bidirectional: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub important: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub specificity: Option<Puzzle3dCompatSpecificity>,
}

/// 🩹 Sparse per-field patch over one `Puzzle3dMeta` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dMetaPatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub kind_catalogs: Option<Option<Puzzle3dKindCatalogs>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub kind_compatibility: Option<Puzzle3dKindCompatibilityDelta>,
}

//#endregion 🔖️Patches

//#region 🔖️Deltas
/// 🧩 Identified-collection delta for an object's `vortices`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dVorticesDelta {
    pub added: Vec<Puzzle3dVortex>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle3dVortexPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle3dVortex` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dVortexPatchEntry {
    pub id: String,
    pub patch: Puzzle3dVortexPatch,
}

/// 🧩 Identified-collection delta for `objects`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dObjectsDelta {
    pub added: Vec<Puzzle3dObject>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle3dObjectPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle3dObject` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dObjectPatchEntry {
    pub id: String,
    pub patch: Puzzle3dObjectPatch,
}

/// 🧩 Identified-collection delta for `attractions`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dAttractionsDelta {
    pub added: Vec<Puzzle3dAttraction>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle3dAttractionPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle3dAttraction` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dAttractionPatchEntry {
    pub id: String,
    pub patch: Puzzle3dAttractionPatch,
}

/// 🧩 Identified-collection delta for `targetVolumes`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dTargetVolumesDelta {
    pub added: Vec<Puzzle3dTargetVolume>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle3dTargetVolumePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle3dTargetVolume` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dTargetVolumePatchEntry {
    pub id: String,
    pub patch: Puzzle3dTargetVolumePatch,
}

/// 🧩 Identified-collection delta for `references`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dReferencesDelta {
    pub added: Vec<Puzzle3dReference>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle3dReferencePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle3dReference` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dReferencePatchEntry {
    pub id: String,
    pub patch: Puzzle3dReferencePatch,
}

/// 🧩 Keyed-row delta for `meta.kindCompatibility` (a row is addressed by its source and target kinds).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle3dKindCompatibilityDelta {
    pub added: Vec<Puzzle3dKindCompatibility>,
    pub removed: Vec<Puzzle3dKindCompatibilityKey>,
    pub patched: Vec<Puzzle3dKindCompatibilityPatchEntry>,
    pub reordered: Option<Vec<Puzzle3dKindCompatibilityKey>>,
}

/// 🩹 One patched `Puzzle3dKindCompatibility` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle3dKindCompatibilityPatchEntry {
    pub id: Puzzle3dKindCompatibilityKey,
    pub patch: Puzzle3dKindCompatibilityPatch,
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
fn vortex_key(item: &Puzzle3dVortex) -> String {
    item.id.clone()
}

fn object_key(item: &Puzzle3dObject) -> String {
    item.id.clone()
}

fn attraction_key(item: &Puzzle3dAttraction) -> String {
    item.id.clone()
}

fn target_volume_key(item: &Puzzle3dTargetVolume) -> String {
    item.id.clone()
}

fn reference_key(item: &Puzzle3dReference) -> String {
    item.id.clone()
}

fn kind_compatibility_key(item: &Puzzle3dKindCompatibility) -> Puzzle3dKindCompatibilityKey {
    Puzzle3dKindCompatibilityKey { source: item.source.clone(), target: item.target.clone() }
}

fn text_label(id: &String) -> String {
    id.clone()
}

fn kind_compatibility_label(id: &Puzzle3dKindCompatibilityKey) -> String {
    format!("{} -> {}", id.source, id.target)
}

//#endregion 🔖️Keys

keyed_delta!(Puzzle3dVorticesDelta, Puzzle3dVortexPatchEntry, Puzzle3dVortexPatch, Puzzle3dVortex, String, Vec<Puzzle3dVortex>, vortex_key, text_label);
keyed_delta!(Puzzle3dObjectsDelta, Puzzle3dObjectPatchEntry, Puzzle3dObjectPatch, Puzzle3dObject, String, Vec<Puzzle3dObject>, object_key, text_label);
keyed_delta!(Puzzle3dAttractionsDelta, Puzzle3dAttractionPatchEntry, Puzzle3dAttractionPatch, Puzzle3dAttraction, String, Vec<Puzzle3dAttraction>, attraction_key, text_label);
keyed_delta!(Puzzle3dTargetVolumesDelta, Puzzle3dTargetVolumePatchEntry, Puzzle3dTargetVolumePatch, Puzzle3dTargetVolume, String, Vec<Puzzle3dTargetVolume>, target_volume_key, text_label);
keyed_delta!(Puzzle3dReferencesDelta, Puzzle3dReferencePatchEntry, Puzzle3dReferencePatch, Puzzle3dReference, String, Vec<Puzzle3dReference>, reference_key, text_label);
keyed_delta!(Puzzle3dKindCompatibilityDelta, Puzzle3dKindCompatibilityPatchEntry, Puzzle3dKindCompatibilityPatch, Puzzle3dKindCompatibility, Puzzle3dKindCompatibilityKey, Vec<Puzzle3dKindCompatibility>, kind_compatibility_key, kind_compatibility_label);

impl ItemPatch<Puzzle3dVortex> for Puzzle3dVortexPatch {
    fn apply_to(&self, item: &mut Puzzle3dVortex, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.vortex_kind {
            item.vortex_kind = value.clone();
        }
        if let Some(value) = &self.label {
            item.label = value.clone();
        }
        if let Some(value) = &self.position {
            item.position = *value;
        }
        if let Some(value) = &self.direction {
            item.direction = *value;
        }
        if let Some(value) = &self.radius {
            item.radius = *value;
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
        if later.vortex_kind.is_some() {
            self.vortex_kind = later.vortex_kind;
        }
        if later.label.is_some() {
            self.label = later.label;
        }
        if later.position.is_some() {
            self.position = later.position;
        }
        if later.direction.is_some() {
            self.direction = later.direction;
        }
        if later.radius.is_some() {
            self.radius = later.radius;
        }
        if later.hidden.is_some() {
            self.hidden = later.hidden;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
    }
    fn inverse(&self, base: &Puzzle3dVortex) -> Self {
        Self {
            vortex_kind: self.vortex_kind.as_ref().map(|_| base.vortex_kind.clone()),
            label: self.label.as_ref().map(|_| base.label.clone()),
            position: self.position.as_ref().map(|_| base.position),
            direction: self.direction.as_ref().map(|_| base.direction),
            radius: self.radius.as_ref().map(|_| base.radius),
            hidden: self.hidden.as_ref().map(|_| base.hidden),
            locked: self.locked.as_ref().map(|_| base.locked),
        }
    }
    fn between(base: &Puzzle3dVortex, other: &Puzzle3dVortex) -> Self {
        Self {
            vortex_kind: (base.vortex_kind != other.vortex_kind).then(|| other.vortex_kind.clone()),
            label: (base.label != other.label).then(|| other.label.clone()),
            position: (base.position != other.position).then(|| other.position),
            direction: (base.direction != other.direction).then(|| other.direction),
            radius: (base.radius != other.radius).then(|| other.radius),
            hidden: (base.hidden != other.hidden).then(|| other.hidden),
            locked: (base.locked != other.locked).then(|| other.locked),
        }
    }
    fn is_empty(&self) -> bool {
        self.vortex_kind.is_none() && self.label.is_none() && self.position.is_none() && self.direction.is_none() && self.radius.is_none() && self.hidden.is_none() && self.locked.is_none()
    }
}

impl ItemPatch<Puzzle3dObject> for Puzzle3dObjectPatch {
    fn apply_to(&self, item: &mut Puzzle3dObject, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.label {
            item.label = value.clone();
        }
        if let Some(value) = &self.object_kind {
            item.object_kind = value.clone();
        }
        if let Some(value) = &self.anchor {
            item.anchor = *value;
        }
        if let Some(value) = &self.origin {
            item.origin = *value;
        }
        if let Some(value) = &self.orientation {
            item.orientation = *value;
        }
        if let Some(value) = &self.scale {
            item.scale = value.clone();
        }
        if let Some(value) = &self.mesh_url {
            item.mesh_url = value.clone();
        }
        if let Some(delta) = &self.vortices {
            item.vortices = delta.apply_to(&item.vortices, capability).map_err(|error| error.under(["vortices"]))?;
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
        if later.label.is_some() {
            self.label = later.label;
        }
        if later.object_kind.is_some() {
            self.object_kind = later.object_kind;
        }
        if later.anchor.is_some() {
            self.anchor = later.anchor;
        }
        if later.origin.is_some() {
            self.origin = later.origin;
        }
        if later.orientation.is_some() {
            self.orientation = later.orientation;
        }
        if later.scale.is_some() {
            self.scale = later.scale;
        }
        if later.mesh_url.is_some() {
            self.mesh_url = later.mesh_url;
        }
        match (&mut self.vortices, later.vortices) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        if later.hidden.is_some() {
            self.hidden = later.hidden;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
    }
    fn inverse(&self, base: &Puzzle3dObject) -> Self {
        Self {
            label: self.label.as_ref().map(|_| base.label.clone()),
            object_kind: self.object_kind.as_ref().map(|_| base.object_kind.clone()),
            anchor: self.anchor.as_ref().map(|_| base.anchor),
            origin: self.origin.as_ref().map(|_| base.origin),
            orientation: self.orientation.as_ref().map(|_| base.orientation),
            scale: self.scale.as_ref().map(|_| base.scale.clone()),
            mesh_url: self.mesh_url.as_ref().map(|_| base.mesh_url.clone()),
            vortices: self.vortices.as_ref().map(|delta| delta.inverse(&base.vortices)),
            hidden: self.hidden.as_ref().map(|_| base.hidden),
            locked: self.locked.as_ref().map(|_| base.locked),
        }
    }
    fn between(base: &Puzzle3dObject, other: &Puzzle3dObject) -> Self {
        Self {
            label: (base.label != other.label).then(|| other.label.clone()),
            object_kind: (base.object_kind != other.object_kind).then(|| other.object_kind.clone()),
            anchor: (base.anchor != other.anchor).then(|| other.anchor),
            origin: (base.origin != other.origin).then(|| other.origin),
            orientation: (base.orientation != other.orientation).then(|| other.orientation),
            scale: (base.scale != other.scale).then(|| other.scale.clone()),
            mesh_url: (base.mesh_url != other.mesh_url).then(|| other.mesh_url.clone()),
            vortices: Some(Puzzle3dVorticesDelta::between(&base.vortices, &other.vortices)).filter(|delta| !delta.is_empty()),
            hidden: (base.hidden != other.hidden).then(|| other.hidden),
            locked: (base.locked != other.locked).then(|| other.locked),
        }
    }
    fn is_empty(&self) -> bool {
        self.label.is_none() && self.object_kind.is_none() && self.anchor.is_none() && self.origin.is_none() && self.orientation.is_none() && self.scale.is_none() && self.mesh_url.is_none() && self.vortices.as_ref().is_none_or(Puzzle3dVorticesDelta::is_empty) && self.hidden.is_none() && self.locked.is_none()
    }
}

impl ItemPatch<Puzzle3dAttraction> for Puzzle3dAttractionPatch {
    fn apply_to(&self, item: &mut Puzzle3dAttraction, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.attracting {
            item.attracting = value.clone();
        }
        if let Some(value) = &self.attracted {
            item.attracted = value.clone();
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
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.attracting.is_some() {
            self.attracting = later.attracting;
        }
        if later.attracted.is_some() {
            self.attracted = later.attracted;
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
    }
    fn inverse(&self, base: &Puzzle3dAttraction) -> Self {
        Self {
            attracting: self.attracting.as_ref().map(|_| base.attracting.clone()),
            attracted: self.attracted.as_ref().map(|_| base.attracted.clone()),
            gap: self.gap.as_ref().map(|_| base.gap),
            shift: self.shift.as_ref().map(|_| base.shift),
            rise: self.rise.as_ref().map(|_| base.rise),
            rotation: self.rotation.as_ref().map(|_| base.rotation),
            turn: self.turn.as_ref().map(|_| base.turn),
            tilt: self.tilt.as_ref().map(|_| base.tilt),
            x: self.x.as_ref().map(|_| base.x),
            y: self.y.as_ref().map(|_| base.y),
        }
    }
    fn between(base: &Puzzle3dAttraction, other: &Puzzle3dAttraction) -> Self {
        Self {
            attracting: (base.attracting != other.attracting).then(|| other.attracting.clone()),
            attracted: (base.attracted != other.attracted).then(|| other.attracted.clone()),
            gap: (base.gap != other.gap).then(|| other.gap),
            shift: (base.shift != other.shift).then(|| other.shift),
            rise: (base.rise != other.rise).then(|| other.rise),
            rotation: (base.rotation != other.rotation).then(|| other.rotation),
            turn: (base.turn != other.turn).then(|| other.turn),
            tilt: (base.tilt != other.tilt).then(|| other.tilt),
            x: (base.x != other.x).then(|| other.x),
            y: (base.y != other.y).then(|| other.y),
        }
    }
    fn is_empty(&self) -> bool {
        self.attracting.is_none() && self.attracted.is_none() && self.gap.is_none() && self.shift.is_none() && self.rise.is_none() && self.rotation.is_none() && self.turn.is_none() && self.tilt.is_none() && self.x.is_none() && self.y.is_none()
    }
}

impl ItemPatch<Puzzle3dTargetVolume> for Puzzle3dTargetVolumePatch {
    fn apply_to(&self, item: &mut Puzzle3dTargetVolume, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.origin {
            item.origin = *value;
        }
        if let Some(value) = &self.orientation {
            item.orientation = *value;
        }
        if let Some(value) = &self.scale {
            item.scale = value.clone();
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
        if later.origin.is_some() {
            self.origin = later.origin;
        }
        if later.orientation.is_some() {
            self.orientation = later.orientation;
        }
        if later.scale.is_some() {
            self.scale = later.scale;
        }
        if later.hidden.is_some() {
            self.hidden = later.hidden;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
    }
    fn inverse(&self, base: &Puzzle3dTargetVolume) -> Self {
        Self {
            origin: self.origin.as_ref().map(|_| base.origin),
            orientation: self.orientation.as_ref().map(|_| base.orientation),
            scale: self.scale.as_ref().map(|_| base.scale.clone()),
            hidden: self.hidden.as_ref().map(|_| base.hidden),
            locked: self.locked.as_ref().map(|_| base.locked),
        }
    }
    fn between(base: &Puzzle3dTargetVolume, other: &Puzzle3dTargetVolume) -> Self {
        Self {
            origin: (base.origin != other.origin).then(|| other.origin),
            orientation: (base.orientation != other.orientation).then(|| other.orientation),
            scale: (base.scale != other.scale).then(|| other.scale.clone()),
            hidden: (base.hidden != other.hidden).then(|| other.hidden),
            locked: (base.locked != other.locked).then(|| other.locked),
        }
    }
    fn is_empty(&self) -> bool {
        self.origin.is_none() && self.orientation.is_none() && self.scale.is_none() && self.hidden.is_none() && self.locked.is_none()
    }
}

impl ItemPatch<Puzzle3dReference> for Puzzle3dReferencePatch {
    fn apply_to(&self, item: &mut Puzzle3dReference, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.source {
            item.source = value.clone();
        }
        if let Some(value) = &self.origin {
            item.origin = *value;
        }
        if let Some(value) = &self.width_world {
            item.width_world = *value;
        }
        if let Some(value) = &self.locked {
            item.locked = *value;
        }
        if let Some(value) = &self.hidden {
            item.hidden = *value;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.source.is_some() {
            self.source = later.source;
        }
        if later.origin.is_some() {
            self.origin = later.origin;
        }
        if later.width_world.is_some() {
            self.width_world = later.width_world;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
        if later.hidden.is_some() {
            self.hidden = later.hidden;
        }
    }
    fn inverse(&self, base: &Puzzle3dReference) -> Self {
        Self {
            source: self.source.as_ref().map(|_| base.source.clone()),
            origin: self.origin.as_ref().map(|_| base.origin),
            width_world: self.width_world.as_ref().map(|_| base.width_world),
            locked: self.locked.as_ref().map(|_| base.locked),
            hidden: self.hidden.as_ref().map(|_| base.hidden),
        }
    }
    fn between(base: &Puzzle3dReference, other: &Puzzle3dReference) -> Self {
        Self {
            source: (base.source != other.source).then(|| other.source.clone()),
            origin: (base.origin != other.origin).then(|| other.origin),
            width_world: (base.width_world != other.width_world).then(|| other.width_world),
            locked: (base.locked != other.locked).then(|| other.locked),
            hidden: (base.hidden != other.hidden).then(|| other.hidden),
        }
    }
    fn is_empty(&self) -> bool {
        self.source.is_none() && self.origin.is_none() && self.width_world.is_none() && self.locked.is_none() && self.hidden.is_none()
    }
}

impl ItemPatch<Puzzle3dKindCompatibility> for Puzzle3dKindCompatibilityPatch {
    fn apply_to(&self, item: &mut Puzzle3dKindCompatibility, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
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
    fn inverse(&self, base: &Puzzle3dKindCompatibility) -> Self {
        Self {
            bidirectional: self.bidirectional.as_ref().map(|_| base.bidirectional),
            important: self.important.as_ref().map(|_| base.important),
            specificity: self.specificity.as_ref().map(|_| base.specificity),
        }
    }
    fn between(base: &Puzzle3dKindCompatibility, other: &Puzzle3dKindCompatibility) -> Self {
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

impl ItemPatch<Puzzle3dMeta> for Puzzle3dMetaPatch {
    fn apply_to(&self, item: &mut Puzzle3dMeta, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.kind_catalogs {
            item.kind_catalogs = value.clone();
        }
        if let Some(delta) = &self.kind_compatibility {
            item.kind_compatibility = delta.apply_to(&item.kind_compatibility, capability).map_err(|error| error.under(["kindCompatibility"]))?;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.kind_catalogs.is_some() {
            self.kind_catalogs = later.kind_catalogs;
        }
        match (&mut self.kind_compatibility, later.kind_compatibility) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
    }
    fn inverse(&self, base: &Puzzle3dMeta) -> Self {
        Self {
            kind_catalogs: self.kind_catalogs.as_ref().map(|_| base.kind_catalogs.clone()),
            kind_compatibility: self.kind_compatibility.as_ref().map(|delta| delta.inverse(&base.kind_compatibility)),
        }
    }
    fn between(base: &Puzzle3dMeta, other: &Puzzle3dMeta) -> Self {
        Self {
            kind_catalogs: (base.kind_catalogs != other.kind_catalogs).then(|| other.kind_catalogs.clone()),
            kind_compatibility: Some(Puzzle3dKindCompatibilityDelta::between(&base.kind_compatibility, &other.kind_compatibility)).filter(|delta| !delta.is_empty()),
        }
    }
    fn is_empty(&self) -> bool {
        self.kind_catalogs.is_none() && self.kind_compatibility.as_ref().is_none_or(Puzzle3dKindCompatibilityDelta::is_empty)
    }
}

impl MutationDiff<Puzzle3dSnapshot> for Puzzle3dDiff {
    fn apply(&self, base: &Puzzle3dSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle3dSnapshot> {
        let mut item = match &self.artifact {
            Some(artifact) => artifact.to_snapshot(),
            None => base.clone(),
        };
        if let Some(value) = &self.schema {
            item.schema = value.clone();
        }
        if let Some(value) = &self.domain {
            item.domain = value.clone();
        }
        if let Some(patch) = &self.meta {
            patch.apply_to(&mut item.meta, capability).map_err(|error| error.under(["meta"]))?;
        }
        if let Some(delta) = &self.objects {
            item.objects = delta.apply_to(&item.objects, capability).map_err(|error| error.under(["objects"]))?;
        }
        if let Some(delta) = &self.attractions {
            item.attractions = delta.apply_to(&item.attractions, capability).map_err(|error| error.under(["attractions"]))?;
        }
        if let Some(delta) = &self.target_volumes {
            item.target_volumes = delta.apply_to(&item.target_volumes, capability).map_err(|error| error.under(["targetVolumes"]))?;
        }
        if let Some(delta) = &self.references {
            item.references = delta.apply_to(&item.references, capability).map_err(|error| error.under(["references"]))?;
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
        if later.domain.is_some() {
            self.domain = later.domain;
        }
        match (&mut self.meta, later.meta) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.objects, later.objects) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.attractions, later.attractions) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.target_volumes, later.target_volumes) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.references, later.references) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
    }
}

impl DiffAlgebra<Puzzle3dSnapshot> for Puzzle3dDiff {
    fn inverse(&self, base: &Puzzle3dSnapshot) -> Self {
        if self.artifact.is_some() {
            return Self { artifact: Some(Box::new(Puzzle3dArtifact::from_snapshot(base.clone()))), ..Default::default() };
        }
        Self {
            artifact: None,
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            domain: self.domain.as_ref().map(|_| base.domain.clone()),
            meta: self.meta.as_ref().map(|patch| patch.inverse(&base.meta)),
            objects: self.objects.as_ref().map(|delta| delta.inverse(&base.objects)),
            attractions: self.attractions.as_ref().map(|delta| delta.inverse(&base.attractions)),
            target_volumes: self.target_volumes.as_ref().map(|delta| delta.inverse(&base.target_volumes)),
            references: self.references.as_ref().map(|delta| delta.inverse(&base.references)),
        }
    }
    fn between(base: &Puzzle3dSnapshot, other: &Puzzle3dSnapshot) -> Self {
        Self {
            artifact: None,
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            domain: (base.domain != other.domain).then(|| other.domain.clone()),
            meta: Some(<Puzzle3dMetaPatch as ItemPatch<_>>::between(&base.meta, &other.meta)).filter(|patch| !patch.is_empty()),
            objects: Some(Puzzle3dObjectsDelta::between(&base.objects, &other.objects)).filter(|delta| !delta.is_empty()),
            attractions: Some(Puzzle3dAttractionsDelta::between(&base.attractions, &other.attractions)).filter(|delta| !delta.is_empty()),
            target_volumes: Some(Puzzle3dTargetVolumesDelta::between(&base.target_volumes, &other.target_volumes)).filter(|delta| !delta.is_empty()),
            references: Some(Puzzle3dReferencesDelta::between(&base.references, &other.references)).filter(|delta| !delta.is_empty()),
        }
    }
    fn is_empty(&self) -> bool {
        self.artifact.is_none() && self.schema.is_none() && self.domain.is_none() && self.meta.as_ref().is_none_or(|patch| patch.is_empty()) && self.objects.as_ref().is_none_or(Puzzle3dObjectsDelta::is_empty) && self.attractions.as_ref().is_none_or(Puzzle3dAttractionsDelta::is_empty) && self.target_volumes.as_ref().is_none_or(Puzzle3dTargetVolumesDelta::is_empty) && self.references.as_ref().is_none_or(Puzzle3dReferencesDelta::is_empty)
    }
}
