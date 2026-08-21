//! @emoji 🧠️ The headless UI runtime — presentation state and the reconciler that turns it into
//! [`ui_contract`] patches. It renders nothing and knows about no renderer.
//!
//! ```text
//! projections → Entity<T> state → Present → ComponentTree → keyed reconcile → UiPatch
//! ```
//!
//! Two properties make this crate what it is:
//!
//! 1. **A transaction is persistent and atomically published.** [`transaction`]'s
//!    `FrameTransaction::step()` advances seven scheduler-bounded stages. Reconciliation happens on
//!    shadow state, and only a completed snapshot changes the retained revisions.
//! 2. **It runs on both sides of the plugin boundary.** A guest plugin embeds it to produce patches;
//!    a host embeds it to drive its own screens. Hence no `Send` bounds and no dependency beyond the
//!    contract and shared job protocol — it compiles for `wasm32-wasip2` and
//!    `wasm32-unknown-unknown` by construction.

#[path = "🦀️context.rs"]
mod context;
#[path = "🦀️dispatch.rs"]
mod dispatch;
#[path = "🦀️entity.rs"]
mod entity;
#[path = "🦀️gateway.rs"]
mod gateway;
#[path = "🦀️inbox.rs"]
mod inbox;
#[path = "🦀️presence.rs"]
mod presence;
#[path = "🦀️present.rs"]
mod present;
#[path = "🦀️reconcile.rs"]
mod reconcile;
#[path = "🦀️tracking.rs"]
mod tracking;
#[path = "🦀️transaction.rs"]
mod transaction;

pub use context::*;
pub use dispatch::*;
pub use entity::*;
pub use gateway::*;
pub use inbox::*;
pub use presence::*;
pub use present::*;
pub use reconcile::*;
pub use tracking::*;
pub use transaction::*;
