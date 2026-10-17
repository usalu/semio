//! 🔎️ Header parsing and decode entry points of the JPEG codec: the JFIF APP0 parser, the byte source, the decode entry points, the header record and the marker-segment header parser. See [`super`].

use super::*;

//#region Decode
/// 🏷️ Attempts to parse `seg` (the APP0 segment body, i.e. everything after the 2-byte length
/// field) as a real JFIF header (ITU-T T.871 §: 5-byte `"JFIF\0"` identifier + version + units +
/// x/y density + thumbnail dims + optional embedded RGB thumbnail). `None` if the identifier
/// doesn't match — a non-JFIF APP0 (e.g. a bare Exif/other APP0) is retained verbatim in
/// `other_segments` instead by the caller.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn parse_jfif_app0(seg: &[u8]) -> Option<(JfifVersion, JfifDensityUnits, u16, u16, Option<JfifThumbnail>)> {
    if seg.len() < 14 || &seg[0..5] != b"JFIF\0" {
        return None;
    }
    let version = (seg[5], seg[6]);
    let units = JfifDensityUnits::from_u8(seg[7]).ok()?;
    let x_density = ((seg[8] as u16) << 8) | seg[9] as u16;
    let y_density = ((seg[10] as u16) << 8) | seg[11] as u16;
    let tw = seg[12];
    let th = seg[13];
    let need = 3 * tw as usize * th as usize;
    let thumbnail = if tw > 0 && th > 0 && seg.len() >= 14 + need { Some(JfifThumbnail { width: tw, height: th, rgb_data: seg[14..14 + need].to_vec() }) } else { None };
    Some((version, units, x_density, y_density, thumbnail))
}

type JfifVersion = (u8, u8);

/// 🧩️ Random-access compressed JPEG source; implementations may be chunk ropes and need
/// not join the complete compressed input into a contiguous allocation.
pub trait JpgByteSource {
    fn len(&self) -> usize;
    fn byte(&self, index: usize) -> Option<u8>;

    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl JpgByteSource for [u8] {
    fn len(&self) -> usize {
        <[u8]>::len(self)
    }

    fn byte(&self, index: usize) -> Option<u8> {
        self.get(index).copied()
    }
}

impl JpgByteSource for &[u8] {
    fn len(&self) -> usize {
        <[u8]>::len(self)
    }

    fn byte(&self, index: usize) -> Option<u8> {
        self.get(index).copied()
    }
}

fn source_range(data: &dyn JpgByteSource, at: usize, len: usize) -> Result<Vec<u8>, JpgError> {
    let end = at.checked_add(len).ok_or_else(|| JpgError::Malformed("segment range overflow".into()))?;
    if end > data.len() {
        return Err(JpgError::Malformed("segment out of bounds".into()));
    }
    (at..end).map(|index| data.byte(index).ok_or_else(|| JpgError::Malformed("segment out of bounds".into()))).collect()
}

/// 📥 Decodes baseline sequential JPEG (SOF0 only) into an RGBA raster.
/// Any other SOFn marker (progressive/extended/lossless/arithmetic) is a
/// typed `JpgError::Unsupported` naming the exact variant — never decoded.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_jpg(data: &[u8]) -> Result<JpgSnapshot, JpgError> {
    decode_jpg_source(&data)
}

/// 🧩️ Decodes a baseline JPEG directly from a bounded random-access chunk source.
pub fn decode_jpg_source(data: &dyn JpgByteSource) -> Result<JpgSnapshot, JpgError> {
    let mut decoder = JpgStepDecoder::new(data)?;
    loop {
        if let Some(snapshot) = decoder.step(data, usize::MAX)? {
            return Ok(snapshot);
        }
    }
}

/// 🧾️ Admits logical content and exact native observations as separate owners.
pub fn decode_jpg_with_observations(data:&dyn JpgByteSource)->Result<JpgDecodedDocument,JpgError>{
    let mut decoder=JpgStepDecoder::new(data)?;
    loop {if let Some(snapshot)=decoder.step(data,usize::MAX)? {let observations=decoder.take_observations().ok_or_else(||JpgError::Malformed("native JPEG observations missing after completion".into()))?;return Ok(JpgDecodedDocument{snapshot,observations});}}
}

