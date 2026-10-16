//! 🚧️ Run errors: the run failure, the document-load progress record and the activation refusal.
//!
//! Holds the `🔖️Types` region of the module; the root re-exports its items.

use super::*;

//#region 🔖️Types

/// 🚧️ A failure computing a studio's workflow headlessly.
#[derive(Debug)]
pub enum RunError {
    UnknownNode(String),
    UnknownPort {
        edge_id: String,
        node_id: String,
        port_id: String,
    },
    Incompatible {
        edge_id: String,
        produced: MediaType,
        accepted: MediaType,
    },
    UnregisteredConversion {
        edge_id: String,
        from: MediaForm,
        to: MediaForm,
    },
    MissingRequiredInput {
        node_id: String,
        port_id: String,
    },
    MultiplicityViolation {
        node_id: String,
        port_id: String,
        count: usize,
    },
    NoConverter {
        class: MediaClass,
        from: MediaForm,
        to: MediaForm,
    },
    Cycle(Vec<String>),
    Host(String),
    Original(semio_framework_job::OriginalTurnFailure),
    Kernel(semio_framework_actor::KernelError),
    ActorCall(semio_framework_plugin_host::original_actor_context::OriginalActorFixedCall),
    Frame(semio_framework_actor::pack::PackError),
    /// 🤝️ The guest itself was refused at admission (`ActivationRefusal.fault`, e.g. `plugin.channel-mismatch` with its
    /// `guest`/`host` params): the structured fault, so the runner's caller tells its localized notice.
    Refused(semio_framework::Fault),
    Media(MediaError),
    Io {
        path: PathBuf,
        source: std::io::Error,
    },
    Serde(serde_json::Error),
    MutationRefused(Vec<protocol::MutationMessage>),
    /// 🛑️ `SpaceRunner`'s `OperationContext.cancel` was cancelled — checked at the top of
    /// `compute_node`, before that node's `open`/`exchange`, so a cancelled run stops before its
    /// NEXT node rather than mid-exchange (an in-flight `exchange` future is not itself
    /// preemptible — same honest limitation `semio-framework-os-services::ComputePool` documents).
    Cancelled,
}

/// 📊️ One node's whole-document load progress as its guest polls it (`📓️api-stepped-document-load.md` §2.2): the
/// operations folded so far of the operations the load folds, monotonic within one load. Ephemeral and local-only — handed
/// to the observer [`SpaceRunner::with_document_load_progress`] installs, never recorded into the run document.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeDocumentLoadProgress {
    pub node_id: String,
    pub completed: u64,
    pub total: u64,
}

impl std::fmt::Display for RunError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownNode(node) => write!(formatter, "unknown workflow node {node}"),
            Self::UnknownPort { edge_id, node_id, port_id } => write!(formatter, "workflow edge {edge_id} references unknown port `{port_id}` on node `{node_id}`"),
            Self::Incompatible { edge_id, produced, accepted } => write!(formatter, "workflow edge {edge_id} type mismatch: producer offers {produced:?}, consumer accepts {accepted:?}"),
            Self::UnregisteredConversion { edge_id, from, to } => write!(formatter, "workflow edge {edge_id} negotiated a conversion {from:?} -> {to:?} but no converter is registered for it"),
            Self::MissingRequiredInput { node_id, port_id } => write!(formatter, "input port `{port_id}` on node `{node_id}` is required but has no incoming edge"),
            Self::MultiplicityViolation { node_id, port_id, count } => write!(formatter, "input port `{port_id}` on node `{node_id}` accepts at most one connection but has {count}"),
            Self::NoConverter { class, from, to } => write!(formatter, "no media converter registered for {class:?}: {from:?} -> {to:?}"),
            Self::Cycle(nodes) => write!(formatter, "workflow has a cycle (unreachable nodes: {nodes:?})"),
            Self::Host(message) => write!(formatter, "host error: {message}"),
            Self::Original(error) => write!(formatter,"original run source refused: {error:?}"),
            Self::Kernel(error) => std::fmt::Display::fmt(error,formatter),
            Self::ActorCall(original) => write!(formatter,"original actor call refused: {original:?}"),
            Self::Frame(error) => std::fmt::Display::fmt(error, formatter),
            Self::Refused(fault) => write!(formatter, "{}: {}", fault.code.0, fault.message),
            Self::Media(error) => write!(formatter, "media error: {error}"),
            Self::Io { path, source } => write!(formatter, "io error at {}: {source}", path.display()),
            Self::Serde(error) => write!(formatter, "(de)serialization error: {error}"),
            Self::MutationRefused(messages) => write!(formatter, "run document refused an operation: {}", messages.iter().map(|message| format!("{}: {}", message.code.0, message.message)).collect::<Vec<_>>().join("; ")),
            Self::Cancelled => formatter.write_str("run cancelled"),
        }
    }
}

impl std::error::Error for RunError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Media(error) => Some(error),
            Self::Kernel(error) => Some(error),
            Self::Frame(error) => Some(error),
            Self::Io { source, .. } => Some(source),
            Self::Serde(error) => Some(error),
            _ => None,
        }
    }
}

/// 🚪️ An activation that installed no actor, as the runner reports it: the guest's own admission fault when it carries
/// one (audit F1), else the host's reason in words.
pub fn activation_refusal_error(refusal: semio_framework_plugin_host::activation::ActivationRefusal) -> RunError {
    match refusal.fault {
        Some(fault) => RunError::Refused(fault),
        None => RunError::Host(refusal.reason),
    }
}

impl From<semio_framework_actor::pack::PackError> for RunError {
    fn from(error: semio_framework_actor::pack::PackError) -> Self { Self::Frame(error) }
}

impl From<MediaError> for RunError {
    fn from(error: MediaError) -> Self {
        Self::Media(error)
    }
}

impl From<serde_json::Error> for RunError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serde(error)
    }
}

//#endregion 🔖️Types
