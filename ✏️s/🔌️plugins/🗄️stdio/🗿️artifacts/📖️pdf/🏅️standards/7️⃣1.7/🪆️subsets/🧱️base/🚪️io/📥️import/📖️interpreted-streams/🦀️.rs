//! 📖️ Decodes in place, in bounded units, exactly the retained streams the semantic lift interprets — page, form, glyph, pattern and appearance programs, font programs and character maps, colour, function and shading data, small images, scripts, attachments and metadata — and leaves every other stream (private application data, structure, unreferenced objects, large sampled images typed as artifacts of their encoded body) as its original encoded bytes with its declared filter chain, so an import costs what its interpreted content costs and Save carries the rest through byte for byte.
//!
//! @see ../../../🔨️modules/🗜️filters/🦀️.rs `declares_logical_codec` — which streams the import keeps encoded.

use crate::standards::v1_7::subsets::base::modules::lexer::{grow_table, retain_stream_entry, table_growth, CosGrant, CosReceipt, CosStatus, PdfCosCursor};
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfStreamBytes, PdfStreamFilter};
use semio_framework_value::ValueRefusalKind;
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::io::foreign_artifacts::encoded_sampled_image;
use semio_s_artifact_stdio_pdf_inferences::standards::v1_7::subsets::base::modules::filters::{declares_logical_codec, DecodedStreamData, PdfDecodedStreamCursor, StreamDataGrant, StreamDataReceipt, StreamDataStatus};
use std::mem::{replace, size_of};

/// 🪜️ The deepest direct nesting the walk follows inside one object.
const MAX_DEPTH: usize = 32;

/// 🧭️ What the lift reads an object as, which decides the objects it leads to and whether its body is decoded.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Role {
    Catalog, Node, Resources, Fonts, XObjects, States, Patterns, Shadings, Spaces,
    Font, Descriptor, Glyphs, CMap, XObject, State, SoftMask, Pattern, Shading, Space, Function,
    Annotation, Appearance, Appearances, Icons, Action, Actions, Outline, Names, Scripts, Files, FileSpec, Attachments,
    Form, Field, Intent, Content, Data,
}

impl Role {
    /// 🗜️ Whether the lift interprets the decoded body of a stream it reads in this role.
    fn decodes(self) -> bool {
        matches!(self, Role::Content | Role::Data | Role::CMap | Role::XObject | Role::Appearances | Role::Pattern | Role::Shading | Role::Function)
    }
}

/// 🫙️ One interpreted stream being decoded in place: its decoder, the decoded body and filter chain, the dictionary without its filter declaration, then the replaced dictionary and encoded body retired after one atomic swap.
struct Decoding {
    position: usize,
    step: DecodeStep,
    decoder: PdfDecodedStreamCursor,
    output: Option<DecodedStreamData>,
    kept: Option<Vec<PdfDictEntry>>,
    retiring: Option<PdfCosCursor>,
    encoded: PdfStreamBytes,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum DecodeStep { Decode, CloseDecoder { decoded: bool }, Sanitize, Swap, RetireDictionary, Retire }

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase { Seed, Walk, Complete, Closing, Closed }

/// 📖️ Walks the document from its catalog the way the semantic lift reads it and decodes each interpreted stream in place, one object or one bounded decode run per unit.
pub struct InterpretedStreamCursor {
    phase: Phase,
    worklist: Vec<(ObjRef, Role)>,
    visited: Vec<(ObjRef, Role)>,
    decoding: Option<Box<Decoding>>,
    maximum_stream_bytes: usize,
    decoded: usize,
    kept: usize,
}

impl InterpretedStreamCursor {
    pub fn new(maximum_stream_bytes: usize) -> Self {
        Self { phase: Phase::Seed, worklist: Vec::new(), visited: Vec::new(), decoding: None, maximum_stream_bytes, decoded: 0, kept: 0 }
    }

    /// 📊️ Streams decoded in place, and interpreted streams left encoded because no bounded decoder accepts their chain.
    pub fn counts(&self) -> (usize, usize) { (self.decoded, self.kept) }

