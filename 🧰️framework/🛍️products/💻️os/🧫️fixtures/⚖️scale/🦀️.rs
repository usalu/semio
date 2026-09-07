//! 🧫️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (F1-scale-fixture, design-workforce.md §3): the one
//! parametric wasm32-wasip2 `world actor` component every generated scale-fixture registry record
//! points at. ONE binary, ONE build — which of the seven synthetic behaviours it exhibits is chosen
//! at `instance-open` time by the JSON-encoded `config` pack the host derives from the manifest's
//! `scaleFixture.profile` field (see the TypeScript generator, `🧑‍💻dev/📦️packages/🟦️typescript/
//! 📜️script.ts`'s `#region 🔖️ScaleFixture`), never by a Cargo feature or a separate build.
//!
//! `idle` (activates, no work) · `cpu` (busy-loop `cpuBusyMs` per turn) · `ui` (`uiPatchesPerTurn`
//! revisioned patches per turn) · `io` (requests one capability, waits for the grant/denial) ·
//! `hang` (burns `hangOverrunMultiplier`× its own declared deadline before returning — the host
//! watchdog is what is supposed to catch this, not this crate) · `crash` (traps on turn
//! `crashAfterTurns`) · `stateful` (accumulates one byte per turn; `checkpoint`/`restore` round-trip
//! that accumulator byte-for-byte). The actual behaviour lives in `🎭️profile` — a plain, WIT-free
//! state machine unit-tested on the host target; this file is ONLY the WIT<->plain bridge.
//!
//! Own `wit-bindgen generate!` + own `Guest` impls + own `export!`, deliberately NOT a dependency on
//! `semio-framework-plugin` — that SDK's `component` module (`🔌️plugin/🦀️.rs`) wires the
//! SAME `world actor` for real production plugins via its declarative `Plugin::builder`/
//! `plugin_exports!` app machinery (commands, windows, panels), which this synthetic actor has no
//! use for. This file follows the SDK's *pattern* (generate!/Guest impls/export!, the "only types
//! named directly in `reactor.wit`'s own signature get aliased under `exports::…`" rule) without
//! inheriting its weight — see `📓️terra-A2b-bridge-report.md` §1 for that rule, re-verified here
//! against this crate's own `cargo check` output rather than assumed.

#![allow(unsafe_op_in_unsafe_fn)]

#[path = "🎭️profile/🦀️.rs"]
pub mod profile;

//#region 🔖️Wit
#[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
pub mod guest {
    //! 🧩️ WASI P2 component exports for `world actor` — see the module-level doc above for why this
    //! crate does its own `generate!` instead of depending on `semio-framework-plugin`.
    use wit_bindgen::generate;

    generate!({
        world: "actor",
        path: "../../../../🔨️modules/🔌️plugin/🧬️schema",
    });

    use crate::profile::{self, PlainEffect, TurnBudget, TurnOutcome};
    use std::cell::RefCell;
    use exports::semio::framework::checkpoint::Guest as CheckpointGuest;
    use exports::semio::framework::describe::Guest as DescribeGuest;
    use exports::semio::framework::jobs::{Guest as JobsGuest, JobBudget, JobStep};
    use exports::semio::framework::reactor::{Budget as WitBudget, Effect as WitEffect, Event as WitEvent, Guest as ReactorGuest, TurnResult as WitTurnResult, TurnStatus as WitTurnStatus};
    use semio::framework::capabilities::CapabilityChange;
    use semio::framework::effects::{RequestCapabilityEffect, RequestCapabilityParams};
    use semio::framework::events::CompletionResult;
    use semio::framework::instance_lifetime::{CapturedReceipt, Lifetime, Receipt, UiPatchReceipt};
    use semio::framework::types::PluginError;
    use semio::framework::ui::{PatchOp, PatchUpsert, SurfaceRef, UiPatch};

    pub struct FixtureGuest;

    #[derive(Clone, Copy, Default)]
    struct UiAuthority {
        lifetime: Option<Lifetime>,
        captured: Option<CapturedReceipt>,
        live: bool,
        patch_sequence: u64,
    }

    thread_local! {
        static UI_AUTHORITY: RefCell<UiAuthority> = RefCell::new(UiAuthority::default());
    }

    fn capture_ui_authority(open: &semio::framework::events::InstanceOpenEvent) -> Receipt {
        let lifetime = Lifetime { activation_generation: open.activation_generation, instance_id: open.instance, guest_lifetime: 1 };
        let captured = CapturedReceipt { lifetime, request_sequence: open.request_sequence };
        UI_AUTHORITY.with(|slot| *slot.borrow_mut() = UiAuthority { lifetime: Some(lifetime), captured: Some(captured), live: false, patch_sequence: 0 });
        Receipt::Captured(captured)
    }

