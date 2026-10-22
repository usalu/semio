//! 🧹️ Direct rewriting mutation — `RemoveParameterBinding`: takes one key out of the `parameter_bindings`
//! map.
use crate::standards::v1::subsets::any::schema::diff::RewritingDiff;
use crate::standards::v1::subsets::any::schema::snapshot::RewritingSnapshot;

//#region 🔖️Mutation
/// 🧹️ `remove-parameter-binding` payload.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "remove-parameter-binding")]
pub struct RemoveParameterBinding {
    pub key: String,
}

/// 🏗️ Builder — wraps the payload in its dispatch variant.
pub fn remove_parameter_binding<Op>(key: String) -> Op
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::remove_parameter_binding::RemoveParameterBinding>,
{
    Op::from(RemoveParameterBinding { key })
}

impl<Op> protocol::MutationKind<RewritingSnapshot, Op> for RemoveParameterBinding
where
    Op: protocol::Mutation<RewritingSnapshot, Diff = RewritingDiff> + From<RemoveParameterBinding> + From<crate::standards::v1::subsets::any::schema::mutations::change_parameter_binding::ChangeParameterBinding>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "remove", entity: "parameter-binding", kind: "remove-parameter-binding", record: "RemovedParameterBinding" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native(&format!("Remove parameter binding \"{}\"", self.key), &format!("Parameterbindung \"{}\" entfernen", self.key))
    }
    fn target(&self) -> Vec<String> {
        vec![self.key.clone()]
    }
}

impl<Op> protocol::ColdMutationKind<RewritingSnapshot, Op> for RemoveParameterBinding
where
    Op: protocol::ColdMutation<RewritingSnapshot, ReferenceDiff = RewritingDiff> + From<RemoveParameterBinding> + From<crate::standards::v1::subsets::any::schema::mutations::change_parameter_binding::ChangeParameterBinding>,
{
    fn diff(&self, base: &RewritingSnapshot) -> protocol::MutationOutcome<RewritingDiff> {
        super::diff::diff(self, base)
    }
    fn inverse(&self, base: &RewritingSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
    Ok({
        super::inverse::inverse(self, base)?
    
    })
}
}
//#endregion 🔖️Mutation
