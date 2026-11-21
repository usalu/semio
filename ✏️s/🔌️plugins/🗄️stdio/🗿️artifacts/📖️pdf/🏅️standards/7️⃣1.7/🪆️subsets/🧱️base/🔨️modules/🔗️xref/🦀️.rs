//! 🔗️ File structure (ISO 32000-1 §7.5): classic cross-reference tables, cross-reference streams,
//! hybrid files, `/Prev` chains, object streams, and the brute-force `N G obj` scan every shipping
//! reader falls back to. The [`Resolver`] materializes indirect objects on demand (decrypting
//! them when the document carries a standard security handler) and can flatten the whole graph
//! into the snapshot's retained `objects` lane.

use crate::standards::v1_7::subsets::base::schema::graph_source::ObjectSource;
use crate::standards::v1_7::subsets::base::modules::encryption::Decryptor;
use crate::standards::v1_7::subsets::base::modules::filters::decode_stream_admitted;
use crate::standards::v1_7::subsets::base::modules::lexer::{dict_i64, find_last_subslice, frame_indirect_with_length, ObjectScanCursor, ObjectScanStep, NormalObject, PdfHeader, PdfIndirectLengthCursor, PdfStreamDictionaryCursor, retain_stream_entry, CosGrant, CosLimits, CosStatus, Lexer};
use crate::standards::v1_7::subsets::base::io::error::{malformed, PResult, PdfEngineError};
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfStreamFilter};
use std::collections::{HashMap, HashSet};

#[path = "🌐️file-structure/🦀️.rs"]
mod file_structure;
pub use file_structure::{PdfFileStructure, PdfFileStructureCursor, PdfFileStructureEvent, PdfFileStructureReceipt};


pub use semio_s_artifact_stdio_pdf_xref_stream::{ObjectStreamCursor, ObjectStreamEvent, ObjectStreamReceipt, ObjectStreamHeaderCursor, ObjectStreamHeaderEvent, ObjectStreamHeaderReceipt, ObjectStreamMemberCursor, XrefEntry, XrefStreamError, XrefStreamEvent, XrefStreamReceipt, XrefStreamRowCursor, XrefStreamSectionCursor};
pub use semio_s_artifact_stdio_pdf_xref_stream::{RevisionIndex, XrefIndex, XrefIndexLookup, XrefIndexScan, XrefIndexStatus, XrefRow};
pub use semio_s_artifact_stdio_pdf_xref_stream::{ClassicSectionCursor, ClassicSectionEvent, ClassicSectionReceipt, ClassicXrefCursor, ClassicXrefEvent, ClassicXrefReceipt, FooterCursor, FooterStep};

pub use semio_s_artifact_stdio_pdf_xref_stream::{
    NormalProjection, NormalProjectionCursor, NormalProjectionEvent, NormalProjectionReceipt,
    StructuredXrefCursor, StructuredXrefEvent, StructuredXrefReceipt,
    XrefFileOrderCursor, XrefFileOrderEvent, XrefFileOrderReceipt,
    GraphMaterializationEvent, GraphMaterializationReceipt, RetainedGraphBuilder,
    StructuredTrailer, XrefTrailerCursor, XrefIndexReceipt,
    XrefAssemblyCursor, XrefAssemblyEvent, XrefAssemblyReceipt,
    XrefOffsetCursor, XrefOffsetStatus, XrefRevisionCursor, XrefRevisionEvent, XrefRevisionReceipt,
};

/// 🧮 Trailer keys the writer derives from the file it lays out (ISO 32000-1 §7.5.5, §7.5.8.2) —
/// never document content, so a reader drops them and a writer states them afresh.
pub const WRITER_TRAILER_KEYS: [&str; 10] = ["Size", "Prev", "XRefStm", "Encrypt", "Type", "W", "Index", "Filter", "DecodeParms", "Length"];
//#region 🔖️Entries
/// 🧾 The merged cross-reference state of a file: every live entry plus the first (newest)
/// trailer dictionary.
pub struct XrefState {
    pub entries: XrefIndex,
    pub trailer: Vec<PdfDictEntry>,
    pub brute_forced: bool,
}
//#endregion 🔖️Entries

//#region 🔖️Parsing
/// 🧵 Follows `/Prev` (and hybrid `/XRefStm`) chains from `start_offset`, merging older sections
/// without overwriting newer entries. Falls back to a brute-force `N G obj` scan if the structured
/// chain can't even be started, and to the scan for any offset that does not point at its object.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn build_xref(data: &[u8]) -> PResult<XrefState> {
    collect_xref(data, true).ok_or_else(|| PdfEngineError::Malformed("unable to reconstruct the PDF cross-reference table".into()))
}

