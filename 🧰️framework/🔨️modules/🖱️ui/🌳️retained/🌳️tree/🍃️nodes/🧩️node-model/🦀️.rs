//! 🧩️ Node model: the node keys, flags, widget specs and state, the layout, paint and accepted layout buckets and the `Node` with its accessors. See [`super`].

use super::*;

// #region tree
/// 🔑️ Stable child identity for keyed reconciliation: the source `UiNode`'s explicit `id` field
/// when it has one, else a `(variant, ordinal)` positional fallback.
#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum NodeKey {
    Explicit(String),
    Positional(u32, u32),
}

/// 🔑️ Borrows the original explicit text or copies a positional identity without heap effects.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum NodeKeyRef<'a> {
    Explicit(&'a str),
    Positional(u32, u32),
}

impl NodeKey {
    pub fn borrowed(&self) -> NodeKeyRef<'_> {
        match self {
            Self::Explicit(text) => NodeKeyRef::Explicit(text),
            Self::Positional(variant, ordinal) => NodeKeyRef::Positional(*variant, *ordinal),
        }
    }
}

/// 🚩️ Per-node dirty/interaction bits. Hand-rolled over a `u16` (no `bitflags` dep) to keep the
/// crate dependency-free for this ~10-flag set.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct NodeFlags(pub(super) u16);

impl NodeFlags {
    pub const HOVERED: NodeFlags = NodeFlags(1 << 0);
    pub const ACTIVE: NodeFlags = NodeFlags(1 << 1);
    pub const FOCUSED: NodeFlags = NodeFlags(1 << 2);
    pub const DIRTY_LAYOUT: NodeFlags = NodeFlags(1 << 3);
    pub const DIRTY_PAINT: NodeFlags = NodeFlags(1 << 4);
    pub const SUBTREE_DIRTY: NodeFlags = NodeFlags(1 << 5);
    pub const OVERLAY: NodeFlags = NodeFlags(1 << 6);
    pub const CLIPS_CHILDREN: NodeFlags = NodeFlags(1 << 7);
    pub const HIT_TRANSPARENT: NodeFlags = NodeFlags(1 << 8);
    pub const HAS_POPUP: NodeFlags = NodeFlags(1 << 9);
    /// 🫳️ M5 `events`: this node is a drag source (has, or can have, a registered `DragPayload`).
    /// Purely advisory for paint (grab-cursor affordance)/cursor derivation — `events` itself tracks
    /// draggability via its own `EventRouter::set_drag_payload` registry, not this flag.
    pub const DRAG_SOURCE: NodeFlags = NodeFlags(1 << 10);
    /// 🎯️ M5 `events`: this node accepts drops (paired with `EventRouter::set_drop_accept` for the
    /// finer per-widget predicate). `events::nearest_accepting_drop_target` walks the bubble chain
    /// looking for this flag.
    pub const DROP_TARGET: NodeFlags = NodeFlags(1 << 11);
    /// 🖱️ M5 `events`: this node owns a scrollable viewport (`WidgetState::scroll_offset`).
    /// `events::nearest_scrollable_ancestor` walks the bubble chain from a wheel event's hit target
    /// looking for this flag.
    pub const SCROLLABLE: NodeFlags = NodeFlags(1 << 12);
    /// ⌨️ CSS `:focus-visible` in one bit: this node holds focus AND the gesture that gave it focus
    /// was a KEY, not a pointer. React gets the distinction free from the browser's own heuristic;
    /// `EventRouter` reproduces it by stamping this alongside [`NodeFlags::FOCUSED`] only while its
    /// `focus_visible` latch is set (set on `KeyDown`, cleared on `PointerDown`), so a clicked
    /// control takes focus without painting the accent ring React would only show for the keyboard.
    pub const FOCUS_VISIBLE: NodeFlags = NodeFlags(1 << 13);
    pub const DISCLOSURE_CHANGED: NodeFlags = NodeFlags(1 << 14);

    pub const fn empty() -> Self {
        NodeFlags(0)
    }

    pub fn set(&mut self, flag: NodeFlags, on: bool) {
        if on {
            self.0 |= flag.0;
        } else {
            self.0 &= !flag.0;
        }
    }

    pub fn contains(&self, flag: NodeFlags) -> bool {
        self.0 & flag.0 == flag.0
    }
}

/// 🧩️ Retained per-node widget spec. For M2 a thin clone of the last-applied `UiNode` (used as the
/// reconcile diff baseline); refined into per-variant retained fields in M4.
#[derive(Clone, Debug, PartialEq)]
pub struct WidgetSpec(pub UiNode);

