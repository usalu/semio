//! 📄️ `set-template` — replaces the template block of the container. It builds its own sparse diff and concrete inverse from its payload and reads of `base`.

use super::*;
use crate::schema::snapshot::*;

//#region 🔖️Payload
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
pub struct SetTemplate {
    pub template: DwgTemplate,
}

impl protocol::MutationKind<DwgSnapshot, DwgMutation> for SetTemplate {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "set", entity: "template", kind: "set-template", record: "SetTemplate" };

    fn diff(&self, base: &DwgSnapshot) -> protocol::MutationOutcome<<DwgMutation as Mutation<DwgSnapshot>>::Diff> {
        protocol::MutationOutcome::new(DwgDiff { template: (base.template != self.template).then(|| self.template.clone()), ..DwgDiff::default() })
    }
    fn inverse(&self, base: &DwgSnapshot) -> Result<Vec<DwgMutation>, semio_framework_value::ValueError> {
        Ok((base.template != self.template).then(|| DwgMutation::SetTemplate(set_template::SetTemplate { template: base.template.clone() })).into_iter().collect())
    }
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Set template", "Vorlage setzen")
    }
    fn target(&self) -> Vec<String> {
        Vec::new()
    }
}
//#endregion 🔖️Payload
