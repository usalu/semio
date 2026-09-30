//! 🧠️ Local-first action kernel contracts: actions, operations, capabilities, window I/O.

use crate::manifest::MediaType;
use dsl::DslValue;
pub use dsl::{Diagnostic, Fault, FaultCause, FaultCode, FaultFrom, FaultOrigin, FaultScope, Severity};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};
use ui_wgpu::wgpu::UiNode;

//#region 🔖️Identifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct ArtifactHandle(pub u128);

/// 🌉️ Hand-written, not derived: `u128` has no `ToValue`/`FromValue` scalar impl (no JavaScript
/// number equivalent), so the mirror carries it as a decimal string — same treatment
/// `BrokerCapabilityGrant.token` gets in `🎠️kernel/🦀️.rs`'s own doc.
impl dsl::ToValue for ArtifactHandle {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.0.to_string())
    }
}
impl dsl::FromValue for ArtifactHandle {
    fn from_value(value: DslValue) -> Result<Self, dsl::ValueError> {
        match value {
            DslValue::String(s) => s.parse().map(ArtifactHandle).map_err(|_| dsl::ValueError::new(format!("expected a u128 decimal string for ArtifactHandle, found {s:?}"))),
            other => Err(dsl::ValueError::new(format!("expected a string for ArtifactHandle, found {other:?}"))),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct WindowHandle(pub u128);

/// 🌉️ Hand-written, not derived — see [`ArtifactHandle`]'s impl doc directly above (same `u128`
/// decimal-string mirror).
impl dsl::ToValue for WindowHandle {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.0.to_string())
    }
}
impl dsl::FromValue for WindowHandle {
    fn from_value(value: DslValue) -> Result<Self, dsl::ValueError> {
        match value {
            DslValue::String(s) => s.parse().map(WindowHandle).map_err(|_| dsl::ValueError::new(format!("expected a u128 decimal string for WindowHandle, found {s:?}"))),
            other => Err(dsl::ValueError::new(format!("expected a string for WindowHandle, found {other:?}"))),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AssetHandle(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CapabilityToken(pub u128);

/// 🌉️ Hand-written, not derived — see [`ArtifactHandle`]'s impl doc above (same `u128`
/// decimal-string mirror).
impl dsl::ToValue for CapabilityToken {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.0.to_string())
    }
}
impl dsl::FromValue for CapabilityToken {
    fn from_value(value: DslValue) -> Result<Self, dsl::ValueError> {
        match value {
            DslValue::String(s) => s.parse().map(CapabilityToken).map_err(|_| dsl::ValueError::new(format!("expected a u128 decimal string for CapabilityToken, found {s:?}"))),
            other => Err(dsl::ValueError::new(format!("expected a string for CapabilityToken, found {other:?}"))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct PluginInstanceId(pub String);

// 🎞️ CW3 kernel cut-over: MutationId/ActorId/ArtifactId/ArtifactVersion/SchemaId moved to
// `protocol_core` (frozen contract `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md`),
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
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct InvocationId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct ActionId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct CommandId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct AppInstanceId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaVersion(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum Rights {
    Read,
    Write,
    Invoke,
    Open,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum ArtifactKind {
    Document,
    Projection,
    Window,
    Asset,
    Network,
    Backbone,
    Engine,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum Scope {
    Instance,
    App,
    Plugin,
    Global,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CapabilityRequirement {
    pub artifact: ArtifactKind,
    pub rights: Rights,
    pub scope: Scope,
}

// 🚧️ BLOCKED (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): `artifact:
// ArtifactId` below is `protocol_core::ArtifactId` — defined in the `semio-framework-replication`
// crate (`📡️replication/🆔️ids/🦀️.rs`), which implements its OWN `crate::value::ToValue`
// (`🌱️value/🦀️.rs`, separately path-mounted into that crate) — a structurally-identical but
// nominally DISTINCT trait from this crate's `dsl::ToValue` (`os_dsl::schema::{ToValue,
// FromValue}`, canonical definition per `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs`
// L332-347). Confirmed via source inspection, not `cargo check` alone: `ArtifactId` does not
// implement `dsl::ToValue`, so `#[derive(ToValue, FromValue)]` here would fail. Fixing it means
// giving replication's frozen protocol newtypes a second, os-kernel-flavored `ToValue`/`FromValue`
// impl — out of this pass's scope (replication is a frozen contract, not a named target of this
// ticket). Left serde-only.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct Capability {
    pub subject: PluginInstanceId,
    pub artifact: ArtifactId,
    pub rights: Rights,
    pub scope: Scope,
}

// 🤝️ Keeps pace with `Capability` above, exactly as that type's own note prescribes: both of this
// struct's field types now carry `ToValue`/`FromValue` (`Capability` by derive, `CapabilityToken` by
// the hand-written u128 decimal-string mirror at `🔖️Capability`'s head), so the blocker the previous
// note described is discharged and the derive follows.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct ActionInvocation {
    pub id: InvocationId,
    pub app: AppInstanceId,
    pub action: ActionId,
    pub input: DslValue,
    pub actor: ActorId,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub causal_context: Vec<MutationId>,
}

/// 🎛️ A dispatched invocation of a `CommandDefinition` — the command mirror of `ActionInvocation`.
/// No `causal_context`: commands are not chained off a prior operation the way an action's follow-up can be.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
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
//#endregion 🔖️Clipboard

//#region 🔖️Effect
/// 🎫️ Correlates an `Effect` that expects a completion with the `Event::Completed` (or
/// `Event::HttpChunk`/`JobProgress`/`JobCompleted`) that answers it — `request-id = u64` in
/// `📜️wit/📜️types.wit`. Minted host-side per pending request; the guest SDK's request registry
/// (`📓️design-abi.md` §4) parks a future on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
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
    /// 🔁️ Asks the shell to invoke an extension capability. `req` is MINTED by the guest
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

    // --- new variants (📓️design-abi.md §2's table; nothing constructs these yet) ---
    /// 📨️ Replaces every non-UI/non-event `AppFrame::*` plus `backbone-send` — `target`
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
    /// 🧵️ Routed by the host `IoRouter` to the owning plugin as an `Event::Request` — one
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
    /// 💡️ Asks the shell to open its own host-owned ephemeral inference port for the active
    /// document and offer one reviewable proposal. It carries no document id, no space id, no
    /// idempotency key, no receipt and no credential: the shell already owns the document scope, it
    /// mints the request identity, it holds every lifecycle state, and it alone decides whether the
    /// document's execution-target lease permits the port to start. Nothing this effect starts is
    /// ever persisted into the document — the eventual proposal reaches the artifact only through
    /// the server-stamped approval command, never through this effect's own result.
    RequestInferenceProposal {
        kind: InferenceProposalKind,
    },
}

impl Effect {
    /// 🔁️ The single constructor for the `#[non_exhaustive]` [`Effect::InvokeExtension`] variant —
    /// `req` MUST come from the guest `RequestRegistry` (`RequestRegistry::request_continuation`)
    /// or from a host-side WIT lift of an id the guest already minted. Hand-written ids resolve
    /// nothing, which is exactly the foot-gun the variant's `#[non_exhaustive]` closes.
    pub fn invoke_extension(req: RequestId, extension_id: String, capability: String, request_json: String) -> Self {
        Self::InvokeExtension { req, extension_id, capability, request_json }
    }
}

/// 💡️ The closed set of host-owned inference proposals a program may ask its shell to open. It is
/// deliberately an intent, not a job description: no model, provider, prompt, budget or transport
/// is nameable here, so a program can never widen what the shell will actually run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub enum InferenceProposalKind {
    GisMapBoundsRegion,
}

/// 🚦 Where a spawned job runs — `📓️design-abi.md` §2's `spawn-job.placement`: `Inline` shares
/// the instance's own turn budget, `Isolated` gets its own pooled actor, `Exclusive` gets a
/// dedicated one (e.g. flow/brep tessellation, per `📓️design-abi.md` §5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum JobPlacement {
    Inline,
    Isolated,
    Exclusive,
}

//#region 🧵️SpawnedJobDrive
impl JobPlacement {
    /// 🚦 The WIT `job-placement` case name this placement crosses the actor boundary as
    /// (`🔌️plugin/🧬️schema/📜️.wit`'s `enum job-placement`). jco hands a bare string for a WIT `enum`,
    /// not a `{tag}` record, so a renderer door reads THIS vocabulary and nothing else.
    pub fn wire_name(self) -> &'static str {
        match self {
            Self::Inline => "inline",
            Self::Isolated => "isolated",
            Self::Exclusive => "exclusive",
        }
    }

    /// 🚦 The inverse of [`Self::wire_name`]. An unknown spelling is `None` rather than a silent
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

    /// 🚦 The whole vocabulary, in WIT declaration order — what a language-agnostic law enumerates.
    pub const ALL: [Self; 3] = [Self::Inline, Self::Isolated, Self::Exclusive];
}

/// 🧰️ The `Effect::SpawnJob` kind of every framework reserved tool verb (undo, redo, copy/paste,
/// selection and interaction verbs): a live job each host starts on the spawning instance, steps to its
/// end and answers with `Event::JobCompleted`, never a replayable product job. Twin of TypeScript
/// `FRAMEWORK_RESERVED_JOB_KIND` in `🎠️kernel/🟦️.ts`; both are read from `🧫️fixtures/🧵️spawned-job-drive`.
pub const FRAMEWORK_RESERVED_JOB_KIND: &str = "framework.reserved.tool";

/// 🧵 How many `step-job` observations ONE host admission of a spawned job may take before it gives
/// up. A framework reserved tool job (`interactionSelect`/`interactionHover`/`clearSelection`) reaches
/// `Done` in two; the ceiling is the host's patience, not the job's expected cost.
pub const SPAWNED_JOB_STEP_CEILING: u32 = 32;

/// 🧵 The fuel one `step-job` is granted. Deliberately the host's STATIC budget rather than a lane
/// grant: a reserved tool job is an interaction the user is waiting on, and a budget that varied per
/// call would make "did the selection apply?" depend on scheduler weather.
pub const SPAWNED_JOB_FUEL: u64 = 50_000_000;

/// 🧵 The wall deadline one `step-job` is granted, in milliseconds.
pub const SPAWNED_JOB_DEADLINE_MS: u32 = 100;

/// 🧵 One observation of the guest's `jobs::step-job` export, in the WIT `job-step` vocabulary.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum SpawnedJobStep {
    Running,
    Done { value: Vec<u8> },
    Failed { value: Vec<u8> },
}

/// 🧵 What the host owes the guest once a spawned job reached a terminal step: the `Event::JobCompleted`
/// payload, plus how many steps it took to get there (the number a budget law reads).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SpawnedJobCompletion {
    pub steps: u32,
    pub outcome: RequestOutcome,
}

/// 🧵 Why a spawned job produced no completion. Both are loud: a host that silently stopped pumping
/// is exactly the defect this contract closes — every `interactionSelect`/`interactionHover` on the
/// wgpu target admitted its reserved tool job and then never ran it, so hover and selection were
/// published perfectly and applied never (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-world3d-interaction-2026-09-13.md` §7).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SpawnedJobDriveError {
    /// ⏳️ [`SPAWNED_JOB_STEP_CEILING`] observations passed with no terminal step.
    Stalled { steps: u32 },
    /// 📏️ More observations than the ceiling admits — a driver bug, never a guest one.
    Overrun { steps: u32 },
}

impl std::fmt::Display for SpawnedJobDriveError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Stalled { steps } => write!(formatter, "spawned job did not reach a terminal step within {steps} host steps"),
            Self::Overrun { steps } => write!(formatter, "spawned job drive took {steps} steps, past the {SPAWNED_JOB_STEP_CEILING}-step ceiling"),
        }
    }
}

impl std::error::Error for SpawnedJobDriveError {}

/// 🧵 The ONE rule that turns a transcript of `step-job` observations into the `Event::JobCompleted`
/// the host owes the guest. Every host on every renderer answers through this, and its TypeScript
/// twin `spawnedJobCompletion` in `🎠️kernel/🟦️.ts` answers identically; both drive
/// `🧫️fixtures/🧵️spawned-job-drive/🔣️.json`.
///
/// Observations AFTER the first terminal step are ignored rather than refused: a driver that stops
/// stepping the instant it sees `Done` and one that read a batch both hand over the same completion.
pub fn spawned_job_completion(steps: &[SpawnedJobStep]) -> Result<SpawnedJobCompletion, SpawnedJobDriveError> {
    if steps.len() as u32 > SPAWNED_JOB_STEP_CEILING {
        return Err(SpawnedJobDriveError::Overrun { steps: steps.len() as u32 });
    }
    for (index, step) in steps.iter().enumerate() {
        let outcome = match step {
            SpawnedJobStep::Running => continue,
            SpawnedJobStep::Done { value } => RequestOutcome::Ok(value.clone()),
            SpawnedJobStep::Failed { value } => RequestOutcome::Err(value.clone()),
        };
        return Ok(SpawnedJobCompletion { steps: index as u32 + 1, outcome });
    }
    Err(SpawnedJobDriveError::Stalled { steps: steps.len() as u32 })
}

