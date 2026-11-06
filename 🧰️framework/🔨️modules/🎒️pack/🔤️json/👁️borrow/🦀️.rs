//! 👁️ Native JSON projection borrows typed fields and a guarded external backing.
use super::{JsonWriteNode,JsonWriteSource,JsonWriteEdge};
use semio_framework_value::{DslValue,Number,ValueError,ValueRefusalKind};

/// 🪪️ Context-local data selectors never issue retention or execution authority.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct JsonBorrowKey(pub [u64;4]);

/// 🧩️ An extension's checked projection table avoids dependency edges into its domain owners.
#[derive(Clone,Copy)]
pub struct JsonBorrowProjection{
 pub node:for<'a>fn(&'a dyn std::any::Any,u8,&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>,
 pub child:for<'a>fn(&'a dyn std::any::Any,u8,usize,&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,ValueError>,
 pub edge:fn(&dyn std::any::Any,u8,usize,usize,&dyn JsonBorrowContext)->Result<JsonWriteEdge,ValueError>,
 pub key:for<'a>fn(&'a dyn std::any::Any,u8,usize,&'a dyn JsonBorrowContext)->Result<&'a str,ValueError>,
}

/// 🏛️ The embedding provider retains its actual backing guard throughout one writer slice.
pub trait JsonBorrowContext:std::any::Any{
 fn node(&self,key:JsonBorrowKey)->Result<JsonWriteNode<'_>,ValueError>;
 fn child(&self,key:JsonBorrowKey,index:usize)->Result<JsonBorrowNode<'_>,ValueError>;
 fn key(&self,key:JsonBorrowKey,index:usize)->Result<&str,ValueError>;
 fn edge(&self,_key:JsonBorrowKey,index:usize,_cursor:usize)->Result<JsonWriteEdge,ValueError>{Ok(JsonWriteEdge{child:index,cursor:0})}
}

/// 🌿️ A projection holds a real native owner reference, fixed scalar or guarded-context selector.
#[derive(Clone,Copy)]
pub enum JsonBorrowNode<'a>{Native(&'a dyn JsonNativeBorrow,u8),Projected(&'a dyn std::any::Any,&'static JsonBorrowProjection,u8),Scalar(JsonWriteNode<'a>),Context(JsonBorrowKey)}

/// 🧬️ Derived implementations expose bounded field/index access over their existing wire declarations.
pub trait JsonNativeBorrow{
 fn json_edge(&self,_mode:u8,index:usize,_cursor:usize,_context:&dyn JsonBorrowContext)->Result<JsonWriteEdge,ValueError>{Ok(JsonWriteEdge{child:index,cursor:0})}
 fn json_node<'a>(&'a self,mode:u8,context:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>;
 fn json_child<'a>(&'a self,_mode:u8,_index:usize,_context:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,ValueError>{Err(json_borrow_absent())}
 fn json_key<'a>(&'a self,_mode:u8,_index:usize,_context:&'a dyn JsonBorrowContext)->Result<&'a str,ValueError>{Err(json_borrow_absent())}
}

/// 📭️ Ordinary typed graphs have no external storage to resolve.
pub struct JsonBorrowNoContext;
impl JsonBorrowContext for JsonBorrowNoContext{
 fn node(&self,_:JsonBorrowKey)->Result<JsonWriteNode<'_>,ValueError>{Err(json_borrow_absent())}
 fn child(&self,_:JsonBorrowKey,_:usize)->Result<JsonBorrowNode<'_>,ValueError>{Err(json_borrow_absent())}
 fn key(&self,_:JsonBorrowKey,_:usize)->Result<&str,ValueError>{Err(json_borrow_absent())}
}

pub fn json_borrow_absent()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"native JSON projection field, mode or backing is absent")}
pub fn json_borrow_native<T:JsonNativeBorrow>(value:&T)->JsonBorrowNode<'_>{JsonBorrowNode::Native(value,0)}

/// 🔎️ Its caller keeps both the native root and backing immutable for every resumed encoder slice.
pub struct JsonNativeSource<'a>{pub root:&'a dyn JsonNativeBorrow,pub context:&'a dyn JsonBorrowContext}
impl<'a> JsonBorrowNode<'a>{
 pub fn project_edge(self,index:usize,cursor:usize,context:&dyn JsonBorrowContext)->Result<JsonWriteEdge,ValueError>{match self{Self::Native(value,mode)=>value.json_edge(mode,index,cursor,context),Self::Projected(value,table,mode)=>(table.edge)(value,mode,index,cursor,context),Self::Context(key)=>context.edge(key,index,cursor),Self::Scalar(_)=>Err(json_borrow_absent())}}

