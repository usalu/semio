//! 🧠️ Local-first action kernel contracts: actions, operations, capabilities, window I/O.

use crate::manifest::MediaType;
use dsl::DslValue;
pub use dsl::{Diagnostic, Fault, FaultCause, FaultCode, FaultFrom, FaultOrigin, FaultScope, Severity};
use serde::{Deserialize, Serialize};
use ui_wgpu::wgpu::UiNode;

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
#[serde(rename_all = "camelCase")]
pub enum Rights {
    Read,
    Write,
    Invoke,
    Open,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[serde(rename_all = "camelCase")]
pub enum Scope {
    Instance,
    App,
    Plugin,
    Global,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
#[serde(rename_all = "camelCase")]
pub struct PastePlacement {
    #[serde(default)]
    pub anchor: PasteAnchor,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<[f64; 3]>,
}

/// 📋️ A copied document fragment: `dsl_text` is the human-readable/`text/plain`-fallback encoding
/// (printed via the source app's own `ArtifactDsl` grammar over a fragment-shaped projection),
/// `pack_bytes` is the lossless binary lane for same-app/compatible paste. `media_type` is the
/// cross-app compatibility key (see `media_types_compatible`) an app's `clipboard_accepts()` checks
/// before offering to paste a fragment copied from a different app.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClipboardFragment {
    pub schema: String,
    pub media_type: MediaType,
    pub dsl_text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pack_bytes: Option<Vec<u8>>,
    pub source_app: String,
    pub label: String,
}

/// 🧯️ Clipboard operation failures — an app's `copy_fragment`/`paste_operations` return these instead
/// of panicking on an empty selection or an incompatible fragment.
#[derive(Debug)]
pub enum ClipboardError {
    EmptySelection,
    IncompatibleMediaType(MediaType),
    ParseFailed(String),
}

impl std::fmt::Display for ClipboardError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySelection => formatter.write_str("nothing selected to copy"),
            Self::IncompatibleMediaType(media_type) => write!(formatter, "clipboard fragment media type {media_type:?} not accepted by this app"),
            Self::ParseFailed(message) => write!(formatter, "clipboard fragment failed to parse: {message}"),
        }
    }
}

impl std::error::Error for ClipboardError {}
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
    OpenWindow {
        req: RequestId,
        kind: WindowKindId,
        params: DslValue,
    },
    CloseWindow {
        window: WindowHandle,
    },
    Notify {
        message: String,
    },
    /// 📋️ Asks the shell to write a copied/cut fragment to the OS clipboard (system clipboard where
    /// available, session-local fallback otherwise) — emitted by `VcsArtifactApp`'s `copy`/`cut`
    /// interception, never constructed by an app directly.
    ClipboardWrite {
        fragment: ClipboardFragment,
    },
    RequestSync,
    /// @emoji 🧭️ Navigates the shell to a URI (studio/instance/document route).
    Navigate {
        uri: String,
    },
    /// @emoji 📂️ Replaces the active app instance's document with pack+spr bytes — the host-owned
    /// counterpart of `loadAppArtifactPack`, used when the plugin resolves a catalog/example studio
    /// and needs the shell to swap the live store without going through a persistence binding.
    LoadDocument {
        pack: Vec<u8>,
        spr: Vec<u8>,
    },
    /// @emoji 🌐️ Opens an external URL in a new browser tab — the host-bridge substitute for a program
    /// reaching into `web-sys`/`window()` directly, which the plugin capability lint forbids.
    OpenExternalUrl {
        url: String,
    },
    /// @emoji 🗂️ Replaces the active studio/window panel state with a serialized panel JSON.
    SetPanel {
        panel_json: String,
    },
    /// @emoji ⬇️ Downloads an in-memory media export as a file (base64 or utf-8 `data`).
    DownloadMediaExport {
        filename: String,
        mime_type: String,
        data: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        encoding: Option<String>,
    },
    /// @emoji 🖼️ Renders one or more icon-scene requests to images and downloads each.
    IconRenderExport {
        items: Vec<IconRenderExportItem>,
    },
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
    SetActiveUtility {
        window_id: String,
        utility_id: String,
    },
    /// @emoji 🛠️ Programmatically switches the host-owned active tool of the active mode — the effect
    /// form of `setActiveTool`, letting a plugin change tools without a user click. Empty `tool_id`
    /// deactivates the current tool.
    SetActiveTool {
        tool_id: String,
    },
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
    SendMessage {
        target: MessageEndpoint,
        payload: Vec<u8>,
    },
    /// @emoji 📣️ Replaces `AppFrame::Events` — a pub/sub broadcast, not a directed message.
    PublishEvent {
        topic: String,
        payload: Vec<u8>,
    },
    BlobWrite {
        req: RequestId,
        media_type: MediaType,
        bytes: Vec<u8>,
    },
    /// @emoji 📥️ Also answers a lazy `read-asset` miss (assets are preloaded in
    /// `Event::InstanceOpen.assets`; this is the fallback for one that wasn't).
    BlobLoad {
        req: RequestId,
        hash: String,
    },
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
    DocumentRead {
        req: RequestId,
        doc: ArtifactHandle,
        lane: String,
    },
    DocumentWrite {
        req: RequestId,
        doc: ArtifactHandle,
        lane: String,
        ops: Vec<u8>,
    },
    LinkResolve {
        req: RequestId,
        link: String,
    },
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
    IoCompose {
        req: RequestId,
        key: String,
        sources: Vec<String>,
    },
    CacheDerive {
        req: RequestId,
        engine_id: String,
        input: Vec<u8>,
    },
    CacheRead {
        req: RequestId,
        engine_id: String,
        key: String,
    },
    /// @emoji ⏱️ Replaces self-tick loops and `pending_effects()` polling — the host wakes the
    /// instance with `Event::Timer { id }` after `after_ms`, repeating if `repeat` is set.
    SetTimer {
        id: u64,
        after_ms: u64,
        #[serde(default)]
        repeat: bool,
    },
    SpawnJob {
        job: u64,
        kind: String,
        input: Vec<u8>,
        placement: JobPlacement,
    },
    CancelJob {
        job: u64,
    },
    /// @emoji ↩️ Answers an inbound `Event::Request { req, .. }` within a bounded number of turns.
    Respond {
        req: RequestId,
        result: RequestOutcome,
    },
    StorageRead {
        req: RequestId,
        key: String,
    },
    StorageWrite {
        req: RequestId,
        key: String,
        bytes: Vec<u8>,
    },
    StorageDelete {
        req: RequestId,
        key: String,
    },
    RequestCapability {
        req: RequestId,
        capability: CapabilityRequest,
    },
    ReleaseCapability {
        id: CapabilityId,
    },
    /// @emoji 📡️ Replaces `backbone-poll`/`backbone-status` — inbound traffic on `topic` arrives
    /// as `Event::Message`.
    Subscribe {
        topic: String,
    },
    Unsubscribe {
        topic: String,
    },
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
    },
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
pub use semio_framework_os_kernel::{decode_presence_peer, encode_presence_peer, PresencePeer, PresenceUi, PresenceViewKind, PresenceWindowView};
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

