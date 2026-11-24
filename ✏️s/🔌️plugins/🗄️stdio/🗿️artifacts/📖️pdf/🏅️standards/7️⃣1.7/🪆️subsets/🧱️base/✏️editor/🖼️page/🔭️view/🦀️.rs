//! 🔭️ PDF page view: ephemeral local zoom and page navigation, text search and the page thumbnail strip, as pure logic over a snapshot.
//! Nothing here touches the document and nothing here knows page painting. [Canvas camera](https://www.w3.org/TR/css-transforms-1/)

use semio_plugin::Canvas2dScene;
use semio_s_artifact_stdio_pdf_page_coordinates::PageViewport;
use semio_s_artifact_stdio_pdf_snapshot_2::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use serde_json::{json, Value};

#[path = "🎬️actions/🦀️.rs"]
mod actions;
pub use self::actions::*;

#[path = "🔢️page-number/🦀️.rs"]
mod page_numbers;
pub use self::page_numbers::*;

#[path = "🔍️search/🦀️.rs"]
mod search;
pub use self::search::*;

#[path = "🖼️thumbnails/🦀️.rs"]
mod thumbnails;
pub use self::thumbnails::*;

//#region 🔭️View
pub const ZOOM_MIN: f64 = 0.1;
pub const ZOOM_MAX: f64 = 8.0;
pub const ZOOM_STEPS: [f64; 11] = [0.1, 0.25, 0.5, 0.75, 1.0, 1.25, 1.5, 2.0, 3.0, 4.0, 8.0];
const FIT_MARGIN: f64 = 16.0;
const PAGE_GAP: f64 = 16.0;

/// 🖍️ One scene layer drawn above the page (search highlights).
pub type OverlayLayer = Value;

/// 🔍️ How the zoom factor is chosen: explicitly, or derived from the viewport and the current page.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ZoomMode {
    Factor(f64),
    FitPage,
    FitWidth,
    ActualSize,
}

/// 🔭️ Ephemeral local view state: zoom mode, anchored page, fit revision and the search cursor.
#[derive(Clone, Debug, PartialEq)]
pub struct ViewState {
    pub mode: ZoomMode,
    pub page: usize,
    pub revision: u32,
    pub query: String,
    pub match_case: bool,
    pub active: Option<usize>,
}

impl Default for ViewState {
    fn default() -> Self {
        Self { mode: ZoomMode::ActualSize, page: 0, revision: 0, query: String::new(), match_case: false, active: None }
    }
}

/// 🎬️ Every user intent over the view; each is a pure reducer step, none touches the document.
#[derive(Clone, Debug, PartialEq)]
pub enum ViewIntent {
    ZoomIn,
    ZoomOut,
    FitPage,
    FitWidth,
    ActualSize,
    GoToPage(usize),
    SetQuery(String),
    ToggleMatchCase,
    NextHit,
    PreviousHit,
}

/// 🎬️ The next view state; `hits` are the hits of the state's own query, so stepping lands the camera on the hit's page.
pub fn reduce_view(snapshot: &PdfSnapshot, view: &ViewState, intent: &ViewIntent, viewport: (f64, f64), hits: &[SearchHit]) -> ViewState {
    let mut next = view.clone();
    let last = snapshot.pages.len().saturating_sub(1);
    let refit = view.revision.wrapping_add(1);
    match intent {
        ViewIntent::ZoomIn => next.mode = ZoomMode::Factor(zoom_in(resolve_zoom(snapshot, view, viewport))),
        ViewIntent::ZoomOut => next.mode = ZoomMode::Factor(zoom_out(resolve_zoom(snapshot, view, viewport))),
        ViewIntent::FitPage => { next.mode = ZoomMode::FitPage; next.revision = refit; }
        ViewIntent::FitWidth => { next.mode = ZoomMode::FitWidth; next.revision = refit; }
        ViewIntent::ActualSize => next.mode = ZoomMode::ActualSize,
        ViewIntent::GoToPage(page) => { next.page = (*page).min(last); next.revision = refit; }
        ViewIntent::SetQuery(query) => { next.query = query.clone(); next.active = None; }
        ViewIntent::ToggleMatchCase => { next.match_case = !view.match_case; next.active = None; }
        ViewIntent::NextHit | ViewIntent::PreviousHit => {
            next.active = step_hit(hits.len(), view.active.filter(|at| *at < hits.len()), *intent == ViewIntent::NextHit);
            if let Some(hit) = next.active.and_then(|at| hits.get(at)) {
                next.page = hit.page.min(last);
                next.revision = refit;
            }
        }
    }
    next
}

