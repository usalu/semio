use super::*;
use crate::editor::procedure::unit_tests::context::{imperative_app, render as render_body};
use crate::editor::procedure::ImperativeCommand;

#[semio_framework_async_macros::async_test]
async fn renders_table_scene() {
    let mut app = imperative_app().await;
    let json = render_body(&mut app, IMPERATIVE_PLAY_BODY_MAIN).await;
    assert!(json.contains("table"));
}

#[semio_framework_async_macros::async_test]
async fn run_command_expands_scope_into_readable_rows_without_truncation() {
    use crate::editor::procedure::commands::run;
    use crate::editor::procedure::unit_tests::context::dispatch;
    let mut app = imperative_app().await;
    dispatch(&mut app, ImperativeCommand::Run(run::Run {})).await;
    let json = render_body(&mut app, IMPERATIVE_PLAY_BODY_MAIN).await;
    assert!(json.contains("log.print"), "main table lists default path steps after run");
}
