//! 🟰️ Convergence early exit and applied mutation: the convergence early exit of the live fold, the applied mutation as history editing addresses it, the transition retraction closure and the document backbone batches. See [`super`].

use super::*;

//#region 🔖️EphemeralLanes
//#region 🔖️Backbone
//#region 🔖️Backbone
//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
impl<P, Mutation> StoreLive<P, Mutation>
where
    P: Clone + PartialEq + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    /// 🟰️ Lets every Report replay of this store stop where its state meets the live projection's state after the same
    /// prefix (`Arc::ptr_eq`, then `PartialEq`), splicing the unchanged tail instead of replaying it.
    pub fn enable_convergence_early_exit(&mut self) {
        let equal: fn(&P, &P) -> bool = |left, right| left == right;
        self.snapshot_equality = Some(equal);
    }
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
/// 🧾️ One applied operation as history editing addresses it. `effective` describes the operation that folds — the
/// replacement's leaf (semantic kind, display name) when a valid supersession replaces the recorded input, which the
/// recorded `meta` never restamps.
pub struct AppliedMutation<'a, Mutation> {
    pub mutation_id: MutationId,
    pub edit_id: &'a str,
    pub position: usize,
    pub op_index: u32,
    pub operation: &'a Mutation,
    pub meta: Option<&'a MutationMeta>,
    pub transaction: Option<&'a protocol::TransactionRef>,
    pub supersession: Option<&'a protocol::EffectiveSupersession>,
    pub effective: &'static semio_framework_os_spr::MutationLeafDescriptor,
    /// 🫷️ The cross-artifact unit the operation belongs to, `None` when it is its own unit ([`ArtifactStore::unit_id`]): an
    /// operation of a unit takes no supersession in one document alone.
    pub unit: Option<String>,
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
/// 🧊️ Cold-retires a scratch edit copy — a decoded arrival that history already holds, a replay
/// lookup clone, a conflict listing — through its operations' [`Mutation::retire_cold`].
/// 🕸️ Every transition among `transitions` that `refused` names or that causally depends — through `dependencies`,
/// transitively — on one that does, as ids in log order: what one refusal retracts. One law for a store's event log and a
/// persisted `.spr` log ([`retract_history_transitions_from_spr`]), so both retract exactly the same transitions.
pub fn transition_retraction_closure<'a, T>(transitions: &'a [T], identity: impl Fn(&'a T) -> &'a str, dependencies: impl Fn(&'a T) -> Vec<&'a str>, refused: &[String]) -> Vec<String> {
    let mut retracted: HashSet<&'a str> = transitions.iter().map(&identity).filter(|id| refused.iter().any(|name| name == id)).collect();
    loop {
        let before = retracted.len();
        for transition in transitions {
            if dependencies(transition).into_iter().any(|dependency| retracted.contains(dependency)) {
                retracted.insert(identity(transition));
            }
        }
        if retracted.len() == before {
            break;
        }
    }
    transitions.iter().map(&identity).filter(|id| retracted.contains(id)).map(str::to_string).collect()
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️ArtifactStore
/// 📦️ `envelopes`, in authoring order, as the fewest document-backbone batches its declaration admits
/// (`crate::os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_*`): a committed tool transaction of many operations leaves as several
/// batches the hub takes one by one, never as one batch it refuses as undeclared.
pub fn document_backbone_batches(envelopes: Vec<semio_framework_os_spr::MutationEnvelope>) -> Vec<Vec<semio_framework_os_spr::MutationEnvelope>> {
    const FRAMING_BYTES: usize = 16;
    let mut batches = Vec::new();
    let mut batch: Vec<semio_framework_os_spr::MutationEnvelope> = Vec::new();
    let (mut bytes, mut dependencies, mut segments) = (FRAMING_BYTES, 0usize, 0usize);
    for envelope in envelopes {
        let mut encoded = Vec::new();
        semio_framework_os_spr::encode_envelope(&envelope, &mut encoded);
        let full = bytes + encoded.len() > semio_framework_os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES
            || batch.len() == semio_framework_os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_ENVELOPES
            || dependencies + envelope.dependencies.len() > semio_framework_os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_DEPENDENCIES
            || segments + envelope.target.len() > semio_framework_os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_TARGET_SEGMENTS;
        if full && !batch.is_empty() {
            batches.push(std::mem::take(&mut batch));
            (bytes, dependencies, segments) = (FRAMING_BYTES, 0, 0);
        }
        bytes += encoded.len();
        dependencies += envelope.dependencies.len();
        segments += envelope.target.len();
        batch.push(envelope);
    }
    if !batch.is_empty() {
        batches.push(batch);
    }
    batches
}
//#endregion 🔖️ArtifactStore
//#endregion 🔖️ArtifactStore
//#endregion 🔖️History
//#endregion 🔖️TextFormat
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority
//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
