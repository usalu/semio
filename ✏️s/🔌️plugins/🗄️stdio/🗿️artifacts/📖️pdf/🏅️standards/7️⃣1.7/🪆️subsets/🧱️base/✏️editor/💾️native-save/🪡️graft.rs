//! 🪡️ Builds only the paid COS replacements for an existing page's geometry and document Info.

use crate::standards::v1_7::subsets::base::{
    modules::{lexer::CosGrant, lower::lower_info_with},
    schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfInfo, PdfObject, PdfPage, PdfSnapshot},
};

const MAX_DICTIONARY: usize = 32;
const MAX_NODES: usize = 128;
const MAX_LEAF: usize = 128;
pub const GRAFT_WORK: usize = 4096;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GraftRefusal { Catalog, Pages, Rotation, Geometry, Info, Extent, Source }

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct GraftTurn {
    pub work: usize,
    pub copied_bytes: usize,
    pub allocated: usize,
    pub ready: bool,
    pub needs: Option<usize>,
}

#[derive(Default, semio_framework_value::RetireOwned)]
pub struct OrdinaryRows {
    pub page: Option<PdfIndirectObject>,
    pub info: Option<PdfIndirectObject>,
    pub remove: Option<usize>,
    pub info_ref: Option<ObjRef>,
}

impl OrdinaryRows {
    pub fn overlay(&self) -> GraphOverlay<'_> {
        GraphOverlay { page: self.page.as_ref(), info: self.info.as_ref(), remove: self.remove }
    }
}

pub struct OrdinaryGraft {
    root: Option<ObjRef>,
    next_page: usize,
    seen: [bool; 256],
    rows: OrdinaryRows,
    ready: bool,
}

impl Default for OrdinaryGraft {
    fn default() -> Self {
        Self { root: None, next_page: 0, seen: [false; 256], rows: OrdinaryRows::default(), ready: false }
    }
}

