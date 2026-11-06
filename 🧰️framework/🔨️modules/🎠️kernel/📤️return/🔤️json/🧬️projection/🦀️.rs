//! 🧬️ Domain-owned checked tables project native fields without a value mirror.
use super::*;
pub(crate) trait KernelJson:std::any::Any+Sized{
 const JSON:JsonBorrowProjection=JsonBorrowProjection{node:Self::project_node,child:Self::project_child,key:Self::project_key,edge:Self::project_edge};
 fn json_node<'a>(&'a self,mode:u8,context:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>;
 fn json_child<'a>(&'a self,_:u8,_:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{Err(json_borrow_absent())}
 fn json_key<'a>(&'a self,_:u8,_:usize,_:&'a dyn JsonBorrowContext)->Result<&'a str,JsonBorrowError>{Err(json_borrow_absent())}
 fn json_edge(&self,mode:u8,index:usize,_:usize,_:&dyn JsonBorrowContext)->Result<JsonWriteEdge,JsonBorrowError>{if mode!=0{return Err(json_borrow_absent())}Ok(JsonWriteEdge{child:index,cursor:0})}
 fn project_node<'a>(value:&'a dyn std::any::Any,m:u8,c:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{value.downcast_ref::<Self>().ok_or_else(json_borrow_absent)?.json_node(m,c)}
 fn project_child<'a>(value:&'a dyn std::any::Any,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{value.downcast_ref::<Self>().ok_or_else(json_borrow_absent)?.json_child(m,i,c)}
 fn project_key<'a>(value:&'a dyn std::any::Any,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<&'a str,JsonBorrowError>{value.downcast_ref::<Self>().ok_or_else(json_borrow_absent)?.json_key(m,i,c)}
 fn project_edge(value:&dyn std::any::Any,m:u8,i:usize,cursor:usize,c:&dyn JsonBorrowContext)->Result<JsonWriteEdge,JsonBorrowError>{value.downcast_ref::<Self>().ok_or_else(json_borrow_absent)?.json_edge(m,i,cursor,c)}
}
pub(crate) fn project<T:KernelJson>(value:&T)->JsonBorrowNode<'_>{JsonBorrowNode::Projected(value,&T::JSON,0)}
macro_rules! native{($($ty:ty),*)=>{$(impl KernelJson for $ty{
 fn json_node<'a>(&'a self,m:u8,c:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{JsonNativeBorrow::json_node(self,m,c)}
 fn json_child<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{JsonNativeBorrow::json_child(self,m,i,c)}
 fn json_key<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<&'a str,JsonBorrowError>{JsonNativeBorrow::json_key(self,m,i,c)}
 fn json_edge(&self,m:u8,i:usize,cursor:usize,c:&dyn JsonBorrowContext)->Result<JsonWriteEdge,JsonBorrowError>{JsonNativeBorrow::json_edge(self,m,i,cursor,c)}
})*}}
native!((),bool,u8,u16,u32,u64,usize,i8,i16,i32,i64,isize,f32,f64,String,semio_framework_value::DslValue,semio_framework_value::RetainedCloneGrant,semio_framework_value::RetainedCloneProgress);
impl<T:KernelJson> KernelJson for Vec<T>{
 fn json_node<'a>(&'a self,m:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Array(self.len()))}
 fn json_child<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}self.get(i).map(project).ok_or_else(json_borrow_absent)}
}
impl<T:KernelJson,const N:usize> KernelJson for [T;N]{
 fn json_node<'a>(&'a self,m:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Array(N))}
 fn json_child<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}self.get(i).map(project).ok_or_else(json_borrow_absent)}
}
impl<T:KernelJson> KernelJson for Option<T>{
 fn json_node<'a>(&'a self,m:u8,c:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{match self{Some(v)=>v.json_node(m,c),None if m==0=>Ok(JsonWriteNode::Null),_=>Err(json_borrow_absent())}}
 fn json_child<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{self.as_ref().ok_or_else(json_borrow_absent)?.json_child(m,i,c)}
 fn json_key<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<&'a str,JsonBorrowError>{self.as_ref().ok_or_else(json_borrow_absent)?.json_key(m,i,c)}
 fn json_edge(&self,m:u8,i:usize,cursor:usize,c:&dyn JsonBorrowContext)->Result<JsonWriteEdge,JsonBorrowError>{self.as_ref().ok_or_else(json_borrow_absent)?.json_edge(m,i,cursor,c)}
}
macro_rules! indirect{($ty:ty)=>{impl<T:KernelJson> KernelJson for $ty{
 fn json_node<'a>(&'a self,m:u8,c:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{(**self).json_node(m,c)}
 fn json_child<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{(**self).json_child(m,i,c)}
 fn json_key<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<&'a str,JsonBorrowError>{(**self).json_key(m,i,c)}
 fn json_edge(&self,m:u8,i:usize,cursor:usize,c:&dyn JsonBorrowContext)->Result<JsonWriteEdge,JsonBorrowError>{(**self).json_edge(m,i,cursor,c)}
}}}
indirect!(Box<T>);indirect!(std::sync::Arc<T>);
macro_rules! tuple{($length:expr;$($ty:ident:$index:tt),+)=>{impl<$($ty:KernelJson),+> KernelJson for($($ty,)+){
 fn json_node<'a>(&'a self,m:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Array($length))}
 fn json_child<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{if m!=0{return Err(json_borrow_absent())}match i{$($index=>Ok(project(&self.$index)),)+_=>Err(json_borrow_absent())}}
}}}
tuple!(1;A:0);tuple!(2;A:0,B:1);tuple!(3;A:0,B:1,C:2);tuple!(4;A:0,B:1,C:2,D:3);
pub(crate) fn is_default<T:Default+PartialEq>(value:&T)->bool{value==&T::default()}
pub(crate) fn is_false(value:&bool)->bool{!*value}
pub(crate) mod decimal_generation{
 use super::*;
 pub fn borrow_json<'a>(value:&'a u64,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{Ok(JsonBorrowNode::Scalar(JsonWriteNode::QuotedUnsigned(u128::from(*value))))}
}
pub(crate) use decimal_generation as request_sequence;
