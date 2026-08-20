//! 🎚️ The kernel's tolerance model: a fixed global [`Resolution`], per-entity [`Tol`] values with
//! a containment ordering (vertex ≥ its edges ≥ their faces), and a certified interval type [`Iv`]
//! used by [`crate::artifacts::semio::standards::v1::subsets::brep::schema::snapshot::vector::predicates`] to decide when a fast `f64` computation is trustworthy versus
//! when it must escalate to exact arithmetic. Geometric decision code should never compare raw
//! `f64`s with `==`/`<` — it should go through a `Tol` or an `Iv`.
//!
//! Moved from `🧰️framework/🔨️modules/🧊️3d/📐️brep/📏️tolerance` in ticket
//! 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave PEEL4.

// #region 🔖️Resolution

/// 🎚️ Kernel-wide default resolutions, used to seed new tolerances before any entity-specific
/// tightening/loosening happens.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Resolution {
    /// 🎚️ Default linear tolerance (model units).
    pub linear: f64,
    /// 🎚️ Default angular tolerance (radians).
    pub angular: f64,
    /// 🎚️ Default curve/surface parametric tolerance.
    pub param: f64,
}

impl Resolution {
    pub const DEFAULT: Resolution = Resolution { linear: 1e-7, angular: 1e-9, param: 1e-9 };
}

impl Default for Resolution {
    fn default() -> Self {
        Resolution::DEFAULT
    }
}

// #endregion 🔖️Resolution

// #region 🔖️Tol

/// 🎚️ A single linear tolerance value attached to one entity (vertex containment ball radius,
/// edge tube radius, or face shell thickness).
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, serde::Serialize, serde::Deserialize)]
pub struct Tol(pub f64);

impl Tol {
    pub const DEFAULT: Tol = Tol(Resolution::DEFAULT.linear);

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(value: f64) -> Self {
        debug_assert!(value.is_finite(), "tolerance must be finite");
        Tol(value.max(0.0))
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn value(self) -> f64 {
        self.0
    }
    /// 🎚️ True when `distance` is within this tolerance of zero.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn contains(self, distance: f64) -> bool {
        distance.abs() <= self.0
    }
    /// 🎚️ The tighter (smaller) of two tolerances — used when an operation must satisfy both
    /// operands' requirements simultaneously.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn tighter(self, o: Tol) -> Tol {
        Tol(self.0.min(o.0))
    }
    /// 🎚️ The looser (larger) of two tolerances — used when propagating tolerance up the
    /// containment hierarchy (an edge's tolerance must cover every incident vertex tolerance).
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn looser(self, o: Tol) -> Tol {
        Tol(self.0.max(o.0))
    }
    /// 🎚️ Scales the tolerance, clamping to zero rather than going negative on a negative factor.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn scaled(self, factor: f64) -> Tol {
        Tol((self.0 * factor).max(0.0))
    }
}

impl Default for Tol {
    fn default() -> Self {
        Tol::DEFAULT
    }
}

/// 🎚️ Checks the tolerance-containment invariant: every entry in `finer` must be ≤ the
/// corresponding bound in `coarser` (e.g. a vertex's tolerance must be ≥ zero and every incident
/// edge's tolerance ≥ the vertex's, every incident face's ≥ the edge's). Returns the first
/// violating pair, if any.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn check_containment(finer_label: &str, finer: Tol, coarser_label: &str, coarser: Tol) -> Option<(String, String)> {
    if finer.value() > coarser.value() {
        Some((finer_label.to_string(), coarser_label.to_string()))
    } else {
        None
    }
}

// #endregion 🔖️Tol

// #region 🔖️Interval

/// 🎚️ A certified closed interval `[lo, hi]` used for filtered (interval-arithmetic) evaluation.
/// Arithmetic ops widen conservatively so that the true real-valued result is always contained in
/// the returned interval — the caller escalates to exact arithmetic exactly when `contains_zero()`
/// leaves ambiguity that matters (a sign test straddling zero).
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Iv {
    pub lo: f64,
    pub hi: f64,
}

