//! 📄️ Retains private genesis pages through separately bounded source copy and whole backing release.
use crate::store::{OwnedSchemaDecodePage,OwnedSchemaDecodePages,OwnedSchemaDecodeCredits,OwnedSchemaDecodeAdmissionFault,OWNED_SCHEMA_DECODE_PAGE_BYTES};
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep};
use std::mem::ManuallyDrop;

#[derive(Default)]
pub(crate) struct PrivateChildGenesisInput{
    encoder:crate::store::MemberGenesisEnvelopeEncoder,
    pages:PrivateChildInputPages,
    ready:bool,
    closing:bool,
}
impl PrivateChildGenesisInput{
    pub(crate) fn next_capacity_byte_demand(&self,source:crate::store::MemberGenesisEnvelopeSource<'_>)->Result<usize,semio_framework_value::ValueError>{
        if let Some(length)=self.encoder.encoded_length(source)?{return Ok(if self.pages.pages.is_none(){OwnedSchemaDecodePages::allocation_byte_demand_for(length.div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES)).unwrap_or(usize::MAX)}else{0});}
        Ok(self.encoder.next_capacity_byte_demand(source).unwrap_or(usize::MAX))
    }
    pub(crate) fn advance(&mut self,source:crate::store::MemberGenesisEnvelopeSource<'_>,grant:RetainedCloneGrant)->Result<RetainedCloneStep,semio_framework_value::ValueError>{
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.closing{return Err(input_fault());}
        if self.ready{return Ok(RetainedCloneStep::Complete(Default::default()));}
        let Some(length)=self.encoder.encoded_length(source)? else{
            let progress=self.encoder.encode_step(source,&mut[],grant)?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(progress.processed_bytes!=0||progress.retained_capacity_bytes!=0),copied_bytes:progress.processed_bytes,retained_capacity_bytes:progress.retained_capacity_bytes,..Default::default()}));
        };
        let step=self.pages.prepare_output(length,grant).map_err(|_|input_fault())?;
        if matches!(step,RetainedCloneStep::Complete(_)){self.ready=true;return Ok(step);}
        if step.progress()!=Default::default(){return Ok(step);}
        if self.pages.pages.is_none(){return Ok(step);}
        let output=self.pages.writable_output(grant.maximum_copy_bytes).ok_or_else(input_fault)?;
        let progress=self.encoder.encode_step(source,output,grant)?;
        self.pages.commit_output(progress.written_bytes).map_err(|_|input_fault())?;
        if progress.complete{self.pages.finish_output().map_err(|_|input_fault())?;}
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(progress.processed_bytes!=0),copied_bytes:progress.processed_bytes,retained_capacity_bytes:progress.retained_capacity_bytes,..Default::default()}))
    }
    pub(crate) fn take_ready(&mut self)->Option<OwnedSchemaDecodePages>{if self.closing||!self.ready{return None;}self.pages.take_ready()}
    pub(crate) fn begin_close(&mut self){self.closing=true;self.pages.begin_close();}
    pub(crate) fn next_close_byte_demand(&self)->usize{if !self.pages.terminal_is_empty(){self.pages.next_close_byte_demand()}else{self.encoder.next_close_byte_demand()}}
    pub(crate) fn close_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,semio_framework_value::ValueError>{
        if !self.closing{return Err(input_fault());}
        if !self.pages.terminal_is_empty(){return self.pages.close_granted(grant).map_err(|_|input_fault());}
        self.encoder.close_granted(grant)
    }
    pub(crate) fn terminal_is_empty(&self)->bool{self.pages.terminal_is_empty()&&self.encoder.terminal_is_empty()}
}
fn input_fault()->semio_framework_value::ValueError{semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvariantViolated,"private genesis input lost its exact admitted source or output prefix")}

