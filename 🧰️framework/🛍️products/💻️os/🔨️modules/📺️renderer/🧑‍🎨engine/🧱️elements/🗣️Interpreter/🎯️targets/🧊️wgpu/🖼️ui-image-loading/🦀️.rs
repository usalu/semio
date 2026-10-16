//! 🖼️ UI image loading and the image render step.
//!
//! The UI image caches, byte and data url admission, SVG rasterization and the retained image render step.

use crate::element_contracts::{FrameworkWidgetContext, RasterUploadRefusal, WorkerCell};
use crate::{queue_canvas_image_upload_sized, queue_canvas_image_upload_with};
use infinite_world::world::{WorldAssetFault, WorldAssetMetadataId, WorldAssetRequestKind};
use ui_wgpu::wgpu::{ActionDescriptor, Rect, Theme, WidgetContext, WidgetInteractionMaps};

//#region UiImageLoading
static UI_IMAGE_FETCH_MISS: WorkerCell<std::collections::HashMap<String, String>> = WorkerCell::new(Default::default);
static UI_IMAGE_LAST_URL: WorkerCell<std::collections::HashMap<String, String>> = WorkerCell::new(Default::default);
static UI_IMAGE_URL_CACHE: WorkerCell<std::collections::HashMap<String, String>> = WorkerCell::new(Default::default);
pub static UI_IMAGE_SIZES: WorkerCell<std::collections::HashMap<String, (u32, u32)>> = WorkerCell::new(Default::default);
/// 🚫️ Per image id, the inline source the raster authority refused as the image's own ([`RasterUploadRefusal::Invalid`]) — kept, not
/// re-offered each paint; the image paints its fallback until its source changes.
static UI_IMAGE_REFUSED: WorkerCell<std::collections::HashMap<String, String>> = WorkerCell::new(Default::default);
static UI_IMAGE_ASSET_FAULT: WorkerCell<Option<WorldAssetFault>> = WorkerCell::new(Default::default);
#[cfg(test)]
thread_local! {
    pub static INLINE_SVG_PARSE_CALLS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
}

pub fn take_ui_image_asset_fault() -> Option<WorldAssetFault> {
    UI_IMAGE_ASSET_FAULT.with(|cell| cell.borrow_mut().take())
}

/** 📥️ Applies fetched bytes after the Canvas producer reserves its fixed generation and credits. */
pub fn apply_ui_image_bytes(id: &str, url: &str, bytes: &[u8]) {
    let svg = ui_image_looks_like_svg(url, bytes);
    let mut size = None;
    let dimensions = || {
        let measured = if svg { std::str::from_utf8(bytes).ok().and_then(svg_dimensions) } else { raster_dimensions(bytes) };
        size = measured;
        measured.map(|(width, height)| (width, height, Vec::new())).ok_or_else(Vec::new)
    };
    let decode = |_: &[u8]| {
        let decoded = if svg { std::str::from_utf8(bytes).ok().and_then(rasterize_svg_to_rgba).map(|(pixels, _, _)| pixels) } else { decode_raster_bytes(bytes).map(|(pixels, _, _)| pixels) };
        decoded
    };
    let key = match queue_canvas_image_upload_with("ui-image", id, bytes, dimensions, decode) {
        Ok(key) => key,
        Err(RasterUploadRefusal::Invalid(_)) => {
            UI_IMAGE_FETCH_MISS.with(|cell| {
                cell.borrow_mut().insert(id.to_string(), url.to_string());
            });
            return;
        }
        Err(RasterUploadRefusal::Busy) => return,
    };
    let Some((width, height)) = size else { return };
    UI_IMAGE_URL_CACHE.with(|cell| {
        cell.borrow_mut().insert(id.to_string(), key);
    });
    UI_IMAGE_SIZES.with(|cell| {
        cell.borrow_mut().insert(id.to_string(), (width, height));
    });
    UI_IMAGE_LAST_URL.with(|cell| {
        cell.borrow_mut().insert(id.to_string(), url.to_string());
    });
}

fn queue_ui_image_url_fetch(id: &str, url: &str) {
    let already_current = UI_IMAGE_LAST_URL.with(|cell| cell.borrow().get(id).map(String::as_str) == Some(url));
    if already_current {
        return;
    }
    let already_failed = UI_IMAGE_FETCH_MISS.with(|cell| cell.borrow().get(id).map(String::as_str) == Some(url));
    if already_failed {
        return;
    }
    let kind = WorldAssetMetadataId::try_from_str(id).map(|id| WorldAssetRequestKind::UiImage { id });
    let result = kind.and_then(|kind| semio_framework_os_renderer_wgpu_asset_authority::reserve_renderer_asset_request(kind, url).map(|_| ()));
    if let Err(fault) = result {
        UI_IMAGE_ASSET_FAULT.with(|cell| *cell.borrow_mut() = Some(fault));
    }
}

