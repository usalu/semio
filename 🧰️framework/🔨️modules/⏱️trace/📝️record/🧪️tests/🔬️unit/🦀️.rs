//! 🔬️ Laws of the structured span record: the language-agnostic vocabulary twin, level admission,
//! JSON rendering (held against a third-party parser, per `AGENTS.md`'s validate-with-one-external-
//! library rule for tests), the injected-sink contract a server's own handler laws build on, and
//! the bounded counter table an introspection route reads.
use super::*;

fn vocabulary() -> serde_json::Value {
    serde_json::from_str(include_str!("../../../🧫️fixtures/🛰️span-vocabulary/🔣️.json")).unwrap()
}

//#region 🌍️Vocabulary
#[test]
fn the_declared_events_match_the_language_agnostic_fixture() {
    let fixture = vocabulary();
    let declared: Vec<String> = fixture["events"].as_array().unwrap().iter().map(|entry| entry.as_str().unwrap().to_string()).collect();
    assert_eq!(declared, SERVER_SPAN_EVENTS.to_vec());
}

#[test]
fn the_declared_events_are_sorted_and_unique() {
    let mut sorted = SERVER_SPAN_EVENTS.to_vec();
    sorted.sort_unstable();
    sorted.dedup();
    assert_eq!(sorted, SERVER_SPAN_EVENTS.to_vec());
}

#[test]
fn every_outcome_carries_the_level_the_fixture_declares() {
    let fixture = vocabulary();
    let rows = fixture["outcomes"].as_array().unwrap();
    assert_eq!(rows.len(), TRACE_OUTCOMES.len());
    for (row, outcome) in rows.iter().zip(TRACE_OUTCOMES) {
        assert_eq!(row["name"].as_str().unwrap(), outcome.as_str());
        assert_eq!(row["level"].as_str().unwrap(), outcome.level().as_str());
    }
}

#[test]
fn every_level_spelling_round_trips_through_the_fixture_order() {
    let fixture = vocabulary();
    let declared: Vec<&str> = fixture["levels"].as_array().unwrap().iter().map(|entry| entry.as_str().unwrap()).collect();
    let levels = [TraceLevel::Off, TraceLevel::Error, TraceLevel::Warn, TraceLevel::Info, TraceLevel::Debug];
    assert_eq!(declared, levels.iter().map(|level| level.as_str()).collect::<Vec<_>>());
    for level in levels {
        assert_eq!(TraceLevel::parse(level.as_str()), Some(level));
    }
    assert_eq!(TraceLevel::parse("  INFO "), Some(TraceLevel::Info));
    assert_eq!(TraceLevel::parse("chatty"), None);
}
//#endregion 🌍️Vocabulary

//#region 🪜️Admission
#[test]
fn a_tracer_emits_only_what_its_level_admits() {
    let (tracer, sink) = Tracer::capturing(TraceLevel::Warn);
    tracer.emit(TraceRecord::new("server.boot", TraceOutcome::Ok));
    tracer.emit(TraceRecord::new("server.directory.command", TraceOutcome::Refused));
    tracer.emit(TraceRecord::new("server.document.socket", TraceOutcome::Failed));
    let events: Vec<String> = sink.records().into_iter().map(|record| record.event).collect();
    assert_eq!(events, vec!["server.directory.command".to_string(), "server.document.socket".to_string()]);
}

#[test]
fn a_disabled_tracer_says_and_counts_nothing() {
    let tracer = Tracer::disabled();
    tracer.emit(TraceRecord::new("server.boot", TraceOutcome::Failed));
    assert!(!tracer.enabled(TraceLevel::Error));
    assert!(tracer.counters().is_empty());
}

#[test]
fn an_unset_environment_is_info_on_stderr_and_an_unparseable_one_does_not_fail_the_boot() {
    assert_eq!(Tracer::from_lookup(|_| None).level(), TraceLevel::Info);
    assert_eq!(Tracer::from_lookup(|key| (key == TRACE_LEVEL_ENV).then(|| "debug".to_string())).level(), TraceLevel::Debug);
    assert_eq!(Tracer::from_lookup(|key| (key == TRACE_LEVEL_ENV).then(|| "loud".to_string())).level(), TraceLevel::Info);
    assert_eq!(Tracer::from_lookup(|key| (key == TRACE_SINK_ENV).then(|| "none".to_string())).level(), TraceLevel::Info);
}
//#endregion 🪜️Admission

//#region 🧾️Rendering
#[test]
fn a_full_record_renders_as_one_json_object_a_third_party_parser_reads_back() {
    let record = TraceRecord {
        level: TraceLevel::Warn,
        event: "server.directory.command".to_string(),
        outcome: TraceOutcome::Refused,
        request_id: Some("r00000000002a".to_string()),
        principal: Some("user:ada".to_string()),
        space: Some("space-1".to_string()),
        artifact: Some("doc-7".to_string()),
        duration_us: Some(1234),
        detail: Some("rate-limited".to_string()),
    };
    let line = record.to_json_line();
    assert!(!line.contains('\n'));
    let parsed: serde_json::Value = serde_json::from_str(&line).unwrap();
    assert_eq!(parsed["level"], "warn");
    assert_eq!(parsed["event"], "server.directory.command");
    assert_eq!(parsed["outcome"], "refused");
    assert_eq!(parsed["requestId"], "r00000000002a");
    assert_eq!(parsed["principal"], "user:ada");
    assert_eq!(parsed["space"], "space-1");
    assert_eq!(parsed["artifact"], "doc-7");
    assert_eq!(parsed["durationUs"], 1234);
    assert_eq!(parsed["detail"], "rate-limited");
}

