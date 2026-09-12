use super::{assert_fixture_descriptor, DemoDiff, DemoSnapshot};
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
#[path = "↩️restore-n/🦀️.rs"]
mod restore_n;
pub use restore_n::RestoreN;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, ToValue, FromValue, dsl::Mutations, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = DemoSnapshot, diff = DemoDiff, schema = "severity.doc")]
pub(crate) enum SeverityMutation {
    SetN(SetN),
    SetWarningN(SetWarningN),
    SetErrorN(SetErrorN),
    SetFatalN(SetFatalN),
    RestoreN(RestoreN),
}
