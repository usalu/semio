//! 🎛️ The wgpu shell's native media presenter: the adapter between the slots a presented frame carries ([`PresentedMediaSlot`], consumed by the
//! browser host as `HTMLMediaElement` overlays) and [`MediaSlotPresenter`], which plays them natively. [`NativeMediaHost::present`] is the one call
//! a frame makes: it reconciles the slots, ticks every player by the frame's elapsed time, uploads each new video picture as a keyed raster
//! (the renderer then paints one image quad under [`MediaQuad::raster_key`] in the slot rectangle, the same keyed-raster door icon previews use)
//! and answers the transport [`Control`]s to draw and expose. The bytes of a slot's medium come from its owner's media-export port;
//! [`NativeMediaHost::awaiting_bytes`] names the slots that still wait for them and [`NativeMediaHost::provide_bytes`] hands them in.

use semio_renderer_media_native::{open_system_audio_output, AudioOutputOpener, Control, ControlRole, MediaQuad, MediaSlotPresenter, RasterUploadPort, SlotDescriptor, SlotRect};
use ui_contract::AccessibilityProjectionNode;
use semio_ui_wgpu_paint_overlay::{media_window_handle, publish_media_overlays, MediaOverlay, MediaOverlayQuad, MediaOverlayRow, MediaOverlayShape};
use semio_renderer_wgpu_media_slots::{PresentedMediaRect, PresentedMediaSlot};

pub use semio_renderer_media_native::{load_native_media, parse_slot_props, ControlKind, CutRequest, NativeMedia as DecodedMedia, NativeMediaFault, SlotLoad};

fn rect(area: &PresentedMediaRect) -> SlotRect {
    SlotRect::new(area.x, area.y, area.width, area.height)
}

/// 📍️ A presented slot as the presenter sees it: its token, its rectangle clipped to the visible clip, whether another window covers it, and its parsed props.
pub fn slot_descriptor(slot: &PresentedMediaSlot) -> SlotDescriptor {
    let (body, clip) = (rect(&slot.rect), rect(&slot.clip));
    let (x, y) = (body.x.max(clip.x), body.y.max(clip.y));
    let visible = SlotRect::new(x, y, (body.x + body.w).min(clip.x + clip.w).max(x) - x, (body.y + body.h).min(clip.y + clip.h).max(y) - y);
    SlotDescriptor { token: slot.token.clone(), rect: if visible.w > 0.0 && visible.h > 0.0 { body } else { visible }, occluded: slot.occluded || visible.w <= 0.0 || visible.h <= 0.0, props: parse_slot_props(&slot.props) }
}

/// 🖼️ The shell's UI image upload queue as the presenter's [`RasterUploadPort`]: the frame transaction's per-frame drain hands each picture to the raster texture table under the key the media row's quad names.
pub struct UiImageUploads;

impl RasterUploadPort for UiImageUploads {
    fn upload(&mut self, _surface_id: &str, key: &str, width: u32, height: u32, rgba: &[u8]) -> Result<(), semio_renderer_media_native::RasterUploadRefusal> {
        if semio_ui_wgpu_paint_overlay::submit_ui_image_upload(key, width, height, rgba.to_vec()) {
            Ok(())
        } else {
            Err(semio_renderer_media_native::RasterUploadRefusal::Busy)
        }
    }
}

/// 🎞️ What one frame asks the renderer to paint and expose for the media slots.
#[derive(Debug, Default, PartialEq)]
pub struct MediaFrame {
    pub quads: Vec<(String, MediaQuad)>,
    pub controls: Vec<(String, Vec<Control>)>,
}

/// 🎛️ What a pointer or key did to the media slots, beyond moving their own transport.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MediaInput {
    Ignored,
    Consumed,
    Cut { token: String, request: CutRequest },
}

/// 🖱️ The phase of a pointer event routed to the media slots.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaPointer {
    Down,
    Move,
    Up,
}

fn row_shape(control: &Control) -> MediaOverlayShape {
    match control.role {
        ControlRole::Button => MediaOverlayShape::Button,
        ControlRole::Slider | ControlRole::ProgressBar => MediaOverlayShape::Slider,
        ControlRole::Alert => MediaOverlayShape::Alert,
        ControlRole::Group | ControlRole::Status | ControlRole::Output | ControlRole::Text => MediaOverlayShape::Text,
    }
}

