//! 🔐️ A refused duplicate keeps the actual first request's live commit authority.
use super::*;
#[test]
fn duplicate_live_identity_preserves_first_actual_request(){
 let corpus:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 let router=ArtifactInferenceRouter::new();
 for step in corpus["steps"].as_array().unwrap(){let accepted=router.admit_live_identity(step["cancellationId"].as_str().unwrap(),step["revision"].as_u64().unwrap(),step["generation"].as_u64().unwrap()).is_ok();assert_eq!(accepted,step["accepted"].as_bool().unwrap());}
 let live=router.live_commits.lock().unwrap();
 let actual:Vec<_>=live.iter().map(|(id,(revision,generation))|serde_json::json!({"cancellationId":id,"revision":revision,"generation":generation})).collect();
 assert_eq!(serde_json::Value::Array(actual),corpus["expected"]);
}
