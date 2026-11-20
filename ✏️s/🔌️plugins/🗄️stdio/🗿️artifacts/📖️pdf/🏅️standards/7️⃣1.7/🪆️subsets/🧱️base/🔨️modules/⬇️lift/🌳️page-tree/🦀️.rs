//! 🌳️ Resumable page-tree traversal over the already retained COS graph.

use crate::standards::v1_7::subsets::base::modules::lexer::CosGrant;
use crate::standards::v1_7::subsets::base::schema::graph_source::ObjectSource;
use crate::standards::v1_7::subsets::base::schema::snapshot::{ObjRef, PdfObject};
use semio_s_artifact_stdio_pdf_xref_stream::{XrefIndex, XrefIndexStatus, XrefRow};

const MAX_LEVELS: usize = 65;

/// 📍 Addresses a dictionary entry without cloning its native value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PageAttribute {
    pub owner: ObjRef,
    pub entry: usize,
}

/// 🧬 Nearest declarations inherited along the traversal path.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct PageInheritance {
    pub media_box: Option<PageAttribute>,
    pub crop_box: Option<PageAttribute>,
    pub rotate: Option<PageAttribute>,
    pub resources: Option<PageAttribute>,
}

/// 🌿 One leaf and the positions of its inheritable attributes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PageTreeLeaf {
    pub reference: ObjRef,
    pub inheritance: PageInheritance,
}

/// 🎫 The walker returns one semantic event at most per turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PageTreeEvent {
    More,
    NeedsAllocation { bytes: usize },
    NeedsRelease { bytes: usize },
    BranchResources(PageAttribute),
    Page(PageTreeLeaf),
    Complete,
    Cancelled,
    Closed,
}

/// 🪙 Reports observed work and index backing effects.
#[derive(Debug, PartialEq, Eq)]
#[must_use]
pub struct PageTreeReceipt {
    pub event: Result<PageTreeEvent, &'static str>,
    pub work: usize,
    pub allocated_bytes: usize,
    pub released_bytes: usize,
}

#[derive(Clone, Copy)]
enum Kids { Inline(usize), Indirect(ObjRef) }

#[derive(Clone, Copy)]
enum Stage { Admit, Seen, Dictionary, Children }

#[derive(Clone, Copy)]
struct Frame {
    reference: ObjRef,
    stage: Stage,
    inheritance: PageInheritance,
    local_resources: Option<PageAttribute>,
    kids: Option<Kids>,
    position: usize,
    is_pages: Option<bool>,
    contents: bool,
}

impl Frame {
    fn new(reference: ObjRef, inheritance: PageInheritance) -> Self {
        Self {
            reference, stage: Stage::Admit, inheritance, local_resources: None,
            kids: None, position: 0, is_pages: None, contents: false,
        }
    }

    fn scan(&mut self, source: &mut dyn ObjectSource) -> bool {
        let mut inspected = false;
        source.inspect(self.reference, &mut |node| {
            let Some(dict) = node.and_then(PdfObject::as_dict) else { return };
            inspected = true;
            let Some(entry) = dict.get(self.position) else { return; };
            let place = PageAttribute { owner: self.reference, entry: self.position };
            match entry.key.as_str() {
                "Type" => { self.is_pages.get_or_insert(entry.value.as_name() == Some("Pages")); }
                "Kids" => {
                    self.kids.get_or_insert_with(|| entry.value.as_ref().map_or(Kids::Inline(place.entry), Kids::Indirect));
                }
                "Contents" => self.contents = true,
                "MediaBox" if self.inheritance.media_box.is_none_or(|value| value.owner != self.reference) => self.inheritance.media_box = Some(place),
                "CropBox" if self.inheritance.crop_box.is_none_or(|value| value.owner != self.reference) => self.inheritance.crop_box = Some(place),
                "Rotate" if self.inheritance.rotate.is_none_or(|value| value.owner != self.reference) => self.inheritance.rotate = Some(place),
                "Resources" if self.local_resources.is_none() => {
                    self.inheritance.resources = Some(place);
                    self.local_resources = Some(place);
                }
                _ => {}
            }
            self.position += 1;
        });
        inspected
    }

    fn next_kid(&mut self, source: &mut dyn ObjectSource) -> Option<Option<ObjRef>> {
        let mut result = None;
        let mut read = |value: Option<&PdfObject>| {
            result = value.and_then(PdfObject::as_array).and_then(|kids| kids.get(self.position)).map(PdfObject::as_ref);
        };
        match self.kids? {
            Kids::Inline(entry) => source.inspect(self.reference, &mut |node| {
                read(node.and_then(PdfObject::as_dict).and_then(|dict| dict.get(entry)).map(|entry| &entry.value));
            }),
            Kids::Indirect(reference) => source.inspect(reference, &mut read),
        }
        self.position += 1;
        result
    }
}