/// 🧵 The event a completed spawned job owes its actor — the same correlation the guest's own reactor
/// reads (`⚛️reactor/🔄️turn/🦀️.rs`: the `job` id IS the parked request id, so no host-side table
/// exists or is needed).
pub fn spawned_job_completed_event(job: u64, completion: &SpawnedJobCompletion) -> Event {
    Event::JobCompleted { job, result: completion.outcome.clone() }
}

#[cfg(test)]
#[path = "🧪️tests/🧵️spawned-job-drive/🦀️.rs"]
mod spawned_job_drive_tests;
//#endregion 🧵️SpawnedJobDrive

/// 🖼️ One icon-render export request: the destination filename plus the opaque icon-scene
/// render request forwarded to the shell's `iconRenderPort`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct IconRenderExportItem {
    pub filename: String,
    pub request: DslValue,
}

//#region ⬇️MediaExportEncoding
/// ⬇️ The only `Effect::DownloadMediaExport::encoding` value that means "`data` is not text".
/// Declared here, beside the effect it qualifies, so no shell can invent a second spelling.
pub const MEDIA_EXPORT_BASE64_ENCODING: &str = "base64";

/// ⬇️ The textual encoding a producer may state EXPLICITLY — `puzzle3d`'s `exportFixture` does
/// (`✏️s/🔌️plugins/🧩️puzzle/…/📤️export-fixture/🦀️.rs`). It means exactly what an absent `encoding`
/// means: `data` IS the file. Named rather than left to fall through an "anything that is not
/// base64 is text" branch, which is how a genuinely unknown encoding used to be saved as text.
pub const MEDIA_EXPORT_UTF8_ENCODING: &str = "utf-8";

/// ⬇️ Why a `DownloadMediaExport` envelope could not become bytes. Both are loud on purpose: a shell
/// that quietly saved the envelope's `data` as text is exactly the defect this contract closes —
/// every binary export in the repo reached disk as base64 TEXT under a binary file name
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️io-surface-2026-09-13.md` §7.3).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MediaExportEncodingError {
    /// 🗣️ An encoding no shell knows how to read. `None` and [`MEDIA_EXPORT_BASE64_ENCODING`] are the
    /// whole vocabulary; a segmented-download handle is consumed by the segmented lane before it
    /// ever reaches here.
    Unsupported { encoding: String },
    /// 🔤️ `encoding` said base64 and `data` is not canonical RFC 4648 standard base64.
    Malformed { detail: String },
}

impl std::fmt::Display for MediaExportEncodingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported { encoding } => write!(formatter, "unsupported media-export encoding {encoding:?}"),
            Self::Malformed { detail } => write!(formatter, "malformed base64 media export ({detail})"),
        }
    }
}

impl std::error::Error for MediaExportEncodingError {}

/// ⬇️ The ONE rule that turns a [`Effect::DownloadMediaExport`] envelope into the bytes the user
/// saves — no `encoding` (or [`MEDIA_EXPORT_UTF8_ENCODING`]) means `data` IS the file, and
/// [`MEDIA_EXPORT_BASE64_ENCODING`] means `data` carries the bytes. Every shell on every renderer answers through this, and its
/// TypeScript twin `mediaExportBytes` in `🎠️kernel/🟦️.ts` answers identically; both drive
/// `🧫️fixtures/⬇️media-export-encoding/🔣️.json`.
pub fn media_export_bytes(data: &str, encoding: Option<&str>) -> Result<Vec<u8>, MediaExportEncodingError> {
    match encoding {
        None | Some(MEDIA_EXPORT_UTF8_ENCODING) => Ok(data.as_bytes().to_vec()),
        Some(MEDIA_EXPORT_BASE64_ENCODING) => semio_framework_io_base64::base64_standard_decode(data).map_err(|error| MediaExportEncodingError::Malformed { detail: error.to_string() }),
        Some(other) => Err(MediaExportEncodingError::Unsupported { encoding: other.to_string() }),
    }
}

#[cfg(test)]
#[path = "🧪️tests/⬇️media-export-encoding/🦀️.rs"]
mod media_export_encoding_tests;
//#endregion ⬇️MediaExportEncoding

//#region 🎞️VideoRenderProgram
/// 🎟️ The host capability a plugin requests (`CapabilityRequest.id` in its package descriptor) before any host renders
/// its [`Effect::VideoRenderExport`]: the host lends its canvas and its encoder only to plugins that declared the ask, and
/// answers every other plugin with the `capability` refusal of `🧫️fixtures/🎞️video-render-job/🔣️.json`.
pub const MEDIA_VIDEO_RENDER_CAPABILITY: &str = "media.video-render";

/// 🎞️ The schema id every [`VideoRenderProgram`] states; `🧫️fixtures/🎞️video-render-program/🔣️.json` is its
/// language-agnostic law, `videoRenderProgramProblem` in `🎠️kernel/🟦️.ts` its TypeScript twin.
pub const VIDEO_RENDER_PROGRAM_SCHEMA: &str = "semio.video-render.program.v1";

/// 📏️ The largest picture edge a program may ask for (the raster video tier's own ceiling).
pub const VIDEO_RENDER_PROGRAM_MAXIMUM_EDGE: u32 = 4096;

/// 🎞️ The fastest frame rate a program may ask for.
pub const VIDEO_RENDER_PROGRAM_MAXIMUM_FPS: u32 = 120;

/// ⏱️ The most frames one program may render (ten minutes at 60 fps).
pub const VIDEO_RENDER_PROGRAM_MAXIMUM_FRAMES: u64 = 36_000;

/// ✏️ One path in a program's shared path table: `verbs` is one letter per segment (`M` move, `L` line,
/// `Q` quadratic, `C` cubic, `Z` close), `points` the flat `x, y` pairs they consume (1, 1, 2, 3, 0).
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct VideoRenderPath {
    pub verbs: String,
    pub points: Vec<f64>,
}

/// 🖼️ One picture a program draws from: a same-origin absolute path (`/🖼️assets/…`) or a `data:image/…` URL, never a
/// foreign origin — a host fetches it itself, and a cross-origin picture would taint the canvas it encodes from.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct VideoRenderImage {
    pub url: String,
}

/// 🎨️ One paint operation of a frame, composited in list order over the program background. Every `transform` is
/// `[a, b, c, d, e, f]` (`x' = a x + c y + e`, `y' = b x + d y + f`) into device pixels, y down; every colour is straight
/// RGBA in `0..=1`. `Fill`/`Stroke` paint path `path` of the table (a stroke `width` path units wide); `Image` draws the
/// `crop` (`[x, y, width, height]`, normalised to the picture) of image `image` onto the unit square that `transform`
/// places, at `opacity`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum VideoRenderOp {
    Fill { path: u32, transform: [f64; 6], color: [f64; 4] },
    Stroke { path: u32, transform: [f64; 6], color: [f64; 4], width: f64 },
    Image { image: u32, crop: [f64; 4], transform: [f64; 6], opacity: f64 },
}

/// 🖼️ One distinct picture: its paint operations over the program background.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct VideoRenderScene {
    pub ops: Vec<VideoRenderOp>,
}

/// ⏯️ Scene `scene` shown for `frames` consecutive frames.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct VideoRenderRun {
    pub scene: u32,
    pub frames: u32,
}

/// 🎞️ Everything a host needs to render a video without asking the guest again: picture size, frame rate,
/// background, a shared path table, the pictures it draws from, the distinct scenes and the run-length timeline that
/// plays them. Identical consecutive frames are one run, so a still slide costs one scene however long it plays.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct VideoRenderProgram {
    pub schema: String,
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    pub background: [f64; 4],
    pub paths: Vec<VideoRenderPath>,
    pub images: Vec<VideoRenderImage>,
    pub scenes: Vec<VideoRenderScene>,
    pub timeline: Vec<VideoRenderRun>,
}

/// 🚨️ Why a host refuses a [`VideoRenderProgram`]; `code()` is the fixture's language-agnostic vocabulary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VideoRenderProgramError {
    Schema { schema: String },
    Dimensions { width: u32, height: u32 },
    FrameRate { fps: u32 },
    Empty,
    TooLong { frames: u64 },
    Timeline { run: usize },
    PathVerbs { path: usize },
    ImageUrl { image: usize },
    PathIndex { scene: usize, op: usize },
    ImageIndex { scene: usize, op: usize },
    Paint { scene: usize, op: usize },
}

impl VideoRenderProgramError {
    /// 🔤️ The fixture's error code for this refusal.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Schema { .. } => "schema",
            Self::Dimensions { .. } => "dimensions",
            Self::FrameRate { .. } => "frameRate",
            Self::Empty => "empty",
            Self::TooLong { .. } => "tooLong",
            Self::Timeline { .. } => "timeline",
            Self::PathVerbs { .. } => "pathVerbs",
            Self::ImageUrl { .. } => "imageUrl",
            Self::PathIndex { .. } => "pathIndex",
            Self::ImageIndex { .. } => "imageIndex",
            Self::Paint { .. } => "paint",
        }
    }
}

impl std::fmt::Display for VideoRenderProgramError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Schema { schema } => write!(formatter, "video program schema {schema:?} is not {VIDEO_RENDER_PROGRAM_SCHEMA}"),
            Self::Dimensions { width, height } => write!(formatter, "video program picture {width}x{height} is not an even size within 2..={VIDEO_RENDER_PROGRAM_MAXIMUM_EDGE}"),
            Self::FrameRate { fps } => write!(formatter, "video program frame rate {fps} is outside 1..={VIDEO_RENDER_PROGRAM_MAXIMUM_FPS}"),
            Self::Empty => formatter.write_str("video program has no frames"),
            Self::TooLong { frames } => write!(formatter, "video program has {frames} frames, more than {VIDEO_RENDER_PROGRAM_MAXIMUM_FRAMES}"),
            Self::Timeline { run } => write!(formatter, "video program timeline run {run} names no scene or shows it for zero frames"),
            Self::PathVerbs { path } => write!(formatter, "video program path {path} has verbs its points do not match"),
            Self::ImageUrl { image } => write!(formatter, "video program image {image} is neither a same-origin path nor a data:image URL"),
            Self::PathIndex { scene, op } => write!(formatter, "video program scene {scene} op {op} names no path"),
            Self::ImageIndex { scene, op } => write!(formatter, "video program scene {scene} op {op} names no image"),
            Self::Paint { scene, op } => write!(formatter, "video program scene {scene} op {op} has a colour, transform, width, crop or opacity out of range"),
        }
    }
}

impl std::error::Error for VideoRenderProgramError {}

impl VideoRenderProgram {
    /// 🎞️ Frames the timeline plays.
    pub fn frame_count(&self) -> u64 {
        self.timeline.iter().map(|run| u64::from(run.frames)).sum()
    }

    /// ⏱️ Playing time in milliseconds, rounded half up — what the encoded MP4 states.
    pub fn duration_milliseconds(&self) -> u64 {
        (self.frame_count() * 1000 + u64::from(self.fps) / 2) / u64::from(self.fps.max(1))
    }

    /// 🚦️ The ONE admission rule every host applies before it renders a frame.
    pub fn validate(&self) -> Result<(), VideoRenderProgramError> {
        if self.schema != VIDEO_RENDER_PROGRAM_SCHEMA {
            return Err(VideoRenderProgramError::Schema { schema: self.schema.clone() });
        }
        let edge = |value: u32| (2..=VIDEO_RENDER_PROGRAM_MAXIMUM_EDGE).contains(&value) && value % 2 == 0;
        if !edge(self.width) || !edge(self.height) {
            return Err(VideoRenderProgramError::Dimensions { width: self.width, height: self.height });
        }
        if self.fps == 0 || self.fps > VIDEO_RENDER_PROGRAM_MAXIMUM_FPS {
            return Err(VideoRenderProgramError::FrameRate { fps: self.fps });
        }
        let frames = self.frame_count();
        if frames == 0 {
            return Err(VideoRenderProgramError::Empty);
        }
        if frames > VIDEO_RENDER_PROGRAM_MAXIMUM_FRAMES {
            return Err(VideoRenderProgramError::TooLong { frames });
        }
        if let Some(run) = self.timeline.iter().position(|run| run.frames == 0 || run.scene as usize >= self.scenes.len()) {
            return Err(VideoRenderProgramError::Timeline { run });
        }
        if let Some(path) = self.paths.iter().position(|path| !video_render_path_is_well_formed(path)) {
            return Err(VideoRenderProgramError::PathVerbs { path });
        }
        if let Some(image) = self.images.iter().position(|image| !video_render_image_url_is_admitted(&image.url)) {
            return Err(VideoRenderProgramError::ImageUrl { image });
        }
        if !self.background.iter().all(|value| video_render_unit(*value)) {
            return Err(VideoRenderProgramError::Paint { scene: 0, op: 0 });
        }
        let finite = |values: &[f64]| values.iter().all(|value| value.is_finite());
        for (scene_index, scene) in self.scenes.iter().enumerate() {
            for (op_index, op) in scene.ops.iter().enumerate() {
                let (reference, painted) = match op {
                    VideoRenderOp::Fill { path, transform, color } => ((*path as usize) < self.paths.len(), finite(transform) && color.iter().all(|value| video_render_unit(*value))),
                    VideoRenderOp::Stroke { path, transform, color, width } => ((*path as usize) < self.paths.len(), finite(transform) && color.iter().all(|value| video_render_unit(*value)) && width.is_finite() && *width >= 0.0),
                    VideoRenderOp::Image { image, crop, transform, opacity } => ((*image as usize) < self.images.len(), finite(transform) && video_render_crop_is_admitted(crop) && video_render_unit(*opacity)),
                };
                if !reference {
                    return Err(match op {
                        VideoRenderOp::Image { .. } => VideoRenderProgramError::ImageIndex { scene: scene_index, op: op_index },
                        _ => VideoRenderProgramError::PathIndex { scene: scene_index, op: op_index },
                    });
                }
                if !painted {
                    return Err(VideoRenderProgramError::Paint { scene: scene_index, op: op_index });
                }
            }
        }
        Ok(())
    }
}

fn video_render_unit(value: f64) -> bool {
    value.is_finite() && (0.0..=1.0).contains(&value)
}

/// ✏️ A path's verbs consume exactly its points, start with a move and every coordinate is finite.
pub fn video_render_path_is_well_formed(path: &VideoRenderPath) -> bool {
    let mut needed = 0usize;
    for (index, verb) in path.verbs.chars().enumerate() {
        needed += match verb {
            'M' => 2,
            'L' => 2,
            'Q' => 4,
            'C' => 6,
            'Z' => 0,
            _ => return false,
        };
        if index == 0 && verb != 'M' {
            return false;
        }
    }
    needed == path.points.len() && path.points.iter().all(|value| value.is_finite())
}

/// 🖼️ An image URL a host may fetch for a program: a same-origin absolute path (`/…`, never the protocol-relative
/// `//…`) or an inline `data:image/…` URL.
pub fn video_render_image_url_is_admitted(url: &str) -> bool {
    (url.starts_with('/') && !url.starts_with("//")) || url.starts_with("data:image/")
}

