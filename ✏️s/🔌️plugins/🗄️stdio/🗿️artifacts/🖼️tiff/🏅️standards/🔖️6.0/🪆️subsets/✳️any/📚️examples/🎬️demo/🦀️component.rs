//! 📚️ Example demo for stdio.tiff.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
pub async fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
/// 🖼️ Genuine `encode_tiff(demo_tiff_snapshot())` bytes (populated by engine fixture honesty).
pub const NATIVE_BYTES: &[u8] = include_bytes!("🖼️assets/🖼️example.tiff");
pub async fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON).await
}

#[cfg(test)]
mod tests {
    use super::*;
    #[semio_framework_async_macros::async_test]
    async fn demo_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }

    /// 🧪️ Ticket 26/08/12/INTRODUCE-INFERENCE-SCHEMA-FAMILY-WITH-DEPENDENCY-AWARE-CACHING's
    /// inference laws, exercised against this example's own real fixture (`PRIMARY_TEXT`,
    /// parsed through the real `ArtifactDsl` codec — not a hand-built stub).
    #[semio_framework_async_macros::async_test]
    async fn inference_determinism_law() {
        use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::inferences::TiffInference;
        use crate::artifacts::tiff::TiffSnapshot;
        use protocol::Inference;
        let snapshot = <TiffSnapshot as store::ArtifactDsl>::parse_dsl(PRIMARY_TEXT).expect("demo fixture must parse");
        assert_eq!(TiffInference::infer(&snapshot), TiffInference::infer(&snapshot));
    }

    #[semio_framework_async_macros::async_test]
    async fn inference_default_law() {
        use crate::artifacts::tiff::standards::v6_0::subsets::any::schema::inferences::TiffInference;
        use crate::artifacts::tiff::TiffSnapshot;
        use protocol::Inference;
        assert_eq!(TiffInference::infer(&TiffSnapshot::default()), TiffInference::default());
    }

    //#region 🧪️SubsetRoundtrip
    struct TiffAnyRoundtrip;

    impl store::os_store::test_support::SubsetRoundtripSpec for TiffAnyRoundtrip {
        type Snapshot = crate::artifacts::tiff::TiffSnapshot;
        type Mutation = crate::artifacts::tiff::TiffMutation;
        type Inference = crate::artifacts::tiff::standards::v6_0::subsets::any::schema::inferences::TiffInference;

        async fn dialect() -> store::os_io::ArtifactDialect {
            store::os_io::ArtifactDialect { artifact_kind: "s.stdio.tiff".into(), standard: "6.0".into(), subset: "*".into() }
        }

        async fn fidelity() -> store::os_store::test_support::IoFidelityClass {
            store::os_store::test_support::IoFidelityClass::Exact
        }

        async fn drops() -> &'static [&'static str] {
            &[]
        }

        async fn parse_native(asset: &store::os_store::test_support::ExampleAsset<'_>) -> Result<Self::Snapshot, String> {
            crate::artifacts::tiff::engine::decode_tiff(asset.bytes)
        }

        async fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String> {
            crate::artifacts::tiff::engine::encode_tiff(snapshot)
        }

        async fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String> {
            crate::artifacts::tiff::engine::decode_tiff(bytes)
        }

        async fn infer(snapshot: &Self::Snapshot) -> Self::Inference {
            use protocol::Inference;
            Self::Inference::infer(snapshot)
        }

        async fn sample_mutations(snapshot: &Self::Snapshot) -> Vec<Self::Mutation> {
            use crate::artifacts::tiff::schema::snapshot::{TiffFieldType, TiffValues, TAG_IMAGE_WIDTH};
            let width = snapshot
                .ifds
                .first()
                .and_then(|ifd| ifd.entries.iter().find(|t| t.tag == TAG_IMAGE_WIDTH))
                .and_then(|t| match &t.values {
                    TiffValues::Long(v) => v.first().copied(),
                    _ => None,
                })
                .unwrap_or(1);
            vec![crate::artifacts::tiff::TiffMutation::SetTag { ifd_index: 0, tag: TAG_IMAGE_WIDTH, kind: TiffFieldType::Long, values: TiffValues::Long(vec![width + 1]) }]
        }

        async fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>> {
            crate::artifacts::tiff::engine::decode_tiff(bytes).map(|_| ()).map_err(|e| vec![e])
        }

        async fn validate_negative(_bytes: &[u8]) -> Result<Vec<String>, String> {
            Err("SKIP:owning subset has no negative fixture".into())
        }
    }

    #[semio_framework_async_macros::async_test]
    async fn demo_subset_integrated_roundtrip() {
        let asset = store::os_store::test_support::ExampleAsset { bytes: NATIVE_BYTES, text: None, provenance: "✳️any/📚️examples/🎬️demo/🖼️assets/🖼️example.tiff" };
        store::os_store::test_support::assert_subset_roundtrip::<TiffAnyRoundtrip>(&asset, None);
    }
    //#endregion 🧪️SubsetRoundtrip
}
