//! ⚖️ Real-store authoring laws: an app's own stores, catalogs and canonical operation wires author, undo and redo one mutation.
//!
//! The laws mount the exact catalog an app supplies (`build_*_store_owners`) on a store minted the way the runtime mints it, so a mutation
//! vocabulary that cannot author through its real store fails here instead of at the first undo, genesis or interaction in the browser.
use super::ArtifactApp;
use semio_framework_value::retained_clone::{RetainedCloneBirthDemand, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};
use store::{ArtifactCommand, ArtifactStore, SpaceMember};

/// ⚖️ What one mutation published through a real store: its authored operation rows and the snapshot after apply, undo and redo.
pub struct StoreAuthoringLaw<P> {
    pub forward_rows: Vec<Vec<u8>>,
    pub inverse_rows: Vec<Vec<u8>>,
    pub applied: P,
    pub undone: P,
    pub redone: P,
}

/// 🧹️ Drives one store's owned close to its terminal-empty witness under independently quoted per-axis grants.
fn close_store<P, M>(store: &mut ArtifactStore<P, M>)
where
    P: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + store::ArtifactPack + Send + Sync + 'static,
    M: Clone + semio_framework_value::ToValue + semio_framework_value::FromValue + protocol::Mutation<P> + protocol::OpBinary + protocol::OpText + Send + 'static,
{
    if store.close_owned_terminal_is_empty() { return; }
    if store.close_owned_phase_witness().contains("started=false") { drop(store.detach_backbone().expect("a closing law store releases its backbone handle")); }
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 };
    let mut stalled = 0;
    for _ in 0..1 << 24 {
        if store.close_owned_terminal_is_empty() { return; }
        let demand = store.close_owned_demands(grant.maximum_copy_bytes).expect("current independent store close frontier");
        let turn = RetainedCloneGrant { maximum_depth: demand.depth.max(grant.maximum_depth), ..grant };
        let step = store.close_owned_step(turn).expect("a law store closes under its exact owner grant");
        assert!(step.progress().fits(turn));
        stalled = if step.progress() == RetainedCloneProgress::default() { stalled + 1 } else { 0 };
        assert!(stalled < 4096 || std::thread::panicking(), "law store close stalled on a fully funded turn");
        if matches!(step, RetainedCloneStep::Complete(_)) { assert!(store.close_owned_terminal_is_empty()); return; }
    }
    panic!("law store did not reach its exact terminal-empty witness");
}

/// 📄️ Mounts `A`'s real document catalog and runtime wire on a fresh document store, then applies, undoes and redoes `mutation` through it.
pub async fn author_through_document_store<A: ArtifactApp>(mutation: A::Mutation) -> StoreAuthoringLaw<A::Snapshot>
where
    A::Snapshot: Clone,
{
    let demand = A::document_store_owners_source_demands().expect("the app quotes its document catalog");
    let owners = store::fund_document_store_owners(RetainedCloneBirthDemand { capacity_bytes: demand.capacity_bytes, depth: demand.depth.max(1) }, |grant| A::build_document_store_owners(grant).expect("the app supplies a document catalog"));
    let envelope = store::create_document_envelope::<A::Snapshot, A::Mutation>(A::DOCUMENT_SCHEMA, "authoring-law-document", A::initial_snapshot().await, None);
    let mut document = Box::pin(ArtifactStore::new(envelope, protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into()))).await.expect("the law document store");
    document.install_operation_wire(store::ArtifactOperationWire::semantic::<A::Snapshot>());
    store::install_unscheduled_catalog(&mut document, owners).expect("the app's real document catalog installs");
    let law = crate::with_authoring_identity!(|identity| {
        Box::pin(document.dispatch(ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }, &mut identity)).await.expect("the mutation authors through the real document store");
        let applied = document.snapshot_ref().clone();
        let (forward_rows, inverse_rows) = document.authored_edit_operation_bytes(&mut identity).expect("the applied edit rows author").expect("the applied edit has rows");
        Box::pin(document.dispatch(ArtifactCommand::Undo, &mut identity)).await.expect("undo authors through the real document store");
        let undone = document.snapshot_ref().clone();
        Box::pin(document.dispatch(ArtifactCommand::Redo, &mut identity)).await.expect("redo authors through the real document store");
        let redone = document.snapshot_ref().clone();
        drop(identity.pause().expect("law identity receipt"));
        StoreAuthoringLaw { forward_rows, inverse_rows, applied, undone, redone }
    });
    close_store(&mut document);
    law
}

/// 🎚️ Mounts `A`'s real config catalog and runtime wire on a fresh config store, then applies, undoes and redoes `mutation` through it.
pub async fn author_through_config_store<A: ArtifactApp>(mutation: A::ConfigMutation) -> StoreAuthoringLaw<A::Config>
where
    A::Config: Clone,
{
    let demand = A::config_store_owners_source_demands().expect("the app quotes its config catalog");
    let owners = store::fund_document_store_owners(RetainedCloneBirthDemand { capacity_bytes: demand.capacity_bytes, depth: demand.depth.max(1) }, |grant| A::build_config_store_owners(grant).expect("the app supplies a config catalog"));
    let envelope = store::create_config_envelope::<A::Config, A::ConfigMutation>(A::config_schema().await, "authoring-law-config", A::initial_config().await, None).await;
    let mut config = Box::pin(store::ConfigStore::new(envelope, protocol::ActorId(crate::app::LOCAL_ACTOR_ID.into()))).await.expect("the law config store");
    store::install_unscheduled_catalog(&mut config, owners).expect("the app's real config catalog installs");
    let law = crate::with_authoring_identity!(|identity| {
        Box::pin(config.dispatch(ArtifactCommand::Apply { mutations: vec![mutation], transaction: None }, &mut identity)).await.expect("the mutation authors through the real config store");
        let applied = config.snapshot_ref().clone();
        let (forward_rows, inverse_rows) = config.authored_edit_operation_bytes(&mut identity).expect("the applied edit rows author").expect("the applied edit has rows");
        Box::pin(config.dispatch(ArtifactCommand::Undo, &mut identity)).await.expect("undo authors through the real config store");
        let undone = config.snapshot_ref().clone();
        Box::pin(config.dispatch(ArtifactCommand::Redo, &mut identity)).await.expect("redo authors through the real config store");
        let redone = config.snapshot_ref().clone();
        drop(identity.pause().expect("law identity receipt"));
        StoreAuthoringLaw { forward_rows, inverse_rows, applied, undone, redone }
    });
    close_store(&mut config);
    law
}
