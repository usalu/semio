//! ✒️ Reads CFF1 Type 2 charstrings as vector outlines.

use super::truetype::TrueTypeOutlineSegment as CffOutlineSegment;

#[derive(Clone, Copy, Default)]
struct Index<'a> {
    offsets: &'a [u8],
    objects: &'a [u8],
    size: usize,
    count: usize,
}

impl<'a> Index<'a> {
    fn read(source: &'a [u8], start: usize) -> Option<(Self, usize)> {
        let count = usize::from(u16::from_be_bytes(source.get(start..start.checked_add(2)?)?.try_into().ok()?));
        let offset_start = start + 2;
        if count == 0 { return Some((Self::default(), offset_start)); }
        let size = usize::from(*source.get(offset_start)?);
        if !(1..=4).contains(&size) { return None; }
        let object_start = (offset_start + 1).checked_add((count + 1) * size)?;
        let offsets = source.get(offset_start + 1..object_start)?;
        if offset_at(offsets, size, 0)? != 0 { return None; }
        let end = object_start.checked_add(offset_at(offsets, size, count)?)?;
        Some((Self { offsets, objects: source.get(object_start..end)?, size, count }, end))
    }

    fn get(self, object: usize) -> Option<&'a [u8]> {
        if object >= self.count { return None; }
        self.objects.get(offset_at(self.offsets, self.size, object)?..offset_at(self.offsets, self.size, object + 1)?)
    }

    fn subroutine(self, encoded: f64) -> Option<&'a [u8]> {
        let bias = match self.count { 0..=1239 => 107, 1240..=33899 => 1131, _ => 32768 };
        self.get(usize::try_from(exact_integer(encoded)?.checked_add(bias)?).ok()?)
    }
}

fn offset_at(offsets: &[u8], size: usize, index: usize) -> Option<usize> {
    let start = index.checked_mul(size)?;
    let mut number = 0usize;
    for &byte in offsets.get(start..start.checked_add(size)?)? { number = number.checked_mul(256)?.checked_add(usize::from(byte))?; }
    number.checked_sub(1)
}

fn exact_integer(number: f64) -> Option<i32> {
    (number.fract() == 0.0 && (f64::from(i32::MIN)..=f64::from(i32::MAX)).contains(&number)).then_some(number as i32)
}

fn next_number(input: &mut &[u8], lead: u8, dictionary: bool) -> Option<f64> {
    fn take<const N: usize>(input: &mut &[u8]) -> Option<[u8; N]> {
        let (start, rest) = input.split_at_checked(N)?;
        *input = rest;
        start.try_into().ok()
    }
    Some(match lead {
        28 => f64::from(i16::from_be_bytes(take(input)?)),
        29 if dictionary => f64::from(i32::from_be_bytes(take(input)?)),
        30 if dictionary => read_real(input)?,
        32..=246 => f64::from(i32::from(lead) - 139),
        247..=250 => f64::from((i32::from(lead) - 247) * 256 + i32::from(take::<1>(input)?[0]) + 108),
        251..=254 => f64::from(-(i32::from(lead) - 251) * 256 - i32::from(take::<1>(input)?[0]) - 108),
        255 if !dictionary => f64::from(i32::from_be_bytes(take(input)?)) / 65536.0,
        _ => return None,
    })
}

fn read_real(input: &mut &[u8]) -> Option<f64> {
    let mut text = [0; 64];
    let mut length = 0;
    while let Some((&byte, rest)) = input.split_first() {
        *input = rest;
        for nibble in [byte >> 4, byte & 15] {
            let character = match nibble {
                0..=9 => b'0' + nibble,
                10 => b'.',
                11 | 12 => b'e',
                14 => b'-',
                15 => return std::str::from_utf8(&text[..length]).ok()?.parse::<f64>().ok().filter(|value| value.is_finite()),
                _ => return None,
            };
            *text.get_mut(length)? = character;
            length += 1;
            if nibble == 12 {
                *text.get_mut(length)? = b'-';
                length += 1;
            }
        }
    }
    None
}

type FontMatrix = [f64; 6];
const DEFAULT_MATRIX: FontMatrix = [0.001, 0.0, 0.0, 0.001, 0.0, 0.0];

fn matrix_point([a, b, c, d, e, f]: FontMatrix, [x, y]: [f64; 2]) -> [f64; 2] {
    [a * x + c * y + e, b * x + d * y + f]
}

#[derive(Default)]
struct Dictionary {
    char_strings: Option<usize>,
    charset: Option<usize>,
    encoding: Option<usize>,
    private: Option<(usize, usize)>,
    subrs: Option<usize>,
    fd_array: Option<usize>,
    fd_select: Option<usize>,
    cid: bool,
    matrix: Option<FontMatrix>,
}

