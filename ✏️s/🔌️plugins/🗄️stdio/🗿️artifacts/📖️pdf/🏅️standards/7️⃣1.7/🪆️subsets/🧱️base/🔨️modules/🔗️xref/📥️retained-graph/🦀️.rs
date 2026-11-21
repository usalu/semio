//! 📥️ Materializes retained PDF objects in authored order under the source's allocation and work grants.

use crate::standards::v1_7::subsets::base::io::text::snapshot::retained_text::{RetainedTextCursor, RetainedTextGrant, RetainedTextReceipt, RetainedTextStatus};
use crate::standards::v1_7::subsets::base::modules::lexer::{CosGrant, CosLimits, CosReceipt, CosStatus, PdfCosCursor, PdfHeader, PdfIndirectLengthCursor, PdfStreamDictionaryCursor, StreamSpan};
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfStreamFilter};
use crate::standards::v1_7::subsets::base::modules::xref::{GraphMaterializationEvent, NormalProjection, NormalProjectionCursor, NormalProjectionEvent, ObjectStreamCursor, ObjectStreamEvent, PdfFileStructure, PdfFileStructureCursor, PdfFileStructureEvent, PdfFileStructureReceipt, RetainedGraphBuilder, StructuredTrailer, XrefEntry, XrefFileOrderCursor, XrefFileOrderEvent, XrefIndex, XrefIndexLookup, XrefIndexReceipt, XrefIndexStatus, XrefStreamError, XrefTrailerCursor, WRITER_TRAILER_KEYS};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraphImportEvent { More, Object, Complete, NeedsAllocation { bytes: usize }, NeedsRelease { bytes: usize }, Cancelled, Closed }

pub struct GraphImportReceipt {
    pub event: Result<GraphImportEvent, XrefStreamError>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

pub struct ImportedGraph {
    pub objects: Vec<PdfIndirectObject>,
    pub positions: XrefIndex,
}


/// 🧹️ Retires a transferred graph when its enclosing document import ends before publishing.
#[derive(Default)]
pub struct ImportedGraphRetirement {
    current: Option<RetiringValue>,
    closing_positions: bool,
}

impl ImportedGraphRetirement {
    pub fn close_step(&mut self, graph: &mut ImportedGraph, grant: CosGrant) -> GraphImportReceipt {
        if grant.work == 0 { return more(0); }
        if let Some(current) = self.current.as_mut() {
            let mut step = current.advance(grant);
            if matches!(step.event, Ok(GraphImportEvent::Closed)) {
                self.current = None;
                step.event = Ok(GraphImportEvent::More);
            }
            return step;
        }
        if let Some(object) = graph.objects.pop() {
            self.current = Some(RetiringValue::new(object.value));
            return more(1);
        }
        if graph.objects.capacity() != 0 { return release(&mut graph.objects, grant); }
        if !self.closing_positions {
            graph.positions.begin_close();
            self.closing_positions = true;
            return more(1);
        }
        let step = graph.positions.close_step(grant);
        let event = step.status.map_err(XrefStreamError::Structure).map(|status| match status {
            XrefIndexStatus::Closed => GraphImportEvent::Closed,
            XrefIndexStatus::NeedsRelease { bytes } => GraphImportEvent::NeedsRelease { bytes },
            _ => GraphImportEvent::More,
        });
        GraphImportReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes }
    }
}

enum Phase { Reserve, Order, Schedule(u32), Normal, TakeNormal, CloseNormal, RawStream, StreamHeader, Member, Publish, CloseOrder, Complete, Closing, Closed }

struct RawStream {
    span: StreamSpan,
    dictionary: PdfStreamDictionaryCursor,
    bytes: Vec<u8>,
}

struct ObjectStream {
    number: u32,
    bytes: Vec<u8>,
    header: ObjectStreamHeader,
    members: Option<ObjectStreamCursor>,
}

struct ObjectStreamHeader {
    fields: Vec<PdfDictEntry>,
    field: usize,
    count: Option<usize>,
    first: Option<usize>,
}

impl ObjectStreamHeader {
    fn new(fields: Vec<PdfDictEntry>) -> Self {
        Self { fields, field: 0, count: None, first: None }
    }

    fn advance(&mut self) -> bool {
        let Some(field) = self.fields.get(self.field) else { return false; };
        self.field += 1;
        let slot = match field.key.as_str() { "N" => &mut self.count, "First" => &mut self.first, _ => return true };
        if slot.is_none() { *slot = field.value.as_i64().and_then(|number| usize::try_from(number).ok()); }
        true
    }

    fn fields(&self) -> Option<(usize, usize)> { Some((self.count?, self.first?)) }
}

#[path = "📏️compressed-length/🦀️.rs"]
mod compressed_length;
use compressed_length::CompressedLength;

struct RetiringValue {
    body: Vec<u8>,
    filters: Vec<PdfStreamFilter>,
    tree: PdfCosCursor,
}

impl RetiringValue {
    fn new(value: PdfObject) -> Self {
        let (body, filters, tree) = match value {
            PdfObject::Stream { data, filters, dict } => (data, filters, PdfCosCursor::retire_dictionary(dict)),
            value => (Vec::new(), Vec::new(), PdfCosCursor::retire_parsed_value(value)),
        };
        Self { body, filters, tree }
    }

    fn advance(&mut self, grant: CosGrant) -> GraphImportReceipt {
        if !self.tree.terminal_is_empty() { return cos(self.tree.close_step(grant)); }
        if self.body.capacity() != 0 { return release(&mut self.body, grant); }
        if let Some(filter) = self.filters.last() {
            let bytes = match filter {
                PdfStreamFilter::Jbig2 { globals: Some(bytes) } => bytes.capacity(),
                PdfStreamFilter::Crypt { name: Some(name) } => name.capacity(),
                _ => 0,
            };
            if bytes > grant.release_bytes { return report(Ok(GraphImportEvent::NeedsRelease { bytes }), 0); }
            self.filters.pop();
            return released(bytes);
        }
        if self.filters.capacity() != 0 { return release(&mut self.filters, grant); }
        report(Ok(GraphImportEvent::Closed), 0)
    }
}

/// 🧭️ Holds the current stream, indirect length and unpublished object until their original backing can move or retire.
pub struct GraphImportCursor {
    phase: Phase,
    order: XrefFileOrderCursor,
    builder: RetainedGraphBuilder,
    normal: Option<Box<NormalProjectionCursor>>,
    length: Option<(ObjRef, PdfIndirectLengthCursor, Option<Option<usize>>)>,
    length_reference: Option<(ObjRef, XrefIndexLookup)>,
    compressed_length: Option<CompressedLength>,
    raw: Option<RawStream>,
    stream: Option<ObjectStream>,
    current: ObjRef,
    pending: Option<PdfObject>,
    retiring: Option<RetiringValue>,
    maximum_stream_bytes: usize,
}

impl GraphImportCursor {
    pub fn new(maximum_objects: usize, maximum_stream_bytes: usize) -> Self {
        Self {
            phase: Phase::Reserve, order: XrefFileOrderCursor::default(), builder: RetainedGraphBuilder::new(maximum_objects),
            normal: None, length: None, length_reference: None, compressed_length: None, raw: None, stream: None, current: ObjRef { num: 0, gen: 0 },
            pending: None, retiring: None, maximum_stream_bytes,
        }
    }

