//! 🖼️ Image containers and classical image processing: grayscale pyramids, gradients, filtering, patch correlation and PNG codec behind an interface.

// #region 🔖️Types
/// 🌫️ Row-major single-channel image with luma values in `[0, 1]`; pixel `(x, y)` lives at `data[y * width + x]`.
#[derive(Clone, Debug, PartialEq)]
pub struct ImageGray {
    pub width: u32,
    pub height: u32,
    pub data: Vec<f32>,
}

impl ImageGray {
    /// 🌫️ Zero-filled grayscale image of the given size.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, data: vec![0.0; (width as usize) * (height as usize)] }
    }

    /// 🔍️ Pixel value at integer coordinates `(x, y)`.
    pub fn get(&self, x: u32, y: u32) -> f32 {
        self.data[(y * self.width + x) as usize]
    }

    /// ✏️ Writes the pixel value at integer coordinates `(x, y)`.
    pub fn set(&mut self, x: u32, y: u32, value: f32) {
        self.data[(y * self.width + x) as usize] = value;
    }

    /// 🎚️ Grayscale conversion of an RGBA image via BT.601 luma weights `0.299 R + 0.587 G + 0.114 B`, normalized to `[0, 1]`.
    /// <https://en.wikipedia.org/wiki/Rec._601>
    pub fn from_rgba8_luma(src: &ImageRgba8) -> Self {
        let mut out = Self::new(src.width, src.height);
        for (dst, px) in out.data.iter_mut().zip(src.data.as_chunks::<4>().0.iter()) {
            *dst = (0.299 * f32::from(px[0]) + 0.587 * f32::from(px[1]) + 0.114 * f32::from(px[2])) / 255.0;
        }
        out
    }

    /// 📐️ Bilinear sample at subpixel `(x, y)`; coordinates are clamped to the image border, and an empty image samples to `0`.
    pub fn sample(&self, x: f32, y: f32) -> f32 {
        if self.width == 0 || self.height == 0 {
            return 0.0;
        }
        let xf = x.clamp(0.0, (self.width - 1) as f32);
        let yf = y.clamp(0.0, (self.height - 1) as f32);
        let x0 = xf.floor() as u32;
        let y0 = yf.floor() as u32;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);
        let fx = xf - x0 as f32;
        let fy = yf - y0 as f32;
        let top = self.get(x0, y0) * (1.0 - fx) + self.get(x1, y0) * fx;
        let bottom = self.get(x0, y1) * (1.0 - fx) + self.get(x1, y1) * fx;
        top * (1.0 - fy) + bottom * fy
    }
}

/// 🎨️ Row-major 8-bit RGBA image with interleaved channels; pixel `(x, y)` occupies `data[(y * width + x) * 4 ..][..4]`.
#[derive(Clone, Debug, PartialEq)]
pub struct ImageRgba8 {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

impl ImageRgba8 {
    /// 🎨️ Zero-filled (transparent black) RGBA image of the given size.
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height, data: vec![0; (width as usize) * (height as usize) * 4] }
    }

    /// 🔍️ RGBA pixel at integer coordinates `(x, y)`.
    pub fn get_rgba(&self, x: u32, y: u32) -> [u8; 4] {
        let idx = ((y * self.width + x) * 4) as usize;
        [self.data[idx], self.data[idx + 1], self.data[idx + 2], self.data[idx + 3]]
    }

    /// 📐️ Bilinear RGB sample at subpixel `(x, y)`, normalized to `[0, 1]` per channel; coordinates are clamped to the image border, and an empty image samples to black.
    pub fn sample_rgb(&self, x: f32, y: f32) -> [f32; 3] {
        if self.width == 0 || self.height == 0 {
            return [0.0; 3];
        }
        let xf = x.clamp(0.0, (self.width - 1) as f32);
        let yf = y.clamp(0.0, (self.height - 1) as f32);
        let x0 = xf.floor() as u32;
        let y0 = yf.floor() as u32;
        let x1 = (x0 + 1).min(self.width - 1);
        let y1 = (y0 + 1).min(self.height - 1);
        let fx = xf - x0 as f32;
        let fy = yf - y0 as f32;
        let p00 = self.get_rgba(x0, y0);
        let p10 = self.get_rgba(x1, y0);
        let p01 = self.get_rgba(x0, y1);
        let p11 = self.get_rgba(x1, y1);
        let mut out = [0.0f32; 3];
        for (channel, slot) in out.iter_mut().enumerate() {
            let top = f32::from(p00[channel]) * (1.0 - fx) + f32::from(p10[channel]) * fx;
            let bottom = f32::from(p01[channel]) * (1.0 - fx) + f32::from(p11[channel]) * fx;
            *slot = (top * (1.0 - fy) + bottom * fy) / 255.0;
        }
        out
    }
}
// #endregion 🔖️Types

// #region 🔖️Codec
/// ⚠️ Error type for image codec operations (PNG, JPEG); `Dimensions` signals a size/buffer mismatch before any encoding is attempted, and `UnsupportedJpeg` flags progressive/arithmetic/non-baseline JPEG variants stdio's shared `jpg::engine` codec deliberately does not attempt.
#[derive(Clone, Debug, PartialEq)]
pub enum ImageError {
    Decode(String),
    Encode(String),
    Dimensions,
    UnsupportedJpeg(String),
}

impl std::fmt::Display for ImageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Decode(msg) => write!(f, "image decode failed: {msg}"),
            Self::Encode(msg) => write!(f, "image encode failed: {msg}"),
            Self::Dimensions => write!(f, "image dimensions do not match the pixel buffer"),
            Self::UnsupportedJpeg(msg) => write!(f, "unsupported jpeg variant: {msg}"),
        }
    }
}

impl std::error::Error for ImageError {}

// #region 🔖️PngViaStdio
/// 📥️ Decodes a PNG byte stream into RGBA by calling stdio's real `png::engine::decode_png`
/// in-process (`semio-s-plugin-remodeling` already depends on `semio-s-plugin-stdio` — same crate
/// family, no wasm/IPC) instead of a plugin-local codec. Extraction ticket
/// `26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT`, W5a: replaces the
/// former `png` crate (external-library) decode path — the photogrammetry pipeline only ever
/// needs a flat RGBA raster here, not a document-level `semio/image` snapshot, so the direct
/// same-process stdio call is the simpler of the two extraction shapes the ticket allows.
/// <https://www.w3.org/TR/png-3/>
pub fn decode_png(bytes: &[u8]) -> Result<ImageRgba8, ImageError> {
    let snapshot = semio_s_plugin_stdio::artifacts::png::io::decode_png(bytes).map_err(ImageError::Decode)?;
    Ok(ImageRgba8 { width: snapshot.width, height: snapshot.height, data: snapshot.pixels })
}

/// 📤️ Encodes an RGBA image as an 8-bit RGBA PNG byte stream via stdio's real
/// `png::engine::encode_png` (see `decode_png` above for the extraction rationale).
pub fn encode_png(img: &ImageRgba8) -> Result<Vec<u8>, ImageError> {
    let expected_len = (img.width as usize) * (img.height as usize) * 4;
    if img.width == 0 || img.height == 0 || img.data.len() != expected_len {
        return Err(ImageError::Dimensions);
    }
    let snapshot = semio_s_plugin_stdio::artifacts::png::PngSnapshot { width: img.width, height: img.height, pixels: img.data.clone(), ..Default::default() };
    semio_s_plugin_stdio::artifacts::png::io::encode_png(&snapshot).map_err(ImageError::Encode)
}

/// 📤️ Encodes row-major 16-bit grayscale samples as a 16-bit grayscale PNG byte stream
/// (big-endian per the PNG spec), for lossless DSM/heightfield export.
///
/// 🕳️ **stdio gap** (reported in W5a's `stdio_gaps`): stdio's `png::engine::encode_png` always
/// canonicalizes the pixel payload to 8-bit RGBA / color type 6 regardless of the snapshot's own
/// `bit_depth`/`color_type` fields (see that function's own `EncodeScopeNote`) — it has no 16-bit
/// grayscale encode path. `semio-framework-pixels::encode_png_gray16` (framework tier, no
/// third-party dependency) fills that gap directly.
pub fn encode_png_gray16(data: &[u16], width: u32, height: u32) -> Result<Vec<u8>, ImageError> {
    if width == 0 || height == 0 || data.len() != (width as usize) * (height as usize) {
        return Err(ImageError::Dimensions);
    }
    semio_framework_pixels::encode_png_gray16(width, height, data).map_err(|e| ImageError::Encode(e.to_string()))
}
// #endregion 🔖️PngViaStdio

// #region 🔖️JpegViaStdio
/// 📥️ Decodes a JFIF/JPEG byte stream into RGBA by calling stdio's real
/// `jpg::engine::decode_jpg` in-process (same extraction rationale as `decode_png` above).
/// Baseline sequential only — progressive/arithmetic/lossless SOFn variants surface as
/// `ImageError::UnsupportedJpeg`, matching this function's pre-extraction contract.
pub fn decode_jpeg(bytes: &[u8]) -> Result<ImageRgba8, ImageError> {
    let snapshot = semio_s_plugin_stdio::artifacts::jpg::engine::decode_jpg(bytes).map_err(|error| match error {
        semio_s_plugin_stdio::artifacts::jpg::engine::JpgError::Unsupported(msg) => ImageError::UnsupportedJpeg(msg),
        semio_s_plugin_stdio::artifacts::jpg::engine::JpgError::Malformed(msg) => ImageError::Decode(msg),
    })?;
    Ok(ImageRgba8 { width: snapshot.width, height: snapshot.height, data: snapshot.pixels })
}

