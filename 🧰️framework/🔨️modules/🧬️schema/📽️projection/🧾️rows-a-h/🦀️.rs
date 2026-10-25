//! 🧾️ Projection rows from `ActionAddress` to `HierarchyProvider`, in declaration order.

    use super::SchemaMetadata;

    pub(super) const ROWS: &[SchemaMetadata] = &[
        SchemaMetadata {
            name: "ActionAddress",
            version: 1,
            typescript: r####"/**
 * 📍️ Fully qualified address of an action owned by one concrete window instance.
 */
export type ActionAddress = { pluginId: string, appId: string, modeId: string, windowKindId: string, windowInstanceId: string, actionId: string, };"####,
        },
        SchemaMetadata {
            name: "ActionArgControl",
            version: 3,
            typescript: r####"/**
 * 🎚️ Declarative input control for one action argument — a lean manifest-altitude enum,
 * deliberately NOT `ui_wgpu::wgpu::UiControlNode` (whose variants embed live values and immediate-dispatch
 * wiring). Renderers map each variant onto a staged form field. Tagged with `kind` to mirror the
 * sibling `UtilityNode`/`UiControlNode` declarative-tree convention.
 */
export type ActionArgControl = { "kind": "text", placeholder?: string, } | { "kind": "multiline" } | { "kind": "number", min?: number, max?: number, step?: number, unit?: string, precision?: number, displayUnit?: string, displayFactor?: number, } | { "kind": "stepper", min?: number, max?: number, step?: number, unit?: string, precision?: number, displayUnit?: string, displayFactor?: number, snaps?: Array<number>, snapSource?: SnapSource, } | { "kind": "slider", min: number, max: number, step?: number, unit?: string, precision?: number, displayUnit?: string, displayFactor?: number, snaps?: Array<number>, snapSource?: SnapSource, scale?: NumberScale, } | { "kind": "dial", min: number, max: number, step?: number, unit?: string, precision?: number, displayUnit?: string, displayFactor?: number, snaps?: Array<number>, snapSource?: SnapSource, } | { "kind": "toggle" } | { "kind": "select", options: Array<ActionArgOption>, } | { "kind": "segmented", options: Array<ActionArgOption>, } | { "kind": "vector", dims: number, min?: number, max?: number, unit?: string, step?: number, snaps?: Array<number>, snapSource?: SnapSource, precision?: number, displayUnit?: string, displayFactor?: number, } | { "kind": "color", alpha: boolean, } | { "kind": "reference", kinds: Array<string>, domain?: string, granularity?: string, many?: boolean, minItems?: number, maxItems?: number, idType?: ReferenceIdType, } | { "kind": "iconSelect", classifierKind: string, } | { "kind": "artifactKind", roles: Array<AppRole>, } | { "kind": "surfaceApp", roles: Array<AppRole>, dialectArg: string, };"####,
        },
        SchemaMetadata {
            name: "ActionArgDef",
            version: 2,
            typescript: r####"/**
 * 📝️ Declares one input of an action or a mutation: its `id` (the JSON key sent in `ActionDescriptor.args`; for a
 * mutation input the single-segment RFC 6901 pointer of its key relative to its parent value, e.g. `/dx`), human
 * `label`, stored value `schema` (see `🔖️ArgSchema` — D6: this is the sole persisted truth, `control()` below is
 * derived from it), an optional widget `presentation` hint, whether it is `required`, an optional `default` value,
 * an optional localized `description`, and the `group`/`order` a form sorts it by. An empty
 * `ActionDefinition.args` (the common case) means a no-argument action. Mutation inputs come from
 * [`mutation_input_defs`].
 */
export type ActionArgDef = { id: string,
/**
 * 🗣️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown, schema: ArgSchema, presentation?: ArgPresentation, required: boolean,
/**
 * 🫥️ Whether the value admits `null` (`type: [T, "null"]`, or a `null` branch beside one value branch): a "clear"
 * that a form offers beside the value, distinct from leaving an optional input out.
 */
nullable?: boolean, default?: unknown, description?: unknown, group?: string, order?: number, };"####,
        },
        SchemaMetadata {
            name: "ActionArgOption",
            version: 1,
            typescript: r####"/**
 * 🔘️ One selectable option of a `Select` argument control — the persisted `value` and its
 * human `label`.
 */
export type ActionArgOption = { value: string,
/**
 * 🔖️ Manifest-level, locale×terminology-checked — see `LocalizedLabel`. Not yet owned-schema-mirrored
 * (follow-up: `LocalizedLabel` itself has no `TS` impl).
 */
label: unknown, };"####,
        },
        SchemaMetadata {
            name: "ActionDefinition",
            version: 1,
            typescript: r####"/**
 * 📇️ Declares one action an app can receive via `ActionDescriptor.action`.
 */
export type ActionDefinition = { id: string,
/**
 * 🎚️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
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
 * 📨️ One addressed action invocation with named JSON arguments.
 */
export type ActionInvocation = { address: ActionAddress, arguments: Record<string, unknown>, };"####,
        },
        SchemaMetadata {
            name: "ActionKind",
            version: 1,
            typescript: r####"/**
 * 🗂️ Classifies a declared action by how it interacts with VCS history.
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
 * 🎯️ What an `ActionDefinition`/`CommandDefinition` MEANS to an agent: effects, policy,
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
 * 🤖️ What a package OFFERS to agents — see the region header above for the critical
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
            version: 2,
            typescript: r####"export type AppDefinition = { id: string,
/**
 * 👁️✏️ Whether this surface may mutate the artifact it is bound to — see `AppRole`.
 */
role: AppRole,
/**
 * 📕️ The dialect coordinate (artifact kind, standard, subset) this surface is bound to — see
 * `ArtifactDialect`. Together with `role` this derives the canonical `id` via `surface_app_id`.
 */
dialect: ArtifactDialect,
/**
 * 📺️ The app's own display name (e.g. "Puzzle 3D") — manifest-level, locale×terminology-checked,
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
 * 🪟️ See `modes` above — `WindowKinds` is `NonEmptyVec<WindowKindDefinition>`.
 */
windowKinds: WindowKindDefinition[],
/**
 * 🕹️ The app-wide action roster: every action dispatchable in ANY window of this app that no
 * window kind claims as its own, including the framework-injected History/Clipboard/tutorial
 * constants. A window kind's dispatchable set is `WindowKindDefinition.actions` followed by this
 * roster minus the ids some window claims — `resolveWindowActions` (`🎯️action-bus/🟦️.ts`), whose
 * Rust twin is `window_kind_actions`. Kept here rather than copied into every window kind because a
 * copy made the package descriptor grow as `apps × window kinds × actions`.
 */
actions: Array<ActionDefinition>, panelTabs: Array<PanelTabDefinition>, keybindings: Array<Keybinding>,
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
 * 🖱️ The interaction domains (hover + selection) this app exposes (referenced by
 * `WindowKindDefinition.interactions`) — see `crate::InteractionDefinition`.
 */
interactions: Array<InteractionDefinition>, namedLayouts: Array<NamedLayout>, defaultLayout?: WindowLayout,
/**
 * 🔑️ Terminology ids this app declares beyond the implicit "native" default.
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
 * 🖨️ This app's workflow output ports — see `crate::MediaPortSpec`.
 */
mediaOutputs: Array<MediaPortSpec>,
/**
 * 📦️ OS resource kinds this app produces/consumes — see `crate::ArtifactKindSpec`. Drives
 * `framework/product/os/core`'s artifact catalog registry instead of a hardcoded per-app match.
 */
artifactKinds: Array<ArtifactKindSpec>,
/**
 * 🧮️ This app's typed configuration record — see `crate::ConfigSpec`. Empty until per-app waves
 * populate it.
 */
config: ConfigSpec,
/**
 * ⌨️ This app's typed binary command grammar — see `crate::CommandGrammar`. Empty until per-app
 * waves populate it.
 */
commandGrammar: CommandGrammar,
/**
 * 📡️ This app's typed media I/O surface — see `crate::AppIo`. Not yet populated; `media_inputs`/
 * `media_outputs`/`artifact_kinds` above remain the live source of truth until later waves migrate
 * onto this.
 */
io: AppIo,
/**
 * 📢️ The localized notices of the fault codes this app's guest refuses with (`ArtifactApp::fault_notices`, design
 * §20.12), stamped at registration — what both shells show for a refused dispatch after the framework's own table.
 */
faultNotices: Array<FaultNoticeDefinition>, };"####,
        },
        SchemaMetadata {
            name: "AppIo",
            version: 1,
            typescript: r####"/**
 * 🔌️ An app's full media I/O surface — the artifact schema/type every app carries implicitly (see
 * `artifact_in_port`/`artifact_out_port`) plus whatever additional workflow ports, catalog
 * export/import formats, and OS presentation it declares itself. Scaffolding for the typed manifest
 * surface (`AppDefinition.io`); apps don't populate this yet — later waves migrate `media_inputs`/
 * `media_outputs`/`artifact_kinds` onto it.
 */
export type AppIo = { artifactSchema: string, artifactMediaType: MediaType,
/**
 * 🔗️ App-specific ports only — the implicit artifact ports are auto-injected by `all_ports`.
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
 * 🚦️ When the gateway must pause for human approval before committing an invocation of this
 * capability.
 */
export type ApprovalMode = "never" | "whenDestructive" | "always";"####,
        },
        SchemaMetadata {
            name: "ArgFormat",
            version: 1,
            typescript: r####"/**
 * 🧬️ Semantic refinement of a `String`-typed `ArgSchema` leaf — what KIND of string this is,
 * beyond "text". Orthogonal to `ArgPresentation` (which is about the WIDGET, not the value's
 * semantics): a `Color` format could still render as free text in a minimal shell.
 */
export type ArgFormat = { "kind": "artifactRef" } | { "kind": "windowId" } | { "kind": "entityId", entityKind: string, } | { "kind": "iconId" } | { "kind": "color" } | { "kind": "uri" } | { "kind": "json" } | { "kind": "locale" } | { "kind": "terminology" } | { "kind": "artifactKind", roles: Array<AppRole>, } | { "kind": "surfaceApp", roles: Array<AppRole>, dialectArg: string, } | { "kind": "documentRevision" } | { "kind": "targetRevision" };"####,
        },
        SchemaMetadata {
            name: "ArgPresentation",
            version: 2,
            typescript: r####"/**
 * 🖼️ How to WIDGET-render an argument beyond what its `ArgSchema` alone implies — consumed by
 * `ActionArgDef::control()` (e.g. a bounded `Number` still renders `Slider` without this, but a
 * single-bound one needs it to opt in).
 */
export type ArgPresentation = { "kind": "slider" } | { "kind": "stepper" } | { "kind": "dial" } | { "kind": "segmented" } | { "kind": "iconSelect", classifierKind: string, } | { "kind": "multiline" } | { "kind": "hidden" } | { "kind": "color" };"####,
        },
        SchemaMetadata {
            name: "SnapSource",
            version: 1,
            typescript: r####"/**
 * 🧲️ Where a number's snap points come from beyond its static `snaps`: every multiple of its `step`, a window
 * config value named `key` (the grid spacing a tool snaps to), or the document value at JSON `pointer`.
 */
export type SnapSource = { "kind": "step" } | { "kind": "config", key: string, } | { "kind": "snapshot", pointer: string, };"####,
        },
        SchemaMetadata {
            name: "OptionSource",
            version: 1,
            typescript: r####"/**
 * 🗝️ Where a string choice's options come from (`x-semio-ui.optionSource`): the KEYS of the object `pointer` leads to in
 * the document the editor previews — an RFC 6901 template whose `{field}` segments take the edited payload's top-level
 * member: an object's key, or, on an array, the record whose own `field` equals it (`/hostSnapshot/widgets/{id}/params`).
 * TypeScript twin: `OptionSource`.
 */
export type OptionSource = { "kind": "snapshot", pointer: string, };"####,
        },
        SchemaMetadata {
            name: "NumberScale",
            version: 1,
            typescript: r####"/**
 * 📈️ How a slider or dial maps its travel onto the value range.
 */
export type NumberScale = "linear" | "log";"####,
        },
        SchemaMetadata {
            name: "ReferenceIdType",
            version: 1,
            typescript: r####"/**
 * 🔢️ The JSON type of a reference's ids: text ids (the default, left off the wire) or integer ids. A selection id is
 * always text, so an integer reference stages the integer its selected text spells.
 */
export type ReferenceIdType = "string" | "integer";"####,
        },
        SchemaMetadata {
            name: "ArgSchema",
            version: 3,
            typescript: r####"/**
 * 🌳️ The stored, engine-neutral shape of one action argument's value — see this region's
 * header comment for the D6 stored/derived split.
 */
export type ArgSchema = { "kind": "string", options: Array<ActionArgOption>, optionSource?: OptionSource, minLen?: number, maxLen?: number, pattern?: string, format?: ArgFormat, } | { "kind": "number", min?: number, minExclusive?: boolean, max?: number, maxExclusive?: boolean, step?: number, integer: boolean, unit?: string, snaps?: Array<number>, snapSource?: SnapSource, softMin?: number, softMax?: number, precision?: number, displayUnit?: string, displayFactor?: number, scale?: NumberScale, } | { "kind": "boolean" } | { "kind": "vector", dims: number, min?: number, max?: number, unit?: string, step?: number, snaps?: Array<number>, snapSource?: SnapSource, precision?: number, displayUnit?: string, displayFactor?: number, } | { "kind": "reference", kinds: Array<string>, domain?: string, granularity?: string, many?: boolean, minItems?: number, maxItems?: number, idType?: ReferenceIdType, } | { "kind": "array", items: ArgSchema, minItems?: number, maxItems?: number, } | { "kind": "object", fields: Array<ActionArgDef>, } | { "kind": "any" };"####,
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
 * 🏹️ Stdio export target kind ids (e.g. `stdio.json`) — additive peer of `export_formats`.
 */
exportStdioKinds: Array<string>,
/**
 * 📥️ Stdio import source kind ids — additive peer of `import_formats`.
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
 * 🧮️ What one capability touches — read/write resource selectors plus the three coarse flags
 * the gateway's policy/preview machinery gates on.
 */
export type CapabilityEffects = { reads: Array<ResourceSelector>, writes: Array<ResourceSelector>, external: boolean, destructive: boolean, reversible: boolean, };"####,
        },
        SchemaMetadata {
            name: "CapabilityExecution",
            version: 1,
            typescript: r####"/**
 * ⚙️ Preview/undo/idempotency/cancellation shape of one capability invocation.
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
 * 🛡️ The scope/approval gate a capability invocation must clear — `scopes` are
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
 * 📍️ Fully qualified address of one command.
 */
export type CommandAddress = { owner: CommandOwnerAddress, commandId: string, };"####,
        },
        SchemaMetadata {
            name: "CommandDefinition",
            version: 1,
            typescript: r####"/**
 * 🎛️ Declares one command: a categorized verb offered in the footer command panel.
 * Its owner and availability are derived from the containing OS, plugin, app, or mode definition.
 * Handling a command may emit VCS-tracked operations exactly like an operation-kind action — see
 * `ArtifactApp::handle_command`/`ActionEmit`.
 */
export type CommandDefinition = { id: string,
/**
 * ✅️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown,
/**
 * 🗂️ Footer category tab this command groups under (an open id, e.g. "document", "appearance").
 */
category: string, iconId: IconName, kind: ActionKind,
/**
 * ♻️ Reuses `ActionArgDef` — one staged-form contract shared by actions, dialogs, and commands.
 */
args: Array<ActionArgDef>, keybindings: Array<PlatformKeybinding>, inPalette: boolean,
/**
 * 🧬️ See `ActionDefinition.semantics` — same D6/§3.1 field, same defaulting/inheritance story.
 */
semantics: ActionSemantics, };"####,
        },
        SchemaMetadata {
            name: "CommandInvocation",
            version: 1,
            typescript: r####"/**
 * 📨️ One addressed command invocation with named JSON arguments.
 */
export type CommandInvocation = { address: CommandAddress, arguments: Record<string, unknown>, };"####,
        },
        SchemaMetadata {
            name: "CommandOwnerAddress",
            version: 1,
            typescript: r####"/**
 * 📍️ Hierarchical owner of a command definition.
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
export type ContributedInferenceMetadata = { owner: string, artifactKind: string, artifactSchema: string, artifactSchemaVersion: number, documentSchema: string, documentSchemaVersion: number, inferenceSchema: string, inferenceSchemaVersion: number, algorithmVersion: number, policyVersion: number, contributor: string, dependsOn: Array<string>, payload?: InferencePayloadContract, };"####,
        },
        SchemaMetadata {
            name: "InferencePayloadContract",
            version: 1,
            typescript: r####"/**
 * 📜️ One inference's published payload contract — the schema-first answer to "what do I send?".
 * `inputSchema`/`outputSchema` are JSON Schema 2020-12 documents as TEXT, authored beside the
 * inference and carried verbatim. `progressUnit` names what the bounded job counts.
 */
export type InferencePayloadContract = { payloadSchemaId: string, inputSchema: string, outputSchema: string, progressUnit?: string, artifactBinding?: InferenceArtifactBinding, commit?: InferenceCommitBinding, };"####,
        },
        SchemaMetadata {
            name: "InferenceCommitBinding",
            version: 1,
            typescript: r####"/**
 * 📌️ How one inference's result becomes a document edit: the artifact action that commits it.
 */
export type InferenceCommitBinding = { action: string, };"####,
        },
        SchemaMetadata {
            name: "InferenceArtifactBinding",
            version: 1,
            typescript: r####"/**
 * 🔗️ How one inference's canonical request is BOUND to an artifact document: the payload field the
 * artifact's own pack/spr pair is written into, and the encoding it is written in.
 */
export type InferenceArtifactBinding = { field: string, encoding: string, required: boolean, };"####,
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
            name: "DialogChoice",
            version: 2,
            typescript: r####"/**
 * 🎛️ One decision button of a [`DialogDefinition`]: a localized `label`, an optional `description` a
 * reader hears with it (`aria-describedby`) and a sighted user reads beside it, the `action` it
 * dispatches, the staged args it `requires` (enabled once they resolve, and the only form args it
 * sends), its visual `tone`, and whether it is `destructive` (danger styling, and an agent lane must
 * ask before taking it).
 */
export type DialogChoice = { id: string,
/**
 * ☑️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown, description?: unknown, action: ActionRef, requires?: Array<string>, tone?: Tone, destructive?: boolean, };"####,
        },
        SchemaMetadata {
            name: "DialogDefinition",
            version: 3,
            typescript: r####"/**
 * 🗨️ A declared modal form dialog: a glass veil covers the screen and an info box (styled
 * identically to the introduction walkthrough box, see `ui_react`'s `GLASS_OVERLAY_BOX_CLASS`)
 * presents `args` as a staged form. Submit dispatches `submit_action` with the merged effective
 * args; empty `args` degenerates to a message/confirm dialog. Opened only via
 * `Effect::OpenDialog`; the shell owns open/close as ephemeral chrome state, never the document.
 */
export type DialogDefinition = { id: string,
/**
 * ✔️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
title: unknown, body?: unknown, args: Array<ActionArgDef>,
/**
 * 📇️ References an action owned by the active window kind, dispatched with merged args.
 */
submitAction: ActionRef, submitLabel: unknown,
/**
 * 📘️ Optional active-window action reference dispatched on any dismissal (Escape, veil
 * click, or the Cancel button).
 */
cancelAction?: ActionRef, cancelLabel?: unknown,
/**
 * 🔀️ Further decisions offered beside the submit, in focus order before it. Each is gated on and
 * dispatches only the args it [`requires`](DialogChoice::requires) (plus the seed context), with
 * [`DIALOG_CHOICE_ARG`] naming the choice, so one action can serve several choices
 * (`historyEditCommit{choice}`); the submit alone is gated on the dialog's required args.
 */
choices?: Array<DialogChoice>, };"####,
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
 * 🌐️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown, iconId: IconName, artifactJson: string,
/**
 * ✏️ The DIALECT this example is authored for, never one app of it. An example is a document of an artifact's subset, and every surface bound to that subset — the editor and the viewer alike — opens the same eight fixtures; `SubsetDeclaration.examples` already models them that way. Stamped at registration from the registering app's own `AppDefinition.dialect`, and resolved by [`examples_for_app`] (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 */
dialect: ArtifactDialect, };"####,
        },
        SchemaMetadata {
            name: "ExecutionClass",
            version: 1,
            typescript: r####"/**
 * ⏱️ How long-running/interactive an invocation of this capability is — the gateway's job
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
            name: "FaultNoticeDefinition",
            version: 1,
            typescript: r####"/**
 * 📢️ One app-declared fault notice (design §20.12): the `code` a guest refusal carries (`<app>.<area>.<name>`) and its
 * text in every shell locale × terminology, whose `{name}` placeholders are filled from the fault's `params` — never from
 * its English `message`.
 */
export type FaultNoticeDefinition = { code: string, label: unknown, };"####,
        },
        SchemaMetadata {
            name: "GranularityDefinition",
            version: 1,
            typescript: r####"/**
 * 🔬️ One selectable/hoverable level of detail within a domain (e.g. mesh's object/face/edge/vertex).
 */
export type GranularityDefinition = { id: string,
/**
 * 🔠️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
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
    ];
