//! 📥️ Items of `store` that belong to `os-store-space` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_os_vcs::io::binary::genesis::{AdmittedArtifactGenesis, ArtifactGenesisProducer};
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::{ActorId, HybridLogicalTimestamp};
use semio_framework_os_spr::{Mutation, OpBinary, OpText};
use serde::Deserialize;
use std::collections::HashMap;
use semio_framework_os_store_engine_owners::{DocumentStoreOwners, funded_member_store_owners};
use semio_framework_os_store_engine_owners::member_producer::MemberStoreProducer;
use semio_framework_os_store_engine_owners::close_unscheduled_catalog;
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_vcs::io::binary::entity_identity::{mint_space_checkpoint_id, mint_space_alternative_id};
use semio_framework_os_vcs::{Author, VcsError};
use semio_framework_os_store_space_schema::{ArtifactBackboneRef, ArtifactCommand, ArtifactEnvelope};
use semio_framework_os_store_envelope::owner::OwnerRef;
//#region 🔖️EphemeralLanes

use semio_framework_os_store_ephemeral::*;
use semio_framework_os_store_text_io::decode_document_pack_bytes;
use semio_framework_os_store_engine_owners::funded_bounded_artifact_store_owners;
use semio_framework_os_store_document::create_document_envelope_from_genesis;
use semio_framework_os_store_initialization_runtime::lifted_document_materialize::now_ms;
use semio_framework_os_store_text_io::text_format::parse_decoded_document_spr;
use semio_framework_os_store_initialization_runtime::native_publication_producer::{ArtifactStorePublication, ArtifactSnapshotPublication};
//#region 🔖️Backbone
//#region 🔖️Backbone
use semio_framework_os_store_backbone as backbone;
use backbone::Backbones;
use semio_framework_os_store_space_schema::space_history_document::{SpaceAlternative, SpaceCheckpoint, SpaceHistorySnapshot, SpaceMemberPin};
use semio_framework_os_store_space_schema::space_history_mutations::{CommitSpaceCheckpoint, CreateSpaceAlternative, SpaceHistoryMutation, SwitchSpaceAlternative};
use semio_framework_os_store_space_member::lifted_store::MemberDirectory;
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
use semio_framework_os_store_space_member::lifted_store::SpaceMember;
use semio_framework_os_store_space_member::lifted_store::NoMembers;
//#region 🔖️Composition
impl MemberDirectory for SpaceHost {
    async fn head_pack(&self, artifact_id: &str) -> Option<Result<Vec<u8>, VcsError>> {
        match self.member(artifact_id).await {
            Some(member) => Some(member.document_pack_bytes().await),
            None => None,
        }
    }

    async fn checkpoint_pack(&self, artifact_id: &str, checkpoint_id: &str) -> Option<Result<Vec<u8>, VcsError>> {
        match self.member(artifact_id).await {
            Some(member) => Some(member.pack_at_checkpoint(checkpoint_id).await),
            None => None,
        }
    }
}
//#endregion 🔖️Composition

//#region 🔖️Composition
/// 🏭️ Generic genesis-construction helper every `space_members!`-generated `MemberFactory::
/// create` arm calls — the composition sibling of `ArtifactCodec`/`register_document_codec` above.
/// Bakes a brand-new child store from a freshly-baked initial pack (composition genesis, see
/// `CompositionCoordinator::dispatch_group`): `initial_pack` decoded as `P` (empty rejected — a
/// genesis caller always has an initial snapshot, and silently substituting `P::default()` would
/// mint a child whose content nobody chose), stamped with `dialect`. Replaces the old
/// `ChildStoreFactory`/`TypedChildStoreFactory` object-erased pair (O1 — a global `Arc<dyn
/// ChildStoreFactory>` registry keyed by runtime kind string is exactly the dyn-dispatched seam the
/// program drops): the registry's kind-keying now lives in the generated enum's own `match kind`.
pub async fn create_member_store<P, Mutation, G, C>(schema: &str, id: &str, dialect: &semio_framework_artifact_reference::ArtifactDialect, initial_pack: &[u8], actor: ActorId, producer:&G,catalog:&C) -> Result<ArtifactStore<P, Mutation>, VcsError>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
    G: ArtifactGenesisProducer<P> + ArtifactStorePublication<P> + Sync,
    C:MemberStoreProducer<P,Mutation>,
{
    if initial_pack.is_empty() {
        return Err(VcsError::Deserialize(format!("child genesis for {id} carries an empty initial pack")));
    }
    let initial = producer.decode_genesis_pack(initial_pack).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let mut envelope = create_document_envelope_from_genesis::<P, Mutation>(schema, id, AdmittedArtifactGenesis::from_decoded_pack(initial, initial_pack.to_vec()), None);
    envelope.dialect = Some(dialect.clone());
    let mut store = ArtifactStore::new::<G>(envelope, actor).await?;
    install_funded_member_store_owners(&mut store,catalog)?;
    Ok(store)
}

