//! 🧪️ Neutral paint samples and immutable preparation boundaries.
use semio_framework_test_fixtures::fixture_text;

use crate::{FillStyle,GradientStop};
use super::*;

#[test]
fn shared_fill_samples() {
    let cases:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🎨️fill/🎨️sampling/🔣️.json")).unwrap();
    let mut count=0;
    for case in cases.as_array().unwrap() {
        let fill:FillStyle=serde_json::from_value(case["fill"].clone()).unwrap();
        let mut wallet=finite_fill_wallet();let prepared=finish_preparation(PreparedFill::new(fill),&mut wallet);
        if case["error"]==true {let(error,job)=prepared.err().unwrap_or_else(||panic!("{} must fail",case["name"]));assert!(!error.is_empty());wallet.close_owned(job.into_retirement());continue;}
        let paint=prepared.unwrap_or_else(|(error,_)|panic!("{error}"));
        for sample in case["samples"].as_array().unwrap() {
            let point=serde_json::from_value(sample["point"].clone()).unwrap();
            let expected:[f64;4]=serde_json::from_value(sample["color"].clone()).unwrap();
            let actual=paint.sample(point).unwrap();
            for index in 0..4 {assert!((actual[index]-expected[index]).abs()<1e-12,"{}: {actual:?} != {expected:?}",case["name"]);}
            count+=1;
        }
        wallet.close_owned(paint.into_retirement());
    }
    assert!(count>0);
}

#[test]
fn preparation_owns_stops_and_rejects_invalid_numbers() {
    let mut fill=FillStyle::LinearGradient {x1:0.0,y1:0.0,x2:100.0,y2:0.0,stops:vec![GradientStop {offset:0.0,color:[1.0,0.0,0.0,1.0]},GradientStop {offset:1.0,color:[0.0,0.0,1.0,1.0]}].into()};
    let mut wallet=finite_fill_wallet();let prepared=finish_preparation(PreparedFill::new(fill.clone()),&mut wallet).unwrap_or_else(|(error,_)|panic!("{error}"));
    if let FillStyle::LinearGradient {stops,..}=&mut fill {stops[0].color[0]=0.0;}
    assert_eq!(prepared.sample([0.0,0.0]).unwrap(),[1.0,0.0,0.0,1.0]);
    for value in [f64::NAN,f64::INFINITY,f64::NEG_INFINITY] {
        assert!(prepared.sample([value,0.0]).is_err());
        let(error,job)=finish_preparation(PreparedFill::new(FillStyle::Solid {color:[0.0,0.0,0.0,value]}),&mut wallet).err().unwrap();assert!(!error.is_empty());wallet.close_owned(job.into_retirement());
        let(error,job)=finish_preparation(PreparedFill::new(FillStyle::RadialGradient {cx:value,cy:0.0,r:1.0,stops:vec![].into()}),&mut wallet).err().unwrap();assert!(!error.is_empty());wallet.close_owned(job.into_retirement());
    }
    wallet.close_owned(prepared.into_retirement());
}