//#region 🔖️PagedCommandIngress
pub const COMMAND_PAGE_MAXIMUM_BYTES: usize = 4_096;
pub const COMMAND_MAXIMUM_PAGES: usize = 64;
pub const COMMAND_MAXIMUM_BYTES: usize = COMMAND_PAGE_MAXIMUM_BYTES * COMMAND_MAXIMUM_PAGES;
pub const COMMAND_BATCH_MAXIMUM_ITEMS: usize = 64;

#[derive(Clone, Debug, PartialEq)]
pub struct FixedCommandPage {
    bytes: [u8; COMMAND_PAGE_MAXIMUM_BYTES],
    len: u16,
}

impl FixedCommandPage {
    pub fn try_from_array(bytes: [u8; COMMAND_PAGE_MAXIMUM_BYTES], len: u32) -> Result<Self, Fault> {
        let len = usize::try_from(len).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-page-length"), "command page length is not representable"))?;
        if len > COMMAND_PAGE_MAXIMUM_BYTES {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-page-length"), "command page length exceeds its fixed 4096-byte authority"));
        }
        if bytes[len..].iter().any(|byte| *byte != 0) {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-page-padding"), "command page carries nonzero bytes outside its declared authority"));
        }
        Ok(Self { bytes, len: len as u16 })
    }

    pub fn try_copy_from(bytes: &[u8]) -> Result<Self, Fault> {
        if bytes.len() > COMMAND_PAGE_MAXIMUM_BYTES {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-page-length"), "command page length exceeds its fixed 4096-byte authority"));
        }
        let mut fixed = [0; COMMAND_PAGE_MAXIMUM_BYTES];
        fixed[..bytes.len()].copy_from_slice(bytes);
        Ok(Self { bytes: fixed, len: bytes.len() as u16 })
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.bytes[..usize::from(self.len)]
    }

    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl Serialize for FixedCommandPage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeTuple;
        let mut tuple = serializer.serialize_tuple(self.len() + 1)?;
        tuple.serialize_element(&self.len)?;
        for byte in self.as_slice() {
            tuple.serialize_element(byte)?;
        }
        tuple.end()
    }
}

impl<'de> Deserialize<'de> for FixedCommandPage {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct FixedCommandPageVisitor;

        impl<'de> serde::de::Visitor<'de> for FixedCommandPageVisitor {
            type Value = FixedCommandPage;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a length-prefixed fixed command page")
            }

            fn visit_seq<A>(self, mut sequence: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::SeqAccess<'de>,
            {
                let len = sequence.next_element::<u16>()?.ok_or_else(|| serde::de::Error::custom("fixed command page omitted its length"))?;
                if usize::from(len) > COMMAND_PAGE_MAXIMUM_BYTES {
                    return Err(serde::de::Error::custom("fixed command page exceeds 4096 bytes"));
                }
                let mut bytes = [0; COMMAND_PAGE_MAXIMUM_BYTES];
                for byte in bytes.iter_mut().take(usize::from(len)) {
                    *byte = sequence.next_element::<u8>()?.ok_or_else(|| serde::de::Error::custom("fixed command page ended before its declared length"))?;
                }
                if sequence.next_element::<u8>()?.is_some() {
                    return Err(serde::de::Error::custom("fixed command page carries bytes beyond its declared length"));
                }
                Ok(FixedCommandPage { bytes, len })
            }
        }

        deserializer.deserialize_seq(FixedCommandPageVisitor)
    }
}

#[derive(Debug, PartialEq)]
pub struct CommandPageSet {
    pages: std::collections::VecDeque<FixedCommandPage>,
    byte_len: usize,
    generic_shape_valid: bool,
    all_nonempty: bool,
}

impl CommandPageSet {
    pub fn try_new() -> Result<Self, Fault> {
        let mut pages = std::collections::VecDeque::new();
        pages.try_reserve_exact(COMMAND_MAXIMUM_PAGES).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-page-allocation"), "fixed command page authority could not reserve its exact 64 slots"))?;
        Ok(Self { pages, byte_len: 0, generic_shape_valid: true, all_nonempty: true })
    }

    pub fn try_push(&mut self, page: FixedCommandPage) -> Result<(), (Fault, FixedCommandPage)> {
        if self.pages.len() == COMMAND_MAXIMUM_PAGES {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-page-count"), "command page authority is saturated"), page));
        }
        let Some(byte_len) = self.byte_len.checked_add(page.len()).filter(|total| *total <= COMMAND_MAXIMUM_BYTES) else {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-byte-cap"), "command exceeds its fixed 262144-byte authority"), page));
        };
        if page.is_empty() {
            self.generic_shape_valid = false;
            self.all_nonempty = false;
        }
        if self.pages.back().is_some_and(|previous| previous.len() != COMMAND_PAGE_MAXIMUM_BYTES) {
            self.generic_shape_valid = false;
        }
        self.pages.push_back(page);
        self.byte_len = byte_len;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.pages.len()
    }

    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        let Some(length) = self.pages.front().map(FixedCommandPage::len) else {
            return (true, 0);
        };
        if length > maximum_bytes {
            return (false, 0);
        }
        let page = self.pages.pop_front().expect("fixed command page was present");
        let released = page.len();
        self.byte_len -= released;
        drop(page);
        (self.pages.is_empty(), released)
    }
}

#[derive(Debug, PartialEq)]
pub struct PagedCommand {
    pages: std::collections::VecDeque<FixedCommandPage>,
    byte_len: usize,
    kind: u8,
    metadata: u32,
    item_count: u32,
}

impl PagedCommand {
    pub fn try_from_pages(pages: CommandPageSet) -> Result<Self, (Fault, CommandPageSet)> {
        if pages.is_empty() || pages.len() > COMMAND_MAXIMUM_PAGES {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-page-count"), "command requires 1..=64 admitted pages"), pages));
        }
        if !pages.generic_shape_valid {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-page-shape"), "command pages must be nonempty, at most 4096 bytes, and every nonterminal page must be full"), pages));
        }
        let Some(kind) = pages.pages.front().and_then(|page| page.as_slice().first()).copied() else {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-page-empty"), "command has no kind byte"), pages));
        };
        Ok(Self { pages: pages.pages, byte_len: pages.byte_len, kind, metadata: 0, item_count: 0 })
    }

    pub fn try_from_presence_pages(own_color: Option<u8>, pages: CommandPageSet, item_count: usize) -> Result<Self, (Fault, CommandPageSet)> {
        if item_count > COMMAND_BATCH_MAXIMUM_ITEMS || pages.len() != item_count.max(1) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-presence-item-cap"), "Presence command requires one exact page per peer and at most 64 peers"), pages));
        }
        if (item_count == 0 && pages.byte_len != 0) || (item_count != 0 && !pages.all_nonempty) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-presence-page-shape"), "each Presence peer page must be nonempty and at most 4096 bytes"), pages));
        }
        let metadata = own_color.map_or(0, |color| (1u32 << 8) | u32::from(color));
        Ok(Self { pages: pages.pages, byte_len: pages.byte_len, kind: 28, metadata, item_count: item_count as u32 })
    }

    pub fn byte_len(&self) -> usize {
        self.byte_len
    }

    pub fn page_len(&self) -> usize {
        self.pages.len()
    }

    pub fn front_page(&self) -> Option<&FixedCommandPage> {
        self.pages.front()
    }

    pub fn release_front_page(&mut self, maximum_bytes: usize) -> Option<(bool, usize)> {
        let Some(page_len) = self.pages.front().map(FixedCommandPage::len) else {
            return None;
        };
        if page_len > maximum_bytes {
            return None;
        }
        let page = self.pages.pop_front().expect("front page was present");
        self.byte_len -= page.len();
        let released = page.len();
        drop(page);
        Some((self.pages.is_empty(), released))
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.pages.is_empty() && self.byte_len == 0
    }

    pub fn kind(&self) -> u8 {
        self.kind
    }

    pub fn metadata(&self) -> u32 {
        self.metadata
    }

    pub fn item_count(&self) -> u32 {
        self.item_count
    }
}