    /// 📄️ Whether the next units only decode body bytes of an interpreted stream (see [`PdfDecodedStreamCursor::decodes_body_bytes`]).
    pub fn decodes_body_bytes(&self) -> bool {
        self.phase == Phase::Walk && self.decoding.as_ref().is_some_and(|decoding| decoding.step == DecodeStep::Decode && decoding.decoder.decodes_body_bytes())
    }

    /// 🎚️ Whether the next unit reserves a fresh decoder history.
    pub fn next_reserves_history(&self) -> bool {
        self.phase == Phase::Walk && self.decoding.as_ref().is_some_and(|decoding| decoding.step == DecodeStep::Decode && decoding.decoder.next_reserves_history())
    }

    /// 🧭️ Names the unit the walk runs next, for the turn census.
    pub fn trace(&self) -> &'static str {
        match (self.phase, self.decoding.as_ref().map(|decoding| decoding.step)) {
            (Phase::Seed, _) => "seed",
            (Phase::Walk, None) => "visit",
            (Phase::Walk, Some(DecodeStep::Decode)) => "decode",
            (Phase::Walk, Some(DecodeStep::CloseDecoder { .. })) => "close-decoder",
            (Phase::Walk, Some(DecodeStep::Sanitize)) => "sanitize",
            (Phase::Walk, Some(DecodeStep::Swap)) => "swap",
            (Phase::Walk, Some(DecodeStep::RetireDictionary)) => "retire-dictionary",
            (Phase::Walk, Some(DecodeStep::Retire)) => "retire",
            _ => "close",
        }
    }

    /// 🧹️ Whether the walk completed or was cancelled and only retirement remains.
    pub fn closing(&self) -> bool { matches!(self.phase, Phase::Closing | Phase::Closed) }

    /// ▶️ Advances one seed, one walked object or one bounded decode run over the stable retained graph.
    pub fn advance(&mut self, objects: &mut [PdfIndirectObject], trailer: &[PdfDictEntry], position: impl Fn(ObjRef) -> Option<usize>, grant: CosGrant) -> CosReceipt {
        if grant.work == 0 { return receipt(Ok(CosStatus::More), 0); }
        match self.phase {
            Phase::Complete => return receipt(Ok(CosStatus::Complete { end: self.decoded }), 0),
            Phase::Closing => return receipt(Ok(CosStatus::Cancelled), 0),
            Phase::Closed => return receipt(Ok(CosStatus::Closed), 0),
            Phase::Seed => {
                let bytes = table_growth(&self.worklist, 1);
                if bytes > grant.allocation_bytes { return receipt(Ok(CosStatus::NeedsAllocation { bytes }), 0); }
                if !grow_table(&mut self.worklist, 1) { return receipt(Err("unable to allocate the interpreted PDF stream walk"), 0); }
                if let Some(root) = trailer.iter().find(|entry| entry.key == "Root").and_then(|entry| entry.value.as_ref()) { self.worklist.push((root, Role::Catalog)); }
                self.phase = Phase::Walk;
                return CosReceipt { allocated_bytes: bytes, ..receipt(Ok(CosStatus::More), 1) };
            }
            Phase::Walk => {}
        }
        if self.decoding.is_some() { return self.decode(objects, grant); }
        let Some((reference, role)) = self.worklist.pop() else {
            self.phase = Phase::Complete;
            return receipt(Ok(CosStatus::Complete { end: self.decoded }), 1);
        };
        let Err(slot) = self.visited.binary_search(&(reference, role)) else { return receipt(Ok(CosStatus::More), 1); };
        let Some(index) = position(reference).filter(|index| *index < objects.len()) else { return receipt(Ok(CosStatus::More), 1); };
        let value = &objects[index].value;
        let decodes = role.decodes() && matches!(value, PdfObject::Stream { dict, filters, .. }
            if filters.is_empty() && declares_logical_codec(dict) && !(role == Role::XObject && encoded_sampled_image(dict)));
        let bytes = table_growth(&self.worklist, references(value, 0)) + table_growth(&self.visited, 1) + if decodes { size_of::<Decoding>() } else { 0 };
        if bytes > grant.allocation_bytes {
            self.worklist.push((reference, role));
            return receipt(Ok(CosStatus::NeedsAllocation { bytes }), 0);
        }
        if !grow_table(&mut self.worklist, references(value, 0)) || !grow_table(&mut self.visited, 1) {
            return receipt(Err("unable to grow the interpreted PDF stream walk"), 0);
        }
        self.visited.insert(slot, (reference, role));
        let worklist = &mut self.worklist;
        visit(role, value, &mut |target, next| worklist.push((target, next)), 0);
        if decodes {
            self.decoding = Some(Box::new(Decoding {
                position: index, step: DecodeStep::Decode, decoder: PdfDecodedStreamCursor::new(self.maximum_stream_bytes, self.maximum_stream_bytes),
                output: None, kept: None, retiring: None, encoded: PdfStreamBytes::default(),
            }));
        }
        CosReceipt { allocated_bytes: bytes, ..receipt(Ok(CosStatus::More), 1) }
    }

