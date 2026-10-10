//! 📣️ Laws of the wgpu shell's app fault notices (design §20.12), over the neutral corpus React's `appFaultNoticeV1` reads
//! (`🛂️manifest/🧫️fixtures/🧫️fault-notices`): a refused guest dispatch is told in the shell's language and terminology from
//! the framework's table first and the refusing app's published `faultNotices` second, `{name}` from `Fault.params` alone, and
//! mirrored as the polite `shell.notice` status; an undeclared code is React's generic refusal, never its raw code.

use super::*;

fn corpus() -> Value {
    serde_json::from_str(include_str!("../../../../../../../../../🔨️modules/🛂️manifest/🧫️fixtures/🧫️fault-notices/🔣️.json")).expect("the fault-notices corpus parses")
}

fn app_notices(corpus: &Value) -> Vec<semio_framework::FaultNoticeDefinition> {
    let table = corpus["tables"].as_array().expect("tables").iter().find(|table| table["id"] == corpus["appNotices"]).expect("the app table");
    serde_json::from_value(table["notices"].clone()).expect("published notices decode")
}

fn refused(row: &Value, notices: &[semio_framework::FaultNoticeDefinition], severity: semio_framework::Severity) -> RefusedGuestFault {
    let declared = &row["fault"];
    let mut fault = semio_framework::Fault::new(semio_framework::FaultOrigin::App, declared["code"].as_str().expect("code").to_string(), declared["message"].as_str().unwrap_or(""));
    fault.severity = severity;
    for (name, value) in declared["params"].as_object().into_iter().flatten() {
        fault = fault.with_param(name.clone(), value.as_str().expect("param"));
    }
    fault.causes = declared["causes"].as_array().into_iter().flatten().map(|cause| semio_framework::FaultCause { message: String::new(), code: Some(semio_framework::FaultCode::new(cause["code"].as_str().expect("cause").to_string())) }).collect();
    RefusedGuestFault { fault, notices: notices.to_vec() }
}

/// ⚖️ LAW: every corpus refusal is told with the corpus text and code — a framework history-lane code as a warning, an app code
/// with the fault's own severity — and an undeclared one as React's localized `dispatch-failed` notice without a code.
#[test]
fn every_corpus_refusal_is_told_in_the_shells_language() {
    let corpus = corpus();
    let notices = app_notices(&corpus);
    for severity in [semio_framework::Severity::Warning, semio_framework::Severity::Error] {
        for row in corpus["resolutions"].as_array().expect("resolutions") {
            let refused = refused(row, &notices, severity);
            let terminology = Terminology::parse(row["terminology"].as_str().expect("terminology")).expect("a terminology");
            let locale = Locale::parse(row["locale"].as_str().expect("locale")).expect("a locale");
            let error = format!("handle_action failed: {}: {}", refused.fault.code.0, refused.fault.message);
            let actual = classify_dispatch_fault_notice(&error, Some(&refused), terminology, locale);
            let expected = match row["expected"].as_object() {
                Some(expected) => {
                    let code = expected["code"].as_str().expect("code");
                    (expected["text"].as_str().expect("text").to_string(), if semio_framework::kernel::history_notice(code).is_some() { semio_framework::Severity::Warning } else { severity }, Some(code.to_string()))
                }
                None => (if locale == Locale::De { "Die Eingabe konnte nicht zugestellt werden." } else { "The input could not be delivered." }.to_string(), semio_framework::Severity::Info, None),
            };
            assert_eq!(actual, expected, "{} ({severity:?})", row["id"]);
        }
    }
}

