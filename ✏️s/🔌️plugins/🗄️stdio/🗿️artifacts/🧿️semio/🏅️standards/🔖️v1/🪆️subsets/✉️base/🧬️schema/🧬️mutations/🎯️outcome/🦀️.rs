//! 🎯️ Reads the refusals out of any semio mutation outcome, for adapters that cannot name the kernel's protocol types.

/// 🚦️ The messages in `outcome` that genuinely REFUSE the mutation — `Error` and `Fatal` only,
/// rendered for a failure report. The frozen mutation-outcome contract
/// (`26/08/16/MUTATION-OUTCOMES-MERGE-POLICIES-AND-FIRST-CLASS-CONFLICTS/📋️contract-freeze.md` §C2)
/// makes `Info`/`Warning` ADVISORY: they ride along with a diff that WAS applied in full, and the
/// contract's own worked example is `.info("mutation.cascade", …)` — which `🧊️brep`'s
/// `delete-vertex` and `🕸️graph`'s `delete-node` both raise on every well-formed body, naming the
/// edges the deletion also had to remove. A caller that reads "any message" as "rejected" therefore
/// reports a refusal that never happened and fails a scenario the codec answered correctly.
///
/// Generic over the diff type and declared ONCE here rather than eighteen times, because the
/// question is the envelope's, not any one arm's: every `mutate-semio-*` adapter needs it, and none
/// of them can name this crate's private `protocol` extern-crate alias to ask it directly.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_mutation_refusals<D>(outcome: &protocol::MutationOutcome<D>) -> Vec<String> {
    outcome.messages().iter().filter(|message| message.level >= semio_framework_diagnostic::Severity::Error).map(|message| format!("{:?} {:?}: {}", message.level, message.code, message.message)).collect()
}
/// 🚦️ The FAULT CODES of the refusing messages in `outcome`, in order — the same `Error`/`Fatal`
/// filter [`semio_mutation_refusals`] applies, reduced to the frozen `mutation.*` code alone. The
/// envelope's own routing law is stated in terms of codes (a mismatched arm must raise exactly
/// `mutation.target-missing`), and `✉️mutate-semio-base` cannot read `MutationMessage::code` itself
/// because `protocol` is a private extern-crate alias of this crate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn semio_mutation_refusal_codes<D>(outcome: &protocol::MutationOutcome<D>) -> Vec<String> {
    outcome.messages().iter().filter(|message| message.level >= semio_framework_diagnostic::Severity::Error).map(|message| message.code.0.clone()).collect()
}
