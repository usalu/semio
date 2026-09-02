//! 🏃️ Headless OS workflow runner (Shape V2 entry).
// 🔁️ ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W1: was
// aliased to `semio_framework_os_kernel` (the wasm-safe kernel crate, which never mounted
// `🔁️workflow` and architecturally can't — see that crate's glue.rs comment); `🔁️workflow` is
// mounted in `semio-framework` (the full framework crate, already a direct dependency below) instead.
extern crate semio_framework as workflow;
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
#[path = "../../🦀️.rs"]
mod run_lib;
pub use run_lib::*;
