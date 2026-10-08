use super::*;
use crate::wgpu::host::physical_job_close_tests::measured;
use semio_framework_job::{InteractiveJob, InteractiveJobCloseStep as Close, RetainedCloneGrant};

#[test]
fn prepared_original_owner_closes_through_the_actual_job_full_grant_contract() {
    let _guard = prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["prepared"];
    for copy in fixture["copyGrants"].as_array().unwrap() {
        let input = PreparedRenderInput::new(law["sceneRevision"].as_u64().unwrap(), law["previewGeneration"].as_u64().unwrap(), DrawList::default(), None, 0.0);
        let mut job = PreparedRenderJob::new(input, 1);
        InteractiveJob::begin_close(&mut job);
        let mut complete = false;
        for _ in 0..law["maximumTurns"].as_u64().unwrap() {
            let copy = copy.as_u64().unwrap() as usize;
            let grant = RetainedCloneGrant {
                maximum_items: 1,
                maximum_copy_bytes: copy,
                maximum_capacity_bytes: job.next_close_capacity_byte_demand(copy).unwrap(),
                maximum_release_bytes: job.next_close_release_byte_demand().unwrap(),
                maximum_depth: job.next_close_depth_demand().unwrap(),
            };
            let (step, births, frees) = measured(|| InteractiveJob::close_step(&mut job, grant));
            let progress = match step {
                Close::Pending { progress } | Close::Complete { progress } => progress,
                Close::Blocked => panic!("requested prepared close was blocked"),
                Close::Refused(kind) => panic!("admitted prepared close refused: {kind:?}"),
            };
            assert!(progress.fits(grant));
            assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (births, frees));
            if matches!(step, Close::Complete { .. }) { complete = true; break; }
        }
        assert!(complete, "prepared original owner must physically finish within declared work");
        assert!(InteractiveJob::terminal_is_empty(&job));
        let (_, births, frees) = measured(|| drop(job));
        assert_eq!((births, frees), (0, 0));
    }
}

#[test]
fn prepared_original_atlas_owner_preserves_denials_and_reports_every_physical_release() {
    let _guard = prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["prepared"];
    let atlas = &law["atlas"];
    for copy in fixture["copyGrants"].as_array().unwrap() {
        let source = vec![91; atlas["byteLength"].as_u64().unwrap() as usize];
        let mut pages = PreparedAtlasPages::try_new(atlas["width"].as_u64().unwrap() as u32, atlas["height"].as_u64().unwrap() as u32, atlas["channels"].as_u64().unwrap() as u8, source.len()).unwrap();
        let mut row = 0;
        while row < pages.height() { pages.push_page(&source, row).unwrap(); row = pages.next_row(); }
        let original = pages.page(0).unwrap().0.as_ptr();
        let payload_bytes = pages.len() * atlas["pageBytes"].as_u64().unwrap() as usize;
        let atlas_release = payload_bytes + size_of::<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>() + size_of::<PreparedFixedPage<PreparedRenderUpload>>();
        let mut input = PreparedRenderInput::new(law["sceneRevision"].as_u64().unwrap(), law["previewGeneration"].as_u64().unwrap(), DrawList::default(), None, 0.0);
        let expected_release = atlas_release + input.draw.layers.capacity() * size_of::<DrawLayer>() + size_of::<PreparedFixedPage<RenderDirective>>();
        input.uploads.try_push(PreparedRenderUpload::GlyphAtlasPages { pixels: pages }).unwrap();
        let mut job = PreparedRenderJob::new(input, 1);
        assert!(job.commands.take().unwrap().terminal_is_empty());
        InteractiveJob::begin_close(&mut job);
        let copy = copy.as_u64().unwrap() as usize;
        let release = job.next_close_release_byte_demand().unwrap();
        assert_eq!(release, atlas["pageBytes"].as_u64().unwrap() as usize);
        for (items, released, depth) in [(0, release, 1), (1, release - 1, 1), (1, release, 0)] {
            let grant = RetainedCloneGrant { maximum_items: items, maximum_copy_bytes: copy, maximum_capacity_bytes: 0, maximum_release_bytes: released, maximum_depth: depth };
            let (step, births, frees) = measured(|| InteractiveJob::close_step(&mut job, grant));
            assert!(matches!(step, Close::Pending { progress } if progress == Default::default()) || matches!(step, Close::Refused(semio_framework_value::ValueRefusalKind::DepthLimit)));
            assert_eq!((births, frees), (0, 0));
            let PreparedRenderUpload::GlyphAtlasPages { pixels } = job.input.as_ref().unwrap().uploads.get(0).unwrap() else { panic!("original atlas owner changed") };
            assert_eq!(pixels.page(0).unwrap().0.as_ptr(), original);
            assert_eq!(pixels.page(0).unwrap().0, &source[..atlas["pageBytes"].as_u64().unwrap() as usize]);
        }
        let mut total_release = 0;
        let mut complete = false;
        for _ in 0..law["maximumTurns"].as_u64().unwrap() {
            let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_capacity_bytes: job.next_close_capacity_byte_demand(copy).unwrap(), maximum_release_bytes: job.next_close_release_byte_demand().unwrap(), maximum_depth: job.next_close_depth_demand().unwrap() };
            let (step, births, frees) = measured(|| InteractiveJob::close_step(&mut job, grant));
            let progress = match step { Close::Pending { progress } | Close::Complete { progress } => progress, step => panic!("funded atlas close failed: {step:?}") };
            assert!(progress.fits(grant));
            assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (births, frees));
            assert_eq!(progress.copied_bytes, 0);
            total_release += frees;
            if matches!(step, Close::Complete { .. }) { complete = true; break; }
        }
        assert!(complete && InteractiveJob::terminal_is_empty(&job));
        assert_eq!(total_release, expected_release);
        let (_, births, frees) = measured(|| drop(job));
        assert_eq!((births, frees), (0, 0));
        println!("[DEBUG] prepared original atlas copy={copy} physicalRelease={total_release} deniedIdentity=unchanged terminalDrop=0");
    }
}

