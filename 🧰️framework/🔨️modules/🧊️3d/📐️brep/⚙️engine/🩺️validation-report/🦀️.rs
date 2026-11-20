//! 🩺️ Funded validation and JSON diagnostics for one original native shape.

use super::{Brep,PreviewReach};
use semio_framework_value::{RetainedAllocationEffect,ValueError,ValueRefusalKind};
use semio_framework_3d_brep_validation::{BodyValidationJob,ValidationReportCursor};
use semio_framework_value::{RetainedCloneProgress as Progress,RetirementDemand};
use semio_framework_value::retained_clone::retained_clone_grant_admits;

const MAXIMUM_BYTES:usize=1<<20;

#[derive(Clone,Copy)]
enum ReportStage {Scope,Validate,Measure,Reserve,Write,Done}
semio_framework_value::artifact_retire_leaf!(ReportStage);

/// 📋️ Retains the selected shape walk, physical validation, and owned diagnostic text.
pub struct NativeValidationReport {
    source:usize,
    length:usize,
    scope:PreviewReach,
    validation:BodyValidationJob,
    stage:ReportStage,
    cursor:ValidationReportCursor,
    capacity:usize,
    text:String,
}
semio_framework_value::artifact_retire_struct!(NativeValidationReport {source,length,scope,validation,stage,cursor,capacity,text});

impl NativeValidationReport {
    /// 🧭️ Admits validation of the borrowed original handle and its exact topology.
    pub fn new(kernel:&Brep,handle:&str)->Result<Self,ValueError>{
        let certified=kernel.original_validation_is_certified(handle)?;
        Ok(Self{source:handle.as_ptr()as usize,length:handle.len(),scope:PreviewReach::new(kernel.preview_validation_revision()),
            validation:BodyValidationJob::for_preview_gate(kernel.tessellation_body()),stage:if certified{ReportStage::Measure}else{ReportStage::Scope},
            cursor:Default::default(),capacity:0,text:String::new()})
    }
    pub fn phase(&self)->&'static str{
        match self.stage{
            ReportStage::Scope=>"validating-scope",ReportStage::Validate=>"validating",ReportStage::Measure|ReportStage::Reserve=>"measuring-validation-report",
            ReportStage::Write=>"writing-validation-report",ReportStage::Done=>"validation-report-ready",
        }
    }

    fn check(&self,kernel:&Brep,handle:&str)->Result<(),ValueError>{
        self.scope.check(kernel)?;
        if self.source!=handle.as_ptr()as usize||self.length!=handle.len(){return Err(ValueError::invariant("validation report source changed"));}
        Ok(())
    }

    /// 🪜️ Quotes the next validation report operation against the same source.
    pub fn demand(&self,kernel:&Brep,handle:&str,copy:usize)->Result<RetirementDemand,ValueError>{
        self.check(kernel,handle)?;
        let base=RetirementDemand{depth:1,..Default::default()};
        match self.stage{
            ReportStage::Scope=>self.scope.demand(kernel,handle,copy),
            ReportStage::Validate=>{
                let mut need=self.validation.funded_demands_for(kernel.tessellation_body(),Some(self.scope.scope()))?;
                need.depth=need.depth.max(self.scope.depth());
                Ok(need)
            }
            ReportStage::Measure=>Ok(RetirementDemand{depth:self.cursor.depth(&self.validation)?,..base}),
            ReportStage::Reserve=>Ok(RetirementDemand{capacity_bytes:self.capacity,..base}),
            ReportStage::Write=>Ok(RetirementDemand{
                copy_bytes:self.cursor.scalar(&self.validation)?.map_or(0,|value|value.width()),
                depth:self.cursor.depth(&self.validation)?,..base
            }),
            ReportStage::Done=>Ok(Default::default()),
        }
    }

    /// 📝️ Advances one funded validation or report operation against the same source.
    pub fn step(&mut self,kernel:&Brep,handle:&str,effect:&mut RetainedAllocationEffect<'_>)->Result<(Option<String>,Progress),ValueError>{
        let grant=effect.grant();
        let demand=self.demand(kernel,handle,grant.maximum_copy_bytes)?;
        if matches!(self.stage,ReportStage::Done)||grant.maximum_items==0||!retained_clone_grant_admits(grant,demand){
            return Ok((None,Default::default()));
        }
        match self.stage{
            ReportStage::Scope=>{
                let progress=self.scope.step(kernel,handle,effect)?;
                if self.scope.done(){self.stage=ReportStage::Validate;}
                return Ok((None,progress));
            }
            ReportStage::Validate=>{
                let (progress,_)=self.validation.step_funded_for(kernel.tessellation_body(),Some(self.scope.scope()),1,effect)?;
                if self.validation.is_complete(){self.stage=ReportStage::Measure;}
                return Ok((None,progress));
            }
            _=>{}
        }
        if !effect.performed(1)?{return Ok((None,Default::default()));}
        let mut receipt=Progress{copied_items:1,..Default::default()};
        if matches!(self.stage,ReportStage::Reserve){
            self.text.try_reserve_exact(self.capacity).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"validation report allocation failed"))?;
            receipt.retained_capacity_bytes=self.text.capacity();
            if !receipt.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"validation report exceeded its grant").with_retained_progress(receipt));}
            self.stage=ReportStage::Write;
            return Ok((None,receipt));
        }
        if let Some(value)=self.cursor.scalar(&self.validation)?{
            self.cursor.consume(&value);
            if matches!(self.stage,ReportStage::Measure){
                self.capacity+=value.width();
                if self.capacity>MAXIMUM_BYTES{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"validation diagnostics exceed the report limit"));}
            }else{
                self.text.push_str(std::str::from_utf8(value.bytes()).expect("JSON scalar is UTF-8"));
                receipt.copied_bytes=value.width();
            }
        }else{
            if self.cursor.next(self.validation.issue_count()){
                if matches!(self.stage,ReportStage::Measure){
                    self.cursor=Default::default();
                    self.stage=ReportStage::Reserve;
                }else{
                    self.stage=ReportStage::Done;
                    return Ok((Some(std::mem::take(&mut self.text)),receipt));
                }
            }
        }
        Ok((None,receipt))
    }
}