/// ✍️ A focused editable text widget's live buffer (`events`' M5 key-routing writes here). Byte
/// offsets throughout (`caret`/`anchor`), not char indices: Rust string slicing/`replace_range` are
/// natively byte-indexed, and `events::{prev_char_boundary, next_char_boundary}` step these
/// safely across multi-byte UTF-8 without an O(n) char-counting pass on every keystroke. Selection
/// is `anchor..caret` in either order (mirrors the DOM `Selection` model: `anchor` is where the
/// selection started, `caret`/`focus` is the live end that arrow keys move).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct EditState {
    pub text: String,
    pub caret: usize,
    pub anchor: usize,
    /// 🈶️ IME preedit text, `Some` only mid-composition (`events::UiEvent::Ime`). Modeled so the
    /// shape is ready to receive real OS IME events; actually wiring winit's `Ime`/a hidden DOM
    /// input to this is a later `host`-region concern, out of `events`' scope.
    pub composition: Option<String>,
    pub scroll_x: f32,
}

/// 🎛️ Interactive per-node state that survives `reconcile::apply_tree` untouched (only `spec` is
/// ever overwritten by reconciliation — see `reconcile::diff_and_update`), which is exactly the
/// "focused buffer wins over a fresh incoming `value`" guarantee M5 `events` needs: as long as
/// `edit` stays `Some`, an `apply_tree` call re-diffing this node's declarative `value` never
/// touches it. `events::FocusState::set_focus` seeds `edit` from the widget's declarative value on
/// focus and clears it on blur, so external state governs again once editing ends.
#[derive(Clone, Debug, Default)]
pub struct WidgetState {
    pub edit: Option<EditState>,
    pub caret_visible: bool,
    pub slider_draft_value: Option<f64>,
    /// 🎚️ The open press of a continuous control (a dragged `Slider`, a typed number `Input` without a commit policy):
    /// the `gesture` every tick and the release of that press carry — the scrub protocol, `📓️api-scrub-machine.md` of
    /// ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING. `None` while no press is open.
    pub scrub_gesture: Option<String>,
    /// 🎚️ The value the open press last ticked: a blur releases the press on it when the field's draft is refused, as
    /// React's lane releases on the value last offered. `None` while no press is open.
    pub scrub_offered: Option<semio_framework_value::DslValue>,
    pub slider_readout_click_at: Option<f64>,
    /// 🚧️ The refusal a numeric control's kept draft earned (unreadable, or crossing a hard bound): the producer-localized
    /// text naming the bound, empty for an unlabelled one. While set the draft stays in `edit`, nothing is dispatched, and
    /// the control paints the draft in the error colour and the refusal where it has room; the next edit clears it.
    pub number_refusal: Option<String>,
    pub stepper_hovered_segment: Option<i8>,
    /// 🖱️ M5 `events` scroll routing's live offset for a `NodeFlags::SCROLLABLE` node.
    pub scroll_offset: (f32, f32),
    /// 🔽️ M5/W2 wiring: whether a `Select`'s synthesized popup (`reconcile::children_of`'s `Select`
    /// arm always builds the item rows unconditionally, per that module's own doc comment) is
    /// currently shown. Toggled by `events::EventRouter::dispatch`'s `Select`-click handling (via
    /// `open_overlay`/`close_overlay`, see `EventRouter::toggle_select_popup`/`finish_close`), read
    /// by `paint::paint_select` to decide whether to paint the popup at all.
    pub open: bool,
    pub disclosure_open: Option<bool>,
    /// ⌨️ Which option row of an OPEN `Select` the keyboard currently highlights — React's
    /// `aria-activedescendant`/`data-highlighted` row (`🧱️elements/🔽️Select/🟦️.tsx`'s `activeId`).
    /// Distinct from `NodeFlags::HOVERED` (a pointer fact) and from the Select's own committed
    /// `value`: arrowing through a popup moves this and nothing else, and only `Enter` commits.
    /// Written by `events::EventRouter::route_select_key`, read by `paint`'s popup rows.
    pub highlighted: Option<usize>,
    pub select_popup: Option<semio_framework_ui_select_behavior::SelectPopupGeometry>,
}