    fn decode(&mut self, objects: &mut [PdfIndirectObject], grant: CosGrant) -> CosReceipt {
        let decoding = self.decoding.as_mut().expect("active interpreted PDF stream");
        let step = decoding.step;
        match step {
            DecodeStep::Decode => {
                let PdfObject::Stream { dict, data, .. } = &objects[decoding.position].value else { return receipt(Err("interpreted PDF stream changed shape"), 0); };
                let mut step = decoding.decoder.advance(dict, data, stream_grant(grant), false);
                let refused = step.status.as_ref().is_err_and(|error| matches!(error.kind, ValueRefusalKind::InvalidValue | ValueRefusalKind::UnsupportedOwner));
                match (matches!(step.status, Ok(StreamDataStatus::Complete)), refused) {
                    (true, _) => {
                        decoding.output = decoding.decoder.take_output();
                        if decoding.output.is_none() { return receipt(Err("interpreted PDF stream lost its decoded body"), step.work); }
                        decoding.decoder.begin_close();
                        decoding.step = DecodeStep::CloseDecoder { decoded: true };
                        step.status = Ok(StreamDataStatus::More);
                    }
                    (false, true) => {
                        decoding.decoder.begin_close();
                        decoding.step = DecodeStep::CloseDecoder { decoded: false };
                        step.status = Ok(StreamDataStatus::More);
                    }
                    _ => {}
                }
                stream_receipt(step)
            }
            DecodeStep::CloseDecoder { decoded } => {
                let mut step = decoding.decoder.close_step(stream_grant(grant));
                if matches!(step.status, Ok(StreamDataStatus::Closed)) {
                    step.status = Ok(StreamDataStatus::More);
                    step.work = step.work.max(1);
                    if decoded { decoding.step = DecodeStep::Sanitize; } else { decoding.step = DecodeStep::Retire; self.kept += 1; }
                }
                stream_receipt(step)
            }
            DecodeStep::Sanitize => {
                let PdfObject::Stream { dict, .. } = &objects[decoding.position].value else { return receipt(Err("interpreted PDF stream changed shape"), 0); };
                let retained = || dict.iter().filter(|entry| retain_stream_entry(entry, true));
                let bytes = retained().map(|entry| size_of::<PdfDictEntry>() + entry.key.len() + held_bytes(&entry.value, 0)).sum::<usize>();
                if bytes > grant.allocation_bytes { return receipt(Ok(CosStatus::NeedsAllocation { bytes }), 0); }
                decoding.kept = Some(retained().cloned().collect());
                decoding.step = DecodeStep::Swap;
                CosReceipt { allocated_bytes: bytes, ..receipt(Ok(CosStatus::More), 1) }
            }
            DecodeStep::Swap => {
                let PdfObject::Stream { dict, data, filters } = &mut objects[decoding.position].value else { return receipt(Err("interpreted PDF stream changed shape"), 0); };
                let DecodedStreamData { bytes, filters: chain } = decoding.output.take().expect("decoded interpreted PDF stream");
                decoding.encoded = replace(data, bytes);
                drop(replace(filters, chain));
                decoding.retiring = Some(PdfCosCursor::retire_dictionary(replace(dict, decoding.kept.take().expect("sanitized interpreted PDF stream dictionary"))));
                decoding.step = DecodeStep::RetireDictionary;
                self.decoded += 1;
                receipt(Ok(CosStatus::More), 1)
            }
            DecodeStep::RetireDictionary => {
                let step = decoding.retiring.as_mut().expect("replaced interpreted PDF stream dictionary").close_step(grant);
                if step.status == Ok(CosStatus::Closed) {
                    decoding.retiring = None;
                    decoding.step = DecodeStep::Retire;
                    return CosReceipt { status: Ok(CosStatus::More), work: step.work.max(1), ..step };
                }
                step
            }
            DecodeStep::Retire => self.retire_decoding(grant),
        }
    }