    /// ▶️ Advances the same source and index as the previous turn, with no source or object cloning.
    pub fn advance(&mut self, source: &[u8], index: &XrefIndex, grant: CosGrant, cancelled: bool) -> GraphImportReceipt {
        use GraphImportEvent as Event;
        if cancelled { self.begin_close(); }
        match self.phase {
            Phase::Closing => return report(Ok(Event::Cancelled), 0),
            Phase::Closed => return report(Ok(Event::Closed), 0),
            Phase::Complete => return report(Ok(Event::Complete), 0),
            _ if grant.work == 0 => return report(Ok(Event::More), 0),
            _ => {}
        }
        if self.retiring.is_some() { return self.retire_value(grant); }
        if self.length.is_some() { return self.advance_length(source, grant); }
        if self.length_reference.is_some() { return self.find_length_source(index, grant); }
        if self.compressed_length.is_some() { return self.advance_compressed_length(source, index, grant); }
        match self.phase {
            Phase::Reserve | Phase::Publish => self.build_graph(grant),
            Phase::Order => {
                let step = self.order.advance(index, grant, false);
                if let Ok(XrefFileOrderEvent::Object(number)) = step.event { self.phase = Phase::Schedule(number); }
                if step.event == Ok(XrefFileOrderEvent::Complete) { self.phase = Phase::CloseOrder; self.order.begin_close(); }
                order(step)
            }
            Phase::Schedule(number) => self.schedule(index, number, grant),
            Phase::Normal => self.advance_normal(source, grant),
            Phase::TakeNormal => self.take_normal(),
            Phase::CloseNormal => self.close_normal(grant),
            Phase::RawStream => self.raw_stream(source, grant),
            Phase::StreamHeader => self.stream_header(),
            Phase::Member => self.member(grant),
            Phase::CloseOrder => {
                if self.stream.is_some() { return self.retire_stream(grant); }
                let step = self.order.close_step(grant);
                if step.event == Ok(XrefFileOrderEvent::Closed) { self.phase = Phase::Complete; }
                order(step)
            }
            _ => unreachable!(),
        }
    }

    fn schedule(&mut self, index: &XrefIndex, number: u32, grant: CosGrant) -> GraphImportReceipt {
        match index.get(&number).copied() {
            Some(XrefEntry::Normal { offset, gen }) => {
                if self.stream.is_some() { return self.retire_stream(grant); }
                self.current = ObjRef { num: number, gen };
                let bytes = std::mem::size_of::<NormalProjectionCursor>();
                if bytes > grant.allocation_bytes { return report(Ok(GraphImportEvent::NeedsAllocation { bytes }), 0); }
                let cursor = NormalProjectionCursor::new(self.current, offset, CosLimits::default(), self.maximum_stream_bytes).keep_object_streams();
                let Some(cursor) = admit_box(cursor) else { return failed("unable to allocate the PDF object projection"); };
                self.normal = Some(cursor);
                self.phase = Phase::Normal;
                return GraphImportReceipt { allocated_bytes: bytes, ..more(1) };
            }
            Some(XrefEntry::Compressed { stream_num, index }) => {
                let Some(stream) = self.stream.as_mut().filter(|stream| stream.number == stream_num) else {
                    return failed("object stream does not precede its indexed member");
                };
                let Some(members) = stream.members.as_mut() else { return failed("compressed PDF object has no decoded member source"); };
                if let Err(reason) = members.select(number, index) { return failed(reason); }
                self.current = ObjRef { num: number, gen: 0 };
                self.phase = Phase::Member;
            }
            None => return failed("ordered PDF object disappeared from its cross-reference index"),
        }
        more(1)
    }

    fn advance_normal(&mut self, source: &[u8], grant: CosGrant) -> GraphImportReceipt {
        let cursor = self.normal.as_mut().expect("original PDF object projection");
        let step = cursor.advance(source, grant, false);
        match step.event {
            Ok(NormalProjectionEvent::Complete) => self.phase = Phase::TakeNormal,
            Ok(NormalProjectionEvent::NeedsLength(reference)) => self.request_length(reference),
            _ => {}
        }
        projection(step)
    }

    fn request_length(&mut self, reference: ObjRef) {
        self.length_reference = Some((reference, XrefIndexLookup::new(reference.num)));
    }

    fn find_length_source(&mut self, index: &XrefIndex, grant: CosGrant) -> GraphImportReceipt {
        let (reference, lookup) = self.length_reference.as_mut().expect("PDF stream length reference lookup");
        let (work, entry) = lookup.advance(index, grant.work);
        let Some(entry) = entry else { return more(work); };
        let reference = *reference;
        self.length_reference = None;
        let carrier = self.compressed_length.as_ref().map(CompressedLength::stream_number);
        let owner = carrier.unwrap_or(self.current.num);
        let result = match entry {
            Some(XrefEntry::Normal { offset, gen }) if gen == reference.gen && reference.num != owner => {
                self.length = Some((reference, PdfIndirectLengthCursor::new(reference, offset), None));
                Ok(())
            }
            Some(XrefEntry::Compressed { .. }) if reference.gen == 0 && carrier.is_some() => {
                Err("PDF object stream length cannot be compressed")
            }
            Some(XrefEntry::Compressed { stream_num, index: member }) if reference.gen == 0 => {
                if stream_num == owner {
                    Err("PDF object stream cannot contain its own length")
                } else {
                    self.compressed_length = Some(CompressedLength::new(reference, stream_num, member));
                    Ok(())
                }
            }
            _ => self.waiting_length_projection().provide_length(reference, None),
        };
        report(result.map(|_| GraphImportEvent::More).map_err(XrefStreamError::Structure), work)
    }

    fn waiting_length_projection(&mut self) -> &mut NormalProjectionCursor {
        let waiting = if let Some(compressed) = self.compressed_length.as_mut() {
            compressed.projection.as_deref_mut()
        } else { self.normal.as_deref_mut() };
        waiting.expect("waiting PDF object")
    }

    fn advance_length(&mut self, source: &[u8], grant: CosGrant) -> GraphImportReceipt {
        let (reference, cursor, resolved) = self.length.as_mut().expect("original indirect PDF stream length");
        if let Some(length) = *resolved {
            let step = cursor.close_step(grant);
            if step.status == Ok(CosStatus::Closed) {
                let reference = *reference;
                self.length = None;
                let result = self.waiting_length_projection().provide_length(reference, length);
                if let Err(error) = result { return GraphImportReceipt { event: Err(XrefStreamError::Structure(error)), ..cos(step) }; }
            }
            return cos(step);
        }
        let mut step = cursor.advance(source, grant, false);
        match step.status {
            Ok(CosStatus::Complete { .. }) => { *resolved = Some(cursor.length()); cursor.begin_close(); }
            Err(_) => { *resolved = Some(None); cursor.begin_close(); step.status = Ok(CosStatus::More); }
            _ => {}
        }
        cos(step)
    }

    fn take_normal(&mut self) -> GraphImportReceipt {
        let cursor = self.normal.as_mut().expect("completed PDF object projection");
        match cursor.take() {
            Some(NormalProjection::Value(value)) => self.pending = Some(value),
            Some(NormalProjection::ReferenceStream { dict, span }) => {
                self.raw = Some(RawStream { span, dictionary: PdfStreamDictionaryCursor::new(dict, false), bytes: Vec::new() });
            }
            Some(NormalProjection::ObjectStream(PdfObject::Stream { dict, data, filters })) => {
                self.stream = Some(ObjectStream { number: self.current.num, bytes: data, header: ObjectStreamHeader::new(dict), members: None });
                self.retiring = Some(RetiringValue::new(PdfObject::Stream { dict: Vec::new(), data: Vec::new(), filters }));
            }
            Some(NormalProjection::Omitted) => {}
            _ => return failed("completed PDF graph object was not transferable"),
        }
        cursor.begin_close();
        self.phase = Phase::CloseNormal;
        more(1)
    }

