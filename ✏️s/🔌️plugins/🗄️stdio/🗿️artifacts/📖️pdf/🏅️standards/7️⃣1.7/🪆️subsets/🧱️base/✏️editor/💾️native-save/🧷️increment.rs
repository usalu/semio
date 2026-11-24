//! 🧷️ Plans an appended (incremental) section for an imported PDF: one bounded step per page lifts the page in its own slice of the retained graph, compares it with the typed page and grafts only what moved.

use std::{cell::Cell, collections::{BTreeMap, HashSet}, sync::Arc};
use semio_framework_value::{ByteReadSource, RetireOwned};
use crate::standards::v1_7::subsets::base::{
    io::{export::{Moved, Reading, graft::Graft}, import::original_custody::{self, GraphDigest, RetainedOriginal}},
    modules::{lexer::dict_get, lower::lower_info},
    schema::{graph_source::ObjectSource, snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfSnapshot}},
};
use super::{classic_graph_quote::GraphLoan, graph::object_capacity, original_tail::{OriginalTail, TailRefusal}};

/// ⛽️ Fuel one page step spends at least; a slice that reads more is charged by the bytes it read.
pub const PAGE_WORK: usize = 4096;

/// 🍰️ The most bytes one slice may read before the page counts as too large for the appended route.
const SLICE_BYTES: usize = 8 << 20;

/// 🧾️ Bytes the baseline digest hashes per step.
const DIGEST_BYTES: usize = 64 * 1024;

/// 🗜️ Why an edit cannot be spelled as an appended section and the whole document must be written compactly instead.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Compact { Original(TailRefusal), Encrypted, Root, Baseline, Retained, Tree, Document, Page, Resources, Extent }

/// 📎️ The retained original bytes of an imported document and the digest of the retained graph as it was opened.
#[derive(Clone)]
pub struct OriginalBase {
    pub source: Arc<dyn ByteReadSource + Send + Sync>,
    pub objects: [u8; 32],
}

/// 🧷️ The changed and new objects of an appended section, ascending by object number.
#[derive(Default, RetireOwned)]
pub struct IncrementRows {
    pub rows: Vec<PdfIndirectObject>,
}

/// 💰️ The retained-grant room one step may spend on new rows.
#[derive(Clone, Copy, Debug)]
pub struct IncrementRoom { pub copy: usize, pub capacity: usize }

