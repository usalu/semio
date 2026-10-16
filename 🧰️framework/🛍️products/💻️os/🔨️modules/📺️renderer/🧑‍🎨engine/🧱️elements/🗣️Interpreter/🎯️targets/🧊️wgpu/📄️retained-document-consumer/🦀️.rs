//! 📄️ Retained document consumer.
//!
//! The retained document frame cursor, the flow setter and the render step that consumes one retained UI document.

use semio_framework_os_renderer_wgpu_ui_engine_focus::ink_and_vfs_focus::close_retiring_focus_clipboard_one;
use crate::retained_ui_engine::{ui_document_close_pending_for, UI_ENGINE};
use crate::scene_interaction::{drive_mounted_layout_text_one, layout_step_is_window_work, FrameworkSceneHost};
use crate::element_contracts::{FrameworkWidgetContext, ScenePointerTarget};
use std::sync::atomic::{AtomicBool, Ordering};
use ui_contract::UiDocumentLease;
use ui_wgpu::wgpu::Rect;

//#region 📄️RetainedDocumentConsumer
static DOCUMENT_PAGE_OPPORTUNITY_CONSUMED: AtomicBool = AtomicBool::new(false);

#[derive(Clone, Copy, Default)]
enum UiDocumentFramePhase {
    #[default]
    Ingress,
    /// 🌳️ Projects the published `UiDocumentTree`'s records into the engine's paintable arena. Sits
    /// between ingress and viewport because layout reads the arena's own parent/child links, which
    /// do not exist until this phase has run (`📓️wgpu-blank-paint-2026-09-12.md` §5).
    Reconcile,
    Viewport,
    Layout,
    Paint,
    Complete,
    Fault,
}

/// 📏️ Opportunities one document's PAINT phase may stay `Pending` before the stall is named. Well
/// under `SHELL_WINDOW_PAINT_OPPORTUNITIES`, so the console says WHICH engine paint phase is parked
/// long before the chrome walk gives up on the window.
const UI_DOCUMENT_PAINT_STALL_NOTICE: u32 = 65_536;

#[derive(Default)]
pub struct UiDocumentFrameCursor {
    pub phase: UiDocumentFramePhase,
    /// 🩺️ Consecutive `Pending` paint opportunities, reset whenever the phase changes.
    stalled: u32,
    /// ⏱️ Whether the last step advanced THIS document's own ladder ([`Self::last_step_was_own_work`]).
    own_work: bool,
}

impl UiDocumentFrameCursor {
    /// ⏱️ Whether the last step was this document's own work — false when it only ran the shared retirement lanes, only waited on
    /// the layout worker pool ([`ui_wgpu::wgpu::UiLayoutStep::Awaiting`]) or advanced another surface's layout. A host's
    /// non-convergence budget spends an opportunity only on own work, so a slow worker or a busy neighbour never faults a document
    /// that is converging.
    pub fn last_step_was_own_work(&self) -> bool {
        self.own_work
    }

    pub fn terminal_is_complete(&self) -> bool {
        matches!(self.phase, UiDocumentFramePhase::Complete)
    }

    pub fn terminal_is_fault(&self) -> bool {
        matches!(self.phase, UiDocumentFramePhase::Fault)
    }

    /// 🩺️ Names the phase a per-frame document cursor is parked in, so a paint that burns its whole
    /// opportunity budget reports WHICH of the six phases refused to advance instead of only that it
    /// did (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn phase_name(&self) -> &'static str {
        match self.phase {
            UiDocumentFramePhase::Ingress => "ingress",
            UiDocumentFramePhase::Reconcile => "reconcile",
            UiDocumentFramePhase::Viewport => "viewport",
            UiDocumentFramePhase::Layout => "layout",
            UiDocumentFramePhase::Paint => "paint",
            UiDocumentFramePhase::Complete => "complete",
            UiDocumentFramePhase::Fault => "fault",
        }
    }

    /// 📐️ Whether intrinsic layout is accepted and the cursor has reached retained paint.
    pub fn layout_is_accepted(&self) -> bool {
        matches!(self.phase, UiDocumentFramePhase::Paint | UiDocumentFramePhase::Complete)
    }

    /// 🧭️ Re-solves accepted content against the compact host box before its first paint.
    pub fn restart_viewport_after_host_reflow(&mut self) {
        if matches!(self.phase, UiDocumentFramePhase::Paint) {
            self.phase = UiDocumentFramePhase::Viewport;
            self.stalled = 0;
        }
    }
}

