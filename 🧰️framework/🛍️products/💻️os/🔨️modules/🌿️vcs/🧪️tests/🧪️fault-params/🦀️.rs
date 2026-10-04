use super::*;

/// 📢️ A full history refuses under `history.full` and names its capacity as the notice's `{n}` param (design §20.12) — the shells fill
/// the count from this structured value, never from the English message; every other refusal carries no param.
#[test]
fn a_full_history_names_its_capacity_as_the_notice_param() {
    use semio_framework_diagnostic::FaultFrom;
    let fault = VcsError::HistoryFull { capacity: 64 }.into_fault();
    assert_eq!((fault.code.0.as_str(), fault.param("n")), ("history.full", Some("64")));
    let replaying = VcsError::HistoryReplaying.into_fault();
    assert_eq!((replaying.code.0.as_str(), replaying.params), ("history.replaying", None));
}

/// 🐘️ An edit over its admitted rows refuses under the framework-labelled `mutation.too-large` (design §20.5, §20.12): the shells tell the
/// framework notice, so the fault names no param and no English row count reaches the person.
#[test]
fn an_oversized_edit_refuses_under_the_framework_too_large_code() {
    use semio_framework_diagnostic::FaultFrom;
    let fault = VcsError::TooLarge { rows: 65_537, capacity: 65_536 }.into_fault();
    assert_eq!((fault.code.0.as_str(), fault.params), ("mutation.too-large", None));
}
