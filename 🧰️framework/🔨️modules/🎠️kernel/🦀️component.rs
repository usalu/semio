//! 🧠️ Local-first action kernel contracts: actions, operations, capabilities, window I/O.

use serde::{Deserialize, Serialize};
pub use dsl::{Diagnostic, Fault, FaultCause, FaultCode, FaultFrom, FaultOrigin, FaultScope, Severity};
use dsl::DslValue;
use ui_wgpu::wgpu::UiNode;
use crate::manifest::MediaType;

//#region 🔖️Identifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ArtifactHandle(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WindowHandle(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AssetHandle(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CapabilityToken(pub u128);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PluginInstanceId(pub String);

// 🎞️ CW3 kernel cut-over: MutationId/ActorId/ArtifactId/ArtifactVersion/SchemaId moved to
// `protocol_core` (frozen contract `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md`),
// re-exported here under their original names — shapes are unchanged (plain serde-transparent
// String/u64 newtypes), so every existing reference (internal `kernel` types below, and external
// crates like `framework/sync`/`framework/product/os/semio_hub` that import them straight from
// `semio_framework`) keeps resolving without edits. `SchemaVersion` below is NOT re-exported
// from `protocol_core` — that crate's own `SchemaVersion` is `u32`-shaped (a distinct, unrelated
// protocol-format concept), incompatible with this kernel's `String`-shaped version below, which
// several external crates (`framework/sync`, semio_hub storage crates) still construct from plain
// strings; moving it would be a breaking shape change out of this wave's scope.
pub use protocol_core::{ActorId, ArtifactId, ArtifactVersion, MutationId, SchemaId};

/// 🪪️ Identifies one dispatched invocation — of an action *or* a command; both route through the same
/// `KernelMutation`/`UndoGroup` history bookkeeping.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InvocationId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ActionId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CommandId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AppInstanceId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaVersion(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WindowKindId(pub String);
//#endregion 🔖️Identifiers

//#region 🔖️HybridLogicalTimestamp
// 🎯️ W6 kernel unification: re-exports `protocol_core::HybridLogicalTimestamp` now (identical
// `{actor, physical_ms, logical}` shape, plus a real actor-tiebroken `Ord`/`PartialOrd` the old
// local struct lacked). The CW3-era deferral note this region used to carry (kept local because
// this struct's `#[serde(rename_all = "camelCase")]` put `physicalMs` on the wire, vs.
// `protocol_core`'s unrenamed `physical_ms`) is resolved: this wave already rewired the JSON/wire
// boundary end-to-end (W5's binary `protocol_wire` codec, its TS twin, and the fixture
// byte-identity canary all speak `physical_ms`), so the wire-format reconciliation this note
// deferred is verified, not assumed.
pub use protocol_core::HybridLogicalTimestamp;
//#endregion 🔖️HybridLogicalTimestamp

