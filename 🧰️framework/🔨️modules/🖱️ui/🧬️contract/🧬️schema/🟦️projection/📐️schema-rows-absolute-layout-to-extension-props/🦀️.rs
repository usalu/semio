//! 📐️ Schema rows from `AbsoluteLayout` to `ExtensionProps`: 18 versioned wire type rows of the owned semantic UI TypeScript projection, in table order. See [`super`].

use super::*;

pub(super) const ABSOLUTE_LAYOUT: SchemaMetadata = SchemaMetadata {
    name: "AbsoluteLayout",
    version: 1,
    typescript: r####"/**
 * 📌️ A freeform positioning context — children carry their own placement outside normal flow.
 */
export type AbsoluteLayout = { sizingWidth: Sizing, sizingHeight: Sizing, };"####,
};

pub(super) const ACCESSIBILITY_SPEC: SchemaMetadata = SchemaMetadata {
    name: "AccessibilitySpec",
    version: 1,
    typescript: r####"/**
 * ♿️ The accessibility intent every node carries once, resolved correctly by every renderer. No
 * `role` field: the semantic role is implied by [`crate::Component`] — a `Component::Button` is a
 * button on every renderer, so naming the role again here would just be a second, driftable source
 * of truth.
 */
export type AccessibilitySpec = { label: Label | null, description: Label | null, live: Liveness, shortcut: string | null, hidden: boolean, };"####,
};

pub(super) const ACTION_BINDING: SchemaMetadata = SchemaMetadata {
    name: "ActionBinding",
    version: 1,
    typescript: r####"/**
 * 🔗️ One node-carried binding from a [`Trigger`] moment to a versioned [`ActionId`]. Replaces every
 * old `on_change`/`action`/`drop_action`/... field scattered across the wgpu target's per-component
 * node structs — a record's `bindings: Vec<ActionBinding>` is the one place any of them now live.
 */
export type ActionBinding = { trigger: Trigger, action: ActionId, args: UiValue | null,
/**
 * 🔐️ An optional capability token a host must hold before this binding is even offered —
 * orthogonal to `args`, which is data the action consumes rather than a permission gate.
 */
capability: string | null, };"####,
};

pub(super) const ACTION_ID: SchemaMetadata = SchemaMetadata {
    name: "ActionId",
    version: 1,
    typescript: r####"/**
 * 🆔️ A versioned action address. `scope` names the controller/domain (the old `ActionDescriptor`'s
 * stringly `controller_id`, e.g. `"cad-play"`, grepped verbatim from the plugin fleet's
 * `ActionFactory::new(CONTROLLER_ID)` call sites), `name` the verb (the old `action`, e.g.
 * `"objectMove"`/`"setValue"`/`"addWidget"`), and `version` is new: it lets a renderer reject or
 * migrate a stale action instead of silently invoking the wrong one — the one axis the old stringly
 * pair never carried.
 */
export type ActionId = { scope: string, name: string, version: number, };"####,
};

pub(super) const ACTIVITY: SchemaMetadata = SchemaMetadata {
    name: "Activity",
    version: 1,
    typescript: r####"/**
 * 🧭️ The activity lifecycle of a node, orthogonal to `disabled`/`transition` — was `UiStatus` on the
 * old wgpu target's `UiPresence`. Lives on the document (`crate::UiNodeRecord::activity`) because it
 * is genuinely part of what the node IS this revision, not an ephemeral input-frequency signal.
 */
export type Activity = "waiting" | "loading" | "idle" | "finished";"####,
};

pub(super) const ALIGN: SchemaMetadata = SchemaMetadata {
    name: "Align",
    version: 1,
    typescript: r####"/**
 * ↕️ Cross-axis alignment — the CSS `align-items` equivalent, `Stretch` default so a node fills its
 * cross axis unless it opts out.
 */
export type Align = "start" | "center" | "end" | "stretch" | "baseline";"####,
};

