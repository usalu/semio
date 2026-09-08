//! 🎲️ Probability distributions and special functions: erf, gamma, beta, and pdf/cdf/quantile/sampling for the standard statistical laws.
//!
//! Moved verbatim from `🧰️framework/🔨️modules/🧮️math/🎲️probability` in ticket 26/08/12/
//! DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS wave M3c: Rust-only compute internals
//! backing `📊️table`'s inferences (no TS twin — this is algorithm, not boundary vocabulary).

// #region 🔖️Error
/// ⚠️ Fallible-construction and fallible-inversion error type shared by every distribution in this crate.
#[derive(Debug)]
pub enum ProbabilityError {
    /// 🚫️ A constructor argument (e.g. `std_dev`, `dof`, `n`) fell outside its valid range.
    InvalidParameter { name: &'static str, value: f64 },
    /// 🚫️ A call-site argument (e.g. a `quantile` probability) fell outside its valid domain.
    OutOfDomain { name: &'static str, value: f64 },
    /// 🚫️ An iterative numerical method (series, continued fraction, Newton/bisection) ran out of iterations.
    NoConvergence { what: &'static str },
}

impl std::fmt::Display for ProbabilityError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidParameter { name, value } => write!(formatter, "invalid parameter {name} = {value}"),
            Self::OutOfDomain { name, value } => write!(formatter, "{name} = {value} outside domain"),
            Self::NoConvergence { what } => write!(formatter, "no convergence in {what}"),
        }
    }
}

impl std::error::Error for ProbabilityError {}
// #endregion 🔖️Error

// #region 🔖️Special
/// 🧮️ Natural log of the gamma function via the Lanczos approximation (g=7, 9-term coefficient
/// set), with the reflection formula `Γ(x)Γ(1-x) = π/sin(πx)` used for `x < 0.5` to keep the
/// Lanczos series in its region of validity. Numerical Recipes §6.1 `gammln`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ln_gamma(x: f64) -> f64 {
    const G: f64 = 7.0;
    const COEFFS: [f64; 9] =
        [0.999_999_999_999_809_9, 676.520_368_121_885_1, -1_259.139_216_722_402_8, 771.323_428_777_653_1, -176.615_029_162_140_6, 12.507_343_278_686_905, -0.138_571_095_265_720_12, 9.984_369_578_019_572e-6, 1.505_632_735_149_311_6e-7];
    if x < 0.5 {
        let pi = std::f64::consts::PI;
        return (pi / (pi * x).sin()).ln() - ln_gamma(1.0 - x);
    }
    let x = x - 1.0;
    let mut a = COEFFS[0];
    let t = x + G + 0.5;
    for (i, &c) in COEFFS.iter().enumerate().skip(1) {
        a += c / (x + i as f64);
    }
    0.5 * (2.0 * std::f64::consts::PI).ln() + (x + 0.5) * t.ln() - t + a.ln()
}

/// 🧮️ Regularized lower incomplete gamma `P(a, x) = γ(a, x) / Γ(a)`: a power series for `x < a+1`,
/// a modified-Lentz continued fraction (via [`gamma_q`]) otherwise. Numerical Recipes §6.2 `gser`/`gcf`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gamma_p(a: f64, x: f64) -> f64 {
    if x < 0.0 || a <= 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return 0.0;
    }
    if x < a + 1.0 {
        gamma_series(a, x)
    } else {
        1.0 - gamma_cf(a, x)
    }
}