/// ✂️ A crop `[x, y, width, height]` lies inside the unit picture and covers a non-empty area.
pub fn video_render_crop_is_admitted(crop: &[f64; 4]) -> bool {
    let [x, y, width, height] = *crop;
    crop.iter().all(|value| value.is_finite()) && x >= 0.0 && y >= 0.0 && width > 0.0 && height > 0.0 && x + width <= 1.0 + 1e-9 && y + height <= 1.0 + 1e-9
}

#[cfg(test)]
#[path = "🧪️tests/🎞️video-render-program/🦀️.rs"]
mod video_render_program_tests;
//#endregion 🎞️VideoRenderProgram

//#region 🧵️VideoRenderJob
/// 🧩️ Which encoder produced a finished render: the host's platform encoder (the OS or browser H.264 encoder plugged in
/// behind the raster video tier's port) or the tier's own first-party encoder.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "kebab-case")]
#[value(rename_all = "kebab-case")]
pub enum VideoRenderEncoderTier {
    Platform,
    FirstParty,
}

/// 🏁️ How one host video render job ended.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "status", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "status", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum VideoRenderJobOutcome {
    Done { bytes: u64, tier: VideoRenderEncoderTier },
    Cancelled { completed: u64 },
    Refused { code: String },
    Failed { reason: String },
}

/// 🧵️ One fact of a host video render job's life — ephemeral, local-only state of the host that renders it. A host
/// never mutates its task list: it appends one of these and the task list is the fold of the log
/// ([`VideoRenderJobLedger`]); a cancel is a command that appends [`Self::CancelRequested`], which the render loop reads
/// back from the ledger before its next frame. `🧫️fixtures/🎞️video-render-job/🔣️.json` is the law both twins fold.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(tag = "kind", rename_all = "camelCase", rename_all_fields = "camelCase")]
pub enum VideoRenderJobEvent {
    Started { job: u64, owner: String, filename: String, frames: u64, at_ms: u64 },
    Progressed { job: u64, completed: u64 },
    CancelRequested { job: u64 },
    Finished { job: u64, outcome: VideoRenderJobOutcome },
}

/// 🧵️ One running job as a task list shows it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoRenderJobRow {
    pub job: u64,
    pub owner: String,
    pub filename: String,
    pub frames: u64,
    pub completed: u64,
    pub cancelling: bool,
    pub started_at_ms: u64,
}

/// 🚨️ Why a [`VideoRenderJobLedger`] refuses an event; `code()` is the fixture's vocabulary.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum VideoRenderJobEventError {
    StaleJob { job: u64, last: u64 },
    UnknownJob { job: u64 },
    ProgressRegressed { job: u64, completed: u64, previous: u64 },
    ProgressOverrun { job: u64, completed: u64, frames: u64 },
    AlreadyCancelling { job: u64 },
}

impl VideoRenderJobEventError {
    /// 🔤️ The fixture's error code for this refusal.
    pub fn code(&self) -> &'static str {
        match self {
            Self::StaleJob { .. } => "staleJob",
            Self::UnknownJob { .. } => "unknownJob",
            Self::ProgressRegressed { .. } => "progressRegressed",
            Self::ProgressOverrun { .. } => "progressOverrun",
            Self::AlreadyCancelling { .. } => "alreadyCancelling",
        }
    }
}

impl std::fmt::Display for VideoRenderJobEventError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StaleJob { job, last } => write!(formatter, "video render job {job} does not follow job {last}"),
            Self::UnknownJob { job } => write!(formatter, "video render job {job} is not running"),
            Self::ProgressRegressed { job, completed, previous } => write!(formatter, "video render job {job} went back from frame {previous} to {completed}"),
            Self::ProgressOverrun { job, completed, frames } => write!(formatter, "video render job {job} reports frame {completed} of {frames}"),
            Self::AlreadyCancelling { job } => write!(formatter, "video render job {job} is already cancelling"),
        }
    }
}

impl std::error::Error for VideoRenderJobEventError {}

/// 📒️ The fold of a host's [`VideoRenderJobEvent`] log: the running jobs in start order and the last job id issued.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct VideoRenderJobLedger {
    last_job: u64,
    running: Vec<VideoRenderJobRow>,
}

impl VideoRenderJobLedger {
    /// 📒️ Folds a whole log; the error names the index of the first event the ledger refused.
    pub fn fold<'a>(events: impl IntoIterator<Item = &'a VideoRenderJobEvent>) -> Result<Self, (usize, VideoRenderJobEventError)> {
        let mut ledger = Self::default();
        for (index, event) in events.into_iter().enumerate() {
            ledger.apply(event).map_err(|error| (index, error))?;
        }
        Ok(ledger)
    }

    /// ➕️ Applies one event, or refuses it and leaves the ledger as it was.
    pub fn apply(&mut self, event: &VideoRenderJobEvent) -> Result<(), VideoRenderJobEventError> {
        match event {
            VideoRenderJobEvent::Started { job, owner, filename, frames, at_ms } => {
                if *job <= self.last_job {
                    return Err(VideoRenderJobEventError::StaleJob { job: *job, last: self.last_job });
                }
                self.last_job = *job;
                self.running.push(VideoRenderJobRow { job: *job, owner: owner.clone(), filename: filename.clone(), frames: *frames, completed: 0, cancelling: false, started_at_ms: *at_ms });
            }
            VideoRenderJobEvent::Progressed { job, completed } => {
                let row = self.running.iter_mut().find(|row| row.job == *job).ok_or(VideoRenderJobEventError::UnknownJob { job: *job })?;
                if *completed < row.completed {
                    return Err(VideoRenderJobEventError::ProgressRegressed { job: *job, completed: *completed, previous: row.completed });
                }
                if *completed > row.frames {
                    return Err(VideoRenderJobEventError::ProgressOverrun { job: *job, completed: *completed, frames: row.frames });
                }
                row.completed = *completed;
            }
            VideoRenderJobEvent::CancelRequested { job } => {
                let row = self.running.iter_mut().find(|row| row.job == *job).ok_or(VideoRenderJobEventError::UnknownJob { job: *job })?;
                if row.cancelling {
                    return Err(VideoRenderJobEventError::AlreadyCancelling { job: *job });
                }
                row.cancelling = true;
            }
            VideoRenderJobEvent::Finished { job, .. } => {
                let index = self.running.iter().position(|row| row.job == *job).ok_or(VideoRenderJobEventError::UnknownJob { job: *job })?;
                self.running.remove(index);
            }
        }
        Ok(())
    }

    /// 🧵️ Every running job, in start order.
    pub fn running(&self) -> &[VideoRenderJobRow] {
        &self.running
    }

    /// 🔎️ Running job `job`, if any.
    pub fn row(&self, job: u64) -> Option<&VideoRenderJobRow> {
        self.running.iter().find(|row| row.job == job)
    }

    /// 🔢️ The last job id issued (`0` before the first).
    pub fn last_job(&self) -> u64 {
        self.last_job
    }
}

#[cfg(test)]
#[path = "🧪️tests/🧵️video-render-job/🦀️.rs"]
mod video_render_job_tests;
//#endregion 🧵️VideoRenderJob

//#region 📤️FileOpenImport
/// 📥️ Bytes ONE import chunk may carry to the guest.
///
/// 🧊️ Derived from the guest's per-request contiguous ceiling, never a literal: an import's
/// `payload` crosses as one string, and every guest hop that carries it asks for one contiguous
/// block, so a whole document sent as a single invocation asks the fixed guest heap for a block
/// several times that ceiling. Half the ceiling leaves the other half for the invocation envelope
/// the chunk rides in.
pub const IMPORT_CHUNK_BYTES: usize = semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / 2;

/// 📥️ The argument names one import chunk is dispatched with. Declared here, beside
/// [`Effect::RequestFileOpen`], so no shell can invent a second spelling of the same envelope —
/// the wgpu shell used to send `{json, payload}` and React `{payload, name, chunk, chunkCount}`,
/// and a plugin could satisfy only one of them.
pub const IMPORT_ARGUMENT_PAYLOAD: &str = "payload";
pub const IMPORT_ARGUMENT_NAME: &str = "name";
pub const IMPORT_ARGUMENT_CHUNK: &str = "chunk";
pub const IMPORT_ARGUMENT_CHUNK_COUNT: &str = "chunkCount";
pub const IMPORT_ARGUMENT_INDEX: &str = "index";
pub const IMPORT_ARGUMENT_TOTAL: &str = "total";

/// 📥️ One chunk of one opened file, positioned in its own run.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ImportChunk {
    pub payload: String,
    pub chunk: usize,
    pub chunk_count: usize,
}

/// 📥️ Slices one opened file's contents into the chunks a shell dispatches, so no single import
/// invocation asks the guest for a contiguous block above its own per-request ceiling.
///
/// 🔤️ Sliced by UTF-8 EXTENT, not by code units: the guest measures `text.len()` in bytes, so a
/// slice counted in UTF-16 units would overrun the cap by up to 3× on non-ASCII text. No slice ever
/// splits a code point, and an empty payload still yields exactly one chunk — a picked empty file is
/// a real pick the guest must be told about.
///
/// TypeScript twin: `importPayloadChunks` in `🎠️kernel/🟦️.ts`; both drive
/// `🧫️fixtures/📤️file-open-import/🔣️.json`.
pub fn import_payload_chunks(payload: &str) -> Vec<ImportChunk> {
    let mut pages: Vec<String> = Vec::new();
    let mut page = String::new();
    for character in payload.chars() {
        if page.len() + character.len_utf8() > IMPORT_CHUNK_BYTES {
            pages.push(core::mem::take(&mut page));
        }
        page.push(character);
    }
    if !page.is_empty() || pages.is_empty() {
        pages.push(page);
    }
    let chunk_count = pages.len();
    pages.into_iter().enumerate().map(|(chunk, payload)| ImportChunk { payload, chunk, chunk_count }).collect()
}

/// 📥️ The arguments ONE import chunk is dispatched with — `fan_out` is `Some((index, total))` only
/// when the picker was opened with `multiple`, exactly as React's `dispatchOpenedFiles` extends a
/// multi-file pick. Integers are minted as [`Number::UInt`] carriers, never floats: the guest's
/// [`ImportStaging`] reads `chunk`/`chunkCount` with `DslValue::as_u64`, which a `Float` never answers.
pub fn import_chunk_arguments(name: &str, chunk: &ImportChunk, fan_out: Option<(usize, usize)>) -> DslValue {
    let uint = |value: usize| DslValue::Number(dsl::os_dsl::schema::Number::UInt(value as u64));
    let mut entries = vec![
        (IMPORT_ARGUMENT_PAYLOAD.to_string(), DslValue::String(chunk.payload.clone())),
        (IMPORT_ARGUMENT_NAME.to_string(), DslValue::String(name.to_string())),
        (IMPORT_ARGUMENT_CHUNK.to_string(), uint(chunk.chunk)),
        (IMPORT_ARGUMENT_CHUNK_COUNT.to_string(), uint(chunk.chunk_count)),
    ];
    if let Some((index, total)) = fan_out {
        entries.push((IMPORT_ARGUMENT_INDEX.to_string(), uint(index)));
        entries.push((IMPORT_ARGUMENT_TOTAL.to_string(), uint(total)));
    }
    DslValue::Object(entries)
}

//#region 📥️ImportStaging
/// 📦️ Chunks one staged import may reassemble: 128 × [`IMPORT_CHUNK_BYTES`] = 4 MiB, the byte authority of one
/// document archive — the largest thing a person opens from a file — so no staged run outgrows one document.
pub const IMPORT_STAGING_MAXIMUM_CHUNKS: usize = 128;

