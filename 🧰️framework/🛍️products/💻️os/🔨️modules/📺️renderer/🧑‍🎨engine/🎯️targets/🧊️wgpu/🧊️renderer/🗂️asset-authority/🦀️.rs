//! 🗂️ Renderer asset authority: process-wide asset I/O statics, request and seal functions, the fetch owner, and the page and format structure cursors.
use super::*;

//#region 📡️RendererAssetAuthority
static RENDERER_ASSET_IO: OnceLock<Mutex<WorldAssetIoAuthority>> = OnceLock::new();
static RENDERER_ASSET_GENERATION: AtomicU64 = AtomicU64::new(1);

pub fn renderer_asset_io() -> &'static Mutex<WorldAssetIoAuthority> {
    RENDERER_ASSET_IO.get_or_init(|| Mutex::new(WorldAssetIoAuthority::default()))
}

pub fn reserve_renderer_asset_request(kind: WorldAssetRequestKind, url: &str) -> Result<WorldAssetRequestToken, WorldAssetFault> {
    let generation = RENDERER_ASSET_GENERATION.fetch_add(1, Ordering::Relaxed).max(1);
    renderer_asset_io().lock().expect("renderer asset authority lock").reserve_request(generation, 0, kind, url)
}

pub fn take_next_renderer_asset() -> Option<WorldAssetFetchOwner> {
    renderer_asset_io().lock().ok()?.take_next()
}

pub fn reserve_renderer_asset_response(owner: &mut WorldAssetFetchOwner, byte_credits: usize) -> bool {
    renderer_asset_io().lock().is_ok_and(|mut authority| authority.reserve_response(owner, byte_credits).is_ok())
}

