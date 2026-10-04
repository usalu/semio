//! 📬️ Folds one authenticated, receipt-sealed directory page into the Home transient projection.
//!
//! The item this lane publishes is the canonical page itself (at most `store::os_directory::DIRECTORY_EVENT_PAGE_MAX_BYTES`),
//! so its cost never grows with the directory. Non-invertible by declaration: the projection is derived hub state the host
//! re-reads from the origin, never an edit anyone undoes.

use super::{HomeTransient, HomeTransientMutation};
use crate::editor::home::transient::DirectoryPageAdmission;

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "apply-directory-page")]
#[mutation_leaf(contract = ::protocol)]
pub struct ApplyDirectoryPage {
    /// 📄️ Canonical `DirectoryEventPageV1` JSON the authenticated hub returned, sealed by its receipt.
    pub page_json: String,
}

impl protocol::MutationKind<HomeTransient, HomeTransientMutation> for ApplyDirectoryPage {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "apply", entity: "directory-page", kind: "apply-directory-page", record: "ApplyDirectoryPage" };
    fn diff(&self, base: &HomeTransient) -> protocol::MutationOutcome<HomeTransient> {
        let Ok(page) = store::os_directory::DirectoryEventPageV1::parse_canonical_json(&self.page_json) else {
            return protocol::MutationOutcome::new(base.clone()).absorb_messages([protocol::MutationMessage::fatal("mutation.invariant", "The directory page is not one canonical, receipt-sealed page.").at(["directory"])]);
        };
        match base.directory().admit_page(&page) {
            Ok(DirectoryPageAdmission::Held) => protocol::MutationOutcome::new(base.clone()).warning("mutation.no-op", format!("The directory projection already holds the frontier through sequence {}.", page.through_seq_inclusive)),
            Ok(DirectoryPageAdmission::Fold) => protocol::MutationOutcome::new(HomeTransient::with_directory(base.directory().fold_page(&page))),
            Err(fault) => protocol::MutationOutcome::new(base.clone()).absorb_messages([protocol::MutationMessage::error("mutation.target-mismatch", format!("The directory page does not continue the frontier the projection holds ({}).", fault.code.0)).at(["directory"])]),
        }
    }
    fn inverse(&self, _base: &HomeTransient) -> Result<Vec<HomeTransientMutation>, semio_framework_value::ValueError> {
    Ok((|| {
        Vec::new()
    
    })())
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Apply Directory Page", "Verzeichnisseite anwenden")
    }
    fn target(&self) -> Vec<String> {
        vec!["directory".into()]
    }
}
