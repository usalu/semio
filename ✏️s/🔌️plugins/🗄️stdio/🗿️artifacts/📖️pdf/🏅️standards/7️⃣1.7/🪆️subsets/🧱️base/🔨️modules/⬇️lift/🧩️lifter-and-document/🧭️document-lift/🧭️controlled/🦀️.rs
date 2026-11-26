//! 🧭️ Projects bounded native PDF pages with vector content, standard Type1 text, sampled images and simple annotations.

use super::*;
use crate::standards::v1_7::subsets::base::modules::{
    colour::{extra_entries, lift_ext_g_state}, content::{bounded_number, content_extent, FontTable, PdfFixedCodeWidth}, fonts::{controlled_simple_text, ControlledFontResources, FontCodec},
    images::{ControlledImageBinding, ControlledImageResources}, lexer::{CosReceipt, CosStatus, grow_table, table_growth},
};
use crate::standards::v1_7::subsets::base::schema::snapshot::{PdfBaseEncoding, PdfCharProc, PdfContent, PdfExtGState, PdfFont, PdfFontKind, PdfFontProgram, PdfFormXObject, PdfNamedProperties};
use crate::standards::v1_7::subsets::base::io::text_string::decode_text_string;
use crate::standards::v1_7::subsets::base::schema::content_mapping::{rename_target, ResourceMap};
use crate::standards::v1_7::subsets::base::modules::lift::{Category as LiftCategory, FontLift, FontStart};
use std::collections::HashMap;
use std::mem::{size_of, take};

const MAX_DICTIONARY: usize = 32;
const MAX_COPY: usize = 128;
const MAX_ROLE_OWNERS: usize = 32;
const MAX_ANNOTATIONS: usize = 4096;
const MAX_PAGES: usize = 16_384;
/// ♻️ The most one release unit credits: a sixteenth of the interactive component's 1 MiB per-turn release grant, so a unit still fits a turn that already released other backing.
const RELEASE_UNIT: usize = 64 * 1024;
const MAX_TREE_DEPTH: usize = 34;
const OUTLINE_DEPTH: usize = 32;
const NAVIGATION_ROLES: usize = 1;
/// 🏁️ The annotation array resume marker of a page whose annotations are lifted after its content, in the reference lifter's order.
const AFTER_CONTENT: usize = usize::MAX;
const FONT_HEAD_ROLES: usize = 6;
const INFO_KEYS: [&str; 9] = ["Title", "Author", "Subject", "Keywords", "Creator", "Producer", "CreationDate", "ModDate", "Trapped"];

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
    Open, Trailer(usize), Catalog(usize), Info(usize), OptionalContent, PageCapacity, Prewalk, ClosePrewalk, Walk,
    Resources { location: ResourceLocation, resume: Resume },
    Entries { location: ResourceLocation, category: Category, index: usize, resume: Resume },
    Image { location: ResourceLocation, index: usize, resume: Resume },
    State { location: ResourceLocation, category: Category, index: usize, owner: ObjRef, resume: Resume },
    Font { location: ResourceLocation, index: usize, owner: ObjRef, resume: Resume },
    ColdFont { location: ResourceLocation, index: usize, owner: ObjRef, resume: Resume },
    ColdGlyph { location: ResourceLocation, index: usize, owner: ObjRef, resume: Resume },
    ColdFinish { location: ResourceLocation, index: usize, owner: ObjRef, resume: Resume },
    ColdForm { location: ResourceLocation, index: usize, owner: ObjRef, resume: Resume },
    ColdState { location: ResourceLocation, index: usize, resume: Resume },
    ColdResource { location: ResourceLocation, category: Category, index: usize, resume: Resume },
    InfoValue { index: usize, field: InfoField, target: ObjRef },
    Page(usize), PageGroup { owner: ObjRef, resume: usize }, Inherit(usize), Ancestor { owner: ObjRef, depth: usize },
    AnnotationArray(AnnotationArray, usize, usize), Annotation(AnnotationArray, usize, usize, ObjRef),
    RoleStart, RoleArray { owner: Option<ObjRef>, index: usize, count: usize }, Rename(usize),
    RoleStream { stream: ObjRef, next: RoleNext }, Program, Mirror(usize), Publish,
    CloseWalk, Navigation(Lane), Ready, Deferred(ControlledLiftBarrier), Cancelled, Closing, Closed,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum InfoField { Title, Author, Subject, Keywords, Creator, Producer, CreationDate, ModDate }

impl InfoField {
    fn of(key: &str) -> Option<Self> {
        Some(match key {
            "Title" => Self::Title, "Author" => Self::Author, "Subject" => Self::Subject, "Keywords" => Self::Keywords,
            "Creator" => Self::Creator, "Producer" => Self::Producer, "CreationDate" => Self::CreationDate, "ModDate" => Self::ModDate,
            _ => return None,
        })
    }
}

/// 🧭️ The document lanes lifted after every page, in the reference lifter's catalog order.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Lane { Outlines, Names, Dests, Labels, Form, Preferences, OpenAction, Metadata, Extra, Release }

/// 🪢️ A tree node: an indirect object, or a direct dictionary nested under at most two keys of an indirect owner.
#[derive(Clone, Copy)]
enum Node { Object(ObjRef), Entry { owner: ObjRef, outer: Option<TreeKey>, key: TreeKey } }

/// 🗝️ The catalog keys a direct tree root may sit under.
#[derive(Clone, Copy)]
enum TreeKey { Names, Dests, PageLabels, AcroForm }

impl TreeKey {
    fn name(self) -> &'static str {
        match self { Self::Names => "Names", Self::Dests => "Dests", Self::PageLabels => "PageLabels", Self::AcroForm => "AcroForm" }
    }
}

impl Node {
    fn inspect(self, source: &mut dyn ObjectSource, read: &mut dyn FnMut(Option<&PdfObject>)) {
        match self {
            Self::Object(owner) => source.inspect(owner, read),
            Self::Entry { owner, outer, key } => source.inspect(owner, &mut |value| {
                let node = match outer { Some(outer) => value.and_then(|value| value.dict_get(outer.name())), None => value };
                read(node.and_then(|node| node.dict_get(key.name())));
            }),
        }
    }

    fn within(owner: ObjRef, outer: Option<TreeKey>, key: TreeKey, value: &PdfObject) -> Option<Self> {
        match value {
            PdfObject::Ref(target) => Some(Self::Object(*target)),
            PdfObject::Dict(_) => Some(Self::Entry { owner, outer, key }),
            _ => None,
        }
    }
}

/// 🪜️ One level of a resumable outline, name tree or number tree walk.
#[derive(Clone, Copy)]
struct Frame { node: Option<Node>, kid: u32, pair: u32, names: bool, field_type: u8 }

impl Frame {
    fn at(node: Option<Node>) -> Self { Self { node, kid: 0, pair: 0, names: false, field_type: 0 } }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Resume { Walk, Inherit, Role }

/// 📝️ The steps of the interactive form lane: its default resources, its own entries, its field tree.
#[derive(Clone, Copy, PartialEq, Eq)]
enum FormStep { Start, Head, Fields }
#[derive(Clone, Copy, PartialEq, Eq)]
enum ResourceLocation { Direct(PageAttribute), Indirect(ObjRef), FormDefaults(ObjRef) }

#[derive(Clone, Copy, PartialEq, Eq)]
#[derive(semio_framework_value::RetireOwned)]
enum Category { Font, XObject, ExtGState, ColorSpace, Shading, Pattern, Properties }

impl Category {
    fn key(self) -> &'static str {
        match self { Self::Font => "Font", Self::XObject => "XObject", Self::ExtGState => "ExtGState", Self::ColorSpace => "ColorSpace", Self::Shading => "Shading", Self::Pattern => "Pattern", Self::Properties => "Properties" }
    }

    /// 🎨️ Whether the reference builder lifts the category's entries one by one (colour spaces, shadings, patterns).
    fn is_cold(self) -> bool { matches!(self, Self::ColorSpace | Self::Shading | Self::Pattern) }

    fn lifted(self) -> LiftCategory {
        match self {
            Self::Font => LiftCategory::Font, Self::XObject => LiftCategory::XObject, Self::ExtGState => LiftCategory::ExtGState, Self::ColorSpace => LiftCategory::ColorSpace,
            Self::Shading => LiftCategory::Shading, Self::Pattern => LiftCategory::Pattern, Self::Properties => LiftCategory::Properties,
        }
    }
}

enum Entry { End, Item { name: [u8; MAX_COPY], length: usize, owner: ObjRef }, Direct }

/// 🎰️ The font or form owner slots one page's resources bind, in dictionary order: the table grows by a quoted unit before it fills and keeps its capacity from page to page until the lift releases it, so a page may bind any number of resources.
#[derive(Default)]
struct PageSlots { slots: Vec<u16> }

impl PageSlots {
    /// 📥️ Grows the table when it is full; a receipt means this unit was spent on the growth.
    fn room(&mut self, grant: CosGrant) -> Option<ControlledLiftReceipt> {
        let bytes = table_growth(&self.slots, 1);
        if bytes == 0 { return None; }
        if let Some(denied) = budget(grant, bytes, 0) { return Some(denied); }
        if !grow_table(&mut self.slots, 1) { return Some(ControlledLiftReceipt { event: Err("unable to grow the PDF page slot table"), ..work(0, 0) }); }
        Some(work(bytes, 0))
    }

    /// 🎯️ Binds one more owner slot into the room [`Self::room`] made; false when the slot does not fit a table entry.
    fn push(&mut self, slot: usize) -> bool {
        let Ok(slot) = u16::try_from(slot) else { return false; };
        if self.slots.len() == self.slots.capacity() { return false; }
        self.slots.push(slot);
        true
    }

    fn as_slice(&self) -> &[u16] { &self.slots }

    fn held_bytes(&self) -> usize { self.slots.capacity() * size_of::<u16>() }
}

impl ResourceLocation {
    fn inspect(self, source: &mut dyn ObjectSource, read: &mut dyn FnMut(Option<&PdfObject>)) {
        match self {
            Self::Direct(attribute) => inspect_attribute(source, attribute, read),
            Self::Indirect(owner) => source.inspect(owner, read),
            Self::FormDefaults(catalog) => source.inspect(catalog, &mut |value| read(value.and_then(|value| value.dict_get("AcroForm")).and_then(|form| form.dict_get("DR")))),
        }
    }
}
#[derive(Clone, Copy)]
enum AnnotationArray { Direct(PageAttribute), Indirect(ObjRef) }
#[derive(Clone, Copy)]
struct RoleNext { owner: Option<ObjRef>, index: usize, count: usize }

/// 🧳️ Unpublished typed ownership for the application’s original-lane retirement.
pub struct ControlledLiftPartial {
    pub snapshot: Option<PdfSnapshot>,
    pub page: Option<PdfPage>,
}

/// 🗺️ Lifts qualified vector, text and sampled-image pages over the stable borrowed native graph.
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
    font_resources: ControlledFontResources,
    page_resources: Option<ResourceLocation>,
    page_font_dictionary: bool,
    page_fonts: PageSlots,
    states: GraphicsStates,
    content_roles: usize,
    images: ControlledImageResources,
    program: Option<PageProgramCursor>,
    page_refs: Vec<ObjRef>,
    visited: Vec<u32>,
    frames: [Frame; MAX_TREE_DEPTH],
    frames_len: usize,
    walking: bool,
    detail: [u8; MAX_COPY],
    detail_len: usize,
    cold: DocumentLiftState,
    font_lift: Option<Box<FontLift>>,
    registered: usize,
    registered_typed: [usize; 3],
    cold_bytes: usize,
    page_forms: PageSlots,
    renames: ResourceMap,
    mirror: PdfContent,
    releasing: Option<usize>,
    annotations: Option<AnnotationArray>,
    acro_form: Option<Node>,
    form_step: FormStep,
    form_resources: bool,
    debug_trace: Vec<&'static str>,
}

use ControlledLiftEvent as Event;
use ControlledLiftBarrier as Barrier;

impl ControlledDocumentLiftCursor {
    pub(crate) fn new() -> Self {
        Self {
            stage: Stage::Open, snapshot: None, page: None, trailer_seen: 0, catalog: None, info: None, tree: None,
            walk: None, leaf: None, contents: None, program: None,
            font_resources: Default::default(), page_resources: None, page_font_dictionary: false, page_fonts: Default::default(),
            states: Default::default(), content_roles: 0, images: Default::default(),
            page_refs: Vec::new(), visited: Vec::new(), frames: [Frame::at(None); MAX_TREE_DEPTH], frames_len: 0, walking: false,
            detail: [0; MAX_COPY], detail_len: 0,
            cold: DocumentLiftState::default(), font_lift: None, registered: 0, registered_typed: [0; 3], cold_bytes: 0, page_forms: PageSlots::default(), renames: ResourceMap::default(), mirror: PdfContent::default(), releasing: None,
            annotations: None, acro_form: None, form_step: FormStep::Start, form_resources: false, debug_trace: Vec::new(),
        }
    }

    /// 🔎️ Names the entry a deferred lift stopped at and the number of pages lifted before it.
    pub fn deferred_detail(&self) -> Option<(ControlledLiftBarrier, &str, usize)> {
        let Stage::Deferred(reason) = self.stage else { return None; };
        let detail = std::str::from_utf8(&self.detail[..self.detail_len]).unwrap_or("");
        Some((reason, detail, self.snapshot.as_ref().map_or(0, |snapshot| snapshot.pages.len())))
    }

