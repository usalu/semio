//! 🔮️ Mutation oracle for this subset — every mutation kind the subset declares, performed by the
//! registered `mp4` reference implementation so the subject's own mutation has an independent
//! result to be compared against instead of being checked against its own reading.
//!
//! The vocabulary is per SUBSET, not per artifact: two standards of the same format declare
//! different mutations, and a subset that shares an implementation with another reaches it through
//! the shared family modules rather than by copying it.
//!
//! `mp4` genuinely reads AND writes: every mutation below decodes the real fixture into typed
//! tracks/samples with `Mp4Reader`, mutates that owned model, and re-muxes a fresh real file with
//! `Mp4Writer` — never a byte splice. Its public surface covers exactly one AVC (`avc1`) sample
//! entry per track, a single SPS/single PPS and a hardcoded 4-byte NAL length size; that is the real
//! fixture's own shape (see `../../../../🧫️fixtures/…` derivation note in this artifact's mutation
//! case feature file), not a narrowing this oracle imposes.
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
        "set-snapshot" => reference::mutate_set_snapshot(input, &params),
        "set-ftyp" => reference::mutate_set_ftyp(input, &params),
        "insert-track" => reference::mutate_insert_track(input, &params),
        "remove-track" => reference::mutate_remove_track(input, &params),
        "set-track-dimensions" => reference::mutate_set_track_dimensions(input, &params),
        "set-track-codec" => reference::mutate_set_track_codec(input, &params),
        "insert-sample" => reference::mutate_insert_sample(input, &params),
        "remove-sample" => reference::mutate_remove_sample(input, &params),
        "set-sample-sync" => reference::mutate_set_sample_sync(input, &params),
        "" => Err("mutation spec carries no `kind`".to_string()),
        kind => Err(format!("mutation kind {:?} has no oracle implementation ({} input byte(s))", kind, input.len())),
    }
}

/// 👁️ Projects real ISO-BMFF bytes onto the shape every producer is compared through: `ftyp`,
/// per-track geometry/codec digests, and every sample's duration/cts-offset/sync/payload digest.
/// H.264 samples are container-typed and payload-opaque, so this is the exact comparison PDF/OBJ
/// use, not the lossy raster oracles' bucket/histogram approximation — a digest stands in for the
/// raw payload only because a real sample can run tens of kilobytes.
#[cfg(feature = "oracles")]
pub fn project_mp4_mutation(input: &[u8]) -> Result<Json, String> {
    reference::project(input)
}

/// 🚫️ Without the `oracles` feature the reference implementation is not linked at all.
#[cfg(not(feature = "oracles"))]
pub fn oracle_apply_mutation(_input: &[u8], _spec: &Json) -> Result<Vec<u8>, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}

#[cfg(not(feature = "oracles"))]
pub fn project_mp4_mutation(_input: &[u8]) -> Result<Json, String> {
    Err("the `oracles` feature is disabled — this host was not built with the registered reference implementations".to_string())
}
//#endregion 🔖️Dispatch

//#region 🔖️Reference
#[cfg(feature = "oracles")]
mod reference {
    use semio_repo_test_host::{digest, Json};
    use std::io::Cursor;

    //#region 🔖️JsonReading
    fn number(value: &Json, key: &str, fallback: f64) -> f64 {
        match value.get(key) {
            Some(Json::Number(found)) => *found,
            _ => fallback,
        }
    }

    fn boolean(value: &Json, key: &str, fallback: bool) -> bool {
        match value.get(key) {
            Some(Json::Bool(found)) => *found,
            _ => fallback,
        }
    }

    fn strings(value: &Json, key: &str) -> Vec<String> {
        match value.get(key) {
            Some(Json::Array(items)) => items.iter().filter_map(|item| if let Json::String(text) = item { Some(text.clone()) } else { None }).collect(),
            _ => Vec::new(),
        }
    }

    /// 🔎️ Every byte of a JSON number array — the raw payload a `sample`/`sps`/`pps` param carries.
    fn bytes(value: &Json, key: &str) -> Vec<u8> {
        match value.get(key) {
            Some(Json::Array(items)) => items.iter().filter_map(|item| if let Json::Number(n) = item { Some(*n as u8) } else { None }).collect(),
            _ => Vec::new(),
        }
    }
    //#endregion 🔖️JsonReading

    //#region 🔖️Model
    /// 🎬️ This module's own owned reading of a decoded ISO-BMFF file — deliberately not the
    /// production `Mp4Snapshot` (a second, independent model, per the fleet brief's "no reference
    /// oracle compares an implementation with itself" rule).
    struct DecodedMovie {
        major_brand: String,
        minor_version: u32,
        compatible_brands: Vec<String>,
        timescale: u32,
        tracks: Vec<DecodedTrack>,
    }

