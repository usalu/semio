//! 🎟️ WFC's selected lifecycle grants can release every backing they already admit.
use semio_framework_plugin::__value::retirement::admit_owned_retirement;
use semio_framework_plugin::MountedOwnerPolicyV1;

const FIXTURE: &str = include_str!("../🧫️fixtures/🎟️mounted-owner-policy/🔣️.json");

#[test]
fn the_portable_policy_matches_native_and_third_party_wire_output() {
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("policy fixture");
    let selected = crate::WFC_MOUNTED_OWNER_POLICY.validate().expect("portable WFC authority");
    let baseline = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    for (actual, previous) in [(selected.preparation, baseline.preparation), (selected.maintenance, baseline.maintenance), (selected.close, baseline.close)] {
        let native: std::collections::BTreeMap<&str, usize> = actual.wire_fields().into_iter().collect();
        assert_eq!(serde_json::to_value(native).expect("native wire output"), fixture["grant"]);
        assert_eq!(serde_json::to_value(actual).expect("third-party wire output"), fixture["grant"]);
        assert_eq!(actual.maximum_items, previous.maximum_items);
        assert_eq!(actual.maximum_copy_bytes, previous.maximum_copy_bytes);
        assert_eq!(actual.maximum_capacity_bytes, previous.maximum_capacity_bytes);
        assert_eq!(actual.maximum_depth, previous.maximum_depth);
        assert_eq!(actual.maximum_release_bytes, actual.maximum_capacity_bytes);
    }
}


fn retire_step(caller: &mut semio_framework_plugin::artifact_app_laws::OriginalWorkFixture, owner: &mut dyn semio_framework_plugin::__value::ErasedSnapshotRetirement, grant: semio_framework_job::RetainedCloneGrant) -> semio_framework_job::RetainedCloneProgress {
    caller.turn(grant, |cx| cx.with_original_retirement(|cx| {
        let demand = owner.next_demand(grant.maximum_copy_bytes)?;
        semio_framework_job::close_original_retained_step(cx, demand, |remaining| owner.close_step(remaining))
    })).expect("same original caller retires the selected phase").progress()
}

#[test]
fn every_selected_phase_retires_the_maximum_bitmap_pin_backing() {
    let fixture: serde_json::Value = serde_json::from_str(FIXTURE).expect("retirement fixture");
    let law = &fixture["retirement"];
    let count = law["pinCapacity"].as_u64().expect("pin capacity") as usize;
    let maximum_turns = law["maximumTurns"].as_u64().expect("turn bound") as usize;
    let baseline = MountedOwnerPolicyV1::INTERACTIVE_COMPONENT;
    let selected = crate::WFC_MOUNTED_OWNER_POLICY;
    for (grant, old) in [(selected.preparation, baseline.preparation), (selected.maintenance, baseline.maintenance), (selected.close, baseline.close)] {
        let original = Vec::<semio_s_artifact_wfc_bitmap::schema::snapshot::BitmapPinnedPixel>::with_capacity(count);
        let backing = original.capacity() * std::mem::size_of::<semio_s_artifact_wfc_bitmap::schema::snapshot::BitmapPinnedPixel>();
        assert!(backing >= law["minimumBackingBytes"].as_u64().expect("minimum backing") as usize);
        assert!(backing <= law["maximumBackingBytes"].as_u64().expect("maximum backing") as usize);
        let mut caller = semio_framework_plugin::artifact_app_laws::OriginalWorkFixture::new();
        let mut owner = caller.turn(grant, |cx| {
            let turn = admit_owned_retirement(original, cx.retained_grant());
            cx.consume_fuel(turn.work);
            let (owner, progress) = turn.result.map_err(|(error, _)| semio_framework_job::OriginalTurnFailure::from(error))?;
            cx.consume_retained(progress)?;
            Ok(owner)
        }).expect("same original caller admits the bitmap backing");
        let mut blocked = false;
        for _ in 0..maximum_turns {
            if owner.next_release_byte_demand().expect("release quote") > old.maximum_release_bytes {
                let refused = retire_step(&mut caller, owner.as_mut(), old);
                assert_eq!(refused.released_bytes, 0);
                assert!(!owner.terminal_is_empty());
                blocked = true;
                break;
            }
            retire_step(&mut caller, owner.as_mut(), old);
        }
        assert!(blocked, "the original one MiB release grant cannot free this same admitted backing");
        let mut released = 0usize;
        for _ in 0..maximum_turns {
            if owner.terminal_is_empty() { break; }
            let progress = retire_step(&mut caller, owner.as_mut(), grant);
            assert!(progress.fits(grant));
            released += progress.released_bytes;
        }
        assert!(owner.terminal_is_empty(), "the selected release policy closes within the portable turn bound");
        assert!(released >= backing, "the physical receipt includes the original pin allocation");
        caller.finish();
    }
}
