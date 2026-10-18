//! ✏️ Store laws of non-destructive history editing: effective forwards under supersessions, Report-mode replays with
//! per-mutation outcomes, the finalize gate, scoped and unscoped supersessions, replica convergence, cancellation,
//! persistence, convergence early exit and the prefix-snapshot ring (ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING).
use super::*;

include!("🧰️supersede-replay-harness-fixtures/🦀️.rs");
include!("🧫️supersede-corpus-and-interior-supersede-laws/🦀️.rs");
include!("🔂️replica-convergence-cancellation-and-prefix-ring-laws/🦀️.rs");
include!("🪢️fold-site-hydration-and-oversize-message-laws/🦀️.rs");
include!("🌲️trunk-history-shape-and-command-decode-laws/🦀️.rs");
include!("↩️retraction-and-bounded-history-read-laws/🦀️.rs");