fn ui_image_looks_like_svg(url: &str, bytes: &[u8]) -> bool {
    if url.to_ascii_lowercase().ends_with(".svg") {
        return true;
    }
    let head = &bytes[..bytes.len().min(256)];
    let text = String::from_utf8_lossy(head);
    let trimmed = text.trim_start_matches('\u{feff}').trim_start();
    trimmed.starts_with("<svg") || (trimmed.starts_with("<?xml") && text.contains("<svg"))
}

fn decode_raster_bytes(bytes: &[u8]) -> Option<(Vec<u8>, u32, u32)> {
    let image = image::load_from_memory(bytes).ok()?;
    let rgba = image.to_rgba8();
    let (width, height) = rgba.dimensions();
    Some((rgba.into_raw(), width, height))
}

fn raster_dimensions(bytes: &[u8]) -> Option<(u32, u32)> {
    image::ImageReader::new(std::io::Cursor::new(bytes)).with_guessed_format().ok()?.into_dimensions().ok()
}

const UI_IMAGE_SVG_MAX_DIMENSION: f32 = 2048.0;

/** 🖼️ Rasterizes SVG text to straight-alpha-ish RGBA at (up to `UI_IMAGE_SVG_MAX_DIMENSION`-clamped)
 * natural size, reusing the same `usvg`/`resvg`/`tiny_skia` pipeline the `icon_atlas` module uses for
 * Lucide icons (that module's `rasterize_svg` is fixed at a 24x24 icon size and private to its own
 * module, so this is a sibling implementation at natural aspect ratio rather than a shared call). */
fn rasterize_svg_to_rgba(svg_text: &str) -> Option<(Vec<u8>, u32, u32)> {
    let mut options = usvg::Options::default();
    options.fontdb_mut().load_system_fonts();
    let tree = usvg::Tree::from_str(svg_text, &options).ok()?;
    let size = tree.size();
    let natural_w = size.width().max(1.0);
    let natural_h = size.height().max(1.0);
    let scale = (UI_IMAGE_SVG_MAX_DIMENSION / natural_w.max(natural_h)).min(1.0);
    let width = (natural_w * scale).round().max(1.0) as u32;
    let height = (natural_h * scale).round().max(1.0) as u32;
    let mut pixmap = tiny_skia::Pixmap::new(width, height)?;
    let transform = tiny_skia::Transform::from_scale(width as f32 / natural_w, height as f32 / natural_h);
    resvg::render(&tree, transform, &mut pixmap.as_mut());
    Some((pixmap.take(), width, height))
}

fn svg_dimensions(svg_text: &str) -> Option<(u32, u32)> {
    let mut options = usvg::Options::default();
    options.fontdb_mut().load_system_fonts();
    let tree = usvg::Tree::from_str(svg_text, &options).ok()?;
    let size = tree.size();
    let natural_w = size.width().max(1.0);
    let natural_h = size.height().max(1.0);
    let scale = (UI_IMAGE_SVG_MAX_DIMENSION / natural_w.max(natural_h)).min(1.0);
    Some(((natural_w * scale).round().max(1.0) as u32, (natural_h * scale).round().max(1.0) as u32))
}

fn percent_decode_basic(input: &str) -> Vec<u8> {
    let bytes = input.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(byte) = u8::from_str_radix(std::str::from_utf8(&bytes[i + 1..i + 3]).unwrap_or(""), 16) {
                out.push(byte);
                i += 3;
                continue;
            }
        }
        out.push(if bytes[i] == b'+' { b' ' } else { bytes[i] });
        i += 1;
    }
    out
}

/** 🖊️ Decodes a `data:image/svg+xml[;base64],...` URL's SVG text (base64 or percent-encoded/plain
 * UTF-8 body) — `decode_canvas_image` only handles `image/png`/`image/jpeg`, so inline SVG data URLs
 * previously fell straight through to the `alt`-text fallback. */
fn parse_svg_data_url_bytes(src: &str) -> Option<Vec<u8>> {
    use base64::Engine;
    #[cfg(test)]
    INLINE_SVG_PARSE_CALLS.with(|calls| calls.set(calls.get() + 1));
    let rest = src.strip_prefix("data:image/svg+xml")?;
    let comma = rest.find(',')?;
    let params = &rest[..comma];
    let payload = &rest[comma + 1..];
    if params.contains("base64") {
        base64::engine::general_purpose::STANDARD.decode(payload).ok()
    } else {
        Some(percent_decode_basic(payload))
    }
}

