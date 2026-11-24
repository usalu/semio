use std::borrow::Cow;
use crate::standards::v1_7::subsets::base::{
    io::{error::{PResult, PdfEngineError}, export::{EncodeOptions, PreparedPdfGraph}},
    modules::{lexer::dict_get, writer::DocumentTrailer},
    schema::snapshot::{ObjRef, PdfDictEntry, PdfImageBody, PdfIndirectObject, PdfObject, PdfOp, PdfSnapshot, PdfStreamFilter},
};
use super::classic_graph_quote::{GraphLoan, GraphRefusal, object_order};
use super::ordinary_graft::OrdinaryRows;

/// 📌️ Owns the exact prepared current COS graph, or reloans the pinned original when reconciliation borrows it.
#[derive(semio_framework_value::RetireOwned)]
pub struct PinnedPdfGraph {
    source: usize,
    revision: [u8; 32],
    version: String,
    reconciled: Option<Vec<PdfIndirectObject>>,
    order: Option<Vec<u32>>,
    ordinary: Option<OrdinaryRows>,
    graft_pending: bool,
    trailer: DocumentTrailer,
}

const REFERENCE_SLACK: usize = 2048;

/// 🔢️ The table a file-ordered roster needs for its ascending view; an ascending roster needs none.
fn order_extent(objects: &[PdfIndirectObject]) -> usize {
    if objects.windows(2).all(|pair| pair[0].id.num < pair[1].id.num) { 0 } else { objects.len() * size_of::<u32>() }
}

/// 📐️ Length-based byte size of one retained value, counting every node's inline frame.
fn object_bytes(value: &PdfObject) -> usize {
    size_of::<PdfObject>() + match value {
        PdfObject::Str(bytes) => bytes.len(),
        PdfObject::Text(text) | PdfObject::Name(text) => text.len(),
        PdfObject::Real(decimal) => decimal.coefficient.len(),
        PdfObject::Array(values) => values.iter().map(object_bytes).sum(),
        PdfObject::Dict(entries) => entries_bytes(entries),
        PdfObject::Stream { dict, data, filters } => entries_bytes(dict) + data.len() + filters.len() * size_of::<PdfStreamFilter>(),
        _ => 0,
    }
}

fn entries_bytes(entries: &[PdfDictEntry]) -> usize {
    entries.iter().map(|entry| size_of::<PdfDictEntry>() + entry.key.len() + object_bytes(&entry.value)).sum()
}

/// 📏️ Capacity-based heap size of one owned value, excluding its own inline frame.
pub(crate) fn object_capacity(value: &PdfObject) -> usize {
    match value {
        PdfObject::Str(bytes) => bytes.capacity(),
        PdfObject::Text(text) | PdfObject::Name(text) => text.capacity(),
        PdfObject::Real(decimal) => decimal.coefficient.capacity(),
        PdfObject::Array(values) => values.capacity() * size_of::<PdfObject>() + values.iter().map(object_capacity).sum::<usize>(),
        PdfObject::Dict(entries) => entries.capacity() * size_of::<PdfDictEntry>() + entries.iter().map(|entry| entry.key.capacity() + object_capacity(&entry.value)).sum::<usize>(),
        PdfObject::Stream { dict, data, filters } => dict.capacity() * size_of::<PdfDictEntry>()
            + dict.iter().map(|entry| entry.key.capacity() + object_capacity(&entry.value)).sum::<usize>()
            + data.retained_bytes().allocated_bytes() + filters.capacity() * size_of::<PdfStreamFilter>(),
        _ => 0,
    }
}

