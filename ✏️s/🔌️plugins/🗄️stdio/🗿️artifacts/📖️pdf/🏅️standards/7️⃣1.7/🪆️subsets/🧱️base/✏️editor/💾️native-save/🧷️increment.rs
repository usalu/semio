//! 🧷️ Plans an appended (incremental) section for an imported PDF: one bounded step per page lifts the page in its own slice of the retained graph, compares it with the typed page and grafts only what moved.

use std::{cell::Cell, collections::{BTreeMap, HashSet}, sync::Arc};
use semio_framework_value::{ByteReadSource, RetireOwned};
use crate::standards::v1_7::subsets::base::{
    io::{export::{Moved, Reading, graft::Graft}, import::original_custody::{self, GraphDigest, RetainedOriginal}},
    modules::{content::content_references, lexer::dict_get, lift::{Category, ResourceClaims}, lower::lower_info},
    schema::{graph_source::ObjectSource, snapshot::{ObjRef, PdfAnnotationKind, PdfDictEntry, PdfIndirectObject, PdfObject, PdfSnapshot}},
};
use super::{classic_graph_quote::GraphLoan, graph::object_capacity, original_tail::{OriginalTail, TailRefusal}};

/// ⛽️ Fuel one page step spends at least; a slice that reads more is charged by the bytes it read.
pub const PAGE_WORK: usize = 4096;

/// 🆔 The most resource ids the plan's claim tables hold before the appended route gives up; the tables are plan state outside the retained grant, like the seeds and the page list.
const CLAIMED_IDS: usize = 1 << 16;

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
    leaves: Vec<(ObjRef, ObjRef)>,
    existing: Option<ExistingResources>,
    claims: ResourceClaims,
}

impl IncrementPlan {
    /// 📎️ Plans against an explicitly given original file and baseline digest.
    pub fn new(base: OriginalBase) -> Self { Self::with(Some(base), None) }

    /// 🗃️ Plans against the file the local custody holds for `document`: the file it last saved, else the one it was opened from.
    pub fn retained(document: String) -> Self { Self::with(None, Some(document)) }