/// 🧮️ Regularized upper incomplete gamma `Q(a, x) = 1 - P(a, x)`, computed directly via the
/// continued-fraction branch when `x >= a+1` to avoid the cancellation that `1.0 - gamma_p(a, x)`
/// would suffer in that regime.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn gamma_q(a: f64, x: f64) -> f64 {
    if x < 0.0 || a <= 0.0 {
        return f64::NAN;
    }
    if x == 0.0 {
        return 1.0;
    }
    if x < a + 1.0 {
        1.0 - gamma_series(a, x)
    } else {
        gamma_cf(a, x)
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn gamma_series(a: f64, x: f64) -> f64 {
    const MAX_ITER: usize = 200;
    const EPS: f64 = 1e-15;
    let gln = ln_gamma(a);
    let mut ap = a;
    let mut sum = 1.0 / a;
    let mut del = sum;
    for _ in 0..MAX_ITER {
        ap += 1.0;
        del *= x / ap;
        sum += del;
        if del.abs() < sum.abs() * EPS {
            break;
        }
    }
    sum * (-x + a * x.ln() - gln).exp()
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn gamma_cf(a: f64, x: f64) -> f64 {
    const MAX_ITER: usize = 200;
    const EPS: f64 = 1e-15;
    const TINY: f64 = 1e-300;
    let gln = ln_gamma(a);
    let mut b = x + 1.0 - a;
    let mut c = 1.0 / TINY;
    let mut d = 1.0 / b;
    let mut h = d;
    for i in 1..=MAX_ITER {
        let an = -(i as f64) * (i as f64 - a);
        b += 2.0;
        d = an * d + b;
        if d.abs() < TINY {
            d = TINY;
        }
        c = b + an / c;
        if c.abs() < TINY {
            c = TINY;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < EPS {
            break;
        }
    }
    (-x + a * x.ln() - gln).exp() * h
}

/// 🧮️ Error function via the identity `erf(x) = sign(x) * P(1/2, x²)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn erf(x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    sign * gamma_p(0.5, x * x)
}

/// 🧮️ Complementary error function `1 - erf(x)`, computed via the `gamma_q` continued-fraction
/// branch for `x > 0` to avoid the cancellation `1.0 - erf(x)` would suffer for large `x`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn erfc(x: f64) -> f64 {
    if x == 0.0 {
        return 1.0;
    }
    if x < 0.0 {
        1.0 + gamma_p(0.5, x * x)
    } else {
        gamma_q(0.5, x * x)
    }
}

/// 🧮️ Natural log of the beta function `B(a, b) = Γ(a)Γ(b) / Γ(a+b)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn ln_beta(a: f64, b: f64) -> f64 {
    ln_gamma(a) + ln_gamma(b) - ln_gamma(a + b)
}