/// 📦️ The bytes one staged run may hold, implied by [`IMPORT_STAGING_MAXIMUM_CHUNKS`] chunks of at most
/// [`IMPORT_CHUNK_BYTES`] each.
pub const IMPORT_STAGING_MAXIMUM_BYTES: usize = IMPORT_STAGING_MAXIMUM_CHUNKS * IMPORT_CHUNK_BYTES;

/// 🧵️ Runs one app instance stages at once — one per file a person is picking into this document. A new pick takes
/// the slot of the least recently advanced run, so an abandoned or cancelled pick holds memory only until then:
/// at most `IMPORT_STAGING_RUNS × IMPORT_STAGING_MAXIMUM_BYTES` per instance.
pub const IMPORT_STAGING_RUNS: usize = 2;

/// 🚫️ Why the staging refused one chunk — a typed code the dispatch turns into a fault, never a silent drop.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImportStagingRefusal {
    Envelope,
    Chunk,
    Gap,
}

impl ImportStagingRefusal {
    /// 🏷️ The stable fault code.
    pub fn code(self) -> &'static str {
        match self {
            Self::Envelope => "file-import.envelope",
            Self::Chunk => "file-import.chunk",
            Self::Gap => "file-import.gap",
        }
    }

    /// 🗣️ What a person is told.
    pub fn message(self) -> &'static str {
        match self {
            Self::Envelope => "The file arrived with an invalid chunk envelope, or is larger than one document may be.",
            Self::Chunk => "One part of the file is larger than one import chunk.",
            Self::Gap => "A part of the file arrived out of order; pick the file again.",
        }
    }
}

/// 📥️ What one action's arguments are to the staging: not an import chunk at all, a chunk of a run that is still open
/// (`next_chunk` of `chunk_count` is its progress), or the whole file — as the arguments the app's action decodes:
/// `payload` and `name` (plus `index`/`total` of a multi-file pick and every other argument the invocation carried),
/// never the chunk envelope.
#[derive(Clone, Debug, PartialEq)]
pub enum ImportArguments {
    NotAnImport,
    Staged { next_chunk: usize, chunk_count: usize },
    Whole(DslValue),
}

struct ImportRun {
    name: String,
    fan_out: Option<(usize, usize)>,
    chunk_count: usize,
    next_chunk: usize,
    pages: Vec<String>,
    touched: u64,
}

/// 🧵️ One app instance's open file-import runs: the framework half of every [`Effect::RequestFileOpen`] import, so no
/// app stages chunks itself. Instance-scoped, never process-global — two people importing into two documents of the
/// same component never see each other's bytes, and an instance that closes takes its runs with it. Pages stay
/// chunk-sized until the run closes.
#[derive(Default)]
pub struct ImportStaging {
    runs: Vec<ImportRun>,
    sequence: u64,
}

impl ImportStaging {
    /// 📥️ Admits the arguments one import action was dispatched with ([`import_chunk_arguments`]). A run is keyed by
    /// `(name, fan-out, chunk_count)`: a re-pick restarts its run at chunk 0, a chunk the run already admitted is a
    /// retransmission acknowledged at the cursor, any other out-of-order chunk drops the run as a gap.
    pub fn admit_args(&mut self, args: Option<&DslValue>) -> Result<ImportArguments, ImportStagingRefusal> {
        let Some(DslValue::Object(entries)) = args else {
            return Ok(ImportArguments::NotAnImport);
        };
        let field = |key: &str| entries.iter().find(|(name, _)| name == key).map(|(_, value)| value);
        let Some(count) = field(IMPORT_ARGUMENT_CHUNK_COUNT) else {
            return Ok(ImportArguments::NotAnImport);
        };
        let position = |value: Option<&DslValue>| value.and_then(DslValue::as_u64).and_then(|value| usize::try_from(value).ok());
        let chunk_count = position(Some(count)).ok_or(ImportStagingRefusal::Envelope)?;
        let chunk = position(field(IMPORT_ARGUMENT_CHUNK)).ok_or(ImportStagingRefusal::Envelope)?;
        let payload = field(IMPORT_ARGUMENT_PAYLOAD).and_then(DslValue::as_str).ok_or(ImportStagingRefusal::Envelope)?;
        let name = field(IMPORT_ARGUMENT_NAME).and_then(DslValue::as_str).ok_or(ImportStagingRefusal::Envelope)?;
        let fan_out = match (field(IMPORT_ARGUMENT_INDEX), field(IMPORT_ARGUMENT_TOTAL)) {
            (None, None) => None,
            (index, total) => match (position(index), position(total)) {
                (Some(index), Some(total)) if index < total => Some((index, total)),
                _ => return Err(ImportStagingRefusal::Envelope),
            },
        };
        if chunk_count == 0 || chunk_count > IMPORT_STAGING_MAXIMUM_CHUNKS || chunk >= chunk_count {
            return Err(ImportStagingRefusal::Envelope);
        }
        if payload.len() > IMPORT_CHUNK_BYTES {
            return Err(ImportStagingRefusal::Chunk);
        }
        let envelope = [IMPORT_ARGUMENT_PAYLOAD, IMPORT_ARGUMENT_NAME, IMPORT_ARGUMENT_CHUNK, IMPORT_ARGUMENT_CHUNK_COUNT, IMPORT_ARGUMENT_INDEX, IMPORT_ARGUMENT_TOTAL];
        let whole = |text: String| {
            let uint = |value: usize| DslValue::Number(dsl::os_dsl::schema::Number::UInt(value as u64));
            let mut arguments = vec![(IMPORT_ARGUMENT_PAYLOAD.to_string(), DslValue::String(text)), (IMPORT_ARGUMENT_NAME.to_string(), DslValue::String(name.to_string()))];
            if let Some((index, total)) = fan_out {
                arguments.push((IMPORT_ARGUMENT_INDEX.to_string(), uint(index)));
                arguments.push((IMPORT_ARGUMENT_TOTAL.to_string(), uint(total)));
            }
            arguments.extend(entries.iter().filter(|(key, _)| !envelope.contains(&key.as_str())).cloned());
            ImportArguments::Whole(DslValue::Object(arguments))
        };
        if chunk_count == 1 {
            return Ok(whole(payload.to_string()));
        }
        self.sequence = self.sequence.saturating_add(1);
        let sequence = self.sequence;
        let held = self.runs.iter().position(|run| run.name == name && run.fan_out == fan_out && run.chunk_count == chunk_count);
        let index = match held {
            Some(index) if self.runs[index].next_chunk == chunk => index,
            Some(index) if chunk != 0 && chunk < self.runs[index].next_chunk => {
                let run = &mut self.runs[index];
                run.touched = sequence;
                return Ok(ImportArguments::Staged { next_chunk: run.next_chunk, chunk_count: run.chunk_count });
            }
            Some(index) if chunk != 0 => {
                self.runs.swap_remove(index);
                return Err(ImportStagingRefusal::Gap);
            }
            Some(index) => {
                self.runs[index] = ImportRun { name: name.to_string(), fan_out, chunk_count, next_chunk: 0, pages: Vec::new(), touched: sequence };
                index
            }
            None if chunk != 0 => return Err(ImportStagingRefusal::Gap),
            None => {
                if self.runs.len() >= IMPORT_STAGING_RUNS {
                    if let Some(stalest) = self.runs.iter().enumerate().min_by_key(|(_, run)| run.touched).map(|(index, _)| index) {
                        self.runs.swap_remove(stalest);
                    }
                }
                self.runs.push(ImportRun { name: name.to_string(), fan_out, chunk_count, next_chunk: 0, pages: Vec::new(), touched: sequence });
                self.runs.len() - 1
            }
        };
        let run = &mut self.runs[index];
        run.pages.push(payload.to_string());
        run.next_chunk = chunk.saturating_add(1);
        run.touched = sequence;
        if run.next_chunk < run.chunk_count {
            return Ok(ImportArguments::Staged { next_chunk: run.next_chunk, chunk_count: run.chunk_count });
        }
        let closed = self.runs.swap_remove(index);
        Ok(whole(closed.pages.concat()))
    }

    /// 🛑️ Drops every open run — what closing the instance or a person's cancel does to picks still arriving.
    pub fn cancel_all(&mut self) {
        self.runs.clear();
    }

    /// 🔎️ Open runs as `(name, next_chunk, chunk_count, bytes)` — the progress census the laws read.
    pub fn open_runs(&self) -> Vec<(String, usize, usize, usize)> {
        self.runs.iter().map(|run| (run.name.clone(), run.next_chunk, run.chunk_count, run.pages.iter().map(String::len).sum())).collect()
    }
}
//#endregion 📥️ImportStaging


#[cfg(test)]
#[path = "🧪️tests/📤️file-open-import/🦀️.rs"]
mod file_open_import_tests;
//#endregion 📤️FileOpenImport
//#endregion 🔖️Effect

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct InverseMutation {
    pub target_mutation: MutationId,
    pub inverse_diff: ArtifactDiff,
    pub base_version: ArtifactVersion,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<MutationId>,
    pub undo_policy: UndoPolicy,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct KernelMutation {
    pub id: MutationId,
    pub document: ArtifactHandle,
    pub base_version: ArtifactVersion,
    pub invocation_id: InvocationId,
    pub diff: ArtifactDiff,
    pub inverse: InverseMutation,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub dependencies: Vec<MutationId>,
    pub author: ActorId,
    pub timestamp: HybridLogicalTimestamp,
}

/// 🧩️ One member edit folded into a group undo — pairs the owning document handle with the edit
/// id inside it, so `UndoGroup.member_edits` can name edits that live on documents other than the
/// group's own `invocation_id` target (composite/child-document dispatch, ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM `📓️design-full-plan.md` section "1. Kernel
/// primitives" — grouping). Additive only: nothing in this wave constructs one yet.
#[derive(Clone, Debug, PartialEq, Eq, Hash, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct EditRef {
    pub document: ArtifactHandle,
    pub edit_id: String,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct UndoGroup {
    pub invocation_id: InvocationId,
    pub mutations: Vec<MutationId>,
    pub inverse_mutations: Vec<InverseMutation>,
    /// 🧩️ Cross-document member edits folded into this group's undo (composite dispatch across
    /// parent + child documents) — additive, empty for every group that isn't composite.
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub member_edits: Vec<EditRef>,
}

/// 🐢️ What part of the shell's rendered UI an action actually invalidates — lets `refresh-ui`
/// skip re-rendering/re-fetching sections nothing touched. Absent from JSON (older/unmodified plugins)
/// deserializes to `Full`, so any program that never sets this keeps today's whole-shell-refresh
/// behavior exactly. `None` means "nothing to re-render at all" (e.g. a pure telemetry/heartbeat action).
// 🐢️ `rename_all = "camelCase"` alone only renames the *variant* names (Full/None/Partial ->
// full/none/partial via `tag = "kind"`) — it does NOT cascade into a struct variant's own fields, which
// would otherwise serialize as snake_case (`window_bodies`) and silently desync from the TS
// `UiDirtyScope` type's camelCase `windowBodies`. `rename_all_fields` is the attribute that renames
// fields *within* variants; both are needed together.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase", tag = "kind")]
pub enum UiDirtyScope {
    #[default]
    Full,
    None,
    Partial {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        window_bodies: Vec<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        #[value(default, skip_serializing_if = "Vec::is_empty")]
        panel_bodies: Vec<String>,
        #[serde(default)]
        #[value(default)]
        utilities: bool,
        #[serde(default)]
        #[value(default)]
        tools: bool,
        #[serde(default)]
        #[value(default)]
        engagements: bool,
        #[serde(default)]
        #[value(default)]
        measures: bool,
        #[serde(default)]
        #[value(default)]
        labels: bool,
    },
}

/// 🔖️ One flag-addressed section of a batched `refresh-ui` — the boolean fields of
/// [`UiDirtyScope::Partial`] named by value so ONE predicate serves all of them, on both renderers.
/// Windows and panels are addressed by body key instead ([`UiDirtyScope::wants_window_body`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UiDirtySection {
    Utilities,
    Tools,
    Engagements,
    Measures,
    Labels,
}