//#region 🔖️BoundedDecode
const MAX_PNG_ROW_PIXELS: u32 = 4_096;
const MAX_JPEG_COMPRESSED_BYTES: usize = 131_072;
const MAX_STILL_PIXELS: u64 = 262_144;
const COMPRESSED_ROPE_LEAF_BYTES: usize = 4_096;

#[derive(Default)]
struct CompressedRopeReadCounters {
    sequential_reads: std::sync::atomic::AtomicUsize,
    sequential_bytes: std::sync::atomic::AtomicUsize,
    largest_sequential_read: std::sync::atomic::AtomicUsize,
    random_byte_reads: std::sync::atomic::AtomicUsize,
}

#[cfg(test)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CompressedRopeReadMetrics {
    pub sequential_reads: usize,
    pub sequential_bytes: usize,
    pub largest_sequential_read: usize,
    pub random_byte_reads: usize,
    pub largest_random_read: usize,
}

/// 🧩️ Persistent bounded compressed-input rope with independently shared leaves.
#[derive(Clone, Default)]
pub struct CompressedChunkRope {
    chunks: Vec<std::sync::Arc<[u8]>>,
    len: usize,
    reads: std::sync::Arc<CompressedRopeReadCounters>,
}

impl CompressedChunkRope {
    pub fn from_leaves(leaves: impl IntoIterator<Item = std::sync::Arc<[u8]>>, max_bytes: usize) -> Result<Self, ImageError> {
        let mut rope = Self::default();
        for leaf in leaves {
            rope.push(leaf, max_bytes)?;
        }
        if rope.chunks.is_empty() {
            return Err(ImageError::Decode("compressed input has no leaves".into()));
        }
        Ok(rope)
    }

    pub fn push(&mut self, bytes: impl Into<std::sync::Arc<[u8]>>, max_bytes: usize) -> Result<(), ImageError> {
        let bytes = bytes.into();
        if bytes.is_empty() || bytes.len() > COMPRESSED_ROPE_LEAF_BYTES {
            return Err(ImageError::Decode("compressed input leaf exceeds 4 KiB".into()));
        }
        let next = self.len.checked_add(bytes.len()).ok_or_else(|| ImageError::Decode("compressed input length overflow".into()))?;
        if next > max_bytes {
            return Err(ImageError::Decode("compressed input exceeds its bounded envelope".into()));
        }
        self.len = next;
        self.chunks.push(bytes);
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.len
    }

    fn byte(&self, index: usize) -> Option<u8> {
        self.reads.random_byte_reads.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        if index >= self.len {
            return None;
        }
        let mut remaining = index;
        for chunk in &self.chunks {
            if remaining < chunk.len() {
                return chunk.get(remaining).copied();
            }
            remaining = remaining.checked_sub(chunk.len())?;
        }
        None
    }

    #[cfg(test)]
    pub fn leaf_lengths(&self) -> Vec<usize> {
        self.chunks.iter().map(|chunk| chunk.len()).collect()
    }

    #[cfg(test)]
    pub fn read_metrics(&self) -> CompressedRopeReadMetrics {
        let random_byte_reads = self.reads.random_byte_reads.load(std::sync::atomic::Ordering::Relaxed);
        CompressedRopeReadMetrics {
            sequential_reads: self.reads.sequential_reads.load(std::sync::atomic::Ordering::Relaxed),
            sequential_bytes: self.reads.sequential_bytes.load(std::sync::atomic::Ordering::Relaxed),
            largest_sequential_read: self.reads.largest_sequential_read.load(std::sync::atomic::Ordering::Relaxed),
            random_byte_reads,
            largest_random_read: usize::from(random_byte_reads != 0),
        }
    }
}

impl semio_s_plugin_stdio::artifacts::jpg::engine::JpgByteSource for CompressedChunkRope {
    fn len(&self) -> usize {
        self.len
    }

    fn byte(&self, index: usize) -> Option<u8> {
        CompressedChunkRope::byte(self, index)
    }
}

#[derive(Clone)]
struct ChunkRopeReader {
    rope: CompressedChunkRope,
    chunk: usize,
    offset: usize,
}

impl ChunkRopeReader {
    fn new(rope: CompressedChunkRope) -> Self {
        Self { rope, chunk: 0, offset: 0 }
    }
}

impl std::io::Read for ChunkRopeReader {
    fn read(&mut self, output: &mut [u8]) -> std::io::Result<usize> {
        let mut written = 0;
        let read_limit = output.len().min(COMPRESSED_ROPE_LEAF_BYTES);
        while written < read_limit {
            let Some(chunk) = self.rope.chunks.get(self.chunk) else { break };
            if self.offset == chunk.len() {
                self.chunk = self.chunk.checked_add(1).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "compressed rope chunk cursor overflow"))?;
                self.offset = 0;
                continue;
            }
            let available = chunk.len().checked_sub(self.offset).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "compressed rope offset exceeds leaf"))?;
            let remaining = read_limit.checked_sub(written).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "compressed rope output cursor overflow"))?;
            let count = available.min(remaining);
            let output_end = written.checked_add(count).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "compressed rope output range overflow"))?;
            let input_end = self.offset.checked_add(count).ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidData, "compressed rope input range overflow"))?;
            output[written..output_end].copy_from_slice(&chunk[self.offset..input_end]);
            self.offset = input_end;
            written = output_end;
        }
        self.rope.reads.sequential_reads.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.rope.reads.sequential_bytes.fetch_add(written, std::sync::atomic::Ordering::Relaxed);
        self.rope.reads.largest_sequential_read.fetch_max(written, std::sync::atomic::Ordering::Relaxed);
        Ok(written)
    }
}

/// ⏱️ Outcome of one owned still-image decoder microstep.
pub enum BoundedDecodeProgress {
    Working,
    Complete(ImageRgba8),
    Failed(ImageError),
}

enum BoundedDecodeState {
    Probe { mime: String, rope: CompressedChunkRope },
    PngRead { reader: ChunkRopeReader, buffer: Vec<u8> },
    PngDecode { buffer: Vec<u8> },
    PngRows { decoder: semio_framework_pixels::PngScanlineDecoder, width: u32, height: u32, pixels: Vec<u8> },
    JpegProbe { rope: CompressedChunkRope, cursor: usize },
    Jpeg { rope: CompressedChunkRope },
    Finished,
}

/// 🧩️ Repository-owned decoder state over a persistent 4-KiB-leaf rope. PNG consumes one
/// scanline per call. JPEG probing advances 4 KiB per call and the shared baseline codec reads the
/// same rope directly inside its fixed byte/pixel envelope, with no whole-input join allocation.
pub struct BoundedStillDecoder {
    state: BoundedDecodeState,
}

impl BoundedStillDecoder {
    pub fn new(mime: &str, rope: CompressedChunkRope) -> Self {
        Self { state: BoundedDecodeState::Probe { mime: mime.into(), rope } }
    }

