//! 🪟️ Surface tree assembly: built node to runtime tree transfer, paged text carriers, scene surfaces and window kind definitions.
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_os_kernel as pack;
use semio_framework::{ActionDefinition, WindowKindDefinition};
use semio_ui_contract::BuiltNode;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-ui-contract`
/// and `semio-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_ui_contract::*;
use semio_ui_runtime::{ComponentTree, TreeNode};
use semio_ui_locale::LocalizedLabel;
use semio_ui_layout::layout::WindowOptions;
use semio_plugin_ownership::sdk_fault::PluginAssemblyError;



/// 🌉️ Transfers the contract builder's fixed node owner to the runtime tree without a recursive
/// copy or a second uncensused tree allocation.
pub fn built_to_tree(node: BuiltNode) -> TreeNode {
    node
}

/// 🌉️ Transfers a complete fixed root into the runtime tree owner.
pub fn built_to_component_tree(node: BuiltNode) -> ComponentTree {
    ComponentTree::new(built_to_tree(node))
}

/// 🧩️ One numbered text leaf of a paged text carrier. The key is explicit rather than positional
/// because `try_build` stamps every unparented builder with the SAME `"#0"` fallback, which siblings
/// may not share.
// 🚫️async: E1 pure node construction consumed synchronously by `section_text_chunks` — see R9.
fn section_leaf(value: Label, index: usize, attributes: Option<UiFixedMap<UiText>>) -> UiAssemblyResult<BuiltNode> {
    let mut node = text(value).try_id(format!("c{index}")).map_err(|_| ui_assembly_error("section-chunk-id"))?.try_build().map_err(|_| ui_assembly_error("section-chunk-build"))?;
    if let (Some(attributes), Component::Text(props)) = (attributes, &mut node.component) {
        props.data_attributes = Some(attributes);
    }
    Ok(node)
}

/// 🧩️ Packs UTF-8 slices that each fit `UI_TEXT_MAX_BYTES` into ONE text node: the first slice is
/// `TextProps.value`, the rest ride as ascending `data_attributes` (`01`..`32`) so a leaf holds
/// `UI_TEXT_MAX_BYTES * (1 + UI_FIXED_LIST_ITEMS)` bytes without another tree node. A 512-byte
/// leaf per slice made Nakagin's instances lane ~112 nodes and the world surface one over
/// `UI_DOCUMENT_NODES`.
// 🚫️async: E1 pure in-memory projection consumed synchronously by `section_component_tree` — see R9.
fn section_text_chunks(payload: &str) -> UiAssemblyResult<Vec<BuiltNode>> {
    let pack = 1 + UI_FIXED_LIST_ITEMS;
    let mut slices = Vec::with_capacity(payload.len().div_ceil(UI_TEXT_MAX_BYTES).max(1));
    let mut start = 0;
    while start < payload.len() {
        let mut end = (start + UI_TEXT_MAX_BYTES).min(payload.len());
        while end > start && !payload.is_char_boundary(end) {
            end -= 1;
        }
        if end == start {
            return Err(ui_assembly_error("section-chunk-boundary"));
        }
        slices.push(payload[start..end].to_string());
        start = end;
    }
    if slices.is_empty() {
        slices.push(String::new());
    }
    let mut chunks = Vec::with_capacity(slices.len().div_ceil(pack));
    for pack_slices in slices.chunks(pack) {
        let Ok(value) = Label::try_from(pack_slices[0].clone()) else { return Err(ui_assembly_error("section-chunk-label")) };
        let mut attributes = UiFixedMap::default();
        for (offset, slice) in pack_slices.iter().enumerate().skip(1) {
            let Some(key) = UiText::try_from_str(&format!("{offset:02}")) else { return Err(ui_assembly_error("section-chunk-attr-key")) };
            let Some(attr) = UiText::try_from_str(slice) else { return Err(ui_assembly_error("section-chunk-attr")) };
            attributes.try_push(key, attr).map_err(|_| ui_assembly_error("section-chunk-attr-order"))?;
        }
        chunks.push(section_leaf(value, chunks.len(), if attributes.is_empty() { None } else { Some(attributes) })?);
    }
    Ok(chunks)
}

/// 🪪️ Names one complete publication of a semantic input value.
pub fn input_value_body_key(value: &str) -> String {
    format!("framework.input.value.{}.{}", value.len(), semio_ui_scene::scene_lane_hash(value))
}