impl Dictionary {
    fn read(mut input: &[u8]) -> Option<Self> {
        let mut dict = Self::default();
        let mut args = Vec::with_capacity(12);
        while let Some((&lead, bytes)) = input.split_first() {
            input = bytes;
            if lead >= 28 {
                if args.len() == 48 { return None; }
                args.push(next_number(&mut input, lead, true)?);
                continue;
            }
            let key = if lead == 12 {
                let (&second, rest) = input.split_first()?;
                input = rest;
                (12, second)
            } else { (lead, 0) };
            let integer = |index| usize::try_from(exact_integer(*args.get(index)?)?).ok();
            match key {
                (15, 0) => dict.charset = Some(integer(0)?),
                (16, 0) => dict.encoding = Some(integer(0)?),
                (17, 0) => dict.char_strings = Some(integer(0)?),
                (18, 0) => dict.private = Some((integer(0)?, integer(1)?)),
                (19, 0) => dict.subrs = Some(integer(0)?),
                (12, 6) if integer(0)? != 2 => return None,
                (12, 7) => dict.matrix = Some(args.as_slice().try_into().ok()?),
                (12, 30) => dict.cid = true,
                (12, 36) => dict.fd_array = Some(integer(0)?),
                (12, 37) => dict.fd_select = Some(integer(0)?),
                _ => {}
            }
            args.clear();
        }
        args.is_empty().then_some(dict)
    }
}

enum LocalIndexes<'a> {
    Shared(Index<'a>),
    PerFont { source: &'a [u8], dictionaries: Index<'a>, selection: FdSelect<'a> },
}

enum FdSelect<'a> { Direct(&'a [u8]), Ranges(&'a [u8]) }

impl<'a> FdSelect<'a> {
    fn read(input: &'a [u8], glyphs: usize) -> Option<Self> {
        let (&format, bytes) = input.split_first()?;
        match format {
            0 => Some(Self::Direct(bytes.get(..glyphs)?)),
            3 => {
                let count = usize::from(read_u16(bytes, 0)?);
                let ranges = bytes.get(2..2 + 3 * count)?;
                if count == 0 || count > glyphs || read_u16(ranges, 0)? != 0
                    || usize::from(read_u16(bytes, 2 + 3 * count)?) != glyphs { return None; }
                Some(Self::Ranges(ranges))
            }
            _ => None,
        }
    }

    fn dictionary(&self, glyph: u16) -> Option<usize> {
        match self {
            Self::Direct(values) => values.get(usize::from(glyph)).copied().map(usize::from),
            Self::Ranges(rows) => {
                let mut low = 0;
                let mut high = rows.len() / 3;
                while low + 1 < high {
                    let middle = low + (high - low) / 2;
                    if read_u16(rows, 3 * middle)? <= glyph { low = middle; } else { high = middle; }
                }
                Some(usize::from(*rows.get(3 * low + 2)?))
            }
        }
    }
}

fn read_u16(source: &[u8], at: usize) -> Option<u16> {
    Some(u16::from_be_bytes(source.get(at..at.checked_add(2)?)?.try_into().ok()?))
}

fn local_subroutines<'a>(source: &'a [u8], dictionary: &Dictionary) -> Option<Index<'a>> {
    let Some((size, offset)) = dictionary.private else { return Some(Index::default()); };
    let private = Dictionary::read(source.get(offset..offset.checked_add(size)?)?)?;
    match private.subrs {
        Some(subrs) => Some(Index::read(source, offset.checked_add(subrs)?)?.0),
        None => Some(Index::default()),
    }
}

/// ✒️ Borrows one OpenType CFF 1 font and resolves its glyph and subroutine indexes.
pub struct CffOutlineFont<'a> {
    glyphs: Index<'a>,
    global: Index<'a>,
    local: LocalIndexes<'a>,
    source: &'a [u8],
    strings: Index<'a>,
    charset: usize,
    encoding: usize,
    cid: bool,
    matrix: Option<FontMatrix>,
}

impl<'a> CffOutlineFont<'a> {
    /// 🗃️ Reads one CFF header and the indexes needed for horizontal glyph paint.
    pub fn new(source: &'a [u8]) -> Option<Self> {
        let [1, 0, size, _, ..] = *source else { return None; };
        if size < 4 { return None; }
        let (names, top_start) = Index::read(source, usize::from(size))?;
        let (top, string_start) = Index::read(source, top_start)?;
        if names.count != 1 || top.count != 1 || names.get(0)?.is_empty() { return None; }
        let dict = Dictionary::read(top.get(0)?)?;
        let (strings, global_start) = Index::read(source, string_start)?;
        let (global, _) = Index::read(source, global_start)?;
        let (glyphs, _) = Index::read(source, dict.char_strings?)?;
        let local = match (dict.fd_array, dict.fd_select) {
            (None, None) => LocalIndexes::Shared(local_subroutines(source, &dict)?),
            (Some(array), Some(select)) => LocalIndexes::PerFont {
                source, dictionaries: Index::read(source, array)?.0,
                selection: FdSelect::read(source.get(select..)?, glyphs.count)?,
            },
            _ => return None,
        };
        let charset = dict.charset.unwrap_or(0);
        if dict.cid && charset < 3 { return None; }
        Some(Self { source, strings, charset, encoding: dict.encoding.unwrap_or(0), cid: dict.cid,
            glyphs, global, local, matrix: dict.matrix })
    }

