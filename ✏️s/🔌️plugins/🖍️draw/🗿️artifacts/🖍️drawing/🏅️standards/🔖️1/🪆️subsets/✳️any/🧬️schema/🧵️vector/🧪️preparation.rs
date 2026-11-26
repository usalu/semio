//! 🧪️ Actual original Draw custody, native heap receipts and shared refusal witnesses reach the same preparation frontier.
use semio_framework_io_ports::{ChildAdmissionRead, ChildAdmissionSource, EmbeddedChildAdmitterV1, OriginalChildVectorPreparation, erase_child_admission_source, vector::{EmbeddedChildVectorSourceV1, VectorGraphAdvance, VectorGraphSourceV1}};
use semio_framework_value::{RetirementDemand, ValueError, ValueRefusalKind, retained_clone::{RetainedCloneGrant, RetainedCloneProgress}};
use semio_framework_value_clone_source_testing::FixtureSource;
use semio_framework_value_testing_allocator::observe_retirement_allocations;
use semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot;
use semio_s_artifact_draw_drawing_model::validation::DRAWING_ADMITTER;
use semio_s_artifact_draw_drawing_model::vector_source::DRAWING_VECTOR_SOURCE;
use crate::work_fixture::{with_effect, WORK};
use semio_framework_io_ports::OriginalChildVectorAdvance;
use semio_framework_value::RetainedCloneStep;

fn advance<'a>(owner: &mut OriginalChildVectorPreparation, read: &ChildAdmissionRead<'a>, grant: RetainedCloneGrant) -> Result<OriginalChildVectorAdvance<'a>, ValueError> {
    let (result, heap) = with_effect(grant, WORK, &mut None, |effect| observe_retirement_allocations(|| owner.advance(read, effect)));
    let progress = result.as_ref().map_or_else(ValueError::retained_progress, |step| step.progress);
    assert!(progress.fits(grant));
    assert_eq!(heap, (progress.retained_capacity_bytes, progress.released_bytes), "vector advance under {grant:?}");
    result
}

