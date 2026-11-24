//! 🖼️ PDF page thumbnail strip: one row per page that selects the page, drags onto another row to reorder through the existing `move-page` action, and offers move earlier or later as child rows.
//! [PDF page tree](https://www.iso.org/standard/51502.html)

use super::*;
use semio_plugin::plugin_app_close_prelude as ui;
use semio_plugin::{ActionId, Buildable, HasBase, HasChildren, PluginAssemblyError, SurfaceKind, UiAssemblyResult, UiMapBuilder, UiText, UiValue, WindowKindDefinition, WindowOptions};
use semio_ui_locale::{Locale, LocalizedLabel};

//#region 🖼️Thumbnails
pub const THUMBNAILS_WINDOW_KIND_ID: &str = "pdf.thumbnails";
pub const THUMBNAILS_BODY_KEY: &str = "pdf.thumbnails";
pub const THUMBNAIL_EDGE: f64 = 96.0;
/// 🖐️ The drag payload key; the framework merges a drop's payload into the drop action's arguments, so a dragged page arrives as the `page` argument of `move-page`.
pub const THUMBNAIL_DRAG_KEY: &str = "page";

/// 🪟 The thumbnail strip window.
pub fn thumbnails_window_definition() -> WindowKindDefinition {
    WindowKindDefinition {
        initial_utility_id: None,
        id: THUMBNAILS_WINDOW_KIND_ID.into(),
        label: LocalizedLabel::native("Pages", "Seiten"),
        body_key: THUMBNAILS_BODY_KEY.into(),
        surface_kind: SurfaceKind::BlockList,
        icon_id: "layout-grid".into(),
        options: WindowOptions::default(),
        actions: Vec::new(),
        utilities: Vec::new(),
        interactions: Vec::new(),
        params_schema: None,
        artifact_snapshot_schema: None,
        input_event_schema: None,
        output_schema: None,
        capabilities: Vec::new(),
    }
}

/// 🖼️ One strip entry: the page and its scaled frame (the longest side is `edge`).
#[derive(Clone, Debug, PartialEq)]
pub struct Thumbnail {
    pub page: usize,
    pub width: f64,
    pub height: f64,
    pub target: String,
}

pub fn thumbnails(snapshot: &PdfSnapshot, edge: f64) -> Vec<Thumbnail> {
    page_viewports(snapshot).enumerate().map(|(page, viewport)| {
        let [_, _, width, height] = viewport.clip_rect();
        let scale = edge / width.max(height).max(1.0);
        Thumbnail { page, width: (width * scale).round().max(1.0), height: (height * scale).round().max(1.0), target: format!("p{page}:paper") }
    }).collect()
}

/// 🔀 The `move-page` arguments (page, destination index) that drop page `from` at slot `to`, or none when nothing changes.
pub fn reorder_args(count: usize, from: usize, to: usize) -> Option<(usize, usize)> {
    (from < count && to < count && from != to).then_some((from, to))
}

fn label(locale: Locale, en: &str, de: &str) -> String {
    match locale {
        Locale::De => de.to_string(),
        Locale::En => en.to_string(),
    }
}

fn error(code: &'static str) -> PluginAssemblyError {
    PluginAssemblyError::new(code, "pdf thumbnail strip admission failed")
}

fn numbers(entries: &[(&str, f64)]) -> UiAssemblyResult<UiValue> {
    let mut map = UiMapBuilder::try_new().ok_or_else(|| error("pdf.thumbnails.args"))?;
    for (key, value) in entries {
        map.try_insert((*key).to_owned(), UiValue::Number(*value)).map_err(|_| error("pdf.thumbnails.args"))?;
    }
    Ok(UiValue::Map(map.finish()))
}

/// 🖼️ The strip body. `object_domain` is the page editor's object selection domain; activating a row selects that page in it.
pub fn render_thumbnail_strip(snapshot: &PdfSnapshot, controller_id: &str, object_domain: &str, locale: Locale) -> UiAssemblyResult<semio_ui_contract::BuiltNode> {
    render_thumbnail_strip_marked(snapshot, controller_id, object_domain, locale, &|_| Vec::new())
}

