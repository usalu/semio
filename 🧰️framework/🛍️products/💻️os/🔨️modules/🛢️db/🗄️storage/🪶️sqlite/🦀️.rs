//! 🗄️ SQLite storage behind the process-wide typed, retained database I/O lane.
#![allow(async_fn_in_trait)]
extern crate semio_framework_os_kernel as pack;

//#region 🔖️SqliteStorage
#[cfg(not(target_arch = "wasm32"))]
#[path = "💽️sqlite-storage/🦀️.rs"]
mod sqlite_storage;

#[cfg(not(target_arch = "wasm32"))]
pub use sqlite_storage::SqliteStorage;
//#endregion 🔖️SqliteStorage