    struct DecodedTrack {
        width: u16,
        height: u16,
        timescale: u32,
        sps: Vec<u8>,
        pps: Vec<u8>,
        samples: Vec<::mp4::Mp4Sample>,
    }

    fn fourcc(value: &str) -> ::mp4::FourCC {
        let mut padded = [b' '; 4];
        for (index, byte) in value.as_bytes().iter().take(4).enumerate() {
            padded[index] = *byte;
        }
        std::str::from_utf8(&padded).expect("ASCII-padded 4-byte brand is valid UTF-8").parse::<::mp4::FourCC>().expect("exactly 4 bytes")
    }

    /// 📥️ Decodes real ISO-BMFF bytes into this module's owned `DecodedMovie` — `mp4::Mp4Reader`'s
    /// own real parse, every video track and every one of its real samples read back out of `mdat`.
    fn read_movie(input: &[u8]) -> Result<DecodedMovie, String> {
        let mut reader = ::mp4::Mp4Reader::read_header(Cursor::new(input.to_vec()), input.len() as u64).map_err(|error| format!("mp4 could not parse the ISO-BMFF stream: {error}"))?;
        let major_brand = reader.major_brand().to_string();
        let minor_version = reader.minor_version();
        let compatible_brands = reader.compatible_brands().iter().map(|brand| brand.to_string()).collect();
        let timescale = reader.timescale();

        let mut track_ids: Vec<u32> = reader.tracks().keys().copied().collect();
        track_ids.sort_unstable();
        let mut tracks = Vec::with_capacity(track_ids.len());
        for track_id in track_ids {
            let (width, height, track_timescale, sps, pps) = {
                let track = reader.tracks().get(&track_id).ok_or_else(|| format!("mp4 track {track_id} vanished between listing and lookup"))?;
                let sps = track.sequence_parameter_set().map_err(|error| format!("mp4 track {track_id} has no SPS: {error}"))?.to_vec();
                let pps = track.picture_parameter_set().map_err(|error| format!("mp4 track {track_id} has no PPS: {error}"))?.to_vec();
                (track.width(), track.height(), track.timescale(), sps, pps)
            };
            let count = reader.sample_count(track_id).map_err(|error| format!("mp4 could not count track {track_id}'s samples: {error}"))?;
            let mut samples = Vec::with_capacity(count as usize);
            for sample_id in 1..=count {
                if let Some(sample) = reader.read_sample(track_id, sample_id).map_err(|error| format!("mp4 could not read track {track_id} sample {sample_id}: {error}"))? {
                    samples.push(sample);
                }
            }
            tracks.push(DecodedTrack { width, height, timescale: track_timescale, sps, pps, samples });
        }
        Ok(DecodedMovie { major_brand, minor_version, compatible_brands, timescale, tracks })
    }

    /// ✍️ Re-muxes `movie` into fresh, real ISO-BMFF bytes with `mp4::Mp4Writer` — the ONLY channel
    /// from the decoded model back to bytes, for every mutation kind below.
    fn write_movie(movie: &DecodedMovie) -> Result<Vec<u8>, String> {
        let config = ::mp4::Mp4Config { major_brand: fourcc(&movie.major_brand), minor_version: movie.minor_version, compatible_brands: movie.compatible_brands.iter().map(|brand| fourcc(brand)).collect(), timescale: movie.timescale };
        let mut buffer = Cursor::new(Vec::<u8>::new());
        let mut writer = ::mp4::Mp4Writer::write_start(&mut buffer, &config).map_err(|error| format!("mp4 writer could not start: {error}"))?;
        for track in &movie.tracks {
            let track_config = ::mp4::TrackConfig {
                track_type: ::mp4::TrackType::Video,
                timescale: track.timescale,
                language: "und".to_string(),
                media_conf: ::mp4::MediaConfig::AvcConfig(::mp4::AvcConfig { width: track.width, height: track.height, seq_param_set: track.sps.clone(), pic_param_set: track.pps.clone() }),
            };
            writer.add_track(&track_config).map_err(|error| format!("mp4 writer could not add track: {error}"))?;
        }
        for (index, track) in movie.tracks.iter().enumerate() {
            let track_id = index as u32 + 1;
            for sample in &track.samples {
                writer.write_sample(track_id, sample).map_err(|error| format!("mp4 writer could not write sample: {error}"))?;
            }
        }
        writer.write_end().map_err(|error| format!("mp4 writer could not finish: {error}"))?;
        Ok(buffer.into_inner())
    }

