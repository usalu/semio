//! 🔺️ SemioKitDiff — sparse keyed-row diff over `SemioKitSnapshot`. Six independently diffable fields: the four ordered
//! collections (`types`, `designs`, `objects`, `models`) and the link collection (`representations`) each carry an index-keyed
//! triple — removed base indices, modified rows (a sparse per-row diff) and added rows with their final position — while the
//! single optional `properties` CHILD slot carries its replacement (`Some(None)` clears it). No whole-list slot anywhere.

use crate::standards::v1::subsets::base::schema::triples::{absorb_indexed_slot, apply_indexed_rows, inverse_indexed_rows, validate_indexed_triple, IndexedRow, IndexedTripleDiff, Replace};
use crate::standards::v1::subsets::kit::schema::snapshot::{SemioKitConnection, SemioKitDesign, SemioKitPiece, SemioKitSnapshot, SemioKitType};
use crate::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot;
use crate::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot;
use crate::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot;
use framework_schema::ArtifactSchema;
use protocol::MutationDiff;

//#region 🔖️RowDiffs
/// 🧩 Sparse diff of one type: each present field is the new value.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioKitTypeDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
}

impl IndexedRow<SemioKitType> for SemioKitTypeDiff {
    fn apply_row(&self, base: &SemioKitType) -> SemioKitType {
        SemioKitType { id: base.id.clone(), name: self.name.clone().unwrap_or_else(|| base.name.clone()), category: self.category.clone().unwrap_or_else(|| base.category.clone()) }
    }
    fn inverse_row(&self, base: &SemioKitType) -> Self {
        Self { name: self.name.as_ref().map(|_| base.name.clone()), category: self.category.as_ref().map(|_| base.category.clone()) }
    }
    fn absorb_row(&mut self, other: Self) {
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.category.is_some() {
            self.category = other.category;
        }
    }
    fn row_is_empty(&self) -> bool {
        self.name.is_none() && self.category.is_none()
    }
}

/// 🧩 Sparse diff of one design: `name`, and the design's whole `pieces` / `connections` content when the design is edited.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioKitDesignDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pieces: Option<Vec<SemioKitPiece>>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub connections: Option<Vec<SemioKitConnection>>,
}

impl IndexedRow<SemioKitDesign> for SemioKitDesignDiff {
    fn apply_row(&self, base: &SemioKitDesign) -> SemioKitDesign {
        SemioKitDesign {
            id: base.id.clone(),
            name: self.name.clone().unwrap_or_else(|| base.name.clone()),
            pieces: self.pieces.clone().unwrap_or_else(|| base.pieces.clone()),
            connections: self.connections.clone().unwrap_or_else(|| base.connections.clone()),
        }
    }
    fn inverse_row(&self, base: &SemioKitDesign) -> Self {
        Self { name: self.name.as_ref().map(|_| base.name.clone()), pieces: self.pieces.as_ref().map(|_| base.pieces.clone()), connections: self.connections.as_ref().map(|_| base.connections.clone()) }
    }
    fn absorb_row(&mut self, other: Self) {
        if other.name.is_some() {
            self.name = other.name;
        }
        if other.pieces.is_some() {
            self.pieces = other.pieces;
        }
        if other.connections.is_some() {
            self.connections = other.connections;
        }
    }
    fn row_is_empty(&self) -> bool {
        self.name.is_none() && self.pieces.is_none() && self.connections.is_none()
    }
}

/// 🔗️ Sparse diff of one representation link: only its pin ever changes in place.
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct SemioKitLinkDiff {
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub pin: Option<store::LinkPin>,
}

impl IndexedRow<store::ArtifactLink> for SemioKitLinkDiff {
    fn apply_row(&self, base: &store::ArtifactLink) -> store::ArtifactLink {
        store::ArtifactLink { target: base.target.clone(), pin: self.pin.clone().unwrap_or_else(|| base.pin.clone()), role: base.role.clone() }
    }
    fn inverse_row(&self, base: &store::ArtifactLink) -> Self {
        Self { pin: self.pin.as_ref().map(|_| base.pin.clone()) }
    }
    fn absorb_row(&mut self, other: Self) {
        if other.pin.is_some() {
            self.pin = other.pin;
        }
    }
    fn row_is_empty(&self) -> bool {
        self.pin.is_none()
    }
}
//#endregion 🔖️RowDiffs

//#region 🔖️Diff
#[derive(Clone, Debug, Default, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.semio.kit.diff")]
pub struct SemioKitDiff {
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub types: Option<IndexedTripleDiff<SemioKitTypeDiff, SemioKitType>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub designs: Option<IndexedTripleDiff<SemioKitDesignDiff, SemioKitDesign>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub objects: Option<IndexedTripleDiff<Replace<store::ArtifactChild<SemioObjectSnapshot>>, store::ArtifactChild<SemioObjectSnapshot>>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub models: Option<IndexedTripleDiff<Replace<store::ArtifactChild<SemioModelSnapshot>>, store::ArtifactChild<SemioModelSnapshot>>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub properties: Option<Option<store::ArtifactChild<SemioValueSnapshot>>>,
    #[state(artifact)]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub representations: Option<IndexedTripleDiff<SemioKitLinkDiff, store::ArtifactLink>>,
}

