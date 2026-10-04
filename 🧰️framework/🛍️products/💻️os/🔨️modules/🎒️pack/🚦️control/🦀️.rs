//! 🚦️ Caller-owned finite command admission, cooperative cancellation and native transport progress.
use crate::os_pack::{PackError,PackRefusal,PackRetryDisposition,PackTransportCategory,PackTransportContext,PackTransportPolicy,PackTransportProgress,TransportAdmission,TransportContextRefusalCause};
pub use semio_framework_async::CancelToken;
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::sync::{Arc,Mutex};
/// 📊️ Actual retained input backing has separate finite credit from transport error cells.
#[derive(Clone,Copy,Debug)]
pub struct CommandInputProgress{pub requested_bytes:usize,pub live_bytes:usize,pub cumulative_bytes:usize,pub validated_bytes:usize}

impl InputLedger{
 fn checkpoint(&self)->Result<(),PackError>{if self.cancellation.is_cancelled_now(){Err(input_refusal(ValueRefusalKind::Canceled,"command caller canceled"))}else{Ok(())}}
 fn validated(&self,bytes:usize)->Result<(),PackError>{let event={let state=self.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);CommandInputProgress{requested_bytes:0,live_bytes:state.live,cumulative_bytes:state.cumulative,validated_bytes:bytes}};(self.observer)(event);self.checkpoint()}
}
struct InputState{maximum:usize,live:usize,cumulative:usize}
struct InputLedger{state:Mutex<InputState>,cancellation:CancelToken,observer:Box<dyn Fn(CommandInputProgress)+Send+Sync>}
/// 🪪️ One command retains caller diagnostic transport and separately admitted input backing.
pub struct CommandContext{transport:PackTransportContext,inputs:Arc<InputLedger>}
impl Clone for CommandContext{fn clone(&self)->Self{Self{transport:self.transport.clone(),inputs:self.inputs.clone()}}}
impl CommandContext{
 /// 🎟️ Caller bootstrap storage and the caller token precede the finite retained-input credit.
 pub fn try_new<F:Fn(CommandInputProgress)+Send+Sync+'static>(transport:PackTransportContext,maximum_input_bytes:u64,cancellation:CancelToken,observer:F)->Result<Self,PackError>{let maximum=usize::try_from(maximum_input_bytes).map_err(|_|input_refusal(ValueRefusalKind::OwnershipLimit,"command input credit exceeds address space"))?;if cancellation.is_cancelled_now(){return Err(input_refusal(ValueRefusalKind::Canceled,"command caller canceled"))}Ok(Self{transport,inputs:Arc::new(InputLedger{state:Mutex::new(InputState{maximum,live:0,cumulative:0}),cancellation,observer:Box::new(observer)})})}
 pub fn transport(&self)->&PackTransportContext{&self.transport}
 fn claim(&self,bytes:usize)->Result<InputClaim,PackError>{if self.inputs.cancellation.is_cancelled_now(){return Err(input_refusal(ValueRefusalKind::Canceled,"command caller canceled"))}let event={let mut state=self.inputs.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);let cumulative=state.cumulative.checked_add(bytes).filter(|total|*total<=state.maximum).ok_or_else(||input_refusal(ValueRefusalKind::OwnershipLimit,"command cumulative input credit exceeded"))?;state.cumulative=cumulative;state.live+=bytes;CommandInputProgress{requested_bytes:bytes,live_bytes:state.live,cumulative_bytes:cumulative,validated_bytes:0}};let claim=InputClaim{ledger:self.inputs.clone(),bytes};(self.inputs.observer)(event);if self.inputs.cancellation.is_cancelled_now(){return Err(input_refusal(ValueRefusalKind::Canceled,"command caller canceled"))}Ok(claim)}
 fn checkpoint(&self)->Result<(),PackError>{if self.inputs.cancellation.is_cancelled_now(){Err(input_refusal(ValueRefusalKind::Canceled,"command caller canceled"))}else{Ok(())}}
}
fn input_refusal(kind:ValueRefusalKind,message:&'static str)->PackError{PackError::Refusal(PackRefusal::ValueRefusal(ValueError::new(kind,message)))}
struct InputClaim{ledger:Arc<InputLedger>,bytes:usize}
impl Drop for InputClaim{fn drop(&mut self){let event={let mut state=self.ledger.state.lock().unwrap_or_else(std::sync::PoisonError::into_inner);state.live-=self.bytes;CommandInputProgress{requested_bytes:0,live_bytes:state.live,cumulative_bytes:state.cumulative,validated_bytes:0}};if !std::thread::panicking(){(self.ledger.observer)(event);}}}
/// 📦️ The exact Vec backing stays paid until its actual retained owner drops.
pub struct CommandBytes{bytes:Vec<u8>,claim:InputClaim}
impl std::ops::Deref for CommandBytes{type Target=[u8];fn deref(&self)->&[u8]{&self.bytes}}
impl crate::os_pack::PackSource for CommandBytes{type Error=PackRefusal;async fn len(&self)->u64{self.bytes.len()as u64}async fn read_at(&self,offset:u64,buf:&mut[u8])->Result<usize,PackRefusal>{crate::os_pack::PackSource::read_at(&self.bytes,offset,buf).await}}
impl std::fmt::Debug for CommandBytes{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{f.debug_struct("CommandBytes").field("length",&self.bytes.len()).finish()}}
/// 🔍️ Immutable spans validate every scalar before unchecked transfer of the already verified backing.
impl CommandBytes{pub fn into_text(self)->Result<CommandText,PackError>{
 self.claim.ledger.checkpoint()?;let mut position=0;while position<self.bytes.len(){self.claim.ledger.checkpoint()?;let end=position.saturating_add(65536).min(self.bytes.len());let valid=match std::str::from_utf8(&self.bytes[position..end]){Ok(_)=>end-position,Err(error)if error.error_len().is_none()&&end<self.bytes.len()=>error.valid_up_to(),Err(_)=>return Err(input_refusal(ValueRefusalKind::InvalidValue,"command input is not UTF8"))};if valid==0{return Err(input_refusal(ValueRefusalKind::InvalidValue,"command input is not UTF8"))}self.claim.ledger.validated(valid)?;position+=valid;}self.claim.ledger.checkpoint()?;let text=unsafe{String::from_utf8_unchecked(self.bytes)};Ok(CommandText{text,_claim:self.claim})
}}
/// 📝️ UTF8 conversion transfers the same actual input backing and credit without cloning.
pub struct CommandText{text:String,_claim:InputClaim}
impl std::ops::Deref for CommandText{type Target=str;fn deref(&self)->&str{&self.text}}
impl std::fmt::Display for CommandText{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{self.text.fmt(f)}}
struct CommandPolicy<F>{cancellation:CancelToken,observer:F}
impl<F:Fn(PackTransportProgress)+Send+Sync> PackTransportPolicy for CommandPolicy<F>{
 fn checkpoint(&self,event:PackTransportProgress)->Result<(),ValueError>{if self.cancellation.is_cancelled_now(){return Err(ValueError::new(ValueRefusalKind::Canceled,"command caller canceled"))}(self.observer)(event);Ok(())}
 fn progress(&self,event:PackTransportProgress){(self.observer)(event);}
}
/// 🎛️ Keeps the supplied caller token and observer in the same fallibly admitted context cell.
pub fn admit_command_transport<F:Fn(PackTransportProgress)+Send+Sync+'static>(maximum_transport_bytes:u64,cancellation:CancelToken,observer:F)->Result<PackTransportContext,PackError>{
 let maximum=usize::try_from(maximum_transport_bytes).map_err(|_|PackError::Refusal(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"command transport allowance exceeds address space"))))?;
 PackTransportContext::try_new(TransportAdmission::new(maximum,0),CommandPolicy{cancellation,observer}).map_err(|refusal|match refusal.cause{TransportContextRefusalCause::Admission(refusal)=>PackError::Refusal(PackRefusal::TransportAdmission{category:PackTransportCategory::NativeIo,refusal}),TransportContextRefusalCause::Value(error)=>PackError::Refusal(PackRefusal::ValueRefusal(error))})
}
/// 📥️ Reads through the retained caller context in bounded physical pages.
pub async fn read_command_file(path:&std::path::Path,maximum_bytes:u64,context:&CommandContext)->Result<CommandBytes,PackError>{
 use crate::os_pack::PackSource;
 let source=crate::os_pack::io::FilePackSource::open(path,context.transport.clone())?;let length=source.len().await;
 read_command_source(&source,length,maximum_bytes,context).await
}
/// 📚️ A retained source's trusted prefix uses the same finite input credit and physical pages.
pub async fn read_command_source<S:crate::os_pack::PackSource<Error=PackError>>(source:&S,length:u64,maximum_bytes:u64,context:&CommandContext)->Result<CommandBytes,PackError>{
 if length>maximum_bytes{return Err(PackError::Refusal(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::WorkLimit,"command input exceeds max_file_len"))))}
 let length=usize::try_from(length).map_err(|_|PackError::Refusal(PackRefusal::ValueRefusal(ValueError::new(ValueRefusalKind::OwnershipLimit,"command input exceeds address space"))))?;
 let claim=context.claim(length)?;let mut bytes=Vec::new();bytes.try_reserve_exact(length).map_err(|_|input_refusal(ValueRefusalKind::AllocationFailed,"command input allocation failed"))?;context.checkpoint()?;
 if bytes.capacity()!=length{return Err(input_refusal(ValueRefusalKind::InvariantViolated,"command exact input allocation changed capacity"))}
 while bytes.len()<length{context.checkpoint()?;let offset=bytes.len();let end=offset.saturating_add(4096).min(length);bytes.resize(end,0);source.read_exact_at(offset as u64,&mut bytes[offset..end]).await?;context.checkpoint()?;}Ok(CommandBytes{bytes,claim})
}
/// ⌨️ Process-lifetime stdin input retains its context until EOF, a command or process exit; it is not a library shutdown handle.
pub fn enable_process_stdin_cancellation(context:&PackTransportContext,cancellation:CancelToken)->Result<(),PackError>{
 let input_context=context.clone();context.operation(PackTransportCategory::NativeIo,PackRetryDisposition::Never,0,||{
  std::thread::Builder::new().name("semio-command-cancel".into()).spawn(move||{
   use std::io::Read;
   let stdin=std::io::stdin();let mut input=stdin.lock();let mut command=[0u8;6];let mut length=0usize;
   loop{let mut byte=[0u8;1];let read=input_context.operation(PackTransportCategory::NativeIo,PackRetryDisposition::Never,1,||input.read(&mut byte).map(|count|(count,count)));match read{Ok(0)=>break,Ok(_)=>{if byte[0]==b'\n'{if length==6&&&command==b"cancel"{cancellation.cancel_now();break}length=0;}else if byte[0]!=b'\r'{if length<command.len(){command[length]=byte[0];}length=length.saturating_add(1);}},Err(error)=>{eprintln!("cancel input: {error}");break}}}
  }).map(|handle|{drop(handle);((),0)})
 })
}
/// 🖥️ Native binaries expose actual transport work on stderr without altering their document output.
pub fn print_command_progress(event:PackTransportProgress){if event.phase==crate::os_pack::PackTransportPhase::AfterOperation&&event.bytes>0{eprintln!("Processed {} bytes",event.bytes);}}
/// 🔘 Optional stdin control consumes only its own flag and retains the caller's token and context.
pub fn configure_stdin_cancellation(args:&mut Vec<String>,context:&PackTransportContext,cancellation:CancelToken)->Result<(),PackError>{if let Some(index)=args.iter().position(|arg|arg=="--cancel-stdin"){args.remove(index);eprintln!("Type cancel and press Enter to cancel.");enable_process_stdin_cancellation(context,cancellation)?;}Ok(())}
/// 📈️ Native binary observers expose retained input admission before expensive input work.
pub fn print_command_input_progress(event:CommandInputProgress){if event.requested_bytes>0{eprintln!("Reading {} bytes",event.requested_bytes);}if event.validated_bytes>0{eprintln!("Validated {} bytes",event.validated_bytes);}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
