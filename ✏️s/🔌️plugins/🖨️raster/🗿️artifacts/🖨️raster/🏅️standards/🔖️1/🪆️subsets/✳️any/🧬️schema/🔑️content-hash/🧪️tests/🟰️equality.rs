//! 🟰️ Exact decoded-image comparison uses the same canonical fields across finite grants.
use super::*;

#[test]
fn canonical_image_equality_matches_independent_neutral_vectors() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🟰️equality.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap(){
        let left:SemioImageSnapshot=semio_framework_pack_json::from_json_str(&case["left"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        let right:SemioImageSnapshot=semio_framework_pack_json::from_json_str(&case["right"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
        for grant in fixture["grants"].as_array().unwrap(){
            let budget=grant.as_u64().unwrap() as usize;let mut work=RasterImageContentEquality::new();
            for turn in 0..100_000{
                let progress=work.advance(&left,&right,budget).unwrap();assert!(progress.processed_bytes<=budget);assert!(turn<99_999);
                if progress.done{assert_eq!(progress.equal,case["equal"].as_bool().unwrap(),"{}",case["name"]);assert_eq!(work.advance(&left,&right,budget).unwrap().processed_bytes,0);break;}
            }
        }
    }
}

#[test]
fn exact_image_equality_binds_both_original_roots_and_refuses_invalid_grants() {
    let left=semio_image_from_rgba8(1,1,vec![255;4]);let right=left.clone();let other=right.clone();
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🟰️equality.json")).unwrap();
    for budget in fixture["invalidGrants"].as_array().unwrap(){assert!(RasterImageContentEquality::new().advance(&left,&right,budget.as_u64().unwrap() as usize).is_err());}
    let mut work=RasterImageContentEquality::new();assert!(!work.advance(&left,&right,2).unwrap().done);
    assert!(work.advance(&left,&other,2).is_err());assert!(work.advance(&other,&right,2).is_err());
}

#[test]
fn exact_image_equality_distinguishes_absent_and_empty_profiles() {
    let left=semio_image_from_rgba8(1,1,vec![255;4]);let mut right=left.clone();right.icc=Some(Vec::new());let mut work=RasterImageContentEquality::new();
    for turn in 0..1000{let progress=work.advance(&left,&right,3).unwrap();assert!(turn<999);if progress.done{assert!(!progress.equal);return;}}
}

#[test]
fn empty_canonical_fields_and_large_equal_prefixes_still_yield() {
    use semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageMetadataEntry;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🟰️equality.json")).unwrap();
    let entries=fixture["emptyMetadataEntries"].as_u64().unwrap() as usize;
    let mut left=semio_image_from_rgba8(16_384,1,vec![7;65_536]);
    left.metadata=(0..entries).map(|_|SemioImageMetadataEntry {key:String::new(),value:String::new()}).collect();
    let right=left.clone();let mut work=RasterImageContentEquality::new();let mut turns=0;
    loop{let progress=work.advance(&left,&right,32768).unwrap();assert!(progress.processed_bytes<=32768);turns+=1;assert!(turns<100);if progress.done{assert!(progress.equal);break;}}
    assert!(turns>=8);let mut changed=right.clone();changed.frames[0].rgba8[65_535]=8;let mut work=RasterImageContentEquality::new();let mut turns=0;
    loop{let progress=work.advance(&left,&changed,32768).unwrap();turns+=1;if progress.done{assert!(!progress.equal);break;}}
    assert!(turns>=4);
}
