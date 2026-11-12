//! ⚖️ Settles the original receiving allocation of a typed snapshot file against the native owned bytes while its body is written or read, and keeps the first refusal cause.
use semio_framework_io_sqlite_snapshot::{SqliteSnapshotControl, SqliteSnapshotPhase};
use semio_framework_value::ValueError;

/// ⚖️ Typed snapshot file settlement over the receiving scope of one control.
pub struct TypedFileSettlement<'a,'b>{pub control:std::cell::RefCell<semio_framework_io_sqlite_snapshot::SqliteSnapshotReceivingControl<'a,'b>>,pub before:usize,pub remaining:usize,pub phase:SqliteSnapshotPhase,post_body:std::cell::Cell<bool>,pub cause:std::cell::RefCell<Option<ValueError>>}
impl<'a,'b> TypedFileSettlement<'a,'b>{
    pub fn new(control:&'a mut SqliteSnapshotControl<'b>,before:usize,phase:SqliteSnapshotPhase)->Self{let remaining=control.allocation_remaining_bytes();Self{control:std::cell::RefCell::new(control.receiving_scope()),before,remaining,phase,post_body:std::cell::Cell::new(false),cause:std::cell::RefCell::new(None)}}
    pub fn settle(&self,owned:usize)->Result<(),ValueError>{semio_framework_io_sqlite_snapshot::artifact::receiving::settle(&mut self.control.borrow_mut(),self.remaining,self.before,owned)}
    pub fn finish_body(&self,owned:usize)->Result<(),ValueError>{self.settle(owned)?;self.post_body.set(true);Ok(())}
    pub fn observe(&self,owned:usize,completed:usize,total:usize)->bool{if !self.post_body.get(){return true}let result=self.settle(owned).and_then(|_|self.control.borrow_mut().checkpoint(self.phase,completed,total));match result{Ok(())=>true,Err(error)=>{*self.cause.borrow_mut()=Some(error);false}}}
    pub fn refused(&self,owned:usize,error:ValueError)->ValueError{match self.settle(owned){Err(cause)=>cause,Ok(())=>self.cause.borrow_mut().take().unwrap_or(error)}}
}

/// 🫴️ Receives one typed original snapshot built by a cumulative native decoder, settling the SQL allocation ceiling against the owned bytes and keeping the first refusal.
pub fn decode_typed<T:semio_framework_value::retirement::RetireOwned>(control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_io_control::NativeSnapshotDecodeOwner<'_,'_>,output:&mut Option<crate::lifted_serialization::SnapshotReceivingOutput<T>>,construct:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<T,ValueError>)->crate::lifted_serialization::SnapshotReceivingOutcome{
    use semio_framework_io_sqlite_snapshot::SqliteStageFailure as Stage;
    use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,ValueRefusalKind};
    use semio_framework_value_native_owned::native_decoding::NativeDecodeProgress;
    let admitted=(||->Result<(usize,usize),ValueError>{
        if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original snapshot output slot is occupied"))}
        native_owner.native().checkpoint()?;control.checkpoint(SqliteSnapshotPhase::DecodeNative,0,0)?;
        let before=native_owner.native().owned_bytes();let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native snapshot allowance overflow"))?;Ok((before,maximum))
    })();
    let(before,maximum)=match admitted{Ok(admitted)=>admitted,Err(error)=>return RetainedWorkOutcome{result:Err(Stage::NotStarted(error)),work:0}};
    let settlement=TypedFileSettlement::new(control,before,SqliteSnapshotPhase::DecodeNative);
    let mut observer=|event:NativeDecodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
    let mut construct=Some(construct);
    let turn=native_owner.scoped_native(maximum,&mut observer,|owner|owner.receive::<T,crate::lifted_serialization::SnapshotReceivingOutput<T>,semio_framework_io_control::NativeSnapshotFailure>(output,&mut |slot,published,_marker,_unwind,native,body|{
        let before_work=native.actual_work();
        let operation=(||->Result<T,ValueError>{
            let value=construct.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"typed snapshot construction was already consumed"))?(native)?;
            body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<T>(),maximum_depth:1,..Default::default()})?;
            *slot=Some(value);
            body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<T>(),..Default::default()})?;
            slot.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"typed snapshot receiving slot was lost"))
        })().map_err(semio_framework_io_control::NativeSnapshotFailure::from);
        let settled=settlement.finish_body(native.owned_bytes());
        *published=Some(match settled{Ok(())=>operation.map_err(Stage::Operation),Err(refusal)=>Err(Stage::Settlement{operation,refusal,receipt:None})});
        RetainedWorkOutcome{result:Ok(()),work:native.actual_work().checked_sub(before_work).expect("original native work is monotone")}
    }));
    let result=match settlement.cause.borrow_mut().take(){Some(refusal)=>Err(Stage::Settlement{operation:turn.result,refusal,receipt:None}),None=>turn.result.map_err(Stage::Operation)};
    RetainedWorkOutcome{result,work:turn.work}
}

