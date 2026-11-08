use super::{DemoDiff, DemoSnapshot, assert_fixture_descriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[path = "🔢️set-n/🦀️.rs"]
mod set_n;
pub use set_n::SetN;
#[path = "↩️assign-n/🦀️.rs"]
mod assign_n;
pub use assign_n::AssignN;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, semio_framework_value_derive::RetireOwned, FromValue, semio_os_dsl::Mutations, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree, semio_framework_value::NativeReceive, semio_framework_dsl_record_derive::DslReceiving)]
#[canonical_json(owner=semio_framework_pack_json)]
#[serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = DemoSnapshot, diff = DemoDiff, schema = "timestamped.doc", reference = DemoSnapshot, reference_diff = DemoDiff)]
pub enum TimestampedMutation {
    SetN(SetN),
    AssignN(AssignN),
}

semio_os_dsl::original_ordinal_operation_receiving!(TimestampedMutation);

semio_os_store_one_item_publication::artifact_persisted_binary_operation!(TimestampedMutation);
