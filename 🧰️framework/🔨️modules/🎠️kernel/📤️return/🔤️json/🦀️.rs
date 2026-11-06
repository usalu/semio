//! 📤️ Bounded JSON traversal over a retained kernel result and its actual UI arena.
use semio_framework_pack_json::{JsonBorrowContext,JsonBorrowKey,JsonBorrowNode,JsonBorrowProjection,JsonBorrowError,JsonNativeBorrow,JsonWriteSource,JsonWriteNode,JsonWriteEdge,json_borrow_absent};
use semio_framework_kernel::TurnResult;
use semio_ui_contract::{UiArenaBorrow,ui_value_arena_registry};
#[path="🧬️projection/🦀️.rs"]mod projection;
use projection::{KernelJson,project,is_default,is_false,decimal_generation,request_sequence};
#[path="🎠️kernel/🦀️.rs"]mod kernel;
#[path="🖼️ui/🦀️.rs"]mod ui;
#[path="🖼️ui/🏛️collections/🦀️.rs"]mod collections;
struct KernelJsonContext{arena:UiArenaBorrow}
impl JsonBorrowContext for KernelJsonContext{
 fn node(&self,_:JsonBorrowKey)->Result<JsonWriteNode<'_>,JsonBorrowError>{Err(json_borrow_absent())}
 fn child(&self,_:JsonBorrowKey,_:usize)->Result<JsonBorrowNode<'_>,JsonBorrowError>{Err(json_borrow_absent())}
 fn key(&self,_:JsonBorrowKey,_:usize)->Result<&str,JsonBorrowError>{Err(json_borrow_absent())}
}
/// 🏛️ Holds the genuine UI backing guard for one synchronous paid encoder slice.
pub struct KernelTurnJsonSource<'a>{result:&'a TurnResult,context:KernelJsonContext}
impl<'a> KernelTurnJsonSource<'a>{
 pub fn try_borrow(result:&'a TurnResult)->Result<Option<Self>,JsonBorrowError>{ui_value_arena_registry().borrow().map(|arena|arena.map(|arena|Self{result,context:KernelJsonContext{arena}})).map_err(|_|json_borrow_absent())}
 fn at(&self,path:&[usize])->Result<JsonBorrowNode<'_>,JsonBorrowError>{let Some((&first,tail))=path.split_first()else{return Ok(JsonBorrowNode::Scalar(JsonWriteNode::Object(1)))};if first!=0{return Err(json_borrow_absent())}let mut node=project(self.result);for index in tail{node=node.project_child(*index,&self.context)?}Ok(node)}
}
impl JsonWriteSource for KernelTurnJsonSource<'_>{
 fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,JsonBorrowError>{self.at(path)?.project_node(&self.context)}
 fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,JsonBorrowError>{if path.is_empty(){if index==0{Ok("Ok")}else{Err(json_borrow_absent())}}else{self.at(path)?.project_key(index,&self.context)}}
 fn traversal_edge_at_path(&self,path:&[usize],index:usize,cursor:usize)->Result<JsonWriteEdge,JsonBorrowError>{if path.is_empty(){if index==0{Ok(JsonWriteEdge{child:0,cursor:0})}else{Err(json_borrow_absent())}}else{self.at(path)?.project_edge(index,cursor,&self.context)}}
}
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
