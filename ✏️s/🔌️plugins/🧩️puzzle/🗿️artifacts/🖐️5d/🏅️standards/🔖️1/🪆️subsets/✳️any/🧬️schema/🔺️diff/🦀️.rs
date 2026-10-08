//! 🧬️ Puzzle5d diff schema — sparse typed delta over the artifact: per-field entity patches and id-keyed collection deltas.

use crate::standards::v1::subsets::any::schema::Puzzle5dArtifact;
use crate::{Puzzle5dCompatSpecificity, Puzzle5dFastener, Puzzle5dGrip, Puzzle5dKindCatalogsExtra, Puzzle5dKindCompatibility, Puzzle5dMeta, Puzzle5dPart, Puzzle5dPart2d, Puzzle5dPart3d, Puzzle5dPartAnchor, Puzzle5dGrip2d, Puzzle5dGrip3d, Puzzle5dScale, Puzzle5dTargetVolume};
use crate::Puzzle5dSnapshot;
use ::semio_framework_schema::ArtifactSchema;
use protocol::{DiffAlgebra, MutationDiff};
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
/// 🔺️ Sparse typed delta for the puzzle5d artifact: per-field entity patches and id-keyed collection deltas.
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
#[artifact_schema(id = "s.puzzle.puzzle5d")]
pub struct Puzzle5dDiff {
    #[state(artifact)]
    pub artifact: Option<Box<Puzzle5dArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub domain: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    #[state(artifact)]
    pub label: Option<Option<String>>,
    #[state(artifact)]
    pub meta: Option<Puzzle5dMetaPatch>,
    #[child(kind = "s.stdio.semio")]
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    #[state(artifact)]
    pub kind_catalogs: Option<Option<store::ArtifactChild<SemioKitSnapshot>>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    #[state(artifact)]
    pub kind_catalogs_extra: Option<Option<Puzzle5dKindCatalogsExtra>>,
    #[state(artifact)]
    pub kind_compatibility: Option<Puzzle5dKindCompatibilityDelta>,
    #[state(artifact)]
    pub parts: Option<Puzzle5dPartsDelta>,
    #[state(artifact)]
    pub fasteners: Option<Puzzle5dFastenersDelta>,
    #[state(artifact)]
    pub target_volumes: Option<Puzzle5dTargetVolumesDelta>,
}
//#endregion 🔖️Diff

