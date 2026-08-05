//! 🧹️ Remodel play app commands — clearing and resetting reconstruction results.

use crate::apps::remodel::config::{RemodelConfig, RemodelConfigOperation};
use crate::artifacts::remodel::op::RemodelOperation;
use crate::artifacts::remodel::{MeshSource, RemodelMesh, RemodelScene};
use semio_framework_plugin::{mesh_from_kind, ConfigView, DocumentView, Emit, Fault, MeshData};
use serde::{Deserialize, Serialize};

//#region 🔖️Results
/// 📦️ The seeded stand-in mesh a fresh document (and `resetPlaceholderMesh`) carries.
fn placeholder_result() -> RemodelMesh {
    RemodelMesh { mesh: mesh_from_kind("box"), source: MeshSource::Placeholder, texture_asset_id: None, watertight: None }
}

/// 🫙️ An empty mesh result — what `clearMeshResult`/`clearResult` leave behind.
fn empty_result() -> RemodelMesh {
    RemodelMesh { mesh: MeshData::default(), source: MeshSource::Placeholder, texture_asset_id: None, watertight: None }
}
//#endregion 🔖️Results

//#region 🔖️ResetPlaceholderMesh
pub mod reset_placeholder_mesh {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "reset-placeholder-mesh")]
    pub struct ResetPlaceholderMesh {}

    pub fn handle(_payload: &ResetPlaceholderMesh, _doc: &DocumentView<'_, RemodelScene>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelOperation, RemodelConfigOperation>, Fault> {
        Ok(Emit::operations(vec![RemodelOperation::SetMeshResult { mesh: Box::new(placeholder_result()) }]))
    }
}
//#endregion 🔖️ResetPlaceholderMesh

//#region 🔖️ClearSparse
pub mod clear_sparse {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "clear-sparse")]
    pub struct ClearSparse {}

    pub fn handle(_payload: &ClearSparse, _doc: &DocumentView<'_, RemodelScene>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelOperation, RemodelConfigOperation>, Fault> {
        Ok(Emit::operations(vec![RemodelOperation::SetSparse { sparse: None }]))
    }
}
//#endregion 🔖️ClearSparse

//#region 🔖️ClearDense
pub mod clear_dense {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "clear-dense")]
    pub struct ClearDense {}

    pub fn handle(_payload: &ClearDense, _doc: &DocumentView<'_, RemodelScene>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelOperation, RemodelConfigOperation>, Fault> {
        Ok(Emit::operations(vec![RemodelOperation::SetDense { dense: None }]))
    }
}
//#endregion 🔖️ClearDense

//#region 🔖️ClearMeshResult
pub mod clear_mesh_result {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "clear-mesh-result")]
    pub struct ClearMeshResult {}

    pub fn handle(_payload: &ClearMeshResult, _doc: &DocumentView<'_, RemodelScene>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelOperation, RemodelConfigOperation>, Fault> {
        Ok(Emit::operations(vec![RemodelOperation::SetMeshResult { mesh: Box::new(empty_result()) }]))
    }
}
//#endregion 🔖️ClearMeshResult

//#region 🔖️ClearTracks
pub mod clear_tracks {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "clear-tracks")]
    pub struct ClearTracks {}

    pub fn handle(_payload: &ClearTracks, _doc: &DocumentView<'_, RemodelScene>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelOperation, RemodelConfigOperation>, Fault> {
        Ok(Emit::operations(vec![RemodelOperation::SetTracks { tracks: Vec::new() }]))
    }
}
//#endregion 🔖️ClearTracks

//#region 🔖️ClearGeoProducts
pub mod clear_geo_products {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "clear-geo-products")]
    pub struct ClearGeoProducts {}

    pub fn handle(_payload: &ClearGeoProducts, _doc: &DocumentView<'_, RemodelScene>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelOperation, RemodelConfigOperation>, Fault> {
        Ok(Emit::operations(vec![RemodelOperation::SetGeoProducts { geo: None }]))
    }
}
//#endregion 🔖️ClearGeoProducts

//#region 🔖️ClearResult
pub mod clear_result {
    use super::*;

    #[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
    #[dsl(keyword = "clear-result")]
    pub struct ClearResult {}

    /// 🧹️ Resets all seven `ReconstructionResults` fields in one undoable step.
    pub fn handle(_payload: &ClearResult, _doc: &DocumentView<'_, RemodelScene>, _cfg: &ConfigView<'_, RemodelConfig>) -> Result<Emit<RemodelOperation, RemodelConfigOperation>, Fault> {
        Ok(Emit::operations(vec![
            RemodelOperation::SetMeshResult { mesh: Box::new(empty_result()) },
            RemodelOperation::SetSparse { sparse: None },
            RemodelOperation::SetDense { dense: None },
            RemodelOperation::SetTrajectory { trajectory: None },
            RemodelOperation::SetTracks { tracks: Vec::new() },
            RemodelOperation::SetGeoProducts { geo: None },
            RemodelOperation::SetQc { qc: None },
        ]))
    }
}
//#endregion 🔖️ClearResult

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::apps::remodel::testkit::{app, dispatch};
    use crate::apps::remodel::RemodelCommand;
    use semio_framework_plugin::{testkit, PluginApp};

    #[test]
    fn clear_result_resets_all_seven_result_fields_and_reset_placeholder_restores_the_box() {
        let mut app = app();
        let result = dispatch(&mut app, RemodelCommand::ClearResult(clear_result::ClearResult {}));
        assert_eq!(result.operations.len(), 7, "clearResult resets all 7 ReconstructionResults fields");
        assert_eq!(app.projection().expect("materialize projection").results.mesh.mesh.vertex_count(), 0);
        dispatch(&mut app, RemodelCommand::ResetPlaceholderMesh(reset_placeholder_mesh::ResetPlaceholderMesh {}));
        assert_eq!(app.projection().expect("materialize projection").results.mesh.source, MeshSource::Placeholder);
        assert!(app.projection().expect("materialize projection").results.mesh.mesh.vertex_count() > 0);
    }

    #[test]
    fn undo_redo_round_trip_through_the_wrapper() {
        let mut app = app();
        let placeholder_vertex_count = app.projection().expect("materialize projection").results.mesh.mesh.vertex_count();
        assert!(placeholder_vertex_count > 0, "the seeded placeholder box must have vertices");
        testkit::assert_undo_redo_round_trip(
            &mut app,
            RemodelCommand::ClearResult(clear_result::ClearResult {}),
            |app| app.projection().expect("materialize projection").results.mesh.mesh.vertex_count(),
            placeholder_vertex_count,
            0,
        );
    }

    #[test]
    fn each_narrow_clear_touches_exactly_one_result_field() {
        let mut app = app();
        for command in [
            RemodelCommand::ClearSparse(clear_sparse::ClearSparse {}),
            RemodelCommand::ClearDense(clear_dense::ClearDense {}),
            RemodelCommand::ClearMeshResult(clear_mesh_result::ClearMeshResult {}),
            RemodelCommand::ClearTracks(clear_tracks::ClearTracks {}),
            RemodelCommand::ClearGeoProducts(clear_geo_products::ClearGeoProducts {}),
        ] {
            assert_eq!(dispatch(&mut app, command).operations.len(), 1);
        }
    }
}
//#endregion 🧪️Tests
