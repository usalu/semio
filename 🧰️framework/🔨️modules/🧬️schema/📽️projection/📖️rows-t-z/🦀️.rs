//! 📖️ Projection rows from `TutorialDocumentEvent` to the last row, in declaration order.

    use super::SchemaMetadata;

    pub(super) const ROWS: &[SchemaMetadata] = &[
        SchemaMetadata {
            name: "TutorialDocumentEvent",
            version: 1,
            typescript: r####"/**
 * 🖋️ One document-track entry — mirrors `store::os_store::store_schema::ArtifactCommand` with `Mutation =
 * serde_json::Value` (opaque per-app mutation JSON, already the wire shape of every `KernelMutation`
 * diff). This is the SOLE source of document mutation during playback: recorded `TutorialEvent`s are
 * annotational only, never re-dispatched, because re-dispatching a plugin action is non-deterministic
 * (fresh ids/timestamps) and would double-apply against this track.
 */
export type TutorialDocumentEvent = { at: bigint, kind: TutorialDocumentEventKind, };"####,
        },
        SchemaMetadata {
            name: "TutorialDocumentEventKind",
            version: 1,
            typescript: r####"/**
 * 🖋️ See `TutorialDocumentEvent`. `Edit` carries both `forwards` and `backwards` operations
 * verbatim from the vcs edit that produced it — the source of exact bidirectional scrubbing.
 */
export type TutorialDocumentEventKind = { "kind": "edit", forwards: unknown[], backwards: unknown[], description?: string, } | { "kind": "undo" } | { "kind": "redo" } | { "kind": "checkpoint", message?: string, } | { "kind": "checkoutCheckpoint", checkpointId: string, } | { "kind": "switchAlternative", alternativeId: string, } | { "kind": "load", documentDsl: string, previousDsl: string, };"####,
        },
        SchemaMetadata {
            name: "TutorialEasing",
            version: 1,
            typescript: r####"/**
 * 🪄️ Interpolation curve into a `TutorialCameraKeyframe` from its predecessor on the same window.
 */
export type TutorialEasing = "linear" | "easeInOut" | "hold";"####,
        },
        SchemaMetadata {
            name: "TutorialEvent",
            version: 1,
            typescript: r####"/**
 * 🏷️ One recorded action/command/keypress, annotational only — see `TutorialTracks::events`.
 */
export type TutorialEvent = { at: bigint, kind: TutorialEventKind, };"####,
        },
        SchemaMetadata {
            name: "TutorialEventKind",
            version: 1,
            typescript: r####"/**
 * 🏷️ What one `TutorialEvent` annotates.
 */
export type TutorialEventKind = { "kind": "action", action: string, args?: unknown, } | { "kind": "command", command: string, args?: unknown, } | { "kind": "key", keys: string, };"####,
        },
        SchemaMetadata {
            name: "TutorialGestureCue",
            version: 1,
            typescript: r####"/**
 * 👻️ One ghost-cursor gesture cue, reusing the introduction demonstration vocabulary verbatim —
 * both shells already resolve/render `IntroductionGesture`/`IntroductionPoint`/`IntroductionCursor`.
 */
export type TutorialGestureCue = { at: bigint, durationMs: bigint, gesture: IntroductionGesture, cursor?: IntroductionCursor, };"####,
        },
        SchemaMetadata {
            name: "TutorialNarrationCue",
            version: 1,
            typescript: r####"/**
 * 🎙️ One voiceover cue: `text` is both the TTS script and the caption fallback; `audio`
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
 * 🖼️ Normalized 0–1 viewport rect for a `TutorialVideoCue` overlay.
 */
export type TutorialOverlayRect = { x: number, y: number, width: number, height: number, };"####,
        },
        SchemaMetadata {
            name: "TutorialTracks",
            version: 1,
            typescript: r####"/**
 * 🎞️ The seven parallel tracks of a `TutorialDefinition`'s timeline; every entry's `at` is a
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
 * 🖋️ The sole source of document mutation during playback — see `TutorialDocumentEventKind`.
 */
document: Array<TutorialDocumentEvent>, camera: Array<TutorialCameraKeyframe>, gestures: Array<TutorialGestureCue>, };"####,
        },
        SchemaMetadata {
            name: "TutorialUiChange",
            version: 1,
            typescript: r####"/**
 * 🩹️ One typed, sparse UI-state change — the alphabet `compose_tutorial_ui` replays over a prior
 * `TutorialUiSnapshot` to reconstruct state at any timeline offset without shipping a full snapshot at
 * every sample.
 */
export type TutorialUiChange = { "kind": "activeMode", id: string, } | { "kind": "focusedWindow", id?: string, } | { "kind": "activeUtility", windowId: string, utilityId?: string, } | { "kind": "activeTool", id?: string, } | { "kind": "layout", layout: WindowLayout, } | { "kind": "panelTab", group: string, tabId?: string, } | { "kind": "panelState", panelJson: string, } | { "kind": "selection", domainId: string, granularity: string, ids: Array<string>, } | { "kind": "dialog", id?: string, args?: unknown, } | { "kind": "treeExpansion", id: string, expanded: boolean, } | { "kind": "commandPanel", open: boolean, };"####,
        },
        SchemaMetadata {
            name: "TutorialUiKeyframe",
            version: 1,
            typescript: r####"/**
 * 🧮️ One UI-state track entry: either a full restore-point snapshot (a valid seek anchor) or a
 * sparse list of changes since the previous sample.
 */
export type TutorialUiKeyframe = { at: bigint, sample: TutorialUiSample, };"####,
        },
        SchemaMetadata {
            name: "TutorialUiSample",
            version: 1,
            typescript: r####"/**
 * 🧮️ See `TutorialUiKeyframe`.
 */
export type TutorialUiSample = { "kind": "snapshot", state: TutorialUiSnapshot, } | { "kind": "delta", changes: Array<TutorialUiChange>, };"####,
        },
        SchemaMetadata {
            name: "TutorialUiSnapshot",
            version: 1,
            typescript: r####"/**
 * 🧮️ Renderer-neutral restore point for chrome/UI state — a superset of `ViewModel` plus the
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
 * 📹️ A timed video overlay — e.g. a presenter webcam picture-in-picture, or an authored clip.
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
            typescript: r####"export type UiInputNode = { id: string, inputKind: string, value: string, placeholder?: Label, accessibilityLabel?: Label, commit?: string, min?: number, max?: number, step?: number, accept?: string, onChange: ActionDescriptor, presence?: UiPresence, menu?: UiMenuRef, };"####,
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
 * initials chip (contract-freeze §C7.6 of ticket `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/
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
        SchemaMetadata {
            name: "UiProgressNode",
            version: 1,
            typescript: r####"/**
 * 📶️ The retained twin of `ui_contract::Component::Progress`: a read-only bar, determinate on
 * `0..=total` or an indeterminate sweep while `total` is `None`. `value_text` is the localized spoken
 * form the accessibility projection announces; paint never invents a percentage string.
 */
export type UiProgressNode = { id: string, completed: number, total?: number, valueText: Label, presence?: UiPresence, menu?: UiMenuRef, };"####,
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
        SchemaMetadata { name: "UiTreeItemAction", version: 3, typescript: r####"export type UiTreeItemAction = { iconId: IconName, label?: Label, action: ActionDescriptor, placement?: UiTreeActionPlacement, disabled?: boolean, reason?: Label, };"#### },
        SchemaMetadata {
            name: "UiTreeItemNode",
            version: 2,
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
menu?: UiMenuRef,
/**
 * 🪟️ The materialised slice of this row's logical child list — see `UiTreeWindow`. A row with
 * `window.total > 0` is expandable even while `items` is empty or absent.
 */
window?: UiTreeWindow,
/**
 * 🎯️ The interaction granularity this row picks when the owning tree carries the domain binding
 * (`UiTreeNode.interactionDomain`), keyed by this row's own `id`.
 */
granularity?: string, 
/**
 * 🚦️ The semantic tone this row's label and icon paint (`Theme::tone_ink`) — a history row's outcome, a refused
 * input: one of the four outcome roles, never the only cue of the state it colours.
 */
tone?: Tone, };"####,
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
        SchemaMetadata {
            name: "UiTreeSectionNode",
            version: 1,
            typescript: r####"export type UiTreeSectionNode = { id: string, label?: Label, defaultOpen?: boolean, presence?: UiPresence, items: Array<UiTreeItemNode>,
/**
 * 🪟️ The materialised slice of this section's logical child list — see `UiTreeWindow`.
 */
window?: UiTreeWindow, };"####,
        },
        SchemaMetadata {
            name: "UiTreeWindow",
            version: 1,
            typescript: r####"/**
 * 🪟️ The materialised slice of a logically `total`-long tree child list: the node's `items` are
 * the entries `[offset, offset + items.length)` of that list. `total > 0` with no materialised
 * items means expandable-but-not-yet-loaded, never "empty" — a renderer pitches the
 * unmaterialised rows as empty spacer bands so the scrollbar spans the whole document. The
 * legacy-node mirror of the UI contract's `TreeWindow`.
 */
export type UiTreeWindow = { total: number, offset: number, rowExtent: UiTreeWindowRowExtent, };"####,
        },
        SchemaMetadata {
            name: "UiTreeWindowRowExtent",
            version: 1,
            typescript: r####"/** 🪟️ Closed geometry token for one unmaterialised row in a virtual Tree window. */
export type UiTreeWindowRowExtent = "standard" | "compactText" | "compactSmallControl" | "compactControl";"####,
        },
        SchemaMetadata {
            name: "UndoMode",
            version: 1,
            typescript: r####"/**
 * ↩️ How a committed invocation of this capability can be undone.
 */
export type UndoMode = { "kind": "none" } | { "kind": "inverse" } | { "kind": "compensate", capability: string, };"####,
        },
        SchemaMetadata { name: "UtilityCategory", version: 1, typescript: r####"export type UtilityCategory = "selection" | "utilities" | "history" | "sync";"#### },
        SchemaMetadata {
            name: "UtilityDefinition",
            version: 1,
            typescript: r####"/**
 * 🧰️ Declares one interactive utility (a live-preview pointer mode) an app exposes. Distinct from
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
allowsActionsWhileActive: boolean,
/**
 * ⏯️ Declares that activating this utility runs a visible, abortable algorithm (tool run contract §2.4);
 * reserves the tool run actions through [`crate::tool_run::RESERVED`].
 */
run?: ToolRunDefinition, };"####,
        },
        SchemaMetadata {
            name: "UtilityRef",
            version: 1,
            typescript: r####"/**
 * 🧰️ A validated reference into an app's `AppDefinition.utilities` registry — the utility mirror of
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
activeToolId?: string,
/**
 * 📌️ Host-owned panel state, opaque to the guest — the ONE long string a view context carries.
 * Contributions are NOT a view-state field: the host publisher installs them by the paged
 * `setContributions` command run and the guest folds them into its own registry.
 */
panelJson?: string,
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
windowInstances: Array<ViewWindowInstance>,
/**
 * 🪟️ Every tree container the host holds state for, flattened over all panel bodies — the ONE
 * source of truth for which containers are open and which rows are on screen. A guest reads them
 * per body and materialises exactly the named windows; it keeps no expansion state of its own,
 * which is what lets an open container survive a refresh.
 */
treeWindows: Array<TreeWindowRequest>,
/**
 * 🪟️ Rows the tallest visible panel body fits, the shared first-paint budget a guest spends in
 * document order over containers the host has not yet seen.
 */
treeViewportRows?: number, };"####,
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
 * 🌅️ The first arm of each new window instance, accepted by this window's utility roster.
 */
initialUtilityId?: UtilityRef,
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
disabled?: boolean, onChange: ActionDescriptor, } | { "kind": "number", id: string, label?: string, value: number,
/**
 * ⬇️ Inclusive floor; `None` leaves the value unbounded below.
 */
min?: number,
/**
 * ⬆️ Inclusive ceiling; `None` leaves the value unbounded above — renderers must not clamp.
 */
max?: number, step?: number,
/**
 * 🎚️ Absolute value already prepared/committed, drawn as a soft extent behind the entry.
 */
ready?: number,
/**
 * 🌀️ When true, the measure tree leaf shows a loading ring while work continues.
 */
loading?: boolean,
/**
 * 🌀️ When true, the measure tree leaf shows a dashed, slower waiting ring; `loading` takes precedence when both are set.
 */
waiting?: boolean,
/**
 * 🚫️ When true, the entry is inert.
 */
disabled?: boolean, onChange: ActionDescriptor, } | { "kind": "toggle", id: string, iconId: IconName, label?: string, pressed: boolean, text?: string, onChange: ActionDescriptor, } | { "kind": "group", id: string, label: string, defaultOpen?: boolean,
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
 * 🧷️ Catalog presentation of the artifact owned by an app.
 */
export type ArtifactPresentation = { id: string, name: string, dimension: string, componentKind: string, };"####,
        },
        SchemaMetadata {
            name: "ComposerEntryDescriptor",
            version: 1,
            typescript: r####"/**
 * 🎹️ One registered composer route; executable composition stays runtime-only.
 */
export type ComposerEntryDescriptor = { writes: ArtifactDialect, reads: Array<ArtifactDialect>, };"####,
        },
        SchemaMetadata {
            name: "ConfigSpec",
            version: 2,
            typescript: r####"/**
 * 🧮️ An app's complete typed configuration declaration: one input per field, its `id` the config key.
 */
export type ConfigSpec = { fields: Array<ActionArgDef>, };"####,
        },
        SchemaMetadata {
            name: "CommandVariantSpec",
            version: 2,
            typescript: r####"/**
 * 🎛️ One keyword and its typed command fields, `required` where the grammar demands the field.
 */
export type CommandVariantSpec = { keyword: string, fields: Array<ActionArgDef>, };"####,
        },
        SchemaMetadata {
            name: "CommandGrammar",
            version: 1,
            typescript: r####"/**
 * 🎛️ An app's complete typed binary command grammar.
 */
export type CommandGrammar = { variants: Array<CommandVariantSpec>, };"####,
        },
        SchemaMetadata {
            name: "FileTypeContribution",
            version: 1,
            typescript: r####"/**
 * 🗂️ One import or export format contributed by an app.
 */
export type FileTypeContribution = { formatKind: string, mediaType: MediaType, imports: boolean, exports: boolean, };"####,
        },
        SchemaMetadata {
            name: "IoEntryDirection",
            version: 1,
            typescript: r####"/**
 * 🚪️ Direction of one registered IO dialect route.
 */
export type IoEntryDirection = "import" | "export";"####,
        },
        SchemaMetadata {
            name: "IoEntryDescriptor",
            version: 1,
            typescript: r####"/**
 * 🚪️ One registered route between owned artifact dialects.
 */
export type IoEntryDescriptor = { owner: ArtifactDialect, counterpart: ArtifactDialect, direction: IoEntryDirection, };"####,
        },
        SchemaMetadata {
            name: "TopicContribution",
            version: 1,
            typescript: r####"/**
 * 🗂️ Open plugin contribution keyed by a dot-namespaced topic.
 */
export type TopicContribution = { topic: string, payload: unknown, };"####,
        },
    ];
