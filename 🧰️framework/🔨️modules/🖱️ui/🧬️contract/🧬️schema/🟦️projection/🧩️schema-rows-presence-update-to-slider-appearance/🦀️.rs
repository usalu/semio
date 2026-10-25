//! 🧩️ Schema rows from `PresenceUpdate` to `SliderAppearance`: 16 versioned wire type rows of the owned semantic UI TypeScript projection, in table order. See [`super`].

use super::*;

pub(super) const PRESENCE_UPDATE: SchemaMetadata = SchemaMetadata {
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
};

pub(super) const PROGRESS_PROPS: SchemaMetadata = SchemaMetadata {
    name: "ProgressProps",
    version: 1,
    typescript: r####"/**
 * 📶️ Props for `Component::Progress` — a read-only progress bar. `total` absent means indeterminate: a
 * renderer shows a busy sweep and announces no value; present means determinate on `0..=total`, and the
 * percentage is derived by the renderer, never sent. `value_text` is the already-localized spoken form
 * (`aria-valuetext`); plugins never send a percentage string of their own.
 */
export type ProgressProps = { completed: number, total: number | null, valueText: Label, };"####,
};

pub(super) const QUOTA_KIND: SchemaMetadata = SchemaMetadata {
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
};

pub(super) const RING_PROPS: SchemaMetadata = SchemaMetadata {
    name: "RingProps",
    version: 1,
    typescript: r####"/**
 * 💍️ Props for `Component::Ring`. `on_change` moved to the record's `bindings`.
 */
export type RingProps = { orbId: string, t: number, };"####,
};

pub(super) const ROW_ACTION: SchemaMetadata = SchemaMetadata {
    name: "RowAction",
    version: 4,
    typescript: r####"/**
 * 🎬️ One action affordance painted on (or reachable from) a [`Component::TreeItem`] or
 * [`Component::TableRow`] row. It names only its `verb`: the scope, version and argument map are the
 * row's ONE [`RowTarget`], which every row action and the row's activation inherit, so N actions cost N
 * verbs, never N argument maps.
 */
export type RowAction = {
/**
 * 🖼️ Icon key. See [`ButtonProps::icon`] for why this is a plain `String`, not a closed enum.
 */
icon: string, label: Label | null,
/**
 * ▶️ The verb this action fires on the row's [`RowTarget`].
 */
verb: string, placement: RowActionPlacement,
/**
 * 🚫️ A disabled action paints and announces disabled and never dispatches ([`RowTarget::action_binding`] refuses it
 * typed). Only a disabled action carries the flag, so an enabled row costs nothing for it.
 */
disabled: boolean,
/**
 * 💬️ Why a disabled action cannot run, producer-localized. A disabled action stays focusable (`aria-disabled`, WAI-ARIA
 * focusable-when-disabled) and every renderer names this reason as its description (`aria-describedby`).
 */
reason?: Label | null, };"####,
};

pub(super) const ROW_ACTION_PLACEMENT: SchemaMetadata = SchemaMetadata {
    name: "RowActionPlacement",
    version: 1,
    typescript: r####"/**
 * 📍️ Where a [`RowAction`] paints: on the tree row itself, or folded into the row's context menu.
 */
export type RowActionPlacement = "row" | "menu";"####,
};

pub(super) const ROW_TARGET: SchemaMetadata = SchemaMetadata {
    name: "RowTarget",
    version: 1,
    typescript: r####"/**
 * 🎯️ The ONE action target of a row: the controller `scope` at its contract `version` that every
 * [`RowAction::verb`] and the row's `activation` fire in, the ONE argument map they all inherit, and the
 * verb the row's primary activation (open, select) fires. A host dispatches a row verb as
 * [`RowTarget::binding`] — the same [`crate::ActionBinding`] shape any record binding has, so a tree row
 * and a table row with the same target dispatch identically.
 */
export type RowTarget = { scope: string, version: number, args: { [key in string]?: UiValue } | null,
/**
 * 🏹️ The verb the row's primary activation fires on this target.
 */
activation: string | null, };"####,
};

