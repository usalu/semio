//! 🖼️ The image window kit.
use semio_framework::{ActionDefinition, ActionKind, WindowKindDefinition};
use semio_framework_ui_contract::BuiltNode;
/// 🧬️ SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (`sdk-flip`, 26/08/20): the choke point. Every
/// UI-bearing plugin in the fleet imports its UI vocabulary through THIS block, so flipping it
/// here redirects all 33 at once — `ui_wgpu` leaves the guest dependency graph entirely
/// (registrar-request: drop it from this crate's `Cargo.toml`, add `semio-framework-ui-contract`
/// and `semio-framework-ui-runtime` in its place). `Ui*Node`/`UiPresence`/`UiControlNode`/
/// `ActionDescriptor` have no drop-in replacement of the same name — see
/// `📓️recipe-plugin.md` in this ticket's folder for the variant-by-variant mechanical
/// translation (`UiNode::Text{..}` → `ui::text(..)`, `ActionDescriptor` → `ActionBinding`, …).
/// `ui::*` is the contract crate's own builder DSL (`ui::stack()`, `ui::button()`, …); the bare
/// glob brings every contract type (`Component`, `UiPatch`, `Label`, `ActionId`, …) into scope
/// too, since this module already used every one of the old wgpu-target types unqualified.
use semio_framework_ui_contract::*;
use semio_framework_ui_locale::Locale;
use semio_framework_ui_locale::LocalizedLabel;
use crate::WindowKit;
use semio_framework_plugin_interaction::UiAssemblyResult;
use semio_framework_plugin_window_transient::paged_text_carrier;
use semio_framework_plugin_interaction::ui_assembly_error;
use semio_framework_plugin_window_transient::window_kind_definition;



/// 🖼️ Raw pixel payload as a base64 data URI; `width`/`height` ride along in the alt text since
/// `UiImageNode` carries no separate dimension fields.
#[derive(Clone, Debug, PartialEq)]
pub struct ImageView {
    pub width: u32,
    pub height: u32,
    pub mime: String,
    pub base64: String,
}

pub struct ImageWindowKit;

/// 🚚️ Out-of-doc lane key for an image whose data URI is larger than one [`UiText`]
/// (`UI_TEXT_MAX_BYTES` is 512, so no real composited PNG fits). The payload rides in a
/// [`paged_text_carrier`] sibling of the image node under the window's own body key — the same
/// shape `scene_surface` uses for a world scene's lanes, and readable by the existing lane
/// reassembly (`fixture_carrier_text`, the React Interpreter's `surfaceSceneLaneText`).
pub const IMAGE_WINDOW_PIXELS_LANE_KEY: &str = "framework.window.image.pixels";

/// 🖼️ Node key of the image itself once the payload moved out of the doc; distinct from the lane
/// carrier's key so both can be children of one window body.
pub const IMAGE_WINDOW_PAGED_NODE_KEY: &str = "framework.window.image.view";

/// 🧩️ Stable child key for an image projection mounted beside domain controls in the same body.
pub const IMAGE_WINDOW_CONTENT_NODE_KEY: &str = "framework.window.image.content";

/// 🚫️ Stable node key for the localized state shown when an artifact remains mounted but its
/// current image representation cannot be projected by the renderer.
pub const IMAGE_WINDOW_UNAVAILABLE_NODE_KEY: &str = "framework.window.image.unavailable";

impl WindowKit for ImageWindowKit {
    type ViewModel = ImageView;
    const KIND_ID: &'static str = "framework.window.image";

    fn window_kind() -> WindowKindDefinition {
        window_kind_definition(Self::KIND_ID, "Image", "Bild", SurfaceKind::Canvas2d, "image", Vec::new())
    }

    fn editable_window_kind() -> WindowKindDefinition {
        window_kind_definition(Self::KIND_ID, "Image", "Bild", SurfaceKind::Canvas2d, "image", vec![ActionDefinition::bounded_catalog("set-pixel-region", LocalizedLabel::native("Set Pixel Region", "Pixelbereich setzen"), ActionKind::Mutation)])
    }

    /// 🖼️ An image's payload size is a property of the document, never of the UI contract, so a
    /// composite too large for one [`UiText`] pages out of the doc instead of refusing the whole
    /// window. Below the `UiText` bound the node is byte-identical to what this always produced;
    /// above it the `src` names [`IMAGE_WINDOW_PIXELS_LANE_KEY`] and the data URI rides in a
    /// [`paged_text_carrier`] sibling. Before this, every raster viewer window died at assembly
    /// with `ui.fixed-capacity … image-window.source` because `UI_TEXT_MAX_BYTES` is 512 and a real
    /// composited PNG never fits (ticket 26/09/19/SEMIO-TECH-PLAY-GRID-WITH-EVERY-APP, raster §3).
    fn render(view: &ImageView) -> UiAssemblyResult<BuiltNode> {
        let source = format!("data:{};base64,{}", view.mime, view.base64);
        let alt = UiText::try_format(format_args!("{}x{}", view.width, view.height)).map(Label).ok_or_else(|| ui_assembly_error("image-window.alt"))?;
        if let Some(src) = UiText::try_format(format_args!("{source}")) {
            return image(src).alt(alt).try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("image-window.id"))?.try_build().map_err(|_| ui_assembly_error("image-window.build"));
        }
        let lane = UiText::try_format(format_args!("{IMAGE_WINDOW_PIXELS_LANE_KEY}")).ok_or_else(|| ui_assembly_error("image-window.lane-key"))?;
        let node = image(lane).alt(alt).try_id(IMAGE_WINDOW_PAGED_NODE_KEY).map_err(|_| ui_assembly_error("image-window.paged-id"))?.try_build().map_err(|_| ui_assembly_error("image-window.paged-build"))?;
        let carrier = paged_text_carrier(IMAGE_WINDOW_PIXELS_LANE_KEY, &source)?;
        column()
            .try_id(Self::KIND_ID)
            .map_err(|_| ui_assembly_error("image-window.id"))?
            .try_children(vec![node, carrier])
            .map_err(|_| ui_assembly_error("image-window.paged-children"))?
            .try_build()
            .map_err(|_| ui_assembly_error("image-window.build"))
    }
}