/// 🐢️ The selection law both shells answer to, and the union one coalesced refresh pass owes.
///
/// The React shell has always read a scope (`🛠️ShellHelpers/🟦️.tsx`'s `uiRefreshWants*` +
/// `buildUiRefreshRequest`); the wgpu shell's `refresh_ui` walked EVERY window and panel on every
/// settle and threw `InvocationResult::ui_scope` away — 116 of 137 renders per converging edit
/// answered `patched=0`, and the flow window was re-minted eight times for nothing
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-edit-convergence-perf-2026-09-14.md` §7).
/// These predicates are that shell's half of the same law, driven by the same fixture
/// (`🧫️fixtures/🐢️ui-dirty-scope/🔣️.json`) as the TypeScript twin in `🎠️kernel/🟦️.ts`.
impl UiDirtyScope {
    /// 🚫️ Nothing to re-render at all — the caller must not even open a refresh pass.
    pub fn asks_for_nothing(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn wants_window_body(&self, body_key: &str) -> bool {
        match self {
            Self::Full => true,
            Self::None => false,
            Self::Partial { window_bodies, .. } => window_bodies.iter().any(|body| body == body_key),
        }
    }

    pub fn wants_panel_body(&self, body_key: &str) -> bool {
        match self {
            Self::Full => true,
            Self::None => false,
            Self::Partial { panel_bodies, .. } => panel_bodies.iter().any(|body| body == body_key),
        }
    }

    pub fn wants_section(&self, section: UiDirtySection) -> bool {
        match self {
            Self::Full => true,
            Self::None => false,
            Self::Partial { utilities, tools, engagements, measures, labels, .. } => match section {
                UiDirtySection::Utilities => *utilities,
                UiDirtySection::Tools => *tools,
                UiDirtySection::Engagements => *engagements,
                UiDirtySection::Measures => *measures,
                UiDirtySection::Labels => *labels,
            },
        }
    }

    /// 🛍️ The app-static operator/palette catalogue never goes stale inside an app instance, so it
    /// carries no flag of its own: only a full scope — a session switch, or the first fetch — asks
    /// for it. Mirrors `uiRefreshWantsCatalogue`.
    pub fn wants_catalogue(&self) -> bool {
        matches!(self, Self::Full)
    }

    /// 🤝️ The scope ONE pass must cover when a second was asked for while the first was crossing
    /// into the guest — the union, never the newer alone. Twin of `mergeUiDirtyScopeV1`.
    pub fn merged_with(self, other: Self) -> Self {
        match (self, other) {
            (Self::Full, _) | (_, Self::Full) => Self::Full,
            (Self::None, scope) | (scope, Self::None) => scope,
            (
                Self::Partial { window_bodies, panel_bodies, utilities, tools, engagements, measures, labels },
                Self::Partial {
                    window_bodies: other_window_bodies,
                    panel_bodies: other_panel_bodies,
                    utilities: other_utilities,
                    tools: other_tools,
                    engagements: other_engagements,
                    measures: other_measures,
                    labels: other_labels,
                },
            ) => Self::Partial {
                window_bodies: union_body_keys(window_bodies, other_window_bodies),
                panel_bodies: union_body_keys(panel_bodies, other_panel_bodies),
                utilities: utilities || other_utilities,
                tools: tools || other_tools,
                engagements: engagements || other_engagements,
                measures: measures || other_measures,
                labels: labels || other_labels,
            },
        }
    }
}

/// 🤝️ First-seen order, no duplicates — the `[...new Set([...first, ...second])]` the TypeScript
/// twin builds, spelled without a hash set so the two orders cannot drift.
fn union_body_keys(first: Vec<String>, second: Vec<String>) -> Vec<String> {
    let mut union = first;
    for key in second {
        if !union.contains(&key) {
            union.push(key);
        }
    }
    union
}

#[cfg(test)]
#[path = "🧪️tests/🐢️ui-dirty-scope/🦀️.rs"]
mod ui_dirty_scope_tests;

//#region 🔖️HistoryWire
/// 🚦️ `Severity` crosses the serde half of the history wire as its bare camelCase name, like its value form.
mod history_severity_serde {
    use super::Severity;
    use serde::{Deserialize, Deserializer, Serializer};

    fn name(severity: Severity) -> &'static str {
        match severity {
            Severity::Info => "info",
            Severity::Warning => "warning",
            Severity::Error => "error",
            Severity::Fatal => "fatal",
        }
    }

    fn parse<E: serde::de::Error>(text: &str) -> Result<Severity, E> {
        match text {
            "info" => Ok(Severity::Info),
            "warning" => Ok(Severity::Warning),
            "error" => Ok(Severity::Error),
            "fatal" => Ok(Severity::Fatal),
            other => Err(E::custom(format!("unknown severity {other:?}"))),
        }
    }

    pub fn serialize<S: Serializer>(severity: &Severity, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(name(*severity))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Severity, D::Error> {
        parse(&String::deserialize(deserializer)?)
    }

    /// 🫥️ The optional twin: absent or `null` is `None`.
    pub mod option {
        use super::Severity;
        use serde::{Deserialize, Deserializer, Serializer};

        pub fn serialize<S: Serializer>(severity: &Option<Severity>, serializer: S) -> Result<S::Ok, S::Error> {
            match severity {
                Some(severity) => serializer.serialize_some(super::name(*severity)),
                None => serializer.serialize_none(),
            }
        }

        pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Option<Severity>, D::Error> {
            Option::<String>::deserialize(deserializer)?.map(|text| super::parse(&text)).transpose()
        }
    }
}

/// 🛠️ The committed tool transaction a history row's edit carries (`MutationMeta.transaction`): its id and the
/// `<appId>#<toolId>` tool that authored it.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryTransaction {
    pub id: String,
    pub tool: String,
}

/// 📨️ One outcome message of a history mutation row, the wire mirror of `MutationMessage`: `code` is one of the frozen
/// `mutation.*` codes a host localizes, `message` is English prose for logs.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryMutationMessage {
    #[serde(with = "history_severity_serde")]
    pub level: Severity,
    pub code: String,
    pub message: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub target: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub op_index: Option<u32>,
}

/// ✏️ One applied mutation of a history row: its replica-independent id, applied position and index inside its edit,
/// its localized kind label, its outcome (the time-travel replay's while a session holds a report, else the durable
/// one) and its editing state. `editable` = the op has an input schema and emits no foreign steps; `pending` = it is
/// downstream of the mutation being edited and not applied in the preview; `edited` = the session holds a draft for it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryMutationEntry {
    pub mutation_id: String,
    pub position: u32,
    pub op_index: u32,
    pub label: dsl::LocalizedLabel,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "history_severity_serde::option")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub worst: Option<Severity>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub messages: Vec<HistoryMutationMessage>,
    #[serde(default)]
    #[value(default)]
    pub superseded: bool,
    #[serde(default)]
    #[value(default)]
    pub withdrawn: bool,
    #[serde(default)]
    #[value(default)]
    pub editable: bool,
    #[serde(default)]
    #[value(default)]
    pub pending: bool,
    #[serde(default)]
    #[value(default)]
    pub edited: bool,
}

/// 🚦️ The stage of a live history-edit session (an inactive session is an absent `HistoryPatch.timeTravel`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum HistoryTimeTravelStage {
    #[default]
    Editing,
    Replaying,
    Reviewing,
    Choosing,
    Finalizing,
}

/// 🧭️ What a reviewing session shows: nothing accepted (the committed head), drafts awaiting a replay (after a cancelled or
/// faulted one), a report that blocks finalizing, or a report ready to finalize.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum HistoryTimeTravelReview {
    NoChanges,
    NeedsReplay,
    Blocked,
    Ready,
}

/// ⏪️ The live history-edit session of one instance, as every host renders its band: identity and generation (every
/// `historyEdit*` verb may echo `generation`; a stale one is `timeTravel.stale`), stage, the edited mutation and its
/// label, replay progress, the report's worst severity, whether that report blocks finalizing, the last fault code, how
/// many drafts are accepted, what a review shows and whether a replay can be run again.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryTimeTravel {
    pub session_id: String,
    pub generation: u32,
    pub stage: HistoryTimeTravelStage,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub target_label: Option<dsl::LocalizedLabel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub done: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none", with = "history_severity_serde::option")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub worst: Option<Severity>,
    #[serde(default)]
    #[value(default)]
    pub blocking: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fault: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub accepted_count: u32,
    /// 🧭️ While reviewing: what the review shows (absent in every other stage).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub review: Option<HistoryTimeTravelReview>,
    /// 🔁️ Whether `historyEditRerun` would start a replay now (a cancelled or faulted replay left drafts to replay).
    #[serde(default)]
    #[value(default)]
    pub rerunnable: bool,
}

/// 🧾️ One host-projectable row in the session command timeline, one per committed tool transaction (else per edit;
/// rows without an edit are session commands such as undo or a noted shell command). Hosts key rows by `edit_id`,
/// falling back to `seq` for a row without an edit. The payload is deliberately presentation-neutral: the host owns
/// windowing and retains entries beyond any visible range.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryEntry {
    pub seq: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub edit_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transaction: Option<HistoryTransaction>,
    /// ✏️ The `Supersede` history transition this row is — a history edit, or the undo or redo of one.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub transition_id: Option<String>,
    /// 🖋️ The actor who authored this row's edit or history transition.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    pub action_id: String,
    /// 🏷️ Every shell locale's text for this row, resolved by the renderer against the
    /// active locale — never a pre-resolved string, so switching the shell locale re-renders the
    /// whole ledger instead of leaving already-logged rows in the locale they were dispatched in.
    pub label: dsl::LocalizedLabel,
    pub kind: String,
    pub timestamp: String,
    /// 📜️ The newest forward operations of this row's edit, newest last — a bounded preview; `op_count` says how many the
    /// edit holds.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub op_lines: Vec<String>,
    /// 🔢️ Forward operations of this row's edit; more than `op_lines` holds means the preview omits the older ones.
    #[serde(default)]
    #[value(default)]
    pub op_count: u64,
    #[serde(default)]
    #[value(default)]
    pub applied: bool,
    #[serde(default)]
    #[value(default)]
    pub revertible: bool,
    #[serde(default = "history_entry_count")]
    #[value(default = "history_entry_count")]
    pub count: u32,
    /// 🚦️ The worst severity over `mutations`.
    #[serde(default, skip_serializing_if = "Option::is_none", with = "history_severity_serde::option")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub worst: Option<Severity>,
    /// ✏️ The edit's applied mutations in op order, empty for a row without an applied edit.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub mutations: Vec<HistoryMutationEntry>,
}

// 🚫️async: E4 fn-pointer slot
fn history_entry_count() -> u32 {
    1
}

impl HistoryEntry {
    /// 🔑️ The key a host folds this row under: `edit:<editId>`, else `transition:<transitionId>`, else `seq:<seq>`.
    /// TypeScript twin `historyEntryKey`.
    pub fn key(&self) -> String {
        match (&self.edit_id, &self.transition_id) {
            (Some(edit_id), _) => format!("edit:{edit_id}"),
            (None, Some(transition_id)) => format!("transition:{transition_id}"),
            (None, None) => format!("seq:{}", self.seq),
        }
    }
}

/// 🧾️ Ordered history delta returned in the same response as an accepted interaction.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryPatch {
    /// Monotonic command-log cursor after applying this patch.
    pub cursor: u64,
    /// Upserts, ordered newest-first to match the logical history projection.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub upserts: Vec<HistoryEntry>,
    #[serde(default)]
    #[value(default)]
    pub can_undo: bool,
    #[serde(default)]
    #[value(default)]
    pub can_redo: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub active_alternative_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub current_checkpoint_id: Option<String>,
    #[serde(default)]
    #[value(default)]
    pub command_filter: String,
    /// ⏪️ The live history-edit session; absent while none is open. Every patch carries the current status in full.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub time_travel: Option<HistoryTimeTravel>,
}

#[cfg(test)]
#[path = "🧪️tests/🧪️history-patch/🦀️.rs"]
mod history_patch_tests;
//#endregion 🔖️HistoryWire

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
    pub view_state: super::ViewModel,
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
    pub view_state: super::ViewModel,
}
//#endregion 🔖️Invocation

//#region 🔖️Presence
pub use semio_framework_os_kernel::{decode_presence_history_edit, decode_presence_peer, encode_presence_history_edit, encode_presence_peer, PresenceHistoryEdit, PresenceHistoryEditStage, PresencePeer, PresenceToolRun, PresenceToolRunState, PresenceUi, PresenceViewKind, PresenceWindowView};
//#endregion 🔖️Presence

//#region 🔖️Window
#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct PhysicalSize {
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct Appearance {
    pub mode: String,
}

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
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

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
pub struct WindowInput {
    pub window: WindowHandle,
    pub params: DslValue,
    pub document_snapshot: DslValue,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(rename_all = "camelCase")]
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
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub enum RequestOutcome {
    Ok(Vec<u8>),
    Err(Vec<u8>),
}

//#region 🔖️PagedCommandIngress
/// 🧵️ Definitions relocated to `semio-framework-os-kernel`'s `os_spr::channel` module (ticket
/// 26/08/23/END-TO-END-TESTING-REFACTOR): this file cannot depend on
/// `semio-framework-os-kernel` without a cycle (`semio_framework_os_kernel::{decode_presence_peer,
/// PresencePeer, ...}` above already depends on it), yet the ONE real functional consumer of this
/// paged-command-ingress machinery — `📡️spr/🧵️channel/🦀️.rs` — lives entirely inside
/// that crate and could not reach these types at all. Re-exported here, unchanged, so every
/// existing `semio_framework::kernel::X` / `manifest::kernel::X` call site (and this file's own
/// `#[cfg(test)] mod extension_activation_tests` below, via its `use super::*`) keeps resolving —
/// same pattern this file's own `PresencePeer` re-export above already uses.
pub use semio_framework_os_kernel::channel::{
    CommandBatch, CommandBatchDriver, CommandBatchProgress, CommandDriverRegistry, CommandEnvelope, CommandEnvelopeSet, CommandIngressStatus, CommandPageCursor, CommandPageSet, FixedCommandPage, PagedCommand, PagedCommandReader,
    RejectedCommandBuild, RejectedCommandBuildRegistry, COMMAND_BATCH_MAXIMUM_ITEMS, COMMAND_MAXIMUM_BYTES, COMMAND_MAXIMUM_PAGES, COMMAND_PAGE_MAXIMUM_BYTES,
};
//#endregion 🔖️PagedCommandIngress

