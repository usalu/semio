//! ⛽️ A read-only work view borrows its real caller fuel and fresh readiness predicate.
use semio_framework_value_retained_clone::ValueError;
/// 🔭️ The context keeps cancellation, deadline and retirement authority; this adapter only reads them.
pub struct OriginalJobWorkRecipient<'turn>{fuel:&'turn u64,ready:&'turn mut (dyn FnMut()->bool+Send+'turn)}
impl<'turn> OriginalJobWorkRecipient<'turn>{pub fn new(fuel:&'turn u64,ready:&'turn mut (dyn FnMut()->bool+Send+'turn))->Self{Self{fuel,ready}}}
/// 🫴️ The resident producer; each Job turn lends its read-only fuel and readiness as the context.
pub struct OriginalJobWorkDriver;
impl semio_framework_value_native_owned::native_decoding::OriginalWorkRecipient for OriginalJobWorkDriver{
 type Context<'turn,'source>=OriginalJobWorkRecipient<'turn> where 'source:'turn;
 fn validate_context<'turn,'source>(&self,_:&OriginalJobWorkRecipient<'turn>)->Result<(),ValueError> where 'source:'turn{Ok(())}
 fn source_turn<'turn,'source>(&self,_:&OriginalJobWorkRecipient<'turn>)->Option<semio_framework_value_native_owned::native_decoding::OriginalSourceTurnIdentity> where 'source:'turn{None}
 fn admits<'reply,'turn,'source>(&mut self,c:&mut OriginalJobWorkRecipient<'turn>,quote:semio_framework_value_native_owned::native_decoding::OriginalWorkQuote<'reply>,observed:u64,additional:u64)->semio_framework_value_native_owned::native_decoding::OriginalWorkQuoteComplete<'reply> where 'source:'turn{quote.complete(Ok((c.ready)()&&observed.checked_add(additional).is_some_and(|work|work<=*c.fuel)))}
 fn settle_partial<'reply,'turn,'source>(&mut self,_:&mut OriginalJobWorkRecipient<'turn>,check:semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheck<'reply>)->semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.complete(Ok(()))}
 fn recover_partial<'reply,'turn,'source>(&mut self,_:&mut OriginalJobWorkRecipient<'turn>,check:semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheck<'reply>)->semio_framework_value_native_owned::native_decoding::OriginalWorkPartialCheckComplete<'reply> where 'source:'turn{check.pending()}
}

/// 📬️ Retains the complete recovery call and independent Source posting refusal.
#[derive(Debug)]
pub struct OriginalNativeReceivingOutcome<T=()>{
 pub call:Result<T,semio_framework_value_native_owned::native_decoding::NativeCallFailure<T,ValueError>>,
 pub source_refusal:Option<ValueError>,
}
impl<T> OriginalNativeReceivingOutcome<T>{
 /// 🗨️ Borrows one actual retained cause while preserving every independent field.
 pub fn message(&self)->&str{self.source_refusal.as_ref().or_else(||self.call.as_ref().err().and_then(|failure|failure.operation.as_ref().or(failure.context.as_ref()).or(failure.receipt_refusal.as_ref()))).map_or("original native receiving scope failed",|error|error.message.as_ref())}
 /// 📊️ Returns the original retained physical progress of the first actual refusal.
 pub fn retained_progress(&self)->semio_framework_value_retained_clone::RetainedCloneProgress{self.source_refusal.as_ref().or_else(||self.call.as_ref().err().and_then(|failure|failure.operation.as_ref().or(failure.context.as_ref()).or(failure.receipt_refusal.as_ref()))).map_or(Default::default(),|error|error.retained_progress())}

 /// 🧮️ Quotes both moves of a typed Source output and its separate refusal slot.
 pub fn typed_placement()->Result<semio_framework_value_retained_clone::RetainedCloneProgress,ValueError>{
  let bytes=std::mem::size_of::<Option<Self>>().checked_mul(2).and_then(|n|n.checked_add(std::mem::size_of::<Option<ValueError>>())).ok_or_else(||ValueError::invariant("typed native Source placement overflow"))?;
  Ok(semio_framework_value_retained_clone::RetainedCloneProgress{copied_items:2,copied_bytes:bytes,..Default::default()})
 }
 /// ⛽️ Quotes the typed output movement and fixed Source scope transitions.
 pub fn typed_placement_work()->Result<u64,ValueError>{let p=Self::typed_placement()?;u64::try_from(p.copied_bytes).ok().and_then(|n|n.checked_add(2)).ok_or_else(||ValueError::invariant("typed native Source work overflow"))}
 /// 📏️ Quotes both actual destination placements before recovery enters.
 pub fn placement()->semio_framework_value_retained_clone::RetainedCloneProgress{semio_framework_value_retained_clone::RetainedCloneProgress{copied_items:2,copied_bytes:std::mem::size_of::<Option<Self>>()+std::mem::size_of::<Option<ValueError>>(),..Default::default()}}
}

/// 📨️ Projects the existing caller error policy without changing complete recovery outcomes.
pub fn native_call_result<T,E:From<ValueError>>(result:Result<T,semio_framework_value_native_owned::native_decoding::NativeCallFailure<T,E>>)->Result<T,E>{result.map_err(|failure|failure.operation.unwrap_or_else(||E::from(failure.context.or(failure.receipt_refusal).unwrap_or_else(||ValueError::invariant("original native call failed without an error")))))}

impl<T:semio_framework_value_retained_clone::retirement::RetireOwned> semio_framework_value_retained_clone::retirement::RetireOwned for OriginalNativeReceivingOutcome<T>{
 type RetirementCursor=<(Result<T,semio_framework_value_native_owned::native_decoding::NativeCallFailure<T,ValueError>>,Option<ValueError>)as semio_framework_value_retained_clone::retirement::RetireOwned>::RetirementCursor;
 fn retirement_ready(&self)->bool{use semio_framework_value_retained_clone::retirement::RetireOwned;self.call.retirement_ready()&&self.source_refusal.retirement_ready()}
 fn retirement_seed(self)->Self::RetirementCursor{semio_framework_value_retained_clone::retirement::RetireOwned::retirement_seed((self.call,self.source_refusal))}
 fn controlled_retirement_supported()->bool{semio_framework_value_retained_clone::retirement::support::controlled_retirement_supported::<Self>()}
 fn controlled_retirement_supported_in(path:&semio_framework_value_retained_clone::retirement::RetirementSupport<'_>)->bool{path.supports::<Result<T,semio_framework_value_native_owned::native_decoding::NativeCallFailure<T,ValueError>>>()&&path.supports::<Option<ValueError>>()}
}

/// 🚪️ The Source selects one authority purpose before lending its actual decoder.
pub enum OriginalReceivingOperation<'operation,T=()>{
 Receive(&'operation mut dyn FnMut(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,semio_framework_value_retained_clone::RetainedCloneGrant)->Result<T,ValueError>),
 Recover(&'operation mut dyn FnMut(&mut semio_framework_value_native_owned::native_decoding::NativeRetirementDecodeControl<'_, '_>,semio_framework_value_retained_clone::RetainedCloneGrant)->Result<T,ValueError>),
 RecoverValue(&'operation mut dyn FnMut(&mut semio_framework_value_native_owned::native_decoding::NativeRetirementDecodeControl<'_, '_>,semio_framework_value_retained_clone::RetainedCloneGrant)->Result<T,ValueError>),
}
impl<T> OriginalReceivingOperation<'_,T>{pub fn is_typed(&self)->bool{matches!(self,Self::RecoverValue(_))}pub fn is_retirement(&self)->bool{matches!(self,Self::Recover(_)|Self::RecoverValue(_))}}
