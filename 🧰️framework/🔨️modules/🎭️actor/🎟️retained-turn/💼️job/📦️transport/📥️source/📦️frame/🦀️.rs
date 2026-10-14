//! 📦️ Compact immutable source metadata carries semantic extents without retained authority or dynamic storage.
use super::{OriginalJobSource,ValueError,ValueRefusalKind};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct OriginalJobSourceWireHeader{pub job:u64,pub kind_bytes:usize,pub input_bytes:Option<usize>,pub checkpoint_bytes:Option<usize>}
impl OriginalJobSourceWireHeader{
 pub fn from_source(source:&OriginalJobSource)->Self{Self{job:source.job,kind_bytes:source.kind.len(),input_bytes:source.input.as_ref().map(|bytes|bytes.len()),checkpoint_bytes:source.checkpoint.as_ref().map(|bytes|bytes.len())}}
 pub fn extent(self)->Result<usize,ValueError>{40usize.checked_add(self.kind_bytes).and_then(|bytes|bytes.checked_add(self.input_bytes.unwrap_or(0))).and_then(|bytes|bytes.checked_add(self.checkpoint_bytes.unwrap_or(0))).ok_or_else(||invalid("original source frame extent overflow"))}
 pub fn bytes(self)->[u8;40]{let mut bytes=[0;40];bytes[..4].copy_from_slice(b"SJS1");bytes[4]=u8::from(self.input_bytes.is_some())|(u8::from(self.checkpoint_bytes.is_some())<<1);for(index,value)in[self.job,self.kind_bytes as u64,self.input_bytes.unwrap_or(0)as u64,self.checkpoint_bytes.unwrap_or(0)as u64].into_iter().enumerate(){bytes[8+index*8..16+index*8].copy_from_slice(&value.to_le_bytes());}bytes}
 pub fn read(bytes:&[u8],extent:usize)->Result<Self,ValueError>{
  if bytes.len()!=40||&bytes[..4]!=b"SJS1"||bytes[4]>3||bytes[5..8]!=[0;3]{return Err(invalid("original source frame has an invalid header"))}let mut words=[0u64;4];for(index,word)in words.iter_mut().enumerate(){*word=u64::from_le_bytes(bytes[8+index*8..16+index*8].try_into().unwrap());}let length=|index|usize::try_from(words[index]).map_err(|_|invalid("original source frame extent exceeds target address space"));if(bytes[4]&1==0&&words[2]!=0)||(bytes[4]&2==0&&words[3]!=0){return Err(invalid("original source frame absent field has a byte extent"))}let header=Self{job:words[0],kind_bytes:length(1)?,input_bytes:if bytes[4]&1!=0{Some(length(2)?)}else{None},checkpoint_bytes:if bytes[4]&2!=0{Some(length(3)?)}else{None}};if header.extent()?!=extent{return Err(invalid("original source frame differs from its supplied physical extent"))}Ok(header)
 }
 pub fn field_range(self,field:u8)->Result<Option<(usize,usize)>,ValueError>{self.extent()?;Ok(match field{0=>Some((40,self.kind_bytes)),1=>self.input_bytes.map(|length|(40+self.kind_bytes,length)),2=>self.checkpoint_bytes.map(|length|(40+self.kind_bytes+self.input_bytes.unwrap_or(0),length)),_=>None})}
}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