/// 📨️ Everything the host delivers into a guest's `reactor::poll` — the full inbound contract
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase")]
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
///
/// 📏️ Inline extent of ONE exact patch owner — the block every arena slot that parks a publication
/// reserves by value, and the number [`UI_TURN_PATCHES_MAXIMUM`] is read off.
pub const UI_TURN_PATCH_OWNER_BYTES: usize = 4_096;

/// 📏️ How many surfaces one turn result may carry. This is the fixed CAPACITY of [`UiTurnPatches`],
/// not the admission rule — admission is the per-turn BYTE budget (`Budget::max_patch_bytes`, bounded
/// by [`UI_TURN_PATCH_BUDGET_BYTES`]), so a turn carries every patch that is ready and fits.
///
/// 🐛️ Until 2026-09-15 this was `1`, and the wire contract "one patch per crossing" made N published
/// surfaces cost N + 1 host round trips by construction — the floor
/// `📓️reactor-reconcile-spin-2026-09-14.md` §7 named and could not move. It is DERIVED, never chosen:
/// the page is an inline exact owner parked by value in every transport slot and every handback, so
/// the guest's declared contiguous-request ceiling is what one such block may cost, at
/// [`UI_TURN_PATCH_OWNER_BYTES`] per publication.
pub const UI_TURN_PATCHES_MAXIMUM: usize = semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES / UI_TURN_PATCH_OWNER_BYTES;

/// 📏️ Ceiling every lane's `Budget::max_patch_bytes` declaration must stay under — a turn result is
/// an ASSEMBLED answer crossing the boundary, and a single assembled answer may claim a quarter of
/// what the declared budget admits (`🧮️memory/🦀️.rs`'s `GUEST_HOST_ANSWER_CEILING_BYTES`). The floor
/// is the contiguous ceiling itself: a lane that declared less than one contiguous page could not
/// carry one document-scaled publication at all.
pub const UI_TURN_PATCH_BUDGET_BYTES: usize = semio_framework_trace::GUEST_HOST_ANSWER_CEILING_BYTES / 4;

/// 📏️ Fixed wire envelope of one patch, and of one of its operations — the same two numbers the
/// native host's own `patch_wire_bytes` charges (`🔌️plugin/🖥️host/📥️ui-patch/🦀️.rs`), so the guest
/// that admits a batch and the host that refuses one price the envelope identically.
pub const UI_TURN_PATCH_WIRE_ENVELOPE_BYTES: usize = 32;
pub const UI_TURN_PATCH_OP_WIRE_ENVELOPE_BYTES: usize = 16;

/// 📐️ What ONE patch costs the turn it joins: its wire envelope, its surface name, one operation
/// envelope per operation, and the physical backing its operation storage actually holds. The last
/// term is what makes this an UPPER bound of the host's wire measure — an operation's pack payload is
/// never larger than the inline `UiPatchOp` that produced it — so a batch the guest admits can never
/// be a batch the host refuses.
pub fn ui_patch_turn_bytes(patch: &UiPatch) -> usize {
    UI_TURN_PATCH_WIRE_ENVELOPE_BYTES
        .saturating_add(patch.surface.as_ref().len())
        .saturating_add(patch.ops.len().saturating_mul(UI_TURN_PATCH_OP_WIRE_ENVELOPE_BYTES))
        .saturating_add(patch.ops.allocated_bytes())
}

pub const UI_TURN_PATCH_RETIRE_SLOTS: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct UiTurnPatchRetireKey {
    slot: usize,
    epoch: u64,
}

#[derive(Debug, Default)]
struct UiTurnPatchRetireSlot {
    epoch: u64,
    reserved: bool,
    contents: Option<UiTurnPatchContents>,
}

#[derive(Debug, Default)]
struct UiTurnPatchContents {
    pending: semio_framework_ui_contract::UiPendingPatch,
}

impl UiTurnPatchContents {
    fn terminal_is_empty(&self) -> bool {
        self.pending.terminal_is_empty()
    }

    fn close_step(&mut self, items: usize, bytes: usize) -> Result<semio_framework_ui_contract::UiValueRetirementStep, &'static str> {
        use semio_framework_ui_contract::UiValueRetirementStep;
        if items == 0 || bytes == 0 {
            return Ok(UiValueRetirementStep::default());
        }
        if !self.pending.terminal_is_empty() {
            return self.pending.close_step(1, bytes);
        }
        Ok(UiValueRetirementStep { complete: true, ..Default::default() })
    }
}

struct UiTurnPatchHandback {
    ready: std::sync::atomic::AtomicBool,
    owner: std::cell::UnsafeCell<std::mem::MaybeUninit<(UiTurnPatchRetireKey, UiTurnPatchContents)>>,
}

/// 🔒️ A reserved key has one non-cloneable producer; its slot cannot be reused before the sole arena consumer retires the returned owner.
unsafe impl Sync for UiTurnPatchHandback {}

impl UiTurnPatchHandback {
    const fn new() -> Self {
        Self { ready: std::sync::atomic::AtomicBool::new(false), owner: std::cell::UnsafeCell::new(std::mem::MaybeUninit::uninit()) }
    }

    fn publish(&self, key: UiTurnPatchRetireKey, contents: UiTurnPatchContents) {
        unsafe {
            (*self.owner.get()).write((key, contents));
        }
        self.ready.store(true, std::sync::atomic::Ordering::Release);
    }

    fn take(&self) -> Option<(UiTurnPatchRetireKey, UiTurnPatchContents)> {
        if !self.ready.load(std::sync::atomic::Ordering::Acquire) {
            return None;
        }
        let owner = unsafe { (*self.owner.get()).assume_init_read() };
        self.ready.store(false, std::sync::atomic::Ordering::Release);
        Some(owner)
    }
}

const _: () = assert!(size_of::<(UiTurnPatchRetireKey, UiTurnPatchContents)>() <= UI_TURN_PATCH_OWNER_BYTES);
static UI_TURN_PATCH_HANDBACKS: [UiTurnPatchHandback; UI_TURN_PATCH_RETIRE_SLOTS] = [const { UiTurnPatchHandback::new() }; UI_TURN_PATCH_RETIRE_SLOTS];
static UI_TURN_PATCH_RETIRE_ARENA: std::sync::Mutex<UiTurnPatchRetireArena> =
    std::sync::Mutex::new(UiTurnPatchRetireArena { slots: [const { UiTurnPatchRetireSlot { epoch: 0, reserved: false, contents: None } }; UI_TURN_PATCH_RETIRE_SLOTS], next_epoch: 1, epoch_exhausted: false, close_cursor: 0 });

#[derive(Debug)]
struct UiTurnPatchRetireArena {
    slots: [UiTurnPatchRetireSlot; UI_TURN_PATCH_RETIRE_SLOTS],
    next_epoch: u64,
    epoch_exhausted: bool,
    close_cursor: usize,
}

impl Default for UiTurnPatchRetireArena {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| UiTurnPatchRetireSlot::default()), next_epoch: 1, epoch_exhausted: false, close_cursor: 0 }
    }
}

impl UiTurnPatchRetireArena {
    fn reserve(&mut self) -> Option<UiTurnPatchRetireKey> {
        if self.epoch_exhausted {
            return None;
        }
        let slot = self.slots.iter().position(|slot| !slot.reserved)?;
        let epoch = self.next_epoch;
        let target = &mut self.slots[slot];
        target.epoch = epoch;
        target.reserved = true;
        match epoch.checked_add(1) {
            Some(next) => self.next_epoch = next,
            None => self.epoch_exhausted = true,
        }
        Some(UiTurnPatchRetireKey { slot, epoch })
    }

    fn release_empty(&mut self, key: UiTurnPatchRetireKey) -> bool {
        let Some(slot) = self.slots.get_mut(key.slot) else { return false };
        if !slot.reserved || slot.epoch != key.epoch || slot.contents.is_some() {
            return false;
        }
        slot.reserved = false;
        true
    }

    #[expect(clippy::result_large_err, reason = "A refused retirement handback returns the exact patch contents without allocation.")]
    fn handback(&mut self, key: UiTurnPatchRetireKey, contents: UiTurnPatchContents) -> Result<(), UiTurnPatchContents> {
        let Some(slot) = self.slots.get_mut(key.slot).filter(|slot| slot.reserved && slot.epoch == key.epoch && slot.contents.is_none()) else {
            return Err(contents);
        };
        slot.contents = Some(contents);
        Ok(())
    }

    fn close_one(&mut self, items: usize, bytes: usize) -> bool {
        for offset in 0..UI_TURN_PATCH_RETIRE_SLOTS {
            let Some(index) = self.close_cursor.checked_add(offset).map(|index| index % UI_TURN_PATCH_RETIRE_SLOTS) else { return false };
            let slot = &mut self.slots[index];
            if !slot.reserved || slot.contents.is_none() {
                continue;
            }
            let Some(next) = index.checked_add(1) else { return false };
            self.close_cursor = next % UI_TURN_PATCH_RETIRE_SLOTS;
            let contents = slot.contents.as_mut().expect("selected returned patch owner");
            if contents.close_step(items, bytes).expect("exact returned patch retirement").complete && contents.terminal_is_empty() {
                slot.contents = None;
                slot.reserved = false;
            }
            return true;
        }
        false
    }
}

#[cfg(test)]
fn with_ui_turn_patch_retire_arena<T>(f: impl FnOnce(&mut UiTurnPatchRetireArena) -> T) -> T {
    let mut arena = UI_TURN_PATCH_RETIRE_ARENA.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&mut arena)
}

pub fn close_ui_turn_patch_owner_one() -> bool {
    close_ui_turn_patch_owner_with_grant(1, 4096)
}

/// ♻️ One retirement unit of a returned turn patch, against the caller's grant.
///
/// The grant is what keeps a document-scaled publication off the round-trip ladder: the patch a
/// 180-object world-3d document swap returns here needs 1 091 retirement units, and the reactor
/// spends one turn — one host round trip — per unit it cannot finish (ticket 26/09/02 W-S2, measured
/// 2026-09-10: 24.3 s and 8 799 worker messages for one example switch).
pub fn close_ui_turn_patch_owner_with_grant(items: usize, bytes: usize) -> bool {
    if items == 0 || bytes == 0 {
        return false;
    }
    let mut arena = match UI_TURN_PATCH_RETIRE_ARENA.try_lock() {
        Ok(arena) => arena,
        Err(std::sync::TryLockError::WouldBlock) => return false,
        Err(std::sync::TryLockError::Poisoned(_)) => return false,
    };
    for offset in 0..UI_TURN_PATCH_RETIRE_SLOTS {
        let index = (arena.close_cursor + offset) % UI_TURN_PATCH_RETIRE_SLOTS;
        if let Some((key, contents)) = UI_TURN_PATCH_HANDBACKS[index].take() {
            if let Err(contents) = arena.handback(key, contents) {
                UI_TURN_PATCH_HANDBACKS[index].publish(key, contents);
                return false;
            }
            arena.close_cursor = (index + 1) % UI_TURN_PATCH_RETIRE_SLOTS;
            return true;
        }
    }
    arena.close_one(items, bytes)
}

pub const UI_TURN_PATCH_TRANSPORT_SLOTS: usize = 64;
const UI_TURN_PATCH_TRANSPORT_TOKEN_BYTES: usize = 32;
const UI_TURN_PATCH_TRANSPORT_MAGIC: [u8; 8] = *b"semui005";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct UiTurnPatchTransportKey {
    slot: usize,
    epoch: u64,
    session: u64,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum UiTurnPatchTransportState {
    #[default]
    Vacant,
    Building,
    Published,
    CheckedOut,
    Closing,
}

#[derive(Debug, Default)]
struct UiTurnPatchTransportSlot {
    epoch: u64,
    session: u64,
    state: UiTurnPatchTransportState,
    owner: Option<UiTurnPatches>,
    external: bool,
}

#[derive(Debug)]
struct UiTurnPatchTransportArena {
    slots: [UiTurnPatchTransportSlot; UI_TURN_PATCH_TRANSPORT_SLOTS],
    close_cursor: usize,
}

impl Default for UiTurnPatchTransportArena {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| UiTurnPatchTransportSlot::default()), close_cursor: 0 }
    }
}

impl UiTurnPatchTransportArena {
    #[expect(clippy::result_large_err, reason = "Admission refusal preserves the exact patch owner without allocating.")]
    fn reserve(&mut self, session: u64, owner: UiTurnPatches) -> Result<UiTurnPatchTransportKey, UiTurnPatches> {
        let Some(slot) = self.slots.iter().position(|slot| slot.state == UiTurnPatchTransportState::Vacant) else { return Err(owner) };
        let Some(epoch) = self.slots[slot].epoch.checked_add(1) else { return Err(owner) };
        self.slots[slot] = UiTurnPatchTransportSlot { epoch, session, state: UiTurnPatchTransportState::Building, owner: Some(owner), external: true };
        Ok(UiTurnPatchTransportKey { slot, epoch, session })
    }

    fn slot_mut(&mut self, key: UiTurnPatchTransportKey) -> Option<&mut UiTurnPatchTransportSlot> {
        let slot = self.slots.get_mut(key.slot)?;
        (slot.epoch == key.epoch && slot.session == key.session && slot.state != UiTurnPatchTransportState::Vacant).then_some(slot)
    }