    /// 📇️ Visits the specified CID for every glyph of a CID-keyed font.
    pub fn visit_cids(&self, mut visit: impl FnMut(u16, u16)) -> Option<()> {
        if !self.cid { return None; }
        self.visit_charset(|cid, glyph| { visit(cid, glyph); Some(()) })
    }

    /// 🏷️ Visits the original glyph name of a name-keyed CFF glyph.
    pub fn visit_glyph_names(&self, mut visit: impl FnMut(&'a str, u16)) -> Option<()> {
        if self.cid { return None; }
        self.visit_charset(|sid, glyph| { visit(self.glyph_name(sid)?, glyph); Some(()) })
    }

    fn glyph_name(&self, sid: u16) -> Option<&'a str> {
        let sid = usize::from(sid);
        if let Some(&name) = STANDARD_STRINGS.get(sid) { return Some(name); }
        std::str::from_utf8(self.strings.get(sid - STANDARD_STRINGS.len())?).ok().filter(|name| !name.is_empty())
    }

    fn visit_charset(&self, mut visit: impl FnMut(u16, u16) -> Option<()>) -> Option<()> {
        match self.charset {
            0 => {
                if self.glyphs.count > 229 { return None; }
                for glyph in 0..self.glyphs.count as u16 { visit(glyph, glyph)?; }
                return Some(());
            }
            1 | 2 => {
                let table = if self.charset == 1 { EXPERT_CHARSET } else { EXPERT_SUBSET_CHARSET };
                for (glyph, &sid) in table.get(..self.glyphs.count)?.iter().enumerate() { visit(sid, glyph as u16)?; }
                return Some(());
            }
            _ => {}
        }
        let (&format, mut bytes) = self.source.get(self.charset..)?.split_first()?;
        let mut glyph = 1;
        visit(0, 0)?;
        while glyph < self.glyphs.count {
            let first = read_u16(bytes, 0)?;
            bytes = bytes.get(2..)?;
            let length = match format {
                0 => 1,
                1 => {
                    let (&remaining, rest) = bytes.split_first()?;
                    bytes = rest;
                    usize::from(remaining) + 1
                }
                2 => {
                    let remaining = read_u16(bytes, 0)?;
                    bytes = bytes.get(2..)?;
                    usize::from(remaining) + 1
                }
                _ => return None,
            };
            if length > self.glyphs.count - glyph { return None; }
            for offset in 0..length {
                visit(first.checked_add(u16::try_from(offset).ok()?)?, u16::try_from(glyph).ok()?)?;
                glyph += 1;
            }
        }
        Some(())
    }

    /// 📜️ Reads the name-keyed built-in code map, including custom supplements.
    pub fn builtin_encoding(&self) -> Option<Vec<(u8, String)>> {
        if self.cid { return None; }
        if self.encoding == 0 {
            return Some(super::STANDARD_ENCODING.iter().map(|&(code, name)| (code, name.into())).collect());
        }
        if self.encoding == 1 {
            return EXPERT_ENCODING.iter().enumerate().filter(|(_, sid)| **sid != 0)
                .map(|(code, &sid)| Some((code as u8, self.glyph_name(sid)?.into()))).collect();
        }
        let mut sids = Vec::with_capacity(self.glyphs.count);
        self.visit_charset(|sid, _| { sids.push(sid); Some(()) })?;
        let (&format, bytes) = self.source.get(self.encoding..)?.split_first()?;
        let kind = format & 127;
        if kind > 1 { return None; }
        let (&count, mut bytes) = bytes.split_first()?;
        let mut codes = [None; 256];
        let mut glyph = 1;
        for _ in 0..count {
            let (&first, rest) = bytes.split_first()?;
            bytes = rest;
            let end = match kind {
                0 => first,
                1 => {
                    let (&additional, rest) = bytes.split_first()?;
                    bytes = rest;
                    first.checked_add(additional)?
                }
                _ => return None,
            };
            for code in first..=end {
                if codes[usize::from(code)].replace(*sids.get(glyph)?).is_some() { return None; }
                glyph += 1;
            }
        }
        if format & 128 != 0 {
            let (&count, rest) = bytes.split_first()?;
            for row in rest.get(..usize::from(count) * 3)?.chunks_exact(3) {
                let sid = read_u16(row, 1)?;
                if !sids.contains(&sid) { return None; }
                codes[usize::from(row[0])] = Some(sid);
            }
        }
        codes.into_iter().enumerate().filter_map(|(code, sid)| sid.map(|sid| (code as u8, sid)))
            .map(|(code, sid)| Some((code, self.glyph_name(sid)?.into()))).collect()
    }

    /// 🖋️ Interprets one glyph's bounded Type 2 path program.
    pub fn outline(&self, glyph: u16) -> Option<Vec<CffOutlineSegment>> {
        self.read_outline(glyph, false)
    }

    /// 🧭️ Interprets a raw CFF glyph in normalized 1000-unit PDF font space.
    pub fn raw_outline(&self, glyph: u16) -> Option<Vec<CffOutlineSegment>> {
        self.read_outline(glyph, true)
    }

