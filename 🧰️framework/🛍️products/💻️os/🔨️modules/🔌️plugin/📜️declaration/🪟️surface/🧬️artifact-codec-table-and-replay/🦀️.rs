//! 🧬️ Artifact codec table and replay: the server minted document id check, the genesis pair, the pair snapshot, op application, envelope replay and the version one codec table. See [`super`].

use super::*;

/// 🪪️ Accepts only one server-minted nonzero lowercase artifact identity.
fn is_server_minted_artifact_document_id_v1(document_id: &str) -> bool {
    let Some(suffix) = document_id.strip_prefix("artifact-") else { return false };
    suffix.len() == 32 && suffix.bytes().all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte)) && suffix.bytes().any(|byte| byte != b'0')
}

/// 🪡️ Builds one app-owned initial pair at a server-minted identity while preserving an exact
/// empty history and cursor. This is the single producer behind `world actor`'s `codec.genesis`
/// export, so every plugin gets creation authority from the one implementation rather than from
/// a per-package native factory the host has to link (ticket 26/09/18 slice TC3b).
pub async fn artifact_app_genesis_pair<A: ArtifactApp>(document_id: &str) -> Result<store::os_store::serialization::ArtifactPackFiles, store::VcsError> {
    let dialect: ArtifactDialect = A::DIALECT.into();
    if !is_server_minted_artifact_document_id_v1(document_id) {
        return Err(store::VcsError::ValidationFailed("artifact genesis identity is not a server-minted artifact id".into()));
    }
    let codec=<A::Snapshot as store::os_store::serialization::ArtifactStoreSnapshot>::reference_snapshot_codec().ok_or_else(||store::VcsError::NativeEncoding(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"native genesis requires the original retained constructor")))?;
    let birth=A::reference_birth().ok_or_else(||store::VcsError::NativeEncoding(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"genesis requires its explicit reference producer or retained native source")))?;
    let snapshot=birth().map_err(|fault|store::VcsError::ValidationFailed(format!("{}: {}",fault.code.0,fault.message)))?;
    let mut envelope = create_document_envelope::<A::Snapshot,A::Mutation>(A::DOCUMENT_SCHEMA,document_id,snapshot,None,&codec).into_owners();
    envelope.dialect = Some(dialect.clone());
    let zero_cursor = envelope.cursor.as_ref().is_some_and(|cursor| cursor.applied_edit_ids.is_empty() && cursor.redo_edit_ids.is_empty() && cursor.checkpoint_id.is_none());
    if envelope.id != document_id
        || envelope.dialect.as_ref() != Some(&dialect)
        || !envelope.vcs.edits.is_empty()
        || !envelope.vcs.changes.is_empty()
        || !envelope.vcs.checkpoints.is_empty()
        || !envelope.vcs.alternatives.is_empty()
        || !zero_cursor
    {
        ArtifactEnvelope::from_owners(envelope).retire_unadopted();
        return Err(store::VcsError::ValidationFailed("artifact genesis produced nonzero history".into()));
    }
    let printed = store::os_store::text_format::print_document_pack(&envelope).await;
    ArtifactEnvelope::from_owners(envelope).retire_unadopted();
    printed
}

