//! 🔬️ Rust reducer, id minting and runner against the transaction-law fixture, with the example drag
//! gesture statecharts the fixture's chart variants describe; the third-party `blake3` crate pins the ids.
#![allow(unexpected_cfgs)]

use super::*;
use machine::Command;
use machine::Machine;
use machine::Snapshot;
use machine::TimerId;
use protocol::ActorId;
use protocol::HybridLogicalTimestamp;
use protocol::TransactionRef;
use machine::{InvokeId, StatechartEvent, TestHost, Trigger};
use serde_json::{json, Value};

include!("📄️fixture-readers/🦀️.rs");

include!("🧮️reducer-and-id-laws/🦀️.rs");

include!("🌊️gesture-tool-laws/🦀️.rs");

include!("🏃️runner-laws/🦀️.rs");

include!("🫧️scrub-laws/🦀️.rs");

include!("⌨️typing-laws/🦀️.rs");

include!("🕸️node-drag-and-once-laws/🦀️.rs");