/// 📏️ Upper bound on the record count of an owned carrier, including its root.
pub fn paged_text_carrier_nodes(payload: &str) -> usize {
    let width = (UI_TEXT_MAX_BYTES - 3) * (1 + UI_FIXED_LIST_ITEMS);
    let mut pages = payload.len().div_ceil(width).max(1);
    let mut nodes = 1 + pages;
    while pages > UI_BUILT_CHILDREN_MAX {
        pages = pages.div_ceil(UI_BUILT_CHILDREN_MAX);
        nodes += pages;
    }
    nodes
}

/// 🧩️ Builds ONE paged text carrier: a root container keyed by `root_key`, over a balanced
/// `UI_BUILT_CHILDREN_MAX`-ary tree of packed text leaves whose depth-first concatenation is
/// `payload`.
///
/// The payload rides as text rather than as a `Component::Extension`'s `UiValue` because
/// `UiValue`'s owned collections are capped at `UI_VALUE_MAX_ITEMS` entries each and are credited
/// against the ONE live arena page (`UI_VALUE_LIVE_PAGES`) the virtualised panel author needs — a
/// whole measures map would either fail admission or starve that author. Each leaf is still a
/// node so the reconciler pages it through `SURFACE_RECONCILE_PAGE_BYTES`, but packing
/// `data_attributes` keeps the node envelope `O(lanes + ceil(bytes / pack))` so a Nakagin-scale
/// world surface stays inside `UI_DOCUMENT_NODES`.
///
/// Two carriers share this one shape: a reserved refresh section (`section_component_tree`) and a
/// world-3d scene lane (`scene_surface`'s out-of-doc payloads). Both are bounded by their number
/// of packed pages, never by a fixed byte ceiling.
// 🚫️async: E1 pure in-memory projection called from `plugin_render_surface`'s locked instance — see R9.
pub fn paged_text_carrier(root_key: &str, payload: &str) -> UiAssemblyResult<BuiltNode> {
    let mut level = section_text_chunks(payload)?;
    let mut generation = 0usize;
    while level.len() > UI_BUILT_CHILDREN_MAX {
        let mut source = level.into_iter();
        let mut next = Vec::new();
        loop {
            let page: Vec<BuiltNode> = source.by_ref().take(UI_BUILT_CHILDREN_MAX).collect();
            if page.is_empty() {
                break;
            }
            let id = format!("p{generation}-{}", next.len());
            next.push(column().try_id(id).map_err(|_| ui_assembly_error("section-page-id"))?.try_children(page).map_err(|_| ui_assembly_error("section-page"))?.try_build().map_err(|_| ui_assembly_error("section-page-build"))?);
        }
        level = next;
        generation += 1;
    }
    column().try_id(root_key).map_err(|_| ui_assembly_error("section-root-id"))?.try_children(level).map_err(|_| ui_assembly_error("section-root"))?.try_build().map_err(|_| ui_assembly_error("section-root-build"))
}

/// 🖼️ Builds an accessible image with an inline or retained source URI.
pub fn image_with_source(id: &str, source: &str, alt: Label) -> UiAssemblyResult<BuiltNode> {
    if let Some(src) = UiText::try_from_str(source) {
        return image(src).alt(alt).try_id(id).map_err(|_| ui_assembly_error("image.id"))?
            .try_build().map_err(|_| ui_assembly_error("image.build"));
    }
    let key = format!("framework.ui.image.source.{}", semio_ui_scene::scene_lane_hash(source));
    image_with_retained_source(id, &key, source, alt)
}

fn image_with_retained_source(id: &str, key: &str, source: &str, alt: Label) -> UiAssemblyResult<BuiltNode> {
    let src = UiText::try_from_str(key).ok_or_else(|| ui_assembly_error("image.source-key"))?;
    let bytes = u32::try_from(source.len()).map_err(|_| ui_assembly_error("image.source-size"))?;
    let carrier = paged_text_carrier(key, source)?;
    image(src).alt(alt).try_id(id).map_err(|_| ui_assembly_error("image.id"))?
        .try_source_body(bytes, carrier).map_err(|_| ui_assembly_error("image.source-body"))?
        .try_build().map_err(|_| ui_assembly_error("image.build"))
}

/// 🪟️ Addresses resource requests to the rendered body without changing the visible form page.
pub struct WindowResourceRequests<'a> {
    pub window_id: &'a str,
    pub body_key: &'a str,
    pub requests: &'a [semio_framework::TreeWindowRequest],
}

