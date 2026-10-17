//! 🌉️ Language-neutral component ABI: handles, pages, messages, ledgers and ports.

use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;

pub use semio_framework_value::retained_clone::RetainedCloneGrant;

#[path = "🪪️limits-and-identifiers/🦀️.rs"]
mod limits_and_identifiers;
pub use limits_and_identifiers::*;

#[path = "📨️status-and-messages/🦀️.rs"]
mod status_and_messages;
pub use status_and_messages::*;

#[path = "🧱️message-codec/🦀️.rs"]
mod message_codec;
pub use message_codec::*;

#[path = "⏳️page-transfer/🦀️.rs"]
mod page_transfer;
pub use page_transfer::*;

#[path = "🗃️ledgers-and-port/🦀️.rs"]
mod ledgers_and_port;
pub use ledgers_and_port::*;

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
