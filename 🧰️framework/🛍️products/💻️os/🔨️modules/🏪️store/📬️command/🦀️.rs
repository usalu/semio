//! 📬️ The command text and binary formats: the `.ops` header grammar, command header lines, supersede input lines and
//! the command transaction codec shared by the text format, the history fold and the composition coordinator.
//!
//! Layering: `📓️2026-10-13-wave-e1-os-kernel.md` (ticket BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT).
use protocol::{MutationId, UndoPolicy, Mutation, MutationDiff, OpBinary, OpText};
use semio_framework_os_vcs::{Author, VcsError};
use semio_framework_diagnostic::{TextError, TextSpan};
use semio_framework_os_pack::{PackDecodeOptions, PackEncodeOptions, PackError, PackRefusal, pack_rt};
#[path="🧬️schema/🦀️.rs"]
mod schema;
pub use schema::{ArtifactCommand, HistoryLane, OpenToolTransaction, SupersedeInput};
#[path="♻️retirement/🦀️.rs"]
mod retirement;

#[path="🛬️receiving/🦀️.rs"]
pub mod receiving;

#[path = "📜️ops-header-lines/🦀️.rs"]
mod ops_header_lines;
pub use ops_header_lines::{
    ops_line_provenance, OpsAuthor, OpsHeaderLine, OpsPin, OpsSupersededInput,
};

#[path = "🏷️command-header-supersede-inputs-and-policy-tokens/🦀️.rs"]
mod command_header_supersede_inputs_and_policy_tokens;
pub use command_header_supersede_inputs_and_policy_tokens::{
    conflict_resolution_from_ordinal, conflict_resolution_ordinal, conflict_resolution_to_token,
    history_lane_from_ordinal, history_lane_ordinal, history_lane_to_token, merge_policy_to_token,
    parse_conflict_resolution_token, parse_history_lane_token, parse_merge_policy_token,
    parse_supersede_input_lines, parse_supersede_inputs, parse_undo_policy_token,
    print_supersede_inputs, read_supersede_input, read_supersede_inputs, undo_policy_from_ordinal,
    undo_policy_ordinal, undo_policy_to_token, write_supersede_inputs, CommandHeaderLine,
    SupersedeInputLine,
};

#[path = "📝️command-print-parse-and-codec/🦀️.rs"]
mod command_print_parse_and_codec;
pub use command_print_parse_and_codec::{
    dedent_command_lines, parse_command, parse_indented_ops, print_command, print_indented_ops,
    read_command_op, read_command_ops, read_command_str, read_command_transaction,
    transaction_from_text, transaction_text, write_command_ops, write_command_str,
    write_command_transaction, COMMAND_BINARY_FORMAT,
};

#[path = "🧯️command-decode-error-and-retirement/🦀️.rs"]
mod command_decode_error_and_retirement;
pub use command_decode_error_and_retirement::{
    retire_command, retire_operations, retire_scratch_edits, retire_scratch_operations,
    retire_supersede_inputs, CommandDecodeError,
};
use command_decode_error_and_retirement::operation_envelope_serde;

#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;
