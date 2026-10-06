//! 🫳️ Exact history wire demand borrows original checkpoint and alternative fields.
use crate::os_store::{SpaceHistorySnapshot,SpaceCheckpoint,SpaceAlternative,SpaceMemberPin};
use crate::os_vcs::Author;
use crate::os_spr::HybridLogicalTimestamp;
use crate::sqlite_snapshot::SnapshotEncoding;
use semio_framework_pack_json::{JsonWriteSource,JsonWriteNode as J};
use semio_framework_dsl_record::{BorrowedFieldSpec as F,BorrowedRecordSpec as R,BorrowedShape as H,RecordLayout};
use semio_framework_dsl_record::native_encoding::{FieldProjectionSource,FieldProjectionView as V};
use semio_framework_value::{NativeEncodeControl,Number,ValueError,ValueRefusalKind as K};
pub(super) struct Source<'a>(pub(super) &'a SpaceHistorySnapshot);
enum Node<'a>{
 Root(&'a SpaceHistorySnapshot),Checkpoints(&'a [SpaceCheckpoint]),Checkpoint(&'a SpaceCheckpoint),
 Authors(&'a [Author]),Author(&'a Author),Clock(&'a HybridLogicalTimestamp),
 Members(&'a [SpaceMemberPin]),Member(&'a SpaceMemberPin),
 Alternatives(&'a [SpaceAlternative]),Alternative(&'a SpaceAlternative),
 Ids(&'a [String]),Text(&'a str),UInt(u64),
}
fn absent()->ValueError{ValueError::new(K::InvariantViolated,"borrowed history ordinal path is absent")}
impl<'a> Node<'a>{
 fn child(self,index:usize)->Result<Self,ValueError>{
  Ok(match self{
   Self::Root(value)=>match index{0=>Self::Checkpoints(&value.checkpoints),1=>Self::Alternatives(&value.alternatives),2=>Self::Text(value.active_alternative_id.as_deref().ok_or_else(absent)?),_=>return Err(absent())},
   Self::Checkpoints(rows)=>Self::Checkpoint(rows.get(index).ok_or_else(absent)?),
   Self::Checkpoint(value)=>{
    if index==0{Self::Text(&value.id)}else if value.parent_id.is_some()&&index==1{Self::Text(value.parent_id.as_deref().ok_or_else(absent)?)}else{
     match index.checked_sub(usize::from(value.parent_id.is_some())).ok_or_else(absent)?{
      1=>Self::Text(&value.message),2=>Self::Authors(&value.authors),3=>Self::Clock(&value.timestamp),4=>Self::Members(&value.members),_=>return Err(absent()),
     }
    }
   },
   Self::Authors(rows)=>Self::Author(rows.get(index).ok_or_else(absent)?),
   Self::Author(value)=>match index{0=>Self::Text(&value.id),1=>Self::Text(&value.name),2=>Self::Text(value.avatar.as_deref().ok_or_else(absent)?),_=>return Err(absent())},
   Self::Clock(value)=>Self::UInt(match index{0=>value.actor,1=>value.physical_ms,2=>value.logical,_=>return Err(absent())}),
   Self::Members(rows)=>Self::Member(rows.get(index).ok_or_else(absent)?),
   Self::Member(value)=>Self::Text(match index{0=>&value.document_id,1=>&value.checkpoint_id,2=>&value.alternative_id,_=>return Err(absent())}),
   Self::Alternatives(rows)=>Self::Alternative(rows.get(index).ok_or_else(absent)?),
   Self::Alternative(value)=>match index{0=>Self::Text(&value.id),1=>Self::Text(&value.name),2=>Self::Ids(&value.checkpoint_ids),_=>return Err(absent())},
   Self::Ids(rows)=>Self::Text(rows.get(index).ok_or_else(absent)?),
   Self::Text(_)|Self::UInt(_)=>return Err(absent()),
  })
 }
 fn view(self)->J<'a>{match self{
  Self::Root(value)=>J::Object(2+usize::from(value.active_alternative_id.is_some())),
  Self::Checkpoints(rows)=>J::Array(rows.len()),Self::Checkpoint(value)=>J::Object(5+usize::from(value.parent_id.is_some())),
  Self::Authors(rows)=>J::Array(rows.len()),Self::Author(value)=>J::Object(2+usize::from(value.avatar.is_some())),
  Self::Clock(_)=>J::Object(3),Self::Members(rows)=>J::Array(rows.len()),Self::Member(_)=>J::Object(3),
  Self::Alternatives(rows)=>J::Array(rows.len()),Self::Alternative(_)=>J::Object(3),Self::Ids(rows)=>J::Array(rows.len()),
  Self::Text(text)=>J::String(text),Self::UInt(value)=>J::Number(Number::UInt(value)),
 }}
 fn key(self,index:usize)->Result<&'static str,ValueError>{
  let keys:&[&str]=match self{
   Self::Root(value)=>if value.active_alternative_id.is_some(){&["checkpoints","alternatives","activeAlternativeId"]}else{&["checkpoints","alternatives"]},
   Self::Checkpoint(value)=>if value.parent_id.is_some(){&["id","parentId","message","authors","timestamp","members"]}else{&["id","message","authors","timestamp","members"]},
   Self::Author(value)=>if value.avatar.is_some(){&["id","name","avatar"]}else{&["id","name"]},
   Self::Clock(_)=>&["actor","physical_ms","logical"],Self::Member(_)=>&["documentId","checkpointId","alternativeId"],
   Self::Alternative(_)=>&["id","name","checkpointIds"],_=>return Err(absent()),
  };keys.get(index).copied().ok_or_else(absent)
 }
}
impl Source<'_>{
 fn at(&self,path:&[usize])->Result<Node<'_>,ValueError>{let mut node=Node::Root(self.0);for index in path{node=node.child(*index)?;}Ok(node)}
}
impl JsonWriteSource for Source<'_>{
 fn node_at_path(&self,path:&[usize])->Result<J<'_>,ValueError>{Ok(self.at(path)?.view())}
 fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{self.at(path)?.key(index)}
}
impl FieldProjectionSource for Source<'_>{
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{
  if path.is_empty(){return Ok(V::Record(&[1]))}
  if path[0]!=0{return Err(absent())}
  Ok(match self.node_at_path(&path[1..])?{J::Null=>V::IntrinsicNull,J::Bool(value)=>V::IntrinsicBool(value),J::Number(value)=>V::IntrinsicNumber(value),J::String(value)=>V::IntrinsicText(value),J::Array(length)=>V::IntrinsicArray(length),J::Object(length)=>V::IntrinsicObject(length)})
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{
  if path.first()!=Some(&0){return Err(absent())}self.object_key_at_path(&path[1..],index)
 }
}
fn spec()->R{const FIELDS:&[F]=&[F::new(1,"value",H::Value)];R{keyword:None,layout:RecordLayout::Lines,fields:FIELDS}}
pub(super) fn measure(value:&SpaceHistorySnapshot,encoding:SnapshotEncoding,maximum:usize,control:&mut NativeEncodeControl<'_>)->Result<usize,ValueError>{
 let source=Source(value);
 match encoding{
  SnapshotEncoding::Text=>usize::try_from(semio_framework_pack_json::write_json_source_into(&source,maximum as u64,64,u64::MAX,&mut |_:&[u8],_:&mut NativeEncodeControl<'_>|Ok::<_,ValueError>(()),control)?).map_err(|_|ValueError::new(K::WorkLimit,"history wire byte count exceeds address space")),
  SnapshotEncoding::Binary=>{let mut options=pack::record::EncodeOptions::default();options.limits.max_file_len=maximum as u64;pack::record::measure_document_borrowed(&source,&spec(),&options,control)},
 }
}