/// 🫴️ Receives one typed original payload emitted by a cumulative native encoder, settling the SQL allocation ceiling against the owned bytes and keeping the first refusal.
pub fn encode_typed(control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_io_control::NativeSnapshotEncodeOwner<'_,'_>,output:&mut Option<crate::lifted_serialization::SnapshotReceivingOutput<semio_framework_io_schema::IoPayload>>,construct:impl FnOnce(&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_io_schema::IoPayload,ValueError>)->crate::lifted_serialization::SnapshotReceivingOutcome{
    use semio_framework_io_schema::IoPayload;
    use semio_framework_io_sqlite_snapshot::SqliteStageFailure as Stage;
    use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedWorkOutcome,ValueRefusalKind,native_encoding::NativeEncodeProgress};
    let admitted=(||->Result<(usize,usize),ValueError>{
        if output.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"original snapshot output slot is occupied"))}
        native_owner.native().checkpoint()?;control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;
        let before=native_owner.native().owned_bytes();let maximum=before.checked_add(control.allocation_remaining_bytes()).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native snapshot allowance overflow"))?;Ok((before,maximum))
    })();
    let(before,maximum)=match admitted{Ok(admitted)=>admitted,Err(error)=>return RetainedWorkOutcome{result:Err(Stage::NotStarted(error)),work:0}};
    let settlement=TypedFileSettlement::new(control,before,SqliteSnapshotPhase::EncodeNative);
    let mut observer=|event:NativeEncodeProgress|settlement.observe(event.owned_bytes,event.completed,event.total);
    let mut construct=Some(construct);
    let turn=native_owner.scoped_native(maximum,&mut observer,|owner|owner.receive::<IoPayload,crate::lifted_serialization::SnapshotReceivingOutput<IoPayload>,semio_framework_io_control::NativeSnapshotFailure>(output,&mut |slot,published,_marker,_unwind,native,body|{
        let before_work=native.actual_work();
        let operation=(||->Result<IoPayload,ValueError>{
            let payload=construct.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"typed payload construction was already consumed"))?(native)?;
            let capacity=match &payload{IoPayload::Binary(bytes)=>bytes.capacity(),IoPayload::Text(text)=>text.capacity()};
            body.admit_frontier(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:size_of::<IoPayload>(),maximum_capacity_bytes:capacity,maximum_release_bytes:0,maximum_depth:1})?;
            *slot=Some(payload);
            body.record_progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<IoPayload>(),retained_capacity_bytes:capacity,released_bytes:0})?;
            slot.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"typed payload receiving slot was lost"))
        })().map_err(semio_framework_io_control::NativeSnapshotFailure::from);
        let settled=settlement.finish_body(native.owned_bytes());
        *published=Some(match settled{Ok(())=>operation.map_err(Stage::Operation),Err(refusal)=>Err(Stage::Settlement{operation,refusal,receipt:None})});
        RetainedWorkOutcome{result:Ok(()),work:native.actual_work().checked_sub(before_work).expect("original native work is monotone")}
    }));
    let result=match settlement.cause.borrow_mut().take(){Some(refusal)=>Err(Stage::Settlement{operation:turn.result,refusal,receipt:None}),None=>turn.result.map_err(Stage::Operation)};
    RetainedWorkOutcome{result,work:turn.work}
}
