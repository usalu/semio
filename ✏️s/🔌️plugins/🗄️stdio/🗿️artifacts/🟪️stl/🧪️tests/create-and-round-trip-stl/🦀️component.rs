//! 🦀️ STL creation and round-trip case — Rust adapter.
//!
//! The oracle writes the mesh with the registered reference implementation; the subject decodes that
//! same artifact with this repository's reader and re-encodes it with this repository's writer. Both
//! results are read back by the INDEPENDENT reader before the `semantic-mesh-v1` profile compares
//! them. The subject half is gated behind the generated host's `sut` feature.

use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_plugin_stdio_test_oracle::mesh::{oracle_create_stl, project_stl, MeshSpec};

//#region 🔖️Input
const SCENARIOS: [&str; 2] = ["tetrahedron-round-trips", "quad-round-trips"];

/// 🧫️ The scenario's mesh description, read from the feature so both producers share one input.
fn spec(ctx: &Context) -> Result<MeshSpec, String> {
    Ok(MeshSpec::from_json(&ctx.doc_json()?))
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn oracle(ctx: &Context) -> Result<Outcome, String> {
    let bytes = oracle_create_stl(&spec(ctx)?)?;
    let projection = project_stl(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::spec;
    use semio_repo_test_host::{Context, Outcome};
    use semio_s_plugin_stdio_test_oracle::mesh::{oracle_create_stl, project_stl};
    use semio_s_plugin_stdio::artifacts::stl::standards::ascii::subsets::any::io::{decode_stl_binary, encode_stl_binary};

    pub fn run(ctx: &Context) -> Result<Outcome, String> {
        let reference = oracle_create_stl(&spec(ctx)?)?;
        let snapshot = decode_stl_binary(&reference).map_err(|error| format!("decode_stl_binary failed: {}", error))?;
        let bytes = encode_stl_binary(&snapshot);
        let projection = project_stl(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for scenario in SCENARIOS {
        built = built.oracle(scenario, oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(scenario, subject::run);
        }
    }
    built
}
//#endregion 🔖️Registration
