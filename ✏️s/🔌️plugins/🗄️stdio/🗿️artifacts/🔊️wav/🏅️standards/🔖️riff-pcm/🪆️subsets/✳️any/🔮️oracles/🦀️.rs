//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered reference implementation so the subject's own mutation has an independent result to
//! be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared `audio` module rather than by copying it.
//!
//! The RIFF container is read and written entirely through the third-party `riff` crate (`riff::Chunk`
//! walks the top-level chunks, `riff::ChunkContents::write` frames and pads them), composed with this
//! module's own reading of the 16-byte PCM `fmt ` layout and the little-endian PCM16 `data` words —
//! the same composition the AVI oracle uses, and nothing shared with the subject codec. Auxiliary
//! RIFF chunks stay opaque ordered bytes.
//!
//! @see ../🔣️oracle.json — the mutation catalog this module is measured against.
//! @see ../🧬️schema/🧬️mutations/🦀️.rs — the mutation vocabulary itself (`KINDS`).

use semio_repo_test_host::Json;

//#region 🔖️Dispatch
/// 🦠️ Applies one declared mutation kind to a real artifact and returns the re-serialized bytes.
/// `params` is the leaf's wire payload (`payload_value()`), read by the same field names the schema declares.
/// An unrecognised kind is an error, never a silent no-op: a mutation that is quietly skipped
/// reports as a passing test.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation(input: &[u8], spec: &Json) -> Result<Vec<u8>, String> {
    let params = spec.get("params").cloned().unwrap_or(Json::Object(Vec::new()));
    match spec.str("kind").as_str() {
        "set-fmt" => reference::mutate_set_fmt(input, &params),
        "set-data" => reference::mutate_set_data(input, &params),
        "patch-data" => reference::mutate_patch_data(input, &params),
        "set-other-chunks" => reference::mutate_set_other_chunks(input, &params),
        "set-snapshot" => reference::mutate_set_snapshot(input, &params),
        "patch-snapshot" => reference::mutate_patch_snapshot(input, &params),
        "" => Err("mutation spec carries no `kind`".to_string()),
        kind => Err(format!("mutation kind {:?} has no oracle implementation ({} input byte(s))", kind, input.len())),
    }
}

/// 👁️ Projects real RIFF/WAVE bytes onto the shape every producer is compared through: the decoded
/// format block, the decoded samples, and every other retained chunk. PCM is lossless, so exact
/// sample values are the legitimate comparison — no bucket/histogram approximation, unlike the
/// lossy raster oracles in `🔮️oracles/🖼️raster/🦀️.rs`.
#[cfg(feature = "oracles")]
pub fn project_wav_mutation(input: &[u8]) -> Result<Json, String> {
    reference::project(input)
}

/// ↩️ Applies the INDEPENDENTLY computed inverse of `spec` on top of `mutated`, so that
/// `inverse(m) . m` must be the identity on the semantic projection. Every `WavMutation` variant's
/// inverse is "restore `base`'s own value for the facet this kind replaced"
/// (`../🧬️schema/🧬️mutations/🦀️.rs`), reimplemented here over the independent owned
/// PCM model, never by calling that trait.
#[cfg(feature = "oracles")]
pub fn oracle_apply_mutation_inverse(original_input: &[u8], spec: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
    let kind = spec.str("kind");
    if kind.is_empty() {
        return Err("mutation spec carries no `kind`".to_string());
    }
    reference::apply_inverse(original_input, &kind, &spec.get("params").cloned().unwrap_or(Json::Object(Vec::new())), mutated)
}

