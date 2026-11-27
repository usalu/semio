//! 🧭️ Caller-owned semantic staging over a retained PDF graph.

use super::*;
use crate::standards::v1_7::subsets::base::io::error::PResult;

#[path = "🧭️controlled/🦀️.rs"]
mod controlled;
pub use controlled::{ControlledDocumentLiftCursor, ControlledLiftBarrier, ControlledLiftEvent, ControlledLiftPartial, ControlledLiftReceipt};

/// ⬇️ Lifts a whole document from its trailer.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lift_document(trailer: &[PdfDictEntry], declared_version: &str, source: &mut dyn ObjectSource) -> PResult<PdfSnapshot> {
    let lifter=lift_document_with(trailer, declared_version, source);if let Some(error)=lifter.admission_error {return Err(error)}Ok(lifter.snapshot)
}

/// ⬇️ Lifts a whole document and hands back the lifter (its reference → id maps included).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lift_document_with<'s>(trailer: &[PdfDictEntry], declared_version: &str, source: &'s mut dyn ObjectSource) -> Lifter<'s> {
    lift_with_cursor(DocumentLiftCursor::new(), trailer, declared_version, source)
}

/// ⬇️ Lifts a document slice whose resource ids continue `claims` (the ids the slices before it claimed) and logs its own claims into `claims` when the lifter is unbound with [`Lifter::unbind`] or read through [`Lifter::take_claims`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn lift_document_claiming<'s>(trailer: &[PdfDictEntry], declared_version: &str, source: &'s mut dyn ObjectSource, claims: ResourceClaims) -> Lifter<'s> {
    let mut cursor = DocumentLiftCursor::new();
    if let Some(state) = cursor.lifter.as_mut() {
        let ResourceClaims { ids, used, inline_ids, log } = claims;
        (state.ids, state.used, state.inline_ids, state.claim_log) = (ids, used, inline_ids, Some(log));
    }
    lift_with_cursor(cursor, trailer, declared_version, source)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn lift_with_cursor<'s>(mut cursor: DocumentLiftCursor, trailer: &[PdfDictEntry], declared_version: &str, source: &'s mut dyn ObjectSource) -> Lifter<'s> {
    let grant = DocumentLiftGrant {
        traversal: CosGrant { work: 4096, allocation_bytes: usize::MAX, release_bytes: usize::MAX }, semantic: true,
    };
    loop {
        match cursor.advance(trailer, declared_version, source, grant, false).status {
            DocumentLiftStatus::Complete => return Lifter::bind(source, cursor.take_result().expect("complete document lift")),
            DocumentLiftStatus::Failed => {
                while cursor.close_traversal_step(grant.traversal).is_some() {}
                while cursor.close_page_content_step(grant.traversal).is_some() {}
                let partial = cursor.take_partial().expect("failed document lift");
                return Lifter::bind(source, partial.lifter);
            }
            DocumentLiftStatus::More => {}
            _ => unreachable!("synchronous lift admitted both traversal and semantic work"),
        }
    }
}

/// 🧩 One existing semantic lift operation. A page or resource operation can decode nested data.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentLiftAction {
    Header,
    Info,
    OptionalContent,
    EmbeddedFiles,
    RecordPage,
    BranchResources,
    ProjectPage { index: usize },
    Catalog,
    Finalize,
}

/// 🎟️ Separate grants for retained traversal and content work and permission for one cold semantic operation.
#[derive(Clone, Copy, Debug, Default)]
pub struct DocumentLiftGrant {
    pub traversal: CosGrant,
    pub semantic: bool,
}

/// 🎫 Publication state of the document lift.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DocumentLiftStatus {
    More,
    NeedsSemantic(DocumentLiftAction),
    Complete,
    Cancelled,
    Failed,
}

/// 🧾 A traversal receipt, a content receipt, or the cold semantic operation performed in this turn.
#[derive(Debug)]
#[must_use]
pub struct DocumentLiftReceipt {
    pub status: DocumentLiftStatus,
    pub traversal: Option<PageTreeReceipt>,
    pub content: Option<PageContentReceipt>,
    pub semantic: Option<DocumentLiftAction>,
}