impl OrdinaryGraft {
    /// 🛠️ Quotes and funds one change. Unchanged objects always remain borrowed from the original COS.
    pub fn step(
        &mut self, loan: GraphLoan<'_>, snapshot: &PdfSnapshot, grant: CosGrant, maximum_copy: usize,
    ) -> Result<GraftTurn, GraftRefusal> {
        if self.ready { return Ok(GraftTurn { ready: true, ..Default::default() }); }
        if grant.work < GRAFT_WORK { return Ok(Default::default()); }
        if self.root.is_none() {
            if loan.objects.len() > self.seen.len() || loan.objects.first().is_none_or(|row| row.id.num == 0)
                || loan.objects.windows(2).any(|pair| pair[0].id.num >= pair[1].id.num) { return Err(GraftRefusal::Source); }
            let catalog = dictionary(loan, loan.trailer.root).ok_or(GraftRefusal::Catalog)?;
            let root = dict_get(catalog, "Pages").and_then(reference).ok_or(GraftRefusal::Catalog)?;
            let pages = dictionary(loan, root).ok_or(GraftRefusal::Pages)?;
            let Some(PdfObject::Array(kids)) = dict_get(pages, "Kids") else { return Err(GraftRefusal::Pages); };
            if !name_is(pages, "Type", "Pages") || loan.objects.len() > self.seen.len() || kids.len() != snapshot.pages.len()
                || dict_get(pages, "Count") != Some(&PdfObject::Int(kids.len() as i64))
            { return Err(GraftRefusal::Pages); }
            self.root = Some(root);
            self.rows.info_ref = loan.trailer.info;
            return Ok(work(0, false));
        }
        if let Some(page) = snapshot.pages.get(self.next_page) {
            let root = self.root.ok_or(GraftRefusal::Pages)?;
            let pages = dictionary(loan, root).ok_or(GraftRefusal::Pages)?;
            let Some(PdfObject::Array(kids)) = dict_get(pages, "Kids") else { return Err(GraftRefusal::Pages); };
            let owner = kids.get(self.next_page).and_then(reference).ok_or(GraftRefusal::Pages)?;
            let index = loan.objects.binary_search_by_key(&owner.num, |row| row.id.num).map_err(|_| GraftRefusal::Pages)?;
            if self.seen[index] { return Err(GraftRefusal::Pages); }
            let original = dictionary(loan, owner).ok_or(GraftRefusal::Pages)?;
            if !name_is(original, "Type", "Page") || dict_get(original, "Parent").and_then(reference) != Some(root) {
                return Err(GraftRefusal::Pages);
            }
            let old = match dict_get(original, "Rotate").or_else(|| dict_get(pages, "Rotate")) {
                None => 0,
                Some(PdfObject::Int(n)) => i32::try_from(*n).map_err(|_| GraftRefusal::Rotation)?,
                _ => return Err(GraftRefusal::Rotation),
            };
            let changes = patch_page(page, original, pages, old)?;
            let mut allocated = 0;
            if changes.iter().any(Option::is_some) {
                if self.rows.page.is_some() { return Err(GraftRefusal::Geometry); }
                if original.len() > MAX_DICTIONARY { return Err(GraftRefusal::Extent); }
                let mut count = original.len();
                let mut new_bytes = 0;
                for (key, value) in changes.iter().flatten() {
                    let mut entries = original.iter().filter(|entry| entry.key == *key);
                    let present = entries.next().is_some();
                    if entries.next().is_some() { return Err(GraftRefusal::Geometry); }
                    match value {
                        Some(value) => {
                            count += usize::from(!present);
                            new_bytes += key.len() + value.allocated();
                        }
                        None => count -= usize::from(present),
                    }
                }
                if count > MAX_DICTIONARY { return Err(GraftRefusal::Extent); }
                let mut nodes = 0;
                let extent = count * size_of::<PdfDictEntry>() + new_bytes + child_extent(original, &mut nodes, 0)?;
                if let Some(refusal) = missing_grant(extent, grant, maximum_copy) { return Ok(refusal); }
                let entries = replacement_page(original, &changes, count);
                allocated = dict_capacity(&entries, entries.capacity());
                self.rows.page = Some(PdfIndirectObject { id: owner, value: PdfObject::Dict(entries) });
            }
            self.seen[index] = true;
            self.next_page += 1;
            return Ok(work(allocated, false));
        }
        let result = self.info(loan, &snapshot.info, grant, maximum_copy)?;
        self.ready = result.ready;
        Ok(result)
    }

    pub fn ready(&self) -> bool { self.ready }

    pub fn into_rows(self) -> Result<OrdinaryRows, Self> {
        if self.ready { Ok(self.rows) } else { Err(self) }
    }

    pub fn take_partial(self) -> OrdinaryRows { self.rows }