/// 📡️ How one response seal ended. See [`RuntimeMailbox::seal_renderer_asset_response`] for why
/// `Busy` is not a refusal.
pub enum RendererAssetSealStep {
    Granted,
    Busy,
    Refused(&'static str),
}

pub fn seal_renderer_asset_response(owner: &mut WorldAssetFetchOwner) -> bool {
    renderer_asset_io().lock().is_ok_and(|mut authority| authority.seal_response(owner).is_ok())
}

pub fn return_renderer_asset(owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {
    let Ok(mut authority) = renderer_asset_io().lock() else { return Err(owner) };
    authority.return_owner(owner)
}

pub fn take_next_completed_renderer_asset_step() -> Option<WorldAssetFetchOwner> {
    renderer_asset_io().lock().ok()?.take_next_completed_step()
}

pub fn finish_renderer_asset(owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {
    let Ok(mut authority) = renderer_asset_io().lock() else { return Err(owner) };
    authority.finish(owner)
}

pub fn retire_cancelled_renderer_asset_step() -> bool {
    renderer_asset_io().lock().is_ok_and(|mut authority| authority.retire_cancelled_step())
}

pub fn close_renderer_asset_step() -> bool {
    if !crate::icon_export_asset::close_all_step() {
        return false;
    }
    let Ok(mut authority) = renderer_asset_io().lock() else { return false };
    authority.begin_close();
    authority.close_step() && authority.terminal_is_empty()
}

/// 🗺️ Returns every pending `MapTile` claim for one closed engine surface to the shared lane.
pub fn cancel_renderer_map_tile_assets(surface: &str) {
    let Ok(mut authority) = renderer_asset_io().lock() else { return };
    authority.cancel_map_tiles_for_surface(surface);
    for _ in 0..WORLD_ASSET_REQUEST_CAPACITY {
        if !authority.retire_cancelled_step() {
            break;
        }
    }
}

pub enum RendererAssetFetchOwner {
    World { surface: WorldAssetMetadataId, surface_token: AdmittedSurfaceToken, owner: WorldAssetFetchOwner },
    Shared(WorldAssetFetchOwner),
}

impl RendererAssetFetchOwner {
    pub fn url(&self) -> &str {
        match self {
            Self::World { owner, .. } | Self::Shared(owner) => owner.url(),
        }
    }

    pub fn kind(&self) -> WorldAssetRequestKind {
        match self {
            Self::World { owner, .. } | Self::Shared(owner) => owner.kind(),
        }
    }

    pub fn owner_mut(&mut self) -> &mut WorldAssetFetchOwner {
        match self {
            Self::World { owner, .. } | Self::Shared(owner) => owner,
        }
    }

    pub fn owner(&self) -> &WorldAssetFetchOwner {
        match self {
            Self::World { owner, .. } | Self::Shared(owner) => owner,
        }
    }

    pub fn begin_close(&mut self) {
        self.owner_mut().begin_close();
    }

    pub fn close_step(&mut self) -> bool {
        self.owner_mut().close_step()
    }

    pub fn generation(&self) -> u64 {
        self.owner().generation()
    }

    pub fn revision(&self) -> u64 {
        self.owner().revision()
    }

    pub fn decode_page(&self) -> Result<Option<&WorldAssetResponsePage>, WorldAssetFault> {
        self.owner().decode_page()
    }

    fn decode_page_at(&self, index: u16) -> Result<Option<&WorldAssetResponsePage>, WorldAssetFault> {
        self.owner().decode_page_at(index)
    }

    pub fn advance_decode_page(&mut self) -> Result<(), WorldAssetFault> {
        self.owner_mut().advance_decode_page()
    }

    pub fn rewind_decode_pages(&mut self) -> Result<(), WorldAssetFault> {
        self.owner_mut().rewind_decode_pages()
    }

    pub fn received_bytes(&self) -> usize {
        self.owner().received_bytes()
    }
}

pub const RENDERER_ASSET_PROBE_BYTES: usize = 64;
pub const RENDERER_ASSET_PARSE_BLOCK_BYTES: usize = 256;

/// 🖼️ The straight-RGBA ceiling ONE decoded image asset may claim — 16 megapixels, i.e. the 4096²
/// surface every graphics backend in `🖌️render/🔌️backend` guarantees (`max_texture_dimension` 8192 on
/// the conservative default, 16384 on Metal/D3D12). It is a pre-decode guard read out of the PNG
/// `IHDR` and the JPEG frame header, so a compressed bomb is refused before any pixel buffer is
/// reserved.
///
/// 🩸️ It used to be 16 MiB = 4 megapixels, which refused the reference underlay the puzzle3d
/// playground itself ships (`/infinite-assets/🏘️abbau-aufbau-masterarbeit-grundriss/🖼️.jpg`,
/// 2275×2560 = 23 296 000 straight-RGBA bytes). React's `WorldReferenceLayer` hands that same file to
/// the browser decoder and paints it, so on wgpu the scene lost its reference plane — and, because
/// the refusal was escalated to a frame fault, the whole surface was quarantined ~3 s into every boot
/// (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY, `📓️w3a-asset-decoder-boot-fault.md`).
pub const RENDERER_ASSET_PIXEL_BYTES: usize = 64 * 1024 * 1024;

pub struct RendererAssetPageIndex {
    starts: Box<[usize; WORLD_ASSET_RESPONSE_PAGE_CAPACITY]>,
    lengths: Box<[u16; WORLD_ASSET_RESPONSE_PAGE_CAPACITY]>,
    pub len: u16,
    pub total: usize,
}

impl RendererAssetPageIndex {
    pub fn new() -> Self {
        Self { starts: Box::new([0; WORLD_ASSET_RESPONSE_PAGE_CAPACITY]), lengths: Box::new([0; WORLD_ASSET_RESPONSE_PAGE_CAPACITY]), len: 0, total: 0 }
    }

    pub fn admit(&mut self, bytes: usize) -> Result<(), &'static str> {
        let slot = usize::from(self.len);
        if slot == WORLD_ASSET_RESPONSE_PAGE_CAPACITY || bytes == 0 || bytes > WORLD_ASSET_RESPONSE_PAGE_BYTES {
            return Err("asset response page index exceeded fixed credits");
        }
        self.starts[slot] = self.total;
        self.lengths[slot] = u16::try_from(bytes).map_err(|_| "asset response page exceeded fixed byte credits")?;
        self.total = self.total.checked_add(bytes).ok_or("asset response page index overflowed")?;
        self.len += 1;
        Ok(())
    }

    fn locate(&self, absolute: usize) -> Result<(u16, usize), &'static str> {
        if absolute >= self.total {
            return Err("asset semantic read exceeded sealed bytes");
        }
        let mut low = 0usize;
        let mut high = usize::from(self.len);
        while low < high {
            let middle = low + (high - low) / 2;
            if self.starts[middle] <= absolute {
                low = middle + 1;
            } else {
                high = middle;
            }
        }
        let slot = low.checked_sub(1).ok_or("asset semantic page index was empty")?;
        let offset = absolute - self.starts[slot];
        if offset >= usize::from(self.lengths[slot]) {
            return Err("asset semantic page index contained a gap");
        }
        Ok((slot as u16, offset))
    }

    pub fn read<const N: usize>(&self, owner: &RendererAssetFetchOwner, absolute: usize) -> Result<[u8; N], &'static str> {
        let mut result = [0; N];
        for (delta, output) in result.iter_mut().enumerate() {
            let at = absolute.checked_add(delta).ok_or("asset semantic byte address overflowed")?;
            let (page_index, page_offset) = self.locate(at)?;
            let page = owner.decode_page_at(page_index).map_err(|_| "asset semantic random read lost its response owner")?.ok_or("asset semantic random read lost its response page")?;
            *output = *page.bytes().get(page_offset).ok_or("asset semantic random read exceeded its response page")?;
        }
        Ok(result)
    }
}

