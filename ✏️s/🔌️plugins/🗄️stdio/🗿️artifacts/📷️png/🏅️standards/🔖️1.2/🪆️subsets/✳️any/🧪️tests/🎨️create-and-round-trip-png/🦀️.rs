//! 🦀️ PNG creation and round-trip case — Rust adapter.
//!
//! The oracle writes the image with the `png` reference encoder; the subject decodes that same
//! artifact with this repository's `decode_png` and re-encodes it with `encode_png`. Both byte
//! streams are read back by the independent `png` decoder before the `semantic-raster-v1` profile
//! compares the decoded samples — a round trip that loses one pixel is a failure.

use semio_s_plugin_stdio_raster_test_oracle::{oracle_create_png, project_png, RasterSpec};
use semio_repo_test_host::{Adapter, Context, Outcome};

//#region 🔖️Input
const SCENARIOS: [&str; 8] = [
    "rgba-gradient-round-trips",
    "non-square-image-round-trips",
    "single-pixel-round-trips",
    "precision-16bit-gray-stays-exact",
    "indexed-2bit-duplicate-palette-stays-exact",
    "rgba8-multi-idat-private-stays-exact",
    "rgba8-adam7-stays-exact",
    "grayscale-1bit-stays-exact",
];

/// 🧫️ The scenario's image description, read from the feature so both producers share one input.
fn spec(ctx: &Context) -> Result<RasterSpec, String> {
    Ok(RasterSpec::from_json(&ctx.doc_json()?))
}

fn committed_source(ctx: &Context) -> Result<Option<&'static [u8]>, String> {
    Ok(match ctx.doc_json()?.str("fixture").as_str() {
        "precision-16bit-gray" => Some(include_bytes!("../../../../../../🧫️fixtures/🧬️canonical-source/precision-16bit-gray.png")),
        "indexed-2bit-duplicate-palette" => Some(include_bytes!("../../../../../../🧫️fixtures/🧬️canonical-source/indexed-2bit-duplicate-palette.png")),
        "rgba8-multi-idat-private" => Some(include_bytes!("../../../../../../🧫️fixtures/🧬️canonical-source/rgba8-multi-idat-private.png")),
        "rgba8-adam7" => Some(include_bytes!("../../../../../../🧫️fixtures/🧬️canonical-source/rgba8-adam7.png")),
        "grayscale-1bit" => Some(include_bytes!("../../../../../../🧫️fixtures/🧬️canonical-source/grayscale-1bit.png")),
        "" => None,
        fixture => return Err(format!("unknown PNG fixture {fixture:?}")),
    })
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn oracle(ctx: &Context) -> Result<Outcome, String> {
    if let Some(source) = committed_source(ctx)? {
        let projection = semio_s_artifact_stdio_png_test_oracle::standards::v1_2::subsets::any::project_png_mutation(source)?;
        return Ok(Outcome::with_raw(source.to_vec(), projection));
    }
    let bytes = oracle_create_png(&spec(ctx)?)?;
    let projection = project_png(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{committed_source, spec};
    use semio_s_plugin_stdio_raster_test_oracle::{oracle_create_png, project_png};
    use semio_repo_test_host::{Context, Outcome};
    use semio_s_artifact_stdio_png::standards::v1_2::subsets::any::io::{decode_png, encode_png};

    pub fn run(ctx: &Context) -> Result<Outcome, String> {
        if let Some(source) = committed_source(ctx)? {
            let snapshot = decode_png(source)?;
            let bytes = encode_png(&snapshot)?;
            let projection = semio_s_artifact_stdio_png_test_oracle::standards::v1_2::subsets::any::project_png_mutation(&bytes)?;
            return Ok(Outcome::with_raw(bytes, projection));
        }
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
