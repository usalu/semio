//! ⚙️ Rewriting mutation store, application, derivation, and behavior laws.

use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
use crate::standards::v1::subsets::any::schema::errors::TrinityRewritingError;
use crate::standards::v1::subsets::any::schema::model::REWRITE_RULE_SCHEMA;
use store::{os_store::document_materialize::create_document_envelope, os_store::store_schema::ArtifactCommand, os_store::store_schema::ArtifactEnvelope, ArtifactStore};

//#region 🔖️Store
pub type RewriteRuleEnvelope = ArtifactEnvelope<RewritingSnapshot, RewriteRuleMutation>;
pub type RewriteRuleStore = ArtifactStore<RewritingSnapshot, RewriteRuleMutation>;

pub fn create_rewrite_rule_envelope(id: &str, state: RewritingSnapshot) -> RewriteRuleEnvelope {
    create_document_envelope(REWRITE_RULE_SCHEMA, id, state, None, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer)
}

/// 🔐️ Opens a rewriting store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`); the editor app installs the same
/// catalog through `build_document_store_owners`, every standalone store goes through here.
pub async fn new_rewrite_rule_store(envelope: RewriteRuleEnvelope, actor: protocol::ActorId) -> Result<OwnedRewriteRuleStore, store::VcsError> {
    let mut store = RewriteRuleStore::new::<::semio_framework_os_kernel::os_store::ArtifactSnapshotPublication>(envelope, actor).await?;
    store::install_unscheduled_catalog(&mut store, store::os_store::document_store_owners::funded_bounded_artifact_store_owners::<RewritingSnapshot, RewriteRuleMutation>())?;
    Ok(OwnedRewriteRuleStore(store))
}

/// 🔚 A standalone rewriting store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so the guard runs that loop on drop (skipped while unwinding).
pub struct OwnedRewriteRuleStore(RewriteRuleStore);

impl OwnedRewriteRuleStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        self.0.close_owned_unscheduled().expect("rewriting document store closes through its exact bounded owners");
    }
}

impl std::ops::Deref for OwnedRewriteRuleStore {
    type Target = RewriteRuleStore;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for OwnedRewriteRuleStore {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Drop for OwnedRewriteRuleStore {
    fn drop(&mut self) {
        if !std::thread::panicking() {
            self.close();
        }
    }
}
//#endregion 🔖️Store

//#region 🔖️BatchHelpers
/// ▶️ Applies one mutation to a snapshot through the central applier (`protocol::apply_diff`): the step of the editor's node-graph fold and of the mutation tests.
pub fn apply_rewrite_rule_mutation(snapshot: &mut RewritingSnapshot, mutation: &RewriteRuleMutation) -> protocol::MutationApplyResult<()> {
    let outcome = protocol::Mutation::diff(mutation, snapshot);
    let next = protocol::apply_diff(outcome.diff(), snapshot)?;
    *snapshot = next;
    Ok(())
}

pub fn inverse_rewrite_rule_mutation(snapshot: &RewritingSnapshot, mutation: &RewriteRuleMutation) -> Result<Vec<RewriteRuleMutation>, semio_framework_value::ValueError> {
    Ok({
    protocol::Mutation::inverse(mutation, snapshot)?

    })
}

/// ▶️ Dispatches a batch of granular mutations as one VCS edit.
pub async fn dispatch_rewrite_rule_mutations(store: &mut RewriteRuleStore, mutations: Vec<RewriteRuleMutation>, identity: &mut ::store::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<(), TrinityRewritingError> {
    if mutations.is_empty() {
        return Ok(());
    }
    store.dispatch(ArtifactCommand::Apply { mutations, transaction: None }, identity).await.map_err(TrinityRewritingError::from).map(|_| ())
}
//#endregion 🔖️BatchHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
use crate::standards::v1::subsets::any::schema::mutations::register_rewrite_rule_mutation_descriptors;