impl ImageWindowKit {
    fn render_content(view: &ImageView) -> UiAssemblyResult<BuiltNode> {
        let source = format!("data:{};base64,{}", view.mime, view.base64);
        let alt = UiText::try_format(format_args!("{}x{}", view.width, view.height)).map(Label).ok_or_else(|| ui_assembly_error("image-window.alt"))?;
        if let Some(src) = UiText::try_format(format_args!("{source}")) {
            return image(src).alt(alt).try_id(IMAGE_WINDOW_CONTENT_NODE_KEY).map_err(|_| ui_assembly_error("image-window.content-id"))?.try_build().map_err(|_| ui_assembly_error("image-window.content-build"));
        }
        let lane = UiText::try_format(format_args!("{IMAGE_WINDOW_PIXELS_LANE_KEY}")).ok_or_else(|| ui_assembly_error("image-window.lane-key"))?;
        let node = image(lane).alt(alt).try_id(IMAGE_WINDOW_PAGED_NODE_KEY).map_err(|_| ui_assembly_error("image-window.paged-id"))?.try_build().map_err(|_| ui_assembly_error("image-window.paged-build"))?;
        let carrier = paged_text_carrier(IMAGE_WINDOW_PIXELS_LANE_KEY, &source)?;
        column()
            .try_id(IMAGE_WINDOW_CONTENT_NODE_KEY)
            .map_err(|_| ui_assembly_error("image-window.content-id"))?
            .try_children(vec![node, carrier])
            .map_err(|_| ui_assembly_error("image-window.paged-children"))?
            .try_build()
            .map_err(|_| ui_assembly_error("image-window.content-build"))
    }

    /// 🧩️ Renders one bounded domain accessory and the image projection under the kit body.
    pub fn render_with_accessory(view: &ImageView, accessory: BuiltNode) -> UiAssemblyResult<BuiltNode> {
        let content = Self::render_content(view)?;
        column()
            .try_id(Self::KIND_ID)
            .map_err(|_| ui_assembly_error("image-window.id"))?
            .try_children([accessory, content])
            .map_err(|_| ui_assembly_error("image-window.accessory-children"))?
            .try_build()
            .map_err(|_| ui_assembly_error("image-window.build"))
    }

    /// 🗣️ Resolves the domain-neutral unavailable-preview label for the active shell locale.
    pub const fn unavailable_label(locale: Locale) -> &'static str {
        match locale {
            Locale::En => "Preview unavailable",
            Locale::De => "Vorschau nicht verfügbar",
        }
    }

    /// 🚫️ Keeps an image artifact's window mounted when its current representation has no
    /// renderer projection, so sibling source and Details surfaces remain usable.
    pub fn render_unavailable(locale: Locale) -> UiAssemblyResult<BuiltNode> {
        let label = Label::try_from(Self::unavailable_label(locale)).map_err(|_| ui_assembly_error("image-window.unavailable-label"))?;
        let unavailable = text(label).try_id(IMAGE_WINDOW_UNAVAILABLE_NODE_KEY).map_err(|_| ui_assembly_error("image-window.unavailable-id"))?.try_build().map_err(|_| ui_assembly_error("image-window.unavailable-build"))?;
        column().try_id(Self::KIND_ID).map_err(|_| ui_assembly_error("image-window.id"))?.try_child(unavailable).map_err(|_| ui_assembly_error("image-window.unavailable-child"))?.try_build().map_err(|_| ui_assembly_error("image-window.build"))
    }

    /// 🧩️ Keeps domain controls usable when the selected image page has no preview projection.
    pub fn render_unavailable_with_accessory(locale: Locale, accessory: BuiltNode) -> UiAssemblyResult<BuiltNode> {
        let label = Label::try_from(Self::unavailable_label(locale)).map_err(|_| ui_assembly_error("image-window.unavailable-label"))?;
        let unavailable = text(label).try_id(IMAGE_WINDOW_UNAVAILABLE_NODE_KEY).map_err(|_| ui_assembly_error("image-window.unavailable-id"))?.try_build().map_err(|_| ui_assembly_error("image-window.unavailable-build"))?;
        column()
            .try_id(Self::KIND_ID)
            .map_err(|_| ui_assembly_error("image-window.id"))?
            .try_children([accessory, unavailable])
            .map_err(|_| ui_assembly_error("image-window.accessory-children"))?
            .try_build()
            .map_err(|_| ui_assembly_error("image-window.build"))
    }
}