/// ♿️ What an assistive technology asked of one control row.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MediaAccessibility {
    Focus,
    Blur,
    Activate,
    Value(String),
}

/// 🔑️ The accessibility key of a control row, the id an assistive technology addresses it by.
pub fn control_key(kind: ControlKind) -> &'static str {
    match kind {
        ControlKind::Root => "root",
        ControlKind::Status => "status",
        ControlKind::Progress => "progress",
        ControlKind::CancelLoad => "cancel-load",
        ControlKind::PlayPause => "play-pause",
        ControlKind::Seek => "seek",
        ControlKind::Position => "position",
        ControlKind::Duration => "duration",
        ControlKind::SelectionStart => "selection-start",
        ControlKind::SelectionEnd => "selection-end",
        ControlKind::Cut => "cut",
    }
}

fn control_of_key(key: &str) -> Option<ControlKind> {
    [ControlKind::Root, ControlKind::Status, ControlKind::Progress, ControlKind::CancelLoad, ControlKind::PlayPause, ControlKind::Seek, ControlKind::Position, ControlKind::Duration, ControlKind::SelectionStart, ControlKind::SelectionEnd, ControlKind::Cut].into_iter().find(|kind| control_key(*kind) == key)
}

fn accessibility_role(control: &Control) -> (&'static str, &'static str) {
    match control.role {
        ControlRole::Group => ("group", "off"),
        ControlRole::Alert => ("status", "assertive"),
        ControlRole::Status | ControlRole::Output => ("status", "polite"),
        ControlRole::ProgressBar => ("progressbar", "off"),
        ControlRole::Button => ("button", "off"),
        ControlRole::Slider => ("slider", "off"),
        ControlRole::Text => ("paragraph", "off"),
    }
}

/// ♿️ The accessibility projection of one slot's rows, in reading order: the root group at depth 0 and every other row below it, with the role, name, value, shortcut, state and rectangle the browser overlay's DOM announces.
pub fn media_accessibility_nodes(controls: &[Control], focus: Option<ControlKind>) -> Vec<AccessibilityProjectionNode> {
    controls
        .iter()
        .enumerate()
        .map(|(index, control)| {
            let (role, live) = accessibility_role(control);
            let interactive = matches!(control.role, ControlRole::Button | ControlRole::Slider);
            AccessibilityProjectionNode {
                node_id: index as u64 + 1,
                key: control_key(control.kind).to_owned(),
                role: role.to_owned(),
                depth: usize::from(control.kind != ControlKind::Root),
                label: Some(if control.label.is_empty() { control.text.clone() } else { control.label.clone() }),
                live: live.to_owned(),
                shortcut: control.shortcuts.map(str::to_owned),
                disabled: !control.enabled,
                focusable: interactive,
                tabbable: interactive && control.enabled,
                actionable: control.role == ControlRole::Button,
                focused: focus == Some(control.kind),
                editable: control.role == ControlRole::Slider,
                rect: Some([control.rect.x, control.rect.y, control.rect.w, control.rect.h]),
                value_min: matches!(control.role, ControlRole::Slider | ControlRole::ProgressBar).then_some(control.min as f64),
                value_max: matches!(control.role, ControlRole::Slider | ControlRole::ProgressBar).then_some(control.max as f64),
                value_now: control.value.map(|value| value as f64),
                value_text: control.value_text.clone().or_else(|| (!control.text.is_empty() && control.role != ControlRole::Button).then(|| control.text.clone())),
                busy: control.role == ControlRole::ProgressBar,
                ..Default::default()
            }
        })
        .collect()
}