pub struct RendererAssetPageCursor {
    offset: u16,
    pub absolute: usize,
}

impl RendererAssetPageCursor {
    pub fn new() -> Self {
        Self { offset: 0, absolute: 0 }
    }

    pub fn read_block(&mut self, owner: &mut RendererAssetFetchOwner) -> Result<Option<([u8; RENDERER_ASSET_PARSE_BLOCK_BYTES], u16)>, &'static str> {
        let Some(page) = owner.decode_page().map_err(|_| "asset structure cursor lost its response page")? else { return Ok(None) };
        let start = usize::from(self.offset);
        if start >= page.bytes().len() {
            return Err("asset structure cursor exceeded its response page");
        }
        let count = RENDERER_ASSET_PARSE_BLOCK_BYTES.min(page.bytes().len() - start);
        let mut block = [0; RENDERER_ASSET_PARSE_BLOCK_BYTES];
        block[..count].copy_from_slice(&page.bytes()[start..start + count]);
        self.offset += count as u16;
        self.absolute = self.absolute.checked_add(count).ok_or("asset structure cursor byte count overflowed")?;
        if usize::from(self.offset) == page.bytes().len() {
            owner.advance_decode_page().map_err(|_| "asset structure cursor could not advance its response page")?;
            self.offset = 0;
        }
        Ok(Some((block, count as u16)))
    }

    pub fn read_byte(&mut self, owner: &mut RendererAssetFetchOwner) -> Result<Option<u8>, &'static str> {
        let Some(page) = owner.decode_page().map_err(|_| "asset semantic cursor lost its response page")? else { return Ok(None) };
        let offset = usize::from(self.offset);
        let byte = page.bytes().get(offset).copied().ok_or("asset semantic cursor exceeded its response page")?;
        self.offset += 1;
        self.absolute = self.absolute.checked_add(1).ok_or("asset semantic cursor byte count overflowed")?;
        if usize::from(self.offset) == page.bytes().len() {
            owner.advance_decode_page().map_err(|_| "asset semantic cursor could not advance its response page")?;
            self.offset = 0;
        }
        Ok(Some(byte))
    }
}

pub enum RendererAssetFormatCursor {
    Glb(GlbStructureCursor),
    Png(PngStructureCursor),
    Jpeg(JpegStructureCursor),
    Svg(TextAssetStructureCursor),
    Protobuf(ProtobufStructureCursor),
    Opaque(TextAssetStructureCursor),
}