    fn read_outline(&self, glyph: u16, normalize: bool) -> Option<Vec<CffOutlineSegment>> {
        let (local, sub_matrix) = match &self.local {
            LocalIndexes::Shared(index) => (*index, None),
            LocalIndexes::PerFont { source, dictionaries, selection } => {
                let dictionary = Dictionary::read(dictionaries.get(selection.dictionary(glyph)?)?)?;
                (local_subroutines(source, &dictionary)?, dictionary.matrix)
            }
        };
        let mut execution = Interpreter::new(self.global, local);
        if execution.run(self.glyphs.get(usize::from(glyph))?, 0)? != Flow::End { return None; }
        let mut segments = execution.geometry.segments;
        if normalize {
            let map = |point| {
                let [x, y] = match (self.matrix, sub_matrix) {
                    (Some(top), Some(sub)) => matrix_point(top, matrix_point(sub, point)),
                    (top, sub) => matrix_point(sub.or(top).unwrap_or(DEFAULT_MATRIX), point),
                };
                let result = [x * 1000.0, y * 1000.0];
                result.iter().all(|value| value.is_finite()).then_some(result)
            };
            use CffOutlineSegment::*;
            for segment in &mut segments {
                match segment {
                    MoveTo(to) | LineTo(to) => *to = map(*to)?,
                    QuadraticTo { control, to } => { *control = map(*control)?; *to = map(*to)?; }
                    CubicTo { first, second, to } => { *first = map(*first)?; *second = map(*second)?; *to = map(*to)?; }
                    Close => {}
                }
            }
        }
        Some(segments)
    }
}

const OPERANDS: usize = 48;
const MAX_COMMANDS: usize = 65_536;
const MAX_SEGMENTS: usize = 8192;
const MAX_STEMS: usize = 96;

#[derive(PartialEq, Eq)]
enum Flow { End, Return }

struct Interpreter<'a> {
    global: Index<'a>,
    local: Index<'a>,
    stack: [f64; OPERANDS],
    stored: [Option<f64>; 32],
    len: usize,
    width_seen: bool,
    stems: usize,
    remaining: usize,
    geometry: Geometry,
}

impl<'a> Interpreter<'a> {
    fn new(global: Index<'a>, local: Index<'a>) -> Self {
        Self { global, local, stack: [0.0; OPERANDS], stored: [None; 32], len: 0, width_seen: false, stems: 0, remaining: MAX_COMMANDS, geometry: Geometry::default() }
    }

    fn run(&mut self, mut input: &[u8], depth: usize) -> Option<Flow> {
        if depth > 10 { return None; }
        while let Some((&lead, rest)) = input.split_first() {
            input = rest;
            self.remaining = self.remaining.checked_sub(1)?;
            match lead {
                28 | 32..=255 => {
                    self.push(next_number(&mut input, lead, false)?)?;
                }
                10 | 29 => {
                    let subroutine = self.pop()?;
                    let index = if lead == 10 { self.local } else { self.global };
                    if self.run(index.subroutine(subroutine)?, depth + 1)? == Flow::End { return Some(Flow::End); }
                }
                11 if depth > 0 => return Some(Flow::Return),
                14 => {
                    self.width(0);
                    if self.len != 0 { return None; }
                    self.geometry.close()?;
                    return Some(Flow::End);
                }
                1 | 3 | 18 | 19 | 20 | 23 => {
                    self.width(self.len & !1);
                    if self.len % 2 != 0 { return None; }
                    self.stems += self.len / 2;
                    self.len = 0;
                    if self.stems > MAX_STEMS { return None; }
                    if matches!(lead, 19 | 20) { input = input.get(self.stems.div_ceil(8)..)?; }
                }
                4 | 21 | 22 => {
                    let expected = if lead == 21 { 2 } else { 1 };
                    self.width(expected);
                    if self.len != expected { return None; }
                    let delta = match lead {
                        4 => [0.0, self.stack[0]],
                        22 => [self.stack[0], 0.0],
                        _ => [self.stack[0], self.stack[1]],
                    };
                    self.len = 0;
                    self.geometry.start(delta)?;
                }
                12 => {
                    let (&op, tail) = input.split_first()?;
                    input = tail;
                    if matches!(op, 34..=37) {
                        let len = self.take_operands();
                        self.geometry.flex(op, &self.stack[..len])?;
                    } else {
                        self.compute(op)?;
                    }
                }
                5..=8 | 24..=27 | 30..=31 => {
                    let len = self.take_operands();
                    self.geometry.draw(lead, &self.stack[..len])?;
                }
                _ => return None,
            }
        }
        None
    }

    fn push(&mut self, value: f64) -> Option<()> {
        if !value.is_finite() { return None; }
        *self.stack.get_mut(self.len)? = value;
        self.len += 1;
        Some(())
    }

    fn pop(&mut self) -> Option<f64> {
        self.len = self.len.checked_sub(1)?;
        Some(self.stack[self.len])
    }

