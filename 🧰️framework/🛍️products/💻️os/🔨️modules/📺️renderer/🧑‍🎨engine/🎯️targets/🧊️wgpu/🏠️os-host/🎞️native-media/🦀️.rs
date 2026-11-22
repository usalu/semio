//! 🎞️ The native media of one `OsHost`: the presenter that plays the slots a presented frame carries, the byte transfers and decodes that run off the UI thread, the pointer, key, cut and assistive-technology routes of its transport controls, and the repaint it asks of the windows that show them. See [`super`].

use semio_renderer_wgpu_media_presenter::{DecodedMedia, MediaAccessibility, MediaInput, MediaPointer, NativeMediaFault, NativeMediaHost, UiImageUploads};
use semio_renderer_wgpu_media_slots::PresentedMediaSlot;
use semio_renderer_wgpu_native_platform::native_accessibility::{publish_native_accessibility_windows, NativeAccessibilityWindow};
use std::collections::{BTreeMap, BTreeSet};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};

const MAXIMUM_FRAME_DELTA_MS: f64 = 250.0;

/// 🏷️ The window id a slot's transport controls announce under.
pub const ACCESSIBILITY_WINDOW_PREFIX: &str = "media.";

enum Transfer {
    Progress { token: String, done: u64, total: u64, detail: String },
    Decoded { token: String, decoded: Result<DecodedMedia, NativeMediaFault> },
    Ended { token: String, failure: Option<String> },
}

/// 📦️ The coordinates the owner's media-export port needs for one slot: instance, output port, document and revision.
struct SlotResource {
    instance: u32,
    port: String,
    parent_document_id: String,
    revision: u64,
}

fn slot_resource(slot: &PresentedMediaSlot) -> Option<SlotResource> {
    let port = slot.props.get("resource")?.get("outputPort")?.as_str()?.to_owned();
    let revision = slot.props.get("revision")?.as_str()?.parse().ok()?;
    Some(SlotResource { instance: slot.app_instance_id, port, parent_document_id: slot.parent_document_id.clone(), revision })
}

/// 🏁️ What a routed input did: whether a media control took it, and the cut action JSON (`{windowId, controllerId, action, args}`, the page host's media action) it asks the shell to run.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct MediaRoute {
    pub taken: bool,
    pub action: Option<String>,
    /// 🔁️ The event is also the page's (a press focuses the slot node in the engine like any focusable node, so the page's Tab order continues from it).
    pub pass_through: bool,
}

/// 🎛️ Native playback of the media slots of a window.
pub struct NativeMedia {
    host: NativeMediaHost,
    inbox: Arc<Mutex<Vec<Transfer>>>,
    started: BTreeSet<String>,
    owners: BTreeMap<String, (String, String)>,
    windows: BTreeMap<String, String>,
    nodes: Vec<(String, String, u64)>,
    engine_slot: Option<String>,
    backward: bool,
    active: BTreeSet<String>,
    last_seconds: Option<f64>,
}

impl NativeMedia {
    pub fn new() -> Self {
        Self { host: NativeMediaHost::new(true), inbox: Arc::default(), started: BTreeSet::new(), owners: BTreeMap::new(), windows: BTreeMap::new(), nodes: Vec::new(), engine_slot: None, backward: false, active: BTreeSet::new(), last_seconds: None }
    }

    fn drain(&mut self) {
        let transfers = self.inbox.lock().map(|mut inbox| std::mem::take(&mut *inbox)).unwrap_or_default();
        for transfer in transfers {
            match transfer {
                Transfer::Progress { token, done, total, detail } => self.host.presenter().transfer_progress(&token, done, total, &detail),
                Transfer::Decoded { token, decoded } => self.host.finish_decode(&token, decoded),
                Transfer::Ended { token, failure } => self.host.presenter().transfer_ended(&token, failure),
            }
        }
    }

    fn start_transfers(&mut self, slots: &[PresentedMediaSlot]) {
        for token in self.host.awaiting_bytes() {
            if self.started.contains(&token) {
                continue;
            }
            let (Some(slot), Some(cancel)) = (slots.iter().find(|slot| slot.token == token), self.host.presenter().cancel_flag(&token)) else { continue };
            self.started.insert(token.clone());
            let Some(resource) = slot_resource(slot) else {
                self.host.presenter().transfer_ended(&token, Some("framework.media.transport@1 · resource".to_owned()));
                continue;
            };
            let inbox = Arc::clone(&self.inbox);
            semio_renderer_wgpu_host_base::spawn_app_task(async move {
                let push = |transfer: Transfer| {
                    if let Ok(mut inbox) = inbox.lock() {
                        inbox.push(transfer);
                    }
                };
                let progress_token = token.clone();
                let collected = semio_renderer_wgpu_program_bridge::collect_media_export(
                    resource.instance,
                    &resource.port,
                    &resource.parent_document_id,
                    resource.revision,
                    &|| cancel.load(Ordering::Relaxed),
                    &mut |done, total, detail: &str| push(Transfer::Progress { token: progress_token.clone(), done, total, detail: detail.to_owned() }),
                )
                .await;
                match collected {
                    Ok((bytes, _)) => {
                        let decoded = semio_renderer_wgpu_media_presenter::load_native_media(&bytes, &mut |done, total| push(Transfer::Progress { token: progress_token.clone(), done, total, detail: String::new() }), &|| cancel.load(Ordering::Relaxed));
                        push(Transfer::Decoded { token, decoded });
                    }
                    Err(error) => push(Transfer::Ended { failure: (!cancel.load(Ordering::Relaxed)).then(|| error.to_string()), token }),
                }
            });
        }
        self.started.retain(|token| slots.iter().any(|slot| &slot.token == token));
    }

