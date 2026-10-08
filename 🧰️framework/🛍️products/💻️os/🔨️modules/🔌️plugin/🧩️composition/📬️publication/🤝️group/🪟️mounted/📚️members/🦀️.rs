use super::*;

struct MountedMemberReceiptRows {
    rows: [Option<Box<MountedMemberReceipt>>; 65],
    child_count: usize,
}
impl MountedGroupPreparedChildEdits for MountedMemberReceiptRows {
    fn len(&self) -> usize { self.child_count }
    fn get(&self, index: usize) -> Option<&str> { if index >= self.child_count { return None; } self.rows[index + 1].as_ref()?.ready_edit_id() }
}

pub(crate) struct MountedGroupMemberReceipts {
    rows: MountedMemberReceiptRows,
    identity: [(usize,usize); 2],
    parent_handle: ArtifactHandle,
    parent_touched: bool,
    receipt_cursor: usize,
    issuer: MountedArtifactHandleIssuer,
    issued_document: Option<ArtifactHandle>,
    operation_count: usize,
    final_receipt: Option<MountedGroupReceipt>,
    final_phase: usize,
    final_cursor: usize,
    pending_triple: Option<(KernelMutation,MutationId,InverseMutation)>,
    pending_close_field: usize,
    close_cursor: usize,
    closing: bool,
    transferred: bool,
    issuer_closed: bool,
}