/// 🔍️ Reads an intact xref without entering recovery, for selecting the file’s offset convention.
pub fn structured_xref(data: &[u8]) -> Option<XrefState> {
    collect_xref(data, false)
}

fn collect_xref(data: &[u8], repair: bool) -> Option<XrefState> {
    let mut cursor = StructuredXrefCursor::new(data.len(), CosLimits::default(), usize::MAX);
    let grant = CosGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
    let result = loop {
        match cursor.advance(data, grant, false).event {
            Ok(StructuredXrefEvent::More) => {}
            Ok(StructuredXrefEvent::RecoveryRequired) => {
                if cursor.recover_compatible(data).ok() != Some(true) && !cursor.skip_failed() { break None; }
            }
            Ok(StructuredXrefEvent::Complete { offsets_valid }) => break cursor.take().map(|(index, trailer)| (index, trailer, offsets_valid)),
            _ => break None,
        }
    };
    cursor.begin_close();
    loop {
        match cursor.close_step(grant).event {
            Ok(StructuredXrefEvent::Closed) => break,
            Ok(StructuredXrefEvent::More) => {}
            _ => return None,
        }
    }
    let (entries, primary_trailer, offsets_agree) = result?;
    let root_resolvable = primary_trailer.get_inheritable("Root")
        .and_then(PdfObject::as_ref)
        .is_some_and(|root| entries.get(&root.num).is_some_and(|entry| generation(entry) == root.gen));
    finish_xref(data, entries, primary_trailer.into_entries(), offsets_agree, root_resolvable, repair)
}

fn finish_xref(data: &[u8], mut entries: XrefIndex, mut trailer: Vec<PdfDictEntry>, offsets_agree: bool, root_resolvable: bool, repair: bool) -> Option<XrefState> {
    let brute_forced = !offsets_agree || !root_resolvable;
    if brute_forced {
        if !repair { return None; }
        recover_xref(data, &mut entries, &mut trailer, !root_resolvable)?;
    }
    Some(XrefState { entries, trailer, brute_forced })
}

/// 🗺️ Reads the authored header and assembles the index in the selected source coordinates.
pub fn read_file_xref(source: &[u8]) -> PResult<(PdfHeader, usize, XrefState)> {
    let mut cursor = PdfFileStructureCursor::new(usize::MAX);
    let grant = CosGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
    let file = loop {
        match cursor.advance(source, grant, false).event {
            Ok(PdfFileStructureEvent::More) => {}
            Ok(PdfFileStructureEvent::NotPdf) => break Err(PdfEngineError::NotPdf),
            Ok(PdfFileStructureEvent::RecoveryRequired) if cursor.recover_compatible(source) => {}
            Ok(PdfFileStructureEvent::Complete) => break cursor.take().ok_or_else(|| PdfEngineError::Malformed("completed PDF structure could not be transferred".into())),
            Err(XrefStreamError::Structure(reason)) => break malformed(reason),
            Err(XrefStreamError::Decode(error)) => break Err(PdfEngineError::Malformed(error.into_message())),
            _ => break malformed("unable to assemble structured PDF input"),
        }
    };
    cursor.begin_close();
    loop {
        match cursor.close_step(grant).event {
            Ok(PdfFileStructureEvent::Closed) => break,
            Ok(PdfFileStructureEvent::More) => {}
            _ => return malformed("unable to close PDF source structure"),
        }
    }

    let PdfFileStructure { header, origin, index, trailer, intact, root_resolves } = file?;
    let xref = finish_xref(&source[origin..], index, trailer.into_entries(), intact, root_resolves, true)
        .ok_or_else(|| PdfEngineError::Malformed("unable to reconstruct the PDF cross-reference table".into()))?;
    Ok((header, origin, xref))
}

