//! 🧭️ Projects bounded native PDF pages with vector content, standard Type1 text and simple annotations.

use super::*;
use crate::standards::v1_7::subsets::base::modules::{
    content::{bounded_number, FontTable}, fonts::{standard_font, FontCodec},
};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfBaseEncoding, PdfFont, PdfFontKind, PdfSimpleEncoding};
use std::mem::size_of;

const MAX_DICTIONARY: usize = 32;
const MAX_COPY: usize = 128;
const MAX_ROLE_OWNERS: usize = 32;
const MAX_FONTS: usize = 8;
const MAX_ANNOTATIONS: usize = 8;
const MAX_PAGES: usize = 16_384;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlledLiftBarrier { Trailer, Catalog, Info, PageTree, Page, Resources, Roles, Program }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControlledLiftEvent {
    More, NeedsAllocation { bytes: usize }, NeedsRelease { bytes: usize },
    Complete, Deferred(ControlledLiftBarrier), Cancelled, Closed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[must_use]
pub struct ControlledLiftReceipt {
    pub event: Result<ControlledLiftEvent, &'static str>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

enum Stage {
    Open, Trailer(usize), Catalog(usize), Info(usize), PageCapacity, Walk,
    BranchResources(PageAttribute), ResourceOwner { owner: ObjRef, resume: Resume },
    Font { resources: ObjRef, owner: ObjRef, resume: Resume },
    Page(usize), Inherit(usize), Ancestor { owner: ObjRef, depth: usize },
    AnnotationArray(AnnotationArray, usize, usize), Annotation(AnnotationArray, usize, usize, ObjRef),
    RoleStart, RoleArray { owner: Option<ObjRef>, index: usize, count: usize },
    RoleStream { stream: ObjRef, next: RoleNext }, Program, Mirror, Publish,
    CloseWalk, Ready, Deferred(ControlledLiftBarrier), Cancelled, Closing, Closed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Resume { Walk, Inherit, Role }
#[derive(Clone, Copy)]
enum AnnotationArray { Direct(PageAttribute), Indirect(ObjRef) }
#[derive(Clone, Copy)]
struct RoleNext { owner: Option<ObjRef>, index: usize, count: usize }

/// 🧳️ Unpublished typed ownership for the application’s original-lane retirement.
pub struct ControlledLiftPartial {
    pub snapshot: Option<PdfSnapshot>,
    pub page: Option<PdfPage>,
}

/// 🗺️ Lifts ordinary vector and standard Type1 text pages over the stable borrowed native graph.
#[must_use]
pub struct ControlledDocumentLiftCursor {
    stage: Stage,
    snapshot: Option<PdfSnapshot>,
    page: Option<PdfPage>,
    trailer_seen: u8,
    catalog: Option<ObjRef>,
    info: Option<ObjRef>,
    tree: Option<ObjRef>,
    walk: Option<PageTreeCursor>,
    leaf: Option<PageTreeLeaf>,
    contents: Option<PageAttribute>,
    indirect_contents: bool,
    font_owners: [ObjRef; MAX_FONTS],
    page_font: Option<(ObjRef, usize)>,
    owners: [ObjRef; MAX_ROLE_OWNERS],
    owners_len: usize,
    program: Option<PageProgramCursor>,
}

use ControlledLiftEvent as Event;
use ControlledLiftBarrier as Barrier;

impl ControlledDocumentLiftCursor {
    pub(crate) fn new() -> Self {
        Self {
            stage: Stage::Open, snapshot: None, page: None, trailer_seen: 0, catalog: None, info: None, tree: None,
            walk: None, leaf: None, contents: None, owners: [ObjRef::default(); MAX_ROLE_OWNERS],
            owners_len: 0, program: None, indirect_contents: false,
            font_owners: [ObjRef::default(); MAX_FONTS], page_font: None,
        }
    }

    /// 📦️ Quotes when the next step may reserve empty page, role or annotation slots.
    pub fn reserving_empty_slots(&self) -> bool { matches!(self.stage, Stage::PageCapacity | Stage::AnnotationArray(_, 0, _)) }

    /// 📨️ Quotes the inline annotation body before it is written into an admitted page slot.
    pub fn next_annotation_body_bytes(&self) -> usize { if matches!(self.stage, Stage::Annotation(..)) { size_of::<PdfAnnotation>() } else { 0 } }

    /// ▶️ Advances only measured work. Requires the native graph's borrowed inspection and role semantics.
    pub fn advance(&mut self, trailer: &[PdfDictEntry], version: &str, source: &mut dyn ObjectSource, grant: CosGrant, cancelled: bool) -> ControlledLiftReceipt {
        if cancelled && !matches!(self.stage, Stage::Closing | Stage::Closed) { self.stage = Stage::Cancelled; }
        match self.stage {
            Stage::Ready => quote(Event::Complete),
            Stage::Deferred(reason) => quote(Event::Deferred(reason)),
            Stage::Cancelled => quote(Event::Cancelled),
            Stage::Closing => self.close_step(grant),
            Stage::Closed => quote(Event::Closed),
            _ if grant.work == 0 => quote(Event::More),
            Stage::Open => self.open(version, source, grant),
            Stage::Trailer(index) => self.trailer(trailer, grant, index),
            Stage::Catalog(index) => {
                let mut result = None;
                source.inspect(self.catalog.expect("qualified PDF catalog"), &mut |value| {
                    result = Some(self.catalog_entry(value, grant, index));
                });
                result.unwrap_or_else(|| self.defer(Barrier::Catalog))
            }
            Stage::Info(index) => {
                let Some(owner) = self.info else { self.stage = Stage::PageCapacity; return work(0, 0); };
                let mut turn = None;
                source.inspect(owner, &mut |value| { turn = Some(self.info_entry(value, grant, index)); });
                turn.unwrap_or_else(|| self.defer(Barrier::Info))
            }
            Stage::PageCapacity => self.reserve_pages(source, grant),
            Stage::Walk => self.walk_page(source, grant),
            Stage::BranchResources(attribute) => self.resource_attribute(source, attribute, Resume::Walk),
            Stage::ResourceOwner { owner, resume } => self.resource_owner(source, owner, resume),
            Stage::Font { resources, owner, resume } => self.font(source, grant, resources, owner, resume),
            Stage::Page(index) => {
                let leaf = self.leaf.expect("active PDF page leaf");
                let mut turn = None;
                source.inspect(leaf.reference, &mut |value| { turn = Some(self.page_entry(value, leaf.reference, index)); });
                turn.unwrap_or_else(|| self.defer(Barrier::Page))
            }
            Stage::AnnotationArray(array, index, resume) => self.annotation_array(source, array, index, resume, grant),
            Stage::Annotation(array, index, resume, owner) => {
                let mut receipt = None;
                source.inspect(owner, &mut |value| { receipt = qualified_annotation(value, self.leaf.expect("active PDF page").reference, grant); });
                match receipt {
                    Some(Ok((annotation, bytes))) => {
                        self.page.as_mut().expect("active typed page").annotations.push(annotation);
                        self.stage = Stage::AnnotationArray(array, index + 1, resume);
                        work(bytes, 0)
                    }
                    Some(Err(turn)) => turn,
                    None => self.defer(Barrier::Page),
                }
            }
            Stage::Inherit(index) => self.inherit(source, index),
            Stage::Ancestor { owner, depth } => self.ancestor(source, owner, depth),
            Stage::RoleStart => self.role_start(source),
            Stage::RoleArray { owner, index, count } => self.role_array(source, owner, index, count),
            Stage::RoleStream { stream, next } => {
                let mut canonical = false;
                source.inspect(stream, &mut |value| { canonical = matches!(value, Some(PdfObject::Stream { dict, .. }) if dict.is_empty()); });
                if !canonical { return self.defer(Barrier::Roles); }
                self.stage = Stage::RoleArray { owner: next.owner, index: next.index, count: next.count };
                work(0, 0)
            }
            Stage::Program => self.program(source, grant, false),
            Stage::Mirror => self.program(source, grant, true),
            Stage::Publish => self.publish_page(grant),
            Stage::CloseWalk => self.close_walk(grant, true),
        }
    }

    /// 🫴️ Transfers a complete typed projection. The importer supplies raw graph, trailer and encryption lanes.
    pub fn take_result(&mut self) -> Option<PdfSnapshot> {
        if !matches!(self.stage, Stage::Ready) { return None; }
        self.stage = Stage::Closed;
        self.snapshot.take()
    }

    /// ♻️ Starts retiring active input and planner custody after cancellation or deferral.
    pub fn begin_close(&mut self) {
        if matches!(self.stage, Stage::Closed | Stage::Closing) { return; }
        if let Some(walk) = self.walk.as_mut() { walk.begin_close(); }
        if let Some(program) = self.program.as_mut() { program.begin_close(); }
        self.stage = Stage::Closing;
    }

    /// 🧹️ Settles active cursors before returning the unpublished page and snapshot separately.
    pub fn close_step(&mut self, grant: CosGrant) -> ControlledLiftReceipt {
        if matches!(self.stage, Stage::Closed) { return quote(Event::Closed); }
        if !matches!(self.stage, Stage::Closing) || grant.work == 0 { return quote(Event::More); }
        if let Some(program) = self.program.as_mut() {
            let mut turn = content_turn(program.close_step(grant));
            if turn.event == Ok(Event::Closed) { self.program = None; turn.event = Ok(Event::More); }
            return turn;
        }
        if self.walk.is_some() { return self.close_walk(grant, false); }
        self.stage = Stage::Closed;
        ControlledLiftReceipt { event: Ok(Event::Closed), ..work(0, 0) }
    }

    /// 📦️ Transfers unpublished allocations once the active planner and content cursor have settled.
    pub fn take_partial(&mut self) -> ControlledLiftPartial {
        assert!(matches!(self.stage, Stage::Closed), "active PDF lift must retire its cursors");
        ControlledLiftPartial { snapshot: self.snapshot.take(), page: self.page.take() }
    }

    fn open(&mut self, version: &str, source: &dyn ObjectSource, grant: CosGrant) -> ControlledLiftReceipt {
        if version.len() > MAX_COPY || !source.admitted_roles().is_empty() { return self.defer(Barrier::Trailer); }
        let change = version != "1.7";
        let a = STDIO_PDF17_DOCUMENT_SCHEMA.len() + 3 + if change { version.len() } else { 0 };
        let r = if change { 3 } else { 0 };
        if let Some(denied) = budget(grant, a, r) { return denied; }
        let mut snapshot = PdfSnapshot::default();
        if change { snapshot.declared_version = version.to_owned(); }
        self.snapshot = Some(snapshot);
        self.stage = Stage::Trailer(0);
        work(a, r)
    }

    fn trailer(&mut self, trailer: &[PdfDictEntry], grant: CosGrant, index: usize) -> ControlledLiftReceipt {
        let Some(entry) = trailer.get(index) else {
            if self.catalog.is_none() { return self.defer(Barrier::Trailer); }
            self.stage = Stage::Catalog(0);
            return work(0, 0);
        };
        let mut acquired = 0;
        let flag = match entry.key.as_str() {
            "Root" if self.trailer_seen & 1 == 0 => {
                self.catalog = entry.value.as_ref();
                if self.catalog.is_none() { return self.defer(Barrier::Trailer); }
                1
            }
            "Info" if self.trailer_seen & 2 == 0 => {
                self.info = entry.value.as_ref();
                if self.info.is_none() && entry.value != PdfObject::Null { return self.defer(Barrier::Info); }
                2
            }
            "ID" if self.trailer_seen & 4 == 0 => {
                let Some([PdfObject::Str(a), PdfObject::Str(b)]) = entry.value.as_array() else { return self.defer(Barrier::Trailer); };
                if a.len() > MAX_COPY || b.len() > MAX_COPY { return self.defer(Barrier::Trailer); }
                acquired = a.len() + b.len();
                if let Some(denied) = budget(grant, acquired, 0) { return denied; }
                self.snapshot_mut().document_id = Some([a.clone(), b.clone()]);
                4
            }
            "Root" | "Info" | "ID" | "Encrypt" => return self.defer(Barrier::Trailer),
            _ => 0,
        };
        self.trailer_seen |= flag;
        self.stage = Stage::Trailer(index + 1);
        work(acquired, 0)
    }

    fn catalog_entry(&mut self, value: Option<&PdfObject>, grant: CosGrant, index: usize) -> ControlledLiftReceipt {
        let Some(dict) = small_dict(value) else { return self.defer(Barrier::Catalog); };
        let Some(entry) = dict.get(index) else {
            self.stage = Stage::Info(0);
            return work(0, 0);
        };
        if entry.key.len() > MAX_COPY || dict[..index].iter().any(|other| other.key == entry.key) { return self.defer(Barrier::Catalog); }
        let value = &entry.value;
        let mut turn = work(0, 0);
        match entry.key.as_str() {
            "Type" if value.as_name() == Some("Catalog") => {}
            "Pages" if value.as_ref().is_some() => self.tree = value.as_ref(),
            "PageLayout" => self.snapshot_mut().page_layout = value.as_name().and_then(page_layout_from_name),
            "PageMode" => self.snapshot_mut().page_mode = value.as_name().and_then(page_mode_from_name),
            "Version" | "Lang" => {
                if value.as_ref().is_some() { return self.defer(Barrier::Catalog); }
                let version = entry.key == "Version";
                let authored = if version { value.as_name() } else { as_native_text(value) };
                if let Some(text) = authored {
                    if text.len() > MAX_COPY { return self.defer(Barrier::Catalog); }
                    let snapshot = self.snapshot_mut();
                    if !version || text > snapshot.declared_version.as_str() {
                        let r = if version { snapshot.declared_version.capacity() } else { 0 };
                        if let Some(denied) = budget(grant, text.len(), r) { return denied; }
                        if version { snapshot.declared_version = text.to_owned(); } else { snapshot.language = Some(text.to_owned()); }
                        turn = work(text.len(), r);
                    }
                }
            }
            "MarkInfo" => {
                let Some(flags) = small_dict(Some(value)) else { return self.defer(Barrier::Catalog); };
                let flag = |key| dict_get(flags, key).and_then(PdfObject::as_bool).unwrap_or(false);
                self.snapshot_mut().mark_info = Some(PdfMarkInfo { marked: flag("Marked"), user_properties: flag("UserProperties"), suspects: flag("Suspects") });
            }
            _ => return self.defer(Barrier::Catalog),
        }
        self.stage = Stage::Catalog(index + 1);
        turn
    }

    fn info_entry(&mut self, value: Option<&PdfObject>, grant: CosGrant, index: usize) -> ControlledLiftReceipt {
        let Some(dict) = small_dict(value) else { return self.defer(Barrier::Info); };
        let Some(entry) = dict.get(index) else { self.stage = Stage::PageCapacity; return work(0, 0); };
        if entry.key.len() > MAX_COPY || dict[..index].iter().any(|other| other.key == entry.key) || entry.value.as_ref().is_some() { return self.defer(Barrier::Info); }
        let info = &mut self.snapshot_mut().info;
        let field = match entry.key.as_str() {
            "Title" => &mut info.title, "Author" => &mut info.author, "Subject" => &mut info.subject,
            "Keywords" => &mut info.keywords, "Creator" => &mut info.creator, "Producer" => &mut info.producer,
            "Trapped" => &mut info.trapped,
            "CreationDate" | "ModDate" => {
                if let PdfObject::Date(date) = &entry.value {
                    if entry.key == "CreationDate" { info.creation_date = Some(date.clone()); } else { info.modification_date = Some(date.clone()); }
                }
                self.stage = Stage::Info(index + 1);
                return work(0, 0);
            }
            _ => return self.defer(Barrier::Info),
        };
        let text = if entry.key == "Trapped" { entry.value.as_name() } else { as_native_text(&entry.value) };
        let bytes = text.map_or(0, str::len);
        if bytes > MAX_COPY { return self.defer(Barrier::Info); }
        if let Some(denied) = budget(grant, bytes, 0) { return denied; }
        *field = text.map(str::to_owned);
        self.stage = Stage::Info(index + 1);
        work(bytes, 0)
    }

    fn reserve_pages(&mut self, source: &mut dyn ObjectSource, grant: CosGrant) -> ControlledLiftReceipt {
        let Some(root) = self.tree else {
            self.stage = Stage::Ready;
            return ControlledLiftReceipt { event: Ok(Event::Complete), ..work(0, 0) };
        };
        let mut count = None;
        source.inspect(root, &mut |value| { count = small_dict(value).and_then(|dict| match dict_get(dict, "Count") { Some(PdfObject::Int(n)) => usize::try_from(*n).ok(), _ => None }).filter(|count| *count <= MAX_PAGES); });
        let Some(count) = count else { return self.defer(Barrier::PageTree); };
        let bytes = count * (size_of::<PdfPage>() + size_of::<PdfAdmittedStreamRole>());
        if let Some(denied) = budget(grant, bytes, 0) { return denied; }
        let snapshot = self.snapshot_mut();
        if snapshot.pages.try_reserve_exact(count).is_err() {
            return ControlledLiftReceipt { event: Err("unable to allocate PDF page table"), ..work(0, 0) };
        }
        let pages = snapshot.pages.capacity() * size_of::<PdfPage>();
        if snapshot.admitted_stream_roles.try_reserve_exact(count).is_err() {
            return ControlledLiftReceipt { event: Err("unable to allocate PDF stream role table"), ..work(pages, 0) };
        }
        let held = pages + snapshot.admitted_stream_roles.capacity() * size_of::<PdfAdmittedStreamRole>();
        self.walk = Some(PageTreeCursor::new(root));
        self.stage = Stage::Walk;
        work(held, 0)
    }

    fn walk_page(&mut self, source: &mut dyn ObjectSource, grant: CosGrant) -> ControlledLiftReceipt {
        let walk = self.walk.as_mut().expect("active PDF page tree");
        let turn = walk.advance(source, grant, false);
        match turn.event {
            Ok(PageTreeEvent::Page(leaf)) => {
                if self.snapshot_mut().pages.len() == self.snapshot_mut().pages.capacity() { return self.defer_turn(Barrier::PageTree, tree_turn(turn)); }
                self.leaf = Some(leaf);
                self.page = Some(PdfPage::default());
                self.contents = None;
                self.indirect_contents = false;
                self.page_font = None;
                self.owners_len = 0;
                self.stage = Stage::Page(0);
            }
            Ok(PageTreeEvent::BranchResources(attribute)) => self.stage = Stage::BranchResources(attribute),
            Ok(PageTreeEvent::Complete) => { walk.begin_close(); self.stage = Stage::CloseWalk; }
            Err(_) => return self.defer_turn(Barrier::PageTree, tree_turn(turn)),
            _ => {}
        }
        tree_turn(turn)
    }

    fn close_walk(&mut self, grant: CosGrant, completing: bool) -> ControlledLiftReceipt {
        let turn = self.walk.as_mut().expect("closing PDF page tree").close_step(grant);
        let closed = turn.event == Ok(PageTreeEvent::Closed);
        let mut turn = tree_turn(turn);
        if closed {
            self.walk = None;
            if completing { self.stage = Stage::Ready; turn.event = Ok(Event::Complete); }
        }
        turn
    }

    fn page_entry(&mut self, value: Option<&PdfObject>, owner: ObjRef, index: usize) -> ControlledLiftReceipt {
        let Some(dict) = small_dict(value) else { return self.defer(Barrier::Page); };
        let Some(entry) = dict.get(index) else { self.stage = Stage::Inherit(0); return work(0, 0); };
        if entry.key.len() > MAX_COPY || dict[..index].iter().any(|other| other.key == entry.key) { return self.defer(Barrier::Page); }
        let value = &entry.value;
        let page = self.page.as_mut().expect("active typed page");
        match entry.key.as_str() {
            "Type" if value.as_name() == Some("Page") => {}
            "Parent" if value.as_ref().is_some() => {}
            "Contents" => self.contents = Some(PageAttribute { owner, entry: index }),
            "MediaBox" | "CropBox" | "Rotate" | "Resources" => {}
            "BleedBox" | "TrimBox" | "ArtBox" => {
                let Some(rect) = direct_rect(value) else { return self.defer(Barrier::Page); };
                match entry.key.as_str() {
                    "BleedBox" => page.bleed_box = Some(rect),
                    "TrimBox" => page.trim_box = Some(rect),
                    _ => page.art_box = Some(rect),
                }
            }
            "UserUnit" | "StructParents" | "Dur" => {
                let Some(number) = geometry_number(value) else { return self.defer(Barrier::Page); };
                match entry.key.as_str() {
                    "UserUnit" => page.user_unit = Some(number),
                    "StructParents" => page.struct_parents = Some(bounded_integer(value).expect("qualified number") as u32),
                    _ => page.duration = Some(number),
                }
            }
            "Annots" if matches!(value, PdfObject::Null) => {}
            "Annots" if value.as_array().is_some() || value.as_ref().is_some() => {
                let array = value.as_ref().map_or(AnnotationArray::Direct(PageAttribute { owner, entry: index }), AnnotationArray::Indirect);
                self.stage = Stage::AnnotationArray(array, 0, index + 1);
                return work(0, 0);
            }
            "AA" if empty_dictionary(Some(value)) => {}
            "Group" => match simple_group(value) {
                Some(group) => page.group = Some(group),
                None => return self.defer(Barrier::Page),
            }
            _ => return self.defer(Barrier::Page),
        }
        self.stage = Stage::Page(index + 1);
        work(0, 0)
    }

    fn annotation_array(&mut self, source: &mut dyn ObjectSource, array: AnnotationArray, index: usize, resume: usize, grant: CosGrant) -> ControlledLiftReceipt {
        let mut item = None;
        let mut read = |value: Option<&PdfObject>| {
            item = value.and_then(PdfObject::as_array).filter(|items| items.len() <= MAX_ANNOTATIONS)
                .map(|items| (items.len(), items.get(index).and_then(PdfObject::as_ref)));
        };
        match array {
            AnnotationArray::Direct(attribute) => inspect_attribute(source, attribute, &mut read),
            AnnotationArray::Indirect(owner) => source.inspect(owner, &mut read),
        }
        let Some((count, owner)) = item else { return self.defer(Barrier::Page); };
        if index == count { self.stage = Stage::Page(resume); return work(0, 0); }
        let Some(owner) = owner else { return self.defer(Barrier::Page); };
        let annotations = &mut self.page.as_mut().expect("active typed page").annotations;
        let mut bytes = 0;
        if index == 0 {
            if let Some(turn) = budget(grant, count * size_of::<PdfAnnotation>(), 0) { return turn; }
            if annotations.try_reserve_exact(count).is_err() {
                return ControlledLiftReceipt { event: Err("unable to allocate PDF annotation table"), ..work(0, 0) };
            }
            bytes = annotations.capacity() * size_of::<PdfAnnotation>();
        }
        self.stage = Stage::Annotation(array, index, resume, owner);
        work(bytes, 0)
    }

    fn inherit(&mut self, source: &mut dyn ObjectSource, index: usize) -> ControlledLiftReceipt {
        let leaf = self.leaf.expect("qualified PDF page");
        let inherited = leaf.inheritance;
        let attributes = [inherited.media_box, inherited.crop_box, inherited.rotate];
        if index == attributes.len() {
            if let Some(attribute) = inherited.resources { return self.resource_attribute(source, attribute, Resume::Inherit); }
            self.resume(Resume::Inherit);
            return work(0, 0);
        }
        let mut valid = true;
        if let Some(attribute) = attributes[index] {
            let page = self.page.as_mut().expect("active typed page");
            inspect_attribute(source, attribute, &mut |value| {
                let Some(value) = value else { valid = false; return; };
                if index == 2 {
                    let rotation = bounded_integer(value);
                    valid = rotation.is_some();
                    page.rotate = rotation.unwrap_or_default() as i32;
                }
                else if let Some(rect) = direct_rect(value) {
                    if index == 0 { page.media_box = rect; } else { page.crop_box = Some(rect); }
                } else { valid = false; }
            });
        }
        if !valid { return self.defer(Barrier::Page); }
        self.stage = Stage::Inherit(index + 1);
        work(0, 0)
    }

    fn resource_attribute(&mut self, source: &mut dyn ObjectSource, attribute: PageAttribute, resume: Resume) -> ControlledLiftReceipt {
        let mut inspection = None;
        inspect_attribute(source, attribute, &mut |value| {
            inspection = Some((empty_dictionary(value), value.and_then(PdfObject::as_ref)));
        });
        match inspection {
            Some((true, _)) => self.resume(resume),
            Some((false, Some(owner))) => self.stage = Stage::ResourceOwner { owner, resume },
            _ => return self.defer(Barrier::Resources),
        }
        work(0, 0)
    }

    fn resource_owner(&mut self, source: &mut dyn ObjectSource, owner: ObjRef, resume: Resume) -> ControlledLiftReceipt {
        let mut font = None;
        let mut empty = false;
        source.inspect(owner, &mut |value| {
            empty = empty_dictionary(value);
            font = font_entry(value).and_then(|entry| entry.value.as_ref());
        });
        if resume == Resume::Role {
            if self.page_font.map(|(resources, _)| resources) != font.map(|_| owner) || !empty && font.is_none() {
                return self.defer(Barrier::Resources);
            }
        } else if let Some(font) = font {
            if resume == Resume::Walk && !self.snapshot_mut().pages.is_empty() { return self.defer(Barrier::Resources); }
            self.stage = Stage::Font { resources: owner, owner: font, resume };
            return work(0, 0);
        } else if !empty { return self.defer(Barrier::Resources); }
        self.resume(resume);
        work(0, 0)
    }

    fn font(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, resources: ObjRef, owner: ObjRef, resume: Resume) -> ControlledLiftReceipt {
        let mut name = [0; MAX_COPY];
        let mut length = None;
        source.inspect(resources, &mut |value| {
            let Some(entry) = font_entry(value) else { return; };
            let n = entry.key.len();
            if n != 0 && n <= name.len() {
                name[..n].copy_from_slice(entry.key.as_bytes());
                length = Some(n);
            }
        });
        let Some(length) = length else { return self.defer(Barrier::Resources); };
        let name = std::str::from_utf8(&name[..length]).expect("authored PDF font name");
        let count = self.snapshot_mut().fonts.len();
        let mut acquired = 0;
        let index = if let Some(index) = self.font_owners[..count].iter().position(|reference| *reference == owner) {
            if self.snapshot_mut().fonts[index].id != name { return self.defer(Barrier::Resources); }
            index
        } else {
            if count == MAX_FONTS || self.snapshot_mut().fonts.iter().any(|font| font.id == name) {
                return self.defer(Barrier::Resources);
            }
            let mut shape = None;
            source.inspect(owner, &mut |value| { shape = standard_type1(value); });
            let Some((base_font, base)) = shape else { return self.defer(Barrier::Resources); };
            acquired = length + base_font.len() + if count == 0 { MAX_FONTS * size_of::<PdfFont>() } else { 0 };
            if let Some(denied) = budget(grant, acquired, 0) { return denied; }
            let fonts = &mut self.snapshot_mut().fonts;
            if count == 0 { *fonts = Vec::with_capacity(MAX_FONTS); }
            fonts.push(PdfFont {
                id: name.to_owned(),
                kind: PdfFontKind::Type1 {
                    base_font: base_font.to_owned(), encoding: PdfSimpleEncoding { base, differences: Vec::new() },
                    first_char: 0, widths: Vec::new(), descriptor: None, program: None,
                },
                to_unicode: None, extra: Vec::new(),
            });
            self.font_owners[count] = owner;
            count
        };
        if resume == Resume::Inherit { self.page_font = Some((resources, index)); }
        self.resume(resume);
        work(acquired, 0)
    }

    fn resume(&mut self, resume: Resume) {
        self.stage = match resume {
            Resume::Walk => Stage::Walk,
            Resume::Inherit => {
                let leaf = self.leaf.expect("active PDF page");
                Stage::Ancestor { owner: leaf.reference, depth: 0 }
            }
            Resume::Role => Stage::RoleStart,
        };
    }

    fn ancestor(&mut self, source: &mut dyn ObjectSource, owner: ObjRef, depth: usize) -> ControlledLiftReceipt {
        if self.contents.is_none() { self.stage = Stage::Program; return work(0, 0); }
        if depth >= 64 { return self.defer(Barrier::Roles); }
        let mut step = None;
        source.inspect(owner, &mut |value| {
            let Some(dict) = small_dict(value) else { return; };
            step = match dict_get(dict, "Resources") {
                Some(value) if empty_dictionary(Some(value)) && self.page_font.is_none() => Some(Stage::RoleStart),
                Some(value) => value.as_ref().map(|owner| Stage::ResourceOwner { owner, resume: Resume::Role }),
                None => dict_get(dict, "Parent").and_then(PdfObject::as_ref).map(|owner| Stage::Ancestor { owner, depth: depth + 1 }).or_else(|| self.page_font.is_none().then_some(Stage::RoleStart)),
            };
        });
        let Some(stage) = step else { return self.defer(Barrier::Resources); };
        self.stage = stage;
        work(0, 0)
    }

    fn role_start(&mut self, source: &mut dyn ObjectSource) -> ControlledLiftReceipt {
        let mut shape = None;
        inspect_attribute(source, self.contents.expect("active PDF Contents"), &mut |value| {
            shape = match value {
                Some(PdfObject::Ref(owner)) => Some((Some(*owner), None)),
                Some(PdfObject::Array(items)) => Some((None, Some(items.len()))),
                Some(PdfObject::Null) => Some((None, None)),
                _ => None,
            };
        });
        self.stage = match shape {
            Some((Some(owner), _)) => {
                self.push_owner(owner);
                self.indirect_contents = true;
                Stage::RoleArray { owner: Some(owner), index: 0, count: usize::MAX }
            }
            Some((None, Some(count))) => Stage::RoleArray { owner: None, index: 0, count },
            Some((None, None)) => Stage::Program,
            _ => return self.defer(Barrier::Roles),
        };
        work(0, 0)
    }

    fn role_array(&mut self, source: &mut dyn ObjectSource, owner: Option<ObjRef>, index: usize, count: usize) -> ControlledLiftReceipt {
        let mut shape = None;
        let mut inspect = |value: Option<&PdfObject>| {
            shape = match value {
                Some(PdfObject::Stream { dict, .. }) if count == usize::MAX && dict.is_empty() => Some((0, None)),
                Some(PdfObject::Array(items)) if items.len() <= MAX_ROLE_OWNERS => {
                    Some((items.len(), items.get(index).and_then(PdfObject::as_ref)))
                }
                _ => None,
            };
        };
        if let Some(owner) = owner { source.inspect(owner, &mut inspect); }
        else { inspect_attribute(source, self.contents.expect("direct PDF Contents"), &mut inspect); }
        let Some((count, reference)) = shape else { return self.defer(Barrier::Roles); };
        if index == count { self.stage = Stage::Program; return work(0, 0); }
        let Some(stream) = reference else { return self.defer(Barrier::Roles); };
        if !self.push_owner(stream) { return self.defer(Barrier::Roles); }
        self.stage = Stage::RoleStream { stream, next: RoleNext { owner, index: index + 1, count } };
        work(0, 0)
    }

    fn push_owner(&mut self, owner: ObjRef) -> bool {
        if self.owners[..self.owners_len].contains(&owner) { return true; }
        let Some(slot) = self.owners.get_mut(self.owners_len) else { return false; };
        *slot = owner;
        self.owners_len += 1;
        true
    }

    fn program(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, mirror: bool) -> ControlledLiftReceipt {
        let owner = self.leaf.expect("active PDF page").reference;
        if self.program.is_none() {
            self.program = Some(PageProgramCursor::new(owner));
            return work(0, 0);
        }
        let program = self.program.as_mut().expect("active PDF page program");
        let turn = if let Some((_, index)) = self.page_font {
            let font = &self.snapshot.as_ref().expect("active typed PDF snapshot").fonts[index];
            program.advance_with_fonts(source, &Type1Binding(font), grant, false)
        } else {
            program.advance(source, grant, false)
        };
        match turn.event {
            Ok(PageContentEvent::Complete) if mirror => self.stage = Stage::Publish,
            Ok(PageContentEvent::Complete) => {
                self.page.as_mut().expect("active typed page").content = program.take_program().expect("ready PDF operators");
                self.program = None;
                self.stage = if self.contents.is_some() { Stage::Mirror } else { Stage::Publish };
            }
            Ok(PageContentEvent::Deferred(_)) | Err(_) => return self.defer_turn(Barrier::Program, content_turn(turn)),
            _ => {}
        }
        content_turn(turn)
    }

    fn publish_page(&mut self, grant: CosGrant) -> ControlledLiftReceipt {
        let mut acquired = 0;
        if let Some(contents) = self.contents {
            let count = self.snapshot_mut().fonts.len();
            let mut fonts = self.font_owners;
            fonts[..count].sort_unstable();
            let context = self.page_font.map(|(resources, index)| (resources, self.font_owners[index]));
            let capacity = self.owners_len + 1 + count + usize::from(context.is_some());
            acquired = 2 * (size_of::<PdfGraphPath>() + "Contents".len()) + capacity * size_of::<PdfGraphIdentity>()
                + context.map_or(0, |_| size_of::<PdfGraphPath>() + "Font".len());
            if let Some(denied) = budget(grant, acquired, 0) { return denied; }
            let identity = PdfGraphIdentity { owner: contents.owner, path: vec![PdfGraphPath::Entry { key: "Contents".into() }] };
            let mut dependencies = Vec::with_capacity(capacity);
            dependencies.push(identity.clone());
            if let Some((resources, _)) = context {
                dependencies.push(PdfGraphIdentity { owner: resources, path: vec![PdfGraphPath::Entry { key: "Font".into() }] });
            }
            let (initial, remaining) = self.owners[..self.owners_len].split_at(if self.indirect_contents { 1 } else { self.owners_len });
            dependencies.extend(initial.iter().copied().map(root_identity));
            if let Some((_, owner)) = context { dependencies.push(root_identity(owner)); }
            dependencies.extend(remaining.iter().copied().map(root_identity));
            dependencies.extend(fonts[..count].iter().copied().filter(|owner| context.is_none_or(|(_, active)| *owner != active)).map(root_identity));
            let content = self.program.as_mut().and_then(PageProgramCursor::take_program).expect("ready admitted PDF operators");
            self.program = None;
            self.snapshot_mut().admitted_stream_roles.push(PdfAdmittedStreamRole { identity, dependencies, value: PdfStreamRoleValue::Operators { content } });
        }
        let page = self.page.take().expect("qualified projected page");
        self.snapshot_mut().pages.push(page);
        self.stage = Stage::Walk;
        work(acquired, 0)
    }

    fn snapshot_mut(&mut self) -> &mut PdfSnapshot { self.snapshot.as_mut().expect("active typed PDF snapshot") }

    fn defer(&mut self, reason: Barrier) -> ControlledLiftReceipt { self.defer_turn(reason, work(0, 0)) }

    fn defer_turn(&mut self, reason: Barrier, turn: ControlledLiftReceipt) -> ControlledLiftReceipt {
        self.stage = Stage::Deferred(reason);
        ControlledLiftReceipt { event: Ok(Event::Deferred(reason)), ..turn }
    }
}

struct Type1Binding<'a>(&'a PdfFont);

impl FontTable for Type1Binding<'_> {
    fn font(&self, _: &str) -> Option<&FontCodec> { None }

    fn controlled_type1(&self, name: &str) -> Option<PdfBaseEncoding> {
        let PdfFontKind::Type1 { encoding, .. } = &self.0.kind else { return None; };
        (self.0.id == name).then_some(encoding.base.unwrap_or(PdfBaseEncoding::Standard))
    }
}

fn font_entry(value: Option<&PdfObject>) -> Option<&PdfDictEntry> {
    let PdfObject::Dict(resources) = value? else { return None; };
    let [font] = resources.as_slice() else { return None; };
    if font.key != "Font" { return None; }
    let PdfObject::Dict(fonts) = &font.value else { return None; };
    let [entry] = fonts.as_slice() else { return None; };
    Some(entry)
}

fn standard_type1(value: Option<&PdfObject>) -> Option<(&'static str, Option<PdfBaseEncoding>)> {
    let PdfObject::Dict(dict) = value? else { return None; };
    if dict.len() > 4 || dict.iter().enumerate().any(|(index, entry)| {
        !matches!(entry.key.as_str(), "Type" | "Subtype" | "BaseFont" | "Encoding")
            || dict[..index].iter().any(|other| other.key == entry.key)
    }) { return None; }
    if dict_name(dict, "Type") != Some("Font") || dict_name(dict, "Subtype") != Some("Type1") { return None; }
    let base_font = dict_name(dict, "BaseFont")?;
    let font = standard_font(base_font)?;
    if font.symbolic || font.name != base_font { return None; }
    let encoding = match dict_get(dict, "Encoding") {
        None => None,
        Some(value) => Some(match value.as_name()? {
            "StandardEncoding" => PdfBaseEncoding::Standard,
            "WinAnsiEncoding" => PdfBaseEncoding::WinAnsi,
            _ => return None,
        }),
    };
    Some((font.name, encoding))
}

fn root_identity(owner: ObjRef) -> PdfGraphIdentity { PdfGraphIdentity { owner, path: Vec::new() } }

fn quote(event: Event) -> ControlledLiftReceipt { ControlledLiftReceipt { event: Ok(event), work: 0, allocated_bytes: 0, released_bytes: 0 } }
fn work(allocated_bytes: usize, released_bytes: usize) -> ControlledLiftReceipt { ControlledLiftReceipt { work: 1, allocated_bytes, released_bytes, ..quote(Event::More) } }

fn budget(grant: CosGrant, allocation: usize, release: usize) -> Option<ControlledLiftReceipt> {
    if allocation > grant.allocation_bytes { Some(quote(Event::NeedsAllocation { bytes: allocation })) }
    else if release > grant.release_bytes { Some(quote(Event::NeedsRelease { bytes: release })) }
    else { None }
}

fn tree_turn(turn: PageTreeReceipt) -> ControlledLiftReceipt {
    let event = turn.event.map(|event| match event {
        PageTreeEvent::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
        PageTreeEvent::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
        _ => Event::More,
    });
    ControlledLiftReceipt { event, work: turn.work, allocated_bytes: turn.allocated_bytes, released_bytes: turn.released_bytes }
}

fn content_turn(turn: PageContentReceipt) -> ControlledLiftReceipt {
    let event = turn.event.map(|event| match event {
        PageContentEvent::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
        PageContentEvent::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
        PageContentEvent::Closed => Event::Closed,
        _ => Event::More,
    });
    ControlledLiftReceipt { event, work: turn.work, allocated_bytes: turn.allocated_bytes, released_bytes: turn.released_bytes }
}

fn small_dict(value: Option<&PdfObject>) -> Option<&[PdfDictEntry]> {
    value?.as_dict().filter(|dict| dict.len() <= MAX_DICTIONARY)
}

fn empty_dictionary(value: Option<&PdfObject>) -> bool {
    matches!(value, Some(PdfObject::Null)) || matches!(value, Some(PdfObject::Dict(dict)) if dict.is_empty())
}

fn inspect_attribute(source: &mut dyn ObjectSource, attribute: PageAttribute, read: &mut dyn FnMut(Option<&PdfObject>)) {
    source.inspect(attribute.owner, &mut |value| {
        read(value.and_then(PdfObject::as_dict).and_then(|dict| dict.get(attribute.entry)).map(|entry| &entry.value));
    });
}

fn direct_rect(value: &PdfObject) -> Option<PdfRect> {
    let [a, b, c, d] = value.as_array()? else { return None; };
    Some([geometry_number(a)?, geometry_number(b)?, geometry_number(c)?, geometry_number(d)?])
}

fn geometry_number(value: &PdfObject) -> Option<f64> { bounded_number(value).ok().flatten() }

fn bounded_integer(value: &PdfObject) -> Option<i64> {
    if let PdfObject::Int(value) = value { Some(*value) } else { geometry_number(value).map(|number| number as i64) }
}

fn as_native_text(value: &PdfObject) -> Option<&str> {
    match value { PdfObject::Name(name) | PdfObject::Text(name) => Some(name), _ => None }
}

fn qualified_annotation(value: Option<&PdfObject>, page: ObjRef, grant: CosGrant) -> Option<Result<(PdfAnnotation, usize), ControlledLiftReceipt>> {
    let dict = small_dict(value)?;
    let text = dict_name(dict, "Subtype")? == "Text";
    if !text && dict_name(dict, "Subtype") != Some("Link") { return None; }
    let rect = direct_rect(dict_get(dict, "Rect")?)?;
    let (mut open, mut flags, mut border, mut color, mut action) = (false, 0, None, None, None);
    let mut strings = [None; 8];
    for (index, entry) in dict.iter().enumerate() {
        if entry.key.len() > MAX_COPY || dict[..index].iter().any(|other| other.key == entry.key) { return None; }
        let value = &entry.value;
        let slot = match entry.key.as_str() {
            "Contents" => (0, as_native_text(value)?), "NM" => (1, as_native_text(value)?),
            "Name" if text => (2, value.as_name()?), "State" if text => (3, as_native_text(value)?),
            "StateModel" if text => (4, as_native_text(value)?), "T" if text => (5, as_native_text(value)?),
            "Subj" if text => (6, as_native_text(value)?), "H" if !text => (7, value.as_name()?),
            key => {
                match key {
                    "Type" if value.as_name() == Some("Annot") => {}
                    "Subtype" | "Rect" => {}
                    "P" if value.as_ref() == Some(page) => {}
                    "Open" if text => open = value.as_bool()?,
                    "F" => { let PdfObject::Int(raw) = value else { return None; }; flags = u32::try_from(*raw).ok()?; }
                    "Border" => {
                        let [x, y, width] = value.as_array()? else { return None; };
                        border = Some(PdfBorderStyle { width: geometry_number(width)?, style: None, dash: None, radii: Some([geometry_number(x)?, geometry_number(y)?]) });
                    }
                    "C" => {
                        let values = value.as_array()?;
                        if !matches!(values.len(), 0 | 1 | 3 | 4) { return None; }
                        let mut numbers = [0.0; 4];
                        for (number, raw) in numbers.iter_mut().zip(values) { *number = geometry_number(raw)?; }
                        color = Some((numbers, values.len()));
                    }
                    "A" if !text => action = Some(qualified_uri(value)?),
                    _ => return None,
                }
                continue;
            }
        };
        if slot.1.len() > MAX_COPY { return None; }
        strings[slot.0] = Some(slot.1);
    }
    if !text && action.is_none() { return None; }
    let bytes = strings.iter().flatten().map(|text| text.len()).sum::<usize>()
        + color.as_ref().map_or(0, |(_, len)| len * size_of::<f64>()) + action.as_ref().map_or(0, |(uri, _)| uri.len());
    if let Some(turn) = budget(grant, bytes, 0) { return Some(Err(turn)); }
    let [contents, name, icon, state, state_model, title, subject, highlight] = strings.map(|text| text.map(str::to_owned));
    let kind = if text { PdfAnnotationKind::Text { open, icon, state, state_model } } else {
        let action = action.map(|(uri, is_map)| PdfAction { kind: PdfActionKind::Uri { uri: uri.to_owned(), is_map }, next: Vec::new() });
        PdfAnnotationKind::Link { action, destination: None, highlight, quad_points: Vec::new() }
    };
    let markup = (title.is_some() || subject.is_some()).then_some(PdfMarkupAnnotation { title, subject, ..Default::default() });
    Some(Ok((PdfAnnotation { contents, name, flags, border, markup,
        color: color.map_or_else(Vec::new, |(numbers, len)| numbers[..len].to_vec()), ..PdfAnnotation::new(rect, kind)
    }, bytes)))
}

fn qualified_uri(value: &PdfObject) -> Option<(&str, bool)> {
    let dict = small_dict(Some(value))?;
    if dict.len() > 4 || dict_name(dict, "S") != Some("URI") || dict.iter().enumerate().any(|(index, entry)| {
        !matches!(entry.key.as_str(), "Type" | "S" | "URI" | "IsMap") || dict[..index].iter().any(|other| other.key == entry.key)
    }) { return None; }
    if dict_get(dict, "Type").is_some_and(|value| value.as_name() != Some("Action")) { return None; }
    let uri = as_native_text(dict_get(dict, "URI")?)?;
    let is_map = match dict_get(dict, "IsMap") { Some(value) => value.as_bool()?, None => false };
    (uri.len() <= MAX_COPY).then_some((uri, is_map))
}

fn simple_group(value: &PdfObject) -> Option<PdfTransparencyGroup> {
    let dict = small_dict(Some(value))?;
    let color_space = match dict_get(dict, "CS").map(PdfObject::as_name) {
        None => None,
        Some(Some("DeviceGray")) => Some(PdfColorSpace::DeviceGray),
        Some(Some("DeviceRGB")) => Some(PdfColorSpace::DeviceRgb),
        Some(Some("DeviceCMYK")) => Some(PdfColorSpace::DeviceCmyk),
        _ => return None,
    };
    Some(PdfTransparencyGroup { color_space, isolated: dict_get(dict, "I").and_then(PdfObject::as_bool).unwrap_or(false), knockout: dict_get(dict, "K").and_then(PdfObject::as_bool).unwrap_or(false) })
}
