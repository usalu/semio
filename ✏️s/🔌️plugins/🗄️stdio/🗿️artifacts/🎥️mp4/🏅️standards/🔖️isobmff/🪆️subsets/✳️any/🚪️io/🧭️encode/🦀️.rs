//! 🧭️ Encode of the MP4 codec: the whole-file encode, the track plan, the measure and emit phases, the box size helpers, and the bounded incremental mux with its checkpointed state and the HEVC and metadata header helpers. See [`super`].

use super::*;

//#region 🔖️Encode
/// ✍️ Real ISO-BMFF encode from `Mp4Snapshot` (see this module's doc comment for the exact
/// codec_retention_law scope: logical `ftyp` and semantic sample payloads are preserved; `moov`
/// internals are a fresh, spec-valid rebuild). The deterministic layout is `ftyp`, `moov`,
/// a canonical empty `free`, and `mdat`; a first pass measures `moov`, and the second writes
/// the resulting logical chunk offsets.
pub fn encode_mp4(snapshot: &Mp4Snapshot) -> Vec<u8> {
    let mut major_brand_bytes = [b' '; 4];
    for (i, b) in snapshot.ftyp.major_brand.as_bytes().iter().take(4).enumerate() {
        major_brand_bytes[i] = *b;
    }
    let mut ftyp_payload = Vec::new();
    ftyp_payload.extend_from_slice(&major_brand_bytes);
    ftyp_payload.extend_from_slice(&snapshot.ftyp.minor_version.to_be_bytes());
    for brand in &snapshot.ftyp.compatible_brands {
        let mut b4 = [b' '; 4];
        for (i, b) in brand.as_bytes().iter().take(4).enumerate() {
            b4[i] = *b;
        }
        ftyp_payload.extend_from_slice(&b4);
    }
    let ftyp = write_box(b"ftyp", &ftyp_payload);

    let all_sample_bytes: Vec<u8> = snapshot.tracks.iter().flat_map(|t| t.samples.iter().flat_map(|s| s.data.clone())).collect();
    let mdat = write_box(b"mdat", &all_sample_bytes);
    let measured_moov = build_moov(snapshot, 0);
    let free = write_box(b"free", &[]);
    let mdat_data_offset = u32::try_from(ftyp.len() + measured_moov.len() + free.len() + 8).expect("MP4 media offset exceeds stco range");
    let moov = build_moov(snapshot, mdat_data_offset);

    [ftyp, moov, free, mdat].concat()
}

/// 🧵️ One bounded advance of the native ISO-BMFF serializer.
#[derive(Debug, PartialEq, Eq)]
pub enum Mp4EncodeAdvance {
    Progress,
    Chunk(Vec<u8>),
    Complete,
}

#[derive(Clone, Debug, Default)]
#[derive(semio_framework_value::RetireOwned)]
struct Mp4TrackPlan {
    sample_bytes: u64,
    pub(super) chunk_count: usize,
    retained_chunks: bool,
    pub(super) stsc_entries: usize,
    stts_runs: usize,
    ctts_runs: usize,
    sync_samples: usize,
    all_sync: bool,
    ctts_present: bool,
    ctts_version: u8,
    uniform_sample_size: Option<u32>,
    codec_bytes: usize,
    stbl_bytes: usize,
    trak_bytes: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(semio_framework_value::RetireOwned)]
enum Mp4MeasurePhase {
    TrackChunks,
    TrackSamples,
    TrackStsc,
    CodecSps,
    CodecPps,
    CodecAvcExtension,
    CodecHevcArrays,
    CodecHevcNals,
    FinalizeTrack,
    Finalize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(semio_framework_value::RetireOwned)]
enum Mp4EmitPhase {
    FtypHeader,
    FtypFixed,
    FtypBrand,
    MoovHeader,
    Mvhd,
    TrackHeader,
    Tkhd,
    EdtsHeader,
    ElstHeader,
    Edit,
    MdiaHeader,
    Mdhd,
    HdlrHeader,
    HdlrFixed,
    HdlrName,
    HdlrNull,
    MinfHeader,
    Vmhd,
    Dinf,
    StblHeader,
    StsdHeader,
    SampleEntryHeader,
    SampleEntryFixed,
    CodecHeader,
    CodecFixed,
    CodecNalLength,
    CodecNalData,
    CodecCount,
    CodecExtensionFixed,
    CodecHevcArray,
    CodecHevcNalLength,
    CodecHevcNalData,
    CodecExtensions,
    SttsHeader,
    SttsRunMeasure,
    SttsRunEmit,
    StssHeader,
    StssEntry,
    CttsHeader,
    CttsRunMeasure,
    CttsRunEmit,
    StscHeader,
    StscEntry,
    StszHeader,
    StszEntry,
    StcoHeader,
    StcoEntry,
    StcoAdvance,
    TrackDone,
    UdtaHeader,
    MetaHeader,
    MetaPrefix,
    IlstHeader,
    TitleHeader,
    TitleData,
    EncoderHeader,
    EncoderData,
    Free,
    MdatHeader,
    MdatSample,
    Complete,
}

fn mp4_box_size(payload: usize) -> Result<usize, String> {
    let size = payload.checked_add(8).ok_or("mp4.encode.box-size-overflow")?;
    u32::try_from(size).map_err(|_| "mp4.encode.box-size-overflow".to_string())?;
    Ok(size)
}

fn mp4_box_header(kind: &[u8; 4], total: usize) -> Result<Vec<u8>, String> {
    let mut bytes = Vec::with_capacity(8);
    bytes.extend_from_slice(&u32::try_from(total).map_err(|_| "mp4.encode.box-size-overflow")?.to_be_bytes());
    bytes.extend_from_slice(kind);
    Ok(bytes)
}

fn mp4_codec_payload_base(codec: &Mp4Codec) -> usize {
    if codec.format.is_avc() { 7 + usize::from(codec.extension.is_some()) * 4 } else if codec.format.is_hevc() { 23 } else { 0 }
}

fn mp4_codec_extension_bytes(track: &Mp4Track) -> usize {
    track.metadata.color.as_ref().map_or(0, |color| 18 + usize::from(color.full_range.is_some())) + usize::from(track.metadata.pixel_aspect_ratio.is_some()) * 16 + usize::from(track.metadata.bitrate.is_some()) * 20
}

fn mp4_metadata_item_size(value: &str) -> Result<usize, String> {
    mp4_box_size(mp4_box_size(8usize.checked_add(value.len()).ok_or("mp4.encode.metadata-size-overflow")?)?)
}

fn mp4_udta_size(movie: &Mp4Movie) -> Result<usize, String> {
    if movie.title.is_none() && movie.encoder.is_none() {
        return Ok(0);
    }
    let items = movie.title.as_deref().map(mp4_metadata_item_size).transpose()?.unwrap_or(0).checked_add(movie.encoder.as_deref().map(mp4_metadata_item_size).transpose()?.unwrap_or(0)).ok_or("mp4.encode.metadata-size-overflow")?;
    mp4_box_size(mp4_box_size(4usize.checked_add(33).and_then(|value| value.checked_add(mp4_box_size(items).ok()?)).ok_or("mp4.encode.metadata-size-overflow")?)?)
}