    fn accept_ui_authority(receipt: &Receipt) -> bool {
        UI_AUTHORITY.with(|slot| {
            let mut authority = slot.borrow_mut();
            if authority.captured.as_ref().is_some_and(|expected| {
                matches!(
                    receipt,
                    Receipt::Captured(actual)
                        if actual.request_sequence == expected.request_sequence
                            && actual.lifetime.activation_generation == expected.lifetime.activation_generation
                            && actual.lifetime.instance_id == expected.lifetime.instance_id
                            && actual.lifetime.guest_lifetime == expected.lifetime.guest_lifetime
                )
            }) {
                authority.live = true;
                true
            } else {
                false
            }
        })
    }

    fn issue_ui_patch(imported: bool, malformed: bool) -> Option<(UiPatch, UiPatchReceipt)> {
        UI_AUTHORITY.with(|slot| {
            let mut authority = slot.borrow_mut();
            if !authority.live {
                return None;
            }
            let lifetime = authority.lifetime?;
            authority.patch_sequence = authority.patch_sequence.checked_add(1)?;
            let sequence = authority.patch_sequence;
            let operation = if malformed && sequence == 1 { PatchOp::Upsert(PatchUpsert { node: vec![255] }) } else { PatchOp::SetRoot(if malformed { 300 + sequence } else if imported { 100 + sequence } else { 200 + sequence }) };
            let patch = UiPatch {
                surface: SurfaceRef { instance: lifetime.instance_id, surface: "map.main".to_string() },
                revision: sequence,
                base_revision: sequence.saturating_sub(1),
                ops: vec![operation],
            };
            Some((patch, UiPatchReceipt { lifetime, patch_sequence: sequence }))
        })
    }

    fn command_ingress_status(page: Option<exports::semio::framework::reactor::CommandIngressPage>) -> exports::semio::framework::reactor::CommandIngressStatus {
        use exports::semio::framework::reactor::{CommandIngressStatus, CommandPageCursor};
        page.map_or_else(
            || CommandIngressStatus {
                kind: 0,
                cursor: CommandPageCursor { owner: 0, generation: 0, command_index: 0, command_count: 0, instance: 0, seq: 0, kind: 0, page_index: 0, page_count: 0, item_count: 0, metadata: 0 },
                fault: Vec::new(),
            },
            |page| CommandIngressStatus { kind: 2, cursor: page.cursor, fault: Vec::new() },
        )
    }