pub(super) const SCROLL_AXES: SchemaMetadata = SchemaMetadata {
    name: "ScrollAxes",
    version: 1,
    typescript: r####"/**
 * 🖱️ Which axes a [`ScrollLayout`] permits overflow scrolling on.
 */
export type ScrollAxes = "none" | "horizontal" | "vertical" | "both";"####,
};

pub(super) const SCROLL_LAYOUT: SchemaMetadata = SchemaMetadata {
    name: "ScrollLayout",
    version: 1,
    typescript: r####"/**
 * 🖱️ A viewport clipping its content and permitting overflow scroll on the named axes.
 */
export type ScrollLayout = { axes: ScrollAxes, padding: EdgeSpace, sizing: Sizing, };"####,
};

pub(super) const SELECT_APPEARANCE: SchemaMetadata = SchemaMetadata {
    name: "SelectAppearance",
    version: 1,
    typescript: r####"/**
 * 🗳️ How a select shows its one choice: a `menu` that opens its options on demand, or `segmented` — every option
 * visible at once as one row of mutually exclusive buttons (a radio group), for the few-option choices a descriptor
 * declares segmented (`x-semio-ui.widget: "segmented"`). Arrow keys move the choice along a segmented row.
 */
export type SelectAppearance = "menu" | "segmented";"####,
};

pub(super) const SELECT_ITEM: SchemaMetadata = SchemaMetadata {
    name: "SelectItem",
    version: 1,
    typescript: r####"/**
 * 🔽️ One option of a [`Component::Select`].
 */
export type SelectItem = { value: string, label: Label, };"####,
};

pub(super) const SELECT_PROPS: SchemaMetadata = SchemaMetadata {
    name: "SelectProps",
    version: 2,
    typescript: r####"/**
 * 🔽️ Props for `Component::Select`. `on_change` moved to the record's `bindings`. `appearance` only chooses how the one
 * choice is shown ([`SelectAppearance`]); value, items and `Trigger::Change` are the same in both.
 */
export type SelectProps = { value: string, items: Array<SelectItem>, placeholder: Label | null, appearance?: SelectAppearance, };"####,
};

pub(super) const SEPARATOR_PROPS: SchemaMetadata = SchemaMetadata {
    name: "SeparatorProps",
    version: 1,
    typescript: r####"/**
 * ➖️ Props for `Component::Separator`. Every field the old `UiSeparatorNode` carried
 * (`presence`, `menu`) now lives on the record, so this is intentionally empty — kept as its own
 * struct (rather than a unit variant) purely for structural symmetry with every other component.
 */
export type SeparatorProps = Record<never, never>;"####,
};

pub(super) const SIZE_TOKEN: SchemaMetadata = SchemaMetadata {
    name: "SizeToken",
    version: 1,
    typescript: r####"/**
 * 📏️ A component's t-shirt size. Mirrors the one real precedent in the wgpu target's old
 * `StyleSpec.size` (`"md"`); no dedicated component-size ramp exists in tokens.json yet — see the
 * packet report.
 */
export type SizeToken = "xs" | "sm" | "md" | "lg" | "xl";"####,
};

pub(super) const SIZING: SchemaMetadata = SchemaMetadata {
    name: "Sizing",
    version: 1,
    typescript: r####"/**
 * 📏️ How a node sizes itself along one axis relative to its parent's flow — `Fixed` still names a
 * [`SpaceToken`], never a pixel value.
 */
export type Sizing = "hug" | "fill" | { "fixed": SpaceToken };"####,
};

pub(super) const SLIDER_APPEARANCE: SchemaMetadata = SchemaMetadata {
    name: "SliderAppearance",
    version: 1,
    typescript: r####"/**
 * 🎡️ How a slider draws its travel: a straight `track`, or a rotary `dial` sweeping one full counter-clockwise
 * turn with the travel's centre at three o'clock — so a `-180°..180°` angle points where it turns
 * ([`dial_angle`]/[`dial_position`]). Both take the same keys, pointer law and accessibility.
 */
export type SliderAppearance = "track" | "dial";"####,
};
