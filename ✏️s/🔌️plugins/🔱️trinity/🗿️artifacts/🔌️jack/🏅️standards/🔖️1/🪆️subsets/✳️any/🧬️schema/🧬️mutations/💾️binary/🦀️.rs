//! 📡️ Trinity graph mutation binary framing and registry surface.

pub use crate::standards::v1::subsets::any::schema::wire_runtime::*;

/// 🧾️ Direct-owner binary tags in aggregate declaration order.
pub const BINARY_TAG_REGISTRY: &[(&str, u8)] = &[("SetQuery", super::set_query::binary::BINARY_TAG)];