/// 🎞️ The overlay a slot paints: its panel rectangle, its contained picture and its transport rows, every row but the root group.
pub fn media_overlay(slot: &PresentedMediaSlot, quad: Option<&MediaQuad>, controls: &[Control], focus: Option<ControlKind>) -> MediaOverlay {
    let rows = controls
        .iter()
        .filter(|control| control.kind != ControlKind::Root)
        .map(|control| {
            let shape = row_shape(control);
            let text = match shape {
                MediaOverlayShape::Slider => String::new(),
                MediaOverlayShape::Button => if control.text.is_empty() { control.label.clone() } else { control.text.clone() },
                _ => control.value_text.clone().filter(|value| !value.is_empty()).unwrap_or_else(|| if control.text.is_empty() { control.label.clone() } else { control.text.clone() }),
            };
            let fraction = control.value.filter(|_| control.max > control.min).map(|value| (value.saturating_sub(control.min)) as f32 / (control.max - control.min) as f32);
            MediaOverlayRow { shape, rect: [control.rect.x, control.rect.y, control.rect.w, control.rect.h], text, fraction, enabled: control.enabled, focused: focus == Some(control.kind) }
        })
        .collect();
    MediaOverlay { window: media_window_handle(&slot.window_id), node_id: slot.node_id.parse().unwrap_or(0), rect: [slot.rect.x, slot.rect.y, slot.rect.width, slot.rect.height], quad: quad.map(|quad| MediaOverlayQuad { rect: [quad.rect.x, quad.rect.y, quad.rect.w, quad.rect.h], raster_key: quad.raster_key.clone() }), rows }
}

/// 🎛️ The native media host of one window: every slot's player, audio stream and transport, the overlays it paints and the input it takes.
pub struct NativeMediaHost {
    presenter: MediaSlotPresenter,
    visible: Vec<String>,
    focus: Option<(String, ControlKind)>,
    drag: Option<(String, ControlKind)>,
    press: Option<(String, ControlKind)>,
}

impl NativeMediaHost {
    /// 🏗️ A host that plays through the operating system's audio output; `can_apply` shows the cut button when the owner takes the cut action.
    pub fn new(can_apply: bool) -> Self {
        Self::with_audio(Box::new(open_system_audio_output), can_apply)
    }

    /// 🏗️ A host that opens its audio through `open_audio` (a law's recording output).
    pub fn with_audio(open_audio: AudioOutputOpener, can_apply: bool) -> Self {
        Self { presenter: MediaSlotPresenter::new(open_audio, can_apply), visible: Vec::new(), focus: None, drag: None, press: None }
    }

    /// 🖼️ Presents one frame's slots: reconcile, tick every slot by `delta_ms`, upload new pictures, collect quads and controls.
    pub fn present(&mut self, slots: &[PresentedMediaSlot], delta_ms: u64, uploads: &mut dyn RasterUploadPort) -> MediaFrame {
        self.presenter.reconcile(slots.iter().map(slot_descriptor).collect());
        let mut frame = MediaFrame::default();
        self.visible.clear();
        for slot in slots {
            self.presenter.tick(&slot.token, delta_ms, uploads);
            if let Some(quad) = self.presenter.quad(&slot.token) {
                frame.quads.push((slot.token.clone(), quad));
            }
            frame.controls.push((slot.token.clone(), self.presenter.controls(&slot.token)));
            if !slot_descriptor(slot).occluded {
                self.visible.push(slot.token.clone());
            }
        }
        if self.focus.as_ref().is_some_and(|(token, _)| !slots.iter().any(|slot| &slot.token == token)) {
            self.focus = None;
        }
        frame
    }

    /// 🎞️ Presents the frame and publishes the overlays the retained paint draws over each slot's panel; `true` while any slot plays, so the host keeps asking for frames.
    pub fn present_overlays(&mut self, slots: &[PresentedMediaSlot], delta_ms: u64, uploads: &mut dyn RasterUploadPort) -> bool {
        let frame = self.present(slots, delta_ms, uploads);
        let overlays: Vec<MediaOverlay> = slots
            .iter()
            .filter(|slot| self.visible.contains(&slot.token))
            .map(|slot| {
                let quad = frame.quads.iter().find(|(token, _)| token == &slot.token).map(|(_, quad)| quad);
                let controls = frame.controls.iter().find(|(token, _)| token == &slot.token).map_or(&[][..], |(_, controls)| controls.as_slice());
                media_overlay(slot, quad, controls, self.focus.as_ref().filter(|(token, _)| token == &slot.token).map(|(_, kind)| *kind))
            })
            .collect();
        publish_media_overlays(overlays);
        slots.iter().any(|slot| self.presenter.playing(&slot.token)) || !self.presenter.awaiting_bytes().is_empty()
    }

