//! 🔄️ Scanline decoder of the pixels codec: the resumable PNG scanline decoder with its step, source and stream records. See [`super`].

use super::*;

//#region PngScanlineDecoder
/// ⏱️ Incremental PNG decoder with a bounded cost per call, for callers that must stay
/// interaction-friendly (progress/cancellation, a fixed worker-step ceiling) while decoding a
/// still image. For non-interlaced images nothing is eager: construction only walks the chunk
/// headers and verifies the (small) chunks before the first `IDAT`; each [`Self::step`] then
/// inflates at most its unit budget of zlib output bytes straight from the `IDAT` chunks —
/// verifying every chunk CRC as its last byte is consumed and the zlib Adler-32 at the end — and
/// yields a row as soon as one whole filtered scanline has been inflated, unfiltered and
/// canonicalized to RGBA8. Interlaced images decode all Adam7 passes eagerly on construction (rare
/// in practice for camera/photogrammetry input) and then simply drain a pre-built row queue.
pub struct PngScanlineDecoder {
    #[doc(hidden)] pub width: u32,
    #[doc(hidden)] pub height: u32,
    pub(super) rows: PngScanlineSource,
}

/// 👣️ Outcome of one bounded [`PngScanlineDecoder::step`].
#[derive(Debug, PartialEq, Eq)]
pub enum PngScanlineStep {
    /// ⏳️ The unit budget ran out before the next scanline was complete.
    Pending,
    /// 🧾️ One canonical RGBA8 scanline (`width * 4` bytes), top to bottom.
    Row(Vec<u8>),
    /// 🌊️ Every scanline has been yielded and the stream's checksums verified.
    Done,
}

enum PngScanlineSource {
    Streamed(Box<PngScanlineStream>),
    Queued(std::collections::VecDeque<Vec<u8>>),
}

struct PngScanlineStream {
    pub data: Vec<u8>,
    pub(super) ihdr: Ihdr,
    pub(super) palette: Vec<[u8; 3]>,
    pub(super) palette_alpha: Vec<u8>,
    pub(super) gray_trans: Option<u32>,
    pub(super) rgb_trans: Option<(u32, u32, u32)>,
    /// 🖱️ Next unread byte of `data` inside the current `IDAT` chunk.
    pub(super) cursor: usize,
    /// 🚰️ End (exclusive) of the current `IDAT` chunk's data.
    chunk_end: usize,
    /// 💧️ Running CRC register of the current `IDAT` chunk (type + data so far).
    chunk_crc: u32,
    /// ⏪️ Offset of the first chunk after the `IDAT` run.
    after_idat: usize,
    /// 🗜️ Zlib stream bytes (all `IDAT` data) not consumed yet.
    zlib_remaining: usize,
    pub(super) inflater: semio_framework_deflate::Inflater,
    pub(super) pending: Option<u8>,
    inflate_done: bool,
    adler: (u32, u32),
    pub(super) row: Vec<u8>,
    pub(super) prev: Option<Vec<u8>>,
    pub(super) row_bytes: usize,
    pub(super) bpp: usize,
    next_y: u32,
}

impl PngScanlineStream {
    /// 📥️ Next zlib byte from the `IDAT` run, verifying each chunk's CRC as it is crossed.
    fn next_zlib_byte(&mut self) -> Result<Option<u8>, String> {
        if self.zlib_remaining == 0 {
            return Ok(None);
        }
        while self.cursor == self.chunk_end {
            let stored = u32::from_be_bytes([self.data[self.chunk_end], self.data[self.chunk_end + 1], self.data[self.chunk_end + 2], self.data[self.chunk_end + 3]]);
            if self.chunk_crc ^ 0xFFFF_FFFF != stored {
                return Err("png: chunk CRC mismatch (IDAT)".into());
            }
            let header = self.chunk_end + 4;
            let len = u32::from_be_bytes([self.data[header], self.data[header + 1], self.data[header + 2], self.data[header + 3]]) as usize;
            self.chunk_crc = crc32_update(0xFFFF_FFFF, &self.data[header + 4..header + 8]);
            self.cursor = header + 8;
            self.chunk_end = self.cursor + len;
        }
        let byte = self.data[self.cursor];
        self.chunk_crc = crc32_update(self.chunk_crc, &[byte]);
        self.cursor += 1;
        self.zlib_remaining -= 1;
        Ok(Some(byte))
    }