fn close(owner: &mut OriginalChildVectorPreparation, grant: RetainedCloneGrant) -> RetainedCloneStep {
    let (result, heap) = with_effect(grant, WORK, &mut None, |effect| observe_retirement_allocations(|| owner.close_step(effect)));
    let step = result.unwrap();
    let progress = step.progress();
    assert!(progress.fits(grant));
    assert_eq!(heap, (progress.retained_capacity_bytes, progress.released_bytes), "vector close under {grant:?}");
    step
}
fn g(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
fn observed<T>(grant:RetainedCloneGrant,operation:impl FnOnce()->(T,RetainedCloneProgress))->T{let((result,p),heap)=observe_retirement_allocations(operation);assert!(p.fits(grant));assert_eq!(heap,(p.retained_capacity_bytes,p.released_bytes));result}
fn refuse_axes(d:RetirementDemand,mut operation:impl FnMut(RetainedCloneGrant)->RetainedCloneProgress){let original=g(d);for axis in 0..5{let mut denied=original;let value=match axis{0=>&mut denied.maximum_items,1=>&mut denied.maximum_copy_bytes,2=>&mut denied.maximum_capacity_bytes,3=>&mut denied.maximum_release_bytes,_=>&mut denied.maximum_depth};if *value==0{continue}*value-=1;assert_eq!(operation(denied),Default::default());}}
fn corpus()->serde_json::Value{serde_json::from_str(include_str!("../✅️validation/🧪️.json")).unwrap()}
fn patched(base:&serde_json::Value,patches:&serde_json::Value)->serde_json::Value{let mut value=base.clone();for patch in patches.as_array().unwrap(){let at=patch["path"].as_str().unwrap();if patch["op"]=="remove"{value.as_object_mut().unwrap().remove("artboard");}else{*value.pointer_mut(at).unwrap()=patch["value"].clone()}}value}
fn run(value:serde_json::Value,valid:bool,cut:Option<usize>){let expected:DrawingSnapshot=serde_json::from_value(value.clone()).unwrap();let source=FixtureSource::from_owner(expected.clone());let d=OriginalChildVectorPreparation::birth_demand();let mut owner=observed(g(d),||{let(owner,p)=OriginalChildVectorPreparation::admit(g(d)).unwrap();(owner,p)});let mut capacity=0;let mut release=0;let mut done=false;
 for _ in 0..cut.unwrap_or(100000){let read=ChildAdmissionRead{source:erase_child_admission_source(source.borrow(),0x4348494c),admitter:&DRAWING_ADMITTER,vector:&DRAWING_VECTOR_SOURCE};let d=owner.next_demand(&read,16*1024*1024).unwrap();refuse_axes(d,|grant|advance(&mut owner,&read,grant).unwrap().progress);let step=advance(&mut owner,&read,g(d)).unwrap();capacity+=step.progress.retained_capacity_bytes;release+=step.progress.released_bytes;if step.done{assert_eq!(capacity,release);done=true;assert_eq!(step.error.is_none(),valid);assert_eq!(step.graph.is_some(),valid);if let Some(graph)=step.graph{assert_eq!(graph as*const dyn VectorGraphSourceV1 as*const (),source.borrow().get()as*const DrawingSnapshot as*const ());assert_eq!(graph.root_count(),expected.layers.len());assert_eq!(step.canvas.is_some(),value.get("artboard").is_some());}break}}
 if cut.is_none(){assert!(done)}owner.begin_close();for _ in 0..100000{if owner.terminal_is_empty(){break}let d=owner.close_demand(16*1024*1024).unwrap();refuse_axes(d,|grant|close(&mut owner,grant).progress());let p=close(&mut owner,g(d)).progress();capacity+=p.retained_capacity_bytes;release+=p.released_bytes;}assert!(owner.terminal_is_empty());assert_eq!(capacity,release);assert_eq!(source.borrow().get(),&expected);}
#[test]
fn genuine_vector_preparation_matches_shared_owned_admission_oracle_and_physical_receipts(){let laws:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🔌️ports/🧩️admission/🧵️vector/🧳️prepare/🧪️.json")).unwrap();assert_eq!(laws["cases"].as_array().unwrap().len(),13);let corpus=corpus();for case in corpus["cases"].as_array().unwrap(){run(patched(&corpus["base"],&case["patch"]),case["valid"].as_bool().unwrap(),None)}for cut in laws["cases"][8]["cuts"].as_array().unwrap(){run(corpus["base"].clone(),true,Some(cut.as_u64().unwrap()as usize))}}
struct OtherAdmitter;
impl EmbeddedChildAdmitterV1 for OtherAdmitter{fn birth_demand(&self)->RetirementDemand{DRAWING_ADMITTER.birth_demand()}fn begin(&self,g:RetainedCloneGrant)->Result<(semio_framework_io_ports::EmbeddedChildAdmission,RetainedCloneProgress),ValueError>{DRAWING_ADMITTER.begin(g)}}
static OTHER_ADMITTER:OtherAdmitter=OtherAdmitter;
struct OtherVector;
impl EmbeddedChildVectorSourceV1 for OtherVector{fn read_demand(&self)->RetirementDemand{DRAWING_VECTOR_SOURCE.read_demand()}fn read<'a>(&self,s:ChildAdmissionSource<'a>,g:RetainedCloneGrant)->Result<VectorGraphAdvance<'a>,ValueError>{DRAWING_VECTOR_SOURCE.read(s,g)}}
static OTHER_VECTOR:OtherVector=OtherVector;
#[test]
fn original_source_projection_and_registered_pair_cannot_change(){let document:DrawingSnapshot=serde_json::from_value(corpus()["base"].clone()).unwrap();let source=FixtureSource::from_owner(document.clone());let other=FixtureSource::from_owner(document);let(mut owner,_)=OriginalChildVectorPreparation::admit(g(OriginalChildVectorPreparation::birth_demand())).unwrap();{let read=ChildAdmissionRead{source:erase_child_admission_source(source.borrow(),1),admitter:&DRAWING_ADMITTER,vector:&DRAWING_VECTOR_SOURCE};let d=owner.next_demand(&read,8192).unwrap();advance(&mut owner,&read,g(d)).unwrap();}for read in [ChildAdmissionRead{source:erase_child_admission_source(other.borrow(),1),admitter:&DRAWING_ADMITTER,vector:&DRAWING_VECTOR_SOURCE},ChildAdmissionRead{source:erase_child_admission_source(source.borrow(),2),admitter:&DRAWING_ADMITTER,vector:&DRAWING_VECTOR_SOURCE},ChildAdmissionRead{source:erase_child_admission_source(source.borrow(),1),admitter:&OTHER_ADMITTER,vector:&DRAWING_VECTOR_SOURCE},ChildAdmissionRead{source:erase_child_admission_source(source.borrow(),1),admitter:&DRAWING_ADMITTER,vector:&OTHER_VECTOR}]{let(result,heap)=observe_retirement_allocations(||owner.next_demand(&read,8192));assert!(result.is_err());assert_eq!(heap,(0,0));}owner.begin_close();while !owner.terminal_is_empty(){let d=owner.close_demand(8192).unwrap();close(&mut owner,g(d));}}

struct FailingVector;
impl EmbeddedChildVectorSourceV1 for FailingVector{fn read_demand(&self)->RetirementDemand{DRAWING_VECTOR_SOURCE.read_demand()}fn read<'a>(&self,s:ChildAdmissionSource<'a>,g:RetainedCloneGrant)->Result<VectorGraphAdvance<'a>,ValueError>{let step=DRAWING_VECTOR_SOURCE.read(s,g)?;Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Paid consumer refusal").with_retained_progress(step.progress))}}
static FAILING_VECTOR:FailingVector=FailingVector;
#[test]
fn paid_consumer_error_keeps_original_custody_and_exact_receipt(){let document:DrawingSnapshot=serde_json::from_value(corpus()["base"].clone()).unwrap();let source=FixtureSource::from_owner(document);let(mut owner,_)=OriginalChildVectorPreparation::admit(g(OriginalChildVectorPreparation::birth_demand())).unwrap();let mut failed=false;for _ in 0..100000{let read=ChildAdmissionRead{source:erase_child_admission_source(source.borrow(),1),admitter:&DRAWING_ADMITTER,vector:&FAILING_VECTOR};let d=owner.next_demand(&read,16*1024*1024).unwrap();match advance(&mut owner,&read,g(d)){Ok(_)=>{},Err(error)=>{assert_eq!(error.retained_progress().copied_bytes,d.copy_bytes);failed=true;break}}}assert!(failed);owner.begin_close();while !owner.terminal_is_empty(){let d=owner.close_demand(8192).unwrap();close(&mut owner,g(d));}}

struct MalformedVector{present:bool}
impl EmbeddedChildVectorSourceV1 for MalformedVector{fn read_demand(&self)->RetirementDemand{DRAWING_VECTOR_SOURCE.read_demand()}fn read<'a>(&self,s:ChildAdmissionSource<'a>,g:RetainedCloneGrant)->Result<VectorGraphAdvance<'a>,ValueError>{let mut step=DRAWING_VECTOR_SOURCE.read(s,g)?;step.done=!self.present;if !self.present{step.graph=None}Ok(step)}}
static PREMATURE_VECTOR:MalformedVector=MalformedVector{present:true};static MISSING_VECTOR:MalformedVector=MalformedVector{present:false};
#[test]
fn graph_publication_requires_complete_original_read_custody(){for vector in [&PREMATURE_VECTOR,&MISSING_VECTOR]{let document:DrawingSnapshot=serde_json::from_value(corpus()["base"].clone()).unwrap();let source=FixtureSource::from_owner(document);let(mut owner,_)=OriginalChildVectorPreparation::admit(g(OriginalChildVectorPreparation::birth_demand())).unwrap();let mut failed=false;for _ in 0..100000{let read=ChildAdmissionRead{source:erase_child_admission_source(source.borrow(),1),admitter:&DRAWING_ADMITTER,vector};let d=owner.next_demand(&read,16*1024*1024).unwrap();match advance(&mut owner,&read,g(d)){Ok(step)=>{assert!(step.graph.is_none());},Err(error)=>{let progress=error.retained_progress();assert_eq!(progress.copied_bytes,d.copy_bytes);assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(0,0));failed=true;break}}}assert!(failed);owner.begin_close();while !owner.terminal_is_empty(){let d=owner.close_demand(8192).unwrap();close(&mut owner,g(d));}}}