/// 🧳 A stopped lift's native result and remaining semantic planning ownership.
pub struct PartialDocumentLift {
    pub lifter: DocumentLiftState,
    pub catalog: Vec<PdfDictEntry>,
    pub pages: Vec<PageTreeLeaf>,
    pub failed_page: Option<PdfPage>,
}

#[derive(Clone, Copy)]
enum Stage {
    Header,
    Info,
    OptionalContent,
    EmbeddedFiles,
    Traverse,
    RecordPage(PageTreeLeaf),
    BranchResources(PageAttribute),
    CloseTraversal,
    BeginPage,
    PreparePage,
    RetirePageProgram,
    ProjectPage,
    Catalog,
    Finalize,
    Complete,
    Cancelled,
    Failed,
}

/// 🛠️ Retains an unpublished Lifter between page-tree and semantic turns.
#[must_use]
pub struct DocumentLiftCursor {
    lifter: Option<DocumentLiftState>,
    stage: Stage,
    catalog: Vec<PdfDictEntry>,
    walk: Option<PageTreeCursor>,
    leaves: Vec<PageTreeLeaf>,
    next_page: usize,
    failed_page: Option<PdfPage>,
    page_program: Option<PageProgramCursor>,
}

impl DocumentLiftCursor {
    /// 🌱 Starts without borrowing the retained graph, trailer, or declared version.
    pub fn new() -> Self {
        Self {
            lifter: Some(DocumentLiftState::default()), stage: Stage::Header,
            catalog: Vec::new(), walk: None, leaves: Vec::new(), next_page: 0, failed_page: None, page_program: None,
        }
    }

    /// 🎟️ Opens the controlled resource-free semantic route with explicit cold-path barriers.
    pub fn controlled() -> ControlledDocumentLiftCursor { ControlledDocumentLiftCursor::new() }

    /// 📊 Reports discovered and fully projected pages.
    pub fn page_progress(&self) -> (usize, usize) { (self.leaves.len().max(self.next_page), self.next_page) }

    /// ▶️ Advances retained traversal, page content, or a separately permitted cold semantic operation.
    pub fn advance(&mut self, trailer: &[PdfDictEntry], declared_version: &str, source: &mut dyn ObjectSource, grant: DocumentLiftGrant, cancelled: bool) -> DocumentLiftReceipt {
        if cancelled && !matches!(self.stage, Stage::Complete | Stage::Cancelled | Stage::Failed) {
            self.stage = Stage::Cancelled;
            if let Some(walk) = self.walk.as_mut() { walk.begin_close(); }
            if let Some(program) = self.page_program.as_mut() { program.begin_close(); }
        }
        match self.stage {
            Stage::Complete => return receipt(DocumentLiftStatus::Complete),
            Stage::Cancelled => return receipt(DocumentLiftStatus::Cancelled),
            Stage::Failed => return receipt(DocumentLiftStatus::Failed),
            Stage::Traverse => return self.traverse(source, grant.traversal),
            Stage::CloseTraversal => {
                let traversal = self.close_traversal_step(grant.traversal);
                if self.walk.is_none() { self.stage = Stage::BeginPage; }
                return DocumentLiftReceipt { traversal, ..receipt(DocumentLiftStatus::More) };
            }
            Stage::BeginPage => {
                if grant.traversal.work == 0 { return receipt(DocumentLiftStatus::More); }
                self.stage = if let Some(leaf) = self.leaves.get(self.next_page) {
                    self.page_program = Some(PageProgramCursor::new(leaf.reference));
                    Stage::PreparePage
                } else { Stage::Catalog };
                return DocumentLiftReceipt {
                    content: Some(PageContentReceipt { event: Ok(PageContentEvent::More), work: 1, allocated_bytes: 0, released_bytes: 0 }),
                    ..receipt(DocumentLiftStatus::More)
                };
            }
            Stage::PreparePage => return self.prepare_page(source, grant.traversal),
            Stage::RetirePageProgram => {
                let content = self.close_page_content_step(grant.traversal);
                if self.page_program.is_none() { self.stage = Stage::ProjectPage; }
                return DocumentLiftReceipt { content, ..receipt(DocumentLiftStatus::More) };
            }
            _ => {}
        }
        let action = match self.stage {
            Stage::Header => DocumentLiftAction::Header,
            Stage::Info => DocumentLiftAction::Info,
            Stage::OptionalContent => DocumentLiftAction::OptionalContent,
            Stage::EmbeddedFiles => DocumentLiftAction::EmbeddedFiles,
            Stage::RecordPage(_) => DocumentLiftAction::RecordPage,
            Stage::BranchResources(_) => DocumentLiftAction::BranchResources,
            Stage::ProjectPage => DocumentLiftAction::ProjectPage { index: self.next_page },
            Stage::Catalog => DocumentLiftAction::Catalog,
            Stage::Finalize => DocumentLiftAction::Finalize,
            _ => unreachable!("semantic stage requires a declared action"),
        };
        if !grant.semantic { return receipt(DocumentLiftStatus::NeedsSemantic(action)); }
        let mut lifter = Lifter::bind(source, self.lifter.take().expect("document lift owns its semantic state"));
        self.perform(&mut lifter, trailer, declared_version, action);
        let failed = lifter.admission_error.is_some();
        self.lifter = Some(lifter.unbind());
        let status = if failed {
            self.stage = Stage::Failed;
            if let Some(walk) = self.walk.as_mut() { walk.begin_close(); }
            DocumentLiftStatus::Failed
        } else if matches!(self.stage, Stage::Complete) {
            DocumentLiftStatus::Complete
        } else {
            DocumentLiftStatus::More
        };
        DocumentLiftReceipt { semantic: Some(action), ..receipt(status) }
    }