impl MountedGroupMemberReceipts {
    fn refusal(message: &str) -> ValueError { ValueError::new(ValueRefusalKind::InvariantViolated,message) }
    fn identity(group: &str, actor: &str) -> [(usize,usize);2] { [(group.as_ptr() as usize,group.len()),(actor.as_ptr() as usize,actor.len())] }
    fn from_original(group: &str, actor: &str, parent: ArtifactHandle, parent_touched: bool, children: usize) -> Result<Self,ValueError> {
        if children > 64 { return Err(Self::refusal("mounted receipt rows exceed original admitted child count")); }
        Ok(Self { rows: MountedMemberReceiptRows { rows: std::array::from_fn(|_|None), child_count: children }, identity: Self::identity(group,actor), parent_handle: parent, parent_touched, receipt_cursor: usize::from(!parent_touched), issuer: MountedArtifactHandleIssuer::new(), issued_document: None, operation_count: 0, final_receipt: None, final_phase: 0, final_cursor: usize::from(!parent_touched), pending_triple: None, pending_close_field: 0, close_cursor: 0, closing: false, transferred: false, issuer_closed: false })
    }
    /// 📦️ Seals actual parent handle, original group/actor borrows and fixed member cardinality without allocation.
    pub(crate) fn new<M: SpaceMember + MemberFactory>(group: &PrivateChildPublicationGroup<M>, actor: &str, parent_handle: ArtifactHandle) -> Result<Self,ValueError> { Self::from_original(group.group_id(),actor,parent_handle,group.parent_touched(),group.child_count()) }
    pub(crate) fn frame_birth_bytes() -> usize { size_of::<Self>() }
    fn validate<M: SpaceMember + MemberFactory>(&self,group: &PrivateChildPublicationGroup<M>,actor: &str) -> Result<(),ValueError> {
        if self.closing || self.transferred || self.identity != Self::identity(group.group_id(),actor) || self.rows.child_count != group.child_count() || self.parent_touched != group.parent_touched() { return Err(Self::refusal("mounted receipt set lost its original member/group/actor authority")); } Ok(())
    }
    pub(crate) fn receipt_index(&self) -> Option<usize> { (!self.closing && !self.transferred && self.receipt_cursor <= self.rows.child_count).then_some(self.receipt_cursor) }
    pub(crate) fn ready_for_final(&self) -> bool { !self.closing && !self.transferred && self.receipt_cursor > self.rows.child_count }
    fn source<'a>(&self, group: &'a PrivateChildPublicationGroup<impl SpaceMember + MemberFactory>, actor: &'a str) -> Result<Option<MountedMemberReceiptSource<'a>>,ValueError> {
        self.validate(group,actor)?;
        let Some(index)=self.receipt_index() else { return Ok(None); };
        let Some(publication)=group.prepared_publication(index) else { return Ok(None); };
        let Some(document)=(if index==0 { Some(self.parent_handle) } else { self.issued_document }) else { return Ok(None); };
        Ok(Some(MountedMemberReceiptSource { publication, document, invocation_id: group.group_id(), group_id: group.group_id(), fallback_author: actor }))
    }
    pub(crate) fn next_capacity_byte_demand<M: SpaceMember + MemberFactory>(&self,group: &PrivateChildPublicationGroup<M>,actor: &str) -> Result<usize,ValueError> {
        let Some(source)=self.source(group,actor)? else { return Ok(0); };
        self.rows.rows[self.receipt_cursor].as_ref().map_or(Ok(MountedMemberReceipt::frame_birth_bytes()),|row|row.next_capacity_byte_demand(&source))
    }
    pub(crate) fn next_release_byte_demand<M: SpaceMember + MemberFactory>(&self,group: &PrivateChildPublicationGroup<M>,actor: &str) -> Result<usize,ValueError> {
        let Some(source)=self.source(group,actor)? else { return Ok(0); };
        self.rows.rows[self.receipt_cursor].as_ref().map_or(Ok(0),|row|row.next_release_byte_demand(&source))
    }
    fn progress(birth:usize,release:usize) -> RetainedCloneStep { MountedGroupReceipt::progress(birth,release) }
    fn advance_row(&mut self,index:usize,source:&MountedMemberReceiptSource<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.rows.rows[index].is_none(){let birth=MountedMemberReceipt::frame_birth_bytes();if grant.maximum_capacity_bytes<birth{return Ok(RetainedCloneStep::Progress(Default::default()));}self.rows.rows[index]=Some(Box::new(MountedMemberReceipt::new(source)?));return Ok(Self::progress(birth,0));}
        self.rows.rows[index].as_mut().unwrap().advance(source,grant)
    }
    /// 🧾️ Completes one original member row before its exact pointer acknowledgment unlocks the next member.
    pub(crate) fn advance_receipt<M: SpaceMember + MemberFactory>(&mut self,group:&mut PrivateChildPublicationGroup<M>,registry:&ChildMemberRegistry<M>,actor:&str,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.validate(group,actor)?;
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        let Some(index)=self.receipt_index()else{return Ok(RetainedCloneStep::Complete(Default::default()));};
        let Some(token)=group.prepared_receipt_identity(index)else{return Ok(RetainedCloneStep::Progress(Default::default()));};
        if index>0&&self.issued_document.is_none(){if !self.issuer.ready(){return self.issuer.advance(group.prepared_child_artifact_id(index,registry).ok_or_else(||Self::refusal("mounted child lost its original prepared artifact identity"))?,grant);}self.issued_document=self.issuer.take(grant);return Ok(Self::progress(0,0));}
        let source=self.source(group,actor)?.ok_or_else(||Self::refusal("mounted member receipt is not prepared"))?;
        let row_complete=self.rows.rows[index].as_ref().is_some_and(|row|row.is_complete());
        if !row_complete{return self.advance_row(index,&source,grant);}
        let operations=self.rows.rows[index].as_ref().unwrap().operation_count();
        let total=self.operation_count.checked_add(operations).ok_or_else(||Self::refusal("mounted receipt operation count overflow"))?;
        if !group.acknowledge_prepared_receipt_identity(token,grant){return Err(Self::refusal("mounted receipt original acknowledgment became stale"));}
        self.operation_count=total;self.receipt_cursor+=1;self.issuer=MountedArtifactHandleIssuer::new();self.issued_document=None;Ok(Self::progress(0,0))
    }
    fn final_source<'a>(rows:&'a MountedMemberReceiptRows,parent_touched:bool,invocation:&'a str)->Result<MountedGroupReceiptSource<'a>,ValueError>{
        let parent=if parent_touched{Some(rows.rows[0].as_ref().and_then(|row|row.ready_edit_id()).ok_or_else(||Self::refusal("mounted parent row must complete before final metadata"))?)}else{None};
        Ok(MountedGroupReceiptSource{invocation_id:invocation,parent_edit_id:parent,child_edit_ids:MountedGroupChildEditSource::Ready(rows)})
    }
    fn validate_final(&self,invocation:&str)->Result<(),ValueError>{if !self.ready_for_final()||self.identity[0]!=(invocation.as_ptr()as usize,invocation.len()){return Err(Self::refusal("mounted final receipt lost its original completed row set"));}Ok(())}
    pub(crate) fn next_final_capacity_byte_demand(&self,invocation:&str)->Result<usize,ValueError>{
        self.validate_final(invocation)?;let Some(final_receipt)=self.final_receipt.as_ref()else{return Ok(0);};
        if self.final_phase==0{return final_receipt.next_capacity_byte_demand(&Self::final_source(&self.rows,self.parent_touched,invocation)?);}
        if self.final_phase==2&&self.final_cursor<=self.rows.child_count{let row=self.rows.rows[self.final_cursor].as_ref().unwrap();return final_receipt.next_member_capacity_byte_demand(row.ready_document().unwrap(),row.ready_edit_id().unwrap());}Ok(0)
    }
    pub(crate) fn next_final_release_byte_demand(&self)->usize{self.final_receipt.as_ref().map_or(0,|row|if self.final_phase==0{row.next_metadata_release_byte_demand()}else if self.final_phase==2{row.next_member_release_byte_demand()}else{0})}
    /// 🧬️ Assembles final metadata from noncontiguous completed rows and moves one original triple or member edit per turn.
    pub(crate) fn advance_final(&mut self,invocation:&str,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.validate_final(invocation)?;if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.final_receipt.is_none(){self.final_receipt=Some(MountedGroupReceipt::new(self.operation_count,&Self::final_source(&self.rows,self.parent_touched,invocation)?)?);return Ok(Self::progress(0,0));}
        let final_receipt=self.final_receipt.as_mut().unwrap();
        if self.final_phase==0{let step=final_receipt.advance_metadata(&Self::final_source(&self.rows,self.parent_touched,invocation)?,grant)?;if let RetainedCloneStep::Complete(progress)=step{self.final_phase=1;return Ok(RetainedCloneStep::Progress(progress));}return Ok(step);}
        if self.final_phase==1{if self.final_cursor>self.rows.child_count{self.final_cursor=usize::from(!self.parent_touched);self.final_phase=2;return Ok(Self::progress(0,0));}if self.pending_triple.is_none(){self.pending_triple=self.rows.rows[self.final_cursor].as_mut().unwrap().take_next_triple(grant);if self.pending_triple.is_none(){self.final_cursor+=1;}return Ok(Self::progress(0,0));}return final_receipt.push_mutation_triple(&mut self.pending_triple,grant);}
        if self.final_phase==2{if self.final_cursor>self.rows.child_count{self.final_phase=3;return Ok(Self::progress(0,0));}let row=self.rows.rows[self.final_cursor].as_ref().unwrap();let count=final_receipt.member_edits.len();let step=final_receipt.push_member_edit(row.ready_document().unwrap(),row.ready_edit_id().unwrap(),grant)?;if final_receipt.member_edits.len()>count{self.final_cursor+=1;}return Ok(step);}
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    pub(crate) fn final_receipt(&self)->Option<&MountedGroupReceipt>{self.final_receipt.as_ref()}
    pub(crate) fn final_receipt_mut(&mut self)->Option<&mut MountedGroupReceipt>{self.final_receipt.as_mut()}
    pub(crate) fn final_ready(&self)->bool{!self.closing&&self.final_receipt.as_ref().is_some_and(MountedGroupReceipt::is_complete)}
    pub(crate) fn take_final(&mut self,grant:RetainedCloneGrant)->Option<MountedGroupReceiptOutput>{let output=self.final_receipt.as_mut()?.take(grant)?;self.transferred=true;Some(output)}
    pub(crate) fn next_close_byte_demand(&self)->Result<usize,ValueError>{
        if let Some(triple)=self.pending_triple.as_ref(){return Ok(match self.pending_close_field{0..=9=>MountedGroupReceipt::mutation_demand(&triple.0,self.pending_close_field),10=>triple.1.0.capacity(),11..=14=>MountedGroupReceipt::inverse_demand(&triple.2,self.pending_close_field-11),_=>0});}
        if let Some(row)=self.final_receipt.as_ref().filter(|row|!row.terminal_is_empty()){return Ok(row.next_close_byte_demand());}
        if self.close_cursor<=self.rows.child_count{if let Some(row)=self.rows.rows[self.close_cursor].as_ref(){return if row.terminal_is_empty(){Ok(MountedMemberReceipt::frame_birth_bytes())}else{row.next_close_byte_demand()};}}Ok(0)
    }
    /// 📏️ Retains exact row currencies and separately prices each paid terminal frame.
    pub(crate) fn retirement_demands(&self) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.terminal_is_empty() { return Ok(Default::default()); }
        if self.pending_triple.is_some() { return Ok(semio_framework_value::RetirementDemand { release_bytes: self.next_close_byte_demand()?, depth: if matches!(self.pending_close_field, 7 | 8 | 14) { 4 } else { 3 }, ..Default::default() }); }
        if let Some(row) = self.final_receipt.as_ref().filter(|row| !row.terminal_is_empty()) { return mounted_nested_retirement_demand(row.retirement_demands()?, 1); }
        if self.final_receipt.is_some() { return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() }); }
        if self.close_cursor <= self.rows.child_count {
            if let Some(row) = self.rows.rows[self.close_cursor].as_ref() {
                return if row.terminal_is_empty() { Ok(semio_framework_value::RetirementDemand { release_bytes: MountedMemberReceipt::frame_birth_bytes(), depth: 2, ..Default::default() }) } else { mounted_nested_retirement_demand(row.retirement_demands()?, 2) };
            }
            return Ok(semio_framework_value::RetirementDemand { depth: 1, ..Default::default() });
        }
        Ok(semio_framework_value::RetirementDemand { depth: 2, ..Default::default() })
    }
    /// 🍂️ Keeps all completed row edits, typed vectors and physical frames until their exact release phase is funded.
    pub(crate) fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}let demand=self.next_close_byte_demand()?;if grant.maximum_items==0||grant.maximum_release_bytes<demand{return Ok(RetainedCloneStep::Progress(Default::default()));}if !mounted_retirement_grant_funds(self.retirement_demands()?, grant) { return Ok(RetainedCloneStep::Progress(Default::default())); }self.closing=true;
        if let Some(triple)=self.pending_triple.as_mut(){let ready=match self.pending_close_field{0..=9=>MountedGroupReceipt::clear_mutation(&mut triple.0,self.pending_close_field),10=>{MountedGroupReceipt::clear_text(&mut triple.1.0);true},11..=14=>MountedGroupReceipt::clear_inverse(&mut triple.2,self.pending_close_field-11),_=>{self.pending_triple=None;self.pending_close_field=0;return Ok(Self::progress(0,0));}};if ready{self.pending_close_field+=1;}return Ok(Self::progress(0,demand));}
        if let Some(row)=self.final_receipt.as_mut().filter(|row|!row.terminal_is_empty()){return Ok(row.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant }));}
        if self.final_receipt.is_some(){self.final_receipt=None;return Ok(Self::progress(0,0));}
        if self.close_cursor<=self.rows.child_count{if let Some(row)=self.rows.rows[self.close_cursor].as_mut(){if !row.terminal_is_empty(){return row.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 2, ..grant });}drop(self.rows.rows[self.close_cursor].take());self.close_cursor+=1;return Ok(Self::progress(0,demand));}self.close_cursor+=1;return Ok(Self::progress(0,0));}
        let step=self.issuer.close_step(RetainedCloneGrant { maximum_depth: grant.maximum_depth - 1, ..grant });self.issuer_closed=true;Ok(step)
    }
    pub(crate) fn terminal_is_empty(&self)->bool{self.closing&&self.pending_triple.is_none()&&self.final_receipt.is_none()&&self.close_cursor>self.rows.child_count&&self.issuer_closed}
    pub(crate) fn terminal_frame_byte_demand(&self)->Option<usize>{self.terminal_is_empty().then_some(size_of::<Self>())}
}
impl Drop for MountedGroupMemberReceipts{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"mounted member receipt set requires all original rows and physical frames funded before Drop");}}

#[cfg(test)]
include!("🧪️tests/🦀️.rs");
