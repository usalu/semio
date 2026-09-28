//! 🤏️ Pure pointer-set, pinch-frame, and 2D camera laws shared by native viewport hosts.

/// 👆️ One active contact in surface-local pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GesturePointer {
    pub id: u64,
    pub x: f64,
    pub y: f64,
}

/// 🤏️ One two-contact frame.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PinchFrame {
    pub centroid_x: f64,
    pub centroid_y: f64,
    pub distance: f64,
    pub angle: f64,
}

/// 🧭️ One incremental pinch transform.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PinchStep {
    pub scale: f64,
    pub pan_x: f64,
    pub pan_y: f64,
    pub rotation: f64,
    pub centroid_x: f64,
    pub centroid_y: f64,
}

/// 🪄️ The surface action selected by one pointer transition.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum GestureVerdict {
    Single,
    PinchBegin(PinchFrame),
    Pinch(PinchStep),
    Held,
    PinchEnd,
}

/// 🧠️ Per-surface contact memory, latched until the final pinch contact leaves.
#[derive(Clone, Debug, Default)]
pub struct GestureRecognizer {
    pointers: Vec<GesturePointer>,
    frame: Option<PinchFrame>,
    latched: bool,
}

impl GestureRecognizer {
    /// 👆️ Feeds one contact press.
    pub fn down(&mut self, pointer: GesturePointer) -> GestureVerdict {
        match self.pointers.iter().position(|candidate| candidate.id == pointer.id) {
            Some(index) => self.pointers[index] = pointer,
            None => self.pointers.push(pointer),
        }
        if self.pointers.len() < 2 {
            return if self.latched { GestureVerdict::Held } else { GestureVerdict::Single };
        }
        let frame = self.pinch_frame().expect("two contacts have a pinch frame");
        self.frame = Some(frame);
        if self.latched {
            GestureVerdict::Held
        } else {
            self.latched = true;
            GestureVerdict::PinchBegin(frame)
        }
    }

    /// 🖐️ Feeds one contact move.
    pub fn move_to(&mut self, pointer: GesturePointer) -> GestureVerdict {
        let Some(index) = self.pointers.iter().position(|candidate| candidate.id == pointer.id) else {
            return if self.latched { GestureVerdict::Held } else { GestureVerdict::Single };
        };
        self.pointers[index] = pointer;
        if !self.latched {
            return GestureVerdict::Single;
        }
        let next = self.pinch_frame();
        let verdict = match (self.frame, next) {
            (Some(previous), Some(next)) => GestureVerdict::Pinch(pinch_step(previous, next)),
            _ => GestureVerdict::Held,
        };
        self.frame = next;
        verdict
    }

    /// 👋️ Feeds one contact lift or cancellation.
    pub fn up(&mut self, id: u64) -> GestureVerdict {
        let Some(index) = self.pointers.iter().position(|candidate| candidate.id == id) else {
            return if self.latched { GestureVerdict::Held } else { GestureVerdict::Single };
        };
        self.pointers.remove(index);
        if !self.latched {
            return GestureVerdict::Single;
        }
        if self.pointers.is_empty() {
            self.frame = None;
            self.latched = false;
            GestureVerdict::PinchEnd
        } else {
            self.frame = self.pinch_frame();
            GestureVerdict::Held
        }
    }

    /// 🔒️ Whether a multi-contact gesture owns the surface.
    pub fn is_latched(&self) -> bool {
        self.latched
    }

    /// 🔢️ Number of retained contacts.
    pub fn pointer_count(&self) -> usize {
        self.pointers.len()
    }

    /// 🧹️ Releases one retained contact allocation unit for bounded surface retirement.
    pub fn retire_step(&mut self) -> bool {
        if self.pointers.pop().is_some() {
            return true;
        }
        if self.pointers.capacity() != 0 {
            self.pointers = Vec::new();
            self.frame = None;
            self.latched = false;
            return true;
        }
        false
    }

    fn pinch_frame(&self) -> Option<PinchFrame> {
        let first = self.pointers.first()?;
        let second = self.pointers.get(1)?;
        let dx = second.x - first.x;
        let dy = second.y - first.y;
        Some(PinchFrame { centroid_x: (first.x + second.x) * 0.5, centroid_y: (first.y + second.y) * 0.5, distance: dx.hypot(dy), angle: dy.atan2(dx) })
    }
}

const PINCH_MIN_DISTANCE_PX: f64 = 1.0e-3;

fn shortest_angle_delta(from: f64, to: f64) -> f64 {
    let two_pi = std::f64::consts::TAU;
    let mut delta = (to - from).rem_euclid(two_pi);
    if delta > std::f64::consts::PI {
        delta -= two_pi;
    }
    if delta == -std::f64::consts::PI { std::f64::consts::PI } else { delta }
}

