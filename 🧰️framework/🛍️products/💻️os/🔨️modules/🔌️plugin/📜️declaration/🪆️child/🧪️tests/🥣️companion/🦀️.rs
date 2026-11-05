//! 🥣️ Companion tests of the decl-views child editor: original member lookup, vector read qualification, source custody and admission, run against the public surface of `semio-plugin-decl-views`.
//!
//! 🧭️ The test sources stay beside their owners and are mounted here with `#[path]` below a wrapper that restates the imports of the owning module; every fixture is read at run time from the decl-views folder.
#![cfg(test)]

extern crate semio_os_kernel as store;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as pack;

#[path = "🔎️lookup/🦀️.rs"]
mod lookup;
#[path = "📖️vector/🦀️.rs"]
mod vector;
#[path = "🔗️custody/🦀️.rs"]
mod custody;
#[path = "🤝️admission/🦀️.rs"]
mod admission;