impl RendererAssetFormatCursor {
    pub fn new(kind: WorldAssetRequestKind, prefix: &[u8], total_bytes: usize) -> Result<Self, &'static str> {
        match kind {
            WorldAssetRequestKind::Glb => Ok(Self::Glb(GlbStructureCursor::new(total_bytes))),
            WorldAssetRequestKind::ReferenceImage => {
                if prefix.starts_with(b"\x89PNG\r\n\x1a\n") {
                    Ok(Self::Png(PngStructureCursor::new(total_bytes)))
                } else if prefix.starts_with(&[0xff, 0xd8, 0xff]) {
                    Ok(Self::Jpeg(JpegStructureCursor::new(total_bytes)))
                } else if renderer_asset_svg_prefix_is_valid(prefix) {
                    Ok(Self::Svg(TextAssetStructureCursor::new(total_bytes, true, true)))
                } else if renderer_asset_reference_prefix_is_valid(prefix) {
                    Ok(Self::Opaque(TextAssetStructureCursor::new(total_bytes, false, false)))
                } else {
                    Err("reference image format was outside the React media contract")
                }
            }
            WorldAssetRequestKind::UiImage { .. } | WorldAssetRequestKind::MapTile { vector: false, .. } => {
                if prefix.starts_with(b"\x89PNG\r\n\x1a\n") {
                    Ok(Self::Png(PngStructureCursor::new(total_bytes)))
                } else if prefix.starts_with(&[0xff, 0xd8, 0xff]) {
                    Ok(Self::Jpeg(JpegStructureCursor::new(total_bytes)))
                } else {
                    Ok(Self::Svg(TextAssetStructureCursor::new(total_bytes, true, true)))
                }
            }
            WorldAssetRequestKind::MapTile { vector: true, .. } => Ok(Self::Protobuf(ProtobufStructureCursor::new(total_bytes))),
            WorldAssetRequestKind::Terrain { .. } => Ok(Self::Opaque(TextAssetStructureCursor::new(total_bytes, false, false))),
        }
    }

    pub fn feed(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        match self {
            Self::Glb(cursor) => cursor.feed(bytes),
            Self::Png(cursor) => cursor.feed(bytes),
            Self::Jpeg(cursor) => cursor.feed(bytes),
            Self::Svg(cursor) | Self::Opaque(cursor) => cursor.feed(bytes),
            Self::Protobuf(cursor) => cursor.feed(bytes),
        }
    }

    pub fn finish(&self) -> Result<(), &'static str> {
        match self {
            Self::Glb(cursor) => cursor.finish(),
            Self::Png(cursor) => cursor.finish(),
            Self::Jpeg(cursor) => cursor.finish(),
            Self::Svg(cursor) | Self::Opaque(cursor) => cursor.finish(),
            Self::Protobuf(cursor) => cursor.finish(),
        }
    }
}

enum GlbStructurePhase {
    Header { bytes: [u8; 12], len: u8 },
    ChunkHeader { bytes: [u8; 8], len: u8 },
    ChunkPayload { remaining: u32 },
    Terminal,
}

struct GlbStructureCursor {
    total_bytes: usize,
    consumed: usize,
    json: bool,
    bin: bool,
    phase: GlbStructurePhase,
}

impl GlbStructureCursor {
    fn new(total_bytes: usize) -> Self {
        Self { total_bytes, consumed: 0, json: false, bin: false, phase: GlbStructurePhase::Header { bytes: [0; 12], len: 0 } }
    }

    fn feed(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        for &byte in bytes {
            self.consumed = self.consumed.checked_add(1).ok_or("GLB byte count overflowed")?;
            if self.consumed > self.total_bytes {
                return Err("GLB exceeded its declared byte claim");
            }
            match &mut self.phase {
                GlbStructurePhase::Header { bytes, len } => {
                    bytes[usize::from(*len)] = byte;
                    *len += 1;
                    if *len == 12 {
                        if &bytes[..4] != b"glTF" || u32::from_le_bytes(bytes[4..8].try_into().expect("fixed GLB version")) != 2 || usize::try_from(u32::from_le_bytes(bytes[8..12].try_into().expect("fixed GLB length"))).ok() != Some(self.total_bytes)
                        {
                            return Err("GLB header was invalid");
                        }
                        self.phase = GlbStructurePhase::ChunkHeader { bytes: [0; 8], len: 0 };
                    }
                }
                GlbStructurePhase::ChunkHeader { bytes, len } => {
                    bytes[usize::from(*len)] = byte;
                    *len += 1;
                    if *len == 8 {
                        let length = u32::from_le_bytes(bytes[..4].try_into().expect("fixed GLB chunk length"));
                        let kind = u32::from_le_bytes(bytes[4..8].try_into().expect("fixed GLB chunk kind"));
                        if length == 0 || !length.is_multiple_of(4) || self.consumed.checked_add(length as usize).is_none_or(|end| end > self.total_bytes) {
                            return Err("GLB chunk exceeded fixed structure credits");
                        }
                        match kind {
                            0x4e4f534a if !self.json && !self.bin => self.json = true,
                            0x004e4942 if self.json && !self.bin => self.bin = true,
                            _ => return Err("GLB chunk order or kind was unsupported"),
                        }
                        self.phase = GlbStructurePhase::ChunkPayload { remaining: length };
                    }
                }
                GlbStructurePhase::ChunkPayload { remaining } => {
                    *remaining -= 1;
                    if *remaining == 0 {
                        self.phase = if self.consumed == self.total_bytes { GlbStructurePhase::Terminal } else { GlbStructurePhase::ChunkHeader { bytes: [0; 8], len: 0 } };
                    }
                }
                GlbStructurePhase::Terminal => return Err("GLB contained trailing bytes"),
            }
        }
        Ok(())
    }

