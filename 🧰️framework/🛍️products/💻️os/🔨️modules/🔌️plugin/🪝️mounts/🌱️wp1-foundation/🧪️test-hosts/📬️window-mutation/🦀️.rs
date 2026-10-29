//! 📬️ Test host of `window_mutation`: the erased-payload and refresh laws and the close helpers the window tests share.
use semio_framework_plugin_window_config_mutation::refresh::WindowAuthorityRefreshStep;
use semio_framework_plugin_window_config_mutation::ErasedWindowMutationValue;
use semio_framework_value::retirement::RetireOwned;
pub use semio_framework_plugin_window_transient::window_mutation::*;
pub use semio_framework_plugin_window_config_mutation::refresh::{refresh_demands, refresh_transfer, WindowRefreshSnapshot};
pub(crate) use crate::window_mutation_tests::tests::{close_original as close_test_original, close_with as close_test_original_with_pump};
#[doc(hidden)]
#[path = "../../../../🪟️window/📬️mutation/🧪️tests/🦀️.rs"]
pub mod tests;