    /// 🏁️ After the last scanline: drains the deflate stream, checks the Adler-32 trailer, the
    /// last `IDAT` CRC and every chunk after the run through `IEND`.
    pub(super) fn finish(&mut self, budget: &mut usize) -> Result<bool, String> {
        while !self.inflate_done {
            if *budget == 0 {
                return Ok(false);
            }
            *budget -= 1;
            if self.pending.is_none() && self.zlib_remaining > 4 {
                self.pending = self.next_zlib_byte()?;
            }
            let input_complete = self.pending.is_none() && self.zlib_remaining <= 4;
            match self.inflater.advance(&mut self.pending, input_complete).map_err(|error| format!("png: inflate: {error:?}"))? {
                semio_framework_deflate::InflateOutcome::NeedInput => {}
                semio_framework_deflate::InflateOutcome::Wrote(byte) => self.adler_push(byte),
                semio_framework_deflate::InflateOutcome::Done => self.inflate_done = true,
            }
        }
        if self.zlib_remaining != 4 || self.pending.is_some() {
            return Err("png: zlib stream length does not match its deflate data".into());
        }
        let mut trailer = [0u8; 4];
        for slot in &mut trailer {
            *slot = self.next_zlib_byte()?.ok_or("png: truncated zlib trailer")?;
        }
        if u32::from_be_bytes(trailer) != (self.adler.1 << 16) | self.adler.0 {
            return Err("png: zlib adler32 mismatch".into());
        }
        let stored = u32::from_be_bytes([self.data[self.chunk_end], self.data[self.chunk_end + 1], self.data[self.chunk_end + 2], self.data[self.chunk_end + 3]]);
        if self.cursor != self.chunk_end || self.chunk_crc ^ 0xFFFF_FFFF != stored {
            return Err("png: chunk CRC mismatch (IDAT)".into());
        }
        read_chunks_from(&self.data, self.after_idat)?;
        Ok(true)
    }

    fn adler_push(&mut self, byte: u8) {
        self.adler.0 = (self.adler.0 + byte as u32) % 65_521;
        self.adler.1 = (self.adler.1 + self.adler.0) % 65_521;
    }
}

impl PngScanlineDecoder {
    /// 🌱️ Copies `data` and starts a decoder over it (see [`Self::from_vec`]).
    pub fn new(data: &[u8]) -> Result<Self, RasterError> {
        Self::from_vec(data.to_vec())
    }