    /// 🧬️ `mp4::Mp4Sample` does not implement `Clone` — a real field-by-field copy, not a derive.
    fn clone_sample(sample: &::mp4::Mp4Sample) -> ::mp4::Mp4Sample {
        ::mp4::Mp4Sample { start_time: sample.start_time, duration: sample.duration, rendering_offset: sample.rendering_offset, is_sync: sample.is_sync, bytes: sample.bytes.clone() }
    }

    fn owned_sample(value: &Json, key: &str) -> ::mp4::Mp4Sample {
        let entry = value.get(key).cloned().unwrap_or(Json::Object(Vec::new()));
        ::mp4::Mp4Sample { start_time: 0, duration: number(&entry, "duration", 0.0) as u32, rendering_offset: number(&entry, "ctsOffset", 0.0) as i32, is_sync: boolean(&entry, "sync", true), bytes: ::mp4::Bytes::from(bytes(&entry, "data")) }
    }
    //#endregion 🔖️Model

    //#region 🔖️Mutate
    /// 🏷️ `SetFtyp` — replaces `major_brand`/`minor_version`/`compatible_brands`; tracks untouched.
    pub fn mutate_set_ftyp(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut movie = read_movie(input)?;
        movie.major_brand = params.str("majorBrand");
        movie.minor_version = number(params, "minorVersion", movie.minor_version as f64) as u32;
        let brands = strings(params, "compatibleBrands");
        if !brands.is_empty() {
            movie.compatible_brands = brands;
        }
        write_movie(&movie)
    }

    /// ➕️ `InsertTrack` — a real second video track, duplicated from track 0 (the real fixture's
    /// only track: it carries no audio, so a genuinely distinct real second track does not exist —
    /// see this artifact's mutation case feature file's own note on that bound).
    pub fn mutate_insert_track(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut movie = read_movie(input)?;
        let source = movie.tracks.first().ok_or("mp4: no track to duplicate for insert-track")?;
        let clone = DecodedTrack { width: source.width, height: source.height, timescale: source.timescale, sps: source.sps.clone(), pps: source.pps.clone(), samples: source.samples.iter().map(clone_sample).collect() };
        let index = (number(params, "index", movie.tracks.len() as f64) as usize).min(movie.tracks.len());
        movie.tracks.insert(index, clone);
        write_movie(&movie)
    }

    /// ➖️ `RemoveTrack` — drops the track at `index`; on this single-track real fixture that means
    /// zero tracks remain, a legitimate real ISO-BMFF structural state `mp4::Mp4Writer` still muxes.
    pub fn mutate_remove_track(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut movie = read_movie(input)?;
        let index = number(params, "index", 0.0) as usize;
        if index >= movie.tracks.len() {
            return Err(format!("mp4: remove-track index {index} out of range ({} track(s))", movie.tracks.len()));
        }
        movie.tracks.remove(index);
        write_movie(&movie)
    }

    /// 📐️ `SetTrackDimensions` — replaces one track's `width`/`height`; codec/samples untouched.
    pub fn mutate_set_track_dimensions(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut movie = read_movie(input)?;
        let index = number(params, "trackIndex", 0.0) as usize;
        let track = movie.tracks.get_mut(index).ok_or_else(|| format!("mp4: set-track-dimensions track {index} out of range"))?;
        track.width = number(params, "width", track.width as f64) as u16;
        track.height = number(params, "height", track.height as f64) as u16;
        write_movie(&movie)
    }

    /// 🎞️ `SetTrackCodec` — replaces one track's SPS/PPS wholesale; geometry/samples untouched. The
    /// new SPS/PPS are the mutation's OWN parameter value (structural test data, not a re-derivation
    /// of the real fixture — the fixture supplies only one real codec configuration).
    pub fn mutate_set_track_codec(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut movie = read_movie(input)?;
        let index = number(params, "trackIndex", 0.0) as usize;
        let track = movie.tracks.get_mut(index).ok_or_else(|| format!("mp4: set-track-codec track {index} out of range"))?;
        let sps = bytes(params, "sps");
        let pps = bytes(params, "pps");
        if !sps.is_empty() {
            track.sps = sps;
        }
        if !pps.is_empty() {
            track.pps = pps;
        }
        write_movie(&movie)
    }

    /// ➕️ `InsertSample` — inserts one new sample at `index`; every other real sample untouched.
    pub fn mutate_insert_sample(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut movie = read_movie(input)?;
        let track_index = number(params, "trackIndex", 0.0) as usize;
        let track = movie.tracks.get_mut(track_index).ok_or_else(|| format!("mp4: insert-sample track {track_index} out of range"))?;
        let index = (number(params, "index", track.samples.len() as f64) as usize).min(track.samples.len());
        track.samples.insert(index, owned_sample(params, "sample"));
        write_movie(&movie)
    }