    /// 🏷️ Names the stage the next unit runs, so a refused grant can say which construct asked for it.
    pub fn stage_name(&self) -> &'static str {
        match self.stage {
            Stage::Open => "open", Stage::Trailer(_) => "trailer", Stage::Catalog(_) => "catalog", Stage::Info(_) | Stage::InfoValue { .. } => "info",
            Stage::OptionalContent => "optional-content", Stage::PageCapacity => "page-capacity", Stage::Prewalk | Stage::ClosePrewalk => "page-references",
            Stage::Walk | Stage::CloseWalk => "page-tree", Stage::Resources { .. } | Stage::Entries { .. } => "resources", Stage::Image { .. } => "image",
            Stage::State { .. } => "graphics-state", Stage::Font { .. } => "font", Stage::ColdFont { .. } => "reference-font", Stage::ColdGlyph { .. } => "type3-glyph",
            Stage::ColdFinish { .. } => "font-codec", Stage::ColdForm { .. } => "form", Stage::ColdState { .. } => "direct-graphics-state", Stage::ColdResource { .. } => "resource", Stage::Rename(_) => "rename", Stage::Page(_) | Stage::PageGroup { .. } | Stage::Inherit(_) | Stage::Ancestor { .. } => "page",
            Stage::AnnotationArray(..) | Stage::Annotation(..) => "annotation", Stage::RoleStart | Stage::RoleArray { .. } | Stage::RoleStream { .. } => "content-role",
            Stage::Program => "content", Stage::Mirror(_) => "content-role", Stage::Publish => "publish-page", Stage::Navigation(_) => "navigation",
            Stage::Ready => "ready", Stage::Deferred(_) => "deferred", Stage::Cancelled => "cancelled", Stage::Closing | Stage::Closed => "close",
        }
    }

    /// 🔢️ Pages lifted so far.
    pub fn pages_lifted(&self) -> usize { self.snapshot.as_ref().map_or(0, |snapshot| snapshot.pages.len()) }

    /// 📦️ Quotes empty table, sample or operator backing for the next work unit (a page program's doubled operator vector, the empty content role it is mirrored into).
    pub fn reserving_empty_slots(&self) -> bool {
        matches!(self.stage, Stage::PageCapacity | Stage::AnnotationArray(_, 0, _))
            || matches!(self.stage, Stage::Image { .. }) && self.images.reserves_empty()
            || matches!(self.stage, Stage::Program) && self.program.as_ref().is_some_and(PageProgramCursor::reserving_empty_backing)
            || matches!(self.stage, Stage::Mirror(index) if !self.mirror.has_reserved_slot() && self.page.as_ref().is_some_and(|page| index < page.content.len()))
    }

    /// 📨️ Quotes a typed body before insertion into an admitted slot.
    pub fn next_inline_body_bytes(&self) -> usize {
        match self.stage {
            Stage::Annotation(..) => size_of::<PdfAnnotation>(),
            Stage::State { .. } => size_of::<PdfExtGState>().max(size_of::<PdfNamedProperties>()),
            Stage::Font { .. } => self.font_resources.inline_body_bytes(),
            Stage::Image { .. } => self.images.inline_body_bytes(self.snapshot.as_ref().expect("active PDF image snapshot")),
            _ => 0,
        }
    }

    /// 📏️ The bytes one body-run unit stands for: a source byte read and one byte written, or two bytes of any other copy.
    pub const RUN_UNIT_COPY: usize = 2;
    /// 📏️ The most units one step of a body run costs, so a run granted at least this many always advances.
    pub const RUN_UNIT_MAXIMUM: usize = PageProgramCursor::UNIT_COPY_MAXIMUM.div_ceil(Self::RUN_UNIT_COPY);

    /// 🏃️ Whether the next unit is a body run for [`Self::advance_bodies`]: copying an image's packed bytes or hashing its encoded body (one byte per unit), or running the page program (see [`Self::RUN_UNIT_COPY`]); never an empty reservation.
    pub fn runs_bodies(&self) -> bool {
        match self.stage {
            Stage::Program => self.program.as_ref().is_some_and(|program| !program.reserving_empty_backing()),
            Stage::Image { .. } => self.images.copies_bytes(),
            _ => false,
        }
    }

    /// 🏃️ Advances like [`Self::advance`], but runs a [`Self::runs_bodies`] stage as one body run of up to `grant.work` units of [`Self::RUN_UNIT_COPY`] bytes, at least [`Self::RUN_UNIT_MAXIMUM`] of them.
    pub fn advance_bodies(&mut self, trailer: &[PdfDictEntry], version: &str, source: &mut dyn ObjectSource, grant: CosGrant, cancelled: bool) -> ControlledLiftReceipt {
        if cancelled || grant.work == 0 || !matches!(self.stage, Stage::Program) { return self.advance(trailer, version, source, grant, cancelled); }
        self.program_run(source, grant)
    }

    /// ▶️ Advances only measured work. Requires the native graph's borrowed inspection and role semantics.
    pub fn advance(&mut self, trailer: &[PdfDictEntry], version: &str, source: &mut dyn ObjectSource, grant: CosGrant, cancelled: bool) -> ControlledLiftReceipt {
        if cancelled && !matches!(self.stage, Stage::Closing | Stage::Closed) { self.stage = Stage::Cancelled; }
        { let name = self.stage_name(); if self.debug_trace.last() != Some(&name) { if self.debug_trace.len() >= 16 { self.debug_trace.remove(0); } self.debug_trace.push(name); } }
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
                let Some(owner) = self.info else { self.stage = Stage::OptionalContent; return work(0, 0); };
                let mut turn = None;
                source.inspect(owner, &mut |value| { turn = Some(self.info_entry(value, grant, index)); });
                turn.unwrap_or_else(|| self.defer(Barrier::Info))
            }
            Stage::InfoValue { index, field, target } => {
                let mut turn = None;
                source.inspect(target, &mut |value| { turn = Some(self.info_field(field, value, grant, index)); });
                turn.unwrap_or_else(|| self.defer(Barrier::Info))
            }
            Stage::OptionalContent => self.optional_content(source, grant),
            Stage::PageCapacity => self.reserve_pages(source, grant),
            Stage::Prewalk => self.prewalk(source, grant),
            Stage::ClosePrewalk => {
                let turn = self.walk.as_mut().expect("closing PDF page reference walk").close_step(grant);
                if turn.event == Ok(PageTreeEvent::Closed) {
                    self.walk = Some(PageTreeCursor::new(self.tree.expect("qualified PDF page tree")));
                    self.stage = Stage::Walk;
                }
                tree_turn(turn)
            }
            Stage::Walk => self.walk_page(source, grant),
            Stage::Resources { location, resume } => self.resources(source, location, resume),
            Stage::Entries { location, category, index, resume } => self.entries(source, location, category, index, resume),
            Stage::Image { location, index, resume } => self.image(source, grant, location, index, resume),
            Stage::Font { location, index, owner, resume } => self.font(source, grant, location, index, owner, resume),
            Stage::ColdFont { location, index, owner, resume } => self.cold_font(source, grant, location, index, owner, resume),
            Stage::ColdGlyph { location, index, owner, resume } => self.cold_glyph(source, grant, location, index, owner, resume),
            Stage::ColdFinish { location, index, owner, resume } => self.cold_finish(source, grant, location, index, owner, resume),
            Stage::ColdForm { location, index, owner, resume } => self.cold_form(source, grant, location, index, owner, resume),
            Stage::ColdState { location, index, resume } => self.cold_state(source, grant, location, index, resume),
            Stage::ColdResource { location, category, index, resume } => self.cold_resource(source, grant, location, category, index, resume),
            Stage::State { location, category, index, owner, resume } => self.state(source, grant, location, category, index, owner, resume),
            Stage::Page(index) => {
                let leaf = self.leaf.expect("active PDF page leaf");
                let mut turn = None;
                source.inspect(leaf.reference, &mut |value| { turn = Some(self.page_entry(value, leaf.reference, index)); });
                turn.unwrap_or_else(|| self.defer(Barrier::Page))
            }
            Stage::PageGroup { owner, resume } => {
                let mut group = None;
                source.inspect(owner, &mut |value| { group = value.and_then(simple_group); });
                let Some(group) = group else { return self.defer_detail(Barrier::Page, "Group"); };
                self.page.as_mut().expect("active typed page").group = Some(group);
                self.stage = Stage::Page(resume);
                work(0, 0)
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
                    None => self.cold_annotation(source, grant, array, index, resume, owner),
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
            Stage::Program => self.program(source, grant),
            Stage::Rename(index) => self.rename(grant, index),
            Stage::Mirror(index) => self.mirror(grant, index),
            Stage::Publish => self.publish_page(source, grant),
            Stage::CloseWalk => self.close_walk(grant, true),
            Stage::Navigation(lane) => self.navigate(source, grant, lane),
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
        self.images.begin_close();
        self.font_resources.begin_close();
        if let Some(walk) = self.walk.as_mut() { walk.begin_close(); }
        if let Some(program) = self.program.as_mut() { program.begin_close(); }
        self.stage = Stage::Closing;
    }

    /// 🧹️ Settles active cursors before returning the unpublished page and snapshot separately.
    pub fn close_step(&mut self, grant: CosGrant) -> ControlledLiftReceipt {
        if matches!(self.stage, Stage::Closed) { return quote(Event::Closed); }
        if !matches!(self.stage, Stage::Closing) || grant.work == 0 { return quote(Event::More); }
        let image = self.images.close_step(grant);
        if image.status != Ok(CosStatus::Closed) { return image_turn(image); }
        let font = self.font_resources.close_step(grant);
        if font.status != Ok(CosStatus::Closed) || font.work != 0 { return image_turn(font); }
        if let Some(program) = self.program.as_mut() {
            let mut turn = content_turn(program.close_step(grant));
            if turn.event == Ok(Event::Closed) { self.program = None; turn.event = Ok(Event::More); }
            return turn;
        }
        if self.walk.is_some() { return self.close_walk(grant, false); }
        if let Some(turn) = self.release_held(grant, false) { return turn; }
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
                let Some(flags) = small_dict(Some(value)) else { return self.defer_detail(Barrier::Catalog, "MarkInfo"); };
                let flag = |key| dict_get(flags, key).and_then(PdfObject::as_bool).unwrap_or(false);
                self.snapshot_mut().mark_info = Some(PdfMarkInfo { marked: flag("Marked"), user_properties: flag("UserProperties"), suspects: flag("Suspects") });
            }
            "AcroForm" => {
                let catalog = self.catalog.expect("qualified PDF catalog");
                let Some(form) = Node::within(catalog, None, TreeKey::AcroForm, value) else { return self.defer_detail(Barrier::Catalog, "AcroForm"); };
                self.acro_form = Some(form);
            }
            "OutputIntents" => return self.defer_detail(Barrier::Catalog, &entry.key),
            "Type" | "Pages" => return self.defer(Barrier::Catalog),
            _ => {}
        }
        self.stage = Stage::Catalog(index + 1);
        turn
    }

    fn info_entry(&mut self, value: Option<&PdfObject>, grant: CosGrant, index: usize) -> ControlledLiftReceipt {
        let Some(dict) = value.and_then(PdfObject::as_dict) else { return self.defer(Barrier::Info); };
        let Some(entry) = dict.get(index) else { self.stage = Stage::OptionalContent; return work(0, 0); };
        if dict[..index].iter().any(|other| other.key == entry.key) { return self.defer(Barrier::Info); }
        if let Some(field) = InfoField::of(&entry.key) {
            if let PdfObject::Ref(target) = entry.value {
                self.stage = Stage::InfoValue { index, field, target };
                return work(0, 0);
            }
            return self.info_field(field, Some(&entry.value), grant, index);
        }
        if entry.key == "Trapped" {
            let text = entry.value.as_name();
            let bytes = text.map_or(0, str::len);
            if let Some(denied) = budget(grant, bytes, 0) { return denied; }
            self.snapshot_mut().info.trapped = text.map(str::to_owned);
            self.stage = Stage::Info(index + 1);
            return work(bytes, 0);
        }
        let extra = &self.snapshot.as_ref().expect("active typed PDF snapshot").info.extra;
        let table = if extra.capacity() == 0 { dict.iter().filter(|entry| !INFO_KEYS.contains(&entry.key.as_str())).count() * size_of::<PdfDictEntry>() } else { 0 };
        let bytes = table + entry.key.len() + direct_bytes(&entry.value);
        if let Some(denied) = budget(grant, bytes, 0) { return denied; }
        let extra = &mut self.snapshot_mut().info.extra;
        if table != 0 && extra.try_reserve_exact(table / size_of::<PdfDictEntry>()).is_err() {
            return ControlledLiftReceipt { event: Err("unable to allocate PDF information extras"), ..work(0, 0) };
        }
        extra.push(entry.clone());
        self.stage = Stage::Info(index + 1);
        work(bytes, 0)
    }

    fn info_field(&mut self, field: InfoField, value: Option<&PdfObject>, grant: CosGrant, index: usize) -> ControlledLiftReceipt {
        let info = &mut self.snapshot.as_mut().expect("active typed PDF snapshot").info;
        if matches!(field, InfoField::CreationDate | InfoField::ModDate) {
            let date = match value { Some(PdfObject::Date(date)) => Some(date.clone()), _ => None };
            if field == InfoField::CreationDate { info.creation_date = date; } else { info.modification_date = date; }
            self.stage = Stage::Info(index + 1);
            return work(0, 0);
        }
        let quoted = match value {
            Some(PdfObject::Text(text) | PdfObject::Name(text)) => text.len(),
            Some(PdfObject::Str(bytes)) => 3 * bytes.len() + 3,
            _ => 0,
        };
        if let Some(denied) = budget(grant, quoted, 0) { return denied; }
        let text = match value {
            Some(PdfObject::Text(text) | PdfObject::Name(text)) => Some(text.clone()),
            Some(PdfObject::Str(bytes)) => Some(decode_text_string(bytes)),
            _ => None,
        };
        let bytes = text.as_ref().map_or(0, String::capacity);
        *match field {
            InfoField::Title => &mut info.title, InfoField::Author => &mut info.author, InfoField::Subject => &mut info.subject,
            InfoField::Keywords => &mut info.keywords, InfoField::Creator => &mut info.creator, _ => &mut info.producer,
        } = text;
        self.stage = Stage::Info(index + 1);
        work(bytes.min(quoted), 0)
    }

    /// 👁️ Lifts the catalog's optional content (groups and default configuration) before any page, in the reference lifter's order, through its builder over the persistent reference state, so property lists and later lanes resolve group ids.
    fn optional_content(&mut self, source: &mut dyn ObjectSource, grant: CosGrant) -> ControlledLiftReceipt {
        let mut catalog = None;
        source.inspect(self.catalog.expect("qualified PDF catalog"), &mut |value| {
            catalog = value.and_then(PdfObject::as_dict).filter(|entries| dict_get(entries, "OCProperties").is_some()).map(<[PdfDictEntry]>::to_vec);
        });
        let Some(catalog) = catalog else { self.stage = Stage::PageCapacity; return work(0, 0); };
        let properties = dict_get(&catalog, "OCProperties").cloned().expect("catalog optional content");
        let bytes = 4 * reach(source, &properties, 3) + direct_bytes(&PdfObject::Dict(catalog.clone()));
        if let Some(denied) = budget(grant, bytes, 0) { return denied; }
        let lifted = self.cold_step(source, |lifter, _| lifter.lift_optional_content(&catalog));
        let Some(((), roles)) = lifted.filter(|(_, roles)| roles.is_empty()) else { return self.defer_detail(Barrier::Catalog, "OCProperties"); };
        drop(roles);
        let content = self.cold.snapshot.optional_content.take();
        self.snapshot_mut().optional_content = content;
        self.cold_bytes += bytes;
        self.stage = Stage::PageCapacity;
        work(bytes, 0)
    }

    fn reserve_pages(&mut self, source: &mut dyn ObjectSource, grant: CosGrant) -> ControlledLiftReceipt {
        let Some(root) = self.tree else {
            self.stage = Stage::Navigation(Lane::Outlines);
            return work(0, 0);
        };
        let mut count = None;
        source.inspect(root, &mut |value| { count = small_dict(value).and_then(|dict| match dict_get(dict, "Count") { Some(PdfObject::Int(n)) => usize::try_from(*n).ok(), _ => None }).filter(|count| *count <= MAX_PAGES); });
        let Some(count) = count else { return self.defer(Barrier::PageTree); };
        let roles = count;
        let snapshot = self.snapshot.as_mut().expect("active PDF lift snapshot");
        if snapshot.pages.capacity() < count {
            if let Some(denied) = budget(grant, count * size_of::<PdfPage>(), 0) { return denied; }
            if snapshot.pages.try_reserve_exact(count).is_err() {
                return ControlledLiftReceipt { event: Err("unable to allocate PDF page table"), ..work(0, 0) };
            }
            return work(snapshot.pages.capacity() * size_of::<PdfPage>(), 0);
        }
        let roles_table = &mut snapshot.admitted_stream_roles;
        if roles_table.capacity() - roles_table.len() < roles {
            let before = roles_table.capacity();
            if let Some(denied) = budget(grant, (roles_table.len() + roles - before) * size_of::<PdfAdmittedStreamRole>(), 0) { return denied; }
            if roles_table.try_reserve_exact(roles).is_err() {
                return ControlledLiftReceipt { event: Err("unable to allocate PDF stream role table"), ..work(0, 0) };
            }
            return work((roles_table.capacity() - before) * size_of::<PdfAdmittedStreamRole>(), 0);
        }
        if let Some(denied) = budget(grant, count * size_of::<ObjRef>(), 0) { return denied; }
        if self.page_refs.try_reserve_exact(count).is_err() {
            return ControlledLiftReceipt { event: Err("unable to allocate PDF page reference table"), ..work(0, 0) };
        }
        self.walk = Some(PageTreeCursor::new(root));
        self.stage = Stage::Prewalk;
        work(self.page_refs.capacity() * size_of::<ObjRef>(), 0)
    }

    /// 📇️ Collects every page reference in tree order before any page is lifted, so destinations reach later pages like the reference lifter's.
    fn prewalk(&mut self, source: &mut dyn ObjectSource, grant: CosGrant) -> ControlledLiftReceipt {
        let walk = self.walk.as_mut().expect("active PDF page reference walk");
        let turn = walk.advance(source, grant, false);
        match turn.event {
            Ok(PageTreeEvent::Page(leaf)) => {
                if self.page_refs.len() == self.page_refs.capacity() { return self.defer_turn(Barrier::PageTree, tree_turn(turn)); }
                self.page_refs.push(leaf.reference);
            }
            Ok(PageTreeEvent::Complete) => { walk.begin_close(); self.stage = Stage::ClosePrewalk; }
            Err(_) => return self.defer_turn(Barrier::PageTree, tree_turn(turn)),
            _ => {}
        }
        tree_turn(turn)
    }

    fn walk_page(&mut self, source: &mut dyn ObjectSource, grant: CosGrant) -> ControlledLiftReceipt {
        let walk = self.walk.as_mut().expect("active PDF page tree");
        let turn = walk.advance(source, grant, false);
        match turn.event {
            Ok(PageTreeEvent::Page(leaf)) => {
                let lifted = self.snapshot_mut().pages.len();
                if lifted == self.snapshot_mut().pages.capacity() || self.page_refs.get(lifted) != Some(&leaf.reference) {
                    return self.defer_turn(Barrier::PageTree, tree_turn(turn));
                }
                self.leaf = Some(leaf);
                self.page = Some(PdfPage::default());
                self.contents = None;
                self.page_resources = None;
                self.page_font_dictionary = false;
                self.page_fonts.slots.clear();
                self.page_forms.slots.clear();
                self.renames = ResourceMap::default();
                self.annotations = None;
                self.images.clear_page();
                self.stage = Stage::Page(0);
            }
            Ok(PageTreeEvent::BranchResources(attribute)) => self.stage = Stage::Resources { location: ResourceLocation::Direct(attribute), resume: Resume::Walk },
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
            if completing { self.stage = Stage::Navigation(Lane::Outlines); }
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
                self.annotations = Some(value.as_ref().map_or(AnnotationArray::Direct(PageAttribute { owner, entry: index }), AnnotationArray::Indirect));
            }
            "AA" if empty_dictionary(Some(value)) => {}
            "Group" => match value {
                PdfObject::Ref(group) => {
                    self.stage = Stage::PageGroup { owner: *group, resume: index + 1 };
                    return work(0, 0);
                }
                value => match simple_group(value) {
                    Some(group) => page.group = Some(group),
                    None => return self.defer_detail(Barrier::Page, "Group"),
                },
            }
            _ => return self.defer_detail(Barrier::Page, &entry.key),
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
        if index == count { self.stage = if resume == AFTER_CONTENT { Stage::Publish } else { Stage::Page(resume) }; return work(0, 0); }
        let Some(owner) = owner else { return self.defer(Barrier::Page); };
        let growth = if self.acro_form.is_some() { annotation_reference_growth(&self.cold.annotation_refs) } else { 0 };
        if let Some(turn) = budget(grant, growth + if index == 0 { count * size_of::<PdfAnnotation>() } else { 0 }, 0) { return turn; }
        let mut bytes = 0;
        if index == 0 {
            let annotations = &mut self.page.as_mut().expect("active typed page").annotations;
            if annotations.try_reserve_exact(count).is_err() {
                return ControlledLiftReceipt { event: Err("unable to allocate PDF annotation table"), ..work(0, 0) };
            }
            bytes = annotations.capacity() * size_of::<PdfAnnotation>();
        }
        if self.acro_form.is_some() {
            if self.cold.annotation_refs.try_reserve(1).is_err() { return ControlledLiftReceipt { event: Err("unable to allocate the PDF widget position table"), ..work(0, 0) }; }
            let page = self.snapshot.as_ref().expect("active typed PDF snapshot").pages.len() as u32;
            self.cold.annotation_refs.insert(owner, (page, index as u32));
            self.cold_bytes += growth;
            bytes += growth;
        }
        self.stage = Stage::Annotation(array, index, resume, owner);
        work(bytes, 0)
    }

    /// 🔗️ Lifts an annotation outside the inline-quoted subset through the reference builder: no pop-ups, replies or optional content; one with appearance streams runs over the persistent reference state ([`Self::cold_appearance`]).
    fn cold_annotation(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, array: AnnotationArray, index: usize, resume: usize, owner: ObjRef) -> ControlledLiftReceipt {
        let mut dict = None;
        source.inspect(owner, &mut |value| {
            dict = value.and_then(PdfObject::as_dict).filter(|dict| !dict.iter().any(|entry| matches!(entry.key.as_str(), "Popup" | "IRT" | "OC"))).map(<[PdfDictEntry]>::to_vec);
        });
        let Some(dict) = dict else { return self.defer_detail(Barrier::Page, "Annots"); };
        if dict_get(&dict, "AP").is_some() { return self.cold_appearance(source, grant, array, index, resume, dict); }
        let estimate = size_of::<PdfAnnotation>() + 2 * size_of::<PdfAction>() + 3 * dict.iter()
            .filter(|entry| entry.key != "P")
            .map(|entry| size_of::<PdfDictEntry>() + entry.key.len() + reach(source, &entry.value, 2)).sum::<usize>();
        if let Some(denied) = budget(grant, estimate, 0) { return denied; }
        let Some((annotation, _)) = self.cold(source, false, |lifter| lifter.lift_annotation(&dict)) else { return self.defer_detail(Barrier::Page, "Annots"); };
        let retained = annotation_heap(&annotation).map_or(estimate, |heap| heap.min(estimate));
        self.page.as_mut().expect("active typed page").annotations.push(annotation);
        self.stage = Stage::AnnotationArray(array, index + 1, resume);
        work(retained, 0)
    }

    /// 🖼️ Lifts one annotation with appearance streams (its forms, their resources and `Operators` roles) through the reference builder over the persistent reference state, then moves what it typed into the document lanes in the reference order.
    fn cold_appearance(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, array: AnnotationArray, index: usize, resume: usize, dict: Vec<PdfDictEntry>) -> ControlledLiftReceipt {
        if let Some(turn) = self.register_font(grant) { return turn; }
        if let Some(turn) = self.register_typed(grant) { return turn; }
        let mut fonts = self.font_resources.owners.clone();
        fonts.sort_unstable();
        let mut images = 0;
        let appearance = dict_get(&dict, "AP").cloned().expect("annotation appearance");
        let snapshot = self.snapshot.as_ref().expect("active typed PDF snapshot");
        let bytes = size_of::<PdfAnnotation>() + 2 * size_of::<PdfAction>() + 3 * dict.iter().filter(|entry| entry.key != "P").map(|entry| size_of::<PdfDictEntry>() + entry.key.len() + reach(source, &entry.value, 2)).sum::<usize>()
            + appearance_quote(source, &appearance, &fonts, &mut images) + 4 * registration_bytes("APN")
            + table_growth(&snapshot.admitted_stream_roles, 8 + 2 * images) + table_growth(&snapshot.images, images) + self.images.adoption_growth(images) + images * registration_bytes("");
        if let Some(denied) = budget(grant, bytes, 0) { return denied; }
        let scope = self.leaf.map_or(0, |leaf| leaf.reference.num as usize);
        let lifted = self.cold_step(source, |lifter, _| { lifter.scope = scope; lifter.lift_annotation(&dict) });
        let Some((annotation, roles)) = lifted else { return self.defer_detail(Barrier::Page, "Annots/AP"); };
        if !self.merge_roles(roles) { return ControlledLiftReceipt { event: Err("unable to grow the PDF stream role table"), ..work(0, 0) }; }
        if let Err(detail) = self.merge_typed() { return self.defer_detail(Barrier::Page, detail); }
        self.page.as_mut().expect("active typed page").annotations.push(annotation);
        self.cold_bytes += bytes;
        self.stage = Stage::AnnotationArray(array, index + 1, resume);
        work(bytes, 0)
    }

    fn inherit(&mut self, source: &mut dyn ObjectSource, index: usize) -> ControlledLiftReceipt {
        let leaf = self.leaf.expect("qualified PDF page");
        let inherited = leaf.inheritance;
        let attributes = [inherited.media_box, inherited.crop_box, inherited.rotate];
        if index == attributes.len() {
            if let Some(attribute) = inherited.resources { return self.resources(source, ResourceLocation::Direct(attribute), Resume::Inherit); }
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

    fn resources(&mut self, source: &mut dyn ObjectSource, location: ResourceLocation, resume: Resume) -> ControlledLiftReceipt {
        let mut owner = None;
        let mut shape = None;
        let mut unsupported = "Resources";
        location.inspect(source, &mut |value| {
            owner = value.and_then(PdfObject::as_ref);
            shape = resource_shape(value);
            if shape.is_none() { unsupported = unsupported_resource(value); }
        });
        if let (ResourceLocation::Direct(_), Some(owner)) = (location, owner) {
            self.stage = Stage::Resources { location: ResourceLocation::Indirect(owner), resume };
            return work(0, 0);
        }
        let Some(shape) = shape else { return self.defer_detail(Barrier::Resources, unsupported); };
        if resume == Resume::Role {
            if self.page_resources != Some(location) { return self.defer(Barrier::Resources); }
            self.resume(resume);
            return work(0, 0);
        }
        if resume == Resume::Inherit {
            self.page_resources = Some(location);
            self.page_font_dictionary = shape.fonts;
        }
        if resume == Resume::Walk && shape.populated && !self.snapshot_mut().pages.is_empty() { return self.defer(Barrier::Resources); }
        self.stage = Stage::Entries { location, category: Category::Font, index: 0, resume };
        work(0, 0)
    }

    fn entries(&mut self, source: &mut dyn ObjectSource, location: ResourceLocation, category: Category, index: usize, resume: Resume) -> ControlledLiftReceipt {
        let entry = read_entry(source, location, category, index);
        match entry {
            None => return self.defer_detail(Barrier::Resources, category.key()),
            Some(Entry::End) => match category {
                Category::Font => self.stage = Stage::Entries { location, category: Category::XObject, index: 0, resume },
                Category::XObject => self.stage = Stage::Entries { location, category: Category::ExtGState, index: 0, resume },
                Category::ExtGState => self.stage = Stage::Entries { location, category: Category::ColorSpace, index: 0, resume },
                Category::ColorSpace => self.stage = Stage::Entries { location, category: Category::Shading, index: 0, resume },
                Category::Shading => self.stage = Stage::Entries { location, category: Category::Pattern, index: 0, resume },
                Category::Pattern => self.stage = Stage::Entries { location, category: Category::Properties, index: 0, resume },
                Category::Properties => self.resume(resume),
            },
            Some(Entry::Direct) if category == Category::ExtGState && resume == Resume::Inherit => self.stage = Stage::ColdState { location, index, resume },
            Some(Entry::Direct) if category.is_cold() && resume == Resume::Inherit => self.stage = Stage::ColdResource { location, category, index, resume },
            Some(Entry::Direct) => return self.defer_detail(Barrier::Resources, category.key()),
            Some(Entry::Item { name, length, owner }) => match category {
                Category::Font => self.stage = Stage::Font { location, index, owner, resume },
                Category::XObject => {
                    let mut form = false;
                    source.inspect(owner, &mut |value| { form = value.and_then(|value| value.dict_get("Subtype")).and_then(PdfObject::as_name) == Some("Form"); });
                    if form {
                        self.stage = Stage::ColdForm { location, index, owner, resume };
                        return work(0, 0);
                    }
                    let Some(binding) = ControlledImageBinding::new(owner, &name[..length]) else { return self.defer_detail(Barrier::Resources, "XObject"); };
                    self.images.begin(binding);
                    self.stage = Stage::Image { location, index, resume };
                }
                Category::ColorSpace | Category::Shading | Category::Pattern => self.stage = Stage::ColdResource { location, category, index, resume },
                Category::ExtGState | Category::Properties => self.stage = Stage::State { location, category, index, owner, resume },
            },
        }
        work(0, 0)
    }

    fn image(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, location: ResourceLocation, index: usize, resume: Resume) -> ControlledLiftReceipt {
        let growth = self.images.page_growth();
        if resume == Resume::Inherit && growth != 0 {
            if let Some(denied) = budget(grant, growth, 0) { return denied; }
            if !self.images.grow_page() { return ControlledLiftReceipt { event: Err("unable to grow the PDF page image table"), ..work(0, 0) }; }
            return work(growth, 0);
        }
        let Some(turn) = self.images.advance(source, self.snapshot.as_mut().expect("active PDF image snapshot"), grant, self.content_roles) else {
            let Some(owner) = self.images.abandon() else { return self.defer_detail(Barrier::Resources, "XObject/Image"); };
            self.stage = Stage::ColdForm { location, index, owner, resume };
            return work(0, 0);
        };
        if let Ok(CosStatus::Complete { end }) = turn.status {
            if resume == Resume::Inherit && !self.images.push_page(end) { return self.defer_turn(Barrier::Resources, image_turn(turn)); }
            self.stage = Stage::Entries { location, category: Category::XObject, index: index + 1, resume };
        }
        image_turn(turn)
    }

    fn state(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, location: ResourceLocation, category: Category, index: usize, owner: ObjRef, resume: Resume) -> ControlledLiftReceipt {
        let entry = read_entry(source, location, category, index);
        let Some(Entry::Item { name, length, owner: current }) = entry else { return self.defer(Barrier::Resources); };
        if current != owner { return self.defer(Barrier::Resources); }
        let name = std::str::from_utf8(&name[..length]).expect("authored PDF resource name");
        let snapshot = self.snapshot.as_mut().expect("active PDF state snapshot");
        let admitted = match category {
            Category::ExtGState => self.states.admit_ext_g_state(source, snapshot, name, owner, grant),
            _ => self.states.admit_properties(source, snapshot, name, owner, self.cold.ids.get(&(LiftCategory::OptionalContent, owner)).map(String::as_str), grant),
        };
        let Some((turn, complete)) = admitted else { return self.defer_detail(Barrier::Resources, category.key()); };
        if complete { self.stage = Stage::Entries { location, category, index: index + 1, resume }; }
        turn
    }

    fn font(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, location: ResourceLocation, index: usize, owner: ObjRef, resume: Resume) -> ControlledLiftReceipt {
        if resume == Resume::Inherit { if let Some(turn) = self.page_fonts.room(grant) { return turn; } }
        let entry = read_entry(source, location, Category::Font, index);
        let Some(Entry::Item { name, length, owner: current }) = entry else { return self.defer(Barrier::Resources); };
        if current != owner { return self.defer(Barrier::Resources); }
        let name = std::str::from_utf8(&name[..length]).expect("authored PDF font name");
        let snapshot = self.snapshot.as_mut().expect("active PDF font snapshot");
        let turn = self.font_resources.admit(&mut |id, read| source.inspect(id, read), snapshot, owner, name, grant);
        let Some(turn) = turn else {
            if self.font_resources.slot_of(owner).is_some() || snapshot.fonts.iter().any(|font| font.id == name) { return self.defer_detail(Barrier::Resources, "Font/Name"); }
            self.stage = Stage::ColdFont { location, index, owner, resume };
            return work(0, 0);
        };
        if let Ok(CosStatus::Complete { end: slot }) = turn.status {
            if resume == Resume::Inherit && !self.page_fonts.push(slot) { return self.defer_turn(Barrier::Resources, image_turn(turn)); }
            self.stage = Stage::Entries { location, category: Category::Font, index: index + 1, resume };
        }
        image_turn(turn)
    }

    /// 🔤️ Names the font subtype a deferred reference builder stopped at.
    fn font_detail(source: &mut dyn ObjectSource, owner: ObjRef) -> &'static str {
        let mut kind = "Font";
        source.inspect(owner, &mut |value| {
            kind = match value.and_then(|value| value.dict_get("Subtype")).and_then(PdfObject::as_name) {
                Some("Type3") => "Font/Type3", Some("Type1") => "Font/Type1", Some("MMType1") => "Font/MMType1",
                Some("TrueType") => "Font/TrueType", Some("Type0") => "Font/Type0", _ => "Font",
            };
        });
        kind
    }

    /// 🪪️ Registers one more typed font in the reference builder's id table, so glyph programs depend on every claimed font like the reference lifter's.
    fn register_font(&mut self, grant: CosGrant) -> Option<ControlledLiftReceipt> {
        let owner = *self.font_resources.owners.get(self.registered)?;
        let id = &self.snapshot.as_ref().expect("active typed PDF snapshot").fonts[self.registered].id;
        let bytes = registration_bytes(id);
        if let Some(denied) = budget(grant, bytes, 0) { return Some(denied); }
        let id = id.clone();
        self.cold.ids.insert((LiftCategory::Font, owner), id.clone());
        self.cold.used.entry(LiftCategory::Font).or_default().insert(id);
        self.registered += 1;
        self.cold_bytes += bytes;
        Some(work(bytes, 0))
    }

    /// 🧊️ Runs one step of the reference font builder over the persistent reference state; the returned roles are the ones the step recorded.
    fn cold_step<R>(&mut self, source: &mut dyn ObjectSource, run: impl FnOnce(&mut Lifter<'_>, &mut Option<Box<FontLift>>) -> R) -> Option<(R, Vec<PdfAdmittedStreamRole>)> {
        let mut state = take(&mut self.cold);
        state.page_refs = take(&mut self.page_refs);
        let mut lifter = Lifter::bind(&mut *source, state);
        let result = run(&mut lifter, &mut self.font_lift);
        let mut state = lifter.unbind();
        self.page_refs = take(&mut state.page_refs);
        let failed = state.admission_error.is_some();
        self.cold = state;
        let roles = source.take_admitted_roles();
        (!failed).then_some((result, roles))
    }

    /// 🧬️ Places resource roles before the content roles, replacing a role of the same identity and kind in place like the reference source does.
    fn merge_roles(&mut self, roles: Vec<PdfAdmittedStreamRole>) -> bool {
        let content_roles = self.content_roles;
        let table = &mut self.snapshot.as_mut().expect("active typed PDF snapshot").admitted_stream_roles;
        for role in roles {
            let end = table.len() - content_roles;
            if let Some(current) = table[..end].iter_mut().find(|current| current.identity == role.identity && current.value.kind() == role.value.kind()) {
                *current = role;
                continue;
            }
            if !grow_table(table, 1) { return false; }
            table.insert(end, role);
        }
        true
    }

    fn cold_font(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, location: ResourceLocation, index: usize, owner: ObjRef, resume: Resume) -> ControlledLiftReceipt {
        if let Some(turn) = self.font_resources.retire_pending(grant) { return image_turn(turn); }
        if let Some(turn) = self.register_font(grant) { return turn; }
        let entry = read_entry(source, location, Category::Font, index);
        let Some(Entry::Item { name, length, owner: current }) = entry else { return self.defer(Barrier::Resources); };
        if current != owner { return self.defer(Barrier::Resources); }
        let name = std::str::from_utf8(&name[..length]).expect("authored PDF font name").to_owned();
        if self.cold.used.get(&LiftCategory::Font).is_some_and(|used| used.contains(&name)) { return self.defer_detail(Barrier::Resources, "Font/Name"); }
        let roles = &self.snapshot.as_ref().expect("active typed PDF snapshot").admitted_stream_roles;
        let bytes = font_quote(source, owner) + registration_bytes(&name) + size_of::<FontLift>() + table_growth(roles, FONT_HEAD_ROLES);
        if let Some(denied) = budget(grant, bytes, 0) { return denied; }
        if !grow_table(&mut self.snapshot_mut().admitted_stream_roles, FONT_HEAD_ROLES) { return ControlledLiftReceipt { event: Err("unable to grow the PDF stream role table"), ..work(0, 0) }; }
        let started = self.cold_step(source, |lifter, pending| {
            if let FontStart::Lifting(lift) = lifter.begin_font(&name, &PdfObject::Ref(owner)) { *pending = Some(lift); }
        });
        let Some(((), roles)) = started.filter(|_| self.font_lift.is_some()) else { return self.defer_detail(Barrier::Resources, Self::font_detail(source, owner)); };
        if !self.merge_roles(roles) { return ControlledLiftReceipt { event: Err("unable to grow the PDF stream role table"), ..work(0, 0) }; }
        self.cold_bytes += bytes;
        let glyphs = self.font_lift.as_ref().expect("claimed reference font").remaining_glyphs();
        self.stage = if glyphs == 0 { Stage::ColdFinish { location, index, owner, resume } } else { Stage::ColdGlyph { location, index, owner, resume } };
        work(bytes, 0)
    }

    fn cold_glyph(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, location: ResourceLocation, index: usize, owner: ObjRef, resume: Resume) -> ControlledLiftReceipt {
        let lift = self.font_lift.as_ref().expect("lifting reference font");
        let Some(glyph) = lift.next_glyph().and_then(PdfObject::as_ref) else { return self.defer_detail(Barrier::Resources, "Font/Type3"); };
        let mut fonts = self.font_resources.owners.clone();
        fonts.sort_unstable();
        let mut dependencies = source.role_dependencies(&root_identity(glyph));
        for font in fonts { dependencies.extend(source.role_dependencies(&root_identity(font))); }
        let mut program = program_quote(&[]);
        source.inspect(glyph, &mut |value| { if let Some(PdfObject::Stream { data, .. }) = value { program = program_quote(&data.to_vec_owner()); } });
        let roles = &self.snapshot.as_ref().expect("active typed PDF snapshot").admitted_stream_roles;
        let bytes = identity_bytes(&dependencies) + program + size_of::<PdfCharProc>() + table_growth(roles, 1);
        drop(dependencies);
        if let Some(denied) = budget(grant, bytes, 0) { return denied; }
        let stepped = self.cold_step(source, |lifter, pending| lifter.lift_char_proc(pending.as_mut().expect("lifting reference font")));
        let Some((_, roles)) = stepped else { return self.defer_detail(Barrier::Resources, "Font/Type3"); };
        if !self.merge_roles(roles) { return ControlledLiftReceipt { event: Err("unable to grow the PDF stream role table"), ..work(0, 0) }; }
        self.cold_bytes += bytes;
        if self.font_lift.as_ref().expect("lifting reference font").remaining_glyphs() == 0 { self.stage = Stage::ColdFinish { location, index, owner, resume }; }
        work(bytes, 0)
    }

    /// 🪪️ Registers one more image, graphics state or property list typed on the controlled path in the reference builder's id table, so a form's own resources resolve them like the reference lift.
    fn register_typed(&mut self, grant: CosGrant) -> Option<ControlledLiftReceipt> {
        let snapshot = self.snapshot.as_ref().expect("active typed PDF snapshot");
        let [images, states, properties] = self.registered_typed;
        let (lane, category, owner, id) = if images < snapshot.images.len() {
            (0, LiftCategory::XObject, self.images.owner(images), &snapshot.images[images].id)
        } else if states < snapshot.ext_g_states.len() {
            (1, LiftCategory::ExtGState, self.states.states[states], &snapshot.ext_g_states[states].id)
        } else if properties < snapshot.properties.len() {
            (2, LiftCategory::Properties, self.states.properties[properties], &snapshot.properties[properties].name)
        } else {
            return None;
        };
        let bytes = registration_bytes(id);
        if let Some(denied) = budget(grant, bytes, 0) { return Some(denied); }
        let id = id.clone();
        if owner != ObjRef::default() { self.cold.ids.insert((category, owner), id.clone()); }
        self.cold.used.entry(category).or_default().insert(id);
        self.registered_typed[lane] += 1;
        self.cold_bytes += bytes;
        Some(work(bytes, 0))
    }

    /// 🖼️ Lifts one form XObject (its own resources, nested forms and its `Operators` role) through the reference builder over the persistent reference state, then moves what it typed into the document lanes in the reference order.
    fn cold_form(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, location: ResourceLocation, index: usize, owner: ObjRef, resume: Resume) -> ControlledLiftReceipt {
        let image = is_image(source, owner);
        if resume == Resume::Inherit {
            if let Some(turn) = self.page_forms.room(grant) { return turn; }
            let growth = self.images.page_growth();
            if growth != 0 {
                if let Some(denied) = budget(grant, growth, 0) { return denied; }
                if !self.images.grow_page() { return ControlledLiftReceipt { event: Err("unable to grow the PDF page image table"), ..work(0, 0) }; }
                return work(growth, 0);
            }
        }
        if let Some(turn) = self.register_font(grant) { return turn; }
        if let Some(turn) = self.register_typed(grant) { return turn; }
        let entry = read_entry(source, location, Category::XObject, index);
        let Some(Entry::Item { name, length, owner: current }) = entry else { return self.defer(Barrier::Resources); };
        if current != owner { return self.defer(Barrier::Resources); }
        let name = std::str::from_utf8(&name[..length]).expect("authored PDF form name").to_owned();
        let claimed = self.cold.ids.get(&(LiftCategory::XObject, owner)).cloned();
        let mut turn = work(0, 0);
        match claimed {
            Some(id) if id == name => {}
            Some(_) => return self.defer_detail(Barrier::Resources, "XObject/Name"),
            None => {
                if self.cold.used.get(&LiftCategory::XObject).is_some_and(|used| used.contains(&name)) { return self.defer_detail(Barrier::Resources, "XObject/Name"); }
                let mut fonts = self.font_resources.owners.clone();
                fonts.sort_unstable();
                let mut images = 0;
                let snapshot = self.snapshot.as_ref().expect("active typed PDF snapshot");
                let quote = if image { image_quote(source, owner, &mut images) } else { form_quote(source, owner, &fonts, 0, &mut images) };
                let bytes = quote + registration_bytes(&name) + table_growth(&snapshot.admitted_stream_roles, 8 + 2 * images)
                    + table_growth(&snapshot.images, images) + self.images.adoption_growth(images) + images * registration_bytes("");
                if let Some(denied) = budget(grant, bytes, 0) { return denied; }
                let lifted = self.cold_step(source, |lifter, _| lifter.lift_x_object(&name, &PdfObject::Ref(owner)));
                let Some((id, roles)) = lifted.filter(|(id, _)| *id == name) else { return self.defer_detail(Barrier::Resources, if image { "XObject/Image" } else { "XObject/Form" }); };
                drop(id);
                if !self.merge_roles(roles) { return ControlledLiftReceipt { event: Err("unable to grow the PDF stream role table"), ..work(0, 0) }; }
                if let Err(detail) = self.merge_typed() { return self.defer_detail(Barrier::Resources, detail); }
                self.cold_bytes += bytes;
                turn = work(bytes, 0);
            }
        }
        if resume == Resume::Inherit {
            let snapshot = self.snapshot.as_ref().expect("active typed PDF snapshot");
            let bound = if image { snapshot.images.iter().position(|typed| typed.id == name).is_some_and(|slot| self.images.push_page(slot)) } else { snapshot.forms.iter().position(|form| form.id == name).is_some_and(|slot| self.page_forms.push(slot)) };
            if !bound { return self.defer_turn(Barrier::Resources, turn); }
        }
        self.stage = Stage::Entries { location, category: Category::XObject, index: index + 1, resume };
        turn
    }

    /// 🚚️ Moves the images, fonts, graphics states, property lists, forms, shadings, patterns and colour spaces a reference step typed into the document lanes (their order is the reference order) and binds images, fonts, states and property lists to their owner slots.
    fn merge_typed(&mut self) -> Result<(), &'static str> {
        let typed = &mut self.cold.snapshot;
        let snapshot = self.snapshot.as_mut().expect("active typed PDF snapshot");
        let owner_of = |ids: &HashMap<(LiftCategory, ObjRef), String>, category: LiftCategory, id: &str| ids.iter().find(|((kind, _), value)| *kind == category && value.as_str() == id).map_or(ObjRef::default(), |((_, owner), _)| *owner);
        for image in typed.images.drain(..) {
            let owner = owner_of(&self.cold.ids, LiftCategory::XObject, &image.id);
            if !grow_table(&mut snapshot.images, 1) || !self.images.adopt(owner) { return Err("XObject/Image/Adopt"); }
            snapshot.images.push(image);
            self.registered_typed[0] += 1;
        }
        for font in typed.fonts.drain(..) {
            let owner = owner_of(&self.cold.ids, LiftCategory::Font, &font.id);
            if !grow_table(&mut snapshot.fonts, 1) || self.font_resources.bind(owner).is_none() { return Err("Font"); }
            snapshot.fonts.push(font);
            self.registered += 1;
        }
        for state in typed.ext_g_states.drain(..) {
            if !grow_table(&mut snapshot.ext_g_states, 1) || !grow_table(&mut self.states.states, 1) { return Err("ExtGState"); }
            self.states.states.push(owner_of(&self.cold.ids, LiftCategory::ExtGState, &state.id));
            snapshot.ext_g_states.push(state);
            self.registered_typed[1] += 1;
        }
        for properties in typed.properties.drain(..) {
            if !grow_table(&mut snapshot.properties, 1) || !grow_table(&mut self.states.properties, 1) { return Err("Properties"); }
            self.states.properties.push(owner_of(&self.cold.ids, LiftCategory::Properties, &properties.name));
            snapshot.properties.push(properties);
            self.registered_typed[2] += 1;
        }
        for form in typed.forms.drain(..) {
            if !grow_table(&mut snapshot.forms, 1) { return Err("XObject/Form"); }
            snapshot.forms.push(form);
        }
        for shading in typed.shadings.drain(..) {
            if !grow_table(&mut snapshot.shadings, 1) { return Err("Shading"); }
            snapshot.shadings.push(shading);
        }
        for pattern in typed.patterns.drain(..) {
            if !grow_table(&mut snapshot.patterns, 1) { return Err("Pattern"); }
            snapshot.patterns.push(pattern);
        }
        for space in typed.color_spaces.drain(..) {
            if !grow_table(&mut snapshot.color_spaces, 1) { return Err("ColorSpace"); }
            snapshot.color_spaces.push(space);
        }
        Ok(())
    }

    /// 🎛️ Lifts one direct (inline) graphics state of the page's resources through the reference builder under the page's scope, exactly as the reference lifter claims it (an id already used by another scope becomes `name_2`, ...), and records the rename the page program then applies.
    fn cold_state(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, location: ResourceLocation, index: usize, resume: Resume) -> ControlledLiftReceipt {
        if let Some(turn) = self.register_font(grant) { return turn; }
        if let Some(turn) = self.register_typed(grant) { return turn; }
        let mut found = None;
        read_category(source, location, Category::ExtGState, &mut |entries| {
            found = entries.and_then(|entries| entries.get(index)).filter(|entry| !holds_reference(&entry.value)).map(|entry| (entry.key.len(), direct_bytes(&entry.value)));
        });
        let Some((length, value_bytes)) = found else { return self.defer_detail(Barrier::Resources, "ExtGState"); };
        let snapshot = self.snapshot.as_ref().expect("active typed PDF snapshot");
        let bytes = 3 * value_bytes + 6 * (length + 8 + size_of::<String>()) + size_of::<(ObjRef, String)>() + 128 + size_of::<PdfExtGState>() + table_growth(&snapshot.ext_g_states, 1) + table_growth(&self.states.states, 1);
        if let Some(denied) = budget(grant, bytes, 0) { return denied; }
        let mut entry = None;
        read_category(source, location, Category::ExtGState, &mut |entries| { entry = entries.and_then(|entries| entries.get(index)).cloned(); });
        let Some(entry) = entry else { return self.defer(Barrier::Resources); };
        let scope = self.leaf.map_or(0, |leaf| leaf.reference.num as usize);
        let stepped = self.cold_step(source, |lifter, _| { lifter.scope = scope; lifter.lift_ext_g_state_entry(&entry.key, &entry.value) });
        let Some((id, roles)) = stepped else { return self.defer_detail(Barrier::Resources, "ExtGState"); };
        if !roles.is_empty() || self.cold.snapshot.ext_g_states.len() != 1 { return self.defer_detail(Barrier::Resources, "ExtGState"); }
        if let Err(detail) = self.merge_typed() { return self.defer_detail(Barrier::Resources, detail); }
        if id != entry.key { self.renames.ext_g_states.insert(entry.key, id); }
        self.cold_bytes += bytes;
        self.stage = Stage::Entries { location, category: Category::ExtGState, index: index + 1, resume };
        work(bytes, 0)
    }

    /// 🎨️ Lifts one colour space, shading or pattern resource (direct entries under the page's scope, exactly as the reference lifter claims them) through the reference builder over the persistent reference state, moves what it typed into the document lanes and records the rename the page program then applies; a resource claimed before only resolves its id.
    fn cold_resource(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, location: ResourceLocation, category: Category, index: usize, resume: Resume) -> ControlledLiftReceipt {
        if let Some(turn) = self.register_font(grant) { return turn; }
        if let Some(turn) = self.register_typed(grant) { return turn; }
        let mut found = None;
        read_category(source, location, category, &mut |entries| { found = entries.and_then(|entries| entries.get(index)).cloned(); });
        let Some(entry) = found else { return self.defer_detail(Barrier::Resources, category.key()); };
        if let Some(claimed) = entry.value.as_ref().and_then(|owner| self.cold.ids.get(&(category.lifted(), owner))).cloned() {
            self.rename_resource(category, &entry.key, &claimed);
            self.stage = Stage::Entries { location, category, index: index + 1, resume };
            return work(0, 0);
        }
        let mut fonts = self.font_resources.owners.clone();
        fonts.sort_unstable();
        let mut images = 0;
        let snapshot = self.snapshot.as_ref().expect("active typed PDF snapshot");
        let bytes = resource_quote(source, &entry.value, &fonts, &mut images) + 2 * registration_bytes(&entry.key) + 2 * entry.key.len()
            + table_growth(&snapshot.admitted_stream_roles, 8 + 2 * images) + table_growth(&snapshot.images, images) + self.images.adoption_growth(images) + images * registration_bytes("")
            + table_growth(&snapshot.color_spaces, 1) + table_growth(&snapshot.shadings, 1) + table_growth(&snapshot.patterns, 1);
        if let Some(denied) = budget(grant, bytes, 0) { return denied; }
        let scope = self.leaf.map_or(0, |leaf| leaf.reference.num as usize);
        let stepped = self.cold_step(source, |lifter, _| {
            lifter.scope = scope;
            match category {
                Category::ColorSpace => lifter.lift_colour_space_entry(&entry.key, &entry.value),
                Category::Shading => lifter.lift_shading_entry(&entry.key, &entry.value),
                _ => lifter.lift_pattern(&entry.key, &entry.value),
            }
        });
        let Some((id, roles)) = stepped else { return self.defer_detail(Barrier::Resources, category.key()); };
        if !self.merge_roles(roles) { return ControlledLiftReceipt { event: Err("unable to grow the PDF stream role table"), ..work(0, 0) }; }
        if let Err(detail) = self.merge_typed() { return self.defer_detail(Barrier::Resources, detail); }
        self.rename_resource(category, &entry.key, &id);
        self.cold_bytes += bytes;
        self.stage = Stage::Entries { location, category, index: index + 1, resume };
        work(bytes, 0)
    }

    /// 🏷️ Records that the page's authored resource name stands for another id (names are only renamed when they differ; an empty id is a resource the reference builder did not type).
    fn rename_resource(&mut self, category: Category, name: &str, id: &str) {
        if id.is_empty() || id == name { return; }
        let table = match category {
            Category::ColorSpace => &mut self.renames.color_spaces, Category::Shading => &mut self.renames.shadings, _ => &mut self.renames.patterns,
        };
        table.insert(name.to_owned(), id.to_owned());
    }

    /// 📝️ Lifts the interactive form: its default resources through the resource stages, its own entries through the reference builder, then its field tree one field or widget kid per unit (the reference order).
    fn acro_form(&mut self, source: &mut dyn ObjectSource, grant: CosGrant) -> ControlledLiftReceipt {
        let Some(form) = self.acro_form else { return self.next_lane(Lane::Form); };
        match self.form_step {
            FormStep::Start => {
                let mut shape = None;
                form.inspect(source, &mut |value| {
                    shape = small_dict(value).filter(|dict| dict.iter().enumerate().all(|(index, entry)| dict[..index].iter().all(|other| other.key != entry.key)))
                        .map(|dict| dict.iter().position(|entry| entry.key == "DR").map(|at| (at, dict[at].value.clone())));
                });
                let Some(resources) = shape else { return self.defer_detail(Barrier::Catalog, "AcroForm"); };
                self.form_step = FormStep::Head;
                let Some((at, value)) = resources else { return work(0, 0); };
                let location = match (value, form) {
                    (PdfObject::Ref(target), _) => ResourceLocation::Indirect(target),
                    (PdfObject::Dict(_), Node::Object(owner)) => ResourceLocation::Direct(PageAttribute { owner, entry: at }),
                    (PdfObject::Dict(_), Node::Entry { owner, .. }) => ResourceLocation::FormDefaults(owner),
                    _ => return self.defer_detail(Barrier::Catalog, "AcroForm/DR"),
                };
                self.form_resources = true;
                self.page_fonts.slots.clear();
                self.stage = Stage::Resources { location, resume: Resume::Inherit };
                work(0, 0)
            }
            FormStep::Head => {
                let mut dict = None;
                form.inspect(source, &mut |value| { dict = value.and_then(PdfObject::as_dict).map(<[PdfDictEntry]>::to_vec); });
                let Some(dict) = dict else { return self.defer_detail(Barrier::Catalog, "AcroForm"); };
                let snapshot = self.snapshot.as_ref().expect("active typed PDF snapshot");
                let mut fonts: Vec<String> = self.page_fonts.as_slice().iter().map(|slot| snapshot.fonts[usize::from(*slot)].id.clone()).collect();
                fonts.sort();
                let bytes = size_of::<PdfAcroForm>() + 3 * direct_bytes(&PdfObject::Dict(dict.clone())) + fonts.iter().map(|id| id.len() + size_of::<String>()).sum::<usize>();
                if let Some(denied) = budget(grant, bytes, 0) { return denied; }
                let Some((lifted, _)) = self.cold(source, false, |lifter| lifter.lift_acro_form_scalars(&dict, fonts)) else { return self.defer_detail(Barrier::Catalog, "AcroForm"); };
                self.snapshot_mut().acro_form = Some(lifted);
                self.page_fonts.slots.clear();
                self.walking = true;
                self.frames_len = 0;
                self.push_frame(Frame::at(Some(form)));
                self.form_step = FormStep::Fields;
                work(bytes, 0)
            }
            FormStep::Fields => self.form_field(source, grant),
        }
    }

    /// 🧩️ Lifts the next kid of the innermost form node: a widget-only kid adds its page annotation position to the field that owns it, a field kid is lifted without its kids and becomes the next node.
    fn form_field(&mut self, source: &mut dyn ObjectSource, grant: CosGrant) -> ControlledLiftReceipt {
        let Some(depth) = self.frames_len.checked_sub(1) else { return self.next_lane(Lane::Form); };
        let frame = self.frames[depth];
        let node = frame.node.expect("PDF form node");
        let mut item = None;
        with_array(source, node, if depth == 0 { "Fields" } else { "Kids" }, &mut |items| { item = items.and_then(|items| items.get(frame.kid as usize)).cloned(); });
        let Some(item) = item else { self.frames_len -= 1; return work(0, 0); };
        if depth > 0 {
            let widget = self.cold_step(source, |lifter, _| lifter.form_widget_kid(&item));
            let Some((widget, _)) = widget else { return self.defer_detail(Barrier::Catalog, "AcroForm/Kids"); };
            if let Some(widget) = widget {
                let fields = &self.snapshot.as_ref().expect("active typed PDF snapshot").acro_form.as_ref().expect("lifted AcroForm").fields;
                let parent = form_level(fields, depth - 1).last().expect("PDF form parent");
                let bytes = table_growth(&parent.widgets, 1);
                if let Some(denied) = budget(grant, bytes, 0) { return denied; }
                let acro = self.snapshot.as_mut().expect("active typed PDF snapshot").acro_form.as_mut().expect("lifted AcroForm");
                let parent = form_level_mut(&mut acro.fields, depth - 1).last_mut().expect("PDF form parent");
                if let Some(widget) = widget {
                    if !grow_table(&mut parent.widgets, 1) { return ControlledLiftReceipt { event: Err("unable to allocate PDF form widgets"), ..work(0, 0) }; }
                    parent.widgets.push(widget);
                }
                self.frames[depth].kid += 1;
                return work(bytes, 0);
            }
        }
        let PdfObject::Ref(target) = item else { return self.defer_detail(Barrier::Catalog, "AcroForm/Fields"); };
        let mut dict = None;
        source.inspect(target, &mut |value| { dict = value.and_then(PdfObject::as_dict).map(<[PdfDictEntry]>::to_vec); });
        let Some(dict) = dict.filter(|_| depth <= FORM_DEPTH) else { self.frames[depth].kid += 1; return work(0, 0); };
        let fields = &self.snapshot.as_ref().expect("active typed PDF snapshot").acro_form.as_ref().expect("lifted AcroForm").fields;
        let bytes = size_of::<PdfFormField>() + table_growth(form_level(fields, depth), 1) + table_growth(&self.snapshot.as_ref().expect("active typed PDF snapshot").admitted_stream_roles, 1) + 64
            + 3 * dict.iter().map(|entry| size_of::<PdfDictEntry>() + entry.key.len() + reach(source, &entry.value, 2)).sum::<usize>();
        if let Some(denied) = budget(grant, bytes, 0) { return denied; }
        let inherited = field_type_name(frame.field_type);
        let lifted = self.cold_step(source, |lifter, _| lifter.lift_form_field_head(&PdfObject::Ref(target), inherited));
        let Some((head, roles)) = lifted else { return self.defer_detail(Barrier::Catalog, "AcroForm/Fields"); };
        if !self.merge_roles(roles) { return ControlledLiftReceipt { event: Err("unable to grow the PDF stream role table"), ..work(0, 0) }; }
        self.frames[depth].kid += 1;
        let Some((field, field_type)) = head else { return work(bytes, 0); };
        let acro = self.snapshot.as_mut().expect("active typed PDF snapshot").acro_form.as_mut().expect("lifted AcroForm");
        if !push_grown(form_level_mut(&mut acro.fields, depth), field) { return ControlledLiftReceipt { event: Err("unable to allocate PDF form fields"), ..work(0, 0) }; }
        let mut child = Frame::at(Some(Node::Object(target)));
        child.field_type = field_type_code(field_type.as_deref());
        self.push_frame(child);
        work(bytes, 0)
    }

    /// 🪞️ Copies one operator of the lifted page into the content role, before the page's renames, so the role holds the authored program without parsing the content a second time.
    fn mirror(&mut self, grant: CosGrant, index: usize) -> ControlledLiftReceipt {
        let Some(op) = self.page.as_ref().expect("active typed page").content.get(index) else { self.stage = Stage::Rename(0); return work(0, 0); };
        if !self.mirror.has_reserved_slot() {
            let Ok(bytes) = self.mirror.next_allocation_bytes() else { return ControlledLiftReceipt { event: Err("unable to quote the PDF content role page"), ..work(0, 0) }; };
            if let Some(denied) = budget(grant, bytes, 0) { return denied; }
            return match self.mirror.reserve_one_cold(bytes) {
                Ok(progress) if progress.progressed => work(progress.allocated_bytes, 0),
                _ => ControlledLiftReceipt { event: Err("unable to allocate the PDF content role"), ..work(0, 0) },
            };
        }
        let bytes = operator_bytes(op).expect("collected PDF operators own measured backing");
        if let Some(denied) = budget(grant, bytes, 0) { return denied; }
        if self.mirror.push_reserved(op.clone()).is_err() { return ControlledLiftReceipt { event: Err("the PDF content role page lost its reserved slot"), ..work(0, 0) }; }
        self.stage = Stage::Mirror(index + 1);
        work(bytes, 0)
    }

    fn mirror_bytes(&self) -> usize { self.mirror.allocated_bytes() + self.mirror.iter().filter_map(operator_bytes).sum::<usize>() }

    /// 🔁️ Renames one operator of the lifted page through the page's resource renames, as the reference lifter's `rename_content` does for the whole page; the content role keeps the authored names.
    fn rename(&mut self, grant: CosGrant, index: usize) -> ControlledLiftReceipt {
        let renames = &self.renames;
        let content = &mut self.page.as_mut().expect("active typed page").content;
        let Some(op) = content.get_mut(index).filter(|_| !(renames.fonts.is_empty() && renames.x_objects.is_empty() && renames.ext_g_states.is_empty() && renames.shadings.is_empty() && renames.patterns.is_empty() && renames.color_spaces.is_empty() && renames.properties.is_empty())) else {
            self.stage = self.annotations.take().map_or(Stage::Publish, |array| Stage::AnnotationArray(array, 0, AFTER_CONTENT));
            return work(0, 0);
        };
        let turn = match rename_target(op, renames) {
            Some((name, id)) => {
                let released = name.capacity();
                if let Some(denied) = budget(grant, id.len(), released) { return denied; }
                *name = id.to_owned();
                work(name.capacity(), released)
            }
            None => work(0, 0),
        };
        self.stage = Stage::Rename(index + 1);
        turn
    }

    fn cold_finish(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, location: ResourceLocation, index: usize, owner: ObjRef, resume: Resume) -> ControlledLiftReceipt {
        if resume == Resume::Inherit { if let Some(turn) = self.page_fonts.room(grant) { return turn; } }
        let snapshot = self.snapshot.as_ref().expect("active typed PDF snapshot");
        let bytes = size_of::<PdfFont>() + table_growth(&snapshot.fonts, 1) + self.font_resources.owner_growth();
        if let Some(denied) = budget(grant, bytes, 0) { return denied; }
        if !grow_table(&mut self.snapshot_mut().fonts, 1) { return ControlledLiftReceipt { event: Err("unable to grow the PDF font table"), ..work(0, 0) }; }
        let lift = self.font_lift.take().expect("lifted reference font");
        let finished = self.cold_step(source, |lifter, _| lifter.finish_font(*lift));
        let nested = {
            let typed = &self.cold.snapshot;
            typed.fonts.len() != 1 || !typed.images.is_empty() || !typed.forms.is_empty() || !typed.ext_g_states.is_empty() || !typed.shadings.is_empty()
                || !typed.patterns.is_empty() || !typed.color_spaces.is_empty() || !typed.properties.is_empty()
        };
        let Some((_, roles)) = finished.filter(|_| !nested) else { return self.defer_detail(Barrier::Resources, if nested { "Font/Type3/Resources" } else { Self::font_detail(source, owner) }); };
        if !self.merge_roles(roles) { return ControlledLiftReceipt { event: Err("unable to grow the PDF stream role table"), ..work(0, 0) }; }
        let font = self.cold.snapshot.fonts.pop().expect("lifted reference font");
        self.snapshot_mut().fonts.push(font);
        let Some(slot) = self.font_resources.bind(owner) else { return ControlledLiftReceipt { event: Err("unable to grow the PDF font owner table"), ..work(0, 0) }; };
        self.registered += 1;
        let turn = work(bytes, 0);
        if resume == Resume::Inherit && !self.page_fonts.push(slot) { return self.defer_turn(Barrier::Resources, turn); }
        self.stage = Stage::Entries { location, category: Category::Font, index: index + 1, resume };
        turn
    }

    fn resume(&mut self, resume: Resume) {
        if resume == Resume::Inherit && self.form_resources {
            self.form_resources = false;
            self.stage = Stage::Navigation(Lane::Form);
            return;
        }
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
        let no_binding = self.page_resources.is_none();
        let mut step = None;
        source.inspect(owner, &mut |value| {
            let Some(dict) = small_dict(value) else { return; };
            step = if let Some(entry) = dict.iter().position(|entry| entry.key == "Resources") {
                Some(Stage::Resources { location: ResourceLocation::Direct(PageAttribute { owner, entry }), resume: Resume::Role })
            } else {
                dict_get(dict, "Parent").and_then(PdfObject::as_ref).map(|owner| Stage::Ancestor { owner, depth: depth + 1 }).or_else(|| no_binding.then_some(Stage::RoleStart))
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
            Some((Some(owner), _)) => Stage::RoleArray { owner: Some(owner), index: 0, count: usize::MAX },
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
        self.stage = Stage::RoleStream { stream, next: RoleNext { owner, index: index + 1, count } };
        work(0, 0)
    }

    fn program(&mut self, source: &mut dyn ObjectSource, grant: CosGrant) -> ControlledLiftReceipt {
        let owner = self.leaf.expect("active PDF page").reference;
        if self.program.is_none() {
            self.program = Some(PageProgramCursor::new(owner));
            return work(0, 0);
        }
        let program = self.program.as_mut().expect("active PDF page program");
        if let Some(PdfOp::PaintXObject { name }) = program.pending_operator() {
            let snapshot = self.snapshot.as_ref().expect("active PDF image snapshot");
            if !self.images.page_contains(snapshot, name.as_str()) && !self.page_forms.as_slice().iter().any(|slot| snapshot.forms[usize::from(*slot)].id == *name) {
                return self.defer(Barrier::Program);
            }
        }
        let turn = if !self.page_fonts.slots.is_empty() {
            let fonts = PageFontTable { fonts: &self.snapshot.as_ref().expect("active typed PDF snapshot").fonts, slots: self.page_fonts.as_slice(), codecs: &self.cold.font_codecs };
            program.advance_with_fonts(source, &fonts, grant, false)
        } else {
            program.advance(source, grant, false)
        };
        match turn.event {
            Ok(PageContentEvent::Complete) => {
                self.page.as_mut().expect("active typed page").content = program.take_program().expect("ready PDF operators");
                self.program = None;
                self.stage = if self.contents.is_some() { Stage::Mirror(0) } else { Stage::Rename(0) };
            }
            Ok(PageContentEvent::Deferred(_)) | Err(_) => return self.defer_turn(Barrier::Program, content_turn(turn)),
            _ => {}
        }
        content_turn(turn)
    }

    /// 🏃️ Runs the page program as one body run: a unit that moves source bytes (joining, lexing) costs one run unit per byte, every other unit the bytes it copies at [`Self::RUN_UNIT_COPY`] per run unit (at least one), so `work` counts run units; the run ends before an empty reservation, when the next unit no longer fits, when the program completes, and at any event but `More`, which a later unit of the same run would otherwise hide.
    fn program_run(&mut self, source: &mut dyn ObjectSource, grant: CosGrant) -> ControlledLiftReceipt {
        let mut run = ControlledLiftReceipt { work: 0, ..quote(Event::More) };
        while matches!(self.stage, Stage::Program) && run.work < grant.work {
            let program = self.program.as_ref();
            if run.work != 0 && program.is_some_and(PageProgramCursor::reserving_empty_backing) { break; }
            let remaining = grant.work - run.work;
            let cost = program.map_or(Some(0), PageProgramCursor::unit_copy).map(|bytes| bytes.div_ceil(Self::RUN_UNIT_COPY).max(1));
            if cost.is_some_and(|cost| cost > remaining) { break; }
            let unit = CosGrant { work: if cost.is_some() { 1 } else { remaining }, allocation_bytes: grant.allocation_bytes - run.allocated_bytes, release_bytes: grant.release_bytes - run.released_bytes };
            let turn = self.program(source, unit);
            if run.work != 0 && matches!(turn.event, Ok(Event::NeedsAllocation { .. } | Event::NeedsRelease { .. })) { break; }
            if run.work == 0 && turn.work == 0 && turn.event != Ok(Event::More) { return turn; }
            run.work += cost.unwrap_or(turn.work.max(1));
            run.allocated_bytes += turn.allocated_bytes;
            run.released_bytes += turn.released_bytes;
            if turn.event != Ok(Event::More) { run.event = turn.event; break; }
        }
        run
    }

    /// 📰️ Publishes the page and its content role; the role depends on the graph exactly as the reference lifter's: the contents closure, the font dictionary closure, then every claimed font's closure in object order.
    fn publish_page(&mut self, source: &mut dyn ObjectSource, grant: CosGrant) -> ControlledLiftReceipt {
        let mut acquired = 0;
        if let Some(contents) = self.contents {
            let identity = PdfGraphIdentity { owner: contents.owner, path: vec![PdfGraphPath::Entry { key: "Contents".into() }] };
            let mut dependencies = source.role_dependencies(&identity);
            if let Some(location) = self.page_resources.filter(|_| self.page_font_dictionary) {
                let font = PdfGraphPath::Entry { key: "Font".into() };
                let context = match location {
                    ResourceLocation::Indirect(owner) => PdfGraphIdentity { owner, path: vec![font] },
                    ResourceLocation::Direct(attribute) => PdfGraphIdentity { owner: attribute.owner, path: vec![PdfGraphPath::Entry { key: "Resources".into() }, font] },
                    ResourceLocation::FormDefaults(owner) => PdfGraphIdentity { owner, path: vec![PdfGraphPath::Entry { key: "AcroForm".into() }, PdfGraphPath::Entry { key: "DR".into() }, font] },
                };
                dependencies.extend(source.role_dependencies(&context));
            }
            let mut fonts = self.font_resources.owners.clone();
            fonts.sort_unstable();
            for font in fonts { dependencies.extend(source.role_dependencies(&root_identity(font))); }
            let mut seen = std::collections::HashSet::new();
            dependencies.retain(|dependency| seen.insert(dependency.clone()));
            drop(seen);
            dependencies.shrink_to_fit();
            acquired = table_growth(&self.snapshot_mut().admitted_stream_roles, 1) + identity_bytes(std::slice::from_ref(&identity)) + identity_bytes(&dependencies);
            if let Some(denied) = budget(grant, acquired, 0) { return denied; }
            if !grow_table(&mut self.snapshot_mut().admitted_stream_roles, 1) { return ControlledLiftReceipt { event: Err("unable to grow the PDF stream role table"), ..work(0, 0) }; }
            let content = take(&mut self.mirror);
            self.snapshot_mut().admitted_stream_roles.push(PdfAdmittedStreamRole { identity, dependencies, value: PdfStreamRoleValue::Operators { content } });
            self.content_roles += 1;
        }
        let page = self.page.take().expect("qualified projected page");
        self.snapshot_mut().pages.push(page);
        self.stage = Stage::Walk;
        work(acquired, 0)
    }

    fn snapshot_mut(&mut self) -> &mut PdfSnapshot { self.snapshot.as_mut().expect("active typed PDF snapshot") }

    fn defer(&mut self, reason: Barrier) -> ControlledLiftReceipt { self.defer_turn(reason, work(0, 0)) }

    fn defer_detail(&mut self, reason: Barrier, detail: &str) -> ControlledLiftReceipt {
        let length = detail.floor_char_boundary(MAX_COPY);
        self.detail[..length].copy_from_slice(&detail.as_bytes()[..length]);
        self.detail_len = length;
        self.defer(reason)
    }

    /// ♻️ Releases the lift's held tables and reference builder state in units that credit at most [`RELEASE_UNIT`] bytes each, dropping one table, one font codec or the reference snapshot per unit, so neither completion nor close asks one browser turn for more release than it grants; `None` once nothing is held.
    fn release_held(&mut self, grant: CosGrant, fonts: bool) -> Option<ControlledLiftReceipt> {
        let remaining = match self.releasing {
            Some(remaining) => remaining,
            None => {
                let held = self.navigation_bytes() + self.cold_bytes + self.states.held_bytes() + self.mirror_bytes() + self.images.held_bytes() + if fonts { self.font_resources.held_bytes() } else { 0 };
                if held == 0 { self.drop_held(fonts); return None; }
                held
            }
        };
        let piece = remaining.min(RELEASE_UNIT);
        if piece > grant.release_bytes { self.releasing = Some(remaining); return Some(quote(Event::NeedsRelease { bytes: piece })); }
        if remaining == piece { self.drop_held(fonts); self.releasing = None; } else { self.drop_next(); self.releasing = Some(remaining - piece); }
        Some(work(0, piece))
    }

    /// 🪓️ Drops the next held component in release order: navigation tables, the content role mirror, graphics-state owners, an unfinished font, one font codec, the reference snapshot.
    fn drop_next(&mut self) {
        if self.navigation_bytes() != 0 {
            self.page_refs = Vec::new();
            self.visited = Vec::new();
            self.page_fonts = PageSlots::default();
            self.page_forms = PageSlots::default();
        } else if self.mirror.allocated_bytes() != 0 {
            self.mirror = PdfContent::default();
        } else if self.states.held_bytes() != 0 {
            self.states = GraphicsStates::default();
        } else if self.images.held_bytes() != 0 {
            self.images.release();
        } else if self.font_lift.is_some() {
            self.font_lift = None;
        } else if let Some(id) = self.cold.font_codecs.keys().next().cloned() {
            self.cold.font_codecs.remove(&id);
        } else {
            drop(take(&mut self.cold.snapshot));
        }
    }

    /// 🧹️ Drops everything [`Self::release_held`] has credited.
    fn drop_held(&mut self, fonts: bool) {
        self.page_refs = Vec::new();
        self.page_fonts = PageSlots::default();
        self.page_forms = PageSlots::default();
        self.mirror = PdfContent::default();
        self.visited = Vec::new();
        self.release_cold();
        self.states = GraphicsStates::default();
        self.images.release();
        if fonts { self.font_resources.release(); }
    }

    /// 🧺️ Drops the reference builder state (ids, codecs, programs, an unfinished font) once its quoted bytes were credited.
    fn release_cold(&mut self) {
        self.cold = DocumentLiftState::default();
        self.font_lift = None;
        self.registered = 0;
        self.registered_typed = [0; 3];
        self.cold_bytes = 0;
    }

    /// 🧘️ Whether no page, page program or font is half built, so that [`Self::held_bytes`] is every byte the lift credits back before it completes.
    pub fn between_pages(&self) -> bool { matches!(self.stage, Stage::Walk) && self.page.is_none() && self.program.is_none() && self.font_lift.is_none() }

    /// 📏️ Bytes the lift holds only to build its result and credits back at completion or close: the navigation tables, the content role mirror, the graphics-state and image owner tables and the reference builder state.
    pub fn held_bytes(&self) -> usize { self.navigation_bytes() + self.cold_bytes + self.states.held_bytes() + self.mirror_bytes() + self.images.held_bytes() + self.font_resources.held_bytes() }

    fn navigation_bytes(&self) -> usize { self.page_refs.capacity() * size_of::<ObjRef>() + self.visited.capacity() * size_of::<u32>() + self.page_fonts.held_bytes() + self.page_forms.held_bytes() }

    fn catalog_value(&self, source: &mut dyn ObjectSource, key: &str) -> Option<PdfObject> {
        let mut found = None;
        source.inspect(self.catalog.expect("qualified PDF catalog"), &mut |value| { found = value.and_then(|value| value.dict_get(key)).cloned(); });
        found
    }

    /// 🧊️ Runs one reference-lifter builder over the page references in hand; refuses any builder that admitted files, ids or stream roles beyond `roles`.
    fn cold<R>(&mut self, source: &mut dyn ObjectSource, roles: bool, run: impl FnOnce(&mut Lifter<'_>) -> R) -> Option<(R, Vec<PdfAdmittedStreamRole>)> {
        let before = source.admitted_roles().len();
        let mut lifter = Lifter::bind(&mut *source, DocumentLiftState { page_refs: take(&mut self.page_refs), ..Default::default() });
        let result = run(&mut lifter);
        let state = lifter.unbind();
        self.page_refs = state.page_refs;
        let pure = state.admission_error.is_none() && state.snapshot.embedded_files.is_empty() && state.ids.is_empty() && state.admitted_stream_roles.is_empty();
        if !pure || (!roles && source.admitted_roles().len() != before) { return None; }
        let recorded = if roles { source.take_admitted_roles() } else { Vec::new() };
        Some((result, recorded))
    }

    fn next_lane(&mut self, lane: Lane) -> ControlledLiftReceipt {
        self.walking = false;
        self.frames_len = 0;
        self.stage = Stage::Navigation(match lane {
            Lane::Outlines => Lane::Names, Lane::Names => Lane::Dests, Lane::Dests => Lane::Labels, Lane::Labels => Lane::Form, Lane::Form => Lane::Preferences,
            Lane::Preferences => Lane::OpenAction, Lane::OpenAction => Lane::Metadata, Lane::Metadata => Lane::Extra, Lane::Extra | Lane::Release => Lane::Release,
        });
        work(0, 0)
    }

    fn push_frame(&mut self, frame: Frame) -> bool {
        let Some(slot) = self.frames.get_mut(self.frames_len) else { return false; };
        *slot = frame;
        self.frames_len += 1;
        true
    }

    /// 👣️ Grows the visited table of the current tree walk so the next [`Self::mark`] fits; a receipt means this unit was spent on the table.
    fn visit_room(&mut self, grant: CosGrant) -> Option<ControlledLiftReceipt> {
        let bytes = table_growth(&self.visited, 1);
        if bytes == 0 { return None; }
        if let Some(denied) = budget(grant, bytes, 0) { return Some(denied); }
        if !grow_table(&mut self.visited, 1) { return Some(ControlledLiftReceipt { event: Err("unable to allocate PDF tree walk"), ..work(0, 0) }); }
        Some(work(bytes, 0))
    }

    /// 👣️ Marks an object visited by the current tree walk after [`Self::visit_room`]; false when it was seen before.
    fn mark(&mut self, number: u32) -> bool {
        let Err(slot) = self.visited.binary_search(&number) else { return false; };
        self.visited.insert(slot, number);
        true
    }

    fn navigate(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, lane: Lane) -> ControlledLiftReceipt {
        match lane {
            Lane::Outlines => self.outlines(source, grant),
            Lane::Names | Lane::Labels => self.number_or_name_tree(source, grant, lane),
            Lane::Dests => self.catalog_dests(source, grant),
            Lane::Form => self.acro_form(source, grant),
            Lane::Preferences => {
                let preferences = match self.catalog_value(source, "ViewerPreferences") {
                    Some(PdfObject::Ref(target)) => source.get(target),
                    other => other,
                };
                if let Some(PdfObject::Dict(preferences)) = preferences {
                    let bytes = size_of::<PdfViewerPreferences>() + 2 * preferences.iter().map(|entry| size_of::<PdfDictEntry>() + entry.key.len() + direct_bytes(&entry.value)).sum::<usize>();
                    if let Some(denied) = budget(grant, bytes, 0) { return denied; }
                    self.snapshot_mut().viewer_preferences = Some(lift_viewer_preferences(&preferences));
                    let _ = self.next_lane(lane);
                    return work(bytes, 0);
                }
                self.next_lane(lane)
            }
            Lane::OpenAction => {
                let Some(open) = self.catalog_value(source, "OpenAction") else { return self.next_lane(lane); };
                let bytes = size_of::<PdfOpenAction>() + 3 * reach(source, &open, 3);
                if let Some(denied) = budget(grant, bytes, 0) { return denied; }
                let Some((action, _)) = self.cold(source, false, |lifter| lifter.lift_open_action(&open)) else { return self.defer_detail(Barrier::Catalog, "OpenAction"); };
                self.snapshot_mut().open_action = action;
                let _ = self.next_lane(lane);
                work(bytes, 0)
            }
            Lane::Metadata => {
                let Some(value) = self.catalog_value(source, "Metadata") else { return self.next_lane(lane); };
                let mut body = 0;
                if let PdfObject::Ref(target) = &value { source.inspect(*target, &mut |stream| { body = stream.map_or(0, |stream| reach_direct(stream) + if let PdfObject::Stream { data, .. } = stream { 3 * data.len() } else { 0 }); }); }
                let bytes = 2 * body + size_of::<PdfAdmittedStreamRole>() + 4 * size_of::<PdfGraphIdentity>() + table_growth(&self.snapshot_mut().admitted_stream_roles, NAVIGATION_ROLES);
                if let Some(denied) = budget(grant, bytes, 0) { return denied; }
                if !grow_table(&mut self.snapshot_mut().admitted_stream_roles, NAVIGATION_ROLES) { return ControlledLiftReceipt { event: Err("unable to grow the PDF stream role table"), ..work(0, 0) }; }
                let Some((text, roles)) = self.cold(source, true, |lifter| lifter.lift_metadata(&value)) else { return self.defer_detail(Barrier::Catalog, "Metadata"); };
                let snapshot = self.snapshot.as_mut().expect("active typed PDF snapshot");
                let at = snapshot.admitted_stream_roles.len() - self.content_roles;
                if roles.len() > snapshot.admitted_stream_roles.capacity() - snapshot.admitted_stream_roles.len() { return self.defer_detail(Barrier::Roles, "Metadata"); }
                for (offset, role) in roles.into_iter().enumerate() { snapshot.admitted_stream_roles.insert(at + offset, role); }
                snapshot.metadata = text;
                let _ = self.next_lane(lane);
                work(bytes, 0)
            }
            Lane::Extra => {
                let mut extra = Vec::new();
                let mut bytes = 0;
                source.inspect(self.catalog.expect("qualified PDF catalog"), &mut |value| {
                    let entries = value.and_then(PdfObject::as_dict).unwrap_or(&[]);
                    bytes = entries.iter().filter(|entry| !CATALOG_KEYS.contains(&entry.key.as_str())).map(|entry| size_of::<PdfDictEntry>() + entry.key.len() + direct_bytes(&entry.value)).sum();
                    if bytes <= grant.allocation_bytes { extra = extra_entries(entries, &CATALOG_KEYS); }
                });
                if let Some(denied) = budget(grant, bytes, 0) { return denied; }
                self.snapshot_mut().catalog_extra = extra;
                let _ = self.next_lane(lane);
                work(bytes, 0)
            }
            Lane::Release => {
                if let Some(turn) = self.release_held(grant, true) { return turn; }
                self.stage = Stage::Ready;
                ControlledLiftReceipt { event: Ok(Event::Complete), ..work(0, 0) }
            }
        }
    }

    fn outlines(&mut self, source: &mut dyn ObjectSource, grant: CosGrant) -> ControlledLiftReceipt {
        if !self.walking {
            let first = match self.catalog_value(source, "Outlines") {
                None => return self.next_lane(Lane::Outlines),
                Some(PdfObject::Ref(root)) => {
                    let mut first = None;
                    source.inspect(root, &mut |value| { first = value.and_then(|value| value.dict_get("First")).map(PdfObject::as_ref); });
                    first
                }
                Some(root) => root.dict_get("First").map(PdfObject::as_ref),
            };
            let Some(first) = first else { return self.next_lane(Lane::Outlines); };
            self.visited.clear();
            self.walking = true;
            self.push_frame(Frame::at(first.map(Node::Object)));
            return work(0, 0);
        }
        let Some(depth) = self.frames_len.checked_sub(1) else { return self.next_lane(Lane::Outlines); };
        let Some(Node::Object(item)) = self.frames[depth].node else { self.frames_len -= 1; return work(0, 0); };
        if let Some(turn) = self.visit_room(grant) { return turn; }
        if self.visited.binary_search(&item.num).is_ok() || depth > OUTLINE_DEPTH {
            self.mark(item.num);
            self.frames_len -= 1;
            return work(0, 0);
        }
        let mut dict = None;
        source.inspect(item, &mut |value| { dict = value.and_then(PdfObject::as_dict).map(<[PdfDictEntry]>::to_vec); });
        let Some(dict) = dict else { self.mark(item.num); self.frames_len -= 1; return work(0, 0); };
        let estimate = size_of::<PdfOutlineItem>() + 2 * size_of::<PdfAction>() + 3 * dict.iter()
            .filter(|entry| !matches!(entry.key.as_str(), "Parent" | "Prev" | "Next" | "First" | "Last"))
            .map(|entry| entry.key.len() + size_of::<PdfDictEntry>() + reach(source, &entry.value, 2)).sum::<usize>();
        let outlines = &self.snapshot.as_ref().expect("active typed PDF snapshot").outlines;
        let growth = table_growth(outline_level(outlines, depth), 1);
        if let Some(denied) = budget(grant, growth + estimate, 0) { return denied; }
        let Some((lifted, _)) = self.cold(source, false, |lifter| lifter.lift_outline_item(&dict, Vec::new())) else { return self.defer_detail(Barrier::Catalog, "Outlines"); };
        let retained = growth + outline_item_heap(&lifted).min(estimate);
        let level = outline_level_mut(&mut self.snapshot.as_mut().expect("active typed PDF snapshot").outlines, depth);
        if !push_grown(level, lifted) { return ControlledLiftReceipt { event: Err("unable to allocate PDF outline items"), ..work(0, 0) }; }
        self.mark(item.num);
        self.frames[depth].node = dict_get(&dict, "Next").and_then(PdfObject::as_ref).map(Node::Object);
        if let Some(first) = dict_get(&dict, "First") {
            if !self.push_frame(Frame::at(first.as_ref().map(Node::Object))) { return self.defer_detail(Barrier::Catalog, "Outlines"); }
        }
        work(retained, 0)
    }

    fn number_or_name_tree(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, lane: Lane) -> ControlledLiftReceipt {
        let catalog = self.catalog.expect("qualified PDF catalog");
        if !self.walking {
            let root = if lane == Lane::Labels {
                self.catalog_value(source, "PageLabels").and_then(|labels| Node::within(catalog, None, TreeKey::PageLabels, &labels))
            } else {
                match self.catalog_value(source, "Names") {
                    None => None,
                    Some(PdfObject::Ref(names)) => {
                        let mut found = (None, false);
                        source.inspect(names, &mut |value| {
                            let dict = value.and_then(PdfObject::as_dict).unwrap_or(&[]);
                            found = (dict_get(dict, "Dests").and_then(|dests| Node::within(names, None, TreeKey::Dests, dests)), dict_get(dict, "EmbeddedFiles").is_some());
                        });
                        if found.1 { return self.defer_detail(Barrier::Catalog, "EmbeddedFiles"); }
                        found.0
                    }
                    Some(names) => {
                        if names.dict_get("EmbeddedFiles").is_some() { return self.defer_detail(Barrier::Catalog, "EmbeddedFiles"); }
                        names.dict_get("Dests").and_then(|dests| Node::within(catalog, Some(TreeKey::Names), TreeKey::Dests, dests))
                    }
                }
            };
            let Some(root) = root else { return self.next_lane(lane); };
            if let Some(turn) = self.visit_room(grant) { return turn; }
            self.visited.clear();
            if let Node::Object(root) = root { self.mark(root.num); }
            self.walking = true;
            self.push_frame(Frame::at(Some(root)));
            return work(0, 0);
        }
        let Some(depth) = self.frames_len.checked_sub(1) else { return self.next_lane(lane); };
        let frame = self.frames[depth];
        let node = frame.node.expect("PDF tree node");
        if !frame.names {
            let mut kid = Ok(None);
            with_array(source, node, "Kids", &mut |items| {
                kid = match items.and_then(|items| items.get(frame.kid as usize)) {
                    None => Ok(None),
                    Some(PdfObject::Ref(target)) => Ok(Some(*target)),
                    Some(_) => Err(()),
                };
            });
            match kid {
                Err(()) => return self.defer_detail(Barrier::Catalog, if lane == Lane::Labels { "PageLabels" } else { "Names" }),
                Ok(None) => self.frames[depth].names = true,
                Ok(Some(target)) => {
                    if let Some(turn) = self.visit_room(grant) { return turn; }
                    if self.mark(target.num) && !self.push_frame(Frame::at(Some(Node::Object(target)))) { return self.defer_detail(Barrier::Catalog, "Names"); }
                    self.frames[depth].kid += 1;
                }
            }
            return work(0, 0);
        }
        let mut pair = None;
        with_array(source, node, if lane == Lane::Labels { "Nums" } else { "Names" }, &mut |items| {
            pair = items.and_then(|items| items.chunks(2).nth(frame.pair as usize)).and_then(|pair| match pair {
                [key, value] => Some((key.clone(), value.clone())),
                _ => None,
            });
        });
        let Some((key, value)) = pair else { self.frames_len -= 1; return work(0, 0); };
        self.frames[depth].pair += 1;
        let estimate = 3 * (direct_bytes(&key) + reach(source, &key, 1) + reach(source, &value, 2));
        if lane == Lane::Labels {
            let PdfObject::Int(index) = key else { return work(0, 0); };
            let labels = &self.snapshot.as_ref().expect("active typed PDF snapshot").page_labels;
            let growth = table_growth(labels, 1);
            if let Some(denied) = budget(grant, growth + estimate, 0) { self.frames[depth].pair -= 1; return denied; }
            let Some((label, _)) = self.cold(source, false, |lifter| lifter.lift_page_label(index, &value)) else { return self.defer_detail(Barrier::Catalog, "PageLabels"); };
            let retained = growth + label.prefix.as_ref().map_or(0, String::capacity).min(estimate);
            if !push_grown(&mut self.snapshot_mut().page_labels, label) { return ControlledLiftReceipt { event: Err("unable to allocate PDF page labels"), ..work(0, 0) }; }
            return work(retained, 0);
        }
        let destinations = &self.snapshot.as_ref().expect("active typed PDF snapshot").named_destinations;
        let growth = table_growth(destinations, 1);
        if let Some(denied) = budget(grant, growth + size_of::<PdfNamedDestination>() + estimate, 0) { self.frames[depth].pair -= 1; return denied; }
        let Some((named, _)) = self.cold(source, false, |lifter| {
            let name = lifter.text(Some(&key))?;
            lifter.lift_destination(&value).map(|destination| PdfNamedDestination { name, destination })
        }) else { return self.defer_detail(Barrier::Catalog, "Dests"); };
        let Some(named) = named else { return work(0, 0); };
        let retained = growth + named_destination_heap(&named).min(estimate);
        if !push_grown(&mut self.snapshot_mut().named_destinations, named) { return ControlledLiftReceipt { event: Err("unable to allocate PDF named destinations"), ..work(0, 0) }; }
        work(retained, 0)
    }

    fn catalog_dests(&mut self, source: &mut dyn ObjectSource, grant: CosGrant) -> ControlledLiftReceipt {
        let catalog = self.catalog.expect("qualified PDF catalog");
        if !self.walking {
            let Some(node) = self.catalog_value(source, "Dests").and_then(|dests| Node::within(catalog, None, TreeKey::Dests, &dests)) else { return self.next_lane(Lane::Dests); };
            self.walking = true;
            self.push_frame(Frame::at(Some(node)));
            return work(0, 0);
        }
        let frame = self.frames[0];
        let mut entry = None;
        frame.node.expect("PDF destination dictionary").inspect(source, &mut |value| {
            entry = value.and_then(PdfObject::as_dict).and_then(|entries| entries.get(frame.pair as usize)).cloned();
        });
        let Some(entry) = entry else { return self.next_lane(Lane::Dests); };
        let destinations = &self.snapshot.as_ref().expect("active typed PDF snapshot").named_destinations;
        let growth = table_growth(destinations, 1);
        let estimate = 3 * (entry.key.len() + reach(source, &entry.value, 2));
        if let Some(denied) = budget(grant, growth + size_of::<PdfNamedDestination>() + estimate, 0) { return denied; }
        self.frames[0].pair += 1;
        let Some((destination, _)) = self.cold(source, false, |lifter| lifter.lift_destination(&entry.value)) else { return self.defer_detail(Barrier::Catalog, "Dests"); };
        let Some(destination) = destination else { return work(0, 0); };
        let named = PdfNamedDestination { name: entry.key, destination };
        let retained = growth + named_destination_heap(&named).min(estimate);
        if !push_grown(&mut self.snapshot_mut().named_destinations, named) { return ControlledLiftReceipt { event: Err("unable to allocate PDF named destinations"), ..work(0, 0) }; }
        work(retained, 0)
    }

    fn defer_turn(&mut self, reason: Barrier, turn: ControlledLiftReceipt) -> ControlledLiftReceipt {
        self.stage = Stage::Deferred(reason);
        ControlledLiftReceipt { event: Ok(Event::Deferred(reason)), ..turn }
    }
}

/// 🔡️ The fonts a page binds: inline-typed fonts decode through the controlled codes, fonts typed by the reference builder through its own codec.
struct PageFontTable<'a> { fonts: &'a [PdfFont], slots: &'a [u16], codecs: &'a HashMap<String, FontCodec> }

impl PageFontTable<'_> {
    fn bound(&self, name: &str) -> Option<&PdfFont> {
        self.slots.iter().map(|slot| &self.fonts[usize::from(*slot)]).find(|font| font.id == name)
    }

    fn inline(&self, name: &str) -> Option<&PdfFont> { self.bound(name).filter(|font| !self.codecs.contains_key(&font.id)) }
}

impl FontTable for PageFontTable<'_> {
    fn font(&self, name: &str) -> Option<&FontCodec> { self.codecs.get(&self.bound(name)?.id) }

    fn controlled_type1(&self, name: &str) -> Option<PdfBaseEncoding> {
        let PdfFontKind::Type1 { encoding, program: None, .. } = &self.inline(name)?.kind else { return None; };
        self.controlled_code_width(name).is_none().then_some(encoding.base.unwrap_or(PdfBaseEncoding::Standard))
    }

    fn controlled_type1_text(&self, name: &str, byte: u8) -> Option<&'static str> {
        let PdfFontKind::Type1 { encoding, program: None, .. } = &self.inline(name)?.kind else { return None; };
        controlled_simple_text(encoding.base.unwrap_or(PdfBaseEncoding::Standard), &encoding.differences, byte)
    }

    fn controlled_code_width(&self, name: &str) -> Option<PdfFixedCodeWidth> {
        match &self.inline(name)?.kind {
            PdfFontKind::Type1 { base_font, program: None, .. } if matches!(base_font.as_str(), "Symbol" | "ZapfDingbats") => Some(PdfFixedCodeWidth::One),
            PdfFontKind::Type1 { program: Some(PdfFontProgram::OpenType { .. }), .. } => Some(PdfFixedCodeWidth::One),
            PdfFontKind::Type0 { descendant, .. } if descendant.true_type => Some(PdfFixedCodeWidth::Two),
            _ => None,
        }
    }
}

