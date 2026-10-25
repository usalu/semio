//! 🎠️ Kernel effects, events, turn results, invocation contexts, broker hooks and the return content wire.

use semio_framework_actor::byte_page::ACTOR_BYTE_PAGE_BYTES;
use semio_framework_ui_contract::PresenceUpdate;
use semio_framework_value::DslValue;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use serde::Deserialize;
use serde::Serialize;
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use semio_framework_ui_declaration::ui::UiNode;
use semio_framework_kernel_history::HistoryPatch;
use semio_framework_kernel_identity::{ActionInvocation, ActivationEvent, AppEvent, AppInstanceId, ArtifactHandle, BrokerCapabilityGrant, CapabilityChange, CapabilityGrant, CapabilityId, CapabilityRequest, CommandInvocation, FailureAction, KernelMutation, PluginInstanceId, QuotaBreach, QuotaSchema, RequestId, RequestOutcome, TurnStatus, UndoGroup, Usage, WindowHandle, WindowKindId};
use semio_framework_kernel_media_jobs::{IconRenderExportItem, SpawnedJobCompletion, VideoRenderProgram};
use semio_framework_kernel_ui::{UiDirtyScope, UiTurnPatches};
use semio_framework_manifest_media::MediaType;
use semio_framework_manifest_view_model::ViewModel;

pub use semio_framework_actor::{OriginalActorTurnWork,RetainedTurnInput,RetainedTurnReceipt,OriginalJobSource,OriginalJobSourceCursor,OriginalJobTurnLoan,OriginalJobBudget,OriginalJobCallFailure,OriginalJobStartReply,OriginalJobStepReply,OriginalJobCancelReply,OriginalJobOutputState,OriginalJobOutcome,OriginalJobOutputChunk,OriginalJobOutputReply,OriginalJobOutputAckReply};

pub use semio_framework_diagnostic::Diagnostic;

pub use semio_framework_diagnostic::Fault;

pub use semio_framework_diagnostic::FaultCause;

pub use semio_framework_diagnostic::FaultCode;

pub use semio_framework_diagnostic::FaultFrom;

pub use semio_framework_diagnostic::FaultOrigin;

pub use semio_framework_diagnostic::FaultScope;

pub use semio_framework_diagnostic::Severity;

//#region 🔖️Invocation
/// ⚓️ How a paste anchors the copied fragment relative to the paste point — the seven placement
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

/// 📌️ Where/how a paste places its fragment: `anchor` picks the reference point, `position` (when
/// given) overrides where that reference point lands.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PastePlacement {
    #[serde(default)]
    pub anchor: PasteAnchor,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub position: Option<[f64; 3]>,
}

#[path="📋️clipboard/🦀️.rs"]
mod clipboard_envelope;

pub use clipboard_envelope::{CLIPBOARD_TEXT_MAX_BYTES,CLIPBOARD_METADATA_MAX_WIRE_BYTES,CLIPBOARD_FRAGMENT_MAX_WIRE_BYTES,CLIPBOARD_PASTE_MAX_WIRE_BYTES};

