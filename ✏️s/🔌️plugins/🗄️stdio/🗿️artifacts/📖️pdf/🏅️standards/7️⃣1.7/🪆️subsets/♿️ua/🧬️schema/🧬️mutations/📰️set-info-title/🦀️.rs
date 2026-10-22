//! 🏷️ Authoritative PDF/UA mutation for setting the document title conformance axis.

use crate::standards::v1_7::subsets::base::schema::diff::{PdfDiff, PdfInfoDiff, PdfSet};
use crate::standards::v1_7::subsets::base::schema::snapshot::PdfSnapshot;
use protocol::{MutationKind, MutationOutcome, SemanticDescriptor};

//#region 🔖️Mutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetInfoTitle {
    pub title: String,
}

impl<Op> MutationKind<PdfSnapshot, Op> for SetInfoTitle
where
    Op: protocol::Mutation<PdfSnapshot, Diff = PdfDiff> + From<SetInfoTitle>,
{
    const SEMANTICS: SemanticDescriptor = SemanticDescriptor { verb: "set", entity: "info-title", kind: "set-info-title", record: "Set" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Set PDF/UA title \"{}\"", self.title), &format!("PDF/UA-Titel \"{}\" setzen", self.title))
    }

    fn target(&self) -> Vec<String> {
        vec!["Info.Title".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfSnapshot, Op> for SetInfoTitle
where
    Op: protocol::ColdMutation<PdfSnapshot, ReferenceDiff = PdfDiff> + From<SetInfoTitle>,
{
    fn diff(&self, base: &PdfSnapshot) -> MutationOutcome<PdfDiff> {
        let wanted = (!self.title.is_empty()).then(|| self.title.clone());
        let change = (base.info.title != wanted).then(|| PdfInfoDiff { title: Some(PdfSet::from_option(&wanted)), ..Default::default() });
        MutationOutcome::new(PdfDiff { info: change, ..Default::default() })
    }
    fn inverse(&self, base: &PdfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![Op::from(SetInfoTitle { title: base.info.title.clone().unwrap_or_default() })]
    
    })())
}
}
//#endregion 🔖️Mutation

//#region 🧪️Tests

//#endregion 🧪️Tests

//#region 🔖️Facets
//#endregion 🔖️Facets
