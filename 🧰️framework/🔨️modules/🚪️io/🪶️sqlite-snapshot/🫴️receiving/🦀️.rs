//! 🫴️ Concrete SQLite receiving preserves the original generic owner and physical settlement.
use semio_framework_io_control::{NativeSnapshotBodyWallet,NativeSnapshotDecodeOwner,NativeSnapshotEncodeOwner};
use semio_framework_value::{NativeEncodeControl, RetainedCloneGrant, retained_clone::RetainedCloneProgress, ValueError, ValueRefusalKind};
use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
/// 🎞️ A domain frame keeps its complete fallible status in original receiving custody.
pub trait NativeSqliteReceivingFrame:semio_framework_value::retirement::RetireOwned{
 fn receiving_status(&mut self)->&mut Option<Result<(),ValueError>>;
}
macro_rules! original_sqlite_receiving{
 ($trait:ident,$owner:ident,$direction:ident,$progress:path,$phase:ident)=>{
 /// 🪶️ Loans concrete SQL receiving with original work, body and refusal custody.
 pub trait $trait{
  fn receive_sqlite_snapshot<F:NativeSqliteReceivingFrame,O:semio_framework_value::retirement::RetireOwned>(&mut self,control:&mut crate::SqliteSnapshotControl<'_>,construct:&mut impl FnMut()->F,operation:&mut impl FnMut(&mut F,&mut crate::artifact::receiving::Port<'_,'_,'_,NativeSnapshotBodyWallet>)->Result<(),ValueError>,publish:&mut impl FnMut(&mut F)->Result<O,ValueError>)->semio_framework_value::RetainedWorkOutcome<O,semio_framework_io_control::NativeSnapshotFailure>;
 }
 impl $trait for $owner<'_,'_>{
  fn receive_sqlite_snapshot<F:NativeSqliteReceivingFrame,O:semio_framework_value::retirement::RetireOwned>(&mut self,control:&mut crate::SqliteSnapshotControl<'_>,construct:&mut impl FnMut()->F,operation:&mut impl FnMut(&mut F,&mut crate::artifact::receiving::Port<'_,'_,'_,NativeSnapshotBodyWallet>)->Result<(),ValueError>,publish:&mut impl FnMut(&mut F)->Result<O,ValueError>)->semio_framework_value::RetainedWorkOutcome<O,semio_framework_io_control::NativeSnapshotFailure>{
   use semio_framework_io_control::{NativeSnapshotFailure,native_snapshot_turn};
   native_snapshot_turn(|work|{
    self.native().checkpoint()?;let before=self.native().owned_bytes();
    let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"original SQLite receiving extent overflow"))?;
    let settlement=crate::artifact::receiving::ReceivingSettlement::new(control,before,crate::SqliteSnapshotPhase::$phase);
    let mut observer=|event:$progress|settlement.observe(event.owned_bytes,event.completed,event.total);
    let mut published=None;let turn=self.scoped_native(maximum,&mut observer,|owner|owner.receive::<F,O,NativeSnapshotFailure>(&mut published,&mut |slot,output,_marker,_failure,native,body|{
     let before_work=settlement.with_control(|control|control.performed_work());
     let result=(||{
      let owned=settlement.with_control(|control|{
       let mut port=crate::artifact::receiving::Port{control,native:crate::artifact::receiving::Direction::$direction(native),body};
       port.work(1,std::mem::size_of::<F>())?;*slot=Some(construct());
       let frame=slot.as_mut().ok_or_else(||ValueError::invariant("original SQL receiving frame absent"))?;let status=operation(frame,&mut port);*frame.receiving_status()=Some(status);
       Ok::<_,ValueError>(port.owned())
      })?;
      let status=slot.as_mut().ok_or_else(||ValueError::invariant("original SQL receiving frame absent"))?.receiving_status().take().ok_or_else(||ValueError::invariant("original SQL receiving status absent"))?;
      let settled=settlement.finish_body(owned);
      match(status,settled){
       (Err(primary),Err(receipt))=>return Err(NativeSnapshotFailure::from(primary).with_settlement_refusal(receipt).unwrap_or_else(|(failure,_)|failure)),
       (Err(primary),Ok(()))=>return Err(primary.into()),
       (Ok(()),Err(receipt))=>return Err(NativeSnapshotFailure::from(receipt)),
       (Ok(()),Ok(()))=>{}
      }
      native.checkpoint()?;
      let value=settlement.with_control(|control|{
       let mut port=crate::artifact::receiving::Port{control,native:crate::artifact::receiving::Direction::$direction(native),body};
       port.work(1,std::mem::size_of::<O>())?;publish(slot.as_mut().ok_or_else(||ValueError::invariant("original SQL receiving frame absent"))?)
      }).map_err(NativeSnapshotFailure::from)?;
      *output=Some(value);Ok(())
     })();
     semio_framework_value::RetainedWorkOutcome{result,work:u128::from(settlement.with_control(|control|control.performed_work()).saturating_sub(before_work))}
    }));
    let turn=semio_framework_value::RetainedWorkOutcome{result:turn.result.map_err(|error|settlement.refused_failure(self.native().owned_bytes(),error)).and_then(|()|published.take().ok_or_else(||NativeSnapshotFailure::from(ValueError::literal(ValueRefusalKind::InvariantViolated,"original SQL receiving published no output")))),work:turn.work};
    work.take(turn)
   })
  }
 }
 };
}
original_sqlite_receiving!(SqliteSnapshotDecodeReceiving,NativeSnapshotDecodeOwner,Decode,semio_framework_value_native_owned::native_decoding::NativeDecodeProgress,ProjectSnapshot);
original_sqlite_receiving!(SqliteSnapshotEncodeReceiving,NativeSnapshotEncodeOwner,Encode,semio_framework_value::native_encoding::NativeEncodeProgress,ReconstructSnapshot);
impl crate::artifact::receiving::ReceivingWallet for NativeSnapshotBodyWallet{

 fn admit_frontier(&self,demand:RetainedCloneGrant)->Result<(),ValueError>{NativeSnapshotBodyWallet::admit_frontier(self,demand)}
 fn record_progress(&mut self,progress:RetainedCloneProgress)->Result<(),ValueError>{NativeSnapshotBodyWallet::record_progress(self,progress)}
 fn allocate_vec_into<T>(&mut self,native:&mut NativeDecodeControl<'_>,count:usize,output:&mut Vec<T>,depth:usize)->Result<(),ValueError>{NativeSnapshotBodyWallet::allocate_vec_into(self,native,count,output,depth)}
 fn allocate_encode_vec_into<T>(&mut self,native:&mut NativeEncodeControl<'_>,count:usize,output:&mut Vec<T>,depth:usize)->Result<(),ValueError>{NativeSnapshotBodyWallet::allocate_encode_vec_into(self,native,count,output,depth)}
 fn copy_text_into(&mut self,native:&mut NativeDecodeControl<'_>,text:&str,output:&mut String,depth:usize)->Result<(),ValueError>{NativeSnapshotBodyWallet::copy_text_into(self,native,text,output,depth)}
 fn copy_encode_text_into(&mut self,native:&mut NativeEncodeControl<'_>,text:&str,output:&mut String,depth:usize)->Result<(),ValueError>{NativeSnapshotBodyWallet::copy_encode_text_into(self,native,text,output,depth)}
}