/// 🧮️ Regularized incomplete beta `I_x(a, b)` via a modified-Lentz continued fraction, with the
/// symmetry swap `I_x(a,b) = 1 - I_{1-x}(b,a)` applied when `x > (a+1)/(a+b+2)` to keep the
/// fraction in its fast-converging region. Numerical Recipes §6.4 `betai`/`betacf`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn beta_inc(a: f64, b: f64, x: f64) -> f64 {
    if !(0.0..=1.0).contains(&x) {
        return f64::NAN;
    }
    if x == 0.0 || x == 1.0 {
        return x;
    }
    let front = (ln_gamma(a + b) - ln_gamma(a) - ln_gamma(b) + a * x.ln() + b * (1.0 - x).ln()).exp();
    if x < (a + 1.0) / (a + b + 2.0) {
        front * beta_cf(a, b, x) / a
    } else {
        1.0 - front * beta_cf(b, a, 1.0 - x) / b
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn beta_cf(a: f64, b: f64, x: f64) -> f64 {
    const MAX_ITER: usize = 300;
    const EPS: f64 = 1e-15;
    const TINY: f64 = 1e-300;
    let qab = a + b;
    let qap = a + 1.0;
    let qam = a - 1.0;
    let mut c = 1.0;
    let mut d = 1.0 - qab * x / qap;
    if d.abs() < TINY {
        d = TINY;
    }
    d = 1.0 / d;
    let mut h = d;
    for m in 1..=MAX_ITER {
        let mf = m as f64;
        let m2 = 2.0 * mf;
        let aa_even = mf * (b - mf) * x / ((qam + m2) * (a + m2));
        d = 1.0 + aa_even * d;
        if d.abs() < TINY {
            d = TINY;
        }
        c = 1.0 + aa_even / c;
        if c.abs() < TINY {
            c = TINY;
        }
        d = 1.0 / d;
        h *= d * c;
        let aa_odd = -(a + mf) * (qab + mf) * x / ((a + m2) * (qap + m2));
        d = 1.0 + aa_odd * d;
        if d.abs() < TINY {
            d = TINY;
        }
        c = 1.0 + aa_odd / c;
        if c.abs() < TINY {
            c = TINY;
        }
        d = 1.0 / d;
        let del = d * c;
        h *= del;
        if (del - 1.0).abs() < EPS {
            break;
        }
    }
    h
}

/// 🧮️ Standard normal inverse CDF via Acklam's rational approximation (~1.15e-9 relative
/// accuracy), refined by one Halley step using [`erfc`] to push accuracy to ~1e-14. Returns
/// `NaN` for `p` outside `[0, 1]`, and `-inf`/`+inf` exactly at `p == 0.0`/`p == 1.0`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn normal_quantile(p: f64) -> f64 {
    if !(0.0..=1.0).contains(&p) {
        return f64::NAN;
    }
    if p == 0.0 {
        return f64::NEG_INFINITY;
    }
    if p == 1.0 {
        return f64::INFINITY;
    }
    const A: [f64; 6] = [-3.969_683_028_665_376e1, 2.209_460_984_245_205e2, -2.759_285_104_469_687e2, 1.383_577_518_672_69e2, -3.066_479_806_614_716e1, 2.506_628_277_459_239];
    const B: [f64; 5] = [-5.447_609_879_822_406e1, 1.615_858_368_580_409e2, -1.556_989_798_598_866e2, 6.680_131_188_771_972e1, -1.328_068_155_288_572e1];
    const C: [f64; 6] = [-7.784_894_002_430_293e-3, -3.223_964_580_411_365e-1, -2.400_758_277_161_838, -2.549_732_539_343_734, 4.374_664_141_464_968, 2.938_163_982_698_783];
    const D: [f64; 4] = [7.784_695_709_041_462e-3, 3.224_671_290_700_398e-1, 2.445_134_137_142_996, 3.754_408_661_907_416];
    const P_LOW: f64 = 0.024_25;
    let x = if p < P_LOW {
        let q = (-2.0 * p.ln()).sqrt();
        (((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5]) / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    } else if p <= 1.0 - P_LOW {
        let q = p - 0.5;
        let r = q * q;
        (((((A[0] * r + A[1]) * r + A[2]) * r + A[3]) * r + A[4]) * r + A[5]) * q / (((((B[0] * r + B[1]) * r + B[2]) * r + B[3]) * r + B[4]) * r + 1.0)
    } else {
        let q = (-2.0 * (1.0 - p).ln()).sqrt();
        -(((((C[0] * q + C[1]) * q + C[2]) * q + C[3]) * q + C[4]) * q + C[5]) / ((((D[0] * q + D[1]) * q + D[2]) * q + D[3]) * q + 1.0)
    };
    let e = 0.5 * erfc(-x / std::f64::consts::SQRT_2) - p;
    let u = e * (2.0 * std::f64::consts::PI).sqrt() * (x * x / 2.0).exp();
    x - u / (1.0 + x * u / 2.0)
}
// #endregion 🔖️Special

// #region 🔖️Traits
/// 🔔️ A continuous univariate distribution over `f64` support.
pub trait Continuous {
    fn pdf(&self, x: f64) -> f64;
    fn ln_pdf(&self, x: f64) -> f64;
    fn cdf(&self, x: f64) -> f64;
    fn quantile(&self, p: f64) -> Result<f64, ProbabilityError>;
    fn sample(&self, rng: &mut semio_framework_geometry::random::Rng) -> f64;
}

/// 🎯️ A discrete univariate distribution over `u64` support.
pub trait Discrete {
    fn pmf(&self, k: u64) -> f64;
    fn ln_pmf(&self, k: u64) -> f64;
    fn cdf(&self, k: u64) -> f64;
    fn quantile(&self, p: f64) -> Result<u64, ProbabilityError>;
    fn sample(&self, rng: &mut semio_framework_geometry::random::Rng) -> u64;
}
// #endregion 🔖️Traits

// #region 🔖️Invert
/// 🧭️ Inverts a monotone `f` at `target` by Newton's method seeded at `x0`, falling back to
/// bisection on `[lo, hi]` the moment a Newton step would leave the bracket — shared by every
/// non-closed-form `quantile` in this crate (all continuous distributions except [`Normal`]).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn newton_bisect(f: impl Fn(f64) -> f64, target: f64, lo: f64, hi: f64, x0: f64) -> Result<f64, ProbabilityError> {
    const MAX_ITER: usize = 100;
    const EPS: f64 = 1e-12;
    const H: f64 = 1e-6;
    let mut lo = lo;
    let mut hi = hi;
    let mut x = x0.clamp(lo, hi);
    for _ in 0..MAX_ITER {
        let fx = f(x) - target;
        if fx.abs() < EPS {
            return Ok(x);
        }
        if fx < 0.0 {
            lo = x;
        } else {
            hi = x;
        }
        let step = H.max(x.abs() * H);
        let deriv = (f(x + step) - f(x - step)) / (2.0 * step);
        let mut next = if deriv.abs() > 1e-300 { x - fx / deriv } else { f64::NAN };
        if !next.is_finite() || next <= lo || next >= hi {
            next = 0.5 * (lo + hi);
        }
        if (next - x).abs() < EPS * (1.0 + x.abs()) {
            return Ok(next);
        }
        x = next;
    }
    if (hi - lo).abs() < 1e-9 {
        Ok(x)
    } else {
        Err(ProbabilityError::NoConvergence { what: "newton_bisect" })
    }
}

