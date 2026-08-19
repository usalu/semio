// #region flex
//! 📏️ Taffy-backed flex layout for the retained tree (`tree`/`reconcile`). Taffy's own types
//! (`taffy::TaffyTree`, `taffy::NodeId`, `taffy::Style`, …) are fully wrapped by `LayoutEngine` and
//! never appear in any item visible outside this crate — `LayoutEngine` itself is `pub(crate)`
//! (narrowest visibility that compiles) since only the retained `engine` façade needs it.
//! Style mapping reuses `layout::gap_for_token`/`layout::padding_for_token` (the old immediate-mode
//! `layout` region stays in place — `widgets`/`chrome` still call its `layout_vertical`/
//! `layout_horizontal` directly, so it isn't deleted this milestone). Pixel-parity requirements:
//! every child of a `Stack` gets `flex_grow: 1.0` so leftover main-axis space distributes equally
//! among siblings, matching the old hand-rolled stack layout's `extra_per_child` behaviour; a
//! `Field`'s sole synthetic child (`reconcile::children_of`) gets the same treatment so it fills the
//! label-adjusted remainder `widgets::render_widget`'s `WidgetNode::Field` branch carves out (see
//! `apply_field_metrics`) — a `Section`'s synthetic children deliberately do *not* grow (see
//! `apply_section_metrics`), since `WidgetNode::Section` stacks them at their own intrinsic size with
//! no `extra_per_child`-style redistribution, unlike a `Stack`'s or `Field`'s.

use std::collections::HashMap;

use taffy::prelude::*;

use crate::wgpu::arena::NodeId;
use crate::wgpu::component::ui::UiNode;
use crate::wgpu::layout::{gap_for_token, padding_for_token};
use crate::wgpu::text::FontAtlas;
use crate::wgpu::theme::Theme;
use crate::wgpu::tree::{NodeFlags, UiTree};

/// 🖋️ Default text size (px) used for intrinsic measurement during layout, ahead of the per-node
/// resolved style a later paint milestone introduces.
const DEFAULT_TEXT_SIZE_PX: f32 = 14.0;

/// 🍃️ Per-taffy-leaf context: which retained nodes need a measure callback (only `Text`) and which
/// don't (everything else measures as zero-size content, matching the pre-taffy immediate-mode
/// widgets that size themselves from fixed control-height/theme metrics rather than intrinsic text).
enum LeafContext {
    None,
    Text(String),
}

/// 🖇️ Reads intrinsic content size for taffy's leaf-measurement callback. Implemented for
/// `text::FontAtlas` so taffy can ask fontdue for wrap-aware text metrics without ui_wgpu's flex
/// module depending on fontdue directly.
pub(crate) trait TextMeasure {
    async fn measure(&mut self, text: &str, max_width: Option<f32>) -> (f32, f32);
}

impl TextMeasure for FontAtlas {
    async fn measure(&mut self, text: &str, max_width: Option<f32>) -> (f32, f32) {
        match max_width {
            Some(width) if width > 0.0 => self.measure_text_wrapped(text, width, DEFAULT_TEXT_SIZE_PX),
            _ => self.measure_text(text, DEFAULT_TEXT_SIZE_PX),
        }
    }
}

async fn quantize_width(width: Option<f32>) -> Option<u32> {
    width.map(|w| w.round().max(0.0) as u32)
}

/// 📦️ Maps a retained node's `WidgetSpec` to a taffy `Style`. `Stack` becomes a real flex container
/// (direction/gap/padding from its own fields); `Field`/`Section` become a column flex container too
/// (their reconciled synthetic child(ren) need real flexbox participation to match `widgets`' hand-
/// rolled geometry — see `apply_field_metrics`/`apply_section_metrics`), zeroed here since both are
/// theme-dependent; every other variant is a content leaf (auto-sized, measured via `LeafContext`
/// where applicable). `flex_grow` is layered on top by the caller for children of a `Stack`/`Field`,
/// not set here, since it depends on the *parent's* kind.
async fn style_for(node: &UiNode) -> Style {
    match node {
        UiNode::Stack(stack) => {
            let vertical = stack.direction != "horizontal";
            Style {
                display: Display::Flex,
                flex_direction: if vertical { FlexDirection::Column } else { FlexDirection::Row },
                gap: Size { width: length(0.0_f32), height: length(0.0_f32) },
                padding: Rect { left: length(0.0_f32), right: length(0.0_f32), top: length(0.0_f32), bottom: length(0.0_f32) },
                size: Size { width: auto(), height: auto() },
                ..Default::default()
            }
        }
        UiNode::Field(_) | UiNode::Section(_) => Style {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            gap: Size { width: length(0.0_f32), height: length(0.0_f32) },
            padding: Rect { left: length(0.0_f32), right: length(0.0_f32), top: length(0.0_f32), bottom: length(0.0_f32) },
            size: Size { width: auto(), height: auto() },
            ..Default::default()
        },
        _ => Style { size: Size { width: auto(), height: auto() }, ..Default::default() },
    }
}

