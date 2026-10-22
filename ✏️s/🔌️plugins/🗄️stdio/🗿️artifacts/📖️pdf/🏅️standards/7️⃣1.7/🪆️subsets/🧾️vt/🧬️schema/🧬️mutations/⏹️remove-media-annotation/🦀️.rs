//! ⏹️ Authoritative PDF/VT mutation for removing a matching Movie or Sound annotation.

use crate::standards::v1_7::subsets::vt::schema::mutations::insert_media_annotation::InsertMediaAnnotation;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct RemoveMediaAnnotation {
    pub subtype: String,
    pub title: String,
}

impl<Op> MutationKind<PdfSnapshot, Op> for RemoveMediaAnnotation
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<RemoveMediaAnnotation> + From<crate::standards::v1_7::subsets::vt::schema::mutations::insert_media_annotation::InsertMediaAnnotation>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "remove", entity: "media-annotation", kind: "remove-media-annotation", record: "Remove" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove {} media annotation", self.subtype), &format!("{} Medienanmerkung entfernen", self.subtype))
    }

    fn target(&self) -> Vec<String> {
        vec![self.subtype.clone(), self.title.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for RemoveMediaAnnotation
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<RemoveMediaAnnotation> + From<crate::standards::v1_7::subsets::vt::schema::mutations::insert_media_annotation::InsertMediaAnnotation>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let rows = support::media_annotation(base, &self.subtype, &self.title).map_or_else(PdfDiff::default, |id| support::remove_object_rows(base, id));
        MutationOutcome::new(diff::graph_edit(rows))
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok({
            support::media_annotation(base, &self.subtype, &self.title).map(|id| Op::from(InsertMediaAnnotation { subtype: self.subtype.clone(), title: self.title.clone(), placements: support::placements_of(base, &[id]) })).into_iter().collect()
        })
    }
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
