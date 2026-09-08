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

/// @emoji 🎛️ A dispatched invocation of a `CommandDefinition` — the command mirror of `ActionInvocation`.
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
        #[value(default, skip_serializing_if = "Option::is_none")]
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
        #[value(default, skip_serializing_if = "Option::is_none")]
        read_as: Option<String>,
        import_action: String,
        #[serde(default)]
        #[value(default)]
        multiple: bool,
    },
    /// @emoji 🎞️ Asks the shell to decode a video (via file picker, or `payload` bytes when the
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
    /// @emoji ✨️ Spawns a plugin instance (idempotent on `os_instance_id`) without focusing it.
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
    /// @emoji 🪟️ Spawns (if needed) and focuses/navigates to a plugin instance.
    OpenPluginInstance {
        plugin_id: String,
        app_id: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
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
        #[value(default, skip_serializing_if = "Option::is_none")]
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
        #[value(default, skip_serializing_if = "Option::is_none")]
        args: Option<DslValue>,
        delay_ms: u64,
    },
    /// @emoji ⏪️ Asks the shell to redispatch a shell-owned command (dock/theme/locale/panel chrome)
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
    /// @emoji 🔍️ On-demand io-dialect lookup — the routing table itself is preloaded in
    /// `Event::InstanceOpen`; this is only for entries that weren't.
    RegistryQuery {
        req: RequestId,
        kind: String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        #[value(default, skip_serializing_if = "Option::is_none")]
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
    /// @emoji 💡️ Asks the shell to open its own host-owned ephemeral inference port for the active
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

/// @emoji 🖼️ One icon-render export request: the destination filename plus the opaque icon-scene
/// render request forwarded to the shell's `iconRenderPort`.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct IconRenderExportItem {
    pub filename: String,
    pub request: DslValue,
}
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

/// @emoji 🐢️ What part of the shell's rendered UI an action actually invalidates — lets `refresh-ui`
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

/// 🧾️ One host-projectable row in the session command timeline. The payload is deliberately
/// presentation-neutral: the host owns windowing and retains entries beyond any visible range.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
pub struct HistoryEntry {
    pub seq: u64,
    pub action_id: String,
    pub label: String,
    pub kind: String,
    pub timestamp: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub op_lines: Vec<String>,
    #[serde(default)]
    #[value(default)]
    pub applied: bool,
    #[serde(default)]
    #[value(default)]
    pub revertible: bool,
    #[serde(default = "history_entry_count")]
    #[value(default = "history_entry_count")]
    pub count: u32,
}

// 🚫️async: E4 fn-pointer slot
fn history_entry_count() -> u32 {
    1
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
}

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

/// @emoji 🎛️ Context for a dispatched `CommandInvocation` — the command mirror of `ActionContext`.
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
pub use semio_framework_os_kernel::{decode_presence_peer, encode_presence_peer, PresencePeer, PresenceUi, PresenceViewKind, PresenceWindowView};
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
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
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
pub const UI_TURN_PATCHES_MAXIMUM: usize = 1;
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

