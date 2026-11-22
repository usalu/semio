//! 🌅️ JPEG 2000 (JPX) decoding: JP2 boxes, codestream markers, tier-2 packets, tier-1 blocks, inverse DWT and MCT.
//! [ITU-T T.800](https://www.itu.int/rec/T-REC-T.800) · [ISO 15444-1](https://www.iso.org/standard/78321.html)

#[path = "🌊️dwt/🦀️.rs"]
mod dwt;
#[path = "🧱️tier1/🦀️.rs"]
mod tier1;

use super::{Control, RasterError};

const MAX_PIXELS: u64 = 1 << 28;

/// 🖼️ A decoded JPEG 2000 image reduced to interleaved 8-bit samples.
#[derive(Clone, Debug, PartialEq)]
pub struct JpxImage {
    pub width: u32,
    pub height: u32,
    pub components: usize,
    pub samples: Vec<u8>,
    pub enumerated_colour_space: Option<u32>,
    pub bit_depth: u32,
    pub alpha_component: Option<usize>,
}

#[derive(Clone, Debug)]
struct ComponentInfo {
    precision: u32,
    signed: bool,
    dx: i64,
    dy: i64,
}

#[derive(Clone, Debug)]
struct Coding {
    levels: usize,
    xcb: u32,
    ycb: u32,
    style: u8,
    reversible: bool,
    precincts: Vec<(u32, u32)>,
}

#[derive(Clone, Debug)]
struct Quantization {
    style: u8,
    guard: i32,
    steps: Vec<(i32, i32)>,
}

#[derive(Clone, Debug)]
struct Params {
    progression: u8,
    layers: usize,
    mct: bool,
    sop: bool,
    eph: bool,
    coding: Vec<Coding>,
    quantization: Vec<Quantization>,
}

struct Reader<'a> {
    data: &'a [u8],
    position: usize,
}

fn be(data: &[u8], at: usize, count: usize) -> Result<u32, RasterError> {
    let bytes = data.get(at..at + count).ok_or(RasterError::Malformed("truncated JPEG 2000 data"))?;
    Ok(bytes.iter().fold(0, |value, byte| value << 8 | u32::from(*byte)))
}

fn ceil_div(a: i64, b: i64) -> i64 { (a + b - 1).div_euclid(b) }
fn ceil_shift(a: i64, shift: u32) -> i64 { (a + (1i64 << shift) - 1) >> shift }

fn parse_coding(data: &[u8], at: usize, scod: u8) -> Result<Coding, RasterError> {
    let levels = be(data, at, 1)? as usize;
    if levels > 32 { return Err(RasterError::Malformed("too many decomposition levels")); }
    let (xcb, ycb) = ((be(data, at + 1, 1)? & 15) + 2, (be(data, at + 2, 1)? & 15) + 2);
    let style = be(data, at + 3, 1)? as u8;
    let reversible = be(data, at + 4, 1)? == 1;
    let precincts = (0..=levels).map(|r| if scod & 1 == 1 { be(data, at + 5 + r, 1).map(|byte| (byte & 15, byte >> 4)) } else { Ok((15, 15)) }).collect::<Result<Vec<_>, _>>()?;
    Ok(Coding { levels, xcb, ycb, style, reversible, precincts })
}

fn parse_quantization(data: &[u8], at: usize, end: usize) -> Result<Quantization, RasterError> {
    let sq = be(data, at, 1)?;
    let (style, guard) = ((sq & 31) as u8, (sq >> 5) as i32);
    let mut steps = Vec::new();
    let mut cursor = at + 1;
    while cursor < end {
        if style == 0 {
            steps.push(((be(data, cursor, 1)? >> 3) as i32, 0));
            cursor += 1;
        } else {
            let value = be(data, cursor, 2)?;
            steps.push(((value >> 11) as i32, (value & 0x7FF) as i32));
            cursor += 2;
        }
    }
    Ok(Quantization { style, guard, steps })
}

struct Codestream<'a> {
    data: &'a [u8],
    x1: i64,
    y1: i64,
    x0: i64,
    y0: i64,
    tw: i64,
    th: i64,
    tx0: i64,
    ty0: i64,
    components: Vec<ComponentInfo>,
    params: Params,
    tiles: Vec<Option<(Params, Vec<u8>)>>,
}

