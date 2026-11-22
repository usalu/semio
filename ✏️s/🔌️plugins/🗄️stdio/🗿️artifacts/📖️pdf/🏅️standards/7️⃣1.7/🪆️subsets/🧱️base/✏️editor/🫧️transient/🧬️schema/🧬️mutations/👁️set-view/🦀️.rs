//! 👁️ Replaces the whole page view of one editor instance.

use crate::editor::pdf_view::transient::component::mutations::{PdfViewTransient, PdfViewTransientDiff, PdfViewTransientMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "set-view")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetView {
    pub zoom_mode: String,
    pub zoom_permille: u32,
    pub page: u32,
    pub revision: u32,
    pub query: String,
    pub match_case: bool,
    pub active: Option<u32>,
}

impl<Op> protocol::MutationKind<PdfViewTransient, Op> for SetView
where
    Op: protocol::Mutation<PdfViewTransient, Diff = PdfViewTransientDiff> + From<SetView>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "view", kind: "set-view", record: "SetView" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Set View", "Ansicht setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["zoomMode".into()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfViewTransient, Op> for SetView
where
    Op: protocol::ColdMutation<PdfViewTransient, ReferenceDiff = PdfViewTransientDiff> + From<SetView>,
{
    fn diff(&self, base: &PdfViewTransient) -> protocol::MutationOutcome<PdfViewTransientDiff> {
        let diff = PdfViewTransientDiff {
            zoom_mode: (base.zoom_mode != self.zoom_mode).then(|| self.zoom_mode.clone()),
            zoom_permille: (base.zoom_permille != self.zoom_permille).then_some(self.zoom_permille),
            page: (base.page != self.page).then_some(self.page),
            revision: (base.revision != self.revision).then_some(self.revision),
            query: (base.query != self.query).then(|| self.query.clone()),
            match_case: (base.match_case != self.match_case).then_some(self.match_case),
            active: (base.active != self.active).then_some(self.active),
        };
        if protocol::DiffAlgebra::<PdfViewTransient>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The view already holds that state.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &PdfViewTransient) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(vec![Self { zoom_mode: base.zoom_mode.clone(), zoom_permille: base.zoom_permille, page: base.page, revision: base.revision, query: base.query.clone(), match_case: base.match_case, active: base.active }.into()])
    }
}

protocol::retained_set_record!(SetView => PdfViewTransient, PdfViewTransientDiff, PdfViewTransientMutation { authored: SetView = |leaf| leaf, source_work: 1, patch: |value: SetView| (PdfViewTransientDiff { zoom_mode: Some(value.zoom_mode), zoom_permille: Some(value.zoom_permille), page: Some(value.page), revision: Some(value.revision), query: Some(value.query), match_case: Some(value.match_case), active: Some(value.active) }, 7), patch_work: || Ok(7), inverse: |root: PdfViewTransient| (SetView { zoom_mode: root.zoom_mode, zoom_permille: root.zoom_permille, page: root.page, revision: root.revision, query: root.query, match_case: root.match_case, active: root.active }.into(), 8), inverse_work: || Ok(8) });
