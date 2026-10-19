//! 📮️ Publishes original Energy owners through common funded in-place Job pages.
use super::*;
#[path="📶️prepared/🦀️.rs"]
mod prepared;
use prepared::{EnergySurfacePreparation,PreparedSurfacePayload};
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn permits(grant:RetainedCloneGrant,demand:RetirementDemand)->bool{grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth}
fn enter(cx:&mut StepContext<'_>,demand:RetirementDemand)->Result<bool,ValueError>{if cx.is_cancelled()||!permits(cx.retained_grant(),demand){return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes})?;Ok(true)}
struct OriginalChunk<'a>(&'a[u8]);
impl JobPayloadStagedSource for OriginalChunk<'_>{fn byte_len(&self)->usize{self.0.len()}fn copy_byte_demand(&self)->Result<usize,JobPayloadAdmissionFault>{Ok(self.0.len())}fn depth_demand(&self)->Result<usize,JobPayloadAdmissionFault>{Ok(1)}fn write_bytes(&self,target:&mut[MaybeUninit<u8>])->Result<(),JobPayloadAdmissionFault>{for(slot,byte)in target.iter_mut().zip(self.0){slot.write(*byte);}Ok(())}}

pub(super) struct OriginalSurfacePayload<'a>(pub(super)&'a crate::results::SurfaceEnergyTable);
impl ToolRunTickPayloadSource for OriginalSurfacePayload<'_>{
 fn update(&self)->semio_framework_tool_run::ToolRunPayloadUpdate{semio_framework_tool_run::ToolRunPayloadUpdate::Replace}
 fn original_identity(&self)->usize{self.0 as*const _ as usize}
 fn byte_len(&self)->usize{energy_surface_payload_bytes(self.0.len().min(ENERGY_SURFACE_PAYLOAD_MAXIMUM_ROWS))}
 fn range_demands(&self,offset:usize,count:usize)->Result<RetirementDemand,ValueError>{if offset.checked_add(count).is_none_or(|end|end>self.byte_len()){return Err(invalid("original surface payload range exceeds native owner"))}let mut cursor=offset;let end=offset+count;let mut copy=count;while cursor<end{let position=cursor%4;let length=(4-position).min(end-cursor);let native=if cursor<8||((cursor-8)/4)%5==0{4}else{size_of::<f64>()+size_of::<crate::model::EntityId>()};copy=copy.checked_add(native+4+size_of::<usize>()).ok_or_else(||invalid("original surface copy extent overflow"))?;cursor+=length;}Ok(RetirementDemand{copy_bytes:copy,depth:if end>8{3}else{1},..Default::default()})}
 fn write_range(&self,offset:usize,target:&mut[u8])->Result<(),ValueError>{self.range_demands(offset,target.len())?;let mut written=0;while written<target.len(){let cursor=offset+written;let position=cursor%4;let word=if cursor<4{ENERGY_SURFACE_PAYLOAD_MAGIC}else if cursor<8{(self.0.len().min(ENERGY_SURFACE_PAYLOAD_MAXIMUM_ROWS)as u32).to_le_bytes()}else{let row=(cursor-8)/ENERGY_SURFACE_PAYLOAD_ROW_BYTES;let column=((cursor-8)%ENERGY_SURFACE_PAYLOAD_ROW_BYTES)/4;let(id,energy)=self.0.publication_entry(row).ok_or_else(||invalid("original surface row disappeared"))?;if column==0{id.0.to_le_bytes()}else{let joules=match column{1=>energy.conduction_loss_j,2=>energy.conduction_gain_j,3=>energy.solar_transmitted_j,_=>energy.solar_absorbed_j};((joules/3_600_000.0)as f32).to_le_bytes()}};let count=(4-position).min(target.len()-written);target[written..written+count].copy_from_slice(&word[position..position+count]);written+=count;}Ok(())}
}

