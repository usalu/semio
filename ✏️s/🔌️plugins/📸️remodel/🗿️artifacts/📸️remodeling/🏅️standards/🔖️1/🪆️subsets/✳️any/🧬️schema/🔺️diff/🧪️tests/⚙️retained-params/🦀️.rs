use super::*;
use protocol::{RetainedPatchSource, RetainedRowPatchFields};
use semio_framework_value::{
    retained_clone::{RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep},
    retirement::{controlled::ControlledRetirement, RetireOwned},
    RetirementDemand,
};
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ReconstructionParams;

fn grant(demand: RetirementDemand) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth }
}

fn source<T: RetireOwned + Sync>(value: T) -> RetainedCloneSource<T> {
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 65536, maximum_capacity_bytes: 65536, maximum_release_bytes: 65536, maximum_depth: 4096 };
    let mut admission = RetainedCloneSource::admission_owned(value, ());
    let mut output = None;
    for _ in 0..4096 {
        if admission.terminal_is_empty() {
            return output.expect("funded original source");
        }
        let mut actual = None;
        let outcome = if admission.phase() == "ready" { admission.take_output(&mut output, grant) } else { admission.advance(grant, &mut actual) };
        assert!(outcome.result.unwrap().progress().fits(grant));
    }
    panic!("source admission did not settle");
}

fn close_source<T: RetireOwned + Sync>(source: &mut RetainedCloneSource<T>) {
    for _ in 0..4096 {
        if source.terminal_is_empty() {
            return;
        }
        let grant = grant(source.next_close_demand(4096).unwrap());
        assert!(source.close_step(grant).result.unwrap().progress().fits(grant));
    }
    panic!("source closure did not settle");
}

fn retire<T: RetireOwned>(value: T) {
    let mut owner = ControlledRetirement::new(value).unwrap();
    for _ in 0..4096 {
        if owner.terminal_is_empty() {
            return;
        }
        let grant = grant(owner.next_demand(4096).unwrap());
        assert!(owner.step(grant).unwrap().progress().fits(grant));
    }
    panic!("owned value retirement did not settle");
}

fn values(case: &serde_json::Value) -> (ReconstructionParams, RemodelingParamsDiff) {
    let mut original = ReconstructionParams::default();
    let mut patch = RemodelingParamsDiff::default();
    let before = u32::try_from(case["before"].as_u64().unwrap()).unwrap();
    let after = u32::try_from(case["after"].as_u64().unwrap()).unwrap();
    macro_rules! facet {
        ($field:ident, $member:ident) => {{
            original.$field.$member = before;
            let mut authored = original.$field.clone();
            authored.$member = after;
            patch.$field = Some(authored);
        }};
    }
    match case["field"].as_str().unwrap() {
        "ingest" => facet!(ingest, max_frames),
        "feature" => facet!(feature, target_count),
        "matching" => facet!(matching, sequential_window),
        "sfm" => facet!(sfm, ransac_iterations),
        "dense" => facet!(dense, max_points),
        "mesh" => facet!(mesh, texture_size),
        "motion" => facet!(motion, max_tracks),
        "geo" => facet!(geo, ortho_max_px),
        _ => panic!("undeclared parameter facet"),
    }
    (original, patch)
}

fn prepare(case: &serde_json::Value, kind: RetainedPatchSource, cut: Option<usize>) -> usize {
    let (row, patch) = values(case);
    let expected = match kind {
        RetainedPatchSource::Authored => patch.clone(),
        RetainedPatchSource::Inverse => patch.restoring(&row),
    };
    let mut original = source(row);
    let mut authored = source(patch);
    let (mut fields, birth_work) = retained_params::Fields::new(kind);
    assert_eq!(birth_work, retained_params::Fields::new_work_demand().unwrap());
    let mut turns = 0;
    let mut complete = false;
    while cut.is_none_or(|cut| turns < cut) {
        let demand = fields.demand(authored.borrow(), original.borrow(), 64, 64).unwrap();
        let work = fields.advance_work_demand(authored.borrow(), original.borrow(), 64, 64).unwrap();
        let selected = grant(demand);
        for denied in [RetainedCloneGrant::default(), RetainedCloneGrant { maximum_items: 0, ..selected }, RetainedCloneGrant { maximum_depth: 0, ..selected }] {
            let outcome = fields.advance(authored.borrow(), original.borrow(), 64, 64, denied);
            assert_eq!(outcome.work, 0);
            assert_eq!(outcome.result.unwrap().progress(), Default::default());
        }
        let denied_work = fields.advance(authored.borrow(), original.borrow(), 64, 0, selected);
        assert_eq!(denied_work.work, 0);
        assert_eq!(denied_work.result.unwrap().progress(), Default::default());
        let outcome = fields.advance(authored.borrow(), original.borrow(), 64, 64, selected);
        assert!(outcome.work <= work);
        let step = outcome.result.unwrap();
        assert!(step.progress().fits(selected));
        turns += 1;
        assert!(turns < 4096);
        if matches!(step, RetainedCloneStep::Complete(_)) {
            complete = true;
            break;
        }
    }
    if cut.is_none() {
        assert!(complete);
        let (result, receipt) = fields.take(grant(RetirementDemand { copy_bytes: 2 * std::mem::size_of::<RemodelingParamsDiff>(), depth: 1, ..Default::default() })).unwrap();
        assert!(receipt.copied_items > 0);
        let result = result.unwrap();
        assert_eq!(result, expected);
        let json: serde_json::Value = serde_json::from_str(&semio_framework_pack_json::to_json_string(&result)).unwrap();
        let field = case["field"].as_str().unwrap();
        let member = case["member"].as_str().unwrap();
        assert_eq!(
            json[field][member],
            case[match kind {
                RetainedPatchSource::Authored => "after",
                RetainedPatchSource::Inverse => "before",
            }]
        );
        assert!(json.as_object().unwrap().iter().all(|(key, value)| key == field || value.is_null()));
        retire(result);
    }
    fields.begin_close();
    for close_turn in 0..4096 {
        if fields.terminal_is_empty() {
            break;
        }
        assert!(close_turn < 4095);
        let work = fields.close_work_demand(4096).unwrap();
        let selected = grant(fields.close_demand(4096).unwrap());
        let denied = fields.close_step(RetainedCloneGrant::default());
        assert_eq!(denied.work, 0);
        assert_eq!(denied.result.unwrap().progress(), Default::default());
        let outcome = fields.close_step(selected);
        assert!(outcome.work <= work);
        assert!(outcome.result.unwrap().progress().fits(selected));
    }
    assert!(fields.terminal_is_empty());
    drop(fields);
    close_source(&mut authored);
    close_source(&mut original);
    retire(expected);
    turns
}

/// ⚙️ Every neutral parameter facet prepares only its authored fields and its exact original inverse under physical grants.
#[test]
fn retained_parameter_fields_match_neutral_facets_and_close_every_frontier() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️retained-params/🔣️.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        for kind in [RetainedPatchSource::Authored, RetainedPatchSource::Inverse] {
            let turns = prepare(case, kind, None);
            for cut in 0..=turns {
                prepare(case, kind, Some(cut));
            }
        }
    }
}
