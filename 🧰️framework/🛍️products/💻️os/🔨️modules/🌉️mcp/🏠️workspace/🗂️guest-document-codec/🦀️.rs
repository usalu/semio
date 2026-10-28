//! 🗂️ Guest document codec routes: print, apply, replay, compile and sqlite snapshot bridges.

#[cfg(not(target_arch = "wasm32"))]
use semio_framework_plugin_host::GuestRuntimes;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
use std::sync::Mutex;
#[cfg(not(target_arch = "wasm32"))]
use crate::compiled_components::shared_plugin_runtime;
#[cfg(not(target_arch = "wasm32"))]
use crate::plugin_artifact_channel::shared_compiled_component;
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_mcp_foundation::errors::{GatewayError, GatewayErrorCode};

//#region 🗂️GuestDocumentCodec

/// ⛽️ The ceiling one pure `codec` call runs under. The WIT states the contract these functions
/// honour — no storage, no window, no effect, one throwaway instance per call — so what bounds them
/// is wall time, not effect or frame counts. 60 s is the same order as [`headless_open_budget`],
/// because printing or applying against an 80 KB document pack is work of the same scale as bringing
/// an instance up, and a guest that needs longer is wedged rather than busy.
/// 🧵️ The binding generation this gateway claims for a document backbone. One session opens one
/// binding per instance and never rebinds — a rebind is what a shell does when a window swaps
/// documents, which a headless channel with one bound artifact per plugin cannot do — so the
/// generation is a constant rather than a counter that would imply a history it does not have.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) const DOCUMENT_BACKBONE_BINDING_GENERATION: u64 = 1;

#[cfg(not(target_arch = "wasm32"))]
fn headless_codec_budget() -> semio_framework::kernel::Budget {
    semio_framework::kernel::Budget { fuel: u64::MAX, deadline_us: 60_000_000, ..semio_framework_plugin_host::GUEST_CODEC_BUDGET }
}

/// 🌐️ The process's document-socket I/O reactor, for the hub DOCUMENT SOCKET and nothing else: the one [`store::sync::DocumentSocketReactor`] the native artifact actors poll their own socket against, so this gateway owns no runtime of its own.
///
/// 🐛️ A hub document opened here dials its socket on a `WorkerPool` thread that no Tokio runtime owns (`semio_framework_os_services::TokioHostRuntime` carries no reactor despite the name), so the first dial panicked with "there is no reactor running" until the caller entered a reactor around `ArtifactHost::open` and the guest codec calls (measured live on hub 7681, 2026-09-22). The same reactor drives the event pump of [`store::sync::ArtifactHost::pump_events_key`].
#[cfg(not(target_arch = "wasm32"))]
pub fn hub_socket_reactor() -> Result<&'static store::sync::DocumentSocketReactor, GatewayError> {
    store::sync::DocumentSocketReactor::shared().map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("starting the hub document-socket reactor: {error}")))
}

/// 💓️ Whether a hub document event is a moment an agent session owes the hub a presence beat: a
/// new connection (`Session`) or a live status on it. Everything else leaves the roster as it is.
#[cfg(not(target_arch = "wasm32"))]
pub fn agent_presence_moment(event: &store::sync::ArtifactEvent) -> bool {
    matches!(event, store::sync::ArtifactEvent::Session { .. } | store::sync::ArtifactEvent::Status(store::sync::ArtifactSyncStatus { remote: store::sync::RemoteState::Live { .. }, .. }))
}

/// 🤖️ The identity-only peer an agent session beats: every app-owned ephemeral absent, and no
/// admitted field claimed, since the hub overwrites label, user, role, color, surface and principal
/// kind with what it authenticated for the socket.
#[cfg(not(target_arch = "wasm32"))]
pub fn agent_presence_peer(actor: &str) -> store::os_spr::PresencePeer {
    store::os_spr::PresencePeer {
        actor: actor.to_string(),
        connected_at_ms: 0,
        label: None,
        presence_pack: None,
        user_id: None,
        role: None,
        drag_ghost_json: None,
        interaction: None,
        color: None,
        surface: None,
        views: Vec::new(),
        ui: None,
        tool_run: None,
        principal_kind: None,
        active_tool: None,
        history_edit: None,
        typing: Vec::new(),
    }
}

