//! 🔧️ Direct rewriting mutation — `ChangeParameterBinding`: upserts one key on the `parameter_bindings` map.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;
use semio_framework_graph_manifest::PropertyValue;

//#region 🔖️Mutation
/// 🔧️ `change-parameter-binding` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-parameter-binding")]
pub struct ChangeParameterBinding {
    pub key: String,
    pub new_value: PropertyValue,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn change_parameter_binding<Op>(key: String, new_value: PropertyValue) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::change_parameter_binding::ChangeParameterBinding>,
{
    Op::from(ChangeParameterBinding { key, new_value })
}

impl<Op> protocol::MutationKind<RewritingSnapshot, Op> for ChangeParameterBinding
where
    Op: protocol::Mutation<RewritingSnapshot, Diff = RewritingDiff> + From<ChangeParameterBinding> + From<crate::standards::v1::subsets::any::schema::mutations::remove_parameter_binding::RemoveParameterBinding>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "parameter-binding", kind: "change-parameter-binding", record: "ChangedParameterBinding" };

    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Change parameter binding \"{}\"", self.key), &format!("Parameterbindung \"{}\" ändern", self.key))
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}
//#endregion 🔖️Mutation
