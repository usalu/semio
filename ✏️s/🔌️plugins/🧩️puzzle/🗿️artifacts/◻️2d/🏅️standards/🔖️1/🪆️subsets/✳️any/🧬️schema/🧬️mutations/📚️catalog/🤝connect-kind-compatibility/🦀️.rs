//! 🤝 Puzzle2d mutation — `ConnectKindCompatibility`: allows one kind-id pair to link.

use crate::standards::v1::subsets::any::schema::mutations::disconnect_kind_compatibility::DisconnectKindCompatibility;
use semio_framework_value::paged::PagedUtf8;

use crate::standards::v1::subsets::any::schema::diff::Puzzle2dDiff;
use {crate::standards::v1::subsets::any::schema::Puzzle2dCompatSpecificity, crate::Puzzle2dSnapshot};

//#region 🔖️Mutation
/// 🤝 `connect-kind-compatibility` payload. A duplicate `(source, target)` pair is a no-op.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
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
pub fn connect_kind_compatibility<Op: From<ConnectKindCompatibility>>(source: PagedUtf8<{ usize::MAX }>, target: PagedUtf8<{ usize::MAX }>, bidirectional: bool, important: bool, specificity: Puzzle2dCompatSpecificity, index: Option<usize>) -> Op {
    Op::from(ConnectKindCompatibility { source, target, bidirectional, important, specificity, index })
}

impl<Op> protocol::MutationKind<Puzzle2dSnapshot, Op> for ConnectKindCompatibility
where
    Op: protocol::Mutation<Puzzle2dSnapshot, Diff = Puzzle2dDiff> + From<ConnectKindCompatibility> + From<DisconnectKindCompatibility>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "connect", entity: "kind-compatibility", kind: "connect-kind-compatibility", record: "ConnectedKindCompatibility" };

    fn diff(&self, base: &Puzzle2dSnapshot) -> protocol::MutationOutcome<Puzzle2dDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &Puzzle2dSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
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
