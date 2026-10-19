//! 🏊️ Kernel pool: the kernel pool state implementation, request queue, pool futures and the pool runner.
use super::*;

#[path="🎟️task-ingress/🦀️.rs"]
mod original_task_ingress;
pub(crate) use original_task_ingress::{OriginalKernelTaskInput,OriginalKernelTaskProducer};

#[path = "🏗️pool-construction-and-product-replay/🦀️.rs"]
mod pool_construction_and_product_replay;

#[path = "📼️job-replay-and-command-maintenance/🦀️.rs"]
mod job_replay_and_command_maintenance;

#[path = "🧩️app-lifecycle-and-realm-close/🦀️.rs"]
mod app_lifecycle_and_realm_close;

#[path = "🔄️exchange-and-turn-settling/🦀️.rs"]
mod exchange_and_turn_settling;

#[path = "🚀️turn-dispatch-and-pool-runner/🦀️.rs"]
mod turn_dispatch_and_pool_runner;
pub use turn_dispatch_and_pool_runner::run_kernel_pool;