fn parse_codestream(data: &[u8]) -> Result<Codestream<'_>, RasterError> {
    if be(data, 0, 2)? != 0xFF4F { return Err(RasterError::Malformed("missing JPEG 2000 SOC marker")); }
    let mut position = 2usize;
    let mut stream: Option<Codestream<'_>> = None;
    let mut current: Option<usize> = None;
    while position + 2 <= data.len() {
        let marker = be(data, position, 2)?;
        if marker == 0xFFD9 { break; }
        let length = be(data, position + 2, 2)? as usize;
        let body = position + 4;
        let end = position + 2 + length;
        if end > data.len() { return Err(RasterError::Malformed("truncated JPEG 2000 marker segment")); }
        match marker {
            0xFF51 => {
                let count = be(data, body + 34, 2)? as usize;
                let components = (0..count).map(|i| {
                    let at = body + 36 + 3 * i;
                    let ssiz = be(data, at, 1)?;
                    Ok(ComponentInfo { precision: (ssiz & 0x7F) + 1, signed: ssiz & 0x80 != 0, dx: i64::from(be(data, at + 1, 1)?.max(1)), dy: i64::from(be(data, at + 2, 1)?.max(1)) })
                }).collect::<Result<Vec<_>, RasterError>>()?;
                if count == 0 || count > 16384 { return Err(RasterError::Malformed("invalid JPEG 2000 component count")); }
                let [x1, y1, x0, y0, tw, th, tx0, ty0] = [2, 6, 10, 14, 18, 22, 26, 30].map(|offset| i64::from(be(data, body + offset, 4).unwrap_or(0)));
                if tw == 0 || th == 0 || x1 <= x0 || y1 <= y0 { return Err(RasterError::Malformed("invalid JPEG 2000 image geometry")); }
                if ((x1 - x0) as u64) * ((y1 - y0) as u64) > MAX_PIXELS { return Err(RasterError::TooLarge); }
                let default_coding = Coding { levels: 0, xcb: 6, ycb: 6, style: 0, reversible: true, precincts: vec![(15, 15)] };
                let params = Params { progression: 0, layers: 1, mct: false, sop: false, eph: false, coding: vec![default_coding; count], quantization: vec![Quantization { style: 0, guard: 2, steps: Vec::new() }; count] };
                let tiles = (ceil_div(x1 - tx0, tw) * ceil_div(y1 - ty0, th)) as usize;
                stream = Some(Codestream { data, x1, y1, x0, y0, tw, th, tx0, ty0, components, params, tiles: vec![None; tiles] });
            }
            0xFF52 | 0xFF53 | 0xFF5C | 0xFF5D => {
                let state = stream.as_mut().ok_or(RasterError::Malformed("coding marker before SIZ"))?;
                let target = match current { Some(tile) => state.tiles[tile].as_mut().map(|(params, _)| params), None => Some(&mut state.params) }.ok_or(RasterError::Malformed("marker outside tile"))?;
                let count = target.coding.len();
                match marker {
                    0xFF52 => {
                        let scod = be(data, body, 1)? as u8;
                        target.sop = scod & 2 != 0;
                        target.eph = scod & 4 != 0;
                        target.progression = be(data, body + 1, 1)? as u8;
                        target.layers = be(data, body + 2, 2)? as usize;
                        target.mct = be(data, body + 4, 1)? != 0;
                        let coding = parse_coding(data, body + 5, scod)?;
                        for slot in &mut target.coding { *slot = coding.clone(); }
                    }
                    0xFF53 => {
                        let wide = count > 256;
                        let component = be(data, body, if wide { 2 } else { 1 })? as usize;
                        let at = body + if wide { 2 } else { 1 };
                        let scoc = be(data, at, 1)? as u8;
                        *target.coding.get_mut(component).ok_or(RasterError::Malformed("COC component out of range"))? = parse_coding(data, at + 1, scoc)?;
                    }
                    0xFF5C => {
                        let quantization = parse_quantization(data, body, end)?;
                        for slot in &mut target.quantization { *slot = quantization.clone(); }
                    }
                    _ => {
                        let wide = count > 256;
                        let component = be(data, body, if wide { 2 } else { 1 })? as usize;
                        *target.quantization.get_mut(component).ok_or(RasterError::Malformed("QCC component out of range"))? = parse_quantization(data, body + if wide { 2 } else { 1 }, end)?;
                    }
                }
            }
            0xFF5F | 0xFF60 | 0xFF61 | 0xFF5E => return Err(RasterError::Unsupported("JPEG 2000 progression change, packed headers or region of interest")),
            0xFF90 => {
                let state = stream.as_mut().ok_or(RasterError::Malformed("SOT before SIZ"))?;
                let tile = be(data, body, 2)? as usize;
                let psot = be(data, body + 2, 4)? as usize;
                if tile >= state.tiles.len() { return Err(RasterError::Malformed("tile index out of range")); }
                if state.tiles[tile].is_none() { state.tiles[tile] = Some((state.params.clone(), Vec::new())); }
                current = Some(tile);
                let tile_end = if psot == 0 { data.len() } else { (position + psot).min(data.len()) };
                let mut cursor = end;
                loop {
                    let next = be(data, cursor, 2)?;
                    if next == 0xFF93 { break; }
                    let segment = be(data, cursor + 2, 2)? as usize;
                    let (before, after) = (cursor + 4, cursor + 2 + segment);
                    match next {
                        0xFF52 | 0xFF53 | 0xFF5C | 0xFF5D => {
                            let (params, _) = state.tiles[tile].as_mut().unwrap();
                            let count = params.coding.len();
                            match next {
                                0xFF52 => {
                                    let scod = be(data, before, 1)? as u8;
                                    params.sop = scod & 2 != 0;
                                    params.eph = scod & 4 != 0;
                                    params.progression = be(data, before + 1, 1)? as u8;
                                    params.layers = be(data, before + 2, 2)? as usize;
                                    params.mct = be(data, before + 4, 1)? != 0;
                                    let coding = parse_coding(data, before + 5, scod)?;
                                    for slot in &mut params.coding { *slot = coding.clone(); }
                                }
                                0xFF5C => { let quantization = parse_quantization(data, before, after)?; for slot in &mut params.quantization { *slot = quantization.clone(); } }
                                0xFF53 => {
                                    let wide = count > 256;
                                    let component = be(data, before, if wide { 2 } else { 1 })? as usize;
                                    let at = before + if wide { 2 } else { 1 };
                                    let scoc = be(data, at, 1)? as u8;
                                    *params.coding.get_mut(component).ok_or(RasterError::Malformed("COC component out of range"))? = parse_coding(data, at + 1, scoc)?;
                                }
                                _ => {
                                    let wide = count > 256;
                                    let component = be(data, before, if wide { 2 } else { 1 })? as usize;
                                    *params.quantization.get_mut(component).ok_or(RasterError::Malformed("QCC component out of range"))? = parse_quantization(data, before + if wide { 2 } else { 1 }, after)?;
                                }
                            }
                        }
                        0xFF5F | 0xFF60 | 0xFF61 | 0xFF5E => return Err(RasterError::Unsupported("JPEG 2000 progression change, packed headers or region of interest")),
                        _ => {}
                    }
                    cursor = after;
                }
                let body_start = cursor + 2;
                if body_start > tile_end { return Err(RasterError::Malformed("empty JPEG 2000 tile-part")); }
                state.tiles[tile].as_mut().unwrap().1.extend_from_slice(&data[body_start..tile_end]);
                position = tile_end;
                continue;
            }
            _ => {}
        }
        position = end;
    }
    stream.ok_or(RasterError::Malformed("JPEG 2000 codestream without SIZ"))
}

