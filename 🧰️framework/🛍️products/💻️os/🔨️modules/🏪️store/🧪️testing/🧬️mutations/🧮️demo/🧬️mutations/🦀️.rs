use super::{DemoDiff, DemoSnapshot, assert_fixture_descriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[path = "🔢️set-n/🦀️.rs"]
mod set_n;
pub use set_n::SetN;
#[path = "🗑️delete-n/🦀️.rs"]
mod delete_n;
pub use delete_n::DeleteN;
#[path = "➕️add-n/🦀️.rs"]
mod add_n;
pub use add_n::AddN;
#[path = "↩️assign-n/🦀️.rs"]
mod assign_n;
pub use assign_n::AssignN;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, semio_framework_value_derive::RetireOwned, semio_framework_value::CanonicalJsonTree, dsl::Mutations, semio_framework_dsl_record_derive::DslEnum)]
#[canonical_json(owner=semio_framework_pack_json)]
#[serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = DemoSnapshot, diff = DemoDiff, schema = "demo.doc")]
pub(crate) enum DemoMutation {
    SetN(SetN),
    DeleteN(DeleteN),
    AddN(AddN),
    AssignN(AssignN),
}
