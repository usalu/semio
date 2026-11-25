//! 🔤️ Admits one borrowed embedded font program and its direct Unicode maps.

use super::cmap::PdfDirectToUnicodeCursor;
use crate::standards::v1_7::subsets::base::io::foreign_artifacts::{artifact_reference, artifact_reference_bytes};
use crate::standards::v1_7::subsets::base::modules::lexer::{CosGrant, CosReceipt, CosStatus, grow_table, table_growth};
use crate::standards::v1_7::subsets::base::schema::snapshot::{
    ObjRef, PdfBaseEncoding, PdfCMap, PdfCidFont, PdfCidToGid, PdfCidWidthRun, PdfDictEntry, PdfFont, PdfFontDescriptor, PdfFontKind, PdfFontProgram, PdfObject,
    PdfEncodingDifference, PdfSimpleEncoding, PdfSnapshot, PdfStreamFilter, PdfToUnicode, PdfToUnicodeMapping,
};
use crate::standards::v1_7::subsets::base::schema::stream_roles::{PdfAdmittedStreamRole, PdfGraphIdentity, PdfStreamRoleValue};
use semio_framework_hash::Sha256;
use semio_framework_value::ByteReadSource;
use std::mem::size_of;

const NAME: usize = 128;
const OPENTYPE: &str = "s.stdio.font.opentype";
const TRUETYPE: &str = "s.stdio.font.truetype";
const MAPPINGS: usize = 256;

type Inspect<'a> = dyn FnMut(ObjRef, &mut dyn FnMut(Option<&PdfObject>)) + 'a;

struct ShortName { bytes: [u8; NAME], length: usize }

impl ShortName {
    fn new(text: &str) -> Option<Self> {
        if text.is_empty() || text.len() > NAME { return None; }
        let mut bytes = [0; NAME];
        bytes[..text.len()].copy_from_slice(text.as_bytes());
        Some(Self { bytes, length: text.len() })
    }

    fn text(&self) -> &str { std::str::from_utf8(&self.bytes[..self.length]).expect("admitted PDF font name") }
}

#[derive(Clone, Copy)]
enum Phase { Descendant, Descriptor, Widths, Program, Unicode(usize), CloseUnicode(usize), Ready, Closing, Closed }

/// 🧾️ Transfers an embedded font together with its physical program and Unicode-map provenance.
pub struct AdmittedEmbeddedFont {
    pub font: PdfFont,
    pub roles: [PdfAdmittedStreamRole; 2],
    dependencies: FontDependencies,
}

impl AdmittedEmbeddedFont {
    /// 🔗 Borrows every graph owner that the typed font depended on.
    pub fn dependencies(&self) -> &[ObjRef] { self.dependencies.owners() }
}

/// 🔏️ Retains only bounded font metadata, an incremental digest and two independent map outputs.
pub struct PdfEmbeddedFontCursor {
    owner: ObjRef,
    descendant: Option<Composite>,
    descriptor_owner: Option<ObjRef>,
    unicode_owner: ObjRef,
    program_owner: Option<ObjRef>,
    font: ShortName,
    first: u32,
    count: usize,
    descriptor: PdfFontDescriptor,
    widths: Vec<f64>,
    hash: Sha256,
    offset: usize,
    filter_wire: &'static [u8],
    unicode: PdfDirectToUnicodeCursor,
    maps: [Option<PdfToUnicode>; 2],
    phase: Phase,
    deferred: bool,
}