    fn close_normal(&mut self, grant: CosGrant) -> GraphImportReceipt {
        let step = self.retire_normal(grant);
        if self.normal.is_none() {
            self.phase = if self.raw.is_some() { Phase::RawStream }
            else if self.stream.is_some() { Phase::StreamHeader }
            else if self.pending.is_some() { Phase::Publish }
            else { Phase::Order };
        }
        step
    }

    fn retire_normal(&mut self, grant: CosGrant) -> GraphImportReceipt {
        retire_projection(&mut self.normal, grant)
    }

    fn raw_stream(&mut self, source: &[u8], grant: CosGrant) -> GraphImportReceipt {
        let raw = self.raw.as_mut().expect("preserved coded PDF stream");
        let Some(bytes) = source.get(raw.span.start..raw.span.end) else { return failed("raw PDF stream source was replaced"); };
        if bytes.len() > self.maximum_stream_bytes { return failed("retained PDF stream exceeds the declared byte ceiling"); }
        if raw.bytes.capacity() < bytes.len() {
            if bytes.len() > grant.allocation_bytes { return report(Ok(GraphImportEvent::NeedsAllocation { bytes: bytes.len() }), 0); }
            if raw.bytes.try_reserve_exact(bytes.len()).is_err() { return failed("unable to allocate retained PDF stream"); }
            return GraphImportReceipt { allocated_bytes: raw.bytes.capacity(), ..more(1) };
        }
        if raw.bytes.len() < bytes.len() {
            let start = raw.bytes.len();
            let count = (bytes.len() - start).min(grant.work);
            raw.bytes.extend_from_slice(&bytes[start..start + count]);
            return more(count);
        }
        let step = raw.dictionary.advance(grant, false);
        if matches!(step.status, Ok(CosStatus::Complete { .. })) {
            let mut raw = self.raw.take().expect("settled coded PDF stream");
            let dict = raw.dictionary.take_dictionary().expect("retained raw dictionary");
            self.pending = Some(PdfObject::Stream { dict, data: raw.bytes, filters: Vec::new() });
            self.phase = Phase::Publish;
        }
        cos(step)
    }

    fn stream_header(&mut self) -> GraphImportReceipt {
        let stream = self.stream.as_mut().expect("decoded PDF object-stream owner");
        if stream.header.advance() { return more(1); }
        let Some((count, first)) = stream.header.fields() else { return failed("PDF object stream is missing N or First"); };
        stream.members = Some(ObjectStreamCursor::new(count, first, CosLimits::default()));
        self.retiring = retire_object_stream_header(&mut stream.header);
        self.phase = Phase::Order;
        more(1)
    }

    fn member(&mut self, grant: CosGrant) -> GraphImportReceipt {
        let stream = self.stream.as_mut().expect("original compressed PDF object source");
        let cursor = stream.members.as_mut().expect("selected PDF object-stream member");
        let step = cursor.advance(&stream.bytes, grant, false);
        match step.event {
            Ok(ObjectStreamEvent::Complete { .. }) => self.pending = cursor.take_value(),
            Ok(ObjectStreamEvent::Idle) if self.pending.is_some() => self.phase = Phase::Publish,
            _ => {}
        }
        object_stream(step)
    }

    fn build_graph(&mut self, grant: CosGrant) -> GraphImportReceipt {
        if let Some(value) = self.pending.take() {
            if let Err(object) = self.builder.offer(PdfIndirectObject { id: self.current, value }) {
                self.pending = Some(object.value);
                return failed("PDF object could not enter the retained graph");
            }
            return more(1);
        }
        let step = self.builder.advance(grant, false);
        let mut receipt = materialization(step);
        if matches!(receipt.event, Ok(GraphImportEvent::Object)) {
            if matches!(self.phase, Phase::Reserve) { receipt.event = Ok(GraphImportEvent::More); }
            self.phase = Phase::Order;
        }
        receipt
    }

    /// 📤️ Transfers the ordered objects and their position index on successful completion.
    pub fn take(&mut self) -> Option<ImportedGraph> {
        if !matches!(self.phase, Phase::Complete) { return None; }
        let (objects, positions) = self.builder.finish()?;
        self.phase = Phase::Closed;
        Some(ImportedGraph { objects, positions })
    }

    pub fn begin_close(&mut self) {
        if !matches!(self.phase, Phase::Closing | Phase::Closed) {
            self.order.begin_close();
            self.builder.begin_close();
            if let Some(normal) = self.normal.as_mut() { normal.begin_close(); }
            self.length_reference = None;
            if let Some((_, length, _)) = self.length.as_mut() { length.begin_close(); }
            if let Some(compressed) = self.compressed_length.as_mut() { compressed.begin_close(); }
            self.phase = Phase::Closing;
        }
    }

    pub fn close_step(&mut self, grant: CosGrant) -> GraphImportReceipt {
        use GraphImportEvent as Event;
        if matches!(self.phase, Phase::Closed) { return report(Ok(Event::Closed), 0); }
        if !matches!(self.phase, Phase::Closing) || grant.work == 0 { return more(0); }
        if self.retiring.is_some() { return self.retire_value(grant); }
        if let Some((_, length, _)) = self.length.as_mut() {
            let step = length.close_step(grant);
            if step.status == Ok(CosStatus::Closed) { self.length = None; }
            return cos(step);
        }
        if self.compressed_length.is_some() { return self.close_compressed_length(grant); }
        if self.normal.is_some() { return self.retire_normal(grant); }
        if let Some(raw) = self.raw.as_mut() {
            if raw.bytes.capacity() != 0 { return release(&mut raw.bytes, grant); }
            let step = raw.dictionary.close_step(grant);
            if step.status == Ok(CosStatus::Closed) { self.raw = None; }
            return cos(step);
        }
        if self.stream.is_some() { return self.retire_stream(grant); }
        if let Some(value) = self.pending.take().or_else(|| self.builder.take_retired_object().map(|object| object.value)) {
            self.retiring = Some(RetiringValue::new(value));
            return more(1);
        }
        let ordered = self.order.close_step(grant);
        if ordered.event != Ok(XrefFileOrderEvent::Closed) || ordered.work != 0 { return order(ordered); }
        let graph = self.builder.close_step(grant);
        if graph.event == Ok(GraphMaterializationEvent::Closed) {
            self.phase = Phase::Closed;
            return GraphImportReceipt { event: Ok(Event::Closed), ..materialization(graph) };
        }
        materialization(graph)
    }

    fn retire_value(&mut self, grant: CosGrant) -> GraphImportReceipt {
        let mut step = self.retiring.as_mut().expect("PDF value retirement owner").advance(grant);
        if matches!(step.event, Ok(GraphImportEvent::Closed)) { self.retiring = None; step.event = Ok(GraphImportEvent::More); }
        step
    }