//#region 🔖️Capability
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum Rights {
    Read,
    Write,
    Invoke,
    Open,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum ArtifactKind {
    Document,
    Projection,
    Window,
    Asset,
    Network,
    Backbone,
    Engine,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum Scope {
    Instance,
    App,
    Plugin,
    Global,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct CapabilityRequirement {
    pub artifact: ArtifactKind,
    pub rights: Rights,
    pub scope: Scope,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capability {
    pub subject: PluginInstanceId,
    pub artifact: ArtifactId,
    pub rights: Rights,
    pub scope: Scope,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CapabilityGrant {
    pub token: CapabilityToken,
    pub capability: Capability,
}
//#endregion 🔖️Capability

//#region 🔖️Invocation
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionDef {
    pub id: ActionId,
    pub input_schema: SchemaId,
    pub output_schema: SchemaId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub required_capabilities: Vec<CapabilityRequirement>,
    pub deterministic: bool,
    pub produces_operations: bool,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionInvocation {
    pub id: InvocationId,
    pub app: AppInstanceId,
    pub action: ActionId,
    pub input: DslValue,
    pub actor: ActorId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub causal_context: Vec<MutationId>,
}

/// @emoji 🎛️ A dispatched invocation of a `CommandDefinition` — the command mirror of `ActionInvocation`.
/// No `causal_context`: commands are not chained off a prior operation the way an action's follow-up can be.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandInvocation {
    pub id: InvocationId,
    pub app: AppInstanceId,
    pub command: CommandId,
    pub input: DslValue,
    pub actor: ActorId,
}

/// 📋️ How a paste anchors the copied fragment relative to the paste point — the seven placement
/// modes semio_compose_rs's `copyDesign`/`pasteDesign` supported, now an OS-owned concept.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum PasteAnchor {
    #[default]
    Original,
    Middle,
    Centroid,
    BottomLeft,
    BottomRight,
    TopLeft,
    TopRight,
}

/// 📋️ Where/how a paste places its fragment: `anchor` picks the reference point, `position` (when
/// given) overrides where that reference point lands.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct PastePlacement {
    #[serde(default)]
    pub anchor: PasteAnchor,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub position: Option<[f64; 3]>,
}

/// 📋️ A copied document fragment: `dsl_text` is the human-readable/`text/plain`-fallback encoding
/// (printed via the source app's own `ArtifactDsl` grammar over a fragment-shaped projection),
/// `pack_bytes` is the lossless binary lane for same-app/compatible paste. `media_type` is the
/// cross-app compatibility key (see `media_types_compatible`) an app's `clipboard_accepts()` checks
/// before offering to paste a fragment copied from a different app.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ClipboardFragment {
    pub schema: String,
    pub media_type: MediaType,
    pub dsl_text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
    pub pack_bytes: Option<Vec<u8>>,
    pub source_app: String,
    pub label: String,
}

/// 🧯️ Clipboard operation failures — an app's `copy_fragment`/`paste_operations` return these instead
/// of panicking on an empty selection or an incompatible fragment.
#[derive(Debug, thiserror::Error)]
pub enum ClipboardError {
    #[error("nothing selected to copy")]
    EmptySelection,
    #[error("clipboard fragment media type {0:?} not accepted by this app")]
    IncompatibleMediaType(MediaType),
    #[error("clipboard fragment failed to parse: {0}")]
    ParseFailed(String),
}
//#endregion 🔖️Clipboard

//#region 🔖️Effect
/// 🎫️ Correlates an `Effect` that expects a completion with the `Event::Completed` (or
/// `Event::HttpChunk`/`JobProgress`/`JobCompleted`) that answers it — `request-id = u64` in
/// `📜️wit/📜️types.wit`. Minted host-side per pending request; the guest SDK's request registry
/// (`📓️design-abi.md` §4) parks a future on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RequestId(pub u64);

// 🪪️ `rename_all` on an enum only renames variant tags ("setActiveUtility"), not the fields *inside* each
// struct-variant — those need `rename_all_fields` (serde 1.0.126+) or every multi-word field here
// (window_kind_id, mime_type, plugin_id, ...) silently serializes as snake_case, breaking any TS side
// that destructures camelCase (confirmed live: `SetActiveUtility` was shipping `window_kind_id`/`utility_id`,
// so the host-owned utility switch after `openVortexSuggestions` never applied and the brush preview never
// rendered).
/// 🐚️ A typed side effect the guest emits toward the host — `📓️design-abi.md` §2, replacing
/// `HostEffect` now that plugins and extensions share one `actor` world. Every variant that
/// existed as `HostEffect` keeps its exact name and fields (mechanical `HostEffect` → `Effect`
/// rename at every call site); six of them (`OpenWindow`, `RequestFileOpen`, `RequestMediaFrames`,
/// `SpawnPluginInstance`, `OpenDialog`, `DispatchAction`) additionally gain a `req: RequestId` now
/// that they complete; `InvokeExtension` loses `response_action` and gains `req` (the SDK resumes
/// the awaiting future instead of a redispatch). The rest are new: messaging, blobs, documents,
/// links, registry lookups, io composition, engine caches, jobs, storage, capability admin, and
/// pub/sub — see `📓️design-abi.md` §2's table.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Effect {
    OpenWindow { req: RequestId, kind: WindowKindId, params: DslValue },
    CloseWindow { window: WindowHandle },
    Notify { message: String },
    /// 📋️ Asks the shell to write a copied/cut fragment to the OS clipboard (system clipboard where
    /// available, session-local fallback otherwise) — emitted by `VcsArtifactApp`'s `copy`/`cut`
    /// interception, never constructed by an app directly.
    ClipboardWrite { fragment: ClipboardFragment },
    RequestSync,
    /// @emoji 🧭️ Navigates the shell to a URI (studio/instance/document route).
    Navigate { uri: String },
    /// @emoji 📂️ Replaces the active app instance's document with pack+spr bytes — the host-owned
    /// counterpart of `loadAppArtifactPack`, used when the plugin resolves a catalog/example studio
    /// and needs the shell to swap the live store without going through a persistence binding.
    LoadDocument { pack: Vec<u8>, spr: Vec<u8> },
    /// @emoji 🌐️ Opens an external URL in a new browser tab — the host-bridge substitute for a program
    /// reaching into `web-sys`/`window()` directly, which the plugin capability lint forbids.
    OpenExternalUrl { url: String },
    /// @emoji 🗂️ Replaces the active studio/window panel state with a serialized panel JSON.
    SetPanel { panel_json: String },
    /// @emoji ⬇️ Downloads an in-memory media export as a file (base64 or utf-8 `data`).
    DownloadMediaExport {
        filename: String,
        mime_type: String,
        data: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        encoding: Option<String>,
    },
    /// @emoji 🖼️ Renders one or more icon-scene requests to images and downloads each.
    IconRenderExport { items: Vec<IconRenderExportItem> },
    /// @emoji 📤️ Asks the shell to open a file picker and re-dispatch `import_action` with the
    /// picked file's contents as `{ payload, name }` args. When `multiple` is set, the picker allows
    /// selecting several files and `import_action` is re-dispatched once per file, sequentially, each
    /// call extending the args with `{ index, total }`.
    RequestFileOpen {
        req: RequestId,
        accept: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        read_as: Option<String>,
        import_action: String,
        #[serde(default)]
        multiple: bool,
    },
    /// @emoji 🎞️ Asks the shell to decode a video (via file picker, or `payload` bytes when the
    /// caller already has them, e.g. a drop zone) and re-dispatch `frame_action` once per sampled
    /// frame with `{payload: dataUrl(image/jpeg), name, frameIndex, timestampMs, index, total, width,
    /// height, ...args}`, then `done_action` once with `{name, durationMs, frameCount, sampledCount,
    /// width, height, codec, ...args}`. `sample_stride`/`max_frames`/`max_long_edge_px`/`fps_hint` are
    /// hints only (0 = host default); a host that can't decode the codec dispatches `fallback_action`
    /// once instead, with `{payload: dataUrl(raw container bytes), name, ...args}` — mirrors
    /// `RequestFileOpen`'s per-file re-dispatch shape but fans out video frames instead of files.
    RequestMediaFrames {
        req: RequestId,
        accept: String,
        frame_action: String,
        done_action: String,
        fallback_action: String,
        #[serde(default)]
        sample_stride: u32,
        #[serde(default)]
        max_frames: u32,
        #[serde(default)]
        max_long_edge_px: u32,
        #[serde(default)]
        fps_hint: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        payload: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
    },
    /// @emoji ✨️ Spawns a plugin instance (idempotent on `os_instance_id`) without focusing it.
    SpawnPluginInstance {
        req: RequestId,
        plugin_id: String,
        app_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        os_instance_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        document_json: Option<String>,
    },
    /// @emoji 🪟️ Spawns (if needed) and focuses/navigates to a plugin instance.
    OpenPluginInstance {
        plugin_id: String,
        app_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        os_instance_id: Option<String>,
    },
    /// @emoji 🧰️ Programmatically switches the host-owned active utility of a window instance — the effect
    /// form of `setActiveUtility`, letting a plugin change utilities without a user click.
    SetActiveUtility { window_id: String, utility_id: String },
    /// @emoji 🛠️ Programmatically switches the host-owned active tool of the active mode — the effect
    /// form of `setActiveTool`, letting a plugin change tools without a user click. Empty `tool_id`
    /// deactivates the current tool.
    SetActiveTool { tool_id: String },
    /// @emoji 🗨️ Opens a declared `AppDefinition.dialogs` entry; `args` (an object keyed by arg id)
    /// pre-seeds the staged form. Kernel-altitude — plain `String`/`Value`, no manifest types.
    OpenDialog {
        req: RequestId,
        dialog_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
    },
    /// @emoji 🔁️ Re-dispatches `action` onto the same plugin instance after `delay_ms` — lets a
    /// plugin's `handle_action` advance staged/progressive work (e.g. a multi-pass reconstruction)
    /// over several ticks without blocking the host. The host feeds the follow-up response's own
    /// `requestedEffects` back through the same effect-application pass, so a `DispatchAction` can
    /// itself emit another one, chaining as many ticks as the plugin needs.
    DispatchAction {
        req: RequestId,
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
        delay_ms: u64,
    },
    /// @emoji ⏪️ Asks the shell to redispatch a shell-owned command (dock/theme/locale/panel chrome)
    /// whose real mutation and its inverse both live client-side — the plugin has no access to that
    /// state, so `revertToCommand` on a `Shell`-kind history row bubbles the row's stored inverse out
    /// here instead of replaying it internally the way a `View`-kind row does (see
    /// `NOTE_SHELL_COMMAND_ACTION_ID` and `VcsArtifactApp::dispatch_action`'s `REVERT_TO_COMMAND_ACTION_ID`
    /// arm). The shell is expected to redispatch `action_id`/`args` through its normal command funnel,
    /// which itself calls `noteShellCommand` again — so the revert is itself a new, further-revertible row.
    ReplayShellCommand {
        action_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
    },
    /// @emoji 🎯️ Patches live world-3d selection chrome and document-tree highlights without
    /// re-rendering the composite window body or rebuilding instance geometry JSON.
    PatchWorld3dChrome {
        selection_json: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        vortices_json: Option<String>,
        document_selected_ids: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        document_highlighted_ids: Option<Vec<String>>,
    },
    /// @emoji 🔁️ Asks the shell to invoke an extension capability — the SDK resumes the awaiting
    /// future on `Event::Completed { req, .. }` instead of a `response_action` redispatch.
    InvokeExtension {
        req: RequestId,
        extension_id: String,
        capability: String,
        request_json: String,
    },

    // --- new variants (📓️design-abi.md §2's table; nothing constructs these yet) ---
    /// @emoji 📨️ Replaces every non-UI/non-event `AppFrame::*` plus `backbone-send` — `target`
    /// picks shell vs. backbone vs. a specific plugin/extension/topic.
    SendMessage { target: MessageEndpoint, payload: Vec<u8> },
    /// @emoji 📣️ Replaces `AppFrame::Events` — a pub/sub broadcast, not a directed message.
    PublishEvent { topic: String, payload: Vec<u8> },
    BlobWrite { req: RequestId, media_type: MediaType, bytes: Vec<u8> },
    /// @emoji 📥️ Also answers a lazy `read-asset` miss (assets are preloaded in
    /// `Event::InstanceOpen.assets`; this is the fallback for one that wasn't).
    BlobLoad { req: RequestId, hash: String },
    HttpRequest {
        req: RequestId,
        method: String,
        url: String,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        headers: Vec<(String, String)>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        body: Option<Vec<u8>>,
        #[serde(default)]
        stream: bool,
    },
    DocumentRead { req: RequestId, doc: ArtifactHandle, lane: String },
    DocumentWrite { req: RequestId, doc: ArtifactHandle, lane: String, ops: Vec<u8> },
    LinkResolve { req: RequestId, link: String },
    /// @emoji 🔍️ On-demand io-dialect lookup — the routing table itself is preloaded in
    /// `Event::InstanceOpen`; this is only for entries that weren't.
    RegistryQuery {
        req: RequestId,
        kind: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        filter: Option<DslValue>,
    },
    /// @emoji 🧵️ Routed by the host `IoRouter` to the owning plugin as an `Event::Request` — one
    /// hop, no re-entrancy.
    IoCompose { req: RequestId, key: String, sources: Vec<String> },
    CacheDerive { req: RequestId, engine_id: String, input: Vec<u8> },
    CacheRead { req: RequestId, engine_id: String, key: String },
    /// @emoji ⏱️ Replaces self-tick loops and `pending_effects()` polling — the host wakes the
    /// instance with `Event::Timer { id }` after `after_ms`, repeating if `repeat` is set.
    SetTimer { id: u64, after_ms: u64, #[serde(default)] repeat: bool },
    SpawnJob { job: u64, kind: String, input: Vec<u8>, placement: JobPlacement },
    CancelJob { job: u64 },
    /// @emoji ↩️ Answers an inbound `Event::Request { req, .. }` within a bounded number of turns.
    Respond { req: RequestId, result: RequestOutcome },
    StorageRead { req: RequestId, key: String },
    StorageWrite { req: RequestId, key: String, bytes: Vec<u8> },
    StorageDelete { req: RequestId, key: String },
    RequestCapability { req: RequestId, capability: CapabilityRequest },
    ReleaseCapability { id: CapabilityId },
    /// @emoji 📡️ Replaces `backbone-poll`/`backbone-status` — inbound traffic on `topic` arrives
    /// as `Event::Message`.
    Subscribe { topic: String },
    Unsubscribe { topic: String },
}

/// 🚦 Where a spawned job runs — `📓️design-abi.md` §2's `spawn-job.placement`: `Inline` shares
/// the instance's own turn budget, `Isolated` gets its own pooled actor, `Exclusive` gets a
/// dedicated one (e.g. flow/brep tessellation, per `📓️design-abi.md` §5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum JobPlacement {
    Inline,
    Isolated,
    Exclusive,
}

/// @emoji 🖼️ One icon-render export request: the destination filename plus the opaque icon-scene
/// render request forwarded to the shell's `iconRenderPort`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IconRenderExportItem {
    pub filename: String,
    pub request: DslValue,
}
//#endregion 🔖️Effect

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppEvent {
    pub kind: String,
    pub payload: DslValue,
}