fn recover_xref(data: &[u8], entries: &mut XrefIndex, trailer: &mut Vec<PdfDictEntry>, needs_root: bool) -> Option<()> {
    let mut scanner = ObjectScanCursor::new();
    let mut catalog = None;
    let mut stream_root = None;
    let mut streams = Vec::new();

    loop {
        let (id, offset) = match scanner.advance(data, 4096).1 {
            ObjectScanStep::Found { id, offset } => (id, offset),
            ObjectScanStep::More => continue,
            ObjectScanStep::Complete => break,
        };
        let Ok(object) = frame_indirect_with_length(data, offset, &mut |_| None) else { continue };
        let resume = object.stream.filter(|span| span.resume > span.end).map_or(object.value_end, |span| span.resume);
        scanner.resume_after(data, resume);
        if entries.is_free(id.num) { continue; }
        entries.insert_row(XrefRow { number: id.num, entry: Some(XrefEntry::Normal { offset, gen: id.gen }) }, true).ok()?;
        match object.value.dict_get("Type").and_then(PdfObject::as_name) {
            Some("Catalog") if needs_root => catalog = Some(id),
            Some("XRef") if needs_root => {
                if let Some(root) = object.value.dict_get("Root").and_then(PdfObject::as_ref) {
                    stream_root = Some(root);
                }
            }
            Some("ObjStm") => {
                if let (PdfObject::Dict(dict), Some(span)) = (object.value, object.stream) {
                    streams.push((id.num, offset, dict, span));
                }
            }
            _ => {}
        }
    }

    for (stream_num, offset, dict, span) in streams.into_iter().rev() {
        if !matches!(entries.get(&stream_num), Some(XrefEntry::Normal { offset: current, .. }) if *current == offset) { continue; }
        let Some((body, count, first)) = open_object_stream(&dict, &data[span.start..span.end]) else { continue };
        let mut cursor = ObjectStreamHeaderCursor::new(count, first);
        let mut index = 0;
        loop {
            match cursor.advance(&body, 4096, false).event {
                Ok(ObjectStreamHeaderEvent::Member { number, .. }) => {
                    entries.insert_row(XrefRow { number, entry: Some(XrefEntry::Compressed { stream_num, index }) }, false).ok()?;
                    index = index.checked_add(1)?;
                }
                Ok(ObjectStreamHeaderEvent::More) => {}
                _ => break,
            }
        }
    }

    if needs_root {
        trailer.retain(|entry| entry.key != "Root");
        if let Some(root) = catalog.or(stream_root) {
            trailer.insert(0, PdfDictEntry::new("Root", PdfObject::Ref(root)));
        } else if let Some(offset) = find_last_subslice(data, b"trailer") {
            if let Ok(PdfObject::Dict(entries)) = Lexer::new(data).at(offset + 7).parse_object() {
                for entry in entries {
                    if !trailer.iter().any(|existing| existing.key == entry.key) { trailer.push(entry); }
                }
            }
        }
    }
    Some(())
}

/// 🔍 Locates `startxref` and returns the offset it points at, if any.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn startxref_offset(data: &[u8]) -> Option<usize> {
    let mut cursor = FooterCursor::new(data.len());
    loop {
        if let FooterStep::Complete(offset) = cursor.advance(data, 4096) {
            return offset;
        }
    }
}
//#endregion 🔖️Parsing

//#region 🔖️Resolver
/// 🗃️ The retained objects and their original generation-aware positions.
pub struct ResolvedGraph {
    pub objects: Vec<PdfIndirectObject>,
    pub positions: XrefIndex,
}

struct ResolvedObjectStream {
    bytes: Vec<u8>,
    count: usize,
    first: usize,
    members: Option<Vec<(u32, usize)>>,
}

impl ResolvedObjectStream {
    fn member_directory(&mut self) -> Option<&[(u32, usize)]> {
        if self.members.is_none() {
            let mut header = ObjectStreamHeaderCursor::new(self.count, self.first);
            let mut members = Vec::new();
            loop {
                match header.advance(&self.bytes, 4096, false).event.ok()? {
                    ObjectStreamHeaderEvent::More => {}
                    ObjectStreamHeaderEvent::Member { number, offset } => members.push((number, offset)),
                    ObjectStreamHeaderEvent::Complete => break,
                    ObjectStreamHeaderEvent::Cancelled => return None,
                }
            }
            self.members = Some(members);
        }
        self.members.as_deref()
    }

}

fn resolve_ordered_member(cursor: &mut ObjectStreamCursor, bytes: &[u8], number: u32, index: u32, grant: CosGrant) -> Option<PdfObject> {
    cursor.select(number, index).ok()?;
    let value = loop {
        match cursor.advance(bytes, grant, false).event.ok()? {
            ObjectStreamEvent::More => {}
            ObjectStreamEvent::Complete { .. } => break cursor.take_value()?,
            _ => return None,
        }
    };
    loop {
        match cursor.advance(bytes, grant, false).event.ok()? {
            ObjectStreamEvent::More => {}
            ObjectStreamEvent::Idle => return Some(value),
            _ => return None,
        }
    }
}

fn close_ordered_member(cursor: &mut Option<ObjectStreamCursor>, grant: CosGrant) {
    let Some(mut cursor) = cursor.take() else { return; };
    cursor.begin_close();
    while cursor.close_step(grant).event == Ok(ObjectStreamEvent::More) {}
}

