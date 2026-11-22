//! 💿️ Ui image sources, skeleton and overlays: the image admission ledger and data url decoding, the skeleton blocks and the overlay backdrop, surface, chrome and tooltip painters. See `semio_ui_wgpu_paint`.

use semio_ui_declaration::ui::{UiNode, UiPresence, UiStatus};
use semio_ui_draw_contract::DrawList;
use semio_ui_text::FontAtlas;
use semio_ui_theme::geometry::Rect;
use semio_ui_theme::{Level, Theme};
use semio_ui_wgpu_layout::chrome::push_chrome_border;
use semio_ui_wgpu_widget_kit::draw_text_on;

const RETAINED_NODE_FIXED_OUTPUT_BYTES: usize = 64 * 1024;

// #region paint
//#region 🖼️UiImageSources
// 🖼️ React's `ImageView` is a real `<img src>` (`🗣️Interpreter/🟦️.tsx:1972-1975`) — the browser owns
// fetch, decode, cache and `object-fit`. A GPU canvas owns all four itself. The split here mirrors
// what the os renderer already does for the SceneHost path (`🗣️Interpreter/🎯️targets/🧊️wgpu`'s
// `UiImageLoading` region): a `src` the process can decode *without I/O* — a `data:` URL — is decoded
// here, in first-party code (`semio-framework-pixels`, the owned PNG codec), published as a natural
// size plus one pending RGBA8 upload keyed by `src`, and drawn as a `KIND_RASTER` quad. Any other
// `src` is a fetch, which this crate has no authority to perform: it stays the host asset pipeline's
// job (`WorldAssetRequestKind::UiImage`), and until that host resolves it the placeholder chrome
// below is what shows — unchanged from before this region existed.
//
// 🗜️ PNG and JPEG `data:` sources decode through the in-repo `semio-framework-pixels` decoders (the
// same `decode_encoded_blocking` the os renderer's scene-host image path uses), so no third-party
// codec enters the runtime graph. Any other media type answers [`UiImageAdmission::Unsupported`].

/// 🖼️ The largest encoded `src` payload this registry will even look at — the same ceiling the os
/// renderer's own `RETAINED_UI_IMAGE_SOURCE_BYTES` carries for a retained image source.
pub const UI_IMAGE_SOURCE_MAX_BYTES: usize = 4 * 1024 * 1024;

/// 📏️ Largest decoded edge admitted, so one hostile source cannot claim an unbounded texture.
pub const UI_IMAGE_MAX_DIMENSION: u32 = 4096;

/// 📒️ How many decoded sources the registry retains before refusing new ones — a fixed ledger, not
/// an evicting cache, so a frame's draw never races a retirement it did not ask for.
pub const UI_IMAGE_LEDGER_ENTRIES: usize = 64;

/// 🏔️ React's `max-h-64` on the `<img>` (`🗣️Interpreter/🟦️.tsx:1974`): 16rem at the 16px root size.
pub const UI_IMAGE_MAX_BOX_HEIGHT: f32 = 256.0;

/// 🔏️ What admitting one `src` did.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UiImageAdmission {
    /// ✅️ Already decoded (this call or an earlier one) — `ui_image_natural_size` answers its size.
    Ready,
    /// 🌐️ Not decodable in-process (a URL, or a `data:` payload in a codec this crate does not own):
    /// the host asset pipeline owns it.
    Deferred,
    /// 🚫️ A `data:` URL this crate recognises the shape of but owns no decoder for (anything but PNG and JPEG).
    Unsupported,
    /// ⚠️ Malformed source, over-large payload, or a full ledger.
    Refused,
}

/// 📤️ One decoded source waiting for the host to hand it to `RasterTextureTable` — drained through
/// [`take_ui_image_upload`] and pushed as a `prepared::PreparedRasterProducer` under the same `key`
/// the draw list's `KIND_RASTER` instance carries.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UiImageUpload {
    pub key: String,
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<u8>,
}

