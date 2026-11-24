//! 🌊️ Writes a quoted classic PDF graph with one bounded control buffer and borrowed stream bodies.

use crate::standards::v1_7::subsets::base::{
    modules::filters::encode_stream, schema::snapshot::PdfObject,
};
use super::classic_graph_quote::{GraphLoan, GraphQuote, GraphRefusal, SourceIdentity, is_flate, preserved_trailer, stream_extent};
use super::classic_stored_flate::StoredFlate;
use crate::standards::v1_7::subsets::base::modules::lexer::{write_name, write_object, write_string};
use semio_framework_value::ByteReadSource;
use std::io::{self, Write};

#[derive(Debug)]
pub enum WriteRefusal { Graph(GraphRefusal), Sink(io::Error) }
impl From<GraphRefusal> for WriteRefusal { fn from(reason: GraphRefusal) -> Self { Self::Graph(reason) } }
impl From<io::Error> for WriteRefusal { fn from(error: io::Error) -> Self { Self::Sink(error) } }

#[derive(Clone, Copy, Default, Debug)]
pub struct GraphWriteTurn {
    pub prepared: usize,
    pub emitted: usize,
    pub done: bool,
    pub control: bool,
}

/// 📎️ How an appended section continues the retained original: its byte length, the start of its last section, the size its table reaches and whether it spells that table as a stream.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AppendFrame { pub length: usize, pub prev: usize, pub size: u32, pub stream: bool, pub separator: bool }

#[derive(Clone, Copy)]
enum Phase {
    Header, Copy, Separator, Object, Body, ObjectEnd, XrefHeader, XrefRun, XrefRow, Trailer,
    StreamOpen, StreamRun, StreamClose, StreamBody, StreamEnd, Done,
}

pub struct ClassicGraphWriter {
    source: SourceIdentity,
    limit: usize,
    offsets: Vec<u32>,
    prefix: usize,
    buffer: Vec<u8>,
    buffer_at: usize,
    object: usize,
    body_at: usize,
    flate: Option<StoredFlate>,
    output_at: usize,
    xref_at: usize,
    xref_row: u32,
    xref_object: usize,
    append: Option<AppendFrame>,
    copy_at: usize,
    run_left: u32,
    stream_at: usize,
    phase: Phase,
}

impl ClassicGraphWriter {
    /// 🌱 The caller donates one empty paid control buffer of the quoted prefix and one paid offset table of one entry per object.
    pub fn start(quote: &GraphQuote, buffer: Vec<u8>, mut offsets: Vec<u32>) -> Result<Self, (GraphRefusal, Vec<u8>, Vec<u32>)> {
        let Some(limit) = quote.maximum() else { return Err((GraphRefusal::NotQuoted, buffer, offsets)); };
        if !buffer.is_empty() || buffer.capacity() < quote.prefix() || !offsets.is_empty() || offsets.capacity() < quote.objects() {
            return Err((GraphRefusal::ControlExtent, buffer, offsets));
        }
        offsets.resize(quote.objects(), 0);
        Ok(Self { source: quote.source(), limit, offsets, prefix: quote.prefix(), buffer,
            buffer_at: 0, object: 0, body_at: 0, flate: None, output_at: 0, xref_at: 0,
            xref_row: 0, xref_object: 0, append: None, copy_at: 0, run_left: 0, stream_at: 0, phase: Phase::Header })
    }

    /// 📎️ Starts a writer that first copies the retained original unchanged, then appends the quoted changed and new objects, one new table section and a trailer linking back to the original's last section.
    pub fn start_append(quote: &GraphQuote, buffer: Vec<u8>, offsets: Vec<u32>, frame: AppendFrame) -> Result<Self, (GraphRefusal, Vec<u8>, Vec<u32>)> {
        let mut writer = Self::start(quote, buffer, offsets)?;
        writer.append = Some(frame);
        writer.phase = Phase::Copy;
        Ok(writer)
    }

