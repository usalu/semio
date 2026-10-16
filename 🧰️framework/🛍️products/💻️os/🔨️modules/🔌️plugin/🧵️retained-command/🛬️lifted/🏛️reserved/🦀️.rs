//! 🏛️ Lift file: 1 item, about 20 lines, of `👷️job/🏛️reserved/🦀️.rs` that belong to crate `retained-command` (WP3).
//!
//! WP3 owns both crates: the framework reserved route capacity names `ArtifactRetainedWorkCapacity`, so it moves up from `job` to `retained-command`. Until `retained-command` is carved, the source module re-exports the item with `pub use`.
use super::*;

/// 🎛️ The one place a framework-reserved route id becomes its retained job. Deliberately a plain
/// non-`async` fn OUTSIDE `dispatch_framework_reserved_action`'s generator: every arm erases into
/// the same [`ArtifactReservedToolJob`], so one `match` here costs one small frame, whereas the
/// same arms spelled inline each owned a separate, non-overlapping `run_framework_reserved_job`
/// future slot in the single resume frame — 1.80 MiB of stack for ONE dispatch in an unoptimized
/// build, more than a default 2 MiB thread or an 8 MiB wasm shadow stack can absorb once the rest
/// of the route is layered on top (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️selection-overflow-2026-09-09.md`). `copy`/`cut`/`paste` reach it only when the app declares
/// no reserved job of its own.
/// 🧮️ The ONE work capacity every framework-reserved route declares. Its durable lane is a
/// bounded config store whose preflight ([`admit_bounded_config_mutation`]) declares one
/// POINT-INVERTIBLE item — a forward row plus an inverse row — so a reserved gesture of `n`
/// items costs `n` × that, in the very unit `ArtifactStore::fold_batch_item` measures. The
/// ceiling is the store's own one-item maximum expressed in items, so a gesture this refuses is
/// one the store could never admit either. `interactionSelect` and its five interaction siblings
/// used to declare ONE work item against that TWO-row footprint, which is the same
/// declaration-vs-arithmetic split `📓️fold-contract-2026-09-10.md` fixed one layer down
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
pub(crate) const FRAMEWORK_RESERVED_ROUTE_CAPACITY: crate::retained_command::ArtifactRetainedWorkCapacity =
    crate::retained_command::ArtifactRetainedWorkCapacity::for_invertible_items(store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_WORK_ITEMS / store::os_store::one_item::ARTIFACT_STORE_ONE_ITEM_INVERTIBLE_WORK_ITEMS);