/// 🧭️ Sets one retained surface's logical flow — the wgpu twin of React's `FlowProvider`, which
/// `🖼️Panel/🟦️.tsx:516` wraps every anchored panel in with `flowFromAnchor(anchor)`. A surface with
/// no anchor (a window body, a dialog) gets no provider on React either and stays at
/// `UiFlow::DEFAULT`, so only the anchored-panel painters call this.
///
/// `Ui::set_window_flow` existed with ZERO production callers until ticket 26/09/17 packet W15a:
/// every surface sat at the default flow, so the RTL vocabulary W2k built (mirrored overlay
/// placement, mirrored Select inline edge, mirrored inline arrow keys) could never fire. It is
/// latent rather than visible today — both shipped locales are LTR, and an `End`-anchored panel is
/// the only thing that turns it on — but the seam is now real instead of dead.
pub fn set_ui_document_flow(window_id: &str, flow: ui_contract::UiFlow) {
    UI_ENGINE.with(|cell| cell.borrow_mut().set_window_flow(window_id, flow));
}

/// 👥️ Hands every retained window the peers' notes by record key (sorted by key) — the wgpu twin of the host presence
/// overlay React provides at the shell root. `true` when any window's notes changed.
pub fn set_ui_presence_notes(notes: &[(String, String)]) -> bool {
    UI_ENGINE.with(|cell| cell.borrow_mut().set_presence_notes(notes))
}

/// 🎯️ Stamps one reconciled Tree candidate's host-owned selected row through the retained presence
/// channel. Structural document records stay immutable and a stale document generation is refused.
pub fn stamp_ui_document_tree_selected_item(window_id: &str, generation: u64, selected_key: &str) -> bool {
    UI_ENGINE.with(|cell| cell.borrow_mut().stamp_tree_selected_item(window_id, generation, selected_key))
}

pub fn begin_ui_document_opportunity(consumed: bool) {
    DOCUMENT_PAGE_OPPORTUNITY_CONSUMED.store(consumed, Ordering::Release);
}

/// 🩺️ Console line for a retained-document terminal fault. `eprintln!` is a no-op inside a
/// `wasm32-unknown-unknown` Worker, so a fault recorded there left no trace at all.
pub fn document_debug_log(line: &str) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&wasm_bindgen::JsValue::from_str(line));
    #[cfg(not(target_arch = "wasm32"))]
    eprintln!("{line}");
}

pub fn apply_retained_document_page(engine: &mut ui_wgpu::wgpu::Ui, cursor: &mut UiDocumentFrameCursor, window_id: &str, page: ui_contract::UiDocumentNodePage, step: &mut semio_framework_job::StepContext<'_>) {
    if let Err(rejection) = engine.apply_document_page(window_id, page, step) {
        if rejection.fault == ui_wgpu::wgpu::tree::UiDocumentTreeFault::Generation && (step.is_cancelled() || step.should_yield()) {
            return;
        }
        document_debug_log(&format!("[TRACE] ui-doc page fault window={window_id} generation={} index={} fault={:?} cancelled={} yielded={}", rejection.generation, rejection.index, rejection.fault, step.is_cancelled(), step.should_yield()));
        cursor.phase = UiDocumentFramePhase::Fault;
    }
}

