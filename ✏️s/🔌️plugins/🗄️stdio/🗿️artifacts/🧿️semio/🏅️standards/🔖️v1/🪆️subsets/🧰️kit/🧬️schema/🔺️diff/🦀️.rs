//! 🔺️ SemioKitDiff — sparse per-field diff over `SemioKitSnapshot`. Six independently diffable
//! fields, each an `Option<…>` slot (`None` = untouched by this diff) — the same per-field shape
//! `📦️object`'s diff facet uses, scaled up to six fields including two CHILD collections, one
//! optional CHILD slot, and one LINK collection. Every mutation triad's own `🔺️diff` leaf builds
//! the touched field's whole new value directly from `(payload, base)`, never apply-then-capture
//! — `📓️taxonomy.md`'s whole-list-replace convention, unchanged by D2's resolution (Concern B:
//! the shape itself was never the defect).

use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitDesign, SemioKitSnapshot, SemioKitType};
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;

//#region 🔖️ListWrappers
/// 📋 Whole-list wrappers, one per collection field — every mutation triad rebuilds the full
/// ordered `values` vec from `base` and wraps it here (`🔤️text`'s own `SemioTextRunList` shape).
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioKitTypeList {
    pub values: Vec<SemioKitType>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioKitDesignList {
    pub values: Vec<SemioKitDesign>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioKitObjectChildList {
    pub values: Vec<store::ArtifactChild<SemioObjectSnapshot>>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioKitModelChildList {
    pub values: Vec<store::ArtifactChild<SemioModelSnapshot>>,
}
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase", default)]
pub struct SemioKitLinkList {
    pub values: Vec<store::ArtifactLink>,
}
//#endregion 🔖️ListWrappers

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.kit.diff")]
pub struct SemioKitDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<SemioKitTypeList>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub designs: Option<SemioKitDesignList>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<SemioKitObjectChildList>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub models: Option<SemioKitModelChildList>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Option<store::ArtifactChild<SemioValueSnapshot>>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub representations: Option<SemioKitLinkList>,
}

impl SemioKitDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.types.is_none() && self.designs.is_none() && self.objects.is_none() && self.models.is_none() && self.properties.is_none() && self.representations.is_none()
    }
}

impl MutationDiff<SemioKitSnapshot> for SemioKitDiff {
    fn apply(&self, base: &SemioKitSnapshot, capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SemioKitSnapshot> {
        let mut next = base.clone();
        if let Some(t) = &self.types {
            next.types = t.values.clone();
        }
        if let Some(d) = &self.designs {
            next.designs = d.values.clone();
        }
        if let Some(o) = &self.objects {
            next.objects = o.values.clone();
        }
        if let Some(m) = &self.models {
            next.models = m.values.clone();
        }
        if let Some(p) = &self.properties {
            next.properties = p.clone();
        }
        if let Some(r) = &self.representations {
            next.representations = r.values.clone();
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        if other.types.is_some() {
            self.types = other.types;
        }
        if other.designs.is_some() {
            self.designs = other.designs;
        }
        if other.objects.is_some() {
            self.objects = other.objects;
        }
        if other.models.is_some() {
            self.models = other.models;
        }
        if other.properties.is_some() {
            self.properties = other.properties;
        }
        if other.representations.is_some() {
            self.representations = other.representations;
        }
    }
}

/// 🧮️ `kit`'s own `DiffAlgebra` — required by the `✉️base` envelope's own dispatch.
impl protocol::command::DiffAlgebra<SemioKitSnapshot> for SemioKitDiff {
    fn between(base: &SemioKitSnapshot, other: &SemioKitSnapshot) -> Self {
        SemioKitDiff {
            types: (base.types != other.types).then(|| SemioKitTypeList { values: other.types.clone() }),
            designs: (base.designs != other.designs).then(|| SemioKitDesignList { values: other.designs.clone() }),
            objects: (base.objects != other.objects).then(|| SemioKitObjectChildList { values: other.objects.clone() }),
            models: (base.models != other.models).then(|| SemioKitModelChildList { values: other.models.clone() }),
            properties: (base.properties != other.properties).then(|| other.properties.clone()),
            representations: (base.representations != other.representations).then(|| SemioKitLinkList { values: other.representations.clone() }),
        }
    }
    fn inverse(&self, base: &SemioKitSnapshot) -> Self {
        SemioKitDiff {
            types: self.types.as_ref().map(|_| SemioKitTypeList { values: base.types.clone() }),
            designs: self.designs.as_ref().map(|_| SemioKitDesignList { values: base.designs.clone() }),
            objects: self.objects.as_ref().map(|_| SemioKitObjectChildList { values: base.objects.clone() }),
            models: self.models.as_ref().map(|_| SemioKitModelChildList { values: base.models.clone() }),
            properties: self.properties.as_ref().map(|_| base.properties.clone()),
            representations: self.representations.as_ref().map(|_| SemioKitLinkList { values: base.representations.clone() }),
        }
    }
    fn is_empty(&self) -> bool {
        self.is_empty_diff()
    }
}
//#endregion 🔖️Diff

//#region 🔖️HandcraftedDiffCodec















//#endregion 🔖️HandcraftedDiffCodec

//#region 🔖️Demo
/// 🌱 Representative `SemioKitDiff` cases — single source of truth for
/// `diff_grammar_conformance_law`/`protocol_walk_law` in `🚪️io/🦀️.rs`.
#[cfg(test)]
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn demo_diff_cases() -> Vec<SemioKitDiff> {
    use crate::standards::v1::subsets::kit::schema::snapshot::demo_kit_snapshot;
    vec![
        SemioKitDiff::default(),
        SemioKitDiff { types: Some(SemioKitTypeList { values: demo_kit_snapshot().types }), ..Default::default() },
        SemioKitDiff { properties: Some(None), ..Default::default() },
        SemioKitDiff { representations: Some(SemioKitLinkList { values: demo_kit_snapshot().representations }), ..Default::default() },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