/// 📋️ A copied document fragment: `dsl_text` is the human-readable/`text/plain`-fallback encoding
/// (printed via the source app's own `ArtifactDsl` grammar over a fragment-shaped projection),
/// `pack_bytes` is the lossless binary lane for same-app/compatible paste. `media_type` is the
/// cross-app compatibility key (see `media_types_compatible`) an app's `clipboard_accepts()` checks
/// before offering to paste a fragment copied from a different app.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
pub struct ClipboardFragment {
    pub schema: String,
    pub media_type: MediaType,
    pub dsl_text: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
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

//#endregion 🔖️Invocation
//#region 🔖️Effect
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
// 🚧️ Both `Serialize` AND `Deserialize` stay unconditional (kept additive, not stripped):
// `semio-framework-plugin`'s `RefreshResponse` (`🔌️plugin/🦀️.rs`, `requested_effects: Vec<Effect>`)
// derives `Serialize` in production and needs `Effect: Serialize`; this file's own `TurnResult`
// (`effects: Vec<Effect>`, above `🔖️Invocation`) derives `Deserialize` in production too — a real
// consumer a first pass at `#[cfg_attr(test, derive(Deserialize))]` missed (confirmed the hard way:
// `cargo check -p semio-framework` failed E0277 on `Effect: serde::Deserialize` at `TurnResult`'s
// own derive site before this was reverted). `Effect` already carries `ToValue`/`FromValue`
// alongside — this file's own 4 `#[cfg(test)]` round-trip oracles in `🛂️manifest/🦀️.rs` exercise
// the same, still-production, serde derives; nothing further to move to `[dev-dependencies]`.
// Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
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
    /// ✂️ Asks the shell to write a copied/cut fragment to the OS clipboard (system clipboard where
    /// available, session-local fallback otherwise) — emitted by `VcsArtifactApp`'s `copy`/`cut`
    /// interception, never constructed by an app directly.
    ClipboardWrite {
        fragment: ClipboardFragment,
    },
    RequestSync,
    /// 🧭️ Navigates the shell to a URI (studio/instance/document route).
    Navigate {
        uri: String,
    },
    /// 📂️ Replaces the active app instance's document with pack+spr bytes — the host-owned
    /// counterpart of `loadAppArtifactPack`, used when the plugin resolves a catalog/example studio
    /// and needs the shell to swap the live store without going through a persistence binding.
    LoadDocument {
        pack: Vec<u8>,
        spr: Vec<u8>,
    },
    /// 🌐️ Opens an external URL in a new browser tab — the host-bridge substitute for a program
    /// reaching into `web-sys`/`window()` directly, which the plugin capability lint forbids.
    OpenExternalUrl {
        url: String,
    },
    /// 🗂️ Replaces the active studio/window panel state with a serialized panel JSON.
    SetPanel {
        panel_json: String,
    },
    /// ⬇️ Downloads an in-memory media export as a file (base64 or utf-8 `data`).
    DownloadMediaExport {
        filename: String,
        mime_type: String,
        data: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        encoding: Option<String>,
    },
    /// 🖼️ Renders one or more icon-scene requests to images and downloads each.
    IconRenderExport {
        items: Vec<IconRenderExportItem>,
    },
    /// 🎥️ Asks the host to render `program` frame by frame, encode it as H.264 in an MP4 and
    /// download it as `filename` — the host owns the canvas and the encoder a guest has none of, lends
    /// them only to a plugin that requested [`MEDIA_VIDEO_RENDER_CAPABILITY`], and runs the render as
    /// an event-sourced job ([`VideoRenderJobEvent`]) with progress and cancellation. See [`VideoRenderProgram`].
    VideoRenderExport {
        filename: String,
        program: VideoRenderProgram,
    },
    /// 📤️ Asks the shell to open a file picker and re-dispatch `import_action` with the
    /// picked file's contents as `{ payload, name }` args. When `multiple` is set, the picker allows
    /// selecting several files and `import_action` is re-dispatched once per file, sequentially, each
    /// call extending the args with `{ index, total }`.
    RequestFileOpen {
        req: RequestId,
        accept: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        read_as: Option<String>,
        import_action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
        #[serde(default)]
        #[value(default)]
        multiple: bool,
    },
    /// 🎞️ Asks the shell to decode a video (via file picker, or `payload` bytes when the
    /// caller already has them, e.g. a drop zone) and re-dispatch `frame_action` once per sampled
    /// frame with `{payload: dataUrl(image/jpeg), name, frameIndex, timestampMs, index, total, width,
    /// height, ...args}`, then `done_action` once with `{name, durationMs, frameCount, sampledCount,
    /// width, height, codec, ...args}`. `sample_stride`/🧰️framework/🔨️modules/🎠️kernel`max_frames`/🧰️framework/🔨️modules/🎠️kernel`max_long_edge_px`/🧰️framework/🔨️modules/🎠️kernel`fps_hint` are
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
        #[value(default)]
        sample_stride: u32,
        #[serde(default)]
        #[value(default)]
        max_frames: u32,
        #[serde(default)]
        #[value(default)]
        max_long_edge_px: u32,
        #[serde(default)]
        #[value(default)]
        fps_hint: f64,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        payload: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
    },
    /// ✨️ Spawns a plugin instance (idempotent on `os_instance_id`) without focusing it.
    SpawnPluginInstance {
        req: RequestId,
        plugin_id: String,
        app_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        os_instance_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        label: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        document_json: Option<String>,
    },
    /// 🪟️ Spawns (if needed) and focuses/navigates to a plugin instance.
    OpenPluginInstance {
        plugin_id: String,
        app_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        os_instance_id: Option<String>,
    },
    /// 🧰️ Programmatically switches the host-owned active utility of a window instance — the effect
    /// form of `setActiveUtility`, letting a plugin change utilities without a user click.
    SetActiveUtility {
        window_id: String,
        utility_id: String,
    },
    /// 🛠️ Programmatically switches the host-owned active tool of the active mode — the effect
    /// form of `setActiveTool`, letting a plugin change tools without a user click. Empty `tool_id`
    /// deactivates the current tool.
    SetActiveTool {
        tool_id: String,
    },
    /// 🗨️ Opens a declared `AppDefinition.dialogs` entry; `args` (an object keyed by arg id)
    /// pre-seeds the staged form. Kernel-altitude — plain `String`/`Value`, no manifest types.
    OpenDialog {
        req: RequestId,
        dialog_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
    },
    /// 🔁️ Re-dispatches `action` onto the same plugin instance after `delay_ms` — lets a
    /// plugin's `handle_action` advance staged/progressive work (e.g. a multi-pass reconstruction)
    /// over several ticks without blocking the host. The host feeds the follow-up response's own
    /// `requestedEffects` back through the same effect-application pass, so a `DispatchAction` can
    /// itself emit another one, chaining as many ticks as the plugin needs.
    DispatchAction {
        req: RequestId,
        action: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
        delay_ms: u64,
    },
    /// ⏪️ Asks the shell to redispatch a shell-owned command (dock/theme/locale/panel chrome)
    /// whose real mutation and its inverse both live client-side — the plugin has no access to that
    /// state, so `revertToCommand` on a `🐚️Shell`-kind history row bubbles the row's stored inverse out
    /// here instead of replaying it internally the way a `View`-kind row does (see
    /// `NOTE_SHELL_COMMAND_ACTION_ID` and `VcsArtifactApp::dispatch_action`'s `REVERT_TO_COMMAND_ACTION_ID`
    /// arm). The shell is expected to redispatch `action_id`/`args` through its normal command funnel,
    /// which itself calls `noteShellCommand` again — so the revert is itself a new, further-revertible row.
    ReplayShellCommand {
        action_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
    },
    /// 🔌️ Asks the shell to invoke an extension capability. `req` is MINTED by the guest
    /// request registry (`semio-framework-plugin`'s `RequestRegistry`) and is meaningless — a
    /// silently discarded result — when it is written by hand: `Event::Completed { req, .. }` is
    /// routed exclusively through that registry, and an id with no registry slot resolves nothing.
    /// The variant is therefore `#[non_exhaustive]`: no crate outside this one can write the struct
    /// literal, and the only in-repo constructor is [`Effect::invoke_extension`], called from the
    /// three registry/host lift sites that legitimately own a minted id. Application code reaches
    /// this effect exclusively through `Emit::extension_invocations`.
    #[non_exhaustive]
    InvokeExtension {
        req: RequestId,
        extension_id: String,
        capability: String,
        request_json: String,
    },

    /// ✉️ Replaces every non-UI/non-event `AppFrame::*` plus `backbone-send` — `target`
    /// picks shell vs. backbone vs. a specific plugin/extension/topic.
    SendMessage {
        target: MessageEndpoint,
        payload: Vec<u8>,
    },
    /// 📣️ Replaces `AppFrame::Events` — a pub/sub broadcast, not a directed message.
    PublishEvent {
        topic: String,
        payload: Vec<u8>,
    },
    BlobWrite {
        req: RequestId,
        media_type: MediaType,
        bytes: Vec<u8>,
    },
    /// 📥️ Also answers a lazy `read-asset` miss (assets are preloaded in
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
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        headers: Vec<(String, String)>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        body: Option<Vec<u8>>,
        #[serde(default)]
        #[value(default)]
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
    /// 🔍️ On-demand io-dialect lookup — the routing table itself is preloaded in
    /// `Event::InstanceOpen`; this is only for entries that weren't.
    RegistryQuery {
        req: RequestId,
        kind: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
        filter: Option<DslValue>,
    },
    /// 🗺️ Routed by the host `IoRouter` to the owning plugin as an `Event::Request` — one
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
    /// ⏱️ Replaces self-tick loops and `pending_effects()` polling — the host wakes the
    /// instance with `Event::Timer { id }` after `after_ms`, repeating if `repeat` is set.
    SetTimer {
        id: u64,
        after_ms: u64,
        #[serde(default)]
        #[value(default)]
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
    /// ↩️ Answers an inbound `Event::Request { req, .. }` within a bounded number of turns.
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
    /// 📡️ Replaces `backbone-poll`/`backbone-status` — inbound traffic on `topic` arrives
    /// as `Event::Message`.
    Subscribe {
        topic: String,
    },
    Unsubscribe {
        topic: String,
    },
    /// 💡️ Requests an installed owner's operation within the shell's authenticated document scope.
    RequestServiceOperation {
        owner: String,
        service_id: String,
        action: String,
        payload: DslValue,
    },
}

