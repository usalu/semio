//! 📥️ Items of `store` that belong to `os-store-codec-factory` (PLAN-OS-KERNEL-CRATES.md, Appendix C): lifted out of `🏪️store/🦀️.rs`.
use semio_framework_os_store_text_io::ArtifactStoreSnapshot as _;
use semio_framework_value::retained_clone::RetainedCloneStep;
use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
use semio_framework_value::ValueError;
use semio_framework_os_spr::ActorId;
use semio_framework_os_spr::{Mutation, OpBinary, OpText};
use serde::Deserialize;
use semio_framework_os_store_engine_owners::DocumentStoreOwners;
use semio_framework_os_store_engine_owners::close_unscheduled_catalog;
use semio_framework_os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority;
use semio_framework_os_vcs::VcsError;
use semio_framework_os_store_space_schema::ArtifactCursor;
//#region 🔖️EphemeralLanes

use semio_framework_os_store_ephemeral::*;
use semio_framework_os_io::sqlite_snapshot_operations::ArtifactSqliteSnapshotOperations;
use semio_framework_os_store_text_io::{ArtifactCodec, ArtifactNativeSnapshotIdentity, ArtifactPackFiles, ArtifactSqliteSnapshotHost};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_store_text_io::ArtifactCodecApplyFuture;
#[cfg(target_arch = "wasm32")]
use semio_framework_os_store_text_io::ArtifactCodecApplyFuture;
use semio_framework_os_store_owned_schema_decode::artifact_retirement_self_grant;
use pack::snapshot::{ArtifactDsl,ArtifactPack};
use semio_framework_os_store_text_io::ArtifactStoreSnapshot;
use semio_framework_os_store_codec_registry::canonical_check_in_line;
use semio_framework_os_store_engine_owners::funded_bounded_artifact_store_owners;
use semio_framework_os_store_text_io::text_format::{ArtifactTextFiles, ParsedDocumentText, parse_document_pack, parse_document_text, print_document_pack, print_document_text, print_edit_lines};
use semio_framework_os_store_text_io::text_format::project_envelope_history;
use semio_framework_os_store_ledger::edit_from_operation_envelope;
use semio_framework_os_store_canonical_json::ArtifactCanonicalJsonTree;
use semio_framework_os_store_one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES;
use semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication;
//#region 🔖️Backbone
//#region 🔖️Backbone
use semio_framework_os_store_backbone as backbone;
use semio_framework_os_store_engine::lifted_store::ArtifactStore;
//#region 🔖️Authority
//#region 🔖️EphemeralLanes
//#region 🔖️Text
//#region 🔖️Pack
//#region 🔖️CodecRegistry
//#region 🔖️CodecRegistry
//#region 🗃️BoundedArtifactStoreOwners

/// 🪶️ Constructs a document kind's codec with a required, actual relational snapshot implementation, over
/// [`bare_artifact_codec`]. One call per document kind registers it (`register_document_codec_for_app`).
pub fn artifact_codec<P, Mutation>(schema: impl Into<String>) -> ArtifactCodec
where
    P: Clone + PartialEq + ToValue + FromValue + ArtifactDsl + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + ArtifactSqliteSnapshotHost + semio_framework_value::retirement::RetireOwned + Sync,
    Mutation: self::Mutation<P> + PartialEq + ToValue + FromValue + OpText + OpBinary + semio_framework_value::retirement::RetireOwned + Sync + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<P>,
{
    let mut codec = bare_artifact_codec::<P, Mutation>(schema);
    codec.snapshot_sqlite = Some(P::sqlite_codec());
    codec
}