#[derive(Default)]
struct UiImageLedger {
    entries: Vec<(String, u32, u32)>,
    pending: Vec<UiImageUpload>,
    unsupported: Vec<String>,
}

static UI_IMAGE_LEDGER: std::sync::LazyLock<std::sync::Mutex<UiImageLedger>> = std::sync::LazyLock::new(|| std::sync::Mutex::new(UiImageLedger::default()));

fn ui_image_ledger<R>(read: impl FnOnce(&mut UiImageLedger) -> R) -> Option<R> {
    UI_IMAGE_LEDGER.lock().ok().map(|mut ledger| read(&mut ledger))
}

/// 🛂️ Decodes `src` if this process can, and publishes its natural size + one pending upload. Safe to
/// call every frame: an already-admitted `src` is a map lookup.
pub fn admit_ui_image(src: &str) -> UiImageAdmission {
    if src.is_empty() || src.len() > UI_IMAGE_SOURCE_MAX_BYTES {
        return UiImageAdmission::Refused;
    }
    let known = ui_image_ledger(|ledger| (ledger.entries.iter().any(|(key, _, _)| key == src), ledger.unsupported.iter().any(|key| key == src), ledger.entries.len()));
    let Some((ready, unsupported, admitted)) = known else { return UiImageAdmission::Refused };
    if ready {
        return UiImageAdmission::Ready;
    }
    if unsupported {
        return UiImageAdmission::Unsupported;
    }
    let Some((media, payload)) = split_data_url(src) else { return UiImageAdmission::Deferred };
    if !(media.starts_with("image/png") || media.starts_with("image/jpeg")) {
        ui_image_ledger(|ledger| ledger.unsupported.push(src.to_string()));
        return UiImageAdmission::Unsupported;
    }
    if admitted >= UI_IMAGE_LEDGER_ENTRIES {
        return UiImageAdmission::Refused;
    }
    let Some(bytes) = payload else { return UiImageAdmission::Refused };
    let Ok(decoded) = semio_framework_pixels::image_decoding::decode_encoded_blocking(bytes, semio_framework_pixels::image_decoding::EncodedDecodeLimits::MAXIMUM, &|| false) else { return UiImageAdmission::Refused };
    let image = decoded.image;
    if image.width == 0 || image.height == 0 || image.width > UI_IMAGE_MAX_DIMENSION || image.height > UI_IMAGE_MAX_DIMENSION {
        return UiImageAdmission::Refused;
    }
    let admitted = ui_image_ledger(|ledger| {
        if ledger.entries.len() >= UI_IMAGE_LEDGER_ENTRIES {
            return false;
        }
        ledger.entries.push((src.to_string(), image.width, image.height));
        ledger.pending.push(UiImageUpload { key: src.to_string(), width: image.width, height: image.height, pixels: image.pixels });
        true
    });
    if admitted == Some(true) {
        UiImageAdmission::Ready
    } else {
        UiImageAdmission::Refused
    }
}

/// 🔬️ The decoded pixel size of an admitted `src`, or `None` while the host still owns it.
pub fn ui_image_natural_size(src: &str) -> Option<(u32, u32)> {
    ui_image_ledger(|ledger| ledger.entries.iter().find(|(key, _, _)| key == src).map(|(_, width, height)| (*width, *height)))?
}

/// 📥️ Takes one decoded source off the upload queue — the host's per-frame drain.
pub fn take_ui_image_upload() -> Option<UiImageUpload> {
    ui_image_ledger(|ledger| if ledger.pending.is_empty() { None } else { Some(ledger.pending.remove(0)) })?
}

/// 🧹️ Retires one ledger item per call; answers `true` once the whole registry is terminal-empty.
pub fn close_ui_image_ledger_step() -> bool {
    ui_image_ledger(|ledger| {
        if ledger.pending.pop().is_some() || ledger.entries.pop().is_some() || ledger.unsupported.pop().is_some() {
            return false;
        }
        true
    })
    .unwrap_or(false)
}

