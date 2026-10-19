//! 🎒️ Retained canonical codec1 documents borrow original fields per turn and lend only paid output bytes.
use crate::{BorrowedProjectedPackCursor,BorrowedPackSection};
use semio_framework_pack_record_preflight::BorrowedSchemaHashCursor;
use semio_framework_replication_codec::{RetainedDeflateEncoder,Crc32cCursor};
use semio_framework_pack_format::retained_encoding::{self as grammar,RetainedCanonicalLayout,RetainedChunkEntry};
use semio_framework_dsl_record::{BorrowedRecordSpec,native_encoding::FieldProjectionSource};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetirementDemand,ValueError,ValueRefusalKind};
use semio_framework_value::list::{PagedList,PagedListError};
#[derive(Clone,Copy)]
struct FrameExtent{raw:usize,stored:usize}

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
enum Phase{ContentBirth,Schema,Segment,FrameCatalog,ChunkBirth,ChunkRead,ChunkHash,ChunkCatalog,Header,Prefix,Checksum,ChunkPrefix,ChunkEmit,ChunkChecksum,TablePrefix,TableBody,TableChecksum,End,EndChecksum,Footer,Complete,Closing,Closed}

/// 🎟️ The sole physical receipt and initialized final output prefix from one document turn.
#[derive(Default,Debug)]
pub struct BorrowedPackDocumentProgress{pub progress:RetainedCloneProgress,pub written_bytes:usize,pub complete:bool}
/// ⚠️ Original refusal authority retains every physical currency without discarding final output custody.
#[derive(Debug)]
pub struct BorrowedPackDocumentFailure{pub reason:ValueError,pub written_bytes:usize}
impl From<ValueError> for BorrowedPackDocumentFailure{fn from(reason:ValueError)->Self{Self{reason,written_bytes:0}}}

