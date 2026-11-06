//! 🏛️ Opaque stores expose their actual pages, text and fixed fields without cloned iterators.
use super::*;
use semio_ui_contract::*;
use semio_framework_value::{Number,JsonInteger,json_integer};

impl KernelJson for UiText{
 fn json_node<'a>(&'a self,m:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::String(self.as_str()))}
}
impl KernelJson for UiValue{
 fn json_node<'a>(&'a self,m:u8,c:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}match self{
  Self::Null=>Ok(JsonWriteNode::Null),Self::Bool(v)=>Ok(JsonWriteNode::Bool(*v)),Self::Number(v)=>Ok(match json_integer(*v){Some(JsonInteger::Unsigned(v))=>JsonWriteNode::Number(Number::UInt(v)),Some(JsonInteger::Signed(v))=>JsonWriteNode::Number(Number::Int(v)),None if v.is_finite()=>JsonWriteNode::Number(Number::Float(*v)),None=>JsonWriteNode::Null}),Self::Text(v)=>v.json_node(0,c),Self::List(v)=>v.json_node(0,c),Self::Map(v)=>v.json_node(0,c),
 }}
 fn json_child<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}match self{Self::List(v)=>v.json_child(0,i,c),Self::Map(v)=>v.json_child(0,i,c),_=>Err(json_borrow_absent())}}
 fn json_key<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<&'a str,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}match self{Self::Map(v)=>v.json_key(0,i,c),_=>Err(json_borrow_absent())}}
 fn json_edge(&self,m:u8,i:usize,cursor:usize,c:&dyn JsonBorrowContext)->Result<JsonWriteEdge,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}match self{Self::List(v)=>v.json_edge(0,i,cursor,c),Self::Map(v)=>v.json_edge(0,i,cursor,c),_=>Err(json_borrow_absent())}}
}
fn arena(context:&dyn JsonBorrowContext)->Result<&UiArenaBorrow,JsonBorrowError>{Ok(&(context as &dyn std::any::Any).downcast_ref::<KernelJsonContext>().ok_or_else(json_borrow_absent)?.arena)}
fn collection(handle:Option<UiCollectionHandle>,len:usize,c:&dyn JsonBorrowContext)->Result<usize,JsonBorrowError>{match handle{Some(handle)=>{let(head,items)=arena(c)?.borrowed_collection(handle).ok_or_else(json_borrow_absent)?;if items!=len{return Err(json_borrow_absent())}Ok(head)},None if len==0=>Ok(UI_VALUE_NONE),None=>Err(json_borrow_absent())}}
fn page<'a>(handle:Option<UiCollectionHandle>,index:usize,c:&'a dyn JsonBorrowContext)->Result<(usize,&'a UiPageValue),JsonBorrowError>{arena(c)?.borrowed_page(handle.ok_or_else(json_borrow_absent)?,index).ok_or_else(json_borrow_absent)}
fn edge(handle:Option<UiCollectionHandle>,len:usize,index:usize,cursor:usize,c:&dyn JsonBorrowContext)->Result<JsonWriteEdge,JsonBorrowError>{if index>=len{return Err(json_borrow_absent())}let child=if index==0{collection(handle,len,c)?}else{cursor};let(next,_)=page(handle,child,c)?;if(index+1==len)!=(next==UI_VALUE_NONE){return Err(json_borrow_absent())}Ok(JsonWriteEdge{child,cursor:next})}
impl KernelJson for UiList{
 fn json_node<'a>(&'a self,m:u8,c:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}collection(self.handle,self.len(),c)?;Ok(JsonWriteNode::Array(self.len()))}
 fn json_child<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}let(_,UiPageValue::List(value))=page(self.handle,i,c)? else{return Err(json_borrow_absent())};Ok(project(value))}
 fn json_edge(&self,m:u8,i:usize,cursor:usize,c:&dyn JsonBorrowContext)->Result<JsonWriteEdge,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}edge(self.handle,self.len(),i,cursor,c)}
}
impl KernelJson for UiMap{
 fn json_node<'a>(&'a self,m:u8,c:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}collection(self.handle,self.len(),c)?;Ok(JsonWriteNode::Object(self.len()))}
 fn json_child<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}let(_,UiPageValue::Map(_,value))=page(self.handle,i,c)? else{return Err(json_borrow_absent())};Ok(project(value))}
 fn json_key<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<&'a str,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}let(_,UiPageValue::Map(key,_))=page(self.handle,i,c)? else{return Err(json_borrow_absent())};Ok(key.as_str())}
 fn json_edge(&self,m:u8,i:usize,cursor:usize,c:&dyn JsonBorrowContext)->Result<JsonWriteEdge,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}edge(self.handle,self.len(),i,cursor,c)}
}
impl<T:KernelJson,const N:usize> KernelJson for UiFixedList<T,N>{
 fn json_node<'a>(&'a self,m:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Array(self.len()))}
 fn json_child<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}self.get(i).map(project).ok_or_else(json_borrow_absent)}
}
impl<T:KernelJson> KernelJson for UiFixedMap<T>{
 fn json_node<'a>(&'a self,m:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Object(self.len()))}
 fn json_child<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}self.get(i).map(|(_,v)|project(v)).ok_or_else(json_borrow_absent)}
 fn json_key<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<&'a str,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}self.get(i).map(|(k,_)|k.as_str()).ok_or_else(json_borrow_absent)}
}
impl KernelJson for UiFixedBytes{
 fn json_node<'a>(&'a self,m:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Array(self.len()))}
 fn json_child<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}self.as_slice().get(i).map(|v|JsonBorrowNode::Scalar(JsonWriteNode::Number(Number::UInt(u64::from(*v))))).ok_or_else(json_borrow_absent)}
}
impl KernelJson for UiPatchOps{
 fn json_node<'a>(&'a self,m:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Array(self.len()))}
 fn json_child<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}self.get(i).map(project).ok_or_else(json_borrow_absent)}
}
impl KernelJson for semio_framework_kernel_ui::UiTurnPatches{
 fn json_node<'a>(&'a self,m:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Array(self.len()))}
 fn json_child<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}self.get(i).map(project).ok_or_else(json_borrow_absent)}
}
macro_rules! decimal{($($ty:ty),*)=>{$(impl KernelJson for $ty{
 fn json_node<'a>(&'a self,m:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::QuotedUnsigned(self.0 as u128))}
})*}}
decimal!(semio_framework_kernel_identity::ArtifactHandle,semio_framework_kernel_identity::WindowHandle,UiPublicationRevision);
