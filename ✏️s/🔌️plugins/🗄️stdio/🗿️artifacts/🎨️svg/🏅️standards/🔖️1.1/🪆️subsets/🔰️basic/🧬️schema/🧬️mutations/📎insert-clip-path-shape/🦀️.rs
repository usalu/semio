//! 📎️ `insert-clip-path-shape` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct InsertClipPathShape {
    pub(crate) clip_path_id: String,
    pub(crate) index: usize,
    pub(crate) node: SvgNode,
}

impl protocol::MutationKind<SvgSnapshot, SvgBasicMutation> for InsertClipPathShape {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "insert", entity: "clip-path-shape", kind: "insert-clip-path-shape", record: "InsertClipPathShape" };

    fn diff(&self, base: &SvgSnapshot) -> protocol::MutationOutcome<SvgDiff> {
        let Self { clip_path_id, index, node } = self;
        {
            if carries_text(node) {
                return protocol::MutationOutcome::error(CODE_REJECTED, "the inserted shape carries a text element -- SVG Basic 1.1 forbids clipping to text".to_string(), Vec::<String>::new());
            }
            if let Some(message) = subtree_profile_violation(node) {
                return protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new());
            }
            match resolve_clip_path(base, clip_path_id) {
                Ok(target) => protocol::MutationOutcome::new(insert_child_diff(&target, *index, node)),
                Err(message) => protocol::MutationOutcome::error(CODE_REJECTED, message, Vec::<String>::new()),
            }
        }
    }
    fn inverse(&self, base: &SvgSnapshot) -> Result<Vec<SvgBasicMutation>, semio_framework_value::ValueError> {
        let Self { clip_path_id, index, .. } = self;
        Ok(match path_of_id(base, clip_path_id) {
            Some(target) => vec![SvgBasicMutation::RemoveElement(remove_element::RemoveElement { parent: target, index: *index })],
            None => Vec::new(),
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Insert clip path shape", "Form des Beschneidungspfads einfügen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