impl WindowResourceRequests<'_> {
    fn paged(source: &str, inline_carrier_nodes: usize) -> bool {
        source.starts_with("data:image/") && source.len() <= window_resource::WINDOW_RESOURCE_MAX_BYTES
            && paged_text_carrier_nodes(source) > inline_carrier_nodes
    }

    /// 📏️ Bounds the retained nodes owned by this image's pending page.
    pub fn image_body_nodes(source: &str) -> usize {
        Self::image_body_nodes_up_to(source, TREE_WINDOW_BODY_NODE_BUDGET - 3)
    }

    /// 📐️ Prices the retained carrier with the caller's stable inline-node allowance.
    pub fn image_body_nodes_up_to(source: &str, inline_carrier_nodes: usize) -> usize {
        if UiText::try_from_str(source).is_some() { return 0; }
        if !Self::paged(source, inline_carrier_nodes) { return paged_text_carrier_nodes(source); }
        Self::image_page_nodes()
    }

    /// 📄️ Maximum carrier records belonging to one exact resource page.
    pub fn image_page_nodes() -> usize {
        let leaf_width = (UI_TEXT_MAX_BYTES - 3) * (1 + UI_FIXED_LIST_ITEMS);
        1 + (window_resource::WINDOW_RESOURCE_PAGE_BYTES + 3).div_ceil(leaf_width)
    }

    /// 🖼️ Publishes an exact revision-bound image page for the requested window.
    pub fn image(&self, id: &str, source: &str, alt: Label) -> UiAssemblyResult<BuiltNode> {
        self.image_up_to(id, source, alt, TREE_WINDOW_BODY_NODE_BUDGET - 3)
    }

    /// 🎞️ Chooses the inline carrier or a resource page using a stable caller allowance.
    pub fn image_up_to(&self, id: &str, source: &str, alt: Label, inline_carrier_nodes: usize) -> UiAssemblyResult<BuiltNode> {
        if !Self::paged(source, inline_carrier_nodes) { return image_with_source(id, source, alt); }
        let resource = window_resource::WindowResource::new(self.window_id, id, source).ok_or_else(|| ui_assembly_error("image.source-size"))?;
        let key = resource.request_key();
        let index = self.requests.iter().find(|request| request.body_key == self.body_key && request.node_key == key && request.open.is_none() && request.rows == 1)
            .map_or(1, |request| request.offset.clamp(1, resource.pages()));
        let page = window_resource::WindowResourcePage { resource, index };
        image_with_retained_source(id, &page.key(), page.text(source).ok_or_else(|| ui_assembly_error("image.source-page"))?, alt)
    }
}

/// 🧩️ Builds one reserved refresh section's retained surface tree — [`paged_text_carrier`] rooted
/// at the section's own reserved body key.
// 🚫️async: E1 pure in-memory projection called from `plugin_render_surface`'s locked instance — see R9.
pub fn section_component_tree(section: semio_framework::UiRefreshSection, payload: &str) -> UiAssemblyResult<ComponentTree> {
    paged_text_carrier(section.body_key(), payload).map(built_to_component_tree)
}

/// 🎬️ Encodes a typed product scene behind the semantic surface contract: the scene's SPINE inside
/// the fixed-capacity `SurfaceProps.doc`, and every payload lane the scene declares
/// (`SceneDoc::split_lanes`) beside it as its own [`paged_text_carrier`] child of the surface node.
///
/// A scene that declares no lanes (the default — a bounded per-frame descriptor like
/// `Canvas2dScene`) still publishes exactly one childless surface node, byte-identical to what
/// this produced before lanes existed. A scene that does declare them (`World3dScene`) is bounded
/// by its number of carrier pages instead of by `UI_FIXED_BYTES` — which is what a 57 281-byte
/// Nakagin world needs, since the 32 KiB `UiFixedBytes` doc cannot page and refused it outright.
pub fn scene_surface<T: semio_ui_scene::SceneDoc>(id: &str, kind: SurfaceKind, scene: &T) -> UiAssemblyResult<BuiltNode> {
    let (spine, lanes) = scene.split_lanes();
    let props = semio_ui_scene::encode(kind, &spine).map_err(|error| ui_assembly_error_because("scene-surface.encode", error))?;
    let carriers = lanes.iter().map(|lane| paged_text_carrier(lane.key, &lane.payload)).collect::<UiAssemblyResult<Vec<BuiltNode>>>()?;
    surface(props).try_id(id).map_err(|_| ui_assembly_error("scene-surface.id"))?.try_children(carriers).map_err(|_| ui_assembly_error("scene-surface.lanes"))?.try_build().map_err(|_| ui_assembly_error("scene-surface.build"))
}

/// 🏷️ Builds one semantic text node from the SDK's retained non-node label type.
pub fn built_text_node(label: semio_ui_locale::Label) -> Result<BuiltNode, semio_ui_locale::Label> {
    let value = label.to_string();
    let Ok(value) = Label::try_from(value) else { return Err(label) };
    text(value).try_build().map_err(|_| label)
}

