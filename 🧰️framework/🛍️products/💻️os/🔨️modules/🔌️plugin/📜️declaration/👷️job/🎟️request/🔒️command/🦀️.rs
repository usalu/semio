//! 🔒️ The same immutable command and retained OpBinary source stay bound through original producer admission.
use semio_framework::action_bus::{OriginalToolSourceBinding,RetainedToolWireInput};
use semio_framework_job::StepContext;
use semio_framework_value::{ValueError,ValueRefusalKind};

/// 🔒️ Immutable custody of a command whose exact retained encoding was issued by typed ingress.
pub struct ArtifactOriginalToolCommand<C>{
 command:Box<C>,
 encoded_bytes:usize,
 page_count:usize,
 first_page:Option<usize>,
 source_binding:OriginalToolSourceBinding,
}
impl<C> ArtifactOriginalToolCommand<C>{
 pub fn from_admitted_op_binary(command:Box<C>,input:&RetainedToolWireInput,source_binding:OriginalToolSourceBinding)->Self{
  Self{command,encoded_bytes:input.declared_bytes(),page_count:input.page_count(),first_page:input.page(0).map(|page|page.as_ptr()as usize),source_binding}
 }
 /// 📏️ Borrows the exact original encoding extent after validating retained source and native turn authority.
 pub fn admitted_encoded_bytes(&self,input:&RetainedToolWireInput,cx:&StepContext<'_>)->Result<usize,ValueError>{
  if !self.source_binding.matches(cx)||self.encoded_bytes!=input.declared_bytes()||self.page_count!=input.page_count()||self.first_page!=input.page(0).map(|page|page.as_ptr()as usize){
   return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original command encoding source or authority changed"));
  }
  Ok(self.encoded_bytes)
 }
 pub fn into_box(self)->Box<C>{self.command}
}
impl<C> std::ops::Deref for ArtifactOriginalToolCommand<C>{
 type Target=C;
 fn deref(&self)->&C{&self.command}
}
impl<C> AsRef<C> for ArtifactOriginalToolCommand<C>{fn as_ref(&self)->&C{&self.command}}
impl<C:semio_framework_value::retirement::RetireOwned> semio_framework_value::retirement::RetireOwned for ArtifactOriginalToolCommand<C>{
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{self.command.retirement()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.command.retirement_birth_bytes()}
 fn controlled_retirement_supported()->bool{<Box<C> as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()}
}