    fn compute(&mut self, op: u8) -> Option<()> {
        let boolean = |value| if value { 1.0 } else { 0.0 };
        let result = match op {
            0 => return Some(()),
            3 | 4 | 10 | 11 | 12 | 15 | 24 => {
                let second = self.pop()?;
                let first = self.pop()?;
                match op {
                    3 => boolean(first != 0.0 && second != 0.0),
                    4 => boolean(first != 0.0 || second != 0.0),
                    10 => first + second,
                    11 => first - second,
                    12 => first / second,
                    15 => boolean(first == second),
                    _ => first * second,
                }
            }
            5 => boolean(self.pop()? == 0.0),
            9 => self.pop()?.abs(),
            14 => -self.pop()?,
            18 => { self.pop()?; return Some(()); }
            20 => {
                let index = usize::try_from(exact_integer(self.pop()?)?).ok()?;
                let value = self.pop()?;
                *self.stored.get_mut(index)? = Some(value);
                return Some(());
            }
            21 => {
                let index = usize::try_from(exact_integer(self.pop()?)?).ok()?;
                (*self.stored.get(index)?)?
            }
            22 => {
                let v2 = self.pop()?;
                let v1 = self.pop()?;
                let s2 = self.pop()?;
                let s1 = self.pop()?;
                if v1 <= v2 { s1 } else { s2 }
            }
            26 => self.pop()?.sqrt(),
            27 => *self.stack.get(self.len.checked_sub(1)?)?,
            28 => {
                let tail = self.len.checked_sub(2)?;
                self.stack.swap(tail, tail + 1);
                return Some(());
            }
            29 => {
                let index = exact_integer(self.pop()?)?.max(0) as usize;
                *self.stack.get(self.len.checked_sub(index + 1)?)?
            }
            30 => {
                let shift = exact_integer(self.pop()?)?;
                let count = exact_integer(self.pop()?)?;
                let slice = self.stack.get_mut(self.len.checked_sub(usize::try_from(count).ok()?)?..self.len)?;
                if count > 0 { slice.rotate_right(shift.rem_euclid(count) as usize); }
                return Some(());
            }
            _ => return None,
        };
        self.push(result)
    }

    fn take_operands(&mut self) -> usize { std::mem::take(&mut self.len) }

    fn width(&mut self, expected: usize) {
        if !self.width_seen && self.len == expected + 1 {
            self.stack.copy_within(1..self.len, 0);
            self.len -= 1;
        }
        self.width_seen = true;
    }
}

#[derive(Default)]
struct Geometry {
    position: [f64; 2],
    open: bool,
    segments: Vec<CffOutlineSegment>,
}

impl Geometry {
    fn advance(&mut self, [dx, dy]: [f64; 2]) -> Option<[f64; 2]> {
        self.position[0] += dx;
        self.position[1] += dy;
        self.position.iter().all(|component| component.is_finite()).then_some(self.position)
    }

    fn push(&mut self, segment: CffOutlineSegment) -> Option<()> {
        if self.segments.len() == MAX_SEGMENTS { return None; }
        self.segments.push(segment);
        Some(())
    }

    fn start(&mut self, delta: [f64; 2]) -> Option<()> {
        self.close()?;
        let point = self.advance(delta)?;
        self.open = true;
        self.push(CffOutlineSegment::MoveTo(point))
    }

    fn line(&mut self, delta: [f64; 2]) -> Option<()> {
        let point = self.advance(delta)?;
        self.push(CffOutlineSegment::LineTo(point))
    }

    fn curve(&mut self, first: [f64; 2], second: [f64; 2], end: [f64; 2]) -> Option<()> {
        let first = self.advance(first)?;
        let second = self.advance(second)?;
        let to = self.advance(end)?;
        self.push(CffOutlineSegment::CubicTo { first, second, to })
    }

    fn close(&mut self) -> Option<()> {
        if self.open {
            self.open = false;
            self.push(CffOutlineSegment::Close)?;
        }
        Some(())
    }

    fn curves(&mut self, args: &[f64]) -> Option<()> {
        for curve in args.chunks(6) {
            let [a, b, c, d, e, f] = *curve else { return None; };
            self.curve([a, b], [c, d], [e, f])?;
        }
        Some(())
    }

    fn lines(&mut self, args: &[f64]) -> Option<()> {
        for edge in args.chunks(2) {
            let [x, y] = *edge else { return None; };
            self.line([x, y])?;
        }
        Some(())
    }