#[test]
fn prepared_paint_retirement_drains_actual_stop_owners_and_composes_the_real_ramp() {
    let sources:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🎨️fill/🎨️sampling/🔣️.json")).unwrap();let rows:serde_json::Value=serde_json::from_str(fixture_text!("../../../🧪️tests/🧫️fixtures/🎨️fill/🎨️sampling/🧹️retirement/🔣️.json")).unwrap();
    for row in rows.as_array().unwrap(){for grant in [1,7,4096]{
        let source=sources.as_array().unwrap().iter().find(|source|source["name"]==row["source"]).unwrap();let mut fill:FillStyle=serde_json::from_value(source["fill"].clone()).unwrap();
        if let Some(repeat)=row["repeat"].as_u64(){match &mut fill {FillStyle::LinearGradient{stops,..}|FillStyle::RadialGradient{stops,..}=>*stops=vec![stops[0].clone();repeat as usize].into(),_=>panic!("gradient required")}}
        let mut wallet=finite_fill_wallet();let paint=finish_preparation(PreparedFill::new(fill.clone()),&mut wallet).unwrap_or_else(|(error,_)|panic!("{error}"));let samples:Vec<[f64;4]>=source["samples"].as_array().unwrap().iter().map(|sample|paint.sample(serde_json::from_value(sample["point"].clone()).unwrap()).unwrap()).collect();
        let mut owner=paint.into_retirement();let(zero,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.close_step(Default::default()).unwrap());assert_eq!(zero.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut work=0;
        for _ in 0..5000{if owner.terminal_is_empty(){break}let before=work;for _ in 0..grant{if owner.terminal_is_empty(){break}let d=fill_close_demand(&owner);let p=wallet.close_turn(d,|g|owner.close_step(g)).unwrap().progress();assert_ne!(p,Default::default());work+=p.copied_items as u64;}assert!(work>before&&work-before<=grant as u64);}
        assert!(work>0);assert!(owner.terminal_is_empty());assert!(owner.owner.original().is_none());let stable=owner.close_step(Default::default()).unwrap();assert!(matches!(stable,semio_framework_value::RetainedCloneStep::Complete(_)));assert_eq!(stable.progress(),Default::default());wallet.close_owned(owner);
        for (sample,actual) in source["samples"].as_array().unwrap().iter().zip(samples){let expected:[f64;4]=serde_json::from_value(sample["color"].clone()).unwrap();for c in 0..4{assert!((actual[c]-expected[c]).abs()<1e-12);}}
        if matches!(fill,FillStyle::LinearGradient{..}|FillStyle::RadialGradient{..}){
            let mut paint=finish_preparation(PreparedFill::new(fill),&mut wallet).unwrap_or_else(|(error,_)|panic!("{error}"));let pointer=paint.ramp.stops.as_ptr();let ramp=std::mem::replace(&mut paint.ramp,GradientRamp{stops:Vec::new(),scratch:Vec::new()});wallet.close_owned(paint.into_retirement());let mut retired=ramp.into_retirement();assert_eq!(retired.owner.original().unwrap().stops.as_ptr(),pointer);let mut work=0;
            for _ in 0..5000{if retired.terminal_is_empty(){break}let before=work;for _ in 0..grant{if retired.terminal_is_empty(){break}let d=ramp_close_demand(&retired);let p=wallet.close_turn(d,|g|retired.close_step(g)).unwrap().progress();assert_ne!(p,Default::default());work+=p.copied_items as u64;}assert!(work-before<=grant as u64);}assert!(work>0);assert!(retired.terminal_is_empty());assert!(retired.owner.original().is_none());wallet.close_owned(retired);
        }
    }}
}


#[path="../../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/🎟️work/🧪️tests/🦀️.rs"] mod fill_wallet;
fn finite_fill_wallet()->fill_wallet::Wallet{let v:serde_json::Value=serde_json::from_str(include_str!("../../🎟️work/🧫️fixtures/🔣️.json")).unwrap();let stock=&v["stock"];let n=|key|stock[key].as_u64().unwrap()as usize;fill_wallet::Wallet::from_grant(semio_framework_value::RetainedCloneGrant{maximum_items:n("items"),maximum_copy_bytes:n("copy"),maximum_capacity_bytes:n("capacity"),maximum_release_bytes:n("release"),maximum_depth:n("depth")},v["maxTurns"].as_u64().unwrap()as usize)}
#[test]
fn owned_fill_preparation_is_funded_and_matches_tiny_skia(){
 use semio_framework_value::work::RetainedWorkLedger;use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let v:serde_json::Value=serde_json::from_str(include_str!("../../🎟️work/🧫️fixtures/🔣️.json")).unwrap();let width=v["width"].as_u64().unwrap()as u32;let stops:Vec<GradientStop>=serde_json::from_value(v["stops"].clone()).unwrap();let fill=FillStyle::LinearGradient{x1:0.,y1:0.,x2:width as f64,y2:0.,stops:stops.into()};let(mut job,h)=observe(||PreparedFill::new(fill));assert!(!h.overflowed);assert_eq!((h.requested_bytes,h.released_bytes),(0,0));let mut wallet=finite_fill_wallet();let mut done=false;
 for _ in 0..v["maxTurns"].as_u64().unwrap(){let d=job.next_work_demand();for axis in v["axes"].as_array().unwrap(){let axis=axis.as_str().unwrap();if match axis{"items"=>false,"copy"=>d.copy_bytes==0,"capacity"=>d.capacity_bytes==0,"release"=>d.release_bytes==0,"depth"=>d.depth==0,_=>unreachable!()}{continue}let mut g=wallet.remaining();g.maximum_items=1;match axis{"items"=>g.maximum_items=0,"copy"=>g.maximum_copy_bytes=d.copy_bytes-1,"capacity"=>g.maximum_capacity_bytes=d.capacity_bytes-1,"release"=>g.maximum_release_bytes=d.release_bytes-1,"depth"=>g.maximum_depth=d.depth-1,_=>unreachable!()}let mut ledger=RetainedWorkLedger::new(g);let before=job.progress();let(result,h)=observe(||job.advance(1,&mut ledger));let p=result.unwrap();assert_eq!(p,before);assert_eq!(job.last_turn_work(),0);assert_eq!(ledger.receipt(),Default::default());assert!(!h.overflowed);assert_eq!((h.requested_bytes,h.released_bytes),(0,0));}
  let p=wallet.turn(1,|work|job.advance(1,work)).unwrap();if p.done{done=true;break}assert!(job.last_turn_work()>0,"finite fill stock cannot progress");
 }assert!(done);let mut reference=tiny_skia::Pixmap::new(width,1).unwrap();let mut paint=tiny_skia::Paint::default();paint.shader=tiny_skia::LinearGradient::new(tiny_skia::Point::from_xy(0.,0.),tiny_skia::Point::from_xy(width as f32,0.),vec![tiny_skia::GradientStop::new(0.,tiny_skia::Color::from_rgba(0.,0.,1.,1.).unwrap()),tiny_skia::GradientStop::new(1.,tiny_skia::Color::from_rgba(1.,0.,0.,1.).unwrap())],tiny_skia::SpreadMode::Pad,tiny_skia::Transform::identity()).unwrap();let mut path=tiny_skia::PathBuilder::new();path.push_rect(tiny_skia::Rect::from_xywh(0.,0.,width as f32,1.).unwrap());reference.fill_path(&path.finish().unwrap(),&paint,tiny_skia::FillRule::Winding,tiny_skia::Transform::identity(),None);
 for (x,pixel)in reference.data().chunks_exact(4).enumerate(){let actual=job.sample([x as f64+0.5,0.5]).unwrap();for c in 0..4{assert!(((actual[c]*255.).round()as i16-i16::from(pixel[c])).abs()<=1);}}assert_eq!(job.sample([-1.,0.]).unwrap(),serde_json::from_value::<[f64;4]>(v["expectedStart"].clone()).unwrap());assert_eq!(job.sample([width as f64+1.,0.]).unwrap(),serde_json::from_value::<[f64;4]>(v["expectedEnd"].clone()).unwrap());let(owner,h)=observe(||job.into_retirement());assert_eq!((h.requested_bytes,h.released_bytes),(0,0));wallet.close_owned(owner);
}
#[test]
fn refused_fill_retains_owned_input_and_cached_failure_has_zero_effects(){
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let fill=FillStyle::LinearGradient{x1:0.,y1:0.,x2:1.,y2:0.,stops:vec![GradientStop{offset:f64::NAN,color:[0.,0.,0.,1.]}].into()};let mut job=PreparedFill::new(fill);let mut wallet=finite_fill_wallet();let mut failed=false;for _ in 0..100_000{if wallet.turn(1,|work|job.advance(1,work)).is_err(){assert_eq!(job.last_turn_work(),1);assert_eq!(job.last_turn_semantic_work(),1);failed=true;break}}assert!(failed);let mut zero=semio_framework_value::work::RetainedWorkLedger::new(Default::default());let(result,h)=observe(||job.advance(1,&mut zero));assert!(result.is_err());assert_eq!(job.last_turn_work(),0);assert_eq!(job.last_turn_semantic_work(),0);assert_eq!((h.requested_bytes,h.released_bytes),(0,0));wallet.close_owned(job.into_retirement());
}

fn finish_preparation(mut job:PreparedFill,wallet:&mut fill_wallet::Wallet)->Result<PreparedFill,(&'static str,PreparedFill)>{for _ in 0..100_000{match wallet.turn(1,|work|job.advance(1,work)){Ok(p)if p.done=>return Ok(job),Ok(_)=>assert!(job.last_turn_work()>0,"finite preparation stock exhausted"),Err(error)=>return Err((error,job))}}panic!("finite fill preparation did not finish")}
fn fill_close_demand(owner:&PreparedFillRetirement)->semio_framework_value::RetirementDemand{let copy_bytes=owner.next_copy_byte_demand().unwrap();let release_bytes=owner.next_release_byte_demand().unwrap();semio_framework_value::RetirementDemand{copy_bytes,capacity_bytes:owner.next_capacity_byte_demand(if copy_bytes>0{copy_bytes}else{release_bytes}).unwrap(),release_bytes,depth:owner.next_depth_demand().unwrap()}}
fn ramp_close_demand(owner:&GradientRampRetirement)->semio_framework_value::RetirementDemand{let copy_bytes=owner.next_copy_byte_demand().unwrap();let release_bytes=owner.next_release_byte_demand().unwrap();semio_framework_value::RetirementDemand{copy_bytes,capacity_bytes:owner.next_capacity_byte_demand(if copy_bytes>0{copy_bytes}else{release_bytes}).unwrap(),release_bytes,depth:owner.next_depth_demand().unwrap()}}