/// 📐️ Resolved rect from the last taffy layout pass, in the node's **parent-relative** coordinate
/// space (taffy's own `Layout::location`/`Layout::size` semantics — no extra transform needed when
/// consuming it; a later paint milestone accumulates ancestor offsets while walking the tree, same
/// as it already walks parent/child links for painting). `cached_text_measure` mirrors the last
/// `(text, wrap width bucket)` this node was measured at, so `flex::LayoutEngine` can skip
/// re-shaping an unchanged text node against an unchanged constraint.
/// 📏️ `(text, wrap width bucket)` key paired with its measured `(width, height)` — see
/// `LayoutBucket::cached_text_measure`.
pub type TextMeasureCache = Option<((String, Option<u32>), (f32, f32))>;

#[derive(Clone, Debug, Default)]
pub struct LayoutBucket {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub inline_suffix_width: f32,
    pub cached_text_measure: TextMeasureCache,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AcceptedLayout {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub inline_suffix_width: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(super) struct MountedLayoutRecord {
    pub(super) generation: u64,
    pub(super) layout: AcceptedLayout,
}

/// 🎨️ M4 decision: stays an empty marker. Every `paint::paint_*` function recomputes its `DrawList`
/// entries fresh from `spec`+`layout`+`theme` on each visit instead of caching tessellation output —
/// the composite widgets that would benefit most from caching (Select's open menu, Tree's rows)
/// aren't retained children yet (that's a later reconcile milestone), so there's nothing stable to
/// key a cache on without duplicating that future work; recomputing a handful of quads/glyph-runs
/// per dirty node per frame is cheap relative to the tessellation this replaces. Revisit once
/// composite expansion lands and glyph-run caching becomes worth the bookkeeping.
#[derive(Clone, Debug, Default)]
pub struct PaintBucket;

/// 🍃️ One retained tree node: tree links, identity, spec/state/layout/paint buckets, dirty flags.
pub struct Node {
    pub parent: Option<NodeId>,
    pub first_child: Option<NodeId>,
    pub last_child: Option<NodeId>,
    pub prev_sibling: Option<NodeId>,
    pub next_sibling: Option<NodeId>,
    pub(super) document_id: Option<UiNodeId>,
    pub key: NodeKey,
    pub spec: WidgetSpec,
    pub component_generation: u64,
    /// 📐️ The AUTHORED layout for this node — `Some` exactly when a `UiNodeRecord` published one
    /// (`record.layout`, the same value React's `layoutSpecStyle` reads), `None` for the legacy
    /// declarative `UiNode` chrome path that carries no layout vocabulary of its own. `flex` picks
    /// its dialect from this presence; see that region's header for why the two differ.
    pub layout_spec: Option<ui_contract::LayoutSpec>,
    pub state: WidgetState,
    pub layout: LayoutBucket,
    pub(super) mounted_layout: [MountedLayoutRecord; 2],
    pub paint: PaintBucket,
    pub flags: NodeFlags,
    /// 🎬️ What this node dispatches THROUGH: its published address plus one versioned `ActionId` per
    /// bound `Trigger`, stamped by the document reconcile's mount step and re-stamped on every
    /// revision. `None` for a node no document published (the chrome's immediate-mode widgets, the
    /// testkit's declarative `apply_tree` path) — such a node has no surface/revision to be stale
    /// against and keeps firing the bare `ActionDescriptor` its spec carries, exactly as React keeps
    /// `onAction` for its own unowned scene hosts.
    pub intent: Option<semio_framework_ui_action::UiIntentBindings>,
}

impl Node {
    pub fn new(key: NodeKey, spec: WidgetSpec) -> Self {
        let component_generation = u64::from(matches!(&spec.0, UiNode::ComponentScene(_)));
        let disclosure_open = match &spec.0 {
            UiNode::Section(section) => Some(section.default_open.unwrap_or(true)),
            _ => None,
        };
        Self {
            parent: None,
            first_child: None,
            last_child: None,
            prev_sibling: None,
            next_sibling: None,
            document_id: None,
            key,
            spec,
            component_generation,
            layout_spec: None,
            state: WidgetState { disclosure_open, ..WidgetState::default() },
            layout: LayoutBucket::default(),
            mounted_layout: [MountedLayoutRecord::default(); 2],
            paint: PaintBucket,
            flags: NodeFlags::empty(),
            intent: None,
        }
    }

    pub const fn component_generation(&self) -> u64 {
        self.component_generation
    }

    pub fn interaction_identity_matches(&self, other: &Self) -> bool {
        self.key == other.key
            && self.component_generation == other.component_generation
            && std::mem::discriminant(&self.spec.0) == std::mem::discriminant(&other.spec.0)
            && match (&self.spec.0, &other.spec.0) {
                (UiNode::ComponentScene(left), UiNode::ComponentScene(right)) => left.host_id == right.host_id,
                _ => true,
            }
    }
}
// #endregion tree