/// 🧩️ Applies one `os_spr::encode_ops_vec` batch to an app-owned pair — the guest half of
/// `world actor`'s `codec.apply-ops` export, structurally the same reduction
/// `store::os_store::serialization::ArtifactCodec::apply_ops_binary` performs for a natively linked codec.
/// 🔭️ The LIVE snapshot of one artifact `pack`/`spr` pair, without loading it into any
/// instance and without leaving a history shell behind. This is how a READER — an inference
/// computing over the artifact its caller named, never an editor — gets the document it was
/// asked about: the pair is the authoritative one, `parse_document_pack` replays it exactly as
/// a document archive load folds it, and `into_snapshot` retires the envelope through the store's own
/// bounded path instead of dropping it (a bare drop aborts the process — every terminal shell
/// under `ArtifactEnvelope` asserts its owners were retired first).
pub async fn artifact_pair_snapshot<P, Mutation>(pack: &[u8], spr: &[u8]) -> Result<P, store::VcsError>
where
    P: ArtifactPack + ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot + store::StoreState,
    Mutation: store::StoreMutationFor<P>,
{
    if pack.is_empty() || spr.is_empty() {
        return Err(store::VcsError::Deserialize("artifact pair carries no pack+spr baseline".into()));
    }
    let parsed: store::os_store::text_format::ParsedDocumentText<P, Mutation> = store::os_store::text_format::parse_document_pack(pack, spr, &::semio_framework_os_kernel::os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| store::VcsError::Deserialize(error.to_string()))?;
    Ok(parsed.into_snapshot())
}

/// 🎟️ Refuses borrowed codec sources before decode until an original retained reduction recipient exists.
pub async fn artifact_app_apply_ops<A: ArtifactApp>(_pack: &[u8], _spr: &[u8], _ops: &[u8], _identity: &mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'_>) -> Result<store::os_store::serialization::ArtifactPackFiles, store::VcsError> {
    Err(store::VcsError::NativeEncoding(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"codec requires an original retained reduction operation")))
}

/// 📜️ Preserves the typed blocking history fault at the codec boundary.
pub fn replay_envelopes_fault(error: store::VcsError) -> Fault {
    match error {
        store::VcsError::Rejected { messages, .. } => {
            let blocking: Vec<String> = messages.iter().filter(|message| protocol::MergePolicy::Normal.rejects(message.level)).take(8).map(|message| format!("{}: {}", message.code.0, message.message)).collect();
            Fault::new(FaultOrigin::Framework, FaultCode::new("history.ledger-not-replayable"), format!("the ledger does not replay under the check-in policy ({} blocking message(s)): {}", messages.len(), blocking.join("; ")))
        }
        other => other.into_fault(),
    }
}

/// 🎟️ Refuses the borrowed ledger before parsing or an unfunded catalog construction.
pub async fn artifact_app_replay_envelopes<A: ArtifactApp>(_pack: &[u8], _spr: &[u8], _envelopes: &[u8]) -> Result<store::os_store::serialization::ArtifactPackFiles, store::VcsError> {
    Err(store::VcsError::NativeEncoding(ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"codec requires an original retained reduction operation")))
}

/// 🧬️ The future one `codec` answer of an [`ArtifactCodecTableV1`] resolves to.
pub type ArtifactCodecFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, Fault>> + 'a>>;