// 🎯️ W6 kernel unification: re-exports `protocol::ArtifactDiff` (schema: `SchemaId`, payload:
// `Vec<u8>` — the binary shape from W5's causal envelope reshape) in place of the old kernel-local
// `{schema_id, payload: Value}` shape. Zero external consumers of the old shape existed outside
// this crate's own (now-deleted) OS JSON-patch kernel and `store`/`store_sync` (both repointed to
// `protocol::ArtifactDiff` directly in this same wave) — verified by a repo-wide grep before this
// change, not assumed.
pub use protocol::ArtifactDiff;

// 🎯️ W6 kernel unification: re-exports `protocol_core::UndoPolicy` (identical variants; the old
// CW3-era deferral note about a `#[serde(rename_all = "camelCase")]` mismatch no longer applies —
// see `HybridLogicalTimestamp`'s doc above for the same reconciliation).
pub use protocol_core::UndoPolicy;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseMutation {
    pub target_mutation: MutationId,
    pub inverse_diff: ArtifactDiff,
    pub base_version: ArtifactVersion,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<MutationId>,
    pub undo_policy: UndoPolicy,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KernelMutation {
    pub id: MutationId,
    pub document: ArtifactHandle,
    pub base_version: ArtifactVersion,
    pub invocation_id: InvocationId,
    pub diff: ArtifactDiff,
    pub inverse: InverseMutation,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<MutationId>,
    pub author: ActorId,
    pub timestamp: HybridLogicalTimestamp,
}

/// 🧩️ One member edit folded into a group undo — pairs the owning document handle with the edit
/// id inside it, so `UndoGroup.member_edits` can name edits that live on documents other than the
/// group's own `invocation_id` target (composite/child-document dispatch, ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM `📓️design-full-plan.md` section "1. Kernel
/// primitives" — grouping). Additive only: nothing in this wave constructs one yet.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditRef {
    pub document: ArtifactHandle,
    pub edit_id: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UndoGroup {
    pub invocation_id: InvocationId,
    pub mutations: Vec<MutationId>,
    pub inverse_mutations: Vec<InverseMutation>,
    /// 🧩️ Cross-document member edits folded into this group's undo (composite dispatch across
    /// parent + child documents) — additive, empty for every group that isn't composite.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub member_edits: Vec<EditRef>,
}

/// @emoji 🐢️ What part of the shell's rendered UI an action actually invalidates — lets `refresh-ui`
/// skip re-rendering/re-fetching sections nothing touched. Absent from JSON (older/unmodified plugins)
/// deserializes to `Full`, so any program that never sets this keeps today's whole-shell-refresh
/// behavior exactly. `None` means "nothing to re-render at all" (e.g. a pure telemetry/heartbeat action).
// 🐢️ `rename_all = "camelCase"` alone only renames the *variant* names (Full/None/Partial ->
// full/none/partial via `tag = "kind"`) — it does NOT cascade into a struct variant's own fields, which
// would otherwise serialize as snake_case (`window_bodies`) and silently desync from the TS
// `UiDirtyScope` type's camelCase `windowBodies`. `rename_all_fields` is the attribute that renames
// fields *within* variants; both are needed together.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum UiDirtyScope {
    #[default]
    Full,
    None,
    Partial {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        window_bodies: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        panel_bodies: Vec<String>,
        #[serde(default)]
        utilities: bool,
        #[serde(default)]
        tools: bool,
        #[serde(default)]
        engagements: bool,
        #[serde(default)]
        measures: bool,
        #[serde(default)]
        labels: bool,
    }
}

