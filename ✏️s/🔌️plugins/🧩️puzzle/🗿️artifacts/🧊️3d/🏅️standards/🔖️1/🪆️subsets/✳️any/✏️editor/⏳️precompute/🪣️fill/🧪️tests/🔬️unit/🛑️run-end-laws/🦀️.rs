/// 🛑️ Run end laws: rule refusals as warnings and the declared visible end of every case and own-mesh variant.

/// ⚖️ LAW: a rule refusal is a `warning` record, never a collision — a document whose target volume
/// lies away from every open vortex refuses each constructed candidate as `outside-target-volume`,
/// appends nothing and ends with the `no-free-placement` warning step.
#[test]
fn fill_run_job_reports_rule_refusals_as_warnings_and_the_stall_as_a_warning_step() {
    let roots = nakagin_scale_roots();
    let mut scene = (*roots.scene).clone();
    scene.scene_snapshot.target_volumes.push(WorldVolumeProps { id: "elsewhere".into(), origin: [1.0e6, 1.0e6, 1.0e6], orientation: Some([0.0, 0.0, 0.0, 1.0]), scale: None });
    let mut job = FillRunJob::new(FillBuilder::begin_preparation(FillPreparationRoots::new(Arc::new(scene), roots.meshes.clone()), Operation::new(OperationId(89), RevisionId(1), Generation(1), 43), 10), fill_run_identity(), vec![NAKAGIN_MESH_URL.to_string()], [0; 32]);
    let mut mirror = FillRunMirror::new();
    mirror.drive(&mut job, u64::MAX, 1_000_000);
    let [tested, locked, collisions, rejected, marked] = job.counters();
    assert!(tested > 0 && tested == rejected && locked == 0 && collisions == 0 && marked > 0, "{:?}", job.counters());
    assert!(mirror.verdicts.iter().all(|(_, verdict, reason)| (*verdict == ToolRunVerdict::Warning && *reason == FillRunReason::OutsideTargetVolume.code()) || (*verdict == ToolRunVerdict::Danger && *reason == FillRunReason::VortexExhausted.code())));
    assert_eq!(mirror.verdicts.iter().filter(|(_, _, reason)| *reason == FillRunReason::VortexExhausted.code()).count() as u64, marked, "every vortex whose candidates were all refused is marked once");
    assert!(mirror.ops.is_empty() && mirror.entities.is_empty());
    let last = mirror.steps.last().expect("stall step");
    assert_eq!((last.kind, last.reason, last.args.clone()), (ToolRunStepKind::Warning, FillRunReason::NoFreePlacement.code(), vec![ToolRunStepArg::Unsigned(0)]));
    assert_eq!(mirror.progress.as_ref().map(|progress| progress.state), Some(ToolRunState::Complete));
}

/// 🏁️ How a run ended as its ledger saw it: `None` when the last step is `success:requested-reached` over a met request, else
/// the declared stall reason its `warning` step names. A run that ended without a step, with an undeclared reason, or with
/// zero verdicts and no stall reason fails the law.
fn fill_run_visible_end(name: &str, job: &FillRunJob, mirror: &FillRunMirror, requested: usize) -> Option<&'static str> {
    let [tested, locked, _, _, _] = job.counters();
    assert_eq!(mirror.progress.as_ref().map(|progress| progress.state), Some(ToolRunState::Complete), "{name}: the run's last progress is complete");
    let last = mirror.steps.last().unwrap_or_else(|| panic!("{name}: the run ended with counters {:?} and no step", job.counters()));
    assert_eq!(last.args, vec![ToolRunStepArg::Unsigned(locked)], "{name}: the terminal step carries the placement count");
    match (last.kind, FillRunReason::from_code(last.reason)) {
        (ToolRunStepKind::Success, Some(FillRunReason::RequestedReached)) => {
            assert!(locked >= requested as u64 && tested >= locked && requested > 0, "{name}: requested-reached with {locked} of {requested} placed and {tested} tested");
            None
        }
        (ToolRunStepKind::Warning, Some(reason)) if FillStall::ALL.iter().any(|stall| stall.reason() == reason) => Some(reason.id()),
        (kind, reason) => panic!("{name}: the run ended with {kind:?} {reason:?} after {tested} verdicts instead of a declared terminal reason"),
    }
}