/// 🗂️ One package's own document codec, as the component exports it — the compiled component plus
/// the artifact schema it was registered for.
///
/// 🧭️ `runtime` is the process-wide [`shared_plugin_runtime`] and never a second one: a
/// `wasmtime::Component` belongs to the `Engine` that compiled it, so `compiled` is only
/// instantiable by the runtime that produced it.
#[cfg(not(target_arch = "wasm32"))]
struct GuestCodecRoute {
    pub dialect: semio_framework_artifact_reference::ArtifactDialect,
    pub artifact_schema: String,
    pub plugin_id: String,
    pub runtime: Arc<GuestRuntimes>,
    pub compiled: semio_framework_plugin_host::CompiledHandle,
}

/// 🗂️ Every guest-backed codec this process has registered, in registration order.
///
/// 🧭️ A process-global table exists because [`store::os_store::serialization::ArtifactCodec`]'s four operations are bare,
/// NON-CAPTURING `fn` pointers — an erasure table, by design, so that a schema-string-keyed caller
/// can print and apply without naming a concrete `P`/`Mutation`. A thunk therefore cannot close over
/// a component, and cannot be told which schema it was registered under either: the signatures carry
/// only bytes. So a thunk resolves its route by asking the components themselves, in order, and
/// taking the first that answers — see [`guest_print_mirror`] for why that is the WIT's own
/// discriminator rather than a guess.
#[cfg(not(target_arch = "wasm32"))]
fn guest_codec_routes() -> &'static Mutex<Vec<Arc<GuestCodecRoute>>> {
    static ROUTES: std::sync::OnceLock<Mutex<Vec<Arc<GuestCodecRoute>>>> = std::sync::OnceLock::new();
    ROUTES.get_or_init(|| Mutex::new(Vec::new()))
}

#[cfg(not(target_arch = "wasm32"))]
fn guest_codec_route_snapshot() -> Vec<Arc<GuestCodecRoute>> {
    guest_codec_routes().lock().unwrap_or_else(std::sync::PoisonError::into_inner).clone()
}

/// 📥️ `(pack, spr) -> (dsl, ops)` through whichever registered component owns this pair.
///
/// 🧭️ **The component IS the discriminator.** `interface codec`'s own contract states it: "a pair
/// the component cannot print back is not a document of this kind". So asking each registered
/// package in turn and taking the first that answers is not a guess at the schema — it is running
/// the exact fence the interface defines, once per candidate. In a real gateway session the
/// candidate list is the packages whose documents this agent has open, which is one or two.
#[cfg(not(target_arch = "wasm32"))]
pub fn guest_print_mirror<'a>(pack: &'a [u8], spr: &'a [u8]) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<store::os_store::text_format::ArtifactTextFiles, store::VcsError>> + Send + 'a>> {
    Box::pin(async move {
        let routes = guest_codec_route_snapshot();
        let mut refusals: Vec<String> = Vec::new();
        for route in &routes {
            match route.runtime.codec_print_mirror(&route.compiled, &route.artifact_schema, pack, spr, &headless_codec_budget()).await {
                Ok(mirror) => return Ok(store::os_store::text_format::ArtifactTextFiles { dsl: mirror.dsl, ops: mirror.ops }),
                Err(error) => refusals.push(format!("{}/{}: {error}", route.plugin_id, route.artifact_schema)),
            }
        }
        Err(store::VcsError::Deserialize(format!("no registered guest codec prints this pair ({} candidate(s): {})", routes.len(), refusals.join("; "))))
    })
}

