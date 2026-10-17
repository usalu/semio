//! ⏯️ Resumable encoding of the AVI codec: the steps of the encode cursor that emit the RIFF headers, the stream headers and the chunk payloads. See [`super`].

use super::*;

//#region 🔖️Encode
impl AviEncodeCursor {
    pub fn new(_: &AviSnapshot) -> Self {
        Self {
            phase: AviEncodePhase::MeasureStreams,
            stream_index: 0,
            item_index: 0,
            offset: 0,
            hdrl_children_bytes: chunk_total(56).expect("fixed AVI main header fits"),
            movi_children_bytes: 0,
            unknown_bytes: 0,
            total_chunks: 0,
            current_stream_children_bytes: 0,
            idx1_offset: 4,
            emitted_bytes: 0,
        }
    }

    pub fn emitted_bytes(&self) -> u64 {
        self.emitted_bytes
    }

    pub fn advance(&mut self, snapshot: &AviSnapshot, maximum_bytes: usize) -> Result<AviEncodeAdvance, String> {
        if maximum_bytes == 0 {
            return Err("avi.encode.zero-byte-grant".into());
        }
        match self.phase {
            AviEncodePhase::MeasureStreams => self.measure_stream(snapshot),
            AviEncodePhase::MeasureStreamExtras => self.measure_stream_extra(snapshot),
            AviEncodePhase::MeasureHdrlExtras => self.measure_hdrl_extra(snapshot),
            AviEncodePhase::MeasureMovi => self.measure_movi(snapshot),
            AviEncodePhase::MeasureUnknown => self.measure_unknown(snapshot),
            AviEncodePhase::RiffHeader => {
                let hdrl = list_total(self.hdrl_children_bytes)?;
                let movi = list_total(self.movi_children_bytes)?;
                let idx1 = if snapshot.idx1_present { chunk_total(self.total_chunks.checked_mul(16).ok_or("avi.encode.idx1-size-overflow")?)? } else { 0 };
                let body = 4usize.checked_add(hdrl).and_then(|value| value.checked_add(movi)).and_then(|value| value.checked_add(idx1)).and_then(|value| value.checked_add(self.unknown_bytes)).ok_or("avi.encode.riff-size-overflow")?;
                let bytes = riff_root_header(body)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::HdrlHeader)
            }
            AviEncodePhase::HdrlHeader => {
                let bytes = list_header(b"hdrl", self.hdrl_children_bytes)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::AvihHeader)
            }
            AviEncodePhase::AvihHeader => {
                let bytes = chunk_header(b"avih", 56)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::AvihData)
            }
            AviEncodePhase::AvihData => {
                let bytes = avih_fixed(&snapshot.main_header);
                let next = self.phase_after_avih(snapshot);
                self.emit_fixed(&bytes, maximum_bytes, next)
            }
            AviEncodePhase::MeasureEmittedStream => self.measure_emitted_stream(snapshot),
            AviEncodePhase::MeasureEmittedStreamExtras => self.measure_emitted_stream_extra(snapshot),
            AviEncodePhase::StreamListHeader => {
                let bytes = list_header(b"strl", self.current_stream_children_bytes)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::StreamHeaderHeader)
            }
            AviEncodePhase::StreamHeaderHeader => {
                let stream = self.stream(snapshot)?;
                let bytes = chunk_header(b"strh", stream_header_payload_len(stream)?)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::StreamHeaderData)
            }
            AviEncodePhase::StreamHeaderData => {
                let stream = self.stream(snapshot)?;
                let bytes = stream_header_fixed(&stream.strh)?;
                let next = if stream.strh.strh_extra.is_empty() { self.phase_after_stream_header_data(stream) } else { AviEncodePhase::StreamHeaderExtra };
                self.emit_fixed(&bytes, maximum_bytes, next)
            }
            AviEncodePhase::StreamHeaderExtra => {
                let stream = self.stream(snapshot)?;
                let extra = &stream.strh.strh_extra;
                let next = self.phase_after_stream_header_data(stream);
                self.emit_borrowed(extra, maximum_bytes, next)
            }
            AviEncodePhase::StreamHeaderPad => self.emit_padding(maximum_bytes, AviEncodePhase::StreamFormatHeader),
            AviEncodePhase::StreamFormatHeader => {
                let stream = self.stream(snapshot)?;
                let bytes = chunk_header(b"strf", stream_format_payload_len(&stream.strf))?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::StreamFormatData)
            }
            AviEncodePhase::StreamFormatData => {
                let stream = self.stream(snapshot)?;
                let bytes = stream_format_fixed(&stream.strf);
                let next = if stream_format_extra(&stream.strf).is_empty() {
                    if stream_format_payload_len(&stream.strf) % 2 == 1 { AviEncodePhase::StreamFormatPad } else { AviEncodePhase::StreamFormatDone }
                } else {
                    AviEncodePhase::StreamFormatExtra
                };
                if bytes.is_empty() {
                    self.phase = next;
                    return Ok(AviEncodeAdvance::Progress);
                }
                self.emit_fixed(&bytes, maximum_bytes, next)
            }
            AviEncodePhase::StreamFormatExtra => {
                let stream = self.stream(snapshot)?;
                let bytes = stream_format_extra(&stream.strf);
                let next = if stream_format_payload_len(&stream.strf) % 2 == 1 { AviEncodePhase::StreamFormatPad } else { AviEncodePhase::StreamFormatDone };
                self.emit_borrowed(bytes, maximum_bytes, next)
            }
            AviEncodePhase::StreamFormatPad => self.emit_padding(maximum_bytes, AviEncodePhase::StreamFormatDone),
            AviEncodePhase::StreamFormatDone => {
                self.phase = self.phase_after_stream_format(snapshot)?;
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::StreamExtraHeader => {
                let item = self.stream_extra(snapshot)?;
                let bytes = raw_header(item)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::StreamExtraData)
            }
            AviEncodePhase::StreamExtraData => {
                let item = self.stream_extra(snapshot)?;
                let next = if raw_payload_len(item)? % 2 == 1 { AviEncodePhase::StreamExtraPad } else { AviEncodePhase::StreamExtraDone };
                self.emit_borrowed(&item.data, maximum_bytes, next)
            }
            AviEncodePhase::StreamExtraPad => self.emit_padding(maximum_bytes, AviEncodePhase::StreamExtraDone),
            AviEncodePhase::StreamExtraDone => {
                self.phase = self.phase_after_stream_extra(snapshot)?;
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::HdrlExtraHeader => {
                let item = snapshot.hdrl_extra.get(self.item_index).ok_or("avi.encode.hdrl-extra-missing")?;
                let bytes = raw_header(item)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::HdrlExtraData)
            }
            AviEncodePhase::HdrlExtraData => {
                let item = snapshot.hdrl_extra.get(self.item_index).ok_or("avi.encode.hdrl-extra-missing")?;
                let next = if raw_payload_len(item)? % 2 == 1 { AviEncodePhase::HdrlExtraPad } else { AviEncodePhase::HdrlExtraDone };
                self.emit_borrowed(&item.data, maximum_bytes, next)
            }
            AviEncodePhase::HdrlExtraPad => self.emit_padding(maximum_bytes, AviEncodePhase::HdrlExtraDone),
            AviEncodePhase::HdrlExtraDone => {
                self.phase = self.phase_after_hdrl_extra(snapshot);
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::MoviHeader => {
                self.stream_index = 0;
                self.item_index = 0;
                let bytes = list_header(b"movi", self.movi_children_bytes)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::MoviSeek)
            }
            AviEncodePhase::MoviSeek => {
                self.phase = self.seek_movi(snapshot, false);
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::MoviChunkHeader => {
                let chunk = self.movi_chunk(snapshot)?;
                let bytes = chunk_header(&fourcc4(&chunk.fourcc), chunk.data.len())?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::MoviChunkData)
            }
            AviEncodePhase::MoviChunkData => {
                let chunk = self.movi_chunk(snapshot)?;
                let next = if chunk.data.len() % 2 == 1 { AviEncodePhase::MoviChunkPad } else { AviEncodePhase::MoviChunkDone };
                self.emit_borrowed(&chunk.data, maximum_bytes, next)
            }
            AviEncodePhase::MoviChunkPad => self.emit_padding(maximum_bytes, AviEncodePhase::MoviChunkDone),
            AviEncodePhase::MoviChunkDone => {
                self.item_index += 1;
                self.phase = AviEncodePhase::MoviSeek;
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::Idx1Header => {
                let bytes = chunk_header(b"idx1", self.total_chunks.checked_mul(16).ok_or("avi.encode.idx1-size-overflow")?)?;
                self.idx1_offset = 4;
                self.stream_index = 0;
                self.item_index = 0;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::Idx1Seek)
            }
            AviEncodePhase::Idx1Seek => {
                self.phase = self.seek_movi(snapshot, true);
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::Idx1Entry => {
                let chunk = self.movi_chunk(snapshot)?;
                let mut bytes = Vec::with_capacity(16);
                bytes.extend_from_slice(&fourcc4(&chunk.fourcc));
                bytes.extend_from_slice(&(if chunk.keyframe { 0x10u32 } else { 0 }).to_le_bytes());
                bytes.extend_from_slice(&self.idx1_offset.to_le_bytes());
                bytes.extend_from_slice(&u32::try_from(chunk.data.len()).map_err(|_| "avi.encode.chunk-too-large")?.to_le_bytes());
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::Idx1EntryDone)
            }
            AviEncodePhase::Idx1EntryDone => {
                let bytes = self.movi_chunk(snapshot)?.data.len();
                let total = chunk_total(bytes)?;
                self.idx1_offset = self.idx1_offset.checked_add(u32::try_from(total).map_err(|_| "avi.encode.movi-offset-overflow")?).ok_or("avi.encode.movi-offset-overflow")?;
                self.item_index += 1;
                self.phase = AviEncodePhase::Idx1Seek;
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::UnknownSeek => {
                self.phase = match snapshot.unknown_chunks.get(self.item_index) {
                    Some(item) if item.fourcc.starts_with("movi:") => {
                        self.item_index += 1;
                        AviEncodePhase::UnknownSeek
                    }
                    Some(_) => AviEncodePhase::UnknownHeader,
                    None => AviEncodePhase::Complete,
                };
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::UnknownHeader => {
                let item = self.unknown(snapshot)?;
                let bytes = raw_header(item)?;
                self.emit_fixed(&bytes, maximum_bytes, AviEncodePhase::UnknownData)
            }
            AviEncodePhase::UnknownData => {
                let item = self.unknown(snapshot)?;
                let next = if raw_payload_len(item)? % 2 == 1 { AviEncodePhase::UnknownPad } else { AviEncodePhase::UnknownDone };
                self.emit_borrowed(&item.data, maximum_bytes, next)
            }
            AviEncodePhase::UnknownPad => self.emit_padding(maximum_bytes, AviEncodePhase::UnknownDone),
            AviEncodePhase::UnknownDone => {
                self.item_index += 1;
                self.phase = AviEncodePhase::UnknownSeek;
                Ok(AviEncodeAdvance::Progress)
            }
            AviEncodePhase::Complete => Ok(AviEncodeAdvance::Complete),
        }
    }

    fn measure_stream(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodeAdvance, String> {
        let Some(stream) = snapshot.streams.get(self.stream_index) else {
            self.phase = AviEncodePhase::MeasureHdrlExtras;
            self.item_index = 0;
            return Ok(AviEncodeAdvance::Progress);
        };
        self.current_stream_children_bytes = chunk_total(stream_header_payload_len(stream)?)?.checked_add(chunk_total(stream_format_payload_len(&stream.strf))?).ok_or("avi.encode.strl-size-overflow")?;
        self.item_index = 0;
        self.phase = AviEncodePhase::MeasureStreamExtras;
        Ok(AviEncodeAdvance::Progress)
    }

    fn measure_stream_extra(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodeAdvance, String> {
        let stream = self.stream(snapshot)?;
        if let Some(item) = stream.strl_extra.get(self.item_index) {
            self.current_stream_children_bytes = self.current_stream_children_bytes.checked_add(raw_total(item)?).ok_or("avi.encode.strl-size-overflow")?;
            self.item_index += 1;
            return Ok(AviEncodeAdvance::Progress);
        }
        self.hdrl_children_bytes = self.hdrl_children_bytes.checked_add(list_total(self.current_stream_children_bytes)?).ok_or("avi.encode.hdrl-size-overflow")?;
        self.stream_index += 1;
        self.phase = AviEncodePhase::MeasureStreams;
        Ok(AviEncodeAdvance::Progress)
    }

    fn measure_hdrl_extra(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodeAdvance, String> {
        if let Some(item) = snapshot.hdrl_extra.get(self.item_index) {
            self.hdrl_children_bytes = self.hdrl_children_bytes.checked_add(raw_total(item)?).ok_or("avi.encode.hdrl-size-overflow")?;
            self.item_index += 1;
            return Ok(AviEncodeAdvance::Progress);
        }
        self.phase = AviEncodePhase::MeasureMovi;
        self.stream_index = 0;
        self.item_index = 0;
        Ok(AviEncodeAdvance::Progress)
    }

    fn measure_movi(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodeAdvance, String> {
        let Some(stream) = snapshot.streams.get(self.stream_index) else {
            self.phase = AviEncodePhase::MeasureUnknown;
            self.item_index = 0;
            return Ok(AviEncodeAdvance::Progress);
        };
        if let Some(chunk) = stream.chunks.get(self.item_index) {
            self.movi_children_bytes = self.movi_children_bytes.checked_add(chunk_total(chunk.data.len())?).ok_or("avi.encode.movi-size-overflow")?;
            self.total_chunks = self.total_chunks.checked_add(1).ok_or("avi.encode.chunk-count-overflow")?;
            self.item_index += 1;
            return Ok(AviEncodeAdvance::Progress);
        }
        self.stream_index += 1;
        self.item_index = 0;
        Ok(AviEncodeAdvance::Progress)
    }

    fn measure_unknown(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodeAdvance, String> {
        if let Some(item) = snapshot.unknown_chunks.get(self.item_index) {
            if !item.fourcc.starts_with("movi:") {
                self.unknown_bytes = self.unknown_bytes.checked_add(raw_total(item)?).ok_or("avi.encode.unknown-size-overflow")?;
            }
            self.item_index += 1;
            return Ok(AviEncodeAdvance::Progress);
        }
        self.phase = AviEncodePhase::RiffHeader;
        self.stream_index = 0;
        self.item_index = 0;
        Ok(AviEncodeAdvance::Progress)
    }

    fn measure_emitted_stream(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodeAdvance, String> {
        let Some(stream) = snapshot.streams.get(self.stream_index) else {
            self.phase = AviEncodePhase::HdrlExtraHeader;
            self.item_index = 0;
            if snapshot.hdrl_extra.is_empty() {
                self.phase = AviEncodePhase::MoviHeader;
            }
            return Ok(AviEncodeAdvance::Progress);
        };
        self.current_stream_children_bytes = chunk_total(stream_header_payload_len(stream)?)?.checked_add(chunk_total(stream_format_payload_len(&stream.strf))?).ok_or("avi.encode.strl-size-overflow")?;
        self.item_index = 0;
        self.phase = AviEncodePhase::MeasureEmittedStreamExtras;
        Ok(AviEncodeAdvance::Progress)
    }

    fn measure_emitted_stream_extra(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodeAdvance, String> {
        let stream = self.stream(snapshot)?;
        if let Some(item) = stream.strl_extra.get(self.item_index) {
            self.current_stream_children_bytes = self.current_stream_children_bytes.checked_add(raw_total(item)?).ok_or("avi.encode.strl-size-overflow")?;
            self.item_index += 1;
            return Ok(AviEncodeAdvance::Progress);
        }
        self.item_index = 0;
        self.phase = AviEncodePhase::StreamListHeader;
        Ok(AviEncodeAdvance::Progress)
    }

    fn phase_after_avih(&mut self, snapshot: &AviSnapshot) -> AviEncodePhase {
        self.stream_index = 0;
        self.item_index = 0;
        if snapshot.streams.is_empty() {
            if snapshot.hdrl_extra.is_empty() { AviEncodePhase::MoviHeader } else { AviEncodePhase::HdrlExtraHeader }
        } else {
            AviEncodePhase::MeasureEmittedStream
        }
    }

    fn phase_after_stream_header_data(&self, stream: &AviStream) -> AviEncodePhase {
        if stream_header_payload_len(stream).unwrap_or(0) % 2 == 1 { AviEncodePhase::StreamHeaderPad } else { AviEncodePhase::StreamFormatHeader }
    }

    fn phase_after_stream_format(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodePhase, String> {
        let stream = self.stream(snapshot)?;
        Ok(if stream.strl_extra.is_empty() { self.stream_index += 1; AviEncodePhase::MeasureEmittedStream } else { self.item_index = 0; AviEncodePhase::StreamExtraHeader })
    }

    fn phase_after_stream_extra(&mut self, snapshot: &AviSnapshot) -> Result<AviEncodePhase, String> {
        let stream = self.stream(snapshot)?;
        self.item_index += 1;
        if self.item_index < stream.strl_extra.len() {
            Ok(AviEncodePhase::StreamExtraHeader)
        } else {
            self.stream_index += 1;
            self.item_index = 0;
            Ok(AviEncodePhase::MeasureEmittedStream)
        }
    }

    fn phase_after_hdrl_extra(&mut self, snapshot: &AviSnapshot) -> AviEncodePhase {
        self.item_index += 1;
        if self.item_index < snapshot.hdrl_extra.len() { AviEncodePhase::HdrlExtraHeader } else { self.item_index = 0; AviEncodePhase::MoviHeader }
    }

    pub(super) fn stream<'a>(&self, snapshot: &'a AviSnapshot) -> Result<&'a AviStream, String> {
        snapshot.streams.get(self.stream_index).ok_or_else(|| "avi.encode.stream-missing".into())
    }

    fn stream_extra<'a>(&self, snapshot: &'a AviSnapshot) -> Result<&'a RiffChunk, String> {
        self.stream(snapshot)?.strl_extra.get(self.item_index).ok_or_else(|| "avi.encode.stream-extra-missing".into())
    }

    fn movi_chunk<'a>(&self, snapshot: &'a AviSnapshot) -> Result<&'a AviChunk, String> {
        snapshot.streams.get(self.stream_index).and_then(|stream| stream.chunks.get(self.item_index)).ok_or_else(|| "avi.encode.movi-chunk-missing".into())
    }

    pub(super) fn unknown<'a>(&self, snapshot: &'a AviSnapshot) -> Result<&'a RiffChunk, String> {
        snapshot.unknown_chunks.get(self.item_index).filter(|item| !item.fourcc.starts_with("movi:")).ok_or_else(|| "avi.encode.unknown-chunk-missing".into())
    }

    fn seek_movi(&mut self, snapshot: &AviSnapshot, idx1: bool) -> AviEncodePhase {
        match snapshot.streams.get(self.stream_index) {
            Some(stream) if self.item_index < stream.chunks.len() => if idx1 { AviEncodePhase::Idx1Entry } else { AviEncodePhase::MoviChunkHeader },
            Some(_) => {
                self.stream_index += 1;
                self.item_index = 0;
                if idx1 { AviEncodePhase::Idx1Seek } else { AviEncodePhase::MoviSeek }
            }
            None => {
                self.item_index = 0;
                if idx1 || !snapshot.idx1_present { AviEncodePhase::UnknownSeek } else { AviEncodePhase::Idx1Header }
            }
        }
    }

    fn emit_fixed(&mut self, bytes: &[u8], maximum_bytes: usize, next: AviEncodePhase) -> Result<AviEncodeAdvance, String> {
        self.emit_borrowed(bytes, maximum_bytes, next)
    }

    fn emit_borrowed(&mut self, bytes: &[u8], maximum_bytes: usize, next: AviEncodePhase) -> Result<AviEncodeAdvance, String> {
        let end = self.offset.checked_add(maximum_bytes).unwrap_or(usize::MAX).min(bytes.len());
        let chunk = bytes.get(self.offset..end).ok_or("avi.encode.source-shape-changed")?.to_vec();
        self.offset = end;
        self.emitted_bytes = self.emitted_bytes.checked_add(chunk.len() as u64).ok_or("avi.encode.emitted-size-overflow")?;
        if self.offset == bytes.len() {
            self.offset = 0;
            self.phase = next;
        }
        if chunk.is_empty() { Ok(AviEncodeAdvance::Progress) } else { Ok(AviEncodeAdvance::Chunk(chunk)) }
    }

    fn emit_padding(&mut self, maximum_bytes: usize, next: AviEncodePhase) -> Result<AviEncodeAdvance, String> {
        self.emit_fixed(&[0], maximum_bytes, next)
    }
}
//#endregion 🔖️Encode
