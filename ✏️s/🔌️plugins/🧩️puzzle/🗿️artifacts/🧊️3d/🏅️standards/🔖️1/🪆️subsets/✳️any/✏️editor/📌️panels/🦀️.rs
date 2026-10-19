//! 📌️ Puzzle3d panels — the catalogue, artifact outliner, inspection and settings panel bodies.

#![allow(clippy::result_large_err)]


extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

//#region 🌳️Modules
#[path = "🗿️artifact/🦀️.rs"]
pub mod artifact;
#[path = "🛍️catalogue/🦀️.rs"]
pub mod catalogue;
#[path = "🔍️inspection/🦀️.rs"]
pub mod inspection;
#[path = "⚙️settings/🦀️.rs"]
pub mod settings;
//#endregion 🌳️Modules