impl PinnedPdfGraph {
    /// 🧬️ The reference planner uses the actual encode selector and never omits trailer extras.
    /// A retained Original job must receive this owner after controlled reconciliation rather than call this cold path.
    pub fn prepare_reference(snapshot: &PdfSnapshot, revision: [u8; 32]) -> PResult<Self> {
        let graph = PreparedPdfGraph::from_snapshot(snapshot, &EncodeOptions::default())?;
        if graph.write.encryption.is_some() || graph.write.xref_stream {
            return Err(PdfEngineError::Unsupported("segmented Save requires a clear classic PDF graph".into()));
        }
        let reconciled = match graph.objects {
            Cow::Borrowed(objects) => {
                if !std::ptr::eq(objects, snapshot.objects.as_slice()) {
                    return Err(PdfEngineError::Malformed("reconciliation returned a foreign borrowed graph".into()));
                }
                None
            }
            Cow::Owned(objects) => Some(objects),
        };
        let order = object_order(reconciled.as_deref().unwrap_or(&snapshot.objects))
            .map_err(|_| PdfEngineError::Unsupported("segmented Save requires a unique object roster".into()))?;
        Ok(Self { source: std::ptr::from_ref(snapshot) as usize, revision, version: graph.version, reconciled, order, ordinary: None, graft_pending: false, trailer: graph.trailer })
    }


    /// 🧮️ Bounds the capacity a reconciled reference graph can own: every retained row may be copied and every typed lane may be spelled afresh.
    pub fn reference_extent(snapshot: &PdfSnapshot) -> Option<usize> {
        let retained = snapshot.objects.iter().try_fold(0usize, |sum, object| sum.checked_add(size_of::<PdfIndirectObject>() + object_bytes(&object.value)))?;
        let pages = snapshot.pages.iter().try_fold(0usize, |sum, page| {
            sum.checked_add(page.content.len().checked_mul(size_of::<PdfOp>() + 64)?)?.checked_add(512 + page.annotations.len().checked_mul(1024)?)
        })?;
        let images = snapshot.images.iter().try_fold(0usize, |sum, image| sum.checked_add(match &image.body {
            PdfImageBody::Samples { values } => values.len().checked_mul(4)?,
            _ => 0,
        } + 1024))?;
        let fonts = snapshot.fonts.len().checked_mul(2048)?;
        retained.checked_add(order_extent(&snapshot.objects))?.checked_add(pages)?.checked_add(images)?.checked_add(fonts)?.checked_mul(2)?.checked_add(REFERENCE_SLACK)
    }

    /// 📏️ The capacity this graph actually owns beyond its fixed frame once a reference reconciliation filled it.
    pub fn reference_capacity(&self) -> usize {
        self.reconciled.as_ref().map_or(0, |objects| objects.capacity() * size_of::<PdfIndirectObject>()
            + objects.iter().map(|object| object_capacity(&object.value)).sum::<usize>())
            + self.retained_control_capacity()
    }

    /// 🧮️ Quotes the retained candidate's entire shallow control allocation before the graph is reloaned.
    pub fn retained_control_extent(snapshot: &PdfSnapshot) -> Option<usize> {
        if snapshot.encryption.is_some() || snapshot.objects.is_empty() || snapshot.trailer.len() > 16
            || snapshot.declared_version.len() > 16
            || dict_get(&snapshot.trailer, "Root").and_then(PdfObject::as_ref).is_none()
        { return None; }
        let mut extent = snapshot.declared_version.len().max(3) + order_extent(&snapshot.objects);
        if let Some([first, second]) = &snapshot.document_id {
            if first.len() > 128 || second.len() > 128 { return None; }
            extent += first.len() + second.len();
        }
        for entry in snapshot.trailer.iter().filter(|entry| !matches!(entry.key.as_str(), "Root" | "Info" | "ID")) {
            let body = match &entry.value {
                PdfObject::Null | PdfObject::Bool(_) | PdfObject::Int(_) | PdfObject::Ref(_) => 0,
                PdfObject::Real(decimal) if decimal.scale <= 384 && decimal.coefficient.bytes().all(|byte| byte.is_ascii_digit()) => decimal.coefficient.len(),
                PdfObject::Name(text) | PdfObject::Text(text) => text.len(),
                PdfObject::Str(bytes) => bytes.len(),
                _ => return None,
            };
            if entry.key.len() > 128 || body > 128 { return None; }
            extent += size_of::<PdfDictEntry>() + entry.key.len() + body;
        }
        Some(extent)
    }

