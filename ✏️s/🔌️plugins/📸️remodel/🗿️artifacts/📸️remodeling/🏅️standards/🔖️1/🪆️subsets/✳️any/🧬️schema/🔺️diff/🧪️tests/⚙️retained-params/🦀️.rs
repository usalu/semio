use super::*;
use super::retained_test_support::{grant, retire};
use protocol::{RetainedDone, RetainedPatchSource, RetainedReply, RetainedRowPatchFields};
use semio_framework_value::{
    retained_clone::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneRef, RetainedCloneStep},
    OriginalRetirementFailure, RetirementDemand, ValueError,
};
use semio_framework_value_clone_source_testing::{FixtureSource, drive::drive_reply};
use semio_s_artifact_remodel_remodeling_model::standards::v1::subsets::any::schema::snapshot::ReconstructionParams;

fn receipt(result: &Result<RetainedCloneStep, OriginalRetirementFailure>) -> RetainedCloneProgress {
    match result {
        Ok(step) => step.progress(),
        Err(error) => error.retained_progress(),
    }
}

fn advance(fields: &mut retained_params::Fields, patch: RetainedCloneRef<'_, RemodelingParamsDiff>, row: RetainedCloneRef<'_, ReconstructionParams>, maximum: u64, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome<RetainedCloneStep, OriginalRetirementFailure> {
    drive_reply(grant, 64, receipt, |effect, reply: RetainedReply<'_>, recover| -> Result<RetainedDone<'_>, ValueError> {
        fields.advance_reply(patch, row, 64, maximum, effect, reply, recover)
    })
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
    let original = FixtureSource::from_owner(row);
    let authored = FixtureSource::from_owner(patch);
    let (mut fields, birth_work) = retained_params::Fields::new(kind);
    assert_eq!(birth_work, retained_params::Fields::new_work_demand().unwrap());
    let mut turns = 0;
    let mut complete = false;
    while cut.is_none_or(|cut| turns < cut) {
        let demand = fields.demand(authored.borrow(), original.borrow(), 64, 64).unwrap();
        let work = fields.advance_work_demand(authored.borrow(), original.borrow(), 64, 64).unwrap();
        let selected = grant(demand);
        for denied in [RetainedCloneGrant::default(), RetainedCloneGrant { maximum_items: 0, ..selected }, RetainedCloneGrant { maximum_depth: 0, ..selected }] {
            let outcome = advance(&mut fields, authored.borrow(), original.borrow(), 64, denied);
            assert_eq!(outcome.work, 0);
            assert_eq!(outcome.result.unwrap().progress(), Default::default());
        }
        let denied_work = advance(&mut fields, authored.borrow(), original.borrow(), 0, selected);
        assert_eq!(denied_work.work, 0);
        assert_eq!(denied_work.result.unwrap().progress(), Default::default());
        let outcome = advance(&mut fields, authored.borrow(), original.borrow(), 64, selected);
        assert!(outcome.work <= u128::from(work));
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
        let close = |fields: &mut retained_params::Fields, grant| {
            drive_reply(grant, 4096, receipt, |effect, reply: RetainedReply<'_>, recover| -> Result<RetainedDone<'_>, ValueError> {
                fields.close_reply(None, 4096, effect, reply, recover)
            })
        };
        let denied = close(&mut fields, RetainedCloneGrant::default());
        assert_eq!(denied.work, 0);
        assert_eq!(denied.result.unwrap().progress(), Default::default());
        let outcome = close(&mut fields, selected);
        assert!(outcome.work <= u128::from(work));
        assert!(outcome.result.unwrap().progress().fits(selected));
    }
    assert!(fields.terminal_is_empty());
    drop(fields);
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