impl SemioKitDiff {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn is_empty_diff(&self) -> bool {
        self.types.as_ref().is_none_or(IndexedTripleDiff::is_unchanged)
            && self.designs.as_ref().is_none_or(IndexedTripleDiff::is_unchanged)
            && self.objects.as_ref().is_none_or(IndexedTripleDiff::is_unchanged)
            && self.models.as_ref().is_none_or(IndexedTripleDiff::is_unchanged)
            && self.properties.is_none()
            && self.representations.as_ref().is_none_or(IndexedTripleDiff::is_unchanged)
    }
}

impl MutationDiff<SemioKitSnapshot> for SemioKitDiff {
    fn apply(&self, base: &SemioKitSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<SemioKitSnapshot> {
        let mut next = base.clone();
        if let Some(types) = &self.types {
            validate_indexed_triple(types, base.types.len(), ["types"])?;
            next.types = apply_indexed_rows(types, &base.types);
        }
        if let Some(designs) = &self.designs {
            validate_indexed_triple(designs, base.designs.len(), ["designs"])?;
            next.designs = apply_indexed_rows(designs, &base.designs);
        }
        if let Some(objects) = &self.objects {
            validate_indexed_triple(objects, base.objects.len(), ["objects"])?;
            next.objects = apply_indexed_rows(objects, &base.objects);
        }
        if let Some(models) = &self.models {
            validate_indexed_triple(models, base.models.len(), ["models"])?;
            next.models = apply_indexed_rows(models, &base.models);
        }
        if let Some(properties) = &self.properties {
            next.properties = properties.clone();
        }
        if let Some(representations) = &self.representations {
            validate_indexed_triple(representations, base.representations.len(), ["representations"])?;
            next.representations = apply_indexed_rows(representations, &base.representations);
        }
        Ok(next)
    }

    fn absorb(&mut self, other: Self) {
        absorb_indexed_slot(&mut self.types, other.types);
        absorb_indexed_slot(&mut self.designs, other.designs);
        absorb_indexed_slot(&mut self.objects, other.objects);
        absorb_indexed_slot(&mut self.models, other.models);
        if other.properties.is_some() {
            self.properties = other.properties;
        }
        absorb_indexed_slot(&mut self.representations, other.representations);
    }
}

/// 🧮️ `kit`'s own `DiffAlgebra` — required by the `✉️base` envelope's own dispatch. `inverse` is the concrete negative diff of the
/// keyed rows (the `properties` slot restores the base child); `between` is the positional sync/import delta, never used by
/// mutation leaves.
impl protocol::command::DiffAlgebra<SemioKitSnapshot> for SemioKitDiff {
    fn inverse(&self, base: &SemioKitSnapshot) -> Self {
        SemioKitDiff {
            types: self.types.as_ref().map(|d| inverse_indexed_rows(d, &base.types)),
            designs: self.designs.as_ref().map(|d| inverse_indexed_rows(d, &base.designs)),
            objects: self.objects.as_ref().map(|d| inverse_indexed_rows(d, &base.objects)),
            models: self.models.as_ref().map(|d| inverse_indexed_rows(d, &base.models)),
            properties: self.properties.as_ref().map(|_| base.properties.clone()),
            representations: self.representations.as_ref().map(|d| inverse_indexed_rows(d, &base.representations)),
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
    use crate::standards::v1::subsets::base::schema::triples::{IndexAdded, IndexModified};
    use crate::standards::v1::subsets::kit::schema::snapshot::demo_kit_snapshot;
    let demo = demo_kit_snapshot();
    vec![
        SemioKitDiff::default(),
        SemioKitDiff { types: Some(IndexedTripleDiff { added: demo.types.iter().cloned().enumerate().map(|(index, item)| IndexAdded { index, item }).collect(), ..Default::default() }), ..Default::default() },
        SemioKitDiff { properties: Some(None), ..Default::default() },
        SemioKitDiff { representations: Some(IndexedTripleDiff { added: demo.representations.iter().cloned().enumerate().map(|(index, item)| IndexAdded { index, item }).collect(), ..Default::default() }), ..Default::default() },
        SemioKitDiff {
            types: Some(IndexedTripleDiff { removed: vec![0], modified: vec![IndexModified { index: 1, diff: SemioKitTypeDiff { name: Some("Renamed".into()), category: None } }], ..Default::default() }),
            designs: Some(IndexedTripleDiff {
                modified: vec![IndexModified { index: 0, diff: SemioKitDesignDiff { name: Some("Edited".into()), pieces: Some(demo.designs[0].pieces.clone()), connections: Some(Vec::new()) } }],
                ..Default::default()
            }),
            ..Default::default()
        },
    ]
}
//#endregion 🔖️Demo

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