/// 🧵️ Three segment passes retain bounded compression and source locators instead of raw/output shadows.
pub struct BorrowedPackDocumentCursor{
 source:BorrowedProjectedPackCursor,schema:BorrowedSchemaHashCursor,deflate:RetainedDeflateEncoder,
 phase:Phase,identity:Option<usize>,spec:Option<BorrowedRecordSpec>,layout:RetainedCanonicalLayout,
 measure:bool,segment:usize,offset:usize,raw:usize,need_raw:bool,raw_done:bool,pending:Option<u8>,hash_pending:bool,compressed:Option<u8>,
 crc:Crc32cCursor,content:PagedList<semio_framework_hash::Hasher,1>,written:usize,maximum_file:usize,maximum_body:usize,frame_size:usize,frames:PagedList<FrameExtent,{isize::MAX as usize}>,frame_index:usize,
 chunk_entries:PagedList<RetainedChunkEntry,{isize::MAX as usize}>,chunk_index:usize,chunk_offset:usize,payload_crc:Crc32cCursor,
}
impl Default for BorrowedPackDocumentCursor{fn default()->Self{Self::with_maximum_file_len(usize::MAX)}}
fn invalid(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
fn page_error(reason:PagedListError)->ValueError{ValueError::literal(ValueRefusalKind::OwnershipLimit,reason.reason)}
impl BorrowedPackDocumentCursor{
 /// 📏️ Keeps the caller's final output ceiling while preserving canonical compression and framing.
 pub fn with_maximum_file_len(maximum_file:usize)->Self{Self{source:BorrowedProjectedPackCursor::for_document(),schema:Default::default(),deflate:Default::default(),phase:Phase::ContentBirth,identity:None,spec:None,layout:Default::default(),measure:true,segment:0,offset:0,raw:0,need_raw:false,raw_done:false,pending:None,hash_pending:false,compressed:None,crc:Crc32cCursor::new(),content:PagedList::empty(),written:0,maximum_file,maximum_body:usize::MAX,frame_size:1048576,frames:PagedList::empty(),frame_index:0,chunk_entries:PagedList::empty(),chunk_index:0,chunk_offset:0,payload_crc:Crc32cCursor::new()}}
 /// 🛡️ Captures the original uncompressed Body ceiling independently of compressed output custody.
 pub fn with_maximum_body_len(mut self,maximum_body:usize)->Self{self.maximum_body=maximum_body;self}
 /// 🪜️ Limits actual original body paths independently of schema work and caller structural grants.
 pub fn with_maximum_source_depth(mut self,maximum:usize)->Self{self.source.set_maximum_source_depth(maximum);self}
 /// 🎛️ Admits authored canonical codec1 frame boundaries while keeping the producer's original-source ownership.
 pub fn with_options(options:&semio_framework_replication_codec::PackEncodeOptions)->Result<Self,ValueError>{if options.codec.0!=1||!options.canonical||options.chunk_threshold!=262144||options.chunk_size!=1048576{return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner,"retained Pack requires authored canonical codec1 chunk options"));}let mut cursor=Self::with_maximum_file_len(usize::try_from(options.limits.max_file_len).unwrap_or(usize::MAX));cursor.frame_size=usize::try_from(options.frame_size.max(1)).map_err(|_|invalid("retained Pack frame exceeds address space"))?;Ok(cursor)}
 pub fn is_complete(&self)->bool{self.phase==Phase::Complete}
 pub fn terminal_is_empty(&self)->bool{self.phase==Phase::Closed&&self.source.terminal_is_empty()&&self.schema.terminal_is_empty()&&self.deflate.terminal_is_empty()&&self.frames.terminal_is_empty()&&self.content.terminal_is_empty()&&self.chunk_entries.terminal_is_empty()}
 pub fn output_len(&self)->Option<usize>{if self.measure{None}else{self.layout.total().ok()}}
 pub fn emitted_bytes(&self)->usize{self.written}
 fn segment_kind(&self)->u8{[crate::KIND_SYMBOLS,crate::KIND_DOCUMENT,crate::KIND_MANIFEST][self.segment]}
 fn source_boundary(&self)->bool{match self.segment{0=>matches!(self.source.section(),BorrowedPackSection::Body|BorrowedPackSection::Complete),1=>self.source.is_complete()||self.raw==self.frame_size,_=>self.raw==self.layout.manifest_len().unwrap_or(usize::MAX)}}
 fn extent(&self)->FrameExtent{if self.segment==1{self.frames[self.frame_index]}else{FrameExtent{raw:self.layout.raw[self.segment],stored:self.layout.stored[self.segment]}}}
 fn demands(&self)->Result<(usize,usize,usize),ValueError>{Ok(match self.phase{
  Phase::ContentBirth if !self.content.has_reserved_slot()=>(self.content.next_allocation_bytes().map_err(page_error)?,self.content.next_reserve_copy_byte_demand().map_err(page_error)?,self.content.next_reserve_depth_demand().map_err(page_error)?+1),
  Phase::ContentBirth=>(0,std::mem::size_of::<semio_framework_hash::Hasher>(),self.content.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::Schema=>(self.schema.next_capacity_byte_demand()?,self.schema.next_copy_byte_demand()?,self.schema.next_depth_demand()?+1),
  Phase::FrameCatalog if !self.frames.has_reserved_slot()=>(self.frames.next_allocation_bytes().map_err(page_error)?,self.frames.next_reserve_copy_byte_demand().map_err(page_error)?,self.frames.next_reserve_depth_demand().map_err(page_error)?+1),
  Phase::FrameCatalog=>(0,std::mem::size_of::<FrameExtent>()+if self.source.is_complete(){32}else{0},self.frames.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::ChunkBirth=>(0,if self.chunk_index<self.source.chunk_count(){std::mem::size_of::<semio_framework_hash::Hasher>()}else{0},self.content.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::ChunkRead=>(0,usize::from(self.chunk_offset<self.source.chunk_len(self.chunk_index)?),self.source.chunk_depth(self.chunk_index)?+1),
  Phase::ChunkHash=>(0,2,self.content.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::ChunkCatalog if !self.chunk_entries.has_reserved_slot()=>(self.chunk_entries.next_allocation_bytes().map_err(page_error)?,self.chunk_entries.next_reserve_copy_byte_demand().map_err(page_error)?,self.chunk_entries.next_reserve_depth_demand().map_err(page_error)?+1),
  Phase::ChunkCatalog=>(0,std::mem::size_of::<RetainedChunkEntry>()+32,self.chunk_entries.next_pop_depth_demand().map_err(page_error)?.max(self.content.next_pop_depth_demand().map_err(page_error)?)+1),
  Phase::ChunkPrefix=>(0,if self.offset<grammar::prefix_len(self.chunk_entries[self.chunk_index].length,self.chunk_entries[self.chunk_index].length,false){2}else{0},self.chunk_entries.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::ChunkEmit if self.compressed.is_some()=>(0,2,1),
  Phase::ChunkEmit=>(0,usize::from(self.chunk_offset<self.source.chunk_len(self.chunk_index)?),self.source.chunk_depth(self.chunk_index)?+1),
  Phase::ChunkChecksum|Phase::TableChecksum=>(0,usize::from(self.offset<4),1),
  Phase::TablePrefix=>(0,if self.offset<grammar::prefix_len(self.layout.chunk_table_raw,self.layout.chunk_table_raw,false){2}else{0},1),
  Phase::TableBody=>(0,if self.offset<self.layout.chunk_table_raw{2}else{0},self.chunk_entries.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::Segment if self.hash_pending=>(0,1,self.content.next_pop_depth_demand().map_err(page_error)?+1),
  Phase::Segment if self.compressed.is_some()=>(0,2,1),
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
 fn compressor_bookkeeping_copy(&self)->usize{std::mem::size_of::<bool>()+usize::from(!self.measure&&self.deflate.phase().as_str()=="output")*std::mem::size_of::<Option<u8>>()}
 fn start_segment(&mut self)->Result<(),ValueError>{self.deflate.restart()?;self.raw=0;self.need_raw=false;self.raw_done=false;self.pending=None;self.hash_pending=false;self.compressed=None;Ok(())}
 fn fixed(&mut self,byte:u8,hash:bool,output:&mut[u8])->BorrowedPackDocumentProgress{if output.is_empty(){return Default::default();}output[0]=byte;if hash{self.crc.update_page(&[byte]);}self.offset+=1;self.written+=1;BorrowedPackDocumentProgress{progress:RetainedCloneProgress{copied_items:1,copied_bytes:1+usize::from(hash),..Default::default()},written_bytes:1,complete:false}}
 fn finish_segment(&mut self)->Result<(),ValueError>{
  if self.measure{
   if self.segment==1{self.phase=Phase::FrameCatalog;return Ok(());}self.layout.raw[self.segment]=self.raw;self.layout.stored[self.segment]=self.deflate.output_len();
   if self.segment==0{self.layout.symbols=self.source.symbol_count().ok_or_else(||invalid("retained Pack symbols have no complete authority"))?;}
   if self.segment<2{self.segment+=1;self.start_segment()?;}else{
    let total=self.layout.total()?;if total>self.maximum_file{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"retained Pack final output exceeds caller ceiling"));}
    self.measure=false;self.segment=0;self.source.restart_emission()?;self.start_segment()?;self.phase=Phase::Header;self.offset=0;self.crc=Crc32cCursor::new();
   }
  }else{let extent=self.extent();if self.raw!=extent.raw||self.deflate.output_len()!=extent.stored{return Err(invalid("retained Pack original source changed its measured segment"));}self.phase=Phase::Checksum;self.offset=0;}
  Ok(())
 }
 /// ✍️ Performs one bounded original-source, compressor, framing or final-output custody event.
 pub fn advance(&mut self,source:&dyn FieldProjectionSource,spec:BorrowedRecordSpec,output:&mut[u8],grant:RetainedCloneGrant)->Result<BorrowedPackDocumentProgress,BorrowedPackDocumentFailure>{
  if matches!(self.phase,Phase::Closing|Phase::Closed){return Err(invalid("retained Pack document is closing").into());}
  if self.is_complete(){return Ok(BorrowedPackDocumentProgress{complete:true,..Default::default()});}
  if self.identity.is_some_and(|id|id!=source.projection_identity())||self.spec.is_some_and(|old|old.fields.as_ptr()!=spec.fields.as_ptr()||old.fields.len()!=spec.fields.len()){return Err(invalid("retained Pack immutable original source authority changed").into());}
  let(capacity,copy,depth)=self.demands()?;if grant.maximum_items==0||grant.maximum_capacity_bytes<capacity||grant.maximum_copy_bytes<copy||grant.maximum_depth<depth{return Ok(Default::default());}
  self.identity=Some(source.projection_identity());self.spec=Some(spec);
  let mut progress=RetainedCloneProgress{copied_items:1,..Default::default()};
  match self.phase{
   Phase::ContentBirth=>{if !self.content.has_reserved_slot(){progress=self.content.reserve_one_funded(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map_err(|failure|page_error(failure.refusal()).with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:failure.allocated_bytes,..Default::default()}))?;}else{self.content.push_reserved(semio_framework_hash::Hasher::new()).map_err(|_|invalid("retained document hash has no admitted slot"))?;progress.copied_bytes=std::mem::size_of::<semio_framework_hash::Hasher>();self.phase=Phase::Schema;}},
   Phase::Schema=>{let step=self.schema.advance(spec,RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant})?;progress=step.progress;if step.complete{self.layout.schema=self.schema.digest().ok_or_else(||invalid("retained schema digest is absent"))?;self.phase=Phase::Segment;}},
   Phase::FrameCatalog=>{
    if !self.frames.has_reserved_slot(){progress=self.frames.reserve_one_funded(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map_err(|failure|page_error(failure.refusal()).with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:failure.allocated_bytes,..Default::default()}))?;}
    else{let extent=FrameExtent{raw:self.raw,stored:self.deflate.output_len()};self.frames.push_reserved(extent).map_err(|_|invalid("retained Pack frame has no admitted slot"))?;progress.copied_bytes=std::mem::size_of::<FrameExtent>();self.layout.frames+=1;self.layout.raw[1]=self.layout.raw[1].checked_add(self.raw).ok_or_else(||invalid("retained Pack body length overflow"))?;self.layout.document_span=self.layout.document_span.checked_add(grammar::span(extent.raw,extent.stored)?).ok_or_else(||invalid("retained Pack document span overflow"))?;
     if self.source.is_complete(){self.layout.fields=self.source.field_count().ok_or_else(||invalid("retained Pack body has no complete authority"))?;self.layout.content=*self.content[0].finalize().as_bytes();progress.copied_bytes+=32;if self.source.chunk_count()!=0{self.chunk_index=0;self.layout.chunk_table_raw=grammar::varint_len(self.source.chunk_count());self.phase=Phase::ChunkBirth;return Ok(BorrowedPackDocumentProgress{progress,..Default::default()});}self.segment=2;}self.start_segment()?;self.phase=Phase::Segment;
    }
   },
   Phase::ChunkBirth=>{if self.chunk_index==self.source.chunk_count(){self.segment=2;self.start_segment()?;self.phase=Phase::Segment;}else{self.content[0]=semio_framework_hash::Hasher::new();progress.copied_bytes=std::mem::size_of::<semio_framework_hash::Hasher>();self.payload_crc=Crc32cCursor::new();self.chunk_offset=0;self.phase=Phase::ChunkRead;}},
   Phase::ChunkRead=>{if self.chunk_offset==self.source.chunk_len(self.chunk_index)?{self.phase=Phase::ChunkCatalog;}else{self.pending=Some(self.source.chunk_byte(source,self.chunk_index,self.chunk_offset)?);progress.copied_bytes=1;self.phase=Phase::ChunkHash;}},
   Phase::ChunkHash=>{let byte=self.pending.take().ok_or_else(||invalid("retained chunk hash lost original byte"))?;self.content[0].update(&[byte]);self.payload_crc.update_page(&[byte]);progress.copied_bytes=2;self.chunk_offset+=1;self.phase=Phase::ChunkRead;},
   Phase::ChunkCatalog=>{
    if !self.chunk_entries.has_reserved_slot(){progress=self.chunk_entries.reserve_one_funded(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map_err(|failure|page_error(failure.refusal()).with_retained_progress(RetainedCloneProgress{retained_capacity_bytes:failure.allocated_bytes,..Default::default()}))?;}
    else{let length=self.source.chunk_len(self.chunk_index)?;let offset=self.layout.chunks_start()?.checked_add(self.layout.chunk_span).and_then(|value|value.checked_add(grammar::prefix_len(length,length,false))).ok_or_else(||invalid("retained chunk payload offset overflow"))?;let entry=RetainedChunkEntry{offset,length,crc:self.payload_crc.finish(),hash:*self.content[0].finalize().as_bytes()};self.layout.chunk_table_raw=self.layout.chunk_table_raw.checked_add(entry.raw_len()).ok_or_else(||invalid("retained chunk table size overflow"))?;self.chunk_entries.push_reserved(entry).map_err(|_|invalid("retained chunk entry has no admitted backing"))?;progress.copied_bytes=std::mem::size_of::<RetainedChunkEntry>()+32;self.layout.chunk_span=self.layout.chunk_span.checked_add(grammar::raw_span(length)?).ok_or_else(||invalid("retained chunk span overflow"))?;self.layout.chunks+=1;self.chunk_index+=1;self.phase=Phase::ChunkBirth;}
   },
   Phase::Segment if self.hash_pending=>{self.content[0].update(&[self.pending.ok_or_else(||invalid("retained Pack body hash lost raw custody"))?]);self.hash_pending=false;progress.copied_bytes=1;},
   Phase::Segment if self.compressed.is_some()=>{if output.is_empty(){return Ok(Default::default());}let byte=self.compressed.take().unwrap();self.offset=0;return Ok(self.fixed(byte,true,output));},
   Phase::Segment if self.deflate.is_complete()=>{self.finish_segment()?;},
   Phase::Segment if self.need_raw=>{
    if self.source_boundary(){self.raw_done=true;self.need_raw=false;}
    else if self.segment==2{self.pending=Some(self.layout.manifest_byte(self.raw)?);self.raw+=1;self.need_raw=false;progress.copied_bytes=1;}
    else{
     let mut byte=[0;1];let child_grant=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,maximum_copy_bytes:grant.maximum_copy_bytes.min(self.source.next_minimum_copy_bytes().max(1)),..grant};
     let step=self.source.advance(source,spec,&mut byte,child_grant).map_err(|failure|BorrowedPackDocumentFailure{reason:failure.reason,written_bytes:0})?;progress=step.progress;
     if step.written_bytes!=0{self.pending=Some(byte[0]);self.raw=self.raw.checked_add(1).ok_or_else(||invalid("retained Pack raw length overflow"))?;self.need_raw=false;self.hash_pending=self.measure&&self.segment==1;if self.measure&&self.segment==1&&self.layout.raw[1].checked_add(self.raw).is_none_or(|length|length>self.maximum_body){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"retained Pack original body exceeds caller ceiling").with_retained_progress(progress).into());}}
    }
   },
   Phase::Segment=>{let bookkeeping=self.compressor_bookkeeping_copy();let mut byte=[0;1];let step=self.deflate.advance(&mut self.pending,self.raw_done,&mut byte,RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,maximum_copy_bytes:grant.maximum_copy_bytes-bookkeeping,..grant})?;progress=step.progress;self.need_raw=step.need_input;if step.written_bytes!=0&&!self.measure{self.compressed=Some(byte[0]);}progress.copied_items=progress.copied_items.max(1);progress.copied_bytes+=bookkeeping;},
   Phase::Header=>{if self.offset==semio_framework_pack_format::HEADER_SIZE{self.phase=Phase::Prefix;self.offset=0;self.crc=Crc32cCursor::new();}else{return Ok(self.fixed(self.layout.header_byte(self.offset,self.crc.finish()),self.offset<20,output));}},
   Phase::Prefix=>{let extent=self.extent();if self.offset==grammar::prefix_len(extent.raw,extent.stored,true){self.phase=Phase::Segment;self.offset=0;}else{return Ok(self.fixed(grammar::prefix_byte(self.segment_kind(),extent.raw,extent.stored,true,self.offset),true,output));}},
   Phase::Checksum=>{if self.offset<4{return Ok(self.fixed(self.crc.finish().to_le_bytes()[self.offset],false,output));}if self.segment==0&&self.layout.chunks!=0{self.chunk_index=0;self.phase=Phase::ChunkPrefix;}else if self.segment==1&&self.frame_index+1<self.frames.len(){self.frame_index+=1;self.start_segment()?;self.phase=Phase::Prefix;}else if self.segment==1&&self.layout.chunks!=0{self.phase=Phase::TablePrefix;}else if self.segment<2{self.segment+=1;self.start_segment()?;self.phase=Phase::Prefix;}else{self.phase=Phase::End;}self.offset=0;self.crc=Crc32cCursor::new();},
   Phase::ChunkPrefix=>{let length=self.chunk_entries[self.chunk_index].length;if self.offset<grammar::prefix_len(length,length,false){return Ok(self.fixed(grammar::prefix_byte(crate::KIND_CHUNK,length,length,false,self.offset),true,output));}self.chunk_offset=0;self.phase=Phase::ChunkEmit;},
   Phase::ChunkEmit=>{if let Some(byte)=self.compressed{if output.is_empty(){return Ok(Default::default());}self.compressed=None;self.chunk_offset+=1;return Ok(self.fixed(byte,true,output));}if self.chunk_offset==self.source.chunk_len(self.chunk_index)?{self.phase=Phase::ChunkChecksum;self.offset=0;}else{self.compressed=Some(self.source.chunk_byte(source,self.chunk_index,self.chunk_offset)?);progress.copied_bytes=1;}},
   Phase::ChunkChecksum=>{if self.offset<4{return Ok(self.fixed(self.crc.finish().to_le_bytes()[self.offset],false,output));}self.chunk_index+=1;if self.chunk_index<self.layout.chunks{self.phase=Phase::ChunkPrefix;}else{self.segment=1;self.start_segment()?;self.phase=Phase::Prefix;}self.offset=0;self.crc=Crc32cCursor::new();},
   Phase::TablePrefix=>{let length=self.layout.chunk_table_raw;if self.offset<grammar::prefix_len(length,length,false){return Ok(self.fixed(grammar::prefix_byte(crate::KIND_CHUNK_TABLE,length,length,false,self.offset),true,output));}self.offset=0;self.chunk_offset=0;self.chunk_index=0;self.phase=Phase::TableBody;},
   Phase::TableBody=>{if self.offset==self.layout.chunk_table_raw{self.offset=0;self.phase=Phase::TableChecksum;}else{let count_bytes=grammar::varint_len(self.layout.chunks);let entry=self.offset>=count_bytes;let byte=if entry{self.chunk_entries[self.chunk_index].byte(self.chunk_offset)?}else{grammar::varint_byte(self.layout.chunks,self.offset)};let step=self.fixed(byte,true,output);if step.written_bytes!=0&&entry{self.chunk_offset+=1;if self.chunk_offset==self.chunk_entries[self.chunk_index].raw_len(){self.chunk_index+=1;self.chunk_offset=0;}}return Ok(step);}},
   Phase::TableChecksum=>{if self.offset<4{return Ok(self.fixed(self.crc.finish().to_le_bytes()[self.offset],false,output));}self.segment=2;self.start_segment()?;self.phase=Phase::Prefix;self.offset=0;self.crc=Crc32cCursor::new();},
   Phase::End=>{if self.offset<3{return Ok(self.fixed(grammar::prefix_byte(crate::KIND_END,0,0,false,self.offset),true,output));}self.phase=Phase::EndChecksum;self.offset=0;},
   Phase::EndChecksum=>{if self.offset<4{return Ok(self.fixed(self.crc.finish().to_le_bytes()[self.offset],false,output));}self.phase=Phase::Footer;self.offset=0;self.crc=Crc32cCursor::new();},
   Phase::Footer=>{if self.offset<semio_framework_pack_format::FOOTER_SIZE{return Ok(self.fixed(self.layout.footer_byte(self.offset,self.crc.finish())?,self.offset<80,output));}if self.written!=self.layout.total()?{return Err(invalid("retained Pack final output disagrees with measured exact length").into());}self.phase=Phase::Complete;},
   _=>return Err(invalid("retained Pack document work frontier is invalid").into()),
  }
  #[cfg(debug_assertions)]
  if progress==RetainedCloneProgress::default(){eprintln!("[DEBUG] funded Pack document idle phase={} measure={} segment={} need-raw={} source-section={:?} source-min={} source-cap={:?} source-depth={:?} deflate-phase={} quoted-copy={} quoted-cap={} quoted-depth={} grant={grant:?}",self.phase as u8,self.measure,self.segment,self.need_raw,self.source.section(),self.source.next_minimum_copy_bytes(),self.source.next_capacity_byte_demand(),self.source.next_advance_depth_demand(),self.deflate.phase().as_str(),copy,capacity,depth);}
  Ok(BorrowedPackDocumentProgress{progress,written_bytes:0,complete:self.is_complete()})
 }
 /// 🍂️ Quotes and closes the original child owners individually; output custody stays with its caller.
 pub fn retirement_demands(&self)->Result<RetirementDemand,ValueError>{let mut demand=if !self.source.terminal_is_empty(){self.source.retirement_demands()?}else if !self.schema.terminal_is_empty(){self.schema.retirement_demands()?}else if !self.deflate.terminal_is_empty(){self.deflate.retirement_demands()?}else if !self.frames.terminal_is_empty(){RetirementDemand{release_bytes:if self.frames.is_empty(){self.frames.next_release_allocation_bytes().map_err(page_error)?}else{0},depth:if self.frames.is_empty(){self.frames.next_release_depth_demand()}else{self.frames.next_pop_depth_demand()}.map_err(page_error)?,..Default::default()}}else if !self.chunk_entries.terminal_is_empty(){RetirementDemand{release_bytes:if self.chunk_entries.is_empty(){self.chunk_entries.next_release_allocation_bytes().map_err(page_error)?}else{0},depth:if self.chunk_entries.is_empty(){self.chunk_entries.next_release_depth_demand()}else{self.chunk_entries.next_pop_depth_demand()}.map_err(page_error)?,..Default::default()}}else if !self.content.terminal_is_empty(){RetirementDemand{release_bytes:if self.content.is_empty(){self.content.next_release_allocation_bytes().map_err(page_error)?}else{0},depth:if self.content.is_empty(){self.content.next_release_depth_demand()}else{self.content.next_pop_depth_demand()}.map_err(page_error)?,..Default::default()}}else{RetirementDemand::default()};demand.depth+=1;Ok(demand)}
 pub fn close(&mut self,grant:RetainedCloneGrant)->Result<BorrowedPackDocumentProgress,BorrowedPackDocumentFailure>{
  if self.terminal_is_empty(){return Ok(BorrowedPackDocumentProgress{complete:true,..Default::default()});}let demand=self.retirement_demands()?;if grant.maximum_items==0||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Ok(Default::default());}
  self.phase=Phase::Closing;let child=RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant};let progress=if !self.source.terminal_is_empty(){self.source.close(child).map_err(|failure|BorrowedPackDocumentFailure{reason:failure.reason,written_bytes:0})?.progress}else if !self.schema.terminal_is_empty(){self.schema.close(child)?.progress}else if !self.deflate.terminal_is_empty(){match self.deflate.close_step(child)?{RetainedCloneStep::Progress(receipt)|RetainedCloneStep::Complete(receipt)=>receipt,_=>return Err(invalid("retained Deflate close lost physical receipt").into())}}else if !self.frames.terminal_is_empty(){let released=if self.frames.pop().is_some(){0}else{self.frames.release_empty_page(child.maximum_release_bytes).map_err(page_error)?.released_allocation_bytes};RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()}}else if !self.chunk_entries.terminal_is_empty(){let released=if self.chunk_entries.pop().is_some(){0}else{self.chunk_entries.release_empty_page(child.maximum_release_bytes).map_err(page_error)?.released_allocation_bytes};RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()}}else if !self.content.terminal_is_empty(){let released=if self.content.pop().is_some(){0}else{self.content.release_empty_page(child.maximum_release_bytes).map_err(page_error)?.released_allocation_bytes};RetainedCloneProgress{copied_items:1,released_bytes:released,..Default::default()}}else{self.phase=Phase::Closed;self.identity=None;self.spec=None;RetainedCloneProgress{copied_items:1,..Default::default()}};
  Ok(BorrowedPackDocumentProgress{progress,written_bytes:0,complete:self.terminal_is_empty()})
 }
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
