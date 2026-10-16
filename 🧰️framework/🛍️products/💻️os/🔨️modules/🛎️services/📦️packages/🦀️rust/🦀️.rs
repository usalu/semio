//! 📦️ Package glue — wiring only. Domain lives at owner `🦀️.rs`. `tokio` is confined to
//! that owner file (and this glue file never imports it directly); there is no wasm-bindgen surface
//! here because this crate has no wasm target — it is a native host-process crate by construction
//! (tokio's multi-thread runtime never targets wasm32).

#[path = "../../🦀️.rs"]
mod component;
#[path = "../../👀️file-watcher/🦀️.rs"]
mod file_watcher;
#[path = "../../🚪️native-io/🦀️.rs"]
mod native_io;
pub use component::*;
pub use file_watcher::OwnedFileChangeWatcher;
pub use native_io::{NATIVE_IO_PATH_CAPACITY, NativeIoJob, NativeIoRequest, NativeIoValue, NativeModifiedSet, NativePathSet};

/// 🪝️ Test hook of the companion crate `semio-framework-os-services-tests`: the private host modules next to the owner's service hooks, public only behind the `testkit` feature.
#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    pub use super::component::testkit::*;
    pub mod file_watcher {
        pub use crate::file_watcher::*;
    }
    pub mod native_io {
        pub use crate::native_io::*;
    }
}
