//! 🦀️ GIF creation and round-trip case — Rust adapter.
//!
//! GIF is palette-based, so exact RGBA is a quantizer choice and the projection deliberately reports
//! frame geometry and opaque-sample counts. The oracle writes with the `gif` reference encoder; the
//! subject decodes that artifact with this repository's `decode_gif` and re-encodes with
//! `encode_gif`. Both are read back by the independent decoder.

use semio_repo_test_host::oracle_raster::{oracle_create_gif, project_gif, RasterSpec};
use semio_repo_test_host::{Adapter, Context, Outcome};

//#region 🔖️Input
const SCENARIOS: [&str; 2] = ["single-frame-round-trips", "non-square-frame-round-trips"];

/// 🧫️ The scenario's image description, read from the feature so both producers share one input.
fn spec(ctx: &Context) -> Result<RasterSpec, String> {
    Ok(RasterSpec::from_json(&ctx.doc_json()?))
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn oracle(ctx: &Context) -> Result<Outcome, String> {
    let bytes = oracle_create_gif(&spec(ctx)?)?;
    let projection = project_gif(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::spec;
    use semio_repo_test_host::oracle_raster::{oracle_create_gif, project_gif};
    use semio_repo_test_host::{Context, Outcome};
    use semio_s_plugin_stdio::artifacts::gif::standards::v89a::subsets::any::io::{decode_gif, encode_gif};

    pub fn run(ctx: &Context) -> Result<Outcome, String> {
        let reference = oracle_create_gif(&spec(ctx)?)?;
        let snapshot = decode_gif(&reference)?;
        let bytes = encode_gif(&snapshot)?;
        let projection = project_gif(&bytes)?;
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
