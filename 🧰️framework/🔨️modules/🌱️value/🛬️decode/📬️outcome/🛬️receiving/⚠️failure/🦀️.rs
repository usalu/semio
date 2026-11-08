//! 🌉️ Native receiving retains the complete original effect owner at every Nested boundary.
use crate::{ValueError,OriginalReceivingPrimary,OriginalRetirementFailure,RetainedCloneProgress,NativeReceivingEffectError,NativeReceivingEffectFailure,retirement::RetireOwned};
/// 🧳️ Owns the exact selected error domain without overwriting any independent receiving cause.
#[derive(Debug)]
pub enum NativeReceivingFailure{Primary(OriginalReceivingPrimary),ValueEffect(NativeReceivingEffectFailure<ValueError>),RetirementEffect(NativeReceivingEffectFailure<OriginalRetirementFailure>),ReceivingEffect(NativeReceivingEffectFailure<OriginalReceivingPrimary>)}
impl NativeReceivingFailure{
 /// 🧾️ Preserves the original enclosing action's actual physical span.
 pub fn retained_progress(&self)->RetainedCloneProgress{match self{Self::Primary(error)=>error.retained_progress(),Self::ValueEffect(error)=>error.retained_progress(),Self::RetirementEffect(error)=>error.retained_progress(),Self::ReceivingEffect(error)=>error.retained_progress()}}
}
impl From<ValueError> for NativeReceivingFailure{fn from(error:ValueError)->Self{Self::Primary(error.into())}}
impl From<OriginalReceivingPrimary> for NativeReceivingFailure{fn from(error:OriginalReceivingPrimary)->Self{Self::Primary(error)}}
impl From<OriginalRetirementFailure> for NativeReceivingFailure{fn from(error:OriginalRetirementFailure)->Self{Self::Primary(error.into())}}
impl From<NativeReceivingEffectFailure<ValueError>> for NativeReceivingFailure{fn from(error:NativeReceivingEffectFailure<ValueError>)->Self{Self::ValueEffect(error)}}
impl From<NativeReceivingEffectFailure<OriginalRetirementFailure>> for NativeReceivingFailure{fn from(error:NativeReceivingEffectFailure<OriginalRetirementFailure>)->Self{Self::RetirementEffect(error)}}
impl From<NativeReceivingEffectFailure<OriginalReceivingPrimary>> for NativeReceivingFailure{fn from(error:NativeReceivingEffectFailure<OriginalReceivingPrimary>)->Self{Self::ReceivingEffect(error)}}
impl NativeReceivingEffectError for NativeReceivingFailure{fn receiving_progress(&self)->RetainedCloneProgress{self.retained_progress()}}
impl RetireOwned for NativeReceivingFailure{
 fn retirement_ready(&self)->bool{match self{Self::Primary(error)=>error.retirement_ready(),Self::ValueEffect(error)=>error.retirement_ready(),Self::RetirementEffect(error)=>error.retirement_ready(),Self::ReceivingEffect(error)=>error.retirement_ready()}}
 type RetirementCursor=<(Option<OriginalReceivingPrimary>,Option<NativeReceivingEffectFailure<ValueError>>,Option<NativeReceivingEffectFailure<OriginalRetirementFailure>>,Option<NativeReceivingEffectFailure<OriginalReceivingPrimary>>)as RetireOwned>::RetirementCursor;
 fn retirement_seed(self)->Self::RetirementCursor{let values=match self{Self::Primary(error)=>(Some(error),None,None,None),Self::ValueEffect(error)=>(None,Some(error),None,None),Self::RetirementEffect(error)=>(None,None,Some(error),None),Self::ReceivingEffect(error)=>(None,None,None,Some(error))};values.retirement_seed()}
 fn retirement_birth_bytes(&self)->Option<usize>{self.retirement_ready().then_some(size_of::<Self::RetirementCursor>())}
 fn controlled_retirement_supported()->bool{true}
}
impl std::fmt::Display for NativeReceivingFailure{
 fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{
  Self::Primary(error)=>f.write_str(error.message()),
  Self::ValueEffect(error)=>match error.child.as_ref().or(error.producer_refusal.as_ref()).or(error.work_refusal.as_ref()){Some(cause)=>std::fmt::Display::fmt(cause,f),None=>f.write_str(error.receiving.message())},
  Self::RetirementEffect(error)=>if let Some(cause)=error.child.as_ref(){std::fmt::Display::fmt(cause,f)}else if let Some(cause)=error.producer_refusal.as_ref().or(error.work_refusal.as_ref()){std::fmt::Display::fmt(cause,f)}else{f.write_str(error.receiving.message())},
  Self::ReceivingEffect(error)=>if let Some(cause)=error.child.as_ref(){f.write_str(cause.message())}else if let Some(cause)=error.producer_refusal.as_ref().or(error.work_refusal.as_ref()){std::fmt::Display::fmt(cause,f)}else{f.write_str(error.receiving.message())}
 }}
}
impl std::error::Error for NativeReceivingFailure{}
