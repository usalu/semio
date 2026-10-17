//! 📐️ Entity diff records of the DXF diff: the per-kind entity diffs, the entity diff enum and the modified, added and collection records of the entities. See [`super`].

use super::*;

//#region 🔖️EntityDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfLineDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub start: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub end: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfCircleDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub center: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfArcDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub center: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub radius: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub start_angle: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub end_angle: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}

/// 🔺️ `vertices` is a weak leaf value (a polyline's own vertex list) — whole-vec replaced,
/// never sub-diffed (recipe's weak-entity rule).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfPolylineDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub vertices: Option<Vec<DxfVertex>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub closed: Option<bool>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfTextDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfSolidDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub points: Option<[[f64; 3]; 4]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfInsertDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub block_name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub scale: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layer: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}

/// 🔺️ `group_codes` is a weak leaf value — whole-vec replaced (the entity's `kind` never
/// changes within an `Other` variant match; a kind change is handled by `DxfEntityDiff::Replace`).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfOtherDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub group_codes: Option<Vec<(i32, DxfValue)>>,
}

/// 🔺️ Per-entity diff: `Replace` when the entity KIND changes at this index (the plan's
/// json/xml "Replace on kind change" rule applied to this enum collection element); otherwise a
/// kind-specific sparse field diff.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub enum DxfEntityDiff {
    Replace { entity: DxfEntity },
    Line(DxfLineDiff),
    Circle(DxfCircleDiff),
    Arc(DxfArcDiff),
    Polyline(DxfPolylineDiff),
    Text(DxfTextDiff),
    Solid(DxfSolidDiff),
    Insert(DxfInsertDiff),
    Other(DxfOtherDiff),
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfEntityModified {
    pub index: usize,
    pub diff: DxfEntityDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfEntityAdded {
    pub index: usize,
    pub entity: DxfEntity,
}

/// 🔺️ Index-keyed removed/modified/added triple over an entity collection — reused for BOTH
/// `DxfSnapshot::entities` and each `DxfBlock::entities`.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfEntitiesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<DxfEntityModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<DxfEntityAdded>,
}

impl DxfEntitiesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn apply(&self, base: &[DxfEntity]) -> Vec<DxfEntity> {
        let modified: Vec<(usize, DxfEntityDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, DxfEntity)> = self.added.iter().map(|a| (a.index, a.entity.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn inverse(&self, base: &[DxfEntity]) -> Option<Self> {
        let modified: Vec<(usize, <DxfEntity as DxfIndexElem>::Diff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, DxfEntity)> = self.added.iter().map(|a| (a.index, a.entity.clone())).collect();
        let (removed, modified, added) = generic_inverse(&self.removed, &modified, &added, base);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| DxfEntityModified { index, diff }).collect(), added: added.into_iter().map(|(index, entity)| DxfEntityAdded { index, entity }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(usize, DxfEntityDiff)> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d1a: Vec<(usize, DxfEntity)> = d1.added.into_iter().map(|a| (a.index, a.entity)).collect();
        let d2m: Vec<(usize, DxfEntityDiff)> = d2.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d2a: Vec<(usize, DxfEntity)> = d2.added.into_iter().map(|a| (a.index, a.entity)).collect();
        let (removed, modified, added) = generic_absorb_pair::<DxfEntity>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| DxfEntityModified { index, diff }).collect(), added: added.into_iter().map(|(index, entity)| DxfEntityAdded { index, entity }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️EntityDiff
