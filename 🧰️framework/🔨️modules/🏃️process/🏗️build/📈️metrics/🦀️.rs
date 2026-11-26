//! 📈️ Reader of the multi-resolution metrics rings written by the TypeScript sampler. It reads the files `t0.ring`, `t1.ring`, ... of one directory into the merged, non-uniform series that `readSeriesV1` produces: dense recent, coarse old, no overlap. The format is documented in `./🧬️schema/🔣️.json` (`Ring` and `Point`); this crate uses the standard library only.

use std::{
    fmt,
    fs::File,
    io::{self, Read, Seek, SeekFrom},
    path::Path,
};

const MAGIC: &[u8; 4] = b"SMRG";
const VERSION: u16 = 1;
const HEADER_BYTES: usize = 64;
const SLOT_BYTES: usize = 36;
const STAMP_BYTES: usize = 8;
const MAXIMUM_METRICS: usize = 4096;
const MAXIMUM_TIERS: u16 = 16;

/// 📍️ One point of a merged series: the bucket `[start, start + width)` of tier `tier` with its `count`, `min`, `max`, `sum` (mean is `sum / count`) and `last`. A `partial` point is a coarse bucket that straddles the start of the finer tier's coverage: `width` is only its visible part before that start, the statistics are those of the whole bucket. @see ./🧬️schema/🔣️.json
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub start: u64,
    pub width: u64,
    pub tier: u16,
    pub count: u32,
    pub min: f64,
    pub max: f64,
    pub sum: f64,
    pub last: f64,
    pub partial: bool,
}

/// 🗂️ The merged series of the requested metrics, in the column order of the files, each metric with its points ordered by start. Metrics without a point are listed with an empty vector.
#[derive(Debug, Clone, PartialEq)]
pub struct Series {
    pub from_ms: u64,
    pub to_ms: u64,
    pub metrics: Vec<(String, Vec<Point>)>,
}

/// 🚫️ Why a series could not be read; the texts are English first, German second.
#[derive(Debug)]
pub enum MetricsError {
    Range { from_ms: u64, to_ms: u64 },
    Unknown(Vec<String>),
    Io(io::Error),
}

impl fmt::Display for MetricsError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Range { from_ms, to_ms } => write!(
                formatter,
                "The range from {from_ms} to {to_ms} is empty or reversed. / Der Bereich von {from_ms} bis {to_ms} ist leer oder umgekehrt."
            ),
            Self::Unknown(ids) => write!(
                formatter,
                "The metric ids {0} are not in the rings. / Die Metrik-Kennungen {0} stehen nicht in den Ringen.",
                ids.join(", ")
            ),
            Self::Io(error) => write!(formatter, "{error}"),
        }
    }
}

impl std::error::Error for MetricsError {}

impl From<io::Error> for MetricsError {
    fn from(error: io::Error) -> Self {
        Self::Io(error)
    }
}

struct Header {
    index: u16,
    bucket_ms: u64,
    buckets: u64,
    metrics: usize,
    row_bytes: usize,
    data_offset: u64,
    head: u64,
    digest: u64,
    ids: Vec<String>,
}

struct Tier {
    file: File,
    header: Header,
}

fn u32_at(bytes: &[u8], at: usize) -> u32 {
    u32::from_le_bytes([bytes[at], bytes[at + 1], bytes[at + 2], bytes[at + 3]])
}

fn u64_at(bytes: &[u8], at: usize) -> u64 {
    u64::from(u32_at(bytes, at)) | (u64::from(u32_at(bytes, at + 4)) << 32)
}

fn f64_at(bytes: &[u8], at: usize) -> f64 {
    let mut raw = [0_u8; 8];
    raw.copy_from_slice(&bytes[at..at + 8]);
    f64::from_le_bytes(raw)
}

fn read_at(file: &mut File, at: u64, length: usize) -> io::Result<Vec<u8>> {
    let mut buffer = vec![0_u8; length];
    file.seek(SeekFrom::Start(at))?;
    file.read_exact(&mut buffer)?;
    Ok(buffer)
}

