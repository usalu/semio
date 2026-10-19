//! ✍️ Drag, scroll and edit routing: the drag promotion, update and drop target resolution, the scroll routing and the text, ime and key edit routing with the clipboard intents. See [`super`].

use super::*;

//#region 🔖️Outcome
impl Dispatcher {
    //#region 🫳️DragApi
    pub(super) fn maybe_promote_to_drag(&mut self, pointer: PointerId, x: f32, y: f32) {
        let Some(entry) = self.capture.get(&pointer).copied() else { return };
        if entry.kind != CaptureKind::Press {
            return;
        }
        let Some(payload) = self.drag_payloads.get(&entry.element).cloned() else { return };
        let Some((origin_x, origin_y)) = self.press_origin.get(&pointer).copied() else { return };
        if (x - origin_x).powi(2) + (y - origin_y).powi(2) < DRAG_PROMOTE_THRESHOLD_SQ {
            return;
        }
        self.capture.insert(pointer, CaptureEntry { element: entry.element, kind: CaptureKind::Drag, revision: entry.revision });
        self.drag = Some(DragSession { source: entry.element, payload, ghost: None, pointer_x: x, pointer_y: y, drop_target: None });
    }

    pub(super) fn update_drag(&mut self, tree: &DispatchTree, root: FrameNodeId, x: f32, y: f32) {
        if let Some(drag) = self.drag.as_mut() {
            drag.pointer_x = x;
            drag.pointer_y = y;
        }
        let target = hit_test(tree, root, x, y).and_then(|hit| self.nearest_accepting_drop_target(tree, hit));
        let element = target.and_then(|id| tree.node(id)).map(|node| node.element);
        if let Some(drag) = self.drag.as_mut() {
            drag.drop_target = element;
        }
    }

    fn nearest_accepting_drop_target(&self, tree: &DispatchTree, from: FrameNodeId) -> Option<FrameNodeId> {
        let mut found = None;
        bubble(tree, from, |id| {
            let Some(node) = tree.node(id) else { return false };
            if !node.flags.contains(DispatchFlags::DROP_TARGET) {
                return false;
            }
            let accepts = match self.drop_accept.get(&node.element) {
                Some(predicate) => self.drag.as_ref().is_some_and(|drag| predicate(&drag.payload)),
                None => true,
            };
            if accepts {
                found = Some(id);
                true
            } else {
                false
            }
        });
        found
    }

    /// 👻️ Attaches a ghost to the active drag session — a host/widget calls this once a session is
    /// promoted; see [`DragGhost`]'s own doc for why painting it is next-frame work, not this call's.
    pub fn set_drag_ghost(&mut self, ghost: DragGhost) {
        if let Some(drag) = self.drag.as_mut() {
            drag.ghost = Some(ghost);
        }
    }
    //#endregion 🫳️DragApi

    //#region 🖱️ScrollApi
    pub(super) fn route_scroll(&mut self, tree: &DispatchTree, root: FrameNodeId, x: f32, y: f32, delta_x: f32, delta_y: f32) -> bool {
        let Some(hit) = hit_test(tree, root, x, y) else { return false };
        let Some(scrollable) = nearest_scrollable_ancestor(tree, hit) else { return false };
        let Some(element) = tree.node(scrollable).map(|node| node.element) else { return false };
        if !self.scroll_offsets.contains_key(&element) && self.scroll_offsets.len() >= 256 {
            return false;
        }
        let (offset_x, offset_y) = self.scroll_offset(element);
        self.scroll_offsets.insert(element, ((offset_x + delta_x).max(0.0), (offset_y + delta_y).max(0.0)));
        true
    }

    pub(super) fn update_scroll_thumb(&mut self, scrollable: ElementId, axis: ScrollAxis, pointer: PointerId, x: f32, y: f32) {
        let Some((origin_x, origin_y, start_x, start_y)) = self.thumb_start.get(&pointer).copied() else { return };
        let (delta_x, delta_y) = (x - origin_x, y - origin_y);
        let offset = match axis {
            ScrollAxis::Horizontal => ((start_x + delta_x).max(0.0), start_y),
            ScrollAxis::Vertical => (start_x, (start_y + delta_y).max(0.0)),
        };
        self.scroll_offsets.insert(scrollable, offset);
    }
    //#endregion 🖱️ScrollApi

    //#region ✍️EditApi
    pub(super) fn route_text_insert(&mut self, text: &str) -> bool {
        let Some(id) = self.focus else { return false };
        let Some(edit) = self.edit_states.get_mut(&id) else { return false };
        insert_at_caret(edit, text);
        true
    }