struct ResourceShape { fonts: bool, populated: bool }

/// 🏷️ Names the first resource category the controlled shape does not qualify.
fn unsupported_resource(value: Option<&PdfObject>) -> &'static str {
    let entries = value.and_then(PdfObject::as_dict).unwrap_or(&[]);
    for entry in entries {
        let qualified = matches!(&entry.value, PdfObject::Dict(_) | PdfObject::Ref(_));
        match entry.key.as_str() {
            "Font" if !qualified => return "Font", "XObject" if !qualified => return "XObject",
            "ExtGState" if !qualified => return "ExtGState", "Properties" if !qualified => return "Properties",
            "ColorSpace" if !qualified => return "ColorSpace", "Shading" if !qualified => return "Shading", "Pattern" if !qualified => return "Pattern",
            "Font" | "XObject" | "ExtGState" | "ColorSpace" | "Shading" | "Pattern" | "Properties" | "ProcSet" | "Encoding" => {}
            _ => return "Resources",
        }
    }
    "Resources"
}

fn resource_shape(value: Option<&PdfObject>) -> Option<ResourceShape> {
    if empty_dictionary(value) { return Some(ResourceShape { fonts: false, populated: false }); }
    let dict = small_dict(value)?;
    let mut shape = ResourceShape { fonts: false, populated: false };
    for (index, entry) in dict.iter().enumerate() {
        if dict[..index].iter().any(|other| other.key == entry.key) { return None; }
        match entry.key.as_str() {
            "Font" | "XObject" | "ExtGState" | "ColorSpace" | "Shading" | "Pattern" | "Properties" => {
                match &entry.value {
                    PdfObject::Dict(_) | PdfObject::Ref(_) => {}
                    _ => return None,
                }
                shape.populated = true;
                if entry.key == "Font" { shape.fonts = true; }
            }
            "ProcSet" | "Encoding" => {}
            _ => return None,
        }
    }
    Some(shape)
}

