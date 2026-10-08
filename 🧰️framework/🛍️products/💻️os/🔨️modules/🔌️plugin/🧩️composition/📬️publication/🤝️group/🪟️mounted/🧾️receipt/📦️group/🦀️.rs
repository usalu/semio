use super::*;
use semio_framework_value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
use std::mem::ManuallyDrop;

pub(crate) trait MountedGroupPreparedChildEdits {
    fn len(&self)->usize;
    fn get(&self,index:usize)->Option<&str>;
}

#[derive(Clone,Copy)]
pub(crate) enum MountedGroupChildEditSource<'a> {
    OriginalStrings(&'a [String]),
    Ready(&'a dyn MountedGroupPreparedChildEdits),
}
impl MountedGroupChildEditSource<'_> {
    fn len(&self)->usize{match self{Self::OriginalStrings(ids)=>ids.len(),Self::Ready(rows)=>rows.len()}}
    fn get(&self,index:usize)->Option<&str>{match self{Self::OriginalStrings(ids)=>ids.get(index).map(String::as_str),Self::Ready(rows)=>rows.get(index)}}
    fn identity(&self)->[(usize,usize);2]{match self{Self::OriginalStrings(ids)=>[(ids.as_ptr()as usize,ids.len()),(0,0)],Self::Ready(rows)=>[(*rows as *const dyn MountedGroupPreparedChildEdits as *const () as usize,rows.len()),(1,0)]}}
}

pub(crate) struct MountedGroupReceiptSource<'a> {
    pub(crate) invocation_id: &'a str,
    pub(crate) parent_edit_id: Option<&'a str>,
    pub(crate) child_edit_ids: MountedGroupChildEditSource<'a>,
}
impl MountedGroupReceiptSource<'_> {
    fn identity(&self)->[(usize,usize);4] { let children=self.child_edit_ids.identity();[(self.invocation_id.as_ptr()as usize,self.invocation_id.len()),self.parent_edit_id.map_or((0,0),|id|(id.as_ptr()as usize,id.len())),children[0],children[1]] }
}

pub(crate) struct MountedGroupReceiptOutput {
    pub(crate) mutations: Vec<KernelMutation>,
    pub(crate) inverse_group: UndoGroup,
    pub(crate) command_edit_id: Option<String>,
    pub(crate) command_child_edit_ids: Vec<String>,
}

pub(crate) struct MountedGroupReceipt {
    mutations: ManuallyDrop<Vec<KernelMutation>>,
    undo_ids: ManuallyDrop<Vec<MutationId>>,
    inverses: ManuallyDrop<Vec<InverseMutation>>,
    member_edits: ManuallyDrop<Vec<EditRef>>,
    command_children: ManuallyDrop<Vec<String>>,
    invocation: MountedReceiptBytes,
    parent: MountedReceiptBytes,
    child: MountedReceiptBytes,
    edit: MountedReceiptBytes,
    identity: [(usize,usize);4],
    active_identity: Option<(usize,usize)>,
    edit_identity: Option<(usize,usize)>,
    triple_identity: Option<[(usize,usize);3]>,
    triple_offset: usize,
    operation_count: usize,
    member_count: usize,
    member_document: Option<ArtifactHandle>,
    child_count: usize,
    has_parent: bool,
    phase: usize,
    offset: usize,
    close_phase: usize,
    close_field: usize,
    closing: bool,
    transferred: bool,
}

