use super::*;
use crate::schema::mutations::set_snapshot;
use crate::standards::v_ecma_376::subsets::base::schema::inferences::DocxInference;
use crate::{DocxMutation, DocxSnapshot};
use protocol::Inference;

use store::os_store::test_support::{self, ExampleAsset, IoFidelityClass, SubsetRoundtripSpec};

#[semio_framework_async_macros::async_test]
async fn demo_source_nonempty() {
    assert!(!PRIMARY_TEXT.is_empty());
    let _ = source();
}

struct DocxAnyRoundtrip;

impl SubsetRoundtripSpec for DocxAnyRoundtrip {
    type Snapshot = DocxSnapshot;
    type Mutation = DocxMutation;
    type Inference = DocxInference;

    async fn dialect() -> store::os_io::ArtifactDialect {
        store::os_io::ArtifactDialect { artifact_kind: "s.stdio.docx".into(), standard: "ecma-376".into(), subset: "*".into() }
    }

    async fn fidelity() -> IoFidelityClass {
        IoFidelityClass::Exact
    }

    async fn drops() -> &'static [&'static str] {
        &[]
    }

    async fn parse_native(asset: &ExampleAsset<'_>) -> Result<Self::Snapshot, String> {
        crate::engine::decode_docx(asset.bytes).map_err(|e| e.to_string())
    }

    async fn export_native(snapshot: &Self::Snapshot) -> Result<Vec<u8>, String> {
        crate::engine::encode_docx(snapshot).map_err(|e| e.to_string())
    }

    async fn reimport_native(bytes: &[u8]) -> Result<Self::Snapshot, String> {
        crate::engine::decode_docx(bytes).map_err(|e| e.to_string())
    }

    async fn infer(snapshot: &Self::Snapshot) -> Self::Inference {
        DocxInference::infer(snapshot)
    }

    async fn sample_mutations(snapshot: &Self::Snapshot) -> Vec<Self::Mutation> {
        vec![DocxMutation::SetSnapshot(set_snapshot::SetSnapshot { snapshot: snapshot.clone() })]
    }

    async fn validate_payload(bytes: &[u8]) -> Result<(), Vec<String>> {
        crate::engine::decode_docx(bytes).map(|_| ()).map_err(|e| vec![e.to_string()])
    }

    async fn validate_negative(_bytes: &[u8]) -> Result<Vec<String>, String> {
        Err("SKIP:owning subset has no negative fixture".into())
    }
}

#[semio_framework_async_macros::async_test]
async fn demo_subset_integrated_roundtrip() {
    let asset = ExampleAsset { bytes: NATIVE_BYTES, text: None, provenance: "✳️any/📚️examples/🎬️demo/🖼️assets/📜️example.docx" };
    test_support::assert_subset_roundtrip::<DocxAnyRoundtrip>(&asset, None).await;
}
