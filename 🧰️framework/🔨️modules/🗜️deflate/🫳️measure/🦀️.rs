//! 📏️ Streaming fixed-Huffman measurement retains only the canonical matching window.
use super::*;

/// 🗜️ Counts the ordinary greedy codec without constructing raw or compressed output buffers.
pub struct DeflateMeasure{
 head:[u32;HASH_SIZE],previous:[u32;WINDOW_SIZE],bytes:[u8;WINDOW_SIZE+512],
 received:usize,position:usize,bits:usize,work:usize,
}
impl DeflateMeasure{
 /// 🌱️ Inline storage has the same empty chains and final fixed-block header as the producer.
 pub fn new()->Self{Self{head:[u32::MAX;HASH_SIZE],previous:[u32::MAX;WINDOW_SIZE],bytes:[0;WINDOW_SIZE+512],received:0,position:0,bits:3,work:0}}
 /// 🧼️ Starts another independently compressed segment with the same inline window.
 pub fn reset(&mut self){self.head.fill(u32::MAX);self.previous.fill(u32::MAX);self.received=0;self.position=0;self.bits=3;self.work=0;}
 fn at(&self,position:usize)->u8{self.bytes[position%self.bytes.len()]}
 /// 📥️ Consumes one borrowed byte, retaining at most 260 bytes of lookahead.
 pub fn push(&mut self,byte:u8,checkpoint:&mut impl FnMut()->Result<(),ValueError>)->Result<(),ValueError>{
  if self.received>=u32::MAX as usize{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Deflate measurement exceeds position width"))}
  self.bytes[self.received%(WINDOW_SIZE+512)]=byte;self.received+=1;
  while self.received-self.position>=MAX_MATCH+2{self.consume(checkpoint)?;}Ok(())
 }
 fn tick(&mut self,checkpoint:&mut impl FnMut()->Result<(),ValueError>)->Result<(),ValueError>{self.work=self.work.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Deflate measurement work overflow"))?;if self.work%256==0{checkpoint()?;}Ok(())}
 fn consume(&mut self,checkpoint:&mut impl FnMut()->Result<(),ValueError>)->Result<(),ValueError>{
  let i=self.position;let available=self.received-i;let(mut best,mut distance)=(0,0);
  if available>=MIN_MATCH{let hash=hash3(self.at(i),self.at(i+1),self.at(i+2));let mut candidate=self.head[hash];let mut chain=0;
   while candidate!=u32::MAX&&chain<MAX_CHAIN{self.tick(checkpoint)?;let position=candidate as usize;if i-position>WINDOW_SIZE{break}let mut length=0;while length<available.min(MAX_MATCH){self.tick(checkpoint)?;if self.at(position+length)!=self.at(i+length){break}length+=1;}if length>best{best=length;distance=i-position;}candidate=self.previous[position%WINDOW_SIZE];chain+=1;}
  }
  let consumed=if best>=MIN_MATCH{let length=length_index_for(best as u16);let symbol=257+length;let literal_bits=if symbol<=279{7}else{8};let dist=distance_index_for(distance as u32);self.bits=self.bits.checked_add(literal_bits+usize::from(LENGTH_EXTRA[length])+5+usize::from(DIST_EXTRA[dist])).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Deflate measured bit count overflow"))?;best}else{self.bits=self.bits.checked_add(if self.at(i)<=143{8}else{9}).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Deflate measured literal count overflow"))?;1};
  let end=i+consumed;while self.position<end{let position=self.position;if self.received-position>=MIN_MATCH{let hash=hash3(self.at(position),self.at(position+1),self.at(position+2));self.previous[position%WINDOW_SIZE]=self.head[hash];self.head[hash]=position as u32;}self.position+=1;self.tick(checkpoint)?;}Ok(())
 }
 /// 🏁️ Finishes the same final end-of-block symbol and byte padding as ordinary Deflate.
 pub fn finish(&mut self,checkpoint:&mut impl FnMut()->Result<(),ValueError>)->Result<usize,ValueError>{while self.position<self.received{self.consume(checkpoint)?;}checkpoint()?;self.bits.checked_add(7).and_then(|bits|bits.checked_add(7)).map(|bits|bits/8).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Deflate measured byte count overflow"))}
}
impl Default for DeflateMeasure{fn default()->Self{Self::new()}}
