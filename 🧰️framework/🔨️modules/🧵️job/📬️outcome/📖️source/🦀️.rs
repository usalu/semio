//! 📖️ Fresh immutable page reads preserve the actual sealed payload header and paid ledger identity.
use super::*;
impl RetainedJobPayload{
 pub fn original_wire_identity(&self)->(usize,usize,Option<(u64,u64)>){(self as*const Self as usize,self.len(),self.ledger.as_ref().map(|ledger|(ledger.operation.0,ledger.generation.0)))}
 pub fn original_byte_at(&self,index:usize)->Option<u8>{if index>=self.len(){return None}let mut offset=index;for page in self.pages.iter().flatten(){let bytes=page.bytes();if offset<bytes.len(){return bytes.get(offset).copied()}offset-=bytes.len();}None}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
