//! 🧲️ Focus and outbox state impls of the board focus: the focus state and the outbox state methods. See [`super`].

use super::*;

impl BoardFocusState {
    pub fn set_selection_screen_preview(&mut self, points: Option<Vec<Point>>) {
        if points.is_none() {
            self.selection_preview_crossing = false;
        }
        self.selection_screen_preview = points;
    }

    pub fn sync_selection_screen_overlay(&mut self, start_screen: Point, screen_points: &[Point]) {
        if screen_points.len() < 2 {
            self.selection_screen_preview = None;
            self.selection_preview_crossing = false;
            return;
        }
        let last = *screen_points.last().unwrap_or(&start_screen);
        self.selection_preview_crossing = !selection_drag_enclosing(self.selection_options.method.as_str(), start_screen, screen_points);
        self.selection_screen_preview = Some(if self.selection_options.method == "lasso" { screen_points.to_vec() } else { vec![start_screen, Point::new(last.x, start_screen.y), last, Point::new(start_screen.x, last.y)] });
    }

    pub(crate) fn is_preselecting(&self) -> bool {
        matches!(&self.interaction, Interaction::Selection { .. })
    }

    /// 💠️ Live area-select preview ids, or committed selection when not preselecting.
    pub(super) fn selection_chrome_ids(&self) -> BTreeSet<String> {
        if self.is_preselecting() || !self.preselect.is_empty() {
            self.preselect.clone()
        } else {
            self.selection.clone()
        }
    }

    /// 🧿️ True during left‑button rectangle/lasso drag so callers can avoid descriptor round‑trips that fight the live marquee state.
    pub fn is_dragging_area_select(&self) -> bool {
        matches!(&self.interaction, Interaction::Selection { .. })
    }

    /// 🧭️ Source handle id while a link wire is drawn (`LinkDragSnap` / `LinkTargetNode`).
    pub(crate) fn active_link_source_handle_id(&self) -> Option<&str> {
        match &self.interaction {
            Interaction::LinkDragSnap { source_id, .. } | Interaction::LinkTargetNode { source_id, .. } | Interaction::ExternalLinkPreview { source_id, .. } => Some(source_id.as_str()),
            _ => None,
        }
    }
}

impl BoardOutboxState {
    pub(super) fn reserve_selection_event(&mut self, kind: BoardEventKind, ids: &[String], anchor_ids: Option<&[String]>, removed_ids: Option<&[String]>, gesture: Option<&str>) -> Option<BoardEventReservation> {
        self.reserve_owned_event(BoardOwnedEvent::selection(kind, ids, anchor_ids, removed_ids, gesture))
    }

    pub fn reserve_owned_event(&mut self, event: Result<BoardOwnedEvent, BoardEventFault>) -> Option<BoardEventReservation> {
        let event = match event {
            Ok(event) => event,
            Err(_) => {
                self.event_schema_fault = true;
                return None;
            }
        };
        match self.events.reserve_event(event) {
            Ok(reservation) => Some(reservation),
            Err(event) => {
                if self.event_overflow.is_none() {
                    self.event_overflow = Some(event);
                } else {
                    self.event_schema_fault = true;
                }
                None
            }
        }
    }

    pub fn reserve_owned_batch(&mut self, first: Result<BoardOwnedEvent, BoardEventFault>, second: Option<Result<BoardOwnedEvent, BoardEventFault>>) -> Option<BoardEventBatchReservation> {
        let first = match first {
            Ok(event) => event,
            Err(_) => {
                self.event_schema_fault = true;
                return None;
            }
        };
        let mut batch = BoardEventBatchReservation::one(first);
        if let Some(second) = second {
            let second = match second {
                Ok(event) => event,
                Err(_) => {
                    self.event_schema_fault = true;
                    return None;
                }
            };
            if batch.push(second).is_err() {
                self.event_schema_fault = true;
                return None;
            }
        }
        match self.events.reserve_batch(batch) {
            Ok(batch) => Some(batch),
            Err(batch) => {
                if self.event_batch_overflow.is_none() {
                    self.event_batch_overflow = Some(batch);
                } else {
                    self.event_schema_fault = true;
                }
                None
            }
        }
    }

    pub fn publish_event_reservation(&mut self, reservation: BoardEventReservation) {
        if self.events.publish_reserved(reservation).is_err() {
            self.event_schema_fault = true;
        }
    }

    pub fn publish_event_batch(&mut self, reservation: BoardEventBatchReservation) {
        if self.events.publish_batch(reservation).is_err() {
            self.event_schema_fault = true;
        }
    }

    //#region 🎬️Gestures
    /// 🪪️ Opens the next pointer gesture: a fresh id, nothing staged yet.
    pub fn begin_gesture(&mut self) -> GestureStage {
        self.gesture_serial = self.gesture_serial.wrapping_add(1);
        GestureStage { id: format!("gesture-{}", self.gesture_serial), select: None, restore: None }
    }

    /// 🏁️ Publishes one released gesture as ONE batch, so a host flushes `select` and the record together.
    pub fn publish_gesture_release(&mut self, gesture: &GestureStage, record: Option<Result<BoardOwnedEvent, BoardEventFault>>) {
        let (first, second) = match gesture_release_events(gesture, record) {
            [Some(select), record] => (select, record),
            [None, Some(record)] => (record, None),
            [None, None] => return,
        };
        if let Some(reservation) = self.reserve_owned_batch(first, second) {
            self.publish_event_batch(reservation);
        }
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn drain_events_json(&mut self) -> String {
        let mut out = String::from("[");
        let mut first = true;
        while let Some(event) = self.events.pop() {
            if !first {
                out.push(',');
            }
            first = false;
            event.write_json(&mut out);
        }
        if let Some(event) = self.event_overflow.take() {
            if !first {
                out.push(',');
            }
            event.write_json(&mut out);
        }
        out.push(']');
        out
    }

    pub fn pop_owned_event(&mut self) -> Option<BoardOwnedEvent> {
        if let Some(event) = self.events.pop().or_else(|| self.event_overflow.take()) {
            return Some(event);
        }
        let batch = self.event_batch_overflow.as_mut()?;
        let event = batch.pop();
        if batch.is_empty() {
            self.event_batch_overflow = None;
        }
        event
    }

    pub fn peek_owned_event(&self) -> Option<&BoardOwnedEvent> {
        self.events.iter().next().or(self.event_overflow.as_ref()).or_else(|| self.event_batch_overflow.as_ref().and_then(BoardEventBatchReservation::peek))
    }

    pub fn event_terminal_faulted(&self) -> bool {
        self.event_schema_fault
    }

    pub fn clear_link_gesture_events(&mut self) {
        let compat = self.link_compat_nodes_emit_key.is_some();
        let ring = self.link_target_ring_emit_key.is_some();
        let empty: &[String] = &[];
        let compat_event = compat.then(|| BoardOwnedEvent::link_compatible("", empty));
        let ring_event = ring.then(|| BoardOwnedEvent::link_ring("", None, empty));
        let (first, second) = match (compat_event, ring_event) {
            (Some(first), second) => (first, second),
            (None, Some(first)) => (first, None),
            (None, None) => return,
        };
        let Some(reservation) = self.reserve_owned_batch(first, second) else {
            return;
        };
        if compat {
            self.link_compat_nodes_emit_key = None;
        }
        if ring {
            self.link_target_ring_emit_key = None;
        }
        self.publish_event_batch(reservation);
    }
}
