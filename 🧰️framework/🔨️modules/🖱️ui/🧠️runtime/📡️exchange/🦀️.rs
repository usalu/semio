//! 📡️ The headless runtime exchange — the bounded command gateway, the coalescing projection inbox and the presence hub through which a surface talks to the actor world and to its peers. Pure bounded queues and tables: no entity, no presenter, no reconciler.

#[path = "../📮️gateway/🦀️.rs"]
mod gateway;
#[path = "../📥️inbox/🦀️.rs"]
mod inbox;
#[path = "../👥️presence/🦀️.rs"]
mod presence;

pub use gateway::*;
pub use inbox::*;
pub use presence::*;