/// 🧬️ One registered app's `world actor` `codec` answers as plain functions of its type, recorded when the app
/// is registered. Every answer is a pure function of the app's TYPE — its snapshot record shape, its initial
/// snapshot and dialect, its owner catalogue — never of a constructed instance, so a `codec` call reads this
/// table and constructs no app: no store around an initial snapshot, no action registry, no window owners,
/// nothing to drain and close afterwards.
#[derive(Clone, Copy)]
pub struct ArtifactCodecTableV1 {
    /// 🧬️ Structural fingerprint of the app's snapshot record shape — the same 32 bytes
    /// `store::os_store::serialization::ArtifactCodec::pack_schema_hash` carries, read off `Snapshot::record_spec()`. `None` when the
    /// snapshot is a hand-written `ArtifactPack` with no `RecordSpec`, exactly as the native codec table reports
    /// `[0; 32]` for that case.
    /// 🚫️async: E4 fn-pointer slot
    pub pack_schema_hash: fn() -> Option<[u8; 32]>,
    /// 🌱️ The canonical empty document of the app's kind at `document_id`: its initial snapshot, its own
    /// dialect, and an exactly zero history (no edits, changes, checkpoints or alternatives, and an empty cursor).
    /// 🚫️async: E4 fn-pointer slot
    pub genesis: for<'a> fn(&'a str) -> ArtifactCodecFuture<'a, store::os_store::serialization::ArtifactPackFiles>,
    /// 📥️ `(pack, spr) -> (dsl, ops)` mirror of a pair of the app's kind — the pair-validation fence of a host
    /// whose codec it does not link.
    /// 🚫️async: E4 fn-pointer slot
    pub print_mirror: for<'a> fn(&'a [u8], &'a [u8]) -> ArtifactCodecFuture<'a, store::os_store::text_format::ArtifactTextFiles>,
    /// 🧩️ Applies one `os_spr::encode_ops_vec` batch to a pair of the app's kind and returns the next pair.
    /// 🚫️async: E4 fn-pointer slot
    pub apply_ops: for<'a, 'b> fn(&'a [u8], &'a [u8], &'a [u8], &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>) -> ArtifactCodecFuture<'a, store::os_store::serialization::ArtifactPackFiles>,
    /// 📜️ Folds one `os_spr::encode_envelopes` ledger stream onto a pair of the app's kind through the replica
    /// merge gate (`store::replay_envelopes_onto_pair`).
    /// 🚫️async: E4 fn-pointer slot
    pub replay_envelopes: for<'a> fn(&'a [u8], &'a [u8], &'a [u8]) -> ArtifactCodecFuture<'a, store::os_store::serialization::ArtifactPackFiles>,
}

/// 🧬️ The [`ArtifactCodecTableV1`] of every app built on `A`.
pub fn artifact_codec_table<A: ArtifactApp>() -> ArtifactCodecTableV1 {
    fn pack_schema_hash<A: ArtifactApp>() -> Option<[u8; 32]> {
        <A::Snapshot as pack::snapshot::ArtifactPackMetadata>::native_record_spec_producer().map(|_|<A::Snapshot as pack::snapshot::ArtifactPackMetadata>::native_pack_schema_hash())
    }
    fn genesis<A: ArtifactApp>(document_id: &str) -> ArtifactCodecFuture<'_, store::os_store::serialization::ArtifactPackFiles> {
        Box::pin(async move { artifact_app_genesis_pair::<A>(document_id).await.map_err(|error| error.into_fault()) })
    }
    fn print_mirror<'a, A: ArtifactApp>(pack: &'a [u8], spr: &'a [u8]) -> ArtifactCodecFuture<'a, store::os_store::text_format::ArtifactTextFiles> {
        Box::pin(async move {
            let descriptor=store::os_store::native_artifact_codec::<A::Snapshot>(A::DOCUMENT_SCHEMA);
            let reference=descriptor.reference().map_err(store::VcsError::into_fault)?;
            (reference.print_mirror)(pack,spr).await.map_err(store::VcsError::into_fault)
        })
    }
    fn apply_ops<'a, 'b, A: ArtifactApp>(pack: &'a [u8], spr: &'a [u8], ops: &'a [u8], identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>) -> ArtifactCodecFuture<'a, store::os_store::serialization::ArtifactPackFiles> {
        Box::pin(async move { artifact_app_apply_ops::<A>(pack, spr, ops, identity).await.map_err(|error| error.into_fault()) })
    }
    fn replay_envelopes<'a, A: ArtifactApp>(pack: &'a [u8], spr: &'a [u8], envelopes: &'a [u8]) -> ArtifactCodecFuture<'a, store::os_store::serialization::ArtifactPackFiles> {
        Box::pin(async move { artifact_app_replay_envelopes::<A>(pack, spr, envelopes).await.map_err(replay_envelopes_fault) })
    }
    ArtifactCodecTableV1 { pack_schema_hash: pack_schema_hash::<A>, genesis: genesis::<A>, print_mirror: print_mirror::<A>, apply_ops: apply_ops::<A>, replay_envelopes: replay_envelopes::<A> }
}
