//! 🎨️ Schema rows from `SliderProps` to `Trigger`: 22 versioned wire type rows of the owned semantic UI TypeScript projection, in table order. See [`super`].

use super::*;

pub(super) const SLIDER_PROPS: SchemaMetadata = SchemaMetadata {
    name: "SliderProps",
    version: 4,
    typescript: r####"/**
 * 🎚️ Props for `Component::Slider`. `on_change` moved to the record's `bindings`. `min`/`max` are the travel;
 * `snaps` are its detents: strictly ascending, finite, inside `min..=max`, at most [`crate::UI_FIXED_LIST_ITEMS`]
 * of them — every renderer paints one tick per snap at its [`slider_axis_position`] and resolves a pointer value
 * through [`slider_pointer_value`] and a key through [`slider_key_value`]. Values are stored values; the readout,
 * the spoken value and a typed value are display values (`stored × display_factor`, at `precision`, beside
 * `display_unit` else `unit`, read back by [`ui_number_typed_value`]). A typed value is refused by `limits` (the
 * travel itself when absent).
 */
export type SliderProps = { value: number, min: number, max: number, step: number, unit: string | null, snaps: Array<number>, appearance?: SliderAppearance, scale?: UiNumberScale, precision?: number | null, displayUnit?: string | null, displayFactor?: number | null, limits?: UiNumberLimits | null, };"####,
};

pub(super) const SPACE_TOKEN: SchemaMetadata = SchemaMetadata {
    name: "SpaceToken",
    version: 1,
    typescript: r####"/**
 * 📐️ Closed spacing scale a renderer resolves against the active theme's spacing ramp — never a raw
 * `f32`/px. tokens.json's `spacing` table today only names `compact`/`touch` (see [`crate::Density`]);
 * no full ramp exists there yet, so this scale is the shape this packet's own brief specifies
 * verbatim (`None,Xs,Sm,Md,Lg,Xl,…`) pending a registrar-added token set — flagged in the packet report.
 */
export type SpaceToken = "none" | "xs" | "sm" | "md" | "lg" | "xl" | "xxl";"####,
};

pub(super) const STACK_LAYOUT: SchemaMetadata = SchemaMetadata {
    name: "StackLayout",
    version: 1,
    typescript: r####"/**
 * 📚️ A one-axis flex-like arrangement — expressible by CSS flex, a taffy tree, or a native stack.
 */
export type StackLayout = { axis: Axis, gap: SpaceToken, padding: EdgeSpace, align: Align, justify: Justify, grow: boolean, wrap: boolean, };"####,
};

pub(super) const STYLE_SPEC: SchemaMetadata = SchemaMetadata {
    name: "StyleSpec",
    version: 1,
    typescript: r####"/**
 * 🎨️ A node's design-token styling — five closed enums, never a raw color or a raw pixel value. A
 * renderer resolves every field against the active theme; this struct only names the tokens. Each
 * field is omitted from the wire at its default, so a default-styled node costs nothing to encode.
 */
export type StyleSpec = { variant: Variant, size: SizeToken, density: Density, tone: Tone, emphasis: Emphasis, };"####,
};

pub(super) const SURFACE_DOC: SchemaMetadata = SchemaMetadata {
    name: "SurfaceDoc",
    version: 1,
    typescript: r####"/**
 * 📦️ An opaque, pack-encoded payload. The contract never parses it — `doc_schema` on the owning
 * [`SurfaceProps`] names the version-specific shape (e.g. `"world3d@1"`) that some other layer (the
 * `🎬️scene` crate) knows how to decode.
 */
export type SurfaceDoc = { bytes: Array<number>, };"####,
};

pub(super) const SURFACE_ID: SchemaMetadata = SchemaMetadata {
    name: "SurfaceId",
    version: 1,
    typescript: r####"/**
 * 🪧️ A render surface address — today's dotted strings, e.g. `"note.play.navigator"`.
 */
export type SurfaceId = string;"####,
};