/// 🎚️ Applies `theme`-resolved gap/padding onto a freshly built `Stack` style (kept separate from
/// `style_for` so the latter stays theme-independent and trivially testable).
async fn apply_stack_metrics(style: &mut Style, stack: &crate::wgpu::component::ui::UiStackNode, theme: &Theme) {
    let gap = gap_for_token(theme, stack.gap.as_deref());
    let padding = padding_for_token(theme, stack.padding.as_deref());
    style.gap = Size { width: length(gap), height: length(gap) };
    style.padding = Rect { left: length(padding), right: length(padding), top: length(padding), bottom: length(padding) };
}

/// 🎚️ `widgets::render_widget`'s `WidgetNode::Field` branch: `Rect::new(bounds.x, bounds.y + label_h
/// + gap, bounds.w, bounds.h - label_h - gap)`, where `label_h = theme.font_size_small` and
/// `gap = gap_for_token(theme, Some("standard"))`. Reserving that same top padding on `Field`'s own
/// taffy container, combined with `style_with_grow` granting its sole child `flex_grow: 1.0`, resolves
/// that child to the identical rect taffy-side (default `align_items: Stretch` already matches the
/// full `bounds.w`, since `Field`'s container has no left/right padding).
async fn apply_field_metrics(style: &mut Style, theme: &Theme) {
    let label_h = theme.font_size_small;
    let gap = gap_for_token(theme, Some("standard"));
    style.padding.top = length(label_h + gap);
}

/// 🔖️ Mirrors `widgets`'/`paint`'s own private `PANEL_HEADER` constant: the header-row height
/// `WidgetNode::Section`'s branch reserves for its content unconditionally (`y = bounds.y +
/// PANEL_HEADER`, even when `label` is `None` — only the header's chevron+text *paint* is gated on
/// `label.is_some()`, not this offset).
const SECTION_HEADER_HEIGHT: f32 = 24.0;

/// 🎚️ `WidgetNode::Section`'s branch stacks its children with a plain `y += h + ctx.theme.gap_standard`
/// loop — each kept at its own intrinsic size, never `layout_vertical`'s `extra_per_child` leftover
/// redistribution (unlike a `Stack`'s or `Field`'s child; see `apply_field_metrics`). Reserving the
/// header offset as top padding and `theme.gap_standard` as the inter-row gap reproduces that
/// positioning without granting `flex_grow` — `style_with_grow`'s `flex_grow_child` gate deliberately
/// stays `Stack`/`Field`-only.
async fn apply_section_metrics(style: &mut Style, theme: &Theme) {
    style.padding.top = length(SECTION_HEADER_HEIGHT);
    style.gap = Size { width: length(0.0_f32), height: length(theme.gap_standard) };
}

async fn leaf_context(node: &UiNode) -> LeafContext {
    match node {
        UiNode::Text(text) => LeafContext::Text(text.value.clone().into_string()),
        _ => LeafContext::None,
    }
}

/// 🧮️ `taffy::NodeId` lookup for one retained `UiTree` (not authoritative UI state).
struct TaffyNodeMapping {
    by_ui: HashMap<NodeId, taffy::NodeId>,
}

/// 🧮️ Owns a taffy flexbox tree mirroring one retained `UiTree` and the `NodeId -> taffy::NodeId`
/// mapping between them. Used only by the `engine` façade (a later milestone); never exposed
/// outside the crate.
pub(crate) struct LayoutEngine {
    taffy: TaffyTree<LeafContext>,
    nodes: TaffyNodeMapping,
}

impl Default for LayoutEngine {
    fn default() -> Self {
        Self { taffy: TaffyTree::new(), nodes: TaffyNodeMapping { by_ui: HashMap::new() } }
    }
}

impl LayoutEngine {
    pub(crate) async fn new() -> Self {
        Self::default()
    }

