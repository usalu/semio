//! 📖️ Original dictionary lookup borrows one ranked entry and compares one actual key byte per grant.
use super::{Dictionary,Value};
use protocol::value::{ValueError,ValueRefusalKind,RetainedCloneGrant,RetainedCloneProgress};
#[derive(Default,protocol::value::RetireOwned)]
pub struct OriginalDictionaryLookup{binding:Option<(usize,Option<usize>,usize,usize)>,rank:usize,byte:usize,phase:u8,found:Option<usize>}
impl OriginalDictionaryLookup{
 pub fn comparison_pending(&self)->bool{self.phase==1&&self.binding.is_some_and(|binding|self.byte<binding.3)}
 pub fn complete(&self)->bool{self.phase==2}
 pub fn reset(&mut self){*self=Self::default()}
 fn identity(dictionary:&Dictionary,needle:&str)->(usize,Option<usize>,usize,usize){(dictionary as*const Dictionary as usize,dictionary.original_root_identity(),needle.as_ptr()as usize,needle.len())}
 /// 🪜️ Reserves no backing and returns only the real metadata/comparison effects performed.
 pub fn step(&mut self,dictionary:&Dictionary,needle:&str,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),ValueError>{
  let identity=Self::identity(dictionary,needle);if self.binding.is_some_and(|original|original!=identity){return Err(changed())}if self.complete(){return Ok((true,Default::default()))}if grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,Default::default()))}if self.comparison_pending()&&grant.maximum_copy_bytes==0{return Ok((false,Default::default()))}self.binding=Some(identity);let mut receipt=RetainedCloneProgress{copied_items:1,..Default::default()};
  if self.phase==0{let Some((key,_))=dictionary.entry_at_rank(self.rank)else{self.phase=2;return Ok((true,receipt))};if key.len()==needle.len(){self.byte=0;self.phase=1;}else{self.rank+=1;}return Ok((false,receipt))}
  if self.byte==needle.len(){self.found=Some(self.rank);self.phase=2;return Ok((true,receipt))}let(key,_)=dictionary.entry_at_rank(self.rank).ok_or_else(changed)?;receipt.copied_bytes=1;if key.as_bytes()[self.byte]==needle.as_bytes()[self.byte]{self.byte+=1;}else{self.rank+=1;self.byte=0;self.phase=0;}Ok((false,receipt))
 }
 /// 🫴️ Lends the genuine source value after completion; no key or value donor is cloned.
 pub fn result<'a>(&self,dictionary:&'a Dictionary)->Result<Option<&'a Value>,ValueError>{if self.binding.is_some_and(|original|original.0!=dictionary as*const Dictionary as usize||original.1!=dictionary.original_root_identity())||!self.complete(){return Err(changed())}self.found.map(|rank|dictionary.entry_at_rank(rank).map(|(_,value)|value).ok_or_else(changed)).transpose()}
}
fn changed()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original dictionary lookup source or needle changed")}