/// 🔗️ Splits `data:[<mediatype>][;base64],<payload>` into its media type and decoded bytes. A
/// non-`data:` `src` answers `None` (the host owns it); a `data:` URL whose payload is malformed
/// answers the media type with `None` bytes so the caller can refuse rather than defer.
fn split_data_url(src: &str) -> Option<(&str, Option<Vec<u8>>)> {
    let rest = src.strip_prefix("data:")?;
    let comma = rest.find(',')?;
    let header = &rest[..comma];
    let payload = &rest[comma + 1..];
    let base64 = header.ends_with(";base64");
    let media = header.strip_suffix(";base64").unwrap_or(header);
    let bytes = if base64 { decode_base64(payload) } else { Some(percent_decode(payload)) };
    Some((media, bytes))
}

/// 🔤️ RFC 4648 standard base64 (with `-`/`_` accepted for the URL alphabet), whitespace-tolerant —
/// hand-written because a runtime dependency on a third-party codec is forbidden (AGENTS.md).
fn decode_base64(value: &str) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(value.len() / 4 * 3);
    let mut accumulator: u32 = 0;
    let mut bits = 0u32;
    for byte in value.bytes() {
        let sextet = match byte {
            b'A'..=b'Z' => u32::from(byte - b'A'),
            b'a'..=b'z' => u32::from(byte - b'a') + 26,
            b'0'..=b'9' => u32::from(byte - b'0') + 52,
            b'+' | b'-' => 62,
            b'/' | b'_' => 63,
            b'=' => break,
            b'\n' | b'\r' | b'\t' | b' ' => continue,
            _ => return None,
        };
        accumulator = (accumulator << 6) | sextet;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push(((accumulator >> bits) & 0xFF) as u8);
        }
    }
    Some(out)
}

/// 🔡️ Percent-decoding for a non-base64 `data:` payload, `+` meaning space.
fn percent_decode(value: &str) -> Vec<u8> {
    let bytes = value.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'%' && index + 2 < bytes.len() {
            if let Some(byte) = std::str::from_utf8(&bytes[index + 1..index + 3]).ok().and_then(|hex| u8::from_str_radix(hex, 16).ok()) {
                out.push(byte);
                index += 3;
                continue;
            }
        }
        out.push(if bytes[index] == b'+' { b' ' } else { bytes[index] });
        index += 1;
    }
    out
}

/// 📐️ Where the decoded bitmap actually lands inside the node's laid-out rect: React's
/// `max-h-64 max-w-full object-contain` (`🗣️Interpreter/🟦️.tsx:1974`) — the element's box is the
/// intrinsic size clamped independently on each axis, and `object-contain` then letterboxes the
/// natural aspect ratio inside that box, centred.
pub fn ui_image_content_rect(bounds: Rect, natural_w: u32, natural_h: u32) -> Rect {
    let natural_w = natural_w as f32;
    let natural_h = natural_h as f32;
    if natural_w <= 0.0 || natural_h <= 0.0 || bounds.w <= 0.0 || bounds.h <= 0.0 {
        return bounds;
    }
    let box_w = natural_w.min(bounds.w);
    let box_h = natural_h.min(bounds.h.min(UI_IMAGE_MAX_BOX_HEIGHT));
    let scale = (box_w / natural_w).min(box_h / natural_h);
    let width = natural_w * scale;
    let height = natural_h * scale;
    Rect::new(bounds.x + (box_w - width) * 0.5, bounds.y + (box_h - height) * 0.5, width, height)
}
//#endregion 🖼️UiImageSources

//#region 🦴️Skeleton
// 🦴️ React never paints a half-built element: `interpretUiNodeBusyShell` (`🗣️Interpreter/🟦️.tsx:2147-2154`)
// swaps the whole component for `elementSkeleton(record.component.type)` (`🧱️elements/🦴️Skeletons/🟦️.tsx:54-115`)
// for as long as its `activity` is `loading`/`waiting` — a shaped, element-kind-keyed placeholder,
// not just the border tint this target used to be limited to. `Progress` is the one exemption there
// (a progress bar IS the busy affordance) and is exempt here too.
//
// Shapes are the same *ratios* React's utility classes encode, resolved against this target's own
// theme tokens rather than re-deriving a rem scale: `h-medium` → `Theme::control_height`,
// `h-4`/`h-3` → the body/small font sizes, `gap-single`/`p-single` → `gap_standard`/`padding_standard`.