#[derive(Debug)]
pub struct PagedCommandReader {
    command: PagedCommand,
    offset: usize,
}

impl PagedCommandReader {
    pub fn new(command: PagedCommand) -> Self {
        Self { command, offset: 0 }
    }

    pub fn kind(&self) -> u8 {
        self.command.kind()
    }

    pub fn read_byte(&mut self) -> Result<u8, Fault> {
        let byte = self.command.front_page().and_then(|page| page.as_slice().get(self.offset)).copied().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-decode-truncated"), "paged command ended inside a field"))?;
        self.offset += 1;
        if self.command.front_page().is_some_and(|page| self.offset == page.len()) {
            let _ = self.command.release_front_page(COMMAND_PAGE_MAXIMUM_BYTES).expect("fully consumed fixed page is releasable");
            self.offset = 0;
        }
        Ok(byte)
    }

    pub fn read_varint(&mut self) -> Result<u64, Fault> {
        let mut value = 0u64;
        for shift in (0..70).step_by(7) {
            let byte = self.read_byte()?;
            if shift == 63 && byte > 1 {
                return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-decode-varint"), "paged command varint overflowed u64"));
            }
            value |= u64::from(byte & 0x7f) << shift;
            if byte & 0x80 == 0 {
                return Ok(value);
            }
        }
        Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-decode-varint"), "paged command varint exceeds ten bytes"))
    }

    pub fn read_bounded_bytes(&mut self, maximum: usize) -> Result<Vec<u8>, Fault> {
        let length = usize::try_from(self.read_varint()?).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-field-length"), "paged command field length is not representable"))?;
        if length > maximum {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-field-cap"), "paged command field exceeds its exact bounded decode authority"));
        }
        let mut bytes = Vec::new();
        bytes.try_reserve_exact(length).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-field-allocation"), "paged command field could not reserve its exact bounded authority"))?;
        for _ in 0..length {
            bytes.push(self.read_byte()?);
        }
        Ok(bytes)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.offset == 0 && self.command.terminal_is_empty()
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        let Some(page_len) = self.command.front_page().map(FixedCommandPage::len) else {
            return (self.offset == 0, 0);
        };
        if page_len > maximum_bytes {
            return (false, 0);
        }
        let released = self.command.release_front_page(maximum_bytes).expect("front fixed page was grant-admitted").1;
        self.offset = 0;
        (self.command.terminal_is_empty(), released)
    }
}

#[derive(Debug, PartialEq)]
pub struct CommandEnvelope {
    pub instance: u32,
    pub seq: u64,
    pub command: PagedCommand,
}

#[derive(Debug, PartialEq)]
pub struct CommandBatch {
    pub generation: u64,
    commands: std::collections::VecDeque<CommandBatchEntry>,
    pages: std::collections::VecDeque<FixedCommandPage>,
    bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct CommandBatchEntry {
    instance: u32,
    seq: u64,
    kind: u8,
    metadata: u32,
    item_count: u32,
    page_count: u32,
    remaining_pages: u32,
}

#[derive(Debug, PartialEq)]
pub struct CommandEnvelopeSet {
    commands: std::collections::VecDeque<CommandBatchEntry>,
    page_storage: std::collections::VecDeque<FixedCommandPage>,
    pages: usize,
    bytes: usize,
}

impl CommandEnvelopeSet {
    pub fn try_new() -> Result<Self, Fault> {
        let mut commands = std::collections::VecDeque::new();
        commands.try_reserve_exact(COMMAND_BATCH_MAXIMUM_ITEMS).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-batch-allocation"), "fixed command batch authority could not reserve its exact 64 slots"))?;
        let mut page_storage = std::collections::VecDeque::new();
        page_storage.try_reserve_exact(COMMAND_MAXIMUM_PAGES).map_err(|_| Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-batch-page-allocation"), "fixed command batch authority could not reserve its exact 64 page slots"))?;
        Ok(Self { commands, page_storage, pages: 0, bytes: 0 })
    }

    pub fn try_push(&mut self, command: CommandEnvelope) -> Result<(), (Fault, CommandEnvelope)> {
        if self.commands.len() == COMMAND_BATCH_MAXIMUM_ITEMS {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-batch-cap"), "command batch exceeds its exact 64-item authority"), command));
        }
        let pages = match self.pages.checked_add(command.command.page_len()) {
            Some(pages) if pages <= COMMAND_MAXIMUM_PAGES => pages,
            _ => return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-batch-page-cap"), "command batch exceeds its aggregate 64-page authority"), command)),
        };
        let bytes = match self.bytes.checked_add(command.command.byte_len()) {
            Some(bytes) if bytes <= COMMAND_MAXIMUM_BYTES => bytes,
            _ => return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-batch-byte-cap"), "command batch exceeds its aggregate 262144-byte authority"), command)),
        };
        let CommandEnvelope { instance, seq, command } = command;
        let PagedCommand { pages: mut command_pages, kind, metadata, item_count, .. } = command;
        let page_count = u32::try_from(command_pages.len()).expect("admitted command page count is u32-bounded");
        self.commands.push_back(CommandBatchEntry { instance, seq, kind, metadata, item_count, page_count, remaining_pages: page_count });
        while let Some(page) = command_pages.pop_front() {
            self.page_storage.push_back(page);
        }
        self.pages = pages;
        self.bytes = bytes;
        Ok(())
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        let Some(command) = self.commands.front_mut() else {
            return (self.page_storage.is_empty(), 0);
        };
        if command.remaining_pages == 0 {
            let _terminal = self.commands.pop_front().expect("empty command-build shell was present");
            return (self.commands.is_empty() && self.page_storage.is_empty(), 0);
        }
        let Some(page_len) = self.page_storage.front().map(FixedCommandPage::len) else {
            return (false, 0);
        };
        if page_len > maximum_bytes {
            return (false, 0);
        }
        let page = self.page_storage.pop_front().expect("command-build page was present");
        let released = page.len();
        self.pages -= 1;
        self.bytes -= released;
        command.remaining_pages -= 1;
        drop(page);
        if command.remaining_pages == 0 {
            let _terminal = self.commands.pop_front().expect("empty command-build shell was present");
        }
        (self.commands.is_empty() && self.page_storage.is_empty(), released)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.commands.is_empty() && self.page_storage.is_empty() && self.pages == 0 && self.bytes == 0
    }
}

#[derive(Debug)]
pub struct RejectedCommandBuild {
    rejected: Option<CommandEnvelope>,
    admitted: CommandEnvelopeSet,
}

impl RejectedCommandBuild {
    pub fn new(admitted: CommandEnvelopeSet, rejected: CommandEnvelope) -> Self {
        Self { rejected: Some(rejected), admitted }
    }