pub(super) const SURFACE_KIND: SchemaMetadata = SchemaMetadata {
    name: "SurfaceKind",
    version: 1,
    typescript: r####"/**
 * 🖼️ The 15 embeddable product surface kinds, every tag kebab-case. `VirtualFileSystem` used to be
 * the one camelCase holdout (`"virtualFileSystem"`); the rename reached the contract, the wgpu
 * target's own `SurfaceKind` and this projection, so there is exactly ONE spelling repo-wide.
 */
export type SurfaceKind = "canvas-2d" | "world-3d" | "node-graph" | "text-editor" | "table" | "paint-2d" | "virtual-file-system" | "tiled-map" | "board-2d" | "icon-render" | "ink-canvas" | "graph-timeline" | "block-list" | "diff-view" | "event-feed";"####,
};

pub(super) const SURFACE_PROPS: SchemaMetadata = SchemaMetadata {
    name: "SurfaceProps",
    version: 1,
    typescript: r####"/**
 * 🗺️ An embedded product surface. Replaces the old `UiComponentSceneNode`'s 15 sparse
 * `Option<XxxScene>` fields with exactly ONE payload, identified by `doc_schema` — the 15 product
 * scene structs themselves stay product payloads and move to `🖱️ui/🎬️scene/🦀️.rs` in a
 * later packet, never into this dependency-free contract crate. See this file's own module doc for
 * the exact reasoning behind each field (and each field the scaffold this replaces used to carry but
 * no longer does).
 */
export type SurfaceProps = { kind: SurfaceKind,
/**
 * 🏷️ `"<kind>@<version>"`, e.g. `"world3d@1"` — the axis a renderer gates its own per-kind decode
 * logic on. Never validated against `kind` by this crate (see [`parse_doc_schema`]); a mismatch
 * between the two is a `🎬️scene`-crate-level authoring bug, not a contract violation.
 */
docSchema: string, doc: SurfaceDoc,
/**
 * 🔗️ Surface-level intents — bindings that fire against the surface itself (e.g. a "focus"/
 * "reset view" action a host chrome offers around the embedded content), as opposed to intents the
 * embedded content's own scene graph interprets internally via `doc`'s opaque bytes.
 */
bindings: Array<ActionBinding>, };"####,
};

pub(super) const TABLE_PROPS: SchemaMetadata = SchemaMetadata {
    name: "TableProps",
    version: 1,
    typescript: r####"/**
 * 📊️ Props for `Component::Table` — a column-headed data table. The header lives HERE, as props, and
 * every [`TableRowProps`] child is one row, so a table costs one node record plus one per MATERIALISED
 * row however many columns and row actions it has. `window` is the same [`TreeWindow`] contract a tree
 * section carries: the children are the rows `[offset, offset + children.len())` of a logically
 * `total`-long row list, a renderer pitches the unmaterialised rows as spacers and asks for the rows its
 * viewport shows through `ViewModel::tree_windows` — one windowing mechanism and one body-wide node
 * ledger ([`TREE_WINDOW_BODY_NODE_BUDGET`]) for trees and tables alike.
 */
export type TableProps = {
/**
 * 🏷️ The table's accessible name — what assistive technology announces on entering it.
 */
label: Label,
/**
 * 🗂️ The column headers, in cell order.
 */
columns: Array<Label>,
/**
 * ↕️ Artifact-localized name of one logical row.
 */
rowLabel: Label | null,
/**
 * ↔️ Artifact-localized name of one logical column.
 */
columnLabel: Label | null,
/**
 * 🎬️ Header of the trailing actions column a renderer adds when any row carries row actions.
 */
actionsLabel: Label | null,
/**
 * 🪟️ The materialised slice of the logical row list — see [`TreeWindow`].
 */
window: TreeWindow | null,
/**
 * ↔️ The materialised slice of the logical column list.
 */
columnWindow: TreeWindow | null, };"####,
};

pub(super) const TABLE_ROW_PROPS: SchemaMetadata = SchemaMetadata {
    name: "TableRowProps",
    version: 2,
    typescript: r####"/**
 * 📊️ Props for `Component::TableRow` — one row of a [`TableProps`] table: its cells in column order, its
 * row-scoped actions and its target, all as props, so a row is ONE node record. Actions and activation
 * are verbs on the row's ONE [`RowTarget`], exactly as a [`TreeItemProps`] row's are.
 */
export type TableRowProps = {
/**
 * 📝️ The row's cells, positional to the table's `columns`.
 */
cells: Array<string>,
/**
 * 🎬️ Row-scoped actions, rendered in the table's trailing actions column.
 */
rowActions: Array<RowAction>,
/**
 * 🎯️ The one target `row_actions` and the activation fire on — present exactly when either is.
 */
target: RowTarget | null, };"####,
};

