//! 💼️ The well-known cold job kinds of [`../📜️jobs.wit`](../📜️jobs.wit): the one definition the guest reactor registers and the host routers call.

/// 🗺️ Single-hop `io-run` of a plugin's own registry; `input` is the JSON `(source, target, IoPayload)` tuple.
pub const JOB_KIND_IO_RUN: &str = "semio.io-run";
/// 🗺️ Single-hop `io-sniff`; the result is one confidence rank byte.
pub const JOB_KIND_IO_SNIFF: &str = "semio.io-sniff";
/// 💡️ Artifact inference.
pub const JOB_KIND_INFER: &str = "semio.infer";
/// 🧬️ Composite mutation planning.
pub const JOB_KIND_MUTATION_PLAN: &str = "semio.mutation-plan";
/// 🔀️ Artifact migration.
pub const JOB_KIND_MIGRATE: &str = "semio.migrate";
/// 🧩️ One-hop compose, routed by the host; the guest registers it as the `compose-await` packet, not as a builtin job.
pub const JOB_KIND_COMPOSE: &str = "semio.compose";
