//! 🩹️ Own14 path-scoped page edit with frozen snapshot identity.
use super::PdfMutation;
use crate::standards::v1_4::subsets::base::schema::{diff::PdfDiff,snapshot::PdfSnapshot};
use semio_s_artifact_stdio_contract::editing;
use protocol::command::DiffAlgebra;
use protocol::{MutationKind,MutationOutcome,SemanticDescriptor};
#[derive(Clone,Debug,PartialEq,value_derive::ToValue,value_derive::FromValue,dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol,input_schema = Self::input_schema_at_path)]
pub struct PatchSnapshot { pub patch: editing::SnapshotPatch }
impl PatchSnapshot {
    pub fn input_schema_at_path(&self)->Option<&'static str>{Some(editing::snapshot_patch_input_schema("https://json.schemas.assets.semio-tech.com/s/stdio/pdf/1.4/base/snapshot.json",&self.patch).unwrap_or(<Self as protocol::MutationLeaf>::PAYLOAD_SCHEMA))}
    fn next(&self,base:&PdfSnapshot)->Result<PdfSnapshot,editing::SnapshotEditError>{
        static VALIDATOR:std::sync::OnceLock<Result<semio_framework_schema::OwnedJsonSchemaValidator,String>>=std::sync::OnceLock::new();
        let validator=VALIDATOR.get_or_init(||semio_framework_schema::OwnedJsonSchemaValidator::compile(include_str!("../../📸️snapshot/🔣️.json")).map_err(|error|error.to_string())).as_ref().map_err(|error|editing::SnapshotEditError::new("snapshot-edit.invalid-schema-contract","",error.clone()))?;
        let next:PdfSnapshot=editing::patch::apply_validated_snapshot_patch(base,&self.patch,validator)?;
        if next.schema!=base.schema{return Err(editing::SnapshotEditError::new("snapshot-edit.schema-identity","/schema","Snapshot patch cannot change schema identity"))}
        editing::inverse_snapshot_patches(base,&self.patch)?;
        Ok(next)
    }
}
impl MutationKind<PdfSnapshot,PdfMutation> for PatchSnapshot {
    const SEMANTICS:SemanticDescriptor=SemanticDescriptor{verb:"edit",entity:"snapshot",kind:"patch-snapshot",record:"PatchSnapshot"};
    fn diff(&self,base:&PdfSnapshot)->MutationOutcome<PdfDiff>{match self.next(base){Ok(next)=>MutationOutcome::new(PdfDiff::between(base,&next)),Err(error)=>MutationOutcome::refuse(error.outcome_code(),format!("{}: {}",error.code,error.message),[error.path])}}
    fn inverse(&self,base:&PdfSnapshot)->Result<Vec<PdfMutation>,semio_framework_value::ValueError>{
        self.next(base).and_then(|_|editing::inverse_snapshot_patches(base,&self.patch)).map(|parts|parts.into_iter().map(|patch|PdfMutation::PatchSnapshot(Self{patch})).collect()).map_err(|error|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,format!("{}: {}",error.code,error.message)))
    }
    fn label(&self)->semio_framework_ui_locale::LocalizedLabel{editing::snapshot_patch_label(&self.patch)}
    fn target(&self)->Vec<String>{self.patch.target()}
}
