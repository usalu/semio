//! 🎒️ Retained canonical codec1 documents borrow original fields per turn and lend only paid output bytes.
use crate::{BorrowedProjectedPackCursor,BorrowedPackSection};
use semio_framework_pack_record_preflight::BorrowedSchemaHashCursor;
use semio_framework_replication_codec::{RetainedDeflateEncoder,Crc32cCursor};
use semio_framework_pack_format::retained_encoding::{self as grammar,RetainedCanonicalLayout,RetainedChunkEntry};
use semio_framework_dsl_record::{BorrowedRecordSpec,native_encoding::FieldProjectionSource};
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind,retirement::RetireOwned};
use semio_framework_value_persistent::{list::PagedList, list::PagedListError, list::{PagedListReleaseFailure,PagedListReleaseOutcome,PagedListReservationFailure,PagedListReservationOutcome}};
#[derive(Clone,Copy)]
struct FrameExtent{raw:usize,stored:usize}
semio_framework_value::artifact_retire_leaf!(FrameExtent);
/// 🎒️ A foreign plain value held in a paged list retires as one inline leaf.
pub struct Held<T>(T);
impl<T> std::ops::Deref for Held<T>{type Target=T;fn deref(&self)->&T{&self.0}}
impl<T> std::ops::DerefMut for Held<T>{fn deref_mut(&mut self)->&mut T{&mut self.0}}
/// 🎒️ The held plain value drops as one paid inline action.
pub struct HeldCursor<T>(Option<Held<T>>);
impl<T:Send+'static> RetireOwned for Held<T>{type RetirementCursor=HeldCursor<T>;fn retirement_seed(self)->Self::RetirementCursor{HeldCursor(Some(self))}fn retirement_birth_bytes(&self)->Option<usize>{Some(std::mem::size_of::<HeldCursor<T>>())}fn controlled_retirement_supported()->bool{true}}
impl<T:Send+'static> HeldCursor<T>{
    fn close_step(&mut self,_body:usize,effect:&mut RetainedAllocationEffect<'_>)->semio_framework_value::retirement::RetirementStep{use semio_framework_value::retirement::{RetirementStep,record_retirement_work};if self.0.is_none(){return RetirementStep::Complete(0)}match record_retirement_work(effect,1){Ok(work)=>{self.0=None;RetirementStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()},work)},Err((error,work))=>RetirementStep::Failure(error.into(),work)}}
}
impl<T:Send+'static> semio_framework_value::retirement::RetirementCursor for HeldCursor<T>{
    semio_framework_value::native_retirement_reply!();
    fn terminal_is_empty(&self)->bool{self.0.is_none()}
}

/// 📏️ Chunk payload bytes one bounded read or emit event moves: a chunk byte costs a copy and a hash or checksum update, far below one segment codec event, so chunk events move runs instead of single bytes.
const CHUNK_RUN:usize=64;
/// 🏃️ Raw segment bytes one original-source pull stages for the compressor: one segment event pulls at most one such run and drives every hash, compressor and compressed-output step it feeds, so a segment costs events per run instead of several per raw byte.
const SEGMENT_RUN:usize=64;
/// 📦️ Compressed bytes one kept block holds: the measure pass appends every segment's compressed bytes into such blocks, and one emit event of the second pass lends at most one block's remaining bytes.
const STORE_BLOCK:usize=256;
/// 🛑️ Compressor steps one segment event may chain at most, a backstop far above what one run of [`SEGMENT_RUN`] bytes needs.
const SEGMENT_STEPS:usize=1<<16;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Phase{ContentBirth,Schema,Segment,FrameCatalog,ChunkBirth,ChunkRead,ChunkCatalog,Header,Prefix,Checksum,ChunkPrefix,ChunkEmit,ChunkChecksum,TablePrefix,TableBody,TableChecksum,End,EndChecksum,Footer,Complete,Closing,Closed}

/// 🎟️ The sole physical receipt and initialized final output prefix from one document turn.
#[derive(Default,Debug)]
pub struct BorrowedPackDocumentProgress{pub progress:RetainedCloneProgress,pub written_bytes:usize,pub complete:bool}
/// ⚠️ Original refusal authority retains every physical currency without discarding final output custody.
#[derive(Debug)]
pub struct BorrowedPackDocumentFailure{pub reason:ValueError,pub written_bytes:usize}
impl From<ValueError> for BorrowedPackDocumentFailure{fn from(reason:ValueError)->Self{Self{reason,written_bytes:0}}}

