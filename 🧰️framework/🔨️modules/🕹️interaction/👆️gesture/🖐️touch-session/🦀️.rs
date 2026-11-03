//! 🖐️ Touch session: the domain-neutral recogniser every viewport host feeds with raw contacts and a clock. It tells a tap from a long press from a drag, defers the primary press until the finger has decided (so a second finger never starts a marquee under a pinch), and turns two fingers into the camera step of the shared [`GestureRecognizer`]. A held finger asks for its long press at the deadline [`LONG_PRESS_MS`], while it is still down: [`TouchSession::next_deadline_ms`] tells the host when to wake it with [`TouchSession::tick`], no polling. Pure and deterministic: time is an argument, nothing is read from a clock or a timer.
//!
//! 🔗️ Replayed on the language-agnostic fixture `🖐️touch-session.json`; the camera law is [`apply_pinch_to_orbit`](super::apply_pinch_to_orbit); the wheel and pan units of the native orbit controller are [`pinch_navigation`]; see `visual-editor-architecture.md` §8.4.

use super::{GesturePointer, GestureRecognizer, GestureVerdict, OrbitProjection, PinchStep};

pub const TOUCH_SLOP_PX: f64 = 8.0;
pub const LONG_PRESS_MS: f64 = 450.0;
pub const DOUBLE_TAP_MS: f64 = 300.0;
pub const DOUBLE_TAP_SLOP_PX: f64 = 24.0;
pub const STALE_CONTACT_MS: f64 = 3000.0;

/// 🧭️ What the host does after one contact event, in order.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TouchStep {
    Press { x: f64, y: f64, additive: bool },
    Drag { x: f64, y: f64, dx: f64, dy: f64, additive: bool },
    Release { x: f64, y: f64, additive: bool },
    Orbit { x: f64, y: f64, dx: f64, dy: f64 },
    Camera(PinchStep),
    CameraEnd,
    LongPress { x: f64, y: f64 },
    Tap { x: f64, y: f64 },
    DoubleTap { x: f64, y: f64 },
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Phase {
    Idle,
    Pending { id: u64, x0: f64, y0: f64, t0: f64 },
    Orbiting { id: u64, x: f64, y: f64 },
    Forwarding { id: u64, x: f64, y: f64, additive: bool },
    Camera,
}

/// 🖐️ The session of one surface. `one_finger_orbit` sends a one-finger drag to the camera (tap-to-nudge devices, where a drag never edits a handle); `long_press` turns a held finger into an additive press. Both are read when a finger lands.
#[derive(Clone, Debug)]
pub struct TouchSession {
    pub one_finger_orbit: bool,
    pub long_press: bool,
    phase: Phase,
    recognizer: GestureRecognizer,
    last_tap: Option<(f64, f64, f64)>,
    last_ms: f64,
}

impl Default for TouchSession {
    fn default() -> Self {
        Self { one_finger_orbit: false, long_press: true, phase: Phase::Idle, recognizer: GestureRecognizer::default(), last_tap: None, last_ms: 0.0 }
    }
}

fn far(x0: f64, y0: f64, x: f64, y: f64) -> bool {
    (x - x0).hypot(y - y0) > TOUCH_SLOP_PX
}

impl TouchSession {
    /// 🔽️ A finger lands at `(x, y)` at `now` milliseconds. A gesture whose last contact is older than [`STALE_CONTACT_MS`] lost its lift; it is cancelled first so a lost release never turns the next finger into a second contact.
    pub fn down(&mut self, now: f64, id: u64, x: f64, y: f64) -> Vec<TouchStep> {
        let mut steps = if self.is_active() && now - self.last_ms > STALE_CONTACT_MS { self.cancel() } else { Vec::new() };
        self.last_ms = now;
        let verdict = self.recognizer.down(GesturePointer { id, x, y });
        match (verdict, self.phase) {
            (GestureVerdict::Single, Phase::Idle) => self.phase = Phase::Pending { id, x0: x, y0: y, t0: now },
            (GestureVerdict::PinchBegin(_), phase) => {
                if let Phase::Forwarding { x, y, additive, .. } = phase {
                    steps.push(TouchStep::Release { x, y, additive });
                }
                self.phase = Phase::Camera;
            }
            _ => {}
        }
        steps
    }