    fn retire_stream(&mut self, grant: CosGrant) -> GraphImportReceipt {
        let stream = self.stream.as_mut().expect("PDF object stream retirement owner");
        if let Some(members) = stream.members.as_mut() {
            members.begin_close();
            let step = members.close_step(grant);
            if step.event == Ok(ObjectStreamEvent::Closed) { stream.members = None; }
            return object_stream(step);
        }
        if stream.header.fields.capacity() != 0 {
            self.retiring = retire_object_stream_header(&mut stream.header);
            return more(1);
        }
        if stream.bytes.capacity() != 0 { return release(&mut stream.bytes, grant); }
        self.stream = None;
        more(1)
    }
}

fn retire_object_stream_header(header: &mut ObjectStreamHeader) -> Option<RetiringValue> {
    if header.fields.capacity() == 0 { return None; }
    Some(RetiringValue::new(PdfObject::Dict(std::mem::take(&mut header.fields))))
}

fn retire_projection(cursor: &mut Option<Box<NormalProjectionCursor>>, grant: CosGrant) -> GraphImportReceipt {
    let step = cursor.as_mut().expect("closing PDF object projection").close_step(grant);
    if !matches!(step.event, Ok(NormalProjectionEvent::Closed)) || step.work != 0 { return projection(step); }
    let bytes = std::mem::size_of::<NormalProjectionCursor>();
    if bytes > grant.release_bytes { return report(Ok(GraphImportEvent::NeedsRelease { bytes }), 0); }
    *cursor = None;
    released(bytes)
}

fn report(event: Result<GraphImportEvent, XrefStreamError>, work: usize) -> GraphImportReceipt {
    GraphImportReceipt { event, work, allocated_bytes: 0, released_bytes: 0 }
}

fn more(work: usize) -> GraphImportReceipt { report(Ok(GraphImportEvent::More), work) }
fn failed(reason: &'static str) -> GraphImportReceipt { report(Err(XrefStreamError::Structure(reason)), 1) }
fn released(bytes: usize) -> GraphImportReceipt { GraphImportReceipt { released_bytes: bytes, ..more(1) } }

fn release<T>(source: &mut Vec<T>, grant: CosGrant) -> GraphImportReceipt {
    let bytes = source.capacity() * std::mem::size_of::<T>();
    if bytes > grant.release_bytes { return report(Ok(GraphImportEvent::NeedsRelease { bytes }), 0); }
    *source = Vec::new();
    released(bytes)
}

macro_rules! receipt_from {
    ($name:ident, $input:ty, $kind:ident, $event:ident, $error:expr $(, $pattern:pat => $result:expr)?) => {
        fn $name(step: $input) -> GraphImportReceipt {
            let event = step.$event.map_err($error).map(|status| match status {
                $kind::NeedsAllocation { bytes } => GraphImportEvent::NeedsAllocation { bytes },
                $kind::NeedsRelease { bytes } => GraphImportEvent::NeedsRelease { bytes },
                $($pattern => $result,)?
                _ => GraphImportEvent::More,
            });
            GraphImportReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes }
        }
    };
}

receipt_from!(cos, CosReceipt, CosStatus, status, XrefStreamError::Structure);
receipt_from!(projection, crate::standards::v1_7::subsets::base::modules::xref::NormalProjectionReceipt, NormalProjectionEvent, event, |error| error);
receipt_from!(order, crate::standards::v1_7::subsets::base::modules::xref::XrefFileOrderReceipt, XrefFileOrderEvent, event, XrefStreamError::Structure);
receipt_from!(object_stream, crate::standards::v1_7::subsets::base::modules::xref::ObjectStreamReceipt, ObjectStreamEvent, event, XrefStreamError::Structure);
receipt_from!(materialization, crate::standards::v1_7::subsets::base::modules::xref::GraphMaterializationReceipt, GraphMaterializationEvent, event, XrefStreamError::Structure, GraphMaterializationEvent::Ready => GraphImportEvent::Object);

pub(crate) fn admit_box<T>(value: T) -> Option<Box<T>> {
    if std::mem::size_of::<T>() == 0 { return Some(Box::new(value)); }
    let pointer = std::ptr::NonNull::new(unsafe { std::alloc::alloc(std::alloc::Layout::new::<T>()) }.cast::<T>())?;
    unsafe { pointer.as_ptr().write(value); Some(Box::from_raw(pointer.as_ptr())) }
}


/// ♻️ Separates a file that needs the desktop reference reader from a rejected structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PdfByteGraphEvent {
    More, Object, NeedsAllocation { bytes: usize }, NeedsRelease { bytes: usize },
    ReferenceRequired, NotPdf, Complete, Cancelled, Closed,
}

pub struct PdfByteGraphReceipt {
    pub event: Result<PdfByteGraphEvent, XrefStreamError>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

pub struct PdfByteGraph {
    pub header: PdfHeader,
    pub origin: usize,
    pub trailer: StructuredTrailer,
    pub graph: ImportedGraph,
}

enum BytePhase { Structure, CloseStructure, AllocateProjection, Project, CloseIndex, Reference, NotPdf, Complete, Closing, Closed }

/// 🧩️ Composes bounded file structure and graph work while borrowing original unencrypted PDF bytes.
pub struct PdfByteGraphCursor {
    phase: BytePhase,
    structure: PdfFileStructureCursor,
    file: Option<PdfFileStructure>,
    projection: Option<Box<GraphImportCursor>>,
    closing_trailer: Option<XrefTrailerCursor>,
    maximum_objects: usize,
    maximum_stream_bytes: usize,
}

impl PdfByteGraphCursor {
    pub fn new(maximum_objects: usize, maximum_stream_bytes: usize) -> Self {
        Self {
            phase: BytePhase::Structure, structure: PdfFileStructureCursor::new(maximum_stream_bytes),
            file: None, projection: None, closing_trailer: None, maximum_objects, maximum_stream_bytes,
        }
    }

    pub fn advance(&mut self, source: &[u8], grant: CosGrant, cancelled: bool) -> PdfByteGraphReceipt {
        use PdfByteGraphEvent as Event;
        if cancelled { self.begin_close(); }
        match self.phase {
            BytePhase::Closing => report_bytes(Ok(Event::Cancelled), 0),
            BytePhase::Closed => report_bytes(Ok(Event::Closed), 0),
            BytePhase::Reference => report_bytes(Ok(Event::ReferenceRequired), 0),
            BytePhase::NotPdf => report_bytes(Ok(Event::NotPdf), 0),
            BytePhase::Complete => report_bytes(Ok(Event::Complete), 0),
            _ if grant.work == 0 => report_bytes(Ok(Event::More), 0),
            BytePhase::Structure => {
                let step = self.structure.advance(source, grant, false);
                match step.event {
                    Ok(PdfFileStructureEvent::Complete) => {
                        let Some(file) = self.structure.take() else { return byte_error("PDF structure could not transfer"); };
                        let needs_reference = !file.intact || file.trailer.get_inheritable("Encrypt").is_some();
                        self.structure.begin_close();
                        self.phase = if needs_reference { BytePhase::Reference } else { BytePhase::CloseStructure };
                        self.file = Some(file);
                    }
                    Ok(PdfFileStructureEvent::RecoveryRequired) => self.phase = BytePhase::Reference,
                    Ok(PdfFileStructureEvent::NotPdf) => self.phase = BytePhase::NotPdf,
                    _ => {}
                }
                structure_receipt(step)
            }
            BytePhase::CloseStructure => {
                let step = self.structure.close_step(grant);
                if matches!(step.event, Ok(PdfFileStructureEvent::Closed)) { self.phase = BytePhase::AllocateProjection; }
                structure_receipt(step)
            }
            BytePhase::AllocateProjection => {
                let file = self.file.as_ref().expect("admitted PDF byte graph source");
                if file.index.len() > self.maximum_objects { return byte_error("PDF cross-reference exceeds the admitted item count"); }
                let bytes = Self::projection_bytes();
                if bytes > grant.allocation_bytes { return report_bytes(Ok(Event::NeedsAllocation { bytes }), 0); }
                let Some(cursor) = admit_box(GraphImportCursor::new(file.index.len(), self.maximum_stream_bytes)) else {
                    return byte_error("unable to allocate the PDF graph projection");
                };
                self.projection = Some(cursor);
                self.phase = BytePhase::Project;
                PdfByteGraphReceipt { allocated_bytes: bytes, ..report_bytes(Ok(Event::More), 1) }
            }
            BytePhase::Project => {
                let file = self.file.as_ref().expect("original PDF graph source");
                let Some(bytes) = source.get(file.origin..) else { return byte_error("PDF source origin has changed"); };
                let step = self.projection.as_mut().expect("retained PDF graph projection").advance(bytes, &file.index, grant, false);
                if matches!(step.event, Ok(GraphImportEvent::Complete)) {
                    self.file.as_mut().expect("original cross-reference source").index.begin_close();
                    self.phase = BytePhase::CloseIndex;
                }
                graph_receipt(step)
            }
            BytePhase::CloseIndex => {
                let step = self.file.as_mut().expect("PDF source index retirement").index.close_step(grant);
                if step.status == Ok(XrefIndexStatus::Closed) { self.phase = BytePhase::Complete; }
                index_receipt(step)
            }
        }
    }

