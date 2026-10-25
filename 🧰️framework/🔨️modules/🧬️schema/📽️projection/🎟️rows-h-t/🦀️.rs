//! 🎟️ Projection rows from `HoverSpec` to `TutorialDefinition`, in declaration order.

    use super::SchemaMetadata;

    pub(super) const ROWS: &[SchemaMetadata] = &[
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
            name: "HostedArtifactKind",
            version: 1,
            typescript: r####"/**
 * 🏠️ One artifact kind a package hosts — the kind id, one document schema of its owner's codecs, and the OWNER: the plugin
 * that owns the kind (declares it and its codec), a declared dependency of the host (see `PluginManifest::hosted_artifact_kinds`).
 * The owner is explicit, never read off the kind id — an embedded surface's kind (`3d.cad`) names no plugin.
 */
export type HostedArtifactKind = { id: string, schema: string, owner: string, };"####,
        },
        SchemaMetadata {
            name: "IconName",
            version: 2,
            typescript: r####"export type IconName = "alert-circle" | "align-left" | "animate" | "app-window" | "architect" | "architect-graph" | "arrow-down" | "arrow-left" | "arrow-right" | "arrow-right-left" | "arrow-up" | "award" | "bar-chart-3" | "beam" | "bell" | "book-open" | "box" | "building" | "cad-shape" | "calendar" | "calendar-days" | "camera" | "check" | "check-circle-2" | "chevron-down" | "chevron-left" | "chevron-right" | "chevron-up" | "chevrons-up-down" | "circle" | "circle-dot" | "clipboard" | "clipboard-list" | "clock" | "cloud" | "cloud-download" | "code" | "columns" | "combine" | "component" | "copy" | "cpu" | "crosshair" | "cylinder" | "dag" | "display-windows" | "document-jack" | "document-report" | "download" | "draw" | "edit" | "edit-3" | "eraser" | "export" | "external-link" | "eye" | "eye-off" | "fem-app" | "fem-model" | "file" | "file-archive" | "file-code" | "file-image" | "file-json" | "file-spreadsheet" | "file-text" | "file-type" | "file-video" | "filter" | "flip-horizontal" | "flip-vertical" | "flow" | "flow-graph" | "focus" | "folder" | "folder-open" | "folder-plus" | "forms" | "gis2d" | "gis3d" | "git-branch" | "git-commit" | "git-merge" | "globe" | "graduation-cap" | "graph-dag" | "graph-media" | "grid-3x3" | "grip-vertical" | "hammer" | "hand" | "hard-drive" | "hash" | "help-circle" | "hexagon" | "home" | "hud-overlay" | "image" | "image-plus" | "image-up" | "imperative" | "import" | "info" | "landmark" | "lasso" | "layers" | "layout" | "layout-grid" | "library" | "lightbulb" | "link" | "link-2-off" | "list" | "list-checks" | "list-ordered" | "list-tree" | "loader-2" | "lock" | "lock-open" | "lod-depth" | "lowpoly-model" | "magnet" | "map" | "math-app" | "math-graph" | "maximize-2" | "message-circle" | "message-square" | "minimize-2" | "minus" | "monitor" | "moon" | "more-horizontal" | "mouse-pointer" | "mouse-pointer-2" | "move" | "move-3d" | "network" | "note" | "note-math" | "paint-bucket" | "paintbrush" | "palette" | "panel-catalogue" | "panel-inspection" | "panel-left" | "panel-parameters" | "panel-right" | "panel-top" | "pause" | "pen-tool" | "pencil" | "pipette" | "play" | "play-circle" | "plug" | "plus" | "preview" | "procedural2d" | "process-workpiece" | "projection-axonometric" | "projection-curvilinear" | "projection-dimetric" | "projection-isometric" | "projection-oblique" | "projection-oblique-cabinet" | "projection-oblique-cavalier" | "projection-oblique-military" | "projection-one-point" | "projection-orthographic" | "projection-parallel" | "projection-perspective" | "projection-three-point" | "projection-trimetric" | "projection-two-point" | "puzzle" | "puzzle5d-3d" | "raster" | "reasoning-wires" | "rectangle-tool" | "redo" | "redo-2" | "relocate-3d" | "remodel-app" | "remodel-model" | "rotate-ccw" | "rotate-cw" | "s" | "save" | "scaling" | "scan" | "scan-line" | "scene-3d" | "scissors" | "search" | "select-all" | "sequence" | "settings" | "settings-2" | "shapes" | "shooting-scene" | "sigma" | "skip-back" | "skip-forward" | "slab" | "sliders-horizontal" | "smartphone" | "smile" | "sparkles" | "square" | "square-arrow-down-left" | "square-arrow-down-right" | "square-arrow-up-left" | "square-arrow-up-right" | "square-dashed" | "sticky-note" | "sun" | "table-2" | "tablet" | "tags" | "terrain-3d" | "text-cursor" | "text-search" | "toggle-left" | "transform-3d" | "trash" | "trash-2" | "triangle" | "triangle-alert" | "trinity" | "trinity-lhs" | "trinity-rewriting" | "trinity-rhs" | "type" | "typography" | "undo" | "undo-2" | "unlink" | "unlock" | "user" | "users" | "volume-brush" | "window" | "workbench" | "workflow" | "wrench" | "writer" | "x" | "zoom-in" | "zoom-out";"####,
        },
        SchemaMetadata {
            name: "IdempotencyMode",
            version: 1,
            typescript: r####"/**
 * 🔁️ Whether replaying the same invocation twice is safe, and how the gateway makes it so.
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
 * 🖱️ Ghost-cursor glyph, mirroring `🎨️ui.css`'s `--cursor-*` custom cursors.
 */
export type IntroductionCursor = "default" | "pointer" | "grab" | "grabbing" | "crosshair" | "move";"####,
        },
        SchemaMetadata {
            name: "IntroductionDefinition",
            version: 1,
            typescript: r####"/**
 * 🎓️ A first-run walkthrough an app declares to introduce its UI, utilities, and actions to a
 * first-time user. Rendered as an ordered sequence of `IntroductionStepDefinition`s over a full-screen
 * glass veil; the shell owns playback (start/advance/skip) as ephemeral chrome state, never the
 * document.
 */
export type IntroductionDefinition = {
/**
 * 🎚️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
title: unknown, steps: Array<IntroductionStepDefinition>, };"####,
        },
        SchemaMetadata {
            name: "IntroductionDemonstration",
            version: 1,
            typescript: r####"/**
 * 🎬️ A looping ghost-cursor demonstration attached to an interaction-gated
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
 * 👆️ A gesture a demonstration plays: the ghost cursor travels to (or between) `IntroductionPoint`s
 * and performs the visual press/release affordance for the gesture kind.
 */
export type IntroductionGesture = { "kind": "leftClick", at: IntroductionPoint, } | { "kind": "rightClick", at: IntroductionPoint, } | { "kind": "doubleClick", at: IntroductionPoint, } | { "kind": "drag", from: IntroductionPoint, to: IntroductionPoint, button?: IntroductionPointerButton, modifiers?: Array<IntroductionKeyModifier>, } | { "kind": "scroll", at: IntroductionPoint, deltaY: number, } | { "kind": "orbit", from: IntroductionPoint, to: IntroductionPoint, button?: IntroductionPointerButton, modifiers?: Array<IntroductionKeyModifier>, };"####,
        },
        SchemaMetadata {
            name: "IntroductionInteraction",
            version: 1,
            typescript: r####"/**
 * ✅️ One thing the user must do to complete an interaction-gated `IntroductionStepDefinition` —
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
 * 👉️ What one `IntroductionInteraction` requires: `Action`/`Utility`/`Tool`/`Panel`/`Expand`
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
 * ⌨️ Keyboard modifier held during a drag-like demonstration.
 */
export type IntroductionKeyModifier = "alt" | "shift" | "control" | "meta";"####,
        },
        SchemaMetadata {
            name: "IntroductionLogo",
            version: 1,
            typescript: r####"/**
 * 🏛️ One institution/partner logo shown in an `IntroductionStepDefinition`'s info box — a plain
 * URL pair (no DOM/CSS types), optionally linking out when clicked.
 */
export type IntroductionLogo = { src: string, darkSrc: string | null, alt: string, href: string | null, };"####,
        },
        SchemaMetadata {
            name: "IntroductionPlacement",
            version: 1,
            typescript: r####"/**
 * 📍️ Where the info box is placed relative to its anchor.
 */
export type IntroductionPlacement = "auto" | "top" | "bottom" | "left" | "right" | "center";"####,
        },
        SchemaMetadata {
            name: "IntroductionPoint",
            version: 1,
            typescript: r####"/**
 * 📌️ Where a demonstration gesture points, resolvable to a viewport pixel at play time. One
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
 * 🖱️ Which mouse button a drag-like demonstration presses.
 */
export type IntroductionPointerButton = "left" | "middle" | "right";"####,
        },
        SchemaMetadata {
            name: "IntroductionStepDefinition",
            version: 1,
            typescript: r####"/**
 * 🪜️ One step of an `IntroductionDefinition`: an info box pointing at `introduce`, with `show`
 * raising extra elements above the glass veil and `interactions` completing the step.
 */
export type IntroductionStepDefinition = { id: string,
/**
 * ☑️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
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
        SchemaMetadata {
            name: "JobKindId",
            version: 1,
            typescript: r####"/**
 * 🧵️ Id of a job kind registered in the artifact tool factory registry.
 */
export type JobKindId = string;"####,
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
 * 🔖️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
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
 * 💻️ Operating-system command catalog shared by every renderer.
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
export type PackageDescriptor = { descriptorVersion: number, role: PackageRole, manifest: PluginManifest, activationEvents: Array<ActivationEvent>, capabilityRequests: Array<CapabilityRequest>, extensionPoints: Array<ExtensionPointDeclaration>, execution: ExecutionMode, quotas: QuotaSchema, contributions: ContributionSet, assets: Array<AssetDeclaration>, };"####,
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
 * ✔️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
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
 * ⌨️ Operating system selector for a platform-specific keybinding.
 */
export type Platform = "macOs" | "windows" | "linux";"####,
        },
        SchemaMetadata {
            name: "PlatformKeybinding",
            version: 1,
            typescript: r####"/**
 * ⌨️ One command chord, optionally restricted to a host platform.
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
 * 🔌️ Plugin-scope commands this program exposes — apply whenever any of its apps is focused.
 */
commands: Array<CommandDefinition>,
/**
 * 📚️ Plugin-level artifact kinds (library plugins with zero apps declare kinds here).
 */
artifactKinds: Array<ArtifactKindSpec>,
/**
 * 🏠️ Artifact kinds another package owns whose documents this plugin's apps open — one row per kind and document
 * schema its owner's codecs decode (`PluginBuilder::host_artifact`, or a surface of another plugin's artifact the plugin
 * registers). Distinct from `artifact_kinds`: the codec rows stay
 * the owner's, and a trusted catalog binds these kinds to the owner's codecs.
 */
hostedArtifactKinds: Array<HostedArtifactKind>,
/**
 * 🔗️ Direct plugin dependencies this plugin requires to load — see `PluginDependency`/
 * `resolve_load_order`.
 */
dependencies: Array<PluginDependency>,
/**
 * 📦️ Artifact-kind contributions (mutations/inferences) this plugin contributes onto artifact
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
 * 👁️ Whether/how the gateway can show the effect of an invocation before committing it.
 */
export type PreviewMode = "none" | "dryRun" | "diff";"####,
        },
        SchemaMetadata {
            name: "ProgramContributionEntry",
            version: 1,
            typescript: r####"/**
 * 🧩️ One host-aggregated plugin contribution entry — the element shape of the paged
 * `setContributions` command payload, never a view-state field.
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
 * 🎯️ A templated resource-selector string identifying what a capability reads/writes —
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
 * 🪫️ Non-empty; the first entry is the domain's default mode.
 */
modes: Array<SelectionMode>, methods: Array<SelectionMethod>, merges: Array<MergeMode>,
/**
 * 🏹️ Whether selecting a target expands to its descendant closure — requires
 * `hierarchy != HierarchyProvider::Flat`.
 */
transitive: boolean,
/**
 * 📢️ Whether this domain's own selection mirrors into `PresenceInteraction` for peers.
 */
broadcast: boolean, };"####,
        },
        SchemaMetadata {
            name: "SurfaceKind",
            version: 1,
            typescript: r####"export type SurfaceKind = "canvas-2d" | "world-3d" | "node-graph" | "text-editor" | "table" | "paint-2d" | "virtual-file-system" | "tiled-map" | "board-2d" | "icon-render" | "ink-canvas" | "graph-timeline" | "block-list" | "diff-view" | "event-feed";"####,
        },
        SchemaMetadata {
            name: "ToolDefinition",
            version: 1,
            typescript: r####"/**
 * 🛠️ Declares one mode-level tool: an activatable, stateful capability of a whole app mode.
 * Distinct from `UtilityDefinition` (a per-window pointer mode — a utility is a tool for a specific
 * window) and `CommandDefinition` (a fire-once verb): exactly one tool is active per app at a time,
 * and activation is host-owned session view state (`ViewModel.active_tool_id`), never a document
 * field or VCS operation. A tool's live options are supplied dynamically via `ArtifactApp::tool_measures`,
 * keyed by tool id — not part of this static declaration.
 */
export type ToolDefinition = { id: string,
/**
 * 🌐️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
label: unknown, iconId: IconName, keys?: string,
/**
 * ⏯️ Declares that this tool runs a visible, abortable, finalizable algorithm (tool run contract §2.4);
 * reserves the tool run actions through [`crate::tool_run::RESERVED`].
 */
run?: ToolRunDefinition, };"####,
        },
        SchemaMetadata {
            name: "ToolRef",
            version: 1,
            typescript: r####"/**
 * 🛠️ A validated reference into an app's `AppDefinition.tools` registry — the tool mirror of
 * `UtilityRef`, scoping tools to modes with a typed, resolvable id.
 */
export type ToolRef = string;"####,
        },
        SchemaMetadata {
            name: "ToolRunCounterDefinition",
            version: 1,
            typescript: r####"/**
 * 🔟️ One progress counter.
 */
export type ToolRunCounterDefinition = { id: string, label: unknown, };"####,
        },
        SchemaMetadata {
            name: "ToolRunDefinition",
            version: 2,
            typescript: r####"/**
 * 📜️ Static run declaration attached to tool and utility definitions (§2.4).
 */
export type ToolRunDefinition = { mutating: boolean, rebase: ToolRunRebasePolicy, reconfigure: ToolRunReconfigurePolicy, unit: unknown, stages: Array<ToolRunStageDefinition>, counters: Array<ToolRunCounterDefinition>, reasons: Array<ToolRunReasonDefinition>, trace: ToolRunTraceKind, runJob: JobKindId, revalidateJob?: JobKindId, settings?: ToolRunSettingsReads, windows?: Array<string>, member?: string, };"####,
        },
        SchemaMetadata {
            name: "ToolRunReasonDefinition",
            version: 1,
            typescript: r####"/**
 * 🗯️ One reason code with its verdict and EN/DE template (`{0}`..`{3}` take step arguments).
 */
export type ToolRunReasonDefinition = { code: number, id: string, verdict: ToolRunVerdict, template: unknown, };"####,
        },
        SchemaMetadata {
            name: "ToolRunRebasePolicy",
            version: 1,
            typescript: r####"/**
 * 🔄️ How a mutating run reacts to head changes (§2.7.5).
 */
export type ToolRunRebasePolicy = "revalidate" | "restart" | "freeze";"####,
        },
        SchemaMetadata {
            name: "ToolRunReconfigurePolicy",
            version: 1,
            typescript: r####"/**
 * 🎚️ How a run reacts to settings changes (§3.3).
 */
export type ToolRunReconfigurePolicy = "resume" | "restart";"####,
        },
        SchemaMetadata {
            name: "ToolRunSettingsReads",
            version: 1,
            typescript: r####"/**
 * 🎚️ The settings a run's jobs read (§3.3): RFC 6901 JSON Pointers into the app config document and, per
 * window kind id, into that kind's window config documents. `settingsChanged` fires only when a value
 * behind one of them changes; an empty declaration reads no settings, so no settings publication ever
 * reconfigures the run.
 */
export type ToolRunSettingsReads = { config?: Array<string>, windowConfig?: { [key in string]?: Array<string> }, };"####,
        },
        SchemaMetadata {
            name: "ToolRunStageDefinition",
            version: 1,
            typescript: r####"/**
 * 🪜️ One algorithm stage.
 */
export type ToolRunStageDefinition = { id: string, label: unknown, };"####,
        },
        SchemaMetadata {
            name: "ToolRunTraceKind",
            version: 1,
            typescript: r####"/**
 * 🫥️ Trace subject kind a run emits.
 */
export type ToolRunTraceKind = "instance3d" | "placement2d" | "entity" | "none";"####,
        },
        SchemaMetadata {
            name: "ToolRunVerdict",
            version: 1,
            typescript: r####"/**
 * 🚥️ Verdict of one traced attempt; colours are framework semantic tokens.
 */
export type ToolRunVerdict = "testing" | "success" | "warning" | "danger";"####,
        },
        SchemaMetadata {
            name: "TreeWindowRequest",
            version: 1,
            typescript: r####"/**
 * 🪟️ One tree container's host-known state: whether the user opened or closed it (absent = the
 * author's own default still stands) and the row window on screen, overscan included. `bodyKey`
 * names the panel body the container lives in, `nodeKey` the authored container key within it.
 */
export type TreeWindowRequest = { bodyKey: string, nodeKey: string, open?: boolean, offset: number, rows: number, };"####,
        },
        SchemaMetadata {
            name: "TutorialAssetSrc",
            version: 1,
            typescript: r####"/**
 * 📦️ Where a tutorial media asset's bytes live. `Blob` is wire-identical to `store::BlobRef`
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
 * 🎬️ What must be true at t=0: the document the tutorial sandboxes and the initial UI/camera
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
 * 🎥️ One camera track keyframe for a specific window instance.
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
 * 🎥️ A camera pose — `Orbit` mirrors `World3dScene.camera_json`/`OrbitController`, `Canvas`
 * mirrors `Canvas2dScene`'s `cameraX`/`cameraY`/`zoom`.
 */
export type TutorialCameraState = { "kind": "orbit", position: [number, number, number], target: [number, number, number], up: [number, number, number], fov?: number, } | { "kind": "canvas", x: number, y: number, zoom: number, };"####,
        },
        SchemaMetadata {
            name: "TutorialCaption",
            version: 1,
            typescript: r####"/**
 * 💬️ One timed caption sub-segment of a `TutorialNarrationCue`.
 */
export type TutorialCaption = { at: bigint, durationMs: bigint,
/**
 * 📄️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
text: unknown, };"####,
        },
        SchemaMetadata {
            name: "TutorialChapter",
            version: 1,
            typescript: r####"/**
 * 📖️ One scrub-bar marker in a `TutorialDefinition`'s timeline.
 */
export type TutorialChapter = { id: string, at: bigint,
/**
 * 🩺️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
 */
title: unknown, body?: unknown, };"####,
        },
        SchemaMetadata {
            name: "TutorialDefinition",
            version: 1,
            typescript: r####"/**
 * 🎬️ A recorded, timed, replayable walkthrough — the timeline sibling of the step-gated
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
 * 🔠️ Manifest-level, locale×terminology-checked — see `LocalizedLabel` (follow-up: no owned schema mirror yet).
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
 * ▶️ Starting conditions the player restores into its sandbox before t=0.
 */
base: TutorialBase, tracks: TutorialTracks,
/**
 * 🧾️ Recorder provenance (ISO 8601 timestamp); `None` means hand-authored.
 */
recordedAt?: string, };"####,
        },
    ];