    pub fn from_admitted(admitted: CommandEnvelopeSet) -> Self {
        Self { rejected: None, admitted }
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        if let Some(rejected) = self.rejected.as_mut() {
            let Some((empty, released)) = rejected.command.release_front_page(maximum_bytes) else {
                return (false, 0);
            };
            if empty {
                let _terminal = self.rejected.take().expect("rejected command reached terminal empty");
            }
            return (self.terminal_is_empty(), released);
        }
        self.admitted.close_step(maximum_bytes)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.rejected.is_none() && self.admitted.terminal_is_empty()
    }

    pub fn remaining_pages(&self) -> usize {
        self.rejected.as_ref().map_or(0, |rejected| rejected.command.page_len()) + self.admitted.pages
    }

    pub fn remaining_bytes(&self) -> usize {
        self.rejected.as_ref().map_or(0, |rejected| rejected.command.byte_len()) + self.admitted.bytes
    }
}

#[derive(Debug)]
pub struct RejectedCommandBuildRegistry<const CAPACITY: usize> {
    slots: [Option<RejectedCommandBuild>; CAPACITY],
    close_index: usize,
    occupied: usize,
}

impl<const CAPACITY: usize> RejectedCommandBuildRegistry<CAPACITY> {
    pub fn new() -> Self {
        assert!(CAPACITY > 0);
        Self { slots: std::array::from_fn(|_| None), close_index: 0, occupied: 0 }
    }

    pub fn can_insert(&self, key: u64) -> bool {
        self.slots[key as usize % CAPACITY].is_none()
    }

    pub fn try_insert(&mut self, key: u64, owner: RejectedCommandBuild) -> Result<(), (Fault, RejectedCommandBuild)> {
        let index = key as usize % CAPACITY;
        if self.slots[index].is_some() {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-build-close-capacity"), "fixed rejected command-build close registry is occupied or collided"), owner));
        }
        self.slots[index] = Some(owner);
        self.occupied += 1;
        Ok(())
    }

    pub fn insert_admitted(&mut self, key: u64, owner: RejectedCommandBuild) {
        let index = key as usize % CAPACITY;
        assert!(self.slots[index].is_none(), "fixed rejected command-build admission changed before insert");
        self.slots[index] = Some(owner);
        self.occupied += 1;
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        if self.occupied == 0 {
            return (true, 0, 0);
        }
        for _ in 0..CAPACITY {
            let index = self.close_index;
            self.close_index = (self.close_index + 1) % CAPACITY;
            let Some(owner) = self.slots[index].as_mut() else {
                continue;
            };
            let (terminal, released) = owner.close_step(maximum_bytes);
            if terminal {
                let terminal = self.slots[index].take().expect("terminal rejected command build was present");
                self.occupied -= 1;
                assert!(terminal.terminal_is_empty(), "rejected command build terminal witness changed before removal");
            }
            return (self.occupied == 0, 1, released);
        }
        (false, 0, 0)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.occupied == 0
    }
}

impl CommandBatch {
    pub fn try_new(generation: u64, commands: CommandEnvelopeSet) -> Result<Self, (Fault, CommandEnvelopeSet)> {
        if commands.commands.is_empty() {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-batch-cap"), "command batch requires at least one exact command owner"), commands));
        }
        Ok(Self { generation, commands: commands.commands, pages: commands.page_storage, bytes: commands.bytes })
    }

    pub fn generation(&self) -> u64 {
        self.generation
    }

    pub fn remaining_pages(&self) -> usize {
        self.pages.len()
    }

    pub fn remaining_bytes(&self) -> usize {
        self.bytes
    }

