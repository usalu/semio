//! 🧵 Borrowed UTF8 XML1.0 validation with source and comparison work grants.

/// 📏 A byte span in the immutable original UTF8 source.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct XmlSpan { pub start: usize, pub end: usize }
/// 📬 An admitted borrowed element or attribute boundary.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum XmlValidationEvent {
    Element { depth: usize, name: XmlSpan },
    Attribute { depth: usize, name: XmlSpan, value: XmlSpan },
}
/// 🧾 Work performed and the next syntax event or terminal result.
#[derive(Clone, Copy, Debug)]
pub struct XmlValidationStep {
    pub work: usize,
    pub offset: usize,
    pub done: bool,
    pub event: Option<XmlValidationEvent>,
    pub error: Option<&'static str>,
}
fn space(c: u32) -> bool { matches!(c, 9 | 10 | 13 | 32) }
fn character(c: u32) -> bool { matches!(c, 9 | 10 | 13 | 0x20..=0xd7ff | 0xe000..=0xfffd | 0x10000..=0x10ffff) }
fn start_name(c: u32) -> bool { matches!(c, 58 | 95 | 65..=90 | 97..=122 | 0xc0..=0xd6 | 0xd8..=0xf6 | 0xf8..=0x2ff | 0x370..=0x37d | 0x37f..=0x1fff | 0x200c..=0x200d | 0x2070..=0x218f | 0x2c00..=0x2fef | 0x3001..=0xd7ff | 0xf900..=0xfdcf | 0xfdf0..=0xfffd | 0x10000..=0xeffff) }
fn name_char(c: u32) -> bool { start_name(c) || matches!(c, 45 | 46 | 48..=57 | 0xb7 | 0x300..=0x36f | 0x203f..=0x2040) }
fn matches(source: &[u8], span: XmlSpan, word: &[u8]) -> bool { span.end - span.start == word.len() && &source[span.start..span.end] == word }
/// 🧭 Fixed metadata; each unit reads one byte plus at most16 token lookups, or compares one pair.
pub struct XmlValidationCursor {
    offset: usize,
    mode: &'static str,
    error: Option<&'static str>,
    terminal: bool,
    cancelled: bool,
    source: Option<(usize, usize)>,
    scalar: u32,
    continuation: u8,
    scalar_start: usize,
    minimum: u32,
    depth: usize,
    roots: usize,
    ancestors: [XmlSpan; 256],
    attributes: [XmlSpan; 1024],
    count: usize,
    name_start: usize,
    element: XmlSpan,
    attribute: XmlSpan,
    value_start: usize,
    quote: u32,
    spaced: bool,
    declaration: bool,
    declaration_stage: u8,
    declaration_field: u8,
    declaration_value: [u8; 3],
    declaration_length: usize,
    pi_start: usize,
    token: usize,
    tail: u8,
    return_mode: &'static str,
    entity_mode: u32,
    entity_value: u32,
    entity_digits: usize,
    entity_name: [u8; 5],
    entity_length: usize,
    compare: &'static str,
    compare_index: usize,
    compare_offset: usize,
    pending_event: Option<XmlValidationEvent>,
    root: Option<XmlSpan>,
}
impl Default for XmlValidationCursor { fn default() -> Self { Self::new() } }
impl XmlValidationCursor {
    /// 🌱 Starts without retaining source bytes, strings, or a DOM.
    pub fn new() -> Self {
        Self { offset:0, mode:"text", error:None, terminal:false, cancelled:false, source:None, scalar:0, continuation:0, scalar_start:0, minimum:0, depth:0, roots:0, ancestors:[XmlSpan::default();256], attributes:[XmlSpan::default();1024], count:0, name_start:0, element:XmlSpan::default(), attribute:XmlSpan::default(), value_start:0, quote:0, spaced:false, declaration:false, declaration_stage:0, declaration_field:0, declaration_value:[0;3], declaration_length:0, pi_start:0, token:0, tail:0, return_mode:"", entity_mode:0, entity_value:0, entity_digits:0, entity_name:[0;5], entity_length:0, compare:"", compare_index:0, compare_offset:0, pending_event:None, root:None }
    }
    /// 🔄 Reuses fixed metadata for a new immutable member without allocation.
    pub fn restart(&mut self) {
        self.offset=0; self.mode="text"; self.error=None; self.terminal=false; self.cancelled=false; self.source=None; self.scalar=0; self.continuation=0; self.scalar_start=0; self.minimum=0; self.depth=0; self.roots=0; self.count=0; self.name_start=0; self.value_start=0; self.quote=0; self.spaced=false; self.declaration=false; self.declaration_stage=0; self.declaration_field=0; self.declaration_length=0; self.pi_start=0; self.token=0; self.tail=0; self.return_mode=""; self.entity_mode=0; self.entity_value=0; self.entity_digits=0; self.entity_length=0; self.compare=""; self.compare_index=0; self.compare_offset=0; self.pending_event=None; self.root=None;
    }
    /// 🏷 Returns the admitted root name as an original byte span.
    pub fn root_name(&self) -> Option<XmlSpan> { self.root }
    /// 🧹 Revokes future work without releasing any child-owned allocation.
    pub fn close(&mut self) { self.cancelled=true; self.terminal=true; self.source=None; self.pending_event=None; }
    /// 🪹 Whether cancellation has removed the source binding.
    pub fn terminal_is_empty(&self) -> bool { self.cancelled && self.source.is_none() && self.pending_event.is_none() }
    fn fail(&mut self, error: &'static str) { self.error=Some(error); }
    fn finish_element(&mut self) {
        self.count=0;
        if self.depth==0 {
            if self.roots!=0 { self.fail("xml.validation.root"); return; }
            self.roots+=1; self.root=Some(self.element);
        }
        if self.depth==256 { self.fail("xml.validation.depth"); return; }
        self.ancestors[self.depth]=self.element; self.depth+=1;
        self.pending_event=Some(XmlValidationEvent::Element { depth:self.depth, name:self.element });
    }
    fn finish_attribute(&mut self, end: usize) {
        if self.declaration {
            let value=&self.declaration_value[..self.declaration_length.min(3)];
            if self.declaration_field==1 && value!=b"1.0" || self.declaration_field==2 && self.declaration_length==0 || self.declaration_field==3 && value!=b"yes" && value!=b"no" { self.fail("xml.validation.declaration"); }
            self.declaration_stage=self.declaration_field; self.mode="after-value"; return;
        }
        self.pending_event=Some(XmlValidationEvent::Attribute { depth:self.depth, name:self.attribute, value:XmlSpan { start:self.value_start, end } }); self.mode="after-value";
    }
    fn entity(&mut self, return_mode: &'static str) { self.return_mode=return_mode; self.mode="entity"; self.entity_mode=0; self.entity_length=0; self.entity_digits=0; self.entity_value=0; }
    fn accept(&mut self, c: u32, begin: usize, end: usize, source: &[u8]) {
        let mut again=true;
        while again && self.error.is_none() {
            again=false;
            match self.mode {
                "text" => {
                    if c==0xfeff && begin==0 { continue; }
                    if c==60 { self.mode="lt"; self.tail=0; continue; }
                    if self.depth==0 { if !space(c) { self.fail("xml.validation.root"); } continue; }
                    if c==38 { self.entity("text"); continue; }
                    if c==62 && self.tail==2 { self.fail("xml.validation.character-data"); continue; }
                    self.tail=if c==93 { (self.tail+1).min(2) } else { 0 };
                }
                "lt" => {
                    if c==47 { self.mode="close-start"; continue; }
                    if c==33 { self.mode="bang"; self.token=0; continue; }
                    if c==63 { self.mode="pi-start"; self.pi_start=begin-1; continue; }
                    if !start_name(c) { self.fail("xml.validation.name"); continue; }
                    self.name_start=begin; self.mode="open-name";
                }
                "open-name" => {
                    if name_char(c) { continue; }
                    self.element=XmlSpan { start:self.name_start, end:begin }; self.finish_element(); self.mode="between"; self.spaced=false; again=true;
                }
                "between" => {
                    if space(c) { self.spaced=true; continue; }
                    if c==62 && !self.declaration { self.mode="text"; self.tail=0; continue; }
                    if c==47 && !self.declaration { self.mode="slash"; continue; }
                    if c==63 && self.declaration { if self.declaration_stage==0 { self.fail("xml.validation.declaration"); } else { self.mode="declaration-end"; } continue; }
                    if !self.spaced { self.fail("xml.validation.attribute-space"); continue; }
                    if !start_name(c) { self.fail("xml.validation.name"); continue; }
                    self.name_start=begin; self.mode="attribute-name";
                }
                "attribute-name" => {
                    if name_char(c) { continue; }
                    self.attribute=XmlSpan { start:self.name_start, end:begin };
                    if self.declaration {
                        self.declaration_field=if matches(source,self.attribute,b"version") { 1 } else if matches(source,self.attribute,b"encoding") { 2 } else if matches(source,self.attribute,b"standalone") { 3 } else { 0 };
                        if self.declaration_field==0 || self.declaration_field<=self.declaration_stage || self.declaration_stage==0 && self.declaration_field!=1 { self.fail("xml.validation.declaration"); }
                    } else {
                        if self.count==1024 { self.fail("xml.validation.attributes"); continue; }
                        self.compare="attribute"; self.compare_index=0; self.compare_offset=0;
                    }
                    self.mode="before-equals"; again=true;
                }
                "before-equals" => { if space(c) { continue; } if c!=61 { self.fail("xml.validation.attribute-equals"); continue; } self.mode="before-value"; }
                "before-value" => { if space(c) { continue; } if c!=34 && c!=39 { self.fail("xml.validation.attribute-value"); continue; } self.quote=c; self.value_start=end; self.declaration_length=0; self.mode="value"; }
                "value" => {
                    if c==self.quote { self.finish_attribute(begin); continue; }
                    if c==60 { self.fail("xml.validation.attribute-value"); continue; }
                    if self.declaration {
                        if self.declaration_field==2 {
                            if !(matches!(c,65..=90 | 97..=122) || self.declaration_length>0 && matches!(c,48..=57 | 46 | 95 | 45)) { self.fail("xml.validation.declaration"); }
                        } else if c>127 || self.declaration_length==3 { self.fail("xml.validation.declaration"); } else { self.declaration_value[self.declaration_length]=c as u8; }
                        self.declaration_length+=1; continue;
                    }
                    if c==38 { self.entity("value"); }
                }
                "after-value" => { self.mode="between"; self.spaced=false; again=true; }
                "slash" => { if c!=62 { self.fail("xml.validation.close"); continue; } self.depth-=1; self.mode="text"; self.tail=0; }
                "close-start" => { if !start_name(c) { self.fail("xml.validation.name"); continue; } self.name_start=begin; self.mode="close-name"; }
                "close-name" => {
                    if name_char(c) { continue; }
                    self.element=XmlSpan { start:self.name_start, end:begin };
                    if self.depth==0 { self.fail("xml.validation.close-name"); continue; }
                    self.compare="close"; self.compare_offset=0; self.mode="close-tail"; again=true;
                }
                "close-tail" => { if space(c) { continue; } if c!=62 { self.fail("xml.validation.close"); continue; } self.mode="text"; self.tail=0; }
                "bang" => {
                    if self.token==0 && c==45 { self.mode="comment-open"; continue; }
                    if self.token==0 && c==91 { if self.depth==0 { self.fail("xml.validation.cdata"); continue; } self.mode="cdata-open"; self.token=0; continue; }
                    self.fail(if c==68 { "xml.validation.doctype" } else { "xml.validation.markup" });
                }
                "comment-open" => { if c!=45 { self.fail("xml.validation.comment"); } else { self.mode="comment"; self.tail=0; } }
                "comment" => {
                    if self.tail==2 { if c!=62 { self.fail("xml.validation.comment"); } else { self.mode="text"; self.tail=0; } continue; }
                    self.tail=if c==45 { self.tail+1 } else { 0 };
                }
                "cdata-open" => { let expected=b"CDATA["[self.token] as u32; self.token+=1; if c!=expected { self.fail("xml.validation.cdata"); } else if self.token==6 { self.mode="cdata"; self.tail=0; } }
                "cdata" => { if c==62 && self.tail==2 { self.mode="text"; self.tail=0; } else { self.tail=if c==93 { (self.tail+1).min(2) } else { 0 }; } }
                "pi-start" => { if !start_name(c) { self.fail("xml.validation.processing-instruction"); continue; } self.name_start=begin; self.mode="pi-name"; }
                "pi-name" => {
                    if name_char(c) { continue; }
                    self.element=XmlSpan { start:self.name_start, end:begin };
                    let xml=self.element.end-self.element.start==3 && source[self.name_start..self.element.end].eq_ignore_ascii_case(b"xml");
                    if xml {
                        if !matches(source,self.element,b"xml") || self.pi_start!=0 && !(self.pi_start==3 && source.starts_with(&[239,187,191])) || !space(c) { self.fail("xml.validation.processing-instruction"); continue; }
                        self.declaration=true; self.declaration_stage=0; self.mode="between"; self.spaced=true; continue;
                    }
                    if c==63 { self.mode="pi-end"; continue; }
                    if !space(c) { self.fail("xml.validation.processing-instruction"); continue; }
                    self.mode="pi-body";
                }
                "pi-body" => { if c==63 { self.mode="pi-end"; } }
                "pi-end" => { if c==62 { self.mode="text"; self.tail=0; } else if c!=63 { self.mode="pi-body"; } }
                "declaration-end" => { if c!=62 { self.fail("xml.validation.declaration"); } else { self.declaration=false; self.mode="text"; } }
                "entity" => {
                    if c==59 {
                        if self.entity_mode==0 {
                            let name=&self.entity_name[..self.entity_length];
                            if ![b"amp".as_slice(),b"lt",b"gt",b"apos",b"quot"].contains(&name) { self.fail("xml.validation.entity"); }
                        } else if self.entity_digits==0 { self.fail("xml.validation.entity"); } else if !character(self.entity_value) { self.fail("xml.validation.character"); }
                        self.mode=self.return_mode; self.tail=0; continue;
                    }
                    if self.entity_mode==0 && self.entity_length==0 && c==35 { self.entity_mode=10; continue; }
                    if self.entity_mode==10 && self.entity_digits==0 && c==120 { self.entity_mode=16; continue; }
                    if self.entity_mode==0 { if c>127 || self.entity_length==5 { self.fail("xml.validation.entity"); continue; } self.entity_name[self.entity_length]=c as u8; self.entity_length+=1; continue; }
                    let digit=if (48..=57).contains(&c) { Some(c-48) } else if self.entity_mode==16 && (65..=70).contains(&c) { Some(c-55) } else if self.entity_mode==16 && (97..=102).contains(&c) { Some(c-87) } else { None };
                    if let Some(digit)=digit { if digit>=self.entity_mode || self.entity_value>0x10ffff { self.fail("xml.validation.entity"); continue; } self.entity_digits+=1; self.entity_value=self.entity_value*self.entity_mode+digit; } else { self.fail("xml.validation.entity"); }
                }
                _ => self.fail("xml.validation.state"),
            }
        }
    }
    fn metadata(&mut self, source: &[u8]) {
        if self.compare=="close" {
            let span=self.ancestors[self.depth-1];
            if span.end-span.start!=self.element.end-self.element.start { self.fail("xml.validation.close-name"); self.compare=""; return; }
            if self.compare_offset==span.end-span.start { self.depth-=1; self.compare=""; return; }
            if source[span.start+self.compare_offset]!=source[self.element.start+self.compare_offset] { self.fail("xml.validation.close-name"); }
            self.compare_offset+=1; return;
        }
        if self.compare_index==self.count { self.attributes[self.count]=self.attribute; self.count+=1; self.compare=""; return; }
        let span=self.attributes[self.compare_index];
        if span.end-span.start!=self.attribute.end-self.attribute.start { self.compare_index+=1; self.compare_offset=0; return; }
        if self.compare_offset==span.end-span.start { self.fail("xml.validation.duplicate-attribute"); return; }
        if source[span.start+self.compare_offset]!=source[self.attribute.start+self.compare_offset] { self.compare_index+=1; self.compare_offset=0; } else { self.compare_offset+=1; }
    }
    /// 🚶 Advances within a source-byte/name-comparison work allowance.
    pub fn advance(&mut self, source: &[u8], maximum_work: usize) -> XmlValidationStep {
        if self.cancelled { return XmlValidationStep { work:0, offset:self.offset, done:true, event:None, error:Some("xml.validation.closed") }; }
        let identity=(source.as_ptr() as usize,source.len());
        if let Some(bound)=self.source { if bound!=identity { self.fail("xml.validation.source"); } } else { self.source=Some(identity); }
        if source.len()>8388608 { self.fail("xml.validation.source-limit"); }
        let mut work=0;
        while work<maximum_work && self.error.is_none() && !self.terminal && self.pending_event.is_none() {
            if !self.compare.is_empty() { self.metadata(source); work+=1; continue; }
            if self.offset==source.len() {
                if self.continuation!=0 { self.fail("xml.validation.utf8"); } else if self.mode!="text" || self.depth!=0 { self.fail("xml.validation.unclosed"); } else if self.roots!=1 { self.fail("xml.validation.root"); } else { self.terminal=true; }
                work+=1; break;
            }
            let byte=source[self.offset]; self.offset+=1; work+=1;
            if self.continuation==0 {
                self.scalar_start=self.offset-1;
                if byte<128 { if !character(byte as u32) { self.fail("xml.validation.character"); } else { self.accept(byte as u32,self.scalar_start,self.offset,source); } continue; }
                match byte {
                    194..=223 => { self.scalar=(byte&31) as u32; self.continuation=1; self.minimum=128; }
                    224..=239 => { self.scalar=(byte&15) as u32; self.continuation=2; self.minimum=2048; }
                    240..=244 => { self.scalar=(byte&7) as u32; self.continuation=3; self.minimum=65536; }
                    _ => self.fail("xml.validation.utf8"),
                }
            } else {
                if byte&192!=128 { self.fail("xml.validation.utf8"); continue; }
                self.scalar=self.scalar*64+(byte&63) as u32; self.continuation-=1;
                if self.continuation==0 {
                    if self.scalar<self.minimum || self.scalar>0x10ffff || (0xd800..=0xdfff).contains(&self.scalar) { self.fail("xml.validation.utf8"); } else if !character(self.scalar) { self.fail("xml.validation.character"); } else { self.accept(self.scalar,self.scalar_start,self.offset,source); }
                }
            }
        }
        XmlValidationStep { work, offset:self.offset, done:self.terminal, event:self.pending_event.take(), error:self.error }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