    fn finish(&self) -> Result<(), &'static str> {
        if self.consumed == self.total_bytes && self.json && self.bin && matches!(self.phase, GlbStructurePhase::Terminal) {
            Ok(())
        } else {
            Err("GLB structure ended before JSON and BIN reached terminal")
        }
    }
}

enum PngStructurePhase {
    Signature { bytes: [u8; 8], len: u8 },
    ChunkHeader { bytes: [u8; 8], len: u8 },
    ChunkPayload { remaining: u32, kind: [u8; 4], ihdr: [u8; 13], ihdr_len: u8 },
    Crc { remaining: u8, terminal_after: bool },
    Terminal,
}

pub struct PngStructureCursor {
    total_bytes: usize,
    consumed: usize,
    ihdr: bool,
    idat: bool,
    pub phase: PngStructurePhase,
}

impl PngStructureCursor {
    pub fn new(total_bytes: usize) -> Self {
        Self { total_bytes, consumed: 0, ihdr: false, idat: false, phase: PngStructurePhase::Signature { bytes: [0; 8], len: 0 } }
    }

    pub fn feed(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        for &byte in bytes {
            self.consumed = self.consumed.checked_add(1).ok_or("PNG byte count overflowed")?;
            if self.consumed > self.total_bytes {
                return Err("PNG exceeded its sealed byte claim");
            }
            match &mut self.phase {
                PngStructurePhase::Signature { bytes, len } => {
                    bytes[usize::from(*len)] = byte;
                    *len += 1;
                    if *len == 8 {
                        if bytes != b"\x89PNG\r\n\x1a\n" {
                            return Err("PNG signature was invalid");
                        }
                        self.phase = PngStructurePhase::ChunkHeader { bytes: [0; 8], len: 0 };
                    }
                }
                PngStructurePhase::ChunkHeader { bytes, len } => {
                    bytes[usize::from(*len)] = byte;
                    *len += 1;
                    if *len == 8 {
                        let length = u32::from_be_bytes(bytes[..4].try_into().expect("fixed PNG chunk length"));
                        let kind: [u8; 4] = bytes[4..8].try_into().expect("fixed PNG chunk kind");
                        let terminal_after = kind == *b"IEND";
                        if self.consumed.checked_add(length as usize).and_then(|end| end.checked_add(4)).is_none_or(|end| end > self.total_bytes) || (kind == *b"IHDR" && (self.ihdr || length != 13)) || (terminal_after && length != 0) {
                            return Err("PNG chunk exceeded fixed structure credits");
                        }
                        if length == 0 {
                            self.phase = PngStructurePhase::Crc { remaining: 4, terminal_after };
                        } else {
                            self.phase = PngStructurePhase::ChunkPayload { remaining: length, kind, ihdr: [0; 13], ihdr_len: 0 };
                        }
                    }
                }
                PngStructurePhase::ChunkPayload { remaining, kind, ihdr, ihdr_len } => {
                    if *kind == *b"IHDR" {
                        ihdr[usize::from(*ihdr_len)] = byte;
                        *ihdr_len += 1;
                    }
                    *remaining -= 1;
                    if *remaining == 0 {
                        if *kind == *b"IHDR" {
                            let width = u32::from_be_bytes(ihdr[..4].try_into().expect("fixed PNG width"));
                            let height = u32::from_be_bytes(ihdr[4..8].try_into().expect("fixed PNG height"));
                            let pixel_bytes = usize::try_from(width).ok().and_then(|width| usize::try_from(height).ok().and_then(|height| width.checked_mul(height))).and_then(|pixels| pixels.checked_mul(4));
                            if width == 0 || height == 0 || pixel_bytes.is_none_or(|bytes| bytes > RENDERER_ASSET_PIXEL_BYTES) {
                                return Err("PNG dimensions exceeded fixed pixel credits");
                            }
                            self.ihdr = true;
                        } else if *kind == *b"IDAT" {
                            self.idat = true;
                        }
                        self.phase = PngStructurePhase::Crc { remaining: 4, terminal_after: false };
                    }
                }
                PngStructurePhase::Crc { remaining, terminal_after } => {
                    *remaining -= 1;
                    if *remaining == 0 {
                        self.phase = if *terminal_after { PngStructurePhase::Terminal } else { PngStructurePhase::ChunkHeader { bytes: [0; 8], len: 0 } };
                    }
                }
                PngStructurePhase::Terminal => return Err("PNG contained trailing bytes"),
            }
        }
        Ok(())
    }

