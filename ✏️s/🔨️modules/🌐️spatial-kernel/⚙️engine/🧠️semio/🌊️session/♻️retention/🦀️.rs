use super::*;
use semio_framework_value::RetirementDemand;
use semio_framework_3d::brep::engine::BrepRetentionJob;

/// 🧹️ Owns the original incoming keys and removed session rows through independently funded retention.
pub(super) struct SessionRetention {request:Option<ControlledRetirement<Vec<String>>>,body:BrepRetentionJob,cache:Option<ControlledRetirement<((String,u64),CachedMesh)>>,job:Option<ControlledRetirement<((String,u64),RetainedTessellation)>>,phase:usize,slot:usize,family:usize,claim:usize,found:bool,clock:u64,cancelled:bool,receipt:RetainedCloneProgress}
impl SessionRetention {
    pub(super) fn new(session:&Session,handles:Vec<String>)->Result<Self,(ValueError,Vec<String>)>{
        if session.is_closed(){return Err((ownership_busy(),handles));}
        let claims=match session.state.claims.try_lock(){Ok(owner)=>owner,Err(_)=>return Err((ownership_busy(),handles))};
        if claims.get(&session.state.authority).is_none(){return Err((ownership_busy(),handles));}
        let kernel=match session.kernel().try_lock(){Ok(owner)=>owner,Err(_)=>return Err((ownership_busy(),handles))};
        Ok(Self {request:Some(ControlledRetirement::new(handles)?),body:BrepRetentionJob::new(&kernel),cache:None,job:None,phase:0,slot:0,family:0,claim:0,found:false,clock:session.state.next_authority.load(std::sync::atomic::Ordering::Acquire),cancelled:false,receipt:Default::default()})
    }
    pub(super) fn terminal_is_empty(&self)->bool{self.request.is_none()&&self.body.terminal_is_empty()&&self.cache.is_none()&&self.job.is_none()&&(self.cancelled||self.phase==6)}
    fn owner_demand<T:semio_framework_value::retirement::RetireOwned>(owner:&ControlledRetirement<T>,copy:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand {copy_bytes:owner.next_copy_byte_demand()?,capacity_bytes:owner.next_capacity_byte_demand(copy)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?})}
    pub(super) fn demand(&self,session:&Session,copy:usize)->Result<RetirementDemand,ValueError>{
        if self.terminal_is_empty(){return Ok(Default::default());}
        if let Some(owner)=&self.cache{return Self::owner_demand(owner,copy);}
        if let Some(owner)=&self.job{return Self::owner_demand(owner,copy);}
        if self.cancelled{if let Some(owner)=&self.request{return Self::owner_demand(owner,copy);}let kernel=session.kernel().try_lock().map_err(|_|ownership_busy())?;return self.body_demand(&kernel,copy);}
        if self.phase==2{return self.request.as_ref().map_or(Ok(RetirementDemand {depth:1,..Default::default()}),|owner|Self::owner_demand(owner,copy));}
        if self.phase>=3&&self.clock!=session.state.next_authority.load(std::sync::atomic::Ordering::Acquire){return Ok(RetirementDemand {depth:1,..Default::default()});}
        if self.phase==1{
            let source=self.request.as_ref().and_then(ControlledRetirement::original).ok_or_else(ownership_busy)?;
            let Some(key)=source.get(self.slot)else{return Ok(RetirementDemand {depth:1,..Default::default()});};
            let family=session.state.claims.try_lock().map_err(|_|ownership_busy())?;let own=family.get(&session.state.authority).ok_or_else(ownership_busy)?;
            return Ok(RetirementDemand {copy_bytes:own.handles.next_insert_copy_byte_demand(key)?,capacity_bytes:if own.handles.contains_key(key){0}else{own.handles.next_insert_capacity_byte_demand(key,copy)?},release_bytes:0,depth:family.next_insert_depth_demand(&session.state.authority)?.checked_add(own.handles.next_insert_depth_demand(key)?).ok_or_else(ownership_busy)?});
        }
        if self.phase==3{
            let kernel=session.kernel().try_lock().map_err(|_|ownership_busy())?;
            if self.body.candidate_handle(&kernel).is_some(){let family=session.state.claims.try_lock().map_err(|_|ownership_busy())?;let depth=family.next_extract_slot_depth_demand(self.family)?.max(1);let child=family.slot_entry(self.family).map_or(Ok(0),|(_,claims)|claims.handles.next_extract_slot_depth_demand(self.claim))?;return Ok(RetirementDemand {depth:depth.checked_add(child).ok_or_else(ownership_busy)?,..Default::default()});}
            return self.body_demand(&kernel,copy);
        }
        let depth=if self.phase==0{let family=session.state.claims.try_lock().map_err(|_|ownership_busy())?;let own=family.get(&session.state.authority).ok_or_else(ownership_busy)?;family.next_insert_depth_demand(&session.state.authority)?.checked_add(own.handles.next_extract_slot_depth_demand(self.slot)?.max(1)).ok_or_else(ownership_busy)?}else if self.phase==4{session.mesh_cache().try_lock().map_err(|_|ownership_busy())?.next_extract_slot_depth_demand(self.slot)?.max(1)}else if self.phase==5{session.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?.original().ok_or_else(ownership_busy)?.jobs.next_extract_slot_depth_demand(self.slot)?.max(1)}else{1};Ok(RetirementDemand {depth,..Default::default()})
    }
    fn body_demand(&self,kernel:&Brep,copy:usize)->Result<RetirementDemand,ValueError>{Ok(RetirementDemand {copy_bytes:self.body.next_copy_byte_demand(kernel)?,capacity_bytes:self.body.next_capacity_byte_demand(kernel,copy)?,release_bytes:self.body.next_release_byte_demand(kernel)?,depth:self.body.next_depth_demand(kernel)?})}
    pub(super) fn normal_step_progress(&self)->RetainedCloneProgress{self.receipt}
    fn advance()->RetainedCloneStep{ownership_advance()}
    pub(super) fn step(&mut self,session:&Session,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.receipt=Default::default();let result=self.step_retained(session,grant);if let Ok(step)=&result{self.receipt=step.progress();}result}
    fn step_retained(&mut self,session:&Session,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(Default::default()));}
        if let Some(owner)=&mut self.cache{let result=owner.step(grant);self.receipt=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.cache=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if let Some(owner)=&mut self.job{let result=owner.step(grant);self.receipt=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.job=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if self.cancelled{
            if let Some(owner)=&mut self.request{let result=owner.step(grant);self.receipt=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.request=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
            let mut kernel=session.kernel().try_lock().map_err(|_|ownership_busy())?;let result=self.body.step(&mut kernel,grant);self.receipt=self.body.normal_step_progress();return result;
        }
        if self.phase==2{if let Some(owner)=&mut self.request{let result=owner.step(grant);self.receipt=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.request=None;self.phase=3;}return Ok(RetainedCloneStep::Progress(step.progress()));}}
        if self.phase==3{
            let family=session.state.claims.try_lock().map_err(|_|ownership_busy())?;let mut kernel=session.kernel().try_lock().map_err(|_|ownership_busy())?;
            if self.clock==session.state.next_authority.load(std::sync::atomic::Ordering::Acquire)&&self.body.candidate_handle(&kernel).is_none(){let result=self.body.step(&mut kernel,grant);self.receipt=self.body.normal_step_progress();if self.body.terminal_is_empty(){self.phase=4;self.slot=0;}drop(family);return result.map(|step|RetainedCloneStep::Progress(step.progress()));}
        }
        let demand=self.demand(session,grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_capacity_bytes<demand.capacity_bytes||grant.maximum_release_bytes<demand.release_bytes||grant.maximum_depth<demand.depth{return Ok(ownership_progress());}
        if self.phase>=4&&self.clock!=session.state.next_authority.load(std::sync::atomic::Ordering::Acquire){let family=session.state.claims.try_lock().map_err(|_|ownership_busy())?;let kernel=session.kernel().try_lock().map_err(|_|ownership_busy())?;self.clock=session.state.next_authority.load(std::sync::atomic::Ordering::Acquire);self.body=BrepRetentionJob::new(&kernel);self.phase=3;self.slot=0;self.family=0;self.claim=0;self.found=false;drop(family);return Ok(Self::advance());}
        if self.phase==0{
            let mut family=session.state.claims.try_lock().map_err(|_|ownership_busy())?;let own=family.get_mut(&session.state.authority).ok_or_else(ownership_busy)?;
            if grant.maximum_depth<own.handles.next_extract_slot_depth_demand(self.slot)?.max(1){return Ok(ownership_progress());}
            if self.slot>=own.handles.slot_count(){self.phase=1;self.slot=0;}else{if let Some((_,live))=own.handles.slot_entry_mut(self.slot){*live=false;}self.slot+=1;session.claims_changed();self.clock=session.state.next_authority.load(std::sync::atomic::Ordering::Acquire);}return Ok(Self::advance());
        }
        if self.phase==1{
            let source=self.request.as_mut().and_then(ControlledRetirement::original_mut).ok_or_else(ownership_busy)?;
            let Some(key)=source.get_mut(self.slot)else{self.phase=2;self.slot=0;return Ok(Self::advance());};
            let mut family=session.state.claims.try_lock().map_err(|_|ownership_busy())?;let own=family.get_mut(&session.state.authority).ok_or_else(ownership_busy)?;
            if let Some(live)=own.handles.get_mut(key.as_str()){*live=true;self.slot+=1;session.claims_changed();self.clock=session.state.next_authority.load(std::sync::atomic::Ordering::Acquire);return Ok(Self::advance());}
            if own.handles.next_insert_capacity_byte_demand(key,grant.maximum_copy_bytes)?!=0{return match own.handles.reserve_insert_step(key,grant){Ok(receipt)=>Ok(RetainedCloneStep::Progress(receipt)),Err((error,receipt))=>{self.receipt=receipt;Err(error)}};}
            let original=std::mem::take(key);match own.handles.insert_reserved(original,true,grant){Ok((previous,receipt))=>{assert!(previous.is_none());self.slot+=1;session.claims_changed();self.clock=session.state.next_authority.load(std::sync::atomic::Ordering::Acquire);Ok(RetainedCloneStep::Progress(receipt))},Err((error,original,_))=>{*key=original;Err(error)}}
        }else if self.phase==2{
            self.phase=3;Ok(Self::advance())
        }else if self.phase==3{
            let family=session.state.claims.try_lock().map_err(|_|ownership_busy())?;let mut kernel=session.kernel().try_lock().map_err(|_|ownership_busy())?;let clock=session.state.next_authority.load(std::sync::atomic::Ordering::Acquire);
            if clock!=self.clock{self.clock=clock;self.family=0;self.claim=0;self.found=false;self.body.restart();return Ok(Self::advance());}
            if let Some(handle)=self.body.candidate_handle(&kernel){
                let depth=family.next_extract_slot_depth_demand(self.family)?.max(1);let child=family.slot_entry(self.family).map_or(Ok(0),|(_,claims)|claims.handles.next_extract_slot_depth_demand(self.claim))?;if grant.maximum_depth<depth.checked_add(child).ok_or_else(ownership_busy)?{return Ok(ownership_progress());}
                if self.family>=family.slot_count(){self.body.decide(&kernel,self.found)?;self.family=0;self.claim=0;self.found=false;return Ok(Self::advance());}
                if let Some((_,claims))=family.slot_entry(self.family){if claims.active&&self.claim<claims.handles.slot_count(){if let Some((key,live))=claims.handles.slot_entry(self.claim){self.found|=*live&&key==handle;}self.claim+=1;}else{self.family+=1;self.claim=0;}}else{self.family+=1;self.claim=0;}return Ok(Self::advance());
            }
            let result=self.body.step(&mut kernel,grant);self.receipt=self.body.normal_step_progress();if self.body.terminal_is_empty(){self.phase=4;self.slot=0;}result.map(|step|RetainedCloneStep::Progress(step.progress()))
        }else if self.phase==4{
            let kernel=session.kernel().try_lock().map_err(|_|ownership_busy())?;let mut cache=session.mesh_cache().try_lock().map_err(|_|ownership_busy())?;
            if grant.maximum_depth<cache.next_extract_slot_depth_demand(self.slot)?.max(1){return Ok(ownership_progress());}
            if self.slot>=cache.slot_count(){self.phase=5;self.slot=0;return Ok(Self::advance());}
            if let Some(original)=cache.extract_slot_if(self.slot,|key,value|value.valid&&kernel.contains_live_handle(&key.0)){self.cache=Some(controlled(original));}self.slot+=1;Ok(Self::advance())
        }else if self.phase==5{
            let kernel=session.kernel().try_lock().map_err(|_|ownership_busy())?;let mut owner=session.tessellation_jobs().try_lock().map_err(|_|ownership_busy())?;let registry=owner.original_mut().ok_or_else(ownership_busy)?;
            if grant.maximum_depth<registry.jobs.next_extract_slot_depth_demand(self.slot)?.max(1){return Ok(ownership_progress());}
            if self.slot>=registry.jobs.slot_count(){self.phase=6;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress {copied_items:1,..Default::default()}));}
            if let Some(original)=registry.jobs.extract_slot_if(self.slot,|key,value|!value.retired&&kernel.contains_live_handle(&key.0)){self.job=Some(controlled(original));}self.slot+=1;Ok(Self::advance())
        }else{Ok(RetainedCloneStep::Complete(Default::default()))}
    }
    pub(super) fn cancel(&mut self){self.cancelled=true;self.body.cancel();}
}