    // 🧬️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (B1 world-collapse): every `Guest` method mirrors
    // the WIT's `async func` — `world actor`'s seven exports all went async together when it gained
    // its `host-async` import (a sync export is uncallable on the async-configured host Store, S7).
    // None of these bodies actually suspends: this fixture is pure in-memory profile bookkeeping and
    // calls no `host-async` import at all, which is deliberate — the bench measures the RUNTIME, so
    // the guest must not add work of its own.
    impl ReactorGuest for FixtureGuest {
        async fn poll(
            events: Vec<WitEvent>,
            command_page: Option<exports::semio::framework::reactor::CommandIngressPage>,
            _cold_pair_page: Option<exports::semio::framework::reactor::ColdDocumentPairPage>,
            budget: WitBudget,
        ) -> Result<WitTurnResult, PluginError> {
            let mut lifecycle_receipt = None;
            let mut lifecycle_turn = false;
            for event in events {
                match event {
                    WitEvent::InstanceOpen(open) => {
                        profile::on_instance_open(&open.config);
                        lifecycle_receipt = Some(capture_ui_authority(&open));
                        lifecycle_turn = true;
                    }
                    WitEvent::InstanceLifecycleAck(receipt) => {
                        accept_ui_authority(&receipt);
                        lifecycle_turn = true;
                    }
                    WitEvent::Completed(completed) => {
                        if matches!(completed.outcome, CompletionResult::Ok(_) | CompletionResult::Fault(_)) {
                            profile::on_completed(completed.req);
                        }
                    }
                    // 🔻️ Bench budget 8 (design-workforce.md §4): "capability revoked at runtime ->
                    // denied completion, actor stays alive, quota counters zero" — only `revoked`
                    // needs a reaction here (`granted`/`narrowed` don't change what the `io` profile
                    // already assumed once its `event.completed` arrived).
                    WitEvent::CapabilityChanged(changed) => {
                        if let CapabilityChange::Revoked(id) = changed.change {
                            profile::on_capability_revoked(&id);
                        }
                    }
                    _ => {}
                }
            }

            let turn_budget = TurnBudget { fuel: budget.fuel, deadline_ms: budget.deadline_ms, max_effects: budget.max_effects, max_patch_bytes: budget.max_patch_bytes, max_frames: budget.max_frames };
            let outcome = if lifecycle_turn { TurnOutcome::default() } else { profile::turn(turn_budget, now_ms) };
            let (import_sink, return_with_import, malformed_first) = profile::ui_boundary_mode();
            let issued = if outcome.patches.is_empty() { None } else { issue_ui_patch(import_sink, malformed_first) };
            let mut ui_patch_receipt = None;
            let mut ui_patches = Vec::new();
            if let Some((patch, receipt)) = issued {
                ui_patch_receipt = Some(receipt);
                if import_sink {
                    semio::framework::host_async::emit_patch(&patch);
                }
                if !import_sink || return_with_import {
                    let returned = if import_sink {
                        UiPatch { surface: patch.surface.clone(), revision: patch.revision, base_revision: patch.base_revision, ops: vec![PatchOp::SetRoot(900 + receipt.patch_sequence)] }
                    } else {
                        patch
                    };
                    ui_patches.push(returned);
                }
            }

            let effects = outcome
                .effects
                .into_iter()
                .map(|effect| match effect {
                    PlainEffect::RequestCapability { req, id, scope, reason, optional } => WitEffect::RequestCapability(RequestCapabilityEffect { req, params: RequestCapabilityParams { id, scope, reason, optional } }),
                })
                .collect();

            Ok(WitTurnResult {
                ui_patches,
                effects,
                // 👥️ This fixture is pure synthetic profile bookkeeping with no interaction state,
                // so it genuinely emits no render-plane presence — an empty list is the correct
                // value here, not a placeholder.
                presence: Vec::new(),
                next_wake: outcome.next_wake_ms,
                status: if outcome.status_more_work { WitTurnStatus::MoreWork } else { WitTurnStatus::Idle },
                fuel_used: outcome.fuel_used,
                command_ingress: command_ingress_status(command_page),
                cold_pair_ingress: exports::semio::framework::reactor::ColdPairIngressStatus::Idle,
                lifecycle_receipt,
                ui_patch_receipt,
            })
        }
    }

    impl JobsGuest for FixtureGuest {
        async fn start_job(job: u64, kind: String, input: Vec<u8>) -> Result<(), PluginError> {
            profile::jobs::start_job(job, &kind, input);
            Ok(())
        }

        async fn step_job(job: u64, _budget: JobBudget) -> Result<JobStep, PluginError> {
            Ok(match profile::jobs::step_job(job) {
                profile::jobs::JobOutcome::Done(bytes) => JobStep::Done(bytes),
                profile::jobs::JobOutcome::Failed(bytes) => JobStep::Failed(bytes),
            })
        }

        async fn cancel_job(job: u64) {
            profile::jobs::cancel_job(job);
        }

        async fn take_segmented_download_chunk(_instance_id: u32, _operation_id: u64) -> Result<Option<Vec<u8>>, PluginError> {
            Ok(None)
        }
    }

    impl CheckpointGuest for FixtureGuest {
        async fn checkpoint() -> Result<Vec<u8>, PluginError> {
            profile::checkpoint().map_err(|message| PluginError::Fault(message.into_bytes()))
        }

        async fn restore(state: Vec<u8>) -> Result<(), PluginError> {
            profile::restore(&state).map_err(|message| PluginError::Fault(message.into_bytes()))
        }
    }

    impl DescribeGuest for FixtureGuest {
        async fn describe() -> Vec<u8> {
            // 🚧️ Placeholder — a real packed `PackageDescriptor` is packet E1-describe's job. F1
            // only proves `reactor`/`jobs`/`checkpoint`/`describe` are wired and the crate builds
            // for `wasm32-wasip2`; nothing in this ticket's V1 bench reads this actor's `describe()`
            // output (the generated `🔣️registry.json`/`🔣️catalog.json` are what the bench parses).
            Vec::new()
        }
    }

    export!(FixtureGuest);

    pub fn now_ms() -> i64 {
        semio::framework::pure::now_ms()
    }
}

/// 🕰️ `pure::now-ms` outside a real wasm32-wasip2 component-guest build (native `cargo check
/// --all-targets`, unit tests) — wall-clock time, so `cpu`/`hang` unit tests measure real elapsed
/// milliseconds the same way the guest export does.
#[cfg(not(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2")))]
pub fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_millis() as i64).unwrap_or(0)
}

#[cfg(all(feature = "component-guest", target_arch = "wasm32", target_env = "p2"))]
pub use guest::now_ms;
//#endregion 🔖️Wit