 pub fn project_node(self,context:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{match self{Self::Native(value,mode)=>value.json_node(mode,context),Self::Projected(value,table,mode)=>(table.node)(value,mode,context),Self::Scalar(value)=>Ok(value),Self::Context(key)=>context.node(key)}}
 pub fn project_child(self,index:usize,context:&'a dyn JsonBorrowContext)->Result<Self,ValueError>{match self{Self::Native(value,mode)=>value.json_child(mode,index,context),Self::Projected(value,table,mode)=>(table.child)(value,mode,index,context),Self::Context(key)=>context.child(key,index),Self::Scalar(_)=>Err(json_borrow_absent())}}
 pub fn project_key(self,index:usize,context:&'a dyn JsonBorrowContext)->Result<&'a str,ValueError>{match self{Self::Native(value,mode)=>value.json_key(mode,index,context),Self::Projected(value,table,mode)=>(table.key)(value,mode,index,context),Self::Context(key)=>context.key(key,index),Self::Scalar(_)=>Err(json_borrow_absent())}}
}
impl JsonNativeSource<'_>{fn at(&self,path:&[usize])->Result<JsonBorrowNode<'_>,ValueError>{let mut node=JsonBorrowNode::Native(self.root,0);for index in path{node=node.project_child(*index,self.context)?}Ok(node)}}
impl JsonWriteSource for JsonNativeSource<'_>{
 fn traversal_edge_at_path(&self,path:&[usize],index:usize,cursor:usize)->Result<JsonWriteEdge,ValueError>{self.at(path)?.project_edge(index,cursor,self.context)}
 fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{self.at(path)?.project_node(self.context)}
 fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{self.at(path)?.project_key(index,self.context)}
}

macro_rules! unsigned{($($ty:ty),*)=>{$(impl JsonNativeBorrow for $ty{fn json_node<'a>(&'a self,mode:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{if mode!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Number(Number::UInt(*self as u64)))}})*};}
macro_rules! signed{($($ty:ty),*)=>{$(impl JsonNativeBorrow for $ty{fn json_node<'a>(&'a self,mode:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{if mode!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Number(Number::Int(*self as i64)))}})*};}
unsigned!(u8,u16,u32,u64,usize);signed!(i8,i16,i32,i64,isize);
impl JsonNativeBorrow for bool{fn json_node<'a>(&'a self,mode:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{if mode!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Bool(*self))}}
impl JsonNativeBorrow for (){fn json_node<'a>(&'a self,mode:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{if mode!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Null)}}
impl JsonNativeBorrow for f64{fn json_node<'a>(&'a self,mode:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{if mode!=0{return Err(json_borrow_absent())}Ok(if self.is_finite(){JsonWriteNode::Number(Number::Float(*self))}else{JsonWriteNode::Null})}}
impl JsonNativeBorrow for f32{fn json_node<'a>(&'a self,mode:u8,context:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{if mode!=0{return Err(json_borrow_absent())}let _=context;Ok(if self.is_finite(){JsonWriteNode::Float32(*self)}else{JsonWriteNode::Null})}}
impl JsonNativeBorrow for String{fn json_node<'a>(&'a self,mode:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{if mode!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::String(self))}}
impl JsonNativeBorrow for str{fn json_node<'a>(&'a self,mode:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{if mode!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::String(self))}}
macro_rules! indirect{($ty:ty)=>{impl<T:JsonNativeBorrow+?Sized> JsonNativeBorrow for $ty{
 fn json_edge(&self,m:u8,i:usize,cursor:usize,c:&dyn JsonBorrowContext)->Result<JsonWriteEdge,ValueError>{(**self).json_edge(m,i,cursor,c)}
 fn json_node<'a>(&'a self,m:u8,c:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{(**self).json_node(m,c)}
 fn json_child<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,ValueError>{(**self).json_child(m,i,c)}
 fn json_key<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<&'a str,ValueError>{(**self).json_key(m,i,c)}
}};}
indirect!(Box<T>);indirect!(std::sync::Arc<T>);indirect!(&T);
macro_rules! tuple{($length:expr;$($ty:ident:$index:tt),+)=>{impl<$($ty:JsonNativeBorrow),+> JsonNativeBorrow for($($ty,)+){
 fn json_node<'a>(&'a self,m:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{if m!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Array($length))}
 fn json_child<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,ValueError>{if m!=0{return Err(json_borrow_absent())}match i{$($index=>Ok(json_borrow_native(&self.$index)),)+_=>Err(json_borrow_absent())}}
}};}
tuple!(1;A:0);tuple!(2;A:0,B:1);tuple!(3;A:0,B:1,C:2);tuple!(4;A:0,B:1,C:2,D:3);
impl<T:JsonNativeBorrow> JsonNativeBorrow for Option<T>{
 fn json_edge(&self,m:u8,i:usize,cursor:usize,c:&dyn JsonBorrowContext)->Result<JsonWriteEdge,ValueError>{self.as_ref().ok_or_else(json_borrow_absent)?.json_edge(m,i,cursor,c)}
 fn json_node<'a>(&'a self,mode:u8,c:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{match self{Some(v)=>v.json_node(mode,c),None if mode==0=>Ok(JsonWriteNode::Null),_=>Err(json_borrow_absent())}}
 fn json_child<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,ValueError>{self.as_ref().ok_or_else(json_borrow_absent)?.json_child(m,i,c)}
 fn json_key<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<&'a str,ValueError>{self.as_ref().ok_or_else(json_borrow_absent)?.json_key(m,i,c)}
}
impl<T:JsonNativeBorrow> JsonNativeBorrow for Vec<T>{
 fn json_node<'a>(&'a self,mode:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{if mode!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Array(self.len()))}
 fn json_child<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,ValueError>{if m!=0{return Err(json_borrow_absent())}self.get(i).map(json_borrow_native).ok_or_else(json_borrow_absent)}
}
impl<T:JsonNativeBorrow,const N:usize> JsonNativeBorrow for [T;N]{
 fn json_node<'a>(&'a self,mode:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{if mode!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Array(N))}
 fn json_child<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,ValueError>{if m!=0{return Err(json_borrow_absent())}self.get(i).map(json_borrow_native).ok_or_else(json_borrow_absent)}
}
impl<T:JsonNativeBorrow,E:JsonNativeBorrow> JsonNativeBorrow for Result<T,E>{
 fn json_node<'a>(&'a self,mode:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{if mode!=0{return Err(json_borrow_absent())}Ok(JsonWriteNode::Object(1))}
 fn json_child<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,ValueError>{if m!=0||i!=0{return Err(json_borrow_absent())}Ok(match self{Ok(v)=>json_borrow_native(v),Err(v)=>json_borrow_native(v)})}
 fn json_key<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<&'a str,ValueError>{if m!=0||i!=0{return Err(json_borrow_absent())}Ok(if self.is_ok(){"Ok"}else{"Err"})}
}
impl JsonNativeBorrow for DslValue{
 fn json_node<'a>(&'a self,m:u8,_:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,ValueError>{if m!=0{return Err(json_borrow_absent())}Ok(match self{Self::Null=>JsonWriteNode::Null,Self::Bool(v)=>JsonWriteNode::Bool(*v),Self::Number(v)=>JsonWriteNode::Number(*v),Self::String(v)=>JsonWriteNode::String(v),Self::Bytes(v)=>JsonWriteNode::Array(v.len()),Self::Array(v)=>JsonWriteNode::Array(v.len()),Self::Object(v)=>JsonWriteNode::Object(v.len())})}
 fn json_child<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,ValueError>{if m!=0{return Err(json_borrow_absent())}match self{Self::Array(v)=>v.get(i).map(json_borrow_native),Self::Object(v)=>v.get(i).map(|(_,v)|json_borrow_native(v)),Self::Bytes(v)=>v.get(i).map(|v|JsonBorrowNode::Scalar(JsonWriteNode::Number(Number::UInt(u64::from(*v))))),_=>None}.ok_or_else(json_borrow_absent)}
 fn json_key<'a>(&'a self,m:u8,i:usize,_:&'a dyn JsonBorrowContext)->Result<&'a str,ValueError>{if m!=0{return Err(json_borrow_absent())}match self{Self::Object(v)=>v.get(i).map(|(key,_)|key.as_str()),_=>None}.ok_or_else(json_borrow_absent)}
}

#[path="🎟️control/🦀️.rs"]mod control;
