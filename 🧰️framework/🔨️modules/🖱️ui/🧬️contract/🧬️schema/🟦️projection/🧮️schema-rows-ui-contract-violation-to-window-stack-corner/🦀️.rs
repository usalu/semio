//! 🧮️ Schema rows from `UiContractViolation` to `WindowStackCorner`: 17 versioned wire type rows of the owned semantic UI TypeScript projection, in table order. See [`super`].

use super::*;

pub(super) const UI_CONTRACT_VIOLATION: SchemaMetadata = SchemaMetadata {
    name: "UiContractViolation",
    version: 5,
    typescript: r####"/**
 * ⚠️ One structural invariant a [`crate::UiSnapshot`] fails — every variant here is a whole-document
 * shape property, never a per-patch wire quota (those are [`PatchRejection::QuotaExceeded`]).
 */
export type UiContractViolation = { "type": "cycle", node: UiNodeId, } | { "type": "orphanChild", parent: UiNodeId, child: UiNodeId, } | { "type": "duplicateSiblingKey", parent: UiNodeId, key: string, } | { "type": "nodeQuota", count: number, max: number, } | { "type": "depthQuota", node: UiNodeId, depth: number, max: number, } | { "type": "danglingRoot", node: UiNodeId, } | { "type": "sectionNested", node: UiNodeId, } | { "type": "nonFiniteNumber", node: UiNodeId, } | { "type": "invalidTreeInlineToolbar", node: UiNodeId, toolbar: UiNodeId, } | { "type": "invalidTreeSectionHeaderToolbar", node: UiNodeId, toolbar: UiNodeId, } | { "type": "invalidTreeDetail", node: UiNodeId, detail: UiNodeId, } | { "type": "invalidRowTarget", node: UiNodeId, } | { "type": "invalidSnaps", node: UiNodeId, } | { "type": "invalidNumberRange", node: UiNodeId, };"####,
};

pub(super) const UI_DOCUMENT_LIMITS: SchemaMetadata = SchemaMetadata {
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
};

pub(super) const UI_INTENT: SchemaMetadata = SchemaMetadata {
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
};

pub(super) const UI_NODE_ID: SchemaMetadata = SchemaMetadata {
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
};

pub(super) const UI_NODE_RECORD: SchemaMetadata = SchemaMetadata {
    name: "UiNodeRecord",
    version: 1,
    typescript: r####"/**
 * 📦️ One row of the flat node table. Never nests another record — children are addressed by
 * [`UiNodeId`] only, so a patch can `Upsert` or `Remove` exactly one node without touching its
 * neighbours or ancestors.
 */
export type UiNodeRecord = { id: UiNodeId,
/**
 * 🦄️ Reconciliation key, unique only among this node's own siblings (not surface-wide).
 */
key: string, component: Component, layout: LayoutSpec, style: StyleSpec, activity: Activity, disabled: boolean, transition: TransitionHint | null, accessibility: AccessibilitySpec, bindings: Array<ActionBinding>, menu: MenuRef | null, children: Array<UiNodeId>, };"####,
};

pub(super) const UI_NUMBER_BOUND: SchemaMetadata = SchemaMetadata {
    name: "UiNumberBound",
    version: 1,
    typescript: r####"/**
 * 🧱️ One hard bound of a numeric control's value: the limit, whether the limit itself is excluded, and the
 * refusal a renderer shows when a typed value crosses it — already localized by the producer and naming the
 * bound in display units (the contract carries no locale). A crossing value is never dispatched: the draft is
 * kept and the refusal shown.
 */
export type UiNumberBound = { value: number, exclusive?: boolean, refusal?: Label | null, };"####,
};

pub(super) const UI_NUMBER_LIMITS: SchemaMetadata = SchemaMetadata {
    name: "UiNumberLimits",
    version: 1,
    typescript: r####"/**
 * 🛤️ The hard range a numeric control's value must keep, each side open when absent. A slider's `min`/`max`
 * are then only its travel (a soft range): a typed value may leave the travel while the limits admit it.
 */
export type UiNumberLimits = { min?: UiNumberBound | null, max?: UiNumberBound | null, };"####,
};

pub(super) const UI_NUMBER_SCALE: SchemaMetadata = SchemaMetadata {
    name: "UiNumberScale",
    version: 1,
    typescript: r####"/**
 * 📈️ How a slider's travel maps onto its track: evenly (`linear`), or by ratio (`log`, a strictly positive
 * travel) — see [`slider_axis_position`]/[`slider_axis_value`].
 */
export type UiNumberScale = "linear" | "log";"####,
};

