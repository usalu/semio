//! 🧫️ Shared exact coverage vectors and interruption contracts for native export.
use super::*;
use crate::retirement::tests::{physical_wallet,funded_turn,funded_close_owned,observed,Wallet,test_wallet};

fn fixtures() -> serde_json::Value {serde_json::from_str(crate::text("🖊️coverage/🧪️tests/🧫️fixtures/🔣️.json")).unwrap()}
fn input(value:&serde_json::Value)->CoverageInput {
    let v=&value["input"];
    CoverageInput {width:v["width"].as_u64().unwrap() as u32,height:v["height"].as_u64().unwrap() as u32,
        transform:std::array::from_fn(|at|v["transform"][at].as_f64().unwrap()),
        rule:if v["rule"]=="evenodd" {CoverageRule::EvenOdd} else {CoverageRule::NonZero},
        contours:v["contours"].as_array().unwrap().iter().map(|row|row.as_array().unwrap().iter().map(|p|[p[0].as_f64().unwrap(),p[1].as_f64().unwrap()]).collect()).collect()}
}
#[test]
fn shared_exact_area_vectors_are_budget_independent() {
    for row in fixtures().as_array().unwrap() {
        let expected:Vec<u8>=row["expected"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect();
        for grant in [1,7,4096] {
            let mut wallet=physical_wallet();let mut job=CoverageJob::new(input(row));let mut work=0;let mut done=false;
            for _ in 0..100_000 {let progress=funded_turn(&mut wallet,grant,|ledger|job.advance(grant,ledger)).unwrap();assert!(progress.work-work<=grant as u64);work=progress.work;if progress.done {done=true;break;}}
            assert!(done,"{} failed to complete",row["name"]);assert_eq!(job.result().unwrap().coverage,expected,"{}",row["name"]);
            let(close,output)=job.into_retirement();let output=output.unwrap();assert_eq!(output.coverage,expected);funded_close_owned(close,&mut wallet);funded_close_owned(output,&mut wallet);
        }
    }
}
#[test]
fn cancelled_preparation_and_sweep_never_publish_partial_coverage() {
    let rows=fixtures();
    for steps in [0,1,20,60] {
        let mut wallet=physical_wallet();let mut job=CoverageJob::new(input(&rows[3]));assert_eq!(job.result(),Err(PixelEditError::Incomplete));
        for _ in 0..steps {funded_turn(&mut wallet,1,|work|job.advance(1,work)).unwrap();}job.cancel();
        assert_eq!(funded_turn(&mut wallet,1,|work|job.advance(1,work)),Err(PixelEditError::Cancelled));assert_eq!(job.result(),Err(PixelEditError::Cancelled));let(close,output)=job.into_retirement();assert!(output.is_none());funded_close_owned(close,&mut wallet);
    }
}
#[test]
fn malformed_input_and_grants_are_rejected() {
    let rows=fixtures();let base=input(&rows[0]);
    let mut bad=base.clone();bad.width=0;reject_owned_input(bad);
    let mut bad=base.clone();bad.width=16384;bad.height=16384;reject_owned_input(bad);
    let mut bad=base.clone();bad.transform[4]=f64::INFINITY;reject_owned_input(bad);
    let mut bad=base.clone();bad.contours[0][0][0]=f64::NAN;reject_owned_input(bad);
    let mut wallet=physical_wallet();let mut job=CoverageJob::new(base);assert!(funded_turn(&mut wallet,1,|work|job.advance(0,work)).is_err());funded_close_owned(job,&mut wallet);
}

fn reject_owned_input(input:CoverageInput){let mut wallet=physical_wallet();let mut job=CoverageJob::new(input);let mut failed=false;for _ in 0..100_000{if funded_turn(&mut wallet,1,|work|job.advance(1,work)).is_err(){failed=true;break}}assert!(failed);assert!(job.result().is_err());funded_close_owned(job,&mut wallet);}
fn close_demand(owner:&CoverageRetirement)->semio_framework_value::RetirementDemand{let copy_bytes=owner.next_copy_byte_demand().unwrap();let release_bytes=owner.next_release_byte_demand().unwrap();semio_framework_value::RetirementDemand{copy_bytes,capacity_bytes:owner.next_capacity_byte_demand(if copy_bytes>0{copy_bytes}else{release_bytes}).unwrap(),release_bytes,depth:owner.next_depth_demand().unwrap()}}
fn close_turn(owner:&mut CoverageRetirement,wallet:&mut Wallet)->semio_framework_value::RetainedCloneStep{let d=close_demand(owner);wallet.close_turn_observed(d,|grant|{let(result,(allocated,released))=observed(||owner.close_step(grant));(result,test_wallet::HeapObservation{requested_bytes:allocated,released_bytes:released,overflowed:false})}).unwrap()}

#[test]
fn coverage_retirement_transfers_actual_masks_and_drains_private_sweep_owners() {
    let sources=fixtures();let cases:serde_json::Value=serde_json::from_str(crate::text("🖊️coverage/🧪️tests/🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
    for row in cases.as_array().unwrap() {for grant in [1,7,4096] {
        let source=sources.as_array().unwrap().iter().find(|source|source["name"]==row["source"]).unwrap();let mut value=input(source);let stop=row["stop"].as_str().unwrap();
        if stop=="failure" {value.contours[0][0][0]=1_000_000_001.0;}
        let mut wallet=physical_wallet();let mut job=CoverageJob::new(value);
        if stop=="failure" {let mut failed=false;for _ in 0..200_000{if funded_turn(&mut wallet,1,|work|job.advance(1,work)).is_err(){failed=true;break}}assert!(failed);}
        else if stop=="preparing" {funded_turn(&mut wallet,1,|work|job.advance(1,work)).unwrap();}
        else if stop!="fresh" {
            let stage=match stop {"events"|"cancelled"=>Stage::Events,"apply"=>Stage::Apply,"copy"=>Stage::Copy,"sort"=>Stage::Sort,"cross"=>Stage::Cross,"wind"=>Stage::Wind,"span"=>Stage::Span,"finish"=>Stage::Finish,"write"=>Stage::Write,"done"=>Stage::Done,_=>panic!("unknown stop")};
            for _ in 0..200_000 {if job.stage==stage {break;}funded_turn(&mut wallet,1,|work|job.advance(1,work)).unwrap();}assert!(job.stage==stage,"{}",row["name"]);
        }
        let expected:Vec<u8>=source["expected"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect();
        let published=row["publishedBeforeCancel"].as_bool().unwrap_or(false);if published {assert_eq!(job.result().unwrap().coverage,expected);}
        let mask=job.mask.coverage.as_ptr();let owned_contours=job.input.contours.len();
        if stop=="cancelled"||published||row["cancelBeforeTransfer"].as_bool().unwrap_or(false) {job.cancel();assert_eq!(job.mask.coverage.as_ptr(),mask);assert_eq!(job.input.contours.len(),owned_contours);}
        let has_output=row["output"].as_bool().unwrap();let (mut retired,output)=job.into_retirement();assert_eq!(output.is_some(),has_output);
        if let Some(output)=&output {assert_eq!(output.coverage.as_ptr(),mask);assert_eq!(output.coverage,expected);}
        let(zero,heap)=observed(||retired.close_step(Default::default()).unwrap());assert_eq!(zero.progress(),Default::default());assert_eq!(heap,(0,0));
        let mut work=0;
        for _ in 0..300_000 {
            if retired.terminal_is_empty(){break}let owners=retired.owner.original().map_or(0,|job|job.input.contours.len());let before=work;
            for _ in 0..grant{if retired.terminal_is_empty(){break}let progress=close_turn(&mut retired,&mut wallet).progress();assert_ne!(progress,Default::default());work+=progress.copied_items as u64;}
            assert!(work>=before&&work-before<=grant as u64);let remaining=retired.owner.original().map_or(0,|job|job.input.contours.len());assert!(remaining<=owners);
        }
        assert!(work>0);assert!(retired.terminal_is_empty());assert!(retired.owner.original().is_none());
        let(stable,heap)=observed(||retired.close_step(Default::default()).unwrap());assert!(matches!(stable,semio_framework_value::RetainedCloneStep::Complete(_)));assert_eq!(stable.progress(),Default::default());assert_eq!(heap,(0,0));
        funded_close_owned(retired,&mut wallet);if let Some(output)=output{funded_close_owned(output,&mut wallet);}
    }}
}

fn funding_fixture()->serde_json::Value{serde_json::from_str(include_str!("../🎟️work/🧫️fixtures/🔣️.json")).unwrap()}
fn funding_stock(v:&serde_json::Value)->semio_framework_value::RetainedCloneGrant{let v=&v["stock"];semio_framework_value::RetainedCloneGrant{maximum_items:v["items"].as_u64().unwrap()as usize,maximum_copy_bytes:v["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:v["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:v["release"].as_u64().unwrap()as usize,maximum_depth:v["depth"].as_u64().unwrap()as usize}}
fn funding_rectangle(v:&serde_json::Value)->CoverageInput{CoverageInput{width:v["width"].as_u64().unwrap()as u32,height:v["height"].as_u64().unwrap()as u32,transform:[1.,0.,0.,1.,0.,0.],rule:CoverageRule::NonZero,contours:vec![vec![[0.,0.],[8.,0.],[8.,8.],[0.,8.]]]}}
#[test]
fn coverage_actual_storage_uses_one_finite_stock_and_matches_independent_tiny_skia(){
 use semio_framework_value::work::RetainedWorkLedger;
 let v=funding_fixture();let original=funding_rectangle(&v);let original_ptr=original.contours.as_ptr();let(mut job,h)=observed(||CoverageJob::new(original));assert_eq!(h,(0,0));assert_eq!(job.input.contours.as_ptr(),original_ptr);let mut remaining=funding_stock(&v);let mut done=false;let mut saw_allocation=false;
 for _ in 0..v["maximumTurns"].as_u64().unwrap(){
  let d=job.next_work_demand();for axis in v["axes"].as_array().unwrap(){let axis=axis.as_str().unwrap();if match axis{"items"=>false,"copy"=>d.copy_bytes==0,"capacity"=>d.capacity_bytes==0,"release"=>d.release_bytes==0,"depth"=>d.depth==0,_=>unreachable!()}{continue}let mut g=remaining;g.maximum_items=1;match axis{"items"=>g.maximum_items=0,"copy"=>g.maximum_copy_bytes=d.copy_bytes-1,"capacity"=>g.maximum_capacity_bytes=d.capacity_bytes-1,"release"=>g.maximum_release_bytes=d.release_bytes-1,"depth"=>g.maximum_depth=d.depth-1,_=>unreachable!()}let mut work=RetainedWorkLedger::new(g);let before=job.work;let(result,h)=observed(||job.advance(1,&mut work));let p=result.unwrap();assert_eq!(p.work,before);assert_eq!(job.last_turn_work(),0);assert_eq!(work.receipt(),Default::default());assert_eq!(h,(0,0));assert_eq!(job.input.contours.as_ptr(),original_ptr);}
  let mut turn=remaining;turn.maximum_items=turn.maximum_items.min(1);let mut work=RetainedWorkLedger::new(turn);let(result,h)=observed(||job.advance(1,&mut work));let p=result.unwrap();let used=work.receipt();assert!(used.fits(turn));assert_eq!(h,(used.retained_capacity_bytes,used.released_bytes));saw_allocation|=h.0>0;remaining.maximum_items-=used.copied_items;remaining.maximum_copy_bytes-=used.copied_bytes;remaining.maximum_capacity_bytes-=used.retained_capacity_bytes;remaining.maximum_release_bytes-=used.released_bytes;if p.done{done=true;break}assert!(used!=Default::default()||job.last_turn_work()>0,"finite coverage stock exhausted at {:?}",job.next_work_demand());
 }assert!(done&&saw_allocation);let expected:Vec<u8>=v["expected"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect();assert_eq!(job.result().unwrap().coverage,expected);let mut reference=tiny_skia::Pixmap::new(8,8).unwrap();let mut path=tiny_skia::PathBuilder::new();path.push_rect(tiny_skia::Rect::from_xywh(0.,0.,8.,8.).unwrap());let mut paint=tiny_skia::Paint::default();paint.set_color_rgba8(255,255,255,255);reference.fill_path(&path.finish().unwrap(),&paint,tiny_skia::FillRule::Winding,tiny_skia::Transform::identity(),None);let alpha:Vec<u8>=reference.data().chunks_exact(4).map(|p|p[3]).collect();assert_eq!(job.result().unwrap().coverage,alpha);let pointer=job.result().unwrap().coverage.as_ptr();let((close,mask),h)=observed(||job.into_retirement());assert_eq!(h,(0,0));let mask=mask.unwrap();assert_eq!(mask.coverage.as_ptr(),pointer);let mut wallet=Wallet::from_grant(remaining,v["maximumTurns"].as_u64().unwrap()as usize);funded_close_owned(close,&mut wallet);funded_close_owned(mask,&mut wallet);
}
#[test]
fn coverage_refused_owned_input_and_cached_error_preserve_custody_without_heap(){
 use semio_framework_value::work::RetainedWorkLedger;
 let v=funding_fixture();let mut input=funding_rectangle(&v);input.width=0;let pointer=input.contours.as_ptr();let(mut job,h)=observed(||CoverageJob::new(input));assert_eq!(h,(0,0));assert_eq!(job.input.contours.as_ptr(),pointer);let mut wallet=physical_wallet();assert!(funded_turn(&mut wallet,1,|work|job.advance(1,work)).is_err());assert_eq!(job.last_turn_work(),1);assert_eq!(job.last_turn_semantic_work(),1);let mut zero=RetainedWorkLedger::new(Default::default());let(e,h)=observed(||job.advance(1,&mut zero));assert!(e.is_err());assert_eq!(h,(0,0));assert_eq!(job.last_turn_work(),0);let((close,output),h)=observed(||job.into_retirement());assert_eq!(h,(0,0));assert!(output.is_none());funded_close_owned(close,&mut wallet);
}
