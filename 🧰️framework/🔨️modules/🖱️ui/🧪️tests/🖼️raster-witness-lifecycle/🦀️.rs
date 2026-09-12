//! 🖼️ LAW: a raster operation's witness pair is admitted ONCE and then owned by its retirement.
//!
//! `commit_presented_step` re-read the candidate and presentation witnesses on every retirement step.
//! The retirement retires those very slots one field at a time, so the step after the first field was
//! consumed compared a half-empty slot against the operation that owned it and refused — the frame
//! quarantined itself with `raster commit candidate witness was stale` at ~7.8 s on every boot
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-document-reconcile-2026-09-12.md` §6). The same
//! shape refused a retirement whose presentation slot it had itself begun to consume.
//!
//! The neutral oracle is `🖱️ui/🧫️fixtures/🖼️raster-witness-lifecycle/🔣️.json`; its TypeScript twin is
//! `📺️renderer/🧑‍🎨engine/🧪️tests/🖼️wgpu-raster-witness/🟦️.ts`.

use super::*;

fn law() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/🖼️raster-witness-lifecycle/🔣️.json")).expect("raster witness lifecycle fixture")
}

fn witness(law: &serde_json::Value, name: &str) -> RasterTextureWitness {
    let source = &law["operations"][name];
    RasterTextureWitness {
        scene_revision: source["sceneRevision"].as_u64().expect("fixture scene revision"),
        preview_generation: source["previewGeneration"].as_u64().expect("fixture preview generation"),
        operation: source["operation"].as_u64().expect("fixture operation"),
    }
}

fn reserved_and_presented(law: &serde_json::Value, name: &str) -> RasterOperationWitnessLedger {
    let operation = witness(law, name);
    let mut ledger = RasterOperationWitnessLedger::default();
    ledger.admit_candidate(operation);
    ledger.arm_presentation(operation).expect("its own candidate arms");
    ledger
}

fn retire_to_terminal(ledger: &mut RasterOperationWitnessLedger, budget: usize) -> usize {
    for step in 1..=budget {
        if ledger.retire_step() {
            return step;
        }
    }
    panic!("raster witness retirement exceeded its fixed budget");
}

#[test]
fn a_surface_with_no_raster_yet_owes_nothing_and_never_reads_as_stale() {
    let law = law();
    let ledger = RasterOperationWitnessLedger::default();
    for name in ["first", "second", "laterScene"] {
        assert_eq!(ledger.retirement_admission(witness(&law, name)), Ok(RasterWitnessAdmission::Nothing), "an unreserved surface owes nothing to any operation");
    }
    assert!(ledger.is_empty());
    assert_eq!(law["admission"]["rules"][0]["verdict"], "Nothing");
}

#[test]
fn a_commit_stays_admissible_at_every_step_of_the_retirement_it_owns() {
    let law = law();
    let operation = witness(&law, "first");
    let second = witness(&law, "second");
    let mut ledger = reserved_and_presented(&law, "first");
    assert_eq!(ledger.retirement_admission(operation), Ok(RasterWitnessAdmission::Retire));
    assert_eq!(ledger.begin_retirement(operation), Ok(RasterWitnessAdmission::Retire));
    assert_eq!(ledger.retiring(), Some(operation));
    let expected = law["retirement"]["expected"]["reservedAndPresented"].as_u64().expect("fixture step count") as usize;
    let occupied = law["admission"]["occupied"].as_str().expect("fixture occupied fault");
    let mut steps = 0usize;
    loop {
        // 🩺️ THE defect: this admission used to turn into a refusal the moment the cursor consumed
        // the candidate slot's first field.
        assert_eq!(ledger.begin_retirement(operation), Ok(RasterWitnessAdmission::Retire), "an operation is never stale against its own retirement");
        assert_eq!(ledger.retirement_admission(second), Err(occupied), "…and nobody else may take the slots it is consuming");
        steps += 1;
        assert!(steps <= expected, "retirement exceeded the fixture's own step count");
        if ledger.retire_step() {
            break;
        }
    }
    assert_eq!(steps, expected, "the fixture pins the exact bounded step count");
    assert!(ledger.is_empty(), "a completed retirement leaves the ledger terminal and reusable");
    assert_eq!(ledger.retiring(), None);
    assert_eq!(ledger.retirement_admission(operation), Ok(RasterWitnessAdmission::Nothing));
    assert_eq!(ledger.retirement_admission(second), Ok(RasterWitnessAdmission::Nothing), "the next operation starts from a clean ledger");
}

#[test]
fn an_operation_that_reserved_but_never_presented_retires_its_candidate_alone() {
    let law = law();
    let operation = witness(&law, "first");
    let mut ledger = RasterOperationWitnessLedger::default();
    ledger.admit_candidate(operation);
    assert!(ledger.presenting_is_empty());
    assert_eq!(ledger.begin_retirement(operation), Ok(RasterWitnessAdmission::Retire));
    let expected = law["retirement"]["expected"]["reservedOnly"].as_u64().expect("fixture step count") as usize;
    assert_eq!(retire_to_terminal(&mut ledger, expected + 1), expected);
    assert!(ledger.is_empty());
}

#[test]
fn the_fixtures_step_arithmetic_is_the_authoritys_own() {
    let law = law();
    let fields = law["witness"]["fields"].as_array().expect("fixture witness fields").len();
    let occupied = law["retirement"]["stepsPerOccupiedSlot"].as_u64().expect("fixture occupied slot steps") as usize;
    let empty = law["retirement"]["stepsPerEmptySlot"].as_u64().expect("fixture empty slot steps") as usize;
    let terminal = law["retirement"]["terminalStep"].as_u64().expect("fixture terminal step") as usize;
    assert_eq!(occupied, fields + 1, "one step per field plus the step that observes the slot terminal");
    assert_eq!(empty, 1);
    assert_eq!(law["retirement"]["expected"]["reservedAndPresented"].as_u64().expect("fixture") as usize, occupied * 2 + terminal);
    assert_eq!(law["retirement"]["expected"]["reservedOnly"].as_u64().expect("fixture") as usize, occupied + empty + terminal);
}

#[test]
fn a_foreign_operation_is_refused_by_name_and_the_ledger_is_left_untouched() {
    let law = law();
    let first = witness(&law, "first");
    let second = witness(&law, "second");
    let ledger = reserved_and_presented(&law, "first");
    assert_eq!(ledger.retirement_admission(second), Err(law["admission"]["rules"][3]["fault"].as_str().expect("fixture fault")));
    assert_eq!(ledger.candidate(), Some(first), "a refused admission consumes nothing");
    assert_eq!(ledger.presenting(), Some(first));

    let mut crossed = RasterOperationWitnessLedger::default();
    crossed.admit_candidate(first);
    crossed.arm_presentation(first).expect("its own candidate arms");
    let _ = crossed.retire_step();
    assert!(crossed.candidate().is_none() && !crossed.candidate_is_empty(), "a half-consumed slot reads as neither whole nor empty");
}

#[test]
fn a_presentation_arms_only_its_own_candidate_and_only_once() {
    let law = law();
    let first = witness(&law, "first");
    let second = witness(&law, "second");
    let mut ledger = RasterOperationWitnessLedger::default();
    assert_eq!(ledger.arm_presentation(first), Err(law["presentation"]["armsOnlyItsOwnCandidate"].as_str().expect("fixture fault")));
    ledger.admit_candidate(first);
    assert_eq!(ledger.arm_presentation(second), Err(law["presentation"]["armsOnlyItsOwnCandidate"].as_str().expect("fixture fault")));
    assert_eq!(ledger.arm_presentation(first), Ok(()));
    assert_eq!(ledger.arm_presentation(first), Err(law["presentation"]["refusesASecondArming"].as_str().expect("fixture fault")));
}

#[test]
fn one_frames_rasters_share_one_candidate_and_a_foreign_one_is_named_occupied() {
    let law = law();
    let first = witness(&law, "first");
    let second = witness(&law, "second");
    let mut ledger = RasterOperationWitnessLedger::default();
    assert!(!ledger.candidate_occupied_by(first));
    ledger.admit_candidate(first);
    assert!(!ledger.candidate_occupied_by(first), "a second raster of the same frame reuses the candidate");
    ledger.admit_candidate(first);
    assert_eq!(ledger.candidate(), Some(first));
    assert!(ledger.candidate_occupied_by(second), "a later operation may not overwrite a live candidate");
}

#[test]
fn a_presentation_from_another_operation_is_refused_by_its_own_name() {
    let law = law();
    let first = witness(&law, "first");
    let second = witness(&law, "second");
    let mut ledger = RasterOperationWitnessLedger::default();
    ledger.admit_candidate(first);
    ledger.arm_presentation(first).expect("its own candidate arms");
    let _ = ledger.retire_step();
    let _ = ledger.retire_step();
    let _ = ledger.retire_step();
    let _ = ledger.retire_step();
    ledger.admit_candidate(second);
    assert_eq!(ledger.retirement_admission(second), Err(law["admission"]["rules"][4]["fault"].as_str().expect("fixture fault")));
}