    fn control_at(&self, x: f32, y: f32) -> Option<(String, ControlKind)> {
        self.visible.iter().rev().find_map(|token| self.presenter.hit_test(token, x, y).map(|kind| (token.clone(), kind)))
    }

    /// 🖱️ Routes one pointer event: a press focuses the control under it and starts a drag on a range; the release presses a button that is still under the pointer. A press outside every control is `Ignored`.
    pub fn pointer(&mut self, phase: MediaPointer, x: f32, y: f32) -> MediaInput {
        match phase {
            MediaPointer::Down => {
                let Some((token, kind)) = self.control_at(x, y) else { return MediaInput::Ignored };
                self.presenter.focus(&token, Some(kind));
                self.focus = Some((token.clone(), kind));
                if matches!(kind, ControlKind::Seek | ControlKind::SelectionStart | ControlKind::SelectionEnd) {
                    if let Some(value) = self.presenter.value_at(&token, kind, x) {
                        self.presenter.set_value(&token, kind, value);
                    }
                    self.drag = Some((token, kind));
                } else {
                    self.press = Some((token, kind));
                }
                MediaInput::Consumed
            }
            MediaPointer::Move => {
                let Some((token, kind)) = self.drag.clone() else { return if self.press.is_some() { MediaInput::Consumed } else { MediaInput::Ignored } };
                if let Some(value) = self.presenter.value_at(&token, kind, x) {
                    self.presenter.set_value(&token, kind, value);
                }
                MediaInput::Consumed
            }
            MediaPointer::Up => {
                if self.drag.take().is_some() {
                    return MediaInput::Consumed;
                }
                let Some((token, kind)) = self.press.take() else { return MediaInput::Ignored };
                if self.control_at(x, y) != Some((token.clone(), kind)) {
                    return MediaInput::Consumed;
                }
                if kind == ControlKind::Cut {
                    return self.presenter.cut(&token).map_or(MediaInput::Consumed, |request| MediaInput::Cut { token, request });
                }
                self.presenter.activate(&token, kind);
                MediaInput::Consumed
            }
        }
    }

    /// ⇥️ The focusable rows of one slot in the browser overlay's tab order: its enabled buttons and ranges in reading order. Between slots the page's own focus traversal decides, each slot being one stop of it.
    fn tab_stops(&self, token: &str) -> Vec<ControlKind> {
        self.presenter.controls(token).into_iter().filter(|control| control.enabled && matches!(control.role, ControlRole::Button | ControlRole::Slider)).map(|control| control.kind).collect()
    }

    /// 🔦️ The slot that holds the media focus.
    pub fn focused_token(&self) -> Option<&str> {
        self.focus.as_ref().map(|(token, _)| token.as_str())
    }

    /// 🚪️ Takes the focus into slot `token` the way Tab reaches an overlay: at its first tab stop, or at its last when `backward`. `false` when the slot has no tab stop, so the page's traversal goes on.
    pub fn enter(&mut self, token: &str, backward: bool) -> bool {
        let stops = self.tab_stops(token);
        let Some(kind) = (if backward { stops.last() } else { stops.first() }).copied().filter(|_| self.visible.iter().any(|held| held == token)) else { return false };
        self.presenter.focus(token, Some(kind));
        self.focus = Some((token.to_owned(), kind));
        true
    }

    /// 🚪️ Lets go of the media focus (the page's focus moved elsewhere).
    pub fn release(&mut self) {
        if let Some((token, _)) = self.focus.take() {
            self.presenter.focus(&token, None);
        }
    }

    /// ⇥️ Moves the focus to the next (or, `backward`, previous) tab stop of the focused slot; `Consumed` when it moved. From the last stop forward and the first stop backward the focus leaves the slot (the page's traversal takes it to the next stop, which may be the next slot) and the key is `Ignored`; a key with no media focus is `Ignored` too.
    pub fn tab(&mut self, backward: bool) -> MediaInput {
        let Some((token, kind)) = self.focus.clone() else { return MediaInput::Ignored };
        let stops = self.tab_stops(&token);
        let position = stops.iter().position(|stop| *stop == kind);
        let target = match (position, backward) {
            (Some(index), false) => stops.get(index + 1),
            (Some(index), true) => index.checked_sub(1).and_then(|previous| stops.get(previous)),
            (None, false) => stops.first(),
            (None, true) => stops.last(),
        }
        .copied();
        match target {
            Some(next) => {
                self.presenter.focus(&token, Some(next));
                self.focus = Some((token, next));
                MediaInput::Consumed
            }
            None => {
                self.release();
                MediaInput::Ignored
            }
        }
    }

