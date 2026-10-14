//! 📚️ Drawing examples: the example documents the subset advertises and `setActiveExample` loads.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use semio_framework_plugin::ExampleSource;
use std::sync::OnceLock;
/// 📚️ The one example-document catalogue of this subset — the same slice the shell's example switcher
/// reads through `SubsetDeclaration.examples` and the same one `setActiveExample` resolves its
/// `example_id` against, so an id the switcher offers is exactly an id the command accepts.
/// `✏️editor/📚️examples/🎬️demo-session` is deliberately NOT a row here: it carries a `.cmd.semio`
/// command-replay script, a session fixture for the editor's own tests, not a drawing document, and
/// registering it would advertise an id `setActiveExample` could never parse into a document.
pub fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![crate::demo::source()]).as_slice()
}

//#region 🪢️TaxonomyMounts
#[path = "🎬️demo/🦀️.rs"]
pub mod demo;
//#endregion 🪢️TaxonomyMounts
