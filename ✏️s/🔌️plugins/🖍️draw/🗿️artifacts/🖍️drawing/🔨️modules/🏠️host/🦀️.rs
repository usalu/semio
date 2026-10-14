//! 🏠️ Drawing host: the retained store owner, its mutation authorities and the store initialization job.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;


//#region 🪢️TaxonomyMounts
#[path = "📬️outcome/🦀️.rs"]
pub mod outcome;
#[path = "🧰️owned/🦀️.rs"]
pub mod owned;
//#endregion 🪢️TaxonomyMounts
