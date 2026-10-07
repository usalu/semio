//! 🛡️ Validity over the owned XML document.
use crate::schema::snapshot::XmlNode;
use crate::XmlSnapshot;
use semio_framework_diagnostic::{Diagnostic,FaultCode,FaultScope,Severity,TextSpan};
    pub const CODE_DOCTYPE_MISSING: &str = "stdio.xml.valid.doctype-missing";
    pub const CODE_ROOT_NAME_MISMATCH: &str = "stdio.xml.valid.root-name-mismatch";
    pub const CODE_STANDALONE_EXTERNAL_SUBSET: &str = "stdio.xml.valid.standalone-external-subset";
    pub const CODE_VALIDITY_NOT_VERIFIED: &str = "stdio.xml.valid.validity-not-fully-verified";

    /// 🌳️ The actual root element's tag name, if a root element is present at all.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn root_element_name(snapshot: &XmlSnapshot) -> Option<&str> {
        match &snapshot.doc.root {
            Some(XmlNode::Element { name, .. }) => Some(name.as_str()),
            _ => None,
        }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn hard(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Error, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    fn soft(code: &'static str, message: String) -> Diagnostic {
        Diagnostic { code: FaultCode::new(code), severity: Severity::Warning, span: TextSpan::at(1, 1), message, expected: None, scope: FaultScope::default() }
    }

    /// 🛡️ Real, scope-limited W3C XML 1.0 Fifth Edition §5.1 validity checks against one
    /// already-decoded `XmlSnapshot`. Shared single source of truth: `XmlValidComposer::compose`
    /// hard-gates on this (pre-serialization, authoritative), `XmlValidBuilder::build` hard-gates on
    /// this too, and the registered `SubsetValidator` re-runs it post-hoc against the wire payload for
    /// the D5 validate-on-build hook.
    // 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
    pub fn check_valid_conformance(snapshot: &XmlSnapshot) -> Vec<Diagnostic> {
        check_conformance(snapshot, &mut |_, _| Ok(())).expect("uncontrolled validity callback cannot refuse")
    }

    pub(crate) fn check_conformance(snapshot: &XmlSnapshot, progress: &mut dyn FnMut(usize, usize) -> Result<(), semio_framework_value::ValueError>) -> Result<Vec<Diagnostic>, semio_framework_value::ValueError> {
        let mut out = Vec::new();
        let total = snapshot.doc.prolog.len().checked_add(snapshot.doc.epilog.len()).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::WorkLimit,"XML validity boundary count overflow"))?; progress(0, total)?;
        if root_element_name(snapshot).is_none() { out.push(hard("stdio.xml.valid.document-element-missing", "XML validity requires a document element".into())); }
        let mut invalid_boundary = false;
        for (ordinal, node) in snapshot.doc.prolog.iter().chain(&snapshot.doc.epilog).enumerate() { if ordinal % 256 == 0 { progress(ordinal, total)?; } invalid_boundary |= !matches!(node, XmlNode::Comment { .. } | XmlNode::ProcessingInstruction { .. }); }
        if invalid_boundary { out.push(hard("stdio.xml.valid.boundary-kind", "XML validity permits only comments and processing instructions outside the document element".into())); }
        if snapshot.doc.doctype.as_ref().is_some_and(|value| value.prolog_position > snapshot.doc.prolog.len() as u64) { out.push(hard("stdio.xml.valid.doctype-position", "XML validity requires the doctype to occupy its declared prolog position".into())); }
        if let Some(declaration) = &snapshot.doc.declaration {
            if declaration.version != "1.0" { out.push(hard("stdio.xml.valid.version", "XML 1.0 validity requires declaration version 1.0".into())); }
            if let Some(encoding) = &declaration.encoding {
                let mut valid = !encoding.is_empty(); progress(0, encoding.len())?;
                for (ordinal, byte) in encoding.bytes().enumerate() { if ordinal % 256 == 0 { progress(ordinal, encoding.len())?; } valid &= if ordinal == 0 { byte.is_ascii_alphabetic() } else { byte.is_ascii_alphanumeric() || matches!(byte, b'.' | b'_' | b'-') }; }
                progress(encoding.len(), encoding.len())?;
                if !valid { out.push(hard("stdio.xml.valid.encoding-name", "XML validity requires a syntactically valid declared encoding name".into())); }
            }
        }
        match &snapshot.doc.doctype {
            None => {
                out.push(hard(CODE_DOCTYPE_MISSING, "no <!DOCTYPE ...> declaration present -- XML 1.0 §5.1 validity requires one (a document without one can be well-formed at best)".into()));
            }
            Some(doctype) => {
                if let Some(actual_root) = root_element_name(snapshot) {
                    if doctype.name != actual_root {
                        out.push(hard(CODE_ROOT_NAME_MISMATCH, format!("doctype declares root name '{}' but the actual root element is '<{actual_root}>' -- §2.8 requires the DOCTYPE Name to match the document element", doctype.name)));
                    }
                }
                if doctype.external_id.is_some() && snapshot.doc.declaration.as_ref().and_then(|d| d.standalone) == Some(true) {
                    out.push(soft(CODE_STANDALONE_EXTERNAL_SUBSET, "XML declaration says standalone=\"yes\" but the doctype references an external subset (SYSTEM/PUBLIC) -- suspicious per §2.9".into()));
                }
            }
        }
        out.push(soft(
            CODE_VALIDITY_NOT_VERIFIED,
            "validity not fully verified: the typed doctype retains its name, external identifiers and entity declarations; DTD element and attribute content models (§3.2/§3.3) are absent from this owned schema".into(),
        ));
        progress(total, total)?; Ok(out)
    }