impl Effect {
    /// 🔂️ The single constructor for the `#[non_exhaustive]` [`Effect::InvokeExtension`] variant —
    /// `req` MUST come from the guest `RequestRegistry` (`RequestRegistry::request_continuation`)
    /// or from a host-side WIT lift of an id the guest already minted. Hand-written ids resolve
    /// nothing, which is exactly the foot-gun the variant's `#[non_exhaustive]` closes.
    pub fn invoke_extension(req: RequestId, extension_id: String, capability: String, request_json: String) -> Self {
        Self::InvokeExtension { req, extension_id, capability, request_json }
    }
}

/// 🚦 Where a spawned job runs — `📓️design-abi.md` §2's `spawn-job.placement`: `Inline` shares
/// the instance's own turn budget, `Isolated` gets its own pooled actor, `Exclusive` gets a
/// dedicated one (e.g. flow/brep tessellation, per `📓️design-abi.md` §5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
pub enum JobPlacement {
    Inline,
    Isolated,
    Exclusive,
}

//#region 🧵️SpawnedJobDrive
impl JobPlacement {
    /// 🔤️ The WIT `job-placement` case name this placement crosses the actor boundary as
    /// (`🔌️plugin/🧬️schema/📜️.wit`'s `enum job-placement`). jco hands a bare string for a WIT `enum`,
    /// not a `{tag}` record, so a renderer door reads THIS vocabulary and nothing else.
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::Inline => "inline",
            Self::Isolated => "isolated",
            Self::Exclusive => "exclusive",
        }
    }

    /// 🔙️ The inverse of [`Self::wire_name`]. An unknown spelling is `None` rather than a silent
    /// default: a host that guessed `Inline` for an unrecognised placement would run a job meant for
    /// its own pooled actor inside the spawning instance's turn budget.
    pub fn from_wire_name(name: &str) -> Option<Self> {
        match name {
            "inline" => Some(Self::Inline),
            "isolated" => Some(Self::Isolated),
            "exclusive" => Some(Self::Exclusive),
            _ => None,
        }
    }

    /// 📚️ The whole vocabulary, in WIT declaration order — what a language-agnostic law enumerates.
    pub const ALL: [Self; 3] = [Self::Inline, Self::Isolated, Self::Exclusive];
}

