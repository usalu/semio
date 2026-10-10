//! 🌳️ Original scalar frames retain their high-water pages while borrowed source nodes are visited.
use super::{SemioValue,ValueSqliteTables,References,number};
use store::sqlite_snapshot::{SqliteSnapshotControl,artifact::{Cell,RowWriter}};
use semio_framework_value::{ValueError,ValueRefusalKind,list::PagedList};
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}

#[derive(Clone,Copy,semio_framework_value::RetireOwned)]
struct Frame{ordinal:usize,children:usize,next:usize,id:i64}

/// 🪜️ Logical traversal depth changes without releasing or replacing any original page.
#[derive(semio_framework_value::RetireOwned)]
pub(crate)struct TreeScratch{frames:PagedList<Frame,{usize::MAX}>,depth:usize}
impl TreeScratch{
 pub(crate)fn empty()->Self{Self{frames:PagedList::new(),depth:0}}
 fn push(&mut self,frame:Frame,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
  if self.depth==self.frames.len(){
   while !self.frames.has_reserved_slot(){let quote=self.frames.next_allocation_bytes().map_err(|_|invalid("value projection page quote"))?;let remaining=control.allocation_remaining_bytes();control.admit_allocation_bytes(quote)?;let progress=self.frames.reserve_one(remaining).map_err(|_|invalid("value projection page admission"))?;if progress.allocated_bytes!=quote{return Err(invalid("value projection page extent"))}}
   self.frames.push_reserved(frame).map_err(|_|invalid("value projection frame slot"))?;
  }else{self.frames[self.depth]=frame;}
  self.depth+=1;Ok(())
 }
 fn source<'a>(&self,root:&'a SemioValue,out:&mut RowWriter<'_,'_>)->Result<&'a SemioValue,ValueError>{let mut current=root;for depth in 1..self.depth{out.checkpoint()?;current=child(current,self.frames[depth].ordinal)?.0;}Ok(current)}
}

fn count(value:&SemioValue)->usize{match value{SemioValue::List{items}=>items.len(),SemioValue::Map{entries}=>entries.len(),_=>0}}
fn child(value:&SemioValue,ordinal:usize)->Result<(&SemioValue,Option<&str>),ValueError>{match value{SemioValue::List{items}=>items.get(ordinal).map(|value|(value,None)),SemioValue::Map{entries}=>entries.get(ordinal).map(|entry|(&entry.value,Some(entry.key.as_str()))),_=>None}.ok_or_else(||invalid("value projection scalar source address"))}

/// 🍃️ Projects each actual original variant and ordered relationship without owning borrowed references.
pub(crate)fn visit(root:&SemioValue,tables:ValueSqliteTables,names:Option<&dyn References>,scratch:&mut TreeScratch,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 if scratch.depth!=0{return Err(invalid("value projection traversal is already live"))}
 scratch.push(Frame{ordinal:0,children:count(root),next:0,id:0},out.control())?;
 let(mut current,mut key)=(root,None);let mut root_id=0;
 loop{
  out.checkpoint()?;let mut cells=[Cell::Null;7];
  cells[0]=Cell::Text(match current{SemioValue::Null=>"null",SemioValue::Bool{..}=>"bool",SemioValue::Int{..}=>"int",SemioValue::Float{..}=>"float",SemioValue::Str{..}=>"str",SemioValue::Bytes{..}=>"bytes",SemioValue::List{..}=>"list",SemioValue::Map{..}=>"map",SemioValue::Ref{..}=>"ref"});
  match current{SemioValue::Bool{value}=>cells[1]=Cell::Integer(i64::from(*value)),SemioValue::Int{lexeme}=>cells[2]=Cell::Text(lexeme),SemioValue::Float{lexeme}=>cells[3]=Cell::Text(lexeme),SemioValue::Str{value}=>cells[4]=Cell::Text(value),SemioValue::Bytes{value}=>cells[5]=Cell::Blob(value),SemioValue::Ref{id}=>cells[6]=match names{Some(names)=>Cell::Integer(names.lookup(&id.value,out)?),None=>Cell::Text(&id.value)},_=>{}}
  let id=out.insert(tables.value,&cells)?;scratch.frames[scratch.depth-1].id=id;
  if scratch.depth==1{root_id=id;}else{let parent=scratch.frames[scratch.depth-2].id;let ordinal=number(scratch.frames[scratch.depth-1].ordinal)?;if let Some(key)=key{out.insert(tables.map_entry,&[Cell::Integer(parent),Cell::Integer(ordinal),Cell::Text(key),Cell::Integer(id)])?;}else{out.insert(tables.list_element,&[Cell::Integer(parent),Cell::Integer(ordinal),Cell::Integer(id)])?;}}
  if count(current)>0{
   scratch.frames[scratch.depth-1].next=1;let(next,next_key)=child(current,0)?;scratch.push(Frame{ordinal:0,children:count(next),next:0,id:0},out.control())?;current=next;key=next_key;continue;
  }
  loop{
   scratch.depth-=1;if scratch.depth==0{return Ok(root_id)}
   let frame=scratch.frames[scratch.depth-1];if frame.next==frame.children{continue}
   scratch.frames[scratch.depth-1].next+=1;let parent=scratch.source(root,out)?;let(next,next_key)=child(parent,frame.next)?;scratch.push(Frame{ordinal:frame.next,children:count(next),next:0,id:0},out.control())?;current=next;key=next_key;break;
  }
 }
}