struct Block { x0: i64, y0: i64, x1: i64, y1: i64, cell: (i64, i64), leaf: (usize, usize), included: bool, zero_planes: u32, lblock: u32, passes: u32, segments: Vec<(Vec<u8>, u32, u32)> }

struct TagTree { levels: Vec<(usize, usize)>, nodes: Vec<(u32, u32)> }

impl TagTree {
    fn new(width: usize, height: usize) -> Self {
        let (mut levels, mut w, mut h) = (Vec::new(), width.max(1), height.max(1));
        loop {
            levels.push((w, h));
            if w == 1 && h == 1 { break; }
            w = w.div_ceil(2);
            h = h.div_ceil(2);
        }
        let total = levels.iter().map(|(w, h)| w * h).sum();
        Self { levels, nodes: vec![(u32::MAX, 0); total] }
    }

    fn decode(&mut self, x: usize, y: usize, threshold: u32, bits: &mut BitReader<'_>) -> Result<bool, RasterError> {
        let mut path = Vec::new();
        let mut offset = 0;
        for (level, &(w, _)) in self.levels.iter().enumerate() {
            path.push(offset + (y >> level) * w + (x >> level));
            offset += self.levels[level].0 * self.levels[level].1;
        }
        let mut low = 0;
        for &node in path.iter().rev() {
            if low > self.nodes[node].1 { self.nodes[node].1 = low } else { low = self.nodes[node].1 }
            while low < threshold && low < self.nodes[node].0 {
                if bits.bit()? == 1 { self.nodes[node].0 = low } else { low += 1 }
            }
            self.nodes[node].1 = low;
        }
        Ok(self.nodes[path[0]].0 < threshold)
    }
}

struct BitReader<'a> {
    data: &'a [u8],
    position: usize,
    current: u8,
    count: u32,
}

impl<'a> BitReader<'a> {
    fn bit(&mut self) -> Result<u32, RasterError> {
        if self.count == 0 {
            let previous = self.current;
            self.current = *self.data.get(self.position).ok_or(RasterError::Malformed("truncated JPEG 2000 packet header"))?;
            self.position += 1;
            self.count = if previous == 0xFF { 7 } else { 8 };
        }
        self.count -= 1;
        Ok(u32::from(self.current >> self.count) & 1)
    }

    fn bits(&mut self, count: u32) -> Result<u32, RasterError> {
        (0..count).try_fold(0, |value, _| Ok(value << 1 | self.bit()?))
    }

    fn finish(&mut self) {
        if self.current == 0xFF { self.position += 1; }
        self.count = 0;
        self.current = 0;
    }
}

struct PrecinctBand { blocks: Vec<usize>, cols: usize, inclusion: TagTree, zero: TagTree }

struct Subband { orientation: u8, x0: i64, y0: i64, x1: i64, y1: i64, blocks: Vec<Block>, precincts: Vec<PrecinctBand>, planes: i32, step: f32 }

