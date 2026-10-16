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
