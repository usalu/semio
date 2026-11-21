//! 🪓️ Axis-aligned box splitting.

use super::*;

impl SolidPrimitive {
    /// 🪓️ Returns the positive and negative box halves in geometric plane order.
    pub fn split_by_axis_plane(self, origin: [f64; 3], normal: [f64; 3]) -> Result<[Self; 2], &'static str> {
        let Self::Box { origin: base, width, depth, height } = self else { return Err("original Split requires a certified box"); };
        if origin.into_iter().chain(normal).any(|coordinate| !coordinate.is_finite()) {
            return Err("box split plane must be finite");
        }
        let axis = normal.iter().position(|&value| value != 0.0).ok_or("box split plane requires a nonzero normal")?;
        if normal[axis + 1..].iter().any(|&value| value != 0.0) { return Err("original Split requires an axis-aligned plane"); }
        let mut upper_base = base.to_array();
        let mut lower_size = [width, depth, height];
        let mut upper_size = lower_size;
        lower_size[axis] = origin[axis] - upper_base[axis];
        upper_size[axis] = (upper_base[axis] + upper_size[axis]) - origin[axis];
        if lower_size[axis].min(upper_size[axis]) <= Tol::DEFAULT.value() {
            return Err("original Split requires a plane strictly inside the box");
        }
        upper_base[axis] = origin[axis];
        let shape = |origin, [width, depth, height]: [f64; 3]| Self::Box { origin, width, depth, height };
        let lower = shape(base, lower_size);
        let upper = shape(Pnt3::from_array(upper_base), upper_size);
        lower.validate()?;
        upper.validate()?;
        Ok(if normal[axis].is_sign_positive() { [upper, lower] } else { [lower, upper] })
    }
}
