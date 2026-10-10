//! 📥️ Budgeted PNG reconstruction with private RGBA publication.
use super::{RasterImage,Ihdr,parse_ihdr,samples_per_pixel,packed_row_bytes,bpp_bytes,ADAM7,adam7_pass_dims,pixel_to_rgba,paeth,crc32_update};
use semio_framework_deflate::{Inflater,InflateOutcome};
use semio_framework_value::{retained_clone::{RetainedCloneGrant,RetainedCloneProgress},value::list::PagedList};
#[derive(Clone,Debug,semio_framework_value::RetireOwned)]
pub struct PngDecodeInput{pub data:Vec<u8>,pub max_pixels:usize,pub max_bytes:usize,pub max_chunks:usize}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct PngDecodeProgress{pub phase:&'static str,pub bytes:usize,pub total_bytes:usize,pub pixels:usize,pub total_pixels:usize,pub work:u64,pub done:bool}
#[derive(Clone,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub enum PngDecodeError{Invalid(&'static str),Incomplete,Cancelled}
impl std::fmt::Display for PngDecodeError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Invalid(v)=>f.write_str(v),Self::Incomplete=>f.write_str("PNG decode is incomplete"),Self::Cancelled=>f.write_str("PNG decode cancelled")}}}
impl std::error::Error for PngDecodeError{}
fn invalid(v:&'static str)->PngDecodeError{PngDecodeError::Invalid(v)}
fn be(data:&[u8],at:usize)->u32{u32::from_be_bytes(data[at..at+4].try_into().unwrap())}
#[derive(semio_framework_value::RetireOwned)]
struct Chunk{kind:[u8;4],start:usize,at:usize,end:usize,crc:u32}
struct PngInflater(Inflater);
impl std::ops::Deref for PngInflater{type Target=Inflater;fn deref(&self)->&Inflater{&self.0}}
impl std::ops::DerefMut for PngInflater{fn deref_mut(&mut self)->&mut Inflater{&mut self.0}}
struct PngInflaterRetirement {owner:Option<PngInflater>}
impl semio_framework_value::retirement::RetireOwned for PngInflater {
 fn retirement(self)->Box<dyn semio_framework_value::retirement::RetirementCursor>{Box::new(PngInflaterRetirement {owner:Some(self)})}
 fn retirement_birth_bytes(&self)->Option<usize>{Some(size_of::<PngInflaterRetirement>())}
 fn controlled_retirement_supported()->bool{true}
}
impl semio_framework_value::retirement::RetirementCursor for PngInflaterRetirement {
 fn close_step(&mut self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->semio_framework_value::retirement::RetirementStep {
  use semio_framework_value::retirement::RetirementStep;
  let Some(owner)=self.owner.as_mut() else{return RetirementStep::Complete;};
  let copy=if owner.next_retained_release_allocation_bytes().is_some(){0}else{size_of::<PngInflater>()};if grant.maximum_items==0||grant.maximum_copy_bytes<copy||grant.maximum_depth==0{return RetirementStep::BudgetExhausted;}
  if owner.retained_terminal_is_empty(){self.owner=None;return RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()});}
  match owner.close_retained_step(grant.maximum_items,grant.maximum_release_bytes){semio_framework_deflate::RetainedInflateCloseStep::Complete=>RetirementStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()}),semio_framework_deflate::RetainedInflateCloseStep::Pending {released_items,released_bytes}=>{if released_items==0&&released_bytes==0{RetirementStep::BudgetExhausted}else{RetirementStep::Progress(RetainedCloneProgress {copied_items:1,copied_bytes:copy,released_bytes,..Default::default()})}}}
 }
 fn terminal_is_empty(&self)->bool{self.owner.is_none()}
 fn next_close_byte_demand(&self)->Option<usize>{Some(self.owner.as_ref().and_then(|owner|owner.next_retained_release_allocation_bytes()).unwrap_or(0))}
 fn next_work_byte_demand(&self)->Result<usize,semio_framework_value::ValueError>{Ok(self.owner.as_ref().map_or(0,|owner|if owner.next_retained_release_allocation_bytes().is_some(){0}else{size_of::<PngInflater>()}))}
 fn next_birth_bytes(&self,_maximum_bytes:usize)->Option<usize>{Some(0)}
 fn terminal_release_bytes(&self)->Option<usize>{Some(size_of::<Self>())}
}
/// 🧩️ Reconstruction consumes one chunk byte, inflated byte, or expanded pixel per work unit.
#[derive(semio_framework_value::RetireOwned)]
pub struct PngDecodeJob{
 source:Vec<u8>,max_pixels:usize,max_chunks:usize,total_bytes:usize,phase:&'static str,work:u64,bytes:usize,pixels:usize,total_pixels:usize,
 at:usize,chunks:usize,chunk:Option<Chunk>,header:Option<Ihdr>,palette:[[u8;3];256],palette_length:usize,alpha:[u8;256],alpha_length:usize,gray_trans:Option<u32>,rgb_trans:Option<(u32,u32,u32)>,had_palette:bool,had_transparency:bool,
 had_idat:bool,closed_idat:bool,ranges:PagedList<(usize,usize),65536>,z_length:usize,range:usize,range_at:usize,z_at:usize,z_header:[u8;2],z_header_length:usize,tail:[u8;4],tail_length:usize,inflater:Option<PngInflater>,pending:Option<u8>,output:RasterImage,allocation_at:u8,
 pass:usize,pass_width:usize,pass_height:usize,sx:u32,sy:u32,dx:u32,dy:u32,row:usize,column:usize,row_at:Option<usize>,row_bytes:usize,stride:usize,bpp:usize,filter:u8,
 current:Vec<u8>,previous:Vec<u8>,raw_bytes:usize,expected_bytes:usize,adler_a:u32,adler_b:u32,stream_done:bool,cancelled:bool,failed:Option<PngDecodeError>,
}
impl PngDecodeJob{
 pub fn new(input:PngDecodeInput)->Result<Self,(PngDecodeError,PngDecodeInput)>{
  if !(1..=16777216).contains(&input.max_pixels)||!(8..=67108864).contains(&input.max_bytes)||!(1..=65536).contains(&input.max_chunks)||input.data.len()<8||input.data.len()>input.max_bytes||input.data[..8]!=[137,80,78,71,13,10,26,10]{return Err((invalid("Invalid PNG input contract"),input));}
  let total_bytes=input.data.len();
  Ok(Self{source:input.data,max_pixels:input.max_pixels,max_chunks:input.max_chunks,total_bytes,phase:"chunks",work:0,bytes:8,pixels:0,total_pixels:0,at:8,chunks:0,chunk:None,header:None,palette:[[0;3];256],palette_length:0,alpha:[0;256],alpha_length:0,gray_trans:None,rgb_trans:None,had_palette:false,had_transparency:false,had_idat:false,closed_idat:false,ranges:PagedList::empty(),z_length:0,range:0,range_at:0,z_at:0,z_header:[0;2],z_header_length:0,tail:[0;4],tail_length:0,inflater:None,pending:None,output:RasterImage::default(),allocation_at:0,pass:0,pass_width:0,pass_height:0,sx:0,sy:0,dx:1,dy:1,row:0,column:0,row_at:None,row_bytes:0,stride:0,bpp:0,filter:0,current:Vec::new(),previous:Vec::new(),raw_bytes:0,expected_bytes:0,adler_a:1,adler_b:0,stream_done:false,cancelled:false,failed:None})
 }
 fn parse_chunk(&mut self,c:Chunk)->Result<(),PngDecodeError>{
  let source=&self.source;let data=&source[c.start..c.end];
  let h=self.header.as_ref().map(|h|(h.width,h.height,h.bit_depth,h.color_type,h.interlace));
  if h.is_none()&&c.kind!=*b"IHDR"{return Err(invalid("IHDR must be first"));}
  if c.kind!=*b"IDAT"&&self.had_idat{self.closed_idat=true;}
  match &c.kind{
   b"IHDR"=>{
    if h.is_some()||self.chunks!=1{return Err(invalid("Duplicate or misplaced IHDR"));}
    if data.len()!=13{return Err(invalid("Invalid PNG header size"));}let width=be(data,0);let height=be(data,4);let depth=data[8];let color=data[9];let valid=match color{0=>matches!(depth,1|2|4|8|16),2|4|6=>matches!(depth,8|16),3=>matches!(depth,1|2|4|8),_=>false};
    if width==0||height==0||width>16384||height>16384||!valid||data[10]!=0||data[11]!=0||data[12]>1{return Err(invalid("Invalid PNG header"));}
    if width as usize*height as usize>self.max_pixels{return Err(invalid("PNG pixel limit exceeded"));}
    let header=parse_ihdr(data).map_err(|_|invalid("Invalid PNG header"))?;self.total_pixels=crate::editing::validate_extent(header.width,header.height).map_err(|_|invalid("Invalid PNG dimensions"))?;
    if self.total_pixels>self.max_pixels{return Err(invalid("PNG pixel limit exceeded"));}self.header=Some(header);
   }
   b"PLTE"=>{
    let color=h.unwrap().3;let depth=h.unwrap().2;
    if self.had_palette||self.had_transparency||self.had_idat||matches!(color,0|4)||data.is_empty()||data.len()>768||data.len()%3!=0||color==3&&data.len()/3>1usize<<depth{return Err(invalid("Invalid PNG palette"));}
    self.palette_length=data.len()/3;for(i,value)in data.chunks_exact(3).enumerate(){self.palette[i]=[value[0],value[1],value[2]];}self.had_palette=true;
   }
   b"tRNS"=>{
    if self.had_transparency||self.had_idat{return Err(invalid("Invalid transparency placement"));}self.had_transparency=true;let color=h.unwrap().3;let depth=h.unwrap().2;
    match color{
     3=>{if !self.had_palette||data.is_empty()||data.len()>self.palette_length{return Err(invalid("Invalid palette transparency"));}self.alpha_length=data.len();self.alpha[..data.len()].copy_from_slice(data);}
     0|2=>{if data.len()!=if color==0{2}else{6}{return Err(invalid("Invalid sample transparency"));}let mut samples=[0u32;3];for (i,pair) in data.chunks_exact(2).enumerate(){samples[i]=u32::from(u16::from_be_bytes([pair[0],pair[1]]));if samples[i]>(1u32<<depth)-1{return Err(invalid("Transparency exceeds sample depth"));}}if color==0{self.gray_trans=Some(samples[0]);}else{self.rgb_trans=Some((samples[0],samples[1],samples[2]));}}
     _=>return Err(invalid("Transparency is forbidden for alpha images")),
    }
   }
   b"IDAT"=>{
    if self.closed_idat||h.unwrap().3==3&&!self.had_palette{return Err(invalid("Invalid IDAT placement"));}
    self.had_idat=true;self.ranges.push_reserved((c.start,c.end)).map_err(|_|invalid("PNG range backing was not admitted"))?;self.z_length+=data.len();
   }
   b"IEND"=>{
    if !data.is_empty()||!self.had_idat||self.at!=self.total_bytes||self.z_length<6{return Err(invalid("Invalid PNG end"));}
    self.phase="allocation";self.range_at=self.ranges.get(0).unwrap().0;let header=self.header.as_ref().unwrap();
    self.output.width=header.width;self.output.height=header.height;
    self.bpp=bpp_bytes(header);self.stride=packed_row_bytes(header.width,header.color_type,header.bit_depth);
    
    for p in 0..if header.interlace==1{7}else{1}{let(w,h)=if header.interlace==1{adam7_pass_dims(header.width,header.height,p)}else{(header.width,header.height)};if w!=0&&h!=0{self.expected_bytes+=(packed_row_bytes(w,header.color_type,header.bit_depth)+1)*h as usize;}}
    self.next_pass();
   }
   _=>if c.kind[0]&32==0{return Err(invalid("Unknown critical PNG chunk"));},
  }
  Ok(())
 }
 fn chunk_step(&mut self,grant:RetainedCloneGrant)->Result<usize,PngDecodeError>{
  let source=&self.source;
  if self.chunk.is_none(){
   self.chunks+=1;if self.at+12>self.total_bytes||self.chunks>self.max_chunks{return Err(invalid("Truncated PNG or chunk limit exceeded"));}
   let length=be(source,self.at)as usize;if length>2147483647{return Err(invalid("Invalid chunk length"));}let end=self.at+8+length;
   if length>2147483647||end+4>self.total_bytes{return Err(invalid("Invalid chunk length"));}
   let kind:[u8;4]=source[self.at+4..self.at+8].try_into().unwrap();if !kind.iter().all(u8::is_ascii_alphabetic){return Err(invalid("Invalid chunk type"));}
   self.chunk=Some(Chunk{kind,start:self.at+8,at:self.at+4,end,crc:0xffffffff});self.bytes=self.at+4;return Ok(0);
  }
  let c=self.chunk.as_mut().unwrap();
  if c.at<c.end{c.crc=crc32_update(c.crc,&source[c.at..c.at+1]);c.at+=1;self.bytes=c.at;return Ok(0);}
  if c.crc^0xffffffff!=be(source,c.end){return Err(invalid("PNG CRC mismatch"));}
  if c.kind==*b"IDAT"&&!self.ranges.has_reserved_slot(){return self.ranges.reserve_one_funded(grant).map(|step|step.retained_capacity_bytes).map_err(|_|invalid("PNG range allocation refused"));}
  self.at=c.end+4;self.bytes=self.at;let c=self.chunk.take().unwrap();self.parse_chunk(c)?;Ok(0)
 }
 fn next_pass(&mut self){
  let h=self.header.as_ref().unwrap();
  while self.pass<if h.interlace==1{7}else{1}{
   let spec=if h.interlace==1{ADAM7[self.pass]}else{(0,0,1,1)};self.pass+=1;(self.sx,self.sy,self.dx,self.dy)=spec;
   let (w,height)=if h.interlace==1{adam7_pass_dims(h.width,h.height,self.pass-1)}else{(h.width,h.height)};
   self.pass_width=w as usize;self.pass_height=height as usize;
   if w!=0&&height!=0{self.row=0;self.row_at=None;self.column=0;self.row_bytes=packed_row_bytes(w,h.color_type,h.bit_depth);return;}
  }
  self.pass_width=0;
 }
 fn z_byte(&mut self)->Option<u8>{
  let &(start,end)=self.ranges.get(self.range)?;
  if self.range_at==end{self.range+=1;self.range_at=self.ranges.get(self.range).map(|r|r.0).unwrap_or(0);return None;}
  debug_assert!(self.range_at>=start);let value=self.source[self.range_at];self.range_at+=1;self.z_at+=1;Some(value)
 }
 fn raw(&mut self,value:u8)->Result<(),PngDecodeError>{
  self.raw_bytes+=1;if self.raw_bytes>self.expected_bytes||self.pass_width==0{return Err(invalid("Excess PNG scanline data"));}
  self.adler_a=(self.adler_a+u32::from(value))%65521;self.adler_b=(self.adler_b+self.adler_a)%65521;
  let Some(i)=self.row_at else{if value>4{return Err(invalid("Invalid PNG filter"));}self.filter=value;self.row_at=Some(0);return Ok(());};
  let left=if i>=self.bpp{self.current[i-self.bpp]}else{0};let up=if self.row!=0{self.previous[i]}else{0};let corner=if self.row!=0&&i>=self.bpp{self.previous[i-self.bpp]}else{0};
  let prediction=match self.filter{0=>0,1=>left,2=>up,3=>((u16::from(left)+u16::from(up))/2)as u8,_=>paeth(left,up,corner)};
  self.current[i]=value.wrapping_add(prediction);self.row_at=Some(i+1);if i+1==self.row_bytes{self.phase="pixels";self.column=0;}Ok(())
 }
 fn pixel_step(&mut self)->Result<(),PngDecodeError>{
  let h=self.header.as_ref().unwrap();let spp=samples_per_pixel(h.color_type);let mut samples=[0u32;4];
  for(c,v)in samples[..spp].iter_mut().enumerate(){let sample=self.column*spp+c;let bit=sample*h.bit_depth as usize;let at=bit/8;*v=match h.bit_depth{16=>u32::from(u16::from_be_bytes([self.current[at],self.current[at+1]])),8=>u32::from(self.current[at]),_=>u32::from(self.current[at]>>(8-h.bit_depth-bit as u8%8))&((1u32<<h.bit_depth)-1)};}
  if h.color_type==3&&samples[0]as usize>=self.palette_length{return Err(invalid("PNG palette sample is out of range"));}
  let rgba=pixel_to_rgba(&samples,h,&self.palette[..self.palette_length],&self.alpha[..self.alpha_length],self.gray_trans,self.rgb_trans).map_err(|_|invalid("PNG palette sample is out of range"))?;
  let at=((self.sy as usize+self.row*self.dy as usize)*h.width as usize+self.sx as usize+self.column*self.dx as usize)*4;
  self.output.pixels[at..at+4].copy_from_slice(&rgba);self.pixels+=1;self.column+=1;
  if self.column==self.pass_width{std::mem::swap(&mut self.current,&mut self.previous);self.row_at=None;self.row+=1;if self.row==self.pass_height{self.next_pass();}self.phase="inflating";}
  Ok(())
 }
 fn inflate_step(&mut self,maximum_capacity_bytes:usize)->Result<usize,PngDecodeError>{
  if self.output.pixels.len()<self.total_pixels*4{self.output.pixels.resize((self.output.pixels.len()+4096).min(self.total_pixels*4),0);return Ok(0);}
  if self.current.len()<self.stride{self.current.resize((self.current.len()+4096).min(self.stride),0);return Ok(0);}
  if self.previous.len()<self.stride{self.previous.resize((self.previous.len()+4096).min(self.stride),0);return Ok(0);}
  if self.z_header_length<2{
   if let Some(b)=self.z_byte(){self.z_header[self.z_header_length]=b;self.z_header_length+=1;}if self.z_header_length==2{let cmf=self.z_header[0];let flg=self.z_header[1];if cmf&15!=8||cmf>>4>7||u16::from_be_bytes([cmf,flg])%31!=0||flg&32!=0{return Err(invalid("Invalid PNG zlib header"));}self.inflater=Some(PngInflater(Inflater::try_new_retained(1usize<<((cmf>>4)+8),32768).map_err(|_|invalid("Invalid PNG DEFLATE stream"))?));}return Ok(0);
  }
  if self.stream_done{
   if self.tail_length<4{if let Some(b)=self.z_byte(){self.tail[self.tail_length]=b;self.tail_length+=1;}return Ok(0);}
   if self.z_at!=self.z_length||be(&self.tail,0)!=((self.adler_b<<16)|self.adler_a)||self.raw_bytes!=self.expected_bytes||self.pass_width!=0||self.pixels!=self.total_pixels{return Err(invalid("PNG stream size or Adler mismatch"));}
   self.phase="complete";return Ok(0);
  }
  if self.inflater.as_ref().unwrap().next_retained_allocation_bytes().is_some(){return self.inflater.as_mut().unwrap().reserve_retained_history(maximum_capacity_bytes).map(|step|step.allocated_bytes).map_err(|_|invalid("PNG DEFLATE allocation refused"));}
  if self.pending.is_none()&&self.z_at<self.z_length-4{let Some(b)=self.z_byte()else{return Ok(0);};self.pending=Some(b);}
  let result=self.inflater.as_mut().unwrap().advance(&mut self.pending,self.z_at==self.z_length-4).map_err(|_|invalid("Invalid PNG DEFLATE stream"))?;
  match result{
   InflateOutcome::Wrote(value)=>self.raw(value)?,
   InflateOutcome::Done=>{if self.pending.is_some()||self.z_at!=self.z_length-4||self.inflater.as_ref().unwrap().terminal_unused_bytes()!=Some(0){return Err(invalid("Trailing DEFLATE data"));}self.stream_done=true;}
   InflateOutcome::NeedInput=>{},
  }
  Ok(0)
 }
 fn check(&self)->Result<(),PngDecodeError>{if self.cancelled{return Err(PngDecodeError::Cancelled);}if self.phase=="transferred"{return Err(PngDecodeError::Incomplete);}if let Some(e)=&self.failed{return Err(e.clone());}Ok(())}
 fn allocation_step(&mut self)->Result<usize,PngDecodeError>{
  let allocated=match self.allocation_at{0=>{self.output.pixels.try_reserve_exact(self.total_pixels*4).map_err(|_|invalid("PNG output allocation refused"))?;self.output.pixels.capacity()},1=>{self.current.try_reserve_exact(self.stride).map_err(|_|invalid("PNG row allocation refused"))?;self.current.capacity()},2=>{self.previous.try_reserve_exact(self.stride).map_err(|_|invalid("PNG previous row allocation refused"))?;self.previous.capacity()},_=>{self.phase="inflating";0}};self.allocation_at+=1;Ok(allocated)
 }
 fn pending_range_allocation(&self)->bool{self.phase=="chunks"&&self.chunk.as_ref().is_some_and(|chunk|chunk.at==chunk.end&&chunk.kind==*b"IDAT")&&!self.ranges.has_reserved_slot()}
 pub fn next_copy_byte_demand(&self)->Result<usize,PngDecodeError>{
  if matches!(self.phase,"complete"|"transferred"){return Ok(0);}if self.pending_range_allocation(){return self.ranges.next_reserve_copy_byte_demand().map(|copy|copy+32).map_err(|_|invalid("PNG range copy demand refused"));}
  if self.phase=="chunks"{return Ok(32+self.chunk.as_ref().filter(|chunk|chunk.at==chunk.end&&matches!(&chunk.kind,b"PLTE"|b"tRNS")).map_or(0,|chunk|(chunk.end-chunk.start).min(768)));}
  if self.phase=="inflating"{if self.output.pixels.len()<self.total_pixels*4{return Ok(32+(self.total_pixels*4-self.output.pixels.len()).min(4096));}if self.current.len()<self.stride{return Ok(32+(self.stride-self.current.len()).min(4096));}if self.previous.len()<self.stride{return Ok(32+(self.stride-self.previous.len()).min(4096));}return Ok(size_of::<PngInflater>().max(32));}Ok(32)
 }
 pub fn next_capacity_byte_demand(&self)->Result<usize,PngDecodeError>{
  if self.pending_range_allocation(){return self.ranges.next_allocation_bytes().map_err(|_|invalid("PNG range capacity demand refused"));}
  if self.phase=="allocation"{return Ok(match self.allocation_at{0=>self.total_pixels*4,1|2=>self.stride,_=>0});}
  Ok(if self.phase=="inflating"{self.inflater.as_ref().and_then(|owner|owner.next_retained_allocation_bytes()).unwrap_or(0)}else{0})
 }
 pub fn progress(&self)->PngDecodeProgress{PngDecodeProgress{phase:self.phase,bytes:self.bytes,total_bytes:self.total_bytes,pixels:self.pixels,total_pixels:self.total_pixels,work:self.work,done:self.phase=="complete"}}
 pub fn source(&self)->&[u8]{&self.source}
 pub fn advance(&mut self,grant:RetainedCloneGrant)->Result<(PngDecodeProgress,RetainedCloneProgress),PngDecodeError>{
  self.check()?;let mut receipt=RetainedCloneProgress::default();
  for _ in 0..grant.maximum_items{if self.phase=="complete"{break;}let copy=self.next_copy_byte_demand()?;let capacity=self.next_capacity_byte_demand()?;if grant.maximum_depth==0||copy>grant.maximum_copy_bytes.saturating_sub(receipt.copied_bytes)||capacity>grant.maximum_capacity_bytes.saturating_sub(receipt.retained_capacity_bytes){break;}
   let result=match self.phase{"chunks"=>self.chunk_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:grant.maximum_capacity_bytes-receipt.retained_capacity_bytes,maximum_release_bytes:0,maximum_depth:grant.maximum_depth}),"allocation"=>self.allocation_step(),"pixels"=>self.pixel_step().map(|_|0),_=>self.inflate_step(grant.maximum_capacity_bytes-receipt.retained_capacity_bytes)};let allocated=match result{Ok(value)=>value,Err(error)=>{self.failed=Some(error.clone());return Err(error);}};self.work+=1;receipt.copied_items+=1;receipt.copied_bytes+=copy;receipt.retained_capacity_bytes+=allocated;
  }
  Ok((self.progress(),receipt))
 }
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn result(&self)->Result<&RasterImage,PngDecodeError>{self.check()?;if self.phase!="complete"{return Err(PngDecodeError::Incomplete);}Ok(&self.output)}
 pub fn take_result(&mut self,grant:RetainedCloneGrant)->Result<Option<(RasterImage,RetainedCloneProgress)>,PngDecodeError>{self.check()?;if self.phase!="complete"{return Err(PngDecodeError::Incomplete);}if grant.maximum_items==0||grant.maximum_copy_bytes<size_of::<RasterImage>()||grant.maximum_depth==0{return Ok(None);}self.phase="transferred";Ok(Some((std::mem::take(&mut self.output),RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<RasterImage>(),..Default::default()})))}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
