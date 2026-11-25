//! 🧵️ Resumable XML document builder of `stdio.xml` (1.0/🧱️base): builds exactly the tree `xml_document_from_text` builds, one token per step on an explicit element stack instead of one recursion per document.

use super::{parse_attrs, parse_misc, parse_name, parse_xml_declaration_prolog, skip_ws, xml_unescape_text};
use crate::schema::snapshot::{validate_xml_document_boundaries, XmlAttr, XmlDeclaration, XmlDoctype, XmlDocument, XmlNode};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Phase {
    Prolog,
    Content,
    Epilog,
    Done,
}

struct OpenElement {
    name: String,
    attrs: Vec<XmlAttr>,
    children: Vec<XmlNode>,
}

/// 🧵️ What one bounded advance of an [`XmlDocumentCursor`] produced.
pub enum XmlCursorStep {
    More,
    Complete(XmlDocument),
}

/// 🌊️ Owns one XML text and parses it resumably: each advance consumes whole tokens until its byte budget is reached, so one turn costs the budget plus at most one token.
pub struct XmlDocumentCursor {
    text: String,
    start: usize,
    end: usize,
    pos: usize,
    phase: Phase,
    declaration: Option<XmlDeclaration>,
    doctype: Option<XmlDoctype>,
    prolog: Vec<XmlNode>,
    open: Vec<OpenElement>,
    root: Option<XmlNode>,
}

impl XmlDocumentCursor {
    /// 🌱️ Takes the text whose surrounding whitespace the document ignores, as `xml_document_from_text` does.
    pub fn new(text: String) -> Self {
        let start = text.len() - text.trim_start().len();
        let end = start + text.trim().len();
        Self { text, start, end, pos: 0, phase: Phase::Prolog, declaration: None, doctype: None, prolog: Vec::new(), open: Vec::new(), root: None }
    }

    /// 📏️ Bytes of the trimmed document consumed so far.
    pub fn consumed(&self) -> usize {
        self.pos
    }

    /// 📐️ Bytes of the trimmed document in total.
    pub fn total(&self) -> usize {
        self.end - self.start
    }

    /// ➡️ Consumes at least `budget` bytes (or the rest of the document) and reports the finished document once the last token is read.
    pub fn advance(&mut self, budget: usize) -> Result<XmlCursorStep, String> {
        let goal = self.pos.saturating_add(budget.max(1));
        loop {
            match self.phase {
                Phase::Done => return Err("XML document cursor already completed".into()),
                Phase::Prolog => self.prolog_step()?,
                Phase::Content => self.token()?,
                Phase::Epilog => return self.epilog_step().map(XmlCursorStep::Complete),
            }
            if self.pos >= goal {
                return Ok(XmlCursorStep::More);
            }
        }
    }

    /// 🧺️ Hands out the half-built element chain so its owner can retire it, folded from the open stack into one nested element.
    pub fn take_partial(&mut self) -> Option<XmlNode> {
        let mut node = self.root.take();
        while let Some(OpenElement { name, attrs, mut children }) = self.open.pop() {
            children.extend(node.take());
            node = Some(XmlNode::Element { name, attrs, children });
        }
        node
    }

    /// 🧺️ Hands out the source text so its owner can retire it.
    pub fn take_text(&mut self) -> String {
        self.start = 0;
        self.end = 0;
        self.pos = 0;
        std::mem::take(&mut self.text)
    }

    fn prolog_step(&mut self) -> Result<(), String> {
        if self.start == self.end {
            self.phase = Phase::Epilog;
            return Ok(());
        }
        let source = &self.text[self.start..self.end];
        let mut pos = 0;
        self.declaration = parse_xml_declaration_prolog(source, &mut pos)?;
        let (doctype, prolog) = parse_misc(source, &mut pos, true)?;
        self.doctype = doctype;
        self.prolog = prolog;
        self.pos = pos;
        self.phase = Phase::Content;
        Ok(())
    }

