use super::*;

/// 🏦️ Drives one step under an explicit finite authoring wallet equal to the granted stock: no job turn exists in these laws.
pub(super) fn funded<R>(grant:semio_framework_value::retained_clone::RetainedCloneGrant,step:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->R)->R{semio_framework_value::AuthoringWallet::new(grant,1<<20).run(grant,1<<16,step).unwrap().expect("work quote admitted").output}
/// 🏁️ Reads a completed retirement poll as its funded step; a pending poll made no progress.
pub(super) fn settled(poll:semio_framework_value::OriginalRetirementPoll)->Result<semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::ValueError>{match poll{semio_framework_value::RetainedWorkPoll::Complete(turn)=>turn.result.map_err(semio_framework_value::OriginalRetirementFailure::into_value_error),semio_framework_value::RetainedWorkPoll::Pending{..}=>Ok(semio_framework_value::retained_clone::RetainedCloneStep::Progress(Default::default()))}}
/// 🔁️ A controlled owner stepped under a granted effect, concrete or erased.
pub(super) trait NarrowOwner {
    fn narrow_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll;
    fn narrow_need(&self,copy:usize)->usize;
}
impl<T:semio_framework_value::retirement::RetireOwned> NarrowOwner for semio_framework_value::retirement::controlled::ControlledRetirement<T> {
    fn narrow_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{self.step(effect)}
    fn narrow_need(&self,copy:usize)->usize{self.next_demand(copy).unwrap().copy_bytes}
}
impl NarrowOwner for &mut dyn semio_framework_value::retirement::controlled::ErasedControlledRetirement {
    fn narrow_step(&mut self,effect:&mut semio_framework_value::RetainedAllocationEffect<'_>)->semio_framework_value::OriginalRetirementPoll{(**self).step(effect)}
    fn narrow_need(&self,copy:usize)->usize{(**self).next_demand(copy).unwrap().copy_bytes}
}
/// 🔁️ Steps a controlled owner under `grant`; a refused quote retries once with the copy grant widened to the owner's exact quote, and the grant actually used is returned.
pub(super) fn narrow(grant:semio_framework_value::retained_clone::RetainedCloneGrant,owner:&mut impl NarrowOwner)->(semio_framework_value::retained_clone::RetainedCloneStep,semio_framework_value::retained_clone::RetainedCloneGrant){
    let first=settled(funded(grant,|effect|owner.narrow_step(effect))).unwrap();
    if first.progress()!=Default::default(){return(first,grant)}
    let wide=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.max(owner.narrow_need(grant.maximum_copy_bytes)),..grant};
    (settled(funded(wide,|effect|owner.narrow_step(effect))).unwrap(),wide)
}
/// ⏱️ Runs one normal tessellation step under its exact funded grant.
pub(super) fn step_normal(job:&mut MeshTessellationJob)->Result<(MeshTessellationStep,semio_framework_value::retained_clone::RetainedCloneProgress),semio_framework_value::ValueError>{let grant=funded_normal_grant(job);funded(grant,|effect|job.step(effect))}
pub(super) fn funded_normal_grant(job:&MeshTessellationJob)->semio_framework_value::retained_clone::RetainedCloneGrant{semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:128,maximum_capacity_bytes:job.next_normal_capacity_byte_demand(128).unwrap(),maximum_release_bytes:job.next_normal_release_byte_demand().unwrap(),maximum_depth:job.next_normal_depth_demand().unwrap()}}

pub(super) fn authored_source(law:&serde_json::Value)->HalfedgeMesh{
    let positions=serde_json::from_value::<Vec<[f32;3]>>(law["positions"].clone()).unwrap();let faces=serde_json::from_value::<Vec<Vec<u32>>>(law["faces"].clone()).unwrap();let mut mesh=HalfedgeMesh::from_faces(&positions,&faces).unwrap();for record in law["metadataCapture"]["attributes"].as_array().unwrap(){let semantic=match record["semantic"].as_str().unwrap(){"normal"=>MeshAttributeSemantic::Normal,"uv"=>MeshAttributeSemantic::Uv,"color"=>MeshAttributeSemantic::Color,_=>panic!("unknown neutral semantic")};mesh.set_attribute(record["name"].as_str().unwrap().into(),MeshAttribute {domain:MeshAttributeDomain::Vertex,semantic,interpolation:semio_framework_mesh_engine::MeshAttributeInterpolation::Linear,values:record["values"].as_array().unwrap().iter().cloned().map(semio_framework_value::DslValue::from).collect(),indices:None}).unwrap();}mesh
}

