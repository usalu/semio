//! 👆️ Touch word selection of the text editor: React's `createTouchSelectionV1` (`🧱️elements/✏️TextEditor/👆️touch-selection/🟦️.ts`). A finger held still for [`LONG_PRESS_MS`], or a second tap close to the first in time and place, selects the word under it; a drag past the slop, a cancel or an early lift disarms the hold. Mouse and pen keep their own click-count path. Coordinates are window-local, so the slop and the selection share one frame.

use super::*;
use std::{cell::RefCell, collections::BTreeMap};

pub(super) const LONG_PRESS_MS: f64 = 500.0;
pub(super) const DOUBLE_TAP_MS: f64 = 350.0;
pub(super) const SLOP_PX: f32 = 10.0;

#[derive(Clone, Copy)]
pub(super) enum Touch {
    Down { x: f32, y: f32 },
    Move { x: f32, y: f32 },
    Up,
    Cancel,
}

#[derive(Default)]
struct Recognizer {
    window_id: String,
    origin: Option<(f32, f32, f64)>,
    fired: bool,
    last_tap: Option<(f32, f32, f64)>,
}

fn far(x: f32, y: f32, from: (f32, f32)) -> bool {
    (x - from.0).hypot(y - from.1) > SLOP_PX
}

impl Recognizer {
    fn feed(&mut self, now: f64, touch: Touch) -> Option<(f32, f32)> {
        match touch {
            Touch::Down { x, y } => {
                self.origin = Some((x, y, now));
                self.fired = false;
            }
            Touch::Move { x, y } => {
                if self.origin.is_some_and(|(ox, oy, _)| far(x, y, (ox, oy))) {
                    self.origin = None;
                }
            }
            Touch::Cancel => self.origin = None,
            Touch::Up => {
                let (origin, fired) = (self.origin.take()?, std::mem::take(&mut self.fired));
                if fired {
                    return None;
                }
                let (x, y, _) = origin;
                if self.last_tap.is_some_and(|(tx, ty, at)| now - at <= DOUBLE_TAP_MS && !far(x, y, (tx, ty))) {
                    self.last_tap = None;
                    return Some((x, y));
                }
                self.last_tap = Some((x, y, now));
            }
        }
        None
    }

    fn due(&mut self, now: f64) -> Option<(f32, f32)> {
        let (x, y, at) = self.origin.filter(|_| !self.fired)?;
        (now - at >= LONG_PRESS_MS).then(|| {
            self.fired = true;
            self.last_tap = None;
            (x, y)
        })
    }

    fn deadline(&self) -> Option<f64> {
        self.origin.filter(|_| !self.fired).map(|(_, _, at)| at + LONG_PRESS_MS)
    }
}

thread_local! {
    static RECOGNIZERS: RefCell<BTreeMap<String, Recognizer>> = RefCell::new(BTreeMap::new());
}

/// ⏳️ The wall-clock time (milliseconds of [`app_now_ms`]) at which the earliest held finger of any text editor selects its word, or `None` without one.
pub(super) fn next_deadline_ms() -> Option<f64> {
    RECOGNIZERS.with(|recognizers| recognizers.borrow().values().filter_map(Recognizer::deadline).min_by(f64::total_cmp))
}

impl AppInteractionState {
    /// 👆️ One contact of a touch pointer on the text editor `host_id` of `window_id`; a double tap selects at once.
    pub(super) fn text_touch(&mut self, window_id: &str, host_id: &str, touch: Touch) {
        let selected = RECOGNIZERS.with(|recognizers| {
            let mut recognizers = recognizers.borrow_mut();
            let recognizer = recognizers.entry(host_id.to_owned()).or_default();
            if matches!(touch, Touch::Down { .. }) {
                recognizer.window_id = window_id.to_owned();
            }
            recognizer.feed(app_now_ms(), touch).map(|point| (recognizer.window_id.clone(), point))
        });
        if let Some((window_id, (x, y))) = selected {
            self.select_text_word(&window_id, x, y);
        }
    }

