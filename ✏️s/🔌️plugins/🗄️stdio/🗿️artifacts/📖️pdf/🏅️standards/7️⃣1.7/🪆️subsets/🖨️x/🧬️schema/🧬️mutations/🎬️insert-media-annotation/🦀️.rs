//! 🎬️ Authoritative PDF/X mutation for inserting a Movie or Sound annotation.

use crate::standards::v1_7::subsets::x::schema::mutations::remove_media_annotation::RemoveMediaAnnotation;
use crate::standards::v1_7::subsets::base::schema::conformance_support as support;
use crate::standards::v1_7::subsets::base::schema::diff::{self, PdfDiff};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertMediaAnnotation {
    pub subtype: String,
    pub title: String,
    #[value(default, skip_serializing_if = "Vec::is_empty")]
    pub placements: Vec<support::ObjectPlacement>,
}

impl<Op> MutationKind<PdfSnapshot, Op> for InsertMediaAnnotation
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<InsertMediaAnnotation> + From<crate::standards::v1_7::subsets::x::schema::mutations::remove_media_annotation::RemoveMediaAnnotation>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "insert", entity: "media-annotation", kind: "insert-media-annotation", record: "Insert" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Insert {} media annotation", self.subtype), &format!("{} Medienanmerkung einfügen", self.subtype))
    }

    fn target(&self) -> Vec<String> {
        vec![self.subtype.clone(), self.title.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for InsertMediaAnnotation
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<InsertMediaAnnotation> + From<crate::standards::v1_7::subsets::x::schema::mutations::remove_media_annotation::RemoveMediaAnnotation>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let (_, rows) = support::insert_object_rows(base, support::media_annotation_object(&self.subtype, &self.title), &self.placements);
        MutationOutcome::new(diff::graph_edit(rows))
    }
    fn inverse(&self, _base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Op::from(RemoveMediaAnnotation { subtype: self.subtype.clone(), title: self.title.clone() })]
    
    })())
}
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