    fn close_one(&mut self, items: usize, bytes: usize) -> Result<UiTurnPatchTransportProgress, &'static str> {
        for offset in 0..UI_TURN_PATCH_TRANSPORT_SLOTS {
            let index = (self.close_cursor + offset) % UI_TURN_PATCH_TRANSPORT_SLOTS;
            if self.slots[index].state != UiTurnPatchTransportState::Closing {
                continue;
            }
            self.close_cursor = (index + 1) % UI_TURN_PATCH_TRANSPORT_SLOTS;
            if self.slots[index].external {
                return Ok(UiTurnPatchTransportProgress::Blocked);
            }
            let owner = self.slots[index].owner.as_mut().ok_or("closing turn patch transport lost its exact owner")?;
            let step = owner.close_step_with_grant(items.max(1), bytes.max(1))?;
            if step.complete {
                let epoch = self.slots[index].epoch;
                self.slots[index] = UiTurnPatchTransportSlot { epoch, ..UiTurnPatchTransportSlot::default() };
                return Ok(UiTurnPatchTransportProgress::Pending { released_items: step.released_items.max(1), released_bytes: step.released_bytes });
            }
            return Ok(if step.progressed { UiTurnPatchTransportProgress::Pending { released_items: step.released_items, released_bytes: step.released_bytes } } else { UiTurnPatchTransportProgress::Blocked });
        }
        Ok(UiTurnPatchTransportProgress::Idle)
    }

    #[cfg(test)]
    fn request_session_close(&mut self, session: u64) -> bool {
        let Some(slot) = self.slots.iter_mut().find(|slot| slot.session == session && slot.state != UiTurnPatchTransportState::Vacant) else { return false };
        if slot.state != UiTurnPatchTransportState::CheckedOut {
            slot.state = UiTurnPatchTransportState::Closing;
        }
        true
    }
}

static UI_TURN_PATCH_TRANSPORT_ARENA: std::sync::Mutex<UiTurnPatchTransportArena> =
    std::sync::Mutex::new(UiTurnPatchTransportArena { slots: [const { UiTurnPatchTransportSlot { epoch: 0, session: 0, state: UiTurnPatchTransportState::Vacant, owner: None, external: false } }; UI_TURN_PATCH_TRANSPORT_SLOTS], close_cursor: 0 });

struct UiTurnPatchTransportHandback {
    ready: std::sync::atomic::AtomicBool,
    value: std::cell::UnsafeCell<std::mem::MaybeUninit<(UiTurnPatchTransportKey, Option<UiTurnPatches>)>>,
}

/// 🔒️ A reserved slot has exactly one producer or checked-out lease; its epoch cannot be reused before the sole arena consumer takes this handback.
unsafe impl Sync for UiTurnPatchTransportHandback {}

impl UiTurnPatchTransportHandback {
    const fn new() -> Self {
        Self { ready: std::sync::atomic::AtomicBool::new(false), value: std::cell::UnsafeCell::new(std::mem::MaybeUninit::uninit()) }
    }
    fn publish(&self, key: UiTurnPatchTransportKey, owner: Option<UiTurnPatches>) {
        unsafe {
            (*self.value.get()).write((key, owner));
        }
        self.ready.store(true, std::sync::atomic::Ordering::Release);
    }
    fn take(&self) -> Option<(UiTurnPatchTransportKey, Option<UiTurnPatches>)> {
        if !self.ready.load(std::sync::atomic::Ordering::Acquire) {
            return None;
        }
        let value = unsafe { (*self.value.get()).assume_init_read() };
        self.ready.store(false, std::sync::atomic::Ordering::Release);
        Some(value)
    }
}

const _: () = assert!(size_of::<(UiTurnPatchTransportKey, Option<UiTurnPatches>)>() <= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES);
static UI_TURN_PATCH_TRANSPORT_HANDBACKS: [UiTurnPatchTransportHandback; UI_TURN_PATCH_TRANSPORT_SLOTS] = [const { UiTurnPatchTransportHandback::new() }; UI_TURN_PATCH_TRANSPORT_SLOTS];

#[cfg(test)]
fn with_ui_turn_patch_transport_arena<T>(f: impl FnOnce(&mut UiTurnPatchTransportArena) -> T) -> T {
    let mut arena = UI_TURN_PATCH_TRANSPORT_ARENA.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
    f(&mut arena)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiTurnPatchTransportProgress {
    Idle,
    Blocked,
    Pending { released_items: usize, released_bytes: usize },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiTurnPatchTransportStep {
    MoreWork,
    Blocked,
    Fault(&'static str),
    Ready,
    Cancelled,
    Stale,
}

pub struct UiTurnPatchTransportProducer {
    key: UiTurnPatchTransportKey,
    patch: usize,
    operation: usize,
    ready: bool,
    transferred: bool,
    closing: bool,
}

impl UiTurnPatchTransportProducer {
    #[expect(clippy::result_large_err, reason = "A contended or full transport arena returns the exact patch owner for retry.")]
    pub fn try_new(session: u64, owner: UiTurnPatches) -> Result<Self, UiTurnPatches> {
        let Ok(mut arena) = UI_TURN_PATCH_TRANSPORT_ARENA.try_lock() else {
            return Err(owner);
        };
        let key = arena.reserve(session, owner)?;
        Ok(Self { key, patch: 0, operation: 0, ready: false, transferred: false, closing: false })
    }

    pub fn drive_one(&mut self, session: u64, cancelled: bool, deadline_expired: bool) -> UiTurnPatchTransportStep {
        if session != self.key.session || self.transferred {
            return UiTurnPatchTransportStep::Stale;
        }
        if cancelled || self.closing {
            if !self.closing {
                UI_TURN_PATCH_TRANSPORT_HANDBACKS[self.key.slot].publish(self.key, None);
                self.closing = true;
            }
            return UiTurnPatchTransportStep::Cancelled;
        }
        if deadline_expired {
            return UiTurnPatchTransportStep::MoreWork;
        }
        let mut arena = match UI_TURN_PATCH_TRANSPORT_ARENA.try_lock() {
            Ok(arena) => arena,
            Err(std::sync::TryLockError::WouldBlock) => return UiTurnPatchTransportStep::Blocked,
            Err(std::sync::TryLockError::Poisoned(_)) => return UiTurnPatchTransportStep::Fault("turn patch transport arena is poisoned"),
        };
        if arena.slot_mut(self.key).is_some_and(|slot| slot.state == UiTurnPatchTransportState::Closing) {
            UI_TURN_PATCH_TRANSPORT_HANDBACKS[self.key.slot].publish(self.key, None);
            self.closing = true;
            return UiTurnPatchTransportStep::Cancelled;
        }
        let step = (|| {
            let slot = arena.slot_mut(self.key)?;
            if slot.state != UiTurnPatchTransportState::Building {
                return None;
            }
            let owner = slot.owner.as_ref()?;
            let Some(patch) = owner.iter().nth(self.patch) else {
                slot.state = UiTurnPatchTransportState::Published;
                return Some(UiTurnPatchTransportStep::Ready);
            };
            if self.operation < patch.ops.len() {
                self.operation = self.operation.checked_add(1)?;
                return Some(UiTurnPatchTransportStep::MoreWork);
            }
            self.patch = self.patch.checked_add(1)?;
            self.operation = 0;
            Some(UiTurnPatchTransportStep::MoreWork)
        })();
        let Some(step) = step else { return UiTurnPatchTransportStep::Stale };
        self.ready |= step == UiTurnPatchTransportStep::Ready;
        step
    }

    pub fn take_ready(&mut self) -> Result<Option<[u8; UI_TURN_PATCH_TRANSPORT_TOKEN_BYTES]>, &'static str> {
        if !self.ready || self.transferred || self.closing {
            return Ok(None);
        }
        let mut arena = match UI_TURN_PATCH_TRANSPORT_ARENA.try_lock() {
            Ok(arena) => arena,
            Err(std::sync::TryLockError::WouldBlock) => return Ok(None),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err("turn patch transport arena is poisoned"),
        };
        let slot = arena.slot_mut(self.key).filter(|slot| slot.state == UiTurnPatchTransportState::Published && slot.owner.is_some() && slot.external).ok_or("exact turn patch publication is no longer available")?;
        let mut token = [0u8; UI_TURN_PATCH_TRANSPORT_TOKEN_BYTES];
        token[..8].copy_from_slice(&UI_TURN_PATCH_TRANSPORT_MAGIC);
        token[8..16].copy_from_slice(&(self.key.slot as u64).to_le_bytes());
        token[16..24].copy_from_slice(&self.key.epoch.to_le_bytes());
        token[24..32].copy_from_slice(&self.key.session.to_le_bytes());
        slot.external = false;
        self.transferred = true;
        Ok(Some(token))
    }
}

impl Drop for UiTurnPatchTransportProducer {
    fn drop(&mut self) {
        if self.transferred || self.closing {
            return;
        }
        UI_TURN_PATCH_TRANSPORT_HANDBACKS[self.key.slot].publish(self.key, None);
        self.closing = true;
    }
}

pub struct UiTurnPatchTransportLease {
    key: UiTurnPatchTransportKey,
    owner: Option<UiTurnPatches>,
}

impl UiTurnPatchTransportLease {
    pub fn try_from_token(token: &[u8], expected_session: u64) -> Result<Self, &'static str> {
        if token.len() != UI_TURN_PATCH_TRANSPORT_TOKEN_BYTES || token[..8] != UI_TURN_PATCH_TRANSPORT_MAGIC {
            return Err("invalid turn patch transport token");
        }
        let slot = usize::try_from(u64::from_le_bytes(token[8..16].try_into().map_err(|_| "invalid turn patch slot")?)).map_err(|_| "invalid turn patch slot")?;
        let epoch = u64::from_le_bytes(token[16..24].try_into().map_err(|_| "invalid turn patch epoch")?);
        let session = u64::from_le_bytes(token[24..32].try_into().map_err(|_| "invalid turn patch session")?);
        if session != expected_session {
            return Err("stale turn patch session");
        }
        let key = UiTurnPatchTransportKey { slot, epoch, session };
        let mut arena = match UI_TURN_PATCH_TRANSPORT_ARENA.try_lock() {
            Ok(arena) => arena,
            Err(std::sync::TryLockError::WouldBlock) => return Err("turn patch transport arena is busy"),
            Err(std::sync::TryLockError::Poisoned(_)) => return Err("turn patch transport arena is poisoned"),
        };
        let owner = (|| {
            let slot = arena.slot_mut(key)?;
            if slot.state != UiTurnPatchTransportState::Published || slot.external || slot.owner.is_none() {
                return None;
            }
            slot.state = UiTurnPatchTransportState::CheckedOut;
            slot.external = true;
            slot.owner.take()
        })()
        .ok_or("stale or duplicate turn patch token")?;
        Ok(Self { key, owner: Some(owner) })
    }

    #[expect(clippy::result_large_err, reason = "A refused transfer preserves the exact lease and its retirement authority for retry.")]
    pub fn take_owner(mut self) -> Result<UiTurnPatches, Self> {
        let Ok(mut arena) = UI_TURN_PATCH_TRANSPORT_ARENA.try_lock() else {
            return Err(self);
        };
        let Some(slot) = arena.slot_mut(self.key).filter(|slot| slot.state == UiTurnPatchTransportState::CheckedOut && slot.external && slot.owner.is_none()) else {
            return Err(self);
        };
        let Some(owner) = self.owner.take() else { return Err(self) };
        let epoch = slot.epoch;
        *slot = UiTurnPatchTransportSlot { epoch, ..UiTurnPatchTransportSlot::default() };
        Ok(owner)
    }
}

impl Drop for UiTurnPatchTransportLease {
    fn drop(&mut self) {
        let Some(owner) = self.owner.take() else { return };
        UI_TURN_PATCH_TRANSPORT_HANDBACKS[self.key.slot].publish(self.key, Some(owner));
    }
}

pub fn close_ui_turn_patch_transport_one() -> Result<UiTurnPatchTransportProgress, &'static str> {
    close_ui_turn_patch_transport_with_grant(1, 4096)
}

/// ♻️ One retirement unit of a closing turn-patch transport, against the caller's PAGE grant.
///
/// The transport owns the patch the turn hands to the host, so a document-scaled publication parks a
/// document-scaled owner here; retiring it one item per unit is one host round trip per item, because
/// the reactor answers `MoreWork` for as long as anything is outstanding (ticket 26/09/02, W-S2 §7
/// left this ladder ungranted, W-B2 grants it).
pub fn close_ui_turn_patch_transport_with_grant(items: usize, bytes: usize) -> Result<UiTurnPatchTransportProgress, &'static str> {
    let mut arena = match UI_TURN_PATCH_TRANSPORT_ARENA.try_lock() {
        Ok(arena) => arena,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(UiTurnPatchTransportProgress::Blocked),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("turn patch transport arena is poisoned"),
    };
    for offset in 0..UI_TURN_PATCH_TRANSPORT_SLOTS {
        let index = (arena.close_cursor + offset) % UI_TURN_PATCH_TRANSPORT_SLOTS;
        let Some((key, owner)) = UI_TURN_PATCH_TRANSPORT_HANDBACKS[index].take() else {
            continue;
        };
        let Some(slot) = arena.slot_mut(key).filter(|slot| {
            slot.external
                && if owner.is_some() {
                    slot.state == UiTurnPatchTransportState::CheckedOut && slot.owner.is_none()
                } else {
                    matches!(slot.state, UiTurnPatchTransportState::Building | UiTurnPatchTransportState::Published | UiTurnPatchTransportState::Closing) && slot.owner.is_some()
                }
        }) else {
            UI_TURN_PATCH_TRANSPORT_HANDBACKS[index].publish(key, owner);
            return Err("exact turn patch handback does not match its reserved slot");
        };
        if owner.is_some() {
            slot.owner = owner;
        }
        slot.external = false;
        slot.state = UiTurnPatchTransportState::Closing;
        arena.close_cursor = (index + 1) % UI_TURN_PATCH_TRANSPORT_SLOTS;
        return Ok(UiTurnPatchTransportProgress::Pending { released_items: 0, released_bytes: 0 });
    }
    arena.close_one(items, bytes)
}