/// 📖️ Reads entry `index` of a resource category, following one indirect category dictionary.
fn read_entry(source: &mut dyn ObjectSource, location: ResourceLocation, category: Category, index: usize) -> Option<Entry> {
    let mut entry = None;
    let mut indirect = None;
    location.inspect(source, &mut |value| match value.and_then(|value| value.dict_get(category.key())) {
        Some(PdfObject::Ref(target)) => indirect = Some(*target),
        _ => entry = category_entry(value, category, index),
    });
    if let Some(target) = indirect {
        source.inspect(target, &mut |value| entry = value.and_then(PdfObject::as_dict).and_then(|entries| category_item(entries, index, category)));
    }
    entry
}

fn category_entry(value: Option<&PdfObject>, category: Category, index: usize) -> Option<Entry> {
    let Some(resources) = value.and_then(PdfObject::as_dict) else { return empty_dictionary(value).then_some(Entry::End); };
    let Some(subdictionary) = resources.iter().find(|entry| entry.key == category.key()) else { return Some(Entry::End); };
    category_item(subdictionary.value.as_dict()?, index, category)
}

fn category_item(entries: &[PdfDictEntry], index: usize, category: Category) -> Option<Entry> {
    let Some(entry) = entries.get(index) else { return Some(Entry::End); };
    if entries[..index].iter().any(|other| other.key == entry.key) { return None; }
    let length = entry.key.len();
    if length == 0 || length > MAX_COPY { return None; }
    let Some(owner) = entry.value.as_ref() else { return (category.is_cold() || matches!(entry.value, PdfObject::Dict(_))).then_some(Entry::Direct); };
    let mut name = [0; MAX_COPY];
    name[..length].copy_from_slice(entry.key.as_bytes());
    Some(Entry::Item { name, length, owner })
}