pub(super) const TEXT_PROPS: SchemaMetadata = SchemaMetadata {
    name: "TextProps",
    version: 1,
    typescript: r####"/**
 * 📝️ Props for `Component::Text`.
 */
export type TextProps = { value: Label, emphasize: boolean | null, dataAttributes: { [key in string]?: string } | null, };"####,
};

pub(super) const TOGGLE_APPEARANCE: SchemaMetadata = SchemaMetadata {
    name: "ToggleAppearance",
    version: 1,
    typescript: r####"/** ☑️ Binary-control presentation. */
export type ToggleAppearance = "button" | "checkbox";"####,
};

pub(super) const TOGGLE_PROPS: SchemaMetadata = SchemaMetadata {
    name: "ToggleProps",
    version: 1,
    typescript: r####"/**
 * 🔀️ Props for `Component::Toggle`. `on` is the explicit state this contract adds — the old
 * `UiToggleNode` smuggled it through `presence.selected`, exactly the implicit coupling this
 * contract exists to remove. `on_change` moved to the record's `bindings`.
 */
export type ToggleProps = { appearance?: ToggleAppearance, on: boolean, icon: string, text: Label | null, };"####,
};

pub(super) const TONE: SchemaMetadata = SchemaMetadata {
    name: "Tone",
    version: 1,
    typescript: r####"/**
 * 🎨️ The semantic color role a renderer resolves against the active theme — named after
 * tokens.json's `colors` table's semantic entries (`primary`, `secondary`, `tertiary`, `danger`,
 * `warning`, `info`, `success`). `Neutral` is the default: no explicit accent, inherit the
 * surrounding surface/text color.
 */
export type Tone = "neutral" | "primary" | "secondary" | "tertiary" | "info" | "success" | "warning" | "danger";"####,
};

pub(super) const TRANSITION_HINT: SchemaMetadata = SchemaMetadata {
    name: "TransitionHint",
    version: 1,
    typescript: r####"/**
 * 🎞️ The transient visual emphasis a node is entering — orthogonal to `activity`/`disabled`. A node
 * carrying neither is in its steady state; the renderer clears this once the transition has played.
 */
export type TransitionHint = "introducing" | "celebrating";"####,
};

pub(super) const TREE_ITEM_PROPS: SchemaMetadata = SchemaMetadata {
    name: "TreeItemProps",
    version: 3,
    typescript: r####"/**
 * 🌿️ Props for `Component::TreeItem` — a single row. `items`/`control` are gone: nested items and
 * the old inline `control: Option<UiControlNode>` are now ordinary children on the record (the
 * `UiControlNode` enum does not get ported — every one of its old variants is already a
 * [`Component`] variant in its own right, so a control-as-child-node needs no separate wrapper
 * type). `inlineToolbar` identifies the one direct horizontal Toolbar child whose real Button
 * children remain independently focusable and actionable inside the row. `detail` identifies one
 * direct Surface child placed in a bounded band below the row. The row's primary click
 * action (old `action: Option<ActionDescriptor>`) is [`RowTarget::activation`], a verb on the
 * row's target — never a record `Trigger::Activate` binding.
 */
export type TreeItemProps = { label: Label, description: string | null, icon: string | null, defaultOpen: boolean | null, draggable: boolean | null, dragData: { [key in string]?: string } | null,
/**
 * 👁️ Domain "eye toggle": the row stays visible, dimmed, and clickable (to un-hide). NOT the
 * same axis as the record's `activity`/`disabled` — a dimmed row is still fully interactive.
 */
dimmed: boolean | null,
/**
 * ☑️ The row's choice state when it is one option of a single-choice list: `Some(true)` is the chosen option, `Some(false)` a
 * choosable one that is not chosen, `None` not an option row. Every renderer exposes it as the row's selected state (React
 * `aria-selected`, the wgpu ARIA mirror's `selected`) and paints the chosen row selected — never by its icon alone.
 */
selected: boolean | null,
/**
 * 🪟️ The materialised slice of this row's logical child list — see [`TreeWindow`].
 */
window: TreeWindow | null,
/**
 * 🎯️ The interaction granularity this row picks when the tree carries the domain binding: the
 * row is a pick target of the tree's `interaction_domain`, keyed by its own record key, so it
 * needs no argument map and no per-row binding of its own.
 */
granularity: string | null,
/**
 * 🎛️ A direct `ContainerRole::Toolbar` child laid out horizontally inside this row. The
 * document validator requires the target to be one of this record's direct children and every
 * toolbar child to be a real `Component::Button`.
 */
inlineToolbar: UiNodeId | null,
/**
 * 🎞️ A direct Surface child placed in a bounded detail band below this row. It is neither an
 * inline control nor a nested TreeItem, and remains a real independently hosted scene.
 */
detail: UiNodeId | null, rowActions: Array<RowAction>,
/**
 * 🎯️ The one target `row_actions` and the activation fire on — present exactly when either is.
 */
target: RowTarget | null, };"####,
};