/// 🏁️ LAW (language-neutral scene_snapshot `🎞️fill-run.json` `laws.visibleEnd`, ticket lane W1-H): every fill run ends where the user
/// can see why — every shipped case and every own-mesh variant ends with `success:requested-reached` or a `warning` step
/// naming a declared stall reason, never with zero verdicts and no reason. The own-mesh Nakagin variant packs more bodies
/// into one spatial cell than one bookkeeping page holds; it must still be planned and match its declared outcome.
#[test]
fn fill_run_ends_visibly_with_a_declared_reason_for_every_case_and_own_mesh_variant() {
    let scene_snapshot: serde_json::Value = serde_json::from_str(FILL_RUN_FIXTURE).expect("fill run scene_snapshot");
    for case in scene_snapshot["cases"].as_array().expect("cases") {
        let (document, seed, requested) = (case["document"].as_str().expect("document"), case["seed"].as_u64().expect("seed"), case["requested"].as_u64().expect("requested") as usize);
        let (roots, lane, _) = example_fill_roots(document, seed as u32);
        let mut job = fill_run_job(roots, lane, seed, requested);
        let mut mirror = FillRunMirror::new();
        mirror.drive(&mut job, u64::MAX, 1_000_000);
        let name = format!("{document} seed {seed} requested {requested}");
        assert_eq!(fill_run_visible_end(&name, &job, &mirror, requested), case["expected"]["stall"].as_str(), "{name}");
    }
    let mut disagreements = Vec::new();
    for case in scene_snapshot["laws"]["visibleEnd"]["cases"].as_array().expect("visible end cases") {
        let (document, seed, requested, scale) = (case["document"].as_str().expect("document"), case["seed"].as_u64().expect("seed"), case["requested"].as_u64().expect("requested") as usize, case["ownMeshScale"].as_f64().expect("own mesh scale") as f32);
        let (roots, lane, _) = own_mesh_fill_roots(document, seed as u32, scale);
        let mut job = fill_run_job(roots, lane, seed, requested);
        let mut mirror = FillRunMirror::new();
        mirror.drive(&mut job, u64::MAX, 1_000_000);
        let name = format!("{document} seed {seed} requested {requested} own-mesh {scale}");
        let stall = fill_run_visible_end(&name, &job, &mirror, requested);
        let [tested, locked, collisions, rejected, marked] = job.counters();
        assert!(tested > 0 || stall.is_some(), "{name}: zero verdicts always name their stall");
        let actual = serde_json::json!({ "tested": tested, "locked": locked, "collisions": collisions, "rejected": rejected, "marked": marked, "stall": stall });
        if actual != case["expected"] {
            disagreements.push(format!("{name}: actual {actual}"));
        }
    }
    assert!(disagreements.is_empty(), "the visible-end law disagrees:\n{}", disagreements.join("\n"));
}

/// 🚧️ LAW (ticket lane W1-H): an artifact page that fills mid-plan ends the run with the `artifact-capacity` warning step over
/// what was already placed, the candidate under test reaching its verdict first — never a bare `Complete` the ledger cannot
/// explain.
#[test]
fn fill_run_job_mid_plan_capacity_stall_ends_with_a_visible_warning_step() {
    let (roots, lane, _) = example_fill_roots("concrete-forest", 7);
    let mut job = fill_run_job(roots, lane, 7, 8);
    let mut mirror = FillRunMirror::new();
    let mut sequence = 0;
    loop {
        match fill_run_turn(&mut job, u64::MAX, &mut sequence) {
            FillRunTurn::Tick(tick) => mirror.apply(tick),
            FillRunTurn::Checkpoint(bytes) => {
                mirror.checkpoints.push(bytes);
                break;
            }
            FillRunTurn::Complete => panic!("the run completed before its first placement"),
            FillRunTurn::Yield => {}
        }
    }
    job.builder.collection_over_capacity = true;
    mirror.drive(&mut job, u64::MAX, 1_000);
    assert_eq!(fill_run_visible_end("mid-plan capacity", &job, &mirror, 8), Some(FillRunReason::ArtifactCapacity.id()));
    let [tested, locked, _, _, _] = job.counters();
    assert_eq!((locked, mirror.verdicts.len() as u64), (1, tested), "one placement, and every tested candidate reached its verdict");
}
