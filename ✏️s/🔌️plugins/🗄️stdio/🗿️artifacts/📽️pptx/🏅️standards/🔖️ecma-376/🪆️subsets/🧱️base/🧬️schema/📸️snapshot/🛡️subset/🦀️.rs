//! 🛡️ Literal root namespaces and relationship types establish exact PresentationML profiles.
use super::PptxSnapshot;
use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::FaultScope;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;

use semio_framework_value::NativeEncodeControl;
use semio_framework_value::{ValueError, ValueRefusalKind};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlAttr, XmlDocument, XmlNode};
const STRICT: &str = "http://purl.oclc.org/ooxml/presentationml/main";
const TRANS: &str = "http://schemas.openxmlformats.org/presentationml/2006/main";
const TRANS_DRAW: &str = "http://schemas.openxmlformats.org/drawingml/2006/main";
const STRICT_REL: &str = "http://purl.oclc.org/ooxml/officeDocument/relationships";
const TRANS_REL: &str = "http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const VML: &str = "urn:schemas-microsoft-com:vml";
const MC: &str = "http://schemas.openxmlformats.org/markup-compatibility/2006";
fn equal(a: &str, b: &str, control: &mut NativeEncodeControl<'_>) -> Result<bool, ValueError> {
    if a.len() != b.len() {
        return Ok(false);
    }
    control.scoped_stage(|control| -> Result<bool, ValueError> {
        control.begin_stage(a.len())?;
        for (a, b) in a.as_bytes().chunks(256).zip(b.as_bytes().chunks(256)) {
            let same = a == b;
            control.advance(a.len())?;
            if !same {
                return Ok(false);
            }
        }
        Ok(true)
    })
}
fn text(parts: &[&str], control: &mut NativeEncodeControl<'_>) -> Result<String, ValueError> {
    let length = parts.iter().try_fold(0usize, |sum, part| sum.checked_add(part.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "PPTX profile text overflow")))?;
    control.scoped_stage(|control| -> Result<String, ValueError> {
        control.begin_stage(length)?;
        control.charge(length)?;
        let mut output = String::new();
        output.try_reserve_exact(length).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "PPTX profile text allocation"))?;
        for part in parts {
            let mut start = 0;
            while start < part.len() {
                let mut end = (start + 256).min(part.len());
                while !part.is_char_boundary(end) {
                    end -= 1;
                }
                output.push_str(&part[start..end]);
                control.advance(end - start)?;
                start = end;
            }
        }
        Ok(output)
    })
}
fn diagnostic(output: &mut Vec<Diagnostic>, subset: &str, suffix: &str, severity: Severity, parts: &[&str], control: &mut NativeEncodeControl<'_>) -> Result<(), ValueError> {
    control.charge(std::mem::size_of::<Diagnostic>())?;
    output.try_reserve_exact(1).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "PPTX diagnostic allocation"))?;
    output.push(Diagnostic { code: FaultCode::new(text(&["stdio.pptx.", subset, ".", suffix], control)?), severity, span: TextSpan::at(1, 1), message: text(parts, control)?, expected: None, scope: FaultScope::default() });
    Ok(())
}
#[derive(Default)]
struct Flags {
    main_strict: bool,
    main_trans: bool,
    trans: bool,
    vml: bool,
    family: bool,
    alternate: bool,
    conformance: bool,
}
struct Frame<'a> {
    nodes: &'a [XmlNode],
    position: usize,
    attrs: &'a [XmlAttr],
}
fn push<'a>(frames: &mut Vec<Frame<'a>>, nodes: &'a [XmlNode], attrs: &'a [XmlAttr], control: &mut NativeEncodeControl<'_>) -> Result<(), ValueError> {
    if frames.len() == frames.capacity() {
        control.charge(64 * std::mem::size_of::<Frame<'a>>())?;
        frames.try_reserve_exact(64).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "PPTX profile frontier allocation"))?;
    }
    frames.push(Frame { nodes, position: 0, attrs });
    Ok(())
}
fn colon(name: &str, control: &mut NativeEncodeControl<'_>) -> Result<Option<usize>, ValueError> {
    control.scoped_stage(|control| -> Result<Option<usize>, ValueError> {
        control.begin_stage(name.len())?;
        for (index, chunk) in name.as_bytes().chunks(256).enumerate() {
            let found = chunk.iter().position(|byte| *byte == b':');
            control.advance(chunk.len())?;
            if let Some(offset) = found {
                return Ok(Some(index * 256 + offset));
            }
        }
        Ok(None)
    })
}
fn namespace<'a>(attrs: &'a [XmlAttr], prefix: Option<&str>, control: &mut NativeEncodeControl<'_>) -> Result<Option<&'a str>, ValueError> {
    for attr in attrs {
        control.checkpoint()?;
        if match prefix {
            None => attr.name == "xmlns",
            Some(prefix) => match attr.name.strip_prefix("xmlns:") {
                Some(name) => equal(name, prefix, control)?,
                None => false,
            },
        } {
            return Ok(Some(&attr.value));
        }
    }
    Ok(None)
}
fn inspect(document: &XmlDocument, control: &mut NativeEncodeControl<'_>) -> Result<Flags, ValueError> {
    let mut flags = Flags::default();
    let mut frames = Vec::new();
    for forest in [document.prolog.as_slice(), document.root.as_ref().map_or(&[][..], std::slice::from_ref), document.epilog.as_slice()] {
        push(&mut frames, forest, &[], control)?;
        while !frames.is_empty() {
            let frame = frames.last_mut().unwrap();
            let Some(node) = frame.nodes.get(frame.position) else {
                frames.pop();
                continue;
            };
            frame.position += 1;
            control.checkpoint()?;
            let XmlNode::Element { name, attrs, children } = node else { continue };
            for attr in attrs {
                control.checkpoint()?;
                if attr.name == "xmlns" || attr.name.starts_with("xmlns:") {
                    flags.trans |= equal(&attr.value, TRANS, control)? || equal(&attr.value, TRANS_DRAW, control)?;
                    flags.vml |= equal(&attr.value, VML, control)?;
                    flags.family |= attr.value.starts_with("http://purl.oclc.org/ooxml/");
                }
            }
            let root = document.root.as_ref().is_some_and(|root| std::ptr::eq(root, node));
            if root {
                for attr in attrs {
                    control.checkpoint()?;
                    flags.conformance |= attr.name == "conformance" && attr.value == "strict";
                }
            }
            let separator = colon(name, control)?;
            let prefix = separator.map(|position| &name[..position]);
            let local = separator.map_or(name.as_str(), |position| &name[position + 1..]);
            if root && equal(local, "presentation", control)? {
                if let Some(namespace) = namespace(attrs, prefix, control)? {
                    flags.main_strict = equal(namespace, STRICT, control)?;
                    flags.main_trans = equal(namespace, TRANS, control)?;
                }
            }
            if equal(local, "AlternateContent", control)? {
                let mut namespace = namespace(attrs, prefix, control)?;
                if namespace.is_none() {
                    for frame in frames.iter().rev() {
                        namespace = self::namespace(frame.attrs, prefix, control)?;
                        if namespace.is_some() {
                            break;
                        }
                    }
                }
                if let Some(namespace) = namespace {
                    flags.alternate |= equal(namespace, MC, control)?;
                }
            }
            push(&mut frames, children, attrs, control)?;
        }
    }
    Ok(flags)
}
fn segment<'a>(part: &'a str, parts: &mut Vec<&'a str>, control: &mut NativeEncodeControl<'_>) -> Result<(), ValueError> {
    match part {
        "" | "." => {}
        ".." => {
            parts.pop();
        }
        _ => {
            if parts.len() == parts.capacity() {
                control.charge(64 * std::mem::size_of::<&str>())?;
                parts.try_reserve_exact(64).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "PPTX relationship frontier allocation"))?;
            }
            parts.push(part);
        }
    }
    Ok(())
}
fn path(target: &str, control: &mut NativeEncodeControl<'_>) -> Result<String, ValueError> {
    if let Some(target) = target.strip_prefix('/') {
        return text(&[target], control);
    }
    let mut parts = Vec::new();
    control.scoped_stage(|control| -> Result<(), ValueError> {
        control.begin_stage(target.len())?;
        let mut start = 0;
        for (index, byte) in target.bytes().enumerate() {
            if byte == b'/' {
                segment(&target[start..index], &mut parts, control)?;
                start = index + 1;
            }
            control.step()?;
        }
        segment(&target[start..], &mut parts, control)
    })?;
    let length = parts.iter().try_fold(parts.len().saturating_sub(1), |total, part| total.checked_add(part.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "PPTX relationship text overflow")))?;
    control.scoped_stage(|control| -> Result<String, ValueError> {
        control.begin_stage(length)?;
        control.charge(length)?;
        let mut output = String::new();
        output.try_reserve_exact(length).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "PPTX relationship text allocation"))?;
        for (index, part) in parts.iter().enumerate() {
            if index != 0 {
                output.push('/');
                control.step()?;
            }
            let mut start = 0;
            while start < part.len() {
                let mut end = (start + 256).min(part.len());
                while !part.is_char_boundary(end) {
                    end -= 1;
                }
                output.push_str(&part[start..end]);
                control.advance(end - start)?;
                start = end;
            }
        }
        Ok(output)
    })
}
pub(crate) fn check(snapshot: &PptxSnapshot, subset: &str, control: &mut NativeEncodeControl<'_>) -> Result<Vec<Diagnostic>, ValueError> {
    let strict = subset == "strict";
    let mut output = Vec::new();
    let mut main = None;
    for relationship in snapshot.opc.relationships.relationships("").into_iter().flatten() {
        control.checkpoint()?;
        if relationship.rel_type.ends_with("/officeDocument") {
            main = Some(path(&relationship.target, control)?);
            break;
        }
    }
    let mut main_part = None;
    if let Some(path) = main.as_deref() {
        for part in &snapshot.xml_parts {
            control.checkpoint()?;
            if equal(&part.path, path, control)? {
                main_part = Some(part);
                break;
            }
        }
    }
    if main_part.is_none() {
        diagnostic(&mut output, subset, if strict { "main-ns-not-strict" } else { "main-ns-not-transitional" }, Severity::Error, &["package has no resolvable main PresentationML document"], control)?;
    }
    for part in &snapshot.xml_parts {
        control.checkpoint()?;
        let flags = inspect(&part.document, control)?;
        if main_part.is_some_and(|main| std::ptr::eq(main, part)) {
            if !(if strict { flags.main_strict } else { flags.main_trans }) {
                diagnostic(
                    &mut output,
                    subset,
                    if strict { "main-ns-not-strict" } else { "main-ns-not-transitional" },
                    Severity::Error,
                    &["root officeDocument part ", &part.path, " does not declare the expected PresentationML root namespace ", if strict { STRICT } else { TRANS }],
                    control,
                )?;
            }
            if strict && !flags.conformance {
                diagnostic(&mut output, subset, "conformance-attr-missing", Severity::Warning, &["root officeDocument part ", &part.path, " does not declare conformance=\"strict\""], control)?;
            }
            if !strict && flags.conformance {
                diagnostic(&mut output, subset, "conformance-attr-not-transitional", Severity::Warning, &["root officeDocument part ", &part.path, " declares conformance=\"strict\""], control)?;
            }
        }
        if strict {
            if flags.trans {
                diagnostic(&mut output, subset, "transitional-ns-present", Severity::Error, &["part ", &part.path, " declares a Transitional OOXML main namespace"], control)?;
            }
            if flags.vml {
                diagnostic(&mut output, subset, "vml-present", Severity::Error, &["part ", &part.path, " declares VML markup ", VML], control)?;
            }
            if flags.alternate {
                diagnostic(&mut output, subset, "alternate-content-present", Severity::Warning, &["part ", &part.path, " contains mc:AlternateContent markup"], control)?;
            }
        } else if flags.family {
            diagnostic(&mut output, subset, "strict-ns-present", Severity::Error, &["part ", &part.path, " declares a Strict OOXML namespace"], control)?;
        }
    }
    for (owner, relationships) in snapshot.opc.relationships.groups() {
        for relationship in relationships {
            control.checkpoint()?;
            if strict && relationship.rel_type.starts_with(TRANS_REL) {
                diagnostic(&mut output, subset, "relationship-base-not-strict", Severity::Error, &["relationship ", &relationship.id, " owned by ", owner, " uses Transitional relationship type ", &relationship.rel_type], control)?;
            }
            if !strict && relationship.rel_type.starts_with(STRICT_REL) {
                diagnostic(&mut output, subset, "strict-ns-present", Severity::Error, &["relationship ", &relationship.id, " owned by ", owner, " uses Strict relationship type ", &relationship.rel_type], control)?;
            }
        }
    }
    Ok(output)
}