#[test]
fn prepared_original_abandoned_owners_report_full_physical_grants() {
    let _guard = prepared_process_guard();
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let law = &fixture["prepared"];
    let recipe = &law["atlas"];
    for copy in fixture["copyGrants"].as_array().unwrap() {
        let copy = copy.as_u64().unwrap() as usize;
        let input = PreparedRenderInput::new(law["sceneRevision"].as_u64().unwrap(), law["previewGeneration"].as_u64().unwrap(), DrawList::default(), None, 0.0);
        let slot = usize::from(input.abandonment_slot);
        let expected = input.draw.layers.capacity() * size_of::<DrawLayer>() + size_of::<PreparedFixedPage<RenderDirective>>() + size_of::<PreparedRenderInput>();
        drop(input);
        let original = PREPARED_RENDER_INPUT_ABANDONMENT_OWNER[slot].load(Ordering::Acquire);
        assert!(!original.is_null());
        for grant in [RetainedCloneGrant::default()] {
            let (step, births, frees) = measured(|| PreparedRenderInput::close_abandoned_step(grant));
            assert!(matches!(step, Close::Pending { progress } if progress == Default::default()));
            assert_eq!((births, frees), (0, 0));
            assert_eq!(PREPARED_RENDER_INPUT_ABANDONMENT_OWNER[slot].load(Ordering::Acquire), original);
        }
        let mut released = 0;
        let mut complete = false;
        for _ in 0..law["maximumTurns"].as_u64().unwrap() {
            let grant = PreparedRenderInput::next_abandoned_close_demands(copy).unwrap();
            if grant.maximum_release_bytes != 0 {
                let original = PREPARED_RENDER_INPUT_ABANDONMENT_OWNER[slot].load(Ordering::Acquire);
                let denied = RetainedCloneGrant { maximum_release_bytes: grant.maximum_release_bytes - 1, ..grant };
                let (step, births, frees) = measured(|| PreparedRenderInput::close_abandoned_step(denied));
                assert!(matches!(step, Close::Pending { progress } if progress == Default::default()));
                assert_eq!((births, frees), (0, 0));
                assert_eq!(PREPARED_RENDER_INPUT_ABANDONMENT_OWNER[slot].load(Ordering::Acquire), original);
            }
            let (step, births, frees) = measured(|| PreparedRenderInput::close_abandoned_step(grant));
            let progress = match step { Close::Pending { progress } | Close::Complete { progress } => progress, step => panic!("original abandoned input failed: {step:?}") };
            assert!(progress.fits(grant));
            assert_eq!((progress.retained_capacity_bytes, progress.released_bytes), (births, frees));
            released += frees;
            if matches!(step, Close::Complete { .. }) { complete = true; break; }
        }
        assert!(complete);
        assert_eq!(released, expected);
        assert!(PREPARED_RENDER_INPUT_ABANDONMENT_OWNER[slot].load(Ordering::Acquire).is_null());
        assert_eq!(PREPARED_RENDER_INPUT_ABANDONMENT_STATE[slot].load(Ordering::Acquire), 0);

        let source = vec![91; recipe["byteLength"].as_u64().unwrap() as usize];
        let mut atlas = PreparedAtlasPages::try_new(recipe["width"].as_u64().unwrap() as u32, recipe["height"].as_u64().unwrap() as u32, recipe["channels"].as_u64().unwrap() as u8, source.len()).unwrap();
        while atlas.next_row() < atlas.height() { atlas.push_page(&source, atlas.next_row()).unwrap(); }
        let slot = usize::from(atlas.abandonment_slot);
        let expected = atlas.len() * PREPARED_ATLAS_PAGE_BYTES + size_of::<[Option<PreparedAtlasPage>; PREPARED_ATLAS_PAGE_CAPACITY]>() + size_of::<PreparedAtlasAbandonment>();
        drop(atlas);
        let original = PREPARED_ATLAS_ABANDONMENT_OWNER[slot].load(Ordering::Acquire);
        assert!(!original.is_null());
        let release = PreparedAtlasPages::next_abandoned_close_demands(copy).unwrap().maximum_release_bytes;
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: copy, maximum_release_bytes: release - 1, maximum_depth: 1, ..Default::default() };
        let (step, births, frees) = measured(|| PreparedAtlasPages::close_abandoned_step(grant));
        assert!(matches!(step, Close::Pending { progress } if progress == Default::default()));
        assert_eq!((births, frees), (0, 0));
        assert_eq!(PREPARED_ATLAS_ABANDONMENT_OWNER[slot].load(Ordering::Acquire), original);
        let mut released = 0;
        let mut complete = false;
        for _ in 0..law["maximumTurns"].as_u64().unwrap() {
            let grant = PreparedAtlasPages::next_abandoned_close_demands(copy).unwrap();
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
        println!("[DEBUG] original abandoned input/atlas copy={copy} exactBoxAndBackingRelease=true custodyRetained=true");
    }
}
