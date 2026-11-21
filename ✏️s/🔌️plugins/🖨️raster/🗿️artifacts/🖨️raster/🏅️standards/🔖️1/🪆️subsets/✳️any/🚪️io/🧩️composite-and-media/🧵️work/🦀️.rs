//! 🧩️ One original composite source funds preparation, pixel work, raw handoff and private retirement.
use super::*;
use semio_framework_pixels::compositing::layers::{RasterStackResult,RasterStackRetirement};
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,OriginalRetirementFailure,OriginalRetirementPoll,RetainedAllocationEffect,RetainedWorkPoll,retained_clone::RetainedCloneClose};
use std::mem::ManuallyDrop;

#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct CompositeData{job:Option<RasterStackJob>,retired:Option<RasterStackRetirement>,output:Option<RasterStackResult>,phase:u8}
pub struct RasterCompositeWork{document:usize,preparing:ManuallyDrop<Option<RasterStackPreparationWork>>,data:ManuallyDrop<Option<CompositeData>>,retirement:RetainedCloneClose,closing:bool,failed:bool}
fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn progress_only(poll: OriginalRetirementPoll) -> RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure> {
    match poll {
        RetainedWorkPoll::Complete(turn) => RetainedWorkOutcome { result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())),work:turn.work },
        RetainedWorkPoll::Pending { work } => RetainedWorkOutcome { result:Ok(RetainedCloneStep::Progress(Default::default())),work },
    }
}
impl RasterCompositeWork{
    pub fn new(document:&RasterSnapshot)->Result<Self,ValueError>{Self::from_layers(document,&document.layers,false)}
    pub fn from_layers(document:&RasterSnapshot,layers:&[RasterLayerNode],editable:bool)->Result<Self,ValueError>{let preparing=if editable{RasterStackPreparationWork::for_edit(document,layers)}else{RasterStackPreparationWork::from_layers(document,layers)}?;Ok(Self{document:document as*const _ as usize,preparing:ManuallyDrop::new(Some(preparing)),data:ManuallyDrop::new(Some(CompositeData{job:None,retired:None,output:None,phase:0})),retirement:Default::default(),closing:false,failed:false})}
    pub fn output_ready(&self)->bool{!self.closing&&!self.failed&&self.data.as_ref().is_some_and(|data|data.phase==6&&data.output.is_some())}
    pub fn demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if self.closing||self.failed||!(1..=32768).contains(&body){return Err(refusal("raster.composite-work-inactive"))}let data=self.data.as_ref().ok_or_else(||refusal("raster.composite-work-transferred"))?;match data.phase{0=>if self.preparing.as_ref().unwrap().output_ready(){Ok(RetirementDemand{depth:1,..Default::default()})}else{self.preparing.as_ref().unwrap().demands(body)},2=>self.preparing.as_ref().unwrap().close_demands(body),3=>Ok(data.job.as_ref().unwrap().next_work_demand()),5=>data.retired.as_ref().unwrap().owner.selected_demand(body),_=>Ok(RetirementDemand{depth:1,..Default::default()})}}
    pub fn advance(&mut self,document:&RasterSnapshot,body:usize,effect:&mut RetainedAllocationEffect<'_>)
        -> RetainedWorkOutcome<RetainedCloneStep,OriginalRetirementFailure>
    {
        let idle = || RetainedWorkOutcome { result:Ok(RetainedCloneStep::Progress(Default::default())),work:0 };
        let fail = |error:ValueError| RetainedWorkOutcome { result:Err(error.into()),work:0 };
        let grant = effect.grant();
        let demand = match self.demands(body) { Ok(demand)=>demand,Err(error)=>return fail(error) };
        if !semio_framework_value::retained_clone::retained_clone_grant_admits(grant,demand) { return idle(); }
        if self.document != document as *const _ as usize {
            self.failed = true;
            return fail(refusal("raster.composite-source-revision-changed"));
        }
        let data = self.data.as_mut().unwrap();
        let turn = match data.phase {
            0 if !self.preparing.as_ref().unwrap().output_ready() => {
                let turn = self.preparing.as_mut().unwrap().advance(document,body,effect);
                RetainedWorkOutcome { result:turn.result.map(|step|RetainedCloneStep::Progress(step.progress())).map_err(Into::into), work:turn.work }
            }
            2 if !self.preparing.as_ref().unwrap().terminal_is_empty() => {
                progress_only(self.preparing.as_mut().unwrap().close_step(body,effect))
            }
            3 => {
                let turn = advance_stack_turn(data.job.as_mut().unwrap(),256,effect);
                RetainedWorkOutcome { result:turn.result.map(|(state,progress)|{
                    if state.done { data.phase=4; }
                    RetainedCloneStep::Progress(progress)
                }).map_err(Into::into),work:turn.work }
            }
            5 if !data.retired.as_ref().unwrap().terminal_is_empty() => {
                progress_only(data.retired.as_mut().unwrap().close_step(body,effect))
            }
            6 => RetainedWorkOutcome { result:Ok(RetainedCloneStep::Complete(Default::default())),work:0 },
            phase => {
                match effect.remaining_work_units() {
                    Ok(0) => return idle(),
                    Ok(_) => {}
                    Err(error) => return fail(error),
                }
                match effect.performed(1) {
                    Ok(true) => {}
                    Ok(false) => return fail(refusal("raster.composite-work-grant")),
                    Err(error) => return fail(error),
                }
                let header = RetainedCloneProgress { copied_items:1,..Default::default() };
                let result = match phase {
                    0 => { data.phase=1; Ok(RetainedCloneStep::Progress(header)) }
                    1 => {
                        let turn = self.preparing.as_mut().unwrap().take_input(grant);
                        turn.result.map(|output| {
                            if let Some((input,progress))=output {
                                data.job=Some(RasterStackJob::new(input));
                                self.preparing.as_mut().unwrap().cancel();
                                data.phase=2;
                                RetainedCloneStep::Progress(progress)
                            } else { RetainedCloneStep::Progress(Default::default()) }
                        })
                    }
                    2 => { self.preparing.take(); data.phase=3; Ok(RetainedCloneStep::Progress(header)) }
                    4 => {
                        let (retired,output)=data.job.take().unwrap().into_retirement();
                        data.retired=Some(retired);
                        data.output=output;
                        if data.output.is_some() { data.phase=5; Ok(RetainedCloneStep::Progress(header)) }
                        else { Err(refusal("raster.composite-output-unavailable")) }
                    }
                    5 => { data.retired=None; data.phase=6; Ok(RetainedCloneStep::Complete(header)) }
                    _ => Err(refusal("raster.composite-work-phase")),
                };
                RetainedWorkOutcome { result:result.map_err(Into::into),work:1 }
            }
        };
        if turn.result.is_err() { self.failed=true; }
        turn
    }
    pub fn take_result(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(RasterStackResult,RetainedCloneProgress)>>{if !self.output_ready(){return RetainedWorkOutcome{result:Err(refusal("raster.composite-output-unavailable")),work:0}}if grant.maximum_items==0||grant.maximum_depth==0{return RetainedWorkOutcome{result:Ok(None),work:0}}let data=self.data.as_mut().unwrap();let result=data.output.take().unwrap();data.phase=7;RetainedWorkOutcome{result:Ok(Some((result,RetainedCloneProgress{copied_items:1,..Default::default()}))),work:1}}
    pub fn cancel(&mut self){self.closing=true;if let Some(preparing)=self.preparing.as_mut(){preparing.cancel();}}
    pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(preparing)=self.preparing.as_ref(){return preparing.close_demands(body)}self.retirement.next_owner_demands::<CompositeData>(self.data.is_some(),&None,body)}
    pub fn close_step(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>)->OriginalRetirementPoll {
        let complete = |step| RetainedWorkPoll::Complete(RetainedWorkOutcome { result:step,work:0 });
        if !self.closing { return complete(Err(refusal("raster.composite-close-intent").into())); }
        if let Some(preparing) = self.preparing.as_mut() {
            if !preparing.terminal_is_empty() { return preparing.close_step(body,effect); }
            match effect.performed(1) {
                Ok(true) => {
                    self.preparing.take();
                    return RetainedWorkPoll::Complete(RetainedWorkOutcome { result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress { copied_items:1,..Default::default() })),work:1 });
                }
                Ok(false) => return complete(Ok(RetainedCloneStep::Progress(Default::default()))),
                Err(error) => return complete(Err(error.into())),
            }
        }
        if !self.retirement.is_empty() { return self.retirement.step_granted(body,effect); }
        let turn=self.retirement.begin_granted(&mut self.data,effect);
        RetainedWorkPoll::Complete(RetainedWorkOutcome {
            result:turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Complete(Default::default()))).map_err(Into::into),
            work:turn.work,
        })
    }
    pub fn terminal_is_empty(&self)->bool{self.preparing.is_none()&&self.data.is_none()&&self.retirement.is_empty()}
}
impl Drop for RasterCompositeWork{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"composite work retains original private roots until funded close");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.preparing);ManuallyDrop::drop(&mut self.data)}}}}