    fn perform(&mut self, lifter: &mut Lifter<'_>, trailer: &[PdfDictEntry], declared_version: &str, action: DocumentLiftAction) {
        self.stage = match action {
            DocumentLiftAction::Header => {
                lifter.snapshot.declared_version = declared_version.to_string();
                let root = dict_get(trailer, "Root").map(|value| lifter.source.deref(value));
                if let Some(PdfObject::Dict(catalog) | PdfObject::Stream { dict: catalog, .. }) = root {
                    self.catalog = catalog;
                }
                if let Some(version) = dict_name(&self.catalog, "Version").filter(|version| *version > declared_version) {
                    lifter.snapshot.declared_version = version.to_string();
                }
                Stage::Info
            }
            DocumentLiftAction::Info => {
                if let Some(value) = dict_get(trailer, "Info") {
                    let value = lifter.source.deref(value);
                    if let Some(dict) = value.as_dict() { lifter.snapshot.info = lifter.lift_info(dict); }
                }
                if let Some(PdfObject::Array(items)) = dict_get(trailer, "ID") {
                    if let [PdfObject::Str(a), PdfObject::Str(b)] = items.as_slice() { lifter.snapshot.document_id = Some([a.clone(), b.clone()]); }
                }
                Stage::OptionalContent
            }
            DocumentLiftAction::OptionalContent => {
                lifter.lift_optional_content(&self.catalog);
                Stage::EmbeddedFiles
            }
            DocumentLiftAction::EmbeddedFiles => {
                lifter.lift_embedded_files(&self.catalog);
                if let Some(root) = dict_get(&self.catalog, "Pages").and_then(PdfObject::as_ref) {
                    self.walk = Some(PageTreeCursor::new(root));
                    Stage::Traverse
                } else { Stage::Catalog }
            }
            DocumentLiftAction::RecordPage => {
                let Stage::RecordPage(leaf) = self.stage else { unreachable!() };
                lifter.page_refs.push(leaf.reference);
                self.leaves.push(leaf);
                Stage::Traverse
            }
            DocumentLiftAction::BranchResources => {
                let Stage::BranchResources(attribute) = self.stage else { unreachable!() };
                if let Some(resources) = lifter.attribute_value(attribute) {
                    let saved_scope = std::mem::replace(&mut lifter.scope, attribute.owner.num as usize);
                    lifter.lift_resources(&resources);
                    lifter.scope = saved_scope;
                }
                Stage::Traverse
            }
            DocumentLiftAction::ProjectPage { index } => {
                let program = self.page_program.take().and_then(|mut cursor| cursor.take_program());
                let page = lifter.lift_page(index as u32, self.leaves[index], program);
                if lifter.admission_error.is_none() {
                    lifter.snapshot.pages.push(page);
                    self.next_page += 1;
                } else {
                    self.failed_page = Some(page);
                }
                Stage::BeginPage
            }
            DocumentLiftAction::Catalog => {
                lifter.lift_catalog(&self.catalog);
                Stage::Finalize
            }
            DocumentLiftAction::Finalize => {
                lifter.snapshot.admitted_stream_roles = lifter.source.take_admitted_roles();
                lifter.snapshot.admitted_stream_roles.append(&mut lifter.admitted_stream_roles);
                lifter.snapshot.objects = std::mem::take(&mut lifter.artifacts).into_objects();
                self.catalog = Vec::new();
                self.leaves = Vec::new();
                Stage::Complete
            }
        };
    }

