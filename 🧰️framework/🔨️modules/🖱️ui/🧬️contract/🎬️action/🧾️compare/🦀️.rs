//! 🧬️ Canonical action compare traversal over actual fields.
use crate::*;
use semio_framework_ui_contract_values::{ui_compare_fields,ui_compare_scalar};
use semio_framework_ui_contract_values::compare_protocol::*;
ui_compare_scalar!(Trigger);
crate::ui_action_field_catalog!(ui_compare_fields);
