//! 🧪️ Verifies headless activation against the instance lifecycle's neutral result vectors.

use super::*;

#[test]
fn readiness_requires_a_settled_ack_of_the_matching_captured_receipt() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let request: ActorInstanceOpenRequest = serde_json::from_value(corpus["request"].clone()).unwrap();
    let captured: ActorInstanceLifecycleReceipt = serde_json::from_value(corpus["captured"].clone()).unwrap();
    for (name, steps) in corpus["cases"].as_object().unwrap() {
        let mut acknowledging = false;
        for step in steps.as_array().unwrap() {
            let status: TurnStatus = serde_json::from_value(step["status"].clone()).unwrap();
            let receipt = if step["receipt"] == "captured" {
                Some(captured)
            } else {
                serde_json::from_value(step["receipt"].clone()).unwrap()
            };
            let actual = match instance_open_progress(request, acknowledging, &status, receipt) {
                Ok(InstanceOpenProgress::WaitForCapture) => "wait",
                Ok(InstanceOpenProgress::Acknowledge(exact)) => {
                    assert_eq!(exact, captured, "{name}");
                    acknowledging = true;
                    "ack"
                }
                Ok(InstanceOpenProgress::Live) => "live",
                Err(_) => "refused",
            };
            assert_eq!(step["progress"], actual, "{name}");
        }
    }
}
