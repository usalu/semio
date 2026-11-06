//! 🧪️ Original output allocation and scalar work remain bounded while transposing the neutral stroke columns.
use super::*;
#[global_allocator]
static HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
use semio_plugin::artifact_app_laws::OriginalWorkFixture;

fn grant()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:128}}
#[test]
fn columns_preserve_serde_points_through_original_bounded_turns(){
    let fixture:semio_framework_value::serde_json::Value=semio_framework_value::serde_json::from_str(include_str!("../../fixture.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap(){let xs=case["xs"].as_array().unwrap().iter().map(|value|value.as_f64().unwrap()).collect::<Vec<_>>();let ys=case["ys"].as_array().unwrap().iter().map(|value|value.as_f64().unwrap()).collect::<Vec<_>>();let mut collector=StrokeColumns::default();let mut caller=OriginalWorkFixture::new();
        caller.turn(grant(),|cx|{let turn=collector.reserve(xs.len(),cx.retained_grant());semio_framework_job::receive_original_retained_work(cx,turn)?;Ok(())}).unwrap();
        loop{let before=collector.cursor;let mut done=false;caller.turn(grant(),|cx|{let turn=collector.advance(&xs,&ys,fixture["maximumWork"].as_u64().unwrap()as usize,RetainedCloneGrant{maximum_copy_bytes:fixture["maximumCopyBytes"].as_u64().unwrap()as usize,..cx.retained_grant()},&mut||true);assert!(turn.work<=fixture["maximumWork"].as_u64().unwrap());let result=turn.result.map(|(ready,p)|{done=ready;if ready{RetainedCloneStep::Complete(p)}else{RetainedCloneStep::Progress(p)}});semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result,work:turn.work})?;Ok(())}).unwrap();assert!(collector.cursor-before<=3);if done{break;}}
        let actual=semio_framework_value::serde_json::to_value(collector.points.iter().map(|point|[point.x,point.y]).collect::<Vec<_>>()).unwrap();assert_eq!(actual,case["expected"]);caller.retire_owned(collector.take_points(),grant());caller.finish();
    }
}

#[test]
fn refused_reservation_and_cancelled_prefix_keep_owned_points(){
    let mut collector=StrokeColumns::default();let denied=collector.reserve(4,RetainedCloneGrant{maximum_capacity_bytes:0,..grant()});assert_eq!(denied.work,0);assert_eq!(collector.points.capacity(),0);
    let mut caller=OriginalWorkFixture::new();caller.turn(grant(),|cx|{semio_framework_job::receive_original_retained_work(cx,collector.reserve(4,cx.retained_grant()))?;Ok(())}).unwrap();
    caller.turn(grant(),|cx|{let turn=collector.advance(&[0.0,1.0,2.0,3.0],&[0.0,1.0,2.0,3.0],2,cx.retained_grant(),&mut||true);semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result:turn.result.map(|(_,p)|RetainedCloneStep::Progress(p)),work:turn.work})?;Ok(())}).unwrap();assert_eq!(collector.points.len(),2);caller.cancel().cancel_now();caller.retire_owned(collector.take_points(),grant());caller.finish();
}

#[test]
fn full_schema_point_owner_is_reserved_once_and_copied_in_paid_pages(){
 let law:semio_framework_value::serde_json::Value=semio_framework_value::serde_json::from_str(include_str!("../../fixture.json")).unwrap();let count=law["maximumPoints"].as_u64().unwrap()as usize;let page=law["pageWork"].as_u64().unwrap()as usize;assert_eq!(count,RASTER_STROKE_MAXIMUM_POINTS);let demand=StrokeColumns::reserve_demands(count).unwrap();assert_eq!(demand.capacity_bytes,law["reservationBytes"].as_u64().unwrap()as usize);assert_eq!(demand.copy_bytes,size_of::<Vec<RasterStrokePoint>>());assert!(StrokeColumns::reserve_demands(count+1).is_err());let mut caller=OriginalWorkFixture::new();let mut owner=StrokeColumns::default();let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant(),|cx|{semio_framework_job::receive_original_retained_work(cx,owner.reserve(count,cx.retained_grant()))?;Ok(())}).unwrap());assert_eq!(heap.requested_bytes,demand.capacity_bytes);let xs=(0..count).map(|n|n as f64).collect::<Vec<_>>();let ys=vec![0.5;count];let mut turns=0;loop{let mut done=false;let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||caller.turn(grant(),|cx|{let turn=owner.advance(&xs,&ys,page,cx.retained_grant(),&mut||true);assert!(turn.work<=page as u64);let result=turn.result.map(|(ready,p)|{done=ready;assert!(p.copied_bytes<=page*size_of::<RasterStrokePoint>());RetainedCloneStep::Progress(p)});semio_framework_job::receive_original_retained_work(cx,RetainedWorkOutcome{result,work:turn.work})?;Ok(())}).unwrap());assert_eq!(heap.requested_bytes,0);turns+=1;if done{break;}}assert_eq!(turns,count.div_ceil(page));let expected=semio_framework_value::serde_json::to_value(xs.iter().zip(&ys).map(|(x,y)|[*x,*y]).collect::<Vec<_>>()).unwrap();assert_eq!(semio_framework_value::serde_json::to_value(owner.points.iter().map(|p|[p.x,p.y]).collect::<Vec<_>>()).unwrap(),expected);caller.retire_owned(owner.take_points(),grant());caller.finish();
}