    fn terminal_is_empty(&self) -> bool {
        self.commands.is_empty() && self.pages.is_empty() && self.bytes == 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandBatchProgress {
    PageReady,
    Waiting,
    Complete,
    Faulted,
}

#[derive(Debug)]
pub struct CommandBatchDriver {
    owner: u64,
    batch: CommandBatch,
    command_index: u32,
    page_index: u32,
    admitted_page_count: u32,
    admitted_kind: u8,
    faulted: bool,
    waiting: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CommandDriverRetentionState {
    Active,
    Suspended,
    Closing,
}

#[derive(Debug)]
struct CommandDriverRetentionSlot {
    key: u64,
    generation: u64,
    driver: CommandBatchDriver,
    state: CommandDriverRetentionState,
    close_previous: Option<u16>,
    close_next: Option<u16>,
}

#[derive(Debug)]
pub struct CommandDriverRegistry<const CAPACITY: usize> {
    slots: [Option<CommandDriverRetentionSlot>; CAPACITY],
    close_head: Option<u16>,
    close_tail: Option<u16>,
    occupied: usize,
}

impl<const CAPACITY: usize> CommandDriverRegistry<CAPACITY> {
    pub fn new() -> Self {
        assert!(CAPACITY > 0 && CAPACITY <= usize::from(u16::MAX));
        Self { slots: std::array::from_fn(|_| None), close_head: None, close_tail: None, occupied: 0 }
    }

    pub fn try_insert(&mut self, key: u64, generation: u64, driver: CommandBatchDriver) -> Result<(), (Fault, CommandBatchDriver)> {
        if !self.can_insert(key) {
            return Err((Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-driver-capacity"), "fixed retained command-driver slot is occupied or collided"), driver));
        }
        self.insert_admitted(key, generation, driver);
        Ok(())
    }

    pub fn can_insert(&self, key: u64) -> bool {
        self.slots[key as usize % CAPACITY].is_none()
    }

    pub fn insert_admitted(&mut self, key: u64, generation: u64, driver: CommandBatchDriver) {
        let index = key as usize % CAPACITY;
        assert!(self.slots[index].is_none(), "fixed retained command-driver admission changed before insert");
        self.slots[index] = Some(CommandDriverRetentionSlot { key, generation, driver, state: CommandDriverRetentionState::Active, close_previous: None, close_next: None });
        self.occupied += 1;
    }

    pub fn with_driver_mut<R>(&mut self, key: u64, generation: u64, f: impl FnOnce(&mut CommandBatchDriver) -> R) -> Result<R, Fault> {
        let slot = self.slot_mut(key, generation)?;
        if slot.state != CommandDriverRetentionState::Active {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-driver-not-active"), "retained command driver is suspended or closing"));
        }
        Ok(f(&mut slot.driver))
    }

    pub fn prepare_suspend(&mut self, key: u64, generation: u64) -> Result<(), Fault> {
        let index = self.index_of(key, generation)?;
        if self.slots[index].as_ref().expect("retained command slot exists").state != CommandDriverRetentionState::Active {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-driver-suspend-state"), "retained command driver is not active before suspension"));
        }
        self.link_close(index);
        self.slots[index].as_mut().expect("retained command slot exists").state = CommandDriverRetentionState::Suspended;
        Ok(())
    }

    pub fn resume(&mut self, key: u64, generation: u64) -> Result<(), Fault> {
        let index = self.index_of(key, generation)?;
        if self.slots[index].as_ref().expect("retained command slot exists").state != CommandDriverRetentionState::Suspended {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-driver-resume-state"), "retained command driver is not suspended before resume"));
        }
        self.unlink_close(index);
        self.slots[index].as_mut().expect("retained command slot exists").state = CommandDriverRetentionState::Active;
        Ok(())
    }

    pub fn begin_close(&mut self, key: u64, generation: u64) -> Result<(), Fault> {
        let index = self.index_of(key, generation)?;
        let state = self.slots[index].as_ref().expect("retained command slot exists").state;
        if state == CommandDriverRetentionState::Active {
            self.link_close(index);
        }
        self.slots[index].as_mut().expect("retained command slot exists").state = CommandDriverRetentionState::Closing;
        Ok(())
    }

    pub fn begin_close_key(&mut self, key: u64) -> Result<u64, Fault> {
        let index = key as usize % CAPACITY;
        let generation = match self.slots[index].as_ref() {
            Some(slot) if slot.key == key => slot.generation,
            _ => return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-driver-stale"), "retained command driver key is stale")),
        };
        self.begin_close(key, generation)?;
        Ok(generation)
    }

    pub fn remove_terminal(&mut self, key: u64, generation: u64) -> Result<(), Fault> {
        let index = self.index_of(key, generation)?;
        if !self.slots[index].as_ref().expect("retained command slot exists").driver.terminal_is_empty() {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-driver-nonterminal-remove"), "retained command driver cannot be removed before terminal empty"));
        }
        if self.slots[index].as_ref().expect("retained command slot exists").state != CommandDriverRetentionState::Active {
            self.unlink_close(index);
        }
        let terminal = self.slots[index].take().expect("retained command slot exists");
        self.occupied -= 1;
        drop(terminal);
        Ok(())
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
        let Some(index) = self.close_head.map(usize::from) else {
            return (self.occupied == 0, 0, 0);
        };
        let (terminal, released) = {
            let slot = self.slots[index].as_mut().expect("close-list command slot exists");
            slot.driver.close_step(maximum_bytes)
        };
        if terminal {
            self.unlink_close(index);
            let terminal = self.slots[index].take().expect("terminal command slot exists");
            self.occupied -= 1;
            drop(terminal);
        }
        (self.occupied == 0, 1, released)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.occupied == 0 && self.close_head.is_none() && self.close_tail.is_none()
    }

    pub fn has_close_work(&self) -> bool {
        self.close_head.is_some()
    }

    pub fn contains(&self, key: u64, generation: u64) -> bool {
        self.index_of(key, generation).is_ok()
    }

    pub fn is_active(&self, key: u64, generation: u64) -> bool {
        self.index_of(key, generation).ok().and_then(|index| self.slots[index].as_ref()).is_some_and(|slot| slot.state == CommandDriverRetentionState::Active)
    }

    fn slot_mut(&mut self, key: u64, generation: u64) -> Result<&mut CommandDriverRetentionSlot, Fault> {
        let index = self.index_of(key, generation)?;
        Ok(self.slots[index].as_mut().expect("retained command slot exists"))
    }

    fn index_of(&self, key: u64, generation: u64) -> Result<usize, Fault> {
        let index = key as usize % CAPACITY;
        match self.slots[index].as_ref() {
            Some(slot) if slot.key == key && slot.generation == generation => Ok(index),
            _ => Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-driver-stale"), "retained command driver identity or generation is stale")),
        }
    }

    fn link_close(&mut self, index: usize) {
        let previous = self.close_tail;
        let index_u16 = u16::try_from(index).expect("command registry capacity is u16-bounded");
        {
            let slot = self.slots[index].as_mut().expect("retained command slot exists");
            slot.close_previous = previous;
            slot.close_next = None;
        }
        if let Some(previous) = previous {
            self.slots[usize::from(previous)].as_mut().expect("previous close slot exists").close_next = Some(index_u16);
        } else {
            self.close_head = Some(index_u16);
        }
        self.close_tail = Some(index_u16);
    }

    fn unlink_close(&mut self, index: usize) {
        let (previous, next) = {
            let slot = self.slots[index].as_ref().expect("retained command slot exists");
            (slot.close_previous, slot.close_next)
        };
        if let Some(previous) = previous {
            self.slots[usize::from(previous)].as_mut().expect("previous close slot exists").close_next = next;
        } else {
            self.close_head = next;
        }
        if let Some(next) = next {
            self.slots[usize::from(next)].as_mut().expect("next close slot exists").close_previous = previous;
        } else {
            self.close_tail = previous;
        }
        let slot = self.slots[index].as_mut().expect("retained command slot exists");
        slot.close_previous = None;
        slot.close_next = None;
    }
}

impl CommandBatchDriver {
    pub fn new(owner: u64, batch: CommandBatch) -> Self {
        Self { owner, batch, command_index: 0, page_index: 0, admitted_page_count: 0, admitted_kind: 0, faulted: false, waiting: false }
    }

    pub fn next_page(&mut self) -> Result<Option<(CommandPageCursor, FixedCommandPage)>, Fault> {
        if self.faulted || self.waiting {
            return Ok(None);
        }
        let Some(command) = self.batch.commands.front() else {
            return Ok(None);
        };
        if self.admitted_page_count == 0 {
            self.admitted_page_count = command.page_count;
            self.admitted_kind = command.kind;
        }
        let bytes = self.batch.pages.front().ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-owner-empty"), "nonterminal command owner has no page"))?.clone();
        let cursor = CommandPageCursor {
            owner: self.owner,
            generation: self.batch.generation,
            command_index: self.command_index,
            command_count: u32::try_from(self.batch.commands.len()).unwrap_or(u32::MAX).saturating_add(self.command_index),
            instance: command.instance,
            seq: command.seq,
            kind: self.admitted_kind,
            page_index: self.page_index,
            page_count: self.admitted_page_count,
            item_count: command.item_count,
            metadata: command.metadata,
        };
        Ok(Some((cursor, bytes)))
    }

    pub fn observe(&mut self, status: &CommandIngressStatus, maximum_release_bytes: usize) -> Result<CommandBatchProgress, Fault> {
        let cursor = match status {
            CommandIngressStatus::Idle => {
                return Ok(if self.batch.commands.is_empty() {
                    CommandBatchProgress::Complete
                } else if self.faulted {
                    CommandBatchProgress::Faulted
                } else if self.waiting {
                    CommandBatchProgress::Waiting
                } else {
                    CommandBatchProgress::PageReady
                });
            }
            CommandIngressStatus::PageAccepted(cursor) | CommandIngressStatus::Backpressure(cursor) | CommandIngressStatus::CommandPending(cursor) | CommandIngressStatus::CommandComplete(cursor) => cursor,
            CommandIngressStatus::Fault { cursor, .. } => cursor,
        };
        self.validate_cursor(cursor)?;
        match status {
            CommandIngressStatus::Backpressure(_) => Ok(CommandBatchProgress::PageReady),
            CommandIngressStatus::PageAccepted(_) => {
                let Some(command) = self.batch.commands.front_mut() else {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-owner-missing"), "accepted page has no exact host owner"));
                };
                let page_len = self.batch.pages.front().map(FixedCommandPage::len).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-owner-missing"), "accepted page has no exact retained batch page"))?;
                if page_len > maximum_release_bytes {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-release-budget"), "accepted page exceeds its exact release grant"));
                }
                let page = self.batch.pages.pop_front().expect("accepted retained batch page was present");
                let released = page.len();
                self.batch.bytes -= released;
                drop(page);
                command.remaining_pages =
                    command.remaining_pages.checked_sub(1).ok_or_else(|| Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-page-underflow"), "accepted page arrived after the retained command page owner was empty"))?;
                let empty = command.remaining_pages == 0;
                self.page_index = self.page_index.saturating_add(1);
                if empty {
                    self.waiting = true;
                    Ok(CommandBatchProgress::Waiting)
                } else {
                    Ok(CommandBatchProgress::PageReady)
                }
            }
            CommandIngressStatus::CommandPending(_) => {
                self.waiting = true;
                Ok(CommandBatchProgress::Waiting)
            }
            CommandIngressStatus::CommandComplete(_) => {
                let Some(command) = self.batch.commands.front() else {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-owner-missing"), "terminal command has no exact host owner"));
                };
                if command.remaining_pages != 0 {
                    return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-terminal-pages"), "terminal acknowledgement arrived before every exact page was released"));
                }
                let _terminal = self.batch.commands.pop_front().expect("terminal command was present");
                self.command_index = self.command_index.saturating_add(1);
                self.page_index = 0;
                self.admitted_page_count = 0;
                self.admitted_kind = 0;
                self.waiting = false;
                Ok(if self.batch.commands.is_empty() { CommandBatchProgress::Complete } else { CommandBatchProgress::PageReady })
            }
            CommandIngressStatus::Fault { .. } => {
                self.faulted = true;
                Ok(CommandBatchProgress::Faulted)
            }
            CommandIngressStatus::Idle => unreachable!(),
        }
    }

    pub fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize) {
        let Some(command) = self.batch.commands.front_mut() else {
            return (self.batch.pages.is_empty(), 0);
        };
        if command.remaining_pages == 0 {
            let _terminal = self.batch.commands.pop_front().expect("empty retained command shell was present");
            self.command_index = self.command_index.saturating_add(1);
            self.page_index = 0;
            self.admitted_page_count = 0;
            self.admitted_kind = 0;
            self.waiting = false;
            return (self.batch.terminal_is_empty(), 0);
        }
        let Some(page_len) = self.batch.pages.front().map(FixedCommandPage::len) else {
            return (false, 0);
        };
        if page_len > maximum_bytes {
            return (false, 0);
        }
        let page = self.batch.pages.pop_front().expect("retained batch close page was present");
        let released = page.len();
        self.batch.bytes -= released;
        drop(page);
        command.remaining_pages -= 1;
        let empty = command.remaining_pages == 0;
        if empty {
            let _terminal = self.batch.commands.pop_front().expect("empty command was present");
            self.command_index = self.command_index.saturating_add(1);
            self.page_index = 0;
            self.admitted_page_count = 0;
            self.admitted_kind = 0;
            self.waiting = false;
        }
        (self.batch.terminal_is_empty(), released)
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.batch.terminal_is_empty()
    }

    pub fn generation(&self) -> u64 {
        self.batch.generation
    }

    pub fn remaining_pages(&self) -> usize {
        self.batch.pages.len()
    }

    pub fn remaining_bytes(&self) -> usize {
        self.batch.bytes
    }

    fn validate_cursor(&self, cursor: &CommandPageCursor) -> Result<(), Fault> {
        let Some(command) = self.batch.commands.front() else {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-owner-missing"), "ingress status has no exact host owner"));
        };
        if cursor.owner != self.owner
            || cursor.generation != self.batch.generation
            || cursor.command_index != self.command_index
            || cursor.instance != command.instance
            || cursor.seq != command.seq
            || cursor.kind != self.admitted_kind
            || cursor.page_index != self.page_index
            || cursor.page_count != self.admitted_page_count
            || cursor.item_count != command.item_count
            || cursor.metadata != command.metadata
        {
            return Err(Fault::new(FaultOrigin::Framework, FaultCode::new("plugin.command-cursor-mismatch"), "ingress status does not identify the exact retained host owner"));
        }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CommandPageCursor {
    pub owner: u64,
    pub generation: u64,
    pub command_index: u32,
    pub command_count: u32,
    pub instance: u32,
    pub seq: u64,
    pub kind: u8,
    pub page_index: u32,
    pub page_count: u32,
    pub item_count: u32,
    pub metadata: u32,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum CommandIngressStatus {
    Idle,
    PageAccepted(CommandPageCursor),
    Backpressure(CommandPageCursor),
    CommandPending(CommandPageCursor),
    CommandComplete(CommandPageCursor),
    Fault { cursor: CommandPageCursor, fault: Vec<u8> },
}
//#endregion 🔖️PagedCommandIngress

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
    Activate {
        reason: ActivationEvent,
    },
    SuspendRequest,
    CapabilityChanged {
        change: CapabilityChange,
    },
    QuotaChanged {
        quotas: QuotaSchema,
    },

    CommandIngressPage {
        cursor: CommandPageCursor,
        bytes: FixedCommandPage,
    },

    /// 🎬️ `wit-flip` (26/08/20) — a user action against a UI node, `pack`-encoded
    /// `semio_framework_ui_contract::UiIntent`. Separate from paged command ingress so the host can
    /// tell a genuine UI interaction from a channel command without decoding the payload —
    /// `component.wit`'s `events::ui-intent-event`.
    UiIntent {
        instance: PluginInstanceId,
        intent: Vec<u8>,
    },

    SurfaceVisible {
        surface: String,
    },
    SurfaceHidden {
        surface: String,
    },
    SurfaceResized {
        surface: String,
        width: u32,
        height: u32,
    },
    PatchAck {
        surface: String,
        revision: u64,
    },
    /// 🩹️ Guest resends a full patch body (not a diff) on rejection — `revision`/`reason` name
    /// what the host couldn't apply.
    PatchRejected {
        surface: String,
        revision: u64,
        reason: String,
    },

    Completed {
        req: RequestId,
        result: RequestOutcome,
    },
    HttpChunk {
        req: RequestId,
        bytes: Vec<u8>,
        done: bool,
    },
    JobProgress {
        job: u64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        progress: Option<Vec<u8>>,
    },
    JobCompleted {
        job: u64,
        result: RequestOutcome,
    },

    Message {
        source: MessageEndpoint,
        payload: Vec<u8>,
    },

    Timer {
        id: u64,
    },
    Wake,

    /// ↩️ The former `extension.invoke`/`artifact-compose`/`io-run`/`io-sniff`/`artifact-infer`/
    /// `artifact-mutation-plan`/`migrate-artifact` — answered with `Effect::Respond` within a
    /// bounded number of turns, or by spawning a job.
    Request {
        req: RequestId,
        from: MessageEndpoint,
        capability: String,
        payload: Vec<u8>,
    },
}
//#endregion 🔖️Event

//#region 🔖️ActivationEvent
/// 🚀️ Why an instance was activated — `📓️design-abi.md` §2's activation-event list, matched
/// against a `manifest::PackageDescriptor.activation_events` declaration at install time.
#[derive(Debug, PartialEq, Serialize, Deserialize)]
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
pub use semio_framework_ui_contract::{PresenceUpdate, UiPatch, UiPatchOp};
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
    CheckpointReady { checkpoint: semio_framework_actor::JobCheckpoint },
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
    /// 👥️ M2 (ticket 26/08/17 `design-unified.md`): render-plane presence derived this turn by the
    /// reactor's own `PresenceHub` — `(surface, node_key)`-addressed, TTL-scoped, NEVER a document
    /// revision (a turn where only presence changed emits `presence` and zero `ui_patches`). Distinct
    /// from the roster's own replication channel (`protocol::PresencePeer`/`adopt_presence`), which
    /// is unchanged and carries collaboration TRUTH, not render addressing — see this field's own
    /// wire doc at `kernel_turn_result_to_wit` for the WIT `presence-update` repoint this pairs with.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub presence: Vec<PresenceUpdate>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_wake: Option<u64>,
    pub status: TurnStatus,
    pub fuel_used: u64,
    pub command_ingress: CommandIngressStatus,
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
#[serde(transparent)]
pub struct CapabilityId(pub String);

/// 🙏️ A guest's ask for a capability — `📓️design-abi.md` §5. Replaces `CapabilityRequirement`
/// for the plugin/extension actor runtime. The kernel-level `CapabilityRequirement`/`Rights`/
/// `Scope` action-dispatch model (above, `🔖️Capability` region) stays as-is: it has live
/// consumers outside this packet's owned paths (`🔌️plugin/🏗️builder`, `🔌️plugin/🖥️host`,
/// `🔌️plugin/🦀️component.rs`) — see this packet's report for the full consumer list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
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
#[serde(rename_all = "camelCase")]
pub struct BrokerCapabilityGrant {
    /// 🔢️ `u128` has no JavaScript number equivalent, so the mirror carries it as a decimal string —
    /// same treatment `PluginDependency.version` gets in `🛂️manifest`.
    pub token: CapabilityToken,
    pub id: CapabilityId,
    pub scope: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
#[serde(rename_all = "camelCase")]
pub struct QuotaSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub memory_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fuel_per_turn: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub turn_deadline_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tables: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mailbox_len: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub outstanding_requests: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timers: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub storage_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub network_bytes_per_min: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ui_nodes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub patch_hz: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub blob_resident_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub gpu_ms_per_frame: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background_ms_per_min: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
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
    fn admit_effect(&self, instance: &PluginInstanceId, effect: &Effect) -> impl std::future::Future<Output = Result<(), Fault>> + Send;
    fn on_turn_finished(&self, instance: &PluginInstanceId, usage: &Usage) -> impl std::future::Future<Output = ()> + Send;
    fn on_breach(&self, instance: &PluginInstanceId, breach: &QuotaBreach) -> impl std::future::Future<Output = FailureAction> + Send;
    fn on_capability_change(&self, instance: &PluginInstanceId, change: &CapabilityChange) -> impl std::future::Future<Output = ()> + Send;
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

    fn presence_driver(pages: &[&[u8]], item_count: usize) -> CommandBatchDriver {
        let mut page_set = CommandPageSet::try_new().unwrap();
        if pages.is_empty() {
            page_set.try_push(FixedCommandPage::try_copy_from(&[]).unwrap()).unwrap();
        } else {
            for page in pages {
                page_set.try_push(FixedCommandPage::try_copy_from(page).unwrap()).unwrap();
            }
        }
        let command = PagedCommand::try_from_presence_pages(Some(7), page_set, item_count).unwrap();
        let mut commands = CommandEnvelopeSet::try_new().unwrap();
        commands.try_push(CommandEnvelope { instance: 3, seq: 9, command }).unwrap();
        let batch = CommandBatch::try_new(11, commands).unwrap();
        CommandBatchDriver::new(5, batch)
    }

    fn generic_driver(first: &[u8], last: &[u8]) -> CommandBatchDriver {
        let mut page_set = CommandPageSet::try_new().unwrap();
        page_set.try_push(FixedCommandPage::try_copy_from(first).unwrap()).unwrap();
        page_set.try_push(FixedCommandPage::try_copy_from(last).unwrap()).unwrap();
        let command = PagedCommand::try_from_pages(page_set).unwrap();
        let mut commands = CommandEnvelopeSet::try_new().unwrap();
        commands.try_push(CommandEnvelope { instance: 4, seq: 12, command }).unwrap();
        CommandBatchDriver::new(6, CommandBatch::try_new(13, commands).unwrap())
    }

    #[test]
    fn generic_multi_page_owner_advances_only_after_each_exact_ack() {
        let first = [3u8; COMMAND_PAGE_MAXIMUM_BYTES];
        let mut driver = generic_driver(&first, b"tail");
        let (first_cursor, first_page) = driver.next_page().unwrap().unwrap();
        assert_eq!(first_cursor.page_count, 2);
        assert_eq!(first_page.len(), COMMAND_PAGE_MAXIMUM_BYTES);
        assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(first_cursor), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::PageReady);
        let (last_cursor, last_page) = driver.next_page().unwrap().unwrap();
        assert_eq!(last_cursor.page_index, 1);
        assert_eq!(last_page.as_slice(), b"tail");
        assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(last_cursor.clone()), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::Waiting);
        let mut terminal = last_cursor;
        terminal.page_index += 1;
        assert_eq!(driver.observe(&CommandIngressStatus::CommandComplete(terminal), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::Complete);
    }

    #[test]
    fn generic_backpressure_retains_the_exact_page_and_retry_cursor() {
        let first = [3u8; COMMAND_PAGE_MAXIMUM_BYTES];
        let mut driver = generic_driver(&first, b"tail");
        let (cursor, page) = driver.next_page().unwrap().unwrap();
        assert_eq!(driver.observe(&CommandIngressStatus::Backpressure(cursor.clone()), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::PageReady);
        let (retry_cursor, retry_page) = driver.next_page().unwrap().unwrap();
        assert_eq!(retry_cursor, cursor);
        assert_eq!(retry_page, page);
    }

    #[test]
    fn generic_stale_generation_status_is_rejected_without_releasing_the_owner() {
        let first = [3u8; COMMAND_PAGE_MAXIMUM_BYTES];
        let mut driver = generic_driver(&first, b"tail");
        let (mut stale, page) = driver.next_page().unwrap().unwrap();
        stale.generation -= 1;
        assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(stale), COMMAND_PAGE_MAXIMUM_BYTES).unwrap_err().code.0, "plugin.command-cursor-mismatch");
        assert_eq!(driver.next_page().unwrap().unwrap().1, page);
    }

    #[test]
    fn generic_cancel_after_first_page_releases_untouched_tail_in_one_bounded_close_step() {
        let first = [3u8; COMMAND_PAGE_MAXIMUM_BYTES];
        let mut driver = generic_driver(&first, b"tail");
        let (cursor, _) = driver.next_page().unwrap().unwrap();
        assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(cursor), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::PageReady);
        assert_eq!(driver.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 4));
        assert!(driver.terminal_is_empty());
    }

    #[test]
    fn fixed_command_driver_registry_returns_exact_colliding_owner_without_replacement() {
        let first = [3u8; COMMAND_PAGE_MAXIMUM_BYTES];
        let mut registry = CommandDriverRegistry::<2>::new();
        registry.try_insert(1, 9, generic_driver(&first, b"one")).unwrap();
        let (_, mut rejected) = registry.try_insert(3, 10, generic_driver(&first, b"rejected")).unwrap_err();
        assert_eq!(rejected.next_page().unwrap().unwrap().1.as_slice(), &first);
        registry.begin_close(1, 9).unwrap();
        while !registry.close_step(COMMAND_PAGE_MAXIMUM_BYTES).0 {}
        assert!(registry.terminal_is_empty());
    }

    #[test]
    fn suspended_command_driver_becomes_bounded_close_authority_if_caller_does_not_resume() {
        let first = [3u8; COMMAND_PAGE_MAXIMUM_BYTES];
        let mut registry = CommandDriverRegistry::<2>::new();
        registry.try_insert(1, 9, generic_driver(&first, b"tail")).unwrap();
        registry.prepare_suspend(1, 9).unwrap();
        assert_eq!(registry.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (false, 1, COMMAND_PAGE_MAXIMUM_BYTES));
        assert_eq!(registry.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 1, 4));
        assert!(registry.terminal_is_empty());
    }

    #[test]
    fn stale_command_driver_resume_cannot_reanimate_a_reused_direct_slot() {
        let first = [3u8; COMMAND_PAGE_MAXIMUM_BYTES];
        let mut registry = CommandDriverRegistry::<1>::new();
        registry.try_insert(1, 9, generic_driver(&first, b"tail")).unwrap();
        registry.prepare_suspend(1, 9).unwrap();
        assert_eq!(registry.resume(1, 8).unwrap_err().code.0, "plugin.command-driver-stale");
        registry.begin_close(1, 9).unwrap();
        while !registry.close_step(COMMAND_PAGE_MAXIMUM_BYTES).0 {}
    }

    #[test]
    fn retained_batch_arena_has_no_nested_page_or_descriptor_destructor() {
        assert!(!std::mem::needs_drop::<FixedCommandPage>());
        assert!(!std::mem::needs_drop::<CommandBatchEntry>());
        let mut commands = CommandEnvelopeSet::try_new().unwrap();
        for seq in 0..COMMAND_BATCH_MAXIMUM_ITEMS as u64 {
            let mut pages = CommandPageSet::try_new().unwrap();
            pages.try_push(FixedCommandPage::try_copy_from(&[3]).unwrap()).unwrap();
            commands.try_push(CommandEnvelope { instance: 1, seq, command: PagedCommand::try_from_pages(pages).unwrap() }).unwrap();
        }
        let mut driver = CommandBatchDriver::new(7, CommandBatch::try_new(8, commands).unwrap());
        for _ in 0..COMMAND_BATCH_MAXIMUM_ITEMS {
            assert_eq!(driver.close_step(COMMAND_PAGE_MAXIMUM_BYTES).1, 1);
        }
        assert!(driver.terminal_is_empty());
    }

    #[test]
    fn fault_after_last_page_ack_closes_the_empty_descriptor_shell_without_page_release_theater() {
        let mut driver = presence_driver(&[b"one"], 1);
        let (cursor, _) = driver.next_page().unwrap().unwrap();
        assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(cursor), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::Waiting);
        assert_eq!(driver.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 0));
        assert!(driver.terminal_is_empty());
    }