/// 🧩️ `(pack, spr, encode_ops_vec) -> (pack, spr, ops text)` through the same route resolution.
/// The ops text is the component's own print of the RESULT pair, never a host re-derivation.
#[cfg(not(target_arch = "wasm32"))]
pub fn guest_apply_ops_binary<'a, 'b>(pack: &'a [u8], spr: &'a [u8], ops: &'a [u8], identity: &'a mut semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority<'b>) -> store::os_store::serialization::ArtifactCodecApplyFuture<'a> {
    Box::pin(async move {
        let routes = guest_codec_route_snapshot();
        let mut refusals: Vec<String> = Vec::new();
        for route in &routes {
            let budget = headless_codec_budget();
            let applied = match route.runtime.codec_apply_ops(&route.compiled, &route.artifact_schema, pack, spr, ops, &budget, &mut *identity).await {
                Ok(applied) => applied,
                Err(error) => {
                    refusals.push(format!("{}/{}: {error}", route.plugin_id, route.artifact_schema));
                    continue;
                }
            };
            let mirror = route
                .runtime
                .codec_print_mirror(&route.compiled, &route.artifact_schema, &applied.pack, &applied.spr, &budget)
                .await
                .map_err(|error| store::VcsError::Serialize(format!("{}/{} applied the batch but cannot print the result: {error}", route.plugin_id, route.artifact_schema)))?;
            return Ok((applied.pack, applied.spr, mirror.ops));
        }
        Err(store::VcsError::Deserialize(format!("no registered guest codec applies this batch ({} candidate(s): {})", routes.len(), refusals.join("; "))))
    })
}

/// 📜️ `(pack, spr, encode_envelopes) -> (pack, spr, ops text)` — the replica fold of a ledger
/// stream through the same route resolution; the ops text is the component's own print of the result.
#[cfg(not(target_arch = "wasm32"))]
fn guest_replay_envelopes<'a>(pack: &'a [u8], spr: &'a [u8], envelopes: &'a [u8]) -> store::os_store::serialization::ArtifactCodecApplyFuture<'a> {
    Box::pin(async move {
        let routes = guest_codec_route_snapshot();
        let mut refusals: Vec<String> = Vec::new();
        for route in &routes {
            let budget = headless_codec_budget();
            let replayed = match route.runtime.codec_replay_envelopes(&route.compiled, &route.artifact_schema, pack, spr, envelopes, &budget, |_, _| {}, &semio_framework_plugin_guest_contract::GuestCallCancellation::default()).await {
                Ok(replayed) => replayed,
                Err(error) => {
                    refusals.push(format!("{}/{}: {error}", route.plugin_id, route.artifact_schema));
                    continue;
                }
            };
            let mirror = route
                .runtime
                .codec_print_mirror(&route.compiled, &route.artifact_schema, &replayed.pack, &replayed.spr, &budget)
                .await
                .map_err(|error| store::VcsError::Serialize(format!("{}/{} replayed the ledger but cannot print the result: {error}", route.plugin_id, route.artifact_schema)))?;
            return Ok((replayed.pack, replayed.spr, mirror.ops));
        }
        Err(store::VcsError::Deserialize(format!("no registered guest codec replays this ledger ({} candidate(s): {})", routes.len(), refusals.join("; "))))
    })
}

/// 🚫️ `interface codec` exports five functions and neither of these is one of them: there is no
/// `compile-dsl` and no `edit-text-from-envelope` in the WIT, because both belong to the FOLDER text
/// lane (`FolderTextStorage`'s `.dsl`/`.ops` writes) and a guest-backed codec exists precisely for a
/// binding that has no folder. A typed refusal naming that is the honest answer; fabricating text
/// here would put bytes in a `.ops` file that no component ever produced.
#[cfg(not(target_arch = "wasm32"))]
pub fn guest_compile_dsl<'a>(_dsl: &'a str, _ops: &'a str) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(store::os_store::serialization::ArtifactPackFiles, String), store::VcsError>> + Send + 'a>> {
    Box::pin(async move { Err(store::VcsError::Deserialize("a guest-backed document codec has no `compile-dsl`: `interface codec` exports pack-schema-hash, genesis, print-mirror, apply-ops and replay-envelopes only".to_string())) })
}

#[cfg(not(target_arch = "wasm32"))]
pub fn guest_edit_text_from_envelope<'a>(_envelope: &'a store::os_spr::MutationEnvelope) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<String, store::VcsError>> + 'a>> {
    Box::pin(async move { Err(store::VcsError::Deserialize("a guest-backed document codec has no per-envelope text printer: `interface codec` prints whole pairs, not single edits".to_string())) })
}

