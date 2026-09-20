//! 🔢️ The frame-generation hold and the ONE presentation authority the build and the presenter
//! share, driven by the language-agnostic oracle `🧫️fixtures/🔢️frame-generation-hold/🔣️.json` that
//! `🧪️tests/🔢️frame-generation-hold/🟦️.ts` re-derives independently.
//!
//! Everything the transcript touches is production code: the mounted input callbacks
//! ([`enqueue_host_event`], [`enqueue_host_metrics`], [`advance_frame_generation`]) with the real
//! [`FrameGenerationHold`], and the real [`crate::RuntimePresentationAuthority`] whose live and
//! admitted pairs decide whether a finished packet is presented or the surface is quarantined.
//!
//! The two defects this pins, both measured on 6118
//! (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-edit-convergence-perf-2026-09-14.md` §6):
//! every pointer move renumbered the generation and cancelled the in-flight build from phase 0
//! (28 supersessions per converging edit, 26 % of the wall clock); and holding the number ALONE let
//! the build finish into a presenter that re-read a moving authority and answered
//! `prepared render revision is stale: live=35, packet=27` → `worker-present-failed`.

use super::*;
use ui_render::{PointerId, PointerInfo, PointerKind};

#[derive(serde::Deserialize)]
struct StepFixture {
    op: String,
    #[serde(default)]
    repeat: Option<u32>,
}

#[derive(serde::Deserialize)]
struct ExpectFixture {
    generation: u64,
    delivered: u32,
    supersessions: u32,
    #[serde(rename = "framesPresented")]
    frames_presented: u32,
    quarantines: u32,
}

#[derive(serde::Deserialize)]
struct RowFixture {
    id: String,
    steps: Vec<StepFixture>,
    expect: ExpectFixture,
    #[serde(default, rename = "quarantinesIfPresenterRereadsLiveAuthority")]
    quarantines_if_presenter_rereads_live_authority: u32,
}

#[derive(serde::Deserialize)]
struct HoldFixture {
    rows: Vec<RowFixture>,
    laws: Vec<String>,
}

fn fixture() -> HoldFixture {
    serde_json::from_str(include_str!("../../🧫️fixtures/🔢️frame-generation-hold/🔣️.json")).expect("frame-generation-hold fixture JSON")
}

#[derive(Default)]
struct Tally {
    delivered: u32,
    supersessions: u32,
    frames_presented: u32,
    quarantines: u32,
    quarantines_against_live: u32,
}

/// 🎬️ One host loop, replayed from the transcript against the real callbacks and the real authority.
fn replay(row: &RowFixture) -> (u64, Tally) {
    let mut events = ui_host::EventQueue::new();
    let mut scheduler = ui_render::FrameScheduler::new();
    let token = ui_host::UiThreadToken::mint_for_host();
    let pointer = PointerInfo { id: PointerId(1), kind: PointerKind::Mouse, pressure: None, tilt: None };
    let authority = crate::RuntimePresentationAuthority::new();
    let mut generation = 0u64;
    let mut live_build: Option<(crate::RuntimePresentationWitness, u64)> = None;
    let mut pending_packet: Option<crate::RuntimePresentationWitness> = None;
    let mut tally = Tally::default();
    for step in &row.steps {
        for index in 0..step.repeat.unwrap_or(1) {
            let hold = if live_build.is_some() { FrameGenerationHold::UnderLiveBuild } else { FrameGenerationHold::Free };
            match step.op.as_str() {
                "input" => {
                    assert_eq!(enqueue_host_event(&mut events, &mut scheduler, token, &mut generation, hold, DispatchEvent::PointerMove { pointer, x: index as f32, y: 0.0, modifiers: ui_render::EventModifiers::default() }), ui_host::EnqueueOutcome::Accepted, "{}: input {index} was refused", row.id);
                    tally.delivered += 1;
                }
                "metrics" => {
                    enqueue_host_metrics(&mut events, &mut scheduler, token, &mut generation, hold, 800 + index % 32, 600 + index % 32, 2.0);
                    tally.delivered += 1;
                }
                "sceneChange" => authority.mark_scene_changed(),
                "redraw" => {
                    if live_build.is_none() {
                        assert!(advance_frame_generation(&mut generation), "{}: frame generation exhausted", row.id);
                    }
                    authority.observe_input_generation(generation);
                }
                "forceRenumber" => {
                    assert!(advance_frame_generation(&mut generation), "{}: frame generation exhausted", row.id);
                    authority.observe_input_generation(generation);
                }
                "buildAdmit" => {
                    assert!(live_build.is_none(), "{}: two builds admitted at once", row.id);
                    authority.observe_input_generation(generation);
                    let witness = authority.witness_for(generation).expect("the authority publishes the generation a build is admitted at");
                    authority.admit_build(witness);
                    live_build = Some((witness, generation));
                }
                "buildComplete" => {
                    let Some((witness, build_generation)) = live_build.take() else { panic!("{}: no build to complete", row.id) };
                    if build_generation == generation {
                        pending_packet = Some(witness);
                    } else {
                        tally.supersessions += 1;
                    }
                }
                "present" => {
                    if let Some(packet) = pending_packet.take() {
                        if packet == authority.admitted() {
                            tally.frames_presented += 1;
                        } else {
                            tally.quarantines += 1;
                        }
                        if packet != authority.current() {
                            tally.quarantines_against_live += 1;
                        }
                    }
                }
                other => panic!("{}: unknown transcript operation {other}", row.id),
            }
        }
    }
    (generation, tally)
}

/// ⚖️ Every transcript, against the real callbacks and the real authority.
#[test]
fn the_frame_generation_hold_and_the_shared_authority_answer_the_oracle() {
    let fixture = fixture();
    for row in &fixture.rows {
        let (generation, tally) = replay(row);
        assert_eq!(generation, row.expect.generation, "generation for {}", row.id);
        assert_eq!(tally.delivered, row.expect.delivered, "delivered for {}", row.id);
        assert_eq!(tally.supersessions, row.expect.supersessions, "supersessions for {}", row.id);
        assert_eq!(tally.frames_presented, row.expect.frames_presented, "frames presented for {}", row.id);
        assert_eq!(tally.quarantines, row.expect.quarantines, "quarantines for {}", row.id);
    }
}

/// 🩸️ The counterfactual the oracle declares: a presenter that re-read the LIVE authority instead of
/// the pair its build was admitted under refuses a correctly-built packet and quarantines the
/// surface. Without this the fix above would be indistinguishable from a gate that never fires.
#[test]
fn re_reading_the_live_authority_at_admission_is_the_measured_surface_fault() {
    let fixture = fixture();
    for row in &fixture.rows {
        let (_, tally) = replay(row);
        assert_eq!(tally.quarantines_against_live, row.quarantines_if_presenter_rereads_live_authority, "live-authority quarantines for {}", row.id);
    }
}

/// 📜️ The oracle's own law list — an empty one would make every assertion above vacuous.
#[test]
fn the_oracle_declares_its_laws() {
    let fixture = fixture();
    assert_eq!(fixture.laws.len(), 5);
    assert!(fixture.laws.iter().any(|law| law == "an-input-event-never-renumbers-the-frame-generation-underneath-a-live-build"));
    assert!(fixture.laws.iter().any(|law| law == "the-presenter-admits-a-packet-against-the-pair-its-build-was-admitted-under"));
}
