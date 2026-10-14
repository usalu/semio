use super::*;
use semio_framework_os_flow::geometry::{GeometryWorkIdentity,GeometryWorkProgress,GeometryWorkFailure};
use semio_framework_value::RetirementDemand;

/// 🧭️ One original source and caller identity remain attached to its paid key and query admission.
pub(super) struct TessellationAdmission {
    pub(super) identity:GeometryWorkIdentity,
    pub(super) source:usize,
    pub(super) length:usize,
    pub(super) tolerance:u64,
    pub(super) handle:GeometryHandle,
    pub(super) claim:String,
    pub(super) job_key:TessellationJobKey,
    pub(super) copied:usize,
    pub(super) phase:u8,
    pub(super) units:usize,
    pub(super) cancelled:bool,
}
semio_framework_value::artifact_retire_struct!(TessellationAdmission {identity,source,length,tolerance,handle,claim,job_key,copied,phase,units,cancelled});

impl TessellationAdmission {
    pub(super) fn new(identity:GeometryWorkIdentity,handle:&str,tolerance:f64)->Self {
        Self {identity,source:handle.as_ptr()as usize,length:handle.len(),tolerance:tolerance.to_bits(),handle:GeometryHandle(String::new()),claim:String::new(),job_key:(String::new(),tolerance.to_bits(),identity.operation,identity.generation),copied:0,phase:0,units:1,cancelled:false}
    }
    pub(super) fn matches(&self,identity:GeometryWorkIdentity,handle:&str,tolerance:f64)->bool {
        self.identity==identity&&self.source==handle.as_ptr()as usize&&self.length==handle.len()&&self.tolerance==tolerance.to_bits()
    }
    pub(super) fn text_demand(&self,source:&str)->semio_framework_value::RetirementDemand {
        let capacity=matches!(self.phase,0|2|6);
        let bytes=if capacity{0}else{source[self.copied..].chars().next().map_or(0,char::len_utf8)};
        semio_framework_value::RetirementDemand {copy_bytes:bytes,capacity_bytes:if capacity{self.length}else{0},depth:3,..Default::default()}
    }
    pub(super) fn demand(&self,session:&Session,registry:&TessellationJobRegistry,source:&str,copy:usize)->Result<RetirementDemand,ValueError>{
        if matches!(self.phase,0|1|2|3|6|7){return Ok(self.text_demand(source));}
        let mut demand=RetirementDemand{depth:3,..Default::default()};
        if matches!(self.phase,4|5){
            let family=session.state.claims.try_lock().map_err(|_|ownership_busy())?;
            let own=family.get(&session.state.authority).ok_or_else(ownership_busy)?;
            demand.depth=family.next_contains_depth_demand(&session.state.authority)?.checked_add(own.handles.next_insert_depth_demand(&self.claim)?).ok_or_else(ownership_busy)?;
            if self.phase==4{demand.capacity_bytes=own.handles.next_insert_capacity_byte_demand(&self.claim,copy)?;}else{demand.copy_bytes=1;}
        }else if matches!(self.phase,8|9){
            demand.depth=registry.jobs.next_insert_depth_demand(&self.job_key)?.max(3);
            if self.phase==8{demand.capacity_bytes=registry.jobs.next_insert_capacity_byte_demand(&self.job_key,copy)?;}
            else{let kernel=session.kernel().try_lock().map_err(|_|ownership_busy())?;demand.depth=demand.depth.max(kernel.tessellation_source_depth_demand(self.handle.as_str())?.checked_add(2).ok_or_else(ownership_busy)?);}
        }else{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tessellation admission phase is invalid"));}
        Ok(demand)
    }
    pub(super) fn text_step(&mut self,source:&str,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,ValueError> {
        let demand=self.text_demand(source);
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Default::default());}
        let target=match self.phase {0|1=>&mut self.handle.0,2|3=>&mut self.claim,6|7=>&mut self.job_key.0,_=>return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tessellation text stage is invalid"))};
        let mut receipt=RetainedCloneProgress {copied_items:1,..Default::default()};
        if matches!(self.phase,0|2|6){
            target.try_reserve_exact(self.length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"original tessellation key backing reservation failed"))?;
            receipt.retained_capacity_bytes=target.capacity();self.phase+=1;self.copied=0;
        }else{
            let end=self.copied+source[self.copied..].chars().next().map_or(0,char::len_utf8);
            target.push_str(&source[self.copied..end]);receipt.copied_bytes=end-self.copied;self.copied=end;
            if end==source.len(){self.phase+=1;self.copied=0;}
        }
        self.units+=1;
        if !receipt.fits(grant){return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tessellation allocator exceeded admitted backing").with_retained_progress(receipt));}
        Ok(receipt)
    }
}

impl Session {
    pub(super) fn admission_demands(&self,identity:GeometryWorkIdentity,handle:&str,tolerance:f64,copy:usize)->Result<Option<RetirementDemand>,ValueError>{
        let owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;
        let registry=owner.original().ok_or_else(ownership_busy)?;
        if let Some(row)=registry.jobs.iter().find(|(key,_)|key.0==handle&&key.1==tolerance.to_bits()&&key.2==identity.operation&&key.3==identity.generation).map(|(_,row)|row){
            if row.identity!=identity||row.source!=handle.as_ptr()as usize||row.length!=handle.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original tessellation source or caller changed"));}
            return Ok(None);
        }
        match &registry.admission{
            Some(admission) if admission.matches(identity,handle,tolerance)=>admission.demand(self,registry,handle,copy).map(Some),
            Some(_)=>Err(ownership_busy()),
            None=>Ok(Some(RetirementDemand{depth:3,..Default::default()})),
        }
    }
    pub(super) fn admission_step(&self,identity:GeometryWorkIdentity,handle:&str,tolerance:f64,units:usize,grant:RetainedCloneGrant,spent:&mut usize)->Result<Option<GeometryWorkProgress>,ValueError>{
        let Some(demand)=self.admission_demands(identity,handle,tolerance,grant.maximum_copy_bytes)?else{return Ok(None);};
        if units==0||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(Some(Default::default()));}
        *spent=1;
        let mut owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;
        let registry=owner.original_mut().ok_or_else(ownership_busy)?;
        if registry.admission.is_none(){registry.admission=Some(TessellationAdmission::new(identity,handle,tolerance));return Ok(Some(GeometryWorkProgress{retained:RetainedCloneProgress{copied_items:1,..Default::default()},consumed_units:1}));}
        let admission=registry.admission.as_mut().ok_or_else(ownership_busy)?;
        if !admission.matches(identity,handle,tolerance){return Err(ownership_busy());}
        if matches!(admission.phase,0|1|2|3|6|7){
            let retained=admission.text_step(handle,grant)?;
            return Ok(Some(GeometryWorkProgress{consumed_units:usize::from(retained.copied_items!=0),retained}));
        }
        let mut retained=RetainedCloneProgress{copied_items:1,..Default::default()};
        if matches!(admission.phase,4|5){
            let mut family=self.state.claims.try_lock().map_err(|_|ownership_busy())?;
            let own=family.get_mut(&self.state.authority).ok_or_else(ownership_busy)?;
            if admission.phase==4{
                if own.handles.get(admission.claim.as_str())==Some(&true){admission.phase=6;}
                else if own.handles.contains_key(admission.claim.as_str()){admission.phase=5;}
                else if demand.capacity_bytes!=0{retained=own.handles.reserve_insert_step(&admission.claim,grant).map_err(|(error,receipt)|error.with_retained_progress(receipt))?;}
                else{admission.phase=5;}
            }else{
                if let Some(live)=own.handles.get_mut(admission.claim.as_str()){*live=true;retained.copied_bytes=1;admission.phase=6;admission.units+=1;self.claims_changed();return Ok(Some(GeometryWorkProgress{retained,consumed_units:1}));}
                let key=std::mem::take(&mut admission.claim);
                match own.handles.insert_reserved(key,true,grant){Ok((previous,receipt))=>{assert!(previous.is_none());retained=RetainedCloneProgress{copied_bytes:1,..receipt};admission.phase=6;self.claims_changed();},Err((error,key,_))=>{admission.claim=key;return Err(error);}}
            }
        }else if admission.phase==8{
            if demand.capacity_bytes!=0{retained=registry.jobs.reserve_insert_step(&admission.job_key,grant).map_err(|(error,receipt)|error.with_retained_progress(receipt))?;}else{admission.phase=9;}
        }else if admission.phase==9{
            if registry.jobs.len()>=TESSELLATION_JOB_CAPACITY{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"original tessellation job registry is full"));}
            let kernel=self.kernel().try_lock().map_err(|_|ownership_busy())?;
            let(job,validation)=kernel.admit_tessellation_job(admission.handle.as_str(),tolerance)?;
            let validation_units=usize::from(validation);
            let original=registry.admission.take().ok_or_else(ownership_busy)?;
            let row=RetainedTessellation{identity:original.identity,source:original.source,length:original.length,source_units:original.units+1,geometry:Some(original.handle),admission_claim:original.claim,validation_units,validation:validation.then(||semio_framework_3d_brep_queries::validation::BodyValidationJob::new(kernel.tessellation_body())),validated:validation_units==0,job,last_step:registry.clock,retired:false,output:None,previous_mesh:None};
            match registry.jobs.insert_reserved(original.job_key,row,grant){Ok((previous,receipt))=>{assert!(previous.is_none());retained=receipt;},Err((error,key,row))=>{registry.admission=Some(TessellationAdmission{identity:row.identity,source:row.source,length:row.length,tolerance:key.1,handle:row.geometry.unwrap(),claim:row.admission_claim,job_key:key,copied:0,phase:9,units:original.units,cancelled:original.cancelled});return Err(error);}}
        }
        if let Some(admission)=&mut registry.admission{admission.units+=1;}
        Ok(Some(GeometryWorkProgress{retained,consumed_units:1}))
    }
}