fn resolve_ui_image_svg(id: &str, src: &str) -> (Result<String, RasterUploadRefusal>, Option<(u32, u32)>) {
    let size = std::cell::Cell::new(None);
    let key = queue_canvas_image_upload_with(
        "ui-image",
        id,
        src.as_bytes(),
        || {
            let bytes = parse_svg_data_url_bytes(src).ok_or_else(Vec::new)?;
            let measured = match std::str::from_utf8(&bytes).ok().and_then(svg_dimensions) {
                Some(measured) => measured,
                None => return Err(bytes),
            };
            size.set(Some(measured));
            Ok((measured.0, measured.1, bytes))
        },
        |bytes| std::str::from_utf8(bytes).ok().and_then(rasterize_svg_to_rgba).map(|(pixels, _, _)| pixels),
    );
    (key, size.get())
}

/** 🖼️ Resolves an INLINE `data:` source in place and publishes it into the same three caches a
 * fetched URL lands in, so `render_ui_image_step`'s draw phase reads exactly ONE cache either way.
 * `data:image/svg+xml` rasterises through `resvg`; every other `data:` media type decodes through the
 * renderer's png/jpeg support (`decode_canvas_image_bytes`). This is what phase 2 used to answer
 * `ScenePaintStep::Fault` for: every `Component::Image` carrying an inline bitmap faulted the whole
 * paint step instead of rendering, on browser and native alike.
 *
 * Nothing is recorded as a miss on failure: `queue_canvas_image_upload_*` answers `None` for genuine
 * back-pressure (a busy raster queue) as well as for an undecodable payload, and a permanent miss on
 * the former would blank the image forever. An unresolved source is simply re-offered next paint,
 * bounded by the raster queue's own admission.
 *
 * @see `🧱️elements/🗣️Interpreter/🟦️.tsx` — `ImageView` (a plain `<img src>`, where the browser owns
 * data-URL decoding natively)
 */
fn resolve_ui_image_data_url(id: &str, src: &str) {
    let current = UI_IMAGE_LAST_URL.with(|cell| cell.borrow().get(id).is_some_and(|last| last == src));
    if current || UI_IMAGE_REFUSED.with(|cell| cell.borrow().get(id).is_some_and(|refused| refused == src)) {
        return;
    }
    let (key, size) = if src.starts_with("data:image/svg+xml") { resolve_ui_image_svg(id, src) } else { queue_canvas_image_upload_sized("ui-image", id, src) };
    let key = match key {
        Ok(key) => key,
        Err(RasterUploadRefusal::Invalid(_)) => {
            UI_IMAGE_REFUSED.with(|cell| {
                cell.borrow_mut().insert(id.to_string(), src.to_string());
            });
            return;
        }
        Err(RasterUploadRefusal::Busy) => return,
    };
    let Some((width, height)) = size else { return };
    UI_IMAGE_REFUSED.with(|cell| {
        cell.borrow_mut().remove(id);
    });
    UI_IMAGE_URL_CACHE.with(|cell| {
        cell.borrow_mut().insert(id.to_string(), key);
    });
    UI_IMAGE_SIZES.with(|cell| {
        cell.borrow_mut().insert(id.to_string(), (width, height));
    });
    UI_IMAGE_LAST_URL.with(|cell| {
        cell.borrow_mut().insert(id.to_string(), src.to_string());
    });
}

/// 🖼️ Returns pixels only for the requested image source while preserving the fallback during loading.
pub fn current_ui_image_key(id: &str, src: &str) -> Option<String> {
    if id.len() > 256 || src.is_empty() || src.len() > RETAINED_UI_IMAGE_SOURCE_BYTES {
        return None;
    }
    if src.starts_with("data:") {
        resolve_ui_image_data_url(id, src);
    } else {
        queue_ui_image_url_fetch(id, src);
    }
    if !UI_IMAGE_LAST_URL.with(|cell| cell.borrow().get(id).is_some_and(|current| current == src)) {
        return None;
    }
    UI_IMAGE_URL_CACHE.with(|cell| cell.borrow().get(id).cloned())
}

#[cfg(test)]
fn resolve_ui_image_url(id: &str, url: &str) -> (Option<String>, Option<(u32, u32)>) {
    queue_ui_image_url_fetch(id, url);
    let Some(key) = UI_IMAGE_URL_CACHE.with(|cell| cell.borrow().get(id).cloned()) else {
        return (None, None);
    };
    let size = UI_IMAGE_SIZES.with(|cell| cell.borrow().get(id).copied());
    (Some(key), size)
}

