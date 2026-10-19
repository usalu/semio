//! 🧩️ Byte-addressable canonical container grammar shared by retained original-source producers.
use super::*;

pub fn varint_len(mut value:usize)->usize{let mut size=1;while value>=128{value>>=7;size+=1;}size}
pub fn varint_byte(value:usize,index:usize)->u8{let shift=index*7;((value>>shift)&127)as u8|if index+1<varint_len(value){128}else{0}}
pub fn prefix_len(raw:usize,stored:usize,compressed:bool)->usize{2+varint_len(stored)+if compressed{varint_len(raw)}else{0}}
pub fn prefix_byte(kind:u8,raw:usize,stored:usize,compressed:bool,index:usize)->u8{match index{0=>kind,1=>if compressed{3}else{0},n if n<2+varint_len(stored)=>varint_byte(stored,n-2),n=>varint_byte(raw,n-2-varint_len(stored))}}
pub fn span(raw:usize,stored:usize)->Result<usize,ValueError>{prefix_len(raw,stored,true).checked_add(stored).and_then(|value|value.checked_add(4)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained Pack segment size overflow"))}
pub fn raw_span(length:usize)->Result<usize,ValueError>{prefix_len(length,length,false).checked_add(length).and_then(|value|value.checked_add(4)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained Pack raw segment size overflow"))}
/// 🧱️ One original chunk's standard index metadata carries no payload shadow.
pub struct RetainedChunkEntry{pub offset:usize,pub length:usize,pub crc:u32,pub hash:[u8;32]}
impl RetainedChunkEntry{
 pub fn raw_len(&self)->usize{varint_len(self.offset)+2*varint_len(self.length)+36}
 pub fn byte(&self,mut index:usize)->Result<u8,ValueError>{for value in [self.offset,self.length,self.length]{let size=varint_len(value);if index<size{return Ok(varint_byte(value,index));}index-=size;}if index<4{return Ok(self.crc.to_le_bytes()[index]);}self.hash.get(index-4).copied().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained chunk table entry byte exceeds grammar"))}
}

/// 🪪️ Fixed canonical metadata holds no output, schema graph, symbol string or record shadow.
pub struct RetainedCanonicalLayout{pub raw:[usize;3],pub stored:[usize;3],pub document_span:usize,pub frames:usize,pub chunks:usize,pub chunk_span:usize,pub chunk_table_raw:usize,pub symbols:usize,pub fields:usize,pub schema:[u8;32],pub content:[u8;32]}
impl Default for RetainedCanonicalLayout{fn default()->Self{Self{raw:[0;3],stored:[0;3],document_span:0,frames:0,chunks:0,chunk_span:0,chunk_table_raw:0,symbols:0,fields:0,schema:[0;32],content:[0;32]}}}
impl RetainedCanonicalLayout{
 pub fn chunks_start(&self)->Result<usize,ValueError>{HEADER_SIZE.checked_add(span(self.raw[0],self.stored[0])?).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained Pack chunk offset overflow"))}
 pub fn document_start(&self)->Result<usize,ValueError>{self.chunks_start()?.checked_add(self.chunk_span).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained Pack document offset overflow"))}
 pub fn chunk_table_start(&self)->Result<usize,ValueError>{self.document_start()?.checked_add(self.document_span).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained Pack chunk table offset overflow"))}
 pub fn chunk_table_span(&self)->Result<usize,ValueError>{if self.chunks==0{Ok(0)}else{raw_span(self.chunk_table_raw)}}
 pub fn manifest_start(&self)->Result<usize,ValueError>{self.chunk_table_start()?.checked_add(self.chunk_table_span()?).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained Pack manifest offset overflow"))}
 pub fn total(&self)->Result<usize,ValueError>{self.manifest_start()?.checked_add(span(self.raw[2],self.stored[2])?).and_then(|value|value.checked_add(7+FOOTER_SIZE)).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"retained Pack file length overflow"))}
 pub fn manifest_values(&self)->Result<[usize;13],ValueError>{Ok([self.document_start()?,self.document_span,self.frames,HEADER_SIZE,span(self.raw[0],self.stored[0])?,if self.chunks==0{0}else{self.chunk_table_start()?},self.chunk_table_span()?,0,0,self.raw[1],self.fields,self.chunks,self.symbols])}
 pub fn manifest_len(&self)->Result<usize,ValueError>{Ok(33+self.manifest_values()?.into_iter().map(varint_len).sum::<usize>())}
 pub fn manifest_byte(&self,index:usize)->Result<u8,ValueError>{if index==0{return Ok(0);}if index<33{return Ok(self.schema[index-1]);}let mut offset=index-33;for value in self.manifest_values()?{let size=varint_len(value);if offset<size{return Ok(varint_byte(value,offset));}offset-=size;}Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained manifest byte exceeds declared grammar"))}
 pub fn required_flags(&self)->u32{REQUIRED_COMPRESSED|if self.chunks==0{0}else{REQUIRED_CHUNKED}}
 pub fn header_byte(&self,index:usize,crc:u32)->u8{match index{0..=7=>MAGIC[index],8..=9=>FORMAT_VERSION_MAJOR.to_le_bytes()[index-8],10..=11=>FORMAT_VERSION_MINOR.to_le_bytes()[index-10],12..=15=>self.required_flags().to_le_bytes()[index-12],16..=19=>OPTIONAL_CANONICAL.to_le_bytes()[index-16],20..=23=>crc.to_le_bytes()[index-20],_=>0}}
 pub fn footer_byte(&self,index:usize,crc:u32)->Result<u8,ValueError>{Ok(match index{0..=7=>FOOTER_MAGIC[index],8..=9=>FORMAT_VERSION_MAJOR.to_le_bytes()[index-8],10..=11=>FORMAT_VERSION_MINOR.to_le_bytes()[index-10],12..=15=>self.required_flags().to_le_bytes()[index-12],16..=23=>(self.manifest_start()?as u64).to_le_bytes()[index-16],24..=31=>(span(self.raw[2],self.stored[2])?as u64).to_le_bytes()[index-24],32..=39=>(self.total()?as u64).to_le_bytes()[index-32],40..=71=>self.content[index-40],72..=79=>0,80..=83=>crc.to_le_bytes()[index-80],_=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"retained footer byte exceeds declared grammar"))})}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
