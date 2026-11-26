//! 🏁️ Exercises the retained publisher's stale-root recovery through retirement and a subsequent commit.

use semio_plugin_tests_01::retained_command_test_hosts::retained_command_unit_1::{test_retained_stale_publication_boundary, test_retained_stale_window_publication};
use semio_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::TestApp;
use semio_plugin_tests_05::publication_window_owner::{Mutation, Owner};
use semio_plugin_window_config::WindowConfigOwner;

#[semio_framework_async_macros::async_test]
async fn stale_publication_retires_without_blocking_the_next_valid_command() {
    test_retained_stale_publication_boundary::<TestApp<false, 1>>().await;
}

#[semio_framework_async_macros::async_test]
async fn stale_window_publication_retires_and_a_fresh_window_write_commits() {
    let window_id = "publication.window";
    let mut view = semio_framework::ViewModel::new(semio_ui_locale::Locale::En, semio_ui_locale::Terminology::Native);
    view.window_id = Some(window_id.into());
    view.window_instances.push(semio_framework::ViewWindowInstance {
        id: window_id.into(),
        window_kind_id: Owner::WINDOW_KIND_ID.into(),
    });
    test_retained_stale_window_publication::<TestApp<false, 1>, Owner>(view, || Mutation::Zoom { zoom: 2.5 }).await;
}
