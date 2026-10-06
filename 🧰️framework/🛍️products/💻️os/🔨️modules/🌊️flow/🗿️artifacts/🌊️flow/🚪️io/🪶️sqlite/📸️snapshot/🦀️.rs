//! 🌊️ Flow controlled native and relational I/O.
use crate::vcs::*;
use crate::io::text::snapshot::*;
#[path = "🚦️native/🛫️encoding/🦀️.rs"]
pub(crate) mod flow_native_encoding;
#[path = "🚦️native/🛬️decoding/🦀️.rs"]
pub(crate) mod flow_native_decoding;
#[path = "🚦️native/♻️retirement/🦀️.rs"]
pub(crate) mod flow_native_retirement;
#[path = "🚦️native/🛂️carrier/🦀️.rs"]
pub(crate) mod flow_native_carrier;
#[path = "🚦️native/📏️bound/🦀️.rs"]
pub(crate) mod flow_native_bound;
#[path = "🚦️native/🛂️capability/🦀️.rs"]
pub(crate) mod flow_snapshot_capability;
#[path = "🛫️projection/🦀️.rs"]
pub(crate) mod flow_sql_projection;
#[path = "🛬️reconstruction/🦀️.rs"]
pub(crate) mod flow_sql_reconstruction;
