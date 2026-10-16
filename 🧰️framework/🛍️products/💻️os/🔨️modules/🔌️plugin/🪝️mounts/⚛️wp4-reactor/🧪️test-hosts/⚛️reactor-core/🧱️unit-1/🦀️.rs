//! 🧪️ Unit 1 of the facade test host of `reactor-core`: out of line because its `#[path]` mounts must not pass through a module directory that does not exist.

use semio_framework_ui_runtime::DEFAULT_REVISION_TOLERANCE;
use semio_framework::kernel::Effect;
use semio_framework::kernel::Event;
use semio_framework_plugin_reactor_tasks::INSTANCE_METADATA;
use semio_framework_plugin_reactor_tasks::tasks_instance_metadata::InstanceMetadataRegistry;
use semio_framework::kernel::MessageEndpoint;
use semio_framework_plugin_reactor_patches::PATCHES;
use semio_framework_plugin_reactor_lifecycle::REACTOR_CLOSES;
use semio_framework_plugin_reactor_lifecycle::REACTOR_CLOSE_CURSOR;
use semio_framework_plugin_reactor_tasks::tasks_fixed_slots::REACTOR_TASK_SLOTS;
use semio_framework_plugin_reactor_tasks::REGISTRY;
use semio_framework_plugin_reactor_lifecycle::life_reactor_close::ReactorCloseRegistry;
use std::cell::RefCell;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework::kernel::RequestOutcome;
#[cfg(test)]
use semio_framework_ui_runtime::SurfaceReconcileReadyPatch;
use semio_framework_plugin_reactor_tasks::TASK_EXECUTOR;
use semio_framework_plugin_reactor_tasks::TASK_RECORDS;
use semio_framework_plugin_reactor_tasks::TASK_RESUMES;
use semio_framework_plugin_reactor_tasks::tasks_task_lane::TaskResumeOutcome;
use semio_framework_plugin_reactor_turn::TurnMoreWorkSources;
use semio_framework::kernel::UiPatch;
use semio_framework_plugin_reactor_lifecycle::life_reactor_close::activate_reactor_close;
use semio_framework_plugin_reactor_tasks::tasks_extension_continuation::append_extension_response_page;
use semio_framework_plugin_reactor_surfaces::checkpoint;
use semio_framework_plugin_actor_host::executor;
use semio_framework_plugin_reactor_tasks::tasks_extension_continuation::host;
use semio_framework_plugin_reactor_lifecycle::instance_lifetime;
use semio_framework_ui_runtime::is_stale_intent;
use semio_framework_plugin_reactor_patches as patches;
use semio_framework_plugin_reactor_lifecycle as pending;
use semio_framework_plugin_reactor_turn as turn;
use semio_framework_ui_contract as ui_contract;
#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
use semio_framework_plugin_reactor_wit_bridge::wit_bridge::poll;
use semio_framework_plugin_reactor_tasks::tasks_extension_continuation::queue_extension_invocation;
use semio_framework_plugin_reactor_lifecycle::life_reactor_close::reactor_close_complete;
use semio_framework_plugin_reactor_lifecycle::life_reactor_close::reactor_close_fault;
use semio_framework_plugin_reactor_lifecycle::life_reactor_close::release_reactor_close;
use semio_framework_plugin_actor_host::requests;
use semio_framework_plugin_reactor_lifecycle::life_reactor_close::reserve_reactor_close;
use semio_framework_plugin_reactor_turn::turn_close_step::step_reactor_close;
use semio_framework_plugin_reactor_tasks::tasks_extension_continuation::take_extension_response;

#[cfg(test)]
#[path = "../../../../../⚛️reactor/🧪️tests/🔬️shell-fault-frame/🦀️.rs"]
mod shell_fault_frame_tests;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🧪️tests/📦️package-assets/🦀️.rs"]
mod package_asset_tests;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🧪️tests/🔬️command-ingress-terminal/🦀️.rs"]
mod command_ingress_terminal_tests;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🧪️tests/⚛️reactor-driver/🦀️.rs"]
pub(crate) mod reactor_driver;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🧪️tests/🔬️m1-m2-reactor/🦀️.rs"]
mod m1_m2_reactor_tests;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🧪️tests/🔬️reconcile-budget/🦀️.rs"]
mod reconcile_budget_tests;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🧪️tests/🔬️reconcile-spin/🦀️.rs"]
mod reconcile_spin_tests;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🧪️tests/🧺️turn-patch-batch/🦀️.rs"]
mod turn_patch_batch_tests;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🧪️tests/🚚️more-work-drive/🦀️.rs"]
mod more_work_drive_tests;
#[cfg(test)]
#[path = "../../../../../⚛️reactor/🧪️tests/🔬️extension-continuation/🦀️.rs"]
mod extension_continuation_tests;
#[cfg(test)]
#[path="../../../../../⚛️reactor/🎟️context/🌉️call/🧪️tests/🔌️registry/🦀️.rs"]
mod original_registered_job_loan_tests;