pub fn clamp_zoom(zoom: f64) -> f64 {
    if zoom.is_finite() { zoom.clamp(ZOOM_MIN, ZOOM_MAX) } else { 1.0 }
}

pub fn zoom_in(zoom: f64) -> f64 {
    ZOOM_STEPS.iter().copied().find(|step| *step > zoom + 1e-9).unwrap_or(ZOOM_MAX)
}

pub fn zoom_out(zoom: f64) -> f64 {
    ZOOM_STEPS.iter().rev().copied().find(|step| *step < zoom - 1e-9).unwrap_or(ZOOM_MIN)
}

/// 📜 Every page's viewport, stacked top to bottom with a fixed gap.
pub fn page_viewports(snapshot: &PdfSnapshot) -> impl Iterator<Item = PageViewport> + '_ {
    snapshot.pages.iter().scan(0.0, |top, page| {
        let viewport = PageViewport::new(page.media_box, page.crop_box, page.rotate, page.user_unit, *top);
        *top += viewport.clip_rect()[3] + PAGE_GAP;
        Some(viewport)
    })
}

/// 📐 The page's displayed size in canvas units, rotation and crop included.
pub fn page_size(snapshot: &PdfSnapshot, page: usize) -> Option<(f64, f64)> {
    page_viewports(snapshot).nth(page).map(|viewport| { let [_, _, width, height] = viewport.clip_rect(); (width, height) })
}

/// 📜 Canvas ordinate of the page's top edge.
pub fn page_top(snapshot: &PdfSnapshot, page: usize) -> f64 {
    page_viewports(snapshot).nth(page).map_or(0.0, |viewport| viewport.clip_rect()[1])
}

/// 🔭️ The zoom factor the state resolves to inside `viewport` (width, height).
pub fn resolve_zoom(snapshot: &PdfSnapshot, view: &ViewState, viewport: (f64, f64)) -> f64 {
    let (width, height) = page_size(snapshot, view.page).unwrap_or((1.0, 1.0));
    let fit = |available: f64, extent: f64| clamp_zoom((available - 2.0 * FIT_MARGIN).max(1.0) / extent.max(1.0));
    match view.mode {
        ZoomMode::Factor(zoom) => clamp_zoom(zoom),
        ZoomMode::ActualSize => 1.0,
        ZoomMode::FitWidth => fit(viewport.0, width),
        ZoomMode::FitPage => fit(viewport.0, width).min(fit(viewport.1, height)),
    }
}

/// 🖼️ A rectangle as a closed path in scene segments.
pub fn rect_segments(x: f64, y: f64, width: f64, height: f64) -> Value {
    json!([{ "kind": "move", "to": [x, y] }, { "kind": "line", "to": [x + width, y] }, { "kind": "line", "to": [x + width, y + height] }, { "kind": "line", "to": [x, y + height] }, { "kind": "close" }])
}

/// 🖼️ The page scene (`layers_json` is the painted page) with the view's camera, host framing for the fit modes, and `overlay` layers on top.
/// The camera is the view's centre; fit modes hand the host the page bounds so it fits its own measured viewport.
pub fn scene_for_view(snapshot: &PdfSnapshot, layers_json: &str, view: &ViewState, viewport: (f64, f64), overlay: Vec<OverlayLayer>) -> Canvas2dScene {
    let mut layers: Vec<Value> = serde_json::from_str(layers_json).unwrap_or_default();
    layers.extend(overlay);
    let (width, height) = page_size(snapshot, view.page).unwrap_or((0.0, 0.0));
    let top = page_top(snapshot, view.page);
    let mut scene = Canvas2dScene::base(width / 2.0, top + height / 2.0, resolve_zoom(snapshot, view, viewport), serde_json::to_string(&layers).unwrap_or_else(|_| "[]".into()));
    scene.framing = match view.mode {
        ZoomMode::FitPage => Some(semio_plugin::Canvas2dFraming { revision: view.revision, bounds: [0.0, top, width, top + height], padding: FIT_MARGIN }),
        ZoomMode::FitWidth => Some(semio_plugin::Canvas2dFraming { revision: view.revision, bounds: [0.0, top, width, top], padding: FIT_MARGIN }),
        _ => None,
    };
    scene
}
//#endregion 🔭️View
