use semio_framework_test_fixtures::fixture_text;
use semio_framework_value_clone_source_testing::FixtureSourceBirth;
use serde::Deserialize;
use std::{
    mem::{ManuallyDrop, size_of},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

fn close_fixture<T: RetainedClone>(cursor: &mut impl RetainedCloneCursor<T>, items: usize, work: usize) -> Result<RetainedCloneStep, crate::OriginalRetirementFailure> {
    let d=cursor.close_demands(work)?;
    let grant=RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(cursor.close_demands(grant.maximum_copy_bytes)?,d);
    let quote=cursor.close_work_demand(work)?;let turn=effect::close_step(cursor,grant);assert!(turn.work<=u128::from(quote));let step=turn.result?;
    assert!(step.progress().fits(grant));
    Ok(step)
}

fn retire_fixture<T: RetireOwned>(owner: &mut crate::retirement::controlled::ControlledRetirement<T>, items: usize, work: usize) -> Result<RetainedCloneStep, crate::OriginalRetirementFailure> {
    let d=owner.selected_demand(work)?;let grant=RetainedCloneGrant{maximum_items:items,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};assert_eq!(owner.selected_demand(grant.maximum_copy_bytes)?,d);
    let result=effect::settle(grant,|e|owner.step(e)).result;assert!(owner.last_turn_work()<=1);let step=result?;
    assert!(step.progress().fits(grant));
    Ok(step)
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Fixture {
    maximum_entries: usize,
    entry_count: usize,
    value_prefix: String,
    grant: GrantFixture,
    cancellation_after_entries: usize,
    expected: CloseExpectation,
}
#[derive(Deserialize)]
#[serde(rename_all="camelCase")]
struct CloseExpectation{spent_close_turns:usize,spent_close_cpu:u64}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GrantFixture {
    maximum_items: usize,
    maximum_copy_bytes: usize,
    maximum_capacity_bytes: usize,
    maximum_release_bytes: usize,
    maximum_depth: usize,
}

fn fixture() -> Fixture {
    serde_json::from_str(fixture_text!("../../../📋️paged-list/🧪️tests/🧫️fixtures/📦️copy/🔣️.json")).expect("paged list retained fixture")
}

fn source(fixture: &Fixture) -> PagedList<String, 1024> {
    assert_eq!(fixture.maximum_entries, 1024);
    let mut source = PagedList::default();
    for ordinal in 0..fixture.entry_count {
        while !source.has_reserved_slot() {
            assert!(effect::progressed(&effect::run(effect::STOCK,|e|source.reserve_one(e)).result.expect("fixture reserve")));
        }
        source.push_reserved(format!("{}{}", fixture.value_prefix, ordinal)).expect("fixture placement");
    }
    source
}

fn grant(fixture: &Fixture) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: fixture.grant.maximum_items, maximum_copy_bytes: fixture.grant.maximum_copy_bytes, maximum_capacity_bytes: fixture.grant.maximum_capacity_bytes, maximum_depth: fixture.grant.maximum_depth, maximum_release_bytes: fixture.grant.maximum_release_bytes }
}

