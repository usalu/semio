//! 🏠️ Ownership stamp stored on each independently persisted child envelope.
use crate::os_io::ArtifactRef;
use semio_framework_value_derive::{FromValue, ToValue};

/// 🪪️ Exact owning parent, declared slot and durable child identity.
#[derive(Clone, Debug, PartialEq, Eq, ToValue, FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct OwnerRef {
    pub parent: ArtifactRef,
    pub slot: String,
    pub child_id: String,
}
