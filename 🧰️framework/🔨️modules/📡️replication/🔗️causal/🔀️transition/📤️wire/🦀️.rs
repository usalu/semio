//! 📤️ Borrows one original transition and emits its existing canonical payload without an intermediate allocation.
use super::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
#[derive(Clone,Copy)]
enum Part<'a>{Number(u64),Text(&'a str),Optional(Option<&'a str>),Bytes(&'a[u8]),Raw(&'a[u8]),Byte(u8)}
fn varint_length(value:u64)->usize{((64-value.leading_zeros()as usize).max(1)+6)/7}
fn varint_byte(value:u64,index:usize)->u8{let next=value>>(index*7);(next as u8&127)|if next>=128{128}else{0}}
impl Part<'_>{
 fn len(self)->usize{match self{Self::Number(value)=>varint_length(value),Self::Text(text)=>varint_length(text.len()as u64)+text.len(),Self::Bytes(bytes)=>varint_length(bytes.len()as u64)+bytes.len(),Self::Optional(None)=>1,Self::Optional(Some(text))=>1+varint_length(text.len()as u64)+text.len(),Self::Raw(bytes)=>bytes.len(),Self::Byte(_)=>1}}
 fn byte(self,index:usize)->u8{match self{Self::Raw(bytes)=>bytes[index],Self::Number(value)=>varint_byte(value,index),Self::Byte(value)=>value,Self::Optional(None)=>0,Self::Optional(Some(_))if index==0=>1,Self::Optional(Some(text))=>Self::Text(text).byte(index-1),Self::Text(text)=>Self::Bytes(text.as_bytes()).byte(index),Self::Bytes(bytes)=>{let header=varint_length(bytes.len()as u64);if index<header{varint_byte(bytes.len()as u64,index)}else{bytes[index-header]}}}}
}
#[derive(Clone,Copy,PartialEq,Eq)]
struct WireIdentity{address:usize,kind:u8,epoch:u64,scope:Option<(usize,usize)>,borrowed:Option<HistoryTransitionBorrowedIdentity>}
/// 🪡️ Fixed cursor coordinates retain no borrowed pointer after the caller's write returns.
#[derive(Default)]
pub struct HistoryTransitionWireCursor{source:Option<WireIdentity>,stage:u8,row:usize,field:u8,offset:usize,written:usize,finished:bool}
impl HistoryTransitionWireCursor{
 /// 📈️ Counts exactly the canonical payload bytes already delivered to the original recipient.
 pub fn written(&self)->usize{self.written}
 /// 🏁️ Reports exhaustion only after the final original field has been delivered.
 pub fn is_finished(&self)->bool{self.finished}
 fn part<'a>(&self,source:&'a HistoryTransition)->Option<Part<'a>>{use Part::*;if self.stage==0{return Some(Number(match source{HistoryTransition::Revert{..}=>0,HistoryTransition::Reinstate{..}=>1,HistoryTransition::Commit(_)=>2,HistoryTransition::Branch{..}=>3,HistoryTransition::Checkout{..}=>4,HistoryTransition::Repin{..}=>5,HistoryTransition::Supersede(_)=>6}))}match source{
  HistoryTransition::Revert{mutation_ids}|HistoryTransition::Reinstate{mutation_ids}=>match self.stage{1=>Some(Number(mutation_ids.len()as u64)),2=>mutation_ids.get(self.row).map(|id|Text(&id.0)),_=>None},
  HistoryTransition::Branch{alternative_id,name,checkpoint_id}=>match self.stage{1=>Some(Text(alternative_id)),2=>Some(Text(name)),3=>Some(Text(checkpoint_id)),_=>None},
  HistoryTransition::Checkout{checkpoint_id,alternative_id}=>match self.stage{1=>Some(Text(checkpoint_id)),2=>Some(Optional(alternative_id.as_deref())),_=>None},
  HistoryTransition::Repin{checkpoint_id,pinned_checkpoint_id,pins}=>match self.stage{1=>Some(Text(checkpoint_id)),2=>Some(Text(pinned_checkpoint_id)),3=>Some(Number(pins.len()as u64)),4=>pins.get(self.row).map(|pin|Text(if self.field==0{&pin.child_uri}else{&pin.checkpoint_id})),_=>None},
  HistoryTransition::Supersede(s)=>match self.stage{1=>Some(Optional(s.scope.as_deref())),2=>Some(Number(s.inputs.len()as u64)),3=>s.inputs.get(self.row).and_then(|input|match self.field{0=>Some(Text(&input.target.0)),1=>Some(Byte(u8::from(matches!(input.replacement,InputReplacement::Withdrawn)))),2=>match &input.replacement{InputReplacement::Input{schema,..}=>Some(Text(schema)),_=>None},3=>match &input.replacement{InputReplacement::Input{payload,..}=>Some(Bytes(payload)),_=>None},_=>None}),_=>None},
  HistoryTransition::Commit(c)=>match self.stage{1=>Some(Text(&c.checkpoint_id)),2=>Some(Optional(c.parent_id.as_deref())),3=>Some(Text(&c.change_id)),4=>Some(Number(c.mutation_ids.len()as u64)),5=>c.mutation_ids.get(self.row).map(|id|Text(&id.0)),6=>Some(Optional(c.description.as_deref())),7=>Some(Text(&c.saved_at)),8=>Some(Number(c.authors.len()as u64)),9=>c.authors.get(self.row).map(|author|match self.field{0=>Text(&author.id),1=>Text(&author.name),_=>Optional(author.avatar.as_deref())}),10=>Some(Optional(c.message.as_deref())),11=>Some(Text(&c.timestamp)),12=>Some(Optional(c.line_id.as_deref())),_=>None},
 }}
 fn advance(&mut self,source:&HistoryTransition){self.offset=0;match source{
  HistoryTransition::Revert{mutation_ids}|HistoryTransition::Reinstate{mutation_ids}if self.stage==2=>{self.row+=1;if self.row<mutation_ids.len(){return}},
  HistoryTransition::Repin{pins,..}if self.stage==4=>{self.field+=1;if self.field<2{return}self.field=0;self.row+=1;if self.row<pins.len(){return}},
  HistoryTransition::Supersede(s)if self.stage==3=>{self.field+=1;let fields=if matches!(s.inputs[self.row].replacement,InputReplacement::Withdrawn){2}else{4};if self.field<fields{return}self.field=0;self.row+=1;if self.row<s.inputs.len(){return}},
  HistoryTransition::Commit(c)if self.stage==5=>{self.row+=1;if self.row<c.mutation_ids.len(){return}self.row=0;},
  HistoryTransition::Commit(c)if self.stage==9=>{self.field+=1;if self.field<3{return}self.field=0;self.row+=1;if self.row<c.authors.len(){return}},_=>{}
 }self.stage+=1;self.row=0;}
 fn skip_empty(&mut self,source:&HistoryTransition){match source{HistoryTransition::Commit(c)if self.stage==5&&c.mutation_ids.is_empty()=>self.stage=6,HistoryTransition::Commit(c)if self.stage==9&&c.authors.is_empty()=>self.stage=10,_=>{}}}
 fn write_parts<'a>(&mut self,identity:WireIdentity,output:&mut[u8],part:impl Fn(&Self)->Result<Option<Part<'a>>,ValueError>,advance:impl Fn(&mut Self),skip:impl Fn(&mut Self))->Result<usize,ValueError>{
  if self.source.is_some_and(|original|original!=identity){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"transition wire cursor requires its original source"))}if output.is_empty()||self.finished{return Ok(0)}self.source=Some(identity);let mut copied=0;while copied<output.len(){skip(self);let Some(part)=part(self)?else{self.finished=true;break};output[copied]=part.byte(self.offset);self.offset+=1;copied+=1;self.written=self.written.checked_add(1).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"transition wire extent overflow"))?;if self.offset==part.len(){advance(self);}}if part(self)?.is_none(){skip(self);self.finished=part(self)?.is_none();}Ok(copied)
 }
 /// 📦️ Writes at most the supplied output slice; a different original owner is refused before any byte changes.
 pub fn write(&mut self,source:&HistoryTransition,output:&mut[u8])->Result<usize,ValueError>{self.write_parts(WireIdentity{address:source as*const _ as usize,kind:0,epoch:0,scope:None,borrowed:None},output,|cursor|Ok(cursor.part(source)),|cursor|cursor.advance(source),|cursor|cursor.skip_empty(source))}
 fn draft_part<'a>(&self,drafts:&'a HistoryInputDrafts,scope:Option<&'a str>)->Result<Option<Part<'a>>,ValueError>{
  use Part::*;Ok(match self.stage{0=>Some(Number(6)),1=>Some(Optional(scope)),2=>Some(Number(drafts.len()as u64)),3=>match drafts.at_rank(self.row){None=>None,Some((target,replacement))=>match self.field{0=>Some(Text(&target.0)),1=>Some(Byte(u8::from(matches!(replacement,InputReplacement::Withdrawn)))),2=>match replacement{InputReplacement::Input{schema,payload}=>{if payload.len()>SUPERSEDE_PAYLOAD_MAX_BYTES{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"supersede payload exceeds its canonical codec limit"))}Some(Text(schema))},_=>None},3=>match replacement{InputReplacement::Input{payload,..}=>Some(Bytes(payload)),_=>None},_=>None}},_=>None})
 }
 fn advance_draft(&mut self,drafts:&HistoryInputDrafts){self.offset=0;if self.stage==3{self.field+=1;let fields=if matches!(drafts.at_rank(self.row).unwrap().1,InputReplacement::Withdrawn){2}else{4};if self.field<fields{return}self.field=0;self.row+=1;if self.row<drafts.len(){return}}self.stage+=1;self.row=0;}
 /// 📝️ Streams the original sorted drafts without constructing a replacement transition or copying its targets and payloads.
 pub fn write_supersession(&mut self,drafts:&HistoryInputDrafts,scope:Option<&str>,output:&mut[u8])->Result<usize,ValueError>{
  if drafts.is_empty()||scope.is_some_and(|scope|scope.len()>SUPERSEDE_SCOPE_MAX_BYTES){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"supersede requires original inputs and a bounded scope"))}let identity=WireIdentity{address:drafts as*const _ as usize,kind:1,epoch:drafts.mutation_epoch(),scope:scope.map(|scope|(scope.as_ptr()as usize,scope.len())),borrowed:None};self.write_parts(identity,output,|cursor|cursor.draft_part(drafts,scope),|cursor|cursor.advance_draft(drafts),|_|{})
 }

}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

#[path="👁️.rs"]
mod borrowed;
pub use borrowed::{HistoryTransitionBorrowedIdentity,HistoryTransitionCommitView,HistoryTransitionBranchView,HistoryTransitionEditMode,HistoryTransitionEditsView,HistoryTransitionPinSource,HistoryTransitionRepinView};
