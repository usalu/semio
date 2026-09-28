//! ⏳️ Shared control identities remain exact and progress stays localized at the full counter width.
use super::*;
#[test]
fn cancellation_outcomes_preserve_actual_worker_failures() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cancellationOutcomes"].as_array().unwrap(){let lane=cancellation_result_lane(row["userRequested"].as_bool().unwrap(),row["workerFault"].as_bool().unwrap());assert_eq!(lane,if row["lane"]=="terminal"{TypedOperationResultLane::Terminal}else{TypedOperationResultLane::Fault});}
}
#[test]
fn cancellation_identity_matches_the_neutral_schema_cases() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){let args=dsl::json::from_json_str(&row["args"].to_string()).unwrap();let value=decode_operation_cancellation(&args);assert_eq!(value.is_some(),row["valid"].as_bool().unwrap(),"{}",row["name"]);if let Some((operation,generation))=value{assert_eq!(format!("{operation:016x}"),row["args"]["operationId"].as_str().unwrap());assert_eq!(format!("{generation:016x}"),row["args"]["generation"].as_str().unwrap());}}
}
#[test]
fn progress_captions_preserve_counter_precision_and_explicit_locale() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for (key,locale) in [("en",Locale::En),("de",Locale::De)]{for count in fixture["counts"].as_array().unwrap(){let text=count.as_str().unwrap();for cancelling in [false,true]{let labels=&fixture["labels"][key];let expected=format!("{} · {}: {text}",labels[if cancelling{"cancelling"}else{"working"}].as_str().unwrap(),labels["units"].as_str().unwrap());assert_eq!(operation_progress_text(locale,text.parse().unwrap(),cancelling),expected);}}}
}
