//! 🎒️ Durable output custody composes the shared retained codec and declared semio framing.
use super::*;
use ::pack::record::BorrowedPackDocumentCursor;
use semio_framework_dsl_record::{BorrowedDslRecord,native_encoding::FieldProjectionSource};

#[derive(Default)]
pub struct DurablePackOutputV1 { cursor:Option<Box<BorrowedPackDocumentCursor>>, output:Option<Vec<u8>>, prefix:usize, phase:u8 }
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
impl DurablePackOutputV1 {
    pub fn next_demand(&self)->Result<RetirementDemand,ValueError>{
        Ok(match self.phase {
            0=>RetirementDemand{capacity_bytes:size_of::<BorrowedPackDocumentCursor>(),..copy(size_of::<BorrowedPackDocumentCursor>()+size_of::<Option<Box<BorrowedPackDocumentCursor>>>()+size_of::<u8>()+size_of::<usize>())},
            1 if self.cursor.as_ref().unwrap().output_len().is_some()=>copy(size_of::<u8>()),
            1|4=>{let cursor=self.cursor.as_ref().unwrap();RetirementDemand{capacity_bytes:cursor.next_capacity_byte_demand()?,copy_bytes:cursor.next_copy_byte_demand()?+usize::from(self.phase==4),depth:cursor.next_depth_demand()?+1,..Default::default()}},
            2=>RetirementDemand{capacity_bytes:self.cursor.as_ref().unwrap().output_len().unwrap().checked_add(self.prefix).ok_or_else(||invalid("durable Pack total length overflow"))?,..copy(size_of::<Vec<u8>>()+size_of::<u8>())},
            3=>copy(1+size_of::<usize>()),
            5=>{let mut demand=self.cursor.as_ref().unwrap().retirement_demands()?;demand.depth+=1;Ok::<_,ValueError>(demand)?},
            6=>RetirementDemand{release_bytes:size_of::<BorrowedPackDocumentCursor>(),..copy(size_of::<Option<Box<BorrowedPackDocumentCursor>>>()+size_of::<u8>())},
            _=>Default::default(),
        })
    }
    pub fn advance<T:BorrowedDslRecord+FieldProjectionSource>(&mut self,source:&T,envelope:&str,maximum:usize,grant:RetainedCloneGrant,native:&mut NativeEncodeControl<'_>)->Result<RetainedCloneProgress,ValueError>{
        if self.ready(){return Ok(Default::default());}
        let demand=self.next_demand()?;if !permit(demand,grant){return Ok(Default::default());}native.checkpoint()?;
        let mut receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        match self.phase {
            0=>{self.prefix=semio_framework_os_store_text_io::semio_format::declared_envelope_prefix_len(envelope,semio_framework_os_store_text_io::semio_format::Component::Pack,1)?;let ceiling=maximum.checked_sub(self.prefix).ok_or_else(||invalid("durable envelope exceeds output ceiling"))?;native.admit_turn_capacity(grant.maximum_capacity_bytes)?;native.charge(size_of::<BorrowedPackDocumentCursor>())?;self.cursor=Some(Box::new(BorrowedPackDocumentCursor::with_maximum_file_len(ceiling).with_maximum_body_len(maximum).with_maximum_source_depth(DURABLE_OWNED_GROUP_JSON_MAX_DEPTH)));self.phase=1;receipt.retained_capacity_bytes=size_of::<BorrowedPackDocumentCursor>();},
            1 if self.cursor.as_ref().unwrap().output_len().is_some()=>self.phase=2,
            1|4=>{let emitting=self.phase==4;let mut byte=[0];let child=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes-usize::from(emitting),maximum_depth:grant.maximum_depth-1,..grant};let step=match self.cursor.as_mut().unwrap().advance(source,T::RECORD,if emitting{&mut byte}else{&mut[]},child){Ok(step)=>step,Err(failure)=>{let mut progress=failure.reason.retained_progress();if failure.written_bytes!=0{if !emitting||failure.written_bytes!=1{return Err(invalid("durable Pack refusal exposed an invalid output prefix").with_retained_progress(progress));}let output=self.output.as_mut().unwrap();if output.len()>=output.capacity(){return Err(invalid("durable Pack refusal exceeded measured output").with_retained_progress(progress));}output.push(byte[0]);progress.copied_bytes+=1;}return Err(failure.reason.with_retained_progress(progress));}};receipt=step.progress;if step.written_bytes!=0{let output=self.output.as_mut().unwrap();if output.len()>=output.capacity(){return Err(invalid("durable Pack changed its measured output length").with_retained_progress(receipt));}output.push(byte[0]);receipt.copied_bytes+=1;}if step.complete{self.phase=5;}},
            2=>{native.admit_turn_capacity(grant.maximum_capacity_bytes)?;self.output=Some(native.allocate_vec(demand.capacity_bytes)?);self.phase=3;receipt.retained_capacity_bytes=demand.capacity_bytes;},
            3=>{let output=self.output.as_mut().unwrap();if let Some(byte)=semio_framework_os_store_text_io::semio_format::declared_binary_prefix_byte(envelope,semio_framework_os_store_text_io::semio_format::Component::Pack,1,output.len())?{output.push(byte);}else{self.phase=4;}},
            5=>{let step=self.cursor.as_mut().unwrap().close(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map_err(|failure|failure.reason)?;receipt=step.progress;if step.complete{self.phase=6;}},
            6=>{self.cursor=None;self.phase=7;receipt.released_bytes=size_of::<BorrowedPackDocumentCursor>();},
            _=>return Err(invalid("durable Pack caller has an invalid frontier")),
        }
        if !receipt.fits(grant){return Err(invalid("durable Pack child exceeded original grant").with_retained_progress(receipt));}Ok(receipt)
    }
    pub fn ready(&self)->bool{self.phase==7&&self.output.is_some()}
    pub fn next_take_demand(&self)->RetirementDemand{copy(size_of::<Option<Vec<u8>>>()+size_of::<u8>())}
    pub fn take(&mut self,grant:RetainedCloneGrant)->Option<(Vec<u8>,RetainedCloneProgress)>{let demand=self.next_take_demand();if !self.ready()||!permit(demand,grant){return None;}self.phase=8;Some((self.output.take()?,RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
    pub fn next_close_demand(&self)->Result<RetirementDemand,ValueError>{if self.cursor.as_ref().is_some_and(|cursor|!cursor.terminal_is_empty()){let mut demand=self.cursor.as_ref().unwrap().retirement_demands()?;demand.depth+=1;Ok(demand)}else{Ok(RetirementDemand{release_bytes:self.output.as_ref().map_or(0,Vec::capacity)+usize::from(self.cursor.is_some())*size_of::<BorrowedPackDocumentCursor>(),..copy(size_of::<Option<Vec<u8>>>()+size_of::<Option<Box<BorrowedPackDocumentCursor>>>()+size_of::<u8>())})}}
    pub fn close(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError>{if self.terminal_is_empty(){return Ok(Default::default());}let demand=self.next_close_demand()?;if !permit(demand,grant){return Ok(Default::default());}if self.cursor.as_ref().is_some_and(|cursor|!cursor.terminal_is_empty()){return self.cursor.as_mut().unwrap().close(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).map(|step|step.progress).map_err(|failure|failure.reason);}self.cursor=None;self.output=None;self.phase=8;Ok(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()})}
    pub fn terminal_is_empty(&self)->bool{self.phase==8&&self.output.is_none()&&self.cursor.is_none()}
}
impl Drop for DurablePackOutputV1 {fn drop(&mut self){assert!(std::thread::panicking()||(self.output.is_none()&&self.cursor.as_ref().is_none_or(|cursor|cursor.terminal_is_empty())),"durable Pack output must hand off or retire its exact children before Drop");}}