#[derive(Default)]
pub(crate) struct PrivateChildInputPages{
    pages:ManuallyDrop<Option<OwnedSchemaDecodePages>>,
    pending:Option<OwnedSchemaDecodePage>,
    source:Option<(usize,usize)>,
    position:usize,
    maximum_bytes:Option<usize>,
    output_finished:bool,
    lent_output_bytes:usize,
    ready:bool,
    closing:bool,
}
impl PrivateChildInputPages{
    pub(crate) fn prepare_output(&mut self,maximum_bytes:usize,grant:RetainedCloneGrant)->Result<RetainedCloneStep,OwnedSchemaDecodeAdmissionFault>{
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.source.is_some(){return Err(OwnedSchemaDecodeAdmissionFault::Sealed);}
        if self.closing{return Err(OwnedSchemaDecodeAdmissionFault::Sealed);}
        if self.maximum_bytes.is_some_and(|prior|prior!=maximum_bytes){return Err(OwnedSchemaDecodeAdmissionFault::ByteCapacity);}
        if self.ready{return Ok(RetainedCloneStep::Complete(Default::default()));}
        if self.pages.is_none(){
            if maximum_bytes==0{return Err(OwnedSchemaDecodeAdmissionFault::ZeroCapacity);}
            let count=maximum_bytes.div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES);
            let birth=OwnedSchemaDecodePages::allocation_byte_demand_for(count).ok_or(OwnedSchemaDecodeAdmissionFault::ByteCapacity)?;
            if birth>grant.maximum_capacity_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
            *self.pages=Some(OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits{maximum_pages:count,maximum_bytes})?);self.maximum_bytes=Some(maximum_bytes);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:birth,..Default::default()}));
        }
        if self.pending.as_ref().is_some_and(|page|page.len()==OWNED_SCHEMA_DECODE_PAGE_BYTES||self.output_finished){
            let page=self.pending.take().unwrap();self.pages.as_mut().unwrap().admit_page(page).map_err(|(fault,page)|{self.pending=Some(page);fault})?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));
        }
        if self.output_finished{self.pages.as_mut().unwrap().seal()?;self.ready=true;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        if self.pending.is_none(){self.pending=Some(OwnedSchemaDecodePage::empty());return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        Ok(RetainedCloneStep::Progress(Default::default()))
    }
    pub(crate) fn writable_output(&mut self,maximum_copy_bytes:usize)->Option<&mut[u8]>{
        if self.closing||self.ready||self.output_finished{return None;}
        let remaining=self.maximum_bytes?.saturating_sub(self.position);
        let page=self.pending.as_mut()?;self.lent_output_bytes=maximum_copy_bytes.min(remaining).min(OWNED_SCHEMA_DECODE_PAGE_BYTES-page.len());
        Some(page.writable_prefix(self.lent_output_bytes))
    }
    pub(crate) fn commit_output(&mut self,written:usize)->Result<(),OwnedSchemaDecodeAdmissionFault>{
        if self.closing||self.ready||self.output_finished{return Err(OwnedSchemaDecodeAdmissionFault::Sealed);}
        if written>self.lent_output_bytes{return Err(OwnedSchemaDecodeAdmissionFault::ByteCapacity);}
        if written>self.maximum_bytes.ok_or(OwnedSchemaDecodeAdmissionFault::ByteCapacity)?.saturating_sub(self.position){return Err(OwnedSchemaDecodeAdmissionFault::ByteCapacity);}
        self.pending.as_mut().ok_or(OwnedSchemaDecodeAdmissionFault::ByteCapacity)?.commit_written(written).map_err(|_|OwnedSchemaDecodeAdmissionFault::ByteCapacity)?;self.position+=written;self.lent_output_bytes=0;Ok(())
    }
    pub(crate) fn finish_output(&mut self)->Result<(),OwnedSchemaDecodeAdmissionFault>{if Some(self.position)!=self.maximum_bytes{return Err(OwnedSchemaDecodeAdmissionFault::ByteCapacity);}self.output_finished=true;Ok(())}
    pub(crate) fn advance_source(&mut self,source:&[u8],grant:RetainedCloneGrant)->Result<RetainedCloneStep,OwnedSchemaDecodeAdmissionFault>{
        let paused=||RetainedCloneStep::Progress(Default::default());
        if grant.maximum_items==0{return Ok(paused());}
        if self.maximum_bytes.is_some(){return Err(OwnedSchemaDecodeAdmissionFault::Sealed);}
        if self.closing{return Err(OwnedSchemaDecodeAdmissionFault::Sealed);}
        let identity=(source.as_ptr()as usize,source.len());
        if self.source.is_some_and(|original|original!=identity){return Err(OwnedSchemaDecodeAdmissionFault::Sealed);}
        if self.ready{return Ok(RetainedCloneStep::Complete(Default::default()));}
        if self.pages.is_none(){
            if source.is_empty(){return Err(OwnedSchemaDecodeAdmissionFault::ZeroCapacity);}
            let count=source.len().div_ceil(OWNED_SCHEMA_DECODE_PAGE_BYTES);
            let birth=OwnedSchemaDecodePages::allocation_byte_demand_for(count).ok_or(OwnedSchemaDecodeAdmissionFault::ByteCapacity)?;
            if birth>grant.maximum_capacity_bytes{return Ok(paused());}
            *self.pages=Some(OwnedSchemaDecodePages::try_with_credits(OwnedSchemaDecodeCredits{maximum_pages:count,maximum_bytes:source.len()})?);
            self.source=Some(identity);
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:birth,..Default::default()}));
        }
        if self.pending.as_ref().is_some_and(|page|page.len()==OWNED_SCHEMA_DECODE_PAGE_BYTES||self.position==source.len()){
            let page=self.pending.take().unwrap();
            self.pages.as_mut().unwrap().admit_page(page).map_err(|(fault,page)|{self.pending=Some(page);fault})?;
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));
        }
        if self.position==source.len(){self.pages.as_mut().unwrap().seal()?;self.ready=true;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        let copied=self.pending.get_or_insert_with(OwnedSchemaDecodePage::empty).append_granted(&source[self.position..],grant.maximum_copy_bytes);
        self.position+=copied;
        Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:usize::from(copied!=0),copied_bytes:copied,..Default::default()}))
    }
    pub(crate) fn take_ready(&mut self)->Option<OwnedSchemaDecodePages>{if !self.ready||self.closing{return None;}self.source=None;self.pages.take()}
    pub(crate) fn begin_close(&mut self){self.closing=true;}
    pub(crate) fn next_close_byte_demand(&self)->usize{if self.pending.is_some(){return 0;}self.pages.as_ref().map_or(0,|pages|if pages.page_count()==0{pages.allocation_byte_demand()}else{0})}
    pub(crate) fn close_granted(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,OwnedSchemaDecodeAdmissionFault>{
        if !self.closing{return Err(OwnedSchemaDecodeAdmissionFault::Sealed);}
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if self.pending.take().is_some(){return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
        if let Some(pages)=self.pages.as_mut(){
            if pages.close_take_page().is_some(){return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()}));}
            let released_bytes=pages.allocation_byte_demand();
            if released_bytes>grant.maximum_release_bytes{return Ok(RetainedCloneStep::Progress(Default::default()));}
            self.pages.take();return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,released_bytes,..Default::default()}));
        }
        self.source=None;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,..Default::default()}))
    }
    pub(crate) fn terminal_is_empty(&self)->bool{self.pages.is_none()&&self.pending.is_none()&&self.source.is_none()}
}
impl Drop for PrivateChildInputPages{
    fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"private genesis input dropped before exact pages and whole backing returned");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.pages);}}}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