/// 🧭️ Resolves indirect objects, compressed members, and encrypted stream bodies.
pub struct Resolver<'a> {
    data: &'a [u8],
    pub xref: XrefIndex,
    cache: HashMap<u32, PdfObject>,
    object_streams: HashMap<u32, ResolvedObjectStream>,
    ordered_stream: Option<(u32, Option<ObjectStreamCursor>)>,
    decryptor: Option<Decryptor>,
    resolving: HashSet<u32>,
}

impl<'a> Resolver<'a> {
    pub fn new(data: &'a [u8], xref: XrefIndex) -> Self {
        Self { data, xref, cache: HashMap::new(), object_streams: HashMap::new(), ordered_stream: None, decryptor: None, resolving: HashSet::new() }
    }

    pub fn set_decryptor(&mut self, decryptor: Option<Decryptor>) {
        self.decryptor = decryptor;
        self.cache.clear();
        self.object_streams.clear();
        self.ordered_stream = None;
    }

    /// 🎯 The object `num` refers to, undecoded (stream data still filtered) but decrypted.
    pub fn resolve(&mut self, num: u32) -> Option<PdfObject> {
        if let Some(v) = self.cache.get(&num) {
            return Some(v.clone());
        }
        let value = self.read_uncached(num)?;
        self.cache.insert(num, value.clone());
        Some(value)
    }

    /// 🔗️ Resolves only the generation requested by an original indirect reference.
    pub fn resolve_reference(&mut self, reference: ObjRef) -> Option<PdfObject> {
        if generation(self.xref.get(&reference.num)?) != reference.gen { return None; }
        self.resolve(reference.num)
    }

    fn read_uncached(&mut self, num: u32) -> Option<PdfObject> {
        self.read_graph_object(num, false).ok().flatten()
    }

    fn read_graph_object(&mut self, num: u32, project: bool) -> PResult<Option<PdfObject>> {
        if !self.resolving.insert(num) { return Ok(None); }
        let result = self.read_entry(num, project);
        self.resolving.remove(&num);
        result
    }

    fn resolve_stream_length(&mut self, reference: ObjRef) -> Option<usize> {
        let entry = *self.xref.get(&reference.num)?;
        if generation(&entry) != reference.gen || self.resolving.contains(&reference.num) { return None; }
        if let Some(value) = self.cache.get(&reference.num) {
            return value.as_i64().and_then(|length| usize::try_from(length).ok());
        }
        match entry {
            XrefEntry::Normal { offset, .. } => {
                let mut cursor = PdfIndirectLengthCursor::new(reference, offset);
                let grant = CosGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
                let length = loop {
                    match cursor.advance(self.data, grant, false).status {
                        Ok(CosStatus::More) => {}
                        Ok(CosStatus::Complete { .. }) => break cursor.length(),
                        _ => break None,
                    }
                };
                cursor.begin_close();
                while cursor.close_step(grant).status == Ok(CosStatus::More) {}
                length
            }
            XrefEntry::Compressed { stream_num, index } => {
                self.resolving.insert(reference.num);
                let value = self.resolve_compressed(stream_num, index, reference.num, false);
                self.resolving.remove(&reference.num);
                value.and_then(|value| value.as_i64()).and_then(|length| usize::try_from(length).ok())
            }
        }
    }

    fn frame_normal(&mut self, id: ObjRef, offset: usize) -> Option<NormalObject> {
        let object = frame_indirect_with_length(self.data, offset, &mut |reference| self.resolve_stream_length(reference)).ok()?;
        (object.id == id).then_some(object)
    }

    fn project_normal(&mut self, id: ObjRef, offset: usize) -> PResult<Option<PdfObject>> {
        let mut cursor = NormalProjectionCursor::new(id, offset, CosLimits::default(), usize::MAX);
        let grant = CosGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
        let result = loop {
            match cursor.advance(self.data, grant, false).event {
                Ok(NormalProjectionEvent::More) => {}
                Ok(NormalProjectionEvent::NeedsLength(reference)) => {
                    let length = self.resolve_stream_length(reference);
                    if let Err(reason) = cursor.provide_length(reference, length) { break malformed(reason); }
                }
                Ok(NormalProjectionEvent::Complete) => break Ok(cursor.take()),
                Err(XrefStreamError::Structure(reason)) => break malformed(reason),
                Err(XrefStreamError::Decode(error)) => break Err(PdfEngineError::Malformed(error.into_message())),
                Ok(_) => break malformed("unexpected retained PDF object state"),
            }
        };
        cursor.begin_close();
        loop {
            match cursor.close_step(grant).event {
                Ok(NormalProjectionEvent::Closed) => break,
                Ok(NormalProjectionEvent::More) => {}
                _ => return malformed("unable to close the retained PDF object cursor"),
            }
        }
        match result? {
            Some(NormalProjection::Value(value)) => Ok(Some(value)),
            Some(NormalProjection::ReferenceStream { dict, span }) => normalize_framed_stream(dict, &self.data[span.start..span.end]).map(Some),
            Some(NormalProjection::Omitted | NormalProjection::ObjectStream(_)) => Ok(None),
            None => malformed("completed PDF object was unavailable"),
        }
    }

