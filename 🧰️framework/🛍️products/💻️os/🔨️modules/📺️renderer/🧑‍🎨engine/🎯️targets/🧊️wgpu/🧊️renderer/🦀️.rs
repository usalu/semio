#[cfg(target_arch = "wasm32")]
extern crate semio_framework_async as wasm_bindgen_futures;

// 🧊️ Raw wgpu WASM renderer for declarative framework UiNode trees.
//
// 🧭️ Rough correspondence with the React shell (`framework/renderer/react/os-shell.tsx`), as a
// discoverability breadcrumb rather than a rigorous mapping:
// - this crate's top-level shell/state struct ~ React's `#region 🔖️types` + `FrameworkOsShell`.
// - the `dock` module below (window tree, stack chrome, split resize) ~ React's `Mode`
//   component and the `WindowLayoutNode` tree helpers in `#region ShellHelpers`.
// - `interpreter`/widget rendering ~ React's `UiNode` component tree rendering.

extern crate framework_surface_node_graph as framework_surface_tiled_map;
extern crate infinite_canvas as infinite_world;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as dsl_core;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as store_sync;
#[cfg(not(target_arch = "wasm32"))]
#[path = "../../../../../../../../🔨️modules/🖼️assets/🥽️mesh/🦀️.rs"]
mod mesh_assets;
#[path = "../../../🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod dock;

#[path = "../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod engine_canvas;

#[path = "../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod interpreter;

#[path = "../../../🧱️elements/🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod program_bridge;

//#region 🏠️🧳️PluginHostConfig
// 🐛️ Lives at the crate root, not inside `program_bridge` above (see that module's own `PluginHostConfig`
// region for why) — this is the file's real directory, so the 3-`..` climb to
// `framework/plugin/registry/generated/🖥️hosts.rs` actually resolves.
#[path = "../../../../../🔌️plugin/📇️registry/🤖️generated/🖥️hosts.rs"]
mod generated_plugin_hosts;
//#endregion 🏠️🧳️PluginHostConfig

#[path = "../../../🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod scenes;

#[path = "../../../🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod shell;

#[path = "../../../🧱️elements/🖼️IconRenderHost/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod icon_atlas;

//#region 🔖️OsHostDecomposition
// 🏠️ ticket 26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (packet os-host): the seam that ends
// this crate owning the actor kernel and ends its continuous redraw. `deadlines`/`kernel_seam` are
// leaves (no dependency on `os_host`/`winit_app`); `os_host` composes `AppRuntime` with them;
// `winit_app` is the new `ApplicationHandler` — see that file's own module docstring for why it
// hand-rolls the event loop instead of using `ui_host::window::NativeHost<D>` directly. Mounted here,
// away from the peer program's `parallel_runtime` mount just below, per this ticket's own OWNS list.
#[path = "../⏰️deadlines/🦀️.rs"]
mod deadlines;

#[path = "../🪢️kernel-seam/🦀️.rs"]
mod kernel_seam;

#[path = "../🏠️os-host/🦀️.rs"]
mod os_host;

#[path = "../📸️render-snapshot/🦀️.rs"]
mod render_snapshot;

#[path = "../📮️runtime-mailbox-core/🦀️.rs"]
mod runtime_mailbox_core;

// 🧵️ P3b (INTERACTIVE-JOB-RUNTIME-REFACTOR, ui-thread-isolation): the `InteractiveJob` seam for the
// slice of `AppRuntime::frame()` that genuinely is `Send`-safe today — see that file's own module
// docstring for exactly what moves and, more importantly, what still cannot.
#[path = "../🧵️frame-job/🦀️.rs"]
mod frame_job;

#[path = "../📐️surface-lane/🦀️.rs"]
mod surface_lane;

#[cfg(target_arch = "wasm32")]
#[path = "../🌐️browser-worker/🦀️.rs"]
mod browser_worker;

#[path = "../🪟️winit-app/🦀️.rs"]
mod winit_app;
//#endregion 🔖️OsHostDecomposition

// 🎠️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-kernel-loop): the real multi-shard `Kernel`
// loop — `ParallelRuntime` — used by both `kernel_runtime` (below) and `scale_bench`. Native-only,
// same reason `kernel_runtime`/`scale_bench` themselves are: native guest execution and the shared
// native worker pool are not available on wasm32.
#[cfg(not(target_arch = "wasm32"))]
#[path = "../🎠️runtime/🦀️.rs"]
pub mod parallel_runtime;

use infinite_world::world::{
    begin_world3d_dynamic_retirement, enqueue_world3d_event, finish_world3d_asset, publish_world3d_asset_mesh_lease, reserve_world3d_asset_response, retire_cancelled_world3d_asset_step, return_world3d_asset, seal_world3d_asset_response,
    step_world3d_draw_rebuild, step_world3d_dynamic_retirement, step_world3d_interaction, step_world3d_snapshot, take_next_completed_world3d_asset_step, take_next_world3d_asset, world3d_asset_cancellation_requested,
    world3d_dynamic_retirement_terminal_is_empty, world3d_interaction_front_generation, World3dSnapshotApplyStep, WorldAssetFault, WorldAssetFetchOwner, WorldAssetIoAuthority, WorldAssetMetadataId, WorldAssetRequestKind, WorldAssetRequestToken,
    WorldAssetResponsePage, WorldDrawRebuildStep, WorldDynamicFault, WorldInteractionAuthorityStep, WorldInteractionIntent, WORLD_ASSET_RESPONSE_BYTE_CAPACITY, WORLD_ASSET_RESPONSE_PAGE_BYTES, WORLD_ASSET_RESPONSE_PAGE_CAPACITY,
};
use program_bridge::filter_plugins;
#[cfg(not(target_arch = "wasm32"))]
use program_bridge::load_wasm_plugins;
#[cfg(target_arch = "wasm32")]
use program_bridge::parse_plugin_entries;
use shell::ShellState;
use std::cell::RefCell;
use std::future::Future;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex, OnceLock};
#[cfg(target_arch = "wasm32")]
use ui_wgpu::wgpu::apply_canvas_cursor;
use ui_wgpu::wgpu::ActionDescriptor;
// 🏚️ `dispatch_window_event`/`WindowInputState`/`schedule_frame` no longer imported here — they were
// `SemioApp`/`start_frame_loop`-only (both deleted, packet os-host); `winit_app.rs` normalizes input
// itself via `ui_host::event` instead. See the `OsHostDecomposition — SemioApp deletion` region above.
#[cfg(target_arch = "wasm32")]
use semio_framework_async::browser::spawn_local;
use ui_wgpu::wgpu::{
    apply_window_cursor, fetch_font_bytes, mesh3d_abort, mesh3d_abort_step, mesh3d_allocate_step, mesh3d_begin, mesh3d_begin_close, mesh3d_close_step, mesh3d_read_write_u32, mesh3d_read_write_vec3, mesh3d_seal, mesh3d_update_vec3, mesh3d_write_u32,
    mesh3d_write_vec2, mesh3d_write_vec3, resolve_semio_cursor, CursorDragState, DrawList, FontAtlas, GpuContext, IconAtlas, InputState, KeyAction, Mesh3dFault, Mesh3dField, Mesh3dItem, Mesh3dLease, Mesh3dSchema, Mesh3dWriteToken, PointerModifiers,
    SemioCursor, Theme,
};
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;
// 🏚️ `ApplicationHandler`/`WindowEvent`/`ActiveEventLoop`/`EventLoopProxy`/`WindowAttributes`/
// `WindowId` no longer imported here — all `SemioApp`-only (deleted, packet os-host); `winit_app.rs`
// imports each of these itself. `EventLoop`/`Window` stay: `run_native`/`semio_wgpu_mount` still
// construct the event loop and `AppRuntime` still names `Window` throughout.
use winit::event_loop::EventLoop;
#[cfg(not(target_arch = "wasm32"))]
use winit::window::Fullscreen;
use winit::window::Window;

//#region 📡️RendererAssetAuthority
static RENDERER_ASSET_IO: OnceLock<Mutex<WorldAssetIoAuthority>> = OnceLock::new();
static RENDERER_ASSET_GENERATION: AtomicU64 = AtomicU64::new(1);

fn renderer_asset_io() -> &'static Mutex<WorldAssetIoAuthority> {
    RENDERER_ASSET_IO.get_or_init(|| Mutex::new(WorldAssetIoAuthority::default()))
}

pub(crate) fn reserve_renderer_asset_request(kind: WorldAssetRequestKind, url: &str) -> Result<WorldAssetRequestToken, WorldAssetFault> {
    let generation = RENDERER_ASSET_GENERATION.fetch_add(1, Ordering::Relaxed).max(1);
    renderer_asset_io().lock().expect("renderer asset authority lock").reserve_request(generation, 0, kind, url)
}

fn take_next_renderer_asset() -> Option<WorldAssetFetchOwner> {
    renderer_asset_io().lock().ok()?.take_next()
}

fn reserve_renderer_asset_response(owner: &mut WorldAssetFetchOwner, byte_credits: usize) -> bool {
    renderer_asset_io().lock().is_ok_and(|mut authority| authority.reserve_response(owner, byte_credits).is_ok())
}

fn seal_renderer_asset_response(owner: &mut WorldAssetFetchOwner) -> bool {
    renderer_asset_io().lock().is_ok_and(|mut authority| authority.seal_response(owner).is_ok())
}

fn return_renderer_asset(owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {
    let Ok(mut authority) = renderer_asset_io().lock() else { return Err(owner) };
    authority.return_owner(owner)
}

fn take_next_completed_renderer_asset_step() -> Option<WorldAssetFetchOwner> {
    renderer_asset_io().lock().ok()?.take_next_completed_step()
}

fn finish_renderer_asset(owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {
    let Ok(mut authority) = renderer_asset_io().lock() else { return Err(owner) };
    authority.finish(owner)
}

fn retire_cancelled_renderer_asset_step() -> bool {
    renderer_asset_io().lock().is_ok_and(|mut authority| authority.retire_cancelled_step())
}

fn close_renderer_asset_step() -> bool {
    let Ok(mut authority) = renderer_asset_io().lock() else { return false };
    authority.begin_close();
    authority.close_step() && authority.terminal_is_empty()
}

pub(crate) enum RendererAssetFetchOwner {
    World { surface: WorldAssetMetadataId, owner: WorldAssetFetchOwner },
    Shared(WorldAssetFetchOwner),
}

impl RendererAssetFetchOwner {
    pub(crate) fn url(&self) -> &str {
        match self {
            Self::World { owner, .. } | Self::Shared(owner) => owner.url(),
        }
    }

    pub(crate) fn kind(&self) -> WorldAssetRequestKind {
        match self {
            Self::World { owner, .. } | Self::Shared(owner) => owner.kind(),
        }
    }

    fn owner_mut(&mut self) -> &mut WorldAssetFetchOwner {
        match self {
            Self::World { owner, .. } | Self::Shared(owner) => owner,
        }
    }

    fn owner(&self) -> &WorldAssetFetchOwner {
        match self {
            Self::World { owner, .. } | Self::Shared(owner) => owner,
        }
    }

    fn begin_close(&mut self) {
        self.owner_mut().begin_close();
    }

    fn close_step(&mut self) -> bool {
        self.owner_mut().close_step()
    }

    fn token(&self) -> WorldAssetRequestToken {
        match self {
            Self::World { owner, .. } | Self::Shared(owner) => owner.token(),
        }
    }

    fn generation(&self) -> u64 {
        self.owner().generation()
    }

    fn revision(&self) -> u64 {
        self.owner().revision()
    }

    fn take_decode_page(&mut self) -> Result<Option<WorldAssetResponsePage>, WorldAssetFault> {
        self.owner_mut().take_decode_page()
    }

    fn decode_page(&self) -> Result<Option<&WorldAssetResponsePage>, WorldAssetFault> {
        self.owner().decode_page()
    }

    fn decode_page_at(&self, index: u16) -> Result<Option<&WorldAssetResponsePage>, WorldAssetFault> {
        self.owner().decode_page_at(index)
    }

    fn advance_decode_page(&mut self) -> Result<(), WorldAssetFault> {
        self.owner_mut().advance_decode_page()
    }

    fn rewind_decode_pages(&mut self) -> Result<(), WorldAssetFault> {
        self.owner_mut().rewind_decode_pages()
    }

    fn received_bytes(&self) -> usize {
        self.owner().received_bytes()
    }
}

const RENDERER_ASSET_PROBE_BYTES: usize = 64;
const RENDERER_ASSET_PARSE_BLOCK_BYTES: usize = 256;
const RENDERER_ASSET_PIXEL_BYTES: usize = 16 * 1024 * 1024;

struct RendererAssetPageIndex {
    starts: Box<[usize; WORLD_ASSET_RESPONSE_PAGE_CAPACITY]>,
    lengths: Box<[u16; WORLD_ASSET_RESPONSE_PAGE_CAPACITY]>,
    len: u16,
    total: usize,
}

impl RendererAssetPageIndex {
    fn new() -> Self {
        Self { starts: Box::new([0; WORLD_ASSET_RESPONSE_PAGE_CAPACITY]), lengths: Box::new([0; WORLD_ASSET_RESPONSE_PAGE_CAPACITY]), len: 0, total: 0 }
    }

    fn admit(&mut self, bytes: usize) -> Result<(), &'static str> {
        let slot = usize::from(self.len);
        if slot == WORLD_ASSET_RESPONSE_PAGE_CAPACITY || bytes == 0 || bytes > WORLD_ASSET_RESPONSE_PAGE_BYTES {
            return Err("asset response page index exceeded fixed credits");
        }
        self.starts[slot] = self.total;
        self.lengths[slot] = u16::try_from(bytes).map_err(|_| "asset response page exceeded fixed byte credits")?;
        self.total = self.total.checked_add(bytes).ok_or("asset response page index overflowed")?;
        self.len += 1;
        Ok(())
    }

    fn locate(&self, absolute: usize) -> Result<(u16, usize), &'static str> {
        if absolute >= self.total {
            return Err("asset semantic read exceeded sealed bytes");
        }
        let mut low = 0usize;
        let mut high = usize::from(self.len);
        while low < high {
            let middle = low + (high - low) / 2;
            if self.starts[middle] <= absolute {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
        let slot = low.checked_sub(1).ok_or("asset semantic page index was empty")?;
        let offset = absolute - self.starts[slot];
        if offset >= usize::from(self.lengths[slot]) {
            return Err("asset semantic page index contained a gap");
        }
        Ok((slot as u16, offset))
    }

    fn read<const N: usize>(&self, owner: &RendererAssetFetchOwner, absolute: usize) -> Result<[u8; N], &'static str> {
        let mut result = [0; N];
        for (delta, output) in result.iter_mut().enumerate() {
            let at = absolute.checked_add(delta).ok_or("asset semantic byte address overflowed")?;
            let (page_index, page_offset) = self.locate(at)?;
            let page = owner.decode_page_at(page_index).map_err(|_| "asset semantic random read lost its response owner")?.ok_or("asset semantic random read lost its response page")?;
            *output = *page.bytes().get(page_offset).ok_or("asset semantic random read exceeded its response page")?;
        }
        Ok(result)
    }
}

struct RendererAssetPageCursor {
    offset: u16,
    absolute: usize,
}

impl RendererAssetPageCursor {
    fn new() -> Self {
        Self { offset: 0, absolute: 0 }
    }

    fn read_block(&mut self, owner: &mut RendererAssetFetchOwner) -> Result<Option<([u8; RENDERER_ASSET_PARSE_BLOCK_BYTES], u16)>, &'static str> {
        let Some(page) = owner.decode_page().map_err(|_| "asset structure cursor lost its response page")? else { return Ok(None) };
        let start = usize::from(self.offset);
        if start >= page.bytes().len() {
            return Err("asset structure cursor exceeded its response page");
        }
        let count = RENDERER_ASSET_PARSE_BLOCK_BYTES.min(page.bytes().len() - start);
        let mut block = [0; RENDERER_ASSET_PARSE_BLOCK_BYTES];
        block[..count].copy_from_slice(&page.bytes()[start..start + count]);
        self.offset += count as u16;
        self.absolute = self.absolute.checked_add(count).ok_or("asset structure cursor byte count overflowed")?;
        if usize::from(self.offset) == page.bytes().len() {
            owner.advance_decode_page().map_err(|_| "asset structure cursor could not advance its response page")?;
            self.offset = 0;
        }
        Ok(Some((block, count as u16)))
    }

    fn read_byte(&mut self, owner: &mut RendererAssetFetchOwner) -> Result<Option<u8>, &'static str> {
        let Some(page) = owner.decode_page().map_err(|_| "asset semantic cursor lost its response page")? else { return Ok(None) };
        let offset = usize::from(self.offset);
        let byte = page.bytes().get(offset).copied().ok_or("asset semantic cursor exceeded its response page")?;
        self.offset += 1;
        self.absolute = self.absolute.checked_add(1).ok_or("asset semantic cursor byte count overflowed")?;
        if usize::from(self.offset) == page.bytes().len() {
            owner.advance_decode_page().map_err(|_| "asset semantic cursor could not advance its response page")?;
            self.offset = 0;
        }
        Ok(Some(byte))
    }
}

enum RendererAssetFormatCursor {
    Glb(GlbStructureCursor),
    Png(PngStructureCursor),
    Jpeg(JpegStructureCursor),
    Svg(TextAssetStructureCursor),
    Protobuf(ProtobufStructureCursor),
    Opaque(TextAssetStructureCursor),
}

impl RendererAssetFormatCursor {
    fn new(kind: WorldAssetRequestKind, prefix: &[u8], total_bytes: usize) -> Result<Self, &'static str> {
        match kind {
            WorldAssetRequestKind::Glb => Ok(Self::Glb(GlbStructureCursor::new(total_bytes))),
            WorldAssetRequestKind::ReferenceImage | WorldAssetRequestKind::UiImage { .. } | WorldAssetRequestKind::MapTile { vector: false, .. } => {
                if prefix.starts_with(b"\x89PNG\r\n\x1a\n") {
                    Ok(Self::Png(PngStructureCursor::new(total_bytes)))
                } else if prefix.starts_with(&[0xff, 0xd8, 0xff]) {
                    Ok(Self::Jpeg(JpegStructureCursor::new(total_bytes)))
                } else {
                    Ok(Self::Svg(TextAssetStructureCursor::new(total_bytes, true, true)))
                }
            }
            WorldAssetRequestKind::MapTile { vector: true, .. } => Ok(Self::Protobuf(ProtobufStructureCursor::new(total_bytes))),
            WorldAssetRequestKind::Terrain { .. } => Ok(Self::Opaque(TextAssetStructureCursor::new(total_bytes, false, false))),
        }
    }

    fn feed(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        match self {
            Self::Glb(cursor) => cursor.feed(bytes),
            Self::Png(cursor) => cursor.feed(bytes),
            Self::Jpeg(cursor) => cursor.feed(bytes),
            Self::Svg(cursor) | Self::Opaque(cursor) => cursor.feed(bytes),
            Self::Protobuf(cursor) => cursor.feed(bytes),
        }
    }

    fn finish(&self) -> Result<(), &'static str> {
        match self {
            Self::Glb(cursor) => cursor.finish(),
            Self::Png(cursor) => cursor.finish(),
            Self::Jpeg(cursor) => cursor.finish(),
            Self::Svg(cursor) | Self::Opaque(cursor) => cursor.finish(),
            Self::Protobuf(cursor) => cursor.finish(),
        }
    }
}

enum GlbStructurePhase {
    Header { bytes: [u8; 12], len: u8 },
    ChunkHeader { bytes: [u8; 8], len: u8 },
    ChunkPayload { remaining: u32 },
    Terminal,
}

struct GlbStructureCursor {
    total_bytes: usize,
    consumed: usize,
    json: bool,
    bin: bool,
    phase: GlbStructurePhase,
}

impl GlbStructureCursor {
    fn new(total_bytes: usize) -> Self {
        Self { total_bytes, consumed: 0, json: false, bin: false, phase: GlbStructurePhase::Header { bytes: [0; 12], len: 0 } }
    }

    fn feed(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        for &byte in bytes {
            self.consumed = self.consumed.checked_add(1).ok_or("GLB byte count overflowed")?;
            if self.consumed > self.total_bytes {
                return Err("GLB exceeded its declared byte claim");
            }
            match &mut self.phase {
                GlbStructurePhase::Header { bytes, len } => {
                    bytes[usize::from(*len)] = byte;
                    *len += 1;
                    if *len == 12 {
                        if &bytes[..4] != b"glTF" || u32::from_le_bytes(bytes[4..8].try_into().expect("fixed GLB version")) != 2 || usize::try_from(u32::from_le_bytes(bytes[8..12].try_into().expect("fixed GLB length"))).ok() != Some(self.total_bytes)
                        {
                            return Err("GLB header was invalid");
                        }
                        self.phase = GlbStructurePhase::ChunkHeader { bytes: [0; 8], len: 0 };
                    }
                }
                GlbStructurePhase::ChunkHeader { bytes, len } => {
                    bytes[usize::from(*len)] = byte;
                    *len += 1;
                    if *len == 8 {
                        let length = u32::from_le_bytes(bytes[..4].try_into().expect("fixed GLB chunk length"));
                        let kind = u32::from_le_bytes(bytes[4..8].try_into().expect("fixed GLB chunk kind"));
                        if length == 0 || !length.is_multiple_of(4) || self.consumed.checked_add(length as usize).is_none_or(|end| end > self.total_bytes) {
                            return Err("GLB chunk exceeded fixed structure credits");
                        }
                        match kind {
                            0x4e4f534a if !self.json && !self.bin => self.json = true,
                            0x004e4942 if self.json && !self.bin => self.bin = true,
                            _ => return Err("GLB chunk order or kind was unsupported"),
                        }
                        self.phase = GlbStructurePhase::ChunkPayload { remaining: length };
                    }
                }
                GlbStructurePhase::ChunkPayload { remaining } => {
                    *remaining -= 1;
                    if *remaining == 0 {
                        self.phase = if self.consumed == self.total_bytes { GlbStructurePhase::Terminal } else { GlbStructurePhase::ChunkHeader { bytes: [0; 8], len: 0 } };
                    }
                }
                GlbStructurePhase::Terminal => return Err("GLB contained trailing bytes"),
            }
        }
        Ok(())
    }

    fn finish(&self) -> Result<(), &'static str> {
        if self.consumed == self.total_bytes && self.json && self.bin && matches!(self.phase, GlbStructurePhase::Terminal) {
            Ok(())
        } else {
            Err("GLB structure ended before JSON and BIN reached terminal")
        }
    }
}

enum PngStructurePhase {
    Signature { bytes: [u8; 8], len: u8 },
    ChunkHeader { bytes: [u8; 8], len: u8 },
    ChunkPayload { remaining: u32, kind: [u8; 4], ihdr: [u8; 13], ihdr_len: u8 },
    Crc { remaining: u8, terminal_after: bool },
    Terminal,
}

struct PngStructureCursor {
    total_bytes: usize,
    consumed: usize,
    ihdr: bool,
    idat: bool,
    phase: PngStructurePhase,
}

impl PngStructureCursor {
    fn new(total_bytes: usize) -> Self {
        Self { total_bytes, consumed: 0, ihdr: false, idat: false, phase: PngStructurePhase::Signature { bytes: [0; 8], len: 0 } }
    }

    fn feed(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        for &byte in bytes {
            self.consumed = self.consumed.checked_add(1).ok_or("PNG byte count overflowed")?;
            if self.consumed > self.total_bytes {
                return Err("PNG exceeded its sealed byte claim");
            }
            match &mut self.phase {
                PngStructurePhase::Signature { bytes, len } => {
                    bytes[usize::from(*len)] = byte;
                    *len += 1;
                    if *len == 8 {
                        if bytes != b"\x89PNG\r\n\x1a\n" {
                            return Err("PNG signature was invalid");
                        }
                        self.phase = PngStructurePhase::ChunkHeader { bytes: [0; 8], len: 0 };
                    }
                }
                PngStructurePhase::ChunkHeader { bytes, len } => {
                    bytes[usize::from(*len)] = byte;
                    *len += 1;
                    if *len == 8 {
                        let length = u32::from_be_bytes(bytes[..4].try_into().expect("fixed PNG chunk length"));
                        let kind: [u8; 4] = bytes[4..8].try_into().expect("fixed PNG chunk kind");
                        let terminal_after = kind == *b"IEND";
                        if self.consumed.checked_add(length as usize).and_then(|end| end.checked_add(4)).is_none_or(|end| end > self.total_bytes) || (kind == *b"IHDR" && (self.ihdr || length != 13)) || (terminal_after && length != 0) {
                            return Err("PNG chunk exceeded fixed structure credits");
                        }
                        if length == 0 {
                            self.phase = PngStructurePhase::Crc { remaining: 4, terminal_after };
                        } else {
                            self.phase = PngStructurePhase::ChunkPayload { remaining: length, kind, ihdr: [0; 13], ihdr_len: 0 };
                        }
                    }
                }
                PngStructurePhase::ChunkPayload { remaining, kind, ihdr, ihdr_len } => {
                    if *kind == *b"IHDR" {
                        ihdr[usize::from(*ihdr_len)] = byte;
                        *ihdr_len += 1;
                    }
                    *remaining -= 1;
                    if *remaining == 0 {
                        if *kind == *b"IHDR" {
                            let width = u32::from_be_bytes(ihdr[..4].try_into().expect("fixed PNG width"));
                            let height = u32::from_be_bytes(ihdr[4..8].try_into().expect("fixed PNG height"));
                            let pixel_bytes = usize::try_from(width).ok().and_then(|width| usize::try_from(height).ok().and_then(|height| width.checked_mul(height))).and_then(|pixels| pixels.checked_mul(4));
                            if width == 0 || height == 0 || pixel_bytes.is_none_or(|bytes| bytes > RENDERER_ASSET_PIXEL_BYTES) {
                                return Err("PNG dimensions exceeded fixed pixel credits");
                            }
                            self.ihdr = true;
                        } else if *kind == *b"IDAT" {
                            self.idat = true;
                        }
                        self.phase = PngStructurePhase::Crc { remaining: 4, terminal_after: false };
                    }
                }
                PngStructurePhase::Crc { remaining, terminal_after } => {
                    *remaining -= 1;
                    if *remaining == 0 {
                        self.phase = if *terminal_after { PngStructurePhase::Terminal } else { PngStructurePhase::ChunkHeader { bytes: [0; 8], len: 0 } };
                    }
                }
                PngStructurePhase::Terminal => return Err("PNG contained trailing bytes"),
            }
        }
        Ok(())
    }

    fn finish(&self) -> Result<(), &'static str> {
        if self.consumed == self.total_bytes && self.ihdr && self.idat && matches!(self.phase, PngStructurePhase::Terminal) {
            Ok(())
        } else {
            Err("PNG structure ended before IHDR, IDAT, and IEND reached terminal")
        }
    }
}

enum JpegStructurePhase {
    Start { bytes: [u8; 2], len: u8 },
    MarkerPrefix,
    Marker,
    SegmentLength { marker: u8, bytes: [u8; 2], len: u8 },
    Segment { marker: u8, remaining: u16, prefix: [u8; 5], prefix_len: u8 },
    Scan,
    ScanMarker,
    Terminal,
}

struct JpegStructureCursor {
    total_bytes: usize,
    consumed: usize,
    dimensions: bool,
    phase: JpegStructurePhase,
}

impl JpegStructureCursor {
    fn new(total_bytes: usize) -> Self {
        Self { total_bytes, consumed: 0, dimensions: false, phase: JpegStructurePhase::Start { bytes: [0; 2], len: 0 } }
    }

    fn feed(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        for &byte in bytes {
            self.consumed = self.consumed.checked_add(1).ok_or("JPEG byte count overflowed")?;
            if self.consumed > self.total_bytes {
                return Err("JPEG exceeded its sealed byte claim");
            }
            match &mut self.phase {
                JpegStructurePhase::Start { bytes, len } => {
                    bytes[usize::from(*len)] = byte;
                    *len += 1;
                    if *len == 2 {
                        if *bytes != [0xff, 0xd8] {
                            return Err("JPEG start marker was invalid");
                        }
                        self.phase = JpegStructurePhase::MarkerPrefix;
                    }
                }
                JpegStructurePhase::MarkerPrefix => {
                    if byte != 0xff {
                        return Err("JPEG marker prefix was invalid");
                    }
                    self.phase = JpegStructurePhase::Marker;
                }
                JpegStructurePhase::Marker => match byte {
                    0xff => {}
                    0xd9 => self.phase = JpegStructurePhase::Terminal,
                    0x01 | 0xd0..=0xd7 => self.phase = JpegStructurePhase::MarkerPrefix,
                    marker => self.phase = JpegStructurePhase::SegmentLength { marker, bytes: [0; 2], len: 0 },
                },
                JpegStructurePhase::SegmentLength { marker, bytes, len } => {
                    bytes[usize::from(*len)] = byte;
                    *len += 1;
                    if *len == 2 {
                        let length = u16::from_be_bytes(*bytes);
                        if length < 2 || self.consumed.checked_add(usize::from(length - 2)).is_none_or(|end| end > self.total_bytes) {
                            return Err("JPEG segment exceeded fixed structure credits");
                        }
                        self.phase = JpegStructurePhase::Segment { marker: *marker, remaining: length - 2, prefix: [0; 5], prefix_len: 0 };
                    }
                }
                JpegStructurePhase::Segment { marker, remaining, prefix, prefix_len } => {
                    if usize::from(*prefix_len) < prefix.len() {
                        prefix[usize::from(*prefix_len)] = byte;
                        *prefix_len += 1;
                    }
                    *remaining -= 1;
                    if *remaining == 0 {
                        if matches!(*marker, 0xc0..=0xc3 | 0xc5..=0xc7 | 0xc9..=0xcb | 0xcd..=0xcf) {
                            if *prefix_len < 5 {
                                return Err("JPEG frame header was truncated");
                            }
                            let height = u16::from_be_bytes([prefix[1], prefix[2]]);
                            let width = u16::from_be_bytes([prefix[3], prefix[4]]);
                            let pixel_bytes = usize::from(width).checked_mul(usize::from(height)).and_then(|pixels| pixels.checked_mul(4));
                            if width == 0 || height == 0 || pixel_bytes.is_none_or(|bytes| bytes > RENDERER_ASSET_PIXEL_BYTES) {
                                return Err("JPEG dimensions exceeded fixed pixel credits");
                            }
                            self.dimensions = true;
                        }
                        self.phase = if *marker == 0xda { JpegStructurePhase::Scan } else { JpegStructurePhase::MarkerPrefix };
                    }
                }
                JpegStructurePhase::Scan => {
                    if byte == 0xff {
                        self.phase = JpegStructurePhase::ScanMarker;
                    }
                }
                JpegStructurePhase::ScanMarker => match byte {
                    0x00 | 0xd0..=0xd7 => self.phase = JpegStructurePhase::Scan,
                    0xff => {}
                    0xd9 => self.phase = JpegStructurePhase::Terminal,
                    marker => self.phase = JpegStructurePhase::SegmentLength { marker, bytes: [0; 2], len: 0 },
                },
                JpegStructurePhase::Terminal => return Err("JPEG contained trailing bytes"),
            }
        }
        Ok(())
    }

    fn finish(&self) -> Result<(), &'static str> {
        if self.consumed == self.total_bytes && self.dimensions && matches!(self.phase, JpegStructurePhase::Terminal) {
            Ok(())
        } else {
            Err("JPEG structure ended before a bounded frame and EOI reached terminal")
        }
    }
}

struct TextAssetStructureCursor {
    total_bytes: usize,
    consumed: usize,
    require_svg: bool,
    validate_utf8: bool,
    saw_open_angle: bool,
    saw_svg: bool,
    utf8_remaining: u8,
}

impl TextAssetStructureCursor {
    fn new(total_bytes: usize, require_svg: bool, validate_utf8: bool) -> Self {
        Self { total_bytes, consumed: 0, require_svg, validate_utf8, saw_open_angle: false, saw_svg: false, utf8_remaining: 0 }
    }

    fn feed(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        for &byte in bytes {
            self.consumed = self.consumed.checked_add(1).ok_or("text asset byte count overflowed")?;
            if self.consumed > self.total_bytes {
                return Err("text asset exceeded its sealed byte claim");
            }
            if !self.validate_utf8 {
                continue;
            }
            if self.utf8_remaining != 0 {
                if byte & 0xc0 != 0x80 {
                    return Err("text asset contained invalid UTF-8 continuation");
                }
                self.utf8_remaining -= 1;
            } else if byte >= 0x80 {
                self.utf8_remaining = match byte {
                    0xc2..=0xdf => 1,
                    0xe0..=0xef => 2,
                    0xf0..=0xf4 => 3,
                    _ => return Err("text asset contained invalid UTF-8 lead byte"),
                };
            }
            if byte == b'<' {
                self.saw_open_angle = true;
            } else if self.saw_open_angle && (byte == b's' || byte == b'S') {
                self.saw_svg = true;
            }
        }
        Ok(())
    }

    fn finish(&self) -> Result<(), &'static str> {
        if self.consumed == self.total_bytes && (!self.validate_utf8 || self.utf8_remaining == 0) && (!self.require_svg || self.saw_svg) {
            Ok(())
        } else {
            Err("text asset structure did not reach its bounded terminal witness")
        }
    }
}

enum ProtobufStructurePhase {
    Key { value: u64, shift: u8 },
    Varint { value: u64, shift: u8 },
    Fixed { remaining: u8 },
    Length { value: u64, shift: u8 },
    Bytes { remaining: u64 },
}

struct ProtobufStructureCursor {
    total_bytes: usize,
    consumed: usize,
    fields: usize,
    phase: ProtobufStructurePhase,
}

impl ProtobufStructureCursor {
    fn new(total_bytes: usize) -> Self {
        Self { total_bytes, consumed: 0, fields: 0, phase: ProtobufStructurePhase::Key { value: 0, shift: 0 } }
    }

    fn feed(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        for &byte in bytes {
            self.consumed = self.consumed.checked_add(1).ok_or("protobuf byte count overflowed")?;
            if self.consumed > self.total_bytes {
                return Err("protobuf exceeded its sealed byte claim");
            }
            match &mut self.phase {
                ProtobufStructurePhase::Key { value, shift } => {
                    protobuf_varint_byte(value, shift, byte)?;
                    if byte & 0x80 == 0 {
                        let key = *value;
                        let field = key >> 3;
                        let wire = (key & 7) as u8;
                        if field == 0 {
                            return Err("protobuf field zero is invalid");
                        }
                        self.fields = self.fields.checked_add(1).ok_or("protobuf field count overflowed")?;
                        self.phase = match wire {
                            0 => ProtobufStructurePhase::Varint { value: 0, shift: 0 },
                            1 => ProtobufStructurePhase::Fixed { remaining: 8 },
                            2 => ProtobufStructurePhase::Length { value: 0, shift: 0 },
                            5 => ProtobufStructurePhase::Fixed { remaining: 4 },
                            _ => return Err("protobuf wire type was unsupported"),
                        };
                    }
                }
                ProtobufStructurePhase::Varint { value, shift } => {
                    protobuf_varint_byte(value, shift, byte)?;
                    if byte & 0x80 == 0 {
                        self.phase = ProtobufStructurePhase::Key { value: 0, shift: 0 };
                    }
                }
                ProtobufStructurePhase::Fixed { remaining } => {
                    *remaining -= 1;
                    if *remaining == 0 {
                        self.phase = ProtobufStructurePhase::Key { value: 0, shift: 0 };
                    }
                }
                ProtobufStructurePhase::Length { value, shift } => {
                    protobuf_varint_byte(value, shift, byte)?;
                    if byte & 0x80 == 0 {
                        if *value > (self.total_bytes - self.consumed) as u64 {
                            return Err("protobuf length-delimited field exceeded fixed credits");
                        }
                        self.phase = if *value == 0 { ProtobufStructurePhase::Key { value: 0, shift: 0 } } else { ProtobufStructurePhase::Bytes { remaining: *value } };
                    }
                }
                ProtobufStructurePhase::Bytes { remaining } => {
                    *remaining -= 1;
                    if *remaining == 0 {
                        self.phase = ProtobufStructurePhase::Key { value: 0, shift: 0 };
                    }
                }
            }
        }
        Ok(())
    }

    fn finish(&self) -> Result<(), &'static str> {
        if self.consumed == self.total_bytes && self.fields != 0 && matches!(self.phase, ProtobufStructurePhase::Key { value: 0, shift: 0 }) {
            Ok(())
        } else {
            Err("protobuf structure did not reach a field boundary")
        }
    }
}

fn protobuf_varint_byte(value: &mut u64, shift: &mut u8, byte: u8) -> Result<(), &'static str> {
    if *shift >= 64 || (*shift == 63 && byte > 1) {
        return Err("protobuf varint exceeded fixed width");
    }
    *value |= u64::from(byte & 0x7f) << *shift;
    if byte & 0x80 != 0 {
        *shift += 7;
    }
    Ok(())
}

const GLB_JSON_ATOM_BYTES: usize = 64;
const GLB_SCHEMA_ITEM_CAPACITY: usize = 512;
const GLB_SCHEMA_OUTPUT_BYTES: usize = 16 * 1024 * 1024;

#[derive(Clone, Copy)]
struct GlbJsonAtom {
    bytes: [u8; GLB_JSON_ATOM_BYTES],
    len: u8,
    total: u32,
    overflow: bool,
}

impl GlbJsonAtom {
    fn new() -> Self {
        Self { bytes: [0; GLB_JSON_ATOM_BYTES], len: 0, total: 0, overflow: false }
    }

    fn push(&mut self, byte: u8) -> Result<(), &'static str> {
        self.total = self.total.checked_add(1).ok_or("GLB JSON atom length overflowed")?;
        if usize::from(self.len) < self.bytes.len() {
            self.bytes[usize::from(self.len)] = byte;
            self.len += 1;
        } else {
            self.overflow = true;
        }
        Ok(())
    }

    fn equals(&self, expected: &str) -> bool {
        !self.overflow && usize::from(self.len) == expected.len() && &self.bytes[..usize::from(self.len)] == expected.as_bytes()
    }

    fn unsigned(&self) -> Result<u64, &'static str> {
        if self.overflow || self.len == 0 {
            return Err("GLB JSON integer exceeded fixed atom credits");
        }
        std::str::from_utf8(&self.bytes[..usize::from(self.len)]).ok().and_then(|value| value.parse().ok()).ok_or("GLB JSON integer was invalid")
    }

    fn float(&self) -> Result<f32, &'static str> {
        if self.overflow || self.len == 0 {
            return Err("GLB JSON number exceeded fixed atom credits");
        }
        let value = std::str::from_utf8(&self.bytes[..usize::from(self.len)]).ok().and_then(|value| value.parse::<f32>().ok()).ok_or("GLB JSON number was invalid")?;
        value.is_finite().then_some(value).ok_or("GLB JSON number was not finite")
    }
}

enum GlbJsonToken {
    ObjectStart,
    ObjectEnd,
    ArrayStart,
    ArrayEnd,
    Colon,
    Comma,
    String(GlbJsonAtom),
    Number(GlbJsonAtom),
    Literal(Option<bool>),
}

enum GlbJsonLexPhase {
    Idle,
    String { atom: GlbJsonAtom, escape: bool, unicode_remaining: u8 },
    Number(GlbJsonAtom),
    Literal { expected: &'static [u8], index: u8, value: Option<bool> },
}

struct GlbJsonTokenCursor {
    phase: GlbJsonLexPhase,
    replay: Option<u8>,
}

impl GlbJsonTokenCursor {
    fn new() -> Self {
        Self { phase: GlbJsonLexPhase::Idle, replay: None }
    }

    fn feed(&mut self, byte: u8) -> Result<Option<GlbJsonToken>, &'static str> {
        let byte = self.replay.take().unwrap_or(byte);
        match &mut self.phase {
            GlbJsonLexPhase::Idle => match byte {
                b' ' | b'\n' | b'\r' | b'\t' => Ok(None),
                b'{' => Ok(Some(GlbJsonToken::ObjectStart)),
                b'}' => Ok(Some(GlbJsonToken::ObjectEnd)),
                b'[' => Ok(Some(GlbJsonToken::ArrayStart)),
                b']' => Ok(Some(GlbJsonToken::ArrayEnd)),
                b':' => Ok(Some(GlbJsonToken::Colon)),
                b',' => Ok(Some(GlbJsonToken::Comma)),
                b'"' => {
                    self.phase = GlbJsonLexPhase::String { atom: GlbJsonAtom::new(), escape: false, unicode_remaining: 0 };
                    Ok(None)
                }
                b'-' | b'0'..=b'9' => {
                    let mut atom = GlbJsonAtom::new();
                    atom.push(byte)?;
                    self.phase = GlbJsonLexPhase::Number(atom);
                    Ok(None)
                }
                b't' => {
                    self.phase = GlbJsonLexPhase::Literal { expected: b"true", index: 1, value: Some(true) };
                    Ok(None)
                }
                b'f' => {
                    self.phase = GlbJsonLexPhase::Literal { expected: b"false", index: 1, value: Some(false) };
                    Ok(None)
                }
                b'n' => {
                    self.phase = GlbJsonLexPhase::Literal { expected: b"null", index: 1, value: None };
                    Ok(None)
                }
                _ => Err("GLB JSON token was invalid"),
            },
            GlbJsonLexPhase::String { atom, escape, unicode_remaining } => {
                if *unicode_remaining != 0 {
                    if !byte.is_ascii_hexdigit() {
                        return Err("GLB JSON unicode escape was invalid");
                    }
                    *unicode_remaining -= 1;
                    if *unicode_remaining == 0 {
                        atom.push(b'?')?;
                    }
                    return Ok(None);
                }
                if *escape {
                    *escape = false;
                    match byte {
                        b'"' | b'\\' | b'/' => atom.push(byte)?,
                        b'b' => atom.push(8)?,
                        b'f' => atom.push(12)?,
                        b'n' => atom.push(b'\n')?,
                        b'r' => atom.push(b'\r')?,
                        b't' => atom.push(b'\t')?,
                        b'u' => *unicode_remaining = 4,
                        _ => return Err("GLB JSON escape was invalid"),
                    }
                    return Ok(None);
                }
                match byte {
                    b'\\' => {
                        *escape = true;
                        Ok(None)
                    }
                    b'"' => {
                        let atom = *atom;
                        self.phase = GlbJsonLexPhase::Idle;
                        Ok(Some(GlbJsonToken::String(atom)))
                    }
                    0..=31 => Err("GLB JSON string contained a control byte"),
                    _ => {
                        atom.push(byte)?;
                        Ok(None)
                    }
                }
            }
            GlbJsonLexPhase::Number(atom) => {
                if matches!(byte, b'0'..=b'9' | b'-' | b'+' | b'.' | b'e' | b'E') {
                    atom.push(byte)?;
                    Ok(None)
                } else {
                    let atom = *atom;
                    self.phase = GlbJsonLexPhase::Idle;
                    self.replay = Some(byte);
                    Ok(Some(GlbJsonToken::Number(atom)))
                }
            }
            GlbJsonLexPhase::Literal { expected, index, value } => {
                if expected.get(usize::from(*index)).copied() != Some(byte) {
                    return Err("GLB JSON literal was invalid");
                }
                *index += 1;
                if usize::from(*index) == expected.len() {
                    let value = *value;
                    self.phase = GlbJsonLexPhase::Idle;
                    Ok(Some(GlbJsonToken::Literal(value)))
                } else {
                    Ok(None)
                }
            }
        }
    }

    fn replay(&mut self) -> Result<Option<GlbJsonToken>, &'static str> {
        let Some(byte) = self.replay else { return Ok(None) };
        self.feed(byte)
    }

    fn finish(&mut self) -> Result<Option<GlbJsonToken>, &'static str> {
        match std::mem::replace(&mut self.phase, GlbJsonLexPhase::Idle) {
            GlbJsonLexPhase::Idle => self.replay(),
            GlbJsonLexPhase::Number(atom) => Ok(Some(GlbJsonToken::Number(atom))),
            _ => Err("GLB JSON token ended before terminal"),
        }
    }
}

#[derive(Clone, Copy, Default)]
struct GlbAccessorSchema {
    view: u16,
    byte_offset: u32,
    component: u16,
    count: u32,
    kind: u8,
    view_set: bool,
    normalized: bool,
}

#[derive(Clone, Copy, Default)]
struct GlbViewSchema {
    byte_offset: u32,
    byte_length: u32,
    byte_stride: u16,
}

#[derive(Clone, Copy)]
struct GlbPrimitiveSchema {
    position: u16,
    normal: Option<u16>,
    uv: Option<u16>,
    indices: Option<u16>,
    mode: u8,
    position_set: bool,
}

#[derive(Clone, Copy, Default)]
struct GlbMeshSchema {
    primitive_start: u16,
    primitive_len: u16,
}

#[derive(Clone, Copy)]
struct GlbNodeSchema {
    mesh: Option<u16>,
    child_start: u16,
    child_len: u16,
    translation: [f32; 3],
    rotation: [f32; 4],
    scale: [f32; 3],
    matrix: Option<[f32; 16]>,
}

impl Default for GlbNodeSchema {
    fn default() -> Self {
        Self { mesh: None, child_start: 0, child_len: 0, translation: [0.0; 3], rotation: [0.0, 0.0, 0.0, 1.0], scale: [1.0; 3], matrix: None }
    }
}

#[derive(Clone, Copy, Default)]
struct GlbSceneSchema {
    node_start: u16,
    node_len: u16,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GlbNumericArrayKind {
    NodeChildren,
    NodeTranslation,
    NodeRotation,
    NodeScale,
    NodeMatrix,
    SceneNodes,
}

struct GlbNumericArray {
    kind: GlbNumericArrayKind,
    depth: u16,
    start: u16,
    values: [f32; 16],
    len: u16,
}

impl GlbNumericArray {
    fn new(kind: GlbNumericArrayKind, depth: u16, start: u16) -> Self {
        Self { kind, depth, start, values: [0.0; 16], len: 0 }
    }

    fn push(&mut self, value: f32) -> Result<(), &'static str> {
        let slot = usize::from(self.len);
        if slot == self.values.len() {
            return Err("GLB node or scene array exceeded fixed item credits");
        }
        self.values[slot] = value;
        self.len += 1;
        Ok(())
    }
}

impl Default for GlbPrimitiveSchema {
    fn default() -> Self {
        Self { position: 0, normal: None, uv: None, indices: None, mode: 4, position_set: false }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GlbSchemaSection {
    None,
    Accessors,
    Views,
    Meshes,
    Nodes,
    Scenes,
}

struct GlbSchemaOutput {
    accessors: Box<[Option<GlbAccessorSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    views: Box<[Option<GlbViewSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    primitives: Box<[Option<GlbPrimitiveSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    meshes: Box<[Option<GlbMeshSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    nodes: Box<[Option<GlbNodeSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    scenes: Box<[Option<GlbSceneSchema>; GLB_SCHEMA_ITEM_CAPACITY]>,
    node_edges: Box<[Option<u16>; GLB_SCHEMA_ITEM_CAPACITY]>,
    scene_roots: Box<[Option<u16>; GLB_SCHEMA_ITEM_CAPACITY]>,
    accessor_len: u16,
    view_len: u16,
    primitive_len: u16,
    mesh_len: u16,
    node_len: u16,
    scene_len: u16,
    node_edge_len: u16,
    scene_root_len: u16,
    default_scene: Option<u16>,
    json_bytes: u32,
    output_bytes: usize,
}

impl GlbSchemaOutput {
    fn new() -> Self {
        Self {
            accessors: Box::new([None; GLB_SCHEMA_ITEM_CAPACITY]),
            views: Box::new([None; GLB_SCHEMA_ITEM_CAPACITY]),
            primitives: Box::new([None; GLB_SCHEMA_ITEM_CAPACITY]),
            meshes: Box::new([None; GLB_SCHEMA_ITEM_CAPACITY]),
            nodes: Box::new([None; GLB_SCHEMA_ITEM_CAPACITY]),
            scenes: Box::new([None; GLB_SCHEMA_ITEM_CAPACITY]),
            node_edges: Box::new([None; GLB_SCHEMA_ITEM_CAPACITY]),
            scene_roots: Box::new([None; GLB_SCHEMA_ITEM_CAPACITY]),
            accessor_len: 0,
            view_len: 0,
            primitive_len: 0,
            mesh_len: 0,
            node_len: 0,
            scene_len: 0,
            node_edge_len: 0,
            scene_root_len: 0,
            default_scene: None,
            json_bytes: 0,
            output_bytes: 0,
        }
    }

    fn validate(&mut self) -> Result<(), &'static str> {
        if let Some(scene) = self.default_scene {
            if usize::from(scene) >= usize::from(self.scene_len) {
                return Err("GLB default scene referenced a missing scene");
            }
        }
        for node in self.nodes[..usize::from(self.node_len)].iter().flatten() {
            if node.mesh.is_some_and(|mesh| usize::from(mesh) >= usize::from(self.mesh_len)) {
                return Err("GLB node referenced a missing mesh");
            }
            let start = usize::from(node.child_start);
            let end = start.checked_add(usize::from(node.child_len)).ok_or("GLB child range overflowed")?;
            if end > usize::from(self.node_edge_len) || self.node_edges[start..end].iter().flatten().any(|child| usize::from(*child) >= usize::from(self.node_len)) {
                return Err("GLB node referenced a missing child");
            }
        }
        for scene in self.scenes[..usize::from(self.scene_len)].iter().flatten() {
            let start = usize::from(scene.node_start);
            let end = start.checked_add(usize::from(scene.node_len)).ok_or("GLB scene root range overflowed")?;
            if end > usize::from(self.scene_root_len) || self.scene_roots[start..end].iter().flatten().any(|node| usize::from(*node) >= usize::from(self.node_len)) {
                return Err("GLB scene referenced a missing root node");
            }
        }
        let mut output_bytes = 0usize;
        for primitive in self.primitives[..usize::from(self.primitive_len)].iter().flatten() {
            if !matches!(primitive.mode, 4..=6) {
                continue;
            }
            let position = self.accessor(primitive.position)?;
            if position.component != 5126 || position.kind != 3 {
                return Err("GLB POSITION accessor was not FLOAT VEC3");
            }
            self.validate_accessor_span(position)?;
            let position_bytes = usize::try_from(position.count).ok().and_then(|count| count.checked_mul(3)).and_then(|count| count.checked_mul(std::mem::size_of::<f32>())).ok_or("GLB vertex output bytes overflowed")?;
            output_bytes = output_bytes.checked_add(position_bytes).ok_or("GLB vertex output bytes overflowed")?;
            if let Some(normal) = primitive.normal {
                let normal = self.accessor(normal)?;
                if !matches!(normal.component, 5126 | 5120 | 5122) || (normal.component != 5126 && !normal.normalized) || normal.kind != 3 || normal.count != position.count {
                    return Err("GLB NORMAL accessor did not match POSITION");
                }
                self.validate_accessor_span(normal)?;
                let normal_bytes = usize::try_from(normal.count).ok().and_then(|count| count.checked_mul(3)).and_then(|count| count.checked_mul(std::mem::size_of::<f32>())).ok_or("GLB normal output bytes overflowed")?;
                output_bytes = output_bytes.checked_add(normal_bytes).ok_or("GLB normal output bytes overflowed")?;
            } else {
                output_bytes = output_bytes.checked_add(position_bytes).ok_or("GLB generated normal bytes overflowed")?;
            }
            if let Some(uv) = primitive.uv {
                let uv = self.accessor(uv)?;
                if !matches!(uv.component, 5126 | 5121 | 5123) || (uv.component != 5126 && !uv.normalized) || uv.kind != 2 || uv.count != position.count {
                    return Err("GLB TEXCOORD_0 accessor did not match POSITION");
                }
                self.validate_accessor_span(uv)?;
                let uv_bytes = usize::try_from(uv.count).ok().and_then(|count| count.checked_mul(2)).and_then(|count| count.checked_mul(std::mem::size_of::<f32>())).ok_or("GLB UV output bytes overflowed")?;
                output_bytes = output_bytes.checked_add(uv_bytes).ok_or("GLB UV output bytes overflowed")?;
            }
            let source_indices = match primitive.indices {
                Some(indices) => {
                    let indices = self.accessor(indices)?;
                    if !matches!(indices.component, 5121 | 5123 | 5125) || indices.kind != 1 || indices.normalized {
                        return Err("GLB index accessor used an unsupported component or shape");
                    }
                    self.validate_accessor_span(indices)?;
                    indices.count
                }
                None => position.count,
            };
            let triangle_indices = match primitive.mode {
                4 => source_indices,
                5 | 6 => source_indices.saturating_sub(2).checked_mul(3).ok_or("GLB triangle expansion overflowed")?,
                _ => 0,
            };
            if triangle_indices == 0 || !triangle_indices.is_multiple_of(3) {
                return Err("GLB triangle primitive had no complete triangles");
            }
            let index_bytes = usize::try_from(triangle_indices).ok().and_then(|count| count.checked_mul(std::mem::size_of::<u32>())).ok_or("GLB index output bytes overflowed")?;
            output_bytes = output_bytes.checked_add(index_bytes).ok_or("GLB index output bytes overflowed")?;
            if output_bytes > GLB_SCHEMA_OUTPUT_BYTES {
                return Err("GLB semantic output exceeded fixed byte credits");
            }
        }
        if output_bytes == 0 {
            return Err("GLB schema contained no triangle primitive output");
        }
        self.output_bytes = output_bytes;
        Ok(())
    }

    fn accessor(&self, index: u16) -> Result<&GlbAccessorSchema, &'static str> {
        self.accessors.get(usize::from(index)).and_then(Option::as_ref).ok_or("GLB primitive referenced a missing accessor")
    }

    fn validate_accessor_span(&self, accessor: &GlbAccessorSchema) -> Result<(), &'static str> {
        if !accessor.view_set {
            return Err("GLB sparse or viewless accessor was unsupported");
        }
        let view = self.views.get(usize::from(accessor.view)).and_then(Option::as_ref).ok_or("GLB accessor referenced a missing bufferView")?;
        let components = usize::from(accessor.kind);
        let component_bytes = match accessor.component {
            5120 | 5121 => 1,
            5122 | 5123 => 2,
            5125 | 5126 => 4,
            _ => return Err("GLB accessor component type was unsupported"),
        };
        let item_bytes = components.checked_mul(component_bytes).ok_or("GLB accessor item bytes overflowed")?;
        let stride = if view.byte_stride == 0 { item_bytes } else { usize::from(view.byte_stride) };
        if stride < item_bytes {
            return Err("GLB bufferView stride was smaller than its accessor item");
        }
        let span = usize::try_from(accessor.count.saturating_sub(1)).ok().and_then(|count| count.checked_mul(stride)).and_then(|bytes| bytes.checked_add(item_bytes)).and_then(|bytes| bytes.checked_add(accessor.byte_offset as usize));
        if span.is_none_or(|span| span > view.byte_length as usize) {
            return Err("GLB accessor exceeded its bufferView");
        }
        Ok(())
    }
}

struct GlbSchemaCursor {
    page: RendererAssetPageCursor,
    header: [u8; 20],
    header_len: u8,
    json_remaining: u32,
    lexer: GlbJsonTokenCursor,
    depth: u16,
    section: GlbSchemaSection,
    last_string: Option<GlbJsonAtom>,
    pending_key: Option<GlbJsonAtom>,
    expecting_value: bool,
    current_accessor: Option<GlbAccessorSchema>,
    current_view: Option<GlbViewSchema>,
    current_primitive: Option<GlbPrimitiveSchema>,
    current_mesh: Option<GlbMeshSchema>,
    current_node: Option<GlbNodeSchema>,
    current_scene: Option<GlbSceneSchema>,
    numeric_array: Option<GlbNumericArray>,
    primitives_depth: Option<u16>,
    attributes_depth: Option<u16>,
    output: GlbSchemaOutput,
    terminal: bool,
}

impl GlbSchemaCursor {
    fn new() -> Self {
        Self {
            page: RendererAssetPageCursor::new(),
            header: [0; 20],
            header_len: 0,
            json_remaining: 0,
            lexer: GlbJsonTokenCursor::new(),
            depth: 0,
            section: GlbSchemaSection::None,
            last_string: None,
            pending_key: None,
            expecting_value: false,
            current_accessor: None,
            current_view: None,
            current_primitive: None,
            current_mesh: None,
            current_node: None,
            current_scene: None,
            numeric_array: None,
            primitives_depth: None,
            attributes_depth: None,
            output: GlbSchemaOutput::new(),
            terminal: false,
        }
    }

    fn step(&mut self, owner: &mut RendererAssetFetchOwner) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        if self.header_len < 20 {
            let Some(byte) = self.page.read_byte(owner)? else { return Err("GLB semantic header was truncated") };
            self.header[usize::from(self.header_len)] = byte;
            self.header_len += 1;
            if self.header_len == 20 {
                if &self.header[..4] != b"glTF" || &self.header[16..20] != b"JSON" {
                    return Err("GLB semantic cursor did not find the JSON chunk");
                }
                self.json_remaining = u32::from_le_bytes(self.header[12..16].try_into().expect("fixed GLB JSON length"));
                self.output.json_bytes = self.json_remaining;
                if self.json_remaining == 0 || self.json_remaining as usize > owner.received_bytes().saturating_sub(20) {
                    return Err("GLB JSON chunk exceeded sealed input credits");
                }
            }
            return Ok(false);
        }
        if let Some(token) = self.lexer.replay()? {
            self.apply_token(token)?;
            return Ok(false);
        }
        if self.json_remaining == 0 {
            if let Some(token) = self.lexer.finish()? {
                self.apply_token(token)?;
                return Ok(false);
            }
            if self.depth != 0
                || self.current_accessor.is_some()
                || self.current_view.is_some()
                || self.current_primitive.is_some()
                || self.current_mesh.is_some()
                || self.current_node.is_some()
                || self.current_scene.is_some()
                || self.numeric_array.is_some()
            {
                return Err("GLB JSON schema ended with live nested ownership");
            }
            self.output.validate()?;
            self.terminal = true;
            return Ok(true);
        }
        for _ in 0..RENDERER_ASSET_PARSE_BLOCK_BYTES {
            let Some(byte) = self.page.read_byte(owner)? else { return Err("GLB JSON chunk was truncated") };
            self.json_remaining -= 1;
            if let Some(token) = self.lexer.feed(byte)? {
                self.apply_token(token)?;
                return Ok(false);
            }
            if self.json_remaining == 0 {
                return Ok(false);
            }
        }
        Ok(false)
    }

    fn apply_token(&mut self, token: GlbJsonToken) -> Result<(), &'static str> {
        match token {
            GlbJsonToken::ObjectStart => {
                let next_depth = self.depth.checked_add(1).ok_or("GLB JSON depth overflowed")?;
                if self.section == GlbSchemaSection::Accessors && self.depth == 2 {
                    self.current_accessor = Some(GlbAccessorSchema::default());
                } else if self.section == GlbSchemaSection::Views && self.depth == 2 {
                    self.current_view = Some(GlbViewSchema::default());
                } else if self.section == GlbSchemaSection::Meshes && self.depth == 2 {
                    self.current_mesh = Some(GlbMeshSchema { primitive_start: self.output.primitive_len, primitive_len: 0 });
                } else if self.section == GlbSchemaSection::Meshes && self.primitives_depth == Some(self.depth) {
                    self.current_primitive = Some(GlbPrimitiveSchema::default());
                } else if self.section == GlbSchemaSection::Meshes && self.depth == 5 && self.pending_key.as_ref().is_some_and(|key| key.equals("attributes")) {
                    self.attributes_depth = Some(next_depth);
                } else if self.section == GlbSchemaSection::Nodes && self.depth == 2 {
                    self.current_node = Some(GlbNodeSchema::default());
                } else if self.section == GlbSchemaSection::Scenes && self.depth == 2 {
                    self.current_scene = Some(GlbSceneSchema::default());
                }
                self.depth = next_depth;
                self.consume_value();
            }
            GlbJsonToken::ObjectEnd => {
                if self.depth == 0 {
                    return Err("GLB JSON object depth underflowed");
                }
                if self.section == GlbSchemaSection::Accessors && self.depth == 3 {
                    self.finish_accessor()?;
                } else if self.section == GlbSchemaSection::Views && self.depth == 3 {
                    self.finish_view()?;
                } else if self.section == GlbSchemaSection::Meshes && self.current_primitive.is_some() && self.primitives_depth == Some(self.depth - 1) {
                    self.finish_primitive()?;
                } else if self.section == GlbSchemaSection::Meshes && self.depth == 3 {
                    self.finish_mesh()?;
                } else if self.section == GlbSchemaSection::Nodes && self.depth == 3 {
                    self.finish_node()?;
                } else if self.section == GlbSchemaSection::Scenes && self.depth == 3 {
                    self.finish_scene()?;
                }
                if self.attributes_depth == Some(self.depth) {
                    self.attributes_depth = None;
                }
                self.depth -= 1;
                self.last_string = None;
            }
            GlbJsonToken::ArrayStart => {
                let key = self.pending_key;
                let next_depth = self.depth.checked_add(1).ok_or("GLB JSON depth overflowed")?;
                if self.depth == 1 {
                    self.section = if key.as_ref().is_some_and(|key| key.equals("accessors")) {
                        GlbSchemaSection::Accessors
                    } else if key.as_ref().is_some_and(|key| key.equals("bufferViews")) {
                        GlbSchemaSection::Views
                    } else if key.as_ref().is_some_and(|key| key.equals("meshes")) {
                        GlbSchemaSection::Meshes
                    } else if key.as_ref().is_some_and(|key| key.equals("nodes")) {
                        GlbSchemaSection::Nodes
                    } else if key.as_ref().is_some_and(|key| key.equals("scenes")) {
                        GlbSchemaSection::Scenes
                    } else {
                        GlbSchemaSection::None
                    };
                } else if self.section == GlbSchemaSection::Meshes && self.depth == 3 && key.as_ref().is_some_and(|key| key.equals("primitives")) {
                    self.primitives_depth = Some(next_depth);
                } else if self.section == GlbSchemaSection::Nodes && self.depth == 3 {
                    self.numeric_array = key.as_ref().and_then(|key| {
                        if key.equals("children") {
                            Some(GlbNumericArray::new(GlbNumericArrayKind::NodeChildren, next_depth, self.output.node_edge_len))
                        } else if key.equals("translation") {
                            Some(GlbNumericArray::new(GlbNumericArrayKind::NodeTranslation, next_depth, 0))
                        } else if key.equals("rotation") {
                            Some(GlbNumericArray::new(GlbNumericArrayKind::NodeRotation, next_depth, 0))
                        } else if key.equals("scale") {
                            Some(GlbNumericArray::new(GlbNumericArrayKind::NodeScale, next_depth, 0))
                        } else if key.equals("matrix") {
                            Some(GlbNumericArray::new(GlbNumericArrayKind::NodeMatrix, next_depth, 0))
                        } else {
                            None
                        }
                    });
                } else if self.section == GlbSchemaSection::Scenes && self.depth == 3 && key.as_ref().is_some_and(|key| key.equals("nodes")) {
                    self.numeric_array = Some(GlbNumericArray::new(GlbNumericArrayKind::SceneNodes, next_depth, self.output.scene_root_len));
                }
                self.depth = next_depth;
                self.consume_value();
            }
            GlbJsonToken::ArrayEnd => {
                if self.depth == 0 {
                    return Err("GLB JSON array depth underflowed");
                }
                if self.primitives_depth == Some(self.depth) {
                    self.primitives_depth = None;
                }
                if self.numeric_array.as_ref().is_some_and(|array| array.depth == self.depth) {
                    self.finish_numeric_array()?;
                }
                if self.depth == 2 {
                    self.section = GlbSchemaSection::None;
                }
                self.depth -= 1;
                self.last_string = None;
            }
            GlbJsonToken::Colon => {
                self.pending_key = self.last_string.take();
                self.expecting_value = true;
            }
            GlbJsonToken::Comma => {
                self.last_string = None;
            }
            GlbJsonToken::String(atom) => {
                if self.expecting_value {
                    self.apply_string(atom)?;
                    self.consume_value();
                } else {
                    self.last_string = Some(atom);
                }
            }
            GlbJsonToken::Number(atom) => {
                if self.numeric_array.is_some() {
                    self.push_numeric_array(atom.float()?)?;
                } else if self.expecting_value {
                    self.apply_unsigned(atom)?;
                }
                self.consume_value();
            }
            GlbJsonToken::Literal(value) => {
                if self.expecting_value {
                    self.apply_literal(value)?;
                }
                self.consume_value();
            }
        }
        Ok(())
    }

    fn apply_unsigned(&mut self, value: GlbJsonAtom) -> Result<(), &'static str> {
        let Some(key) = self.pending_key.as_ref() else { return Ok(()) };
        if self.section == GlbSchemaSection::Accessors && self.depth == 3 {
            let value = value.unsigned()?;
            let accessor = self.current_accessor.as_mut().ok_or("GLB accessor scalar arrived without an owner")?;
            if key.equals("bufferView") {
                accessor.view = u16::try_from(value).map_err(|_| "GLB accessor bufferView exceeded fixed index credits")?;
                accessor.view_set = true;
            } else if key.equals("byteOffset") {
                accessor.byte_offset = u32::try_from(value).map_err(|_| "GLB accessor byteOffset overflowed")?;
            } else if key.equals("componentType") {
                accessor.component = u16::try_from(value).map_err(|_| "GLB accessor componentType overflowed")?;
            } else if key.equals("count") {
                accessor.count = u32::try_from(value).map_err(|_| "GLB accessor count overflowed")?;
            }
        } else if self.section == GlbSchemaSection::Views && self.depth == 3 {
            let value = value.unsigned()?;
            let view = self.current_view.as_mut().ok_or("GLB bufferView scalar arrived without an owner")?;
            if key.equals("buffer") && value != 0 {
                return Err("GLB external buffer index was unsupported");
            } else if key.equals("byteOffset") {
                view.byte_offset = u32::try_from(value).map_err(|_| "GLB bufferView byteOffset overflowed")?;
            } else if key.equals("byteLength") {
                view.byte_length = u32::try_from(value).map_err(|_| "GLB bufferView byteLength overflowed")?;
            } else if key.equals("byteStride") {
                view.byte_stride = u16::try_from(value).map_err(|_| "GLB bufferView byteStride exceeded fixed credits")?;
            }
        } else if self.section == GlbSchemaSection::Meshes && self.current_primitive.is_some() && (self.attributes_depth == Some(self.depth) || self.depth == 5) {
            let value = value.unsigned()?;
            let primitive = self.current_primitive.as_mut().expect("checked GLB primitive owner");
            if self.attributes_depth == Some(self.depth) {
                let index = u16::try_from(value).map_err(|_| "GLB primitive attribute index exceeded fixed credits")?;
                if key.equals("POSITION") {
                    primitive.position = index;
                    primitive.position_set = true;
                } else if key.equals("NORMAL") {
                    primitive.normal = Some(index);
                } else if key.equals("TEXCOORD_0") {
                    primitive.uv = Some(index);
                }
            } else if self.depth == 5 {
                if key.equals("indices") {
                    primitive.indices = Some(u16::try_from(value).map_err(|_| "GLB primitive index accessor exceeded fixed credits")?);
                } else if key.equals("mode") {
                    primitive.mode = u8::try_from(value).map_err(|_| "GLB primitive mode overflowed")?;
                }
            }
        } else if self.section == GlbSchemaSection::Nodes && self.depth == 3 && key.equals("mesh") {
            self.current_node.as_mut().ok_or("GLB node mesh arrived without an owner")?.mesh = Some(u16::try_from(value.unsigned()?).map_err(|_| "GLB node mesh index exceeded fixed credits")?);
        } else if self.section == GlbSchemaSection::None && self.depth == 1 && key.equals("scene") {
            self.output.default_scene = Some(u16::try_from(value.unsigned()?).map_err(|_| "GLB default scene index exceeded fixed credits")?);
        }
        Ok(())
    }

    fn apply_literal(&mut self, value: Option<bool>) -> Result<(), &'static str> {
        let Some(key) = self.pending_key.as_ref() else { return Ok(()) };
        if self.section == GlbSchemaSection::Accessors && self.depth == 3 && key.equals("normalized") {
            self.current_accessor.as_mut().ok_or("GLB accessor normalized flag arrived without an owner")?.normalized = value.ok_or("GLB accessor normalized flag was null")?;
        }
        Ok(())
    }

    fn apply_string(&mut self, value: GlbJsonAtom) -> Result<(), &'static str> {
        let Some(key) = self.pending_key.as_ref() else { return Ok(()) };
        if self.section == GlbSchemaSection::Accessors && self.depth == 3 && key.equals("type") {
            self.current_accessor.as_mut().ok_or("GLB accessor type arrived without an owner")?.kind = if value.equals("SCALAR") {
                1
            } else if value.equals("VEC2") {
                2
            } else if value.equals("VEC3") {
                3
            } else if value.equals("VEC4") {
                4
            } else {
                return Err("GLB accessor type was unsupported");
            };
        }
        Ok(())
    }

    fn consume_value(&mut self) {
        self.pending_key = None;
        self.expecting_value = false;
    }

    fn finish_accessor(&mut self) -> Result<(), &'static str> {
        let accessor = self.current_accessor.take().ok_or("GLB accessor owner was missing")?;
        if accessor.component == 0 || accessor.count == 0 || accessor.kind == 0 {
            return Err("GLB accessor omitted required fixed fields");
        }
        let slot = usize::from(self.output.accessor_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB accessor count exceeded fixed item credits");
        }
        self.output.accessors[slot] = Some(accessor);
        self.output.accessor_len += 1;
        Ok(())
    }

    fn finish_view(&mut self) -> Result<(), &'static str> {
        let view = self.current_view.take().ok_or("GLB bufferView owner was missing")?;
        if view.byte_length == 0 {
            return Err("GLB bufferView omitted its byte length");
        }
        let slot = usize::from(self.output.view_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB bufferView count exceeded fixed item credits");
        }
        self.output.views[slot] = Some(view);
        self.output.view_len += 1;
        Ok(())
    }

    fn finish_primitive(&mut self) -> Result<(), &'static str> {
        let primitive = self.current_primitive.take().ok_or("GLB primitive owner was missing")?;
        if !primitive.position_set {
            return Err("GLB primitive omitted POSITION");
        }
        let slot = usize::from(self.output.primitive_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB primitive count exceeded fixed item credits");
        }
        self.output.primitives[slot] = Some(primitive);
        self.output.primitive_len += 1;
        let mesh = self.current_mesh.as_mut().ok_or("GLB primitive completed without a mesh owner")?;
        mesh.primitive_len = mesh.primitive_len.checked_add(1).ok_or("GLB mesh primitive count overflowed")?;
        Ok(())
    }

    fn finish_mesh(&mut self) -> Result<(), &'static str> {
        let mesh = self.current_mesh.take().ok_or("GLB mesh owner was missing")?;
        if mesh.primitive_len == 0 {
            return Err("GLB mesh contained no primitive");
        }
        let slot = usize::from(self.output.mesh_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB mesh count exceeded fixed item credits");
        }
        self.output.meshes[slot] = Some(mesh);
        self.output.mesh_len += 1;
        Ok(())
    }

    fn finish_node(&mut self) -> Result<(), &'static str> {
        let node = self.current_node.take().ok_or("GLB node owner was missing")?;
        let slot = usize::from(self.output.node_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB node count exceeded fixed item credits");
        }
        self.output.nodes[slot] = Some(node);
        self.output.node_len += 1;
        Ok(())
    }

    fn finish_scene(&mut self) -> Result<(), &'static str> {
        let scene = self.current_scene.take().ok_or("GLB scene owner was missing")?;
        let slot = usize::from(self.output.scene_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB scene count exceeded fixed item credits");
        }
        self.output.scenes[slot] = Some(scene);
        self.output.scene_len += 1;
        Ok(())
    }

    fn finish_numeric_array(&mut self) -> Result<(), &'static str> {
        let array = self.numeric_array.take().ok_or("GLB numeric array owner was missing")?;
        match array.kind {
            GlbNumericArrayKind::NodeChildren => {
                let node = self.current_node.as_mut().ok_or("GLB children completed without a node owner")?;
                node.child_start = array.start;
                node.child_len = array.len;
            }
            GlbNumericArrayKind::NodeTranslation => {
                if array.len != 3 {
                    return Err("GLB node translation did not contain three values");
                }
                self.current_node.as_mut().ok_or("GLB translation completed without a node owner")?.translation.copy_from_slice(&array.values[..3]);
            }
            GlbNumericArrayKind::NodeRotation => {
                if array.len != 4 {
                    return Err("GLB node rotation did not contain four values");
                }
                self.current_node.as_mut().ok_or("GLB rotation completed without a node owner")?.rotation.copy_from_slice(&array.values[..4]);
            }
            GlbNumericArrayKind::NodeScale => {
                if array.len != 3 {
                    return Err("GLB node scale did not contain three values");
                }
                self.current_node.as_mut().ok_or("GLB scale completed without a node owner")?.scale.copy_from_slice(&array.values[..3]);
            }
            GlbNumericArrayKind::NodeMatrix => {
                if array.len != 16 {
                    return Err("GLB node matrix did not contain sixteen values");
                }
                self.current_node.as_mut().ok_or("GLB matrix completed without a node owner")?.matrix = Some(array.values);
            }
            GlbNumericArrayKind::SceneNodes => {
                let scene = self.current_scene.as_mut().ok_or("GLB roots completed without a scene owner")?;
                scene.node_start = array.start;
                scene.node_len = array.len;
            }
        }
        Ok(())
    }

    fn push_numeric_array(&mut self, value: f32) -> Result<(), &'static str> {
        let array = self.numeric_array.as_mut().ok_or("GLB numeric array owner was missing")?;
        match array.kind {
            GlbNumericArrayKind::NodeChildren => {
                let slot = usize::from(self.output.node_edge_len);
                if slot == GLB_SCHEMA_ITEM_CAPACITY {
                    return Err("GLB child edges exceeded fixed item credits");
                }
                self.output.node_edges[slot] = Some(exact_u16(value, "GLB child index was not an admitted integer")?);
                self.output.node_edge_len += 1;
                array.len = array.len.checked_add(1).ok_or("GLB child count overflowed")?;
            }
            GlbNumericArrayKind::SceneNodes => {
                let slot = usize::from(self.output.scene_root_len);
                if slot == GLB_SCHEMA_ITEM_CAPACITY {
                    return Err("GLB scene roots exceeded fixed item credits");
                }
                self.output.scene_roots[slot] = Some(exact_u16(value, "GLB scene node index was not an admitted integer")?);
                self.output.scene_root_len += 1;
                array.len = array.len.checked_add(1).ok_or("GLB scene root count overflowed")?;
            }
            _ => array.push(value)?,
        }
        Ok(())
    }
}

fn exact_u16(value: f32, detail: &'static str) -> Result<u16, &'static str> {
    if value < 0.0 || value > u16::MAX as f32 || value.fract() != 0.0 {
        return Err(detail);
    }
    Ok(value as u16)
}

type GlbMatrix = [[f32; 4]; 4];

#[derive(Clone, Copy)]
struct GlbInstanceOutput {
    primitive: u16,
    matrix: GlbMatrix,
    vertex_base: u32,
    index_base: u32,
    vertex_count: u32,
    index_count: u32,
    explicit_normals: bool,
}

#[derive(Clone, Copy)]
struct GlbNodePlanFrame {
    node: u16,
    parent: GlbMatrix,
    phase: u8,
    item: u16,
}

enum GlbPlanMode {
    Roots { scene: Option<u16>, index: u16 },
    Nodes { scene: Option<u16>, next_root: u16 },
    Fallback { primitive: u16 },
    Terminal,
}

struct GlbInstancePlanCursor {
    instances: Box<[Option<GlbInstanceOutput>; GLB_SCHEMA_ITEM_CAPACITY]>,
    instance_len: u16,
    stack: Box<[Option<GlbNodePlanFrame>; GLB_SCHEMA_ITEM_CAPACITY]>,
    stack_len: u16,
    mode: GlbPlanMode,
    vertex_count: u32,
    index_count: u32,
    has_uvs: bool,
    output_bytes: usize,
}

impl GlbInstancePlanCursor {
    fn new(schema: &GlbSchemaOutput) -> Self {
        let scene = schema.default_scene.or((schema.scene_len != 0).then_some(0));
        let mode = if schema.node_len == 0 { GlbPlanMode::Fallback { primitive: 0 } } else { GlbPlanMode::Roots { scene, index: 0 } };
        Self { instances: Box::new([None; GLB_SCHEMA_ITEM_CAPACITY]), instance_len: 0, stack: Box::new([None; GLB_SCHEMA_ITEM_CAPACITY]), stack_len: 0, mode, vertex_count: 0, index_count: 0, has_uvs: false, output_bytes: 0 }
    }

    fn step(&mut self, schema: &GlbSchemaOutput) -> Result<bool, &'static str> {
        match self.mode {
            GlbPlanMode::Roots { scene, index } => {
                let root = if let Some(scene) = scene {
                    let scene = schema.scenes.get(usize::from(scene)).and_then(Option::as_ref).ok_or("GLB plan lost its scene")?;
                    if index >= scene.node_len {
                        self.mode = GlbPlanMode::Terminal;
                        return Ok(false);
                    }
                    schema.scene_roots.get(usize::from(scene.node_start) + usize::from(index)).and_then(Option::as_ref).copied().ok_or("GLB plan lost a scene root")?
                } else {
                    if index >= schema.node_len {
                        self.mode = GlbPlanMode::Terminal;
                        return Ok(false);
                    }
                    index
                };
                self.push_node(root, glb_identity())?;
                self.mode = GlbPlanMode::Nodes { scene, next_root: index + 1 };
                Ok(false)
            }
            GlbPlanMode::Nodes { scene, next_root } => {
                if self.stack_len == 0 {
                    self.mode = GlbPlanMode::Roots { scene, index: next_root };
                    return Ok(false);
                }
                let slot = usize::from(self.stack_len - 1);
                let mut frame = self.stack[slot].take().ok_or("GLB node traversal lost its retained frame")?;
                let node = *schema.nodes.get(usize::from(frame.node)).and_then(Option::as_ref).ok_or("GLB node traversal referenced a missing node")?;
                let local = node.matrix.map(glb_matrix_from_array).unwrap_or_else(|| glb_trs(node.translation, node.rotation, node.scale));
                let world = glb_matrix_mul(frame.parent, local);
                if frame.phase == 0 {
                    frame.phase = 1;
                    frame.item = 0;
                }
                if frame.phase == 1 {
                    if let Some(mesh) = node.mesh {
                        let mesh = schema.meshes.get(usize::from(mesh)).and_then(Option::as_ref).ok_or("GLB node traversal lost a mesh")?;
                        if frame.item < mesh.primitive_len {
                            let primitive = mesh.primitive_start.checked_add(frame.item).ok_or("GLB primitive range overflowed")?;
                            frame.item += 1;
                            self.stack[slot] = Some(frame);
                            self.admit_instance(schema, primitive, world)?;
                            return Ok(false);
                        }
                    }
                    frame.phase = 2;
                    frame.item = 0;
                }
                if frame.item < node.child_len {
                    let child = schema.node_edges.get(usize::from(node.child_start) + usize::from(frame.item)).and_then(Option::as_ref).copied().ok_or("GLB node traversal lost a child edge")?;
                    frame.item += 1;
                    self.stack[slot] = Some(frame);
                    self.push_node(child, world)?;
                    return Ok(false);
                }
                self.stack_len -= 1;
                Ok(false)
            }
            GlbPlanMode::Fallback { primitive } => {
                if primitive >= schema.primitive_len {
                    self.mode = GlbPlanMode::Terminal;
                    return Ok(false);
                }
                self.admit_instance(schema, primitive, glb_identity())?;
                self.mode = GlbPlanMode::Fallback { primitive: primitive + 1 };
                Ok(false)
            }
            GlbPlanMode::Terminal => Ok(true),
        }
    }

    fn push_node(&mut self, node: u16, parent: GlbMatrix) -> Result<(), &'static str> {
        let slot = usize::from(self.stack_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB node traversal exceeded fixed depth credits");
        }
        if self.stack[..slot].iter().flatten().any(|frame| frame.node == node) {
            return Err("GLB node traversal contained a cycle");
        }
        self.stack[slot] = Some(GlbNodePlanFrame { node, parent, phase: 0, item: 0 });
        self.stack_len += 1;
        Ok(())
    }

    fn admit_instance(&mut self, schema: &GlbSchemaOutput, primitive: u16, matrix: GlbMatrix) -> Result<(), &'static str> {
        let primitive_schema = schema.primitives.get(usize::from(primitive)).and_then(Option::as_ref).ok_or("GLB plan referenced a missing primitive")?;
        if !matches!(primitive_schema.mode, 4..=6) {
            return Ok(());
        }
        let position = schema.accessor(primitive_schema.position)?;
        let source_indices = primitive_schema.indices.map(|index| schema.accessor(index).map(|accessor| accessor.count)).transpose()?.unwrap_or(position.count);
        let index_count = match primitive_schema.mode {
            4 => source_indices,
            5 | 6 => source_indices.saturating_sub(2).checked_mul(3).ok_or("GLB triangle expansion overflowed")?,
            _ => 0,
        };
        if index_count == 0 || !index_count.is_multiple_of(3) {
            return Err("GLB instance had no complete triangle output");
        }
        let vertex_base = self.vertex_count;
        let index_base = self.index_count;
        let vertex_count = vertex_base.checked_add(position.count).ok_or("GLB instantiated vertex count overflowed")?;
        let output_index_count = index_base.checked_add(index_count).ok_or("GLB instantiated index count overflowed")?;
        let has_uvs = self.has_uvs || primitive_schema.uv.is_some();
        let vertex_bytes = usize::try_from(vertex_count).ok().and_then(|count| count.checked_mul(if has_uvs { 32 } else { 24 })).ok_or("GLB instantiated vertex bytes overflowed")?;
        let index_bytes = usize::try_from(output_index_count).ok().and_then(|count| count.checked_mul(4)).ok_or("GLB instantiated index bytes overflowed")?;
        let output_bytes = vertex_bytes.checked_add(index_bytes).ok_or("GLB instantiated output bytes overflowed")?;
        if output_bytes > GLB_SCHEMA_OUTPUT_BYTES {
            return Err("GLB instantiated output exceeded fixed byte credits");
        }
        let slot = usize::from(self.instance_len);
        if slot == GLB_SCHEMA_ITEM_CAPACITY {
            return Err("GLB instantiated primitive count exceeded fixed item credits");
        }
        self.instances[slot] = Some(GlbInstanceOutput { primitive, matrix, vertex_base, index_base, vertex_count: position.count, index_count, explicit_normals: primitive_schema.normal.is_some() });
        self.instance_len += 1;
        self.vertex_count = vertex_count;
        self.index_count = output_index_count;
        self.has_uvs = has_uvs;
        self.output_bytes = output_bytes;
        Ok(())
    }
}

fn glb_identity() -> GlbMatrix {
    [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]
}

fn glb_matrix_from_array(values: [f32; 16]) -> GlbMatrix {
    [values[0..4].try_into().expect("fixed GLB matrix column"), values[4..8].try_into().expect("fixed GLB matrix column"), values[8..12].try_into().expect("fixed GLB matrix column"), values[12..16].try_into().expect("fixed GLB matrix column")]
}

fn glb_matrix_mul(left: GlbMatrix, right: GlbMatrix) -> GlbMatrix {
    let mut result = [[0.0; 4]; 4];
    for column in 0..4 {
        for row in 0..4 {
            for axis in 0..4 {
                result[column][row] += left[axis][row] * right[column][axis];
            }
        }
    }
    result
}

fn glb_trs(translation: [f32; 3], rotation: [f32; 4], scale: [f32; 3]) -> GlbMatrix {
    let [x, y, z, w] = rotation;
    let x2 = x + x;
    let y2 = y + y;
    let z2 = z + z;
    let xx = x * x2;
    let xy = x * y2;
    let xz = x * z2;
    let yy = y * y2;
    let yz = y * z2;
    let zz = z * z2;
    let wx = w * x2;
    let wy = w * y2;
    let wz = w * z2;
    [
        [(1.0 - (yy + zz)) * scale[0], (xy + wz) * scale[0], (xz - wy) * scale[0], 0.0],
        [(xy - wz) * scale[1], (1.0 - (xx + zz)) * scale[1], (yz + wx) * scale[1], 0.0],
        [(xz + wy) * scale[2], (yz - wx) * scale[2], (1.0 - (xx + yy)) * scale[2], 0.0],
        [translation[0], translation[1], translation[2], 1.0],
    ]
}

fn glb_transform_point(matrix: GlbMatrix, point: [f32; 3]) -> [f32; 3] {
    [
        matrix[0][0] * point[0] + matrix[1][0] * point[1] + matrix[2][0] * point[2] + matrix[3][0],
        matrix[0][1] * point[0] + matrix[1][1] * point[1] + matrix[2][1] * point[2] + matrix[3][1],
        matrix[0][2] * point[0] + matrix[1][2] * point[1] + matrix[2][2] * point[2] + matrix[3][2],
    ]
}

fn glb_transform_normal(matrix: GlbMatrix, normal: [f32; 3]) -> [f32; 3] {
    let (a00, a01, a02) = (matrix[0][0], matrix[1][0], matrix[2][0]);
    let (a10, a11, a12) = (matrix[0][1], matrix[1][1], matrix[2][1]);
    let (a20, a21, a22) = (matrix[0][2], matrix[1][2], matrix[2][2]);
    let det = a00 * (a11 * a22 - a12 * a21) - a01 * (a10 * a22 - a12 * a20) + a02 * (a10 * a21 - a11 * a20);
    if det.abs() <= f32::EPSILON {
        return normal;
    }
    let inverse_det = det.recip();
    let transformed = [
        ((a11 * a22 - a12 * a21) * normal[0] + (a12 * a20 - a10 * a22) * normal[1] + (a10 * a21 - a11 * a20) * normal[2]) * inverse_det,
        ((a02 * a21 - a01 * a22) * normal[0] + (a00 * a22 - a02 * a20) * normal[1] + (a01 * a20 - a00 * a21) * normal[2]) * inverse_det,
        ((a01 * a12 - a02 * a11) * normal[0] + (a02 * a10 - a00 * a12) * normal[1] + (a00 * a11 - a01 * a10) * normal[2]) * inverse_det,
    ];
    glb_normalize(transformed)
}

fn glb_normalize(value: [f32; 3]) -> [f32; 3] {
    let length = value.iter().map(|value| value * value).sum::<f32>().sqrt();
    if length <= f32::EPSILON {
        value
    } else {
        value.map(|value| value / length)
    }
}

#[derive(Clone, Copy)]
enum GlbMaterializePhase {
    Plan,
    Header,
    Allocate,
    Positions,
    Normals,
    Uvs,
    Indices,
    GenerateNormals,
    NormalizeNormals,
    Seal,
    Ready,
    Closing,
}

struct GlbMaterializeCursor {
    schema: GlbSchemaOutput,
    plan: GlbInstancePlanCursor,
    write: Option<Mesh3dWriteToken>,
    lease: Option<Mesh3dLease>,
    phase: GlbMaterializePhase,
    bin_start: usize,
    bin_bytes: usize,
    instance: u16,
    item: u32,
    normal_substep: u8,
    normal_indices: [u32; 3],
    normal_positions: [[f32; 3]; 3],
    normal_face: [f32; 3],
}

impl GlbMaterializeCursor {
    fn new(schema: GlbSchemaOutput) -> Self {
        let plan = GlbInstancePlanCursor::new(&schema);
        Self { schema, plan, write: None, lease: None, phase: GlbMaterializePhase::Plan, bin_start: 0, bin_bytes: 0, instance: 0, item: 0, normal_substep: 0, normal_indices: [0; 3], normal_positions: [[0.0; 3]; 3], normal_face: [0.0; 3] }
    }

    fn step(&mut self, owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex) -> Result<bool, &'static str> {
        match self.phase {
            GlbMaterializePhase::Plan => {
                if self.plan.step(&self.schema)? {
                    if self.plan.output_bytes == 0 {
                        return Err("GLB plan produced no reachable triangle output");
                    }
                    let schema = Mesh3dSchema { vertices: self.plan.vertex_count, indices: self.plan.index_count, face_ids: 0, vertex_ids: 0, edges: 0, edge_ids: 0, uvs: if self.plan.has_uvs { self.plan.vertex_count } else { 0 }, colors: 0 };
                    self.write = Some(mesh3d_begin(owner.generation(), owner.revision(), schema).map_err(glb_mesh_fault)?);
                    self.phase = GlbMaterializePhase::Header;
                }
                Ok(false)
            }
            GlbMaterializePhase::Header => {
                let header_at = 20usize.checked_add(self.schema.json_bytes as usize).ok_or("GLB BIN header offset overflowed")?;
                let header = pages.read::<8>(owner, header_at)?;
                if &header[4..] != b"BIN\0" {
                    return Err("GLB semantic materializer did not find BIN");
                }
                self.bin_bytes = u32::from_le_bytes(header[..4].try_into().expect("fixed GLB BIN length")) as usize;
                self.bin_start = header_at.checked_add(8).ok_or("GLB BIN payload offset overflowed")?;
                if self.bin_bytes == 0 || self.bin_start.checked_add(self.bin_bytes).is_none_or(|end| end > pages.total) {
                    return Err("GLB BIN payload exceeded sealed page credits");
                }
                self.validate_bin_spans()?;
                self.phase = GlbMaterializePhase::Allocate;
                Ok(false)
            }
            GlbMaterializePhase::Allocate => {
                if mesh3d_allocate_step(self.write_token()?).map_err(glb_mesh_fault)? {
                    self.phase = GlbMaterializePhase::Positions;
                }
                Ok(false)
            }
            GlbMaterializePhase::Positions => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::Normals);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let value = self.read_vec(owner, pages, primitive.position, self.item, 3)?;
                let value = glb_transform_point(instance.matrix, [value[0], value[1], value[2]]);
                mesh3d_write_vec3(self.write_token()?, Mesh3dField::Positions, value).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::Normals => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::Uvs);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let value = if instance.explicit_normals {
                    let primitive = self.primitive(instance)?;
                    let accessor = primitive.normal.ok_or("GLB explicit normal stage lost its accessor")?;
                    let value = self.read_vec(owner, pages, accessor, self.item, 3)?;
                    glb_transform_normal(instance.matrix, [value[0], value[1], value[2]])
                } else {
                    [0.0; 3]
                };
                mesh3d_write_vec3(self.write_token()?, Mesh3dField::Normals, value).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::Uvs => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::Indices);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if !self.plan.has_uvs {
                    self.advance_phase(GlbMaterializePhase::Indices);
                    return Ok(false);
                }
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let value = if let Some(accessor) = primitive.uv {
                    let value = self.read_vec(owner, pages, accessor, self.item, 2)?;
                    [value[0], value[1]]
                } else {
                    [0.0; 2]
                };
                mesh3d_write_vec2(self.write_token()?, Mesh3dField::Uvs, value).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::Indices => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::GenerateNormals);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if self.item == instance.index_count {
                    self.next_instance();
                    return Ok(false);
                }
                let primitive = self.primitive(instance)?;
                let index = self.read_output_index(owner, pages, primitive, self.item, instance.vertex_count)?;
                let index = instance.vertex_base.checked_add(index).ok_or("GLB instantiated index overflowed")?;
                mesh3d_write_u32(self.write_token()?, Mesh3dField::Indices, index).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::GenerateNormals => {
                if self.instance == self.plan.instance_len {
                    self.advance_phase(GlbMaterializePhase::NormalizeNormals);
                    return Ok(false);
                }
                let instance = self.instance()?;
                if instance.explicit_normals {
                    self.next_instance();
                    return Ok(false);
                }
                let triangle_count = instance.index_count / 3;
                if self.item == triangle_count {
                    self.next_instance();
                    return Ok(false);
                }
                let token = self.write_token()?;
                match self.normal_substep {
                    0..=2 => {
                        let component = usize::from(self.normal_substep);
                        let index = instance.index_base.checked_add(self.item * 3).and_then(|index| index.checked_add(component as u32)).ok_or("GLB generated-normal index address overflowed")?;
                        self.normal_indices[component] = mesh3d_read_write_u32(token, Mesh3dField::Indices, index).map_err(glb_mesh_fault)?;
                        self.normal_substep += 1;
                    }
                    3..=5 => {
                        let component = usize::from(self.normal_substep - 3);
                        self.normal_positions[component] = mesh3d_read_write_vec3(token, Mesh3dField::Positions, self.normal_indices[component]).map_err(glb_mesh_fault)?;
                        self.normal_substep += 1;
                    }
                    6 => {
                        let a = [self.normal_positions[1][0] - self.normal_positions[0][0], self.normal_positions[1][1] - self.normal_positions[0][1], self.normal_positions[1][2] - self.normal_positions[0][2]];
                        let b = [self.normal_positions[2][0] - self.normal_positions[0][0], self.normal_positions[2][1] - self.normal_positions[0][1], self.normal_positions[2][2] - self.normal_positions[0][2]];
                        self.normal_face = [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
                        self.normal_substep += 1;
                    }
                    7..=9 => {
                        let component = usize::from(self.normal_substep - 7);
                        let index = self.normal_indices[component];
                        let prior = mesh3d_read_write_vec3(token, Mesh3dField::Normals, index).map_err(glb_mesh_fault)?;
                        mesh3d_update_vec3(token, Mesh3dField::Normals, index, [prior[0] + self.normal_face[0], prior[1] + self.normal_face[1], prior[2] + self.normal_face[2]]).map_err(glb_mesh_fault)?;
                        if self.normal_substep == 9 {
                            self.normal_substep = 0;
                            self.item += 1;
                        } else {
                            self.normal_substep += 1;
                        }
                    }
                    _ => return Err("GLB generated-normal cursor exceeded its fixed phase"),
                }
                Ok(false)
            }
            GlbMaterializePhase::NormalizeNormals => {
                if self.instance == self.plan.instance_len {
                    self.phase = GlbMaterializePhase::Seal;
                    return Ok(false);
                }
                let instance = self.instance()?;
                if instance.explicit_normals {
                    self.next_instance();
                    return Ok(false);
                }
                if self.item == instance.vertex_count {
                    self.next_instance();
                    return Ok(false);
                }
                let index = instance.vertex_base.checked_add(self.item).ok_or("GLB normal output index overflowed")?;
                let token = self.write_token()?;
                let normal = glb_normalize(mesh3d_read_write_vec3(token, Mesh3dField::Normals, index).map_err(glb_mesh_fault)?);
                mesh3d_update_vec3(token, Mesh3dField::Normals, index, normal).map_err(glb_mesh_fault)?;
                self.item += 1;
                Ok(false)
            }
            GlbMaterializePhase::Seal => {
                let token = self.write.take().ok_or("GLB materializer lost its mesh write claim")?;
                match mesh3d_seal(token) {
                    Ok(lease) => {
                        self.lease = Some(lease);
                        self.phase = GlbMaterializePhase::Ready;
                        Ok(true)
                    }
                    Err(fault) => {
                        self.write = Some(token);
                        Err(glb_mesh_fault(fault))
                    }
                }
            }
            GlbMaterializePhase::Ready => Ok(true),
            GlbMaterializePhase::Closing => Ok(false),
        }
    }

    fn validate_bin_spans(&self) -> Result<(), &'static str> {
        for accessor in self.schema.accessors[..usize::from(self.schema.accessor_len)].iter().flatten() {
            let view = self.schema.views.get(usize::from(accessor.view)).and_then(Option::as_ref).ok_or("GLB materializer lost an accessor view")?;
            let end = (view.byte_offset as usize).checked_add(view.byte_length as usize).ok_or("GLB bufferView BIN span overflowed")?;
            if end > self.bin_bytes {
                return Err("GLB bufferView exceeded the BIN payload");
            }
        }
        Ok(())
    }

    fn read_vec(&self, owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex, accessor: u16, item: u32, components: usize) -> Result<[f32; 4], &'static str> {
        let schema = self.schema.accessor(accessor)?;
        if usize::from(schema.kind) != components || item >= schema.count {
            return Err("GLB accessor semantic read exceeded its shape");
        }
        let mut result = [0.0; 4];
        for (component, output) in result[..components].iter_mut().enumerate() {
            *output = self.read_component(owner, pages, schema, item, component)?;
        }
        Ok(result)
    }

    fn read_component(&self, owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex, accessor: &GlbAccessorSchema, item: u32, component: usize) -> Result<f32, &'static str> {
        let view = self.schema.views.get(usize::from(accessor.view)).and_then(Option::as_ref).ok_or("GLB accessor semantic read lost its bufferView")?;
        let component_bytes = glb_component_bytes(accessor.component)?;
        let item_bytes = usize::from(accessor.kind).checked_mul(component_bytes).ok_or("GLB accessor semantic item bytes overflowed")?;
        let stride = if view.byte_stride == 0 { item_bytes } else { usize::from(view.byte_stride) };
        let relative = (view.byte_offset as usize)
            .checked_add(accessor.byte_offset as usize)
            .and_then(|offset| usize::try_from(item).ok().and_then(|item| item.checked_mul(stride)).and_then(|item| offset.checked_add(item)))
            .and_then(|offset| offset.checked_add(component * component_bytes))
            .ok_or("GLB accessor semantic address overflowed")?;
        let absolute = self.bin_start.checked_add(relative).ok_or("GLB accessor absolute address overflowed")?;
        glb_read_component(owner, pages, absolute, accessor.component, accessor.normalized)
    }

    fn read_output_index(&self, owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex, primitive: &GlbPrimitiveSchema, output: u32, vertex_count: u32) -> Result<u32, &'static str> {
        let triangle = output / 3;
        let component = output % 3;
        let source = match (primitive.mode, component) {
            (4, component) => triangle * 3 + component,
            (5, component) if triangle.is_multiple_of(2) => triangle + component,
            (5, 0) => triangle + 1,
            (5, 1) => triangle,
            (5, 2) => triangle + 2,
            (6, 0) => 0,
            (6, 1) => triangle + 1,
            (6, 2) => triangle + 2,
            _ => return Err("GLB materializer reached an unsupported primitive mode"),
        };
        let result = if let Some(accessor) = primitive.indices {
            let accessor = self.schema.accessor(accessor)?;
            let value = self.read_component(owner, pages, accessor, source, 0)?;
            if value < 0.0 || value > u32::MAX as f32 || value.fract() != 0.0 {
                return Err("GLB index value was not an admitted integer");
            }
            value as u32
        } else {
            source
        };
        (result < vertex_count).then_some(result).ok_or("GLB index exceeded POSITION count")
    }

    fn instance(&self) -> Result<GlbInstanceOutput, &'static str> {
        self.plan.instances.get(usize::from(self.instance)).and_then(Option::as_ref).copied().ok_or("GLB materializer lost an instance plan")
    }

    fn primitive(&self, instance: GlbInstanceOutput) -> Result<&GlbPrimitiveSchema, &'static str> {
        self.schema.primitives.get(usize::from(instance.primitive)).and_then(Option::as_ref).ok_or("GLB materializer lost a primitive schema")
    }

    fn write_token(&self) -> Result<Mesh3dWriteToken, &'static str> {
        self.write.ok_or("GLB materializer lost its mesh write claim")
    }

    fn take_mesh_lease(&mut self) -> Option<Mesh3dLease> {
        self.lease.take()
    }

    fn restore_mesh_lease(&mut self, lease: Mesh3dLease) {
        assert!(self.lease.replace(lease).is_none(), "GLB publication restores exactly one retained lease");
    }

    fn next_instance(&mut self) {
        self.instance += 1;
        self.item = 0;
        self.normal_substep = 0;
    }

    fn advance_phase(&mut self, phase: GlbMaterializePhase) {
        self.phase = phase;
        self.instance = 0;
        self.item = 0;
        self.normal_substep = 0;
    }

    fn begin_close(&mut self) {
        self.phase = GlbMaterializePhase::Closing;
    }

    fn close_step(&mut self) -> bool {
        self.begin_close();
        if let Some(token) = self.write {
            match mesh3d_abort(token) {
                Ok(()) | Err(Mesh3dFault::Closing) => {}
                Err(Mesh3dFault::Stale) => {
                    self.write = None;
                    return self.lease.is_none();
                }
                Err(_) => return false,
            }
            match mesh3d_abort_step(token) {
                Ok(true) | Err(Mesh3dFault::Stale) => self.write = None,
                Ok(false) | Err(_) => return false,
            }
            return self.lease.is_none();
        }
        if let Some(lease) = self.lease {
            match mesh3d_begin_close(lease) {
                Ok(()) | Err(Mesh3dFault::Closing) => {}
                Err(Mesh3dFault::Stale) => {
                    self.lease = None;
                    return true;
                }
                Err(_) => return false,
            }
            match mesh3d_close_step(lease) {
                Ok(true) | Err(Mesh3dFault::Stale) => self.lease = None,
                Ok(false) | Err(_) => return false,
            }
        }
        true
    }
}

fn glb_mesh_fault(fault: Mesh3dFault) -> &'static str {
    match fault {
        Mesh3dFault::Closing => "GLB paged mesh authority was closing",
        Mesh3dFault::ItemCapacity => "GLB paged mesh authority exhausted fixed owner slots",
        Mesh3dFault::PageCapacity => "GLB paged mesh authority exhausted aggregate page credits",
        Mesh3dFault::ByteCapacity => "GLB paged mesh exceeded fixed byte credits",
        Mesh3dFault::Schema => "GLB paged mesh schema was invalid",
        Mesh3dFault::Stale => "GLB paged mesh generation/revision witness was stale",
        Mesh3dFault::Order => "GLB paged mesh field order was invalid",
        Mesh3dFault::Incomplete => "GLB paged mesh was not terminally complete",
    }
}

fn glb_component_bytes(component: u16) -> Result<usize, &'static str> {
    match component {
        5120 | 5121 => Ok(1),
        5122 | 5123 => Ok(2),
        5125 | 5126 => Ok(4),
        _ => Err("GLB component type was unsupported"),
    }
}

fn glb_read_component(owner: &RendererAssetFetchOwner, pages: &RendererAssetPageIndex, absolute: usize, component: u16, normalized: bool) -> Result<f32, &'static str> {
    let value = match component {
        5120 => {
            let value = i8::from_le_bytes(pages.read::<1>(owner, absolute)?);
            if normalized {
                (f32::from(value) / 127.0).max(-1.0)
            } else {
                f32::from(value)
            }
        }
        5121 => {
            let value = u8::from_le_bytes(pages.read::<1>(owner, absolute)?);
            if normalized {
                f32::from(value) / 255.0
            } else {
                f32::from(value)
            }
        }
        5122 => {
            let value = i16::from_le_bytes(pages.read::<2>(owner, absolute)?);
            if normalized {
                (f32::from(value) / 32767.0).max(-1.0)
            } else {
                f32::from(value)
            }
        }
        5123 => {
            let value = u16::from_le_bytes(pages.read::<2>(owner, absolute)?);
            if normalized {
                f32::from(value) / 65535.0
            } else {
                f32::from(value)
            }
        }
        5125 => {
            let value = u32::from_le_bytes(pages.read::<4>(owner, absolute)?);
            if normalized {
                value as f32 / u32::MAX as f32
            } else {
                value as f32
            }
        }
        5126 if !normalized => f32::from_le_bytes(pages.read::<4>(owner, absolute)?),
        5126 => return Err("GLB FLOAT accessor could not be normalized"),
        _ => return Err("GLB semantic component type was unsupported"),
    };
    value.is_finite().then_some(value).ok_or("GLB semantic component was not finite")
}

#[derive(Clone, Copy)]
enum RendererAssetProbePhase {
    Reading,
    Parsing,
    Semantic,
    Materializing,
    Ready,
    Closing,
}

struct RendererAssetProbe {
    owner: Option<RendererAssetFetchOwner>,
    prefix: [u8; RENDERER_ASSET_PROBE_BYTES],
    prefix_len: u8,
    observed_bytes: usize,
    page_cursor: RendererAssetPageCursor,
    page_index: RendererAssetPageIndex,
    format: Option<RendererAssetFormatCursor>,
    glb_schema: Option<GlbSchemaCursor>,
    glb_materialize: Option<GlbMaterializeCursor>,
    phase: RendererAssetProbePhase,
}

enum RendererAssetProbeStep {
    Pending,
    Ready,
    Fault(&'static str),
}

impl RendererAssetProbe {
    fn new(owner: RendererAssetFetchOwner) -> Self {
        Self {
            owner: Some(owner),
            prefix: [0; RENDERER_ASSET_PROBE_BYTES],
            prefix_len: 0,
            observed_bytes: 0,
            page_cursor: RendererAssetPageCursor::new(),
            page_index: RendererAssetPageIndex::new(),
            format: None,
            glb_schema: None,
            glb_materialize: None,
            phase: RendererAssetProbePhase::Reading,
        }
    }

    fn owner(&self) -> &RendererAssetFetchOwner {
        self.owner.as_ref().expect("asset probe owns response")
    }

    fn owner_mut(&mut self) -> &mut RendererAssetFetchOwner {
        self.owner.as_mut().expect("asset probe owns response")
    }

    fn step(&mut self) -> RendererAssetProbeStep {
        match self.phase {
            RendererAssetProbePhase::Reading => {
                let page = match self.owner().decode_page() {
                    Ok(Some(page)) => page,
                    Ok(None) => return self.finish_probe(),
                    Err(_) => return self.fault("asset response page cursor lost exact ownership"),
                };
                self.observed_bytes = match self.observed_bytes.checked_add(page.bytes().len()) {
                    Some(bytes) => bytes,
                    None => return self.fault("asset response byte count overflowed"),
                };
                if let Err(detail) = self.page_index.admit(page.bytes().len()) {
                    return self.fault(detail);
                }
                let remaining = RENDERER_ASSET_PROBE_BYTES - usize::from(self.prefix_len);
                let take = remaining.min(page.bytes().len());
                if take != 0 {
                    let start = usize::from(self.prefix_len);
                    self.prefix[start..start + take].copy_from_slice(&page.bytes()[..take]);
                    self.prefix_len += take as u8;
                }
                if self.owner_mut().advance_decode_page().is_err() {
                    return self.fault("asset response page cursor could not advance");
                }
                RendererAssetProbeStep::Pending
            }
            RendererAssetProbePhase::Parsing => {
                let owner = self.owner.as_mut().expect("asset probe owns response");
                let block = match self.page_cursor.read_block(owner) {
                    Ok(Some(block)) => block,
                    Ok(None) => {
                        if self.format.as_ref().expect("parsing asset owns format cursor").finish().is_err() {
                            return self.fault("asset retained structure decoder rejected malformed input");
                        }
                        if self.owner_mut().rewind_decode_pages().is_err() {
                            return self.fault("asset response could not rewind after retained structure decode");
                        }
                        self.page_cursor = RendererAssetPageCursor::new();
                        if matches!(self.owner().kind(), WorldAssetRequestKind::Glb) {
                            self.glb_schema = Some(GlbSchemaCursor::new());
                            self.phase = RendererAssetProbePhase::Semantic;
                            return RendererAssetProbeStep::Pending;
                        }
                        self.phase = RendererAssetProbePhase::Ready;
                        return RendererAssetProbeStep::Ready;
                    }
                    Err(detail) => return self.fault(detail),
                };
                if self.format.as_mut().expect("parsing asset owns format cursor").feed(&block.0[..usize::from(block.1)]).is_err() {
                    return self.fault("asset retained structure decoder rejected malformed input");
                }
                RendererAssetProbeStep::Pending
            }
            RendererAssetProbePhase::Semantic => {
                let mut schema = self.glb_schema.take().expect("semantic GLB probe owns schema cursor");
                let result = schema.step(self.owner_mut());
                match result {
                    Ok(false) => {
                        self.glb_schema = Some(schema);
                        RendererAssetProbeStep::Pending
                    }
                    Ok(true) => {
                        self.glb_materialize = Some(GlbMaterializeCursor::new(schema.output));
                        self.phase = RendererAssetProbePhase::Materializing;
                        RendererAssetProbeStep::Pending
                    }
                    Err(detail) => self.fault(detail),
                }
            }
            RendererAssetProbePhase::Materializing => {
                let mut materialize = self.glb_materialize.take().expect("GLB probe owns materializer");
                let result = materialize.step(self.owner.as_ref().expect("asset probe owns response"), &self.page_index);
                self.glb_materialize = Some(materialize);
                match result {
                    Ok(false) => RendererAssetProbeStep::Pending,
                    Ok(true) => {
                        self.phase = RendererAssetProbePhase::Ready;
                        RendererAssetProbeStep::Ready
                    }
                    Err(detail) => self.fault(detail),
                }
            }
            RendererAssetProbePhase::Ready => RendererAssetProbeStep::Ready,
            RendererAssetProbePhase::Closing => RendererAssetProbeStep::Pending,
        }
    }

    fn finish_probe(&mut self) -> RendererAssetProbeStep {
        if self.observed_bytes != self.owner().received_bytes() {
            return self.fault("asset response byte witness did not match its sealed claim");
        }
        let prefix_bytes = self.prefix;
        let prefix = &prefix_bytes[..usize::from(self.prefix_len)];
        let valid = match self.owner().kind() {
            WorldAssetRequestKind::Glb => {
                prefix.len() >= 12
                    && &prefix[..4] == b"glTF"
                    && u32::from_le_bytes(prefix[4..8].try_into().expect("fixed GLB version bytes")) == 2
                    && usize::try_from(u32::from_le_bytes(prefix[8..12].try_into().expect("fixed GLB length bytes"))).ok() == Some(self.observed_bytes)
            }
            WorldAssetRequestKind::ReferenceImage | WorldAssetRequestKind::UiImage { .. } => renderer_asset_image_prefix_is_valid(prefix),
            WorldAssetRequestKind::MapTile { vector: false, .. } => renderer_asset_image_prefix_is_valid(prefix),
            WorldAssetRequestKind::MapTile { vector: true, .. } | WorldAssetRequestKind::Terrain { .. } => self.observed_bytes != 0,
        };
        if !valid {
            return self.fault("asset response format probe rejected malformed input");
        }
        if self.owner_mut().rewind_decode_pages().is_err() {
            return self.fault("asset response could not rewind for its retained decoder");
        }
        self.format = match RendererAssetFormatCursor::new(self.owner().kind(), prefix, self.observed_bytes) {
            Ok(cursor) => Some(cursor),
            Err(detail) => return self.fault(detail),
        };
        self.page_cursor = RendererAssetPageCursor::new();
        self.phase = RendererAssetProbePhase::Parsing;
        RendererAssetProbeStep::Pending
    }

    fn fault(&mut self, detail: &'static str) -> RendererAssetProbeStep {
        self.owner_mut().begin_close();
        self.phase = RendererAssetProbePhase::Closing;
        RendererAssetProbeStep::Fault(detail)
    }

    fn take_ready_mesh_lease(&mut self) -> Option<Mesh3dLease> {
        (matches!(self.phase, RendererAssetProbePhase::Ready) && matches!(self.owner().kind(), WorldAssetRequestKind::Glb)).then(|| self.glb_materialize.as_mut()?.take_mesh_lease()).flatten()
    }

    fn restore_ready_mesh_lease(&mut self, lease: Mesh3dLease) {
        self.glb_materialize.as_mut().expect("ready GLB probe owns its materializer").restore_mesh_lease(lease);
    }

    fn finish_ready_mesh(&mut self) {
        assert!(self.glb_materialize.as_ref().is_some_and(|materialize| materialize.lease.is_none()), "published GLB probe relinquished its paged mesh lease");
        self.owner_mut().begin_close();
        self.phase = RendererAssetProbePhase::Closing;
    }

    fn begin_close(&mut self) {
        if let Some(materialize) = self.glb_materialize.as_mut() {
            materialize.begin_close();
        }
        self.owner_mut().begin_close();
        self.phase = RendererAssetProbePhase::Closing;
    }

    fn close_step(&mut self) -> bool {
        self.begin_close();
        if self.glb_materialize.as_mut().is_some_and(|materialize| !materialize.close_step()) {
            return false;
        }
        self.glb_materialize = None;
        self.owner_mut().close_step()
    }

    fn take_terminal_owner(&mut self) -> Option<RendererAssetFetchOwner> {
        self.owner.as_ref().is_some_and(|owner| owner.owner().terminal_is_empty()).then(|| self.owner.take().expect("terminal probe owner"))
    }
}

fn renderer_asset_image_prefix_is_valid(prefix: &[u8]) -> bool {
    prefix.starts_with(b"\x89PNG\r\n\x1a\n")
        || prefix.starts_with(&[0xff, 0xd8, 0xff])
        || std::str::from_utf8(prefix).ok().is_some_and(|text| {
            let text = text.trim_start_matches(|character: char| character.is_ascii_whitespace());
            text.starts_with("<svg") || text.starts_with("<?xml")
        })
}
//#endregion 📡️RendererAssetAuthority

//#region 🧵️RendererWorkerPool
/// 🧵️ Resolves the interactive OS process's single worker pool for every renderer subsystem.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn renderer_worker_pool() -> semio_framework_async::WorkerPool {
    let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, cores))
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn renderer_worker_pool() -> semio_framework_async::WorkerPool {
    semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, 1))
}

#[cfg(not(target_arch = "wasm32"))]
struct RendererIoHandle {
    slot: usize,
    generation: u64,
    completed: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoHandle {
    fn try_take(&mut self) -> Option<Result<semio_framework_os_services::NativeIoValue, String>> {
        let result = renderer_io_with_node(self.slot, self.generation, |node| {
            let result = node.result.take()?;
            node.result_taken = true;
            RENDERER_IO_WAKE.store(true, std::sync::atomic::Ordering::Release);
            Some(result)
        })
        .flatten();
        self.completed |= result.is_some();
        result
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl std::future::Future for RendererIoHandle {
    type Output = Result<semio_framework_os_services::NativeIoValue, String>;

    fn poll(mut self: std::pin::Pin<&mut Self>, cx: &mut std::task::Context<'_>) -> std::task::Poll<Self::Output> {
        if let Some(result) = self.try_take() {
            return std::task::Poll::Ready(result);
        }
        let registered = renderer_io_with_node(self.slot, self.generation, |node| {
            node.waker = Some(cx.waker().clone());
            if let Some(session) = node.session.as_ref() {
                let _ = session.register_wake(cx.waker());
            }
        })
        .is_some();
        if registered || renderer_io_generation_live(self.slot, self.generation) {
            RENDERER_IO_WAKE.store(true, std::sync::atomic::Ordering::Release);
            std::task::Poll::Pending
        } else {
            self.completed = true;
            std::task::Poll::Ready(Err("native I/O mounted session generation is stale".into()))
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for RendererIoHandle {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        if let Some(slot) = RENDERER_IO_SLOTS.get(self.slot) {
            if slot.generation.load(std::sync::atomic::Ordering::Acquire) == self.generation {
                slot.cancel_requested.store(true, std::sync::atomic::Ordering::Release);
            }
        }
        RENDERER_IO_WAKE.store(true, std::sync::atomic::Ordering::Release);
    }
}

#[cfg(not(target_arch = "wasm32"))]
const RENDERER_IO_SESSION_SLOTS: usize = semio_framework_job::WORKER_JOB_SESSION_SLOTS + 1;
#[cfg(not(target_arch = "wasm32"))]
const RENDERER_IO_VACANT: u8 = 0;
#[cfg(not(target_arch = "wasm32"))]
const RENDERER_IO_LIVE: u8 = 1;
#[cfg(not(target_arch = "wasm32"))]
const RENDERER_IO_CHECKED_OUT: u8 = 2;
#[cfg(not(target_arch = "wasm32"))]
const RENDERER_IO_EXHAUSTED: u8 = 3;

#[cfg(not(target_arch = "wasm32"))]
struct RendererIoSlot {
    generation: std::sync::atomic::AtomicU64,
    state: std::sync::atomic::AtomicU8,
    node: std::sync::atomic::AtomicPtr<RendererIoNode>,
    cancel_requested: std::sync::atomic::AtomicBool,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoSlot {
    const fn vacant() -> Self {
        Self { generation: std::sync::atomic::AtomicU64::new(0), state: std::sync::atomic::AtomicU8::new(RENDERER_IO_VACANT), node: std::sync::atomic::AtomicPtr::new(std::ptr::null_mut()), cancel_requested: std::sync::atomic::AtomicBool::new(false) }
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct RendererIoNode {
    session: Option<semio_framework_job::WorkerJobSession<semio_framework_os_services::NativeIoJob>>,
    rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<semio_framework_os_services::NativeIoJob>>,
    ticket: Option<semio_framework_job::WorkerJobTicket>,
    result: Option<Result<semio_framework_os_services::NativeIoValue, String>>,
    result_taken: bool,
    detached: bool,
    cancel: semio_framework_async::CancelToken,
    waker: Option<std::task::Waker>,
}

#[cfg(not(target_arch = "wasm32"))]
impl RendererIoNode {
    fn wake(&mut self) {
        if let Some(waker) = self.waker.take() {
            waker.wake();
        }
    }

    fn close_detached_result(&mut self) -> bool {
        if !self.detached {
            return false;
        }
        let Some(result) = self.result.as_mut() else { return false };
        match result {
            Ok(value) if !value.terminal_is_empty() => {
                let _ = value.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                return true;
            }
            Ok(_) => {}
            Err(error) if !error.is_empty() => {
                if error.len() > semio_framework_job::JOB_PAYLOAD_PAGE_BYTES {
                    return false;
                }
                error.clear();
                return true;
            }
            Err(_) => {}
        }
        self.result = None;
        self.result_taken = true;
        true
    }

    fn pump_one(&mut self) {
        if self.close_detached_result() {
            self.wake();
            return;
        }
        if let Some(rejected) = self.rejected.as_mut() {
            let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if rejected.terminal_is_empty() {
                self.rejected = None;
            }
            self.wake();
            return;
        }
        let Some(session) = self.session.as_ref() else {
            self.wake();
            return;
        };
        let _ = session.take_wake();
        if self.result.is_some() || self.detached {
            if !matches!(session.poll(), semio_framework_job::WorkerJobPoll::Closing | semio_framework_job::WorkerJobPoll::TerminalEmpty) {
                let _ = session.begin_close();
                self.wake();
                return;
            }
            let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if session.terminal_is_empty() {
                self.session = None;
            }
            self.wake();
            return;
        }
        match session.poll() {
            semio_framework_job::WorkerJobPoll::Idle => match session.try_submit_step(&renderer_worker_pool(), semio_framework_async::Lane::Io) {
                Ok(ticket) => self.ticket = Some(ticket),
                Err(semio_framework_job::WorkerJobSubmitFault::Pool(semio_framework_async::WorkerSubmitErrorKind::Saturated | semio_framework_async::WorkerSubmitErrorKind::Contended)) => {
                    if let Ok(rejected) = session.take_rejected() {
                        rejected.resume();
                    }
                }
                Err(error) => {
                    if let Ok(rejected) = session.take_rejected() {
                        rejected.begin_close();
                    } else {
                        let _ = session.begin_close();
                    }
                    self.result = Some(Err(format!("native I/O worker submission failed: {error:?}")));
                }
            },
            semio_framework_job::WorkerJobPoll::Outcome => {
                let Some(ticket) = self.ticket.take() else {
                    self.result = Some(Err("native I/O outcome lost its exact ticket".into()));
                    let _ = session.begin_close();
                    self.wake();
                    return;
                };
                match session.take_outcome(ticket) {
                    Ok(mut owner) => {
                        if matches!(owner.take_outcome(), semio_framework_job::StepOutcome::Yield) {
                            if owner.resume().is_err() {
                                self.result = Some(Err("native I/O yield could not resume exact owner".into()));
                            }
                        } else {
                            owner.begin_close();
                            self.result = Some(Err("native I/O published a nonterminal retained payload".into()));
                        }
                    }
                    Err(_) => self.result = Some(Err("native I/O outcome handback contention".into())),
                }
            }
            semio_framework_job::WorkerJobPoll::Terminal => match session.take_terminal() {
                Ok(mut owner) => {
                    self.result = owner.job_mut().take_result().or_else(|| Some(Err("native I/O terminal owner had no structured result".into())));
                    owner.begin_close();
                }
                Err(_) => self.result = Some(Err("native I/O terminal handback contention".into())),
            },
            semio_framework_job::WorkerJobPoll::Rejected => {
                if let Ok(rejected) = session.take_rejected() {
                    rejected.resume();
                }
            }
            semio_framework_job::WorkerJobPoll::Closing => {
                let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                if session.terminal_is_empty() {
                    self.session = None;
                }
            }
            semio_framework_job::WorkerJobPoll::Submitted | semio_framework_job::WorkerJobPoll::CheckedOut | semio_framework_job::WorkerJobPoll::TerminalEmpty => {}
        }
        self.wake();
    }

    fn terminal_is_empty(&self) -> bool {
        self.session.is_none() && self.rejected.is_none() && self.result.is_none() && (self.detached || self.result_taken)
    }
}

#[cfg(not(target_arch = "wasm32"))]
static RENDERER_IO_SLOTS: [RendererIoSlot; RENDERER_IO_SESSION_SLOTS] = [const { RendererIoSlot::vacant() }; RENDERER_IO_SESSION_SLOTS];
#[cfg(not(target_arch = "wasm32"))]
static RENDERER_IO_WAKE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
#[cfg(not(target_arch = "wasm32"))]
static RENDERER_IO_PUMP_CURSOR: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

#[cfg(not(target_arch = "wasm32"))]
fn renderer_io_with_node<R>(index: usize, generation: u64, use_node: impl FnOnce(&mut RendererIoNode) -> R) -> Option<R> {
    let slot = RENDERER_IO_SLOTS.get(index)?;
    if slot.generation.load(std::sync::atomic::Ordering::Acquire) != generation || slot.state.compare_exchange(RENDERER_IO_LIVE, RENDERER_IO_CHECKED_OUT, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
        return None;
    }
    if slot.generation.load(std::sync::atomic::Ordering::Acquire) != generation {
        slot.state.store(RENDERER_IO_LIVE, std::sync::atomic::Ordering::Release);
        return None;
    }
    let pointer = slot.node.load(std::sync::atomic::Ordering::Acquire);
    let result = (!pointer.is_null()).then(|| use_node(unsafe { &mut *pointer }));
    slot.state.store(RENDERER_IO_LIVE, std::sync::atomic::Ordering::Release);
    result
}

#[cfg(not(target_arch = "wasm32"))]
fn renderer_io_generation_live(index: usize, generation: u64) -> bool {
    RENDERER_IO_SLOTS.get(index).is_some_and(|slot| {
        slot.generation.load(std::sync::atomic::Ordering::Acquire) == generation
            && matches!(slot.state.load(std::sync::atomic::Ordering::Acquire), RENDERER_IO_LIVE | RENDERER_IO_CHECKED_OUT)
            && !slot.node.load(std::sync::atomic::Ordering::Acquire).is_null()
    })
}

#[cfg(not(target_arch = "wasm32"))]
fn pump_renderer_io_sessions(maximum_sessions: usize) -> usize {
    if maximum_sessions == 0 {
        return 0;
    }
    let start = RENDERER_IO_PUMP_CURSOR.fetch_add(maximum_sessions, std::sync::atomic::Ordering::AcqRel);
    let mut advanced = 0;
    for offset in 0..RENDERER_IO_SESSION_SLOTS {
        if advanced == maximum_sessions {
            break;
        }
        let index = start.wrapping_add(offset) % RENDERER_IO_SESSION_SLOTS;
        let slot = &RENDERER_IO_SLOTS[index];
        if slot.state.compare_exchange(RENDERER_IO_LIVE, RENDERER_IO_CHECKED_OUT, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            continue;
        }
        let pointer = slot.node.load(std::sync::atomic::Ordering::Acquire);
        if pointer.is_null() {
            slot.state.store(RENDERER_IO_VACANT, std::sync::atomic::Ordering::Release);
            continue;
        }
        let node = unsafe { &mut *pointer };
        let cancel_advanced = slot.cancel_requested.swap(false, std::sync::atomic::Ordering::AcqRel);
        if cancel_advanced {
            node.detached = true;
            node.cancel.cancel_now();
            if let Some(session) = node.session.as_ref() {
                let _ = session.begin_close();
            }
            if let Some(rejected) = node.rejected.as_mut() {
                rejected.begin_close();
            }
            node.wake();
        }
        if !cancel_advanced {
            node.pump_one();
        }
        if node.terminal_is_empty() {
            slot.node.store(std::ptr::null_mut(), std::sync::atomic::Ordering::Release);
            slot.cancel_requested.store(false, std::sync::atomic::Ordering::Release);
            drop(unsafe { Box::from_raw(pointer) });
            slot.state.store(if slot.generation.load(std::sync::atomic::Ordering::Acquire) == u64::MAX { RENDERER_IO_EXHAUSTED } else { RENDERER_IO_VACANT }, std::sync::atomic::Ordering::Release);
        } else {
            slot.state.store(RENDERER_IO_LIVE, std::sync::atomic::Ordering::Release);
        }
        advanced += 1;
    }
    if RENDERER_IO_SLOTS.iter().any(|slot| slot.state.load(std::sync::atomic::Ordering::Acquire) == RENDERER_IO_LIVE) {
        RENDERER_IO_WAKE.store(true, std::sync::atomic::Ordering::Release);
    }
    advanced
}

#[cfg(not(target_arch = "wasm32"))]
fn submit_renderer_io(request: semio_framework_os_services::NativeIoRequest) -> Result<RendererIoHandle, String> {
    use semio_framework_job::{allocate_operation_id, root_cancel_token, BatchDriveConfig, BatchJobParams, Generation, InteractiveStage, INTERACTIVE_LANE_FUEL, INTERACTIVE_LANE_WALL_US};
    let Some((slot_index, generation)) = RENDERER_IO_SLOTS.iter().enumerate().find_map(|(index, slot)| {
        if slot.state.compare_exchange(RENDERER_IO_VACANT, RENDERER_IO_CHECKED_OUT, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return None;
        }
        let Some(generation) = slot.generation.load(std::sync::atomic::Ordering::Acquire).checked_add(1) else {
            slot.state.store(RENDERER_IO_EXHAUSTED, std::sync::atomic::Ordering::Release);
            return None;
        };
        slot.generation.store(generation, std::sync::atomic::Ordering::Release);
        slot.cancel_requested.store(false, std::sync::atomic::Ordering::Release);
        Some((index, generation))
    }) else {
        return Err("native I/O mounted session registry is exhausted".into());
    };
    let job = semio_framework_os_services::NativeIoJob::new(request);
    let cancel = root_cancel_token();
    let params = BatchJobParams {
        operation: allocate_operation_id(),
        generation: Generation(generation),
        cancel: cancel.clone(),
        config: BatchDriveConfig { site: "os_renderer_native_io", stage: InteractiveStage::InteractiveStep, fuel_per_step: INTERACTIVE_LANE_FUEL, step_budget_us: INTERACTIVE_LANE_WALL_US },
        now_us: semio_framework_job::default_now_us,
    };
    let (session, rejected, result) = match semio_framework_job::WorkerJobSession::try_new(job, params) {
        Ok(session) => (Some(session), None, None),
        Err(mut rejected) => {
            rejected.begin_close();
            (None, Some(rejected), Some(Err("native I/O retained session registry is full".into())))
        }
    };
    let node = Box::new(RendererIoNode { session, rejected, ticket: None, result, result_taken: false, detached: false, cancel, waker: None });
    RENDERER_IO_SLOTS[slot_index].node.store(Box::into_raw(node), std::sync::atomic::Ordering::Release);
    RENDERER_IO_SLOTS[slot_index].state.store(RENDERER_IO_LIVE, std::sync::atomic::Ordering::Release);
    RENDERER_IO_WAKE.store(true, std::sync::atomic::Ordering::Release);
    Ok(RendererIoHandle { slot: slot_index, generation, completed: false })
}

#[cfg(not(target_arch = "wasm32"))]
async fn run_renderer_io(request: semio_framework_os_services::NativeIoRequest) -> Result<semio_framework_os_services::NativeIoValue, String> {
    submit_renderer_io(request)?.await
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod renderer_io_retained_tests {
    use super::*;

    #[test]
    fn mounted_registry_max_plus_one_zero_pump_drop_and_generation_are_exact() {
        let mut handles = Vec::with_capacity(RENDERER_IO_SESSION_SLOTS);
        for index in 0..RENDERER_IO_SESSION_SLOTS {
            let path = std::path::PathBuf::from(format!("/semio-retained-native-io-{index:04}"));
            let request_pointer = path.as_os_str().as_encoded_bytes().as_ptr();
            let handle = submit_renderer_io(semio_framework_os_services::NativeIoRequest::ReadBytes(path)).expect("logical maximum plus one has a mounted rejection owner");
            if index + 1 == RENDERER_IO_SESSION_SLOTS {
                let plus_one_request_pointer = request_pointer;
                let returned_pointer =
                    renderer_io_with_node(handle.slot, handle.generation, |node| node.rejected.as_ref().and_then(|rejected| rejected.job().retained_request_backing_identity()).expect("maximum plus one retains the exact rejected request"))
                        .expect("maximum plus one mounted generation");
                assert_eq!(returned_pointer, plus_one_request_pointer);
            }
            handles.push(handle);
        }
        assert_eq!(pump_renderer_io_sessions(0), 0);
        let first_slot = handles[0].slot;
        let first_generation = handles[0].generation;
        drop(handles);
        assert_eq!(pump_renderer_io_sessions(1), 1, "one host turn advances one mounted control opportunity");
        for _ in 0..RENDERER_IO_SESSION_SLOTS * 16 {
            if RENDERER_IO_SLOTS.iter().all(|slot| slot.state.load(std::sync::atomic::Ordering::Acquire) != RENDERER_IO_LIVE) {
                break;
            }
            assert!(pump_renderer_io_sessions(1) <= 1);
        }
        assert!(RENDERER_IO_SLOTS.iter().all(|slot| slot.state.load(std::sync::atomic::Ordering::Acquire) != RENDERER_IO_LIVE));
        assert!(!renderer_io_generation_live(first_slot, first_generation));
        let replacement = submit_renderer_io(semio_framework_os_services::NativeIoRequest::ProcessResidentBytes).expect("closed registry slot is reusable with a new generation");
        assert_eq!(replacement.slot, first_slot);
        assert!(replacement.generation > first_generation);
        drop(replacement);
        for _ in 0..16 {
            let _ = pump_renderer_io_sessions(1);
        }
    }
}
//#endregion 🧵️RendererWorkerPool

//#region 🎠️KernelRuntime
/// 🎭️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet H3-wgpu-native; upgraded by terra-kernel-loop):
/// `📓️design-runtime.md` §1's "wgpu native" host — `Kernel` runs as a persistent, bounded-poll state
/// machine on the renderer's shared worker pool; the winit thread only submits requests and drains
/// outbound results. terra-kernel-loop replaced the
/// original single-`ShardLoop` request-servant with `crate::parallel_runtime::ParallelRuntime`: real
/// `Kernel::submit`/`tick`/`complete` (DRR fairness, failure-ladder/metrics bookkeeping) dispatched
/// across K logical `ShardExecutor`s on the same pool, one per `ShardTable`-pinned shard — see
/// `📓️terra-kernel-loop-report.md` for what is (and, per that report's own honest-gaps section, is
/// NOT) wired all the way through. `ProgramBridgeBackend::Wasm` (in `🌉️ProgramBridge/`) holds a
/// [`KernelClient`] instead of the deleted `Arc<WasmPluginRuntime>`; every plugin turn now executes
/// through `Kernel` + `GuestRuntime`/`WasmtimeRuntime` + `ParallelRuntime` on pool workers, never
/// in-process on the winit thread.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) mod kernel_runtime {
    use semio_framework::kernel::{BrokerCapabilityGrant, Budget as TurnBudget, Effect, Event, JobPlacement, MessageEndpoint, QuotaSchema, TurnResult, UiPatch as KernelUiPatch};
    use semio_framework_actor::{
        intersect_capabilities, ActivationEvent, ActorId, ActorKind, Backpressure, CapabilityGrant, Envelope, JobOperation, JobProgressIdentity, JobProgressKind, JobProgressLiveAuthority, JobProgressOverlayStore, JobProgressReceipt,
        JobProgressRejected, JobPublication, JobReplayFault, JobReplayLog, JobReplayPublicationKind, JobReplayPublicationPolicy, JobReplayRecordHeader, JobReplayRequest, JobReplayRoute, JobReplayStep, JobTurn, Lane, Origin, PackageHash, PackageId,
        Payload,
    };
    use semio_framework_plugin_host::shard::ShardOutcome;
    use semio_framework_plugin_host::{GuestRuntime, GuestRuntimes, OwnedRuntime, PackageRef};
    use std::collections::HashMap;
    use std::future::Future;
    use std::mem::ManuallyDrop;
    use std::path::PathBuf;
    use std::pin::Pin;
    use std::sync::{Arc, Mutex, OnceLock};
    use std::task::{Context, Poll, Waker};
    use std::time::Duration;
    use ui_contract::{SurfaceId, UiDocumentBuilder, UiDocumentLease, UiDocumentLimits, UiFixedList, UiPatchApplyOutcome, UiPatchApplyProducer, UiPatchApplyRejected, UiPatchApplyStep, UiRevision, UiSnapshotState, UI_DOCUMENT_LEASE_SLOTS};

    #[derive(Default)]
    struct RendererSequenceAuthority {
        committed: u64,
        reserved: Option<u64>,
        exhausted: bool,
    }

    impl RendererSequenceAuthority {
        fn try_reserve(&mut self) -> Result<u64, String> {
            if self.exhausted || self.reserved.is_some() {
                return Err("renderer sequence authority is exhausted or reserved".to_string());
            }
            let generation = self.committed.checked_add(1).filter(|generation| *generation != 0).ok_or_else(|| "renderer sequence authority is permanently exhausted".to_string())?;
            self.reserved = Some(generation);
            Ok(generation)
        }

        fn commit(&mut self, generation: u64) -> Result<(), String> {
            if self.reserved != Some(generation) || self.committed.checked_add(1) != Some(generation) {
                return Err("renderer sequence reservation is stale".to_string());
            }
            self.reserved = None;
            self.committed = generation;
            self.exhausted = generation == u64::MAX;
            Ok(())
        }

        fn rollback(&mut self, generation: u64) {
            if self.reserved == Some(generation) {
                self.reserved = None;
            }
        }
    }

    struct RendererSequenceReservation {
        generation: u64,
        active: bool,
    }

    impl RendererSequenceReservation {
        fn generation(&self) -> u64 {
            self.generation
        }

        fn commit(mut self) -> Result<u64, String> {
            let mut authority = renderer_sequence_authority().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            authority.commit(self.generation)?;
            self.active = false;
            Ok(self.generation)
        }
    }

    impl Drop for RendererSequenceReservation {
        fn drop(&mut self) {
            if !self.active {
                return;
            }
            let mut authority = renderer_sequence_authority().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
            authority.rollback(self.generation);
        }
    }

    fn renderer_sequence_authority() -> &'static Mutex<RendererSequenceAuthority> {
        static AUTHORITY: OnceLock<Mutex<RendererSequenceAuthority>> = OnceLock::new();
        AUTHORITY.get_or_init(|| Mutex::new(RendererSequenceAuthority::default()))
    }

    fn reserve_seq() -> Result<RendererSequenceReservation, String> {
        let mut authority = renderer_sequence_authority().lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        let generation = authority.try_reserve()?;
        Ok(RendererSequenceReservation { generation, active: true })
    }

    const JOB_PROGRESS_PRESENTATION_CAPACITY: usize = 64;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum JobProgressPresentationState {
        Vacant,
        Reserved,
        Ready,
        CheckedOut,
        Presented,
    }

    #[derive(Clone, Copy, Debug)]
    struct JobProgressPresentationSlot {
        epoch: u64,
        admission_sequence: u64,
        state: JobProgressPresentationState,
        identity: JobProgressIdentity,
        kind: JobProgressKind,
        applied_progress: u64,
    }

    impl JobProgressPresentationSlot {
        fn vacant() -> Self {
            Self {
                epoch: 0,
                admission_sequence: 0,
                state: JobProgressPresentationState::Vacant,
                identity: JobProgressIdentity { actor: ActorId(0), job: 0, operation: 0, base_revision: 0, generation: 0, step_sequence: 0, preview_sequence: 0 },
                kind: JobProgressKind::Yield,
                applied_progress: 0,
            }
        }
    }

    struct JobProgressPresentationBridge {
        slots: [JobProgressPresentationSlot; JOB_PROGRESS_PRESENTATION_CAPACITY],
        reserve_cursor: usize,
        next_admission_sequence: u64,
    }

    impl JobProgressPresentationBridge {
        fn new() -> Self {
            Self { slots: [JobProgressPresentationSlot::vacant(); JOB_PROGRESS_PRESENTATION_CAPACITY], reserve_cursor: 0, next_admission_sequence: 1 }
        }

        fn reserve(&mut self, identity: JobProgressIdentity, kind: JobProgressKind, applied_progress: u64) -> Option<JobProgressPresentationToken> {
            let index = (0..JOB_PROGRESS_PRESENTATION_CAPACITY)
                .map(|offset| (self.reserve_cursor + offset) % JOB_PROGRESS_PRESENTATION_CAPACITY)
                .find(|index| self.slots[*index].state == JobProgressPresentationState::Vacant && self.slots[*index].epoch != u64::MAX)?;
            let admission_sequence = self.next_admission_sequence;
            self.next_admission_sequence = self.next_admission_sequence.checked_add(1)?;
            let slot = &mut self.slots[index];
            slot.epoch = slot.epoch.checked_add(1)?;
            slot.admission_sequence = admission_sequence;
            slot.state = JobProgressPresentationState::Reserved;
            slot.identity = identity;
            slot.kind = kind;
            slot.applied_progress = applied_progress;
            self.reserve_cursor = (index + 1) % JOB_PROGRESS_PRESENTATION_CAPACITY;
            Some(JobProgressPresentationToken { index, epoch: slot.epoch })
        }

        fn publish(&mut self, token: JobProgressPresentationToken) -> bool {
            let Some(slot) = self.slots.get_mut(token.index) else { return false };
            if slot.epoch != token.epoch || slot.state != JobProgressPresentationState::Reserved {
                return false;
            }
            slot.state = JobProgressPresentationState::Ready;
            true
        }

        fn cancel(&mut self, token: JobProgressPresentationToken) -> bool {
            let Some(slot) = self.slots.get_mut(token.index) else { return false };
            if slot.epoch != token.epoch || !matches!(slot.state, JobProgressPresentationState::Reserved | JobProgressPresentationState::Ready) {
                return false;
            }
            slot.state = JobProgressPresentationState::Vacant;
            true
        }

        fn can_cancel(&self, token: JobProgressPresentationToken) -> bool {
            self.slots.get(token.index).is_some_and(|slot| slot.epoch == token.epoch && matches!(slot.state, JobProgressPresentationState::Reserved | JobProgressPresentationState::Ready))
        }

        fn take(&mut self) -> Option<JobProgressPresentationLease> {
            let index = self.oldest_ready_index()?;
            let slot = &mut self.slots[index];
            slot.state = JobProgressPresentationState::CheckedOut;
            Some(JobProgressPresentationLease { token: JobProgressPresentationToken { index, epoch: slot.epoch }, identity: slot.identity, kind: slot.kind, applied_progress: slot.applied_progress, terminal: false })
        }

        fn oldest_ready_index(&self) -> Option<usize> {
            self.slots.iter().enumerate().filter(|(_, slot)| slot.state == JobProgressPresentationState::Ready).min_by_key(|(_, slot)| slot.admission_sequence).map(|(index, _)| index)
        }

        fn return_lease(&mut self, token: JobProgressPresentationToken) -> bool {
            let Some(slot) = self.slots.get_mut(token.index) else { return false };
            if slot.epoch != token.epoch || slot.state != JobProgressPresentationState::CheckedOut {
                return false;
            }
            slot.state = JobProgressPresentationState::Ready;
            true
        }

        fn presented(&mut self, token: JobProgressPresentationToken) -> bool {
            let Some(slot) = self.slots.get_mut(token.index) else { return false };
            if slot.epoch != token.epoch || slot.state != JobProgressPresentationState::CheckedOut {
                return false;
            }
            slot.state = JobProgressPresentationState::Presented;
            true
        }

        fn release_presented(&mut self, token: JobProgressPresentationToken) -> bool {
            let Some(slot) = self.slots.get_mut(token.index) else { return false };
            if slot.epoch != token.epoch || slot.state != JobProgressPresentationState::Presented {
                return false;
            }
            slot.state = JobProgressPresentationState::Vacant;
            true
        }

        fn terminal_is_empty(&self) -> bool {
            self.slots.iter().all(|slot| slot.state == JobProgressPresentationState::Vacant)
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct JobProgressPresentationToken {
        index: usize,
        epoch: u64,
    }

    pub(crate) struct JobProgressPresentationLease {
        token: JobProgressPresentationToken,
        identity: JobProgressIdentity,
        kind: JobProgressKind,
        applied_progress: u64,
        terminal: bool,
    }

    impl JobProgressPresentationLease {
        pub(crate) fn visual(&self) -> (JobProgressKind, u64) {
            (self.kind, self.applied_progress)
        }

        pub(crate) fn acknowledge_presented(&mut self) -> bool {
            if self.terminal {
                return false;
            }
            let admitted = KernelClient::get().try_acknowledge_job_progress(self.token);
            self.terminal = admitted;
            admitted
        }
    }

    impl Drop for JobProgressPresentationLease {
        fn drop(&mut self) {
            if !self.terminal {
                let returned = job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").return_lease(self.token);
                assert!(returned, "job-progress presentation lease must return to its exact generation slot");
            }
        }
    }

    fn job_progress_presentation_bridge() -> &'static Mutex<JobProgressPresentationBridge> {
        static BRIDGE: OnceLock<Mutex<JobProgressPresentationBridge>> = OnceLock::new();
        BRIDGE.get_or_init(|| Mutex::new(JobProgressPresentationBridge::new()))
    }

    pub(crate) fn take_job_progress_presentation() -> Option<JobProgressPresentationLease> {
        job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").take()
    }

    fn next_seq() -> Result<u64, String> {
        reserve_seq()?.commit()
    }

    fn decode_actor_turn_result(result: &semio_framework_actor::TurnResult, session: u64) -> Result<TurnResult, String> {
        let status = match &result.status {
            semio_framework_actor::TurnStatus::Idle => semio_framework::kernel::TurnStatus::Idle,
            semio_framework_actor::TurnStatus::MoreWork => semio_framework::kernel::TurnStatus::MoreWork,
            semio_framework_actor::TurnStatus::CheckpointReady { checkpoint } => semio_framework::kernel::TurnStatus::CheckpointReady { checkpoint: checkpoint.clone() },
            semio_framework_actor::TurnStatus::Faulted { detail } => semio_framework::kernel::TurnStatus::Faulted(detail.clone()),
            status => return Err(format!("kernel: unexpected job status in reactor turn: {status:?}")),
        };
        Ok(TurnResult {
            ui_patches: semio_framework::kernel::UiTurnPatchTransportLease::try_from_token(&result.ui_patches, session)
                .map_err(|error| format!("kernel: decode ui patch transport: {error}"))?
                .take_owner()
                .map_err(|_| "kernel: turn patch transport lease lost its exact owner".to_string())?,
            effects: serde_json::from_slice(&result.effects).map_err(|error| format!("kernel: decode effects: {error}"))?,
            presence: Vec::new(),
            next_wake: result.next_wake,
            status,
            fuel_used: result.usage.fuel,
            command_ingress: serde_json::from_slice(&result.command_ingress).map_err(|error| format!("kernel: decode command ingress: {error}"))?,
        })
    }

    /// ⛽️ One generous constant turn budget until the DRR scheduler threads a real per-lane one
    /// through (same honestly-flagged gap `PluginInstanceHandle`'s `RELAY_JOB_BUDGET` already
    /// documents on the host side for jobs — this is its `reactor::poll` turn-budget twin).
    const TURN_BUDGET: TurnBudget = TurnBudget { fuel: 50_000_000, deadline_ms: 100, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 8 };

    /// ⏳️ terra-kernel-loop: same tripwire shape as `scale_bench`'s own `PUMP_OUTCOME_TIMEOUT` —
    /// how long `run_turn`'s tick loop waits for a granted turn's `ShardOutcome` before giving up.
    const RUN_TURN_OUTCOME_TIMEOUT: Duration = Duration::from_secs(5);

    /// 🧵️ P1e (INTERACTIVE-JOB-RUNTIME-REFACTOR, one-pool-worker-runtime): sized from
    /// `semio_framework_async::worker_count_for` — the SAME formula [`crate::renderer_worker_pool`]
    /// itself sizes its one process-wide `WorkerPool` from — rather than a fresh ad-hoc formula,
    /// keeping "no component sizes itself per-CPU" true even though a shard count is minted before
    /// the pool object itself is touched here (`ShardExecutor` count, not thread count — shards are a
    /// pure scheduling/affinity unit post-P1c, never a thread per shard). `available_parallelism()`
    /// failing (rare; a sandboxed/exotic host) falls back to `4` cores' worth of shards rather than
    /// faulting the kernel pool task before it can start.
    fn native_shard_count() -> u16 {
        let cores = std::thread::available_parallelism().map(|n| n.get()).unwrap_or(4);
        semio_framework_async::worker_count_for(semio_framework_async::ProcessKind::InteractiveNative, cores) as u16
    }

    //#region 🔖️ExtensionIndex
    /// 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): the descriptor-driven
    /// source of truth for "which extensions activate alongside plugin X" — `📓️design-unified.md`
    /// §M6. Embedded at COMPILE time via `include_str!` (never a runtime path lookup into `🤖️
    /// generated/**`, which is gitignored and has no stable runtime location once packaged) — the
    /// same registry `🖥️hosts.rs` a few lines above this module already mounts as real Rust source,
    /// just read as data here instead of compiled as code. READ-ONLY: this file is `🤖️generated/**`,
    /// registrar-owned; never edited by this packet.
    const PLUGINS_REGISTRY_JSON: &str = include_str!("../../../../../🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json");

    /// 🧩️ The handful of fields this host actually needs out of one registry entry — every other
    /// field (`hashes`, `dependsOn`, `activationEvents`, …) is irrelevant to activation and left
    /// unparsed (serde ignores unknown JSON fields by default).
    #[derive(serde::Deserialize)]
    struct PluginDescriptorJson {
        #[serde(rename = "pluginId")]
        plugin_id: String,
        role: String,
        #[serde(default)]
        capabilities: Vec<String>,
        #[serde(default)]
        extends: Option<String>,
    }

    /// 🧩️ One extension's activation-relevant identity — `extension_id`/`package` are the SAME
    /// string (the extension crate's own `pluginId`, deliberately distinct from its parent's
    /// `PackageId` — see `activate_extensions_of`'s own doc for why this matters for package-wide
    /// quarantine isolation). `capability_requests` mirrors the registry's `capabilities` array
    /// verbatim as unscoped [`CapabilityGrant`]s (`scope: None` — this host has no capability broker
    /// populating scopes for ANY actor kind yet, extension or plugin).
    #[derive(Clone)]
    struct ExtensionRecord {
        extension_id: String,
        package: PackageId,
        capability_requests: Vec<CapabilityGrant>,
    }

    /// 🧩️ `by_parent[plugin_id]` = every installed extension whose descriptor `extends == plugin_id`
    /// — exactly the "descriptors carry `extends`... zero special-casing" design requirement.
    struct ExtensionIndex {
        by_parent: HashMap<String, Vec<ExtensionRecord>>,
    }

    impl ExtensionIndex {
        fn load() -> Self {
            let mut by_parent: HashMap<String, Vec<ExtensionRecord>> = HashMap::new();
            let entries: Vec<PluginDescriptorJson> = serde_json::from_str(PLUGINS_REGISTRY_JSON).unwrap_or_default();
            for entry in entries {
                if entry.role != "extension" {
                    continue;
                }
                let Some(parent) = entry.extends else { continue };
                let record = ExtensionRecord { extension_id: entry.plugin_id.clone(), package: PackageId(entry.plugin_id), capability_requests: entry.capabilities.into_iter().map(|capability| CapabilityGrant { capability, scope: None }).collect() };
                by_parent.entry(parent).or_default().push(record);
            }
            Self { by_parent }
        }

        fn extensions_of(&self, plugin_id: &str) -> &[ExtensionRecord] {
            self.by_parent.get(plugin_id).map(Vec::as_slice).unwrap_or(&[])
        }
    }

    /// 🧩️ Parsed once, lazily — the registry JSON is fixed at compile time (`include_str!`), so
    /// there is nothing to invalidate or re-read across the process's lifetime.
    fn extension_index() -> &'static ExtensionIndex {
        static INDEX: OnceLock<ExtensionIndex> = OnceLock::new();
        INDEX.get_or_init(ExtensionIndex::load)
    }

    /// 🧩️ Mirrors `program_bridge::load_wasm_plugins`'s own "first `.wasm` file directly inside the
    /// plugin's own directory" convention — kept as a small local helper rather than importing that
    /// module's version, which is embedded inside its own `find`-style closure, not a reusable fn.
    async fn find_wasm_artifact(dir: &std::path::Path) -> Option<PathBuf> {
        let request = semio_framework_os_services::NativeIoRequest::ScanDirectory { path: dir.to_path_buf(), directories_only: false, extension: Some("wasm".into()), first_only: true };
        match crate::run_renderer_io(request).await.ok()? {
            semio_framework_os_services::NativeIoValue::Paths(mut paths) => paths.pop(),
            _ => None,
        }
    }
    //#endregion 🔖️ExtensionIndex

    //#region 🔖️Requests/Outcomes
    struct CreateAppRequestOwner {
        wasm_path: Option<PathBuf>,
        plugin_id: Option<String>,
        app_id: Option<String>,
    }

    impl CreateAppRequestOwner {
        fn new(wasm_path: PathBuf, plugin_id: String, app_id: String) -> Self {
            Self { wasm_path: Some(wasm_path), plugin_id: Some(plugin_id), app_id: Some(app_id) }
        }

        fn into_parts(mut self) -> (PathBuf, String, String) {
            (self.wasm_path.take().expect("create request path is present"), self.plugin_id.take().expect("create request plugin is present"), self.app_id.take().expect("create request app is present"))
        }

        fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
            if let Some(length) = self.wasm_path.as_ref().map(|path| path.as_os_str().len()) {
                if length > maximum_bytes {
                    return (false, 0, 0);
                }
                drop(self.wasm_path.take().expect("create request path is present"));
                return (self.terminal_is_empty(), 1, length);
            }
            for field in [&mut self.plugin_id, &mut self.app_id] {
                if let Some(length) = field.as_ref().map(String::len) {
                    if length > maximum_bytes {
                        return (false, 0, 0);
                    }
                    drop(field.take().expect("create request string is present"));
                    return (self.terminal_is_empty(), 1, length);
                }
            }
            (true, 0, 0)
        }

        fn terminal_is_empty(&self) -> bool {
            self.wasm_path.is_none() && self.plugin_id.is_none() && self.app_id.is_none()
        }

        fn remaining_bytes(&self) -> usize {
            self.wasm_path.as_ref().map_or(0, |path| path.as_os_str().len()) + self.plugin_id.as_ref().map_or(0, String::len) + self.app_id.as_ref().map_or(0, String::len)
        }
    }

    struct QueuedKernelEvent {
        surface_visible: Option<String>,
    }

    impl QueuedKernelEvent {
        fn try_from_events(events: Vec<Event>) -> Result<Self, RejectedKernelEvents> {
            let mut events = std::collections::VecDeque::from(events);
            if events.len() != 1 {
                return Err(RejectedKernelEvents { events });
            }
            match events.pop_front().expect("one queued event is present") {
                Event::SurfaceVisible { surface } => Ok(Self { surface_visible: Some(surface) }),
                rejected => {
                    events.push_front(rejected);
                    Err(RejectedKernelEvents { events })
                }
            }
        }

        fn into_event(mut self) -> Event {
            Event::SurfaceVisible { surface: self.surface_visible.take().expect("queued surface-visible event is present") }
        }

        fn close_step(&mut self, maximum_bytes: usize) -> (bool, usize, usize) {
            let Some(length) = self.surface_visible.as_ref().map(String::len) else {
                return (true, 0, 0);
            };
            if length > maximum_bytes {
                return (false, 0, 0);
            }
            drop(self.surface_visible.take().expect("queued surface-visible event is present"));
            (true, 1, length)
        }

        fn remaining_bytes(&self) -> usize {
            self.surface_visible.as_ref().map_or(0, String::len)
        }
    }

    struct RejectedKernelEvents {
        events: std::collections::VecDeque<Event>,
    }

    impl RejectedKernelEvents {
        fn close_step(&mut self) -> (bool, usize) {
            let Some(event) = self.events.pop_front() else {
                return (true, 0);
            };
            drop(event);
            (self.events.is_empty(), 1)
        }

        fn terminal_is_empty(&self) -> bool {
            self.events.is_empty()
        }
    }

    pub(crate) enum KernelRequest {
        CreateApp {
            owner: CreateAppRequestOwner,
        },
        DestroyApp {
            owner: Arc<KernelCloseSubmission>,
        },
        /// 📡️ Non-command reactor events. Command bytes use [`KernelRequest::ExchangeCommands`]
        /// so the retained host batch lowers exactly one admitted page per turn.
        Exchange {
            instance: u32,
            event: QueuedKernelEvent,
        },
        AdvanceRetained {
            instance: u32,
            surface: SurfaceId,
        },
        MountProductReplay {
            owner: MountedProductReplayRequest,
        },
        RetireProductReplay {
            owner: MountedProductReplayRequest,
        },
        RetireProductReplayRefusal {
            owner: RefusedProductReplay,
        },
        AdvanceProductReplay {
            instance: u32,
        },
        ExchangeCommands {
            instance: u32,
            driver: semio_framework::kernel::CommandBatchDriver,
        },
        CloseRejectedCommandBuild {
            key: u64,
            owner: semio_framework::kernel::RejectedCommandBuild,
        },
        CloseRejectedEvents {
            owner: RejectedKernelEvents,
        },
        CloseRealm {
            owner: Arc<KernelCloseSubmission>,
        },
        AcknowledgeJobProgress {
            token: JobProgressPresentationToken,
        },
        AcknowledgeTypedOperationResult {
            token: TypedOperationResultToken,
        },
    }

    impl KernelRequest {
        fn command_credits(&self) -> (usize, usize) {
            match self {
                Self::ExchangeCommands { driver, .. } => (driver.remaining_pages(), driver.remaining_bytes()),
                Self::CloseRejectedCommandBuild { owner, .. } => (owner.remaining_pages(), owner.remaining_bytes()),
                Self::CreateApp { owner } => (0, owner.remaining_bytes()),
                Self::Exchange { event, .. } => (0, event.remaining_bytes()),
                Self::AdvanceRetained { .. }
                | Self::MountProductReplay { .. }
                | Self::RetireProductReplay { .. }
                | Self::RetireProductReplayRefusal { .. }
                | Self::AdvanceProductReplay { .. }
                | Self::DestroyApp { .. }
                | Self::CloseRealm { .. }
                | Self::CloseRejectedEvents { .. }
                | Self::AcknowledgeJobProgress { .. }
                | Self::AcknowledgeTypedOperationResult { .. } => (0, 0),
            }
        }
    }

    const KERNEL_CLOSE_SUBMISSION_CAPACITY: usize = 64;
    const KERNEL_CLOSE_UNADMITTED: u8 = 0;
    const KERNEL_CLOSE_ADMITTING: u8 = 1;
    const KERNEL_CLOSE_READY: u8 = 2;
    const KERNEL_CLOSE_SCHEDULED: u8 = 3;
    const KERNEL_CLOSE_QUEUEING: u8 = 4;
    const KERNEL_CLOSE_QUEUED: u8 = 5;
    const KERNEL_CLOSE_COMPLETE: u8 = 6;
    const KERNEL_CLOSE_FAULT: u8 = 7;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) enum KernelCloseStatus {
        AdmissionBlocked,
        Pending,
        Complete,
        Fault,
    }

    struct KernelCloseSubmissionRegistry {
        slots: Mutex<[Option<(u32, u64, Arc<KernelCloseSubmission>)>; KERNEL_CLOSE_SUBMISSION_CAPACITY]>,
    }

    impl KernelCloseSubmissionRegistry {
        fn new() -> Self {
            Self { slots: Mutex::new(std::array::from_fn(|_| None)) }
        }

        fn slot(instance: u32) -> usize {
            instance as usize % KERNEL_CLOSE_SUBMISSION_CAPACITY
        }

        fn try_insert(&self, instance: u32, generation: u64, owner: Arc<KernelCloseSubmission>) -> Result<(), Arc<KernelCloseSubmission>> {
            let Ok(mut slots) = self.slots.try_lock() else {
                return Err(owner);
            };
            let slot = &mut slots[Self::slot(instance)];
            if let Some((_, _, retained)) = slot {
                if retained.terminal_status().is_some() {
                    let terminal = slot.take().expect("terminal kernel close submission is retained");
                    drop(terminal);
                }
            }
            if slot.is_some() {
                return Err(owner);
            }
            *slot = Some((instance, generation, owner));
            Ok(())
        }

        fn try_remove(&self, instance: u32, generation: u64) -> bool {
            let Ok(mut slots) = self.slots.try_lock() else {
                return false;
            };
            let slot = &mut slots[Self::slot(instance)];
            if !matches!(slot, Some((retained_instance, retained_generation, _)) if *retained_instance == instance && *retained_generation == generation) {
                return false;
            }
            let terminal = slot.take().expect("exact terminal kernel close submission is retained");
            drop(terminal);
            true
        }

        #[cfg(test)]
        fn contains(&self, instance: u32, generation: u64) -> bool {
            self.slots.try_lock().is_ok_and(|slots| matches!(&slots[Self::slot(instance)], Some((retained_instance, retained_generation, _)) if *retained_instance == instance && *retained_generation == generation))
        }
    }

    struct KernelCloseSubmission {
        instance: u32,
        realm: bool,
        generation: u64,
        queue: Arc<KernelRequestQueue>,
        pool: semio_framework_async::WorkerPool,
        registry: std::sync::Weak<KernelCloseSubmissionRegistry>,
        phase: std::sync::atomic::AtomicU8,
    }

    impl KernelCloseSubmission {
        fn terminal_status(&self) -> Option<KernelCloseStatus> {
            match self.phase.load(std::sync::atomic::Ordering::Acquire) {
                KERNEL_CLOSE_COMPLETE => Some(KernelCloseStatus::Complete),
                KERNEL_CLOSE_FAULT => Some(KernelCloseStatus::Fault),
                _ => None,
            }
        }

        fn try_admit(self: &Arc<Self>) -> bool {
            if self.phase.compare_exchange(KERNEL_CLOSE_UNADMITTED, KERNEL_CLOSE_ADMITTING, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
                return self.phase.load(std::sync::atomic::Ordering::Acquire) >= KERNEL_CLOSE_READY;
            }
            let Some(registry) = self.registry.upgrade() else {
                self.phase.store(KERNEL_CLOSE_FAULT, std::sync::atomic::Ordering::Release);
                return false;
            };
            match registry.try_insert(self.instance, self.generation, self.clone()) {
                Ok(()) => {
                    self.phase.store(KERNEL_CLOSE_READY, std::sync::atomic::Ordering::Release);
                    true
                }
                Err(rejected) => {
                    drop(rejected);
                    self.phase.store(KERNEL_CLOSE_UNADMITTED, std::sync::atomic::Ordering::Release);
                    false
                }
            }
        }

        fn try_schedule(self: &Arc<Self>) -> bool {
            if self.phase.compare_exchange(KERNEL_CLOSE_READY, KERNEL_CLOSE_SCHEDULED, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
                return false;
            }
            let scheduled = self.clone();
            let job: semio_framework_async::Job = Box::new(move || scheduled.run_queue_admission());
            match self.pool.try_submit(semio_framework_async::Lane::Maintenance, job) {
                Ok(()) => true,
                Err(error) => {
                    self.phase.compare_exchange(KERNEL_CLOSE_SCHEDULED, KERNEL_CLOSE_READY, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).ok();
                    drop(error.into_job());
                    false
                }
            }
        }

        fn run_queue_admission(self: Arc<Self>) {
            if self.phase.compare_exchange(KERNEL_CLOSE_SCHEDULED, KERNEL_CLOSE_QUEUEING, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
                return;
            }
            let wake = Waker::from(self.clone());
            self.phase.store(KERNEL_CLOSE_QUEUED, std::sync::atomic::Ordering::Release);
            let request = if self.realm { KernelRequest::CloseRealm { owner: self.clone() } } else { KernelRequest::DestroyApp { owner: self.clone() } };
            match self.queue.try_push(request, Arc::new(ResponseSlot::default()), Some(&wake)) {
                Ok(()) => {}
                Err((request, slot)) => {
                    drop(request);
                    drop(slot);
                    if self.phase.compare_exchange(KERNEL_CLOSE_QUEUED, KERNEL_CLOSE_READY, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok() {
                        let _ = self.try_schedule();
                    }
                }
            }
        }

        fn finish(&self, status: KernelCloseStatus) {
            let phase = match status {
                KernelCloseStatus::Complete => KERNEL_CLOSE_COMPLETE,
                KernelCloseStatus::Fault => KERNEL_CLOSE_FAULT,
                KernelCloseStatus::AdmissionBlocked | KernelCloseStatus::Pending => return,
            };
            self.phase.store(phase, std::sync::atomic::Ordering::Release);
            if let Some(registry) = self.registry.upgrade() {
                let _ = registry.try_remove(self.instance, self.generation);
            }
        }
    }

    impl std::task::Wake for KernelCloseSubmission {
        fn wake(self: Arc<Self>) {
            let _ = self.try_schedule();
        }

        fn wake_by_ref(self: &Arc<Self>) {
            let _ = self.try_schedule();
        }
    }

    #[must_use = "kernel close ownership must be polled through terminal completion"]
    pub(crate) struct KernelCloseHandle {
        owner: Arc<KernelCloseSubmission>,
    }

    impl KernelCloseHandle {
        pub(crate) fn poll(&self) -> KernelCloseStatus {
            if let Some(status) = self.owner.terminal_status() {
                if let Some(registry) = self.owner.registry.upgrade() {
                    let _ = registry.try_remove(self.owner.instance, self.owner.generation);
                }
                return status;
            }
            if !self.owner.try_admit() {
                return self.owner.terminal_status().unwrap_or(KernelCloseStatus::AdmissionBlocked);
            }
            let _ = self.owner.try_schedule();
            KernelCloseStatus::Pending
        }

        pub(crate) fn instance(&self) -> u32 {
            self.owner.instance
        }

        pub(crate) fn generation(&self) -> u64 {
            self.owner.generation
        }
    }

    const PRODUCT_REPLAY_KIND_BYTES: usize = 256;
    const PRODUCT_REPLAY_INPUT_PAGE_CAPACITY: usize = semio_framework_actor::JOB_REPLAY_RECORD_CAPACITY;
    const PRODUCT_REPLAY_INPUT_BYTES: usize = semio_framework_actor::JOB_REPLAY_PAGE_BYTES * PRODUCT_REPLAY_INPUT_PAGE_CAPACITY;
    const PRODUCT_REPLAY_PROFILE_COUNT: usize = 4;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum RawSpawnJobDisposition {
        Pending,
        Accepted,
        Rejected,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum RawSpawnJobClosePhase {
        Input,
        Kind,
        Terminal,
    }

    #[derive(Debug)]
    struct RawSpawnJobOwner {
        job: u64,
        kind: Option<String>,
        input: Option<Vec<u8>>,
        placement: JobPlacement,
        selected_index: usize,
        fixed_witness_acked: bool,
        mount_acked: bool,
        qualification_acked: bool,
        replay_acked: bool,
        checkpoint_ordinal: Option<u64>,
        checkpoint_digest: Option<u64>,
        terminal_ordinal: Option<u64>,
        terminal_prefix: Option<u64>,
        accepted_replay_ordinal: Option<u64>,
        disposition: RawSpawnJobDisposition,
        close_phase: RawSpawnJobClosePhase,
    }

    impl RawSpawnJobOwner {
        fn from_effect(selected_index: usize, effect: Effect) -> Self {
            let Effect::SpawnJob { job, kind, input, placement } = effect else { unreachable!("admitted product source remains SpawnJob") };
            Self {
                job,
                kind: Some(kind),
                input: Some(input),
                placement,
                selected_index,
                fixed_witness_acked: false,
                mount_acked: false,
                qualification_acked: false,
                replay_acked: false,
                checkpoint_ordinal: None,
                checkpoint_digest: None,
                terminal_ordinal: None,
                terminal_prefix: None,
                accepted_replay_ordinal: None,
                disposition: RawSpawnJobDisposition::Pending,
                close_phase: RawSpawnJobClosePhase::Input,
            }
        }

        fn kind(&self) -> &str {
            self.kind.as_deref().expect("raw product kind remains owned before retirement")
        }

        fn input(&self) -> &[u8] {
            self.input.as_deref().expect("raw product input remains owned before retirement")
        }

        fn acknowledge_fixed_witness(&mut self) {
            self.fixed_witness_acked = true;
        }

        fn acknowledge_mount(&mut self) {
            assert!(self.fixed_witness_acked && self.disposition == RawSpawnJobDisposition::Pending, "raw product mount follows exact fixed witness construction");
            self.mount_acked = true;
        }

        fn acknowledge_qualification(&mut self, checkpoint: Option<MountedProductReplayCheckpoint>) {
            assert!(self.mount_acked && self.disposition == RawSpawnJobDisposition::Pending, "raw product qualification follows accepted mount");
            self.qualification_acked = true;
            self.checkpoint_ordinal = checkpoint.map(|checkpoint| checkpoint.ordinal);
            self.checkpoint_digest = checkpoint.map(|checkpoint| checkpoint.digest);
        }

        fn acknowledge_replay_boundary(&mut self, terminal: JobReplayRecordHeader, accepted_replay_ordinal: Option<u64>) -> Result<(), &'static str> {
            if !self.fixed_witness_acked || !self.mount_acked || !self.qualification_acked || self.disposition != RawSpawnJobDisposition::Pending || accepted_replay_ordinal.is_none() {
                return Err("raw product backing cannot retire before fixed, mount, qualification, and replay acknowledgements");
            }
            self.replay_acked = true;
            self.terminal_ordinal = Some(terminal.ordinal);
            self.terminal_prefix = Some(terminal.prefix_digest);
            self.accepted_replay_ordinal = accepted_replay_ordinal;
            self.disposition = RawSpawnJobDisposition::Accepted;
            Ok(())
        }

        fn reject(&mut self) {
            if self.disposition == RawSpawnJobDisposition::Pending {
                self.disposition = RawSpawnJobDisposition::Rejected;
            }
        }

        fn observe_identity(&self) {
            let _ = (
                self.job,
                self.kind.as_deref(),
                self.input.as_deref(),
                self.placement,
                self.selected_index,
                self.fixed_witness_acked,
                self.mount_acked,
                self.qualification_acked,
                self.replay_acked,
                self.checkpoint_ordinal,
                self.checkpoint_digest,
                self.terminal_ordinal,
                self.terminal_prefix,
                self.accepted_replay_ordinal,
                self.disposition,
            );
        }

        fn close_step(&mut self) -> bool {
            self.observe_identity();
            assert!(self.disposition != RawSpawnJobDisposition::Pending, "raw product backing closes only after accepted ACK boundary or exact rejection");
            match self.close_phase {
                RawSpawnJobClosePhase::Input => {
                    drop(self.input.take().expect("raw admitted product input backing closes once"));
                    self.close_phase = RawSpawnJobClosePhase::Kind;
                }
                RawSpawnJobClosePhase::Kind => {
                    drop(self.kind.take().expect("raw admitted product kind backing closes once"));
                    self.close_phase = RawSpawnJobClosePhase::Terminal;
                }
                RawSpawnJobClosePhase::Terminal => {}
            }
            self.close_phase == RawSpawnJobClosePhase::Terminal
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct MountedProductReplayRecoveryToken {
        index: usize,
        generation: u64,
    }

    struct RetainedMountedProductReplayRequest {
        instance: u32,
        job: u64,
        job_kind: [u8; PRODUCT_REPLAY_KIND_BYTES],
        job_kind_length: u16,
        request: JobReplayRequest,
        placement: JobPlacement,
        raw: RawSpawnJobOwner,
    }

    enum MountedProductReplayRecoveryOwner {
        Request(RetainedMountedProductReplayRequest),
        Claim {
            request: RetainedMountedProductReplayRequest,
            actor: ActorId,
            record_cursor: usize,
            checkpoint: Option<MountedProductReplayCheckpoint>,
        },
        Authority {
            request: RetainedMountedProductReplayRequest,
            actor: ActorId,
            route: JobReplayRoute,
            operation: JobOperation,
            generation: u64,
            seed: u64,
            checkpoint: Option<MountedProductReplayCheckpoint>,
            terminal: JobReplayRecordHeader,
            worker_count: u16,
            worker_slot: u16,
            begin: bool,
            restore_start_ordinal: Option<u64>,
            profile_cursor: usize,
            profile_started: bool,
        },
    }

    impl MountedProductReplayRecoveryOwner {
        fn instance(&self) -> u32 {
            match self {
                Self::Request(request) | Self::Claim { request, .. } | Self::Authority { request, .. } => request.instance,
            }
        }

        fn job(&self) -> u64 {
            match self {
                Self::Request(request) | Self::Claim { request, .. } | Self::Authority { request, .. } => request.job,
            }
        }

        fn observe_close_identity(&self) {
            match self {
                Self::Request(request) => {
                    let _ = (&request.job_kind, request.job_kind_length, request.request, request.placement);
                    request.raw.observe_identity();
                }
                Self::Claim { request, actor, record_cursor, checkpoint } => {
                    let _ = (&request.job_kind, request.job_kind_length, request.request, request.placement, actor, record_cursor, checkpoint);
                    request.raw.observe_identity();
                }
                Self::Authority { request, actor, route, operation, generation, seed, checkpoint, terminal, worker_count, worker_slot, begin, restore_start_ordinal, profile_cursor, profile_started } => {
                    let _ = (
                        &request.job_kind,
                        request.job_kind_length,
                        request.request,
                        request.placement,
                        actor,
                        route,
                        operation,
                        generation,
                        seed,
                        checkpoint,
                        terminal,
                        worker_count,
                        worker_slot,
                        begin,
                        restore_start_ordinal,
                        profile_cursor,
                        profile_started,
                    );
                    request.raw.observe_identity();
                }
            }
        }

        fn close_step(&mut self) -> bool {
            self.observe_close_identity();
            match self {
                Self::Request(request) | Self::Claim { request, .. } | Self::Authority { request, .. } => request.raw.close_step(),
            }
        }
    }

    struct MountedProductReplayRecoverySlot {
        generation: u64,
        reserved: bool,
        owner: Option<MountedProductReplayRecoveryOwner>,
    }

    struct MountedProductReplayRecoveryRegistry {
        slots: [MountedProductReplayRecoverySlot; JOB_PROGRESS_ACTIVE_CAPACITY],
        abandoned: [Option<MountedProductReplayRecoveryOwner>; JOB_PROGRESS_ACTIVE_CAPACITY],
        cursor: usize,
        abandoned_cursor: usize,
    }

    impl MountedProductReplayRecoveryRegistry {
        fn new() -> Self {
            Self { slots: std::array::from_fn(|_| MountedProductReplayRecoverySlot { generation: 0, reserved: false, owner: None }), abandoned: std::array::from_fn(|_| None), cursor: 0, abandoned_cursor: 0 }
        }

        fn reserve(&mut self) -> Option<MountedProductReplayRecoveryToken> {
            let index = self.slots.iter().position(|slot| !slot.reserved && slot.owner.is_none())?;
            let generation = self.slots[index].generation.checked_add(1).filter(|generation| *generation != 0)?;
            self.slots[index].generation = generation;
            self.slots[index].reserved = true;
            Some(MountedProductReplayRecoveryToken { index, generation })
        }

        fn release(&mut self, token: MountedProductReplayRecoveryToken) -> bool {
            let Some(slot) = self.slots.get_mut(token.index) else { return false };
            if slot.generation != token.generation || !slot.reserved || slot.owner.is_some() {
                return false;
            }
            slot.reserved = false;
            true
        }

        fn publish(&mut self, token: MountedProductReplayRecoveryToken, owner: MountedProductReplayRecoveryOwner) {
            if let Some(slot) = self.slots.get_mut(token.index) {
                if slot.generation == token.generation && slot.reserved && slot.owner.is_none() {
                    slot.owner = Some(owner);
                    return;
                }
            }
            let slot = self.abandoned.iter_mut().find(|slot| slot.is_none()).expect("pre-reserved product replay recovery keeps one discoverable exact owner per shell");
            *slot = Some(owner);
        }

        fn close_one(&mut self) -> bool {
            if let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.slots[*index].owner.is_some()) {
                self.cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
                if self.slots[index].owner.as_mut().expect("selected product replay recovery owner").close_step() {
                    let terminal = self.slots[index].owner.take().expect("terminal product replay recovery owner");
                    terminal.observe_close_identity();
                    drop(terminal);
                    self.slots[index].reserved = false;
                }
                return true;
            }
            let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.abandoned_cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.abandoned[*index].is_some()) else {
                return false;
            };
            self.abandoned_cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
            if self.abandoned[index].as_mut().expect("selected abandoned product replay owner").close_step() {
                let terminal = self.abandoned[index].take().expect("terminal abandoned product replay owner");
                terminal.observe_close_identity();
                drop(terminal);
            }
            true
        }

        fn has_close_work(&self) -> bool {
            self.slots.iter().any(|slot| slot.owner.is_some()) || self.abandoned.iter().any(Option::is_some)
        }

        fn has_instance(&self, instance: u32) -> bool {
            self.slots.iter().filter_map(|slot| slot.owner.as_ref()).chain(self.abandoned.iter().flatten()).any(|owner| owner.instance() == instance)
        }

        fn close_instance_one(&mut self, instance: u32) -> bool {
            if let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.slots[*index].owner.as_ref().is_some_and(|owner| owner.instance() == instance)) {
                self.cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
                if self.slots[index].owner.as_mut().expect("selected instance product replay recovery owner").close_step() {
                    let terminal = self.slots[index].owner.take().expect("terminal instance product replay recovery owner");
                    terminal.observe_close_identity();
                    drop(terminal);
                    self.slots[index].reserved = false;
                }
                return true;
            }
            let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.abandoned_cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.abandoned[*index].as_ref().is_some_and(|owner| owner.instance() == instance)) else {
                return false;
            };
            self.abandoned_cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
            if self.abandoned[index].as_mut().expect("selected abandoned instance product replay owner").close_step() {
                let terminal = self.abandoned[index].take().expect("terminal abandoned instance product replay owner");
                terminal.observe_close_identity();
                drop(terminal);
            }
            true
        }
    }

    fn mounted_product_replay_recovery_registry() -> &'static Mutex<MountedProductReplayRecoveryRegistry> {
        static REGISTRY: OnceLock<Mutex<MountedProductReplayRecoveryRegistry>> = OnceLock::new();
        REGISTRY.get_or_init(|| Mutex::new(MountedProductReplayRecoveryRegistry::new()))
    }

    /// 🎫️ Exact typed product owner moved from one successful mounted-session exchange.
    #[derive(Debug)]
    pub(crate) struct MountedProductReplayRequest {
        instance: u32,
        job: u64,
        job_kind: [u8; PRODUCT_REPLAY_KIND_BYTES],
        job_kind_length: u16,
        request: JobReplayRequest,
        placement: JobPlacement,
        raw: Option<RawSpawnJobOwner>,
        recovery: Option<MountedProductReplayRecoveryToken>,
    }

    impl MountedProductReplayRequest {
        fn from_admitted_effect(instance: u32, selected_index: usize, effect: Effect, recovery: MountedProductReplayRecoveryToken) -> Self {
            let mut raw = RawSpawnJobOwner::from_effect(selected_index, effect);
            let request = JobReplayRequest::from_spawn(raw.kind(), raw.input());
            let mut job_kind = [0; PRODUCT_REPLAY_KIND_BYTES];
            job_kind[..raw.kind().len()].copy_from_slice(raw.kind().as_bytes());
            raw.acknowledge_fixed_witness();
            Self { instance, job: raw.job, job_kind, job_kind_length: raw.kind().len() as u16, request, placement: raw.placement, raw: Some(raw), recovery: Some(recovery) }
        }

        fn job_kind(&self) -> &[u8] {
            &self.job_kind[..usize::from(self.job_kind_length)]
        }

        pub(crate) fn rejection_reason(&self) -> String {
            format!("mounted product replay authority rejected for instance {}, job {}", self.instance, self.job)
        }

        fn raw(&self) -> &RawSpawnJobOwner {
            self.raw.as_ref().expect("mounted product replay request retains exact raw SpawnJob")
        }

        fn raw_mut(&mut self) -> &mut RawSpawnJobOwner {
            self.raw.as_mut().expect("mounted product replay request retains exact mutable raw SpawnJob")
        }

        fn into_retained(mut self) -> (MountedProductReplayRecoveryToken, RetainedMountedProductReplayRequest) {
            let recovery = self.recovery.take().expect("mounted product replay recovery authority transfers once");
            let raw = self.raw.take().expect("mounted product replay raw SpawnJob transfers once");
            let retained = RetainedMountedProductReplayRequest { instance: self.instance, job: self.job, job_kind: self.job_kind, job_kind_length: self.job_kind_length, request: self.request, placement: self.placement, raw };
            (recovery, retained)
        }

        pub(crate) fn retire(mut self) {
            self.raw_mut().reject();
            let (recovery, retained) = self.into_retained();
            mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").publish(recovery, MountedProductReplayRecoveryOwner::Request(retained));
        }
    }

    impl Drop for MountedProductReplayRequest {
        fn drop(&mut self) {
            let Some(recovery) = self.recovery.take() else { return };
            let Some(mut raw) = self.raw.take() else { return };
            raw.reject();
            let retained = RetainedMountedProductReplayRequest { instance: self.instance, job: self.job, job_kind: self.job_kind, job_kind_length: self.job_kind_length, request: self.request, placement: self.placement, raw };
            mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").publish(recovery, MountedProductReplayRecoveryOwner::Request(retained));
        }
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct MountedProductReplayRefusalToken {
        index: usize,
        generation: u64,
        instance: u32,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) enum MountedProductReplayRefusalCause {
        KindCapacity,
        InputCapacity,
        RecoveryCapacity,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum MountedProductReplayRefusalPhase {
        Retry,
        SplitSpawn,
        Input,
        Kind,
        RemainingEffects,
        RemainingBacking,
        Terminal,
    }

    struct RetainedRefusedProductReplay {
        instance: u32,
        cause: MountedProductReplayRefusalCause,
        selected_index: usize,
        spawn: Option<Effect>,
        spawn_job: Option<u64>,
        spawn_placement: Option<JobPlacement>,
        spawn_kind: Option<String>,
        spawn_input: Option<Vec<u8>>,
        remaining_effects: Option<Vec<Effect>>,
        phase: MountedProductReplayRefusalPhase,
    }

    impl RetainedRefusedProductReplay {
        fn observe_identity(&self) {
            let _ = (self.instance, self.cause, self.selected_index, self.spawn_job, self.spawn_placement);
            if let Some(Effect::SpawnJob { job, kind, input, placement }) = self.spawn.as_ref() {
                let _ = (job, kind.as_bytes(), input.as_slice(), placement);
            }
        }

        fn retry(&mut self) -> Option<MountedProductReplayRequest> {
            if self.phase != MountedProductReplayRefusalPhase::Retry {
                return None;
            }
            let recovery = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").reserve()?;
            let Some(spawn) = self.spawn.take() else {
                self.phase = MountedProductReplayRefusalPhase::SplitSpawn;
                let _ = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").release(recovery);
                return None;
            };
            let request = MountedProductReplayRequest::from_admitted_effect(self.instance, self.selected_index, spawn, recovery);
            self.phase = MountedProductReplayRefusalPhase::RemainingEffects;
            Some(request)
        }

        fn close_step(&mut self) -> bool {
            self.observe_identity();
            match self.phase {
                MountedProductReplayRefusalPhase::Retry => {
                    self.phase = MountedProductReplayRefusalPhase::SplitSpawn;
                }
                MountedProductReplayRefusalPhase::SplitSpawn => {
                    let Some(effect) = self.spawn.take() else {
                        self.phase = MountedProductReplayRefusalPhase::RemainingEffects;
                        return false;
                    };
                    let Effect::SpawnJob { job, kind, input, placement } = effect else { unreachable!("product replay refusal retains the selected SpawnJob") };
                    self.spawn_job = Some(job);
                    self.spawn_placement = Some(placement);
                    self.spawn_kind = Some(kind);
                    self.spawn_input = Some(input);
                    self.phase = MountedProductReplayRefusalPhase::Input;
                }
                MountedProductReplayRefusalPhase::Input => {
                    drop(self.spawn_input.take().expect("refused product input backing closes once"));
                    self.phase = MountedProductReplayRefusalPhase::Kind;
                }
                MountedProductReplayRefusalPhase::Kind => {
                    drop(self.spawn_kind.take().expect("refused product kind backing closes once"));
                    self.phase = MountedProductReplayRefusalPhase::RemainingEffects;
                }
                MountedProductReplayRefusalPhase::RemainingEffects => {
                    let remaining = self.remaining_effects.as_mut().expect("refused product outcome remainder is retained");
                    if let Some(effect) = remaining.pop() {
                        drop(effect);
                    } else {
                        self.phase = MountedProductReplayRefusalPhase::RemainingBacking;
                    }
                }
                MountedProductReplayRefusalPhase::RemainingBacking => {
                    drop(self.remaining_effects.take().expect("refused product outcome backing closes once"));
                    self.phase = MountedProductReplayRefusalPhase::Terminal;
                }
                MountedProductReplayRefusalPhase::Terminal => {}
            }
            self.phase == MountedProductReplayRefusalPhase::Terminal
        }
    }

    struct MountedProductReplayRefusalSlot {
        generation: u64,
        instance: Option<u32>,
        reserved: bool,
        owner: Option<RetainedRefusedProductReplay>,
    }

    struct MountedProductReplayRefusalRegistry {
        slots: [MountedProductReplayRefusalSlot; JOB_PROGRESS_ACTIVE_CAPACITY],
        abandoned: [Option<RetainedRefusedProductReplay>; JOB_PROGRESS_ACTIVE_CAPACITY],
        cursor: usize,
        abandoned_cursor: usize,
    }

    struct MountedProductReplayRefusalStep {
        request: Option<MountedProductReplayRequest>,
        progressed: bool,
    }

    impl MountedProductReplayRefusalRegistry {
        fn new() -> Self {
            Self { slots: std::array::from_fn(|_| MountedProductReplayRefusalSlot { generation: 0, instance: None, reserved: false, owner: None }), abandoned: std::array::from_fn(|_| None), cursor: 0, abandoned_cursor: 0 }
        }

        fn reserve(&mut self, instance: u32) -> Option<MountedProductReplayRefusalToken> {
            let index = self.slots.iter().position(|slot| !slot.reserved && slot.owner.is_none())?;
            let generation = self.slots[index].generation.checked_add(1).filter(|generation| *generation != 0)?;
            self.slots[index].generation = generation;
            self.slots[index].instance = Some(instance);
            self.slots[index].reserved = true;
            Some(MountedProductReplayRefusalToken { index, generation, instance })
        }

        fn release(&mut self, token: MountedProductReplayRefusalToken) -> bool {
            let Some(slot) = self.slots.get_mut(token.index) else { return false };
            if slot.generation != token.generation || slot.instance != Some(token.instance) || !slot.reserved || slot.owner.is_some() {
                return false;
            }
            slot.instance = None;
            slot.reserved = false;
            true
        }

        fn publish(&mut self, token: MountedProductReplayRefusalToken, owner: RetainedRefusedProductReplay) {
            if let Some(slot) = self.slots.get_mut(token.index) {
                if slot.generation == token.generation && slot.instance == Some(owner.instance) && slot.reserved && slot.owner.is_none() {
                    slot.owner = Some(owner);
                    return;
                }
            }
            let slot = self.abandoned.iter_mut().find(|slot| slot.is_none()).expect("pre-reserved refusal authority keeps one discoverable product owner per bridge call");
            *slot = Some(owner);
        }

        fn step_one(&mut self, retry: bool) -> MountedProductReplayRefusalStep {
            if let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.slots[*index].owner.is_some()) {
                self.cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
                let owner = self.slots[index].owner.as_mut().expect("selected product refusal owner");
                if retry {
                    if let Some(request) = owner.retry() {
                        return MountedProductReplayRefusalStep { request: Some(request), progressed: true };
                    }
                    if owner.phase == MountedProductReplayRefusalPhase::Retry {
                        return MountedProductReplayRefusalStep { request: None, progressed: true };
                    }
                }
                let terminal = owner.close_step();
                if terminal {
                    let terminal = self.slots[index].owner.take().expect("terminal product refusal owner");
                    assert_eq!(terminal.phase, MountedProductReplayRefusalPhase::Terminal, "product refusal terminal identity changed before release");
                    self.slots[index].instance = None;
                    self.slots[index].reserved = false;
                    drop(terminal);
                }
                return MountedProductReplayRefusalStep { request: None, progressed: true };
            }
            let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.abandoned_cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.abandoned[*index].is_some()) else {
                return MountedProductReplayRefusalStep { request: None, progressed: false };
            };
            self.abandoned_cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
            let owner = self.abandoned[index].as_mut().expect("selected abandoned product refusal owner");
            let terminal = owner.close_step();
            if terminal {
                drop(self.abandoned[index].take().expect("terminal abandoned product refusal owner"));
            }
            MountedProductReplayRefusalStep { request: None, progressed: true }
        }

        fn has_work(&self) -> bool {
            self.slots.iter().any(|slot| slot.owner.is_some()) || self.abandoned.iter().any(Option::is_some)
        }

        fn has_any(&self) -> bool {
            self.slots.iter().any(|slot| slot.reserved || slot.owner.is_some()) || self.abandoned.iter().any(Option::is_some)
        }

        fn has_instance(&self, instance: u32) -> bool {
            self.slots.iter().any(|slot| slot.instance == Some(instance)) || self.abandoned.iter().flatten().any(|owner| owner.instance == instance)
        }

        fn close_instance_one(&mut self, instance: u32) -> bool {
            if let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.slots[*index].owner.as_ref().is_some_and(|owner| owner.instance == instance)) {
                self.cursor = index;
                let _ = self.step_one(false);
                return true;
            }
            if let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.abandoned_cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.abandoned[*index].as_ref().is_some_and(|owner| owner.instance == instance)) {
                self.abandoned_cursor = index;
                let _ = self.step_one(false);
                return true;
            }
            false
        }
    }

    fn mounted_product_replay_refusal_registry() -> &'static Mutex<MountedProductReplayRefusalRegistry> {
        static REGISTRY: OnceLock<Mutex<MountedProductReplayRefusalRegistry>> = OnceLock::new();
        REGISTRY.get_or_init(|| Mutex::new(MountedProductReplayRefusalRegistry::new()))
    }

    pub(crate) struct MountedProductReplayAdmissionPermit {
        token: Option<MountedProductReplayRefusalToken>,
    }

    impl MountedProductReplayAdmissionPermit {
        fn release(mut self) {
            let token = self.token.take().expect("product admission permit releases once");
            assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").release(token), "product admission permit release keeps exact generation");
        }

        fn refuse(mut self, cause: MountedProductReplayRefusalCause, selected_index: usize, spawn: Effect, remaining_effects: Vec<Effect>) -> RefusedProductReplay {
            let token = self.token.take().expect("product admission permit transfers once");
            RefusedProductReplay { token: Some(token), instance: token.instance, cause, selected_index, spawn: Some(spawn), remaining_effects: Some(remaining_effects) }
        }
    }

    impl Drop for MountedProductReplayAdmissionPermit {
        fn drop(&mut self) {
            let Some(token) = self.token.take() else { return };
            let _ = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").release(token);
        }
    }

    pub(crate) struct RefusedProductReplay {
        token: Option<MountedProductReplayRefusalToken>,
        instance: u32,
        cause: MountedProductReplayRefusalCause,
        selected_index: usize,
        spawn: Option<Effect>,
        remaining_effects: Option<Vec<Effect>>,
    }

    impl RefusedProductReplay {
        pub(crate) fn rejection_reason(&self) -> &'static str {
            match self.cause {
                MountedProductReplayRefusalCause::KindCapacity => "mounted product replay job kind exceeds its fixed authority",
                MountedProductReplayRefusalCause::InputCapacity => "mounted product replay input exceeds its fixed page authority",
                MountedProductReplayRefusalCause::RecoveryCapacity => "mounted product replay recovery capacity is full",
            }
        }

        fn into_retained(mut self) -> (MountedProductReplayRefusalToken, RetainedRefusedProductReplay) {
            let token = self.token.take().expect("product refusal recovery authority transfers once");
            let phase = if self.cause == MountedProductReplayRefusalCause::RecoveryCapacity { MountedProductReplayRefusalPhase::Retry } else { MountedProductReplayRefusalPhase::SplitSpawn };
            let retained = RetainedRefusedProductReplay {
                instance: self.instance,
                cause: self.cause,
                selected_index: self.selected_index,
                spawn: self.spawn.take(),
                spawn_job: None,
                spawn_placement: None,
                spawn_kind: None,
                spawn_input: None,
                remaining_effects: self.remaining_effects.take(),
                phase,
            };
            (token, retained)
        }

        pub(crate) fn retire(self) {
            let (token, retained) = self.into_retained();
            mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").publish(token, retained);
        }
    }

    impl Drop for RefusedProductReplay {
        fn drop(&mut self) {
            let Some(token) = self.token.take() else { return };
            let retained = RetainedRefusedProductReplay {
                instance: self.instance,
                cause: self.cause,
                selected_index: self.selected_index,
                spawn: self.spawn.take(),
                spawn_job: None,
                spawn_placement: None,
                spawn_kind: None,
                spawn_input: None,
                remaining_effects: self.remaining_effects.take(),
                phase: if self.cause == MountedProductReplayRefusalCause::RecoveryCapacity { MountedProductReplayRefusalPhase::Retry } else { MountedProductReplayRefusalPhase::SplitSpawn },
            };
            mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").publish(token, retained);
        }
    }

    pub(crate) enum MountedProductReplayAdmission {
        None,
        Admitted(MountedProductReplayRequest),
        Refused(RefusedProductReplay),
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct MountedProductReplayCheckpoint {
        ordinal: u64,
        digest: u64,
        pages: u8,
        applied_progress: u64,
    }

    struct MountedProductReplayClaim {
        request: Option<MountedProductReplayRequest>,
        actor: ActorId,
        record_cursor: usize,
        checkpoint: Option<MountedProductReplayCheckpoint>,
    }

    impl MountedProductReplayClaim {
        fn request(&self) -> &MountedProductReplayRequest {
            self.request.as_ref().expect("mounted product replay claim retains request")
        }

        fn instance(&self) -> u32 {
            self.request().instance
        }

        fn job(&self) -> u64 {
            self.request().job
        }

        fn retire(mut self) {
            let mut request = self.request.take().expect("mounted product replay claim retires once");
            request.raw_mut().reject();
            let (recovery, request) = request.into_retained();
            mounted_product_replay_recovery_registry()
                .lock()
                .expect("product replay recovery lock")
                .publish(recovery, MountedProductReplayRecoveryOwner::Claim { request, actor: self.actor, record_cursor: self.record_cursor, checkpoint: self.checkpoint });
        }
    }

    impl Drop for MountedProductReplayClaim {
        fn drop(&mut self) {
            let Some(mut request) = self.request.take() else { return };
            request.raw_mut().reject();
            let (recovery, request) = request.into_retained();
            mounted_product_replay_recovery_registry()
                .lock()
                .expect("product replay recovery lock")
                .publish(recovery, MountedProductReplayRecoveryOwner::Claim { request, actor: self.actor, record_cursor: self.record_cursor, checkpoint: self.checkpoint });
        }
    }

    #[derive(Clone, Copy)]
    struct MountedProductReplayExpected {
        instance: u32,
        actor: ActorId,
        turn: JobTurn,
        request: JobReplayRequest,
        placement: JobPlacement,
        route: JobReplayRoute,
        checkpoint: Option<MountedProductReplayCheckpoint>,
        terminal: JobReplayRecordHeader,
        process_worker_count: u16,
        process_worker_slot: u16,
        logical_worker_count: u16,
        logical_worker_slot: u16,
        begin: bool,
        restore_start_ordinal: Option<u64>,
    }

    /// 🪪️ Full mounted identity qualified against the retained shard log before replay.
    #[derive(Debug)]
    struct MountedProductReplayAuthority {
        request_owner: Option<MountedProductReplayRequest>,
        actor: ActorId,
        route: JobReplayRoute,
        operation: JobOperation,
        generation: u64,
        seed: u64,
        checkpoint: Option<MountedProductReplayCheckpoint>,
        terminal: JobReplayRecordHeader,
        worker_count: u16,
        worker_slot: u16,
        begin: bool,
        restore_start_ordinal: Option<u64>,
        profile_cursor: usize,
        profile_started: bool,
    }

    impl MountedProductReplayAuthority {
        fn request_owner(&self) -> &MountedProductReplayRequest {
            self.request_owner.as_ref().expect("mounted product replay authority retains request")
        }

        fn request_owner_mut(&mut self) -> &mut MountedProductReplayRequest {
            self.request_owner.as_mut().expect("mounted product replay authority retains mutable request")
        }

        fn instance(&self) -> u32 {
            self.request_owner().instance
        }

        fn job(&self) -> u64 {
            self.request_owner().job
        }

        fn validate(self, expected: MountedProductReplayExpected) -> Result<Self, Self> {
            let request_owner = self.request_owner();
            let kind = std::str::from_utf8(request_owner.job_kind()).ok();
            let kind_identity = kind.map(|kind| JobReplayRequest::from_spawn(kind, &[]));
            let raw_request = JobReplayRequest::from_spawn(request_owner.raw().kind(), request_owner.raw().input());
            let valid = request_owner.instance == expected.instance
                && request_owner.job == expected.turn.job
                && request_owner.raw().job == request_owner.job
                && request_owner.raw().kind().as_bytes() == request_owner.job_kind()
                && raw_request == request_owner.request
                && request_owner.raw().placement == request_owner.placement
                && kind_identity.is_some_and(|kind| kind.controller == expected.request.controller && kind.tool == expected.request.tool && kind.schema == expected.request.schema && kind.version == expected.request.version)
                && request_owner.request == expected.request
                && request_owner.placement == expected.placement
                && self.actor == expected.actor
                && self.operation == expected.turn.operation
                && self.generation == expected.turn.operation.generation
                && self.seed == expected.turn.operation.seed
                && self.route == expected.route
                && self.checkpoint == expected.checkpoint
                && self.terminal == expected.terminal
                && expected.terminal.worker_count == expected.process_worker_count
                && expected.terminal.worker_slot == expected.process_worker_slot
                && self.worker_count == expected.logical_worker_count
                && self.worker_slot == expected.logical_worker_slot
                && self.begin == expected.begin
                && self.restore_start_ordinal == expected.restore_start_ordinal;
            if valid {
                Ok(self)
            } else {
                Err(self)
            }
        }

        fn acknowledge_raw_retirement_boundary(&mut self) -> Result<(), &'static str> {
            if self.profile_cursor != PRODUCT_REPLAY_PROFILE_COUNT || self.profile_started {
                return Err("raw product retirement requires every logical replay profile to reach terminal ACK");
            }
            let terminal = self.terminal;
            let accepted_replay_ordinal = self.restore_start_ordinal;
            self.request_owner_mut().raw_mut().acknowledge_replay_boundary(terminal, accepted_replay_ordinal)
        }

        fn retire(mut self) {
            let mut request = self.request_owner.take().expect("mounted product replay authority retires once");
            request.raw_mut().reject();
            let (recovery, request) = request.into_retained();
            mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").publish(
                recovery,
                MountedProductReplayRecoveryOwner::Authority {
                    request,
                    actor: self.actor,
                    route: self.route,
                    operation: self.operation,
                    generation: self.generation,
                    seed: self.seed,
                    checkpoint: self.checkpoint,
                    terminal: self.terminal,
                    worker_count: self.worker_count,
                    worker_slot: self.worker_slot,
                    begin: self.begin,
                    restore_start_ordinal: self.restore_start_ordinal,
                    profile_cursor: self.profile_cursor,
                    profile_started: self.profile_started,
                },
            );
        }
    }

    impl Drop for MountedProductReplayAuthority {
        fn drop(&mut self) {
            let Some(mut request) = self.request_owner.take() else { return };
            request.raw_mut().reject();
            let (recovery, request) = request.into_retained();
            mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").publish(
                recovery,
                MountedProductReplayRecoveryOwner::Authority {
                    request,
                    actor: self.actor,
                    route: self.route,
                    operation: self.operation,
                    generation: self.generation,
                    seed: self.seed,
                    checkpoint: self.checkpoint,
                    terminal: self.terminal,
                    worker_count: self.worker_count,
                    worker_slot: self.worker_slot,
                    begin: self.begin,
                    restore_start_ordinal: self.restore_start_ordinal,
                    profile_cursor: self.profile_cursor,
                    profile_started: self.profile_started,
                },
            );
        }
    }

    fn mounted_product_replay_profile(process_default: u16, cursor: usize) -> Option<u16> {
        [1, 2, 4, process_default].get(cursor).copied().filter(|worker_count| *worker_count != 0)
    }

    fn mounted_product_replay_worker_slot(process_slot: u16, worker_count: u16) -> Option<u16> {
        (worker_count != 0 && process_slot != u16::MAX).then_some(process_slot % worker_count)
    }

    //#region 📬️TypedOperationResultExchange
    pub(crate) const TYPED_OPERATION_RESULT_PAGE_BYTES: usize = 4_096;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) struct TypedOperationResultToken {
        pub receiver: u32,
        pub operation: u64,
        pub generation: u64,
        pub sequence: u32,
        pub attempt: u8,
    }

    #[derive(Clone)]
    pub(crate) struct TypedOperationResultPage {
        pub token: TypedOperationResultToken,
        pub lane: u8,
        bytes: [u8; TYPED_OPERATION_RESULT_PAGE_BYTES],
        len: usize,
    }

    impl TypedOperationResultPage {
        const PAGE_MAGIC: &'static [u8] = b"semio.typed-operation-page.v1\0";
        const ACK_MAGIC: &'static [u8] = b"semio.typed-operation-ack.v1\0";

        pub(crate) fn try_copy_from(token: TypedOperationResultToken, lane: u8, bytes: &[u8]) -> Result<Self, &'static str> {
            if bytes.len() > TYPED_OPERATION_RESULT_PAGE_BYTES {
                return Err("typed-operation result exceeds renderer fixed page authority");
            }
            let mut page = Self { token, lane, bytes: [0; TYPED_OPERATION_RESULT_PAGE_BYTES], len: bytes.len() };
            page.bytes[..bytes.len()].copy_from_slice(bytes);
            Ok(page)
        }

        pub(crate) fn bytes(&self) -> &[u8] {
            &self.bytes[..self.len]
        }

        fn decode_guest_message(bytes: &[u8]) -> Option<Self> {
            let body = bytes.strip_prefix(Self::PAGE_MAGIC)?;
            if body.len() < 30 {
                return None;
            }
            let token = TypedOperationResultToken {
                receiver: u32::from_le_bytes(body[0..4].try_into().ok()?),
                operation: u64::from_le_bytes(body[4..12].try_into().ok()?),
                generation: u64::from_le_bytes(body[12..20].try_into().ok()?),
                sequence: u32::from_le_bytes(body[20..24].try_into().ok()?),
                attempt: body[24],
            };
            let lane = body[25];
            if lane > 11 {
                return None;
            }
            let len = u32::from_le_bytes(body[26..30].try_into().ok()?) as usize;
            if len > TYPED_OPERATION_RESULT_PAGE_BYTES || body.len() != 30 + len {
                return None;
            }
            Self::try_copy_from(token, lane, &body[30..]).ok()
        }

        fn encode_ack(token: TypedOperationResultToken) -> Vec<u8> {
            let mut bytes = Vec::with_capacity(Self::ACK_MAGIC.len() + 25);
            bytes.extend_from_slice(Self::ACK_MAGIC);
            bytes.extend_from_slice(&token.receiver.to_le_bytes());
            bytes.extend_from_slice(&token.operation.to_le_bytes());
            bytes.extend_from_slice(&token.generation.to_le_bytes());
            bytes.extend_from_slice(&token.sequence.to_le_bytes());
            bytes.push(token.attempt);
            bytes
        }
    }

    /// 🎯️ Object-safe renderer boundary for one retained page and its exact ACK token.
    /// The installed plugin host keeps the original owner until `acknowledge` succeeds.
    pub(crate) trait TypedOperationResultExchange: Send + Sync {
        fn take_page(&self, receiver: u32) -> Option<TypedOperationResultPage>;
        fn acknowledge(&self, token: TypedOperationResultToken) -> bool;
    }

    struct MountedTypedOperationResultPage {
        page: TypedOperationResultPage,
        acknowledge: Arc<dyn Fn(TypedOperationResultToken) -> bool + Send + Sync>,
    }

    struct MountedTypedOperationResultExchange {
        pages: Mutex<[Option<MountedTypedOperationResultPage>; 64]>,
    }

    impl MountedTypedOperationResultExchange {
        fn new() -> Self {
            Self { pages: Mutex::new(std::array::from_fn(|_| None)) }
        }

        fn publish(&self, page: TypedOperationResultPage, acknowledge: Arc<dyn Fn(TypedOperationResultToken) -> bool + Send + Sync>) -> Result<(), TypedOperationResultPage> {
            let mut pages = self.pages.lock().expect("typed-operation renderer exchange lock");
            if let Some(pending) = pages.iter_mut().flatten().find(|pending| {
                pending.page.token.receiver == page.token.receiver && pending.page.token.operation == page.token.operation && pending.page.token.generation == page.token.generation && pending.page.token.sequence == page.token.sequence
            }) {
                if page.token.attempt >= pending.page.token.attempt {
                    *pending = MountedTypedOperationResultPage { page, acknowledge };
                }
                return Ok(());
            }
            let Some(slot) = pages.iter_mut().find(|slot| slot.is_none()) else { return Err(page) };
            *slot = Some(MountedTypedOperationResultPage { page, acknowledge });
            Ok(())
        }
    }

    impl TypedOperationResultExchange for MountedTypedOperationResultExchange {
        fn take_page(&self, receiver: u32) -> Option<TypedOperationResultPage> {
            self.pages.lock().expect("typed-operation renderer exchange lock").iter().flatten().find(|pending| pending.page.token.receiver == receiver).map(|pending| pending.page.clone())
        }

        fn acknowledge(&self, token: TypedOperationResultToken) -> bool {
            let mut pages = self.pages.lock().expect("typed-operation renderer exchange lock");
            let Some(index) = pages.iter().position(|pending| pending.as_ref().is_some_and(|pending| pending.page.token == token)) else { return false };
            let accepted = pages[index].as_ref().is_some_and(|pending| (pending.acknowledge)(token));
            if accepted {
                pages[index] = None;
            }
            accepted
        }
    }

    fn typed_operation_result_exchange() -> &'static OnceLock<Arc<dyn TypedOperationResultExchange>> {
        static EXCHANGE: OnceLock<Arc<dyn TypedOperationResultExchange>> = OnceLock::new();
        &EXCHANGE
    }

    pub(crate) fn install_typed_operation_result_exchange(exchange: Arc<dyn TypedOperationResultExchange>) -> Result<(), Arc<dyn TypedOperationResultExchange>> {
        typed_operation_result_exchange().set(exchange)
    }

    fn mounted_typed_operation_result_exchange() -> &'static Arc<MountedTypedOperationResultExchange> {
        static EXCHANGE: OnceLock<Arc<MountedTypedOperationResultExchange>> = OnceLock::new();
        EXCHANGE.get_or_init(|| Arc::new(MountedTypedOperationResultExchange::new()))
    }

    fn install_mounted_typed_operation_result_exchange() {
        let exchange: Arc<dyn TypedOperationResultExchange> = mounted_typed_operation_result_exchange().clone();
        let _ = install_typed_operation_result_exchange(exchange);
    }

    pub(crate) fn publish_typed_operation_result_page(page: TypedOperationResultPage, acknowledge: Arc<dyn Fn(TypedOperationResultToken) -> bool + Send + Sync>) -> Result<(), TypedOperationResultPage> {
        install_mounted_typed_operation_result_exchange();
        mounted_typed_operation_result_exchange().publish(page, acknowledge)
    }

    #[cfg(test)]
    mod typed_operation_result_exchange_tests {
        use super::*;
        use std::sync::atomic::{AtomicUsize, Ordering};

        #[test]
        fn concrete_exchange_is_installed_and_retains_the_page_until_the_receiver_accepts_the_exact_ack() {
            install_mounted_typed_operation_result_exchange();
            assert!(typed_operation_result_exchange().get().is_some());

            let exchange = MountedTypedOperationResultExchange::new();
            let token = TypedOperationResultToken { receiver: u32::MAX, operation: u64::MAX, generation: u64::MAX - 1, sequence: u32::MAX, attempt: u8::MAX };
            let page = TypedOperationResultPage::try_copy_from(token, u8::MAX, b"retained-page").expect("bounded typed-operation page");
            let attempts = Arc::new(AtomicUsize::new(0));
            let callback_attempts = attempts.clone();
            let published = exchange.publish(page, Arc::new(move |acknowledged| acknowledged == token && callback_attempts.fetch_add(1, Ordering::SeqCst) > 0));
            assert!(published.is_ok(), "first retained page slot");

            let wrong = TypedOperationResultToken { sequence: token.sequence - 1, ..token };
            assert!(!exchange.acknowledge(wrong));
            assert!(!exchange.acknowledge(token));
            assert_eq!(exchange.take_page(token.receiver).expect("rejected ACK retains the exact page").bytes(), b"retained-page");
            assert!(exchange.acknowledge(token));
            assert!(exchange.take_page(token.receiver).is_none());
            assert_eq!(attempts.load(Ordering::SeqCst), 2);
        }

        #[test]
        fn renderer_wire_maximum_plus_one_and_exchange_saturation_preserve_exact_owners() {
            let token = TypedOperationResultToken { receiver: 7, operation: 11, generation: 13, sequence: 17, attempt: 1 };
            assert!(TypedOperationResultPage::try_copy_from(token, 0, &[0; TYPED_OPERATION_RESULT_PAGE_BYTES]).is_ok());
            assert!(TypedOperationResultPage::try_copy_from(token, 0, &[0; TYPED_OPERATION_RESULT_PAGE_BYTES + 1]).is_err());
            let mut wire = Vec::from(TypedOperationResultPage::PAGE_MAGIC);
            wire.extend_from_slice(&token.receiver.to_le_bytes());
            wire.extend_from_slice(&token.operation.to_le_bytes());
            wire.extend_from_slice(&token.generation.to_le_bytes());
            wire.extend_from_slice(&token.sequence.to_le_bytes());
            wire.push(token.attempt);
            wire.push(0);
            wire.extend_from_slice(&(TYPED_OPERATION_RESULT_PAGE_BYTES as u32).to_le_bytes());
            wire.extend_from_slice(&[0x2a; TYPED_OPERATION_RESULT_PAGE_BYTES]);
            assert_eq!(TypedOperationResultPage::decode_guest_message(&wire).expect("exact maximum guest page").bytes(), &[0x2a; TYPED_OPERATION_RESULT_PAGE_BYTES]);
            wire.push(0);
            assert!(TypedOperationResultPage::decode_guest_message(&wire).is_none(), "maximum plus one wire byte is rejected");

            let exchange = MountedTypedOperationResultExchange::new();
            for sequence in 0..64 {
                let token = TypedOperationResultToken { sequence, ..token };
                let page = TypedOperationResultPage::try_copy_from(token, 0, &[sequence as u8]).expect("bounded saturation page");
                assert!(exchange.publish(page, Arc::new(|_| true)).is_ok());
            }
            let overflow_token = TypedOperationResultToken { sequence: 64, ..token };
            let overflow = TypedOperationResultPage::try_copy_from(overflow_token, 0, b"overflow").expect("bounded overflow owner");
            let returned = match exchange.publish(overflow, Arc::new(|_| true)) {
                Ok(()) => panic!("maximum plus one page must be returned"),
                Err(returned) => returned,
            };
            assert_eq!(returned.token, overflow_token);
            assert_eq!(returned.bytes(), b"overflow");

            let retries = MountedTypedOperationResultExchange::new();
            let first = TypedOperationResultPage::try_copy_from(token, 0, b"first").expect("first attempt");
            retries.publish(first, Arc::new(|_| true)).unwrap_or_else(|_| panic!("first attempt slot"));
            let latest_token = TypedOperationResultToken { attempt: 2, ..token };
            let latest = TypedOperationResultPage::try_copy_from(latest_token, 0, b"latest").expect("latest attempt");
            retries.publish(latest, Arc::new(|_| true)).unwrap_or_else(|_| panic!("latest attempt replaces stable identity"));
            assert!(!retries.acknowledge(token), "stale attempt must not consume the replacement owner");
            assert_eq!(retries.take_page(token.receiver).expect("latest owner retained").token, latest_token);
        }
    }
    //#endregion 📬️TypedOperationResultExchange

    pub(crate) struct ExchangeSurfaceDocument {
        pub surface: SurfaceId,
        pub document: UiDocumentLease,
    }

    pub(crate) struct ExchangeOutcome {
        pub frames: Vec<protocol::AppFrame>,
        /// 🖼️ Surfaces this turn repainted or retained on desync — reconciled against the kernel
        /// pool state machine's retained tree (`KernelPoolState::retained`); see that field's doc for the
        /// full-body-vs-desync policy.
        pub surfaces: UiFixedList<ExchangeSurfaceDocument, UI_DOCUMENT_LEASE_SLOTS>,
        /// 🧾️ Every effect this turn produced that was NOT one of the `Effect::SendMessage{target:
        /// Shell{..}}` entries already unpacked into `frames` above — `📓️design-abi.md` §2's
        /// replacement for the deleted `AppFrame::Effects` wrapper: effects now travel as real
        /// `kernel::Effect` values on `TurnResult.effects` directly, not re-encoded as an `AppFrame`.
        pub effects: Vec<Effect>,
        pub command_ingress: semio_framework::kernel::CommandIngressStatus,
    }

    impl ExchangeOutcome {
        pub(crate) fn take_typed_operation_result(&self, receiver: u32) -> Option<TypedOperationResultPage> {
            typed_operation_result_exchange().get()?.take_page(receiver)
        }

        pub(crate) fn acknowledge_typed_operation_result(&self, token: TypedOperationResultToken) -> bool {
            typed_operation_result_exchange().get().is_some_and(|exchange| exchange.acknowledge(token))
        }

        pub(crate) fn take_surface(&mut self, surface: &str) -> Option<UiDocumentLease> {
            let index = self.surfaces.iter().position(|entry| entry.surface.as_ref() == surface)?;
            self.surfaces.swap_remove(index).map(|entry| entry.document)
        }

        pub(crate) fn take_product_replay_authority(&mut self, instance: u32, permit: MountedProductReplayAdmissionPermit) -> MountedProductReplayAdmission {
            let Some(index) = self.effects.iter().position(|effect| matches!(effect, Effect::SpawnJob { .. })) else {
                permit.release();
                return MountedProductReplayAdmission::None;
            };
            let (kind, input) = match &self.effects[index] {
                Effect::SpawnJob { kind, input, .. } => (kind.as_str(), input.as_slice()),
                _ => unreachable!("selected product replay effect remains SpawnJob"),
            };
            let cause = if kind.len() > PRODUCT_REPLAY_KIND_BYTES {
                Some(MountedProductReplayRefusalCause::KindCapacity)
            } else if input.len() > PRODUCT_REPLAY_INPUT_BYTES {
                Some(MountedProductReplayRefusalCause::InputCapacity)
            } else {
                None
            };
            let recovery = cause.is_none().then(|| mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").reserve()).flatten();
            if let Some(recovery) = recovery {
                let spawn = self.effects.remove(index);
                let request = MountedProductReplayRequest::from_admitted_effect(instance, index, spawn, recovery);
                permit.release();
                return MountedProductReplayAdmission::Admitted(request);
            }
            let cause = cause.unwrap_or(MountedProductReplayRefusalCause::RecoveryCapacity);
            let spawn = self.effects.remove(index);
            let remaining_effects = std::mem::take(&mut self.effects);
            MountedProductReplayAdmission::Refused(permit.refuse(cause, index, spawn, remaining_effects))
        }
    }

    pub(crate) enum KernelOutcome {
        Created(Result<u32, String>),
        Exchanged(Result<ExchangeOutcome, String>),
        ProductReplayMounted(Result<(), MountedProductReplayRequest>),
    }
    //#endregion

    //#region 🔖️KernelFuture — the leaf `Future` every `ProgramBridgeEntry` async method awaits
    #[derive(Default)]
    struct ResponseSlot {
        result: Mutex<Option<KernelOutcome>>,
        waker: Mutex<Option<Waker>>,
    }

    impl ResponseSlot {
        fn deliver(&self, outcome: KernelOutcome) {
            *self.result.lock().expect("response slot lock") = Some(outcome);
            if let Some(waker) = self.waker.lock().expect("response slot lock").take() {
                waker.wake();
            }
        }
    }

    /// 🌉 The genuinely-yielding leaf every plugin call now awaits, replacing the old in-process
    /// `WasmPluginRuntime::exchange` blocking call. The renderer worker-pool and app-task drivers
    /// supply its `Waker`; this future only stores and wakes the most recent one.
    struct KernelFuture {
        slot: Arc<ResponseSlot>,
        request: Option<KernelRequest>,
        queue: Arc<KernelRequestQueue>,
    }

    impl Future for KernelFuture {
        type Output = KernelOutcome;
        fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            let this = self.get_mut();
            if let Some(request) = this.request.take() {
                if let Err((request, slot)) = this.queue.try_push(request, this.slot.clone(), Some(cx.waker())) {
                    drop(slot);
                    this.request = Some(request);
                    return Poll::Pending;
                }
            }
            let mut result = this.slot.result.lock().expect("response slot lock");
            if let Some(outcome) = result.take() {
                return Poll::Ready(outcome);
            }
            drop(result);
            *this.slot.waker.lock().expect("response slot lock") = Some(cx.waker().clone());
            Poll::Pending
        }
    }

    fn persistent_command_completion_port_ready() -> bool {
        KERNEL_REQUEST_QUEUE_CAPACITY != 0 && semio_framework::kernel::COMMAND_MAXIMUM_PAGES != 0 && semio_framework::kernel::COMMAND_MAXIMUM_BYTES >= semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES
    }
    //#endregion

    //#region 🔖️KernelClient
    #[derive(Clone)]
    pub(crate) struct KernelClient {
        queue: Arc<KernelRequestQueue>,
        _task: Arc<KernelPoolFuture>,
        close_submissions: Arc<KernelCloseSubmissionRegistry>,
    }

    fn global_client() -> &'static OnceLock<KernelClient> {
        static CLIENT: OnceLock<KernelClient> = OnceLock::new();
        &CLIENT
    }

    impl KernelClient {
        /// ▶️ Mounts the kernel request state machine on the process-wide worker pool exactly once.
        pub(crate) fn get() -> KernelClient {
            install_mounted_typed_operation_result_exchange();
            global_client()
                .get_or_init(|| {
                    let queue = Arc::new(KernelRequestQueue::default());
                    let task = KernelPoolFuture::spawn(crate::renderer_worker_pool(), semio_framework_async::Lane::Interactive, run_kernel_pool(queue.clone()));
                    KernelClient { queue, _task: task, close_submissions: Arc::new(KernelCloseSubmissionRegistry::new()) }
                })
                .clone()
        }

        fn submit(&self, request: KernelRequest) -> KernelFuture {
            KernelFuture { slot: Arc::new(ResponseSlot::default()), request: Some(request), queue: self.queue.clone() }
        }

        fn try_acknowledge_job_progress(&self, token: JobProgressPresentationToken) -> bool {
            self.queue.try_push(KernelRequest::AcknowledgeJobProgress { token }, Arc::new(ResponseSlot::default()), None).is_ok()
        }

        pub(crate) fn take_typed_operation_result_page(&self, receiver: u32) -> Option<TypedOperationResultPage> {
            typed_operation_result_exchange().get()?.take_page(receiver)
        }

        pub(crate) fn acknowledge_typed_operation_result(&self, token: TypedOperationResultToken) -> bool {
            typed_operation_result_exchange().get().is_some_and(|exchange| exchange.acknowledge(token))
        }

        pub(crate) async fn create_app(&self, wasm_path: PathBuf, plugin_id: String, app_id: String) -> Result<u32, String> {
            match self.submit(KernelRequest::CreateApp { owner: CreateAppRequestOwner::new(wasm_path, plugin_id, app_id) }).await {
                KernelOutcome::Created(result) => result,
                KernelOutcome::Exchanged(_) => Err("kernel: unexpected Exchanged response for create_app".into()),
                KernelOutcome::ProductReplayMounted(_) => Err("kernel: unexpected product replay response for create_app".into()),
            }
        }

        pub(crate) fn begin_destroy_app(&self, instance: u32) -> Result<KernelCloseHandle, String> {
            let reservation = reserve_seq()?;
            let owner = Arc::new(KernelCloseSubmission {
                instance,
                realm: false,
                generation: reservation.generation(),
                queue: self.queue.clone(),
                pool: crate::renderer_worker_pool(),
                registry: Arc::downgrade(&self.close_submissions),
                phase: std::sync::atomic::AtomicU8::new(KERNEL_CLOSE_UNADMITTED),
            });
            reservation.commit()?;
            let handle = KernelCloseHandle { owner };
            let _ = handle.poll();
            Ok(handle)
        }

        pub(crate) fn begin_close_realm(&self) -> Result<KernelCloseHandle, String> {
            let reservation = reserve_seq()?;
            let owner = Arc::new(KernelCloseSubmission {
                instance: u32::MAX,
                realm: true,
                generation: reservation.generation(),
                queue: self.queue.clone(),
                pool: crate::renderer_worker_pool(),
                registry: Arc::downgrade(&self.close_submissions),
                phase: std::sync::atomic::AtomicU8::new(KERNEL_CLOSE_UNADMITTED),
            });
            reservation.commit()?;
            let handle = KernelCloseHandle { owner };
            let _ = handle.poll();
            Ok(handle)
        }

        pub(crate) fn destroy_app(&self, instance: u32) {
            let Ok(handle) = self.begin_destroy_app(instance) else { return };
            match handle.poll() {
                KernelCloseStatus::Pending | KernelCloseStatus::Complete => {}
                KernelCloseStatus::AdmissionBlocked | KernelCloseStatus::Fault => {
                    panic!("kernel destroy close authority was not admitted; ProgramBridge must retain and poll begin_destroy_app")
                }
            }
        }

        pub(crate) async fn exchange_commands(&self, instance: u32, commands: Vec<protocol::AppCommand>) -> Result<ExchangeOutcome, String> {
            if !persistent_command_completion_port_ready() {
                return Err("persistent command completion submit/poll/cancel authority is not admitted".to_string());
            }
            let mut envelopes = semio_framework::kernel::CommandEnvelopeSet::try_new().map_err(|fault| fault.to_string())?;
            for command in commands {
                let seq = next_seq()?;
                let command = protocol::encode_app_command(&command).await.map_err(|fault| fault.to_string())?;
                if let Err((fault, rejected)) = envelopes.try_push(semio_framework::kernel::CommandEnvelope { instance, seq, command }) {
                    self.queue.enqueue_retained(KernelRequest::CloseRejectedCommandBuild { key: u64::from(instance), owner: semio_framework::kernel::RejectedCommandBuild::new(envelopes, rejected) }, Arc::new(ResponseSlot::default())).await;
                    return Err(fault.to_string());
                }
            }
            let reservation = reserve_seq()?;
            let generation = reservation.generation();
            let batch = match semio_framework::kernel::CommandBatch::try_new(generation, envelopes) {
                Ok(batch) => {
                    reservation.commit()?;
                    batch
                }
                Err((fault, owners)) => {
                    self.queue.enqueue_retained(KernelRequest::CloseRejectedCommandBuild { key: u64::from(instance), owner: semio_framework::kernel::RejectedCommandBuild::from_admitted(owners) }, Arc::new(ResponseSlot::default())).await;
                    return Err(fault.to_string());
                }
            };
            let driver = semio_framework::kernel::CommandBatchDriver::new(generation, batch);
            match self.submit(KernelRequest::ExchangeCommands { instance, driver }).await {
                KernelOutcome::Exchanged(result) => result,
                KernelOutcome::Created(_) => Err("kernel: unexpected Created response for command exchange".into()),
                KernelOutcome::ProductReplayMounted(_) => Err("kernel: unexpected product replay response for command exchange".into()),
            }
        }

        pub(crate) async fn exchange_events(&self, instance: u32, events: Vec<Event>) -> Result<ExchangeOutcome, String> {
            let event = match QueuedKernelEvent::try_from_events(events) {
                Ok(event) => event,
                Err(owner) => {
                    self.queue.enqueue_retained(KernelRequest::CloseRejectedEvents { owner }, Arc::new(ResponseSlot::default())).await;
                    return Err("only one fixed SurfaceVisible event is admitted per kernel request turn".to_string());
                }
            };
            match self.submit(KernelRequest::Exchange { instance, event }).await {
                KernelOutcome::Exchanged(result) => result,
                KernelOutcome::Created(_) => Err("kernel: unexpected Created response for exchange".into()),
                KernelOutcome::ProductReplayMounted(_) => Err("kernel: unexpected product replay response for exchange".into()),
            }
        }

        pub(crate) async fn advance_retained(&self, instance: u32, surface: SurfaceId) -> Result<ExchangeOutcome, String> {
            match self.submit(KernelRequest::AdvanceRetained { instance, surface }).await {
                KernelOutcome::Exchanged(result) => result,
                KernelOutcome::Created(_) => Err("kernel: unexpected Created response for retained advance".into()),
                KernelOutcome::ProductReplayMounted(_) => Err("kernel: unexpected product replay response for retained advance".into()),
            }
        }

        pub(crate) async fn mount_product_replay(&self, owner: MountedProductReplayRequest) -> Result<(), MountedProductReplayRequest> {
            match self.submit(KernelRequest::MountProductReplay { owner }).await {
                KernelOutcome::ProductReplayMounted(result) => result,
                KernelOutcome::Created(_) | KernelOutcome::Exchanged(_) => unreachable!("mounted product replay request has one response class"),
            }
        }

        pub(crate) fn reserve_product_replay_admission(&self, instance: u32) -> Result<MountedProductReplayAdmissionPermit, String> {
            let token = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").reserve(instance).ok_or_else(|| "mounted product replay refusal capacity is full before exchange".to_string())?;
            Ok(MountedProductReplayAdmissionPermit { token: Some(token) })
        }

        pub(crate) async fn retire_product_replay(&self, owner: MountedProductReplayRequest) {
            match self.submit(KernelRequest::RetireProductReplay { owner }).await {
                KernelOutcome::ProductReplayMounted(Ok(())) => {}
                KernelOutcome::ProductReplayMounted(Err(owner)) => owner.retire(),
                KernelOutcome::Created(_) | KernelOutcome::Exchanged(_) => unreachable!("product replay retirement has one response class"),
            }
        }

        pub(crate) async fn retire_product_replay_refusal(&self, owner: RefusedProductReplay) {
            match self.submit(KernelRequest::RetireProductReplayRefusal { owner }).await {
                KernelOutcome::ProductReplayMounted(Ok(())) => {}
                KernelOutcome::ProductReplayMounted(Err(_)) => unreachable!("typed product refusal has no raw request response"),
                KernelOutcome::Created(_) | KernelOutcome::Exchanged(_) => unreachable!("product replay refusal retirement has one response class"),
            }
        }

        pub(crate) async fn advance_product_replay(&self, instance: u32) -> Result<(), String> {
            match self.submit(KernelRequest::AdvanceProductReplay { instance }).await {
                KernelOutcome::Exchanged(result) => result.map(|_| ()),
                KernelOutcome::Created(_) => Err("kernel: unexpected Created response for product replay advance".into()),
                KernelOutcome::ProductReplayMounted(_) => Err("kernel: unexpected mount response for product replay advance".into()),
            }
        }
    }
    //#endregion

    //#region 🔖️KernelPoolState
    //#region 📄️RetainedDocumentExchange
    enum RetainedPatchApply {
        Applying(UiPatchApplyProducer),
        Ready(UiPatchApplyOutcome),
        Rejected(UiPatchApplyRejected),
    }

    struct RetainedDocumentBuild {
        generation: u64,
        revision: UiRevision,
        cursor: usize,
        builder: Option<UiDocumentBuilder>,
        rejected_record: Option<ui_contract::UiNodeRecord>,
        closing: bool,
    }
    //#endregion 📄️RetainedDocumentExchange

    struct RetainedSurface {
        state: Option<UiSnapshotState>,
        patch: Option<RetainedPatchApply>,
        queued_patches: UiFixedList<KernelUiPatch, UI_DOCUMENT_LEASE_SLOTS>,
        rejected_patches: UiFixedList<KernelUiPatch, UI_DOCUMENT_LEASE_SLOTS>,
        build: Option<RetainedDocumentBuild>,
        build_pending: bool,
        published: Option<UiDocumentLease>,
        closing: Option<UiDocumentLease>,
        exchange_closing: Option<UiDocumentLease>,
        patch_close_complete: bool,
    }

    impl RetainedSurface {
        fn new(surface: SurfaceId) -> Self {
            Self {
                state: Some(UiSnapshotState::new(surface)),
                patch: None,
                queued_patches: UiFixedList::default(),
                rejected_patches: UiFixedList::default(),
                build: None,
                build_pending: false,
                published: None,
                closing: None,
                exchange_closing: None,
                patch_close_complete: false,
            }
        }

        fn admit_patch(&mut self, patch: KernelUiPatch) -> Result<(), KernelUiPatch> {
            if self.patch.is_some() || self.state.is_none() {
                return self.queued_patches.try_push(patch);
            }
            let reservation = match reserve_seq() {
                Ok(reservation) => reservation,
                Err(_) => return Err(patch),
            };
            let state = self.state.take().expect("retained surface state is admitted once");
            let generation = reservation.generation();
            let owner = match UiPatchApplyProducer::try_new(state, patch, UiDocumentLimits::default(), generation) {
                Ok(producer) => RetainedPatchApply::Applying(producer),
                Err(rejected) => RetainedPatchApply::Rejected(rejected),
            };
            let committed = reservation.commit().is_ok();
            self.patch = Some(owner);
            if !committed {
                return Ok(());
            }
            Ok(())
        }

        fn advance_patch_one(&mut self) -> Option<(UiRevision, ui_contract::UiText)> {
            if let Some(patch) = self.rejected_patches.get_mut(0) {
                if patch.ops.pop().is_some() {
                    return None;
                }
                let _ = self.rejected_patches.swap_remove(0);
                return Some((UiRevision::default(), ui_contract::UiText::try_from_str("retained patch admission capacity exhausted").unwrap_or_default()));
            }
            if self.patch.is_none() {
                if let Some(patch) = self.queued_patches.swap_remove(0) {
                    if let Err(patch) = self.admit_patch(patch) {
                        let _ = self.queued_patches.try_push(patch);
                    }
                }
            }
            let current = self.patch.take()?;
            match current {
                RetainedPatchApply::Applying(mut producer) => match producer.drive_one(producer.generation(), false, false) {
                    UiPatchApplyStep::MoreWork => self.patch = Some(RetainedPatchApply::Applying(producer)),
                    UiPatchApplyStep::Ready => match producer.take_ready() {
                        Ok(outcome) => self.patch = Some(RetainedPatchApply::Ready(outcome)),
                        Err(producer) => self.patch = Some(RetainedPatchApply::Applying(producer)),
                    },
                    UiPatchApplyStep::Rejected => match producer.take_rejected() {
                        Ok(rejected) => self.patch = Some(RetainedPatchApply::Rejected(rejected)),
                        Err(producer) => self.patch = Some(RetainedPatchApply::Applying(producer)),
                    },
                },
                RetainedPatchApply::Ready(mut outcome) => {
                    if !outcome.close_step() {
                        self.patch = Some(RetainedPatchApply::Ready(outcome));
                    } else {
                        match outcome.take_state() {
                            Ok(state) => {
                                self.state = Some(state);
                                self.build_pending = true;
                            }
                            Err(outcome) => self.patch = Some(RetainedPatchApply::Ready(outcome)),
                        }
                    }
                }
                RetainedPatchApply::Rejected(mut rejected) => {
                    let revision = rejected.state().map_or(UiRevision::default(), UiSnapshotState::revision);
                    let reason = ui_contract::UiText::try_from_str("retained patch rejected").unwrap_or_default();
                    if !rejected.close_step() {
                        self.patch = Some(RetainedPatchApply::Rejected(rejected));
                    } else {
                        match rejected.take_state() {
                            Ok(state) => self.state = Some(state),
                            Err(rejected) => self.patch = Some(RetainedPatchApply::Rejected(rejected)),
                        }
                    }
                    return Some((revision, reason));
                }
            }
            None
        }

        fn begin_document_build(&mut self) {
            if self.build.is_some() || !self.build_pending {
                return;
            }
            let Some(state) = self.state.as_ref() else { return };
            let Ok(reservation) = reserve_seq() else { return };
            let generation = reservation.generation();
            let builder = match UiDocumentBuilder::try_new(generation, state.surface.clone(), state.revision, state.root, state.revision.0) {
                Ok(builder) => builder,
                Err((_fault, _surface)) => {
                    let _ = ui_contract::close_ui_document_page_one();
                    return;
                }
            };
            let mut build = RetainedDocumentBuild { generation, revision: state.revision, cursor: 0, builder: Some(builder), rejected_record: None, closing: false };
            if reservation.commit().is_err() {
                build.closing = true;
                self.build = Some(build);
                return;
            }
            self.build_pending = false;
            self.build = Some(build);
        }

        fn advance_document_one(&mut self) {
            if let Some(closing) = self.exchange_closing.as_mut() {
                if closing.close_step() && closing.terminal_is_empty() {
                    self.exchange_closing = None;
                }
                return;
            }
            if let Some(closing) = self.closing.as_mut() {
                if closing.close_step() && closing.terminal_is_empty() {
                    self.closing = None;
                }
                return;
            }
            self.begin_document_build();
            let Some(build) = self.build.as_mut() else { return };
            if build.closing {
                let Some(builder) = build.builder.as_mut() else { return };
                if builder.close_step() && builder.terminal_is_empty() {
                    build.builder = None;
                    if build.rejected_record.take().is_some() {
                        return;
                    }
                    self.build = None;
                }
                return;
            }
            let Some(state) = self.state.as_ref() else { return };
            if state.revision != build.revision {
                build.closing = true;
                return;
            }
            if let Some(record) = state.nodes.get_index(build.cursor) {
                let Some(record) = record.credited_clone() else {
                    build.closing = true;
                    return;
                };
                let Some(builder) = build.builder.as_mut() else { return };
                if let Err((_fault, record)) = builder.try_push(record) {
                    build.rejected_record = Some(record);
                    build.closing = true;
                    return;
                }
                let Some(cursor) = build.cursor.checked_add(1) else {
                    build.closing = true;
                    return;
                };
                build.cursor = cursor;
                return;
            }
            let Some(builder) = build.builder.take() else { return };
            match builder.finish() {
                Ok(document) => {
                    self.closing = self.published.replace(document);
                    self.build = None;
                }
                Err((_fault, builder)) => {
                    build.builder = Some(builder);
                    build.closing = true;
                }
            }
        }

        fn try_alias(&self) -> Option<UiDocumentLease> {
            self.published.as_ref()?.try_alias().ok()
        }

        fn close_step(&mut self) -> bool {
            if let Some(current) = self.patch.take() {
                match current {
                    RetainedPatchApply::Applying(mut producer) => {
                        let _ = producer.drive_one(producer.generation(), true, false);
                        self.patch = Some(match producer.take_rejected() {
                            Ok(rejected) => RetainedPatchApply::Rejected(rejected),
                            Err(producer) => RetainedPatchApply::Applying(producer),
                        });
                    }
                    RetainedPatchApply::Ready(mut outcome) => {
                        if outcome.close_step() {
                            match outcome.take_state() {
                                Ok(state) => self.state = Some(state),
                                Err(outcome) => self.patch = Some(RetainedPatchApply::Ready(outcome)),
                            }
                        } else {
                            self.patch = Some(RetainedPatchApply::Ready(outcome));
                        }
                    }
                    RetainedPatchApply::Rejected(mut rejected) => {
                        if rejected.close_step() {
                            match rejected.take_state() {
                                Ok(state) => self.state = Some(state),
                                Err(rejected) => self.patch = Some(RetainedPatchApply::Rejected(rejected)),
                            }
                        } else {
                            self.patch = Some(RetainedPatchApply::Rejected(rejected));
                        }
                    }
                }
                return false;
            }
            if let Some(patch) = self.queued_patches.swap_remove(0) {
                if let Err(patch) = self.admit_patch(patch) {
                    let _ = self.queued_patches.try_push(patch);
                }
                return false;
            }
            if let Some(patch) = self.rejected_patches.get_mut(0) {
                if patch.ops.pop().is_none() {
                    let _ = self.rejected_patches.swap_remove(0);
                }
                return false;
            }
            if let Some(build) = self.build.as_mut() {
                build.closing = true;
                if let Some(builder) = build.builder.as_mut() {
                    if builder.close_step() && builder.terminal_is_empty() {
                        build.builder = None;
                        if build.rejected_record.take().is_some() {
                            return false;
                        }
                        self.build = None;
                    }
                }
                return false;
            }
            if self.build_pending {
                self.build_pending = false;
                return false;
            }
            if let Some(document) = self.published.as_mut() {
                if document.close_step() && document.terminal_is_empty() {
                    self.published = None;
                }
                return false;
            }
            if let Some(document) = self.closing.as_mut() {
                if document.close_step() && document.terminal_is_empty() {
                    self.closing = None;
                }
                return false;
            }
            if let Some(document) = self.exchange_closing.as_mut() {
                if document.close_step() && document.terminal_is_empty() {
                    self.exchange_closing = None;
                }
                return false;
            }
            if let Some(state) = self.state.as_mut() {
                if let Some(id) = state.nodes.keys().next().copied() {
                    drop(state.nodes.remove(&id));
                    return false;
                }
                self.state = None;
                return false;
            }
            if !self.patch_close_complete {
                self.patch_close_complete = ui_contract::close_ui_patch_owner_one();
                return false;
            }
            ui_contract::close_ui_document_page_one()
        }
    }

    const RETAINED_SURFACE_CAPACITY: usize = 64;

    struct RetainedSurfaceSlot {
        generation: u64,
        instance: u32,
        surface: SurfaceId,
        owner: RetainedSurface,
    }

    struct RetainedSurfaceRegistry {
        slots: [Option<RetainedSurfaceSlot>; RETAINED_SURFACE_CAPACITY],
        next_generation: u64,
        generation_exhausted: bool,
        close_cursor: usize,
    }

    impl RetainedSurfaceRegistry {
        fn new() -> Self {
            Self { slots: std::array::from_fn(|_| None), next_generation: 0, generation_exhausted: false, close_cursor: 0 }
        }

        fn index(&self, instance: u32, surface: &SurfaceId) -> Option<usize> {
            self.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.instance == instance && slot.surface == *surface))
        }

        fn get(&self, instance: u32, surface: &SurfaceId) -> Option<&RetainedSurfaceSlot> {
            self.index(instance, surface).and_then(|index| self.slots[index].as_ref())
        }

        fn get_mut(&mut self, instance: u32, surface: &SurfaceId) -> Option<&mut RetainedSurfaceSlot> {
            let index = self.index(instance, surface)?;
            self.slots[index].as_mut()
        }

        fn try_admit(&mut self, instance: u32, surface: SurfaceId) -> Result<usize, SurfaceId> {
            if let Some(index) = self.index(instance, &surface) {
                return Ok(index);
            }
            if self.generation_exhausted {
                return Err(surface);
            }
            let Some(index) = self.slots.iter().position(Option::is_none) else { return Err(surface) };
            let Some(generation) = self.next_generation.checked_add(1).filter(|generation| *generation != 0) else { return Err(surface) };
            let owner = RetainedSurface::new(surface.clone());
            self.slots[index] = Some(RetainedSurfaceSlot { generation, instance, surface, owner });
            self.next_generation = generation;
            self.generation_exhausted = generation == u64::MAX;
            Ok(index)
        }

        fn advance_instance_close_one(&mut self, instance: u32) -> bool {
            let index = self.close_cursor;
            self.close_cursor = (self.close_cursor + 1) % RETAINED_SURFACE_CAPACITY;
            let Some(slot) = self.slots[index].as_mut().filter(|slot| slot.instance == instance) else {
                return !self.slots.iter().flatten().any(|slot| slot.instance == instance);
            };
            if slot.owner.close_step() {
                self.slots[index] = None;
            }
            false
        }

        fn advance_realm_close_one(&mut self) -> bool {
            let index = self.close_cursor;
            self.close_cursor = (self.close_cursor + 1) % RETAINED_SURFACE_CAPACITY;
            let Some(slot) = self.slots[index].as_mut() else { return self.slots.iter().all(Option::is_none) };
            if slot.owner.close_step() {
                self.slots[index] = None;
            }
            false
        }

        fn terminal_is_empty(&self) -> bool {
            self.slots.iter().all(Option::is_none)
        }
    }

    struct PendingSurfaceRejection {
        generation: u64,
        instance: u32,
        surface: SurfaceId,
        revision: UiRevision,
        reason: ui_contract::UiText,
        patch: Option<KernelUiPatch>,
    }

    struct PendingSurfaceRejectionRegistry {
        slots: [Option<PendingSurfaceRejection>; RETAINED_SURFACE_CAPACITY],
        close_cursor: usize,
    }

    impl PendingSurfaceRejectionRegistry {
        fn new() -> Self {
            Self { slots: std::array::from_fn(|_| None), close_cursor: 0 }
        }

        fn index(&self, instance: u32, surface: &SurfaceId) -> Option<usize> {
            self.slots.iter().position(|slot| slot.as_ref().is_some_and(|slot| slot.instance == instance && slot.surface == *surface))
        }

        fn get(&self, instance: u32, surface: &SurfaceId) -> Option<&PendingSurfaceRejection> {
            self.index(instance, surface).and_then(|index| self.slots[index].as_ref())
        }

        fn remove(&mut self, instance: u32, surface: &SurfaceId) -> Option<PendingSurfaceRejection> {
            let index = self.index(instance, surface)?;
            self.slots[index].take()
        }

        fn try_admit(&mut self, rejection: PendingSurfaceRejection) -> Result<(), PendingSurfaceRejection> {
            let Some(slot) = self.slots.iter_mut().find(|slot| slot.is_none()) else { return Err(rejection) };
            *slot = Some(rejection);
            Ok(())
        }

        fn try_upsert(&mut self, rejection: PendingSurfaceRejection) -> Result<(), PendingSurfaceRejection> {
            if let Some(index) = self.index(rejection.instance, &rejection.surface) {
                if self.slots[index].as_ref().is_some_and(|current| current.patch.is_some()) {
                    return Err(rejection);
                }
                self.slots[index] = Some(rejection);
                return Ok(());
            }
            self.try_admit(rejection)
        }

        fn take_instance_one(&mut self, instance: u32) -> Option<PendingSurfaceRejection> {
            let index = self.close_cursor;
            self.close_cursor = (self.close_cursor + 1) % RETAINED_SURFACE_CAPACITY;
            let slot = self.slots[index].as_mut().filter(|slot| slot.instance == instance)?;
            if let Some(patch) = slot.patch.as_mut() {
                if patch.ops.pop().is_none() {
                    slot.patch = None;
                }
                return None;
            }
            self.slots[index].take()
        }

        fn take_any_one(&mut self) -> Option<PendingSurfaceRejection> {
            let index = self.close_cursor;
            self.close_cursor = (self.close_cursor + 1) % RETAINED_SURFACE_CAPACITY;
            if let Some(patch) = self.slots[index].as_mut().and_then(|slot| slot.patch.as_mut()) {
                if patch.ops.pop().is_none() {
                    if let Some(slot) = self.slots[index].as_mut() {
                        slot.patch = None;
                    }
                }
                return None;
            }
            self.slots[index].take()
        }

        fn terminal_is_empty(&self) -> bool {
            self.slots.iter().all(Option::is_none)
        }

        fn instance_is_empty(&self, instance: u32) -> bool {
            !self.slots.iter().flatten().any(|slot| slot.instance == instance)
        }
    }

    struct PendingJobProgressPresentation {
        token: JobProgressPresentationToken,
        receipt: JobProgressReceipt,
    }

    #[derive(Clone, Copy)]
    struct MountedReplayRouteSeed {
        actor: ActorId,
        plugin: [u8; 32],
        package: [u8; 32],
        window: u64,
        document: [u8; 32],
    }

    impl MountedReplayRouteSeed {
        fn compose(self, request: JobReplayRequest) -> JobReplayRoute {
            JobReplayRoute {
                plugin: self.plugin,
                package: self.package,
                controller: request.controller,
                tool: request.tool,
                window: self.window,
                document: self.document,
                request_schema: request.schema,
                request_version: request.version,
                request_digest: request.digest,
            }
        }
    }

    struct MountedJobReplay {
        actor: ActorId,
        authority: JobTurn,
        request: JobReplayRequest,
        placement: JobPlacement,
        log: ManuallyDrop<JobReplayLog>,
        captured: Option<JobPublication>,
        policy: Option<JobReplayPublicationPolicy>,
        terminal_seen: bool,
        replay_requested: bool,
        replay_started: bool,
        replay_submit_sequence: Option<u64>,
        accepted_replay_sequence: Option<u64>,
        replay_worker_count: u16,
        replay_worker_slot: u16,
        recovery: Option<MountedReplayRecoveryToken>,
    }

    impl MountedJobReplay {
        fn pending_replay_start_is_exact(&self, worker_count: u16, worker_slot: u16) -> bool {
            self.replay_requested && !self.replay_started && self.replay_worker_count == worker_count && self.replay_worker_slot == worker_slot
        }

        fn replay_submission_sequence(&mut self) -> Result<u64, String> {
            match self.replay_submit_sequence {
                Some(sequence) => Ok(sequence),
                None => {
                    let sequence = next_seq()?;
                    self.replay_submit_sequence = Some(sequence);
                    Ok(sequence)
                }
            }
        }

        fn accept_replay_submission(&mut self, sequence: u64) -> Result<(), String> {
            if self.replay_started || self.replay_submit_sequence != Some(sequence) {
                return Err("kernel: accepted replay start does not match the retained submission identity".to_string());
            }
            self.replay_started = true;
            self.replay_submit_sequence = None;
            self.accepted_replay_sequence = Some(sequence);
            Ok(())
        }
    }

    #[derive(Clone, Copy)]
    struct MountedReplayRecoveryToken {
        index: usize,
        epoch: u64,
    }

    struct MountedReplayRecoveryOwner {
        generation: u64,
        actor: ActorId,
        job: u64,
        log: Option<JobReplayLog>,
        captured: Option<JobPublication>,
        publication_phase: u8,
    }

    struct MountedReplayRecoverySlot {
        epoch: u64,
        reserved: bool,
        owner: Option<MountedReplayRecoveryOwner>,
    }

    struct MountedReplayRecoveryRegistry {
        slots: [MountedReplayRecoverySlot; JOB_PROGRESS_ACTIVE_CAPACITY],
        abandoned: [Option<MountedReplayRecoveryOwner>; JOB_PROGRESS_ACTIVE_CAPACITY],
        cursor: usize,
        abandoned_cursor: usize,
    }

    impl MountedReplayRecoveryRegistry {
        fn new() -> Self {
            Self { slots: std::array::from_fn(|_| MountedReplayRecoverySlot { epoch: 0, reserved: false, owner: None }), abandoned: std::array::from_fn(|_| None), cursor: 0, abandoned_cursor: 0 }
        }

        fn reserve(&mut self) -> Option<MountedReplayRecoveryToken> {
            let index = self.slots.iter().position(|slot| !slot.reserved && slot.owner.is_none())?;
            let epoch = self.slots[index].epoch.checked_add(1)?;
            self.slots[index].epoch = epoch;
            self.slots[index].reserved = true;
            Some(MountedReplayRecoveryToken { index, epoch })
        }

        fn release(&mut self, token: MountedReplayRecoveryToken) -> bool {
            let Some(slot) = self.slots.get_mut(token.index) else { return false };
            if slot.epoch != token.epoch || !slot.reserved || slot.owner.is_some() {
                return false;
            }
            slot.reserved = false;
            true
        }

        fn publish(&mut self, token: MountedReplayRecoveryToken, owner: MountedReplayRecoveryOwner) {
            if let Some(slot) = self.slots.get_mut(token.index) {
                if slot.epoch == token.epoch && slot.reserved && slot.owner.is_none() {
                    slot.owner = Some(owner);
                    return;
                }
            }
            let slot = self.abandoned.iter_mut().find(|slot| slot.is_none()).expect("pre-reserved recovery makes at most one discoverable owner reachable per mounted shell");
            *slot = Some(owner);
        }

        fn close_one(&mut self) -> bool {
            if let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.slots[*index].owner.is_some()) {
                self.cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
                if mounted_replay_recovery_owner_close_one(self.slots[index].owner.as_mut().expect("published replay recovery owner")) {
                    let terminal = self.slots[index].owner.take().expect("terminal replay recovery owner");
                    debug_assert!(terminal.log.is_none() && terminal.captured.is_none());
                    self.slots[index].reserved = false;
                }
                return true;
            }
            let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).map(|offset| (self.abandoned_cursor + offset) % JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.abandoned[*index].is_some()) else {
                return false;
            };
            self.abandoned_cursor = (index + 1) % JOB_PROGRESS_ACTIVE_CAPACITY;
            if mounted_replay_recovery_owner_close_one(self.abandoned[index].as_mut().expect("discoverable abandoned replay owner")) {
                let terminal = self.abandoned[index].take().expect("terminal abandoned replay owner");
                debug_assert!(terminal.log.is_none() && terminal.captured.is_none());
            }
            true
        }

        fn has_close_work(&self) -> bool {
            self.slots.iter().any(|slot| slot.owner.is_some()) || self.abandoned.iter().any(Option::is_some)
        }
    }

    fn mounted_replay_recovery_owner_close_one(owner: &mut MountedReplayRecoveryOwner) -> bool {
        if let Some(publication) = owner.captured.as_mut() {
            let released = match (&mut publication.outcome, owner.publication_phase) {
                (semio_framework_actor::JobStepOutcome::PreviewReady { preview }, 0) => std::mem::take(preview).capacity(),
                (semio_framework_actor::JobStepOutcome::CheckpointReady { checkpoint }, 0) => std::mem::take(&mut checkpoint.state).capacity(),
                (semio_framework_actor::JobStepOutcome::Complete { candidate }, 0) => std::mem::take(&mut candidate.state).capacity(),
                (semio_framework_actor::JobStepOutcome::Complete { candidate }, 1) => std::mem::take(&mut candidate.output).capacity(),
                (semio_framework_actor::JobStepOutcome::Fault { detail }, 0) => std::mem::take(detail).capacity(),
                _ => 0,
            };
            let phases = match &publication.outcome {
                semio_framework_actor::JobStepOutcome::Complete { .. } => 2,
                semio_framework_actor::JobStepOutcome::PreviewReady { .. } | semio_framework_actor::JobStepOutcome::CheckpointReady { .. } | semio_framework_actor::JobStepOutcome::Fault { .. } => 1,
                semio_framework_actor::JobStepOutcome::Yield | semio_framework_actor::JobStepOutcome::Cancelled => 0,
            };
            owner.publication_phase += 1;
            if owner.publication_phase >= phases {
                owner.captured = None;
            }
            let _ = released;
            return false;
        }
        if let Some(log) = owner.log.as_mut() {
            log.begin_close();
            let now = semio_framework_job::default_now_us();
            let mut preview_sequence = owner.job;
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(owner.actor.0),
                semio_framework_job::Generation(owner.generation),
                now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::MAINTENANCE_LANE_WALL_US)).unwrap_or(semio_framework_job::StepBudget::new(0, 0)),
                semio_framework_job::root_cancel_token(),
                semio_framework_job::default_now_us,
                &mut preview_sequence,
            );
            let _ = log.close_step(&mut context);
            if log.terminal_is_empty() {
                owner.log = None;
            }
            return false;
        }
        true
    }

    fn mounted_replay_recovery_registry() -> &'static Mutex<MountedReplayRecoveryRegistry> {
        static REGISTRY: OnceLock<Mutex<MountedReplayRecoveryRegistry>> = OnceLock::new();
        REGISTRY.get_or_init(|| Mutex::new(MountedReplayRecoveryRegistry::new()))
    }

    impl Drop for MountedJobReplay {
        fn drop(&mut self) {
            let Some(token) = self.recovery.take() else { return };
            if self.log.terminal_is_empty() && self.captured.is_none() && self.policy.is_none() {
                unsafe { ManuallyDrop::drop(&mut self.log) };
                assert!(mounted_replay_recovery_registry().lock().expect("replay recovery lock").release(token));
                return;
            }
            let owner =
                MountedReplayRecoveryOwner { generation: self.authority.operation.generation, actor: self.actor, job: self.authority.job, log: Some(unsafe { ManuallyDrop::take(&mut self.log) }), captured: self.captured.take(), publication_phase: 0 };
            mounted_replay_recovery_registry().lock().expect("replay recovery lock").publish(token, owner);
        }
    }

    struct ClosingKernelApp {
        instance: u32,
        actors: [ActorId; JOB_PROGRESS_ACTIVE_CAPACITY],
        actor_count: usize,
        replay_begin_cursor: usize,
        begin_cursor: usize,
        unregister_cursor: usize,
        route_cursor: usize,
    }

    impl ClosingKernelApp {
        fn contains(&self, actor: ActorId) -> bool {
            self.actors[..self.actor_count].contains(&actor)
        }
    }

    //#region 📦️CommandDocumentRetirement
    const COMMAND_DOCUMENT_RETIREMENT_CAPACITY: usize = UI_DOCUMENT_LEASE_SLOTS * ui_contract::UI_DOCUMENT_LEASE_ALIASES as usize;

    #[derive(Clone, Copy)]
    struct CommandDocumentReservationToken {
        index: usize,
        epoch: u64,
        batch: u64,
        generation: u64,
    }

    struct CommandDocumentPageReservation {
        tokens: UiFixedList<CommandDocumentReservationToken, UI_DOCUMENT_LEASE_SLOTS>,
    }

    enum CommandDocumentRetirementState {
        Reserved { epoch: u64, batch: u64, generation: u64 },
        Pending { epoch: u64, batch: u64, generation: u64, surface: SurfaceId, document: UiDocumentLease },
        Closing { epoch: u64, batch: u64, generation: u64, surface: SurfaceId, document: UiDocumentLease },
    }

    struct CommandDocumentRetirementRegistry {
        slots: [Option<CommandDocumentRetirementState>; COMMAND_DOCUMENT_RETIREMENT_CAPACITY],
        epochs: [u64; COMMAND_DOCUMENT_RETIREMENT_CAPACITY],
        cursor: usize,
    }

    impl CommandDocumentRetirementRegistry {
        fn new() -> Self {
            Self { slots: std::array::from_fn(|_| None), epochs: [0; COMMAND_DOCUMENT_RETIREMENT_CAPACITY], cursor: 0 }
        }

        fn try_reserve_page(&mut self, batch: u64, generation: u64) -> Result<CommandDocumentPageReservation, ()> {
            let mut candidates = UiFixedList::<CommandDocumentReservationToken, UI_DOCUMENT_LEASE_SLOTS>::default();
            for index in 0..COMMAND_DOCUMENT_RETIREMENT_CAPACITY {
                if self.slots[index].is_some() {
                    continue;
                }
                let Some(epoch) = self.epochs[index].checked_add(1) else { continue };
                candidates.try_push(CommandDocumentReservationToken { index, epoch, batch, generation }).map_err(|_| ())?;
                if candidates.len() == UI_DOCUMENT_LEASE_SLOTS {
                    break;
                }
            }
            if candidates.len() != UI_DOCUMENT_LEASE_SLOTS {
                return Err(());
            }
            for token in &candidates {
                self.epochs[token.index] = token.epoch;
                self.slots[token.index] = Some(CommandDocumentRetirementState::Reserved { epoch: token.epoch, batch, generation });
            }
            Ok(CommandDocumentPageReservation { tokens: candidates })
        }

        fn admit(&mut self, reservation: &mut CommandDocumentPageReservation, owner: ExchangeSurfaceDocument) {
            let token = reservation.tokens.pop().expect("one fixed command-document destination is reserved for every fixed turn-result surface");
            self.slots[token.index] = Some(CommandDocumentRetirementState::Pending { epoch: token.epoch, batch: token.batch, generation: token.generation, surface: owner.surface, document: owner.document });
        }

        fn release(&mut self, reservation: &mut CommandDocumentPageReservation) {
            while let Some(token) = reservation.tokens.pop() {
                let matches =
                    self.slots[token.index].as_ref().is_some_and(|state| matches!(state, CommandDocumentRetirementState::Reserved { epoch, batch, generation } if *epoch == token.epoch && *batch == token.batch && *generation == token.generation));
                if matches {
                    self.slots[token.index] = None;
                }
            }
        }

        fn begin_close_batch(&mut self, batch: u64, generation: u64) {
            for slot in &mut self.slots {
                let Some(state) = slot.take() else { continue };
                *slot = match state {
                    CommandDocumentRetirementState::Reserved { batch: owner_batch, generation: owner_generation, .. } if owner_batch == batch && owner_generation == generation => None,
                    CommandDocumentRetirementState::Pending { epoch, batch: owner_batch, generation: owner_generation, surface, document } if owner_batch == batch && owner_generation == generation => {
                        Some(CommandDocumentRetirementState::Closing { epoch, batch: owner_batch, generation: owner_generation, surface, document })
                    }
                    state => Some(state),
                };
            }
        }

        fn begin_close_instance(&mut self, batch: u64) {
            for slot in &mut self.slots {
                let Some(state) = slot.take() else { continue };
                *slot = match state {
                    CommandDocumentRetirementState::Reserved { batch: owner_batch, .. } if owner_batch == batch => None,
                    CommandDocumentRetirementState::Pending { epoch, batch: owner_batch, generation, surface, document } if owner_batch == batch => {
                        Some(CommandDocumentRetirementState::Closing { epoch, batch: owner_batch, generation, surface, document })
                    }
                    state => Some(state),
                };
            }
        }

        fn publish_batch(&mut self, batch: u64, generation: u64, output: &mut UiFixedList<ExchangeSurfaceDocument, UI_DOCUMENT_LEASE_SLOTS>) {
            for slot in &mut self.slots {
                let Some(state) = slot.take() else { continue };
                *slot = match state {
                    CommandDocumentRetirementState::Pending { epoch, batch: owner_batch, generation: owner_generation, surface, document } if owner_batch == batch && owner_generation == generation => {
                        let owner = ExchangeSurfaceDocument { surface, document };
                        match output.try_push(owner) {
                            Ok(()) => None,
                            Err(owner) => Some(CommandDocumentRetirementState::Closing { epoch, batch: owner_batch, generation: owner_generation, surface: owner.surface, document: owner.document }),
                        }
                    }
                    state => Some(state),
                };
            }
        }

        fn close_one(&mut self) -> bool {
            let index = self.cursor;
            self.cursor = (self.cursor + 1) % COMMAND_DOCUMENT_RETIREMENT_CAPACITY;
            let Some(CommandDocumentRetirementState::Closing { document, .. }) = self.slots[index].as_mut() else { return false };
            let terminal = document.close_step() && document.terminal_is_empty();
            if terminal {
                let terminal = self.slots[index].take().expect("terminal command document retirement slot");
                assert!(matches!(&terminal, CommandDocumentRetirementState::Closing { document, .. } if document.terminal_is_empty()), "command document retirement witness changed before removal");
            }
            true
        }

        fn has_close_work(&self) -> bool {
            self.slots.iter().flatten().any(|state| matches!(state, CommandDocumentRetirementState::Closing { .. }))
        }

        fn batch_is_empty(&self, batch: u64, generation: u64) -> bool {
            !self.slots.iter().flatten().any(|state| match state {
                CommandDocumentRetirementState::Reserved { batch: owner_batch, generation: owner_generation, .. }
                | CommandDocumentRetirementState::Pending { batch: owner_batch, generation: owner_generation, .. }
                | CommandDocumentRetirementState::Closing { batch: owner_batch, generation: owner_generation, .. } => *owner_batch == batch && *owner_generation == generation,
            })
        }

        fn instance_is_empty(&self, batch: u64) -> bool {
            !self.slots.iter().flatten().any(|state| match state {
                CommandDocumentRetirementState::Reserved { batch: owner_batch, .. } | CommandDocumentRetirementState::Pending { batch: owner_batch, .. } | CommandDocumentRetirementState::Closing { batch: owner_batch, .. } => *owner_batch == batch,
            })
        }
    }
    //#endregion 📦️CommandDocumentRetirement

    struct KernelPoolState {
        request_queue: Arc<KernelRequestQueue>,
        guest_runtime: Arc<GuestRuntimes>,
        /// 🎠️ terra-kernel-loop: the real multi-shard engine — replaces the single physical
        /// `ShardLoop`/`Kernel::new(.., 1, 0, ..)` this host used to run. `Kernel::new(Native, K, 2,
        /// 64)` (`exclusive_reserve: 2` — item 3 of the packet brief — makes `request_exclusive`
        /// real; no caller in this file exercises it yet, but the reserve pool now genuinely exists).
        /// P1e (INTERACTIVE-JOB-RUNTIME-REFACTOR, one-pool-worker-runtime): every shard now runs as a
        /// pool-scheduled job on `crate::renderer_worker_pool()` — no `ShardExecutor`/forwarder OS
        /// threads — see `🎠️runtime.rs`'s own module doc.
        runtime: crate::parallel_runtime::ParallelRuntime,
        worker_count: u16,
        /// ⏱️ Monotonic milliseconds this host's own `Kernel::tick` calls are stamped with — this
        /// crate's purity-respecting clock source (`Kernel` itself takes no clock, per `🎭️actor`'s
        /// own rule), incremented once per `run_turn`-internal tick, never wall-clock-read.
        now_ms: u64,
        plugin_ordinals: HashMap<String, u16>,
        /// 📇️ `instance_id` (the `u32` `ProgramBridgeEntry`'s callers already address plugin apps
        /// by) → the kernel's own bit-packed `ActorId`, minted by `Kernel::activate`.
        instances: HashMap<u32, ActorId>,
        next_instance_id: u32,
        /// 🖼️ One retained semantic [`UiSnapshotState`] plus its last published generation-qualified
        /// document lease per `(instance, surface)`. Every [`UiPatchOp`] is applied through the
        /// contract's retained, quota-bounded [`UiPatchApplyProducer`]; a rejection preserves both
        /// the document revision and the previously published document.
        retained: RetainedSurfaceRegistry,
        /// 🔁️ Surfaces whose next turn must carry an `Event::PatchRejected`, retaining both
        /// the receiver revision and the contract rejection reason.
        pending_rejections: PendingSurfaceRejectionRegistry,
        retained_command_closes: semio_framework::kernel::CommandDriverRegistry<1>,
        queued_command_closes: semio_framework::kernel::CommandDriverRegistry<1>,
        command_document_closes: CommandDocumentRetirementRegistry,
        rejected_command_builds: semio_framework::kernel::RejectedCommandBuildRegistry<1>,
        rejected_events: Option<RejectedKernelEvents>,
        job_progress: JobProgressOverlayStore,
        replay_routes: [Option<MountedReplayRouteSeed>; JOB_PROGRESS_ACTIVE_CAPACITY],
        job_replays: [Option<MountedJobReplay>; JOB_PROGRESS_ACTIVE_CAPACITY],
        product_replay_claims: [Option<MountedProductReplayClaim>; JOB_PROGRESS_ACTIVE_CAPACITY],
        product_replay_authorities: [Option<MountedProductReplayAuthority>; JOB_PROGRESS_ACTIVE_CAPACITY],
        rejected_job_progress: [Option<JobProgressRejected>; 64],
        pending_job_progress_presentations: [Option<PendingJobProgressPresentation>; JOB_PROGRESS_PRESENTATION_CAPACITY],
        closing_apps: [Option<ClosingKernelApp>; JOB_PROGRESS_ACTIVE_CAPACITY],
        fault_closing_actors: [Option<ActorId>; JOB_PROGRESS_ACTIVE_CAPACITY],
        realm_progress_close_started: bool,
        semantic_close_document_lane: bool,
    }

    impl KernelPoolState {
        /// ⏱️ P3a (INTERACTIVE-JOB-RUNTIME-REFACTOR, ui-thread-isolation): every method in this
        /// state machine is genuinely asynchronous and the whole request loop is mounted once on the
        /// injected renderer worker pool. No executor bridge or dedicated kernel thread remains in
        /// product logic.
        async fn new(request_queue: Arc<KernelRequestQueue>) -> Self {
            let guest_runtime: Arc<GuestRuntimes> = Arc::new(GuestRuntimes::Owned(OwnedRuntime::new()));
            // 🧵️ P1e: the injected process-wide pool (`crate::renderer_worker_pool`), never a pool this
            // type mints for itself — see `ParallelRuntime::new`'s own doc.
            let pool = Arc::new(crate::renderer_worker_pool());
            let worker_count = u16::try_from(pool.worker_count()).unwrap_or(u16::MAX);
            let runtime = crate::parallel_runtime::ParallelRuntime::new(pool, guest_runtime.clone(), native_shard_count(), 2, 64).await;
            Self {
                request_queue,
                guest_runtime,
                runtime,
                worker_count,
                now_ms: 0,
                plugin_ordinals: HashMap::new(),
                instances: HashMap::new(),
                next_instance_id: 1,
                retained: RetainedSurfaceRegistry::new(),
                pending_rejections: PendingSurfaceRejectionRegistry::new(),
                retained_command_closes: semio_framework::kernel::CommandDriverRegistry::new(),
                queued_command_closes: semio_framework::kernel::CommandDriverRegistry::new(),
                command_document_closes: CommandDocumentRetirementRegistry::new(),
                rejected_command_builds: semio_framework::kernel::RejectedCommandBuildRegistry::new(),
                rejected_events: None,
                job_progress: JobProgressOverlayStore::new(),
                replay_routes: [None; JOB_PROGRESS_ACTIVE_CAPACITY],
                job_replays: std::array::from_fn(|_| None),
                product_replay_claims: std::array::from_fn(|_| None),
                product_replay_authorities: std::array::from_fn(|_| None),
                rejected_job_progress: std::array::from_fn(|_| None),
                pending_job_progress_presentations: std::array::from_fn(|_| None),
                closing_apps: std::array::from_fn(|_| None),
                fault_closing_actors: [None; JOB_PROGRESS_ACTIVE_CAPACITY],
                realm_progress_close_started: false,
                semantic_close_document_lane: false,
            }
        }

        fn mount_product_replay(&mut self, mut owner: MountedProductReplayRequest) -> Result<(), MountedProductReplayRequest> {
            let Some(&actor) = self.instances.get(&owner.instance) else { return Err(owner) };
            if owner.job == 0
                || owner.job_kind().is_empty()
                || owner.raw().job != owner.job
                || owner.raw().kind().as_bytes() != owner.job_kind()
                || owner.raw().placement != owner.placement
                || JobReplayRequest::from_spawn(owner.raw().kind(), owner.raw().input()) != owner.request
                || self.product_replay_claims.iter().flatten().any(|claim| claim.actor == actor && claim.job() == owner.job)
                || self.product_replay_authorities.iter().flatten().any(|authority| authority.actor == actor && authority.job() == owner.job)
            {
                return Err(owner);
            }
            let kind = std::str::from_utf8(owner.job_kind()).ok();
            let Some(kind) = kind else { return Err(owner) };
            let kind_identity = JobReplayRequest::from_spawn(kind, &[]);
            if kind_identity.controller != owner.request.controller || kind_identity.tool != owner.request.tool || kind_identity.schema != owner.request.schema || kind_identity.version != owner.request.version {
                return Err(owner);
            }
            let Some(index) = (0..JOB_PROGRESS_ACTIVE_CAPACITY).find(|index| self.product_replay_claims[*index].is_none() && self.product_replay_authorities[*index].is_none()) else { return Err(owner) };
            owner.raw_mut().acknowledge_mount();
            self.product_replay_claims[index] = Some(MountedProductReplayClaim { request: Some(owner), actor, record_cursor: 0, checkpoint: None });
            Ok(())
        }

        fn product_replay_pending_instance(&self) -> Option<u32> {
            self.product_replay_authorities.iter().flatten().next().map(MountedProductReplayAuthority::instance).or_else(|| self.product_replay_claims.iter().flatten().next().map(MountedProductReplayClaim::instance))
        }

        fn abort_product_replay_one(&mut self, instance: u32) {
            if let Some(index) = self.product_replay_authorities.iter().position(|authority| authority.as_ref().is_some_and(|authority| authority.instance() == instance)) {
                self.product_replay_authorities[index].take().expect("faulted product replay authority remains exact").retire();
            } else if let Some(index) = self.product_replay_claims.iter().position(|claim| claim.as_ref().is_some_and(|claim| claim.instance() == instance)) {
                self.product_replay_claims[index].take().expect("faulted product replay claim remains exact").retire();
            }
        }

        async fn advance_product_replay(&mut self, instance: u32) -> Result<ExchangeOutcome, String> {
            let idle = || ExchangeOutcome { frames: Vec::new(), surfaces: UiFixedList::default(), effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle };
            if let Some(index) = self.product_replay_authorities.iter().position(|authority| authority.as_ref().is_some_and(|authority| authority.instance() == instance)) {
                let (actor, job, profile_cursor, profile_started) = {
                    let authority = self.product_replay_authorities[index].as_ref().expect("selected product replay authority");
                    (authority.actor, authority.job(), authority.profile_cursor, authority.profile_started)
                };
                if profile_cursor == PRODUCT_REPLAY_PROFILE_COUNT {
                    self.product_replay_authorities[index].as_mut().expect("completed product replay authority remains exact").acknowledge_raw_retirement_boundary().map_err(str::to_string)?;
                    self.product_replay_authorities[index].take().expect("completed product replay authority remains exact").retire();
                    return Ok(idle());
                }
                let entry_index = self.job_replays.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.actor == actor && entry.authority.job == job)).ok_or_else(|| "kernel: product replay lost its mounted log".to_string())?;
                let process_slot = self.runtime.kernel().actor_record(actor).await.map_or(u16::MAX, |record| record.shard.0);
                let (turn, request, placement, route, checkpoint, terminal, expected_begin, expected_restore_start_ordinal) = {
                    let entry = self.job_replays[entry_index].as_ref().expect("qualified product replay log");
                    let terminal =
                        entry.log.record_header(entry.log.sealed_records().checked_sub(1).ok_or_else(|| "kernel: product replay has no terminal record".to_string())?).ok_or_else(|| "kernel: product replay terminal record is missing".to_string())?;
                    let checkpoint = entry.log.last_checkpoint_header().map(|header| MountedProductReplayCheckpoint {
                        ordinal: header.ordinal,
                        digest: header.payload_digest,
                        pages: u8::try_from(header.payload_bytes.saturating_add(semio_framework_actor::JOB_REPLAY_PAGE_BYTES - 1) / semio_framework_actor::JOB_REPLAY_PAGE_BYTES).unwrap_or(u8::MAX),
                        applied_progress: header.applied_progress,
                    });
                    let (expected_begin, expected_restore_start_ordinal) = if profile_started {
                        if entry.replay_started {
                            (false, entry.accepted_replay_sequence)
                        } else {
                            (true, entry.replay_submit_sequence)
                        }
                    } else {
                        (true, entry.accepted_replay_sequence)
                    };
                    (entry.authority, entry.request, entry.placement, entry.log.route(), checkpoint, terminal, expected_begin, expected_restore_start_ordinal)
                };
                let logical_worker_count = mounted_product_replay_profile(self.worker_count, profile_cursor).ok_or_else(|| "kernel: product replay profile is invalid".to_string())?;
                let logical_worker_slot = mounted_product_replay_worker_slot(process_slot, logical_worker_count).ok_or_else(|| "kernel: product replay slot is invalid".to_string())?;
                let expected = MountedProductReplayExpected {
                    instance,
                    actor,
                    turn,
                    request,
                    placement,
                    route,
                    checkpoint,
                    terminal,
                    process_worker_count: self.worker_count,
                    process_worker_slot: process_slot,
                    logical_worker_count,
                    logical_worker_slot,
                    begin: expected_begin,
                    restore_start_ordinal: expected_restore_start_ordinal,
                };
                let authority = self.product_replay_authorities[index].take().expect("selected product replay authority");
                match authority.validate(expected) {
                    Ok(authority) => self.product_replay_authorities[index] = Some(authority),
                    Err(authority) => {
                        self.product_replay_authorities[index] = Some(authority);
                        return Err("kernel: mounted product replay authority became stale; exact owner remains mounted".to_string());
                    }
                }
                if profile_started {
                    let (requested, complete, accepted_sequence) = {
                        let entry = self.job_replays[entry_index].as_ref().expect("qualified product replay log");
                        (entry.replay_requested, entry.log.replay_is_complete(), entry.accepted_replay_sequence)
                    };
                    if !requested && complete {
                        let authority = self.product_replay_authorities[index].as_mut().expect("completed product replay authority");
                        authority.profile_cursor += 1;
                        authority.profile_started = false;
                        authority.restore_start_ordinal = accepted_sequence;
                        if let Some(worker_count) = mounted_product_replay_profile(self.worker_count, authority.profile_cursor) {
                            authority.worker_count = worker_count;
                            authority.worker_slot = mounted_product_replay_worker_slot(process_slot, worker_count).ok_or_else(|| "kernel: next product replay slot is invalid".to_string())?;
                            authority.begin = true;
                        }
                        return Ok(idle());
                    }
                    let outcome = self.run_turn(actor, instance, Vec::new()).await?;
                    let (replay_started, restore_start_ordinal) = {
                        let entry = self.job_replays[entry_index].as_ref().expect("advanced product replay remains mounted");
                        (entry.replay_started, if entry.replay_started { entry.accepted_replay_sequence } else { entry.replay_submit_sequence })
                    };
                    let authority = self.product_replay_authorities[index].as_mut().expect("advanced product replay authority");
                    authority.begin = !replay_started;
                    authority.restore_start_ordinal = restore_start_ordinal;
                    return Ok(outcome);
                }
                let (worker_count, worker_slot, begin, restore_start_ordinal) = {
                    let authority = self.product_replay_authorities[index].as_ref().expect("starting product replay authority");
                    (authority.worker_count, authority.worker_slot, authority.begin, authority.restore_start_ordinal)
                };
                if !begin {
                    return Err("kernel: product replay start lost its qualified begin authority".to_string());
                }
                self.request_job_replay(actor, job, worker_count, worker_slot, restore_start_ordinal).await?;
                let authority = self.product_replay_authorities[index].as_mut().expect("starting product replay authority");
                authority.profile_started = true;
                authority.restore_start_ordinal = None;
                let outcome = self.run_turn(actor, instance, Vec::new()).await?;
                let (replay_started, restore_start_ordinal) = {
                    let entry = self.job_replays[entry_index].as_ref().expect("started product replay remains mounted");
                    (entry.replay_started, if entry.replay_started { entry.accepted_replay_sequence } else { entry.replay_submit_sequence })
                };
                let authority = self.product_replay_authorities[index].as_mut().expect("started product replay authority");
                authority.begin = !replay_started;
                authority.restore_start_ordinal = restore_start_ordinal;
                return Ok(outcome);
            }

            let Some(index) = self.product_replay_claims.iter().position(|claim| claim.as_ref().is_some_and(|claim| claim.instance() == instance)) else { return Ok(idle()) };
            let (actor, job, request) = {
                let claim = self.product_replay_claims[index].as_ref().expect("selected product replay claim");
                (claim.actor, claim.job(), claim.request().request)
            };
            let Some(entry_index) = self.job_replays.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.actor == actor && entry.authority.job == job)) else {
                return self.run_turn(actor, instance, Vec::new()).await;
            };
            let entry = self.job_replays[entry_index].as_ref().expect("selected mounted replay log");
            if entry.request != request {
                return Err("kernel: product replay request digest does not match the mounted session".to_string());
            }
            if entry.log.has_pending_work() || entry.captured.is_some() || entry.policy.is_some() {
                return Ok(idle());
            }
            if !entry.terminal_seen {
                return self.run_turn(actor, instance, Vec::new()).await;
            }
            let record_cursor = self.product_replay_claims[index].as_ref().expect("selected product replay claim").record_cursor;
            if record_cursor < entry.log.sealed_records() {
                let header = entry.log.record_header(record_cursor).ok_or_else(|| "kernel: product replay record cursor is stale".to_string())?;
                let claim = self.product_replay_claims[index].as_mut().expect("selected product replay claim");
                if header.kind == JobReplayPublicationKind::Checkpoint {
                    let pages = header.payload_bytes.saturating_add(semio_framework_actor::JOB_REPLAY_PAGE_BYTES - 1) / semio_framework_actor::JOB_REPLAY_PAGE_BYTES;
                    claim.checkpoint = Some(MountedProductReplayCheckpoint { ordinal: header.ordinal, digest: header.payload_digest, pages: u8::try_from(pages).unwrap_or(u8::MAX), applied_progress: header.applied_progress });
                }
                claim.record_cursor += 1;
                return Ok(idle());
            }
            let terminal = entry.log.record_header(entry.log.sealed_records().checked_sub(1).ok_or_else(|| "kernel: product replay log is empty".to_string())?).ok_or_else(|| "kernel: product replay terminal record is missing".to_string())?;
            if !matches!(terminal.kind, JobReplayPublicationKind::Commit | JobReplayPublicationKind::Cancelled | JobReplayPublicationKind::Fault) {
                return Err("kernel: product replay log has no typed terminal publication".to_string());
            }
            if terminal.prefix_digest != entry.log.prefix_digest() {
                return Err("kernel: product replay terminal prefix does not cover the retained log".to_string());
            }
            let expected_checkpoint = entry.log.last_checkpoint_header().map(|header| MountedProductReplayCheckpoint {
                ordinal: header.ordinal,
                digest: header.payload_digest,
                pages: u8::try_from(header.payload_bytes.saturating_add(semio_framework_actor::JOB_REPLAY_PAGE_BYTES - 1) / semio_framework_actor::JOB_REPLAY_PAGE_BYTES).unwrap_or(u8::MAX),
                applied_progress: header.applied_progress,
            });
            if self.product_replay_claims[index].as_ref().expect("qualified product replay claim").checkpoint != expected_checkpoint {
                return Err("kernel: product replay checkpoint witness does not match the retained log".to_string());
            }
            let route = entry.log.route();
            let operation = entry.authority.operation;
            let process_slot = self.runtime.kernel().actor_record(actor).await.map_or(u16::MAX, |record| record.shard.0);
            if terminal.worker_count != self.worker_count || terminal.worker_slot != process_slot {
                return Err("kernel: product replay physical process identity changed before qualification".to_string());
            }
            let worker_count = mounted_product_replay_profile(self.worker_count, 0).ok_or_else(|| "kernel: first product replay profile is invalid".to_string())?;
            let worker_slot = mounted_product_replay_worker_slot(process_slot, worker_count).ok_or_else(|| "kernel: first product replay slot is invalid".to_string())?;
            let mut claim = self.product_replay_claims[index].take().expect("qualified product replay claim");
            let mut request_owner = claim.request.take().expect("qualified product replay request transfers once");
            request_owner.raw_mut().acknowledge_qualification(claim.checkpoint);
            self.product_replay_authorities[index] = Some(MountedProductReplayAuthority {
                request_owner: Some(request_owner),
                actor,
                route,
                operation,
                generation: operation.generation,
                seed: operation.seed,
                checkpoint: claim.checkpoint,
                terminal,
                worker_count,
                worker_slot,
                begin: true,
                restore_start_ordinal: None,
                profile_cursor: 0,
                profile_started: false,
            });
            Ok(idle())
        }

        async fn request_job_replay(&mut self, actor: ActorId, job: u64, worker_count: u16, worker_slot: u16, restore_start_ordinal: Option<u64>) -> Result<(), String> {
            if !matches!(worker_count, 1 | 2 | 4) && worker_count != self.worker_count {
                return Err("kernel: replay worker profile is not admitted by the mounted shared process route".to_string());
            }
            let process_slot = self.runtime.kernel().actor_record(actor).await.map_or(u16::MAX, |record| record.shard.0);
            if mounted_product_replay_worker_slot(process_slot, worker_count) != Some(worker_slot) {
                return Err("kernel: replay worker slot does not match the qualified mounted process profile".to_string());
            }
            let entry = self.job_replays.iter_mut().flatten().find(|entry| entry.actor == actor && entry.authority.job == job).ok_or_else(|| "kernel: replay log is not mounted".to_string())?;
            if entry.accepted_replay_sequence != restore_start_ordinal {
                return Err("kernel: replay restore/start ordinal does not match the qualified previous profile".to_string());
            }
            if entry.replay_requested {
                if !entry.pending_replay_start_is_exact(worker_count, worker_slot) {
                    return Err("kernel: replay begin does not match the retained pending restore/start identity".to_string());
                }
                return Ok(());
            }
            if !entry.terminal_seen || entry.log.has_pending_work() || entry.captured.is_some() || entry.policy.is_some() {
                return Err("kernel: replay log is not sealed and quiescent".to_string());
            }
            entry.log.begin_replay(entry.authority.operation.generation).map_err(|fault| format!("kernel: replay begin rejected: {fault:?}"))?;
            entry.replay_requested = true;
            entry.replay_started = false;
            entry.replay_submit_sequence = None;
            entry.accepted_replay_sequence = None;
            entry.replay_worker_count = worker_count;
            entry.replay_worker_slot = worker_slot;
            Ok(())
        }

        fn begin_job_replay_capture(&mut self, actor: ActorId, authority: JobTurn, request: JobReplayRequest, placement: JobPlacement, worker_count: u16, worker_slot: u16, publication: JobPublication) {
            let replay_index = self.job_replays.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.actor == actor && entry.authority.job == authority.job)).or_else(|| self.job_replays.iter().position(Option::is_none));
            let Some(replay_index) = replay_index else {
                self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Capacity, publication));
                return;
            };
            if self.job_replays[replay_index].is_none() {
                let Some(seed) = self.replay_routes.iter().flatten().find(|seed| seed.actor == actor).copied() else {
                    self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Missing, publication));
                    return;
                };
                let Some(recovery) = mounted_replay_recovery_registry().lock().expect("replay recovery lock").reserve() else {
                    self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Capacity, publication));
                    return;
                };
                let Ok(log) = JobReplayLog::new(seed.compose(request), authority.operation.generation) else {
                    assert!(mounted_replay_recovery_registry().lock().expect("replay recovery lock").release(recovery));
                    self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Stale, publication));
                    return;
                };
                self.job_replays[replay_index] = Some(MountedJobReplay {
                    actor,
                    authority,
                    request,
                    placement,
                    log: ManuallyDrop::new(log),
                    captured: None,
                    policy: None,
                    terminal_seen: false,
                    replay_requested: false,
                    replay_started: false,
                    replay_submit_sequence: None,
                    accepted_replay_sequence: None,
                    replay_worker_count: 0,
                    replay_worker_slot: u16::MAX,
                    recovery: Some(recovery),
                });
            }
            let entry = self.job_replays[replay_index].as_mut().expect("mounted replay slot");
            if entry.authority.operation.operation != authority.operation.operation
                || entry.authority.operation.base_revision != authority.operation.base_revision
                || entry.authority.operation.generation != authority.operation.generation
                || entry.authority.operation.seed != authority.operation.seed
                || entry.placement != placement
            {
                self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Stale, publication));
                return;
            }
            let terminal = matches!(&publication.outcome, semio_framework_actor::JobStepOutcome::Complete { .. } | semio_framework_actor::JobStepOutcome::Cancelled | semio_framework_actor::JobStepOutcome::Fault { .. });
            let now = semio_framework_job::default_now_us();
            let mut preview_sequence = publication.turn.operation.preview_sequence;
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(publication.turn.operation.operation),
                semio_framework_job::Generation(publication.turn.operation.generation),
                now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::INTERACTIVE_LANE_WALL_US)).unwrap_or(semio_framework_job::StepBudget::new(0, 0)),
                semio_framework_job::root_cancel_token(),
                semio_framework_job::default_now_us,
                &mut preview_sequence,
            );
            match entry.log.begin_capture(&mut context, actor, worker_count, worker_slot, TURN_BUDGET.fuel, TURN_BUDGET.deadline_ms, publication) {
                Ok(()) => entry.terminal_seen |= terminal,
                Err(rejected) => {
                    let fault = match rejected.fault {
                        JobReplayFault::Stale | JobReplayFault::Sequence | JobReplayFault::Mismatch | JobReplayFault::Exhausted => semio_framework_actor::JobProgressFault::Stale,
                        JobReplayFault::Cancelled => semio_framework_actor::JobProgressFault::Cancelled,
                        JobReplayFault::Deadline => semio_framework_actor::JobProgressFault::Budget,
                        JobReplayFault::Busy => semio_framework_actor::JobProgressFault::Busy,
                        JobReplayFault::Capacity | JobReplayFault::PageBytes | JobReplayFault::ProcessPages => semio_framework_actor::JobProgressFault::Capacity,
                    };
                    self.retain_job_progress_rejection(JobProgressRejected::new(fault, rejected.into_publication()));
                }
            }
        }

        fn job_replay_maintenance_step(&mut self) -> bool {
            let Some(index) = self.job_replays.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.log.has_pending_work() || entry.captured.is_some() || entry.policy.is_some())) else {
                return false;
            };
            if let Some(policy) = self.job_replays[index].as_mut().expect("mounted replay").policy.take() {
                let entry = self.job_replays[index].as_mut().expect("mounted replay");
                let operation = entry.authority.operation;
                let now = semio_framework_job::default_now_us();
                let mut preview_sequence = operation.preview_sequence;
                let mut context = semio_framework_job::StepContext::new(
                    semio_framework_job::OperationId(operation.operation),
                    semio_framework_job::Generation(operation.generation),
                    now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::MAINTENANCE_LANE_WALL_US)).unwrap_or(semio_framework_job::StepBudget::new(0, 0)),
                    semio_framework_job::root_cancel_token(),
                    semio_framework_job::default_now_us,
                    &mut preview_sequence,
                );
                if entry.log.acknowledge_publication(&mut context, policy).is_err() {
                    entry.log.begin_close();
                }
                return true;
            }
            if let Some(publication) = self.job_replays[index].as_mut().expect("mounted replay").captured.take() {
                let entry = self.job_replays[index].as_ref().expect("mounted replay");
                let (actor, authority) = (entry.actor, entry.authority);
                let policy = self.publish_captured_job_progress(actor, authority, publication);
                self.job_replays[index].as_mut().expect("mounted replay").policy = Some(policy);
                return true;
            }
            if self.job_replays[index].as_ref().expect("mounted replay").log.publication_is_ready() {
                let publication = self.job_replays[index].as_mut().expect("mounted replay").log.take_captured_publication();
                self.job_replays[index].as_mut().expect("mounted replay").captured = publication;
                return true;
            }
            let operation = self.job_replays[index].as_ref().expect("mounted replay").authority.operation;
            let now = semio_framework_job::default_now_us();
            let mut preview_sequence = operation.preview_sequence;
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(operation.operation),
                semio_framework_job::Generation(operation.generation),
                now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::INTERACTIVE_LANE_WALL_US)).unwrap_or(semio_framework_job::StepBudget::new(0, 0)),
                semio_framework_job::root_cancel_token(),
                semio_framework_job::default_now_us,
                &mut preview_sequence,
            );
            if self.job_replays[index].as_ref().expect("mounted replay").log.close_started() {
                let _ = self.job_replays[index].as_mut().expect("mounted replay").log.close_step(&mut context);
                if self.job_replays[index].as_ref().expect("mounted replay").log.terminal_is_empty() {
                    let terminal = self.job_replays[index].take().expect("terminal mounted replay");
                    drop(terminal);
                }
                return true;
            }
            if matches!(self.job_replays[index].as_mut().expect("mounted replay").log.maintenance_step(&mut context), JobReplayStep::Refused(_)) {
                self.job_replays[index].as_mut().expect("mounted replay").log.begin_close();
            }
            true
        }

        fn command_maintenance_step(&mut self) -> bool {
            if mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").has_close_work() {
                let _ = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").close_one();
                return !self.command_maintenance_pending();
            }
            if mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work() {
                let step = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(true);
                if let Some(owner) = step.request {
                    if let Err(owner) = self.mount_product_replay(owner) {
                        owner.retire();
                    }
                }
                let _ = step.progressed;
                return !self.command_maintenance_pending();
            }
            if mounted_replay_recovery_registry().lock().expect("replay recovery lock").has_close_work() {
                let _ = mounted_replay_recovery_registry().lock().expect("replay recovery lock").close_one();
                return !self.command_maintenance_pending();
            }
            if self.job_replay_maintenance_step() {
                return !self.command_maintenance_pending();
            }
            if let Some(close_index) = self.fault_closing_actors.iter().position(Option::is_some) {
                let actor = self.fault_closing_actors[close_index].expect("fault close actor");
                if let Some(pending_index) = self.pending_job_progress_presentations.iter().position(|pending| pending.as_ref().is_some_and(|pending| pending.receipt.identity().actor == actor)) {
                    let token = self.pending_job_progress_presentations[pending_index].as_ref().expect("fault pending presentation").token;
                    if !job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").cancel(token) {
                        return false;
                    }
                    let pending = self.pending_job_progress_presentations[pending_index].take().expect("cancelled fault presentation");
                    if self.job_progress.abort(pending.receipt).is_err() {
                        let _ = self.job_progress.begin_close_actor(actor);
                    }
                    return false;
                }
                if self.job_progress.begin_close_actor(actor).is_ok() {
                    self.fault_closing_actors[close_index] = None;
                }
                return false;
            }
            if self.job_progress.has_close_work() {
                let now = semio_framework_job::default_now_us();
                let mut preview_sequence = 0;
                let mut context = semio_framework_job::StepContext::new(
                    semio_framework_job::OperationId(0),
                    semio_framework_job::Generation(0),
                    now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::MAINTENANCE_LANE_WALL_US)).unwrap_or(semio_framework_job::StepBudget::new(0, 0)),
                    semio_framework_job::root_cancel_token(),
                    semio_framework_job::default_now_us,
                    &mut preview_sequence,
                );
                let _ = self.job_progress.close_step(&mut context);
                return !self.command_maintenance_pending();
            }
            if let Some(index) = self.rejected_job_progress.iter().position(Option::is_some) {
                let rejected = self.rejected_job_progress[index].take().expect("fixed rejected job-progress slot is occupied");
                if let Err(rejected) = self.job_progress.retain_rejected(rejected) {
                    self.rejected_job_progress[index] = Some(rejected);
                }
                return !self.command_maintenance_pending();
            }
            if self.command_document_closes.has_close_work() {
                let _ = self.command_document_closes.close_one();
                return !self.command_maintenance_pending();
            }
            if self.retained_command_closes.has_close_work() {
                let _ = self.retained_command_closes.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                return !self.retained_command_closes.has_close_work() && !self.queued_command_closes.has_close_work() && self.rejected_command_builds.terminal_is_empty();
            }
            if self.queued_command_closes.has_close_work() {
                let _ = self.queued_command_closes.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES);
                return !self.queued_command_closes.has_close_work() && self.rejected_command_builds.terminal_is_empty();
            }
            if !self.rejected_command_builds.terminal_is_empty() {
                return self.rejected_command_builds.close_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES).0;
            }
            if let Some(owner) = self.rejected_events.as_mut() {
                let (terminal, _) = owner.close_step();
                if terminal {
                    let terminal = self.rejected_events.take().expect("terminal rejected event owner is present");
                    assert!(terminal.terminal_is_empty(), "rejected event terminal witness changed before removal");
                }
                return self.rejected_events.is_none();
            }
            true
        }

        fn command_maintenance_pending(&self) -> bool {
            mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").has_close_work()
                || mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work()
                || mounted_replay_recovery_registry().lock().expect("replay recovery lock").has_close_work()
                || self.fault_closing_actors.iter().flatten().any(|actor| {
                    self.pending_job_progress_presentations
                        .iter()
                        .find(|pending| pending.as_ref().is_some_and(|pending| pending.receipt.identity().actor == *actor))
                        .is_none_or(|pending| job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").can_cancel(pending.as_ref().expect("matching pending presentation").token))
                })
                || self.job_progress.has_close_work()
                || self.job_replays.iter().flatten().any(|entry| entry.log.has_pending_work() || entry.captured.is_some() || entry.policy.is_some())
                || self.rejected_job_progress.iter().any(Option::is_some)
                || self.retained_command_closes.has_close_work()
                || self.queued_command_closes.has_close_work()
                || self.command_document_closes.has_close_work()
                || !self.rejected_command_builds.terminal_is_empty()
                || self.rejected_events.is_some()
        }

        fn begin_fault_close(&mut self, actor: ActorId) {
            if self.fault_closing_actors.iter().flatten().any(|closing| *closing == actor) {
                return;
            }
            if let Some(slot) = self.fault_closing_actors.iter_mut().find(|slot| slot.is_none()) {
                *slot = Some(actor);
            } else {
                self.job_progress.begin_close_all();
            }
        }

        fn acknowledge_job_progress(&mut self, token: JobProgressPresentationToken) {
            if !job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").presented(token) {
                return;
            }
            let Some(pending) = self.pending_job_progress_presentations[token.index].take() else {
                let _ = job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").release_presented(token);
                return;
            };
            if pending.token != token {
                self.pending_job_progress_presentations[token.index] = Some(pending);
                return;
            }
            let receipt = pending.receipt;
            if let Err((fault, receipt)) = self.job_progress.acknowledge(receipt) {
                let actor = receipt.identity().actor;
                if self.job_progress.abort(receipt).is_err() {
                    let _ = self.job_progress.begin_close_actor(actor);
                }
                crate::log_debug(&format!("kernel: presented job-progress ACK failed closed: {fault}"));
            }
            let _ = job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").release_presented(token);
        }

        fn begin_job_progress(&mut self, actor: ActorId, turn: &JobTurn) -> Result<bool, String> {
            let live = JobProgressLiveAuthority::new(turn.operation.operation, turn.operation.base_revision, turn.operation.generation);
            if self.job_progress.live_authority(actor, turn.job) == Some(live) {
                return Ok(false);
            }
            self.job_progress.begin_operation(actor, turn.job, live).map_err(|fault| fault.to_string())?;
            Ok(true)
        }

        fn retain_job_progress_rejection(&mut self, rejected: JobProgressRejected) {
            let rejected = match self.job_progress.retain_rejected(rejected) {
                Ok(()) => return,
                Err(rejected) => rejected,
            };
            let slot = self.rejected_job_progress.iter_mut().find(|slot| slot.is_none()).expect("one kernel decision cannot exceed the fixed 64-actor rejected-publication handback registry");
            *slot = Some(rejected);
        }

        fn publish_captured_job_progress(&mut self, actor: ActorId, authority: JobTurn, publication: JobPublication) -> JobReplayPublicationPolicy {
            let live = JobProgressLiveAuthority::new(authority.operation.operation, authority.operation.base_revision, authority.operation.generation);
            let stable_identity_matches = authority.step_sequence == 0
                && authority.operation.preview_sequence == 0
                && authority.job == publication.turn.job
                && authority.operation.operation == publication.turn.operation.operation
                && authority.operation.base_revision == publication.turn.operation.base_revision
                && authority.operation.generation == publication.turn.operation.generation;
            if !stable_identity_matches {
                self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Stale, publication));
                return JobReplayPublicationPolicy::Rejected;
            }
            match self.job_progress.live_authority(actor, authority.job) {
                Some(expected) if expected == live => {}
                None => {
                    if let Err(fault) = self.job_progress.begin_operation(actor, authority.job, live) {
                        self.retain_job_progress_rejection(JobProgressRejected::new(fault, publication));
                        return JobReplayPublicationPolicy::Rejected;
                    }
                }
                Some(_) => {
                    self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Stale, publication));
                    return JobReplayPublicationPolicy::Rejected;
                }
            }
            let now = semio_framework_job::default_now_us();
            let mut preview_sequence = publication.turn.operation.preview_sequence;
            let mut context = semio_framework_job::StepContext::new(
                semio_framework_job::OperationId(publication.turn.operation.operation),
                semio_framework_job::Generation(publication.turn.operation.generation),
                now.and_then(|now| semio_framework_job::StepBudget::from_duration(2, now, semio_framework_job::INTERACTIVE_LANE_WALL_US)).unwrap_or(semio_framework_job::StepBudget::new(0, 0)),
                semio_framework_job::root_cancel_token(),
                semio_framework_job::default_now_us,
                &mut preview_sequence,
            );
            let admission = match self.job_progress.preflight(&mut context, actor, &publication, live) {
                Ok(admission) => admission,
                Err(fault) => {
                    self.retain_job_progress_rejection(JobProgressRejected::new(fault, publication));
                    return JobReplayPublicationPolicy::Rejected;
                }
            };
            let identity = JobProgressIdentity::from_publication(actor, &publication);
            let kind = match &publication.outcome {
                semio_framework_actor::JobStepOutcome::Yield => JobProgressKind::Yield,
                semio_framework_actor::JobStepOutcome::PreviewReady { .. } => JobProgressKind::Preview,
                semio_framework_actor::JobStepOutcome::CheckpointReady { .. } => JobProgressKind::Checkpoint,
                semio_framework_actor::JobStepOutcome::Complete { .. } => JobProgressKind::CommitValidated,
                semio_framework_actor::JobStepOutcome::Cancelled => JobProgressKind::Cancelled,
                semio_framework_actor::JobStepOutcome::Fault { .. } => JobProgressKind::Fault,
            };
            let displaced = kind == JobProgressKind::Preview && self.job_progress.has_visible_preview(actor, authority.job);
            let applied_progress = match &publication.outcome {
                semio_framework_actor::JobStepOutcome::CheckpointReady { checkpoint } => checkpoint.applied_progress,
                _ => publication.turn.step_sequence,
            };
            let Some(presentation_token) = job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").reserve(identity, kind, applied_progress) else {
                self.retain_job_progress_rejection(JobProgressRejected::new(semio_framework_actor::JobProgressFault::Busy, publication));
                return JobReplayPublicationPolicy::Rejected;
            };
            let receipt = match self.job_progress.publish_reserved(&mut context, admission, publication, live) {
                Ok(receipt) => receipt,
                Err(rejected) => {
                    let _ = job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").cancel(presentation_token);
                    self.retain_job_progress_rejection(rejected);
                    return JobReplayPublicationPolicy::Rejected;
                }
            };
            if self.pending_job_progress_presentations[presentation_token.index].is_some() || !job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").publish(presentation_token) {
                let _ = job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").cancel(presentation_token);
                let actor = receipt.identity().actor;
                if self.job_progress.abort(receipt).is_err() {
                    let _ = self.job_progress.begin_close_actor(actor);
                }
                return JobReplayPublicationPolicy::Rejected;
            }
            self.pending_job_progress_presentations[presentation_token.index] = Some(PendingJobProgressPresentation { token: presentation_token, receipt });
            if displaced {
                JobReplayPublicationPolicy::Displaced
            } else {
                JobReplayPublicationPolicy::Accepted
            }
        }

        fn plugin_ordinal(&mut self, plugin_id: &str) -> u16 {
            let next = self.plugin_ordinals.len() as u16;
            *self.plugin_ordinals.entry(plugin_id.to_string()).or_insert(next)
        }

        async fn create_app(&mut self, wasm_path: PathBuf, plugin_id: String, app_id: String) -> Result<u32, String> {
            let replay_route_index = self.replay_routes.iter().position(Option::is_none).ok_or_else(|| "kernel: fixed replay route registry is full".to_string())?;
            let mut bytes_owner = match crate::run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadBytes(wasm_path.clone())).await? {
                semio_framework_os_services::NativeIoValue::Bytes(bytes) => bytes,
                _ => return Err("kernel: native I/O returned the wrong value for wasm read".into()),
            };
            let bytes = bytes_owner.single_page().ok_or_else(|| "kernel: populated Wasm exceeds the mounted single-page retained compiler authority".to_string())?;
            let hash = PackageHash(*semio_framework_hash::hash(bytes).as_bytes());
            let plugin_digest = JobReplayRequest::from_spawn(&plugin_id, &[]).tool;
            let document_digest = JobReplayRequest::from_spawn(&app_id, &[]).tool;
            let package_id = PackageId(plugin_id.clone());
            let package_ref = PackageRef { package: package_id.clone(), hash };
            // 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): compile
            // remains a genuine suspension point on the worker-pool-owned request state machine.
            let compiled = self.guest_runtime.compile(&package_ref, bytes).await.map_err(|error| error.to_string());
            let _ = bytes_owner.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            let compiled = compiled?;
            let instance_id = self.next_instance_id;
            self.next_instance_id += 1;
            let plugin_ordinal = self.plugin_ordinal(&plugin_id);
            let actor = self
                .runtime
                .activate(
                    package_id.clone(),
                    plugin_ordinal,
                    ActorKind::PluginApp { plugin: package_id, app_id: app_id.clone(), instance_id },
                    Lane::Interactive,
                    None,
                    ActivationEvent::Manual,
                    &compiled,
                    &[] as &[BrokerCapabilityGrant],
                    &TURN_BUDGET,
                )
                .await?;
            self.replay_routes[replay_route_index] = Some(MountedReplayRouteSeed { actor, plugin: plugin_digest, package: hash.0, window: u64::from(instance_id), document: document_digest });
            self.instances.insert(instance_id, actor);
            // 🐣️ `InstanceOpen` is the first event a fresh instance must receive (`📓️design-abi.md`
            // §2) — `actor`/`config`/`assets`/`capabilities` are placeholders until a real capability
            // broker/asset-preload pipeline lands (A2b/T1 territory, not this packet's).
            let open = Event::InstanceOpen {
                instance: semio_framework::kernel::PluginInstanceId(instance_id.to_string()),
                app_id: semio_framework::kernel::AppInstanceId(app_id),
                actor: "local".to_string(),
                config: Vec::new(),
                assets: Vec::new(),
                capabilities: Vec::new(),
                quotas: QuotaSchema::default(),
            };
            self.run_turn(actor, instance_id, vec![open]).await?;
            // 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): descriptor-driven
            // native cascade — M6's own acceptance wording, "activating a parent brings up its N
            // extension actors." `wasm_path` is always `<modules_root>/<plugin_id>/<file>.wasm`
            // (`program_bridge::load_wasm_plugins`'s own layout convention), so the extensions' own
            // wasm artifacts live as siblings under the same `modules_root`.
            if let Some(modules_root) = wasm_path.parent().and_then(|dir| dir.parent()) {
                self.activate_extensions_of(&plugin_id, actor, modules_root).await;
            }
            Ok(instance_id)
        }

        /// 🧩️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-extension-activation): for every
        /// descriptor in the generated registry with `extends == plugin_id`, compile (`guest_runtime.
        /// compile` is itself keyed/cached by `PackageRef` content hash — no extra cache layer needed
        /// here) and activate it as `ActorKind::Extension`, `Lane::Background` (design doc M6's
        /// decided default — a UI-contributing extension is re-laned on `SurfaceVisible` by whichever
        /// packet wires that event), with `scoped_grants` = [`semio_framework_actor::
        /// intersect_capabilities`] of the parent's own granted set against the extension's requests
        /// — the never-escalate-past-the-parent property. Records the [`semio_framework_actor::Kernel::
        /// link_extension`] cascade edge so [`Self::destroy_app`] takes every extension down with its
        /// parent. Best-effort per extension (mirrors `program_bridge::load_wasm_plugins`'s own "one
        /// bad plugin does not hold the batch hostage" policy) — a missing/broken extension is logged
        /// and skipped, never fails the parent's own `create_app`.
        ///
        /// 🕳️ Honest gap, not worked around: this activates the extension via `ParallelRuntime::
        /// activate` (least-loaded shard), NOT pinned to the parent's exact shard — `ParallelRuntime`
        /// has no `activate_pinned` entry point today (that facade lives in `🎯️targets/🧊️wgpu/📦️packages/🦀️rust/
        /// 🎠️runtime.rs`, owned by a different packet, `kernel-async-native`). The kernel-level
        /// primitive this method WOULD call (`Kernel::activate_pinned`) is built, tested, and green in
        /// `semio-framework-actor`; only the host-level plumbing to reach it through `ParallelRuntime`
        /// is missing. A lease-request for a small additive method is open — see this ticket's report.
        /// `link_extension` (cascade topology, zero-orphan teardown) is unaffected by this gap and
        /// works correctly regardless of which shard the extension landed on.
        async fn activate_extensions_of(&mut self, plugin_id: &str, parent: ActorId, modules_root: &std::path::Path) {
            let extensions = extension_index().extensions_of(plugin_id);
            if extensions.is_empty() {
                return;
            }
            let parent_grants = self.runtime.kernel().actor_record(parent).await.map(|record| record.capabilities).unwrap_or_default();
            for extension in extensions.to_vec() {
                let Some(replay_route_index) = self.replay_routes.iter().position(Option::is_none) else {
                    crate::log_debug("kernel: fixed replay route registry is full; extension activation refused");
                    break;
                };
                let Some(parent_route) = self.replay_routes.iter().flatten().find(|route| route.actor == parent).copied() else {
                    crate::log_debug("kernel: parent replay route disappeared before extension activation");
                    break;
                };
                let extension_dir = modules_root.join(&extension.extension_id);
                let Some(extension_wasm_path) = find_wasm_artifact(&extension_dir).await else {
                    crate::log_debug(&format!("kernel: extension {} of {plugin_id} has no compiled wasm under {}, skipping", extension.extension_id, extension_dir.display()));
                    continue;
                };
                let mut extension_bytes_owner = match crate::run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadBytes(extension_wasm_path.clone())).await {
                    Ok(semio_framework_os_services::NativeIoValue::Bytes(bytes)) => bytes,
                    Ok(_) => {
                        crate::log_debug(&format!("kernel: native I/O returned the wrong value for extension {}", extension.extension_id));
                        continue;
                    }
                    Err(error) => {
                        crate::log_debug(&format!("kernel: failed reading extension {} wasm ({}): {error}", extension.extension_id, extension_wasm_path.display()));
                        continue;
                    }
                };
                let Some(extension_bytes) = extension_bytes_owner.single_page() else {
                    crate::log_debug(&format!("kernel: extension {} exceeds the mounted single-page retained compiler authority", extension.extension_id));
                    continue;
                };
                let extension_hash = PackageHash(*semio_framework_hash::hash(extension_bytes).as_bytes());
                let extension_package_ref = PackageRef { package: extension.package.clone(), hash: extension_hash };
                let extension_compiled = self.guest_runtime.compile(&extension_package_ref, extension_bytes).await;
                let _ = extension_bytes_owner.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
                let extension_compiled = match extension_compiled {
                    Ok(handle) => handle,
                    Err(error) => {
                        crate::log_debug(&format!("kernel: compile failed for extension {}: {error}", extension.extension_id));
                        continue;
                    }
                };
                let extension_ordinal = self.plugin_ordinal(&extension.extension_id);
                let extension_kind = ActorKind::Extension { plugin: PackageId(plugin_id.to_string()), extension_id: extension.extension_id.clone() };
                // 🕳️ Honest gap: the REAL capability enforcement point for a guest instance is the
                // `caps: &[BrokerCapabilityGrant]` argument below, `&[]` here because this native host
                // has no capability broker wired up for ANY actor kind yet — the parent's own
                // activation above passes the identical empty placeholder (A2b/T1 territory). The
                // `intersect_capabilities` call still records the correctly-scoped grant kernel-side
                // (`set_capabilities` below) so the intersection mechanism is exercised end-to-end and
                // ready the moment a broker starts populating `parent_grants` for real.
                match self.runtime.activate(extension.package.clone(), extension_ordinal, extension_kind, Lane::Background, None, ActivationEvent::Manual, &extension_compiled, &[] as &[BrokerCapabilityGrant], &TURN_BUDGET).await {
                    Ok(extension_actor) => {
                        self.replay_routes[replay_route_index] =
                            Some(MountedReplayRouteSeed { actor: extension_actor, plugin: JobReplayRequest::from_spawn(&extension.extension_id, &[]).tool, package: extension_hash.0, window: parent_route.window, document: parent_route.document });
                        let scoped_grants = intersect_capabilities(&parent_grants, &extension.capability_requests).await;
                        if let Err(error) = self.runtime.kernel_mut().set_capabilities(extension_actor, scoped_grants).await {
                            crate::log_debug(&format!("kernel: set_capabilities({extension_actor:?}) failed: {error}"));
                        }
                        if let Err(error) = self.runtime.kernel_mut().link_extension(parent, extension_actor).await {
                            crate::log_debug(&format!("kernel: link_extension({parent:?}, {extension_actor:?}) failed: {error}"));
                        }
                    }
                    Err(error) => crate::log_debug(&format!("kernel: activate failed for extension {}: {error}", extension.extension_id)),
                }
            }
        }

        async fn destroy_app_step(&mut self, instance: u32) -> bool {
            if let Some(index) = self.product_replay_claims.iter().position(|claim| claim.as_ref().is_some_and(|claim| claim.instance() == instance)) {
                self.product_replay_claims[index].take().expect("destroyed app product replay claim remains exact").retire();
                return false;
            }
            if let Some(index) = self.product_replay_authorities.iter().position(|authority| authority.as_ref().is_some_and(|authority| authority.instance() == instance)) {
                self.product_replay_authorities[index].take().expect("destroyed app product replay authority remains exact").retire();
                return false;
            }
            if mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_instance(instance) {
                let _ = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").close_instance_one(instance);
                return false;
            }
            if mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").has_instance(instance) {
                let _ = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").close_instance_one(instance);
                return false;
            }
            let _ = self.retained_command_closes.begin_close_key(u64::from(instance));
            let _ = self.queued_command_closes.begin_close_key(u64::from(instance));
            self.command_document_closes.begin_close_instance(u64::from(instance));
            let close_index = self.closing_apps.iter().position(|slot| slot.as_ref().is_some_and(|close| close.instance == instance));
            let close_index = match close_index {
                Some(index) => index,
                None => {
                    let Some(actor) = self.instances.remove(&instance) else { return true };
                    let removed = self.runtime.kernel_mut().deactivate(actor).await.unwrap_or_else(|_| vec![actor]);
                    if removed.len() > JOB_PROGRESS_ACTIVE_CAPACITY {
                        let _ = self.job_progress.begin_close_all();
                        return false;
                    }
                    let Some(index) = self.closing_apps.iter().position(Option::is_none) else {
                        let _ = self.job_progress.begin_close_all();
                        return false;
                    };
                    let mut actors = [ActorId(0); JOB_PROGRESS_ACTIVE_CAPACITY];
                    let actor_count = removed.len();
                    actors[..actor_count].copy_from_slice(&removed);
                    self.closing_apps[index] = Some(ClosingKernelApp { instance, actors, actor_count, replay_begin_cursor: 0, begin_cursor: 0, unregister_cursor: 0, route_cursor: 0 });
                    return false;
                }
            };
            let mut closing = self.closing_apps[close_index].take().expect("closing app slot");
            if let Some(index) = self.pending_job_progress_presentations.iter().position(|pending| pending.as_ref().is_some_and(|pending| closing.contains(pending.receipt.identity().actor))) {
                let token = self.pending_job_progress_presentations[index].as_ref().expect("pending presentation").token;
                if !job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").cancel(token) {
                    self.closing_apps[close_index] = Some(closing);
                    return false;
                }
                let pending = self.pending_job_progress_presentations[index].take().expect("cancelled pending presentation");
                let actor = pending.receipt.identity().actor;
                if self.job_progress.abort(pending.receipt).is_err() {
                    let _ = self.job_progress.begin_close_actor(actor);
                }
                self.closing_apps[close_index] = Some(closing);
                return false;
            }
            if closing.replay_begin_cursor < closing.actor_count {
                let actor = closing.actors[closing.replay_begin_cursor];
                if let Some(entry) = self.job_replays.iter_mut().flatten().find(|entry| entry.actor == actor && !entry.log.close_started()) {
                    entry.log.begin_close();
                } else {
                    closing.replay_begin_cursor += 1;
                }
                self.closing_apps[close_index] = Some(closing);
                return false;
            }
            if closing.begin_cursor < closing.actor_count {
                let actor = closing.actors[closing.begin_cursor];
                if self.job_progress.begin_close_actor(actor).is_ok() {
                    closing.begin_cursor += 1;
                }
                self.closing_apps[close_index] = Some(closing);
                return false;
            }
            if closing.unregister_cursor < closing.actor_count {
                self.runtime.unregister(closing.actors[closing.unregister_cursor]).await;
                closing.unregister_cursor += 1;
                self.closing_apps[close_index] = Some(closing);
                return false;
            }
            if closing.actors[..closing.actor_count].iter().any(|actor| !self.job_progress.actor_terminal_is_empty(*actor)) {
                let _ = self.command_maintenance_step();
                self.closing_apps[close_index] = Some(closing);
                return false;
            }
            if closing.actors[..closing.actor_count].iter().any(|actor| self.job_replays.iter().flatten().any(|entry| entry.actor == *actor)) {
                let _ = self.command_maintenance_step();
                self.closing_apps[close_index] = Some(closing);
                return false;
            }
            if !self.command_document_closes.instance_is_empty(u64::from(instance)) {
                let _ = self.command_maintenance_step();
                self.closing_apps[close_index] = Some(closing);
                return false;
            }
            if !self.retained.advance_instance_close_one(instance) {
                self.closing_apps[close_index] = Some(closing);
                return false;
            }
            if self.pending_rejections.take_instance_one(instance).is_some() {
                self.closing_apps[close_index] = Some(closing);
                return false;
            }
            if closing.route_cursor < closing.actor_count {
                let actor = closing.actors[closing.route_cursor];
                if let Some(index) = self.replay_routes.iter().position(|seed| seed.is_some_and(|seed| seed.actor == actor)) {
                    self.replay_routes[index] = None;
                }
                closing.route_cursor += 1;
                self.closing_apps[close_index] = Some(closing);
                return false;
            }
            self.pending_rejections.instance_is_empty(instance)
        }

        fn close_realm_progress_step(&mut self) -> bool {
            if let Some(index) = self.product_replay_claims.iter().position(Option::is_some) {
                self.product_replay_claims[index].take().expect("realm product replay claim remains exact").retire();
                return false;
            }
            if let Some(index) = self.product_replay_authorities.iter().position(Option::is_some) {
                self.product_replay_authorities[index].take().expect("realm product replay authority remains exact").retire();
                return false;
            }
            if mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_any() {
                let _ = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false);
                return false;
            }
            if mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").has_close_work() {
                let _ = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").close_one();
                return false;
            }
            if let Some(index) = self.pending_job_progress_presentations.iter().position(Option::is_some) {
                let token = self.pending_job_progress_presentations[index].as_ref().expect("realm pending presentation").token;
                if !job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").cancel(token) {
                    return false;
                }
                let pending = self.pending_job_progress_presentations[index].take().expect("cancelled realm presentation");
                let actor = pending.receipt.identity().actor;
                if self.job_progress.abort(pending.receipt).is_err() {
                    let _ = self.job_progress.begin_close_actor(actor);
                }
                return false;
            }
            if let Some(entry) = self.job_replays.iter_mut().flatten().find(|entry| !entry.log.close_started()) {
                entry.log.begin_close();
                return false;
            }
            if self.job_replays.iter().any(Option::is_some) {
                let _ = self.command_maintenance_step();
                return false;
            }
            if let Some(index) = self.replay_routes.iter().position(Option::is_some) {
                self.replay_routes[index] = None;
                return false;
            }
            if !self.realm_progress_close_started {
                self.job_progress.begin_close_all();
                self.realm_progress_close_started = true;
                return false;
            }
            if !self.job_progress.terminal_is_empty() || self.rejected_job_progress.iter().any(Option::is_some) || !job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").terminal_is_empty() {
                let _ = self.command_maintenance_step();
                return false;
            }
            if !self.retained.advance_realm_close_one() {
                return false;
            }
            if self.pending_rejections.take_any_one().is_some() {
                return false;
            }
            if !self.pending_rejections.terminal_is_empty() {
                return false;
            }
            self.realm_progress_close_started = false;
            true
        }

        async fn exchange(&mut self, instance: u32, mut events: Vec<Event>) -> Result<ExchangeOutcome, String> {
            let Some(&actor) = self.instances.get(&instance) else {
                return Err(format!("kernel: instance {instance} is not registered"));
            };
            if let Some(rejection) = self.pending_rejections.take_instance_one(instance) {
                events.insert(0, Event::PatchRejected { surface: rejection.surface.0, revision: rejection.revision.0, reason: rejection.reason.to_string() });
            }
            self.run_turn(actor, instance, events).await
        }

        async fn deliver_typed_operation_result_ack(&mut self, token: TypedOperationResultToken) -> Result<(), String> {
            let Some(&actor) = self.instances.get(&token.receiver) else {
                return Err("kernel: typed-operation ACK receiver is not registered".to_string());
            };
            let event = Event::Message { source: MessageEndpoint::Shell { instance: semio_framework::kernel::PluginInstanceId(token.receiver.to_string()) }, payload: TypedOperationResultPage::encode_ack(token) };
            let _ = self.run_turn(actor, token.receiver, vec![event]).await?;
            Ok(())
        }

        async fn exchange_commands(&mut self, instance: u32, driver: semio_framework::kernel::CommandBatchDriver) -> Result<ExchangeOutcome, String> {
            let key = u64::from(instance);
            let generation = driver.generation();
            if !self.retained_command_closes.terminal_is_empty() {
                if !self.queued_command_closes.can_insert(key) {
                    return Err("kernel: retained and queued command close registries are saturated; caller owner remains in the queue close lane".to_string());
                }
                self.queued_command_closes.insert_admitted(key, generation, driver);
                self.queued_command_closes.begin_close(key, generation).map_err(|fault| fault.to_string())?;
                let _ = self.command_maintenance_step();
                return Err("kernel: previous cancelled command owner is closing; incoming exact batch moved to the queued close lane".to_string());
            }
            if !self.retained_command_closes.can_insert(key) {
                return Err("kernel: retained command close registry is saturated or collided".to_string());
            }
            self.retained_command_closes.insert_admitted(key, generation, driver);
            let Some(&actor) = self.instances.get(&instance) else {
                self.retained_command_closes.begin_close(key, generation).map_err(|fault| fault.to_string())?;
                let _ = self.command_maintenance_step();
                return Err(format!("kernel: instance {instance} is not registered; exact command owner entered bounded close"));
            };
            let mut combined = ExchangeOutcome { frames: Vec::new(), surfaces: UiFixedList::default(), effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle };
            loop {
                let mut destinations = match self.command_document_closes.try_reserve_page(key, generation) {
                    Ok(destinations) => destinations,
                    Err(()) => {
                        self.command_document_closes.begin_close_batch(key, generation);
                        let _ = self.retained_command_closes.begin_close(key, generation);
                        let _ = self.command_maintenance_step();
                        return Err("kernel: command document destinations saturated before the next turn; retained batch and exact documents entered bounded close".to_string());
                    }
                };
                let page = self.retained_command_closes.with_driver_mut(key, generation, |driver| driver.next_page());
                let page = match page {
                    Ok(Ok(page)) => page,
                    Ok(Err(fault)) => {
                        self.command_document_closes.release(&mut destinations);
                        self.command_document_closes.begin_close_batch(key, generation);
                        let _ = self.retained_command_closes.begin_close(key, generation);
                        let _ = self.command_maintenance_step();
                        return Err(fault.to_string());
                    }
                    Err(fault) => {
                        self.command_document_closes.release(&mut destinations);
                        self.command_document_closes.begin_close_batch(key, generation);
                        let _ = self.retained_command_closes.begin_close(key, generation);
                        let _ = self.command_maintenance_step();
                        return Err(fault.to_string());
                    }
                };
                let events = match page {
                    Some((cursor, bytes)) => vec![Event::CommandIngressPage { cursor, bytes }],
                    None => vec![Event::Wake],
                };
                if let Err(fault) = self.retained_command_closes.prepare_suspend(key, generation) {
                    self.command_document_closes.release(&mut destinations);
                    self.command_document_closes.begin_close_batch(key, generation);
                    let _ = self.retained_command_closes.begin_close(key, generation);
                    let _ = self.command_maintenance_step();
                    return Err(fault.to_string());
                }
                let outcome = match self.run_turn(actor, instance, events).await {
                    Ok(outcome) => outcome,
                    Err(fault) => {
                        self.command_document_closes.release(&mut destinations);
                        let _ = self.retained_command_closes.resume(key, generation);
                        self.command_document_closes.begin_close_batch(key, generation);
                        let _ = self.retained_command_closes.begin_close(key, generation);
                        let _ = self.command_maintenance_step();
                        return Err(fault);
                    }
                };
                let ExchangeOutcome { frames, surfaces, effects, command_ingress } = outcome;
                for surface in surfaces {
                    self.command_document_closes.admit(&mut destinations, surface);
                }
                self.command_document_closes.release(&mut destinations);
                if let Err(fault) = self.retained_command_closes.resume(key, generation) {
                    self.command_document_closes.begin_close_batch(key, generation);
                    let _ = self.retained_command_closes.begin_close(key, generation);
                    let _ = self.command_maintenance_step();
                    return Err(fault.to_string());
                }
                let progress = match self.retained_command_closes.with_driver_mut(key, generation, |driver| driver.observe(&command_ingress, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES)) {
                    Ok(Ok(progress)) => progress,
                    Ok(Err(fault)) => {
                        self.command_document_closes.begin_close_batch(key, generation);
                        let _ = self.retained_command_closes.begin_close(key, generation);
                        let _ = self.command_maintenance_step();
                        return Err(fault.to_string());
                    }
                    Err(fault) => {
                        self.command_document_closes.begin_close_batch(key, generation);
                        let _ = self.retained_command_closes.begin_close(key, generation);
                        let _ = self.command_maintenance_step();
                        return Err(fault.to_string());
                    }
                };
                combined.frames.extend(frames);
                combined.effects.extend(effects);
                combined.command_ingress = command_ingress;
                match progress {
                    semio_framework::kernel::CommandBatchProgress::Complete => {
                        if let Err(fault) = self.retained_command_closes.remove_terminal(key, generation) {
                            self.command_document_closes.begin_close_batch(key, generation);
                            let _ = self.retained_command_closes.begin_close(key, generation);
                            let _ = self.command_maintenance_step();
                            return Err(fault.to_string());
                        }
                        self.command_document_closes.publish_batch(key, generation, &mut combined.surfaces);
                        return Ok(combined);
                    }
                    semio_framework::kernel::CommandBatchProgress::Faulted => {
                        self.command_document_closes.begin_close_batch(key, generation);
                        self.retained_command_closes.begin_close(key, generation).map_err(|fault| fault.to_string())?;
                        let _ = self.command_maintenance_step();
                        return Err("kernel: command ingress faulted; exact driver and surface-document owners entered incremental close".to_string());
                    }
                    semio_framework::kernel::CommandBatchProgress::PageReady | semio_framework::kernel::CommandBatchProgress::Waiting => {}
                }
            }
        }

        /// 🎠️ terra-kernel-loop: the real loop the packet brief's item 1 asks for — `Kernel::submit`
        /// (honouring `Backpressure`; a non-`Accept` result is logged rather than silently ignored,
        /// but does not abort the turn since `Coalesced`/`Dropped` both still leave AT LEAST one
        /// envelope queued and `Rejected` on a freshly-activated actor's own generous Interactive-lane
        /// mailbox should not occur in practice) → `Kernel::tick` → dispatch to the actor's OWN pinned
        /// logical shard executor on the shared pool → wait for that shard's `ShardOutcome` →
        /// `Kernel::complete` (closing the bridging
        /// gap this method's OWN doc comment used to flag as unreached) → hand the result to
        /// `apply_turn_result`. Loops `tick_and_dispatch` until nothing is left to grant — normally
        /// one iteration (this host submits for exactly one actor per call), but `Kernel::tick`'s DRR
        /// scheduler is global, so this stays correct if that ever changes.
        ///
        /// 🕳️ Honest gap: `Kernel::commit_frame`/`apply_scene_patch` are NOT called here —
        /// `KernelPoolState::activate` (via `ParallelRuntime::activate`) still passes `window: None`
        /// for every actor, so `Kernel`'s own `SceneStore` would stay permanently empty regardless;
        /// this host's UI pipeline already has its own frame-boundary mechanism (`retained`/
        /// `apply_ui_patch`, "item 4" of the original H3 packet). Wiring per-window `Kernel::
        /// commit_frame` for real would mean migrating THIS host's whole UI-patch pipeline onto
        /// `Kernel`'s `SceneStore`, a substantially larger, separate refactor out of this packet's
        /// scope (see `📓️terra-kernel-loop-report.md`'s own gaps section).
        async fn run_turn(&mut self, actor: ActorId, instance: u32, events: Vec<Event>) -> Result<ExchangeOutcome, String> {
            let mut envelopes = Vec::with_capacity(events.len().max(1));
            let mut replay_start_index = None;
            if events.is_empty() {
                let replay_index = self.job_replays.iter().position(|entry| entry.as_ref().is_some_and(|entry| entry.actor == actor && entry.replay_requested));
                let (sequence, payload) = if let Some(index) = replay_index {
                    let entry = self.job_replays[index].as_mut().expect("requested replay remains mounted");
                    let Some(turn) = entry.log.expected_turn() else {
                        entry.replay_requested = false;
                        entry.replay_submit_sequence = None;
                        return Ok(ExchangeOutcome { frames: Vec::new(), surfaces: UiFixedList::default(), effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle });
                    };
                    if entry.replay_started {
                        (next_seq()?, Payload::JobStep { turn })
                    } else {
                        let sequence = entry.replay_submission_sequence()?;
                        replay_start_index = Some(index);
                        (sequence, Payload::JobReplay { turn, request: entry.request, worker_count: entry.replay_worker_count, worker_slot: entry.replay_worker_slot })
                    }
                } else {
                    (next_seq()?, Payload::Event { bytes: serde_json::to_vec(&Event::Wake).map_err(|error| error.to_string())? })
                };
                envelopes.push(Envelope { to: actor, from: Origin::Kernel, lane: Lane::Interactive, seq: sequence, deadline_ms: None, coalesce: None, cancel_of: None, payload });
            } else {
                for event in &events {
                    envelopes.push(Envelope {
                        to: actor,
                        from: Origin::Kernel,
                        lane: Lane::Interactive,
                        seq: next_seq()?,
                        deadline_ms: None,
                        coalesce: None,
                        cancel_of: None,
                        payload: Payload::Event { bytes: serde_json::to_vec(event).map_err(|error| error.to_string())? },
                    });
                }
            }
            for envelope in &envelopes {
                let began_job = match &envelope.payload {
                    Payload::JobStep { turn } => self.begin_job_progress(actor, turn)?,
                    Payload::Cancel { .. } => {
                        self.job_progress.begin_close_actor(actor).map_err(|fault| fault.to_string())?;
                        false
                    }
                    _ => false,
                };
                if matches!(self.runtime.submit(envelope).await, Backpressure::Accept) {
                    if let Some(index) = replay_start_index {
                        let entry = self.job_replays[index].as_mut().expect("accepted replay start remains mounted");
                        entry.accept_replay_submission(envelope.seq)?;
                    }
                } else {
                    if began_job {
                        let _ = self.job_progress.begin_close_actor(actor);
                    }
                    crate::log_debug(&format!("kernel: run_turn submit for actor {} was not Accept-ed (mailbox pressure)", actor.0));
                    if replay_start_index.is_some() {
                        return Ok(ExchangeOutcome { frames: Vec::new(), surfaces: UiFixedList::default(), effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle });
                    }
                }
            }
            let mut turn_result: Option<TurnResult> = None;
            let mut fault: Option<String> = None;
            let mut replay_capture_started = false;
            loop {
                self.now_ms += 1;
                let decision = self.runtime.tick_and_dispatch(self.now_ms, |_actor| crate::actor_budget_from_turn_budget(TURN_BUDGET, Lane::Interactive)).await;
                if decision.run.is_empty() {
                    break;
                }
                let outcomes = self.runtime.wait_for_outcomes(decision.run.len(), RUN_TURN_OUTCOME_TIMEOUT);
                if outcomes.len() < decision.run.len() {
                    return Err("kernel: shard produced no outcome for this turn".to_string());
                }
                for outcome in outcomes {
                    match outcome {
                        ShardOutcome::Turn { actor: reported, result } => {
                            let _ = self.runtime.complete_actor(ActorId(reported), &result, self.now_ms).await;
                            if reported == actor.0 {
                                turn_result = Some(decode_actor_turn_result(&result, reported)?);
                            } else {
                                let _ = semio_framework::kernel::close_ui_turn_patch_transport_session_one(reported);
                                let _ = semio_framework::kernel::close_ui_turn_patch_transport_one();
                            }
                        }
                        ShardOutcome::Job { actor: reported, authority, request, placement, publication } => {
                            let worker_slot = self.runtime.kernel().actor_record(ActorId(reported)).await.map_or(u16::MAX, |record| record.shard.0);
                            self.begin_job_replay_capture(ActorId(reported), authority, request, placement, self.worker_count, worker_slot, publication);
                            replay_capture_started = true;
                        }
                        ShardOutcome::Resumed { actor: reported, operation } if reported == actor.0 => {
                            let valid = self.job_replays.iter().flatten().any(|entry| {
                                entry.actor == actor
                                    && entry.replay_started
                                    && entry.authority.operation.operation == operation.operation
                                    && entry.authority.operation.base_revision == operation.base_revision
                                    && entry.authority.operation.generation == operation.generation
                                    && entry.authority.operation.seed == operation.seed
                            });
                            if !valid {
                                return Err("kernel: replay restart returned stale operation identity".to_string());
                            }
                            replay_capture_started = true;
                        }
                        // 🎠️ terra-kernel-loop: a trap must ALSO reach `Kernel::complete` — otherwise
                        // the failure ladder (`FailureState::on_signal`) never sees it, staying just as
                        // inert for the trap path as `Kernel::complete` being uncalled at all used to
                        // leave it. `ShardOutcome::Fault` carries no `TurnResult` (no `fuel_used`, no
                        // `Effect`s — the turn never returned one), so a minimal `Faulted` `TurnResult`
                        // is synthesized from its `message` — the same shape `apply_turn_result`'s
                        // caller already treats a fault as `TurnStatus::Faulted` for retry purposes.
                        ShardOutcome::Fault { actor: reported, message } => {
                            self.begin_fault_close(ActorId(reported));
                            let faulted = semio_framework_actor::TurnResult {
                                ui_patches: Vec::new(),
                                effects: Vec::new(),
                                command_ingress: Vec::new(),
                                next_wake: None,
                                status: semio_framework_actor::TurnStatus::Faulted { detail: message.as_bytes().to_vec() },
                                usage: semio_framework_actor::Usage::default(),
                            };
                            let _ = self.runtime.complete_actor(ActorId(reported), &faulted, self.now_ms).await;
                            if reported == actor.0 {
                                fault = Some(message);
                            }
                        }
                        // 🚧️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (K1, landed mid-session):
                        // `ShardOutcome` also carries checkpoint/resume/cancel control responses.
                        // This kernel pool state machine does not consume those control responses;
                        // a `Job` publication is handled by the explicit owner-moving arm above.
                        // Any remaining control response reaching here — for `actor` OR any other
                        // actor `Kernel::tick` happened to grant in the SAME call — is silently
                        // ignored rather than aborting an otherwise-successful turn; unlike the
                        // ORIGINAL `Fault`/`Job` handling this replaces, this loop may observe outcomes
                        // for actors OTHER than `actor` (DRR is global), so those must not error out.
                        _ => {}
                    }
                }
                if replay_capture_started {
                    break;
                }
            }
            if let Some(message) = fault {
                return Err(message);
            }
            match turn_result {
                Some(result) => self.apply_turn_result(actor, instance, result).await,
                None if replay_capture_started => Ok(ExchangeOutcome { frames: Vec::new(), surfaces: UiFixedList::default(), effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle }),
                None => Err("kernel: shard produced no outcome for this turn".to_string()),
            }
        }

        async fn apply_turn_result(&mut self, actor: ActorId, instance: u32, mut result: TurnResult) -> Result<ExchangeOutcome, String> {
            // 🎠️ terra-kernel-loop: `Kernel::complete` (the bridge this doc comment used to flag as
            // unreached — "bridging the two needs a real pack-encode step this packet didn't reach")
            // is now genuinely called, from `run_turn`, for EVERY `ShardOutcome::Turn` a tick grants
            // (including `actor`'s own, before this method is even invoked) — so `Kernel`'s
            // failure-ladder/metrics bookkeeping is live for this host now, not skipped.
            let _ = actor;
            let mut frames = Vec::new();
            let mut effects = Vec::new();
            for effect in result.effects {
                if let Effect::SendMessage { target: MessageEndpoint::Shell { instance: target_instance }, payload } = &effect {
                    if target_instance.0 == instance.to_string() {
                        if let Some(page) = TypedOperationResultPage::decode_guest_message(payload) {
                            if page.token.receiver == instance {
                                let queue = self.request_queue.clone();
                                let acknowledge = Arc::new(move |token| queue.try_push(KernelRequest::AcknowledgeTypedOperationResult { token }, Arc::new(ResponseSlot::default()), None).is_ok());
                                let _ = publish_typed_operation_result_page(page, acknowledge);
                                continue;
                            }
                        }
                        if let Ok(frame) = protocol::decode_app_frame(payload).await {
                            frames.push(frame);
                            continue;
                        }
                    }
                }
                effects.push(effect);
            }
            let mut surfaces = UiFixedList::default();
            loop {
                match result.ui_patches.try_transfer_one(|patch| self.apply_ui_patch(instance, patch)) {
                    semio_framework::kernel::UiTurnPatchTransfer::Empty => break,
                    semio_framework::kernel::UiTurnPatchTransfer::Transferred(()) => {}
                    semio_framework::kernel::UiTurnPatchTransfer::Refused => return Err("renderer retained patch admission refused without consuming the exact turn owner".to_string()),
                }
            }
            let _ = self.advance_retained_one(instance, None, &mut surfaces);
            Ok(ExchangeOutcome { frames, surfaces, effects, command_ingress: result.command_ingress })
        }

        fn apply_ui_patch(&mut self, instance: u32, patch: KernelUiPatch) -> Result<(), KernelUiPatch> {
            let surface = patch.surface.clone();
            let index = match self.retained.try_admit(instance, surface.clone()) {
                Ok(index) => index,
                Err(surface) => {
                    let Ok(reservation) = reserve_seq() else {
                        return Err(patch);
                    };
                    let generation = reservation.generation();
                    let rejection = PendingSurfaceRejection { generation, instance, surface, revision: UiRevision::default(), reason: ui_contract::UiText::try_from_str("retained surface capacity exhausted").unwrap_or_default(), patch: Some(patch) };
                    match self.pending_rejections.try_admit(rejection) {
                        Ok(()) => {
                            let _ = reservation.commit();
                        }
                        Err(mut rejection) => {
                            return rejection.patch.take().map_or(Ok(()), Err);
                        }
                    }
                    return Ok(());
                }
            };
            let Some(slot) = self.retained.slots[index].as_mut() else { return Err(patch) };
            let rejected = slot.owner.admit_patch(patch).err().and_then(|patch| slot.owner.rejected_patches.try_push(patch).err());
            if let Some(patch) = rejected {
                let Ok(reservation) = reserve_seq() else {
                    return Err(patch);
                };
                let generation = reservation.generation();
                let rejection = PendingSurfaceRejection { generation, instance, surface, revision: UiRevision::default(), reason: ui_contract::UiText::try_from_str("retained patch queues exhausted").unwrap_or_default(), patch: Some(patch) };
                match self.pending_rejections.try_admit(rejection) {
                    Ok(()) => {
                        let _ = reservation.commit();
                    }
                    Err(mut rejection) => {
                        return rejection.patch.take().map_or(Ok(()), Err);
                    }
                }
            }
            Ok(())
        }

        fn advance_retained_surface_one(&mut self, instance: u32, surface: SurfaceId) -> Result<ExchangeOutcome, String> {
            let mut surfaces = UiFixedList::default();
            self.advance_retained_one(instance, Some(&surface), &mut surfaces)?;
            Ok(ExchangeOutcome { frames: Vec::new(), surfaces, effects: Vec::new(), command_ingress: semio_framework::kernel::CommandIngressStatus::Idle })
        }

        fn advance_retained_one(&mut self, instance: u32, requested: Option<&SurfaceId>, out: &mut UiFixedList<ExchangeSurfaceDocument, UI_DOCUMENT_LEASE_SLOTS>) -> Result<(), String> {
            let index = requested.and_then(|surface| self.retained.index(instance, surface)).or_else(|| {
                self.retained.slots.iter().position(|slot| {
                    slot.as_ref().is_some_and(|slot| {
                        slot.instance == instance
                            && (slot.owner.patch.is_some()
                                || !slot.owner.queued_patches.is_empty()
                                || !slot.owner.rejected_patches.is_empty()
                                || slot.owner.build.is_some()
                                || slot.owner.build_pending
                                || slot.owner.closing.is_some()
                                || slot.owner.exchange_closing.is_some())
                    })
                })
            });
            let Some(index) = index else {
                if let Some(surface) = requested {
                    return Err(format!("retained surface '{}' is not admitted", surface.as_ref()));
                }
                if self.semantic_close_document_lane {
                    let _ = ui_contract::close_ui_document_page_one();
                } else {
                    let _ = ui_contract::close_ui_patch_owner_one();
                }
                self.semantic_close_document_lane = !self.semantic_close_document_lane;
                return Ok(());
            };
            let Some(slot) = self.retained.slots[index].as_mut() else { return Err("retained surface slot is vacant".to_string()) };
            let generation = slot.generation;
            let surface = slot.surface.clone();
            let retained = &mut slot.owner;
            let active =
                retained.patch.is_some() || !retained.queued_patches.is_empty() || !retained.rejected_patches.is_empty() || retained.build.is_some() || retained.build_pending || retained.closing.is_some() || retained.exchange_closing.is_some();
            if !active {
                if let Some(rejection) = self.pending_rejections.get(instance, &surface).filter(|rejection| rejection.generation == generation) {
                    if retained.published.is_none() {
                        return Err(rejection.reason.to_string());
                    }
                }
            }
            if active {
                if retained.patch.is_some() || !retained.queued_patches.is_empty() {
                    if let Some(rejection) = retained.advance_patch_one() {
                        let reason = rejection.1.to_string();
                        let owner = PendingSurfaceRejection { generation, instance, surface: surface.clone(), revision: rejection.0, reason: rejection.1, patch: None };
                        let _ = self.pending_rejections.try_upsert(owner);
                        if requested.is_some() && retained.published.is_none() {
                            return Err(reason);
                        }
                    } else if retained.patch.is_none() {
                        let _ = self.pending_rejections.remove(instance, &surface);
                    }
                } else {
                    retained.advance_document_one();
                }
            }
            if retained.exchange_closing.is_none() {
                if let Some(document) = retained.try_alias() {
                    if let Err(rejected) = out.try_push(ExchangeSurfaceDocument { surface: surface.clone(), document }) {
                        retained.exchange_closing = Some(rejected.document);
                        if requested.is_some() {
                            return Err("retained surface exchange capacity exhausted".to_string());
                        }
                    }
                }
            }
            Ok(())
        }
    }

    const KERNEL_REQUEST_QUEUE_CAPACITY: usize = 64;

    struct KernelRequestQueue {
        state: Mutex<KernelRequestQueueState>,
    }

    struct KernelRequestQueueState {
        slots: [Option<(KernelRequest, Arc<ResponseSlot>)>; KERNEL_REQUEST_QUEUE_CAPACITY],
        read: usize,
        write: usize,
        len: usize,
        command_pages: usize,
        command_bytes: usize,
        closing: bool,
        consumer_waker: Option<Waker>,
        producer_waker: Option<Waker>,
    }

    impl Default for KernelRequestQueue {
        fn default() -> Self {
            Self { state: Mutex::new(KernelRequestQueueState { slots: std::array::from_fn(|_| None), read: 0, write: 0, len: 0, command_pages: 0, command_bytes: 0, closing: false, consumer_waker: None, producer_waker: None }) }
        }
    }

    impl KernelRequestQueue {
        fn try_push(&self, request: KernelRequest, slot: Arc<ResponseSlot>, producer: Option<&Waker>) -> Result<(), (KernelRequest, Arc<ResponseSlot>)> {
            let (pages, bytes) = request.command_credits();
            let Ok(mut state) = self.state.try_lock() else {
                return Err((request, slot));
            };
            let admitted_pages = state.command_pages.checked_add(pages).filter(|total| *total <= semio_framework::kernel::COMMAND_MAXIMUM_PAGES);
            let admitted_bytes = state.command_bytes.checked_add(bytes).filter(|total| *total <= semio_framework::kernel::COMMAND_MAXIMUM_BYTES);
            if state.closing || state.len == KERNEL_REQUEST_QUEUE_CAPACITY || admitted_pages.is_none() || admitted_bytes.is_none() {
                if let Some(producer) = producer {
                    state.producer_waker = Some(producer.clone());
                }
                return Err((request, slot));
            }
            let index = state.write;
            assert!(state.slots[index].is_none(), "fixed kernel request write slot is empty after admission");
            state.slots[index] = Some((request, slot));
            state.write = (index + 1) % KERNEL_REQUEST_QUEUE_CAPACITY;
            state.len += 1;
            state.command_pages = admitted_pages.expect("command page credit was admitted");
            state.command_bytes = admitted_bytes.expect("command byte credit was admitted");
            let consumer = state.consumer_waker.take();
            drop(state);
            if let Some(waker) = consumer {
                waker.wake();
            }
            Ok(())
        }

        fn poll(&self, cx: &mut Context<'_>) -> Poll<(KernelRequest, Arc<ResponseSlot>)> {
            let Ok(mut state) = self.state.try_lock() else {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            };
            if state.len == 0 {
                state.consumer_waker = Some(cx.waker().clone());
                return Poll::Pending;
            }
            let index = state.read;
            let request = state.slots[index].take().expect("fixed kernel request read slot is occupied");
            let (pages, bytes) = request.0.command_credits();
            state.read = (index + 1) % KERNEL_REQUEST_QUEUE_CAPACITY;
            state.len -= 1;
            state.command_pages -= pages;
            state.command_bytes -= bytes;
            let producer = state.producer_waker.take();
            drop(state);
            if let Some(waker) = producer {
                waker.wake();
            }
            Poll::Ready(request)
        }

        fn try_next(&self) -> Option<(KernelRequest, Arc<ResponseSlot>)> {
            let mut state = self.state.try_lock().ok()?;
            if state.len == 0 {
                return None;
            }
            let index = state.read;
            let request = state.slots[index].take().expect("fixed kernel request read slot is occupied");
            let (pages, bytes) = request.0.command_credits();
            state.read = (index + 1) % KERNEL_REQUEST_QUEUE_CAPACITY;
            state.len -= 1;
            state.command_pages -= pages;
            state.command_bytes -= bytes;
            let producer = state.producer_waker.take();
            drop(state);
            if let Some(waker) = producer {
                waker.wake();
            }
            Some(request)
        }

        async fn next(&self) -> (KernelRequest, Arc<ResponseSlot>) {
            std::future::poll_fn(|cx| self.poll(cx)).await
        }

        async fn enqueue_retained(&self, request: KernelRequest, slot: Arc<ResponseSlot>) {
            let mut owner = Some((request, slot));
            std::future::poll_fn(|cx| {
                let (request, slot) = owner.take().expect("retained queue admission owner is present");
                match self.try_push(request, slot, Some(cx.waker())) {
                    Ok(()) => Poll::Ready(()),
                    Err(rejected) => {
                        owner = Some(rejected);
                        Poll::Pending
                    }
                }
            })
            .await;
        }

        fn begin_shutdown(&self) -> bool {
            let Ok(mut state) = self.state.try_lock() else {
                return false;
            };
            state.closing = true;
            true
        }

        fn shutdown_step(&self, maximum_bytes: usize) -> (bool, usize, usize) {
            let Ok(mut state) = self.state.try_lock() else {
                return (false, 0, 0);
            };
            if !state.closing || state.len == 0 {
                return (state.closing && state.len == 0, 0, 0);
            }
            let index = state.read;
            let (terminal, processed, released, page_released) = match &mut state.slots[index].as_mut().expect("fixed shutdown request slot is occupied").0 {
                KernelRequest::ExchangeCommands { driver, .. } => {
                    let before = driver.remaining_pages();
                    let (terminal, released) = driver.close_step(maximum_bytes);
                    let page_released = usize::from(driver.remaining_pages() < before);
                    (terminal, usize::from(terminal || page_released != 0), released, page_released)
                }
                KernelRequest::CloseRejectedCommandBuild { owner, .. } => {
                    let before = owner.remaining_pages();
                    let (terminal, released) = owner.close_step(maximum_bytes);
                    let page_released = usize::from(owner.remaining_pages() < before);
                    (terminal, usize::from(terminal || page_released != 0), released, page_released)
                }
                KernelRequest::CreateApp { owner } => {
                    let (terminal, processed, released) = owner.close_step(maximum_bytes);
                    (terminal, processed, released, 0)
                }
                KernelRequest::DestroyApp { owner } => {
                    owner.finish(KernelCloseStatus::Fault);
                    (true, 1, 0, 0)
                }
                KernelRequest::CloseRealm { owner } => {
                    owner.finish(KernelCloseStatus::Fault);
                    (true, 1, 0, 0)
                }
                KernelRequest::Exchange { event, .. } => {
                    let (terminal, processed, released) = event.close_step(maximum_bytes);
                    (terminal, processed, released, 0)
                }
                KernelRequest::AdvanceRetained { .. } | KernelRequest::MountProductReplay { .. } | KernelRequest::RetireProductReplay { .. } | KernelRequest::RetireProductReplayRefusal { .. } | KernelRequest::AdvanceProductReplay { .. } => {
                    (true, 1, 0, 0)
                }
                KernelRequest::CloseRejectedEvents { owner } => {
                    let (terminal, processed) = owner.close_step();
                    (terminal, processed, 0, 0)
                }
                KernelRequest::AcknowledgeJobProgress { token } => {
                    let returned = job_progress_presentation_bridge().lock().expect("job progress presentation bridge lock").return_lease(*token);
                    (returned, usize::from(returned), 0, 0)
                }
            };
            state.command_pages -= page_released;
            state.command_bytes -= released;
            if terminal {
                let terminal = state.slots[index].take().expect("terminal shutdown request was present");
                state.read = (index + 1) % KERNEL_REQUEST_QUEUE_CAPACITY;
                state.len -= 1;
                drop(state);
                drop(terminal);
                return (self.state.try_lock().is_ok_and(|state| state.len == 0), processed, released);
            }
            (false, processed, released)
        }
    }

    pub(crate) struct KernelPoolFuture {
        pool: semio_framework_async::WorkerPool,
        lane: semio_framework_async::Lane,
        future: Mutex<Option<Pin<Box<dyn Future<Output = ()> + Send + 'static>>>>,
        scheduled: std::sync::atomic::AtomicBool,
        notified: std::sync::atomic::AtomicBool,
    }

    impl KernelPoolFuture {
        pub(crate) fn spawn(pool: semio_framework_async::WorkerPool, lane: semio_framework_async::Lane, future: impl Future<Output = ()> + Send + 'static) -> Arc<Self> {
            let task = Arc::new(Self { pool, lane, future: Mutex::new(Some(Box::pin(future))), scheduled: std::sync::atomic::AtomicBool::new(false), notified: std::sync::atomic::AtomicBool::new(true) });
            task.schedule();
            task
        }

        fn schedule(self: &Arc<Self>) {
            self.notified.store(true, std::sync::atomic::Ordering::Release);
            if self.scheduled.swap(true, std::sync::atomic::Ordering::AcqRel) {
                return;
            }
            let task = self.clone();
            self.pool.submit(self.lane, Box::new(move || task.run_turn()));
        }

        fn run_turn(self: Arc<Self>) {
            self.notified.store(false, std::sync::atomic::Ordering::Release);
            if let Some(mut future) = self.future.lock().expect("kernel pool future lock").take() {
                let waker = Waker::from(self.clone());
                let mut context = Context::from_waker(&waker);
                if future.as_mut().poll(&mut context).is_pending() {
                    *self.future.lock().expect("kernel pool future lock") = Some(future);
                }
            }
            self.scheduled.store(false, std::sync::atomic::Ordering::Release);
            if self.notified.load(std::sync::atomic::Ordering::Acquire) && self.future.lock().expect("kernel pool future lock").is_some() {
                self.schedule();
            }
        }
    }

    impl std::task::Wake for KernelPoolFuture {
        fn wake(self: Arc<Self>) {
            self.schedule();
        }

        fn wake_by_ref(self: &Arc<Self>) {
            self.schedule();
        }
    }

    async fn yield_kernel_maintenance_turn() {
        let mut yielded = false;
        std::future::poll_fn(|cx| {
            if yielded {
                Poll::Ready(())
            } else {
                yielded = true;
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        })
        .await
    }

    async fn run_kernel_pool(queue: Arc<KernelRequestQueue>) {
        let mut state = KernelPoolState::new(queue.clone()).await;
        loop {
            if state.command_maintenance_pending() {
                let _ = state.command_maintenance_step();
                yield_kernel_maintenance_turn().await;
                continue;
            }
            let ready = queue.try_next();
            if ready.is_none() {
                if let Some(instance) = state.product_replay_pending_instance() {
                    if let Err(error) = state.advance_product_replay(instance).await {
                        crate::log_debug(&format!("kernel: mounted product replay fault: {error}"));
                        state.abort_product_replay_one(instance);
                    }
                    yield_kernel_maintenance_turn().await;
                    continue;
                }
            }
            let (request, slot) = match ready {
                Some(ready) => ready,
                None => queue.next().await,
            };
            let outcome = match request {
                KernelRequest::CreateApp { owner } => {
                    let (wasm_path, plugin_id, app_id) = owner.into_parts();
                    KernelOutcome::Created(state.create_app(wasm_path, plugin_id, app_id).await)
                }
                KernelRequest::DestroyApp { owner } => {
                    if state.destroy_app_step(owner.instance).await {
                        owner.finish(KernelCloseStatus::Complete);
                    } else {
                        owner.phase.store(KERNEL_CLOSE_READY, std::sync::atomic::Ordering::Release);
                        let _ = owner.try_schedule();
                    }
                    continue;
                }
                KernelRequest::CloseRealm { owner } => {
                    if state.close_realm_progress_step() {
                        owner.finish(KernelCloseStatus::Complete);
                    } else {
                        owner.phase.store(KERNEL_CLOSE_READY, std::sync::atomic::Ordering::Release);
                        let _ = owner.try_schedule();
                    }
                    continue;
                }
                KernelRequest::AcknowledgeJobProgress { token } => {
                    state.acknowledge_job_progress(token);
                    continue;
                }
                KernelRequest::AcknowledgeTypedOperationResult { token } => {
                    let _ = state.deliver_typed_operation_result_ack(token).await;
                    continue;
                }
                KernelRequest::Exchange { instance, event } => KernelOutcome::Exchanged(state.exchange(instance, vec![event.into_event()]).await),
                KernelRequest::AdvanceRetained { instance, surface } => KernelOutcome::Exchanged(state.advance_retained_surface_one(instance, surface)),
                KernelRequest::MountProductReplay { owner } => KernelOutcome::ProductReplayMounted(state.mount_product_replay(owner)),
                KernelRequest::RetireProductReplay { owner } => {
                    owner.retire();
                    KernelOutcome::ProductReplayMounted(Ok(()))
                }
                KernelRequest::RetireProductReplayRefusal { owner } => {
                    owner.retire();
                    KernelOutcome::ProductReplayMounted(Ok(()))
                }
                KernelRequest::AdvanceProductReplay { instance } => KernelOutcome::Exchanged(state.advance_product_replay(instance).await),
                KernelRequest::ExchangeCommands { instance, driver } => KernelOutcome::Exchanged(state.exchange_commands(instance, driver).await),
                KernelRequest::CloseRejectedCommandBuild { key, owner } => {
                    assert!(state.rejected_command_builds.can_insert(key), "worker drains the prior rejected command build before dequeuing another request");
                    state.rejected_command_builds.insert_admitted(key, owner);
                    continue;
                }
                KernelRequest::CloseRejectedEvents { owner } => {
                    assert!(state.rejected_events.is_none(), "worker drains the prior rejected event owner before dequeuing another request");
                    state.rejected_events = Some(owner);
                    continue;
                }
            };
            slot.deliver(outcome);
        }
    }

    #[cfg(test)]
    mod semantic_document_tests {
        use super::*;

        fn document(generation: u64, surface: &str) -> UiDocumentLease {
            let id = ui_contract::UiNodeId(1);
            let surface = SurfaceId::try_from(surface).expect("bounded command retirement surface");
            let mut builder = UiDocumentBuilder::try_new(generation, surface, UiRevision(generation), Some(id), generation).expect("command retirement document slot");
            builder
                .try_push(ui_contract::UiNodeRecord {
                    id,
                    key: ui_contract::UiText::try_from_str("root").expect("bounded command retirement key"),
                    component: ui_contract::Component::Separator(ui_contract::SeparatorProps {}),
                    layout: Default::default(),
                    style: Default::default(),
                    activity: Default::default(),
                    disabled: false,
                    transition: None,
                    accessibility: Default::default(),
                    bindings: Default::default(),
                    menu: None,
                    children: Default::default(),
                })
                .expect("command retirement root");
            builder.finish().expect("command retirement document")
        }

        fn retained_patch(surface: SurfaceId, base: u64, revision: u64) -> KernelUiPatch {
            let id = ui_contract::UiNodeId(1);
            let record = ui_contract::UiNodeRecord {
                id,
                key: ui_contract::UiText::try_from("root").expect("bounded hostile key"),
                component: ui_contract::Component::Separator(ui_contract::SeparatorProps {}),
                layout: Default::default(),
                style: Default::default(),
                activity: Default::default(),
                disabled: false,
                transition: None,
                accessibility: Default::default(),
                bindings: Default::default(),
                menu: None,
                children: Default::default(),
            };
            let mut ops = ui_contract::UiPatchOps::default();
            ops.try_push(ui_contract::UiPatchOp::Upsert(record)).expect("hostile upsert owner");
            ops.try_push(ui_contract::UiPatchOp::SetRoot { id }).expect("hostile root owner");
            KernelUiPatch { surface, base_revision: UiRevision(base), revision: UiRevision(revision), ops }
        }

        #[test]
        fn renderer_sequence_maximum_is_once_then_permanently_exhausted_and_rollback_is_transactional() {
            let mut authority = RendererSequenceAuthority { committed: u64::MAX - 1, reserved: None, exhausted: false };
            let maximum = authority.try_reserve().expect("maximum is admitted once");
            assert_eq!(maximum, u64::MAX);
            authority.commit(maximum).expect("maximum commits once");
            let witness = (authority.committed, authority.reserved, authority.exhausted);
            assert!(authority.try_reserve().is_err());
            assert!(authority.try_reserve().is_err());
            assert_eq!((authority.committed, authority.reserved, authority.exhausted), witness);

            let mut rollback = RendererSequenceAuthority::default();
            let first = rollback.try_reserve().expect("first reservation");
            rollback.rollback(first);
            assert_eq!(rollback.try_reserve().expect("rolled back identity is reusable before commit"), first);
        }

        #[test]
        fn renderer_fixed_registries_return_exact_max_plus_one_surface_and_patch_owners() {
            let mut retained = RetainedSurfaceRegistry::new();
            for index in 0..RETAINED_SURFACE_CAPACITY {
                retained.try_admit(1, SurfaceId::try_from(format!("surface-{index}")).expect("bounded surface")).expect("maximum retained surface");
            }
            let overflow = SurfaceId::try_from("surface-overflow").expect("bounded surface");
            assert_eq!(retained.try_admit(1, overflow.clone()), Err(overflow));

            let mut rejections = PendingSurfaceRejectionRegistry::new();
            for index in 0..RETAINED_SURFACE_CAPACITY {
                let surface = SurfaceId::try_from(format!("rejection-{index}")).expect("bounded surface");
                rejections
                    .try_admit(PendingSurfaceRejection {
                        generation: index as u64 + 1,
                        instance: 1,
                        surface: surface.clone(),
                        revision: UiRevision(index as u64),
                        reason: ui_contract::UiText::try_from_str("capacity").expect("bounded reason"),
                        patch: Some(retained_patch(surface, 0, index as u64 + 1)),
                    })
                    .expect("maximum pending rejection");
            }
            let surface = SurfaceId::try_from("rejection-overflow").expect("bounded surface");
            let rejected = rejections
                .try_admit(PendingSurfaceRejection {
                    generation: 999,
                    instance: 1,
                    surface: surface.clone(),
                    revision: UiRevision(999),
                    reason: ui_contract::UiText::try_from_str("capacity").expect("bounded reason"),
                    patch: Some(retained_patch(surface, 0, 999)),
                })
                .expect_err("maximum plus one rejection");
            assert_eq!(rejected.patch.as_ref().map(|patch| patch.revision), Some(UiRevision(999)));
            while !rejections.terminal_is_empty() {
                let _ = rejections.take_any_one();
            }
            while !retained.advance_realm_close_one() {}
        }

        #[test]
        fn command_batch_ninth_document_is_retained_after_nonterminal_close_and_exactly_returned_or_retired() {
            let first = document(91_001, "command.retirement.first");
            let mut owners = UiFixedList::<ExchangeSurfaceDocument, 9>::default();
            for ordinal in 0..7 {
                owners
                    .try_push(ExchangeSurfaceDocument { surface: SurfaceId::try_from(format!("command.retirement.alias-{ordinal}")).expect("bounded alias surface"), document: first.try_alias().expect("fixed command document alias") })
                    .unwrap_or_else(|_| panic!("command alias owner"));
            }
            owners.try_push(ExchangeSurfaceDocument { surface: SurfaceId::try_from("command.retirement.primary").expect("bounded primary surface"), document: first }).unwrap_or_else(|_| panic!("command primary owner"));
            owners
                .try_push(ExchangeSurfaceDocument { surface: SurfaceId::try_from("command.retirement.ninth").expect("bounded ninth surface"), document: document(91_002, "command.retirement.ninth-document") })
                .unwrap_or_else(|_| panic!("command ninth owner"));
            let mut registry = CommandDocumentRetirementRegistry::new();
            let mut first_page = registry.try_reserve_page(7, 91_101).expect("first fixed page destinations");
            for _ in 0..UI_DOCUMENT_LEASE_SLOTS {
                registry.admit(&mut first_page, owners.swap_remove(0).expect("first page owner"));
            }
            registry.release(&mut first_page);
            let mut second_page = registry.try_reserve_page(7, 91_101).expect("second fixed page destinations");
            registry.admit(&mut second_page, owners.swap_remove(0).expect("ninth page owner"));
            registry.release(&mut second_page);
            registry.begin_close_batch(7, 91_100);
            assert!(!registry.has_close_work(), "stale batch generation cannot mutate pending document owners");
            let mut output = UiFixedList::default();
            registry.publish_batch(7, 91_101, &mut output);
            assert_eq!(output.len(), UI_DOCUMENT_LEASE_SLOTS);
            assert!(!registry.batch_is_empty(7, 91_101), "the ninth owner remains qualified in the close lane");
            while !registry.close_one() {}
            assert!(!registry.batch_is_empty(7, 91_101), "one nonterminal close opportunity cannot discard the ninth owner");
            let mut cleanup = registry.try_reserve_page(8, 91_102).expect("output cleanup destinations");
            for owner in output {
                registry.admit(&mut cleanup, owner);
            }
            registry.release(&mut cleanup);
            registry.begin_close_batch(8, 91_102);
            let mut opportunities = 1;
            while !registry.batch_is_empty(7, 91_101) || !registry.batch_is_empty(8, 91_102) {
                let _ = registry.close_one();
                opportunities += 1;
                assert!(opportunities < 16_384, "command document retirement must converge");
            }
        }

        #[test]
        fn command_document_page_saturation_refuses_before_turn_owner_production() {
            let mut registry = CommandDocumentRetirementRegistry::new();
            let mut reservations = UiFixedList::<CommandDocumentPageReservation, { COMMAND_DOCUMENT_RETIREMENT_CAPACITY / UI_DOCUMENT_LEASE_SLOTS }>::default();
            for generation in 1..=COMMAND_DOCUMENT_RETIREMENT_CAPACITY / UI_DOCUMENT_LEASE_SLOTS {
                reservations.try_push(registry.try_reserve_page(17, generation as u64).expect("absolute command document page credit")).unwrap_or_else(|_| panic!("fixed reservation owner"));
            }
            assert!(registry.try_reserve_page(17, 999).is_err(), "max plus one refuses before a guest turn can produce another document owner");
            for mut reservation in reservations {
                registry.release(&mut reservation);
            }
            assert!(registry.instance_is_empty(17));
        }

        #[test]
        fn stale_cancel_and_fault_document_builds_retain_closer_until_terminal() {
            let surface = SurfaceId::try_from("hostile.document-close").expect("bounded hostile surface");
            let mut retained = RetainedSurface::new(surface.clone());
            retained.admit_patch(retained_patch(surface, 0, 1)).expect("retained patch");
            while retained.patch.is_some() {
                let _ = retained.advance_patch_one();
            }
            retained.begin_document_build();
            retained.advance_document_one();
            let build = retained.build.as_mut().expect("builder retained");
            build.closing = true;
            retained.advance_document_one();
            assert!(retained.build.is_some(), "cancel/stale/fault first close opportunity retains a nonterminal closer");
            while retained.build.is_some() {
                retained.advance_document_one();
            }
            while !retained.close_step() {}
        }

        #[test]
        fn first_render_requires_multiple_fixed_opportunities_stale_patch_keeps_last_valid_and_close_reaches_terminal() {
            let surface_id = SurfaceId::try_from("hostile.first-render").expect("bounded hostile surface");
            let mut retained = RetainedSurface::new(surface_id.clone());
            retained.admit_patch(retained_patch(surface_id.clone(), 0, 1)).expect("first retained patch");
            assert!(retained.try_alias().is_none());
            let mut opportunities = 0;
            while retained.try_alias().is_none() {
                assert!(retained.advance_patch_one().is_none());
                if retained.patch.is_none() {
                    retained.advance_document_one();
                }
                opportunities += 1;
                assert!(opportunities <= ui_contract::UI_DOCUMENT_PATCH_OPS + ui_contract::UI_DOCUMENT_NODES * ui_contract::UI_DOCUMENT_NODES);
            }
            assert!(opportunities > 1);
            let mut last_valid = retained.try_alias().expect("published first document");
            let generation = last_valid.generation();
            retained.admit_patch(retained_patch(surface_id, 0, 2)).expect("stale patch owner retained");
            while retained.advance_patch_one().is_none() {}
            assert_eq!(retained.try_alias().expect("last valid survives stale patch").generation(), generation);
            let _ = last_valid.close_step();
            drop(last_valid);
            while !retained.close_step() {}
        }

        #[test]
        fn mounted_job_progress_presentation_bridge_is_fixed_fifo_and_generation_checked() {
            let mut bridge = JobProgressPresentationBridge::new();
            let identity = |ordinal: usize| JobProgressIdentity { actor: ActorId(ordinal as u64 + 1), job: 7, operation: ordinal as u64 + 11, base_revision: 13, generation: 17, step_sequence: 0, preview_sequence: 1 };
            let mut tokens = Vec::new();
            for ordinal in 0..JOB_PROGRESS_PRESENTATION_CAPACITY {
                let token = bridge.reserve(identity(ordinal), JobProgressKind::Preview, ordinal as u64).expect("fixed presentation slot");
                assert!(bridge.publish(token));
                tokens.push(token);
            }
            assert!(bridge.reserve(identity(999), JobProgressKind::Preview, 0).is_none(), "capacity +1 must reject before publication ownership moves");
            for (ordinal, token) in tokens.into_iter().enumerate() {
                let mut lease = bridge.take().expect("FIFO presentation lease");
                assert_eq!(lease.identity, identity(ordinal));
                assert_eq!(lease.token, token);
                assert!(bridge.presented(token));
                lease.terminal = true;
                assert!(bridge.release_presented(token));
                assert!(!bridge.release_presented(token), "duplicate generation release is rejected");
            }
            assert!(bridge.take().is_none());
            assert!(bridge.terminal_is_empty(), "realm witness requires every fixed presentation slot to be vacant");
        }

        #[test]
        fn cancelled_head_does_not_strand_later_ready_presentations() {
            let mut bridge = JobProgressPresentationBridge::new();
            let identity = |actor: u64| JobProgressIdentity { actor: ActorId(actor), job: 7, operation: actor + 11, base_revision: 13, generation: 17, step_sequence: 0, preview_sequence: 1 };
            let head = bridge.reserve(identity(1), JobProgressKind::Preview, 1).expect("head");
            let middle = bridge.reserve(identity(2), JobProgressKind::Preview, 2).expect("middle");
            let tail = bridge.reserve(identity(3), JobProgressKind::Preview, 3).expect("tail");
            assert!(bridge.publish(head));
            assert!(bridge.publish(middle));
            assert!(bridge.publish(tail));
            assert!(bridge.cancel(head));

            let mut returned = bridge.take().expect("cancelled head is skipped");
            assert_eq!(returned.token, middle);
            assert!(bridge.return_lease(middle));
            returned.terminal = true;
            let mut middle_lease = bridge.take().expect("returned oldest lease retains admitted order");
            assert_eq!(middle_lease.token, middle);
            assert!(bridge.presented(middle));
            middle_lease.terminal = true;
            assert!(bridge.release_presented(middle));

            let mut tail_lease = bridge.take().expect("tail remains reachable after cancelled hole");
            assert_eq!(tail_lease.token, tail);
            assert!(bridge.presented(tail));
            tail_lease.terminal = true;
            assert!(bridge.release_presented(tail));
            assert!(bridge.terminal_is_empty());
        }

        #[test]
        fn mounted_replay_recovery_capacity_plus_one_refuses_without_touching_reserved_identity() {
            let mut registry = MountedReplayRecoveryRegistry::new();
            let tokens: [_; JOB_PROGRESS_ACTIVE_CAPACITY] = std::array::from_fn(|_| registry.reserve().expect("fixed recovery reservation"));
            assert!(registry.reserve().is_none());
            for token in tokens {
                assert!(registry.release(token));
                assert!(!registry.release(token));
            }
            assert!(!registry.has_close_work());
        }

        #[test]
        fn rejected_production_replay_submit_retries_exact_restore_start_identity_before_job_step() {
            let recovery = mounted_replay_recovery_registry().lock().expect("replay recovery lock").reserve().expect("pre-reserved submit recovery");
            let route = JobReplayRoute { plugin: [1; 32], package: [2; 32], controller: [3; 32], tool: [4; 32], window: 5, document: [6; 32], request_schema: [7; 32], request_version: 1, request_digest: [8; 32] };
            let authority = JobTurn { job: 19, operation: semio_framework_actor::JobOperation { operation: 23, base_revision: 29, generation: 31, preview_sequence: 0, seed: 37 }, step_sequence: 0 };
            let request = JobReplayRequest::from_spawn("fixture.production-retry", b"retained-input");
            let mut mounted = MountedJobReplay {
                actor: ActorId(41),
                authority,
                request,
                placement: JobPlacement::Isolated,
                log: ManuallyDrop::new(JobReplayLog::new(route, 31).expect("generation-qualified replay")),
                captured: None,
                policy: None,
                terminal_seen: true,
                replay_requested: true,
                replay_started: false,
                replay_submit_sequence: None,
                accepted_replay_sequence: None,
                replay_worker_count: 4,
                replay_worker_slot: 3,
                recovery: Some(recovery),
            };
            let first = mounted.replay_submission_sequence().expect("first retained restore/start packet");
            let rejected_identity = (mounted.actor, mounted.authority, mounted.request, mounted.replay_worker_count, mounted.replay_worker_slot, first);
            assert!(!mounted.replay_started);
            assert!(mounted.pending_replay_start_is_exact(4, 3));
            assert!(!mounted.pending_replay_start_is_exact(2, 3));
            assert!(!mounted.pending_replay_start_is_exact(4, 2));
            let retry = mounted.replay_submission_sequence().expect("backpressure retries retained restore/start packet");
            assert_eq!((mounted.actor, mounted.authority, mounted.request, mounted.replay_worker_count, mounted.replay_worker_slot, retry), rejected_identity);
            assert!(!mounted.replay_started, "a rejected runtime submit cannot make the next opportunity a JobStep");
            mounted.accept_replay_submission(retry).expect("accepted exact restore/start packet");
            assert!(mounted.replay_started);
            assert_eq!(mounted.replay_submit_sequence, None);
            drop(mounted);
            while mounted_replay_recovery_registry().lock().expect("replay recovery lock").has_close_work() {
                assert!(mounted_replay_recovery_registry().lock().expect("replay recovery lock").close_one());
            }
        }

        fn admitted_product_request(instance: u32, job: u64, kind: &str, input: &[u8], placement: JobPlacement) -> MountedProductReplayRequest {
            let recovery = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").reserve().expect("fixed product replay request recovery");
            MountedProductReplayRequest::from_admitted_effect(instance, 0, Effect::SpawnJob { job, kind: kind.to_string(), input: input.to_vec(), placement }, recovery)
        }

        fn product_admission_permit(instance: u32) -> MountedProductReplayAdmissionPermit {
            let token = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").reserve(instance).expect("fixed product refusal permit");
            MountedProductReplayAdmissionPermit { token: Some(token) }
        }

        fn drain_product_recovery() -> usize {
            let mut opportunities = 0;
            while mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").has_close_work() {
                assert!(mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").close_one());
                opportunities += 1;
            }
            opportunities
        }

        fn product_authority_fixture() -> (MountedProductReplayAuthority, MountedProductReplayExpected) {
            let kind = "action-bus.production";
            let mut request_owner = admitted_product_request(59, 7, kind, b"fixed-input", JobPlacement::Isolated);
            let request = request_owner.request;
            let route =
                JobReplayRoute { plugin: [1; 32], package: [2; 32], controller: request.controller, tool: request.tool, window: 5, document: [6; 32], request_schema: request.schema, request_version: request.version, request_digest: request.digest };
            let operation = JobOperation { operation: 11, base_revision: 13, generation: 17, preview_sequence: 0, seed: 19 };
            let turn = JobTurn { job: 7, operation, step_sequence: 23 };
            let terminal = JobReplayRecordHeader {
                actor: ActorId(29),
                turn,
                ordinal: 31,
                granted_fuel: 37,
                deadline_class_ms: 4,
                worker_count: 7,
                worker_slot: 3,
                cancellation_observed: false,
                kind: JobReplayPublicationKind::Commit,
                policy: JobReplayPublicationPolicy::Accepted,
                applied_progress: 41,
                payload_items: 2,
                payload_bytes: 43,
                payload_digest: 47,
                prefix_digest: 53,
            };
            let checkpoint = Some(MountedProductReplayCheckpoint { ordinal: 61, digest: 67, pages: 2, applied_progress: 69 });
            request_owner.raw_mut().acknowledge_mount();
            request_owner.raw_mut().acknowledge_qualification(checkpoint);
            let expected = MountedProductReplayExpected {
                instance: 59,
                actor: terminal.actor,
                turn,
                request,
                placement: JobPlacement::Isolated,
                route,
                checkpoint,
                terminal,
                process_worker_count: 7,
                process_worker_slot: 3,
                logical_worker_count: 7,
                logical_worker_slot: 3,
                begin: true,
                restore_start_ordinal: Some(71),
            };
            (
                MountedProductReplayAuthority {
                    request_owner: Some(request_owner),
                    actor: terminal.actor,
                    route,
                    operation,
                    generation: operation.generation,
                    seed: operation.seed,
                    checkpoint,
                    terminal,
                    worker_count: 7,
                    worker_slot: 3,
                    begin: true,
                    restore_start_ordinal: Some(71),
                    profile_cursor: 0,
                    profile_started: false,
                },
                expected,
            )
        }

        #[test]
        fn production_action_bus_product_authority_carries_complete_identity_and_returns_exact_owner_on_wrong_field() {
            let (authority, expected) = product_authority_fixture();
            let authority = authority.validate(expected).expect("complete typed authority");
            assert_eq!(authority.request_owner().job_kind(), b"action-bus.production");
            assert_eq!((authority.instance(), authority.job(), authority.operation, authority.generation, authority.seed), (59, 7, expected.turn.operation, 17, 19));
            assert_eq!(authority.request_owner().placement, JobPlacement::Isolated);
            assert_eq!(authority.checkpoint, Some(MountedProductReplayCheckpoint { ordinal: 61, digest: 67, pages: 2, applied_progress: 69 }));
            assert_eq!((authority.worker_count, authority.worker_slot, authority.begin, authority.restore_start_ordinal), (7, 3, true, Some(71)));
            authority.retire();

            let (authority, mut wrong) = product_authority_fixture();
            wrong.checkpoint.as_mut().expect("checkpoint witness").digest ^= 1;
            let token = authority.request_owner().recovery.expect("pre-reserved exact handback");
            let rejected = authority.validate(wrong).expect_err("wrong checkpoint returns exact authority");
            assert_eq!((rejected.instance(), rejected.job(), rejected.generation, rejected.seed, rejected.terminal.prefix_digest), (59, 7, 17, 19, 53));
            rejected.retire();
            {
                let registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
                assert_eq!(registry.slots[token.index].generation, token.generation);
                assert_eq!(registry.slots[token.index].owner.as_ref().map(MountedProductReplayRecoveryOwner::job), Some(7));
            }
            while mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").has_close_work() {
                assert!(mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").close_one());
            }
        }

        fn assert_product_authority_field_rejected(mutate: fn(&mut MountedProductReplayAuthority, &mut MountedProductReplayExpected)) {
            let (mut authority, mut expected) = product_authority_fixture();
            let token = authority.request_owner().recovery.expect("pre-reserved field rejection");
            mutate(&mut authority, &mut expected);
            let rejected = authority.validate(expected).expect_err("wrong product authority field must return the exact shell");
            assert_eq!((rejected.instance(), rejected.job()), (59, 7));
            rejected.retire();
            let mut registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
            assert_eq!((registry.slots[token.index].generation, registry.slots[token.index].owner.as_ref().map(MountedProductReplayRecoveryOwner::job)), (token.generation, Some(7)));
            while registry.has_close_work() {
                assert!(registry.close_one());
            }
        }

        #[test]
        fn production_product_authority_qualifies_every_minted_field_before_replay_work() {
            assert_product_authority_field_rejected(|authority, _| authority.request_owner.as_mut().expect("request").job_kind[0] ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.request.digest[0] ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.placement = JobPlacement::Exclusive);
            assert_product_authority_field_rejected(|_, expected| expected.route.request_digest[0] ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.turn.operation.operation ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.turn.operation.generation ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.turn.operation.seed ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.checkpoint.as_mut().expect("checkpoint").ordinal ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.checkpoint.as_mut().expect("checkpoint").digest ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.checkpoint.as_mut().expect("checkpoint").pages ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.checkpoint.as_mut().expect("checkpoint").applied_progress ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.terminal.ordinal ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.terminal.payload_digest ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.terminal.prefix_digest ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.process_worker_count ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.process_worker_slot ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.logical_worker_count ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.logical_worker_slot ^= 1);
            assert_product_authority_field_rejected(|_, expected| expected.begin = false);
            assert_product_authority_field_rejected(|_, expected| expected.restore_start_ordinal = Some(72));
        }

        #[test]
        fn product_authority_fault_abort_and_close_rediscover_exact_generation_once() {
            let (authority, _) = product_authority_fixture();
            let token = authority.request_owner().recovery.expect("pre-reserved abort handback");
            let mut authorities: [Option<MountedProductReplayAuthority>; JOB_PROGRESS_ACTIVE_CAPACITY] = std::array::from_fn(|_| None);
            authorities[0] = Some(authority);
            authorities[0].take().expect("fault retains exact authority").retire();
            let mut registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
            assert_eq!((registry.slots[token.index].generation, registry.slots[token.index].owner.as_ref().map(MountedProductReplayRecoveryOwner::job)), (token.generation, Some(7)));
            while registry.has_close_work() {
                assert!(registry.close_one());
            }
            assert!(!registry.has_close_work());
        }

        #[test]
        fn product_claim_fault_retains_exact_request_cursor_checkpoint_and_generation() {
            let request = admitted_product_request(83, 89, "action-bus.claim", b"claim-input", JobPlacement::Exclusive);
            let token = request.recovery.expect("pre-reserved claim handback");
            MountedProductReplayClaim { request: Some(request), actor: ActorId(97), record_cursor: 101, checkpoint: Some(MountedProductReplayCheckpoint { ordinal: 103, digest: 107, pages: 2, applied_progress: 109 }) }.retire();
            let mut registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
            assert_eq!(registry.slots[token.index].generation, token.generation);
            assert!(matches!(
                registry.slots[token.index].owner.as_ref(),
                Some(MountedProductReplayRecoveryOwner::Claim {
                    request,
                    actor: ActorId(97),
                    record_cursor: 101,
                    checkpoint: Some(MountedProductReplayCheckpoint { ordinal: 103, digest: 107, pages: 2, applied_progress: 109 }),
                }) if request.instance == 83 && request.job == 89 && request.placement == JobPlacement::Exclusive
            ));
            while registry.has_close_work() {
                assert!(registry.close_one());
            }
            assert!(!registry.has_close_work());
        }

        #[test]
        fn product_effect_after_normal_exchange_is_moved_into_fixed_authority_without_json_escape_hatch() {
            let mut outcome = ExchangeOutcome {
                frames: Vec::new(),
                surfaces: UiFixedList::default(),
                effects: vec![
                    Effect::Notify { message: "before".to_string() },
                    Effect::SpawnJob { job: 73, kind: "action-bus.production".to_string(), input: vec![79, 83], placement: JobPlacement::Inline },
                    Effect::Notify { message: "after".to_string() },
                ],
                command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
            };
            let (kind_pointer, input_pointer) = match &outcome.effects[1] {
                Effect::SpawnJob { kind, input, .. } => (kind.as_ptr(), input.as_ptr()),
                _ => panic!("selected product source remains SpawnJob"),
            };
            let authority = match outcome.take_product_replay_authority(89, product_admission_permit(89)) {
                MountedProductReplayAdmission::Admitted(authority) => authority,
                MountedProductReplayAdmission::None | MountedProductReplayAdmission::Refused(_) => panic!("fixed product exchange must admit"),
            };
            assert_eq!((authority.instance, authority.job, authority.job_kind(), authority.placement), (89, 73, b"action-bus.production".as_slice(), JobPlacement::Inline));
            assert_eq!(authority.request, JobReplayRequest::from_spawn("action-bus.production", &[79, 83]));
            assert_eq!((authority.raw().kind().as_ptr(), authority.raw().input().as_ptr(), authority.raw().selected_index), (kind_pointer, input_pointer, 1));
            assert!(matches!(&outcome.effects[..], [Effect::Notify { message: before }, Effect::Notify { message: after }] if before == "before" && after == "after"));
            authority.retire();
            assert_eq!(drain_product_recovery(), 2);
        }

        #[test]
        fn admitted_product_mount_refusal_returns_exact_raw_allocation_then_closes_one_backing_per_grant() {
            let mut outcome = ExchangeOutcome {
                frames: Vec::new(),
                surfaces: UiFixedList::default(),
                effects: vec![Effect::SpawnJob { job: 91, kind: "raw.mount-refusal".to_string(), input: vec![2, 3, 5, 7], placement: JobPlacement::Exclusive }],
                command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
            };
            let (kind_pointer, input_pointer) = match &outcome.effects[0] {
                Effect::SpawnJob { kind, input, .. } => (kind.as_ptr(), input.as_ptr()),
                _ => panic!("mount-refusal source remains SpawnJob"),
            };
            let request = match outcome.take_product_replay_authority(93, product_admission_permit(93)) {
                MountedProductReplayAdmission::Admitted(owner) => owner,
                MountedProductReplayAdmission::None | MountedProductReplayAdmission::Refused(_) => panic!("bounded raw request must admit"),
            };
            let token = request.recovery.expect("admitted request keeps exact recovery");
            let rejected: Result<(), MountedProductReplayRequest> = Err(request);
            let request = rejected.expect_err("unregistered mount returns the same request owner");
            assert_eq!((request.raw().kind().as_ptr(), request.raw().input().as_ptr(), request.raw().selected_index), (kind_pointer, input_pointer, 0));
            request.retire();
            {
                let registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
                assert!(
                    matches!(registry.slots[token.index].owner.as_ref(), Some(MountedProductReplayRecoveryOwner::Request(request)) if request.raw.kind().as_ptr() == kind_pointer && request.raw.input().as_ptr() == input_pointer && request.raw.disposition == RawSpawnJobDisposition::Rejected)
                );
            }
            assert_eq!(drain_product_recovery(), 2, "raw input and kind backing close on separate grants");
        }

        #[test]
        fn admitted_product_request_claim_and_authority_drop_preserve_raw_allocation_identity() {
            let request = admitted_product_request(95, 97, "raw.request-drop", b"request", JobPlacement::Inline);
            let request_token = request.recovery.expect("request drop recovery");
            let request_identity = (request.raw().kind().as_ptr(), request.raw().input().as_ptr());
            drop(request);
            {
                let registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
                assert!(matches!(registry.slots[request_token.index].owner.as_ref(), Some(MountedProductReplayRecoveryOwner::Request(request)) if (request.raw.kind().as_ptr(), request.raw.input().as_ptr()) == request_identity));
            }
            assert_eq!(drain_product_recovery(), 2);

            let mut request = admitted_product_request(101, 103, "raw.claim-drop", b"claim", JobPlacement::Isolated);
            request.raw_mut().acknowledge_mount();
            let claim_token = request.recovery.expect("claim drop recovery");
            let claim_identity = (request.raw().kind().as_ptr(), request.raw().input().as_ptr());
            drop(MountedProductReplayClaim { request: Some(request), actor: ActorId(107), record_cursor: 0, checkpoint: None });
            {
                let registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
                assert!(matches!(registry.slots[claim_token.index].owner.as_ref(), Some(MountedProductReplayRecoveryOwner::Claim { request, .. }) if (request.raw.kind().as_ptr(), request.raw.input().as_ptr()) == claim_identity));
            }
            assert_eq!(drain_product_recovery(), 2);

            let (authority, _) = product_authority_fixture();
            let authority_token = authority.request_owner().recovery.expect("authority drop recovery");
            let authority_identity = (authority.request_owner().raw().kind().as_ptr(), authority.request_owner().raw().input().as_ptr());
            drop(authority);
            {
                let registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
                assert!(matches!(registry.slots[authority_token.index].owner.as_ref(), Some(MountedProductReplayRecoveryOwner::Authority { request, .. }) if (request.raw.kind().as_ptr(), request.raw.input().as_ptr()) == authority_identity));
            }
            assert_eq!(drain_product_recovery(), 2);
        }

        #[test]
        fn admitted_raw_backing_retires_only_after_fixed_mount_qualification_and_all_replay_acks() {
            let (mut authority, _) = product_authority_fixture();
            assert!(authority.acknowledge_raw_retirement_boundary().is_err(), "raw backing cannot retire before every replay profile ACK");
            authority.profile_cursor = PRODUCT_REPLAY_PROFILE_COUNT;
            authority.profile_started = false;
            authority.restore_start_ordinal = Some(113);
            authority.acknowledge_raw_retirement_boundary().expect("all fixed, mount, qualification, terminal, and replay ACKs admit retirement");
            let raw = authority.request_owner().raw();
            assert_eq!(
                (raw.fixed_witness_acked, raw.mount_acked, raw.qualification_acked, raw.replay_acked, raw.terminal_ordinal, raw.terminal_prefix, raw.accepted_replay_ordinal, raw.disposition),
                (true, true, true, true, Some(authority.terminal.ordinal), Some(authority.terminal.prefix_digest), Some(113), RawSpawnJobDisposition::Accepted)
            );
            authority.retire();
            assert_eq!(drain_product_recovery(), 2);
        }

        #[test]
        fn product_ingress_kind_and_input_max_plus_one_return_exact_spawn_and_remainder() {
            let kind_max = "k".repeat(PRODUCT_REPLAY_KIND_BYTES);
            let mut kind_max_outcome = ExchangeOutcome {
                frames: Vec::new(),
                surfaces: UiFixedList::default(),
                effects: vec![Effect::SpawnJob { job: 101, kind: kind_max, input: vec![1], placement: JobPlacement::Inline }],
                command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
            };
            let admitted = match kind_max_outcome.take_product_replay_authority(103, product_admission_permit(103)) {
                MountedProductReplayAdmission::Admitted(owner) => owner,
                MountedProductReplayAdmission::None | MountedProductReplayAdmission::Refused(_) => panic!("kind MAX must admit"),
            };
            admitted.retire();
            assert_eq!(drain_product_recovery(), 2);

            let input_max = vec![2; PRODUCT_REPLAY_INPUT_BYTES];
            let mut input_max_outcome = ExchangeOutcome {
                frames: Vec::new(),
                surfaces: UiFixedList::default(),
                effects: vec![Effect::SpawnJob { job: 107, kind: "input.max".to_string(), input: input_max, placement: JobPlacement::Isolated }],
                command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
            };
            let admitted = match input_max_outcome.take_product_replay_authority(109, product_admission_permit(109)) {
                MountedProductReplayAdmission::Admitted(owner) => owner,
                MountedProductReplayAdmission::None | MountedProductReplayAdmission::Refused(_) => panic!("input MAX must admit"),
            };
            admitted.retire();
            assert_eq!(drain_product_recovery(), 2);

            let kind_max_plus_one = "z".repeat(PRODUCT_REPLAY_KIND_BYTES + 1);
            let mut kind_refusal_outcome = ExchangeOutcome {
                frames: Vec::new(),
                surfaces: UiFixedList::default(),
                effects: vec![
                    Effect::Notify { message: "before-kind".to_string() },
                    Effect::SpawnJob { job: 113, kind: kind_max_plus_one.clone(), input: vec![3, 5], placement: JobPlacement::Exclusive },
                    Effect::Notify { message: "after-kind".to_string() },
                ],
                command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
            };
            let refusal = match kind_refusal_outcome.take_product_replay_authority(127, product_admission_permit(127)) {
                MountedProductReplayAdmission::Refused(owner) => owner,
                MountedProductReplayAdmission::None | MountedProductReplayAdmission::Admitted(_) => panic!("kind MAX+1 must return typed refusal"),
            };
            assert_eq!((refusal.instance, refusal.cause, refusal.selected_index), (127, MountedProductReplayRefusalCause::KindCapacity, 1));
            assert!(matches!(refusal.spawn.as_ref(), Some(Effect::SpawnJob { job: 113, kind, input, placement: JobPlacement::Exclusive }) if kind == &kind_max_plus_one && input == &[3, 5]));
            assert!(matches!(refusal.remaining_effects.as_deref(), Some([Effect::Notify { message: before }, Effect::Notify { message: after }]) if before == "before-kind" && after == "after-kind"));
            assert!(kind_refusal_outcome.effects.is_empty());
            refusal.retire();
            let mut kind_close_opportunities = 0;
            while mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work() {
                assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false).progressed);
                kind_close_opportunities += 1;
                assert!(kind_close_opportunities <= 8);
            }
            assert!(kind_close_opportunities >= 6);

            let input_max_plus_one = vec![11; PRODUCT_REPLAY_INPUT_BYTES + 1];
            let mut input_refusal_outcome = ExchangeOutcome {
                frames: Vec::new(),
                surfaces: UiFixedList::default(),
                effects: vec![Effect::SpawnJob { job: 131, kind: "input.max-plus-one".to_string(), input: input_max_plus_one, placement: JobPlacement::Inline }],
                command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
            };
            let refusal = match input_refusal_outcome.take_product_replay_authority(137, product_admission_permit(137)) {
                MountedProductReplayAdmission::Refused(owner) => owner,
                MountedProductReplayAdmission::None | MountedProductReplayAdmission::Admitted(_) => panic!("input MAX+1 must return typed refusal"),
            };
            assert_eq!(refusal.cause, MountedProductReplayRefusalCause::InputCapacity);
            assert!(matches!(refusal.spawn.as_ref(), Some(Effect::SpawnJob { job: 131, input, .. }) if input.len() == PRODUCT_REPLAY_INPUT_BYTES + 1));
            refusal.retire();
            while mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work() {
                assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false).progressed);
            }
        }

        #[test]
        fn product_ingress_recovery_max_plus_one_retries_exact_spawn_then_closes_remainder_incrementally() {
            let reservations: [MountedProductReplayRecoveryToken; JOB_PROGRESS_ACTIVE_CAPACITY] = {
                let mut registry = mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock");
                std::array::from_fn(|_| registry.reserve().expect("fill every product recovery slot"))
            };
            let mut outcome = ExchangeOutcome {
                frames: Vec::new(),
                surfaces: UiFixedList::default(),
                effects: vec![
                    Effect::Notify { message: "before-recovery".to_string() },
                    Effect::SpawnJob { job: 139, kind: "recovery.max-plus-one".to_string(), input: vec![13, 17, 19], placement: JobPlacement::Isolated },
                    Effect::Notify { message: "after-recovery".to_string() },
                ],
                command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
            };
            let refusal = match outcome.take_product_replay_authority(149, product_admission_permit(149)) {
                MountedProductReplayAdmission::Refused(owner) => owner,
                MountedProductReplayAdmission::None | MountedProductReplayAdmission::Admitted(_) => panic!("recovery MAX+1 must return typed refusal"),
            };
            assert_eq!((refusal.cause, refusal.selected_index), (MountedProductReplayRefusalCause::RecoveryCapacity, 1));
            assert!(matches!(refusal.spawn.as_ref(), Some(Effect::SpawnJob { job: 139, kind, input, placement: JobPlacement::Isolated }) if kind == "recovery.max-plus-one" && input == &[13, 17, 19]));
            assert_eq!(refusal.remaining_effects.as_ref().map(Vec::len), Some(2));
            refusal.retire();
            assert!(mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").release(reservations[0]));
            let step = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(true);
            let request = step.request.expect("one recovery opportunity retries the exact fixed request");
            assert_eq!(
                (request.instance, request.job, request.job_kind(), request.request, request.placement),
                (149, 139, b"recovery.max-plus-one".as_slice(), JobReplayRequest::from_spawn("recovery.max-plus-one", &[13, 17, 19]), JobPlacement::Isolated)
            );
            request.retire();
            for token in reservations.into_iter().skip(1) {
                assert!(mounted_product_replay_recovery_registry().lock().expect("product replay recovery lock").release(token));
            }
            let raw_close_opportunities = drain_product_recovery();
            assert_eq!(raw_close_opportunities, 2);
            let mut close_opportunities = 0;
            while mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work() {
                assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false).progressed);
                close_opportunities += 1;
                assert!(close_opportunities <= 8);
            }
            assert!(raw_close_opportunities + close_opportunities >= 6, "raw input, kind, two effects, and remainder backing close remain distinct opportunities");
        }

        #[test]
        fn product_refusal_slot_max_plus_one_stops_before_exchange_and_drop_publishes_exact_generation() {
            let permits: [MountedProductReplayRefusalToken; JOB_PROGRESS_ACTIVE_CAPACITY] = {
                let mut registry = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock");
                std::array::from_fn(|index| registry.reserve(index as u32 + 1).expect("fill fixed refusal permits"))
            };
            assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").reserve(u32::MAX).is_none(), "refusal MAX+1 is denied before ActionBus exchange owns a SpawnJob");
            for token in permits {
                assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").release(token));
            }

            let mut outcome = ExchangeOutcome {
                frames: Vec::new(),
                surfaces: UiFixedList::default(),
                effects: vec![
                    Effect::Notify { message: "drop-before".to_string() },
                    Effect::SpawnJob { job: 151, kind: "d".repeat(PRODUCT_REPLAY_KIND_BYTES + 1), input: vec![23, 29], placement: JobPlacement::Exclusive },
                    Effect::Notify { message: "drop-after".to_string() },
                ],
                command_ingress: semio_framework::kernel::CommandIngressStatus::Idle,
            };
            let refusal = match outcome.take_product_replay_authority(157, product_admission_permit(157)) {
                MountedProductReplayAdmission::Refused(owner) => owner,
                MountedProductReplayAdmission::None | MountedProductReplayAdmission::Admitted(_) => panic!("oversized kind must refuse"),
            };
            let token = refusal.token.expect("typed refusal retains its pre-reserved generation");
            drop(refusal);
            {
                let registry = mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock");
                let owner = registry.slots[token.index].owner.as_ref().expect("ordinary refusal Drop publishes exact owner");
                assert_eq!((registry.slots[token.index].generation, owner.instance, owner.cause, owner.selected_index), (token.generation, 157, MountedProductReplayRefusalCause::KindCapacity, 1));
                assert!(matches!(owner.spawn.as_ref(), Some(Effect::SpawnJob { job: 151, input, placement: JobPlacement::Exclusive, .. }) if input == &[23, 29]));
                assert_eq!(owner.remaining_effects.as_ref().map(Vec::len), Some(2));
            }
            let mut opportunities = 0;
            while mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").has_work() {
                assert!(mounted_product_replay_refusal_registry().lock().expect("product replay refusal lock").step_one(false).progressed);
                opportunities += 1;
                assert!(opportunities <= 8);
            }
            assert!(opportunities >= 6);
        }

        #[test]
        fn production_mounted_process_replay_supports_one_two_four_and_default_without_private_pool() {
            let profiles: [_; PRODUCT_REPLAY_PROFILE_COUNT] = std::array::from_fn(|cursor| mounted_product_replay_profile(7, cursor).expect("same-process logical replay profile"));
            assert_eq!(profiles, [1, 2, 4, 7]);
            assert_eq!(profiles.map(|worker_count| mounted_product_replay_worker_slot(11, worker_count).expect("deterministic logical slot")), [0, 1, 3, 4]);
            assert_eq!(mounted_product_replay_profile(7, PRODUCT_REPLAY_PROFILE_COUNT), None);
            assert_eq!(mounted_product_replay_profile(0, 3), None);
            assert_eq!(mounted_product_replay_worker_slot(u16::MAX, 4), None);
        }

        #[test]
        fn production_product_authority_opportunities_remain_sub_eight_ms() {
            for cursor in 0..PRODUCT_REPLAY_PROFILE_COUNT {
                let started = std::time::Instant::now();
                let worker_count = mounted_product_replay_profile(7, cursor).expect("fixed production profile");
                let _worker_slot = mounted_product_replay_worker_slot(11, worker_count).expect("fixed production slot");
                assert!(started.elapsed() < Duration::from_millis(8), "one product authority control opportunity exceeded 8ms");
            }
            let started = std::time::Instant::now();
            let (authority, expected) = product_authority_fixture();
            let authority = authority.validate(expected).expect("one fixed identity opportunity");
            assert!(started.elapsed() < Duration::from_millis(8), "one product authority validation opportunity exceeded 8ms");
            authority.retire();
            assert_eq!(drain_product_recovery(), 2);
        }

        #[test]
        fn populated_mounted_replay_drop_publishes_exact_generation_and_drains_incrementally() {
            let recovery = mounted_replay_recovery_registry().lock().expect("replay recovery lock").reserve().expect("pre-reserved recovery");
            let route = JobReplayRoute { plugin: [1; 32], package: [2; 32], controller: [3; 32], tool: [4; 32], window: 5, document: [6; 32], request_schema: [7; 32], request_version: 1, request_digest: [8; 32] };
            let authority = JobTurn { job: 19, operation: semio_framework_actor::JobOperation { operation: 23, base_revision: 29, generation: 31, preview_sequence: 0, seed: 37 }, step_sequence: 0 };
            drop(MountedJobReplay {
                actor: ActorId(41),
                authority,
                request: JobReplayRequest::from_spawn("fixture.replay", b"seed"),
                placement: JobPlacement::Inline,
                log: ManuallyDrop::new(JobReplayLog::new(route, 31).expect("generation-qualified replay")),
                captured: None,
                policy: None,
                terminal_seen: false,
                replay_requested: false,
                replay_started: false,
                replay_submit_sequence: None,
                accepted_replay_sequence: None,
                replay_worker_count: 0,
                replay_worker_slot: u16::MAX,
                recovery: Some(recovery),
            });
            {
                let registry = mounted_replay_recovery_registry().lock().expect("replay recovery lock");
                let owner = registry.slots[recovery.index].owner.as_ref().expect("drop published recovery owner");
                assert_eq!((owner.generation, owner.actor, owner.job), (31, ActorId(41), 19));
            }
            let mut opportunities = 0;
            while mounted_replay_recovery_registry().lock().expect("replay recovery lock").has_close_work() {
                assert!(mounted_replay_recovery_registry().lock().expect("replay recovery lock").close_one());
                opportunities += 1;
                assert!(opportunities <= 4);
            }
            assert!(opportunities >= 2, "owner transfer and terminal slot release are distinct grants");
        }

        #[test]
        fn panic_after_mounted_capture_transfers_the_exact_generation_to_incremental_recovery() {
            let recovery = mounted_replay_recovery_registry().lock().expect("replay recovery lock").reserve().expect("pre-reserved panic recovery");
            let route = JobReplayRoute { plugin: [9; 32], package: [8; 32], controller: [7; 32], tool: [6; 32], window: 5, document: [4; 32], request_schema: [3; 32], request_version: 1, request_digest: [2; 32] };
            let authority = JobTurn { job: 43, operation: semio_framework_actor::JobOperation { operation: 47, base_revision: 53, generation: 59, preview_sequence: 0, seed: 61 }, step_sequence: 0 };
            let caught = std::panic::catch_unwind(|| {
                let _mounted = MountedJobReplay {
                    actor: ActorId(67),
                    authority,
                    request: JobReplayRequest::from_spawn("fixture.panic", b"owner"),
                    placement: JobPlacement::Exclusive,
                    log: ManuallyDrop::new(JobReplayLog::new(route, 59).expect("generation-qualified panic replay")),
                    captured: None,
                    policy: None,
                    terminal_seen: false,
                    replay_requested: false,
                    replay_started: false,
                    replay_submit_sequence: None,
                    accepted_replay_sequence: None,
                    replay_worker_count: 0,
                    replay_worker_slot: u16::MAX,
                    recovery: Some(recovery),
                };
                panic!("mounted replay panic fixture");
            });
            assert!(caught.is_err());
            {
                let registry = mounted_replay_recovery_registry().lock().expect("replay recovery lock");
                let owner = registry.slots[recovery.index].owner.as_ref().expect("panic published exact owner");
                assert_eq!((owner.generation, owner.actor, owner.job), (59, ActorId(67), 43));
            }
            let mut opportunities = 0;
            while mounted_replay_recovery_registry().lock().expect("replay recovery lock").slots[recovery.index].reserved {
                assert!(mounted_replay_recovery_registry().lock().expect("replay recovery lock").close_one());
                opportunities += 1;
                assert!(opportunities <= JOB_PROGRESS_ACTIVE_CAPACITY * 4);
            }
            assert!(opportunities >= 2);
        }

        fn destroy_request(instance: u32) -> KernelRequest {
            KernelRequest::DestroyApp {
                owner: Arc::new(KernelCloseSubmission {
                    instance,
                    realm: false,
                    generation: u64::from(instance) + 1,
                    queue: Arc::new(KernelRequestQueue::default()),
                    pool: crate::renderer_worker_pool(),
                    registry: std::sync::Weak::new(),
                    phase: std::sync::atomic::AtomicU8::new(KERNEL_CLOSE_QUEUED),
                }),
            }
        }

        fn close_submission(registry: &Arc<KernelCloseSubmissionRegistry>, instance: u32, generation: u64) -> Arc<KernelCloseSubmission> {
            Arc::new(KernelCloseSubmission {
                instance,
                realm: false,
                generation,
                queue: Arc::new(KernelRequestQueue::default()),
                pool: crate::renderer_worker_pool(),
                registry: Arc::downgrade(registry),
                phase: std::sync::atomic::AtomicU8::new(KERNEL_CLOSE_UNADMITTED),
            })
        }

        fn command_request(instance: u32, generation: u64, page_count: usize) -> KernelRequest {
            let mut pages = semio_framework::kernel::CommandPageSet::try_new().unwrap();
            for index in 0..page_count {
                let page = if index + 1 == page_count {
                    semio_framework::kernel::FixedCommandPage::try_copy_from(b"tail").unwrap()
                } else {
                    semio_framework::kernel::FixedCommandPage::try_copy_from(&[3; semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES]).unwrap()
                };
                pages.try_push(page).unwrap();
            }
            let command = semio_framework::kernel::PagedCommand::try_from_pages(pages).unwrap();
            let mut commands = semio_framework::kernel::CommandEnvelopeSet::try_new().unwrap();
            commands.try_push(semio_framework::kernel::CommandEnvelope { instance, seq: generation, command }).unwrap();
            let batch = semio_framework::kernel::CommandBatch::try_new(generation, commands).unwrap();
            KernelRequest::ExchangeCommands { instance, driver: semio_framework::kernel::CommandBatchDriver::new(generation, batch) }
        }

        #[test]
        fn fixed_kernel_request_queue_returns_capacity_plus_one_owner_and_preserves_fifo() {
            let queue = KernelRequestQueue::default();
            for instance in 0..KERNEL_REQUEST_QUEUE_CAPACITY as u32 {
                queue.try_push(destroy_request(instance), Arc::new(ResponseSlot::default()), None).unwrap();
            }
            let (rejected, _) = queue.try_push(destroy_request(999), Arc::new(ResponseSlot::default()), None).unwrap_err();
            assert!(matches!(rejected, KernelRequest::DestroyApp { ref owner } if owner.instance == 999));
            let waker = Waker::noop();
            let mut context = Context::from_waker(waker);
            for expected in 0..KERNEL_REQUEST_QUEUE_CAPACITY as u32 {
                let Poll::Ready((request, _)) = queue.poll(&mut context) else { panic!("fixed request was ready") };
                assert!(matches!(request, KernelRequest::DestroyApp { ref owner } if owner.instance == expected));
            }
        }

        #[test]
        fn fixed_kernel_request_queue_rejects_aggregate_page_credit_plus_one_exactly() {
            let queue = KernelRequestQueue::default();
            queue.try_push(command_request(1, 7, semio_framework::kernel::COMMAND_MAXIMUM_PAGES), Arc::new(ResponseSlot::default()), None).unwrap();
            let (rejected, _) = queue.try_push(command_request(2, 8, 1), Arc::new(ResponseSlot::default()), None).unwrap_err();
            assert!(matches!(rejected, KernelRequest::ExchangeCommands { instance: 2, ref driver } if driver.remaining_pages() == 1));
        }

        #[test]
        fn fixed_kernel_request_queue_contention_returns_the_untouched_owner() {
            let queue = KernelRequestQueue::default();
            let guard = queue.state.lock().unwrap();
            let (rejected, _) = queue.try_push(command_request(7, 11, 1), Arc::new(ResponseSlot::default()), None).unwrap_err();
            assert!(matches!(rejected, KernelRequest::ExchangeCommands { instance: 7, ref driver } if driver.generation() == 11 && driver.remaining_pages() == 1));
            drop(guard);
        }

        #[test]
        fn fixed_kernel_close_registry_returns_the_exact_modulo_collision_and_reuses_only_after_terminal_generation() {
            let registry = Arc::new(KernelCloseSubmissionRegistry::new());
            let first = close_submission(&registry, 3, 11);
            assert!(first.try_admit());
            assert!(registry.contains(3, 11));
            let collision = close_submission(&registry, 3 + KERNEL_CLOSE_SUBMISSION_CAPACITY as u32, 12);
            assert!(!collision.try_admit());
            assert_eq!(collision.instance, 3 + KERNEL_CLOSE_SUBMISSION_CAPACITY as u32);
            assert_eq!(collision.generation, 12);
            assert!(registry.contains(3, 11));
            first.finish(KernelCloseStatus::Complete);
            assert!(!registry.contains(3, 11));
            assert!(collision.try_admit());
            assert!(registry.contains(3 + KERNEL_CLOSE_SUBMISSION_CAPACITY as u32, 12));
        }

        #[test]
        fn fixed_kernel_close_registry_contention_returns_unadmitted_owner_for_exact_retry() {
            let registry = Arc::new(KernelCloseSubmissionRegistry::new());
            let owner = close_submission(&registry, 8, 21);
            let guard = registry.slots.lock().unwrap();
            assert!(!owner.try_admit());
            assert_eq!(owner.phase.load(std::sync::atomic::Ordering::Acquire), KERNEL_CLOSE_UNADMITTED);
            assert_eq!(owner.instance, 8);
            assert_eq!(owner.generation, 21);
            drop(guard);
            assert!(owner.try_admit());
            assert!(registry.contains(8, 21));
        }

        #[test]
        fn fixed_kernel_request_shutdown_faults_the_retained_close_handle_before_terminal_removal() {
            let queue = KernelRequestQueue::default();
            let owner = match destroy_request(19) {
                KernelRequest::DestroyApp { owner } => owner,
                _ => unreachable!(),
            };
            queue.try_push(KernelRequest::DestroyApp { owner: owner.clone() }, Arc::new(ResponseSlot::default()), None).unwrap();
            assert!(queue.begin_shutdown());
            assert_eq!(queue.shutdown_step(0), (true, 1, 0));
            assert_eq!(owner.terminal_status(), Some(KernelCloseStatus::Fault));
        }

        #[test]
        fn fixed_kernel_request_queue_shutdown_releases_one_real_page_per_grant() {
            let queue = KernelRequestQueue::default();
            queue.try_push(command_request(3, 12, 2), Arc::new(ResponseSlot::default()), None).unwrap();
            assert!(queue.begin_shutdown());
            assert_eq!(queue.shutdown_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES - 1), (false, 0, 0));
            assert_eq!(queue.shutdown_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES), (false, 1, semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES));
            assert_eq!(queue.shutdown_step(semio_framework::kernel::COMMAND_PAGE_MAXIMUM_BYTES), (true, 1, 4));
        }

        #[test]
        fn fixed_kernel_request_queue_shutdown_releases_create_fields_one_owner_per_grant() {
            let queue = KernelRequestQueue::default();
            queue.try_push(KernelRequest::CreateApp { owner: CreateAppRequestOwner::new(PathBuf::from("path"), "plugin".to_string(), "app".to_string()) }, Arc::new(ResponseSlot::default()), None).unwrap();
            assert!(queue.begin_shutdown());
            assert_eq!(queue.shutdown_step(3), (false, 0, 0));
            assert_eq!(queue.shutdown_step(4), (false, 1, 4));
            assert_eq!(queue.shutdown_step(6), (false, 1, 6));
            assert_eq!(queue.shutdown_step(3), (true, 1, 3));
        }

        #[test]
        fn fixed_kernel_request_queue_shutdown_releases_surface_and_rejected_events_in_fifo_units() {
            let queue = KernelRequestQueue::default();
            queue.try_push(KernelRequest::Exchange { instance: 1, event: QueuedKernelEvent { surface_visible: Some("surface".to_string()) } }, Arc::new(ResponseSlot::default()), None).unwrap();
            queue.try_push(KernelRequest::CloseRejectedEvents { owner: RejectedKernelEvents { events: std::collections::VecDeque::from([Event::Wake, Event::Wake]) } }, Arc::new(ResponseSlot::default()), None).unwrap();
            assert!(queue.begin_shutdown());
            assert_eq!(queue.shutdown_step(6), (false, 0, 0));
            assert_eq!(queue.shutdown_step(7), (false, 1, 7));
            assert_eq!(queue.shutdown_step(0), (false, 1, 0));
            assert_eq!(queue.shutdown_step(0), (true, 1, 0));
        }
    }
    //#endregion
}
//#endregion 🎠️KernelRuntime

//#region 🔖️ActorBudgetBridge
/// ⚖️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (terra-kernel-loop, unblocking terra-shard-grants'
/// `ShardFrame::Grant` wire change): `semio_framework::kernel::Budget` (what this crate's own
/// `kernel_runtime::TURN_BUDGET`/`scale_bench::turn_budget_of` already speak) →
/// `semio_framework_actor::Budget` (what a `Grant` frame carries over `ShardTransport`, replacing
/// the deleted `ShardLoop::pump(|actor| ..)` budget closure). Shared by both native call sites
/// (`kernel_runtime::run_turn`, `scale_bench::Env::send_payload`) rather than duplicated — CLAUDE.md's
/// "if code is repeated, it MUST be close to each other" — so it lives here, the one file both
/// `#[cfg(not(target_arch = "wasm32"))]` modules already share. `memory_bytes`/`ui_nodes`/
/// `mailbox_len` have no source field on the kernel-`Budget` side; defaulted from `lane` via
/// `lane_defaults::budget_for` rather than invented — the same documented-gap shape
/// `🖥️host/🧵️shard/🦀️.rs`'s own `BudgetBridge` region already uses for the REVERSE
/// direction (`GRANT_BUDGET_DEFAULT_MAX_FRAMES`).
#[cfg(not(target_arch = "wasm32"))]
fn actor_budget_from_turn_budget(budget: semio_framework::kernel::Budget, lane: semio_framework_actor::Lane) -> semio_framework_actor::Budget {
    let base = semio_framework_actor::lane_defaults::budget_for(lane);
    semio_framework_actor::Budget { fuel: budget.fuel, wall_ms: budget.deadline_ms, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, ..base }
}
//#endregion 🔖️ActorBudgetBridge

//#region 🔖️ScaleBench
/// 🧪️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (packet V1b-bench): the native half of the ticket's
/// headline claim — "50+ plugins x 50+ extensions concurrently" — turned from measurable into
/// measured. Drives the REAL `semio-framework-os-scale-fixture` `wasm32-wasip2` component (one
/// compile, many instantiations, exactly the pooling-allocator scenario `build_shared_engine` was
/// built for) through `crate::parallel_runtime::ParallelRuntime` (terra-kernel-loop) — the same engine `//#region 🎠️KernelRuntime`
/// above already wires for the winit renderer, reused here without the winit/GPU half. terra-kernel-loop
/// upgraded this from a single physical `ShardLoop` behind all K shard labels to K real `ShardExecutor`
/// OS threads (see `Env`'s own doc for what this fixed for budgets 3/5/6). `bun nx run @semio-tech/framework-os-dev:bench -- plugins --renderer native`
/// (`🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts`'s `//#region 🔖️Bench`) drives this via
/// `semio-wgpu-native --scale/--scale-wasm/--shards/--report`.
#[cfg(not(target_arch = "wasm32"))]
pub mod scale_bench {
    use semio_framework::kernel::{AppInstanceId, Budget as TurnBudget, CapabilityChange, CapabilityId, Effect, Event, PluginInstanceId, QuotaSchema, TurnResult};
    use semio_framework_actor::{ActivationEvent as ActorActivationTrigger, ActorId, ActorKind, Envelope, JobCheckpoint, JobOperation, Kernel, Lane, Origin, PackageHash, PackageId, Payload};
    use semio_framework_plugin_host::shard::ShardOutcome;
    use semio_framework_plugin_host::{CompiledHandle, GuestRuntime, GuestRuntimes, OwnedRuntime, PackageRef};
    use serde::Deserialize;
    use serde_json::json;
    use std::collections::HashMap;
    use std::path::PathBuf;
    use std::sync::Arc;
    use std::time::{Duration, Instant};

    /// ⏳️ terra-kernel-loop: how long `Env::pump` waits, per `Kernel::tick` call, for that tick's
    /// granted turns' `ShardOutcome`s to arrive from their K real `ShardExecutor` threads before
    /// treating the round as failed — generous (well past any interactive budget this ticket
    /// measures) so it is a genuine "something is stuck" tripwire, never a floor under budget 5's
    /// own timing (budget 5 measures the ACTUAL wait via `wait_for_outcomes`'s elapsed time, not
    /// this ceiling).
    const PUMP_OUTCOME_TIMEOUT: Duration = Duration::from_secs(10);

    //#region 🔖️RegistryJson
    #[derive(Deserialize, Clone, Copy)]
    #[serde(rename_all = "camelCase")]
    struct RegistryQuotas {
        deadline_ms: u32,
        max_effects: u32,
        max_patch_bytes: u32,
        max_frames: u32,
    }

    /// 🌉️ `scaleFixture`/`activationEvents` are kept as raw `serde_json::Value` rather than a second
    /// typed mirror of `FixtureConfig` (`🎭️profile/🦀️.rs`) — the fixture's own guest re-parses
    /// this crate's re-serialized bytes with `serde_json::from_slice`, so byte-for-byte field-name
    /// fidelity matters more here than a typed struct's convenience, and there is exactly one JSON
    /// shape (the TS generator's) to stay honest to.
    #[derive(Deserialize, Clone)]
    #[serde(rename_all = "camelCase")]
    struct RegistryRecord {
        id: String,
        kind: String,
        parent_id: Option<String>,
        activation_events: Vec<serde_json::Value>,
        quotas: RegistryQuotas,
        scale_fixture: serde_json::Value,
    }

    #[derive(Deserialize)]
    struct RegistryFile {
        records: Vec<RegistryRecord>,
    }

    fn profile_of(record: &RegistryRecord) -> &str {
        record.scale_fixture.get("profile").and_then(|v| v.as_str()).unwrap_or("idle")
    }

    fn is_startup(record: &RegistryRecord) -> bool {
        record.activation_events.iter().any(|e| e.get("type").and_then(|v| v.as_str()) == Some("on-startup-finished"))
    }

    /// 🎭️ Faults from actors that were NOT supposed to trap. The fixture ships `hang` (393 records)
    /// and `crash` (343 records) precisely so the watchdog and the failure ladder have something to
    /// catch — together 29% of the catalog — so a blanket `faults == 0` pass condition is really
    /// asking the crash profile not to crash, and it failed budgets 2 and 3 on a sample of the
    /// fixture behaving exactly as designed. `📓️design-workforce.md` §4 does not put a fault
    /// criterion on either budget: budget 2 is a deadline plus "only on-startup-finished actors
    /// live", budget 3 is actor count, shard count and per-shard ceiling. A trap from an
    /// `idle`/`cpu`/`ui`/`io`/`stateful` actor IS a real failure and still counts.
    fn unexpected_faults(outcomes: &[ShardOutcome], actors: &[ActorId], records: &[&RegistryRecord]) -> Vec<String> {
        let by_design: std::collections::HashSet<u64> = actors.iter().zip(records.iter()).filter(|(_, record)| matches!(profile_of(record), "hang" | "crash")).map(|(actor, _)| actor.0).collect();
        outcomes
            .iter()
            .filter_map(|outcome| match outcome {
                ShardOutcome::Fault { actor, message } if !by_design.contains(actor) => Some(message.clone()),
                _ => None,
            })
            .collect()
    }

    /// ⛽️ Bench-wide fuel ceiling — not per-record (`RegistryQuotas` omits fuel on purpose: wasmtime
    /// dispatch + wit-bindgen overhead in an unoptimized `wasip2` build dwarfs plausible production
    /// per-turn ceilings; measured reference: `🗒️note`'s `describe()` alone burns ~92M fuel in debug).
    /// `deadline_ms`/`max_effects`/`max_patch_bytes`/`max_frames` stay record-derived (real per-turn
    /// dimensions this bench exercises, e.g. budget 6's hang deadline).
    const BENCH_FUEL: u64 = 200_000_000;

    fn turn_budget_of(record: &RegistryRecord) -> TurnBudget {
        TurnBudget { fuel: BENCH_FUEL, deadline_ms: record.quotas.deadline_ms, max_effects: record.quotas.max_effects, max_patch_bytes: record.quotas.max_patch_bytes, max_frames: record.quotas.max_frames }
    }

    fn instance_open_event(record: &RegistryRecord, instance_id: u32) -> Event {
        Event::InstanceOpen {
            instance: PluginInstanceId(instance_id.to_string()),
            app_id: AppInstanceId(record.id.clone()),
            actor: "bench".to_string(),
            config: serde_json::to_vec(&record.scale_fixture).unwrap_or_default(),
            assets: Vec::new(),
            capabilities: Vec::new(),
            quotas: QuotaSchema::default(),
        }
    }
    //#endregion 🔖️RegistryJson

    //#region 🔖️Row
    fn row(id: u32, description: &str, status: &str, measured: serde_json::Value, threshold: serde_json::Value, note: &str) -> serde_json::Value {
        json!({ "id": id, "description": description, "status": status, "measured": measured, "threshold": threshold, "note": note })
    }

    fn skipped(id: u32, description: &str, reason: &str) -> serde_json::Value {
        row(id, description, "skipped", serde_json::Value::Null, serde_json::Value::Null, reason)
    }
    //#endregion 🔖️Row

    //#region 🔖️Env
    /// 🧵️ terra-kernel-loop: `Env` now drives its actors through `crate::parallel_runtime::
    /// ParallelRuntime` — real `Kernel::submit`/`tick`/`complete` plus K real `ShardExecutor` OS
    /// threads, ONE per configured shard, replacing the single physical `ShardLoop` every actor's
    /// turn used to serialize behind regardless of `shard_count`. This is what makes budget 3's
    /// "shard assignment" check and budget 5's "interactive p95 under 40 cpu actors" measure a REAL
    /// K-way-parallel instrument for the first time — see `📓️terra-kernel-loop-report.md`.
    /// `Kernel::complete` is also now genuinely called from `pump` (closing the gap `//#region
    /// 🎠️KernelRuntime`'s own `apply_turn_result` doc and budget 8's own note both used to flag).
    struct Env {
        runtime: super::parallel_runtime::ParallelRuntime,
        budgets: HashMap<u64, TurnBudget>,
        seq: u64,
        ordinals: HashMap<String, u16>,
        now_ms: u64,
        /// 🌀️ `ShardOutcome`s already pulled off `runtime`'s aggregated forwarder channel by `pump`
        /// but not yet handed to a caller's `drain()` — mirrors the pre-existing single-`ShardLoop`
        /// `Env::drain`'s own "whatever's on the wire right now" contract, just sourced from a
        /// buffer instead of a single in-process channel (outcomes now arrive asynchronously from K
        /// real threads, so `pump` must collect them eagerly rather than leaving them for `drain` to
        /// read off a transport that no longer exists as a single queue).
        pending: Vec<ShardOutcome>,
    }

    impl Env {
        async fn new(runtime: Arc<GuestRuntimes>, shard_count: u16) -> Self {
            let cores = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
            let pool = Arc::new(semio_framework_async::process_worker_pool(semio_framework_async::WorkerPoolConfig::new(semio_framework_async::ProcessKind::InteractiveNative, cores)));
            let runtime = super::parallel_runtime::ParallelRuntime::new(pool, runtime, shard_count.max(1), 0, 64).await;
            Self { runtime, budgets: HashMap::new(), seq: 0, ordinals: HashMap::new(), now_ms: 0, pending: Vec::new() }
        }

        fn kernel(&self) -> &Kernel {
            self.runtime.kernel()
        }

        fn ordinal(&mut self, package_id: &str) -> u16 {
            let next = self.ordinals.len() as u16;
            *self.ordinals.entry(package_id.to_string()).or_insert(next)
        }

        /// 🎚️ Activation lane defaults to `Background` for every budget (unchanged). Budget 5 is the
        /// sole caller of [`Self::activate_on_lane`] with `Lane::Interactive`: the budget's own text
        /// names an *interactive* actor, and the kernel's placement gate keys off the actor's
        /// ACTIVATION lane, so activating the probe as `Background` measured a background actor and
        /// left the interactive path untested. This is an instrument correction, not a threshold change.
        async fn activate(&mut self, compiled: &CompiledHandle, record: &RegistryRecord) -> Result<ActorId, String> {
            self.activate_on_lane(compiled, record, Lane::Background).await
        }

        async fn activate_on_lane(&mut self, compiled: &CompiledHandle, record: &RegistryRecord, lane: Lane) -> Result<ActorId, String> {
            let kind = if record.kind == "extension" {
                ActorKind::Extension { plugin: PackageId(record.parent_id.clone().unwrap_or_default()), extension_id: record.id.clone() }
            } else {
                ActorKind::PluginApp { plugin: PackageId(record.id.clone()), app_id: record.id.clone(), instance_id: 0 }
            };
            let package_id = record.parent_id.clone().unwrap_or_else(|| record.id.clone());
            let ordinal = self.ordinal(&package_id);
            let budget = turn_budget_of(record);
            let actor = self.runtime.activate(PackageId(package_id), ordinal, kind, lane, None, ActorActivationTrigger::Manual, compiled, &[], &budget).await?;
            self.budgets.insert(actor.0, budget);
            Ok(actor)
        }

        async fn send(&mut self, actor: ActorId, event: &Event) {
            self.send_payload(actor, Payload::Event { bytes: serde_json::to_vec(event).unwrap_or_default() }).await;
        }

        /// 🔀️ `Payload::Suspend`/`Payload::Resume`/`Payload::Cancel` need the same envelope plumbing
        /// as `send`'s `Payload::Event` — factored out so budget 7 (K1's now-unblocked Suspend/Resume
        /// dispatch) can drive them without duplicating the seq/envelope bookkeeping.
        ///
        /// 🐛️ terra-kernel-loop: now a real `Kernel::submit` — the DRR mailbox enqueue, drained by
        /// the NEXT `pump`'s `tick_and_dispatch` call — replacing the ad-hoc direct `ShardFrame::
        /// Grant` this method sent before `Env` had a real `Kernel::tick` loop to submit into.
        /// `Backpressure` is intentionally not surfaced to the caller: every round this bench drives
        /// sends at most a handful of envelopes per actor, far under any lane's mailbox capacity
        /// (128-1024 depending on lane, `lane_defaults::budget_for`), so treating a reject as fatal
        /// here would be testing the mailbox ceiling, not the budget this harness measures.
        async fn send_payload(&mut self, actor: ActorId, payload: Payload) {
            self.send_payload_lane(actor, payload, Lane::Background).await;
        }

        /// 🎯️ terra-bench-instrument: sibling of `send_payload` that lets a caller pick the
        /// envelope's own `Lane` instead of the hardcoded `Lane::Background` every other send in
        /// this harness still uses (`send_payload` now delegates here with `Lane::Background`
        /// unchanged, so every existing call site — budgets 2/3/4/6/7/8's `env.send`, budget 7's
        /// direct `Payload::Suspend`/`Resume` sends — keeps the exact envelope it always sent).
        /// Budget 5 is the ONLY caller that passes `Lane::Interactive`, for the one envelope this
        /// bench ever sends that is meant to model a real interactive command — see that budget's
        /// own round loop for why: the instrument was found to send EVERY bench envelope, including
        /// the "interactive" probe, on `Lane::Background`, which both skips whatever lane-priority
        /// the mailbox/DRR machinery gives `Lane::Interactive` and structurally cannot activate the
        /// terra-interactive-isolation packet's `Kernel::activate`-time placement gate (that gate
        /// reads the ACTOR's own activation lane, set once in `Env::activate` above — unconditionally
        /// `Lane::Background` for every bench actor, out of scope here since `Env::activate` is
        /// shared by every budget, not just 5 — so fixing only this envelope's lane does not, by
        /// itself, make that isolation mechanism reachable from this bench; see this packet's own
        /// report for the honest gap).
        async fn send_payload_lane(&mut self, actor: ActorId, payload: Payload, lane: Lane) {
            self.seq += 1;
            let envelope = Envelope { to: actor, from: Origin::Kernel, lane, seq: self.seq, deadline_ms: None, coalesce: None, cancel_of: None, payload };
            let _ = self.runtime.submit(&envelope).await;
        }

        /// ⏱️ terra-kernel-loop: `Kernel::tick`-drives every actor with a non-empty mailbox to
        /// completion — looping `tick_and_dispatch` until a tick grants nothing (`grants_per_tick`
        /// caps a SINGLE tick's grants at 64, so draining >64 pending actors, e.g. budget 3/4/5's
        /// 100-2550-actor rounds, genuinely takes several ticks; this loop is what makes that real
        /// instead of assuming one call suffices). Each tick's `ShardOutcome`s are awaited via
        /// `wait_for_outcomes` — a genuine blocking wait on the SAME aggregated channel K real
        /// `ShardExecutor` threads report through. `Kernel::complete` is called for every
        /// `ShardOutcome::Turn` collected, closing the gap budget 8's own note used to flag.
        ///
        /// 🎯️ terra-bench-instrument correction: this method's own `start.elapsed()`, timed by a
        /// caller around a WHOLE call, is round wall-time across every actor granted that round —
        /// budget 5 used to time itself this way and that is exactly the defect this packet fixed;
        /// budget 5 now uses `pump_tracking` below instead, which stamps the moment ONE specific
        /// actor's own outcome is observed rather than waiting on this method's own return.
        async fn pump(&mut self) -> Result<usize, String> {
            let mut total = 0usize;
            loop {
                self.now_ms += 1;
                // 🔀️ Cloned BEFORE the call (a small `HashMap<u64, TurnBudget>`, one per activated
                // actor) so the closure below borrows THIS local binding, not `self` — `self.runtime.
                // tick_and_dispatch(..)` already holds `self.runtime` mutably for the duration of the
                // call, and a closure capturing `&self.budgets` directly would conflict with that.
                let budgets = self.budgets.clone();
                let fallback = TurnBudget { fuel: BENCH_FUEL, deadline_ms: 50, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
                let decision = self.runtime.tick_and_dispatch(self.now_ms, |actor| crate::actor_budget_from_turn_budget(budgets.get(&actor.0).copied().unwrap_or(fallback), Lane::Background)).await;
                if decision.run.is_empty() {
                    break;
                }
                let outcomes = self.runtime.wait_for_outcomes(decision.run.len(), PUMP_OUTCOME_TIMEOUT);
                if outcomes.len() < decision.run.len() {
                    let missing = decision.run.len() - outcomes.len();
                    self.pending.extend(outcomes);
                    return Err(format!("Env::pump: {missing} of {} granted turns produced no ShardOutcome within {PUMP_OUTCOME_TIMEOUT:?}", decision.run.len()));
                }
                for outcome in &outcomes {
                    match outcome {
                        ShardOutcome::Turn { actor, result } => {
                            let _ = self.runtime.complete_actor(ActorId(*actor), result, self.now_ms).await;
                        }
                        // 🎠️ terra-kernel-loop: same reasoning as `kernel_runtime::run_turn`'s own
                        // `ShardOutcome::Fault` arm — a trap must reach `Kernel::complete` too, or the
                        // failure ladder never sees the SAME "hang"/"crash" profiles budgets 2/3/6
                        // deliberately exercise.
                        ShardOutcome::Fault { actor, message } => {
                            let faulted = semio_framework_actor::TurnResult {
                                ui_patches: Vec::new(),
                                effects: Vec::new(),
                                command_ingress: Vec::new(),
                                next_wake: None,
                                status: semio_framework_actor::TurnStatus::Faulted { detail: message.clone().into_bytes() },
                                usage: semio_framework_actor::Usage::default(),
                            };
                            let _ = self.runtime.complete_actor(ActorId(*actor), &faulted, self.now_ms).await;
                        }
                        _ => {}
                    }
                }
                total += outcomes.len();
                self.pending.extend(outcomes);
            }
            Ok(total)
        }

        /// 🎯️ terra-bench-instrument: same `Kernel::tick`-drives-every-granted-actor-to-completion
        /// shape as `pump` above — every actor granted this round, `target` included, still gets a
        /// genuine `tick_and_dispatch` → `wait_for_outcomes` → `Kernel::complete` round trip, so the
        /// kernel's own bookkeeping (fuel/throttle/mailbox state) ends this call exactly as
        /// consistent as `pump` would leave it, and the next round starts clean. The ONLY behavioural
        /// difference: `pump`'s own `wait_for_outcomes(decision.run.len(), ..)` blocks for a WHOLE
        /// tick's outcomes at once, so nothing is observable until the SLOWEST of that tick's actors
        /// has reported in. This method instead waits one outcome at a time
        /// (`wait_for_outcomes(1, ..)`) — the same total outcomes arrive, just individually — and
        /// stamps `Instant::now()` the first moment `target`'s own `ShardOutcome` (`Turn` or `Fault`,
        /// whichever arrives) is among them. That stamp, not this call's own return, is budget 5's
        /// actual measurement: see its round loop for why the interval is `send -> this stamp`, not
        /// `send -> this call returning`.
        async fn pump_tracking(&mut self, target: ActorId) -> Result<Option<Instant>, String> {
            let mut target_seen: Option<Instant> = None;
            loop {
                self.now_ms += 1;
                let budgets = self.budgets.clone();
                let fallback = TurnBudget { fuel: BENCH_FUEL, deadline_ms: 50, max_effects: 8, max_patch_bytes: 4096, max_frames: 1 };
                let decision = self.runtime.tick_and_dispatch(self.now_ms, |actor| crate::actor_budget_from_turn_budget(budgets.get(&actor.0).copied().unwrap_or(fallback), Lane::Background)).await;
                if decision.run.is_empty() {
                    break;
                }
                let mut remaining = decision.run.len();
                while remaining > 0 {
                    let outcomes = self.runtime.wait_for_outcomes(1, PUMP_OUTCOME_TIMEOUT);
                    if outcomes.is_empty() {
                        return Err(format!("Env::pump_tracking: {remaining} granted turns produced no ShardOutcome within {PUMP_OUTCOME_TIMEOUT:?}"));
                    }
                    remaining = remaining.saturating_sub(outcomes.len());
                    for outcome in &outcomes {
                        let reporting_actor = match outcome {
                            ShardOutcome::Turn { actor, result } => {
                                let _ = self.runtime.complete_actor(ActorId(*actor), result, self.now_ms).await;
                                Some(*actor)
                            }
                            ShardOutcome::Fault { actor, message } => {
                                let faulted = semio_framework_actor::TurnResult {
                                    ui_patches: Vec::new(),
                                    effects: Vec::new(),
                                    command_ingress: Vec::new(),
                                    next_wake: None,
                                    status: semio_framework_actor::TurnStatus::Faulted { detail: message.clone().into_bytes() },
                                    usage: semio_framework_actor::Usage::default(),
                                };
                                let _ = self.runtime.complete_actor(ActorId(*actor), &faulted, self.now_ms).await;
                                Some(*actor)
                            }
                            _ => None,
                        };
                        if target_seen.is_none() && reporting_actor == Some(target.0) {
                            target_seen = Some(Instant::now());
                        }
                    }
                    self.pending.extend(outcomes);
                }
            }
            Ok(target_seen)
        }

        fn drain(&mut self) -> Vec<ShardOutcome> {
            self.pending.extend(self.runtime.try_recv_outcomes());
            std::mem::take(&mut self.pending)
        }

        async fn unregister(&mut self, actor: ActorId) {
            self.runtime.unregister(actor).await;
        }
    }
    //#endregion 🔖️Env

    async fn process_rss_bytes() -> Option<u64> {
        match crate::run_renderer_io(semio_framework_os_services::NativeIoRequest::ProcessResidentBytes).await.ok()? {
            semio_framework_os_services::NativeIoValue::ResidentBytes(bytes) => bytes,
            _ => None,
        }
    }

    //#region 🔖️Budget2ColdBoot
    async fn budget_2_cold_boot(process_start: Instant, runtime: &Arc<GuestRuntimes>, compiled: &CompiledHandle, records: &[RegistryRecord], shard_count: u16, native_budget_ms: u64) -> serde_json::Value {
        let startup: Vec<&RegistryRecord> = records.iter().filter(|r| is_startup(r)).collect();
        if startup.is_empty() {
            return skipped(2, "cold boot to first interactive frame, only on-startup-finished actors live", "registry carries no on-startup-finished record");
        }
        let mut env = Env::new(runtime.clone(), shard_count).await;
        let mut actors = Vec::with_capacity(startup.len());
        for (index, record) in startup.iter().enumerate() {
            match env.activate(compiled, record).await {
                Ok(actor) => actors.push(actor),
                Err(error) => return row(2, "cold boot to first interactive frame, only on-startup-finished actors live", "fail", json!({ "error": error }), json!({ "nativeMs": native_budget_ms }), "activate/instantiate failed mid cold-boot"),
            }
            env.send(actors[index], &instance_open_event(record, index as u32 + 1)).await;
        }
        if let Err(error) = env.pump().await {
            return row(2, "cold boot to first interactive frame, only on-startup-finished actors live", "fail", json!({ "error": error }), json!({ "nativeMs": native_budget_ms }), "ShardLoop::pump failed");
        }
        let outcomes = env.drain();
        let elapsed_ms = process_start.elapsed().as_millis() as u64;
        let faults = unexpected_faults(&outcomes, &actors, &startup);
        let active = env.kernel().metrics().await.actors;
        let only_startup_live = active as usize == startup.len();
        let pass = faults.is_empty() && only_startup_live && elapsed_ms <= native_budget_ms;
        row(
            2,
            "cold boot to first interactive frame, only on-startup-finished actors live",
            if pass { "pass" } else { "fail" },
            json!({ "elapsedMs": elapsed_ms, "startupActorCount": startup.len(), "activeActorsAfterBoot": active, "faultCount": faults.len(), "faults": faults.iter().take(5).collect::<Vec<_>>() }),
            json!({ "nativeMs": native_budget_ms }),
            "measured from process entry (before engine build/wasm compile) to the last on-startup-finished actor's InstanceOpen turn completing",
        )
    }
    //#endregion 🔖️Budget2ColdBoot

    //#region 🔖️Budget3Activate100
    async fn budget_3_activate_100(runtime: &Arc<GuestRuntimes>, compiled: &CompiledHandle, records: &[RegistryRecord], shard_count: u16) -> serde_json::Value {
        let plugin_records: Vec<&RegistryRecord> = records.iter().filter(|r| r.kind == "plugin").take(50).collect();
        if plugin_records.is_empty() {
            return skipped(3, "activate 50 plugins + 50 extensions of one plugin", "registry carries no plugin-kind record");
        }
        let target_plugin_id = plugin_records[0].id.clone();
        let ext_records: Vec<&RegistryRecord> = records.iter().filter(|r| r.kind == "extension" && r.parent_id.as_deref() == Some(target_plugin_id.as_str())).collect();
        let mut env = Env::new(runtime.clone(), shard_count).await;
        let mut activated: Vec<ActorId> = Vec::new();
        let selected: Vec<&RegistryRecord> = plugin_records.iter().chain(ext_records.iter()).copied().collect();
        let mut instance_id = 1u32;
        for record in &selected {
            match env.activate(compiled, record).await {
                Ok(actor) => {
                    env.send(actor, &instance_open_event(record, instance_id)).await;
                    instance_id += 1;
                    activated.push(actor);
                }
                Err(error) => return row(3, "activate 50 plugins + 50 extensions of one plugin", "fail", json!({ "error": error }), json!({ "activeActors": 100, "shards": shard_count }), "activate/instantiate failed"),
            }
        }
        if let Err(error) = env.pump().await {
            return row(3, "activate 50 plugins + 50 extensions of one plugin", "fail", json!({ "error": error }), json!({ "activeActors": 100, "shards": shard_count }), "ShardLoop::pump failed");
        }
        let outcomes = env.drain();
        let faults = unexpected_faults(&outcomes, &activated, &selected).len();
        let active = env.kernel().metrics().await.actors;
        let mut per_shard: HashMap<u16, u32> = HashMap::new();
        for actor in &activated {
            if let Some(record) = env.kernel().actor_record(*actor).await {
                *per_shard.entry(record.shard.0).or_insert(0) += 1;
            }
        }
        let shards_used = env.kernel().metrics().await.shards;
        let max_shard_load = per_shard.values().copied().max().unwrap_or(0);
        let ceiling = ((activated.len() as f64) / (shard_count.max(1) as f64)).ceil() as u32 + 1;
        let pass = active as usize == 100 && activated.len() == 100 && faults == 0 && shards_used == shard_count as u32 && max_shard_load <= ceiling;
        row(
            3,
            "activate 50 plugins + 50 extensions of one plugin",
            if pass { "pass" } else { "fail" },
            json!({ "activatedCount": activated.len(), "activeActors": active, "shardsConfigured": shard_count, "shardsReported": shards_used, "maxShardLoad": max_shard_load, "shardCeiling": ceiling, "perShardCounts": per_shard, "faultCount": faults }),
            json!({ "activeActors": 100, "shards": shard_count, "maxShardLoadCeiling": "ceil(100/K)+1" }),
            "shard assignment measured via the real Kernel::activate/ShardTable pin — single physical ShardLoop backs all K shard labels for execution",
        )
    }
    //#endregion 🔖️Budget3Activate100

    //#region 🔖️Budget4And5FullScale
    /// 🏋️ Budgets 4 (memory) and 5 (interactive p95 under 40-cpu-actor load) share one fully-activated
    /// registry ("the" 50x50 scale claim) so budget 5 measures real contention against the same live
    /// fleet budget 4 just measured RSS for, instead of paying for a second 2550-instance activation.
    async fn budget_4_and_5(runtime: &Arc<GuestRuntimes>, compiled: &CompiledHandle, records: &[RegistryRecord], shard_count: u16, memory_budget_bytes: u64) -> (serde_json::Value, serde_json::Value) {
        let mut env = Env::new(runtime.clone(), shard_count).await;
        let mut activated: Vec<(ActorId, String)> = Vec::with_capacity(records.len());
        let mut instance_id = 1u32;
        for record in records {
            match env.activate(compiled, record).await {
                Ok(actor) => {
                    env.send(actor, &instance_open_event(record, instance_id)).await;
                    instance_id += 1;
                    activated.push((actor, profile_of(record).to_string()));
                }
                Err(error) => {
                    let fail = row(4, "memory <= K x 512MiB + 256MiB headroom (native RSS <= 1.5GiB)", "fail", json!({ "error": error }), json!({ "maxBytes": memory_budget_bytes }), "activate/instantiate failed mid full-scale run");
                    return (fail, skipped(5, "interactive p95 command->patch <= 16ms web / <= 8ms native, 40 cpu actors saturating background", "budget 4's full-scale activation failed before this could run"));
                }
            }
        }
        if let Err(error) = env.pump().await {
            let fail = row(4, "memory <= K x 512MiB + 256MiB headroom (native RSS <= 1.5GiB)", "fail", json!({ "error": error }), json!({ "maxBytes": memory_budget_bytes }), "ShardLoop::pump failed");
            return (fail, skipped(5, "interactive p95 command->patch <= 16ms web / <= 8ms native, 40 cpu actors saturating background", "budget 4's full-scale activation failed before this could run"));
        }
        let outcomes = env.drain();
        let by_design: std::collections::HashSet<u64> = activated.iter().zip(records.iter()).filter(|(_, record)| matches!(profile_of(record), "hang" | "crash")).map(|((actor, _), _)| actor.0).collect();
        let faults = outcomes.iter().filter(|o| matches!(o, ShardOutcome::Fault { actor, .. } if !by_design.contains(actor))).count();
        let rss = process_rss_bytes().await;
        let active = env.kernel().metrics().await.actors;
        let pass4 = faults == 0 && active as usize == activated.len() && rss.map(|bytes| bytes <= memory_budget_bytes).unwrap_or(false);
        let row4 = row(
            4,
            "memory <= K x 512MiB + 256MiB headroom (native RSS <= 1.5GiB)",
            if rss.is_none() {
                "skipped"
            } else if pass4 {
                "pass"
            } else {
                "fail"
            },
            json!({ "rssBytes": rss, "activatedCount": activated.len(), "activeActors": active, "faultCount": faults }),
            json!({ "maxBytes": memory_budget_bytes }),
            if rss.is_none() {
                "the owned platform resident-memory probe did not return a value on this host"
            } else {
                "RSS sampled once through the renderer WorkerPool I/O lane immediately after all 2550 records were instantiated and given their InstanceOpen turn"
            },
        );

        // Budget 5 — reuse the live fleet: 40 cpu-profile actors + 1 idle-profile "interactive" actor.
        let cpu_actors: Vec<ActorId> = activated.iter().filter(|(_, profile)| profile == "cpu").take(40).map(|(actor, _)| *actor).collect();
        let interactive_actor = activated.iter().find(|(_, profile)| profile == "idle").map(|(actor, _)| *actor);
        let row5 = match interactive_actor {
            None => skipped(5, "interactive p95 command->patch <= 16ms web / <= 8ms native, 40 cpu actors saturating background", "no idle-profile record to use as the interactive target"),
            Some(_interactive_actor) if cpu_actors.len() < 40 => {
                skipped(5, "interactive p95 command->patch <= 16ms web / <= 8ms native, 40 cpu actors saturating background", &format!("only {} cpu-profile actors in registry, need 40", cpu_actors.len()))
            }
            Some(interactive_actor) => {
                const ROUNDS: usize = 30;
                const NATIVE_BUDGET_MS: f64 = 8.0;
                let mut samples_ms: Vec<f64> = Vec::with_capacity(ROUNDS);
                let mut round_faults = 0usize;
                for _ in 0..ROUNDS {
                    // 🎯️ terra-bench-instrument: the 40 cpu-actor `Wake`s are submitted BEFORE the
                    // clock starts, on `Lane::Background` (`env.send`, unchanged) — they still get
                    // GRANTED in the SAME `Kernel::tick` as the interactive command just below
                    // (`grants_per_tick` comfortably covers 41 single-turn grants), so they are
                    // genuinely running/contending on their own real `ShardExecutor` threads for the
                    // WHOLE measured interval below, which is exactly the "40 cpu actors saturating
                    // the background" load this budget names. They are just not what stops the clock.
                    for actor in &cpu_actors {
                        env.send(*actor, &Event::Wake).await;
                    }
                    let start = Instant::now();
                    // 🎯️ terra-bench-instrument: the one envelope in this bench that carries
                    // `Lane::Interactive` (`Env::send_payload_lane`) — every other envelope this
                    // harness ever sends, including the 40 `Wake`s above, stays `Lane::Background`.
                    env.send_payload_lane(
                        interactive_actor,
                        Payload::Event {
                            bytes: serde_json::to_vec(&Event::CommandIngressPage {
                                cursor: semio_framework::kernel::CommandPageCursor {
                                    owner: 1,
                                    generation: 1,
                                    command_index: 0,
                                    command_count: 1,
                                    instance: interactive_actor.0 as u32,
                                    seq: 0,
                                    kind: 0,
                                    page_index: 0,
                                    page_count: 1,
                                    item_count: 0,
                                    metadata: 0,
                                },
                                bytes: semio_framework::kernel::FixedCommandPage::try_copy_from(&[0]).expect("benchmark command page is fixed-authority"),
                            })
                            .unwrap_or_default(),
                        },
                        Lane::Interactive,
                    )
                    .await;
                    // 🎯️ terra-bench-instrument (THE measurement fix): the interval this bench
                    // records is send -> `interactive_actor`'s OWN `ShardOutcome` being observed,
                    // via `Env::pump_tracking`'s `Instant` stamp — NOT the moment `pump_tracking`
                    // itself returns. `pump_tracking` still drives every actor granted this round
                    // (the 40 cpu actors included) all the way to `Kernel::complete`, exactly like
                    // `pump()` does elsewhere in this file, so kernel bookkeeping stays correct for
                    // the next round; those other 40 completions may land AFTER the stamp below and
                    // are deliberately excluded from `samples_ms`. Before this fix, the interval was
                    // `start.elapsed()` taken AFTER `pump()` (bulk-waits for ALL 41 outcomes) had
                    // already returned — i.e. it timed the slowest of 41 actors every round, not this
                    // one actor's own response; see this packet's own report for why that made the
                    // 8ms budget unreachable by construction, independent of scheduler quality.
                    match env.pump_tracking(interactive_actor).await {
                        Ok(Some(seen_at)) => samples_ms.push((seen_at - start).as_secs_f64() * 1000.0),
                        Ok(None) => round_faults += 1,
                        Err(_) => round_faults += 1,
                    }
                    let outcomes = env.drain();
                    if outcomes.iter().any(|o| matches!(o, ShardOutcome::Fault { actor, .. } if *actor == interactive_actor.0)) {
                        round_faults += 1;
                    }
                }
                samples_ms.sort_by(|a, b| a.partial_cmp(b).unwrap());
                let p95 = samples_ms.get(((samples_ms.len() as f64) * 0.95).floor() as usize).copied().unwrap_or(f64::NAN);
                let pass = round_faults == 0 && p95 <= NATIVE_BUDGET_MS;
                row(
                    5,
                    "interactive p95 command->patch <= 16ms web / <= 8ms native, 40 cpu actors saturating background",
                    if pass { "pass" } else { "fail" },
                    json!({ "p95Ms": p95, "rounds": ROUNDS, "roundFaults": round_faults, "samplesMs": samples_ms }),
                    json!({ "nativeMs": NATIVE_BUDGET_MS }),
                    "terra-bench-instrument: measured from the interactive command's own submit to THIS actor's own ShardOutcome (Turn or Fault) being observed on its real ShardExecutor thread (Env::pump_tracking), NOT to global quiescence of all 41 actors granted in the round -- the 40 cpu actors keep running/completing in the background across the measured interval, which is the load this budget specifies, they just no longer gate the clock. The interactive envelope also now carries Lane::Interactive (Env::send_payload_lane); every other envelope in this bench, including the 40 cpu Wakes, stays Lane::Background as before. NOT comparable to any p95 recorded before this fix: those measured full-round wall time across all 41 actors, not this actor's own response.",
                )
            }
        };
        (row4, row5)
    }
    //#endregion 🔖️Budget4And5FullScale

    //#region 🔖️Budget6Hang
    async fn budget_6_hang(runtime: &Arc<GuestRuntimes>, compiled: &CompiledHandle, records: &[RegistryRecord]) -> serde_json::Value {
        let Some(hang_record) = records.iter().find(|r| profile_of(r) == "hang") else {
            return skipped(6, "hang actor killed within 2x budget, shard rebuilt, siblings restored, total pause <= 250ms", "no hang-profile record in registry");
        };
        let sibling_records: Vec<&RegistryRecord> = records.iter().filter(|r| profile_of(r) == "idle").take(3).collect();
        if sibling_records.is_empty() {
            return skipped(6, "hang actor killed within 2x budget, shard rebuilt, siblings restored, total pause <= 250ms", "no idle-profile sibling records in registry");
        }
        let mut env = Env::new(runtime.clone(), 1).await;
        let deadline_ms = hang_record.quotas.deadline_ms;
        let pause_start = Instant::now();
        let hang_actor = match env.activate(compiled, hang_record).await {
            Ok(actor) => actor,
            Err(error) => return row(6, "hang actor killed within 2x budget, shard rebuilt, siblings restored, total pause <= 250ms", "fail", json!({ "error": error }), json!(null), "hang actor activate/instantiate failed"),
        };
        env.send(hang_actor, &instance_open_event(hang_record, 1)).await;
        let mut siblings = Vec::new();
        for (index, record) in sibling_records.iter().enumerate() {
            match env.activate(compiled, record).await {
                Ok(actor) => {
                    env.send(actor, &instance_open_event(record, index as u32 + 2)).await;
                    siblings.push(actor);
                }
                Err(error) => return row(6, "hang actor killed within 2x budget, shard rebuilt, siblings restored, total pause <= 250ms", "fail", json!({ "error": error }), json!(null), "sibling activate/instantiate failed"),
            }
        }
        if env.pump().await.is_err() {
            return row(6, "hang actor killed within 2x budget, shard rebuilt, siblings restored, total pause <= 250ms", "fail", json!(null), json!(null), "ShardLoop::pump failed on InstanceOpen phase");
        }
        // 🐛️ `🎭️profile::turn()` runs unconditionally on EVERY `poll`, including `InstanceOpen` (see
        // `guest::FixtureGuest::poll` in this crate's `🦀️.rs` — it always calls
        // `on_instance_open` THEN `profile::turn`) — the hang profile's overrun busy-loop, and the
        // epoch-interrupt trap it draws, is therefore typically already hit on THIS first turn, not a
        // dedicated follow-up `Wake`. A wasmtime component instance is permanently poisoned after any
        // trap (cannot be re-entered), so a second call into an already-trapped instance correctly
        // fails with "cannot enter component instance" — that message is CONFIRMING evidence of an
        // earlier kill, not a different failure. Checked here first; falls back to an explicit `Wake`
        // only if the InstanceOpen turn happened not to trigger it.
        let open_outcomes = env.drain();
        let hang_fault_on_open = open_outcomes.iter().find_map(|o| match o {
            ShardOutcome::Fault { actor, message } if *actor == hang_actor.0 => Some(message.clone()),
            _ => None,
        });
        let killed_on_open = hang_fault_on_open.is_some();
        let (killed, hang_fault) = if let Some(message) = hang_fault_on_open {
            (true, Some(message))
        } else {
            env.send(hang_actor, &Event::Wake).await;
            let _ = env.pump().await;
            let wake_outcomes = env.drain();
            let message = wake_outcomes.iter().find_map(|o| match o {
                ShardOutcome::Fault { actor, message } if *actor == hang_actor.0 => Some(message.clone()),
                _ => None,
            });
            let killed = message
                .as_deref()
                .map(|m| {
                    let lower = m.to_ascii_lowercase();
                    lower.contains("deadline") || lower.contains("fuel") || lower.contains("cannot enter")
                })
                .unwrap_or(false);
            (killed, message)
        };
        env.unregister(hang_actor).await;
        for actor in &siblings {
            env.send(*actor, &Event::Wake).await;
        }
        let siblings_pumped = env.pump().await.is_ok();
        let sibling_outcomes = env.drain();
        let siblings_ok = siblings.iter().all(|actor| sibling_outcomes.iter().any(|o| matches!(o, ShardOutcome::Turn { actor: a, .. } if *a == actor.0)));
        let pause_ms = pause_start.elapsed().as_millis() as u64;
        let pass = killed && siblings_pumped && siblings_ok && pause_ms <= 250;
        row(
            6,
            "hang actor killed within 2x budget, shard rebuilt, siblings restored, total pause <= 250ms",
            if pass { "pass" } else { "fail" },
            json!({ "declaredDeadlineMs": deadline_ms, "faultMessage": hang_fault, "killed": killed, "killedOnInstanceOpenTurn": killed_on_open, "siblingCount": siblings.len(), "siblingsRestored": siblings_ok, "totalPauseMs": pause_ms }),
            json!({ "killWithinMs": 2 * deadline_ms, "totalPauseMs": 250 }),
            "\"shard rebuilt\" is approximated as unregister+drop of the faulted GuestInstance on the same physical ShardLoop, then a successful next turn for its siblings — no separate OS thread is torn down/recreated in this single-shard-loop harness. Pause is measured from activation, since the hang overrun typically fires on the InstanceOpen turn itself (see note above), not a dedicated follow-up turn.",
        )
    }
    //#endregion 🔖️Budget6Hang

    //#region 🔖️Budget7Stateful
    /// 📸️ K1 landed mid-session (design-workforce.md's own blocker note is now stale): `ShardLoop::
    /// pump` genuinely dispatches `Payload::Suspend{checkpoint:true}` -> `GuestRuntime::checkpoint` ->
    /// `ShardOutcome::Checkpoint` and `Payload::Resume{checkpoint:Some(state)}` -> `GuestRuntime::
    /// restore` -> `ShardOutcome::Resumed`. This measures THAT real dispatch path, not a direct
    /// bypass call — suspend actor A (captures checkpoint bytes), drop A's instance (the "evicted"
    /// half of LRU-suspend), resume a FRESH instance B from those bytes (the "resumed elsewhere"
    /// half), then re-checkpoint B and compare bytes to the original. The LRU eviction TRIGGER itself
    /// (the policy deciding WHEN to suspend) is still not exercised — only the suspend/resume/
    /// checkpoint wire path K1 unblocked.
    const BUDGET_7_DESCRIPTION: &str = "stateful actor LRU-suspended and resumed -> identical state hash";

    async fn budget_7_stateful(runtime: &Arc<GuestRuntimes>, compiled: &CompiledHandle, records: &[RegistryRecord]) -> serde_json::Value {
        let Some(record) = records.iter().find(|r| profile_of(r) == "stateful") else {
            return skipped(7, BUDGET_7_DESCRIPTION, "no stateful-profile record in registry");
        };
        let mut env = Env::new(runtime.clone(), 1).await;
        let actor_a = match env.activate(compiled, record).await {
            Ok(actor) => actor,
            Err(error) => return row(7, BUDGET_7_DESCRIPTION, "fail", json!({ "error": error }), json!(null), "activate/instantiate failed"),
        };
        env.send(actor_a, &instance_open_event(record, 1)).await;
        for _ in 0..5 {
            env.send(actor_a, &Event::Wake).await;
        }
        if env.pump().await.is_err() {
            return row(7, BUDGET_7_DESCRIPTION, "fail", json!(null), json!(null), "pump failed while accumulating state");
        }
        env.drain();

        let operation = JobOperation { operation: actor_a.0, base_revision: 0, generation: actor_a.generation() as u64, preview_sequence: 0, seed: actor_a.0 };
        env.send_payload(actor_a, Payload::Suspend { operation, applied_progress: 0 }).await;
        if env.pump().await.is_err() {
            return row(7, BUDGET_7_DESCRIPTION, "fail", json!(null), json!(null), "pump failed on Suspend");
        }
        let suspend_outcomes = env.drain();
        let Some(state) = suspend_outcomes.iter().find_map(|o| match o {
            ShardOutcome::Checkpoint { actor, checkpoint, .. } if *actor == actor_a.0 => Some(checkpoint.state.clone()),
            _ => None,
        }) else {
            return row(7, BUDGET_7_DESCRIPTION, "fail", json!({ "outcomes": format!("{suspend_outcomes:?}") }), json!(null), "no ShardOutcome::Checkpoint for Suspend");
        };

        // The "evicted" half of LRU-suspend: drop A's live instance from this shard.
        env.unregister(actor_a).await;

        // The "resumed elsewhere" half: a FRESH instance, resumed from the captured checkpoint bytes.
        let actor_b = match env.activate(compiled, record).await {
            Ok(actor) => actor,
            Err(error) => return row(7, BUDGET_7_DESCRIPTION, "fail", json!({ "error": error }), json!(null), "re-activate/instantiate failed"),
        };
        env.send_payload(actor_b, Payload::Resume { operation, checkpoint: JobCheckpoint { state: state.clone(), applied_progress: 0 } }).await;
        if env.pump().await.is_err() {
            return row(7, BUDGET_7_DESCRIPTION, "fail", json!(null), json!(null), "pump failed on Resume");
        }
        let resume_outcomes = env.drain();
        let resumed = resume_outcomes.iter().any(|o| matches!(o, ShardOutcome::Resumed { actor, .. } if *actor == actor_b.0));

        env.send_payload(actor_b, Payload::Suspend { operation, applied_progress: 0 }).await;
        if env.pump().await.is_err() {
            return row(7, BUDGET_7_DESCRIPTION, "fail", json!({ "resumed": resumed }), json!(null), "pump failed on post-resume re-Suspend");
        }
        let recheck_outcomes = env.drain();
        let Some(state_after_resume) = recheck_outcomes.iter().find_map(|o| match o {
            ShardOutcome::Checkpoint { actor, checkpoint, .. } if *actor == actor_b.0 => Some(checkpoint.state.clone()),
            _ => None,
        }) else {
            return row(7, BUDGET_7_DESCRIPTION, "fail", json!({ "resumed": resumed }), json!(null), "no ShardOutcome::Checkpoint after resume");
        };
        let identical = state == state_after_resume;
        let pass = resumed && identical;
        row(
            7,
            BUDGET_7_DESCRIPTION,
            if pass { "pass" } else { "fail" },
            json!({ "resumed": resumed, "checkpointHash": semio_framework_hash::hash(&state).to_hex().to_string(), "resumedCheckpointHash": semio_framework_hash::hash(&state_after_resume).to_hex().to_string(), "identical": identical }),
            json!("Resumed outcome received and identical checkpoint bytes before suspend vs. after resume+re-checkpoint"),
            "measured through the REAL production dispatch path (K1, unblocked mid-session): ShardLoop::pump's Payload::Suspend/Resume -> GuestRuntime::checkpoint/restore -> ShardOutcome::Checkpoint/Resumed. The LRU-eviction TRIGGER (the policy deciding WHEN to suspend) is still not exercised here — this proves the suspend/resume/checkpoint wire path end-to-end, which is exactly what was blocked before K1 landed.",
        )
    }
    //#endregion 🔖️Budget7Stateful

    //#region 🔖️Budget8CapabilityRevoke
    async fn budget_8_capability_revoke(runtime: &Arc<GuestRuntimes>, compiled: &CompiledHandle, records: &[RegistryRecord]) -> serde_json::Value {
        let Some(record) = records.iter().find(|r| profile_of(r) == "io") else {
            return skipped(8, "capability revoked at runtime -> denied completion, actor stays alive, quota counters zero", "no io-profile record in registry");
        };
        let cap_id = record.scale_fixture.get("ioCapabilityId").and_then(|v| v.as_str()).unwrap_or("scale-fixture.io").to_string();
        let budget = turn_budget_of(record);
        let actor = ActorId(0xB8_0000_0001);
        let mut inst = match runtime.instantiate(compiled, actor, &[], &budget).await {
            Ok(instance) => instance,
            Err(error) => return row(8, "capability revoked at runtime -> denied completion, actor stays alive, quota counters zero", "fail", json!({ "error": error.to_string() }), json!(null), "instantiate failed"),
        };
        // 🐛️ `🎭️profile::turn()` runs unconditionally on EVERY `poll` (see budget 6's identical note) —
        // the `io` profile's ONE-TIME `RequestCapability` effect is therefore typically emitted on
        // THIS very first `InstanceOpen` turn, not a dedicated follow-up. Checked on both turns so a
        // real request is never misread as absent just because it landed on turn 1.
        let open_result = match runtime.execute_turn(&mut inst, &[instance_open_event(record, 1)], budget).await {
            Ok(result) => result,
            Err(fault) => return row(8, "capability revoked at runtime -> denied completion, actor stays alive, quota counters zero", "fail", json!({ "error": fault.to_string() }), json!(null), "InstanceOpen turn failed"),
        };
        let requested_on_open = open_result.effects.iter().any(|effect| matches!(effect, Effect::RequestCapability { capability, .. } if capability.id.0 == cap_id));
        let requested_on_wake = match runtime.execute_turn(&mut inst, &[Event::Wake], budget).await {
            Ok(result) => result.effects.iter().any(|effect| matches!(effect, Effect::RequestCapability { capability, .. } if capability.id.0 == cap_id)),
            Err(fault) => return row(8, "capability revoked at runtime -> denied completion, actor stays alive, quota counters zero", "fail", json!({ "error": fault.to_string() }), json!(null), "capability-request turn failed"),
        };
        let requested = requested_on_open || requested_on_wake;
        let revoke_event = Event::CapabilityChanged { change: CapabilityChange::Revoked { id: CapabilityId(cap_id.clone()) } };
        let revoke_result = runtime.execute_turn(&mut inst, &[revoke_event], budget).await;
        let survived_revoke = revoke_result.is_ok();
        let revoke_status = match &revoke_result {
            Ok(result) => format!("{:?}", result.status),
            Err(fault) => fault.to_string(),
        };
        let followup = runtime.execute_turn(&mut inst, &[Event::Wake], budget).await;
        let survived_followup = followup.is_ok();
        let pass = requested && survived_revoke && survived_followup;
        row(
            8,
            "capability revoked at runtime -> denied completion, actor stays alive, quota counters zero",
            if pass { "pass" } else { "fail" },
            json!({ "capabilityId": cap_id, "capabilityRequested": requested, "requestedOnInstanceOpenTurn": requested_on_open, "survivedRevokeTurn": survived_revoke, "statusAfterRevoke": revoke_status, "survivedFollowupTurn": survived_followup }),
            json!("no trap across or after the revoke turn"),
            "\"quota counters zero\" is read here as \"no TurnFault (fuel/deadline/trap) recorded across the revoke turn\": Kernel::complete() (the only path that updates Kernel-level ActorMetrics/ActorStatus) is never called by this harness — same documented gap as the production kernel_runtime module above — so the kernel's own quota counters cannot be read from here.",
        )
    }
    //#endregion 🔖️Budget8CapabilityRevoke

    /// ▶️ `--scale <registry.json> --scale-wasm <fixture.wasm> --shards <K> --report <out.json>`.
    /// Runs budgets 2-8 (budget 1 — registry parse timing — is measured JS-side, no wasm involved) and
    /// writes one JSON report. Returns `0` on a clean harness run (regardless of individual budget
    /// pass/fail — a real measured FAIL is a valid, non-error outcome), `1` if the harness itself could
    /// not set up (bad registry/wasm/report path).
    pub async fn run(registry_path: PathBuf, wasm_path: PathBuf, shard_count: u16, report_path: PathBuf) -> i32 {
        let process_start = Instant::now();
        let mut registry_bytes = match crate::run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadBytes(registry_path.clone())).await {
            Ok(semio_framework_os_services::NativeIoValue::Bytes(bytes)) => bytes,
            Ok(_) => {
                eprintln!("scale-bench: native I/O returned the wrong value for {}", registry_path.display());
                return 1;
            }
            Err(error) => {
                eprintln!("scale-bench: failed to read {}: {error}", registry_path.display());
                return 1;
            }
        };
        let Some(registry_page) = registry_bytes.single_page() else {
            eprintln!("scale-bench: registry exceeds the mounted single-page retained parser authority");
            return 1;
        };
        let registry = serde_json::from_slice(registry_page);
        let _ = registry_bytes.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        let registry: RegistryFile = match registry {
            Ok(value) => value,
            Err(error) => {
                eprintln!("scale-bench: failed to parse {}: {error}", registry_path.display());
                return 1;
            }
        };
        let mut wasm_bytes_owner = match crate::run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadBytes(wasm_path.clone())).await {
            Ok(semio_framework_os_services::NativeIoValue::Bytes(bytes)) => bytes,
            Ok(_) => {
                eprintln!("scale-bench: native I/O returned the wrong value for {}", wasm_path.display());
                return 1;
            }
            Err(error) => {
                eprintln!("scale-bench: failed to read {}: {error}", wasm_path.display());
                return 1;
            }
        };
        let Some(wasm_bytes) = wasm_bytes_owner.single_page() else {
            eprintln!("scale-bench: Wasm exceeds the mounted single-page retained compiler authority");
            return 1;
        };
        let runtime: Arc<GuestRuntimes> = Arc::new(GuestRuntimes::Owned(OwnedRuntime::new()));
        let package_ref = PackageRef { package: PackageId("scale-fixture".to_string()), hash: PackageHash(*semio_framework_hash::hash(wasm_bytes).as_bytes()) };
        let compiled = runtime.compile(&package_ref, wasm_bytes).await;
        let _ = wasm_bytes_owner.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        let compiled = match compiled {
            Ok(handle) => handle,
            Err(error) => {
                eprintln!("scale-bench: compile failed: {error}");
                return 1;
            }
        };

        let row_2 = budget_2_cold_boot(process_start, &runtime, &compiled, &registry.records, shard_count, 1500).await;
        let row_3 = budget_3_activate_100(&runtime, &compiled, &registry.records, shard_count).await;
        let (row_4, row_5) = budget_4_and_5(&runtime, &compiled, &registry.records, shard_count, shard_count as u64 * 512 * 1024 * 1024 + 256 * 1024 * 1024).await;
        let row_6 = budget_6_hang(&runtime, &compiled, &registry.records).await;
        let row_7 = budget_7_stateful(&runtime, &compiled, &registry.records).await;
        let row_8 = budget_8_capability_revoke(&runtime, &compiled, &registry.records).await;

        drop((row_2, row_3, row_4, row_5, row_6, row_7, row_8));
        eprintln!("scale-bench: refusing to write populated report {} until its retained page encoder is mounted (renderer=native, shards={shard_count}, records={}, wasm={})", report_path.display(), registry.records.len(), wasm_path.display());
        1
    }
}
//#endregion 🔖️ScaleBench

#[cfg(not(target_arch = "wasm32"))]
fn spawn_app_task<F>(future: F)
where
    F: std::future::Future<Output = ()> + Send + 'static,
{
    let _ = kernel_runtime::KernelPoolFuture::spawn(renderer_worker_pool(), semio_framework_async::Lane::Interactive, future);
}

#[cfg(target_arch = "wasm32")]
fn spawn_app_task<F>(future: F)
where
    F: std::future::Future<Output = ()> + 'static,
{
    spawn_local(future);
}

#[cfg(target_arch = "wasm32")]
fn log_debug(message: &str) {
    web_sys::console::log_1(&JsValue::from_str(message));
}

#[cfg(not(target_arch = "wasm32"))]
fn log_debug(message: &str) {
    eprintln!("{message}");
}

#[cfg(target_arch = "wasm32")]
fn prefers_dark_scheme() -> bool {
    web_sys::window().and_then(|window| window.match_media("(prefers-color-scheme: dark)").ok().flatten()).map(|query| query.matches()).unwrap_or(true)
}

#[cfg(not(target_arch = "wasm32"))]
fn prefers_dark_scheme() -> bool {
    true
}

fn resolve_theme(appearance_id: &str) -> Theme {
    match appearance_id {
        "light" => Theme::light(),
        "dark" => Theme::dark(),
        _ if prefers_dark_scheme() => Theme::dark(),
        _ => Theme::light(),
    }
}

fn appearance_is_dark(appearance_id: &str) -> bool {
    match appearance_id {
        "light" => false,
        "dark" => true,
        _ => prefers_dark_scheme(),
    }
}

#[cfg(target_arch = "wasm32")]
pub(crate) fn app_now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn app_now_ms() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_secs_f64() * 1000.0).unwrap_or(0.0)
}

//#region 🔖️AsyncBoundaryTests
#[cfg(test)]
mod async_boundary_tests {
    use super::*;

    const LIBRARY_SOURCE: &str = include_str!("🦀️.rs");
    const BINARY_SOURCE: &str = include_str!("../⌨️native-entrypoint/🦀️.rs");
    const MANIFEST_SOURCE: &str = include_str!("../📦️packages/🦀️rust/Cargo.toml");
    const WINT_APP_SOURCE: &str = include_str!("../🪟️winit-app/🦀️.rs");
    const OS_HOST_SOURCE: &str = include_str!("../🏠️os-host/🦀️.rs");
    const GPU_SOURCE: &str = include_str!("../../../../../../../../🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️gpu.rs");
    const DRAW_SOURCE: &str = include_str!("../../../../../../../../🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️draw.rs");
    const PREPARED_SOURCE: &str = include_str!("../../../../../../../../🔨️modules/🖱️ui/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/🦀️prepared.rs");
    const ENGINE_CANVAS_SOURCE: &str = include_str!("../../../🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs");

    #[test]
    fn product_library_has_no_executor_bridge() {
        assert!(!LIBRARY_SOURCE.contains(concat!("poll", "ster")));
        assert!(!LIBRARY_SOURCE.contains(concat!("block", "_on")));
        assert!(LIBRARY_SOURCE.contains("KernelPoolFuture::spawn"));
        assert!(LIBRARY_SOURCE.contains("spawn_app_task"));
        assert!(!LIBRARY_SOURCE.contains(concat!("TASK", "_POOL")));
        assert!(!LIBRARY_SOURCE.contains(concat!("poll", "_tasks")));
        assert!(!LIBRARY_SOURCE.contains("thread_local! {\n        static REAL_WAKER"));
        assert!(!WINT_APP_SOURCE.contains(concat!("poll", "_tasks")));
        assert!(!WINT_APP_SOURCE.contains(concat!("TASK", "_POOL")));
        assert!(!LIBRARY_SOURCE.contains("type RuntimeApply = Box"));
        assert!(!LIBRARY_SOURCE.contains(concat!("mem::", "forget")));
        assert!(!WINT_APP_SOURCE.contains("dispatch_drained_events"));
        assert!(LIBRARY_SOURCE.contains("struct RuntimeDispatchCursor"));
        assert!(LIBRARY_SOURCE.contains("ResumeDispatch"));
        assert!(!LIBRARY_SOURCE.contains("response.array_buffer()"));
        assert!(!LIBRARY_SOURCE.contains("collect_pending_ui_image_fetches"));
        assert!(!LIBRARY_SOURCE.contains("collect_pending_map_tile_fetches"));
        assert!(LIBRARY_SOURCE.contains("struct AppPresentCursor"));
        assert!(!LIBRARY_SOURCE.contains(".submit_prepared("));
        assert!(!GPU_SOURCE.contains("fn apply_prepared_uploads"));
        assert!(!DRAW_SOURCE.contains("for index in 0..schema.vertices"));
        assert!(!DRAW_SOURCE.contains("for index in 0..schema.indices"));
        assert!(DRAW_SOURCE.contains("pub fn ensure_mesh_step"));
        assert!(DRAW_SOURCE.contains("pub fn close_upload_step"));
        assert!(GPU_SOURCE.contains("pub fn close_mesh_upload_step"));
        let upload_close = OS_HOST_SOURCE.find("presenter.close_world_owners_step()").expect("active upload and packet close phase");
        let world_close = OS_HOST_SOURCE.find("runtime.close_world3d_dynamic_step()").expect("world mesh close phase");
        assert!(upload_close < world_close);
    }

    fn retained_raster_contract(draw: &str, gpu: &str, glue: &str, engine: &str) -> bool {
        fn guarded_allocations(source: &str, validation: &str, allocation: &str, expected: usize) -> bool {
            let validations = source.match_indices(validation).map(|(index, _)| index).collect::<Vec<_>>();
            if validations.len() != expected {
                return false;
            }
            let mut allocation_floor = 0;
            for validation_index in validations {
                let Some(relative_allocation) = source[validation_index + validation.len()..].find(allocation) else {
                    return false;
                };
                let allocation_index = validation_index + validation.len() + relative_allocation;
                if allocation_index < allocation_floor {
                    return false;
                }
                let gap = &source[validation_index + validation.len()..allocation_index];
                if ["device.create_texture", ".create_view", "device.create_bind_group", "create_target_texture", "Renderer::new"].iter().any(|marker| gap.contains(marker)) {
                    return false;
                }
                allocation_floor = allocation_index + allocation.len();
            }
            true
        }

        let begin = gpu.find("self.raster_store.begin_presenting").unwrap_or(usize::MAX);
        let present_step = gpu.find("pub fn prepared_present_step").unwrap_or(0);
        let presenter_close = glue.find("self.gpu.close_raster_table_step()").unwrap_or(usize::MAX);
        let world_terminal = glue.find("self.gpu.raster_table_terminal_is_empty()").unwrap_or(0);
        let engine_reservation = "let admission = gpu.reserve_engine_texture(&key, width, height, candidate, expected)?;";
        let engine_reservation_index = engine.find(engine_reservation).unwrap_or(usize::MAX);
        let first_engine_allocation = ["create_target_texture", ".create_view", "Renderer::new"].iter().filter_map(|marker| engine.find(marker)).min().unwrap_or(0);
        let upload_stage = &draw[draw.find("pub fn ensure_raster_step").unwrap_or(draw.len())..draw.find("pub fn get(&self, key: &str) -> Option<&RasterTexture>").unwrap_or(draw.len())];
        let gpu_stage = &draw[draw.find("pub fn stage_gpu_bind_group").unwrap_or(draw.len())..draw.find("pub fn begin_presenting").unwrap_or(draw.len())];
        let cancellation = &draw[draw.find("pub fn cancel_engine_texture_admission").unwrap_or(draw.len())..draw.find("fn claim_stage_before_gpu_allocation").unwrap_or(draw.len())];
        let upload_close = &draw[draw.find("pub fn close_upload_step(&mut self) -> RasterTextureCleanupStep").unwrap_or(draw.len())..];
        let upload_close = &upload_close[..upload_close.find("pub fn close_step(&mut self)").unwrap_or(upload_close.len())];
        draw.contains("pub const RASTER_TEXTURE_TABLE_CAPACITY: usize = 256")
            && draw.contains("pub const RASTER_TEXTURE_KEY_BYTES: usize = 256")
            && draw.contains("pub const RASTER_TEXTURE_ITEM_BYTE_CAPACITY: usize = 16 * 1024 * 1024")
            && draw.contains("pub const RASTER_TEXTURE_TABLE_BYTE_CAPACITY: usize = 256 * 1024 * 1024")
            && draw.contains("const RASTER_TEXTURE_PROBE_CAPACITY: usize = 8")
            && draw.contains("const PAGE_BYTES: usize = 16 * 1024")
            && draw.contains("struct FixedRasterTextureRegistry<T>")
            && draw.contains("pub struct RasterTextureAdmission")
            && draw.contains("candidate: RasterTextureWitnessSlot")
            && draw.contains("current.operation >= candidate.operation")
            && draw.contains("admission.witness != expected")
            && draw.contains("view: Option<wgpu::TextureView>")
            && draw.contains("scene_revision: Option<u64>")
            && draw.contains("preview_generation: Option<u64>")
            && draw.contains("operation: Option<u64>")
            && draw.contains("width: Option<u32>")
            && draw.contains("height: Option<u32>")
            && draw.contains("fn insert_vacant")
            && !draw.contains("fn insert_at")
            && draw.contains("RasterTextureRetirementMode::Commit")
            && draw.contains("RasterTextureRetirementMode::Abort")
            && draw.contains("pub fn commit_presented_step")
            && draw.contains("pub fn abort_presented_step")
            && draw.contains("pub fn terminal_is_empty(&self) -> bool")
            && draw.contains("struct RasterTextureUploadCloseCursor")
            && draw.contains("struct RasterTextureReservationCloseCursor")
            && draw.contains("enum RasterTextureCleanupStep")
            && cancellation.contains("self.reservation.take().expect(\"matching raster reservation\")")
            && cancellation.contains("RasterTextureReservationCloseCursor::cancelled(reservation, admission)")
            && !cancellation.contains(concat!("self.reservation =", " None"))
            && upload_close.contains("RasterTextureUploadCloseCursor::new(upload)")
            && !upload_close.contains("self.presenting.set")
            && !upload_close.contains(concat!("self.reservation =", " None"))
            && draw.contains("self.key == admission.key")
            && draw.contains("self.witness == admission.witness")
            && draw.contains("self.width == admission.width")
            && draw.contains("self.height == admission.height")
            && draw.contains("self.bytes == admission.bytes")
            && draw.contains("self.staged_index == admission.staged_index")
            && draw.contains("self.nonce == admission.nonce")
            && draw.contains("if admission.witness != expected")
            && draw.contains("if !reservation.matches(admission)")
            && draw.contains("if candidate != Some(expected)")
            && draw.contains("if staged_occupied")
            && draw.contains("staged_index: admission.staged_index, staged_nonce: admission.nonce")
            && draw.contains("fn claim_stage_before_gpu_allocation")
            && upload_stage.contains("self.upload = Some(RasterTextureUploadCursor")
            && guarded_allocations(upload_stage, "self.claim_texture_allocation(admission, expected)", "device.create_texture", 1)
            && guarded_allocations(upload_stage, "self.claim_view_allocation(admission, expected)", ".create_view", 1)
            && guarded_allocations(upload_stage, "self.claim_bind_group_allocation(admission, expected)", "device.create_bind_group", 1)
            && guarded_allocations(gpu_stage, "self.claim_bind_group_allocation(&admission, expected)", "device.create_bind_group", 1)
            && upload_stage.contains("if let Err((fault, admission, value)) = self.stage_claimed_texture(admission, value, allocation_claim)")
            && upload_stage.contains("self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor")
            && upload_stage.contains("texture: Some(value.texture)")
            && upload_stage.contains("view: Some(value.view)")
            && upload_stage.contains("bind_group: Some(value.bind_group)")
            && upload_stage.contains("allocation_claim: Some(allocation_claim)")
            && !upload_stage.contains("map_err(|(fault, _, _)| fault)")
            && gpu_stage.matches("self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor").count() == 1
            && gpu_stage.contains("RasterTextureStageFault::Returned { fault, admission, texture, view: raster_view }")
            && gpu_stage.contains("texture: Some(value.texture)")
            && gpu_stage.contains("view: Some(value.view)")
            && gpu_stage.contains("bind_group: Some(value.bind_group)")
            && gpu_stage.contains("allocation_claim: Some(allocation_claim)")
            && !gpu_stage.contains("map_err(|(fault, _, _)| fault)")
            && draw.contains("bind_group: source.bind_group.take()")
            && draw.contains("view: source.view.take()")
            && draw.contains("texture: source.texture.take()")
            && !draw.contains("HashMap<String, RasterTexture>")
            && gpu.contains("self.ensure_raster_texture_step(key, pixels")
            && gpu.contains("pub fn stage_engine_texture")
            && gpu.contains("view: wgpu::TextureView")
            && gpu.contains(".stage_gpu_bind_group")
            && gpu.contains("Result<(), RasterTextureStageFault>")
            && begin < present_step
            && engine.matches(engine_reservation).count() == 1
            && engine_reservation_index < first_engine_allocation
            && guarded_allocations(engine, "gpu.validate_engine_target_texture_allocation(&admission, expected)", "create_target_texture", 1)
            && guarded_allocations(engine, "gpu.validate_engine_target_view_allocation(&admission, expected)", ".create_view", 1)
            && guarded_allocations(engine, "gpu.validate_engine_renderer_allocation(&admission, expected)", "Renderer::new", 1)
            && guarded_allocations(engine, "gpu.validate_engine_replacement_texture_allocation(&admission, expected)", "create_target_texture", 2)
            && guarded_allocations(engine, "gpu.validate_engine_replacement_view_allocation(&admission, expected)", ".create_view", 2)
            && engine.matches("gpu.retain_engine_allocation_fault(admission, Some(texture), None)").count() == 2
            && engine.contains("gpu.retain_engine_allocation_fault(admission, Some(texture), Some(view))")
            && engine.contains("gpu.retain_engine_allocation_fault(admission, Some(replacement_texture), None)")
            && engine.contains("RasterTextureStageFault::Returned { fault, admission, texture, view }")
            && engine.contains("surface.texture = texture")
            && engine.contains("surface.view = view")
            && engine.contains("RasterTextureStageFault::Retained(fault)")
            && engine.contains("let published_view = std::mem::replace(&mut surface.view, replacement_view)")
            && !engine.contains("surface.view.clone()")
            && glue.contains("struct RuntimeRasterOperationAuthority")
            && glue.contains("exhausted: AtomicBool")
            && glue.contains("compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire)")
            && !glue.contains(concat!("next_operation.", "fetch_add(1, Ordering::AcqRel)"))
            && glue.contains("raster_operation_authority: RuntimeRasterOperationAuthority")
            && glue.contains("raster_operation_authority.begin(expected.scene_revision, expected.input_generation)")
            && glue.contains("raster_operation_authority.matches(raster_witness)")
            && glue.contains("RasterCandidateRetirement::Commit")
            && glue.contains("RasterCandidateRetirement::Abort")
            && presenter_close < world_terminal
    }

    #[test]
    fn raster_upload_cache_is_fixed_generation_witnessed_and_mutation_complete() {
        assert!(retained_raster_contract(DRAW_SOURCE, GPU_SOURCE, LIBRARY_SOURCE, ENGINE_CANVAS_SOURCE));
        let mutations = [
            (DRAW_SOURCE.replace("pub const RASTER_TEXTURE_TABLE_CAPACITY: usize = 256", "pub const RASTER_TEXTURE_TABLE_CAPACITY: usize = 257"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("pub const RASTER_TEXTURE_KEY_BYTES: usize = 256", "pub const RASTER_TEXTURE_KEY_BYTES: usize = 255"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (
                DRAW_SOURCE.replace("pub const RASTER_TEXTURE_ITEM_BYTE_CAPACITY: usize = 16 * 1024 * 1024", "pub const RASTER_TEXTURE_ITEM_BYTE_CAPACITY: usize = usize::MAX"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (DRAW_SOURCE.replace("pub struct RasterTextureAdmission", "pub struct ErasedRasterTextureAdmission"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("current.operation >= candidate.operation", "current.operation > candidate.operation"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("view: Option<wgpu::TextureView>", "view_erased: bool"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.replace("view: wgpu::TextureView", "view: &wgpu::TextureView"), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.replace("gpu.reserve_engine_texture", "gpu.realize_without_reservation")),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.replace("let published_view = std::mem::replace(&mut surface.view, replacement_view)", "let published_view = surface.view.clone()")),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.replace("raster_operation_authority: RuntimeRasterOperationAuthority", "raster_operation_authority_erased: bool"), ENGINE_CANVAS_SOURCE.to_string()),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.replace(
                    "let operation = loop {",
                    concat!("let operation = self.0.next_operation.", "fetch_add(1, Ordering::AcqRel); if operation == 0 { return Err(\"raster operation generation exhausted\"); } loop {"),
                ),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace(
                    "let reservation = self.reservation.take().expect(\"matching raster reservation\");",
                    concat!("self.reservation =", " None; let reservation = RasterTextureReservation { key: admission.key, witness: admission.witness, width: admission.width, height: admission.height, bytes: admission.bytes, staged_index: admission.staged_index, nonce: admission.nonce };"),
                ),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace("self.upload_close = Some(RasterTextureUploadCloseCursor::new(upload));", "self.upload = None;"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace("fn claim_stage_before_gpu_allocation", "fn claim_stage_after_gpu_allocation"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.replace("raster_operation_authority.begin(expected.scene_revision, expected.input_generation)", "raster_operation_authority.begin(packet.scene_revision(), packet.preview_generation())"),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.replace("raster_operation_authority.matches(raster_witness)", "true"), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.replace("self.gpu.close_raster_table_step()", "true"), ENGINE_CANVAS_SOURCE.to_string()),
            (
                DRAW_SOURCE.replace("self.claim_texture_allocation(admission, expected)", "self.allocate_texture_without_full_claim(admission, expected)"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace("self.claim_view_allocation(admission, expected)", "self.allocate_view_without_full_claim(admission, expected)"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace("self.claim_bind_group_allocation(admission, expected)", "self.allocate_bind_group_without_full_claim(admission, expected)"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace("self.claim_bind_group_allocation(&admission, expected)", "self.allocate_bind_group_without_full_claim(&admission, expected)"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.replace("gpu.validate_engine_target_texture_allocation(&admission, expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.replace("gpu.validate_engine_target_view_allocation(&admission, expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.replace("gpu.validate_engine_renderer_allocation(&admission, expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.replace("gpu.validate_engine_replacement_texture_allocation(&admission, expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.replace("gpu.validate_engine_replacement_view_allocation(&admission, expected)", "Ok(())"),
            ),
            (
                DRAW_SOURCE.replace(
                    "if let Err((fault, admission, value)) = self.stage_claimed_texture(admission, value, allocation_claim)",
                    "if self.stage_claimed_texture(admission, value, allocation_claim).map_err(|(fault, _, _)| fault).is_err()",
                ),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace("self.upload_close = Some(RasterTextureUploadCloseCursor::new(RasterTextureUploadCursor", "self.upload = None; Some(RasterTextureUploadCursor"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (DRAW_SOURCE.replace("texture: Some(value.texture)", "texture: None"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("view: Some(value.view)", "view: None"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("bind_group: Some(value.bind_group)", "bind_group: None"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("if !reservation.matches(admission)", "if false"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("if candidate != Some(expected)", "if false"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (DRAW_SOURCE.replace("if staged_occupied", "if false"), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.to_string()),
            (
                DRAW_SOURCE.replace("staged_index: admission.staged_index, staged_nonce: admission.nonce", "staged_index: admission.staged_index, staged_nonce: 0"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (
                DRAW_SOURCE.replace("Returned {", "Erased {"),
                GPU_SOURCE.to_string(),
                LIBRARY_SOURCE.to_string(),
                ENGINE_CANVAS_SOURCE.to_string(),
            ),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.replace("surface.texture = texture", "drop(texture)")),
            (DRAW_SOURCE.to_string(), GPU_SOURCE.to_string(), LIBRARY_SOURCE.to_string(), ENGINE_CANVAS_SOURCE.replace("surface.view = view", "drop(view)")),
        ];
        assert_eq!(mutations.len(), 38);
        for (draw, gpu, glue, engine) in mutations {
            assert!(!retained_raster_contract(&draw, &gpu, &glue, &engine));
        }
        let reservation = "let admission = gpu.reserve_engine_texture(&key, width, height, candidate, expected)?;";
        let first_allocation = "let texture = create_target_texture(gpu.device(), width, height);";
        let reservation_after_first_allocation = ENGINE_CANVAS_SOURCE.replacen(reservation, "", 1).replacen(first_allocation, &format!("{first_allocation}\n            {reservation}"), 1);
        assert!(!retained_raster_contract(DRAW_SOURCE, GPU_SOURCE, LIBRARY_SOURCE, &reservation_after_first_allocation));
    }

    #[test]
    fn renderer_asset_probe_keeps_pages_owned_across_chunk_boundaries_and_rejects_malformed_length() {
        fn semantic_glb() -> Vec<u8> {
            let mut json = br#"{"scene":0,"scenes":[{"nodes":[0]}],"nodes":[{"mesh":0,"translation":[1,2,3]}],"accessors":[{"bufferView":0,"componentType":5126,"count":4,"type":"VEC3"},{"bufferView":1,"componentType":5120,"count":4,"type":"VEC3","normalized":true},{"bufferView":2,"componentType":5123,"count":4,"type":"VEC2","normalized":true},{"bufferView":3,"componentType":5121,"count":4,"type":"SCALAR"}],"bufferViews":[{"buffer":0,"byteOffset":0,"byteLength":64,"byteStride":16},{"buffer":0,"byteOffset":64,"byteLength":16,"byteStride":4},{"buffer":0,"byteOffset":80,"byteLength":16,"byteStride":4},{"buffer":0,"byteOffset":96,"byteLength":4}],"meshes":[{"primitives":[{"attributes":{"POSITION":0,"NORMAL":1,"TEXCOORD_0":2},"indices":3,"mode":5}]}]}"#.to_vec();
            while !json.len().is_multiple_of(4) {
                json.push(b' ');
            }
            let mut bin = Vec::with_capacity(100);
            for position in [[0.0f32, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [1.0, 1.0, 0.0]] {
                for component in position {
                    bin.extend_from_slice(&component.to_le_bytes());
                }
                bin.extend_from_slice(&[0; 4]);
            }
            for _ in 0..4 {
                bin.extend_from_slice(&[0, 0, 127, 0]);
            }
            for uv in [[0u16, 0], [u16::MAX, 0], [0, u16::MAX], [u16::MAX, u16::MAX]] {
                for component in uv {
                    bin.extend_from_slice(&component.to_le_bytes());
                }
            }
            bin.extend_from_slice(&[0, 1, 2, 3]);
            let total = 12 + 8 + json.len() + 8 + bin.len();
            let mut glb = Vec::with_capacity(total);
            glb.extend_from_slice(b"glTF");
            glb.extend_from_slice(&2u32.to_le_bytes());
            glb.extend_from_slice(&(total as u32).to_le_bytes());
            glb.extend_from_slice(&(json.len() as u32).to_le_bytes());
            glb.extend_from_slice(b"JSON");
            glb.extend_from_slice(&json);
            glb.extend_from_slice(&(bin.len() as u32).to_le_bytes());
            glb.extend_from_slice(b"BIN\0");
            glb.extend_from_slice(&bin);
            glb
        }

        fn probe(glb: &[u8], split: usize) -> (WorldAssetIoAuthority, RendererAssetProbe) {
            let mut authority = WorldAssetIoAuthority::default();
            authority.reserve(1, 1, WorldAssetRequestKind::Glb, "probe.glb", glb.len()).unwrap();
            let mut owner = authority.take_next().unwrap();
            owner.push_page(WorldAssetResponsePage::try_from_owned(glb[..split].to_vec()).unwrap()).unwrap();
            owner.push_page(WorldAssetResponsePage::try_from_owned(glb[split..].to_vec()).unwrap()).unwrap();
            owner.seal().unwrap();
            authority.return_owner(owner).unwrap();
            let owner = (0..infinite_world::world::WORLD_ASSET_REQUEST_CAPACITY).find_map(|_| authority.take_next_completed_step()).expect("completed probe owner");
            (authority, RendererAssetProbe::new(RendererAssetFetchOwner::Shared(owner)))
        }

        let valid = semantic_glb();
        let (mut authority, mut cursor) = probe(&valid, 5);
        let mut ready = false;
        for _ in 0..4_096 {
            match cursor.step() {
                RendererAssetProbeStep::Pending => {}
                RendererAssetProbeStep::Ready => {
                    ready = true;
                    break;
                }
                RendererAssetProbeStep::Fault(detail) => panic!("valid retained GLB structure faulted: {detail}"),
            }
        }
        assert!(ready);
        let materialize = cursor.glb_materialize.as_ref().expect("retained GLB materializer");
        let lease = materialize.lease.expect("generation-witnessed paged GLB mesh");
        let mut positions = Vec::new();
        let mut normals = Vec::new();
        let mut indices = Vec::new();
        let mut position_cursor = lease.cursor(Mesh3dField::Positions).unwrap();
        while let Some(Mesh3dItem::Vec3(value)) = position_cursor.next().unwrap() {
            positions.extend_from_slice(&value);
        }
        let mut normal_cursor = lease.cursor(Mesh3dField::Normals).unwrap();
        while let Some(Mesh3dItem::Vec3(value)) = normal_cursor.next().unwrap() {
            normals.extend_from_slice(&value);
        }
        let mut index_cursor = lease.cursor(Mesh3dField::Indices).unwrap();
        while let Some(Mesh3dItem::U32(value)) = index_cursor.next().unwrap() {
            indices.push(value);
        }
        let legacy = semio_framework::mesh_from_glb(&valid).expect("legacy glTF oracle");
        assert_eq!(positions, legacy.positions);
        assert_eq!(normals, legacy.normals);
        assert_eq!(indices, legacy.indices);
        cursor.begin_close();
        while !cursor.close_step() {}
        let RendererAssetFetchOwner::Shared(owner) = cursor.take_terminal_owner().unwrap() else { panic!("shared probe") };
        authority.finish(owner).unwrap();
        assert!(authority.terminal_is_empty());

        let malformed = b"glTF\x02\0\0\0\xff\0\0\0";
        let (mut authority, mut cursor) = probe(malformed, 7);
        assert!(matches!(cursor.step(), RendererAssetProbeStep::Pending));
        assert!(matches!(cursor.step(), RendererAssetProbeStep::Pending));
        assert!(matches!(cursor.step(), RendererAssetProbeStep::Fault("asset response format probe rejected malformed input")));
        while !cursor.close_step() {}
        let RendererAssetFetchOwner::Shared(owner) = cursor.take_terminal_owner().unwrap() else { panic!("shared probe") };
        authority.finish(owner).unwrap();
        assert!(authority.terminal_is_empty());
    }

    #[test]
    fn retained_asset_structure_scanners_resume_at_arbitrary_block_boundaries() {
        let png = [b"\x89PNG\r\n\x1a\n".as_slice(), &[0, 0, 0, 13], b"IHDR", &[0, 0, 0, 1, 0, 0, 0, 1, 8, 6, 0, 0, 0], &[0; 4], &[0, 0, 0, 1], b"IDAT", &[0], &[0; 4], &[0, 0, 0, 0], b"IEND", &[0; 4]].concat();
        let mut png_cursor = PngStructureCursor::new(png.len());
        for bytes in png.chunks(3) {
            png_cursor.feed(bytes).expect("bounded PNG block");
        }
        png_cursor.finish().expect("PNG terminal");

        let jpeg = [0xff, 0xd8, 0xff, 0xc0, 0, 11, 8, 0, 1, 0, 1, 1, 1, 0x11, 0, 0xff, 0xd9];
        let mut jpeg_cursor = JpegStructureCursor::new(jpeg.len());
        for bytes in jpeg.chunks(2) {
            jpeg_cursor.feed(bytes).expect("bounded JPEG block");
        }
        jpeg_cursor.finish().expect("JPEG terminal");

        let protobuf = [0x1a, 0x02, 0x08, 0x01];
        let mut protobuf_cursor = ProtobufStructureCursor::new(protobuf.len());
        for byte in protobuf {
            protobuf_cursor.feed(&[byte]).expect("bounded protobuf byte");
        }
        protobuf_cursor.finish().expect("protobuf terminal");
    }

    #[test]
    fn retained_asset_structure_scanners_reject_pixel_and_varint_capacity_plus_one() {
        let png = [b"\x89PNG\r\n\x1a\n".as_slice(), &[0, 0, 0, 13], b"IHDR", &[0, 0, 0x10, 1, 0, 0, 0x10, 0, 8, 6, 0, 0, 0]].concat();
        let mut png_cursor = PngStructureCursor::new(png.len() + 4);
        assert!(png_cursor.feed(&png).is_err());

        let mut protobuf_cursor = ProtobufStructureCursor::new(11);
        assert!(protobuf_cursor.feed(&[0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x80, 0x02]).is_err());
    }

    #[test]
    fn runtime_mailbox_reserves_completion_capacity_and_coalesces_only_matching_keys() {
        let completion = |key: Option<&'static str>, revision: u64| RuntimeCompletion { key, revision, requires_interaction: false, apply: RuntimeApply::Resize { width: 1.0, height: 1.0, dpr: 1.0 } };
        let mut queue = RuntimeCompletionQueue::new();
        for revision in 0..RUNTIME_COMPLETION_CAPACITY - 1 {
            assert!(queue.enqueue(completion(None, revision as u64)));
        }
        assert!(!queue.enqueue(completion(None, 10_000)));
        assert_eq!(queue.len(), RUNTIME_COMPLETION_CAPACITY - 1);

        let mut queue = RuntimeCompletionQueue::new();
        for revision in 0..RUNTIME_COMPLETION_CAPACITY - 1 {
            assert!(queue.enqueue(completion(Some("refresh"), revision as u64)));
        }
        assert!(queue.enqueue(completion(Some("refresh"), 10_000)));
        assert_eq!(queue.len(), RUNTIME_COMPLETION_CAPACITY - 1);
        assert_eq!(queue.ready.back().expect("latest refresh").revision, 10_000);

        let mut queue = RuntimeCompletionQueue::new();
        for _ in 0..RUNTIME_COMPLETION_CAPACITY - 1 {
            assert!(queue.reserve(None));
        }
        assert!(!queue.reserve(None));
        assert!(queue.reserve_interaction());
        assert!(!queue.reserve_interaction());
        queue.finish(completion(None, 20_000));
        assert_eq!(queue.len(), RUNTIME_COMPLETION_CAPACITY);
    }

    #[test]
    fn native_binary_owns_exactly_one_entrypoint_driver() {
        assert_eq!(BINARY_SOURCE.matches(concat!("block", "_on(")).count(), 1);
        assert_eq!(BINARY_SOURCE.matches("drive_entrypoint(").count(), 2);
    }

    #[test]
    fn manifest_has_no_retired_direct_edges() {
        assert!(!MANIFEST_SOURCE.contains(concat!("poll", "ster =")));
        assert!(!MANIFEST_SOURCE.contains(concat!("wasm-bindgen", "-test")));
        assert!(!MANIFEST_SOURCE.lines().any(|line| line.trim_start().starts_with("naga =")));
        assert!(!MANIFEST_SOURCE.lines().any(|line| line.trim_start().starts_with("rfd =")));
        assert!(!MANIFEST_SOURCE.lines().any(|line| line.trim_start().starts_with("ureq =")));
    }

    #[test]
    fn runtime_dispatch_cursor_preserves_coalesced_then_discrete_order() {
        let pointer = ui_host::PointerMoveSample { pointer: ui_render::PointerInfo { id: ui_render::PointerId(1), kind: ui_render::PointerKind::Mouse, pressure: None, tilt: None }, x: 1.0, y: 2.0, generation: ui_host::InputGeneration(1) };
        let scroll = ui_host::ScrollSample { x: 3.0, y: 4.0, delta_x: 5.0, delta_y: 6.0, generation: ui_host::InputGeneration(2) };
        let discrete = ui_host::DiscreteEvent { event: ui_render::DispatchEvent::KeyDown { key: "A".to_string(), modifiers: ui_render::EventModifiers::default() }, generation: ui_host::InputGeneration(3) };
        let mut events = ui_host::DrainedEvents { pointer_move: Some(pointer), scroll: Some(scroll), ..Default::default() };
        events.discrete[0] = Some(discrete);
        let mut cursor = RuntimeDispatchCursor::new(events);

        assert!(matches!(cursor.take_next(), Some(ui_render::DispatchEvent::PointerMove { .. })));
        assert!(matches!(cursor.take_next(), Some(ui_render::DispatchEvent::Scroll { .. })));
        assert!(matches!(cursor.take_next(), Some(ui_render::DispatchEvent::KeyDown { .. })));
        assert!(cursor.take_next().is_none());
        assert!(cursor.terminal_is_empty());
    }

    #[test]
    fn runtime_dispatch_cursor_retires_one_owned_event_per_step() {
        let mut events = ui_host::DrainedEvents::default();
        for (index, slot) in events.discrete.iter_mut().enumerate() {
            *slot = Some(ui_host::DiscreteEvent { event: ui_render::DispatchEvent::Paste { text: "x".repeat(4096) }, generation: ui_host::InputGeneration(index as u64) });
        }
        let mut cursor = RuntimeDispatchCursor::new(events);
        for _ in 0..cursor.events.discrete.len() {
            assert!(!cursor.close_step());
        }
        assert!(cursor.close_step());
        assert!(cursor.terminal_is_empty());
    }

    #[test]
    fn frame_deferred_cursor_advances_one_owned_operation_in_order() {
        let mut actions = FrameActionOwners::default();
        assert!(actions.try_push(ActionDescriptor { controller_id: "a".to_string(), action: "one".to_string(), args: None }).is_ok());
        assert!(actions.try_push(ActionDescriptor { controller_id: "b".to_string(), action: "two".to_string(), args: None }).is_ok());
        let mut cursor = FrameDeferredCursor::new(actions, true, true, true, 1, semio_framework_job::root_cancel_token());
        assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::ShellMaintenance)));
        assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::PumpSync)));
        assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::Action(action)) if action.action == "one"));
        assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::Action(action)) if action.action == "two"));
        assert!(matches!(cursor.take_next(), Some(FrameDeferredWork::FlushTutorial)));
        assert!(cursor.take_next().is_none());
        assert!(cursor.terminal_is_empty());
    }

    #[test]
    fn frame_deferred_cancel_retires_one_action_per_step() {
        let mut actions = FrameActionOwners::default();
        for index in 0..WORLD3D_DEADLINE_CAPACITY {
            assert!(actions.try_push(ActionDescriptor { controller_id: index.to_string(), action: "cancel".to_string(), args: None }).is_ok());
        }
        let mut cursor = FrameDeferredCursor::new(actions, false, true, false, 1, semio_framework_job::root_cancel_token());
        for _ in 0..WORLD3D_DEADLINE_CAPACITY {
            assert!(!cursor.close_step());
        }
        assert!(!cursor.close_step());
        assert!(cursor.close_step());
        assert!(cursor.terminal_is_empty());
    }

    #[test]
    fn frame_deferred_cancel_token_closes_one_exact_owner_per_grant() {
        let cancel = semio_framework_job::root_cancel_token();
        let mut actions = FrameActionOwners::default();
        assert!(actions.try_push(ActionDescriptor { controller_id: "cancel-owner".to_string(), action: "one".to_string(), args: None }).is_ok());
        let mut cursor = FrameDeferredCursor::new(actions, true, true, true, 17, cancel.clone());
        cancel.cancel_now();
        assert!(cursor.cancel.is_cancelled_now());
        cursor.begin_close();
        assert!(!cursor.close_step());
        assert!(!cursor.close_step());
        assert!(!cursor.close_step());
        assert!(!cursor.close_step());
        assert!(cursor.close_step());
        assert!(cursor.terminal_is_empty());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn frame_maintenance_authority_refuses_aba_and_releases_the_exact_generation() {
        let authority = FrameMaintenanceAuthority::new();
        assert!(authority.try_reserve(41));
        assert!(authority.is_live(41));
        assert!(!authority.try_reserve(42));
        assert!(!authority.release(42));
        assert!(authority.is_live(41));
        assert!(authority.release(41));
        assert!(!authority.is_live(41));
        assert!(authority.try_reserve(42));
        assert!(authority.release(42));
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn frame_maintenance_refusal_cell_recovers_exact_identity_without_blocking() {
        let owner = Box::new(73u64);
        let identity = owner.as_ref() as *const u64;
        let cell = FrameMaintenanceOwnerCell::new(owner);
        let Some(owner) = cell.try_take() else { panic!("exact owner admission") };
        assert_eq!(owner.as_ref() as *const u64, identity);
        assert!(cell.try_take().is_none());
        assert!(cell.try_restore(owner).is_ok());
        let Some(owner) = cell.try_take() else { panic!("exact owner handback") };
        assert_eq!(owner.as_ref() as *const u64, identity);
        assert!(cell.try_take().is_none());
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn frame_maintenance_cancel_and_stale_each_close_one_populated_owner_per_grant() {
        let cancel = semio_framework_job::root_cancel_token();
        let mut cancelled_actions = FrameActionOwners::default();
        assert!(cancelled_actions.try_push(ActionDescriptor { controller_id: "cancelled".to_string(), action: "one".to_string(), args: None }).is_ok());
        let mut cancelled = FrameDeferredCursor::new(cancelled_actions, false, false, false, 81, cancel.clone());
        cancel.cancel_now();
        assert_eq!(frame_maintenance_terminal_fault(cancelled.cancel.is_cancelled_now(), false, false), Some("frame maintenance cancelled"));
        cancelled.begin_close();
        let before = cancelled.actions.len;
        assert!(!cancelled.close_step());
        assert_eq!(before - cancelled.actions.len, 1);
        assert!(cancelled.close_step());
        assert!(cancelled.terminal_is_empty());

        let authority = FrameMaintenanceAuthority::new();
        assert!(authority.try_reserve(82));
        assert!(authority.release(82));
        let mut stale_actions = FrameActionOwners::default();
        assert!(stale_actions.try_push(ActionDescriptor { controller_id: "stale".to_string(), action: "one".to_string(), args: None }).is_ok());
        let mut stale = FrameDeferredCursor::new(stale_actions, false, false, false, 82, semio_framework_job::root_cancel_token());
        assert_eq!(frame_maintenance_terminal_fault(false, !authority.is_live(stale.generation), false), Some("frame maintenance generation became stale"));
        stale.begin_close();
        let before = stale.actions.len;
        assert!(!stale.close_step());
        assert_eq!(before - stale.actions.len, 1);
        assert!(stale.close_step());
        assert!(stale.terminal_is_empty());
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn frame_maintenance_test_owner(generation: u64, actions: usize) -> FrameMaintenanceOwner {
        let mut action_owners = FrameActionOwners::default();
        for index in 0..actions {
            assert!(action_owners.try_push(ActionDescriptor { controller_id: format!("owner-{index}"), action: "close".to_string(), args: None }).is_ok());
        }
        let interaction = AppInteractionState {
            shell: ShellState::new(Vec::new(), "maintenance-owner".to_string()),
            input: InputState::default(),
            theme: Theme::default(),
            theme_dark: false,
            last_pointer_x: 0.0,
            last_pointer_y: 0.0,
            pointer_down: false,
            pointer_button: 0,
            modifiers: PointerModifiers::default(),
            wheel_delta: 0.0,
            space_pressed: false,
            wheel_zoom_deadline_ms: 0.0,
            caret_blink_at_ms: 0.0,
            caret_blink_visible: true,
            text_streams: std::array::from_fn(|_| None),
            text_fault: None,
            frame_fault: None,
            text_cancel_pending: false,
            last_sync_pump_ms: 0.0,
        };
        FrameMaintenanceOwner { interaction: Some(interaction), cursor: Some(FrameDeferredCursor::new(action_owners, true, true, true, generation, semio_framework_job::root_cancel_token())), deadline_ms: Some(u64::MAX) }
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn accepted_frame_maintenance_queue_drop_publishes_exact_generation_owner() {
        let registry = FrameMaintenanceExecutionRegistry::new();
        let owner = frame_maintenance_test_owner(91, 1);
        let identity = owner.interaction.as_ref().map(|interaction| interaction.shell.plugin_filter.as_ptr());
        let cell = Arc::new(FrameMaintenanceOwnerCell::new(owner));
        assert!(registry.try_publish(91, cell.clone()).is_ok());
        assert!(!registry.abandon(92));
        assert!(registry.abandon(91));
        let Some((generation, recovered_cell)) = registry.try_take_abandoned() else { panic!("abandoned execution owner") };
        assert_eq!(generation, 91);
        assert!(Arc::ptr_eq(&cell, &recovered_cell));
        let Some(recovered) = recovered_cell.try_take() else { panic!("exact abandoned owner") };
        assert_eq!(recovered.interaction.as_ref().map(|interaction| interaction.shell.plugin_filter.as_ptr()), identity);
    }

    #[cfg(not(target_arch = "wasm32"))]
    #[test]
    fn interrupted_frame_maintenance_execution_restores_before_incremental_recovery() {
        let registry = FrameMaintenanceExecutionRegistry::new();
        let owner = frame_maintenance_test_owner(93, 2);
        let cell = Arc::new(FrameMaintenanceOwnerCell::new(owner));
        assert!(registry.try_publish(93, cell.clone()).is_ok());
        let Some(running_cell) = registry.try_begin(93) else { panic!("running execution cell") };
        let Some(mut running_owner) = running_cell.try_take() else { panic!("running exact owner") };
        let Some(cursor) = running_owner.cursor.as_mut() else { panic!("running cursor") };
        cursor.begin_close();
        running_cell.restore_taken(running_owner);
        assert!(registry.abandon(93));
        let Some((generation, recovered_cell)) = registry.try_take_abandoned() else { panic!("interrupted execution owner") };
        assert_eq!(generation, 93);
        let Some(mut recovered) = recovered_cell.try_take() else { panic!("interrupted exact owner") };
        let Some(cursor) = recovered.cursor.as_mut() else { panic!("interrupted cursor") };
        let before = cursor.actions.len;
        assert!(!cursor.close_step());
        assert_eq!(before - cursor.actions.len, 1);
    }

    #[test]
    fn frame_action_max_plus_one_returns_the_exact_owner() {
        let mut actions = FrameActionOwners::default();
        for index in 0..WORLD3D_DEADLINE_CAPACITY {
            assert!(actions.try_push(ActionDescriptor { controller_id: index.to_string(), action: "admitted".to_string(), args: None }).is_ok());
        }
        let rejected = ActionDescriptor { controller_id: "exact-owner".to_string(), action: "commit".to_string(), args: None };
        let identity = rejected.controller_id.as_ptr();
        let Err(rejected) = actions.try_push(rejected) else { panic!("MAX + 1 must refuse") };
        assert_eq!(rejected.controller_id.as_ptr(), identity);
    }

    #[test]
    fn asset_poll_does_not_accumulate_completed_response_vectors() {
        assert!(!LIBRARY_SOURCE.contains("let mut fetched_glb = Vec::new()"));
        assert!(!LIBRARY_SOURCE.contains("let mut fetched_map = Vec::new()"));
        assert!(!LIBRARY_SOURCE.contains("let mut fetched_ui_images = Vec::new()"));
    }

    #[test]
    fn runtime_presentation_authority_and_candidate_identity_change_independently() {
        let authority = RuntimePresentationAuthority::new();
        authority.observe_input_generation(7);
        let unchanged_candidate = authority.current();
        assert!(authority.matches(unchanged_candidate.scene_revision, unchanged_candidate.input_generation));

        authority.mark_scene_changed();
        assert_eq!(unchanged_candidate, RuntimePresentationWitness { scene_revision: 1, input_generation: 7 });
        assert!(!authority.matches(unchanged_candidate.scene_revision, unchanged_candidate.input_generation));

        let unchanged_authority = authority.current();
        let changed_candidate = RuntimePresentationWitness { scene_revision: unchanged_authority.scene_revision + 1, input_generation: unchanged_authority.input_generation + 1 };
        assert_eq!(authority.current(), unchanged_authority);
        assert!(!authority.matches(changed_candidate.scene_revision, changed_candidate.input_generation));

        authority.observe_input_generation(8);
        assert!(authority.witness_for(7).is_none());
        assert_eq!(authority.witness_for(8), Some(authority.current()));
    }

    #[test]
    fn runtime_raster_operation_authority_rejects_stale_duplicate_and_device_close_ack() {
        let authority = RuntimeRasterOperationAuthority::new();
        let first = authority.begin(3, 5).expect("independent raster operation");
        assert_eq!(first.scene_revision, 3);
        assert_eq!(first.preview_generation, 5);
        assert!(authority.matches(first));
        assert!(authority.begin(3, 5).is_err());
        let stale = ui_wgpu::wgpu::RasterTextureWitness { operation: first.operation.wrapping_add(1), ..first };
        assert!(!authority.matches(stale));
        assert!(authority.release(stale).is_err());
        assert!(authority.matches(first));
        authority.release(first).expect("device close returns exact operation owner");
        assert!(authority.current().is_none());
        assert!(authority.release(first).is_err());
        let second = authority.begin(3, 5).expect("next operation");
        assert!(second.operation > first.operation);
    }

    #[test]
    fn runtime_raster_operation_authority_exhausts_permanently_without_aba() {
        let authority = RuntimeRasterOperationAuthority::new();
        authority.0.next_operation.store(u64::MAX - 1, Ordering::Release);

        let penultimate = authority.begin(8, 13).expect("MAX-1 operation");
        assert_eq!(penultimate.operation, u64::MAX - 1);
        authority.release(penultimate).expect("release MAX-1");

        let last = authority.begin(8, 13).expect("MAX operation");
        assert_eq!(last.operation, u64::MAX);
        assert!(authority.begin(8, 13).is_err(), "live MAX remains occupied");
        authority.release(last).expect("release MAX");
        assert!(authority.begin(8, 13).is_err(), "exhaustion remains permanent after release");
        assert!(authority.current().is_none());
        assert!(authority.0.exhausted.load(Ordering::Acquire));
        assert_eq!(authority.0.next_operation.load(Ordering::Acquire), u64::MAX);
    }

    fn presenter_retirement_contract(glue: &str, prepared: &str, gpu: &str, draw: &str, host: &str, winit: &str) -> bool {
        let table = &draw[draw.find("pub struct MeshGpuTable").unwrap_or(0)..draw.find("pub const WORLD_GLOBALS_SLOT_SIZE").unwrap_or(draw.len())];
        prepared.contains("pub struct PreparedPresenterWitness")
            && prepared.contains("pub fn stage_presented")
            && prepared.contains("pub fn pending_presented")
            && prepared.contains("pub fn acknowledge_presented")
            && prepared.contains("pub fn abort_pending")
            && prepared.contains("pub fn take_last_valid")
            && prepared.contains("pub fn retire_step(&mut self) -> bool")
            && !prepared.contains("Option<Arc<PreparedRenderPacket>>")
            && glue.contains("AppPresentPhase::Acknowledge")
            && glue.contains("AppPresentPhase::Stage")
            && glue.contains("AppPresentPhase::Aborted")
            && glue.contains("self.gate.stage_presented(packet)")
            && glue.contains("self.gate.pending_presented(witness)")
            && glue.contains("self.gate.acknowledge_presented(witness)")
            && glue.contains("struct RuntimePresentationAuthority")
            && glue.contains("presentation_authority: RuntimePresentationAuthority")
            && glue.matches(concat!("runtime.presentation_", "witness_for(self.generation.0)")).count() == 1
            && glue.contains("presentation_witness.scene_revision")
            && glue.contains("presentation_witness.input_generation")
            && glue.matches(concat!("let expected = self.presentation_", "authority.current();")).count() == 2
            && glue.contains("begin_prepared(&token, &self.gate, packet, expected.scene_revision, expected.input_generation)")
            && glue.contains("begin_prepared_offscreen(token, &self.gate, packet, expected.scene_revision, expected.input_generation)")
            && glue.contains("packet.scene_revision() != expected.scene_revision || packet.preview_generation() != expected.input_generation")
            && glue.matches(concat!("self.presentation_authority.", "mark_scene_changed();")).count() == 2
            && glue.contains("runtime_presentation_authority_and_candidate_identity_change_independently")
            && !glue.contains(concat!("let revision = packet.", "scene_revision();"))
            && !glue.contains(concat!("let generation = packet.", "preview_generation();"))
            && !glue.contains(concat!("scene_revision: packet.", "scene_revision()"))
            && !glue.contains(concat!("input_generation: packet.", "preview_generation()"))
            && glue.contains("struct AppPresentedRetirement")
            && glue.contains("acknowledged_eviction")
            && glue.contains("acknowledged_upload_scan")
            && glue.contains("acknowledged_versions")
            && glue.contains("previous.retire_step()")
            && glue.contains("if !gpu.close_mesh_upload_step()")
            && glue.contains("self.gpu.close_mesh_table_step()")
            && glue.contains("pub(crate) fn admit_next_frame")
            && winit.contains(".admit_next_frame(|| frame_build.poll_runtime_and_resubmit")
            && winit.contains("observe_presentation_input_generation(build_generation.0)")
            && !winit.contains("begin_present(frame).is_err()")
            && host.contains("presenter.close_world_owners_step()")
            && host.contains("presenter.world_owners_terminal_is_empty()")
            && draw.contains("pub const MESH_GPU_TABLE_CAPACITY: usize = 256")
            && draw.contains("slots: [Option<MeshGpuEntry<T>>; MESH_GPU_TABLE_CAPACITY]")
            && draw.contains("pub fn evict_mesh_step")
            && draw.contains("pub fn evict_mesh_except_step")
            && draw.contains("mesh_gpu_retirement_preserves_acknowledged_versions")
            && draw.contains("pub fn close_step(&mut self) -> bool")
            && draw.contains("fixed_mesh_gpu_registry_rejects_capacity_plus_one_and_returns_exact_owner")
            && !table.contains("HashMap")
            && !table.contains(".retain(")
            && gpu.contains("self.mesh_store.evict_mesh_except_step(key, keep_versions)")
            && gpu.contains("self.mesh_store.close_step()")
    }

    #[test]
    fn presenter_ack_retirement_source_mutations_are_denied() {
        assert!(presenter_retirement_contract(LIBRARY_SOURCE, PREPARED_SOURCE, GPU_SOURCE, DRAW_SOURCE, OS_HOST_SOURCE, WINT_APP_SOURCE));
        let mutations = [
            (LIBRARY_SOURCE.replace("AppPresentPhase::Acknowledge", "AppPresentPhase::Fullscreen"), PREPARED_SOURCE.to_string(), GPU_SOURCE.to_string(), DRAW_SOURCE.to_string(), OS_HOST_SOURCE.to_string(), WINT_APP_SOURCE.to_string()),
            (
                LIBRARY_SOURCE.replace("self.gate.acknowledge_presented(witness)", "drop(witness); self.gate.acknowledge_presented_unchecked()"),
                PREPARED_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                DRAW_SOURCE.to_string(),
                OS_HOST_SOURCE.to_string(),
                WINT_APP_SOURCE.to_string(),
            ),
            (LIBRARY_SOURCE.to_string(), PREPARED_SOURCE.replace("pub fn abort_pending", "fn abandon_pending"), GPU_SOURCE.to_string(), DRAW_SOURCE.to_string(), OS_HOST_SOURCE.to_string(), WINT_APP_SOURCE.to_string()),
            (
                LIBRARY_SOURCE.to_string(),
                PREPARED_SOURCE.replace("last_valid: Option<PreparedRenderPacket>", "last_valid: Option<Arc<PreparedRenderPacket>>"),
                GPU_SOURCE.to_string(),
                DRAW_SOURCE.to_string(),
                OS_HOST_SOURCE.to_string(),
                WINT_APP_SOURCE.to_string(),
            ),
            (LIBRARY_SOURCE.replace("previous.retire_step()", "drop(previous)"), PREPARED_SOURCE.to_string(), GPU_SOURCE.to_string(), DRAW_SOURCE.to_string(), OS_HOST_SOURCE.to_string(), WINT_APP_SOURCE.to_string()),
            (LIBRARY_SOURCE.replace("AppPresentPhase::Stage", "AppPresentPhase::Render"), PREPARED_SOURCE.to_string(), GPU_SOURCE.to_string(), DRAW_SOURCE.to_string(), OS_HOST_SOURCE.to_string(), WINT_APP_SOURCE.to_string()),
            (LIBRARY_SOURCE.replace("AppPresentPhase::Aborted", "AppPresentPhase::Render"), PREPARED_SOURCE.to_string(), GPU_SOURCE.to_string(), DRAW_SOURCE.to_string(), OS_HOST_SOURCE.to_string(), WINT_APP_SOURCE.to_string()),
            (
                LIBRARY_SOURCE.to_string(),
                PREPARED_SOURCE.to_string(),
                GPU_SOURCE.replace("self.mesh_store.evict_mesh_except_step(key, keep_versions)", "self.mesh_store.evict_mesh_step(key)"),
                DRAW_SOURCE.to_string(),
                OS_HOST_SOURCE.to_string(),
                WINT_APP_SOURCE.to_string(),
            ),
            (
                LIBRARY_SOURCE.to_string(),
                PREPARED_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                DRAW_SOURCE.replace("MESH_GPU_TABLE_CAPACITY: usize = 256", "MESH_GPU_TABLE_CAPACITY: usize = usize::MAX"),
                OS_HOST_SOURCE.to_string(),
                WINT_APP_SOURCE.to_string(),
            ),
            (
                LIBRARY_SOURCE.to_string(),
                PREPARED_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                DRAW_SOURCE.replace("FixedMeshGpuRegistry<GpuMeshBuffers>", "std::collections::HashMap<String, GpuMeshBuffers>"),
                OS_HOST_SOURCE.to_string(),
                WINT_APP_SOURCE.to_string(),
            ),
            (
                LIBRARY_SOURCE.to_string(),
                PREPARED_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                DRAW_SOURCE.replace("pub fn close_step(&mut self) -> bool", "pub fn close_all(&mut self) -> bool"),
                OS_HOST_SOURCE.to_string(),
                WINT_APP_SOURCE.to_string(),
            ),
            (LIBRARY_SOURCE.to_string(), PREPARED_SOURCE.to_string(), GPU_SOURCE.to_string(), DRAW_SOURCE.to_string(), OS_HOST_SOURCE.replace("presenter.world_owners_terminal_is_empty()", "true"), WINT_APP_SOURCE.to_string()),
            (
                LIBRARY_SOURCE.to_string(),
                PREPARED_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                DRAW_SOURCE.to_string(),
                OS_HOST_SOURCE.to_string(),
                WINT_APP_SOURCE.replace(".admit_next_frame(|| frame_build.poll_runtime_and_resubmit", ".poll_runtime_and_resubmit"),
            ),
            (
                LIBRARY_SOURCE.replacen(
                    concat!("let expected = self.presentation_", "authority.current();"),
                    concat!("let expected = RuntimePresentationWitness { scene_revision: packet.", "scene_revision(), input_generation: packet.", "preview_generation() };"),
                    1,
                ),
                PREPARED_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                DRAW_SOURCE.to_string(),
                OS_HOST_SOURCE.to_string(),
                WINT_APP_SOURCE.to_string(),
            ),
            (
                LIBRARY_SOURCE.replace(concat!("runtime.presentation_", "witness_for(self.generation.0)"), "Some(RuntimePresentationWitness { scene_revision: self.generation.0, input_generation: self.generation.0 })"),
                PREPARED_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                DRAW_SOURCE.to_string(),
                OS_HOST_SOURCE.to_string(),
                WINT_APP_SOURCE.to_string(),
            ),
            (LIBRARY_SOURCE.replace(concat!("self.presentation_authority.", "mark_scene_changed();"), ""), PREPARED_SOURCE.to_string(), GPU_SOURCE.to_string(), DRAW_SOURCE.to_string(), OS_HOST_SOURCE.to_string(), WINT_APP_SOURCE.to_string()),
            (
                LIBRARY_SOURCE.to_string(),
                PREPARED_SOURCE.to_string(),
                GPU_SOURCE.to_string(),
                DRAW_SOURCE.to_string(),
                OS_HOST_SOURCE.to_string(),
                WINT_APP_SOURCE.replace("self.runtime.observe_presentation_input_generation(build_generation.0);", ""),
            ),
        ];
        for (glue, prepared, gpu, draw, host, winit) in mutations {
            assert!(!presenter_retirement_contract(&glue, &prepared, &gpu, &draw, &host, &winit));
        }
    }
}
//#endregion 🔖️AsyncBoundaryTests

//#region 📮️RuntimeMailbox

struct RuntimeDispatchCursor {
    events: ui_host::DrainedEvents,
    phase: u8,
    discrete_index: usize,
}

impl RuntimeDispatchCursor {
    fn new(mut events: ui_host::DrainedEvents) -> Self {
        events.metrics = None;
        Self { events, phase: 0, discrete_index: 0 }
    }

    fn take_next(&mut self) -> Option<ui_render::DispatchEvent> {
        if self.phase == 0 {
            self.phase = 1;
            if let Some(sample) = self.events.pointer_move.take() {
                return Some(ui_render::DispatchEvent::PointerMove { pointer: sample.pointer, x: sample.x, y: sample.y });
            }
        }
        if self.phase == 1 {
            self.phase = 2;
            if let Some(sample) = self.events.scroll.take() {
                return Some(ui_render::DispatchEvent::Scroll { x: sample.x, y: sample.y, delta_x: sample.delta_x, delta_y: sample.delta_y });
            }
        }
        while self.discrete_index < self.events.discrete.len() {
            let index = self.discrete_index;
            self.discrete_index += 1;
            if let Some(event) = self.events.discrete[index].take() {
                return Some(event.event);
            }
        }
        None
    }

    fn close_step(&mut self) -> bool {
        self.events.pointer_move = None;
        self.events.scroll = None;
        self.events.metrics = None;
        while self.discrete_index < self.events.discrete.len() {
            let index = self.discrete_index;
            self.discrete_index += 1;
            if self.events.discrete[index].take().is_some() {
                return false;
            }
        }
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.events.pointer_move.is_none() && self.events.scroll.is_none() && self.events.metrics.is_none() && self.events.discrete.iter().all(Option::is_none)
    }
}

/// 📦️ Fixed FIFO ownership for admitted frame actions; refusal returns the identical action.
struct FrameActionOwners {
    slots: [Option<ActionDescriptor>; WORLD3D_DEADLINE_CAPACITY],
    head: usize,
    len: usize,
}

impl Default for FrameActionOwners {
    fn default() -> Self {
        Self { slots: std::array::from_fn(|_| None), head: 0, len: 0 }
    }
}

impl FrameActionOwners {
    fn try_push(&mut self, action: ActionDescriptor) -> Result<(), ActionDescriptor> {
        if self.len == self.slots.len() {
            return Err(action);
        }
        let Some(index) = self.head.checked_add(self.len).map(|index| index % self.slots.len()) else { return Err(action) };
        let Some(next_len) = self.len.checked_add(1) else { return Err(action) };
        if self.slots[index].is_some() {
            return Err(action);
        }
        self.slots[index] = Some(action);
        self.len = next_len;
        Ok(())
    }

    fn pop_front(&mut self) -> Option<ActionDescriptor> {
        if self.len == 0 {
            return None;
        }
        let action = self.slots[self.head].take()?;
        self.head = self.head.checked_add(1).map(|index| index % self.slots.len())?;
        self.len = self.len.checked_sub(1)?;
        Some(action)
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}

struct FrameDeferredCursor {
    actions: FrameActionOwners,
    pump_sync: bool,
    flush_tutorial: bool,
    shell_maintenance: bool,
    phase: u8,
    generation: u64,
    cancel: semio_framework_async::CancelToken,
    closing: bool,
}

enum FrameDeferredWork {
    ShellMaintenance,
    PumpSync,
    Action(ActionDescriptor),
    FlushTutorial,
}

impl FrameDeferredCursor {
    fn new(actions: FrameActionOwners, pump_sync: bool, flush_tutorial: bool, shell_maintenance: bool, generation: u64, cancel: semio_framework_async::CancelToken) -> Self {
        Self { actions, pump_sync, flush_tutorial, shell_maintenance, phase: 0, generation, cancel, closing: false }
    }

    fn take_next(&mut self) -> Option<FrameDeferredWork> {
        if self.phase == 0 {
            self.phase = 1;
            if self.shell_maintenance {
                return Some(FrameDeferredWork::ShellMaintenance);
            }
        }
        if self.phase == 1 {
            self.phase = 2;
            if self.pump_sync {
                return Some(FrameDeferredWork::PumpSync);
            }
        }
        if let Some(action) = self.actions.pop_front() {
            return Some(FrameDeferredWork::Action(action));
        }
        if self.phase == 2 {
            self.phase = 3;
            if self.flush_tutorial {
                return Some(FrameDeferredWork::FlushTutorial);
            }
        }
        None
    }

    fn terminal_is_empty(&self) -> bool {
        !self.closing && self.actions.is_empty() && (self.phase > 0 || !self.shell_maintenance) && (self.phase > 1 || !self.pump_sync) && (self.phase > 2 || !self.flush_tutorial)
    }

    fn close_step(&mut self) -> bool {
        if self.actions.pop_front().is_some() {
            return false;
        }
        if self.shell_maintenance {
            self.shell_maintenance = false;
            return false;
        }
        if self.pump_sync {
            self.pump_sync = false;
            return false;
        }
        if self.flush_tutorial {
            self.flush_tutorial = false;
            return false;
        }
        self.phase = 3;
        self.closing = false;
        true
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn return_shell_maintenance(&mut self) {
        self.phase = 0;
        self.shell_maintenance = true;
        self.closing = false;
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct FrameMaintenanceOwner {
    interaction: Option<AppInteractionState>,
    cursor: Option<FrameDeferredCursor>,
    deadline_ms: Option<u64>,
}

#[cfg(not(target_arch = "wasm32"))]
impl FrameMaintenanceOwner {
    fn new(interaction: AppInteractionState, cursor: FrameDeferredCursor) -> Self {
        Self { interaction: Some(interaction), cursor: Some(cursor), deadline_ms: renderer_worker_pool().now_ms().checked_add(8) }
    }

    fn generation(&self) -> Option<u64> {
        self.cursor.as_ref().map(|cursor| cursor.generation)
    }

    fn cancel(&self) -> Option<&semio_framework_async::CancelToken> {
        self.cursor.as_ref().map(|cursor| &cursor.cancel)
    }

    fn terminal_is_empty(&self) -> bool {
        self.interaction.is_none() && self.cursor.is_none()
    }

    fn take_pair(&mut self) -> Option<(AppInteractionState, FrameDeferredCursor)> {
        if self.interaction.is_none() || self.cursor.is_none() {
            return None;
        }
        Some((self.interaction.take()?, self.cursor.take()?))
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct FrameMaintenanceOwnerCell<T> {
    state: std::sync::atomic::AtomicU8,
    owner: std::cell::UnsafeCell<Option<T>>,
}

#[cfg(not(target_arch = "wasm32"))]
unsafe impl<T: Send> Send for FrameMaintenanceOwnerCell<T> {}

#[cfg(not(target_arch = "wasm32"))]
unsafe impl<T: Send> Sync for FrameMaintenanceOwnerCell<T> {}

#[cfg(not(target_arch = "wasm32"))]
impl<T> FrameMaintenanceOwnerCell<T> {
    fn new(owner: T) -> Self {
        Self { state: std::sync::atomic::AtomicU8::new(0), owner: std::cell::UnsafeCell::new(Some(owner)) }
    }

    fn try_take(&self) -> Option<T> {
        self.state.compare_exchange(0, 1, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).ok()?;
        unsafe { (&mut *self.owner.get()).take() }
    }

    fn try_restore(&self, owner: T) -> Result<(), T> {
        if self.state.load(std::sync::atomic::Ordering::Acquire) != 1 {
            return Err(owner);
        }
        let slot = unsafe { &mut *self.owner.get() };
        if slot.is_some() {
            return Err(owner);
        }
        *slot = Some(owner);
        self.state.store(0, std::sync::atomic::Ordering::Release);
        Ok(())
    }

    fn restore_taken(&self, owner: T) {
        unsafe {
            *self.owner.get() = Some(owner);
        }
        self.state.store(0, std::sync::atomic::Ordering::Release);
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct FrameMaintenanceRefusal {
    owner: Arc<FrameMaintenanceOwnerCell<FrameMaintenanceOwner>>,
    reservation_live: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl FrameMaintenanceRefusal {
    fn try_take(&self) -> Option<FrameMaintenanceOwner> {
        self.owner.try_take()
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct FrameMaintenanceExecutionRegistry {
    state: std::sync::atomic::AtomicU8,
    generation: std::sync::atomic::AtomicU64,
    owner: std::cell::UnsafeCell<Option<Arc<FrameMaintenanceOwnerCell<FrameMaintenanceOwner>>>>,
}

#[cfg(not(target_arch = "wasm32"))]
unsafe impl Send for FrameMaintenanceExecutionRegistry {}

#[cfg(not(target_arch = "wasm32"))]
unsafe impl Sync for FrameMaintenanceExecutionRegistry {}

#[cfg(not(target_arch = "wasm32"))]
impl FrameMaintenanceExecutionRegistry {
    const WRITING: u8 = 1;
    const QUEUED: u8 = 2;
    const RUNNING: u8 = 3;
    const ABANDONED: u8 = 4;
    const RECOVERING: u8 = 5;

    fn new() -> Self {
        Self { state: std::sync::atomic::AtomicU8::new(0), generation: std::sync::atomic::AtomicU64::new(0), owner: std::cell::UnsafeCell::new(None) }
    }

    fn try_publish(&self, generation: u64, owner: Arc<FrameMaintenanceOwnerCell<FrameMaintenanceOwner>>) -> Result<(), Arc<FrameMaintenanceOwnerCell<FrameMaintenanceOwner>>> {
        if self.state.compare_exchange(0, Self::WRITING, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return Err(owner);
        }
        unsafe {
            *self.owner.get() = Some(owner);
        }
        self.generation.store(generation, std::sync::atomic::Ordering::Release);
        self.state.store(Self::QUEUED, std::sync::atomic::Ordering::Release);
        Ok(())
    }

    fn try_begin(&self, generation: u64) -> Option<Arc<FrameMaintenanceOwnerCell<FrameMaintenanceOwner>>> {
        if self.generation.load(std::sync::atomic::Ordering::Acquire) != generation {
            return None;
        }
        self.state.compare_exchange(Self::QUEUED, Self::RUNNING, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).ok()?;
        unsafe { (&*self.owner.get()).as_ref().cloned() }
    }

    fn abandon(&self, generation: u64) -> bool {
        if self.generation.load(std::sync::atomic::Ordering::Acquire) != generation {
            return false;
        }
        let state = self.state.load(std::sync::atomic::Ordering::Acquire);
        if !matches!(state, Self::QUEUED | Self::RUNNING) {
            return false;
        }
        self.state.compare_exchange(state, Self::ABANDONED, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok()
    }

    fn complete(&self, generation: u64) -> bool {
        if self.generation.load(std::sync::atomic::Ordering::Acquire) != generation || self.state.compare_exchange(Self::RUNNING, Self::WRITING, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return false;
        }
        let owner = unsafe { (&mut *self.owner.get()).take() };
        drop(owner);
        self.generation.store(0, std::sync::atomic::Ordering::Release);
        self.state.store(0, std::sync::atomic::Ordering::Release);
        true
    }

    fn reclaim_rejected(&self, generation: u64, fallback: Arc<FrameMaintenanceOwnerCell<FrameMaintenanceOwner>>) -> Arc<FrameMaintenanceOwnerCell<FrameMaintenanceOwner>> {
        if self.generation.load(std::sync::atomic::Ordering::Acquire) != generation {
            return fallback;
        }
        if self.state.compare_exchange(Self::QUEUED, Self::RECOVERING, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return fallback;
        }
        let owner = unsafe { (&mut *self.owner.get()).take() };
        self.generation.store(0, std::sync::atomic::Ordering::Release);
        self.state.store(0, std::sync::atomic::Ordering::Release);
        match owner {
            Some(owner) => owner,
            None => fallback,
        }
    }

    fn try_take_abandoned(&self) -> Option<(u64, Arc<FrameMaintenanceOwnerCell<FrameMaintenanceOwner>>)> {
        self.state.compare_exchange(Self::ABANDONED, Self::RECOVERING, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).ok()?;
        let generation = self.generation.load(std::sync::atomic::Ordering::Acquire);
        let owner = unsafe { (&mut *self.owner.get()).take() }?;
        self.generation.store(0, std::sync::atomic::Ordering::Release);
        self.state.store(0, std::sync::atomic::Ordering::Release);
        Some((generation, owner))
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct FrameMaintenanceAuthority {
    state: std::sync::atomic::AtomicU8,
    generation: std::sync::atomic::AtomicU64,
}

#[cfg(not(target_arch = "wasm32"))]
impl FrameMaintenanceAuthority {
    fn new() -> Self {
        Self { state: std::sync::atomic::AtomicU8::new(0), generation: std::sync::atomic::AtomicU64::new(0) }
    }

    fn try_reserve(&self, generation: u64) -> bool {
        if self.state.compare_exchange(0, 1, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_err() {
            return false;
        }
        self.generation.store(generation, std::sync::atomic::Ordering::Release);
        true
    }

    fn is_live(&self, generation: u64) -> bool {
        self.state.load(std::sync::atomic::Ordering::Acquire) == 1 && self.generation.load(std::sync::atomic::Ordering::Acquire) == generation
    }

    fn release(&self, generation: u64) -> bool {
        if !self.is_live(generation) {
            return false;
        }
        self.generation.store(0, std::sync::atomic::Ordering::Release);
        self.state.compare_exchange(1, 0, std::sync::atomic::Ordering::AcqRel, std::sync::atomic::Ordering::Acquire).is_ok()
    }
}

/// 🛑️ Classifies one maintenance terminal guard before any field work moves.
#[cfg(not(target_arch = "wasm32"))]
fn frame_maintenance_terminal_fault(cancelled: bool, stale: bool, deadline_exceeded: bool) -> Option<&'static str> {
    if cancelled {
        Some("frame maintenance cancelled")
    } else if stale {
        Some("frame maintenance generation became stale")
    } else if deadline_exceeded {
        Some("frame maintenance deadline expired")
    } else {
        None
    }
}

enum RuntimeApply {
    Resize {
        width: f32,
        height: f32,
        dpr: f32,
    },
    DispatchEvents(Option<RuntimeDispatchCursor>),
    ResumeDispatch {
        interaction: Option<AppInteractionState>,
        cursor: Option<RuntimeDispatchCursor>,
    },
    ResumeFrameDeferred {
        interaction: Option<AppInteractionState>,
        cursor: Option<FrameDeferredCursor>,
    },
    RestoreInteraction(Option<AppInteractionState>),
    #[cfg(not(target_arch = "wasm32"))]
    PluginReload(Option<Result<Vec<ProgramBridgeEntry>, String>>),
}

impl RuntimeApply {
    fn start_dispatch(cursor: &mut Option<RuntimeDispatchCursor>, runtime: &mut AppRuntime, handle: &AppHandle) -> bool {
        let Some(cursor_value) = cursor.as_mut() else { return true };
        if cursor_value.terminal_is_empty() {
            cursor.take();
            return true;
        }
        let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else {
            return false;
        };
        let Some(mut interaction) = runtime.interaction.take() else {
            return false;
        };
        if !mailbox.reserve_interaction_future() {
            interaction.frame_fault = Some("runtime dispatch completion credits exhausted".to_string());
            runtime.interaction = Some(interaction);
            return false;
        }
        let mut cursor_value = cursor.take().expect("dispatch cursor admitted above");
        let event = cursor_value.take_next().expect("non-empty dispatch cursor");
        mailbox.spawn_dispatch_reserved(async move {
            crate::winit_app::dispatch_normalized_event(&mut interaction, event).await;
            (interaction, cursor_value)
        });
        true
    }

    fn start_frame_deferred(cursor: &mut Option<FrameDeferredCursor>, runtime: &mut AppRuntime, handle: &AppHandle) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        if runtime.pending_frame_maintenance_refusal.is_none() {
            if let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) {
                runtime.pending_frame_maintenance_refusal = mailbox.try_take_abandoned_frame_maintenance();
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        if let Some(mut refusal) = runtime.pending_frame_maintenance_refusal.take() {
            let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else {
                runtime.pending_frame_maintenance_refusal = Some(refusal);
                return false;
            };
            if refusal.reservation_live {
                if !mailbox.cancel_interaction_future_reservation() {
                    runtime.pending_frame_maintenance_refusal = Some(refusal);
                    return false;
                }
                refusal.reservation_live = false;
            }
            let Some(mut owner) = refusal.try_take() else {
                runtime.pending_frame_maintenance_refusal = Some(refusal);
                return false;
            };
            let Some((interaction, mut returned_cursor)) = owner.take_pair() else {
                if let Err(owner) = refusal.owner.try_restore(owner) {
                    refusal.owner = Arc::new(FrameMaintenanceOwnerCell::new(owner));
                }
                runtime.pending_frame_maintenance_refusal = Some(refusal);
                return false;
            };
            returned_cursor.return_shell_maintenance();
            runtime.interaction = Some(interaction);
            *cursor = Some(returned_cursor);
            return false;
        }
        if cursor.as_ref().is_some_and(|cursor| cursor.closing) {
            let complete = cursor.as_mut().is_some_and(FrameDeferredCursor::close_step);
            if complete {
                cursor.take();
            }
            return complete;
        }
        let Some(cursor_value) = cursor.as_mut() else { return true };
        if cursor_value.terminal_is_empty() {
            cursor.take();
            return true;
        }
        let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else { return false };
        let Some(mut interaction) = runtime.interaction.take() else { return false };
        if !mailbox.reserve_interaction_future() {
            interaction.frame_fault = Some("frame deferred completion credits exhausted".to_string());
            runtime.interaction = Some(interaction);
            return false;
        }
        let Some(mut cursor_value) = cursor.take() else {
            runtime.interaction = Some(interaction);
            return false;
        };
        let Some(work) = cursor_value.take_next() else {
            runtime.interaction = Some(interaction);
            return true;
        };
        if matches!(work, FrameDeferredWork::ShellMaintenance) {
            #[cfg(not(target_arch = "wasm32"))]
            {
                match mailbox.try_spawn_frame_maintenance_reserved(FrameMaintenanceOwner::new(interaction, cursor_value)) {
                    Ok(()) => return true,
                    Err(refusal) => {
                        runtime.pending_frame_maintenance_refusal = Some(refusal);
                        return false;
                    }
                }
            }
            #[cfg(target_arch = "wasm32")]
            {
                mailbox.spawn_frame_maintenance_reserved(async move {
                    cursor_value.shell_maintenance = interaction.shell.advance_chrome_maintenance_step();
                    if cursor_value.shell_maintenance {
                        cursor_value.phase = 0;
                    }
                    (interaction, cursor_value)
                });
                return true;
            }
        }
        mailbox.spawn_frame_deferred_reserved(async move {
            match work {
                FrameDeferredWork::ShellMaintenance => {}
                FrameDeferredWork::PumpSync => {
                    #[cfg(not(target_arch = "wasm32"))]
                    interaction.shell.pump_sync_events().await;
                }
                FrameDeferredWork::Action(action) => {
                    if let Err(error) = interaction.shell.dispatch_action(action).await {
                        interaction.shell.error = Some(error);
                    }
                }
                FrameDeferredWork::FlushTutorial => interaction.shell.tutorial_flush_pending_document_ops().await,
            }
            (interaction, cursor_value)
        });
        true
    }

    fn apply_step(&mut self, runtime: &mut AppRuntime, handle: &AppHandle) -> bool {
        match self {
            Self::Resize { width, height, dpr } => runtime.resize(*width, *height, *dpr),
            Self::DispatchEvents(cursor) => return Self::start_dispatch(cursor, runtime, handle),
            Self::ResumeDispatch { interaction, cursor } => {
                if let Some(returned) = interaction.take() {
                    runtime.interaction = Some(returned);
                }
                return Self::start_dispatch(cursor, runtime, handle);
            }
            Self::ResumeFrameDeferred { interaction, cursor } => {
                if let Some(returned) = interaction.take() {
                    runtime.interaction = Some(returned);
                }
                return Self::start_frame_deferred(cursor, runtime, handle);
            }
            Self::RestoreInteraction(interaction) => runtime.interaction = interaction.take(),
            #[cfg(not(target_arch = "wasm32"))]
            Self::PluginReload(result) => match result.take() {
                Some(Ok(entries)) => {
                    let handle = handle.clone();
                    runtime.submit_interaction(&handle, Some("plugin-boot"), move |mut interaction| async move {
                        interaction.shell.prepare_hot_reload(entries);
                        if let Err(error) = interaction.shell.boot().await {
                            log_debug(&format!("wasm program hot reload failed: {error}"));
                        } else {
                            log_debug("wasm program hot reload complete");
                        }
                        interaction
                    });
                }
                Some(Err(error)) => log_debug(&format!("wasm program reload failed: {error}")),
                None => return true,
            },
        }
        true
    }
}

const RUNTIME_COMPLETION_CAPACITY: usize = 128;
const WORLD3D_DEADLINE_CAPACITY: usize = 256;
const WORLD3D_DEADLINE_ID_BYTES: usize = 256;
const EFFECT_STORM_BUDGET: u32 = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RuntimePresentationWitness {
    scene_revision: u64,
    input_generation: u64,
}

struct RuntimePresentationAuthorityInner {
    scene_revision: AtomicU64,
    input_generation: AtomicU64,
}

#[derive(Clone)]
struct RuntimePresentationAuthority(Arc<RuntimePresentationAuthorityInner>);

impl RuntimePresentationAuthority {
    fn new() -> Self {
        Self(Arc::new(RuntimePresentationAuthorityInner { scene_revision: AtomicU64::new(1), input_generation: AtomicU64::new(0) }))
    }

    fn mark_scene_changed(&self) {
        self.0.scene_revision.fetch_add(1, Ordering::AcqRel);
    }

    fn observe_input_generation(&self, generation: u64) {
        self.0.input_generation.store(generation, Ordering::Release);
    }

    fn current(&self) -> RuntimePresentationWitness {
        RuntimePresentationWitness { scene_revision: self.0.scene_revision.load(Ordering::Acquire), input_generation: self.0.input_generation.load(Ordering::Acquire) }
    }

    fn witness_for(&self, input_generation: u64) -> Option<RuntimePresentationWitness> {
        let witness = self.current();
        (witness.input_generation == input_generation).then_some(witness)
    }

    #[cfg(test)]
    fn matches(&self, scene_revision: u64, input_generation: u64) -> bool {
        self.current() == RuntimePresentationWitness { scene_revision, input_generation }
    }
}

struct RuntimeRasterOperationAuthorityInner {
    next_operation: AtomicU64,
    exhausted: AtomicBool,
    current: Mutex<Option<ui_wgpu::wgpu::RasterTextureWitness>>,
}

#[derive(Clone)]
struct RuntimeRasterOperationAuthority(Arc<RuntimeRasterOperationAuthorityInner>);

impl RuntimeRasterOperationAuthority {
    fn new() -> Self {
        Self(Arc::new(RuntimeRasterOperationAuthorityInner { next_operation: AtomicU64::new(1), exhausted: AtomicBool::new(false), current: Mutex::new(None) }))
    }

    fn begin(&self, scene_revision: u64, preview_generation: u64) -> Result<ui_wgpu::wgpu::RasterTextureWitness, &'static str> {
        let mut current = self.0.current.lock().map_err(|_| "raster operation authority was poisoned")?;
        if current.is_some() {
            return Err("raster operation authority was occupied");
        }
        let operation = loop {
            if self.0.exhausted.load(Ordering::Acquire) {
                return Err("raster operation generation exhausted");
            }
            let operation = self.0.next_operation.load(Ordering::Acquire);
            if operation == u64::MAX {
                if self.0.exhausted.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                    break operation;
                }
                continue;
            }
            if self.0.next_operation.compare_exchange(operation, operation + 1, Ordering::AcqRel, Ordering::Acquire).is_ok() {
                break operation;
            }
        };
        let witness = ui_wgpu::wgpu::RasterTextureWitness { scene_revision, preview_generation, operation };
        *current = Some(witness);
        Ok(witness)
    }

    fn current(&self) -> Option<ui_wgpu::wgpu::RasterTextureWitness> {
        self.0.current.lock().ok().and_then(|current| *current)
    }

    fn matches(&self, witness: ui_wgpu::wgpu::RasterTextureWitness) -> bool {
        self.current() == Some(witness)
    }

    fn release(&self, witness: ui_wgpu::wgpu::RasterTextureWitness) -> Result<(), &'static str> {
        let mut current = self.0.current.lock().map_err(|_| "raster operation authority was poisoned")?;
        if *current != Some(witness) {
            return Err("raster operation authority was stale or duplicated");
        }
        *current = None;
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
type RuntimeHostWaker = Arc<dyn Fn() + Send + Sync>;

#[cfg(target_arch = "wasm32")]
type RuntimeHostWaker = std::rc::Rc<dyn Fn()>;

type RuntimeCompletion = runtime_mailbox_core::Completion<RuntimeApply>;
type RuntimeCompletionQueue = runtime_mailbox_core::BoundedCompletionQueue<RuntimeApply, RUNTIME_COMPLETION_CAPACITY>;

#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../../../../../../🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/📥️enqueue/🧪️tests/🦀️.rs"]
mod runtime_publication_tests;

fn enqueue_runtime_completion(completions: &Mutex<RuntimeCompletionQueue>, presentation: &RuntimePresentationAuthority, waker: &Mutex<Option<RuntimeHostWaker>>, completion: RuntimeCompletion) -> bool {
    let mut queue = completions.lock().expect("runtime completion mailbox lock");
    if !queue.enqueue(completion) {
        return false;
    }
    drop(queue);
    #[cfg(all(test, not(target_arch = "wasm32")))]
    runtime_publication_tests::pause_after_completion_publication();
    presentation.mark_scene_changed();
    if let Some(waker) = waker.lock().expect("runtime completion waker lock").as_ref() {
        waker();
    }
    true
}

struct RuntimeMailboxInner {
    runtime: Mutex<AppRuntime>,
    presentation_authority: RuntimePresentationAuthority,
    raster_operation_authority: RuntimeRasterOperationAuthority,
    world_cursor_wake: infinite_world::world::WorldCursorWakeAuthority,
    completions: Mutex<RuntimeCompletionQueue>,
    waker: Mutex<Option<RuntimeHostWaker>>,
    next_revision: std::sync::atomic::AtomicU64,
    applied_revisions: Mutex<std::collections::HashMap<&'static str, u64>>,
    frame_inputs: Mutex<crate::frame_job::FrameBuildInputs>,
    frame_fault: Mutex<Option<String>>,
    #[cfg(not(target_arch = "wasm32"))]
    frame_maintenance: FrameMaintenanceAuthority,
    #[cfg(not(target_arch = "wasm32"))]
    frame_maintenance_executions: Arc<FrameMaintenanceExecutionRegistry>,
    world3d_close_cursor: Mutex<usize>,
    world3d_close_sequence: Mutex<u64>,
    world3d_asset_cursor: Mutex<usize>,
    world3d_asset_decode_cursor: Mutex<usize>,
    asset_probe: Mutex<Option<RendererAssetProbe>>,
    native_asset_fetching: AtomicBool,
    native_asset_blocked: Mutex<Option<RendererAssetFetchOwner>>,
    #[cfg(not(target_arch = "wasm32"))]
    native_asset_http: Arc<semio_framework_os_services::HttpPool>,
    #[cfg(not(target_arch = "wasm32"))]
    native_asset_https: Arc<semio_framework_os_services::HttpPool>,
    #[cfg(not(target_arch = "wasm32"))]
    native_asset_http_runtime: Arc<semio_framework_async::TokioHostRuntime>,
    #[cfg(not(target_arch = "wasm32"))]
    native_asset_http_scope: semio_framework_async::ScopeHandle,
    #[cfg(not(target_arch = "wasm32"))]
    native_asset_http_cancel: semio_framework_async::CancelToken,
}

impl RuntimeMailboxInner {
    fn try_lock(&self) -> std::sync::TryLockResult<std::sync::MutexGuard<'_, AppRuntime>> {
        self.runtime.try_lock()
    }

    fn completion(&self, key: Option<&'static str>, requires_interaction: bool, apply: RuntimeApply) -> RuntimeCompletion {
        RuntimeCompletion { key, revision: self.next_revision.fetch_add(1, std::sync::atomic::Ordering::Relaxed), requires_interaction, apply }
    }

    fn enqueue(&self, key: Option<&'static str>, requires_interaction: bool, apply: RuntimeApply) -> bool {
        let completion = self.completion(key, requires_interaction, apply);
        enqueue_runtime_completion(&self.completions, &self.presentation_authority, &self.waker, completion)
    }

    fn finish(&self, completion: RuntimeCompletion) {
        let mut queue = self.completions.lock().expect("runtime completion mailbox lock");
        queue.finish(completion);
        drop(queue);
        self.presentation_authority.mark_scene_changed();
        if let Some(waker) = self.waker.lock().expect("runtime completion waker lock").as_ref() {
            waker();
        }
    }
}

#[derive(Clone)]
pub(crate) struct RuntimeMailbox(Arc<RuntimeMailboxInner>);

#[cfg(not(target_arch = "wasm32"))]
struct FrameMaintenanceExecutionEnvelope {
    mailbox: RuntimeMailbox,
    registry: Arc<FrameMaintenanceExecutionRegistry>,
    generation: u64,
    revision: u64,
    armed: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl FrameMaintenanceExecutionEnvelope {
    fn new(mailbox: RuntimeMailbox, registry: Arc<FrameMaintenanceExecutionRegistry>, generation: u64, revision: u64) -> Self {
        Self { mailbox, registry, generation, revision, armed: true }
    }

    fn begin(mut self) -> Option<FrameMaintenanceExecutionGuard> {
        let owner_cell = self.registry.try_begin(self.generation)?;
        let owner = owner_cell.try_take()?;
        self.armed = false;
        Some(FrameMaintenanceExecutionGuard { mailbox: self.mailbox.clone(), registry: self.registry.clone(), owner_cell, owner: Some(owner), generation: self.generation, revision: self.revision, completed: false })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for FrameMaintenanceExecutionEnvelope {
    fn drop(&mut self) {
        if self.armed && self.registry.abandon(self.generation) {
            self.mailbox.0.finish(RuntimeCompletion { key: None, revision: self.revision, requires_interaction: false, apply: RuntimeApply::ResumeFrameDeferred { interaction: None, cursor: None } });
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct FrameMaintenanceExecutionGuard {
    mailbox: RuntimeMailbox,
    registry: Arc<FrameMaintenanceExecutionRegistry>,
    owner_cell: Arc<FrameMaintenanceOwnerCell<FrameMaintenanceOwner>>,
    owner: Option<FrameMaintenanceOwner>,
    generation: u64,
    revision: u64,
    completed: bool,
}

#[cfg(not(target_arch = "wasm32"))]
impl FrameMaintenanceExecutionGuard {
    fn owner_mut(&mut self) -> Option<&mut FrameMaintenanceOwner> {
        self.owner.as_mut()
    }

    fn finish(mut self) -> bool {
        let Some(owner) = self.owner.as_mut() else { return false };
        let Some((interaction, cursor)) = owner.take_pair() else { return false };
        if !self.registry.complete(self.generation) {
            owner.interaction = Some(interaction);
            owner.cursor = Some(cursor);
            return false;
        }
        self.completed = true;
        self.mailbox.0.finish(RuntimeCompletion { key: None, revision: self.revision, requires_interaction: false, apply: RuntimeApply::ResumeFrameDeferred { interaction: Some(interaction), cursor: Some(cursor) } });
        true
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Drop for FrameMaintenanceExecutionGuard {
    fn drop(&mut self) {
        if self.completed {
            return;
        }
        if let Some(owner) = self.owner.take() {
            self.owner_cell.restore_taken(owner);
        }
        if self.registry.abandon(self.generation) {
            self.mailbox.0.finish(RuntimeCompletion { key: None, revision: self.revision, requires_interaction: false, apply: RuntimeApply::ResumeFrameDeferred { interaction: None, cursor: None } });
        }
    }
}

impl RuntimeMailbox {
    fn new(runtime: AppRuntime) -> Self {
        let presentation_authority = RuntimePresentationAuthority::new();
        let raster_operation_authority = RuntimeRasterOperationAuthority::new();
        #[cfg(not(target_arch = "wasm32"))]
        let (native_asset_http, native_asset_https, native_asset_http_runtime, native_asset_http_scope, native_asset_http_cancel) = {
            use semio_framework_async::HostAsyncRuntime;
            let pool = renderer_worker_pool();
            let runtime = Arc::new(semio_framework_async::TokioHostRuntime::with_pool(pool.clone()));
            let scope = runtime.open_scope_now(semio_framework_async::ScopeOwner::Service("renderer_asset_http"), None);
            let compute = Arc::new(semio_framework_os_services::ComputePool::with_pool(2, pool));
            let http_transport = Arc::new(semio_framework_os_services::SocketHttpTransport::new(compute.clone(), runtime.clone(), scope.clone()));
            let https_transport = Arc::new(semio_framework_os_kernel::os_directory::client::native::UreqStreamingHttpTransport::new(compute, runtime.clone(), scope.clone()));
            let http = Arc::new(semio_framework_os_services::HttpPool::new_with_async_transport_now(http_transport, 256 * 1024 * 1024, 1));
            let https = Arc::new(semio_framework_os_services::HttpPool::new_with_async_transport_now(https_transport, 256 * 1024 * 1024, 1));
            (http, https, runtime, scope, semio_framework_async::CancelToken::root_now())
        };
        Self(Arc::new(RuntimeMailboxInner {
            runtime: Mutex::new(runtime),
            presentation_authority,
            raster_operation_authority,
            world_cursor_wake: infinite_world::world::WorldCursorWakeAuthority::new(),
            completions: Mutex::new(RuntimeCompletionQueue::new()),
            waker: Mutex::new(None),
            next_revision: std::sync::atomic::AtomicU64::new(1),
            applied_revisions: Mutex::new(std::collections::HashMap::new()),
            frame_inputs: Mutex::new(crate::frame_job::FrameBuildInputs::default()),
            frame_fault: Mutex::new(None),
            #[cfg(not(target_arch = "wasm32"))]
            frame_maintenance: FrameMaintenanceAuthority::new(),
            #[cfg(not(target_arch = "wasm32"))]
            frame_maintenance_executions: Arc::new(FrameMaintenanceExecutionRegistry::new()),
            world3d_close_cursor: Mutex::new(0),
            world3d_close_sequence: Mutex::new(0),
            world3d_asset_cursor: Mutex::new(0),
            world3d_asset_decode_cursor: Mutex::new(0),
            asset_probe: Mutex::new(None),
            native_asset_fetching: AtomicBool::new(false),
            native_asset_blocked: Mutex::new(None),
            #[cfg(not(target_arch = "wasm32"))]
            native_asset_http,
            #[cfg(not(target_arch = "wasm32"))]
            native_asset_https,
            #[cfg(not(target_arch = "wasm32"))]
            native_asset_http_runtime,
            #[cfg(not(target_arch = "wasm32"))]
            native_asset_http_scope,
            #[cfg(not(target_arch = "wasm32"))]
            native_asset_http_cancel,
        }))
    }

    fn downgrade(&self) -> AppHandle {
        Arc::downgrade(&self.0)
    }

    fn try_lock(&self) -> std::sync::TryLockResult<std::sync::MutexGuard<'_, AppRuntime>> {
        self.0.try_lock()
    }

    fn world_cursor_wake_authority(&self) -> infinite_world::world::WorldCursorWakeAuthority {
        self.0.world_cursor_wake.clone()
    }

    fn presentation_authority(&self) -> RuntimePresentationAuthority {
        self.0.presentation_authority.clone()
    }

    fn raster_operation_authority(&self) -> RuntimeRasterOperationAuthority {
        self.0.raster_operation_authority.clone()
    }

    fn presentation_witness_for(&self, input_generation: u64) -> Option<RuntimePresentationWitness> {
        self.0.presentation_authority.witness_for(input_generation)
    }

    pub(crate) fn observe_presentation_input_generation(&self, generation: u64) {
        self.0.presentation_authority.observe_input_generation(generation);
    }

    pub(crate) fn acknowledge_world_cursor_wake(&self, token: &infinite_world::world::WorldCursorWakeToken) -> bool {
        self.0.world_cursor_wake.acknowledge(token)
    }

    fn close_world_cursor_wake_step(&self) -> bool {
        self.0.world_cursor_wake.close_step() && self.0.world_cursor_wake.terminal_is_empty()
    }

    pub(crate) fn close_world3d_dynamic_step(&self) -> bool {
        if !self.close_renderer_asset_step() {
            return false;
        }
        let Ok(mut runtime) = self.try_lock() else {
            return false;
        };
        let Some(interaction) = runtime.interaction.as_mut() else {
            drop(runtime);
            return self.close_world_cursor_wake_step();
        };
        let Ok(mut cursor) = self.0.world3d_close_cursor.try_lock() else {
            return false;
        };
        let Some(surface_id) = interaction.shell.world3d_states.id_at(*cursor).map(str::to_owned) else {
            drop(cursor);
            drop(runtime);
            return self.close_world_cursor_wake_step();
        };
        let Some(state) = interaction.shell.world3d_states.get_mut(&surface_id) else {
            self.record_frame_fault("world3d close order lost ownership");
            return false;
        };
        if world3d_dynamic_retirement_terminal_is_empty(state) {
            *cursor += 1;
            return false;
        }
        if state.dynamic_retirement_is_idle() {
            begin_world3d_dynamic_retirement(state);
            return false;
        }
        let Ok(mut sequence) = self.0.world3d_close_sequence.try_lock() else {
            return false;
        };
        let now = semio_framework_job::default_now_us();
        let mut context = semio_framework_job::StepContext::new(
            semio_framework_job::OperationId(1),
            semio_framework_job::Generation(1),
            now.and_then(|now| semio_framework_job::StepBudget::from_duration(1, now, semio_framework_job::MAINTENANCE_LANE_WALL_US)).unwrap_or(semio_framework_job::StepBudget::new(0, 0)),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            &mut sequence,
        );
        if step_world3d_dynamic_retirement(state, &mut context) && world3d_dynamic_retirement_terminal_is_empty(state) {
            *cursor += 1;
        }
        false
    }

    pub(crate) fn begin_engine_surface_close(&self, token: engine_canvas::EngineSurfaceToken) -> Result<bool, ()> {
        engine_canvas::begin_engine_surface_close_token(token)
    }

    pub(crate) fn close_engine_surface_step(&self, token: engine_canvas::EngineSurfaceToken, operation: semio_framework_trace::OperationId, sequence: &mut u64) -> bool {
        let Ok(mut runtime) = self.try_lock() else {
            return false;
        };
        let Some(interaction) = runtime.interaction.as_mut() else {
            return false;
        };
        let now = semio_framework_job::default_now_us();
        let deadline = match now.and_then(|now| now.checked_add(1_000)) {
            Some(deadline) => deadline,
            None => return false,
        };
        let mut context = semio_framework_job::StepContext::new(
            operation,
            semio_framework_trace::Generation(token.generation),
            semio_framework_job::StepBudget::new(1, deadline),
            semio_framework_job::root_cancel_token(),
            semio_framework_job::default_now_us,
            sequence,
        );
        engine_canvas::close_engine_surface_step(token, &mut context, &mut interaction.input)
    }

    pub(crate) fn engine_surface_terminal_is_empty(&self, token: engine_canvas::EngineSurfaceToken) -> Result<bool, ()> {
        engine_canvas::engine_surface_terminal_nonopaque_is_empty(token)
    }

    pub(crate) fn close_input_step(&self) -> bool {
        let Ok(mut runtime) = self.try_lock() else {
            return false;
        };
        let Some(interaction) = runtime.interaction.as_mut() else { return true };
        interaction.input.close_step().is_ok_and(|complete| complete) && interaction.input.terminal_is_empty()
    }

    pub(crate) fn take_renderer_asset_step(&self) -> Option<RendererAssetFetchOwner> {
        if let Some(owner) = take_next_renderer_asset() {
            return Some(RendererAssetFetchOwner::Shared(owner));
        }
        let Ok(mut runtime) = self.try_lock() else { return None };
        let interaction = runtime.interaction.as_mut()?;
        let Ok(mut cursor) = self.0.world3d_asset_cursor.try_lock() else { return None };
        let Some(surface_id) = interaction.shell.world3d_states.id_at(*cursor).map(str::to_owned) else {
            *cursor = 0;
            return None;
        };
        *cursor += 1;
        let owner = take_next_world3d_asset(interaction.shell.world3d_states.get_mut(&surface_id)?)?;
        let surface = WorldAssetMetadataId::try_from_str(&surface_id).ok()?;
        Some(RendererAssetFetchOwner::World { surface, owner })
    }

    pub(crate) fn reserve_renderer_asset_response(&self, fetch: &mut RendererAssetFetchOwner, byte_credits: usize) -> bool {
        match fetch {
            RendererAssetFetchOwner::Shared(owner) => reserve_renderer_asset_response(owner, byte_credits),
            RendererAssetFetchOwner::World { surface, owner } => {
                let Ok(mut runtime) = self.try_lock() else { return false };
                let Some(interaction) = runtime.interaction.as_mut() else { return false };
                let Some(state) = interaction.shell.world3d_states.get_mut(surface.as_str()) else { return false };
                reserve_world3d_asset_response(state, owner, byte_credits).is_ok()
            }
        }
    }

    fn take_completed_renderer_asset_step(&self) -> Option<RendererAssetFetchOwner> {
        if let Some(owner) = take_next_completed_renderer_asset_step() {
            return Some(RendererAssetFetchOwner::Shared(owner));
        }
        let Ok(mut runtime) = self.try_lock() else { return None };
        let interaction = runtime.interaction.as_mut()?;
        let Ok(mut cursor) = self.0.world3d_asset_decode_cursor.try_lock() else { return None };
        let Some(surface_id) = interaction.shell.world3d_states.id_at(*cursor).map(str::to_owned) else {
            *cursor = 0;
            return None;
        };
        *cursor += 1;
        let owner = take_next_completed_world3d_asset_step(interaction.shell.world3d_states.get_mut(&surface_id)?)?;
        let surface = WorldAssetMetadataId::try_from_str(&surface_id).ok()?;
        Some(RendererAssetFetchOwner::World { surface, owner })
    }

    fn finish_renderer_asset_owner(&self, fetch: RendererAssetFetchOwner) -> Result<(), RendererAssetFetchOwner> {
        match fetch {
            RendererAssetFetchOwner::Shared(owner) => finish_renderer_asset(owner).map_err(RendererAssetFetchOwner::Shared),
            RendererAssetFetchOwner::World { surface, owner } => {
                let Ok(mut runtime) = self.try_lock() else { return Err(RendererAssetFetchOwner::World { surface, owner }) };
                let Some(interaction) = runtime.interaction.as_mut() else { return Err(RendererAssetFetchOwner::World { surface, owner }) };
                let Some(state) = interaction.shell.world3d_states.get_mut(surface.as_str()) else { return Err(RendererAssetFetchOwner::World { surface, owner }) };
                finish_world3d_asset(state, owner).map_err(|owner| RendererAssetFetchOwner::World { surface, owner })
            }
        }
    }

    fn pump_renderer_asset_decode_step(&self) -> bool {
        let Ok(mut probe_slot) = self.0.asset_probe.try_lock() else { return false };
        if probe_slot.is_none() {
            let Some(owner) = self.take_completed_renderer_asset_step() else { return false };
            *probe_slot = Some(RendererAssetProbe::new(owner));
            return true;
        }
        let probe = probe_slot.as_mut().expect("asset probe initialized above");
        if matches!(probe.phase, RendererAssetProbePhase::Ready) {
            let surface = match probe.owner() {
                RendererAssetFetchOwner::World { surface, .. } => *surface,
                RendererAssetFetchOwner::Shared(_) => return false,
            };
            let Some(lease) = probe.take_ready_mesh_lease() else { return false };
            let Ok(mut runtime) = self.try_lock() else {
                probe.restore_ready_mesh_lease(lease);
                return false;
            };
            let Some(interaction) = runtime.interaction.as_mut() else {
                probe.restore_ready_mesh_lease(lease);
                return false;
            };
            let Some(state) = interaction.shell.world3d_states.get_mut(surface.as_str()) else {
                probe.restore_ready_mesh_lease(lease);
                return false;
            };
            match publish_world3d_asset_mesh_lease(state, probe.owner().url(), lease) {
                Ok(()) => {
                    probe.finish_ready_mesh();
                    return true;
                }
                Err(rejected) => {
                    let stale = rejected.fault == WorldDynamicFault::StaleToken;
                    probe.restore_ready_mesh_lease(rejected.value);
                    if stale {
                        probe.begin_close();
                        drop(runtime);
                        drop(probe_slot);
                        self.record_frame_fault("asset mesh publication generation/revision witness was stale");
                        return true;
                    }
                    return false;
                }
            }
        }
        if matches!(probe.phase, RendererAssetProbePhase::Closing) {
            if !probe.close_step() {
                return true;
            }
            let owner = probe.take_terminal_owner().expect("completed close owns terminal response");
            *probe_slot = None;
            drop(probe_slot);
            if let Err(owner) = self.finish_renderer_asset_owner(owner) {
                let mut probe_slot = self.0.asset_probe.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                *probe_slot = Some(RendererAssetProbe::new(owner));
                probe_slot.as_mut().expect("restored asset probe").begin_close();
            }
            return true;
        }
        match probe.step() {
            RendererAssetProbeStep::Pending => true,
            RendererAssetProbeStep::Ready => true,
            RendererAssetProbeStep::Fault(detail) => {
                drop(probe_slot);
                self.record_frame_fault(detail);
                true
            }
        }
    }

    fn close_renderer_asset_probe_step(&self) -> bool {
        let Ok(mut probe_slot) = self.0.asset_probe.try_lock() else { return false };
        let Some(probe) = probe_slot.as_mut() else { return true };
        if !probe.close_step() {
            return false;
        }
        let owner = probe.take_terminal_owner().expect("asset probe terminal close witness");
        *probe_slot = None;
        drop(probe_slot);
        match self.finish_renderer_asset_owner(owner) {
            Ok(()) => false,
            Err(owner) => {
                let mut probe_slot = self.0.asset_probe.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
                *probe_slot = Some(RendererAssetProbe::new(owner));
                probe_slot.as_mut().expect("restored asset probe").begin_close();
                false
            }
        }
    }

    pub(crate) fn return_renderer_asset_owner(&self, fetch: RendererAssetFetchOwner) -> Result<(), RendererAssetFetchOwner> {
        match fetch {
            RendererAssetFetchOwner::Shared(owner) => return_renderer_asset(owner).map_err(RendererAssetFetchOwner::Shared),
            RendererAssetFetchOwner::World { surface, owner } => {
                let Ok(mut runtime) = self.try_lock() else { return Err(RendererAssetFetchOwner::World { surface, owner }) };
                let Some(interaction) = runtime.interaction.as_mut() else { return Err(RendererAssetFetchOwner::World { surface, owner }) };
                let Some(state) = interaction.shell.world3d_states.get_mut(surface.as_str()) else { return Err(RendererAssetFetchOwner::World { surface, owner }) };
                return_world3d_asset(state, owner).map_err(|owner| RendererAssetFetchOwner::World { surface, owner })
            }
        }
    }

    pub(crate) fn seal_renderer_asset_response(&self, fetch: &mut RendererAssetFetchOwner) -> bool {
        match fetch {
            RendererAssetFetchOwner::Shared(owner) => seal_renderer_asset_response(owner),
            RendererAssetFetchOwner::World { surface, owner } => {
                let Ok(mut runtime) = self.0.runtime.try_lock() else { return false };
                let Some(interaction) = runtime.interaction.as_mut() else { return false };
                let Some(state) = interaction.shell.world3d_states.get_mut(surface.as_str()) else { return false };
                seal_world3d_asset_response(state, owner).is_ok()
            }
        }
    }

    pub(crate) fn close_renderer_asset_step(&self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        self.0.native_asset_http_cancel.cancel_now();
        if !self.close_renderer_asset_probe_step() {
            return false;
        }
        if self.0.native_asset_fetching.load(Ordering::Acquire) {
            let _ = close_renderer_asset_step();
            return false;
        }
        if let Ok(mut blocked) = self.0.native_asset_blocked.try_lock() {
            if let Some(owner) = blocked.as_mut() {
                if !owner.close_step() {
                    return false;
                }
                *blocked = None;
                return false;
            }
        } else {
            return false;
        }
        close_renderer_asset_step()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn renderer_asset_cancelled(&self, fetch: &RendererAssetFetchOwner) -> bool {
        match fetch {
            RendererAssetFetchOwner::Shared(owner) => renderer_asset_io().lock().map_or(true, |authority| authority.cancellation_requested(owner.token())),
            RendererAssetFetchOwner::World { surface, owner } => {
                let Ok(mut runtime) = self.try_lock() else { return true };
                let Some(interaction) = runtime.interaction.as_mut() else { return true };
                interaction.shell.world3d_states.get(surface.as_str()).is_none_or(|state| world3d_asset_cancellation_requested(state, owner.token()))
            }
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn retain_native_asset_blocked(&self, mut fetch: RendererAssetFetchOwner) {
        fetch.begin_close();
        let mut blocked = self.0.native_asset_blocked.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        assert!(blocked.is_none(), "one native asset fetch is admitted at a time");
        *blocked = Some(fetch);
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn pump_native_asset(&self) -> bool {
        if self.0.native_asset_blocked.lock().unwrap_or_else(|poisoned| poisoned.into_inner()).is_some() {
            return false;
        }
        if self.0.native_asset_fetching.compare_exchange(false, true, Ordering::AcqRel, Ordering::Acquire).is_err() {
            return false;
        }
        let Some(mut fetch) = self.take_renderer_asset_step() else {
            self.0.native_asset_fetching.store(false, Ordering::Release);
            return false;
        };
        if !self.reserve_renderer_asset_response(&mut fetch, WORLD_ASSET_RESPONSE_BYTE_CAPACITY) {
            fetch.begin_close();
            match self.return_renderer_asset_owner(fetch) {
                Ok(()) => {}
                Err(fetch) => self.retain_native_asset_blocked(fetch),
            }
            self.0.native_asset_fetching.store(false, Ordering::Release);
            return true;
        }
        let mailbox = self.clone();
        spawn_app_task(async move {
            let result = stream_native_renderer_asset(&mailbox, &mut fetch).await;
            if result.is_err() {
                fetch.begin_close();
            }
            match mailbox.return_renderer_asset_owner(fetch) {
                Ok(()) => {}
                Err(fetch) => mailbox.retain_native_asset_blocked(fetch),
            }
            if let Err(error) = result {
                mailbox.record_frame_fault(error);
            }
            mailbox.0.native_asset_fetching.store(false, Ordering::Release);
            if let Some(waker) = mailbox.0.waker.lock().expect("runtime completion waker lock").as_ref() {
                waker();
            }
        });
        true
    }

    fn set_waker(&self, waker: RuntimeHostWaker) {
        *self.0.waker.lock().expect("runtime completion waker lock") = Some(waker);
    }

    fn enqueue_apply(&self, key: Option<&'static str>, requires_interaction: bool, apply: RuntimeApply) -> bool {
        self.0.enqueue(key, requires_interaction, apply)
    }

    fn has_lossless_capacity(&self) -> bool {
        self.0.completions.lock().expect("runtime completion mailbox lock").len() < RUNTIME_COMPLETION_CAPACITY - 1
    }

    fn has_pending_text_work(&self) -> bool {
        self.try_lock().ok().and_then(|runtime| runtime.interaction.as_ref().map(AppInteractionState::has_pending_text_work)).unwrap_or(false)
    }

    fn take_text_fault(&self) -> Option<String> {
        self.try_lock().ok()?.interaction.as_mut()?.text_fault.take()
    }

    fn take_frame_fault(&self) -> Option<String> {
        if let Some(fault) = self.0.frame_fault.lock().expect("runtime frame fault lock").take() {
            return Some(fault);
        }
        self.try_lock().ok()?.interaction.as_mut()?.frame_fault.take()
    }

    pub(crate) fn record_frame_fault(&self, fault: &'static str) {
        let mut slot = self.0.frame_fault.lock().expect("runtime frame fault lock");
        if slot.is_none() {
            *slot = Some(fault.to_string());
        }
    }

    fn frame_inputs(&self, now_ms: f64) -> crate::frame_job::FrameBuildInputs {
        let mut inputs = self.0.frame_inputs.try_lock().map(|inputs| inputs.clone()).unwrap_or_default();
        inputs.now_ms = now_ms;
        inputs
    }

    fn update_frame_inputs(&self, runtime: &AppRuntime) {
        if !runtime.interaction_available() {
            return;
        }
        *self.0.frame_inputs.lock().expect("runtime frame inputs lock") = crate::frame_job::FrameBuildInputs { wheel_zoom_deadline_ms: runtime.wheel_zoom_deadline_ms, now_ms: app_now_ms() };
    }

    fn reserve_future(&self, key: Option<&'static str>) -> bool {
        self.0.completions.lock().expect("runtime completion mailbox lock").reserve(key)
    }

    fn reserve_interaction_future(&self) -> bool {
        self.0.completions.lock().expect("runtime completion mailbox lock").reserve_interaction()
    }

    fn cancel_interaction_future_reservation(&self) -> bool {
        match self.0.completions.try_lock() {
            Ok(mut completions) => completions.cancel_interaction_reservation(),
            Err(std::sync::TryLockError::Poisoned(error)) => error.into_inner().cancel_interaction_reservation(),
            Err(std::sync::TryLockError::WouldBlock) => false,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn try_take_abandoned_frame_maintenance(&self) -> Option<FrameMaintenanceRefusal> {
        let (generation, owner) = self.0.frame_maintenance_executions.try_take_abandoned()?;
        let _ = self.0.frame_maintenance.release(generation);
        Some(FrameMaintenanceRefusal { owner, reservation_live: false })
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn spawn_interaction_reserved<F>(&self, _key: Option<&'static str>, future: F)
    where
        F: Future<Output = AppInteractionState> + Send + 'static,
    {
        let mailbox = self.clone();
        let revision = mailbox.0.next_revision.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        spawn_app_task(async move {
            let interaction = future.await;
            mailbox.0.finish(RuntimeCompletion { key: None, revision, requires_interaction: false, apply: RuntimeApply::RestoreInteraction(Some(interaction)) });
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn spawn_interaction_reserved<F>(&self, _key: Option<&'static str>, future: F)
    where
        F: Future<Output = AppInteractionState> + 'static,
    {
        let mailbox = self.clone();
        let revision = mailbox.0.next_revision.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        spawn_app_task(async move {
            let interaction = future.await;
            mailbox.0.finish(RuntimeCompletion { key: None, revision, requires_interaction: false, apply: RuntimeApply::RestoreInteraction(Some(interaction)) });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn spawn_dispatch_reserved<F>(&self, future: F)
    where
        F: Future<Output = (AppInteractionState, RuntimeDispatchCursor)> + Send + 'static,
    {
        let mailbox = self.clone();
        let revision = mailbox.0.next_revision.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        spawn_app_task(async move {
            let (interaction, cursor) = future.await;
            mailbox.0.finish(RuntimeCompletion { key: None, revision, requires_interaction: false, apply: RuntimeApply::ResumeDispatch { interaction: Some(interaction), cursor: Some(cursor) } });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn spawn_frame_deferred_reserved<F>(&self, future: F)
    where
        F: Future<Output = (AppInteractionState, FrameDeferredCursor)> + Send + 'static,
    {
        let mailbox = self.clone();
        let revision = mailbox.0.next_revision.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        spawn_app_task(async move {
            let (interaction, cursor) = future.await;
            mailbox.0.finish(RuntimeCompletion { key: None, revision, requires_interaction: false, apply: RuntimeApply::ResumeFrameDeferred { interaction: Some(interaction), cursor: Some(cursor) } });
        });
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn try_spawn_frame_maintenance_reserved(&self, owner: FrameMaintenanceOwner) -> Result<(), FrameMaintenanceRefusal> {
        let Some(generation) = owner.generation() else {
            return Err(FrameMaintenanceRefusal { owner: Arc::new(FrameMaintenanceOwnerCell::new(owner)), reservation_live: true });
        };
        let cell = Arc::new(FrameMaintenanceOwnerCell::new(owner));
        if !self.0.frame_maintenance.try_reserve(generation) {
            return Err(FrameMaintenanceRefusal { owner: cell, reservation_live: true });
        }
        let registry = self.0.frame_maintenance_executions.clone();
        if registry.try_publish(generation, cell.clone()).is_err() {
            let _ = self.0.frame_maintenance.release(generation);
            return Err(FrameMaintenanceRefusal { owner: cell, reservation_live: true });
        }
        let mailbox = self.clone();
        let revision = mailbox.0.next_revision.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        let worker_pool = renderer_worker_pool();
        let execution = FrameMaintenanceExecutionEnvelope::new(mailbox.clone(), registry.clone(), generation, revision);
        let job: semio_framework_async::Job = Box::new(move || {
            let Some(mut execution) = execution.begin() else {
                mailbox.record_frame_fault("frame maintenance exact owner pair was incomplete");
                return;
            };
            let stale;
            {
                let Some(owner) = execution.owner_mut() else {
                    mailbox.record_frame_fault("frame maintenance execution lost its exact owner");
                    return;
                };
                let FrameMaintenanceOwner { interaction, cursor, deadline_ms } = owner;
                let (Some(interaction), Some(cursor)) = (interaction.as_mut(), cursor.as_mut()) else {
                    mailbox.record_frame_fault("frame maintenance exact owner pair was incomplete");
                    return;
                };
                let cancelled = cursor.cancel.is_cancelled_now();
                stale = !mailbox.0.frame_maintenance.is_live(generation) || mailbox.0.presentation_authority.witness_for(generation).is_none();
                let deadline_exceeded = deadline_ms.is_none_or(|deadline| worker_pool.now_ms() > deadline);
                if let Some(fault) = frame_maintenance_terminal_fault(cancelled, stale, deadline_exceeded) {
                    cursor.begin_close();
                    interaction.frame_fault = Some(fault.to_string());
                } else {
                    cursor.shell_maintenance = interaction.shell.advance_chrome_maintenance_step();
                    if cursor.shell_maintenance {
                        cursor.phase = 0;
                    }
                    if deadline_ms.is_some_and(|deadline| worker_pool.now_ms() > deadline) {
                        interaction.frame_fault = Some("frame maintenance bounded field exceeded its deadline".to_string());
                        cursor.begin_close();
                    }
                }
                if !mailbox.0.frame_maintenance.release(generation) && !stale {
                    interaction.frame_fault = Some("frame maintenance authority release faulted".to_string());
                    cursor.begin_close();
                }
            }
            if !execution.finish() {
                mailbox.record_frame_fault("frame maintenance execution handback faulted");
            }
        });
        match renderer_worker_pool().try_submit(semio_framework_async::Lane::Io, job) {
            Ok(()) => Ok(()),
            Err(error) => {
                let reclaimed = registry.reclaim_rejected(generation, cell);
                drop(error.into_job());
                let _ = self.0.frame_maintenance.release(generation);
                Err(FrameMaintenanceRefusal { owner: reclaimed, reservation_live: true })
            }
        }
    }

    #[cfg(target_arch = "wasm32")]
    fn spawn_frame_maintenance_reserved<F>(&self, future: F)
    where
        F: Future<Output = (AppInteractionState, FrameDeferredCursor)> + 'static,
    {
        self.spawn_frame_deferred_reserved(future);
    }

    #[cfg(target_arch = "wasm32")]
    fn spawn_frame_deferred_reserved<F>(&self, future: F)
    where
        F: Future<Output = (AppInteractionState, FrameDeferredCursor)> + 'static,
    {
        let mailbox = self.clone();
        let revision = mailbox.0.next_revision.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        spawn_app_task(async move {
            let (interaction, cursor) = future.await;
            mailbox.0.finish(RuntimeCompletion { key: None, revision, requires_interaction: false, apply: RuntimeApply::ResumeFrameDeferred { interaction: Some(interaction), cursor: Some(cursor) } });
        });
    }

    #[cfg(target_arch = "wasm32")]
    fn spawn_dispatch_reserved<F>(&self, future: F)
    where
        F: Future<Output = (AppInteractionState, RuntimeDispatchCursor)> + 'static,
    {
        let mailbox = self.clone();
        let revision = mailbox.0.next_revision.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        spawn_app_task(async move {
            let (interaction, cursor) = future.await;
            mailbox.0.finish(RuntimeCompletion { key: None, revision, requires_interaction: false, apply: RuntimeApply::ResumeDispatch { interaction: Some(interaction), cursor: Some(cursor) } });
        });
    }

    fn apply_pending_step(&self) -> bool {
        let Ok(mut runtime) = self.try_lock() else {
            return false;
        };
        let mut queue = self.0.completions.lock().expect("runtime completion mailbox lock");
        if queue.ready.front().is_some_and(|completion| completion.requires_interaction && !runtime.interaction_available()) {
            return false;
        }
        let Some(mut completion) = queue.ready.pop_front() else { return false };
        drop(queue);
        if let Some(key) = completion.key {
            let mut applied = self.0.applied_revisions.lock().expect("runtime completion revisions lock");
            if applied.get(key).is_some_and(|revision| *revision >= completion.revision) {
                return true;
            }
            applied.insert(key, completion.revision);
        }
        let handle = self.downgrade();
        if completion.apply.apply_step(&mut runtime, &handle) {
            return true;
        }
        self.0.completions.lock().expect("runtime completion mailbox lock").ready.push_front(completion);
        false
    }
}

/// 🪪️ Weak address for submitting owned work and returning serial completions without retaining
/// `AppRuntime` or a mutex guard across suspension.
type AppHandle = std::sync::Weak<RuntimeMailboxInner>;

//#endregion 📮️RuntimeMailbox

//#region 🎮️AppInteractionState

const TEXT_STREAM_CAPACITY: usize = 64;

#[derive(Clone, Copy)]
struct AppTextStream {
    id: u64,
    token: ui_contract::TextIngressToken,
}

struct AppRuntime {
    atlas: FontAtlas,
    icons: IconAtlas,
    interaction: Option<AppInteractionState>,
    draw: DrawList,
    overlay: DrawList,
    pending_frame_deferred: Option<FrameDeferredCursor>,
    #[cfg(not(target_arch = "wasm32"))]
    pending_frame_maintenance_refusal: Option<FrameMaintenanceRefusal>,
    #[cfg(not(target_arch = "wasm32"))]
    plugin_modules_root: std::path::PathBuf,
    #[cfg(not(target_arch = "wasm32"))]
    native_plugin_mtimes: std::collections::HashMap<std::path::PathBuf, std::time::SystemTime>,
    #[cfg(not(target_arch = "wasm32"))]
    native_hot_swap_scan: Option<RendererIoHandle>,
    #[cfg(not(target_arch = "wasm32"))]
    native_hot_swap_modified: Option<semio_framework_os_services::NativeModifiedSet>,
    #[cfg(not(target_arch = "wasm32"))]
    native_hot_swap_cursor: usize,
    #[cfg(not(target_arch = "wasm32"))]
    native_reload_pending: bool,
}

pub(crate) struct AppInteractionState {
    shell: ShellState,
    input: InputState<ActionDescriptor>,
    theme: Theme,
    theme_dark: bool,
    last_pointer_x: f32,
    last_pointer_y: f32,
    pointer_down: bool,
    pointer_button: i16,
    modifiers: PointerModifiers,
    wheel_delta: f32,
    space_pressed: bool,
    wheel_zoom_deadline_ms: f64,
    caret_blink_at_ms: f64,
    caret_blink_visible: bool,
    text_streams: [Option<AppTextStream>; TEXT_STREAM_CAPACITY],
    text_fault: Option<String>,
    frame_fault: Option<String>,
    text_cancel_pending: bool,
    #[cfg(not(target_arch = "wasm32"))]
    last_sync_pump_ms: f64,
}

impl std::ops::Deref for AppRuntime {
    type Target = AppInteractionState;

    fn deref(&self) -> &Self::Target {
        self.interaction.as_ref().expect("runtime interaction state is worker-owned")
    }
}

impl std::ops::DerefMut for AppRuntime {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.interaction.as_mut().expect("runtime interaction state is worker-owned")
    }
}

impl AppRuntime {
    fn interaction_available(&self) -> bool {
        self.interaction.is_some()
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn submit_interaction<F, Fut>(&mut self, handle: &AppHandle, key: Option<&'static str>, work: F) -> bool
    where
        F: FnOnce(AppInteractionState) -> Fut,
        Fut: Future<Output = AppInteractionState> + Send + 'static,
    {
        let Some(interaction) = self.interaction.take() else { return false };
        let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else {
            self.interaction = Some(interaction);
            return false;
        };
        if !mailbox.reserve_interaction_future() {
            self.interaction = Some(interaction);
            return false;
        }
        mailbox.spawn_interaction_reserved(key, work(interaction));
        true
    }

    #[cfg(target_arch = "wasm32")]
    fn submit_interaction<F, Fut>(&mut self, handle: &AppHandle, key: Option<&'static str>, work: F) -> bool
    where
        F: FnOnce(AppInteractionState) -> Fut,
        Fut: Future<Output = AppInteractionState> + 'static,
    {
        let Some(interaction) = self.interaction.take() else { return false };
        let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else {
            self.interaction = Some(interaction);
            return false;
        };
        if !mailbox.reserve_interaction_future() {
            self.interaction = Some(interaction);
            return false;
        }
        mailbox.spawn_interaction_reserved(key, work(interaction));
        true
    }
}

//#endregion 🎮️AppInteractionState

const FRAME_ENGINE_PACKET_CAPACITY: usize = 256;

struct FrameEnginePackets {
    slots: Box<[Option<engine_canvas::EngineCanvasPacket>; FRAME_ENGINE_PACKET_CAPACITY]>,
    len: usize,
}

impl Default for FrameEnginePackets {
    fn default() -> Self {
        Self { slots: Box::new([const { None }; FRAME_ENGINE_PACKET_CAPACITY]), len: 0 }
    }
}

impl FrameEnginePackets {
    fn try_push(&mut self, packet: engine_canvas::EngineCanvasPacket) -> Result<(), engine_canvas::EngineCanvasPacket> {
        if self.len == FRAME_ENGINE_PACKET_CAPACITY {
            return Err(packet);
        }
        self.slots[self.len] = Some(packet);
        self.len += 1;
        Ok(())
    }

    fn pop(&mut self) -> Option<engine_canvas::EngineCanvasPacket> {
        let index = self.len.checked_sub(1)?;
        self.len = index;
        self.slots[index].take()
    }

    fn last_mut(&mut self) -> Option<&mut engine_canvas::EngineCanvasPacket> {
        self.len.checked_sub(1).and_then(|index| self.slots[index].as_mut())
    }

    fn get(&self, index: usize) -> Option<&engine_canvas::EngineCanvasPacket> {
        (index < self.len).then(|| self.slots[index].as_ref()).flatten()
    }

    fn is_empty(&self) -> bool {
        self.len == 0 && self.slots.iter().all(Option::is_none)
    }
}

pub(crate) struct AppFrameBuild {
    input: ui_wgpu::wgpu::PreparedRenderInput,
    engine_packets: FrameEnginePackets,
    generation: semio_framework_trace::Generation,
    pub(crate) cursor: SemioCursor,
    theme_dark: bool,
    fullscreen: Option<bool>,
    cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>,
    #[cfg(not(target_arch = "wasm32"))]
    job_progress: Option<kernel_runtime::JobProgressPresentationLease>,
}

struct AppFrameAfterChrome {
    resource_input: Option<ui_wgpu::wgpu::PreparedRenderInput>,
    upload_rejected: Option<ui_wgpu::wgpu::PreparedRenderUpload>,
    draw_rejected: Option<ui_wgpu::wgpu::PreparedRenderInputRejected>,
    engine_packets: Option<FrameEnginePackets>,
    deferred_actions: FrameActionOwners,
    fullscreen: Option<bool>,
    cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>,
    #[cfg(not(target_arch = "wasm32"))]
    job_progress: Option<kernel_runtime::JobProgressPresentationLease>,
    retirement: Option<AppFramePreparation>,
}

struct FrameBuildCursor {
    phase: FrameBuildPhase,
    presentation_witness: RuntimePresentationWitness,
    deferred_actions: Option<FrameActionOwners>,
    fullscreen: Option<bool>,
    previous_draw: Option<DrawList>,
    previous_overlay: Option<DrawList>,
    icon_checked: bool,
    icon_pages: Option<ui_wgpu::wgpu::PreparedAtlasPages>,
    icon_upload: Option<ui_wgpu::wgpu::PreparedRenderUpload>,
    engine_resources: Option<engine_canvas::EngineCanvasBuildContext>,
    world_resources: Option<infinite_world::world::World3dBuildContext>,
    resource_input: Option<ui_wgpu::wgpu::PreparedRenderInput>,
    input_rejected: Option<ui_wgpu::wgpu::PreparedRenderInputRejected>,
    engine_packets: FrameEnginePackets,
    engine_rejected: Option<engine_canvas::EngineCanvasPacket>,
    world_rejected: Option<infinite_world::world::World3dBuildRejected>,
    retirement: Option<AppFramePreparation>,
    chrome: shell::ShellChromeFrameCursor,
    cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>,
    #[cfg(not(target_arch = "wasm32"))]
    job_progress: Option<kernel_runtime::JobProgressPresentationLease>,
}

#[derive(Clone, Copy)]
enum FrameBuildPhase {
    Deferred,
    PreparedGpuAbandonment,
    PreparedInputAbandonment,
    PreparedJobAbandonment,
    PreparedAbandonment,
    PacketAbandonment,
    Text,
    Fullscreen,
    NativePoll,
    NativeReload,
    ThemeResolve,
    ThemeAppearance,
    Drag,
    Hover,
    InputFrame,
    WheelDeadline,
    Caret,
    TakeDraw,
    RetireDraw,
    TakeOverlay,
    RetireOverlay,
    IconAtlas,
    Tutorial,
    EngineResources,
    WorldResources,
    Chrome,
    JobProgressTake,
    JobProgressOverlay,
    ResourceInput,
    WorldTransfer,
    IconTransfer,
    EngineTransfer,
    CursorWake,
    Complete,
}

impl FrameBuildCursor {
    fn new(presentation_witness: RuntimePresentationWitness, deferred_actions: FrameActionOwners) -> Self {
        Self {
            phase: FrameBuildPhase::Deferred,
            presentation_witness,
            deferred_actions: Some(deferred_actions),
            fullscreen: None,
            previous_draw: None,
            previous_overlay: None,
            icon_checked: false,
            icon_pages: None,
            icon_upload: None,
            engine_resources: None,
            world_resources: None,
            resource_input: None,
            input_rejected: None,
            engine_packets: FrameEnginePackets::default(),
            engine_rejected: None,
            world_rejected: None,
            retirement: None,
            chrome: shell::ShellChromeFrameCursor::default(),
            cursor_wake: None,
            #[cfg(not(target_arch = "wasm32"))]
            job_progress: None,
        }
    }

    fn close_step(&mut self) -> bool {
        if let Some(rejected) = self.input_rejected.as_mut() {
            if !rejected.close_step() {
                return false;
            }
            self.input_rejected = None;
            return false;
        }
        if let Some(packet) = self.engine_rejected.as_mut() {
            if !packet.close_step() {
                return false;
            }
            self.engine_rejected = None;
            return false;
        }
        if let Some(retirement) = self.retirement.as_mut() {
            if !retirement.close_step() || !retirement.terminal_is_empty() {
                return false;
            }
            self.retirement = None;
            return false;
        }
        if let Some(rejected) = self.world_rejected.as_mut() {
            if !rejected.close_step() {
                return false;
            }
            self.world_rejected = None;
            return false;
        }
        if let Some(draw) = self.previous_draw.as_mut() {
            if !draw.retire_step() {
                return false;
            }
            self.previous_draw = None;
            return false;
        }
        if let Some(overlay) = self.previous_overlay.as_mut() {
            if !overlay.retire_step() {
                return false;
            }
            self.previous_overlay = None;
            return false;
        }
        if let Some(pages) = self.icon_pages.as_mut() {
            if !pages.close_step() || !pages.terminal_is_empty() {
                return false;
            }
            self.icon_pages = None;
            return false;
        }
        if let Some(packet) = self.engine_packets.last_mut() {
            if !packet.close_step() {
                return false;
            }
            self.engine_packets.pop();
            return false;
        }
        if let Some(resources) = self.engine_resources.as_mut() {
            match resources.take_packet_step() {
                Ok(Some(packet)) | Err(packet) => {
                    if let Err(rejected) = self.engine_packets.try_push(packet) {
                        self.engine_rejected = Some(rejected);
                    }
                }
                Ok(None) => self.engine_resources = None,
            }
            return false;
        }
        if self.resource_input.is_none() && (self.world_resources.is_some() || self.icon_upload.is_some()) {
            match ui_wgpu::wgpu::PreparedRenderInput::try_new(self.presentation_witness.scene_revision, self.presentation_witness.input_generation, ui_wgpu::wgpu::DrawList::empty(), None, 0.0) {
                Ok(input) => self.resource_input = Some(input),
                Err(rejected) => self.input_rejected = Some(rejected),
            }
            return false;
        }
        if let Some(resources) = self.world_resources.as_mut() {
            let Some(input) = self.resource_input.as_mut() else { return false };
            match resources.append_step(input) {
                Ok(true) => match resources.take_cursor_wake() {
                    Ok(token) => {
                        self.cursor_wake = token;
                        self.world_resources = None;
                    }
                    Err(_) => return false,
                },
                Ok(false) => {}
                Err(rejected) => self.world_rejected = Some(rejected),
            }
            return false;
        }
        if let Some(upload) = self.icon_upload.take() {
            let Some(input) = self.resource_input.as_mut() else {
                self.icon_upload = Some(upload);
                return false;
            };
            if let Err(upload) = input.try_push_upload(upload) {
                self.world_rejected = Some(infinite_world::world::World3dBuildRejected::Upload(upload));
            }
            return false;
        }
        if let Some(input) = self.resource_input.take() {
            self.retirement = Some(
                AppFrameBuild {
                    generation: semio_framework_trace::Generation(input.preview_generation),
                    input,
                    engine_packets: std::mem::take(&mut self.engine_packets),
                    cursor: SemioCursor::Default,
                    theme_dark: false,
                    fullscreen: self.fullscreen.take(),
                    cursor_wake: self.cursor_wake.take(),
                    #[cfg(not(target_arch = "wasm32"))]
                    job_progress: self.job_progress.take(),
                }
                .into_preparation(),
            );
            return false;
        }
        self.cursor_wake.take();
        self.deferred_actions.as_mut().is_none_or(|actions| actions.pop_front().is_none())
    }

    fn terminal_is_empty(&self) -> bool {
        self.previous_draw.is_none()
            && self.previous_overlay.is_none()
            && self.engine_rejected.is_none()
            && self.world_rejected.is_none()
            && self.engine_packets.is_empty()
            && self.engine_resources.is_none()
            && self.world_resources.is_none()
            && self.resource_input.is_none()
            && self.input_rejected.is_none()
            && self.icon_upload.is_none()
            && self.icon_pages.is_none()
            && self.retirement.is_none()
            && self.cursor_wake.is_none()
            && self.deferred_actions.as_ref().is_none_or(FrameActionOwners::is_empty)
    }
}

struct FrameFinishCursor {
    phase: FrameFinishPhase,
    cursor: SemioCursor,
    pump_sync: bool,
    flush_tutorial: bool,
    shell_maintenance: bool,
    deferred_actions: Option<FrameActionOwners>,
    glyph_started: bool,
    glyph_pages: Option<ui_wgpu::wgpu::PreparedAtlasPages>,
}

#[derive(Clone, Copy)]
enum FrameFinishPhase {
    Inputs,
    Deferred,
    GlyphUpload,
    Cursor,
    Draw,
    Overlay,
    Complete,
}

impl Default for FrameFinishCursor {
    fn default() -> Self {
        Self { phase: FrameFinishPhase::Inputs, cursor: SemioCursor::Default, pump_sync: false, flush_tutorial: false, shell_maintenance: false, deferred_actions: None, glyph_started: false, glyph_pages: None }
    }
}

impl FrameFinishCursor {
    fn close_step(&mut self) -> bool {
        if let Some(pages) = self.glyph_pages.as_mut() {
            if !pages.close_step() || !pages.terminal_is_empty() {
                return false;
            }
            self.glyph_pages = None;
            return false;
        }
        self.deferred_actions.as_mut().is_none_or(|actions| actions.pop_front().is_none())
    }

    fn terminal_is_empty(&self) -> bool {
        self.glyph_pages.is_none() && self.deferred_actions.as_ref().is_none_or(FrameActionOwners::is_empty)
    }
}

enum FrameBuildBoundaryStep {
    Pending,
    Complete(AppFrameAfterChrome),
    Fault(&'static str),
}

enum FrameFinishBoundaryStep {
    Pending,
    Complete(AppFrameBuild),
    Fault(&'static str),
}

struct FrameWheelCursor {
    delta: f32,
    x: f32,
    y: f32,
    ctrl: bool,
    index: usize,
}

impl AppFrameAfterChrome {
    fn close_step(&mut self) -> bool {
        if let Some(rejected) = self.draw_rejected.as_mut() {
            if !rejected.close_step() {
                return false;
            }
            self.draw_rejected = None;
            return false;
        }
        if let Some(upload) = self.upload_rejected.as_mut() {
            if !upload.close_step() {
                return false;
            }
            self.upload_rejected = None;
            return false;
        }
        if self.deferred_actions.pop_front().is_some() {
            return false;
        }
        if self.retirement.is_none() {
            let Some(input) = self.resource_input.take() else {
                if self.cursor_wake.take().is_some() {
                    return false;
                }
                return true;
            };
            let build = AppFrameBuild {
                generation: semio_framework_trace::Generation(input.preview_generation),
                input,
                engine_packets: match self.engine_packets.take() {
                    Some(packets) => packets,
                    None => return false,
                },
                cursor: SemioCursor::Default,
                theme_dark: false,
                fullscreen: self.fullscreen.take(),
                cursor_wake: self.cursor_wake.take(),
                #[cfg(not(target_arch = "wasm32"))]
                job_progress: self.job_progress.take(),
            };
            self.retirement = Some(build.into_preparation());
            return false;
        }
        let Some(retirement) = self.retirement.as_mut() else { return false };
        if !retirement.close_step() || !retirement.terminal_is_empty() {
            return false;
        }
        self.retirement = None;
        true
    }
}

pub(crate) struct FrameTransaction {
    directives: Option<crate::frame_job::FrameDirectives>,
    operation: semio_framework_trace::OperationId,
    generation: semio_framework_trace::Generation,
    base_witness: Option<RuntimePresentationWitness>,
    effect_opportunities: u32,
    dpr: f32,
    stage: FrameTransactionStage,
    phase: AppFrameTransactionPhase,
    board_authority_cursor: usize,
    world3d_authority_cursor: usize,
    scene_camera_cursor: scenes::SceneCameraDispatchCursor,
    deferred_actions: FrameActionOwners,
    build_cursor: Option<FrameBuildCursor>,
    finish_cursor: Option<FrameFinishCursor>,
    after_chrome: Option<AppFrameAfterChrome>,
    wheel: Option<FrameWheelCursor>,
    raster_uploads: Option<scenes::PendingRasterUploadCursor>,
    raster_rejected: Option<ui_wgpu::wgpu::PreparedRasterProducer>,
}

pub(crate) enum AppFrameTransactionStep {
    Pending,
    Complete(AppFrameBuild),
    Fault,
}

/// 🧭️ The mounted frame's seven monotonic publication stages.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum FrameTransactionStage {
    DrainProjectionDeltas,
    RouteIntents,
    FlushEffects,
    PresentSurface,
    ReconcileTree,
    BuildRenderPackets,
    PublishSnapshot,
}

enum AppFrameTransactionPhase {
    SceneCamera,
    Build,
    InputEvents,
    FrameDeferred,
    BoardAuthority,
    World3dSnapshot,
    World3dAuthority,
    WheelStart,
    WheelWorld3d,
    WheelGraph,
    WheelMap,
    WheelBoard,
    RasterUploads,
    Finish,
    Terminal,
}

impl FrameTransaction {
    pub(crate) fn new(directives: crate::frame_job::FrameDirectives, operation: semio_framework_trace::OperationId, generation: semio_framework_trace::Generation, dpr: f32) -> Self {
        Self {
            directives: Some(directives),
            operation,
            generation,
            base_witness: None,
            effect_opportunities: 0,
            dpr,
            stage: FrameTransactionStage::DrainProjectionDeltas,
            phase: AppFrameTransactionPhase::SceneCamera,
            board_authority_cursor: 0,
            world3d_authority_cursor: 0,
            scene_camera_cursor: scenes::SceneCameraDispatchCursor::begin(app_now_ms()),
            deferred_actions: FrameActionOwners::default(),
            build_cursor: None,
            finish_cursor: None,
            after_chrome: None,
            wheel: None,
            raster_uploads: None,
            raster_rejected: None,
        }
    }

    pub(crate) fn step(&mut self, runtime: &RuntimeMailbox, handle: &AppHandle, context: &mut semio_framework_job::StepContext<'_>) -> AppFrameTransactionStep {
        context.set_stage(self.stage_label());
        if context.operation() != self.operation || context.generation() != self.generation || context.is_cancelled() || context.deadline_exceeded() {
            runtime.record_frame_fault("frame opportunity freshness guard refused operation, generation, cancellation, or deadline");
            self.phase = AppFrameTransactionPhase::Terminal;
            return AppFrameTransactionStep::Fault;
        }
        let Some(current_witness) = runtime.presentation_witness_for(self.generation.0) else {
            runtime.record_frame_fault("frame opportunity input generation was superseded");
            self.phase = AppFrameTransactionPhase::Terminal;
            return AppFrameTransactionStep::Fault;
        };
        if let Some(base_witness) = self.base_witness {
            if base_witness != current_witness {
                runtime.record_frame_fault("frame opportunity base revision was superseded");
                self.phase = AppFrameTransactionPhase::Terminal;
                return AppFrameTransactionStep::Fault;
            }
        } else {
            self.base_witness = Some(current_witness);
        }
        if self.stage == FrameTransactionStage::FlushEffects {
            let Some(next) = self.effect_opportunities.checked_add(1) else {
                runtime.record_frame_fault("frame effect opportunity counter exhausted");
                self.phase = AppFrameTransactionPhase::Terminal;
                return AppFrameTransactionStep::Fault;
            };
            if next > EFFECT_STORM_BUDGET {
                runtime.record_frame_fault("frame effect storm budget exhausted");
                self.phase = AppFrameTransactionPhase::Terminal;
                return AppFrameTransactionStep::Fault;
            }
            self.effect_opportunities = next;
        }
        let Some(directives) = self.directives.as_ref() else { return AppFrameTransactionStep::Pending };
        if context.should_yield() {
            return AppFrameTransactionStep::Pending;
        }
        if retire_cancelled_renderer_asset_step() {
            context.consume_fuel(1);
            return AppFrameTransactionStep::Pending;
        }
        if runtime.pump_renderer_asset_decode_step() {
            context.consume_fuel(1);
            return AppFrameTransactionStep::Pending;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if runtime.pump_native_asset() {
            context.consume_fuel(1);
            return AppFrameTransactionStep::Pending;
        }
        let Ok(mut app) = runtime.try_lock() else { return AppFrameTransactionStep::Pending };
        if !app.interaction_available() {
            return AppFrameTransactionStep::Pending;
        }
        match self.phase {
            AppFrameTransactionPhase::SceneCamera => match self.scene_camera_cursor.step() {
                scenes::SceneCameraDispatchStep::Pending => AppFrameTransactionStep::Pending,
                scenes::SceneCameraDispatchStep::Action(action) => {
                    if let Err(_action) = self.deferred_actions.try_push(action) {
                        runtime.record_frame_fault("frame deferred action credits exceeded");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                    AppFrameTransactionStep::Pending
                }
                scenes::SceneCameraDispatchStep::Complete => {
                    self.stage = FrameTransactionStage::RouteIntents;
                    self.phase = AppFrameTransactionPhase::Build;
                    AppFrameTransactionStep::Pending
                }
                scenes::SceneCameraDispatchStep::Fault(fault) => {
                    runtime.record_frame_fault(fault);
                    self.phase = AppFrameTransactionPhase::Terminal;
                    AppFrameTransactionStep::Fault
                }
            },
            AppFrameTransactionPhase::Build => {
                if self.build_cursor.is_none() {
                    let Some(presentation_witness) = runtime.presentation_witness_for(self.generation.0) else {
                        runtime.record_frame_fault("frame presentation input generation was stale before candidate construction");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    };
                    self.build_cursor = Some(FrameBuildCursor::new(presentation_witness, std::mem::take(&mut self.deferred_actions)));
                    return AppFrameTransactionStep::Pending;
                }
                let Some(cursor) = self.build_cursor.as_mut() else { return AppFrameTransactionStep::Pending };
                match app.frame_before_input_step(handle, directives, self.dpr, cursor) {
                    FrameBuildBoundaryStep::Pending => AppFrameTransactionStep::Pending,
                    FrameBuildBoundaryStep::Complete(partial) => {
                        self.build_cursor = None;
                        self.after_chrome = Some(partial);
                        self.stage = FrameTransactionStage::RouteIntents;
                        self.phase = AppFrameTransactionPhase::InputEvents;
                        AppFrameTransactionStep::Pending
                    }
                    FrameBuildBoundaryStep::Fault(fault) => {
                        runtime.record_frame_fault(fault);
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                }
            }
            AppFrameTransactionPhase::InputEvents => {
                let action = match app.input.take_action_step() {
                    Ok(action) => action,
                    Err(_) => {
                        runtime.record_frame_fault("bounded frame input action authority faulted");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                };
                if let Some(action) = action {
                    let Some(partial) = self.after_chrome.as_mut() else {
                        runtime.record_frame_fault("frame input phase lost the retained chrome owner");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    };
                    let Ok(action) = action.into_descriptor() else {
                        runtime.record_frame_fault("bounded frame input action failed materialization");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    };
                    if let Err(_action) = partial.deferred_actions.try_push(action) {
                        runtime.record_frame_fault("frame input action credits exceeded");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                    return AppFrameTransactionStep::Pending;
                }
                if crate::interpreter::drive_scene_interaction_step(&mut app.input) {
                    return AppFrameTransactionStep::Pending;
                }
                self.stage = FrameTransactionStage::FlushEffects;
                self.phase = AppFrameTransactionPhase::FrameDeferred;
                AppFrameTransactionStep::Pending
            }
            AppFrameTransactionPhase::FrameDeferred => {
                let mut deferred = app.pending_frame_deferred.take();
                if deferred.is_some() && !RuntimeApply::start_frame_deferred(&mut deferred, &mut app, handle) {
                    app.pending_frame_deferred = deferred;
                    return AppFrameTransactionStep::Pending;
                }
                self.phase = AppFrameTransactionPhase::BoardAuthority;
                AppFrameTransactionStep::Pending
            }
            AppFrameTransactionPhase::BoardAuthority => {
                let Some(interaction) = app.interaction.as_mut() else { return AppFrameTransactionStep::Pending };
                let Some(surface_id) = interaction.shell.board2d_states.id_at(self.board_authority_cursor) else {
                    self.board_authority_cursor = 0;
                    self.phase = AppFrameTransactionPhase::World3dSnapshot;
                    return AppFrameTransactionStep::Pending;
                };
                if surface_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    runtime.record_frame_fault("board authority surface identifier exceeded fixed credits");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                }
                let surface_id = surface_id.to_owned();
                let Some(surface) = interaction.shell.board2d_states.get(&surface_id) else {
                    runtime.record_frame_fault("board authority surface order lost ownership");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                if surface.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    runtime.record_frame_fault("board authority controller identifier exceeded fixed credits");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                }
                let controller_id = surface.controller_id.clone();
                match engine_canvas::drive_board_authority_step(&surface_id, context) {
                    puzzle::editor::puzzle2d::engine::BoardAuthorityStep::Pending => AppFrameTransactionStep::Pending,
                    puzzle::editor::puzzle2d::engine::BoardAuthorityStep::Cancelled => {
                        if let Err(fault) = engine_canvas::release_board_pointer_claim(&surface_id, &mut app.input) {
                            app.input.record_action_fault(fault);
                            runtime.record_frame_fault("board cancelled claim release faulted");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        }
                        self.board_authority_cursor += 1;
                        AppFrameTransactionStep::Pending
                    }
                    puzzle::editor::puzzle2d::engine::BoardAuthorityStep::Fault => {
                        let _ = engine_canvas::release_board_pointer_claim(&surface_id, &mut app.input);
                        runtime.record_frame_fault("board retained authority faulted");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                    puzzle::editor::puzzle2d::engine::BoardAuthorityStep::Complete => match engine_canvas::publish_board_pointer_step(&surface_id, &mut app.input) {
                        Ok(true) => {
                            self.phase = AppFrameTransactionPhase::InputEvents;
                            AppFrameTransactionStep::Pending
                        }
                        Ok(false) => match engine_canvas::publish_board_event_step(&surface_id, &controller_id, &mut app.input) {
                            Ok(true) => {
                                self.phase = AppFrameTransactionPhase::InputEvents;
                                AppFrameTransactionStep::Pending
                            }
                            Ok(false) => {
                                self.board_authority_cursor += 1;
                                AppFrameTransactionStep::Pending
                            }
                            Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) => {
                                self.phase = AppFrameTransactionPhase::InputEvents;
                                AppFrameTransactionStep::Pending
                            }
                            Err(fault) => {
                                app.input.record_action_fault(fault);
                                runtime.record_frame_fault("board event publication faulted");
                                self.phase = AppFrameTransactionPhase::Terminal;
                                AppFrameTransactionStep::Fault
                            }
                        },
                        Err(ui_wgpu::wgpu::BoundedActionFault::ItemCredits | ui_wgpu::wgpu::BoundedActionFault::ByteCredits) => {
                            self.phase = AppFrameTransactionPhase::InputEvents;
                            AppFrameTransactionStep::Pending
                        }
                        Err(fault) => {
                            app.input.record_action_fault(fault);
                            runtime.record_frame_fault("board retained publication faulted");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            AppFrameTransactionStep::Fault
                        }
                    },
                }
            }
            AppFrameTransactionPhase::World3dSnapshot => {
                let Some(interaction) = app.interaction.as_mut() else {
                    return AppFrameTransactionStep::Pending;
                };
                let Some(surface_id) = interaction.shell.world3d_states.id_at(self.world3d_authority_cursor) else {
                    self.world3d_authority_cursor = 0;
                    self.phase = AppFrameTransactionPhase::World3dAuthority;
                    return AppFrameTransactionStep::Pending;
                };
                if surface_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    runtime.record_frame_fault("world3d snapshot surface identifier exceeded fixed credits");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                }
                let surface_id = surface_id.to_owned();
                let Some(state) = interaction.shell.world3d_states.get_mut(&surface_id) else {
                    runtime.record_frame_fault("world3d snapshot surface order lost ownership");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                if retire_cancelled_world3d_asset_step(state) {
                    context.consume_fuel(1);
                    return AppFrameTransactionStep::Pending;
                }
                match step_world3d_draw_rebuild(state, context) {
                    WorldDrawRebuildStep::Pending => return AppFrameTransactionStep::Pending,
                    WorldDrawRebuildStep::Stale | WorldDrawRebuildStep::Fault => {
                        runtime.record_frame_fault("world3d retained draw rebuild faulted");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                    WorldDrawRebuildStep::Complete => {}
                }
                match step_world3d_snapshot(state, context) {
                    World3dSnapshotApplyStep::Idle | World3dSnapshotApplyStep::Complete => {
                        self.world3d_authority_cursor += 1;
                        AppFrameTransactionStep::Pending
                    }
                    World3dSnapshotApplyStep::Pending => AppFrameTransactionStep::Pending,
                    World3dSnapshotApplyStep::Stale | World3dSnapshotApplyStep::Fault => {
                        runtime.record_frame_fault("world3d typed snapshot apply faulted");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                }
            }
            AppFrameTransactionPhase::World3dAuthority => {
                let Some(interaction) = app.interaction.as_mut() else { return AppFrameTransactionStep::Pending };
                let Some(surface_id) = interaction.shell.world3d_states.id_at(self.world3d_authority_cursor) else {
                    self.world3d_authority_cursor = 0;
                    self.stage = FrameTransactionStage::PresentSurface;
                    self.phase = AppFrameTransactionPhase::WheelStart;
                    return AppFrameTransactionStep::Pending;
                };
                if surface_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                    runtime.record_frame_fault("world3d authority surface identifier exceeded fixed credits");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                }
                let surface_id = surface_id.to_owned();
                let AppInteractionState { shell, input, .. } = interaction;
                let Some(state) = shell.world3d_states.get_mut(&surface_id) else {
                    runtime.record_frame_fault("world3d authority surface order lost ownership");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                let Some(generation) = world3d_interaction_front_generation(state) else {
                    self.world3d_authority_cursor += 1;
                    return AppFrameTransactionStep::Pending;
                };
                match step_world3d_interaction(state, generation, input, context) {
                    WorldInteractionAuthorityStep::Idle | WorldInteractionAuthorityStep::Complete | WorldInteractionAuthorityStep::Stale => {
                        self.world3d_authority_cursor += 1;
                        self.phase = AppFrameTransactionPhase::InputEvents;
                        AppFrameTransactionStep::Pending
                    }
                    WorldInteractionAuthorityStep::Pending => AppFrameTransactionStep::Pending,
                    WorldInteractionAuthorityStep::OutputBlocked => {
                        self.phase = AppFrameTransactionPhase::InputEvents;
                        AppFrameTransactionStep::Pending
                    }
                    WorldInteractionAuthorityStep::Fault => {
                        runtime.record_frame_fault("world3d retained interaction authority faulted");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                }
            }
            AppFrameTransactionPhase::WheelStart => {
                let delta = app.wheel_delta;
                app.wheel_delta = 0.0;
                if delta.abs() == 0.0 {
                    self.stage = FrameTransactionStage::ReconcileTree;
                    self.phase = AppFrameTransactionPhase::RasterUploads;
                    return AppFrameTransactionStep::Pending;
                }
                let x = app.last_pointer_x;
                let y = app.last_pointer_y;
                let ctrl = app.modifiers.ctrl;
                let Some(interaction) = app.interaction.as_mut() else { return AppFrameTransactionStep::Pending };
                interaction.shell.handle_pointer_wheel(x, y, delta, &interaction.input);
                if !ShellState::wheel_propagates_to_scene_surface(interaction.input.hit_at(x, y)) {
                    self.stage = FrameTransactionStage::ReconcileTree;
                    self.phase = AppFrameTransactionPhase::RasterUploads;
                    return AppFrameTransactionStep::Pending;
                }
                let surface_fault =
                    interaction.shell.world3d_states.take_fault().or_else(|| interaction.shell.node_graph_states.take_fault()).or_else(|| interaction.shell.tiled_map_states.take_fault()).or_else(|| interaction.shell.board2d_states.take_fault());
                if let Some(fault) = surface_fault {
                    runtime.record_frame_fault(fault);
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                }
                self.wheel = Some(FrameWheelCursor { delta, x, y, ctrl, index: 0 });
                self.phase = AppFrameTransactionPhase::WheelWorld3d;
                AppFrameTransactionStep::Pending
            }
            AppFrameTransactionPhase::WheelWorld3d => {
                let Some(wheel) = self.wheel.as_mut() else {
                    runtime.record_frame_fault("world3d wheel phase lost its retained cursor");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                let Some(interaction) = app.interaction.as_mut() else { return AppFrameTransactionStep::Pending };
                let Some(surface_id) = interaction.shell.world3d_states.id_at(wheel.index).map(str::to_owned) else {
                    wheel.index = 0;
                    self.phase = AppFrameTransactionPhase::WheelGraph;
                    return AppFrameTransactionStep::Pending;
                };
                wheel.index += 1;
                let Some(state) = interaction.shell.world3d_states.get_mut(&surface_id) else {
                    runtime.record_frame_fault("world3d surface order lost ownership");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                if state.bounds.contains(wheel.x, wheel.y) {
                    if state.surface_id.len() > WORLD3D_DEADLINE_ID_BYTES || state.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                        runtime.record_frame_fault("world3d wheel identifier exceeded fixed credits");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                    let modifiers = PointerModifiers { ctrl: wheel.ctrl, ..PointerModifiers::default() };
                    if enqueue_world3d_event(state, WorldInteractionIntent::wheel(wheel.x, wheel.y, wheel.delta, &modifiers)).is_err() {
                        runtime.record_frame_fault("world3d wheel intent credits exceeded");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                }
                AppFrameTransactionStep::Pending
            }
            AppFrameTransactionPhase::WheelGraph => {
                let Some(wheel) = self.wheel.as_mut() else {
                    runtime.record_frame_fault("graph wheel phase lost its retained cursor");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                let Some(interaction) = app.interaction.as_mut() else { return AppFrameTransactionStep::Pending };
                let Some(surface_id) = interaction.shell.node_graph_states.id_at(wheel.index).map(str::to_owned) else {
                    wheel.index = 0;
                    self.phase = AppFrameTransactionPhase::WheelMap;
                    return AppFrameTransactionStep::Pending;
                };
                wheel.index += 1;
                let Some(surface) = interaction.shell.node_graph_states.get(&surface_id) else {
                    runtime.record_frame_fault("node graph surface order lost ownership");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                if surface.bounds.contains(wheel.x, wheel.y) {
                    if surface_id.len() > WORLD3D_DEADLINE_ID_BYTES || surface.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                        runtime.record_frame_fault("node graph wheel identifier exceeded fixed credits");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                    if let Err(fault) = engine_canvas::node_graph_wheel_into(&surface_id, &surface.controller_id, surface.bounds, wheel.x, wheel.y, wheel.delta, wheel.ctrl, &mut app.input) {
                        app.input.record_action_fault(fault);
                        runtime.record_frame_fault("node graph wheel action admission failed");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                    app.wheel_zoom_deadline_ms = app_now_ms() + 120.0;
                }
                AppFrameTransactionStep::Pending
            }
            AppFrameTransactionPhase::WheelMap => {
                let Some(wheel) = self.wheel.as_mut() else {
                    runtime.record_frame_fault("map wheel phase lost its retained cursor");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                let Some(interaction) = app.interaction.as_mut() else { return AppFrameTransactionStep::Pending };
                let Some(surface_id) = interaction.shell.tiled_map_states.id_at(wheel.index).map(str::to_owned) else {
                    wheel.index = 0;
                    self.phase = AppFrameTransactionPhase::WheelBoard;
                    return AppFrameTransactionStep::Pending;
                };
                wheel.index += 1;
                let Some(surface) = interaction.shell.tiled_map_states.get(&surface_id) else {
                    runtime.record_frame_fault("tiled map surface order lost ownership");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                if surface.bounds.contains(wheel.x, wheel.y) {
                    if surface_id.len() > WORLD3D_DEADLINE_ID_BYTES || surface.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                        runtime.record_frame_fault("tiled map wheel identifier exceeded fixed credits");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                    if let Err(fault) = engine_canvas::tiled_map_wheel_into(&surface_id, &surface.controller_id, surface.bounds, wheel.x, wheel.y, wheel.delta, wheel.ctrl, &mut app.input) {
                        app.input.record_action_fault(fault);
                        runtime.record_frame_fault("tiled map wheel action admission failed");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                }
                AppFrameTransactionStep::Pending
            }
            AppFrameTransactionPhase::WheelBoard => {
                let Some(wheel) = self.wheel.as_mut() else {
                    runtime.record_frame_fault("board wheel phase lost its retained cursor");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                let Some(interaction) = app.interaction.as_mut() else { return AppFrameTransactionStep::Pending };
                let Some(surface_id) = interaction.shell.board2d_states.id_at(wheel.index).map(str::to_owned) else {
                    self.wheel = None;
                    self.stage = FrameTransactionStage::ReconcileTree;
                    self.phase = AppFrameTransactionPhase::RasterUploads;
                    return AppFrameTransactionStep::Pending;
                };
                wheel.index += 1;
                let Some(surface) = interaction.shell.board2d_states.get(&surface_id) else {
                    runtime.record_frame_fault("board surface order lost ownership");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                if surface.bounds.contains(wheel.x, wheel.y) {
                    if surface_id.len() > WORLD3D_DEADLINE_ID_BYTES || surface.controller_id.len() > WORLD3D_DEADLINE_ID_BYTES {
                        runtime.record_frame_fault("board wheel identifier exceeded fixed credits");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                    if let Err(fault) = scenes::puzzle_board_wheel_into(&surface_id, &surface.controller_id, surface.bounds, wheel.x, wheel.y, wheel.delta, &mut app.input) {
                        app.input.record_action_fault(fault);
                        runtime.record_frame_fault("board wheel action admission failed");
                        self.phase = AppFrameTransactionPhase::Terminal;
                        return AppFrameTransactionStep::Fault;
                    }
                }
                AppFrameTransactionStep::Pending
            }
            AppFrameTransactionPhase::RasterUploads => {
                let cursor = self.raster_uploads.get_or_insert_with(Default::default);
                match cursor.step() {
                    scenes::PendingRasterUploadStep::Pending => AppFrameTransactionStep::Pending,
                    scenes::PendingRasterUploadStep::Upload(checked_out) => {
                        let Ok(mut producer) = checked_out.take() else {
                            runtime.record_frame_fault("frame raster producer checkout was stale");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        };
                        let Some(partial) = self.after_chrome.as_mut() else {
                            runtime.record_frame_fault("raster upload phase lost the retained chrome owner");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        };
                        let Some(input) = partial.resource_input.as_mut() else {
                            runtime.record_frame_fault("raster upload phase lost the retained resource input");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        };
                        let Some(upload_items) = input.raster_producers.len().checked_add(input.uploads.len()) else {
                            if let Err(mut returned) = cursor.retain_rejected(producer) {
                                returned.begin_close();
                                self.raster_rejected = Some(returned);
                            }
                            runtime.record_frame_fault("frame raster upload item credits exhausted");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        };
                        if upload_items >= input.limits.max_upload_items {
                            if let Err(mut returned) = cursor.retain_rejected(producer) {
                                returned.begin_close();
                                self.raster_rejected = Some(returned);
                            }
                            runtime.record_frame_fault("frame raster upload item credits exceeded");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        }
                        if !producer.bind_frame_generation(input.preview_generation) {
                            if let Err(mut returned) = cursor.retain_rejected(producer) {
                                returned.begin_close();
                                self.raster_rejected = Some(returned);
                            }
                            runtime.record_frame_fault("frame raster producer generation was stale");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        }
                        if let Err(producer) = input.try_push_raster_producer(producer) {
                            if let Err(mut returned) = cursor.retain_rejected(producer) {
                                returned.begin_close();
                                self.raster_rejected = Some(returned);
                            }
                            runtime.record_frame_fault("frame raster producer fixed admission was refused");
                            self.phase = AppFrameTransactionPhase::Terminal;
                            return AppFrameTransactionStep::Fault;
                        }
                        AppFrameTransactionStep::Pending
                    }
                    scenes::PendingRasterUploadStep::Complete => {
                        self.raster_uploads = None;
                        self.stage = FrameTransactionStage::BuildRenderPackets;
                        self.phase = AppFrameTransactionPhase::Finish;
                        AppFrameTransactionStep::Pending
                    }
                    scenes::PendingRasterUploadStep::Fault(fault) => {
                        runtime.record_frame_fault(fault);
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                }
            }
            AppFrameTransactionPhase::Finish => {
                let Some(partial) = self.after_chrome.as_mut() else {
                    runtime.record_frame_fault("frame finish lost the retained chrome owner");
                    self.phase = AppFrameTransactionPhase::Terminal;
                    return AppFrameTransactionStep::Fault;
                };
                let cursor = self.finish_cursor.get_or_insert_with(Default::default);
                match app.frame_after_input_step(runtime, partial, cursor, context.cancel_token()) {
                    FrameFinishBoundaryStep::Pending => AppFrameTransactionStep::Pending,
                    FrameFinishBoundaryStep::Complete(frame) => {
                        self.after_chrome = None;
                        self.finish_cursor = None;
                        self.stage = FrameTransactionStage::PublishSnapshot;
                        self.phase = AppFrameTransactionPhase::Terminal;
                        self.directives = None;
                        AppFrameTransactionStep::Complete(frame)
                    }
                    FrameFinishBoundaryStep::Fault(fault) => {
                        runtime.record_frame_fault(fault);
                        self.phase = AppFrameTransactionPhase::Terminal;
                        AppFrameTransactionStep::Fault
                    }
                }
            }
            AppFrameTransactionPhase::Terminal => AppFrameTransactionStep::Pending,
        }
    }

    pub(crate) fn close_step(&mut self) -> bool {
        if let Some(producer) = self.raster_rejected.as_mut() {
            if !producer.close_step() {
                return false;
            }
            self.raster_rejected = None;
            return false;
        }
        if let Some(cursor) = self.raster_uploads.as_mut() {
            if !cursor.close_step() {
                return false;
            }
            self.raster_uploads = None;
            return false;
        }
        if let Some(cursor) = self.build_cursor.as_mut() {
            if !cursor.close_step() || !cursor.terminal_is_empty() {
                return false;
            }
            self.build_cursor = None;
            return false;
        }
        if let Some(cursor) = self.finish_cursor.as_mut() {
            if !cursor.close_step() || !cursor.terminal_is_empty() {
                return false;
            }
            self.finish_cursor = None;
            return false;
        }
        self.wheel = None;
        if let Some(partial) = self.after_chrome.as_mut() {
            if !partial.close_step() {
                return false;
            }
            self.after_chrome = None;
            return false;
        }
        if !self.scene_camera_cursor.terminal_is_empty() {
            self.scene_camera_cursor.close_step();
            return false;
        }
        if self.deferred_actions.pop_front().is_some() {
            return false;
        }
        let Some(directives) = self.directives.as_mut() else { return true };
        if !directives.close_step() {
            return false;
        }
        self.directives = None;
        true
    }

    pub(crate) fn stage(&self) -> FrameTransactionStage {
        self.stage
    }

    fn stage_label(&self) -> &'static str {
        match self.stage {
            FrameTransactionStage::DrainProjectionDeltas => "DrainProjectionDeltas",
            FrameTransactionStage::RouteIntents => "RouteIntents",
            FrameTransactionStage::FlushEffects => "FlushEffects",
            FrameTransactionStage::PresentSurface => "PresentSurface",
            FrameTransactionStage::ReconcileTree => "ReconcileTree",
            FrameTransactionStage::BuildRenderPackets => "BuildRenderPackets",
            FrameTransactionStage::PublishSnapshot => "PublishSnapshot",
        }
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.directives.is_none()
            && self.build_cursor.is_none()
            && self.finish_cursor.is_none()
            && self.after_chrome.is_none()
            && self.wheel.is_none()
            && self.raster_uploads.is_none()
            && self.raster_rejected.is_none()
            && self.scene_camera_cursor.terminal_is_empty()
            && self.deferred_actions.is_empty()
    }
}

pub(crate) struct AppFramePresentation {
    packet: Option<ui_wgpu::wgpu::PreparedRenderPacket>,
    engine_packets: FrameEnginePackets,
    generation: semio_framework_trace::Generation,
    pub(crate) cursor: SemioCursor,
    theme_dark: bool,
    fullscreen: Option<bool>,
    cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>,
    #[cfg(not(target_arch = "wasm32"))]
    job_progress: Option<kernel_runtime::JobProgressPresentationLease>,
}

impl AppFrameBuild {
    pub(crate) fn into_preparation(self) -> AppFramePreparation {
        let AppFrameBuild {
            input,
            engine_packets,
            generation,
            cursor,
            theme_dark,
            fullscreen,
            cursor_wake,
            #[cfg(not(target_arch = "wasm32"))]
            job_progress,
        } = self;
        let (job, job_rejected) = match ui_wgpu::wgpu::PreparedRenderJob::try_new(input) {
            Ok(job) => (Some(job), None),
            Err(rejected) => (None, Some(rejected)),
        };
        AppFramePreparation {
            job,
            job_rejected,
            session: None,
            rejected: None,
            engine_packets: Some(engine_packets),
            generation,
            cursor,
            theme_dark,
            fullscreen,
            cursor_wake,
            #[cfg(not(target_arch = "wasm32"))]
            job_progress,
            terminal: false,
        }
    }
}

pub(crate) struct AppFramePreparation {
    job: Option<ui_wgpu::wgpu::PreparedRenderJob>,
    job_rejected: Option<ui_wgpu::wgpu::PreparedRenderJobRejected>,
    session: Option<semio_framework_job::BatchJobSession<ui_wgpu::wgpu::PreparedRenderJob>>,
    rejected: Option<semio_framework_job::WorkerJobSessionAdmissionRejected<ui_wgpu::wgpu::PreparedRenderJob>>,
    engine_packets: Option<FrameEnginePackets>,
    generation: semio_framework_trace::Generation,
    cursor: SemioCursor,
    theme_dark: bool,
    fullscreen: Option<bool>,
    cursor_wake: Option<infinite_world::world::WorldCursorWakeToken>,
    #[cfg(not(target_arch = "wasm32"))]
    job_progress: Option<kernel_runtime::JobProgressPresentationLease>,
    terminal: bool,
}

impl AppFramePreparation {
    pub(crate) fn callback_verdict(&self) -> Option<&semio_framework_trace::CallbackVerdict> {
        self.session.as_ref().and_then(semio_framework_job::BatchJobSession::callback_verdict)
    }

    pub(crate) fn drive_step(&mut self, operation: semio_framework_job::OperationId, generation: semio_framework_job::Generation, cancel: semio_framework_job::CancelToken, _preview_sequence: &mut u64) -> semio_framework_job::StepOutcome {
        if let Some(job) = self.job.take() {
            let params = semio_framework_job::BatchJobParams {
                operation,
                generation,
                cancel,
                config: semio_framework_job::BatchDriveConfig { site: "os_renderer.prepare.worker", stage: semio_framework_job::InteractiveStage::BackgroundStep, fuel_per_step: 1, step_budget_us: 1000 },
                now_us: semio_framework_job::default_now_us,
            };
            match semio_framework_job::BatchJobSession::try_new(job, params) {
                Ok(session) => self.session = Some(session),
                Err(mut rejected) => {
                    rejected.begin_close();
                    self.rejected = Some(rejected);
                }
            }
            return semio_framework_job::StepOutcome::Yield;
        }
        if let Some(rejected) = self.job_rejected.as_mut() {
            if rejected.close_step() {
                self.job_rejected = None;
                self.terminal = true;
                return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
            }
            return semio_framework_job::StepOutcome::Yield;
        }
        if let Some(rejected) = self.rejected.as_mut() {
            let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if rejected.terminal_is_empty() {
                self.rejected = None;
                self.terminal = true;
                return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
            }
            return semio_framework_job::StepOutcome::Yield;
        }
        let Some(session) = self.session.as_mut() else {
            self.terminal = true;
            return semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) });
        };
        if !matches!(session.step(), Ok(semio_framework_job::WorkerJobPoll::Outcome | semio_framework_job::WorkerJobPoll::Terminal)) || !session.checkout_outcome() {
            return semio_framework_job::StepOutcome::Yield;
        }
        match session.checked_out_outcome() {
            Some(semio_framework_job::StepOutcome::Yield) => {
                let _ = session.resume();
                semio_framework_job::StepOutcome::Yield
            }
            Some(semio_framework_job::StepOutcome::Complete(_)) => {
                self.terminal = true;
                semio_framework_job::StepOutcome::Complete(semio_framework_job::CommitCandidate {
                    state: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitState),
                    output: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::CommitOutput),
                })
            }
            Some(semio_framework_job::StepOutcome::Cancelled) => {
                self.terminal = true;
                semio_framework_job::StepOutcome::Cancelled
            }
            Some(semio_framework_job::StepOutcome::PreviewReady(_) | semio_framework_job::StepOutcome::CheckpointReady(_) | semio_framework_job::StepOutcome::Fault(_)) | None => {
                session.begin_close();
                self.terminal = true;
                semio_framework_job::StepOutcome::Fault(semio_framework_job::JobFault { detail: semio_framework_job::RetainedJobPayload::empty(semio_framework_job::JobPayloadStream::Fault) })
            }
        }
    }

    pub(crate) fn take_presentation(&mut self) -> Option<AppFramePresentation> {
        if !self.terminal {
            return None;
        }
        let packet = self.session.as_mut()?.checked_out_job_mut()?.take_packet()?;
        self.session.as_mut()?.begin_close();
        Some(AppFramePresentation {
            packet: Some(packet),
            engine_packets: self.engine_packets.take()?,
            generation: self.generation,
            cursor: self.cursor,
            theme_dark: self.theme_dark,
            fullscreen: self.fullscreen,
            cursor_wake: self.cursor_wake.take(),
            #[cfg(not(target_arch = "wasm32"))]
            job_progress: self.job_progress.take(),
        })
    }

    pub(crate) fn close_step(&mut self) -> bool {
        if let Some(rejected) = self.job_rejected.as_mut() {
            if !rejected.close_step() {
                return false;
            }
            self.job_rejected = None;
            return false;
        }
        if let Some(job) = self.job.as_mut() {
            semio_framework_job::InteractiveJob::begin_close(job);
            match semio_framework_job::InteractiveJob::close_step(job, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES) {
                semio_framework_job::InteractiveJobCloseStep::Complete if semio_framework_job::InteractiveJob::terminal_is_empty(job) => self.job = None,
                _ => return false,
            }
            return false;
        }
        if let Some(rejected) = self.rejected.as_mut() {
            let _ = rejected.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if rejected.terminal_is_empty() {
                self.rejected = None;
            }
            return false;
        }
        if let Some(session) = self.session.as_mut() {
            if !matches!(session.poll(), semio_framework_job::WorkerJobPoll::Closing | semio_framework_job::WorkerJobPoll::TerminalEmpty) {
                session.begin_close();
                return false;
            }
            let _ = session.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
            if session.terminal_is_empty() {
                self.session = None;
            }
            return false;
        }
        if self.cursor_wake.take().is_some() {
            return false;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.job_progress.take().is_some() {
            return false;
        }
        let Some(packets) = self.engine_packets.as_mut() else { return true };
        if let Some(packet) = packets.last_mut() {
            if !packet.close_step() || !packet.terminal_is_empty() {
                return false;
            }
            packets.pop();
            return false;
        }
        self.engine_packets = None;
        false
    }

    pub(crate) fn terminal_is_empty(&self) -> bool {
        self.job.is_none() && self.job_rejected.is_none() && self.session.is_none() && self.rejected.is_none() && self.engine_packets.is_none() && self.cursor_wake.is_none() && {
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.job_progress.is_none()
            }
            #[cfg(target_arch = "wasm32")]
            {
                true
            }
        }
    }
}

pub(crate) struct AppPresenter {
    gpu: GpuContext,
    engine: engine_canvas::EngineCanvasPresenter,
    gate: ui_wgpu::wgpu::PreparedRenderGate,
    presentation_authority: RuntimePresentationAuthority,
    raster_operation_authority: RuntimeRasterOperationAuthority,
    window: Option<Arc<Window>>,
    #[cfg(target_arch = "wasm32")]
    offscreen_token: Option<ui_wgpu::wgpu::OffscreenPresentToken>,
    last_cursor: Option<(SemioCursor, bool)>,
    pending: Option<AppPresentCursor>,
    retirement: Option<AppPresentedRetirement>,
    retained_fault: Option<String>,
    surface_resize: Option<AppSurfaceResizeCursor>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AppSurfaceResizePhase {
    Apply,
    InvalidateEngine,
    ApplyGpu,
    Retire,
    Complete,
}

struct AppSurfaceResizeCursor {
    candidate: Option<crate::surface_lane::PreparedSurfaceResize>,
    phase: AppSurfaceResizePhase,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AppPresentPhase {
    Fullscreen,
    Engine,
    BeginGpu,
    Uploads,
    Stage,
    Render,
    CloseGpu,
    Acknowledge,
    ProgressAcknowledge,
    Aborted,
    Directives,
}

struct AppPresentCursor {
    frame: AppFramePresentation,
    phase: AppPresentPhase,
    engine: usize,
    upload: usize,
    witness: Option<ui_wgpu::wgpu::PreparedPresenterWitness>,
    raster_witness: Option<ui_wgpu::wgpu::RasterTextureWitness>,
    gpu_cursor: Option<ui_wgpu::wgpu::PreparedGpuPresentCursor>,
}

struct AppPresentedRetirement {
    previous: Option<ui_wgpu::wgpu::PreparedRenderPacket>,
    completed_frame: Option<AppFramePresentation>,
    raster: RasterCandidateRetirement,
    acknowledged_eviction: usize,
    acknowledged_upload_scan: usize,
    acknowledged_versions: [u64; ui_wgpu::wgpu::MESH_GPU_KEEP_VERSION_CAPACITY],
    acknowledged_version_count: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RasterCandidateRetirement {
    Infer,
    Abort(ui_wgpu::wgpu::RasterTextureWitness),
    Commit(ui_wgpu::wgpu::RasterTextureWitness),
    Complete,
}

pub(crate) enum AppPresentStep {
    Idle,
    Pending,
    Complete { generation: semio_framework_trace::Generation, cursor: SemioCursor, fullscreen: Option<bool>, cursor_wake: Option<infinite_world::world::WorldCursorWakeToken> },
}

impl AppFramePresentation {
    fn close_step(&mut self) -> bool {
        if let Some(packet) = self.packet.as_mut() {
            if !packet.retire_step() {
                return false;
            }
            self.packet = None;
            return false;
        }
        if let Some(packet) = self.engine_packets.last_mut() {
            if !packet.close_step() || !packet.terminal_is_empty() {
                return false;
            }
            self.engine_packets.pop();
            return false;
        }
        if self.cursor_wake.take().is_some() {
            return false;
        }
        #[cfg(not(target_arch = "wasm32"))]
        if self.job_progress.take().is_some() {
            return false;
        }
        true
    }

    fn terminal_is_empty(&self) -> bool {
        self.packet.is_none() && self.engine_packets.is_empty() && self.cursor_wake.is_none() && {
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.job_progress.is_none()
            }
            #[cfg(target_arch = "wasm32")]
            {
                true
            }
        }
    }
}

impl AppPresentedRetirement {
    fn new(previous: Option<ui_wgpu::wgpu::PreparedRenderPacket>) -> Self {
        Self {
            previous,
            completed_frame: None,
            raster: RasterCandidateRetirement::Infer,
            acknowledged_eviction: 0,
            acknowledged_upload_scan: 0,
            acknowledged_versions: [0; ui_wgpu::wgpu::MESH_GPU_KEEP_VERSION_CAPACITY],
            acknowledged_version_count: 0,
        }
    }

    fn abort(previous: ui_wgpu::wgpu::PreparedRenderPacket, witness: ui_wgpu::wgpu::RasterTextureWitness) -> Self {
        let raster = RasterCandidateRetirement::Abort(witness);
        Self { previous: Some(previous), completed_frame: None, raster, acknowledged_eviction: 0, acknowledged_upload_scan: 0, acknowledged_versions: [0; ui_wgpu::wgpu::MESH_GPU_KEEP_VERSION_CAPACITY], acknowledged_version_count: 0 }
    }

    fn commit(previous: Option<ui_wgpu::wgpu::PreparedRenderPacket>, witness: ui_wgpu::wgpu::RasterTextureWitness) -> Self {
        Self {
            previous,
            completed_frame: None,
            raster: RasterCandidateRetirement::Commit(witness),
            acknowledged_eviction: 0,
            acknowledged_upload_scan: 0,
            acknowledged_versions: [0; ui_wgpu::wgpu::MESH_GPU_KEEP_VERSION_CAPACITY],
            acknowledged_version_count: 0,
        }
    }

    fn step(&mut self, gpu: &mut GpuContext, gate: &ui_wgpu::wgpu::PreparedRenderGate) -> Result<bool, String> {
        if self.raster == RasterCandidateRetirement::Infer {
            self.raster = RasterCandidateRetirement::Complete;
            return Ok(false);
        }
        match self.raster {
            RasterCandidateRetirement::Abort(witness) => {
                if !gpu.abort_presented_rasters_step(witness)? {
                    return Ok(false);
                }
                self.raster = RasterCandidateRetirement::Complete;
                return Ok(false);
            }
            RasterCandidateRetirement::Commit(witness) => {
                if !gpu.commit_presented_rasters_step(witness)? {
                    return Ok(false);
                }
                self.raster = RasterCandidateRetirement::Complete;
                return Ok(false);
            }
            RasterCandidateRetirement::Infer | RasterCandidateRetirement::Complete => {}
        }
        if !gpu.close_mesh_upload_step() {
            return Ok(false);
        }
        if let Some(packet) = gate.last_valid() {
            if self.acknowledged_eviction < packet.evictions().len() {
                let Some(eviction) = packet.evictions().get(self.acknowledged_eviction) else { return Err("prepared eviction cursor exceeded its fixed list".to_string()) };
                let key = match eviction {
                    ui_wgpu::wgpu::PreparedRenderEviction::Mesh { key } => key,
                };
                if let Some(upload) = packet.uploads().get(self.acknowledged_upload_scan) {
                    if let ui_wgpu::wgpu::PreparedRenderUpload::Mesh { key: upload_key, version, .. } = upload {
                        if upload_key == key {
                            let Some(slot) = self.acknowledged_versions.get_mut(self.acknowledged_version_count) else {
                                return Err("prepared mesh eviction keep-version credits exhausted".to_string());
                            };
                            *slot = *version;
                            self.acknowledged_version_count += 1;
                        }
                    }
                    self.acknowledged_upload_scan += 1;
                    return Ok(false);
                }
                if gpu.apply_prepared_eviction_step(packet, self.acknowledged_eviction, &self.acknowledged_versions[..self.acknowledged_version_count])? {
                    self.acknowledged_eviction += 1;
                    self.acknowledged_upload_scan = 0;
                    self.acknowledged_version_count = 0;
                }
                return Ok(false);
            }
        }
        if let Some(previous) = self.previous.as_mut() {
            if !previous.retire_step() {
                return Ok(false);
            }
            self.previous = None;
            return Ok(false);
        }
        if let Some(frame) = self.completed_frame.as_mut() {
            if !frame.close_step() {
                return Ok(false);
            }
            self.completed_frame = None;
            return Ok(false);
        }
        Ok(true)
    }

    fn terminal_is_empty(&self) -> bool {
        self.raster == RasterCandidateRetirement::Complete && self.previous.is_none() && self.completed_frame.as_ref().is_none_or(AppFramePresentation::terminal_is_empty)
    }
}

impl AppPresenter {
    pub(crate) fn dpr(&self) -> f32 {
        self.gpu.dpr()
    }

    pub(crate) fn surface_resize_available(&self) -> bool {
        self.surface_resize.is_none()
    }

    pub(crate) fn begin_surface_resize(&mut self, candidate: crate::surface_lane::PreparedSurfaceResize) -> Result<(), crate::surface_lane::PreparedSurfaceResize> {
        if self.surface_resize.is_some() {
            return Err(candidate);
        }
        self.surface_resize = Some(AppSurfaceResizeCursor { candidate: Some(candidate), phase: AppSurfaceResizePhase::Apply });
        Ok(())
    }

    pub(crate) fn surface_resize_step(&mut self) -> bool {
        let Some(cursor) = self.surface_resize.as_mut() else { return true };
        match cursor.phase {
            AppSurfaceResizePhase::Apply => {
                let Some(candidate) = cursor.candidate else {
                    cursor.phase = AppSurfaceResizePhase::Complete;
                    return false;
                };
                if !self.engine.observe_primary_metrics_generation(candidate.metrics_generation()) {
                    return false;
                }
                cursor.phase = AppSurfaceResizePhase::InvalidateEngine;
                false
            }
            AppSurfaceResizePhase::InvalidateEngine => {
                if !self.engine.invalidate_primary_metrics_step() {
                    return false;
                }
                cursor.phase = AppSurfaceResizePhase::ApplyGpu;
                false
            }
            AppSurfaceResizePhase::ApplyGpu => {
                let Some(candidate) = cursor.candidate else {
                    cursor.phase = AppSurfaceResizePhase::Complete;
                    return false;
                };
                if !candidate.suspended() {
                    self.gpu.resize(candidate.logical_width(), candidate.logical_height(), candidate.scale_factor());
                }
                cursor.phase = AppSurfaceResizePhase::Retire;
                false
            }
            AppSurfaceResizePhase::Retire => {
                cursor.candidate.take();
                cursor.phase = AppSurfaceResizePhase::Complete;
                false
            }
            AppSurfaceResizePhase::Complete => {
                self.surface_resize = None;
                true
            }
        }
    }

    pub(crate) fn close_surface_resize_step(&mut self) -> bool {
        if let Some(cursor) = self.surface_resize.as_mut() {
            if cursor.candidate.take().is_some() {
                cursor.phase = AppSurfaceResizePhase::Retire;
                return false;
            }
            cursor.phase = AppSurfaceResizePhase::Complete;
            self.surface_resize = None;
            return false;
        }
        self.engine.invalidate_primary_metrics_step()
    }

    pub(crate) fn has_pending_presentation(&self) -> bool {
        self.pending.is_some() || self.retirement.is_some() || self.retained_fault.is_some() || self.gate.has_pending_acknowledgement()
    }

    pub(crate) fn close_cursor_wake_step(&mut self) -> bool {
        let Some(pending) = self.pending.as_mut() else { return true };
        if pending.frame.cursor_wake.take().is_some() {
            return false;
        }
        true
    }

    pub(crate) fn close_frame_owners_step(&mut self) -> Result<bool, String> {
        if !self.engine.close_active_candidate_step(&mut self.gpu)? {
            return Ok(false);
        }
        if let Some(mut cursor) = self.pending.take() {
            if let Some(gpu_cursor) = cursor.gpu_cursor.as_mut() {
                gpu_cursor.begin_close();
                if !gpu_cursor.close_step() {
                    self.pending = Some(cursor);
                    return Ok(false);
                }
                cursor.gpu_cursor = None;
                self.pending = Some(cursor);
                return Ok(false);
            }
            if cursor.frame.packet.is_none() {
                cursor.frame.packet = self.gate.abort_pending();
            }
            let retirement = self.retirement.get_or_insert_with(|| AppPresentedRetirement::new(None));
            if retirement.completed_frame.is_some() {
                self.pending = Some(cursor);
                return Err("presenter close retirement capacity exhausted".to_string());
            }
            if let Some(witness) = cursor.raster_witness.take() {
                if let Err(error) = self.raster_operation_authority.release(witness) {
                    cursor.raster_witness = Some(witness);
                    self.pending = Some(cursor);
                    return Err(error.to_string());
                }
                retirement.raster = RasterCandidateRetirement::Abort(witness);
            }
            retirement.completed_frame = Some(cursor.frame);
            return Ok(false);
        }
        if let Some(packet) = self.gate.abort_pending() {
            if self.retirement.is_some() {
                return Err("presenter pending packet cannot overtake retirement".to_string());
            }
            let witness = self.raster_operation_authority.current().ok_or_else(|| "presenter raster operation witness was missing during close".to_string())?;
            self.raster_operation_authority.release(witness).map_err(str::to_owned)?;
            self.retirement = Some(AppPresentedRetirement::abort(packet, witness));
            return Ok(false);
        }
        if let Some(retirement) = self.retirement.as_mut() {
            if !retirement.step(&mut self.gpu, &self.gate)? {
                return Ok(false);
            }
            self.retirement = None;
            return Ok(false);
        }
        if let Some(fault) = self.retained_fault.as_mut() {
            if fault.pop().is_some() {
                return Ok(false);
            }
            self.retained_fault = None;
            return Ok(false);
        }
        if let Some(packet) = self.gate.take_last_valid() {
            self.retirement = Some(AppPresentedRetirement::new(Some(packet)));
            return Ok(false);
        }
        if !self.gate.close_step() {
            return Ok(false);
        }
        Ok(self.pending.is_none() && self.retirement.as_ref().is_none_or(AppPresentedRetirement::terminal_is_empty) && self.retained_fault.is_none() && self.gate.terminal_is_empty() && self.raster_operation_authority.current().is_none())
    }

    pub(crate) fn close_world_owners_step(&mut self) -> Result<bool, String> {
        if !self.close_frame_owners_step()? || !self.engine.terminal_is_empty() {
            return Ok(false);
        }
        if !self.gpu.close_mesh_upload_step() {
            return Ok(false);
        }
        if !self.gpu.close_mesh_table_step() {
            return Ok(false);
        }
        if !self.gpu.close_raster_table_step()? {
            return Ok(false);
        }
        Ok(self.world_owners_terminal_is_empty())
    }

    pub(crate) fn world_owners_terminal_is_empty(&self) -> bool {
        self.pending.is_none()
            && self.retirement.as_ref().is_none_or(AppPresentedRetirement::terminal_is_empty)
            && self.gate.terminal_is_empty()
            && self.gpu.mesh_upload_terminal_is_empty()
            && self.gpu.mesh_table_terminal_is_empty()
            && self.gpu.raster_table_terminal_is_empty()
            && self.raster_operation_authority.current().is_none()
            && self.engine.terminal_is_empty()
            && self.retained_fault.is_none()
    }

    pub(crate) fn admit_next_frame(&mut self, produce: impl FnOnce() -> Option<AppFramePresentation>) -> Option<SemioCursor> {
        if self.has_pending_presentation() {
            return None;
        }
        let frame = produce()?;
        let cursor = frame.cursor;
        self.pending = Some(AppPresentCursor { frame, phase: AppPresentPhase::BeginGpu, engine: 0, upload: 0, witness: None, raster_witness: None, gpu_cursor: None });
        Some(cursor)
    }

    pub(crate) fn present_step(&mut self) -> Result<AppPresentStep, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let _ = RENDERER_IO_WAKE.swap(false, std::sync::atomic::Ordering::AcqRel);
            let _ = pump_renderer_io_sessions(1);
        }
        let _ = semio_framework_job::pump_worker_job_retirements(1, 1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
        if self.pending.is_none() {
            if let Some(retirement) = self.retirement.as_mut() {
                if retirement.step(&mut self.gpu, &self.gate)? {
                    self.retirement = None;
                }
                return Ok(AppPresentStep::Pending);
            }
            if let Some(fault) = self.retained_fault.take() {
                return Err(fault);
            }
            return Ok(AppPresentStep::Idle);
        }
        let Some(cursor) = self.pending.as_mut() else { return Ok(AppPresentStep::Idle) };
        match cursor.phase {
            AppPresentPhase::Aborted => {
                if !self.engine.close_active_candidate_step(&mut self.gpu)? {
                    return Ok(AppPresentStep::Pending);
                }
                if let Some(gpu_cursor) = cursor.gpu_cursor.as_mut() {
                    gpu_cursor.begin_close();
                    if !gpu_cursor.close_step() {
                        return Ok(AppPresentStep::Pending);
                    }
                    cursor.gpu_cursor = None;
                    return Ok(AppPresentStep::Pending);
                }
                let Some(mut aborted) = self.pending.take() else { return Err("aborted presentation cursor was missing".to_string()) };
                let retirement = self.retirement.get_or_insert_with(|| AppPresentedRetirement::new(None));
                if retirement.completed_frame.is_some() {
                    self.pending = Some(aborted);
                    return Err("aborted presentation retirement capacity exhausted".to_string());
                }
                if let Some(witness) = aborted.raster_witness.take() {
                    if let Err(error) = self.raster_operation_authority.release(witness) {
                        aborted.raster_witness = Some(witness);
                        self.pending = Some(aborted);
                        return Err(error.to_string());
                    }
                    retirement.raster = RasterCandidateRetirement::Abort(witness);
                }
                retirement.completed_frame = Some(aborted.frame);
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Fullscreen => {
                if let Some(active) = cursor.frame.fullscreen {
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        if let Some(window) = self.window.as_ref() {
                            window.set_fullscreen(if active { Some(Fullscreen::Borderless(None)) } else { None });
                        }
                    }
                    #[cfg(target_arch = "wasm32")]
                    {
                        use winit::platform::web::WindowExtWebSys;
                        if let Some(canvas) = self.window.as_ref().and_then(|window| window.canvas()) {
                            let document = canvas.owner_document();
                            if active {
                                if let Err(error) = canvas.request_fullscreen() {
                                    web_sys::console::error_2(&"Fullscreen request was rejected".into(), &error);
                                }
                            } else if let Some(document) = document {
                                document.exit_fullscreen();
                            }
                        }
                    }
                }
                cursor.phase = AppPresentPhase::Directives;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Engine => {
                if let Some(packet) = cursor.frame.engine_packets.get(cursor.engine) {
                    let candidate = cursor.raster_witness.ok_or_else(|| "engine raster operation witness was missing".to_string())?;
                    let expected = self.raster_operation_authority.current().ok_or_else(|| "engine raster operation authority was empty".to_string())?;
                    match self.engine.realize_step(&mut self.gpu, packet, candidate, expected) {
                        Ok(true) => cursor.engine += 1,
                        Ok(false) => return Ok(AppPresentStep::Pending),
                        Err(error) => {
                            cursor.phase = AppPresentPhase::Aborted;
                            if self.retained_fault.is_none() {
                                self.retained_fault = Some(format!("engine canvas present: {error}"));
                            }
                            return Ok(AppPresentStep::Pending);
                        }
                    }
                    return Ok(AppPresentStep::Pending);
                }
                cursor.phase = AppPresentPhase::Uploads;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::BeginGpu => {
                let packet = cursor.frame.packet.as_ref().ok_or_else(|| "prepared frame packet was transferred before admission".to_string())?;
                let expected = self.presentation_authority.current();
                #[cfg(not(target_arch = "wasm32"))]
                {
                    let token = ui_wgpu::wgpu::UiPresentToken::mint_for_current_thread();
                    if let Err(error) = self.gpu.begin_prepared(&token, &self.gate, packet, expected.scene_revision, expected.input_generation) {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err(format!("prepared frame admission: {error}"));
                    }
                }
                #[cfg(target_arch = "wasm32")]
                {
                    let Some(token) = self.offscreen_token.as_ref() else {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err("browser presentation requires dedicated Worker authority".to_string());
                    };
                    if let Err(error) = self.gpu.begin_prepared_offscreen(token, &self.gate, packet, expected.scene_revision, expected.input_generation) {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err(format!("offscreen prepared frame admission: {error}"));
                    }
                }
                let witness = match self.raster_operation_authority.begin(expected.scene_revision, expected.input_generation) {
                    Ok(witness) => witness,
                    Err(error) => {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err(format!("raster operation admission: {error}"));
                    }
                };
                cursor.raster_witness = Some(witness);
                cursor.phase = AppPresentPhase::Engine;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Uploads => {
                let packet = cursor.frame.packet.as_ref().ok_or_else(|| "prepared frame packet was transferred before upload".to_string())?;
                if cursor.upload >= packet.uploads().len() {
                    cursor.phase = AppPresentPhase::Stage;
                    return Ok(AppPresentStep::Pending);
                }
                let candidate = cursor.raster_witness.ok_or_else(|| "prepared raster operation witness was missing".to_string())?;
                let expected = self.raster_operation_authority.current().ok_or_else(|| "prepared raster operation authority was empty".to_string())?;
                match self.gpu.apply_prepared_upload_step(packet, cursor.upload, candidate, expected) {
                    Ok(true) => cursor.upload += 1,
                    Ok(false) => {}
                    Err(error) => {
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err(error);
                    }
                }
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Stage => {
                let Some(packet) = cursor.frame.packet.take() else { return Err("validated prepared frame packet was missing".to_string()) };
                cursor.witness = match self.gate.stage_presented(packet) {
                    Ok(witness) => Some(witness),
                    Err(packet) => {
                        cursor.frame.packet = Some(packet);
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err("prepared frame presenter witness was already occupied".to_string());
                    }
                };
                cursor.phase = AppPresentPhase::Render;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Render => {
                let Some(witness) = cursor.witness.as_ref() else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame presenter witness was missing before submit".to_string());
                };
                let Some(packet) = self.gate.pending_presented(witness) else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.witness = None;
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame presenter witness was stale before submit".to_string());
                };
                let raster_witness = cursor.raster_witness.ok_or_else(|| "raster operation witness was missing before submit".to_string())?;
                if !self.raster_operation_authority.matches(raster_witness) {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.witness = None;
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("raster operation authority was stale before submit".to_string());
                }
                if cursor.gpu_cursor.is_none() {
                    match self.gpu.begin_prepared_present(packet, raster_witness) {
                        Ok(gpu_cursor) => cursor.gpu_cursor = Some(gpu_cursor),
                        Err(error) => {
                            cursor.frame.packet = self.gate.abort_pending();
                            cursor.witness = None;
                            cursor.phase = AppPresentPhase::Aborted;
                            return Err(format!("prepared frame submit admission: {error}"));
                        }
                    }
                    return Ok(AppPresentStep::Pending);
                }
                let Some(gpu_cursor) = cursor.gpu_cursor.as_mut() else { return Ok(AppPresentStep::Pending) };
                match self.gpu.prepared_present_step(packet, gpu_cursor) {
                    Ok(true) => cursor.phase = AppPresentPhase::CloseGpu,
                    Ok(false) => {}
                    Err(error) => {
                        gpu_cursor.begin_close();
                        cursor.frame.packet = self.gate.abort_pending();
                        cursor.witness = None;
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err(format!("prepared frame submit step: {error}"));
                    }
                }
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::CloseGpu => {
                let Some(gpu_cursor) = cursor.gpu_cursor.as_mut() else {
                    cursor.phase = AppPresentPhase::Acknowledge;
                    return Ok(AppPresentStep::Pending);
                };
                gpu_cursor.begin_close();
                if gpu_cursor.close_step() {
                    cursor.gpu_cursor = None;
                    cursor.phase = AppPresentPhase::Acknowledge;
                }
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Acknowledge => {
                let Some(witness) = cursor.witness.take() else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame presenter witness was missing".to_string());
                };
                let Some(packet) = self.gate.pending_presented(&witness) else {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame presenter witness was stale before acknowledgement".to_string());
                };
                let expected = self.presentation_authority.current();
                if packet.scene_revision() != expected.scene_revision || packet.preview_generation() != expected.input_generation {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("prepared frame authority was stale before acknowledgement".to_string());
                }
                let raster_witness = cursor.raster_witness.ok_or_else(|| "raster operation witness was missing before acknowledgement".to_string())?;
                if !self.raster_operation_authority.matches(raster_witness) || raster_witness.scene_revision != packet.scene_revision() || raster_witness.preview_generation != packet.preview_generation() {
                    cursor.frame.packet = self.gate.abort_pending();
                    cursor.phase = AppPresentPhase::Aborted;
                    return Err("raster operation authority was stale before acknowledgement".to_string());
                }
                let mut replacement = match self.gate.acknowledge_presented(witness) {
                    Ok(replacement) => replacement,
                    Err(_) => {
                        cursor.frame.packet = self.gate.abort_pending();
                        cursor.phase = AppPresentPhase::Aborted;
                        return Err("prepared frame presenter witness was stale or duplicated".to_string());
                    }
                };
                self.raster_operation_authority.release(raster_witness).map_err(str::to_owned)?;
                cursor.raster_witness = None;
                self.retirement = Some(AppPresentedRetirement::commit(replacement.take_previous(), raster_witness));
                cursor.phase = AppPresentPhase::ProgressAcknowledge;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::ProgressAcknowledge => {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(progress) = cursor.frame.job_progress.as_mut() {
                    if !progress.acknowledge_presented() {
                        return Ok(AppPresentStep::Pending);
                    }
                }
                cursor.phase = AppPresentPhase::Fullscreen;
                Ok(AppPresentStep::Pending)
            }
            AppPresentPhase::Directives => {
                if let Some(window) = self.window.as_ref() {
                    apply_window_cursor(window, cursor.frame.cursor, cursor.frame.theme_dark, &mut self.last_cursor);
                }
                let fullscreen = self.window.is_none().then_some(cursor.frame.fullscreen).flatten();
                let Some(mut completed) = self.pending.take() else { return Err("completed presentation cursor was missing".to_string()) };
                let accepted_cursor = completed.frame.cursor;
                let accepted_generation = completed.frame.generation;
                let cursor_wake = completed.frame.cursor_wake.take();
                let retirement = self.retirement.get_or_insert_with(|| AppPresentedRetirement::new(None));
                if retirement.completed_frame.is_some() {
                    completed.frame.cursor_wake = cursor_wake;
                    self.pending = Some(completed);
                    return Err("completed presentation retirement capacity exhausted".to_string());
                }
                retirement.completed_frame = Some(completed.frame);
                Ok(AppPresentStep::Complete { generation: accepted_generation, cursor: accepted_cursor, fullscreen, cursor_wake })
            }
        }
    }

    pub(crate) fn engine_surface_token_at(&self, index: usize) -> Option<engine_canvas::EngineSurfaceToken> {
        self.engine.surface_token_at(index)
    }

    pub(crate) fn begin_engine_surface_close(&mut self, token: engine_canvas::EngineSurfaceToken) -> bool {
        self.engine.begin_surface_close(token)
    }

    pub(crate) fn close_engine_surface_step(&mut self, token: engine_canvas::EngineSurfaceToken) -> Result<bool, String> {
        self.engine.close_surface_step(&mut self.gpu, token)
    }

    pub(crate) fn engine_surface_terminal_is_empty(&self, token: engine_canvas::EngineSurfaceToken) -> bool {
        self.engine.surface_terminal_is_empty(token)
    }

    pub(crate) fn engine_surfaces_terminal_is_empty(&self) -> bool {
        self.engine.terminal_is_empty()
    }
}

#[cfg(test)]
#[test]
fn realize_fault_remains_scheduled_until_the_aborted_cursor_is_terminal() {
    let glue = include_str!("🦀️.rs");
    let native = include_str!("../🪟️winit-app/🦀️.rs");
    assert!(glue.contains("cursor.phase = AppPresentPhase::Aborted;\n                            if self.retained_fault.is_none()"));
    assert!(glue.contains("return Ok(AppPresentStep::Pending);"));
    assert!(native.contains("if self.presenter.has_pending_presentation()"));
    assert!(native.contains("self.scheduler.invalidate(InvalidationReason::RESOURCE_READY);"));
}

/// 🧪️ P3c: `self_weak` was the only field that made `AppRuntime` definitionally `Rc<RefCell<_>>`-owned
/// (see `AppHandle`'s own doc comment above). With it gone, this assertion lets the compiler — not a
/// person re-deriving the per-field audit by hand every time a field is added — settle whether the
/// struct is `Send` today. The mounted native compiler gate exercising this assertion is recorded in
/// `📓️p3c-explicit-app-handle.md`; wasm32 deliberately excludes it because the renderer's
/// browser-side handles are single-threaded platform values.
#[cfg(not(target_arch = "wasm32"))]
const _: fn() = || {
    fn assert_send<T: Send>() {}
    assert_send::<AppRuntime>();
    assert_send::<AppFrameBuild>();
    assert_send::<AppFramePresentation>();
};

#[cfg(not(target_arch = "wasm32"))]
fn resolve_asset_fetch_url(url: &str) -> String {
    if url.starts_with("http://") || url.starts_with("https://") {
        return url.to_string();
    }
    if url.starts_with('/') {
        let base = std::env::var("SEMIO_ASSET_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:6141".to_string());
        return format!("{}{}", base.trim_end_matches('/'), url);
    }
    url.to_string()
}

#[cfg(not(target_arch = "wasm32"))]
fn resolve_map_tile_fetch_url(url: &str) -> String {
    resolve_asset_fetch_url(url)
}

#[cfg(not(target_arch = "wasm32"))]
fn native_renderer_asset_path(url: &str) -> Result<std::path::PathBuf, String> {
    if url.starts_with("/mesh/") {
        let source = &mesh_assets::resolve_mesh_asset(url)?.source;
        return Ok(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../../../../../../..").join(source));
    }
    if let Some(path) = url.strip_prefix("file://") {
        return Ok(std::path::PathBuf::from(path));
    }
    if url.starts_with('/') {
        return Ok(std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../..").join(url.trim_start_matches('/')));
    }
    Ok(std::path::PathBuf::from(url))
}

#[cfg(not(target_arch = "wasm32"))]
async fn stream_native_renderer_asset(mailbox: &RuntimeMailbox, fetch: &mut RendererAssetFetchOwner) -> Result<(), String> {
    if fetch.url().starts_with("http://") || fetch.url().starts_with("https://") {
        return stream_native_renderer_http_asset(mailbox, fetch).await;
    }
    let path = native_renderer_asset_path(fetch.url())?;
    let mut offset = 0u64;
    loop {
        if mailbox.renderer_asset_cancelled(fetch) {
            return Err("native renderer asset cancelled or stale".into());
        }
        let value = run_renderer_io(semio_framework_os_services::NativeIoRequest::ReadPage { path: path.clone(), offset, max_bytes: WORLD_ASSET_RESPONSE_PAGE_BYTES }).await?;
        let semio_framework_os_services::NativeIoValue::Page { bytes, eof } = value else { return Err("native renderer asset I/O returned the wrong value".into()) };
        if bytes.is_empty() && !eof {
            return Err("native renderer asset produced an empty nonterminal page".into());
        }
        let count = bytes.len();
        if count != 0 {
            push_renderer_asset_page(fetch, bytes)?;
            offset = offset.checked_add(count as u64).ok_or_else(|| "native renderer asset offset overflow".to_string())?;
        }
        if eof {
            break;
        }
    }
    if !mailbox.seal_renderer_asset_response(fetch) {
        return Err("native renderer asset could not seal its exact byte claim".into());
    }
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
fn push_renderer_asset_page(_fetch: &mut RendererAssetFetchOwner, mut bytes: semio_framework_job::RetainedJobPayload) -> Result<(), String> {
    let populated = !bytes.is_empty();
    let _ = bytes.close_step(1, semio_framework_job::JOB_PAYLOAD_PAGE_BYTES);
    if populated {
        Err("native renderer asset page requires a zero-copy retained-page handoff that is not mounted".into())
    } else {
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
async fn stream_native_renderer_http_asset(mailbox: &RuntimeMailbox, fetch: &mut RendererAssetFetchOwner) -> Result<(), String> {
    let context = semio_framework_async::OperationContext {
        actor: 0,
        generation: 0,
        trace: semio_framework_async::TraceId(0),
        lane: semio_framework_async::Lane::Io as u8,
        deadline_ms: Some(semio_framework_job::default_now_ms().and_then(|now| now.checked_add(15_000)).ok_or_else(|| "native asset deadline requires a real clock and checked range".to_string())?),
        cancel: mailbox.0.native_asset_http_cancel.child_now(),
        capability: None,
    };
    let request = semio_framework_os_services::HttpRequest { method: "GET".into(), url: fetch.url().to_owned(), headers: Vec::new(), body: Vec::new() };
    let pool = if fetch.url().starts_with("https://") { &mailbox.0.native_asset_https } else { &mailbox.0.native_asset_http };
    let (head, mut body) = pool
        .fetch(mailbox.0.native_asset_http_runtime.as_ref(), &mailbox.0.native_asset_http_scope, context, semio_framework_actor::PackageId("os.renderer.asset".into()), semio_framework_actor::ActorId(0), request)
        .await
        .map_err(|error| error.to_string())?;
    if !(200..300).contains(&head.status) {
        return Err(format!("native renderer asset HTTP status {}", head.status));
    }
    if let Some(length) = head.headers.iter().find(|(name, _)| name.eq_ignore_ascii_case("content-length")).and_then(|(_, value)| value.parse::<usize>().ok()) {
        if length > WORLD_ASSET_RESPONSE_BYTE_CAPACITY {
            return Err("native renderer asset Content-Length exceeded fixed credits".into());
        }
    }
    while let Some(bytes) = body.next_chunk().await.map_err(|error| error.to_string())? {
        if mailbox.renderer_asset_cancelled(fetch) {
            return Err("native renderer asset cancelled or stale".into());
        }
        if bytes.is_empty() || bytes.len() > WORLD_ASSET_RESPONSE_PAGE_BYTES {
            return Err("native renderer HTTP asset returned an invalid response page".into());
        }
        push_renderer_asset_page(fetch, bytes)?;
    }
    if !mailbox.seal_renderer_asset_response(fetch) {
        return Err("native renderer HTTP asset could not seal its exact byte claim".into());
    }
    Ok(())
}

impl AppRuntime {
    #[cfg(not(target_arch = "wasm32"))]
    fn poll_native_plugin_hot_swap(&mut self) {
        if let Some(entries) = self.native_hot_swap_modified.as_mut() {
            if let Some((path, mtime)) = entries.pop() {
                let previous = self.native_plugin_mtimes.get(&path);
                if previous.is_some_and(|previous| *previous != mtime) {
                    self.native_reload_pending = true;
                }
                self.native_plugin_mtimes.insert(path, mtime);
                return;
            }
            self.native_hot_swap_modified = None;
            return;
        }
        if let Some(scan) = self.native_hot_swap_scan.as_mut() {
            let Some(result) = scan.try_take() else { return };
            self.native_hot_swap_scan = None;
            match result {
                Ok(semio_framework_os_services::NativeIoValue::Modified(entries)) => self.native_hot_swap_modified = Some(entries),
                Ok(_) => log_debug("plugin hot-swap scan returned the wrong native I/O value"),
                Err(error) => log_debug(&format!("plugin hot-swap scan failed: {error}")),
            }
            return;
        }
        let Some(program) = self.shell.plugins.get(self.native_hot_swap_cursor % self.shell.plugins.len().max(1)) else { return };
        self.native_hot_swap_cursor = self.native_hot_swap_cursor.wrapping_add(1);
        let Some(path) = program.wasm_artifact_path().map(std::path::Path::to_path_buf) else { return };
        let mut paths = semio_framework_os_services::NativePathSet::new();
        if paths.try_push(path).is_err() {
            return;
        }
        self.native_hot_swap_scan = submit_renderer_io(semio_framework_os_services::NativeIoRequest::Modified(paths)).ok();
    }

    /// 🎠️ Hot-reload preparation snapshots only the filter and module root. Loading runs on the
    /// process pool; its completion re-enters through the runtime mailbox.
    #[cfg(not(target_arch = "wasm32"))]
    fn maybe_reload_native_plugins(&mut self, handle: &AppHandle) {
        if !self.native_reload_pending {
            return;
        }
        self.native_reload_pending = false;
        let plugin_filter = self.shell.plugin_filter.clone();
        let modules_root = self.plugin_modules_root.clone();
        let Some(mailbox) = handle.upgrade().map(RuntimeMailbox) else { return };
        let accepted = mailbox.reserve_future(Some("plugin-reload"));
        if accepted {
            let revision = mailbox.0.next_revision.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            spawn_app_task(async move {
                let result = load_wasm_plugins(&plugin_filter, &modules_root).await.map(|entries| filter_plugins(entries, &plugin_filter));
                mailbox.0.finish(RuntimeCompletion { key: None, revision, requires_interaction: true, apply: RuntimeApply::PluginReload(Some(result)) });
            });
        }
        if !accepted {
            self.native_reload_pending = true;
        }
    }

    /// 🧵️ Advances exactly one retained pre-input frame owner or chrome child.
    fn frame_before_input_step(&mut self, handle: &AppHandle, build_directives: &crate::frame_job::FrameDirectives, dpr: f32, cursor: &mut FrameBuildCursor) -> FrameBuildBoundaryStep {
        match cursor.phase {
            FrameBuildPhase::Deferred => {
                if !ui_wgpu::wgpu::PreparedAtlasPages::close_abandoned_step() {
                    return FrameBuildBoundaryStep::Pending;
                }
                cursor.phase = FrameBuildPhase::PreparedGpuAbandonment;
            }
            FrameBuildPhase::PreparedGpuAbandonment => {
                if !ui_wgpu::wgpu::PreparedGpuPresentCursor::close_abandoned_step() {
                    return FrameBuildBoundaryStep::Pending;
                }
                cursor.phase = FrameBuildPhase::PreparedInputAbandonment;
            }
            FrameBuildPhase::PreparedInputAbandonment => {
                if !ui_wgpu::wgpu::PreparedRenderInput::close_abandoned_step() {
                    return FrameBuildBoundaryStep::Pending;
                }
                cursor.phase = FrameBuildPhase::PreparedJobAbandonment;
            }
            FrameBuildPhase::PreparedJobAbandonment => {
                if !ui_wgpu::wgpu::PreparedRenderJob::close_abandoned_step() {
                    return FrameBuildBoundaryStep::Pending;
                }
                cursor.phase = FrameBuildPhase::PreparedAbandonment;
            }
            FrameBuildPhase::PreparedAbandonment => {
                if !ui_wgpu::wgpu::PreparedRenderReceiver::close_abandoned_step() {
                    return FrameBuildBoundaryStep::Pending;
                }
                cursor.phase = FrameBuildPhase::PacketAbandonment;
            }
            FrameBuildPhase::PacketAbandonment => {
                if !ui_wgpu::wgpu::PreparedRenderPacket::close_abandoned_step() {
                    return FrameBuildBoundaryStep::Pending;
                }
                cursor.phase = FrameBuildPhase::Text;
            }
            FrameBuildPhase::Text => {
                self.drive_text_operation();
                cursor.phase = FrameBuildPhase::Fullscreen;
            }
            FrameBuildPhase::Fullscreen => {
                cursor.fullscreen = std::mem::take(&mut self.shell.fullscreen_toggle_requested).then(|| {
                    self.shell.fullscreen_active = !self.shell.fullscreen_active;
                    self.shell.fullscreen_active
                });
                cursor.phase = FrameBuildPhase::NativePoll;
            }
            FrameBuildPhase::NativePoll => {
                #[cfg(not(target_arch = "wasm32"))]
                self.poll_native_plugin_hot_swap();
                cursor.phase = FrameBuildPhase::NativeReload;
            }
            FrameBuildPhase::NativeReload => {
                #[cfg(not(target_arch = "wasm32"))]
                self.maybe_reload_native_plugins(handle);
                cursor.phase = FrameBuildPhase::ThemeResolve;
            }
            FrameBuildPhase::ThemeResolve => {
                self.theme = shell::resolve_theme_for_ids(self.shell.active_theme_id_for_frame(), &self.shell.appearance_id);
                cursor.phase = FrameBuildPhase::ThemeAppearance;
            }
            FrameBuildPhase::ThemeAppearance => {
                self.theme_dark = appearance_is_dark(&self.shell.appearance_id);
                cursor.phase = FrameBuildPhase::Drag;
            }
            FrameBuildPhase::Drag => {
                if !self.pointer_down && self.input.drag.active {
                    self.input.end_drag();
                }
                cursor.phase = FrameBuildPhase::Hover;
            }
            FrameBuildPhase::Hover => {
                self.input.update_hover(self.last_pointer_x, self.last_pointer_y);
                cursor.phase = FrameBuildPhase::InputFrame;
            }
            FrameBuildPhase::InputFrame => {
                if self.input.hit_targets.pop().is_some() {
                    return FrameBuildBoundaryStep::Pending;
                }
                self.input.wheel_delta = 0.0;
                self.input.right_click_pos = None;
                cursor.phase = FrameBuildPhase::WheelDeadline;
            }
            FrameBuildPhase::WheelDeadline => {
                if build_directives.wheel_zoom_deadline_cleared && self.wheel_zoom_deadline_ms > 0.0 && app_now_ms() >= self.wheel_zoom_deadline_ms {
                    self.wheel_zoom_deadline_ms = 0.0;
                    engine_canvas::node_graph_clear_wheel_zoom_active();
                }
                cursor.phase = FrameBuildPhase::Caret;
            }
            FrameBuildPhase::Caret => {
                if app_now_ms() - self.caret_blink_at_ms >= 500.0 {
                    self.caret_blink_at_ms = app_now_ms();
                    self.caret_blink_visible = !self.caret_blink_visible;
                    engine_canvas::node_graph_sync_caret_blink(self.caret_blink_visible);
                }
                cursor.phase = FrameBuildPhase::TakeDraw;
            }
            FrameBuildPhase::TakeDraw => {
                cursor.previous_draw = Some(std::mem::take(&mut self.draw));
                cursor.phase = FrameBuildPhase::RetireDraw;
            }
            FrameBuildPhase::RetireDraw => {
                let Some(previous) = cursor.previous_draw.as_mut() else {
                    cursor.phase = FrameBuildPhase::TakeOverlay;
                    return FrameBuildBoundaryStep::Pending;
                };
                if previous.retire_step() {
                    cursor.previous_draw = None;
                    cursor.phase = FrameBuildPhase::TakeOverlay;
                }
            }
            FrameBuildPhase::TakeOverlay => {
                cursor.previous_overlay = Some(std::mem::take(&mut self.overlay));
                cursor.phase = FrameBuildPhase::RetireOverlay;
            }
            FrameBuildPhase::RetireOverlay => {
                let Some(previous) = cursor.previous_overlay.as_mut() else {
                    cursor.phase = FrameBuildPhase::IconAtlas;
                    return FrameBuildBoundaryStep::Pending;
                };
                if previous.retire_step() {
                    cursor.previous_overlay = None;
                    cursor.phase = FrameBuildPhase::IconAtlas;
                }
            }
            FrameBuildPhase::IconAtlas => {
                if !cursor.icon_checked {
                    let mut pending = false;
                    let mut observed = false;
                    ICON_ATLAS_RUNTIME.with(|cell| {
                        if let Some(atlas) = cell.borrow_mut().take() {
                            observed = true;
                            self.icons = atlas;
                            if let Ok(pages) = ui_wgpu::wgpu::PreparedAtlasPages::try_new(self.icons.width, self.icons.height, 4, self.icons.pixels.len()) {
                                cursor.icon_pages = Some(pages);
                                pending = true;
                            }
                        }
                    });
                    cursor.icon_checked = true;
                    if observed && !pending {
                        return FrameBuildBoundaryStep::Fault("frame icon atlas page admission failed");
                    }
                    return FrameBuildBoundaryStep::Pending;
                }
                if let Some(pages) = cursor.icon_pages.as_mut() {
                    let complete = match pages.push_page(&self.icons.pixels, pages.next_row()) {
                        Ok(complete) => complete,
                        Err(_) => return FrameBuildBoundaryStep::Fault("frame icon atlas page construction faulted"),
                    };
                    if !complete {
                        return FrameBuildBoundaryStep::Pending;
                    }
                    let Some(pages) = cursor.icon_pages.take() else { return FrameBuildBoundaryStep::Fault("frame icon atlas pages lost ownership") };
                    cursor.icon_upload = Some(ui_wgpu::wgpu::PreparedRenderUpload::IconAtlasPages { pixels: pages });
                }
                cursor.phase = FrameBuildPhase::Tutorial;
            }
            FrameBuildPhase::Tutorial => {
                self.shell.tutorial_tick(app_now_ms());
                cursor.phase = FrameBuildPhase::EngineResources;
            }
            FrameBuildPhase::EngineResources => {
                cursor.engine_resources = Some(engine_canvas::EngineCanvasBuildContext::new(dpr as f64, cursor.presentation_witness.input_generation, cursor.presentation_witness.scene_revision));
                cursor.phase = FrameBuildPhase::WorldResources;
            }
            FrameBuildPhase::WorldResources => {
                let Some(runtime) = handle.upgrade().map(RuntimeMailbox) else { return FrameBuildBoundaryStep::Fault("frame runtime lost its wake authority") };
                cursor.world_resources = Some(infinite_world::world::World3dBuildContext::new(runtime.world_cursor_wake_authority()));
                cursor.phase = FrameBuildPhase::Chrome;
            }
            FrameBuildPhase::Chrome => {
                let Some(engine_resources) = cursor.engine_resources.as_mut() else { return FrameBuildBoundaryStep::Fault("frame chrome lost engine resources") };
                let Some(world_resources) = cursor.world_resources.as_mut() else { return FrameBuildBoundaryStep::Fault("frame chrome lost world resources") };
                let AppRuntime { atlas, icons, interaction, draw, overlay, .. } = self;
                let Some(interaction) = interaction.as_mut() else { return FrameBuildBoundaryStep::Fault("frame chrome lost interaction state") };
                if interaction.shell.render_chrome_step(&mut cursor.chrome, draw, overlay, atlas, icons, &mut interaction.input, &interaction.theme, engine_resources, world_resources) {
                    cursor.phase = FrameBuildPhase::JobProgressTake;
                }
            }
            FrameBuildPhase::JobProgressTake => {
                #[cfg(not(target_arch = "wasm32"))]
                {
                    cursor.job_progress = kernel_runtime::take_job_progress_presentation();
                }
                cursor.phase = FrameBuildPhase::JobProgressOverlay;
            }
            FrameBuildPhase::JobProgressOverlay => {
                #[cfg(not(target_arch = "wasm32"))]
                if let Some(progress) = cursor.job_progress.as_ref() {
                    let (kind, applied) = progress.visual();
                    let color = match kind {
                        semio_framework_actor::JobProgressKind::CommitValidated => ui_wgpu::wgpu::Rgba::from_srgb8(36, 158, 91, 255),
                        semio_framework_actor::JobProgressKind::Cancelled | semio_framework_actor::JobProgressKind::Fault => ui_wgpu::wgpu::Rgba::from_srgb8(218, 74, 74, 255),
                        _ => ui_wgpu::wgpu::Rgba::from_srgb8(67, 132, 245, 255),
                    };
                    self.overlay.push_solid_overlay([12.0, 12.0, 24.0 + applied.min(100) as f32 * 2.0, 4.0], color);
                }
                cursor.phase = FrameBuildPhase::ResourceInput;
            }
            FrameBuildPhase::ResourceInput => {
                match ui_wgpu::wgpu::PreparedRenderInput::try_new(cursor.presentation_witness.scene_revision, cursor.presentation_witness.input_generation, ui_wgpu::wgpu::DrawList::empty(), None, 0.0) {
                    Ok(input) => cursor.resource_input = Some(input),
                    Err(rejected) => {
                        cursor.input_rejected = Some(rejected);
                        return FrameBuildBoundaryStep::Fault("frame prepared input process admission was refused");
                    }
                }
                cursor.phase = FrameBuildPhase::WorldTransfer;
            }
            FrameBuildPhase::WorldTransfer => {
                let Some(resources) = cursor.world_resources.as_mut() else { return FrameBuildBoundaryStep::Fault("frame transfer lost world resources") };
                let Some(input) = cursor.resource_input.as_mut() else { return FrameBuildBoundaryStep::Fault("frame transfer lost resource input") };
                match resources.append_step(input) {
                    Ok(true) => cursor.phase = FrameBuildPhase::IconTransfer,
                    Ok(false) => {}
                    Err(rejected) => {
                        cursor.world_rejected = Some(rejected);
                        return FrameBuildBoundaryStep::Fault("frame world resource admission exceeded fixed credits");
                    }
                }
            }
            FrameBuildPhase::IconTransfer => {
                if let Some(upload) = cursor.icon_upload.take() {
                    let Some(input) = cursor.resource_input.as_mut() else { return FrameBuildBoundaryStep::Fault("frame icon transfer lost resource input") };
                    if let Err(upload) = input.try_push_upload(upload) {
                        cursor.world_rejected = Some(infinite_world::world::World3dBuildRejected::Upload(upload));
                        return FrameBuildBoundaryStep::Fault("frame icon upload fixed admission was refused");
                    }
                }
                cursor.phase = FrameBuildPhase::EngineTransfer;
            }
            FrameBuildPhase::EngineTransfer => {
                let Some(resources) = cursor.engine_resources.as_mut() else { return FrameBuildBoundaryStep::Fault("frame transfer lost engine resources") };
                match resources.take_packet_step() {
                    Ok(Some(packet)) => {
                        if let Err(rejected) = cursor.engine_packets.try_push(packet) {
                            cursor.engine_rejected = Some(rejected);
                            return FrameBuildBoundaryStep::Fault("frame engine packet credits exceeded");
                        }
                    }
                    Ok(None) => cursor.phase = FrameBuildPhase::CursorWake,
                    Err(packet) => {
                        cursor.engine_rejected = Some(packet);
                        return FrameBuildBoundaryStep::Fault("frame engine packet producer exceeded fixed credits");
                    }
                }
            }
            FrameBuildPhase::CursorWake => {
                let Some(resources) = cursor.world_resources.as_mut() else { return FrameBuildBoundaryStep::Fault("frame cursor wake lost world resources") };
                match resources.take_cursor_wake() {
                    Ok(token) => cursor.cursor_wake = token,
                    Err(_) => return FrameBuildBoundaryStep::Fault("frame cursor wake authority faulted"),
                }
                cursor.phase = FrameBuildPhase::Complete;
            }
            FrameBuildPhase::Complete => {
                if cursor.engine_resources.as_ref().is_some_and(|resources| !resources.terminal_is_empty()) || cursor.world_resources.as_ref().is_some_and(|resources| !resources.terminal_is_empty()) {
                    return FrameBuildBoundaryStep::Fault("frame build resource cursor was not terminal-empty");
                }
                let Some(resource_input) = cursor.resource_input.take() else { return FrameBuildBoundaryStep::Fault("frame build lost resource input") };
                let Some(deferred_actions) = cursor.deferred_actions.take() else { return FrameBuildBoundaryStep::Fault("frame build lost deferred action owners") };
                let engine_packets = std::mem::take(&mut cursor.engine_packets);
                return FrameBuildBoundaryStep::Complete(AppFrameAfterChrome {
                    resource_input: Some(resource_input),
                    upload_rejected: None,
                    draw_rejected: None,
                    engine_packets: Some(engine_packets),
                    deferred_actions,
                    fullscreen: cursor.fullscreen.take(),
                    cursor_wake: cursor.cursor_wake.take(),
                    #[cfg(not(target_arch = "wasm32"))]
                    job_progress: cursor.job_progress.take(),
                    retirement: None,
                });
            }
        }
        FrameBuildBoundaryStep::Pending
    }

    /// 🎞️ Advances one retained post-input owner transfer without driving deferred work inline.
    fn frame_after_input_step(&mut self, runtime: &RuntimeMailbox, partial: &mut AppFrameAfterChrome, cursor: &mut FrameFinishCursor, cancel: semio_framework_job::CancelToken) -> FrameFinishBoundaryStep {
        match cursor.phase {
            FrameFinishPhase::Inputs => {
                runtime.update_frame_inputs(self);
                cursor.phase = FrameFinishPhase::Deferred;
            }
            FrameFinishPhase::Deferred => {
                cursor.deferred_actions = Some(std::mem::take(&mut partial.deferred_actions));
                cursor.flush_tutorial = !self.shell.tutorial_pending_document_ops.is_empty();
                cursor.shell_maintenance = self.shell.chrome_maintenance_pending();
                #[cfg(not(target_arch = "wasm32"))]
                {
                    cursor.pump_sync = app_now_ms() - self.last_sync_pump_ms >= 100.0;
                    if cursor.pump_sync {
                        self.last_sync_pump_ms = app_now_ms();
                    }
                }
                cursor.phase = FrameFinishPhase::GlyphUpload;
            }
            FrameFinishPhase::GlyphUpload => {
                if !cursor.glyph_started {
                    if !self.atlas.take_dirty() {
                        cursor.phase = FrameFinishPhase::Cursor;
                        return FrameFinishBoundaryStep::Pending;
                    }
                    let Some(input) = partial.resource_input.as_ref() else { return FrameFinishBoundaryStep::Fault("frame glyph transfer lost resource input") };
                    let Some(next) = input.raster_producers.len().checked_add(input.uploads.len()).and_then(|count| count.checked_add(1)) else { return FrameFinishBoundaryStep::Fault("frame glyph upload credits exhausted") };
                    if next > input.limits.max_upload_items {
                        return FrameFinishBoundaryStep::Fault("frame glyph upload credits exceeded");
                    }
                    let Ok(pages) = ui_wgpu::wgpu::PreparedAtlasPages::try_new(self.atlas.width, self.atlas.height, 1, self.atlas.pixels.len()) else {
                        return FrameFinishBoundaryStep::Fault("frame glyph atlas page admission failed");
                    };
                    cursor.glyph_pages = Some(pages);
                    cursor.glyph_started = true;
                    return FrameFinishBoundaryStep::Pending;
                }
                let Some(pages) = cursor.glyph_pages.as_mut() else { return FrameFinishBoundaryStep::Fault("frame glyph atlas pages lost ownership") };
                let complete = match pages.push_page(&self.atlas.pixels, pages.next_row()) {
                    Ok(complete) => complete,
                    Err(_) => return FrameFinishBoundaryStep::Fault("frame glyph atlas page construction faulted"),
                };
                if !complete {
                    return FrameFinishBoundaryStep::Pending;
                }
                let Some(input) = partial.resource_input.as_mut() else { return FrameFinishBoundaryStep::Fault("frame glyph transfer lost resource input") };
                let Some(pages) = cursor.glyph_pages.take() else { return FrameFinishBoundaryStep::Fault("frame glyph atlas pages lost ownership") };
                if let Err(upload) = input.try_push_upload(ui_wgpu::wgpu::PreparedRenderUpload::GlyphAtlasPages { pixels: pages }) {
                    partial.upload_rejected = Some(upload);
                    return FrameFinishBoundaryStep::Fault("frame glyph upload fixed admission was refused");
                }
                cursor.phase = FrameFinishPhase::Cursor;
            }
            FrameFinishPhase::Cursor => {
                let hit = self.input.hit_at(self.last_pointer_x, self.last_pointer_y);
                let base_cursor = resolve_semio_cursor(
                    hit,
                    CursorDragState {
                        tree_drag: self.shell.tree_drag.is_some(),
                        dock_drag: self.shell.dock_drag.is_some(),
                        pointer_drag_active: self.input.drag.active,
                        pointer_drag_axis: self.input.drag.axis,
                        pointer_drag_kind: self.input.drag.kind,
                    },
                );
                cursor.cursor = match self.shell.utility_cursor_override(self.last_pointer_x, self.last_pointer_y) {
                    Some(utility_cursor) if matches!(base_cursor, SemioCursor::Default | SemioCursor::Grab | SemioCursor::Selectable | SemioCursor::Pointer) => utility_cursor,
                    _ => base_cursor,
                };
                cursor.phase = FrameFinishPhase::Draw;
            }
            FrameFinishPhase::Draw => {
                let Some(input) = partial.resource_input.as_mut() else { return FrameFinishBoundaryStep::Fault("frame draw transfer lost resource input") };
                input.time_seconds = (app_now_ms() / 1000.0) as f32;
                cursor.phase = FrameFinishPhase::Overlay;
            }
            FrameFinishPhase::Overlay => {
                let Some(input) = partial.resource_input.as_mut() else { return FrameFinishBoundaryStep::Fault("frame overlay transfer lost resource input") };
                let draw = std::mem::replace(&mut self.draw, DrawList::empty());
                let overlay = Some(std::mem::replace(&mut self.overlay, DrawList::empty()));
                if let Err(rejected) = input.try_bind_draw(draw, overlay) {
                    partial.draw_rejected = Some(rejected);
                    return FrameFinishBoundaryStep::Fault("frame draw process admission was refused");
                }
                cursor.phase = FrameFinishPhase::Complete;
            }
            FrameFinishPhase::Complete => {
                let Some(deferred_actions) = cursor.deferred_actions.take() else { return FrameFinishBoundaryStep::Fault("frame completion lost deferred action owners") };
                let has_deferred = cursor.pump_sync || !deferred_actions.is_empty() || cursor.flush_tutorial || cursor.shell_maintenance;
                if has_deferred && self.pending_frame_deferred.is_some() {
                    cursor.deferred_actions = Some(deferred_actions);
                    return FrameFinishBoundaryStep::Fault("frame completion found an unclosed deferred owner");
                }
                let Some(input) = partial.resource_input.take() else {
                    cursor.deferred_actions = Some(deferred_actions);
                    return FrameFinishBoundaryStep::Fault("frame completion lost resource input");
                };
                let Some(engine_packets) = partial.engine_packets.take() else {
                    partial.resource_input = Some(input);
                    cursor.deferred_actions = Some(deferred_actions);
                    return FrameFinishBoundaryStep::Fault("frame completion lost engine packets");
                };
                if has_deferred {
                    self.pending_frame_deferred = Some(FrameDeferredCursor::new(deferred_actions, cursor.pump_sync, cursor.flush_tutorial, cursor.shell_maintenance, input.preview_generation, cancel));
                }
                return FrameFinishBoundaryStep::Complete(AppFrameBuild {
                    generation: semio_framework_trace::Generation(input.preview_generation),
                    input,
                    engine_packets,
                    cursor: cursor.cursor,
                    theme_dark: self.theme_dark,
                    fullscreen: partial.fullscreen.take(),
                    cursor_wake: partial.cursor_wake.take(),
                    #[cfg(not(target_arch = "wasm32"))]
                    job_progress: partial.job_progress.take(),
                });
            }
        }
        FrameFinishBoundaryStep::Pending
    }
}

impl AppInteractionState {
    fn start_text_operation(&mut self, stream: u64, declared_bytes: usize) -> Result<(), String> {
        if self.text_streams.iter().any(|entry| entry.is_some_and(|entry| entry.id == stream)) {
            return Err("text stream was already started".to_string());
        }
        let slot = self.text_streams.iter().position(Option::is_none).ok_or_else(|| "text stream slots exhausted".to_string())?;
        let generation = self.input.text_buffer.generation();
        let token = self.input.text_buffer.begin(generation, declared_bytes, self.input.cursor_pos, self.input.cursor_pos).map_err(|fault| format!("text admission failed: {fault:?}"))?;
        self.text_streams[slot] = Some(AppTextStream { id: stream, token });
        Ok(())
    }

    fn push_text_operation(&mut self, stream: u64, text: String) -> Result<(), String> {
        let token = self.text_streams.iter().flatten().find(|entry| entry.id == stream).map(|entry| entry.token).ok_or_else(|| "text stream chunk arrived before start".to_string())?;
        self.input.text_buffer.push(token, text).map_err(|fault| format!("text chunk admission failed: {fault:?}"))
    }

    fn commit_text_operation(&mut self, stream: u64) -> Result<(), String> {
        let slot = self.text_streams.iter().position(|entry| entry.is_some_and(|entry| entry.id == stream)).ok_or_else(|| "text stream commit arrived before start".to_string())?;
        let token = self.text_streams[slot].expect("text stream slot").token;
        self.input.text_buffer.commit(token).map_err(|fault| format!("text stream commit failed: {fault:?}"))?;
        self.text_streams[slot] = None;
        Ok(())
    }

    fn abort_text_operation(&mut self, stream: u64) -> Result<(), String> {
        let slot = self.text_streams.iter().position(|entry| entry.is_some_and(|entry| entry.id == stream)).ok_or_else(|| "text stream abort arrived before start".to_string())?;
        let token = self.text_streams[slot].take().expect("text stream slot").token;
        self.input.text_buffer.abort(token).map_err(|fault| format!("text stream abort failed: {fault:?}"))
    }

    fn enqueue_text_operation(&mut self, text: String) -> Result<(), String> {
        let generation = self.input.text_buffer.generation();
        self.input.text_buffer.enqueue_owned(generation, text, self.input.cursor_pos, self.input.cursor_pos).map_err(|fault| format!("text admission failed: {fault:?}"))
    }

    fn cancel_text_operations(&mut self) {
        self.text_cancel_pending = true;
    }

    fn undo_text_operation(&mut self) -> bool {
        let Some(cursor) = self.input.text_buffer.undo() else { return false };
        self.input.cursor_pos = cursor.min(self.input.text_buffer.len());
        true
    }

    fn has_pending_text_work(&self) -> bool {
        self.input.text_buffer.reserved_bytes() != 0 || self.text_streams.iter().any(Option::is_some) || self.text_cancel_pending
    }

    fn drive_text_operation(&mut self) {
        if self.text_cancel_pending {
            let generation = self.input.text_buffer.generation();
            let _ = self.input.text_buffer.step(generation, 1, true);
            self.text_cancel_pending = self.input.text_buffer.reserved_bytes() != 0;
            return;
        }
        if let Err(fault) = self.input.drive_text_step() {
            self.text_fault = Some(format!("text edit step failed: {fault:?}"));
        }
    }

    fn resize(&mut self, css_width: f32, css_height: f32, dpr: f32) {
        self.shell.screen_w = (css_width * dpr).max(1.0);
        self.shell.screen_h = (css_height * dpr).max(1.0);
    }

    async fn handle_key(&mut self, action: KeyAction, modifiers: PointerModifiers) {
        if let KeyAction::Space(pressed) = &action {
            if self.shell.context_menu.is_some() && *pressed {
                if let Err(err) = self.shell.handle_keyboard_async(KeyAction::Space(true), &modifiers, &mut self.input).await {
                    log_debug(&format!("keyboard failed: {err}"));
                }
                return;
            }
            self.space_pressed = *pressed;
            return;
        }
        if engine_canvas::node_graph_apply_note_edit_key(action.clone(), &modifiers) {
            return;
        }
        // 🔌️ w2-input-wiring: spawns the ASYNC `handle_keyboard_async` (mirrors this fn's own
        // `on_button`/`on_move` sibling callbacks above, and the `spawn_app_task` pattern this fn
        // used to hand-roll just for search/find-Enter-activation) instead of calling the sync
        // `handle_keyboard` directly. Before this fix `handle_keyboard_async` was entirely dead code
        // (see `report-w3-shell-input-cutover.md`'s "MAJOR FINDING"): the P4 app-keybinding dispatch,
        // P5 idle-Escape-deactivates-utility, and — worst — committing a focused `Input`'s typed text
        // via Enter/Escape never fired. `handle_keyboard_async`'s own top already reimplements the
        // exact search/find-Enter-activation this fn used to hand-duplicate around the sync call, so
        // that duplication is gone, not just moved.
        if let Err(err) = self.shell.handle_keyboard_async(action, &modifiers, &mut self.input).await {
            log_debug(&format!("keyboard failed: {err}"));
        }
    }

    async fn handle_pointer_button(&mut self, x: f32, y: f32, down: bool, button: i16, modifiers: PointerModifiers) {
        self.last_pointer_x = x;
        self.last_pointer_y = y;
        self.pointer_down = down;
        self.pointer_button = button;
        self.modifiers = modifiers.clone();
        if !down {
            let map_had_active_drag = self.shell.tiled_map_states.keys().any(|surface_id| scenes::tiled_map_drag_active(surface_id));
            for (surface_id, surface) in &self.shell.tiled_map_states {
                if !surface.bounds.contains(x, y) && !scenes::tiled_map_drag_active(surface_id) {
                    continue;
                }
                if let Err(fault) = scenes::tiled_map_pointer_up_into(surface_id, &surface.controller_id, surface.bounds, x, y, &mut self.input) {
                    self.input.record_action_fault(fault);
                    return;
                }
            }
            let board_had_active_drag = self.shell.board2d_states.keys().any(|surface_id| scenes::board2d_drag_active(surface_id));
            for (surface_id, surface) in &self.shell.board2d_states {
                if !surface.bounds.contains(x, y) && !scenes::board2d_drag_active(surface_id) {
                    continue;
                }
                if let Err(fault) = scenes::puzzle_board_pointer_up_into(surface_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, &mut self.input) {
                    self.input.record_action_fault(fault);
                    return;
                }
            }
            let board_consumed = self.shell.board2d_states.values().any(|surface| surface.bounds.contains(x, y)) || board_had_active_drag;
            let map_consumed = self.shell.tiled_map_states.values().any(|surface| surface.bounds.contains(x, y)) || map_had_active_drag;
            if map_consumed || board_consumed {
                return;
            }
            if let Err(err) = self.shell.handle_pointer_button(x, y, down, button, &mut self.input, &self.theme).await {
                log_debug(&format!("pointer failed: {err}"));
            }
            let mut world_consumed = false;
            for state in self.shell.world3d_states.values_mut() {
                if !state.bounds.contains(x, y) {
                    continue;
                }
                world_consumed = true;
                if enqueue_world3d_event(state, WorldInteractionIntent::pointer_button(x, y, down, button, &modifiers)).is_err() {
                    self.input.record_action_fault(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                    return;
                }
            }
            if world_consumed {
                return;
            }
            for (surface_id, surface) in &self.shell.node_graph_states {
                if !surface.bounds.contains(x, y) {
                    continue;
                }
                if let Err(fault) = engine_canvas::node_graph_pointer_up_into(surface_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, &mut self.input) {
                    self.input.record_action_fault(fault);
                    return;
                }
            }
            return;
        }
        let mut world_consumed = false;
        for state in self.shell.world3d_states.values_mut() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            world_consumed = true;
            if enqueue_world3d_event(state, WorldInteractionIntent::pointer_button(x, y, down, button, &modifiers)).is_err() {
                self.input.record_action_fault(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                return;
            }
        }
        if world_consumed {
            return;
        }
        for (surface_id, surface) in &self.shell.node_graph_states {
            if !surface.bounds.contains(x, y) {
                continue;
            }
            if down {
                if let Err(fault) = engine_canvas::node_graph_pointer_down_into(surface_id, &surface.controller_id, surface.bounds, x, y, button, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, self.space_pressed, &mut self.input) {
                    self.input.record_action_fault(fault);
                    return;
                }
            } else {
                if let Err(fault) = engine_canvas::node_graph_pointer_up_into(surface_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, &mut self.input) {
                    self.input.record_action_fault(fault);
                    return;
                }
            }
        }
        let mut map_pointer_on_surface = false;
        for (surface_id, surface) in &self.shell.tiled_map_states {
            if !surface.bounds.contains(x, y) {
                continue;
            }
            map_pointer_on_surface = true;
            if down {
                if let Err(fault) = scenes::tiled_map_pointer_down_into(surface_id, &surface.controller_id, surface.bounds, x, y, button, modifiers.shift, modifiers.ctrl_or_meta(), &surface.selection_method, &mut self.input) {
                    self.input.record_action_fault(fault);
                    return;
                }
            }
        }
        if map_pointer_on_surface && (button == 0 || button == 1) {
            return;
        }
        let mut board_pointer_on_surface = false;
        for (surface_id, surface) in &self.shell.board2d_states {
            if !surface.bounds.contains(x, y) {
                continue;
            }
            board_pointer_on_surface = true;
            if down {
                scenes::puzzle_board_pointer_down(surface_id, surface.bounds, x, y, button, modifiers.shift, modifiers.ctrl_or_meta());
            }
        }
        if board_pointer_on_surface && (button == 0 || button == 1) {
            return;
        }
        if let Err(err) = self.shell.handle_pointer_button(x, y, down, button, &mut self.input, &self.theme).await {
            log_debug(&format!("pointer failed: {err}"));
        }
    }

    async fn handle_pointer_move(&mut self, x: f32, y: f32, down: bool, button: i16, modifiers: PointerModifiers) {
        let drag_dx = x - self.last_pointer_x;
        let drag_dy = y - self.last_pointer_y;
        self.last_pointer_x = x;
        self.last_pointer_y = y;
        self.pointer_down = down;
        self.pointer_button = button;
        self.modifiers = modifiers.clone();
        self.shell.handle_pointer_move(x, y, down, &mut self.input, &self.theme);
        if let Err(err) = self.shell.flush_deferred_actions().await {
            log_debug(&format!("deferred actions: {err}"));
        }
        for state in self.shell.world3d_states.values_mut() {
            if !state.bounds.contains(x, y) {
                continue;
            }
            if enqueue_world3d_event(state, WorldInteractionIntent::pointer_move(x, y, drag_dx, drag_dy, down, button, &modifiers)).is_err() {
                self.input.record_action_fault(ui_wgpu::wgpu::BoundedActionFault::ItemCredits);
                return;
            }
        }
        for (surface_id, surface) in &self.shell.node_graph_states {
            if surface.bounds.contains(x, y) {
                if let Err(fault) = engine_canvas::node_graph_pointer_move_into(surface_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, &mut self.input) {
                    self.input.record_action_fault(fault);
                    return;
                }
            }
        }
        for (surface_id, surface) in &self.shell.tiled_map_states {
            if !surface.bounds.contains(x, y) && !scenes::tiled_map_drag_active(surface_id) {
                continue;
            }
            if let Err(fault) = scenes::tiled_map_pointer_move_into(surface_id, &surface.controller_id, surface.bounds, x, y, down, &mut self.input) {
                self.input.record_action_fault(fault);
                return;
            }
        }
        for (surface_id, surface) in &self.shell.board2d_states {
            let inside = surface.bounds.contains(x, y);
            if inside {
                if let Err(fault) = scenes::puzzle_board_pointer_move_into(surface_id, &surface.controller_id, surface.bounds, x, y, modifiers.shift, modifiers.ctrl_or_meta(), modifiers.alt, &mut self.input) {
                    self.input.record_action_fault(fault);
                    return;
                }
            } else {
                if let Err(fault) = scenes::puzzle_board_pointer_leave_into(surface_id, &surface.controller_id, modifiers.alt, &mut self.input) {
                    self.input.record_action_fault(fault);
                    return;
                }
            }
        }
    }
}

//#region 🔖️OsHostDecomposition — SemioApp deletion
// 🏚️ DELETED by ticket 26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (packet os-host):
// `start_frame_loop` (used to live here, ~line 2291 pre-edit — the recursive `schedule_frame`
// rAF/timer chain that called `app.frame()` and immediately rescheduled itself, unconditionally,
// forever), `enum HostUserEvent` and `struct SemioApp` + its `ApplicationHandler` impl (`resumed`
// set `ControlFlow::Poll` at boot — ~line 2383 pre-edit; `window_event`'s `RedrawRequested` arm
// called `window.request_redraw()` unconditionally right after building a frame — ~line 2406
// pre-edit; `about_to_wait` polled a thread-local task pool then ALSO unconditionally
// `window.request_redraw()` every single iteration — ~line 2416-2424 pre-edit). Replaced by
// `winit_app::{HostUserEvent, WinitApp}` — same two-phase boot handshake, but steady-state control
// flow is `WaitUntil(next deadline)`/`Wait`, redraw only fires `if let Some(reason) =
// scheduler.should_render(now)`, while native continuations run on the process pool. See
// `📓️terra-os-host-report.md`'s redraw audit for the full
// before/after per site.
//#endregion 🔖️OsHostDecomposition — SemioApp deletion

async fn boot_runtime(
    window: Arc<Window>,
    plugin_filter: String,
    #[cfg(target_arch = "wasm32")] plugins: Option<wasm_bindgen::JsValue>,
    #[cfg(not(target_arch = "wasm32"))] plugin_modules_root: std::path::PathBuf,
) -> Result<(RuntimeMailbox, AppPresenter), String> {
    let dpr = window.scale_factor() as f32;
    let size = window.inner_size();
    #[cfg(target_arch = "wasm32")]
    let (css_width, css_height, dpr) = {
        use winit::platform::web::WindowExtWebSys;
        let dpr = web_sys::window().map(|host| host.device_pixel_ratio() as f32).unwrap_or(dpr);
        if let Some(canvas) = window.canvas() {
            let css_width = canvas.client_width().max(1) as f32;
            let css_height = canvas.client_height().max(1) as f32;
            canvas.set_width((css_width * dpr) as u32);
            canvas.set_height((css_height * dpr) as u32);
            (css_width, css_height, dpr)
        } else {
            (size.width as f32 / dpr, size.height as f32 / dpr, dpr)
        }
    };
    #[cfg(not(target_arch = "wasm32"))]
    let css_width = size.width as f32 / dpr;
    #[cfg(not(target_arch = "wasm32"))]
    let css_height = size.height as f32 / dpr;

    const ANTA_LATIN: &[u8] = include_bytes!("../../../../../../../../🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf");
    let font_bytes = match fetch_font_bytes("/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf").await {
        Ok(bytes) if bytes.len() > 256 => bytes,
        _ => ANTA_LATIN.to_vec(),
    };
    let atlas = FontAtlas::from_bytes(&font_bytes).map_err(|err| format!("atlas failed: {err}"))?;
    let icons = icon_atlas::build_icon_atlas();
    let mut gpu = GpuContext::from_window(window.clone()).await.map_err(|err| format!("gpu init failed: {err}"))?;
    gpu.resize(css_width, css_height, dpr);
    gpu.upload_font_atlas(&atlas);
    gpu.upload_icon_atlas(&icons);

    #[cfg(target_arch = "wasm32")]
    let entries = {
        let plugins = plugins.ok_or("missing wasm programs")?;
        filter_plugins(parse_plugin_entries(plugins).map_err(|err| format!("program parse failed: {err}"))?, &plugin_filter)
    };
    #[cfg(not(target_arch = "wasm32"))]
    let entries = filter_plugins(load_wasm_plugins(&plugin_filter, &plugin_modules_root).await?, &plugin_filter);

    let mut shell = ShellState::new(entries, plugin_filter.clone());
    shell.screen_w = css_width * dpr;
    shell.screen_h = css_height * dpr;
    shell.boot().await.map_err(|err| format!("shell boot failed: {err}"))?;

    let runtime = RuntimeMailbox::new(AppRuntime {
        atlas,
        icons,
        interaction: Some(AppInteractionState {
            shell,
            input: InputState::default(),
            theme: Theme::default(),
            theme_dark: appearance_is_dark("system"),
            last_pointer_x: 0.0,
            last_pointer_y: 0.0,
            pointer_down: false,
            pointer_button: 0,
            modifiers: PointerModifiers::default(),
            wheel_delta: 0.0,
            space_pressed: false,
            wheel_zoom_deadline_ms: 0.0,
            caret_blink_at_ms: 0.0,
            caret_blink_visible: true,
            text_streams: std::array::from_fn(|_| None),
            text_fault: None,
            frame_fault: None,
            text_cancel_pending: false,
            #[cfg(not(target_arch = "wasm32"))]
            last_sync_pump_ms: 0.0,
        }),
        draw: DrawList::default(),
        overlay: DrawList::default(),
        pending_frame_deferred: None,
        #[cfg(not(target_arch = "wasm32"))]
        pending_frame_maintenance_refusal: None,
        #[cfg(not(target_arch = "wasm32"))]
        plugin_modules_root: plugin_modules_root.clone(),
        #[cfg(not(target_arch = "wasm32"))]
        native_plugin_mtimes: std::collections::HashMap::new(),
        #[cfg(not(target_arch = "wasm32"))]
        native_hot_swap_scan: None,
        #[cfg(not(target_arch = "wasm32"))]
        native_hot_swap_modified: None,
        #[cfg(not(target_arch = "wasm32"))]
        native_hot_swap_cursor: 0,
        #[cfg(not(target_arch = "wasm32"))]
        native_reload_pending: false,
    });
    let presenter = AppPresenter {
        gpu,
        engine: engine_canvas::EngineCanvasPresenter::default(),
        gate: ui_wgpu::wgpu::PreparedRenderGate::default(),
        presentation_authority: runtime.presentation_authority(),
        raster_operation_authority: runtime.raster_operation_authority(),
        window: Some(window.clone()),
        #[cfg(target_arch = "wasm32")]
        offscreen_token: None,
        last_cursor: None,
        pending: None,
        retirement: None,
        retained_fault: None,
        surface_resize: None,
    };

    // 🧹️ P3c: this used to build a `PointerCallbacks` here (5 `Rc<RefCell<AppRuntime>>` clones, one
    // per input kind) and hand it back alongside `runtime`. `winit_app.rs`'s own `HostUserEvent` doc
    // comment records that its one caller stopped using it at the P3a enqueue-only
    // `WindowDelegate`/`dispatch_normalized_event` cutover -- `boot_runtime` was left constructing it
    // anyway because touching this signature wasn't that packet's job. It is
    // this packet's job (removing `self_weak`, see this crate's own `AppHandle` doc comment), and per
    // AGENTS.md's no-legacy-code rule, dead construction is deleted outright. Right-click remains a
    // lossless `DispatchEvent::PointerDown { button: Secondary }` in the enqueue-only contract;
    // `winit_app::dispatch_normalized_event` maps it to button `2` and calls the canonical
    // `handle_pointer_button`, whose Shell path opens the context menu. The redundant callbacks-only
    // `handle_context_menu` wrapper is deleted with its sole caller. See `📓️p3c-explicit-app-handle.md`.
    log_debug("wgpu renderer booted");
    Ok((runtime, presenter))
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
struct NativeSocketProbeSnapshot(String);

#[cfg(not(target_arch = "wasm32"))]
impl store::os_store::ArtifactDsl for NativeSocketProbeSnapshot {
    const EXTENSION: &'static str = "native-socket-probe";

    fn parse_dsl(text: &str) -> Result<Self, store::os_store::TextError> {
        Ok(Self(text.to_string()))
    }

    fn print_dsl(&self) -> String {
        self.0.clone()
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl store::os_store::ArtifactPack for NativeSocketProbeSnapshot {
    fn encode_pack_with(&self, _options: &store::os_store::PackEncodeOptions) -> Result<Vec<u8>, store::os_store::PackError> {
        Ok(self.0.as_bytes().to_vec())
    }

    fn decode_pack_with(bytes: &[u8], _options: &store::os_store::PackDecodeOptions) -> Result<Self, store::os_store::PackError> {
        String::from_utf8(bytes.to_vec()).map(Self).map_err(|error| store::os_store::PackError::Schema(error.to_string()))
    }

    fn record_spec() -> Option<store::os_dsl::RecordSpec> {
        Some(store::os_dsl::RecordSpec::new(
            Some("native-socket-probe"),
            store::os_dsl::RecordLayout::Inline,
            vec![store::os_dsl::FieldSpec::new(0, "value", store::os_dsl::Shape::Value)],
        ))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl store::os_store::ToValue for NativeSocketProbeSnapshot {
    fn to_value(&self) -> store::os_store::DslValue {
        store::os_store::to_dsl_value(self).expect("native probe snapshot value")
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl store::os_store::FromValue for NativeSocketProbeSnapshot {
    fn from_value(value: store::os_store::DslValue) -> Result<Self, store::os_store::ValueError> {
        store::os_store::from_dsl_value(value).map_err(store::os_store::ValueError::new)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, serde::Deserialize)]
struct NativeSocketProbeDiff(String);

#[cfg(not(target_arch = "wasm32"))]
impl store::os_store::MutationDiff<NativeSocketProbeSnapshot> for NativeSocketProbeDiff {
    fn apply(&self, _base: &NativeSocketProbeSnapshot) -> store::os_store::MutationApplyResult<NativeSocketProbeSnapshot> {
        Ok(NativeSocketProbeSnapshot(self.0.clone()))
    }

    fn absorb(&mut self, other: Self) {
        self.0 = other.0;
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl store::os_store::ToValue for NativeSocketProbeDiff {
    fn to_value(&self) -> store::os_store::DslValue {
        store::os_store::to_dsl_value(self).expect("native probe diff value")
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl store::os_store::FromValue for NativeSocketProbeDiff {
    fn from_value(value: store::os_store::DslValue) -> Result<Self, store::os_store::ValueError> {
        store::os_store::from_dsl_value(value).map_err(store::os_store::ValueError::new)
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
enum NativeSocketProbeMutation {
    Set(String),
}

#[cfg(not(target_arch = "wasm32"))]
const NATIVE_SOCKET_PROBE_MUTATION_DESCRIPTOR: store::os_store::MutationLeafDescriptor = store::os_store::MutationLeafDescriptor {
    schema_version: 1,
    owner: "framework/os/renderer/wgpu/native-socket-probe",
    semantic_kind: "set",
    display_name: "Set",
    emoji: "🧪",
    aggregate_variant: "Set",
    payload_schema: "native.socket-grant.probe/v1",
    text_opcode: None,
    binary_tag: None,
    invertibility: store::os_store::MutationInvertibility::ExplicitMutation,
    diff_participation: store::os_store::MutationDiffParticipation::Detect,
    outcome_classes: &[store::os_store::MutationOutcomeClass::Applied],
    composition: store::os_store::MutationComposition::Atomic,
    required_language_surfaces: &[store::os_store::MutationLanguageSurface::Rust],
};

#[cfg(not(target_arch = "wasm32"))]
impl store::os_store::Mutation<NativeSocketProbeSnapshot> for NativeSocketProbeMutation {
    type Diff = NativeSocketProbeDiff;
    const DESCRIPTORS: &'static [store::os_store::MutationLeafDescriptor] = &[NATIVE_SOCKET_PROBE_MUTATION_DESCRIPTOR];

    fn descriptor(&self) -> &'static store::os_store::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }

    fn diff(&self, _base: &NativeSocketProbeSnapshot) -> store::os_store::MutationOutcome<NativeSocketProbeDiff> {
        let Self::Set(value) = self;
        store::os_store::MutationOutcome::new(NativeSocketProbeDiff(value.clone()))
    }

    fn inverse(&self, base: &NativeSocketProbeSnapshot) -> Vec<Self> {
        vec![Self::Set(base.0.clone())]
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl store::os_store::ToValue for NativeSocketProbeMutation {
    fn to_value(&self) -> store::os_store::DslValue {
        store::os_store::to_dsl_value(self).expect("native probe mutation value")
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl store::os_store::FromValue for NativeSocketProbeMutation {
    fn from_value(value: store::os_store::DslValue) -> Result<Self, store::os_store::ValueError> {
        store::os_store::from_dsl_value(value).map_err(store::os_store::ValueError::new)
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl store::os_store::OpText for NativeSocketProbeMutation {
    fn print_op(&self) -> String {
        let Self::Set(value) = self;
        value.clone()
    }

    fn parse_op(line: &str) -> Result<Self, store::os_store::TextError> {
        Ok(Self::Set(line.to_string()))
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl store::os_store::OpBinary for NativeSocketProbeMutation {
    fn encode_op(&self) -> Result<Vec<u8>, store::os_store::ProtocolError> {
        let Self::Set(value) = self;
        Ok(value.as_bytes().to_vec())
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, store::os_store::ProtocolError> {
        String::from_utf8(bytes.to_vec()).map(Self::Set).map_err(|error| store::os_store::ProtocolError::Malformed { what: "native-socket-probe-op", offset: 0, detail: error.to_string() })
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn run_socket_grant_probe() -> i32 {
    use dsl::os_directory::client::{native::NativeDirectoryTransport, DirectoryClient, DocumentSocketSurfaceExpectationV1};
    use dsl::os_directory::{DocumentOpenRendererTargetV1, DocumentOpenSurfaceRoleV1};
    use dsl::os_directory::identity::claimed_local_hub_credential;
    use dsl::os_spr::{ActorId, ArtifactDiff, ArtifactId, HybridLogicalTimestamp, InverseMutation, MutationEnvelope, MutationId, SchemaId};
    use dsl::os_store::sync::{ArtifactActorConfig, ArtifactActorMsg, ArtifactDocumentKey, ArtifactEvent, ArtifactHost, CommandAckOutcome, PersistenceBinding};
    use semio_framework_actor::{ActorId as DirectoryActorId, PackageId as DirectoryPackageId};
    use semio_framework_async::{HostAsyncRuntime, ScopeOwner};
    use semio_framework_os_services::{ComputePool, TokioHostRuntime};

    const PROBE_SCHEMA: &str = "native.socket-grant.probe/v1";
    let Some(credential) = claimed_local_hub_credential("native") else { return 1 };
    let _ = dsl::os_store::register_document_codec(dsl::os_store::ArtifactCodec::of::<NativeSocketProbeSnapshot, NativeSocketProbeMutation>(PROBE_SCHEMA)).await;
    let pool = renderer_worker_pool();
    let runtime = Arc::new(TokioHostRuntime::with_pool(pool.clone()));
    let scope = runtime.open_scope_now(ScopeOwner::Service("native_socket_grant_probe"), None);
    let compute = Arc::new(ComputePool::with_pool(2, pool.clone()));
    let transport = NativeDirectoryTransport::with_new_http_pool_now(runtime, scope, compute, 1_000_000, 2, DirectoryPackageId("native.socket-grant-probe".into()), DirectoryActorId(0));
    let client = Arc::new(DirectoryClient::authenticated(transport, credential.clone()));
    let host = ArtifactHost::new(Arc::new(pool));
    if host.local_hub_ready() {
        return 1;
    }
    host.set_local_hub_credential(credential.clone());
    if host.local_hub_ready() {
        return 1;
    }
    host.set_hub_socket_grant_source(client);
    if !host.local_hub_ready() {
        return 1;
    }
    let document_id = "probe-document";
    let document_key = ArtifactDocumentKey::hub("probe-space", document_id);
    if !host.set_document_socket_surface(
        &document_key,
        DocumentSocketSurfaceExpectationV1 {
            artifact_kind: "native.socket-grant.probe".into(),
            plugin_id: "native.socket-grant.probe".into(),
            package_id: "native.socket-grant.probe.codec".into(),
            version: "1.0.0".into(),
            surface_id: "native.socket-grant.probe.editor".into(),
            app_id: "native.socket-grant.probe.app".into(),
            window_kind_id: "native.socket-grant.probe.window".into(),
            role: DocumentOpenSurfaceRoleV1::Editor,
            renderer_target: DocumentOpenRendererTargetV1::Wgpu,
        },
    ) {
        return 1;
    }
    let channels = host
        .open(ArtifactActorConfig {
            document_id: document_id.into(),
            schema: PROBE_SCHEMA.into(),
            bindings: vec![PersistenceBinding::Hub {
                base_url: credential.hub_origin().into(),
                space_id: "probe-space".into(),
                surface: Some("native.socket-grant.probe.editor".into()),
            }],
            watch_external: false,
            actor: "untrusted-local-probe".into(),
        })
        .await;
    let mut events = host.subscribe_key(&channels.document_key).await;
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(12);
    let mut sessions = 0u8;
    let mut acknowledgements = 0u8;
    let envelope = |sequence: u8| MutationEnvelope {
        mutation_id: MutationId(format!("probe-{sequence}")),
        document_id: ArtifactId(document_id.into()),
        actor: ActorId("untrusted-local-probe".into()),
        dependencies: Vec::new(),
        diff: ArtifactDiff { schema: SchemaId(PROBE_SCHEMA.into()), payload: vec![sequence] },
        inverse: InverseMutation { schema: SchemaId(PROBE_SCHEMA.into()), payload: Vec::new() },
        timestamp: HybridLogicalTimestamp::new(0, 0),
    };
    if channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: vec![envelope(1)] }).is_err() {
        host.close_key(&channels.document_key);
        return 1;
    }
    while std::time::Instant::now() < deadline {
        match events.try_recv() {
            Ok(ArtifactEvent::Session { .. }) => {
                sessions += 1;
                if sessions == 2 && channels.cmd_tx.send(ArtifactActorMsg::LocalMutations { envelopes: vec![envelope(2)] }).is_err() {
                    break;
                }
            }
            Ok(ArtifactEvent::CommandOutcome { outcome: CommandAckOutcome::Accepted, .. }) => {
                acknowledgements += 1;
                if acknowledgements == 2 {
                    host.close_key(&channels.document_key);
                    return 0;
                }
            }
            Ok(_) | Err(tokio::sync::broadcast::error::TryRecvError::Empty) | Err(tokio::sync::broadcast::error::TryRecvError::Lagged(_)) => {}
            Err(tokio::sync::broadcast::error::TryRecvError::Closed) => break,
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    host.close_key(&channels.document_key);
    1
}

#[cfg(not(target_arch = "wasm32"))]
pub fn run_native(plugin_filter: &str, plugin_modules_root: std::path::PathBuf) {
    let event_loop = EventLoop::<winit_app::HostUserEvent>::with_user_event().build().expect("event loop");
    let proxy = event_loop.create_proxy();
    let mut app = winit_app::WinitApp::new(proxy, plugin_filter.to_string(), plugin_modules_root);
    let _ = event_loop.run_app(&mut app);
}

/// 🧪️ ticket 26/08/17/FINISH-HUB-SPACES-COLLABORATION-END-TO-END — headless smoke mode: boots
/// `ShellState` against a live hub (`S_HUB_URL`/`S_USER`/`S_DATA_DIR`) with NO GPU/window at all —
/// `GpuContext`/`winit`/`AppRuntime` are the only GPU-coupled pieces in this crate and this mode never
/// touches any of them, since `ShellState` itself is renderer-agnostic (chrome painting is a separate
/// concern layered on top by `AppRuntime::frame`). Boots, waits (bounded) for identity to mint/restore
/// and the initial directory fold to land, then dumps the Home window's widget tree + a small identity/
/// session summary as JSON to stdout and returns an exit code. An honest, explicit substitute for
/// driving a real window when this environment cannot open one (lane 3-D's brief proposed exactly this
/// shape). Returns `0` on a clean boot+dump, `1` on any hard failure along the way.
#[cfg(not(target_arch = "wasm32"))]
pub async fn run_smoke(plugin_filter: &str, plugin_modules_root: std::path::PathBuf) -> i32 {
    let loaded = match load_wasm_plugins(plugin_filter, &plugin_modules_root).await {
        Ok(entries) => entries,
        Err(error) => {
            eprintln!("smoke: load_wasm_plugins failed: {error}");
            return 1;
        }
    };
    let entries = filter_plugins(loaded, plugin_filter);
    let mut shell = ShellState::new(entries, plugin_filter.to_string());
    if let Err(error) = shell.boot().await {
        eprintln!("smoke: shell.boot() failed: {error}");
        return 1;
    }
    // 🪪️ Identity mint/restore runs on a background OS thread (contract §C3: never blocks
    // `boot()` itself) — poll the same every-frame pump the real render loop uses (drains the
    // identity bootstrap channel + the directory stream + folds any pending events) for up to 5s
    // so a real hub round trip has time to land before the dump.
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
    while std::time::Instant::now() < deadline {
        shell.pump_sync_events().await;
        if shell.identity.is_some() || shell.identity_env.is_none() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    let _ = shell.refresh_ui().await;
    let identity_summary = shell.identity.as_ref().map(|identity| serde_json::json!({ "userId": identity.user_id, "email": identity.email, "hubBaseUrl": identity.hub_base_url }));
    let window_documents: Vec<_> = shell.window_ui.iter().map(|(window, document)| serde_json::json!({ "window": window, "generation": document.generation() })).collect();
    let report = serde_json::json!({
        "booted": true,
        "identity": identity_summary,
        "identityOffline": shell.identity_offline,
        "openSpaceId": shell.open_space_id,
        "session": shell.session.as_ref().map(|session| serde_json::json!({ "pluginId": session.plugin_id, "appId": session.app.id, "role": format!("{:?}", session.app.role) })),
        "windowDocuments": window_documents,
    });
    match serde_json::to_string_pretty(&report) {
        Ok(json) => {
            println!("{json}");
            0
        }
        Err(error) => {
            eprintln!("smoke: report encode failed: {error}");
            1
        }
    }
}

/// 🐚️ Multi-mount: takes an already-created, already-placed canvas from the caller instead of looking
/// up a hardcoded `#root`/`#semio-wgpu-canvas` and taking it over via `set_inner_html("")` — that
/// single-mount assumption meant a second boot call would wipe the first mount's canvas and collide on
/// the same DOM id. The caller (`bootFrameworkOsWgpu` in `🟦️.ts`) now owns creating and placing
/// the canvas, so N independent mounts can coexist on one page.
///
/// Known gap (not yet done — see the plan's Wave 6 D11 notes), **narrowed but not closed** by ticket
/// 26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (packet os-host): the independent, uncancellable
/// `start_frame_loop`/`schedule_frame` recursive `requestAnimationFrame` chain this comment used to
/// describe is DELETED — every redraw now goes through `winit`'s own event loop
/// (`event_loop.spawn_app(app)` below → `winit_app::WinitApp`), and `WindowEvent::CloseRequested`
/// already calls `ActiveEventLoop::exit()`. Whether `exit()` alone now fully tears down a wasm mount
/// (winit's own wasm backend's post-`exit()` behaviour) is UNVERIFIED — this crate still does not
/// build clean (U4, `📓️terra-os-host-report.md`), so a real `semioWgpuUnmount` handle remains
/// deferred, but the mechanism it would need to cancel no longer exists in its old shape.
/// The dozen-plus `thread_local!` globals further up this file (`UI_ENGINE`, `ENGINE_SURFACES`,
/// `SCENE_STATE`, tooltip/dialog/tour chrome state, clipboard mocks, prefs, image-fetch caches, …) are
/// also still page-global, not per-mount — two simultaneous wgpu mounts each render on their own
/// independent GPU device/queue/surface (real, working isolation), but would still cross-talk on shared
/// UI chrome auxiliary state (a tooltip or dialog opened in one mount could show in the other).
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = uploadIconAtlas)]
pub fn upload_icon_atlas(width: u32, height: u32, pixels: &[u8], entries_json: &str) -> Result<(), JsValue> {
    let entries_map: std::collections::HashMap<String, [f32; 4]> = serde_json::from_str(entries_json).map_err(|err| JsValue::from_str(&format!("icon entries parse: {err}")))?;
    let entries: Vec<(String, [f32; 4])> = entries_map.into_iter().collect();
    ICON_ATLAS_RUNTIME.with(|cell| {
        cell.borrow_mut().replace(IconAtlas::from_packed(width, height, pixels.to_vec(), entries));
    });
    Ok(())
}

thread_local! {
    static ICON_ATLAS_RUNTIME: RefCell<Option<IconAtlas>> = RefCell::new(None);
}

//#region 🔖️RoleBoot
// 👁️✏️ Ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §5: boot role from
// `SEMIO_APP_ROLE` (native, read directly)/`VITE_SEMIO_APP_ROLE` (wasm — wasm has no env var
// access, so `🟦️.ts` reads `import.meta.env.VITE_SEMIO_APP_ROLE` and calls
// `semioWgpuSetAppRole` before/at mount), default `editor` (`ChromeRole::from_boot_env`'s own
// fallback). Deliberately additive, same idiom as `ICON_ATLAS_RUNTIME` immediately above: a
// `thread_local` a caller opts into reading (`boot_app_role`) rather than a parameter threaded
// through every existing mount/native entry point — this crate currently fails to build clean for
// reasons entirely outside this lease (a concurrent, unrelated plugin-crate refactor breaks a
// transitive dependency; confirmed via `git status` showing 70+ uncommitted stdio-plugin files —
// see `📓️w1-d-report.md`), so a signature change on `run_native`/`semio_wgpu_mount` could not be
// verified to compile and was avoided.
thread_local! {
    static BOOT_APP_ROLE: RefCell<ui_wgpu::wgpu::component::role_chrome::ChromeRole> = RefCell::new(resolve_native_boot_role());
}

#[cfg(not(target_arch = "wasm32"))]
fn resolve_native_boot_role() -> ui_wgpu::wgpu::component::role_chrome::ChromeRole {
    ui_wgpu::wgpu::component::role_chrome::ChromeRole::from_boot_env(std::env::var("SEMIO_APP_ROLE").ok().as_deref())
}

#[cfg(target_arch = "wasm32")]
fn resolve_native_boot_role() -> ui_wgpu::wgpu::component::role_chrome::ChromeRole {
    ui_wgpu::wgpu::component::role_chrome::ChromeRole::Editor
}

/// 🌐️ wasm boot hook — `🟦️.ts` calls this once, before/at mount time, with
/// `import.meta.env.VITE_SEMIO_APP_ROLE ?? "editor"`.
#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(js_name = semioWgpuSetAppRole)]
pub fn semio_wgpu_set_app_role(role: String) {
    BOOT_APP_ROLE.with(|cell| *cell.borrow_mut() = ui_wgpu::wgpu::component::role_chrome::ChromeRole::from_boot_env(Some(role.as_str())));
}

/// 👁️✏️ The boot-resolved role, contract freeze §5 — `SemioApp`'s session/window-open path is meant
/// to read this to call `Shell::set_window_role`/`set_locale`; wiring that specific call site is
/// this lease's documented gap (see `📓️w1-d-report.md` — this crate's own build break blocks
/// verifying any change deep inside `SemioApp`, so this stops at the boundary of what compiles
/// standalone).
pub fn boot_app_role() -> ui_wgpu::wgpu::component::role_chrome::ChromeRole {
    BOOT_APP_ROLE.with(|cell| *cell.borrow())
}
//#endregion 🔖️RoleBoot
