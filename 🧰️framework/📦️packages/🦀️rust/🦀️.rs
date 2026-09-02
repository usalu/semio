//! 🥅️ Render-independent framework kernel: declarative {@link UiNode}, {@link Platform}, {@link ActionBus}.

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as protocol_core;
// 🔁️ ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W1: `store`
// alias (same pattern every plugin's own glue.rs already uses) so `🔁️workflow/🦀️.rs`'s
// `store::ArtifactPack`/`store::ArtifactDsl`/etc. references resolve once mounted below — this
// crate never referenced `store::` directly before, only through re-exported item names.
extern crate semio_framework_os_kernel as store;
// 🔁️ self-alias so `🔁️workflow/🦀️.rs`'s own `use semio_framework::{...}` lines resolve
// once mounted below — this crate never needed to refer to itself by its external name before.
extern crate self as semio_framework;

pub use ui_wgpu::wgpu::IconName;
pub use ui_wgpu::wgpu::{Locale, Terminology};

//#region 🧬️SchemaMetadata
#[cfg(feature = "typegen")]
pub mod schema_metadata {
    use std::collections::HashSet;

    /// 🧬️ One versioned framework wire type and its owned TypeScript projection.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct SchemaMetadata {
        pub name: &'static str,
        pub version: u16,
        pub typescript: &'static str,
    }

    pub const TYPES: &[SchemaMetadata] = &[
        SchemaMetadata {
            name: "ActionAddress",
            version: 1,
            typescript: r####"/**
 * @emoji 📍️ Fully qualified address of an action owned by one concrete window instance.
 */
export type ActionAddress = { pluginId: string, appId: string, modeId: string, windowKindId: string, windowInstanceId: string, actionId: string, };"####,
        },
        SchemaMetadata {
            name: "ActionArgControl",
            version: 1,
            typescript: r####"/**
 * @emoji 🎚️ Declarative input control for one action argument — a lean manifest-altitude enum,
 * deliberately NOT `ui_wgpu::wgpu::UiControlNode` (whose variants embed live values and immediate-dispatch
 * wiring). Renderers map each variant onto a staged form field. Tagged with `kind` to mirror the
 * sibling `UtilityNode`/`UiControlNode` declarative-tree convention.
 */
export type ActionArgControl = { "kind": "text", placeholder?: string, } | { "kind": "number", min?: number, max?: number, step?: number, } | { "kind": "slider", min: number, max: number, step?: number, unit?: string, } | { "kind": "toggle" } | { "kind": "select", options: Array<ActionArgOption>, } | { "kind": "vec3" } | { "kind": "iconSelect", classifierKind: string, } | { "kind": "artifactKind", roles: Array<AppRole>, } | { "kind": "surfaceApp", roles: Array<AppRole>, dialectArg: string, };"####,
        },
        SchemaMetadata {
            name: "ActionArgDef",
            version: 1,
            typescript: r####"/**
 * @emoji 📝️ Declares one argument of an action: its `id` (the JSON key sent in `ActionDescriptor.args`),
 * human `label`, stored value `schema` (see `🔖️ArgSchema` — D6: this is the sole persisted truth,
 * `control()` below is derived from it), an optional widget `presentation` hint, whether it is
 * `required`, an optional `default` value, and an optional `description`. An empty
 * `ActionDefinition.args` (the common case) means a no-argument action.
 */
export type ActionArgDef = { id: string,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown, schema: ArgSchema, presentation?: ArgPresentation, required: boolean, default?: unknown, description?: string, };"####,
        },
        SchemaMetadata {
            name: "ActionArgOption",
            version: 1,
            typescript: r####"/**
 * @emoji 🔘️ One selectable option of a `Select` argument control — the persisted `value` and its
 * human `label`.
 */
export type ActionArgOption = { value: string,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel`. Not yet owned-schema-mirrored
 * (follow-up: `LocalizedLabel` itself has no `TS` impl).
 */
label: unknown, };"####,
        },
        SchemaMetadata {
            name: "ActionDefinition",
            version: 1,
            typescript: r####"/**
 * @emoji 📇️ Declares one action an app can receive via `ActionDescriptor.action`.
 */
export type ActionDefinition = { id: string,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown, kind: ActionKind, iconId: IconName,
/**
 * 📝️ Typed argument declarations. Empty (the common case) = a no-argument action.
 */
args: Array<ActionArgDef>, keys?: string, inPalette: boolean, category?: string,
/**
 * 🎯️ Effects/policy/execution/use_when — see `🔖️ActionSemantics`. Defaulted per-`kind` by
 * `ActionSemantics::for_kind` in `new`/`new_catalog`; every struct-update-syntax call site
 * (`ActionDefinition { .., ..Self::new_catalog(..) }`) inherits it unchanged from the base
 * expression, so none of the ~126 declaration sites need touching.
 */
semantics: ActionSemantics, };"####,
        },
        SchemaMetadata { name: "ActionDescriptor", version: 1, typescript: r####"export type ActionDescriptor = { controllerId: string, action: string, args?: unknown, };"#### },
        SchemaMetadata {
            name: "ActionInvocation",
            version: 1,
            typescript: r####"/**
 * @emoji 📨️ One addressed action invocation with named JSON arguments.
 */
export type ActionInvocation = { address: ActionAddress, arguments: Record<string, unknown>, };"####,
        },
        SchemaMetadata {
            name: "ActionKind",
            version: 1,
            typescript: r####"/**
 * @emoji 🗂️ Classifies a declared action by how it interacts with VCS history.
 */
export type ActionKind = "mutation" | "view" | "history" | "clipboard" | "shell" | "interaction";"####,
        },
        SchemaMetadata {
            name: "ActionRef",
            version: 1,
            typescript: r####"/**
 * 📇️ A relative action id used by declarations nested beneath an owning window kind.
 * Distinct from `ActionAddress`, which qualifies a dispatched invocation down to a window instance.
 */
export type ActionRef = string;"####,
        },
        SchemaMetadata {
            name: "ActionSemantics",
            version: 1,
            typescript: r####"/**
 * @emoji 🎯️ What an `ActionDefinition`/`CommandDefinition` MEANS to an agent: effects, policy,
 * execution shape, and natural-language framing (`use_when`/`examples`) — everything the MCP
 * catalog compiler needs beyond the UI-shaped fields already on the definition itself. Defaulted
 * per-kind by `for_kind` at construction time; `#[serde(default)]` on the owning field additionally
 * tolerates old serialized manifests with no `semantics` key at all (deserializes to
 * `ActionSemantics::default()`, the type-level default below — NOT re-derived from `kind`, since
 * serde field defaults cannot see sibling fields).
 */
export type ActionSemantics = { effects: CapabilityEffects, policy: CapabilityPolicy, execution: CapabilityExecution, description?: unknown, useWhen: Array<string>, examples: Array<string>, };"####,
        },
        SchemaMetadata {
            name: "ActivationEvent",
            version: 1,
            typescript: r####"/**
 * 🚀️ Why an instance was activated — `📓️design-abi.md` §2's activation-event list, matched
 * against a `manifest::PackageDescriptor.activation_events` declaration at install time.
 */
export type ActivationEvent = { "onCommand": { id: string, } } | { "onViewVisible": { id: string, } } | { "onFileType": { ext: string, } } | { "onArtifactKind": { kind: string, } } | { "onExtensionRequest": { point: string, } } | "onStartupFinished";"####,
        },
        SchemaMetadata {
            name: "AgentContributions",
            version: 1,
            typescript: r####"/**
 * @emoji 🤖️ What a package OFFERS to agents — see the region header above for the critical
 * `capability_requests` vs `AgentContributions` distinction. `capabilities` are fully-qualified
 * capability ids (the same grammar `🌉️mcp/🗂️catalog` compiles — `<plugin_id>.<app_id>.
 * <action_id>` / `….cmd.<id>` / `….mode.<mode_id>.<id>`, `📋️master.md` §3.1); `promoted` is the
 * subset exposed as a first-class MCP tool (`tools/list`) rather than only reachable via
 * `capabilities.search`/`capabilities.describe`. Both empty by default — an absent
 * `AgentContributions` (the `Option` on `PackageDescriptor` stays `None`) means "not yet
 * agent-enabled", never "agent-enabled with zero capabilities" (an empty-but-`Some` value).
 */
export type AgentContributions = { capabilities: Array<string>, promoted: Array<string>, };"####,
        },
        SchemaMetadata {
            name: "AppDefinition",
            version: 1,
            typescript: r####"export type AppDefinition = { id: string,
/**
 * 👁️✏️ Whether this surface may mutate the artifact it is bound to — see `AppRole`.
 */
role: AppRole,
/**
 * 🎯️ The dialect coordinate (artifact kind, standard, subset) this surface is bound to — see
 * `ArtifactDialect`. Together with `role` this derives the canonical `id` via `surface_app_id`.
 */
dialect: ArtifactDialect,
/**
 * 🗣️ The app's own display name (e.g. "Puzzle 3D") — manifest-level, locale×terminology-checked,
 * see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown, breadcrumb: Array<string>, iconId?: IconName, controllerId: string,
/**
 * 🚧️ `Modes` is `NonEmptyVec<ModeDefinition>`, whose `serde(try_from/into = "Vec<T>")` wire
 * format is a flat array — not the `{ first, rest }` shape owned schema exporter would infer from the struct
 * fields, so the wire-accurate array shape is supplied directly instead of deriving `TS` on
 * `NonEmptyVec` itself.
 */
modes: ModeDefinition[], defaultModeId: string,
/**
 * 🚧️ See `modes` above — `WindowKinds` is `NonEmptyVec<WindowKindDefinition>`.
 */
windowKinds: WindowKindDefinition[], panelTabs: Array<PanelTabDefinition>, keybindings: Array<Keybinding>,
/**
 * 🧰️ The interactive utilities this app exposes (referenced by `WindowKindDefinition.utilities`).
 */
utilities: Array<UtilityDefinition>,
/**
 * 🛠️ The mode-level tools this app exposes (referenced by `ModeDefinition.tools`).
 */
tools: Array<ToolDefinition>,
/**
 * 🎛️ Commands owned by this app and active whenever it is focused.
 */
commands: Array<CommandDefinition>,
/**
 * 🕹️ The interaction domains (hover + selection) this app exposes (referenced by
 * `WindowKindDefinition.interactions`) — see `crate::InteractionDefinition`.
 */
interactions: Array<InteractionDefinition>, namedLayouts: Array<NamedLayout>, defaultLayout?: WindowLayout,
/**
 * 🗣️ Terminology ids this app declares beyond the implicit "native" default.
 */
terminologies: Array<string>,
/**
 * 🗺️ Terminology id -> full replacement breadcrumb (product + app segments), e.g. "reuse" ->
 * ["Entwerfen mit Bestand", "Aggregator"]; ids absent here keep the canonical breadcrumb under that terminology.
 */
terminologyBreadcrumbs: { [key in string]?: Array<string> },
/**
 * 🎓️ This app's first-run walkthrough, if it declares one — see `IntroductionDefinition`.
 */
introduction?: IntroductionDefinition,
/**
 * 🎬️ Recorded, timed walkthroughs this app declares — see `TutorialDefinition`. A brand's own
 * `tutorials` (if any) are shown alongside these, never replacing them (unlike `introduction`).
 */
tutorials: Array<TutorialDefinition>,
/**
 * 🗨️ The modal form dialogs this app can open via `Effect::OpenDialog`.
 */
dialogs: Array<DialogDefinition>,
/**
 * 🔌️ This app's workflow input ports — see `crate::MediaPortSpec`.
 */
mediaInputs: Array<MediaPortSpec>,
/**
 * 🔌️ This app's workflow output ports — see `crate::MediaPortSpec`.
 */
mediaOutputs: Array<MediaPortSpec>,
/**
 * 🗂️ OS resource kinds this app produces/consumes — see `crate::ArtifactKindSpec`. Drives
 * `framework/product/os/core`'s artifact catalog registry instead of a hardcoded per-app match.
 */
artifactKinds: Array<ArtifactKindSpec>,
/**
 * 🧮️ This app's typed configuration record — see `crate::ConfigSpec`. Empty until per-app waves
 * populate it.
 */
config: ConfigSpec,
/**
 * 🎛️ This app's typed binary command grammar — see `crate::CommandGrammar`. Empty until per-app
 * waves populate it.
 */
commandGrammar: CommandGrammar,
/**
 * 🔌️ This app's typed media I/O surface — see `crate::AppIo`. Not yet populated; `media_inputs`/
 * `media_outputs`/`artifact_kinds` above remain the live source of truth until later waves migrate
 * onto this.
 */
io: AppIo, };"####,
        },
        SchemaMetadata {
            name: "AppIo",
            version: 1,
            typescript: r####"/**
 * 🔌️ An app's full media I/O surface — the document schema/type every app carries implicitly (see
 * `document_in_port`/`document_out_port`) plus whatever additional workflow ports, catalog
 * export/import formats, and OS presentation it declares itself. Scaffolding for the typed manifest
 * surface (`AppDefinition.io`); apps don't populate this yet — later waves migrate `media_inputs`/
 * `media_outputs`/`artifact_kinds` onto it.
 */
export type AppIo = { documentSchema: string, documentMediaType: MediaType,
/**
 * 🔌️ App-specific ports only — the implicit document ports are auto-injected by `all_ports`.
 */
ports: Array<MediaPortSpec>, exportFormats: Array<string>, importFormats: Array<string>, artifact: ArtifactPresentation, };"####,
        },
        SchemaMetadata {
            name: "AppRef",
            version: 1,
            typescript: r####"/**
 * 🎯️ A surface addressed across plugin boundaries.
 */
export type AppRef = { pluginId: string, appId: string, };"####,
        },
        SchemaMetadata {
            name: "AppRole",
            version: 1,
            typescript: r####"/**
 * 👁️✏️ Whether a surface may change the artifact it is bound to.
 */
export type AppRole = "viewer" | "editor";"####,
        },
        SchemaMetadata {
            name: "ApprovalMode",
            version: 1,
            typescript: r####"/**
 * @emoji 🚦️ When the gateway must pause for human approval before committing an invocation of this
 * capability.
 */
export type ApprovalMode = "never" | "whenDestructive" | "always";"####,
        },
        SchemaMetadata {
            name: "ArgFormat",
            version: 1,
            typescript: r####"/**
 * @emoji 🧬️ Semantic refinement of a `String`-typed `ArgSchema` leaf — what KIND of string this is,
 * beyond "text". Orthogonal to `ArgPresentation` (which is about the WIDGET, not the value's
 * semantics): a `Color` format could still render as free text in a minimal shell.
 */
export type ArgFormat = { "kind": "artifactRef" } | { "kind": "windowId" } | { "kind": "entityId", entityKind: string, } | { "kind": "iconId" } | { "kind": "color" } | { "kind": "uri" } | { "kind": "json" } | { "kind": "locale" } | { "kind": "terminology" } | { "kind": "artifactKind", roles: Array<AppRole>, } | { "kind": "surfaceApp", roles: Array<AppRole>, dialectArg: string, };"####,
        },
        SchemaMetadata {
            name: "ArgPresentation",
            version: 1,
            typescript: r####"/**
 * @emoji 🖼️ How to WIDGET-render an argument beyond what its `ArgSchema` alone implies — consumed by
 * `ActionArgDef::control()` (e.g. a bounded `Number` still renders `Slider` without this, but a
 * single-bound one needs it to opt in).
 */
export type ArgPresentation = { "kind": "slider" } | { "kind": "iconSelect", classifierKind: string, } | { "kind": "multiline" } | { "kind": "hidden" };"####,
        },
        SchemaMetadata {
            name: "ArgSchema",
            version: 1,
            typescript: r####"/**
 * @emoji 🌳️ The stored, engine-neutral shape of one action argument's value — see this region's
 * header comment for the D6 stored/derived split.
 */
export type ArgSchema = { "kind": "string", options: Array<ActionArgOption>, minLen?: number, maxLen?: number, pattern?: string, format?: ArgFormat, } | { "kind": "number", min?: number, max?: number, step?: number, integer: boolean, unit?: string, } | { "kind": "boolean" } | { "kind": "vec3", unit?: string, } | { "kind": "array", items: ArgSchema, minItems?: number, maxItems?: number, } | { "kind": "object", fields: Array<ActionArgDef>, } | { "kind": "any" };"####,
        },
        SchemaMetadata {
            name: "ArtifactContributionDescriptor",
            version: 1,
            typescript: r####"/**
 * 🗂️ Everything one plugin contributes onto one artifact kind it depends on — see the registration
 * gates in contract freeze §4 (accepted only when `artifact_kind`'s owner is a direct
 * `PluginManifest.dependencies` entry).
 */
export type ArtifactContributionDescriptor = { artifactKind: string, mutations: Array<ContributedMutationMetadata>, inferences: Array<ContributedInferenceMetadata>, };"####,
        },
        SchemaMetadata {
            name: "ArtifactDialect",
            version: 1,
            typescript: r####"/**
 * 🎯️ Owned serde twin of `Dialect` — the persisted/wire form; every dialect consumer outside a
 * `'static` compile-time registration (document envelopes, the hub's multi-user pin, WIT
 * `io-run`/`io-routes`, the io leaf generators) reads/writes THIS type.
 */
export type ArtifactDialect = { artifactKind: string, standard: string, subset: string, };"####,
        },
        SchemaMetadata { name: "ArtifactKind", version: 1, typescript: r####"export type ArtifactKind = "document" | "projection" | "window" | "asset" | "network" | "backbone" | "engine";"#### },
        SchemaMetadata {
            name: "ArtifactKindSpec",
            version: 1,
            typescript: r####"/**
 * 🗂️ An app-declared OS resource kind (e.g. a 3D mesh format, a raster format) — the manifest-level
 * counterpart to `AppBuilder::artifact_kind(...)` (`framework/plugin/rs`), letting `framework/product/os/core`
 * build its artifact catalog from `AppDefinition.artifact_kinds` at plugin registration time instead of
 * hardcoding a per-app match on kind-id strings. Carries the manifest-level media-kind fields
 * (`media_type`/`schema`/`export_formats`/`import_formats`) directly
 * so one spec carries both the OS-catalog presentation shape and the `MediaType` a wire actually negotiates
 * — see `crate::media_types_compatible`. `OsArtifactDescriptor` (`framework/product/os/core`) threads
 * `media_type` through so registry lookups return it alongside the rest of the descriptor.
 */
export type ArtifactKindSpec = { id: string, name: string, sourceFormat: string, componentKind: string, dimension: string, mediaCapability: OsMediaCapability, mediaType: MediaType, schema: string,
/**
 * 🗄️ Export target format kind ids (string, the legacy format enum was retired — ticket 26/08/11/
 * SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6).
 */
exportFormats: Array<string>, importFormats: Array<string>,
/**
 * 🗄️ Stdio export target kind ids (e.g. `stdio.json`) — additive peer of `export_formats`.
 */
exportStdioKinds: Array<string>,
/**
 * 🗄️ Stdio import source kind ids — additive peer of `import_formats`.
 */
importStdioKinds: Array<string>, };"####,
        },
        SchemaMetadata {
            name: "AssetDeclaration",
            version: 1,
            typescript: r####"/**
 * 📦️ One asset bundled with a package and preloaded into `kernel::Event::InstanceOpen.assets` —
 * `📓️design-abi.md` §2's `read-asset` replacement.
 */
export type AssetDeclaration = { name: string, mediaType: MediaType, sizeBytes: bigint, sha256: string, };"####,
        },
        SchemaMetadata {
            name: "CapabilityEffects",
            version: 1,
            typescript: r####"/**
 * @emoji 🧮️ What one capability touches — read/write resource selectors plus the three coarse flags
 * the gateway's policy/preview machinery gates on.
 */
export type CapabilityEffects = { reads: Array<ResourceSelector>, writes: Array<ResourceSelector>, external: boolean, destructive: boolean, reversible: boolean, };"####,
        },
        SchemaMetadata {
            name: "CapabilityExecution",
            version: 1,
            typescript: r####"/**
 * @emoji ⚙️ Preview/undo/idempotency/cancellation shape of one capability invocation.
 */
export type CapabilityExecution = { preview: PreviewMode, undo: UndoMode, idempotency: IdempotencyMode, expectedRevision: boolean, cancellable: boolean, class: ExecutionClass, interactiveJob: "unclassified" | "migrated" | "batchOnlyPendingRewrite" | "forbiddenFromUi" | "deleted", };"####,
        },
        SchemaMetadata {
            name: "CapabilityId",
            version: 1,
            typescript: r####"/**
 * 🔑️ A capability's identity — dotted/colon-scoped strings (`storage.read`, `http:<origin>`,
 * `messaging.plugin:<id>`, `extension.invoke:<id>`, ...) per `📓️design-abi.md` §5's catalogue.
 * A `String` newtype rather than a closed enum: several members carry a caller-chosen parameter
 * (`<origin>`/`<uri>`/`<id>`/`<point>`) the broker matches by prefix, and the catalogue is
 * expected to grow as new capability surfaces land — an exhaustive enum would need a matching
 * wildcard arm anyway.
 */
export type CapabilityId = string;"####,
        },
        SchemaMetadata {
            name: "CapabilityPolicy",
            version: 1,
            typescript: r####"/**
 * @emoji 🛡️ The scope/approval gate a capability invocation must clear — `scopes` are
 * `kernel::CapabilityId`s (the Broker's own enforcement primitive, see `🔖️Kernel` below), never a
 * parallel string vocabulary: `ExtensionPointDeclaration.capability_allowance` already establishes
 * that `kernel::CapabilityId` is reachable from this crate with no dependency cycle.
 */
export type CapabilityPolicy = { scopes: Array<CapabilityId>, approval: ApprovalMode, };"####,
        },
        SchemaMetadata {
            name: "CapabilityRequest",
            version: 1,
            typescript: r####"/**
 * 🙏️ A guest's ask for a capability — `📓️design-abi.md` §5. Replaces `CapabilityRequirement`
 * for the plugin/extension actor runtime. The kernel-level `CapabilityRequirement`/`Rights`/
 * `Scope` action-dispatch model (above, `🔖️Capability` region) stays as-is: it has live
 * consumers outside this packet's owned paths (`🔌️plugin/🏗️builder`, `🔌️plugin/🖥️host`,
 * `🔌️plugin/🦀️.rs`) — see this packet's report for the full consumer list.
 */
export type CapabilityRequest = { id: CapabilityId, scope: string, reason: string, optional: boolean, };"####,
        },
        SchemaMetadata { name: "CapabilityRequirement", version: 1, typescript: r####"export type CapabilityRequirement = { artifact: ArtifactKind, rights: Rights, scope: Scope, };"#### },
        SchemaMetadata {
            name: "CommandAddress",
            version: 1,
            typescript: r####"/**
 * @emoji 📍️ Fully qualified address of one command.
 */
export type CommandAddress = { owner: CommandOwnerAddress, commandId: string, };"####,
        },
        SchemaMetadata {
            name: "CommandDefinition",
            version: 1,
            typescript: r####"/**
 * @emoji 🎛️ Declares one command: a categorized verb offered in the footer command panel.
 * Its owner and availability are derived from the containing OS, plugin, app, or mode definition.
 * Handling a command may emit VCS-tracked operations exactly like an operation-kind action — see
 * `ArtifactApp::handle_command`/`ActionEmit`.
 */
export type CommandDefinition = { id: string,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown,
/**
 * 🗂️ Footer category tab this command groups under (an open id, e.g. "document", "appearance").
 */
category: string, iconId: IconName, kind: ActionKind,
/**
 * 📝️ Reuses `ActionArgDef` — one staged-form contract shared by actions, dialogs, and commands.
 */
args: Array<ActionArgDef>, keybindings: Array<PlatformKeybinding>, inPalette: boolean,
/**
 * 🎯️ See `ActionDefinition.semantics` — same D6/§3.1 field, same defaulting/inheritance story.
 */
semantics: ActionSemantics, };"####,
        },
        SchemaMetadata {
            name: "CommandInvocation",
            version: 1,
            typescript: r####"/**
 * @emoji 📨️ One addressed command invocation with named JSON arguments.
 */
export type CommandInvocation = { address: CommandAddress, arguments: Record<string, unknown>, };"####,
        },
        SchemaMetadata {
            name: "CommandOwnerAddress",
            version: 1,
            typescript: r####"/**
 * @emoji 📍️ Hierarchical owner of a command definition.
 */
export type CommandOwnerAddress = "os" | { "plugin": { pluginId: string, } } | { "app": { pluginId: string, appId: string, } } | { "mode": { pluginId: string, appId: string, modeId: string, } };"####,
        },
        SchemaMetadata {
            name: "ContributedInferenceMetadata",
            version: 1,
            typescript: r####"/**
 * 💡️ One inference a plugin contributes onto an artifact kind it depends on — mirrors the native
 * `ArtifactInferenceServiceMetadata` fields (owned strings instead of `&'static str`, since this
 * travels over the wire in a manifest), plus `contributor`/`depends_on` for the contribution's own
 * identity and ordering (contract freeze §4: `owner == contributor`, `artifact_kind == target`).
 */
export type ContributedInferenceMetadata = { owner: string, artifactKind: string, artifactSchema: string, artifactSchemaVersion: number, documentSchema: string, documentSchemaVersion: number, inferenceSchema: string, inferenceSchemaVersion: number, algorithmVersion: number, policyVersion: number, contributor: string, dependsOn: Array<string>, };"####,
        },
        SchemaMetadata {
            name: "ContributedMutationMetadata",
            version: 1,
            typescript: r####"/**
 * 🗂️ One mutation a plugin contributes onto an artifact kind it depends on — the manifest-declared
 * counterpart of a `contributor.list-artifact-mutations` roster entry (contract freeze §3/§6).
 */
export type ContributedMutationMetadata = {
/**
 * 🪪️ `"<target-document-schema>#<contributor-plugin-id>:<kebab-kind>"` (contract freeze §3).
 */
mutationId: string, semantics: ContributedMutationSemantics, schemaVersion: number, algorithmVersion: number, };"####,
        },
        SchemaMetadata {
            name: "ContributedMutationSemantics",
            version: 1,
            typescript: r####"/**
 * 🗂️ The `verb`/`entity`/`kind`/`record` semantic identity of one contributed mutation, carried as
 * owned strings on the wire (the native `SemanticDescriptor` this mirrors lives in the os-kernel
 * protocol crate, which `semio-framework` must not require plugin manifests to link against).
 */
export type ContributedMutationSemantics = { verb: string, entity: string, kind: string, record: string, };"####,
        },
        SchemaMetadata {
            name: "ContributionSet",
            version: 1,
            typescript: r####"/**
 * 🗂️ Everything a package contributes, gathered for static (`describe()`-time) emission —
 * `📓️design-abi.md` §3. `commands`/`topic_contributions`/`artifact_contributions` reuse this
 * crate's existing typed models; `panels` reuses `PanelTabDefinition` (already the typed shape
 * `AppDefinition.panel_tabs` declares, flattened across every app); `inference_services`/
 * `mutation_services` reuse `ContributedInferenceMetadata`/`ContributedMutationMetadata` (a
 * package's OWN registered services on artifact kinds it owns, as opposed to
 * `artifact_contributions`' services contributed onto a DEPENDENCY's kind — same wire shape
 * either way, `contributor == owner` and `depends_on` empty for a self-owned row);
 * `file_types`/`io_entries`/`composer_entries` are new types grounded in `AppIo`/`io::IoKey`/
 * `io::ComposerEntry` — see each type's own doc. `menus`/`themes` stay `DescriptorEntry` — see
 * its doc for why.
 */
export type ContributionSet = { commands: Array<CommandDefinition>, menus: Array<DescriptorEntry>, fileTypes: Array<FileTypeContribution>, panels: Array<PanelTabDefinition>, themes: Array<DescriptorEntry>, topicContributions: Array<TopicContribution>, artifactContributions: Array<ArtifactContributionDescriptor>, inferenceServices: Array<ContributedInferenceMetadata>, mutationServices: Array<ContributedMutationMetadata>, ioEntries: Array<IoEntryDescriptor>, composerEntries: Array<ComposerEntryDescriptor>, };"####,
        },
        SchemaMetadata {
            name: "DescriptorEntry",
            version: 1,
            typescript: r####"/**
 * 🗂️ One free-form descriptor-only contribution row, keyed by `id` with an opaque JSON
 * `payload` — the residual placeholder shape for the two `ContributionSet` categories (`menus`,
 * `themes`) that still have no real declared-contribution precedent anywhere in the codebase
 * (E1-describe surveyed every `[package.metadata.semio]` `contributes`/`consumes` tag and every
 * manifest-adjacent type: no plugin declares a menu or theme as its own manifest concept today —
 * context menus are derived at runtime from `ActionSemantics`/category metadata, and there is no
 * declared theme/palette contribution anywhere under `🖱️ui/🎨️styling`). Additive: nothing
 * constructs one yet, and a future typed model can replace either category without a wire break.
 */
export type DescriptorEntry = { id: string, payload?: unknown, };"####,
        },
        SchemaMetadata {
            name: "DialogDefinition",
            version: 1,
            typescript: r####"/**
 * @emoji 🗨️ A declared modal form dialog: a glass veil covers the screen and an info box (styled
 * identically to the introduction walkthrough box, see `ui_react`'s `GLASS_OVERLAY_BOX_CLASS`)
 * presents `args` as a staged form. Submit dispatches `submit_action` with the merged effective
 * args; empty `args` degenerates to a message/confirm dialog. Opened only via
 * `Effect::OpenDialog`; the shell owns open/close as ephemeral chrome state, never the document.
 */
export type DialogDefinition = { id: string,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
title: unknown, body?: unknown, args: Array<ActionArgDef>,
/**
 * 📇️ References an action owned by the active window kind, dispatched with merged args.
 */
submitAction: ActionRef, submitLabel: unknown,
/**
 * 📇️ Optional active-window action reference dispatched on any dismissal (Escape, veil
 * click, or the Cancel button).
 */
cancelAction?: ActionRef, cancelLabel?: unknown, };"####,
        },
        SchemaMetadata {
            name: "DomainSelection",
            version: 1,
            typescript: r####"/**
 * 🖱️ One domain's current selection: the active granularity, the selected ids, and the anchor id
 * range selection pivots from.
 */
export type DomainSelection = { granularity: string, ids: Array<string>, anchorId?: string, };"####,
        },
        SchemaMetadata {
            name: "ExampleDefinition",
            version: 1,
            typescript: r####"export type ExampleDefinition = { id: string,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown, iconId: IconName, artifactJson: string, appId: string, };"####,
        },
        SchemaMetadata {
            name: "ExecutionClass",
            version: 1,
            typescript: r####"/**
 * @emoji ⏱️ How long-running/interactive an invocation of this capability is — the gateway's job
 * vs. interactive-call dispatch hint.
 */
export type ExecutionClass = "interactive" | "background" | "job";"####,
        },
        SchemaMetadata {
            name: "ExecutionMode",
            version: 1,
            typescript: r####"/**
 * 🚦 How an extension actor runs relative to its host plugin — `📓️design-abi.md` §5. Default
 * `Isolated`: a same-process sandboxed actor, no publisher trust assumed. `Linked` additionally
 * requires the same publisher as the host plugin (enforced at link time, feature-gated to avoid
 * the `semio-framework-os-flow` ↔ extension-crate cycle); `Exclusive` gets a dedicated actor
 * (e.g. flow/brep tessellation); `Cold` runs as a bounded job, not a resident actor.
 */
export type ExecutionMode = "declarative" | "linked" | "isolated" | "exclusive" | "cold";"####,
        },
        SchemaMetadata {
            name: "ExtensionPointDeclaration",
            version: 1,
            typescript: r####"/**
 * 🧩️ One extension point a host plugin publishes — replaces the Cargo `consumes` tag
 * (`📓️design-abi.md` §5). `allowed_modes` gates `Linked` (same publisher required);
 * `capability_allowance`/`quota_ceiling` bound what any extension attaching here can ever hold,
 * regardless of what it requests — "a host can never delegate more than it holds".
 */
export type ExtensionPointDeclaration = { id: string, publisherScope: string, allowedModes: Array<ExecutionMode>, capabilityAllowance: Array<CapabilityId>, quotaCeiling: QuotaSchema, payloadSchema: string, activation: ActivationEvent, };"####,
        },
        SchemaMetadata {
            name: "GranularityDefinition",
            version: 1,
            typescript: r####"/**
 * 🔬️ One selectable/hoverable level of detail within a domain (e.g. mesh's object/face/edge/vertex).
 */
export type GranularityDefinition = { id: string,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown, iconId: IconName, };"####,
        },
        SchemaMetadata {
            name: "HierarchyProvider",
            version: 1,
            typescript: r####"/**
 * 🌳️ Where a domain's target ids come from, and thus what `DomainTopology` (if any) is available for
 * range selection and transitive hover/select closures.
 */
export type HierarchyProvider = { "kind": "flat" } | { "kind": "topology" } | { "kind": "uiTree" } | { "kind": "pathDelimited", delimiter: string, };"####,
        },
        SchemaMetadata {
            name: "HoverSpec",
            version: 1,
            typescript: r####"/**
 * 🐁️ One domain's hover behavior — see `semio_framework::InteractionDefinition::hover`.
 */
export type HoverSpec = { enabled: boolean,
/**
 * 🌳️ Whether hovering a target expands to its descendant closure (root first) — requires
 * `hierarchy != HierarchyProvider::Flat`.
 */
transitive: boolean,
/**
 * 📡️ Named hover channels this domain accepts (e.g. `["pointer"]`); the shared cursor throttle
 * keys off the same channel names.
 */
channels: Array<string>,
/**
 * 📣️ Whether this domain's own hover mirrors into `PresenceInteraction` for peers.
 */
broadcast: boolean, };"####,
        },
        SchemaMetadata {
            name: "IconName",
            version: 1,
            typescript: r####"export type IconName = "alert-circle" | "align-left" | "animate" | "app-window" | "architect" | "architect-graph" | "arrow-down" | "arrow-left" | "arrow-right" | "arrow-right-left" | "arrow-up" | "award" | "bar-chart-3" | "beam" | "bell" | "book-open" | "box" | "building" | "cad-shape" | "calendar" | "calendar-days" | "camera" | "check" | "check-circle-2" | "chevron-down" | "chevron-left" | "chevron-right" | "chevron-up" | "chevrons-up-down" | "circle" | "circle-dot" | "clipboard" | "clipboard-list" | "clock" | "cloud" | "code" | "columns" | "combine" | "component" | "copy" | "cpu" | "crosshair" | "cylinder" | "dag" | "display-windows" | "document-jack" | "document-report" | "download" | "draw" | "edit" | "edit-3" | "eraser" | "export" | "external-link" | "eye" | "eye-off" | "fem-app" | "fem-model" | "file" | "file-archive" | "file-code" | "file-image" | "file-json" | "file-spreadsheet" | "file-text" | "file-type" | "file-video" | "filter" | "flip-horizontal" | "flip-vertical" | "flow" | "flow-graph" | "focus" | "folder" | "folder-open" | "folder-plus" | "forms" | "gis2d" | "gis3d" | "git-branch" | "git-commit" | "git-merge" | "globe" | "graduation-cap" | "graph-dag" | "graph-media" | "grid-3x3" | "grip-vertical" | "hammer" | "hand" | "hard-drive" | "hash" | "help-circle" | "hexagon" | "home" | "hud-overlay" | "image" | "image-plus" | "image-up" | "imperative" | "import" | "info" | "landmark" | "lasso" | "layers" | "layout" | "layout-grid" | "library" | "lightbulb" | "link" | "link-2-off" | "list" | "list-checks" | "list-ordered" | "list-tree" | "loader-2" | "lock" | "lock-open" | "lod-depth" | "lowpoly-model" | "magnet" | "map" | "math-app" | "math-graph" | "maximize-2" | "message-circle" | "message-square" | "minimize-2" | "minus" | "monitor" | "moon" | "more-horizontal" | "mouse-pointer" | "mouse-pointer-2" | "move" | "move-3d" | "network" | "note" | "note-math" | "paint-bucket" | "paintbrush" | "palette" | "panel-catalogue" | "panel-inspection" | "panel-left" | "panel-parameters" | "panel-right" | "panel-top" | "pause" | "pen-tool" | "pencil" | "pipette" | "play" | "play-circle" | "plug" | "plus" | "preview" | "procedural2d" | "process-workpiece" | "projection-axonometric" | "projection-curvilinear" | "projection-dimetric" | "projection-isometric" | "projection-oblique" | "projection-oblique-cabinet" | "projection-oblique-cavalier" | "projection-oblique-military" | "projection-one-point" | "projection-orthographic" | "projection-parallel" | "projection-perspective" | "projection-three-point" | "projection-trimetric" | "projection-two-point" | "puzzle" | "puzzle5d-3d" | "raster" | "reasoning-wires" | "rectangle-tool" | "redo" | "redo-2" | "relocate-3d" | "remodel-app" | "remodel-model" | "rotate-ccw" | "rotate-cw" | "s" | "save" | "scaling" | "scan" | "scan-line" | "scene-3d" | "scissors" | "search" | "select-all" | "sequence" | "settings" | "settings-2" | "shapes" | "shooting-scene" | "sigma" | "skip-back" | "skip-forward" | "slab" | "sliders-horizontal" | "smartphone" | "smile" | "sparkles" | "square" | "square-arrow-down-left" | "square-arrow-down-right" | "square-arrow-up-left" | "square-arrow-up-right" | "square-dashed" | "sticky-note" | "sun" | "table-2" | "tablet" | "tags" | "terrain-3d" | "text-cursor" | "text-search" | "toggle-left" | "transform-3d" | "trash" | "trash-2" | "triangle" | "triangle-alert" | "trinity" | "trinity-lhs" | "trinity-rewrite" | "trinity-rhs" | "type" | "typography" | "undo" | "undo-2" | "unlink" | "unlock" | "user" | "users" | "volume-brush" | "window" | "workbench" | "workflow" | "wrench" | "writer" | "x" | "zoom-in" | "zoom-out";"####,
        },
        SchemaMetadata {
            name: "IdempotencyMode",
            version: 1,
            typescript: r####"/**
 * @emoji 🔁️ Whether replaying the same invocation twice is safe, and how the gateway makes it so.
 */
export type IdempotencyMode = "natural" | "key" | "none";"####,
        },
        SchemaMetadata {
            name: "InteractionDefinition",
            version: 1,
            typescript: r####"/**
 * 🕹️ One interaction domain an app declares (e.g. "graph", "mesh", "ast", "world"): the target
 * universe/hierarchy shared by both its hover and selection sub-specs. `AppDefinition.interactions`
 * holds these; `WindowKindDefinition.interactions` references them via `InteractionRef`.
 */
export type InteractionDefinition = { id: string,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown,
/**
 * 🪜️ Non-empty; the first entry is the domain's default granularity.
 */
granularities: Array<GranularityDefinition>, hierarchy: HierarchyProvider, hover: HoverSpec, selection: SelectionSpec, };"####,
        },
        SchemaMetadata {
            name: "InteractionRef",
            version: 1,
            typescript: r####"/**
 * 📇️ A validated reference into an app's `AppDefinition.interactions` registry — mirrors
 * `ActionRef`/`UtilityRef` exactly.
 */
export type InteractionRef = string;"####,
        },
        SchemaMetadata {
            name: "IntroductionCursor",
            version: 1,
            typescript: r####"/**
 * @emoji 🖱️ Ghost-cursor glyph, mirroring `🎨️ui.css`'s `--cursor-*` custom cursors.
 */
export type IntroductionCursor = "default" | "pointer" | "grab" | "grabbing" | "crosshair" | "move";"####,
        },
        SchemaMetadata {
            name: "IntroductionDefinition",
            version: 1,
            typescript: r####"/**
 * @emoji 🎓️ A first-run walkthrough an app declares to introduce its UI, utilities, and actions to a
 * first-time user. Rendered as an ordered sequence of `IntroductionStepDefinition`s over a full-screen
 * glass veil; the shell owns playback (start/advance/skip) as ephemeral chrome state, never the
 * document.
 */
export type IntroductionDefinition = {
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
title: unknown, steps: Array<IntroductionStepDefinition>, };"####,
        },
        SchemaMetadata {
            name: "IntroductionDemonstration",
            version: 1,
            typescript: r####"/**
 * @emoji 🎬️ A looping ghost-cursor demonstration attached to an interaction-gated
 * `IntroductionStepDefinition`. Plays only while the user's own pointer is idle — any real pointer
 * movement mutes it and restores the real cursor instantly; going idle again while the step is still
 * active replays it from the beginning. `cursor` overrides the glyph shown over the target; omitted, it
 * derives from `gesture` (clicks → pointer, drag → grab/grabbing).
 */
export type IntroductionDemonstration = { gesture: IntroductionGesture, cursor?: IntroductionCursor, };"####,
        },
        SchemaMetadata {
            name: "IntroductionGesture",
            version: 1,
            typescript: r####"/**
 * @emoji 👆️ A gesture a demonstration plays: the ghost cursor travels to (or between) `IntroductionPoint`s
 * and performs the visual press/release affordance for the gesture kind.
 */
export type IntroductionGesture = { "kind": "leftClick", at: IntroductionPoint, } | { "kind": "rightClick", at: IntroductionPoint, } | { "kind": "doubleClick", at: IntroductionPoint, } | { "kind": "drag", from: IntroductionPoint, to: IntroductionPoint, button?: IntroductionPointerButton, modifiers?: Array<IntroductionKeyModifier>, } | { "kind": "scroll", at: IntroductionPoint, deltaY: number, } | { "kind": "orbit", from: IntroductionPoint, to: IntroductionPoint, button?: IntroductionPointerButton, modifiers?: Array<IntroductionKeyModifier>, };"####,
        },
        SchemaMetadata {
            name: "IntroductionInteraction",
            version: 1,
            typescript: r####"/**
 * @emoji ✅️ One thing the user must do to complete an interaction-gated `IntroductionStepDefinition` —
 * rendered as a checklist row in the info box and celebrated individually on completion.
 */
export type IntroductionInteraction = { on: IntroductionInteractionKind,
/**
 * 🏷️ Short checklist label shown in the step's info box.
 */
label: string,
/**
 * 🎉️ Element id stamped `data-celebrated` on completion; `None` falls back to the step's `introduce`.
 */
celebrate?: string, };"####,
        },
        SchemaMetadata {
            name: "IntroductionInteractionKind",
            version: 1,
            typescript: r####"/**
 * @emoji 👉️ What one `IntroductionInteraction` requires: `Action`/`Utility`/`Tool`/`Panel`/`Expand`
 * complete as soon as the user activates that utility/tool, opens that panel tab, or expands that tree
 * section — teaching by doing. `Pan`/`Zoom`/`Orbit` complete on that camera-navigation gesture over the
 * 3D window named by the payload (a window-kind id) — classified from camera-state deltas by the shell
 * that renders the window, so only shells that render a 3D world (the React shell) can complete them.
 */
export type IntroductionInteractionKind = { "kind": "action", "id": ActionRef } | { "kind": "utility", "id": UtilityRef } | { "kind": "tool", "id": ToolRef } | { "kind": "panel", "id": string } | { "kind": "expand", "id": string } | { "kind": "pan", "id": string } | { "kind": "zoom", "id": string } | { "kind": "orbit", "id": string };"####,
        },
        SchemaMetadata {
            name: "IntroductionKeyModifier",
            version: 1,
            typescript: r####"/**
 * @emoji ⌨️ Keyboard modifier held during a drag-like demonstration.
 */
export type IntroductionKeyModifier = "alt" | "shift" | "control" | "meta";"####,
        },
        SchemaMetadata {
            name: "IntroductionLogo",
            version: 1,
            typescript: r####"/**
 * @emoji 🏛️ One institution/partner logo shown in an `IntroductionStepDefinition`'s info box — a plain
 * URL pair (no DOM/CSS types), optionally linking out when clicked.
 */
export type IntroductionLogo = { src: string, darkSrc: string | null, alt: string, href: string | null, };"####,
        },
        SchemaMetadata {
            name: "IntroductionPlacement",
            version: 1,
            typescript: r####"/**
 * @emoji 📍️ Where the info box is placed relative to its anchor.
 */
export type IntroductionPlacement = "auto" | "top" | "bottom" | "left" | "right" | "center";"####,
        },
        SchemaMetadata {
            name: "IntroductionPoint",
            version: 1,
            typescript: r####"/**
 * @emoji 📌️ Where a demonstration gesture points, resolvable to a viewport pixel at play time. One
 * point type covers click targets and drag endpoints across every addressing scheme the shell needs:
 * element-relative, absolute/normalized screen space, absolute/normalized window(pane)-local space, and
 * a 3D scene world position projected through that window's live camera.
 */
export type IntroductionPoint = { "kind": "element", id: string, offset?: [number, number], } | { "kind": "screen", x: number, y: number, } | { "kind": "screenNormalized", x: number, y: number, } | { "kind": "window", id: string, x: number, y: number, } | { "kind": "windowNormalized", id: string, x: number, y: number, } | { "kind": "scene", id: string, position: [number, number, number], } | { "kind": "canvas", id: string, x: number, y: number, } | { "kind": "entity", id: string, domain: string, entity: string, offset?: [number, number], } | { "kind": "curve", id: string, domain: string, entity: string, t: number, } | { "kind": "domain", id: string, domain: string, entity: string, value: number, };"####,
        },
        SchemaMetadata {
            name: "IntroductionPointerButton",
            version: 1,
            typescript: r####"/**
 * @emoji 🖱️ Which mouse button a drag-like demonstration presses.
 */
export type IntroductionPointerButton = "left" | "middle" | "right";"####,
        },
        SchemaMetadata {
            name: "IntroductionStepDefinition",
            version: 1,
            typescript: r####"/**
 * @emoji 🪜️ One step of an `IntroductionDefinition`: an info box pointing at `introduce`, with `show`
 * raising extra elements above the glass veil and `interactions` completing the step.
 */
export type IntroductionStepDefinition = { id: string,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
title: unknown, body: unknown,
/**
 * 🎯️ The single element id raised above the glass, pulsing `data-introduced`, that the info box
 * anchors to. `None` = a screen-style step: full veil, centered info box.
 */
introduce: string | null,
/**
 * 🕳️ Additional element ids raised above the glass — interactive, no pulse — e.g. every 3D window
 * that accepts a catalogue drop while `introduce` teaches the drag source.
 */
show: Array<string>, placement: IntroductionPlacement,
/**
 * ✅️ Interactions completing this step; empty means purely informational (Next-button-only).
 */
interactions: Array<IntroductionInteraction>,
/**
 * 🔢️ Whether `interactions` must complete in declaration order — out-of-order completions are
 * ignored. Unordered: the first incomplete matching interaction completes.
 */
ordered: boolean,
/**
 * 🏛️ Institution/partner logos shown in the info box below the body — e.g. funding acknowledgements.
 */
logos: Array<IntroductionLogo>,
/**
 * 🎬️ Ghost-cursor demonstrations played in order, one after another, then looping back to the first —
 * e.g. a viewport step showing zoom, then pan, then orbit. When the step also declares `interactions`,
 * `demonstrations[i]` previews `interactions[i]` and completed interactions are omitted from replay.
 * Empty means no demonstration.
 */
demonstrations?: Array<IntroductionDemonstration>, };"####,
        },
        SchemaMetadata { name: "Keybinding", version: 1, typescript: r####"export type Keybinding = { keys: string, action: ActionDescriptor, };"#### },
        SchemaMetadata { name: "MeasureSelectItem", version: 1, typescript: r####"export type MeasureSelectItem = { id: string, value: string, label: string, };"#### },
        SchemaMetadata {
            name: "MediaClass",
            version: 1,
            typescript: r####"/**
 * 🧬️ Typed-media lattice: every port/wire in the workflow carries a `MediaType` (`class` × `form`) instead of the legacy string `artifact_kind`. `MediaType` is what a wire negotiates; a format kind id string is only how bytes are encoded once they actually cross a process boundary (see `MediaWireFormat`). Dependent tickets retire `OsMediaCapability` (see the `ArtifactKind` region above) onto `MediaForm::{Brep,Mesh}`, which already covers what `OsMediaCapability::{Brep,MeshOnly}` expresses.
 */
export type MediaClass = "twoD" | "threeD" | "text" | "data" | "graph" | "kit" | "computation" | "presentation";"####,
        },
        SchemaMetadata {
            name: "MediaForm",
            version: 1,
            typescript: r####"/**
 * 🧬️ The shape/representation a `MediaClass` payload takes, orthogonal to `class` — e.g. `ThreeD` × `Brep` vs `ThreeD` × `Mesh`. `Any` only ever appears on the accepting side of a port (see `media_types_compatible`).
 */
export type MediaForm = "any" | "vector" | "raster" | "brep" | "mesh" | "document" | "value" | "dag" | "trinity" | "type" | "design" | "kit" | "flow" | "sequence" | "imperative" | "deck";"####,
        },
        SchemaMetadata {
            name: "MediaPortDirection",
            version: 1,
            typescript: r####"/**
 * 🔀️ Which side of a wire a `MediaPortSpec` sits on.
 */
export type MediaPortDirection = "in" | "out";"####,
        },
        SchemaMetadata {
            name: "MediaPortSpec",
            version: 1,
            typescript: r####"/**
 * 🔌️ A single port an app exposes on the workflow — `kind_id` optionally pins it to one `ArtifactKindSpec.id` when the port is more specific than its `media_type` alone conveys.
 */
export type MediaPortSpec = { id: string, label: string, direction: MediaPortDirection, mediaType: MediaType, kindId?: string, required: boolean, multiplicity: PortMultiplicity, };"####,
        },
        SchemaMetadata {
            name: "MediaType",
            version: 1,
            typescript: r####"/**
 * 🧬️ A port or wire's declared media type — the pair a producer offers or a consumer accepts.
 */
export type MediaType = { class: MediaClass, form: MediaForm, };"####,
        },
        SchemaMetadata {
            name: "MediaWireFormat",
            version: 1,
            typescript: r####"/**
 * 🔌️ How a `MediaType` is actually encoded once it crosses a process boundary — binary payloads
 * carry a format kind id string (the legacy format enum was retired — ticket 26/08/11/
 * SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6), structured payloads carry
 * a schema id instead (see `ArtifactKindSpec::schema`).
 */
export type MediaWireFormat = { "kind": "binary", format_kind: string, } | { "kind": "document", schema: string, };"####,
        },
        SchemaMetadata {
            name: "MergeMode",
            version: 1,
            typescript: r####"/**
 * 🧮️ Set algebra applied when merging new targets into the current selection — see `next_selection`.
 */
export type MergeMode = "replace" | "additive" | "subtractive" | "invertive" | "range";"####,
        },
        SchemaMetadata {
            name: "ModeDefinition",
            version: 1,
            typescript: r####"export type ModeDefinition = { id: string,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown, iconId: IconName,
/**
 * 🛠️ Tools available while this mode is active — references `AppDefinition.tools` ids.
 */
tools: Array<ToolRef>, layoutId?: string,
/**
 * 🎛️ Commands owned by this mode and active only while it is active.
 */
commands: Array<CommandDefinition>, };"####,
        },
        SchemaMetadata { name: "NamedLayout", version: 1, typescript: r####"export type NamedLayout = { id: string, label: string, iconId?: IconName, layout: WindowLayout, origin: string, groupPath?: Array<string>, };"#### },
        SchemaMetadata {
            name: "OsDefinition",
            version: 1,
            typescript: r####"/**
 * @emoji 💻️ Operating-system command catalog shared by every renderer.
 */
export type OsDefinition = { commands: Array<CommandDefinition>, };"####,
        },
        SchemaMetadata {
            name: "OsMediaCapability",
            version: 1,
            typescript: r####"/**
 * 🧬️ Which geometry backend a resource kind's media exporters/importers target — the manifest-level
 * counterpart threaded onto `AppDefinition.artifact_kinds` (see `ArtifactKindSpec`). Canonical home for
 * what used to be duplicated verbatim in `framework/plugin/rs` and `framework/product/os/core/rs`; both
 * now re-export this definition instead of declaring their own.
 */
export type OsMediaCapability = "meshOnly" | "brep";"####,
        },
        SchemaMetadata {
            name: "PackageDescriptor",
            version: 1,
            typescript: r####"/**
 * 📦️ The static, build-time-emitted description of a plugin or extension package —
 * `📓️design-abi.md` §3's `describe()` output (`🛂️.descriptor.semio`/`🔣️.json`).
 * Nothing constructs or reads one yet in this packet: additive contract only (packet
 * A2-abi-sdk's builder wiring and E1-describe's emitter/registry `check` gate consume it next).
 */
export type PackageDescriptor = { descriptorVersion: number, role: PackageRole, manifest: PluginManifest, activationEvents: Array<ActivationEvent>, capabilityRequests: Array<CapabilityRequest>, extensionPoints: Array<ExtensionPointDeclaration>, execution: ExecutionMode, quotas: QuotaSchema, contributions: ContributionSet, assets: Array<AssetDeclaration>, hashes: PackageHashes, };"####,
        },
        SchemaMetadata {
            name: "PackageHashes",
            version: 1,
            typescript: r####"/**
 * #️⃣ Content hashes the registry's `check` gate verifies against the built wasm —
 * `📓️design-abi.md` §3.
 */
export type PackageHashes = { wasmSha256: string, coreWasmSha256: string, descriptorSha256: string, };"####,
        },
        SchemaMetadata {
            name: "PackageRole",
            version: 1,
            typescript: r####"/**
 * 🎭️ Which actor-world role a package fills — `📓️design-abi.md` §3.
 */
export type PackageRole = "plugin" | "extension";"####,
        },
        SchemaMetadata { name: "PanelGroup", version: 1, typescript: r####"export type PanelGroup = "workbench" | "details" | "display" | "settings";"#### },
        SchemaMetadata {
            name: "PanelTabDefinition",
            version: 1,
            typescript: r####"/**
 * 🌳️ A leaf carries `body_key` (its rendered panel); a branch carries `children` (the tab row shown below it). Exactly one of the two is set; `group` is only meaningful on root (non-nested) entries.
 */
export type PanelTabDefinition = { kind: PanelTabKind,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown, group: PanelGroup, bodyKey?: string, children: Array<PanelTabDefinition>, };"####,
        },
        SchemaMetadata {
            name: "PanelTabKind",
            version: 1,
            typescript: r####"/**
 * 🌳️ Closes the informal `FRAMEWORK_CATEGORY_*`/`*_TAB_ID` string-constant convention that used to
 * live in the renderer: every panel tab is either a framework-predefined kind (compile-time
 * exhaustive) or an app-declared custom tab (open id, still required to be unique/non-empty,
 * validated at construction by `AppBuilder`).
 */
export type PanelTabKind = { "kind": "workbenchCategory" } | { "kind": "displayCategory" } | { "kind": "detailsCategory" } | { "kind": "settingsCategory" } | { "kind": "displayWindows" } | { "kind": "displayLayout" } | { "kind": "settingsGeneral" } | { "kind": "settingsTheme" } | { "kind": "settingsDefaultApps" } | { "kind": "app", "id": string };"####,
        },
        SchemaMetadata {
            name: "Platform",
            version: 1,
            typescript: r####"/**
 * @emoji ⌨️ Operating system selector for a platform-specific keybinding.
 */
export type Platform = "macOs" | "windows" | "linux";"####,
        },
        SchemaMetadata {
            name: "PlatformKeybinding",
            version: 1,
            typescript: r####"/**
 * @emoji ⌨️ One command chord, optionally restricted to a host platform.
 */
export type PlatformKeybinding = { chord: string, platform?: Platform, };"####,
        },
        SchemaMetadata {
            name: "PluginDependency",
            version: 1,
            typescript: r####"/**
 * 🔗️ One direct plugin dependency: the depended-on plugin id plus the version requirement it must
 * satisfy — see `resolve_load_order`/`validate_dependency_graph`.
 */
export type PluginDependency = { pluginId: string, version: string, };"####,
        },
        SchemaMetadata {
            name: "PluginManifest",
            version: 1,
            typescript: r####"export type PluginManifest = { pluginId: string, label: string, version: string, apps: Array<AppDefinition>, examples: Array<ExampleDefinition>, capabilities: Array<CapabilityRequirement>,
/**
 * 🗂️ Open plugin contributions — see `TopicContribution`.
 */
topicContributions: Array<TopicContribution>,
/**
 * 🎛️ Plugin-scope commands this program exposes — apply whenever any of its apps is focused.
 */
commands: Array<CommandDefinition>,
/**
 * 🗂️ Plugin-level artifact kinds (library plugins with zero apps declare kinds here).
 */
artifactKinds: Array<ArtifactKindSpec>,
/**
 * 🔗️ Direct plugin dependencies this plugin requires to load — see `PluginDependency`/
 * `resolve_load_order`.
 */
dependencies: Array<PluginDependency>,
/**
 * 🗂️ Artifact-kind contributions (mutations/inferences) this plugin contributes onto artifact
 * kinds it depends on — see `ArtifactContributionDescriptor`.
 */
contributions: Array<ArtifactContributionDescriptor>, };"####,
        },
        SchemaMetadata {
            name: "PortMultiplicity",
            version: 1,
            typescript: r####"/**
 * 🔢️ Whether a `MediaPortSpec` accepts/produces exactly one media value or a stream/collection of them — e.g. a mesh-array input that fans in from several upstream producers.
 */
export type PortMultiplicity = "one" | "many";"####,
        },
        SchemaMetadata {
            name: "PreviewMode",
            version: 1,
            typescript: r####"/**
 * @emoji 👁️ Whether/how the gateway can show the effect of an invocation before committing it.
 */
export type PreviewMode = "none" | "dryRun" | "diff";"####,
        },
        SchemaMetadata {
            name: "ProgramContributionEntry",
            version: 1,
            typescript: r####"/**
 * 🧩️ One host-aggregated plugin contribution entry (`contributionsJson` wire shape).
 */
export type ProgramContributionEntry = { pluginId: string, topicContribution: TopicContribution | null, };"####,
        },
        SchemaMetadata {
            name: "QuotaSchema",
            version: 1,
            typescript: r####"/**
 * 📏️ One scope's resource ceiling — `📓️design-abi.md` §5. Every field is `Option`: `None`
 * inherits from the next scope up in a `QuotaTree` (os → plugin → extension → instance,
 * min-down). A plugin can sit inside its `memory_bytes` limit and still exhaust the host through
 * timers/UI nodes/requests/GPU allocations, which is why the schema is this wide.
 */
export type QuotaSchema = { memoryBytes?: bigint, fuelPerTurn?: bigint, turnDeadlineMs?: bigint, tables?: bigint, mailboxLen?: bigint, messageBytes?: bigint, outstandingRequests?: bigint, timers?: bigint, storageBytes?: bigint, networkBytesPerMin?: bigint, uiNodes?: bigint, patchBytes?: bigint, patchHz?: bigint, blobResidentBytes?: bigint, gpuMsPerFrame?: bigint, backgroundMsPerMin?: bigint, logBytesPerMin?: bigint, };"####,
        },
        SchemaMetadata {
            name: "ResourceSelector",
            version: 1,
            typescript: r####"/**
 * @emoji 🎯️ A templated resource-selector string identifying what a capability reads/writes —
 * documented vocabulary (`"artifact:{self}"`, `"artifact:{arg.<id>}"`, `"config:{self}"`,
 * `"ui:window"`, `"clipboard"`, `"fs:{arg.<id>}"`, `"net:{origin}"`), not a closed enum: a new
 * resource family never needs a manifest schema change.
 */
export type ResourceSelector = string;"####,
        },
        SchemaMetadata { name: "Rights", version: 1, typescript: r####"export type Rights = "read" | "write" | "invoke" | "open";"#### },
        SchemaMetadata { name: "Scope", version: 1, typescript: r####"export type Scope = "instance" | "app" | "plugin" | "global";"#### },
        SchemaMetadata {
            name: "SelectionMethod",
            version: 1,
            typescript: r####"/**
 * 🎯️ How a surface gathers targets for one `interactionSelect` dispatch.
 */
export type SelectionMethod = "pick" | "rectangle" | "lasso";"####,
        },
        SchemaMetadata {
            name: "SelectionMode",
            version: 1,
            typescript: r####"/**
 * 🔢️ How many targets may be selected at once within a domain.
 */
export type SelectionMode = "single" | "multiple";"####,
        },
        SchemaMetadata {
            name: "SelectionSpec",
            version: 1,
            typescript: r####"/**
 * 🖱️ One domain's selection behavior — see `semio_framework::InteractionDefinition::selection`.
 */
export type SelectionSpec = {
/**
 * 🪜️ Non-empty; the first entry is the domain's default mode.
 */
modes: Array<SelectionMode>, methods: Array<SelectionMethod>, merges: Array<MergeMode>,
/**
 * 🌳️ Whether selecting a target expands to its descendant closure — requires
 * `hierarchy != HierarchyProvider::Flat`.
 */
transitive: boolean,
/**
 * 📣️ Whether this domain's own selection mirrors into `PresenceInteraction` for peers.
 */
broadcast: boolean, };"####,
        },
        SchemaMetadata {
            name: "SurfaceKind",
            version: 1,
            typescript: r####"export type SurfaceKind = "canvas-2d" | "world-3d" | "node-graph" | "text-editor" | "table" | "paint-2d" | "virtualFileSystem" | "tiled-map" | "board-2d" | "icon-render" | "ink-canvas" | "graph-timeline" | "block-list" | "diff-view" | "event-feed";"####,
        },
        SchemaMetadata {
            name: "ToolDefinition",
            version: 1,
            typescript: r####"/**
 * @emoji 🛠️ Declares one mode-level tool: an activatable, stateful capability of a whole app mode.
 * Distinct from `UtilityDefinition` (a per-window pointer mode — a utility is a tool for a specific
 * window) and `CommandDefinition` (a fire-once verb): exactly one tool is active per app at a time,
 * and activation is host-owned session view state (`ViewModel.active_tool_id`), never a document
 * field or VCS operation. A tool's live options are supplied dynamically via `ArtifactApp::tool_measures`,
 * keyed by tool id — not part of this static declaration.
 */
export type ToolDefinition = { id: string,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown, iconId: IconName, keys?: string, };"####,
        },
        SchemaMetadata {
            name: "ToolRef",
            version: 1,
            typescript: r####"/**
 * @emoji 🛠️ A validated reference into an app's `AppDefinition.tools` registry — the tool mirror of
 * `UtilityRef`, scoping tools to modes with a typed, resolvable id.
 */
export type ToolRef = string;"####,
        },
        SchemaMetadata {
            name: "TutorialArtifactEvent",
            version: 1,
            typescript: r####"/**
 * @emoji 🖋️ One document-track entry — mirrors `store::ArtifactCommand` with `Mutation =
 * serde_json::Value` (opaque per-app mutation JSON, already the wire shape of every `KernelMutation`
 * diff). This is the SOLE source of document mutation during playback: recorded `TutorialEvent`s are
 * annotational only, never re-dispatched, because re-dispatching a plugin action is non-deterministic
 * (fresh ids/timestamps) and would double-apply against this track.
 */
export type TutorialArtifactEvent = { at: bigint, kind: TutorialArtifactEventKind, };"####,
        },
        SchemaMetadata {
            name: "TutorialArtifactEventKind",
            version: 1,
            typescript: r####"/**
 * @emoji 🖋️ See `TutorialArtifactEvent`. `Edit` carries both `forwards` and `backwards` operations
 * verbatim from the vcs edit that produced it — the source of exact bidirectional scrubbing.
 */
export type TutorialArtifactEventKind = { "kind": "edit", forwards: unknown[], backwards: unknown[], description?: string, coalesceKey?: string, } | { "kind": "undo" } | { "kind": "redo" } | { "kind": "checkpoint", message?: string, } | { "kind": "checkoutCheckpoint", checkpointId: string, } | { "kind": "switchAlternative", alternativeId: string, } | { "kind": "load", documentDsl: string, previousDsl: string, };"####,
        },
        SchemaMetadata {
            name: "TutorialAssetSrc",
            version: 1,
            typescript: r####"/**
 * @emoji 📦️ Where a tutorial media asset's bytes live. `Blob` is wire-identical to `store::BlobRef`
 * (content-addressed Blake3 hash + size + media type) — `framework/core` does not depend on
 * `semio-vcs`, so the shape is mirrored rather than reused; conversion between the two is
 * field-for-field.
 */
export type TutorialAssetSrc = { "kind": "url", url: string, } | { "kind": "blob", hash: string, size: bigint, mediaType: string, } | { "kind": "dataUrl", data: string, };"####,
        },
        SchemaMetadata {
            name: "TutorialBase",
            version: 1,
            typescript: r####"/**
 * @emoji 🎬️ What must be true at t=0: the document the tutorial sandboxes and the initial UI/camera
 * state. The player snapshots the user's live document, loads this in its place, and restores the
 * snapshot on exit — a tutorial can never touch real work.
 */
export type TutorialBase = {
/**
 * 📂️ Full document DSL text (`ArtifactTextFiles.dsl`) to sandbox-load; `None` falls back to `example_id`, and both
 * `None` falls back to the app's default/empty document.
 */
documentDsl?: string, exampleId?: string, ui: TutorialUiSnapshot,
/**
 * 🎥️ Initial camera per window instance (every entry's `at` is `0`).
 */
cameras: Array<TutorialCameraKeyframe>, };"####,
        },
        SchemaMetadata {
            name: "TutorialCameraKeyframe",
            version: 1,
            typescript: r####"/**
 * @emoji 🎥️ One camera track keyframe for a specific window instance.
 */
export type TutorialCameraKeyframe = { at: bigint,
/**
 * 🪟️ Window *instance* id (matches `ViewWindowInstance.id`).
 */
windowId: string, camera: TutorialCameraState,
/**
 * 🪄️ Easing INTO this keyframe from the previous one on the same window.
 */
easing: TutorialEasing, };"####,
        },
        SchemaMetadata {
            name: "TutorialCameraState",
            version: 1,
            typescript: r####"/**
 * @emoji 🎥️ A camera pose — `Orbit` mirrors `World3dScene.camera_json`/`OrbitController`, `Canvas`
 * mirrors `Canvas2dScene`'s `cameraX`/`cameraY`/`zoom`.
 */
export type TutorialCameraState = { "kind": "orbit", position: [number, number, number], target: [number, number, number], up: [number, number, number], fov?: number, } | { "kind": "canvas", x: number, y: number, zoom: number, };"####,
        },
        SchemaMetadata {
            name: "TutorialCaption",
            version: 1,
            typescript: r####"/**
 * @emoji 💬️ One timed caption sub-segment of a `TutorialNarrationCue`.
 */
export type TutorialCaption = { at: bigint, durationMs: bigint,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
text: unknown, };"####,
        },
        SchemaMetadata {
            name: "TutorialChapter",
            version: 1,
            typescript: r####"/**
 * @emoji 📖️ One scrub-bar marker in a `TutorialDefinition`'s timeline.
 */
export type TutorialChapter = { id: string, at: bigint,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
title: unknown, body?: unknown, };"####,
        },
        SchemaMetadata {
            name: "TutorialDefinition",
            version: 1,
            typescript: r####"/**
 * @emoji 🎬️ A recorded, timed, replayable walkthrough — the timeline sibling of the step-gated
 * `IntroductionDefinition`. Where an introduction gates progression on the user performing an
 * interaction, a tutorial plays a multi-track recording (narration, video overlay, UI state, document
 * edits, camera, ghost-cursor gestures) against a sandboxed copy of the document while the user watches,
 * scrubs, or deviates and converges back. A *recording* IS a `TutorialDefinition` — the recorder simply
 * produces a densely-sampled one; nothing distinguishes a hand-authored tutorial from a captured one.
 * Distinct from the docs-tooltip `tutorial` link field in `ui/js/react`'s `UiLabelLeaf` (a URL into the
 * manual) — this is the interactive playback mechanism.
 */
export type TutorialDefinition = { id: string,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
title: unknown, description?: unknown,
/**
 * ⏱️ Total timeline length in milliseconds; every track entry's `at` (+ duration) must fit within.
 */
durationMs: bigint,
/**
 * 📖️ Scrub-bar markers, sorted ascending by `at`.
 */
chapters: Array<TutorialChapter>,
/**
 * 🎬️ Starting conditions the player restores into its sandbox before t=0.
 */
base: TutorialBase, tracks: TutorialTracks,
/**
 * 🧾️ Recorder provenance (ISO 8601 timestamp); `None` means hand-authored.
 */
recordedAt?: string, };"####,
        },
        SchemaMetadata {
            name: "TutorialEasing",
            version: 1,
            typescript: r####"/**
 * @emoji 🪄️ Interpolation curve into a `TutorialCameraKeyframe` from its predecessor on the same window.
 */
export type TutorialEasing = "linear" | "easeInOut" | "hold";"####,
        },
        SchemaMetadata {
            name: "TutorialEvent",
            version: 1,
            typescript: r####"/**
 * @emoji 🏷️ One recorded action/command/keypress, annotational only — see `TutorialTracks::events`.
 */
export type TutorialEvent = { at: bigint, kind: TutorialEventKind, };"####,
        },
        SchemaMetadata {
            name: "TutorialEventKind",
            version: 1,
            typescript: r####"/**
 * @emoji 🏷️ What one `TutorialEvent` annotates.
 */
export type TutorialEventKind = { "kind": "action", action: string, args?: unknown, } | { "kind": "command", command: string, args?: unknown, } | { "kind": "key", keys: string, };"####,
        },
        SchemaMetadata {
            name: "TutorialGestureCue",
            version: 1,
            typescript: r####"/**
 * @emoji 👻️ One ghost-cursor gesture cue, reusing the introduction demonstration vocabulary verbatim —
 * both shells already resolve/render `IntroductionGesture`/`IntroductionPoint`/`IntroductionCursor`.
 */
export type TutorialGestureCue = { at: bigint, durationMs: bigint, gesture: IntroductionGesture, cursor?: IntroductionCursor, };"####,
        },
        SchemaMetadata {
            name: "TutorialNarrationCue",
            version: 1,
            typescript: r####"/**
 * @emoji 🎙️ One voiceover cue: `text` is both the TTS script and the caption fallback; `audio`
 * overrides TTS with a recorded take. The timeline is always the master clock — a still-speaking TTS
 * utterance is cancelled at the next cue's `at`; audio assets are seeked and rate-matched to the
 * playhead instead of played independently.
 */
export type TutorialNarrationCue = { id: string, at: bigint,
/**
 * ⏱️ Audio duration when `audio` is set (recorder-measured); a rough TTS estimate otherwise — used
 * for scrub-bar layout only, never to gate playback.
 */
durationMs: bigint,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
text: unknown, audio?: TutorialAssetSrc,
/**
 * 🗣️ Web Speech API voice-name hint; ignored once `audio` is set.
 */
voice?: string,
/**
 * 🎚️ TTS/audio rate multiplier layered under the player's own playback-rate control.
 */
rate: number,
/**
 * 💬️ Timed caption sub-segments (offsets relative to this cue's `at`); empty means `text` is shown
 * whole for the cue's `duration_ms`.
 */
captions: Array<TutorialCaption>, };"####,
        },
        SchemaMetadata {
            name: "TutorialOverlayRect",
            version: 1,
            typescript: r####"/**
 * @emoji 🖼️ Normalized 0–1 viewport rect for a `TutorialVideoCue` overlay.
 */
export type TutorialOverlayRect = { x: number, y: number, width: number, height: number, };"####,
        },
        SchemaMetadata {
            name: "TutorialTracks",
            version: 1,
            typescript: r####"/**
 * @emoji 🎞️ The seven parallel tracks of a `TutorialDefinition`'s timeline; every entry's `at` is a
 * millisecond offset from tutorial start, and each `Vec` is sorted ascending by `at`
 * (`validate_tutorial` enforces this).
 */
export type TutorialTracks = { narration: Array<TutorialNarrationCue>, video: Array<TutorialVideoCue>,
/**
 * 🏷️ Annotational only — drives affordance pulses and scrub-bar tick marks; playback never
 * re-dispatches these into a plugin (see `TutorialEventKind`).
 */
events: Array<TutorialEvent>, ui: Array<TutorialUiKeyframe>,
/**
 * 🖋️ The sole source of document mutation during playback — see `TutorialArtifactEventKind`.
 */
document: Array<TutorialArtifactEvent>, camera: Array<TutorialCameraKeyframe>, gestures: Array<TutorialGestureCue>, };"####,
        },
        SchemaMetadata {
            name: "TutorialUiChange",
            version: 1,
            typescript: r####"/**
 * @emoji 🩹️ One typed, sparse UI-state change — the alphabet `compose_tutorial_ui` replays over a prior
 * `TutorialUiSnapshot` to reconstruct state at any timeline offset without shipping a full snapshot at
 * every sample.
 */
export type TutorialUiChange = { "kind": "activeMode", id: string, } | { "kind": "focusedWindow", id?: string, } | { "kind": "activeUtility", windowId: string, utilityId?: string, } | { "kind": "activeTool", id?: string, } | { "kind": "layout", layout: WindowLayout, } | { "kind": "panelTab", group: string, tabId?: string, } | { "kind": "panelState", panelJson: string, } | { "kind": "selection", domainId: string, granularity: string, ids: Array<string>, } | { "kind": "dialog", id?: string, args?: unknown, } | { "kind": "treeExpansion", id: string, expanded: boolean, } | { "kind": "commandPanel", open: boolean, };"####,
        },
        SchemaMetadata {
            name: "TutorialUiKeyframe",
            version: 1,
            typescript: r####"/**
 * @emoji 🧮️ One UI-state track entry: either a full restore-point snapshot (a valid seek anchor) or a
 * sparse list of changes since the previous sample.
 */
export type TutorialUiKeyframe = { at: bigint, sample: TutorialUiSample, };"####,
        },
        SchemaMetadata {
            name: "TutorialUiSample",
            version: 1,
            typescript: r####"/**
 * @emoji 🧮️ See `TutorialUiKeyframe`.
 */
export type TutorialUiSample = { "kind": "snapshot", state: TutorialUiSnapshot, } | { "kind": "delta", changes: Array<TutorialUiChange>, };"####,
        },
        SchemaMetadata {
            name: "TutorialUiSnapshot",
            version: 1,
            typescript: r####"/**
 * @emoji 🧮️ Renderer-neutral restore point for chrome/UI state — a superset of `ViewModel` plus the
 * dock/panel/dialog state neither shell serializes today. Deliberately NOT a serialization of either
 * shell's internal store: each shell implements its own `captureUiSnapshot`/`applyUiSnapshot` against
 * this shape. Locale/terminology are excluded on purpose — a tutorial plays in the viewer's own locale.
 */
export type TutorialUiSnapshot = { activeModeId?: string, focusedWindowId?: string,
/**
 * 🧰️ Mirrors `ViewModel.active_utility_by_window_id`.
 */
activeUtilityByWindowId: { [key in string]?: string }, activeToolId?: string, layout?: WindowLayout,
/**
 * 📑️ Active tab id per panel group; groups absent from the map are collapsed/closed.
 */
activePanelTabByGroup: { [key in string]?: string },
/**
 * 🗂️ Opaque program vocabulary, verbatim `ViewModel.panel_json`.
 */
panelJson?: string,
/**
 * 🕹️ Per-domain selection state, keyed by `InteractionDefinition.id` — the framework-owned
 * replacement for the deleted opaque `selection_json`; see `TutorialUiChange::Selection`.
 */
interactionSelection: { [key in string]?: DomainSelection }, openDialogId?: string, expandedTreeIds: Array<string>, commandPanelOpen: boolean, };"####,
        },
        SchemaMetadata {
            name: "TutorialVideoCue",
            version: 1,
            typescript: r####"/**
 * @emoji 📹️ A timed video overlay — e.g. a presenter webcam picture-in-picture, or an authored clip.
 */
export type TutorialVideoCue = { at: bigint, durationMs: bigint, src: TutorialAssetSrc, rect: TutorialOverlayRect,
/**
 * 🔇️ True when narration carries the audio (a webcam take recorded muted).
 */
muted: boolean,
/**
 * ⏩️ Seek offset into the source at cue start.
 */
sourceOffsetMs: bigint, };"####,
        },
        SchemaMetadata { name: "UiButtonNode", version: 1, typescript: r####"export type UiButtonNode = { id?: string, iconId: IconName, label: Label, action: ActionDescriptor, style?: StyleSpec, presence?: UiPresence, menu?: UiMenuRef, };"#### },
        SchemaMetadata {
            name: "UiControlNode",
            version: 1,
            typescript: r####"export type UiControlNode = { "type": "input" } & UiInputNode | { "type": "select" } & UiSelectNode | { "type": "toggle" } & UiToggleNode | { "type": "button" } & UiButtonNode | { "type": "keyValue" } & UiKeyValueNode | { "type": "slider" } & UiSliderNode | { "type": "numberStepper" } & UiNumberStepperNode | { "type": "ring" } & UiRingNode | { "type": "iconSelect" } & UiIconSelectNode;"####,
        },
        SchemaMetadata {
            name: "UiDropOverlaySpec",
            version: 1,
            typescript: r####"/**
 * 📥️ Hover-state copy for a `UiStackNode`'s `drop_overlay`: shown while a drag is over the stack, ahead of `drop_action` firing on release.
 */
export type UiDropOverlaySpec = { title: Label, hint: Label, accept?: string, };"####,
        },
        SchemaMetadata { name: "UiExternalSlotNode", version: 1, typescript: r####"export type UiExternalSlotNode = { pluginId: string, appId: string, bodyKey: string, paramsJson: string, presence?: UiPresence, menu?: UiMenuRef, };"#### },
        SchemaMetadata {
            name: "UiIconSelectNode",
            version: 1,
            typescript: r####"export type UiIconSelectNode = { id: string, value: string, uniform: boolean, classifierKind: string, onChange: ActionDescriptor, presence?: UiPresence, menu?: UiMenuRef, };"####,
        },
        SchemaMetadata { name: "UiImageNode", version: 1, typescript: r####"export type UiImageNode = { id: string, src: string, alt?: Label, presence?: UiPresence, menu?: UiMenuRef, };"#### },
        SchemaMetadata {
            name: "UiInputNode",
            version: 1,
            typescript: r####"export type UiInputNode = { id: string, inputKind: string, value: string, placeholder?: Label, commit?: string, min?: number, max?: number, step?: number, accept?: string, onChange: ActionDescriptor, presence?: UiPresence, menu?: UiMenuRef, };"####,
        },
        SchemaMetadata { name: "UiKeyValueEntry", version: 1, typescript: r####"export type UiKeyValueEntry = { label: Label, value: string, };"#### },
        SchemaMetadata { name: "UiKeyValueNode", version: 1, typescript: r####"export type UiKeyValueNode = { entries: Array<UiKeyValueEntry>, presence?: UiPresence, menu?: UiMenuRef, };"#### },
        SchemaMetadata {
            name: "UiNumberStepperNode",
            version: 1,
            typescript: r####"export type UiNumberStepperNode = { id: string, value: number, step: number, uniform: boolean, onAbsolute: ActionDescriptor, onDelta: ActionDescriptor, presence?: UiPresence, menu?: UiMenuRef, };"####,
        },
        SchemaMetadata {
            name: "UiPeerMark",
            version: 1,
            typescript: r####"/**
 * 🧭️ The shared, compile-time-enforced state model every rendered UI element embeds as a
 * mandatory `presence` field: `state` × `status` × `hover` × `selected`. All combinations are
 * visually distinguishable except `state == Hidden`, which makes the rest irrelevant — see
 * [`UiPresence::visible`]. Defaults to fully inert (`Normal`/`Idle`/`false`/`false`) and is omitted
 * from the wire format entirely at default (see `UiPresence::is_default`).
 * 👥️ One peer's mark on the element carrying this `UiPresence` — hover/selection dot plus
 * initials chip (contract-freeze §C7.6 of ticket `.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/
 * SHARED-PRESENCE-SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION`). `label` is the actor id
 * itself (no display name is carried this far down the stack — see `PeerPresence`'s own doc
 * comment in the plugin crate); a renderer that has the full roster may substitute a friendlier
 * name, but every renderer must always carry SOME text alongside color (never color alone).
 */
export type UiPeerMark = { actor: string, color: number | null, hovered: boolean, selected: boolean, label: string, };"####,
        },
        SchemaMetadata {
            name: "UiPresence",
            version: 1,
            typescript: r####"/**
 * 🧭️ The shared, compile-time-enforced state model every rendered UI element embeds as a
 * mandatory `presence` field: `state` × `status` × `hover` × `selected` × own `color` × peer
 * `marks`. All combinations are visually distinguishable except `state == Hidden`, which makes
 * the rest irrelevant — see [`UiPresence::visible`]. Defaults to fully inert
 * (`Normal`/`Idle`/`false`/`false`/`None`/`[]`) and is omitted from the wire format entirely at
 * default (see `UiPresence::is_default`). `color`/`peers` (ticket 26/08/17/SHARED-PRESENCE-
 * SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION C7.6) make `UiPresence` `Clone`-only — no
 * longer `Copy` — since `peers: Vec<UiPeerMark>` owns heap data; `UiNode::presence()`/
 * `UiControlNode::presence()` therefore return `&UiPresence`, not a by-value copy.
 */
export type UiPresence = { state: UiState, status: UiStatus, hover: boolean, selected: boolean,
/**
 * 🎨️ This session's own hub-assigned palette index — stamped onto every `interaction_domain`-
 * bound tree item by `ui_tree_stamp_presence`, `None` for a folder-only session with no hub.
 */
color: number | null,
/**
 * 👥️ Every OTHER peer currently marking this element (hover and/or selection), sorted by
 * actor.
 */
peers: Array<UiPeerMark>, };"####,
        },
        SchemaMetadata { name: "UiRingNode", version: 1, typescript: r####"export type UiRingNode = { id: string, orbId: string, t: number, onChange: ActionDescriptor, presence?: UiPresence, menu?: UiMenuRef, };"#### },
        SchemaMetadata { name: "UiSelectItem", version: 1, typescript: r####"export type UiSelectItem = { value: string, label: Label, };"#### },
        SchemaMetadata {
            name: "UiSelectNode",
            version: 1,
            typescript: r####"export type UiSelectNode = { id: string, value: string, items: Array<UiSelectItem>, placeholder?: Label, onChange: ActionDescriptor, presence?: UiPresence, menu?: UiMenuRef, };"####,
        },
        SchemaMetadata { name: "UiSeparatorNode", version: 1, typescript: r####"export type UiSeparatorNode = { presence?: UiPresence, menu?: UiMenuRef, };"#### },
        SchemaMetadata {
            name: "UiSliderNode",
            version: 1,
            typescript: r####"export type UiSliderNode = { id: string, value: number, min: number, max: number, step: number, unit?: string, onChange: ActionDescriptor, presence?: UiPresence, menu?: UiMenuRef, };"####,
        },
        SchemaMetadata {
            name: "UiState",
            version: 1,
            typescript: r####"/**
 * 🧭️ The one shared, mandatory visual state every rendered UI element carries — orthogonal to
 * `status` and to the `hover`/`selected` flags. `Hidden` short-circuits everything else: a hidden
 * element is not rendered at all (no layout, no paint, no events) — renderers/reconcile must check
 * this before doing anything with the rest of an element's `UiPresence`.
 */
export type UiState = "introducing" | "celebrating" | "previewed" | "normal" | "disabled" | "hidden";"####,
        },
        SchemaMetadata {
            name: "UiStatus",
            version: 1,
            typescript: r####"/**
 * 🧭️ The activity lifecycle of a UI element, orthogonal to [`UiState`] and composable with it.
 */
export type UiStatus = "waiting" | "loading" | "idle" | "finished";"####,
        },
        SchemaMetadata { name: "UiTextNode", version: 1, typescript: r####"export type UiTextNode = { value: Label, emphasize?: boolean, dataAttributes?: { [key in string]?: string }, presence?: UiPresence, menu?: UiMenuRef, };"#### },
        SchemaMetadata { name: "UiToggleNode", version: 1, typescript: r####"export type UiToggleNode = { id: string, iconId: IconName, text?: Label, onChange: ActionDescriptor, presence?: UiPresence, menu?: UiMenuRef, };"#### },
        SchemaMetadata { name: "UiTreeItemAction", version: 1, typescript: r####"export type UiTreeItemAction = { iconId: IconName, label?: Label, action: ActionDescriptor, placement?: UiTreeActionPlacement, };"#### },
        SchemaMetadata {
            name: "UiTreeItemNode",
            version: 1,
            typescript: r####"export type UiTreeItemNode = { id: string, label: Label, description?: string, iconId?: IconName, presence?: UiPresence, defaultOpen?: boolean, action?: ActionDescriptor, actions?: Array<UiTreeItemAction>, draggable?: boolean, dragData?: { [key in string]?: string }, items?: Array<UiTreeItemNode>, control?: UiControlNode,
/**
 * 👁️ Domain "eye toggle" flag: the row stays visible, dimmed, and clickable (to un-hide) —
 * this is NOT `presence.state == Hidden`, which means not rendered at all.
 */
dimmed?: boolean,
/**
 * 🖱️ Row-level context-menu address — most rows share one `menu.id` across a tree with the row
 * id carried in `args` (e.g. `{"id": row.id}`), rather than minting a unique menu id per row.
 */
menu?: UiMenuRef, };"####,
        },
        SchemaMetadata {
            name: "UiTreeNode",
            version: 1,
            typescript: r####"export type UiTreeNode = { sections: Array<UiTreeSectionNode>, presence?: UiPresence, dropAction?: ActionDescriptor, menu?: UiMenuRef,
/**
 * 🕹️ Binds this rendered tree to an app-declared `InteractionDefinition` domain — the framework
 * (not the app) then owns the domain's selection/hover via `interactionSelect`/`interactionHover`,
 * stamped back onto item `presence` by `ui_tree_stamp_presence`. Replaces the deleted per-app
 * `selected_ids`/`highlighted_ids`/`selection_change` wire surface.
 */
interactionDomain?: string, };"####,
        },
        SchemaMetadata { name: "UiTreeSectionNode", version: 1, typescript: r####"export type UiTreeSectionNode = { id: string, label?: Label, defaultOpen?: boolean, presence?: UiPresence, items: Array<UiTreeItemNode>, };"#### },
        SchemaMetadata {
            name: "UndoMode",
            version: 1,
            typescript: r####"/**
 * @emoji ↩️ How a committed invocation of this capability can be undone.
 */
export type UndoMode = { "kind": "none" } | { "kind": "inverse" } | { "kind": "compensate", capability: string, };"####,
        },
        SchemaMetadata { name: "UtilityCategory", version: 1, typescript: r####"export type UtilityCategory = "selection" | "utilities" | "history" | "sync";"#### },
        SchemaMetadata {
            name: "UtilityDefinition",
            version: 1,
            typescript: r####"/**
 * @emoji 🧰️ Declares one interactive utility (a live-preview pointer mode) an app exposes. Distinct from
 * an `ActionDefinition`: exactly one utility is active per window kind at a time, and activation is
 * host-owned session view state (`ViewModel.active_utility_id`), never a document field or VCS operation.
 */
export type UtilityDefinition = { id: string,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown, iconId: IconName,
/**
 * 🧺️ Visual ribbon collection this utility groups into; `None` = a flat top-level ribbon entry.
 */
group?: string, keys?: string,
/**
 * 🖱️ CSS/winit cursor name applied to the window body while this utility is active.
 */
cursor?: string, category?: UtilityCategory,
/**
 * 🚦️ Whether window-scoped actions stay enabled while this utility is active. Defaults to `false`
 * (matching today's whitelist-based gating where an active utility suppresses the action panel);
 * set `true` for passive view utilities (e.g. cad `cad.play.view.*`) that should not gate actions.
 */
allowsActionsWhileActive: boolean, };"####,
        },
        SchemaMetadata {
            name: "UtilityRef",
            version: 1,
            typescript: r####"/**
 * @emoji 🧰️ A validated reference into an app's `AppDefinition.utilities` registry — the utility mirror of
 * `ActionRef`, scoping utilities to window kinds/modes with a typed, resolvable id.
 */
export type UtilityRef = string;"####,
        },
        SchemaMetadata {
            name: "ViewModel",
            version: 1,
            typescript: r####"export type ViewModel = { activeModeId?: string, activeWindowKindId?: string,
/**
 * 🧰️ Per-call overlay: the host-owned active utility for the window targeted by this `render`/`handle_action`
 * call (`window_id`). On batched `refresh-ui`, the plugin stamps this from
 * `active_utility_by_window_id` per window entry — never from the focused window alone.
 */
activeUtilityId?: string,
/**
 * 🧰️ Host-owned active utility per window **instance** (never a document field, never a VCS operation). The shell
 * sends the full map on every refresh so plugins can build per-pane scene state; tools stay mode-wide via
 * `active_tool_id`.
 */
activeUtilityByWindowId: { [key in string]?: string },
/**
 * 🛠️ The host-owned active tool of the active mode (never a document field, never a VCS operation) —
 * mutually exclusive with `active_utility_id`: activating one clears the other (see the React
 * shell's `onAction` interceptors).
 */
activeToolId?: string, panelJson?: string, contributionsJson?: string,
/**
 * 🗣️ Active UI locale; plugins resolve their own label set from this via `resolve_labels`/
 * `app_labels!`. Non-optional — the shell always resolves one (see `initUiLocaleSync`/
 * `detectShellLocale`) before the first `render`, so "nobody set the locale" is unrepresentable.
 */
locale: Locale,
/**
 * 🗣️ Active terminology id (`Native` default, or an app-declared alternative term set).
 */
terminology: Terminology,
/**
 * 🪟️ The window instance a `render`/`handle_action` call targets — programs key all per-window
 * option state (grid, LOD, selection mode, …) off this, never off `active_window_kind_id`, so that
 * two window instances of the same kind (e.g. split top/perspective panes) never share options.
 */
windowId?: string,
/**
 * 🪟️ The live set of open window instances (base + spawned/split), sent on every refresh/action so
 * `window_engagements`/`window_measures` can return one entry per instance instead of per kind.
 */
windowInstances: Array<ViewWindowInstance>, };"####,
        },
        SchemaMetadata {
            name: "ViewWindowInstance",
            version: 1,
            typescript: r####"/**
 * 🪟️ One live window instance, as seen by a plugin: `id` is the instance id (equal to `window_kind_id`
 * for a base, unsplit window), `window_kind_id` is the `AppDefinition.windowKinds` entry it renders.
 */
export type ViewWindowInstance = { id: string, windowKindId: string, };"####,
        },
        SchemaMetadata {
            name: "WindowEngagement",
            version: 1,
            typescript: r####"export type WindowEngagement = { sessionActive?: boolean, options?: Array<WindowEngagementOption>, input?: WindowEngagementInput, control?: WindowEngagementControl, controls?: Array<WindowEngagementControl>, status?: Array<WindowEngagementStatus>, possibleEngagements?: Array<WindowEngagementPossible>, };"####,
        },
        SchemaMetadata {
            name: "WindowEngagementControl",
            version: 1,
            typescript: r####"export type WindowEngagementControl = { "kind": "slider", id?: string, label?: string, value: number, min: number, max: number, step?: number, unit?: string, disabled?: boolean, onChange?: ActionDescriptor, onCommit?: ActionDescriptor, } | { "kind": "stepper", id?: string, label?: string, value: number, min?: number, max?: number, step?: number, unit?: string, disabled?: boolean, onChange?: ActionDescriptor, onCommit?: ActionDescriptor, } | { "kind": "ring", id?: string, label?: string, value?: string, options: Array<WindowEngagementRingOption>, disabled?: boolean, onSelect?: ActionDescriptor, } | { "kind": "toggleGroup", id?: string, label?: string, value?: string, options: Array<WindowEngagementToggleGroupOption>, disabled?: boolean, onSelect?: ActionDescriptor, } | { "kind": "select", id?: string, label?: string, value?: string, placeholder?: string, items: Array<WindowEngagementSelectItem>, disabled?: boolean, onChange?: ActionDescriptor, };"####,
        },
        SchemaMetadata {
            name: "WindowEngagementInput",
            version: 1,
            typescript: r####"export type WindowEngagementInput = { id?: string, value?: string, placeholder?: string, disabled?: boolean, onChange?: ActionDescriptor, onSubmit?: ActionDescriptor, onRepeatLast?: ActionDescriptor, onAbort?: ActionDescriptor, };"####,
        },
        SchemaMetadata { name: "WindowEngagementOption", version: 1, typescript: r####"export type WindowEngagementOption = { id: string, label?: string, iconId?: IconName, pressed?: boolean, disabled?: boolean, action?: ActionDescriptor, };"#### },
        SchemaMetadata { name: "WindowEngagementPossible", version: 1, typescript: r####"export type WindowEngagementPossible = { id: string, label: string, detail?: string, action?: ActionDescriptor, };"#### },
        SchemaMetadata { name: "WindowEngagementRingOption", version: 1, typescript: r####"export type WindowEngagementRingOption = { id: string, label: string, disabled?: boolean, };"#### },
        SchemaMetadata { name: "WindowEngagementSelectItem", version: 1, typescript: r####"export type WindowEngagementSelectItem = { id: string, value: string, label: string, };"#### },
        SchemaMetadata {
            name: "WindowEngagementSlot",
            version: 1,
            typescript: r####"/**
 * 🤝️ Closed replacement for `Option<WindowEngagement>` — makes "this window kind never engages" a
 * named variant instead of `None`, so absence is an explicit, typed state rather than an implicit gap.
 * ⚠️ `WindowEngagement` is a wide variant (nested `Vec`/`Option` fields), making `Some` far
 * larger than `None` — boxing it would be a breaking public-API change (every construction/match
 * site across ~30 plugins would need `Box::new`/deref updates), out of scope for a mechanical pass.
 */
export type WindowEngagementSlot = { "kind": "none" } | { "kind": "some", "value": WindowEngagement };"####,
        },
        SchemaMetadata { name: "WindowEngagementStatus", version: 1, typescript: r####"export type WindowEngagementStatus = { id: string, text: string, };"#### },
        SchemaMetadata { name: "WindowEngagementToggleGroupOption", version: 1, typescript: r####"export type WindowEngagementToggleGroupOption = { id: string, label: string, disabled?: boolean, };"#### },
        SchemaMetadata {
            name: "WindowKindDefinition",
            version: 1,
            typescript: r####"export type WindowKindDefinition = { id: string,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown, bodyKey: string, surfaceKind: SurfaceKind, iconId: IconName,
/**
 * 🎛️ Always-present chrome facets (was: separately-optional `measures`/`engagement`).
 */
options: WindowOptions,
/**
 * 📇️ Actions owned by this window kind. Mandatory, may be empty, never absent.
 */
actions: Array<ActionDefinition>,
/**
 * 🧰️ Utilities this window kind accepts — references `AppDefinition.utilities` ids. Empty = no utilities.
 */
utilities: Array<UtilityRef>,
/**
 * 🕹️ Interaction domains this window kind accepts — references `AppDefinition.interactions` ids.
 * Empty = no interactions.
 */
interactions: Array<InteractionRef>, paramsSchema?: string, artifactSnapshotSchema?: string, inputEventSchema?: string, outputSchema?: string, capabilities: Array<CapabilityRequirement>, };"####,
        },
        SchemaMetadata { name: "WindowLayout", version: 1, typescript: r####"export type WindowLayout = { root: WindowLayoutRoot, };"#### },
        SchemaMetadata { name: "WindowLayoutAxisNode", version: 1, typescript: r####"export type WindowLayoutAxisNode = { kind: string, size?: number, children: Array<WindowLayoutChild>, };"#### },
        SchemaMetadata { name: "WindowLayoutChild", version: 1, typescript: r####"export type WindowLayoutChild = WindowLayoutAxisNode | WindowLayoutStackNode;"#### },
        SchemaMetadata { name: "WindowLayoutRoot", version: 1, typescript: r####"export type WindowLayoutRoot = WindowLayoutAxisNode | WindowLayoutStackNode;"#### },
        SchemaMetadata { name: "WindowLayoutStackNode", version: 1, typescript: r####"export type WindowLayoutStackNode = { kind: string, size?: number, activeWindowKindId?: string, children: Array<WindowLayoutWindowNode>, };"#### },
        SchemaMetadata {
            name: "WindowLayoutWindowNode",
            version: 1,
            typescript: r####"export type WindowLayoutWindowNode = { kind: string, windowKindId: string, title?: string, instanceId?: string, templateId?: string, corner?: WindowStackCorner, };"####,
        },
        SchemaMetadata {
            name: "WindowMeasure",
            version: 1,
            typescript: r####"export type WindowMeasure = { "kind": "select", id: string, label?: string, value: string, items: Array<MeasureSelectItem>, onChange: ActionDescriptor, } | { "kind": "slider", id: string, label?: string, value: number, min: number, max: number, step?: number,
/**
 * 🎚️ Absolute value on the fixed `[min, max]` range that is already preloaded/ready.
 * Renderers keep `max` stable and draw a highlight from the knob to this extent.
 */
ready?: number,
/**
 * 🌀️ When true, the measure tree leaf shows a loading ring while preload continues.
 */
loading?: boolean,
/**
 * 🌀️ When true, the measure tree leaf shows a dashed, slower waiting ring; `loading` takes precedence when both are set.
 */
waiting?: boolean,
/**
 * 🚫️ When true, the slider is inert — used when a parent weight is zero so joint percentages cannot change anything.
 */
disabled?: boolean,
/**
 * 🪣️ When set, this is a reveal-group id: the host must NOT dispatch `onChange` on every drag
 * value — only on gesture commit (pointer-up) — and while dragging must locally cut off
 * instances tagged with this reveal group's id instead. See `WorldInstancesLayer`'s reveal
 * cutoff store and `revealCutoffs` in `World3dScene.interaction_json`.
 */
reveal?: string, onChange: ActionDescriptor, } | { "kind": "toggle", id: string, iconId: IconName, label?: string, pressed: boolean, text?: string, onChange: ActionDescriptor, } | { "kind": "group", id: string, label: string, defaultOpen?: boolean,
/**
 * 🎯️ When `Some(utility_id)`, this group is *utility-scoped chrome*: the shell surfaces it only while
 * `ViewModel.active_utility_id == utility_id`, and renders it in the dedicated "Utility Options" rail
 * beside the utility bar — never in the always-on Measures overlay. When absent, the group is a
 * general measure and stays in the Measures overlay exactly as before. See [`partition_window_measures`].
 */
activeUtilityId?: string,
/**
 * 🎚️ Optional header slider — when set with `on_change`, the group row hosts a weight control (e.g. object-kind probability).
 */
value?: number, min?: number, max?: number, step?: number, ready?: number, loading?: boolean, waiting?: boolean, onChange?: ActionDescriptor, children: Array<WindowMeasure>, };"####,
        },
        SchemaMetadata {
            name: "WindowOptions",
            version: 1,
            typescript: r####"/**
 * 🎛️ Everything a window kind can expose beyond its rendered body — always present as a shape,
 * empty collections/`WindowEngagementSlot::None` for windows that don't use a given facet.
 * Replaces the previously separately-optional `measures`/`engagement` pair on `WindowKindDefinition`.
 */
export type WindowOptions = { measures: Array<WindowMeasure>, engagement: WindowEngagementSlot, };"####,
        },
        SchemaMetadata {
            name: "ArtifactPresentation",
            version: 1,
            typescript: r####"/**
 * @emoji 🧷️ Catalog presentation of the artifact owned by an app.
 */
export type ArtifactPresentation = { id: string, name: string, dimension: string, componentKind: string, };"####,
        },
        SchemaMetadata {
            name: "ComposerEntryDescriptor",
            version: 1,
            typescript: r####"/**
 * @emoji 🎹️ One registered composer route; executable composition stays runtime-only.
 */
export type ComposerEntryDescriptor = { writes: ArtifactDialect, reads: Array<ArtifactDialect>, };"####,
        },
        SchemaMetadata {
            name: "ConfigFieldShape",
            version: 1,
            typescript: r####"/**
 * @emoji 🧮️ Owned edit and validation shape for one configuration field.
 */
export type ConfigFieldShape = { "kind": "number", min?: number, max?: number, step?: number, } | { "kind": "toggle" } | { "kind": "text" } | { "kind": "select", options: Array<string>, } | { "kind": "record", fields: Array<ConfigFieldSpec>, };"####,
        },
        SchemaMetadata {
            name: "ConfigFieldSpec",
            version: 1,
            typescript: r####"/**
 * @emoji 🧮️ One field in an app configuration record.
 */
export type ConfigFieldSpec = { key: string, label: string, shape: ConfigFieldShape, default?: unknown, };"####,
        },
        SchemaMetadata {
            name: "ConfigSpec",
            version: 1,
            typescript: r####"/**
 * @emoji 🧮️ An app's complete typed configuration declaration.
 */
export type ConfigSpec = { fields: Array<ConfigFieldSpec>, };"####,
        },
        SchemaMetadata {
            name: "CommandFieldSpec",
            version: 1,
            typescript: r####"/**
 * @emoji 🎛️ One field in a keyword-dispatched command variant.
 */
export type CommandFieldSpec = { key: string, shape: ConfigFieldShape, optional: boolean, };"####,
        },
        SchemaMetadata {
            name: "CommandVariantSpec",
            version: 1,
            typescript: r####"/**
 * @emoji 🎛️ One keyword and its typed command fields.
 */
export type CommandVariantSpec = { keyword: string, fields: Array<CommandFieldSpec>, };"####,
        },
        SchemaMetadata {
            name: "CommandGrammar",
            version: 1,
            typescript: r####"/**
 * @emoji 🎛️ An app's complete typed binary command grammar.
 */
export type CommandGrammar = { variants: Array<CommandVariantSpec>, };"####,
        },
        SchemaMetadata {
            name: "FileTypeContribution",
            version: 1,
            typescript: r####"/**
 * @emoji 🗂️ One import or export format contributed by an app.
 */
export type FileTypeContribution = { formatKind: string, mediaType: MediaType, imports: boolean, exports: boolean, };"####,
        },
        SchemaMetadata {
            name: "IoEntryDirection",
            version: 1,
            typescript: r####"/**
 * @emoji 🚪️ Direction of one registered IO dialect route.
 */
export type IoEntryDirection = "import" | "export";"####,
        },
        SchemaMetadata {
            name: "IoEntryDescriptor",
            version: 1,
            typescript: r####"/**
 * @emoji 🚪️ One registered route between owned artifact dialects.
 */
export type IoEntryDescriptor = { owner: ArtifactDialect, counterpart: ArtifactDialect, direction: IoEntryDirection, };"####,
        },
        SchemaMetadata {
            name: "TopicContribution",
            version: 1,
            typescript: r####"/**
 * @emoji 🗂️ Open plugin contribution keyed by a dot-namespaced topic.
 */
export type TopicContribution = { topic: string, payload: unknown, };"####,
        },
    ];

    /// 🔍️ Rejects unversioned, duplicate, or name-mismatched schema rows before generation.
    pub fn validate() -> Result<(), String> {
        let mut names = HashSet::with_capacity(TYPES.len());
        for metadata in TYPES {
            if metadata.version == 0 {
                return Err(format!("schema '{}' has version zero", metadata.name));
            }
            if !names.insert(metadata.name) {
                return Err(format!("duplicate schema '{}'", metadata.name));
            }
            let type_prefix = format!("export type {}", metadata.name);
            let interface_prefix = format!("export interface {}", metadata.name);
            if !metadata.typescript.contains(&type_prefix) && !metadata.typescript.contains(&interface_prefix) {
                return Err(format!("schema '{}' declaration has a mismatched name", metadata.name));
            }
        }
        Ok(())
    }

    /// 🟦️ Renders the stable language projection consumed by framework clients.
    pub fn render_typescript() -> String {
        let mut output = String::from("/** @generated by `bun nx run @semio-tech/framework:generate` from versioned owned framework schema metadata. Do not edit. */\n\nimport type { Label, StyleSpec, MenuRef as UiMenuRef, RowActionPlacement as UiTreeActionPlacement, WindowStackCorner } from \"./🟦️ui-contract.ts\";\nimport type { ShellLocale as Locale, ShellTerminology as Terminology } from \"./🟦️ui-axes.ts\";\n\n");
        for (index, metadata) in TYPES.iter().enumerate() {
            output.push_str(metadata.typescript);
            output.push_str(if index + 1 == TYPES.len() { "\n" } else { "\n\n" });
        }
        output
    }
}
//#endregion 🧬️SchemaMetadata

#[path = "../../🔨️modules/🎯️action-bus/🦀️.rs"]
pub mod action_bus;

#[path = "../../🔨️modules/🚪️io/🦀️.rs"]
pub mod io;

#[path = "../../🔨️modules/🌉️abi/🦀️.rs"]
pub mod abi;

// 🧬️ ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM W1-A task 1: the io vocabulary
// (`StandardId`/`SubsetId`/`Dialect`/`ArtifactDialect`/`ArtifactKindId`/`ArtifactRef`) is mounted
// ONCE, in the os-kernel crate (`io_schema` there) — re-exported here rather than remounted, so
// this crate never compiles a second copy of that file's source text. `io/🦀️.rs` above
// (still double-mounted, D2) reaches it via `crate::io_schema`, which resolves to THIS re-export
// when compiled as part of this crate.
pub use semio_framework_os_kernel::io_schema;

#[path = "../../🔨️modules/🖥️platform/🦀️.rs"]
pub mod platform;

#[path = "../../🔨️modules/🛂️manifest/🦀️.rs"]
pub mod manifest;

// 🕹️ ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM W0: pure hover/selection state
// machine + declaration types, mirroring `manifest`'s own facet-nesting convention (`schema` sits
// alongside the root `component` rather than under it, same as `writer`'s `config { component; schema; }`).
#[path = "."]
pub mod interaction {
    #[path = "../../🔨️modules/🕹️interaction/🦀️.rs"]
    mod component;
    pub use component::*;

    #[path = "../../🔨️modules/🕹️interaction/🧬️schema/🦀️.rs"]
    pub mod schema;
}

// 🔁️ ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W1: mounted
// HERE, not in the os-kernel crate — its `semio_framework::{AppDefinition, MediaClass, MediaType,
// ConfigSpec, Terminology, Locale, …}` references need this crate's full assembled surface (mesh's
// media vocabulary, manifest's kernel types, ui_wgpu's Locale/Terminology — all re-exported below),
// which the wasm-safe os-kernel crate cannot depend on without a real dependency cycle (see the
// os-kernel glue.rs's own comment at the site this used to be attempted). The run crate's own
// `extern crate ... as workflow;` alias points here now, not at the kernel.
#[path = "../../🛍️products/💻️os/🔨️modules/🔁️workflow/🦀️.rs"]
pub mod workflow;

pub use action_bus::{
    optional_json_to_dsl, ActionBus, ErasedToolJob, ToolCancellationPolicy, ToolDispatchError, ToolExecutionContract, ToolExecutionShape, ToolFactoryKey, ToolFreshnessPolicy, ToolJobDispatch, ToolJobFactory, ToolJobFactoryError, ToolOperationSpec,
    ToolPayload, ToolRegistrationError, ToolWireAdmission,
};
pub use dsl::{from_dsl_value, to_dsl_value, DslValue};
pub use dsl::{Diagnostic, Fault, FaultCause, FaultCode, FaultFrom, FaultOrigin, FaultScope, Severity, TextError, TextSpan};

// 🛂️ The declarative component model (layout/utilities/UiNode) lives in `ui_wgpu` now — re-import
// honestly (not a re-export) wherever this crate's manifest/kernel types need it; see `pub mod manifest`.
// 🔺️ Mesh geometry data, primitive construction, and Obj/Glb/Stl codecs are dissolved into a
// dedicated engine crate (consumed only from artifact facet code / engine-to-engine callers such
// as brep tessellation) — no longer part of this framework module's own re-export surface.
pub use semio_framework_mesh_engine::{
    mesh_box, mesh_cone, mesh_cylinder, mesh_from_glb, mesh_from_indexed, mesh_from_indexed_with_face_groups, mesh_from_kind, mesh_from_obj, mesh_from_stl, mesh_ico_sphere, mesh_plane, mesh_to_glb, mesh_to_obj, mesh_to_stl, mesh_torus,
    mesh_uv_sphere, GlbExporter, GlbImporter, IoError, MeshData, MeshExporter, MeshImporter, ObjExporter, ObjImporter, StlExporter, StlImporter,
};
// 🚪️ DWG codec (`dwg_to_bytes`/`dwg_from_bytes`/`mesh_to_dwg_drawing`/…) DELETED (ticket 26/08/12/
// DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave DEDUP): `🔺️mesh/🦀️.rs`
// was a misplaced, fully-duplicated copy of stdio's real DWG artifact
// (`semio_s_plugin_stdio::artifacts::dwg::{dwg_to_bytes, dwg_from_bytes, mesh_to_dwg_drawing, …}`,
// `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/…`). Its sole framework-tier caller
// (`🧊️3d/📐️brep/📦️mesh-io`) moved into stdio's own brep engine this same wave, so this re-export
// has zero remaining callers. `🔺️mesh/🟦️.ts` (unrelated scene-protocol payload types,
// still imported by `🟦️.ts`) was NOT touched — only the Rust DWG codec shared that directory.
// 🔀️ OsMediaCapability/ArtifactKindSpec/MediaClass/MediaForm/MediaType/MediaWireFormat/MediaPortDirection/
// PortMultiplicity/MediaPortSpec/MediaCompat/media_types_compatible/Media/MediaPayload/MediaFingerprint/
// MediaError/MediaConverter/AppIo/ArtifactPresentation/ConfigFieldShape/ConfigFieldSpec/ConfigSpec/
// CommandFieldSpec/CommandVariantSpec/CommandGrammar relocated from `mesh` into `manifest` (ticket
// 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT wave 4a) — reachable below via `pub use manifest::*;`
// instead, so no external call site needs to change.
pub use abi::*;
pub use interaction::*;
pub use io::{
    dialects_for as io_dialects_for,
    format_accept_filter,
    format_descriptor,
    formats_csv,
    io_compose_via,
    io_dispatch,
    io_keys_for,
    list_composer_entries,
    normalize_format_kind,
    preflight_composer_entry_refs,
    preflight_format_descriptors,
    preflight_subset_validators,
    register_composer_entries,
    register_composer_entry_refs,
    register_format_descriptors,
    register_subset_validator,
    register_subset_validators,
    resolve as io_resolve,
    resolve_ready,
    set_io_fallback_dispatcher,
    subset_validator_entry_of,
    wire_artifact_compose,
    wire_decode_composed_artifact,
    wire_list_composer_entries,
    Analysis,
    AnalyzeSource,
    ArtifactDialect,
    AsyncComposeFn,
    ComposeError,
    // 🌀️ `io-async-signatures`: the async `ComposerEntry.compose` plumbing — see that module's own
    // doc comments (`ComposeFuture`/`AsyncComposeFn`/`resolve_ready`) for what each does.
    ComposeFuture,
    ComposeSource,
    ComposedArtifact,
    ComposerEntry,
    Composition,
    Confidence as IoConfidence,
    Dialect,
    ErasedComposeSource,
    FormatDescriptor,
    FormatRegistryError,
    IoDirection,
    IoFallback,
    IoFallbackDispatcher,
    IoKey,
    IoPayload,
    IoResolveError,
    StandardId,
    SubsetId,
    SubsetValidator,
    SubsetValidatorEntry,
    WireComposeSource,
    WireComposedArtifact,
};
pub use manifest as ui;
pub use manifest::kernel::{
    decode_presence_peer,
    encode_presence_peer,
    ActionContext,
    ActionDef,
    ActionId,
    ActionInvocation,
    ActionRequest,
    ActorId,
    AppEvent,
    AppInstanceId,
    Appearance,
    ArtifactDiff,
    ArtifactHandle,
    ArtifactId,
    ArtifactKind,
    ArtifactVersion,
    AssetHandle,
    Capability,
    CapabilityGrant,
    CapabilityRequirement,
    CapabilityToken,
    CommandContext,
    CommandId,
    CommandInvocation,
    Effect,
    HybridLogicalTimestamp,
    IconRenderExportItem,
    InverseMutation,
    InvocationId,
    InvocationResult,
    KernelMutation,
    MutationId,
    PhysicalSize,
    PluginInstanceId,
    PresencePeer,
    PresenceUi,
    PresenceViewKind,
    PresenceWindowView,
    // 🎫️ ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME packet A3-kernel-types: `RequestId`
    // is the completion-correlation id every `req`-carrying `Effect` variant now needs at its call
    // site — re-exported here so plugin call sites can name it as `semio_framework::RequestId` /
    // `semio_framework_plugin::RequestId` without a separate import.
    RequestId,
    Rights,
    SchemaId,
    SchemaVersion,
    Scope,
    UndoGroup,
    UndoPolicy,
    WindowEvent,
    WindowHandle,
    WindowInput,
    WindowKindDef,
    WindowKindId,
    WindowOutput,
};
pub use manifest::*;
pub use platform::{PanelVisibility, Platform, PlatformSpec};
pub use workflow::*;
