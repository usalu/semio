//! 🏷️ `set-solid-name` — authored as its own mutation leaf. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;

//#region 🔖️Payload
/// 🏷️ Sets the `solid`/`endsolid` header/trailer name.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
pub struct SetSolidName {
    pub(crate) name: String,
}

impl protocol::MutationKind<StlSnapshot, StlMutation> for SetSolidName {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "solid-name", kind: "set-solid-name", record: "SetSolidName" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set solid name", "Körpername setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}

impl protocol::ColdMutationKind<StlSnapshot, StlMutation> for SetSolidName {
    fn diff(&self, _base: &StlSnapshot) -> protocol::MutationOutcome<<StlMutation as protocol::ColdMutation<StlSnapshot>>::ReferenceDiff> {
        let Self { name } = self;
        protocol::MutationOutcome::new(diff::diff_set_solid_name(name))
    }
    fn inverse(&self, base: &StlSnapshot) -> Result<Vec<StlMutation>, semio_framework_value::ValueError> {
        Ok(vec![StlMutation::SetSolidName(SetSolidName { name: base.solid_name.clone() })])
    }
}
//#endregion 🔖️Payload
