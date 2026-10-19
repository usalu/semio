//! 🎟️ Native numerical laws keep one actual lower caller, decoder and finite source stock.
use super::*;
use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId};
pub const STOCK:RetainedCloneGrant=RetainedCloneGrant{maximum_items:1<<28,maximum_copy_bytes:1<<30,maximum_capacity_bytes:1<<30,maximum_release_bytes:1<<30,maximum_depth:4096};
pub struct OriginalEnergyCaller{caller:OriginalJobFixtureCaller,decoder:OriginalJobDecodeOwner}
impl OriginalEnergyCaller{
 pub fn new(operation:OperationId,generation:Generation)->Self{Self{caller:OriginalJobFixtureCaller::admit(operation,generation,STOCK,default_now_us,u64::MAX,1<<28),decoder:OriginalJobDecodeOwner::empty()}}
 pub fn cancel(&self){self.caller.cancel().cancel_now();}
 pub fn turn<R>(&mut self,grant:RetainedCloneGrant,fuel:u64,deadline:u64,run:impl FnOnce(&mut StepContext<'_>)->Result<R,OriginalTurnFailure>)->Result<R,OriginalTurnFailure>{self.caller.turn_with_decoder(grant,&mut self.decoder,|cx|{let mut output=None;cx.with_original_work_loan(fuel,deadline,|cx|{output=Some(run(cx)?);Ok(())})?;Ok(output.unwrap())})}
 pub fn close_job(&mut self,job:&mut EnergyJob){job.begin_close();for _ in 0..1_000_000{if job.terminal_is_empty(){return}self.turn(ENERGY_BATCH_GRANT,64,u64::MAX,|cx|cx.with_original_retirement(|cx|job.close_step(cx))).expect("same original Energy numerical close");}panic!("finite Energy numerical close remained pending")}
 pub fn close_packet(&mut self,packet:&mut EnergyWirePacket){for _ in 0..1_000_000{if packet.terminal_is_empty(){return}self.turn(ENERGY_BATCH_GRANT,64,u64::MAX,|cx|{let demand=packet.retirement_demands()?;semio_framework_job::close_original_retained_step(cx,demand,|loan|packet.close_step(loan))?;Ok(())}).expect("same original Energy packet close");}panic!("finite Energy packet close remained pending")}
 pub fn acknowledge(&mut self,job:&mut EnergyJob,lease:&mut Option<EnergyWireLease>,kind:EnergyWireKind){let copied_bytes=std::mem::size_of::<Option<EnergyWireLease>>();for _ in 0..4096{if lease.is_none(){return}self.turn(ENERGY_BATCH_GRANT,1,u64::MAX,|cx|{if cx.retained_grant().maximum_copy_bytes<copied_bytes||cx.retained_grant().maximum_items==0||cx.should_yield(){return Ok(())}let original=lease.take().unwrap();let result=match kind{EnergyWireKind::Checkpoint=>job.ack_checkpoint_packet(original),EnergyWireKind::Commit=>job.ack_commit_packet(original),EnergyWireKind::Fault=>job.ack_fault_packet(original),_=>unreachable!()};if let Err(original)=result{*lease=Some(original);return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"native ACK retained same rejected lease").into())}cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes,..Default::default()})?;Ok(())}).expect("native original ACK keeps its exact lease on refusal");}panic!("native original ACK remained pending")}
 pub fn finish(mut self){for _ in 0..4096{if self.decoder.terminal_is_empty(){break}self.caller.turn(STOCK,|cx|{let demand=self.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|loan|self.decoder.close_step(loan))?;Ok(())}).expect("same original Energy decoder close");}assert!(self.decoder.terminal_is_empty());self.caller.retire(STOCK);}
}
/// 🧪️ Explicit native fixture input is preborn; the actual batch receiver and numerical steps use its persistent original source.
pub fn run_preborn(model:Model,config:SimulationConfig)->Result<Results,Error>{run_preborn_job(Engine::job(model,config).unwrap_or_else(|_|panic!("native numerical fixture input exceeds its authored admission policy")))}
pub fn run_preborn_job(job:EnergyJob)->Result<Results,Error>{
 let mut original=OriginalEnergyCaller::new(job.operation.operation,job.operation.generation);let mut candidate=Some(job);let mut batch=None;
 for _ in 0..4096{if original.turn(ENERGY_BATCH_GRANT,64,u64::MAX,|cx|EnergyBatchRun::admit(&mut candidate,&mut batch,cx)).expect("native batch preserves actual original slots and complete admission failure"){break}}

 let mut batch=batch.unwrap();let mut result=None;let mut terminal=None;
 for _ in 0..10_000_000{let seen=original.turn(ENERGY_BATCH_GRANT,32,u64::MAX,|cx|batch.step(cx)).expect("native batch original turn preserves whole failure");if matches!(seen,EnergyRun::Complete|EnergyRun::Fault|EnergyRun::Cancelled){terminal=Some(seen);break}}
 let terminal=terminal.expect("native batch reaches a terminal outcome under finite original stock");if terminal==EnergyRun::Complete{for _ in 0..4096{if original.turn(ENERGY_BATCH_GRANT,64,u64::MAX,|cx|batch.take_results(&mut result,cx)).expect("native batch original result transfer"){break}}}
 batch.begin_close();for _ in 0..10_000_000{if batch.terminal_is_empty(){break}original.turn(ENERGY_BATCH_GRANT,64,u64::MAX,|cx|cx.with_original_retirement(|cx|batch.close_step(cx))).expect("native batch same source close");}assert!(batch.terminal_is_empty());drop(batch);original.finish();
 match terminal{EnergyRun::Complete=>result.ok_or_else(||Error::severe("native batch completed without its original typed result")),EnergyRun::Cancelled=>Err(Error::severe("native batch cancelled")),_=>Err(Error::severe("native batch numerical fault"))}
}

/// 🏭️ A native law prepares its explicit cold input before admitting the actual execution caller.
pub fn run_preborn_factory(prepare:impl FnOnce(Operation)->EnergyJob)->Result<Results,Error>{run_preborn_job(prepare(Operation::new(allocate_operation_id(),RevisionId(0),Generation(0),0)))}