    /// 🖐️ A finger moves to `(x, y)` at `now` milliseconds.
    pub fn moved(&mut self, now: f64, id: u64, x: f64, y: f64) -> Vec<TouchStep> {
        self.last_ms = now;
        let verdict = self.recognizer.move_to(GesturePointer { id, x, y });
        if let GestureVerdict::Pinch(step) = verdict {
            return vec![TouchStep::Camera(step)];
        }
        match self.phase {
            Phase::Pending { id: owner, x0, y0, t0 } if owner == id => {
                if far(x0, y0, x, y) {
                    if self.one_finger_orbit {
                        self.phase = Phase::Orbiting { id, x, y };
                        return Vec::new();
                    }
                    self.phase = Phase::Forwarding { id, x, y, additive: false };
                    return vec![TouchStep::Press { x: x0, y: y0, additive: false }, TouchStep::Drag { x, y, dx: x - x0, dy: y - y0, additive: false }];
                }
                if self.long_press && now - t0 >= LONG_PRESS_MS {
                    self.phase = Phase::Forwarding { id, x, y, additive: true };
                    return vec![TouchStep::Press { x: x0, y: y0, additive: true }, TouchStep::LongPress { x: x0, y: y0 }];
                }
                Vec::new()
            }
            Phase::Orbiting { id: owner, x: px, y: py } if owner == id => {
                self.phase = Phase::Orbiting { id, x, y };
                vec![TouchStep::Orbit { x, y, dx: x - px, dy: y - py }]
            }
            Phase::Forwarding { id: owner, x: px, y: py, additive } if owner == id => {
                self.phase = Phase::Forwarding { id, x, y, additive };
                vec![TouchStep::Drag { x, y, dx: x - px, dy: y - py, additive }]
            }
            _ => Vec::new(),
        }
    }

    /// 👋️ A finger lifts (or the system cancels it) at `now` milliseconds.
    pub fn up(&mut self, now: f64, id: u64) -> Vec<TouchStep> {
        self.last_ms = now;
        let verdict = self.recognizer.up(id);
        let steps = match (verdict, self.phase) {
            (GestureVerdict::PinchEnd, _) => {
                self.phase = Phase::Idle;
                vec![TouchStep::CameraEnd]
            }
            (_, Phase::Pending { id: owner, x0, y0, t0 }) if owner == id => {
                self.phase = Phase::Idle;
                if self.long_press && now - t0 >= LONG_PRESS_MS {
                    vec![TouchStep::Press { x: x0, y: y0, additive: true }, TouchStep::LongPress { x: x0, y: y0 }, TouchStep::Release { x: x0, y: y0, additive: true }]
                } else {
                    let mut steps = vec![TouchStep::Press { x: x0, y: y0, additive: false }, TouchStep::Release { x: x0, y: y0, additive: false }, TouchStep::Tap { x: x0, y: y0 }];
                    if self.last_tap.is_some_and(|(t, px, py)| now - t <= DOUBLE_TAP_MS && (x0 - px).hypot(y0 - py) <= DOUBLE_TAP_SLOP_PX) {
                        self.last_tap = None;
                        steps.push(TouchStep::DoubleTap { x: x0, y: y0 });
                    } else {
                        self.last_tap = Some((now, x0, y0));
                    }
                    steps
                }
            }
            (_, Phase::Orbiting { id: owner, .. }) if owner == id => {
                self.phase = Phase::Idle;
                Vec::new()
            }
            (_, Phase::Forwarding { id: owner, x, y, additive, .. }) if owner == id => {
                self.phase = Phase::Idle;
                vec![TouchStep::Release { x, y, additive }]
            }
            _ => Vec::new(),
        };
        if self.recognizer.pointer_count() == 0 && !self.recognizer.is_latched() && !matches!(self.phase, Phase::Idle) {
            self.phase = Phase::Idle;
        }
        steps
    }

    /// ⏱️ The clock moved to `now` with no contact event: a finger held still past [`LONG_PRESS_MS`] becomes an additive press and asks for its long press at once, while it is still down.
    pub fn tick(&mut self, now: f64) -> Vec<TouchStep> {
        match self.phase {
            Phase::Pending { id, x0, y0, t0 } if self.long_press && now - t0 >= LONG_PRESS_MS => {
                self.phase = Phase::Forwarding { id, x: x0, y: y0, additive: true };
                vec![TouchStep::Press { x: x0, y: y0, additive: true }, TouchStep::LongPress { x: x0, y: y0 }]
            }
            _ => Vec::new(),
        }
    }

