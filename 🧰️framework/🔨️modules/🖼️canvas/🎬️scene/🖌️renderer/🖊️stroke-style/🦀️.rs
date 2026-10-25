//! 🖊️ Stroke style of the scene: the cap, the join and the stroke record with its peniko conversions. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cap {
    Butt,
    Round,
    Square,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl Cap {
    pub(super) fn to_kurbo(self) -> kurbo::Cap {
        match self {
            Cap::Butt => kurbo::Cap::Butt,
            Cap::Round => kurbo::Cap::Round,
            Cap::Square => kurbo::Cap::Square,
        }
    }
}

/// 🔗️ First-party stroke join style — mirrors `kurbo::Join`. No guest call site sets it
/// explicitly today (every `Stroke` is built via `new`, matching `kurbo::Stroke::new`'s own
/// `Join::Round` default), but the field is real so `Stroke::to_kurbo` (host-only, below) can
/// rebuild a real `kurbo::Stroke` losslessly rather than hardcoding a join.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Join {
    Round,
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl Join {
    pub(super) fn to_kurbo(self) -> kurbo::Join {
        match self {
            Join::Round => kurbo::Join::Round,
        }
    }
}

/// 🖌️ First-party stroke style — plain value fields, no `kurbo::Stroke` wrapper, so
/// `wasm32-wasip2` never needs `kurbo` to build/measure/dispose one. `to_kurbo` (host-only,
/// below) rebuilds a real `kurbo::Stroke` only at rasterization time, matching
/// `kurbo::Stroke::new`'s own field defaults exactly.
#[derive(Clone, Debug, PartialEq)]
pub struct Stroke {
    pub(super) width: f64,
    pub(super) join: Join,
    pub(super) miter_limit: f64,
    pub(super) start_cap: Cap,
    pub(super) end_cap: Cap,
    pub(super) dash_pattern: Vec<f64>,
    pub(super) dash_offset: f64,
}

impl Stroke {
    pub fn new(width: f64) -> Self {
        Self { width, join: Join::Round, miter_limit: 4.0, start_cap: Cap::Round, end_cap: Cap::Round, dash_pattern: Vec::new(), dash_offset: 0.0 }
    }
    pub fn set_dash_pattern(&mut self, pattern: Vec<f64>) {
        self.dash_pattern = pattern;
    }
    /// 👁️ Borrows the original dash sequence without allocating or transferring it.
    pub fn dash_pattern(&self) -> &[f64] { &self.dash_pattern }
    /// 📦️ Observes the original dash allocation extent before custody transfers.
    pub fn dash_capacity(&self) -> usize { self.dash_pattern.capacity() }
    /// 🎁️ Transfers only the original dash buffer while retaining the scalar style.
    pub(crate) fn take_dash_pattern(&mut self) -> Vec<f64> { std::mem::take(&mut self.dash_pattern) }
    pub fn set_start_cap(&mut self, cap: Cap) {
        self.start_cap = cap;
    }
    pub fn set_end_cap(&mut self, cap: Cap) {
        self.end_cap = cap;
    }
    pub(super) fn retirement_step(&mut self) -> bool {
        self.dash_pattern.pop().is_none()
    }
    pub(super) fn retirement_backing_bytes(&self) -> usize {
        self.dash_pattern.capacity().saturating_mul(size_of::<f64>())
    }
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
impl Stroke {
    /// 🖥️ Host/browser-only: rebuilds a real `kurbo::Stroke`, the only place this crate needs
    /// one, at the same `SceneCommand::replay_into` boundary `vello` itself is reached from.
    pub(crate) fn to_kurbo(&self) -> kurbo::Stroke {
        kurbo::Stroke {
            width: self.width,
            join: self.join.to_kurbo(),
            miter_limit: self.miter_limit,
            start_cap: self.start_cap.to_kurbo(),
            end_cap: self.end_cap.to_kurbo(),
            dash_pattern: self.dash_pattern.iter().copied().collect(),
            dash_offset: self.dash_offset,
        }
    }
}