    /// 🌱️ Builds only the retained control trailer. The graph remains private pending the controlled semantic check.
    pub fn retained_candidate(snapshot: &PdfSnapshot, revision: [u8; 32]) -> PResult<Self> {
        Self::retained_control_extent(snapshot).ok_or_else(|| PdfEngineError::Unsupported("retained PDF control trailer exceeds the Save qualification".into()))?;
        let root = dict_get(&snapshot.trailer, "Root").and_then(PdfObject::as_ref).ok_or_else(||
            PdfEngineError::Malformed("retained PDF has no root".into()))?;
        let info = dict_get(&snapshot.trailer, "Info").and_then(PdfObject::as_ref);
        let extra = |entry: &&PdfDictEntry| !matches!(entry.key.as_str(), "Root" | "Info" | "ID");
        let mut entries = Vec::with_capacity(snapshot.trailer.iter().filter(extra).count());
        entries.extend(snapshot.trailer.iter().filter(extra).cloned());
        let version = if snapshot.declared_version.is_empty() { "1.7".to_owned() } else { snapshot.declared_version.clone() };
        let trailer = DocumentTrailer { root, info, id: snapshot.document_id.clone(), extra: entries };
        let order = object_order(&snapshot.objects).map_err(|_| PdfEngineError::Unsupported("retained PDF roster repeats an object number".into()))?;
        Ok(Self { source: std::ptr::from_ref(snapshot) as usize, revision, version, reconciled: None, order, ordinary: None, graft_pending: true, trailer })
    }


    /// 📏️ Counts the capacities actually owned by the shallow retained-control candidate.
    pub fn retained_control_capacity(&self) -> usize {
        let ids = self.trailer.id.as_ref().map_or(0, |[a, b]| a.capacity() + b.capacity());
        self.order.as_ref().map_or(0, |order| order.capacity() * size_of::<u32>()) + self.version.capacity() + ids + self.trailer.extra.capacity() * size_of::<PdfDictEntry>()
            + self.trailer.extra.iter().map(|entry| entry.key.capacity() + match &entry.value {
                PdfObject::Name(text) | PdfObject::Text(text) => text.capacity(),
                PdfObject::Str(bytes) => bytes.capacity(),
                PdfObject::Real(decimal) => decimal.coefficient.capacity(),
                _ => 0,
            }).sum::<usize>()
    }

    pub fn needs_ordinary_graft(&self) -> bool { self.graft_pending }

    /// 📎️ An appended section replaces the ordinary graft: the graph keeps its retained control and only its Info reference may move.
    pub fn enter_increment(&mut self, info: Option<ObjRef>) {
        assert!(self.graft_pending && self.ordinary.is_none());
        if info.is_some() { self.trailer.info = info; }
        self.graft_pending = false;
    }

    /// 📎️ The trailer an appended section states.
    pub fn trailer(&self) -> &DocumentTrailer { &self.trailer }

    /// 🏷️ The declared version the original header carries.
    pub fn version(&self) -> &str { &self.version }

    pub fn install_ordinary_graft(&mut self, rows: OrdinaryRows) {
        assert!(self.graft_pending && self.ordinary.is_none());
        self.trailer.info = rows.info_ref;
        self.ordinary = Some(rows);
        self.graft_pending = false;
    }

    /// 🔐️ A graph quote is only possible for the exact lifetime-pinned snapshot and canonical base.
    pub fn loan<'a>(&'a self, original: &'a PdfSnapshot, revision: [u8; 32]) -> Result<GraphLoan<'a>, GraphRefusal> {
        if self.source != std::ptr::from_ref(original) as usize || self.revision != revision {
            return Err(GraphRefusal::SourceChanged);
        }
        Ok(GraphLoan { version: &self.version, objects: self.reconciled.as_deref().unwrap_or(&original.objects), order: self.order.as_deref(), trailer: &self.trailer, overlay: self.ordinary.as_ref().map_or_else(Default::default, OrdinaryRows::overlay), resolver: None })
    }
}
