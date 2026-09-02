use super::{assert_fixture_descriptor, DemoDiff, DemoSnapshot};
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
#[path = "↩️restore-n/🦀️.rs"]
mod restore_n;
pub use restore_n::RestoreN;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::Mutations, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = DemoSnapshot, diff = DemoDiff, schema = "demo.doc")]
pub(crate) enum DemoMutation {
    SetN(SetN),
    DeleteN(DeleteN),
    AddN(AddN),
    RestoreN(RestoreN),
}
