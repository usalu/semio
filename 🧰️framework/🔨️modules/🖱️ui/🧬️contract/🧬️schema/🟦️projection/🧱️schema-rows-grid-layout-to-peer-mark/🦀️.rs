//! 🧱️ Schema rows from `GridLayout` to `PeerMark`: 19 versioned wire type rows of the owned semantic UI TypeScript projection, in table order. See [`super`].

use super::*;

pub(super) const GRID_LAYOUT: SchemaMetadata = SchemaMetadata {
    name: "GridLayout",
    version: 1,
    typescript: r####"/**
 * 🔲️ A two-dimensional track arrangement — expressible by CSS grid or a taffy grid tree.
 */
export type GridLayout = { columns: Array<GridTrack>, rows: Array<GridTrack>, columnGap: SpaceToken, rowGap: SpaceToken, padding: EdgeSpace, align: Align, justify: Justify, };"####,
};

pub(super) const GRID_TRACK: SchemaMetadata = SchemaMetadata {
    name: "GridTrack",
    version: 1,
    typescript: r####"/**
 * 🔲️ One grid track's sizing rule — `Fraction` is a proportion count, never a pixel width.
 */
export type GridTrack = "auto" | { "fraction": number } | { "fixed": SpaceToken } | "minContent" | "maxContent";"####,
};

pub(super) const ICON_SELECT_PROPS: SchemaMetadata = SchemaMetadata {
    name: "IconSelectProps",
    version: 1,
    typescript: r####"/**
 * 🖼️ Props for `Component::IconSelect`. `on_change` moved to the record's `bindings`.
 */
export type IconSelectProps = { value: string, uniform: boolean, classifierKind: string, };"####,
};

pub(super) const IMAGE_PROPS: SchemaMetadata = SchemaMetadata {
    name: "ImageProps",
    version: 1,
    typescript: r####"/**
 * 🖼️ Props for `Component::Image`.
 */
export type ImageProps = { src: string, alt: Label | null, };"####,
};

pub(super) const INPUT_KIND: SchemaMetadata = SchemaMetadata {
    name: "InputKind",
    version: 1,
    typescript: r####"/**
 * ⌨️ The closed set of input kinds. Grepped from the fleet's actual `UiInputNode.input_kind` string
 * literals (`"text"`, `"textarea"`, `"number"`, `"file"`) plus the values the React `🟦️Interpreter`
 * already branches on for `inputKind` (`"number"`, `"longText"`, `"date"`, `"color"`, `"file"`,
 * default `"text"`) — the union of both sides, since the renderer supports more kinds than any
 * current plugin emits yet. Note the old Rust/TS spelling mismatch this closes: Rust emitted
 * `"textarea"`, TS checked `"longText"`; this enum has exactly one spelling (`LongText`) for that
 * kind. No `Search` kind exists anywhere in the fleet today, so it is not included (see the
 * packet's own report for the grep evidence — adding it back is a one-variant change, not a design
 * change, if a plugin needs it later).
 */
export type InputKind = "text" | "longText" | "number" | "date" | "color" | "file";"####,
};