/** 🧭️ Resolves a plain `Image` UiNode's `src` to an uploaded raster key + natural pixel size:
 * `data:image/svg+xml` and `data:image/{png,jpeg}` decode synchronously; any other non-empty `src`
 * (`http(s)://` absolute or a relative path) is treated as a URL and admitted to the fixed renderer
 * asset authority, rendering whatever was previously cached (or nothing) in the
 * meantime — matches the React reference's plain `<img src>`, which resolves any URL natively. */
#[cfg(test)]
pub(crate) fn resolve_ui_image(id: &str, src: &str) -> (Option<String>, Option<(u32, u32)>) {
    if src.is_empty() {
        return (None, None);
    }
    if src.starts_with("data:image/svg+xml") {
        let (key, size) = resolve_ui_image_svg(id, src);
        return (key.ok(), size);
    }
    if src.starts_with("data:") {
        let (key, size) = queue_canvas_image_upload_sized("ui-image", id, src);
        return (key.ok(), size);
    }
    resolve_ui_image_url(id, src)
}

/** 📐️ CSS `object-fit: contain`-equivalent: the largest sub-rect of `bounds` that preserves the
 * `natural_w/natural_h` aspect ratio, centered. The React reference renders a plain `<img src>`,
 * whose intrinsic aspect ratio is handled by the browser natively; this renderer has no CSS engine,
 * so this reproduces the same visual result from the decoded/rasterized pixel dimensions. */
pub fn object_contain_rect(bounds: Rect, natural_w: f32, natural_h: f32) -> Rect {
    if natural_w <= 0.0 || natural_h <= 0.0 || bounds.w <= 0.0 || bounds.h <= 0.0 {
        return bounds;
    }
    let scale = (bounds.w / natural_w).min(bounds.h / natural_h);
    let w = natural_w * scale;
    let h = natural_h * scale;
    Rect::new(bounds.x + (bounds.w - w) * 0.5, bounds.y + (bounds.h - h) * 0.5, w, h)
}
//#endregion UiImageLoading

const RETAINED_UI_IMAGE_SOURCE_BYTES: usize = 4 * 1024 * 1024;

pub fn render_ui_image_step(image: &ui_wgpu::wgpu::UiImageNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, cursor: &mut ui_wgpu::wgpu::ScenePaintCursor) -> ui_wgpu::wgpu::ScenePaintStep {
    match cursor.phase() {
        0 => {
            if image.id.len() > 256 {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            if cursor.byte() < image.id.len() {
                if cursor.advance_byte().is_err() {
                    return ui_wgpu::wgpu::ScenePaintStep::Fault;
                }
                return ui_wgpu::wgpu::ScenePaintStep::Pending;
            }
            if cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        1 => {
            let src = image.src.trim();
            if src.len() > RETAINED_UI_IMAGE_SOURCE_BYTES {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            if cursor.byte() < src.len() {
                if cursor.advance_byte().is_err() {
                    return ui_wgpu::wgpu::ScenePaintStep::Fault;
                }
                return ui_wgpu::wgpu::ScenePaintStep::Pending;
            }
            if cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        2 => {
            let src = image.src.trim();
            if src.starts_with("data:") {
                resolve_ui_image_data_url(&image.id, src);
            } else if !src.is_empty() {
                queue_ui_image_url_fetch(&image.id, src);
            }
            if cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        3 => {
            let key = UI_IMAGE_URL_CACHE.with(|cell| cell.borrow().get(&image.id).filter(|key| key.len() <= 256).cloned());
            let Some(key) = key else { return cursor.finish() };
            if ctx.draw.try_reserve_retained_items(1).is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            let natural_size = UI_IMAGE_SIZES.with(|cell| cell.borrow().get(&image.id).copied());
            let target = match natural_size.filter(|(width, height)| *width > 0 && *height > 0) {
                Some((width, height)) => object_contain_rect(bounds, width as f32, height as f32),
                None => bounds,
            };
            ctx.draw.push_raster_quad(&key, [target.x, target.y, target.w, target.h], [0.0, 0.0, 1.0, 1.0], 1.0);
            if cursor.advance_item().is_err() || cursor.advance_phase().is_err() {
                return ui_wgpu::wgpu::ScenePaintStep::Fault;
            }
            ui_wgpu::wgpu::ScenePaintStep::Pending
        }
        _ => cursor.finish(),
    }
}

pub use semio_framework_os_renderer_wgpu_contracts::framework_widget_context;