    /// 🔗 Advances one control action or emits one bounded physical prefix.
    pub fn step(&mut self, graph: GraphLoan<'_>, sink: &mut dyn Write, grant: usize, original: Option<&dyn ByteReadSource>) -> Result<GraphWriteTurn, WriteRefusal> {
        if self.source != graph.identity() { return Err(GraphRefusal::SourceChanged.into()); }
        let mut result = GraphWriteTurn::default();
        if !self.buffer.is_empty() {
            let count = try_write(sink, &self.buffer[self.buffer_at..], grant, self.limit - self.output_at)?;
            result.emitted = count;
            self.buffer_at += count;
            self.output_at += count;
            if self.buffer_at == self.buffer.len() { self.buffer.clear(); self.buffer_at = 0; }
            return Ok(result);
        }
        if matches!(self.phase, Phase::Body) {
            let PdfObject::Stream { data, .. } = &graph.object(self.object).ok_or(GraphRefusal::SourceChanged)?.value else {
                return Err(GraphRefusal::SourceChanged.into());
            };
            if let Some(flate) = self.flate.as_mut() {
                let (emitted, checksummed, done) = flate.step(data, sink, grant, self.limit - self.output_at)?;
                self.output_at += emitted;
                result.emitted = emitted;
                result.prepared = checksummed;
                if done { self.flate = None; self.phase = Phase::ObjectEnd; }
                return Ok(result);
            }
            let mut page = [0u8; 4096];
            let want = (data.len() - self.body_at).min(page.len());
            if semio_framework_value::ByteReadSource::read_bytes(data, self.body_at, &mut page[..want]) != want { return Err(GraphRefusal::SourceChanged.into()); }
            let written = try_write(sink, &page[..want], grant, self.limit - self.output_at)?;
            self.output_at += written;
            self.body_at += written;
            result.emitted = written;
            if self.body_at == data.len() { self.phase = Phase::ObjectEnd; }
            return Ok(result);
        }
        if matches!(self.phase, Phase::Copy) {
            let frame = self.append.ok_or(GraphRefusal::SourceChanged)?;
            let source = original.ok_or(GraphRefusal::SourceChanged)?;
            if source.byte_len() != frame.length { return Err(GraphRefusal::SourceChanged.into()); }
            let want = (frame.length - self.copy_at).min(4096).min(grant);
            if want == 0 && self.copy_at < frame.length { return Ok(result); }
            let mut page = [0u8; 4096];
            if source.read_bytes(self.copy_at, &mut page[..want]) != want { return Err(GraphRefusal::SourceChanged.into()); }
            let written = try_write(sink, &page[..want], grant, self.limit - self.output_at)?;
            self.output_at += written;
            self.copy_at += written;
            result.emitted = written;
            if self.copy_at == frame.length {
                self.phase = if graph.len() == 0 { Phase::Done } else if frame.separator { Phase::Separator } else { Phase::Object };
            }
            return Ok(result);
        }
        if matches!(self.phase, Phase::Done) { result.done = true; return Ok(result); }
        if grant < self.prefix { return Ok(result); }
        self.prepare(graph)?;
        result.prepared = self.buffer.len();
        result.control = true;
        Ok(result)
    }