    /// ⏳️ When a still finger turns into a long press, or `None` when nothing waits on the clock.
    pub fn next_deadline_ms(&self) -> Option<f64> {
        match self.phase {
            Phase::Pending { t0, .. } if self.long_press => Some(t0 + LONG_PRESS_MS),
            _ => None,
        }
    }

    /// 🛑️ The system cancelled every contact: forget the gesture; a forwarded press is released where it stands.
    pub fn cancel(&mut self) -> Vec<TouchStep> {
        let steps = match self.phase {
            Phase::Forwarding { x, y, additive, .. } => vec![TouchStep::Release { x, y, additive }],
            _ => Vec::new(),
        };
        self.phase = Phase::Idle;
        self.recognizer = GestureRecognizer::default();
        steps
    }

    /// 🔒️ Whether a gesture is in flight (a finger is down).
    pub fn is_active(&self) -> bool {
        !matches!(self.phase, Phase::Idle)
    }
}

/// 🧭️ One camera step in the units of the native orbit controller: `wheel` is the dolly (perspective) or zoom (parallel) wheel delta of `OrbitController::zoom`, `pan_x` and `pan_y` are the screen-pixel deltas of a shift-right-drag whose `OrbitController::pan` moves target and eye exactly as the web orbit rig does.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CameraNavigation {
    pub wheel: f64,
    pub pan_x: f64,
    pub pan_y: f64,
}

/// 🎥️ The native orbit controller units of one [`PinchStep`], so the native hosts reproduce [`apply_pinch_to_orbit`](super::apply_pinch_to_orbit) with the wheel and drag intents they already have: dolly (`distance * (1 - wheel / 1000)` equals `distance / scale`) or zoom (`zoom * (1 + wheel / 1000)` equals `zoom * scale`), and a pan scaled from the controller's `0.001 * distance` (perspective) or `1 / zoom` (parallel) pixel to the world length of one pixel at the target. A degenerate scale does not zoom; a non-positive viewport height answers no motion.
pub fn pinch_navigation(step: PinchStep, projection: OrbitProjection, viewport_height: f64) -> CameraNavigation {
    if !(viewport_height > 0.0) {
        return CameraNavigation { wheel: 0.0, pan_x: 0.0, pan_y: 0.0 };
    }
    let scale = if step.scale.is_finite() && step.scale > 0.0 { step.scale } else { 1.0 };
    let (wheel, k) = match projection {
        OrbitProjection::Perspective { fov_y_radians } => (1000.0 * (1.0 - 1.0 / scale), 2000.0 * (fov_y_radians / 2.0).tan() / viewport_height),
        OrbitProjection::Orthographic { frustum_height } => (1000.0 * (scale - 1.0), frustum_height / viewport_height),
    };
    CameraNavigation { wheel, pan_x: step.pan_x * k, pan_y: -step.pan_y * k }
}

