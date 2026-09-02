//! ⚙️ ⚙️ Remodeling play app commands command — `set-geo-params`.

use crate::artifacts::remodeling::mutations::update_geo_params;
use crate::artifacts::remodeling::op::RemodelingMutation;
use crate::artifacts::remodeling::{GeoParams, RemodelingSnapshot};
use crate::editor::remodeling::config::{RemodelingConfig, RemodelingConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::DslRecord)]
#[dsl(keyword = "geo-params")]
pub struct SetGeoParams {
    pub enabled: bool,
    #[value(default)]
    pub origin_lon: Option<f64>,
    #[value(default)]
    pub origin_lat: Option<f64>,
    #[value(default)]
    pub origin_alt: Option<f64>,
    pub gsd_m: f32,
    pub dsm_cell_m: f32,
    pub dtm_filter_radius_m: f32,
    pub ortho_max_px: u32,
}

pub async fn handle(payload: &SetGeoParams, _doc: &ArtifactView<'_, RemodelingSnapshot>, _cfg: &ConfigView<'_, RemodelingConfig>) -> Result<Emit<RemodelingMutation, RemodelingConfigMutation>, Fault> {
    Ok(Emit::mutations(vec![update_geo_params(GeoParams {
        enabled: payload.enabled,
        origin_lon: payload.origin_lon,
        origin_lat: payload.origin_lat,
        origin_alt: payload.origin_alt,
        gsd_m: payload.gsd_m,
        dsm_cell_m: payload.dsm_cell_m,
        dtm_filter_radius_m: payload.dtm_filter_radius_m,
        ortho_max_px: payload.ortho_max_px,
    })]))
}