/// 🧵 The event a completed spawned job owes its actor — the same correlation the guest's own reactor
/// reads (`⚛️reactor/🔄️turn/🦀️.rs`: the `job` id IS the parked request id, so no host-side table
/// exists or is needed).
pub fn spawned_job_completed_event(job: u64, completion: &SpawnedJobCompletion) -> Event {
    Event::JobCompleted { job, result: completion.outcome.clone() }
}

//#endregion 🧵️SpawnedJobDrive
//#endregion 🔖️Effect
// 🎯️ W6 kernel unification: re-exports `protocol::ArtifactDiff` (schema: `SchemaId`, payload:
// `Vec<u8>` — the binary shape from W5's causal envelope reshape) in place of the old kernel-local
// `{schema_id, payload: Value}` shape. Zero external consumers of the old shape existed outside
// this crate's own (now-deleted) OS JSON-patch kernel and `store`/`store_sync` (both repointed to
// `protocol::ArtifactDiff` directly in this same wave) — verified by a repo-wide grep before this
// change, not assumed.
pub use ::replication::ArtifactDiff;

// 🎯️ W6 kernel unification: re-exports `protocol_core::UndoPolicy` (identical variants; the old
// CW3-era deferral note about a `#[serde(rename_all = "camelCase")]` mismatch no longer applies —
// see `HybridLogicalTimestamp`'s doc above for the same reconciliation).
pub use ::replication::UndoPolicy;


#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct InvocationResult {
    pub output: DslValue,
    pub mutations: Vec<KernelMutation>,
    pub inverse_group: UndoGroup,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub diagnostics: Vec<Diagnostic>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub requested_effects: Vec<Effect>,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<AppEvent>,
    #[value(default)]
    pub ui_scope: UiDirtyScope,
    /// 🧾️ Incremental command-history delivery. It is independent from `ui_scope`: history must
    /// become visible before effects or an unrelated UI refresh can be queued.
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub history_patch: Option<HistoryPatch>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ActionContext {
    pub invocation: ActionInvocation,
    pub document_snapshot: DslValue,
    pub view_state: ViewModel,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub granted_capabilities: Vec<CapabilityGrant>,
}