impl PdfEmbeddedFontCursor {
    /// 🔍️ Qualifies a horizontal font dictionary without copying its arrays.
    pub fn new(owner: ObjRef, source: &PdfObject) -> Option<Self> {
        let (dict, descendant, descriptor_owner, first, count) = match source.dict_get("Subtype")?.as_name()? {
            "Type1" => {
                let dict = qualified(source, &["Type", "Subtype", "BaseFont", "Encoding", "FirstChar", "LastChar", "Widths", "FontDescriptor", "ToUnicode"])?;
                if name(dict, "Encoding")? != "WinAnsiEncoding" { return None; }
                let first = u8::try_from(integer(dict, "FirstChar")?).ok()?;
                let last = u8::try_from(integer(dict, "LastChar")?).ok()?;
                let count = usize::from(last.checked_sub(first)?) + 1;
                if source.dict_get("Widths")?.as_array()?.len() != count { return None; }
                (dict, None, Some(reference(dict, "FontDescriptor")?), u32::from(first), count)
            }
            "Type0" => {
                let dict = qualified(source, &["Type", "Subtype", "BaseFont", "Encoding", "DescendantFonts", "ToUnicode"])?;
                if name(dict, "Encoding")? != "Identity-H" { return None; }
                let [child] = get(dict, "DescendantFonts")?.as_array()? else { return None; };
                (dict, Some(Composite { owner: child.as_ref()?, default_width: 1000., identity: false }), None, 0, 0)
            }
            _ => return None,
        };
        if name(dict, "Type")? != "Font" { return None; }
        let mut hash = Sha256::new();
        hash.update(if descendant.is_some() { TRUETYPE } else { OPENTYPE }.as_bytes());
        Some(Self {
            owner, descendant, descriptor_owner, unicode_owner: reference(dict, "ToUnicode")?,
            font: ShortName::new(name(dict, "BaseFont")?)?, first, count, descriptor: Default::default(),
            program_owner: None, widths: Vec::new(), hash, offset: 0, filter_wire: b"[]", unicode: PdfDirectToUnicodeCursor::new(MAPPINGS),
            maps: [None, None], phase: if descendant.is_some() { Phase::Descendant } else { Phase::Descriptor }, deferred: false,
        })
    }

    /// 🎚️ Reports the unpublished metadata copy reserve for one bounded turn.
    pub fn inline_body_bytes(&self) -> usize {
        match self.phase {
            Phase::Descriptor => size_of::<PdfFontDescriptor>(),
            Phase::CloseUnicode(0) => size_of::<PdfDirectToUnicodeCursor>(),
            Phase::Ready => size_of::<PdfFont>() + 2 * size_of::<PdfAdmittedStreamRole>() + size_of::<FontDependencies>(),
            _ => 128,
        }
    }

    /// ⏭️ Indicates that the reference importer must own this resource.
    pub fn reference_required(&self) -> bool { self.deferred }

