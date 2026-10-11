use super::*;
use semio_framework_value::retirement::controlled::ControlledRetirement;
#[path = "../../../../../../../🔨️modules/⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
mod allocation;
#[global_allocator]
static ORIGINAL_PORT_ALLOCATOR: allocation::RequestedAllocator=allocation::RequestedAllocator;

struct OriginalPort { owner:semio_framework_value::retirement::controlled::RetainedOwnerGate<ControlledRetirement<String>>, retaining:Option<ControlledRetirement<Vec<String>>>,cancelling:bool,receipt:RetainedCloneProgress }
/// 🔒️ Test fixture only: the port is driven from one thread and `retaining` is read exclusively through `&self` demand queries between steps.
unsafe impl Sync for OriginalPort {}
impl OriginalPort {fn demand(&self,read:impl FnOnce(&ControlledRetirement<String>)->Result<usize,ValueError>)->Result<usize,ValueError>{let owner=self.owner.try_lock().map_err(|_|ValueError::literal(ValueRefusalKind::WorkLimit,"original port owner busy"))?;read(&owner)}}
impl GeometryPort for OriginalPort {
    fn begin_retain(&mut self,handles:Vec<String>)->Result<(),(ValueError,Vec<String>)>{if self.retaining.is_some(){return Err((ValueError::literal(ValueRefusalKind::WorkLimit,"original retain request busy"),handles));}self.retaining=Some(ControlledRetirement::new(handles)?);Ok(())}
    fn retain_terminal_is_empty(&self)->bool{self.retaining.is_none()}
    fn next_retain_copy_byte_demand(&self)->Result<usize,ValueError>{self.retaining.as_ref().map_or(Ok(0),ControlledRetirement::next_copy_byte_demand)}
    fn next_retain_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{self.retaining.as_ref().map_or(Ok(0),|owner|owner.next_capacity_byte_demand(copy))}
    fn next_retain_release_byte_demand(&self)->Result<usize,ValueError>{self.retaining.as_ref().map_or(Ok(0),ControlledRetirement::next_release_byte_demand)}
    fn next_retain_depth_demand(&self)->Result<usize,ValueError>{self.retaining.as_ref().map_or(Ok(0),ControlledRetirement::next_depth_demand)}
    fn retain_step_progress(&self)->RetainedCloneProgress{self.receipt}
    fn retain_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.receipt=Default::default();let Some(owner)=&mut self.retaining else{return Ok(RetainedCloneStep::Complete(Default::default()));};let result=owner.step(grant);self.receipt=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.retaining=None;}Ok(step)}
    fn next_tessellate_copy_byte_demand(&self,_:&str,_:f64)->Result<usize,ValueError>{Ok(0)}
    fn next_tessellate_capacity_byte_demand(&self,_:&str,_:f64,_:usize)->Result<usize,ValueError>{Ok(0)}
    fn next_tessellate_release_byte_demand(&self,_:&str,_:f64)->Result<usize,ValueError>{Ok(0)}
    fn next_tessellate_depth_demand(&self,_:&str,_:f64)->Result<usize,ValueError>{Ok(0)}
    fn tessellate_step(&self,_:&str,_:f64,_:usize,_:RetainedCloneGrant)->Result<(GeometryStep,RetainedCloneProgress),ValueError>{Ok((GeometryStep::Cancelled,Default::default()))}
    fn dispose(&self,_:&str)->Result<(),String>{Ok(())}
    fn begin_cancel(&mut self)->Result<(),ValueError>{self.cancelling=true;Ok(())}
    fn cancel_terminal_is_empty(&self)->bool{!self.cancelling}
    fn next_cancel_copy_byte_demand(&self)->Result<usize,ValueError>{if self.cancelling{self.demand(ControlledRetirement::next_copy_byte_demand)}else{Ok(0)}}
    fn next_cancel_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{if self.cancelling{self.demand(|owner|owner.next_capacity_byte_demand(copy))}else{Ok(0)}}
    fn next_cancel_release_byte_demand(&self)->Result<usize,ValueError>{if self.cancelling{self.demand(ControlledRetirement::next_release_byte_demand)}else{Ok(0)}}
    fn next_cancel_depth_demand(&self)->Result<usize,ValueError>{if self.cancelling{self.demand(ControlledRetirement::next_depth_demand)}else{Ok(0)}}
    fn cancel_step_progress(&self)->RetainedCloneProgress{self.receipt}
    fn cancel_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{self.receipt=Default::default();if !self.cancelling{return Ok(RetainedCloneStep::Complete(Default::default()));}let owner=self.owner.get_mut();let result=owner.step(grant);self.receipt=owner.step_progress();let step=result?;if owner.terminal_is_empty(){self.cancelling=false;}Ok(step)}
    fn begin_close(&self){}
    fn terminal_is_empty(&self)->bool{self.retain_terminal_is_empty() && self.cancel_terminal_is_empty() && self.owner.try_lock().is_ok_and(|owner|owner.terminal_is_empty())}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{if !self.retain_terminal_is_empty(){return self.retain_step(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}if self.cancelling{return self.cancel_step(grant);}self.owner.get_mut().step(grant)}
    fn next_copy_byte_demand(&self)->Result<usize,ValueError>{if !self.retain_terminal_is_empty(){return self.next_retain_copy_byte_demand();}self.demand(ControlledRetirement::next_copy_byte_demand)}
    fn next_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{if !self.retain_terminal_is_empty(){return self.next_retain_capacity_byte_demand(copy);}self.demand(|owner|owner.next_capacity_byte_demand(copy))}
    fn next_release_byte_demand(&self)->Result<usize,ValueError>{if !self.retain_terminal_is_empty(){return self.next_retain_release_byte_demand();}self.demand(ControlledRetirement::next_release_byte_demand)}
    fn next_depth_demand(&self)->Result<usize,ValueError>{if !self.retain_terminal_is_empty(){return self.next_retain_depth_demand();}self.demand(ControlledRetirement::next_depth_demand)}
}

