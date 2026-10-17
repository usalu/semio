//! 🧮️ Independent required-bound corpus survives capture and maps unchanged into native receiving.
use super::*;
#[test]
fn original_tool_run_receiving_limits_preserve_every_required_captured_bound(){
 let corpus:serde_json::Value=serde_json::from_str(crate::fixture_text("🎞️tick/🛬️receive/🎟️limits/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();
 let policy:ToolRunReceivingLimits=serde_json::from_value(corpus["policy"].clone()).unwrap();
 assert_eq!(policy,ToolRunReceivingLimits::REPORTS);policy.validate().unwrap();
 let captured=policy;let mut edited=policy;edited.tick.maximum_file_bytes=1;assert_ne!(captured,edited);
 for owner in ["tick","recordBody","history"]{for field in corpus["policy"][owner].as_object().unwrap().keys(){let mut omitted=corpus["policy"].clone();omitted[owner].as_object_mut().unwrap().remove(field);assert!(serde_json::from_value::<ToolRunReceivingLimits>(omitted).is_err(),"{owner}.{field}")}}
 for original in [captured.tick,captured.record_body]{let native=original.record_body();assert_eq!(native.pack.max_file_len,u64::from(original.maximum_file_bytes));assert_eq!(native.pack.max_segment_len,u64::from(original.maximum_segment_bytes));assert_eq!(native.pack.max_symbols,original.maximum_symbols);assert_eq!(native.pack.max_depth,original.maximum_depth);assert_eq!(native.pack.max_items,u64::from(original.maximum_items));assert_eq!(native.pack.max_total_alloc,u64::from(original.maximum_total_allocation_bytes));assert_eq!(native.maximum_symbol_utf8_bytes,original.maximum_symbol_utf8_bytes as usize);assert_eq!(native.maximum_symbol_scalars,original.maximum_symbol_scalars as usize);assert_eq!(native.maximum_allocation_bytes,original.maximum_cursor_allocation_bytes as usize)}
 let mut invalid=captured;invalid.tick.maximum_cursor_allocation_bytes=invalid.tick.maximum_total_allocation_bytes+1;assert!(invalid.validate().is_err());
 assert_eq!(corpus["grant"],serde_json::json!([1,17,65536,262144,64]));
}