/// 🧾️ Everything the marker segments before `SOS` declared, plus where the entropy-coded scan
/// starts — the parse half of [`JpgStepDecoder`].
pub(crate) struct JpgHeader {
    pub(crate) quant: HashMap<u8, [i32; 64]>,
    pub(crate) dc_tables: HashMap<u8, HuffTable>,
    pub(crate) ac_tables: HashMap<u8, HuffTable>,
    pub(crate) quant_tables: Vec<JpgQuantTable>,
    pub(crate) huffman_tables: Vec<JpgHuffmanTable>,
    pub(crate) other_segments: Vec<JpgSegment>,
    pub(crate) frame: JpgFrameHeader,
    pub(crate) sof_marker: u8,
    pub(crate) arithmetic_conditioning:Vec<crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::observations::JpgArithmeticConditioning>,
    pub(crate) arithmetic:bool,
    pub(crate) scan_components:Vec<JpgScanComponent>,
    pub(crate) scan_parameters:[u8;3],
    pub(crate) restart_interval_raw: u16,
    pub(crate) restart_interval: Option<u16>,
    pub(crate) jfif_version: JfifVersion,
    pub(crate) jfif_density_units: JfifDensityUnits,
    pub(crate) jfif_x_density: u16,
    pub(crate) jfif_y_density: u16,
    pub(crate) jfif_thumbnail: Option<JfifThumbnail>,
    pub(crate) scan_tabs: Vec<(usize,u8,u8)>,
    pub(crate) scan_start: usize,
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub(crate) fn parse_jpg_header(data: &dyn JpgByteSource, inspect:bool) -> Result<JpgHeader, JpgError> {
    if data.len() < 4 || data.byte(0) != Some(0xFF) || data.byte(1) != Some(0xD8) {
        return Err(JpgError::Malformed("missing SOI".into()));
    }
    let mut i = 2usize;
    let mut quant: HashMap<u8, [i32; 64]> = HashMap::new();
    let mut dc_tables: HashMap<u8, HuffTable> = HashMap::new();
    let mut ac_tables: HashMap<u8, HuffTable> = HashMap::new();
    let mut quant_tables: Vec<JpgQuantTable> = Vec::new();
    let mut huffman_tables: Vec<JpgHuffmanTable> = Vec::new();
    let mut other_segments: Vec<JpgSegment> = Vec::new();
    let mut frame: Option<JpgFrameHeader> = None;
    let mut sof_marker: u8 = 0;
    let mut arithmetic_conditioning=Vec::new();
    let mut arithmetic=false;
    let mut restart_interval_raw = 0u16;
    let mut restart_interval: Option<u16> = None;
    let mut jfif_version: JfifVersion = (1, 1);
    let mut jfif_density_units = JfifDensityUnits::Aspect;
    let mut jfif_x_density: u16 = 1;
    let mut jfif_y_density: u16 = 1;
    let mut jfif_thumbnail: Option<JfifThumbnail> = None;

    loop {
        if i + 1 >= data.len() {
            return Err(JpgError::Malformed("truncated before EOI".into()));
        }
        if data.byte(i) != Some(0xFF) {
            i += 1;
            continue;
        }
        let marker = data.byte(i.checked_add(1).ok_or_else(|| JpgError::Malformed("marker cursor overflow".into()))?).ok_or_else(|| JpgError::Malformed("truncated marker".into()))?;
        i += 2;
        match marker {
            0xD8 => continue, // stray SOI, tolerate
            0xD9 => return Err(JpgError::Malformed("EOI before SOS".into())),
            0xC0 | 0xC1 | 0xC2 | 0xC3 | 0xC5 | 0xC6 | 0xC7 | 0xC9 | 0xCA | 0xCB | 0xCD | 0xCE | 0xCF => {
                if !inspect && marker!=0xc0 {return Err(JpgError::Unsupported(format!("non-baseline frame process 0x{marker:02x}")));}
                let len = read_u16(data, i)?;
                let seg = source_range(data, i + 2, len.saturating_sub(2))?;
                if seg.len() < 6 {
                    return Err(JpgError::Malformed("SOF0 segment too short".into()));
                }
                let height = ((seg[1] as u16) << 8) | seg[2] as u16;
                let width = ((seg[3] as u16) << 8) | seg[4] as u16;
                let nc = seg[5] as usize;
                let mut components = Vec::with_capacity(nc);
                for k in 0..nc {
                    let base = 6 + k * 3;
                    if base + 2 >= seg.len() {
                        return Err(JpgError::Malformed("SOF0 component list truncated".into()));
                    }
                    components.push(JpgFrameComponent { id: seg[base], h_sampling: seg[base + 1] >> 4, v_sampling: seg[base + 1] & 0x0F, quant_table_id: seg[base + 2] });
                }
                frame = Some(JpgFrameHeader { precision: seg[0], width, height, components });
                sof_marker = marker;
                i += len;
            }
            0xDB => {
                let len = read_u16(data, i)?;
                let mut p = i + 2;
                let end = i + len;
                while p < end {
                    if p >= data.len() {
                        return Err(JpgError::Malformed("DQT truncated".into()));
                    }
                    let table_info = data.byte(p).ok_or_else(|| JpgError::Malformed("DQT truncated".into()))?;
                    let pq = table_info >> 4;
                    let tq = table_info & 0x0F;
                    if pq>1||tq>3||p+1+64*(pq as usize+1)>end{return Err(JpgError::Malformed("DQT selector, precision or declared body is invalid".into()));}
                    p += 1;
                    let mut tbl = [0i32; 64];
                    let mut tbl_u16 = [0u16; 64];
                    for (z, slot) in tbl.iter_mut().enumerate() {
                        if pq == 0 {
                            let v = data.byte(p).ok_or_else(|| JpgError::Malformed("DQT truncated".into()))?;
                            *slot = v as i32;
                            tbl_u16[z] = v as u16;
                            p += 1;
                        } else {
                            let hi = data.byte(p).ok_or_else(|| JpgError::Malformed("DQT truncated".into()))?;
                            let lo = data.byte(p.checked_add(1).ok_or_else(|| JpgError::Malformed("DQT cursor overflow".into()))?).ok_or_else(|| JpgError::Malformed("DQT truncated".into()))?;
                            let v = ((hi as u16) << 8) | lo as u16;
                            *slot = v as i32;
                            tbl_u16[z] = v;
                            p += 2;
                        }
                    }
                    quant.insert(tq, tbl);
                    quant_tables.push(JpgQuantTable { id: tq, precision: pq, values: tbl_u16 });
                }
                i += len;
            }
            0xC4 => {
                let len = read_u16(data, i)?;
                let mut p = i + 2;
                let end = i + len;
                while p < end {
                    if p + 17 > end || end>data.len() {
                        return Err(JpgError::Malformed("DHT truncated".into()));
                    }
                    let table_info = data.byte(p).ok_or_else(|| JpgError::Malformed("DHT truncated".into()))?;
                    let class = table_info >> 4;
                    let id = table_info & 0x0F;
                    if class>1||id>3{return Err(JpgError::Malformed("DHT selector exceeds its native domain".into()));}
                    p += 1;
                    let mut bits = [0u8; 16];
                    bits.copy_from_slice(&source_range(data, p, 16)?);
                    p += 16;
                    let count: usize = bits.iter().map(|&b| b as usize).sum();
                    if p+count>end{return Err(JpgError::Malformed("DHT symbols exceed the declared table segment".into()));}
                    let values = source_range(data, p, count)?;
                    p += count;
                    let table = build_huffman(&bits, &values)?;
                    let huffman_class = JpgHuffmanClass::from_u8(class).map_err(JpgError::Malformed)?;
                    if class == 0 {
                        dc_tables.insert(id, table);
                    } else {
                        ac_tables.insert(id, table);
                    }
                    huffman_tables.push(JpgHuffmanTable { id, class: huffman_class, bits, values });
                }
                i += len;
            }
            0xCC => {
                arithmetic=true;
                if !inspect {return Err(JpgError::Unsupported("arithmetic coding conditioning (DAC present)".into()));}
                let len=read_u16(data,i)?;let body=source_range(data,i+2,len.checked_sub(2).ok_or_else(||JpgError::Malformed("DAC length is invalid".into()))?)?;
                if body.len()%2!=0 {return Err(JpgError::Malformed("DAC requires selector/value pairs".into()));}
                for pair in body.chunks_exact(2){if pair[0]>>4>1||pair[0]&15>3{return Err(JpgError::Malformed("DAC selector exceeds its native domain".into()));}arithmetic_conditioning.push(crate::standards::v_jfif_1_01::subsets::document::io::binary::snapshot::observations::JpgArithmeticConditioning{selector:pair[0],value:pair[1]});}i+=len;
            }
            0xDD => {
                let len = read_u16(data, i)?;
                let seg = source_range(data, i + 2, 2)?;
                restart_interval_raw = ((seg[0] as u16) << 8) | seg[1] as u16;
                restart_interval = Some(restart_interval_raw);
                i += len;
            }
            0xDA => {
                let frame = frame.ok_or_else(|| JpgError::Malformed("SOS before SOF0".into()))?;
                let len = read_u16(data, i)?;
                let seg = source_range(data, i + 2, len.saturating_sub(2))?;
                let ns = *seg.first().ok_or_else(|| JpgError::Malformed("SOS truncated".into()))? as usize;
                if !(1..=4).contains(&ns) || seg.len()!=1+2*ns+3 {return Err(JpgError::Malformed("SOS component shape is invalid".into()));}
                if !inspect && seg[1+2*ns..]!=[0,63,0] {return Err(JpgError::Unsupported("non-baseline sequential scan parameters".into()));}
                let mut scan_tabs: Vec<(usize,u8,u8)> = Vec::with_capacity(ns);
                for k in 0..ns {
                    let base = 1 + k * 2;
                    let sel = *seg.get(base).ok_or_else(|| JpgError::Malformed("SOS truncated".into()))?;
                    let dcac = *seg.get(base + 1).ok_or_else(|| JpgError::Malformed("SOS truncated".into()))?;
                    let ci=frame.components.iter().position(|component|component.id==sel).ok_or_else(||JpgError::Malformed("SOS names an absent frame component".into()))?;
                    if scan_tabs.iter().any(|(index,_,_)|*index==ci) {return Err(JpgError::Malformed("SOS component selectors must be unique".into()));}
                    scan_tabs.push((ci,dcac >> 4,dcac & 0x0F));
                }
                if !inspect && ns != frame.components.len() {
                    return Err(JpgError::Unsupported("multi-scan (non-interleaved) baseline JPEG".into()));
                }
                i += len;
                return Ok(JpgHeader {
                    quant,
                    dc_tables,
                    ac_tables,
                    quant_tables,
                    huffman_tables,
                    other_segments,
                    frame,
                    sof_marker,
                    arithmetic_conditioning,
                    arithmetic,
                    scan_components:(0..ns).map(|index|JpgScanComponent{id:seg[1+2*index],dc_table_id:seg[2+2*index]>>4,ac_table_id:seg[2+2*index]&15}).collect(),
                    scan_parameters:seg[1+2*ns..].try_into().expect("admitted SOS arity"),
                    restart_interval_raw,
                    restart_interval,
                    jfif_version,
                    jfif_density_units,
                    jfif_x_density,
                    jfif_y_density,
                    jfif_thumbnail,
                    scan_tabs,
                    scan_start: i,
                });
            }
            0xE0 => {
                let len = read_u16(data, i)?;
                let seg = source_range(data, i + 2, len.saturating_sub(2))?;
                match parse_jfif_app0(&seg) {
                    Some((version, units, xd, yd, thumb)) => {
                        jfif_version = version;
                        jfif_density_units = units;
                        jfif_x_density = xd;
                        jfif_y_density = yd;
                        jfif_thumbnail = thumb;
                    }
                    None => other_segments.push(JpgSegment { marker, data: seg }),
                }
                i += len;
            }
            0xE1..=0xEF | 0xFE => {
                let len = read_u16(data, i)?;
                let seg = source_range(data, i + 2, len.saturating_sub(2))?;
                other_segments.push(JpgSegment { marker, data: seg });
                i += len;
            }
            0x01 | 0xD0..=0xD7 => {} // TEM / stray restart outside a scan: no length field, skip
            _ => return Err(JpgError::Malformed(format!("unhandled marker 0xFF{marker:02X} before SOS"))),
        }
    }
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn read_u16(data: &dyn JpgByteSource, at: usize) -> Result<usize, JpgError> {
    let hi = data.byte(at).ok_or_else(|| JpgError::Malformed("marker length truncated".into()))?;
    let lo = data.byte(at.checked_add(1).ok_or_else(|| JpgError::Malformed("marker length cursor overflow".into()))?).ok_or_else(|| JpgError::Malformed("marker length truncated".into()))?;
    Ok(((hi as usize) << 8) | lo as usize)
}
//#endregion Decode