    /// ▶️ Performs one structural, width, digest or Unicode turn over borrowed objects.
    pub fn advance(&mut self, inspect: &mut Inspect<'_>, mut grant: CosGrant) -> CosReceipt {
        if self.deferred { return receipt(Err("embedded font resource needs reference admission"), 0); }
        if grant.work == 0 { return receipt(Ok(CosStatus::More), 0); }
        grant.work = 1;
        match self.phase {
            Phase::Descendant => {
                let child = self.descendant.as_mut().expect("pending CIDFont descendant");
                let mut shape = None;
                inspect(child.owner, &mut |value| shape = value.and_then(|value| cid_font(value, self.font.text())));
                let Some((descriptor, default_width, identity, first, count)) = shape else { return self.defer(); };
                child.default_width = default_width;
                child.identity = identity;
                self.descriptor_owner = Some(descriptor);
                self.first = first;
                self.count = count;
                self.phase = Phase::Descriptor;
                more()
            }
            Phase::Descriptor => {
                let mut shape = None;
                let program_key = if self.descendant.is_some() { "FontFile2" } else { "FontFile3" };
                inspect(self.descriptor_owner.expect("qualified descriptor owner"), &mut |value| {
                    shape = value.and_then(|value| descriptor(value, self.font.text(), program_key));
                });
                let Some((descriptor, program)) = shape else { return self.defer(); };
                self.descriptor = descriptor;
                self.program_owner = Some(program);
                self.phase = if self.count == 0 { Phase::Program } else { Phase::Widths };
                more()
            }
            Phase::Widths if self.widths.capacity() == 0 => {
                let bytes = self.count * size_of::<f64>();
                if bytes > grant.allocation_bytes { return receipt(Ok(CosStatus::NeedsAllocation { bytes }), 0); }
                if self.widths.try_reserve_exact(self.count).is_err() { return receipt(Err("unable to allocate PDF font widths"), 1); }
                CosReceipt { allocated_bytes: self.widths.capacity() * size_of::<f64>(), ..more() }
            }
            Phase::Widths => {
                let mut next = None;
                if let Some(child) = self.descendant {
                    inspect(child.owner, &mut |value| {
                        next = value.and_then(|font| font.dict_get("W")?.as_array()?.get(1)?.as_array()?.get(self.widths.len())?.as_f64())
                            .filter(|value| value.is_finite() && *value >= 0.);
                    });
                } else {
                    inspect(self.owner, &mut |value| {
                        next = value.and_then(|font| font.dict_get("Widths")?.as_array()?.get(self.widths.len())?.as_i64()).map(|value| value as f64);
                    });
                }
                let Some(width) = next else { return self.defer(); };
                self.widths.push(width);
                if self.widths.len() == self.count { self.phase = Phase::Program; }
                more()
            }
            Phase::Program => {
                let mut qualified = false;
                inspect(self.program_owner.expect("qualified font descriptor"), &mut |value| {
                    let Some(PdfObject::Stream { dict, data, filters }) = value else { return; };
                    if dict.len() != 1 { return; }
                    let prefix = |expected: &[u8]| expected.iter().enumerate().all(|(index, byte)| data.byte_at(index) == Some(*byte));
                    let valid = if self.descendant.is_some() {
                        prefix(&[0, 1, 0, 0]) && integer(dict, "Length1") == Some(data.len() as i64)
                    } else {
                        name(dict, "Subtype") == Some("OpenType") && prefix(b"OTTO")
                    };
                    if !valid { return; }
                    let Some(wire) = filter_wire(filters) else { return; };
                    if self.offset > data.len() { return; }
                    let end = (self.offset + 1).min(data.len());
                    if let Some(byte) = data.byte_at(self.offset).filter(|_| end > self.offset) { self.hash.update(&[byte]); }
                    self.offset = end;
                    self.filter_wire = wire;
                    qualified = true;
                    if end == data.len() { self.phase = Phase::Unicode(0); }
                });
                if qualified { more() } else { self.defer() }
            }
            Phase::Unicode(index) => {
                let mut turn = None;
                inspect(self.unicode_owner, &mut |value| {
                    let Some(PdfObject::Stream { dict, data, filters }) = value else { return; };
                    if !dict.is_empty() || filter_wire(filters).is_none() { return; }
                    turn = Some(self.unicode.advance(data, grant, false));
                });
                let Some(mut turn) = turn else { return self.defer(); };
                if matches!(turn.status, Ok(CosStatus::Complete { .. })) {
                    let mapping = self.unicode.take_mapping().expect("complete font Unicode map");
                    let width = mapping.byte_width;
                    self.maps[index] = Some(mapping);
                    let code_width = if self.descendant.is_some() { 2 } else { 1 };
                    if width != code_width { return self.defer(); }
                    self.phase = Phase::CloseUnicode(index);
                    turn.status = Ok(CosStatus::More);
                } else if self.unicode.reference_required() {
                    self.deferred = true;
                }
                turn
            }
            Phase::CloseUnicode(index) => {
                let mut turn = self.unicode.close_step(grant);
                if turn.status == Ok(CosStatus::Closed) {
                    if index == 0 {
                        self.unicode = PdfDirectToUnicodeCursor::new(MAPPINGS);
                        self.phase = Phase::Unicode(1);
                    } else {
                        self.phase = Phase::Ready;
                    }
                    turn.status = Ok(CosStatus::More);
                    turn.work = turn.work.max(1);
                }
                turn
            }
            Phase::Ready => receipt(Ok(CosStatus::Complete { end: self.offset }), 0),
            Phase::Closing | Phase::Closed => self.close_step(grant),
        }
    }

    fn defer(&mut self) -> CosReceipt {
        self.deferred = true;
        receipt(Err("embedded font resource needs reference admission"), 1)
    }

    /// 📏️ Quotes all separately allocated identity strings and dependency slots.
    pub fn publication_bytes(&self, name: &str) -> usize {
        let shared = name.len() + 2 * self.font.length + 2 * size_of::<PdfGraphIdentity>();
        let resource = if self.descendant.is_some() {
            self.font.length + "Identity-HAdobeIdentity".len() + 2 * artifact_reference_bytes(TRUETYPE)
                + if self.count == 0 { 0 } else { size_of::<PdfCidWidthRun>() }
        } else {
            2 * artifact_reference_bytes(OPENTYPE)
        };
        shared + resource
    }

