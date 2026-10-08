//! 📎 `set-parts` — replaces the unmodeled package files (`project.bcfp`, ...) the archive retains verbatim. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetParts {
    pub(crate) parts: Vec<BcfRawPart>,
}

impl protocol::MutationKind<BcfSnapshot, BcfMutation> for SetParts {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "parts", kind: "set-parts", record: "SetParts" };

    fn diff(&self, base: &BcfSnapshot) -> protocol::MutationOutcome<<BcfMutation as Mutation<BcfSnapshot>>::Diff> {
        match base.parts == self.parts {
            true => protocol::MutationOutcome::new(BcfDiff::default()),
            false => protocol::MutationOutcome::new(BcfDiff { version: None, topics: None, parts: Some(BcfPartsDiff { removed: (0..base.parts.len()).collect(), modified: Vec::new(), added: self.parts.iter().enumerate().map(|(index, item)| IndexedAdded { index, item: item.clone() }).collect() }) }),
        }
    }
    fn inverse(&self, base: &BcfSnapshot) -> Result<Vec<BcfMutation>, semio_framework_value::ValueError> {
        Ok((base.parts != self.parts).then(|| BcfMutation::SetParts(set_parts::SetParts { parts: base.parts.clone() })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set package parts", "Paketdateien setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