pub(super) const UI_PATCH: SchemaMetadata = SchemaMetadata {
    name: "UiPatch",
    version: 1,
    typescript: r####"/**
 * 🩹️ A revisioned batch of [`UiPatchOp`]s. Applies atomically: `base_revision` must equal the
 * receiver's current revision or the whole batch is rejected (never partially applied), and success
 * advances the receiver to `revision`.
 */
export type UiPatch = { surface: SurfaceId, baseRevision: UiRevision, revision: UiRevision, ops: Array<UiPatchOp>, };"####,
};

pub(super) const UI_PATCH_OP: SchemaMetadata = SchemaMetadata {
    name: "UiPatchOp",
    version: 1,
    typescript: r####"/**
 * 🩹️ One mutation to a single node (or the root pointer) in an already-received [`UiSnapshot`].
 */
export type UiPatchOp = { "type": "upsert" } & UiNodeRecord | { "type": "setComponent", id: UiNodeId, component: Component, } | { "type": "setLayout", id: UiNodeId, layout: LayoutSpec, } | { "type": "setActivity", id: UiNodeId, activity: Activity, disabled: boolean, } | { "type": "setChildren", id: UiNodeId, children: Array<UiNodeId>, } | { "type": "setStyle", id: UiNodeId, style: StyleSpec, } | { "type": "setAccessibility", id: UiNodeId, accessibility: AccessibilitySpec, } | { "type": "setBindings", id: UiNodeId, bindings: Array<ActionBinding>, } | { "type": "setMenu", id: UiNodeId, menu: MenuRef | null, } | { "type": "remove", id: UiNodeId, } | { "type": "setRoot", id: UiNodeId, };"####,
};

pub(super) const UI_REVISION: SchemaMetadata = SchemaMetadata {
    name: "UiRevision",
    version: 1,
    typescript: r####"/**
 * 🔢️ A snapshot's wire revision — advances by one per accepted [`UiPatch`]; a patch whose
 * `base_revision` does not match the receiver's current revision is rejected whole.
 */
export type UiRevision = number;"####,
};

pub(super) const UI_SNAPSHOT: SchemaMetadata = SchemaMetadata {
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
};

pub(super) const UI_VALUE: SchemaMetadata = SchemaMetadata {
    name: "UiValue",
    version: 1,
    typescript: r####"/**
 * 🧬️ A neutral, JSON-shaped value — the ONE recursive type in this crate. Every node in
 * `📃️document.rs` avoids inline recursion by addressing children through [`crate::UiNodeId`] instead
 * of nesting a node inside another; `UiValue` is the deliberate exception because it does not
 * describe document shape at all, it describes an arbitrary opaque payload (action args, extension
 * props) that genuinely IS JSON-shaped, and `Vec`/`BTreeMap` already give the schema an indirection to
 * resolve (heap-allocated, not an inline field) rather than the infinitely-sized-struct problem
 * direct node-in-node nesting would create.
 *
 * ⚠️ The os-kernel's `DslValue` (`🧰️framework/🔨️modules/🌱️value/🦀️.rs`) must NEVER appear in
 * this crate — this crate has no such dependency and stays `wasm32-wasip2`/`wasm32-unknown-unknown`
 * safe by construction. `From`/`Into` conversions between `UiValue` and `DslValue` belong in the
 * os-kernel crate, never here.
 */
export type UiValue = null | boolean | number | string | Array<UiValue> | { [key in string]?: UiValue };"####,
};

pub(super) const VARIANT: SchemaMetadata = SchemaMetadata {
    name: "Variant",
    version: 1,
    typescript: r####"/**
 * 🖌️ The chrome treatment a renderer paints a component with — independent of [`Tone`] (which color
 * role) and [`Emphasis`] (how prominent).
 */
export type Variant = "solid" | "outline" | "ghost" | "plain";"####,
};

pub(super) const WINDOW_LAYOUT: SchemaMetadata = SchemaMetadata {
    name: "WindowLayout",
    version: 1,
    typescript: r####"/**
 * 🪟️ The window-shell root. Moved here from the wgpu target's `WindowLayout` — same name, one
 * recursive `WindowLayoutNode` root instead of the old `WindowLayoutRoot` `Axis`/`Stack` union.
 */
export type WindowLayout = { root: WindowLayoutNode, };"####,
};

pub(super) const WINDOW_LAYOUT_NODE: SchemaMetadata = SchemaMetadata {
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
};

pub(super) const WINDOW_STACK_CORNER: SchemaMetadata = SchemaMetadata {
    name: "WindowStackCorner",
    version: 1,
    typescript: r####"/**
 * 🪟️ Corner of a window stack where a tab chip docks. Ported verbatim from the wgpu target's
 * `WindowStackCorner`.
 */
export type WindowStackCorner = "topLeft" | "topRight" | "bottomLeft" | "bottomRight";"####,
};
