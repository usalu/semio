//! 📈️ Compensated arithmetic and derivative partitioning for bounded polynomial contacts.
/// 🧭️ Indicates that the bounded polynomial could not resolve its finite distinct roots.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UnresolvedRoot;

const MAX_DEGREE: usize = 7;

/// 🪙️ Evaluation floor for compensated relative roundoff.
pub const ROUNDOFF: f64 = 128.0 * f64::EPSILON * f64::EPSILON;

/// 🧮️ Ascending power coefficients for a polynomial of degree at most seven.
pub type Polynomial = [Wide; MAX_DEGREE + 1];

/// 🔢️ A compensated finite scalar represented as a sum of two floating-point terms.
#[derive(Clone, Copy, Default)]
pub struct Wide {
    high: f64,
    low: f64,
}

impl Wide {
    pub fn scalar(high: f64) -> Self { Self { high, low: 0.0 } }

    pub fn sum(a: f64, b: f64) -> Self {
        let high = a + b;
        let shift = high - a;
        Self { high, low: (a - (high - shift)) + (b - shift) }
    }

    pub fn value(self) -> f64 { self.high + self.low }

    pub fn plus(self, rhs: Self) -> Self {
        let head = Self::sum(self.high, rhs.high);
        Self::sum(head.high, head.low + self.low + rhs.low)
    }

    pub fn times(self, rhs: Self) -> Self {
        let high = self.high * rhs.high;
        let low = self.high.mul_add(rhs.high, -high) + self.high * rhs.low + self.low * rhs.high + self.low * rhs.low;
        Self::sum(high, low)
    }

    pub fn scaled(self, factor: f64) -> Self {
        self.times(Self::scalar(factor))
    }

    pub fn divided(self, divisor: f64) -> Self {
        self.divided_by(Self::scalar(divisor))
    }

    pub fn divided_by(self, divisor: Self) -> Self {
        let first = self.high / divisor.high;
        let remainder = self.plus(divisor.scaled(-first));
        Self::sum(first, remainder.value() / divisor.high)
    }
}

/// 🌱️ An isolated parameter and its stationary-root evidence.
#[derive(Clone, Copy, Default)]
pub struct Root {
    pub parameter: f64,
    pub stationary: bool,
}

/// 📁️ Fixed-capacity isolated roots in local parameter order.
#[derive(Default)]
pub struct Roots {
    values: [Root; MAX_DEGREE],
    count: usize,
}

impl Roots {
    fn push(&mut self, parameter: f64, stationary: bool) -> Result<(), UnresolvedRoot> {
        if self.count != 0 && self.values[self.count - 1].parameter == parameter { return Ok(()); }
        *self.values.get_mut(self.count).ok_or(UnresolvedRoot)? = Root { parameter, stationary };
        self.count += 1;
        Ok(())
    }

    pub fn iter(&self) -> impl Iterator<Item = Root> + '_ {
        self.values[..self.count].iter().copied()
    }
}

/// 📓️ Evaluates a compensated polynomial in a span's local parameter.
pub fn evaluate(polynomial: &Polynomial, degree: usize, parameter: f64) -> Wide {
    polynomial[..=degree].iter().rev().fold(Wide::default(), |value, coefficient| {
        value.scaled(parameter).plus(*coefficient)
    })
}

fn bracket_crosses(a: f64, b: f64) -> bool {
    (a < 0.0 && b > 0.0) || (a > 0.0 && b < 0.0)
}

fn refine(polynomial: &Polynomial, degree: usize, mut lo: f64, mut hi: f64, mut at_lo: f64) -> f64 {
    for _ in 0..96 {
        let middle = (lo + hi) * 0.5;
        if middle == lo || middle == hi { break; }
        let value = evaluate(polynomial, degree, middle).value();
        if value == 0.0 { return middle; }
        if bracket_crosses(at_lo, value) { hi = middle; }
        else { lo = middle; at_lo = value; }
    }
    let at_hi = evaluate(polynomial, degree, hi).value();
    if at_lo.abs() < at_hi.abs() { lo } else { hi }
}

/// 📏️ Sums the magnitudes of the compensated coefficients.
pub fn coefficient_size(polynomial: &Polynomial) -> f64 {
    polynomial.iter().map(|coefficient| coefficient.value().abs()).sum()
}

/// 🎯️ Isolates distinct roots by recursively partitioning at derivative zeros.
pub fn roots(polynomial: &Polynomial) -> Result<Roots, UnresolvedRoot> {
    if !coefficient_size(polynomial).is_finite() { return Err(UnresolvedRoot); }
    let degree = polynomial.iter().rposition(|coefficient| coefficient.value() != 0.0).unwrap_or(0);
    let mut roots_out = Roots::default();
    if degree == 0 { return Ok(roots_out); }
    let value_roundoff = ROUNDOFF * coefficient_size(polynomial);
    let mut derivative = [Wide::default(); MAX_DEGREE + 1];
    for power in 1..=degree {
        derivative[power - 1] = polynomial[power].scaled(power as f64);
    }
    let critical = roots(&derivative)?;
    let mut knots = [1.0; MAX_DEGREE + 1];
    knots[0] = -1.0;
    let mut count = 1;
    for parameter in critical.iter().map(|root| root.parameter).filter(|parameter| *parameter > -1.0 && *parameter < 1.0) {
        knots[count] = parameter;
        count += 1;
    }
    count += 1;
    let values = knots.map(|parameter| {
        let value = evaluate(polynomial, degree, parameter).value();
        if value.abs() <= value_roundoff { 0.0 } else { value }
    });
    for index in 0..count {
        let right_crossing = index + 1 < count && bracket_crosses(values[index], values[index + 1]);
        if values[index] == 0.0 {
            let parameter = knots[index];
            let derivative_roundoff = ROUNDOFF * coefficient_size(&derivative);
            let stationary = degree > 1 && (index > 0 && index + 1 < count
                || evaluate(&derivative, degree - 1, parameter).value().abs() <= derivative_roundoff);
            roots_out.push(parameter, stationary)?;
        }
        if right_crossing {
            roots_out.push(refine(polynomial, degree, knots[index], knots[index + 1], values[index]), false)?;
        }
    }
    if roots_out.count > degree { return Err(UnresolvedRoot); }
    Ok(roots_out)
}