    pub fn finish(&self) -> Result<(), &'static str> {
        if self.consumed == self.total_bytes && self.ihdr && self.idat && matches!(self.phase, PngStructurePhase::Terminal) {
            Ok(())
        } else {
            Err("PNG structure ended before IHDR, IDAT, and IEND reached terminal")
        }
    }
}

enum JpegStructurePhase {
    Start { bytes: [u8; 2], len: u8 },
    MarkerPrefix,
    Marker,
    SegmentLength { marker: u8, bytes: [u8; 2], len: u8 },
    Segment { marker: u8, remaining: u16, prefix: [u8; 5], prefix_len: u8 },
    Scan,
    ScanMarker,
    Terminal,
}

pub struct JpegStructureCursor {
    total_bytes: usize,
    consumed: usize,
    pub dimensions: bool,
    pub phase: JpegStructurePhase,
}

impl JpegStructureCursor {
    pub fn new(total_bytes: usize) -> Self {
        Self { total_bytes, consumed: 0, dimensions: false, phase: JpegStructurePhase::Start { bytes: [0; 2], len: 0 } }
    }

    pub fn feed(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        for &byte in bytes {
            self.consumed = self.consumed.checked_add(1).ok_or("JPEG byte count overflowed")?;
            if self.consumed > self.total_bytes {
                return Err("JPEG exceeded its sealed byte claim");
            }
            match &mut self.phase {
                JpegStructurePhase::Start { bytes, len } => {
                    bytes[usize::from(*len)] = byte;
                    *len += 1;
                    if *len == 2 {
                        if *bytes != [0xff, 0xd8] {
                            return Err("JPEG start marker was invalid");
                        }
                        self.phase = JpegStructurePhase::MarkerPrefix;
                    }
                }
                JpegStructurePhase::MarkerPrefix => {
                    if byte != 0xff {
                        return Err("JPEG marker prefix was invalid");
                    }
                    self.phase = JpegStructurePhase::Marker;
                }
                JpegStructurePhase::Marker => match byte {
                    0xff => {}
                    0xd9 => self.phase = JpegStructurePhase::Terminal,
                    0x01 | 0xd0..=0xd7 => self.phase = JpegStructurePhase::MarkerPrefix,
                    marker => self.phase = JpegStructurePhase::SegmentLength { marker, bytes: [0; 2], len: 0 },
                },
                JpegStructurePhase::SegmentLength { marker, bytes, len } => {
                    bytes[usize::from(*len)] = byte;
                    *len += 1;
                    if *len == 2 {
                        let length = u16::from_be_bytes(*bytes);
                        if length < 2 || self.consumed.checked_add(usize::from(length - 2)).is_none_or(|end| end > self.total_bytes) {
                            return Err("JPEG segment exceeded fixed structure credits");
                        }
                        self.phase = JpegStructurePhase::Segment { marker: *marker, remaining: length - 2, prefix: [0; 5], prefix_len: 0 };
                    }
                }
                JpegStructurePhase::Segment { marker, remaining, prefix, prefix_len } => {
                    if usize::from(*prefix_len) < prefix.len() {
                        prefix[usize::from(*prefix_len)] = byte;
                        *prefix_len += 1;
                    }
                    *remaining -= 1;
                    if *remaining == 0 {
                        if matches!(*marker, 0xc0..=0xc3 | 0xc5..=0xc7 | 0xc9..=0xcb | 0xcd..=0xcf) {
                            if *prefix_len < 5 {
                                return Err("JPEG frame header was truncated");
                            }
                            let height = u16::from_be_bytes([prefix[1], prefix[2]]);
                            let width = u16::from_be_bytes([prefix[3], prefix[4]]);
                            let pixel_bytes = usize::from(width).checked_mul(usize::from(height)).and_then(|pixels| pixels.checked_mul(4));
                            if width == 0 || height == 0 || pixel_bytes.is_none_or(|bytes| bytes > RENDERER_ASSET_PIXEL_BYTES) {
                                return Err("JPEG dimensions exceeded fixed pixel credits");
                            }
                            self.dimensions = true;
                        }
                        self.phase = if *marker == 0xda { JpegStructurePhase::Scan } else { JpegStructurePhase::MarkerPrefix };
                    }
                }
                JpegStructurePhase::Scan => {
                    if byte == 0xff {
                        self.phase = JpegStructurePhase::ScanMarker;
                    }
                }
                JpegStructurePhase::ScanMarker => match byte {
                    0x00 | 0xd0..=0xd7 => self.phase = JpegStructurePhase::Scan,
                    0xff => {}
                    0xd9 => self.phase = JpegStructurePhase::Terminal,
                    marker => self.phase = JpegStructurePhase::SegmentLength { marker, bytes: [0; 2], len: 0 },
                },
                JpegStructurePhase::Terminal => return Err("JPEG contained trailing bytes"),
            }
        }
        Ok(())
    }