struct Resolution { x0: i64, y0: i64, x1: i64, y1: i64, ppx: u32, ppy: u32, pw: i64, ph: i64, bands: Vec<Subband> }

fn build_resolutions(stream: &Codestream<'_>, params: &Params, component: usize, tx0: i64, ty0: i64, tx1: i64, ty1: i64) -> Result<Vec<Resolution>, RasterError> {
    let info = &stream.components[component];
    let coding = &params.coding[component];
    let quantization = &params.quantization[component];
    let (cx0, cy0, cx1, cy1) = (ceil_div(tx0, info.dx), ceil_div(ty0, info.dy), ceil_div(tx1, info.dx), ceil_div(ty1, info.dy));
    let levels = coding.levels;
    let mut resolutions = Vec::new();
    for r in 0..=levels {
        let shift = (levels - r) as u32;
        let (rx0, ry0, rx1, ry1) = (ceil_shift(cx0, shift), ceil_shift(cy0, shift), ceil_shift(cx1, shift), ceil_shift(cy1, shift));
        let (ppx, ppy) = coding.precincts[r];
        let (pw, ph) = if rx1 > rx0 && ry1 > ry0 { (ceil_shift(rx1, ppx) - (rx0 >> ppx), ceil_shift(ry1, ppy) - (ry0 >> ppy)) } else { (0, 0) };
        let orientations: &[u8] = if r == 0 { &[0] } else { &[1, 2, 3] };
        let mut bands = Vec::new();
        for &orientation in orientations {
            let nb = if r == 0 { levels as u32 } else { (levels - r + 1) as u32 };
            let (xob, yob) = (i64::from(orientation & 1), i64::from(orientation >> 1));
            let bound = |value: i64, offset: i64| if nb == 0 { value } else { ceil_shift(value - (offset << (nb - 1)), nb) };
            let (bx0, by0, bx1, by1) = (bound(cx0, xob), bound(cy0, yob), bound(cx1, xob), bound(cy1, yob));
            let band_index = if r == 0 { 0 } else { 3 * (r - 1) + orientation as usize };
            let (exponent, mantissa) = match quantization.style {
                1 => {
                    let (e0, m0) = *quantization.steps.first().ok_or(RasterError::Malformed("missing quantization step"))?;
                    (e0 - if r == 0 { 0 } else { r as i32 - 1 }, m0)
                }
                _ => *quantization.steps.get(band_index).ok_or(RasterError::Malformed("missing quantization step"))?,
            };
            let gain = [0, 1, 1, 2][orientation as usize];
            let planes = quantization.guard + exponent - 1;
            let step = if coding.reversible { 1.0 } else { (2f64.powi(info.precision as i32 + gain - exponent) * (1.0 + f64::from(mantissa) / 2048.0)) as f32 };
            let (xcb, ycb) = (coding.xcb.min(ppx - u32::from(r > 0)), coding.ycb.min(ppy - u32::from(r > 0)));
            let mut blocks = Vec::new();
            let mut precincts: Vec<PrecinctBand> = Vec::new();
            if bx1 > bx0 && by1 > by0 && pw > 0 {
                let (px, py) = (ppx - u32::from(r > 0), ppy - u32::from(r > 0));
                let (bw, bh) = (1i64 << xcb, 1i64 << ycb);
                let mut grids: Vec<Vec<usize>> = vec![Vec::new(); (pw * ph) as usize];
                let mut dims: Vec<(i64, i64, i64, i64)> = vec![(i64::MAX, i64::MIN, i64::MAX, i64::MIN); (pw * ph) as usize];
                for cy in (by0 >> ycb)..ceil_shift(by1, ycb) {
                    for cxi in (bx0 >> xcb)..ceil_shift(bx1, xcb) {
                        let (x0, y0, x1, y1) = ((cxi * bw).max(bx0), (cy * bh).max(by0), ((cxi + 1) * bw).min(bx1), ((cy + 1) * bh).min(by1));
                        let precinct = ((x0 >> px) - (rx0 >> ppx)) + pw * ((y0 >> py) - (ry0 >> ppy));
                        if precinct < 0 || precinct >= pw * ph { return Err(RasterError::Malformed("code-block outside its precinct")); }
                        let d = &mut dims[precinct as usize];
                        *d = (d.0.min(cxi), d.1.max(cxi), d.2.min(cy), d.3.max(cy));
                        grids[precinct as usize].push(blocks.len());
                        blocks.push(Block { x0, y0, x1, y1, cell: (cxi, cy), leaf: (0, 0), included: false, zero_planes: 0, lblock: 3, passes: 0, segments: Vec::new() });
                    }
                }
                for (index, members) in grids.into_iter().enumerate() {
                    let d = dims[index];
                    for &member in &members { let cell = blocks[member].cell; blocks[member].leaf = ((cell.0 - d.0) as usize, (cell.1 - d.2) as usize); }
                    let cols = if members.is_empty() { 0 } else { (d.1 - d.0 + 1) as usize };
                    let rows = if members.is_empty() { 0 } else { (d.3 - d.2 + 1) as usize };
                    precincts.push(PrecinctBand { blocks: members, cols, inclusion: TagTree::new(cols, rows), zero: TagTree::new(cols, rows) });
                }
            }
            bands.push(Subband { orientation, x0: bx0, y0: by0, x1: bx1, y1: by1, blocks, precincts, planes, step });
        }
        resolutions.push(Resolution { x0: rx0, y0: ry0, x1: rx1, y1: ry1, ppx, ppy, pw, ph, bands });
    }
    Ok(resolutions)
}