/// 👥️ [`render_thumbnail_strip`] with the captions of the collaborators on each page appended to its row text, so the row says who is there in words and not only in colour.
pub fn render_thumbnail_strip_marked(snapshot: &PdfSnapshot, controller_id: &str, object_domain: &str, locale: Locale, peers: &dyn Fn(usize) -> Vec<String>) -> UiAssemblyResult<semio_ui_contract::BuiltNode> {
    let action = |name: &str| ActionId::try_v1(controller_id, name).ok_or_else(|| error("pdf.thumbnails.action"));
    let count = snapshot.pages.len();
    let mut tree = semio_ui_contract::tree().try_id("pdf-thumbnails").map_err(|_| error("pdf.thumbnails.root-id"))?;
    for thumbnail in thumbnails(snapshot, THUMBNAIL_EDGE) {
        let page = thumbnail.page;
        let mut text = format!("{} {} / {count} ({}x{})", label(locale, "Page", "Seite"), page_number(page), thumbnail.width, thumbnail.height);
        let here = peers(page);
        if !here.is_empty() {
            text.push_str(&format!(" - {}: {}", label(locale, "here", "hier"), here.join(", ")));
        }
        let targets = json!([{ "granularity": "object", "id": thumbnail.target }]).to_string();
        let mut selection = UiMapBuilder::try_new().ok_or_else(|| error("pdf.thumbnails.selection"))?;
        for (key, value) in [("domainId", object_domain), ("targets", targets.as_str()), ("merge", "replace"), ("method", "pick")] {
            selection.try_insert(key.to_owned(), UiValue::Text(UiText::clipped(value))).map_err(|_| error("pdf.thumbnails.selection"))?;
        }
        let mut drag = semio_ui_contract::UiFixedMap::default();
        drag.try_push(UiText::clipped(THUMBNAIL_DRAG_KEY), UiText::clipped(&page_number(page).to_string())).map_err(|_| error("pdf.thumbnails.drag"))?;
        let mut row = semio_ui_contract::tree_item(ui::Label(UiText::clipped(&text))).try_id(format!("pdf-thumbnail-{page}")).map_err(|_| error("pdf.thumbnails.entry-id"))?
            .default_open(false).draggable(true).drag_data(drag)
            .try_on_with(ui::Trigger::Activate, action(semio_plugin::INTERACTION_SELECT_ACTION_ID)?, UiValue::Map(selection.finish())).map_err(|_| error("pdf.thumbnails.selection"))?
            .try_on_with(ui::Trigger::Drop, action("move-page")?, numbers(&[("x", page_number(page) as f64)])?).map_err(|_| error("pdf.thumbnails.drop"))?;
        for (id, text, to) in [("earlier", label(locale, "Move page earlier", "Seite nach vorne"), page.checked_sub(1)), ("later", label(locale, "Move page later", "Seite nach hinten"), Some(page + 1))] {
            let Some((from, to)) = to.and_then(|to| reorder_args(count, page, to)) else { continue };
            let item = semio_ui_contract::tree_item(ui::Label(UiText::clipped(&text))).try_id(format!("pdf-thumbnail-{page}-{id}")).map_err(|_| error("pdf.thumbnails.move-id"))?
                .try_on_with(ui::Trigger::Activate, action("move-page")?, numbers(&[("page", page_number(from) as f64), ("x", page_number(to) as f64)])?).map_err(|_| error("pdf.thumbnails.move"))?
                .try_build().map_err(|_| error("pdf.thumbnails.move"))?;
            row = row.try_child(item).map_err(|_| error("pdf.thumbnails.child"))?;
        }
        tree = tree.try_child(row.try_build().map_err(|_| error("pdf.thumbnails.entry"))?).map_err(|_| error("pdf.thumbnails.child"))?;
    }
    tree.try_build().map_err(|_| error("pdf.thumbnails.root"))
}
//#endregion 🖼️Thumbnails
