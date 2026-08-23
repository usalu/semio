//! 🦀️ ZIP creation and editing case — Rust adapter.
//!
//! The oracle writes and edits the archive with the `zip` reference implementation; the subject
//! decodes the same artifact with this repository's `decode_zip`, applies the same edit to the typed
//! snapshot, and re-encodes with `encode_zip`. Both results are read back by the independent reader
//! before the `semantic-archive-v1` profile compares the members as a set.

use semio_s_plugin_stdio_test_oracle::archive::{oracle_create_zip, oracle_remove_zip_entry, project_zip, ArchiveSpec};
use semio_repo_test_host::{Adapter, Context, Json, Outcome};

//#region 🔖️Input
const ROUND_TRIP: [&str; 2] = ["three-members-round-trip", "empty-archive-round-trips"];
const REMOVE: &str = "removing-a-member";

/// 🧫️ The scenario's archive description, read from the feature so both producers share one input.
fn spec(ctx: &Context) -> Result<ArchiveSpec, String> {
    Ok(ArchiveSpec::from_json(&ctx.doc_json()?))
}

/// 🗑️ The member name the scenario removes.
fn removed_name(ctx: &Context) -> Result<String, String> {
    match ctx.doc_json()?.get("remove") {
        Some(Json::String(name)) => Ok(name.clone()),
        _ => Err("scenario declares no member to remove".to_string()),
    }
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn oracle_round_trip(ctx: &Context) -> Result<Outcome, String> {
    let bytes = oracle_create_zip(&spec(ctx)?)?;
    let projection = project_zip(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

fn oracle_remove(ctx: &Context) -> Result<Outcome, String> {
    let bytes = oracle_remove_zip_entry(&oracle_create_zip(&spec(ctx)?)?, &removed_name(ctx)?)?;
    let projection = project_zip(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{removed_name, spec};
    use semio_s_plugin_stdio_test_oracle::archive::{oracle_create_zip, project_zip};
    use semio_repo_test_host::{Context, Outcome};
    use semio_s_plugin_stdio::artifacts::zip::standards::v2_0::subsets::any::io::{decode_zip, encode_zip};

    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let reference = oracle_create_zip(&spec(ctx)?)?;
        let snapshot = decode_zip(&reference).map_err(|error| format!("decode_zip failed: {:?}", error))?;
        let bytes = encode_zip(&snapshot).map_err(|error| format!("encode_zip failed: {:?}", error))?;
        let projection = project_zip(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn remove(ctx: &Context) -> Result<Outcome, String> {
        let reference = oracle_create_zip(&spec(ctx)?)?;
        let mut snapshot = decode_zip(&reference).map_err(|error| format!("decode_zip failed: {:?}", error))?;
        let name = removed_name(ctx)?;
        let before = snapshot.entries.len();
        snapshot.entries.retain(|entry| entry.name != name);
        if snapshot.entries.len() == before {
            return Err(format!("member {} was not present to remove", name));
        }
        let bytes = encode_zip(&snapshot).map_err(|error| format!("encode_zip failed: {:?}", error))?;
        let projection = project_zip(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for scenario in ROUND_TRIP {
        built = built.oracle(scenario, oracle_round_trip);
        #[cfg(feature = "sut")]
        {
            built = built.subject(scenario, subject::round_trip);
        }
    }
    built = built.oracle(REMOVE, oracle_remove);
    #[cfg(feature = "sut")]
    {
        built = built.subject(REMOVE, subject::remove);
    }
    built
}
//#endregion 🔖️Registration
