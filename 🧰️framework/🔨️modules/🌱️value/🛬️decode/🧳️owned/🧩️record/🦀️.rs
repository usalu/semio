//! 🧩️ Native record keys and borrowed schema descriptors preserve genuine original member identities.
#[derive(Clone,Copy)]pub enum NativeRecordKey<'a>{Name(&'a str),Ordinal(u16)}
impl NativeRecordKey<'_>{pub fn comparison_bytes(self)->usize{match self{Self::Name(name)=>name.len(),Self::Ordinal(_)=>size_of::<u16>()}}}
#[derive(Clone,Copy)]pub struct NativeRecordField{pub name:&'static str,pub ordinal:u16}
impl NativeRecordField{pub fn matches(self,key:NativeRecordKey<'_>)->bool{match key{NativeRecordKey::Name(name)=>self.name==name,NativeRecordKey::Ordinal(ordinal)=>self.ordinal==ordinal}}}

#[path="🔎️schema/🦀️.rs"]mod schema;
pub use schema::{NativeSchemaLeaf,NativeSchemaKeySource,NativeRecordSchema,NativeFieldMatch,NativeRecordMatcher};

#[path="📦️storage/🦀️.rs"]mod storage;
pub use storage::NativeRecordStorage;

