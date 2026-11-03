include!("../🧰️prelude/🦀️.rs");

#[path = "../🧰️context/🦀️.rs"]
pub(crate) mod context;

include!("🧮️extent-and-step-laws/🦀️.rs");
include!("🎛️dispatch-and-operation-laws/🦀️.rs");
include!("💡️suggestion-and-scene-laws/🦀️.rs");

#[path = "../🤖️agent-lane/🦀️.rs"]
mod agent_lane;

#[path = "../🧵️retained-wiring/🦀️.rs"]
mod retained_wiring;