/// 🪶️ Resolves semantic snapshot conversion by its declared schema and exact dialect.
/// 🖥️ Classifies the VM's intrinsic outcome separately from a provider rejection.
#[cfg(not(target_arch = "wasm32"))]
fn guest_sqlite_turn_error(error: semio_framework_plugin_guest_contract::TurnFault) -> semio_framework::io_schema::IoError {
    use semio_framework_plugin_guest_contract::TurnFault;
    use semio_framework_value::{ValueError, ValueRefusalKind};
    let kind = match &error {
        TurnFault::Cancelled => ValueRefusalKind::Canceled,
        TurnFault::Exhausted | TurnFault::DeadlineExceeded | TurnFault::FuelExhausted => ValueRefusalKind::WorkLimit,
        TurnFault::ActorCall(_) | TurnFault::Host(_) | TurnFault::Guest(_) | TurnFault::Trapped(_) => ValueRefusalKind::InvariantViolated,
    };
    semio_framework::io_schema::IoError::from_value_error(ValueError::new(kind, error.to_string()))
}

#[cfg(not(target_arch = "wasm32"))]
fn guest_sqlite_route(schema: &str, dialect: &semio_framework_artifact_reference::ArtifactDialect) -> Result<Arc<GuestCodecRoute>, semio_framework::io_schema::IoError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    guest_codec_routes().lock().unwrap_or_else(std::sync::PoisonError::into_inner).iter().find(|route| route.artifact_schema == schema && &route.dialect == dialect).cloned().ok_or_else(|| semio_framework::io_schema::IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!("no guest SQLite provider for {schema} at {}", dialect.to_coordinate()))))
}

#[cfg(not(target_arch = "wasm32"))]
fn guest_sqlite_export(schema: &str, dialect: &semio_framework_artifact_reference::ArtifactDialect, payload: &semio_framework::io_schema::IoPayload, control: &mut semio_framework::sqlite_snapshot::SqliteSnapshotControl<'_>, native: &mut store::NativeSnapshotDecodeOwner<'_, '_>) -> semio_framework::io_schema::IoResult<semio_framework::sqlite_snapshot::SqliteDatabase> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    use semio_framework::{ io_schema::{IoError, IoOutcome, IoPayload}, sqlite_snapshot::{self, SnapshotEncoding, SqliteSnapshotPhase}};
    use semio_framework_os_kernel::io::{self};
    use semio_framework_plugin_guest_contract::{sqlite_wire, GuestCallCancellation};
    control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 0).map_err(IoError::from_value_error)?;
    let route = guest_sqlite_route(schema, dialect)?;
    let (encoding, bytes) = match payload { IoPayload::Binary(bytes) => (SnapshotEncoding::Binary, bytes.as_slice()), IoPayload::Text(text) => (SnapshotEncoding::Text, text.as_bytes()) };
    let limits = control.limits();
    let cancel = GuestCallCancellation::default();
    let _reactor = hub_socket_reactor().map_err(|error| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated, error.to_string())))?.enter();
    let mut progress_refusal = None;
    let grant=native.grant();
    let result=native.native().with_encoding_receiver(|remaining,observer,allocate|{
        let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new_forwarded(remaining,observer,allocate).map_err(IoError::from_value_error)?;
        semio_framework_async::block_on(route.runtime.codec_sqlite_export(&route.compiled,&dialect.to_coordinate(),encoding.as_str(),bytes,limits,&headless_codec_budget(),|_,_|{if let Err(cause)=control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0){if progress_refusal.is_none(){progress_refusal=Some(cause)}cancel.cancel();}},&cancel,grant,&mut identity)).map_err(|error|progress_refusal.take().map(IoError::from_value_error).unwrap_or_else(||guest_sqlite_turn_error(error)))
    })?;
    let file = match result { sqlite_wire::SnapshotFileResult::Done(file) => file, sqlite_wire::SnapshotFileResult::Rejected(rejection) => return Err(rejection.into_io_error().map_err(IoError::from_value_error)?), sqlite_wire::SnapshotFileResult::Pending(_) => return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, "guest SQLite export remains in guest custody pending its funded retirement"))) };
    let mut database = sqlite_snapshot::import_sqlite_database_controlled(&file.bytes, control).map_err(IoError::from_value_error)?;
    let (actual, actual_encoding) = io::io_mechanism::take_sqlite_snapshot_metadata_controlled(&mut database,control).map_err(IoError::from_value_error)?;
    if actual != *dialect || actual_encoding != encoding { return Err(IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "guest SQLite export metadata disagrees with its exact requested snapshot"))); }
    Ok(IoOutcome { value: database, diagnostics: sqlite_wire::decode_diagnostics(&file.diagnostics).map_err(IoError::from_value_error)? })
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(semio_framework_value::RetireOwned)]
struct GuestSqliteImportFrame {
    pub(crate) database: Option<semio_framework::sqlite_snapshot::SqliteDatabase>,
    pub bytes: Vec<u8>,
    pub(crate) coordinate: String,
    pub(super) wire: Option<semio_framework_plugin_guest_contract::sqlite_wire::SnapshotPayloadResult>,
    diagnostics: Vec<semio_framework::Diagnostic>,
}