/// 🦴️ How many blocks the widest skeleton draws — `group`/`section`'s three, plus the header line.
pub const SKELETON_MAX_BLOCKS: usize = 4;

/// 🧩️ The element-kind axis React's `ElementSkeletonKind` names, reduced to the shapes that differ.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SkeletonKind {
    Line,
    Control,
    Separator,
    Image,
    Rows,
    Ring,
    Field,
    Panel,
    Fill,
}

/// 🔖️ `UiNode` → the skeleton React's `elementSkeleton` would pick for the same `Component::type`.
pub fn skeleton_kind(node: &UiNode) -> SkeletonKind {
    match node {
        UiNode::Text(_) => SkeletonKind::Line,
        UiNode::Separator(_) => SkeletonKind::Separator,
        UiNode::Image(_) => SkeletonKind::Image,
        UiNode::Ring(_) => SkeletonKind::Ring,
        UiNode::Field(_) => SkeletonKind::Field,
        UiNode::Section(_) | UiNode::Group(_) | UiNode::OverviewCard(_) => SkeletonKind::Panel,
        UiNode::KeyValue(_) | UiNode::Tree(_) | UiNode::Table(_) => SkeletonKind::Rows,
        UiNode::Stack(_) | UiNode::CardStack(_) | UiNode::ComponentScene(_) | UiNode::ExternalSlot(_) => SkeletonKind::Fill,
        UiNode::Slider(_) => SkeletonKind::Line,
        UiNode::Button(_) | UiNode::Input(_) | UiNode::Select(_) | UiNode::Toggle(_) | UiNode::NumberStepper(_) | UiNode::IconSelect(_) => SkeletonKind::Control,
        UiNode::Progress(_) => SkeletonKind::Line,
    }
}

/// 🧱️ The placeholder blocks one skeleton draws inside `bounds`, in paint order. Fixed-capacity: the
/// retained pass admits `SKELETON_MAX_BLOCKS` items up front and never allocates mid-frame.
pub fn skeleton_blocks(kind: SkeletonKind, bounds: Rect, theme: &Theme) -> ([Rect; SKELETON_MAX_BLOCKS], usize) {
    let mut blocks = [Rect::new(0.0, 0.0, 0.0, 0.0); SKELETON_MAX_BLOCKS];
    let line = theme.font_size_body;
    let small = theme.font_size_small;
    let control = theme.control_height.min(bounds.h.max(0.0));
    let gap = theme.gap_standard;
    let pad = theme.padding_standard;
    let width = bounds.w.max(0.0);
    let count = match kind {
        SkeletonKind::Line => {
            blocks[0] = Rect::new(bounds.x, bounds.y, width * 0.6, line);
            1
        }
        SkeletonKind::Control => {
            blocks[0] = Rect::new(bounds.x, bounds.y, width, control);
            1
        }
        SkeletonKind::Separator => {
            blocks[0] = Rect::new(bounds.x, bounds.y, width, theme.stroke_hairline);
            1
        }
        SkeletonKind::Image => {
            blocks[0] = Rect::new(bounds.x, bounds.y, width, bounds.h.min(UI_IMAGE_MAX_BOX_HEIGHT * 0.5).max(0.0));
            1
        }
        SkeletonKind::Rows => {
            blocks[0] = Rect::new(bounds.x, bounds.y, width, line);
            blocks[1] = Rect::new(bounds.x, bounds.y + line + gap, width * 0.8, line);
            2
        }
        SkeletonKind::Ring => {
            let size = control.min(width);
            blocks[0] = Rect::new(bounds.x, bounds.y, size, size);
            1
        }
        SkeletonKind::Field => {
            blocks[0] = Rect::new(bounds.x, bounds.y, width * 0.4, small);
            blocks[1] = Rect::new(bounds.x, bounds.y + small + gap, width, control);
            2
        }
        SkeletonKind::Panel => {
            let inner = (width - pad * 2.0).max(0.0);
            blocks[0] = Rect::new(bounds.x + pad, bounds.y + pad, inner * 0.5, line);
            blocks[1] = Rect::new(bounds.x + pad, bounds.y + pad + line + gap * 2.0, inner, control);
            blocks[2] = Rect::new(bounds.x + pad, bounds.y + pad + line + control + gap * 4.0, inner, control);
            3
        }
        SkeletonKind::Fill => {
            let inner = (width - pad * 2.0).max(0.0);
            blocks[0] = Rect::new(bounds.x + pad, bounds.y + pad, inner * 0.6, line);
            blocks[1] = Rect::new(bounds.x + pad, bounds.y + pad + line + gap, inner, (bounds.h - pad * 2.0 - line - gap).max(0.0));
            2
        }
    };
    (blocks, count)
}

