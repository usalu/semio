// 🧵️ HUB WP3: mount hub of WP3 UI engine, frame core and host: ui-engine-*, host-plumbing, frame-actions, runtime-mailbox, app-runtime, frame-presentation, frame-transaction, frame-job, native-probe, host-winit, browser-worker.
// This file is spliced into the crate root by `include!` from the renderer root, so every item below stays a crate-root item;
// its `#[path]` and `include!` literals resolve against this folder (`📥️mounts/<hub>/`), hence the `../../` prefix of the renderer folder.
// Carving a crate removes its unit here and puts `extern crate <crate> as <old module name>;` in its place (PLAN-RENDERER-WGPU-CRATES.md, section 10.3).

#[cfg(test)]
use semio_framework_os_renderer_wgpu_ui_engine_tests::fixture_original_turn;

#[cfg(test)]
include!("../../../../../🧪️tests/🧊️wgpu-renderer-standalone/🦀️.rs");

#[cfg(test)]
include!("../../../../../🧪️tests/🐕️wgpu-present-stall-watch/🦀️.rs");

#[path = "../../../../../🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🦀️.rs"]
pub mod interpreter;


pub(crate) use semio_framework_os_renderer_wgpu_runtime_mailbox::*;



pub(crate) use semio_framework_os_renderer_wgpu_frame_presentation::*;

pub(crate) use semio_framework_os_renderer_wgpu_app_runtime::*;

pub(crate) use semio_framework_os_renderer_wgpu_native_probe::*;

//#region 🔖️AsyncBoundaryTests
#[cfg(test)]
#[path = "../../../../../🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs"]
mod async_boundary_tests;
//#endregion 🔖️AsyncBoundaryTests

//#region 🔖️FrameActionLedgerTests
#[cfg(test)]
#[path = "../../../../../🧪️tests/🧾️frame-action-ledger/🦀️.rs"]
mod frame_action_ledger_tests;
//#endregion 🔖️FrameActionLedgerTests

//#region 🔖️WheelApplicationPointTests
#[cfg(test)]
#[path = "../../../../../🧪️tests/🖱️wheel-application-point/🦀️.rs"]
mod wheel_application_point_tests;
//#endregion 🔖️WheelApplicationPointTests

//#region 🔖️OsHostDecomposition
// 🏠️ ticket 26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY (packet os-host): the seam that ends
// this crate owning the actor kernel and ends its continuous redraw. `deadlines` is a leaf (no
// dependency on `os_host`/`winit_app`); `os_host` composes `AppRuntime` with it; `winit_app` is the
// new `ApplicationHandler` — see that file's own module docstring for why it hand-rolls the event
// loop instead of using `ui_host::window::NativeHost<D>` directly. Mounted here, away from the peer
// program's `parallel_runtime` mount just below, per this ticket's own OWNS list.
//
// 🪢️ There used to be a second, `#[cfg(test)]`-only `kernel_seam` module here: a bounded
// intent/outcome mailbox whose own docstring admitted "Test fixture … Production rendering dispatches
// through RuntimeMailbox; this fixture has no production router." It had no production consumer at
// all (its `KernelSeam`/`AppKernelSeam`/`HostWaker` were named only by its own test) and it shadowed
// the name of the REAL production exchange, `KernelOutcome` in `🔖️KernelClient` below. Ticket
// 26/09/17/WGPU-RENDERER-REACT-PARITY packet W2k retired it: `📮️RuntimeMailbox` is the one production
// router, and it is already the exact counterpart of React's own dispatch seam — `onActionStable`
// taking a descriptor built by `uiIntentToActionDescriptor` (`🛠️ShellHelpers/🟦️.tsx:2051`). Nothing
// about wgpu's runtime semantics needed the fixture; what DID differ was the controller identity that
// descriptor carried, and that is fixed at the source in `ui_wgpu`'s `UiIntentCommand::descriptor`.
use semio_framework_os_renderer_wgpu_host_plumbing::{browser_original_output, deadlines, input_wire, render_snapshot, surface_lane};
#[cfg(not(target_arch = "wasm32"))]
use semio_framework_os_renderer_wgpu_host_plumbing::native_caller;
pub(crate) use semio_framework_os_renderer_wgpu_host_winit::*;
// 🧵️ P3b (INTERACTIVE-JOB-RUNTIME-REFACTOR, ui-thread-isolation): the `InteractiveJob` seam for the
// slice of `AppRuntime::frame()` that genuinely is `Send`-safe today — see that file's own module
// docstring for exactly what moves and, more importantly, what still cannot.
pub(crate) use semio_framework_os_renderer_wgpu_frame_job::*;
#[cfg(target_arch = "wasm32")]
pub(crate) use semio_framework_os_renderer_wgpu_browser_worker::*;
//#endregion 🔖️OsHostDecomposition

//#region 🧵️FrameCoreTestHosts
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../../../../../../../../../🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🔗️commit/📥️enqueue/🧪️tests/📥️enqueue/🦀️.rs"]
mod runtime_publication_tests_host;
//#endregion 🧵️FrameCoreTestHosts
//#region 🧵️FrameCoreTestHosts
#[cfg(test)]
#[path = "../../../../../🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs"]
mod tests_host;
#[cfg(test)]
#[path = "../../../🧵️frame-job/📬️outcome/🧪️tests/🦀️.rs"]
mod original_outcome_tests_host;
//#endregion 🧵️FrameCoreTestHosts
//#region 🧵️FrameCoreTestHosts
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "../../🪶️sqlite/🧪️tests/🦀️.rs"]
mod native_socket_sqlite_tests_host;
//#endregion 🧵️FrameCoreTestHosts
//#region 🧵️FrameCoreTestHosts
#[cfg(test)]
#[path = "../../../../../🧪️tests/🔬️wgpu-winit-app-p3c/🦀️.rs"]
mod p3c_tests_host;
#[cfg(test)]
#[path = "../../../../../🧪️tests/🔬️wgpu-winit-app-callback-latency/🦀️.rs"]
mod callback_latency_tests_host;
#[cfg(test)]
#[path = "../../../../../🧪️tests/🔢️frame-generation-hold/🦀️.rs"]
mod frame_generation_hold_tests_host;
//#endregion 🧵️FrameCoreTestHosts
//#region 🧵️FrameCoreOsHostTestHost
#[cfg(test)]
include!("../../../../../🧪️tests/🧊️wgpu-os-host-standalone/🦀️.rs");
//#endregion 🧵️FrameCoreOsHostTestHost
//#region 🧷️InterpreterTestHosts
// 🧪️ Test hosts of the interpreter laws that name the monolith (scenes, engine canvas, the frame fixtures): the `ui-engine` crates are built with their `testkit` feature in this test unit, and the law files keep the namespace they were written against.
#[cfg(all(test, not(target_arch = "wasm32")))]
#[path = "🧪️test-hosts/🧷️interpreter-command-wiring/🦀️.rs"]
mod interpreter_command_wiring_host;

#[cfg(test)]
#[path = "🧪️test-hosts/🔎️interpreter-introspection/🦀️.rs"]
mod interpreter_introspection_host;
//#endregion 🧷️InterpreterTestHosts
pub(crate) use semio_framework_os_renderer_wgpu_frame_actions::*;
pub(crate) use semio_framework_os_renderer_wgpu_frame_transaction::*;
