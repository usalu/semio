//! 🥚️ Global nearest parameter of an authored elliptic interval.
use super::{invalid, ValueError, ValueRefusalKind, PI, TAU};
use semio_framework_3d_brep_curve::bounded_polynomial::{roots, Polynomial, Wide, ROUNDOFF};

/// 🧲️ Compares both trim endpoints and every stationary point of the elliptic distance.
pub(super) fn nearest(x: f64, y: f64, a: f64, b: f64, range: (f64, f64)) -> Result<f64, ValueError> {
    if ![x, y, a, b, range.0, range.1].into_iter().all(f64::is_finite) || a <= 0.0 || b <= 0.0 {
        return Err(invalid());
    }
    let (lo, hi) = (range.0.min(range.1), range.0.max(range.1));
    if lo == hi || (a == b && x == 0.0 && y == 0.0) { return Ok(lo); }
    let scale = a.max(b).max(x.abs()).max(y.abs());
    let [a, b, x, y] = [a, b, x, y].map(|value| Wide::scalar(value).divided(scale));
    if a.value() == 0.0 || b.value() == 0.0 { return Err(unresolved()); }
    let square = |value: Wide| value.times(value);
    let score = |cosine: Wide, sine: Wide| square(a.times(cosine).plus(x.scaled(-1.0)))
        .plus(square(b.times(sine).plus(y.scaled(-1.0))));
    let (sin_lo, cos_lo) = lo.sin_cos();
    let origin = sin_lo.atan2(cos_lo);
    let mut best = (lo, score(Wide::scalar(cos_lo), Wide::scalar(sin_lo)), None);
    let mut consider = |t: f64, cosine: Wide, sine: Wide, verify| {
        let value = score(cosine, sine);
        let difference = value.plus(best.1.scaled(-1.0)).value();
        let uncertainty = ROUNDOFF * value.value().max(best.1.value());
        if difference < -uncertainty || (difference.abs() <= uncertainty && t < best.0) {
            best = (t, value, verify);
        }
    };
    let (sin_hi, cos_hi) = hi.sin_cos();
    consider(hi, Wide::scalar(cos_hi), Wide::scalar(sin_hi), None);
    let difference = square(b).plus(square(a).scaled(-1.0));
    for (sign, phase) in [(1.0, 0.0), (-1.0, PI)] {
        let along_x = a.times(x).scaled(sign);
        let along_y = b.times(y).scaled(sign);
        let mut equation = Polynomial::default();
        equation[0] = along_y.scaled(-1.0);
        equation[1] = difference.plus(along_x).scaled(2.0);
        equation[3] = along_x.plus(difference.scaled(-1.0)).scaled(2.0);
        equation[4] = along_y;
        for root in roots(&equation).map_err(|_| unresolved())?.iter() {
            let u = Wide::scalar(root.parameter);
            let u2 = square(u);
            let denominator = Wide::scalar(1.0).plus(u2);
            let cosine = Wide::scalar(1.0).plus(u2.scaled(-1.0)).scaled(sign).divided_by(denominator);
            let sine = u.scaled(2.0 * sign).divided_by(denominator);
            let angle = phase + 2.0 * root.parameter.atan();
            let parameter = lo + (angle - origin).rem_euclid(TAU);
            if parameter >= lo && parameter <= hi {
                consider(parameter, cosine, sine, Some((cosine.value(), sine.value())));
            }
        }
    }
    if let Some((cosine, sine)) = best.2 {
        let (actual_sine, actual_cosine) = best.0.sin_cos();
        if (cosine - actual_cosine).hypot(sine - actual_sine) > 1e-10 { return Err(unresolved()); }
    }
    Ok(best.0)
}

fn unresolved() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "elliptical trim projection exceeded finite parameter precision")
}