/// 🧾️ One host-projectable row in the session command timeline. The payload is deliberately
/// presentation-neutral: the host owns windowing and retains entries beyond any visible range.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryEntry {
    pub seq: u64,
    pub action_id: String,
    pub label: String,
    pub kind: String,
    pub timestamp: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub op_lines: Vec<String>,
    #[serde(default)]
    pub applied: bool,
    #[serde(default)]
    pub revertible: bool,
    #[serde(default = "history_entry_count")]
    pub count: u32,
}

// 🚫️async: E4 fn-pointer slot
fn history_entry_count() -> u32 {
    1
}

/// 🧾️ Ordered history delta returned in the same response as an accepted interaction.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoryPatch {
    /// Monotonic command-log cursor after applying this patch.
    pub cursor: u64,
    /// Upserts, ordered newest-first to match the logical history projection.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub upserts: Vec<HistoryEntry>,
    #[serde(default)]
    pub can_undo: bool,
    #[serde(default)]
    pub can_redo: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_checkpoint_id: Option<String>,
    #[serde(default)]
    pub command_filter: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InvocationResult {
    pub output: DslValue,
    pub mutations: Vec<KernelMutation>,
    pub inverse_group: UndoGroup,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_effects: Vec<Effect>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<AppEvent>,
    #[serde(default)]
    pub ui_scope: UiDirtyScope,
    /// 🧾️ Incremental command-history delivery. It is independent from `ui_scope`: history must
    /// become visible before effects or an unrelated UI refresh can be queued.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub history_patch: Option<HistoryPatch>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionContext {
    pub invocation: ActionInvocation,
    pub document_snapshot: DslValue,
    pub view_state: super::ViewModel,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub granted_capabilities: Vec<CapabilityGrant>,
}

