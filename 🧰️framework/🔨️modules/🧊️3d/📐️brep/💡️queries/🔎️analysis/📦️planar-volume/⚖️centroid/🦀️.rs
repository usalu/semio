//! ⚖️ Compensated first moments for planar tetrahedra and disjoint solid members.

use super::{Pnt3, ValueError, accumulate, invalid};
use semio_framework_3d_brep_vector::Vec3;

#[derive(Default)]
pub(super) struct Centroid {
    local: [f64; 3],
    local_correction: [f64; 3],
    group_origin: Option<Pnt3>,
    group: [f64; 3],
    group_correction: [f64; 3],
    pub(super) result: Option<Pnt3>,
}

impl Centroid {
    pub(super) fn tetrahedron(&mut self, a: Vec3, b: Vec3, c: Vec3, weight: f64) {
        for (axis, value) in (a+b+c).to_array().into_iter().enumerate() {
            accumulate(&mut self.local[axis], &mut self.local_correction[axis], value*weight);
        }
    }

    pub(super) fn finish_member(&mut self, origin: Pnt3, signed: f64, volume: f64) -> Result<(), ValueError> {
        if volume <= 0. { return Err(invalid()); }
        let center = origin+Vec3::from_array(self.local.map(|moment| (moment/signed)*0.25));
        if !center.to_array().into_iter().all(f64::is_finite) { return Err(invalid()); }
        let anchor = *self.group_origin.get_or_insert(center);
        for (axis, value) in (center-anchor).to_array().into_iter().enumerate() {
            accumulate(&mut self.group[axis], &mut self.group_correction[axis], value*volume);
        }
        self.local=[0.;3];
        self.local_correction=[0.;3];
        Ok(())
    }

    pub(super) fn finish(&mut self, volume: f64) -> Result<(), ValueError> {
        let point=self.group_origin.ok_or_else(invalid)?+Vec3::from_array(self.group.map(|moment|moment/volume));
        if !point.to_array().into_iter().all(f64::is_finite) {return Err(invalid());}
        self.result=Some(point);
        Ok(())
    }
}