    fn read_entry(&mut self, num: u32, project: bool) -> PResult<Option<PdfObject>> {
        let value = match self.xref.get(&num).copied() {
            Some(XrefEntry::Normal { offset, gen }) => {
                let id = ObjRef { num, gen };
                if project && self.decryptor.is_none() { return self.project_normal(id, offset); }
                let Some(NormalObject { value, stream, .. }) = self.frame_normal(id, offset) else { return Ok(None); };
                let value = match (value, stream) {
                    (PdfObject::Dict(dict), Some(span)) => {
                        let raw = &self.data[span.start..span.end];
                        let stream_kind = dict.iter().find(|entry| entry.key == "Type").and_then(|entry| entry.value.as_name());
                        if project && matches!(stream_kind, Some("XRef" | "ObjStm")) { return Ok(None); }
                        PdfObject::Stream { dict, data: raw.to_vec(), filters: Vec::new() }
                    }
                    (value, _) => value,
                };
                match (&self.decryptor, &value) {
                    (Some(decryptor), _) if !is_xref_stream(&value) => Some(decryptor.decrypt_object(value, num, id.gen)),
                    _ => Some(value),
                }
            }
            Some(XrefEntry::Compressed { stream_num, index }) => self.resolve_compressed(stream_num, index, num, project),
            None => None,
        };
        if project {
            value.map(normalize_retained_object).transpose()
        } else {
            Ok(value)
        }
    }

    fn load_object_stream(&mut self, number: u32) -> Option<ResolvedObjectStream> {
        let (bytes, count, first) = if self.decryptor.is_some() || self.cache.contains_key(&number) {
            let source = self.cache.remove(&number).or_else(|| self.read_uncached(number))?;
            if source.dict_get("Type").and_then(PdfObject::as_name) != Some("ObjStm") { return None; }
            let PdfObject::Stream { dict, data, .. } = source else { return None; };
            open_object_stream(&dict, &data)?
        } else {
            let XrefEntry::Normal { offset, gen } = *self.xref.get(&number)? else { return None; };
            if !self.resolving.insert(number) { return None; }
            let frame = self.frame_normal(ObjRef { num: number, gen }, offset);
            self.resolving.remove(&number);
            let frame = frame?;
            if frame.value.dict_get("Type").and_then(PdfObject::as_name) != Some("ObjStm") { return None; }
            let (PdfObject::Dict(dict), Some(span)) = (frame.value, frame.stream) else { return None; };
            open_object_stream(&dict, &self.data[span.start..span.end])?
        };

        Some(ResolvedObjectStream { bytes, count, first, members: None })
    }

    fn resolve_compressed(&mut self, stream_num: u32, index: u32, number: u32, project: bool) -> Option<PdfObject> {
        let grant = CosGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
        if !self.object_streams.contains_key(&stream_num) {
            let stream = self.load_object_stream(stream_num)?;
            self.object_streams.insert(stream_num, stream);
        }
        let stream = self.object_streams.get_mut(&stream_num)?;
        if project {
            if self.ordered_stream.as_ref().map(|(number, _)| *number) != Some(stream_num) {
                if let Some((_, mut prior)) = self.ordered_stream.take() { close_ordered_member(&mut prior, grant); }
                self.ordered_stream = Some((stream_num, Some(ObjectStreamCursor::new(stream.count, stream.first, CosLimits::default()))));
            }
            let (_, sequential) = self.ordered_stream.as_mut().expect("ordered object stream");
            if let Some(cursor) = sequential.as_mut() {
                if let Some(value) = resolve_ordered_member(cursor, &stream.bytes, number, index, grant) { return Some(value); }
                close_ordered_member(sequential, grant);
            }
        }
        let length = stream.bytes.len();
        let members = stream.member_directory()?;
        let index = index as usize;
        let &(embedded, start) = members.get(index)?;
        if embedded != number { return None; }
        let end = members.get(index + 1).map_or(length, |&(_, offset)| offset);
        let bytes = &stream.bytes;
        let mut member = ObjectStreamMemberCursor::new(start, end, CosLimits::default());
        let value = loop {
            match member.advance(bytes, grant, false).status {
                Ok(CosStatus::More) => {}
                Ok(CosStatus::Complete { .. }) => break member.take_value(),
                _ => break None,
            }
        };
        member.begin_close();
        while matches!(member.close_step(grant).status, Ok(CosStatus::More)) {}
        value
    }

