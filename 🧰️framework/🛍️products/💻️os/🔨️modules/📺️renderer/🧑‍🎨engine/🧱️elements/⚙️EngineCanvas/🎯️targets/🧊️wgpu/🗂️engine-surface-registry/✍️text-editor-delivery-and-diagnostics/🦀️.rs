//! ✍️ Text editor delivery and diagnostics: the text editor delivery state and publication, the engine canvas log helpers, the published graph interaction and the engine camera parser. See [`super`].

use super::*;

//#region ✍️TextEditorDelivery
pub const TEXT_EDITOR_PENDING_ECHO_CAPACITY: usize = 256;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TextEditorDeliverySnapshot {
    pub controller_id: String,
    pub surface_id: String,
    pub text: String,
    pub start: usize,
    pub end: usize,
}

/// ⌨️ The editor's open typing run as its host sees it (design §13.2 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): the
/// window folds every `textEdit` delivery that names the buffer (`typing`) into ONE run; the host ends it on idle, on a pure
/// caret move, when the editor loses focus and when the page is hidden, with one commit signal, never one edit per keystroke.
#[derive(Clone, Debug, PartialEq)]
pub struct TextEditorTypingRun {
    pub controller_id: String,
    pub surface_id: String,
    pub last_ms: f64,
}

/// 🏁️ Why the host ends an open typing run before its idle bound (the `typingCommit` reasons of the text-splice corpus
/// `hostSignals`): the editor lost keyboard focus, or the page was hidden or left.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TextEditorTypingEnd {
    Blur,
    Hidden,
}

impl TextEditorTypingEnd {
    /// 🏷️ The `typingCommit` reason the commit signal carries.
    pub fn reason(self) -> &'static str {
        match self {
            Self::Blur => "blur",
            Self::Hidden => "hidden",
        }
    }
}

/// ⏰️ The host clock the typing run's idle bound reads.
#[cfg(target_arch = "wasm32")]
pub fn text_editor_now_ms() -> f64 {
    js_sys::Date::now()
}

#[cfg(not(target_arch = "wasm32"))]
pub fn text_editor_now_ms() -> f64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|duration| duration.as_secs_f64() * 1000.0).unwrap_or(0.0)
}

pub struct TextEditorDeliveryInFlight {
    pub source: EngineSurfaceToken,
    pub token: std::num::NonZeroU64,
    pub snapshot: TextEditorDeliverySnapshot,
    pub pending_members: u8,
}

pub struct TextEditorDraftPublication {
    pub source: String,
    pub restore: Option<String>,
    pub action: semio_framework_ui_action::RetainedStringAction,
}

pub struct TextEditorDeliveryPublication {
    pub source: String,
    pub action: semio_framework_ui_action::RetainedStringAction,
    pub snapshot: TextEditorDeliverySnapshot,
    pub include_selection: bool,
}

#[derive(Default)]
pub struct TextEditorDeliveryState {
    pub active: Option<TextEditorDeliveryInFlight>,
    pub latest: Option<TextEditorDeliverySnapshot>,
    pub publication: Option<TextEditorDeliveryPublication>,
    pub pending_echoes: VecDeque<String>,
    acknowledged: Option<String>,
    pub read_only: bool,
    pub selection_undeclared: bool,
    pub explicit_dirty: bool,
    pub explicit_base: Option<String>,
    pub explicit_changes: Option<String>,
    pub explicit_conflict: bool,
    pub explicit_past: VecDeque<String>,
    pub explicit_future: VecDeque<String>,
    pub explicit_snapshot: Option<String>,
    pub explicit_publication: Option<TextEditorDraftPublication>,
    pub explicit_active: Option<std::num::NonZeroU64>,
    pub explicit_error: Option<String>,
    pub typing: Option<TextEditorTypingRun>,
    pub typing_end: Option<TextEditorTypingEnd>,
}

impl TextEditorDeliveryState {
    pub fn reconcile_buffer(&mut self, buffer: &str) -> bool {
        if let Some(index) = self.pending_echoes.iter().position(|pending| pending == buffer) {
            self.pending_echoes.drain(..=index);
            self.acknowledged = Some(buffer.to_owned());
            return false;
        }
        if self.acknowledged.as_deref() == Some(buffer) {
            return false;
        }
        self.pending_echoes.clear();
        self.acknowledged = Some(buffer.to_owned());
        true
    }

    pub fn offer(&mut self, snapshot: TextEditorDeliverySnapshot) {
        if self.latest.as_ref() == Some(&snapshot) || self.active.as_ref().is_some_and(|active| active.snapshot == snapshot) {
            return;
        }
        if self.publication.as_ref().is_some_and(|publication| publication.snapshot != snapshot) {
            self.publication = None;
        }
        self.latest = Some(snapshot);
    }

    /// 🏁️ Marks the open typing run (or the one its in-flight keystroke opens) to end for `end` once nothing is in flight;
    /// `false` when the editor neither types nor has a delivery in flight.
    pub fn end_typing(&mut self, end: TextEditorTypingEnd) -> bool {
        let typing = self.typing.is_some() || self.active.is_some() || self.latest.is_some() || self.publication.is_some();
        if typing {
            self.typing_end = Some(end);
        }
        typing
    }

    pub fn guest_has_text(&self, text: &str) -> bool {
        self.pending_echoes.back().map_or_else(|| self.acknowledged.as_deref() == Some(text), |pending| pending == text)
    }