    pub(super) fn route_ime(&mut self, event: &ImeEvent) -> bool {
        let Some(id) = self.focus else { return false };
        let Some(edit) = self.edit_states.get_mut(&id) else { return false };
        match event {
            ImeEvent::Start => edit.composition = Some(String::new()),
            ImeEvent::Update { text, .. } => edit.composition = Some(text.clone()),
            ImeEvent::Commit { text } => {
                edit.composition = None;
                insert_at_caret(edit, text);
            }
            ImeEvent::Cancel => edit.composition = None,
        }
        true
    }

    /// ⌨️ Ported from `events.rs::route_edit_key` verbatim for caret motion/`Home`/`End`/`Backspace`/
    /// `Delete`; the clipboard shortcuts synthesize a `UiIntent` against a well-known dispatch-owned
    /// [`ActionId`] (`scope: "dispatch"`) rather than a bespoke `UiCommand` variant, since
    /// `DispatchOutcome` has no side-channel beyond `intents` — see this file's report for the
    /// trade-off. A no-operation if nothing is focused or the focused element has no [`EditState`].
    pub(super) fn route_edit_key(&mut self, key: &str, modifiers: EventModifiers) -> (bool, Option<UiIntent>) {
        let Some(id) = self.focus else { return (false, None) };
        let Some(edit) = self.edit_states.get_mut(&id) else { return (false, None) };
        let has_selection = edit.anchor != edit.caret;
        match key {
            "ArrowLeft" => {
                edit.caret = if has_selection && !modifiers.shift { selection_bounds(edit.anchor, edit.caret).0 } else { prev_char_boundary(&edit.text, edit.caret) };
                if !modifiers.shift {
                    edit.anchor = edit.caret;
                }
            }
            "ArrowRight" => {
                edit.caret = if has_selection && !modifiers.shift { selection_bounds(edit.anchor, edit.caret).1 } else { next_char_boundary(&edit.text, edit.caret) };
                if !modifiers.shift {
                    edit.anchor = edit.caret;
                }
            }
            "Home" => {
                edit.caret = 0;
                if !modifiers.shift {
                    edit.anchor = 0;
                }
            }
            "End" => {
                edit.caret = edit.text.len();
                if !modifiers.shift {
                    edit.anchor = edit.text.len();
                }
            }
            "Backspace" => {
                if has_selection {
                    let (start, end) = selection_bounds(edit.anchor, edit.caret);
                    edit.text.replace_range(start..end, "");
                    edit.caret = start;
                    edit.anchor = start;
                } else if edit.caret > 0 {
                    let start = prev_char_boundary(&edit.text, edit.caret);
                    edit.text.replace_range(start..edit.caret, "");
                    edit.caret = start;
                    edit.anchor = start;
                }
            }
            "Delete" => {
                if has_selection {
                    let (start, end) = selection_bounds(edit.anchor, edit.caret);
                    edit.text.replace_range(start..end, "");
                    edit.caret = start;
                    edit.anchor = start;
                } else if edit.caret < edit.text.len() {
                    let end = next_char_boundary(&edit.text, edit.caret);
                    edit.text.replace_range(edit.caret..end, "");
                }
            }
            "c" | "C" if modifiers.ctrl || modifiers.meta => {
                if !has_selection {
                    return (false, None);
                }
                let (start, end) = selection_bounds(edit.anchor, edit.caret);
                let text = edit.text[start..end].to_string();
                return (true, self.build_clipboard_intent("clipboardCopy", text));
            }
            "x" | "X" if modifiers.ctrl || modifiers.meta => {
                if !has_selection {
                    return (false, None);
                }
                let (start, end) = selection_bounds(edit.anchor, edit.caret);
                let text = edit.text[start..end].to_string();
                edit.text.replace_range(start..end, "");
                edit.caret = start;
                edit.anchor = start;
                return (true, self.build_clipboard_intent("clipboardCut", text));
            }
            "v" | "V" if modifiers.ctrl || modifiers.meta => {
                return (true, self.build_clipboard_intent("clipboardPasteRequested", String::new()));
            }
            _ => return (false, None),
        }
        (true, None)
    }

    fn build_clipboard_intent(&mut self, name: &str, text: String) -> Option<UiIntent> {
        let action = ActionId::try_v1("dispatch", name)?;
        let input = (!text.is_empty()).then(|| UiText::try_from_string(text).ok()).flatten().map(UiValue::Text);
        self.seq += 1;
        Some(UiIntent {
            surface: SurfaceId::default(),
            revision: UiRevision::default(),
            node: UiNodeId::default(),
            node_key: UiText::default(),
            trigger: Trigger::Commit,
            action,
            args: None,
            input,
            seq: self.seq,
        })
    }
    //#endregion ✍️EditApi

}
//#endregion 🔖️Outcome