/// 🎚️ Incrementally serializes retained ISO-BMFF structure without materializing encoded media or sample tables.
#[derive(semio_framework_value::RetireOwned)]
pub struct Mp4EncodeCursor {
    measure: Option<Mp4MeasurePhase>,
    emit: Mp4EmitPhase,
    plans: Vec<Mp4TrackPlan>,
    working: Mp4TrackPlan,
    pub(super) track_index: usize,
    item_index: usize,
    sub_index: usize,
    pub(super) offset: usize,
    run_count: u32,
    run_u32: u32,
    run_i32: i32,
    chunk_sum: usize,
    previous_chunk_count: Option<u32>,
    moov_bytes: usize,
    mdat_bytes: u64,
    pub(super) mdat_data_offset: u32,
    stco_sample_index: usize,
    stco_sample_remaining: usize,
    stco_offset: u64,
    pub(super) emitted_bytes: u64,
}

impl Mp4EncodeCursor {
    pub fn new(_: &Mp4Snapshot) -> Self {
        Self {
            measure: Some(Mp4MeasurePhase::TrackChunks),
            emit: Mp4EmitPhase::FtypHeader,
            plans: Vec::new(),
            working: Mp4TrackPlan { all_sync: true, ..Mp4TrackPlan::default() },
            track_index: 0,
            item_index: 0,
            sub_index: 0,
            offset: 0,
            run_count: 0,
            run_u32: 0,
            run_i32: 0,
            chunk_sum: 0,
            previous_chunk_count: None,
            moov_bytes: 0,
            mdat_bytes: 0,
            mdat_data_offset: 0,
            stco_sample_index: 0,
            stco_sample_remaining: 0,
            stco_offset: 0,
            emitted_bytes: 0,
        }
    }

    pub fn emitted_bytes(&self) -> u64 { self.emitted_bytes }

    pub fn advance(&mut self, snapshot: &Mp4Snapshot, maximum_bytes: usize) -> Result<Mp4EncodeAdvance, String> {
        if maximum_bytes == 0 { return Err("mp4.encode.zero-byte-grant".into()); }
        if let Some(phase) = self.measure { return self.measure(snapshot, phase); }
        self.emit(snapshot, maximum_bytes)
    }

    fn measure(&mut self, snapshot: &Mp4Snapshot, phase: Mp4MeasurePhase) -> Result<Mp4EncodeAdvance, String> {
        let Some(track) = snapshot.tracks.get(self.track_index) else {
            self.measure = Some(Mp4MeasurePhase::Finalize);
            return self.finalize_measurement(snapshot);
        };
        match phase {
            Mp4MeasurePhase::TrackChunks => {
                if let Some(count) = track.chunk_sample_counts.get(self.item_index) {
                    self.chunk_sum = self.chunk_sum.checked_add(*count as usize).ok_or("mp4.encode.chunk-count-overflow")?;
                    self.item_index += 1;
                } else {
                    self.working.retained_chunks = !track.chunk_sample_counts.is_empty() && self.chunk_sum == track.samples.len();
                    self.working.chunk_count = if self.working.retained_chunks { track.chunk_sample_counts.len() } else { 1 };
                    self.item_index = 0;
                    self.measure = Some(Mp4MeasurePhase::TrackSamples);
                }
            }
            Mp4MeasurePhase::TrackSamples => {
                if let Some(sample) = track.samples.get(self.item_index) {
                    let size = u32::try_from(sample.data.len()).map_err(|_| "mp4.encode.sample-too-large")?;
                    self.working.sample_bytes = self.working.sample_bytes.checked_add(u64::from(size)).ok_or("mp4.encode.mdat-size-overflow")?;
                    if self.item_index == 0 || track.samples[self.item_index - 1].duration != sample.duration { self.working.stts_runs += 1; }
                    if self.item_index == 0 || track.samples[self.item_index - 1].cts_offset != sample.cts_offset { self.working.ctts_runs += 1; }
                    self.working.all_sync &= sample.sync;
                    self.working.sync_samples += usize::from(sample.sync);
                    self.working.ctts_present |= sample.cts_offset != 0;
                    self.working.ctts_version |= u8::from(sample.cts_offset < 0);
                    self.working.uniform_sample_size = match (self.item_index, self.working.uniform_sample_size) { (0, _) => Some(size), (_, Some(first)) if first == size => Some(first), _ => None };
                    self.item_index += 1;
                } else {
                    self.item_index = 0;
                    self.previous_chunk_count = None;
                    self.measure = Some(Mp4MeasurePhase::TrackStsc);
                }
            }
            Mp4MeasurePhase::TrackStsc => {
                if self.item_index < self.working.chunk_count {
                    let count = if self.working.retained_chunks { track.chunk_sample_counts[self.item_index] } else { track.samples.len() as u32 };
                    if self.previous_chunk_count != Some(count) { self.working.stsc_entries += 1; self.previous_chunk_count = Some(count); }
                    self.item_index += 1;
                } else {
                    self.working.codec_bytes = mp4_codec_payload_base(&track.codec);
                    self.item_index = 0;
                    self.measure = Some(if track.codec.format.is_avc() { Mp4MeasurePhase::CodecSps } else if track.codec.format.is_hevc() { Mp4MeasurePhase::CodecHevcArrays } else { Mp4MeasurePhase::FinalizeTrack });
                }
            }
            Mp4MeasurePhase::CodecSps => {
                if let Some(nal) = track.codec.sps.get(self.item_index) { self.working.codec_bytes = self.working.codec_bytes.checked_add(2 + nal.len()).ok_or("mp4.encode.codec-size-overflow")?; self.item_index += 1; } else { self.item_index = 0; self.measure = Some(Mp4MeasurePhase::CodecPps); }
            }
            Mp4MeasurePhase::CodecPps => {
                if let Some(nal) = track.codec.pps.get(self.item_index) { self.working.codec_bytes = self.working.codec_bytes.checked_add(2 + nal.len()).ok_or("mp4.encode.codec-size-overflow")?; self.item_index += 1; } else { self.item_index = 0; self.measure = Some(if track.codec.extension.is_some() { Mp4MeasurePhase::CodecAvcExtension } else { Mp4MeasurePhase::FinalizeTrack }); }
            }
            Mp4MeasurePhase::CodecAvcExtension => {
                let extension = track.codec.extension.as_ref().ok_or("mp4.encode.avc-extension-missing")?;
                if let Some(nal) = extension.sps_ext.get(self.item_index) { self.working.codec_bytes = self.working.codec_bytes.checked_add(2 + nal.len()).ok_or("mp4.encode.codec-size-overflow")?; self.item_index += 1; } else { self.measure = Some(Mp4MeasurePhase::FinalizeTrack); }
            }
            Mp4MeasurePhase::CodecHevcArrays => {
                let Some(config) = track.codec.hevc.as_ref() else { self.measure = Some(Mp4MeasurePhase::FinalizeTrack); return Ok(Mp4EncodeAdvance::Progress); };
                if let Some(array) = config.arrays.get(self.item_index) { self.working.codec_bytes = self.working.codec_bytes.checked_add(3).ok_or("mp4.encode.codec-size-overflow")?; if array.nal_units.is_empty() { self.item_index += 1; } else { self.sub_index = 0; self.measure = Some(Mp4MeasurePhase::CodecHevcNals); } } else { self.measure = Some(Mp4MeasurePhase::FinalizeTrack); }
            }
            Mp4MeasurePhase::CodecHevcNals => {
                let array = track.codec.hevc.as_ref().and_then(|config| config.arrays.get(self.item_index)).ok_or("mp4.encode.hevc-array-missing")?;
                if let Some(nal) = array.nal_units.get(self.sub_index) { self.working.codec_bytes = self.working.codec_bytes.checked_add(2 + nal.len()).ok_or("mp4.encode.codec-size-overflow")?; self.sub_index += 1; } else { self.item_index += 1; self.measure = Some(Mp4MeasurePhase::CodecHevcArrays); }
            }
            Mp4MeasurePhase::FinalizeTrack => self.finalize_track(track)?,
            Mp4MeasurePhase::Finalize => return self.finalize_measurement(snapshot),
        }
        Ok(Mp4EncodeAdvance::Progress)
    }

