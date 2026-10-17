//! 📷️ Raster play app camera commands — `set-camera` and `set-camera-zoom`.

pub mod set_camera {
    use crate::config::{RasterConfig, RasterConfigMutation, SetCameraEdit};
    use semio_s_artifact_raster_raster_mutations::RasterMutation;
    use semio_s_artifact_raster_raster_model::RasterCamera;
    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    use semio_framework_value_derive::{FromValue, ToValue};

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "camera")]
    pub struct SetCamera {
        #[dsl(block)]
        pub camera: RasterCamera,
    }

    pub fn handle(payload: &SetCamera, _doc: &ArtifactView<'_, RasterSnapshot>, _cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        Ok(Emit::config(vec![RasterConfigMutation::SetCamera(SetCameraEdit { camera: payload.camera.clone() })]))
    }
}

pub mod set_camera_zoom {
    use crate::config::{RasterConfig, RasterConfigMutation, SetCameraEdit};
    use semio_s_artifact_raster_raster_mutations::RasterMutation;
    use semio_s_artifact_raster_raster_model::RasterCamera;
    use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
    use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
    use semio_framework_value_derive::{FromValue, ToValue};

    #[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord, semio_framework_value::RetireOwned)]
    #[dsl(keyword = "camera-zoom")]
    pub struct SetCameraZoom {
        pub zoom: f64,
    }

    pub fn handle(payload: &SetCameraZoom, _doc: &ArtifactView<'_, RasterSnapshot>, cfg: &ConfigView<'_, RasterConfig>) -> Result<Emit<RasterMutation, RasterConfigMutation>, Fault> {
        let camera = RasterCamera { zoom: payload.zoom, ..cfg.snapshot.camera.clone() };
        Ok(Emit::config(vec![RasterConfigMutation::SetCamera(SetCameraEdit { camera })]))
    }
}
