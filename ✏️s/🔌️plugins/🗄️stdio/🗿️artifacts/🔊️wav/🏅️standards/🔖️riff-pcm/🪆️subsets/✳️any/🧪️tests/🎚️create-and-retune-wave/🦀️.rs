//! 🦀️ WAVE creation and retuning case — Rust adapter.
//!
//! The owned oracle writes the waveform independently; the subject DECODES that same artifact with this
//! repository's `decode_wav` and re-encodes it with `encode_wav`. Both byte streams are then read
//! back by the independent owned reader, so a lossy decode or a malformed re-encode surfaces as a
//! real difference instead of a producer agreeing with its own reading.

use semio_s_plugin_stdio_test_oracle::audio::{oracle_create_wav, oracle_retune_wav, project_wav, AudioSpec};
use semio_repo_test_host::{Adapter, Context, Outcome};

//#region 🔖️Input
const ROUND_TRIP: [&str; 2] = ["mono-sawtooth-round-trips", "stereo-round-trips"];
const RETUNE: &str = "retuned-sample-rate";

/// 🧫️ The scenario's waveform description, read from the feature so both producers share one input.
fn spec(ctx: &Context) -> Result<AudioSpec, String> {
    Ok(AudioSpec::from_json(&ctx.doc_json()?))
}

/// 🎚️ The retune target the scenario declares.
fn retune_to(ctx: &Context) -> Result<u32, String> {
    match ctx.doc_json()?.get("retuneTo") {
        Some(semio_repo_test_host::Json::Number(value)) => Ok(*value as u32),
        _ => Err("scenario declares no retuneTo sample rate".to_string()),
    }
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn oracle_round_trip(ctx: &Context) -> Result<Outcome, String> {
    let bytes = oracle_create_wav(&spec(ctx)?)?;
    let projection = project_wav(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

fn oracle_retune(ctx: &Context) -> Result<Outcome, String> {
    let bytes = oracle_retune_wav(&oracle_create_wav(&spec(ctx)?)?, retune_to(ctx)?)?;
    let projection = project_wav(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{retune_to, spec};
    use semio_s_plugin_stdio_test_oracle::audio::{oracle_create_wav, project_wav};
    use semio_repo_test_host::{Context, Outcome};
    use semio_s_plugin_stdio::artifacts::wav::standards::riff_pcm::subsets::any::io::{decode_wav, encode_wav};

    /// 🔁️ Decode the reference artifact with our reader, re-encode with our writer, project both.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let reference = oracle_create_wav(&spec(ctx)?)?;
        let snapshot = decode_wav(&reference)?;
        let bytes = encode_wav(&snapshot);
        let projection = project_wav(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🎚️ Change only the declared sample rate; every sample must survive untouched.
    pub fn retune(ctx: &Context) -> Result<Outcome, String> {
        let reference = oracle_create_wav(&spec(ctx)?)?;
        let mut snapshot = decode_wav(&reference)?;
        let rate = retune_to(ctx)?;
        snapshot.fmt.sample_rate = rate;
        snapshot.fmt.byte_rate = rate * u32::from(snapshot.fmt.block_align);
        let bytes = encode_wav(&snapshot);
        let projection = project_wav(&bytes)?;
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
    built = built.oracle(RETUNE, oracle_retune);
    #[cfg(feature = "sut")]
    {
        built = built.subject(RETUNE, subject::retune);
    }
    built
}
//#endregion 🔖️Registration