    /// 🔁️ Runs a taffy layout pass over `tree` rooted at `root` if (and only if) anything in the
    /// tree carries `DIRTY_LAYOUT`/`SUBTREE_DIRTY` (checking the root alone suffices — `mark_dirty`
    /// always bubbles `SUBTREE_DIRTY` to the root, so an all-clean tree is a single flag read, no
    /// walk). Returns whether a layout pass actually ran.
    pub(crate) async fn compute(&mut self, tree: &mut UiTree, root: NodeId, atlas: &mut FontAtlas, theme: &Theme, available_width: f32, available_height: f32) -> bool {
        let Some(root_node) = tree.node(root) else { return false };
        let needs_layout = root_node.flags.contains(NodeFlags::DIRTY_LAYOUT) || root_node.flags.contains(NodeFlags::SUBTREE_DIRTY);
        if !needs_layout {
            return false;
        }

        self.prune_removed(tree);
        let root_taffy = self.sync(tree, theme, root, false);

        let mut root_style = self.taffy.style(root_taffy).cloned().unwrap_or_default();
        root_style.size = Size { width: length(available_width), height: length(available_height) };
        let _ = self.taffy.set_style(root_taffy, root_style);

        let available = Size { width: AvailableSpace::Definite(available_width), height: AvailableSpace::Definite(available_height) };
        let _ = self.taffy.compute_layout_with_measure(root_taffy, available, |known_dimensions, available_space, _node_id, node_context, _style| {
            if let (Some(width), Some(height)) = (known_dimensions.width, known_dimensions.height) {
                return Size { width, height };
            }
            match node_context {
                Some(LeafContext::Text(text)) => {
                    let max_width = known_dimensions.width.or_else(|| available_space.width.into_option());
                    let (measured_w, measured_h) = atlas.measure(text, max_width);
                    Size { width: known_dimensions.width.unwrap_or(measured_w), height: known_dimensions.height.unwrap_or(measured_h) }
                }
                _ => Size::ZERO,
            }
        });

        self.write_back(tree, atlas, root);
        true
    }

    /// 🧹️ Drops taffy-side nodes whose retained counterpart no longer exists (removed by
    /// `reconcile`), so the mapping doesn't grow unbounded across the tree's lifetime.
    async fn prune_removed(&mut self, tree: &UiTree) {
        let stale: Vec<NodeId> = self.nodes.by_ui.keys().copied().filter(|id| !tree.contains(*id)).collect();
        for id in stale {
            if let Some(taffy_id) = self.nodes.by_ui.remove(&id) {
                let _ = self.taffy.remove(taffy_id);
            }
        }
    }

    /// 🌲️ Depth-first: ensures every retained node reachable from `id` has a taffy counterpart,
    /// refreshing style/children only for nodes that are new or carry `DIRTY_LAYOUT` (everything
    /// else keeps its existing taffy node untouched, letting taffy's own layout cache skip
    /// recomputation for genuinely unchanged subtrees). `flex_grow_child` is true when `id`'s parent
    /// is a `Stack` or `Field` — the two kinds whose child(ren) should grow to fill leftover space
    /// (a `Section`'s children deliberately don't; see `apply_section_metrics`).
    async fn sync(&mut self, tree: &UiTree, theme: &Theme, id: NodeId, flex_grow_child: bool) -> taffy::NodeId {
        let node = tree.node(id).expect("sync called with a live NodeId");
        let grows_children = matches!(node.spec.0, UiNode::Stack(_) | UiNode::Field(_));
        let dirty = node.flags.contains(NodeFlags::DIRTY_LAYOUT);
        let existing = self.nodes.by_ui.get(&id).copied();

        let children: Vec<NodeId> = tree.children(id).collect();
        let child_taffy_ids: Vec<taffy::NodeId> = children.iter().map(|&child_id| self.sync(tree, theme, child_id, grows_children)).collect();

        let taffy_id = match existing {
            Some(taffy_id) if !dirty => taffy_id,
            Some(taffy_id) => {
                let style = self.style_with_grow(&node.spec.0, theme, flex_grow_child);
                let _ = self.taffy.set_style(taffy_id, style);
                taffy_id
            }
            None => {
                let style = self.style_with_grow(&node.spec.0, theme, flex_grow_child);
                self.taffy.new_leaf_with_context(style, leaf_context(&node.spec.0)).expect("taffy leaf insert")
            }
        };
        if existing.is_none() || dirty {
            let _ = self.taffy.set_children(taffy_id, &child_taffy_ids);
        }
        self.nodes.by_ui.insert(id, taffy_id);
        taffy_id
    }

    async fn style_with_grow(&self, node: &UiNode, theme: &Theme, flex_grow_child: bool) -> Style {
        let mut style = style_for(node);
        match node {
            UiNode::Stack(stack) => apply_stack_metrics(&mut style, stack, theme),
            UiNode::Field(_) => apply_field_metrics(&mut style, theme),
            UiNode::Section(_) => apply_section_metrics(&mut style, theme),
            _ => {}
        }
        if flex_grow_child {
            style.flex_grow = 1.0;
        }
        style
    }

