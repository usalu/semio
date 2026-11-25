//! 🏁️ Exercises the retained publisher's stale-root recovery through retirement and a subsequent commit.

use semio_plugin_tests_01::retained_command_test_hosts::retained_command_unit_1::test_retained_stale_publication_boundary;
use semio_plugin_tests_05::plugin_runtime_plugin_builder_contract_support_1::TestApp;

#[semio_framework_async_macros::async_test]
async fn stale_publication_retires_without_blocking_the_next_valid_command() {
    test_retained_stale_publication_boundary::<TestApp<false, 1>>().await;
}