/// 🧵️ One measuring pass projects and deflates every segment once and keeps its compressed bytes (bounded by the final output); the emitting pass lends those kept bytes after the framing the measured lengths now determine, so no segment is projected or deflated twice.
pub struct BorrowedPackDocumentCursor{
 last_work:u64,source:BorrowedProjectedPackCursor,schema:BorrowedSchemaHashCursor,deflate:RetainedDeflateEncoder,
 phase:Phase,identity:Option<usize>,spec:Option<BorrowedRecordSpec>,layout:RetainedCanonicalLayout,
 measure:bool,segment:usize,offset:usize,raw:usize,need_raw:bool,raw_done:bool,staged:[u8;SEGMENT_RUN],staged_start:usize,staged_hashed:usize,staged_end:usize,compressed:Option<u8>,
 crc:Crc32cCursor,content:PagedList<Held<semio_framework_hash::Hasher>,1>,written:usize,maximum_file:usize,maximum_body:usize,frame_size:usize,frames:PagedList<FrameExtent,{isize::MAX as usize}>,frame_index:usize,
 chunk_entries:PagedList<Held<RetainedChunkEntry>,{isize::MAX as usize}>,chunk_index:usize,chunk_offset:usize,payload_crc:Crc32cCursor,
 store:PagedList<Held<[u8;STORE_BLOCK]>,{isize::MAX as usize}>,store_fill:usize,store_read:usize,
}
impl Default for BorrowedPackDocumentCursor{fn default()->Self{Self::with_maximum_file_len(usize::MAX)}}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn page_error(reason:PagedListError)->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,reason.reason)}
use crate::{reserve_list as reserve,release_list as release};
impl BorrowedPackDocumentCursor{
 /// 📏️ Keeps the caller's final output ceiling while preserving canonical compression and framing.
 pub fn with_maximum_file_len(maximum_file:usize)->Self{Self{last_work:0,source:BorrowedProjectedPackCursor::for_document(),schema:Default::default(),deflate:Default::default(),phase:Phase::ContentBirth,identity:None,spec:None,layout:Default::default(),measure:true,segment:0,offset:0,raw:0,need_raw:false,raw_done:false,staged:[0;SEGMENT_RUN],staged_start:0,staged_hashed:0,staged_end:0,compressed:None,crc:Crc32cCursor::new(),content:PagedList::empty(),written:0,maximum_file,maximum_body:usize::MAX,frame_size:1048576,frames:PagedList::empty(),frame_index:0,chunk_entries:PagedList::empty(),chunk_index:0,chunk_offset:0,payload_crc:Crc32cCursor::new(),store:PagedList::empty(),store_fill:0,store_read:0}}
 /// 🛡️ Captures the original uncompressed Body ceiling independently of compressed output custody.
 pub fn with_maximum_body_len(mut self,maximum_body:usize)->Self{self.maximum_body=maximum_body;self}
 /// 🪜️ Limits actual original body paths independently of schema work and caller structural grants.
 pub fn with_maximum_source_depth(mut self,maximum:usize)->Self{self.source.set_maximum_source_depth(maximum);self}
 /// 🎛️ Admits authored canonical codec1 frame boundaries while keeping the producer's original-source ownership.
 pub fn with_options(options:&semio_framework_replication_codec::PackEncodeOptions)->Result<Self,ValueError>{if options.codec.0!=1||!options.canonical||options.chunk_threshold!=262144||options.chunk_size!=1048576{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retained Pack requires authored canonical codec1 chunk options"));}let mut cursor=Self::with_maximum_file_len(usize::try_from(options.limits.max_file_len).unwrap_or(usize::MAX));cursor.frame_size=usize::try_from(options.frame_size.max(1)).map_err(|_|invalid("retained Pack frame exceeds address space"))?;Ok(cursor)}
 pub fn last_turn_work(&self)->u64{self.last_work}
 pub fn is_complete(&self)->bool{self.phase==Phase::Complete}
 pub fn terminal_is_empty(&self)->bool{self.phase==Phase::Closed&&self.source.terminal_is_empty()&&self.schema.terminal_is_empty()&&self.deflate.terminal_is_empty()&&self.frames.terminal_is_empty()&&self.content.terminal_is_empty()&&self.chunk_entries.terminal_is_empty()&&self.store.terminal_is_empty()}
 pub fn output_len(&self)->Option<usize>{if self.measure{None}else{self.layout.total().ok()}}
 pub fn emitted_bytes(&self)->usize{self.written}
 fn segment_kind(&self)->u8{[semio_framework_replication_codec::ids::KIND_SYMBOLS,semio_framework_replication_codec::ids::KIND_DOCUMENT,semio_framework_replication_codec::ids::KIND_MANIFEST][self.segment]}
 fn source_boundary(&self)->bool{match self.segment{0=>matches!(self.source.section(),BorrowedPackSection::Body|BorrowedPackSection::Complete),1=>self.source.is_complete()||self.raw==self.frame_size,_=>self.raw==self.layout.manifest_len().unwrap_or(usize::MAX)}}
 fn extent(&self)->FrameExtent{if self.segment==1{self.frames[self.frame_index]}else{FrameExtent{raw:self.layout.raw[self.segment],stored:self.layout.stored[self.segment]}}}
 fn demands(&self)->Result<(usize,usize,usize),ValueError>{Ok(match self.phase{
  Phase::ContentBirth if !self.content.has_reserved_slot()=>(self.content.next_allocation_bytes().map_err(page_error)?,self.content.next_reserve_copy_byte_demand().map_err(page_error)?,self.content.next_reserve_depth_demand().map_err(page_error)?+1),
  Phase::ContentBirth=>(0,std::mem::size_of::<semio_framework_hash::Hasher>(),self.content.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::Schema=>(self.schema.next_capacity_byte_demand()?,self.schema.next_copy_byte_demand()?,self.schema.next_depth_demand()?+1),
  Phase::FrameCatalog if !self.frames.has_reserved_slot()=>(self.frames.next_allocation_bytes().map_err(page_error)?,self.frames.next_reserve_copy_byte_demand().map_err(page_error)?,self.frames.next_reserve_depth_demand().map_err(page_error)?+1),
  Phase::FrameCatalog=>(0,std::mem::size_of::<FrameExtent>()+if self.source.is_complete(){32}else{0},self.frames.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::ChunkBirth=>(0,if self.chunk_index<self.source.chunk_count(){std::mem::size_of::<semio_framework_hash::Hasher>()}else{0},self.content.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::ChunkRead=>(0,3*self.chunk_run()?,(self.source.chunk_depth(self.chunk_index)?).max(self.content.next_pop_depth_demand().map_err(page_error)?)+1),
  Phase::ChunkCatalog if !self.chunk_entries.has_reserved_slot()=>(self.chunk_entries.next_allocation_bytes().map_err(page_error)?,self.chunk_entries.next_reserve_copy_byte_demand().map_err(page_error)?,self.chunk_entries.next_reserve_depth_demand().map_err(page_error)?+1),
  Phase::ChunkCatalog=>(0,std::mem::size_of::<RetainedChunkEntry>()+32,self.chunk_entries.next_pop_depth_demand().map_err(page_error)?.max(self.content.next_pop_depth_demand().map_err(page_error)?)+1),
  Phase::ChunkPrefix=>(0,if self.offset<grammar::prefix_len(self.chunk_entries[self.chunk_index].length,self.chunk_entries[self.chunk_index].length,false){2}else{0},self.chunk_entries.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::ChunkEmit=>(0,2*self.chunk_run()?,self.source.chunk_depth(self.chunk_index)?+1),
  Phase::ChunkChecksum|Phase::TableChecksum=>(0,usize::from(self.offset<4),1),
  Phase::TablePrefix=>(0,if self.offset<grammar::prefix_len(self.layout.chunk_table_raw,self.layout.chunk_table_raw,false){2}else{0},1),
  Phase::TableBody=>(0,if self.offset<self.layout.chunk_table_raw{2}else{0},self.chunk_entries.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::Segment if self.staged_hashed<self.staged_end=>(0,self.staged_end-self.staged_hashed,self.content.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::Segment if !self.measure=>(0,2*self.stored_run(),self.store.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::Segment if self.compressed.is_some()&&self.store_full()&&!self.store.has_reserved_slot()=>(self.store.next_allocation_bytes().map_err(page_error)?,self.store.next_reserve_copy_byte_demand().map_err(page_error)?,self.store.next_reserve_depth_demand().map_err(page_error)?+1),
  Phase::Segment if self.compressed.is_some()&&self.store_full()=>(0,std::mem::size_of::<Held<[u8;STORE_BLOCK]>>(),self.store.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::Segment if self.compressed.is_some()=>(0,1,self.store.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::Segment if self.deflate.is_complete()=>(0,0,1),
  Phase::Segment if self.need_raw&&!self.source_boundary()&&self.segment<2=>(self.source.next_capacity_byte_demand()?,self.source.next_minimum_copy_bytes(),self.source.next_advance_depth_demand()?+1),
  Phase::Segment if self.need_raw=>(0,usize::from(!self.source_boundary()),1),
  Phase::Segment=>(self.deflate.next_capacity_byte_demand()?,self.deflate.next_copy_byte_demand()?+self.compressor_bookkeeping_copy(),self.deflate.next_depth_demand()?+1),
  Phase::Header=>(0,if self.offset<20{2}else{1},1),
  Phase::Prefix|Phase::End=>(0,if self.offset<if self.phase==Phase::End{3}else{let extent=self.extent();grammar::prefix_len(extent.raw,extent.stored,true)}{2}else{0},1),
  Phase::Checksum|Phase::EndChecksum=>(0,usize::from(self.offset<4),1),
  Phase::Footer=>(0,if self.offset<80{2}else{1},1),_ =>(0,0,1),
 })}
 pub fn next_capacity_byte_demand(&self)->Result<usize,ValueError>{self.demands().map(|v|v.0)}
 pub fn next_copy_byte_demand(&self)->Result<usize,ValueError>{self.demands().map(|v|v.1)}
 pub fn next_depth_demand(&self)->Result<usize,ValueError>{self.demands().map(|v|v.2)}
 /// 📏️ The chunk bytes the next chunk read or emit event moves: up to [`CHUNK_RUN`], never past the chunk's end.
 fn chunk_run(&self)->Result<usize,ValueError>{Ok(self.source.chunk_len(self.chunk_index)?.saturating_sub(self.chunk_offset).min(CHUNK_RUN))}
 /// 📏️ The kept compressed bytes the next emit event lends: the rest of the segment, never past the end of the block it reads.
 fn stored_run(&self)->usize{(self.extent().stored-self.raw).min(STORE_BLOCK-self.store_read%STORE_BLOCK)}
 /// 📦️ Whether the next kept byte needs a fresh block.
 fn store_full(&self)->bool{self.store.is_empty()||self.store_fill==STORE_BLOCK}
 fn compressor_bookkeeping_copy(&self)->usize{std::mem::size_of::<bool>()+usize::from(self.deflate.phase().as_str()=="output")*std::mem::size_of::<Option<u8>>()}
 fn start_segment(&mut self)->Result<(),ValueError>{if self.measure{self.deflate.restart()?;}self.raw=0;self.need_raw=false;self.raw_done=false;self.staged_start=0;self.staged_hashed=0;self.staged_end=0;self.compressed=None;Ok(())}
 fn fixed(&mut self,byte:u8,hash:bool,output:&mut[u8])->BorrowedPackDocumentProgress{if output.is_empty(){self.last_work=0;return Default::default();}output[0]=byte;if hash{self.crc.update_page(&[byte]);}self.offset+=1;self.written+=1;BorrowedPackDocumentProgress{progress:RetainedCloneProgress{copied_items:1,copied_bytes:1+usize::from(hash),..Default::default()},written_bytes:1,complete:false}}
 fn finish_segment(&mut self)->Result<(),ValueError>{
  if self.measure{
   if self.segment==1{self.phase=Phase::FrameCatalog;return Ok(());}self.layout.raw[self.segment]=self.raw;self.layout.stored[self.segment]=self.deflate.output_len();
   if self.segment==0{self.layout.symbols=self.source.symbol_count().ok_or_else(||invalid("retained Pack symbols have no complete authority"))?;}
   if self.segment<2{self.segment+=1;self.start_segment()?;}else{
    let total=self.layout.total()?;if total>self.maximum_file{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"retained Pack final output exceeds caller ceiling"));}
    self.measure=false;self.segment=0;self.store_read=0;self.start_segment()?;self.phase=Phase::Header;self.offset=0;self.crc=Crc32cCursor::new();
   }
  }else{return Err(invalid("retained Pack emits kept segments without compressing again"));}
  Ok(())
 }
 /// ✍️ Performs one bounded original-source, compressor, framing or final-output custody event.
 pub fn advance(&mut self,source:&dyn FieldProjectionSource,spec:BorrowedRecordSpec,output:&mut[u8],effect:&mut RetainedAllocationEffect<'_>)->Result<BorrowedPackDocumentProgress,BorrowedPackDocumentFailure>{
  let before=effect.performed_work();let result=self.advance_inner(source,spec,output,effect);
  Self::record_own(effect,before,u128::from(self.last_work),result)
 }
 /// 📦️ Performs up to `maximum_events` consecutive bounded events as ONE owner action: every event is still quoted, funded and recorded as its own work unit, byte currencies sum exactly, and the batch is one copied item because a byte-level codec event owns no item of its own. It stops before an event the remaining grant, output or work cannot fund, so an allocating or releasing event always starts the next action.
 pub fn advance_batch(&mut self,source:&dyn FieldProjectionSource,spec:BorrowedRecordSpec,output:&mut[u8],effect:&mut RetainedAllocationEffect<'_>,maximum_events:usize)->Result<BorrowedPackDocumentProgress,BorrowedPackDocumentFailure>{
  let grant=effect.grant();let mut total=BorrowedPackDocumentProgress::default();let mut events=0usize;
  while events<maximum_events&&total.written_bytes<output.len()&&effect.remaining_work_units().is_ok_and(|left|left>0){
   let spent=total.progress;
   let left=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(spent.copied_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(spent.retained_capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(spent.released_bytes),..grant};
   let step=match self.advance(source,spec,&mut output[total.written_bytes..],&mut effect.nested(left)){
    Ok(step)=>step,
    Err(mut failure)=>{let held=failure.reason.retained_progress();failure.reason=failure.reason.with_retained_progress(RetainedCloneProgress{copied_items:held.copied_items,copied_bytes:held.copied_bytes+spent.copied_bytes,retained_capacity_bytes:held.retained_capacity_bytes+spent.retained_capacity_bytes,released_bytes:held.released_bytes+spent.released_bytes});failure.written_bytes+=total.written_bytes;return Err(failure)}
   };
   if self.last_work==0{break}
   events+=1;total.written_bytes+=step.written_bytes;total.progress.copied_bytes+=step.progress.copied_bytes;total.progress.retained_capacity_bytes+=step.progress.retained_capacity_bytes;total.progress.released_bytes+=step.progress.released_bytes;
   if step.complete{break}
  }
  total.progress.copied_items=usize::from(events!=0);total.complete=self.is_complete();self.last_work=events as u64;Ok(total)
 }
 /// 🏃️ One segment event: the step the caller's grant was quoted for, then every further hash, compressor, compressed-output and segment-closing step the grant still funds, with an original-source pull only as its first step; it stops after the compressor asks for input, before a step that allocates and before a compressed byte with no output left, and sums every byte currency exactly.
 fn segment_run(&mut self,source:&dyn FieldProjectionSource,spec:BorrowedRecordSpec,output:&mut[u8],effect:&mut RetainedAllocationEffect<'_>,grant:RetainedCloneGrant)->Result<BorrowedPackDocumentProgress,BorrowedPackDocumentFailure>{
  let mut total=RetainedCloneProgress{copied_items:1,..Default::default()};let(mut written,mut steps)=(0usize,0usize);
  while self.phase==Phase::Segment&&steps<SEGMENT_STEPS{
   let left=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(total.copied_bytes),maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(total.retained_capacity_bytes),maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(total.released_bytes),..grant};
   let pull=self.need_raw&&self.staged_start==self.staged_end&&!self.source_boundary()&&self.segment<2;
   if steps!=0{let(capacity,copy,depth)=self.demands()?;if capacity!=0||left.maximum_copy_bytes<copy||left.maximum_depth<depth||pull{break}}
   if self.compressed.is_some()&&!self.measure&&written==output.len(){if steps==0{self.last_work=0;return Ok(Default::default());}break}
   let closing=self.deflate.is_complete()&&self.staged_hashed==self.staged_end&&self.compressed.is_none();
   let step=match self.segment_step(source,spec,&mut output[written..],effect,left){Ok(step)=>step,Err(mut failure)=>{
    let held=failure.reason.retained_progress();failure.reason=failure.reason.with_retained_progress(RetainedCloneProgress{copied_items:held.copied_items.max(1),copied_bytes:held.copied_bytes+total.copied_bytes,retained_capacity_bytes:held.retained_capacity_bytes+total.retained_capacity_bytes,released_bytes:held.released_bytes+total.released_bytes});failure.written_bytes+=written;return Err(failure)}};
   let need=self.need_raw;steps+=1;written+=step.1;total.copied_bytes+=step.0.copied_bytes;total.retained_capacity_bytes+=step.0.retained_capacity_bytes;total.released_bytes+=step.0.released_bytes;
   if closing||step.0.retained_capacity_bytes!=0||need{break}
  }
  Ok(BorrowedPackDocumentProgress{progress:total,written_bytes:written,complete:false})
 }
 /// 🧩️ One funded segment step: hash staged body bytes, lend one compressed byte, close the segment, stage the next raw run, or advance the compressor by one frontier.
 fn segment_step(&mut self,source:&dyn FieldProjectionSource,spec:BorrowedRecordSpec,output:&mut[u8],effect:&mut RetainedAllocationEffect<'_>,grant:RetainedCloneGrant)->Result<(RetainedCloneProgress,usize),BorrowedPackDocumentFailure>{
  let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
  if self.staged_hashed<self.staged_end{let(from,to)=(self.staged_hashed,self.staged_end);self.content[0].update(&self.staged[from..to]);self.staged_hashed=to;progress.copied_bytes=to-from;return Ok((progress,0));}
  if let Some(byte)=self.compressed.filter(|_|self.measure){
   if self.store_full(){
    if !self.store.has_reserved_slot(){return Ok((reserve(&mut self.store,effect)?,0));}
    self.store.push_reserved(Held([0;STORE_BLOCK])).map_err(|_|invalid("retained Pack kept segment has no admitted block"))?;self.store_fill=0;progress.copied_bytes=std::mem::size_of::<Held<[u8;STORE_BLOCK]>>();return Ok((progress,0));
   }
   let last=self.store.len()-1;self.store[last].0[self.store_fill]=byte;self.store_fill+=1;self.compressed=None;progress.copied_bytes=1;return Ok((progress,0));
  }
  if let Some(byte)=self.compressed{if output.is_empty(){return Err(invalid("retained Pack segment lent a compressed byte without output").into());}self.compressed=None;self.offset=0;let step=self.fixed(byte,true,output);return Ok((step.progress,step.written_bytes));}
  if self.deflate.is_complete(){self.finish_segment()?;return Ok((progress,0));}
  if self.need_raw&&self.staged_start==self.staged_end{
   if self.source_boundary(){self.raw_done=true;self.need_raw=false;return Ok((progress,0));}
   if self.segment==2{
    let count=(self.layout.manifest_len()?-self.raw).min(SEGMENT_RUN).min(grant.maximum_copy_bytes.max(1));
    for index in 0..count{self.staged[index]=self.layout.manifest_byte(self.raw+index)?;}
    self.raw+=count;self.staged_start=0;self.staged_hashed=count;self.staged_end=count;self.need_raw=false;progress.copied_bytes=count;return Ok((progress,0));
   }
   let run=if self.segment==1{(self.frame_size-self.raw).min(SEGMENT_RUN)}else{1};
   let child_grant=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};
   let step=self.source.advance(source,spec,&mut self.staged[..run],&mut effect.nested(child_grant)).map_err(|failure|BorrowedPackDocumentFailure{reason:failure.reason,written_bytes:0})?;progress=step.progress;
   let count=step.written_bytes;
   if count!=0{
    self.raw=self.raw.checked_add(count).ok_or_else(||invalid("retained Pack raw length overflow"))?;self.need_raw=false;self.staged_start=0;self.staged_end=count;self.staged_hashed=if self.measure&&self.segment==1{0}else{count};
    if self.measure&&self.segment==1&&self.layout.raw[1].checked_add(self.raw).is_none_or(|length|length>self.maximum_body){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"retained Pack original body exceeds caller ceiling").with_retained_progress(progress).into());}
   }
   return Ok((progress,0));
  }
  let bookkeeping=self.compressor_bookkeeping_copy();let mut byte=[0;1];
  let mut pending=(self.staged_start<self.staged_end).then(||self.staged[self.staged_start]);let offered=pending.is_some();
  let step=self.deflate.advance(&mut pending,self.raw_done,&mut byte,&mut effect.nested(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,maximum_copy_bytes:grant.maximum_copy_bytes-bookkeeping,..grant}))?;
  if offered&&pending.is_none(){self.staged_start+=1;if self.staged_start==self.staged_end{self.staged_start=0;self.staged_hashed=0;self.staged_end=0;}}
  progress=step.progress;self.need_raw=step.need_input;if step.written_bytes!=0{self.compressed=Some(byte[0]);}progress.copied_items=progress.copied_items.max(1);progress.copied_bytes+=bookkeeping;
  Ok((progress,0))
 }
 /// 📤️ One emit event: lends the next run of the segment's kept compressed bytes and checksums them, then closes the segment once all of them are out.
 fn emit_stored(&mut self,output:&mut[u8])->BorrowedPackDocumentProgress{
  if self.raw==self.extent().stored{self.phase=Phase::Checksum;self.offset=0;return BorrowedPackDocumentProgress{progress:RetainedCloneProgress{copied_items:1,..Default::default()},..Default::default()};}
  let count=self.stored_run().min(output.len());if count==0{self.last_work=0;return Default::default();}
  let(block,at)=(self.store_read/STORE_BLOCK,self.store_read%STORE_BLOCK);output[..count].copy_from_slice(&self.store[block].0[at..at+count]);
  self.crc.update_page(&output[..count]);self.store_read+=count;self.raw+=count;self.written+=count;
  BorrowedPackDocumentProgress{progress:RetainedCloneProgress{copied_items:1,copied_bytes:2*count,..Default::default()},written_bytes:count,complete:false}
 }
 fn record_own<R>(effect:&mut RetainedAllocationEffect<'_>,before:u128,own:u128,result:Result<R,BorrowedPackDocumentFailure>)->Result<R,BorrowedPackDocumentFailure>{
  if effect.performed_work()!=before||own==0{return result}let recorded=u64::try_from(own).map_err(|_|invalid("retained Pack action work overflow")).and_then(|work|effect.performed(work).map_err(|error|error).and_then(|admitted|if admitted{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::WorkLimit,"retained Pack action exceeded its original work"))}));
  match(result,recorded){(Ok(value),Ok(()))=>Ok(value),(Err(failure),_)=>Err(failure),(Ok(_),Err(error))=>Err(error.into())}
 }
 fn advance_inner(&mut self,source:&dyn FieldProjectionSource,spec:BorrowedRecordSpec,output:&mut[u8],effect:&mut RetainedAllocationEffect<'_>)->Result<BorrowedPackDocumentProgress,BorrowedPackDocumentFailure>{
  let grant=effect.grant();self.last_work=0;
  if matches!(self.phase,Phase::Closing|Phase::Closed){return Err(invalid("retained Pack document is closing").into());}
  if self.is_complete(){return Ok(BorrowedPackDocumentProgress{complete:true,..Default::default()});}
  if self.identity.is_some_and(|id|id!=source.projection_identity())||self.spec.is_some_and(|old|old.fields.as_ptr()!=spec.fields.as_ptr()||old.fields.len()!=spec.fields.len()){return Err(invalid("retained Pack immutable original source authority changed").into());}
  let(capacity,copy,depth)=self.demands()?;if grant.maximum_items==0||grant.maximum_capacity_bytes<capacity||grant.maximum_copy_bytes<copy||grant.maximum_depth<depth{return Ok(Default::default());}
  self.last_work=1;self.identity=Some(source.projection_identity());self.spec=Some(spec);
  let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
  match self.phase{
   Phase::ContentBirth=>{if !self.content.has_reserved_slot(){progress=reserve(&mut self.content,effect)?;}else{self.content.push_reserved(Held(semio_framework_hash::Hasher::new())).map_err(|_|invalid("retained document hash has no admitted slot"))?;progress.copied_bytes=std::mem::size_of::<semio_framework_hash::Hasher>();self.phase=Phase::Schema;}},
   Phase::Schema=>{let step=self.schema.advance(spec,&mut effect.nested(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}))?;progress=step.progress;if step.complete{self.layout.schema=self.schema.digest().ok_or_else(||invalid("retained schema digest is absent"))?;self.phase=Phase::Segment;}},
   Phase::FrameCatalog=>{
    if !self.frames.has_reserved_slot(){progress=reserve(&mut self.frames,effect)?;}
    else{let extent=FrameExtent{raw:self.raw,stored:self.deflate.output_len()};self.frames.push_reserved(extent).map_err(|_|invalid("retained Pack frame has no admitted slot"))?;progress.copied_bytes=std::mem::size_of::<FrameExtent>();self.layout.frames+=1;self.layout.raw[1]=self.layout.raw[1].checked_add(self.raw).ok_or_else(||invalid("retained Pack body length overflow"))?;self.layout.document_span=self.layout.document_span.checked_add(grammar::span(extent.raw,extent.stored)?).ok_or_else(||invalid("retained Pack document span overflow"))?;
     if self.source.is_complete(){self.layout.fields=self.source.field_count().ok_or_else(||invalid("retained Pack body has no complete authority"))?;self.layout.content=*self.content[0].finalize().as_bytes();progress.copied_bytes+=32;if self.source.chunk_count()!=0{self.chunk_index=0;self.layout.chunk_table_raw=grammar::varint_len(self.source.chunk_count());self.phase=Phase::ChunkBirth;return Ok(BorrowedPackDocumentProgress{progress,..Default::default()});}self.segment=2;}self.start_segment()?;self.phase=Phase::Segment;
    }
   },
   Phase::ChunkBirth=>{if self.chunk_index==self.source.chunk_count(){self.segment=2;self.start_segment()?;self.phase=Phase::Segment;}else{self.content[0]=Held(semio_framework_hash::Hasher::new());progress.copied_bytes=std::mem::size_of::<semio_framework_hash::Hasher>();self.payload_crc=Crc32cCursor::new();self.chunk_offset=0;self.phase=Phase::ChunkRead;}},
   Phase::ChunkRead=>{if self.chunk_offset==self.source.chunk_len(self.chunk_index)?{self.phase=Phase::ChunkCatalog;}else{let mut run=[0u8;CHUNK_RUN];let count=self.source.chunk_bytes(source,self.chunk_index,self.chunk_offset,&mut run)?;self.content[0].update(&run[..count]);self.payload_crc.update_page(&run[..count]);progress.copied_bytes=3*count;self.chunk_offset+=count;}},
   Phase::ChunkCatalog=>{
    if !self.chunk_entries.has_reserved_slot(){progress=reserve(&mut self.chunk_entries,effect)?;}
    else{let length=self.source.chunk_len(self.chunk_index)?;let offset=self.layout.chunks_start()?.checked_add(self.layout.chunk_span).and_then(|value|value.checked_add(grammar::prefix_len(length,length,false))).ok_or_else(||invalid("retained chunk payload offset overflow"))?;let entry=RetainedChunkEntry{offset,length,crc:self.payload_crc.finish(),hash:*self.content[0].finalize().as_bytes()};self.layout.chunk_table_raw=self.layout.chunk_table_raw.checked_add(entry.raw_len()).ok_or_else(||invalid("retained chunk table size overflow"))?;self.chunk_entries.push_reserved(Held(entry)).map_err(|_|invalid("retained chunk entry has no admitted backing"))?;progress.copied_bytes=std::mem::size_of::<RetainedChunkEntry>()+32;self.layout.chunk_span=self.layout.chunk_span.checked_add(grammar::raw_span(length)?).ok_or_else(||invalid("retained chunk span overflow"))?;self.layout.chunks+=1;self.chunk_index+=1;self.phase=Phase::ChunkBirth;}
   },
   Phase::Segment if !self.measure=>return Ok(self.emit_stored(output)),
   Phase::Segment=>return self.segment_run(source,spec,output,effect,grant),
   Phase::Header=>{if self.offset==semio_framework_pack_format::HEADER_SIZE{self.phase=Phase::Prefix;self.offset=0;self.crc=Crc32cCursor::new();}else{return Ok(self.fixed(self.layout.header_byte(self.offset,self.crc.finish()),self.offset<20,output));}},
   Phase::Prefix=>{let extent=self.extent();if self.offset==grammar::prefix_len(extent.raw,extent.stored,true){self.phase=Phase::Segment;self.offset=0;}else{return Ok(self.fixed(grammar::prefix_byte(self.segment_kind(),extent.raw,extent.stored,true,self.offset),true,output));}},
   Phase::Checksum=>{if self.offset<4{return Ok(self.fixed(self.crc.finish().to_le_bytes()[self.offset],false,output));}if self.segment==0&&self.layout.chunks!=0{self.chunk_index=0;self.phase=Phase::ChunkPrefix;}else if self.segment==1&&self.frame_index+1<self.frames.len(){self.frame_index+=1;self.start_segment()?;self.phase=Phase::Prefix;}else if self.segment==1&&self.layout.chunks!=0{self.phase=Phase::TablePrefix;}else if self.segment<2{self.segment+=1;self.start_segment()?;self.phase=Phase::Prefix;}else{self.phase=Phase::End;}self.offset=0;self.crc=Crc32cCursor::new();},
   Phase::ChunkPrefix=>{let length=self.chunk_entries[self.chunk_index].length;if self.offset<grammar::prefix_len(length,length,false){return Ok(self.fixed(grammar::prefix_byte(semio_framework_replication_codec::ids::KIND_CHUNK,length,length,false,self.offset),true,output));}self.chunk_offset=0;self.phase=Phase::ChunkEmit;},
   Phase::ChunkEmit=>{if self.chunk_offset==self.source.chunk_len(self.chunk_index)?{self.phase=Phase::ChunkChecksum;self.offset=0;}else{let count=self.chunk_run()?.min(output.len());if count==0{self.last_work=0;return Ok(Default::default());}let read=self.source.chunk_bytes(source,self.chunk_index,self.chunk_offset,&mut output[..count])?;self.crc.update_page(&output[..read]);self.chunk_offset+=read;self.written+=read;return Ok(BorrowedPackDocumentProgress{progress:RetainedCloneProgress{copied_items:1,copied_bytes:2*read,..Default::default()},written_bytes:read,complete:false});}},
   Phase::ChunkChecksum=>{if self.offset<4{return Ok(self.fixed(self.crc.finish().to_le_bytes()[self.offset],false,output));}self.chunk_index+=1;if self.chunk_index<self.layout.chunks{self.phase=Phase::ChunkPrefix;}else{self.segment=1;self.start_segment()?;self.phase=Phase::Prefix;}self.offset=0;self.crc=Crc32cCursor::new();},
   Phase::TablePrefix=>{let length=self.layout.chunk_table_raw;if self.offset<grammar::prefix_len(length,length,false){return Ok(self.fixed(grammar::prefix_byte(semio_framework_replication_codec::ids::KIND_CHUNK_TABLE,length,length,false,self.offset),true,output));}self.offset=0;self.chunk_offset=0;self.chunk_index=0;self.phase=Phase::TableBody;},
   Phase::TableBody=>{if self.offset==self.layout.chunk_table_raw{self.offset=0;self.phase=Phase::TableChecksum;}else{let count_bytes=grammar::varint_len(self.layout.chunks);let entry=self.offset>=count_bytes;let byte=if entry{self.chunk_entries[self.chunk_index].byte(self.chunk_offset)?}else{grammar::varint_byte(self.layout.chunks,self.offset)};let step=self.fixed(byte,true,output);if step.written_bytes!=0&&entry{self.chunk_offset+=1;if self.chunk_offset==self.chunk_entries[self.chunk_index].raw_len(){self.chunk_index+=1;self.chunk_offset=0;}}return Ok(step);}},
   Phase::TableChecksum=>{if self.offset<4{return Ok(self.fixed(self.crc.finish().to_le_bytes()[self.offset],false,output));}self.segment=2;self.start_segment()?;self.phase=Phase::Prefix;self.offset=0;self.crc=Crc32cCursor::new();},
   Phase::End=>{if self.offset<3{return Ok(self.fixed(grammar::prefix_byte(semio_framework_replication_codec::ids::KIND_END,0,0,false,self.offset),true,output));}self.phase=Phase::EndChecksum;self.offset=0;},
   Phase::EndChecksum=>{if self.offset<4{return Ok(self.fixed(self.crc.finish().to_le_bytes()[self.offset],false,output));}self.phase=Phase::Footer;self.offset=0;self.crc=Crc32cCursor::new();},
   Phase::Footer=>{if self.offset<semio_framework_pack_format::FOOTER_SIZE{return Ok(self.fixed(self.layout.footer_byte(self.offset,self.crc.finish())?,self.offset<80,output));}if self.written!=self.layout.total()?{return Err(invalid("retained Pack final output disagrees with measured exact length").into());}self.phase=Phase::Complete;},
   _=>return Err(invalid("retained Pack document work frontier is invalid").into()),
  }
  Ok(BorrowedPackDocumentProgress{progress,written_bytes:0,complete:self.is_complete()})
 }
 /// 🍂️ Quotes and closes the original child owners individually; output custody stays with its caller.
 pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{let mut demand=if !self.source.terminal_is_empty(){self.source.retirement_demands()?}else if !self.schema.terminal_is_empty(){self.schema.retirement_demands()?}else if !self.deflate.terminal_is_empty(){self.deflate.retirement_demands()?}else if !self.frames.terminal_is_empty(){RetirementDemand{copy_bytes:if self.frames.is_empty(){self.frames.next_release_copy_byte_demand().map_err(page_error)?}else{0},release_bytes:if self.frames.is_empty(){self.frames.next_release_allocation_bytes().map_err(page_error)?}else{0},depth:if self.frames.is_empty(){self.frames.next_release_depth_demand()}else{self.frames.next_pop_depth_demand()}.map_err(page_error)?,..Default::default()}}else if !self.chunk_entries.terminal_is_empty(){RetirementDemand{copy_bytes:if self.chunk_entries.is_empty(){self.chunk_entries.next_release_copy_byte_demand().map_err(page_error)?}else{0},release_bytes:if self.chunk_entries.is_empty(){self.chunk_entries.next_release_allocation_bytes().map_err(page_error)?}else{0},depth:if self.chunk_entries.is_empty(){self.chunk_entries.next_release_depth_demand()}else{self.chunk_entries.next_pop_depth_demand()}.map_err(page_error)?,..Default::default()}}else if !self.store.terminal_is_empty(){RetirementDemand{copy_bytes:if self.store.is_empty(){self.store.next_release_copy_byte_demand().map_err(page_error)?}else{0},release_bytes:if self.store.is_empty(){self.store.next_release_allocation_bytes().map_err(page_error)?}else{0},depth:if self.store.is_empty(){self.store.next_release_depth_demand()}else{self.store.next_pop_depth_demand()}.map_err(page_error)?,..Default::default()}}else if !self.content.terminal_is_empty(){RetirementDemand{copy_bytes:if self.content.is_empty(){self.content.next_release_copy_byte_demand().map_err(page_error)?}else{0},release_bytes:if self.content.is_empty(){self.content.next_release_allocation_bytes().map_err(page_error)?}else{0},depth:if self.content.is_empty(){self.content.next_release_depth_demand()}else{self.content.next_pop_depth_demand()}.map_err(page_error)?,..Default::default()}}else{RetirementDemand::default()};demand.depth+=1;Ok(demand)}
 pub fn close(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<BorrowedPackDocumentProgress,BorrowedPackDocumentFailure>{
  let before=effect.performed_work();let result=self.close_inner(effect);Self::record_own(effect,before,u128::from(self.last_work),result)
 }
 fn close_inner(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<BorrowedPackDocumentProgress,BorrowedPackDocumentFailure>{
  let grant=effect.grant();self.last_work=0;
  if self.terminal_is_empty(){return Ok(BorrowedPackDocumentProgress{complete:true,..Default::default()});}let demand=self.retirement_demands()?;if grant.maximum_items==0||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Ok(Default::default());}
  self.last_work=1;self.phase=Phase::Closing;let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let progress=if !self.source.terminal_is_empty(){self.source.close(&mut effect.nested(child)).map_err(|failure|BorrowedPackDocumentFailure{reason:failure.reason,written_bytes:0})?.progress}else if !self.schema.terminal_is_empty(){self.schema.close(&mut effect.nested(child))?.progress}else if !self.deflate.terminal_is_empty(){match self.deflate.close_step(&mut effect.nested(child))?{RetainedCloneStep::Progress(receipt)|RetainedCloneStep::Complete(receipt)=>receipt,_=>return Err(invalid("retained Deflate close lost physical receipt").into())}}else if !self.frames.terminal_is_empty(){let released=if self.frames.pop().is_some(){0}else{release(&mut self.frames,effect)?};RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()}}else if !self.chunk_entries.terminal_is_empty(){let released=if self.chunk_entries.pop().is_some(){0}else{release(&mut self.chunk_entries,effect)?};RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()}}else if !self.store.terminal_is_empty(){let released=if self.store.pop().is_some(){0}else{release(&mut self.store,effect)?};RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()}}else if !self.content.terminal_is_empty(){let released=if self.content.pop().is_some(){0}else{release(&mut self.content,effect)?};RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()}}else{self.phase=Phase::Closed;self.identity=None;self.spec=None;RetainedCloneProgress{copied_items:1,..Default::default()}};
  Ok(BorrowedPackDocumentProgress{progress,written_bytes:0,complete:self.terminal_is_empty()})
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