fn packet_order(stream: &Codestream<'_>, params: &Params, tiles: &[Vec<Resolution>], tx0: i64, ty0: i64, tx1: i64, ty1: i64) -> Vec<(usize, usize, usize, usize)> {
    let components = tiles.len();
    let max_res = tiles.iter().map(Vec::len).max().unwrap_or(0);
    let layers = params.layers;
    let mut order = Vec::new();
    let precincts = |c: usize, r: usize| tiles[c].get(r).map_or(0, |res| (res.pw * res.ph) as usize);
    let positions = |order: &mut Vec<(usize, usize, usize, usize)>, resolution_filter: Option<usize>, component_filter: Option<usize>, emit: &mut dyn FnMut(usize, usize, usize)| {
        let _ = order;
        let mut step_x = i64::MAX;
        let mut step_y = i64::MAX;
        for (c, resolutions) in tiles.iter().enumerate() {
            let levels = resolutions.len() - 1;
            for (r, res) in resolutions.iter().enumerate() {
                step_x = step_x.min((stream.components[c].dx << (res.ppx + (levels - r) as u32).min(40)).max(1));
                step_y = step_y.min((stream.components[c].dy << (res.ppy + (levels - r) as u32).min(40)).max(1));
            }
        }
        let mut y = ty0;
        while y < ty1 {
            let mut x = tx0;
            while x < tx1 {
                for c in 0..components {
                    if component_filter.is_some_and(|only| only != c) { continue; }
                    let levels = tiles[c].len() - 1;
                    for (r, res) in tiles[c].iter().enumerate() {
                        if resolution_filter.is_some_and(|only| only != r) { continue; }
                        if res.pw == 0 || res.ph == 0 { continue; }
                        let shift = (levels - r) as u32;
                        let (dx, dy) = (stream.components[c].dx, stream.components[c].dy);
                        let on_y = y % (dy << (res.ppy + shift)) == 0 || (y == ty0 && ((res.y0 << shift) % (1i64 << (res.ppy + shift))) != 0);
                        let on_x = x % (dx << (res.ppx + shift)) == 0 || (x == tx0 && ((res.x0 << shift) % (1i64 << (res.ppx + shift))) != 0);
                        if !on_x || !on_y { continue; }
                        let k = ((ceil_div(x, dx << shift) >> res.ppx) - (res.x0 >> res.ppx)) + res.pw * ((ceil_div(y, dy << shift) >> res.ppy) - (res.y0 >> res.ppy));
                        if k >= 0 && k < res.pw * res.ph { emit(c, r, k as usize); }
                    }
                }
                x += step_x - x.rem_euclid(step_x);
            }
            y += step_y - y.rem_euclid(step_y);
        }
    };
    match params.progression {
        0 => for l in 0..layers { for r in 0..max_res { for c in 0..components { for p in 0..precincts(c, r) { order.push((l, r, c, p)); } } } },
        1 => for r in 0..max_res { for l in 0..layers { for c in 0..components { for p in 0..precincts(c, r) { order.push((l, r, c, p)); } } } },
        2 => for r in 0..max_res {
            let mut found = Vec::new();
            positions(&mut Vec::new(), Some(r), None, &mut |c, r, p| found.push((c, r, p)));
            for (c, r, p) in found { for l in 0..layers { order.push((l, r, c, p)); } }
        },
        3 => {
            let mut found = Vec::new();
            positions(&mut Vec::new(), None, None, &mut |c, r, p| found.push((c, r, p)));
            for (c, r, p) in found { for l in 0..layers { order.push((l, r, c, p)); } }
        }
        _ => for c in 0..components {
            let mut found = Vec::new();
            positions(&mut Vec::new(), None, Some(c), &mut |c, r, p| found.push((c, r, p)));
            for (c, r, p) in found { for l in 0..layers { order.push((l, r, c, p)); } }
        },
    }
    order
}

