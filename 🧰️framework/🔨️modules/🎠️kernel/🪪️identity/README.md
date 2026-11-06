# Design Notes

## Preserved Source Rationale

<a id="source-notes-86"></a>

### 🦀️.rs:86

🎞️ CW3 kernel cut-over: MutationId/ActorId/ArtifactId/ArtifactVersion/SchemaId moved to
`protocol_core` (frozen contract `.🧬semio/🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md`),
re-exported here under their original names — shapes are unchanged (plain serde-transparent
String/u64 newtypes), so every existing reference (internal `kernel` types below, and external
crates like `framework/sync`/`framework/product/os/semio_hub` that import them straight from
`semio_framework`) keeps resolving without edits. `SchemaVersion` below is NOT re-exported
from `protocol_core` — that crate's own `SchemaVersion` is `u32`-shaped (a distinct, unrelated
protocol-format concept), incompatible with this kernel's `String`-shaped version below, which
several external crates (`framework/sync`, semio_hub storage crates) still construct from plain
strings; moving it would be a breaking shape change out of this wave's scope.

<a id="source-notes-137"></a>

### 🦀️.rs:137

🎯️ W6 kernel unification: re-exports `protocol_core::HybridLogicalTimestamp` now (identical
`{actor, physical_ms, logical}` shape, plus a real actor-tiebroken `Ord`/`PartialOrd` the old
local struct lacked). The CW3-era deferral note this region used to carry (kept local because
this struct's `#[serde(rename_all = "camelCase")]` put `physicalMs` on the wire, vs.
`protocol_core`'s unrenamed `physical_ms`) is resolved: this wave already rewired the JSON/wire
boundary end-to-end (W5's binary `protocol_wire` codec, its TS twin, and the fixture
byte-identity canary all speak `physical_ms`), so the wire-format reconciliation this note
deferred is verified, not assumed.

<a id="source-notes-192"></a>

### 🦀️.rs:192

🚧️ BLOCKED (26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS): `artifact:
ArtifactId` below is `protocol_core::ArtifactId` — defined in the `semio-framework-replication`
crate (`📡️replication/🆔️ids/🦀️.rs`), which implements its OWN `crate::value::ToValue`
(`🌱️value/🦀️.rs`, separately path-mounted into that crate) — a structurally-identical but
nominally DISTINCT trait from this crate's `dsl::ToValue` (`os_dsl::schema::{ToValue,
FromValue}`, canonical definition per `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs`
L332-347). Confirmed via source inspection, not `cargo check` alone: `ArtifactId` does not
implement `dsl::ToValue`, so `#[derive(ToValue, FromValue)]` here would fail. Fixing it means
giving replication's frozen protocol newtypes a second, os-kernel-flavored `ToValue`/`FromValue`
impl — out of this pass's scope (replication is a frozen contract, not a named target of this
ticket). Left serde-only.

<a id="source-notes-491"></a>

### 🦀️.rs:491

🔑️ A capability's identity — dotted/colon-scoped strings (`storage.read`, `http:<origin>`,
`messaging.plugin:<id>`, `extension.invoke:<id>`, ...) per `📓️design-abi.md` §5's catalogue.
A `String` newtype rather than a closed enum: several members carry a caller-chosen parameter
(`<origin>`/`<uri>`/`<id>`/`<point>`) the broker matches by prefix, and the catalogue is
expected to grow as new capability surfaces land — an exhaustive enum would need a matching
wildcard arm anyway.

<a id="source-notes-523"></a>

### 🦀️.rs:523

📬️ A broker-issued grant answering a `CapabilityRequest` — `📓️design-abi.md` §5.
Named `BrokerCapabilityGrant`, not the design prose's bare `CapabilityGrant`: this file
already has a `CapabilityGrant` (above, `🔖️Capability` region) for the unrelated kernel-level
action/window capability model (`ActionContext.granted_capabilities`), with live consumers
outside this packet's owned paths (`📦️packages/🦀️rust/🦀️.rs`'s re-export list) — see the
report's naming-collision note.

<a id="source-notes-655"></a>

### 🦀️.rs:655

🔌️ Canonical installed-extension descriptor the host queries at plugin-activation time —
`extension-activation` packet (`📌️important.md`): "on plugin activation, the kernel queries
installed descriptors for `extends == plugin_id` and activates each as `ActorKind::Extension`,
pinned to the parent's shard, capabilities scoped to the parent". Deliberately independent of
the `.sxt` wire-format-specific `ExtensionPackageManifest` (`💻️os/🔨️modules/🧩️extension`, a
different crate's mount set than this file's) and of the guest-side `ExtensionManifest`
(`semio-plugin`) — this shape uses only vocabulary this very file already owns
(`CapabilityId`/`CapabilityRequest`), so `extensions_extending` stays callable from every crate
this file reaches: it is `#[path]`-mounted (as `pub mod kernel`) into `🛂️manifest/🦀️.rs`
alone, which is itself `#[path]`-mounted into THREE crates — `semio-framework` (root),
`semio-framework-graph`, and `semio-s-plugin-stdio` (verified: `grep -rn '#\[path.*🎠️kernel'`
and `grep -rn '#\[path.*🛂️manifest/🦀️component'`, both over absolute paths) — without pulling in
the `.sxt`/guest-SDK dependency edge. `💻️os/🖥️host`'s own install-region `InstalledExtension` is
the `.sxt`-shaped twin of this — see that type's docstring for why the two are NOT unified (same
dependency-edge-law reason `PackagePluginDependency`'s own docstring gives for its wire-shape
duplication).
