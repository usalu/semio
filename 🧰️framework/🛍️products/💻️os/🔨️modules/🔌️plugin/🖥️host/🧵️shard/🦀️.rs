//! 🧵️ `ShardLoop` — `design-runtime.md` §2/§"ShardTransport": the logic one shard runs, IN-PROCESS.
//! Owns a set of live [`super::GuestInstance`]s, pulls [`ShardFrame`]s off a
//! [`semio_framework_actor::ShardTransport`], groups their envelopes per actor, drives
//! [`super::GuestRuntime::execute_turn`]/[`super::GuestRuntime::step_job`], and sends the resulting
//! [`semio_framework::kernel::TurnResult`]/[`super::JobStep`] back over the SAME transport as bytes.
//!
//! MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (P1c, one-pool-worker-runtime): a `ShardLoop` no longer
//! implies a dedicated OS thread — [`executor::ShardExecutor`] drives one behind a plain [`Mutex`],
//! scheduled as jobs onto the shared, process-wide `semio_framework_async::WorkerPool` (see that
//! module's own doc for the single-flight scheduling protocol). `pump`/`pump_primed`'s own contract
//! is UNCHANGED by that move: "drain and execute everything currently buffered in one call" is
//! exactly what one `WorkerPool` job burst wants, whether it used to be one iteration of a dedicated
//! thread's park/pump loop or is now one pool-scheduled job. Written so the identical type can also
//! be driven over stdio by a helper PROCESS (`ProcessTransport`) or a browser Worker — the only thing
//! that changes across "in-process pool-scheduled", "child process", "web worker" is which
//! [`ShardTransports`] variant `ShardLoop::new` receives; `pump`'s own body never branches on which
//! one it got (only the closed-set enum's own delegation impl does — O1/R1's dyn replacement, packet
//! host-dedyn).
//!
//! terra-shard-grants: the wire carries [`ShardFrame`], not raw [`Envelope`] bytes — the kernel's
//! DRR-computed, throttle-scaled per-turn budget now travels WITH the envelopes it grants
//! ([`ShardFrame::Grant`]) instead of `pump` re-deriving one from local constants.

extern crate semio_framework_os_kernel as store;

#[path = "🔁️lifecycle/🦀️.rs"]
mod lifecycle;
use lifecycle::AdmittedAuthority;
#[path="🎟️grant/📨️event/🦀️.rs"]
mod original_event;
use original_event::OriginalShardEvent;
use semio_framework_actor::{OriginalActorInput,OriginalActorGuestReturn,ActorGuestPhase,RetainedTurnInput};
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress};
#[path="🪪️identity/🦀️.rs"]
pub mod identity;

#[path="🎟️grant/🦀️.rs"]
pub mod grant;
pub use identity::{OriginalShardIdentity,OriginalShardIdentityIssuer,ShardIdentityPolicy,native_identity_issuer};
pub use lifecycle::{ShardActorAllocation, ShardRegistrationReason, ShardRegistrationRejected};

use semio_framework_plugin_host_guest_runtime::{GuestInstance, GuestRuntime, GuestRuntimes};
use semio_framework_plugin_guest_contract::{JobStep, PluginHostError, TurnFault};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_actor::{PackageHash, PackageId};
use semio_framework::kernel::{Budget, Effect, Event, JobPlacement, RequestOutcome, TurnResult};
use semio_framework_actor::{ActorId, Envelope, JobCheckpoint, JobCommitCandidate, JobOperation, JobPublication, JobReplayRequest, JobStepOutcome, JobTurn, Payload, ShardTransport};
use semio_framework_trace::{Generation, InteractiveStage, OperationId, Watchdog};
use std::collections::{BTreeSet, HashMap};
use std::mem::{MaybeUninit, size_of};
use std::sync::Arc;
#[cfg(any(test, feature = "testkit"))]
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};

#[path = "📨️shard-frame-and-budget-bridge/🦀️.rs"]
mod shard_frame_and_budget_bridge;
pub use shard_frame_and_budget_bridge::{IssuedShardRetainedRecipient, IssuedShardTurn, ShardFrame, to_actor_turn_result, turn_budget_from_grant, turn_events};
use shard_frame_and_budget_bridge::{SHARD_TURN_ADMITTED_JOBS_MAXIMUM, to_actor_turn_result_in_place};

#[path = "📊️shard-outcome-and-loop-state/🦀️.rs"]
mod shard_outcome_and_loop_state;
pub use shard_outcome_and_loop_state::{JobAuthority, ShardLoop, ShardOutcome};

#[path = "📄️fixed-replay-seed/🦀️.rs"]
mod fixed_replay_seed;
pub use fixed_replay_seed::{FixedReplaySeed, JOB_REPLAY_ABI_BYTES, JOB_REPLAY_KIND_PAGE_CAPACITY, JOB_REPLAY_REFUSAL_SLOT_CAPACITY, JOB_REPLAY_SEED_PAGES, JOB_REPLAY_SEED_PAGE_BYTES, JOB_REPLAY_SEED_SLOT_CAPACITY, MountedReplaySeed, ReplaySeedCloseReason, ReplaySeedPhase, ReplaySpawnRefusal, try_replay_abi_buffer};
#[cfg(any(test, feature = "testkit"))]
pub use fixed_replay_seed::replay_test_authority;
use fixed_replay_seed::ReplaySpawnRefusalPhase;

#[path = "🚦️deferred-owner-ring/🦀️.rs"]
mod deferred_owner_ring;
pub use deferred_owner_ring::{AdmissionLimit, CancelCursor, DeferredAuthority, FixedOwnerRing, OwnerSlot, SHARD_DEFERRED_BYTES, SHARD_DEFERRED_ITEMS, SHARD_FRAME_MAX_BYTES, ShardDrive, TerminalFrameOverflow, split_frame_credit};
use deferred_owner_ring::{FrameAdmission, FrameAdmissionError, defer_completion};
#[cfg(any(test, feature = "testkit"))]
use deferred_owner_ring::OwnerKey;

#[path = "🏗️shard-loop-construction-and-replay-drive/🦀️.rs"]
mod shard_loop_construction_and_replay_drive;

#[path = "🌀️shard-loop-registry-and-pump/🦀️.rs"]
mod shard_loop_registry_and_pump;

#[path = "🎬️shard-loop-frame-turns-and-job-control/🦀️.rs"]
mod shard_loop_frame_turns_and_job_control;

#[path = "🔬️replay-lifecycle-and-fault-helpers/🦀️.rs"]
mod replay_lifecycle_and_fault_helpers;
pub use replay_lifecycle_and_fault_helpers::exercise_replay_lifecycle_trace;
use replay_lifecycle_and_fault_helpers::{interactive_stage_for, start_job_fault_bytes, turn_fault_message};

#[path = "🛰️shard-transports-and-test-doubles/🦀️.rs"]
mod shard_transports_and_test_doubles;
pub use shard_transports_and_test_doubles::{ShardTransports, SharedThreadTransport};
#[cfg(any(test, feature = "testkit"))]
pub use shard_transports_and_test_doubles::{LoopbackProbe, LoopbackTransport, fixture_instance_close_event, fixture_instance_close_request, test_identity_issuer};

pub use lifecycle::LifecycleRetry;
