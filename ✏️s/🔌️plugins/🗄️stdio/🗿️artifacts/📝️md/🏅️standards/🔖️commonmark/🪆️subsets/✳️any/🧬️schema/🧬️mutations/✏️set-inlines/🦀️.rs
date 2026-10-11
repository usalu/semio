//! ✏️ `set-inlines` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetInlines {
    pub(crate) path: Vec<MdPathStep>,
    pub(crate) index: usize,
    pub(crate) inlines: Vec<MdInline>,
}

impl protocol::MutationKind<MdSnapshot, MdMutation> for SetInlines {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "inlines", kind: "set-inlines", record: "SetInlines" };

    fn diff(&self, base: &MdSnapshot) -> protocol::MutationOutcome<<MdMutation as Mutation<MdSnapshot>>::Diff> {
        let Self { path, index, inlines } = self;
        protocol::MutationOutcome::new(match navigate_container(&base.blocks, path).and_then(|c| c.get(*index)) {
            Some(MdBlock::Heading { .. }) => diff_at_path(path, *index, MdBlocksLeafDiff::Modified(MdBlockDiff::Heading { level: None, inlines: Some(inlines.clone()) })),
            Some(MdBlock::Paragraph { .. }) => diff_at_path(path, *index, MdBlocksLeafDiff::Modified(MdBlockDiff::Paragraph { inlines: Some(inlines.clone()) })),
            _ => MdDiff::default(),
        })
    }
    fn inverse(&self, base: &MdSnapshot) -> Result<Vec<MdMutation>, semio_framework_value::ValueError> {
        let Self { path, index, .. } = self;
        Ok({
            let original = match navigate_container(&base.blocks, path).and_then(|c| c.get(*index)) {
                Some(MdBlock::Heading { inlines, .. }) => Some(inlines.clone()),
                Some(MdBlock::Paragraph { inlines }) => Some(inlines.clone()),
                _ => None,
            };
            match original {
                Some(inlines) => vec![MdMutation::SetInlines(set_inlines::SetInlines { path: path.clone(), index: *index, inlines })],
                None => Vec::new(),
            }
        })
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set inlines", "Inline-Elemente setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