    #[test]
    fn rejected_command_build_registry_retains_collision_and_releases_one_exact_page() {
        let mut rejected_pages = CommandPageSet::try_new().unwrap();
        rejected_pages.try_push(FixedCommandPage::try_copy_from(b"rejected").unwrap()).unwrap();
        let rejected = CommandEnvelope { instance: 1, seq: 2, command: PagedCommand::try_from_pages(rejected_pages).unwrap() };
        let mut registry = RejectedCommandBuildRegistry::<1>::new();
        registry.try_insert(1, RejectedCommandBuild::new(CommandEnvelopeSet::try_new().unwrap(), rejected)).unwrap();

        let mut colliding_pages = CommandPageSet::try_new().unwrap();
        colliding_pages.try_push(FixedCommandPage::try_copy_from(b"collision").unwrap()).unwrap();
        let collision = RejectedCommandBuild::new(CommandEnvelopeSet::try_new().unwrap(), CommandEnvelope { instance: 1, seq: 3, command: PagedCommand::try_from_pages(colliding_pages).unwrap() });
        let (_, mut collision) = registry.try_insert(2, collision).unwrap_err();
        assert_eq!(collision.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 9));
        assert_eq!(registry.close_step(COMMAND_PAGE_MAXIMUM_BYTES), (true, 1, 8));
        assert!(registry.terminal_is_empty());
    }

