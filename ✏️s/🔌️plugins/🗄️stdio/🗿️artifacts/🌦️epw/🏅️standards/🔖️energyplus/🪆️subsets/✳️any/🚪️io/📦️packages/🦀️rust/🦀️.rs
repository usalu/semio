//! 🚪️ io-registry of `semio-s-artifact-stdio-epw` — a path-preserving carve of the artifact root mounts.

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;

pub use semio_s_artifact_stdio_contract::apply_mutation;
pub use semio_s_artifact_stdio_contract::apply_mutation_checked;
pub use semio_s_artifact_stdio_contract::mutation_from_payload_json;
pub use semio_s_artifact_stdio_contract::mutation_inverse;
pub use semio_s_artifact_stdio_contract::mutation_payload_json;
pub use semio_s_artifact_stdio_contract::MutationRefusal;
pub use semio_s_artifact_stdio_epw_snapshot::{native_codecs};
pub use semio_s_artifact_stdio_epw_mutations::{STDIO_EPW_DOCUMENT_SCHEMA};
pub use crate::standards::energyplus::subsets::any::io::artifact_items_3::{ARTIFACT_DEFINITION_SCHEMA};
pub use crate::standards::energyplus::subsets::any::io::artifact_items_4::{ARTIFACT_DEFINITION_CONSTRAINT};
pub use crate::standards::energyplus::subsets::any::io::artifact_items_5::{validate_definition_schema};
pub use crate::standards::energyplus::subsets::any::io::artifact_items_6::{definition};
pub use crate::standards::energyplus::subsets::any::io::artifact_items_7::{formats};
pub use crate::standards::energyplus::subsets::any::io::artifact_items_9::{artifact_kind, assembly, declaration};
pub use crate::standards::energyplus::subsets::any::io::artifact_items_10::{contribution};
pub use crate::standards::energyplus::subsets::any::io::artifact_items_11::{register, register_pilot_languages};

#[path = "🪞️mirror/🦀️.rs"]
mod mirror;
pub use mirror::*;
