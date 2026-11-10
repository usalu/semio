//! 📥️ Test-only source admission re-exported as the module the owned decoding suites address, with the shared receiver driver.
pub use semio_framework_value_testing_admission::*;
#[path="../../../../🧪️testing/📥️admission/receiver-driver.rs"]
mod driver;
pub use driver::{admit,bank,close,close_source,funded,lifecycle,none,receive_cut,receive_faulted,receive_traced,retire,settle,Trace};