pub(super) const ANCHOR: SchemaMetadata = SchemaMetadata {
    name: "Anchor",
    version: 1,
    typescript: r####"/**
 * 🧭️ A logical 9-point placement, `Start`/`End` rather than `Left`/`Right` so it stays correct under
 * RTL locales without a renderer-side flip (CLAUDE.md's multi-language accessibility mandate).
 */
export type Anchor = "topStart" | "top" | "topEnd" | "start" | "center" | "end" | "bottomStart" | "bottom" | "bottomEnd";"####,
};

pub(super) const AXIS: SchemaMetadata = SchemaMetadata {
    name: "Axis",
    version: 1,
    typescript: r####"/**
 * ↔️ The main axis a [`StackLayout`] or [`WindowLayoutNode::Split`] lays its children along.
 */
export type Axis = "horizontal" | "vertical";"####,
};

pub(super) const BUILT_NODE: SchemaMetadata = SchemaMetadata {
    name: "BuiltNode",
    version: 1,
    typescript: r####"/**
 * 🧱️ The contract-local shape a builder terminates into — everything a [`crate::UiNodeRecord`]
 * carries except `id` (minted by the runtime at reconciliation, never by an author) and with
 * `children` nested inline rather than addressed by [`crate::UiNodeId`], since a freshly authored
 * tree has no ids yet to address by. Every field below serializes away at its default, mirroring the
 * wire-cost guarantee this whole builder family exists to make automatic.
 */
export type BuiltNode = {
/**
 * 🔑️ The reconciliation key — an author-set [`HasBase::id`] or a positional `"#N"` fallback. See
 * [`HasChildren::child`] for why the fallback is only stable while sibling order is unchanged.
 */
key: string, component: Component, layout: LayoutSpec, style: StyleSpec, activity: Activity, disabled: boolean, accessibility: AccessibilitySpec, bindings: Array<ActionBinding>, menu: MenuRef | null, children: Array<BuiltNode>, };"####,
};

pub(super) const BUTTON_PROPS: SchemaMetadata = SchemaMetadata {
    name: "ButtonProps",
    version: 1,
    typescript: r####"/**
 * 🔘️ Props for `Component::Button`. `action` moved to the record's `bindings` (keyed by
 * `Trigger::Activate`); `style` moved to the record's `crate::StyleSpec`.
 */
export type ButtonProps = {
/**
 * 🖼️ Icon key. The old `IconName` is generated per-consuming-crate via a `#[path]` mount (see
 * `🧰️framework/🔨️modules/🖼️assets/🔣️icons/🤖️generated/🪪️icon-name/🦀️.rs`), not a publishable
 * dependency this crate's `Cargo.toml` — which this packet is forbidden from editing — could
 * take on. A plain `String` icon key is the only viable choice here; flagged as a
 * registrar-request in `📓️terra-contract-doc-report.md` in case a shared icon crate should
 * exist instead.
 */
icon: string, label: Label, };"####,
};

pub(super) const COMPONENT: SchemaMetadata = SchemaMetadata {
    name: "Component",
    version: 1,
    typescript: r####"/**
 * 🧩️ The closed set of things a [`crate::UiNodeRecord`] can render.
 *
 * ⚠️ Unlike the old `UiNode`, no `#[allow(clippy::large_enum_variant)]` is needed: the old
 * `ComponentScene` variant carried up to fifteen `Option<XxxScene>` fields inline, which is exactly
 * the size disparity that lint was suppressing. `Component::Surface` now carries one
 * `crate::SurfaceProps` (a single pack-encoded payload keyed by a `doc_schema` id), so the variants
 * are all comparably small.
 */
export type Component = { "type": "container" } & ContainerProps | { "type": "text" } & TextProps | { "type": "button" } & ButtonProps | { "type": "separator" } & SeparatorProps | { "type": "input" } & InputProps | { "type": "select" } & SelectProps | { "type": "toggle" } & ToggleProps | { "type": "keyValueList" } & KeyValueListProps | { "type": "slider" } & SliderProps | { "type": "numberStepper" } & NumberStepperProps | { "type": "ring" } & RingProps | { "type": "iconSelect" } & IconSelectProps | { "type": "progress" } & ProgressProps | { "type": "tree" } & TreeProps | { "type": "treeSection" } & TreeSectionProps | { "type": "treeItem" } & TreeItemProps | { "type": "image" } & ImageProps | { "type": "surface" } & SurfaceProps | { "type": "extension" } & ExtensionProps | { "type": "table" } & TableProps | { "type": "tableRow" } & TableRowProps;"####,
};