    /// 📭️ Quotes the allocation released when the completed graph is transferred.
    pub const fn projection_bytes() -> usize { std::mem::size_of::<GraphImportCursor>() }

    /// 📤️ Transfers original objects and trailer with a separate grant for releasing the empty projection box.
    pub fn take(&mut self, grant: CosGrant) -> Option<(PdfByteGraph, usize)> {
        let bytes = Self::projection_bytes();
        if !matches!(self.phase, BytePhase::Complete) || grant.work == 0 || bytes > grant.release_bytes { return None; }
        let graph = self.projection.as_mut()?.take()?;
        self.projection = None;
        let file = self.file.take().expect("completed PDF byte graph retains its header and trailer");
        self.phase = BytePhase::Closed;
        Some((PdfByteGraph { header: file.header, origin: file.origin, trailer: file.trailer, graph }, bytes))
    }

    pub fn begin_close(&mut self) {
        if matches!(self.phase, BytePhase::Closing | BytePhase::Closed) { return; }
        self.structure.begin_close();
        if let Some(file) = self.file.as_mut() { file.index.begin_close(); }
        if let Some(graph) = self.projection.as_mut() { graph.begin_close(); }
        self.phase = BytePhase::Closing;
    }

    pub fn close_step(&mut self, grant: CosGrant) -> PdfByteGraphReceipt {
        use PdfByteGraphEvent as Event;
        if matches!(self.phase, BytePhase::Closed) { return report_bytes(Ok(Event::Closed), 0); }
        if !matches!(self.phase, BytePhase::Closing) || grant.work == 0 { return report_bytes(Ok(Event::More), 0); }
        if let Some(projection) = self.projection.as_mut() {
            let step = projection.close_step(grant);
            if !matches!(step.event, Ok(GraphImportEvent::Closed)) || step.work != 0 { return graph_receipt(step); }
            let bytes = Self::projection_bytes();
            if bytes > grant.release_bytes { return report_bytes(Ok(Event::NeedsRelease { bytes }), 0); }
            self.projection = None;
            return PdfByteGraphReceipt { released_bytes: bytes, ..report_bytes(Ok(Event::More), 1) };
        }
        if let Some(file) = self.file.as_mut() {
            let step = file.index.close_step(grant);
            if step.status != Ok(XrefIndexStatus::Closed) || step.work != 0 { return index_receipt(step); }
            let file = self.file.take().expect("PDF file owns its transferred trailer");
            self.closing_trailer = Some(file.trailer.into_retirement());
            return report_bytes(Ok(Event::More), 1);
        }
        if let Some(trailer) = self.closing_trailer.as_mut() {
            let step = trailer.advance(grant);
            if step.status == Ok(CosStatus::Closed) { self.closing_trailer = None; }
            let step = cos(step);
            return graph_receipt(step);
        }
        let step = self.structure.close_step(grant);
        if matches!(step.event, Ok(PdfFileStructureEvent::Closed)) {
            self.phase = BytePhase::Closed;
            return PdfByteGraphReceipt { event: Ok(Event::Closed), ..structure_receipt(step) };
        }
        structure_receipt(step)
    }
}

fn report_bytes(event: Result<PdfByteGraphEvent, XrefStreamError>, work: usize) -> PdfByteGraphReceipt {
    PdfByteGraphReceipt { event, work, allocated_bytes: 0, released_bytes: 0 }
}

fn byte_error(reason: &'static str) -> PdfByteGraphReceipt { report_bytes(Err(XrefStreamError::Structure(reason)), 1) }

fn graph_receipt(step: GraphImportReceipt) -> PdfByteGraphReceipt {
    use PdfByteGraphEvent as Event;
    let event = step.event.map(|status| match status {
        GraphImportEvent::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
        GraphImportEvent::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
        GraphImportEvent::Object => Event::Object,
        _ => Event::More,
    });
    PdfByteGraphReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes }
}

fn structure_receipt(step: PdfFileStructureReceipt) -> PdfByteGraphReceipt {
    use PdfByteGraphEvent as Event;
    let event = step.event.map(|status| match status {
        PdfFileStructureEvent::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
        PdfFileStructureEvent::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
        _ => Event::More,
    });
    PdfByteGraphReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes }
}

fn index_receipt(step: XrefIndexReceipt) -> PdfByteGraphReceipt {
    let event = step.status.map_err(XrefStreamError::Structure).map(|status| match status {
        XrefIndexStatus::NeedsRelease { bytes } => PdfByteGraphEvent::NeedsRelease { bytes },
        _ => PdfByteGraphEvent::More,
    });
    PdfByteGraphReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes }
}


pub struct PreparedPdfFile {
    pub header: PdfHeader,
    pub graph: ImportedGraph,
    pub trailer: Vec<PdfDictEntry>,
}

enum PreparePhase { Allocate, Bytes, TakeBytes, ReleaseBytes, Trailer, TakeTrailer, AppendInherited, Text, Ready, Closing, Closed }

/// 🧾️ Prepares the authored graph and its native metadata without borrowing it across turns.
pub struct PdfFilePreparationCursor {
    phase: PreparePhase,
    bytes: Option<Box<PdfByteGraphCursor>>,
    header: Option<PdfHeader>,
    graph: Option<ImportedGraph>,
    graph_retirement: ImportedGraphRetirement,
    dictionary: Option<PdfStreamDictionaryCursor>,
    inherited: [Option<PdfDictEntry>; 4],
    trailer: Vec<PdfDictEntry>,
    trailer_replacement: Vec<PdfDictEntry>,
    trailer_reverse: usize,
    trailer_entry_retirement: Option<PdfCosCursor>,
    text: Option<RetainedTextCursor>,
    maximum_objects: usize,
    maximum_stream_bytes: usize,
}

