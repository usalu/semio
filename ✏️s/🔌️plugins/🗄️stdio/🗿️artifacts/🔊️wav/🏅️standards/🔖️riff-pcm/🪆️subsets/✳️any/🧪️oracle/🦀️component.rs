//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `audio` module rather than by copying it.
//!
//! The shared owned audio oracle decodes and encodes the `fmt `/`data` pair independently from the
//! subject codec while this subset supplies its mutation vocabulary. Auxiliary RIFF chunks remain
//! opaque ordered bytes at that boundary.
//!
//! @see ../🧪️oracle/🔣️.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️component.rs — the mutation vocabulary itself (`KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
    match spec.str("kind").as_str() {
        "no-mutation" => Ok(input.to_vec()),
        "set-fmt" => reference::mutate_set_fmt(input, &params),
        "set-data" => reference::mutate_set_data(input, &params),
        "set-other-chunks" => reference::mutate_set_other_chunks(input, &params),
        "set-snapshot" => reference::mutate_set_snapshot(input, &params),
        "" => Err("mutation spec carries no `kind`".to_string()),
        kind => Err(format!("mutation kind {:?} has no oracle implementation ({} input byte(s))", kind, input.len())),
    }
}

/// 👁️ Projects real RIFF/WAVE bytes onto the shape every producer is compared through: the decoded
/// format block, the decoded samples, and every other retained chunk. PCM is lossless, so exact
/// sample values are the legitimate comparison — no bucket/histogram approximation, unlike the
/// lossy raster oracles in `🧪️oracle/🖼️raster/🦀️component.rs`.
#[cfg(feature = "oracles")]
pub fn project_wav_mutation(input: &[u8]) -> Result<Json, String> {
    reference::project(input)
}

/// ↩️ Applies the INDEPENDENTLY computed inverse of `spec` on top of `mutated`, so that
/// `inverse(m) . m` must be the identity on the semantic projection. Every `WavMutation` variant's
/// inverse is "restore `base`'s own value for the facet this kind replaced"
/// (`../🧬️schema/🧬️mutations/🦀️component.rs`), reimplemented here over the independent owned
/// PCM model, never by calling that trait.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation_inverse(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    reference::apply_inverse(original_input, &kind, mutated)
}

