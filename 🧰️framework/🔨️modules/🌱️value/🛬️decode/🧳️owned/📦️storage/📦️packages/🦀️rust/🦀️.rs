//! 📦️ Canonical typed raw allocation and initialized prefix custody.
pub use semio_framework_value_control_contract::*;
#[path="../../🦀️.rs"]mod storage;
pub use storage::{NativeOwnedStorage,NativeOwnedPrefix,NativeSeedTemplate};

#[path="../../../🧩️record/📦️storage/🦀️.rs"]mod record;
pub use record::NativeRecordStorage;