/// ⏱️ What one planning step cost.
#[derive(Clone, Copy, Debug, Default)]
pub struct IncrementTurn { pub work: usize, pub copied: usize, pub capacity: usize, pub ready: bool }

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum Stage { #[default] Baseline, Tail, Document, Pages, Restore, Coverage, Done }

#[derive(Default)]
struct Seen { fonts: HashSet<String>, xobjects: HashSet<String>, states: HashSet<String>, shadings: HashSet<String>, patterns: HashSet<String>, colors: HashSet<String>, properties: HashSet<String> }

pub struct IncrementPlan {
    base: Option<OriginalBase>,
    document: Option<String>,
    held: Option<Arc<RetainedOriginal>>,
    measured: [u8; 32],
    touched: Vec<u32>,
    pending: Vec<u32>,
    fresh: u32,
    stage: Stage,
    tail: Option<OriginalTail>,
    digest: GraphDigest,
    root: Option<ObjRef>,
    catalog: Option<ObjRef>,
    next: u32,
    page: usize,
    rows: IncrementRows,
    info: Option<ObjRef>,
    seen: Seen,
}

impl IncrementPlan {
    /// 📎️ Plans against an explicitly given original file and baseline digest.
    pub fn new(base: OriginalBase) -> Self { Self::with(Some(base), None) }

    /// 🗃️ Plans against the file the local custody holds for `document`: the file it last saved, else the one it was opened from.
    pub fn retained(document: String) -> Self { Self::with(None, Some(document)) }

    fn with(base: Option<OriginalBase>, document: Option<String>) -> Self {
        Self { base, document, held: None, measured: [0; 32], touched: Vec::new(), pending: Vec::new(), fresh: 0,
            stage: Stage::Baseline, tail: None, digest: GraphDigest::default(), root: None, catalog: None, next: 0, page: 0,
            rows: IncrementRows::default(), info: None, seen: Seen::default() }
    }

    pub fn ready(&self) -> bool { self.stage == Stage::Done }
    pub fn tail(&self) -> Option<OriginalTail> { self.tail }
    pub fn info(&self) -> Option<ObjRef> { self.info }
    pub fn rows(&self) -> &[PdfIndirectObject] { &self.rows.rows }
    pub fn source(&self) -> Option<&Arc<dyn ByteReadSource + Send + Sync>> { self.base.as_ref().map(|base| &base.source) }

    /// 🗃️ The custody file this plan appends to, once it resolved one.
    pub fn held(&self) -> Option<&Arc<RetainedOriginal>> { self.held.as_ref() }

    /// 🧾️ The digest of the retained graph this plan measured.
    pub fn measured(&self) -> [u8; 32] { self.measured }

    /// 🧷️ The existing object numbers every appended section so far has replaced, ascending.
    pub fn replaced(&self) -> Vec<u32> {
        let mut numbers: Vec<u32> = self.touched.iter().copied()
            .chain(self.rows.rows.iter().map(|row| row.id.num).filter(|number| *number < self.fresh)).collect();
        numbers.sort_unstable();
        numbers.dedup();
        numbers
    }
    pub fn into_rows(self) -> IncrementRows { self.rows }

    /// 🔢️ The number of entries the new table reaches: the original's, or one past the highest appended object.
    pub fn size(&self) -> u32 {
        let last = self.rows.rows.last().map_or(0, |row| row.id.num + 1);
        self.tail.map_or(last, |tail| tail.size.max(last))
    }

    /// 🪜️ Advances one bounded step: the tail, the baseline digest, the document lanes, one page, then the resource coverage.
    pub fn step(&mut self, loan: GraphLoan<'_>, snapshot: &PdfSnapshot, room: IncrementRoom) -> Result<IncrementTurn, Compact> {
        match self.stage {
            Stage::Baseline => self.hash_baseline(loan),
            Stage::Tail => self.read_tail(loan, snapshot),
            Stage::Document => self.lift_document(loan, snapshot, room),
            Stage::Pages => self.lift_page(loan, snapshot, room),
            Stage::Restore => self.restore(loan, room),
            Stage::Coverage => self.coverage(snapshot),
            Stage::Done => Ok(IncrementTurn { ready: true, ..Default::default() }),
        }
    }

    fn read_tail(&mut self, loan: GraphLoan<'_>, snapshot: &PdfSnapshot) -> Result<IncrementTurn, Compact> {
        if snapshot.encryption.is_some() { return Err(Compact::Encrypted); }
        let base = self.base.as_ref().ok_or(Compact::Retained)?;
        let tail = OriginalTail::read(&*base.source).map_err(|refusal| match refusal {
            TailRefusal::Encrypted => Compact::Encrypted,
            other => Compact::Original(other),
        })?;
        if tail.root != loan.trailer.root { return Err(Compact::Root); }
        let last = loan.base_len().checked_sub(1).and_then(|last| loan.base(last)).map_or(0, |row| row.id.num);
        self.next = last.max(tail.size - 1).checked_add(1).ok_or(Compact::Extent)?;
        self.fresh = self.next;
        self.tail = Some(tail);
        self.stage = Stage::Document;
        Ok(IncrementTurn { work: 1, ..Default::default() })
    }

    fn hash_baseline(&mut self, loan: GraphLoan<'_>) -> Result<IncrementTurn, Compact> {
        let spent = self.digest.advance(loan.base_len(), |index| loan.base(index), DIGEST_BYTES);
        let work = (spent / 16).max(1);
        if self.digest.done(loan.base_len()) {
            let digest = std::mem::take(&mut self.digest).finish(loan.base_len());
            if let Some(base) = &self.base {
                if base.objects != digest { return Err(Compact::Baseline); }
            } else {
                let document = self.document.as_deref().ok_or(Compact::Retained)?;
                let held = original_custody::find(&digest, document).ok_or(Compact::Retained)?;
                self.touched = held.touched.clone();
                self.pending = held.touched.clone();
                self.base = Some(OriginalBase { source: held.clone(), objects: digest });
                self.held = Some(held);
            }
            self.measured = digest;
            self.stage = Stage::Tail;
        }
        Ok(IncrementTurn { work, ..Default::default() })
    }

    /// ♻️ Re-states one object an earlier appended section replaced, so a document edited back (undone) to the retained value does not keep the earlier section's value.
    fn restore(&mut self, loan: GraphLoan<'_>, room: IncrementRoom) -> Result<IncrementTurn, Compact> {
        let Some(number) = self.pending.pop() else {
            self.stage = Stage::Coverage;
            return Ok(IncrementTurn { work: 1, ..Default::default() });
        };
        if self.rows.rows.iter().any(|row| row.id.num == number) { return Ok(IncrementTurn { work: 1, ..Default::default() }); }
        let Some(object) = loan.base_index(number).and_then(|index| loan.base(index)) else { return Ok(IncrementTurn { work: 1, ..Default::default() }) };
        let capacity = row_capacity(object);
        let copied = capacity + size_of::<PdfIndirectObject>();
        if copied > room.copy || capacity > room.capacity { return Err(Compact::Extent); }
        self.rows.rows.push(object.clone());
        Ok(IncrementTurn { work: (capacity / 16).max(1), copied, capacity, ready: false })
    }

    fn lift_document(&mut self, loan: GraphLoan<'_>, snapshot: &PdfSnapshot, room: IncrementRoom) -> Result<IncrementTurn, Compact> {
        let catalog = loan.trailer.root;
        let root = dictionary(loan, catalog).and_then(|entries| dict_get(entries, "Pages")).and_then(reference).ok_or(Compact::Tree)?;
        let tree = dictionary(loan, root).ok_or(Compact::Tree)?;
        let Some(PdfObject::Array(kids)) = dict_get(tree, "Kids") else { return Err(Compact::Tree) };
        if kids.len() != snapshot.pages.len() || !name_is(tree, "Type", "Pages") { return Err(Compact::Tree); }
        let mut unique = HashSet::with_capacity(kids.len());
        if !kids.iter().all(|kid| reference(kid).is_some_and(|kid| unique.insert(kid))) { return Err(Compact::Tree); }
        self.root = Some(root);
        self.catalog = Some(catalog);
        let mut seeds = BTreeMap::new();
        seeds.insert(root.num, row(root, PdfObject::Dict(restrict(tree, &[]))));
        let trailer = trailer_entries(loan);
        let version = &snapshot.declared_version;
        let bytes = Cell::new(0);
        let reading = Reading::of(Slice { loan, seeds: &seeds, bytes: &bytes }, &trailer, version).map_err(|_| Compact::Document)?;
        let moved = Moved::between(&reading.lanes, snapshot);
        if !moved.catalog.is_empty() || moved.regenerates() || reading.lanes.document_id != snapshot.document_id {
            return Err(Compact::Document);
        }
        let mut work = PAGE_WORK.max(bytes.get() / 16);
        if moved.info {
            let reference = loan.trailer.info.unwrap_or(ObjRef { num: self.next, gen: 0 });
            let info = row(reference, PdfObject::Dict(lower_info(&snapshot.info)));
            let mut check = seeds.clone();
            check.insert(reference.num, info.clone());
            let mut trailer = trailer;
            trailer.retain(|entry| entry.key != "Info");
            trailer.push(PdfDictEntry::new("Info", PdfObject::Ref(reference)));
            bytes.set(0);
            let reading = Reading::of(Slice { loan, seeds: &check, bytes: &bytes }, &trailer, version).map_err(|_| Compact::Document)?;
            work += bytes.get() / 16;
            if Moved::between(&reading.lanes, snapshot).info { return Err(Compact::Document); }
            let capacity = row_capacity(&info);
            let copied = capacity + size_of::<PdfIndirectObject>();
            if copied > room.copy || capacity > room.capacity { return Err(Compact::Extent); }
            if reference.num == self.next { self.next += 1; }
            self.rows.rows.push(info);
            self.info = Some(reference);
            self.stage = Stage::Pages;
            return Ok(IncrementTurn { work, copied, capacity, ready: false });
        }
        self.stage = Stage::Pages;
        Ok(IncrementTurn { work, ..Default::default() })
    }

    fn lift_page(&mut self, loan: GraphLoan<'_>, snapshot: &PdfSnapshot, room: IncrementRoom) -> Result<IncrementTurn, Compact> {
        let index = self.page;
        if index == snapshot.pages.len() {
            self.stage = Stage::Restore;
            return Ok(IncrementTurn { work: 1, ..Default::default() });
        }
        let (catalog, root) = self.catalog.zip(self.root).ok_or(Compact::Tree)?;
        let tree = dictionary(loan, root).ok_or(Compact::Tree)?;
        let Some(PdfObject::Array(kids)) = dict_get(tree, "Kids") else { return Err(Compact::Tree) };
        let page_ref = kids.get(index).and_then(reference).ok_or(Compact::Tree)?;
        let original = dictionary(loan, page_ref).ok_or(Compact::Tree)?;
        if !name_is(original, "Type", "Page") || dict_get(original, "Parent").and_then(reference) != Some(root) { return Err(Compact::Tree); }
        let mut seeds = BTreeMap::new();
        seeds.insert(catalog.num, row(catalog, PdfObject::Dict(vec![
            PdfDictEntry::new("Type", PdfObject::name("Catalog")), PdfDictEntry::new("Pages", PdfObject::Ref(root))])));
        seeds.insert(root.num, row(root, PdfObject::Dict(restrict(tree, &[page_ref]))));
        seeds.insert(page_ref.num, row(page_ref, PdfObject::Dict(original.to_vec())));
        for owner in [original, tree] {
            if let Some(resources) = dict_get(owner, "Resources") { seed_resources(loan, resources, &mut seeds, 0); }
        }
        let trailer = vec![PdfDictEntry::new("Root", PdfObject::Ref(catalog))];
        let version = &snapshot.declared_version;
        let bytes = Cell::new(0);
        let reading = Reading::of(Slice { loan, seeds: &seeds, bytes: &bytes }, &trailer, version).map_err(|_| Compact::Page)?;
        let mut work = PAGE_WORK.max(bytes.get() / 16);
        let typed = &snapshot.pages[index];
        if reading.lanes.pages.len() != 1 || !resources_agree(&reading.lanes, snapshot) { return Err(Compact::Resources); }
        let mut turn = IncrementTurn::default();
        let mut final_reading = reading;
        if final_reading.lanes.pages[0] != *typed {
            let probe = PdfSnapshot { pages: vec![typed.clone()], objects: Vec::new(), trailer: Vec::new(), admitted_stream_roles: Vec::new(), ..final_reading.lanes.clone() };
            let before = self.next;
            let mut graft = Graft::sparse(&probe, seeds.clone(), trailer.clone(), self.next);
            let original = final_reading;
            let mut moved = Moved::between(&original.lanes, &probe);
            if moved.resources || !moved.catalog.is_empty() || moved.info || moved.regenerates() { return Err(Compact::Page); }
            let mut current: Option<Reading> = None;
            let mut settled = None;
            for _ in 0..3 {
                if !graft.round(current.as_ref().unwrap_or(&original), &original, &moved).map_err(|_| Compact::Page)? { return Err(Compact::Page); }
                bytes.set(0);
                let reading = Reading::of(Slice { loan, seeds: &graft.graph, bytes: &bytes }, &graft.trailer, version).map_err(|_| Compact::Page)?;
                work += bytes.get() / 16;
                let still = Moved::between(&reading.lanes, &probe);
                if still.is_empty() { settled = Some(reading); break; }
                if still == moved { return Err(Compact::Page); }
                moved = still;
                current = Some(reading);
            }
            let reading = settled.ok_or(Compact::Page)?;
            let mut added = Vec::new();
            let (mut capacity, mut copied) = (0, 0);
            for (number, object) in &graft.graph {
                match seeds.get(number) {
                    Some(seed) if seed == object => continue,
                    Some(_) if *number != page_ref.num => return Err(Compact::Page),
                    None if *number < before => return Err(Compact::Page),
                    _ => {}
                }
                capacity += row_capacity(object);
                copied += row_capacity(object) + size_of::<PdfIndirectObject>();
                added.push(object.clone());
            }
            if copied > room.copy || capacity > room.capacity { return Err(Compact::Extent); }
            self.next = graft.next;
            self.rows.rows.extend(added);
            turn.copied = copied;
            turn.capacity = capacity;
            final_reading = reading;
        }
        if work > SLICE_BYTES { return Err(Compact::Extent); }
        self.record(&final_reading.lanes);
        self.page += 1;
        turn.work = work;
        Ok(turn)
    }

    fn record(&mut self, lanes: &PdfSnapshot) {
        let seen = &mut self.seen;
        seen.fonts.extend(lanes.fonts.iter().map(|item| item.id.clone()));
        seen.xobjects.extend(lanes.images.iter().map(|item| item.id.clone()).chain(lanes.forms.iter().map(|item| item.id.clone())));
        seen.states.extend(lanes.ext_g_states.iter().map(|item| item.id.clone()));
        seen.shadings.extend(lanes.shadings.iter().map(|item| item.id.clone()));
        seen.patterns.extend(lanes.patterns.iter().map(|item| item.id.clone()));
        seen.colors.extend(lanes.color_spaces.iter().map(|item| item.name.clone()));
        seen.properties.extend(lanes.properties.iter().map(|item| item.name.clone()));
    }

    fn coverage(&mut self, snapshot: &PdfSnapshot) -> Result<IncrementTurn, Compact> {
        let seen = &self.seen;
        let covered = snapshot.fonts.iter().all(|item| seen.fonts.contains(&item.id))
            && snapshot.images.iter().all(|item| seen.xobjects.contains(&item.id))
            && snapshot.forms.iter().all(|item| seen.xobjects.contains(&item.id))
            && snapshot.ext_g_states.iter().all(|item| seen.states.contains(&item.id))
            && snapshot.shadings.iter().all(|item| seen.shadings.contains(&item.id))
            && snapshot.patterns.iter().all(|item| seen.patterns.contains(&item.id))
            && snapshot.color_spaces.iter().all(|item| seen.colors.contains(&item.name))
            && snapshot.properties.iter().all(|item| seen.properties.contains(&item.name));
        if !covered { return Err(Compact::Resources); }
        self.rows.rows.sort_by_key(|row| row.id.num);
        if self.rows.rows.windows(2).any(|pair| pair[0].id.num == pair[1].id.num) { return Err(Compact::Page); }
        self.stage = Stage::Done;
        Ok(IncrementTurn { work: 1, ready: true, ..Default::default() })
    }
}

/// 🔍️ Whether every resource the slice's page binds is exactly the typed document's resource of that id.
fn resources_agree(read: &PdfSnapshot, typed: &PdfSnapshot) -> bool {
    read.fonts.iter().all(|item| typed.font(&item.id) == Some(item))
        && read.images.iter().all(|item| typed.image(&item.id) == Some(item))
        && read.forms.iter().all(|item| typed.form(&item.id) == Some(item))
        && read.ext_g_states.iter().all(|item| typed.ext_g_state(&item.id) == Some(item))
        && read.shadings.iter().all(|item| typed.shading(&item.id) == Some(item))
        && read.patterns.iter().all(|item| typed.pattern(&item.id) == Some(item))
        && read.color_spaces.iter().all(|item| typed.color_spaces.iter().any(|candidate| candidate == item))
        && read.properties.iter().all(|item| typed.properties.iter().any(|candidate| candidate == item))
}

/// 🍰️ The retained graph seen through a handful of overriding rows; the slice counts what it reads so a page that drags in too much falls back to the compact route.
struct Slice<'a> {
    loan: GraphLoan<'a>,
    seeds: &'a BTreeMap<u32, PdfIndirectObject>,
    bytes: &'a Cell<usize>,
}

