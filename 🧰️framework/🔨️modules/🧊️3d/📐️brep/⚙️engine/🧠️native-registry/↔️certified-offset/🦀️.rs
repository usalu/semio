//! ↔️ Original sharp offsets of certified rectangular solids.

use super::*;

impl BrepRegistry {
    /// 📦️ Plans an axis-aligned sharp box offset in the source's world position.
    pub fn plan_certified_solid_offset(&mut self, handle: &str, distance: f64) -> Result<SolidPublication, ValueError> {
        if !distance.is_finite() || distance.abs() <= 1e-15 {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "solid offset requires a finite nonzero distance"));
        }
        let Some(SolidPrimitive::Box { origin, width, depth, height }) = self.live_solid_primitive(handle) else {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original sharp solid offset requires a certified box"));
        };
        let axes = [(origin.x, width), (origin.y, depth), (origin.z, height)].map(|(min, size)| {
            let max = (min + size) + distance;
            let min = min - distance;
            (min, max - min)
        });
        if axes.iter().any(|&(min, span)| !min.is_finite() || !span.is_finite() || span <= Tol::DEFAULT.value()) {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "solid offset collapsed a box dimension or exceeded the coordinate range"));
        }
        let [(x, width), (y, depth), (z, height)] = axes;
        self.plan_solid_primitive(SolidPrimitive::Box { origin: Pnt3::new(x, y, z), width, depth, height })
    }
}