    /// 📦️ Moves both admitted Unicode maps and creates the budgeted foreign resource references.
    pub fn take(&mut self, name: &str) -> Option<AdmittedEmbeddedFont> {
        if !matches!(self.phase, Phase::Ready) { return None; }
        let mut digest = self.hash.clone();
        digest.update(self.filter_wire);
        let digest = digest.finalize();
        let [primary, role] = std::mem::take(&mut self.maps);
        let base_font = self.font.text();
        self.descriptor.font_name = base_font.to_owned();
        let descriptor = std::mem::take(&mut self.descriptor);
        let widths = std::mem::take(&mut self.widths);
        let (kind, resource) = if let Some(child) = self.descendant {
            let program = PdfFontProgram::TrueType { reference: artifact_reference(TRUETYPE, digest) };
            let descendant = PdfCidFont {
                true_type: true, base_font: base_font.to_owned(), system_info: Default::default(), descriptor,
                default_width: child.default_width,
                widths: if widths.is_empty() { Vec::new() } else { vec![PdfCidWidthRun { start_cid: self.first, widths }] },
                default_vertical: None, vertical_metrics: Vec::new(),
                cid_to_gid: child.identity.then_some(PdfCidToGid::Identity), program: Some(program.clone()), extra: Vec::new(),
            };
            (PdfFontKind::Type0 { base_font: base_font.to_owned(), cmap: PdfCMap::identity_h(), descendant }, program)
        } else {
            let program = PdfFontProgram::OpenType { reference: artifact_reference(OPENTYPE, digest) };
            (PdfFontKind::Type1 {
                base_font: base_font.to_owned(),
                encoding: PdfSimpleEncoding { base: Some(PdfBaseEncoding::WinAnsi), differences: Vec::new() },
                first_char: self.first, widths, descriptor: Some(descriptor), program: Some(program.clone()),
            }, program)
        };
        let font = PdfFont { id: name.to_owned(), kind, to_unicode: Some(primary.expect("primary Unicode result")), extra: Vec::new() };
        let roles = [
            graph_role(self.unicode_owner, PdfStreamRoleValue::UnicodeMap { mapping: role.expect("provenance Unicode result") }),
            graph_role(self.program_owner.expect("qualified native font stream"), PdfStreamRoleValue::FontProgram { program: resource }),
        ];
        self.phase = Phase::Closed;
        let descriptor = self.descriptor_owner.expect("qualified descriptor owner");
        let child = self.descendant.map(|font| font.owner);
        let dependencies = FontDependencies {
            owners: [descriptor, self.unicode_owner, self.program_owner.unwrap(), child.unwrap_or(descriptor)],
            count: 3 + usize::from(child.is_some()),
        };
        Some(AdmittedEmbeddedFont { font, roles, dependencies })
    }

    /// ♻️ Parks all unpublished font resources for separate release credit.
    pub fn begin_close(&mut self) {
        if !matches!(self.phase, Phase::Closed | Phase::Closing) {
            self.unicode.begin_close();
            self.phase = Phase::Closing;
        }
    }

    /// 🧹️ Retires the active map parser, captured result maps and widths under measured grants.
    pub fn close_step(&mut self, grant: CosGrant) -> CosReceipt {
        if matches!(self.phase, Phase::Closed) { return receipt(Ok(CosStatus::Closed), 0); }
        if grant.work == 0 || !matches!(self.phase, Phase::Closing) { return receipt(Ok(CosStatus::More), 0); }
        let mut turn = self.unicode.close_step(grant);
        if turn.status != Ok(CosStatus::Closed) { return turn; }
        if turn.work != 0 { turn.status = Ok(CosStatus::More); return turn; }
        if let Some(slot) = self.maps.iter_mut().find(|slot| slot.is_some()) {
            let mappings = &mut slot.as_mut().expect("unpublished native map").mappings;
            if let Some(mapping) = mappings.last() {
                let released_bytes = match mapping {
                    PdfToUnicodeMapping::Char { text, .. } | PdfToUnicodeMapping::Range { text, .. } => text.capacity(),
                };
                if released_bytes > grant.release_bytes { return receipt(Ok(CosStatus::NeedsRelease { bytes: released_bytes }), 0); }
                mappings.pop();
                return CosReceipt { released_bytes, ..more() };
            }
            let released_bytes = mappings.capacity() * size_of::<PdfToUnicodeMapping>();
            if released_bytes > grant.release_bytes { return receipt(Ok(CosStatus::NeedsRelease { bytes: released_bytes }), 0); }
            *slot = None;
            return CosReceipt { released_bytes, ..more() };
        }
        let released_bytes = self.widths.capacity() * size_of::<f64>();
        if released_bytes > grant.release_bytes { return receipt(Ok(CosStatus::NeedsRelease { bytes: released_bytes }), 0); }
        self.widths = Vec::new();
        self.phase = Phase::Closed;
        CosReceipt { released_bytes, ..receipt(Ok(CosStatus::Closed), 1) }
    }
}