/// 🎲️ Marsaglia–Tsang (2000) gamma-variate sampler: squeeze method for `shape >= 1`, boosted by
/// `U^(1/shape)` for `shape < 1` via the identity `Gamma(shape) = Gamma(shape+1) * U^(1/shape)`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn gamma_sample(shape: f64, rng: &mut semio_framework_geometry::random::Rng) -> f64 {
    if shape < 1.0 {
        let u = rng.next_f64();
        return gamma_sample(shape + 1.0, rng) * u.powf(1.0 / shape);
    }
    let d = shape - 1.0 / 3.0;
    let c = 1.0 / (9.0 * d).sqrt();
    loop {
        let (mut x, mut v);
        loop {
            let u1 = 1.0 - rng.next_f64();
            let u2 = rng.next_f64();
            x = (-2.0 * u1.ln()).sqrt() * (2.0 * std::f64::consts::PI * u2).cos();
            v = 1.0 + c * x;
            if v > 0.0 {
                break;
            }
        }
        v = v * v * v;
        let u = rng.next_f64();
        if u < 1.0 - 0.0331 * x * x * x * x {
            return d * v;
        }
        if u.ln() < 0.5 * x * x + d * (1.0 - v + v.ln()) {
            return d * v;
        }
    }
}
// #endregion 🔖️Invert

// #region 🔖️Continuous

// #region 🔖️Normal
/// 🔔️ Gaussian distribution with mean `mean` and standard deviation `std_dev`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Normal {
    pub mean: f64,
    pub std_dev: f64,
}

impl Normal {
    /// 🔔️ Standard normal, `mean = 0`, `std_dev = 1`.
    pub const STANDARD: Self = Self { mean: 0.0, std_dev: 1.0 };

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(mean: f64, std_dev: f64) -> Result<Self, ProbabilityError> {
        if std_dev.is_nan() || std_dev <= 0.0 {
            return Err(ProbabilityError::InvalidParameter { name: "std_dev", value: std_dev });
        }
        Ok(Self { mean, std_dev })
    }
}

impl Continuous for Normal {
    fn pdf(&self, x: f64) -> f64 {
        self.ln_pdf(x).exp()
    }

    fn ln_pdf(&self, x: f64) -> f64 {
        let z = (x - self.mean) / self.std_dev;
        -0.5 * z * z - self.std_dev.ln() - 0.5 * (2.0 * std::f64::consts::PI).ln()
    }

    fn cdf(&self, x: f64) -> f64 {
        0.5 * erfc(-(x - self.mean) / (self.std_dev * std::f64::consts::SQRT_2))
    }