#[derive(Clone, Copy)]
enum Life { Active, Complete, Cancelled, Failed(&'static str), Closing, Closed }

/// 🧭 Walks the PDF Kids arrays in order, borrowing each node and admitting deduplication backing.
#[must_use]
pub struct PageTreeCursor {
    stack: [Option<Frame>; MAX_LEVELS],
    depth: usize,
    visited: XrefIndex,
    life: Life,
}

impl PageTreeCursor {
    /// 🌱 Opens the first branch at its cross-reference identity.
    pub fn new(root: ObjRef) -> Self {
        let mut stack = [None; MAX_LEVELS];
        stack[0] = Some(Frame::new(root, PageInheritance::default()));
        Self { stack, depth: 1, visited: XrefIndex::default(), life: Life::Active }
    }

    /// ▶️ Inspects at most the granted dictionary, Kids, or visited-index actions.
    pub fn advance(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, cancelled: bool) -> PageTreeReceipt {
        if cancelled && matches!(self.life, Life::Active) { self.life = Life::Cancelled; }
        let mut receipt = more();
        receipt.event = match self.life {
            Life::Active => self.advance_active(source, grant, &mut receipt),
            Life::Complete => Ok(PageTreeEvent::Complete),
            Life::Cancelled => Ok(PageTreeEvent::Cancelled),
            Life::Failed(reason) => Err(reason),
            Life::Closing => Ok(PageTreeEvent::More),
            Life::Closed => Ok(PageTreeEvent::Closed),
        };
        if let Err(reason) = receipt.event { self.life = Life::Failed(reason); }
        receipt
    }

    fn advance_active(&mut self, source: &mut dyn ObjectSource, grant: CosGrant, receipt: &mut PageTreeReceipt) -> Result<PageTreeEvent, &'static str> {
        while receipt.work < grant.work {
            let Some(index) = self.depth.checked_sub(1) else {
                self.life = Life::Complete;
                return Ok(PageTreeEvent::Complete);
            };
            let frame = self.stack[index].as_mut().expect("page tree frame");
            if matches!(frame.stage, Stage::Seen) {
                let turn = self.visited.advance(CosGrant {
                    work: grant.work - receipt.work,
                    allocation_bytes: grant.allocation_bytes - receipt.allocated_bytes,
                    release_bytes: grant.release_bytes - receipt.released_bytes,
                }, false);
                receipt.work += turn.work;
                receipt.allocated_bytes += turn.allocated_bytes;
                receipt.released_bytes += turn.released_bytes;
                match turn.status? {
                    XrefIndexStatus::Inserted => frame.stage = Stage::Dictionary,
                    XrefIndexStatus::AlreadyKnown => self.pop(),
                    status => return index_event(status),
                }
                continue;
            }
            receipt.work += 1;
            match frame.stage {
                Stage::Admit => {
                    if !self.visited.offer(XrefRow { number: frame.reference.num, entry: None }) {
                        return Err("page traversal already owns a pending reference");
                    }
                    frame.stage = Stage::Seen;
                }
                Stage::Dictionary => {
                    let position = frame.position;
                    if !frame.scan(source) { self.pop(); continue; }
                    if position != frame.position { continue; }
                    let branch = frame.is_pages.unwrap_or(false) || (frame.kids.is_some() && !frame.contents);
                    if branch {
                        frame.stage = Stage::Children;
                        frame.position = 0;
                        if let Some(resources) = frame.local_resources {
                            return Ok(PageTreeEvent::BranchResources(resources));
                        }
                    } else {
                        let leaf = PageTreeLeaf { reference: frame.reference, inheritance: frame.inheritance };
                        self.pop();
                        return Ok(PageTreeEvent::Page(leaf));
                    }
                }
                Stage::Children => match frame.next_kid(source) {
                    Some(Some(reference)) if self.depth < MAX_LEVELS => {
                        self.stack[self.depth] = Some(Frame::new(reference, frame.inheritance));
                        self.depth += 1;
                    }
                    Some(_) => {}
                    None => self.pop(),
                },
                Stage::Seen => unreachable!("page references are admitted separately"),
            }
        }
        Ok(PageTreeEvent::More)
    }

    fn pop(&mut self) {
        self.depth -= 1;
        self.stack[self.depth] = None;
    }

    /// ♻️ Begins closing the traversal's visited backing.
    pub fn begin_close(&mut self) {
        if matches!(self.life, Life::Closed) { return; }
        self.life = Life::Closing;
        self.visited.begin_close();
    }

    /// 🧹 Settles the index's remaining native allocations.
    pub fn close_step(&mut self, grant: CosGrant) -> PageTreeReceipt {
        if matches!(self.life, Life::Closed) { return PageTreeReceipt { event: Ok(PageTreeEvent::Closed), ..more() }; }
        if !matches!(self.life, Life::Closing) { return more(); }
        let turn = self.visited.close_step(grant);
        if turn.status == Ok(XrefIndexStatus::Closed) { self.life = Life::Closed; }
        PageTreeReceipt {
            event: turn.status.and_then(index_event),
            work: turn.work, allocated_bytes: turn.allocated_bytes, released_bytes: turn.released_bytes,
        }
    }
}

fn more() -> PageTreeReceipt {
    PageTreeReceipt { event: Ok(PageTreeEvent::More), work: 0, allocated_bytes: 0, released_bytes: 0 }
}

fn index_event(status: XrefIndexStatus) -> Result<PageTreeEvent, &'static str> {
    match status {
        XrefIndexStatus::More => Ok(PageTreeEvent::More),
        XrefIndexStatus::NeedsAllocation { bytes } => Ok(PageTreeEvent::NeedsAllocation { bytes }),
        XrefIndexStatus::NeedsRelease { bytes } => Ok(PageTreeEvent::NeedsRelease { bytes }),
        XrefIndexStatus::Cancelled => Ok(PageTreeEvent::Cancelled),
        XrefIndexStatus::Closed => Ok(PageTreeEvent::Closed),
        XrefIndexStatus::Inserted | XrefIndexStatus::AlreadyKnown => Err("unexpected page traversal index event"),
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
pub(crate) mod tests;