    /// 📚️ Materializes every entry of the xref table into `PdfIndirectObject`s with decoded
    /// stream data (the retained-graph lane), in file order.
    pub fn resolve_all(&mut self, omit: Option<u32>) -> PResult<ResolvedGraph> {
        let mut order = XrefFileOrderCursor::default();
        let grant = CosGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
        let mut graph = RetainedGraphBuilder::new(self.xref.len());
        let result = (|| {
            settle_graph_builder(&mut graph, grant)?;
            loop {
                let num = match order.advance(&self.xref, grant, false).event {
                    Ok(XrefFileOrderEvent::Object(number)) => number,
                    Ok(XrefFileOrderEvent::More) => continue,
                    Ok(XrefFileOrderEvent::Complete) => break,
                    Err(error) => return malformed(error),
                    Ok(_) => return malformed("unexpected PDF object scheduler state"),
                };
                if omit == Some(num) { continue; }
                let value = match self.cache.remove(&num) {
                    Some(value) => Some(normalize_retained_object(value)?),
                    None => self.read_graph_object(num, true)?,
                };
                if let Some(value) = value {
                    if value.dict_get("Type").and_then(PdfObject::as_name) == Some("ObjStm") {
                        self.cache.insert(num, value);
                        continue;
                    }
                    if is_xref_stream(&value) {
                        continue;
                    }
                    let gen = self.xref.get(&num).map_or(0, generation);
                    graph.offer(PdfIndirectObject { id: ObjRef { num, gen }, value })
                        .map_err(|_| PdfEngineError::Malformed("retained PDF object could not enter its original graph position".into()))?;
                    settle_graph_builder(&mut graph, grant)?;
                }
            }
            let (objects, positions) = graph.finish().expect("settled ordered PDF graph");
            Ok(ResolvedGraph { objects, positions })
        })();
        if let Some((_, mut prior)) = self.ordered_stream.take() { close_ordered_member(&mut prior, grant); }
        order.begin_close();
        if order.close_step(grant).event != Ok(XrefFileOrderEvent::Closed) {
            return malformed("unable to close PDF object ordering");
        }
        graph.begin_close();
        loop {
            match graph.close_step(grant).event {
                Ok(GraphMaterializationEvent::Closed) => break,
                Ok(GraphMaterializationEvent::More) => {}
                Ok(GraphMaterializationEvent::RetireObject) => { drop(graph.take_retired_object()); }
                _ => return malformed("unable to close PDF graph materialization"),
            }
        }
        result
    }
}

fn settle_graph_builder(graph: &mut RetainedGraphBuilder, grant: CosGrant) -> PResult<()> {
    loop {
        match graph.advance(grant, false).event {
            Ok(GraphMaterializationEvent::Ready) => return Ok(()),
            Ok(GraphMaterializationEvent::More) => {}
            Err(reason) => return malformed(reason),
            _ => return malformed("unexpected PDF graph materialization state"),
        }
    }
}

fn generation(entry: &XrefEntry) -> u16 {
    match entry {
        XrefEntry::Normal { gen, .. } => *gen,
        XrefEntry::Compressed { .. } => 0,
    }
}

fn open_object_stream(dict: &[PdfDictEntry], raw: &[u8]) -> Option<(Vec<u8>, usize, usize)> {
    let count = usize::try_from(dict_i64(dict, "N")?).ok()?;
    let first = usize::try_from(dict_i64(dict, "First")?).ok()?;
    let (body, _) = decode_stream_admitted(dict, raw).ok()?;
    Some((body, count, first))
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn is_xref_stream(value: &PdfObject) -> bool {
    matches!(value, PdfObject::Stream { .. }) && value.dict_get("Type").and_then(PdfObject::as_name) == Some("XRef")
}

/// 🧹 Converts parsed COS into snapshot form: stream filters with logical decoders are applied,
/// their declarations dropped from the dictionary and kept as the typed pipeline.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn normalize_pdf_object(value: PdfObject) -> PResult<PdfObject> {
    match value {
        PdfObject::Array(items) => Ok(PdfObject::Array(items.into_iter().map(normalize_pdf_object).collect::<PResult<_>>()?)),
        PdfObject::Dict(entries) => Ok(PdfObject::Dict(entries.into_iter().map(|entry| Ok(PdfDictEntry { key: entry.key, value: normalize_pdf_object(entry.value)? })).collect::<PResult<_>>()?)),
        PdfObject::Stream { dict, data, filters } => normalize_stream(dict, data, filters),
        value => Ok(value),
    }
}
fn normalize_retained_object(value: PdfObject) -> PResult<PdfObject> {
    if is_xref_stream(&value) || value.dict_get("Type").and_then(PdfObject::as_name) == Some("ObjStm") {
        Ok(value)
    } else {
        normalize_pdf_object(value)
    }
}