/// 🛑️ The original row selector remains attached through every funded cancellation and close event.
#[derive(Clone,Copy)]
pub(super) struct TessellationSourceCancellation {identity:GeometryWorkIdentity,source:usize,length:usize,tolerance:u64,slot:usize,done:bool}
semio_framework_value::artifact_retire_leaf!(TessellationSourceCancellation);
impl TessellationSourceCancellation {
    fn matches(&self,identity:GeometryWorkIdentity,handle:&str,tolerance:f64)->bool{self.identity==identity&&self.source==handle.as_ptr()as usize&&self.length==handle.len()&&self.tolerance==tolerance.to_bits()}
    fn row_matches(&self,key:&TessellationJobKey,row:&RetainedTessellation)->bool{self.identity==row.identity&&self.source==row.source&&self.length==row.length&&self.tolerance==key.1}
}
impl Session {
    pub fn cancel_tessellation_demands(&self,identity:GeometryWorkIdentity,handle:&str,tolerance:f64,copy:usize)->Result<RetirementDemand,ValueError>{
        let owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;
        let registry=owner.original().ok_or_else(ownership_busy)?;
        if registry.cancellation.is_some(){return Err(ownership_busy());}
        let Some(target)=&registry.source_cancellation else{
            if registry.closing.is_some()||registry.admission_closing.is_some(){return Err(ownership_busy());}
            return Ok(RetirementDemand{depth:3,..Default::default()});
        };
        if !target.matches(identity,handle,tolerance){if target.done{return Ok(RetirementDemand{depth:3,..Default::default()});}return Err(ownership_busy());}
        if target.done{return Ok(Default::default());}
        if let Some(closing)=&registry.admission_closing{return closing.selected_demand(copy);}
        if let Some(closing)=&registry.closing{return closing.selected_demand(copy);}
        if registry.admission.as_ref().is_some_and(|admission|admission.matches(identity,handle,tolerance)){return Ok(RetirementDemand{depth:3,..Default::default()});}
        Ok(RetirementDemand{depth:registry.jobs.next_slot_depth_demand(target.slot)?.max(registry.jobs.next_extract_slot_depth_demand(target.slot)?).max(3),..Default::default()})
    }
    pub fn cancel_tessellation_step(&self,identity:GeometryWorkIdentity,handle:&str,tolerance:f64,cx:&mut semio_framework_job::StepContext<'_>)->Result<(RetainedCloneStep,GeometryWorkProgress),GeometryWorkFailure>{
        Self::check_geometry_caller(identity,cx)?;
        if cx.should_yield(){return Ok((RetainedCloneStep::Progress(Default::default()),Default::default()))}
        let mut spent=0;let result=self.cancel_tessellation_original_step(identity,handle,tolerance,1,cx.retained_grant(),&mut spent);
        Self::receive_geometry_work(result.map_err(|primary|GeometryWorkFailure::new(primary,spent)),cx)
    }
    fn cancel_tessellation_original_step(&self,identity:GeometryWorkIdentity,handle:&str,tolerance:f64,units:usize,grant:RetainedCloneGrant,spent:&mut usize)->Result<(RetainedCloneStep,GeometryWorkProgress),ValueError>{
        let demand=self.cancel_tessellation_demands(identity,handle,tolerance,grant.maximum_copy_bytes)?;
        let empty=GeometryWorkProgress::default();
        if units==0||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok((RetainedCloneStep::Progress(Default::default()),empty));}
        *spent=1;
        let mut owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;
        let registry=owner.original_mut().ok_or_else(ownership_busy)?;
        if registry.source_cancellation.as_ref().is_none_or(|target|!target.matches(identity,handle,tolerance)){
            registry.source_cancellation=Some(TessellationSourceCancellation{identity,source:handle.as_ptr()as usize,length:handle.len(),tolerance:tolerance.to_bits(),slot:0,done:false});
            let retained=RetainedCloneProgress{copied_items:1,..Default::default()};return Ok((RetainedCloneStep::Progress(retained),GeometryWorkProgress{retained,consumed_units:1}));
        }
        if registry.source_cancellation.as_ref().is_some_and(|target|target.done){*spent=0;return Ok((RetainedCloneStep::Complete(Default::default()),empty));}
        if let Some(closing)=&mut registry.admission_closing{
            let step=closing.step(RetainedCloneGrant{maximum_items:grant.maximum_items.min(units).min(1),..grant})?;let retained=step.progress();if closing.terminal_is_empty(){registry.admission_closing=None;}
            return Ok((RetainedCloneStep::Progress(retained),GeometryWorkProgress{retained,consumed_units:*spent}));
        }
        if let Some(closing)=&mut registry.closing{
            let step=closing.step(RetainedCloneGrant{maximum_items:grant.maximum_items.min(units).min(1),..grant})?;let retained=step.progress();if closing.terminal_is_empty(){registry.closing=None;}
            return Ok((RetainedCloneStep::Progress(retained),GeometryWorkProgress{retained,consumed_units:*spent}));
        }
        let retained=RetainedCloneProgress{copied_items:1,..Default::default()};
        if registry.admission.as_ref().is_some_and(|admission|admission.matches(identity,handle,tolerance)){
            let mut original=registry.admission.take().ok_or_else(ownership_busy)?;original.cancelled=true;
            match ControlledRetirement::new(original){Ok(closing)=>registry.admission_closing=Some(closing),Err((error,original))=>{registry.admission=Some(original);return Err(error);}}
            return Ok((RetainedCloneStep::Progress(retained),GeometryWorkProgress{retained,consumed_units:1}));
        }
        let target=registry.source_cancellation.as_mut().ok_or_else(ownership_busy)?;
        if target.slot>=registry.jobs.slot_count(){target.done=true;return Ok((RetainedCloneStep::Complete(retained),GeometryWorkProgress{retained,consumed_units:1}));}
        let slot=target.slot;target.slot+=1;
        if let Some((key,row))=registry.jobs.slot_entry_mut(slot){if target.row_matches(key,row){row.job.cancel();row.retired=true;}}
        if let Some(original)=registry.jobs.extract_slot_if(slot,|key,row|!target.row_matches(key,row)){
            match ControlledRetirement::new(original){Ok(closing)=>registry.closing=Some(closing),Err((error,key_row))=>{let(key,row)=key_row;match registry.jobs.insert_reserved(key,row,grant){Ok((previous,_))=>assert!(previous.is_none()),Err((_error,_key,_row))=>panic!("original cancelled row lost its admitted slot")};return Err(error);}}
        }
        Ok((RetainedCloneStep::Progress(retained),GeometryWorkProgress{retained,consumed_units:1}))
    }
}