/// @emoji 🎛️ Context for a dispatched `CommandInvocation` — the command mirror of `ActionContext`.
/// No `document_snapshot`/`granted_capabilities`: `VcsArtifactApp` owns the store directly and
/// commands don't yet carry a capability grant model (mirrors actions' current state).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandContext {
    pub invocation: CommandInvocation,
    pub view_state: super::ViewModel,
}
//#endregion 🔖️Invocation

//#region 🔖️Presence
pub use semio_framework_os_kernel::{PresencePeer, PresenceUi, PresenceViewKind, PresenceWindowView, decode_presence_peer, encode_presence_peer};
//#endregion 🔖️Presence

//#region 🔖️Window
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PhysicalSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Appearance {
    pub mode: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowEvent {
    pub kind: String,
    pub payload: DslValue,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionRequest {
    pub invocation: ActionInvocation,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowKindDef {
    pub id: WindowKindId,
    pub params_schema: SchemaId,
    pub artifact_snapshot_schema: SchemaId,
    pub input_event_schema: SchemaId,
    pub output_schema: SchemaId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<CapabilityRequirement>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowInput {
    pub window: WindowHandle,
    pub params: DslValue,
    pub document_snapshot: DslValue,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<WindowEvent>,
    pub size: PhysicalSize,
    pub scale_factor: f64,
    pub appearance: Appearance,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WindowOutput {
    pub ui: UiNode,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<ActionRequest>,
}
//#endregion 🔖️Window

//#region 🔖️MergeStrategy
// 🎞️ `26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS` C10/W0: the CRDT-era
// per-artifact-kind merge projection is gone — CLAUDE.md forbids CRDTs and the projection had no
// remaining callers; merge behavior is now the single repo-wide `MergePolicy` setting (C3).
// `ArtifactMergeKind` itself stays: it is still a real artifact-kind tag, just without that reading.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ArtifactMergeKind {
    PlainRecord,
    OrderedSequence,
    TextSequence,
    TombstonedGraph,
    ContentAddressedBlob,
}
//#endregion 🔖️MergeStrategy

//#region 🔖️Event
/// 📨️ Who a `Event::Message` came from / an `Effect::SendMessage` targets — `📓️design-abi.md`
/// §2. This single shape replaces `backbone-poll`, the `DocumentChanged` push, `InvokeExtension`
/// replies, and topic subscriptions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum MessageEndpoint {
    Shell { instance: PluginInstanceId },
    Backbone { uri: String },
    PluginInstance { id: PluginInstanceId },
    Extension { id: String },
    Topic { name: String },
}

/// ✅️ The shared `result<pack, fault-bytes>` shape from `📜️wit/📜️types.wit`, carried by
/// `Event::Completed`/`Event::JobCompleted` and `Effect::Respond`. `Err` bytes are an encoded
/// fault the SDK decodes by originating request kind — the host never interprets it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RequestOutcome {
    Ok(Vec<u8>),
    Err(Vec<u8>),
}

/// 📨️ Everything the host delivers into a guest's `reactor::poll` — the full inbound contract
/// from `📓️design-abi.md` §2. Lifecycle events open/close/activate/suspend an instance and push
/// capability/quota changes; channel/surface/completion/messaging/timer/request events drive a
/// turn. Nothing constructs one yet — additive, packet A2-abi-sdk's executor is the first reader.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum Event {
    /// 🐣️ First event an instance receives — config/assets/capabilities/quotas are preloaded so
    /// the first `poll` never blocks. `actor` is a placeholder `String` until the concurrently
    /// landing `🎭️actor` crate's `RuntimeActorId` exists (this packet must not depend on it —
    /// see the report's `🎭️actor` naming-hazard note).
    InstanceOpen {
        instance: PluginInstanceId,
        app_id: AppInstanceId,
        actor: String,
        config: Vec<u8>,
        assets: Vec<(String, Vec<u8>)>,
        capabilities: Vec<BrokerCapabilityGrant>,
        quotas: QuotaSchema,
    },
    InstanceClose,
    Activate { reason: ActivationEvent },
    SuspendRequest,
    CapabilityChanged { change: CapabilityChange },
    QuotaChanged { quotas: QuotaSchema },

    /// 📡️ The `exchange(id, cmds)` → `poll([app-command{id,seq,cmd}…], budget)` collapse
    /// (`📓️design-abi.md` §2 "`exchange` collapse") — routes to the existing `PluginApp` dispatch
    /// unchanged.
    AppCommandEvent { instance: PluginInstanceId, seq: u64, command: Vec<u8> },

    /// 🎬️ `wit-flip` (26/08/20) — a user action against a UI node, `pack`-encoded
    /// `semio_framework_ui_contract::UiIntent`. Split out of `AppCommandEvent` so the host can
    /// tell a genuine UI interaction from a channel command without decoding the payload —
    /// `component.wit`'s `events::ui-intent-event`.
    UiIntent { instance: PluginInstanceId, intent: Vec<u8> },

    SurfaceVisible { surface: String },
    SurfaceHidden { surface: String },
    SurfaceResized { surface: String, width: u32, height: u32 },
    PatchAck { surface: String, revision: u64 },
    /// 🩹️ Guest resends a full patch body (not a diff) on rejection — `revision`/`reason` name
    /// what the host couldn't apply.
    PatchRejected { surface: String, revision: u64, reason: String },

    Completed { req: RequestId, result: RequestOutcome },
    HttpChunk { req: RequestId, bytes: Vec<u8>, done: bool },
    JobProgress {
        job: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        progress: Option<Vec<u8>>,
    },
    JobCompleted { job: u64, result: RequestOutcome },

    Message { source: MessageEndpoint, payload: Vec<u8> },

    Timer { id: u64 },
    Wake,

    /// ↩️ The former `extension.invoke`/`artifact-compose`/`io-run`/`io-sniff`/`artifact-infer`/
    /// `artifact-mutation-plan`/`migrate-artifact` — answered with `Effect::Respond` within a
    /// bounded number of turns, or by spawning a job.
    Request { req: RequestId, from: MessageEndpoint, capability: String, payload: Vec<u8> },
}
//#endregion 🔖️Event

//#region 🔖️ActivationEvent
/// 🚀️ Why an instance was activated — `📓️design-abi.md` §2's activation-event list, matched
/// against a `manifest::PackageDescriptor.activation_events` declaration at install time.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum ActivationEvent {
    OnCommand { id: String },
    OnViewVisible { id: String },
    OnFileType { ext: String },
    OnArtifactKind { kind: String },
    OnExtensionRequest { point: String },
    OnStartupFinished,
}
//#endregion 🔖️ActivationEvent