    /// 📝️ Copies taffy's resolved `location`/`size` into each node's `LayoutBucket` (parent-relative,
    /// same space taffy itself uses — see `tree::LayoutBucket`'s doc comment) and clears
    /// `DIRTY_LAYOUT`. Text nodes also get their `cached_text_measure` refreshed at the node's final
    /// resolved width, so a following unchanged-constraint measurement is a cache hit.
    async fn write_back(&mut self, tree: &mut UiTree, atlas: &mut FontAtlas, id: NodeId) {
        if let Some(&taffy_id) = self.nodes.by_ui.get(&id) {
            if let Ok(layout) = self.taffy.layout(taffy_id) {
                let (x, y, width, height) = (layout.location.x, layout.location.y, layout.size.width, layout.size.height);
                let text_value = match tree.node(id).map(|n| &n.spec.0) {
                    Some(UiNode::Text(text)) => Some(text.value.clone().into_string()),
                    _ => None,
                };
                if let Some(node) = tree.node_mut(id) {
                    node.layout.x = x;
                    node.layout.y = y;
                    node.layout.width = width;
                    node.layout.height = height;
                    node.flags.set(NodeFlags::DIRTY_LAYOUT, false);
                    // write_back always walks the whole subtree from `root`, so by the time it
                    // finishes every descendant is up to date and SUBTREE_DIRTY can clear too —
                    // otherwise it would never clear and `compute`'s early-out would never fire.
                    node.flags.set(NodeFlags::SUBTREE_DIRTY, false);
                }
                if let Some(value) = text_value {
                    let key = (value.clone(), quantize_width(Some(width)));
                    let already_cached = tree.node(id).and_then(|n| n.layout.cached_text_measure.as_ref().map(|(k, _)| k.clone())) == Some(key.clone());
                    if !already_cached {
                        let measured = atlas.measure(&value, Some(width));
                        if let Some(node) = tree.node_mut(id) {
                            node.layout.cached_text_measure = Some((key, measured));
                        }
                    }
                }
            }
        }
        let children: Vec<NodeId> = tree.children(id).collect();
        for child in children {
            self.write_back(tree, atlas, child);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::wgpu::component::ui::{UiFieldNode, UiPresence, UiSectionNode, UiStackNode, UiTextNode};

    async fn text(value: &str) -> UiNode {
        UiNode::Text(UiTextNode { value: value.into(), emphasize: None, data_attributes: None, presence: UiPresence::default(), menu: None })
    }

    async fn stack(direction: &str, children: Vec<UiNode>) -> UiNode {
        UiNode::Stack(UiStackNode { direction: direction.into(), gap: Some("none".into()), padding: Some("none".into()), id: None, presence: UiPresence::default(), activate: None, drop_action: None, drop_overlay: None, children, menu: None })
    }

    #[test]
    async fn vertical_stack_lays_children_top_to_bottom_with_correct_y_offsets() {
        let mut tree = UiTree::new();
        tree.apply_tree(&stack("vertical", vec![text("hello"), text("a longer line of text")]));
        let root = tree.root.unwrap();
        let mut engine = LayoutEngine::new();
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();

        let ran = engine.compute(&mut tree, root, &mut atlas, &theme, 400.0, 400.0);
        assert!(ran);

        let children: Vec<NodeId> = tree.children(root).collect();
        assert_eq!(children.len(), 2);
        let first = &tree.node(children[0]).unwrap().layout;
        let second = &tree.node(children[1]).unwrap().layout;
        assert_eq!(first.y, 0.0);
        assert!(second.y >= first.y + first.height);
    }

    #[test]
    async fn horizontal_stack_distributes_equal_leftover_width_across_children() {
        let mut tree = UiTree::new();
        let children = vec![
            UiNode::Separator(crate::wgpu::component::ui::UiSeparatorNode { presence: UiPresence::default(), menu: None }),
            UiNode::Separator(crate::wgpu::component::ui::UiSeparatorNode { presence: UiPresence::default(), menu: None }),
            UiNode::Separator(crate::wgpu::component::ui::UiSeparatorNode { presence: UiPresence::default(), menu: None }),
        ];
        tree.apply_tree(&stack("horizontal", children));
        let root = tree.root.unwrap();
        let mut engine = LayoutEngine::new();
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();

        engine.compute(&mut tree, root, &mut atlas, &theme, 300.0, 100.0);

        let widths: Vec<f32> = tree.children(root).map(|id| tree.node(id).unwrap().layout.width).collect();
        assert_eq!(widths.len(), 3);
        for w in &widths {
            assert!((*w - 100.0).abs() < 0.5, "expected equal-thirds width, got {w}");
        }
    }

    #[test]
    async fn recomputing_with_nothing_dirty_is_a_no_operation() {
        let mut tree = UiTree::new();
        tree.apply_tree(&stack("vertical", vec![text("hello")]));
        let root = tree.root.unwrap();
        let mut engine = LayoutEngine::new();
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();

        assert!(engine.compute(&mut tree, root, &mut atlas, &theme, 200.0, 200.0));
        let first_pass = tree.node(root).unwrap().layout.clone();

        let ran_again = engine.compute(&mut tree, root, &mut atlas, &theme, 200.0, 200.0);
        assert!(!ran_again, "DIRTY_LAYOUT/SUBTREE_DIRTY are cleared after a pass, so a second call must early-out");
        let second_pass = tree.node(root).unwrap().layout.clone();
        assert_eq!((first_pass.x, first_pass.y, first_pass.width, first_pass.height), (second_pass.x, second_pass.y, second_pass.width, second_pass.height));
    }

    #[test]
    async fn text_measurement_is_cached_per_unchanged_width() {
        let mut atlas = FontAtlas::builtin();
        let first = atlas.measure("hello world", Some(120.0));
        let second = atlas.measure("hello world", Some(120.0));
        assert_eq!(first, second);

        let mut tree = UiTree::new();
        tree.apply_tree(&stack("vertical", vec![text("cache me")]));
        let root = tree.root.unwrap();
        let mut engine = LayoutEngine::new();
        let theme = Theme::default();
        engine.compute(&mut tree, root, &mut atlas, &theme, 200.0, 200.0);

        let child = tree.children(root).next().unwrap();
        let cached = tree.node(child).unwrap().layout.cached_text_measure.clone();
        assert!(cached.is_some(), "text node should have a cached measurement after layout");
    }

    //#region 🔖️FieldSectionGrowSemantics
    #[test]
    async fn field_child_grows_to_fill_the_label_adjusted_remainder() {
        let mut tree = UiTree::new();
        let field = UiNode::Field(UiFieldNode { id: "f".into(), label: "Label".into(), description: None, required: None, error: None, child: Box::new(text("child")), presence: UiPresence::default(), menu: None });
        tree.apply_tree(&field);
        let root = tree.root.unwrap();
        let mut engine = LayoutEngine::new();
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();

        engine.compute(&mut tree, root, &mut atlas, &theme, 200.0, 100.0);

        let child = tree.children(root).next().unwrap();
        let layout = &tree.node(child).unwrap().layout;
        let label_h = theme.font_size_small;
        let gap = gap_for_token(&theme, Some("standard"));
        assert!((layout.y - (label_h + gap)).abs() < 0.5, "child should start below the label, got y={}", layout.y);
        assert!((layout.height - (100.0 - label_h - gap)).abs() < 0.5, "child should fill the label-adjusted remainder, got height={}", layout.height);
        assert!((layout.width - 200.0).abs() < 0.5, "child should stretch to the field's full width, got width={}", layout.width);
    }

    #[test]
    async fn section_children_stack_below_the_header_at_their_own_intrinsic_height_with_gap() {
        let mut tree = UiTree::new();
        let section = UiNode::Section(UiSectionNode { id: "s".into(), label: Some("Section".into()), default_open: Some(true), presence: UiPresence::default(), children: vec![text("a"), text("a longer line of text")], menu: None });
        tree.apply_tree(&section);
        let root = tree.root.unwrap();
        let mut engine = LayoutEngine::new();
        let mut atlas = FontAtlas::builtin();
        let theme = Theme::default();

        engine.compute(&mut tree, root, &mut atlas, &theme, 200.0, 200.0);

        let children: Vec<NodeId> = tree.children(root).collect();
        assert_eq!(children.len(), 2);
        let first = tree.node(children[0]).unwrap().layout.clone();
        let second = tree.node(children[1]).unwrap().layout.clone();
        assert!((first.y - SECTION_HEADER_HEIGHT).abs() < 0.5, "first child should start right below the header offset, got y={}", first.y);
        assert!((second.y - (first.y + first.height + theme.gap_standard)).abs() < 0.5, "second child should sit one gap below the first child's own intrinsic height, got y={} first.height={}", second.y, first.height);
        assert!((first.width - 200.0).abs() < 0.5, "children should stretch to the section's full width, got width={}", first.width);
    }
    //#endregion 🔖️FieldSectionGrowSemantics
}
// #endregion flex