pub(super) const INPUT_PROPS: SchemaMetadata = SchemaMetadata {
    name: "InputProps",
    version: 5,
    typescript: r####"/**
 * ⌨️ Props for `Component::Input`. `on_change` moved to the record's `bindings`
 * (`Trigger::Change`/`Trigger::Commit`).
 */
export type InputProps = { kind: InputKind, value: string, placeholder: Label | null,
/**
 * 🫳️ Commit convention string carried verbatim from the old wire shape (e.g. `"blur"`) — no
 * closed set of these was found in the fleet, unlike `input_kind`.
 */
commit: string | null, min: number | null, max: number | null, step: number | null, accept: string | null,
/**
 * 🔣️ Fraction digits a `InputKind::Number` field shows and commits — see [`round_ui_number`].
 */
precision: number | null,
/**
 * 📌️ Detents of a `InputKind::Number` field under the detent law of [`SliderProps::snaps`] against `min`/`max`
 * (unbounded when absent): its page keys jump between them ([`ui_number_key_value`]), typing never snaps.
 */
snaps: Array<number>,
/**
 * 🔁️ A `InputKind::Number` field's `value` is the stored number; it shows and reads `stored × display_factor`
 * ([`ui_number_display_text`]/[`ui_number_typed_value`]).
 */
displayFactor?: number | null,
/**
 * ⛔️ The hard range a typed number must keep (`min`/`max` themselves when absent), see [`UiNumberLimits`].
 */
limits?: UiNumberLimits | null,
/**
 * 🎯️ Opaque producer-owned identity for retaining an in-progress draft across
 * revisions while the same semantic edit target remains mounted.
 */
draftTarget?: string | null,
/**
 * 🧾️ Decimal u64 revision of the native document projection that authored this input.
 */
publicationRevision?: string | null, };"####,
};

pub(super) const JUSTIFY: SchemaMetadata = SchemaMetadata {
    name: "Justify",
    version: 1,
    typescript: r####"/**
 * ↔️ Main-axis distribution — the CSS `justify-content` equivalent.
 */
export type Justify = "start" | "center" | "end" | "spaceBetween" | "spaceAround" | "spaceEvenly";"####,
};

pub(super) const KEY_VALUE_ENTRY: SchemaMetadata = SchemaMetadata {
    name: "KeyValueEntry",
    version: 1,
    typescript: r####"/**
 * 🗝️ One row of a [`Component::KeyValueList`].
 */
export type KeyValueEntry = { label: Label, value: string, };"####,
};

pub(super) const KEY_VALUE_LIST_PROPS: SchemaMetadata = SchemaMetadata {
    name: "KeyValueListProps",
    version: 1,
    typescript: r####"/**
 * 🗝️ Props for `Component::KeyValueList`.
 */
export type KeyValueListProps = { entries: Array<KeyValueEntry>, };"####,
};

pub(super) const LABEL: SchemaMetadata = SchemaMetadata {
    name: "Label",
    version: 1,
    typescript: r####"/**
 * 🏷️ Display-ready UI text carried on the wire.
 *
 * ⚠️ Decision (flagged per packet brief): the old `UiNode`'s `Label` (`crate::wgpu::Label` in
 * `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🏷️label/🦀️.rs`) is NOT reused here. It
 * is defined inside the old wgpu-target UI package, imports that package's own `Locale`/
 * `Terminology` axes, and its whole point (`From<LabelText>` wired to the `app_labels!` macro,
 * no `From<&str>`) is a compile-time-checked-label enforcement mechanism that lives at the
 * authoring boundary, not the wire boundary — and this crate must not depend on that package at
 * all (no engine, no wgpu; see `🦀️.rs`). This is therefore a minimal, independent transparent
 * string. The localization/terminology resolution that used to happen via `LabelText::fill` still
 * happens upstream of the runtime (manifest/host), before a `Label` ever reaches this contract.
 */
export type Label = string;"####,
};

pub(super) const LAYOUT_SPEC: SchemaMetadata = SchemaMetadata {
    name: "LayoutSpec",
    version: 1,
    typescript: r####"/**
 * 🧬️ The renderer-neutral layout vocabulary a [`crate::UiNodeRecord`] carries — expressible by CSS
 * flex/grid, by a taffy tree, and by native stacks alike. No CSS strings, no taffy types, no pixel
 * geometry: every metric is a closed enum over [`SpaceToken`].
 */
export type LayoutSpec = { "kind": "leaf" } & LeafLayout | { "kind": "stack" } & StackLayout | { "kind": "grid" } & GridLayout | { "kind": "overlay" } & OverlayLayout | { "kind": "scroll" } & ScrollLayout | { "kind": "absolute" } & AbsoluteLayout;"####,
};

