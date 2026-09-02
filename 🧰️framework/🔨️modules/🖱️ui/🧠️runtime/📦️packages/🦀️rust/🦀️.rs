//! @emoji 🧠️ The headless UI runtime — presentation state and the reconciler that turns it into
//! [`ui_contract`] patches. It renders nothing and knows about no renderer.
//!
//! ```text
//! projections → Entity<T> state → Present → ComponentTree → keyed reconcile → UiPatch
//! ```
//!
//! Two properties make this crate what it is:
//!
//! 1. **Production frame authority is mounted by the product host.** This headless crate retains the
//!    entity, presentation, and reconciliation primitives; its former standalone transaction is a
//!    test oracle only so production cannot select an unmounted alternate frame authority.
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
#[cfg(test)]
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
#[cfg(test)]
pub use transaction::*;