/// 🏭️ Generic reload helper every `space_members!`-generated `MemberFactory::open` arm
/// calls — reconstructs a previously-persisted child from its full envelope pack via the same
/// `parse_document_pack` → `reset(envelope, applied, redo)` path `apply_ops_binary` uses, so a
/// reloaded child restores its exact undo/redo cursor position, not merely its content. The
/// invariant is `owner.is_some() ⇒ dialect.is_some()` — every envelope that is somebody's child
/// knows which dialect it materializes as, which is what lets `ArtifactView.children` type a child
/// without consulting the parent.
pub async fn open_member_store<P, Mutation, G, C>(schema: &str, expected: &semio_framework_artifact_reference::ArtifactRef, owner: Option<&OwnerRef>, envelope_pack: &[u8], actor: ActorId, producer:&G,catalog:&C) -> Result<ArtifactStore<P, Mutation>, VcsError>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
    G: ArtifactGenesisProducer<P> + ArtifactStorePublication<P> + Sync,
    C:MemberStoreProducer<P,Mutation>,
{
    let (pack, spr) = decode_document_pack_bytes(envelope_pack).await?;
    let history = semio_framework_os_spr::decode_history(&spr, &semio_framework_os_spr::DecodeOptions::default()).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    validate_member_history_identity(&history, schema, expected, owner)?;
    let parsed = parse_decoded_document_spr::<P, Mutation>(&pack, history, producer).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let mut store = ArtifactStore::new::<G>(parsed.envelope, actor).await?;
    install_funded_member_store_owners(&mut store,catalog)?;
    Ok(store)
}

/// 🔐️ Installs a completed catalog and retires it exactly when the store refuses its transfer.
pub fn install_unscheduled_catalog<P, Mutation>(store: &mut ArtifactStore<P, Mutation>, owners: Result<DocumentStoreOwners<P, Mutation>, ValueError>) -> Result<(), VcsError>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
{
    let owners = owners.map_err(|error| VcsError::ValidationFailed(error.into_message()))?;
    store.install_document_store_owners_exact(owners).map_err(|(error, owners)| { close_unscheduled_catalog(owners); VcsError::ValidationFailed(error.into_message()) })
}

/// 🔐️ Installs a self-funded member catalog and leaves the store retaining every original owner on refusal.
pub fn install_funded_member_store_owners<P, Mutation, C>(store: &mut ArtifactStore<P, Mutation>,producer:&C) -> Result<(), VcsError>
where
    P: Clone + ToValue + FromValue + Send + Sync + 'static,
    Mutation: Clone + ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static,
    C:MemberStoreProducer<P,Mutation>,
{
    install_unscheduled_catalog(store, funded_member_store_owners::<P, Mutation,C>(producer))
}

pub fn validate_member_history_identity(history: &semio_framework_os_spr::HistoryLog, schema: &str, expected: &semio_framework_artifact_reference::ArtifactRef, owner: Option<&OwnerRef>) -> Result<(), VcsError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

    let persisted_dialect = history.composition.as_ref().and_then(|composition| composition.dialect.as_ref());
    let dialect_matches = persisted_dialect.is_some_and(|(kind, standard, subset)| kind == &expected.dialect.artifact_kind && standard == &expected.dialect.standard && subset == &expected.dialect.subset);
    let persisted_owner = history.composition.as_ref().and_then(|composition| composition.owner.as_ref());
    let owner_matches = match (persisted_owner, owner) {
        (None, None) => true,
        (Some((parent, slot, child_id)), Some(owner)) => slot == &owner.slot && child_id == &owner.child_id && semio_framework_artifact_reference::ArtifactRef::parse_uri(parent).is_ok_and(|parent| parent == owner.parent),
        _ => false,
    };
    if history.doc_id != expected.artifact_id || history.schema != schema || !dialect_matches || !owner_matches {
        return Err(VcsError::Deserialize("member envelope does not match requested identity, schema, dialect and ownership".into()));
    }
    Ok(())
}
//#endregion 🔖️Composition

