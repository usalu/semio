//! 🧮️ A selection’s established JSON string receives ordered packed coverage under original effects.

use super::{failed, fixed};
use semio_framework_pack_json::{JsonGrammarCursor, JsonMemberPolicy};
use semio_framework_value::{
    DslValue, NativeDecodeControl, RetainedAllocationEffect, RetainedCloneProgress, RetireOwned,
    RetirementDemand, ValueError, ValueRefusalKind,
};
use semio_s_artifact_raster_raster_model::{RasterSelectionSpan, RasterSelectionSpans, RasterSelectionSpansBuilder};
use std::mem::size_of;

#[derive(Default, RetireOwned)]
#[value(send_only)]
pub(super) struct Selection {
    parser: Option<JsonGrammarCursor<DslValue>>,
    parsed: Option<DslValue>,
    builder: Option<RasterSelectionSpansBuilder>,
    pending: Option<RasterSelectionSpan>,
    output: Option<RasterSelectionSpans>,
    rank: usize,
}

#[derive(Clone,Copy)]
enum Frontier { Start, Read, Pack, Row, Push, Finish, Advance, Take }

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue,"Raster selection JSON requires ordered selection span rows")
}

fn row(value: &DslValue) -> Result<RasterSelectionSpan,ValueError> {
    let DslValue::Object(fields)=value else { return Err(invalid()); };
    if fields.len()!=3 { return Err(invalid()); }
    let (mut start,mut length,mut coverage)=(None,None,None);
    for (key,value) in fields {
        let DslValue::Number(number)=value else { return Err(invalid()); };
        let number=number.as_u64().and_then(|n|u32::try_from(n).ok()).ok_or_else(invalid)?;
        match key.as_str() {
            "start"=>start=Some(number),
            "length"=>length=Some(number),
            "coverage"=>coverage=Some(number),
            _=>return Err(invalid()),
        }
    }
    Ok(RasterSelectionSpan {start:start.ok_or_else(invalid)?,length:length.ok_or_else(invalid)?,coverage:coverage.ok_or_else(invalid)?})
}

impl Selection {
    pub(super) fn ready(&self) -> bool { self.output.is_some() }

    pub(super) fn take(&mut self) -> Option<RasterSelectionSpans> { self.output.take() }

    fn rows(&self) -> Result<&[DslValue],ValueError> {
        match self.parsed.as_ref() {
            Some(DslValue::Array(rows))=>Ok(rows),
            _=>Err(invalid()),
        }
    }

    fn frontier(&self) -> Result<Frontier,ValueError> {
        let Some(builder)=self.builder.as_ref() else {
            return Ok(if self.parsed.is_some(){Frontier::Pack}else if self.parser.is_some(){Frontier::Read}else{Frontier::Start});
        };
        if builder.output_ready(){return Ok(Frontier::Take);}
        if builder.finishing(){return Ok(Frontier::Advance);}
        if self.rank==self.rows()?.len(){return Ok(Frontier::Finish);}
        if !builder.accepts_row(){return Ok(Frontier::Advance);}
        if self.pending.is_some(){return Ok(Frontier::Push);}
        Ok(Frontier::Row)
    }

    pub(super) fn demand(&self,source:&str,_body:usize)->Result<RetirementDemand,ValueError>{
        let builder=||self.builder.as_ref().ok_or_else(failed);
        match self.frontier()? {
            Frontier::Start=>Ok(fixed(size_of::<Option<JsonGrammarCursor<DslValue>>>())),
            Frontier::Read=>{
                let mut demand=self.parser.as_ref().ok_or_else(failed)?.normal_step_demands(source)
                    .map_err(|error|ValueError::literal(error.kind(),"Raster selection JSON grammar refused"))?;
                demand.copy_bytes=demand.copy_bytes.checked_add(size_of::<Option<DslValue>>()).ok_or_else(failed)?;
                Ok(demand)
            }
            Frontier::Pack=>Ok(fixed(size_of::<Option<RasterSelectionSpansBuilder>>())),
            Frontier::Row=>Ok(fixed(size_of::<Option<RasterSelectionSpan>>())),
            Frontier::Push=>builder()?.push_demands(),
            Frontier::Finish=>builder()?.finish_demands(),
            Frontier::Advance=>builder()?.demands(),
            Frontier::Take=>builder()?.take_demands(),
        }
    }

    pub(super) fn work(&self)->Result<u64,ValueError>{
        if matches!(self.frontier()?,Frontier::Advance) {
            self.builder.as_ref().ok_or_else(failed)?.advance_work_demand()
        } else {Ok(1)}
    }

    pub(super) fn step(
        &mut self,source:&str,body:usize,native:&mut NativeDecodeControl<'_>,effect:&mut RetainedAllocationEffect<'_>,
    )->Result<RetainedCloneProgress,ValueError>{
        let demand=self.demand(source,body)?;
        let frontier=self.frontier()?;
        let posted=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        match frontier {
            Frontier::Read=>{
                let parser=self.parser.as_mut().ok_or_else(failed)?;
                let mut grant=effect.grant();
                let header=size_of::<Option<DslValue>>();
                grant.maximum_copy_bytes=grant.maximum_copy_bytes.saturating_sub(header);
                let parsed=parser.step(source,1,native,&mut effect.nested(grant));
                let mut progress=parser.normal_step_progress();
                match parsed {
                    Ok(Some(value))=>{
                        self.parsed=Some(value);
                        progress.copied_bytes+=header;
                    }
                    Ok(None)=>{}
                    Err(error)=>return Err(ValueError::literal(error.kind(),"Raster selection JSON grammar refused").with_retained_progress(progress)),
                }
                Ok(progress)
            }
            Frontier::Push=>{
                let progress=self.builder.as_mut().ok_or_else(failed)?.push(&mut self.pending,effect)?.progress();
                if self.pending.is_none(){self.rank+=1;}
                Ok(progress)
            }
            Frontier::Finish=>self.builder.as_mut().ok_or_else(failed)?.finish(effect).map(|step|step.progress()),
            Frontier::Advance=>{
                let turn=self.builder.as_mut().ok_or_else(failed)?.advance(effect)?;
                turn.transpose().map(|step|step.map_or(Default::default(),|step|step.progress()))
            }
            Frontier::Take=>{
                self.output=self.builder.as_mut().ok_or_else(failed)?.take(effect)?;
                Ok(if self.output.is_some(){posted}else{Default::default()})
            }
            _=>{
                if !effect.performed(1)?{return Ok(Default::default());}
                match frontier {
                    Frontier::Start=>self.parser=Some(JsonGrammarCursor::new(JsonMemberPolicy::Reject)),
                    Frontier::Pack=>self.builder=Some(RasterSelectionSpansBuilder::new(self.rows()?.len())?),
                    Frontier::Row=>self.pending=Some(row(&self.rows()?[self.rank])?),
                    _=>unreachable!(),
                }
                Ok(posted)
            }
        }
    }
}