impl ObjectSource for Slice<'_> {
    fn inspect(&mut self, reference: ObjRef, read: &mut dyn FnMut(Option<&PdfObject>)) {
        let object = self.seeds.get(&reference.num).filter(|object| object.id == reference).or_else(|| self.loan.lookup(reference));
        self.bytes.set(self.bytes.get() + object.map_or(0, |object| match &object.value { PdfObject::Stream { data, .. } => data.len() + 64, _ => 64 }));
        read(object.map(|object| &object.value));
    }

    fn get(&mut self, reference: ObjRef) -> Option<PdfObject> {
        let mut found = None;
        self.inspect(reference, &mut |value| found = value.cloned());
        found
    }
}

fn row(id: ObjRef, value: PdfObject) -> PdfIndirectObject { PdfIndirectObject { id, value } }

fn reference(value: &PdfObject) -> Option<ObjRef> {
    match value { PdfObject::Ref(reference) => Some(*reference), _ => None }
}

fn dictionary(loan: GraphLoan<'_>, owner: ObjRef) -> Option<&[PdfDictEntry]> {
    match &loan.lookup(owner)?.value { PdfObject::Dict(entries) => Some(entries), _ => None }
}

fn name_is(dictionary: &[PdfDictEntry], key: &str, wanted: &str) -> bool {
    matches!(dict_get(dictionary, key), Some(PdfObject::Name(name)) if name == wanted)
}