fn normalize_stream(dict: Vec<PdfDictEntry>, data: Vec<u8>, filters: Vec<PdfStreamFilter>) -> PResult<PdfObject> {
    let (data, filters, normalized) = if filters.is_empty() {
        retained_stream_data(&dict, std::borrow::Cow::Owned(data))?
    } else {
        (data, filters, true)
    };
    let dict = dict.into_iter()
        .filter(|entry| retain_stream_entry(entry, normalized))
        .map(|entry| Ok(PdfDictEntry { key: entry.key, value: normalize_pdf_object(entry.value)? }))
        .collect::<PResult<_>>()?;
    Ok(PdfObject::Stream { dict, data, filters })
}

fn normalize_framed_stream(dict: Vec<PdfDictEntry>, raw: &[u8]) -> PResult<PdfObject> {
    let (data, filters, normalized) = retained_stream_data(&dict, std::borrow::Cow::Borrowed(raw))?;
    let mut cursor = PdfStreamDictionaryCursor::new(dict, normalized);
    let grant = CosGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
    loop {
        match cursor.advance(grant, false).status {
            Ok(CosStatus::Complete { .. }) => break,
            Ok(CosStatus::More) => {}
            _ => return malformed("unable to project the PDF stream dictionary"),
        }
    }
    let dict = cursor.take_dictionary().expect("completed native stream dictionary");
    Ok(PdfObject::Stream { dict, data, filters })
}

fn retained_stream_data(dict: &[PdfDictEntry], raw: std::borrow::Cow<'_, [u8]>) -> PResult<(Vec<u8>, Vec<PdfStreamFilter>, bool)> {
    if !dict.iter().any(|entry| matches!(entry.key.as_str(), "Filter" | "F")) {
        return Ok((raw.into_owned(), Vec::new(), true));
    }
    match decode_stream_admitted(dict, &raw) {
        Ok((data, filters)) => Ok((data, filters, true)),
        Err(PdfEngineError::Unsupported(_)) | Err(PdfEngineError::Malformed(_)) => Ok((raw.into_owned(), Vec::new(), false)),
        Err(error) => Err(error),
    }
}

//#endregion 🔖️Resolver

//#region 🔖️Sources
impl ObjectSource for Resolver<'_> {
    fn get(&mut self, reference: ObjRef) -> Option<PdfObject> {
        self.resolve_reference(reference).map(|value| normalize_pdf_object(value).unwrap_or(PdfObject::Null))
    }
}

/// 🧱 Anything that can allocate indirect objects while a typed lane is lowered.
pub trait ObjectSink {
    /// ➕ Stores `value` as a new indirect object and returns its reference.
    fn add(&mut self, value: PdfObject) -> ObjRef;
    /// 📦️ Resolves admitted foreign native bodies only at the native lowering boundary.
    fn resolve_artifact(&self, _reference: &semio_framework_artifact_reference::ArtifactRef) -> PResult<PdfObject> { Err(PdfEngineError::Unsupported("native artifact resolution is unavailable".into())) }
}
//#endregion 🔖️Sources

//#region 🧪️Tests

//#endregion 🧪️Tests

#[cfg(test)]
mod retained_graph_tests {
    use super::*;

    #[test]
    fn structured_xref_checks_exact_object_generations_under_one_work_unit() {
        let source = include_bytes!("../../🧪️tests/🧫️fixtures/🔗️incremental-free/📑️table.pdf");
        let rows = structured_xref(source).unwrap().entries;
        let mut validation = XrefOffsetCursor::default();
        loop {
            let (work, status) = validation.advance(source, &rows, 1, false);
            assert!(work <= 1);
            if status == XrefOffsetStatus::Complete { break; }
            assert_eq!(status, XrefOffsetStatus::More);
        }
        let Some(XrefEntry::Normal { offset, gen }) = rows.get(&1).copied() else { panic!("fixture catalog is indirect") };
        let wrong = ObjRef { num: 1, gen: gen + 1 };
        let mut rows = XrefIndex::default();
        rows.insert_row(XrefRow { number: 1, entry: Some(XrefEntry::Normal { offset, gen: wrong.gen }) }, false).unwrap();
        let mut validation = XrefOffsetCursor::default();
        loop {
            let (work, status) = validation.advance(source, &rows, 1, false);
            assert!(work <= 1);
            match status {
                XrefOffsetStatus::More => {}
                XrefOffsetStatus::Invalid { id, offset: target } => {
                    assert_eq!((id, target), (wrong, offset));
                    break;
                }
                other => panic!("unexpected validation result {other:?}"),
            }
        }
        assert_eq!(XrefOffsetCursor::default().advance(source, &rows, 1, true), (0, XrefOffsetStatus::Cancelled));
    }