    fn prepare(&mut self, graph: GraphLoan<'_>) -> Result<(), WriteRefusal> {
        let size = match self.append {
            Some(frame) => frame.size,
            None => graph.object(graph.len() - 1).ok_or(GraphRefusal::ObjectRoster)?.id.num + 1,
        };
        let last = match self.append { Some(AppendFrame { stream: true, .. }) => Phase::StreamOpen, _ => Phase::XrefHeader };
        let out = &mut self.buffer;
        self.phase = match self.phase {
            Phase::Header => {
                let version = if graph.version.is_empty() { "1.7" } else { graph.version };
                write!(out, "%PDF-{version}\n%")?;
                out.extend_from_slice(&[0xE2, 0xE3, 0xCF, 0xD3, b'\n']);
                Phase::Object
            }
            Phase::Object => {
                let object = graph.object(self.object).ok_or(GraphRefusal::SourceChanged)?;
                self.offsets[self.object] = self.output_at as u32;
                writeln!(out, "{} {} obj", object.id.num, object.id.gen)?;
                match &object.value {
                    PdfObject::Stream { dict, data, filters } => {
                        out.extend_from_slice(b"<<");
                        let mut first = true;
                        for entry in dict.iter().filter(|entry| entry.key != "Length"
                            && (filters.is_empty() || !matches!(entry.key.as_str(), "Filter" | "DecodeParms")))
                        {
                            append_entry(out, &mut first, &entry.key, &entry.value);
                        }
                        self.flate = is_flate(filters).then(StoredFlate::new);
                        if self.flate.is_some() {
                            append_entry(out, &mut first, "Filter", &PdfObject::name("FlateDecode"));
                        } else if !filters.is_empty() {
                            for entry in encode_stream(&[], filters).1 {
                                append_entry(out, &mut first, &entry.key, &entry.value);
                            }
                        }
                        append_entry(out, &mut first, "Length", &PdfObject::Int(stream_extent(filters, data.len())? as i64));
                        out.extend_from_slice(b">>\nstream\n");
                        self.body_at = 0;
                        Phase::Body
                    }
                    value => {
                        write_object(out, value);
                        out.extend_from_slice(b"\nendobj\n");
                        next_object(&mut self.object, graph.len(), last)
                    }
                }
            }
            Phase::ObjectEnd => {
                out.extend_from_slice(b"\nendstream\nendobj\n");
                next_object(&mut self.object, graph.len(), last)
            }
            Phase::XrefHeader if self.append.is_some() => {
                self.xref_at = self.output_at;
                out.extend_from_slice(b"xref\n");
                Phase::XrefRun
            }
            Phase::XrefRun => {
                let count = run_length(graph, self.xref_object);
                write!(out, "{} {count}\n", graph.object(self.xref_object).ok_or(GraphRefusal::SourceChanged)?.id.num)?;
                self.run_left = count;
                Phase::XrefRow
            }
            Phase::XrefRow if self.append.is_some() => {
                let object = graph.object(self.xref_object).ok_or(GraphRefusal::SourceChanged)?;
                writeln!(out, "{:010} {:05} n ", self.offsets[self.xref_object], object.id.gen)?;
                self.xref_object += 1;
                self.run_left -= 1;
                match (self.run_left, self.xref_object == graph.len()) {
                    (0, true) => Phase::Trailer,
                    (0, false) => Phase::XrefRun,
                    _ => Phase::XrefRow,
                }
            }
            Phase::StreamOpen => {
                self.xref_at = self.output_at;
                write!(out, "{size} 0 obj\n<</Type /XRef /Size {} /W [1 4 2] /Index [", size + 1)?;
                self.xref_object = 0;
                Phase::StreamRun
            }
            Phase::StreamRun => {
                if self.xref_object == graph.len() { Phase::StreamClose } else {
                    let count = run_length(graph, self.xref_object);
                    write!(out, "{} {count} ", graph.object(self.xref_object).ok_or(GraphRefusal::SourceChanged)?.id.num)?;
                    self.xref_object += count as usize;
                    Phase::StreamRun
                }
            }
            Phase::StreamClose => {
                write!(out, "{size} 1]")?;
                let mut first = false;
                write_identity(out, graph, &mut first);
                let frame = self.append.ok_or(GraphRefusal::SourceChanged)?;
                write!(out, " /Prev {} /Length {}>>\nstream\n", frame.prev, 7 * (graph.len() + 1))?;
                self.stream_at = 0;
                Phase::StreamBody
            }
            Phase::StreamBody => {
                let total = graph.len() + 1;
                let batch = (self.prefix / 7).min(total - self.stream_at);
                for row in self.stream_at..self.stream_at + batch {
                    let (offset, generation) = match graph.object(row) {
                        Some(object) => (self.offsets[row], object.id.gen),
                        None => (self.xref_at as u32, 0),
                    };
                    out.push(1);
                    out.extend_from_slice(&offset.to_be_bytes());
                    out.extend_from_slice(&generation.to_be_bytes());
                }
                self.stream_at += batch;
                if self.stream_at == total { Phase::StreamEnd } else { Phase::StreamBody }
            }
            Phase::StreamEnd => {
                write!(out, "\nendstream\nendobj\nstartxref\n{}\n%%EOF\n", self.xref_at)?;
                Phase::Done
            }
            Phase::Separator => {
                out.push(b'\n');
                Phase::Object
            }
            Phase::XrefHeader => {
                self.xref_at = self.output_at;
                write!(out, "xref\n0 {size}\n{:010} 65535 f \n", next_free(graph, 1, 0))?;
                self.xref_row = 1;
                Phase::XrefRow
            }
            Phase::XrefRow => {
                let number = self.xref_row;
                if let Some(object) = graph.object(self.xref_object).filter(|object| object.id.num == number) {
                    let offset = self.offsets[self.xref_object];
                    writeln!(out, "{offset:010} {:05} n ", object.id.gen)?;
                    self.xref_object += 1;
                } else {
                    writeln!(out, "{:010} 00000 f ", next_free(graph, number + 1, self.xref_object))?;
                }
                self.xref_row += 1;
                if self.xref_row == size { Phase::Trailer } else { Phase::XrefRow }
            }
            Phase::Trailer => {
                out.extend_from_slice(b"trailer\n<</Size ");
                write!(out, "{size}")?;
                let mut first = false;
                write_identity(out, graph, &mut first);
                if let Some(frame) = self.append { write!(out, " /Prev {}", frame.prev)?; }
                write!(out, ">>\nstartxref\n{}\n%%EOF\n", self.xref_at)?;
                Phase::Done
            }
            Phase::Body | Phase::Copy | Phase::Done => unreachable!("prepared graph advancing outside a control turn"),
        };
        if self.buffer.len() > self.prefix { return Err(GraphRefusal::ControlExtent.into()); }
        Ok(())
    }