    #[test]
    fn fixed_page_rejects_nonzero_padding_outside_declared_length() {
        let mut bytes = [0; COMMAND_PAGE_MAXIMUM_BYTES];
        bytes[7] = 1;
        assert_eq!(FixedCommandPage::try_from_array(bytes, 7).unwrap_err().code.0, "plugin.command-page-padding");
    }

    #[test]
    fn zero_presence_page_ack_releases_the_present_empty_owner_then_completes() {
        let mut driver = presence_driver(&[], 0);
        let (cursor, bytes) = driver.next_page().unwrap().unwrap();
        assert!(bytes.is_empty());
        assert_eq!(cursor.kind, 28);
        assert_eq!(cursor.item_count, 0);
        assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(cursor.clone()), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::Waiting);
        let mut terminal = cursor;
        terminal.page_index += 1;
        assert_eq!(driver.observe(&CommandIngressStatus::CommandComplete(terminal), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::Complete);
        assert!(driver.terminal_is_empty());
    }

    #[test]
    fn malformed_first_presence_fault_retains_then_closes_each_untouched_page() {
        let mut driver = presence_driver(&[b"bad-first", b"untouched-second"], 2);
        let (cursor, _) = driver.next_page().unwrap().unwrap();
        assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(cursor.clone()), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::PageReady);
        let mut fault_cursor = cursor;
        fault_cursor.page_index += 1;
        assert_eq!(driver.observe(&CommandIngressStatus::Fault { cursor: fault_cursor, fault: b"malformed".to_vec() }, COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::Faulted);
        assert!(driver.close_step(COMMAND_PAGE_MAXIMUM_BYTES).0);
        assert!(driver.terminal_is_empty());
    }

    #[test]
    fn malformed_middle_presence_fault_preserves_fifo_tail_for_bounded_close() {
        let mut driver = presence_driver(&[b"first", b"bad-middle", b"tail"], 3);
        let (first, _) = driver.next_page().unwrap().unwrap();
        assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(first), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::PageReady);
        let (middle, _) = driver.next_page().unwrap().unwrap();
        assert_eq!(driver.observe(&CommandIngressStatus::PageAccepted(middle.clone()), COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::PageReady);
        let mut fault_cursor = middle;
        fault_cursor.page_index += 1;
        assert_eq!(driver.observe(&CommandIngressStatus::Fault { cursor: fault_cursor, fault: b"malformed".to_vec() }, COMMAND_PAGE_MAXIMUM_BYTES).unwrap(), CommandBatchProgress::Faulted);
        assert!(driver.close_step(COMMAND_PAGE_MAXIMUM_BYTES).0);
    }
}
//#endregion 🔖️ExtensionActivation