    fn remember(&mut self, slots: &[PresentedMediaSlot]) {
        self.owners = slots.iter().map(|slot| (slot.token.clone(), (slot.window_id.clone(), slot.controller_id.clone()))).collect();
        self.windows = slots.iter().map(|slot| (slot.token.clone(), slot.window_id.clone())).collect();
        self.nodes = slots.iter().map(|slot| (slot.token.clone(), slot.window_id.clone(), slot.node_id.parse().unwrap_or(0))).collect();
    }

    /// 🔦️ The slot whose node holds the page's Tab focus right now, read from the retained engine.
    fn engine_focused_slot(&self) -> Option<String> {
        semio_renderer_wgpu_contracts::UI_ENGINE.with(|cell| {
            let engine = cell.borrow();
            self.nodes.iter().find(|(_, window, node)| engine.focused_document_node(window) == Some(*node)).map(|(token, _, _)| token.clone())
        })
    }

    /// 🔁️ Follows the page's focus into and out of the slots: when the engine's focus lands on a slot node the media focus enters it (first stop, or last when the traversal came backward), unless a press already focused a row there; when it leaves, the media focus is released.
    fn sync_focus(&mut self) {
        if self.nodes.is_empty() {
            self.engine_slot = None;
            self.host.release();
            return;
        }
        let focused = self.engine_focused_slot();
        if focused == self.engine_slot {
            return;
        }
        self.engine_slot = focused.clone();
        match focused {
            Some(token) if self.host.focused_token() != Some(token.as_str()) => {
                self.host.enter(&token, self.backward);
            }
            Some(_) => {}
            None => self.host.release(),
        }
    }

    fn announce(&self) {
        let windows = self
            .host
            .accessibility_nodes()
            .into_iter()
            .map(|(token, nodes)| NativeAccessibilityWindow { window_id: format!("{ACCESSIBILITY_WINDOW_PREFIX}{token}"), window_generation: 0, nodes })
            .collect();
        publish_native_accessibility_windows(windows);
    }

    /// 🖼️ Presents one frame's slots at `now` seconds and answers the windows whose retained paint must run again: those with a slot that plays or loads, so the picture, the position and the progress they draw keep moving.
    pub fn present(&mut self, slots: &[PresentedMediaSlot], now: f64) -> Vec<String> {
        let delta = self.last_seconds.map_or(0.0, |last| ((now - last) * 1000.0).clamp(0.0, MAXIMUM_FRAME_DELTA_MS));
        self.last_seconds = Some(now);
        self.remember(slots);
        self.drain();
        self.sync_focus();
        let busy = self.host.present_overlays(slots, delta as u64, &mut UiImageUploads);
        self.start_transfers(slots);
        self.announce();
        self.active = slots.iter().filter(|slot| self.host.is_active(&slot.token)).map(|slot| slot.token.clone()).collect();
        let mut windows: Vec<String> = if busy { self.active.iter().filter_map(|token| self.windows.get(token).cloned()).collect() } else { Vec::new() };
        windows.sort();
        windows.dedup();
        windows
    }

    /// 🪟️ Every window that shows a media slot, for a repaint after input changed what the controls draw.
    pub fn windows(&self) -> Vec<String> {
        let mut windows: Vec<String> = self.windows.values().cloned().collect();
        windows.sort();
        windows.dedup();
        windows
    }

    fn route(&self, input: MediaInput) -> MediaRoute {
        match input {
            MediaInput::Ignored => MediaRoute::default(),
            MediaInput::Consumed => MediaRoute { taken: true, ..Default::default() },
            MediaInput::Cut { token, request } => {
                let action = self.owners.get(&token).map(|(window_id, controller_id)| {
                    serde_json::json!({ "windowId": window_id, "controllerId": controller_id, "action": request.action_id, "args": { "revision": request.revision, "value": request.value } }).to_string()
                });
                MediaRoute { taken: true, action, pass_through: false }
            }
        }
    }

    /// 🖱️ Routes a pointer press, move or release at logical `(x, y)`.
    pub fn pointer(&mut self, phase: MediaPointer, x: f32, y: f32) -> MediaRoute {
        let input = self.host.pointer(phase, x, y);
        let mut route = self.route(input);
        route.pass_through = route.taken && phase == MediaPointer::Down;
        route
    }

    /// ⌨️ Routes a key to the slot that holds the focus.
    pub fn key(&mut self, key: &str, shift: bool, ctrl: bool, alt: bool, meta: bool) -> MediaRoute {
        let tab = key == "Tab" && !ctrl && !alt && !meta;
        if tab {
            self.backward = shift;
            if self.host.focused_token().is_none() {
                if let Some(token) = self.engine_focused_slot() {
                    if self.host.enter(&token, shift) {
                        self.engine_slot = Some(token);
                        return MediaRoute { taken: true, ..Default::default() };
                    }
                }
            }
        }
        let input = self.host.key(key, shift, ctrl, alt, meta);
        self.route(input)
    }

    /// ♿️ Routes one assistive-technology action on a media window (`window_id` is `media.<slot token>`); a window that is not a media window is not taken.
    pub fn accessibility(&mut self, window_id: &str, key: &str, event: MediaAccessibility) -> MediaRoute {
        let Some(token) = window_id.strip_prefix(ACCESSIBILITY_WINDOW_PREFIX) else { return MediaRoute::default() };
        let input = self.host.accessibility(token, key, event);
        self.route(input)
    }
}