//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️TextFormat
//#region 🔖️History
//#region 🔖️ArtifactStore
//#region 🔖️Backbone
//#region 🔖️BlobStore
//#region 🔖️Space
//#region SpaceHistoryDocument

/// 🔐️ The one owner catalog the dogfooded `S_SPACE_HISTORY_SCHEMA` meta document installs.
/// `install_document_store_owners_exact`'s own doc says there is NO default catalog, and every
/// history insertion asks its store for the exact mutation retirement factory, so without this a
/// `SpaceHost` could not record a single space checkpoint — `commit_space_checkpoint` refused with
/// `edit history insertion requires its exact mutation retirement factory` and the meta store could
/// never reach `ArtifactStore::drop`'s terminal-empty witness either.
pub fn space_history_store_owners() -> Result<DocumentStoreOwners<SpaceHistorySnapshot, SpaceHistoryMutation>, ValueError> {
    funded_bounded_artifact_store_owners::<SpaceHistorySnapshot, SpaceHistoryMutation>()
}

/// 🏛️ Composes many `SpaceMember` documents under one space-wide checkpoint/alternative
/// timeline, itself stored in a dogfooded `S_SPACE_HISTORY_SCHEMA` (`"os.space.history"`)
/// meta-document. App-agnostic: this crate has no notion of what a member document *is*, only that
/// it satisfies `SpaceMember`.
pub struct SpaceHost<M = NoMembers> {
    pub meta: ArtifactStore<SpaceHistorySnapshot, SpaceHistoryMutation>,
    pub members: HashMap<String, M>,
}

/// 🚪️ Drains the meta document to `ArtifactStore::drop`'s exact terminal-empty witness
/// before the host's own shell falls. The host OWNS its meta store outright — no caller is handed a
/// handle to it — so there is nobody else who could retire it, and a bounded drain here is the same
/// shape `🌉️mcp`'s `HeadlessWorkspace` uses at process shutdown. Registered MEMBERS are not touched:
/// each one is its own document with its own owner catalog, closed by whoever opened it.
impl<M> Drop for SpaceHost<M> {
    fn drop(&mut self) {
        if std::thread::panicking() {
            return;
        }
        drop(self.meta.detach_backbone().expect("a closing space meta document releases its backbone"));
        self.meta.close_owned_unscheduled().expect("space meta document reaches its exact terminal-empty witness under its quoted owner demands");
    }
}

impl<M: SpaceMember> SpaceHost<M> {
    pub async fn new(meta_envelope: ArtifactEnvelope<SpaceHistorySnapshot, SpaceHistoryMutation>, actor: ActorId) -> Result<Self, VcsError> {
        let mut meta = ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(meta_envelope, actor).await?;
        install_unscheduled_catalog(&mut meta, space_history_store_owners())?;
        Ok(Self { meta, members: HashMap::new() })
    }

    pub async fn register_member(&mut self, member: M) {
        self.members.insert(member.document_id().await.to_string(), member);
    }

    /// 📚️ Batch counterpart to `register_member`: registers a space's manifest document, its
    /// collection documents, and any currently-open artifact documents together in one call, so the
    /// very next `commit_space_checkpoint` pins all of them atomically in the SAME space-wide
    /// checkpoint (see `🪐️space`'s `SpaceSnapshot`/`CollectionSnapshot`/document-artifact
    /// stores, W4's storage wiring — this crate stays app-agnostic and never names those types
    /// directly, only their common `SpaceMember` façade). Purely additive sugar over calling
    /// `register_member` three times in this order; no new mechanism.
    pub async fn register_space_documents(&mut self, manifest: M, collections: Vec<M>, artifacts: Vec<M>) {
        self.register_member(manifest).await;
        for collection in collections {
            self.register_member(collection).await;
        }
        for artifact in artifacts {
            self.register_member(artifact).await;
        }
    }

    pub async fn unregister_member(&mut self, document_id: &str) -> Option<M> {
        self.members.remove(document_id)
    }