    fn quantile(&self, p: f64) -> Result<f64, ProbabilityError> {
        if !(0.0..=1.0).contains(&p) {
            return Err(ProbabilityError::OutOfDomain { name: "p", value: p });
        }
        Ok(self.mean + self.std_dev * normal_quantile(p))
    }

    /// 🎲️ Marsaglia polar method: rejects points outside the unit disk, transforms the accepted
    /// point, and discards the second free deviate the transform produces (an `&self` method
    /// can't cache it across calls).
    fn sample(&self, rng: &mut semio_framework_geometry::random::Rng) -> f64 {
        loop {
            let u = 2.0 * rng.next_f64() - 1.0;
            let v = 2.0 * rng.next_f64() - 1.0;
            let s = u * u + v * v;
            if s > 0.0 && s < 1.0 {
                let mul = (-2.0 * s.ln() / s).sqrt();
                return self.mean + self.std_dev * u * mul;
            }
        }
    }
}
// #endregion 🔖️Normal

// #region 🔖️Uniform
/// 📏️ Continuous uniform distribution on `[low, high]`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Uniform {
    pub low: f64,
    pub high: f64,
}

impl Uniform {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(low: f64, high: f64) -> Result<Self, ProbabilityError> {
        if low.is_nan() || high.is_nan() || low >= high {
            return Err(ProbabilityError::InvalidParameter { name: "high", value: high });
        }
        Ok(Self { low, high })
    }
}

impl Continuous for Uniform {
    fn pdf(&self, x: f64) -> f64 {
        if x < self.low || x > self.high {
            0.0
        } else {
            1.0 / (self.high - self.low)
        }
    }

    fn ln_pdf(&self, x: f64) -> f64 {
        self.pdf(x).ln()
    }

    fn cdf(&self, x: f64) -> f64 {
        if x < self.low {
            0.0
        } else if x > self.high {
            1.0
        } else {
            (x - self.low) / (self.high - self.low)
        }
    }

    fn quantile(&self, p: f64) -> Result<f64, ProbabilityError> {
        if !(0.0..=1.0).contains(&p) {
            return Err(ProbabilityError::OutOfDomain { name: "p", value: p });
        }
        Ok(self.low + p * (self.high - self.low))
    }

    fn sample(&self, rng: &mut semio_framework_geometry::random::Rng) -> f64 {
        self.low + rng.next_f64() * (self.high - self.low)
    }
}
// #endregion 🔖️Uniform

// #region 🔖️ChiSquared
/// 📐️ Chi-squared distribution with `dof` degrees of freedom.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ChiSquared {
    pub dof: f64,
}

impl ChiSquared {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(dof: f64) -> Result<Self, ProbabilityError> {
        if dof.is_nan() || dof <= 0.0 {
            return Err(ProbabilityError::InvalidParameter { name: "dof", value: dof });
        }
        Ok(Self { dof })
    }
}

impl Continuous for ChiSquared {
    fn pdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            return 0.0;
        }
        self.ln_pdf(x).exp()
    }

    fn ln_pdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            return f64::NEG_INFINITY;
        }
        let k = self.dof / 2.0;
        (k - 1.0) * x.ln() - x / 2.0 - k * 2f64.ln() - ln_gamma(k)
    }

    fn cdf(&self, x: f64) -> f64 {
        if x < 0.0 {
            return 0.0;
        }
        gamma_p(self.dof / 2.0, x / 2.0)
    }

    /// 📐️ Newton's method seeded with the Wilson–Hilferty cube-root approximation, safeguarded
    /// by bisection on `[0, upper]` where `upper` doubles until `cdf(upper) > p`.
    fn quantile(&self, p: f64) -> Result<f64, ProbabilityError> {
        if !(0.0..=1.0).contains(&p) {
            return Err(ProbabilityError::OutOfDomain { name: "p", value: p });
        }
        if p == 0.0 {
            return Ok(0.0);
        }
        if p == 1.0 {
            return Ok(f64::INFINITY);
        }
        let k = self.dof;
        let term = 2.0 / (9.0 * k);
        let seed = k * (1.0 - term + normal_quantile(p) * term.sqrt()).powi(3);
        let seed = if seed.is_finite() && seed > 0.0 { seed } else { k };
        let mut upper = seed.max(1.0);
        while self.cdf(upper) < p {
            upper *= 2.0;
        }
        newton_bisect(|x| self.cdf(x), p, 0.0, upper, seed)
    }

    /// 🎲️ `2 * gamma_sample(dof/2)`.
    fn sample(&self, rng: &mut semio_framework_geometry::random::Rng) -> f64 {
        2.0 * gamma_sample(self.dof / 2.0, rng)
    }
}
// #endregion 🔖️ChiSquared