/// ♻️ Whether this node's presence says React would have replaced it with a skeleton this frame —
/// `interpretUiNodeBusyShell`'s own predicate, `Progress` exemption included.
pub fn skeleton_replaces_content(node: &UiNode, presence: &UiPresence) -> bool {
    !matches!(node, UiNode::Progress(_)) && matches!(presence.status, UiStatus::Loading | UiStatus::Waiting)
}

//#endregion 🦴️Skeleton

//#region 🪟️OverlayChrome
// 🪟️ `events` decides WHERE an overlay goes (`resolve_overlay_placement_side`); this is the other
// half — the chrome React's portalled surfaces carry that a hit-test flag alone does not draw: the
// modal scrim behind a `Dialog`/`CommandPalette` (`🧱️elements/💬️Dialog/🟦️.tsx`'s overlay) and the
// glass panel + hairline every floating surface sits on. `engine::Ui` paints these into the overlay
// layer AFTER the document pass, which is what actually puts an overlay above panel content.

/// 🪜️ The elevation level a floating surface sits on: React mounts a modal (`Dialog`/`CommandPalette`) on `data-level="dialog"` and every other portalled surface (`Popover`, `ChromeControlHint`) on `data-level="menu"`.
pub fn overlay_surface_level(backdrop: bool) -> Level {
    if backdrop {
        Level::Dialog
    } else {
        Level::Menu
    }
}

/// 🌫️ Fills the whole viewport behind a modal overlay as `ui-veil`: the dialog level's surface at `veilAlpha` over a `veilBlurPx` backdrop blur, never a hardcoded alpha.
pub fn paint_overlay_backdrop(draw: &mut DrawList, viewport: Rect, theme: &Theme) -> usize {
    draw.push_glass([viewport.x, viewport.y, viewport.w, viewport.h], 0.0, theme.veil_glass(Level::Dialog))
}

/// 🪟️ The floating surface itself as `ui-glass` at `level` with its hairline border; the returned glass region stays open as glass content, so the caller paints its content and then calls [`DrawList::end_glass_content`]. React's `--shadow-lg` is `0 0 transparent`, so there is no drop shade.
pub fn paint_overlay_surface(draw: &mut DrawList, bounds: Rect, theme: &Theme, level: Level) -> usize {
    let region = draw.push_glass([bounds.x, bounds.y, bounds.w, bounds.h], 0.0, theme.glass(level));
    draw.begin_glass_content(region);
    push_chrome_border(draw, bounds, theme.stroke_hairline, theme.border_normal, true, true, true, true, semio_ui_draw_contract::SharedBorderState::Normal);
    region
}

/// 🎟️ How many quads one overlay's chrome step may emit: a veil, a glass surface, the content brackets and the four hairline border edges, with slack.
pub const RETAINED_OVERLAY_CHROME_ITEMS: usize = 16;

