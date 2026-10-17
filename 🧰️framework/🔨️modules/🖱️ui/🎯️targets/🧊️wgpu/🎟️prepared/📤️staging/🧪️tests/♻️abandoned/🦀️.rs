use super::*;
use super::tests::{atlas_fixture_authority, atlas_fixture_owner, atlas_fixture_page, prepared_process_guard};

#[test]
fn prepared_original_abandoned_atlas_reports_full_physical_grants() {
    let _guard = prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🧪️tests/♻️physical-job-close/🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["prepared"];
    let recipe = &law["atlas"];
    for copy in fixture["copyGrants"].as_array().unwrap() {
        let copy = copy.as_u64().unwrap() as usize;
        let source = vec![91; recipe["byteLength"].as_u64().unwrap() as usize];
        let mut atlas = atlas_fixture_owner(recipe["width"].as_u64().unwrap() as u32, recipe["height"].as_u64().unwrap() as u32, recipe["channels"].as_u64().unwrap() as u8, source.len()).unwrap();
        while atlas.next_row() < atlas.height() { let row = atlas.next_row(); atlas_fixture_page(&mut atlas, &source, row).unwrap(); }
        let slot = usize::from(atlas.abandonment_slot);
        let expected = atlas.len() * PREPARED_ATLAS_PAGE_BYTES + size_of::<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>() + size_of::<PreparedAtlasAbandonment>();
        drop(atlas);
        let original = PREPARED_ATLAS_ABANDONMENT_OWNER[slot].load(Ordering::Acquire);
        assert!(!original.is_null());
        let grant = RetainedCloneGrant { maximum_copy_bytes: copy, maximum_items: 0, ..atlas_fixture_authority().retirement };
        let (step, births, frees) = measured(|| PreparedAtlasPages::close_abandoned_step(grant));
        assert!(matches!(step, Close::Pending { progress } if progress == Default::default()));
        assert_eq!((births, frees), (0, 0));
        assert_eq!(PREPARED_ATLAS_ABANDONMENT_OWNER[slot].load(Ordering::Acquire), original);
        let mut released = 0;
        let mut complete = false;
        for _ in 0..law["maximumTurns"].as_u64().unwrap() {
            let grant = atlas_fixture_authority().retirement;
            let (step, births, frees) = measured(|| PreparedAtlasPages::close_abandoned_step(grant));
            let progress = match step { Close::Pending { progress } | Close::Complete { progress } => progress, step => panic!("original abandoned atlas failed: {step:?}") };
            assert!(progress.fits(grant));
            assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (births, frees));
            released += frees;
            if matches!(step, Close::Complete { .. }) { complete = true; break; }
        }
        assert!(complete);
        assert_eq!(released, expected);
        assert!(PREPARED_ATLAS_ABANDONMENT_OWNER[slot].load(Ordering::Acquire).is_null());
        assert_eq!(PREPARED_ATLAS_ABANDONMENT_STATE[slot].load(Ordering::Acquire), 0);
    }
}