// #region 🔖️StudentT
/// 📐️ Student's t-distribution with `dof` degrees of freedom.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct StudentT {
    pub dof: f64,
}

impl StudentT {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(dof: f64) -> Result<Self, ProbabilityError> {
        if dof.is_nan() || dof <= 0.0 {
            return Err(ProbabilityError::InvalidParameter { name: "dof", value: dof });
        }
        Ok(Self { dof })
    }
}

impl Continuous for StudentT {
    fn pdf(&self, x: f64) -> f64 {
        self.ln_pdf(x).exp()
    }

    fn ln_pdf(&self, x: f64) -> f64 {
        let v = self.dof;
        ln_gamma((v + 1.0) / 2.0) - ln_gamma(v / 2.0) - 0.5 * (v * std::f64::consts::PI).ln() - (v + 1.0) / 2.0 * (1.0 + x * x / v).ln()
    }

    fn cdf(&self, x: f64) -> f64 {
        let v = self.dof;
        let ib = beta_inc(v / 2.0, 0.5, v / (v + x * x));
        if x >= 0.0 {
            1.0 - 0.5 * ib
        } else {
            0.5 * ib
        }
    }

    fn quantile(&self, p: f64) -> Result<f64, ProbabilityError> {
        if !(0.0..=1.0).contains(&p) {
            return Err(ProbabilityError::OutOfDomain { name: "p", value: p });
        }
        if p == 0.0 {
            return Ok(f64::NEG_INFINITY);
        }
        if p == 1.0 {
            return Ok(f64::INFINITY);
        }
        let seed = normal_quantile(p);
        let mut bound = seed.abs().max(1.0) * 4.0 + 10.0;
        while self.cdf(bound) < p {
            bound *= 2.0;
        }
        newton_bisect(|x| self.cdf(x), p, -bound, bound, seed)
    }

    /// 🎲️ `normal_sample / sqrt(chi2_sample(dof) / dof)`.
    fn sample(&self, rng: &mut semio_framework_geometry::random::Rng) -> f64 {
        let z = Normal::STANDARD.sample(rng);
        let chi2 = 2.0 * gamma_sample(self.dof / 2.0, rng);
        z / (chi2 / self.dof).sqrt()
    }
}
// #endregion 🔖️StudentT

// #region 🔖️FisherF
/// 📐️ Fisher–Snedecor F-distribution with `dof1`, `dof2` degrees of freedom.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FisherF {
    pub dof1: f64,
    pub dof2: f64,
}

impl FisherF {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(dof1: f64, dof2: f64) -> Result<Self, ProbabilityError> {
        if dof1.is_nan() || dof1 <= 0.0 {
            return Err(ProbabilityError::InvalidParameter { name: "dof1", value: dof1 });
        }
        if dof2.is_nan() || dof2 <= 0.0 {
            return Err(ProbabilityError::InvalidParameter { name: "dof2", value: dof2 });
        }
        Ok(Self { dof1, dof2 })
    }
}

impl Continuous for FisherF {
    fn pdf(&self, x: f64) -> f64 {
        if x <= 0.0 {
            return 0.0;
        }
        self.ln_pdf(x).exp()
    }

    fn ln_pdf(&self, x: f64) -> f64 {
        if x <= 0.0 {
            return f64::NEG_INFINITY;
        }
        let (d1, d2) = (self.dof1, self.dof2);
        0.5 * d1 * d1.ln() + 0.5 * d2 * d2.ln() + (0.5 * d1 - 1.0) * x.ln() - 0.5 * (d1 + d2) * (d2 + d1 * x).ln() - ln_beta(0.5 * d1, 0.5 * d2)
    }

