//! ⚠️ Flow host errors: the `FlowCoreError` taxonomy and the content-addressed shared payload registry.

use super::*;

// #region ⚠️ Errors
/// 🧯️ `FlowHost`'s error type — wraps JSON codec failures, the `dag` crate's own `DagError`, and
/// this crate's own graph-editing validation failures. Every variant's Display text is byte-for-byte
/// identical to the `String` it replaces, so downstream `.to_string()` call sites and JSON error
/// envelopes are unaffected.
#[derive(Debug,semio_framework_value::RetireOwned)]
pub enum FlowCoreError {
    /// 🌉️ Holds the formatted message rather than a codec error type directly — `pack::json`'s
    /// `JsonError` (syntax) and the value derive's `ValueError` (shape) both fold into this,
    /// matching the Display text `serde_json::Error` used to produce.
    Json(String),
    Dag(dag::DagError),
    WidgetIdExists(String),
    UnknownWidget(String),
    UnknownNeuronWidget(String),
    NotVariadicInput(String),
    NotVariadicOutput(String),
    NotNeuron(String),
    WidgetNotNeuron(String),
    MaxInputPortsReached(String),
    MaxOutputPortsReached(String),
    UnknownInputPort(String),
    UnknownOutputPort(String),
    MinInputPorts {
        widget: String,
        min: usize,
    },
    MinOutputPorts {
        widget: String,
        min: usize,
    },
    NoOutputPort(String),
    NoInputPort(String),
    SelfConnection,
    /// 🔌️ The drawn wire would carry a value the target port does not accept — the source port's
    /// declared value schemas and the target's are disjoint. Both sides are carried so a surface can
    /// name them in the user's own language.
    IncompatiblePortTypes {
        source: String,
        source_type: String,
        target: String,
        target_type: String,
    },
    SelfInsertion,
    CycleWouldBeCreated,
    ConnectionAlreadyExists,
    UnknownSynapse(String),
    UnknownWidgetLayout(String),
    CollapseNeedsTwoWidgets,
    CollapseUnknownWidgets,
    CollapseContainsClusters,
    UnknownCluster(String),
    WidgetNotCluster(String),
    /// 📦️ A content-addressed payload referenced a digest this guest process does not hold, so the
    /// sender must carry the body again — see [`resolve_flow_shared_payload`].
    UnknownSharedPayload(String),
    /// 🧾️ The history store refused a gesture's leaves. No partial history row survives: the history was reset to the live
    /// content, so earlier undo steps are gone, and the carried text names the refusal.
    HistoryRefused(String),
}

impl std::fmt::Display for FlowCoreError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => formatter.write_str(error),
            Self::Dag(error) => std::fmt::Display::fmt(error, formatter),
            Self::WidgetIdExists(id) => write!(formatter, "widget id already exists: {id}"),
            Self::UnknownWidget(id) => write!(formatter, "unknown widget: {id}"),
            Self::UnknownNeuronWidget(id) => write!(formatter, "unknown neuron widget: {id}"),
            Self::NotVariadicInput(id) => write!(formatter, "{id} is not variadic"),
            Self::NotVariadicOutput(id) => write!(formatter, "{id} is not variadic output"),
            Self::NotNeuron(id) => write!(formatter, "{id} is not a neuron"),
            Self::WidgetNotNeuron(id) => write!(formatter, "widget is not a neuron: {id}"),
            Self::MaxInputPortsReached(id) => write!(formatter, "{id} reached max input ports"),
            Self::MaxOutputPortsReached(id) => write!(formatter, "{id} reached max output ports"),
            Self::UnknownInputPort(id) => write!(formatter, "unknown input port: {id}"),
            Self::UnknownOutputPort(id) => write!(formatter, "unknown output port: {id}"),
            Self::MinInputPorts { widget, min } => write!(formatter, "{widget} requires at least {min} inputs"),
            Self::MinOutputPorts { widget, min } => write!(formatter, "{widget} requires at least {min} outputs"),
            Self::NoOutputPort(id) => write!(formatter, "{id} has no output port"),
            Self::NoInputPort(id) => write!(formatter, "{id} has no input port"),
            Self::SelfConnection => formatter.write_str("cannot connect widget to itself"),
            Self::IncompatiblePortTypes { source, source_type, target, target_type } => write!(formatter, "{source} carries {source_type}, {target} accepts {target_type}"),
            Self::SelfInsertion => formatter.write_str("cannot insert widget between itself"),
            Self::CycleWouldBeCreated => formatter.write_str("connection would create cycle"),
            Self::ConnectionAlreadyExists => formatter.write_str("connection already exists"),
            Self::UnknownSynapse(id) => write!(formatter, "unknown synapse: {id}"),
            Self::UnknownWidgetLayout(id) => write!(formatter, "unknown widget layout: {id}"),
            Self::CollapseNeedsTwoWidgets => formatter.write_str("select at least two widgets to collapse"),
            Self::CollapseUnknownWidgets => formatter.write_str("selection contains unknown widgets"),
            Self::CollapseContainsClusters => formatter.write_str("cannot collapse clusters"),
            Self::UnknownCluster(id) => write!(formatter, "unknown cluster: {id}"),
            Self::WidgetNotCluster(id) => write!(formatter, "widget is not a cluster: {id}"),
            Self::UnknownSharedPayload(digest) => write!(formatter, "unknown shared payload: {digest}"),
            Self::HistoryRefused(reason) => write!(formatter, "history refused the edit: {reason}"),
        }
    }
}