pub(super) struct EnergyTickPublication{
 source:ControlledRetirement<ToolRunTickWriter>,
 prepared:EnergySurfacePreparation,
 publication:u64,
 offset:usize,
 builder:Option<Box<RetainedPayloadBuilder>>,
 wire:Option<ToolRunTickWireCursor>,
 counters:[u64;5],
 counter_index:usize,
 progress_phase:u8,
 surface_requested:bool,
 surface_probed:bool,
 surface_selected:bool,
 surface_index:usize,
 ack_due:bool,
 pub(super) delivered:bool,
 empty:bool,
}
impl EnergyTickPublication{
 fn birth_demands()->RetirementDemand{RetirementDemand{copy_bytes:2*size_of::<Self>()+2*size_of::<ToolRunProgress>()+size_of::<Option<ToolRunTickWriter>>()+size_of::<Option<Box<Self>>>(),capacity_bytes:size_of::<Self>(),depth:1,..Default::default()}}
 pub(super) fn admit(writer:&mut Option<ToolRunTickWriter>,cursor:&EnergyRunCursor,identity:ToolRunIdentity,state:ToolRunState,stage:EnergySimulationRunStage,surface_requested:bool,cx:&mut StepContext<'_>)->Result<Option<Box<Self>>,ValueError>{let demand=Self::birth_demands();if !enter(cx,demand)?{return Ok(None)}let publication=writer.as_ref().ok_or_else(||invalid("original Energy writer is absent"))?.original_sequence().checked_add(1).ok_or_else(||invalid("original Energy publication identity overflow"))?;let mut original=writer.take().unwrap();let progress=ToolRunProgress{identity,sequence:0,state,stage:stage.index(),completed:cursor.completed(),total:(stage.index()>=EnergySimulationRunStage::Run.index()).then(||cursor.warmup_timesteps+u64::from(cursor.total_timesteps)),counters:Vec::new(),units_per_second:0.0,conflicts:0,steps:ToolRunStepRing::new()};original.progress(progress);let source=ControlledRetirement::new(original).map_err(|(error,_)|error)?;Ok(Some(Box::new(Self{source,prepared:EnergySurfacePreparation::empty(),publication,offset:0,builder:None,wire:None,counters:[cursor.warmup_timesteps,cursor.run_timesteps,cursor.tiers_published,(cursor.facility_electricity_kwh*1_000.0).round().max(0.0)as u64,cursor.input_identity_bytes],counter_index:0,progress_phase:0,surface_requested,surface_probed:!surface_requested,surface_selected:false,surface_index:0,ack_due:false,delivered:false,empty:false})))}
 fn poll_demands(&self,table:Option<&crate::results::SurfaceEnergyTable>)->Result<RetirementDemand,ValueError>{
 if self.progress_phase==0{return Ok(RetirementDemand{copy_bytes:size_of::<Vec<ToolRunCounter>>()+size_of::<u8>(),capacity_bytes:5*size_of::<ToolRunCounter>(),depth:2,..Default::default()})}
 if self.counter_index<5{return Ok(RetirementDemand{copy_bytes:size_of::<u64>()+size_of::<ToolRunCounter>()+2*size_of::<usize>(),depth:2,..Default::default()})}
 if self.progress_phase==1{return Ok(RetirementDemand{copy_bytes:size_of::<u8>(),depth:2,..Default::default()})}
 if !self.surface_probed{return Ok(RetirementDemand{copy_bytes:4*size_of::<f64>()+size_of::<usize>()+2*size_of::<bool>(),depth:3,..Default::default()})}
 if self.surface_selected&&!self.prepared.complete(){return self.prepared.demand(table.ok_or_else(||invalid("original selected Energy table is absent"))?)}
 let Some(builder)=self.builder.as_ref()else{return Ok(RetainedPayloadBuilder::boxed_birth_demands())};if !builder.is_initialized(){return Ok(builder.initialization_demand())}
 let Some(wire)=self.wire.as_ref()else{return Ok(ToolRunTickWireCursor::birth_demands())};
 if self.ack_due{let mut demand=wire.acknowledge_demands();demand.copy_bytes+=size_of::<bool>();return Ok(demand)}
 if !wire.chunk().is_empty(){if builder.staged_page_len().is_none(){return builder.staged_page_birth_demands()}if builder.staged_page_remaining()<wire.chunk().len(){return builder.staged_commit_demands()}return builder.staged_source_demands(&OriginalChunk(wire.chunk()),size_of::<bool>())}
 if !wire.is_complete(){let payload=table.filter(|_|self.surface_selected).map(|table|PreparedSurfacePayload{table,prepared:&self.prepared,publication:self.publication,offset:self.offset});return wire.advance_demands(self.source.original().ok_or_else(||invalid("original Energy tick is absent"))?,payload.as_ref().map(|p|p as&dyn ToolRunTickPayloadSource))}
 if builder.staged_page_len().is_some(){return builder.staged_commit_demands()}
 Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})
 }
 pub(super) fn poll<'a>(&'a mut self,cx:&mut StepContext<'_>,table:Option<&crate::results::SurfaceEnergyTable>)->Result<Option<JobOutcomeBorrow<'a>>,ValueError>{
 if cx.is_cancelled(){return Ok(None)}
 if self.progress_phase==0{let demand=RetirementDemand{copy_bytes:size_of::<Vec<ToolRunCounter>>()+size_of::<u8>(),capacity_bytes:5*size_of::<ToolRunCounter>(),depth:2,..Default::default()};if !enter(cx,demand)?{return Ok(None)}self.source.original_mut().unwrap().original_progress_mut().unwrap().counters.try_reserve_exact(5).map_err(|_|invalid("original progress counter allocation refused"))?;self.progress_phase=1;return Ok(None)}
 if self.counter_index<5{let demand=RetirementDemand{copy_bytes:size_of::<u64>()+size_of::<ToolRunCounter>()+2*size_of::<usize>(),depth:2,..Default::default()};if !enter(cx,demand)?{return Ok(None)}let index=self.counter_index;self.source.original_mut().unwrap().original_progress_mut().unwrap().counters.push(ToolRunCounter{counter:index as u16,value:self.counters[index]});self.counter_index+=1;return Ok(None)}
 if self.progress_phase==1{let demand=RetirementDemand{copy_bytes:size_of::<u8>(),depth:2,..Default::default()};if !enter(cx,demand)?{return Ok(None)}self.progress_phase=2;return Ok(None)}
 if !self.surface_probed{let demand=RetirementDemand{copy_bytes:4*size_of::<f64>()+size_of::<usize>()+2*size_of::<bool>(),depth:3,..Default::default()};if !enter(cx,demand)?{return Ok(None)}if let Some((_,energy))=table.and_then(|t|t.publication_entry(self.surface_index)){self.surface_selected=(energy.conduction_loss_j!=0.0)|(energy.conduction_gain_j!=0.0)|(energy.solar_transmitted_j!=0.0)|(energy.solar_absorbed_j!=0.0);self.surface_index+=1;self.surface_probed=self.surface_selected||table.is_none_or(|t|self.surface_index>=t.len());}else{self.surface_probed=true;}return Ok(None)}
 if self.surface_selected&&!self.prepared.complete(){let table=table.ok_or_else(||invalid("original selected Energy table is absent"))?;let demand=self.prepared.demand(table)?;if !permits(cx.retained_grant(),demand){return Ok(None)}let progress=self.prepared.advance(table,cx.retained_grant())?;cx.consume_retained(progress)?;return Ok(None)}
 if self.builder.is_none(){self.builder=RetainedPayloadBuilder::admit_boxed(JobPayloadStream::Preview,cx)?;return Ok(None)}
 let builder=self.builder.as_mut().unwrap();if !builder.is_initialized(){builder.advance_initialization(cx)?;return Ok(None)}
 let payload=table.filter(|_|self.surface_selected).map(|table|PreparedSurfacePayload{table,prepared:&self.prepared,publication:self.publication,offset:self.offset});let port=payload.as_ref().map(|p|p as&dyn ToolRunTickPayloadSource);let source=self.source.original().ok_or_else(||invalid("original tick source already retiring"))?;
 if self.wire.is_none(){if let Some((wire,receipt))=ToolRunTickWireCursor::admit(source,port,cx.retained_grant())?{cx.consume_retained(receipt)?;self.wire=Some(wire);}return Ok(None)}
 let wire=self.wire.as_mut().unwrap();
 if self.ack_due{let mut demand=wire.acknowledge_demands();demand.copy_bytes+=size_of::<bool>();if !permits(cx.retained_grant(),demand){return Ok(None)}let mut receipt=wire.acknowledge_chunk(cx.retained_grant());receipt.copied_bytes+=size_of::<bool>();self.ack_due=false;cx.consume_retained(receipt)?;return Ok(None)}
 if !wire.chunk().is_empty(){if builder.staged_page_len().is_none(){builder.begin_staged_page_funded(cx)?;return Ok(None)}if builder.staged_page_remaining()<wire.chunk().len(){builder.commit_staged_page_funded(cx)?;return Ok(None)}if builder.write_staged_source_funded(cx,&OriginalChunk(wire.chunk()),size_of::<bool>())?{self.ack_due=true;}return Ok(None)}
 if !wire.is_complete(){let receipt=wire.advance(source,port,cx.retained_grant())?;cx.consume_retained(receipt)?;return Ok(None)}
 if builder.staged_page_len().is_some(){builder.commit_staged_page_funded(cx)?;return Ok(None)}
 if builder.published().is_none(){builder.seal(cx)?;return Ok(None)}
 if cx.retained_grant().maximum_copy_bytes<size_of::<bool>(){return Ok(None)}let result=JobOutcomeBorrow::admit_preview_with_cursor(cx,builder.published().unwrap(),size_of::<bool>())?;if result.is_some(){self.delivered=true;}Ok(result)
 }
 pub(super) fn published_surface(&self)->bool{self.surface_selected}
 /// ⏩️ The original numerical table remains frozen until every declared publication range is delivered.
 pub(super) fn has_more_ranges(&self,table:Option<&crate::results::SurfaceEnergyTable>)->Result<bool,ValueError>{if !self.surface_selected{return Ok(false)}let table=table.ok_or_else(||invalid("original range continuation lost its table"))?;let extent=table.len().checked_mul(24).and_then(|rows|rows.checked_add(76)).ok_or_else(||invalid("original range continuation extent overflow"))?;Ok(self.offset.checked_add(prepared::STRIDE).is_some_and(|next|next<extent))}
 /// 🔄️ Original wire, pages and writer close before the paid successor resumes the same prepared publication.
 pub(super) fn advance_range(&mut self,writer:&mut Option<ToolRunTickWriter>,cx:&mut StepContext<'_>)->Result<(),ValueError>{if !self.delivered||cx.is_cancelled(){return Ok(())}if self.wire.is_some()||self.builder.is_some()||!self.source.terminal_is_empty(){let demand=self.retirement_demands()?;if !permits(cx.retained_grant(),demand){return Ok(())}let progress=self.close_admitted(cx.retained_grant(),demand)?;cx.consume_retained(progress)?;return Ok(())}let demand=RetirementDemand{copy_bytes:2*size_of::<ControlledRetirement<ToolRunTickWriter>>()+size_of::<Option<ToolRunTickWriter>>()+size_of::<usize>()+size_of::<bool>(),depth:1,..Default::default()};if !permits(cx.retained_grant(),demand){return Ok(())}let original=writer.take().ok_or_else(||invalid("original range successor writer is absent"))?;self.source=ControlledRetirement::new(original).map_err(|(error,original)|{*writer=Some(original);error})?;self.offset=self.offset.checked_add(prepared::STRIDE).ok_or_else(||invalid("original range offset overflow"))?;self.delivered=false;cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})}
 pub(super) fn admit_successor(&self,cx:&mut StepContext<'_>)->Result<Option<ToolRunTickWriter>,ValueError>{if cx.is_cancelled(){return Ok(None)}let source=self.source.original().ok_or_else(||invalid("original tick source retired before successor"))?;let result=source.admit_original_successor(cx.retained_grant())?;if let Some((writer,receipt))=result{cx.consume_retained(receipt)?;Ok(Some(writer))}else{Ok(None)}}
 pub(super) fn borrow_outcome<'a>(&'a self,descriptor:&'a JobOutcomeDescriptor)->Result<JobOutcomeView<'a>,ValueError>{descriptor.preview(self.builder.as_ref().and_then(|b|b.published()).ok_or_else(||invalid("original Energy tick is not sealed"))?)}
 pub(super) fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{
 if let Some(wire)=self.wire.as_ref(){if !wire.terminal_is_empty(){let mut demand=wire.retirement_demands();demand.depth+=1;return Ok(demand)}return Ok(RetirementDemand{copy_bytes:size_of::<Option<ToolRunTickWireCursor>>(),depth:1,..Default::default()})}
 if let Some(builder)=self.builder.as_ref(){if !builder.terminal_is_empty(){let mut demand=builder.retirement_demands()?;demand.depth+=1;return Ok(demand)}return Ok(RetainedPayloadBuilder::boxed_release_demands())}
 if !self.source.terminal_is_empty(){return Ok(RetirementDemand{copy_bytes:self.source.next_copy_byte_demand()?,capacity_bytes:self.source.next_capacity_byte_demand(usize::MAX)?,release_bytes:self.source.next_release_byte_demand()?,depth:self.source.next_depth_demand()?+1})}
 if !self.prepared.terminal_is_empty(){return if !self.prepared.is_closing(){Ok(RetirementDemand{copy_bytes:size_of::<bool>(),depth:1,..Default::default()})}else{self.prepared.close_demands()}}
 Ok(if self.empty{Default::default()}else{RetirementDemand{copy_bytes:size_of::<Option<Box<Self>>>()+size_of::<bool>(),release_bytes:size_of::<Self>(),depth:1,..Default::default()}})
 }
 pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->Result<InteractiveJobCloseStep,ValueError>{
  let demand=self.retirement_demands()?;
  if !permits(grant,demand){return Ok(InteractiveJobCloseStep::Pending{progress:Default::default()})}
  let progress=self.close_admitted(grant,demand)?;
  Ok(if self.empty{InteractiveJobCloseStep::Complete{progress}}else{InteractiveJobCloseStep::Pending{progress}})
 }
 fn close_admitted(&mut self,grant:RetainedCloneGrant,demand:RetirementDemand)->Result<RetainedCloneProgress,ValueError>{if let Some(wire)=self.wire.as_mut(){if !wire.terminal_is_empty(){return Ok(wire.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}))}self.wire=None;return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})}if let Some(builder)=self.builder.as_mut(){if !builder.terminal_is_empty(){return builder.close_step_granted(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|step.progress())}self.builder=None;return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})}if !self.source.terminal_is_empty(){return self.source.step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|step.progress())}if !self.prepared.terminal_is_empty(){if !self.prepared.is_closing(){self.prepared.begin_close();return Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})}return self.prepared.close_step(grant)}self.empty=true;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})}
 pub(super) fn terminal_is_empty(&self)->bool{self.empty}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