/// 🔁️ The `@id-identity-round-trip` scenario's own independent computation: decode the `fmt `/`data`
/// pair, retain opaque chunks, and write a fresh file from that model alone.
/// Deliberately NOT `oracle_apply_mutation`'s `no-mutation` arm, which is a verbatim echo of the
/// input bytes (the correct reference answer for "apply nothing", and no evidence of a parse).
#[cfg(feature = "oracles")]
pub fn oracle_identity_round_trip(input: &[u8]) -> Result<Vec<u8>, String> {
    reference::rewrite(input)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation_inverse(_original_input: &[u8], _spec: &Json, _mutated: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn oracle_identity_round_trip(_input: &[u8]) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_wav_mutation(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️Reference
#[cfg(feature = "oracles")]
mod reference {
    use crate::audio::{decode_pcm16_wav, encode_pcm16_wav, PcmWav, PcmWavFormat};
    use semio_repo_test_host::Json;

    //#region 🔖️JsonReading
    /// 🔎️ A `u16`/`u32` field, or `fallback` for anything else.
    fn number(value: &Json, key: &str, fallback: f64) -> f64 {
        match value.get(key) {
            Some(Json::Number(found)) => *found,
            _ => fallback,
        }
    }

    /// 🔎️ Every byte of a JSON number array, saturating out-of-range values.
    fn bytes(value: &Json, key: &str) -> Vec<u8> {
        match value.get(key) {
            Some(Json::Array(items)) => items.iter().filter_map(|item| if let Json::Number(n) = item { Some(*n as u8) } else { None }).collect(),
            _ => Vec::new(),
        }
    }

    /// 🔎️ Every `i16` of a JSON number array — the sample vocabulary this oracle writes.
    fn samples(value: &Json, key: &str) -> Vec<i16> {
        match value.get(key) {
            Some(Json::Array(items)) => items.iter().filter_map(|item| if let Json::Number(n) = item { Some(*n as i16) } else { None }).collect(),
            _ => Vec::new(),
        }
    }

    /// 🔎️ `{"fourcc": "...", "data": [byte, ...]}` entries — the `other_chunks` vocabulary.
    fn chunk_list(value: &Json, key: &str) -> Vec<(String, Vec<u8>)> {
        value.array(key).into_iter().map(|entry| (entry.str("fourcc"), bytes(&entry, "data"))).collect()
    }
    //#endregion 🔖️JsonReading

    //#region 🔖️FmtSpec
    /// 📐️ The plain PCM16 format fields exercised by this subset's catalog.
    fn fmt_spec_of(value: &Json) -> PcmWavFormat {
        PcmWavFormat { channels: number(value, "channels", 1.0) as u16, sample_rate: number(value, "sampleRate", 44_100.0) as u32, bits_per_sample: 16 }
    }
    //#endregion 🔖️FmtSpec

    //#region 🔖️ReadWrite
    /// 📥️ Decodes one file through the shared independent owned boundary.
    fn read(input: &[u8]) -> Result<PcmWav, String> {
        decode_pcm16_wav(input)
    }

    /// 📤️ Encodes one semantic model through the shared independent owned boundary.
    fn write(wav: &PcmWav) -> Result<Vec<u8>, String> {
        encode_pcm16_wav(wav)
    }
    //#endregion 🔖️ReadWrite

    //#region 🔖️Mutate
    /// 🎚️ `SetFmt` — replaces the format block wholesale; `data`/`other_chunks` are untouched.
    pub fn mutate_set_fmt(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut wav = read(input)?;
        wav.format = fmt_spec_of(&params.get("fmt").cloned().unwrap_or(Json::Object(Vec::new())));
        write(&wav)
    }

    /// 🔊️ `SetData` — replaces the sample data wholesale; `fmt`/`other_chunks` are untouched.
    pub fn mutate_set_data(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut wav = read(input)?;
        wav.samples = samples(&params.get("data").cloned().unwrap_or(Json::Object(Vec::new())), "samples");
        write(&wav)
    }

    /// 📎️ `SetOtherChunks` — replaces the verbatim chunk list wholesale; `fmt`/`data` are untouched.
    pub fn mutate_set_other_chunks(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut wav = read(input)?;
        wav.other_chunks = chunk_list(params, "chunks");
        write(&wav)
    }

    /// 🔁️ `SetSnapshot` — full replace: `fmt`, `data` and `other_chunks` all come from `params`.
    pub fn mutate_set_snapshot(_input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        write(&PcmWav {
            format: fmt_spec_of(&params.get("fmt").cloned().unwrap_or(Json::Object(Vec::new()))),
            samples: samples(&params.get("data").cloned().unwrap_or(Json::Object(Vec::new())), "samples"),
            other_chunks: chunk_list(params, "otherChunks"),
        })
    }
    //#endregion 🔖️Mutate

    //#region 🔖️Inverse
    /// 🔁️ Decodes and rewrites a fresh file from the owned model alone.
    pub fn rewrite(input: &[u8]) -> Result<Vec<u8>, String> {
        write(&read(input)?)
    }

    /// ↩️ The real inverse of `kind`, computed from the PRE-mutation recording and applied on top
    /// of `mutated`: each variant restores exactly the one facet it replaced, leaving the other two
    /// as the forward mutation left them.
    pub fn apply_inverse(original_input: &[u8], kind: &str, mutated: &[u8]) -> Result<Vec<u8>, String> {
        let original = read(original_input)?;
        if kind == "set-snapshot" {
            return write(&original);
        }
        let mut restored = read(mutated)?;
        match kind {
            "no-mutation" => {}
            "set-fmt" => restored.format = original.format,
            "set-data" => restored.samples = original.samples,
            "set-other-chunks" => restored.other_chunks = original.other_chunks,
            other => return Err(format!("mutation kind {other:?} has no oracle inverse ({} mutated byte(s))", mutated.len())),
        }
        write(&restored)
    }
    //#endregion 🔖️Inverse

    //#region 🔖️Project
    /// 👁️ The INDEPENDENT projection: format block (re-derived, not trusted from the header),
    /// every decoded sample, and every retained chunk.
    pub fn project(input: &[u8]) -> Result<Json, String> {
        let wav = read(input)?;
        let block_align = (wav.format.channels as u32) * (wav.format.bits_per_sample as u32 / 8);
        let byte_rate = wav.format.sample_rate * block_align;
        Ok(Json::Object(vec![
            ("format".to_string(), Json::String("wav".to_string())),
            ("audioFormat".to_string(), Json::Number(1.0)),
            ("channels".to_string(), Json::Number(wav.format.channels as f64)),
            ("sampleRate".to_string(), Json::Number(wav.format.sample_rate as f64)),
            ("bitsPerSample".to_string(), Json::Number(wav.format.bits_per_sample as f64)),
            ("byteRate".to_string(), Json::Number(byte_rate as f64)),
            ("blockAlign".to_string(), Json::Number(block_align as f64)),
            ("sampleCount".to_string(), Json::Number(wav.samples.len() as f64)),
            ("samples".to_string(), Json::Array(wav.samples.iter().map(|sample| Json::Number(*sample as f64)).collect())),
            ("otherChunkCount".to_string(), Json::Number(wav.other_chunks.len() as f64)),
            (
                "otherChunks".to_string(),
                Json::Array(
                    wav.other_chunks.into_iter().map(|(fourcc, data)| Json::Object(vec![("fourcc".to_string(), Json::String(fourcc)), ("data".to_string(), Json::Array(data.iter().map(|byte| Json::Number(*byte as f64)).collect()))])).collect(),
                ),
            ),
        ]))
    }
    //#endregion 🔖️Project
}
//#endregion 🔖️Reference
