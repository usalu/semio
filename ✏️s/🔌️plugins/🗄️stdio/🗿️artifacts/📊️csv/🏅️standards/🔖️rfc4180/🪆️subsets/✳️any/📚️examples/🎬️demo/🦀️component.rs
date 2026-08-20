//! 📚️ Example demo for stdio.csv.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
/// 📄️ Genuine RFC 4180 bytes for the demo snapshot (`encode_csv(demo_csv_snapshot())`).
pub const NATIVE_BYTES: &[u8] = include_str!("🖼️assets/📊️example.csv").as_bytes();
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON).await
}

//#region 🔖️P2P1BinaryFixtures
/// 🎒️ Genuine `encode_pack` bytes of the demo snapshot (P2-P1 `fixture_honesty_law`).
pub const PACK_BYTES: &[u8] = include_bytes!("🖼️assets/🎒️example.pack.semio");
/// 📡️ Genuine `encode_op` bytes of a real `CsvMutation` (P2-P1 `protocol_walk_law`, Spr facet).
pub const SPR_BYTES: &[u8] = include_bytes!("🖼️assets/📡️example.spr.semio");
//#endregion 🔖️P2P1BinaryFixtures

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::csv::schema::snapshot::{CsvField, CsvRecord};
    use crate::artifacts::csv::standards::v_rfc4180::subsets::any::schema::inferences::CsvInference;
    use crate::artifacts::csv::{CsvMutation, CsvSnapshot};
    use protocol::Inference;
    
    use store::os_store::test_support::{self, ExampleAsset, IoFidelityClass, SubsetRoundtripSpec};

    #[semio_framework_async_macros::async_test]
    async fn demo_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }

    struct CsvAnyRoundtrip;

    impl SubsetRoundtripSpec for CsvAnyRoundtrip {
        type Snapshot = CsvSnapshot;
        type Mutation = CsvMutation;
        type Inference = CsvInference;

        async fn dialect() -> store::os_io::ArtifactDialect {
            store::os_io::ArtifactDialect { artifact_kind: "s.stdio.csv".into(), standard: "rfc4180".into(), subset: "*".into() }
        }

        async fn fidelity() -> IoFidelityClass {
            IoFidelityClass::Exact
        }

        async fn drops() -> &'static [&'static str] {
            &[]
        }

        async fn parse_native(asset: &ExampleAsset<'_>) -> Result<Self::Snapshot, String> {
            let text = std::str::from_utf8(asset.bytes).map_err(|e| e.to_string())?;
            Ok(crate::artifacts::csv::schema::snapshot::decode_csv_with(text, true))
        }

        async fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String> {
            Ok(crate::artifacts::csv::schema::snapshot::encode_csv(snapshot).into_bytes())
        }

        async fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String> {
            let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
            Ok(crate::artifacts::csv::schema::snapshot::decode_csv_with(text, true))
        }

        async fn infer(snapshot: &Self::Snapshot) -> Self::Inference {
            CsvInference::infer(snapshot)
        }

        async fn sample_mutations(snapshot: &Self::Snapshot) -> Vec<Self::Mutation> {
            vec![CsvMutation::InsertRecord { index: snapshot.records.len(), record: CsvRecord { fields: vec![CsvField { value: "roundtrip".into(), quoted: false }] } }]
        }

        async fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>> {
            std::str::from_utf8(bytes).map_err(|e| vec![e.to_string()]).and_then(|text| crate::artifacts::csv::schema::snapshot::decode_csv(text).map_err(|e| vec![e])).map(|_| ())
        }

        async fn validate_negative(_bytes: &[u8]) -> Result<Vec<String>, String> {
            Err("SKIP:owning subset has no negative fixture".into())
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn demo_subset_integrated_roundtrip() {
        let asset = ExampleAsset { bytes: NATIVE_BYTES, text: Some(std::str::from_utf8(NATIVE_BYTES).expect("utf-8 csv")), provenance: "✳️any/📚️examples/🎬️demo/🖼️assets/📊️example.csv" };
        test_support::assert_subset_roundtrip::<CsvAnyRoundtrip>(&asset, None);
    }
}
