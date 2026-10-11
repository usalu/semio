//! 🎚️ `set-format` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetFormat {
    pub(crate) format: PlyFormat,
}

impl protocol::MutationKind<PlySnapshot, PlyMutation> for SetFormat {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "format", kind: "set-format", record: "SetFormat" };

    fn diff(&self, base: &PlySnapshot) -> protocol::MutationOutcome<<PlyMutation as Mutation<PlySnapshot>>::Diff> {
        let Self { format } = self;
        protocol::MutationOutcome::new(diff_set_format(*format))
    }
    fn inverse(&self, base: &PlySnapshot) -> Result<Vec<PlyMutation>, semio_framework_value::ValueError> {
        Ok(vec![PlyMutation::SetFormat(set_format::SetFormat { format: base.format })])
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set format", "Format setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
