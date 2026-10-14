//! 📚️ Puzzle3d examples — the authored documents the example picker opens: concrete forest and the Nakagin capsule tower.

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

//#region 🌳️Modules
#[path = "🌲️concrete-forest/🦀️.rs"]
pub mod concrete_forest;
#[cfg(test)]
#[path = "🌲️concrete-forest/🧪️tests/🧩️example/🦀️.rs"]
mod concrete_forest_tests;
#[path = "🏗️nakagin-capsule-tower/🦀️.rs"]
pub mod nakagin_capsule_tower;
#[cfg(test)]
#[path = "🏗️nakagin-capsule-tower/🧪️tests/🧩️example/🦀️.rs"]
mod nakagin_capsule_tower_tests;
//#endregion 🌳️Modules