    fn with(base: Option<OriginalBase>, document: Option<String>) -> Self {
        Self { base, document, held: None, measured: [0; 32], touched: Vec::new(), pending: Vec::new(), fresh: 0,
            stage: Stage::Baseline, tail: None, digest: GraphDigest::default(), root: None, catalog: None, next: 0, page: 0,
            rows: IncrementRows::default(), info: None, seen: Seen::default(), leaves: Vec::new(), existing: None, claims: ResourceClaims::default() }
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
        if !name_is(tree, "Type", "Pages") { return Err(Compact::Tree); }
        self.leaves = page_leaves(loan, root)?;
        if self.leaves.len() != snapshot.pages.len() { return Err(Compact::Tree); }
        self.root = Some(root);
        self.catalog = Some(catalog);
        let mut seeds = BTreeMap::new();
        if !snapshot.outlines.is_empty() || !snapshot.named_destinations.is_empty() || snapshot.open_action.is_some() {
            seed_page_stubs(loan, &mut seeds, &self.leaves, None)?;
        } else {
            seeds.insert(root.num, row(root, PdfObject::Dict(restrict(tree, &[]))));
        }
        let trailer = trailer_entries(loan);
        let version = &snapshot.declared_version;
        let bytes = Cell::new(0);
        let claimed = self.claims.mark();
        let reading = Reading::of_claiming(Slice { loan, seeds: &seeds, bytes: &bytes }, &trailer, version, &mut self.claims).map_err(|_| Compact::Document)?;
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
            self.claims.rollback(claimed);
            let reading = Reading::of_claiming(Slice { loan, seeds: &check, bytes: &bytes }, &trailer, version, &mut self.claims).map_err(|_| Compact::Document)?;
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
        let (page_ref, parent) = *self.leaves.get(index).ok_or(Compact::Tree)?;
        let original = dictionary(loan, page_ref).ok_or(Compact::Tree)?;
        if !name_is(original, "Type", "Page") || dict_get(original, "Parent").and_then(reference) != Some(parent) { return Err(Compact::Tree); }
        let chain = ancestors(loan, parent, root)?;
        let mut seeds = BTreeMap::new();
        let mut resource_rows = HashSet::new();
        let mut stub = vec![PdfDictEntry::new("Type", PdfObject::name("Catalog")), PdfDictEntry::new("Pages", PdfObject::Ref(root))];
        if let Some(properties) = dictionary(loan, catalog).and_then(|entries| dict_get(entries, "OCProperties")) {
            seed_resources(loan, properties, &mut seeds, &mut resource_rows, 0);
            stub.push(PdfDictEntry::new("OCProperties", properties.clone()));
        }
        seeds.insert(catalog.num, row(catalog, PdfObject::Dict(stub)));
        let typed = &snapshot.pages[index];
        let links = typed.annotations.iter().any(|annotation| matches!(annotation.kind, PdfAnnotationKind::Link { .. } | PdfAnnotationKind::Widget { .. }));
        let at = if links { index } else { 0 };
        if links {
            seed_page_stubs(loan, &mut seeds, &self.leaves, Some(page_ref))?;
        } else {
            for (position, node) in chain.iter().enumerate() {
                let child = if position == 0 { page_ref } else { chain[position - 1] };
                seeds.insert(node.num, row(*node, PdfObject::Dict(restrict(dictionary(loan, *node).ok_or(Compact::Tree)?, &[child]))));
            }
        }
        seeds.insert(page_ref.num, row(page_ref, PdfObject::Dict(original.to_vec())));
        for owner in std::iter::once(page_ref).chain(chain.iter().copied()) {
            if let Some(resources) = dictionary(loan, owner).and_then(|owner| dict_get(owner, "Resources")) { seed_resources(loan, resources, &mut seeds, &mut resource_rows, 0); }
        }
        let trailer = vec![PdfDictEntry::new("Root", PdfObject::Ref(catalog))];
        let version = &snapshot.declared_version;
        let bytes = Cell::new(0);
        let claimed = self.claims.mark();
        let reading = Reading::of_claiming(Slice { loan, seeds: &seeds, bytes: &bytes }, &trailer, version, &mut self.claims).map_err(|_| Compact::Page)?;
        if self.claims.len() > CLAIMED_IDS { return Err(Compact::Extent); }
        let mut work = PAGE_WORK.max(bytes.get() / 16);
        if reading.lanes.pages.len() != if links { self.leaves.len() } else { 1 } || !resources_agree(&reading.lanes, snapshot) { return Err(Compact::Resources); }
        let mut turn = IncrementTurn::default();
        let mut final_reading = reading;
        if final_reading.lanes.pages[at] != *typed {
            let mut pages = final_reading.lanes.pages.clone();
            pages[at] = typed.clone();
            let mut probe = PdfSnapshot { pages, objects: final_reading.lanes.objects.clone(), trailer: Vec::new(), admitted_stream_roles: Vec::new(), ..final_reading.lanes.clone() };
            let used = content_references(&typed.content);
            let mut unseen: Vec<(Category, &'static str, &String)> = Vec::new();
            for id in &used.fonts { if probe.font(id).is_none() { unseen.push((Category::Font, "Font", id)); probe.fonts.extend(snapshot.font(id).cloned()); } }
            for id in &used.x_objects {
                if probe.image(id).is_none() && probe.form(id).is_none() { unseen.push((Category::XObject, "XObject", id)); }
                if probe.image(id).is_none() { probe.images.extend(snapshot.image(id).cloned()); }
                if probe.form(id).is_none() { probe.forms.extend(snapshot.form(id).cloned()); }
            }
            for id in &used.ext_g_states { if probe.ext_g_state(id).is_none() { unseen.push((Category::ExtGState, "ExtGState", id)); probe.ext_g_states.extend(snapshot.ext_g_state(id).cloned()); } }
            for id in &used.shadings { if probe.shading(id).is_none() { probe.shadings.extend(snapshot.shading(id).cloned()); } }
            for id in &used.patterns { if probe.pattern(id).is_none() { probe.patterns.extend(snapshot.pattern(id).cloned()); } }
            for name in &used.color_spaces { if !probe.color_spaces.iter().any(|space| &space.name == name) { probe.color_spaces.extend(snapshot.color_spaces.iter().find(|space| &space.name == name).cloned()); } }
            for name in &used.properties { if !probe.properties.iter().any(|item| &item.name == name) { probe.properties.extend(snapshot.properties.iter().find(|item| &item.name == name).cloned()); } }
            let before = self.next;
            let mut graft = Graft::sparse(&probe, seeds.clone(), trailer.clone(), self.next);
            if !unseen.is_empty() {
                if self.existing.is_none() {
                    work += self.leaves.len() * 16;
                    self.existing = Some(ExistingResources::index(loan, &self.leaves)?);
                }
                let index = self.existing.as_ref().expect("indexed existing resources");
                graft.bind_existing(unseen.iter().filter_map(|(category, _, id)| index.unique(*category, id).map(|target| ((*category, (*id).clone()), target))).collect());
            }
            let original = final_reading;
            let mut moved = Moved::between(&original.lanes, &probe);
            if !moved.catalog.is_empty() || moved.info || moved.regenerates() { return Err(Compact::Page); }
            moved.resources = false;
            let mut current: Option<Reading> = None;
            let mut settled = None;
            for _ in 0..3 {
                if !graft.round(current.as_ref().unwrap_or(&original), &original, &moved).map_err(|_| Compact::Page)? { return Err(Compact::Page); }
                bytes.set(0);
                self.claims.rollback(claimed);
                let reading = Reading::of_claiming(Slice { loan, seeds: &graft.graph, bytes: &bytes }, &graft.trailer, version, &mut self.claims).map_err(|_| Compact::Page)?;
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
                    Some(_) if *number != page_ref.num && !resource_rows.contains(number) => return Err(Compact::Page),
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

/// 🔗️ The document's existing indirect resource objects by name, indexed in one pass over every page and page-tree node's `/Resources` (fonts, XObjects and graphics states): a name is bound only when exactly one object carries it. A wrong binding cannot pass the slice's re-read, which must equal the typed resources.
struct ExistingResources(std::collections::HashMap<(Category, String), Option<ObjRef>>);

/// 🧮️ The most names one Save's index holds before the appended route gives up.
const EXISTING_NAMES: usize = 1 << 16;

impl ExistingResources {
    fn index(loan: GraphLoan<'_>, leaves: &[(ObjRef, ObjRef)]) -> Result<Self, Compact> {
        let resolve = |value: &PdfObject| -> Option<Vec<PdfDictEntry>> {
            match value { PdfObject::Dict(entries) => Some(entries.clone()), PdfObject::Ref(target) => dictionary(loan, *target).map(<[PdfDictEntry]>::to_vec), _ => None }
        };
        let mut index: std::collections::HashMap<(Category, String), Option<ObjRef>> = std::collections::HashMap::new();
        let mut owners: HashSet<ObjRef> = HashSet::new();
        for (page, parent) in leaves { owners.insert(*page); owners.insert(*parent); }
        for owner in owners {
            let Some(resources) = dictionary(loan, owner).and_then(|entries| dict_get(entries, "Resources")).and_then(resolve) else { continue };
            for (category, key) in [(Category::Font, "Font"), (Category::XObject, "XObject"), (Category::ExtGState, "ExtGState")] {
                let Some(section) = dict_get(&resources, key).and_then(resolve) else { continue };
                for entry in &section {
                    let PdfObject::Ref(target) = &entry.value else { continue };
                    let slot = index.entry((category, entry.key.clone())).or_insert(Some(*target));
                    if *slot != Some(*target) { *slot = None; }
                }
                if index.len() > EXISTING_NAMES { return Err(Compact::Extent); }
            }
        }
        Ok(Self(index))
    }

    fn unique(&self, category: Category, id: &str) -> Option<ObjRef> { self.0.get(&(category, id.to_owned())).copied().flatten() }
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

/// 📇️ Seeds every node of the page tree as authored and every page except `full` as a bare page under its own parent, so outline, destination and link targets read back as their page indices without lifting another page's content.
fn seed_page_stubs(loan: GraphLoan<'_>, seeds: &mut BTreeMap<u32, PdfIndirectObject>, leaves: &[(ObjRef, ObjRef)], full: Option<ObjRef>) -> Result<(), Compact> {
    for (page, parent) in leaves {
        if !seeds.contains_key(&parent.num) { seeds.insert(parent.num, row(*parent, PdfObject::Dict(dictionary(loan, *parent).ok_or(Compact::Tree)?.to_vec()))); }
        let mut node = *parent;
        for _ in 0..MAX_TREE_DEPTH {
            let Some(up) = dictionary(loan, node).and_then(|entries| dict_get(entries, "Parent")).and_then(reference) else { break; };
            if seeds.contains_key(&up.num) { break; }
            seeds.insert(up.num, row(up, PdfObject::Dict(dictionary(loan, up).ok_or(Compact::Tree)?.to_vec())));
            node = up;
        }
        if Some(*page) != full {
            seeds.insert(page.num, row(*page, PdfObject::Dict(vec![PdfDictEntry::new("Type", PdfObject::name("Page")), PdfDictEntry::new("Parent", PdfObject::Ref(*parent))])));
        }
    }
    Ok(())
}

/// 🌲️ Deepest page-tree nesting the appended route follows.
const MAX_TREE_DEPTH: usize = 64;

/// 🌲️ Every page of a (possibly nested) page tree in document order with its parent node; each node and page reached once.
fn page_leaves(loan: GraphLoan<'_>, root: ObjRef) -> Result<Vec<(ObjRef, ObjRef)>, Compact> {
    let mut leaves = Vec::new();
    let mut seen = HashSet::from([root]);
    let mut stack: Vec<(ObjRef, usize)> = vec![(root, 0)];
    while let Some((node, next)) = stack.pop() {
        let entries = dictionary(loan, node).ok_or(Compact::Tree)?;
        let Some(PdfObject::Array(kids)) = dict_get(entries, "Kids") else { return Err(Compact::Tree) };
        let Some(kid) = kids.get(next) else { continue; };
        stack.push((node, next + 1));
        let kid = reference(kid).filter(|kid| seen.insert(*kid)).ok_or(Compact::Tree)?;
        let child = dictionary(loan, kid).ok_or(Compact::Tree)?;
        if dict_get(child, "Parent").and_then(reference) != Some(node) { return Err(Compact::Tree); }
        if name_is(child, "Type", "Pages") {
            if stack.len() >= MAX_TREE_DEPTH { return Err(Compact::Tree); }
            stack.push((kid, 0));
        } else if name_is(child, "Type", "Page") {
            leaves.push((kid, node));
        } else {
            return Err(Compact::Tree);
        }
    }
    Ok(leaves)
}

/// 🪜️ The page-tree nodes from `parent` up to `root`, both included.
fn ancestors(loan: GraphLoan<'_>, parent: ObjRef, root: ObjRef) -> Result<Vec<ObjRef>, Compact> {
    let mut chain = vec![parent];
    while *chain.last().expect("page tree chain") != root {
        if chain.len() >= MAX_TREE_DEPTH { return Err(Compact::Tree); }
        let up = dictionary(loan, *chain.last().expect("page tree chain")).and_then(|entries| dict_get(entries, "Parent")).and_then(reference).ok_or(Compact::Tree)?;
        chain.push(up);
    }
    Ok(chain)
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
fn seed_resources(loan: GraphLoan<'_>, value: &PdfObject, seeds: &mut BTreeMap<u32, PdfIndirectObject>, owned: &mut HashSet<u32>, depth: usize) {
    match value {
        PdfObject::Ref(target) if depth < 4 => {
            if let Some(object) = loan.lookup(*target).filter(|object| matches!(object.value, PdfObject::Dict(_))) {
                if seeds.insert(target.num, object.clone()).is_none() {
                    owned.insert(target.num);
                    if let PdfObject::Dict(entries) = &object.value {
                        for entry in entries { seed_resources(loan, &entry.value, seeds, owned, depth + 1); }
                    }
                }
            }
        }
        PdfObject::Dict(entries) if depth < 4 => for entry in entries { seed_resources(loan, &entry.value, seeds, owned, depth + 1); },
        PdfObject::Array(items) if depth < 4 => for item in items { seed_resources(loan, item, seeds, owned, depth + 1); },
        _ => {}
    }
}

fn row_capacity(object: &PdfIndirectObject) -> usize { object_capacity(&object.value) }

/// 🧾️ The digest of a retained graph the document opened with, computed in one go.
pub fn objects_digest(objects: &[PdfIndirectObject]) -> [u8; 32] { original_custody::objects_digest(objects) }