/// 🌐️ Preserves original supplied port and physical Box through every independently denied grant.
#[test]
fn original_geometry_port_full_grant_retains_child_and_terminal_box() {
    use allocation::observe_backing;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/♻️ownership/🔣️.json")).unwrap();
    for copy in fixture["copyGrants"].as_array().unwrap(){
        let ((port,pointer),source_births,source_frees)=observe_backing(||{let mut value=String::with_capacity(fixture["source"]["capacityBytes"].as_u64().unwrap() as usize);value.push_str(fixture["source"]["text"].as_str().unwrap());let pointer=value.as_ptr();let mut port=Box::new(OriginalPort{owner:semio_framework_value::retirement::controlled::RetainedOwnerGate::new(ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("original port source refused: {error}"))),retaining:None,cancelling:false,receipt:Default::default()});assert_eq!(port.owner.get_mut().original().unwrap().as_ptr(),pointer);(port as Box<dyn GeometryPort>,pointer)});
        assert_eq!(source_frees,0);
        let original_port=port.as_ref() as *const dyn GeometryPort as *const ();
        let (mut retirement,births,frees)=observe_backing(||GeometryPortRetirement::new(port));assert_eq!((births,frees),(0,0));
        let denied=RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0};
        for grant in [denied,RetainedCloneGrant{maximum_items:1,..denied}]{let (step,births,frees)=observe_backing(||retirement.close_step(grant).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((births,frees),(0,0));assert_eq!(retirement.port.as_ref().unwrap().as_ref() as *const dyn GeometryPort as *const (),original_port);}
        let copy=copy.as_u64().unwrap() as usize;let mut cursor_births=0;let mut released=0;let mut logical=0;let mut refused=0;let mut turns=0;let mut box_released=false;
        while !retirement.terminal_is_empty(){
            turns+=1;assert!(turns<65536);
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:retirement.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:retirement.next_release_byte_demand().unwrap(),maximum_depth:retirement.next_depth_demand().unwrap()};
            let capacity=grant.maximum_capacity_bytes.checked_sub(1).map(|bytes|RetainedCloneGrant{maximum_capacity_bytes:bytes,..grant});
            let release=(retirement.next_copy_byte_demand().unwrap()==0).then(||grant.maximum_release_bytes.checked_sub(1).map(|bytes|RetainedCloneGrant{maximum_release_bytes:bytes,..grant})).flatten();
            for denied in [capacity,release,grant.maximum_depth.checked_sub(1).map(|depth|RetainedCloneGrant{maximum_depth:depth,..grant})].into_iter().flatten(){let (step,births,frees)=observe_backing(||retirement.close_step(denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((births,frees),(0,0));assert_eq!(retirement.port.as_ref().unwrap().as_ref() as *const dyn GeometryPort as *const (),original_port);refused+=1;}
            let terminal_child=retirement.port.as_ref().unwrap().terminal_is_empty();
            let (step,births,frees)=observe_backing(||retirement.close_step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!((births,frees),(progress.retained_capacity_bytes,progress.released_bytes));cursor_births+=births;released+=frees;logical+=progress.copied_bytes;
            if terminal_child{assert_eq!(frees,std::mem::size_of::<OriginalPort>());assert!(matches!(step,RetainedCloneStep::Complete(_)));box_released=true;}
        }
        assert!(box_released);assert!(refused>0);assert_eq!(logical,fixture["expected"]["utf8Bytes"].as_u64().unwrap() as usize);assert_eq!(released,source_births+cursor_births);
        let (_,births,frees)=observe_backing(||drop(retirement));assert_eq!((births,frees),(0,0));
        eprintln!("[DEBUG] original GeometryPort sameBox=true sourcePointer={pointer:p} copy={copy} actualSource={source_births} actualCursor={cursor_births} actualReleased={released} refused={refused} turns={turns} outerBoxSeparate=true terminalDropFree=0");
    }
}

/// 📥️ Accepted and refused retain requests keep exact caller vectors and close with actual original physical receipts.
#[test]
fn original_geometry_port_retain_returns_exact_refused_source() {
    use allocation::observe_backing;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️work.json")).unwrap();
    let mut port=OriginalPort {owner:semio_framework_value::retirement::controlled::RetainedOwnerGate::new(ControlledRetirement::new(String::new()).unwrap_or_else(|_|unreachable!())),retaining:None,cancelling:false,receipt:Default::default()};
    drop(port.owner.get_mut().take_original());
    let (handles,born,freed)=observe_backing(||{let mut handles=Vec::with_capacity(fixture["requestCapacityItems"].as_u64().unwrap() as usize);for handle in fixture["handles"].as_array().unwrap(){handles.push(handle.as_str().unwrap().to_string());}handles});assert_eq!(freed,0);let vector=handles.as_ptr();let key=handles[0].as_ptr();
    let (accepted,birth,release)=observe_backing(||port.begin_retain(handles));accepted.unwrap_or_else(|(error,_)|panic!("original request admission: {error}"));assert_eq!((birth,release),(0,0));assert_eq!(port.retaining.as_ref().unwrap().original().unwrap().as_ptr(),vector);assert_eq!(port.retaining.as_ref().unwrap().original().unwrap()[0].as_ptr(),key);
    let incoming=fixture["refusedHandles"].as_array().unwrap().iter().map(|handle|handle.as_str().unwrap().to_string()).collect::<Vec<_>>();let incoming_pointer=incoming.as_ptr();let incoming_key=incoming[0].as_ptr();let (refused,birth,release)=observe_backing(||port.begin_retain(incoming));let (_,incoming)=refused.err().unwrap();assert_eq!((birth,release),(0,0));assert_eq!(incoming.as_ptr(),incoming_pointer);assert_eq!(incoming[0].as_ptr(),incoming_key);drop(incoming);
    let zero=RetainedCloneGrant {maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0};let(step,birth,release)=observe_backing(||port.retain_step(zero).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((birth,release),(0,0));
    let(mut admitted,mut physical,mut turns)=(0,0,0);while !port.retain_terminal_is_empty() {let copy=port.next_retain_copy_byte_demand().unwrap();let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:port.next_retain_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:port.next_retain_release_byte_demand().unwrap(),maximum_depth:port.next_retain_depth_demand().unwrap()};let(step,birth,release)=observe_backing(||port.retain_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((birth,release),(step.progress().retained_capacity_bytes,step.progress().released_bytes));admitted+=birth;physical+=release;turns+=1;assert!(turns<10000);}
    assert_eq!(born+admitted,physical);let(_,birth,release)=observe_backing(||drop(port));assert_eq!((birth,release),(0,0));eprintln!("[DEBUG] Original geometry retain exactSource=true refusedSameVector=true source={born} admitted={admitted} physical={physical} turns={turns} terminalDrop=0");
}

/// 🛑️ Cancellation flags the original owner without effects and then closes its exact payload under caller grants.
#[test]
fn original_geometry_port_cancel_retains_actual_payload_until_funded_closure(){
    use allocation::observe_backing;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/♻️ownership/🔣️.json")).unwrap();
    for copy in fixture["copyGrants"].as_array().unwrap(){
        let((mut port,pointer),source,before)=observe_backing(||{let mut text=String::with_capacity(fixture["source"]["capacityBytes"].as_u64().unwrap()as usize);text.push_str(fixture["source"]["text"].as_str().unwrap());let pointer=text.as_ptr();(OriginalPort {owner:semio_framework_value::retirement::controlled::RetainedOwnerGate::new(ControlledRetirement::new(text).unwrap_or_else(|_|unreachable!())),retaining:None,cancelling:false,receipt:Default::default()},pointer)});assert_eq!(before,0);
        let(result,birth,release)=observe_backing(||port.begin_cancel());result.unwrap();assert_eq!((birth,release),(0,0));assert_eq!(port.owner.get_mut().original().unwrap().as_ptr(),pointer);assert!(!port.cancel_terminal_is_empty());
        let zero=RetainedCloneGrant {maximum_items:0,maximum_copy_bytes:0,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:0};let(step,birth,release)=observe_backing(||port.cancel_step(zero).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((birth,release),(0,0));assert!(!port.cancel_terminal_is_empty());
        let copy=copy.as_u64().unwrap()as usize;let(mut born,mut freed,mut turns)=(0,0,0);
        while !port.cancel_terminal_is_empty(){let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:port.next_cancel_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:port.next_cancel_release_byte_demand().unwrap(),maximum_depth:port.next_cancel_depth_demand().unwrap()};for denied in [(grant.maximum_capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(1),..grant}),(grant.maximum_release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(1),..grant})].into_iter().flatten(){let(step,birth,release)=observe_backing(||port.cancel_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((birth,release),(0,0));assert!(!port.cancel_terminal_is_empty());}let(step,birth,release)=observe_backing(||port.cancel_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((birth,release),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=birth;freed+=release;turns+=1;assert!(turns<100000);}
        assert_eq!(source+born,freed);let(_,birth,release)=observe_backing(||drop(port));assert_eq!((birth,release),(0,0));eprintln!("[DEBUG] Original geometry cancel source={source} admitted={born} physical={freed} copy={copy} turns={turns} beginEffects=0 terminalDrop=0");
    }
}
