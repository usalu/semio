//! @emoji 🧬️ The semantic UI contract — the single language-neutral boundary between the headless UI
//! runtime and every renderer (React DOM, the custom GPU family, anything later).
//!
//! Three properties define this crate:
//!
//! 1. **Flat, not recursive.** A [`UiSnapshot`] is an id-keyed table of [`UiNodeRecord`]s, never a
//!    nested tree. That is what lets one patch address one node, and what makes the whole surface
//!    schema-projectable — the owned versioned metadata keeps recursive wire types explicit.
//! 2. **Synchronous.** Validation and patch application are run-to-completion transactions with no
//!    suspension point — see ruling U1 in ticket `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`.
//! 3. **Dependency-free.** serde and the styling tokens, nothing else. No engine, no `wgpu`, no
//!    `winit`, no actor kernel, no os-kernel `dsl` — so this compiles for `wasm32-wasip2` guests and
//!    `wasm32-unknown-unknown` browsers by construction, and a CI `cargo tree` assertion keeps it so.

//#region 🧬️SchemaMetadata
#[cfg(feature = "typegen")]
pub mod schema_metadata {
    use std::collections::HashSet;

    /// 🧬️ One versioned semantic UI wire type and its owned TypeScript projection.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct SchemaMetadata {
        pub name: &'static str,
        pub version: u16,
        pub typescript: &'static str,
    }

    pub const TYPES: &[SchemaMetadata] = &[
        SchemaMetadata {
            name: "AbsoluteLayout",
            version: 1,
            typescript: r####"/**
 * 📌️ A freeform positioning context — children carry their own placement outside normal flow.
 */
export type AbsoluteLayout = { sizingWidth: Sizing, sizingHeight: Sizing, };"####,
        },
        SchemaMetadata {
            name: "AccessibilitySpec",
            version: 1,
            typescript: r####"/**
 * ♿️ The accessibility intent every node carries once, resolved correctly by every renderer. No
 * `role` field: the semantic role is implied by [`crate::Component`] — a `Component::Button` is a
 * button on every renderer, so naming the role again here would just be a second, driftable source
 * of truth.
 */
export type AccessibilitySpec = { label: Label | null, description: Label | null, live: Liveness, shortcut: string | null, hidden: boolean, };"####,
        },
        SchemaMetadata {
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
        },
        SchemaMetadata {
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
        },
        SchemaMetadata {
            name: "Activity",
            version: 1,
            typescript: r####"/**
 * 🧭️ The activity lifecycle of a node, orthogonal to `disabled`/`transition` — was `UiStatus` on the
 * old wgpu target's `UiPresence`. Lives on the document (`crate::UiNodeRecord::activity`) because it
 * is genuinely part of what the node IS this revision, not an ephemeral input-frequency signal.
 */
export type Activity = "waiting" | "loading" | "idle" | "finished";"####,
        },
        SchemaMetadata {
            name: "Align",
            version: 1,
            typescript: r####"/**
 * ↕️ Cross-axis alignment — the CSS `align-items` equivalent, `Stretch` default so a node fills its
 * cross axis unless it opts out.
 */
export type Align = "start" | "center" | "end" | "stretch" | "baseline";"####,
        },
        SchemaMetadata {
            name: "Anchor",
            version: 1,
            typescript: r####"/**
 * 🧭️ A logical 9-point placement, `Start`/`End` rather than `Left`/`Right` so it stays correct under
 * RTL locales without a renderer-side flip (CLAUDE.md's multi-language accessibility mandate).
 */
export type Anchor = "topStart" | "top" | "topEnd" | "start" | "center" | "end" | "bottomStart" | "bottom" | "bottomEnd";"####,
        },
        SchemaMetadata {
            name: "Axis",
            version: 1,
            typescript: r####"/**
 * ↔️ The main axis a [`StackLayout`] or [`WindowLayoutNode::Split`] lays its children along.
 */
export type Axis = "horizontal" | "vertical";"####,
        },
        SchemaMetadata {
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
        },
        SchemaMetadata {
            name: "ButtonProps",
            version: 1,
            typescript: r####"/**
 * 🔘️ Props for `Component::Button`. `action` moved to the record's `bindings` (keyed by
 * `Trigger::Activate`); `style` moved to the record's `crate::StyleSpec`.
 */
export type ButtonProps = {
/**
 * 🖼️ Icon key. The old `IconName` is generated per-consuming-crate via a `#[path]` mount (see
 * `🧰️framework/🔨️modules/🖱️ui/🖼️assets/🔣️icons/🤖️generated/🦀️icon_name.rs`), not a publishable
 * dependency this crate's `Cargo.toml` — which this packet is forbidden from editing — could
 * take on. A plain `String` icon key is the only viable choice here; flagged as a
 * registrar-request in `📓️terra-contract-doc-report.md` in case a shared icon crate should
 * exist instead.
 */
icon: string, label: Label, };"####,
        },
        SchemaMetadata {
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
export type Component = { "type": "container" } & ContainerProps | { "type": "text" } & TextProps | { "type": "button" } & ButtonProps | { "type": "separator" } & SeparatorProps | { "type": "input" } & InputProps | { "type": "select" } & SelectProps | { "type": "toggle" } & ToggleProps | { "type": "keyValueList" } & KeyValueListProps | { "type": "slider" } & SliderProps | { "type": "numberStepper" } & NumberStepperProps | { "type": "ring" } & RingProps | { "type": "iconSelect" } & IconSelectProps | { "type": "tree" } & TreeProps | { "type": "treeSection" } & TreeSectionProps | { "type": "treeItem" } & TreeItemProps | { "type": "image" } & ImageProps | { "type": "surface" } & SurfaceProps | { "type": "extension" } & ExtensionProps;"####,
        },
        SchemaMetadata {
            name: "ContainerProps",
            version: 1,
            typescript: r####"/**
 * 🌿️ Props for `Component::Container` — the box-with-children component every old `Stack`/
 * `Section`/`Group`/`Field` collapses into (see [`ContainerRole`]). `direction`/`gap`/`padding` do
 * NOT live here — they are `crate::LayoutSpec`, on the record. The old `Field`'s single
 * `child: Box<UiNode>` is simply `children[0]` on the record; there is nothing left to special-case.
 */
export type ContainerProps = { role: ContainerRole, label: Label | null, description: string | null, required: boolean | null, error: string | null, defaultOpen: boolean | null, dropOverlay: DropOverlaySpec | null, };"####,
        },
        SchemaMetadata {
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
        },
        SchemaMetadata {
            name: "Density",
            version: 1,
            typescript: r####"/**
 * 📐️ Named directly from tokens.json's `spacing` table, the only two spacing tokens the styling
 * package actually ships (`compact`, `touch`). `Standard` is the deliberate default occupying the gap
 * between them — no dedicated token for the middle case exists yet.
 */
export type Density = "compact" | "standard" | "touch";"####,
        },
        SchemaMetadata {
            name: "DropOverlaySpec",
            version: 1,
            typescript: r####"/**
 * 📥️ Hover-state copy for a [`ContainerProps::drop_overlay`] — shown while a drag is over the
 * container, ahead of its `Drop`-triggered [`crate::ActionBinding`] firing on release.
 */
export type DropOverlaySpec = { title: Label, hint: Label, accept: string | null, };"####,
        },
        SchemaMetadata {
            name: "EdgeSpace",
            version: 1,
            typescript: r####"/**
 * 📐️ Per-side padding that costs one [`SpaceToken`] on the wire in the common uniform case, instead
 * of four always-present fields — mirrors CSS shorthand's 1/2/4-value forms.
 */
export type EdgeSpace = { "all": SpaceToken } | { "symmetric": { vertical: SpaceToken, horizontal: SpaceToken, } } | { "each": { top: SpaceToken, right: SpaceToken, bottom: SpaceToken, left: SpaceToken, } };"####,
        },
        SchemaMetadata {
            name: "Emphasis",
            version: 1,
            typescript: r####"/**
 * 🔆️ Visual prominence, orthogonal to [`Variant`] and [`Tone`].
 */
export type Emphasis = "subtle" | "regular" | "strong";"####,
        },
        SchemaMetadata {
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
 * model, owned by packet `contract-action`'s `🦀️action.rs`, not this file. Defining it here
 * risks the exact duplicate-definition collision U2 calls out as worse than an unresolved name.
 */
props: UiValue, };"####,
        },
        SchemaMetadata {
            name: "GridLayout",
            version: 1,
            typescript: r####"/**
 * 🔲️ A two-dimensional track arrangement — expressible by CSS grid or a taffy grid tree.
 */
export type GridLayout = { columns: Array<GridTrack>, rows: Array<GridTrack>, columnGap: SpaceToken, rowGap: SpaceToken, padding: EdgeSpace, align: Align, justify: Justify, };"####,
        },
        SchemaMetadata {
            name: "GridTrack",
            version: 1,
            typescript: r####"/**
 * 🔲️ One grid track's sizing rule — `Fraction` is a proportion count, never a pixel width.
 */
export type GridTrack = "auto" | { "fraction": number } | { "fixed": SpaceToken } | "minContent" | "maxContent";"####,
        },
        SchemaMetadata {
            name: "IconSelectProps",
            version: 1,
            typescript: r####"/**
 * 🖼️ Props for `Component::IconSelect`. `on_change` moved to the record's `bindings`.
 */
export type IconSelectProps = { value: string, uniform: boolean, classifierKind: string, };"####,
        },
        SchemaMetadata {
            name: "ImageProps",
            version: 1,
            typescript: r####"/**
 * 🖼️ Props for `Component::Image`.
 */
export type ImageProps = { src: string, alt: Label | null, };"####,
        },
        SchemaMetadata {
            name: "InputKind",
            version: 1,
            typescript: r####"/**
 * ⌨️ The closed set of input kinds. Grepped from the fleet's actual `UiInputNode.input_kind` string
 * literals (`"text"`, `"textarea"`, `"number"`, `"file"`) plus the values the React `Interpreter`
 * already branches on for `inputKind` (`"number"`, `"longText"`, `"date"`, `"color"`, `"file"`,
 * default `"text"`) — the union of both sides, since the renderer supports more kinds than any
 * current plugin emits yet. Note the old Rust/TS spelling mismatch this closes: Rust emitted
 * `"textarea"`, TS checked `"longText"`; this enum has exactly one spelling (`LongText`) for that
 * kind. No `Search` kind exists anywhere in the fleet today, so it is not included (see the
 * packet's own report for the grep evidence — adding it back is a one-variant change, not a design
 * change, if a plugin needs it later).
 */
export type InputKind = "text" | "longText" | "number" | "date" | "color" | "file";"####,
        },
        SchemaMetadata {
            name: "InputProps",
            version: 1,
            typescript: r####"/**
 * ⌨️ Props for `Component::Input`. `on_change` moved to the record's `bindings`
 * (`Trigger::Change`/`Trigger::Commit`).
 */
export type InputProps = { kind: InputKind, value: string, placeholder: Label | null,
/**
 * 🫳️ Commit convention string carried verbatim from the old wire shape (e.g. `"blur"`) — no
 * closed set of these was found in the fleet, unlike `input_kind`.
 */
commit: string | null, min: number | null, max: number | null, step: number | null, accept: string | null, };"####,
        },
        SchemaMetadata {
            name: "Justify",
            version: 1,
            typescript: r####"/**
 * ↔️ Main-axis distribution — the CSS `justify-content` equivalent.
 */
export type Justify = "start" | "center" | "end" | "spaceBetween" | "spaceAround" | "spaceEvenly";"####,
        },
        SchemaMetadata {
            name: "KeyValueEntry",
            version: 1,
            typescript: r####"/**
 * 🗝️ One row of a [`Component::KeyValueList`].
 */
export type KeyValueEntry = { label: Label, value: string, };"####,
        },
        SchemaMetadata {
            name: "KeyValueListProps",
            version: 1,
            typescript: r####"/**
 * 🗝️ Props for `Component::KeyValueList`.
 */
export type KeyValueListProps = { entries: Array<KeyValueEntry>, };"####,
        },
        SchemaMetadata {
            name: "Label",
            version: 1,
            typescript: r####"/**
 * 🏷️ Display-ready UI text carried on the wire.
 *
 * ⚠️ Decision (flagged per packet brief): the old `UiNode`'s `Label` (`crate::wgpu::Label` in
 * `🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️label.rs`) is NOT reused here. It
 * is defined inside the old wgpu-target UI package, imports that package's own `Locale`/
 * `Terminology` axes, and its whole point (`From<LabelText>` wired to the `app_labels!` macro,
 * no `From<&str>`) is a compile-time-checked-label enforcement mechanism that lives at the
 * authoring boundary, not the wire boundary — and this crate must not depend on that package at
 * all (no engine, no wgpu; see `📦️glue.rs`). This is therefore a minimal, independent transparent
 * string. The localization/terminology resolution that used to happen via `LabelText::fill` still
 * happens upstream of the runtime (manifest/host), before a `Label` ever reaches this contract.
 */