impl PdfFilePreparationCursor {
    pub fn new(maximum_objects: usize, maximum_stream_bytes: usize) -> Self {
        Self {
            phase: PreparePhase::Allocate, bytes: None, header: None, graph: None, graph_retirement: Default::default(),
            dictionary: None, inherited: std::array::from_fn(|_| None), trailer: Vec::new(),
            trailer_replacement: Vec::new(), trailer_reverse: 0, trailer_entry_retirement: None, text: None, maximum_objects, maximum_stream_bytes,
        }
    }

    pub fn advance(&mut self, source: &[u8], grant: CosGrant, cancelled: bool) -> PdfByteGraphReceipt {
        use PdfByteGraphEvent as Event;
        if cancelled { self.begin_close(); }
        match self.phase {
            PreparePhase::Ready => return report_bytes(Ok(Event::Complete), 0),
            PreparePhase::Closing => return report_bytes(Ok(Event::Cancelled), 0),
            PreparePhase::Closed => return report_bytes(Ok(Event::Closed), 0),
            _ if grant.work == 0 => return report_bytes(Ok(Event::More), 0),
            _ => {}
        }
        match self.phase {
            PreparePhase::Allocate => {
                let bytes = std::mem::size_of::<PdfByteGraphCursor>();
                if bytes > grant.allocation_bytes { return report_bytes(Ok(Event::NeedsAllocation { bytes }), 0); }
                let Some(cursor) = admit_box(PdfByteGraphCursor::new(self.maximum_objects, self.maximum_stream_bytes)) else {
                    return byte_error("unable to allocate the PDF file structure cursor");
                };
                self.bytes = Some(cursor);
                self.phase = PreparePhase::Bytes;
                PdfByteGraphReceipt { allocated_bytes: bytes, ..report_bytes(Ok(Event::More), 1) }
            }
            PreparePhase::Bytes => {
                let mut step = self.bytes.as_mut().expect("stable PDF source cursor").advance(source, grant, false);
                if matches!(step.event, Ok(Event::Complete)) {
                    self.phase = PreparePhase::TakeBytes;
                    step.event = Ok(Event::More);
                }
                step
            }
            PreparePhase::TakeBytes => self.take_bytes(grant),
            PreparePhase::ReleaseBytes => {
                let bytes = std::mem::size_of::<PdfByteGraphCursor>();
                if bytes > grant.release_bytes { return report_bytes(Ok(Event::NeedsRelease { bytes }), 0); }
                self.bytes = None;
                self.phase = PreparePhase::Trailer;
                PdfByteGraphReceipt { released_bytes: bytes, ..report_bytes(Ok(Event::More), 1) }
            }
            PreparePhase::Trailer => {
                let step = self.dictionary.as_mut().expect("authored PDF trailer custody").advance(grant, false);
                if matches!(step.status, Ok(CosStatus::Complete { .. })) { self.phase = PreparePhase::TakeTrailer; }
                graph_receipt(cos(step))
            }
            PreparePhase::TakeTrailer => {
                self.trailer = self.dictionary.as_mut().expect("settled PDF trailer custody").take_dictionary().expect("completed trailer");
                self.dictionary = None;
                self.phase = PreparePhase::AppendInherited;
                report_bytes(Ok(Event::More), 1)
            }
            PreparePhase::AppendInherited => self.append_inherited(grant),
            PreparePhase::Text => {
                let graph = self.graph.as_mut().expect("attached PDF graph");
                let step = self.text.as_mut().expect("native PDF text admission").advance(
                    &mut graph.objects, &mut self.trailer, |id| exact_position(&graph.positions, id), text_grant(grant), false,
                );
                if step.status == Ok(RetainedTextStatus::Complete) { self.text = None; self.phase = PreparePhase::Ready; }
                text_receipt(step)
            }
            _ => unreachable!(),
        }
    }

    /// 🧵️ Quotes the next unit when it reserves empty retained slots or a fresh Flate history.
    pub fn reserving_empty_backing(&self) -> bool {
        matches!(self.phase, PreparePhase::Bytes) && self.bytes.as_ref().is_some_and(|file|
            matches!(file.phase, BytePhase::Project) && file.projection.as_ref().is_some_and(|graph|
                matches!(graph.phase, Phase::Reserve) || matches!(graph.phase, Phase::Normal)
                    && graph.retiring.is_none() && graph.length.is_none() && graph.length_reference.is_none() && graph.compressed_length.is_none()
                    && graph.normal.as_ref().is_some_and(|stream| stream.next_reserves_history())))
    }

    fn take_bytes(&mut self, grant: CosGrant) -> PdfByteGraphReceipt {
        use PdfByteGraphEvent as Event;
        let bytes = PdfByteGraphCursor::projection_bytes();
        if bytes > grant.release_bytes { return report_bytes(Ok(Event::NeedsRelease { bytes }), 0); }
        let (file, _) = self.bytes.as_mut().expect("complete PDF bytes cursor").take(grant).expect("complete PDF byte graph");
        self.header = Some(file.header);
        self.graph = Some(file.graph);
        let (trailer, inherited) = file.trailer.into_parts();
        self.inherited = inherited;
        self.dictionary = Some(PdfStreamDictionaryCursor::with_retention(trailer, retain_trailer_entry));
        self.phase = PreparePhase::ReleaseBytes;
        PdfByteGraphReceipt { released_bytes: bytes, ..report_bytes(Ok(Event::More), 1) }
    }

    fn append_inherited(&mut self, grant: CosGrant) -> PdfByteGraphReceipt {
        use PdfByteGraphEvent as Event;
        if self.trailer_replacement.capacity() != 0 {
            if let Some(entry) = self.trailer.pop() {
                self.trailer_replacement.push(entry);
                return report_bytes(Ok(Event::More), 1);
            }
            if self.trailer.capacity() != 0 { return graph_receipt(release(&mut self.trailer, grant)); }
            let len = self.trailer_replacement.len();
            if self.trailer_reverse < len / 2 {
                self.trailer_replacement.swap(self.trailer_reverse, len - 1 - self.trailer_reverse);
                self.trailer_reverse += 1;
                return report_bytes(Ok(Event::More), 1);
            }
            self.trailer = std::mem::take(&mut self.trailer_replacement);
            return report_bytes(Ok(Event::More), 1);
        }
        let needed = self.trailer.len() + self.inherited.iter().flatten().count();
        if self.trailer.capacity() < needed {
            let bytes = needed * std::mem::size_of::<PdfDictEntry>();
            if bytes > grant.allocation_bytes { return report_bytes(Ok(Event::NeedsAllocation { bytes }), 0); }
            if self.trailer_replacement.try_reserve_exact(needed).is_err() { return byte_error("unable to allocate the merged PDF trailer"); }
            return PdfByteGraphReceipt { allocated_bytes: self.trailer_replacement.capacity() * std::mem::size_of::<PdfDictEntry>(), ..report_bytes(Ok(Event::More), 1) };
        }
        if let Some(entry) = self.inherited.iter_mut().find_map(Option::take) {
            self.trailer.push(entry);
            return report_bytes(Ok(Event::More), 1);
        }
        self.text = Some(RetainedTextCursor::new(self.graph.as_ref().expect("prepared PDF graph").objects.len()));
        self.phase = PreparePhase::Text;
        report_bytes(Ok(Event::More), 1)
    }

    /// 📤️ Moves the original graph and prepared metadata to semantic lifting without a clone.
    pub fn take(&mut self) -> Option<PreparedPdfFile> {
        if !matches!(self.phase, PreparePhase::Ready) { return None; }
        let file = PreparedPdfFile {
            header: self.header.take()?, graph: self.graph.take()?, trailer: std::mem::take(&mut self.trailer),
        };
        self.phase = PreparePhase::Closed;
        Some(file)
    }