    /// 🧺️ Retires what the stream in flight still owns once its decoder closed: an unswapped decoded body and dictionary, the replaced dictionary, the encoded body, then the record itself.
    fn retire_decoding(&mut self, grant: CosGrant) -> CosReceipt {
        let decoding = self.decoding.as_mut().expect("retiring interpreted PDF stream");
        if let Some(output) = decoding.output.as_mut() {
            if !output.bytes.retained_bytes().terminal_is_empty() { return retire_page(&mut output.bytes, grant); }
            let bytes = output.filters.capacity() * size_of::<PdfStreamFilter>();
            if bytes > grant.release_bytes { return receipt(Ok(CosStatus::NeedsRelease { bytes }), 0); }
            decoding.output = None;
            return CosReceipt { released_bytes: bytes, ..receipt(Ok(CosStatus::More), 1) };
        }
        if let Some(kept) = decoding.kept.take() {
            decoding.retiring = Some(PdfCosCursor::retire_dictionary(kept));
            return receipt(Ok(CosStatus::More), 1);
        }
        if let Some(retiring) = decoding.retiring.as_mut() {
            let step = retiring.close_step(grant);
            if step.status == Ok(CosStatus::Closed) { decoding.retiring = None; return CosReceipt { status: Ok(CosStatus::More), work: step.work.max(1), ..step }; }
            return step;
        }
        if !decoding.encoded.retained_bytes().terminal_is_empty() { return retire_page(&mut decoding.encoded, grant); }
        let bytes = size_of::<Decoding>();
        if bytes > grant.release_bytes { return receipt(Ok(CosStatus::NeedsRelease { bytes }), 0); }
        self.decoding = None;
        CosReceipt { released_bytes: bytes, ..receipt(Ok(CosStatus::More), 1) }
    }

    /// ♻️ Starts retiring the walk after completion, cancellation or refusal; a stream not yet swapped keeps its encoded body, a swapped one keeps its decoded body.
    pub fn begin_close(&mut self) {
        if matches!(self.phase, Phase::Closing | Phase::Closed) { return; }
        if let Some(decoding) = self.decoding.as_mut() {
            let step = decoding.step;
            match step {
                DecodeStep::Decode => { decoding.decoder.begin_close(); decoding.step = DecodeStep::CloseDecoder { decoded: false }; }
                DecodeStep::CloseDecoder { .. } => decoding.step = DecodeStep::CloseDecoder { decoded: false },
                DecodeStep::Sanitize | DecodeStep::Swap => decoding.step = DecodeStep::Retire,
                DecodeStep::RetireDictionary | DecodeStep::Retire => {}
            }
        }
        self.phase = Phase::Closing;
    }

