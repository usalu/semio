//! 📐️ Shared translate/rotate/scale gesture of the stdio geometry editors.
//!
//! One `SetValue` at [`TRANSFORM_PATH`] carries `{ translate, rotate, scale, pivot }` (all optional, vectors of three numbers,
//! `rotate` in degrees applied as Euler X then Y then Z like three.js `Euler("XYZ")`). Every editor answers the gesture with its own
//! concrete kinds, so history, undo and persistence stay the ordinary mutation path. Cross-checked against three.js in
//! `🧪️tests/🧫️fixtures/📐️transform`.

use super::edit_fault;
use super::SnapshotEditEvent;
use semio_framework_plugin::Fault;
use semio_framework_value::DslValue;

pub const TRANSFORM_PATH: &str = "/transform";

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub translate: [f64; 3],
    pub rotate_degrees: [f64; 3],
    pub scale: [f64; 3],
    pub pivot: [f64; 3],
}

impl Default for Transform {
    fn default() -> Self {
        Self { translate: [0.0; 3], rotate_degrees: [0.0; 3], scale: [1.0; 3], pivot: [0.0; 3] }
    }
}

fn vector(value: &DslValue, key: &str, default: [f64; 3]) -> Result<[f64; 3], Fault> {
    let Some(found) = value.get(key) else { return Ok(default) };
    let items = found.as_array().filter(|items| items.len() == 3).ok_or_else(|| edit_fault("snapshot-edit.transform-invalid", format!("transform.{key} must be three numbers")))?;
    let mut out = [0.0; 3];
    for (slot, item) in out.iter_mut().zip(items) {
        *slot = item.as_f64().filter(|number| number.is_finite()).ok_or_else(|| edit_fault("snapshot-edit.transform-invalid", format!("transform.{key} must be three finite numbers")))?;
    }
    Ok(out)
}

impl Transform {
    /// 🧭️ The transform named by `event`, `None` when the event is not the transform gesture.
    pub fn from_event(event: &SnapshotEditEvent) -> Option<Result<Self, Fault>> {
        let SnapshotEditEvent::SetValue { path, value } = event else { return None };
        (path == TRANSFORM_PATH).then(|| Self::from_value(value))
    }

    pub fn from_value(value: &DslValue) -> Result<Self, Fault> {
        value.as_object().ok_or_else(|| edit_fault("snapshot-edit.transform-invalid", "transform must be an object"))?;
        let defaults = Self::default();
        let scale = vector(value, "scale", defaults.scale)?;
        if scale.iter().any(|factor| *factor == 0.0) {
            return Err(edit_fault("snapshot-edit.transform-invalid", "transform.scale must not contain zero"));
        }
        Ok(Self { translate: vector(value, "translate", defaults.translate)?, rotate_degrees: vector(value, "rotate", defaults.rotate_degrees)?, scale, pivot: vector(value, "pivot", defaults.pivot)? })
    }

    pub fn is_identity(&self) -> bool {
        self.translate == [0.0; 3] && self.rotate_degrees == [0.0; 3] && self.scale == [1.0; 3]
    }

    /// 🔄️ Row-major rotation matrix, Euler XYZ exactly as three.js composes it.
    pub fn rotation(&self) -> [[f64; 3]; 3] {
        let [x, y, z] = self.rotate_degrees.map(f64::to_radians);
        let (a, b, c, d, e, f) = (x.cos(), x.sin(), y.cos(), y.sin(), z.cos(), z.sin());
        let (ae, af, be, bf) = (a * e, a * f, b * e, b * f);
        [[c * e, -c * f, d], [af + be * d, ae - bf * d, -b * c], [bf - ae * d, be + af * d, a * c]]
    }

    /// 📍️ A position: `pivot + R·S·(p − pivot) + translate`.
    pub fn point(&self, point: [f64; 3]) -> [f64; 3] {
        let rotation = self.rotation();
        let local: [f64; 3] = std::array::from_fn(|axis| (point[axis] - self.pivot[axis]) * self.scale[axis]);
        std::array::from_fn(|row| self.pivot[row] + self.translate[row] + rotation[row].iter().zip(local).map(|(entry, value)| entry * value).sum::<f64>())
    }

    /// 🧭️ A unit direction: rotated by the inverse transpose of `R·S`, renormalised.
    pub fn direction(&self, direction: [f64; 3]) -> [f64; 3] {
        let rotation = self.rotation();
        let scaled: [f64; 3] = std::array::from_fn(|axis| direction[axis] / self.scale[axis]);
        let rotated: [f64; 3] = std::array::from_fn(|row| rotation[row].iter().zip(scaled).map(|(entry, value)| entry * value).sum());
        let length = rotated.iter().map(|value| value * value).sum::<f64>().sqrt();
        if length == 0.0 { rotated } else { rotated.map(|value| value / length) }
    }

    /// 🎯️ A unit quaternion `[x, y, z, w]` composed in front of an existing one (a node rotation under this transform).
    pub fn quaternion(&self, base: [f64; 4]) -> [f64; 4] {
        let [x, y, z] = self.rotate_degrees.map(|degrees| degrees.to_radians() / 2.0);
        let (c1, c2, c3, s1, s2, s3) = (x.cos(), y.cos(), z.cos(), x.sin(), y.sin(), z.sin());
        let delta = [s1 * c2 * c3 + c1 * s2 * s3, c1 * s2 * c3 - s1 * c2 * s3, c1 * c2 * s3 + s1 * s2 * c3, c1 * c2 * c3 - s1 * s2 * s3];
        let [ax, ay, az, aw] = delta;
        let [bx, by, bz, bw] = base;
        let product = [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz];
        let length = product.iter().map(|value| value * value).sum::<f64>().sqrt();
        product.map(|value| value / length)
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
