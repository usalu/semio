//! 🔺️ Sparse diff builder for `UpdateGeoParams` — the field is always present, so there is no
//! missing-target case. Non-finite/non-positive distances, an out-of-range origin, or a zero ortho
//! resolution ⇒ Fatal `mutation.invariant`; identical params ⇒ Warning `mutation.no-op`.
use crate::artifacts::remodeling::diff::RemodelingDiff;
use crate::artifacts::remodeling::RemodelingSnapshot;

//#region 🔖️Diff
pub fn diff(payload: &super::UpdateGeoParams, base: &RemodelingSnapshot) -> protocol::MutationOutcome<RemodelingDiff> {
    let params = &payload.params;
    let distances_ok = [params.gsd_m, params.dsm_cell_m, params.dtm_filter_radius_m].iter().all(|value| value.is_finite() && *value > 0.0);
    let lat_ok = params.origin_lat.map_or(true, |lat| lat.is_finite() && (-90.0..=90.0).contains(&lat));
    let lon_ok = params.origin_lon.map_or(true, |lon| lon.is_finite() && (-180.0..=180.0).contains(&lon));
    if !distances_ok || params.ortho_max_px == 0 || !lat_ok || !lon_ok {
        return protocol::MutationOutcome::fatal("mutation.invariant", "Geo params need finite positive distances, a positive ortho resolution, and an in-range origin.", Vec::<String>::new());
    }
    if *params == base.params.geo {
        return protocol::MutationOutcome::empty().warn("mutation.no-op", "Geo params are unchanged.");
    }
    let mut params_state = base.params.clone();
    params_state.geo = params.clone();
    protocol::MutationOutcome::new(RemodelingDiff { params: Some(params_state), ..Default::default() })
}
//#endregion 🔖️Diff