    pub async fn member(&self, document_id: &str) -> Option<&M> {
        self.members.get(document_id)
    }

    pub async fn member_mut(&mut self, document_id: &str) -> Option<&mut M> {
        self.members.get_mut(document_id)
    }

    pub async fn meta_snapshot(&self) -> Result<SpaceHistorySnapshot, VcsError> {
        self.meta.snapshot()
    }

    /// 🔗️ Attaches a backbone to the space-wide meta-document, same runtime-attach/detach
    /// contract as any other `ArtifactStore` — default is unattached, this is always an
    /// explicit call.
    pub async fn attach_backbone(&mut self, backbone: Backbones) -> Result<(), VcsError> {
        self.meta.attach_backbone(backbone).await
    }

    /// ✂️ Detaches the meta-document's backbone; the space history stays in memory.
    pub async fn detach_backbone(&mut self) -> Result<Option<Backbones>, VcsError> {
        self.meta.detach_backbone()
    }

    pub async fn backbone_ref(&self) -> Option<&ArtifactBackboneRef> {
        self.meta.backbone_ref()
    }

    /// 📡️ Drains inbound backbone messages into the meta-document's edit timeline.
    pub async fn tick(&mut self) -> Result<bool, VcsError> {
        self.meta.tick().await
    }

    /// 💾️ Commits every dirty member (leaving clean members' existing checkpoints untouched),
    /// pins each member's resulting `(checkpoint, alternative)`, and records one `SpaceCheckpoint`
    /// on the meta-document — applied *and* committed there too, so the space history itself is
    /// durable the moment this returns.
    ///
    /// 📣️ BOTH meta commands go through `dispatch`, so BOTH flush their events. The `Apply`
    /// used to go through `dispatch_inner` on the reasoning that the following `CommitCheckpoint`
    /// flushed a whole SNAPSHOT that already carried it — true until the event-sourced transition
    /// deleted snapshot broadcast (`BackboneMessage::Snapshot` → `Genesis`, identity only). After
    /// it, `CommitCheckpoint` flushes only its own transition events and the space checkpoint's
    /// `Apply` never left the process: a second host attached to the same backbone folded nothing
    /// (`space_vcs_host_meta_document_is_backbone_attachable_and_detachable`, 0 checkpoints instead
    /// of 1). Re-sending is not a risk — `flush_outbound` drains a queue of PENDING events, so an
    /// event announced once is not announced again.
    pub async fn commit_space_checkpoint(&mut self, message: String, authors: Vec<Author>, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        let mut document_ids: Vec<String> = self.members.keys().cloned().collect();
        document_ids.sort();
        let mut pins = Vec::with_capacity(document_ids.len());
        for document_id in &document_ids {
            let member = self.members.get_mut(document_id).expect("just collected from members");
            if member.is_dirty().await {
                member.commit_checkpoint(message.clone(), authors.clone(), identity).await?;
            }
            let checkpoint_id = member.current_checkpoint_id().await.ok_or(VcsError::NoCheckpoint)?;
            pins.push(SpaceMemberPin { document_id: document_id.clone(), checkpoint_id, alternative_id: member.current_alternative_id().await.unwrap_or_default() });
        }
        let pins_fingerprint = identity.encode(|control|semio_framework_pack_json::to_json_string_controlled(&pins,control))?.into_bytes();
        let checkpoint_id = identity.encode(|control|mint_space_checkpoint_id(&message,&pins_fingerprint,control))?;
        let parent_id = self.meta.snapshot()?.checkpoints.last().map(|checkpoint| checkpoint.id.clone());
        let checkpoint = SpaceCheckpoint { id: checkpoint_id.clone(), parent_id, message: message.clone(), authors, timestamp: HybridLogicalTimestamp::new(0, now_ms()), members: pins };
        self.meta.dispatch(ArtifactCommand::Apply { mutations: vec![SpaceHistoryMutation::CommitSpaceCheckpoint(CommitSpaceCheckpoint { checkpoint })], transaction: None }, identity).await?;
        self.meta.dispatch(ArtifactCommand::CommitCheckpoint { message: None, authors: Vec::new() }, identity).await?;
        Ok(checkpoint_id)
    }

