//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `audio` module rather than by copying it.
//!
//! `hound` genuinely decodes/encodes the `fmt `/`data` pair — the interpretive part a PCM library
//! exists for. It has no concept of any OTHER top-level RIFF chunk, so this module hand-splices
//! those: RIFF chunk framing is a fixed 8-byte fourcc+size wire header the format itself defines,
//! not an interpretation this oracle is making on hound's behalf (the same allowance already on
//! record for OBJ's "no reference writer" oracle).
//!
//! @see ../🧪️oracle/🔣️component.json — the mutation catalog this module is measured against.
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

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
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
    /// 📐️ The `fmt ` fields this oracle round-trips through `hound::WavSpec` — scoped to plain
    /// 16-bit PCM, the only form the catalog's own scenarios exercise (`ext`/non-PCM formats stay
    /// the subject-only `Raw` boundary the codec already documents).
    struct FmtSpec {
        channels: u16,
        sample_rate: u32,
    }

    fn fmt_spec_of(value: &Json) -> FmtSpec {
        FmtSpec { channels: number(value, "channels", 1.0) as u16, sample_rate: number(value, "sampleRate", 44_100.0) as u32 }
    }

    fn hound_spec(fmt: &FmtSpec) -> ::hound::WavSpec {
        ::hound::WavSpec { channels: fmt.channels, sample_rate: fmt.sample_rate, bits_per_sample: 16, sample_format: ::hound::SampleFormat::Int }
    }
    //#endregion 🔖️FmtSpec

    //#region 🔖️ReadWrite
    /// 🔮️ Decodes the `fmt `/`data` pair with `hound` — the interpretive core it exists for.
    fn read_fmt_and_samples(input: &[u8]) -> Result<(::hound::WavSpec, Vec<i16>), String> {
        let mut reader = ::hound::WavReader::new(std::io::Cursor::new(input.to_vec())).map_err(|error| format!("hound could not parse the WAVE: {error}"))?;
        let spec = reader.spec();
        let samples = reader.samples::<i16>().collect::<Result<Vec<i16>, _>>().map_err(|error| format!("hound could not decode WAVE samples: {error}"))?;
        Ok((spec, samples))
    }

    /// 🔮️ Encodes a fresh `fmt `/`data` pair with `hound`.
    fn write_fmt_and_samples(spec: ::hound::WavSpec, samples: &[i16]) -> Result<Vec<u8>, String> {
        let mut cursor = std::io::Cursor::new(Vec::new());
        {
            let mut writer = ::hound::WavWriter::new(&mut cursor, spec).map_err(|error| format!("hound wav header: {error}"))?;
            for sample in samples {
                writer.write_sample(*sample).map_err(|error| format!("hound wav sample: {error}"))?;
            }
            writer.finalize().map_err(|error| format!("hound wav finalize: {error}"))?;
        }
        Ok(cursor.into_inner())
    }

    /// 🧩️ Walks the top-level RIFF chunks hound has no concept of, collecting everything that is
    /// neither `fmt ` nor `data`, in on-disk order — chunk *framing*, not interpretation (see the
    /// module doc comment).
    fn scan_other_chunks(input: &[u8]) -> Result<Vec<(String, Vec<u8>)>, String> {
        if input.len() < 12 || &input[0..4] != b"RIFF" || &input[8..12] != b"WAVE" {
            return Err("wav: missing RIFF/WAVE magic".to_string());
        }
        let mut pos = 12usize;
        let mut other = Vec::new();
        while pos + 8 <= input.len() {
            let fourcc = &input[pos..pos + 4];
            let size = u32::from_le_bytes(input[pos + 4..pos + 8].try_into().map_err(|_| "wav: bad chunk size".to_string())?) as usize;
            let body_start = pos + 8;
            let body_end = body_start + size;
            if body_end > input.len() {
                return Err(format!("wav: chunk {:?} overruns file", String::from_utf8_lossy(fourcc)));
            }
            if fourcc != b"fmt " && fourcc != b"data" {
                other.push((String::from_utf8_lossy(fourcc).into_owned(), input[body_start..body_end].to_vec()));
            }
            pos = body_end + (size % 2);
        }
        Ok(other)
    }

    /// 🧩️ Appends RIFF chunks after whatever `hound` wrote and patches the outer `RIFF` size field
    /// — the mechanical inverse of `scan_other_chunks`, equally fixed by the format's own wire shape.
    fn append_other_chunks(mut wav_bytes: Vec<u8>, chunks: &[(String, Vec<u8>)]) -> Vec<u8> {
        for (fourcc, data) in chunks {
            let mut fcc = fourcc.clone().into_bytes();
            fcc.resize(4, b' ');
            wav_bytes.extend_from_slice(&fcc[0..4]);
            wav_bytes.extend_from_slice(&(data.len() as u32).to_le_bytes());
            wav_bytes.extend_from_slice(data);
            if data.len() % 2 == 1 {
                wav_bytes.push(0);
            }
        }
        let riff_len = (wav_bytes.len() - 8) as u32;
        wav_bytes[4..8].copy_from_slice(&riff_len.to_le_bytes());
        wav_bytes
    }
    //#endregion 🔖️ReadWrite

    //#region 🔖️Mutate
    /// 🎚️ `SetFmt` — replaces the format block wholesale; `data`/`other_chunks` are untouched.
    pub fn mutate_set_fmt(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let (_, old_samples) = read_fmt_and_samples(input)?;
        let other = scan_other_chunks(input)?;
        let fmt = fmt_spec_of(&params.get("fmt").cloned().unwrap_or(Json::Object(Vec::new())));
        let bytes = write_fmt_and_samples(hound_spec(&fmt), &old_samples)?;
        Ok(append_other_chunks(bytes, &other))
    }

    /// 🔊️ `SetData` — replaces the sample data wholesale; `fmt`/`other_chunks` are untouched.
    pub fn mutate_set_data(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let (old_spec, _) = read_fmt_and_samples(input)?;
        let other = scan_other_chunks(input)?;
        let new_samples = samples(&params.get("data").cloned().unwrap_or(Json::Object(Vec::new())), "samples");
        let bytes = write_fmt_and_samples(old_spec, &new_samples)?;
        Ok(append_other_chunks(bytes, &other))
    }

    /// 📎️ `SetOtherChunks` — replaces the verbatim chunk list wholesale; `fmt`/`data` are untouched.
    pub fn mutate_set_other_chunks(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let (old_spec, old_samples) = read_fmt_and_samples(input)?;
        let bytes = write_fmt_and_samples(old_spec, &old_samples)?;
        Ok(append_other_chunks(bytes, &chunk_list(params, "chunks")))
    }

    /// 🔁️ `SetSnapshot` — full replace: `fmt`, `data` and `other_chunks` all come from `params`.
    pub fn mutate_set_snapshot(_input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let fmt = fmt_spec_of(&params.get("fmt").cloned().unwrap_or(Json::Object(Vec::new())));
        let new_samples = samples(&params.get("data").cloned().unwrap_or(Json::Object(Vec::new())), "samples");
        let bytes = write_fmt_and_samples(hound_spec(&fmt), &new_samples)?;
        Ok(append_other_chunks(bytes, &chunk_list(params, "otherChunks")))
    }
    //#endregion 🔖️Mutate

    //#region 🔖️Project
    /// 👁️ The INDEPENDENT projection: format block (re-derived, not trusted from the header),
    /// every decoded sample, and every retained chunk.
    pub fn project(input: &[u8]) -> Result<Json, String> {
        let (spec, samples) = read_fmt_and_samples(input)?;
        let other = scan_other_chunks(input)?;
        let block_align = (spec.channels as u32) * (spec.bits_per_sample as u32 / 8);
        let byte_rate = spec.sample_rate * block_align;
        Ok(Json::Object(vec![
            ("format".to_string(), Json::String("wav".to_string())),
            ("audioFormat".to_string(), Json::Number(1.0)),
            ("channels".to_string(), Json::Number(spec.channels as f64)),
            ("sampleRate".to_string(), Json::Number(spec.sample_rate as f64)),
            ("bitsPerSample".to_string(), Json::Number(spec.bits_per_sample as f64)),
            ("byteRate".to_string(), Json::Number(byte_rate as f64)),
            ("blockAlign".to_string(), Json::Number(block_align as f64)),
            ("sampleCount".to_string(), Json::Number(samples.len() as f64)),
            ("samples".to_string(), Json::Array(samples.iter().map(|sample| Json::Number(*sample as f64)).collect())),
            ("otherChunkCount".to_string(), Json::Number(other.len() as f64)),
            (
                "otherChunks".to_string(),
                Json::Array(other.into_iter().map(|(fourcc, data)| Json::Object(vec![("fourcc".to_string(), Json::String(fourcc)), ("data".to_string(), Json::Array(data.iter().map(|byte| Json::Number(*byte as f64)).collect()))])).collect()),
            ),
        ]))
    }
    //#endregion 🔖️Project
}
//#endregion 🔖️Reference