/// 🧹️ Transfers one exact retired component before document reconcile can resume.
fn forward_retired_component_scene_one(window_id: &str) -> bool {
    let retirement = UI_ENGINE.with(|cell| cell.borrow().retired_component_scene(window_id));
    let Some(retired) = retirement else { return true };
    if !semio_framework_os_renderer_wgpu_scene_state::scene_surface_retirement_can_admit() {
        return false;
    }
    let owner = ScenePointerTarget::from(retired.clone());
    let _retained = semio_framework_os_renderer_wgpu_scene_state::retire_scene_identity(&owner);
    #[cfg(any(test, feature = "testkit"))]
    eprintln!("[TRACE] scene retirement forward window={} host={} kind={:?} retained={_retained}", owner.window_id, owner.host_id, owner.kind);
    UI_ENGINE.with(|cell| {
        assert!(cell.borrow_mut().acknowledge_retired_component_scene(window_id, &retired), "the exact retired component scene head changed before acknowledgement");
    });
    false
}

/// 🚦️ A refused ingress opportunity is a RETRY, never a fault. `begin_document`,
/// `apply_document_page` and `finish_document` all refuse a step whose `StepContext` is
/// cancelled or out of budget, and this arm used to read every refusal as the terminal
/// `Fault` phase — one `Deadline` on the very first opportunity froze the surface's page
/// ingress for the life of the shell (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-blank-paint-2026-09-12.md`). The budget is checked HERE instead, so an
/// opportunity that cannot pay is simply not spent and the next one resumes the same page.
///
/// 🩺️ One line per ADMITTED ingress — never per page, never per frame. A surface that
/// republishes an unchanged document answers `Published` and stays silent, so this line is
/// exactly "surface X started ingesting a new document", the fact whose ABSENCE was the
/// whole defect: every document after a surface's first was answered `Published` and never
/// reached the arena (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// 🌳️ The one production writer of the paintable arena: `Ui::apply_tree` is
/// `cfg(test/testkit)`, so without this phase `tree.root` is `None` forever and
/// `frame_into_step` answers `Missing` before it reads anything else.
///
/// 📐️ Advances on THIS window's own predicate, never on the layout queue's verdict.
///
/// 🩸️ `step_layouts` answers for the queue: `Idle` means it was empty and `Ready { .. }`
/// may name another surface. Reading either as "my layout is done" moved the cursor to
/// `Paint` against a still-dirty root, where `frame_into_step` short-circuits `Pending`
/// before it ever opens a paint frame — measured on 6118 as `procedural-main` burning all
/// 1 048 576 window-paint opportunities, twice a minute, which starved every OTHER
/// window's paint (the preview's World3d snapshot-apply cursor advanced three times in
/// 120 s) (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
///
/// 🔁️ A paint that is waiting on a layout is not a paint step at all: the root
/// went dirty again (a viewport change, a theme propagation, a fresh reconcile),
/// and only the Layout phase can re-arm the lane that clears it.
pub fn render_ui_document_step(
    cursor: &mut UiDocumentFrameCursor,
    document: &UiDocumentLease,
    bounds: Rect,
    ctx: &mut FrameworkWidgetContext<'_>,
    window_id: &str,
    controller_id: &str,
    driver_drag: ui_wgpu::wgpu::UiDriverDrag,
    hosts: &mut semio_framework_os_renderer_wgpu_scene_state::SceneEngineHosts<'_>,
    step:&mut semio_framework_job::StepContext<'_>,
) -> bool {
    cursor.own_work = false;
    if step.should_yield(){return false;}
    if close_retiring_focus_clipboard_one() {
        return false;
    }
    if semio_framework_os_renderer_wgpu_scene_state::close_retired_scene_surface_one() {
        return false;
    }
    if !forward_retired_component_scene_one(window_id) {
        return false;
    }
    if ui_document_close_pending_for(window_id) {
        cursor.own_work = true;
        return false;
    }
    let Ok(header) = document.header() else {
        cursor.phase = UiDocumentFramePhase::Fault;
        return false;
    };
    let generation = header.generation;
    let viewport_w = bounds.w.max(1.0);
    let viewport_h = bounds.h.max(1.0);
    cursor.own_work = !matches!(cursor.phase, UiDocumentFramePhase::Layout);
    UI_ENGINE.with(|cell| {
        let mut engine = cell.borrow_mut();
        engine.set_driver_drag(driver_drag);
        match cursor.phase {
            UiDocumentFramePhase::Ingress if step.is_cancelled() || step.should_yield() => {}
            UiDocumentFramePhase::Ingress => match {
                let status = engine.document_status(window_id, generation);
                if status == ui_wgpu::wgpu::engine::UiDocumentIngressStatus::Vacant {
                    document_debug_log(&format!("[TRACE] ui-doc ingress window={window_id} generation={generation} nodes={}", header.node_count));
                }
                status
            } {
                ui_wgpu::wgpu::engine::UiDocumentIngressStatus::Vacant => {
                    if let Err((_fault, _header)) = engine.begin_document(window_id, header, step) {
                        document_debug_log(&format!("[TRACE] ui-doc begin refused window={window_id} generation={generation} nodes={} fault={_fault:?}", _header.node_count));
                        if !matches!(
                            _fault,
                            ui_wgpu::wgpu::engine::UiDocumentIngressFault::Cancelled
                                | ui_wgpu::wgpu::engine::UiDocumentIngressFault::Deadline
                                | ui_wgpu::wgpu::engine::UiDocumentIngressFault::InterruptedClose
                                | ui_wgpu::wgpu::engine::UiDocumentIngressFault::ValidationPending
                        ) {
                            cursor.phase = UiDocumentFramePhase::Fault;
                        }
                    }
                }
                ui_wgpu::wgpu::engine::UiDocumentIngressStatus::Pending { next_page, node_count } => {
                    if next_page == node_count {
                        match engine.finish_document(window_id, generation, step) {
                            Ok(()) => cursor.phase = UiDocumentFramePhase::Reconcile,
                            Err(
                                ui_wgpu::wgpu::engine::UiDocumentIngressFault::ValidationPending
                                | ui_wgpu::wgpu::engine::UiDocumentIngressFault::InterruptedClose
                                | ui_wgpu::wgpu::engine::UiDocumentIngressFault::Cancelled
                                | ui_wgpu::wgpu::engine::UiDocumentIngressFault::Deadline,
                            ) => {}
                            Err(_fault) => {
                                document_debug_log(&format!("[TRACE] ui-doc finish fault window={window_id} generation={generation} fault={_fault:?}"));
                                cursor.phase = UiDocumentFramePhase::Fault;
                            }
                        }
                    } else {
                        match document.read_node_page(next_page) {
                            Ok(Some(page)) => {
                                apply_retained_document_page(&mut engine, cursor, window_id, page, step);
                            }
                            _ => {
                                document_debug_log(&format!("[TRACE] ui-doc page read fault window={window_id} generation={generation} index={next_page}"));
                                cursor.phase = UiDocumentFramePhase::Fault;
                            }
                        }
                    }
                }
                ui_wgpu::wgpu::engine::UiDocumentIngressStatus::Published => cursor.phase = UiDocumentFramePhase::Reconcile,
            },
            UiDocumentFramePhase::Reconcile => match engine.step_document_reconcile(window_id, controller_id, step) {
                ui_wgpu::wgpu::reconcile::UiDocumentReconcileStep::Complete => cursor.phase = UiDocumentFramePhase::Viewport,
                ui_wgpu::wgpu::reconcile::UiDocumentReconcileStep::Pending => {}
                ui_wgpu::wgpu::reconcile::UiDocumentReconcileStep::Fault(fault) => {
                    document_debug_log(&format!("[TRACE] ui-doc reconcile fault window={window_id} fault={fault:?}"));
                    cursor.phase = UiDocumentFramePhase::Fault;
                }
            },
            UiDocumentFramePhase::Viewport => {
                engine.set_theme(*ctx.theme);
                engine.set_viewport(window_id, viewport_w, viewport_h);
                cursor.phase = UiDocumentFramePhase::Layout;
            }
            UiDocumentFramePhase::Layout => {
                let layout = drive_mounted_layout_text_one(&mut engine,ctx.atlas,step);
                cursor.own_work = layout_step_is_window_work(&layout, window_id);
                if let ui_wgpu::wgpu::UiLayoutStep::Refused { refusal, .. } = layout {
                    document_debug_log(&format!("[TRACE] ui-doc retained layout refusal window={window_id} stage={} kind={}", refusal.stage, refusal.kind.as_str()));
                    cursor.phase = UiDocumentFramePhase::Fault;
                } else if engine.layout_is_dirty(window_id) {
                    engine.request_layout(window_id);
                } else {
                    cursor.phase = UiDocumentFramePhase::Paint;
                    cursor.own_work = true;
                }
            }
            UiDocumentFramePhase::Paint => {
                let progress_before = engine.paint_frame_progress(window_id);
                let document_generation = engine.tree_revision(window_id).unwrap_or(generation);
                let mut scene_host = FrameworkSceneHost {
                    input: ctx.input,
                    theme: ctx.theme,
                    scroll_offsets: ctx.scroll_offsets,
                    collapsed_sections: ctx.collapsed_sections,
                    open_selects: ctx.open_selects,
                    chrome_labels: hosts.chrome_labels,
                    window_id,
                    driver_drag,
                    document_generation,
                    window_generation: engine.surface_generation(window_id).unwrap_or(generation),
                    world3d_states: &mut *hosts.world3d_states,
                    world_resources: &mut *hosts.world_resources,
                };
                let paint = engine.frame_into_step(window_id, ui_wgpu::wgpu::geometry::Rect { x: bounds.x, y: bounds.y, w: viewport_w, h: viewport_h }, ctx.atlas, ctx.icons, Some(&mut scene_host), ctx.draw);
                match paint {
                    ui_wgpu::wgpu::UiFrameStep::Ready | ui_wgpu::wgpu::UiFrameStep::Missing => {
                        cursor.stalled = 0;
                        cursor.phase = UiDocumentFramePhase::Complete;
                    }
                    ui_wgpu::wgpu::UiFrameStep::Pending if engine.layout_is_dirty(window_id) => {
                        cursor.stalled = 0;
                        cursor.phase = UiDocumentFramePhase::Layout;
                    }
                    ui_wgpu::wgpu::UiFrameStep::Pending => {
                        if engine.paint_frame_progress(window_id) != progress_before {
                            cursor.stalled = 0;
                        } else {
                            cursor.stalled = cursor.stalled.saturating_add(1);
                        }
                        if cursor.stalled > 0 && cursor.stalled % UI_DOCUMENT_PAINT_STALL_NOTICE == 0 {
                            document_debug_log(&format!("[TRACE] ui-doc paint stalled window={window_id} opportunities={} {}", cursor.stalled, engine.paint_stall_census(window_id)));
                        }
                    }
                    ui_wgpu::wgpu::UiFrameStep::Fault => {
                        document_debug_log(&format!(
                            "[TRACE] ui-doc paint fault window={window_id} phase={:?} sync-line={} nodes={:?} {}",
                            engine.paint_frame_phase(window_id),
                            engine.paint_frame_sync_fault_line(window_id),
                            engine.tree(window_id).map(|tree| tree.root.is_some()),
                            engine.paint_stall_census(window_id)
                        ));
                        cursor.phase = UiDocumentFramePhase::Fault;
                    }
                }
            }
            UiDocumentFramePhase::Complete | UiDocumentFramePhase::Fault => {}
        }
        cursor.terminal_is_complete()
    })
}
//#endregion 📄️RetainedDocumentConsumer
