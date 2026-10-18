use crate::io::text::directory_command_sha256;
use super::*;
use crate::{DirectoryCommandOutcomeV1, DirectoryCommandResultV1};
use crate::schema::{DocumentOpenCheckpointV1, DocumentScope};
use crate::io::binary::checkpoint_pair::CANONICAL_CHECKPOINT_PAIR_MEDIA_TYPE_V1;
use semio_framework_async::{CancelToken, TraceId};
use semio_framework_test_fixtures::fixture_text;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, AtomicU32, AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};

include!("🔐️credential-wipe-fakes-and-event-page-laws/🦀️.rs");
include!("🛰️execution-target-admission-and-stream-laws/🦀️.rs");
include!("🚦️turns-receipts-cancellation-and-store-laws/🦀️.rs");