impl MountedGroupReceipt {
    pub(crate) fn new(operation_count:usize,source:&MountedGroupReceiptSource<'_>)->Result<Self,ValueError> {
        let count=source.child_edit_ids.len()+usize::from(source.parent_edit_id.is_some());
        let bytes=operation_count.checked_mul(size_of::<KernelMutation>()+size_of::<MutationId>()+size_of::<InverseMutation>()).and_then(|n|count.checked_mul(size_of::<EditRef>()).and_then(|members|n.checked_add(members))).and_then(|n|source.child_edit_ids.len().checked_mul(size_of::<String>()).and_then(|children|n.checked_add(children)));
        if bytes.is_none_or(|bytes|bytes>MOUNTED_RECEIPT_MAXIMUM_BYTES) { return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"mounted group receipt requires its exact admitted operation and member sets")); }
        Ok(Self{mutations:ManuallyDrop::new(Vec::new()),undo_ids:ManuallyDrop::new(Vec::new()),inverses:ManuallyDrop::new(Vec::new()),member_edits:ManuallyDrop::new(Vec::new()),command_children:ManuallyDrop::new(Vec::new()),invocation:MountedReceiptBytes::new(),parent:MountedReceiptBytes::new(),child:MountedReceiptBytes::new(),edit:MountedReceiptBytes::new(),identity:source.identity(),active_identity:None,edit_identity:None,triple_identity:None,triple_offset:0,operation_count,member_count:count,member_document:None,child_count:source.child_edit_ids.len(),has_parent:source.parent_edit_id.is_some(),phase:0,offset:0,close_phase:0,close_field:0,closing:false,transferred:false})
    }
    fn validate(&self,source:&MountedGroupReceiptSource<'_>)->Result<(),ValueError> { if self.closing||self.transferred||self.identity!=source.identity(){Err(ValueError::new(ValueRefusalKind::InvariantViolated,"mounted group receipt lost its original borrowed metadata"))}else{Ok(())} }
    fn text<'a>(&self,source:&'a MountedGroupReceiptSource<'a>)->Result<Option<&'a str>,ValueError> { Ok(match self.phase {5=>Some(source.invocation_id),6=>source.parent_edit_id,7=>{let index=self.command_children.len();if index==self.child_count{None}else{Some(source.child_edit_ids.get(index).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"mounted child receipt requires its original completed member row"))?)}},_=>None}) }
    fn bytes(&self)->&MountedReceiptBytes {match self.phase {5=>&self.invocation,6=>&self.parent,_=>&self.child}}
    fn bytes_mut(&mut self)->&mut MountedReceiptBytes {match self.phase {5=>&mut self.invocation,6=>&mut self.parent,_=>&mut self.child}}
    /// 🌱️ Queries one complete typed backing or one bounded metadata candidate before ownership moves.
    pub(crate) fn next_capacity_byte_demand(&self,source:&MountedGroupReceiptSource<'_>)->Result<usize,ValueError> {
        self.validate(source)?;
        Ok(match self.phase {0=>self.operation_count*size_of::<KernelMutation>(),1=>self.operation_count*size_of::<MutationId>(),2=>self.operation_count*size_of::<InverseMutation>(),3=>self.member_count*size_of::<EditRef>(),4=>self.child_count*size_of::<String>(),5..=7=>match self.text(source)?{Some(text)=>self.bytes().next_capacity_byte_demand((text.len()-self.offset).min(64))?,None=>0},_=>0})
    }
    pub(crate) fn next_metadata_release_byte_demand(&self)->usize {if (5..=7).contains(&self.phase){self.bytes().next_growth_release_byte_demand()}else{0}}
    /// 🪪️ Copies only the current original UTF8 prefix into separately admitted receipt storage.
    pub(crate) fn advance_metadata(&mut self,source:&MountedGroupReceiptSource<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        self.validate(source)?;
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.phase<5{
            let bytes=self.next_capacity_byte_demand(source)?;
            if grant.maximum_capacity_bytes<bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
            match self.phase {0=>*self.mutations=Vec::with_capacity(self.operation_count),1=>*self.undo_ids=Vec::with_capacity(self.operation_count),2=>*self.inverses=Vec::with_capacity(self.operation_count),3=>*self.member_edits=Vec::with_capacity(self.member_count),4=>*self.command_children=Vec::with_capacity(self.child_count),_=>unreachable!()}
            self.phase+=1;return Ok(Self::progress(bytes,0));
        }
        if self.phase==8{return Ok(RetainedCloneStep::Complete(Default::default()));}
        let Some(text)=self.text(source)?else{self.phase+=1;self.offset=0;return Ok(Self::progress(0,0));};
        let identity=(text.as_ptr()as usize,text.len());
        if self.active_identity.is_some_and(|original|original!=identity){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"mounted group receipt changed its current original ID"));}
        self.active_identity=Some(identity);
        let offset=self.offset;let additional=(text.len()-offset).min(64);
        if additional>0{
            if !self.bytes().ready_for(additional){return self.bytes_mut().reserve_step(additional,grant);}
            let step=self.bytes_mut().append_step(&text.as_bytes()[offset..offset+additional],grant)?;self.offset+=step.progress().copied_bytes;return Ok(step);
        }
        if self.phase==7{let bytes=self.child.take(grant).unwrap();self.command_children.push(unsafe{String::from_utf8_unchecked(bytes)});self.child=MountedReceiptBytes::new();if self.command_children.len()==self.child_count{self.phase=8;}}
        else{self.phase+=1;}
        self.offset=0;self.active_identity=None;Ok(Self::progress(0,0))
    }
    fn validate_triple(&self,lane:&(KernelMutation,MutationId,InverseMutation))->Result<[(usize,usize);3],ValueError>{
        let ids=[lane.0.id.0.as_str(),lane.1.0.as_str(),lane.2.target_mutation.0.as_str()];let identity=ids.map(|id|(id.as_ptr()as usize,id.len()));
        if self.phase!=8||self.closing||self.transferred||self.mutations.len()>=self.operation_count||identity[0].1!=identity[1].1||identity[0].1!=identity[2].1||self.triple_identity.is_some_and(|original|original!=identity){Err(ValueError::new(ValueRefusalKind::InvariantViolated,"mounted group receipt requires its exact original ready operation"))}else{Ok(identity)}
    }
    /// 🔬️ Validates three original ID prefixes before a distinct one-item ownership transfer.
    pub(crate) fn push_mutation_triple(&mut self,lane:&mut Option<(KernelMutation,MutationId,InverseMutation)>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        let original=lane.as_ref().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"mounted receipt ready operation is absent"))?;let identity=self.validate_triple(original)?;
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.triple_offset<identity[0].1{
            let count=(grant.maximum_copy_bytes / 3).min(21).min(identity[0].1-self.triple_offset);if count==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
            let range=self.triple_offset..self.triple_offset+count;let expected=&original.0.id.0.as_bytes()[range.clone()];
            if expected!=&original.1.0.as_bytes()[range.clone()]||expected!=&original.2.target_mutation.0.as_bytes()[range]{return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"mounted group receipt rejected a foreign original operation ID prefix"));}
            self.triple_identity=Some(identity);self.triple_offset+=count;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:count*3,..Default::default()}));
        }
        let (mutation,id,inverse)=lane.take().unwrap();self.mutations.push(mutation);self.undo_ids.push(id);self.inverses.push(inverse);self.triple_identity=None;self.triple_offset=0;Ok(Self::progress(0,0))
    }
    fn validate_member(&self,document:ArtifactHandle,edit_id:&str)->Result<(),ValueError>{if self.phase!=8||self.closing||self.transferred||self.member_edits.len()>=self.member_count||self.edit_identity.is_some_and(|original|original!=(edit_id.as_ptr()as usize,edit_id.len()))||self.member_document.is_some_and(|original|original!=document){Err(ValueError::new(ValueRefusalKind::InvariantViolated,"mounted group receipt requires its exact prepared member edit"))}else{Ok(())}}
    pub(crate) fn next_member_capacity_byte_demand(&self,document:ArtifactHandle,edit_id:&str)->Result<usize,ValueError>{self.validate_member(document,edit_id)?;self.edit.next_capacity_byte_demand((edit_id.len()-self.offset).min(64))}
    pub(crate) fn next_member_release_byte_demand(&self)->usize{self.edit.next_growth_release_byte_demand()}
    /// 🪪️ Copies one exact prepared member edit ID independently from all operation IDs.
    pub(crate) fn push_member_edit(&mut self,document:ArtifactHandle,edit_id:&str,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.validate_member(document,edit_id)?;if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        self.edit_identity=Some((edit_id.as_ptr()as usize,edit_id.len()));self.member_document=Some(document);
        let additional=(edit_id.len()-self.offset).min(64);
        if additional>0{if !self.edit.ready_for(additional){return self.edit.reserve_step(additional,grant);}let step=self.edit.append_step(&edit_id.as_bytes()[self.offset..self.offset+additional],grant)?;self.offset+=step.progress().copied_bytes;return Ok(step);}
        let bytes=self.edit.take(grant).unwrap();self.member_edits.push(EditRef{document,edit_id:unsafe{String::from_utf8_unchecked(bytes)}});self.edit=MountedReceiptBytes::new();self.offset=0;self.edit_identity=None;self.member_document=None;Ok(Self::progress(0,0))
    }
    pub(crate) fn is_complete(&self)->bool {self.phase==8&&!self.closing&&!self.transferred&&self.mutations.len()==self.operation_count&&self.member_edits.len()==self.member_count&&self.edit_identity.is_none()}
    /// 🧳️ Transfers all final original vectors and UTF8 backing without postdecision allocation.
    pub(crate) fn take(&mut self,grant:RetainedCloneGrant)->Option<MountedGroupReceiptOutput>{
        if !self.is_complete()||grant.maximum_items==0{return None;}
        let invocation=self.invocation.take(grant)?;let parent=if self.has_parent{Some(unsafe{String::from_utf8_unchecked(self.parent.take(grant)? )})}else{self.parent.close_step(grant);None};
        self.child.close_step(grant);self.edit.close_step(grant);self.transferred=true;
        Some(MountedGroupReceiptOutput{mutations:std::mem::take(&mut *self.mutations),inverse_group:UndoGroup{invocation_id:InvocationId(unsafe{String::from_utf8_unchecked(invocation)}),mutations:std::mem::take(&mut *self.undo_ids),inverse_mutations:std::mem::take(&mut *self.inverses),member_edits:std::mem::take(&mut *self.member_edits)},command_edit_id:parent,command_child_edit_ids:std::mem::take(&mut *self.command_children)})
    }
    /// ↩️ Retains original completed output for funded cancellation before common publication.
    pub(crate) fn retain_output(&mut self,output:&mut Option<MountedGroupReceiptOutput>,grant:RetainedCloneGrant)->RetainedCloneStep{
        if grant.maximum_items==0||!self.transferred||self.closing||output.is_none()||[self.mutations.capacity(),self.undo_ids.capacity(),self.inverses.capacity(),self.member_edits.capacity(),self.command_children.capacity()].iter().any(|capacity|*capacity!=0)||!self.invocation.terminal_is_empty()||!self.parent.terminal_is_empty()||!self.child.terminal_is_empty()||!self.edit.terminal_is_empty(){return RetainedCloneStep::Progress(Default::default());}
        let MountedGroupReceiptOutput{mutations,inverse_group,command_edit_id,command_child_edit_ids}=output.take().unwrap();let UndoGroup{invocation_id,mutations:undo_ids,inverse_mutations,member_edits}=inverse_group;
        *self.mutations=mutations;*self.undo_ids=undo_ids;*self.inverses=inverse_mutations;*self.member_edits=member_edits;*self.command_children=command_child_edit_ids;self.invocation=MountedReceiptBytes::from_original(invocation_id.0.into_bytes());self.parent=MountedReceiptBytes::from_original(command_edit_id.unwrap_or_default().into_bytes());self.transferred=false;self.closing=true;self.close_phase=0;self.close_field=0;self.triple_identity=None;self.triple_offset=0;Self::progress(0,0)
    }
    fn progress(birth:usize,release:usize)->RetainedCloneStep{RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:birth,released_bytes:release,..Default::default()})}
    fn text_demand(text:&String)->usize{text.capacity()}
    fn clear_text(text:&mut String){drop(std::mem::take(text));}
    fn dependencies_demand(ids:&Vec<MutationId>)->usize{ids.last().map_or(ids.capacity()*size_of::<MutationId>(),|id|id.0.capacity())}
    fn clear_dependencies(ids:&mut Vec<MutationId>)->bool{if let Some(id)=ids.last_mut(){Self::clear_text(&mut id.0);ids.pop();false}else{drop(std::mem::take(ids));true}}
    fn inverse_demand(inverse:&InverseMutation,field:usize)->usize{match field{0=>inverse.target_mutation.0.capacity(),1=>inverse.inverse_diff.schema.0.capacity(),2=>inverse.inverse_diff.payload.capacity(),3=>Self::dependencies_demand(&inverse.dependencies),_=>0}}
    fn clear_inverse(inverse:&mut InverseMutation,field:usize)->bool{match field{0=>Self::clear_text(&mut inverse.target_mutation.0),1=>Self::clear_text(&mut inverse.inverse_diff.schema.0),2=>drop(std::mem::take(&mut inverse.inverse_diff.payload)),3=>return Self::clear_dependencies(&mut inverse.dependencies),_=>{}}true}
    fn mutation_demand(mutation:&KernelMutation,field:usize)->usize{match field{0=>mutation.id.0.capacity(),1=>mutation.invocation_id.0.capacity(),2=>mutation.diff.schema.0.capacity(),3=>mutation.diff.payload.capacity(),4..=7=>Self::inverse_demand(&mutation.inverse,field-4),8=>Self::dependencies_demand(&mutation.dependencies),9=>mutation.author.0.capacity(),_=>0}}
    fn clear_mutation(mutation:&mut KernelMutation,field:usize)->bool{match field{0=>Self::clear_text(&mut mutation.id.0),1=>Self::clear_text(&mut mutation.invocation_id.0),2=>Self::clear_text(&mut mutation.diff.schema.0),3=>drop(std::mem::take(&mut mutation.diff.payload)),4..=7=>return Self::clear_inverse(&mut mutation.inverse,field-4),8=>return Self::clear_dependencies(&mut mutation.dependencies),9=>Self::clear_text(&mut mutation.author.0),_=>{}}true}
    /// 🍂️ Borrows the next exact original field or empty-vector backing retirement demand.
    pub(crate) fn next_close_byte_demand(&self)->usize{if self.transferred{return 0;}match self.close_phase{0=>self.mutations.last().map_or(self.mutations.capacity()*size_of::<KernelMutation>(),|mutation|Self::mutation_demand(mutation,self.close_field)),1=>self.undo_ids.last().map_or(self.undo_ids.capacity()*size_of::<MutationId>(),|id|id.0.capacity()),2=>self.inverses.last().map_or(self.inverses.capacity()*size_of::<InverseMutation>(),|inverse|Self::inverse_demand(inverse,self.close_field)),3=>self.member_edits.last().map_or(self.member_edits.capacity()*size_of::<EditRef>(),|edit|edit.edit_id.capacity()),4=>self.command_children.last().map_or(self.command_children.capacity()*size_of::<String>(),Self::text_demand),5=>self.invocation.next_close_byte_demand(),6=>self.parent.next_close_byte_demand(),7=>self.child.next_close_byte_demand(),8=>self.edit.next_close_byte_demand(),_=>0}}
    /// 📏️ Quotes the fixed causal field path or the selected original byte cursor.
    pub(crate) fn retirement_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.terminal_is_empty() { return Ok(Default::default()); }
        if (5..=8).contains(&self.close_phase) {
            let owner = match self.close_phase { 5 => &self.invocation, 6 => &self.parent, 7 => &self.child, _ => &self.edit };
            return mounted_nested_retirement_demand(owner.retirement_demands()?, 1);
        }
        let depth = match self.close_phase {
            0 if !self.mutations.is_empty() => if matches!(self.close_field, 7 | 8) { 4 } else { 3 },
            2 if !self.inverses.is_empty() => if self.close_field == 3 { 4 } else { 3 },
            1 | 3 | 4 => 2,
            _ => 1,
        };
        Ok(semio_framework_value::RetirementDemand { release_bytes: self.next_close_byte_demand(), depth, ..Default::default() })
    }
    /// ♻️ Closes one retained causal field per turn and funds every indivisible allocation before taking it.
    pub(crate) fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedCloneStep{
        if self.terminal_is_empty(){return RetainedCloneStep::Complete(Default::default());}
        let bytes=self.next_close_byte_demand();if grant.maximum_items==0||grant.maximum_release_bytes<bytes{return RetainedCloneStep::Progress(Default::default());}if !mounted_retirement_grant_funds(self.retirement_demands().expect("fixed receipt close demand"), grant) { return RetainedCloneStep::Progress(Default::default()); }self.closing=true;
        match self.close_phase{
            0=>if let Some(mutation)=self.mutations.last_mut(){if self.close_field==10{self.mutations.pop();self.close_field=0;}else if Self::clear_mutation(mutation,self.close_field){self.close_field+=1;}}else{drop(std::mem::take(&mut *self.mutations));self.close_phase+=1;},
            1=>if let Some(id)=self.undo_ids.last_mut(){Self::clear_text(&mut id.0);self.undo_ids.pop();}else{drop(std::mem::take(&mut *self.undo_ids));self.close_phase+=1;},
            2=>if let Some(inverse)=self.inverses.last_mut(){if self.close_field==4{self.inverses.pop();self.close_field=0;}else if Self::clear_inverse(inverse,self.close_field){self.close_field+=1;}}else{drop(std::mem::take(&mut *self.inverses));self.close_phase+=1;},
            3=>if let Some(edit)=self.member_edits.last_mut(){Self::clear_text(&mut edit.edit_id);self.member_edits.pop();}else{drop(std::mem::take(&mut *self.member_edits));self.close_phase+=1;},
            4=>if let Some(id)=self.command_children.last_mut(){Self::clear_text(id);self.command_children.pop();}else{drop(std::mem::take(&mut *self.command_children));self.close_phase+=1;},
            5..=8=>{let owner=match self.close_phase{5=>&mut self.invocation,6=>&mut self.parent,7=>&mut self.child,_=>&mut self.edit};let step=owner.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant });if owner.terminal_is_empty(){self.close_phase+=1;}return step;},
            _=>{}
        }
        Self::progress(0,bytes)
    }
    pub(crate) fn terminal_is_empty(&self)->bool{self.transferred||(self.close_phase==9&&self.mutations.is_empty()&&self.mutations.capacity()==0&&self.undo_ids.is_empty()&&self.undo_ids.capacity()==0&&self.inverses.is_empty()&&self.inverses.capacity()==0&&self.member_edits.is_empty()&&self.member_edits.capacity()==0&&self.command_children.is_empty()&&self.command_children.capacity()==0&&self.invocation.terminal_is_empty()&&self.parent.terminal_is_empty()&&self.child.terminal_is_empty()&&self.edit.terminal_is_empty())}

}
impl Drop for MountedGroupReceipt{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"mounted group receipt requires exact transfer or funded terminal causal close");}}

#[cfg(test)]
include!("🧪️tests/🦀️.rs");

mod command {use super::*;include!("📚️command/🦀️.rs");}
pub(crate) use command::{PagedCommandLog,CommandAppend,CommandAppendRefusal,CommandPrune,MountedCommandEntry,MountedCommandEntrySource,MountedCommandEntryRefusal};
mod member { use super::*; include!("👤️member/🦀️.rs"); }
pub(crate) use member::{MountedMemberReceipt,MountedMemberReceiptSource,MountedMemberReceiptOutput};

#[cfg(test)]
pub(crate) use member::{MemberReceiptOperation,MemberReceiptPublication,member_receipt_fixture,member_receipt_source};

mod receipt_members { use super::*; include!("../../📚️members/🦀️.rs"); }
pub(crate) use receipt_members::MountedGroupMemberReceipts;