    fn draw(&mut self, op: u8, args: &[f64]) -> Option<()> {
        if args.is_empty() || !self.open { return None; }
        match op {
            5 => self.lines(args)?,
            6 | 7 => {
                for (index, &delta) in args.iter().enumerate() {
                    self.line(if (op == 6) == (index % 2 == 0) { [delta, 0.0] } else { [0.0, delta] })?;
                }
            }
            8 => self.curves(args)?,
            24 if args.len() >= 8 => {
                let (curves, edge) = args.split_at(args.len() - 2);
                self.curves(curves)?;
                self.lines(edge)?;
            }
            25 if args.len() >= 8 => {
                let (edges, curve) = args.split_at(args.len() - 6);
                self.lines(edges)?;
                self.curves(curve)?;
            }
            26 | 27 => {
                let (prefix, curves) = args.split_at(args.len() % 4);
                if prefix.len() > 1 || curves.is_empty() { return None; }
                for (index, curve) in curves.chunks_exact(4).enumerate() {
                    let start = if index == 0 { prefix.first().copied().unwrap_or(0.0) } else { 0.0 };
                    let [a, b, c, d] = *curve else { return None; };
                    if op == 26 { self.curve([start, a], [b, c], [0.0, d])?; }
                    else { self.curve([a, start], [b, c], [d, 0.0])?; }
                }
            }
            30 | 31 => {
                let (curves, extra) = args.split_at(args.len() - args.len() % 4);
                if extra.len() > 1 || curves.is_empty() { return None; }
                let count = curves.len() / 4;
                for (index, curve) in curves.chunks_exact(4).enumerate() {
                    let end = if index + 1 == count { extra.first().copied().unwrap_or(0.0) } else { 0.0 };
                    let [a, b, c, d] = *curve else { return None; };
                    if (op == 31) == (index % 2 == 0) { self.curve([a, 0.0], [b, c], [end, d])?; }
                    else { self.curve([0.0, a], [b, c], [d, end])?; }
                }
            }
            _ => return None,
        }
        Some(())
    }

    fn flex(&mut self, op: u8, args: &[f64]) -> Option<()> {
        if !self.open { return None; }
        match (op, args) {
            (34, &[a,b,c,d,e,f,g]) => {
                self.curve([a, 0.0], [b, c], [d, 0.0])?;
                self.curve([e, 0.0], [f, -c], [g, 0.0])?;
            }
            (35, args) if args.len() == 13 => self.curves(&args[..12])?,
            (36, &[a,b,c,d,e,f,g,h,i]) => {
                self.curve([a, b], [c, d], [e, 0.0])?;
                self.curve([f, 0.0], [g, h], [i, -(b + d + h)])?;
            }
            (37, &[a,b,c,d,e,f,g,h,i,j,k]) => {
                let x = a + c + e + g + i;
                let y = b + d + f + h + j;
                let end = if x.abs() > y.abs() { [k, -y] } else { [-x, k] };
                self.curve([a, b], [c, d], [e, f])?;
                self.curve([g, h], [i, j], end)?;
            }
            _ => return None,
        }
        Some(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_name_keyed_glyph_uses_declared_names_encoding_and_matrix() {
        let case: serde_json::Value = serde_json::from_str(include_str!("🧫️contracts.json")).unwrap();
        let hex = case["font"].as_str().unwrap();
        let bytes = (0..hex.len()).step_by(2).map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).unwrap()).collect::<Vec<_>>();
        let cff = CffOutlineFont::new(&bytes).unwrap();
        let mut names = Vec::new();
        cff.visit_glyph_names(|name, _| names.push(name)).unwrap();
        assert_eq!(serde_json::json!(names), case["names"]);
        assert_eq!(serde_json::json!(cff.builtin_encoding().unwrap()), case["encoding"]);
        let path = cff.raw_outline(case["glyph"].as_u64().unwrap() as u16).unwrap();
        let path = path.into_iter().map(|segment| match segment {
            CffOutlineSegment::MoveTo(to) => serde_json::json!(["moveTo", [to]]),
            CffOutlineSegment::LineTo(to) => serde_json::json!(["lineTo", [to]]),
            CffOutlineSegment::Close => serde_json::json!(["closePath", []]),
            _ => panic!("unexpected contract outline segment"),
        }).collect::<Vec<_>>();
        assert_eq!(serde_json::json!(path), case["outline"]);
        let hex = case["expertFont"].as_str().unwrap();
        let bytes = (0..hex.len()).step_by(2).map(|offset| u8::from_str_radix(&hex[offset..offset + 2], 16).unwrap()).collect::<Vec<_>>();
        let font = CffOutlineFont::new(&bytes).unwrap();
        let entries = font.builtin_encoding().unwrap();
        for pair in case["expertEncoding"].as_array().unwrap() {
            assert_eq!(entries.iter().find(|(code, _)| u64::from(*code) == pair[0].as_u64().unwrap()).map(|(_, name)| name.as_str()), pair[1].as_str());
        }
    }

    #[test]
    fn computed_operands_are_used_in_order() {
        let mut interpreter = Interpreter::new(Index::default(), Index::default());
        let program = [149, 159, 12, 10, 139, 21, 151, 141, 12, 12, 139, 5, 14];
        assert!(interpreter.run(&program, 0) == Some(Flow::End));
        assert_eq!(interpreter.geometry.segments, [
            CffOutlineSegment::MoveTo([30.0, 0.0]),
            CffOutlineSegment::LineTo([36.0, 0.0]),
            CffOutlineSegment::Close,
        ]);
    }

