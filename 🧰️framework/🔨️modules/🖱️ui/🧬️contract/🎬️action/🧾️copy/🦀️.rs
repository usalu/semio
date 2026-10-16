//! 🧬️ Canonical action copy traversal over actual fields.
use crate::*;
use semio_framework_ui_contract_values::{ui_copy_fields,ui_copy_scalar};
use semio_framework_ui_contract_values::copy_protocol::*;
ui_copy_scalar!(Trigger);
crate::ui_action_field_catalog!(ui_copy_fields);
