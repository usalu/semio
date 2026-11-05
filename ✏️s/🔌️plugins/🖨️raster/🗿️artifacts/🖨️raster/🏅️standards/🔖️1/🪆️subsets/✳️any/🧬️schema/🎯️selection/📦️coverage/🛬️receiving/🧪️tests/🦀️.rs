//! 🧪️ Flat original span arrays retain their wire order across packed page boundaries.
use super::*;
use semio_framework_plugin::artifact_app_laws::OriginalWorkFixture;

#[test]
fn original_flat_coverage_receives_packed_pages(){
    for count in [0,1,255,256,257,4097]{
        let rows=(0..count).map(|index|RasterSelectionSpan{start:index as u32,length:1,coverage:128}).collect::<Vec<_>>();
        let mut caller=OriginalWorkFixture::new();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:128};
        let packed=caller.receive_native::<_,RasterSelectionSpans>(rows,grant);
        assert_eq!(packed.len(),count);
        assert_eq!(packed.maximum_end(),count as u64);
        assert_eq!(packed.pages().len(),RasterSelectionSpans::page_count(count));
        assert!(packed.iter().enumerate().all(|(index,row)|row.start==index as u32&&row.length==1&&row.coverage==128));
        caller.retire_owned(packed,grant);
        caller.finish();
    }
}

#[test]
fn original_coverage_construction_proves_span_order_length_and_coverage(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../fixture.json")).unwrap();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:32768,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:128};
    for rows in fixture["invalidRows"].as_array().unwrap(){let rows=rows.as_array().unwrap().iter().map(|row|RasterSelectionSpan{start:row["start"].as_u64().unwrap()as u32,length:row["length"].as_u64().unwrap()as u32,coverage:row["coverage"].as_u64().unwrap()as u32}).collect::<Vec<_>>();let mut caller=OriginalWorkFixture::new();let error=caller.try_receive_native::<_,RasterSelectionSpans>(rows,grant).err().expect("invalid coverage must refuse before publication");caller.retire_owned(error,grant);caller.finish();}
    let mut caller=OriginalWorkFixture::new();let root=caller.receive_native::<_,RasterSelectionSpans>(vec![RasterSelectionSpan{start:0,length:1,coverage:0}],grant);assert_eq!(root.maximum_end(),1);assert_eq!(root[0].coverage,0);caller.retire_owned(root,grant);caller.finish();
}
