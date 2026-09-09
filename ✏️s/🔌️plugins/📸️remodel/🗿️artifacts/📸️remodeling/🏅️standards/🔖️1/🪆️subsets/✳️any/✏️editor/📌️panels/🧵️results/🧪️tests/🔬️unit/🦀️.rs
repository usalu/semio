use super::*;
use crate::editor::remodeling::testkit::{app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn a_fresh_document_reports_no_sparse_dense_trajectory_or_geo_products() {
    let mut app = app().await;
    let body = render_body(&mut app, REMODELING_PLAY_BODY_RESULTS).await;
    assert_eq!(body.matches("none").count(), 4, "sparse/dense/trajectory/geo all report 'none': {body}");
}
