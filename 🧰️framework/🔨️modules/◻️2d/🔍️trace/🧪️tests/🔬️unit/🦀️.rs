
use super::*;
#[path="../../../🛤️path/🎟️work/🧪️tests/🦀️.rs"]mod funding;
fn with_trace(width:u32,height:u32,mask:Vec<u8>,threshold:f64,epsilon:f64,inspect:impl FnOnce(Result<&[PathSegment],BitmapTraceError>)){let law:serde_json::Value=serde_json::from_str(include_str!("../../🎟️work/📐️.json")).unwrap();let stock=&law["stock"];let mut wallet=funding::Wallet::from_grant(semio_framework_value::RetainedCloneGrant{maximum_items:stock["items"].as_u64().unwrap()as usize,maximum_copy_bytes:stock["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:stock["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:stock["release"].as_u64().unwrap()as usize,maximum_depth:stock["depth"].as_u64().unwrap()as usize},law["maxTurns"].as_u64().unwrap()as usize);let mut job=BitmapTraceJob::new(BitmapTraceInput{width,height,mask,threshold,simplify_epsilon:epsilon,max_pixels:width.saturating_mul(height).max(1)as usize,max_edges:65536,max_segments:65536,max_work:1000000});let mut stopped=false;for _ in 0..law["maxTurns"].as_u64().unwrap(){match wallet.turn(1,|ledger|job.advance(1,ledger)){Ok(p)=>{if p.done{stopped=true;break;}assert!(job.last_turn_work()>0,"Trace finite stock blocked: {:?}",job.next_work_demand());},Err(_)=>{stopped=true;break;}}}assert!(stopped);inspect(job.result());wallet.close_owned(job);}


#[test]
fn traces_filled_square() {
    let width = 8_u32;
    let height = 8_u32;
    let mut mask = vec![0_u8; (width * height) as usize];
    for y in 2..6 {
        for x in 2..6 {
            mask[(y * width + x) as usize] = 255;
        }
    }
    with_trace(width,height,mask,0.5,0.5,|result|{let segments=result.unwrap();
    assert_eq!(segments.iter().filter(|segment| matches!(segment, PathSegment::Move { .. })).count(), 1);
    assert!(segments.len() <= 6, "a solid square must trace its boundary without interior scanlines");});
}

#[test]
fn traces_disjoint_regions_with_move_per_contour() {
    let width = 10_u32;
    let height = 10_u32;
    let mut mask = vec![0_u8; (width * height) as usize];
    for y in 1..3 {
        for x in 1..3 {
            mask[(y * width + x) as usize] = 255;
        }
    }
    for y in 6..8 {
        for x in 6..8 {
            mask[(y * width + x) as usize] = 255;
        }
    }
    with_trace(width,height,mask,0.5,0.5,|result|{let segments=result.unwrap();
    let moves = segments.iter().filter(|segment| matches!(segment, PathSegment::Move { .. })).count();
    assert!(moves >= 2, "each disjoint contour must start with its own move");});
}

#[test]
fn traces_corner_touching_regions_as_separate_contours() {
    let mask = [255_u8, 0, 0, 255];
    with_trace(2,2,mask.to_vec(),0.5,0.0,|result|{let segments=result.unwrap();
    assert_eq!(segments.iter().filter(|segment| matches!(segment, PathSegment::Move { .. })).count(), 2);});
}

#[test]
fn trace_bitmap_errors_on_zero_dimensions() {
    with_trace(0,5,Vec::new(),0.5,0.5,|result|{let err=result.unwrap_err();
    assert!(matches!(err, BitmapTraceError::Invalid(_)));});
}

#[test]
fn trace_bitmap_errors_on_short_buffer() {
    with_trace(4,4,vec![0_u8;2],0.5,0.5,|result|{let err=result.unwrap_err();
    assert!(matches!(err, BitmapTraceError::Invalid(message) if message.contains("expects")));});
}

#[test]
fn trace_bitmap_returns_empty_geometry_when_no_pixels_above_threshold() {
    let mask = vec![0_u8; 16];
    with_trace(4,4,mask,0.5,0.5,|result|assert!(result.unwrap().is_empty()));
}

#[test]
fn trace_bitmap_rejects_invalid_parameters() {
    with_trace(1,1,vec![255],f64::NAN,0.0,|result|assert!(result.is_err()));
    with_trace(1,1,vec![255],0.5,-1.0,|result|assert!(result.is_err()));
}

#[test]
fn perpendicular_distance_handles_degenerate_zero_length_line() {
    let dist = distance([3.0, 4.0], [0.0, 0.0], [0.0, 0.0]);
    assert!((dist - 5.0).abs() < 1e-9);
}
