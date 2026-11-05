//! 📥️ Flat DSL coverage rows feed the same granted packed builder as original native arrays.
use super::super::*;
use semio_framework_dsl_record::{DslOwnedReceivingField,OwnedFieldReceiving,FieldValue,receiving::owned::{OwnedSlot,OwnedReceivingSlot,funds,metadata,payload,receipt,nested,child_grant,normal,physical,move_into,close_source,post_child_close,invariant,invalid}};
use semio_framework_value::{NativeDecodeControl,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome};
use semio_framework_value_native_owned::OriginalReceivingPrimary;
use std::mem::ManuallyDrop;

pub struct RasterCoverageFieldReceiving{source:OwnedSlot<FieldValue>,current:OwnedSlot<FieldValue>,row:OwnedSlot<RasterSelectionSpan>,output:OwnedSlot<RasterSelectionSpans>,child:OwnedReceivingSlot<RasterSelectionSpan>,builder:ManuallyDrop<Option<RasterSelectionSpansBuilder>>,index:usize,length:usize,phase:u8,closing:bool,failed:bool}
impl Default for RasterCoverageFieldReceiving{fn default()->Self{Self{source:Default::default(),current:Default::default(),row:Default::default(),output:Default::default(),child:OwnedReceivingSlot::new(),builder:ManuallyDrop::new(None),index:0,length:0,phase:0,closing:false,failed:false}}}
impl DslOwnedReceivingField for RasterSelectionSpans{type Cursor=RasterCoverageFieldReceiving;fn receiving_cursor()->Self::Cursor{Self::Cursor::default()}}
impl RasterCoverageFieldReceiving{
    fn builder(&self)->Result<&RasterSelectionSpansBuilder,ValueError>{self.builder.as_ref().ok_or_else(invariant)}
    fn selected(&mut self,input:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{
        if self.closing||self.failed{return RetainedWorkOutcome{result:Err(invariant().into()),work:0};}
        if self.phase==11{return RetainedWorkOutcome{result:Ok(Default::default()),work:0};}
        let demand=match self.demands(input,body){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error.into()),work:0}};
        if !funds(grant,demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0};}
        if let Err(error)=native.checkpoint().and_then(|_|semio_framework_value_native_owned::owned::receiving_capacity(native,demand.capacity_bytes)){return RetainedWorkOutcome{result:Err(error.into()),work:0};}
        match self.phase{
            2 if !self.child.is_ready()=>return self.child.step(&mut self.current.value,native,body,child_grant(grant)),
            3 if self.row.value.is_none()=>return normal(self.child.take_into(&mut self.row.value,child_grant(grant))),
            4 if !self.child.terminal_is_empty()=>return post_child_close(self.child.close_step(child_grant(grant)),native,grant),
            5=>{let turn=self.builder.as_mut().unwrap().push(&mut self.row.value,grant);let turn=physical(native.receive_retained_work(grant,RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}));if self.row.value.is_none(){self.index+=1;self.phase=if self.index==self.length{7}else if self.builder().unwrap().accepts_row(){1}else{6};}return turn;},
            6 if !self.builder().unwrap().output_ready()&&!self.builder().unwrap().accepts_row()=>{let turn=self.builder.as_mut().unwrap().advance(grant);return physical(native.receive_retained_work(grant,turn));},
            7=>{let turn=self.builder.as_mut().unwrap().finish(grant);let turn=physical(native.receive_retained_work(grant,RetainedWorkOutcome{result:turn.result.map(RetainedCloneStep::Progress),work:turn.work}));if turn.result.is_ok(){self.phase=6;}return turn;},
            8=>{let turn=self.builder.as_mut().unwrap().take(grant);let result=turn.result.map(|output|if let Some((output,p))=output{*self.output.value=Some(output);self.builder.as_mut().unwrap().begin_close();self.phase=9;p}else{Default::default()});return RetainedWorkOutcome{result:result.map_err(Into::into),work:turn.work};},
            9 if !self.builder().unwrap().terminal_is_empty()=>{let turn=self.builder.as_mut().unwrap().close_step(grant);return physical(native.receive_retained_work(grant,turn));},
            10 if !self.source.terminal_is_empty()=>return close_source(&mut self.source,native,grant),
            _=>{},
        }
        let result=(||{match self.phase{
            0=>{let length=match input.as_ref(){Some(FieldValue::List(rows)|FieldValue::Tuple(rows))=>rows.len(),_=>return Err(invalid())};let builder=RasterSelectionSpansBuilder::new(length)?;self.length=length;*self.source.value=input.take();*self.builder=Some(builder);self.phase=if length==0{7}else{1};},
            1=>{let Some(FieldValue::List(rows)|FieldValue::Tuple(rows))=self.source.value.as_mut()else{return Err(invariant())};*self.current.value=Some(std::mem::replace(rows.get_mut(self.index).ok_or_else(invariant)?,FieldValue::Absent));self.phase=2;},
            2=>self.phase=3,3=>self.phase=4,
            4=>{self.child=OwnedReceivingSlot::new();self.phase=5;},
            6=>self.phase=if self.builder()?.output_ready(){8}else{1},
            9=>{drop(self.builder.take());self.phase=10;},10=>self.phase=11,
            _=>return Err(invariant()),
        }Ok(receipt(demand))})();RetainedWorkOutcome{result:result.map_err(|error|OriginalReceivingPrimary::from(error.with_retained_progress(receipt(metadata::<()>())))),work:1}
    }
}
impl OwnedFieldReceiving<RasterSelectionSpans> for RasterCoverageFieldReceiving{
    fn demands(&self,_source:&Option<FieldValue>,body:usize)->Result<RetirementDemand,ValueError>{
        if self.closing||self.failed{return Err(invariant());}
        match self.phase{
            0=>Ok(payload::<RasterSelectionSpansBuilder>()),1=>Ok(payload::<FieldValue>()),
            2 if !self.child.is_ready()=>nested(self.child.demands(&self.current.value,body)?),
            3 if self.row.value.is_none()=>nested(nested(payload::<RasterSelectionSpan>())?),
            4 if !self.child.terminal_is_empty()=>nested(self.child.close_demands(body)?),
            4=>Ok(payload::<OwnedReceivingSlot<RasterSelectionSpan>>()),
            5=>self.builder()?.push_demands(),
            6 if !self.builder()?.output_ready()&&!self.builder()?.accepts_row()=>self.builder()?.demands(),
            7=>self.builder()?.finish_demands(),8=>self.builder()?.take_demands(),
            9 if !self.builder()?.terminal_is_empty()=>self.builder()?.close_demands(body),
            9=>Ok(payload::<RasterSelectionSpansBuilder>()),
            10 if !self.source.terminal_is_empty()=>self.source.close_demands(body),
            11=>Ok(Default::default()),_=>Ok(metadata::<()>()),
        }
    }
    fn step(&mut self,source:&mut Option<FieldValue>,native:&mut NativeDecodeControl<'_>,body:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress,OriginalReceivingPrimary>{let turn=self.selected(source,native,body,grant);if turn.result.is_err()&&turn.work>0{self.failed=true;}turn}
    fn is_ready(&self)->bool{!self.closing&&!self.failed&&self.phase==11&&self.output.value.is_some()}
    fn take_into(&mut self,output:&mut Option<RasterSelectionSpans>,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{if !self.is_ready(){return RetainedWorkOutcome{result:Err(invariant()),work:0};}move_into(&mut self.output.value,output,grant)}
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if self.terminal_is_empty(){return Ok(Default::default());}if !self.closing{return Ok(metadata::<()>());}
        if !self.child.terminal_is_empty(){return nested(self.child.close_demands(body)?);}
        if !self.current.terminal_is_empty(){return self.current.close_demands(body);}
        if !self.row.terminal_is_empty(){return self.row.close_demands(body);}
        if let Some(builder)=self.builder.as_ref(){return if builder.terminal_is_empty(){Ok(payload::<RasterSelectionSpansBuilder>())}else{builder.close_demands(body)};}
        if !self.output.terminal_is_empty(){return self.output.close_demands(body);}self.source.close_demands(body)
    }
    fn close_step(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<RetainedCloneProgress>{
        if self.terminal_is_empty(){return RetainedWorkOutcome{result:Ok(Default::default()),work:0};}
        let demand=match self.close_demands(grant.maximum_copy_bytes){Ok(d)=>d,Err(error)=>return RetainedWorkOutcome{result:Err(error),work:0}};if !funds(grant,demand){return RetainedWorkOutcome{result:Ok(Default::default()),work:0};}
        if !self.closing{self.closing=true;if let Some(builder)=self.builder.as_mut(){builder.begin_close();}return RetainedWorkOutcome{result:Ok(receipt(demand)),work:1};}
        if !self.child.terminal_is_empty(){return self.child.close_step(child_grant(grant));}
        if !self.current.terminal_is_empty(){return self.current.close_step(grant);}
        if !self.row.terminal_is_empty(){return self.row.close_step(grant);}
        if let Some(builder)=self.builder.as_mut(){if !builder.terminal_is_empty(){return physical(builder.close_step(grant));}drop(self.builder.take());return RetainedWorkOutcome{result:Ok(receipt(demand)),work:1};}
        if !self.output.terminal_is_empty(){return self.output.close_step(grant);}self.source.close_step(grant)
    }
    fn terminal_is_empty(&self)->bool{self.child.terminal_is_empty()&&self.current.terminal_is_empty()&&self.row.terminal_is_empty()&&self.builder.is_none()&&self.output.terminal_is_empty()&&self.source.terminal_is_empty()}
}
impl Drop for RasterCoverageFieldReceiving{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"packed coverage field receiving retains original rows until funded closure");}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