/// 🧹️ Observes accepted original keys, claim slots and removed payloads through physical terminal closure.
#[cfg(test)]
#[test]
fn original_session_retention_transfers_incoming_keys_and_closes_removed_rows(){
    use super::physical_session_custody::observe;
    let port:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🌐️geometry/🧫️fixtures/🎟️work.json")).unwrap();
    let topology:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../🧰️framework/🔨️modules/🧊️3d/📐️brep/📸️representation/🕸️topology/🧫️fixtures/🎟️reachability/🔣️.json")).unwrap();
    let law=&topology["retention"];assert_eq!(law["keptBoxes"],1);assert_eq!(law["removedBoxes"],1);
    for cancel in [false,true]{
        let((source,kept,discard),heap)=observe(||{let source=Session::new();let(kept,discard)=source.with_kernel(|kernel|Ok((kernel.box_prim_sync(1.,1.,1.).unwrap(),kernel.box_prim_sync(2.,2.,2.).unwrap()))).unwrap();(source,kept,discard)});let(mut born,mut freed)=(heap.0,heap.1);
        let(request,heap)=observe(||{let mut request=Vec::with_capacity(port["requestCapacityItems"].as_u64().unwrap() as usize);request.push(kept.as_str().to_string());request});born+=heap.0;freed+=heap.1;let original=request.as_ptr();let key=request[0].as_ptr();
        let(mut job,heap)=observe(||SessionRetention::new(&source,request).unwrap_or_else(|(error,_)|panic!("original session request: {error}")));assert_eq!(heap,(0,0));assert_eq!(job.request.as_ref().unwrap().original().unwrap().as_ptr(),original);assert_eq!(job.request.as_ref().unwrap().original().unwrap()[0].as_ptr(),key);let mut turns=0;
        while !job.terminal_is_empty(){
            if cancel&&turns==10{let(_,heap)=observe(||job.cancel());assert_eq!(heap,(0,0));}
            let copy=job.demand(&source,0).unwrap().copy_bytes;let demand=job.demand(&source,copy).unwrap();let grant=RetainedCloneGrant {maximum_items:law["itemsPerTurn"].as_u64().unwrap() as usize,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
            let(step,heap)=observe(||job.step(&source,RetainedCloneGrant {maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!(heap,(0,0));
            let(step,heap)=observe(||job.step(&source,grant).unwrap());assert_eq!(step.progress(),job.normal_step_progress());assert!(step.progress().fits(grant));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;turns+=1;assert!(turns<100000);
        }
        {let kernel=source.kernel().try_lock().unwrap();assert!(kernel.contains_live_handle(kept.as_str()));assert_eq!(kernel.contains_live_handle(discard.as_str()),cancel);}
        let(_,heap)=observe(||drop(job));assert_eq!(heap,(0,0));
        let(mut caller,heap)=observe(||controlled((kept.0,discard.0)));assert_eq!(heap,(0,0));while !caller.terminal_is_empty(){let copy=caller.next_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:caller.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:caller.next_release_byte_demand().unwrap(),maximum_depth:caller.next_depth_demand().unwrap()};let(step,heap)=observe(||caller.step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}let(_,heap)=observe(||drop(caller));assert_eq!(heap,(0,0));
        let(mut capture,heap)=observe(||SessionCapture::owned(source));assert_eq!(heap,(0,0));while !capture.terminal_is_empty(){let copy=capture.next_close_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:capture.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:capture.next_close_release_byte_demand().unwrap(),maximum_depth:capture.next_close_depth_demand().unwrap()};let(step,heap)=observe(||capture.close_step(grant).unwrap());assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.0;freed+=heap.1;}let(_,heap)=observe(||drop(capture));assert_eq!(heap,(0,0));assert_eq!(born,freed);eprintln!("[DEBUG] Original Session retention cancel={cancel} originalRequest=true claimSlots=true physical={freed} turns={turns} terminalDrop=0");
    }
}
