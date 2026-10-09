use super::{DemoDiff, DemoSnapshot, assert_fixture_descriptor};
use semio_framework_value_derive::{FromValue, ToValue};
use serde::{Deserialize, Serialize};

#[path = "🔢️set-n/🦀️.rs"]
mod set_n;
pub use set_n::SetN;
#[path = "⚠️set-warning-n/🦀️.rs"]
mod set_warning_n;
pub use set_warning_n::SetWarningN;
#[path = "🚫️set-error-n/🦀️.rs"]
mod set_error_n;
pub use set_error_n::SetErrorN;
#[path = "🛑️set-fatal-n/🦀️.rs"]
mod set_fatal_n;
pub use set_fatal_n::SetFatalN;
#[path = "↩️assign-n/🦀️.rs"]
mod assign_n;
pub use assign_n::AssignN;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, semio_framework_value_derive::RetireOwned, FromValue, dsl::Mutations, semio_framework_dsl_record_derive::DslEnum)]
#[serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = DemoSnapshot, diff = DemoDiff, schema = "severity.doc")]
pub(crate) enum SeverityMutation {
    SetN(SetN),
    SetWarningN(SetWarningN),
    SetErrorN(SetErrorN),
    SetFatalN(SetFatalN),
    AssignN(AssignN),
}
