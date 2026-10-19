//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Affero General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

//! 🔌️ The repo Model Context Protocol server: the bounded JSON-RPC contract, the hash-chained event
//! log, the line-delimited stdio transport, the session state machine, request routing and the
//! repository tool/resource/prompt surface. Domain behaviour lives behind [`Repository`]; this crate
//! owns the wire and nothing else. The contract it implements is
//! `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/🧬️schema/🔣️.json`.

//#endregion 🧲️Header

use std::collections::BTreeMap;
use std::io::{BufRead, BufReader, Read, Write};
use std::sync::atomic::{AtomicBool, AtomicI64, AtomicU64, Ordering};
use std::sync::{Arc, Condvar, Mutex, OnceLock};

//#region 🔖️Children
        }),
    )?;
    Ok(server)
}

#[path = "🔏️digest-and-json/🦀️.rs"]
mod digest_and_json;
pub use digest_and_json::*;

#[path = "📨️protocol/🦀️.rs"]
mod protocol;
pub use protocol::*;

#[path = "📡️event-log/🦀️.rs"]
mod event_log;
pub use event_log::*;

#[path = "🔐️session/🦀️.rs"]
mod session;
pub use session::*;

#[path = "🚦️routing/🦀️.rs"]
mod routing;
pub use routing::*;

#[path = "🗄️repository/🦀️.rs"]
mod repository;
pub use repository::*;

#[path = "🚀️entrypoint/🦀️.rs"]
mod entrypoint;
pub use entrypoint::*;

//#endregion 🔖️Children

//#region 🧪️Tests

#[cfg(test)]
#[path = "../../🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

//#endregion 🧪️Tests