    pub fn begin_close(&mut self) {
        if matches!(self.phase, PreparePhase::Closing | PreparePhase::Closed) { return; }
        if let Some(bytes) = self.bytes.as_mut() { bytes.begin_close(); }
        if let Some(text) = self.text.as_mut() { text.begin_close(); }
        self.phase = PreparePhase::Closing;
    }

    pub fn close_step(&mut self, grant: CosGrant) -> PdfByteGraphReceipt {
        use PdfByteGraphEvent as Event;
        if matches!(self.phase, PreparePhase::Closed) { return report_bytes(Ok(Event::Closed), 0); }
        if !matches!(self.phase, PreparePhase::Closing) || grant.work == 0 { return report_bytes(Ok(Event::More), 0); }
        if let Some(cursor) = self.bytes.as_mut() {
            let mut step = cursor.close_step(grant);
            if !matches!(step.event, Ok(Event::Closed)) || step.work != 0 {
                if matches!(step.event, Ok(Event::Closed)) { step.event = Ok(Event::More); }
                return step;
            }
            let bytes = std::mem::size_of::<PdfByteGraphCursor>();
            if bytes > grant.release_bytes { return report_bytes(Ok(Event::NeedsRelease { bytes }), 0); }
            self.bytes = None;
            return PdfByteGraphReceipt { released_bytes: bytes, ..report_bytes(Ok(Event::More), 1) };
        }
        if let Some(cursor) = self.text.as_mut() {
            let step = cursor.close_step(text_grant(grant));
            if step.status == Ok(RetainedTextStatus::Closed) { self.text = None; }
            return text_receipt(step);
        }
        if let Some(cursor) = self.trailer_entry_retirement.as_mut() {
            let step = cursor.close_step(grant);
            if step.status == Ok(CosStatus::Closed) { self.trailer_entry_retirement = None; }
            return graph_receipt(cos(step));
        }
        if let Some(entry) = self.inherited.iter_mut().find_map(Option::take) {
            self.trailer_entry_retirement = Some(PdfCosCursor::retire_dictionary_entry(entry));
            return report_bytes(Ok(Event::More), 1);
        }
        if let Some(cursor) = self.dictionary.as_mut() {
            let step = cursor.close_step(grant);
            if step.status == Ok(CosStatus::Closed) { self.dictionary = None; }
            return graph_receipt(cos(step));
        }
        for entries in [&mut self.trailer, &mut self.trailer_replacement] {
            if entries.capacity() == 0 { continue; }
            self.dictionary = Some(PdfStreamDictionaryCursor::with_retention(std::mem::take(entries), retain_trailer_entry));
            return report_bytes(Ok(Event::More), 1);
        }
        if let Some(graph) = self.graph.as_mut() {
            let step = self.graph_retirement.close_step(graph, grant);
            if matches!(step.event, Ok(GraphImportEvent::Closed)) { self.graph = None; }
            return graph_receipt(step);
        }
        self.phase = PreparePhase::Closed;
        report_bytes(Ok(Event::Closed), 0)
    }
}

pub fn exact_position(positions: &XrefIndex, id: ObjRef) -> Option<usize> {
    match positions.get(&id.num) {
        Some(XrefEntry::Normal { offset, gen }) if *gen == id.gen => Some(*offset),
        _ => None,
    }
}

fn retain_trailer_entry(entry: &PdfDictEntry, _: bool) -> bool { !WRITER_TRAILER_KEYS.contains(&entry.key.as_str()) }

fn text_grant(grant: CosGrant) -> RetainedTextGrant {
    RetainedTextGrant { work: grant.work, allocation_bytes: grant.allocation_bytes, release_bytes: grant.release_bytes }
}

