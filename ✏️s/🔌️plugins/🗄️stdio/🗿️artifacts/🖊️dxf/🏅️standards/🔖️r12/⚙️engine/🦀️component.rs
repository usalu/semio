//! ⚙️ DxfEngine — owns a real `DxfArtifact`.

use crate::artifacts::dxf::{DxfArtifact, DxfDiff, DxfMutation, DxfSnapshot, STDIO_DXF_DOCUMENT_SCHEMA};

//#region 🔖️DocumentHelpers
/// 🌱 Empty persisted snapshot.
pub fn empty_dxf_snapshot() -> DxfSnapshot {
    DxfSnapshot::default()
}
//#endregion 🔖️DocumentHelpers

//#region 🔖️Register
/// 🗂️ Registers codecs and the artifact schema descriptor.
pub fn register() {
    crate::artifacts::dxf::composer::register();
    register_artifact_schema();
    register_pilot_languages();
    store::register_document_codec(store::ArtifactCodec::of::<DxfSnapshot, DxfMutation>(STDIO_DXF_DOCUMENT_SCHEMA));
}

/// 📌️ Registers handcrafted facet grammars (text) and protocols (binary).
pub fn register_pilot_languages() {
    dsl::register_language(dsl::LanguageSpec {
        id: "stdio.dxf",
        extension: Some("dxf"),
        role: dsl::LanguageRole::Document,
        grammar: Some(crate::artifacts::dxf::schema::snapshot::text::COMPONENT_GRAMMAR_SEMIO),
        grammar_path: Some(crate::artifacts::dxf::schema::snapshot::text::COMPONENT_GRAMMAR_PATH),
        protocol: Some(crate::artifacts::dxf::schema::snapshot::binary::COMPONENT_PROTOCOL_SEMIO),
        protocol_path: Some(crate::artifacts::dxf::schema::snapshot::binary::COMPONENT_PROTOCOL_PATH),
        hooks: dsl::passthrough_hooks("stdio.dxf"),
    });
}

/// 📌️ Registers schema leaves for `s.stdio.dxf`.
pub fn register_artifact_schema() {
    ::schema::register_artifact_schema_descriptor(crate::artifacts::dxf::schema::dxf_artifact_schema_descriptor());
}
//#endregion 🔖️Register

//#region 🔖️ArtifactEngine
/// ⚙️ `stdio.dxf` artifact engine.
pub struct DxfEngine {
    artifact_state: DxfArtifact,
    snapshot_state: DxfSnapshot,
}

impl DxfEngine {
    /// 🏗️ Builds an engine from a persisted snapshot.
    pub fn new(snapshot: DxfSnapshot) -> Self {
        let artifact_state = DxfArtifact::from_snapshot(snapshot.clone());
        Self { artifact_state, snapshot_state: snapshot }
    }
}
//#endregion 🔖️ArtifactEngine

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_snapshot_matches_schema() {
        let snapshot = empty_dxf_snapshot();
        assert_eq!(snapshot.schema, STDIO_DXF_DOCUMENT_SCHEMA);
    }

    #[test]
    fn codec_round_trip() {
        let snap = empty_dxf_snapshot();
        let text = store::ArtifactDsl::print_dsl(&snap);
        let parsed = <DxfSnapshot as store::ArtifactDsl>::parse_dsl(&text).expect("parse");
        assert_eq!(parsed.schema, snap.schema);
        let bytes = store::ArtifactPack::encode_pack(&snap);
        let decoded = <DxfSnapshot as store::ArtifactPack>::decode_pack(&bytes).expect("decode");
        assert_eq!(decoded, snap);
    }

    //#region 🔖️CodecRetentionLaw
    /// 🔁️ decode→encode retains every field across every section (header vars incl. a point-
    /// component var, all three typed table kinds, an unmodeled table kind, a block with a
    /// nested entity, and every typed entity kind plus one raw-retained unmodeled kind) —
    /// documented NORMAL FORM (see `📸️snapshot` module docs): from the SECOND generation onward
    /// decode/encode is a true fixed point (source float/whitespace formatting isn't preserved,
    /// only semantic content).
    #[test]
    fn codec_retention_law() {
        use crate::artifacts::dxf::schema::snapshot::{
            parse_dxf_document, print_dxf_document, DxfEntity, DxfHeaderVar, DxfLayer, DxfLinetype,
            DxfOtherTable, DxfStyle, DxfTables, DxfTag, DxfValue,
        };
        let snap1 = DxfSnapshot {
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
            blocks: vec![crate::artifacts::dxf::schema::snapshot::DxfBlock {
                name: "MYBLOCK".into(),
                base_point: [0.0, 0.0, 0.0],
                entities: vec![DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] }],
                unknown_group_codes: vec![],
            }],
            entities: vec![
                DxfEntity::Line { start: [0.0, 0.0, 0.0], end: [1.0, 1.0, 0.0], layer: "0".into(), unknown_group_codes: vec![] },
                DxfEntity::Circle { center: [1.0, 1.0, 0.0], radius: 2.0, layer: "0".into(), unknown_group_codes: vec![] },
                DxfEntity::Other { kind: "3DFACE".into(), group_codes: vec![(10, DxfValue::Double { value: 0.0 })] },
            ],
        };
        let text2 = print_dxf_document(&snap1);
        let snap2 = parse_dxf_document(&text2).expect("re-parse");
        assert_eq!(snap1, snap2, "decode(encode(snap)) must be a fixed point");

        let text3 = print_dxf_document(&snap2);
        assert_eq!(text2, text3, "from generation 2 onward, print(parse(text)) must be a true text fixed point too");
    }
    //#endregion 🔖️CodecRetentionLaw
}
//#endregion 🧪️Tests
