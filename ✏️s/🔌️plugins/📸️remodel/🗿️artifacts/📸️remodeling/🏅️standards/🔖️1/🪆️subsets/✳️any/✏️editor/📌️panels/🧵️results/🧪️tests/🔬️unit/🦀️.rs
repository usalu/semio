use super::*;
use crate::editor::remodeling::unit_tests::context::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn a_fresh_document_reports_no_sparse_dense_trajectory_or_geo_products() {
    let mut app = app().await;
    let body = render_body(&mut app, REMODELING_PLAY_BODY_RESULTS).await;
    for label in ["Sparse point cloud: none", "Dense point cloud: none", "Trajectory: none", "Geo products: none"] {
        assert!(body.contains(label), "{label} is reported: {body}");
    }
}