fn digest(raw: &[u8]) -> u64 {
    raw.iter().fold(0xcbf2_9ce4_8422_2325_u64, |hash, byte| (hash ^ u64::from(*byte)).wrapping_mul(0x0100_0000_01b3))
}

fn read_header(file: &mut File) -> Option<Header> {
    let fixed = read_at(file, 0, HEADER_BYTES).ok()?;
    if &fixed[..4] != MAGIC || u16::from_le_bytes([fixed[4], fixed[5]]) != VERSION {
        return None;
    }
    let metrics = u32_at(&fixed, 16) as usize;
    let ids_bytes = u32_at(&fixed, 20) as usize;
    let row_bytes = u32_at(&fixed, 24) as usize;
    let data_offset = u64::from(u32_at(&fixed, 28));
    let buckets = u64::from(u32_at(&fixed, 12));
    let bucket_ms = u64::from(u32_at(&fixed, 8));
    if metrics > MAXIMUM_METRICS || row_bytes != 2 * STAMP_BYTES + metrics * SLOT_BYTES || buckets == 0 || bucket_ms == 0 || data_offset < (HEADER_BYTES + ids_bytes) as u64 {
        return None;
    }
    let raw = read_at(file, HEADER_BYTES as u64, ids_bytes).ok()?;
    let ids: Vec<String> = if metrics == 0 { Vec::new() } else { String::from_utf8(raw.clone()).ok()?.split('\n').map(str::to_owned).collect() };
    if ids.len() != metrics || file.metadata().ok()?.len() != data_offset + buckets * row_bytes as u64 {
        return None;
    }
    Some(Header {
        index: u16::from_le_bytes([fixed[6], fixed[7]]),
        bucket_ms,
        buckets,
        metrics,
        row_bytes,
        data_offset,
        head: u64_at(&fixed, 40),
        digest: digest(&raw),
        ids,
    })
}

fn row_start(buffer: &[u8], at: usize, header: &Header, expected: Option<u64>) -> u64 {
    let start = u64_at(buffer, at);
    let tail = u64_at(buffer, at + header.row_bytes - STAMP_BYTES);
    if start == 0 || start != tail || start % header.bucket_ms != 0 || expected.is_some_and(|wanted| wanted != start) {
        return 0;
    }
    start
}

fn read_row(file: &mut File, header: &Header, start: u64) -> Option<()> {
    if start == 0 || start % header.bucket_ms != 0 {
        return None;
    }
    let at = header.data_offset + (start / header.bucket_ms % header.buckets) * header.row_bytes as u64;
    let buffer = read_at(file, at, header.row_bytes).ok()?;
    (row_start(&buffer, 0, header, Some(start)) != 0).then_some(())
}

fn scan_head(file: &mut File, header: &Header) -> u64 {
    let size = header.buckets as usize * header.row_bytes;
    let Ok(buffer) = read_at(file, header.data_offset, size) else { return 0 };
    let mut head = 0;
    for index in 0..header.buckets as usize {
        let start = row_start(&buffer, index * header.row_bytes, header, None);
        if start > head && (start / header.bucket_ms % header.buckets) as usize == index {
            head = start;
        }
    }
    head
}

fn head_of(file: &mut File, header: &Header) -> u64 {
    let mut head = header.head;
    if head == 0 || read_row(file, header, head).is_none() {
        return scan_head(file, header);
    }
    for _ in 0..header.buckets {
        if read_row(file, header, head + header.bucket_ms).is_none() {
            break;
        }
        head += header.bucket_ms;
    }
    head
}

fn discover(directory: &Path) -> Vec<Tier> {
    let mut tiers: Vec<Tier> = Vec::new();
    for index in 0..MAXIMUM_TIERS {
        let Ok(mut file) = File::open(directory.join(format!("t{index}.ring"))) else { continue };
        let Some(header) = read_header(&mut file) else { continue };
        if header.index != index || tiers.first().is_some_and(|first| first.header.digest != header.digest) {
            continue;
        }
        tiers.push(Tier { file, header });
    }
    tiers
}

struct Chunk {
    tier: u16,
    rows: Vec<(u64, u64, bool, Vec<[f64; 5]>)>,
}

