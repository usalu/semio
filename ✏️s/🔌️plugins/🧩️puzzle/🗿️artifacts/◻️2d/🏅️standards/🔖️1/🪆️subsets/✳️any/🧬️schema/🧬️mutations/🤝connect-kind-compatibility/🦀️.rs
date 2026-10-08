//! 🤝 Puzzle2d mutation — `ConnectKindCompatibility`: allows one kind-id pair to link.

use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
use crate::{Puzzle2dCompatSpecificity, Puzzle2dSnapshot};

//#region 🔖️Mutation
/// 🤝 `connect-kind-compatibility` payload. A duplicate `(source, target)` pair is a no-op.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[dsl(keyword = "connect-kind-compatibility")]
pub struct ConnectKindCompatibility {
    pub source: PagedUtf8<{ usize::MAX }>,
    pub target: PagedUtf8<{ usize::MAX }>,
    pub bidirectional: bool,
    pub important: bool,
    pub specificity: Puzzle2dCompatSpecificity,
    pub index: Option<usize>,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn connect_kind_compatibility(source: PagedUtf8<{ usize::MAX }>, target: PagedUtf8<{ usize::MAX }>, bidirectional: bool, important: bool, specificity: Puzzle2dCompatSpecificity, index: Option<usize>) -> Puzzle2dMutation {
    Puzzle2dMutation::ConnectKindCompatibility(ConnectKindCompatibility { source, target, bidirectional, important, specificity, index })
}

impl protocol::MutationKind<Puzzle2dSnapshot, Puzzle2dMutation> for ConnectKindCompatibility {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "kind-compatibility", kind: "connect-kind-compatibility", record: "ConnectedKindCompatibility" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Puzzle2dMutation>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Connect kind compatibility \"{}\" -> \"{}\"", self.source, self.target), &format!("Artkompatibilität \"{}\" -> \"{}\" herstellen", self.source, self.target))
    }
    fn target(&self) -> Vec<String> {
        vec![self.source.to_string_owner(), self.target.to_string_owner()]
    }
}
//#endregion 🔖️Mutation