//#region 🔖️Patches
/// 🔑️ The identity of one kind-compatibility row: the pair of kinds it links.
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dKindCompatibilityKey {
    pub source: String,
    pub target: String,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dPart2d` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dPart2dPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub x: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub y: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub shape: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub radius: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub width: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub height: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub text: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub icon_kind: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub hidden: Option<Option<bool>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub locked: Option<Option<bool>>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dPart3d` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dPart3dPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub mesh_url: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub orientation: Option<Option<[f64; 4]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scale: Option<Option<Puzzle5dScale>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub label: Option<Option<String>>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dGrip2d` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dGrip2dPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub angle: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub grip_kind: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub radius: Option<Option<f64>>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dGrip3d` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dGrip3dPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub direction: Option<Option<[f64; 3]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub radius: Option<Option<f64>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub label: Option<Option<String>>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dGrip` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dGripPatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub grip_kind: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none", rename = "2d")]
    pub grip_2d: Option<Puzzle5dGrip2dPatch>,
    #[value(default, skip_serializing_if = "Option::is_none", rename = "3d")]
    pub grip_3d: Option<Puzzle5dGrip3dPatch>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dPart` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dPartPatch {
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub part_kind: Option<Option<String>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub anchor: Option<Puzzle5dPartAnchor>,
    #[value(default, skip_serializing_if = "Option::is_none", rename = "2d")]
    pub part_2d: Option<Puzzle5dPart2dPatch>,
    #[value(default, skip_serializing_if = "Option::is_none", rename = "3d")]
    pub part_3d: Option<Puzzle5dPart3dPatch>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub grips: Option<Puzzle5dGripsDelta>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dFastener` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dFastenerPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub fastener_kind: Option<Option<String>>,
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

/// 🩹 Sparse per-field patch over one `Puzzle5dTargetVolume` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dTargetVolumePatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub origin: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub orientation: Option<Option<[f64; 4]>>,
    #[value(default, skip_serializing_if = "Option::is_none", deserialize_with = "deserialize_double_option")]
    pub scale: Option<Option<Puzzle5dScale>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub locked: Option<bool>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dKindCompatibility` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dKindCompatibilityPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub bidirectional: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub important: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub specificity: Option<Puzzle5dCompatSpecificity>,
}

/// 🩹 Sparse per-field patch over one `Puzzle5dMeta` — only the named fields change.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dMetaPatch {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

//#endregion 🔖️Patches

//#region 🔖️Deltas
/// 🧩 Identified-collection delta for a part's `grips`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dGripsDelta {
    pub added: Vec<Puzzle5dGrip>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle5dGripPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle5dGrip` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dGripPatchEntry {
    pub id: String,
    pub patch: Puzzle5dGripPatch,
}

/// 🧩 Identified-collection delta for `parts`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dPartsDelta {
    pub added: Vec<Puzzle5dPart>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle5dPartPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle5dPart` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dPartPatchEntry {
    pub id: String,
    pub patch: Puzzle5dPartPatch,
}

/// 🧩 Identified-collection delta for `fasteners`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dFastenersDelta {
    pub added: Vec<Puzzle5dFastener>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle5dFastenerPatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle5dFastener` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dFastenerPatchEntry {
    pub id: String,
    pub patch: Puzzle5dFastenerPatch,
}

/// 🧩 Identified-collection delta for `targetVolumes`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dTargetVolumesDelta {
    pub added: Vec<Puzzle5dTargetVolume>,
    pub removed: Vec<String>,
    pub patched: Vec<Puzzle5dTargetVolumePatchEntry>,
    pub reordered: Option<Vec<String>>,
}

/// 🩹 One patched `Puzzle5dTargetVolume` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dTargetVolumePatchEntry {
    pub id: String,
    pub patch: Puzzle5dTargetVolumePatch,
}

/// 🧩 Keyed-row delta for `kindCompatibility` (a row is addressed by its source and target kinds).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase", default)]
pub struct Puzzle5dKindCompatibilityDelta {
    pub added: Vec<Puzzle5dKindCompatibility>,
    pub removed: Vec<Puzzle5dKindCompatibilityKey>,
    pub patched: Vec<Puzzle5dKindCompatibilityPatchEntry>,
    pub reordered: Option<Vec<Puzzle5dKindCompatibilityKey>>,
}

/// 🩹 One patched `Puzzle5dKindCompatibility` entry.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all = "camelCase")]
pub struct Puzzle5dKindCompatibilityPatchEntry {
    pub id: Puzzle5dKindCompatibilityKey,
    pub patch: Puzzle5dKindCompatibilityPatch,
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
fn grip_key(item: &Puzzle5dGrip) -> String {
    item.id.clone()
}

fn part_key(item: &Puzzle5dPart) -> String {
    item.id.clone()
}

fn fastener_key(item: &Puzzle5dFastener) -> String {
    item.id.clone()
}

fn target_volume_key(item: &Puzzle5dTargetVolume) -> String {
    item.id.clone()
}

fn kind_compatibility_key(item: &Puzzle5dKindCompatibility) -> Puzzle5dKindCompatibilityKey {
    Puzzle5dKindCompatibilityKey { source: item.source.clone(), target: item.target.clone() }
}

fn text_label(id: &String) -> String {
    id.clone()
}

fn kind_compatibility_label(id: &Puzzle5dKindCompatibilityKey) -> String {
    format!("{} -> {}", id.source, id.target)
}

//#endregion 🔖️Keys

keyed_delta!(Puzzle5dGripsDelta, Puzzle5dGripPatchEntry, Puzzle5dGripPatch, Puzzle5dGrip, String, Vec<Puzzle5dGrip>, grip_key, text_label);
keyed_delta!(Puzzle5dPartsDelta, Puzzle5dPartPatchEntry, Puzzle5dPartPatch, Puzzle5dPart, String, Vec<Puzzle5dPart>, part_key, text_label);
keyed_delta!(Puzzle5dFastenersDelta, Puzzle5dFastenerPatchEntry, Puzzle5dFastenerPatch, Puzzle5dFastener, String, Vec<Puzzle5dFastener>, fastener_key, text_label);
keyed_delta!(Puzzle5dTargetVolumesDelta, Puzzle5dTargetVolumePatchEntry, Puzzle5dTargetVolumePatch, Puzzle5dTargetVolume, String, Vec<Puzzle5dTargetVolume>, target_volume_key, text_label);
keyed_delta!(Puzzle5dKindCompatibilityDelta, Puzzle5dKindCompatibilityPatchEntry, Puzzle5dKindCompatibilityPatch, Puzzle5dKindCompatibility, Puzzle5dKindCompatibilityKey, Vec<Puzzle5dKindCompatibility>, kind_compatibility_key, kind_compatibility_label);

impl ItemPatch<Puzzle5dPart2d> for Puzzle5dPart2dPatch {
    fn apply_to(&self, item: &mut Puzzle5dPart2d, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.x {
            item.x = *value;
        }
        if let Some(value) = &self.y {
            item.y = *value;
        }
        if let Some(value) = &self.shape {
            item.shape = value.clone();
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
        if later.shape.is_some() {
            self.shape = later.shape;
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
        if later.hidden.is_some() {
            self.hidden = later.hidden;
        }
        if later.locked.is_some() {
            self.locked = later.locked;
        }
    }
    fn inverse(&self, base: &Puzzle5dPart2d) -> Self {
        Self {
            x: self.x.as_ref().map(|_| base.x),
            y: self.y.as_ref().map(|_| base.y),
            shape: self.shape.as_ref().map(|_| base.shape.clone()),
            radius: self.radius.as_ref().map(|_| base.radius),
            width: self.width.as_ref().map(|_| base.width),
            height: self.height.as_ref().map(|_| base.height),
            text: self.text.as_ref().map(|_| base.text.clone()),
            icon_kind: self.icon_kind.as_ref().map(|_| base.icon_kind.clone()),
            hidden: self.hidden.as_ref().map(|_| base.hidden),
            locked: self.locked.as_ref().map(|_| base.locked),
        }
    }
    fn between(base: &Puzzle5dPart2d, other: &Puzzle5dPart2d) -> Self {
        Self {
            x: (base.x != other.x).then(|| other.x),
            y: (base.y != other.y).then(|| other.y),
            shape: (base.shape != other.shape).then(|| other.shape.clone()),
            radius: (base.radius != other.radius).then(|| other.radius),
            width: (base.width != other.width).then(|| other.width),
            height: (base.height != other.height).then(|| other.height),
            text: (base.text != other.text).then(|| other.text.clone()),
            icon_kind: (base.icon_kind != other.icon_kind).then(|| other.icon_kind.clone()),
            hidden: (base.hidden != other.hidden).then(|| other.hidden),
            locked: (base.locked != other.locked).then(|| other.locked),
        }
    }
    fn is_empty(&self) -> bool {
        self.x.is_none() && self.y.is_none() && self.shape.is_none() && self.radius.is_none() && self.width.is_none() && self.height.is_none() && self.text.is_none() && self.icon_kind.is_none() && self.hidden.is_none() && self.locked.is_none()
    }
}

impl ItemPatch<Puzzle5dPart3d> for Puzzle5dPart3dPatch {
    fn apply_to(&self, item: &mut Puzzle5dPart3d, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.origin {
            item.origin = *value;
        }
        if let Some(value) = &self.mesh_url {
            item.mesh_url = value.clone();
        }
        if let Some(value) = &self.orientation {
            item.orientation = *value;
        }
        if let Some(value) = &self.scale {
            item.scale = *value;
        }
        if let Some(value) = &self.label {
            item.label = value.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.origin.is_some() {
            self.origin = later.origin;
        }
        if later.mesh_url.is_some() {
            self.mesh_url = later.mesh_url;
        }
        if later.orientation.is_some() {
            self.orientation = later.orientation;
        }
        if later.scale.is_some() {
            self.scale = later.scale;
        }
        if later.label.is_some() {
            self.label = later.label;
        }
    }
    fn inverse(&self, base: &Puzzle5dPart3d) -> Self {
        Self {
            origin: self.origin.as_ref().map(|_| base.origin),
            mesh_url: self.mesh_url.as_ref().map(|_| base.mesh_url.clone()),
            orientation: self.orientation.as_ref().map(|_| base.orientation),
            scale: self.scale.as_ref().map(|_| base.scale),
            label: self.label.as_ref().map(|_| base.label.clone()),
        }
    }
    fn between(base: &Puzzle5dPart3d, other: &Puzzle5dPart3d) -> Self {
        Self {
            origin: (base.origin != other.origin).then(|| other.origin),
            mesh_url: (base.mesh_url != other.mesh_url).then(|| other.mesh_url.clone()),
            orientation: (base.orientation != other.orientation).then(|| other.orientation),
            scale: (base.scale != other.scale).then(|| other.scale),
            label: (base.label != other.label).then(|| other.label.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.origin.is_none() && self.mesh_url.is_none() && self.orientation.is_none() && self.scale.is_none() && self.label.is_none()
    }
}

impl ItemPatch<Puzzle5dGrip2d> for Puzzle5dGrip2dPatch {
    fn apply_to(&self, item: &mut Puzzle5dGrip2d, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.angle {
            item.angle = *value;
        }
        if let Some(value) = &self.grip_kind {
            item.grip_kind = value.clone();
        }
        if let Some(value) = &self.radius {
            item.radius = *value;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.angle.is_some() {
            self.angle = later.angle;
        }
        if later.grip_kind.is_some() {
            self.grip_kind = later.grip_kind;
        }
        if later.radius.is_some() {
            self.radius = later.radius;
        }
    }
    fn inverse(&self, base: &Puzzle5dGrip2d) -> Self {
        Self {
            angle: self.angle.as_ref().map(|_| base.angle),
            grip_kind: self.grip_kind.as_ref().map(|_| base.grip_kind.clone()),
            radius: self.radius.as_ref().map(|_| base.radius),
        }
    }
    fn between(base: &Puzzle5dGrip2d, other: &Puzzle5dGrip2d) -> Self {
        Self {
            angle: (base.angle != other.angle).then(|| other.angle),
            grip_kind: (base.grip_kind != other.grip_kind).then(|| other.grip_kind.clone()),
            radius: (base.radius != other.radius).then(|| other.radius),
        }
    }
    fn is_empty(&self) -> bool {
        self.angle.is_none() && self.grip_kind.is_none() && self.radius.is_none()
    }
}

impl ItemPatch<Puzzle5dGrip3d> for Puzzle5dGrip3dPatch {
    fn apply_to(&self, item: &mut Puzzle5dGrip3d, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.position {
            item.position = *value;
        }
        if let Some(value) = &self.direction {
            item.direction = *value;
        }
        if let Some(value) = &self.radius {
            item.radius = *value;
        }
        if let Some(value) = &self.label {
            item.label = value.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.position.is_some() {
            self.position = later.position;
        }
        if later.direction.is_some() {
            self.direction = later.direction;
        }
        if later.radius.is_some() {
            self.radius = later.radius;
        }
        if later.label.is_some() {
            self.label = later.label;
        }
    }
    fn inverse(&self, base: &Puzzle5dGrip3d) -> Self {
        Self {
            position: self.position.as_ref().map(|_| base.position),
            direction: self.direction.as_ref().map(|_| base.direction),
            radius: self.radius.as_ref().map(|_| base.radius),
            label: self.label.as_ref().map(|_| base.label.clone()),
        }
    }
    fn between(base: &Puzzle5dGrip3d, other: &Puzzle5dGrip3d) -> Self {
        Self {
            position: (base.position != other.position).then(|| other.position),
            direction: (base.direction != other.direction).then(|| other.direction),
            radius: (base.radius != other.radius).then(|| other.radius),
            label: (base.label != other.label).then(|| other.label.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.position.is_none() && self.direction.is_none() && self.radius.is_none() && self.label.is_none()
    }
}

impl ItemPatch<Puzzle5dGrip> for Puzzle5dGripPatch {
    fn apply_to(&self, item: &mut Puzzle5dGrip, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.grip_kind {
            item.grip_kind = value.clone();
        }
        if let Some(patch) = &self.grip_2d {
            patch.apply_to(&mut item.grip_2d, capability).map_err(|error| error.under(["2d"]))?;
        }
        if let Some(patch) = &self.grip_3d {
            patch.apply_to(&mut item.grip_3d, capability).map_err(|error| error.under(["3d"]))?;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.grip_kind.is_some() {
            self.grip_kind = later.grip_kind;
        }
        match (&mut self.grip_2d, later.grip_2d) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.grip_3d, later.grip_3d) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
    }
    fn inverse(&self, base: &Puzzle5dGrip) -> Self {
        Self {
            grip_kind: self.grip_kind.as_ref().map(|_| base.grip_kind.clone()),
            grip_2d: self.grip_2d.as_ref().map(|patch| patch.inverse(&base.grip_2d)),
            grip_3d: self.grip_3d.as_ref().map(|patch| patch.inverse(&base.grip_3d)),
        }
    }
    fn between(base: &Puzzle5dGrip, other: &Puzzle5dGrip) -> Self {
        Self {
            grip_kind: (base.grip_kind != other.grip_kind).then(|| other.grip_kind.clone()),
            grip_2d: Some(<Puzzle5dGrip2dPatch as ItemPatch<_>>::between(&base.grip_2d, &other.grip_2d)).filter(|patch| !patch.is_empty()),
            grip_3d: Some(<Puzzle5dGrip3dPatch as ItemPatch<_>>::between(&base.grip_3d, &other.grip_3d)).filter(|patch| !patch.is_empty()),
        }
    }
    fn is_empty(&self) -> bool {
        self.grip_kind.is_none() && self.grip_2d.as_ref().is_none_or(|patch| patch.is_empty()) && self.grip_3d.as_ref().is_none_or(|patch| patch.is_empty())
    }
}

impl ItemPatch<Puzzle5dPart> for Puzzle5dPartPatch {
    fn apply_to(&self, item: &mut Puzzle5dPart, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.part_kind {
            item.part_kind = value.clone();
        }
        if let Some(value) = &self.anchor {
            item.anchor = *value;
        }
        if let Some(patch) = &self.part_2d {
            patch.apply_to(&mut item.part_2d, capability).map_err(|error| error.under(["2d"]))?;
        }
        if let Some(patch) = &self.part_3d {
            patch.apply_to(&mut item.part_3d, capability).map_err(|error| error.under(["3d"]))?;
        }
        if let Some(delta) = &self.grips {
            item.grips = delta.apply_to(&item.grips, capability).map_err(|error| error.under(["grips"]))?;
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.part_kind.is_some() {
            self.part_kind = later.part_kind;
        }
        if later.anchor.is_some() {
            self.anchor = later.anchor;
        }
        match (&mut self.part_2d, later.part_2d) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.part_3d, later.part_3d) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.grips, later.grips) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
    }
    fn inverse(&self, base: &Puzzle5dPart) -> Self {
        Self {
            part_kind: self.part_kind.as_ref().map(|_| base.part_kind.clone()),
            anchor: self.anchor.as_ref().map(|_| base.anchor),
            part_2d: self.part_2d.as_ref().map(|patch| patch.inverse(&base.part_2d)),
            part_3d: self.part_3d.as_ref().map(|patch| patch.inverse(&base.part_3d)),
            grips: self.grips.as_ref().map(|delta| delta.inverse(&base.grips)),
        }
    }
    fn between(base: &Puzzle5dPart, other: &Puzzle5dPart) -> Self {
        Self {
            part_kind: (base.part_kind != other.part_kind).then(|| other.part_kind.clone()),
            anchor: (base.anchor != other.anchor).then(|| other.anchor),
            part_2d: Some(<Puzzle5dPart2dPatch as ItemPatch<_>>::between(&base.part_2d, &other.part_2d)).filter(|patch| !patch.is_empty()),
            part_3d: Some(<Puzzle5dPart3dPatch as ItemPatch<_>>::between(&base.part_3d, &other.part_3d)).filter(|patch| !patch.is_empty()),
            grips: Some(Puzzle5dGripsDelta::between(&base.grips, &other.grips)).filter(|delta| !delta.is_empty()),
        }
    }
    fn is_empty(&self) -> bool {
        self.part_kind.is_none() && self.anchor.is_none() && self.part_2d.as_ref().is_none_or(|patch| patch.is_empty()) && self.part_3d.as_ref().is_none_or(|patch| patch.is_empty()) && self.grips.as_ref().is_none_or(Puzzle5dGripsDelta::is_empty)
    }
}

impl ItemPatch<Puzzle5dFastener> for Puzzle5dFastenerPatch {
    fn apply_to(&self, item: &mut Puzzle5dFastener, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.source {
            item.source = value.clone();
        }
        if let Some(value) = &self.target {
            item.target = value.clone();
        }
        if let Some(value) = &self.fastener_kind {
            item.fastener_kind = value.clone();
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
        if later.source.is_some() {
            self.source = later.source;
        }
        if later.target.is_some() {
            self.target = later.target;
        }
        if later.fastener_kind.is_some() {
            self.fastener_kind = later.fastener_kind;
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
    fn inverse(&self, base: &Puzzle5dFastener) -> Self {
        Self {
            source: self.source.as_ref().map(|_| base.source.clone()),
            target: self.target.as_ref().map(|_| base.target.clone()),
            fastener_kind: self.fastener_kind.as_ref().map(|_| base.fastener_kind.clone()),
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
    fn between(base: &Puzzle5dFastener, other: &Puzzle5dFastener) -> Self {
        Self {
            source: (base.source != other.source).then(|| other.source.clone()),
            target: (base.target != other.target).then(|| other.target.clone()),
            fastener_kind: (base.fastener_kind != other.fastener_kind).then(|| other.fastener_kind.clone()),
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
        self.source.is_none() && self.target.is_none() && self.fastener_kind.is_none() && self.gap.is_none() && self.shift.is_none() && self.rise.is_none() && self.rotation.is_none() && self.turn.is_none() && self.tilt.is_none() && self.x.is_none() && self.y.is_none()
    }
}

impl ItemPatch<Puzzle5dTargetVolume> for Puzzle5dTargetVolumePatch {
    fn apply_to(&self, item: &mut Puzzle5dTargetVolume, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.origin {
            item.origin = *value;
        }
        if let Some(value) = &self.orientation {
            item.orientation = *value;
        }
        if let Some(value) = &self.scale {
            item.scale = *value;
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
    fn inverse(&self, base: &Puzzle5dTargetVolume) -> Self {
        Self {
            origin: self.origin.as_ref().map(|_| base.origin),
            orientation: self.orientation.as_ref().map(|_| base.orientation),
            scale: self.scale.as_ref().map(|_| base.scale),
            hidden: self.hidden.as_ref().map(|_| base.hidden),
            locked: self.locked.as_ref().map(|_| base.locked),
        }
    }
    fn between(base: &Puzzle5dTargetVolume, other: &Puzzle5dTargetVolume) -> Self {
        Self {
            origin: (base.origin != other.origin).then(|| other.origin),
            orientation: (base.orientation != other.orientation).then(|| other.orientation),
            scale: (base.scale != other.scale).then(|| other.scale),
            hidden: (base.hidden != other.hidden).then(|| other.hidden),
            locked: (base.locked != other.locked).then(|| other.locked),
        }
    }
    fn is_empty(&self) -> bool {
        self.origin.is_none() && self.orientation.is_none() && self.scale.is_none() && self.hidden.is_none() && self.locked.is_none()
    }
}

impl ItemPatch<Puzzle5dKindCompatibility> for Puzzle5dKindCompatibilityPatch {
    fn apply_to(&self, item: &mut Puzzle5dKindCompatibility, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
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
    fn inverse(&self, base: &Puzzle5dKindCompatibility) -> Self {
        Self {
            bidirectional: self.bidirectional.as_ref().map(|_| base.bidirectional),
            important: self.important.as_ref().map(|_| base.important),
            specificity: self.specificity.as_ref().map(|_| base.specificity),
        }
    }
    fn between(base: &Puzzle5dKindCompatibility, other: &Puzzle5dKindCompatibility) -> Self {
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

impl ItemPatch<Puzzle5dMeta> for Puzzle5dMetaPatch {
    fn apply_to(&self, item: &mut Puzzle5dMeta, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
        let _ = capability;
        if let Some(value) = &self.description {
            item.description = value.clone();
        }
        Ok(())
    }
    fn absorb(&mut self, later: Self) {
        if later.description.is_some() {
            self.description = later.description;
        }
    }
    fn inverse(&self, base: &Puzzle5dMeta) -> Self {
        Self {
            description: self.description.as_ref().map(|_| base.description.clone()),
        }
    }
    fn between(base: &Puzzle5dMeta, other: &Puzzle5dMeta) -> Self {
        Self {
            description: (base.description != other.description).then(|| other.description.clone()),
        }
    }
    fn is_empty(&self) -> bool {
        self.description.is_none()
    }
}

impl MutationDiff<Puzzle5dSnapshot> for Puzzle5dDiff {
    fn apply(&self, base: &Puzzle5dSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<Puzzle5dSnapshot> {
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
        if let Some(value) = &self.label {
            item.label = value.clone();
        }
        if let Some(patch) = &self.meta {
            patch.apply_to(&mut item.meta, capability).map_err(|error| error.under(["meta"]))?;
        }
        if let Some(value) = &self.kind_catalogs {
            item.kind_catalogs = value.clone();
        }
        if let Some(value) = &self.kind_catalogs_extra {
            item.kind_catalogs_extra = value.clone();
        }
        if let Some(delta) = &self.kind_compatibility {
            item.kind_compatibility = delta.apply_to(&item.kind_compatibility, capability).map_err(|error| error.under(["kindCompatibility"]))?;
        }
        if let Some(delta) = &self.parts {
            item.parts = delta.apply_to(&item.parts, capability).map_err(|error| error.under(["parts"]))?;
        }
        if let Some(delta) = &self.fasteners {
            item.fasteners = delta.apply_to(&item.fasteners, capability).map_err(|error| error.under(["fasteners"]))?;
        }
        if let Some(delta) = &self.target_volumes {
            item.target_volumes = delta.apply_to(&item.target_volumes, capability).map_err(|error| error.under(["targetVolumes"]))?;
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
        if later.label.is_some() {
            self.label = later.label;
        }
        match (&mut self.meta, later.meta) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        if later.kind_catalogs.is_some() {
            self.kind_catalogs = later.kind_catalogs;
        }
        if later.kind_catalogs_extra.is_some() {
            self.kind_catalogs_extra = later.kind_catalogs_extra;
        }
        match (&mut self.kind_compatibility, later.kind_compatibility) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.parts, later.parts) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.fasteners, later.fasteners) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
        match (&mut self.target_volumes, later.target_volumes) {
            (Some(earlier), Some(next)) => earlier.absorb(next),
            (slot @ None, next) => *slot = next,
            (Some(_), None) => {}
        }
    }
}

impl DiffAlgebra<Puzzle5dSnapshot> for Puzzle5dDiff {
    fn inverse(&self, base: &Puzzle5dSnapshot) -> Self {
        if self.artifact.is_some() {
            return Self { artifact: Some(Box::new(Puzzle5dArtifact::from_snapshot(base.clone()))), ..Default::default() };
        }
        Self {
            artifact: None,
            schema: self.schema.as_ref().map(|_| base.schema.clone()),
            domain: self.domain.as_ref().map(|_| base.domain.clone()),
            label: self.label.as_ref().map(|_| base.label.clone()),
            meta: self.meta.as_ref().map(|patch| patch.inverse(&base.meta)),
            kind_catalogs: self.kind_catalogs.as_ref().map(|_| base.kind_catalogs.clone()),
            kind_catalogs_extra: self.kind_catalogs_extra.as_ref().map(|_| base.kind_catalogs_extra.clone()),
            kind_compatibility: self.kind_compatibility.as_ref().map(|delta| delta.inverse(&base.kind_compatibility)),
            parts: self.parts.as_ref().map(|delta| delta.inverse(&base.parts)),
            fasteners: self.fasteners.as_ref().map(|delta| delta.inverse(&base.fasteners)),
            target_volumes: self.target_volumes.as_ref().map(|delta| delta.inverse(&base.target_volumes)),
        }
    }
    fn between(base: &Puzzle5dSnapshot, other: &Puzzle5dSnapshot) -> Self {
        Self {
            artifact: None,
            schema: (base.schema != other.schema).then(|| other.schema.clone()),
            domain: (base.domain != other.domain).then(|| other.domain.clone()),
            label: (base.label != other.label).then(|| other.label.clone()),
            meta: Some(<Puzzle5dMetaPatch as ItemPatch<_>>::between(&base.meta, &other.meta)).filter(|patch| !patch.is_empty()),
            kind_catalogs: (base.kind_catalogs != other.kind_catalogs).then(|| other.kind_catalogs.clone()),
            kind_catalogs_extra: (base.kind_catalogs_extra != other.kind_catalogs_extra).then(|| other.kind_catalogs_extra.clone()),
            kind_compatibility: Some(Puzzle5dKindCompatibilityDelta::between(&base.kind_compatibility, &other.kind_compatibility)).filter(|delta| !delta.is_empty()),
            parts: Some(Puzzle5dPartsDelta::between(&base.parts, &other.parts)).filter(|delta| !delta.is_empty()),
            fasteners: Some(Puzzle5dFastenersDelta::between(&base.fasteners, &other.fasteners)).filter(|delta| !delta.is_empty()),
            target_volumes: Some(Puzzle5dTargetVolumesDelta::between(&base.target_volumes, &other.target_volumes)).filter(|delta| !delta.is_empty()),
        }
    }
    fn is_empty(&self) -> bool {
        self.artifact.is_none() && self.schema.is_none() && self.domain.is_none() && self.label.is_none() && self.meta.as_ref().is_none_or(|patch| patch.is_empty()) && self.kind_catalogs.is_none() && self.kind_catalogs_extra.is_none() && self.kind_compatibility.as_ref().is_none_or(Puzzle5dKindCompatibilityDelta::is_empty) && self.parts.as_ref().is_none_or(Puzzle5dPartsDelta::is_empty) && self.fasteners.as_ref().is_none_or(Puzzle5dFastenersDelta::is_empty) && self.target_volumes.as_ref().is_none_or(Puzzle5dTargetVolumesDelta::is_empty)
    }
}
