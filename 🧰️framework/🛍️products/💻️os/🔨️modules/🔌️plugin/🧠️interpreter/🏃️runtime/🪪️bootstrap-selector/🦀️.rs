//! 🪪️ Selects pre-Source bootstrap before lending the authenticated scalar header.
use semio_framework_value::ValueError;
use crate::OwnedSemioExport;

/// 🪪️ Selects the authenticated bootstrap header for an actor-context export.
pub fn select_bootstrap(export: OwnedSemioExport, header: &[u8]) -> Result<bool, ValueError> {
    if header.len() != 304 { return Ok(false); }
    if !matches!(export, OwnedSemioExport::AdmitActorContext | OwnedSemioExport::CloseActorContext) {
        return Err(ValueError::invariant("bootstrap header requires its actual ActorContext export"));
    }
    if header[..8] != [83, 65, 66, 49, 1, 0, 48, 1] {
        return Err(ValueError::invariant("bootstrap header changed its closed namespace"));
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn genuine_job_and_failure_headers_reach_escrow_without_granting_bootstrap_authority() {
        let cases: serde_json::Value = serde_json::from_str(include_str!("../../../🖥️host/🎭️guest-runtime/🪙️escrow/🧪️tests/bootstrap-selector.json")).unwrap();
        for row in cases.as_array().unwrap() {
            let export = match row["exportName"].as_str().unwrap() {
                "admitActorContext" => OwnedSemioExport::AdmitActorContext,
                "closeActorContext" => OwnedSemioExport::CloseActorContext,
                "startJob" => OwnedSemioExport::StartJob,
                "stepJob" => OwnedSemioExport::StepJob,
                "publishActorFailure" => OwnedSemioExport::PublishActorFailure,
                other => panic!("unrecognized fixture export {other}"),
            };
            let mut header = vec![0; row["length"].as_u64().unwrap() as usize];
            let prefix: Vec<u8> = serde_json::from_value(row["prefix"].clone()).unwrap();
            header[..prefix.len()].copy_from_slice(&prefix);
            let observed = match select_bootstrap(export, &header) { Ok(true) => "bootstrap", Ok(false) => "escrow", Err(_) => "refused" };
            assert_eq!(observed, row["outcome"].as_str().unwrap(), "{}", row["name"]);
        }
    }
}