pub(super) const LEAF_LAYOUT: SchemaMetadata = SchemaMetadata {
    name: "LeafLayout",
    version: 1,
    typescript: r####"/**
 * 🍃️ A childless terminal node's own box sizing — text, image, and other atomic components.
 */
export type LeafLayout = { width: Sizing, height: Sizing, };"####,
};

pub(super) const LIVENESS: SchemaMetadata = SchemaMetadata {
    name: "Liveness",
    version: 1,
    typescript: r####"/**
 * 📢️ An ARIA-live-region politeness level, translated by each renderer into its own live-announce
 * mechanism (DOM `aria-live`, the GPU renderer's accessibility snapshot, ...).
 */
export type Liveness = "off" | "polite" | "assertive";"####,
};

pub(super) const MENU_REF: SchemaMetadata = SchemaMetadata {
    name: "MenuRef",
    version: 1,
    typescript: r####"/**
 * 📋️ A reference to a resolved context menu — replaces the old `UiMenuRef`'s `DslValue` args with
 * the crate-neutral [`UiValue`].
 */
export type MenuRef = { id: string, args: UiValue | null, };"####,
};

pub(super) const NUMBER_STEPPER_PROPS: SchemaMetadata = SchemaMetadata {
    name: "NumberStepperProps",
    version: 3,
    typescript: r####"/**
 * 🔢️ Props for `Component::NumberStepper`. `on_absolute`/`on_delta` both moved to the record's
 * `bindings`, distinguished by `Trigger`. `precision` is the fraction digits it shows and commits (in display
 * units); `snaps` are its detents under the slider's detent law against `min`/`max` (the page keys stop on them,
 * [`ui_number_key_value`]); the display facets and `limits` read as on [`SliderProps`], `limits` refusing a typed
 * value (`min`/`max` themselves when absent).
 */
export type NumberStepperProps = { value: number, step: number, uniform: boolean, min: number | null, max: number | null, precision: number | null, snaps?: Array<number>, unit?: string | null, displayUnit?: string | null, displayFactor?: number | null, limits?: UiNumberLimits | null, };"####,
};

pub(super) const OVERLAY_LAYOUT: SchemaMetadata = SchemaMetadata {
    name: "OverlayLayout",
    version: 1,
    typescript: r####"/**
 * 🪟️ A positioning context whose children stack on top of one another anchored to the box —
 * modals, popovers, tooltips.
 */
export type OverlayLayout = { anchor: Anchor, inset: EdgeSpace, dismissible: boolean, };"####,
};

pub(super) const OWN_PRESENCE: SchemaMetadata = SchemaMetadata {
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
};

pub(super) const PATCH_REJECTION: SchemaMetadata = SchemaMetadata {
    name: "PatchRejection",
    version: 1,
    typescript: r####"/**
 * 🚫️ Why [`apply_patch`] rejected a [`crate::UiPatch`] — carries enough detail (both revisions, the
 * exceeded quota with its actual/max, or the full violation list) for the existing `patch-rejected`
 * wire event to explain itself and for the sender to resynchronise.
 */
export type PatchRejection = { "type": "revisionMismatch", expected: UiRevision, actual: UiRevision, } | { "type": "unknownNode", id: UiNodeId, } | { "type": "quotaExceeded", quota: QuotaKind, actual: number, max: number, } | { "type": "invariantViolated", violations: Array<UiContractViolation>, };"####,
};

pub(super) const PEER_MARK: SchemaMetadata = SchemaMetadata {
    name: "PeerMark",
    version: 1,
    typescript: r####"/**
 * 👥️ One OTHER peer's mark on a node — hover/selection dot plus initials chip. Ported faithfully
 * from the old wgpu target's `UiPeerMark` (contract-freeze §C7.6, ticket 26/08/17/SHARED-PRESENCE-
 * SESSION-COLORS-AND-UNIVERSAL-ARTIFACT-CREATION) — `label` is still the actor id's display form, not
 * a free-text caption.
 */
export type PeerMark = { actor: string, color: number | null, hovered: boolean, selected: boolean, label: string, };"####,
};