    fn cdf(&self, x: f64) -> f64 {
        if x <= 0.0 {
            return 0.0;
        }
        beta_inc(self.dof1 / 2.0, self.dof2 / 2.0, self.dof1 * x / (self.dof1 * x + self.dof2))
    }

    fn quantile(&self, p: f64) -> Result<f64, ProbabilityError> {
        if !(0.0..=1.0).contains(&p) {
            return Err(ProbabilityError::OutOfDomain { name: "p", value: p });
        }
        if p == 0.0 {
            return Ok(0.0);
        }
        if p == 1.0 {
            return Ok(f64::INFINITY);
        }
        let mut upper = 1.0;
        while self.cdf(upper) < p {
            upper *= 2.0;
        }
        newton_bisect(|x| self.cdf(x), p, 0.0, upper, upper / 2.0)
    }

    /// 🎲️ `(chi2_sample(dof1)/dof1) / (chi2_sample(dof2)/dof2)`.
    fn sample(&self, rng: &mut semio_framework_geometry::random::Rng) -> f64 {
        let c1 = 2.0 * gamma_sample(self.dof1 / 2.0, rng);
        let c2 = 2.0 * gamma_sample(self.dof2 / 2.0, rng);
        (c1 / self.dof1) / (c2 / self.dof2)
    }
}
// #endregion 🔖️FisherF

// #endregion 🔖️Continuous

// #region 🔖️Discrete

// #region 🔖️Bernoulli
/// 🪙️ Bernoulli distribution: `1` with probability `p`, `0` otherwise.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bernoulli {
    pub p: f64,
}

impl Bernoulli {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(p: f64) -> Result<Self, ProbabilityError> {
        if !(0.0..=1.0).contains(&p) {
            return Err(ProbabilityError::InvalidParameter { name: "p", value: p });
        }
        Ok(Self { p })
    }
}

impl Discrete for Bernoulli {
    fn pmf(&self, k: u64) -> f64 {
        match k {
            0 => 1.0 - self.p,
            1 => self.p,
            _ => 0.0,
        }
    }

    fn ln_pmf(&self, k: u64) -> f64 {
        self.pmf(k).ln()
    }

    fn cdf(&self, k: u64) -> f64 {
        match k {
            0 => 1.0 - self.p,
            _ => 1.0,
        }
    }

    fn quantile(&self, p: f64) -> Result<u64, ProbabilityError> {
        if !(0.0..=1.0).contains(&p) {
            return Err(ProbabilityError::OutOfDomain { name: "p", value: p });
        }
        Ok(if p <= 1.0 - self.p { 0 } else { 1 })
    }

    fn sample(&self, rng: &mut semio_framework_geometry::random::Rng) -> u64 {
        u64::from(rng.next_bool(self.p))
    }
}
// #endregion 🔖️Bernoulli

// #region 🔖️Binomial
/// 🎯️ Binomial distribution: number of successes in `n` independent Bernoulli(`p`) trials.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Binomial {
    pub n: u64,
    pub p: f64,
}

impl Binomial {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(n: u64, p: f64) -> Result<Self, ProbabilityError> {
        if !(0.0..=1.0).contains(&p) {
            return Err(ProbabilityError::InvalidParameter { name: "p", value: p });
        }
        Ok(Self { n, p })
    }
}

impl Discrete for Binomial {
    fn pmf(&self, k: u64) -> f64 {
        if k > self.n {
            return 0.0;
        }
        self.ln_pmf(k).exp()
    }

    fn ln_pmf(&self, k: u64) -> f64 {
        if k > self.n {
            return f64::NEG_INFINITY;
        }
        let (n, k) = (self.n as f64, k as f64);
        let ln_coeff = ln_gamma(n + 1.0) - ln_gamma(k + 1.0) - ln_gamma(n - k + 1.0);
        let log_p = if self.p > 0.0 {
            k * self.p.ln()
        } else if k == 0.0 {
            0.0
        } else {
            f64::NEG_INFINITY
        };
        let log_q = if self.p < 1.0 {
            (n - k) * (1.0 - self.p).ln()
        } else if k == n {
            0.0
        } else {
            f64::NEG_INFINITY
        };
        ln_coeff + log_p + log_q
    }