/// 🧩️ Admits a dynamic text root into the fixed-capacity UI tree and preserves admission
/// failure at the plugin assembly boundary.
pub fn built_text_to_component_tree(label: semio_ui_locale::Label) -> UiAssemblyResult<ComponentTree> {
    built_text_node(label).map(built_to_component_tree).map_err(|_| ui_assembly_error("built-text-root"))
}

pub use semio_plugin_interaction::{ui_assembly_error, UiAssemblyResult};

/// 🩺️ [[ui_assembly_error]] with the refusing cause preserved. `SurfaceEncodeError` already
/// distinguishes a pack failure from a payload-capacity overflow from a schema overflow, and
/// discarding it leaves every fixed-capacity refusal reporting the same stage name — indistinguishable
/// at runtime and unactionable in a browser console.
#[doc(hidden)]
pub fn ui_assembly_error_because(stage: &'static str, cause: impl std::fmt::Display) -> PluginAssemblyError {
    PluginAssemblyError::new("ui.fixed-capacity", format!("fixed UI admission failed at {stage}: {cause}"))
}

#[doc(hidden)]
pub fn ui_label<T: TryInto<Label>>(value: T, stage: &'static str) -> UiAssemblyResult<Label> {
    value.try_into().map_err(|_| ui_assembly_error(stage))
}

#[doc(hidden)]
pub fn ui_text<T: TryInto<UiText>>(value: T, stage: &'static str) -> UiAssemblyResult<UiText> {
    value.try_into().map_err(|_| ui_assembly_error(stage))
}

#[cfg(test)]
include!("../🧪️tests/🔬️app-tree-convert/🦀️.rs");

/// 🔀️ Bridges the two SurfaceKind enums that currently coexist: the contract owns the semantic
/// one, while the ui crate still carries the original for the window-kit and descriptor paths
/// this program has not migrated yet. Both carry the same 15 variants with the same serde names,
/// so the mapping goes through that shared wire name and is total rather than lossy. It is
/// deleted with the legacy enum in the deletion wave; it exists because two names for one concept
/// briefly overlap, not as a compatibility layer.
// 🚫️async: E1 pure total mapping consumed by sync struct-literal call sites — see R9.
#[doc(hidden)]
pub fn contract_surface_kind(kind: semio_ui_declaration::ui::SurfaceKind) -> SurfaceKind {
    let name = serde_json::to_string(&kind).unwrap_or_default();
    serde_json::from_str(&name).unwrap_or(SurfaceKind::Canvas2d)
}

/// 🔀️ The inverse of contract_surface_kind, for descriptor paths still speaking the old enum.
// 🚫️async: E1 pure total mapping — see R9.
#[doc(hidden)]
pub fn legacy_surface_kind(kind: SurfaceKind) -> semio_ui_declaration::ui::SurfaceKind {
    let name = serde_json::to_string(&kind).unwrap_or_default();
    serde_json::from_str(&name).unwrap_or(semio_ui_declaration::ui::SurfaceKind::Canvas2d)
}

// 🪟️ The seven framework window kits (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET
// §2.6) — each pairs a `WindowKindDefinition` (read-only + editable variants) with a `render`
// function turning a plain owned view-model into a semantic `BuiltNode`. Frozen kind ids/labels/command ids,
// not app-declared: apps compose these kits into their own `AppDefinition.window_kinds` rather
// than each app hand-rolling a text/table/tree/image/mesh/document/media window from scratch.

/// 🏗️ Shared `WindowKindDefinition` scaffold — every kit differs only in id/label/surface/icon/actions.
#[doc(hidden)]
pub fn window_kind_definition(id: &'static str, en: &'static str, de: &'static str, surface_kind: SurfaceKind, icon_id: &'static str, mut actions: Vec<ActionDefinition>) -> WindowKindDefinition {
    for action in &mut actions {
        action.semantics.execution.interactive_job = semio_framework::InteractiveJobClassification::Migrated;
    }
    WindowKindDefinition {
        initial_utility_id: None,
        id: id.into(),
        label: LocalizedLabel::native(en, de),
        body_key: id.into(),
        surface_kind: legacy_surface_kind(surface_kind),
        icon_id: icon_id.into(),
        options: WindowOptions::default(),
        actions,
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

#[doc(hidden)]
pub use semio_os_kernel as __kernel;
#[doc(hidden)]
pub use semio_framework_pack_json as __pack_json;
#[path = "🎛️engagement/🦀️.rs"]
pub mod engagement;
#[path = "📬️mutation/🦀️.rs"]
pub mod window_mutation;
#[path = "🫧️transient/🦀️.rs"]
pub mod window_transient;
