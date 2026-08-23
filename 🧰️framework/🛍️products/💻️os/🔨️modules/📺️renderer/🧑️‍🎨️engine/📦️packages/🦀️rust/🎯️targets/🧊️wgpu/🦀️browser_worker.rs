//! 🧵️ Dedicated browser Worker owner for the complete frame transaction and OffscreenCanvas surface.

use crate::kernel_seam::{HostWaker, KernelSeam};
use crate::program_bridge::{filter_plugins, parse_plugin_entries, ProgramBridgeEntry};
use crate::shell::ShellState;
use crate::{AppInteractionState, AppPresenter, AppRuntime, RuntimeMailbox};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use ui_host::{WindowDelegate, WindowMetrics};
use ui_render::{CursorRequest, DispatchEvent, EventModifiers, ImeEvent, InvalidationReason, PhysicalSize, PointerButton, PointerId, PointerInfo, PointerKind};
use ui_wgpu::wgpu::{ActionDescriptor, DrawList, FontAtlas, GpuContext, IconAtlas, InputState, OffscreenPresentToken, PointerModifiers, Theme};
use wasm_bindgen::prelude::*;

const MESSAGE_BYTE_CAPACITY: usize = 4 * 1024;
const TEXT_STREAM_CAPACITY: usize = 64;
const TEXT_BYTE_CAPACITY: usize = 256 * 1024;
const ICON_SOURCE_CAPACITY: usize = 512;

//#region 📥️Wire
#[derive(Deserialize)]
struct BrowserBatch {
    replaceable: Vec<BrowserWireEvent>,
    lossless: Vec<BrowserWireEvent>,
}

#[derive(Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase")]
enum BrowserWireEvent {
    PointerMove {
        pointer_id: u64,
        pointer_kind: BrowserPointerKind,
        x: f32,
        y: f32,
        pressure: Option<f32>,
        tilt_x: Option<f32>,
        tilt_y: Option<f32>,
    },
    PointerDown {
        pointer_id: u64,
        pointer_kind: BrowserPointerKind,
        x: f32,
        y: f32,
        pressure: Option<f32>,
        tilt_x: Option<f32>,
        tilt_y: Option<f32>,
        button: BrowserPointerButton,
    },
    PointerUp {
        pointer_id: u64,
        pointer_kind: BrowserPointerKind,
        x: f32,
        y: f32,
        pressure: Option<f32>,
        tilt_x: Option<f32>,
        tilt_y: Option<f32>,
        button: BrowserPointerButton,
    },
    Wheel {
        x: f32,
        y: f32,
        delta_x: f32,
        delta_y: f32,
    },
    Resize {
        width: u32,
        height: u32,
        dpr: f32,
    },
    KeyDown {
        key: String,
        shift: bool,
        ctrl: bool,
        alt: bool,
        meta: bool,
    },
    KeyUp {
        key: String,
        shift: bool,
        ctrl: bool,
        alt: bool,
        meta: bool,
    },
    ImeStart,
    ImeCancel,
    TextChunk {
        stream_id: u64,
        target: TextTarget,
        text: String,
        total_bytes: usize,
        #[serde(rename = "final")]
        final_: bool,
        cursor: Option<usize>,
    },
}

#[derive(Clone, Copy, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum TextTarget {
    Text,
    Paste,
    ImeUpdate,
    ImeCommit,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum BrowserPointerKind {
    Mouse,
    Touch,
    Pen,
    Eraser,
}

#[derive(Clone, Copy, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum BrowserPointerButton {
    Primary,
    Secondary,
    Middle,
}

