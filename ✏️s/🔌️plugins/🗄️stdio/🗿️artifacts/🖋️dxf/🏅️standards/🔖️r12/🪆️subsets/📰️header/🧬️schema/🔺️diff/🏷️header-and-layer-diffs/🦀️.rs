//! 🏷️ Header variable and layer diffs of the DXF diff: the per-element diffs, their modified and added records and the name-keyed collection diffs. See [`super`].

use super::*;

//#region 🔖️HeaderVarDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfHeaderVarDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub group_code: Option<i32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<DxfValue>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub extra_group_codes: Option<Vec<(i32, DxfValue)>>,
}

impl DxfNamedElem for DxfHeaderVar {
    type Diff = DxfHeaderVarDiff;
    fn key(&self) -> &str {
        &self.name
    }
    fn field_changes(a: &Self, b: &Self) -> Self::Diff {
        DxfHeaderVarDiff {
            group_code: (a.group_code != b.group_code).then_some(b.group_code),
            value: (a.value != b.value).then(|| b.value.clone()),
            extra_group_codes: (a.extra_group_codes != b.extra_group_codes).then(|| b.extra_group_codes.clone()),
        }
    }
    fn diff_inverse(d: &Self::Diff, base: &Self) -> Self::Diff {
        DxfHeaderVarDiff { group_code: d.group_code.map(|_| base.group_code), value: d.value.as_ref().map(|_| base.value.clone()), extra_group_codes: d.extra_group_codes.as_ref().map(|_| base.extra_group_codes.clone()) }
    }
    fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let Some(v) = d.group_code {
            item.group_code = v;
        }
        if let Some(v) = &d.value {
            item.value = v.clone();
        }
        if let Some(v) = &d.extra_group_codes {
            item.extra_group_codes = v.clone();
        }
    }
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        if other.group_code.is_some() {
            base.group_code = other.group_code;
        }
        if other.value.is_some() {
            base.value = other.value;
        }
        if other.extra_group_codes.is_some() {
            base.extra_group_codes = other.extra_group_codes;
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfHeaderVarModified {
    pub name: String,
    pub diff: DxfHeaderVarDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfHeaderVarAdded {
    pub index: usize,
    pub header_var: DxfHeaderVar,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfHeaderVarsDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<DxfHeaderVarModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<DxfHeaderVarAdded>,
}

impl DxfHeaderVarsDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn apply(&self, base: &[DxfHeaderVar]) -> Vec<DxfHeaderVar> {
        let modified: Vec<(String, DxfHeaderVarDiff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfHeaderVar)> = self.added.iter().map(|a| (a.index, a.header_var.clone())).collect();
        named_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn inverse(&self, base: &[DxfHeaderVar]) -> Option<Self> {
        let modified: Vec<(String, <DxfHeaderVar as DxfNamedElem>::Diff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfHeaderVar)> = self.added.iter().map(|a| (a.index, a.header_var.clone())).collect();
        let (removed, modified, added) = named_inverse(&self.removed, &modified, &added, base);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfHeaderVarModified { name, diff }).collect(), added: added.into_iter().map(|(index, header_var)| DxfHeaderVarAdded { index, header_var }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(String, DxfHeaderVarDiff)> = d1.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d1a: Vec<(usize, DxfHeaderVar)> = d1.added.into_iter().map(|a| (a.index, a.header_var)).collect();
        let d2m: Vec<(String, DxfHeaderVarDiff)> = d2.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d2a: Vec<(usize, DxfHeaderVar)> = d2.added.into_iter().map(|a| (a.index, a.header_var)).collect();
        let (removed, modified, added) = named_absorb_pair::<DxfHeaderVar>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfHeaderVarModified { name, diff }).collect(), added: added.into_iter().map(|(index, header_var)| DxfHeaderVarAdded { index, header_var }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️HeaderVarDiff

//#region 🔖️LayerDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfLayerDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub color: Option<i32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub linetype: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}

impl DxfNamedElem for DxfLayer {
    type Diff = DxfLayerDiff;
    fn key(&self) -> &str {
        &self.name
    }
    fn field_changes(a: &Self, b: &Self) -> Self::Diff {
        DxfLayerDiff {
            color: (a.color != b.color).then_some(b.color),
            linetype: (a.linetype != b.linetype).then(|| b.linetype.clone()),
            flags: (a.flags != b.flags).then_some(b.flags),
            unknown_group_codes: (a.unknown_group_codes != b.unknown_group_codes).then(|| b.unknown_group_codes.clone()),
        }
    }
    fn diff_inverse(d: &Self::Diff, base: &Self) -> Self::Diff {
        DxfLayerDiff { color: d.color.map(|_| base.color), linetype: d.linetype.as_ref().map(|_| base.linetype.clone()), flags: d.flags.map(|_| base.flags), unknown_group_codes: d.unknown_group_codes.as_ref().map(|_| base.unknown_group_codes.clone()) }
    }
    fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let Some(v) = d.color {
            item.color = v;
        }
        if let Some(v) = &d.linetype {
            item.linetype = v.clone();
        }
        if let Some(v) = d.flags {
            item.flags = v;
        }
        if let Some(v) = &d.unknown_group_codes {
            item.unknown_group_codes = v.clone();
        }
    }
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        if other.color.is_some() {
            base.color = other.color;
        }
        if other.linetype.is_some() {
            base.linetype = other.linetype;
        }
        if other.flags.is_some() {
            base.flags = other.flags;
        }
        if other.unknown_group_codes.is_some() {
            base.unknown_group_codes = other.unknown_group_codes;
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfLayerModified {
    pub name: String,
    pub diff: DxfLayerDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfLayerAdded {
    pub index: usize,
    pub layer: DxfLayer,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfLayersDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<DxfLayerModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<DxfLayerAdded>,
}

impl DxfLayersDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn apply(&self, base: &[DxfLayer]) -> Vec<DxfLayer> {
        let modified: Vec<(String, DxfLayerDiff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfLayer)> = self.added.iter().map(|a| (a.index, a.layer.clone())).collect();
        named_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn inverse(&self, base: &[DxfLayer]) -> Option<Self> {
        let modified: Vec<(String, <DxfLayer as DxfNamedElem>::Diff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfLayer)> = self.added.iter().map(|a| (a.index, a.layer.clone())).collect();
        let (removed, modified, added) = named_inverse(&self.removed, &modified, &added, base);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfLayerModified { name, diff }).collect(), added: added.into_iter().map(|(index, layer)| DxfLayerAdded { index, layer }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(String, DxfLayerDiff)> = d1.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d1a: Vec<(usize, DxfLayer)> = d1.added.into_iter().map(|a| (a.index, a.layer)).collect();
        let d2m: Vec<(String, DxfLayerDiff)> = d2.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d2a: Vec<(usize, DxfLayer)> = d2.added.into_iter().map(|a| (a.index, a.layer)).collect();
        let (removed, modified, added) = named_absorb_pair::<DxfLayer>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfLayerModified { name, diff }).collect(), added: added.into_iter().map(|(index, layer)| DxfLayerAdded { index, layer }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️LayerDiff
