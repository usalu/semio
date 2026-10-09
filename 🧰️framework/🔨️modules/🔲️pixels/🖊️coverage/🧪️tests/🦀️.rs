//! 🧫️ Shared exact coverage vectors and interruption contracts for native export.
use super::*;

fn fixtures() -> serde_json::Value {serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap()}
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
            let mut job=CoverageJob::new(input(row)).unwrap();let mut work=0;let mut done=false;
            for _ in 0..100_000 {let progress=job.advance(grant).unwrap();assert!(progress.work-work<=grant as u64);work=progress.work;if progress.done {done=true;break;}}
            assert!(done,"{} failed to complete",row["name"]);assert_eq!(job.result().unwrap().coverage,expected,"{}",row["name"]);
            assert_eq!(job.into_result().unwrap().coverage,expected);
        }
    }
}
#[test]
fn cancelled_preparation_and_sweep_never_publish_partial_coverage() {
    let rows=fixtures();
    for steps in [0,1,20,60] {
        let mut job=CoverageJob::new(input(&rows[3])).unwrap();assert_eq!(job.result(),Err(PixelEditError::Incomplete));
        for _ in 0..steps {job.advance(1).unwrap();}job.cancel();
        assert_eq!(job.advance(1),Err(PixelEditError::Cancelled));assert_eq!(job.result(),Err(PixelEditError::Cancelled));
    }
}
#[test]
fn malformed_input_and_grants_are_rejected() {
    let rows=fixtures();let base=input(&rows[0]);
    let mut bad=base.clone();bad.width=0;assert!(CoverageJob::new(bad).is_err());
    let mut bad=base.clone();bad.width=16384;bad.height=16384;assert!(CoverageJob::new(bad).is_err());
    let mut bad=base.clone();bad.transform[4]=f64::INFINITY;assert!(CoverageJob::new(bad).is_err());
    let mut bad=base.clone();bad.contours[0][0][0]=f64::NAN;
    let mut job=CoverageJob::new(bad).unwrap();assert!(job.advance(100).is_err());assert!(job.result().is_err());
    let mut job=CoverageJob::new(base).unwrap();assert!(job.advance(0).is_err());
}

#[test]
fn coverage_retirement_transfers_actual_masks_and_drains_private_sweep_owners() {
    let sources=fixtures();let cases:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧹️retirement/🔣️.json")).unwrap();
    for row in cases.as_array().unwrap() {for grant in [1,7,4096] {
        let source=sources.as_array().unwrap().iter().find(|source|source["name"]==row["source"]).unwrap();let mut value=input(source);let stop=row["stop"].as_str().unwrap();
        if stop=="failure" {value.contours[0][0][0]=1_000_000_001.0;}
        let mut job=CoverageJob::new(value).unwrap();
        if stop=="failure" {assert!(job.advance(4096).is_err());}
        else if stop=="preparing" {job.advance(1).unwrap();}
        else if stop!="fresh" {
            let stage=match stop {"events"|"cancelled"=>Stage::Events,"apply"=>Stage::Apply,"copy"=>Stage::Copy,"sort"=>Stage::Sort,"cross"=>Stage::Cross,"wind"=>Stage::Wind,"span"=>Stage::Span,"finish"=>Stage::Finish,"write"=>Stage::Write,"done"=>Stage::Done,_=>panic!("unknown stop")};
            for _ in 0..200_000 {if job.stage==stage {break;}job.advance(1).unwrap();}assert!(job.stage==stage,"{}",row["name"]);
        }
        let expected:Vec<u8>=source["expected"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap() as u8).collect();
        let published=row["publishedBeforeCancel"].as_bool().unwrap_or(false);if published {assert_eq!(job.result().unwrap().coverage,expected);}
        let mask=job.mask.coverage.as_ptr();let owned_contours=job.input.contours.len();
        if stop=="cancelled"||published||row["cancelBeforeTransfer"].as_bool().unwrap_or(false) {job.cancel();assert_eq!(job.mask.coverage.as_ptr(),mask);assert_eq!(job.input.contours.len(),owned_contours);}
        let has_output=row["output"].as_bool().unwrap();let (mut retired,output)=job.into_retirement();assert_eq!(output.is_some(),has_output);
        if let Some(output)=&output {assert_eq!(output.coverage.as_ptr(),mask);assert_eq!(output.coverage,expected);}
        assert!(retired.advance(0).is_err());if usize::BITS>53 {assert!(retired.advance(usize::MAX).is_err());}
        let mut work=0;
        for _ in 0..300_000 {
            let owners=retired.owner.original().map_or(0,|job|job.input.contours.len());let progress=retired.advance(grant).unwrap();
            assert!(progress.work>=work&&progress.work-work<=grant as u64);assert_eq!(progress.phase,if progress.done {"complete"} else {"closing"});
            let remaining=retired.owner.original().map_or(0,|job|job.input.contours.len());assert!(remaining<=owners);
            work=progress.work;if progress.done {break;}
        }
        assert!(work>0);assert!(retired.terminal_is_empty());assert!(retired.owner.original().is_none());
        let stable=retired.advance(1).unwrap();assert!(stable.done);assert_eq!(stable.work,work);
        eprintln!("[DEBUG] Actual coverage retirement {}: grant {}, work {}, output {}",row["name"],grant,work,has_output);
    }}
}
