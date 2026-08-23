//! 🦀️ WAV RIFF-PCM exhaustive mutation case — Rust adapter.
//!
//! Every scenario copies the immutable real recording into the case work directory first; the
//! committed fixture is never written to. `oracle` drives the registered `hound` reference
//! implementation (this subset's own `🧪️oracle/🦀️component.rs`), `subject` drives this repository's
//! own decode → mutate → encode round trip, and both results are read back by the SAME independent
//! `hound`-based projector before the `semantic-audio-v1` profile compares them. The subject half is
//! gated behind the generated host's `sut` feature so the oracle-only run never compiles the local
//! implementation.

use semio_repo_test_host::{Adapter, Context, Outcome};
use semio_s_plugin_stdio_test_oracle::artifacts::wav::standards::v_riff_pcm::subsets::any::{oracle_apply_mutation, project_wav_mutation};

//#region 🔖️Input
const INPUT: &str = "shared://🔊️bauen-mit-bestand-ausschnitt.wav";

/// 🦠️ The catalog's own kinds (`../../🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧪️oracle/🔣️component.json`),
/// duplicated as a plain constant rather than reached through the subject crate — this loop drives
/// oracle registration too, which must build and run with the subject crate absent entirely.
const KINDS: &[&str] = &["no-mutation", "set-snapshot", "set-fmt", "set-data", "set-other-chunks"];

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.wav"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
/// 🔮️ Applies the declared mutation with `hound` and projects the result independently.
fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    let spec = ctx.doc_json()?;
    let bytes = oracle_apply_mutation(&mutable_input(ctx)?, &spec)?;
    let projection = project_wav_mutation(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔮️ The inverse property's reference claim: after mutate-then-undo the recording is unchanged, so
/// the trusted result is simply the pristine original projected through the SAME independent reader.
fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    let bytes = mutable_input(ctx)?;
    let projection = project_wav_mutation(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

/// 🔮️ The round-trip's reference claim: the pristine original, projected independently.
fn round_trip_oracle(ctx: &Context) -> Result<Outcome, String> {
    let bytes = mutable_input(ctx)?;
    let projection = project_wav_mutation(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::{Context, Json, Outcome};
    use semio_s_plugin_stdio::artifacts::wav::standards::riff_pcm::subsets::any::io::{decode_wav, encode_wav};
    use semio_s_plugin_stdio::artifacts::wav::standards::riff_pcm::subsets::any::schema::mutations::{apply_wav_mutation, WavMutation};
    use semio_s_plugin_stdio::artifacts::wav::standards::riff_pcm::subsets::any::schema::snapshot::{RiffChunk, WavData, WavFmt, WavSnapshot};
    use semio_s_plugin_stdio_test_oracle::artifacts::wav::standards::v_riff_pcm::subsets::any::project_wav_mutation;

    //#region 🔖️SpecReading
    /// 🔎️ A second, independently written reading of the SAME `params` JSON schema the oracle reads
    /// in `../../../🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🧪️oracle/🦀️component.rs` — deliberately not
    /// shared code, so a bug in one reading has nothing to hide behind in the other.
    fn number(value: &Json, key: &str, fallback: f64) -> f64 {
        match value.get(key) {
            Some(Json::Number(found)) => *found,
            _ => fallback,
        }
    }

    fn wav_fmt_from_json(value: &Json) -> WavFmt {
        let channels = number(value, "channels", 1.0) as u16;
        let sample_rate = number(value, "sampleRate", 44_100.0) as u32;
        let bits_per_sample = number(value, "bitsPerSample", 16.0) as u16;
        let audio_format = number(value, "audioFormat", 1.0) as u16;
        let block_align = channels * (bits_per_sample / 8);
        let byte_rate = sample_rate * block_align as u32;
        WavFmt { audio_format, channels, sample_rate, byte_rate, block_align, bits_per_sample, ext: None }
    }

    fn wav_data_from_json(value: &Json) -> WavData {
        let samples = match value.get("samples") {
            Some(Json::Array(items)) => items.iter().filter_map(|item| if let Json::Number(n) = item { Some(*n as i16) } else { None }).collect(),
            _ => Vec::new(),
        };
        WavData::Pcm16(samples)
    }

    fn riff_chunks_from_json(value: &Json, key: &str) -> Vec<RiffChunk> {
        value
            .array(key)
            .into_iter()
            .map(|entry| {
                let data = match entry.get("data") {
                    Some(Json::Array(items)) => items.iter().filter_map(|item| if let Json::Number(n) = item { Some(*n as u8) } else { None }).collect(),
                    _ => Vec::new(),
                };
                RiffChunk { fourcc: entry.str("fourcc"), data }
            })
            .collect()
    }

    /// 🦠️ Builds the real `WavMutation` this scenario's `{"kind", "params"}` doc string describes.
    fn mutation_from_spec(spec: &Json, original: &WavSnapshot) -> Result<WavMutation, String> {
        let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
        match spec.str("kind").as_str() {
            "no-mutation" => Ok(WavMutation::NoMutation),
            "set-fmt" => Ok(WavMutation::SetFmt { fmt: wav_fmt_from_json(&params.get("fmt").cloned().unwrap_or(Json::Object(Vec::new()))) }),
            "set-data" => Ok(WavMutation::SetData { data: wav_data_from_json(&params.get("data").cloned().unwrap_or(Json::Object(Vec::new()))) }),
            "set-other-chunks" => Ok(WavMutation::SetOtherChunks { chunks: riff_chunks_from_json(&params, "chunks") }),
            "set-snapshot" => Ok(WavMutation::SetSnapshot {
                snapshot: WavSnapshot {
                    schema: original.schema.clone(),
                    fmt: wav_fmt_from_json(&params.get("fmt").cloned().unwrap_or(Json::Object(Vec::new()))),
                    data: wav_data_from_json(&params.get("data").cloned().unwrap_or(Json::Object(Vec::new()))),
                    other_chunks: riff_chunks_from_json(&params, "otherChunks"),
                },
            }),
            other => Err(format!("test case does not know mutation kind {other:?}")),
        }
    }

    /// ↩️ The inverse of one applied mutation, restoring `original`'s own field — mirrors
    /// `WavMutation::inverse`'s own per-variant mapping (see the mutation vocabulary's own
    /// `inverse_law_mutation_and_diff_level` unit test for that law at the type level; this is the
    /// same law exercised against a real decoded recording instead of a synthetic snapshot).
    fn restore_mutation(applied: &WavMutation, original: &WavSnapshot) -> WavMutation {
        match applied {
            WavMutation::NoMutation => WavMutation::NoMutation,
            WavMutation::SetSnapshot { .. } => WavMutation::SetSnapshot { snapshot: original.clone() },
            WavMutation::SetFmt { .. } => WavMutation::SetFmt { fmt: original.fmt.clone() },
            WavMutation::SetData { .. } => WavMutation::SetData { data: original.data.clone() },
            WavMutation::SetOtherChunks { .. } => WavMutation::SetOtherChunks { chunks: original.other_chunks.clone() },
        }
    }
    //#endregion 🔖️SpecReading

    //#region 🔖️Scenarios
    /// 🎯️ Decode → apply the declared mutation → re-encode, projected through the SAME independent
    /// reader the oracle used.
    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let mut snapshot = decode_wav(&input).map_err(|error| format!("decode_wav failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?, &snapshot)?;
        apply_wav_mutation(&mut snapshot, &mutation);
        let bytes = encode_wav(&snapshot);
        let projection = project_wav_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🎯️ Decode → apply the declared mutation → apply its inverse → re-encode. The result must
    /// project back onto the pristine original — the inverse oracle's own reference claim.
    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let original = decode_wav(&input).map_err(|error| format!("decode_wav failed: {error}"))?;
        let mutation = mutation_from_spec(&ctx.doc_json()?, &original)?;
        let mut snapshot = original.clone();
        apply_wav_mutation(&mut snapshot, &mutation);
        apply_wav_mutation(&mut snapshot, &restore_mutation(&mutation, &original));
        let bytes = encode_wav(&snapshot);
        let projection = project_wav_mutation(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    /// 🎯️ Full parse into the typed snapshot, then re-serialize from the model ALONE — the tripwire
    /// this whole wave exists to enforce: our writer cannot reproduce another writer's byte layout,
    /// so bit-identical output would mean the input was smuggled through rather than parsed.
    pub fn identity_round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = mutable_input(ctx)?;
        let snapshot = decode_wav(&input).map_err(|error| format!("decode_wav failed: {error}"))?;
        let output = encode_wav(&snapshot);
        if output == input {
            return Err("byte pass-through: output is bit-identical to the input".to_string());
        }
        let projection = project_wav_mutation(&output)?;
        Ok(Outcome::with_raw(output, projection))
    }
    //#endregion 🔖️Scenarios
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls. Registers by FULL expanded scenario id
/// (`mutate-<kind>` / `inverse-<kind>`), never the outline's base id — a missing registration is a
/// hard error, never a skip.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for kind in KINDS {
        built = built.oracle(&format!("mutate-{kind}"), mutate_oracle).oracle(&format!("inverse-{kind}"), inverse_oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(&format!("mutate-{kind}"), subject::mutate).subject(&format!("inverse-{kind}"), subject::inverse);
        }
    }
    built = built.oracle("identity-round-trip", round_trip_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::identity_round_trip);
    }
    built
}
//#endregion 🔖️Registration
