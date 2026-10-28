//! 👁️ Canonical transition field views borrow the actual authored graph facts and mutation-ID owner.
use super::*;
/// 🧷️ Read-only source coordinates carry no allocation, execution or publication authority.
#[derive(Clone,Copy,PartialEq,Eq)]
pub struct HistoryTransitionBorrowedIdentity([usize;20]);
fn span(value:Option<&str>)->[usize;2]{value.map_or([0;2],|value|[value.as_ptr()as usize,value.len()])}
/// 📚️ Every field has the exact canonical Commit role without another owned transition record.
pub struct HistoryTransitionCommitView<'a>{pub checkpoint_id:&'a str,pub parent_id:Option<&'a str>,pub change_id:&'a str,pub mutation_ids:&'a[MutationId],pub description:Option<&'a str>,pub saved_at:&'a str,pub authors:&'a[Author],pub message:Option<&'a str>,pub timestamp:&'a str,pub line_id:Option<&'a str>}
impl HistoryTransitionCommitView<'_>{
 /// 🪪️ Captures the actual borrowed field backings rather than the temporary view's stack address.
 pub fn identity(&self)->HistoryTransitionBorrowedIdentity{let fields=[span(Some(self.checkpoint_id)),span(self.parent_id),span(Some(self.change_id)),[self.mutation_ids.as_ptr()as usize,self.mutation_ids.len()],span(self.description),span(Some(self.saved_at)),[self.authors.as_ptr()as usize,self.authors.len()],span(self.message),span(Some(self.timestamp)),span(self.line_id)];let mut identity=[0;20];for(index,field)in fields.into_iter().enumerate(){identity[index*2..index*2+2].copy_from_slice(&field);}HistoryTransitionBorrowedIdentity(identity)}
}
/// 🌿️ A Branch borrows its actual canonical alternative and selected checkpoint identity.
pub struct HistoryTransitionBranchView<'a>{pub alternative_id:&'a str,pub name:&'a str,pub checkpoint_id:&'a str}
impl HistoryTransitionBranchView<'_>{
 /// 🪢️ Exact authored field coordinates remain stable when the borrowing view is rebuilt each turn.
 pub fn identity(&self)->HistoryTransitionBorrowedIdentity{let mut identity=[0;20];for(index,field)in [self.alternative_id,self.name,self.checkpoint_id].into_iter().enumerate(){identity[index*2..index*2+2].copy_from_slice(&span(Some(field)));}HistoryTransitionBorrowedIdentity(identity)}
}
/// ↩️ The canonical event distinguishes removal from reinstatement of the original operation IDs.
#[derive(Clone,Copy,PartialEq,Eq)]
pub enum HistoryTransitionEditMode{Revert,Reinstate}
impl HistoryTransitionEditMode{fn tag(self)->u8{match self{Self::Revert=>0,Self::Reinstate=>1}}}
/// 🧾️ Revert and Reinstate borrow the exact ordered mutation-ID owner rather than copying a transition.
pub struct HistoryTransitionEditsView<'a>{pub mode:HistoryTransitionEditMode,pub mutation_ids:&'a[MutationId]}
impl HistoryTransitionEditsView<'_>{
 /// 🪪️ The source fence includes the actual ordered backing and the selected canonical event kind.
 pub fn identity(&self)->HistoryTransitionBorrowedIdentity{let mut identity=[0;20];identity[..3].copy_from_slice(&[self.mutation_ids.as_ptr()as usize,self.mutation_ids.len(),self.mode.tag()as usize]);HistoryTransitionBorrowedIdentity(identity)}
}
/// 🧷️ The enclosing native publisher authenticates this borrowed original pin roster and epoch.
pub trait HistoryTransitionPinSource{fn source_identity(&self)->(usize,usize);fn len(&self)->usize;fn is_empty(&self)->bool{self.len()==0}fn uri_part(&self,row:usize,part:usize)->Option<&str>;fn checkpoint_id(&self,row:usize)->Option<&str>;}
/// 📌️ Repin borrows canonical checkpoint fields and seven-part original URIs without concatenating text.
pub struct HistoryTransitionRepinView<'a>{pub checkpoint_id:&'a str,pub pinned_checkpoint_id:&'a str,pub pins:&'a dyn HistoryTransitionPinSource}
impl HistoryTransitionRepinView<'_>{
 /// 🪪️ Actual checkpoint backings and the original pin roster survive rebuilt stack views each turn.
 pub fn identity(&self)->HistoryTransitionBorrowedIdentity{let mut identity=[0;20];identity[..2].copy_from_slice(&span(Some(self.checkpoint_id)));identity[2..4].copy_from_slice(&span(Some(self.pinned_checkpoint_id)));let(pins,length)=self.pins.source_identity();identity[4]=pins;identity[5]=length;identity[6]=self.pins.len();HistoryTransitionBorrowedIdentity(identity)}
}
impl HistoryTransitionWireCursor{
 fn repin_view_part<'a>(&self,source:&HistoryTransitionRepinView<'a>)->Result<Option<Part<'a>>,ValueError>{let invalid=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original Repin pin field changed");Ok(match self.stage{0=>Some(Part::Number(5)),1=>Some(Part::Text(source.checkpoint_id)),2=>Some(Part::Text(source.pinned_checkpoint_id)),3=>Some(Part::Number(source.pins.len()as u64)),4 if self.row<source.pins.len()=>Some(match self.field{0=>{let mut length=0usize;for part in 0..7{length=length.checked_add(source.pins.uri_part(self.row,part).ok_or_else(invalid)?.len()).ok_or_else(invalid)?;}Part::Number(length as u64)},1..=7=>Part::Raw(source.pins.uri_part(self.row,self.field as usize-1).ok_or_else(invalid)?.as_bytes()),_=>Part::Text(source.pins.checkpoint_id(self.row).ok_or_else(invalid)?)}),_=>None})}
 fn advance_repin_view(&mut self,source:&HistoryTransitionRepinView<'_>){self.offset=0;if self.stage==4{self.field+=1;if self.field<9{return}self.field=0;self.row+=1;if self.row<source.pins.len(){return}}self.stage+=1;self.row=0;}
 fn skip_repin_view(&mut self,source:&HistoryTransitionRepinView<'_>){if self.stage==4{if source.pins.is_empty(){self.stage=5;return}while (1..=7).contains(&self.field)&&source.pins.uri_part(self.row,self.field as usize-1).is_some_and(str::is_empty){self.field+=1;}}}
 /// 📌️ Original URI segments stream as one canonical length-prefixed field through the portable Repin codec.
 pub fn write_repin_view(&mut self,source:&HistoryTransitionRepinView<'_>,epoch:u64,output:&mut[u8])->Result<usize,ValueError>{let identity=WireIdentity{address:source.checkpoint_id.as_ptr()as usize,kind:6,epoch,scope:None,borrowed:Some(source.identity())};self.write_parts(identity,output,|cursor|cursor.repin_view_part(source),|cursor|cursor.advance_repin_view(source),|cursor|cursor.skip_repin_view(source))}
 /// ↩️ Streams the actual original ordered IDs through the defining Revert or Reinstate codec.
 pub fn write_edits_view(&mut self,source:&HistoryTransitionEditsView<'_>,epoch:u64,output:&mut[u8])->Result<usize,ValueError>{let identity=WireIdentity{address:source.mutation_ids.as_ptr()as usize,kind:4+source.mode.tag(),epoch,scope:None,borrowed:Some(source.identity())};self.write_parts(identity,output,|cursor|Ok(match cursor.stage{0=>Some(Part::Number(source.mode.tag()as u64)),1=>Some(Part::Number(source.mutation_ids.len()as u64)),2=>source.mutation_ids.get(cursor.row).map(|id|Part::Text(&id.0)),_=>None}),|cursor|{cursor.offset=0;if cursor.stage==2{cursor.row+=1;if cursor.row<source.mutation_ids.len(){return}}cursor.stage+=1;cursor.row=0;},|_|{})}
 fn commit_view_part<'a>(&self,source:&HistoryTransitionCommitView<'a>)->Option<Part<'a>>{use Part::*;match self.stage{0=>Some(Number(2)),1=>Some(Text(source.checkpoint_id)),2=>Some(Optional(source.parent_id)),3=>Some(Text(source.change_id)),4=>Some(Number(source.mutation_ids.len()as u64)),5=>source.mutation_ids.get(self.row).map(|id|Text(&id.0)),6=>Some(Optional(source.description)),7=>Some(Text(source.saved_at)),8=>Some(Number(source.authors.len()as u64)),9=>source.authors.get(self.row).map(|author|match self.field{0=>Text(&author.id),1=>Text(&author.name),_=>Optional(author.avatar.as_deref())}),10=>Some(Optional(source.message)),11=>Some(Text(source.timestamp)),12=>Some(Optional(source.line_id)),_=>None}}
 fn advance_commit_view(&mut self,source:&HistoryTransitionCommitView<'_>){self.offset=0;if self.stage==5{self.row+=1;if self.row<source.mutation_ids.len(){return}self.row=0;}if self.stage==9{self.field+=1;if self.field<3{return}self.field=0;self.row+=1;if self.row<source.authors.len(){return}}self.stage+=1;self.row=0;}
 fn skip_commit_view(&mut self,source:&HistoryTransitionCommitView<'_>){if self.stage==5&&source.mutation_ids.is_empty(){self.stage=6;}if self.stage==9&&source.authors.is_empty(){self.stage=10;}}
 /// 🪡️ Streams the original canonical Commit fields through the same portable payload codec.
 pub fn write_commit_view(&mut self,source:&HistoryTransitionCommitView<'_>,epoch:u64,output:&mut[u8])->Result<usize,ValueError>{let identity=WireIdentity{address:source.checkpoint_id.as_ptr()as usize,kind:2,epoch,scope:None,borrowed:Some(source.identity())};self.write_parts(identity,output,|cursor|Ok(cursor.commit_view_part(source)),|cursor|cursor.advance_commit_view(source),|cursor|cursor.skip_commit_view(source))}
 /// 🌱️ Streams actual canonical Branch facts without copying name or identity fields into a temporary record.
 pub fn write_branch_view(&mut self,source:&HistoryTransitionBranchView<'_>,epoch:u64,output:&mut[u8])->Result<usize,ValueError>{let identity=WireIdentity{address:source.alternative_id.as_ptr()as usize,kind:3,epoch,scope:None,borrowed:Some(source.identity())};self.write_parts(identity,output,|cursor|Ok(match cursor.stage{0=>Some(Part::Number(3)),1=>Some(Part::Text(source.alternative_id)),2=>Some(Part::Text(source.name)),3=>Some(Part::Text(source.checkpoint_id)),_=>None}),|cursor|{cursor.offset=0;cursor.stage+=1;cursor.row=0;},|_|{})}
}
