//! 🧰️ Original caller authorities for stdio artifact tests: native snapshot owners, mounted owner policy and routed IO runs.
//!
//! @see 🏪️store `NativeSnapshotDecodeOwner` / `NativeSnapshotEncodeOwner` — the owners every SQLite snapshot codec receives.

#[path = "🔐️grants/🦀️.rs"]
mod grants;
#[path = "🧲️native-owners/🦀️.rs"]
mod native_owners;
#[path = "🛫️native-output/🦀️.rs"]
mod native_output;
#[path = "🔀️routed-io/🦀️.rs"]
mod routed_io;

pub use grants::{NATIVE_MAXIMUM_BYTES, mounted_policy, original_grant, unbounded_grant};
pub use native_output::{assert_output, assert_reconstruction};
pub use native_owners::{decode_control, with_decoding, with_encoding, with_encoding_async};
pub use routed_io::{io_run, with_sqlite_recipient};