fn read_packets(stream: &Codestream<'_>, params: &Params, tile: &mut [Vec<Resolution>], data: &[u8], geometry: (i64, i64, i64, i64), control: Control<'_>) -> Result<(), RasterError> {
    let order = packet_order(stream, params, tile, geometry.0, geometry.1, geometry.2, geometry.3);
    let mut position = 0usize;
    for (done, &(layer, r, c, p)) in order.iter().enumerate() {
        if done % 16 == 0 && !control(done as f32 / order.len().max(1) as f32) { return Err(RasterError::Cancelled); }
        if position >= data.len() { break; }
        if params.sop && data.get(position) == Some(&0xFF) && data.get(position + 1) == Some(&0x91) { position += 6; }
        let resolution = &mut tile[c][r];
        let mut bits = BitReader { data, position, current: 0, count: 0 };
        let mut contributions: Vec<(usize, usize, usize, usize)> = Vec::new();
        if bits.bit()? == 1 {
            for (band_index, band) in resolution.bands.iter_mut().enumerate() {
                let Some(precinct) = band.precincts.get_mut(p) else { continue };
                for &block_index in &precinct.blocks {
                    let block = &mut band.blocks[block_index];
                    let (leaf_x, leaf_y) = block.leaf;
                    let included = if block.included { bits.bit()? == 1 } else { precinct.inclusion.decode(leaf_x, leaf_y, layer as u32 + 1, &mut bits)? };
                    if !included { continue; }
                    if !block.included {
                        let mut threshold = 1;
                        while !precinct.zero.decode(leaf_x, leaf_y, threshold, &mut bits)? { threshold += 1; if threshold > 64 { return Err(RasterError::Malformed("invalid zero bit-plane count")); } }
                        block.zero_planes = threshold - 1;
                        block.included = true;
                    }
                    let passes = if bits.bit()? == 0 { 1 } else if bits.bit()? == 0 { 2 } else {
                        let two = bits.bits(2)?;
                        if two < 3 { 3 + two } else {
                            let five = bits.bits(5)?;
                            if five < 31 { 6 + five } else { 37 + bits.bits(7)? }
                        }
                    };
                    while bits.bit()? == 1 { block.lblock += 1; }
                    let style = params.coding[c].style;
                    let mut remaining = passes;
                    let mut pieces = Vec::new();
                    while remaining > 0 {
                        let reuse = block.segments.last().is_some_and(|(_, used, capacity)| used < capacity);
                        if !reuse { block.segments.push((Vec::new(), 0, tier1::segment_capacity(style, block.passes))); }
                        let at = block.segments.len() - 1;
                        let take = remaining.min(block.segments[at].2 - block.segments[at].1);
                        let length = bits.bits(block.lblock + (31 - take.leading_zeros()))? as usize;
                        block.segments[at].1 += take;
                        block.passes += take;
                        remaining -= take;
                        pieces.push((at, length));
                    }
                    for (at, length) in pieces { contributions.push((band_index, block_index, at, length)); }
                }
            }
        }
        bits.finish();
        position = bits.position;
        if params.eph && data.get(position) == Some(&0xFF) && data.get(position + 1) == Some(&0x92) { position += 2; }
        for (band_index, block_index, at, length) in contributions {
            let chunk = data.get(position..position + length).ok_or(RasterError::Malformed("truncated JPEG 2000 packet body"))?;
            resolution.bands[band_index].blocks[block_index].segments[at].0.extend_from_slice(chunk);
            position += length;
        }
    }
    Ok(())
}

fn decode_component(stream: &Codestream<'_>, params: &Params, component: usize, resolutions: &[Resolution], control: Control<'_>) -> Result<(Vec<f32>, usize, usize), RasterError> {
    let coding = &params.coding[component];
    let mut ll: Option<(Vec<f32>, i64, i64, usize, usize)> = None;
    let total: usize = resolutions.iter().map(|r| r.bands.iter().map(|b| b.blocks.len()).sum::<usize>()).sum();
    let mut done = 0usize;
    for (r, resolution) in resolutions.iter().enumerate() {
        let mut decoded: Vec<(Vec<f32>, usize, usize, i64, i64)> = Vec::new();
        for band in &resolution.bands {
            let (width, height) = ((band.x1 - band.x0).max(0) as usize, (band.y1 - band.y0).max(0) as usize);
            let mut samples = vec![0.0f32; width * height];
            for block in &band.blocks {
                done += 1;
                if done % 8 == 0 && !control(done as f32 / total.max(1) as f32) { return Err(RasterError::Cancelled); }
                let planes = band.planes - block.zero_planes as i32;
                if block.passes == 0 || planes <= 0 { continue; }
                let (bw, bh) = ((block.x1 - block.x0) as usize, (block.y1 - block.y0) as usize);
                let segments: Vec<(Vec<u8>, u32)> = block.segments.iter().map(|(data, used, _)| (data.clone(), *used)).collect();
                let result = tier1::decode_block(&segments, bw, bh, band.orientation, planes as u32, coding.style);
                for y in 0..bh {
                    for x in 0..bw {
                        let half = result.half_magnitudes[y * bw + x];
                        if half == 0 { continue; }
                        let magnitude = if coding.reversible && result.lowest_plane == 0 { (half / 2) as f32 } else { half as f32 / 2.0 };
                        let value = if result.negative[y * bw + x] { -magnitude } else { magnitude };
                        samples[(block.y0 - band.y0) as usize * width + (block.x0 - band.x0) as usize + y * width + x] = value * band.step;
                    }
                }
            }
            decoded.push((samples, width, height, band.x0, band.y0));
        }
        if r == 0 {
            let (samples, width, height, x0, y0) = decoded.remove(0);
            ll = Some((samples, x0, y0, width, height));
            continue;
        }
        let (previous, px0, py0, pw, ph) = ll.take().ok_or(RasterError::Malformed("missing lowest resolution"))?;
        let band_of = |index: usize| { let (samples, width, height, x0, y0) = &decoded[index]; dwt::Band { x0: *x0, y0: *y0, width: *width, height: *height, samples } };
        let low = dwt::Band { x0: px0, y0: py0, width: pw, height: ph, samples: &previous };
        let merged = dwt::inverse_level(&low, &band_of(0), &band_of(1), &band_of(2), resolution.x0, resolution.y0, resolution.x1, resolution.y1, coding.reversible);
        ll = Some((merged, resolution.x0, resolution.y0, (resolution.x1 - resolution.x0) as usize, (resolution.y1 - resolution.y0) as usize));
    }
    let (samples, _, _, width, height) = ll.ok_or(RasterError::Malformed("no resolutions"))?;
    let _ = stream;
    Ok((samples, width, height))
}

