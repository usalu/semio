//! 📝️ Exact store owners for the zero-payload draft lane.

use crate::app::{bounded_document_store_disposer, bounded_document_store_owners, ArtifactOwnedDisposer, NoDraft, NoDraftMutation};
use crate::store;

pub fn no_draft_store_owners() -> store::DocumentStoreOwners<NoDraft, NoDraftMutation> {
    bounded_document_store_owners::<NoDraft, NoDraftMutation>()
}

pub fn no_draft_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::DraftStore<NoDraft, NoDraftMutation>>> {
    bounded_document_store_disposer::<NoDraft, NoDraftMutation>()
}
