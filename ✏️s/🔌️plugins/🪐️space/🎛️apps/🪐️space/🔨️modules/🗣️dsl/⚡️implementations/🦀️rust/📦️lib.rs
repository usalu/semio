//! 📜️ S Studio app — textual document grammar surface + laws (constitutional: dsl).
//!
//! 🕳️ Wraps `semio_framework_os::WorkflowDocument` (os-core's re-export of the kernel `workflow`
//! crate's `DocumentDsl` derive — the dissolved `OsProjection`'s successor, see `## The inversion`),
//! not a locally-owned document type — see `space_op`'s doc comment for why this app owns no
//! document/operation type.

use semio_framework_os::WorkflowDocument;

/// 📖️ Parses `WorkflowDocument` DSL text (the `.s` studio grammar) into a `WorkflowDocument`.
pub fn parse_dsl(text: &str) -> Result<WorkflowDocument, store::TextError> {
    <WorkflowDocument as store::DocumentDsl>::parse_dsl(text)
}

/// 🖨️ Prints a `WorkflowDocument` back to `.s` studio DSL text.
pub fn print_dsl(projection: &WorkflowDocument) -> String {
    store::DocumentDsl::print_dsl(projection)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use space_shared::demo_space_projection;

    #[test]
    fn demo_document_dsl_text_round_trips() {
        let projection = demo_space_projection();
        store::test_support::assert_dsl_round_trip(&projection);
    }

    #[test]
    fn parse_dsl_print_dsl_agree_on_demo_fixture() {
        let projection = demo_space_projection();
        let printed = print_dsl(&projection);
        let reparsed = parse_dsl(&printed).expect("reparse printed dsl");
        assert_eq!(reparsed, projection);
    }

    /// 🧮️ Per-app recipe item 3: `SpaceConfig` (this app's `DocumentApp::Config`, defined in
    /// `space_engine`) round-trips its own DSL extension independently of the `OsProjection` document
    /// grammar above.
    #[test]
    fn space_config_dsl_text_round_trips() {
        store::test_support::assert_dsl_round_trip(&space_engine::SpaceConfig::default());
    }
}
//#endregion 🧪️Tests
