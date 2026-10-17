//! 🎨️ Style, linetype and table diffs of the DXF diff: the name-keyed style and linetype collection diffs and the tables diff that combines them. See [`super`].

use super::*;

//#region 🔖️StyleDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfStyleDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub font_name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}

impl DxfNamedElem for DxfStyle {
    type Diff = DxfStyleDiff;
    fn key(&self) -> &str {
        &self.name
    }
    fn field_changes(a: &Self, b: &Self) -> Self::Diff {
        DxfStyleDiff {
            flags: (a.flags != b.flags).then_some(b.flags),
            font_name: (a.font_name != b.font_name).then(|| b.font_name.clone()),
            unknown_group_codes: (a.unknown_group_codes != b.unknown_group_codes).then(|| b.unknown_group_codes.clone()),
        }
    }
    fn diff_inverse(d: &Self::Diff, base: &Self) -> Self::Diff {
        DxfStyleDiff { flags: d.flags.map(|_| base.flags), font_name: d.font_name.as_ref().map(|_| base.font_name.clone()), unknown_group_codes: d.unknown_group_codes.as_ref().map(|_| base.unknown_group_codes.clone()) }
    }
    fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let Some(v) = d.flags {
            item.flags = v;
        }
        if let Some(v) = &d.font_name {
            item.font_name = v.clone();
        }
        if let Some(v) = &d.unknown_group_codes {
            item.unknown_group_codes = v.clone();
        }
    }
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        if other.flags.is_some() {
            base.flags = other.flags;
        }
        if other.font_name.is_some() {
            base.font_name = other.font_name;
        }
        if other.unknown_group_codes.is_some() {
            base.unknown_group_codes = other.unknown_group_codes;
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfStyleModified {
    pub name: String,
    pub diff: DxfStyleDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfStyleAdded {
    pub index: usize,
    pub style: DxfStyle,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfStylesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<DxfStyleModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<DxfStyleAdded>,
}

impl DxfStylesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn apply(&self, base: &[DxfStyle]) -> Vec<DxfStyle> {
        let modified: Vec<(String, DxfStyleDiff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfStyle)> = self.added.iter().map(|a| (a.index, a.style.clone())).collect();
        named_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn inverse(&self, base: &[DxfStyle]) -> Option<Self> {
        let modified: Vec<(String, <DxfStyle as DxfNamedElem>::Diff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfStyle)> = self.added.iter().map(|a| (a.index, a.style.clone())).collect();
        let (removed, modified, added) = named_inverse(&self.removed, &modified, &added, base);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfStyleModified { name, diff }).collect(), added: added.into_iter().map(|(index, style)| DxfStyleAdded { index, style }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(String, DxfStyleDiff)> = d1.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d1a: Vec<(usize, DxfStyle)> = d1.added.into_iter().map(|a| (a.index, a.style)).collect();
        let d2m: Vec<(String, DxfStyleDiff)> = d2.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d2a: Vec<(usize, DxfStyle)> = d2.added.into_iter().map(|a| (a.index, a.style)).collect();
        let (removed, modified, added) = named_absorb_pair::<DxfStyle>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfStyleModified { name, diff }).collect(), added: added.into_iter().map(|(index, style)| DxfStyleAdded { index, style }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️StyleDiff

//#region 🔖️LinetypeDiff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfLinetypeDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub unknown_group_codes: Option<Vec<(i32, DxfValue)>>,
}

impl DxfNamedElem for DxfLinetype {
    type Diff = DxfLinetypeDiff;
    fn key(&self) -> &str {
        &self.name
    }
    fn field_changes(a: &Self, b: &Self) -> Self::Diff {
        DxfLinetypeDiff {
            flags: (a.flags != b.flags).then_some(b.flags),
            description: (a.description != b.description).then(|| b.description.clone()),
            unknown_group_codes: (a.unknown_group_codes != b.unknown_group_codes).then(|| b.unknown_group_codes.clone()),
        }
    }
    fn diff_inverse(d: &Self::Diff, base: &Self) -> Self::Diff {
        DxfLinetypeDiff { flags: d.flags.map(|_| base.flags), description: d.description.as_ref().map(|_| base.description.clone()), unknown_group_codes: d.unknown_group_codes.as_ref().map(|_| base.unknown_group_codes.clone()) }
    }
    fn diff_apply(d: &Self::Diff, item: &mut Self) {
        if let Some(v) = d.flags {
            item.flags = v;
        }
        if let Some(v) = &d.description {
            item.description = v.clone();
        }
        if let Some(v) = &d.unknown_group_codes {
            item.unknown_group_codes = v.clone();
        }
    }
    fn diff_absorb(base: &mut Self::Diff, other: Self::Diff) {
        if other.flags.is_some() {
            base.flags = other.flags;
        }
        if other.description.is_some() {
            base.description = other.description;
        }
        if other.unknown_group_codes.is_some() {
            base.unknown_group_codes = other.unknown_group_codes;
        }
    }
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfLinetypeModified {
    pub name: String,
    pub diff: DxfLinetypeDiff,
}

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfLinetypeAdded {
    pub index: usize,
    pub linetype: DxfLinetype,
}

#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfLinetypesDiff {
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub removed: Vec<String>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub modified: Vec<DxfLinetypeModified>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub added: Vec<DxfLinetypeAdded>,
}

impl DxfLinetypesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.removed.is_empty() && self.modified.is_empty() && self.added.is_empty()
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn apply(&self, base: &[DxfLinetype]) -> Vec<DxfLinetype> {
        let modified: Vec<(String, DxfLinetypeDiff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfLinetype)> = self.added.iter().map(|a| (a.index, a.linetype.clone())).collect();
        named_apply(base, &self.removed, &modified, &added)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn inverse(&self, base: &[DxfLinetype]) -> Option<Self> {
        let modified: Vec<(String, <DxfLinetype as DxfNamedElem>::Diff)> = self.modified.iter().map(|m| (m.name.clone(), m.diff.clone())).collect();
        let added: Vec<(usize, DxfLinetype)> = self.added.iter().map(|a| (a.index, a.linetype.clone())).collect();
        let (removed, modified, added) = named_inverse(&self.removed, &modified, &added, base);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfLinetypeModified { name, diff }).collect(), added: added.into_iter().map(|(index, linetype)| DxfLinetypeAdded { index, linetype }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn absorb(d1: Self, d2: Self) -> Option<Self> {
        let d1m: Vec<(String, DxfLinetypeDiff)> = d1.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d1a: Vec<(usize, DxfLinetype)> = d1.added.into_iter().map(|a| (a.index, a.linetype)).collect();
        let d2m: Vec<(String, DxfLinetypeDiff)> = d2.modified.into_iter().map(|m| (m.name, m.diff)).collect();
        let d2a: Vec<(usize, DxfLinetype)> = d2.added.into_iter().map(|a| (a.index, a.linetype)).collect();
        let (removed, modified, added) = named_absorb_pair::<DxfLinetype>(&d1.removed, &d1m, &d1a, &d2.removed, &d2m, &d2a);
        let d = Self { removed, modified: modified.into_iter().map(|(name, diff)| DxfLinetypeModified { name, diff }).collect(), added: added.into_iter().map(|(index, linetype)| DxfLinetypeAdded { index, linetype }).collect() };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️LinetypeDiff

//#region 🔖️TablesDiff
/// 🔺️ Groups the three name-keyed table diffs — `DxfTables` itself is a weak grouping struct
/// (not a collection), so its diff is a plain per-field `Option<...>` struct, one per sub-collection.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
pub struct DxfTablesDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub layers: Option<DxfLayersDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub styles: Option<DxfStylesDiff>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub linetypes: Option<DxfLinetypesDiff>,
}

impl DxfTablesDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty(&self) -> bool {
        self.layers.as_ref().is_none_or(DxfLayersDiff::is_empty) && self.styles.as_ref().is_none_or(DxfStylesDiff::is_empty) && self.linetypes.as_ref().is_none_or(DxfLinetypesDiff::is_empty)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn apply(&self, base: &DxfTables) -> DxfTables {
        DxfTables {
            layers: match &self.layers {
                Some(d) => d.apply(&base.layers),
                None => base.layers.clone(),
            },
            styles: match &self.styles {
                Some(d) => d.apply(&base.styles),
                None => base.styles.clone(),
            },
            linetypes: match &self.linetypes {
                Some(d) => d.apply(&base.linetypes),
                None => base.linetypes.clone(),
            },
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn inverse(&self, base: &DxfTables) -> Option<Self> {
        let d = Self {
            layers: self.layers.as_ref().and_then(|layers| layers.inverse(&base.layers)),
            styles: self.styles.as_ref().and_then(|styles| styles.inverse(&base.styles)),
            linetypes: self.linetypes.as_ref().and_then(|linetypes| linetypes.inverse(&base.linetypes)),
        };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub(super) fn absorb(a: Self, b: Self) -> Option<Self> {
        let layers = match (a.layers, b.layers) {
            (None, None) => None,
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (Some(x), Some(y)) => DxfLayersDiff::absorb(x, y),
        };
        let styles = match (a.styles, b.styles) {
            (None, None) => None,
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (Some(x), Some(y)) => DxfStylesDiff::absorb(x, y),
        };
        let linetypes = match (a.linetypes, b.linetypes) {
            (None, None) => None,
            (Some(x), None) => Some(x),
            (None, Some(y)) => Some(y),
            (Some(x), Some(y)) => DxfLinetypesDiff::absorb(x, y),
        };
        let d = Self { layers, styles, linetypes };
        if d.is_empty() {
            None
        } else {
            Some(d)
        }
    }
}
//#endregion 🔖️TablesDiff
