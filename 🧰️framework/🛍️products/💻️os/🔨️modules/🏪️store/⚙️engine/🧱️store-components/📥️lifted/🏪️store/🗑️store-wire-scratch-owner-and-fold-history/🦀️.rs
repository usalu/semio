//! 🗑️ Store wire, scratch owner and fold history: the durable group root reads of the store wire, the scratch owner with its deref and drop, and the history fold. See [`super`].

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
impl<P, Mutation> StoreWire<P, Mutation>
where
    P: ToValue + Send + Sync + 'static,
    Mutation: ToValue + FromValue + super::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    pub fn durable_group_read_root(&self) -> Option<&semio_framework_os_store_durable_group_wire::lifted_durable_group::ArtifactStoreDurableGroupRootV1<P, Mutation>> {
        self.durable_group_root.as_ref().filter(|root| root.visibility.committed() && !root.adopted)
    }

    pub fn ensure_durable_group_idle(&self) -> Result<(), VcsError> {
        if self.durable_group_root.is_some() {
            return Err(VcsError::ValidationFailed("artifact store belongs to an unresolved durable group decision".into()));
        }
        Ok(())
    }

    /// 🧬️ Returns the current history identity used to invalidate derived work.
    pub fn artifact_revision(&self, store_log: &StoreLog<P, Mutation>) -> ArtifactRevision {
        let group = self.durable_group_read_root();
        ArtifactRevision {
            artifact_id: store_log.envelope.id.clone(),
            schema: store_log.envelope.schema.clone(),
            applied_edit_ids: group.map_or_else(|| (*store_log.applied_edit_ids).clone(), |root| root.applied_edit_ids.clone()),
            redo_edit_ids: group.map_or_else(|| (*store_log.redo_edit_ids).clone(), |root| root.redo_edit_ids.clone()),
            checkpoint_id: (*store_log.current_checkpoint_id).clone(),
        }
    }

    pub fn applied_edit_ids<'a>(&'a self, store_log: &'a StoreLog<P, Mutation>) -> &'a semio_framework_os_vcs::HistoryPageStack<String> {
        self.durable_group_read_root().map_or(&store_log.applied_edit_ids, |root| &root.applied_edit_ids)
    }

    /// ↪️ Pending redo stack (edit ids undone since the last fresh `Apply`).
    pub fn redo_edit_ids<'a>(&'a self, store_log: &'a StoreLog<P, Mutation>) -> &'a semio_framework_os_vcs::HistoryPageStack<String> {
        self.durable_group_read_root().map_or(&store_log.redo_edit_ids, |root| &root.redo_edit_ids)
    }

    pub fn replace_backbone_retained(&mut self, store_owners: &mut StoreOwners<P, Mutation>, next: Option<Backbones>) -> Result<(), VcsError> {
        store_owners.displaced_retirements.reserve(usize::from(self.backbone.is_some()))?;
        if let Some(previous) = std::mem::replace(&mut *self.backbone, next) {
            store_owners.displaced_retirements.push_reserved(Box::new(ArtifactStoreBackboneRetirement::new(previous)));
        }
        Ok(())
    }

    /// 🔭️ Why a close that refuses in its displaced-owner phase cannot finish: the queue census
    /// plus the staged-group root that holds its reservation open.
    pub fn close_displaced_witness(&self, store_owners: &StoreOwners<P, Mutation>) -> String {
        let group = self.durable_group_root.as_ref().map_or("none", |root| if root.adopted { "adopted" } else { "staged" });
        format!("{}/group={group}", store_owners.displaced_retirements.witness())
    }

    pub fn close_take_backbone_retirement(&mut self, grant: RetainedCloneGrant) -> Result<(Option<Box<dyn ErasedSnapshotRetirement>>, RetainedCloneProgress), ValueError> {
        if !self.original_outbox.terminal_is_empty(){return ArtifactStoreBackboneRetirement::admit_original_outbox(&mut self.original_outbox,grant)}
        ArtifactStoreBackboneRetirement::admit_attached(&mut self.member_inbox, &mut self.backbone, grant)
    }

    /// 📮️ Announces one composed member's events on THIS store's backbone under that member's
    /// exact lane identity. A composed document owns one replica endpoint — the parent's — so without
    /// this a child lane's edits never leave the process and two replicas diverge on every child edit.
    pub async fn send_member_mutations(&mut self, store_owners: &mut StoreOwners<P, Mutation>, owner: &str, slot: &str, child_id: &str, envelopes: Vec<u8>) -> Result<(), VcsError> {
        if envelopes.is_empty() {
            return Ok(());
        }
        let Some(mut backbone) = self.backbone.take() else { return Ok(()) };
        let result = backbone.send(BackboneMessage::Member { owner: owner.to_string(), slot: slot.to_string(), child_id: child_id.to_string(), envelopes }).await;
        self.replace_backbone_retained(store_owners, Some(backbone))?;
        result
    }

    /// 📬️ Drains every member-addressed message this store pumped off its transport, as
    /// `(owner, slot, child_id, envelopes)` — the composing app resolves the lane and folds it.
    pub fn take_member_inbound(&mut self) -> Vec<(String, String, String, Vec<u8>)> {
        self.member_inbox
            .drain(..)
            .filter_map(|message| match message {
                BackboneMessage::Member { owner, slot, child_id, envelopes } => Some((owner, slot, child_id, envelopes)),
                _ => None,
            })
            .collect()
    }

    /// 📣️ Sends `Genesis` (the initial snapshot pack, identifying the document) followed by the
    /// whole event log in `Mutations` batches no larger than one document-backbone batch.
    pub async fn announce_history(&mut self, store_log: &mut StoreLog<P, Mutation>, store_owners: &mut StoreOwners<P, Mutation>) -> Result<(), VcsError> {
        if self.backbone.is_none() {
            return Ok(());
        }
        let genesis = BackboneMessage::Genesis { pack: store_log.envelope.vcs.genesis.pack().to_vec() };
        let mut messages = vec![genesis];
        let mut batch: Vec<semio_framework_os_spr::MutationEnvelope> = Vec::new();
        let mut batch_bytes = 0usize;
        for event in store_log.event_log()? {
            let mut encoded = Vec::new();
            semio_framework_os_spr::encode_envelope(&event, &mut encoded);
            if !batch.is_empty() && batch_bytes + encoded.len() > semio_framework_os_spr::DOCUMENT_BACKBONE_BATCH_MAXIMUM_BYTES / 2 {
                messages.push(BackboneMessage::from_envelopes(&std::mem::take(&mut batch)));
                batch_bytes = 0;
            }
            batch_bytes += encoded.len();
            batch.push(event);
        }
        if !batch.is_empty() {
            messages.push(BackboneMessage::from_envelopes(&batch));
        }
        let Some(mut backbone) = self.backbone.take() else { return Ok(()) };
        let mut result = Ok(());
        for message in messages {
            result = backbone.send(message).await;
            if result.is_err() {
                break;
            }
        }
        self.replace_backbone_retained(store_owners, Some(backbone))?;
        result
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
/// 🩹️ A scratch owner — a working projection, the operations of a command, an edit not yet in the ledger — that leaves
/// through its technology's cold retirement on EVERY exit path until [`Self::adopt`] hands it to the store. An artifact whose
/// projection or operation owns a fail-closed root aborts the guest on a bare drop, so a command the store refuses (a rejected
/// apply, a refused inverse, an encode failure, a failing diff, a saturated ledger) must never let what it built reach `Drop`.
/// Holding it here makes that structural: a new early return cannot miss it.
pub struct ScratchOwner<T> {
    pub owner: Option<T>,
    pub retire: fn(T),
}

impl<T> ScratchOwner<T> {
    pub fn new(owner: T, retire: fn(T)) -> Self {
        Self { owner: Some(owner), retire }
    }

    /// 🤝️ Hands the owner to the store, which owns its retirement from here on.
    pub fn adopt(mut self) -> T {
        self.owner.take().expect("a scratch owner is adopted exactly once")
    }
}

impl<T> std::ops::Deref for ScratchOwner<T> {
    type Target = T;
    fn deref(&self) -> &T {
        self.owner.as_ref().expect("a scratch owner is present until it is adopted")
    }
}

impl<T> std::ops::DerefMut for ScratchOwner<T> {
    fn deref_mut(&mut self) -> &mut T {
        self.owner.as_mut().expect("a scratch owner is present until it is adopted")
    }
}

impl<T> Drop for ScratchOwner<T> {
    fn drop(&mut self) {
        if let Some(owner) = self.owner.take() {
            (self.retire)(owner);
        }
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
/// 🧮️ Folds the genesis over the effective forwards of `applied_edit_ids` ([`fold_effective_edit`]).
pub async fn fold_history<P, Mutation, I, S>(envelope: &ArtifactEnvelope<P, Mutation>, applied_edit_ids: I, supersessions: &EffectiveSupersessions) -> Result<P, VcsError>
where
    P: Clone,
    Mutation: super::Mutation<P> + OpBinary+protocol::ColdMutation<P>,
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut snapshot = ColdReplayProjection::<P, Mutation>::new(envelope.vcs.genesis.facts().snapshot().clone());
    let mut seen = HashSet::new();
    for edit_id in applied_edit_ids {
        let edit_id = edit_id.as_ref();
        if !seen.insert(edit_id.to_string()) {
            return Err(VcsError::ValidationFailed(format!("history repeats applied edit {edit_id}")));
        }
        let edit = envelope.vcs.edits.find_near(|entry| entry.id == edit_id).ok_or_else(|| VcsError::UnknownEdit(edit_id.to_string()))?;
        fold_effective_edit(&mut snapshot, edit, &envelope.schema, supersessions, usize::MAX);
    }
    Ok(snapshot.into_inner())
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