#[derive(Clone, Copy)]
struct Composite { owner: ObjRef, default_width: f64, identity: bool }

struct FontDependencies { owners: [ObjRef; 4], count: usize }

impl FontDependencies {
    fn owners(&self) -> &[ObjRef] { &self.owners[..self.count] }
}

fn string_bytes(value: &PdfObject) -> Option<&[u8]> {
    match value {
        PdfObject::Text(text) => Some(text.as_bytes()),
        _ => value.as_str_bytes(),
    }
}

fn cid_font(value: &PdfObject, base_font: &str) -> Option<(ObjRef, f64, bool, u32, usize)> {
    let dict = value.as_dict()?;
    let keys = ["Type", "Subtype", "BaseFont", "CIDSystemInfo", "FontDescriptor", "CIDToGIDMap", "W", "DW"];
    if dict.len() > keys.len() || dict.iter().enumerate().any(|(index, entry)| {
        !keys.contains(&entry.key.as_str()) || dict[..index].iter().any(|before| before.key == entry.key)
    }) { return None; }
    if name(dict, "Type")? != "Font" || name(dict, "Subtype")? != "CIDFontType2" || name(dict, "BaseFont")? != base_font { return None; }
    let info = qualified(get(dict, "CIDSystemInfo")?, &["Registry", "Ordering", "Supplement"])?;
    if string_bytes(get(info, "Registry")?)? != b"Adobe" || string_bytes(get(info, "Ordering")?)? != b"Identity"
        || integer(info, "Supplement")? != 0 { return None; }
    let descriptor = reference(dict, "FontDescriptor")?;
    let identity = match get(dict, "CIDToGIDMap") {
        None => false,
        Some(owner) if owner.as_name() == Some("Identity") => true,
        _ => return None,
    };
    let default_width = match get(dict, "DW") {
        Some(value) => value.as_f64().filter(|value| value.is_finite() && *value >= 0.)?,
        None => 1000.,
    };
    let Some(widths) = get(dict, "W") else { return Some((descriptor, default_width, identity, 0, 0)); };
    let [first, array] = widths.as_array()? else { return None; };
    let first = u16::try_from(first.as_i64()?).ok()?;
    let count = array.as_array()?.len();
    if count == 0 || count > MAPPINGS || usize::from(first) + count > u16::MAX as usize + 1 { return None; }
    Some((descriptor, default_width, identity, u32::from(first), count))
}

fn filter_wire(filters: &[PdfStreamFilter]) -> Option<&'static [u8]> {
    match filters {
        [] => Some(b"[]"),
        [PdfStreamFilter::Flate { predictor: None }] => Some(br#"[{"kind":"flate","predictor":null}]"#),
        _ => None,
    }
}

fn descriptor(value: &PdfObject, base_font: &str, program: &str) -> Option<(PdfFontDescriptor, ObjRef)> {
    let dict = qualified(value, &["Type", "FontName", "Flags", "FontBBox", "ItalicAngle", "Ascent", "Descent", "CapHeight", "StemV", program])?;
    if name(dict, "Type")? != "FontDescriptor" || name(dict, "FontName")? != base_font { return None; }
    let flags = u32::try_from(integer(dict, "Flags")?).ok()?;
    if program == "FontFile3" && (flags & 4 != 0 || flags & 32 == 0) { return None; }
    let [a, b, c, d] = value.dict_get("FontBBox")?.as_array()? else { return None; };
    let font_bbox = [a.as_i64()? as f64, b.as_i64()? as f64, c.as_i64()? as f64, d.as_i64()? as f64];
    Some((PdfFontDescriptor {
        flags, font_bbox, italic_angle: integer(dict, "ItalicAngle")? as f64, ascent: integer(dict, "Ascent")? as f64,
        descent: integer(dict, "Descent")? as f64, cap_height: integer(dict, "CapHeight")? as f64,
        stem_v: integer(dict, "StemV")? as f64, ..Default::default()
    }, reference(dict, program)?))
}