/// 🎛️ Context for a dispatched `CommandInvocation` — the command mirror of `ActionContext`.
/// No `document_snapshot`/`granted_capabilities`: `VcsArtifactApp` owns the store directly and
/// commands don't yet carry a capability grant model (mirrors actions' current state).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CommandContext {
    pub invocation: CommandInvocation,
    pub view_state: ViewModel,
}

//#region 🔖️Presence

pub use ::replication::{decode_presence_history_edit, decode_presence_peer, encode_presence_history_edit, encode_presence_peer, PresenceHistoryEdit, PresenceHistoryEditStage, PresencePeer, PresenceToolRun, PresenceToolRunState, PresenceUi, PresenceViewKind, PresenceWindowView};

//#endregion 🔖️Presence
//#region 🔖️Window
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActionRequest {
    pub invocation: ActionInvocation,
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
pub enum MessageEndpoint {
    Shell { instance: PluginInstanceId },
    Backbone { uri: String },
    PluginInstance { id: PluginInstanceId },
    Extension { id: String },
    Topic { name: String },
}

pub use ::replication::wire::command_ingress::{
    FixedCommandPage, CommandPageSet, PagedCommand, PagedCommandReader, CommandEnvelope, CommandBatch, CommandEnvelopeSet, RejectedCommandBuild, RejectedCommandBuildRegistry, CommandBatchProgress, CommandBatchDriver, CommandDriverRegistry, CommandPageCursor, CommandIngressStatus, COMMAND_PAGE_MAXIMUM_BYTES, COMMAND_MAXIMUM_BYTES, COMMAND_MAXIMUM_PAGES, COMMAND_BATCH_MAXIMUM_ITEMS, INVOCATION_RESULT_PACK_MAXIMUM_BYTES
};

/// 📬️ Everything the host delivers into a guest's `reactor::poll` — the full inbound contract
/// from `📓️design-abi.md` §2. Lifecycle events open/close/activate/suspend an instance and push
/// capability/quota changes; channel/surface/completion/messaging/timer/request events drive a
/// turn. Nothing constructs one yet — additive, packet A2-abi-sdk's executor is the first reader.
pub use semio_framework_actor::instance_lifetime::{ActorInstanceCloseRequest, ActorInstanceLifecycleAck, ActorInstanceLifecycleReceipt, ActorInstanceLifetime, ActorInstanceOpenRequest, ActorUiPatchReceipt};

#[path = "📥️cold-pair/🦀️.rs"]
mod cold_pair;

pub use cold_pair::{ColdDocumentPairApplied, ColdDocumentPairCursor, ColdDocumentPairFrontier, ColdDocumentPairHeader, ColdDocumentPairPage, ColdPairIngressStatus, COLD_PAIR_MAXIMUM_BYTES, COLD_PAIR_MAXIMUM_PAGES, COLD_PAIR_PAGE_MAXIMUM_BYTES};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[expect(clippy::large_enum_variant, reason = "Command ingress carries one fixed bounded page inline; delivery must not allocate an indirect page owner.")]
#[derive(semio_framework_value::RetireOwned)]
pub enum Event {
    /// 🐣️ First event an instance receives — config/assets/capabilities/quotas are preloaded so
    /// the first `poll` never blocks. `actor` is a placeholder `String` until the concurrently
    /// landing `🎭️actor` crate's `RuntimeActorId` exists (this packet must not depend on it —
    /// see the report's `🎭️actor` naming-hazard note).
    InstanceOpen {
        request: ActorInstanceOpenRequest,
        app_id: AppInstanceId,
        actor: String,
        config: Vec<u8>,
        assets: Vec<(String, Vec<u8>)>,
        capabilities: Vec<BrokerCapabilityGrant>,
        quotas: QuotaSchema,
    },
    InstanceClose(ActorInstanceCloseRequest),
    InstanceLifecycleAck(ActorInstanceLifecycleAck),
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

    ColdDocumentPairPage(ColdDocumentPairPage),

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
        body_key: String,
        view_state: Vec<u8>,
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
        receipt: ActorUiPatchReceipt,
        surface: String,
        revision: u64,
    },
    /// 🩹️ Guest resends a full patch body (not a diff) on rejection — `revision`/`reason` name
    /// what the host couldn't apply.
    PatchRejected {
        receipt: ActorUiPatchReceipt,
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

    /// ❓️ The former `extension.invoke`/`artifact-compose`/`io-run`/`io-sniff`/`artifact-infer`/
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
//#region 🔖️TurnResult
#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnResult {
    pub retained_receipt: RetainedTurnReceipt,
    pub ui_patches: UiTurnPatches,
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
    pub cold_pair_ingress: ColdPairIngressStatus,
    pub lifecycle_receipt: Option<ActorInstanceLifecycleReceipt>,
    pub ui_patch_receipt: Option<ActorUiPatchReceipt>,
}

impl TurnResult {
    /// 🔗️ Checks the typed count while all publication descendants remain in this structural owner.
    pub fn validate_ui_patch_receipt(&self) -> Result<(), &'static str> {
        ActorUiPatchReceipt::validate_pairing(self.ui_patch_receipt, self.ui_patches.len())
    }
}

