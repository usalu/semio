//! 🚨️ Binding errors of the hub-pair crate: Canonical checkpoint pair receiver, cache and binding errors.

use semio_framework_value::FromValue;
use semio_framework_value::ToValue;

/// ✂️ The longest transport detail a hub-unavailable refusal carries (characters), schema `HubUnavailableCauseV1`.
pub const HUB_UNAVAILABLE_DETAIL_MAX_CHARS: usize = 512;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HubBindingError {
    Cancelled,
    DeadlineExceeded,
    Unauthorized,
    SessionExpired,
    MembershipRequired,
    CapacityExceeded,
    InvalidResponse(&'static str),
    StaleRefresh,
    Unavailable(HubUnavailableCause),
}

/// 🔎️ What made the hub directory unavailable, so a refusal names it instead of one opaque word
/// (schema `HubUnavailableCauseV1`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum HubUnavailableCause {
    /// 🌐️ The HTTP status and, when the hub answered its typed refusal, that refusal's `code`.
    Http { status: u16, code: Option<String> },
    /// 🔌️ The transport failed before any HTTP answer; `detail` is its own bounded cause (an exhausted byte
    /// budget, a refused connection), never dropped.
    Transport { detail: String },
    /// 💰️ This gateway's own network byte budget is spent until its next refill turn — the hub is fine.
    NetworkBudget,
    UndecodableResponse,
}

impl HubUnavailableCause {
    /// 🌐️ An HTTP refusal, keeping the hub's typed `code` (bounded) when its body carries one.
    pub fn http(status: u16, body: &str) -> Self {
        let code = serde_json::from_str::<serde_json::Value>(body).ok().and_then(|value| value.get("code").and_then(serde_json::Value::as_str).map(|code| code.chars().take(64).collect()));
        Self::Http { status, code }
    }

    /// 🔌️ A transport failure, keeping its own detail bounded to [`HUB_UNAVAILABLE_DETAIL_MAX_CHARS`].
    pub fn transport(error: &semio_framework_os_kernel::os_directory::client::TransportError) -> Self {
        if matches!(error, semio_framework_os_kernel::os_directory::client::TransportError::BudgetExhausted) {
            return Self::NetworkBudget;
        }
        let detail = match error {
            semio_framework_os_kernel::os_directory::client::TransportError::Io(detail) => detail.clone(),
            other => other.to_string(),
        };
        Self::Transport { detail: detail.chars().take(HUB_UNAVAILABLE_DETAIL_MAX_CHARS).collect() }
    }

    /// 🧾️ The typed `HubUnavailableCauseV1` a refusal's `details.cause` carries.
    pub fn to_json(&self) -> serde_json::Value {
        match self {
            Self::Http { status, code } => serde_json::json!({ "kind": "http", "status": status, "code": code }),
            Self::Transport { detail } => serde_json::json!({ "kind": "transport", "detail": detail }),
            Self::NetworkBudget => serde_json::json!({ "kind": "network-budget" }),
            Self::UndecodableResponse => serde_json::json!({ "kind": "undecodable-response" }),
        }
    }

    /// 🗣️ The en + de sentence a client shows for this cause (`details.summary`).
    pub fn summary(&self) -> serde_json::Value {
        let (en, de) = match self {
            Self::Http { status, code: Some(code) } => (
                format!("The hub answered HTTP {status} ({code}); the request is retried once the hub recovers."),
                format!("Der Hub antwortete mit HTTP {status} ({code}); die Anfrage wird wiederholt, sobald der Hub wieder bereit ist."),
            ),
            Self::Http { status, code: None } => (
                format!("The hub answered HTTP {status}; the request is retried once the hub recovers."),
                format!("Der Hub antwortete mit HTTP {status}; die Anfrage wird wiederholt, sobald der Hub wieder bereit ist."),
            ),
            Self::Transport { detail } => (
                format!("The hub could not be reached ({detail}); the request is retried once the connection recovers."),
                format!("Der Hub war nicht erreichbar ({detail}); die Anfrage wird wiederholt, sobald die Verbindung wieder steht."),
            ),
            Self::NetworkBudget => (
                "This gateway's network allowance for this minute is used up; the request continues once it refills.".to_string(),
                "Das Netzwerkkontingent dieses Gateways für diese Minute ist aufgebraucht; die Anfrage läuft weiter, sobald es aufgefüllt ist.".to_string(),
            ),
            Self::UndecodableResponse => (
                "The hub's answer could not be read; the request is retried once the hub recovers.".to_string(),
                "Die Antwort des Hubs war nicht lesbar; die Anfrage wird wiederholt, sobald der Hub wieder bereit ist.".to_string(),
            ),
        };
        serde_json::json!({ "en": en, "de": de })
    }
}

impl std::fmt::Display for HubBindingError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Cancelled => formatter.write_str("hub descriptor refresh was cancelled"),
            Self::DeadlineExceeded => formatter.write_str("hub descriptor refresh exceeded its deadline"),
            Self::Unauthorized => formatter.write_str("hub session is unauthorized"),
            Self::SessionExpired => formatter.write_str("hub session is expired"),
            Self::MembershipRequired => formatter.write_str("current hub space membership is required"),
            Self::CapacityExceeded => formatter.write_str("hub descriptor index exceeded its fixed document capacity"),
            Self::InvalidResponse(detail) => write!(formatter, "hub directory response was invalid: {detail}"),
            Self::StaleRefresh => formatter.write_str("hub descriptor refresh was superseded"),
            Self::Unavailable(HubUnavailableCause::Http { status, code: Some(code) }) => write!(formatter, "hub directory is temporarily unavailable (HTTP {status} {code})"),
            Self::Unavailable(HubUnavailableCause::Http { status, code: None }) => write!(formatter, "hub directory is temporarily unavailable (HTTP {status})"),
            Self::Unavailable(HubUnavailableCause::Transport { detail }) => write!(formatter, "hub directory is temporarily unavailable (transport: {detail})"),
            Self::Unavailable(HubUnavailableCause::NetworkBudget) => formatter.write_str("hub directory is temporarily unavailable (network byte budget exhausted until its next refill)"),
            Self::Unavailable(HubUnavailableCause::UndecodableResponse) => formatter.write_str("hub directory is temporarily unavailable (undecodable response)"),
        }
    }
}

impl std::error::Error for HubBindingError {}