    /// ➖️ `RemoveSample` — removes one real sample at `index`; every other real sample untouched.
    pub fn mutate_remove_sample(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut movie = read_movie(input)?;
        let track_index = number(params, "trackIndex", 0.0) as usize;
        let track = movie.tracks.get_mut(track_index).ok_or_else(|| format!("mp4: remove-sample track {track_index} out of range"))?;
        let index = number(params, "index", 0.0) as usize;
        if index >= track.samples.len() {
            return Err(format!("mp4: remove-sample index {index} out of range ({} sample(s))", track.samples.len()));
        }
        track.samples.remove(index);
        write_movie(&movie)
    }

    /// 🔁️ `SetSampleSync` — flips one real sample's sync (random-access) flag; payload untouched.
    pub fn mutate_set_sample_sync(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut movie = read_movie(input)?;
        let track_index = number(params, "trackIndex", 0.0) as usize;
        let track = movie.tracks.get_mut(track_index).ok_or_else(|| format!("mp4: set-sample-sync track {track_index} out of range"))?;
        let index = number(params, "index", 0.0) as usize;
        let sample = track.samples.get_mut(index).ok_or_else(|| format!("mp4: set-sample-sync index {index} out of range"))?;
        sample.is_sync = boolean(params, "sync", sample.is_sync);
        write_movie(&movie)
    }

    /// 🔁️ `SetSnapshot` — a real whole-document replace touches more than one facet at once: this
    /// oracle's own reading is "replace `ftyp` AND drop the last sample of the first track", proving
    /// a real multi-facet rebuild rather than degrading to a single-field alias of `SetFtyp`.
    pub fn mutate_set_snapshot(input: &[u8], params: &Json) -> Result<Vec<u8>, String> {
        let mut movie = read_movie(input)?;
        let ftyp = params.get("ftyp").cloned().unwrap_or(Json::Object(Vec::new()));
        movie.major_brand = ftyp.str("majorBrand");
        movie.minor_version = number(&ftyp, "minorVersion", movie.minor_version as f64) as u32;
        let brands = strings(&ftyp, "compatibleBrands");
        if !brands.is_empty() {
            movie.compatible_brands = brands;
        }
        if let Some(track) = movie.tracks.first_mut() {
            track.samples.pop();
        }
        write_movie(&movie)
    }
    //#endregion 🔖️Mutate

    //#region 🔖️Project
    /// 👁️ The INDEPENDENT projection: `ftyp`, every track's geometry/codec digest, and every
    /// sample's duration/cts-offset/sync/payload digest, arrays order-significant. Track
    /// identifiers are excluded (see this subset's `🔣️component.json` profile description: `mp4`'s
    /// own writer renumbers tracks sequentially on every write, a writer convention, not content).
    pub fn project(input: &[u8]) -> Result<Json, String> {
        let movie = read_movie(input)?;
        Ok(Json::Object(vec![
            ("majorBrand".to_string(), Json::String(movie.major_brand)),
            ("minorVersion".to_string(), Json::Number(movie.minor_version as f64)),
            ("compatibleBrands".to_string(), Json::Array(movie.compatible_brands.into_iter().map(Json::String).collect())),
            ("trackCount".to_string(), Json::Number(movie.tracks.len() as f64)),
            (
                "tracks".to_string(),
                Json::Array(
                    movie
                        .tracks
                        .into_iter()
                        .map(|track| {
                            Json::Object(vec![
                                ("width".to_string(), Json::Number(track.width as f64)),
                                ("height".to_string(), Json::Number(track.height as f64)),
                                ("timescale".to_string(), Json::Number(track.timescale as f64)),
                                ("spsDigest".to_string(), Json::String(digest(&track.sps))),
                                ("ppsDigest".to_string(), Json::String(digest(&track.pps))),
                                ("sampleCount".to_string(), Json::Number(track.samples.len() as f64)),
                                (
                                    "samples".to_string(),
                                    Json::Array(
                                        track
                                            .samples
                                            .into_iter()
                                            .map(|sample| {
                                                Json::Object(vec![
                                                    ("duration".to_string(), Json::Number(sample.duration as f64)),
                                                    ("ctsOffset".to_string(), Json::Number(sample.rendering_offset as f64)),
                                                    ("sync".to_string(), Json::Bool(sample.is_sync)),
                                                    ("len".to_string(), Json::Number(sample.bytes.len() as f64)),
                                                    ("digest".to_string(), Json::String(digest(sample.bytes.as_ref()))),
                                                ])
                                            })
                                            .collect(),
                                    ),
                                ),
                            ])
                        })
                        .collect(),
                ),
            ),
        ]))
    }
    //#endregion 🔖️Project
}
//#endregion 🔖️Reference
