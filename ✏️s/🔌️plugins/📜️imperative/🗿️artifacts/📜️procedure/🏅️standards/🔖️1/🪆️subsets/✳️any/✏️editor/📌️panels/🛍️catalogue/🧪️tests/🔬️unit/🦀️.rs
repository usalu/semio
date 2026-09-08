
use super::*;
use crate::editor::procedure::testkit::{imperative_app, render as render_body};

#[semio_framework_async_macros::async_test]
async fn catalogue_lists_step_kinds_in_native_locale_by_default() {
    let mut app = imperative_app().await;
    let json = render_body(&mut app, IMPERATIVE_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains("Set state"));
    assert!(json.contains("Print log"));
    assert!(json.contains("While"));
}

#[semio_framework_async_macros::async_test]
async fn catalogue_resolves_native_german_from_the_config_locale() {
    use crate::editor::procedure::ImperativeCommand;
    use crate::editor::procedure::commands::set_locale;
    use crate::editor::procedure::testkit::dispatch;
    let mut app = imperative_app().await;
    dispatch(&mut app, ImperativeCommand::SetLocale(set_locale::SetLocale { value: "de-DE".into() })).await;
    let json = render_body(&mut app, IMPERATIVE_PLAY_BODY_CATALOGUE).await;
    assert!(json.contains("Zustand setzen"));
    assert!(json.contains("Log ausgeben"));
    assert!(json.contains("Solange"));
}