/// 🔁️ The `@id-identity-round-trip` scenario's own independent computation: decode the `fmt `/`data`
/// pair, retain opaque chunks, and write a fresh file from that model alone.
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
    use semio_s_plugin_stdio_audio_test_oracle::{PcmWav, PcmWavFormat};
    use riff::{Chunk, ChunkContents, ChunkId, RIFF_ID};
    use semio_repo_test_host::Json;
    use std::io::Cursor;

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

    /// 🔎️ The `i16` samples of a `WavData` wire value — `{"kind": "pcm16", "value": [...]}`, the only sample
    /// vocabulary this owned PCM16 oracle reads and writes; any other kind is refused rather than guessed.
    fn samples(value: Option<&Json>) -> Result<Vec<i16>, String> {
        let value = value.ok_or_else(|| "WavData wire value is missing".to_string())?;
        if value.str("kind") != "pcm16" {
            return Err(format!("the owned PCM16 oracle reads only pcm16 data, not {:?}", value.str("kind")));
        }
        Ok(value.array("value").iter().filter_map(|item| if let Json::Number(n) = item { Some(*n as i16) } else { None }).collect())
    }

    /// 🔎️ `RiffChunk` wire entries `{"fourcc": "...", "data": [byte, ...]}` — the `otherChunks` vocabulary.
    fn chunk_list(value: &Json, key: &str) -> Vec<(String, Vec<u8>)> {
        value.array(key).into_iter().map(|entry| (entry.str("fourcc"), bytes(&entry, "data"))).collect()
    }
    //#endregion 🔖️JsonReading

    //#region 🔖️FmtSpec
    /// 📐️ The PCM16 format fields of a `WavFmt` wire value; `byteRate`/`blockAlign` are re-derived by the writer.
    fn fmt_spec_of(value: Option<&Json>) -> Result<PcmWavFormat, String> {
        let value = value.ok_or_else(|| "WavFmt wire value is missing".to_string())?;
        let bits_per_sample = number(value, "bitsPerSample", 0.0) as u16;
        if bits_per_sample != 16 {
            return Err(format!("the owned PCM16 oracle writes 16-bit samples, not {bits_per_sample}-bit"));
        }
        Ok(PcmWavFormat { channels: number(value, "channels", 1.0) as u16, sample_rate: number(value, "sampleRate", 44_100.0) as u32, bits_per_sample })
    }
    //#endregion 🔖️FmtSpec

    //#region 🔖️ReadWrite
    /// 📥️ Walks the RIFF/WAVE container through `riff::Chunk`: `fmt ` and `data` are read into the owned PCM16 model,
    /// every other chunk is kept as opaque ordered bytes.
    fn read(input: &[u8]) -> Result<PcmWav, String> {
        let riff_error = |error: std::io::Error| format!("riff: {error}");
        let top = Chunk::read(&mut Cursor::new(input), 0).map_err(riff_error)?;
        if top.id() != RIFF_ID || top.read_type(&mut Cursor::new(input)).map_err(riff_error)?.value != *b"WAVE" {
            return Err("wav: missing RIFF/WAVE form".to_string());
        }
        let (mut format, mut samples, mut other_chunks) = (None, None, Vec::new());
        let children: Vec<Chunk> = top.iter(&mut Cursor::new(input)).collect::<std::io::Result<_>>().map_err(riff_error)?;
        for child in children {
            let body = child.read_contents(&mut Cursor::new(input)).map_err(riff_error)?;
            match &child.id().value {
                b"fmt " => format = Some(fmt_of(&body)?),
                b"data" => samples = Some(body.chunks_exact(2).map(|word| i16::from_le_bytes([word[0], word[1]])).collect::<Vec<_>>()),
                fourcc => other_chunks.push((String::from_utf8_lossy(fourcc).into_owned(), body)),
            }
        }
        Ok(PcmWav { format: format.ok_or("wav: missing fmt chunk")?, samples: samples.ok_or("wav: missing data chunk")?, other_chunks })
    }

    /// 📐️ The 16-byte PCM `fmt ` layout: format tag 1, channels, sample rate, byte rate, block alignment, bit depth —
    /// the two derived fields must agree with the three they derive from.
    fn fmt_of(body: &[u8]) -> Result<PcmWavFormat, String> {
        if body.len() < 16 {
            return Err(format!("wav: fmt chunk is {} byte(s), not 16", body.len()));
        }
        let (u16_at, u32_at) = (|at: usize| u16::from_le_bytes([body[at], body[at + 1]]), |at: usize| u32::from_le_bytes([body[at], body[at + 1], body[at + 2], body[at + 3]]));
        if u16_at(0) != 1 || u16_at(14) != 16 {
            return Err(format!("wav: format tag {} at {} bits is not PCM16", u16_at(0), u16_at(14)));
        }
        let format = PcmWavFormat { channels: u16_at(2), sample_rate: u32_at(4), bits_per_sample: 16 };
        if u16_at(12) != format.channels * 2 || u32_at(8) != format.sample_rate * u32::from(format.channels) * 2 {
            return Err("wav: fmt byte rate or block alignment disagrees with channels and sample rate".to_string());
        }
        Ok(format)
    }

    /// 📤️ Frames `fmt `, `data` and every auxiliary chunk through `riff::ChunkContents::write`.
    fn write(wav: &PcmWav) -> Result<Vec<u8>, String> {
        let PcmWavFormat { channels, sample_rate, bits_per_sample } = wav.format;
        if channels == 0 || wav.samples.len() % channels as usize != 0 {
            return Err(format!("wav: {} sample(s) do not fill {channels} channel(s)", wav.samples.len()));
        }
        let mut fmt = Vec::with_capacity(16);
        fmt.extend_from_slice(&1u16.to_le_bytes());
        fmt.extend_from_slice(&channels.to_le_bytes());
        fmt.extend_from_slice(&sample_rate.to_le_bytes());
        fmt.extend_from_slice(&(sample_rate * u32::from(channels) * 2).to_le_bytes());
        fmt.extend_from_slice(&(channels * 2).to_le_bytes());
        fmt.extend_from_slice(&bits_per_sample.to_le_bytes());
        let mut chunks = vec![ChunkContents::Data(ChunkId { value: *b"fmt " }, fmt), ChunkContents::Data(ChunkId { value: *b"data" }, wav.samples.iter().flat_map(|sample| sample.to_le_bytes()).collect())];
        for (fourcc, body) in &wav.other_chunks {
            let value: [u8; 4] = fourcc.as_bytes().try_into().map_err(|_| format!("wav: chunk id {fourcc:?} is not four bytes"))?;
            chunks.push(ChunkContents::Data(ChunkId { value }, body.clone()));
        }
        let mut buffer = Cursor::new(Vec::new());
        ChunkContents::Children(RIFF_ID, ChunkId { value: *b"WAVE" }, chunks).write(&mut buffer).map_err(|error| format!("riff: {error}"))?;
        Ok(buffer.into_inner())
    }
    //#endregion 🔖️ReadWrite

    //#region 🔖️Mutate
    /// 🎚️ `SetFmt` — replaces the format block wholesale; `data`/`other_chunks` are untouched.
    pub fn mutate_set_fmt(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut wav = read(input)?;
        wav.format = fmt_spec_of(params.get("fmt"))?;
        write(&wav)
    }

    /// 🔊️ `SetData` — replaces the sample data wholesale; `fmt`/`other_chunks` are untouched.
    pub fn mutate_set_data(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut wav = read(input)?;
        wav.samples = samples(params.get("data"))?;
        write(&wav)
    }

    /// 🩹️ `PatchData` — independently replaces or moves one bounded PCM16 sample range.
    pub fn mutate_patch_data(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut wav = read(input)?;
        let index = number(params, "index", f64::MAX) as usize;
        let inserted = samples(params.get("data"))?;
        if let Some(Json::Number(move_to)) = params.get("moveTo") {
            let move_to = *move_to as usize;
            if number(params, "removeCount", 0.0) != 0.0 || !inserted.is_empty() || index >= wav.samples.len() || move_to >= wav.samples.len() { return Err("patch-data move is outside the sample range or carries replacement data".into()); }
            let sample = wav.samples.remove(index);
            wav.samples.insert(move_to, sample);
        } else {
            let remove_count = number(params, "removeCount", f64::MAX) as usize;
            let end = index.checked_add(remove_count).ok_or_else(|| "patch-data range overflows".to_string())?;
            if index > wav.samples.len() || end > wav.samples.len() { return Err("patch-data range is outside the sample lane".into()); }
            wav.samples.splice(index..end, inserted);
        }
        write(&wav)
    }

    /// 📎️ `SetOtherChunks` — replaces the verbatim chunk list wholesale; `fmt`/`data` are untouched.
    pub fn mutate_set_other_chunks(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut wav = read(input)?;
        wav.other_chunks = chunk_list(params, "chunks");
        write(&wav)
    }

    /// 🔁️ `SetSnapshot` — full replace: `fmt`, `data` and `otherChunks` all come from the `snapshot` wire value.
    pub fn mutate_set_snapshot(_input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let snapshot = params.get("snapshot").ok_or_else(|| "set-snapshot carries no snapshot".to_string())?;
        write(&PcmWav { format: fmt_spec_of(snapshot.get("fmt"))?, samples: samples(snapshot.get("data"))?, other_chunks: chunk_list(snapshot, "otherChunks") })
    }

    /// 🩹️ `PatchSnapshot` — the one `SnapshotPatch` operation interpreted independently over the owned PCM model: `set` of a
    /// `fmt` field or of one `data.value` sample, `insert`/`remove` of one `data.value` sample, and `remove` of one `otherChunks`
    /// entry. A path this model has no member for is refused, never skipped.
    pub fn mutate_patch_snapshot(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut wav = read(input)?;
        if let Some(operation) = params.get("patch").cloned() {
            let path: Vec<String> = operation.str("path").split('/').skip(1).map(|segment| segment.replace("~1", "/").replace("~0", "~")).collect();
            let value = operation.get("value").cloned();
            let index = |segment: &str| segment.parse::<usize>().map_err(|_| format!("patch-snapshot index {segment:?} is not a number"));
            match (path.iter().map(String::as_str).collect::<Vec<_>>().as_slice(), operation.str("operation").as_str()) {
                (["fmt", "channels"], "set") => wav.format.channels = number(&operation, "value", 0.0) as u16,
                (["fmt", "sampleRate"], "set") => wav.format.sample_rate = number(&operation, "value", 0.0) as u32,
                (["data", "value", at], "set") => *wav.samples.get_mut(index(at)?).ok_or_else(|| format!("patch-snapshot sample {at} is outside the sample lane"))? = sample_of(value)?,
                (["data", "value", at], "remove") => {
                    let at = index(at)?;
                    if at >= wav.samples.len() { return Err(format!("patch-snapshot sample {at} is outside the sample lane")); }
                    wav.samples.remove(at);
                }
                (["data", "value", at], "insert") => {
                    let at = index(at)?;
                    if at > wav.samples.len() { return Err(format!("patch-snapshot insert {at} is outside the sample lane")); }
                    wav.samples.insert(at, sample_of(value)?);
                }
                (["otherChunks", at], "remove") => {
                    let at = index(at)?;
                    if at >= wav.other_chunks.len() { return Err(format!("patch-snapshot chunk {at} is outside the chunk list")); }
                    wav.other_chunks.remove(at);
                }
                (other, operation) => return Err(format!("patch-snapshot {operation} at {other:?} has no oracle implementation")),
            }
        }
        write(&wav)
    }

    /// 🔎️ One PCM16 sample of a `SnapshotPatch` edit value.
    fn sample_of(value: Option<Json>) -> Result<i16, String> {
        match value {
            Some(Json::Number(sample)) if (i16::MIN as f64..=i16::MAX as f64).contains(&sample) && sample.fract() == 0.0 => Ok(sample as i16),
            other => Err(format!("patch-snapshot sample value {other:?} is not a PCM16 integer")),
        }
    }
    //#endregion 🔖️Mutate

    //#region 🔖️Inverse
    /// 🔁️ Decodes and rewrites a fresh file from the owned model alone.
    pub fn rewrite(input: &[u8]) -> Result<Vec<u8>, String> {
        write(&read(input)?)
    }

    /// ↩️ The real inverse of `kind`, computed from the PRE-mutation recording and applied on top
    /// of `mutated`: each variant restores exactly the facet it replaced, leaving the others as the
    /// forward mutation left them — for `patch-snapshot`, every facet one of its edit paths starts in.
    pub fn apply_inverse(original_input: &[u8], kind: &str, params: &Json, mutated: &[u8]) -> Result<Vec<u8>, String> {
        let original = read(original_input)?;
        if kind == "set-snapshot" {
            return write(&original);
        }
        let mut restored = read(mutated)?;
        match kind {
            "set-fmt" => restored.format = original.format,
            "set-data" => restored.samples = original.samples,
            "patch-data" => restored.samples = original.samples,
            "set-other-chunks" => restored.other_chunks = original.other_chunks,
            "patch-snapshot" => {
                let path = params.get("patch").map(|patch| patch.str("path")).unwrap_or_default();
                match path.split('/').nth(1) {
                    Some("fmt") => restored.format = original.format,
                    Some("data") => restored.samples = original.samples.clone(),
                    Some("otherChunks") => restored.other_chunks = original.other_chunks.clone(),
                    other => return Err(format!("patch-snapshot edit facet {other:?} has no oracle inverse")),
                }
            }
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
