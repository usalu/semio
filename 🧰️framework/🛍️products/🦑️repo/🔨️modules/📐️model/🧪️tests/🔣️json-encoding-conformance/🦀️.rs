//! 🦀️ Rust side of the JSON encoding conformance case. The subject half is gated behind the `sut`
//! feature the generated host turns on for the subject role only.

use semio_repo_test_host::Adapter;

//#region 🔖️Scenarios
#[cfg(feature = "sut")]
mod subject {
    use semio_framework_repo_model as model;
    use semio_repo_test_host::{Context, Json, Outcome};

    const GOLDENS: &str = "shared://🔣️json-encoding-conformance/🔣️goldens.json";

    /// 📦️ Decodes every golden into its own domain type and encodes it again.
    pub fn golden_documents_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let mut encoded = Vec::new();
        for entry in ctx.fixture_json(GOLDENS)?.array("goldens") {
            let type_name = entry.str("type");
            let golden = entry.str("json");
            let output = model::round_trip_json(&type_name, &golden)?;
            encoded.push(Json::Object(vec![
                ("type".to_string(), Json::String(type_name)),
                ("encoded".to_string(), Json::String(output)),
            ]));
        }
        Ok(Outcome::projection(Json::Object(vec![("encoded".to_string(), Json::Array(encoded))])))
    }
}
//#endregion 🔖️Scenarios

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let registered = Adapter::new("rust");
    #[cfg(feature = "sut")]
    let registered = registered.subject("golden-documents-round-trip", subject::golden_documents_round_trip);
    registered
}
//#endregion 🔖️Registration
