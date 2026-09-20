use super::*;
use crate::editor::dag::unit_tests::context::{close, new_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn renders_compiled_dag_text_editor() {
    let mut app = new_app().await;
    let json = render_body(&mut app, DAG_PLAY_BODY_COMPILED).await;
    close(&mut app);
    assert!(json.contains("text-editor"));
}
