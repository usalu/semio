//! 🧬️ DxfArtifact schema — full artifact state (mirrors `DxfSnapshot`'s persisted fields
//! one-for-one; see `📸️snapshot/🦀️.rs` module docs for the full typed-model rationale).

use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfCircle, DxfLine, DxfOther, DxfHeaderVar, DxfOtherTable, DxfTables};
use crate::DxfSnapshot;
use framework_schema::ArtifactSchema;

//#region 🔖️Artifact
/// 🧬️ Full `stdio.dxf` artifact state.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_value::RetireOwned)]
#[value(rename_all = "camelCase")]
#[artifact_schema(id = "s.stdio.dxf")]
pub struct DxfArtifact {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[value(default)]
    pub header_vars: Vec<DxfHeaderVar>,
    #[state(artifact)]
    #[value(default)]
    pub tables: DxfTables,
    #[state(artifact)]
    #[value(default)]
    pub other_tables: Vec<DxfOtherTable>,
    #[state(artifact)]
    #[value(default)]
    pub blocks: Vec<DxfBlock>,
    #[state(artifact)]
    #[value(default)]
    pub entities: Vec<DxfEntity>,
}
//#endregion 🔖️Artifact

//#region 🔖️Conversions
impl Default for DxfArtifact {
    fn default() -> Self {
        Self::from_snapshot(DxfSnapshot::default())
    }
}

impl DxfArtifact {
    /// 📸️ Persisted subset.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn to_snapshot(&self) -> DxfSnapshot {
        DxfSnapshot { schema: self.schema.clone(), header_vars: self.header_vars.clone(), tables: self.tables.clone(), other_tables: self.other_tables.clone(), blocks: self.blocks.clone(), entities: self.entities.clone() }
    }

    /// 🧬️ Builds a full artifact from a snapshot.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn from_snapshot(snapshot: DxfSnapshot) -> Self {
        Self { schema: snapshot.schema, header_vars: snapshot.header_vars, tables: snapshot.tables, other_tables: snapshot.other_tables, blocks: snapshot.blocks, entities: snapshot.entities }
    }

    /// 🔄 Writes persistent fields from a snapshot into this artifact.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn set_snapshot(&mut self, snapshot: DxfSnapshot) {
        self.schema = snapshot.schema;
        self.header_vars = snapshot.header_vars;
        self.tables = snapshot.tables;
        self.other_tables = snapshot.other_tables;
        self.blocks = snapshot.blocks;
        self.entities = snapshot.entities;
    }
}
//#endregion 🔖️Conversions

//#region 🔖️Descriptor
/// 🧬️ Descriptor for `s.stdio.dxf`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dxf_artifact_schema_descriptor() -> semio_framework_schema_registry::ArtifactSchemaDescriptor {
    semio_framework_schema_registry::ArtifactSchemaDescriptor {
        id: "s.stdio.dxf",
        artifact: semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") },
        snapshot: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("📸️snapshot/🦀️.rs"),
            typescript: include_str!("📸️snapshot/🟦️.ts"),
            graphql: include_str!("📸️snapshot/🔗️.graphql"),
            json_schema: include_str!("📸️snapshot/🔣️.json"),
            proto: include_str!("📸️snapshot/🛰️.proto"),
        },
        diff: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🔺️diff/🦀️.rs"),
            typescript: include_str!("🔺️diff/🟦️.ts"),
            graphql: include_str!("🔺️diff/🔗️.graphql"),
            json_schema: include_str!("🔺️diff/🔣️.json"),
            proto: include_str!("🔺️diff/🛰️.proto"),
        },
        mutations: semio_framework_schema_registry::FacetLeaves {
            rust: include_str!("🧬️mutations/🦀️.rs"),
            typescript: include_str!("🧬️mutations/🟦️.ts"),
            graphql: include_str!("🧬️mutations/🔗️.graphql"),
            json_schema: include_str!("🧬️mutations/🔣️.json"),
            proto: include_str!("🧬️mutations/🛰️.proto"),
        },
    }
}
//#endregion 🔖️Descriptor
//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets

//#region 🔖️DocumentHelpers
use crate::STDIO_DXF_DOCUMENT_SCHEMA;

/// 🌱 Empty persisted snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn empty_dxf_snapshot() -> DxfSnapshot {
    DxfSnapshot::default()
}

/// 🧬️ Genuinely 2-level-nested (a `BLOCK` with a nested entity), every-section demo snapshot —
/// the single source of truth for `fixture_honesty_law`'s shipped `🗣️.dsl.semio`/
/// `🎒️.pack.semio` fixtures AND `grammar_conformance_law`/`protocol_walk_law`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn demo_dxf_snapshot() -> DxfSnapshot {
    use crate::schema::snapshot::{DxfBlock, DxfEntity, DxfCircle, DxfLine, DxfOther, DxfHeaderVar, DxfLayer, DxfLinetype, DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue};
    DxfSnapshot {
        schema: STDIO_DXF_DOCUMENT_SCHEMA.into(),
        header_vars: vec![
            DxfHeaderVar { name: "$ACADVER".into(), group_code: 1, value: DxfValue::Str { value: "AC1009".into() }, extra_group_codes: vec![] },
            DxfHeaderVar { name: "$INSBASE".into(), group_code: 10, value: DxfValue::Point { value: [1.0, 2.0, 3.0] }, extra_group_codes: vec![] },
        ],
        tables: DxfTables {
            layers: vec![DxfLayer { name: "0".into(), color: 7, linetype: "CONTINUOUS".into(), flags: 0, unknown_group_codes: vec![] }],
            styles: vec![DxfStyle { name: "STANDARD".into(), flags: 0, font_name: "txt".into(), unknown_group_codes: vec![] }],
            linetypes: vec![DxfLinetype { name: "CONTINUOUS".into(), flags: 0, description: "Solid".into(), unknown_group_codes: vec![] }],
        },
        other_tables: vec![DxfOtherTable { name: "VPORT".into(), tags: vec![DxfTag { code: 2, value: "*ACTIVE".into() }] }],
        blocks: vec![DxfBlock { name: "MYBLOCK".into(), base_point: [0.0, 0.0, 0.0], entities: vec![DxfEntity::Line(DxfLine { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] })], unknown_group_codes: vec![] }],
        entities: vec![
            DxfEntity::Line(DxfLine { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }),
            DxfEntity::Circle(DxfCircle { center: [1.0, 1.0, 0.0], radius: 2.0, layer: "0".into(), unknown_group_codes: vec![] }),
            DxfEntity::Other(DxfOther { kind: "3DFACE".into(), group_codes: vec![(10, DxfValue::Double { value: 0.0 })] }),
        ],
    }
}
//#endregion 🔖️DocumentHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
