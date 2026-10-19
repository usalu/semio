//! 🌐️ The proctor end to end: booted on an ephemeral port over a fresh data directory and the
//! fixture course, driven through its real HTTP API with the §9a encoding by an independent HTTP
//! client (`ureq`), exactly as the browser client talks to it — then stopped, reopened from the same
//! SQLite file and asked again. Its presence rooms are joined by an independent websocket client
//! (`tungstenite`) speaking `semio.presence.v1`.
//!
//! @see ../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/QUIZ-PRODUCT-AND-TEACHING-PROCTOR/📓️design.md — §8, §9, §9a, §15
//! @see ../../🧫️fixtures/📚️course/🔣️.json — the course played
//!
//! The suite is split into the child folders next to this file: `🧰️harness` (scratch, the booted server, the wire, presence and answers), `🎮️learner-journey`,
//! `🏆️challenge-sheets`, `🛠️operator` and `🛡️edge` (the tests); this file mounts them and is the crate root of the `end_to_end` test.

use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use teaching_proctor_base::course::load_course;
use teaching_proctor_base::config::{CrossOriginPolicy, Forwarding, Gate, SIGN_UP};
use proctor::instance::Proctor;
use semio_framework_async::CancelToken;
use serde_json::{json, Value};
use server::contract::EventRecord;
use server::gateway::{PresenceSettings, PRESENCE_PROTOCOL_V1};
use server::storage::StorageProfile;
use server::throttle::{Allowance, Limits, Rate};
use tungstenite::client::IntoClientRequest;
use tungstenite::http::{HeaderName, HeaderValue};
use tungstenite::{HandshakeError, Message, WebSocket};

#[path = "🧰️harness/🦀️.rs"]
mod harness;
use harness::*;

#[path = "🎮️learner-journey/🦀️.rs"]
mod learner_journey;

#[path = "🏆️challenge-sheets/🦀️.rs"]
mod challenge_sheets;

#[path = "🛠️operator/🦀️.rs"]
mod operator;
use operator::*;

#[path = "🛡️edge/🦀️.rs"]
mod edge;
use edge::*;