pub(super) const TREE_PRESENTATION: SchemaMetadata = SchemaMetadata {
    name: "TreePresentation",
    version: 1,
    typescript: r####"/** 🌳️ Inherited tree row presentation. */
export type TreePresentation = "standard" | "compact";"####,
};

pub(super) const TREE_PROPS: SchemaMetadata = SchemaMetadata {
    name: "TreeProps",
    version: 1,
    typescript: r####"/**
 * 🌲️ Props for `Component::Tree` — the tree's own binding, nothing else. Sections and items are no
 * longer inline (`sections: Vec<UiTreeSectionNode>`); they are ordinary child nodes
 * (`Component::TreeSection` / `Component::TreeItem`) reached through the record's `children`.
 * `drop_action` moved to the record's `bindings` (`Trigger::Drop`).
 */
export type TreeProps = { presentation?: TreePresentation,
/**
 * 🕹️ Binds this tree to an app-declared `InteractionDefinition` domain — selection/hover for
 * bound items is owned by the framework's presence channel, not by per-item props.
 */
interactionDomain: string | null, };"####,
};

pub(super) const TREE_SECTION_PROPS: SchemaMetadata = SchemaMetadata {
    name: "TreeSectionProps",
    version: 1,
    typescript: r####"/**
 * 🌲️ Props for `Component::TreeSection` — a labeled, collapsible grouping of `TreeItem` children.
 */
export type TreeSectionProps = { label: Label | null, defaultOpen: boolean | null, headerToolbar: UiNodeId | null,
/**
 * 🪟️ The materialised slice of this section's logical child list — see [`TreeWindow`].
 */
window: TreeWindow | null, };"####,
};

pub(super) const TREE_WINDOW: SchemaMetadata = SchemaMetadata {
    name: "TreeWindow",
    version: 1,
    typescript: r####"/**
 * 🪟️ The materialised slice of a logically `total`-long child list: the record's `children` are the
 * entries `[offset, offset + children.len())` of that list. `total > 0` with no materialised
 * children means expandable-but-not-yet-loaded, never "empty"; a renderer pitches the unmaterialised
 * rows as spacers so the scrollbar spans the whole document instead of the loaded window.
 */
export type TreeWindow = { total: number, offset: number, rowExtent: TreeWindowRowExtent, };"####,
};

pub(super) const TREE_WINDOW_ROW_EXTENT: SchemaMetadata = SchemaMetadata {
    name: "TreeWindowRowExtent",
    version: 1,
    typescript: r####"/** 🪟️ Closed geometry token for one unmaterialised row in a virtual Tree window. */
export type TreeWindowRowExtent = "standard" | "compactText" | "compactSmallControl" | "compactControl";"####,
};

pub(super) const TRIGGER: SchemaMetadata = SchemaMetadata {
    name: "Trigger",
    version: 1,
    typescript: r####"/**
 * 🎯️ The lifecycle moment on a node that fires an [`ActionBinding`] — replaces the old single
 * implicit "the" action every node carried with a closed, named set, so one node can bind several
 * distinct moments (e.g. `Change` while typing, `Commit` on blur) without inventing parallel fields.
 */
export type Trigger = "activate" | "change" | "commit" | "delta" | "drop" | "submit" | "abort" | "repeatLast" | "hoverPreview";"####,
};