/// 📖️ Lends the entries of one resource category, following one indirect category dictionary.
fn read_category(source: &mut dyn ObjectSource, location: ResourceLocation, category: Category, read: &mut dyn FnMut(Option<&[PdfDictEntry]>)) {
    let mut indirect = None;
    location.inspect(source, &mut |value| match value.and_then(|value| value.dict_get(category.key())) {
        Some(PdfObject::Ref(target)) => indirect = Some(*target),
        other => read(other.and_then(PdfObject::as_dict)),
    });
    if let Some(target) = indirect { source.inspect(target, &mut |value| read(value.and_then(PdfObject::as_dict))); }
}

/// 🔗️ Whether a direct value refers to another object anywhere inside it.
fn holds_reference(value: &PdfObject) -> bool {
    match value {
        PdfObject::Ref(_) => true,
        PdfObject::Array(items) => items.iter().any(holds_reference),
        PdfObject::Dict(entries) | PdfObject::Stream { dict: entries, .. } => entries.iter().any(|entry| holds_reference(&entry.value)),
        _ => false,
    }
}

/// 🎛️ Admits scalar extended graphics states and plain property lists under one exact owner each.
#[derive(Default)]
struct GraphicsStates { states: Vec<ObjRef>, properties: Vec<ObjRef> }