    pub fn finish(&self) -> Result<(), &'static str> {
        if self.consumed == self.total_bytes && self.dimensions && matches!(self.phase, JpegStructurePhase::Terminal) {
            Ok(())
        } else {
            Err("JPEG structure ended before a bounded frame and EOI reached terminal")
        }
    }
}

struct TextAssetStructureCursor {
    total_bytes: usize,
    consumed: usize,
    require_svg: bool,
    validate_utf8: bool,
    saw_open_angle: bool,
    saw_svg: bool,
    utf8_remaining: u8,
}

impl TextAssetStructureCursor {
    fn new(total_bytes: usize, require_svg: bool, validate_utf8: bool) -> Self {
        Self { total_bytes, consumed: 0, require_svg, validate_utf8, saw_open_angle: false, saw_svg: false, utf8_remaining: 0 }
    }

    fn feed(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        for &byte in bytes {
            self.consumed = self.consumed.checked_add(1).ok_or("text asset byte count overflowed")?;
            if self.consumed > self.total_bytes {
                return Err("text asset exceeded its sealed byte claim");
            }
            if !self.validate_utf8 {
                continue;
            }
            if self.utf8_remaining != 0 {
                if byte & 0xc0 != 0x80 {
                    return Err("text asset contained invalid UTF-8 continuation");
                }
                self.utf8_remaining -= 1;
            } else if byte >= 0x80 {
                self.utf8_remaining = match byte {
                    0xc2..=0xdf => 1,
                    0xe0..=0xef => 2,
                    0xf0..=0xf4 => 3,
                    _ => return Err("text asset contained invalid UTF-8 lead byte"),
                };
            }
            if byte == b'<' {
                self.saw_open_angle = true;
            } else if self.saw_open_angle && (byte == b's' || byte == b'S') {
                self.saw_svg = true;
            }
        }
        Ok(())
    }

    fn finish(&self) -> Result<(), &'static str> {
        if self.consumed == self.total_bytes && (!self.validate_utf8 || self.utf8_remaining == 0) && (!self.require_svg || self.saw_svg) {
            Ok(())
        } else {
            Err("text asset structure did not reach its bounded terminal witness")
        }
    }
}

enum ProtobufStructurePhase {
    Key { value: u64, shift: u8 },
    Varint { value: u64, shift: u8 },
    Fixed { remaining: u8 },
    Length { value: u64, shift: u8 },
    Bytes { remaining: u64 },
}

pub struct ProtobufStructureCursor {
    total_bytes: usize,
    consumed: usize,
    fields: usize,
    pub phase: ProtobufStructurePhase,
}

impl ProtobufStructureCursor {
    pub fn new(total_bytes: usize) -> Self {
        Self { total_bytes, consumed: 0, fields: 0, phase: ProtobufStructurePhase::Key { value: 0, shift: 0 } }
    }