    /// 🎯️ Exact O(1) CDF via the regularized-beta/binomial-CDF identity
    /// `P(X <= k) = I_{1-p}(n-k, k+1)`.
    fn cdf(&self, k: u64) -> f64 {
        if k >= self.n {
            return 1.0;
        }
        beta_inc(self.n as f64 - k as f64, k as f64 + 1.0, 1.0 - self.p)
    }

    /// 🎯️ Walks the CDF from `0` upward — O(n), acceptable since `n` is small in the intended
    /// causal-discovery use case.
    fn quantile(&self, p: f64) -> Result<u64, ProbabilityError> {
        if !(0.0..=1.0).contains(&p) {
            return Err(ProbabilityError::OutOfDomain { name: "p", value: p });
        }
        for k in 0..=self.n {
            if self.cdf(k) >= p {
                return Ok(k);
            }
        }
        Ok(self.n)
    }

    /// 🎯️ `n` independent Bernoulli draws — O(n), no BTPE, not needed at this scale.
    fn sample(&self, rng: &mut semio_framework_geometry::random::Rng) -> u64 {
        (0..self.n).filter(|_| rng.next_bool(self.p)).count() as u64
    }
}
// #endregion 🔖️Binomial

// #region 🔖️Multinomial
/// 🎲️ Multinomial distribution: `n` draws over `probs.len()` categories. Multivariate, so it
/// implements only inherent methods rather than [`Discrete`].
#[derive(Clone, Debug, PartialEq)]
pub struct Multinomial {
    pub n: u64,
    pub probs: Vec<f64>,
}

impl Multinomial {
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn new(n: u64, probs: Vec<f64>) -> Result<Self, ProbabilityError> {
        let sum: f64 = probs.iter().sum();
        if (sum - 1.0).abs() > 1e-9 {
            return Err(ProbabilityError::InvalidParameter { name: "probs_sum", value: sum });
        }
        if probs.iter().any(|&p| p < 0.0) {
            return Err(ProbabilityError::InvalidParameter { name: "probs", value: f64::NAN });
        }
        Ok(Self { n, probs })
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn ln_pmf(&self, counts: &[u64]) -> f64 {
        if counts.len() != self.probs.len() || counts.iter().sum::<u64>() != self.n {
            return f64::NEG_INFINITY;
        }
        let mut result = ln_gamma(self.n as f64 + 1.0);
        for (&count, &p) in counts.iter().zip(self.probs.iter()) {
            result -= ln_gamma(count as f64 + 1.0);
            if count > 0 {
                result += count as f64 * p.ln();
            }
        }
        result
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn pmf(&self, counts: &[u64]) -> f64 {
        self.ln_pmf(counts).exp()
    }

    /// 🎲️ Sequential conditional binomial draws: category `i`'s count is `Binomial(remaining,
    /// p_i / (1 - sum of earlier p's))`, decrementing `remaining` after each draw.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn sample(&self, rng: &mut semio_framework_geometry::random::Rng) -> Vec<u64> {
        let mut remaining = self.n;
        let mut remaining_prob = 1.0;
        let mut counts = Vec::with_capacity(self.probs.len());
        for (i, &p) in self.probs.iter().enumerate() {
            if i == self.probs.len() - 1 {
                counts.push(remaining);
                break;
            }
            let conditional_p = if remaining_prob > 0.0 { (p / remaining_prob).clamp(0.0, 1.0) } else { 0.0 };
            let draw = Binomial { n: remaining, p: conditional_p }.sample(rng);
            counts.push(draw);
            remaining -= draw;
            remaining_prob -= p;
        }
        counts
    }
}
// #endregion 🔖️Multinomial

// #endregion 🔖️Discrete

// #region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
// #endregion 🔖️Tests
