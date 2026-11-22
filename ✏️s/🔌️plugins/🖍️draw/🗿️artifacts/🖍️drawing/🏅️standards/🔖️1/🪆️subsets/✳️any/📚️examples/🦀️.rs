//! 📚️ Drawing examples: the example documents the subset advertises and `setActiveExample` loads.

#![allow(clippy::result_large_err)]
#![allow(unexpected_cfgs)]
extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;

use semio_plugin::ExampleSource;
use std::sync::OnceLock;
/// 📚️ The one example-document catalogue of this subset — the same slice the shell's example switcher
/// reads through `SubsetDeclaration.examples` and the same one `setActiveExample` resolves its
/// `example_id` against, so an id the switcher offers is exactly an id the command accepts.
/// `✏️editor/📚️examples/🎬️demo-session` is deliberately NOT a row here: it carries a `.cmd.semio`
/// command-replay script, a session fixture for the editor's own tests, not a drawing document, and
/// registering it would advertise an id `setActiveExample` could never parse into a document.
pub fn examples() -> &'static [ExampleSource] {
    static EXAMPLES: OnceLock<Vec<ExampleSource>> = OnceLock::new();
    EXAMPLES.get_or_init(|| vec![demo::source(), ExampleSource::new(
        "blank",
        semio_ui_locale::LocalizedLabel::native("Blank Drawing", "Leere Zeichnung"),
        include_str!("../🖼️assets/📄️blank/🗣️.dsl.semio"),
        "file",
    )]).as_slice()
}


/// 📝️ The selectable blank document starts with an empty 1024-square artboard.
#[cfg(test)]
#[test]
fn registered_blank_starts_with_no_layers_or_assets() {
    let source = examples().iter().find(|example| example.id() == "blank").expect("registered blank drawing");
    let document = semio_s_artifact_draw_drawing_model::io::text::snapshot::parse_dsl(&source.document_json()).expect("parse registered blank drawing");
    assert_eq!(document.id, "blank");
    assert!(document.layers.is_empty());
    assert!(document.assets.is_empty());
    let artboard = document.artboard.expect("blank drawing artboard");
    assert_eq!((artboard.width, artboard.height), (1024.0, 1024.0));
}

//#region 🪢️TaxonomyMounts
#[path = "🎬️demo/🦀️.rs"]
pub mod demo;
//#endregion 🪢️TaxonomyMounts