    fn finalize_track(&mut self, track: &Mp4Track) -> Result<(), String> {
        let codec = if track.codec.format.is_jpeg() { 0 } else { mp4_box_size(self.working.codec_bytes)? };
        let sample_entry = mp4_box_size(78usize.checked_add(codec).and_then(|value| value.checked_add(mp4_codec_extension_bytes(track))).ok_or("mp4.encode.stsd-size-overflow")?)?;
        let stsd = mp4_box_size(8usize.checked_add(sample_entry).ok_or("mp4.encode.stsd-size-overflow")?)?;
        let stts = mp4_box_size(8usize.checked_add(self.working.stts_runs.checked_mul(8).ok_or("mp4.encode.stts-size-overflow")?).ok_or("mp4.encode.stts-size-overflow")?)?;
        let stss = if self.working.all_sync { 0 } else { mp4_box_size(8usize.checked_add(self.working.sync_samples.checked_mul(4).ok_or("mp4.encode.stss-size-overflow")?).ok_or("mp4.encode.stss-size-overflow")?)? };
        let ctts = if self.working.ctts_present { mp4_box_size(8usize.checked_add(self.working.ctts_runs.checked_mul(8).ok_or("mp4.encode.ctts-size-overflow")?).ok_or("mp4.encode.ctts-size-overflow")?)? } else { 0 };
        let stsc = mp4_box_size(8usize.checked_add(self.working.stsc_entries.checked_mul(12).ok_or("mp4.encode.stsc-size-overflow")?).ok_or("mp4.encode.stsc-size-overflow")?)?;
        let stsz = mp4_box_size(12usize.checked_add(if self.working.uniform_sample_size.is_some() && !track.samples.is_empty() { 0 } else { track.samples.len().checked_mul(4).ok_or("mp4.encode.stsz-size-overflow")? }).ok_or("mp4.encode.stsz-size-overflow")?)?;
        let stco = mp4_box_size(8usize.checked_add(self.working.chunk_count.checked_mul(4).ok_or("mp4.encode.stco-size-overflow")?).ok_or("mp4.encode.stco-size-overflow")?)?;
        self.working.stbl_bytes = mp4_box_size(stsd.checked_add(stts).and_then(|value| value.checked_add(stss)).and_then(|value| value.checked_add(ctts)).and_then(|value| value.checked_add(stsc)).and_then(|value| value.checked_add(stsz)).and_then(|value| value.checked_add(stco)).ok_or("mp4.encode.stbl-size-overflow")?)?;
        let hdlr = mp4_box_size(25usize.checked_add(track.metadata.handler_name.len()).ok_or("mp4.encode.hdlr-size-overflow")?)?;
        let minf = mp4_box_size(20usize.checked_add(36).and_then(|value| value.checked_add(self.working.stbl_bytes)).ok_or("mp4.encode.minf-size-overflow")?)?;
        let mdia = mp4_box_size(32usize.checked_add(hdlr).and_then(|value| value.checked_add(minf)).ok_or("mp4.encode.mdia-size-overflow")?)?;
        let edts = if track.metadata.edits.is_empty() { 0 } else { mp4_box_size(mp4_box_size(8usize.checked_add(track.metadata.edits.len().checked_mul(12).ok_or("mp4.encode.edts-size-overflow")?).ok_or("mp4.encode.edts-size-overflow")?)?)? };
        self.working.trak_bytes = mp4_box_size(92usize.checked_add(edts).and_then(|value| value.checked_add(mdia)).ok_or("mp4.encode.trak-size-overflow")?)?;
        self.mdat_bytes = self.mdat_bytes.checked_add(self.working.sample_bytes).ok_or("mp4.encode.mdat-size-overflow")?;
        self.plans.push(std::mem::replace(&mut self.working, Mp4TrackPlan { all_sync: true, ..Mp4TrackPlan::default() }));
        self.track_index += 1;
        self.item_index = 0;
        self.sub_index = 0;
        self.chunk_sum = 0;
        self.previous_chunk_count = None;
        self.measure = Some(Mp4MeasurePhase::TrackChunks);
        Ok(())
    }

    fn finalize_measurement(&mut self, snapshot: &Mp4Snapshot) -> Result<Mp4EncodeAdvance, String> {
        let tracks = self.plans.iter().try_fold(0usize, |sum, plan| sum.checked_add(plan.trak_bytes).ok_or("mp4.encode.moov-size-overflow"))?;
        self.moov_bytes = mp4_box_size(108usize.checked_add(tracks).and_then(|value| value.checked_add(mp4_udta_size(&snapshot.movie).ok()?)).ok_or("mp4.encode.moov-size-overflow")?)?;
        let ftyp = mp4_box_size(8usize.checked_add(snapshot.ftyp.compatible_brands.len().checked_mul(4).ok_or("mp4.encode.ftyp-size-overflow")?).ok_or("mp4.encode.ftyp-size-overflow")?)?;
        self.mdat_data_offset = u32::try_from(ftyp.checked_add(self.moov_bytes).and_then(|value| value.checked_add(16)).ok_or("mp4.encode.media-offset-overflow")?).map_err(|_| "mp4.encode.media-offset-overflow")?;
        mp4_box_size(usize::try_from(self.mdat_bytes).map_err(|_| "mp4.encode.mdat-size-overflow")?)?;
        self.measure = None;
        self.track_index = 0;
        self.item_index = 0;
        self.sub_index = 0;
        Ok(Mp4EncodeAdvance::Progress)
    }

