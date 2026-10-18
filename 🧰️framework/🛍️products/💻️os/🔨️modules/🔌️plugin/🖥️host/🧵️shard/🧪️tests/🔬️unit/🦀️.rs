fn fixture_issued_turn(budget: semio_framework_actor::Budget) -> IssuedShardTurn { IssuedShardTurn::new(budget, original_retained_turn(), 1).expect("declared original single envelope") }
pub(super) fn original_retained_turn() -> semio_framework_actor::RetainedTurnInput { let law: serde_json::Value = serde_json::from_str(include_str!("../../../../../../../../🔨️modules/🎭️actor/🎟️retained-turn/📃️policy/🧪️tests/🧫️fixtures/🔣️.json")).unwrap(); serde_json::from_value(law["input"].clone()).unwrap() }
fn idle_receipt() -> semio_framework_actor::RetainedTurnReceipt { original_retained_turn().return_original(semio_framework_value::RetainedCloneProgress::default()).expect("idle turn spends nothing") }

use std::sync::Mutex;
use semio_framework_actor::{ActorId, Envelope, JobCheckpoint, JobOperation, JobPublication, JobReplayRequest, JobStepOutcome, JobTurn, PackageHash, PackageId, Payload};
use semio_framework_plugin_host_shard::{AdmissionLimit, CancelCursor, DeferredAuthority, FixedOwnerRing, IssuedShardTurn, JOB_REPLAY_ABI_BYTES, JOB_REPLAY_KIND_PAGE_CAPACITY, JOB_REPLAY_SEED_PAGES, JOB_REPLAY_SEED_PAGE_BYTES, JobAuthority, MountedReplaySeed, ReplaySeedCloseReason, ReplaySeedPhase, ReplaySpawnRefusal, SHARD_DEFERRED_ITEMS, ShardDrive, ShardFrame, ShardLoop, ShardOutcome, ShardTransports, TerminalFrameOverflow, split_frame_credit, to_actor_turn_result, try_replay_abi_buffer, turn_events};
use std::sync::Arc;
use semio_framework::kernel::{Budget, Effect, Event, JobPlacement, RequestOutcome, TurnResult};
use semio_framework_plugin_host_guest_runtime::{GuestInstanceState, GuestRuntime, GuestRuntimes, PackageRef};
use std::collections::HashMap;
use semio_framework_plugin_guest_contract::{JobStep, PluginHostError};
use std::sync::atomic::Ordering;
use std::mem::size_of;
#[cfg(any(test, feature = "testkit"))]
use semio_framework_plugin_host_shard::{LoopbackTransport, fixture_instance_close_event, fixture_instance_close_request, replay_test_authority};
#[cfg(any(test, feature = "testkit"))]
use semio_framework_plugin_host_guest_runtime::{MockGuestRuntime, RecordingRuntime};

include!("🎫️issued-batches-pump-and-actor-lifecycle/🦀️.rs");
include!("🚀️spawn-and-cancel-job-effects/🦀️.rs");
include!("⏯️suspend-resume-cancel-and-placement/🦀️.rs");
include!("🔄️shard-frame-round-trips/🦀️.rs");
include!("💰️grant-budget-frames-and-budget-bridge/🦀️.rs");
include!("🏁️lane-priority-epoch-yield-and-rings/🦀️.rs");
include!("🔁️terminal-overflow-credit-and-lifecycle-authorities/🦀️.rs");
include!("🧱️replay-owners-close-funnel-and-mounted-cancel/🦀️.rs");
include!("🎯️wake-host-dispatch-refusal-and-services-recipient/🦀️.rs");