    pub fn feed(&mut self, bytes: &[u8]) -> Result<(), &'static str> {
        for &byte in bytes {
            self.consumed = self.consumed.checked_add(1).ok_or("protobuf byte count overflowed")?;
            if self.consumed > self.total_bytes {
                return Err("protobuf exceeded its sealed byte claim");
            }
            match &mut self.phase {
                ProtobufStructurePhase::Key { value, shift } => {
                    protobuf_varint_byte(value, shift, byte)?;
                    if byte & 0x80 == 0 {
                        let key = *value;
                        let field = key >> 3;
                        let wire = (key & 7) as u8;
                        if field == 0 {
                            return Err("protobuf field zero is invalid");
                        }
                        self.fields = self.fields.checked_add(1).ok_or("protobuf field count overflowed")?;
                        self.phase = match wire {
                            0 => ProtobufStructurePhase::Varint { value: 0, shift: 0 },
                            1 => ProtobufStructurePhase::Fixed { remaining: 8 },
                            2 => ProtobufStructurePhase::Length { value: 0, shift: 0 },
                            5 => ProtobufStructurePhase::Fixed { remaining: 4 },
                            _ => return Err("protobuf wire type was unsupported"),
                        };
                    }
                }
                ProtobufStructurePhase::Varint { value, shift } => {
                    protobuf_varint_byte(value, shift, byte)?;
                    if byte & 0x80 == 0 {
                        self.phase = ProtobufStructurePhase::Key { value: 0, shift: 0 };
                    }
                }
                ProtobufStructurePhase::Fixed { remaining } => {
                    *remaining -= 1;
                    if *remaining == 0 {
                        self.phase = ProtobufStructurePhase::Key { value: 0, shift: 0 };
                    }
                }
                ProtobufStructurePhase::Length { value, shift } => {
                    protobuf_varint_byte(value, shift, byte)?;
                    if byte & 0x80 == 0 {
                        if *value > (self.total_bytes - self.consumed) as u64 {
                            return Err("protobuf length-delimited field exceeded fixed credits");
                        }
                        self.phase = if *value == 0 { ProtobufStructurePhase::Key { value: 0, shift: 0 } } else { ProtobufStructurePhase::Bytes { remaining: *value } };
                    }
                }
                ProtobufStructurePhase::Bytes { remaining } => {
                    *remaining -= 1;
                    if *remaining == 0 {
                        self.phase = ProtobufStructurePhase::Key { value: 0, shift: 0 };
                    }
                }
            }
        }
        Ok(())
    }

    pub fn finish(&self) -> Result<(), &'static str> {
        if self.consumed == self.total_bytes && self.fields != 0 && matches!(self.phase, ProtobufStructurePhase::Key { value: 0, shift: 0 }) {
            Ok(())
        } else {
            Err("protobuf structure did not reach a field boundary")
        }
    }
}

fn protobuf_varint_byte(value: &mut u64, shift: &mut u8, byte: u8) -> Result<(), &'static str> {
    if *shift >= 64 || (*shift == 63 && byte > 1) {
        return Err("protobuf varint exceeded fixed width");
    }
    *value |= u64::from(byte & 0x7f) << *shift;
    if byte & 0x80 != 0 {
        *shift += 7;
    }
    Ok(())
}

pub fn renderer_asset_svg_prefix_is_valid(prefix: &[u8]) -> bool {
    std::str::from_utf8(prefix).ok().is_some_and(|text| {
        let text = text.trim_start_matches(|character: char| character.is_ascii_whitespace());
        text.starts_with("<svg") || text.starts_with("<?xml")
    })
}

pub fn renderer_asset_ui_image_prefix_is_valid(prefix: &[u8]) -> bool {
    prefix.starts_with(b"\x89PNG\r\n\x1a\n") || prefix.starts_with(&[0xff, 0xd8, 0xff]) || renderer_asset_svg_prefix_is_valid(prefix)
}

pub fn renderer_asset_reference_prefix_is_valid(prefix: &[u8]) -> bool {
    renderer_asset_ui_image_prefix_is_valid(prefix)
        || prefix.starts_with(b"GIF87a")
        || prefix.starts_with(b"GIF89a")
        || (prefix.len() >= 12 && &prefix[..4] == b"RIFF" && &prefix[8..12] == b"WEBP")
        || prefix.starts_with(b"BM")
        || prefix.starts_with(b"II*\0")
        || prefix.starts_with(b"MM\0*")
        || prefix.starts_with(b"II+\0")
        || prefix.starts_with(b"MM\0+")
        || prefix.starts_with(b"%PDF-")
        || (prefix.len() >= 12 && &prefix[4..8] == b"ftyp")
}
//#endregion 📡️RendererAssetAuthority
