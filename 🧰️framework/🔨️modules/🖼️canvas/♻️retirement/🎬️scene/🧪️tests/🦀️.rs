#[global_allocator]
static ORIGINAL_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
use semio_framework_canvas_scene::{Scene,CanvasSceneRetirement,BezPath,Stroke,FillRule,Affine,Color,RasterImage};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observed;
fn law()->serde_json::Value{serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap()}
#[test]
fn original_fixed_pages_birth_release_and_terminal_drop_use_exact_physical_receipts(){
 let grant:RetainedCloneGrant=serde_json::from_value(law()["grant"].clone()).unwrap();
 let(mut pages,heap)=observed(||semio_framework_async::OriginalFixedSlotPages::<u64,16,16>::new());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
 let mut born=0;while !pages.prepared(){let extent=pages.birth_bytes();for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_capacity_bytes:extent-1,..grant}]{let(step,heap)=observed(||pages.birth(denied).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}let(step,heap)=observed(||pages.birth(grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,0));born+=heap.requested_bytes;}
 for n in 0..256{pages[n]=Some(n as u64)}assert_eq!(pages.iter().flatten().copied().collect::<Vec<_>>(),(0..256).collect::<Vec<u64>>());for n in 0..256{pages[n]=None}
 let mut released=0;while !pages.empty(){let extent=pages.release_bytes();let(step,heap)=observed(||pages.release(RetainedCloneGrant{maximum_release_bytes:extent-1,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let(step,heap)=observed(||pages.release(grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,step.progress().released_bytes));released+=heap.released_bytes;}assert_eq!(born,released);let(_,heap)=observed(||drop(pages));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] Original fixed pages birth/release={born} denied0 ordered256 terminalDrop0");
}
#[test]
fn original_first_party_scene_preserves_unfunded_children_and_exact_paid_release(){
 let fixture=law();let grant:RetainedCloneGrant=serde_json::from_value(fixture["grant"].clone()).unwrap();
 let mut oracle=kurbo::BezPath::new();oracle.move_to((0.0,0.0));oracle.line_to((2.0,3.0));oracle.line_to((5.0,7.0));let expected=kurbo::Shape::bounding_box(&oracle);assert_eq!((expected.x0,expected.y0,expected.x1,expected.y1),(0.0,0.0,5.0,7.0));
 let ((mut scene,mut close),heap)=observed(||{
  let mut path=BezPath::new();path.move_to((0.0,0.0));path.line_to((2.0,3.0));path.line_to((5.0,7.0));
  let mut scene=Scene::new();scene.fill(FillRule::NonZero,Affine::IDENTITY,Color::from_rgba8(255,0,0,255),None,&path);
  let mut stroke=Stroke::new(2.0);stroke.set_dash_pattern(vec![2.0,4.0,8.0]);scene.stroke(&stroke,Affine::IDENTITY,Color::from_rgba8(0,0,255,255),None,&path);
  let image=RasterImage::rgba8(1,1,std::sync::Arc::new(vec![255,0,0,255]));scene.draw_image(&image,Affine::IDENTITY);
  (scene,CanvasSceneRetirement::default())
 });assert_eq!(scene.command_len(),3);let mut born=heap.requested_bytes;let mut released=heap.released_bytes;
 for turn in 0..100000{if close.terminal_is_empty(&scene){break}let demand=close.next_demands(&scene,grant.maximum_copy_bytes).unwrap();
  let mut denied=vec![RetainedCloneGrant{maximum_items:0,..grant}];if demand.capacity_bytes!=0{denied.push(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant})}if demand.release_bytes!=0{denied.push(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant})}if demand.copy_bytes!=0{denied.push(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant})}
  for grant in denied{let(step,heap)=observed(||close.close_step(&mut scene,grant).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
  let(step,heap)=observed(||close.close_step(&mut scene,grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes),"physical receipt turn {turn}");born+=heap.requested_bytes;released+=heap.released_bytes;assert!(turn<99999);
 }assert!(close.terminal_is_empty(&scene));assert_eq!(born,released);let(_,heap)=observed(||{drop(close);drop(scene)});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] Original Canvas first-party scene physical birth/free={born} deniedAllAxes0 terminalDrop0");
}