/// 🎨️ One original metadata slot captures an inline identity while original keys and sample allocations remain unchanged.
#[test]
fn original_mesh_tessellation_metadata_capture_borrows_exact_source_rows(){
    use crate::test_support::observe_tessellation_system as observe;
    use semio_framework_value::{retained_clone::{RetainedCloneGrant,RetainedCloneProgress},retirement::controlled::ControlledRetirement};
    let law:serde_json::Value=crate::fixture_json("🧪️tests/🧫️fixtures/🧩️tessellation/🎟️owners.json");
    for copy in law["copyGrants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){
        let(mut job,source)=observe(||MeshTessellationJob::new(super::tessellation_metadata_tests::authored_source(&law)));
        let mut originals=[(0usize,0usize);3];for(index,(key,attribute))in job.mesh.attributes.iter().enumerate(){originals[index]=(key.as_ptr()as usize,attribute.values.as_ptr()as usize);}
        let mut turns=0;
        while !job.metadata_capture_complete(){let grant=RetainedCloneGrant {maximum_items:law["metadataCapture"]["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:copy,maximum_capacity_bytes:job.next_metadata_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:job.next_metadata_release_byte_demand().unwrap(),maximum_depth:job.next_metadata_depth_demand().unwrap()};assert_eq!(job.next_metadata_copy_byte_demand().unwrap(),0);assert_eq!(grant.maximum_capacity_bytes,0);assert_eq!(grant.maximum_release_bytes,0);for denied in [RetainedCloneGrant {maximum_items:0,..grant},RetainedCloneGrant {maximum_depth:grant.maximum_depth-1,..grant}]{let(step,heap)=observe(||job.capture_metadata_step(denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!(heap,(0,0));assert_eq!(job.metadata_receipt.0,RetainedCloneProgress::default());}let(step,heap)=observe(||job.capture_metadata_step(grant).unwrap());assert_eq!(step.progress().copied_items,1);assert_eq!(step.progress(),job.metadata_receipt.0);assert_eq!(heap,(0,0));turns+=1;for(index,(key,attribute))in job.mesh.attributes.iter().enumerate(){assert_eq!((key.as_ptr()as usize,attribute.values.as_ptr()as usize),originals[index]);}}
        assert_eq!(turns,law["metadataCapture"]["attributes"].as_array().unwrap().len());for semantic in [MeshAttributeSemantic::Normal,MeshAttributeSemantic::Uv,MeshAttributeSemantic::Color]{let attribute=job.semantic_attribute(semantic).unwrap();let source=job.mesh.attributes.values().find(|attribute|attribute.semantic==semantic).unwrap();assert_eq!(attribute.semantic,semantic);assert!(std::ptr::eq(source,attribute));}
        let(mut owner,handoff)=observe(||ControlledRetirement::new(job).unwrap_or_else(|_|panic!("original metadata closure unsupported")));assert_eq!(handoff,(0,0));let(mut born,mut freed,mut close_turns)=(source.0,source.1,0);
        while !owner.terminal_is_empty(){let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_release_byte_demand().unwrap(),maximum_depth:owner.next_depth_demand().unwrap()};let((step,grant),heap)=observe(||super::tessellation_metadata_tests::narrow(grant,&mut owner));assert_eq!(heap,(step.progress().retained_capacity_bytes,step.progress().released_bytes));assert!(step.progress().fits(grant));born+=heap.0;freed+=heap.1;close_turns+=1;assert!(close_turns<100000);}
        assert_eq!(born,freed);let(_,heap)=observe(||drop(owner));assert_eq!(heap,(0,0));
    }
}
