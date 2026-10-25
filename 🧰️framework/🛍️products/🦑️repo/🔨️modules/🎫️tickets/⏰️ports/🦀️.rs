//! 🐛️ The clock, event sink and null issue tracker ports with their system and recording implementations.

use super::*;

//#region ⏰️Ports

/// ⏰️ The clock the lifecycle reads, so a case pins every timestamp.
pub trait Clock {
    /// 🎆️ Today as `(YY, MM, DD)`, the two-digit year the folder scheme uses.
    fn today(&self) -> (i64, i64, i64);
    /// 🖱️ The interaction timestamp, formatted `YYYY-MM-DD HH:MM:SS`.
    fn stamp(&self) -> String;
}

/// 📌️ A clock frozen at one instant.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedClock {
    pub date: (i64, i64, i64),
    pub timestamp: String,
}

impl FixedClock {
    /// 🆕️ Freezes a clock at a date and a timestamp.
    pub fn new(year: i64, month: i64, day: i64, timestamp: impl Into<String>) -> FixedClock {
        FixedClock { date: (year, month, day), timestamp: timestamp.into() }
    }
}

impl Clock for FixedClock {
    fn today(&self) -> (i64, i64, i64) {
        self.date
    }

    fn stamp(&self) -> String {
        self.timestamp.clone()
    }
}

/// 📡️ One emitted event: the kind, the source and the payload as JSON text.
///
/// The payload is carried as text on purpose — no serialisation type from outside this codebase
/// appears anywhere in the public surface of this crate.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordedEvent {
    pub kind: String,
    pub source: String,
    pub payload: String,
}

/// 📮️ Where the lifecycle sends its events.
pub trait EventSink {
    /// 📤️ Emits one event, its payload already encoded. Delivery failures are never fatal.
    fn emit(&self, kind: &str, source: &str, payload: &str);
}

/// 📼️ An in-memory sink that keeps every event in emission order.
#[derive(Debug, Default)]
pub struct RecordingEventSink {
    pub(super) events: RefCell<Vec<RecordedEvent>>,
}

impl RecordingEventSink {
    /// 🕳️ An empty sink.
    pub fn new() -> RecordingEventSink {
        RecordingEventSink::default()
    }

    /// 📹️ Everything emitted so far, in order.
    pub fn recorded(&self) -> Vec<RecordedEvent> {
        self.events.borrow().clone()
    }
}

impl EventSink for RecordingEventSink {
    fn emit(&self, kind: &str, source: &str, payload: &str) {
        self.events.borrow_mut().push(RecordedEvent { kind: kind.to_string(), source: source.to_string(), payload: payload.to_string() });
    }
}

/// 🌐️ The sink that posts to the repository coordinator through `📡️events`.
#[derive(Debug, Clone, Default)]
pub struct CoordinatorEventSink;

impl EventSink for CoordinatorEventSink {
    fn emit(&self, kind: &str, source: &str, payload: &str) {
        let Ok(value) = serde_json::from_str::<Value>(payload) else { return };
        semio_framework_repo_events::emit(kind, source, &value);
    }
}

/// 🕰️ The host wall clock, the production [`Clock`] of the ticket lifecycle.
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemClock;

impl Clock for SystemClock {
    fn today(&self) -> (i64, i64, i64) {
        let (year, month, day) = civil_from_days(unix_seconds() / 86_400);
        (year % 100, month, day)
    }

    fn stamp(&self) -> String {
        let seconds = unix_seconds();
        let (year, month, day) = civil_from_days(seconds / 86_400);
        let rest = seconds.rem_euclid(86_400);
        format!("{year:04}-{month:02}-{day:02} {:02}:{:02}:{:02}", rest / 3600, (rest % 3600) / 60, rest % 60)
    }
}

/// 🖥️ The host wall clock in whole seconds since the Unix epoch.
fn unix_seconds() -> i64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|value| value.as_secs() as i64).unwrap_or_default()
}

/// 📅️ Howard Hinnant's days-since-epoch to civil-date conversion.
fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let shifted = days + 719_468;
    let era = if shifted >= 0 { shifted } else { shifted - 146_096 } / 146_097;
    let day_of_era = shifted - era * 146_097;
    let year_of_era = (day_of_era - day_of_era / 1460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = if month_prime < 10 { month_prime + 3 } else { month_prime - 9 };
    (year_of_era + era * 400 + i64::from(month <= 2), month, day)
}

/// 🚫️ The tracker used when the caller asked for no management traffic: every call succeeds and
/// reports nothing, so a lifecycle runs unchanged with the network switched off.
#[derive(Debug, Default)]
pub struct NullIssueTracker;

impl IssueTracker for NullIssueTracker {
    fn create_issue(&self, _title: &str, _body: &str, _milestone: Option<i64>) -> ProviderResult<String> {
        Ok(String::new())
    }

    fn close_issue(&self, _issue_url: &str) -> ProviderResult<()> {
        Ok(())
    }

    fn reopen_issue(&self, _issue_url: &str) -> ProviderResult<()> {
        Ok(())
    }

    fn update_issue_title(&self, _issue_url: &str, _title: &str) -> ProviderResult<()> { Ok(()) }
    fn delete_issue(&self, _issue_url: &str) -> ProviderResult<()> { Ok(()) }

    fn get_issue_details(&self, _issue_url: &str) -> ProviderResult<Option<ManagementIssue>> {
        Ok(None)
    }

    fn add_comment(&self, _issue_url: &str, _comment: &str) -> ProviderResult<()> {
        Ok(())
    }

    fn add_labels(&self, _issue_url: &str, _labels: &[String]) -> ProviderResult<()> {
        Ok(())
    }

    fn remove_labels(&self, _issue_url: &str, _labels: &[String]) -> ProviderResult<()> {
        Ok(())
    }

    fn find_milestone_by_title(&self, _title: &str) -> ProviderResult<Option<ManagementMilestone>> {
        Ok(None)
    }

    fn list_repo_labels(&self) -> ProviderResult<Vec<ManagementLabel>> {
        Ok(Vec::new())
    }
}

/// 🏷️ The event source every ticket operation carries.
pub const EVENT_SOURCE: &str = "repo-cli";

//#endregion ⏰️Ports
