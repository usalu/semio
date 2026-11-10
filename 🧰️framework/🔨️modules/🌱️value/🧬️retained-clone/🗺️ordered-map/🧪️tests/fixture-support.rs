//! 🧰️ Shared ordered-map unit fixtures: the paging corpus, its BTreeMap oracle and the borrowed clone helper.
use crate::*;
use crate::FixtureSourceBirth;
use crate::retained_clone::RetainedCloneSource;
use semio_framework_value_testing_effect_bank::EffectBank;
use serde::Deserialize;
use std::collections::BTreeMap;
#[path="clone-support.rs"]
pub(crate) mod clone_support;
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Fixture {
    pub(crate) page_capacity: usize,
    pub(crate) entry_count: usize,
    pub(crate) key_prefix: String,
    pub(crate) value_prefix: String,
    pub(crate) long_key_byte_length: usize,
    pub(crate) comparison_grant: ComparisonGrant,
    pub(crate) progress_channels: ProgressChannels,
    pub(crate) repeated_growth: RepeatedGrowth,
    pub(crate) immutable_lookup: ImmutableLookup,
    pub(crate) operations: Vec<Operation>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct RepeatedGrowth {
    pub(crate) entry_count: usize,
    pub(crate) insertions: usize,
    pub(crate) key_prefix: String,
    pub(crate) value_prefix: String,
    pub(crate) expected_entry_count: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ImmutableLookup {
    pub(crate) captured_target: String,
    pub(crate) external_after_capture: String,
    pub(crate) expected_ordinal: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ComparisonGrant {
    pub(crate) maximum_items: usize,
    pub(crate) maximum_bytes: usize,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ProgressChannels {
    pub(crate) comparison_only: bool,
    pub(crate) capacity_only: bool,
    pub(crate) minimum_moved_items: usize,
}

#[derive(Deserialize)]
pub(crate) struct Operation {
    pub(crate) kind: String,
    pub(crate) key: String,
    pub(crate) value: Option<String>,
    pub(crate) expected: Expected,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Expected {
    pub(crate) found: bool,
    pub(crate) ordinal: usize,
    pub(crate) entry_count: usize,
}

pub(crate) fn fixture() -> Fixture {
    serde_json::from_str(include_str!("🧫️fixtures/📦️paging/🔣️.json")).expect("retained ordered-map fixture")
}

pub(crate) fn oracle(fixture: &Fixture) -> BTreeMap<String, String> {
    (0..fixture.entry_count).map(|ordinal| (format!("{}{:04}", fixture.key_prefix, ordinal), format!("{}{}", fixture.value_prefix, ordinal))).collect()
}

pub(crate) fn retained_map(source:&RetainedOrderedMap<String,String>)->(RetainedOrderedMap<String,String>,EffectBank){
 let mut source=RetainedCloneSource::from_owner(source.clone());let policy=RetainedCloneGrant{maximum_items:2,maximum_copy_bytes:64,maximum_capacity_bytes:65536,maximum_depth:64,maximum_release_bytes:65536};
 let output=source.with_bank(|input,bank|{let mut clone=clone_support::Clone::<RetainedOrderedMap<String,String>>::new(bank);for _ in 0..100000{if clone.cursor.output_ready(){break}let d=clone.cursor.advance_demands(input,64,64).unwrap();let g=RetainedCloneGrant{maximum_copy_bytes:policy.maximum_copy_bytes.max(d.copy_bytes),..policy};let work=clone.cursor.advance_work_demand(input,64,64).unwrap();let observed=clone.advance(input,bank,64,g,work);assert!(observed.step.progress().fits(g));}assert!(clone.cursor.output_ready());clone.take(input,bank,64);let(cursor,output)=clone.finish();clone_support::retire(cursor,bank,7,100000);output.expect("retained ordered-map output")}).unwrap();
 clone_support::close_source(&mut source,7,100000);let bank=source.into_bank().unwrap_or_else(|_|panic!("cloned Source retains its actual payer"));(output,bank)
}

