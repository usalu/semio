//! 📘️ Verifies an original replacement against its exact complete Pack Value document.
use super::{header,size_of};
use semio_framework_dsl_record::native_encoding::FieldProjectionSource;
use semio_framework_value::{RetainedAllocationEffect,RetainedCloneProgress,RetirementDemand,ValueError,ValueRefusalKind};
use semio_os_pack::record::BorrowedPackDocumentCursor;

pub(super) struct PackValueComparison{
    cursor:BorrowedPackDocumentCursor,
    offset:usize,
}

impl PackValueComparison{
    pub(super) fn new(maximum:usize)->Self{Self{cursor:BorrowedPackDocumentCursor::with_maximum_file_len(maximum),offset:0}}

    pub(super) fn birth_demand()->RetirementDemand{
        RetirementDemand{capacity_bytes:size_of::<Self>(),copy_bytes:size_of::<Self>()+size_of::<Option<Box<Self>>>(),depth:1,..Default::default()}
    }

    pub(super) fn demands(&self)->Result<RetirementDemand,ValueError>{
        Ok(RetirementDemand{copy_bytes:self.cursor.next_copy_byte_demand()?+1+size_of::<usize>(),capacity_bytes:self.cursor.next_capacity_byte_demand()?,depth:self.cursor.next_depth_demand()?,..Default::default()})
    }

    pub(super) fn advance(&mut self,source:&dyn FieldProjectionSource,wire:&[u8],effect:&mut RetainedAllocationEffect<'_>)->Result<(bool,RetainedCloneProgress),ValueError>{
        let mut byte=[0];
        let step=self.cursor.advance(source,semio_os_pack::pack_rt::wire_value_spec(),&mut byte,effect).map_err(|failure|failure.reason)?;
        let mut progress=step.progress;
        let end=self.offset+step.written_bytes;
        if step.written_bytes!=0{progress.copied_bytes+=step.written_bytes+size_of::<usize>();}
        if wire.get(self.offset..end)!=Some(&byte[..step.written_bytes])||step.complete&&end!=wire.len(){
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"replacement input differs from its admitted canonical Pack Value").with_retained_progress(progress));
        }
        self.offset=end;
        Ok((step.complete,progress))
    }

    pub(super) fn close_demands(&self)->Result<RetirementDemand,ValueError>{
        if self.terminal_is_empty(){return Ok(RetirementDemand{release_bytes:size_of::<Self>(),..header::<Option<Box<Self>>>()})}
        self.cursor.retirement_demands()
    }

    pub(super) fn close(&mut self,effect:&mut RetainedAllocationEffect<'_>)->Result<RetainedCloneProgress,ValueError>{self.cursor.close(effect).map(|step|step.progress).map_err(|failure|failure.reason)}

    pub(super) fn terminal_is_empty(&self)->bool{self.cursor.terminal_is_empty()}
}
