//! 🧵️ Resolves unhinted TrueType glyph contours, including compound component placement.

use super::TrueTypeFont;

const MAX_POINTS: usize = 8_192;
const MAX_COMPONENTS: usize = 256;
const MAX_DEPTH: usize = 24;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TrueTypeOutlineSegment {
    MoveTo([f64; 2]),
    LineTo([f64; 2]),
    QuadraticTo { control: [f64; 2], to: [f64; 2] },
    Close,
}

#[derive(Clone, Copy)]
struct Point { position: [f64; 2], on_curve: bool }

#[derive(Default)]
struct Outline { points: Vec<Point>, ends: Vec<usize> }

struct Limit { points: usize, components: usize, ancestry: [u16; MAX_DEPTH] }

impl TrueTypeFont {
    /// ✒️ Returns the unhinted design-space outline at an original glyph identifier.
    pub fn glyph_outline(&self, glyph: u16) -> Option<Vec<TrueTypeOutlineSegment>> {
        let mut budget = Limit { points: MAX_POINTS, components: MAX_COMPONENTS, ancestry: [0; MAX_DEPTH] };
        let path = outline(self, glyph, 0, &mut budget)?;
        let mut output = Vec::new();
        let mut from = 0;
        for end in path.ends {
            contour(&path.points[from..end], &mut output);
            from = end;
        }
        Some(output)
    }
}

fn outline(font: &TrueTypeFont, glyph: u16, depth: usize, limit: &mut Limit) -> Option<Outline> {
    if glyph >= font.num_glyphs || depth == MAX_DEPTH || limit.ancestry[..depth].contains(&glyph) { return None; }
    limit.components = limit.components.checked_sub(1)?;
    limit.ancestry[depth] = glyph;
    let (start, end) = font.glyph_location(glyph)?;
    let data = font.table("glyf")?.get(start..end)?;
    if data.is_empty() { return Some(Outline::default()); }
    let mut reader = Reader(data);
    let contours = reader.signed()?;
    reader.skip(8)?;
    match contours {
        0 => Some(Outline::default()),
        1..=i16::MAX => simple(reader, contours as usize, limit),
        -1 => compound(font, reader, depth, limit),
        _ => None,
    }
}

fn simple(mut read: Reader<'_>, contours: usize, limit: &mut Limit) -> Option<Outline> {
    if contours > limit.points { return None; }
    let mut ends = Vec::with_capacity(contours);
    for _ in 0..contours {
        let end = read.word()? as usize + 1;
        if ends.last().is_some_and(|previous| *previous >= end) { return None; }
        ends.push(end);
    }
    let count = *ends.last()?;
    limit.points = limit.points.checked_sub(count)?;
    let length = read.word()? as usize;
    read.skip(length)?;
    let mut flags = Vec::with_capacity(count);
    while flags.len() < count {
        let flag = read.byte()?;
        let repeat = if flag & 8 != 0 { read.byte()? as usize + 1 } else { 1 };
        let next = flags.len() + repeat;
        if next > count { return None; }
        flags.resize(next, flag);
    }
    let mut points = Vec::with_capacity(count);
    let mut x = 0i32;
    let mut y = 0i32;
    for &flag in &flags {
        x += delta(&mut read, flag, 2, 16)?;
        points.push(Point { position: [f64::from(x), 0.0], on_curve: flag & 1 != 0 });
    }
    for (&flag, point) in flags.iter().zip(&mut points) {
        y += delta(&mut read, flag, 4, 32)?;
        point.position[1] = f64::from(y);
    }
    Some(Outline { points, ends })
}

fn delta(read: &mut Reader<'_>, flag: u8, short: u8, same: u8) -> Option<i32> {
    let positive = flag & same != 0;
    if flag & short != 0 {
        let magnitude = i32::from(read.byte()?);
        return Some(if positive { magnitude } else { -magnitude });
    }
    if positive { Some(0) } else { read.signed().map(i32::from) }
}

