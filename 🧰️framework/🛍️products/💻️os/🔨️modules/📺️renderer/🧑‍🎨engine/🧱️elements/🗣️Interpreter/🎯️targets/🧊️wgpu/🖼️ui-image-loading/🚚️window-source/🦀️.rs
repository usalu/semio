use super::*;
use semio_ui_contract::window_resource::{WindowResource, WindowResourcePage, WindowResourceReader};
use semio_ui_declaration::ui::UiImageNode;
use semio_ui_wgpu_hosting::scene_slots::{ScenePaintCursor, ScenePaintStep};
use std::{
    collections::HashMap,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
        mpsc::{self, Receiver, TryRecvError},
    },
};

#[path = "🎞️tiles/🦀️.rs"]
mod tiles;
use tiles::ImageLayout;

type Owner = (String, u64, String);
type Decoded = (Vec<u8>, u32, u32);

struct ImageResource {
    resource: WindowResource,
    requested: u32,
    paint_revision: u64,
    display: Display,
}

enum Display {
    Fetching(WindowResourceReader),
    Decoding(DecodeHandle),
    Staging { layout: ImageLayout, pixels: Vec<u8>, next: usize, tile: Vec<u8> },
    Ready(ImageLayout),
    Failed,
}

struct DecodeHandle {
    cancelled: Arc<AtomicBool>,
    result: Receiver<Option<Decoded>>,
}

impl Drop for DecodeHandle {
    fn drop(&mut self) {
        self.cancelled.store(true, Ordering::Release);
    }
}

static IMAGE_RESOURCES: WorkerCell<HashMap<Owner, ImageResource>> = WorkerCell::new(Default::default);
const WINDOW_RESOURCE_RESIDENT_BYTES: usize = 4 * semio_ui_contract::window_resource::WINDOW_RESOURCE_MAX_BYTES;

fn request_page(ctx: &mut FrameworkWidgetContext<'_>, window: &str, generation: u64, image: &str, resource: WindowResource, index: u32) -> bool {
    ctx.input
        .publish_action("framework", "setImageSourcePage", 1024, |builder, _| {
            builder.begin_object(None)?;
            builder.string(Some("windowId"), window)?;
            builder.string(Some("windowGeneration"), &generation.to_string())?;
            builder.string(Some("imageId"), image)?;
            builder.string(Some("resourceKey"), &resource.request_key())?;
            builder.number(Some("page"), f64::from(index))?;
            builder.end_container()
        })
        .is_ok()
}

fn decode(source: &str, cancelled: &AtomicBool) -> Option<Decoded> {
    let cancelled = || cancelled.load(Ordering::Acquire);
    if cancelled() {
        return None;
    }
    if source.starts_with("data:image/svg+xml") {
        let bytes = parse_svg_data_url_bytes(source)?;
        return rasterize_svg_to_rgba(std::str::from_utf8(&bytes).ok()?);
    }
    use semio_framework_pixels::image_decoding::{EncodedDecodeLimits, decode_encoded_blocking};
    let (media, data) = source.split_once(',')?;
    if !matches!(media, "data:image/png;base64" | "data:image/jpeg;base64") {
        return None;
    }
    let bytes = semio_framework_io_base64::base64_standard_decode(data).ok()?;
    let image = decode_encoded_blocking(bytes, EncodedDecodeLimits::MAXIMUM, &cancelled).ok()?.image;
    Some((image.pixels, image.width, image.height))
}

fn decode_in_background(source: &str) -> Option<DecodeHandle> {
    let (send, result) = mpsc::sync_channel(1);
    let cancelled = Arc::new(AtomicBool::new(false));
    let flag = cancelled.clone();
    let source = source.to_owned();
    let job = move || {
        let output = decode(&source, &flag);
        if !flag.load(Ordering::Acquire) {
            let _ = send.try_send(output);
            wake_raster_frame();
        }
    };
    #[cfg(not(target_arch = "wasm32"))]
    semio_renderer_wgpu_host_base::renderer_worker_pool().try_submit(semio_framework_async::Lane::Maintenance, Box::new(job)).ok()?;
    #[cfg(target_arch = "wasm32")]
    job();
    Some(DecodeHandle { cancelled, result })
}

fn wake_raster_frame() {
    #[cfg(not(target_arch = "wasm32"))]
    semio_renderer_wgpu_host_base::request_retained_image_raster_frame();
}