/// 🪶️ Quotes the exact original coordinate allocation before its caller slot adopts it.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn guest_sqlite_import_coordinate(dialect:&semio_framework_artifact_reference::ArtifactDialect,coordinate:&mut String,native:&mut store::NativeSnapshotEncodeOwner<'_, '_>)->Result<(),semio_framework_value::ValueError>{
    use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress};
    if !coordinate.is_empty()||coordinate.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"guest coordinate original slot is occupied"))}
    let extent=dialect.artifact_kind.len().checked_add(dialect.standard.len()).and_then(|n|n.checked_add(dialect.subset.len())).and_then(|n|n.checked_add(2)).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"guest dialect coordinate extent overflow"))?;
    let remaining=native.remaining_grant();
    if remaining.maximum_items==0{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"guest coordinate requires original work authority"))}
    if remaining.maximum_depth==0{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"guest coordinate requires original depth authority"))}
    if remaining.maximum_copy_bytes<extent||remaining.maximum_capacity_bytes<extent{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"guest coordinate requires original physical authority"))}
    let layout=std::alloc::Layout::array::<u8>(extent).map_err(|_|ValueError::literal(ValueRefusalKind::OwnershipLimit,"guest dialect coordinate allocation extent overflow"))?;
    let result=native.native().scoped_stage(|native|{
        native.begin_stage(extent)?;native.charge(extent)?;
        let pointer=unsafe{std::alloc::alloc(layout)};
        if pointer.is_null(){return Err(ValueError::literal(ValueRefusalKind::AllocationFailed,"guest coordinate allocation failed"))}
        *coordinate=unsafe{String::from_raw_parts(pointer,0,extent)};
        for part in [dialect.artifact_kind.as_str(),"@",dialect.standard.as_str(),"/",dialect.subset.as_str()]{coordinate.push_str(part);native.advance(part.len())?;native.checkpoint()?;}
        Ok(())
    });
    let progress=RetainedCloneProgress{copied_items:usize::from(coordinate.capacity()!=0),copied_bytes:coordinate.len(),retained_capacity_bytes:if coordinate.capacity()!=0{extent}else{0},released_bytes:0};
    native.record_progress(progress).map_err(|error|error.with_retained_progress(progress))?;
    result.map_err(|error|error.with_retained_progress(progress))
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn guest_sqlite_import(schema: &str, dialect: &semio_framework_artifact_reference::ArtifactDialect, database: &mut Option<semio_framework::sqlite_snapshot::SqliteDatabase>, encoding: semio_framework::sqlite_snapshot::SnapshotEncoding, control: &mut semio_framework::sqlite_snapshot::SqliteSnapshotControl<'_>, native: &mut store::NativeSnapshotEncodeOwner<'_, '_>) -> semio_framework::io_schema::IoResult<semio_framework::io_schema::IoPayload> {
    use semio_framework::{io_schema::{IoError,IoOutcome,IoPayload},sqlite_snapshot::{self,SqliteSnapshotPhase}};
    use semio_framework_plugin_guest_contract::{sqlite_wire,GuestCallCancellation};
    use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneGrant};
    if database.is_none(){return Err(IoError::from_value_error(ValueError::literal(ValueRefusalKind::InvariantViolated,"guest SQLite import original database is absent")))}
    native.receive_nested::<GuestSqliteImportFrame,semio_framework::io_schema::IoResult<IoPayload>>(|slot,native|{
        *slot=Some(GuestSqliteImportFrame{database:database.take(),bytes:Vec::new(),coordinate:String::new(),wire:None,diagnostics:Vec::new()});
        let frame=slot.as_mut().unwrap();
        Ok((||{
            control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0).map_err(IoError::from_value_error)?;
            let route=guest_sqlite_route(schema,dialect)?;
            let limits=control.limits();
            semio_framework_os_kernel::io::io_mechanism::attach_sqlite_snapshot_metadata(frame.database.as_mut().unwrap(),dialect,encoding,control).map_err(IoError::from_value_error)?;
            frame.bytes=sqlite_snapshot::export_sqlite_database_controlled(frame.database.as_ref().unwrap(),control).map_err(IoError::from_value_error)?;
            guest_sqlite_import_coordinate(dialect,&mut frame.coordinate,native).map_err(IoError::from_value_error)?;
            let cancel=GuestCallCancellation::default();
            let _reactor=hub_socket_reactor().map_err(|error|IoError::from_value_error(ValueError::new(ValueRefusalKind::InvariantViolated,error.to_string())))?.enter();
            let mut progress_refusal=None;
            let grant:RetainedCloneGrant=native.remaining_grant();
            frame.wire=Some(native.native().with_encoding_receiver(|remaining,observer,allocate|{
                let mut identity=semio_framework_os_kernel::os_vcs::io::binary::entity_identity::control::EntityIdentityAuthority::new_forwarded(remaining,observer,allocate).map_err(IoError::from_value_error)?;
                semio_framework_async::block_on(route.runtime.codec_sqlite_import(&route.compiled,&frame.coordinate,&frame.bytes,limits,&headless_codec_budget(),|_,_|{if let Err(cause)=control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0){if progress_refusal.is_none(){progress_refusal=Some(cause)}cancel.cancel();}},&cancel,grant,&mut identity)).map_err(|error|progress_refusal.take().map(IoError::from_value_error).unwrap_or_else(||guest_sqlite_turn_error(error)))
            })?);
            match frame.wire.as_mut().unwrap(){
                sqlite_wire::SnapshotPayloadResult::Rejected(rejection)=>{
                    frame.diagnostics=sqlite_wire::decode_diagnostics(&rejection.diagnostics).map_err(IoError::from_value_error)?;
                    Err(IoError{cause:ValueError::new(rejection.kind,std::mem::take(&mut rejection.message)),diagnostics:std::mem::take(&mut frame.diagnostics)})
                },
                sqlite_wire::SnapshotPayloadResult::Pending(_)=>Err(IoError::from_value_error(ValueError::literal(ValueRefusalKind::OwnershipLimit,"guest SQLite import retains its original component ticket"))),
                sqlite_wire::SnapshotPayloadResult::Done(payload)=>{
                    if payload.encoding.as_str()!=encoding.as_str(){return Err(IoError::from_value_error(ValueError::literal(ValueRefusalKind::InvalidValue,"guest SQLite import returned a different native encoding")))}
                    if matches!(encoding,sqlite_snapshot::SnapshotEncoding::Text)&&std::str::from_utf8(&payload.bytes).is_err(){return Err(IoError::from_value_error(ValueError::literal(ValueRefusalKind::InvalidValue,"guest SQLite import returned invalid UTF8")))}
                    frame.diagnostics=sqlite_wire::decode_diagnostics(&payload.diagnostics).map_err(IoError::from_value_error)?;
                    let bytes=std::mem::take(&mut payload.bytes);
                    let value=match encoding{sqlite_snapshot::SnapshotEncoding::Binary=>IoPayload::Binary(bytes),sqlite_snapshot::SnapshotEncoding::Text=>IoPayload::Text(String::from_utf8(bytes).unwrap())};
                    Ok(IoOutcome{value,diagnostics:std::mem::take(&mut frame.diagnostics)})
                },
            }
        })())
    }).map_err(IoError::from_value_error)?
}

