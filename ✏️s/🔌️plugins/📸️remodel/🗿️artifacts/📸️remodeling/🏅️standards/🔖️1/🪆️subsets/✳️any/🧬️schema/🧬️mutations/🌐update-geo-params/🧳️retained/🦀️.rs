//! 🧳️ Geo-parameter changed-value projection and inverse, after its retained decision.
use super::UpdateGeoParams;
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::GeoParams;
crate::retained_remodeling_parameter_field!(UpdateGeoParams, GeoParams, geo, 7);
impl crate::RetainedParameterScalar for UpdateGeoParams {
    const WORK_DEMAND: u64 = 26;
    const NOOP_MESSAGE: &'static str = "Geo params are unchanged.";
    const REFUSAL_MESSAGE: Option<&'static str> = Some("Geo params need finite positive distances, a positive ortho resolution, and an in-range origin.");
    fn retained_parameter_disposition(&self, original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot) -> (crate::RetainedParameterDisposition, u64) {
        use crate::RetainedParameterDisposition::*;
        let p = &self.params; let old = &original.params.geo;
        let distances = p.gsd_m.is_finite() & (p.gsd_m > 0.0) & p.dsm_cell_m.is_finite() & (p.dsm_cell_m > 0.0) & p.dtm_filter_radius_m.is_finite() & (p.dtm_filter_radius_m > 0.0);
        let (latitude, latitude_work) = match p.origin_lat { None => (true, 1), Some(value) => (value.is_finite() & (value >= -90.0) & (value <= 90.0), 4) };
        let (longitude, longitude_work) = match p.origin_lon { None => (true, 1), Some(value) => (value.is_finite() & (value >= -180.0) & (value <= 180.0), 4) };
        let work = 7 + latitude_work + longitude_work;
        if !distances | (p.ortho_max_px == 0) | !latitude | !longitude { return (UntargetedRefusal, work); }
        fn optional(left: Option<f64>, right: Option<f64>) -> (bool, u64) { match (left, right) { (None, None) => (true, 1), (Some(left), Some(right)) => (left == right, 2), _ => (false, 1) } }
        let (same_lon, lon_work) = optional(p.origin_lon, old.origin_lon);
        let (same_lat, lat_work) = optional(p.origin_lat, old.origin_lat);
        let (same_alt, alt_work) = optional(p.origin_alt, old.origin_alt);
        let same = (p.enabled == old.enabled) & same_lon & same_lat & same_alt & (p.gsd_m == old.gsd_m) & (p.dsm_cell_m == old.dsm_cell_m) & (p.dtm_filter_radius_m == old.dtm_filter_radius_m) & (p.ortho_max_px == old.ortho_max_px);
        (if same { Unchanged } else { Changed }, work + 5 + lon_work + lat_work + alt_work)
    }
}
crate::retained_parameter_scalar_vectors!(UpdateGeoParams, "geo", include_str!("../../🧪️tests/🧫️retained-params/🧪️.json"), |original: &semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::RemodelingSnapshot, change: &serde_json::Map<String, serde_json::Value>| {
    let mut params = original.params.geo.clone();
    if let Some(value) = change.get("enabled") { params.enabled = value.as_bool().unwrap(); }
    if let Some(value) = change.get("gsd_m") { params.gsd_m = value.as_f64().unwrap() as f32; }
    UpdateGeoParams { params }
});