    /// ⏱️ The frame's clock for the text editors: a finger held still past the long-press deadline selects its word without waiting for the next contact event.
    pub fn poll_text_touch(&mut self) {
        let now = app_now_ms();
        let due: Vec<(String, f32, f32)> = RECOGNIZERS.with(|recognizers| recognizers.borrow_mut().values_mut().filter_map(|recognizer| recognizer.due(now).map(|(x, y)| (recognizer.window_id.clone(), x, y))).collect());
        for (window_id, x, y) in due {
            self.select_text_word(&window_id, x, y);
        }
    }

    fn select_text_word(&mut self, window_id: &str, x: f32, y: f32) {
        if let Err(fault) = semio_renderer_wgpu_ui_engine::scene_interaction::text_editor_select_word_at(window_id, x, y, &mut self.input) {
            self.input.record_action_fault(fault);
        }
    }

    /// 👆️ Window-local contact of a touch pointer at shell point `(x, y)` over the text editor `target`.
    pub(super) fn text_touch_at(&mut self, target: &semio_renderer_wgpu_contracts::ScenePointerTarget, x: f32, y: f32, touch: fn(f32, f32) -> Touch) {
        let Some(body) = self.shell.input.retained_window_body_rect(&target.window_id) else { return };
        self.text_touch(&target.window_id, &target.host_id, touch(x - body.x, y - body.y));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn at(recognizer: &mut Recognizer, now: f64, touch: Touch) -> Option<(f32, f32)> {
        recognizer.feed(now, touch)
    }

    #[test]
    fn a_held_finger_selects_once_and_its_lift_selects_nothing() {
        let mut recognizer = Recognizer::default();
        at(&mut recognizer, 0.0, Touch::Down { x: 5.0, y: 5.0 });
        assert_eq!(recognizer.deadline(), Some(LONG_PRESS_MS));
        assert_eq!(recognizer.due(LONG_PRESS_MS - 1.0), None);
        assert_eq!(recognizer.due(LONG_PRESS_MS), Some((5.0, 5.0)));
        assert_eq!(recognizer.due(LONG_PRESS_MS + 1.0), None);
        assert_eq!(at(&mut recognizer, 600.0, Touch::Up), None);
        assert_eq!(recognizer.last_tap, None);
    }

    #[test]
    fn a_drag_past_the_slop_or_a_cancel_disarms_the_hold() {
        let mut recognizer = Recognizer::default();
        at(&mut recognizer, 0.0, Touch::Down { x: 0.0, y: 0.0 });
        at(&mut recognizer, 10.0, Touch::Move { x: 5.0, y: 0.0 });
        assert!(recognizer.deadline().is_some());
        at(&mut recognizer, 20.0, Touch::Move { x: 20.0, y: 0.0 });
        assert_eq!(recognizer.deadline(), None);
        at(&mut recognizer, 30.0, Touch::Down { x: 0.0, y: 0.0 });
        at(&mut recognizer, 40.0, Touch::Cancel);
        assert_eq!(recognizer.due(1000.0), None);
    }

    #[test]
    fn a_second_tap_close_in_time_and_place_selects_and_a_late_or_far_one_does_not() {
        let mut recognizer = Recognizer::default();
        at(&mut recognizer, 0.0, Touch::Down { x: 0.0, y: 0.0 });
        assert_eq!(at(&mut recognizer, 50.0, Touch::Up), None);
        at(&mut recognizer, 100.0, Touch::Down { x: 4.0, y: 0.0 });
        assert_eq!(at(&mut recognizer, 150.0, Touch::Up), Some((4.0, 0.0)));
        at(&mut recognizer, 200.0, Touch::Down { x: 0.0, y: 0.0 });
        assert_eq!(at(&mut recognizer, 250.0, Touch::Up), None);
        at(&mut recognizer, 300.0, Touch::Down { x: 50.0, y: 0.0 });
        assert_eq!(at(&mut recognizer, 350.0, Touch::Up), None);
        at(&mut recognizer, 900.0, Touch::Down { x: 50.0, y: 0.0 });
        assert_eq!(at(&mut recognizer, 950.0, Touch::Up), None);
    }
}
