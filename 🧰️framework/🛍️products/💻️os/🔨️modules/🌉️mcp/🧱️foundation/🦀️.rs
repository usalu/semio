//! 🧱️ Gateway vocabulary: errors, handles, audit lane, reports, local session files and delegated credentials.

#![allow(async_fn_in_trait)]

#[path = "../🤖️agent-credential/🦀️.rs"]
pub mod agent_credential;

#[cfg(test)]
#[path = "../🧪️tests/🤖️agent-credential/🦀️.rs"]
pub mod agent_credential_tests;

#[path = "../📒️audit/🦀️.rs"]
pub mod audit;

#[cfg(test)]
#[path = "../📒️audit/🧪️tests/🔬️quick/🦀️.rs"]
pub mod audit_quick;

#[path = "../🐚️channel/🔡️base64-codec/🦀️.rs"]
pub mod base64_codec;

#[path = "../🏠️workspace/🕰️clock/🦀️.rs"]
pub mod clock;

#[path = "../⚠️errors/🦀️.rs"]
pub mod errors;

#[cfg(test)]
#[path = "../⚠️errors/🧪️tests/🔬️quick/🦀️.rs"]
pub mod errors_quick;

#[path = "../🎫️handles/🦀️.rs"]
pub mod handles;

#[cfg(test)]
#[path = "../🎫️handles/🧪️tests/🔬️quick/🦀️.rs"]
pub mod handles_quick;

#[path = "../💡️inference/🧾️opaque-json/🦀️.rs"]
pub mod opaque_json;

#[path = "../🔐️owner-only/🦀️.rs"]
pub mod owner_only;

#[cfg(test)]
#[path = "../🔐️owner-only/🧪️tests/🔬️unit/🦀️.rs"]
pub mod owner_only_unit;

#[path = "../🏠️workspace/🔗️remote/🔤️percent-codec/🦀️.rs"]
pub mod percent_codec;

#[path = "../🛰️rendezvous/🦀️.rs"]
pub mod rendezvous;

#[cfg(test)]
#[path = "../🛰️rendezvous/🧪️tests/🔬️quick/🦀️.rs"]
pub mod rendezvous_quick;

#[path = "../🧬️schema/📋️reports/🦀️.rs"]
pub mod reports;