    pub fn advance(&mut self) -> BoundedDecodeProgress {
        let state = std::mem::replace(&mut self.state, BoundedDecodeState::Finished);
        match state {
            BoundedDecodeState::Probe { mime, rope } if mime == "image/png" => {
                if rope.len() < 24 || (0..8).any(|index| rope.byte(index) != Some(b"\x89PNG\r\n\x1a\n"[index])) {
                    return BoundedDecodeProgress::Failed(ImageError::Decode("invalid PNG header".into()));
                }
                let width = u32::from_be_bytes(std::array::from_fn(|offset| rope.byte(16 + offset).expect("probed PNG width")));
                let height = u32::from_be_bytes(std::array::from_fn(|offset| rope.byte(20 + offset).expect("probed PNG height")));
                let Some(pixels) = u64::from(width).checked_mul(u64::from(height)) else { return BoundedDecodeProgress::Failed(ImageError::Decode("PNG pixel count overflow".into())) };
                if width == 0 || height == 0 || width > MAX_PNG_ROW_PIXELS || pixels > MAX_STILL_PIXELS {
                    return BoundedDecodeProgress::Failed(ImageError::Decode(format!("PNG exceeds the bounded {MAX_PNG_ROW_PIXELS}-pixel row / {MAX_STILL_PIXELS}-pixel image envelope")));
                }
                self.state = BoundedDecodeState::PngRead { reader: ChunkRopeReader::new(rope), buffer: Vec::new() };
                BoundedDecodeProgress::Working
            }
            BoundedDecodeState::Probe { mime, rope } if mime == "image/jpeg" => {
                if rope.len() > MAX_JPEG_COMPRESSED_BYTES {
                    return BoundedDecodeProgress::Failed(ImageError::Decode(format!("JPEG exceeds the bounded {MAX_JPEG_COMPRESSED_BYTES}-byte decoder envelope")));
                }
                self.state = BoundedDecodeState::JpegProbe { rope, cursor: 0 };
                BoundedDecodeProgress::Working
            }
            BoundedDecodeState::Probe { mime, .. } => BoundedDecodeProgress::Failed(ImageError::Decode(format!("unsupported image mime {mime}"))),
            BoundedDecodeState::PngRead { mut reader, mut buffer } => {
                let mut leaf = [0u8; COMPRESSED_ROPE_LEAF_BYTES];
                match std::io::Read::read(&mut reader, &mut leaf) {
                    Ok(0) => {
                        self.state = BoundedDecodeState::PngDecode { buffer };
                        BoundedDecodeProgress::Working
                    }
                    Ok(read) => {
                        buffer.extend_from_slice(&leaf[..read]);
                        self.state = BoundedDecodeState::PngRead { reader, buffer };
                        BoundedDecodeProgress::Working
                    }
                    Err(error) => BoundedDecodeProgress::Failed(ImageError::Decode(error.to_string())),
                }
            }
            BoundedDecodeState::PngDecode { buffer } => match semio_framework_pixels::PngScanlineDecoder::new(&buffer) {
                Ok(decoder) => {
                    let width = decoder.width();
                    let height = decoder.height();
                    self.state = BoundedDecodeState::PngRows { decoder, width, height, pixels: Vec::with_capacity((width as usize) * (height as usize) * 4) };
                    BoundedDecodeProgress::Working
                }
                Err(error) => BoundedDecodeProgress::Failed(ImageError::Decode(error.to_string())),
            },
            BoundedDecodeState::PngRows { mut decoder, width, height, mut pixels } => match decoder.next_row() {
                Ok(Some(mut row)) => {
                    pixels.append(&mut row);
                    self.state = BoundedDecodeState::PngRows { decoder, width, height, pixels };
                    BoundedDecodeProgress::Working
                }
                Ok(None) => BoundedDecodeProgress::Complete(ImageRgba8 { width, height, data: pixels }),
                Err(error) => BoundedDecodeProgress::Failed(ImageError::Decode(error.to_string())),
            },
            BoundedDecodeState::JpegProbe { rope, cursor } => {
                let end = cursor.checked_add(4_096).unwrap_or(rope.len()).min(rope.len());
                let scan_start = cursor.saturating_sub(8);
                for index in scan_start..end.saturating_sub(8) {
                    if rope.byte(index) != Some(0xff) {
                        continue;
                    }
                    let marker = index.checked_add(1).and_then(|at| rope.byte(at));
                    if matches!(marker, Some(0xc1 | 0xc2 | 0xc3 | 0xc5 | 0xc6 | 0xc7 | 0xc9 | 0xca | 0xcb | 0xcd | 0xce | 0xcf)) {
                        return BoundedDecodeProgress::Failed(ImageError::UnsupportedJpeg("non-baseline JPEG".into()));
                    }
                    if marker == Some(0xc0) {
                        let byte = |offset| index.checked_add(offset).and_then(|at| rope.byte(at)).unwrap_or(0);
                        let height = u64::from(u16::from_be_bytes([byte(5), byte(6)]));
                        let width = u64::from(u16::from_be_bytes([byte(7), byte(8)]));
                        if width == 0 || height == 0 || width.checked_mul(height).is_none_or(|pixels| pixels > MAX_STILL_PIXELS) {
                            return BoundedDecodeProgress::Failed(ImageError::Decode(format!("JPEG exceeds the bounded {MAX_STILL_PIXELS}-pixel decoder envelope")));
                        }
                        self.state = BoundedDecodeState::Jpeg { rope };
                        return BoundedDecodeProgress::Working;
                    }
                }
                if end == rope.len() {
                    BoundedDecodeProgress::Failed(ImageError::Decode("JPEG has no baseline frame header".into()))
                } else {
                    self.state = BoundedDecodeState::JpegProbe { rope, cursor: end };
                    BoundedDecodeProgress::Working
                }
            }
            BoundedDecodeState::Jpeg { rope } => match semio_s_plugin_stdio::artifacts::jpg::engine::decode_jpg_source(&rope) {
                Ok(snapshot) => BoundedDecodeProgress::Complete(ImageRgba8 { width: snapshot.width, height: snapshot.height, data: snapshot.pixels }),
                Err(semio_s_plugin_stdio::artifacts::jpg::engine::JpgError::Unsupported(message)) => BoundedDecodeProgress::Failed(ImageError::UnsupportedJpeg(message)),
                Err(semio_s_plugin_stdio::artifacts::jpg::engine::JpgError::Malformed(message)) => BoundedDecodeProgress::Failed(ImageError::Decode(message)),
            },
            BoundedDecodeState::Finished => BoundedDecodeProgress::Failed(ImageError::Decode("decoder polled after completion".into())),
        }
    }
}

//#endregion 🔖️BoundedDecode

/// 📤️ Encodes an RGBA image as baseline sequential JPEG via stdio's real
/// `jpg::engine::encode_jpg`, at the given IJG-convention quality (`1..=100`). Infallible for any
/// `ImageRgba8` (its own invariants already guarantee `data.len() == width * height * 4`),
/// matching this function's pre-extraction (non-`Result`) signature.
pub fn encode_jpeg(image: &ImageRgba8, quality: u8) -> Vec<u8> {
    let snapshot = semio_s_plugin_stdio::artifacts::jpg::JpgSnapshot { width: image.width, height: image.height, pixels: image.data.clone(), re_encode_quality: Some(quality), ..Default::default() };
    semio_s_plugin_stdio::artifacts::jpg::engine::encode_jpg(&snapshot).expect("a valid ImageRgba8 always encodes")
}
// #endregion 🔖️JpegViaStdio
// #endregion 🔖️Codec

// #region 🔖️Filter
fn mirror_index(i: i64, n: i64) -> i64 {
    let mut i = i;
    loop {
        if i < 0 {
            i = -1 - i;
        } else if i >= n {
            i = 2 * n - 1 - i;
        } else {
            return i;
        }
    }
}

/// 🌀️ Separable Gaussian blur with kernel radius `ceil(3 sigma)` and half-sample mirror padding (which preserves the image mean exactly for symmetric kernels); `sigma <= 0` returns the input unchanged.
/// <https://en.wikipedia.org/wiki/Gaussian_blur>
pub fn gaussian_blur(img: &ImageGray, sigma: f32) -> ImageGray {
    if sigma <= 0.0 || img.width == 0 || img.height == 0 {
        return img.clone();
    }
    let radius = (3.0 * sigma).ceil() as i64;
    let mut kernel = Vec::with_capacity((2 * radius + 1) as usize);
    let inv_two_sigma_sq = -0.5 / (sigma * sigma);
    for k in -radius..=radius {
        kernel.push(((k * k) as f32 * inv_two_sigma_sq).exp());
    }
    let total: f32 = kernel.iter().sum();
    for weight in kernel.iter_mut() {
        *weight /= total;
    }
    let (w, h) = (img.width as i64, img.height as i64);
    let mut horizontal = ImageGray::new(img.width, img.height);
    for y in 0..h {
        for x in 0..w {
            let mut acc = 0.0;
            for (k, weight) in kernel.iter().enumerate() {
                let sx = mirror_index(x + k as i64 - radius, w);
                acc += weight * img.data[(y * w + sx) as usize];
            }
            horizontal.data[(y * w + x) as usize] = acc;
        }
    }
    let mut out = ImageGray::new(img.width, img.height);
    for y in 0..h {
        for x in 0..w {
            let mut acc = 0.0;
            for (k, weight) in kernel.iter().enumerate() {
                let sy = mirror_index(y + k as i64 - radius, h);
                acc += weight * horizontal.data[(sy * w + x) as usize];
            }
            out.data[(y * w + x) as usize] = acc;
        }
    }
    out
}

/// 📦️ Box blur over a `(2 radius + 1)²` window via an integral image; the window is clamped at the borders and the average divides by the number of covered pixels.
pub fn box_blur_integral(img: &ImageGray, radius: u32) -> ImageGray {
    if img.width == 0 || img.height == 0 {
        return img.clone();
    }
    let integral = IntegralImage::build(img);
    let mut out = ImageGray::new(img.width, img.height);
    for y in 0..img.height {
        let y0 = y.saturating_sub(radius);
        let y1 = (y + radius + 1).min(img.height);
        for x in 0..img.width {
            let x0 = x.saturating_sub(radius);
            let x1 = (x + radius + 1).min(img.width);
            let count = f64::from((x1 - x0) * (y1 - y0));
            out.set(x, y, (integral.rect_sum(x0, y0, x1, y1) / count) as f32);
        }
    }
    out
}

/// ➕️ Summed-area table over pixel values and squared values, with a zero top row and left column: `sum`/`sq_sum` have `(width + 1) * (height + 1)` entries and entry `(x, y)` holds the sum over pixels `[0, x) × [0, y)`.
/// <https://en.wikipedia.org/wiki/Summed-area_table>
#[derive(Clone, Debug, PartialEq)]
pub struct IntegralImage {
    pub width: u32,
    pub height: u32,
    pub sum: Vec<f64>,
    pub sq_sum: Vec<f64>,
}