fn compound(font: &TrueTypeFont, mut read: Reader<'_>, depth: usize, limit: &mut Limit) -> Option<Outline> {
    let mut parent = Outline::default();
    loop {
        let flags = read.word()?;
        let glyph = read.word()?;
        let words = flags & 1 != 0;
        let xy = flags & 2 != 0;
        let arguments = [read.argument(words, xy)?, read.argument(words, xy)?];
        let matrix = match flags & 0x00c8 {
            0x0000 => [1.0, 0.0, 0.0, 1.0],
            0x0008 => { let scale = read.scale()?; [scale, 0.0, 0.0, scale] }
            0x0040 => [read.scale()?, 0.0, 0.0, read.scale()?],
            0x0080 => [read.scale()?, read.scale()?, read.scale()?, read.scale()?],
            _ => return None,
        };
        let transform = |[x, y]: [f64; 2]| [x * matrix[0] + y * matrix[2], x * matrix[1] + y * matrix[3]];
        let mut child = outline(font, glyph, depth + 1, limit)?;
        let offset = if xy {
            let offset = arguments.map(f64::from);
            if flags & 0x1800 == 0x0800 { transform(offset) } else { offset }
        } else {
            let [x, y] = parent.points.get(arguments[0] as usize)?.position;
            let [u, v] = transform(child.points.get(arguments[1] as usize)?.position);
            [x - u, y - v]
        };
        let start = parent.points.len();
        for point in &mut child.points {
            let [x, y] = transform(point.position);
            point.position = [x + offset[0], y + offset[1]];
        }
        parent.points.append(&mut child.points);
        parent.ends.extend(child.ends.into_iter().map(|end| start + end));
        if flags & 0x20 == 0 { return Some(parent); }
    }
}

fn contour(points: &[Point], output: &mut Vec<TrueTypeOutlineSegment>) {
    use TrueTypeOutlineSegment::*;
    let first = points[0];
    let last = points[points.len() - 1];
    let (start, body) = if first.on_curve { (first.position, &points[1..]) }
        else if last.on_curve { (last.position, &points[..points.len() - 1]) }
        else { (middle(first.position, last.position), points) };
    output.push(MoveTo(start));
    let mut pending = None;
    for point in body {
        match (pending, point.on_curve) {
            (None, true) => output.push(LineTo(point.position)),
            (Some(control), true) => {
                output.push(QuadraticTo { control, to: point.position });
                pending = None;
            }
            (Some(control), false) => {
                output.push(QuadraticTo { control, to: middle(control, point.position) });
                pending = Some(point.position);
            }
            (None, false) => pending = Some(point.position),
        }
    }
    if let Some(control) = pending { output.push(QuadraticTo { control, to: start }); }
    output.push(Close);
}

fn middle([x, y]: [f64; 2], [u, v]: [f64; 2]) -> [f64; 2] { [(x + u) / 2.0, (y + v) / 2.0] }

struct Reader<'a>(&'a [u8]);

impl Reader<'_> {
    fn byte(&mut self) -> Option<u8> { let (&byte, rest) = self.0.split_first()?; self.0 = rest; Some(byte) }
    fn word(&mut self) -> Option<u16> { Some(u16::from_be_bytes([self.byte()?, self.byte()?])) }
    fn signed(&mut self) -> Option<i16> { self.word().map(|value| value as i16) }
    fn scale(&mut self) -> Option<f64> { Some(f64::from(self.signed()?) / 16384.0) }
    fn skip(&mut self, length: usize) -> Option<()> { self.0 = self.0.get(length..)?; Some(()) }
    fn argument(&mut self, words: bool, signed: bool) -> Option<i32> {
        Some(match (words, signed) {
            (true, true) => i32::from(self.signed()?),
            (true, false) => i32::from(self.word()?),
            (false, true) => i32::from(self.byte()? as i8),
            (false, false) => i32::from(self.byte()?),
        })
    }
}