/// 🗂️ Registers `artifact_schema`'s document codec from the component's OWN `codec` export, and
/// answers the pack-schema hash the component itself computed.
///
/// 🪪️ `expected_pack_schema_hash` is the hub's declared hex hash for this document, and it is a
/// CROSS-CHECK, never the source: the hash that goes into the registry is the one
/// `codec.pack-schema-hash` returned from the component bytes the hub authorized and this process
/// verified by SHA-256. Registering the hub's own number would make `finish_connect_hub`'s
/// `local_schema_hash == authority.pack_schema_hash` compare the hub's value with itself, which is
/// not satisfying that check — it is deleting it.
///
/// 🔁️ Idempotent per schema: a second registration of a schema this process already serves is a
/// no-op that still verifies the hash, so re-opening a document never stacks routes.
///
/// 📦️ The generic pack container extension. A guest-backed codec never reaches the folder
/// text lane (see [`guest_compile_dsl`]), which is the only place this is read.
#[cfg(not(target_arch = "wasm32"))]
pub fn register_guest_document_codec(plugin_id: &str, artifact_schema: &str, dialect: semio_framework_artifact_reference::ArtifactDialect, component: &[u8], expected_pack_schema_hash: &str) -> Result<[u8; 32], GatewayError> {
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

    let runtime = shared_plugin_runtime()?;
    let compiled = shared_compiled_component(runtime.as_ref(), plugin_id, component)?;
    let pack_schema_hash = semio_framework_async::block_on(runtime.codec_pack_schema_hash(&compiled, artifact_schema, &headless_codec_budget()))
        .map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("`{plugin_id}` could not answer codec.pack-schema-hash for `{artifact_schema}`: {error}")))?;
    let computed = pack_schema_hash.iter().map(|byte| format!("{byte:02x}")).collect::<String>();
    if !expected_pack_schema_hash.is_empty() && computed != expected_pack_schema_hash {
        return Err(GatewayError::new(
            GatewayErrorCode::PreconditionFailed,
            format!("`{plugin_id}`'s own codec hashes `{artifact_schema}` to {computed}, but the hub's document descriptor declares {expected_pack_schema_hash}"),
        ));
    }
    let sqlite_schema = semio_framework_async::block_on(runtime.codec_sqlite_schema(&compiled, &dialect.to_coordinate(), &headless_codec_budget())).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("guest semantic SQLite schema: {error}")))?;
    let mut routes = guest_codec_routes().lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some(route) = routes.iter().find(|route| route.artifact_schema == artifact_schema && route.dialect == dialect) {
        if route.plugin_id != plugin_id || route.compiled.package_hash != compiled.package_hash { return Err(GatewayError::new(GatewayErrorCode::PreconditionFailed, "an exact guest snapshot route is already owned by another plugin or component")); }
        return Ok(pack_schema_hash);
    }
    let codec = store::os_store::serialization::ArtifactCodec {
        schema: artifact_schema.to_string(),
        extension: "semio",
        native_identity: store::os_store::serialization::ArtifactNativeSnapshotIdentity::Guest { plugin_id: plugin_id.to_string(), package_hash: compiled.package_hash, schema: artifact_schema.to_string() },
        snapshot_sqlite: Some(store::os_store::serialization::ArtifactSqliteSnapshotCodec { schema: std::borrow::Cow::Owned(sqlite_schema), snapshot_type: None, export: guest_sqlite_export, import: guest_sqlite_import,export_file:None,import_file:None }),
        pack_schema_hash,
        reference:Some(store::os_store::serialization::ArtifactCodecReference{
        compile_dsl: guest_compile_dsl,
        print_mirror: guest_print_mirror,
        edit_text_from_envelope: guest_edit_text_from_envelope,
        apply_ops_binary: guest_apply_ops_binary,
        replay_envelopes: guest_replay_envelopes,
        }),
    };
    let assembly = semio_framework_schema_registry::assembly::begin().map_err(|error| GatewayError::new(GatewayErrorCode::Internal, error.to_string()))?;
    let binding=semio_framework_os_kernel::io::ArtifactCodecBinding::from_codec(semio_framework_os_kernel::io::ArtifactCodecBindingChannel::Guest,plugin_id,Some(dialect.artifact_kind.clone()),Vec::new(),Some(dialect.clone()),None,&codec);
    semio_framework_os_kernel::io::commit_artifact_assembly_registry_plan(&assembly, semio_framework_os_kernel::io::ArtifactAssemblyRegistryPlan { document_bindings:vec![binding],document_codecs: vec![codec.clone()], native_snapshots: vec![semio_framework_os_kernel::io::io_mechanism::NativeSnapshotRegistration { dialect: dialect.clone(), codec }], ..Default::default() }).map_err(|error| GatewayError::new(GatewayErrorCode::Internal, format!("registering a guest-backed semantic codec for `{artifact_schema}`: {error}")))?;
    routes.push(Arc::new(GuestCodecRoute { dialect, artifact_schema: artifact_schema.to_string(), plugin_id: plugin_id.to_string(), runtime: Arc::clone(&runtime), compiled }));
    Ok(pack_schema_hash)
}

//#endregion 🗂️GuestDocumentCodec
