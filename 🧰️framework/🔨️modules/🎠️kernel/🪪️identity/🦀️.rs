//! 🪪️ Kernel identifiers, rights, capabilities, invocation records, broker quotas and activation events: the vocabulary every kernel contract names.

use replication::ArtifactDiff;
use replication::UndoPolicy;
use semio_framework_actor::RetainedTurnInput;
use semio_framework_value::DslValue;
use semio_framework_value::ValueError;
use semio_framework_value::ValueRefusalKind;
use semio_framework_value::retained_clone::RetainedCloneGrant;
use semio_framework_value_derive::FromValue;
use semio_framework_value_derive::ToValue;
use serde::Deserialize;
use serde::Serialize;

//#region 🔖️Identifiers
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[derive(semio_framework_value::RetireOwned)]
pub struct ArtifactHandle(pub u128);

/// 🌉️ Hand-written, not derived: `u128` has no `ToValue`/`FromValue` scalar impl (no JavaScript
/// number equivalent), so the mirror carries it as a decimal string — same treatment
/// `BrokerCapabilityGrant.token` gets in `🎠️kernel/🦀️.rs`'s own doc.
impl semio_framework_value::ToValue for ArtifactHandle {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.0.to_string())
    }
}

impl semio_framework_value::FromValue for ArtifactHandle {
    fn from_value(value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            DslValue::String(s) => s.parse().map(ArtifactHandle).map_err(|_| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a u128 decimal string for ArtifactHandle, found {s:?}"))),
            other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for ArtifactHandle, found {other:?}"))),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[derive(semio_framework_value::RetireOwned)]
pub struct WindowHandle(pub u128);

/// 🌉️ Hand-written, not derived — see [`ArtifactHandle`]'s impl doc directly above (same `u128`
/// decimal-string mirror).
impl semio_framework_value::ToValue for WindowHandle {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.0.to_string())
    }
}

impl semio_framework_value::FromValue for WindowHandle {
    fn from_value(value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            DslValue::String(s) => s.parse().map(WindowHandle).map_err(|_| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a u128 decimal string for WindowHandle, found {s:?}"))),
            other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for WindowHandle, found {other:?}"))),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct AssetHandle(pub u128);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CapabilityToken(pub u128);

semio_framework_value::artifact_retire_leaf!(CapabilityToken);

/// 🌉️ Hand-written, not derived — see [`ArtifactHandle`]'s impl doc above (same `u128`
/// decimal-string mirror).
impl semio_framework_value::ToValue for CapabilityToken {
    fn to_value(&self) -> DslValue {
        DslValue::String(self.0.to_string())
    }
}

impl semio_framework_value::FromValue for CapabilityToken {
    fn from_value(value: DslValue) -> Result<Self, semio_framework_value::ValueError> {
        match value {
            DslValue::String(s) => s.parse().map(CapabilityToken).map_err(|_| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a u128 decimal string for CapabilityToken, found {s:?}"))),
            other => Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("expected a string for CapabilityToken, found {other:?}"))),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
#[derive(semio_framework_value::RetireOwned)]
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
pub use ::replication::{ActorId, ArtifactId, ArtifactVersion, MutationId, SchemaId};

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
#[derive(semio_framework_value::RetireOwned)]
pub struct AppInstanceId(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct SchemaVersion(pub String);

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
#[derive(semio_framework_value::RetireOwned)]
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
pub use ::replication::HybridLogicalTimestamp;

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

//#endregion 🔖️Invocation
//#region 🔖️Effect

/// 🎫️ Correlates an `Effect` that expects a completion with the `Event::Completed` (or
/// `Event::HttpChunk`/`JobProgress`/`JobCompleted`) that answers it — `request-id = u64` in
/// `📜️wit/📜️types.wit`. Minted host-side per pending request; the guest SDK's request registry
/// (`📓️design-abi.md` §4) parks a future on it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize, ToValue, FromValue)]
#[serde(transparent)]
#[value(transparent)]
#[derive(semio_framework_value::RetireOwned)]
pub struct RequestId(pub u64);

//#endregion 🔖️Effect

#[derive(Clone, Debug, PartialEq, ToValue, FromValue)]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
pub struct AppEvent {
    pub kind: String,
    pub payload: DslValue,
}

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

//#endregion 🔖️Window
//#region 🔖️Event
/// ✅️ The shared `result<pack, fault-bytes>` shape from `📜️wit/📜️types.wit`, carried by
/// `Event::Completed`/`Event::JobCompleted` and `Effect::Respond`. `Err` bytes are an encoded
/// fault the SDK decodes by originating request kind — the host never interprets it.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
pub enum RequestOutcome {
    Ok(Vec<u8>),
    Err(Vec<u8>),
}

//#endregion 🔖️Event
//#region 🔖️ActivationEvent

/// 🚀️ Why an instance was activated — `📓️design-abi.md` §2's activation-event list, matched
/// against a `manifest::PackageDescriptor.activation_events` declaration at install time.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[value(rename_all = "camelCase", rename_all_fields = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
pub enum ActivationEvent {
    OnCommand { id: String },
    OnViewVisible { id: String },
    OnFileType { ext: String },
    OnArtifactKind { kind: String },
    OnExtensionRequest { point: String },
    OnStartupFinished,
}

//#endregion 🔖️ActivationEvent
//#region 🔖️Budget

/// ⛽️ Per-turn resource ceiling handed to `reactor::poll` — `📜️wit/📜️reactor.wit`'s `budget`
/// record.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Budget {
    pub retained: RetainedTurnInput,
    pub retained_policy:semio_framework_value::RetainedCloneGrant,
    pub work_fuel:u64,
    pub fuel: u64,
    pub deadline_us: u64,
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
#[derive(semio_framework_value::RetireOwned)]
pub struct CapabilityId(pub String);

/// 🙏️ A guest's ask for a capability — `📓️design-abi.md` §5. Replaces `CapabilityRequirement`
/// for the plugin/extension actor runtime. The kernel-level `CapabilityRequirement`/`Rights`/
/// `Scope` action-dispatch model (above, `🔖️Capability` region) stays as-is: it has live
/// consumers outside this packet's owned paths (`🔌️plugin/🏗️builder`, `🔌️plugin/🖥️host`,
/// `🔌️plugin/🦀️.rs`) — see this packet's report for the full consumer list.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
#[serde(rename_all = "camelCase")]
#[value(rename_all = "camelCase")]
#[derive(semio_framework_value::RetireOwned)]
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
#[derive(semio_framework_value::RetireOwned)]
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
#[derive(semio_framework_value::RetireOwned)]
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
#[derive(semio_framework_value::RetireOwned)]
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

//#endregion 🔖️ExtensionActivation

#[cfg(test)]
use replication::wire::command_ingress::COMMAND_BATCH_MAXIMUM_ITEMS;
#[cfg(test)]
use replication::wire::command_ingress::COMMAND_PAGE_MAXIMUM_BYTES;
#[cfg(test)]
use replication::wire::command_ingress::CommandBatch;
#[cfg(test)]
use replication::wire::command_ingress::CommandBatchDriver;
#[cfg(test)]
use replication::wire::command_ingress::CommandBatchProgress;
#[cfg(test)]
use replication::wire::command_ingress::CommandDriverRegistry;
#[cfg(test)]
use replication::wire::command_ingress::CommandEnvelope;
#[cfg(test)]
use replication::wire::command_ingress::CommandEnvelopeSet;
#[cfg(test)]
use replication::wire::command_ingress::CommandIngressStatus;
#[cfg(test)]
use replication::wire::command_ingress::CommandPageSet;
#[cfg(test)]
use replication::wire::command_ingress::FixedCommandPage;
#[cfg(test)]
use replication::wire::command_ingress::PagedCommand;
#[cfg(test)]
use replication::wire::command_ingress::RejectedCommandBuild;
#[cfg(test)]
use replication::wire::command_ingress::RejectedCommandBuildRegistry;
#[cfg(test)]
use semio_framework_diagnostic::Fault;

//#region 🔖️ExtensionActivation
#[cfg(test)]
#[path = "../🧪️tests/🔬️extension-activation/🦀️.rs"]
mod extension_activation_tests;
//#endregion 🔖️ExtensionActivation
