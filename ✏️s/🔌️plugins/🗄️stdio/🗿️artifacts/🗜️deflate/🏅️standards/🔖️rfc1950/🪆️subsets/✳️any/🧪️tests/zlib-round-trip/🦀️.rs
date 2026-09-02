//! 🦀️ zlib round-trip case — Rust adapter.
//!
//! Deflate leaves the encoding free, so the normative property is the ROUND TRIP, not byte equality.
//! The oracle compresses with `flate2` and inflates with `flate2`; the subject compresses with this
//! repository's `zlib_compress` and inflates with the INDEPENDENT `flate2` inflater. Both project
//! the recovered payload, so a stream this repository emits that no conforming inflater can read is
//! a failure — which byte comparison would never have caught.

use semio_s_plugin_stdio_test_oracle::archive::{oracle_zlib_compress, project_zlib};
use semio_repo_test_host::{Adapter, Context, Outcome};

//#region 🔖️Input
const SCENARIOS: [&str; 3] = ["inflates-what-the-reference-deflated", "handles-an-empty-payload", "handles-incompressible-bytes"];

/// 🧫️ The scenario's payload, read from the feature so both producers get the same bytes.
fn payload(ctx: &Context) -> Vec<u8> {
    ctx.doc_string().unwrap_or("").as_bytes().to_vec()
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn oracle(ctx: &Context) -> Result<Outcome, String> {
    let bytes = oracle_zlib_compress(&payload(ctx))?;
    let projection = project_zlib(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::payload;
    use semio_s_plugin_stdio_test_oracle::archive::project_zlib;
    use semio_repo_test_host::{Context, Outcome};
    use semio_s_plugin_stdio::artifacts::deflate::standards::v_rfc1950::subsets::any::io::{zlib_compress, zlib_decompress};

    pub fn run(ctx: &Context) -> Result<Outcome, String> {
        let input = payload(ctx);
        let bytes = zlib_compress(&input)?;
        // 🔁️Our own inflater must recover the input too — a compressor that only its own reader can
        // read is not a conforming compressor.
        let recovered = zlib_decompress(&bytes)?;
        if recovered != input {
            return Err(format!("zlib_decompress(zlib_compress(input)) lost data: {} bytes in, {} bytes out", input.len(), recovered.len()));
        }
        let projection = project_zlib(&bytes)?;
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