    /// 🌿️ Records a `SpaceAlternative` pinned at the current space checkpoint tip (or none,
    /// if nothing has been committed yet), so it can later be switched back into.
    pub async fn create_space_alternative(&mut self, name: String, identity: &mut EntityIdentityAuthority<'_>) -> Result<String, VcsError> {
        let checkpoint_ids: Vec<String> = self.meta.snapshot()?.checkpoints.last().map(|checkpoint| checkpoint.id.clone()).into_iter().collect();
        let alternative_id = identity.encode(|control|mint_space_alternative_id(&name,&checkpoint_ids,control))?;
        let alternative = SpaceAlternative { id: alternative_id.clone(), name, checkpoint_ids };
        self.meta.dispatch(ArtifactCommand::Apply { mutations: vec![SpaceHistoryMutation::CreateSpaceAlternative(CreateSpaceAlternative { alternative })], transaction: None }, identity).await?;
        Ok(alternative_id)
    }

    /// 🔀️ Fans out to every member pinned by `checkpoint_id`'s `SpaceCheckpoint`, restoring
    /// each to its exact recorded `(checkpoint, alternative)`.
    pub async fn checkout_space_checkpoint(&mut self, checkpoint_id: &str, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let snapshot = self.meta.snapshot()?;
        let checkpoint = snapshot.checkpoints.iter().find(|checkpoint| checkpoint.id == checkpoint_id).ok_or(VcsError::NoCheckpoint)?;
        for pin in &checkpoint.members {
            if let Some(member) = self.members.get_mut(&pin.document_id) {
                member.checkout(&pin.checkpoint_id, &pin.alternative_id, identity).await?;
            }
        }
        Ok(())
    }

    /// 🔀️ Switches the studio's active alternative and fans out to its tip checkpoint's pins.
    pub async fn switch_space_alternative(&mut self, alternative_id: &str, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let snapshot = self.meta.snapshot()?;
        let alternative = snapshot.alternatives.iter().find(|alternative| alternative.id == alternative_id).ok_or_else(|| VcsError::UnknownAlternative(alternative_id.to_string()))?;
        let checkpoint_id = alternative.checkpoint_ids.last().cloned().ok_or(VcsError::NoCheckpoint)?;
        self.meta.dispatch(ArtifactCommand::Apply { mutations: vec![SpaceHistoryMutation::SwitchSpaceAlternative(SwitchSpaceAlternative { alternative_id: alternative_id.to_string() })], transaction: None }, identity).await?;
        self.checkout_space_checkpoint(&checkpoint_id, identity).await
    }

    /// ↩️ Derived, local-only undo: targets whichever registered member has the most recent
    /// `last_local_edit_timestamp` (by {@link HybridLogicalTimestamp::cmp_key}) and undoes just that
    /// member. Never dispatched against the meta-document — space-level undo has no `SpaceHistoryMutation`
    /// of its own, it is purely a cross-member ordering policy.
    pub async fn undo(&mut self, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let mut candidates = Vec::new();
        for (document_id, member) in self.members.iter() {
            if let Some(timestamp) = member.last_local_edit_timestamp().await {
                candidates.push((timestamp.cmp_key(), document_id.clone()));
            }
        }
        let target = candidates.into_iter().max_by_key(|(cmp_key, _)| *cmp_key).map(|(_, document_id)| document_id);
        let document_id = target.ok_or(VcsError::NothingToUndo)?;
        self.members.get_mut(&document_id).ok_or(VcsError::NothingToUndo)?.undo(identity).await
    }

    /// ↪️ Derived, local-only redo: mirrors `undo`, targeting the member with the most
    /// recent `last_undone_local_edit_timestamp`.
    pub async fn redo(&mut self, identity: &mut EntityIdentityAuthority<'_>) -> Result<(), VcsError> {
        let mut candidates = Vec::new();
        for (document_id, member) in self.members.iter() {
            if let Some(timestamp) = member.last_undone_local_edit_timestamp().await {
                candidates.push((timestamp.cmp_key(), document_id.clone()));
            }
        }
        let target = candidates.into_iter().max_by_key(|(cmp_key, _)| *cmp_key).map(|(_, document_id)| document_id);
        let document_id = target.ok_or(VcsError::NothingToRedo)?;
        self.members.get_mut(&document_id).ok_or(VcsError::NothingToRedo)?.redo(identity).await
    }
}
//#endregion SpaceHistoryDocument
//#endregion 🔖️Space
//#endregion 🔖️BlobStore
//#endregion 🔖️Backbone
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