//#region 🔖️UiPatch
/// 🩹️ `wit-flip` (26/08/20): re-exported from `semio-framework-ui-contract`, the language-neutral
/// contract crate's own `UiPatch`/`UiPatchOp` (`🦀️document.rs`) — this file no longer declares its
/// own copy, so there is exactly one definition to keep in sync with `component.wit`'s
/// `ui-patch`/`patch-op` (node-id addressed, not path addressed) rather than two that could drift.
/// Requires a `semio-framework-ui-contract` dependency on every crate that `#[path]`-mounts this
/// file — see this packet's report for the exact registrar-request lines (this crate is not on the
/// registrar-only list for `Cargo.toml`, so the dependency itself is not added here).
pub use semio_framework_ui_contract::{UiPatch, UiPatchOp};
//#endregion 🔖️UiPatch

//#region 🔖️Budget
/// ⛽️ Per-turn resource ceiling handed to `reactor::poll` — `📜️wit/📜️reactor.wit`'s `budget`
/// record.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    pub fuel: u64,
    pub deadline_ms: u32,
    pub max_effects: u32,
    pub max_patch_bytes: u32,
    pub max_frames: u32,
}
//#endregion 🔖️Budget

//#region 🔖️TurnResult
/// 🏁️ Outcome of one `reactor::poll` — `📜️wit/📜️reactor.wit`'s `turn-status` variant.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum TurnStatus {
    Idle,
    MoreWork,
    CheckpointReady,
    Faulted(Vec<u8>),
}

/// 📈️ What a turn actually cost — fed to `BrokerHooks::on_turn_finished` for quota accounting
/// against `QuotaSchema`'s per-turn fields.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Usage {
    pub fuel_used: u64,
    pub effects_emitted: u32,
    pub patch_bytes: u32,
    pub turn_ms: u32,
}

/// 🏁️ Result of one `reactor::poll` call — `📜️wit/📜️reactor.wit`'s `turn-result` record.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnResult {
    pub ui_patches: Vec<UiPatch>,
    pub effects: Vec<Effect>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_wake: Option<u64>,
    pub status: TurnStatus,
    pub fuel_used: u64,
}
//#endregion 🔖️TurnResult

//#region 🔖️Broker
/// 🔑️ A capability's identity — dotted/colon-scoped strings (`storage.read`, `http:<origin>`,
/// `messaging.plugin:<id>`, `extension.invoke:<id>`, ...) per `📓️design-abi.md` §5's catalogue.
/// A `String` newtype rather than a closed enum: several members carry a caller-chosen parameter
/// (`<origin>`/`<uri>`/`<id>`/`<point>`) the broker matches by prefix, and the catalogue is
/// expected to grow as new capability surfaces land — an exhaustive enum would need a matching
/// wildcard arm anyway.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(transparent)]
pub struct CapabilityId(pub String);

