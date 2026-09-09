//! ⏱️ Kernel-level laws for the RESUMABLE tessellator and the pre-tessellation validate gate — the
//! two pieces that turn the preview's formerly one-shot, uncancellable `tessellate` call into an
//! interactive job.
//!
//! Every law here is stated against the public kernel surface only (`TessellationJob`, `Brep`), so
//! it holds for any host driving the job: the flow-brep extension, the wgpu render target, or a
//! future non-Rust implementation of the same contract.
//!
//! Ticket 26/09/09/PROCEDURAL-3D-END-TO-END.

// #region 🔖️Imports
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_sphere};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::engine::{Brep, BrepKernel, GeometryHandle};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::inferences::tessellation::{tessellate_solid, TessellationJob, TessellationPhase, TessellationStep};
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::arena::SolidId;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::topology::Body;
// #endregion 🔖️Imports

// #region 🧰️Fixtures
/// 🧱 A unit box — six planar faces, twelve edges, the smallest solid with more than one unit of
/// work in every phase.
fn boxed_body() -> (Body, SolidId) {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).expect("unit box");
    (body, solid)
}

/// 🔮 A sphere — curved faces, so the face phase is genuinely expensive per unit and the edge phase
/// covers seam/pole topology.
fn sphere_body() -> (Body, SolidId) {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_sphere(&mut body, 1.0, &mut rec).expect("unit sphere");
    (body, solid)
}
// #endregion 🧰️Fixtures

// #region ⏱️BudgetAndProgress
/// ⚖️ LAW: one `step` never spends more than its budget, progress is monotone, and it never
/// overruns the total declared at construction. This is what lets a host stay inside an interactive
/// step ceiling: the budget, not the geometry, decides how long a call takes.
#[test]
fn a_budgeted_step_never_outruns_its_budget_and_progress_is_monotone() {
    for (body, solid) in [boxed_body(), sphere_body()] {
        let mut job = TessellationJob::for_solid(&body, solid, 0.05).expect("job");
        let total = job.progress().units_total;
        assert!(total > 3, "a real solid must declare more than one budget's worth of work, got {total}");
        let mut previous = 0;
        let mut calls = 0;
        loop {
            let before = job.progress().units_done;
            let step = job.step(&body, 3).expect("step");
            let after = job.progress().units_done;
            assert!(after >= previous, "progress went backwards: {previous} then {after}");
            assert!(after <= total, "progress {after} exceeded the declared total {total}");
            assert!(after - before <= 3, "one step spent {} units against a budget of 3", after - before);
            previous = after;
            calls += 1;
            assert!(calls < 10_000, "a bounded job must terminate");
            if matches!(step, TessellationStep::Done(_)) {
                break;
            }
        }
        assert!(calls > 1, "a budget of 3 must not finish a whole solid in one call");
        assert_eq!(job.progress().units_done, total, "a finished job must have spent every unit");
        assert_eq!(job.progress().phase, TessellationPhase::Complete);
        assert_eq!(job.progress().faces_done, job.progress().faces_total, "the faces-done/total pair the UI shows must agree at the end");
    }
}

/// ⚖️ LAW: a zero budget is a legal progress probe — it does no work and terminates nothing.
#[test]
fn a_zero_budget_step_is_a_pure_progress_probe() {
    let (body, solid) = boxed_body();
    let mut job = TessellationJob::for_solid(&body, solid, 0.05).expect("job");
    let before = job.progress();
    assert!(matches!(job.step(&body, 0).expect("step"), TessellationStep::Working(_)));
    assert_eq!(job.progress(), before, "a zero budget must change nothing");
}

/// ⚖️ LAW: the resumable path IS the algorithm — a job driven two units at a time produces a mesh
/// byte-identical to the one-shot `tessellate_solid`, so there is no second implementation to drift.
#[test]
fn stepping_produces_the_same_mesh_as_one_shot_tessellation() {
    for (body, solid) in [boxed_body(), sphere_body()] {
        let one_shot = tessellate_solid(&body, solid, 0.05).expect("one shot");
        let mut job = TessellationJob::for_solid(&body, solid, 0.05).expect("job");
        while !matches!(job.step(&body, 2).expect("step"), TessellationStep::Done(_)) {}
        let (stepped, _report) = job.into_mesh().expect("a completed job yields its mesh");
        assert_eq!(stepped.position, one_shot.position, "stepped positions must match the one-shot result");
        assert_eq!(stepped.index, one_shot.index, "stepped indices must match the one-shot result");
        assert_eq!(stepped.edges, one_shot.edges, "stepped edge polylines must match the one-shot result");
        assert_eq!(stepped.face_groups, one_shot.face_groups);
        assert_eq!(stepped.edge_groups, one_shot.edge_groups);
    }
}

/// ⚖️ LAW: every phase carries a stable, distinct wire tag — the host envelope, the session status
/// object and the UI all name the same phase by the same string.
#[test]
fn phase_tags_are_stable_and_distinct() {
    let tags: Vec<&str> = [TessellationPhase::SamplingEdges, TessellationPhase::MeshingFaces, TessellationPhase::PackingEdges, TessellationPhase::Complete, TessellationPhase::Cancelled].iter().map(|phase| phase.tag()).collect();
    assert_eq!(tags, vec!["samplingEdges", "meshingFaces", "packingEdges", "complete", "cancelled"]);
    let mut sorted = tags.clone();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted.len(), tags.len(), "phase tags must be distinct");
}
// #endregion ⏱️BudgetAndProgress