    #[test]
    fn recursive_and_malformed_charstrings_are_refused() {
        let local = Index { offsets: &[1, 4], objects: &[32, 10, 11], size: 1, count: 1 };
        let mut recursive = Interpreter::new(Index::default(), local);
        assert!(recursive.run(&[32, 10, 14], 0).is_none());
        let mut malformed = Interpreter::new(Index::default(), Index::default());
        assert!(malformed.run(&[139, 139, 21, 154, 5, 14], 0).is_none());
    }
}

/// 🗂️ Standard CFF string IDs from Adobe Technical Note 5176 Appendix A.
const STANDARD_STRINGS: &[&str] = &[
    ".notdef", "space", "exclam", "quotedbl", "numbersign", "dollar", "percent", "ampersand", "quoteright",
    "parenleft", "parenright", "asterisk", "plus", "comma", "hyphen", "period", "slash", "zero",
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    "colon", "semicolon", "less", "equal", "greater", "question", "at", "A", "B",
    "C", "D", "E", "F", "G", "H", "I", "J", "K",
    "L", "M", "N", "O", "P", "Q", "R", "S", "T",
    "U", "V", "W", "X", "Y", "Z", "bracketleft", "backslash", "bracketright",
    "asciicircum", "underscore", "quoteleft", "a", "b", "c", "d", "e", "f",
    "g", "h", "i", "j", "k", "l", "m", "n", "o",
    "p", "q", "r", "s", "t", "u", "v", "w", "x",
    "y", "z", "braceleft", "bar", "braceright", "asciitilde", "exclamdown", "cent", "sterling",
    "fraction", "yen", "florin", "section", "currency", "quotesingle", "quotedblleft", "guillemotleft", "guilsinglleft",
    "guilsinglright", "fi", "fl", "endash", "dagger", "daggerdbl", "periodcentered", "paragraph", "bullet",
    "quotesinglbase", "quotedblbase", "quotedblright", "guillemotright", "ellipsis", "perthousand", "questiondown", "grave", "acute",
    "circumflex", "tilde", "macron", "breve", "dotaccent", "dieresis", "ring", "cedilla", "hungarumlaut",
    "ogonek", "caron", "emdash", "AE", "ordfeminine", "Lslash", "Oslash", "OE", "ordmasculine",
    "ae", "dotlessi", "lslash", "oslash", "oe", "germandbls", "onesuperior", "logicalnot", "mu",
    "trademark", "Eth", "onehalf", "plusminus", "Thorn", "onequarter", "divide", "brokenbar", "degree",
    "thorn", "threequarters", "twosuperior", "registered", "minus", "eth", "multiply", "threesuperior", "copyright",
    "Aacute", "Acircumflex", "Adieresis", "Agrave", "Aring", "Atilde", "Ccedilla", "Eacute", "Ecircumflex",
    "Edieresis", "Egrave", "Iacute", "Icircumflex", "Idieresis", "Igrave", "Ntilde", "Oacute", "Ocircumflex",
    "Odieresis", "Ograve", "Otilde", "Scaron", "Uacute", "Ucircumflex", "Udieresis", "Ugrave", "Yacute",
    "Ydieresis", "Zcaron", "aacute", "acircumflex", "adieresis", "agrave", "aring", "atilde", "ccedilla",
    "eacute", "ecircumflex", "edieresis", "egrave", "iacute", "icircumflex", "idieresis", "igrave", "ntilde",
    "oacute", "ocircumflex", "odieresis", "ograve", "otilde", "scaron", "uacute", "ucircumflex", "udieresis",
    "ugrave", "yacute", "ydieresis", "zcaron", "exclamsmall", "Hungarumlautsmall", "dollaroldstyle", "dollarsuperior", "ampersandsmall",
    "Acutesmall", "parenleftsuperior", "parenrightsuperior", "twodotenleader", "onedotenleader", "zerooldstyle", "oneoldstyle", "twooldstyle", "threeoldstyle",
    "fouroldstyle", "fiveoldstyle", "sixoldstyle", "sevenoldstyle", "eightoldstyle", "nineoldstyle", "commasuperior", "threequartersemdash", "periodsuperior",
    "questionsmall", "asuperior", "bsuperior", "centsuperior", "dsuperior", "esuperior", "isuperior", "lsuperior", "msuperior",
    "nsuperior", "osuperior", "rsuperior", "ssuperior", "tsuperior", "ff", "ffi", "ffl", "parenleftinferior",
    "parenrightinferior", "Circumflexsmall", "hyphensuperior", "Gravesmall", "Asmall", "Bsmall", "Csmall", "Dsmall", "Esmall",
    "Fsmall", "Gsmall", "Hsmall", "Ismall", "Jsmall", "Ksmall", "Lsmall", "Msmall", "Nsmall",
    "Osmall", "Psmall", "Qsmall", "Rsmall", "Ssmall", "Tsmall", "Usmall", "Vsmall", "Wsmall",
    "Xsmall", "Ysmall", "Zsmall", "colonmonetary", "onefitted", "rupiah", "Tildesmall", "exclamdownsmall", "centoldstyle",
    "Lslashsmall", "Scaronsmall", "Zcaronsmall", "Dieresissmall", "Brevesmall", "Caronsmall", "Dotaccentsmall", "Macronsmall", "figuredash",
    "hypheninferior", "Ogoneksmall", "Ringsmall", "Cedillasmall", "questiondownsmall", "oneeighth", "threeeighths", "fiveeighths", "seveneighths",
    "onethird", "twothirds", "zerosuperior", "foursuperior", "fivesuperior", "sixsuperior", "sevensuperior", "eightsuperior", "ninesuperior",
    "zeroinferior", "oneinferior", "twoinferior", "threeinferior", "fourinferior", "fiveinferior", "sixinferior", "seveninferior", "eightinferior",
    "nineinferior", "centinferior", "dollarinferior", "periodinferior", "commainferior", "Agravesmall", "Aacutesmall", "Acircumflexsmall", "Atildesmall",
    "Adieresissmall", "Aringsmall", "AEsmall", "Ccedillasmall", "Egravesmall", "Eacutesmall", "Ecircumflexsmall", "Edieresissmall", "Igravesmall",
    "Iacutesmall", "Icircumflexsmall", "Idieresissmall", "Ethsmall", "Ntildesmall", "Ogravesmall", "Oacutesmall", "Ocircumflexsmall", "Otildesmall",
    "Odieresissmall", "OEsmall", "Oslashsmall", "Ugravesmall", "Uacutesmall", "Ucircumflexsmall", "Udieresissmall", "Yacutesmall", "Thornsmall",
    "Ydieresissmall", "001.000", "001.001", "001.002", "001.003", "Black", "Bold", "Book", "Light",
    "Medium", "Regular", "Roman", "Semibold",
];

