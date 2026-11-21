use std::borrow::Cow;
use crate::standards::v1_7::subsets::base::{
    io::{error::{PResult, PdfEngineError}, export::{EncodeOptions, PreparedPdfGraph}},
    modules::{lexer::dict_get, writer::DocumentTrailer},
    schema::snapshot::{ObjRef, PdfDictEntry, PdfIndirectObject, PdfObject, PdfSnapshot},
};
use super::classic_graph_quote::{GraphLoan, GraphRefusal};
use super::ordinary_graft::OrdinaryRows;

/// 📌️ Owns the exact prepared current COS graph, or reloans the pinned original when reconciliation borrows it.
#[derive(semio_framework_value::RetireOwned)]
pub struct PinnedPdfGraph {
    source: usize,
    revision: [u8; 32],
    version: String,
    reconciled: Option<Vec<PdfIndirectObject>>,
    ordinary: Option<OrdinaryRows>,
    graft_pending: bool,
    trailer: DocumentTrailer,
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
        Ok(Self { source: std::ptr::from_ref(snapshot) as usize, revision, version: graph.version, reconciled, ordinary: None, graft_pending: false, trailer: graph.trailer })
    }


    /// 🧮️ Quotes the retained candidate's entire shallow control allocation before the graph is reloaned.
    pub fn retained_control_extent(snapshot: &PdfSnapshot) -> Option<usize> {
        if snapshot.encryption.is_some() || snapshot.objects.is_empty() || snapshot.trailer.len() > 16
            || snapshot.declared_version.len() > 16
            || dict_get(&snapshot.trailer, "Root").and_then(PdfObject::as_ref).is_none()
        { return None; }
        let mut extent = snapshot.declared_version.len().max(3);
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
        Ok(Self { source: std::ptr::from_ref(snapshot) as usize, revision, version, reconciled: None, ordinary: None, graft_pending: true, trailer })
    }


    /// 📏️ Counts the capacities actually owned by the shallow retained-control candidate.
    pub fn retained_control_capacity(&self) -> usize {
        let ids = self.trailer.id.as_ref().map_or(0, |[a, b]| a.capacity() + b.capacity());
        self.version.capacity() + ids + self.trailer.extra.capacity() * size_of::<PdfDictEntry>()
            + self.trailer.extra.iter().map(|entry| entry.key.capacity() + match &entry.value {
                PdfObject::Name(text) | PdfObject::Text(text) => text.capacity(),
                PdfObject::Str(bytes) => bytes.capacity(),
                PdfObject::Real(decimal) => decimal.coefficient.capacity(),
                _ => 0,
            }).sum::<usize>()
    }

    pub fn needs_ordinary_graft(&self) -> bool { self.graft_pending }

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
        Ok(GraphLoan { version: &self.version, objects: self.reconciled.as_deref().unwrap_or(&original.objects), trailer: &self.trailer, overlay: self.ordinary.as_ref().map_or_else(Default::default, OrdinaryRows::overlay) })
    }
}