// #region 🛑️Cancellation
/// ⚖️ LAW: cancelling retires the job cleanly — terminal phase, partial buffers dropped, no mesh
/// handed out, and every later step is an inert `Cancelled` rather than a panic or a resumption.
#[test]
fn cancel_retires_a_job_without_panicking_or_producing_a_mesh() {
    let (body, solid) = sphere_body();
    let mut job = TessellationJob::for_solid(&body, solid, 0.05).expect("job");
    assert!(matches!(job.step(&body, 2).expect("step"), TessellationStep::Working(_)), "two units must not finish a sphere");
    let before = job.progress().units_done;
    job.cancel();
    assert_eq!(job.progress().phase, TessellationPhase::Cancelled);
    assert!(job.is_terminal());
    for _ in 0..3 {
        assert!(matches!(job.step(&body, 64).expect("step"), TessellationStep::Cancelled(_)), "a cancelled job must stay cancelled");
    }
    assert_eq!(job.progress().units_done, before, "a cancelled job must not keep advancing");
    assert!(job.into_mesh().is_none(), "a cancelled job must never hand out a mesh");
}

/// ⚖️ LAW: cancelling a job that already completed does NOT destroy its result — supersession is
/// only allowed to retire work that is still in flight.
#[test]
fn cancelling_a_completed_job_keeps_its_mesh() {
    let (body, solid) = boxed_body();
    let mut job = TessellationJob::for_solid(&body, solid, 0.05).expect("job");
    while !matches!(job.step(&body, 64).expect("step"), TessellationStep::Done(_)) {}
    job.cancel();
    assert_eq!(job.progress().phase, TessellationPhase::Complete, "a finished job is not cancellable");
    assert!(job.into_mesh().is_some(), "a finished job keeps its mesh through a late cancel");
}
// #endregion 🛑️Cancellation

// #region 🎚️LodCost
/// ⚖️ LAW: the three preview LOD tolerances are genuinely ordered — a finer deflection never
/// produces fewer triangles than a coarser one, and every LOD converges under a bounded number of
/// budgeted steps. Prints the per-LOD cost (steps, triangles, wall time) under `--nocapture`, which
/// is where this ticket's measured numbers come from.
#[test]
fn preview_lod_tolerances_are_ordered_and_bounded() {
    const PREVIEW_TOLERANCES: [(&str, f64); 3] = [("coarse", 0.15), ("default", 0.05), ("fine", 0.02)];
    for (label, body_and_solid) in [("box", boxed_body()), ("sphere", sphere_body())] {
        let (body, solid) = body_and_solid;
        let mut previous_triangles = 0;
        for (lod, tolerance) in PREVIEW_TOLERANCES {
            let started = std::time::Instant::now();
            let mut job = TessellationJob::for_solid(&body, solid, tolerance).expect("job");
            let mut steps = 0;
            while !matches!(job.step(&body, 24).expect("step"), TessellationStep::Done(_)) {
                steps += 1;
                assert!(steps < 10_000, "a bounded job must converge");
            }
            let elapsed = started.elapsed();
            let (mesh, report) = job.into_mesh().expect("mesh");
            let triangles = mesh.index.len() / 3;
            eprintln!("lod {label}/{lod} (deflection {tolerance}): {steps} budgeted steps, {triangles} triangles, {} vertices, {} edge floats, {:?}, max chordal {:.6}", mesh.position.len() / 3, mesh.edges.len(), elapsed, report.max_chordal);
            assert!(triangles >= previous_triangles, "{label}: {lod} produced {triangles} triangles, fewer than the coarser LOD's {previous_triangles}");
            previous_triangles = triangles;
        }
    }
}
// #endregion 🎚️LodCost

// #region 🩺️ValidateGate
/// ⚖️ LAW: a well-formed solid passes the gate, so the gate never blocks legitimate previews.
#[test]
fn the_validate_gate_admits_a_well_formed_solid() {
    let mut kernel = Brep::new();
    let handle = kernel.box_prim(1.0, 1.0, 1.0).expect("box");
    assert_eq!(kernel.validate_gate_sync(&handle), Ok(()), "a healthy box must reach the tessellator");
    let job = kernel.tessellate_job_sync(&handle, 0.05).expect("job");
    assert!(job.progress().units_total > 0, "an admitted solid must declare real work");
}

/// ⚖️ LAW: a handle the kernel does not know is rejected by the gate with a typed, machine-readable
/// code — never a panic and never a silent empty mesh.
#[test]
fn the_validate_gate_rejects_an_unknown_handle_with_a_typed_code() {
    let kernel = Brep::new();
    let issues = kernel.validate_gate_sync(&GeometryHandle("solid-does-not-exist".to_string())).expect_err("an unknown handle must not pass the gate");
    assert_eq!(issues.len(), 1);
    assert_eq!(issues[0].code, "unknown-handle");
    assert!(issues[0].message.contains("solid-does-not-exist"));
}

/// ⚖️ LAW: the gate only judges solid-like shapes — a curve or wire preview has no shell to close,
/// so it passes through untouched instead of being blocked by an unrelated body-level finding.
#[test]
fn the_validate_gate_passes_non_solid_previews_through() {
    let mut kernel = Brep::new();
    let wire = kernel.rectangle_wire(2.0, 1.0).expect("rectangle wire");
    assert_eq!(kernel.validate_gate_sync(&wire), Ok(()), "a wire preview must not be gated on solid validity");
}
// #endregion 🩺️ValidateGate