struct NoObjects;

impl ObjectSource for NoObjects {
    fn get(&mut self, _: ObjRef) -> Option<PdfObject> { None }
}

impl GraphicsStates {
    /// 📦️ Bytes of the two owner tables.
    fn held_bytes(&self) -> usize { (self.states.capacity() + self.properties.capacity()) * size_of::<ObjRef>() }

    fn admit_ext_g_state(&mut self, source: &mut dyn ObjectSource, snapshot: &mut PdfSnapshot, name: &str, owner: ObjRef, grant: CosGrant) -> Option<(ControlledLiftReceipt, bool)> {
        let count = snapshot.ext_g_states.len();
        if let Some(slot) = self.states[..count].iter().position(|existing| *existing == owner) {
            return (snapshot.ext_g_states[slot].id == name).then(|| (work(0, 0), true));
        }
        if snapshot.ext_g_states.iter().any(|state| state.id == name) { return None; }
        let mut text = None;
        source.inspect(owner, &mut |value| { text = scalar_graphics_state(value); });
        let bytes = size_of::<PdfExtGState>() + name.len() + size_of::<String>() + text? + table_growth(&snapshot.ext_g_states, 1) + table_growth(&self.states, 1);
        if let Some(denied) = budget(grant, bytes, 0) { return Some((denied, false)); }
        if !grow_table(&mut snapshot.ext_g_states, 1) || !grow_table(&mut self.states, 1) { return Some((ControlledLiftReceipt { event: Err("unable to grow the PDF graphics state table"), ..work(0, 0) }, false)); }
        let mut form = |_: &PdfObject, _: &mut dyn ObjectSource| -> Option<String> { None };
        let mut font = |_: &PdfObject, _: &mut dyn ObjectSource| -> Option<String> { None };
        let mut state = None;
        source.inspect(owner, &mut |value| {
            state = value.and_then(PdfObject::as_dict).map(|dict| lift_ext_g_state(name, dict, &mut NoObjects, &mut form, &mut font));
        });
        snapshot.ext_g_states.push(state?);
        self.states.push(owner);
        Some((work(bytes, 0), true))
    }

