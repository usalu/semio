//! 👥️ Replaces the whole presence of one collaborator.

use crate::editor::pdf_presence::lane::component::mutations::{PdfPresence, PdfPresenceDiff, PdfPresenceMutation};

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[dsl(keyword = "set-presence")]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetPresence {
    pub page: u32,
    pub pointer_active: bool,
    pub pointer_x: u32,
    pub pointer_y: u32,
    pub view_left: u32,
    pub view_top: u32,
    pub view_right: u32,
    pub view_bottom: u32,
    pub caret_object: String,
    pub caret_offset: u32,
    pub caret_anchor: u32,
}

impl<Op> protocol::MutationKind<PdfPresence, Op> for SetPresence
where
    Op: protocol::Mutation<PdfPresence, Diff = PdfPresenceDiff> + From<SetPresence>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "presence", kind: "set-presence", record: "SetPresence" };

    fn label(&self) -> semio_ui_locale::LocalizedLabel {
        semio_ui_locale::LocalizedLabel::native("Set Presence", "Anwesenheit setzen")
    }
    fn target(&self) -> Vec<String> {
        vec!["page".into()]
    }
}

impl<Op> protocol::ColdMutationKind<PdfPresence, Op> for SetPresence
where
    Op: protocol::ColdMutation<PdfPresence, ReferenceDiff = PdfPresenceDiff> + From<SetPresence>,
{
    fn diff(&self, base: &PdfPresence) -> protocol::MutationOutcome<PdfPresenceDiff> {
        let diff = PdfPresenceDiff {
            page: (base.page != self.page).then_some(self.page),
            pointer_active: (base.pointer_active != self.pointer_active).then_some(self.pointer_active),
            pointer_x: (base.pointer_x != self.pointer_x).then_some(self.pointer_x),
            pointer_y: (base.pointer_y != self.pointer_y).then_some(self.pointer_y),
            view_left: (base.view_left != self.view_left).then_some(self.view_left),
            view_top: (base.view_top != self.view_top).then_some(self.view_top),
            view_right: (base.view_right != self.view_right).then_some(self.view_right),
            view_bottom: (base.view_bottom != self.view_bottom).then_some(self.view_bottom),
            caret_object: (base.caret_object != self.caret_object).then(|| self.caret_object.clone()),
            caret_offset: (base.caret_offset != self.caret_offset).then_some(self.caret_offset),
            caret_anchor: (base.caret_anchor != self.caret_anchor).then_some(self.caret_anchor),
        };
        if protocol::DiffAlgebra::<PdfPresence>::is_empty(&diff) {
            return protocol::MutationOutcome::empty().warning("mutation.no-op", "The presence already holds that state.");
        }
        protocol::MutationOutcome::new(diff)
    }
    fn inverse(&self, base: &PdfPresence) -> Result<Vec<Op>, semio_framework_value::ValueError> {
        Ok(vec![Self {
            page: base.page, pointer_active: base.pointer_active, pointer_x: base.pointer_x, pointer_y: base.pointer_y, view_left: base.view_left, view_top: base.view_top, view_right: base.view_right, view_bottom: base.view_bottom, caret_object: base.caret_object.clone(), caret_offset: base.caret_offset, caret_anchor: base.caret_anchor
        }.into()])
    }
}

protocol::retained_set_record!(SetPresence => PdfPresence, PdfPresenceDiff, PdfPresenceMutation { authored: SetPresence = |leaf| leaf, source_work: 1, patch: |value: SetPresence| (PdfPresenceDiff { page: Some(value.page), pointer_active: Some(value.pointer_active), pointer_x: Some(value.pointer_x), pointer_y: Some(value.pointer_y), view_left: Some(value.view_left), view_top: Some(value.view_top), view_right: Some(value.view_right), view_bottom: Some(value.view_bottom), caret_object: Some(value.caret_object), caret_offset: Some(value.caret_offset), caret_anchor: Some(value.caret_anchor) }, 11), patch_work: || Ok(11), inverse: |root: PdfPresence| (SetPresence { page: root.page, pointer_active: root.pointer_active, pointer_x: root.pointer_x, pointer_y: root.pointer_y, view_left: root.view_left, view_top: root.view_top, view_right: root.view_right, view_bottom: root.view_bottom, caret_object: root.caret_object, caret_offset: root.caret_offset, caret_anchor: root.caret_anchor }.into(), 12), inverse_work: || Ok(12) });