impl Iv {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn exact(v: f64) -> Self {
        Iv { lo: v, hi: v }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(lo: f64, hi: f64) -> Self {
        debug_assert!(lo <= hi);
        Iv { lo, hi }
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn contains_zero(self) -> bool {
        self.lo <= 0.0 && self.hi >= 0.0
    }
    /// 🎚️ `Some(true)`/`Some(false)` when the sign is certain, `None` when the interval straddles
    /// zero and the caller must escalate to an exact recomputation.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sign(self) -> Option<std::cmp::Ordering> {
        if self.hi < 0.0 {
            Some(std::cmp::Ordering::Less)
        } else if self.lo > 0.0 {
            Some(std::cmp::Ordering::Greater)
        } else if self.lo == 0.0 && self.hi == 0.0 {
            Some(std::cmp::Ordering::Equal)
        } else {
            None
        }
    }
    #[allow(clippy::should_implement_trait)]
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn add(self, o: Iv) -> Iv {
        Iv::new(self.lo + o.lo, self.hi + o.hi)
    }
    #[allow(clippy::should_implement_trait)]
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sub(self, o: Iv) -> Iv {
        Iv::new(self.lo - o.hi, self.hi - o.lo)
    }
    #[allow(clippy::should_implement_trait)]
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn mul(self, o: Iv) -> Iv {
        let candidates = [self.lo * o.lo, self.lo * o.hi, self.hi * o.lo, self.hi * o.hi];
        Iv::new(candidates.iter().copied().fold(f64::INFINITY, f64::min), candidates.iter().copied().fold(f64::NEG_INFINITY, f64::max))
    }
    #[allow(clippy::should_implement_trait)]
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn neg(self) -> Iv {
        Iv::new(-self.hi, -self.lo)
    }
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn widen(self, epsilon: f64) -> Iv {
        Iv::new(self.lo - epsilon, self.hi + epsilon)
    }
}

// #endregion 🔖️Interval

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn tol_contains_checks_absolute_distance() {
        let t = Tol::new(0.01);
        assert!(t.contains(0.005));
        assert!(t.contains(-0.005));
        assert!(!t.contains(0.02));
    }

    #[semio_framework_async_macros::async_test]
    async fn tol_tighter_and_looser_pick_correctly() {
        let a = Tol::new(0.1);
        let b = Tol::new(0.5);
        assert_eq!(a.tighter(b), a);
        assert_eq!(a.looser(b), b);
    }

    #[semio_framework_async_macros::async_test]
    async fn negative_tolerance_clamps_to_zero() {
        assert_eq!(Tol::new(-1.0), Tol::new(0.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn check_containment_flags_violation() {
        // A vertex whose own tolerance ball (0.1) is larger than its incident edge's tube (0.01)
        // violates the containment hierarchy: the finer (vertex) must fit inside the coarser (edge).
        let vertex_tol = Tol::new(0.1);
        let edge_tol = Tol::new(0.01);
        let violation = check_containment("vertex-1", vertex_tol, "edge-1", edge_tol);
        assert!(violation.is_some());
        let ok = check_containment("vertex-1", vertex_tol, "edge-1", edge_tol.looser(vertex_tol));
        assert!(ok.is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn interval_add_widens_conservatively() {
        let a = Iv::new(1.0, 2.0);
        let b = Iv::new(-1.0, 3.0);
        let sum = a.add(b);
        assert_eq!(sum, Iv::new(0.0, 5.0));
    }

    #[semio_framework_async_macros::async_test]
    async fn interval_sign_is_none_when_straddling_zero() {
        let iv = Iv::new(-0.001, 0.001);
        assert_eq!(iv.sign(), None);
    }

    #[semio_framework_async_macros::async_test]
    async fn interval_sign_certain_when_strictly_positive_or_negative() {
        assert_eq!(Iv::new(0.5, 1.0).sign(), Some(std::cmp::Ordering::Greater));
        assert_eq!(Iv::new(-1.0, -0.5).sign(), Some(std::cmp::Ordering::Less));
    }

    #[semio_framework_async_macros::async_test]
    async fn interval_mul_contains_true_product_for_mixed_signs() {
        let a = Iv::new(-2.0, 3.0);
        let b = Iv::new(-1.0, 4.0);
        let product = a.mul(b);
        assert!(product.lo <= -8.0 && product.hi >= 12.0);
    }

    mod quick {
        use super::*;

        #[semio_framework_async_macros::async_test]
        async fn interval_arithmetic_always_contains_scalar_result() {
            let mut rng = semio_framework_geometry::random::Rng::from_seed(3);
            for _ in 0..500 {
                let a = rng.next_f64() * 20.0 - 10.0;
                let b = rng.next_f64() * 20.0 - 10.0;
                let ia = Iv::exact(a);
                let ib = Iv::exact(b);
                let sum = ia.add(ib);
                assert!(sum.lo <= a + b && sum.hi >= a + b);
                let prod = ia.mul(ib);
                assert!(prod.lo <= a * b + 1e-12 && prod.hi >= a * b - 1e-12);
            }
        }
    }
}
// #endregion 🔖️Tests