impl IntegralImage {
    /// ➕️ Builds the summed-area tables in one row-prefix pass.
    pub fn build(img: &ImageGray) -> Self {
        let w = img.width as usize;
        let h = img.height as usize;
        let stride = w + 1;
        let mut sum = vec![0.0f64; stride * (h + 1)];
        let mut sq_sum = vec![0.0f64; stride * (h + 1)];
        for y in 0..h {
            let mut row = 0.0f64;
            let mut row_sq = 0.0f64;
            for x in 0..w {
                let v = f64::from(img.data[y * w + x]);
                row += v;
                row_sq += v * v;
                sum[(y + 1) * stride + x + 1] = sum[y * stride + x + 1] + row;
                sq_sum[(y + 1) * stride + x + 1] = sq_sum[y * stride + x + 1] + row_sq;
            }
        }
        Self { width: img.width, height: img.height, sum, sq_sum }
    }

    /// ➕️ Sum of pixel values over the inclusive-exclusive rectangle `x in [x0, x1), y in [y0, y1)`.
    pub fn rect_sum(&self, x0: u32, y0: u32, x1: u32, y1: u32) -> f64 {
        let stride = self.width as usize + 1;
        let (x0, y0, x1, y1) = (x0 as usize, y0 as usize, x1 as usize, y1 as usize);
        self.sum[y1 * stride + x1] - self.sum[y0 * stride + x1] - self.sum[y1 * stride + x0] + self.sum[y0 * stride + x0]
    }

    /// ➕️ Sum of squared pixel values over the inclusive-exclusive rectangle `x in [x0, x1), y in [y0, y1)`.
    pub fn rect_sq_sum(&self, x0: u32, y0: u32, x1: u32, y1: u32) -> f64 {
        let stride = self.width as usize + 1;
        let (x0, y0, x1, y1) = (x0 as usize, y0 as usize, x1 as usize, y1 as usize);
        self.sq_sum[y1 * stride + x1] - self.sq_sum[y0 * stride + x1] - self.sq_sum[y1 * stride + x0] + self.sq_sum[y0 * stride + x0]
    }
}

/// 🔻️ Half-resolution downsample by 2×2 averaging; odd dimensions round up (`div_ceil`) and edge blocks clamp to the last row/column.
pub fn downsample_half(img: &ImageGray) -> ImageGray {
    if img.width == 0 || img.height == 0 {
        return img.clone();
    }
    let out_w = img.width.div_ceil(2);
    let out_h = img.height.div_ceil(2);
    let mut out = ImageGray::new(out_w, out_h);
    for y in 0..out_h {
        let y0 = 2 * y;
        let y1 = (2 * y + 1).min(img.height - 1);
        for x in 0..out_w {
            let x0 = 2 * x;
            let x1 = (2 * x + 1).min(img.width - 1);
            let avg = 0.25 * (img.get(x0, y0) + img.get(x1, y0) + img.get(x0, y1) + img.get(x1, y1));
            out.set(x, y, avg);
        }
    }
    out
}
// #endregion 🔖️Filter

// #region 🔖️Gradient
/// 🧭️ Per-pixel image gradients, row-major with `gx`/`gy` of length `width * height`.
#[derive(Clone, Debug, PartialEq)]
pub struct GradientField {
    pub gx: Vec<f32>,
    pub gy: Vec<f32>,
    pub width: u32,
    pub height: u32,
}

/// 🧭️ 3×3 Scharr gradients with mirror padding, normalized by 1/32 so a unit-slope ramp yields a gradient of exactly 1 per pixel.
/// <https://en.wikipedia.org/wiki/Sobel_operator#Alternative_operators>
pub fn scharr_gradients(img: &ImageGray) -> GradientField {
    let (w, h) = (img.width as i64, img.height as i64);
    let mut gx = vec![0.0f32; (w * h) as usize];
    let mut gy = vec![0.0f32; (w * h) as usize];
    for y in 0..h {
        for x in 0..w {
            let p = |dx: i64, dy: i64| img.data[(mirror_index(y + dy, h) * w + mirror_index(x + dx, w)) as usize];
            let idx = (y * w + x) as usize;
            gx[idx] = (3.0 * (p(1, -1) - p(-1, -1)) + 10.0 * (p(1, 0) - p(-1, 0)) + 3.0 * (p(1, 1) - p(-1, 1))) / 32.0;
            gy[idx] = (3.0 * (p(-1, 1) - p(-1, -1)) + 10.0 * (p(0, 1) - p(0, -1)) + 3.0 * (p(1, 1) - p(1, -1))) / 32.0;
        }
    }
    GradientField { gx, gy, width: img.width, height: img.height }
}

/// 🧭️ Per-pixel gradient magnitude `√(gx² + gy²)` and orientation `atan2(gy, gx)` in radians, both row-major.
pub fn gradient_magnitude_orientation(g: &GradientField) -> (Vec<f32>, Vec<f32>) {
    let mut magnitude = Vec::with_capacity(g.gx.len());
    let mut orientation = Vec::with_capacity(g.gx.len());
    for (&gx, &gy) in g.gx.iter().zip(g.gy.iter()) {
        magnitude.push(gx.hypot(gy));
        orientation.push(gy.atan2(gx));
    }
    (magnitude, orientation)
}
// #endregion 🔖️Gradient

// #region 🔖️Pyramid
/// 🗻️ Coarse-to-fine image pyramid; `levels[0]` is the original and each next level halves the resolution (`scale = 0.5`).
#[derive(Clone, Debug, PartialEq)]
pub struct Pyramid {
    pub levels: Vec<ImageGray>,
    pub scale: f32,
}

/// 🗻️ Builds up to `n_levels` pyramid levels: each level is the previous one Gaussian-blurred with `sigma = 1` then 2×2-downsampled; stops early once a level reaches 1×1.
/// <https://en.wikipedia.org/wiki/Pyramid_(image_processing)>
pub fn build_pyramid(img: &ImageGray, n_levels: usize) -> Pyramid {
    let mut levels = Vec::with_capacity(n_levels);
    if n_levels == 0 {
        return Pyramid { levels, scale: 0.5 };
    }
    levels.push(img.clone());
    while levels.len() < n_levels {
        let prev = levels.last().expect("levels is non-empty after the initial push");
        if prev.width <= 1 && prev.height <= 1 {
            break;
        }
        let next = downsample_half(&gaussian_blur(prev, 1.0));
        levels.push(next);
    }
    Pyramid { levels, scale: 0.5 }
}
// #endregion 🔖️Pyramid

// #region 🔖️Patch
/// 🧩️ Square intensity patch of side `2 radius + 1`, row-major in `data`.
#[derive(Clone, Debug, PartialEq)]
pub struct Patch {
    pub radius: u32,
    pub data: Vec<f32>,
}

/// 🧩️ Bilinearly samples a square patch centered at `(cx, cy)`, with the sampling grid rotated by `rotation` radians (counter-clockwise offset `(dx, dy)` maps to `(cx + dx cos − dy sin, cy + dx sin + dy cos)`).
pub fn extract_patch(img: &ImageGray, cx: f32, cy: f32, radius: u32, rotation: f32) -> Patch {
    let side = (2 * radius + 1) as usize;
    let (sin, cos) = rotation.sin_cos();
    let r = radius as i64;
    let mut data = Vec::with_capacity(side * side);
    for dy in -r..=r {
        for dx in -r..=r {
            let fx = dx as f32;
            let fy = dy as f32;
            data.push(img.sample(cx + fx * cos - fy * sin, cy + fx * sin + fy * cos));
        }
    }
    Patch { radius, data }
}

/// 🎯️ Zero-mean normalized cross correlation in `[-1, 1]`; returns `0` for mismatched sizes or degenerate (near-zero) variance.
/// <https://en.wikipedia.org/wiki/Cross-correlation#Zero-normalized_cross-correlation_(ZNCC)>
pub fn zncc(a: &Patch, b: &Patch) -> f32 {
    if a.data.len() != b.data.len() || a.data.is_empty() {
        return 0.0;
    }
    let n = a.data.len() as f32;
    let mean_a = a.data.iter().sum::<f32>() / n;
    let mean_b = b.data.iter().sum::<f32>() / n;
    let mut numerator = 0.0f32;
    let mut var_a = 0.0f32;
    let mut var_b = 0.0f32;
    for (&pa, &pb) in a.data.iter().zip(b.data.iter()) {
        let da = pa - mean_a;
        let db = pb - mean_b;
        numerator += da * db;
        var_a += da * da;
        var_b += db * db;
    }
    let denominator = (var_a * var_b).sqrt();
    if denominator < 1e-12 {
        return 0.0;
    }
    numerator / denominator
}

/// 📏️ Sum of squared differences between two patches (mismatched sizes compare only the overlapping prefix).
pub fn ssd(a: &Patch, b: &Patch) -> f32 {
    a.data.iter().zip(b.data.iter()).map(|(&pa, &pb)| (pa - pb) * (pa - pb)).sum()
}