    fn info(&mut self, loan: GraphLoan<'_>, info: &PdfInfo, grant: CosGrant, maximum_copy: usize) -> Result<GraftTurn, GraftRefusal> {
        let current = loan.trailer.info.map(|reference| dictionary(loan, reference).ok_or(GraftRefusal::Info)).transpose()?.unwrap_or(&[]);
        let fields = info_fields(info);
        if !info.extra.is_empty() || info.creation_date.is_some() || info.modification_date.is_some()
            || fields.iter().filter_map(|(_, text)| *text).any(|text| !plain(text.as_bytes()))
        { return Err(GraftRefusal::Info); }
        let mut seen = [false; 7];
        let mut changed = false;
        for entry in current {
            let index = fields.iter().position(|(key, _)| *key == entry.key).ok_or(GraftRefusal::Info)?;
            if std::mem::replace(&mut seen[index], true) { return Err(GraftRefusal::Info); }
            let raw = match &entry.value {
                PdfObject::Str(bytes) if index != 6 => bytes,
                PdfObject::Text(text) if index != 6 => text.as_bytes(),
                PdfObject::Name(text) if index == 6 => text.as_bytes(),
                _ => return Err(GraftRefusal::Info),
            };
            if !plain(raw) { return Err(GraftRefusal::Info); }
            changed |= fields[index].1.map(str::as_bytes) != Some(raw);
        }
        changed |= fields.iter().enumerate().any(|(index, (_, text))| !seen[index] && text.is_some());
        if !changed { return Ok(work(0, true)); }
        if fields.iter().all(|(_, text)| text.is_none()) {
            if let Some(info) = loan.trailer.info {
                let index = loan.objects.binary_search_by_key(&info.num, |row| row.id.num).map_err(|_| GraftRefusal::Info)?;
                self.rows.remove = Some(index);
                self.rows.info_ref = None;
            }
            return Ok(work(0, true));
        }
        let extent = 16 * size_of::<PdfDictEntry>() + fields.iter().filter_map(|(key, text)| text.map(|value| key.len() + value.len())).sum::<usize>();
        if let Some(refusal) = missing_grant(extent, grant, maximum_copy) { return Ok(refusal); }
        let reference = match loan.trailer.info {
            Some(info) => info,
            None => ObjRef { num: loan.objects.last().ok_or(GraftRefusal::Info)?.id.num.checked_add(1).ok_or(GraftRefusal::Extent)?, gen: 0 },
        };
        let dict = lower_info_with(info, |text| text.as_bytes().to_vec());
        let allocated = dict_capacity(&dict, dict.capacity());
        self.rows.info = Some(PdfIndirectObject { id: reference, value: PdfObject::Dict(dict) });
        self.rows.info_ref = Some(reference);
        Ok(work(allocated, true))
    }
}

#[derive(Clone, Copy)]
enum PageValue { Integer(i64), Rectangle([i64; 4]) }