    #[test]
    fn repaired_xref_ignores_object_headers_inside_compound_bodies() {
        let source = include_bytes!("../../🧪️tests/🧫️fixtures/🩹️repair-stream.pdf");
        let index = build_xref(source).unwrap();
        assert!(index.brute_forced);
        assert_eq!(index.entries.len(), 7);
        assert_eq!(index.trailer[0], PdfDictEntry::new("Root", PdfObject::Ref(ObjRef { num: 1, gen: 0 })));
        let mut resolver = Resolver::new(source, index.entries);
        assert!(resolver.resolve_reference(ObjRef { num: 1, gen: 1 }).is_none());
        let catalog = resolver.resolve_reference(ObjRef { num: 1, gen: 0 }).unwrap();
        assert_eq!(catalog.dict_get("Pages").and_then(PdfObject::as_ref), Some(ObjRef { num: 2, gen: 0 }));
        assert_eq!(resolver.resolve(5).unwrap().dict_get("BaseFont").and_then(PdfObject::as_name), Some("Helvetica"));
    }

    #[test]
    fn file_structure_retries_header_relative_offsets_after_retiring_absolute_candidates() {
        let source = include_bytes!("../../🧪️tests/🧫️fixtures/🔗️incremental-free/📑️table.pdf");
        let prefix = b"transport prefix\r\n";
        let mut bytes = prefix.to_vec();
        bytes.extend_from_slice(source);
        let mut cursor = PdfFileStructureCursor::new(4096);
        let one = CosGrant { work: 1, allocation_bytes: 128 * 1024, release_bytes: 128 * 1024 };
        let admitted = (0..20_000).find_map(|_| {
            let step = cursor.advance(&bytes, one, false);
            assert!(step.work <= 1);
            if step.event.unwrap() == PdfFileStructureEvent::Complete { cursor.take() } else { None }
        }).expect("preambled PDF structure");
        assert_eq!(admitted.header.offset(), prefix.len());
        assert_eq!(admitted.origin, prefix.len());
        assert!(admitted.intact && admitted.root_resolves && admitted.index.is_free(3));
        let (_, origin, state) = read_file_xref(&bytes).unwrap();
        assert_eq!(origin, prefix.len());
        assert!(!state.brute_forced);
    }

    #[test]
    fn projected_graph_consumes_compressed_members_without_a_random_access_directory() {
        let pdf = include_bytes!("../../🧪️tests/🧫️fixtures/📦️object-stream/🧪️compressed.pdf");
        let index = build_xref(pdf).unwrap();
        let mut resolver = Resolver::new(pdf, index.entries);
        let ResolvedGraph { objects, positions } = resolver.resolve_all(None).unwrap();
        let object = |number| match positions.get(&number).unwrap() {
            XrefEntry::Normal { offset, gen: 0 } => &objects[*offset].value,
            _ => panic!("expected exact compressed member position"),
        };
        assert_eq!(object(6).dict_get("Title").and_then(PdfObject::as_str_bytes), Some(b"Compressed".as_slice()));
        assert_eq!(object(7).as_array().unwrap()[3].as_ref(), Some(ObjRef { num: 6, gen: 0 }));
        assert!(resolver.object_streams.get(&4).unwrap().members.is_none());
        assert!(resolver.ordered_stream.is_none());
    }

    #[test]
    fn projected_graph_retains_decoded_bytes_and_the_typed_filter_pipeline() {
        let source = include_bytes!("../🗜️filters/🪣️stream-data/🧪️tests/🔬️unit/🧪️streams.pdf");
        let index = build_xref(source).unwrap();
        let ResolvedGraph { objects: graph, positions } = Resolver::new(source, index.entries).resolve_all(None).unwrap();
        for (offset, object) in graph.iter().enumerate() {
            assert_eq!(positions.get(&object.id.num), Some(&XrefEntry::Normal { offset, gen: object.id.gen }));
        }
        let bytes = |number| match &graph.iter().find(|object| object.id.num == number).unwrap().value {
            PdfObject::Stream { data, filters, dict } => {
                assert_eq!(filters.len(), 1);
                assert!(!dict.iter().any(|entry| entry.key == "Filter" || entry.key == "Length"));
                data.as_slice()
            }
            _ => panic!("fixture object is a stream"),
        };
        assert_eq!(bytes(4), b"0 0 m 1 1 l S\n".repeat(192));
        assert_eq!(bytes(5), [10, 30, 50, 20, 40, 60]);
    }
}