    /// 🏷️ A property list that is an optional content group is typed as its group id, like the reference lifter; any other must hold atomic values only.
    fn admit_properties(&mut self, source: &mut dyn ObjectSource, snapshot: &mut PdfSnapshot, name: &str, owner: ObjRef, group: Option<&str>, grant: CosGrant) -> Option<(ControlledLiftReceipt, bool)> {
        let count = snapshot.properties.len();
        if let Some(slot) = self.properties[..count].iter().position(|existing| *existing == owner) {
            return (snapshot.properties[slot].name == name).then(|| (work(0, 0), true));
        }
        if snapshot.properties.iter().any(|properties| properties.name == name) { return None; }
        let mut text = None;
        match group {
            Some(group) => text = Some(size_of::<PdfDictEntry>() + "OCG".len() + group.len()),
            None => source.inspect(owner, &mut |value| { text = plain_properties(value); }),
        }
        let bytes = size_of::<PdfNamedProperties>() + name.len() + text? + table_growth(&snapshot.properties, 1) + table_growth(&self.properties, 1);
        if let Some(denied) = budget(grant, bytes, 0) { return Some((denied, false)); }
        if !grow_table(&mut snapshot.properties, 1) || !grow_table(&mut self.properties, 1) { return Some((ControlledLiftReceipt { event: Err("unable to grow the PDF property list table"), ..work(0, 0) }, false)); }
        let mut properties = group.map(|group| PdfNamedProperties { name: name.to_owned(), entries: vec![PdfDictEntry::new("OCG", PdfObject::Name(group.to_owned()))] });
        if group.is_none() {
            source.inspect(owner, &mut |value| {
                properties = value.and_then(PdfObject::as_dict).map(|dict| PdfNamedProperties { name: name.to_owned(), entries: dict.to_vec() });
            });
        }
        snapshot.properties.push(properties?);
        self.properties.push(owner);
        Some((work(bytes, 0), true))
    }
}

fn atomic_bytes(value: &PdfObject) -> Option<usize> {
    match value {
        PdfObject::Null | PdfObject::Bool(_) | PdfObject::Int(_) | PdfObject::Real(_) | PdfObject::Ref(_) => Some(0),
        PdfObject::Name(text) | PdfObject::Text(text) => (text.len() <= MAX_COPY).then_some(text.len()),
        PdfObject::Str(bytes) => (bytes.len() <= MAX_COPY).then_some(bytes.len()),
        _ => None,
    }
}

fn scalar_graphics_state(value: Option<&PdfObject>) -> Option<usize> {
    let dict = small_dict(value)?;
    let mut text = 0;
    for (index, entry) in dict.iter().enumerate() {
        if dict[..index].iter().any(|other| other.key == entry.key) { return None; }
        match (entry.key.as_str(), &entry.value) {
            ("Type", PdfObject::Name(name)) if name == "ExtGState" => {}
            ("LW" | "ML" | "CA" | "ca" | "FL" | "SM", PdfObject::Int(_) | PdfObject::Real(_)) => {}
            ("LC" | "LJ" | "OPM", PdfObject::Int(_)) => {}
            ("OP" | "op" | "AIS" | "SA" | "TK", PdfObject::Bool(_)) => {}
            ("RI" | "BM", PdfObject::Name(name)) if name.len() <= MAX_COPY => text += name.len() + size_of::<String>(),
            _ => return None,
        }
    }
    Some(text)
}

fn plain_properties(value: Option<&PdfObject>) -> Option<usize> {
    let dict = small_dict(value)?;
    let mut bytes = dict.len() * size_of::<PdfDictEntry>();
    for entry in dict {
        if entry.key.len() > MAX_COPY { return None; }
        bytes += entry.key.len() + atomic_bytes(&entry.value)?;
    }
    Some(bytes)
}

fn root_identity(owner: ObjRef) -> PdfGraphIdentity { PdfGraphIdentity { owner, path: Vec::new() } }

/// 📏️ Quotes the identities a role depends on, their path segments included.
fn identity_bytes(identities: &[PdfGraphIdentity]) -> usize {
    identities.len() * size_of::<PdfGraphIdentity>() + identities.iter().map(|identity| identity.path.len() * size_of::<PdfGraphPath>() + identity.path.iter().map(|path| match path {
        PdfGraphPath::Entry { key } => key.len(),
        _ => 0,
    }).sum::<usize>()).sum::<usize>()
}

/// 📏️ Quotes one font id in the reference builder's id and name tables.
fn registration_bytes(id: &str) -> usize { 2 * (id.len() + size_of::<String>()) + size_of::<(ObjRef, String)>() + 64 }

/// 📏️ Quotes one content program typed by the reference builder and recorded as its `Operators` role: the copied body, the operator vector grown by doubling plus the role's exact clone, and twice the operand and inline-image heap the program measures to; the worst case (one operator per two body bytes, each owning a byte twice) where the body does not measure.
fn program_quote(body: &[u8]) -> usize {
    let length = body.len();
    let Some(extent) = content_extent(body) else { return 2 * (size_of::<PdfOp>() * (length / 2 + 1) + length); };
    let grown = extent.operators.next_power_of_two().max(4);
    length + (grown + extent.operators) * size_of::<PdfOp>() + 2 * (extent.operand_bytes + extent.inline_bytes)
}

/// 📏️ Quotes a form typed by the reference builder: its measured program and role ([`program_quote`]), its role's dependencies on every claimed font, its fonts, graphics states and property lists, and its nested forms one level down.
fn form_quote(source: &mut dyn ObjectSource, owner: ObjRef, fonts: &[ObjRef], depth: usize, images: &mut usize) -> usize {
    let mut program = program_quote(&[]);
    let mut resources = None;
    source.inspect(owner, &mut |value| if let Some(PdfObject::Stream { data, dict, .. }) = value {
        program = program_quote(&data.to_vec_owner());
        resources = dict_get(dict, "Resources").cloned();
    });
    let mut dependencies = source.role_dependencies(&root_identity(owner));
    for font in fonts { dependencies.extend(source.role_dependencies(&root_identity(*font))); }
    let mut bytes = program + identity_bytes(&dependencies) + size_of::<PdfFormXObject>();
    drop(dependencies);
    let resources = match resources {
        Some(PdfObject::Ref(target)) => { let mut found = None; source.inspect(target, &mut |value| found = value.cloned()); found }
        other => other,
    };
    let Some(PdfObject::Dict(categories)) = resources else { return bytes; };
    bytes += 2 * direct_bytes(&PdfObject::Dict(categories.clone()));
    for category in &categories {
        let entries = match &category.value {
            PdfObject::Ref(target) => { let mut found = Vec::new(); source.inspect(*target, &mut |value| found = value.and_then(PdfObject::as_dict).map(<[PdfDictEntry]>::to_vec).unwrap_or_default()); found }
            PdfObject::Dict(entries) => entries.clone(),
            _ => Vec::new(),
        };
        for entry in &entries {
            let Some(target) = entry.value.as_ref() else { continue; };
            bytes += match category.key.as_str() {
                "Font" if !fonts.contains(&target) => font_quote(source, target),
                "XObject" if is_image(source, target) => image_quote(source, target, images),
                "XObject" if depth < 1 => form_quote(source, target, fonts, depth + 1, images),
                "ExtGState" | "Properties" => 2 * reach(source, &entry.value, 2),
                "Shading" | "Pattern" | "ColorSpace" => 2 * (reach(source, &entry.value, 3) + reach_streams(source, &entry.value, 3)),
                _ => 0,
            };
        }
    }
    bytes
}