    /// ✅️ Walks the chunk headers, verifies and parses IHDR/PLTE/tRNS, and positions the stream at
    /// the first `IDAT` byte. Only an interlaced image is decoded here, eagerly.
    pub fn from_vec(data: Vec<u8>) -> Result<Self, RasterError> {
        if data.len() < 8 || data[0..8] != PNG_SIGNATURE {
            return Err(RasterError::Codec("png: bad signature".into()));
        }
        let mut ihdr: Option<Ihdr> = None;
        let mut palette: Vec<[u8; 3]> = Vec::new();
        let mut palette_alpha: Vec<u8> = Vec::new();
        let mut gray_trans: Option<u32> = None;
        let mut rgb_trans: Option<(u32, u32, u32)> = None;
        let mut first_idat: Option<usize> = None;
        let mut after_idat: Option<usize> = None;
        let mut zlib_len = 0usize;
        let mut pos = 8usize;
        loop {
            if pos + 8 > data.len() {
                return Err(RasterError::Codec("png: truncated chunk header".into()));
            }
            let len = u32::from_be_bytes([data[pos], data[pos + 1], data[pos + 2], data[pos + 3]]) as usize;
            let ty: [u8; 4] = [data[pos + 4], data[pos + 5], data[pos + 6], data[pos + 7]];
            let start = pos + 8;
            let end = start.checked_add(len).ok_or_else(|| RasterError::Codec("png: chunk length overflow".into()))?;
            if end.checked_add(4).is_none_or(|crc_end| crc_end > data.len()) {
                return Err(RasterError::Codec("png: truncated chunk data or crc".into()));
            }
            if ty == *b"IDAT" {
                if after_idat.is_some() {
                    return Err(RasterError::Codec("png: IDAT chunks are not consecutive".into()));
                }
                first_idat.get_or_insert(pos);
                zlib_len += len;
            } else {
                if first_idat.is_some() && after_idat.is_none() {
                    after_idat = Some(pos);
                }
                if first_idat.is_none() {
                    let chunk = &data[start..end];
                    let stored = u32::from_be_bytes([data[end], data[end + 1], data[end + 2], data[end + 3]]);
                    if crc32_update(crc32_update(0xFFFF_FFFF, &ty), chunk) ^ 0xFFFF_FFFF != stored {
                        return Err(RasterError::Codec(format!("png: chunk CRC mismatch ({})", String::from_utf8_lossy(&ty))));
                    }
                    if ty == *b"IHDR" {
                        ihdr = Some(parse_ihdr(chunk).map_err(RasterError::Codec)?);
                    } else if ty == *b"PLTE" {
                        palette = chunk.as_chunks::<3>().0.to_vec();
                    } else if ty == *b"tRNS" {
                        let color_type = ihdr.as_ref().ok_or_else(|| RasterError::Codec("png: tRNS before IHDR".into()))?.color_type;
                        match color_type {
                            0 if chunk.len() == 2 => gray_trans = Some(u16::from_be_bytes([chunk[0], chunk[1]]) as u32),
                            2 if chunk.len() == 6 => {
                                let r = u16::from_be_bytes([chunk[0], chunk[1]]) as u32;
                                let g = u16::from_be_bytes([chunk[2], chunk[3]]) as u32;
                                let b = u16::from_be_bytes([chunk[4], chunk[5]]) as u32;
                                rgb_trans = Some((r, g, b));
                            }
                            3 => palette_alpha = chunk.to_vec(),
                            _ => {}
                        }
                    }
                }
            }
            pos = end + 4;
            if ty == *b"IEND" {
                break;
            }
            if pos >= data.len() {
                return Err(RasterError::Codec("png: missing IEND".into()));
            }
        }
        let ihdr = ihdr.ok_or_else(|| RasterError::Codec("png: missing IHDR".into()))?;
        let (Some(first_idat), Some(after_idat)) = (first_idat, after_idat) else { return Err(RasterError::Codec("png: missing IDAT".into())) };
        if ihdr.color_type == 3 && palette.is_empty() {
            return Err(RasterError::Codec("png: color type 3 requires PLTE".into()));
        }
        let width = ihdr.width;
        let height = ihdr.height;
        if ihdr.interlace != 0 {
            return Self::interlaced(&data, ihdr, &palette, &palette_alpha, gray_trans, rgb_trans);
        }
        if zlib_len < 6 {
            return Err(RasterError::Codec("png: zlib stream too short".into()));
        }
        let row_bytes = packed_row_bytes(width, ihdr.color_type, ihdr.bit_depth);
        let bpp = bpp_bytes(&ihdr);
        let chunk_start = first_idat + 8;
        let first_len = u32::from_be_bytes([data[first_idat], data[first_idat + 1], data[first_idat + 2], data[first_idat + 3]]) as usize;
        let mut stream = PngScanlineStream {
            chunk_crc: crc32_update(0xFFFF_FFFF, b"IDAT"),
            data,
            ihdr,
            palette,
            palette_alpha,
            gray_trans,
            rgb_trans,
            cursor: chunk_start,
            chunk_end: chunk_start + first_len,
            after_idat,
            zlib_remaining: zlib_len,
            inflater: semio_framework_deflate::Inflater::new(),
            pending: None,
            inflate_done: false,
            adler: (1, 0),
            row: Vec::with_capacity(row_bytes + 1),
            prev: None,
            row_bytes,
            bpp,
            next_y: 0,
        };
        let cmf = stream.next_zlib_byte().map_err(RasterError::Codec)?.ok_or_else(|| RasterError::Codec("zlib stream too short".into()))?;
        let flg = stream.next_zlib_byte().map_err(RasterError::Codec)?.ok_or_else(|| RasterError::Codec("zlib stream too short".into()))?;
        if (cmf & 0x0F) != 8 {
            return Err(RasterError::Codec("unsupported zlib compression method".into()));
        }
        if !((cmf as u16) * 256 + flg as u16).is_multiple_of(31) {
            return Err(RasterError::Codec("zlib CMF/FLG check failed".into()));
        }
        if flg & 0x20 != 0 {
            return Err(RasterError::Codec("zlib preset dictionary not supported".into()));
        }
        Ok(Self { width, height, rows: PngScanlineSource::Streamed(Box::new(stream)) })
    }

