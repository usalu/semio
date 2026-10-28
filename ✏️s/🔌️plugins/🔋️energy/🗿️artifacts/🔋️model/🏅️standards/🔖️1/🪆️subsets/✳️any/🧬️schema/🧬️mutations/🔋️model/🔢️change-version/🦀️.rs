//! 🔢️ Energy model mutation — `ChangeModelVersion`: Sets the model document's own version string.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🔢️ `change-model-version` payload. Sets the model document's own version string.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "change-model-version")]
pub struct ChangeModelVersion {
    pub new_version: String,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn change_model_version<Op: From<ChangeModelVersion>>(new_version: String) -> Op {
    ChangeModelVersion { new_version }.into()
}

//#endregion 🔖️Mutation


impl<M> crate::retained::IdentityPayload<M> for ChangeModelVersion where M:From<ChangeModelVersion> {
 const FIELD:crate::retained::IdentityField=crate::retained::IdentityField::Version;
 const INVALID:protocol::mutation::retained::RetainedDiagnosticSpec=protocol::mutation::retained::RetainedDiagnosticSpec{level:store::__diagnostic::Severity::Fatal,code:"mutation.invariant",message:"An energy model version must not be blank."};
 const UNCHANGED:protocol::mutation::retained::RetainedDiagnosticSpec=protocol::mutation::retained::RetainedDiagnosticSpec{level:store::__diagnostic::Severity::Warning,code:"mutation.no-op",message:"The energy model already has this version."};
 fn source(payload:semio_framework_value::RetainedCloneRef<'_,Self>)->semio_framework_value::RetainedCloneRef<'_,String>{payload.project(0,|p|&p.new_version)}
 fn inverse(value:String)->M{Self{new_version:value}.into()}
}
impl<M> protocol::mutation::retained::RetainedMutationLeaf<crate::snapshot::original::EnergyModelSnapshot,M> for ChangeModelVersion
where M:protocol::Mutation<crate::snapshot::original::EnergyModelSnapshot,Diff=crate::diff::original::EnergyModelDiff>+From<ChangeModelVersion>+Send+'static {
 type Preparation=crate::retained::IdentityLeafPreparation<Self,M>;
}
impl<M> protocol::MutationKind<crate::snapshot::original::EnergyModelSnapshot,M> for ChangeModelVersion
where M:protocol::Mutation<crate::snapshot::original::EnergyModelSnapshot,Diff=crate::diff::original::EnergyModelDiff>+From<ChangeModelVersion> {
 const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor{verb:"change",entity:"model",kind:"change-model-version",record:"ChangedModelVersion"};
 fn label(&self)->semio_framework_ui_locale::LocalizedLabel{semio_framework_ui_locale::LocalizedLabel::native("Change energy model version","Energiemodellversion ändern")}
}

impl semio_s_artifact_energy_model_native_mutations::target::ArtifactOperationTargets for ChangeModelVersion{fn len(&self)->usize{0}fn segment_len(&self,_:usize)->Result<usize,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"this authored identity operation targets the whole document"))}fn read_segment(&self,_:usize,_:usize,_:&mut[u8])->Result<usize,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"this authored identity operation has no entity target fragments"))}}
