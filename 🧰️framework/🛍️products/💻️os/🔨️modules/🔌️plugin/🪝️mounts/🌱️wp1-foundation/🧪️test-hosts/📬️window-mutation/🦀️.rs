//! 📬️ Test host of `window_mutation`: the erased-payload and refresh laws and the close helpers the window tests share.
use semio_framework_value::retirement::RetireOwned;
pub use crate::window_mutation::*;
pub use crate::window_mutation::refresh::{refresh_demands, refresh_transfer, WindowRefreshSnapshot};
pub(crate) use tests::{close_original as close_test_original, close_with as close_test_original_with_pump};
#[path = "../../../../🪟️window/📬️mutation/🧪️tests/🦀️.rs"]
mod tests;