fn pinch_step(previous: PinchFrame, next: PinchFrame) -> PinchStep {
    let degenerate = previous.distance < PINCH_MIN_DISTANCE_PX || next.distance < PINCH_MIN_DISTANCE_PX;
    PinchStep {
        scale: if degenerate { 1.0 } else { next.distance / previous.distance },
        pan_x: next.centroid_x - previous.centroid_x,
        pan_y: next.centroid_y - previous.centroid_y,
        rotation: if degenerate { 0.0 } else { shortest_angle_delta(previous.angle, next.angle) },
        centroid_x: next.centroid_x,
        centroid_y: next.centroid_y,
    }
}

/// 📷️ A centered 2D viewport camera.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Camera2d {
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

/// 🔍️ Inclusive zoom bounds.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ZoomBounds {
    pub min: f64,
    pub max: f64,
}

/// 📐️ Applies one finger-anchored pinch step to a centered 2D camera.
pub fn apply_pinch_to_camera(camera: Camera2d, step: PinchStep, viewport: [f64; 2], bounds: ZoomBounds) -> Camera2d {
    let zoom = if camera.zoom > 0.0 { camera.zoom } else { 1.0 };
    let candidate = zoom * step.scale;
    let next_zoom = if candidate.is_finite() && candidate > 0.0 { candidate.clamp(bounds.min, bounds.max) } else { bounds.min };
    let world_x = camera.x + (step.centroid_x - step.pan_x - viewport[0] * 0.5) / zoom;
    let world_y = camera.y + (step.centroid_y - step.pan_y - viewport[1] * 0.5) / zoom;
    Camera2d { x: world_x - (step.centroid_x - viewport[0] * 0.5) / next_zoom, y: world_y - (step.centroid_y - viewport[1] * 0.5) / next_zoom, zoom: next_zoom }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Fixture {
        recognizer: Vec<Case>,
    }

    #[derive(Deserialize)]
    struct Case {
        events: Vec<Event>,
        verdicts: Vec<Expected>,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Event {
        #[serde(rename = "type")]
        kind: String,
        pointer_id: u64,
        #[serde(default)]
        x: f64,
        #[serde(default)]
        y: f64,
    }

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct Expected {
        kind: String,
        centroid_x: Option<f64>,
        centroid_y: Option<f64>,
        distance: Option<f64>,
        scale: Option<f64>,
        pan_x: Option<f64>,
        pan_y: Option<f64>,
        rotation: Option<f64>,
    }

    #[test]
    fn neutral_recognizer_fixture_matches_the_typescript_authority() {
        let fixture: Fixture = serde_json::from_str(include_str!("🧫️fixtures/🤏️recognizer.json")).expect("recognizer fixture");
        for case in fixture.recognizer {
            let mut recognizer = GestureRecognizer::default();
            for (event, expected) in case.events.into_iter().zip(case.verdicts) {
                let verdict = match event.kind.as_str() {
                    "down" => recognizer.down(GesturePointer { id: event.pointer_id, x: event.x, y: event.y }),
                    "move" => recognizer.move_to(GesturePointer { id: event.pointer_id, x: event.x, y: event.y }),
                    "up" => recognizer.up(event.pointer_id),
                    kind => panic!("unknown event {kind}"),
                };
                match (verdict, expected.kind.as_str()) {
                    (GestureVerdict::Single, "single") | (GestureVerdict::Held, "held") | (GestureVerdict::PinchEnd, "pinchEnd") => {}
                    (GestureVerdict::PinchBegin(frame), "pinchBegin") => {
                        close(frame.centroid_x, expected.centroid_x.expect("centroid x"));
                        close(frame.centroid_y, expected.centroid_y.expect("centroid y"));
                        close(frame.distance, expected.distance.expect("distance"));
                    }
                    (GestureVerdict::Pinch(step), "pinch") => {
                        if let Some(value) = expected.scale { close(step.scale, value); }
                        if let Some(value) = expected.pan_x { close(step.pan_x, value); }
                        if let Some(value) = expected.pan_y { close(step.pan_y, value); }
                        if let Some(value) = expected.rotation { close(step.rotation, value); }
                    }
                    (actual, expected) => panic!("expected {expected}, got {actual:?}"),
                }
            }
        }
    }

    #[test]
    fn pinch_camera_matches_the_neutral_spread_and_translation_vectors() {
        let camera = Camera2d { x: 0.0, y: 0.0, zoom: 1.0 };
        let bounds = ZoomBounds { min: 0.1, max: 100.0 };
        let spread = apply_pinch_to_camera(camera, PinchStep { scale: 2.0, pan_x: 0.0, pan_y: 0.0, rotation: 0.0, centroid_x: 400.0, centroid_y: 300.0 }, [800.0, 600.0], bounds);
        assert_eq!(spread, Camera2d { x: 0.0, y: 0.0, zoom: 2.0 });
        let translated = apply_pinch_to_camera(camera, PinchStep { scale: 1.0, pan_x: 40.0, pan_y: 20.0, rotation: 0.0, centroid_x: 440.0, centroid_y: 320.0 }, [800.0, 600.0], bounds);
        assert_eq!(translated, Camera2d { x: -40.0, y: -20.0, zoom: 1.0 });
    }

    fn close(actual: f64, expected: f64) {
        assert!((actual - expected).abs() < 1.0e-9, "expected {expected}, got {actual}");
    }
}