impl ImageResource {
    fn new(resource: WindowResource, paint_revision: u64) -> Self {
        Self { resource, requested: 0, paint_revision, display: Display::Fetching(WindowResourceReader::new(resource)) }
    }

    fn request(&mut self, ctx: &mut FrameworkWidgetContext<'_>, window: &str, generation: u64, image: &str, page: u32) -> bool {
        if self.requested == page {
            return true;
        }
        if !request_page(ctx, window, generation, image, self.resource, page) {
            return false;
        }
        self.requested = page;
        true
    }
}

fn raster_key(generation: u64, resource: WindowResource, index: usize) -> String {
    format!("{}:{generation}:{index}", resource.request_key())
}

fn stage_raster(window: &str, generation: u64, resource: WindowResource, display: &mut Display) {
    let Display::Staging { layout, pixels, next, tile } = display else {
        return;
    };
    let bounds = layout.tile(*next);
    if tile.is_empty() {
        bounds.fill(pixels, layout.width, tile);
    }
    match crate::queue_decoded_raster_upload_retaining_source(window, raster_key(generation, resource, *next), bounds.padded_width, bounds.padded_height, tile) {
        Ok(_) => {
            *next += 1;
            if *next == layout.count() {
                *display = Display::Ready(*layout);
            }
            wake_raster_frame();
        }
        Err(RasterUploadRefusal::Busy) => wake_raster_frame(),
        Err(RasterUploadRefusal::Invalid(_)) => *display = Display::Failed,
    }
}

/// 🧹️ Releases partial reads and decoding for exactly the window instance being retired.
pub fn close_window_image_resource_step(window: &str, generation: u64) -> bool {
    IMAGE_RESOURCES.with(|cell| {
        let mut resources = cell.borrow_mut();
        let key = resources.keys().find(|owner| owner.0 == window && owner.1 == generation).cloned();
        key.is_some_and(|key| resources.remove(&key).is_some())
    })
}

/// 🧽️ Cancels a prior source when the same image returns to the inline path.
pub fn replace_window_image_resource(image: &UiImageNode, ctx: &mut FrameworkWidgetContext<'_>, window: &str, generation: u64) {
    let owner = (window.to_owned(), generation, image.id.clone());
    IMAGE_RESOURCES.with(|cell| {
        let mut resources = cell.borrow_mut();
        if resources.get_mut(&owner).is_some_and(|entry| entry.request(ctx, window, generation, &image.id, 0)) {
            resources.remove(&owner);
        }
    });
}

/// 🧹️ Releases one source omitted from the completed retained paint pass.
pub fn close_hidden_window_image_resource(ctx: &mut FrameworkWidgetContext<'_>, window: &str, generation: u64, paint_revision: u64) -> bool {
    IMAGE_RESOURCES.with(|cell| {
        let mut resources = cell.borrow_mut();
        let Some(owner) = resources.iter().find(|(owner, entry)| owner.0 == window && owner.1 == generation && entry.paint_revision != paint_revision).map(|(owner, _)| owner.clone()) else {
            return false;
        };
        if !resources.get_mut(&owner).is_some_and(|entry| entry.request(ctx, window, generation, &owner.2, 0)) {
            return false;
        }
        resources.remove(&owner);
        true
    })
}