    /// 🧹️ Finishes retiring the stream in flight, then releases the walk's tables.
    pub fn close_step(&mut self, grant: CosGrant) -> CosReceipt {
        if self.phase == Phase::Closed { return receipt(Ok(CosStatus::Closed), 0); }
        if self.phase != Phase::Closing || grant.work == 0 { return receipt(Ok(CosStatus::More), 0); }
        if let Some(decoding) = self.decoding.as_mut() {
            if matches!(decoding.step, DecodeStep::CloseDecoder { .. }) {
                let mut step = decoding.decoder.close_step(stream_grant(grant));
                if matches!(step.status, Ok(StreamDataStatus::Closed)) {
                    step.status = Ok(StreamDataStatus::More);
                    step.work = step.work.max(1);
                    decoding.step = DecodeStep::Retire;
                }
                return stream_receipt(step);
            }
            return self.retire_decoding(grant);
        }
        let bytes = (self.worklist.capacity() + self.visited.capacity()) * size_of::<(ObjRef, Role)>();
        if bytes > grant.release_bytes { return receipt(Ok(CosStatus::NeedsRelease { bytes }), 0); }
        self.worklist = Vec::new();
        self.visited = Vec::new();
        self.phase = Phase::Closed;
        CosReceipt { released_bytes: bytes, ..receipt(Ok(CosStatus::Closed), 1) }
    }
}

/// 📏️ Bounds the heap one direct value owns once cloned.
fn held_bytes(value: &PdfObject, depth: usize) -> usize {
    size_of::<PdfObject>() + if depth > MAX_DEPTH { 0 } else {
        match value {
            PdfObject::Name(text) | PdfObject::Text(text) => text.len(),
            PdfObject::Str(bytes) => bytes.len(),
            PdfObject::Real(decimal) => decimal.coefficient.len(),
            PdfObject::Array(items) => items.iter().map(|item| held_bytes(item, depth + 1)).sum(),
            PdfObject::Dict(entries) => entries.iter().map(|entry| size_of::<PdfDictEntry>() + entry.key.len() + held_bytes(&entry.value, depth + 1)).sum(),
            _ => 0,
        }
    }
}

/// 🧰️ Runs the walk to completion under unbounded reference grants, for the cold reader.
pub fn admit_interpreted_streams(objects: &mut [PdfIndirectObject], trailer: &[PdfDictEntry], position: impl Fn(ObjRef) -> Option<usize>) -> Result<(usize, usize), &'static str> {
    let grant = CosGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX };
    let mut cursor = InterpretedStreamCursor::new(usize::MAX);
    let result = loop {
        match cursor.advance(objects, trailer, &position, grant).status {
            Ok(CosStatus::Complete { .. }) => break Ok(cursor.counts()),
            Ok(CosStatus::More) => {}
            Err(reason) => break Err(reason),
            Ok(_) => break Err("unexpected interpreted PDF stream walk state under reference grants"),
        }
    };
    cursor.begin_close();
    loop {
        match cursor.close_step(grant).status {
            Ok(CosStatus::Closed) => return result,
            Ok(CosStatus::More) => {}
            Err(reason) => return Err(reason),
            Ok(_) => return Err("unexpected interpreted PDF stream retirement state under reference grants"),
        }
    }
}

/// 🔢️ Bounds the references one object can push: every reference value it holds.
fn references(value: &PdfObject, depth: usize) -> usize {
    if depth > MAX_DEPTH { return 0; }
    match value {
        PdfObject::Ref(_) => 1,
        PdfObject::Array(items) => items.iter().map(|item| references(item, depth + 1)).sum(),
        PdfObject::Dict(entries) | PdfObject::Stream { dict: entries, .. } => entries.iter().map(|entry| references(&entry.value, depth + 1)).sum(),
        _ => 0,
    }
}

/// 🪢️ Follows one value read in `role`: a reference is pushed, an array of a listing role is spread, a direct value is visited in place.
fn follow(value: &PdfObject, role: Role, push: &mut dyn FnMut(ObjRef, Role), depth: usize) {
    match value {
        PdfObject::Ref(target) => push(*target, role),
        other => visit(role, other, push, depth + 1),
    }
}

fn key(value: &PdfObject, name: &str, role: Role, push: &mut dyn FnMut(ObjRef, Role), depth: usize) {
    if let Some(found) = value.dict_get(name) { follow(found, role, push, depth); }
}

fn entries(value: &PdfObject, role: Role, push: &mut dyn FnMut(ObjRef, Role), depth: usize) {
    for entry in value.as_dict().unwrap_or(&[]) { follow(&entry.value, role, push, depth); }
}