    /// ♻️ Transfers the exact same paid scratch owners for the job's original retirement.
    pub fn into_parts(self) -> (Vec<u8>, Vec<u32>) { (self.buffer, self.offsets) }
}

fn next_object(index: &mut usize, count: usize, last: Phase) -> Phase {
    *index += 1;
    if *index < count { Phase::Object } else { last }
}

/// 📎️ How many consecutive object numbers the appended rows hold from `from`.
fn run_length(graph: GraphLoan<'_>, from: usize) -> u32 {
    let first = graph.object(from).map_or(0, |object| object.id.num);
    let mut count = 1;
    while graph.object(from + count as usize).is_some_and(|object| object.id.num == first + count) { count += 1; }
    count
}

fn write_identity(out: &mut Vec<u8>, graph: GraphLoan<'_>, first: &mut bool) {
    let trailer = graph.trailer;
    append_entry(out, first, "Root", &PdfObject::Ref(trailer.root));
    if let Some(info) = trailer.info { append_entry(out, first, "Info", &PdfObject::Ref(info)); }
    if let Some([first_id, second_id]) = &trailer.id {
        out.extend_from_slice(b" /ID[");
        write_string(out, first_id);
        out.push(b' ');
        write_string(out, second_id);
        out.push(b']');
    }
    for entry in trailer.extra.iter().filter(|entry| preserved_trailer(entry)) {
        append_entry(out, first, &entry.key, &entry.value);
    }
}

fn next_free(graph: GraphLoan<'_>, mut number: u32, first: usize) -> u32 {
    for index in first..graph.len() {
        let Some(object) = graph.object(index) else { return 0; };
        if object.id.num < number { continue; }
        if object.id.num != number { return number; }
        number += 1;
    }
    0
}

pub(crate) fn try_write(sink: &mut dyn Write, bytes: &[u8], grant: usize, remaining: usize) -> Result<usize, WriteRefusal> {
    let count = bytes.len().min(grant).min(4096);
    if count == 0 { return Ok(0); }
    if count > remaining { return Err(GraphRefusal::OutputExtent.into()); }
    let done = sink.write(&bytes[..count])?;
    if done > count { return Err(io::Error::from(io::ErrorKind::InvalidData).into()); }
    Ok(done)
}

fn append_entry(out: &mut Vec<u8>, first: &mut bool, key: &str, value: &PdfObject) {
    if !std::mem::replace(first, false) { out.push(b' '); }
    write_name(out, key);
    if !matches!(value, PdfObject::Name(_) | PdfObject::Array(_) | PdfObject::Dict(_) | PdfObject::Str(_) | PdfObject::Stream { .. }) {
        out.push(b' ');
    }
    if let ("URI", PdfObject::Text(text)) = (key, value) { write_string(out, text.as_bytes()); }
    else { write_object(out, value); }
}