    fn emit(&mut self, snapshot: &Mp4Snapshot, maximum_bytes: usize) -> Result<Mp4EncodeAdvance, String> {
        match self.emit {
            Mp4EmitPhase::FtypHeader => {
                let size = mp4_box_size(8 + snapshot.ftyp.compatible_brands.len() * 4)?;
                self.emit_owned(mp4_box_header(b"ftyp", size)?, maximum_bytes, Mp4EmitPhase::FtypFixed)
            }
            Mp4EmitPhase::FtypFixed => {
                let mut bytes = [b' '; 8];
                for (index, byte) in snapshot.ftyp.major_brand.as_bytes().iter().take(4).enumerate() { bytes[index] = *byte; }
                bytes[4..].copy_from_slice(&snapshot.ftyp.minor_version.to_be_bytes());
                self.item_index = 0;
                self.emit_owned(bytes.to_vec(), maximum_bytes, Mp4EmitPhase::FtypBrand)
            }
            Mp4EmitPhase::FtypBrand => {
                let Some(brand) = snapshot.ftyp.compatible_brands.get(self.item_index) else { self.emit = Mp4EmitPhase::MoovHeader; return Ok(Mp4EncodeAdvance::Progress); };
                let mut bytes = [b' '; 4];
                for (index, byte) in brand.as_bytes().iter().take(4).enumerate() { bytes[index] = *byte; }
                self.item_index += 1;
                self.emit_owned(bytes.to_vec(), maximum_bytes, Mp4EmitPhase::FtypBrand)
            }
            Mp4EmitPhase::MoovHeader => {
                self.track_index = 0;
                self.emit_owned(mp4_box_header(b"moov", self.moov_bytes)?, maximum_bytes, Mp4EmitPhase::Mvhd)
            }
            Mp4EmitPhase::Mvhd => self.emit_owned(build_mvhd(&snapshot.movie), maximum_bytes, Mp4EmitPhase::TrackHeader),
            Mp4EmitPhase::TrackHeader => {
                let Some(plan) = self.plans.get(self.track_index) else { self.emit = Mp4EmitPhase::UdtaHeader; return Ok(Mp4EncodeAdvance::Progress); };
                self.emit_owned(mp4_box_header(b"trak", plan.trak_bytes)?, maximum_bytes, Mp4EmitPhase::Tkhd)
            }
            Mp4EmitPhase::Tkhd => self.emit_owned(build_tkhd(self.track(snapshot)?), maximum_bytes, Mp4EmitPhase::EdtsHeader),
            Mp4EmitPhase::EdtsHeader => {
                let track = self.track(snapshot)?;
                if track.metadata.edits.is_empty() { self.emit = Mp4EmitPhase::MdiaHeader; return Ok(Mp4EncodeAdvance::Progress); }
                let size = mp4_box_size(mp4_box_size(8 + track.metadata.edits.len() * 12)?)?;
                self.emit_owned(mp4_box_header(b"edts", size)?, maximum_bytes, Mp4EmitPhase::ElstHeader)
            }
            Mp4EmitPhase::ElstHeader => self.emit_elst_header(snapshot, maximum_bytes),
            Mp4EmitPhase::Edit => self.emit_edit(snapshot, maximum_bytes),
            Mp4EmitPhase::MdiaHeader => {
                let track = self.track(snapshot)?;
                let edts = if track.metadata.edits.is_empty() { 0 } else { mp4_box_size(mp4_box_size(8 + track.metadata.edits.len() * 12)?)? };
                let size = self.plan()?.trak_bytes.checked_sub(8 + 92 + edts).ok_or("mp4.encode.mdia-size-underflow")?;
                self.emit_owned(mp4_box_header(b"mdia", size)?, maximum_bytes, Mp4EmitPhase::Mdhd)
            }
            Mp4EmitPhase::Mdhd => self.emit_owned(build_mdhd(self.track(snapshot)?), maximum_bytes, Mp4EmitPhase::HdlrHeader),
            Mp4EmitPhase::HdlrHeader => {
                let size = mp4_box_size(25 + self.track(snapshot)?.metadata.handler_name.len())?;
                self.emit_owned(mp4_box_header(b"hdlr", size)?, maximum_bytes, Mp4EmitPhase::HdlrFixed)
            }
            Mp4EmitPhase::HdlrFixed => {
                let mut bytes = vec![0; 8];
                bytes.extend_from_slice(b"vide");
                bytes.extend_from_slice(&[0; 12]);
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::HdlrName)
            }
            Mp4EmitPhase::HdlrName => {
                let name = self.track(snapshot)?.metadata.handler_name.as_bytes();
                self.emit_borrowed(name, maximum_bytes, Mp4EmitPhase::HdlrNull)
            }
            Mp4EmitPhase::HdlrNull => self.emit_owned(vec![0], maximum_bytes, Mp4EmitPhase::MinfHeader),
            Mp4EmitPhase::MinfHeader => {
                let hdlr = mp4_box_size(25 + self.track(snapshot)?.metadata.handler_name.len())?;
                let track = self.track(snapshot)?;
                let edts = if track.metadata.edits.is_empty() { 0 } else { mp4_box_size(mp4_box_size(8 + track.metadata.edits.len() * 12)?)? };
                let mdia = self.plan()?.trak_bytes - 8 - 92 - edts;
                let size = mdia.checked_sub(8 + 32 + hdlr).ok_or("mp4.encode.minf-size-underflow")?;
                self.emit_owned(mp4_box_header(b"minf", size)?, maximum_bytes, Mp4EmitPhase::Vmhd)
            }
            Mp4EmitPhase::Vmhd => self.emit_owned(build_vmhd(), maximum_bytes, Mp4EmitPhase::Dinf),
            Mp4EmitPhase::Dinf => self.emit_owned(build_dinf(), maximum_bytes, Mp4EmitPhase::StblHeader),
            Mp4EmitPhase::StblHeader => self.emit_owned(mp4_box_header(b"stbl", self.plan()?.stbl_bytes)?, maximum_bytes, Mp4EmitPhase::StsdHeader),
            Mp4EmitPhase::StsdHeader => self.emit_stsd_header(snapshot, maximum_bytes),
            Mp4EmitPhase::SampleEntryHeader => {
                let track = self.track(snapshot)?;
                let codec = if track.codec.format.is_jpeg() { 0 } else { mp4_box_size(self.plan()?.codec_bytes)? };
                let size = mp4_box_size(78 + codec + mp4_codec_extension_bytes(track))?;
                let mut kind = [0; 4]; kind.copy_from_slice(track.codec.format.fourcc().as_bytes());
                self.emit_owned(mp4_box_header(&kind, size)?, maximum_bytes, Mp4EmitPhase::SampleEntryFixed)
            }
            Mp4EmitPhase::SampleEntryFixed => {
                let track = self.track(snapshot)?;
                let mut kind = [0; 4]; kind.copy_from_slice(track.codec.format.fourcc().as_bytes());
                let bytes = mp4_visual_sample_entry(&kind, track.width as u16, track.height as u16, &track.metadata.visual, &[])[8..].to_vec();
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::CodecHeader)
            }
            Mp4EmitPhase::CodecHeader => {
                let track = self.track(snapshot)?;
                if track.codec.format.is_jpeg() { self.emit = Mp4EmitPhase::CodecExtensions; return Ok(Mp4EncodeAdvance::Progress); }
                let kind = if track.codec.format.is_avc() { b"avcC" } else { b"hvcC" };
                self.emit_owned(mp4_box_header(kind, mp4_box_size(self.plan()?.codec_bytes)?)?, maximum_bytes, Mp4EmitPhase::CodecFixed)
            }
            Mp4EmitPhase::CodecFixed => self.emit_codec_fixed(snapshot, maximum_bytes),
            Mp4EmitPhase::CodecNalLength => self.emit_codec_nal_length(snapshot, maximum_bytes),
            Mp4EmitPhase::CodecNalData => {
                let track = self.track(snapshot)?;
                let list = match self.sub_index { 0 => &track.codec.sps, 1 => &track.codec.pps, 2 => &track.codec.extension.as_ref().ok_or("mp4.encode.avc-extension-missing")?.sps_ext, _ => return Err("mp4.encode.avc-list-invalid".into()) };
                let nal = list.get(self.item_index).ok_or("mp4.encode.avc-nal-missing")?;
                if self.offset + maximum_bytes >= nal.len() { self.item_index += 1; }
                self.emit_borrowed(nal, maximum_bytes, Mp4EmitPhase::CodecNalLength)
            }
            Mp4EmitPhase::CodecCount => {
                self.sub_index = 1;
                self.item_index = 0;
                self.emit_owned(vec![self.track(snapshot)?.codec.pps.len() as u8], maximum_bytes, Mp4EmitPhase::CodecNalLength)
            }
            Mp4EmitPhase::CodecExtensionFixed => {
                let extension = self.track(snapshot)?.codec.extension.as_ref().ok_or("mp4.encode.avc-extension-missing")?;
                self.sub_index = 2;
                self.item_index = 0;
                self.emit_owned(vec![0xfc | (extension.chroma_format & 3), 0xf8 | (extension.bit_depth_luma_minus8 & 7), 0xf8 | (extension.bit_depth_chroma_minus8 & 7), extension.sps_ext.len() as u8], maximum_bytes, Mp4EmitPhase::CodecNalLength)
            }
            Mp4EmitPhase::CodecHevcArray => self.emit_codec_hevc_array(snapshot, maximum_bytes),
            Mp4EmitPhase::CodecHevcNalLength => {
                let array = self.track(snapshot)?.codec.hevc.as_ref().and_then(|config| config.arrays.get(self.item_index)).ok_or("mp4.encode.hevc-array-missing")?;
                let Some(nal) = array.nal_units.get(self.sub_index) else { self.item_index += 1; self.emit = Mp4EmitPhase::CodecHevcArray; return Ok(Mp4EncodeAdvance::Progress); };
                self.emit_owned((nal.len() as u16).to_be_bytes().to_vec(), maximum_bytes, Mp4EmitPhase::CodecHevcNalData)
            }
            Mp4EmitPhase::CodecHevcNalData => {
                let nal = self.track(snapshot)?.codec.hevc.as_ref().and_then(|config| config.arrays.get(self.item_index)).and_then(|array| array.nal_units.get(self.sub_index)).ok_or("mp4.encode.hevc-nal-missing")?;
                if self.offset + maximum_bytes >= nal.len() { self.sub_index += 1; }
                self.emit_borrowed(nal, maximum_bytes, Mp4EmitPhase::CodecHevcNalLength)
            }
            Mp4EmitPhase::CodecExtensions => {
                let bytes = build_codec_extensions(self.track(snapshot)?);
                if bytes.is_empty() { self.emit = Mp4EmitPhase::SttsHeader; return Ok(Mp4EncodeAdvance::Progress); }
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::SttsHeader)
            }
            Mp4EmitPhase::SttsHeader => {
                let mut bytes = mp4_box_header(b"stts", mp4_box_size(8 + self.plan()?.stts_runs * 8)?)?;
                bytes.extend_from_slice(&[0; 4]); bytes.extend_from_slice(&(self.plan()?.stts_runs as u32).to_be_bytes());
                self.item_index = 0; self.run_count = 0;
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::SttsRunMeasure)
            }
            Mp4EmitPhase::SttsRunMeasure => self.measure_emit_u32_run(snapshot, true),
            Mp4EmitPhase::SttsRunEmit => {
                let mut bytes = self.run_count.to_be_bytes().to_vec(); bytes.extend_from_slice(&self.run_u32.to_be_bytes()); self.run_count = 0;
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::SttsRunMeasure)
            }
            Mp4EmitPhase::StssHeader => {
                if self.plan()?.all_sync { self.emit = Mp4EmitPhase::CttsHeader; return Ok(Mp4EncodeAdvance::Progress); }
                let mut bytes = mp4_box_header(b"stss", mp4_box_size(8 + self.plan()?.sync_samples * 4)?)?;
                bytes.extend_from_slice(&[0; 4]); bytes.extend_from_slice(&(self.plan()?.sync_samples as u32).to_be_bytes()); self.item_index = 0;
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::StssEntry)
            }
            Mp4EmitPhase::StssEntry => {
                let track = self.track(snapshot)?;
                let Some(sample) = track.samples.get(self.item_index) else { self.emit = Mp4EmitPhase::CttsHeader; return Ok(Mp4EncodeAdvance::Progress); };
                self.item_index += 1;
                if !sample.sync { return Ok(Mp4EncodeAdvance::Progress); }
                self.emit_owned((self.item_index as u32).to_be_bytes().to_vec(), maximum_bytes, Mp4EmitPhase::StssEntry)
            }
            Mp4EmitPhase::CttsHeader => {
                if !self.plan()?.ctts_present { self.emit = Mp4EmitPhase::StscHeader; return Ok(Mp4EncodeAdvance::Progress); }
                let mut bytes = mp4_box_header(b"ctts", mp4_box_size(8 + self.plan()?.ctts_runs * 8)?)?;
                bytes.extend_from_slice(&[self.plan()?.ctts_version, 0, 0, 0]); bytes.extend_from_slice(&(self.plan()?.ctts_runs as u32).to_be_bytes()); self.item_index = 0; self.run_count = 0;
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::CttsRunMeasure)
            }
            Mp4EmitPhase::CttsRunMeasure => self.measure_emit_i32_run(snapshot),
            Mp4EmitPhase::CttsRunEmit => {
                let mut bytes = self.run_count.to_be_bytes().to_vec(); bytes.extend_from_slice(&(self.run_i32 as u32).to_be_bytes()); self.run_count = 0;
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::CttsRunMeasure)
            }
            Mp4EmitPhase::StscHeader => {
                let mut bytes = mp4_box_header(b"stsc", mp4_box_size(8 + self.plan()?.stsc_entries * 12)?)?;
                bytes.extend_from_slice(&[0; 4]); bytes.extend_from_slice(&(self.plan()?.stsc_entries as u32).to_be_bytes()); self.item_index = 0; self.previous_chunk_count = None;
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::StscEntry)
            }
            Mp4EmitPhase::StscEntry => self.emit_stsc_entry(snapshot, maximum_bytes),
            Mp4EmitPhase::StszHeader => {
                let track = self.track(snapshot)?;
                let uniform = self.plan()?.uniform_sample_size.filter(|_| !track.samples.is_empty());
                let size = mp4_box_size(12 + if uniform.is_some() { 0 } else { track.samples.len() * 4 })?;
                let mut bytes = mp4_box_header(b"stsz", size)?; bytes.extend_from_slice(&[0; 4]); bytes.extend_from_slice(&uniform.unwrap_or(0).to_be_bytes()); bytes.extend_from_slice(&(track.samples.len() as u32).to_be_bytes()); self.item_index = 0;
                let next = if uniform.is_some() { Mp4EmitPhase::StcoHeader } else { Mp4EmitPhase::StszEntry };
                self.emit_owned(bytes, maximum_bytes, next)
            }
            Mp4EmitPhase::StszEntry => {
                let Some(sample) = self.track(snapshot)?.samples.get(self.item_index) else { self.emit = Mp4EmitPhase::StcoHeader; return Ok(Mp4EncodeAdvance::Progress); };
                self.item_index += 1;
                self.emit_owned((sample.data.len() as u32).to_be_bytes().to_vec(), maximum_bytes, Mp4EmitPhase::StszEntry)
            }
            Mp4EmitPhase::StcoHeader => {
                let mut bytes = mp4_box_header(b"stco", mp4_box_size(8 + self.plan()?.chunk_count * 4)?)?; bytes.extend_from_slice(&[0; 4]); bytes.extend_from_slice(&(self.plan()?.chunk_count as u32).to_be_bytes());
                self.item_index = 0; self.stco_sample_index = 0; self.stco_sample_remaining = 0; self.stco_offset = u64::from(self.mdat_data_offset) + self.plans[..self.track_index].iter().map(|plan| plan.sample_bytes).sum::<u64>();
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::StcoEntry)
            }
            Mp4EmitPhase::StcoEntry => {
                if self.item_index >= self.plan()?.chunk_count { self.emit = Mp4EmitPhase::TrackDone; return Ok(Mp4EncodeAdvance::Progress); }
                let offset = u32::try_from(self.stco_offset).map_err(|_| "mp4.encode.media-offset-overflow")?;
                self.stco_sample_remaining = self.chunk_count(snapshot, self.item_index)? as usize;
                self.item_index += 1;
                self.emit_owned(offset.to_be_bytes().to_vec(), maximum_bytes, Mp4EmitPhase::StcoAdvance)
            }
            Mp4EmitPhase::StcoAdvance => {
                if self.stco_sample_remaining == 0 { self.emit = Mp4EmitPhase::StcoEntry; return Ok(Mp4EncodeAdvance::Progress); }
                let sample = self.track(snapshot)?.samples.get(self.stco_sample_index).ok_or("mp4.encode.chunk-sample-missing")?;
                self.stco_offset = self.stco_offset.checked_add(sample.data.len() as u64).ok_or("mp4.encode.media-offset-overflow")?;
                self.stco_sample_index += 1; self.stco_sample_remaining -= 1;
                Ok(Mp4EncodeAdvance::Progress)
            }
            Mp4EmitPhase::TrackDone => { self.track_index += 1; self.item_index = 0; self.emit = Mp4EmitPhase::TrackHeader; Ok(Mp4EncodeAdvance::Progress) }
            Mp4EmitPhase::UdtaHeader => {
                let size = mp4_udta_size(&snapshot.movie)?;
                if size == 0 { self.emit = Mp4EmitPhase::Free; return Ok(Mp4EncodeAdvance::Progress); }
                self.emit_owned(mp4_box_header(b"udta", size)?, maximum_bytes, Mp4EmitPhase::MetaHeader)
            }
            Mp4EmitPhase::MetaHeader => {
                let udta = mp4_udta_size(&snapshot.movie)?;
                self.emit_owned(mp4_box_header(b"meta", udta - 8)?, maximum_bytes, Mp4EmitPhase::MetaPrefix)
            }
            Mp4EmitPhase::MetaPrefix => {
                let mut bytes = vec![0; 4];
                let mut handler = vec![0; 8]; handler.extend_from_slice(b"mdir"); handler.extend_from_slice(b"appl"); handler.extend_from_slice(&[0; 8]); handler.push(0);
                bytes.extend(write_box(b"hdlr", &handler));
                self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::IlstHeader)
            }
            Mp4EmitPhase::IlstHeader => {
                let items = snapshot.movie.title.as_deref().map(mp4_metadata_item_size).transpose()?.unwrap_or(0) + snapshot.movie.encoder.as_deref().map(mp4_metadata_item_size).transpose()?.unwrap_or(0);
                self.emit_owned(mp4_box_header(b"ilst", mp4_box_size(items)?)?, maximum_bytes, if snapshot.movie.title.is_some() { Mp4EmitPhase::TitleHeader } else { Mp4EmitPhase::EncoderHeader })
            }
            Mp4EmitPhase::TitleHeader => {
                let title = snapshot.movie.title.as_deref().ok_or("mp4.encode.title-missing")?;
                self.emit_owned(mp4_metadata_headers(&[0xa9, b'n', b'a', b'm'], title)?, maximum_bytes, Mp4EmitPhase::TitleData)
            }
            Mp4EmitPhase::TitleData => self.emit_borrowed(snapshot.movie.title.as_deref().ok_or("mp4.encode.title-missing")?.as_bytes(), maximum_bytes, Mp4EmitPhase::EncoderHeader),
            Mp4EmitPhase::EncoderHeader => {
                let Some(encoder) = snapshot.movie.encoder.as_deref() else { self.emit = Mp4EmitPhase::Free; return Ok(Mp4EncodeAdvance::Progress); };
                self.emit_owned(mp4_metadata_headers(&[0xa9, b't', b'o', b'o'], encoder)?, maximum_bytes, Mp4EmitPhase::EncoderData)
            }
            Mp4EmitPhase::EncoderData => self.emit_borrowed(snapshot.movie.encoder.as_deref().ok_or("mp4.encode.encoder-missing")?.as_bytes(), maximum_bytes, Mp4EmitPhase::Free),
            Mp4EmitPhase::Free => self.emit_owned(write_box(b"free", &[]), maximum_bytes, Mp4EmitPhase::MdatHeader),
            Mp4EmitPhase::MdatHeader => {
                self.track_index = 0; self.item_index = 0;
                self.emit_owned(mp4_box_header(b"mdat", mp4_box_size(usize::try_from(self.mdat_bytes).map_err(|_| "mp4.encode.mdat-size-overflow")?)?)?, maximum_bytes, Mp4EmitPhase::MdatSample)
            }
            Mp4EmitPhase::MdatSample => {
                while self.track_index < snapshot.tracks.len() && self.item_index >= snapshot.tracks[self.track_index].samples.len() { self.track_index += 1; self.item_index = 0; }
                let Some(track) = snapshot.tracks.get(self.track_index) else { self.emit = Mp4EmitPhase::Complete; return Ok(Mp4EncodeAdvance::Progress); };
                let sample = track.samples.get(self.item_index).ok_or("mp4.encode.sample-missing")?;
                if self.offset + maximum_bytes >= sample.data.len() { self.item_index += 1; }
                self.emit_borrowed(&sample.data, maximum_bytes, Mp4EmitPhase::MdatSample)
            }
            Mp4EmitPhase::Complete => Ok(Mp4EncodeAdvance::Complete),
        }
    }

    /// 🧱️ Runs the `elst header` phase of `emit`.
    fn emit_elst_header(&mut self, snapshot: &Mp4Snapshot, maximum_bytes: usize) -> Result<Mp4EncodeAdvance, String> {
        let track = self.track(snapshot)?;
        let size = mp4_box_size(8 + track.metadata.edits.len() * 12)?;
        let mut bytes = mp4_box_header(b"elst", size)?;
        bytes.extend_from_slice(&[0; 4]);
        bytes.extend_from_slice(&(track.metadata.edits.len() as u32).to_be_bytes());
        self.item_index = 0;
        self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::Edit)
    }

    /// 🪵️ Runs the `edit` phase of `emit`.
    fn emit_edit(&mut self, snapshot: &Mp4Snapshot, maximum_bytes: usize) -> Result<Mp4EncodeAdvance, String> {
        let Some(edit) = self.track(snapshot)?.metadata.edits.get(self.item_index) else { self.emit = Mp4EmitPhase::MdiaHeader; return Ok(Mp4EncodeAdvance::Progress); };
        let mut bytes = Vec::with_capacity(12);
        bytes.extend_from_slice(&(edit.segment_duration as u32).to_be_bytes());
        bytes.extend_from_slice(&(edit.media_time as i32).to_be_bytes());
        bytes.extend_from_slice(&edit.media_rate_integer.to_be_bytes());
        bytes.extend_from_slice(&edit.media_rate_fraction.to_be_bytes());
        self.item_index += 1;
        self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::Edit)
    }

    /// 🔩️ Runs the `stsd header` phase of `emit`.
    fn emit_stsd_header(&mut self, snapshot: &Mp4Snapshot, maximum_bytes: usize) -> Result<Mp4EncodeAdvance, String> {
        let track = self.track(snapshot)?;
        let codec = if track.codec.format.is_jpeg() { 0 } else { mp4_box_size(self.plan()?.codec_bytes)? };
        let entry = mp4_box_size(78 + codec + mp4_codec_extension_bytes(track))?;
        let mut bytes = mp4_box_header(b"stsd", mp4_box_size(8 + entry)?)?;
        bytes.extend_from_slice(&[0; 4]);
        bytes.extend_from_slice(&1u32.to_be_bytes());
        self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::SampleEntryHeader)
    }

    /// ⚙️ Runs the `codec fixed` phase of `emit`.
    fn emit_codec_fixed(&mut self, snapshot: &Mp4Snapshot, maximum_bytes: usize) -> Result<Mp4EncodeAdvance, String> {
        let track = self.track(snapshot)?;
        self.item_index = 0;
        self.sub_index = 0;
        if track.codec.format.is_avc() {
            let (profile, compatibility, level) = track.codec.sps.first().and_then(|sps| sps.get(1..4)).map_or((66, 0, 30), |bytes| (bytes[0], bytes[1], bytes[2]));
            self.emit_owned(vec![1, profile, compatibility, level, 0xfc | (track.codec.nal_length_size.saturating_sub(1) & 3), 0xe0 | (track.codec.sps.len() as u8 & 0x1f)], maximum_bytes, Mp4EmitPhase::CodecNalLength)
        } else {
            let bytes = mp4_hevc_fixed(&track.codec);
            self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::CodecHevcArray)
        }
    }

    /// 🧲️ Runs the `codec nal length` phase of `emit`.
    fn emit_codec_nal_length(&mut self, snapshot: &Mp4Snapshot, maximum_bytes: usize) -> Result<Mp4EncodeAdvance, String> {
        let track = self.track(snapshot)?;
        let list = match self.sub_index { 0 => &track.codec.sps, 1 => &track.codec.pps, 2 => &track.codec.extension.as_ref().ok_or("mp4.encode.avc-extension-missing")?.sps_ext, _ => return Err("mp4.encode.avc-list-invalid".into()) };
        let Some(nal) = list.get(self.item_index) else {
            self.item_index = 0;
            self.emit = match self.sub_index { 0 => Mp4EmitPhase::CodecCount, 1 if track.codec.extension.is_some() => Mp4EmitPhase::CodecExtensionFixed, 1 | 2 => Mp4EmitPhase::CodecExtensions, _ => return Err("mp4.encode.avc-list-invalid".into()) };
            return Ok(Mp4EncodeAdvance::Progress);
        };
        self.emit_owned((nal.len() as u16).to_be_bytes().to_vec(), maximum_bytes, Mp4EmitPhase::CodecNalData)
    }

    /// 🪛️ Runs the `codec hevc array` phase of `emit`.
    fn emit_codec_hevc_array(&mut self, snapshot: &Mp4Snapshot, maximum_bytes: usize) -> Result<Mp4EncodeAdvance, String> {
        let Some(config) = self.track(snapshot)?.codec.hevc.as_ref() else { self.emit = Mp4EmitPhase::CodecExtensions; return Ok(Mp4EncodeAdvance::Progress); };
        let Some(array) = config.arrays.get(self.item_index) else { self.emit = Mp4EmitPhase::CodecExtensions; return Ok(Mp4EncodeAdvance::Progress); };
        self.sub_index = 0;
        let mut bytes = vec![(u8::from(array.array_completeness) << 7) | (array.nal_unit_type & 0x3f)];
        bytes.extend_from_slice(&(array.nal_units.len() as u16).to_be_bytes());
        let next = if array.nal_units.is_empty() { self.item_index += 1; Mp4EmitPhase::CodecHevcArray } else { Mp4EmitPhase::CodecHevcNalLength };
        self.emit_owned(bytes, maximum_bytes, next)
    }

    /// 🔧️ Runs the `stsc entry` phase of `emit`.
    fn emit_stsc_entry(&mut self, snapshot: &Mp4Snapshot, maximum_bytes: usize) -> Result<Mp4EncodeAdvance, String> {
        if self.item_index >= self.plan()?.chunk_count { self.emit = Mp4EmitPhase::StszHeader; return Ok(Mp4EncodeAdvance::Progress); }
        let count = self.chunk_count(snapshot, self.item_index)?;
        let first_chunk = self.item_index as u32 + 1;
        self.item_index += 1;
        if self.previous_chunk_count == Some(count) { return Ok(Mp4EncodeAdvance::Progress); }
        self.previous_chunk_count = Some(count);
        let mut bytes = first_chunk.to_be_bytes().to_vec(); bytes.extend_from_slice(&count.to_be_bytes()); bytes.extend_from_slice(&1u32.to_be_bytes());
        self.emit_owned(bytes, maximum_bytes, Mp4EmitPhase::StscEntry)
    }

    pub(super) fn track<'a>(&self, snapshot: &'a Mp4Snapshot) -> Result<&'a Mp4Track, String> { snapshot.tracks.get(self.track_index).ok_or_else(|| "mp4.encode.track-missing".into()) }
    fn plan(&self) -> Result<&Mp4TrackPlan, String> { self.plans.get(self.track_index).ok_or_else(|| "mp4.encode.track-plan-missing".into()) }
    pub(super) fn chunk_count(&self, snapshot: &Mp4Snapshot, index: usize) -> Result<u32, String> {
        let plan = self.plan()?;
        if plan.retained_chunks { self.track(snapshot)?.chunk_sample_counts.get(index).copied().ok_or_else(|| "mp4.encode.chunk-missing".into()) } else if index == 0 { Ok(self.track(snapshot)?.samples.len() as u32) } else { Err("mp4.encode.chunk-missing".into()) }
    }

    fn measure_emit_u32_run(&mut self, snapshot: &Mp4Snapshot, duration: bool) -> Result<Mp4EncodeAdvance, String> {
        let samples = &self.track(snapshot)?.samples;
        if self.item_index >= samples.len() { if self.run_count > 0 { self.emit = Mp4EmitPhase::SttsRunEmit; } else { self.emit = Mp4EmitPhase::StssHeader; } return Ok(Mp4EncodeAdvance::Progress); }
        let value = if duration { samples[self.item_index].duration } else { 0 };
        if self.run_count == 0 { self.run_u32 = value; self.run_count = 1; self.item_index += 1; } else if self.run_u32 == value { self.run_count = self.run_count.checked_add(1).ok_or("mp4.encode.run-overflow")?; self.item_index += 1; } else { self.emit = Mp4EmitPhase::SttsRunEmit; }
        Ok(Mp4EncodeAdvance::Progress)
    }

    fn measure_emit_i32_run(&mut self, snapshot: &Mp4Snapshot) -> Result<Mp4EncodeAdvance, String> {
        let samples = &self.track(snapshot)?.samples;
        if self.item_index >= samples.len() { if self.run_count > 0 { self.emit = Mp4EmitPhase::CttsRunEmit; } else { self.emit = Mp4EmitPhase::StscHeader; } return Ok(Mp4EncodeAdvance::Progress); }
        let value = samples[self.item_index].cts_offset;
        if self.run_count == 0 { self.run_i32 = value; self.run_count = 1; self.item_index += 1; } else if self.run_i32 == value { self.run_count = self.run_count.checked_add(1).ok_or("mp4.encode.run-overflow")?; self.item_index += 1; } else { self.emit = Mp4EmitPhase::CttsRunEmit; }
        Ok(Mp4EncodeAdvance::Progress)
    }

    fn emit_owned(&mut self, bytes: Vec<u8>, maximum_bytes: usize, next: Mp4EmitPhase) -> Result<Mp4EncodeAdvance, String> { self.emit_borrowed(&bytes, maximum_bytes, next) }

    fn emit_borrowed(&mut self, bytes: &[u8], maximum_bytes: usize, next: Mp4EmitPhase) -> Result<Mp4EncodeAdvance, String> {
        if bytes.is_empty() { self.offset = 0; self.emit = next; return Ok(Mp4EncodeAdvance::Progress); }
        let end = self.offset.checked_add(maximum_bytes).unwrap_or(usize::MAX).min(bytes.len());
        let chunk = bytes.get(self.offset..end).ok_or("mp4.encode.offset-invalid")?.to_vec();
        self.offset = end;
        self.emitted_bytes = self.emitted_bytes.checked_add(chunk.len() as u64).ok_or("mp4.encode.output-size-overflow")?;
        if self.offset == bytes.len() { self.offset = 0; self.emit = next; }
        Ok(Mp4EncodeAdvance::Chunk(chunk))
    }
}