#[path = "📤️return/📦️content/🦀️.rs"]
pub mod return_content;

#[path = "📤️return/📦️content/💌️message/🦀️.rs"]
pub mod return_message;

//#endregion 🔖️TurnResult
//#region 🔖️Broker
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

#[cfg(test)]
use semio_framework_pack_json::ArtifactCanonicalJsonNode as Node;
#[cfg(test)]
use semio_framework_value::Number;
#[cfg(test)]
use semio_framework_kernel_media_jobs::{FRAMEWORK_RESERVED_JOB_KIND, IMPORT_ARGUMENT_CHUNK, IMPORT_ARGUMENT_CHUNK_COUNT, IMPORT_ARGUMENT_INDEX, IMPORT_ARGUMENT_NAME, IMPORT_ARGUMENT_PAYLOAD, IMPORT_ARGUMENT_TOTAL, IMPORT_CHUNK_BYTES, IMPORT_STAGING_MAXIMUM_BYTES, IMPORT_STAGING_RUNS, ImportArguments, ImportChunk, ImportStaging, ImportStagingRefusal, MEDIA_EXPORT_BASE64_ENCODING, MEDIA_EXPORT_UTF8_ENCODING, MediaExportEncodingError, SPAWNED_JOB_DEADLINE_MS, SPAWNED_JOB_FUEL, SPAWNED_JOB_STEP_CEILING, SpawnedJobDriveError, SpawnedJobStep, VIDEO_RENDER_PROGRAM_SCHEMA, import_chunk_arguments, import_payload_chunks, media_export_bytes, spawned_job_completion};

//#region 🔖️Effect
#[cfg(test)]
#[path = "🧪️tests/💡️service-operation/🦀️.rs"]
mod service_operation_tests;

//#region ⬇️MediaExportEncoding
#[cfg(test)]
#[path = "🧪️tests/⬇️media-export-encoding/🦀️.rs"]
mod media_export_encoding_tests;

//#endregion ⬇️MediaExportEncoding
//#region 🎞️VideoRenderProgram
#[cfg(test)]
#[path = "🧪️tests/🎞️video-render-program/🦀️.rs"]
mod video_render_program_tests;

//#endregion 🎞️VideoRenderProgram
//#region 📤️FileOpenImport
#[cfg(test)]
#[path = "🧪️tests/📤️file-open-import/🦀️.rs"]
mod file_open_import_tests;

//#endregion 📤️FileOpenImport
//#region 🧵️SpawnedJobDrive
#[cfg(test)]
#[path = "🧪️tests/🧵️spawned-job-drive/🦀️.rs"]
mod spawned_job_drive_tests;

//#endregion 🧵️SpawnedJobDrive
//#endregion 🔖️Effect
//#region 🔖️TurnResult
#[cfg(test)]
#[path = "🧪️tests/🗣️return-content-dialects/🦀️.rs"]
mod return_content_dialect_tests;

#[cfg(test)]
#[path = "📤️return/📦️content/🖼️framing/🧪️tests/🔬️standalone/🦀️.rs"]
mod return_content_framing_tests;

#[cfg(test)]
#[path = "📤️return/📦️content/💌️message/🧪️tests/💌️message/🦀️.rs"]
mod return_content_message_tests;

#[cfg(test)]
#[path = "📤️return/🏠️source/🧪️tests/🔬️standalone/🦀️.rs"]
mod return_source_inventory_tests;

#[cfg(test)]
#[path = "📤️return/🏠️source/📚️entries/🧪️tests/📚️entries/🦀️.rs"]
mod return_source_entries_tests;

#[cfg(test)]
#[path = "📤️return/🏠️source/📚️entries/🦀️.rs"]
mod return_source_entries;
//#endregion 🔖️TurnResult