/// ⚖️ LAW: the funnel takes the structured refusal recorded beside the string it classifies — the ARIA mirror's `shell.notice`
/// names the app notice by its text and describes it by its code — and ignores a recorded refusal naming another code.
#[test]
fn the_funnel_tells_the_recorded_refusal_and_mirrors_it_as_a_status() {
    let corpus = corpus();
    let notices = app_notices(&corpus);
    let row = corpus["resolutions"].as_array().expect("resolutions").iter().find(|row| row["id"] == "placeholder from params").expect("the params row");
    let mut shell = ShellState::new(Vec::new(), String::new(), semio_framework_ui_locale::Locale::De, semio_framework_ui_locale::Terminology::Native);
    shell.locale_id = "de".into();
    shell.chrome_build.refused_guest_fault = Some(refused(row, &notices, semio_framework::Severity::Warning));
    shell.note_dispatch_fault("handle_action failed: generation3d.gumball.kind-unavailable: widget kind brep.mesh.translate is unavailable");
    let told: Vec<(String, Option<String>, Option<String>)> = shell.chrome_accessibility_nodes(&[]).into_iter().filter(|node| node.key == TRANSIENT_NOTICE_STATUS_ID).map(|node| (node.role, node.label, node.description)).collect();
    assert_eq!(told, [("status".to_string(), Some("Die Widget-Art brep.mesh.translate ist hier nicht verfügbar.".to_string()), Some("generation3d.gumball.kind-unavailable".to_string()))]);
    assert!(shell.chrome_build.refused_guest_fault.is_none(), "the funnel consumes the recorded refusal");
    shell.chrome_build.refused_guest_fault = Some(refused(row, &notices, semio_framework::Severity::Warning));
    shell.note_dispatch_fault("surface render failed");
    let notice = shell.transient_notice().expect("a banner is showing");
    assert_eq!((notice.message.as_str(), notice.code.as_deref()), ("surface render failed", None), "a refusal naming another code is not this string's");
}

/// ⚖️ LAW (S4-BUMP's guest↔host channel handshake, corpus `📡️spr/🧵️channel/🧫️fixtures/🧫️channel-handshake`): every refused case —
/// an older or a newer guest, the host's `admit_guest_channel_version` fault — refused at an instance open reaches the person as
/// the framework's localized `plugin.channel-mismatch` notice naming both channels (en/de), mirrored as the polite `shell.notice`
/// described by the code, and the open's own status keeps that text; never the raw code or the English message.
#[test]
fn a_refused_guest_channel_is_told_as_its_localized_notice() {
    let corpus: Value = serde_json::from_str(include_str!("../../../../../../📡️spr/🧵️channel/🧫️fixtures/🧫️channel-handshake/🔣️.json")).expect("the channel-handshake corpus parses");
    let code = corpus["code"].as_str().expect("code");
    let refused: Vec<(String, i64, i64)> = corpus["cases"].as_array().expect("cases").iter().filter(|case| case["admitted"] == false).map(|case| (case["name"].as_str().expect("name").to_string(), case["guestOffset"].as_i64().expect("offset"), case["hostOffset"].as_i64().unwrap_or(0))).collect();
    assert!(!refused.is_empty(), "the corpus names refused guests");
    for (name, offset, host_offset) in refused {
        let current = i64::from(protocol::CHANNEL_VERSION);
        let host = u32::try_from(current + host_offset).expect("a host version");
        let guest = u32::try_from(current + offset).expect("a guest version");
        let fault = protocol::admit_guest_channel_version(guest, host).expect_err("the corpus case is refused");
        assert_eq!(fault.code.0, code, "{name}");
        for (locale, locale_id, expected) in [
            (semio_framework_ui_locale::Locale::En, "en", format!("This plugin was built for app channel {guest}, but this app speaks app channel {host} — rebuild the plugin.")),
            (semio_framework_ui_locale::Locale::De, "de", format!("Dieses Plugin wurde für App-Kanal {guest} gebaut, diese App spricht aber App-Kanal {host} — Plugin neu bauen.")),
        ] {
            let mut shell = ShellState::new(Vec::new(), String::new(), locale, semio_framework_ui_locale::Terminology::Native);
            shell.locale_id = locale_id.into();
            let detail = shell.note_refused_open(crate::program_bridge::ProgramFault { fault: Some(fault.clone()), frame: None, text: format!("create_app promise failed: {}", fault.describe()) });
            assert_eq!(detail, expected, "{name} {locale_id}: the open's status keeps the localized notice");
            let told: Vec<(String, Option<String>, Option<String>)> = shell.chrome_accessibility_nodes(&[]).into_iter().filter(|node| node.key == TRANSIENT_NOTICE_STATUS_ID).map(|node| (node.role, node.label, node.description)).collect();
            assert_eq!(told, [("status".to_string(), Some(expected.clone()), Some(code.to_string()))], "{name} {locale_id}: told politely, described by its code");
            assert!(!detail.contains(code), "{name} {locale_id}: never the raw code");
            if locale == semio_framework_ui_locale::Locale::De {
                assert!(!detail.contains("speaks app channel"), "{name} {locale_id}: never the English message");
            }
        }
    }
}
