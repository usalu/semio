//! 📦️ Sourcing curate app — binary document surface + laws (constitutional: pack).

use sourcing::CurateDocument;
use store::PackError;

/// 📦️ Encodes a `CurateDocument` to its binary pack form.
pub fn encode(document: &CurateDocument) -> Vec<u8> {
    store::DocumentPack::encode_pack(document)
}

/// 📖️ Decodes a `CurateDocument` from its binary pack form.
pub fn decode(bytes: &[u8]) -> Result<CurateDocument, PackError> {
    <CurateDocument as store::DocumentPack>::decode_pack(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_round_trips_and_agrees_with_dsl() {
        let document = sourcing_dsl::parse_dsl(sourcing_dsl::DEMO_STOCK_TEXT).expect("parse demo-stock example");
        store::test_support::assert_dsl_pack_equivalence(&document);
        let bytes = encode(&document);
        assert_eq!(decode(&bytes).expect("decode"), document);
    }

    #[test]
    fn pack_round_trips_a_mesh_kind_and_a_curated_selection() {
        use sourcing::{GeometryRecipe, ObjectKind};

        let mut document = CurateDocument {
            stock: vec![ObjectKind {
                id: "beam-mesh-custom".into(),
                name: "Custom \"Beam\" \\ Mesh".into(),
                module_id: "beams".into(),
                typology_path: vec!["beams".into(), "steel".into()],
                availability: 5,
                geometry: Box::new(GeometryRecipe::Mesh { positions: vec![0.0, 0.0, 0.0, 1.0, 0.0, 0.0], normals: vec![0.0, 1.0, 0.0, 0.0, 1.0, 0.0], indices: vec![0, 1, 2] }),
            }],
            ..Default::default()
        };
        document.curated = vec![sourcing::CuratedItem { object_id: "beam-mesh-custom".into(), count: 2 }];
        store::test_support::assert_dsl_pack_equivalence(&document);
    }
}
//#endregion 🧪️Tests