fn qualified<'a>(value: &'a PdfObject, keys: &[&str]) -> Option<&'a [PdfDictEntry]> {
    let PdfObject::Dict(dict) = value else { return None; };
    (dict.len() == keys.len() && keys.iter().all(|key| dict.iter().filter(|entry| entry.key == *key).count() == 1)).then_some(dict)
}

fn name<'a>(dict: &'a [PdfDictEntry], key: &str) -> Option<&'a str> { get(dict, key)?.as_name() }
fn reference(dict: &[PdfDictEntry], key: &str) -> Option<ObjRef> { get(dict, key)?.as_ref() }
fn integer(dict: &[PdfDictEntry], key: &str) -> Option<i64> { get(dict, key)?.as_i64() }
fn get<'a>(dict: &'a [PdfDictEntry], key: &str) -> Option<&'a PdfObject> {
    dict.iter().find(|entry| entry.key == key).map(|entry| &entry.value)
}

fn graph_role(owner: ObjRef, value: PdfStreamRoleValue) -> PdfAdmittedStreamRole {
    let identity = PdfGraphIdentity { owner, path: Vec::new() };
    PdfAdmittedStreamRole { dependencies: vec![identity.clone()], identity, value }
}

fn more() -> CosReceipt { receipt(Ok(CosStatus::More), 1) }
fn receipt(status: Result<CosStatus, &'static str>, work: usize) -> CosReceipt {
    CosReceipt { status, work, allocated_bytes: 0, released_bytes: 0 }
}

/// 🪟️ Coordinates the document's font owners (one slot per typed font) and owns an active embedded-font cursor.
#[derive(Default)]
pub struct ControlledFontResources {
    pub owners: Vec<ObjRef>,
    pending: Option<Box<PdfEmbeddedFontCursor>>,
}

impl ControlledFontResources {
    pub const MAX_COUNT: usize = 32;
    pub const ROLES_PER_FONT: usize = 2;

    /// 📏️ Reports the inline transfer reserve for the next font turn.
    pub fn inline_body_bytes(&self) -> usize {
        self.pending.as_ref().map_or(size_of::<PdfFont>(), |owner| owner.inline_body_bytes())
    }

    /// 🔎️ The slot an owner was admitted under, when the document already typed it.
    pub fn slot_of(&self, owner: ObjRef) -> Option<usize> { self.owners.iter().position(|reference| *reference == owner) }

    /// 📏️ Quotes the owner-table growth for one more typed font.
    pub fn owner_growth(&self) -> usize { table_growth(&self.owners, 1) }

    /// 🪢️ Binds a font typed outside this table (the reference font builder) to the next slot after [`Self::owner_growth`] was granted.
    pub fn bind(&mut self, owner: ObjRef) -> Option<usize> {
        if !grow_table(&mut self.owners, 1) { return None; }
        self.owners.push(owner);
        Some(self.owners.len() - 1)
    }

    /// 📦️ Bytes of the owner table this cursor still holds.
    pub fn held_bytes(&self) -> usize { self.owners.capacity() * size_of::<ObjRef>() }

    /// 🧺️ Releases the owner table once its [`Self::held_bytes`] were credited.
    pub fn release(&mut self) { self.owners = Vec::new(); }

