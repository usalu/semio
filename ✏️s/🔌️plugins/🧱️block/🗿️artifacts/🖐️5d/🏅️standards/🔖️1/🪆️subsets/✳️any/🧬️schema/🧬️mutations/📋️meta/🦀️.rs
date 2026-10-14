//! 📋️ Block 5d meta mutation leaves.

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

//#region 🪢️Mounts
#[path = "../🔩add-attribute/🦀️.rs"]
pub mod add_attribute;
#[path = "../👤add-author/🦀️.rs"]
pub mod add_author;
#[path = "../➕add-compatibility-rule/🦀️.rs"]
pub mod add_compatibility_rule;
#[path = "../💬change-meta-description/🦀️.rs"]
pub mod change_meta_description;
#[path = "../🚷remove-attribute/🦀️.rs"]
pub mod remove_attribute;
#[path = "../🙅remove-author/🦀️.rs"]
pub mod remove_author;
#[path = "../✂️remove-compatibility-rule/🦀️.rs"]
pub mod remove_compatibility_rule;
//#endregion 🪢️Mounts
