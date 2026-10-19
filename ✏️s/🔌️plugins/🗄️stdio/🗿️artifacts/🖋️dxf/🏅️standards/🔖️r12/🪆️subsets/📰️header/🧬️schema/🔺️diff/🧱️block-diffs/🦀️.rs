//! 🧱️ Block diffs of the DXF diff: the per-block diff, its index element behaviour, the modified and added records and the block collection diff. See [`super`].

use super::*;

//#region 🔖️BlockDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfBlockDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub base_point: Option<[f64; 3]>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub entities: Option<DxfEntitiesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}

impl DxfIndexElem for DxfBlock {
    type Diff = DxfBlockDiff;
    fn diff_is_empty(d: &Self::Diff) -> bool {
        d == &DxfBlockDiff::default()
    }
    fn field_changes(a: &Self, b: &Self) -> Self::Diff {
        DxfBlockDiff {
            name: (a.name != b.name).then(|| b.name.clone()),
            base_point: (a.base_point != b.base_point).then_some(b.base_point),
            entities: (a.entities != b.entities).then(|| DxfEntitiesDiff { removed: (0..a.entities.len()).collect(), modified: Vec::new(), added: b.entities.iter().cloned().enumerate().map(|(index, entity)| DxfEntityAdded { index, entity }).collect() }),
            unknown_group_codes: (a.unknown_group_codes != b.unknown_group_codes).then(|| b.unknown_group_codes.clone()),
        }
    }
    fn diff_inverse(d: &Self::Diff, base: &Self) -> Self::Diff {
        DxfBlockDiff {
            name: d.name.as_ref().map(|_| base.name.clone()),
            base_point: d.base_point.map(|_| base.base_point),
            entities: d.entities.as_ref().and_then(|entities| entities.inverse(&base.entities)),
            unknown_group_codes: d.unknown_group_codes.as_ref().map(|_| base.unknown_group_codes.clone()),
        }
    }
    fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let Some(v) = &d.name {
            item.name = v.clone();
        }
        if let Some(v) = d.base_point {
            item.base_point = v;
        }
        if let Some(ed) = &d.entities {
            item.entities = ed.apply(&item.entities);
        }
        if let Some(v) = &d.unknown_group_codes {
            item.unknown_group_codes = v.clone();
        }
    }
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        if other.name.is_some() {
            base.name = other.name;
        }
        if other.base_point.is_some() {
            base.base_point = other.base_point;
        }
        base.entities = match (base.entities.take(), other.entities) {
            (None, None) => None,
            (Some(a), None) => Some(a),
            (None, Some(b)) => Some(b),
            (Some(a), Some(b)) => DxfEntitiesDiff::absorb(a, b),
        };
        if other.unknown_group_codes.is_some() {
            base.unknown_group_codes = other.unknown_group_codes;
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfBlockModified {
    pub index: usize,
    pub diff: DxfBlockDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfBlockAdded {
    pub index: usize,
    pub block: DxfBlock,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfBlocksDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<usize>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<DxfBlockModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<DxfBlockAdded>,
}

impl DxfBlocksDiff {
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    pub(super) fn apply(&self, base: &[DxfBlock]) -> Vec<DxfBlock> {
        let modified: Vec<(usize, DxfBlockDiff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, DxfBlock)> = self.added.iter().map(|a| (a.index, a.block.clone())).collect();
        generic_apply(base, &self.removed, &modified, &added)
    }
    pub(super) fn inverse(&self, base: &[DxfBlock]) -> Option<Self> {
        let modified: Vec<(usize, <DxfBlock as DxfIndexElem>::Diff)> = self.modified.iter().map(|m| (m.index, m.diff.clone())).collect();
        let added: Vec<(usize, DxfBlock)> = self.added.iter().map(|a| (a.index, a.block.clone())).collect();
        let (removed, modified, added) = generic_inverse(&self.removed, &modified, &added, base);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| DxfBlockModified { index, diff }).collect(), added: added.into_iter().map(|(index, block)| DxfBlockAdded { index, block }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    pub(super) fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(usize, DxfBlockDiff)> = d1.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d1a: Vec<(usize, DxfBlock)> = d1.added.into_iter().map(|a| (a.index, a.block)).collect();
        let d2m: Vec<(usize, DxfBlockDiff)> = d2.modified.into_iter().map(|m| (m.index, m.diff)).collect();
        let d2a: Vec<(usize, DxfBlock)> = d2.added.into_iter().map(|a| (a.index, a.block)).collect();
        let (removed, modified, added) = generic_absorb_pair::<DxfBlock>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(index, diff)| DxfBlockModified { index, diff }).collect(), added: added.into_iter().map(|(index, block)| DxfBlockAdded { index, block }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️BlockDiff