    pub fn note_dispatched(&mut self, source: EngineSurfaceToken, token: std::num::NonZeroU64, include_edit: bool, include_selection: bool) -> bool {
        if self.active.is_some() {
            return false;
        }
        let Some(snapshot) = self.latest.take() else { return false };
        if include_edit {
            if self.pending_echoes.len() == TEXT_EDITOR_PENDING_ECHO_CAPACITY {
                self.pending_echoes.pop_front();
            }
            self.pending_echoes.push_back(snapshot.text.clone());
        }
        let pending_members = u8::from(include_edit) | (u8::from(include_selection) << 1);
        self.active = Some(TextEditorDeliveryInFlight { source, token, snapshot, pending_members });
        true
    }

    pub fn remove_pending_echo(&mut self, text: &str) -> bool {
        let Some(index) = self.pending_echoes.iter().position(|pending| pending == text) else { return false };
        self.pending_echoes.remove(index);
        true
    }

    pub(in super::super) fn close_string(value: &mut String) -> bool {
        retire_string_page(value)
    }

    fn close_snapshot(snapshot: &mut TextEditorDeliverySnapshot) -> bool {
        Self::close_string(&mut snapshot.text) || Self::close_string(&mut snapshot.surface_id) || Self::close_string(&mut snapshot.controller_id)
    }

    pub(in super::super) fn close_step(&mut self) -> bool {
        if self.latest.as_mut().is_some_and(Self::close_snapshot) {
            return false;
        }
        self.latest = None;
        if self.publication.as_mut().is_some_and(|publication| Self::close_string(&mut publication.source) || Self::close_snapshot(&mut publication.snapshot)) {
            return false;
        }
        self.publication = None;
        if self.active.as_mut().is_some_and(|active| Self::close_snapshot(&mut active.snapshot)) {
            return false;
        }
        self.active = None;
        if self.pending_echoes.back_mut().is_some_and(Self::close_string) {
            return false;
        }
        if self.pending_echoes.pop_back().is_some() {
            return false;
        }
        if self.acknowledged.as_mut().is_some_and(Self::close_string) {
            return false;
        }
        self.acknowledged = None;
        self.read_only = false;
        self.selection_undeclared = false;
        self.explicit_dirty = false;
        if self.explicit_base.as_mut().is_some_and(Self::close_string) {
            return false;
        }
        self.explicit_base = None;
        if self.explicit_changes.as_mut().is_some_and(Self::close_string) {
            return false;
        }
        self.explicit_changes = None;
        self.explicit_conflict = false;
        if self.explicit_past.back_mut().is_some_and(Self::close_string) || self.explicit_future.back_mut().is_some_and(Self::close_string) {
            return false;
        }
        self.explicit_past.pop_back();
        self.explicit_future.pop_back();
        if !self.explicit_past.is_empty() || !self.explicit_future.is_empty() {
            return false;
        }
        self.explicit_snapshot = None;
        self.explicit_publication = None;
        self.explicit_active = None;
        self.explicit_error = None;
        if self.typing.as_mut().is_some_and(|run| Self::close_string(&mut run.controller_id) || Self::close_string(&mut run.surface_id)) {
            return false;
        }
        self.typing = None;
        self.typing_end = None;
        true
    }

    pub(in super::super) fn terminal_is_empty(&self) -> bool {
        self.active.is_none()
            && self.latest.is_none()
            && self.publication.is_none()
            && self.pending_echoes.is_empty()
            && self.acknowledged.is_none()
            && !self.read_only
            && !self.selection_undeclared
            && !self.explicit_dirty
            && self.explicit_base.is_none()
            && !self.explicit_conflict
            && self.explicit_past.is_empty()
            && self.explicit_future.is_empty()
            && self.explicit_snapshot.is_none()
            && self.explicit_publication.is_none()
            && self.explicit_active.is_none()
            && self.explicit_error.is_none()
            && self.typing.is_none()
            && self.typing_end.is_none()
    }
}
//#endregion ✍️TextEditorDelivery

//#region 🪵️EngineCanvasDiagnostics
/// 🚨️ Console ERROR line for a refusal at the engine edge — a scene the editor engine would not take, a member this
/// host could not read. `eprintln!` is a no-op inside a `wasm32-unknown-unknown` Worker.
pub fn engine_canvas_fault_log(line: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(line));
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{line}");
}

/// 🩺️ Console line for a screen-path gesture. `eprintln!` is a no-op inside a
/// `wasm32-unknown-unknown` Worker, so a wire drawn there would otherwise leave no trace at all.
pub fn engine_canvas_debug_log(line: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(line));
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{line}");
}

/// 📌️ The fingerprints of the three interaction actions a node-graph surface last dispatched.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PublishedGraphInteraction {
    pub select: u64,
    pub hover: u64,
    pub viewport: u64,
}

pub fn published_graph_interaction(surface_id: &str) -> PublishedGraphInteraction {
    ENGINE_SURFACES.with(|cell| cell.borrow().get(surface_id).map(|entry| entry.published_graph_interaction).unwrap_or_default())
}
//#endregion 🪵️EngineCanvasDiagnostics

//#region 📷️EngineCamera
/// 🎥️ A `{x, y, zoom}` camera document — the shape React's `parseCameraJson`/`parseBoardCamera` read
/// and the shape both hosts' `set_camera` take.
pub fn engine_camera_from_json(json: &str) -> Option<(f64, f64, f64)> {
    let value = serde_json::from_str::<Value>(json).ok()?;
    let x = value.get("x").and_then(Value::as_f64)?;
    let y = value.get("y").and_then(Value::as_f64)?;
    let zoom = value.get("zoom").and_then(Value::as_f64).filter(|zoom| *zoom > 0.0)?;
    Some((x, y, zoom))
}
//#endregion 📷️EngineCamera