struct PendingText {
    stream_id: u64,
    target: TextTarget,
    declared_bytes: usize,
    bytes: usize,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BrowserTickOutput {
    cursor: &'static str,
    fullscreen: Option<bool>,
    request_frame: bool,
    progress: f32,
    quarantined: bool,
    fault_code: Option<&'static str>,
    fault_detail: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct BrowserBootStepOutput {
    stage: &'static str,
    progress: f32,
    shell_boot: bool,
    complete: bool,
}
//#endregion 📥️Wire

//#region 🧵️Runtime
#[wasm_bindgen]
pub struct BrowserRendererWorker {
    host: Option<crate::os_host::OsHost>,
    retired_host: Option<crate::os_host::OsHostRetirement>,
    text_streams: [Option<PendingText>; TEXT_STREAM_CAPACITY],
    text_bytes: usize,
    latest_generation: u64,
    quarantined: Option<String>,
    close_phase: u8,
}

#[wasm_bindgen]
impl BrowserRendererWorker {
    #[wasm_bindgen(js_name = enqueueBatch)]
    pub fn enqueue_batch(&mut self, events_json: &str, generation: u64) -> Result<(), JsValue> {
        self.ensure_live()?;
        if events_json.len() > MESSAGE_BYTE_CAPACITY {
            return Err(js_error("message-credits", "frame message exceeds the hard byte cap"));
        }
        if generation < self.latest_generation {
            return Ok(());
        }
        self.latest_generation = generation;
        let batch: BrowserBatch = serde_json::from_str(events_json).map_err(|error| js_error("message-decode", &error.to_string()))?;
        if batch.replaceable.len() > 18 || batch.lossless.len() > 16 {
            return Err(js_error("message-items", "frame message exceeds hard item credits"));
        }
        let discrete_commits = self.preflight_host_events(batch.replaceable.iter().chain(&batch.lossless))?;
        let pending_discrete = self.host.as_ref().ok_or_else(|| js_error("worker-closed", "renderer host is unavailable"))?.events.pending_discrete_len();
        if pending_discrete.saturating_add(discrete_commits) > ui_host::DISCRETE_QUEUE_CAPACITY {
            return Err(js_error("downstream-credits", "frame message cannot be admitted atomically into the host mailbox"));
        }
        for event in batch.replaceable.into_iter().chain(batch.lossless) {
            self.apply_wire_event(event)?;
        }
        if let Some(host) = self.host.as_mut() {
            host.frame_generation = generation;
            host.scheduler.invalidate(InvalidationReason::INPUT_STATE);
        }
        Ok(())
    }

    pub fn tick(&mut self, _timestamp_ms: f64, _sequence: u64, generation: u64) -> Result<String, JsValue> {
        self.ensure_live()?;
        if generation != self.latest_generation {
            return Err(js_error("generation-mismatch", "frame tick generation does not match admitted input"));
        }
        if let Some(detail) = self.quarantined.clone() {
            return encode_tick(BrowserTickOutput { cursor: "default", fullscreen: None, request_frame: false, progress: 1.0, quarantined: true, fault_code: Some("present-failed"), fault_detail: Some(detail) });
        }
        let host = self.host.as_mut().ok_or_else(|| js_error("worker-closed", "renderer host is unavailable"))?;
        let outcome = host.redraw_offscreen_worker();
        let text_fault = host.runtime.take_text_fault();
        let frame_fault = host.runtime.take_frame_fault();
        let fault_code = if text_fault.is_some() {
            "text-input-failed"
        } else if frame_fault.is_some() {
            "frame-credits"
        } else {
            "present-failed"
        };
        let present_fault = host.present_fault.take().or(text_fault).or(frame_fault);
        if let Some(detail) = present_fault.clone() {
            self.quarantined = Some(detail);
        }
        encode_tick(BrowserTickOutput {
            cursor: cursor_name(outcome.cursor),
            fullscreen: host.platform_fullscreen.take(),
            request_frame: host.scheduler.next_deadline().is_some() || host.runtime.has_pending_text_work(),
            progress: 1.0,
            quarantined: present_fault.is_some(),
            fault_code: present_fault.as_ref().map(|_| fault_code),
            fault_detail: present_fault,
        })
    }
}

impl BrowserRendererWorker {
    #[wasm_bindgen(js_name = closeStep)]
    pub fn close_step(&mut self) -> Result<bool, JsValue> {
        if self.close_phase == 0 {
            self.close_phase = 1;
            return Ok(false);
        }
        if self.close_phase == 1 {
            let Some(host) = self.host.as_mut() else { return Err(js_error("host-close", "renderer host disappeared before frame cancellation")) };
            if !host.frame_build.close_step() || !host.frame_build.terminal_is_empty() {
                return Ok(false);
            }
            self.close_phase = 2;
            return Ok(false);
        }
        if self.close_phase == 2 {
            if let Some(slot) = self.text_streams.iter().position(Option::is_some) {
                let stream_id = self.text_streams[slot].as_ref().expect("text stream slot").stream_id;
                self.abort_text_stream(slot, stream_id)?;
                return Ok(false);
            }
            self.close_phase = 3;
            return Ok(false);
        }
        if self.close_phase == 3 {
            let Some(host) = self.host.as_mut() else { return Err(js_error("host-close", "renderer host disappeared before event retirement")) };
            if !host.events.close_step() || !host.events.terminal_is_empty() {
                return Ok(false);
            }
            self.close_phase = 4;
            return Ok(false);
        }
        if self.close_phase == 4 {
            let Some(host) = self.host.as_ref() else { return Err(js_error("host-close", "renderer host disappeared before input retirement")) };
            let Ok(mut runtime) = host.runtime.try_lock() else { return Ok(false) };
            let closed = match runtime.interaction.as_mut() {
                Some(interaction) => interaction.input.close_step().map_err(|fault| js_error("text-close", &format!("{fault:?}")))? && interaction.input.terminal_is_empty(),
                None => true,
            };
            if !closed {
                return Ok(false);
            }
            self.close_phase = 5;
            return Ok(false);
        }
        if self.close_phase == 5 {
            self.retired_host = self.host.take().map(crate::os_host::OsHost::into_retirement);
            self.close_phase = 6;
            return Ok(false);
        }
        if let Some(retired) = self.retired_host.as_mut() {
            if !retired.close_step() {
                return Ok(false);
            }
            if !retired.terminal_is_empty() {
                return Err(js_error("host-close", "renderer host retirement completed without a terminal-empty witness"));
            }
            self.retired_host = None;
            return Ok(false);
        }
        Ok(true)
    }

    fn ensure_live(&self) -> Result<(), JsValue> {
        if self.close_phase != 0 || self.host.is_none() {
            return Err(js_error("worker-closing", "renderer Worker no longer admits work"));
        }
        if let Some(detail) = self.quarantined.as_ref() {
            return Err(js_error("worker-quarantined", detail));
        }
        Ok(())
    }

    fn apply_wire_event(&mut self, event: BrowserWireEvent) -> Result<(), JsValue> {
        match event {
            BrowserWireEvent::PointerMove { pointer_id, pointer_kind, x, y, pressure, tilt_x, tilt_y } => self.dispatch(DispatchEvent::PointerMove { pointer: pointer(pointer_id, pointer_kind, pressure, tilt_x, tilt_y), x, y })?,
            BrowserWireEvent::PointerDown { pointer_id, pointer_kind, x, y, pressure, tilt_x, tilt_y, button } => {
                self.dispatch(DispatchEvent::PointerDown { pointer: pointer(pointer_id, pointer_kind, pressure, tilt_x, tilt_y), x, y, button: button.into() })?
            }
            BrowserWireEvent::PointerUp { pointer_id, pointer_kind, x, y, pressure, tilt_x, tilt_y, button } => {
                self.dispatch(DispatchEvent::PointerUp { pointer: pointer(pointer_id, pointer_kind, pressure, tilt_x, tilt_y), x, y, button: button.into() })?
            }
            BrowserWireEvent::Wheel { x, y, delta_x, delta_y } => self.dispatch(DispatchEvent::Scroll { x, y, delta_x, delta_y })?,
            BrowserWireEvent::Resize { width, height, dpr } => {
                self.host.as_mut().ok_or_else(|| js_error("worker-closed", "renderer host is unavailable"))?.handle_metrics(WindowMetrics { physical: PhysicalSize::new(width, height), scale_factor: dpr })
            }
            BrowserWireEvent::KeyDown { key, shift, ctrl, alt, meta } => self.dispatch(DispatchEvent::KeyDown { key, modifiers: EventModifiers { shift, ctrl, alt, meta } })?,
            BrowserWireEvent::KeyUp { key, shift, ctrl, alt, meta } => self.dispatch(DispatchEvent::KeyUp { key, modifiers: EventModifiers { shift, ctrl, alt, meta } })?,
            BrowserWireEvent::ImeStart => self.dispatch(DispatchEvent::Ime(ImeEvent::Start))?,
            BrowserWireEvent::ImeCancel => self.dispatch(DispatchEvent::Ime(ImeEvent::Cancel))?,
            BrowserWireEvent::TextChunk { stream_id, target, text, total_bytes, final_, cursor } => {
                if text.len() > 4 * 1024 {
                    return Err(js_error("text-chunk-credits", "text chunk exceeds the Worker hard cap"));
                }
                let slot = if let Some(slot) = self.text_streams.iter().position(|entry| entry.as_ref().is_some_and(|stream| stream.stream_id == stream_id)) {
                    slot
                } else {
                    let slot = self.text_streams.iter().position(Option::is_none).ok_or_else(|| js_error("text-stream-credits", "too many incomplete text streams"))?;
                    self.text_streams[slot] = Some(PendingText { stream_id, target, declared_bytes: total_bytes, bytes: 0 });
                    if matches!(target, TextTarget::Text | TextTarget::Paste) {
                        self.dispatch(DispatchEvent::TextEditStart { stream: stream_id, target: if target == TextTarget::Text { ui_render::TextEditTarget::Text } else { ui_render::TextEditTarget::Paste }, declared_bytes: total_bytes })?;
                    }
                    slot
                };
                let protocol_error = self.text_streams[slot].as_ref().and_then(|stream| {
                    if stream.target != target {
                        Some(("text-stream-target", "text stream target changed before completion"))
                    } else if stream.declared_bytes != total_bytes {
                        Some(("text-stream-bytes", "text stream byte declaration changed before completion"))
                    } else {
                        None
                    }
                });
                if let Some((code, detail)) = protocol_error {
                    self.abort_text_stream(slot, stream_id)?;
                    return Err(js_error(code, detail));
                }
                if matches!(target, TextTarget::ImeUpdate | TextTarget::ImeCommit) && !final_ {
                    return Err(js_error("ime-chunk-credits", "IME payload must fit one bounded text chunk"));
                }
                let stream = self.text_streams[slot].as_mut().expect("fixed stream slot is occupied");
                stream.bytes = stream.bytes.saturating_add(text.len());
                self.text_bytes = self.text_bytes.saturating_add(text.len());
                if self.text_bytes > TEXT_BYTE_CAPACITY {
                    self.abort_text_stream(slot, stream_id)?;
                    return Err(js_error("text-stream-bytes", "segmented text operations exceeded Worker byte credits"));
                }
                let stream_overflow = stream.bytes > stream.declared_bytes;
                if stream_overflow {
                    self.abort_text_stream(slot, stream_id)?;
                    return Err(js_error("text-stream-bytes", "segmented text operation exceeded its declared byte credits"));
                }
                match target {
                    TextTarget::Text | TextTarget::Paste => self.dispatch(DispatchEvent::TextEditChunk { stream: stream_id, text })?,
                    TextTarget::ImeUpdate => {
                        if final_ {
                            self.dispatch(DispatchEvent::Ime(ImeEvent::Update { text, cursor: cursor.unwrap_or(0) }))?;
                        }
                    }
                    TextTarget::ImeCommit => {
                        if final_ {
                            self.dispatch(DispatchEvent::Ime(ImeEvent::Commit { text }))?;
                        }
                    }
                }
                if final_ {
                    let stream = self.text_streams[slot].take().expect("fixed stream slot is occupied");
                    self.text_bytes = self.text_bytes.saturating_sub(stream.bytes);
                    if matches!(stream.target, TextTarget::Text | TextTarget::Paste) {
                        self.dispatch(DispatchEvent::TextEditCommit { stream: stream_id })?;
                    }
                }
            }
        }
        Ok(())
    }

    fn dispatch(&mut self, event: DispatchEvent) -> Result<(), JsValue> {
        self.host.as_mut().ok_or_else(|| js_error("worker-closed", "renderer host is unavailable"))?.handle_event(event);
        Ok(())
    }

    fn abort_text_stream(&mut self, slot: usize, stream_id: u64) -> Result<(), JsValue> {
        if let Some(stream) = self.text_streams[slot].take() {
            self.text_bytes = self.text_bytes.saturating_sub(stream.bytes);
            if matches!(stream.target, TextTarget::Text | TextTarget::Paste) {
                self.dispatch(DispatchEvent::TextEditAbort { stream: stream_id })?;
            }
        }
        Ok(())
    }

    fn preflight_host_events<'a>(&self, events: impl Iterator<Item = &'a BrowserWireEvent>) -> Result<usize, JsValue> {
        let mut streams: [Option<PendingText>; TEXT_STREAM_CAPACITY] =
            std::array::from_fn(|index| self.text_streams[index].as_ref().map(|stream| PendingText { stream_id: stream.stream_id, target: stream.target, declared_bytes: stream.declared_bytes, bytes: stream.bytes }));
        let mut reserved_bytes = streams.iter().flatten().map(|stream| stream.declared_bytes).sum::<usize>();
        let mut count = 0usize;
        for event in events {
            match event {
                BrowserWireEvent::PointerMove { .. } | BrowserWireEvent::Wheel { .. } | BrowserWireEvent::Resize { .. } => {}
                BrowserWireEvent::TextChunk { stream_id, target, text, total_bytes, final_, .. } => {
                    if text.len() > 4 * 1024 {
                        return Err(js_error("text-chunk-credits", "text chunk exceeds the Worker hard cap"));
                    }
                    if matches!(target, TextTarget::ImeUpdate | TextTarget::ImeCommit) && !final_ {
                        return Err(js_error("ime-chunk-credits", "IME payload must fit one bounded text chunk"));
                    }
                    let slot = match streams.iter().position(|stream| stream.as_ref().is_some_and(|stream| stream.stream_id == *stream_id)) {
                        Some(slot) => slot,
                        None => {
                            let slot = streams.iter().position(Option::is_none).ok_or_else(|| js_error("text-stream-credits", "too many incomplete text streams"))?;
                            reserved_bytes = reserved_bytes.saturating_add(*total_bytes);
                            if reserved_bytes > TEXT_BYTE_CAPACITY {
                                return Err(js_error("text-stream-bytes", "segmented text operations exceeded Worker byte credits"));
                            }
                            streams[slot] = Some(PendingText { stream_id: *stream_id, target: *target, declared_bytes: *total_bytes, bytes: 0 });
                            if matches!(target, TextTarget::Text | TextTarget::Paste) {
                                count += 1;
                            }
                            slot
                        }
                    };
                    let stream = streams[slot].as_mut().expect("preflight stream");
                    if stream.target != *target || stream.declared_bytes != *total_bytes {
                        return Err(js_error("text-stream-protocol", "text stream descriptor changed before completion"));
                    }
                    stream.bytes = stream.bytes.saturating_add(text.len());
                    if stream.bytes > stream.declared_bytes {
                        return Err(js_error("text-stream-bytes", "segmented text operation exceeded its declared byte credits"));
                    }
                    count += 1;
                    if *final_ {
                        reserved_bytes = reserved_bytes.saturating_sub(stream.declared_bytes);
                        streams[slot] = None;
                        if matches!(target, TextTarget::Text | TextTarget::Paste) {
                            count += 1;
                        }
                    }
                }
                _ => count += 1,
            }
        }
        Ok(count)
    }
}
//#endregion 🧵️Runtime

//#region 🚀️Boot
#[wasm_bindgen]
pub struct BrowserRendererBootstrap {
    gpu: Option<GpuContext>,
    plugins: JsValue,
    plugin_filter: String,
    width: u32,
    height: u32,
    dpr: f32,
    wake: js_sys::Function,
    atlas: Option<FontAtlas>,
    icons: Option<IconAtlas>,
    entries: Option<Vec<ProgramBridgeEntry>>,
    shell: Option<ShellState>,
    phase: u8,
}

#[wasm_bindgen]
impl BrowserRendererBootstrap {
    pub fn step(&mut self) -> Result<String, JsValue> {
        let output = match self.phase {
            0 => {
                self.atlas = Some(FontAtlas::from_bytes(&[]).map_err(|error| js_error("font-atlas", &error.to_string()))?);
                BrowserBootStepOutput { stage: "font-atlas", progress: 0.1, shell_boot: false, complete: false }
            }
            1 => {
                if crate::icon_atlas::icon_atlas_source_count() > ICON_SOURCE_CAPACITY {
                    return Err(js_error("icon-credits", "icon atlas source count exceeds the Worker boot cap"));
                }
                self.icons = Some(crate::icon_atlas::build_icon_atlas());
                BrowserBootStepOutput { stage: "icon-atlas", progress: 0.25, shell_boot: false, complete: false }
            }
            2 => {
                self.gpu.as_mut().expect("bootstrap GPU exists").upload_font_atlas(self.atlas.as_ref().expect("bootstrap atlas exists"));
                BrowserBootStepOutput { stage: "font-upload", progress: 0.35, shell_boot: false, complete: false }
            }
            3 => {
                self.gpu.as_mut().expect("bootstrap GPU exists").upload_icon_atlas(self.icons.as_ref().expect("bootstrap icons exist"));
                BrowserBootStepOutput { stage: "icon-upload", progress: 0.45, shell_boot: false, complete: false }
            }
            4 => {
                self.entries = Some(filter_plugins(parse_plugin_entries(self.plugins.clone()).map_err(|error| js_error("plugin-parse", &error.to_string()))?, &self.plugin_filter));
                BrowserBootStepOutput { stage: "plugin-parse", progress: 0.55, shell_boot: false, complete: false }
            }
            5 => {
                let mut shell = ShellState::new(self.entries.take().expect("bootstrap plugin entries exist"), self.plugin_filter.clone());
                shell.screen_w = self.width.max(1) as f32;
                shell.screen_h = self.height.max(1) as f32;
                self.shell = Some(shell);
                BrowserBootStepOutput { stage: "shell-construct", progress: 0.65, shell_boot: false, complete: false }
            }
            6 => BrowserBootStepOutput { stage: "shell-boot", progress: 0.7, shell_boot: true, complete: false },
            _ => BrowserBootStepOutput { stage: "runtime-ready", progress: 0.95, shell_boot: false, complete: true },
        };
        if self.phase != 6 {
            self.phase = self.phase.saturating_add(1);
        }
        serde_json::to_string(&output).map_err(|error| js_error("boot-step-encode", &error.to_string()))
    }

