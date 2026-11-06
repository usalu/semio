//! 📤️ A fixed failure frame lends only original published page spans until paid acknowledgement.
use semio_framework_job::{JobOutcomeDescriptor,OriginalSourceBinding,OriginalTurnFailure,StepContext};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_plugin_guest_contract::original_actor_context::OriginalOwnedJobChunk;

/// 🧱️ One preborn frame retains the paid descriptor and at most one immutable page loan.
pub struct OriginalFailureFrame{binding:Option<OriginalSourceBinding>,job:u64,operation:u64,generation:u64,total:Option<usize>,offset:usize,page:usize,in_page:usize,offered:Option<(usize,usize,usize)>,borrowed:bool}
fn invalid()->OriginalTurnFailure{ValueError::literal(ValueRefusalKind::InvariantViolated,"original failure page changed its Source, job or outstanding loan").into()}
fn charge(cx:&mut StepContext<'_>,bytes:usize)->Result<bool,OriginalTurnFailure>{let grant=cx.retained_grant();if cx.retirement_should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)}
impl OriginalFailureFrame{
    pub const fn empty()->Self{Self{binding:None,job:0,operation:0,generation:0,total:None,offset:0,page:0,in_page:0,offered:None,borrowed:false}}
    pub fn is_empty(&self)->bool{self.binding.is_none()&&self.total.is_none()&&self.offered.is_none()&&!self.borrowed}
    pub fn binding(&self)->Option<OriginalSourceBinding>{self.binding}
    pub fn total(&self)->Option<usize>{self.total}
    pub fn page_index(&self)->usize{self.page}
    fn validate(&self,job:u64,cx:&StepContext<'_>)->Result<(),OriginalTurnFailure>{cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.binding.is_none_or(|binding|!binding.matches(cx))||self.job!=job{return Err(invalid())}Ok(())}
    /// 🎟️ Installs one fixed page cursor only under the actual original caller Source.
    pub fn prepare(&mut self,job:u64,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if job==0{return Err(invalid())}cx.original_source_parent()?.validate_borrowed_context(cx)?;if self.binding.is_some(){self.validate(job,cx)?;return Ok(true)}let bytes=std::mem::size_of::<(Option<OriginalSourceBinding>,u64,u64,u64)>();if !charge(cx,bytes)?{return Ok(false)}self.binding=Some(OriginalSourceBinding::borrow(cx));self.job=job;self.operation=cx.operation().0;self.generation=cx.generation().0;Ok(true)}
    /// 📐️ The original published payload supplies the one immutable byte extent.
    pub fn set_total(&mut self,job:u64,total:usize,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{self.validate(job,cx)?;if let Some(original)=self.total{return if original==total{Ok(true)}else{Err(invalid())}}if !charge(cx,std::mem::size_of::<Option<usize>>())?{return Ok(false)}self.total=Some(total);Ok(true)}
    /// 📃️ Offers a span of the exact published page without moving its body or renewing its cursor.
    pub fn offer(&mut self,job:u64,page:Option<&[u8]>,total:usize,cx:&mut StepContext<'_>)->Result<Option<OriginalOwnedJobChunk>,OriginalTurnFailure>{
        self.validate(job,cx)?;if self.total!=Some(total)||self.offered.is_some()||self.borrowed||self.offset>total{return Err(invalid())}
        if self.offset==total{return Ok(Some(OriginalOwnedJobChunk{offset:self.offset as u64,byte_length:None,output_closed:true}))}
        let page=page.ok_or_else(invalid)?;if self.in_page>=page.len(){return Err(invalid())}
        let pointer=(page.as_ptr() as usize).checked_add(self.in_page).ok_or_else(invalid)?;let metadata=std::mem::size_of::<Option<(usize,usize,usize)>>()+std::mem::size_of::<bool>();let grant=cx.retained_grant();let available=grant.maximum_copy_bytes.saturating_sub(metadata);let length=available.min(page.len()-self.in_page).min(total-self.offset);if length==0{return Ok(None)}if !charge(cx,metadata)?{return Ok(None)}
        self.offered=Some((pointer,length,page.len()));Ok(Some(OriginalOwnedJobChunk{offset:self.offset as u64,byte_length:Some(length as u64),output_closed:false}))
    }
    /// 🔎️ The no-authority scalar may lend only the one span already paid by offer.
    pub fn borrow(&mut self,job:u64,operation:u64,generation:u64,offset:u64,length:u64,page:&[u8])->Result<u64,ValueError>{
        let (pointer,offered,_)=self.offered.ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"no original failure span is offered"))?;
        if self.job!=job||self.operation!=operation||self.generation!=generation||self.borrowed||offset!=self.offset as u64||length!=offered as u64||self.in_page.checked_add(offered).is_none_or(|end|end>page.len())||(page.as_ptr() as usize).checked_add(self.in_page)!=Some(pointer)||pointer>u32::MAX as usize||offered>u32::MAX as usize{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"failure span borrow differs from its original offer"))}
        self.borrowed=true;Ok((pointer as u64)|((offered as u64)<<32))
    }
    /// 🤝️ Exact handback advances the retained physical cursor once under the same Source.
    pub fn acknowledge_chunk(&mut self,job:u64,offset:u64,length:u64,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate(job,cx)?;let (_,offered,page_length)=self.offered.ok_or_else(invalid)?;if !self.borrowed||offset!=self.offset as u64||length!=offered as u64{return Err(invalid())}
        let bytes=std::mem::size_of::<(usize,usize,usize,Option<(usize,usize,usize)>,bool)>();if !charge(cx,bytes)?{return Ok(false)}
        self.offset=self.offset.checked_add(offered).ok_or_else(invalid)?;self.in_page=self.in_page.checked_add(offered).ok_or_else(invalid)?;if self.in_page==page_length{self.page=self.page.checked_add(1).ok_or_else(invalid)?;self.in_page=0}self.offered=None;self.borrowed=false;Ok(true)
    }
    /// 🧹️ The descriptor acknowledges its original admission before the fixed cursor is cleared.
    pub fn acknowledge(&mut self,job:u64,descriptor:&mut Option<JobOutcomeDescriptor>,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        self.validate(job,cx)?;if self.offered.is_some()||self.borrowed||self.total!=Some(self.offset){return Err(invalid())}
        if let Some(original)=descriptor.as_mut(){if !original.is_acknowledged(){let grant=cx.retained_grant();let turn=original.acknowledge(grant);semio_framework_job::receive_original_retained_work(cx,turn)?;return Ok(false)}let bytes=std::mem::size_of::<Option<JobOutcomeDescriptor>>();if !charge(cx,bytes)?{return Ok(false)}descriptor.take();return Ok(false)}
        let bytes=std::mem::size_of::<Self>();if !charge(cx,bytes)?{return Ok(false)}self.binding=None;self.job=0;self.operation=0;self.generation=0;self.total=None;self.offset=0;self.page=0;self.in_page=0;Ok(true)
    }
    /// 🧹️ A frame with no admitted descriptor or offered page returns its original metadata.
    pub fn close_unpublished(&mut self,cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{
        if self.is_empty(){return Ok(true)}self.validate(self.job,cx)?;if self.total.is_some()||self.offered.is_some()||self.borrowed{return Ok(false)}if !charge(cx,std::mem::size_of::<Self>())?{return Ok(false)}self.binding=None;self.job=0;self.operation=0;self.generation=0;Ok(true)
    }
}
impl Drop for OriginalFailureFrame{fn drop(&mut self){assert!(std::thread::panicking()||self.is_empty(),"original failure frame still holds its descriptor or page")}}

