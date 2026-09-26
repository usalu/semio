use super::*;

#[test]
fn shared_fixture_reply_is_exact_and_strict() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).expect("temporal fixture");
    let row = &fixture["cases"][0];
    let values = serde_json::from_value(row["values"].clone()).expect("temporal values");
    let request = HostTemporalFormatRequestV1 { now_ms: row["nowMs"].as_i64().expect("now"), values };
    let reply = HostTemporalFormatReplyV1 {
        profile: HostTemporalProfileV1 { locale: "en-US".into(), time_zone: "America/New_York".into(), hour_cycle: HostHourCycleV1::H12, profile_revision: 8 },
        labels: request.values.iter().map(|value| HostTemporalLabelV1 { id: value.id.clone(), text: format!("host:{}", value.id) }).collect(),
    };
    assert!(reply.matches(&request));
    let mut partial = reply.clone();
    partial.labels.pop();
    assert!(!partial.matches(&request));
    let mut duplicate = reply;
    duplicate.labels[1].id = duplicate.labels[0].id.clone();
    assert!(!duplicate.matches(&request));
}