    #[wasm_bindgen(js_name = bootShell)]
    pub async fn boot_shell(mut self) -> Result<BrowserRendererBootstrap, JsValue> {
        if self.phase != 6 {
            return Err(js_error("boot-phase", "shell boot requested outside its owned phase"));
        }
        self.shell.as_mut().expect("bootstrap shell exists").boot().await.map_err(|error| js_error("shell-boot", &error.to_string()))?;
        self.phase = 7;
        Ok(self)
    }

    pub fn finish(mut self) -> Result<BrowserRendererWorker, JsValue> {
        if self.phase < 7 {
            return Err(js_error("boot-phase", "renderer bootstrap is incomplete"));
        }
        let atlas = self.atlas.take().expect("bootstrap atlas exists");
        let icons = self.icons.take().expect("bootstrap icons exist");
        let shell = self.shell.take().expect("bootstrap shell exists");
        let runtime = RuntimeMailbox::new(AppRuntime {
            atlas,
            icons,
            interaction: Some(AppInteractionState {
                shell,
                input: InputState::<ActionDescriptor>::default(),
                theme: Theme::default(),
                theme_dark: crate::appearance_is_dark("system"),
                last_pointer_x: 0.0,
                last_pointer_y: 0.0,
                pointer_down: false,
                pointer_button: 0,
                modifiers: PointerModifiers::default(),
                wheel_delta: 0.0,
                space_pressed: false,
                wheel_zoom_deadline_ms: 0.0,
                world3d_camera_dispatch_deadlines_ms: HashMap::new(),
                caret_blink_at_ms: 0.0,
                caret_blink_visible: true,
                asset_poll_pending: false,
                text_streams: std::array::from_fn(|_| None),
                text_fault: None,
                frame_fault: None,
                text_cancel_pending: false,
            }),
            draw: DrawList::default(),
            overlay: DrawList::default(),
        });
        let token = OffscreenPresentToken::mint_for_dedicated_worker().map_err(|error| js_error("worker-capability", error))?;
        let presenter = AppPresenter {
            gpu: self.gpu.take().expect("bootstrap GPU exists"),
            engine: crate::engine_canvas::EngineCanvasPresenter::default(),
            gate: ui_wgpu::wgpu::PreparedRenderGate::default(),
            window: None,
            offscreen_token: Some(token),
            last_cursor: None,
        };
        let mut host = crate::os_host::OsHost::new(runtime, presenter);
        let runtime_wake = self.wake.clone();
        host.runtime.set_waker(Rc::new(move || {
            let _ = runtime_wake.call0(&JsValue::NULL);
        }));
        let kernel_wake = self.wake.clone();
        host.kernel.set_waker(HostWaker::new(move || {
            let _ = kernel_wake.call0(&JsValue::NULL);
        }));
        host.scheduler.invalidate(InvalidationReason::STRUCTURE);
        Ok(BrowserRendererWorker { host: Some(host), retired_host: None, text_streams: std::array::from_fn(|_| None), text_bytes: 0, latest_generation: 0, quarantined: None, close_phase: 0 })
    }
}

#[wasm_bindgen(js_name = semioWgpuWorkerBootstrap)]
pub async fn semio_wgpu_worker_bootstrap(canvas: web_sys::OffscreenCanvas, plugins: JsValue, plugin_filter: String, width: u32, height: u32, dpr: f32, wake: js_sys::Function) -> Result<BrowserRendererBootstrap, JsValue> {
    if web_sys::window().is_some() {
        return Err(js_error("ui-isolate-forbidden", "the frame Worker renderer cannot boot in the browser UI isolate"));
    }
    let css_width = width.max(1) as f32 / dpr.max(f32::EPSILON);
    let css_height = height.max(1) as f32 / dpr.max(f32::EPSILON);
    canvas.set_width(width.max(1));
    canvas.set_height(height.max(1));
    let gpu = GpuContext::from_offscreen_canvas(canvas, css_width, css_height, dpr).await.map_err(|error| js_error("gpu-boot", &error))?;
    Ok(BrowserRendererBootstrap { gpu: Some(gpu), plugins, plugin_filter, width, height, dpr, wake, atlas: None, icons: None, entries: None, shell: None, phase: 0 })
}
//#endregion 🚀️Boot

fn pointer(id: u64, kind: BrowserPointerKind, pressure: Option<f32>, tilt_x: Option<f32>, tilt_y: Option<f32>) -> PointerInfo {
    PointerInfo { id: PointerId(id), kind: kind.into(), pressure, tilt: tilt_x.zip(tilt_y) }
}

impl From<BrowserPointerKind> for PointerKind {
    fn from(value: BrowserPointerKind) -> Self {
        match value {
            BrowserPointerKind::Mouse => Self::Mouse,
            BrowserPointerKind::Touch => Self::Touch,
            BrowserPointerKind::Pen => Self::Pen,
            BrowserPointerKind::Eraser => Self::Eraser,
        }
    }
}

impl From<BrowserPointerButton> for PointerButton {
    fn from(value: BrowserPointerButton) -> Self {
        match value {
            BrowserPointerButton::Primary => Self::Primary,
            BrowserPointerButton::Secondary => Self::Secondary,
            BrowserPointerButton::Middle => Self::Middle,
        }
    }
}

fn cursor_name(cursor: CursorRequest) -> &'static str {
    match cursor {
        CursorRequest::Default => "default",
        CursorRequest::Pointer => "pointer",
        CursorRequest::Text => "text",
        CursorRequest::Grab => "grab",
        CursorRequest::Grabbing => "grabbing",
    }
}

fn encode_tick(output: BrowserTickOutput) -> Result<String, JsValue> {
    serde_json::to_string(&output).map_err(|error| js_error("tick-encode", &error.to_string()))
}

fn js_error(code: &str, detail: &str) -> JsValue {
    JsValue::from_str(&format!("{code}: {detail}"))
}
