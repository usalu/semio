//! 🎞️ Chrome motion: CSS cubic-bezier easing, keyed value transitions and enter progress driven by the draw list's frame clock. See [`super`].

use std::cell::RefCell;
use std::collections::HashMap;

const TRANSITION_TRACK_CAPACITY: usize = 4096;

/// 📈️ CSS `cubic-bezier(x1, y1, x2, y2)` evaluated at `t` in `0..=1`: solves the curve's x for its parameter by bisection, then returns y.
pub fn cubic_bezier(easing: [f32; 4], t: f32) -> f32 {
    if !t.is_finite() || t <= 0.0 {
        return 0.0;
    }
    if t >= 1.0 {
        return 1.0;
    }
    let [x1, y1, x2, y2] = easing;
    let axis = |a: f32, b: f32, s: f32| 3.0 * a * s * (1.0 - s) * (1.0 - s) + 3.0 * b * s * s * (1.0 - s) + s * s * s;
    let (mut low, mut high) = (0.0_f32, 1.0_f32);
    for _ in 0..40 {
        let mid = (low + high) * 0.5;
        if axis(x1, x2, mid) < t {
            low = mid;
        } else {
            high = mid;
        }
    }
    axis(y1, y2, (low + high) * 0.5)
}

/// 🪟️ Enter progress `0..=1` after `elapsed_seconds`; reduced motion lands on the end state at once.
pub fn enter_progress(elapsed_seconds: f32, duration_seconds: f32, easing: [f32; 4], reduced: bool) -> f32 {
    if reduced || duration_seconds <= 0.0 || !elapsed_seconds.is_finite() {
        return 1.0;
    }
    cubic_bezier(easing, (elapsed_seconds / duration_seconds).clamp(0.0, 1.0))
}

/// 🔎️ The dialog content scale at `visible` (`0` hidden, `1` shown) between `from` and `1`: `zoom-in-95` on enter and `zoom-out-95` on close.
pub fn dialog_zoom(visible: f32, from: f32) -> f32 {
    from + (1.0 - from) * visible.clamp(0.0, 1.0)
}

/// 💫️ `introduced-border-pulse`: opacity `1 -> 0.7 -> 1` over one period, `phase` in cycles.
pub fn introduced_pulse_opacity(phase: f32) -> f32 {
    0.85 + 0.15 * (phase * std::f32::consts::TAU).cos()
}

#[derive(Clone, Copy, Debug)]
struct TransitionTrack {
    from: f32,
    to: f32,
    start: f32,
}

/// 🎚️ Keyed value transitions: a changed target restarts from the value reached so far, like a CSS transition.
#[derive(Default)]
pub struct Transitions {
    tracks: HashMap<u64, TransitionTrack>,
    enters: HashMap<u64, f32>,
}

impl Transitions {
    pub fn value(&mut self, key: u64, target: f32, now: f32, duration_seconds: f32, easing: [f32; 4], reduced: bool) -> f32 {
        if reduced || now <= 0.0 || !now.is_finite() || duration_seconds <= 0.0 {
            self.tracks.insert(key, TransitionTrack { from: target, to: target, start: now });
            return target;
        }
        if self.tracks.len() >= TRANSITION_TRACK_CAPACITY && !self.tracks.contains_key(&key) {
            self.tracks.clear();
        }
        let sample = |track: &TransitionTrack| {
            if now < track.start {
                return track.to;
            }
            track.from + (track.to - track.from) * cubic_bezier(easing, ((now - track.start) / duration_seconds).clamp(0.0, 1.0))
        };
        let Some(track) = self.tracks.get(&key).copied() else {
            self.tracks.insert(key, TransitionTrack { from: target, to: target, start: now });
            return target;
        };
        if track.to != target {
            let current = sample(&track);
            self.tracks.insert(key, TransitionTrack { from: current, to: target, start: now });
            return current;
        }
        sample(&track)
    }

    /// 🪟️ Enter progress of `key`, started at its first sight; the clock never going backwards past the start restarts it.
    pub fn enter(&mut self, key: u64, now: f32, duration_seconds: f32, easing: [f32; 4], reduced: bool) -> f32 {
        if reduced || now <= 0.0 || !now.is_finite() {
            return 1.0;
        }
        if self.enters.len() >= TRANSITION_TRACK_CAPACITY && !self.enters.contains_key(&key) {
            self.enters.clear();
        }
        let start = *self.enters.entry(key).or_insert(now);
        enter_progress((now - start).max(0.0), duration_seconds, easing, false)
    }

    /// 🧹️ Forgets every enter whose key carries `salt` in its high half.
    pub fn forget_enters(&mut self, salt: u32) {
        self.enters.retain(|key, _| (key >> 32) as u32 != salt);
    }

    pub fn pending(&self, key: u64, now: f32, duration_seconds: f32) -> bool {
        self.tracks.get(&key).is_some_and(|track| track.from != track.to && now - track.start < duration_seconds)
    }
}

thread_local! {
    static CHROME_TRANSITIONS: RefCell<Transitions> = RefCell::new(Transitions::default());
}

/// 🎛️ The shell chrome's shared transition registry (one per render thread): the eased value of `target` for `key`.
pub fn chrome_transition(key: u64, target: f32, now: f32, duration_seconds: f32, easing: [f32; 4], reduced: bool) -> f32 {
    CHROME_TRANSITIONS.with(|transitions| transitions.borrow_mut().value(key, target, now, duration_seconds, easing, reduced))
}

/// ⏳️ Whether the shared transition of `key` is still moving at `now`.
pub fn chrome_transition_pending(key: u64, now: f32, duration_seconds: f32) -> bool {
    CHROME_TRANSITIONS.with(|transitions| transitions.borrow().pending(key, now, duration_seconds))
}

/// 🪟️ The shared registry's enter progress of `(salt, index)`.
pub fn chrome_enter(salt: u32, index: u32, now: f32, duration_seconds: f32, easing: [f32; 4], reduced: bool) -> f32 {
    CHROME_TRANSITIONS.with(|transitions| transitions.borrow_mut().enter((u64::from(salt) << 32) | u64::from(index), now, duration_seconds, easing, reduced))
}

/// 🧹️ Forgets every shared enter of `salt`, so the next appearance fades in again.
pub fn chrome_enter_forget(salt: u32) {
    CHROME_TRANSITIONS.with(|transitions| transitions.borrow_mut().forget_enters(salt));
}

/// 🔑️ Stable transition key of a control by its rect and a caller salt.
pub fn transition_key(rect: [f32; 4], salt: u32) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64 ^ u64::from(salt);
    for value in rect {
        hash = (hash ^ u64::from(value.to_bits())).wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}