/// 🧮️ Census transform per pixel: bit `(dy + r)(2r + 1) + (dx + r)` is set when the neighbor at offset `(dx, dy)` is strictly darker than the center. `radius` is clamped to 3 (7×7 window, 49 bits) to fit `u64`; pixels whose window leaves the image get code `0`.
/// <https://en.wikipedia.org/wiki/Census_transform>
pub fn census_transform(img: &ImageGray, radius: u32) -> Vec<u64> {
    let r = radius.min(3) as i64;
    let (w, h) = (img.width as i64, img.height as i64);
    let side = 2 * r + 1;
    let mut out = vec![0u64; (w * h) as usize];
    for y in r..(h - r) {
        for x in r..(w - r) {
            let center = img.data[(y * w + x) as usize];
            let mut code = 0u64;
            for dy in -r..=r {
                for dx in -r..=r {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    if img.data[((y + dy) * w + x + dx) as usize] < center {
                        code |= 1 << ((dy + r) * side + (dx + r));
                    }
                }
            }
            out[(y * w + x) as usize] = code;
        }
    }
    out
}

/// 🧮️ Hamming distance between two census codes.
pub fn hamming_cost_census(a: u64, b: u64) -> u32 {
    (a ^ b).count_ones()
}
// #endregion 🔖️Patch

// #region 🔖️Warp
/// 🔀️ Affine warp by inverse mapping: `m` maps OUTPUT pixel coordinates to SOURCE coordinates (`sx = m[0]·[x, y, 1]`, `sy = m[1]·[x, y, 1]`), sampled bilinearly with clamped borders.
pub fn warp_affine(img: &ImageGray, m: &[[f32; 3]; 2], out_w: u32, out_h: u32) -> ImageGray {
    let mut out = ImageGray::new(out_w, out_h);
    for y in 0..out_h {
        for x in 0..out_w {
            let fx = x as f32;
            let fy = y as f32;
            let sx = m[0][0] * fx + m[0][1] * fy + m[0][2];
            let sy = m[1][0] * fx + m[1][1] * fy + m[1][2];
            out.set(x, y, img.sample(sx, sy));
        }
    }
    out
}

/// 🔀️ Homography warp by inverse mapping with perspective divide: `h` maps OUTPUT pixel coordinates to SOURCE coordinates in homogeneous form; outputs with a near-zero denominator are set to `0`.
/// <https://en.wikipedia.org/wiki/Homography_(computer_vision)>
pub fn warp_homography(img: &ImageGray, h: &[[f64; 3]; 3], out_w: u32, out_h: u32) -> ImageGray {
    let mut out = ImageGray::new(out_w, out_h);
    for y in 0..out_h {
        for x in 0..out_w {
            let fx = f64::from(x);
            let fy = f64::from(y);
            let denom = h[2][0] * fx + h[2][1] * fy + h[2][2];
            if denom.abs() < 1e-12 {
                continue;
            }
            let sx = (h[0][0] * fx + h[0][1] * fy + h[0][2]) / denom;
            let sy = (h[1][0] * fx + h[1][1] * fy + h[1][2]) / denom;
            out.set(x, y, img.sample(sx as f32, sy as f32));
        }
    }
    out
}

/// 🔀️ Generic remap: output pixel `i` (row-major over `out_w × out_h`) samples the source bilinearly at `(map_x[i], map_y[i])`.
pub fn remap(img: &ImageGray, map_x: &[f32], map_y: &[f32], out_w: u32, out_h: u32) -> ImageGray {
    let mut out = ImageGray::new(out_w, out_h);
    for ((dst, &mx), &my) in out.data.iter_mut().zip(map_x.iter()).zip(map_y.iter()) {
        *dst = img.sample(mx, my);
    }
    out
}
// #endregion 🔖️Warp

// #region 🔖️Nms
/// 📌️ Strict local maxima of a row-major score map: keeps pixels strictly greater than every other score in their `(2 radius + 1)²` window (clamped at borders) and strictly above `threshold`, returned as `(x, y, score)` sorted by descending score, then row-major position, for determinism.
pub fn non_max_suppression(scores: &[f32], width: u32, height: u32, radius: u32, threshold: f32) -> Vec<(u32, u32, f32)> {
    use std::cmp::Ordering;
    let (w, h) = (width as i64, height as i64);
    let r = radius as i64;
    let mut peaks = Vec::new();
    for y in 0..h {
        for x in 0..w {
            let score = scores[(y * w + x) as usize];
            if score.partial_cmp(&threshold) != Some(Ordering::Greater) {
                continue;
            }
            let mut is_max = true;
            'window: for ny in (y - r).max(0)..=(y + r).min(h - 1) {
                for nx in (x - r).max(0)..=(x + r).min(w - 1) {
                    if nx == x && ny == y {
                        continue;
                    }
                    if scores[(ny * w + nx) as usize] >= score {
                        is_max = false;
                        break 'window;
                    }
                }
            }
            if is_max {
                peaks.push((x as u32, y as u32, score));
            }
        }
    }
    peaks.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap_or(Ordering::Equal).then_with(|| (a.1, a.0).cmp(&(b.1, b.0))));
    peaks
}
// #endregion 🔖️Nms