/// 🌳️ Follows the values of a name tree node (`Kids` again as the tree, every second `Names` item as `leaf`).
fn tree(value: &PdfObject, role: Role, leaf: Role, push: &mut dyn FnMut(ObjRef, Role), depth: usize) {
    key(value, "Kids", role, push, depth);
    if let Some(PdfObject::Array(items)) = value.dict_get("Names") {
        for item in items.iter().skip(1).step_by(2) { follow(item, leaf, push, depth); }
    }
}

/// 🗺️ Pushes the objects the lift reads from one value of `role`, mirroring the reference lifter's reads.
fn visit(role: Role, value: &PdfObject, push: &mut dyn FnMut(ObjRef, Role), depth: usize) {
    if depth > MAX_DEPTH { return; }
    if let PdfObject::Array(items) = value {
        if role != Role::Space {
            for item in items { follow(item, role, push, depth); }
            return;
        }
    }
    macro_rules! at { ($name:literal, $next:expr) => { key(value, $name, $next, push, depth) }; }
    match role {
        Role::Catalog => {
            at!("Pages", Role::Node); at!("Metadata", Role::Data); at!("Names", Role::Names); at!("OpenAction", Role::Action);
            at!("Outlines", Role::Outline); at!("AcroForm", Role::Form); at!("OutputIntents", Role::Intent); at!("AF", Role::FileSpec); at!("AA", Role::Actions);
        }
        Role::Node => {
            at!("Kids", Role::Node); at!("Contents", Role::Content); at!("Resources", Role::Resources); at!("Annots", Role::Annotation);
            at!("Thumb", Role::XObject); at!("Metadata", Role::Data); at!("AF", Role::FileSpec); at!("AA", Role::Actions);
        }
        Role::Resources => {
            at!("Font", Role::Fonts); at!("XObject", Role::XObjects); at!("ExtGState", Role::States);
            at!("Pattern", Role::Patterns); at!("Shading", Role::Shadings); at!("ColorSpace", Role::Spaces);
        }
        Role::Fonts => entries(value, Role::Font, push, depth),
        Role::XObjects => entries(value, Role::XObject, push, depth),
        Role::States => entries(value, Role::State, push, depth),
        Role::Patterns => entries(value, Role::Pattern, push, depth),
        Role::Shadings => entries(value, Role::Shading, push, depth),
        Role::Spaces => entries(value, Role::Space, push, depth),
        Role::Glyphs => entries(value, Role::Content, push, depth),
        Role::Actions => entries(value, Role::Action, push, depth),
        Role::Attachments => entries(value, Role::Data, push, depth),
        Role::Font => {
            at!("FontDescriptor", Role::Descriptor); at!("ToUnicode", Role::Data); at!("Encoding", Role::CMap); at!("DescendantFonts", Role::Font);
            at!("CIDToGIDMap", Role::Data); at!("CharProcs", Role::Glyphs); at!("Resources", Role::Resources);
        }
        Role::Descriptor => { at!("FontFile", Role::Data); at!("FontFile2", Role::Data); at!("FontFile3", Role::Data); at!("CIDSet", Role::Data); }
        Role::CMap => at!("UseCMap", Role::CMap),
        Role::Appearances if !matches!(value, PdfObject::Stream { .. }) => entries(value, Role::XObject, push, depth),
        Role::XObject | Role::Appearances => {
            at!("Resources", Role::Resources); at!("SMask", Role::XObject); at!("Mask", Role::XObject); at!("ColorSpace", Role::Space); at!("AF", Role::FileSpec);
        }
        Role::State => {
            at!("SMask", Role::SoftMask); at!("Font", Role::Font);
            for name in ["TR", "TR2", "BG", "BG2", "UCR", "UCR2"] { key(value, name, Role::Function, push, depth); }
        }
        Role::SoftMask => { at!("G", Role::XObject); at!("TR", Role::Function); }
        Role::Pattern => { at!("Resources", Role::Resources); at!("Shading", Role::Shading); at!("ExtGState", Role::State); }
        Role::Shading => { at!("Function", Role::Function); at!("ColorSpace", Role::Space); }
        Role::Space => match value {
            PdfObject::Array(items) => {
                macro_rules! item { ($index:literal, $next:expr) => { if let Some(found) = items.get($index) { follow(found, $next, push, depth); } }; }
                match items.first().and_then(PdfObject::as_name) {
                    Some("ICCBased") => item!(1, Role::Data),
                    Some("Indexed" | "I") => { item!(1, Role::Space); item!(3, Role::Data); }
                    Some("Separation") => { item!(2, Role::Space); item!(3, Role::Function); }
                    Some("DeviceN") => { item!(2, Role::Space); item!(3, Role::Function); item!(4, Role::Space); }
                    Some("Pattern") => item!(1, Role::Space),
                    _ => {}
                }
            }
            PdfObject::Dict(_) => { at!("Colorants", Role::Spaces); at!("Process", Role::Space); at!("ColorSpace", Role::Space); }
            _ => {}
        },
        Role::Function => at!("Functions", Role::Function),
        Role::Annotation => {
            at!("AP", Role::Appearance); at!("A", Role::Action); at!("AA", Role::Actions); at!("FS", Role::FileSpec); at!("MK", Role::Icons);
        }
        Role::Appearance => { at!("N", Role::Appearances); at!("R", Role::Appearances); at!("D", Role::Appearances); }
        Role::Icons => { at!("I", Role::XObject); at!("RI", Role::XObject); at!("IX", Role::XObject); }
        Role::Action => { at!("JS", Role::Data); at!("Next", Role::Action); }
        Role::Outline => { at!("First", Role::Outline); at!("Next", Role::Outline); at!("A", Role::Action); }
        Role::Names => { at!("JavaScript", Role::Scripts); at!("EmbeddedFiles", Role::Files); }
        Role::Scripts => tree(value, Role::Scripts, Role::Action, push, depth),
        Role::Files => tree(value, Role::Files, Role::FileSpec, push, depth),
        Role::FileSpec => at!("EF", Role::Attachments),
        Role::Form => { at!("Fields", Role::Field); at!("DR", Role::Resources); }
        Role::Field => {
            at!("Kids", Role::Field); at!("AP", Role::Appearance); at!("A", Role::Action); at!("AA", Role::Actions);
            at!("V", Role::Data); at!("DV", Role::Data); at!("RV", Role::Data); at!("MK", Role::Icons); at!("DR", Role::Resources);
        }
        Role::Intent => at!("DestOutputProfile", Role::Data),
        Role::Data => at!("Alternate", Role::Space),
        Role::Content => {}
    }
}