    /// 🧾️ Admits a new font or returns the established slot for a shared authored name.
    pub fn admit(&mut self, inspect: &mut Inspect<'_>, snapshot: &mut PdfSnapshot, owner: ObjRef, name: &str, grant: CosGrant) -> Option<CosReceipt> {
        if grant.work == 0 { return Some(receipt(Ok(CosStatus::More), 0)); }
        let slot = snapshot.fonts.len();
        if let Some(cursor) = self.pending.as_mut() {
            let turn = cursor.advance(inspect, grant);
            if cursor.reference_required() { return None; }
            if !matches!(turn.status, Ok(CosStatus::Complete { .. })) { return Some(turn); }
            let allocated_bytes = cursor.publication_bytes(name) + table_growth(&snapshot.fonts, 1) + table_growth(&snapshot.admitted_stream_roles, Self::ROLES_PER_FONT) + table_growth(&self.owners, 1);
            let released_bytes = size_of::<PdfEmbeddedFontCursor>();
            if allocated_bytes > grant.allocation_bytes { return Some(receipt(Ok(CosStatus::NeedsAllocation { bytes: allocated_bytes }), 0)); }
            if released_bytes > grant.release_bytes { return Some(receipt(Ok(CosStatus::NeedsRelease { bytes: released_bytes }), 0)); }
            if !grow_table(&mut snapshot.fonts, 1) || !grow_table(&mut snapshot.admitted_stream_roles, Self::ROLES_PER_FONT) || !grow_table(&mut self.owners, 1) { return Some(receipt(Err("unable to grow the PDF font tables"), 0)); }
            let AdmittedEmbeddedFont { font, roles, .. } = cursor.take(name).expect("published embedded font resource");
            self.pending = None;
            snapshot.fonts.push(font);
            snapshot.admitted_stream_roles.extend(roles);
            self.owners.push(owner);
            return Some(CosReceipt { allocated_bytes, released_bytes, ..receipt(Ok(CosStatus::Complete { end: slot }), 1) });
        }
        if let Some(slot) = self.slot_of(owner) {
            return (snapshot.fonts[slot].id == name).then(|| receipt(Ok(CosStatus::Complete { end: slot }), 1));
        }
        if snapshot.fonts.iter().any(|font| font.id == name) { return None; }
        let mut standard = None;
        let mut embedded = None;
        inspect(owner, &mut |value| {
            standard = standard_type1(value);
            if standard.is_none() {
                embedded = value.and_then(|value| PdfEmbeddedFontCursor::new(owner, value));
            }
        });
        if let Some(StandardType1 { base_font, base, differences, glyph_bytes }) = standard {
            let allocated_bytes = name.len() + base_font.len() + table_growth(&snapshot.fonts, 1) + table_growth(&self.owners, 1) + differences * size_of::<PdfEncodingDifference>() + glyph_bytes;
            if allocated_bytes > grant.allocation_bytes { return Some(receipt(Ok(CosStatus::NeedsAllocation { bytes: allocated_bytes }), 0)); }
            let mut authored = Vec::new();
            if differences != 0 { inspect(owner, &mut |value| { authored = value.map_or_else(Vec::new, |value| encoding_differences(value, differences)); }); }
            if authored.len() != differences { return None; }
            if !grow_table(&mut snapshot.fonts, 1) || !grow_table(&mut self.owners, 1) { return Some(receipt(Err("unable to grow the PDF font table"), 0)); }
            snapshot.fonts.push(PdfFont {
                id: name.to_owned(),
                kind: PdfFontKind::Type1 {
                    base_font: base_font.to_owned(), encoding: PdfSimpleEncoding { base, differences: authored },
                    first_char: 0, widths: Vec::new(), descriptor: None, program: None,
                },
                to_unicode: None, extra: Vec::new(),
            });
            self.owners.push(owner);
            return Some(CosReceipt { allocated_bytes, ..receipt(Ok(CosStatus::Complete { end: slot }), 1) });
        }
        let cursor = embedded?;
        let allocated_bytes = size_of::<PdfEmbeddedFontCursor>();
        if allocated_bytes > grant.allocation_bytes { return Some(receipt(Ok(CosStatus::NeedsAllocation { bytes: allocated_bytes }), 0)); }
        self.pending = Some(Box::new(cursor));
        Some(CosReceipt { allocated_bytes, ..more() })
    }

    /// ♻️ Retains an unfinished font's allocations for controlled cancellation.
    pub fn begin_close(&mut self) {
        if let Some(cursor) = self.pending.as_mut() { cursor.begin_close(); }
    }

    /// 🧹️ Retires an unpublished font, its outer cursor allocation, then the owner table.
    pub fn close_step(&mut self, grant: CosGrant) -> CosReceipt {
        if grant.work == 0 { return receipt(Ok(CosStatus::More), 0); }
        if let Some(turn) = self.retire_pending(grant) { return turn; }
        let released_bytes = self.held_bytes();
        if released_bytes == 0 { return receipt(Ok(CosStatus::Closed), 0); }
        if released_bytes > grant.release_bytes { return receipt(Ok(CosStatus::NeedsRelease { bytes: released_bytes }), 0); }
        self.release();
        CosReceipt { released_bytes, ..receipt(Ok(CosStatus::More), 1) }
    }