const _: () = assert!(size_of::<(UiTurnPatchRetireKey, UiTurnPatchContents)>() <= 4096);
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

    fn close_one(&mut self) -> bool {
        for offset in 0..UI_TURN_PATCH_RETIRE_SLOTS {
            let Some(index) = self.close_cursor.checked_add(offset).map(|index| index % UI_TURN_PATCH_RETIRE_SLOTS) else { return false };
            let slot = &mut self.slots[index];
            if !slot.reserved || slot.contents.is_none() {
                continue;
            }
            let Some(next) = index.checked_add(1) else { return false };
            self.close_cursor = next % UI_TURN_PATCH_RETIRE_SLOTS;
            let contents = slot.contents.as_mut().expect("selected returned patch owner");
            if contents.close_step(1, 4096).expect("exact returned patch retirement").complete && contents.terminal_is_empty() {
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
    arena.close_one()
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
        if session == 0 {
            return Err(owner);
        }
        let Some(slot) = self.slots.iter().position(|slot| slot.state == UiTurnPatchTransportState::Vacant) else { return Err(owner) };
        let Some(epoch) = self.slots[slot].epoch.checked_add(1) else { return Err(owner) };
        self.slots[slot] = UiTurnPatchTransportSlot { epoch, session, state: UiTurnPatchTransportState::Building, owner: Some(owner), external: true };
        Ok(UiTurnPatchTransportKey { slot, epoch, session })
    }

    fn slot_mut(&mut self, key: UiTurnPatchTransportKey) -> Option<&mut UiTurnPatchTransportSlot> {
        let slot = self.slots.get_mut(key.slot)?;
        (slot.epoch == key.epoch && slot.session == key.session && slot.state != UiTurnPatchTransportState::Vacant).then_some(slot)
    }

    fn close_one(&mut self) -> Result<UiTurnPatchTransportProgress, &'static str> {
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
            let step = owner.close_step_with_grant(1, 4096)?;
            if step.complete {
                let epoch = self.slots[index].epoch;
                self.slots[index] = UiTurnPatchTransportSlot { epoch, ..UiTurnPatchTransportSlot::default() };
                return Ok(UiTurnPatchTransportProgress::Pending { released_items: 1, released_bytes: 0 });
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

const _: () = assert!(size_of::<(UiTurnPatchTransportKey, Option<UiTurnPatches>)>() <= 4096);
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
        if session != expected_session || session == 0 {
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
    arena.close_one()
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

/// 🧰️ The fixed exact-owner patch page emitted by one turn.
#[derive(Debug, Default)]
pub struct UiTurnPatches {
    contents: UiTurnPatchContents,
    retirement: Option<UiTurnPatchRetireKey>,
}

pub enum UiTurnPatchTransfer<T> {
    Empty,
    Transferred(T),
    Refused,
}

impl PartialEq for UiTurnPatches {
    fn eq(&self, other: &Self) -> bool {
        self.contents.pending.get() == other.contents.pending.get()
    }
}

impl UiTurnPatches {
    #[expect(clippy::result_large_err, reason = "Refusal returns the exact fixed patch so the caller retains its retirement obligation.")]
    pub fn try_push_ui_patch(&mut self, patch: UiPatch) -> Result<(), UiPatch> {
        if !self.contents.terminal_is_empty() || self.contents.pending.source_mut().is_err() {
            return Err(patch);
        }
        if self.retirement.is_none() {
            let Ok(mut arena) = UI_TURN_PATCH_RETIRE_ARENA.try_lock() else {
                return Err(patch);
            };
            let Some(retirement) = arena.reserve() else { return Err(patch) };
            self.retirement = Some(retirement);
        }
        *self.contents.pending.source_mut().expect("preflighted pending patch slot") = Some(patch);
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = &UiPatch> {
        self.contents.pending.get().into_iter()
    }

    pub fn len(&self) -> usize {
        usize::from(!self.contents.terminal_is_empty())
    }

    pub fn is_empty(&self) -> bool {
        self.contents.terminal_is_empty()
    }

    pub fn try_transfer_one<T>(&mut self, transfer: impl FnOnce(UiPatch) -> Result<T, UiPatch>) -> UiTurnPatchTransfer<T> {
        let Ok(source) = self.contents.pending.source_mut() else {
            return UiTurnPatchTransfer::Refused;
        };
        let Some(patch) = source.take() else { return UiTurnPatchTransfer::Empty };
        match transfer(patch) {
            Ok(value) => UiTurnPatchTransfer::Transferred(value),
            Err(patch) => {
                *source = Some(patch);
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
        if !self.contents.terminal_is_empty() {
            let mut step = self.contents.close_step(items, bytes)?;
            step.complete = false;
            return Ok(step);
        }
        if let Some(retirement) = self.retirement {
            let mut arena = match UI_TURN_PATCH_RETIRE_ARENA.try_lock() {
                Ok(arena) => arena,
                Err(std::sync::TryLockError::WouldBlock) => return Ok(UiValueRetirementStep::default()),
                Err(std::sync::TryLockError::Poisoned(_)) => return Err("turn patch retirement arena is poisoned"),
            };
            if !arena.release_empty(retirement) {
                return Err("exact turn patch retirement reservation missing");
            }
            self.retirement = None;
            return Ok(UiValueRetirementStep { progressed: true, released_items: 1, ..Default::default() });
        }
        Ok(UiValueRetirementStep { complete: true, ..Default::default() })
    }
}

impl IntoIterator for UiTurnPatches {
    type Item = UiPatch;
    type IntoIter = std::option::IntoIter<UiPatch>;

    fn into_iter(self) -> Self::IntoIter {
        let mut owner = self;
        owner.contents.pending.source_mut().expect("only a readable turn patch can be transferred").take().into_iter()
    }
}

impl Drop for UiTurnPatches {
    fn drop(&mut self) {
        let Some(retirement) = self.retirement.take() else { return };
        let contents = std::mem::take(&mut self.contents);
        UI_TURN_PATCH_HANDBACKS[retirement.slot].publish(retirement, contents);
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
#[path = "📤️return/📦️content/🗣️dialects/🦀️.rs"]
mod return_content_dialect_tests;

#[path = "📤️return/📦️content/🦀️.rs"]
pub mod return_content;

#[path = "📤️return/📦️content/💌️message/🦀️.rs"]
pub mod return_message;

#[cfg(test)]
#[path = "📤️return/📦️content/🖼️framing/🦀️.rs"]
mod return_content_framing_tests;

#[cfg(test)]
#[path = "📤️return/📦️content/💌️message/🧪️tests/🦀️.rs"]
mod return_content_message_tests;

#[cfg(test)]
#[path = "📤️return/🏠️source/🦀️.rs"]
mod return_source_inventory_tests;

#[cfg(test)]
#[path = "📤️return/🏠️source/📚️entries/🧪️tests/🦀️.rs"]
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
