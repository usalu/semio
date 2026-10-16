//! 🧪️ The proposal port is object-safe and dispatches every byte-level call to the registered implementation.

use super::*;

struct EchoBindingV1;

impl MapInferenceProposalBindingV1 for EchoBindingV1 {
    fn inference_service(&self) -> ArtifactInferenceService {
        unreachable!("the port law never asks the echo binding for a native executable")
    }

    fn commit_members(&self) -> [&'static str; 2] {
        ["echo-drawing", "echo-value"]
    }

    fn composed_children(&self, pack: &[u8]) -> Result<Vec<String>, MapBindingRefusalV1> {
        std::str::from_utf8(pack).map(|text| text.split(',').map(str::to_owned).collect()).map_err(|_| MapBindingRefusalV1::Undecodable)
    }

    fn infer(&self, pack: &[u8], job_id: &str, budget: &WireArtifactInferenceBudget, checkpoint: &mut dyn FnMut(u64) -> Result<(), ArtifactInferenceExecutionError>) -> Result<MapInferenceRunV1, MapBindingRefusalV1> {
        checkpoint(budget.work_units).map_err(|_| MapBindingRefusalV1::Execution)?;
        Ok(MapInferenceRunV1 { result: pack.to_vec(), proposal: job_id.as_bytes().to_vec() })
    }

    fn approval_effect(&self, pack: &[u8], job_id: &str, _budget: &WireArtifactInferenceBudget) -> Result<MapProposalEffectV1, MapBindingRefusalV1> {
        Ok(MapProposalEffectV1 { forward: job_id.as_bytes().to_vec(), inverse: pack.to_vec() })
    }

    fn undo_effect(&self, pack: &[u8], original_inverse_payload: &[u8], original_job_id: &str) -> Result<MapUndoEffectV1, MapBindingRefusalV1> {
        if original_inverse_payload.is_empty() {
            return Err(MapBindingRefusalV1::Arity);
        }
        Ok(MapUndoEffectV1 { original_forward: original_job_id.as_bytes().to_vec(), original_inverse: original_inverse_payload.to_vec(), forward: original_inverse_payload.to_vec(), inverse: pack.to_vec() })
    }

    fn preview(&self, job_id: &str, proposal: &[u8]) -> Result<MapRegionPreviewV1, MapBindingRefusalV1> {
        if proposal.len() != 5 {
            return Err(MapBindingRefusalV1::Bounds);
        }
        Ok(MapRegionPreviewV1 { region_id: format!("echo-{job_id}"), ring: [[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0], [0.0, 0.0]] })
    }
}

fn budget() -> WireArtifactInferenceBudget {
    WireArtifactInferenceBudget { allocation_bytes: 1024, work_units: 7, recursion_depth: 4 }
}

#[test]
fn the_proposal_port_dispatches_every_byte_level_call_through_a_registered_object() {
    let binding: Arc<dyn MapInferenceProposalBindingV1> = Arc::new(EchoBindingV1);
    assert_eq!(binding.commit_members(), ["echo-drawing", "echo-value"]);
    assert_eq!(binding.composed_children(b"echo-drawing,echo-value"), Ok(vec!["echo-drawing".to_owned(), "echo-value".to_owned()]));
    assert_eq!(binding.composed_children(&[0xff]), Err(MapBindingRefusalV1::Undecodable));
    let mut observed = Vec::new();
    let run = binding.infer(b"pack", "job", &budget(), &mut |completed| {
        observed.push(completed);
        Ok(())
    });
    assert_eq!(run, Ok(MapInferenceRunV1 { result: b"pack".to_vec(), proposal: b"job".to_vec() }));
    assert_eq!(observed, vec![7]);
    assert_eq!(binding.infer(b"pack", "job", &budget(), &mut |_| Err(ArtifactInferenceExecutionError::new("test.interrupted", "interrupted"))), Err(MapBindingRefusalV1::Execution));
    assert_eq!(binding.approval_effect(b"pack", "job", &budget()), Ok(MapProposalEffectV1 { forward: b"job".to_vec(), inverse: b"pack".to_vec() }));
    assert_eq!(binding.undo_effect(b"pack", b"", "job"), Err(MapBindingRefusalV1::Arity));
    assert_eq!(binding.undo_effect(b"pack", b"inverse", "job").map(|effect| effect.forward), Ok(b"inverse".to_vec()));
    assert_eq!(binding.preview("job", b"12345").map(|preview| preview.region_id), Ok("echo-job".to_owned()));
    assert_eq!(binding.preview("job", b"1234"), Err(MapBindingRefusalV1::Bounds));
}