fn mp4_hevc_fixed(codec: &Mp4Codec) -> Vec<u8> {
    let fallback = Mp4HevcConfig::default();
    let config = codec.hevc.as_ref().unwrap_or(&fallback);
    let mut bytes = vec![1, ((config.general_profile_space & 3) << 6) | (u8::from(config.general_tier_flag) << 5) | (config.general_profile_idc & 0x1f)];
    bytes.extend_from_slice(&config.general_profile_compatibility_flags.to_be_bytes());
    bytes.extend_from_slice(&config.general_constraint_indicator_flags.to_be_bytes()[2..]);
    bytes.push(config.general_level_idc);
    bytes.extend_from_slice(&(0xf000 | (config.min_spatial_segmentation_idc & 0x0fff)).to_be_bytes());
    bytes.push(0xfc | (config.parallelism_type & 3)); bytes.push(0xfc | (config.chroma_format_idc & 3)); bytes.push(0xf8 | (config.bit_depth_luma_minus8 & 7)); bytes.push(0xf8 | (config.bit_depth_chroma_minus8 & 7));
    bytes.extend_from_slice(&config.avg_frame_rate.to_be_bytes());
    bytes.push(((config.constant_frame_rate & 3) << 6) | ((config.num_temporal_layers & 7) << 3) | (u8::from(config.temporal_id_nested) << 2) | (codec.nal_length_size.saturating_sub(1) & 3));
    bytes.push(config.arrays.len() as u8);
    bytes
}

fn mp4_metadata_headers(kind: &[u8; 4], value: &str) -> Result<Vec<u8>, String> {
    let data = mp4_box_size(8usize.checked_add(value.len()).ok_or("mp4.encode.metadata-size-overflow")?)?;
    let mut bytes = mp4_box_header(kind, mp4_box_size(data)?)?;
    bytes.extend(mp4_box_header(b"data", data)?);
    bytes.extend_from_slice(&[0, 0, 0, 1, 0, 0, 0, 0]);
    Ok(bytes)
}
//#endregion 🔖️Encode
