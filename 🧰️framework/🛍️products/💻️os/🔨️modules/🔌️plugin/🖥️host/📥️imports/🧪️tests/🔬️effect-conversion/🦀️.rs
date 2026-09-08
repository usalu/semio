use super::*;

/// 💡️ Keeps the async host's schema conversion aligned with the typed kernel proposal intent.
#[semio_framework_async_macros::async_test]
async fn request_inference_proposal_preserves_the_closed_kind() {
    let effect = wit_effects::Effect::RequestInferenceProposal(wit_effects::RequestInferenceProposalEffect { kind: wit_effects::InferenceProposalKind::GisMapBoundsRegion });
    let converted = wit_effect_to_kernel(effect).await.expect("proposal intent converts");
    assert!(matches!(converted, semio_framework::kernel::Effect::RequestInferenceProposal { kind: semio_framework::kernel::InferenceProposalKind::GisMapBoundsRegion }));
}