/// 🖼️ Receives an exact source page and stages its tiles through the renderer's existing raster credits.
pub fn render_window_image_resource(image: &UiImageNode, bounds: Rect, ctx: &mut FrameworkWidgetContext<'_>, cursor: &mut ScenePaintCursor, window: &str, generation: u64, paint_revision: u64) -> ScenePaintStep {
    let Some(page) = WindowResourcePage::parse(&image.src) else { return cursor.finish() };
    if !page.resource.matches_owner(window, &image.id) {
        return cursor.finish();
    }
    let owner = (window.to_owned(), generation, image.id.clone());
    let (layout, progress) = IMAGE_RESOURCES.with(|cell| {
        let mut resources = cell.borrow_mut();
        if !resources.contains_key(&owner) && resources.len() >= semio_ui_contract::UI_DOCUMENT_NODES {
            return (None, None);
        }
        let mut resident = 0;
        let mut raster_busy = false;
        for (_, entry) in resources.iter().filter(|(candidate, _)| *candidate != &owner) {
            match &entry.display {
                Display::Fetching(reader) => {
                    if reader.completed_bytes() > 0 {
                        resident += reader.resource().bytes as usize;
                    }
                }
                Display::Decoding(_) | Display::Staging { .. } => raster_busy = true,
                Display::Ready(_) | Display::Failed => {}
            }
        }
        let entry = resources.entry(owner).or_insert_with(|| ImageResource::new(page.resource, paint_revision));
        if entry.resource != page.resource {
            if !entry.request(ctx, window, generation, &image.id, 0) {
                return (None, None);
            }
            *entry = ImageResource::new(page.resource, paint_revision);
        }
        entry.paint_revision = paint_revision;
        let next = match &mut entry.display {
            Display::Fetching(reader) => {
                if let Some(text) = image.source_page.as_deref().filter(|_| reader.next() == Some(page.index)) {
                    if reader.completed_bytes() == 0 && resident + page.resource.bytes as usize > WINDOW_RESOURCE_RESIDENT_BYTES {
                        return (None, None);
                    }
                    if reader.accept(page, text).is_err() {
                        entry.display = Display::Failed;
                    }
                }
                match &mut entry.display {
                    Display::Fetching(reader) => {
                        if let Some(next) = reader.next() {
                            (next != page.index).then_some(next)
                        } else {
                            if !raster_busy {
                                if let Some(handle) = reader.complete().and_then(decode_in_background) {
                                    entry.display = Display::Decoding(handle);
                                } else {
                                    wake_raster_frame();
                                }
                            }
                            Some(0)
                        }
                    }
                    _ => Some(0),
                }
            }
            Display::Decoding(handle) => {
                match handle.result.try_recv() {
                    Ok(Some((pixels, width, height))) if width > 0 && height > 0 => {
                        entry.display = Display::Staging { layout: ImageLayout { width, height }, pixels, next: 0, tile: Vec::new() };
                        stage_raster(window, generation, page.resource, &mut entry.display);
                    }
                    Ok(_) | Err(TryRecvError::Disconnected) => entry.display = Display::Failed,
                    Err(TryRecvError::Empty) => {}
                }
                Some(0)
            }
            Display::Staging { .. } => {
                stage_raster(window, generation, page.resource, &mut entry.display);
                Some(0)
            }
            Display::Ready(_) | Display::Failed => Some(0),
        };
        if let Some(next) = next {
            entry.request(ctx, window, generation, &image.id, next);
        }
        match &entry.display {
            Display::Ready(layout) => (Some(*layout), None),
            Display::Fetching(reader) => (None, Some(reader.completed_bytes() as f32 / page.resource.bytes as f32 * 0.75)),
            Display::Decoding(_) => {
                wake_raster_frame();
                (None, Some(0.75))
            }
            Display::Staging { layout, next, .. } => (None, Some(0.75 + *next as f32 / layout.count() as f32 * 0.25)),
            Display::Failed => (None, None),
        }
    });
    if let Some(layout) = layout {
        let index = cursor.item();
        if index >= layout.count() {
            return cursor.finish();
        }
        if ctx.draw.try_reserve_retained_items(1).is_err() {
            return ScenePaintStep::Fault;
        }
        let target = object_contain_rect(bounds, layout.width as f32, layout.height as f32);
        let tile = layout.tile(index);
        let sx = target.w / layout.width as f32;
        let sy = target.h / layout.height as f32;
        ctx.draw.push_raster_quad(&raster_key(generation, page.resource, index), [target.x + tile.left as f32 * sx, target.y + tile.top as f32 * sy, tile.width as f32 * sx, tile.height as f32 * sy], tile.uv(), 1.0);
        if cursor.advance_item().is_err() {
            return ScenePaintStep::Fault;
        }
        return ScenePaintStep::Pending;
    }
    if let Some(progress) = progress {
        if ctx.draw.try_reserve_retained_items(2).is_err() {
            return ScenePaintStep::Fault;
        }
        let height = bounds.h.min(4.0);
        ctx.draw.push_rounded([bounds.x, bounds.y, bounds.w, height], ctx.theme.muted, ctx.theme.border_radius);
        ctx.draw.push_rounded([bounds.x, bounds.y, bounds.w * progress, height], ctx.theme.accent, ctx.theme.border_radius);
    }
    cursor.finish()
}