    fn epilog_step(&mut self) -> Result<XmlDocument, String> {
        self.phase = Phase::Done;
        if self.start == self.end {
            return Ok(XmlDocument::default());
        }
        let source = &self.text[self.start..self.end];
        let mut pos = self.pos;
        let (_, epilog) = parse_misc(source, &mut pos, false)?;
        if pos < source.len() {
            return Err("trailing content after root element".into());
        }
        self.pos = pos;
        let document = XmlDocument { root: self.root.take(), doctype: self.doctype.take(), declaration: self.declaration.take(), prolog: std::mem::take(&mut self.prolog), epilog };
        validate_xml_document_boundaries(&document)?;
        Ok(document)
    }

    fn attach(&mut self, node: XmlNode) {
        match self.open.last_mut() {
            Some(parent) => parent.children.push(node),
            None => {
                self.root = Some(node);
                self.phase = Phase::Epilog;
            }
        }
    }

    fn token(&mut self) -> Result<(), String> {
        let source = &self.text[self.start..self.end];
        let mut pos = self.pos;
        let Some(parent) = self.open.last_mut() else {
            if pos >= source.len() || !source[pos..].starts_with('<') {
                return Err("expected element start".into());
            }
            if source[pos..].starts_with("</") {
                return Err("unexpected closing tag".into());
            }
            let node = Self::start_tag(source, &mut pos, &mut self.open)?;
            self.pos = pos;
            if let Some(node) = node {
                self.attach(node);
            }
            return Ok(());
        };
        if pos >= source.len() {
            return Err(format!("unclosed element <{}>: unexpected end of input", parent.name));
        }
        let rest = &source[pos..];
        if rest.starts_with("</") {
            pos += 2;
            let close = parse_name(source, &mut pos)?;
            skip_ws(source, &mut pos);
            if !source[pos..].starts_with('>') {
                return Err("expected > on closing tag".into());
            }
            pos += 1;
            let OpenElement { name, attrs, children } = self.open.pop().expect("open element checked above");
            if close != name {
                return Err(format!("closing tag mismatch: expected </{}>, got </{}>", name, close));
            }
            self.pos = pos;
            self.attach(XmlNode::Element { name, attrs, children });
            return Ok(());
        }
        let node = if rest.starts_with("<![CDATA[") {
            pos += "<![CDATA[".len();
            let end = source[pos..].find("]]>").ok_or("unclosed CDATA section")?;
            let text = source[pos..pos + end].to_string();
            pos += end + 3;
            Some(XmlNode::CData { text })
        } else if rest.starts_with("<!--") {
            pos += 4;
            let end = source[pos..].find("-->").ok_or("unclosed comment")?;
            let text = source[pos..pos + end].to_string();
            pos += end + 3;
            Some(XmlNode::Comment { text })
        } else if rest.starts_with("<?") {
            pos += 2;
            let target = parse_name(source, &mut pos)?;
            skip_ws(source, &mut pos);
            let end = source[pos..].find("?>").ok_or("unclosed processing instruction")?;
            let data = source[pos..pos + end].to_string();
            pos += end + 2;
            Some(XmlNode::ProcessingInstruction { target, data })
        } else if rest.starts_with('<') {
            Self::start_tag(source, &mut pos, &mut self.open)?
        } else {
            let length = rest.find('<').unwrap_or(rest.len());
            let text = xml_unescape_text(&rest[..length])?;
            pos += length;
            Some(XmlNode::Text { text })
        };
        self.pos = pos;
        if let Some(node) = node {
            self.attach(node);
        }
        Ok(())
    }

    fn start_tag(source: &str, pos: &mut usize, open: &mut Vec<OpenElement>) -> Result<Option<XmlNode>, String> {
        *pos += 1;
        let name = parse_name(source, pos)?;
        let attrs = parse_attrs(source, pos)?;
        skip_ws(source, pos);
        if source[*pos..].starts_with("/>") {
            *pos += 2;
            return Ok(Some(XmlNode::Element { name, attrs, children: Vec::new() }));
        }
        if !source[*pos..].starts_with('>') {
            return Err("expected > or />".into());
        }
        *pos += 1;
        open.push(OpenElement { name, attrs, children: Vec::new() });
        Ok(None)
    }
}
