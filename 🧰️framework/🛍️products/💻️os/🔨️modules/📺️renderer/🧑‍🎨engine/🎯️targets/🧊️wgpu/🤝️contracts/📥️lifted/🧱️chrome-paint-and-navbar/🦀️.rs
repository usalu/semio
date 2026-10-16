//! 🔗️ The hub remote status the footer pill and the hub projection share.
//!
//! Lifted verbatim out of the Shell (RENDERER-WP4, plan 5.2): visibility widened to `pub`, bodies unchanged. `contracts` mounts it with one `mod` line; the Shell keeps its originals until then and re-exports from `contracts` afterwards.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum ShellHubRemoteV1 {
    Detached,
    Connecting,
    Live {
        #[serde(rename = "peerCount")]
        peer_count: usize,
    },
    Backoff {
        #[serde(rename = "retryInMs")]
        retry_in_ms: u64,
    },
}