/// 🌳️ The page tree node with only `kids` below it.
fn restrict(tree: &[PdfDictEntry], kids: &[ObjRef]) -> Vec<PdfDictEntry> {
    let mut entries: Vec<PdfDictEntry> = tree.iter().filter(|entry| !matches!(entry.key.as_str(), "Kids" | "Count")).cloned().collect();
    entries.push(PdfDictEntry::new("Kids", PdfObject::Array(kids.iter().map(|kid| PdfObject::Ref(*kid)).collect())));
    entries.push(PdfDictEntry::new("Count", PdfObject::Int(kids.len() as i64)));
    entries
}

fn trailer_entries(loan: GraphLoan<'_>) -> Vec<PdfDictEntry> {
    let mut entries = vec![PdfDictEntry::new("Root", PdfObject::Ref(loan.trailer.root))];
    if let Some(info) = loan.trailer.info { entries.push(PdfDictEntry::new("Info", PdfObject::Ref(info))); }
    if let Some([first, second]) = &loan.trailer.id {
        entries.push(PdfDictEntry::new("ID", PdfObject::Array(vec![PdfObject::Str(first.clone()), PdfObject::Str(second.clone())])));
    }
    entries
}

/// 📚️ Seeds the dictionaries a page's resources are made of (the resource dictionary, its sections and the font dictionaries), never a stream, so a graft can re-state them.
fn seed_resources(loan: GraphLoan<'_>, value: &PdfObject, seeds: &mut BTreeMap<u32, PdfIndirectObject>, depth: usize) {
    match value {
        PdfObject::Ref(target) if depth < 4 => {
            if let Some(object) = loan.lookup(*target).filter(|object| matches!(object.value, PdfObject::Dict(_))) {
                if seeds.insert(target.num, object.clone()).is_none() {
                    if let PdfObject::Dict(entries) = &object.value {
                        for entry in entries { seed_resources(loan, &entry.value, seeds, depth + 1); }
                    }
                }
            }
        }
        PdfObject::Dict(entries) if depth < 4 => for entry in entries { seed_resources(loan, &entry.value, seeds, depth + 1); },
        _ => {}
    }
}

fn row_capacity(object: &PdfIndirectObject) -> usize { object_capacity(&object.value) }

/// 🧾️ The digest of a retained graph the document opened with, computed in one go.
pub fn objects_digest(objects: &[PdfIndirectObject]) -> [u8; 32] { original_custody::objects_digest(objects) }
