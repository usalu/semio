//! 🦀️ BMP creation and round-trip case — Rust adapter.
//!
//! The oracle writes the image with the registered `image` reference encoder; the subject decodes
//! that same artifact with this repository's `decode_bmp` and re-encodes it with `encode_bmp`.
//! Both byte streams are read back by the independent decoder before the profile compares them. The
//! subject half is gated behind the generated host's `sut` feature, so the oracle-only run never
//! compiles the local implementation.
//!
//! That gate is also why this file's subject import spelled `standards::v3` for as long as it did:
//! the module is `standards::v_v3` (the generated barrel prefixes a standard id that starts with a
//! digit-adjacent token), the oracle-only run never compiled the `sut` half, and no parity run had
//! ever been made for this case — so the path was wrong and green at the same time. It was fixed
//! when `parity exhaustive --owner 🗄️stdio --case create-and-round-trip-bmp` was run for the first
//! time under ticket 26/08/23/END-TO-END-TESTING-REFACTOR.

use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_plugin_stdio_test_oracle::raster::{oracle_create_image, project_image, RasterSpec};

//#region 🔖️Input
const FORMAT: &str = "bmp";
const SCENARIOS: [&str; 2] = ["gradient-round-trips", "non-square-round-trips"];

/// 🧫️ The scenario's image description, read from the feature so both producers share one input.
fn spec(ctx: &Context) -> Result<RasterSpec, String> {
    Ok(RasterSpec::from_json(&ctx.doc_json()?))
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn oracle(ctx: &Context) -> Result<Outcome, String> {
    let bytes = oracle_create_image(&spec(ctx)?, FORMAT)?;
    let projection = project_image(&bytes, FORMAT)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{spec, FORMAT};
    use semio_repo_test_host::{Context, Outcome};
    use semio_s_plugin_stdio_test_oracle::raster::{oracle_create_image, project_image};
    use semio_s_plugin_stdio::artifacts::bmp::standards::v_v3::subsets::any::io::{decode_bmp, encode_bmp};

    pub fn run(ctx: &Context) -> Result<Outcome, String> {
        let reference = oracle_create_image(&spec(ctx)?, FORMAT)?;
        let snapshot = decode_bmp(&reference).map_err(|error| format!("decode_bmp failed: {:?}", error))?;
        let bytes = encode_bmp(&snapshot).map_err(|error| format!("encode_bmp failed: {:?}", error))?;
        let projection = project_image(&bytes, FORMAT)?;
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
