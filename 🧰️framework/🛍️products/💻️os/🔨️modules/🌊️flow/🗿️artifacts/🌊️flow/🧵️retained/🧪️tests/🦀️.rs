//! 🧪️ Typed Flow retirement laws: exact owner accounting, field custody under independent grants and worker transfer, driven through the document snapshot.

use crate::retained::{FlowOwner, FlowRetirement};
use crate::*;
use semio_framework_value::retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep};

//#region 🧪️RetirementLaws
#[path = "🧵️retained/🦀️.rs"]
mod tests;

#[path = "🧬️fields/🦀️.rs"]
pub(crate) mod field_tests;
//#endregion 🧪️RetirementLaws
