//! 📖️ Actor vocabulary and wire frames own nominal identities and original retained turn boundaries.
use std::collections::VecDeque;
use semio_framework_value_derive::{FromValue,ToValue};
use semio_framework_actor_retained_turn::RetainedTurnReceipt;
#[path="../🚪️lifetime/🦀️.rs"]
pub mod instance_lifetime;
#[path="../📃️page/🦀️.rs"]
pub mod byte_page;
#[path="../📤️return/🦀️.rs"]
pub mod return_page;
#[path="../📥️cold-pair/🦀️.rs"]
pub mod cold_pair;

#[path = "🆔️identities-and-budgets/🦀️.rs"]
mod identities_and_budgets;
pub use identities_and_budgets::{ActorId, ActorKind, Budget, JobReplayRequest, Lane, PackageHash, PackageId, lane_defaults, replay_mix};

#[path = "🧵️job-operation-records/🦀️.rs"]
mod job_operation_records;
pub use job_operation_records::{JobCheckpoint, JobCommitCandidate, JobOperation, JobPublication, JobStepOutcome, JobTurn};

#[path = "✉️envelope-and-payload/🦀️.rs"]
mod envelope_and_payload;
pub use envelope_and_payload::{CoalesceKey, Envelope, Origin, Payload, WindowId};

#[path = "📬️turn-result-and-mailbox/🦀️.rs"]
mod turn_result_and_mailbox;
pub use turn_result_and_mailbox::{Backpressure, Mailbox, TurnResult, TurnStatus, Usage};

#[path = "🚑️capability-and-failure-policy/🦀️.rs"]
mod capability_and_failure_policy;
pub use capability_and_failure_policy::{CapabilityGrant, FAILURE_DECAY_CLEAN_TURNS, FAILURE_HEARTBEAT_TRAP_THRESHOLD, FAILURE_QUARANTINE_RESTART_THRESHOLD, FailureEscalation, FailureSignal, FailureStage, FailureState, intersect_capabilities, quarantine_duration_ms};

#[path = "🧩️actor-status-and-shards/🦀️.rs"]
mod actor_status_and_shards;
pub use actor_status_and_shards::{ActorStatus, ShardId, ShardKind, clamp_native_shard_count, clamp_web_shard_count};

//#region 🧩️ShardTable

#[cfg(test)]
#[path="../📃️page/🧪️tests/📃️page/🦀️.rs"]
mod byte_page_tests;
#[cfg(test)]
#[path="../📤️return/🧪️tests/📤️return/🦀️.rs"]
mod return_page_tests;
#[cfg(test)]
#[path="../📥️cold-pair/🧪️tests/📥️cold-pair/🦀️.rs"]
mod cold_pair_tests;
//#endregion 🧩️ShardTable