type PageChange = (&'static str, Option<PageValue>);

impl PageValue {
    fn into_object(self) -> PdfObject {
        match self {
            Self::Integer(value) => PdfObject::Int(value),
            Self::Rectangle(rect) => PdfObject::Array(rect.into_iter().map(PdfObject::Int).collect()),
        }
    }

    fn allocated(self) -> usize {
        match self {
            Self::Integer(_) => 0,
            Self::Rectangle(_) => 4 * size_of::<PdfObject>(),
        }
    }
}

fn patch_page(
    page: &PdfPage, original: &[PdfDictEntry], parent: &[PdfDictEntry], rotate: i32,
) -> Result<[Option<PageChange>; 7], GraftRefusal> {
    let box_fields = [
        ("MediaBox", Some(page.media_box), true), ("CropBox", page.crop_box, true),
        ("TrimBox", page.trim_box, false), ("BleedBox", page.bleed_box, false),
        ("ArtBox", page.art_box, false),
    ];
    let mut changes: [Option<PageChange>; 7] = [None; 7];
    for ((key, current, inherited), target) in box_fields.into_iter().zip(&mut changes) {
        let direct = dict_get(original, key);
        let parent_value = if inherited { dict_get(parent, key) } else { None };
        let old = direct.or(parent_value).map(page_rectangle).transpose()?;
        if old == current { continue; }
        if current.is_none() && parent_value.is_some() { return Err(GraftRefusal::Geometry); }
        let value = current.map(|rect| {
            let [left, bottom, right, top] = rect;
            Ok(PageValue::Rectangle([page_integer(left)?, page_integer(bottom)?, page_integer(right)?, page_integer(top)?]))
        }).transpose()?;
        *target = Some((key, value));
    }
    let old_unit = dict_get(original, "UserUnit").map(page_number).transpose()?;
    if old_unit != page.user_unit {
        let value = page.user_unit.map(page_integer).transpose()?;
        if value.is_some_and(|value| value <= 0) { return Err(GraftRefusal::Geometry); }
        changes[5] = Some(("UserUnit", value.map(PageValue::Integer)));
    }
    if rotate != page.rotate {
        if !matches!(page.rotate, 0 | 90 | 180 | 270) { return Err(GraftRefusal::Rotation); }
        changes[6] = Some(("Rotate", Some(PageValue::Integer(i64::from(page.rotate)))));
    }
    Ok(changes)
}

fn page_integer(value: f64) -> Result<i64, GraftRefusal> {
    if value.fract() != 0.0 || !(-1_099_511_627_776.0..=1_099_511_627_776.0).contains(&value) {
        return Err(GraftRefusal::Geometry);
    }
    Ok(value as i64)
}

fn page_rectangle(value: &PdfObject) -> Result<[f64; 4], GraftRefusal> {
    let PdfObject::Array(coordinates) = value else { return Err(GraftRefusal::Geometry); };
    let [left, bottom, right, top] = coordinates.as_slice() else { return Err(GraftRefusal::Geometry); };
    Ok([page_number(left)?, page_number(bottom)?, page_number(right)?, page_number(top)?])
}

fn page_number(value: &PdfObject) -> Result<f64, GraftRefusal> {
    use std::fmt::Write;
    match value {
        PdfObject::Int(value) => Ok(*value as f64),
        PdfObject::Real(value) if value.coefficient.len() <= 32 && value.scale <= 32 && value.coefficient.bytes().all(|byte| byte.is_ascii_digit()) => {
            let mut text = PageNumberText { bytes: [0; 64], len: 0 };
            write!(&mut text, "{value}").map_err(|_| GraftRefusal::Geometry)?;
            std::str::from_utf8(&text.bytes[..text.len]).ok()
                .and_then(|text| text.parse::<f64>().ok()).filter(|value| value.is_finite())
                .ok_or(GraftRefusal::Geometry)
        }
        _ => Err(GraftRefusal::Geometry),
    }
}

struct PageNumberText { bytes: [u8; 64], len: usize }

impl std::fmt::Write for PageNumberText {
    fn write_str(&mut self, text: &str) -> std::fmt::Result {
        let end = self.len + text.len();
        self.bytes.get_mut(self.len..end).ok_or(std::fmt::Error)?.copy_from_slice(text.as_bytes());
        self.len = end;
        Ok(())
    }
}

fn replacement_page(
    original: &[PdfDictEntry], changes: &[Option<PageChange>; 7], count: usize,
) -> Vec<PdfDictEntry> {
    let mut entries = Vec::with_capacity(count);
    for entry in original {
        match changes.iter().flatten().find(|(key, _)| *key == entry.key) {
            Some((key, Some(value))) => entries.push(PdfDictEntry::new(*key, value.into_object())),
            Some((_, None)) => {}
            None => entries.push(entry.clone()),
        }
    }
    for (key, value) in changes.iter().flatten() {
        if let Some(value) = value.filter(|_| dict_get(original, key).is_none()) {
            entries.push(PdfDictEntry::new(*key, value.into_object()));
        }
    }
    entries
}

fn work(allocated: usize, ready: bool) -> GraftTurn {
    GraftTurn { work: GRAFT_WORK, copied_bytes: allocated + if allocated == 0 { 0 } else { size_of::<PdfIndirectObject>() },
        allocated, ready, needs: None }
}

fn missing_grant(extent: usize, grant: CosGrant, maximum_copy: usize) -> Option<GraftTurn> {
    (extent > grant.allocation_bytes || extent + size_of::<PdfIndirectObject>() > maximum_copy)
        .then_some(GraftTurn { work: GRAFT_WORK, needs: Some(extent), ..Default::default() })
}

fn plain(bytes: &[u8]) -> bool { bytes.len() <= MAX_LEAF && bytes.iter().all(|byte| (0x20..=0x7e).contains(byte)) }

fn info_fields(info: &PdfInfo) -> [(&str, Option<&str>); 7] {
    [("Title", info.title.as_deref()), ("Author", info.author.as_deref()), ("Subject", info.subject.as_deref()),
     ("Keywords", info.keywords.as_deref()), ("Creator", info.creator.as_deref()), ("Producer", info.producer.as_deref()),
     ("Trapped", info.trapped.as_deref())]
}

fn reference(value: &PdfObject) -> Option<ObjRef> {
    match value { PdfObject::Ref(reference) => Some(*reference), _ => None }
}

fn dictionary(loan: GraphLoan<'_>, owner: ObjRef) -> Option<&[PdfDictEntry]> {
    match &loan.lookup(owner)?.value {
        PdfObject::Dict(entries) if entries.len() <= MAX_DICTIONARY => Some(entries),
        _ => None,
    }
}

fn dict_get<'a>(dict: &'a [PdfDictEntry], key: &str) -> Option<&'a PdfObject> {
    dict.iter().find(|entry| entry.key == key).map(|entry| &entry.value)
}