//#region 📦️SharedPayloads
/// 📦️ Marks a content-addressed flow payload. `@<digest>\n<body>` CARRIES a body and registers it
/// under that digest; `@<digest>` alone REFERENCES a body this guest process already holds; anything
/// else is a bare body.
///
/// The app-static operator catalogue is the same bytes for every board in a page — 88 438 B of
/// `setNeuronKindInfosJson` and 22 849 B of `setCatalogueJson` — and every flow session used to
/// re-cross both when its surface attached (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️flow-scroll-render-perf-2026-09-15.md` §9). Every session in a page lives in ONE wasm module
/// with one linear memory, so a second board can name what the first delivered instead of moving it
/// again, and a generation that really did change carries a new digest and therefore a new body.
pub const FLOW_SHARED_PAYLOAD_PREFIX: char = '@';

/// 🧩️ Separates the PARTS of one payload. A payload the host composes from several sources — the
/// operator table is the app-static catalogue plus whatever records the scene itself derived — is
/// carried part by part, so appending 1 805 B of scene operators names the 98 642 B of app operators
/// the guest already holds instead of re-crossing them (measured, `🐍️flow-surface-followup-probe.mjs`).
pub const FLOW_SHARED_PAYLOAD_PART_SEPARATOR: char = '\u{1e}';

thread_local! {
    /// 🗂️ The bodies this guest process holds, by content digest. Plain `String`s rather than the
    /// parsed catalogue: a parsed `OperatorInfo` map carries `Value`s that fail closed on a bare
    /// drop, and a process-wide registry has no owner to retire it.
    static FLOW_SHARED_PAYLOAD_BODIES: std::cell::RefCell<HashMap<String, String>> = std::cell::RefCell::new(HashMap::new());
}

/// 📮️ Resolves a content-addressed payload to its body, registering a carried one under its digest.
///
/// Answers [`FlowCoreError::UnknownSharedPayload`] for a reference this process cannot resolve, so a
/// sender whose note of what the guest holds is wrong learns it and carries the body again instead of
/// installing an empty catalogue.
pub fn resolve_flow_shared_payload(payload: &str) -> Result<String, FlowCoreError> {
    let Some(reference) = payload.strip_prefix(FLOW_SHARED_PAYLOAD_PREFIX) else {
        return Ok(payload.to_string());
    };
    match reference.split_once('\n') {
        Some((digest, body)) => {
            FLOW_SHARED_PAYLOAD_BODIES.with(|bodies| bodies.borrow_mut().insert(digest.to_string(), body.to_string()));
            Ok(body.to_string())
        }
        None => FLOW_SHARED_PAYLOAD_BODIES.with(|bodies| bodies.borrow().get(reference).cloned()).ok_or_else(|| FlowCoreError::UnknownSharedPayload(reference.to_string())),
    }
}

/// 🧩️ Resolves every part of a composed payload, in the order the sender wrote them.
pub fn resolve_flow_shared_payload_parts(payload: &str) -> Result<Vec<String>, FlowCoreError> {
    payload.split(FLOW_SHARED_PAYLOAD_PART_SEPARATOR).map(resolve_flow_shared_payload).collect()
}

/// 🧹️ Forgets every registered shared payload — the retirement seam a test needs to prove that a
/// reference to an unheld digest fails closed.
pub fn retire_flow_shared_payloads() {
    FLOW_SHARED_PAYLOAD_BODIES.with(|bodies| bodies.borrow_mut().clear());
}
//#endregion 📦️SharedPayloads

impl std::error::Error for FlowCoreError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Dag(error) => Some(error),
            _ => None,
        }
    }
}

impl From<semio_framework_pack_json::JsonError> for FlowCoreError {
    fn from(error: semio_framework_pack_json::JsonError) -> Self {
        Self::Json(error.to_string())
    }
}

impl From<ValueError> for FlowCoreError {
    fn from(error: ValueError) -> Self {
        Self::Json(error.to_string())
    }
}

impl From<dag::DagError> for FlowCoreError {
    fn from(error: dag::DagError) -> Self {
        Self::Dag(error)
    }
}
// #endregion ⚠️ Errors