/// 🪜️ The deepest field the reference lifter keeps: its fields nest at most this many levels below the form.
const FORM_DEPTH: usize = 32;

/// 🏷️ The field type a kid inherits, as a code (`Btn`, `Tx`, `Ch`, `Sig`; every other type types a container like no type).
fn field_type_code(field_type: Option<&str>) -> u8 {
    match field_type { Some("Btn") => 1, Some("Tx") => 2, Some("Ch") => 3, Some("Sig") => 4, _ => 0 }
}

fn field_type_name(code: u8) -> Option<&'static str> {
    match code { 1 => Some("Btn"), 2 => Some("Tx"), 3 => Some("Ch"), 4 => Some("Sig"), _ => None }
}

fn form_level(fields: &Vec<PdfFormField>, depth: usize) -> &Vec<PdfFormField> {
    (0..depth).fold(fields, |level, _| &level.last().expect("PDF form parent").children)
}

fn form_level_mut(fields: &mut Vec<PdfFormField>, depth: usize) -> &mut Vec<PdfFormField> {
    (0..depth).fold(fields, |level, _| &mut level.last_mut().expect("PDF form parent").children)
}

/// 📏️ Quotes the growth of the widget position table by one entry: nothing while it has room, else a table of the next power of two buckets.
fn annotation_reference_growth(positions: &HashMap<ObjRef, (u32, u32)>) -> usize {
    if positions.len() < positions.capacity() { 0 } else { (positions.capacity() * 2 + 8).next_power_of_two() * (size_of::<(ObjRef, (u32, u32))>() + 1) }
}

/// 🖼️ Whether an object is an image XObject.
fn is_image(source: &mut dyn ObjectSource, owner: ObjRef) -> bool {
    let mut image = false;
    source.inspect(owner, &mut |value| { image = value.and_then(|value| value.dict_get("Subtype")).and_then(PdfObject::as_name) == Some("Image"); });
    image
}

/// 📏️ Quotes an image typed by the reference builder: its body copied up to four times (decoded copy, packed samples, the role's clone, the typed image), its dictionary and colour space streams, and its soft mask and stencil mask images behind it.
fn image_quote(source: &mut dyn ObjectSource, owner: ObjRef, images: &mut usize) -> usize {
    *images += 1;
    let (mut bytes, mut masks, mut count) = (size_of::<PdfImage>() + size_of::<PdfAdmittedStreamRole>(), [ObjRef::default(); 2], 0);
    let mut spaces = None;
    source.inspect(owner, &mut |value| if let Some(value @ PdfObject::Stream { dict, data, .. }) = value {
        bytes += 4 * data.len() + 3 * reach_direct(value);
        spaces = dict_get(dict, "ColorSpace").cloned();
        for key in ["SMask", "Mask"] {
            if let Some(PdfObject::Ref(mask)) = dict_get(dict, key) { masks[count] = *mask; count += 1; }
        }
    });
    if let Some(space) = spaces { bytes += 3 * (reach(source, &space, 3) + reach_streams(source, &space, 3)); }
    for mask in masks.into_iter().take(count) { if is_image(source, mask) { bytes += image_quote(source, mask, images); } }
    bytes
}

/// 📏️ Quotes the stream bodies a value reaches through up to `depth` reference hops (colour space profiles and palettes, function samples, mesh data), each copied up to three times by the reference builder; references beyond the inline window count as a fixed stream.
fn reach_streams(source: &mut dyn ObjectSource, value: &PdfObject, depth: usize) -> usize {
    fn collect(value: &PdfObject, frontier: &mut [ObjRef; 16], len: &mut usize, overflow: &mut usize) {
        match value {
            PdfObject::Ref(target) => match frontier.get_mut(*len) {
                Some(slot) => { *slot = *target; *len += 1; }
                None => *overflow += 1,
            },
            PdfObject::Array(items) => items.iter().for_each(|item| collect(item, frontier, len, overflow)),
            PdfObject::Dict(entries) | PdfObject::Stream { dict: entries, .. } => entries.iter().for_each(|entry| collect(&entry.value, frontier, len, overflow)),
            _ => {}
        }
    }
    let (mut frontier, mut len, mut overflow, mut bytes) = ([ObjRef::default(); 16], 0, 0, 0);
    collect(value, &mut frontier, &mut len, &mut overflow);
    for _ in 0..depth {
        let (current, count) = (frontier, len);
        len = 0;
        for target in &current[..count] {
            source.inspect(*target, &mut |value| if let Some(value) = value {
                if let PdfObject::Stream { data, .. } = value { bytes += data.len(); }
                collect(value, &mut frontier, &mut len, &mut overflow);
            });
        }
    }
    3 * bytes + overflow * 16384
}

/// 📏️ Quotes a colour space, shading or pattern resource typed by the reference builder: the objects and streams it reaches, and for a tiling pattern its program, role and own resources.
fn resource_quote(source: &mut dyn ObjectSource, value: &PdfObject, fonts: &[ObjRef], images: &mut usize) -> usize {
    let mut bytes = 3 * (direct_bytes(value) + reach(source, value, 4)) + reach_streams(source, value, 4);
    if let PdfObject::Ref(owner) = value {
        let mut tiling = false;
        source.inspect(*owner, &mut |object| { tiling = matches!(object, Some(PdfObject::Stream { .. })) && object.and_then(|object| object.dict_get("PatternType")).and_then(PdfObject::as_i64) == Some(1); });
        if tiling { bytes += form_quote(source, *owner, fonts, 0, images); }
    }
    bytes
}

/// 📏️ Quotes the appearance streams of an annotation: every normal, rollover and down entry, a single stream or a dictionary of states, each lifted as a form.
fn appearance_quote(source: &mut dyn ObjectSource, appearance: &PdfObject, fonts: &[ObjRef], images: &mut usize) -> usize {
    let mut dict = match appearance { PdfObject::Dict(entries) => Some(entries.clone()), _ => None };
    if let PdfObject::Ref(target) = appearance { source.inspect(*target, &mut |value| { dict = value.and_then(PdfObject::as_dict).map(<[PdfDictEntry]>::to_vec); }); }
    let Some(dict) = dict else { return 0; };
    let mut bytes = 2 * direct_bytes(&PdfObject::Dict(dict.clone()));
    for entry in dict.iter().filter(|entry| matches!(entry.key.as_str(), "N" | "R" | "D")) {
        let mut states = Vec::new();
        match &entry.value {
            PdfObject::Ref(target) => source.inspect(*target, &mut |value| match value {
                Some(PdfObject::Stream { .. }) => states.push(*target),
                Some(PdfObject::Dict(entries)) => states.extend(entries.iter().filter_map(|state| state.value.as_ref())),
                _ => {}
            }),
            PdfObject::Dict(entries) => states.extend(entries.iter().filter_map(|state| state.value.as_ref())),
            _ => {}
        }
        for state in states { bytes += form_quote(source, state, fonts, 0, images); }
    }
    bytes
}

/// 📏️ Quotes a font typed by the reference builder: the dictionaries and arrays it reads (descriptor, encoding, widths, glyph table, resources; their typed copies are smaller than the objects), the streams it decodes (programs, Unicode and character maps; the admitted copy is the parsed one), and a full simple-font code table.
fn font_quote(source: &mut dyn ObjectSource, owner: ObjRef) -> usize {
    const KEYS: [&str; 13] = ["FontDescriptor", "ToUnicode", "Encoding", "DescendantFonts", "FontFile", "FontFile2", "FontFile3", "CIDToGIDMap", "Widths", "W", "CharProcs", "Resources", "CIDSystemInfo"];
    let mut frontier = [ObjRef::default(); 16];
    let (mut count, mut bytes, mut streams) = (1, 0, 0);
    frontier[0] = owner;
    let mut index = 0;
    while index < count {
        source.inspect(frontier[index], &mut |value| match value {
            Some(PdfObject::Stream { data, .. }) => streams += data.len(),
            Some(value) => {
                bytes += direct_bytes(value);
                let mut push = |reference: ObjRef| if let Some(slot) = frontier.get_mut(count) { *slot = reference; count += 1; } else { bytes += 4096; };
                for key in KEYS {
                    match value.dict_get(key) {
                        Some(PdfObject::Ref(reference)) => push(*reference),
                        Some(PdfObject::Array(items)) if key == "DescendantFonts" => items.iter().filter_map(PdfObject::as_ref).for_each(&mut push),
                        _ => {}
                    }
                }
            }
            None => {}
        });
        index += 1;
    }
    bytes + streams + size_of::<FontCodec>() + 256 * (2 * size_of::<String>() + 64)
}

/// 📏️ Quotes the heap bytes a direct value occupies once cloned (references count as values, they are not followed).
fn direct_bytes(value: &PdfObject) -> usize {
    size_of::<PdfObject>() + match value {
        PdfObject::Name(text) | PdfObject::Text(text) => text.len(),
        PdfObject::Str(bytes) => bytes.len(),
        PdfObject::Array(items) => items.iter().map(direct_bytes).sum(),
        PdfObject::Dict(entries) | PdfObject::Stream { dict: entries, .. } => entries.iter().map(|entry| size_of::<PdfDictEntry>() + entry.key.len() + direct_bytes(&entry.value)).sum(),
        _ => 0,
    }
}

/// 📏️ The heap bytes a typed destination retains: only a named destination owns any.
fn destination_heap(destination: &PdfDestination) -> usize { match destination { PdfDestination::Named { name } => name.capacity(), _ => 0 } }

/// 📏️ The heap bytes a typed file specification retains.
fn file_specification_heap(file: &PdfFileSpecification) -> usize { match file { PdfFileSpecification::Path { path } => path.capacity(), PdfFileSpecification::Embedded { file } => file.capacity() } }

/// 📏️ The heap bytes a typed string list retains.
fn strings_heap(items: &Vec<String>) -> usize { items.capacity() * size_of::<String>() + items.iter().map(String::capacity).sum::<usize>() }

/// 📏️ The heap bytes the entries of a typed dictionary retain: the entry table, each key and the heap of each value.
fn entries_heap(entries: &Vec<PdfDictEntry>) -> usize { entries.capacity() * size_of::<PdfDictEntry>() + entries.iter().map(|entry| entry.key.capacity() + direct_bytes(&entry.value) - size_of::<PdfObject>()).sum::<usize>() }

/// 📏️ The heap bytes a typed action and its chained actions retain.
fn action_heap(action: &PdfAction) -> usize {
    action.next.capacity() * size_of::<PdfAction>() + action.next.iter().map(action_heap).sum::<usize>() + match &action.kind {
        PdfActionKind::GoTo { destination } | PdfActionKind::GoToEmbedded { destination, .. } => destination_heap(destination),
        PdfActionKind::GoToRemote { file, destination, .. } => file_specification_heap(file) + destination_heap(destination),
        PdfActionKind::Launch { file, .. } | PdfActionKind::ImportData { file } => file_specification_heap(file),
        PdfActionKind::Thread { file, .. } => file.as_ref().map_or(0, file_specification_heap),
        PdfActionKind::Uri { uri, .. } => uri.capacity(),
        PdfActionKind::Sound { sound, .. } => sound.capacity(),
        PdfActionKind::Movie { annotation, operation } => annotation.as_ref().map_or(0, String::capacity) + operation.as_ref().map_or(0, String::capacity),
        PdfActionKind::Hide { annotations, .. } => strings_heap(annotations),
        PdfActionKind::Named { name } => name.capacity(),
        PdfActionKind::SubmitForm { url, fields, .. } => url.capacity() + strings_heap(fields),
        PdfActionKind::ResetForm { fields, .. } => strings_heap(fields),
        PdfActionKind::JavaScript { script } => script.capacity(),
        PdfActionKind::SetOptionalContentState { states, .. } => entries_heap(states),
        PdfActionKind::Rendition { entries } | PdfActionKind::Transition { entries } | PdfActionKind::GoTo3dView { entries } => entries_heap(entries),
        PdfActionKind::Unknown { subtype, entries } => subtype.capacity() + entries_heap(entries),
    }
}

/// 📏️ The heap bytes an outline item retains beside its own slot in the level table: its title, destination, action and extra entries (its children are charged as they are lifted).
fn outline_item_heap(item: &PdfOutlineItem) -> usize {
    item.title.capacity() + item.destination.as_ref().map_or(0, destination_heap) + item.action.as_ref().map_or(0, action_heap) + entries_heap(&item.extra)
}

/// 📏️ The heap bytes a named destination retains beside its own slot in the table.
fn named_destination_heap(named: &PdfNamedDestination) -> usize { named.name.capacity() + destination_heap(&named.destination) }

/// 📏️ The heap bytes a link annotation without appearance or markup retains beside its own slot in the page's table; `None` for every other annotation, which keeps its admission quote.
fn annotation_heap(annotation: &PdfAnnotation) -> Option<usize> {
    let PdfAnnotationKind::Link { action, destination, highlight, quad_points } = &annotation.kind else { return None; };
    if annotation.appearance.is_some() || annotation.markup.is_some() { return None; }
    let text = |text: &Option<String>| text.as_ref().map_or(0, String::capacity);
    Some(action.as_ref().map_or(0, action_heap) + destination.as_ref().map_or(0, destination_heap) + text(highlight) + quad_points.capacity() * size_of::<f64>()
        + text(&annotation.contents) + text(&annotation.name) + text(&annotation.modified) + text(&annotation.appearance_state) + text(&annotation.optional_content)
        + annotation.border.as_ref().map_or(0, |border| text(&border.style) + border.dash.as_ref().map_or(0, |dash| dash.capacity() * size_of::<f64>()))
        + annotation.color.capacity() * size_of::<f64>() + entries_heap(&annotation.extra))
}

/// 📏️ Quotes the stream dictionary and the direct entries of a stream object without its body.
fn reach_direct(value: &PdfObject) -> usize {
    match value { PdfObject::Stream { dict, .. } => direct_bytes(&PdfObject::Dict(Vec::new())) + dict.iter().map(|entry| entry.key.len() + direct_bytes(&entry.value)).sum::<usize>(), other => direct_bytes(other) }
}

/// 🛰️ Quotes a value and the objects it references up to `depth` reference hops, without stream bodies; references beyond the inline window count as a fixed object.
fn reach(source: &mut dyn ObjectSource, value: &PdfObject, depth: usize) -> usize {
    #[derive(Default)]
    struct Reach { refs: [ObjRef; 16], len: usize, bytes: usize }
    impl Reach {
        fn value(&mut self, value: &PdfObject) {
            self.bytes += size_of::<PdfObject>();
            match value {
                PdfObject::Ref(target) => match self.refs.get_mut(self.len) {
                    Some(slot) => { *slot = *target; self.len += 1; }
                    None => self.bytes += 4096,
                },
                PdfObject::Name(text) | PdfObject::Text(text) => self.bytes += text.len(),
                PdfObject::Str(bytes) => self.bytes += bytes.len(),
                PdfObject::Array(items) => items.iter().for_each(|item| self.value(item)),
                PdfObject::Dict(entries) | PdfObject::Stream { dict: entries, .. } => for entry in entries {
                    self.bytes += size_of::<PdfDictEntry>() + entry.key.len();
                    self.value(&entry.value);
                },
                _ => {}
            }
        }
    }
    let mut walk = Reach::default();
    walk.value(value);
    for _ in 0..depth {
        let (refs, len) = (walk.refs, walk.len);
        walk.len = 0;
        for target in &refs[..len] { source.inspect(*target, &mut |value| if let Some(value) = value { walk.value(value); }); }
    }
    walk.bytes
}

/// 📥️ Pushes one item after its growth was quoted with [`table_growth`]; false when the allocator refuses.
fn push_grown<T>(items: &mut Vec<T>, item: T) -> bool {
    if !grow_table(items, 1) { return false; }
    items.push(item);
    true
}

fn outline_level(outlines: &Vec<PdfOutlineItem>, depth: usize) -> &Vec<PdfOutlineItem> {
    (0..depth).fold(outlines, |level, _| &level.last().expect("PDF outline parent").children)
}

fn outline_level_mut(outlines: &mut Vec<PdfOutlineItem>, depth: usize) -> &mut Vec<PdfOutlineItem> {
    (0..depth).fold(outlines, |level, _| &mut level.last_mut().expect("PDF outline parent").children)
}

/// 🗂️ Reads the array under `key` of a tree node, following one indirect array.
fn with_array(source: &mut dyn ObjectSource, node: Node, key: &str, read: &mut dyn FnMut(Option<&[PdfObject]>)) {
    let mut indirect = None;
    node.inspect(source, &mut |value| match value.and_then(|value| value.dict_get(key)) {
        Some(PdfObject::Ref(target)) => indirect = Some(*target),
        Some(PdfObject::Array(items)) => read(Some(items.as_slice())),
        _ => read(None),
    });
    if let Some(target) = indirect { source.inspect(target, &mut |value| read(value.and_then(PdfObject::as_array))); }
}

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

fn image_turn(turn: CosReceipt) -> ControlledLiftReceipt {
    let event = turn.status.map(|status| match status {
        CosStatus::NeedsAllocation { bytes } => Event::NeedsAllocation { bytes },
        CosStatus::NeedsRelease { bytes } => Event::NeedsRelease { bytes },
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