pub(super) const CONTAINER_PROPS: SchemaMetadata = SchemaMetadata {
    name: "ContainerProps",
    version: 1,
    typescript: r####"/**
 * 🌿️ Props for `Component::Container` — the box-with-children component every old `Stack`/
 * `Section`/`Group`/`Field` collapses into (see [`ContainerRole`]). `direction`/`gap`/`padding` do
 * NOT live here — they are `crate::LayoutSpec`, on the record. The old `Field`'s single
 * `child: Box<UiNode>` is simply `children[0]` on the record; there is nothing left to special-case.
 */
export type ContainerProps = { role: ContainerRole, label: Label | null, description: string | null, required: boolean | null, error: string | null, defaultOpen: boolean | null, dropOverlay: DropOverlaySpec | null, };"####,
};

pub(super) const CONTAINER_ROLE: SchemaMetadata = SchemaMetadata {
    name: "ContainerRole",
    version: 1,
    typescript: r####"/**
 * 🌿️ The closed set of roles a [`Component::Container`] plays — collapses the old `Stack`/
 * `Section`/`Group`/`Field` variants (all four were "a box with children plus optional chrome") into
 * one component, distinguished only by role. `Form`/`Toolbar` are new roles with no old-`UiNode`
 * counterpart, added per the packet brief for the layouts those two collapsed variants could not
 * previously express as a single node.
 */
export type ContainerRole = "plain" | "section" | "group" | "field" | "form" | "toolbar";"####,
};

pub(super) const DENSITY: SchemaMetadata = SchemaMetadata {
    name: "Density",
    version: 1,
    typescript: r####"/**
 * 📐️ Named directly from tokens.json's `spacing` table, the only two spacing tokens the styling
 * package actually ships (`compact`, `touch`). `Standard` is the deliberate default occupying the gap
 * between them — no dedicated token for the middle case exists yet.
 */
export type Density = "compact" | "standard" | "touch";"####,
};

pub(super) const DROP_OVERLAY_SPEC: SchemaMetadata = SchemaMetadata {
    name: "DropOverlaySpec",
    version: 1,
    typescript: r####"/**
 * 📥️ Hover-state copy for a [`ContainerProps::drop_overlay`] — shown while a drag is over the
 * container, ahead of its `Drop`-triggered [`crate::ActionBinding`] firing on release.
 */
export type DropOverlaySpec = { title: Label, hint: Label, accept: string | null, };"####,
};

pub(super) const EDGE_SPACE: SchemaMetadata = SchemaMetadata {
    name: "EdgeSpace",
    version: 1,
    typescript: r####"/**
 * 📐️ Per-side padding that costs one [`SpaceToken`] on the wire in the common uniform case, instead
 * of four always-present fields — mirrors CSS shorthand's 1/2/4-value forms.
 */
export type EdgeSpace = { "all": SpaceToken } | { "symmetric": { vertical: SpaceToken, horizontal: SpaceToken, } } | { "each": { top: SpaceToken, right: SpaceToken, bottom: SpaceToken, left: SpaceToken, } };"####,
};

pub(super) const EMPHASIS: SchemaMetadata = SchemaMetadata {
    name: "Emphasis",
    version: 1,
    typescript: r####"/**
 * 🔆️ Visual prominence, orthogonal to [`Variant`] and [`Tone`].
 */
export type Emphasis = "subtle" | "regular" | "strong";"####,
};

pub(super) const EXTENSION_PROPS: SchemaMetadata = SchemaMetadata {
    name: "ExtensionProps",
    version: 1,
    typescript: r####"/**
 * 🧩️ Props for `Component::Extension` — the old `ExternalSlot`. `params_json: String` becomes
 * structured `crate::UiValue`; `plugin_id`/`app_id`/`body_key` collapse into one opaque `extension`
 * address string (the old three-part addressing is a concern of whatever resolves `extension` to a
 * slot, not of this contract).
 */
export type ExtensionProps = { extension: string,
/**
 * ⚠️ Decision: `crate::UiValue` is referenced, not defined, here. The packet brief's explicit
 * "leave unresolved" list (`LayoutSpec`/`StyleSpec`/`AccessibilitySpec`/`ActionBinding`/
 * `MenuRef`/`Activity`/`SurfaceProps`) does not name `UiValue`, but `📋️master.md`'s "1. Contract
 * crate" section places `UiValue` right beside `ActionId`/`Trigger`/`UiIntent` — the action
 * model, owned by packet `contract-action`'s `🎬️action.rs`, not this file. Defining it here
 * risks the exact duplicate-definition collision U2 calls out as worse than an unresolved name.
 */
props: UiValue, };"####,
};