#[cfg(test)]
mod tests {
    use super::super::{apply_pinch_to_orbit, OrbitBounds, OrbitPose, ZoomBounds};
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    struct Fixture {
        session: Vec<Case>,
        orbit: Vec<OrbitCase>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Case {
        name: String,
        #[serde(default)]
        one_finger_orbit: bool,
        #[serde(default = "enabled")]
        long_press: bool,
        events: Vec<Event>,
        steps: Vec<Vec<String>>,
        #[serde(default)]
        deadlines: Option<Vec<Option<f64>>>,
    }

    fn enabled() -> bool {
        true
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Event {
        #[serde(rename = "type")]
        kind: String,
        t: f64,
        #[serde(default)]
        pointer_id: u64,
        #[serde(default)]
        x: f64,
        #[serde(default)]
        y: f64,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct OrbitCase {
        name: String,
        pose: [[f64; 3]; 3],
        zoom: f64,
        step: [f64; 5],
        projection: Vec<f64>,
        viewport_height: f64,
        expect_position: [f64; 3],
        expect_target: [f64; 3],
        expect_zoom: f64,
    }

    fn kinds(steps: &[TouchStep]) -> Vec<String> {
        steps
            .iter()
            .map(|step| match step {
                TouchStep::Press { additive, .. } => format!("press{}", if *additive { "+" } else { "" }),
                TouchStep::Drag { additive, .. } => format!("drag{}", if *additive { "+" } else { "" }),
                TouchStep::Release { additive, .. } => format!("release{}", if *additive { "+" } else { "" }),
                TouchStep::Orbit { .. } => "orbit".to_owned(),
                TouchStep::Camera(_) => "camera".to_owned(),
                TouchStep::CameraEnd => "cameraEnd".to_owned(),
                TouchStep::LongPress { .. } => "longPress".to_owned(),
                TouchStep::Tap { .. } => "tap".to_owned(),
                TouchStep::DoubleTap { .. } => "doubleTap".to_owned(),
            })
            .collect()
    }

    fn fixture() -> Fixture {
        serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🖐️touch-session.json")).expect("touch session fixture")
    }

    #[test]
    fn the_language_agnostic_touch_rows_replay_to_the_same_steps() {
        for case in fixture().session {
            let mut session = TouchSession { one_finger_orbit: case.one_finger_orbit, long_press: case.long_press, ..TouchSession::default() };
            assert_eq!(case.events.len(), case.steps.len(), "{}: one step list per event", case.name);
            for (index, (event, expected)) in case.events.iter().zip(&case.steps).enumerate() {
                let steps = match event.kind.as_str() {
                    "down" => session.down(event.t, event.pointer_id, event.x, event.y),
                    "move" => session.moved(event.t, event.pointer_id, event.x, event.y),
                    "up" => session.up(event.t, event.pointer_id),
                    "tick" => session.tick(event.t),
                    "cancel" => session.cancel(),
                    other => panic!("unknown event {other}"),
                };
                assert_eq!(&kinds(&steps), expected, "{} at t={} {}", case.name, event.t, event.kind);
                if let Some(deadlines) = &case.deadlines {
                    assert_eq!(session.next_deadline_ms(), deadlines[index], "{} wake time after t={} {}", case.name, event.t, event.kind);
                }
            }
        }
    }

    #[test]
    fn the_wake_time_is_known_the_moment_the_finger_lands_so_an_idle_host_can_sleep_until_then() {
        let mut session = TouchSession::default();
        assert_eq!(session.next_deadline_ms(), None, "no finger, no wake");
        session.down(1000.0, 1, 5.0, 5.0);
        assert_eq!(session.next_deadline_ms(), Some(1000.0 + LONG_PRESS_MS), "the host sleeps exactly until the long press is due");
        assert_eq!(kinds(&session.tick(1000.0 + LONG_PRESS_MS)), ["press+", "longPress"]);
        assert_eq!(kinds(&session.up(1700.0, 1)), ["release+"], "the lift of a long press does not ask for the menu a second time");
    }

    #[test]
    fn a_held_finger_waits_on_the_clock_only_while_it_has_not_moved() {
        let mut session = TouchSession::default();
        session.down(0.0, 1, 10.0, 10.0);
        assert_eq!(session.next_deadline_ms(), Some(LONG_PRESS_MS));
        assert!(session.tick(LONG_PRESS_MS - 1.0).is_empty());
        assert_eq!(kinds(&session.tick(LONG_PRESS_MS)), ["press+", "longPress"], "the menu is asked for while the finger is still down");
        assert_eq!(session.next_deadline_ms(), None);
        let mut moved = TouchSession::default();
        moved.down(0.0, 1, 10.0, 10.0);
        moved.moved(10.0, 1, 40.0, 10.0);
        assert!(moved.tick(1000.0).is_empty(), "a finger that left its slop is no long press");
    }

    #[test]
    fn the_orbit_rows_match_the_typescript_authority() {
        for case in fixture().orbit {
            let pose = OrbitPose { position: case.pose[0], target: case.pose[1], up: case.pose[2], zoom: case.zoom };
            let step = PinchStep { scale: case.step[0], pan_x: case.step[1], pan_y: case.step[2], rotation: 0.0, centroid_x: case.step[3], centroid_y: case.step[4] };
            let (projection, bounds) = match case.projection[0] as u8 {
                0 => (OrbitProjection::Perspective { fov_y_radians: case.projection[1] }, OrbitBounds { distance: ZoomBounds { min: 0.5, max: 500.0 }, zoom: ZoomBounds { min: 0.1, max: 100.0 } }),
                _ => (OrbitProjection::Orthographic { frustum_height: case.projection[1] }, OrbitBounds { distance: ZoomBounds { min: 0.5, max: 500.0 }, zoom: ZoomBounds { min: 0.1, max: 100.0 } }),
            };
            let next = apply_pinch_to_orbit(pose, step, projection, case.viewport_height, bounds);
            for axis in 0..3 {
                assert!((next.position[axis] - case.expect_position[axis]).abs() < 1.0e-9, "{} position {axis}: {:?}", case.name, next.position);
                assert!((next.target[axis] - case.expect_target[axis]).abs() < 1.0e-9, "{} target {axis}: {:?}", case.name, next.target);
            }
            assert!((next.zoom - case.expect_zoom).abs() < 1.0e-9, "{} zoom", case.name);
        }
    }

    fn native_pan(position: &mut [f64; 3], target: &mut [f64; 3], up: [f64; 3], dx: f64, dy: f64) {
        let offset = [position[0] - target[0], position[1] - target[1], position[2] - target[2]];
        let distance = offset.iter().map(|v| v * v).sum::<f64>().sqrt();
        let cross = |a: [f64; 3], b: [f64; 3]| [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
        let unit = |a: [f64; 3]| {
            let length = a.iter().map(|v| v * v).sum::<f64>().sqrt();
            [a[0] / length, a[1] / length, a[2] / length]
        };
        let right = unit(cross(offset, up));
        let camera_up = unit(cross(right, offset));
        let scale = distance * 0.001;
        for axis in 0..3 {
            let moved = right[axis] * (-dx * scale) + camera_up[axis] * (dy * scale);
            target[axis] += moved;
            position[axis] += moved;
        }
    }

    #[test]
    fn the_native_wheel_and_drag_units_reproduce_the_web_orbit_law() {
        let pose = OrbitPose { position: [10.0, -6.0, 7.0], target: [1.0, 2.0, 0.5], up: [0.0, 0.0, 1.0], zoom: 1.0 };
        let step = PinchStep { scale: 1.6, pan_x: 30.0, pan_y: -12.0, rotation: 0.0, centroid_x: 400.0, centroid_y: 300.0 };
        let projection = OrbitProjection::Perspective { fov_y_radians: 45.0_f64.to_radians() };
        let bounds = OrbitBounds { distance: ZoomBounds { min: 0.5, max: 500.0 }, zoom: ZoomBounds { min: 0.1, max: 100.0 } };
        let web = apply_pinch_to_orbit(pose, step, projection, 800.0, bounds);
        let navigation = pinch_navigation(step, projection, 800.0);
        let offset = [pose.position[0] - pose.target[0], pose.position[1] - pose.target[1], pose.position[2] - pose.target[2]];
        let dollied = 1.0 - navigation.wheel / 1000.0;
        let mut position = [pose.target[0] + offset[0] * dollied, pose.target[1] + offset[1] * dollied, pose.target[2] + offset[2] * dollied];
        let mut target = pose.target;
        native_pan(&mut position, &mut target, pose.up, -navigation.pan_x, -navigation.pan_y);
        for axis in 0..3 {
            assert!((position[axis] - web.position[axis]).abs() < 1.0e-9, "position {axis}: {position:?} vs {:?}", web.position);
            assert!((target[axis] - web.target[axis]).abs() < 1.0e-9, "target {axis}: {target:?} vs {:?}", web.target);
        }
    }

    #[test]
    fn a_degenerate_pinch_does_not_move_the_camera() {
        let still = PinchStep { scale: f64::NAN, pan_x: 0.0, pan_y: 0.0, rotation: 0.0, centroid_x: 0.0, centroid_y: 0.0 };
        assert_eq!(pinch_navigation(still, OrbitProjection::Perspective { fov_y_radians: 0.8 }, 600.0), CameraNavigation { wheel: 0.0, pan_x: 0.0, pan_y: 0.0 });
        assert_eq!(pinch_navigation(still, OrbitProjection::Orthographic { frustum_height: 10.0 }, 0.0), CameraNavigation { wheel: 0.0, pan_x: 0.0, pan_y: 0.0 });
    }
}