fn decode_codestream(data: &[u8], control: Control<'_>) -> Result<JpxImage, RasterError> {
    let stream = parse_codestream(data)?;
    let (width, height) = ((stream.x1 - stream.x0) as usize, (stream.y1 - stream.y0) as usize);
    let count = stream.components.len();
    let mut planes: Vec<Vec<f32>> = vec![vec![0.0; width * height]; count];
    let columns = ceil_div(stream.x1 - stream.tx0, stream.tw);
    let tile_total = stream.tiles.len();
    for (index, tile) in stream.tiles.iter().enumerate() {
        let Some((params, bytes)) = tile else { continue };
        if !control(index as f32 / tile_total as f32) { return Err(RasterError::Cancelled); }
        let (col, row) = (index as i64 % columns, index as i64 / columns);
        let (tx0, ty0) = ((stream.tx0 + col * stream.tw).max(stream.x0), (stream.ty0 + row * stream.th).max(stream.y0));
        let (tx1, ty1) = ((stream.tx0 + (col + 1) * stream.tw).min(stream.x1), (stream.ty0 + (row + 1) * stream.th).min(stream.y1));
        let mut tile_resolutions = (0..count).map(|c| build_resolutions(&stream, params, c, tx0, ty0, tx1, ty1)).collect::<Result<Vec<_>, _>>()?;
        read_packets(&stream, params, &mut tile_resolutions, bytes, (tx0, ty0, tx1, ty1), control)?;
        let mut decoded = Vec::new();
        for c in 0..count { decoded.push(decode_component(&stream, params, c, &tile_resolutions[c], control)?); }
        if params.mct && count >= 3 {
            let reversible = params.coding[0].reversible;
            for i in 0..decoded[0].0.len() {
                let (y0, y1, y2) = (decoded[0].0[i], decoded[1].0[i], decoded[2].0[i]);
                let (r, g, b) = if reversible { let g = y0 - ((y2 + y1) / 4.0).floor(); (y2 + g, g, y1 + g) } else { (y0 + 1.402 * y2, y0 - 0.34413 * y1 - 0.71414 * y2, y0 + 1.772 * y1) };
                decoded[0].0[i] = r;
                decoded[1].0[i] = g;
                decoded[2].0[i] = b;
            }
        }
        for (c, (samples, w, _h)) in decoded.iter().enumerate() {
            let info = &stream.components[c];
            let (cx0, cy0) = (ceil_div(tx0, info.dx), ceil_div(ty0, info.dy));
            let shift = if info.signed { 0.0 } else { (1u64 << (info.precision - 1)) as f32 };
            for y in ty0..ty1 {
                for x in tx0..tx1 {
                    let (sx, sy) = ((ceil_div(x, info.dx) - cx0).min(*w as i64 - 1).max(0) as usize, (ceil_div(y, info.dy) - cy0).max(0) as usize);
                    let value = samples.get(sy * w + sx).copied().unwrap_or(0.0) + shift;
                    planes[c][(y - stream.y0) as usize * width + (x - stream.x0) as usize] = value;
                }
            }
        }
    }
    let mut out = vec![0u8; width * height * count];
    for c in 0..count {
        let info = &stream.components[c];
        let maximum = ((1u64 << info.precision) - 1) as f32;
        let offset = if info.signed { (1u64 << (info.precision - 1)) as f32 } else { 0.0 };
        for i in 0..width * height {
            let value = (planes[c][i] + offset).round().clamp(0.0, maximum);
            out[i * count + c] = (value * 255.0 / maximum).round() as u8;
        }
    }
    Ok(JpxImage { width: width as u32, height: height as u32, components: count, samples: out, enumerated_colour_space: None, bit_depth: stream.components[0].precision, alpha_component: None })
}