export type Label = string;"####,
        },
        SchemaMetadata {
            name: "LayoutSpec",
            version: 1,
            typescript: r####"/**
 * 🧬️ The renderer-neutral layout vocabulary a [`crate::UiNodeRecord`] carries — expressible by CSS
 * flex/grid, by a taffy tree, and by native stacks alike. No CSS strings, no taffy types, no pixel
 * geometry: every metric is a closed enum over [`SpaceToken`].
 */
export type LayoutSpec = { "kind": "leaf" } & LeafLayout | { "kind": "stack" } & StackLayout | { "kind": "grid" } & GridLayout | { "kind": "overlay" } & OverlayLayout | { "kind": "scroll" } & ScrollLayout | { "kind": "absolute" } & AbsoluteLayout;"####,
        },
        SchemaMetadata {
            name: "LeafLayout",
            version: 1,
            typescript: r####"/**
 * 🍃️ A childless terminal node's own box sizing — text, image, and other atomic components.
 */
export type LeafLayout = { width: Sizing, height: Sizing, };"####,
        },
        SchemaMetadata {
            name: "Liveness",
            version: 1,
            typescript: r####"/**
 * 📢️ An ARIA-live-region politeness level, translated by each renderer into its own live-announce
 * mechanism (DOM `aria-live`, the GPU renderer's accessibility snapshot, ...).
 */
export type Liveness = "off" | "polite" | "assertive";"####,
        },
        SchemaMetadata {
            name: "MenuRef",
            version: 1,
            typescript: r####"/**
 * 📋️ A reference to a resolved context menu — replaces the old `UiMenuRef`'s `DslValue` args with
 * the crate-neutral [`UiValue`].
 */
export type MenuRef = { id: string, args: UiValue | null, };"####,
        },
        SchemaMetadata {
            name: "NumberStepperProps",
            version: 1,
            typescript: r####"/**
 * 🔢️ Props for `Component::NumberStepper`. `on_absolute`/`on_delta` both moved to the record's
 * `bindings`, distinguished by `Trigger`.
 */
export type NumberStepperProps = { value: number, step: number, uniform: boolean, };"####,
        },
        SchemaMetadata {
            name: "OverlayLayout",
            version: 1,
            typescript: r####"/**
 * 🪟️ A positioning context whose children stack on top of one another anchored to the box —
 * modals, popovers, tooltips.
 */
export type OverlayLayout = { anchor: Anchor, inset: EdgeSpace, dismissible: boolean, };"####,
        },
        SchemaMetadata {
            name: "OwnPresence",
            version: 1,
            typescript: r####"/**
 * 🙋️ This session's own hover/selection/preview state and palette color on a node — the local half
 * of the presence channel; every OTHER session's equivalent arrives as a [`PeerMark`] in `peers`.
 */
export type OwnPresence = { hovered: boolean, selected: boolean,
/**
 * 👁️ Mid-drag or mid-hover-preview emphasis distinct from `hovered` — e.g. previewing a drop
 * target before release, or a `Trigger::HoverPreview` binding's target while armed.
 */
previewed: boolean, color: number | null, };"####,
        },
        SchemaMetadata {
            name: "PatchRejection",
            version: 1,
            typescript: r####"/**
 * 🚫️ Why [`apply_patch`] rejected a [`crate::UiPatch`] — carries enough detail (both revisions, the
 * exceeded quota with its actual/max, or the full violation list) for the existing `patch-rejected`
 * wire event to explain itself and for the sender to resynchronise.
 */
export type PatchRejection = { "type": "revisionMismatch", expected: UiRevision, actual: UiRevision, } | { "type": "unknownNode", id: UiNodeId, } | { "type": "quotaExceeded", quota: QuotaKind, actual: number, max: number, } | { "type": "invariantViolated", violations: Array<UiContractViolation>, };"####,
        },
        SchemaMetadata {
            name: "PeerMark",
            version: 1,
            typescript: r####"/**
 * 👥️ One OTHER peer's mark on a node — hover/selection dot plus initials chip. Ported faithfully
 * from the old wgpu target's `UiPeerMark` (contract-freeze §C7.6, ticket 26/08/17/SHARED-PRESENCE-
 * SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION) — `label` is still the actor id's display form, not
 * a free-text caption.
 */
export type PeerMark = { actor: string, color: number | null, hovered: boolean, selected: boolean, label: string, };"####,
        },
        SchemaMetadata {
            name: "PresenceUpdate",
            version: 1,
            typescript: r####"/**
 * 📡️ One coalesced, TTL-scoped update on the presence channel, keyed `(surface, node_key)` — never
 * carried by [`crate::UiSnapshot`]/[`crate::UiPatch`] themselves. A receiver clears a peer's mark once
 * `ttl_ms` has elapsed without a fresh `PresenceUpdate` for that key, so a disconnected peer fades out
 * on a timer instead of leaving a stuck mark. Replaces the old `ui_tree_stamp_presence`, which
 * mutated hover/selection/color/peers directly onto tree nodes.
 */
export type PresenceUpdate = { surface: SurfaceId,
/**
 * 🔑️ [`crate::UiNodeRecord::key`], not [`crate::UiNodeId`] — presence must still land on the
 * right element across a reconciliation that reassigns ids but keeps keys stable.
 */
nodeKey: string, own: OwnPresence, peers: Array<PeerMark>, ttlMs: number, };"####,
        },
        SchemaMetadata {
            name: "QuotaKind",
            version: 1,
            typescript: r####"/**
 * 🛡️ Which [`UiDocumentLimits`] field a [`PatchRejection::QuotaExceeded`] names — only the four
 * per-patch quotas `apply_patch` itself enforces directly; `max_nodes`/`max_depth` surface instead as
 * [`UiContractViolation::NodeQuota`]/[`UiContractViolation::DepthQuota`] inside
 * [`PatchRejection::InvariantViolated`], since those are whole-document shape properties only knowable
 * after the draft is built.
 */
export type QuotaKind = "children" | "textBytes" | "patchOps" | "patchBytes";"####,
        },
        SchemaMetadata {
            name: "RingProps",
            version: 1,
            typescript: r####"/**
 * 💍️ Props for `Component::Ring`. `on_change` moved to the record's `bindings`.
 */
export type RingProps = { orbId: string, t: number, };"####,
        },
        SchemaMetadata {
            name: "RowAction",
            version: 1,
            typescript: r####"/**
 * 🎬️ One action affordance painted on (or reachable from) a [`Component::TreeItem`] row —
 * `action` reuses [`crate::ActionBinding`] rather than a second parallel action-id type, since a row
 * action is exactly a binding fired unconditionally on click (no `Trigger` ambiguity to add here).
 */
export type RowAction = {
/**
 * 🖼️ Icon key. See [`ButtonProps::icon`] for why this is a plain `String`, not a closed enum.
 */
icon: string, label: Label | null, action: ActionBinding, placement: RowActionPlacement, };"####,
        },
        SchemaMetadata {
            name: "RowActionPlacement",
            version: 1,
            typescript: r####"/**
 * 📍️ Where a [`RowAction`] paints: on the tree row itself, or folded into the row's context menu.
 */
export type RowActionPlacement = "row" | "menu";"####,
        },
        SchemaMetadata {
            name: "ScrollAxes",
            version: 1,
            typescript: r####"/**
 * 🖱️ Which axes a [`ScrollLayout`] permits overflow scrolling on.
 */
export type ScrollAxes = "none" | "horizontal" | "vertical" | "both";"####,
        },
        SchemaMetadata {
            name: "ScrollLayout",
            version: 1,
            typescript: r####"/**
 * 🖱️ A viewport clipping its content and permitting overflow scroll on the named axes.
 */
export type ScrollLayout = { axes: ScrollAxes, padding: EdgeSpace, sizing: Sizing, };"####,
        },
        SchemaMetadata {
            name: "SelectItem",
            version: 1,
            typescript: r####"/**
 * 🔽️ One option of a [`Component::Select`].
 */
export type SelectItem = { value: string, label: Label, };"####,
        },
        SchemaMetadata {
            name: "SelectProps",
            version: 1,
            typescript: r####"/**
 * 🔽️ Props for `Component::Select`. `on_change` moved to the record's `bindings`.
 */
export type SelectProps = { value: string, items: Array<SelectItem>, placeholder: Label | null, };"####,
        },
        SchemaMetadata {
            name: "SeparatorProps",
            version: 1,
            typescript: r####"/**
 * ➖️ Props for `Component::Separator`. Every field the old `UiSeparatorNode` carried
 * (`presence`, `menu`) now lives on the record, so this is intentionally empty — kept as its own
 * struct (rather than a unit variant) purely for structural symmetry with every other component.
 */
export type SeparatorProps = Record<string, never>;"####,
        },
        SchemaMetadata {
            name: "SizeToken",
            version: 1,
            typescript: r####"/**
 * 📏️ A component's t-shirt size. Mirrors the one real precedent in the wgpu target's old
 * `StyleSpec.size` (`"md"`); no dedicated component-size ramp exists in tokens.json yet — see the
 * packet report.
 */
export type SizeToken = "xs" | "sm" | "md" | "lg" | "xl";"####,
        },
        SchemaMetadata {
            name: "Sizing",
            version: 1,
            typescript: r####"/**
 * 📏️ How a node sizes itself along one axis relative to its parent's flow — `Fixed` still names a
 * [`SpaceToken`], never a pixel value.
 */
export type Sizing = "hug" | "fill" | { "fixed": SpaceToken };"####,
        },
        SchemaMetadata {
            name: "SliderProps",
            version: 1,
            typescript: r####"/**
 * 🎚️ Props for `Component::Slider`. `on_change` moved to the record's `bindings`.
 */
export type SliderProps = { value: number, min: number, max: number, step: number, unit: string | null, };"####,
        },
        SchemaMetadata {
            name: "SpaceToken",
            version: 1,
            typescript: r####"/**
 * 📐️ Closed spacing scale a renderer resolves against the active theme's spacing ramp — never a raw
 * `f32`/px. tokens.json's `spacing` table today only names `compact`/`touch` (see [`crate::Density`]);
 * no full ramp exists there yet, so this scale is the shape this packet's own brief specifies
 * verbatim (`None,Xs,Sm,Md,Lg,Xl,…`) pending a registrar-added token set — flagged in the packet report.
 */
export type SpaceToken = "none" | "xs" | "sm" | "md" | "lg" | "xl" | "xxl";"####,
        },
        SchemaMetadata {
            name: "StackLayout",
            version: 1,
            typescript: r####"/**
 * 📚️ A one-axis flex-like arrangement — expressible by CSS flex, a taffy tree, or a native stack.
 */
export type StackLayout = { axis: Axis, gap: SpaceToken, padding: EdgeSpace, align: Align, justify: Justify, grow: boolean, wrap: boolean, };"####,
        },
        SchemaMetadata {
            name: "StyleSpec",
            version: 1,
            typescript: r####"/**
 * 🎨️ A node's design-token styling — five closed enums, never a raw color or a raw pixel value. A
 * renderer resolves every field against the active theme; this struct only names the tokens. Each
 * field is omitted from the wire at its default, so a default-styled node costs nothing to encode.
 */
export type StyleSpec = { variant: Variant, size: SizeToken, density: Density, tone: Tone, emphasis: Emphasis, };"####,
        },
        SchemaMetadata {
            name: "SurfaceDoc",
            version: 1,
            typescript: r####"/**
 * 📦️ An opaque, pack-encoded payload. The contract never parses it — `doc_schema` on the owning
 * [`SurfaceProps`] names the version-specific shape (e.g. `"world3d@1"`) that some other layer (the
 * `🎬️scene` crate) knows how to decode.
 */
export type SurfaceDoc = { bytes: Array<number>, };"####,
        },
        SchemaMetadata {
            name: "SurfaceId",
            version: 1,
            typescript: r####"/**
 * 🪧️ A render surface address — today's dotted strings, e.g. `"note.play.navigator"`.
 */
export type SurfaceId = string;"####,
        },
        SchemaMetadata {
            name: "SurfaceKind",
            version: 1,
            typescript: r####"/**
 * 🖼️ The 15 embeddable product surface kinds. Ported from the wgpu target's `SurfaceKind`, with its
 * one real wire inconsistency FIXED rather than preserved: `VirtualFileSystem` was
 * `"virtualFileSystem"` (camelCase) where every sibling is kebab-case. This program has no back-compat
 * obligation (greenfield, no users, no legacy support — root `CLAUDE.md`), so the rename is made here
 * deliberately rather than carried forward as debt for "a later packet to make on purpose".
 *
 * **Rename: `"virtualFileSystem"` → `"virtual-file-system"`.**
 */
export type SurfaceKind = "canvas-2d" | "world-3d" | "node-graph" | "text-editor" | "table" | "paint-2d" | "virtual-file-system" | "tiled-map" | "board-2d" | "icon-render" | "ink-canvas" | "graph-timeline" | "block-list" | "diff-view" | "event-feed";"####,
        },
        SchemaMetadata {
            name: "SurfaceProps",
            version: 1,
            typescript: r####"/**
 * 🗺️ An embedded product surface. Replaces the old `UiComponentSceneNode`'s 15 sparse
 * `Option<XxxScene>` fields with exactly ONE payload, identified by `doc_schema` — the 15 product
 * scene structs themselves stay product payloads and move to `🖱️ui/🎬️scene/🦀️component.rs` in a
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
        },
        SchemaMetadata {
            name: "TextProps",
            version: 1,
            typescript: r####"/**
 * 📝️ Props for `Component::Text`.
 */
export type TextProps = { value: Label, emphasize: boolean | null, dataAttributes: { [key in string]?: string } | null, };"####,
        },
        SchemaMetadata {
            name: "ToggleProps",
            version: 1,
            typescript: r####"/**
 * 🔀️ Props for `Component::Toggle`. `on` is the explicit state this contract adds — the old
 * `UiToggleNode` smuggled it through `presence.selected`, exactly the implicit coupling this
 * contract exists to remove. `on_change` moved to the record's `bindings`.
 */
export type ToggleProps = { on: boolean, icon: string, text: Label | null, };"####,
        },
        SchemaMetadata {
            name: "Tone",
            version: 1,
            typescript: r####"/**
 * 🎨️ The semantic color role a renderer resolves against the active theme — named after
 * tokens.json's `colors` table's semantic entries (`primary`, `secondary`, `tertiary`, `danger`,
 * `warning`, `info`, `success`). `Neutral` is the default: no explicit accent, inherit the
 * surrounding surface/text color.
 */
export type Tone = "neutral" | "primary" | "secondary" | "tertiary" | "info" | "success" | "warning" | "danger";"####,
        },
        SchemaMetadata {
            name: "TransitionHint",
            version: 1,
            typescript: r####"/**
 * 🎞️ The transient visual emphasis a node is entering — orthogonal to `activity`/`disabled`. A node
 * carrying neither is in its steady state; the renderer clears this once the transition has played.
 */
export type TransitionHint = "introducing" | "celebrating";"####,
        },
        SchemaMetadata {
            name: "TreeItemProps",
            version: 1,
            typescript: r####"/**
 * 🌿️ Props for `Component::TreeItem` — a single row. `items`/`control` are gone: nested items and
 * the old inline `control: Option<UiControlNode>` are now ordinary children on the record (the
 * `UiControlNode` enum does not get ported — every one of its old variants is already a
 * [`Component`] variant in its own right, so a control-as-child-node needs no separate wrapper
 * type). The row's primary click action (old `action: Option<ActionDescriptor>`) moved to the
 * record's `bindings` (`Trigger::Activate`).
 */
export type TreeItemProps = { label: Label, description: string | null, icon: string | null, defaultOpen: boolean | null, draggable: boolean | null, dragData: { [key in string]?: string } | null,
/**
 * 👁️ Domain "eye toggle": the row stays visible, dimmed, and clickable (to un-hide). NOT the
 * same axis as the record's `activity`/`disabled` — a dimmed row is still fully interactive.
 */
dimmed: boolean | null, rowActions: Array<RowAction>, };"####,
        },
        SchemaMetadata {
            name: "TreeProps",
            version: 1,
            typescript: r####"/**
 * 🌲️ Props for `Component::Tree` — the tree's own binding, nothing else. Sections and items are no
 * longer inline (`sections: Vec<UiTreeSectionNode>`); they are ordinary child nodes
 * (`Component::TreeSection` / `Component::TreeItem`) reached through the record's `children`.
 * `drop_action` moved to the record's `bindings` (`Trigger::Drop`).
 */
export type TreeProps = {
/**
 * 🕹️ Binds this tree to an app-declared `InteractionDefinition` domain — selection/hover for
 * bound items is owned by the framework's presence channel, not by per-item props.
 */
interactionDomain: string | null, };"####,
        },
        SchemaMetadata {
            name: "TreeSectionProps",
            version: 1,
            typescript: r####"/**
 * 🌲️ Props for `Component::TreeSection` — a labeled, collapsible grouping of `TreeItem` children.
 */
export type TreeSectionProps = { label: Label | null, defaultOpen: boolean | null, };"####,
        },
        SchemaMetadata {
            name: "Trigger",
            version: 1,
            typescript: r####"/**
 * 🎯️ The lifecycle moment on a node that fires an [`ActionBinding`] — replaces the old single
 * implicit "the" action every node carried with a closed, named set, so one node can bind several
 * distinct moments (e.g. `Change` while typing, `Commit` on blur) without inventing parallel fields.
 */
export type Trigger = "activate" | "change" | "commit" | "delta" | "drop" | "submit" | "abort" | "repeatLast" | "hoverPreview";"####,
        },
        SchemaMetadata {
            name: "UiContractViolation",
            version: 1,
            typescript: r####"/**
 * ⚠️ One structural invariant a [`crate::UiSnapshot`] fails — every variant here is a whole-document
 * shape property, never a per-patch wire quota (those are [`PatchRejection::QuotaExceeded`]).
 */
export type UiContractViolation = { "type": "cycle", node: UiNodeId, } | { "type": "orphanChild", parent: UiNodeId, child: UiNodeId, } | { "type": "duplicateSiblingKey", parent: UiNodeId, key: string, } | { "type": "nodeQuota", count: number, max: number, } | { "type": "depthQuota", node: UiNodeId, depth: number, max: number, } | { "type": "danglingRoot", node: UiNodeId, } | { "type": "sectionNested", node: UiNodeId, } | { "type": "nonFiniteNumber", node: UiNodeId, };"####,
        },
        SchemaMetadata {
            name: "UiDocumentLimits",
            version: 1,
            typescript: r####"/**
 * 🛡️ Quotas a [`crate::UiSnapshot`]/[`crate::UiPatch`] must stay within. `max_nodes`/`max_depth` bound
 * the document shape and are enforced by [`validate_snapshot`] (surfaced as
 * [`UiContractViolation::NodeQuota`]/[`UiContractViolation::DepthQuota`]); `max_children`/
 * `max_text_bytes`/`max_patch_ops`/`max_patch_bytes` bound one incoming [`crate::UiPatch`] and are
 * enforced directly by [`apply_patch`] (surfaced as [`PatchRejection::QuotaExceeded`]) — rejecting a
 * patch before it is even applied to the shadow draft is cheaper than discovering the violation after.
 */
export type UiDocumentLimits = {
/**
 * 📦️ Total live nodes in one surface. 20 000 comfortably covers the largest known tree (a fully
 * expanded product tree view or timeline) with headroom, while still bounding a malicious
 * plugin's flood well below where a `HashMap<UiNodeId, UiNodeRecord>` becomes a memory concern.
 */
maxNodes: number,
/**
 * 📏️ Deepest legal node-to-root chain. 128 is far beyond any legitimate UI nesting (the deepest
 * real shape, a `Tree`/`TreeSection`/`TreeItem` chain, rarely exceeds a few dozen) and doubles as
 * the traversal's own recursion-depth bound, so it is also the security property that keeps
 * `validate_snapshot`'s stack-free walk cheap even under adversarial input.
 */
maxDepth: number,
/**
 * 👶️ Direct children on one node. 4 096 covers the largest legitimate flat list (an unpaginated
 * tree section or a large `Select`-like listing rendered as children) without letting one node
 * alone approach `max_nodes`.
 */
maxChildren: number,
/**
 * 📝️ UTF-8 bytes in one component's own text-bearing fields (label/description/value/…). 64 KiB
 * is generous for authored UI copy (far beyond a label or even a long description) while refusing
 * to let a single component smuggle an arbitrarily large string through the contract.
 */
maxTextBytes: number,
/**
 * 🩹️ Ops in one [`crate::UiPatch`]. 4 096 mirrors `max_children`'s order of magnitude — no
 * legitimate single reconciliation pass should need more ops than the largest single-node fan-out
 * this crate already permits.
 */
maxPatchOps: number,
/**
 * 📮️ Estimated wire bytes for one [`crate::UiPatch`] (see [`patch_byte_estimate`]). 1 MiB matches
 * a conservative single-frame transport budget — large enough for a full-surface `Upsert` burst,
 * small enough that a malicious patch cannot exhaust an actor mailbox in one message.
 */
maxPatchBytes: number, };"####,
        },
        SchemaMetadata {
            name: "UiIntent",
            version: 1,
            typescript: r####"/**
 * 🎬️ One user action against a specific node at a specific revision — what a renderer emits and the
 * headless runtime dispatches. `revision`/`node_key` let the runtime recognise and drop a `Stale`
 * intent (one whose `revision` trails the surface's current revision by more than one) instead of
 * applying it against geometry the user never actually saw.
 */
export type UiIntent = { surface: SurfaceId, revision: UiRevision, node: UiNodeId,
/**
 * 🔑️ The node's own [`crate::UiNodeRecord::key`], carried alongside the id so a replay or a log
 * entry still identifies the intended element after id churn from an intervening reconciliation.
 */
nodeKey: string, trigger: Trigger, action: ActionId,
/**
 * 🔁️ Echoed verbatim from the firing [`ActionBinding::args`].
 */
args: UiValue | null,
/**
 * ✍️ The trigger-specific payload: `Change`'s new value, `Delta`'s signed step count, `Drop`'s
 * dropped payload — `None` for triggers that carry no data of their own (`Activate`, `Submit`, …).
 */
input: UiValue | null,
/**
 * 🔢️ Renderer-monotonic per surface — lets the runtime order and de-duplicate intents
 * independently of transport delivery order.
 */
seq: bigint, };"####,
        },
        SchemaMetadata {
            name: "UiNodeId",
            version: 1,
            typescript: r####"/**
 * 🔢️ A node's identity within one [`SurfaceId`] — monotonic per surface, never reused, so a stale
 * reference to a removed node is always distinguishable from a fresh node at the same tree position.
 *
 * The TypeScript type is pinned to `number`, not a Rust-centric `bigint` projection for `u64`: serde writes
 * this as a plain JSON number, so `JSON.parse` hands JavaScript a `number` at runtime and a `bigint`
 * declaration would be a type that never actually occurs. Ids are per-surface and monotonic, so the
 * 2^53 exact-integer ceiling is unreachable in practice — a surface would have to mint nine
 * quadrillion nodes to reach it.
 */
export type UiNodeId = number;"####,
        },
        SchemaMetadata {
            name: "UiNodeRecord",
            version: 1,
            typescript: r####"/**
 * 📦️ One row of the flat node table. Never nests another record — children are addressed by
 * [`UiNodeId`] only, so a patch can `Upsert` or `Remove` exactly one node without touching its
 * neighbours or ancestors.
 */
export type UiNodeRecord = { id: UiNodeId,
/**
 * 🔑️ Reconciliation key, unique only among this node's own siblings (not surface-wide).
 */
key: string, component: Component, layout: LayoutSpec, style: StyleSpec, activity: Activity, disabled: boolean, transition: TransitionHint | null, accessibility: AccessibilitySpec, bindings: Array<ActionBinding>, menu: MenuRef | null, children: Array<UiNodeId>, };"####,
        },
        SchemaMetadata {
            name: "UiPatch",
            version: 1,
            typescript: r####"/**
 * 🩹️ A revisioned batch of [`UiPatchOp`]s. Applies atomically: `base_revision` must equal the
 * receiver's current revision or the whole batch is rejected (never partially applied), and success
 * advances the receiver to `revision`.
 */
export type UiPatch = { surface: SurfaceId, baseRevision: UiRevision, revision: UiRevision, ops: Array<UiPatchOp>, };"####,
        },
        SchemaMetadata {
            name: "UiPatchOp",
            version: 1,
            typescript: r####"/**
 * 🩹️ One mutation to a single node (or the root pointer) in an already-received [`UiSnapshot`].
 */
export type UiPatchOp = { "type": "upsert" } & UiNodeRecord | { "type": "setComponent", id: UiNodeId, component: Component, } | { "type": "setLayout", id: UiNodeId, layout: LayoutSpec, } | { "type": "setActivity", id: UiNodeId, activity: Activity, disabled: boolean, } | { "type": "setChildren", id: UiNodeId, children: Array<UiNodeId>, } | { "type": "setStyle", id: UiNodeId, style: StyleSpec, } | { "type": "setAccessibility", id: UiNodeId, accessibility: AccessibilitySpec, } | { "type": "setBindings", id: UiNodeId, bindings: Array<ActionBinding>, } | { "type": "setMenu", id: UiNodeId, menu: MenuRef | null, } | { "type": "remove", id: UiNodeId, } | { "type": "setRoot", id: UiNodeId, };"####,
        },
        SchemaMetadata {
            name: "UiRevision",
            version: 1,
            typescript: r####"/**
 * 🔢️ A snapshot's wire revision — advances by one per accepted [`UiPatch`]; a patch whose
 * `base_revision` does not match the receiver's current revision is rejected whole.
 */
export type UiRevision = number;"####,
        },
        SchemaMetadata {
            name: "UiSnapshot",
            version: 1,
            typescript: r####"/**
 * 📸️ A complete, self-contained render of one surface at one revision — the payload a fresh
 * subscriber receives before any [`UiPatch`] applies. `nodes` is an unordered flat table; tree shape
 * lives entirely in `root` plus each record's own `children`.
 */
export type UiSnapshot = { surface: SurfaceId, revision: UiRevision, root: UiNodeId, nodes: Array<UiNodeRecord>,
/**
 * 📐️ Bumped by the layout engine whenever geometry may have changed for reasons a patch does not
 * itself carry (e.g. a host window resize) — renderers use this to decide whether cached layout
 * results are still trustworthy without diffing every record.
 */
layoutEpoch: bigint, };"####,
        },
        SchemaMetadata {
            name: "UiValue",
            version: 1,
            typescript: r####"/**
 * 🧬️ A neutral, JSON-shaped value — the ONE recursive type in this crate. Every node in
 * `🦀️document.rs` avoids inline recursion by addressing children through [`crate::UiNodeId`] instead
 * of nesting a node inside another; `UiValue` is the deliberate exception because it does not
 * describe document shape at all, it describes an arbitrary opaque payload (action args, extension
 * props) that genuinely IS JSON-shaped, and `Vec`/`BTreeMap` already give the schema an indirection to
 * resolve (heap-allocated, not an inline field) rather than the infinitely-sized-struct problem
 * direct node-in-node nesting would create.
 *
 * ⚠️ The os-kernel's `DslValue` (`🧰️framework/🔨️modules/🌱️value/🦀️component.rs`) must NEVER appear in
 * this crate — this crate has no such dependency and stays `wasm32-wasip2`/`wasm32-unknown-unknown`
 * safe by construction. `From`/`Into` conversions between `UiValue` and `DslValue` belong in the
 * os-kernel crate, never here.
 */
export type UiValue = null | boolean | number | string | Array<UiValue> | { [key in string]?: UiValue };"####,
        },
        SchemaMetadata {
            name: "Variant",
            version: 1,
            typescript: r####"/**
 * 🖌️ The chrome treatment a renderer paints a component with — independent of [`Tone`] (which color
 * role) and [`Emphasis`] (how prominent).
 */
export type Variant = "solid" | "outline" | "ghost" | "plain";"####,
        },
        SchemaMetadata {
            name: "WindowLayout",
            version: 1,
            typescript: r####"/**
 * 🪟️ The window-shell root. Moved here from the wgpu target's `WindowLayout` — same name, one
 * recursive `WindowLayoutNode` root instead of the old `WindowLayoutRoot` `Axis`/`Stack` union.
 */
export type WindowLayout = { root: WindowLayoutNode, };"####,
        },
        SchemaMetadata {
            name: "WindowLayoutNode",
            version: 1,
            typescript: r####"/**
 * 🪟️ The window-shell tree: a single recursive, internally-tagged enum replacing the old
 * `WindowLayoutWindowNode`/`WindowLayoutStackNode`/`WindowLayoutAxisNode` trio and their
 * `kind: String` + `#[serde(untagged)]` scheme. `size` stays an `Option<f64>` fraction of the parent
 * split (a ratio, not a pixel measurement, so it is exempt from the [`SpaceToken`] rule). The
 * `alias = "activeId"` serde alias on the old stack node is dropped — greenfield, fixtures
 * re-handcrafted, no compatibility requirement.
 */
export type WindowLayoutNode = { "kind": "window", window_kind_id: string, title: string | null, instance_id: string | null, template_id: string | null, corner: WindowStackCorner | null, } | { "kind": "stack", size: number | null, active_window_kind_id: string | null, children: Array<WindowLayoutNode>, } | { "kind": "split", axis: Axis, size: number | null, children: Array<WindowLayoutNode>, };"####,
        },
        SchemaMetadata {
            name: "WindowStackCorner",
            version: 1,
            typescript: r####"/**
 * 🪟️ Corner of a window stack where a tab chip docks. Ported verbatim from the wgpu target's
 * `WindowStackCorner`.
 */
export type WindowStackCorner = "topLeft" | "topRight" | "bottomLeft" | "bottomRight";"####,
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

    /// 🟦️ Renders the stable language projection consumed by every semantic UI host.
    pub fn render_typescript() -> String {
        let mut output = String::from("/** @generated by `bun nx run @semio-tech/ui-contract-rs:generate` from versioned owned UI schema metadata. Do not edit. */\n\n");
        for (index, metadata) in TYPES.iter().enumerate() {
            output.push_str(metadata.typescript);
            output.push_str(if index + 1 == TYPES.len() { "\n" } else { "\n\n" });
        }
        output
    }
}
//#endregion 🧬️SchemaMetadata

#[path = "🦀️accessibility.rs"]
mod accessibility;
#[path = "🦀️action.rs"]
mod action;
#[path = "🦀️builder.rs"]
mod builder;
#[path = "🦀️component.rs"]
mod component;
/// 🧪️ Loads and asserts against `📚️examples/🧪️conformance/` — entirely `#[cfg(test)]` inside, so it
/// mounts unconditionally here without affecting the wasm check targets (see the file's own header).
#[path = "🦀️conformance.rs"]
mod conformance;
#[path = "🦀️document.rs"]
mod document;
#[path = "🦀️layout.rs"]
mod layout;
#[path = "🦀️limits.rs"]
mod limits;
#[path = "🦀️presence.rs"]
mod presence;
#[path = "🦀️style.rs"]
mod style;
#[path = "🦀️surface.rs"]
mod surface;

pub use accessibility::*;
pub use action::*;
pub use builder::*;
pub use component::*;
pub use document::*;
pub use layout::*;
pub use limits::*;
pub use presence::*;
pub use style::*;
pub use surface::*;
