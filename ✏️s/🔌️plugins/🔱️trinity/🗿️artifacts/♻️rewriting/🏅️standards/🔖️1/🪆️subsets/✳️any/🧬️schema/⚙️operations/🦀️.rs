//! ⚙️ Rewriting mutation store, application, derivation, and behavior laws.

use crate::standards::v1::subsets::any::schema::mutations::RewriteRuleMutation;
use crate::{RewritingSnapshot, TrinityRewritingError, REWRITE_RULE_SCHEMA};
use store::{create_document_envelope, ArtifactCommand, ArtifactEnvelope, ArtifactStore};

//#region 🔖️Store
pub type RewriteRuleEnvelope = ArtifactEnvelope<RewritingSnapshot, RewriteRuleMutation>;
pub type RewriteRuleStore = ArtifactStore<RewritingSnapshot, RewriteRuleMutation>;

pub fn create_rewrite_rule_envelope(id: &str, state: RewritingSnapshot) -> RewriteRuleEnvelope {
    create_document_envelope(REWRITE_RULE_SCHEMA, id, state, None)
}

/// 🔐️ Opens a rewriting store WITH its exact owner catalog installed. `ArtifactStore::new` installs no
/// catalog, and `reserve_edit_history_slot` refuses every `Apply` without one (`edit history
/// insertion requires its exact mutation retirement factory`); the editor app installs the same
/// catalog through `build_document_store_owners`, every standalone store goes through here.
pub async fn new_rewrite_rule_store(envelope: RewriteRuleEnvelope, actor: protocol::ActorId) -> Result<OwnedRewriteRuleStore, store::VcsError> {
    let mut store = RewriteRuleStore::new(envelope, actor).await?;
    store.install_document_store_owners_exact(crate::standards::v1::subsets::any::schema::retirement::document_store_owners());
    Ok(OwnedRewriteRuleStore(store))
}

/// 🔚 A standalone rewriting store that retires itself: `ArtifactStore::drop` panics `artifact store
/// reached Drop without its exact terminal-empty shallow-shell witness` unless the store walked its
/// bounded close loop first, so the guard runs that loop on drop (skipped while unwinding).
pub struct OwnedRewriteRuleStore(RewriteRuleStore);

impl OwnedRewriteRuleStore {
    /// 🔚 Walks the exact bounded owner close loop to the terminal-empty witness.
    pub fn close(&mut self) {
        while !self.0.close_owned_terminal_is_empty() {
            self.0.close_owned_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("rewriting document store closes through its exact bounded owners");
        }
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
pub fn apply_rewrite_rule_mutation(snapshot: &mut RewritingSnapshot, mutation: &RewriteRuleMutation) -> protocol::MutationApplyResult<()> {
    let outcome = protocol::Mutation::diff(mutation, snapshot);
    let next = protocol::MutationDiff::apply(outcome.diff(), snapshot)?;
    *snapshot = next;
    Ok(())
}

pub fn inverse_rewrite_rule_mutation(snapshot: &RewritingSnapshot, mutation: &RewriteRuleMutation) -> Result<Vec<RewriteRuleMutation>, semio_framework_value::ValueError> {
    Ok({
    protocol::Mutation::inverse(mutation, snapshot)?

    })
}

/// ▶️ Dispatches a batch of granular mutations as one VCS edit.
pub async fn dispatch_rewrite_rule_mutations(store: &mut RewriteRuleStore, mutations: Vec<RewriteRuleMutation>) -> Result<(), TrinityRewritingError> {
    if mutations.is_empty() {
        return Ok(());
    }
    store.dispatch(ArtifactCommand::Apply { mutations, transaction: None }).await.map_err(TrinityRewritingError::from).map(|_| ())
}
//#endregion 🔖️BatchHelpers

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[cfg(test)]
use crate::standards::v1::subsets::any::schema::mutations::register_rewrite_rule_mutation_descriptors;