    /// ⌨️ Routes one key to the slot that holds the focus; `Consumed` when the transport took it.
    pub fn key(&mut self, key: &str, shift: bool, ctrl: bool, alt: bool, meta: bool) -> MediaInput {
        if key == "Tab" && !ctrl && !alt && !meta {
            return self.tab(shift);
        }
        let Some((token, _)) = self.focus.clone() else { return MediaInput::Ignored };
        if self.presenter.key(&token, key, shift, ctrl, alt, meta) { MediaInput::Consumed } else { MediaInput::Ignored }
    }

    /// 🎬️ Whether a slot plays or still loads, so its drawing keeps changing.
    pub fn is_active(&self, token: &str) -> bool {
        self.presenter.playing(token) || self.presenter.awaiting_bytes().iter().any(|awaiting| awaiting == token) || matches!(self.presenter.load_state(token), Some(SlotLoad::Loading { .. }))
    }

    /// ♿️ The accessibility rows of every visible slot, by slot token.
    pub fn accessibility_nodes(&self) -> Vec<(String, Vec<AccessibilityProjectionNode>)> {
        self.visible
            .iter()
            .map(|token| (token.clone(), media_accessibility_nodes(&self.presenter.controls(token), self.focus.as_ref().filter(|(held, _)| held == token).map(|(_, kind)| *kind))))
            .collect()
    }

    /// ♿️ Applies what an assistive technology asked of the row `key` of slot `token`: focus moves the focus, activate presses a button (the cut answers its request), a value sets a range to that many milliseconds.
    pub fn accessibility(&mut self, token: &str, key: &str, event: MediaAccessibility) -> MediaInput {
        let Some(kind) = control_of_key(key).filter(|_| self.visible.iter().any(|held| held == token)) else { return MediaInput::Ignored };
        match event {
            MediaAccessibility::Focus => {
                self.presenter.focus(token, Some(kind));
                self.focus = Some((token.to_owned(), kind));
            }
            MediaAccessibility::Blur => {
                if self.focus.as_ref() == Some(&(token.to_owned(), kind)) {
                    self.presenter.focus(token, None);
                    self.focus = None;
                }
            }
            MediaAccessibility::Activate if kind == ControlKind::Cut => {
                return self.presenter.cut(token).map_or(MediaInput::Consumed, |request| MediaInput::Cut { token: token.to_owned(), request });
            }
            MediaAccessibility::Activate => self.presenter.activate(token, kind),
            MediaAccessibility::Value(text) => {
                let Ok(value) = text.trim().parse::<u64>() else { return MediaInput::Consumed };
                self.presenter.set_value(token, kind, value);
            }
        }
        MediaInput::Consumed
    }

    /// 🏁️ Takes the answer of a decode that ran off the host's thread.
    pub fn finish_decode(&mut self, token: &str, decoded: Result<DecodedMedia, NativeMediaFault>) {
        self.presenter.finish_decode(token, decoded);
    }

    /// 🔎️ The slots that still wait for their medium's bytes.
    pub fn awaiting_bytes(&self) -> Vec<String> {
        self.presenter.awaiting_bytes()
    }

    /// 📥️ Hands a slot its medium's bytes.
    pub fn provide_bytes(&mut self, token: &str, bytes: &[u8]) {
        self.presenter.provide_bytes(token, bytes);
    }

    /// 🎛️ The presenter, for the transfer's progress and cancel.
    pub fn presenter(&mut self) -> &mut MediaSlotPresenter {
        &mut self.presenter
    }

    /// 🧹️ Closes every slot (and its audio stream) and withdraws the overlays.
    pub fn close(&mut self) {
        self.presenter.close();
        self.visible.clear();
        self.focus = None;
        publish_media_overlays(Vec::new());
    }
}

impl Drop for NativeMediaHost {
    fn drop(&mut self) {
        self.presenter.close();
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
