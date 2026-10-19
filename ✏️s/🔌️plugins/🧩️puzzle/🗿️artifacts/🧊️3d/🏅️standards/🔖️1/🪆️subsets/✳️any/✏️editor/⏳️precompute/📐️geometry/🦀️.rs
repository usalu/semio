//! collision}` adapter (the ONE interface boundary this module depends on), the plain
//! 📐️ Puzzle 3d play app — the precompute geometry layer: the `semio_framework_3d::{rigid,
//! `[f64; 3]`/`[f64; 4]` vector and quaternion math the placement solver builds on, the brush
//! placement pose solver itself, and the collision-body/AABB/overlap primitives the brush and
//! fill lanes gate placements with. Rehomed from the former `⚙️engine/📐️geometry` (ticket
//! 26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES): this is interactive brush/fill tool
//! behaviour, so it lives with the app, not the artifact. The `nalgebra`/`parry3d` third-party
//! surface this adapter used to wrap moved into the framework (ticket
//! 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS) — see
//! `semio_framework_3d::rigid` (vectors/points/quaternions/isometries) and
//! `semio_framework_3d::collision` (BVH triangle-mesh intersection + winding-number containment).

#![allow(clippy::result_large_err)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_os_kernel as vcs;
extern crate semio_framework_value_derive as value_derive;

use semio_s_artifact_puzzle_3d_model::Quat;

use semio_s_artifact_puzzle_3d_model::Vec3;

use semio_s_artifact_puzzle_3d_model::WorldVolumeProps;
use semio_framework_3d::collision;
use semio_framework_3d::rigid;
use semio_framework_trace::GUEST_CONTIGUOUS_REQUEST_CEILING_BYTES;
use std::borrow::Borrow;
use std::mem::MaybeUninit;

#[path = "🧱️fixed-owners/🦀️.rs"]
mod fixed_owners;
pub use fixed_owners::*;

#[path = "🧮️pose-math/🦀️.rs"]
mod pose_math;
pub use pose_math::*;

#[path = "🧊️collision-index/🦀️.rs"]
mod collision_index;
pub use collision_index::*;

#[path = "💥️overlap-state-machine/🦀️.rs"]
mod overlap_state_machine;
pub use overlap_state_machine::*;

//#region 🧯️Reservation

//#endregion 🧯️Reservation

//#region 🔒️GeometryAdapter

//#endregion 🔒️GeometryAdapter

//#region 🔖️Constants
//#endregion 🔖️Constants

//#region 🔖️Vectors

//#endregion 🔖️Vectors

//#region 🧮️WorkMeter
#[cfg(any(test, feature = "testkit"))]
thread_local! {
    /// 🧮️ Primitive precompute work done on THIS thread: one unit per surface-distance or containment query, per triangle
    /// clipped against a near face, per spatial-index lookup or member walked, per mesh vertex and triangle ingested, and
    /// per unit the brush and fill machines advance. The interactive laws read it as their clock, so a step's cost is a
    /// deterministic count instead of a wall reading a loaded machine inflates.
    static PRECOMPUTE_WORK: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// 🧮️ Charges `units` of primitive work to this thread's [`precompute_work_done`] meter; a no-op outside test builds.
#[inline(always)]
pub fn precompute_work(units: usize) {
    #[cfg(any(test, feature = "testkit"))]
    PRECOMPUTE_WORK.with(|meter| meter.set(meter.get().saturating_add(units as u64)));
    #[cfg(not(any(test, feature = "testkit")))]
    let _ = units;
}

/// 🕰️ This thread's primitive precompute work so far — the deterministic clock the interactive laws drive and bound steps by.
#[cfg(any(test, feature = "testkit"))]
pub fn precompute_work_done() -> u64 {
    PRECOMPUTE_WORK.with(std::cell::Cell::get)
}

/// ⏲️ [`precompute_work_done`] as a job clock: a step's deadline then expires after a fixed amount of work, not of wall time.
#[cfg(any(test, feature = "testkit"))]
pub fn precompute_work_clock() -> Option<u64> {
    Some(precompute_work_done())
}
//#endregion 🧮️WorkMeter

//#region 🔖️Collision

//#region 🗺️BroadPhase

//#endregion 🗺️BroadPhase

//#region ⏳️OverlapStateMachine

//#endregion ⏳️OverlapStateMachine
//#endregion 🔖️Collision

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[path = "♻️retirement/🦀️.rs"]
mod original_fixed_retirement;