    /// ♻️ Retires an embedded-font cursor that stopped (refused or cancelled) one unit at a time; `None` once no cursor is pending.
    pub fn retire_pending(&mut self, grant: CosGrant) -> Option<CosReceipt> {
        let cursor = self.pending.as_mut()?;
        cursor.begin_close();
        let mut turn = cursor.close_step(grant);
        if turn.status != Ok(CosStatus::Closed) { return Some(turn); }
        if turn.work != 0 { turn.status = Ok(CosStatus::More); return Some(turn); }
        let released_bytes = size_of::<PdfEmbeddedFontCursor>();
        if released_bytes > grant.release_bytes { return Some(receipt(Ok(CosStatus::NeedsRelease { bytes: released_bytes }), 0)); }
        self.pending = None;
        Some(CosReceipt { released_bytes, ..receipt(Ok(CosStatus::More), 1) })
    }
}

/// 🔠️ The qualified standard Type1 font dictionary: its encoding and the exact size of its `/Differences`.
struct StandardType1 { base_font: &'static str, base: Option<PdfBaseEncoding>, differences: usize, glyph_bytes: usize }

const MAX_DIFFERENCE_ITEMS: usize = 192;
const MAX_DIFFERENCES: usize = 128;
const MAX_GLYPH_BYTES: usize = 64;

fn simple_base(name: &str) -> Option<PdfBaseEncoding> {
    match name {
        "StandardEncoding" => Some(PdfBaseEncoding::Standard),
        "WinAnsiEncoding" => Some(PdfBaseEncoding::WinAnsi),
        "MacRomanEncoding" => Some(PdfBaseEncoding::MacRoman),
        _ => None,
    }
}

fn encoding_dictionary(entries: &[PdfDictEntry]) -> Option<(Option<PdfBaseEncoding>, usize, usize)> {
    if entries.len() > 3 || entries.iter().enumerate().any(|(index, entry)| entries[..index].iter().any(|other| other.key == entry.key)) { return None; }
    let (mut base, mut count, mut bytes) = (None, 0, 0);
    for entry in entries {
        match entry.key.as_str() {
            "Type" if entry.value.as_name() == Some("Encoding") => {}
            "BaseEncoding" => base = Some(simple_base(entry.value.as_name()?)?),
            "Differences" => {
                let items = entry.value.as_array().filter(|items| items.len() <= MAX_DIFFERENCE_ITEMS)?;
                for item in items {
                    match item {
                        PdfObject::Int(code) if (0..=0xFFFF).contains(code) => {}
                        PdfObject::Name(glyph) if glyph.len() <= MAX_GLYPH_BYTES => { count += 1; bytes += glyph.len(); }
                        _ => return None,
                    }
                }
                if count > MAX_DIFFERENCES { return None; }
            }
            _ => return None,
        }
    }
    Some((base, count, bytes))
}

fn encoding_differences(value: &PdfObject, count: usize) -> Vec<PdfEncodingDifference> {
    let mut differences = Vec::with_capacity(count);
    let Some(items) = value.dict_get("Encoding").and_then(|encoding| encoding.dict_get("Differences")).and_then(PdfObject::as_array) else { return differences; };
    let mut code = 0u32;
    for item in items {
        match item {
            PdfObject::Int(value) => code = u32::try_from(*value).unwrap_or(0),
            PdfObject::Name(glyph) => { differences.push(PdfEncodingDifference { code, glyph: glyph.clone() }); code += 1; }
            _ => {}
        }
    }
    differences
}

fn standard_type1(value: Option<&PdfObject>) -> Option<StandardType1> {
    let PdfObject::Dict(dict) = value? else { return None; };
    if dict.len() > 4 || dict.iter().enumerate().any(|(index, entry)| {
        !matches!(entry.key.as_str(), "Type" | "Subtype" | "BaseFont" | "Encoding")
            || dict[..index].iter().any(|other| other.key == entry.key)
    }) { return None; }
    if name(dict, "Type") != Some("Font") || name(dict, "Subtype") != Some("Type1") { return None; }
    let base_font = name(dict, "BaseFont")?;
    let font = super::standard_font(base_font)?;
    if font.name != base_font { return None; }
    let (base, differences, glyph_bytes) = match get(dict, "Encoding") {
        None => (None, 0, 0),
        Some(PdfObject::Dict(entries)) => encoding_dictionary(entries)?,
        Some(value) => (Some(simple_base(value.as_name()?)?), 0, 0),
    };
    Some(StandardType1 { base_font: font.name, base, differences, glyph_bytes })
}
