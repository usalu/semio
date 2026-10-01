use semio_framework_os_kernel::*;
pub use semio_framework_os_kernel as store;
pub use semio_framework_os_kernel::{io_schema, os_dsl, sqlite_snapshot};
pub use semio_framework_os_kernel::{os_io, os_pack};
pub use semio_framework_os_kernel::os_pack::codec;

#[path = "../🦀️.rs"]
mod decoding;
#[path = "../../../🪶️native-encoding/🧪️tests/🦀️.rs"]
mod encoding;
#[path = "../../../🪶️native-retirement/🧪️tests/🦀️.rs"]
mod retirement;

#[path = "🧬️octets/🦀️.rs"]
mod octets;
