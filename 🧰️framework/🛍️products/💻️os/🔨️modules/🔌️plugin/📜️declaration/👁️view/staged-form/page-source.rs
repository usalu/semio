//! 📄️ Borrowed navigation lends original tokens, exact decimal offsets and resident choice paths.
use super::StagedBindingPathSource;
use semio_framework::action_staging::{StagedFormPageBinding,StagedFormToken,StagedFormTextFormat};
use semio_framework_value::{ValueError,ValueRefusalKind};
#[derive(Clone,Copy)]
pub struct StagedChoicePageView<'a>{pub path:&'a dyn StagedBindingPathSource,pub offset:&'a str}
/// 🧭️ Prepared page controls supply immutable navigation without copying their original token.
pub trait StagedFormPageBindingSource:Send+Sync{
 fn source_identity(&self)->usize{self as *const Self as *const () as usize}
 fn token(&self)->&StagedFormToken;
 fn field_offset(&self)->&str;
 fn diagnostic_offset(&self)->&str;
 fn choice(&self)->Option<StagedChoicePageView<'_>>;
}
impl StagedFormPageBindingSource for StagedFormPageBinding{
 fn token(&self)->&StagedFormToken{&self.token}
 fn field_offset(&self)->&str{&self.page.field_offset}
 fn diagnostic_offset(&self)->&str{&self.page.diagnostic_offset}
 fn choice(&self)->Option<StagedChoicePageView<'_>>{self.page.choice.as_ref().map(|choice|StagedChoicePageView{path:&choice.path,offset:&choice.offset})}
}
/// 🛡️ Owned and borrowed destinations obey the same exact u64 words and closed path rules.
pub fn validate_staged_page_binding_source(source:&dyn StagedFormPageBindingSource)->Result<(),ValueError>{
 let invalid=||ValueError::literal(ValueRefusalKind::InvalidValue,"staging.page");
 if !source.token().admitted()||!StagedFormTextFormat::DecimalU64.admits(source.field_offset())||!StagedFormTextFormat::DecimalU64.admits(source.diagnostic_offset()){return Err(invalid())}
 if let Some(choice)=source.choice(){
  if !StagedFormTextFormat::DecimalU64.admits(choice.offset)||choice.path.len()==0||choice.path.len()>64{return Err(invalid())}
  for index in 0..choice.path.len(){let part=choice.path.segment(index).ok_or_else(invalid)?;if part.is_empty()||matches!(part,"__proto__"|"prototype"|"constructor"){return Err(invalid())}}
 }
 Ok(())
}
