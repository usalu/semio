use super::{DemoSnapshot, DemoDiff, assert_fixture_descriptor};
use serde::{Deserialize, Serialize};

#[path = "🔢️set-n/🦀️.rs"] mod set_n;
pub use set_n::SetN;
#[path = "↩️restore-n/🦀️.rs"] mod restore_n;
pub use restore_n::RestoreN;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::Mutations, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = DemoSnapshot, diff = DemoDiff, schema = "validated.doc")]
pub(crate) enum ValidatedMutation {
    SetN(SetN),
    RestoreN(RestoreN),
}