fn text_receipt(step: RetainedTextReceipt) -> PdfByteGraphReceipt {
    let event = step.status.map_err(XrefStreamError::Structure).map(|status| match status {
        RetainedTextStatus::NeedsAllocation { bytes } => PdfByteGraphEvent::NeedsAllocation { bytes },
        RetainedTextStatus::NeedsRelease { bytes } => PdfByteGraphEvent::NeedsRelease { bytes },
        _ => PdfByteGraphEvent::More,
    });
    PdfByteGraphReceipt { event, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::standards::v1_7::subsets::base::modules::xref::{read_file_xref, Resolver};

    const COMPRESSED: &[u8] = include_bytes!("../../../🧪️tests/🧫️fixtures/📦️object-stream/🧪️compressed.pdf");

    fn grant() -> CosGrant { CosGrant { work: 1, allocation_bytes: 16_000_000, release_bytes: 16_000_000 } }

    #[test]
    fn retained_import_matches_the_reference_graph_on_every_one_unit_turn() {
        let input = COMPRESSED;
        let (_, offset, xref) = read_file_xref(input).unwrap();
        let mut decode = GraphImportCursor::new(xref.entries.len(), 1_000_000);
        let mut retained = 0usize;
        let mut published = 0;
        let output = (0..100_000).find_map(|_| {
            let step = decode.advance(&input[offset..], &xref.entries, grant(), false);
            assert!(step.work <= 1);
            retained += step.allocated_bytes;
            retained = retained.checked_sub(step.released_bytes).unwrap();
            match step.event.unwrap() {
                GraphImportEvent::More => None,
                GraphImportEvent::Object => { published += 1; None }
                GraphImportEvent::Complete => decode.take(),
                event => panic!("unexpected graph event {event:?}"),
            }
        }).expect("one-unit retained graph completion");
        let reference = Resolver::new(&input[offset..], xref.entries).resolve_all(None).unwrap();
        assert_eq!(published, output.objects.len());
        assert_eq!(output.objects, reference.objects);
        assert!(retained > 0);
    }

    #[test]
    fn cancellation_releases_only_admitted_storage_before_reporting_closed() {
        let (_, offset, xref) = read_file_xref(COMPRESSED).unwrap();
        let mut decode = GraphImportCursor::new(xref.entries.len(), 1_000_000);
        let mut retained = 0usize;
        let mut published = 0;
        for _ in 0..100_000 {
            let step = decode.advance(&COMPRESSED[offset..], &xref.entries, grant(), false);
            retained += step.allocated_bytes;
            retained = retained.checked_sub(step.released_bytes).unwrap();
            if step.event.unwrap() == GraphImportEvent::Object { published += 1; }
            if published == 3 { break; }
        }
        assert_eq!(published, 3);
        assert_eq!(decode.advance(COMPRESSED, &xref.entries, grant(), true).event.unwrap(), GraphImportEvent::Cancelled);
        let denied = CosGrant { release_bytes: 0, ..grant() };
        assert!((0..100).any(|_| matches!(decode.close_step(denied).event, Ok(GraphImportEvent::NeedsRelease { .. }))));
        let mut closed = false;
        for _ in 0..100_000 {
            let step = decode.close_step(grant());
            assert!(step.work <= 1);
            retained = retained.checked_sub(step.released_bytes).unwrap();
            if step.event.unwrap() == GraphImportEvent::Closed { closed = true; break; }
        }
        assert!(closed);
        assert_eq!(retained, 0);
        assert!(decode.take().is_none());
    }
    #[test]
    fn byte_graph_retires_structure_and_graph_after_cancellation() {
        assert!(std::mem::size_of::<PdfByteGraphCursor>() < 32 * 1024);
        assert!(PdfByteGraphCursor::projection_bytes() < 32 * 1024);
        let mut cursor = PdfByteGraphCursor::new(100, 1_000_000);
        let grant = CosGrant { work: 1, allocation_bytes: 32 * 1024, release_bytes: 32 * 1024 };
        let mut retained = 0usize;
        let mut objects = 0;
        for _ in 0..100_000 {
            let step = cursor.advance(COMPRESSED, grant, false);
            assert!(step.work <= 1);
            retained += step.allocated_bytes;
            retained = retained.checked_sub(step.released_bytes).unwrap();
            if step.event.unwrap() == PdfByteGraphEvent::Object { objects += 1; }
            if objects == 3 { break; }
        }
        assert_eq!(objects, 3);
        assert_eq!(cursor.advance(COMPRESSED, grant, true).event.unwrap(), PdfByteGraphEvent::Cancelled);
        for _ in 0..100_000 {
            let step = cursor.close_step(grant);
            assert!(step.work <= 1);
            retained = retained.checked_sub(step.released_bytes).unwrap();
            if step.event.unwrap() == PdfByteGraphEvent::Closed { assert_eq!(retained, 0); return; }
        }
        panic!("PDF byte graph did not release all of its allocations");
    }

    #[test]
    fn prepared_pdf_transcodes_original_metadata_without_a_one_turn_owner_spike() {
        assert!(std::mem::size_of::<PdfFilePreparationCursor>() < 32 * 1024);
        let grant = CosGrant { work: 1, allocation_bytes: 32 * 1024, release_bytes: 32 * 1024 };
        for cancel_before_transfer in [false, true] {
            let mut cursor = PdfFilePreparationCursor::new(100, 1_000_000);
            let mut retained = 0usize;
            let mut complete = false;
            for _ in 0..100_000 {
                let step = cursor.advance(COMPRESSED, grant, false);
                assert!(step.work <= 1);
                retained += step.allocated_bytes;
                retained = retained.checked_sub(step.released_bytes).unwrap();
                match step.event.unwrap() {
                    PdfByteGraphEvent::Complete => { complete = true; break; }
                    PdfByteGraphEvent::Object | PdfByteGraphEvent::More => {}
                    event => panic!("unexpected bounded PDF preparation status {event:?}"),
                }
            }
            assert!(complete);
            if cancel_before_transfer {
                cursor.begin_close();
                for _ in 0..100_000 {
                    let step = cursor.close_step(grant);
                    retained = retained.checked_sub(step.released_bytes).unwrap();
                    if step.event.unwrap() == PdfByteGraphEvent::Closed { break; }
                }
                assert_eq!(retained, 0);
                assert!(cursor.take().is_none());
            } else {
                let file = cursor.take().unwrap();
                let info = file.graph.objects.iter().find(|object| object.id == ObjRef { num: 6, gen: 0 }).unwrap();
                assert_eq!(info.value.dict_get("Title"), Some(&PdfObject::Text("Compressed".into())));
                assert!(!file.trailer.iter().any(|entry| WRITER_TRAILER_KEYS.contains(&entry.key.as_str())));
            }
        }
    }

    #[test]
    fn prepared_incremental_pdf_inherits_root_and_info_and_releases_each_trailer() {
        use std::io::Write;
        let prior = COMPRESSED.windows(9).rposition(|window| window == b"startxref").unwrap();
        let previous_offset = std::str::from_utf8(&COMPRESSED[prior + 9..]).unwrap().split_whitespace().next().unwrap();
        let mut data = COMPRESSED.to_vec();
        let object = data.len() + 1;
        write!(data, "\n50 0 obj\n155\nendobj\n").unwrap();
        let xref = data.len();
        write!(data, "xref\n50 1\n{object:010} 00000 n \ntrailer\n<< /Size 51 /Prev {previous_offset} >>\nstartxref\n{xref}\n%%EOF\n").unwrap();
        let (_, _, original) = read_file_xref(&data).unwrap();
        assert!(!original.brute_forced);
        for cancel in [false, true] {
            let mut cursor = PdfFilePreparationCursor::new(100, 1_000_000);
            let grant = CosGrant { work: 1, allocation_bytes: 32 * 1024, release_bytes: 32 * 1024 };
            let mut retained = 0usize;
            for _ in 0..100_000 {
                let step = cursor.advance(&data, grant, false);
                assert!(step.work <= 1);
                retained = (retained + step.allocated_bytes).checked_sub(step.released_bytes).unwrap();
                match step.event.unwrap() {
                    PdfByteGraphEvent::More | PdfByteGraphEvent::Object => {}
                    PdfByteGraphEvent::Complete => break,
                    event => panic!("unexpected incremental PDF preparation status {event:?}"),
                }
            }
            assert!(matches!(cursor.phase, PreparePhase::Ready));
            if cancel {
                cursor.begin_close();
                for _ in 0..100_000 {
                    let step = cursor.close_step(grant);
                    retained = retained.checked_sub(step.released_bytes).unwrap();
                    if step.event.unwrap() == PdfByteGraphEvent::Closed { break; }
                }
                assert!(matches!(cursor.phase, PreparePhase::Closed));
                assert_eq!(retained, 0);
            } else {
                let file = cursor.take().unwrap();
                let expected: Vec<_> = original.trailer.iter().filter(|entry| retain_trailer_entry(entry, false)).cloned().collect();
                assert_eq!(file.trailer, expected);
                assert!(file.graph.objects.iter().any(|object| object.id.num == 50 && object.value == PdfObject::Int(155)));
            }
        }
    }

    #[test]
    fn inherited_trailer_backing_can_grow_with_one_work_unit() {
        let mut cursor = PdfFilePreparationCursor::new(0, 0);
        let ordered = ["CustomFirst", "CustomSecond", "CustomThird", "Root", "Info"];
        cursor.trailer = Vec::from(ordered[..3].iter().map(|key| PdfDictEntry::new(*key, PdfObject::Null)).collect::<Vec<_>>().into_boxed_slice());
        cursor.inherited[0] = Some(PdfDictEntry::new("Root", PdfObject::Ref(ObjRef { num: 1, gen: 0 })));
        cursor.inherited[1] = Some(PdfDictEntry::new("Info", PdfObject::Ref(ObjRef { num: 6, gen: 0 })));
        cursor.graph = Some(ImportedGraph { objects: Vec::new(), positions: XrefIndex::default() });
        cursor.phase = PreparePhase::AppendInherited;
        let grant = CosGrant { work: 1, allocation_bytes: 4096, release_bytes: 4096 };
        let refused = cursor.append_inherited(CosGrant { allocation_bytes: 0, ..grant });
        assert!(matches!(refused.event, Ok(PdfByteGraphEvent::NeedsAllocation { .. })));
        let mut released = 0;
        let mut allocated = 0;
        for _ in 0..30 {
            let step = cursor.append_inherited(grant);
            assert!(matches!(step.event, Ok(PdfByteGraphEvent::More)) && step.work == 1);
            released += step.released_bytes;
            allocated += step.allocated_bytes;
            if matches!(cursor.phase, PreparePhase::Text) { break; }
        }
        assert!(matches!(cursor.phase, PreparePhase::Text));
        assert!(allocated >= ordered.len() * std::mem::size_of::<PdfDictEntry>());
        assert!(released >= 3 * std::mem::size_of::<PdfDictEntry>());
        assert!(cursor.trailer.iter().map(|entry| entry.key.as_str()).eq(ordered));
    }

}