    fn interlaced(data: &[u8], ihdr: Ihdr, palette: &[[u8; 3]], palette_alpha: &[u8], gray_trans: Option<u32>, rgb_trans: Option<(u32, u32, u32)>) -> Result<Self, RasterError> {
        let chunks = read_chunks(data).map_err(RasterError::Codec)?;
        let mut idat = Vec::new();
        for &(ty, chunk) in &chunks {
            if ty == *b"IDAT" {
                idat.extend_from_slice(chunk);
            }
        }
        let raw = deflate::zlib_decompress(&idat).map_err(RasterError::Codec)?;
        let (width, height) = (ihdr.width, ihdr.height);
        let spp = samples_per_pixel(ihdr.color_type);
        let bpp = bpp_bytes(&ihdr);
        let mut rgba_rows: Vec<Vec<u8>> = (0..height).map(|_| vec![0u8; width as usize * 4]).collect();
        let mut pos = 0usize;
        for (pass, &(sx, sy, stx, sty)) in ADAM7.iter().enumerate() {
            let (pw, ph) = adam7_pass_dims(width, height, pass);
            if pw == 0 || ph == 0 {
                continue;
            }
            let row_bytes = packed_row_bytes(pw, ihdr.color_type, ihdr.bit_depth);
            let (rows, new_pos) = defilter_pass(&raw, pos, ph, row_bytes, bpp).map_err(RasterError::Codec)?;
            pos = new_pos;
            for (j, row) in rows.iter().enumerate() {
                let samples = unpack_samples(row, pw as usize, spp, ihdr.bit_depth);
                let y = sy + j as u32 * sty;
                for i in 0..pw as usize {
                    let px = pixel_to_rgba(&samples[i * spp..i * spp + spp], &ihdr, palette, palette_alpha, gray_trans, rgb_trans).map_err(RasterError::Codec)?;
                    let x = (sx + i as u32 * stx) as usize;
                    rgba_rows[y as usize][x * 4..x * 4 + 4].copy_from_slice(&px);
                }
            }
        }
        Ok(Self { width, height, rows: PngScanlineSource::Queued(rgba_rows.into()) })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    /// ⏭️ Advances by exactly one scanline, returning canonical RGBA8 bytes (`width * 4`), or
    /// `None` once every row has been yielded and the stream verified.
    pub fn next_row(&mut self) -> Result<Option<Vec<u8>>, RasterError> {
        loop {
            match self.step(usize::MAX)? {
                PngScanlineStep::Pending => {}
                PngScanlineStep::Row(row) => return Ok(Some(row)),
                PngScanlineStep::Done => return Ok(None),
            }
        }
    }

    /// 🪜️ One bounded unit of decoding: at most `unit_budget` inflater advances (each consumes at
    /// most one compressed byte and produces at most one filtered-scanline byte), plus the
    /// unfilter/canonicalize of the one scanline they complete.
    pub fn step(&mut self, unit_budget: usize) -> Result<PngScanlineStep, RasterError> {
        let width = self.width;
        let height = self.height;
        let stream = match &mut self.rows {
            PngScanlineSource::Queued(queue) => return Ok(queue.pop_front().map_or(PngScanlineStep::Done, PngScanlineStep::Row)),
            PngScanlineSource::Streamed(stream) => stream,
        };
        let mut budget = unit_budget.max(1);
        if stream.next_y >= height {
            return if stream.finish(&mut budget).map_err(RasterError::Codec)? { Ok(PngScanlineStep::Done) } else { Ok(PngScanlineStep::Pending) };
        }
        while stream.row.len() < stream.row_bytes + 1 {
            if budget == 0 {
                return Ok(PngScanlineStep::Pending);
            }
            budget -= 1;
            if stream.pending.is_none() && stream.zlib_remaining > 4 {
                stream.pending = stream.next_zlib_byte().map_err(RasterError::Codec)?;
            }
            let input_complete = stream.pending.is_none() && stream.zlib_remaining <= 4;
            match stream.inflater.advance(&mut stream.pending, input_complete).map_err(|error| RasterError::Codec(format!("png: inflate: {error:?}")))? {
                semio_framework_deflate::InflateOutcome::NeedInput => {}
                semio_framework_deflate::InflateOutcome::Wrote(byte) => {
                    stream.adler_push(byte);
                    stream.row.push(byte);
                }
                semio_framework_deflate::InflateOutcome::Done => return Err(RasterError::Codec("png: truncated scanline data".into())),
            }
        }
        let ft = stream.row[0];
        let recon = defilter_row(ft, &stream.row[1..], stream.prev.as_deref(), stream.bpp).map_err(RasterError::Codec)?;
        stream.row.clear();
        let spp = samples_per_pixel(stream.ihdr.color_type);
        let samples = unpack_samples(&recon, width as usize, spp, stream.ihdr.bit_depth);
        let mut row = vec![0u8; width as usize * 4];
        for i in 0..width as usize {
            let px = pixel_to_rgba(&samples[i * spp..i * spp + spp], &stream.ihdr, &stream.palette, &stream.palette_alpha, stream.gray_trans, stream.rgb_trans).map_err(RasterError::Codec)?;
            row[i * 4..i * 4 + 4].copy_from_slice(&px);
        }
        stream.prev = Some(recon);
        stream.next_y += 1;
        Ok(PngScanlineStep::Row(row))
    }
}
//#endregion PngScanlineDecoder