/// 🙏️ A guest's ask for a capability — `📓️design-abi.md` §5. Replaces `CapabilityRequirement`
/// for the plugin/extension actor runtime. The kernel-level `CapabilityRequirement`/`Rights`/
/// `Scope` action-dispatch model (above, `🔖️Capability` region) stays as-is: it has live
/// consumers outside this packet's owned paths (`🔌️plugin/🏗️builder`, `🔌️plugin/🖥️host`,
/// `🔌️plugin/🦀️component.rs`) — see this packet's report for the full consumer list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct CapabilityRequest {
    pub id: CapabilityId,
    pub scope: String,
    pub reason: String,
    #[serde(default)]
    pub optional: bool,
}

/// 🎟️ A broker-issued grant answering a `CapabilityRequest` — `📓️design-abi.md` §5.
/// Named `BrokerCapabilityGrant`, not the design prose's bare `CapabilityGrant`: this file
/// already has a `CapabilityGrant` (above, `🔖️Capability` region) for the unrelated kernel-level
/// action/window capability model (`ActionContext.granted_capabilities`), with live consumers
/// outside this packet's owned paths (`📦️packages/🦀️rust/📦️glue.rs`'s re-export list) — see the
/// report's naming-collision note.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct BrokerCapabilityGrant {
    /// 🔢️ `u128` has no JavaScript number equivalent, so the mirror carries it as a decimal string —
    /// same treatment `PluginDependency.version` gets in `🛂️manifest`.
    #[cfg_attr(feature = "typegen", ts(type = "string"))]
    pub token: CapabilityToken,
    pub id: CapabilityId,
    pub scope: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub expires_ms: Option<u64>,
}

/// 🔔️ A grant's lifecycle change, delivered as `Event::CapabilityChanged` — revocation
/// invalidates the guest's handle table so its next await on that capability returns
/// `Fault(capability-revoked)`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum CapabilityChange {
    Granted { id: CapabilityId, grant: BrokerCapabilityGrant },
    Revoked { id: CapabilityId },
    Narrowed { id: CapabilityId, grant: BrokerCapabilityGrant },
}

/// 📏️ One scope's resource ceiling — `📓️design-abi.md` §5. Every field is `Option`: `None`
/// inherits from the next scope up in a `QuotaTree` (os → plugin → extension → instance,
/// min-down). A plugin can sit inside its `memory_bytes` limit and still exhaust the host through
/// timers/UI nodes/requests/GPU allocations, which is why the schema is this wide.
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct QuotaSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub memory_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub fuel_per_turn: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub turn_deadline_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub tables: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub mailbox_len: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub message_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub outstanding_requests: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub timers: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub storage_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub network_bytes_per_min: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub ui_nodes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub patch_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub patch_hz: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub blob_resident_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub gpu_ms_per_frame: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub background_ms_per_min: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub log_bytes_per_min: Option<u64>,
}

/// 🌳️ Resolves a `QuotaSchema` for an instance by walking os → plugin → extension → instance,
/// min-down (`None` at any level defers to the next).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotaTree {
    pub os: QuotaSchema,
    pub plugin: QuotaSchema,
    pub extension: QuotaSchema,
    pub instance: QuotaSchema,
}

/// 💥️ One quota exceeded, fed to `BrokerHooks::on_breach`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuotaBreach {
    pub quota: String,
    pub limit: u64,
    pub actual: u64,
}

/// ⚖️ What the scheduler does about a `QuotaBreach` — `BrokerHooks::on_breach`'s return.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum FailureAction {
    Ignore,
    Throttle { after_ms: u64 },
    Suspend,
    Kill,
}

/// 🪝️ What the scheduler calls into around a turn — admission, accounting, breach policy, and
/// capability-change fan-out. `📓️design-abi.md` §5: "effective permissions = extension requests
/// ∩ the host plugin's extension-point allowance ∩ user approvals ∩ the host plugin's own
/// effective set" — `admit_effect` is where that intersection is enforced.
pub trait BrokerHooks {
    async fn admit_effect(&self, instance: &PluginInstanceId, effect: &Effect) -> Result<(), Fault>;
    async fn on_turn_finished(&self, instance: &PluginInstanceId, usage: &Usage);
    async fn on_breach(&self, instance: &PluginInstanceId, breach: &QuotaBreach) -> FailureAction;
    async fn on_capability_change(&self, instance: &PluginInstanceId, change: &CapabilityChange);
}
//#endregion 🔖️Broker