/// 🔓️ One open overlay's chrome as ONE retained step — the modal veil (when its kind has one) and the glass surface it sits on. The surface's glass region is answered so the overlay's own content can be painted as that region's foreground, which is what keeps the translucent surface from tinting the content it exists to carry. `Err` means the step overran its own retained grant, which the frame ladder turns into a fault exactly like any other retained output refusal.
pub fn retained_overlay_chrome_step(draw: &mut DrawList, viewport: Rect, bounds: Rect, backdrop: bool, theme: &Theme) -> Result<Option<usize>, semio_ui_draw_contract::RetainedOutputError> {
    draw.begin_retained_output(RETAINED_OVERLAY_CHROME_ITEMS, RETAINED_NODE_FIXED_OUTPUT_BYTES)?;
    if backdrop {
        paint_overlay_backdrop(draw, viewport, theme);
    }
    let region = (bounds.w > 0.0 && bounds.h > 0.0).then(|| {
        let region = paint_overlay_surface(draw, bounds, theme, overlay_surface_level(backdrop));
        draw.end_glass_content();
        region
    });
    draw.finish_retained_output().map(|_| region)
}

/// 📜️ One scroll host's thumb as ONE retained step: React's `::-webkit-scrollbar-thumb` is `--scrollbar-thumb` (the border colour) while the host is hovered and `--scrollbar-thumb-hover` (the muted foreground) under the pointer or while dragged, square, on a transparent track. Drawn into the overlay bucket so it rides above the host's content.
pub fn retained_scrollbar_step(draw: &mut DrawList, thumb: Rect, hot: bool, theme: &Theme) -> bool {
    draw.begin_retained_output(1, RETAINED_NODE_FIXED_OUTPUT_BYTES).is_ok() && {
        draw.push_solid_overlay([thumb.x, thumb.y, thumb.w, thumb.h], if hot { theme.text_muted } else { theme.border_normal });
        draw.finish_retained_output().is_ok()
    }
}

/// 💡️ A hover tooltip's own surface size for `label`, measured the way the popup will draw it —
/// `🧱️elements/💡️ChromeControlHint/🟦️.tsx`'s `p-single text-xs` glass surface.
pub fn tooltip_surface_size(label: &str, theme: &Theme, atlas: &mut FontAtlas) -> (f32, f32) {
    let advance = atlas.measure_text(label, theme.font_size_small).0;
    (advance + theme.padding_standard * 2.0, theme.font_size_small + theme.padding_standard * 2.0)
}

/// 🖍️ Draws one hover tooltip at an already-resolved origin.
pub fn paint_tooltip(draw: &mut DrawList, atlas: &mut FontAtlas, label: &str, origin: (f32, f32), theme: &Theme) {
    let (width, height) = tooltip_surface_size(label, theme, atlas);
    let bounds = Rect::new(origin.0, origin.1, width, height);
    paint_overlay_surface(draw, bounds, theme, Level::Menu);
    draw_text_on(draw, atlas, label, bounds.x + theme.padding_standard, bounds.y + theme.padding_standard + theme.font_size_small, theme.font_size_small, theme.text);
    draw.end_glass_content();
}
//#endregion 🪟️OverlayChrome

//#region 🎞️MediaOverlay
/// 🎞️ What one native media slot paints over its panel, addressed by the slot's own identity, its window ([`media_window_handle`]) and its document node id: the contained video picture (a keyed raster the host uploaded) and the transport rows, in reading order.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct MediaOverlay {
    pub window: u64,
    pub node_id: u64,
    pub rect: [f32; 4],
    pub quad: Option<MediaOverlayQuad>,
    pub rows: Vec<MediaOverlayRow>,
}

/// 🖼️ The video picture: where it sits and the raster key its pixels were uploaded under.
#[derive(Clone, Debug, PartialEq)]
pub struct MediaOverlayQuad {
    pub rect: [f32; 4],
    pub raster_key: String,
}

