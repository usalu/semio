//! 🦀️ PNG creation and round-trip case — Rust adapter.
//!
//! The oracle writes the image with the `png` reference encoder; the subject decodes that same
//! artifact with this repository's `decode_png` and re-encodes it with `encode_png`. Both byte
//! streams are read back by the independent `png` decoder before the `semantic-raster-v1` profile
//! compares the decoded samples — a round trip that loses one pixel is a failure.

use semio_s_plugin_stdio_test_oracle::raster::{oracle_create_png, project_png, RasterSpec};
use semio_repo_test_host::{Adapter, Context, Outcome};

//#region 🔖️Input
const SCENARIOS: [&str; 3] = ["rgba-gradient-round-trips", "non-square-image-round-trips", "single-pixel-round-trips"];

/// 🧫️ The scenario's image description, read from the feature so both producers share one input.
fn spec(ctx: &Context) -> Result<RasterSpec, String> {
    Ok(RasterSpec::from_json(&ctx.doc_json()?))
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn oracle(ctx: &Context) -> Result<Outcome, String> {
    let bytes = oracle_create_png(&spec(ctx)?)?;
    let projection = project_png(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::spec;
    use semio_s_plugin_stdio_test_oracle::raster::{oracle_create_png, project_png};
    use semio_repo_test_host::{Context, Outcome};
    use semio_s_plugin_stdio::artifacts::png::standards::v1_2::subsets::any::io::{decode_png, encode_png};

    pub fn run(ctx: &Context) -> Result<Outcome, String> {
        let reference = oracle_create_png(&spec(ctx)?)?;
        let snapshot = decode_png(&reference)?;
        let bytes = encode_png(&snapshot)?;
        let projection = project_png(&bytes)?;
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