#[cfg(test)]
mod tests{
    use super::*;
    use semio_framework_job::{Generation,OperationId};
    use semio_framework_job_testing_caller::OriginalJobFixtureCaller;
    use semio_framework_value::RetainedCloneGrant;
    use semio_framework_value_testing_allocator::observe_retirement_allocations as observe;
    fn clock()->Option<u64>{Some(1)}
    #[test]
    fn unpublished_failure_frame_refuses_foreign_and_denied_original_sources(){
        let stock=RetainedCloneGrant{maximum_items:10_000,maximum_copy_bytes:1_000_000,maximum_capacity_bytes:1_000_000,maximum_release_bytes:1_000_000,maximum_depth:64};
        let mut caller=OriginalJobFixtureCaller::admit(OperationId(71),Generation(3),stock,clock,1_000_000,100_000);
        let mut foreign=OriginalJobFixtureCaller::admit(OperationId(72),Generation(3),stock,clock,1_000_000,100_000);
        let mut frame=OriginalFailureFrame::empty();
        let(result,heap)=observe(||caller.turn(stock,|cx|frame.prepare(91,cx)));assert!(result.unwrap());assert_eq!(heap,(0,0));
        let(result,heap)=observe(||foreign.turn(stock,|cx|frame.prepare(91,cx)));assert!(result.is_err());assert_eq!(heap,(0,0));assert_eq!(foreign.receipt(),Default::default());
        let denied=RetainedCloneGrant{maximum_copy_bytes:0,..stock};let(result,heap)=observe(||caller.turn(denied,|cx|frame.close_unpublished(cx)));assert!(!result.unwrap());assert_eq!(heap,(0,0));assert_eq!(caller.receipt(),Default::default());
        let(result,heap)=observe(||caller.turn(stock,|cx|frame.close_unpublished(cx)));assert!(result.unwrap());assert_eq!(heap,(0,0));assert!(frame.is_empty());drop(frame);foreign.retire(stock);caller.retire(stock);
    }
}
