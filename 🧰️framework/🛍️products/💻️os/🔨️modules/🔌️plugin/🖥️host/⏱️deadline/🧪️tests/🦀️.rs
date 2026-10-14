//! 🕰️ The original absolute deadline never renews when another caller enters.
#[path="../🦀️.rs"]
mod original_deadline;
use original_deadline::{OriginalDeadlineError,remaining_original_deadline_us};
#[test]
fn original_host_deadline_preserves_clock_units_and_expired_boundary(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for case in fixture["cases"].as_array().unwrap(){let now=case["now"].as_u64();let actual=remaining_original_deadline_us(now,case["deadline"].as_u64().unwrap());match case["error"].as_str(){Some("expired")=>assert_eq!(actual,Err(OriginalDeadlineError::Expired)),Some("missing-clock")=>assert_eq!(actual,Err(OriginalDeadlineError::MissingClock)),None=>{let remaining=actual.unwrap();assert_eq!(remaining,case["remaining"].as_u64().unwrap());assert_eq!(remaining.div_ceil(1000),case["wakeMs"].as_u64().unwrap());},_=>panic!("invalid original clock fixture")}}
 assert_eq!(remaining_original_deadline_us(Some(0),u64::MAX).unwrap().div_ceil(1000),u64::MAX.div_ceil(1000));
 eprintln!("[DEBUG] Original Host deadline exactUs=true unchangedDeadline=true expiredBoundary=true missingClockRefused=true wakeRoundingOnly=true maximumDeadlineNoOverflow=true");
}