impl Session {
    fn admission_progress(&self,identity:GeometryWorkIdentity,handle:&str,tolerance:f64)->Result<Option<(usize,usize,&'static str)>,ValueError>{
        let owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;let registry=owner.original().ok_or_else(ownership_busy)?;
        if let Some(admission)=&registry.admission{if !admission.matches(identity,handle,tolerance){return Err(ownership_busy());}let phase=match admission.phase{0|1=>"admitting-handle",2|3=>"admitting-claim",4|5=>"admitting-claim-index",6|7=>"admitting-job-key",_=>"admitting-job-index"};return Ok(Some((admission.units,admission.units+1,phase)));}
        let row=registry.jobs.iter().find(|(key,row)|key.0==handle&&key.1==tolerance.to_bits()&&row.identity==identity&&row.source==handle.as_ptr()as usize&&row.length==handle.len()).map(|(_,row)|row);Ok(row.map(|row|(row.source_units,row.source_units+1,"admitted-source")))
    }
    fn source_tessellation_cancelled(&self,identity:GeometryWorkIdentity,handle:&str,tolerance:f64)->Result<bool,ValueError>{let owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;let registry=owner.original().ok_or_else(ownership_busy)?;Ok(registry.source_cancellation.as_ref().is_some_and(|target|target.matches(identity,handle,tolerance)))}
    pub fn tessellation_demands(&self,identity:semio_framework_os_flow::geometry::GeometryWorkIdentity,handle:&str,tolerance:f64,copy:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{let authority=self;if self.source_tessellation_cancelled(identity,handle,tolerance)?{return Ok(Default::default());}if let Some(demand)=authority.admission_demands(identity,handle,tolerance,copy)?{return Ok(demand);}let kernel=self.kernel().try_lock().map_err(|_|ownership_busy())?;let owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;let registry=owner.original().ok_or_else(ownership_busy)?;
        if let Some(validation)=registry.jobs.iter().find(|(key,row)|key.0==handle&&key.1==tolerance.to_bits()&&row.identity==identity&&row.source==handle.as_ptr()as usize&&row.length==handle.len()).and_then(|(_,row)|row.validation.as_ref()){return validation.funded_demands(kernel.tessellation_body());}
        let bytes=authority.cached_mesh_payload_bytes(handle,tolerance);Ok(semio_framework_value::RetirementDemand{copy_bytes:bytes,capacity_bytes:bytes,depth:1,..Default::default()})}
    fn tessellate_admitted_step(&self,identity:GeometryWorkIdentity,handle:&str,tolerance:f64,units:usize,grant:RetainedCloneGrant,spent:&mut usize)->Result<(TessellationStepOutcome,GeometryWorkProgress),ValueError>{
        let kernel=self.kernel().try_lock().map_err(|_|ownership_busy())?;let mut owner=self.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;let registry=owner.original_mut().ok_or_else(ownership_busy)?;
        let slot=(0..registry.jobs.slot_count()).find(|slot|registry.jobs.slot_entry(*slot).is_some_and(|(key,row)|key.0==handle&&key.1==tolerance.to_bits()&&row.identity==identity&&row.source==handle.as_ptr()as usize&&row.length==handle.len())).ok_or_else(ownership_busy)?;
        let clock=registry.clock;let(key,row)=registry.jobs.slot_entry_mut(slot).ok_or_else(ownership_busy)?;
        if row.retired{return Ok((TessellationStepOutcome::Cancelled,Default::default()));}
        if let Some(validation)=row.validation.as_mut(){
            let demand=validation.funded_demands(kernel.tessellation_body())?;
            let admitted=units!=0&&grant.maximum_items!=0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth;
            *spent=usize::from(admitted);
            let(retained,consumed_units)=validation.step_funded(kernel.tessellation_body(),units,grant)?;
            let progress=validation.progress();if consumed_units!=0{row.last_step=clock;row.validation_units=progress.units_done;}
            return Ok((TessellationStepOutcome::Working{units_done:row.source_units.saturating_add(progress.units_done),units_total:row.source_units.saturating_add(progress.units_total),faces_done:0,faces_total:0,phase:"validating"},GeometryWorkProgress{retained,consumed_units}));
        }
        row.last_step=clock;
        let outcome=self.step_tessellation_row(&kernel,key,row,None,units);if !matches!(outcome,TessellationStepOutcome::Working{..}){registry.retire_slot=0;}Ok((outcome,GeometryWorkProgress::default()))
    }
    fn check_geometry_caller(identity:GeometryWorkIdentity,cx:&semio_framework_job::StepContext<'_>)->Result<(),GeometryWorkFailure>{
        if identity.operation!=cx.operation().0||identity.generation!=cx.generation().0{return Err(GeometryWorkFailure::new(ValueError::literal(ValueRefusalKind::InvariantViolated,"original geometry turn changed its caller identity"),0))}Ok(())
    }
    fn receive_geometry_work<T>(result:Result<(T,GeometryWorkProgress),GeometryWorkFailure>,cx:&mut semio_framework_job::StepContext<'_>)->Result<(T,GeometryWorkProgress),GeometryWorkFailure>{
        let work=match &result{Ok((_,work))=>*work,Err(failure)=>failure.work};let fuel=cx.fuel_remaining();cx.consume_fuel(work.consumed_units as u64);let receipt_refusal=cx.consume_retained(work.retained).err();
        match result{Err(mut failure)=>{failure.receipt_refusal=receipt_refusal;Err(failure)},Ok(original)=>{
            if work.consumed_units as u64>fuel{return Err(GeometryWorkFailure{primary:ValueError::literal(ValueRefusalKind::WorkLimit,"original geometry event exceeds initiating fuel"),work,receipt_refusal})}
            if let Some(primary)=receipt_refusal{return Err(GeometryWorkFailure{primary,work,receipt_refusal:None})}Ok(original)
        }}
    }
    pub fn tessellate_step(&self,identity:GeometryWorkIdentity,handle:&str,tolerance:f64,cx:&mut semio_framework_job::StepContext<'_>)->Result<(semio_framework_os_flow::geometry::GeometryStep,GeometryWorkProgress),GeometryWorkFailure>{
        Self::check_geometry_caller(identity,cx)?;
        if cx.is_cancelled(){return Ok((semio_framework_os_flow::geometry::GeometryStep::Cancelled,Default::default()))}
        if cx.should_yield(){return Ok((semio_framework_os_flow::geometry::GeometryStep::Working{units_done:0,units_total:0,phase:"awaiting-caller"},Default::default()))}
        let mut spent=0;let result=self.tessellate_original_step(identity,handle,tolerance,1,cx.retained_grant(),&mut spent);
        Self::receive_geometry_work(result.map_err(|primary|GeometryWorkFailure::new(primary,spent)),cx)
    }
    fn tessellate_original_step(&self,identity:GeometryWorkIdentity,handle:&str,tolerance:f64,units:usize,grant:RetainedCloneGrant,spent:&mut usize)->Result<(semio_framework_os_flow::geometry::GeometryStep,GeometryWorkProgress),ValueError> {
        use semio_framework_os_flow::geometry::{GeometryStep,GeometryWorkProgress};
        let awaiting=||(GeometryStep::Working {units_done:0,units_total:0,phase:"awaiting-grant"},GeometryWorkProgress::default());
        if self.is_closed(){return Err(ownership_busy());}let authority=self;
        if self.source_tessellation_cancelled(identity,handle,tolerance)?{return Ok((GeometryStep::Cancelled,Default::default()));}
        if let Some(work)=authority.admission_step(identity,handle,tolerance,units,grant,spent)?{let progress=self.admission_progress(identity,handle,tolerance).map_err(|primary|primary.with_retained_progress(work.retained))?;let (units_done,units_total,phase)=progress.unwrap_or((0,0,if work==GeometryWorkProgress::default(){"awaiting-grant"}else{"admitted-source"}));return Ok((GeometryStep::Working{units_done,units_total,phase},work));}
        if units==0||grant.maximum_items==0||grant.maximum_depth==0 {return Ok(awaiting());}
        let fits=|bytes:usize|bytes<=grant.maximum_copy_bytes&&bytes<=grant.maximum_capacity_bytes;
        if !fits(authority.cached_mesh_payload_bytes(handle,tolerance)) {return Ok(awaiting());}
        let before=authority.tessellation_work_units(identity,handle,tolerance);let mut consumed_units=0;
        let(outcome,validation_work)=authority.tessellate_admitted_step(identity,handle,tolerance,units,grant,spent)?;
        if let TessellationStepOutcome::Working{units_done,units_total,phase:"validating",..}=&outcome{return Ok((GeometryStep::Working{units_done:*units_done,units_total:*units_total,phase:"validating"},validation_work));}
        let step=match outcome {
            TessellationStepOutcome::Working {units_done,units_total,phase,..}=>{consumed_units=units_done.saturating_sub(before);GeometryStep::Working {units_done,units_total,phase}},
            TessellationStepOutcome::Ready {mesh,units_total,..}=>{
                let bytes=mesh_payload_bytes(&mesh);
                if !fits(bytes) {return Ok(awaiting());}
                return Ok((GeometryStep::Ready(mesh),GeometryWorkProgress{retained:RetainedCloneProgress {copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,released_bytes:0},consumed_units:units_total.saturating_sub(before)}));
            }
            TessellationStepOutcome::Cancelled=>GeometryStep::Cancelled,
            TessellationStepOutcome::Failed {message}=>GeometryStep::Failed(message),
            TessellationStepOutcome::Invalid {issues}=>GeometryStep::Failed(issues.into_iter().map(|issue|issue.message).collect::<Vec<_>>().join("; ")),
        };
        Ok((step,GeometryWorkProgress{retained:if consumed_units==0{Default::default()}else{RetainedCloneProgress {copied_items:1,..Default::default()}},consumed_units}))
    }
}