const EXPERT_CHARSET: &[u16] = &[
    0, 1, 229, 230, 231, 232, 233, 234, 235, 236, 237, 238, 13, 14, 15, 99,
    239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 27, 28, 249, 250, 251, 252,
    253, 254, 255, 256, 257, 258, 259, 260, 261, 262, 263, 264, 265, 266, 109, 110,
    267, 268, 269, 270, 271, 272, 273, 274, 275, 276, 277, 278, 279, 280, 281, 282,
    283, 284, 285, 286, 287, 288, 289, 290, 291, 292, 293, 294, 295, 296, 297, 298,
    299, 300, 301, 302, 303, 304, 305, 306, 307, 308, 309, 310, 311, 312, 313, 314,
    315, 316, 317, 318, 158, 155, 163, 319, 320, 321, 322, 323, 324, 325, 326, 150,
    164, 169, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339, 340,
    341, 342, 343, 344, 345, 346, 347, 348, 349, 350, 351, 352, 353, 354, 355, 356,
    357, 358, 359, 360, 361, 362, 363, 364, 365, 366, 367, 368, 369, 370, 371, 372,
    373, 374, 375, 376, 377, 378,
];

const EXPERT_SUBSET_CHARSET: &[u16] = &[
    0, 1, 231, 232, 235, 236, 237, 238, 13, 14, 15, 99, 239, 240, 241, 242,
    243, 244, 245, 246, 247, 248, 27, 28, 249, 250, 251, 253, 254, 255, 256, 257,
    258, 259, 260, 261, 262, 263, 264, 265, 266, 109, 110, 267, 268, 269, 270, 272,
    300, 301, 302, 305, 314, 315, 158, 155, 163, 320, 321, 322, 323, 324, 325, 326,
    150, 164, 169, 327, 328, 329, 330, 331, 332, 333, 334, 335, 336, 337, 338, 339,
    340, 341, 342, 343, 344, 345, 346,
];

/// 🔠 ExpertEncoding code-to-SID values from Adobe Technical Note 5176 Appendix B.
const EXPERT_ENCODING: &[u16; 256] = &[
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    1, 229, 230, 0, 231, 232, 233, 234, 235, 236, 237, 238, 13, 14, 15, 99,
    239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 27, 28, 249, 250, 251, 252,
    0, 253, 254, 255, 256, 257, 0, 0, 0, 258, 0, 0, 259, 260, 261, 262,
    0, 0, 263, 264, 265, 0, 266, 109, 110, 267, 268, 269, 0, 270, 271, 272,
    273, 274, 275, 276, 277, 278, 279, 280, 281, 282, 283, 284, 285, 286, 287, 288,
    289, 290, 291, 292, 293, 294, 295, 296, 297, 298, 299, 300, 301, 302, 303, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 304, 305, 306, 0, 0, 307, 308, 309, 310, 311, 0, 312, 0, 0, 313,
    0, 0, 314, 315, 0, 0, 316, 317, 318, 0, 0, 0, 158, 155, 163, 319,
    320, 321, 322, 323, 324, 325, 0, 0, 326, 150, 164, 169, 327, 328, 329, 330,
    331, 332, 333, 334, 335, 336, 337, 338, 339, 340, 341, 342, 343, 344, 345, 346,
    347, 348, 349, 350, 351, 352, 353, 354, 355, 356, 357, 358, 359, 360, 361, 362,
    363, 364, 365, 366, 367, 368, 369, 370, 371, 372, 373, 374, 375, 376, 377, 378,
];