fn name_is(dict: &[PdfDictEntry], key: &str, wanted: &str) -> bool {
    matches!(dict_get(dict, key), Some(PdfObject::Name(name)) if name == wanted)
}

fn child_extent(dict: &[PdfDictEntry], budget: &mut usize, depth: usize) -> Result<usize, GraftRefusal> {
    if depth > 6 || dict.len() > MAX_DICTIONARY { return Err(GraftRefusal::Extent); }
    let mut bytes = 0;
    for (index, entry) in dict.iter().enumerate() {
        if entry.key.len() > MAX_LEAF || dict[..index].iter().any(|earlier| earlier.key == entry.key) {
            return Err(GraftRefusal::Extent);
        }
        bytes += entry.key.len() + value_extent(&entry.value, budget, depth + 1)?;
    }
    Ok(bytes)
}

fn value_extent(value: &PdfObject, budget: &mut usize, depth: usize) -> Result<usize, GraftRefusal> {
    *budget += 1;
    if *budget > MAX_NODES || depth > 6 { return Err(GraftRefusal::Extent); }
    match value {
        PdfObject::Dict(dict) => Ok(dict.len() * size_of::<PdfDictEntry>() + child_extent(dict, budget, depth)?),
        PdfObject::Array(values) if values.len() <= MAX_DICTIONARY => {
            let mut bytes = values.len() * size_of::<PdfObject>();
            for value in values { bytes += value_extent(value, budget, depth + 1)?; }
            Ok(bytes)
        }
        PdfObject::Name(text) | PdfObject::Text(text) if text.len() <= MAX_LEAF => Ok(text.len()),
        PdfObject::Str(bytes) if bytes.len() <= MAX_LEAF => Ok(bytes.len()),
        PdfObject::Null | PdfObject::Bool(_) | PdfObject::Int(_) | PdfObject::Ref(_) | PdfObject::Date(_) => Ok(0),
        PdfObject::Real(decimal) if decimal.coefficient.len() <= MAX_LEAF && decimal.scale <= 384 && decimal.coefficient.bytes().all(|byte| byte.is_ascii_digit()) => Ok(decimal.coefficient.len()),
        _ => Err(GraftRefusal::Extent),
    }
}

fn dict_capacity(dict: &[PdfDictEntry], capacity: usize) -> usize {
    capacity * size_of::<PdfDictEntry>() + dict.iter().map(|entry| entry.key.capacity() + value_capacity(&entry.value)).sum::<usize>()
}

fn value_capacity(value: &PdfObject) -> usize {
    match value {
        PdfObject::Dict(dict) => dict_capacity(dict, dict.capacity()),
        PdfObject::Array(values) => values.capacity() * size_of::<PdfObject>() + values.iter().map(value_capacity).sum::<usize>(),
        PdfObject::Name(text) | PdfObject::Text(text) => text.capacity(),
        PdfObject::Str(bytes) => bytes.capacity(),
        PdfObject::Real(decimal) => decimal.coefficient.capacity(),
        _ => 0,
    }
}

use super::classic_graph_quote::{GraphLoan, GraphOverlay};