    fn traverse(&mut self, source: &mut dyn ObjectSource, grant: CosGrant) -> DocumentLiftReceipt {
        let walk = self.walk.as_mut().expect("active page traversal");
        let traversal = walk.advance(source, grant, false);
        let mut status = DocumentLiftStatus::More;
        match traversal.event {
            Ok(PageTreeEvent::Page(leaf)) => self.stage = Stage::RecordPage(leaf),
            Ok(PageTreeEvent::BranchResources(attribute)) => self.stage = Stage::BranchResources(attribute),
            Ok(PageTreeEvent::Complete) => {
                walk.begin_close();
                self.stage = Stage::CloseTraversal;
            }
            Err(reason) => {
                self.lifter.as_mut().expect("document lift owns its semantic state").admission_error = Some(PdfEngineError::Malformed(reason.into()));
                walk.begin_close();
                self.stage = Stage::Failed;
                status = DocumentLiftStatus::Failed;
            }
            _ => {}
        }
        DocumentLiftReceipt { traversal: Some(traversal), ..receipt(status) }
    }

    fn prepare_page(&mut self, source: &mut dyn ObjectSource, grant: CosGrant) -> DocumentLiftReceipt {
        let cursor = self.page_program.as_mut().expect("active controlled page program");
        let content = cursor.advance(source, grant, false);
        match content.event {
            Ok(PageContentEvent::Complete) => self.stage = Stage::ProjectPage,
            Ok(PageContentEvent::Deferred(_)) => {
                self.page_program = None;
                self.stage = Stage::ProjectPage;
            }
            Err(_) => {
                cursor.begin_close();
                self.stage = Stage::RetirePageProgram;
            }
            _ => {}
        }
        DocumentLiftReceipt { content: Some(content), ..receipt(DocumentLiftStatus::More) }
    }

    /// ♻️ Retires page content before yielding the remaining lifter state on interruption.
    pub fn close_page_content_step(&mut self, grant: CosGrant) -> Option<PageContentReceipt> {
        if !matches!(self.stage, Stage::RetirePageProgram | Stage::Cancelled | Stage::Failed) { return None; }
        let turn = self.page_program.as_mut()?.close_step(grant);
        if turn.event == Ok(PageContentEvent::Closed) { self.page_program = None; }
        Some(turn)
    }

    /// ♻️ Drives page-tree backing retirement before a cancelled or failed result is taken.
    pub fn close_traversal_step(&mut self, grant: CosGrant) -> Option<PageTreeReceipt> {
        if !matches!(self.stage, Stage::CloseTraversal | Stage::Cancelled | Stage::Failed) { return None; }
        let traversal = self.walk.as_mut()?.close_step(grant);
        if traversal.event == Ok(PageTreeEvent::Closed) { self.walk = None; }
        Some(traversal)
    }

    /// 📤 Transfers a completed semantic lift exactly once.
    pub fn take_result(&mut self) -> Option<DocumentLiftState> {
        if matches!(self.stage, Stage::Complete) { self.lifter.take() } else { None }
    }

    /// 🧳 Transfers the retained partial graph after page-tree retirement on a stopped lift.
    pub fn take_partial(&mut self) -> Option<PartialDocumentLift> {
        if !matches!(self.stage, Stage::Cancelled | Stage::Failed) || self.walk.is_some() || self.page_program.is_some() { return None; }
        Some(PartialDocumentLift {
            lifter: self.lifter.take()?, catalog: std::mem::take(&mut self.catalog), pages: std::mem::take(&mut self.leaves),
            failed_page: self.failed_page.take(),
        })
    }
}

fn receipt(status: DocumentLiftStatus) -> DocumentLiftReceipt {
    DocumentLiftReceipt { status, traversal: None, content: None, semantic: None }
}