/// 🔢️ Reads the component count of a JP2 file or codestream without decoding it.
pub fn component_count(data: &[u8]) -> Result<usize, RasterError> {
    let (codestream, _, palette, _) = locate(data)?;
    if let Some(palette) = palette { return Ok(palette.mapping.len()); }
    let at = codestream.windows(2).position(|pair| pair == [0xFF, 0x51]).ok_or(RasterError::Malformed("JPEG 2000 codestream without SIZ"))?;
    Ok(be(codestream, at + 4 + 34, 2)? as usize)
}

struct Palette { entries: Vec<Vec<u32>>, depths: Vec<u32>, mapping: Vec<(usize, u8, usize)> }

fn locate(data: &[u8]) -> Result<(&[u8], Option<u32>, Option<Palette>, Option<usize>), RasterError> {
    if data.starts_with(&[0xFF, 0x4F]) { return Ok((data, None, None, None)); }
    let mut position = 0usize;
    let (mut codestream, mut colour) = (None, None);
    let (mut table, mut mapping): (Option<(Vec<Vec<u32>>, Vec<u32>)>, Vec<(usize, u8, usize)>) = (None, Vec::new());
    let mut alpha: Option<usize> = None;
    while position + 8 <= data.len() {
        let mut length = be(data, position, 4)? as usize;
        let kind = be(data, position + 4, 4)?;
        let mut header = 8;
        if length == 1 { length = be(data, position + 12, 4)? as usize; header = 16; }
        if length == 0 { length = data.len() - position; }
        if length < header || position + length > data.len() { return Err(RasterError::Malformed("invalid JP2 box")); }
        let body = &data[position + header..position + length];
        match kind {
            0x6A703268 => {
                let mut inner = 0usize;
                while inner + 8 <= body.len() {
                    let size = be(body, inner, 4)? as usize;
                    if size < 8 || inner + size > body.len() { break; }
                    match be(body, inner + 4, 4)? {
                        0x636F6C72 if body[inner + 8] == 1 && colour.is_none() => colour = Some(be(body, inner + 11, 4)?),
                        0x70636C72 => {
                            let (count, columns) = (be(body, inner + 8, 2)? as usize, be(body, inner + 10, 1)? as usize);
                            let depths: Vec<u32> = (0..columns).map(|i| Ok((be(body, inner + 11 + i, 1)? & 0x7F) + 1)).collect::<Result<_, RasterError>>()?;
                            let mut at = inner + 11 + columns;
                            let mut entries = Vec::with_capacity(count);
                            for _ in 0..count {
                                let mut row = Vec::with_capacity(columns);
                                for depth in &depths { let width = depth.div_ceil(8) as usize; row.push(be(body, at, width)?); at += width; }
                                entries.push(row);
                            }
                            table = Some((entries, depths));
                        }
                        0x63646566 => {
                            let count = be(body, inner + 8, 2)? as usize;
                            for i in 0..count { if be(body, inner + 12 + 6 * i, 2)? == 1 { alpha = Some(be(body, inner + 10 + 6 * i, 2)? as usize); } }
                        }
                        0x636D6170 => {
                            let mut at = inner + 8;
                            while at + 4 <= inner + size { mapping.push((be(body, at, 2)? as usize, be(body, at + 2, 1)? as u8, be(body, at + 3, 1)? as usize)); at += 4; }
                        }
                        _ => {}
                    }
                    inner += size;
                }
            }
            0x6A703263 => codestream = Some(body),
            _ => {}
        }
        position += length;
    }
    let palette = table.map(|(entries, depths)| Palette { entries, depths, mapping });
    Ok((codestream.ok_or(RasterError::Malformed("JP2 file without a codestream box"))?, colour, palette, alpha))
}

/// 🌅️ Decodes a JP2 file or a raw JPEG 2000 codestream.
pub fn decode(data: &[u8], control: Control<'_>) -> Result<JpxImage, RasterError> {
    let (codestream, colour, palette, alpha) = locate(data)?;
    let mut image = decode_codestream(codestream, control)?;
    image.enumerated_colour_space = colour;
    image.alpha_component = alpha;
    if let Some(palette) = palette {
        if image.components != 1 || palette.mapping.iter().any(|&(component, kind, column)| component != 0 || kind > 1 || (kind == 1 && column >= palette.depths.len())) { return Err(RasterError::Unsupported("JP2 palette mapping")); }
        let channels = palette.mapping.len();
        let mut samples = Vec::with_capacity(image.samples.len() * channels);
        for &sample in &image.samples {
            let index = ((f64::from(sample) * f64::from((1u32 << image.bit_depth) - 1) / 255.0).round() as usize).min(palette.entries.len().saturating_sub(1));
            for &(_, kind, column) in &palette.mapping {
                samples.push(if kind == 0 { sample } else { let depth = palette.depths[column]; (f64::from(palette.entries[index][column]) * 255.0 / (f64::from((1u32 << depth) - 1))).round() as u8 });
            }
        }
        image.samples = samples;
        image.components = channels;
    }
    Ok(image)
}
