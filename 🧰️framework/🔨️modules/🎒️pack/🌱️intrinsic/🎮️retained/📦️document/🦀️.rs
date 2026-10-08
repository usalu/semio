//! 📦️ Intrinsic Document verification and bounded value/chunk replay retain original wire custody.
use super::*;
use crate::format::{RetainedPackAnchorCursor as Anchor,RetainedPackSegmentCursor as Segment,RetainedPackSegmentAdmission as Admission,RetainedPackSourceEvent as Source,RetainedPackSegmentEvent as Event,RetainedPackCatalogCursor as Catalog,RetainedPackCatalogFault as CatalogFault,RetainedPackCatalog,RetainedPackCloseStep as Close,Superblock};
use crate::record::RetainedValueCursor as Value;
use semio_framework_value::list::PagedList;

pub(super) const SCHEMA_GRAPH:&[u8]=&[1,1,1,5,b'v',b'a',b'l',b'u',b'e',0,14];
const IDS:usize=isize::MAX as usize/size_of::<u64>();
type ChunkIds=PagedList<u64,IDS>;
enum Phase{Anchor,Verify,Schema,Replay,Complete}
enum Chunks{None,Gather{remaining:u64,total:u64,start:usize},Read{index:usize,end:usize,seen:u64,found:bool,verified:bool}}
pub(super) struct DocumentDriver{
 anchor:Anchor,segment:Segment,chunk_segment:Segment,catalog:Catalog,value:Value,ids:ChunkIds,
 phase:Phase,offset:usize,chunk_offset:usize,input_work:usize,source_complete:bool,chunk_complete:bool,
 superblock:Option<Superblock>,verified:Option<RetainedPackCatalog>,event:Option<Event>,document_byte:Option<(u64,u8)>,body_ingress:u64,sealed:bool,
 schema_index:usize,schema_hash:semio_framework_hash::Hasher,chunks:Chunks,pending_id:Option<u64>,chunk_hash:semio_framework_hash::Hasher,closed:bool,
}
fn catalog_fault(error:CatalogFault)->PackRefusal{error.into_pack_refusal("retained intrinsic Document catalog")}
fn paged(error:semio_framework_value::list::PagedListError)->PackRefusal{PackRefusal::from_paged_refusal(error,"retained intrinsic chunk references",0)}
fn partial_close(step:Close)->RetainedCloneStep{match close_receipt(step){RetainedCloneStep::Complete(progress)|RetainedCloneStep::Progress(progress)=>RetainedCloneStep::Progress(progress)}}
fn segment_one(cursor:&mut Segment,input:&[u8],offset:&mut usize,complete:&mut bool,maximum:usize)->Result<Option<Event>,PackRefusal>{
 if let Some(bytes)=cursor.next_allocation_bytes(){if bytes>maximum{return Err(capacity())}cursor.reserve_allocation(bytes).map_err(|error|error.fault.into_pack_refusal("retained intrinsic inflater",0))?;return Ok(None)}
 match cursor.preflight(){
  Ok(())=>{let event=if *offset<input.len(){let event=Source::Byte{offset:*offset as u64,value:input[*offset]};*offset+=1;event}else if !*complete{*complete=true;Source::Complete{bytes:*offset as u64,pages:0}}else{return cursor.grant()};cursor.admit(event).map_err(|_|invalid("segment source handback"))?;Ok(None)},
  Err(Admission::Fault(error))=>Err(error.clone()),Err(Admission::Closed)=>Err(invalid("closed document segment")),Err(Admission::Complete)=>Ok(None),Err(Admission::Pending|Admission::InflaterBackpressure)=>cursor.grant(),
 }
}
impl DocumentDriver{
 fn replay_one(&mut self,input:&[u8],maximum:usize)->Result<Option<Token>,PackRefusal>{
  if let Some(bytes)=self.value.next_allocation_bytes()?{if bytes>maximum{return Err(capacity())}self.value.reserve_allocation(bytes).map_err(|error|error.fault)?;return Ok(None)}
  if let Some(id)=self.pending_id{
   let target=self.ids.len().checked_add(1).ok_or_else(capacity)?;if let Some(bytes)=self.ids.next_capacity_allocation_bytes(target).map_err(paged)?{if bytes>maximum{return Err(capacity())}self.ids.reserve_capacity_one(target,bytes).map_err(|error|PackRefusal::from_paged_allocation(error,"retained intrinsic chunk references",0))?;return Ok(None)}
   let entry=self.catalog.chunk(id).map_err(catalog_fault)?;let Chunks::Gather{remaining,total,..}=&mut self.chunks else{return Err(invalid("chunk reference outside declaration"))};*total=total.checked_add(entry.raw_len).ok_or_else(capacity)?;*remaining=remaining.checked_sub(1).ok_or_else(||invalid("too many chunk references"))?;self.ids.push_reserved(id).map_err(|_|invalid("chunk reference exceeds admitted owner"))?;self.pending_id=None;return Ok(None)
  }
  if matches!(self.chunks,Chunks::Read{..}){return self.chunk_one(input,maximum)}
  if let Some(token)=self.value.grant()?{
   match token{
    Token::Tag{value:0x09,..}=>return Ok(None),
    Token::Begin{kind:Container::ChunkedBytes,count}=>{if !matches!(self.chunks,Chunks::None){return Err(invalid("nested chunk declaration"))}self.chunks=Chunks::Gather{remaining:count,total:0,start:self.ids.len()};return Ok(None)},
    Token::Unsigned{role:Role::Chunk,value}=>{if value>u32::MAX as u64{return Err(invalid("chunk id exceeds u32"))}self.pending_id=Some(value);return Ok(None)},
    Token::End(Container::ChunkedBytes)=>{let Chunks::Gather{remaining:0,total,start}=self.chunks else{return Err(invalid("incomplete chunk references"))};self.chunks=Chunks::Read{index:start,end:self.ids.len(),seen:0,found:false,verified:false};return Ok(Some(Token::Unsigned{role:Role::BytesLength,value:total}))},
    _=>return Ok(Some(token)),
   }
  }
  if let Some((index,byte))=self.document_byte{if self.value.ingress_ready(){self.value.admit_byte(index,byte).map_err(|_|invalid("document value handback"))?;self.document_byte=None;self.body_ingress+=1}return Ok(None)}
  if self.source_complete&&self.offset==input.len()&&matches!(self.segment.preflight(),Err(Admission::Complete)){
   if !self.sealed{self.value.seal(self.body_ingress)?;self.sealed=true;return Ok(None)}return Err(invalid("document terminal value progress unavailable"))
  }
  let before=self.offset;if let Some(event)=segment_one(&mut self.segment,input,&mut self.offset,&mut self.source_complete,maximum)?{if let Event::RawByte{segment,value,..}=event{if segment.kind==crate::KIND_DOCUMENT{self.document_byte=Some((self.body_ingress,value))}}}
  self.input_work=self.input_work.checked_add(self.offset-before).ok_or_else(capacity)?;Ok(None)
 }
 fn chunk_one(&mut self,input:&[u8],maximum:usize)->Result<Option<Token>,PackRefusal>{
  let Chunks::Read{index,end,seen,found,verified}=self.chunks else{unreachable!()};
  if index==end{self.chunks=Chunks::None;return Ok(None)}
  let id=*self.ids.get(index).ok_or_else(||invalid("missing retained chunk reference"))?;let entry=self.catalog.chunk(id).map_err(catalog_fault)?;
  if matches!(self.chunk_segment.preflight(),Err(Admission::Complete)){
   if !found||!verified||seen!=entry.raw_len{return Err(invalid("referenced chunk is incomplete"))}self.chunk_segment.replay()?;self.chunk_offset=0;self.chunk_complete=false;self.chunk_hash=semio_framework_hash::Hasher::new();self.chunks=Chunks::Read{index:index+1,end,seen:0,found:false,verified:false};return Ok(None)
  }
  let before=self.chunk_offset;let event=segment_one(&mut self.chunk_segment,input,&mut self.chunk_offset,&mut self.chunk_complete,maximum)?;self.input_work=self.input_work.checked_add(self.chunk_offset-before).ok_or_else(capacity)?;
  match event{
   Some(Event::Begin(segment))if segment.kind==crate::KIND_CHUNK&&segment.payload_offset==entry.offset=>{if found||segment.stored_len!=entry.stored_len||segment.raw_len!=entry.raw_len{return Err(invalid("referenced chunk framing disagrees with catalog"))}self.chunks=Chunks::Read{index,end,seen,found:true,verified:false}},
   Some(Event::RawByte{segment,value,..})if segment.kind==crate::KIND_CHUNK&&segment.payload_offset==entry.offset=>{if !found||seen>=entry.raw_len{return Err(invalid("referenced chunk exceeds declared raw length"))}self.chunk_hash.update(&[value]);self.chunks=Chunks::Read{index,end,seen:seen+1,found,verified};return Ok(Some(Token::Byte(value)))},
   Some(Event::Complete{segment,..})if segment.kind==crate::KIND_CHUNK&&segment.payload_offset==entry.offset=>{if self.chunk_segment.verified_payload_crc32c(segment)!=Some(entry.crc32){return Err(invalid("referenced chunk checksum disagrees with catalog"))}if seen!=entry.raw_len||self.chunk_hash.finalize().as_bytes()!=&entry.blake3{return Err(invalid("referenced chunk hash disagrees with catalog"))}self.chunks=Chunks::Read{index,end,seen,found,verified:true}},
   _=>{}
  }Ok(None)
 }
}
impl Driver for DocumentDriver{
 fn new(limits:PackLimits,length:usize,maximum:usize)->Result<Self,PackRefusal>{
  let mut segment=Segment::try_new(limits.clone(),maximum)?;let mut chunk_segment=match Segment::try_new(limits.clone(),maximum){Ok(cursor)=>cursor,Err(error)=>{while segment.close_step(1,0)!=Close::Complete{}return Err(error)}};
  let symbols=length.min(limits.max_symbols as usize);let chunks=length.min(usize::try_from(limits.max_items).unwrap_or(usize::MAX));
  let mut catalog=match Catalog::try_new(limits.clone(),symbols,length,length,chunks,maximum){Ok(cursor)=>cursor,Err(error)=>{while segment.close_step(1,0)!=Close::Complete{}while chunk_segment.close_step(1,0)!=Close::Complete{}return Err(catalog_fault(error))}};
  let value=match Value::try_new_intrinsic(limits,maximum){Ok(cursor)=>cursor,Err(error)=>{while catalog.close_step(1,0).map_err(paged)?!=Close::Complete{}while segment.close_step(1,0)!=Close::Complete{}while chunk_segment.close_step(1,0)!=Close::Complete{}return Err(error)}};
  Ok(Self{anchor:Anchor::new(),segment,chunk_segment,catalog,value,ids:Default::default(),phase:Phase::Anchor,offset:0,chunk_offset:0,input_work:0,source_complete:false,chunk_complete:false,superblock:None,verified:None,event:None,document_byte:None,body_ingress:0,sealed:false,schema_index:0,schema_hash:semio_framework_hash::Hasher::new(),chunks:Chunks::None,pending_id:None,chunk_hash:semio_framework_hash::Hasher::new(),closed:false})
 }
 fn one(&mut self,input:&[u8],maximum:usize)->Result<Option<Token>,PackRefusal>{
  match self.phase{
   Phase::Anchor=>{let event=if self.offset<input.len(){let event=Source::Byte{offset:self.offset as u64,value:input[self.offset]};self.offset+=1;self.input_work+=1;Some(event)}else if !self.source_complete{self.source_complete=true;Some(Source::Complete{bytes:self.offset as u64,pages:0})}else{None};if self.anchor.grant(event)?{self.superblock=self.anchor.take();self.phase=Phase::Verify;self.offset=0;self.source_complete=false}Ok(None)},
   Phase::Verify=>{
    if self.catalog.has_pending_input(){if let Some(bytes)=self.catalog.next_allocation_bytes().map_err(catalog_fault)?{if bytes>maximum{return Err(capacity())}self.catalog.reserve_allocation(bytes).map_err(|error|catalog_fault(error.fault))?;}else{self.catalog.grant().map_err(catalog_fault)?;}return Ok(None)}
    if let Some(event)=self.event.take(){self.catalog.admit(event).map_err(|_|invalid("catalog segment handback"))?;return Ok(None)}
    if self.catalog.progress().complete{self.verified=self.catalog.take(self.superblock.take().ok_or_else(||invalid("missing verified document anchor"))?).map_err(catalog_fault)?;if self.verified.is_none(){return Err(invalid("catalog has no verified manifest"))}self.phase=Phase::Schema;return Ok(None)}
    let before=self.offset;self.event=segment_one(&mut self.segment,input,&mut self.offset,&mut self.source_complete,maximum)?;self.input_work+=self.offset-before;Ok(None)
   },
   Phase::Schema=>{if self.schema_index<SCHEMA_GRAPH.len(){self.schema_hash.update(&SCHEMA_GRAPH[self.schema_index..self.schema_index+1]);self.schema_index+=1;return Ok(None)}let manifest=self.verified.as_ref().unwrap().manifest;if manifest.field_count!=1||self.schema_hash.finalize().as_bytes()!=&manifest.schema_hash{return Err(invalid("intrinsic document schema mismatch"))}self.segment.replay()?;self.offset=0;self.source_complete=false;self.phase=Phase::Replay;Ok(None)},
   Phase::Replay=>{let token=self.replay_one(input,maximum)?;if matches!(token,Some(Token::Complete{..})){self.phase=Phase::Complete}Ok(token)},
   Phase::Complete=>Ok(None),
  }
 }
 fn allocated_bytes(&self)->usize{self.segment.allocated_bytes()+self.chunk_segment.allocated_bytes()+self.catalog.allocated_bytes()+self.value.allocated_bytes()+self.ids.allocated_bytes()}
 fn input_bytes(&self)->usize{self.input_work}
 fn consumed_bytes(&self)->u64{self.value.consumed_bytes()}
 fn body_bytes(&self,_:&[u8])->u64{self.verified.as_ref().map_or(u64::MAX,|catalog|catalog.manifest.uncompressed_body_len)}
 fn symbol_utf8_bytes(&self,symbol:u64)->Result<usize,PackRefusal>{usize::try_from(self.catalog.symbol_span(symbol).map_err(catalog_fault)?.utf8_len).map_err(|_|capacity())}
 fn symbol_chars(&self,symbol:u64)->Result<usize,PackRefusal>{self.catalog.symbol_chars(symbol).map_err(catalog_fault)}
 fn symbol_char(&self,symbol:u64,index:usize)->Result<Option<char>,PackRefusal>{self.catalog.symbol_char(symbol,index).map_err(catalog_fault)}
 fn next_copy_demand(&self)->usize{if self.ids.is_empty(){0}else{size_of::<u64>()}}
 fn next_release_demand(&self)->Result<usize,PackRefusal>{
  if !self.ids.is_empty(){return Ok(0)}if !self.ids.terminal_is_empty(){return self.ids.next_release_allocation_bytes().map_err(paged)}
  if !self.value.terminal_is_empty(){return Ok(self.value.next_release_allocation_bytes().unwrap_or(0))}
  if !self.catalog.terminal_is_empty(){return Ok(self.catalog.next_release_allocation_bytes().map_err(paged)?.unwrap_or(0))}
  if !self.segment.terminal_is_empty(){return Ok(self.segment.next_release_allocation_bytes().unwrap_or(0))}Ok(self.chunk_segment.next_release_allocation_bytes().unwrap_or(0))
 }
 fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,PackRefusal>{
  if !self.ids.is_empty(){if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<u64>(){return Ok(RetainedCloneStep::Progress(Default::default()))}self.ids.pop();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<u64>(),..Default::default()}))}
  if !self.ids.terminal_is_empty(){let step=self.ids.release_empty_page(grant.maximum_release_bytes).map_err(paged)?;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{released_bytes:step.released_allocation_bytes,..Default::default()}))}
  if !self.value.terminal_is_empty(){return Ok(partial_close(self.value.close_step(grant.maximum_items,grant.maximum_release_bytes)?))}
  if !self.catalog.terminal_is_empty(){return Ok(partial_close(self.catalog.close_step(grant.maximum_items,grant.maximum_release_bytes).map_err(paged)?))}
  if !self.segment.terminal_is_empty(){return Ok(partial_close(self.segment.close_step(grant.maximum_items,grant.maximum_release_bytes)))}
  if !self.chunk_segment.terminal_is_empty(){return Ok(partial_close(self.chunk_segment.close_step(grant.maximum_items,grant.maximum_release_bytes)))}
  self.anchor.close_step();self.event=None;self.document_byte=None;self.pending_id=None;self.superblock=None;self.verified=None;self.closed=true;Ok(RetainedCloneStep::Complete(Default::default()))
 }
 fn terminal_is_empty(&self)->bool{self.closed&&self.ids.terminal_is_empty()&&self.value.terminal_is_empty()&&self.catalog.terminal_is_empty()&&self.segment.terminal_is_empty()&&self.chunk_segment.terminal_is_empty()}
}
