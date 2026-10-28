//! 🛬️ A declared Window vocabulary erases its real Pack receiver without substituting a cold decoder.
use super::*;
use semio_framework_value_native_owned::native_decoding::{NativeDecodeControl,NativeDecodeCloseControl,NativeConstructionStepOutcome,OriginalReceivingPrimary};
use semio_framework_os_store_codec_registry::native_snapshot::{OriginalArtifactPairReceiving,OriginalArtifactPairFailure,OriginalArtifactPackReceiving};
pub type WindowConfigHistorySource=store::os_spr::history::OriginalHistorySource;
pub trait WindowConfigNativePairReceiving<P>:Send{
 fn demands(&self,pack:&[u8],spr:&[u8],native:&dyn NativeDecodeCloseControl,body:usize)->Result<RetirementDemand,OriginalArtifactPairFailure>;
 fn step(&mut self,pack:&[u8],spr:&[u8],native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,OriginalArtifactPairFailure>;
 fn is_ready(&self)->bool;
 fn pack_digest(&self)->Option<[u8;32]>;
 fn admit_history_source(&mut self,pack:&[u8],spr:&[u8],output:&mut Option<WindowConfigHistorySource>,native:&mut NativeDecodeControl<'_>)->NativeConstructionStepOutcome<bool,OriginalReceivingPrimary>;
 fn take_genesis_into(&mut self,pack:&[u8],spr:&[u8],output:&mut Option<P>,native:&dyn NativeDecodeCloseControl,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>;
 fn close_demands(&self,pack:&[u8],spr:&[u8],native:&dyn NativeDecodeCloseControl,body:usize)->Result<RetirementDemand,OriginalArtifactPairFailure>;
 fn close_step(&mut self,pack:&[u8],spr:&[u8],native:&mut dyn NativeDecodeCloseControl,grant:RetainedCloneGrant)->Result<RetainedCloneStep,OriginalArtifactPairFailure>;
 fn terminal_is_empty(&self)->bool;
 fn allocation_bytes(&self)->usize;
}
impl<P:OriginalArtifactPackReceiving> WindowConfigNativePairReceiving<P> for OriginalArtifactPairReceiving<P>{
 fn demands(&self,p:&[u8],s:&[u8],n:&dyn NativeDecodeCloseControl,b:usize)->Result<RetirementDemand,OriginalArtifactPairFailure>{self.demands(p,s,n,b)}
 fn step(&mut self,p:&[u8],s:&[u8],n:&mut NativeDecodeControl<'_>,b:usize,g:RetainedCloneGrant)->Result<RetainedCloneProgress,OriginalArtifactPairFailure>{self.step(p,s,n,b,g)}
 fn is_ready(&self)->bool{self.is_ready()}fn pack_digest(&self)->Option<[u8;32]>{self.pack_digest()}
 fn admit_history_source(&mut self,p:&[u8],s:&[u8],o:&mut Option<WindowConfigHistorySource>,n:&mut NativeDecodeControl<'_>)->NativeConstructionStepOutcome<bool,OriginalReceivingPrimary>{self.admit_history_source(p,s,o,n)}
 fn take_genesis_into(&mut self,p:&[u8],s:&[u8],o:&mut Option<P>,n:&dyn NativeDecodeCloseControl,g:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{self.take_genesis_into(p,s,o,n,g)}
 fn close_demands(&self,p:&[u8],s:&[u8],n:&dyn NativeDecodeCloseControl,b:usize)->Result<RetirementDemand,OriginalArtifactPairFailure>{self.close_demands(p,s,n,b)}
 fn close_step(&mut self,p:&[u8],s:&[u8],n:&mut dyn NativeDecodeCloseControl,g:RetainedCloneGrant)->Result<RetainedCloneStep,OriginalArtifactPairFailure>{self.close_step(p,s,n,g)}
 fn terminal_is_empty(&self)->bool{self.terminal_is_empty()}fn allocation_bytes(&self)->usize{std::mem::size_of::<Self>()}
}
/// 📏️ The vocabulary quotes its actual concrete receiver Box before the original mounted caller admits it.
pub fn window_config_native_pair_demands<P:OriginalArtifactPackReceiving>()->RetirementDemand{{let mut d=OriginalArtifactPairReceiving::<P>::boxed_birth_demands();d.copy_bytes+=std::mem::size_of::<Box<dyn WindowConfigNativePairReceiving<P>>>();d}}
/// 📥️ Both original input allocations stay borrowed by the one real native pair, including every refused receipt.
pub fn admit_window_config_native_pair<P:OriginalArtifactPackReceiving+'static>(pack:&[u8],spr:&[u8],pack_limits:protocol::PackLimits,history_limits:semio_framework_os_store_codec_registry::native_snapshot::OriginalArtifactHistoryReadLimits,native:&mut NativeDecodeControl<'_>,grant:RetainedCloneGrant,output:&mut Option<Box<dyn WindowConfigNativePairReceiving<P>>>)->Result<RetainedCloneProgress,OriginalArtifactPairFailure>{let d=window_config_native_pair_demands::<P>();if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,d){return Ok(Default::default())}if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original Window Pack receiver destination occupied").into())}let Some((receiver,_))=OriginalArtifactPairReceiving::admit_boxed(pack,spr,pack_limits,history_limits,native,grant)?else{return Ok(Default::default())};*output=Some(receiver);let progress=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,..Default::default()};native.record_constructed_allocation(d.capacity_bytes).map_err(|error|OriginalArtifactPairFailure::from(error.with_retained_progress(progress)))?;Ok(progress)}
