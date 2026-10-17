//! 🔐️ First-party, inherited-handle local bootstrap transport and bounded issuance service.

use semio_hub_directory_contract::error::{DirectoryError, DirectoryResult};
use semio_hub_directory_contract::model::{AuthSessionIssue, AuthSessionKind, IssuedAuthSession};
use semio_hub_directory_contract::{constant_time_digest_eq, identity_subject_digest, validate_bounded_auth_text, IdentityVerificationContext, IdentityVerificationControl, LocalBootstrapAcceptFuture, LocalBootstrapClientClass, LocalBootstrapIssueFuture, LocalBootstrapRejectCode, LocalBootstrapTerminalFuture, LocalBootstrapTransport, VerifiedLocalBootstrapRequest, AUTH_TEXT_MAX_BYTES, DEVICE_INSTANCE_MAX_BYTES};
use semio_hub_directory_dispatch::HubDirectories;
use semio_hub_directory::{AuthSessionDirectory, UserDirectory};
use semio_framework_hash::{hex_lower, Sha256};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub const LOCAL_BOOTSTRAP_SCHEMA: &str = "semio.hub.local-bootstrap/v1";
pub const LOCAL_CREDENTIAL_SCHEMA: &str = "semio.hub.local-credential-envelope/v1";
pub const LOCAL_BOOTSTRAP_IDENTITY_PROVIDER: &str = "semio.local.bootstrap/v1";
pub const LOCAL_BOOTSTRAP_FRAME_BYTES_MAX: usize = 16 * 1024;
pub const LOCAL_BOOTSTRAP_OUTSTANDING_MAX: usize = 8;
pub const LOCAL_BOOTSTRAP_PROFILES_MAX: usize = 8;
/// ⏳️ Exchange ids one validity window (`LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS`) admits. An id leaves
/// the window once its own `expiresAt` has passed, so a long-lived launcher may exchange for ever at
/// up to this many exchanges per 15 s; a burst above it is answered `resource-limit`, never an exit.
pub const LOCAL_BOOTSTRAP_REPLAY_WINDOW_MAX: usize = 64;
pub const LOCAL_BOOTSTRAP_EXCHANGE_DEADLINE_MS: i64 = 15_000;
pub const LOCAL_BOOTSTRAP_SESSION_TTL_SECS: i64 = 15 * 60;
const LOCAL_BOOTSTRAP_HMAC_DOMAIN: &[u8] = b"semio/hub/local-bootstrap/v1\0";
const LOCAL_BOOTSTRAP_IDLE_POLL_MS: u64 = 100;
const INHERITED_BOOTSTRAP_DESCRIPTOR: i32 = 3;

#[cfg(unix)]
type LocalBootstrapIo = tokio::net::UnixStream;
#[cfg(windows)]
type LocalBootstrapIo = tokio::fs::File;

#[path = "🧾️wire-frames/🦀️.rs"]
mod wire_frames;
use wire_frames::*;

#[path = "🚰️transport-state/🦀️.rs"]
mod transport_state;
pub use transport_state::*;

#[path = "🔁️exchange-window/🦀️.rs"]
mod exchange_window;
use exchange_window::*;

#[path = "🛰️transport-session/🦀️.rs"]
mod transport_session;

#[path = "🔌️transport-port/🦀️.rs"]
mod transport_port;
use transport_port::*;

#[path = "🎫️issuance-service/🦀️.rs"]
mod issuance_service;
pub use issuance_service::*;

#[path = "📨️frame-io/🦀️.rs"]
mod frame_io;
use frame_io::*;

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
