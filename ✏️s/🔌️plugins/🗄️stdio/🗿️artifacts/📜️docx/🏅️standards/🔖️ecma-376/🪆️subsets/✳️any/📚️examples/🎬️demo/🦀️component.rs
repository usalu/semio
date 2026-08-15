//! 📚️ Example demo for stdio.docx.

use semio_framework_plugin::{ExampleSource, LocalizedLabel};

pub const ID: &str = "demo";
pub fn label() -> LocalizedLabel {
    LocalizedLabel::native("Demo", "Demo")
}
pub const ICON: &str = "file";
pub const PRIMARY_TEXT: &str = include_str!("🖼️assets/🗣️example.dsl.semio");
/// 📦️ Genuine `encode_docx(demo_docx_snapshot())` bytes (populated by engine fixture honesty).
pub const NATIVE_BYTES: &[u8] = include_bytes!("🖼️assets/📜️example.docx");
pub fn source() -> ExampleSource {
    ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::artifacts::docx::standards::v_ecma_376::subsets::any::schema::inferences::DocxInference;
    use crate::artifacts::docx::{DocxMutation, DocxSnapshot};
    use protocol::Inference;
    use semio_framework_plugin::{Dialect, StandardId, SubsetId};
    use store::os_store::test_support::{self, ExampleAsset, IoFidelityClass, SubsetRoundtripSpec};

    #[test]
    fn demo_source_nonempty() {
        assert!(!PRIMARY_TEXT.is_empty());
        let _ = source();
    }

    struct DocxAnyRoundtrip;

    impl SubsetRoundtripSpec for DocxAnyRoundtrip {
        type Snapshot = DocxSnapshot;
        type Mutation = DocxMutation;
        type Inference = DocxInference;

        fn dialect() -> store::os_io::ArtifactDialect {
            store::os_io::ArtifactDialect { artifact_kind: "s.stdio.docx".into(), standard: "ecma-376".into(), subset: "*".into() }
        }

        fn fidelity() -> IoFidelityClass {
            IoFidelityClass::Exact
        }

        fn drops() -> &'static [&'static str] {
            &[]
        }

        fn parse_native(asset: &ExampleAsset<'_>) -> Result<Self::Snapshot, String> {
            crate::artifacts::docx::engine::decode_docx(asset.bytes).map_err(|e| e.to_string())
        }

        fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String> {
            crate::artifacts::docx::engine::encode_docx(snapshot).map_err(|e| e.to_string())
        }

        fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String> {
            crate::artifacts::docx::engine::decode_docx(bytes).map_err(|e| e.to_string())
        }

        fn infer(snapshot: &Self::Snapshot) -> Self::Inference {
            DocxInference::infer(snapshot)
        }

        fn sample_mutations(snapshot: &Self::Snapshot) -> Vec<Self::Mutation> {
            vec![DocxMutation::SetSnapshot { snapshot: snapshot.clone() }]
        }

        fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>> {
            crate::artifacts::docx::engine::decode_docx(bytes).map(|_| ()).map_err(|e| vec![e.to_string()])
        }

        fn validate_negative(_bytes: &[u8]) -> Result<Vec<String>, String> {
            Err("SKIP:owning subset has no negative fixture".into())
        }
    }

    #[test]
    fn demo_subset_integrated_roundtrip() {
        let asset = ExampleAsset { bytes: NATIVE_BYTES, text: None, provenance: "✳️any/📚️examples/🎬️demo/🖼️assets/📜️example.docx" };
        test_support::assert_subset_roundtrip::<DocxAnyRoundtrip>(&asset, None);
    }
}