pub fn close_ui_turn_patch_transport_session_one(session: u64) -> Result<UiTurnPatchTransportProgress, &'static str> {
    let mut arena = match UI_TURN_PATCH_TRANSPORT_ARENA.try_lock() {
        Ok(arena) => arena,
        Err(std::sync::TryLockError::WouldBlock) => return Ok(UiTurnPatchTransportProgress::Blocked),
        Err(std::sync::TryLockError::Poisoned(_)) => return Err("turn patch transport arena is poisoned"),
    };
    if let Some(slot) = arena.slots.iter_mut().find(|slot| slot.session == session && matches!(slot.state, UiTurnPatchTransportState::Building | UiTurnPatchTransportState::Published)) {
        slot.state = UiTurnPatchTransportState::Closing;
        return Ok(UiTurnPatchTransportProgress::Pending { released_items: 0, released_bytes: 0 });
    }
    Ok(if arena.slots.iter().any(|slot| slot.session == session && slot.state != UiTurnPatchTransportState::Vacant) { UiTurnPatchTransportProgress::Blocked } else { UiTurnPatchTransportProgress::Idle })
}

/// 🧰️ One publication slot of the turn page: an exact patch owner and the retirement reservation it
/// hands back when it is dropped.
#[derive(Debug, Default)]
struct UiTurnPatchEntry {
    contents: UiTurnPatchContents,
    retirement: Option<UiTurnPatchRetireKey>,
}

/// 🧰️ The fixed exact-owner patch page emitted by one turn — every patch that was ready when the turn
/// assembled its result, in publication order, bounded by [`UI_TURN_PATCHES_MAXIMUM`] slots and by the
/// caller's own byte budget.
///
/// 📐️ `length` is the pushed prefix and `cursor` the next slot a reader takes, so `[cursor, length)`
/// is exactly the patches still in the page and slots below `cursor` hold nothing but the retirement
/// reservation their patch left behind.
#[derive(Debug)]
pub struct UiTurnPatches {
    entries: [UiTurnPatchEntry; UI_TURN_PATCHES_MAXIMUM],
    length: usize,
    cursor: usize,
}

impl Default for UiTurnPatches {
    fn default() -> Self {
        Self { entries: std::array::from_fn(|_| UiTurnPatchEntry::default()), length: 0, cursor: 0 }
    }
}

/// 🧨️ The page is parked BY VALUE in every transport slot and every transport handback, so its own
/// extent is the one block the guest's declared contiguous-request ceiling has to fund.
const _: () = assert!(size_of::<UiTurnPatches>() <= semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES);

pub enum UiTurnPatchTransfer<T> {
    Empty,
    Transferred(T),
    Refused,
}

impl PartialEq for UiTurnPatches {
    fn eq(&self, other: &Self) -> bool {
        self.len() == other.len() && self.iter().zip(other.iter()).all(|(left, right)| left == right)
    }
}

impl UiTurnPatches {
    /// 📥️ Appends one patch to the page's publication order, reserving that slot's own retirement.
    ///
    /// 🐛️ Until 2026-09-15 the page held exactly ONE patch, so the second surface a turn had ready was
    /// refused here and cost a whole host round trip of its own.
    #[expect(clippy::result_large_err, reason = "Refusal returns the exact fixed patch so the caller retains its retirement obligation.")]
    pub fn try_push_ui_patch(&mut self, patch: UiPatch) -> Result<(), UiPatch> {
        if self.length == UI_TURN_PATCHES_MAXIMUM {
            return Err(patch);
        }
        let index = self.length;
        if !self.entries[index].contents.terminal_is_empty() || self.entries[index].contents.pending.source_mut().is_err() {
            return Err(patch);
        }
        if self.entries[index].retirement.is_none() {
            let Ok(mut arena) = UI_TURN_PATCH_RETIRE_ARENA.try_lock() else {
                return Err(patch);
            };
            let Some(retirement) = arena.reserve() else { return Err(patch) };
            self.entries[index].retirement = Some(retirement);
        }
        *self.entries[index].contents.pending.source_mut().expect("preflighted pending patch slot") = Some(patch);
        self.length = index + 1;
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &UiPatch> {
        self.entries[self.cursor..self.length].iter().filter_map(|entry| entry.contents.pending.get())
    }

    pub fn len(&self) -> usize {
        self.length - self.cursor
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// 📏️ What this page costs its turn's byte budget — the sum every admitting caller compares.
    pub fn turn_bytes(&self) -> usize {
        self.iter().fold(0usize, |total, patch| total.saturating_add(ui_patch_turn_bytes(patch)))
    }

    /// 📤️ Hands out the NEXT patch in publication order, so a reader that loops until `Empty` sees
    /// every surface this turn published exactly once and in the order the guest published it.
    pub fn try_transfer_one<T>(&mut self, transfer: impl FnOnce(UiPatch) -> Result<T, UiPatch>) -> UiTurnPatchTransfer<T> {
        if self.cursor == self.length {
            return UiTurnPatchTransfer::Empty;
        }
        let index = self.cursor;
        let Ok(source) = self.entries[index].contents.pending.source_mut() else {
            return UiTurnPatchTransfer::Refused;
        };
        let Some(patch) = source.take() else { return UiTurnPatchTransfer::Empty };
        match transfer(patch) {
            Ok(value) => {
                self.cursor = index + 1;
                UiTurnPatchTransfer::Transferred(value)
            }
            Err(patch) => {
                *self.entries[index].contents.pending.source_mut().expect("preflighted pending patch slot") = Some(patch);
                UiTurnPatchTransfer::Refused
            }
        }
    }

    pub fn close_step(&mut self) -> bool {
        self.close_step_with_grant(1, 4096).expect("exact turn patch retirement").complete
    }

    pub fn close_step_with_grant(&mut self, items: usize, bytes: usize) -> Result<semio_framework_ui_contract::UiValueRetirementStep, &'static str> {
        use semio_framework_ui_contract::UiValueRetirementStep;
        if items == 0 || bytes == 0 {
            return Ok(UiValueRetirementStep::default());
        }
        for index in 0..self.length {
            if !self.entries[index].contents.terminal_is_empty() {
                let mut step = self.entries[index].contents.close_step(items, bytes)?;
                step.complete = false;
                return Ok(step);
            }
            let Some(retirement) = self.entries[index].retirement else { continue };
            let mut arena = match UI_TURN_PATCH_RETIRE_ARENA.try_lock() {
                Ok(arena) => arena,
                Err(std::sync::TryLockError::WouldBlock) => return Ok(UiValueRetirementStep::default()),
                Err(std::sync::TryLockError::Poisoned(_)) => return Err("turn patch retirement arena is poisoned"),
            };
            if !arena.release_empty(retirement) {
                return Err("exact turn patch retirement reservation missing");
            }
            self.entries[index].retirement = None;
            return Ok(UiValueRetirementStep { progressed: true, released_items: 1, ..Default::default() });
        }
        self.length = 0;
        self.cursor = 0;
        Ok(UiValueRetirementStep { complete: true, ..Default::default() })
    }
}

/// 📤️ Drains the page in publication order; whatever the reader leaves behind retires through the
/// page's own [`Drop`], exactly as a partially transferred page does.
pub struct UiTurnPatchesIntoIter {
    owner: UiTurnPatches,
}

impl Iterator for UiTurnPatchesIntoIter {
    type Item = UiPatch;

    fn next(&mut self) -> Option<UiPatch> {
        match self.owner.try_transfer_one(Ok::<UiPatch, UiPatch>) {
            UiTurnPatchTransfer::Transferred(patch) => Some(patch),
            UiTurnPatchTransfer::Empty | UiTurnPatchTransfer::Refused => None,
        }
    }
}

impl IntoIterator for UiTurnPatches {
    type Item = UiPatch;
    type IntoIter = UiTurnPatchesIntoIter;

    fn into_iter(self) -> Self::IntoIter {
        UiTurnPatchesIntoIter { owner: self }
    }
}

impl Drop for UiTurnPatches {
    fn drop(&mut self) {
        for index in 0..self.entries.len() {
            let Some(retirement) = self.entries[index].retirement.take() else { continue };
            let contents = std::mem::take(&mut self.entries[index].contents);
            UI_TURN_PATCH_HANDBACKS[retirement.slot].publish(retirement, contents);
        }
    }
}

impl Serialize for UiTurnPatches {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        use serde::ser::SerializeSeq;
        let mut sequence = serializer.serialize_seq(Some(self.len()))?;
        for patch in self.iter() {
            sequence.serialize_element(patch)?;
        }
        sequence.end()
    }
}

impl<'de> Deserialize<'de> for UiTurnPatches {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        struct UiTurnPatchesVisitor;

        impl<'de> serde::de::Visitor<'de> for UiTurnPatchesVisitor {
            type Value = UiTurnPatches;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("a fixed turn patch page")
            }

            fn visit_seq<A: serde::de::SeqAccess<'de>>(self, mut access: A) -> Result<Self::Value, A::Error> {
                let mut patches = UiTurnPatches::default();
                while let Some(patch) = access.next_element::<UiPatch>()? {
                    if patches.try_push_ui_patch(patch).is_err() {
                        return Err(serde::de::Error::custom("turn patch page capacity exceeded"));
                    }
                }
                Ok(patches)
            }
        }

        deserializer.deserialize_seq(UiTurnPatchesVisitor)
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TurnResult {
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

#[cfg(test)]
#[path = "🧪️tests/🗣️return-content-dialects/🦀️.rs"]
mod return_content_dialect_tests;

#[path = "📤️return/📦️content/🦀️.rs"]
pub mod return_content;

#[path = "📤️return/📦️content/💌️message/🦀️.rs"]
pub mod return_message;

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
pub(crate) mod return_source_entries;

#[cfg(test)]
#[path = "🧪️tests/🔬️ui-turn-patch/🦀️.rs"]
mod ui_turn_patch_tests;
//#endregion 🔖️TurnResult

//#region 🔖️Broker
/// 🔑️ A capability's identity — dotted/colon-scoped strings (`storage.read`, `http:<origin>`,
/// `messaging.plugin:<id>`, `extension.invoke:<id>`, ...) per `📓️design-abi.md` §5's catalogue.
/// A `String` newtype rather than a closed enum: several members carry a caller-chosen parameter
/// (`<origin>`/`<uri>`/`<id>`/`<point>`) the broker matches by prefix, and the catalogue is
/// expected to grow as new capability surfaces land — an exhaustive enum would need a matching
/// wildcard arm anyway.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
pub struct CapabilityId(pub String);

/// 🙏️ A guest's ask for a capability — `📓️design-abi.md` §5. Replaces `CapabilityRequirement`
/// for the plugin/extension actor runtime. The kernel-level `CapabilityRequirement`/`Rights`/
/// `Scope` action-dispatch model (above, `🔖️Capability` region) stays as-is: it has live
/// consumers outside this packet's owned paths (`🔌️plugin/🏗️builder`, `🔌️plugin/🖥️host`,
/// `🔌️plugin/🦀️.rs`) — see this packet's report for the full consumer list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct CapabilityRequest {
    pub id: CapabilityId,
    pub scope: String,
    pub reason: String,
    #[serde(default)]
    #[value(default)]
    pub optional: bool,
}

/// 🎟️ A broker-issued grant answering a `CapabilityRequest` — `📓️design-abi.md` §5.
/// Named `BrokerCapabilityGrant`, not the design prose's bare `CapabilityGrant`: this file
/// already has a `CapabilityGrant` (above, `🔖️Capability` region) for the unrelated kernel-level
/// action/window capability model (`ActionContext.granted_capabilities`), with live consumers
/// outside this packet's owned paths (`📦️packages/🦀️rust/🦀️.rs`'s re-export list) — see the
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
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct QuotaSchema {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub memory_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub fuel_per_turn: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub turn_deadline_ms: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub tables: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub mailbox_len: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub message_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub outstanding_requests: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub timers: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub storage_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub network_bytes_per_min: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub ui_nodes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub patch_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub patch_hz: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub blob_resident_bytes: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub gpu_ms_per_frame: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub background_ms_per_min: Option<u64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[value(default, skip_serializing_if = "Option::is_none")]
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
/// this file reaches: it is `#[path]`-mounted (as `pub mod kernel`) into `🛂️manifest/🦀️.rs`
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
#[path = "🧪️tests/🔬️extension-activation/🦀️.rs"]
mod extension_activation_tests;
//#endregion 🔖️ExtensionActivation