fn slots_of(buffer: &[u8], at: usize, metrics: usize) -> Vec<[f64; 5]> {
    (0..metrics)
        .map(|metric| {
            let slot = at + STAMP_BYTES + metric * SLOT_BYTES;
            let count = u32_at(buffer, slot);
            if count == 0 {
                [0.0; 5]
            } else {
                [f64::from(count), f64_at(buffer, slot + 4), f64_at(buffer, slot + 12), f64_at(buffer, slot + 20), f64_at(buffer, slot + 28)]
            }
        })
        .collect()
}

/// 📖️ Reads `[from_ms, to_ms)` from the ring files of `directory` as one merged series: the finest tier answers the newest part, each coarser tier only the part older than the oldest row of the finer ones, so points never overlap and get coarser with age. `ids` narrows the metrics (all by default). Missing or foreign tier files are skipped, a directory without rings gives an empty series. It is the same reading as `readSeriesV1` of the TypeScript sampler and the Python oracle. @see https://doc.rust-lang.org/std/fs/struct.File.html
pub fn read_series(directory: &Path, from_ms: u64, to_ms: u64, ids: Option<&[String]>) -> Result<Series, MetricsError> {
    if to_ms <= from_ms {
        return Err(MetricsError::Range { from_ms, to_ms });
    }
    let mut tiers = discover(directory);
    let known: Vec<String> = tiers.first().map(|first| first.header.ids.clone()).unwrap_or_default();
    let wanted: Vec<String> = ids.map(<[String]>::to_vec).unwrap_or_else(|| known.clone());
    let unknown: Vec<String> = wanted.iter().filter(|id| !known.contains(id)).cloned().collect();
    if !unknown.is_empty() {
        return Err(MetricsError::Unknown(unknown));
    }
    let columns: Vec<usize> = wanted.iter().filter_map(|id| known.iter().position(|candidate| candidate == id)).collect();
    let mut chunks: Vec<Chunk> = Vec::new();
    let mut boundary = to_ms;
    let mut covered = false;
    for tier in &mut tiers {
        let width = tier.header.bucket_ms;
        let head = head_of(&mut tier.file, &tier.header);
        if head == 0 {
            continue;
        }
        let horizon = (head + width).saturating_sub(tier.header.buckets * width);
        let low = (from_ms / width * width).max(horizon);
        if boundary < width {
            continue;
        }
        let straddle = if covered && boundary % width != 0 { Some(boundary / width * width) } else { None };
        let high = head.min(straddle.unwrap_or((boundary - width) / width * width));
        if high < low {
            continue;
        }
        let count = ((high - low) / width + 1) as usize;
        let row_bytes = tier.header.row_bytes;
        let first = (low / width % tier.header.buckets) as usize;
        let run = count.min(tier.header.buckets as usize - first);
        let mut buffer = read_at(&mut tier.file, tier.header.data_offset + (first * row_bytes) as u64, run * row_bytes)?;
        if run < count {
            buffer.extend(read_at(&mut tier.file, tier.header.data_offset, (count - run) * row_bytes)?);
        }
        let mut rows = Vec::new();
        for step in 0..count {
            let start = low + step as u64 * width;
            let at = step * row_bytes;
            if row_start(&buffer, at, &tier.header, Some(start)) != 0 {
                let clipped = straddle == Some(start);
                rows.push((start, if clipped { boundary - start } else { width }, clipped, slots_of(&buffer, at, tier.header.metrics)));
            }
        }
        if let Some((oldest, ..)) = rows.first() {
            boundary = boundary.min(*oldest);
            covered = true;
            chunks.push(Chunk { tier: tier.header.index, rows });
        }
    }
    let metrics = wanted
        .iter()
        .zip(&columns)
        .map(|(id, column)| {
            let points = chunks
                .iter()
                .rev()
                .flat_map(|chunk| {
                    chunk.rows.iter().filter_map(move |(start, visible, partial, slots)| {
                        let [count, min, max, sum, last] = slots[*column];
                        (count > 0.0).then_some(Point { start: *start, width: *visible, tier: chunk.tier, count: count as u32, min, max, sum, last, partial: *partial })
                    })
                })
                .collect();
            (id.clone(), points)
        })
        .collect();
    Ok(Series { from_ms, to_ms, metrics })
}