// #region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn compressed_rope(bytes: &[u8], max_bytes: usize) -> CompressedChunkRope {
        let mut rope = CompressedChunkRope::default();
        for chunk in bytes.chunks(3_072) {
            rope.push(std::sync::Arc::<[u8]>::from(chunk), max_bytes).expect("bounded compressed fixture");
        }
        rope
    }

    fn jpeg_access_evidence_accepts(input_len: usize, metrics: CompressedRopeReadMetrics) -> bool {
        metrics.random_byte_reads > 0 && metrics.largest_random_read == 1 && metrics.sequential_bytes == 0 && input_len.checked_mul(2).is_some_and(|second_full_pass| metrics.random_byte_reads < second_full_pass)
    }

    fn jpeg_fixture_access_ceiling(bytes: &[u8]) -> Option<usize> {
        let baseline_frame = bytes.windows(2).position(|window| window == [0xff, 0xc0])?;
        let probed_positions = baseline_frame.checked_add(1)?;
        let safety_probe = probed_positions.checked_mul(2)?.checked_add(4)?;
        bytes.len().checked_add(safety_probe)
    }

    fn lcg_next(state: &mut u32) -> f32 {
        *state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (*state >> 8) as f32 / 16_777_216.0
    }

    fn lcg_image(width: u32, height: u32, seed: u32) -> ImageGray {
        let mut img = ImageGray::new(width, height);
        let mut state = seed;
        for value in img.data.iter_mut() {
            *value = lcg_next(&mut state);
        }
        img
    }

    fn smooth_image(width: u32, height: u32) -> ImageGray {
        let mut img = ImageGray::new(width, height);
        for y in 0..height {
            for x in 0..width {
                img.set(x, y, 0.5 + 0.25 * (x as f32 * 0.23).sin() + 0.2 * (y as f32 * 0.17).cos());
            }
        }
        img
    }

    fn mean_and_variance(img: &ImageGray) -> (f64, f64) {
        let n = img.data.len() as f64;
        let mean = img.data.iter().map(|&v| f64::from(v)).sum::<f64>() / n;
        let variance = img.data.iter().map(|&v| (f64::from(v) - mean).powi(2)).sum::<f64>() / n;
        (mean, variance)
    }

    fn invert3(h: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
        let det = h[0][0] * (h[1][1] * h[2][2] - h[1][2] * h[2][1]) - h[0][1] * (h[1][0] * h[2][2] - h[1][2] * h[2][0]) + h[0][2] * (h[1][0] * h[2][1] - h[1][1] * h[2][0]);
        let inv_det = 1.0 / det;
        [
            [(h[1][1] * h[2][2] - h[1][2] * h[2][1]) * inv_det, (h[0][2] * h[2][1] - h[0][1] * h[2][2]) * inv_det, (h[0][1] * h[1][2] - h[0][2] * h[1][1]) * inv_det],
            [(h[1][2] * h[2][0] - h[1][0] * h[2][2]) * inv_det, (h[0][0] * h[2][2] - h[0][2] * h[2][0]) * inv_det, (h[0][2] * h[1][0] - h[0][0] * h[1][2]) * inv_det],
            [(h[1][0] * h[2][1] - h[1][1] * h[2][0]) * inv_det, (h[0][1] * h[2][0] - h[0][0] * h[2][1]) * inv_det, (h[0][0] * h[1][1] - h[0][1] * h[1][0]) * inv_det],
        ]
    }

    #[test]
    fn luma_conversion_matches_bt601_weights() {
        let mut rgba = ImageRgba8::new(3, 1);
        rgba.data.copy_from_slice(&[255, 0, 0, 255, 255, 255, 255, 255, 10, 20, 30, 255]);
        let gray = ImageGray::from_rgba8_luma(&rgba);
        assert!((gray.get(0, 0) - 0.299).abs() < 1e-4);
        assert!((gray.get(1, 0) - 1.0).abs() < 1e-4);
        let expected = (0.299 * 10.0 + 0.587 * 20.0 + 0.114 * 30.0) / 255.0;
        assert!((gray.get(2, 0) - expected).abs() < 1e-6);
    }

    #[test]
    fn gray_sample_bilinear_interpolates_between_pixels() {
        let mut img = ImageGray::new(2, 2);
        img.data.copy_from_slice(&[0.0, 1.0, 0.0, 1.0]);
        assert!((img.sample(0.5, 0.5) - 0.5).abs() < 1e-6);
        assert!((img.sample(-5.0, -5.0) - 0.0).abs() < 1e-6);
        assert!((img.sample(10.0, 10.0) - 1.0).abs() < 1e-6);
    }

    #[test]
    fn rgba_sample_rgb_normalizes_and_interpolates() {
        let mut img = ImageRgba8::new(2, 1);
        img.data.copy_from_slice(&[0, 0, 0, 255, 255, 102, 0, 255]);
        let mid = img.sample_rgb(0.5, 0.0);
        assert!((mid[0] - 0.5).abs() < 1e-4);
        assert!((mid[1] - 0.2).abs() < 1e-4);
        assert!(mid[2].abs() < 1e-6);
    }

    #[test]
    fn png_rgba_round_trip_is_lossless() {
        let mut img = ImageRgba8::new(4, 3);
        for (i, value) in img.data.iter_mut().enumerate() {
            *value = (i * 17 % 251) as u8;
        }
        let bytes = encode_png(&img).expect("encode succeeds");
        let decoded = decode_png(&bytes).expect("decode succeeds");
        assert_eq!(decoded, img);
    }

    #[test]
    fn png_gray16_encode_decodes_back_losslessly() {
        let data: Vec<u16> = vec![0, 1, 500, 40_000, 65_535, 12_345];
        let bytes = encode_png_gray16(&data, 3, 2).expect("encode succeeds");
        let mut decoder = png::Decoder::new(bytes.as_slice());
        decoder.set_transformations(png::Transformations::IDENTITY);
        let mut reader = decoder.read_info().expect("readable png");
        let mut buf = vec![0u8; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).expect("decodable frame");
        assert_eq!(info.bit_depth, png::BitDepth::Sixteen);
        assert_eq!(info.color_type, png::ColorType::Grayscale);
        let decoded: Vec<u16> = buf[..info.buffer_size()].as_chunks::<2>().0.iter().map(|&pair| u16::from_be_bytes(pair)).collect();
        assert_eq!(decoded, data);
    }

    #[test]
    fn png_codec_rejects_dimension_mismatch() {
        let img = ImageRgba8 { width: 2, height: 2, data: vec![0; 3] };
        assert_eq!(encode_png(&img), Err(ImageError::Dimensions));
        assert_eq!(encode_png_gray16(&[1, 2, 3], 2, 2), Err(ImageError::Dimensions));
        assert!(matches!(decode_png(&[1, 2, 3]), Err(ImageError::Decode(_))));
    }

    #[test]
    fn gaussian_blur_preserves_mean_and_reduces_variance() {
        let img = lcg_image(32, 32, 7);
        let blurred = gaussian_blur(&img, 1.5);
        let (mean_before, var_before) = mean_and_variance(&img);
        let (mean_after, var_after) = mean_and_variance(&blurred);
        assert!((mean_before - mean_after).abs() < 1e-4);
        assert!(var_after < var_before);
    }

    #[test]
    fn box_blur_integral_matches_brute_force_average() {
        let img = lcg_image(9, 7, 21);
        let blurred = box_blur_integral(&img, 2);
        for y in 0..7u32 {
            for x in 0..9u32 {
                let (x0, x1) = (x.saturating_sub(2), (x + 3).min(9));
                let (y0, y1) = (y.saturating_sub(2), (y + 3).min(7));
                let mut total = 0.0f64;
                for sy in y0..y1 {
                    for sx in x0..x1 {
                        total += f64::from(img.get(sx, sy));
                    }
                }
                let expected = total / f64::from((x1 - x0) * (y1 - y0));
                assert!((f64::from(blurred.get(x, y)) - expected).abs() < 1e-5);
            }
        }
    }

    #[test]
    fn integral_image_rect_sums_match_brute_force() {
        let img = lcg_image(16, 16, 42);
        let integral = IntegralImage::build(&img);
        for &(x0, y0, x1, y1) in &[(0u32, 0u32, 16u32, 16u32), (2, 3, 10, 11), (5, 5, 6, 6), (0, 7, 16, 8), (4, 0, 5, 16)] {
            let mut brute_sum = 0.0f64;
            let mut brute_sq = 0.0f64;
            for y in y0..y1 {
                for x in x0..x1 {
                    let v = f64::from(img.get(x, y));
                    brute_sum += v;
                    brute_sq += v * v;
                }
            }
            assert!((integral.rect_sum(x0, y0, x1, y1) - brute_sum).abs() < 1e-8);
            assert!((integral.rect_sq_sum(x0, y0, x1, y1) - brute_sq).abs() < 1e-8);
        }
    }

    #[test]
    fn downsample_half_averages_two_by_two_blocks() {
        let mut img = ImageGray::new(4, 2);
        img.data.copy_from_slice(&[0.0, 1.0, 0.2, 0.4, 1.0, 0.0, 0.6, 0.8]);
        let half = downsample_half(&img);
        assert_eq!((half.width, half.height), (2, 1));
        assert!((half.get(0, 0) - 0.5).abs() < 1e-6);
        assert!((half.get(1, 0) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn scharr_gradients_recover_ramp_slopes() {
        let mut img = ImageGray::new(8, 8);
        for y in 0..8 {
            for x in 0..8 {
                img.set(x, y, 0.03 * x as f32 + 0.07 * y as f32);
            }
        }
        let g = scharr_gradients(&img);
        let idx = (3 * 8 + 3) as usize;
        assert!((g.gx[idx] - 0.03).abs() < 1e-5);
        assert!((g.gy[idx] - 0.07).abs() < 1e-5);
        let (magnitude, orientation) = gradient_magnitude_orientation(&g);
        assert!((magnitude[idx] - (0.03f32 * 0.03 + 0.07 * 0.07).sqrt()).abs() < 1e-5);
        assert!((orientation[idx] - 0.07f32.atan2(0.03)).abs() < 1e-4);
    }

    #[test]
    fn build_pyramid_halves_resolution_per_level() {
        let img = lcg_image(64, 48, 3);
        let pyramid = build_pyramid(&img, 4);
        assert_eq!(pyramid.levels.len(), 4);
        assert_eq!(pyramid.scale, 0.5);
        assert_eq!((pyramid.levels[0].width, pyramid.levels[0].height), (64, 48));
        assert_eq!((pyramid.levels[1].width, pyramid.levels[1].height), (32, 24));
        assert_eq!((pyramid.levels[3].width, pyramid.levels[3].height), (8, 6));
        assert_eq!(pyramid.levels[0], img);
    }

    #[test]
    fn zncc_is_invariant_to_gain_and_bias_and_flips_sign() {
        let img = lcg_image(16, 16, 99);
        let a = extract_patch(&img, 8.0, 8.0, 3, 0.0);
        let gained = Patch { radius: 3, data: a.data.iter().map(|&v| v * 1.7 + 0.2).collect() };
        let negated = Patch { radius: 3, data: a.data.iter().map(|&v| -v).collect() };
        assert!((zncc(&a, &gained) - 1.0).abs() < 1e-4);
        assert!((zncc(&a, &negated) + 1.0).abs() < 1e-4);
        let flat = Patch { radius: 3, data: vec![0.5; a.data.len()] };
        assert_eq!(zncc(&a, &flat), 0.0);
        assert_eq!(ssd(&a, &a), 0.0);
        assert!(ssd(&a, &gained) > 0.0);
    }

    #[test]
    fn extract_patch_quarter_turn_matches_transpose_relation() {
        let img = smooth_image(32, 32);
        let radius = 3u32;
        let side = (2 * radius + 1) as usize;
        let straight = extract_patch(&img, 15.0, 14.0, radius, 0.0);
        let rotated = extract_patch(&img, 15.0, 14.0, radius, std::f32::consts::FRAC_PI_2);
        for row in 0..side {
            for col in 0..side {
                let expected = straight.data[col * side + (side - 1 - row)];
                assert!((rotated.data[row * side + col] - expected).abs() < 1e-3);
            }
        }
    }

    #[test]
    fn census_transform_and_hamming_behave_on_known_pattern() {
        let mut img = ImageGray::new(5, 5);
        for (i, value) in img.data.iter_mut().enumerate() {
            *value = i as f32 / 25.0;
        }
        let codes = census_transform(&img, 1);
        assert_eq!(codes[2 * 5 + 2], 0b1111);
        assert_eq!(codes[0], 0);
        assert_eq!(hamming_cost_census(0b1011, 0b0010), 2);
        assert_eq!(hamming_cost_census(u64::MAX, 0), 64);
    }

    #[test]
    fn warp_affine_identity_returns_original() {
        let img = lcg_image(16, 16, 5);
        let identity = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
        let warped = warp_affine(&img, &identity, 16, 16);
        for (&out, &original) in warped.data.iter().zip(img.data.iter()) {
            assert!((out - original).abs() < 1e-6);
        }
    }

    #[test]
    fn warp_homography_inverse_round_trips_away_from_borders() {
        let img = smooth_image(48, 48);
        let h = [[1.0, 0.03, 1.5], [0.02, 1.0, -1.0], [2e-4, 1e-4, 1.0]];
        let h_inv = invert3(&h);
        let forward = warp_homography(&img, &h, 48, 48);
        let round = warp_homography(&forward, &h_inv, 48, 48);
        let margin = 8u32;
        let mut total = 0.0f64;
        let mut count = 0u32;
        for y in margin..(48 - margin) {
            for x in margin..(48 - margin) {
                total += f64::from((round.get(x, y) - img.get(x, y)).abs());
                count += 1;
            }
        }
        assert!(total / f64::from(count) < 2e-2);
    }

    #[test]
    fn remap_identity_returns_original() {
        let img = lcg_image(12, 10, 11);
        let mut map_x = vec![0.0f32; 120];
        let mut map_y = vec![0.0f32; 120];
        for y in 0..10u32 {
            for x in 0..12u32 {
                map_x[(y * 12 + x) as usize] = x as f32;
                map_y[(y * 12 + x) as usize] = y as f32;
            }
        }
        let out = remap(&img, &map_x, &map_y, 12, 10);
        assert_eq!(out, img);
    }

    #[test]
    fn non_max_suppression_keeps_planted_peaks_sorted() {
        let mut scores = vec![0.0f32; 81];
        scores[3 * 9 + 4] = 1.0;
        scores[3 * 9 + 5] = 0.9;
        scores[9 + 1] = 0.5;
        let peaks = non_max_suppression(&scores, 9, 9, 2, 0.1);
        assert_eq!(peaks, vec![(4, 3, 1.0), (1, 1, 0.5)]);
        assert!(non_max_suppression(&scores, 9, 9, 2, 2.0).is_empty());
    }

    fn gradient_rgba8(width: u32, height: u32) -> ImageRgba8 {
        let mut img = ImageRgba8::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                img.data[idx] = (x * 255 / width.max(1)) as u8;
                img.data[idx + 1] = (y * 255 / height.max(1)) as u8;
                img.data[idx + 2] = (((x + y) * 255) / (width + height).max(1)) as u8;
                img.data[idx + 3] = 255;
            }
        }
        img
    }

    fn checkerboard_rgba8(width: u32, height: u32, cell: u32) -> ImageRgba8 {
        let mut img = ImageRgba8::new(width, height);
        for y in 0..height {
            for x in 0..width {
                let idx = ((y * width + x) * 4) as usize;
                let on = ((x / cell.max(1)) + (y / cell.max(1))).is_multiple_of(2);
                let v = if on { 235 } else { 20 };
                img.data[idx] = v;
                img.data[idx + 1] = v;
                img.data[idx + 2] = v;
                img.data[idx + 3] = 255;
            }
        }
        img
    }

    fn flat_rgba8(width: u32, height: u32, rgb: [u8; 3]) -> ImageRgba8 {
        let mut img = ImageRgba8::new(width, height);
        for px in img.data.as_chunks_mut::<4>().0.iter_mut() {
            *px = [rgb[0], rgb[1], rgb[2], 255];
        }
        img
    }

    fn psnr_rgba8(a: &ImageRgba8, b: &ImageRgba8) -> f64 {
        let mse = a.data.iter().zip(b.data.iter()).map(|(&x, &y)| (f64::from(x) - f64::from(y)).powi(2)).sum::<f64>() / a.data.len() as f64;
        if mse <= 0.0 {
            f64::INFINITY
        } else {
            20.0 * 255.0f64.log10() - 10.0 * mse.log10()
        }
    }

    #[test]
    fn jpeg_round_trip_gradient_meets_psnr_floor() {
        let img = gradient_rgba8(64, 48);
        let bytes = encode_jpeg(&img, 90);
        let decoded = decode_jpeg(&bytes).expect("decode succeeds");
        assert_eq!((decoded.width, decoded.height), (img.width, img.height));
        assert!(psnr_rgba8(&img, &decoded) > 35.0);
    }

    #[test]
    fn jpeg_round_trip_checkerboard_meets_psnr_floor() {
        let img = checkerboard_rgba8(64, 32, 6);
        let bytes = encode_jpeg(&img, 90);
        let decoded = decode_jpeg(&bytes).expect("decode succeeds");
        assert_eq!((decoded.width, decoded.height), (img.width, img.height));
        assert!(psnr_rgba8(&img, &decoded) > 35.0);
    }

    #[test]
    fn jpeg_flat_color_round_trips_with_near_zero_error() {
        let img = flat_rgba8(32, 32, [200, 100, 50]);
        let bytes = encode_jpeg(&img, 90);
        let decoded = decode_jpeg(&bytes).expect("decode succeeds");
        assert_eq!((decoded.width, decoded.height), (img.width, img.height));
        assert!(psnr_rgba8(&img, &decoded) > 45.0);
    }

    #[test]
    fn jpeg_higher_quality_yields_lower_error() {
        let img = gradient_rgba8(48, 48);
        let psnr_high = psnr_rgba8(&img, &decode_jpeg(&encode_jpeg(&img, 100)).expect("decode succeeds"));
        let psnr_low = psnr_rgba8(&img, &decode_jpeg(&encode_jpeg(&img, 10)).expect("decode succeeds"));
        assert!(psnr_high > psnr_low);
    }

    #[test]
    fn jpeg_decode_never_panics_on_truncated_input() {
        let img = gradient_rgba8(32, 32);
        let bytes = encode_jpeg(&img, 80);
        for cut in [0usize, 1, 2, 4, 10, bytes.len() / 4, bytes.len() / 2, (3 * bytes.len()) / 4, bytes.len() - 5, bytes.len() - 1] {
            let result = decode_jpeg(&bytes[..cut]);
            assert!(result.is_err(), "truncation at {cut} bytes should error, not decode");
        }
    }

    #[test]
    fn jpeg_single_mcu_cosine_pattern_recovers_within_quantization_error() {
        let mut img = ImageRgba8::new(16, 16);
        for y in 0..16u32 {
            for x in 0..16u32 {
                let idx = ((y * 16 + x) * 4) as usize;
                let value = (128.0 + 60.0 * (x as f32 * std::f32::consts::PI / 8.0).cos()).round().clamp(0.0, 255.0) as u8;
                img.data[idx] = value;
                img.data[idx + 1] = value;
                img.data[idx + 2] = value;
                img.data[idx + 3] = 255;
            }
        }
        let bytes = encode_jpeg(&img, 95);
        let decoded = decode_jpeg(&bytes).expect("decode succeeds");
        assert_eq!((decoded.width, decoded.height), (16, 16));
        let mut max_abs_error = 0i32;
        for (&expected, &actual) in img.data.iter().zip(decoded.data.iter()) {
            max_abs_error = max_abs_error.max((i32::from(expected) - i32::from(actual)).abs());
        }
        assert!(max_abs_error < 20, "max abs error {max_abs_error} exceeds quantization tolerance");
    }

    #[test]
    fn jpeg_progressive_marker_is_unsupported() {
        let bytes = [0xFFu8, 0xD8, 0xFF, 0xC2, 0x00, 0x0B, 0x08, 0x00, 0x10, 0x00, 0x10, 0x03, 0x01, 0x22, 0x00, 0x02, 0x11, 0x01];
        assert!(matches!(decode_jpeg(&bytes), Err(ImageError::UnsupportedJpeg(_))));
    }

    #[test]
    fn jpeg_decode_rejects_missing_soi() {
        assert!(matches!(decode_jpeg(&[0x00, 0x01, 0x02]), Err(ImageError::Decode(_))));
        assert!(matches!(decode_jpeg(&[]), Err(ImageError::Decode(_))));
    }

    #[test]
    fn production_png_decoder_worker_steps_are_scanline_bounded() {
        let (width, height) = (512, 512);
        let pixels = vec![127; width as usize * height as usize * 4];
        let mut encoded = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut encoded, width, height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().expect("PNG header");
            writer.write_image_data(&pixels).expect("PNG fixture");
        }
        let rope = compressed_rope(&encoded, 1_114_112);
        assert!(rope.leaf_lengths().iter().all(|length| *length <= COMPRESSED_ROPE_LEAF_BYTES));
        let observed_reads = rope.clone();
        let image = std::thread::spawn(move || {
            let mut decoder = BoundedStillDecoder::new("image/png", rope);
            loop {
                let started = std::time::Instant::now();
                let progress = decoder.advance();
                assert!(started.elapsed() < std::time::Duration::from_millis(8), "bounded PNG worker step exceeded 8 ms");
                match progress {
                    BoundedDecodeProgress::Working => {}
                    BoundedDecodeProgress::Complete(image) => break image,
                    BoundedDecodeProgress::Failed(error) => panic!("bounded PNG decode failed: {error}"),
                }
            }
        })
        .join()
        .expect("PNG worker");
        assert_eq!((image.width, image.height), (width, height));
        assert_eq!(image.data.len(), pixels.len());
        let metrics = observed_reads.read_metrics();
        assert!(metrics.sequential_reads > 0 && metrics.sequential_reads <= encoded.len().saturating_add(2));
        assert!(metrics.sequential_bytes > 0 && metrics.sequential_bytes <= encoded.len(), "PNG decoding must never reread the complete input");
        assert!(metrics.largest_sequential_read <= COMPRESSED_ROPE_LEAF_BYTES, "PNG decoding must keep every source read within one leaf");
        assert!(metrics.random_byte_reads > 0 && metrics.random_byte_reads <= encoded.len().min(32), "PNG admission random probes stay within an input-derived header bound");
        assert_eq!(metrics.largest_random_read, 1);
    }

    #[test]
    fn maximum_admitted_png_scanline_stays_below_the_worker_ceiling() {
        let (width, height) = (MAX_PNG_ROW_PIXELS, 64);
        let mut pixels = vec![0; width as usize * height as usize * 4];
        for index in 0..width as usize * height as usize {
            let value = ((index * 131) ^ (index / width as usize * 197)) as u8;
            pixels[index * 4..index * 4 + 4].copy_from_slice(&[value, value.rotate_left(3), !value, 255]);
        }
        let mut encoded = Vec::new();
        {
            let mut encoder = png::Encoder::new(&mut encoded, width, height);
            encoder.set_color(png::ColorType::Rgba);
            encoder.set_depth(png::BitDepth::Eight);
            encoder.write_header().expect("PNG header").write_image_data(&pixels).expect("PNG fixture");
        }
        assert!(encoded.len() <= 1_114_112, "maximum accepted PNG fixture must fit the production input envelope");
        let mut decoder = BoundedStillDecoder::new("image/png", compressed_rope(&encoded, 1_114_112));
        loop {
            let started = std::time::Instant::now();
            let progress = decoder.advance();
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "maximum PNG scanline worker step exceeded 8 ms");
            match progress {
                BoundedDecodeProgress::Working => {}
                BoundedDecodeProgress::Complete(image) => {
                    assert_eq!((image.width, image.height), (width, height));
                    break;
                }
                BoundedDecodeProgress::Failed(error) => panic!("maximum-row PNG failed: {error}"),
            }
        }

        let mut oversized = encoded;
        oversized[20..24].copy_from_slice(&65u32.to_be_bytes());
        let mut decoder = BoundedStillDecoder::new("image/png", compressed_rope(&oversized, 1_114_112));
        loop {
            let started = std::time::Instant::now();
            let progress = decoder.advance();
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "oversized PNG admission step exceeded 8 ms");
            match progress {
                BoundedDecodeProgress::Working => {}
                BoundedDecodeProgress::Failed(_) => break,
                BoundedDecodeProgress::Complete(_) => panic!("oversized PNG must not allocate or decode a full output"),
            }
        }
    }

    #[test]
    fn oversized_jpeg_is_rejected_without_an_unbounded_codec_step() {
        let bytes = vec![0; MAX_JPEG_COMPRESSED_BYTES + 1];
        let mut decoder = BoundedStillDecoder::new("image/jpeg", compressed_rope(&bytes, MAX_JPEG_COMPRESSED_BYTES + 1));
        loop {
            let started = std::time::Instant::now();
            let progress = decoder.advance();
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "bounded JPEG admission step exceeded 8 ms");
            match progress {
                BoundedDecodeProgress::Working => {}
                BoundedDecodeProgress::Failed(_) => break,
                BoundedDecodeProgress::Complete(_) => panic!("oversized JPEG must not enter the completion-only codec"),
            }
        }
    }

    #[test]
    fn admitted_jpeg_reads_the_chunk_rope_without_a_join_allocation() {
        let bytes = encode_jpeg(&checkerboard_rgba8(64, 64, 4), 90);
        assert!(!bytes.windows(2).any(|window| window == [0xff, 0xdd]), "real JPEG fixture must retain the owned decoder's monotone no-restart source path");
        let rope = compressed_rope(&bytes, MAX_JPEG_COMPRESSED_BYTES);
        assert!(rope.leaf_lengths().iter().all(|length| *length <= COMPRESSED_ROPE_LEAF_BYTES));
        let observed_reads = rope.clone();
        let mut decoder = BoundedStillDecoder::new("image/jpeg", rope);
        loop {
            match decoder.advance() {
                BoundedDecodeProgress::Working => {}
                BoundedDecodeProgress::Complete(image) => {
                    assert_eq!((image.width, image.height), (64, 64));
                    break;
                }
                BoundedDecodeProgress::Failed(error) => panic!("bounded JPEG decode failed: {error}"),
            }
        }
        let metrics = observed_reads.read_metrics();
        let fixture_ceiling = jpeg_fixture_access_ceiling(&bytes).expect("real baseline JPEG fixture has a checked source-access ceiling");
        let duplicate_pass = bytes.len().checked_mul(2).expect("bounded JPEG fixture supports a checked duplicate-pass ceiling");
        assert!(fixture_ceiling < duplicate_pass, "owned decoder plus bounded SOF0 probe ceiling must exclude a second complete pass");
        assert!(metrics.random_byte_reads <= fixture_ceiling, "JPEG source accesses exceed the monotone owned decoder plus bounded SOF0 probe ceiling");
        assert!(jpeg_access_evidence_accepts(bytes.len(), metrics), "JPEG evidence must be strict, overflow-safe, one-byte-unit, and below a second complete input pass");
    }

    #[test]
    fn jpeg_access_evidence_rejects_a_simulated_second_complete_pass() {
        let metrics = |random_byte_reads, largest_random_read| CompressedRopeReadMetrics { random_byte_reads, largest_random_read, ..Default::default() };
        assert!(!jpeg_access_evidence_accepts(0, metrics(0, 0)));
        assert!(jpeg_access_evidence_accepts(1, metrics(1, 1)));
        assert!(!jpeg_access_evidence_accepts(1, metrics(2, 1)));
        assert!(!jpeg_access_evidence_accepts(usize::MAX, metrics(1, 1)), "checked multiplication must reject an unrepresentable ceiling");
        let input_len = 4_096usize;
        let second_complete_pass = input_len.checked_mul(2).expect("bounded negative instrumentation fixture");
        assert!(!jpeg_access_evidence_accepts(input_len, metrics(second_complete_pass, 1)), "a simulated second complete input pass must fail the evidence predicate");
        assert!(!jpeg_access_evidence_accepts(input_len, metrics(input_len, 2)), "a multi-byte random access unit must fail the one-byte evidence predicate");
    }

    #[test]
    fn accepted_worst_envelope_jpeg_and_malformed_entropy_steps_are_timed() {
        let (width, height) = (512u32, 512u32);
        let mut pixels = vec![0; width as usize * height as usize * 4];
        for index in 0..width as usize * height as usize {
            let value = (((index % width as usize) / 16 + (index / width as usize) / 16) % 2 * 223) as u8;
            pixels[index * 4..index * 4 + 4].copy_from_slice(&[value, 255 - value, value.rotate_left(2), 255]);
        }
        let snapshot = semio_s_plugin_stdio::artifacts::jpg::JpgSnapshot { width, height, pixels, re_encode_quality: Some(85), ..Default::default() };
        let encoded = semio_s_plugin_stdio::artifacts::jpg::engine::encode_jpg(&snapshot).expect("JPEG fixture");
        assert!(encoded.len() <= MAX_JPEG_COMPRESSED_BYTES);
        let chunks = compressed_rope(&encoded, MAX_JPEG_COMPRESSED_BYTES);
        std::thread::spawn(move || {
            let mut decoder = BoundedStillDecoder::new("image/jpeg", chunks);
            loop {
                let started = std::time::Instant::now();
                let progress = decoder.advance();
                assert!(started.elapsed() < std::time::Duration::from_millis(8), "accepted JPEG worker step exceeded 8 ms in this build profile");
                match progress {
                    BoundedDecodeProgress::Working => {}
                    BoundedDecodeProgress::Complete(image) => {
                        assert_eq!((image.width, image.height), (width, height));
                        break;
                    }
                    BoundedDecodeProgress::Failed(error) => panic!("accepted JPEG failed: {error}"),
                }
            }
        })
        .join()
        .expect("JPEG worker");

        let mut malformed = vec![0; 32_768];
        malformed[..13].copy_from_slice(&[0xff, 0xd8, 0xff, 0xc0, 0x00, 0x0b, 0x08, 0x01, 0x00, 0x01, 0x00, 0x01, 0x11]);
        let chunks = compressed_rope(&malformed, MAX_JPEG_COMPRESSED_BYTES);
        let mut decoder = BoundedStillDecoder::new("image/jpeg", chunks);
        loop {
            let started = std::time::Instant::now();
            let progress = decoder.advance();
            assert!(started.elapsed() < std::time::Duration::from_millis(8), "malformed JPEG worker step exceeded 8 ms in this build profile");
            match progress {
                BoundedDecodeProgress::Working => {}
                BoundedDecodeProgress::Failed(_) => break,
                BoundedDecodeProgress::Complete(_) => panic!("malformed JPEG decoded"),
            }
        }
    }
}
// #endregion 🔖️Tests