fn stream_grant(grant: CosGrant) -> StreamDataGrant {
    StreamDataGrant { work: grant.work, allocation_bytes: grant.allocation_bytes, release_bytes: grant.release_bytes }
}

fn stream_receipt(step: StreamDataReceipt) -> CosReceipt {
    let status = match step.status {
        Ok(StreamDataStatus::NeedsAllocation { bytes }) => Ok(CosStatus::NeedsAllocation { bytes }),
        Ok(StreamDataStatus::NeedsRelease { bytes }) => Ok(CosStatus::NeedsRelease { bytes }),
        Ok(_) => Ok(CosStatus::More),
        Err(_) => Err("interpreted PDF stream decoder refused its grant"),
    };
    CosReceipt { status, work: step.work, allocated_bytes: step.allocated_bytes, released_bytes: step.released_bytes }
}

fn retire_page(bytes: &mut PdfStreamBytes, grant: CosGrant) -> CosReceipt {
    match bytes.cold_retire_page(grant.release_bytes) {
        Err(_) => receipt(Err("unable to retire an interpreted PDF stream page"), 0),
        Ok(step) => match step.needs_release {
            Some(bytes) => receipt(Ok(CosStatus::NeedsRelease { bytes }), 0),
            None => CosReceipt { status: Ok(CosStatus::More), work: step.work.max(1), allocated_bytes: 0, released_bytes: step.released_bytes },
        },
    }
}

fn receipt(status: Result<CosStatus, &'static str>, work: usize) -> CosReceipt {
    CosReceipt { status, work, allocated_bytes: 0, released_bytes: 0 }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