#[test]
fn absent_fields_are_omitted_rather_than_written_as_null() {
    let parsed: serde_json::Value = serde_json::from_str(&TraceRecord::new("server.boot", TraceOutcome::Ok).to_json_line()).unwrap();
    assert_eq!(parsed, serde_json::json!({ "level": "info", "event": "server.boot", "outcome": "ok" }));
}

#[test]
fn a_detail_carrying_quotes_newlines_and_control_bytes_survives_the_round_trip() {
    let mut record = TraceRecord::new("server.document.socket", TraceOutcome::Failed);
    record.detail = Some("he said \"stop\"\n\tat \u{1}/path\\here".to_string());
    let parsed: serde_json::Value = serde_json::from_str(&record.to_json_line()).unwrap();
    assert_eq!(parsed["detail"].as_str().unwrap(), "he said \"stop\"\n\tat \u{1}/path\\here");
}
//#endregion 🧾️Rendering

//#region ⏱️Span
#[test]
fn a_closed_span_carries_its_identity_fields_and_a_duration() {
    let (tracer, sink) = Tracer::capturing(TraceLevel::Info);
    tracer.span("server.document.socket").request(tracer.allocate_request_id()).principal("user:ada").space("space-1").artifact("doc-7").ok();
    let records = sink.records_for("server.document.socket");
    assert_eq!(records.len(), 1);
    assert_eq!(records[0].outcome, TraceOutcome::Ok);
    assert_eq!(records[0].principal.as_deref(), Some("user:ada"));
    assert_eq!(records[0].space.as_deref(), Some("space-1"));
    assert_eq!(records[0].artifact.as_deref(), Some("doc-7"));
    assert_eq!(records[0].request_id.as_deref(), Some("r000000000000"));
    assert!(records[0].duration_us.is_some());
}

#[test]
fn an_opening_record_is_debug_level_so_an_info_operator_sees_one_line_per_request() {
    let (info_tracer, info_sink) = Tracer::capturing(TraceLevel::Info);
    let span = info_tracer.span("server.directory.command");
    span.started();
    span.ok();
    assert_eq!(info_sink.records().len(), 1);

    let (debug_tracer, debug_sink) = Tracer::capturing(TraceLevel::Debug);
    let span = debug_tracer.span("server.directory.command");
    span.started();
    span.ok();
    assert_eq!(debug_sink.records().iter().map(|record| record.outcome).collect::<Vec<_>>(), vec![TraceOutcome::Started, TraceOutcome::Ok]);
}

#[test]
fn request_ids_are_unique_within_one_tracer() {
    let tracer = Tracer::capturing(TraceLevel::Info).0;
    let ids: Vec<String> = (0..64).map(|_| tracer.allocate_request_id()).collect();
    let mut unique = ids.clone();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), ids.len());
}
//#endregion ⏱️Span

//#region 📊️Counters
#[test]
fn counters_tally_every_outcome_and_back_the_percentiles_with_the_reported_durations() {
    let (tracer, _sink) = Tracer::capturing(TraceLevel::Debug);
    tracer.span("server.auth.session.mint").ok();
    tracer.span("server.auth.session.mint").refused("invalid-credentials");
    tracer.span("server.auth.session.mint").failed("directory-unavailable");
    tracer.span("server.auth.session.mint").cancelled("peer-closed");
    tracer.span("server.auth.session.mint").started();
    let rows = tracer.counters();
    assert_eq!(rows.len(), 1);
    let counters = &rows[0].counters;
    assert_eq!((counters.ok, counters.refused, counters.failed, counters.cancelled, counters.started), (1, 1, 1, 1, 1));
    assert_eq!(counters.total(), 4);
    assert_eq!(counters.samples(), 4);
}

#[test]
fn counters_are_sorted_by_event_and_a_null_sink_still_counts() {
    let tracer = Tracer::new(TraceLevel::Info, Box::new(NullSink));
    tracer.span("server.readiness").ok();
    tracer.span("server.boot").ok();
    assert_eq!(tracer.counters().into_iter().map(|row| row.event).collect::<Vec<_>>(), vec!["server.boot".to_string(), "server.readiness".to_string()]);
}

#[test]
fn the_counter_table_is_bounded_and_says_how_much_it_dropped() {
    let tracer = Tracer::new(TraceLevel::Info, Box::new(NullSink));
    for index in 0..COUNTER_EVENT_CAPACITY + 8 {
        tracer.emit(TraceRecord::new(format!("server.synthetic.{index}"), TraceOutcome::Ok));
    }
    assert_eq!(tracer.counters().len(), COUNTER_EVENT_CAPACITY);
    assert_eq!(tracer.dropped_event_count(), 8);
    tracer.emit(TraceRecord::new("server.synthetic.0", TraceOutcome::Ok));
    assert_eq!(tracer.counters().iter().find(|row| row.event == "server.synthetic.0").unwrap().counters.ok, 2);
    assert_eq!(tracer.counters().len(), COUNTER_EVENT_CAPACITY);
    assert_eq!(tracer.dropped_event_count(), 8);
}
//#endregion 📊️Counters