/// 🎛️ What a transport row looks like.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaOverlayShape {
    Text,
    Alert,
    Button,
    Slider,
}

/// 🎛️ One drawn transport row: its box, its caption, the share of a slider that is filled and whether it is enabled or holds the focus.
#[derive(Clone, Debug, PartialEq)]
pub struct MediaOverlayRow {
    pub shape: MediaOverlayShape,
    pub rect: [f32; 4],
    pub text: String,
    pub fraction: Option<f32>,
    pub enabled: bool,
    pub focused: bool,
}

static MEDIA_OVERLAYS: std::sync::Mutex<Vec<MediaOverlay>> = std::sync::Mutex::new(Vec::new());

/// 📮️ Replaces the overlays of the next frame's media slots (the host's per-frame publish).
pub fn publish_media_overlays(overlays: Vec<MediaOverlay>) {
    if let Ok(mut held) = MEDIA_OVERLAYS.lock() {
        *held = overlays;
    }
}

static MEDIA_WINDOWS: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

/// 🔑️ The handle of a window id: its exact position in a process-wide intern table (first sight appends), so a window is named by its own id, never by a hash that two ids could share. `0` only when the table is unavailable.
pub fn media_window_handle(window_id: &str) -> u64 {
    let Ok(mut windows) = MEDIA_WINDOWS.lock() else { return 0 };
    let index = windows.iter().position(|known| known == window_id).unwrap_or_else(|| {
        windows.push(window_id.to_owned());
        windows.len() - 1
    });
    index as u64 + 1
}

/// 🔎️ The overlay of node `node_id` of the window with handle `window`.
pub fn media_overlay_for(window: u64, node_id: u64) -> Option<MediaOverlay> {
    MEDIA_OVERLAYS.lock().ok()?.iter().find(|overlay| overlay.window == window && overlay.node_id == node_id).cloned()
}

/// 📤️ Queues one decoded RGBA8 picture under `key` for the host's per-frame upload drain, replacing an earlier pending picture of that key; `false` when the ledger is full or the pixels do not fit the size.
pub fn submit_ui_image_upload(key: &str, width: u32, height: u32, pixels: Vec<u8>) -> bool {
    if width == 0 || height == 0 || width > UI_IMAGE_MAX_DIMENSION || height > UI_IMAGE_MAX_DIMENSION || pixels.len() != width as usize * height as usize * 4 {
        return false;
    }
    ui_image_ledger(|ledger| {
        ledger.pending.retain(|pending| pending.key != key);
        if ledger.pending.len() >= UI_IMAGE_LEDGER_ENTRIES {
            return false;
        }
        ledger.pending.push(UiImageUpload { key: key.to_string(), width, height, pixels });
        true
    })
    .unwrap_or(false)
}

/// 🖍️ Draws one media row's box (the caption follows as its own text step).
pub fn paint_media_overlay_row(draw: &mut DrawList, row: &MediaOverlayRow, theme: &Theme) {
    let [x, y, w, h] = row.rect;
    let fill = if row.focused { theme.selected } else { theme.muted };
    match row.shape {
        MediaOverlayShape::Button => draw.push_rounded([x, y, w, h], if row.enabled { fill } else { theme.panel }, theme.border_radius),
        MediaOverlayShape::Slider => {
            let track = [x, y + h * 0.5 - 2.0, w, 4.0];
            draw.push_rounded(track, theme.muted, 2.0);
            let filled = row.fraction.unwrap_or(0.0).clamp(0.0, 1.0);
            draw.push_rounded([x, track[1], w * filled, 4.0], theme.accent, 2.0);
            draw.push_rounded([x + w * filled - 4.0, y + h * 0.5 - 4.0, 8.0, 8.0], if row.focused { theme.accent_hover } else { theme.accent }, 4.0);
        }
        MediaOverlayShape::Alert => draw.push_rounded([x, y, w, h], theme.panel, theme.border_radius),
        MediaOverlayShape::Text => {}
    }
}
//#endregion 🎞️MediaOverlay
// #endregion paint
