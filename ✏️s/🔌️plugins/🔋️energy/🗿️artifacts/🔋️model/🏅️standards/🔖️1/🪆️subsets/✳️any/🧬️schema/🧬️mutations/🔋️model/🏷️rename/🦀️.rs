//! 🏷️ Energy model mutation — `RenameModel`: Sets the model's identity field.

use semio_framework_value_derive::{FromValue as FromValueDerive, ToValue as ToValueDerive};

//#region 🔖️Mutation
/// 🏷️ `rename-model` payload. Sets the model's identity field.
#[derive(Clone, Debug, PartialEq, ToValueDerive, FromValueDerive, semio_framework_dsl_record_derive::DslRecord, dsl::MutationLeaf, semio_framework_value::NativeReceive, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol)]
#[value(rename_all = "camelCase")]
#[dsl(keyword = "rename-model")]
pub struct RenameModel {
    pub new_name: String,
}

/// 🏗️ Builder — wraps the payload in whichever dispatch aggregate the caller names.
pub fn rename_model<Op: From<RenameModel>>(new_name: String) -> Op {
    RenameModel { new_name }.into()
}

//#endregion 🔖️Mutation


impl<M> crate::retained::IdentityPayload<M> for RenameModel where M:From<RenameModel> {
 const FIELD:crate::retained::IdentityField=crate::retained::IdentityField::Name;
 const INVALID:protocol::mutation::retained::RetainedDiagnosticSpec=protocol::mutation::retained::RetainedDiagnosticSpec{level:store::__diagnostic::Severity::Fatal,code:"mutation.invariant",message:"An energy model name must not be blank."};
 const UNCHANGED:protocol::mutation::retained::RetainedDiagnosticSpec=protocol::mutation::retained::RetainedDiagnosticSpec{level:store::__diagnostic::Severity::Warning,code:"mutation.no-op",message:"The energy model already has this name."};
 fn source(payload:semio_framework_value::RetainedCloneRef<'_,Self>)->semio_framework_value::RetainedCloneRef<'_,String>{payload.project(0,|p|&p.new_name)}
 fn inverse(value:String)->M{Self{new_name:value}.into()}
}
impl<M> protocol::mutation::retained::RetainedMutationLeaf<crate::snapshot::original::EnergyModelSnapshot,M> for RenameModel
where M:protocol::Mutation<crate::snapshot::original::EnergyModelSnapshot,Diff=crate::diff::original::EnergyModelDiff>+From<RenameModel>+Send+'static {
 type Preparation=crate::retained::IdentityLeafPreparation<Self,M>;
}
impl<M> protocol::MutationKind<crate::snapshot::original::EnergyModelSnapshot,M> for RenameModel
where M:protocol::Mutation<crate::snapshot::original::EnergyModelSnapshot,Diff=crate::diff::original::EnergyModelDiff>+From<RenameModel> {
 const SEMANTICS:protocol::SemanticDescriptor=protocol::SemanticDescriptor{verb:"rename",entity:"model",kind:"rename-model",record:"RenamedModel"};
 fn label(&self)->semio_framework_ui_locale::LocalizedLabel{semio_framework_ui_locale::LocalizedLabel::native("Rename energy model","Energiemodell umbenennen")}
}

impl semio_s_artifact_energy_model_native_mutations::target::ArtifactOperationTargets for RenameModel{fn len(&self)->usize{0}fn segment_len(&self,_:usize)->Result<usize,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"this authored identity operation targets the whole document"))}fn read_segment(&self,_:usize,_:usize,_:&mut[u8])->Result<usize,semio_framework_value::ValueError>{Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"this authored identity operation has no entity target fragments"))}}
