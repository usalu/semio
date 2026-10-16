//! 📡️ JSON-RPC and MCP wire types, registries, the backend port and the notification lane.

#![allow(async_fn_in_trait)]

#[path = "../🖥️ui/📋️job-registry/🦀️.rs"]
pub mod job_registry;

#[path = "../📣️notify/🦀️.rs"]
pub mod notify;

#[path = "../🧭️protocol/🦀️.rs"]
pub mod protocol;

#[path = "../🧬️schema/📐️tool-shapes/🦀️.rs"]
pub mod tool_shapes;