/// 📦️ Constructs a native document codec from its owner's declared capabilities. The engine monomorphizes the erased
/// table's bridge functions for `(P, Mutation)` here — each a zero-sized `fn` item coerced to a bare `fn` pointer —
/// because they parse, print, reduce and replay through `ArtifactStore`; the table itself stays engine-free vocabulary.
///
/// 🚫️async: E4 fn-pointer erasure-table thunks — every bridge goes into an `ArtifactCodec` `fn` slot (unnameable if
/// `async fn`), so each wraps its real async body in `Box::pin` per R1(ii); `schema_hash` is matched explicitly because
/// `Option::map`'s closure is sync (R10 shape 1).
pub fn bare_artifact_codec<P, Mutation>(schema: impl Into<String>) -> ArtifactCodec
where
    P: Clone + PartialEq + ToValue + FromValue + ArtifactDsl + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Sync,
    Mutation: self::Mutation<P> + PartialEq + ToValue + FromValue + OpText + OpBinary + semio_framework_value::retirement::RetireOwned + Sync + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<P>,
{
    /// 🕰️`'static`, not `'a`. These items are coerced to `for<'a> fn(..) -> ..` pointers, so a
    /// `P: 'a` bound would have to hold for EVERY lifetime — which is `'static` by another name,
    /// and which the compiler cannot infer from the bound as written ("one type is more general
    /// than the other"). The enclosing `bare_artifact_codec<P, Mutation>` already requires `'static` of both, so
    /// saying it here costs nothing and lets the coercion succeed.
    fn compile_dsl_impl<'a, P, Mutation>(dsl: &'a str, ops: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(ArtifactPackFiles, String), VcsError>> + Send + 'a>>
    where
        P: Clone + ArtifactDsl + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static + FromValue,
        Mutation: OpText + OpBinary + self::Mutation<P> + Send + Sync + 'static + Clone + semio_framework_os_spr::ColdMutation<P>,
    {
        Box::pin(async move {
            let parsed: ParsedDocumentText<P, Mutation> = parse_document_text(dsl, ops, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
            let envelope = parsed.into_envelope();
            let pack_files = print_document_pack(&envelope).await;
            let dsl_mirror = envelope.vcs.genesis.facts().snapshot().print_dsl();
            envelope.retire_unadopted();
            Ok((pack_files?, dsl_mirror))
        })
    }

    /// 🕰️`'static`, not `'a`. These items are coerced to `for<'a> fn(..) -> ..` pointers, so a
    /// `P: 'a` bound would have to hold for EVERY lifetime — which is `'static` by another name,
    /// and which the compiler cannot infer from the bound as written ("one type is more general
    /// than the other"). The enclosing `bare_artifact_codec<P, Mutation>` already requires `'static` of both, so
    /// saying it here costs nothing and lets the coercion succeed.
    fn print_mirror_impl<'a, P, Mutation>(pack: &'a [u8], spr: &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<ArtifactTextFiles, VcsError>> + Send + 'a>>
    where
        P: Clone + ArtifactDsl + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static + FromValue,
        Mutation: OpText + OpBinary + self::Mutation<P> + Send + Sync + 'static + Clone + semio_framework_os_spr::ColdMutation<P>,
    {
        Box::pin(async move {
            let parsed: ParsedDocumentText<P, Mutation> = parse_document_pack(pack, spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
            let envelope = parsed.into_envelope();
            let mirror = print_document_text(&envelope).await;
            envelope.retire_unadopted();
            mirror
        })
    }

    /// 🎯️ `encode_ops_vec(&[])` is a NON-empty framed header, not a zero-length byte
    /// slice — the empty-batch fast path must key off the DECODED mutation count, or an
    /// empty gesture builds a store, hits `EmptyApply`, and (before TC5) aborted in Drop.
    ///
    /// 🛂️ The throwaway reduction store gets the framework's own bounded owner
    /// catalogue. `ArtifactStore::new` installs NONE, and an uninstalled store cannot
    /// retire: the close cursor below answers `artifact store has no owner-supplied
    /// bounded disposer` for the first nonempty batch through a LINKED Rust codec —
    /// the path stdio and gis take on a hub. The guest twin
    /// (`plugin_runtime::artifact_app_apply_ops`) installs the APP's own catalogue
    /// there, which this thunk has no app type to ask for; the framework catalogue is
    /// the exact equivalent for a store that lives only for one reduction (ticket
    /// 26/09/18 slices TC3e §7d, TC4 §2).
    ///
    /// 🗄️ Never `?` while `store` is live: `ArtifactStore`'s Drop asserts a terminal
    /// shallow shell, so an early return here aborts the process instead of reporting the
    /// fault. Capture every fallible step as a value, run the close cursor, forget the
    /// store, THEN propagate — the guest twin does the same (ticket 26/09/18 TC3e/TC5).
    fn apply_ops_binary_impl<'a, 'b, P, Mutation>(pack: &'a [u8], spr: &'a [u8], ops_vec: &'a [u8], identity: &'a mut EntityIdentityAuthority<'b>) -> ArtifactCodecApplyFuture<'a>
    where
        P: ToValue + ArtifactDsl + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Sync + Clone + FromValue,
        Mutation: ToValue + FromValue + OpText + OpBinary + self::Mutation<P> + semio_framework_value::retirement::RetireOwned + Sync + ArtifactCanonicalJsonTree + Clone + semio_framework_os_spr::ColdMutation<P>,
        {
        Box::pin(async move {
            let mutations: Vec<Mutation> = if ops_vec.is_empty() {
                Vec::new()
            } else {
                let op_blobs = semio_framework_os_spr::decode_ops_vec(ops_vec).map_err(|error| VcsError::Deserialize(error.to_string()))?;
                let mut decoded = Vec::with_capacity(op_blobs.len());
                for bytes in &op_blobs {
                    decoded.push(Mutation::decode_op(bytes).map_err(|error| VcsError::Deserialize(error.to_string()))?);
                }
                decoded
            };
            if mutations.is_empty() {
                if pack.is_empty() && spr.is_empty() {
                    return Ok((Vec::new(), Vec::new(), String::new()));
                }
                let parsed = parse_document_pack::<P, Mutation>(pack, spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
                let envelope = parsed.into_envelope();
                let printed = print_document_pack(&envelope).await;
                envelope.retire_unadopted();
                let files = printed?;
                return Ok((files.pack, files.spr, files.ops));
            }
            let (mut store, installed) = if pack.is_empty() && spr.is_empty() {
                return Err(VcsError::Deserialize("apply_ops_binary: lane has no pack+spr baseline".into()));
            } else {
                let parsed = parse_document_pack::<P, Mutation>(pack, spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
                let mut envelope = parsed.into_envelope();
                let (applied, redo) = match &envelope.cursor {
                    Some(cursor) => (cursor.applied_edit_ids.clone(), cursor.redo_edit_ids.clone()),
                    None => (envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect(), semio_framework_os_vcs::HistoryPageStack::new()),
                };
                envelope.cursor = Some(ArtifactCursor::new(applied, redo, envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone())));
                let mut store = ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(envelope, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await?;
                let installed = funded_bounded_artifact_store_owners::<P, Mutation>().and_then(|owners| store.install_document_store_owners_exact(owners).map_err(|(error, owners)| { close_unscheduled_catalog(owners); error })).map_err(|error| VcsError::ValidationFailed(error.into_message()));
                (store, installed)
            };
            let applied = match installed {
                Ok(()) => store.dispatch_apply_exact(mutations, identity).await,
                Err(error) => Err(error),
            };
            let printed = match &applied {
                Ok(_) => print_document_pack(&store.log.envelope).await,
                Err(_) => Err(VcsError::ValidationFailed("artifact codec apply skipped print after a failed reduction".into())),
            };
            let closed = close_codec_reduction_store(store);
            applied?;
            closed?;
            let files = printed?;
            Ok((files.pack, files.spr, files.ops))
        })
    }

    fn replay_envelopes_impl<'a, P, Mutation>(pack: &'a [u8], spr: &'a [u8], envelopes: &'a [u8]) -> ArtifactCodecApplyFuture<'a>
    where
        P: ToValue + ArtifactDsl + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + semio_framework_value::retirement::RetireOwned + Sync + Clone + FromValue,
        Mutation: Clone + ToValue + FromValue + OpText + OpBinary + self::Mutation<P> + semio_framework_value::retirement::RetireOwned + Sync + ArtifactCanonicalJsonTree + semio_framework_os_spr::ColdMutation<P>,
        {
        Box::pin(async move {
            let files = replay_envelopes_onto_pair::<P, Mutation>(pack, spr, envelopes, funded_bounded_artifact_store_owners::<P, Mutation>).await?;
            Ok((files.pack, files.spr, files.ops))
        })
    }

    fn edit_text_from_envelope_impl<'a, P, Mutation>(envelope: &'a semio_framework_os_spr::MutationEnvelope) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, VcsError>> + 'a>>
    where
        Mutation: OpText + OpBinary,
    {
        Box::pin(async move {
            let edit = edit_from_operation_envelope::<Mutation>(envelope).await?;
            print_edit_lines(&edit).await
        })
    }

    ArtifactCodec {
        schema: schema.into(),
        extension: P::EXTENSION,
        native_identity: ArtifactNativeSnapshotIdentity::typed::<P>(),
        snapshot_sqlite: P::sqlite_snapshot_codec(),
        pack_schema_hash: match P::record_spec() {
            Some(spec) => pack::record::schema_hash(&spec),
            None => [0u8; 32],
        },
        compile_dsl: compile_dsl_impl::<P, Mutation>,
        print_mirror: print_mirror_impl::<P, Mutation>,
        edit_text_from_envelope: edit_text_from_envelope_impl::<P, Mutation>,
        apply_ops_binary: apply_ops_binary_impl::<P, Mutation>,
        replay_envelopes: replay_envelopes_impl::<P, Mutation>,
    }
}

/// 🧺️ Close-cursor step ceiling for the throwaway store `ArtifactCodec::apply_ops_binary` builds.
/// Every `Pending` step releases at least one retained item, and a codec's store is bounded by one
/// parsed document pack, so this is a liveness ceiling rather than a work budget.
pub const ARTIFACT_CODEC_APPLY_CLOSE_MAXIMUM_STEPS: usize = 1 << 20;

/// 🧺️ Runs a throwaway reduction store's close cursor to exact terminal emptiness and forgets the
/// shell, so no codec thunk can reach `ArtifactStore`'s asserting `Drop` with a live owner. Shared by
/// [`ArtifactCodec::apply_ops_binary`] and [`replay_envelopes_onto_pair`].
pub fn close_codec_reduction_store<P, Mutation>(mut store: ArtifactStore<P, Mutation>) -> Result<(), VcsError>
where
    P: ToValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static + Clone + FromValue,
    Mutation: ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static + Clone + semio_framework_os_spr::ColdMutation<P>,
{
    let mut closed = Err(VcsError::ValidationFailed("artifact codec store did not reach terminal emptiness within its bounded close budget".into()));
    for _ in 0..ARTIFACT_CODEC_APPLY_CLOSE_MAXIMUM_STEPS {
        let step = store.close_owned_demands(ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES).and_then(|demand| store.close_owned_step(artifact_retirement_self_grant(demand, ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES)));
        match step {
            Ok(RetainedCloneStep::Complete(_)) => {
                closed = if store.close_owned_terminal_is_empty() { Ok(()) } else { Err(VcsError::ValidationFailed("artifact codec store reported close completion without terminal emptiness".into())) };
                break;
            }
            Ok(RetainedCloneStep::Progress(_)) => continue,
            Err(error) => {
                closed = Err(VcsError::ValidationFailed(error.into_message()));
                break;
            }
        }
    }
    let drop_ready = store.log.envelope_detached
        && store.live.current_detached
        && store.wire.backbone.is_none()
        && store.log.dag.terminal_is_empty()
        && store.log.applied_edit_ids.is_empty()
        && store.log.redo_edit_ids.is_empty()
        && store.log.current_checkpoint_id.is_none()
        && store.log.local_actor_id.0.is_empty()
        && store.cursor.revision_accumulator.applied.is_empty()
        && store.cursor.revision_accumulator.redo.is_empty()
        && store.cursor.revision_accumulator.mutation_positions.is_empty()
        && store.cursor.revision_accumulator.indexed_edits.is_empty()
        && store.cursor.revision_accumulator.unit_flags.is_empty()
        && store.live.tail_undo_cache.is_none()
        && store.owners.snapshot_read_leases.terminal_is_empty()
        && store.owners.displaced_retirements.terminal_is_empty()
        && store.owners.owned_disposer.is_none()
        && store.owners.owned_disposer_terminal
        && store.replay.pending_report.is_idle()
        && store.wire.durable_group_root.is_none();
    std::mem::forget(store);
    closed?;
    if drop_ready { Ok(()) } else { Err(VcsError::ValidationFailed("artifact codec store close left a live shallow-shell owner".into())) }
}

/// 📜️ Folds an [`semio_framework_os_spr::encode_envelopes`] ledger stream onto one authoritative pair through
/// the SAME gate every replica folds a remote envelope through ([`ArtifactStore::ingest_remote`]):
/// causal order, history transitions (undo/redo/checkpoint), edit identity and conflict quarantine are
/// exactly a replica's, so the printed pair is the one any client holding that ledger prefix holds.
/// A quarantined conflict or an envelope left waiting on an unknown dependency refuses the fold, and a
/// raw-mutation apply (`apply_ops_binary`) is never a substitute: it mints new edit ids and drops every
/// transition. A ledger that changed effective inputs is judged once, as the history it folds to
/// ([`ArtifactStore::refuse_blocking_history`]) — never at the intermediate states its envelopes pass through, so a later
/// supersession heals an earlier one. This is how a hub materializes a Check In from its own ledger.
///
/// See `🌎️hub/🗿️artifact-authority/📌️check-in` and `db::artifact_ledger_tail`.
pub async fn replay_envelopes_onto_pair<P, Mutation>(pack: &[u8], spr: &[u8], envelopes: &[u8], owners: impl FnOnce() -> Result<DocumentStoreOwners<P, Mutation>, ValueError>) -> Result<ArtifactPackFiles, VcsError>
where
    P: ToValue + ArtifactPack + semio_framework_os_store_text_io::ArtifactStoreSnapshot + Send + Sync + 'static + Clone + FromValue,
    Mutation: ToValue + FromValue + self::Mutation<P> + OpBinary + OpText + Send + 'static + Clone + semio_framework_os_spr::ColdMutation<P>,
{
    if pack.is_empty() || spr.is_empty() {
        return Err(VcsError::Deserialize("replay-envelopes has no pack+spr baseline".into()));
    }
    let envelopes = semio_framework_os_spr::decode_envelopes(envelopes).map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let parsed = parse_document_pack::<P, Mutation>(pack, spr, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer).await.map_err(|error| VcsError::Deserialize(error.to_string()))?;
    let mut envelope = parsed.into_envelope();
    let _canonical_line = canonical_check_in_line(&envelope.id);
    envelope.active_alternative_id = None;
    envelope.viewer_checkpoint_id = None;
    project_envelope_history(&mut envelope)?;
    let (applied, redo) = match &envelope.cursor {
        Some(cursor) => (cursor.applied_edit_ids.clone(), cursor.redo_edit_ids.clone()),
        None => (envelope.vcs.edits.iter().map(|edit| edit.id.clone()).collect(), semio_framework_os_vcs::HistoryPageStack::new()),
    };
    envelope.cursor = Some(ArtifactCursor::new(applied, redo, envelope.cursor.as_ref().and_then(|cursor| cursor.checkpoint_id.clone())));
    let mut store = ArtifactStore::new::<semio_framework_os_store_initialization_runtime::native_publication_producer::ArtifactSnapshotPublication>(envelope, ActorId(semio_framework_os_spr::LOCAL_ACTOR_ID.into())).await?;
    let baseline = store.supersessions().clone();
    let mut folded = owners().and_then(|owners| store.install_document_store_owners_exact(owners).map_err(|(error, owners)| { close_unscheduled_catalog(owners); error })).map_err(|error| VcsError::ValidationFailed(error.into_message()));
    for envelope in envelopes {
        if folded.is_err() { break; }
        match store.ingest_remote(envelope).await {
            Ok(report) if report.accepted && report.conflict.is_none() => {}
            Ok(_) => {
                folded = Err(VcsError::ValidationFailed("replay-envelopes quarantined a ledger envelope as a conflict".into()));
                break;
            }
            Err(error) => {
                folded = Err(error);
                break;
            }
        }
    }
    if folded.is_ok() && !store.log.dag.pending_is_empty() {
        folded = Err(VcsError::ValidationFailed("replay-envelopes left a ledger envelope waiting on an unknown dependency".into()));
    }
    if folded.is_ok() && *store.supersessions() != baseline {
        folded = store.refuse_blocking_history();
    }
    let printed = match &folded {
        Ok(()) => print_document_pack(&store.log.envelope).await,
        Err(_) => Err(VcsError::ValidationFailed("replay-envelopes skipped print after a refused fold".into())),
    };
    let closed = close_codec_reduction_store(store);
    folded?;
    closed?;
    printed
}
//#endregion 🗃️BoundedArtifactStoreOwners
//#endregion 🔖️CodecRegistry
//#endregion 🔖️CodecRegistry
//#endregion 🔖️Pack
//#endregion 🔖️Text
//#endregion 🔖️EphemeralLanes
//#endregion 🔖️Authority

//#endregion 🔖️Backbone
//#endregion 🔖️Backbone
//#endregion 🔖️EphemeralLanes
