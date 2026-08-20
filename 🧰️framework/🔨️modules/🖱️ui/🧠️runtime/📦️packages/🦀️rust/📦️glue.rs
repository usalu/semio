//! @emoji 🧠️ The headless UI runtime — presentation state and the reconciler that turns it into
//! [`ui_contract`] patches. It renders nothing and knows about no renderer.
//!
//! ```text
//! projections → Entity<T> state → Present → ComponentTree → keyed reconcile → UiPatch
//! ```
//!
//! Two properties make this crate what it is:
//!
//! 1. **A transaction is run-to-completion.** [`transaction`]'s `transact()` drains the inbox,
//!    dispatches intents, flushes effects to a fixpoint, presents dirty surfaces and reconciles —
//!    without a single suspension point (ruling U1, ticket
//!    `26/08/20/SEMANTIC-UI-CONTRACT-AND-RENDERER-FAMILY`). Async lives at the edges: the embedder's
//!    event loop, the actor mailbox, asset loading. `spawn_local` hands a future to the embedder's
//!    executor and the result re-enters through the inbox at the next transaction boundary, so a
//!    mutable entity reference can never be held across an await.
//! 2. **It runs on both sides of the plugin boundary.** A guest plugin embeds it to produce patches;
//!    a host embeds it to drive its own screens. Hence no `Send` bounds and no dependency beyond the
//!    contract — it compiles for `wasm32-wasip2` and `wasm32-unknown-unknown` by construction.

#[path = "🦀️entity.rs"]
mod entity;
#[path = "🦀️context.rs"]
mod context;
#[path = "🦀️tracking.rs"]
mod tracking;
#[path = "🦀️present.rs"]
mod present;
#[path = "🦀️reconcile.rs"]
mod reconcile;
#[path = "🦀️gateway.rs"]
mod gateway;
#[path = "🦀️inbox.rs"]
mod inbox;
#[path = "🦀️presence.rs"]
mod presence;
#[path = "🦀️dispatch.rs"]
mod dispatch;
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