//#region 🔖️ExtensionActivation
/// 🧩️ Canonical installed-extension descriptor the host queries at plugin-activation time —
/// `extension-activation` packet (`📌️important.md`): "on plugin activation, the kernel queries
/// installed descriptors for `extends == plugin_id` and activates each as `ActorKind::Extension`,
/// pinned to the parent's shard, capabilities scoped to the parent". Deliberately independent of
/// the `.sxt` wire-format-specific `ExtensionPackageManifest` (`💻️os/🔨️modules/🧩️extension`, a
/// different crate's mount set than this file's) and of the guest-side `ExtensionManifest`
/// (`semio-framework-plugin`) — this shape uses only vocabulary this very file already owns
/// (`CapabilityId`/`CapabilityRequest`), so `extensions_extending` stays callable from every crate
/// this file reaches: it is `#[path]`-mounted (as `pub mod kernel`) into `🛂️manifest/🦀️component.rs`
/// alone, which is itself `#[path]`-mounted into THREE crates — `semio-framework` (root),
/// `semio-framework-graph`, and `semio-s-plugin-stdio` (verified: `grep -rn '#\[path.*🎠️kernel'`
/// and `grep -rn '#\[path.*🛂️manifest/🦀️component'`, both over absolute paths) — without pulling in
/// the `.sxt`/guest-SDK dependency edge. `💻️os/🖥️host`'s own install-region `InstalledExtension` is
/// the `.sxt`-shaped twin of this — see that type's docstring for why the two are NOT unified (same
/// dependency-edge-law reason `PackagePluginDependency`'s own docstring gives for its wire-shape
/// duplication).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ExtensionDescriptor {
    pub extension_id: String,
    /// 🔗️ The plugin id this extension extends — contract freeze §4's `extends`.
    pub extends: String,
    pub version: String,
    pub content_hash: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<CapabilityId>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub capability_requests: Vec<CapabilityRequest>,
}

/// 🔍️ Data-driven extends-query: every descriptor whose `extends` names `plugin_id`, order
/// preserved — the ONE call a plugin activation makes, whether zero, one, or the scale fixture's
/// 2,500 synthetic extensions are installed. No branch on `installed.len()` anywhere in this
/// function — the whole point of routing the scale fixture through identical code.
pub async fn extensions_extending<'a>(plugin_id: &str, installed: &'a [ExtensionDescriptor]) -> Vec<&'a ExtensionDescriptor> {
    installed.iter().filter(|descriptor| descriptor.extends == plugin_id).collect()
}

/// 🔒️ "capabilities scoped to the parent" — intersects an extension's own capability asks with
/// its parent plugin's already-effective set, so an extension actor can never end up holding a
/// capability its host plugin does not itself hold (`📓️design-abi.md` §5's admission formula,
/// the same intersection `BrokerHooks`'s own module doc names: "effective permissions = extension
/// requests ∩ the host plugin's extension-point allowance ∩ ..."). Order follows `requested`.
pub async fn scope_capabilities_to_parent(parent_effective: &[CapabilityId], requested: &[CapabilityId]) -> Vec<CapabilityId> {
    requested.iter().filter(|id| parent_effective.contains(id)).cloned().collect()
}

#[cfg(test)]
mod extension_activation_tests {
    use super::*;
    use std::future::Future;
    use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

    /// 🚫️async: E5 executor bridge — `extensions_extending`/`scope_capabilities_to_parent` are
    /// pure `Vec`/`String` work with zero suspension points, so they complete on the very first
    /// poll by construction; this hand-rolled poll-once bridge is the sanctioned E5 shape
    /// (`📌️important.md` R2/R4 clause 5 — a `#[test] fn` body is a sanctioned executor entry
    /// point) rather than pulling a runtime dependency into three crates for two pure fns. One per
    /// crate this file is compiled into, as R2 requires.
    fn block_on<F: Future>(future: F) -> F::Output {
        fn no_op(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, no_op, no_op, no_op);
        let waker = unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) };
        let mut cx = Context::from_waker(&waker);
        let mut future = std::pin::pin!(future);
        match future.as_mut().poll(&mut cx) {
            Poll::Ready(value) => value,
            Poll::Pending => panic!("block_on: future was not ready on first poll — this fn is documented I/O-free"),
        }
    }

    fn descriptor(extension_id: &str, extends: &str) -> ExtensionDescriptor {
        ExtensionDescriptor { extension_id: extension_id.into(), extends: extends.into(), version: "0.1.0".into(), content_hash: format!("hash-{extension_id}"), capabilities: Vec::new(), capability_requests: Vec::new() }
    }

    /// 🧫️ 64 synthetic descriptors, half extending `flow` and half `cad` — a smaller stand-in for
    /// the scale fixture's 50×50 shape, proving `extensions_extending` is a plain filter with no
    /// branch on `installed.len()`.
    #[test]
    fn extensions_extending_filters_by_extends_at_scale_and_returns_none_for_an_unknown_plugin() {
        let installed: Vec<ExtensionDescriptor> = (0..64).map(|i| descriptor(&format!("ext-{i}"), if i % 2 == 0 { "flow" } else { "cad" })).collect();

        let matched = block_on(extensions_extending("flow", &installed));
        assert_eq!(matched.len(), 32, "half of 64 synthetic descriptors extend `flow`");
        assert!(matched.iter().all(|d| d.extends == "flow"));

        let none = block_on(extensions_extending("nonexistent-plugin", &installed));
        assert!(none.is_empty());
    }

    #[test]
    fn scope_capabilities_to_parent_intersects_and_drops_what_the_parent_lacks() {
        let parent = vec![CapabilityId("storage.read".into()), CapabilityId("http:example.com".into())];
        let requested = vec![CapabilityId("storage.read".into()), CapabilityId("storage.write".into())];

        let scoped = block_on(scope_capabilities_to_parent(&parent, &requested));
        assert_eq!(scoped, vec![CapabilityId("storage.read".into())], "storage.write is not in the parent's effective set, so it must be dropped");
    }

    #[test]
    fn scope_capabilities_to_parent_is_empty_when_the_parent_grants_nothing() {
        let requested = vec![CapabilityId("storage.read".into())];
        assert!(block_on(scope_capabilities_to_parent(&[], &requested)).is_empty());
    }
}
//#endregion 🔖️ExtensionActivation
