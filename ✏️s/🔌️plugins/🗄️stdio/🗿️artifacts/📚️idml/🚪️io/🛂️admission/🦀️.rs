//! 🛂 Borrowed IDML package admission with funded numeric metadata pages.
use crate::IdmlSnapshot;
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::validation::{XmlValidationCursor,XmlValidationEvent,XmlSpan};
const PAGE:usize=256;
const PAGE_BYTES:usize=PAGE*std::mem::size_of::<u32>();
const PAGE_COPY_BYTES:usize=PAGE_BYTES*2;
/// 📚 Borrowed ordered package fields owned by the canonical IDML admission boundary.
pub trait IdmlAdmissionParts {
    fn source_identity(&self)->usize {self as *const Self as *const () as usize}

    fn part_count(&self)->usize;
    fn part_path(&self,index:usize)->&str;
    fn part_bytes(&self,index:usize)->&[u8];
    fn dom_version(&self)->Option<&str>;
}
impl IdmlAdmissionParts for IdmlSnapshot {
    fn part_count(&self)->usize {self.parts.len()}
    fn part_path(&self,index:usize)->&str {&self.parts[index].path}
    fn part_bytes(&self,index:usize)->&[u8] {&self.parts[index].bytes}
    fn dom_version(&self)->Option<&str> {Some(&self.dom_version)}
}
impl IdmlAdmissionParts for semio_s_artifact_stdio_zip::ZipSnapshot {
    fn part_count(&self)->usize {self.entries.len()}
    fn part_path(&self,index:usize)->&str {&self.entries[index].name}
    fn part_bytes(&self,index:usize)->&[u8] {&self.entries[index].data}
    fn dom_version(&self)->Option<&str> {None}
}
/// 🧷 Replacement bytes borrow the same immutable original mutation payload.
#[derive(Clone,Copy)]
pub struct IdmlPartReplacement<'a> { pub index:usize, pub bytes:&'a [u8] }
/// 📚 An immutable snapshot with optional replacement bytes at a resolved original index.
#[derive(Clone,Copy)]
pub struct IdmlAdmissionSource<'a> { pub snapshot:&'a dyn IdmlAdmissionParts, pub replacement:Option<IdmlPartReplacement<'a>>, pub compare_version:bool }
impl<'a> IdmlAdmissionSource<'a> {
    /// 📸 Admits the snapshot's persisted version against its designmap source.
    pub fn snapshot(snapshot:&'a IdmlSnapshot)->Self { Self { snapshot,replacement:None,compare_version:true } }
    /// 📦 Borrows an ordered package provider while inferring its lexical version.
    pub fn parts(snapshot:&'a dyn IdmlAdmissionParts)->Self {Self{snapshot,replacement:None,compare_version:false}}
    fn data(self,index:usize)->&'a [u8] { if let Some(replacement)=self.replacement { if replacement.index==index { return replacement.bytes; } } self.snapshot.part_bytes(index) }
    fn identity(self)->SourceIdentity { SourceIdentity { snapshot:self.snapshot.source_identity(), replacement:self.replacement.map(|part|(part.index,part.bytes.as_ptr() as usize,part.bytes.len())),compare_version:self.compare_version } }
}
#[derive(Clone,Copy,PartialEq,Eq)]
struct SourceIdentity { snapshot:usize,replacement:Option<(usize,usize,usize)>,compare_version:bool }
/// 📏 Separate source/comparison work and exact metadata allocation/closure authority.
#[derive(Clone,Copy,Debug,Default)]
pub struct IdmlAdmissionGrant { pub maximum_work:usize,pub maximum_copy_bytes:usize,pub maximum_capacity_bytes:usize,pub maximum_release_bytes:usize }
/// 🏷 The decoded version remains a lexical source view until its funded semantic copy.
#[derive(Clone,Copy,Debug)]
pub struct IdmlVersionView { pub part_index:usize,pub value:XmlSpan,pub decoded_length:usize }
/// 🧾 Progress, metadata ownership and terminal admission evidence.
#[derive(Clone,Copy,Debug)]
pub struct IdmlAdmissionStep { pub work:usize,pub copied_bytes:usize,pub retained_capacity_bytes:usize,pub released_bytes:usize,pub done:bool,pub error:Option<&'static str>,pub version:Option<IdmlVersionView> }
/// 🧭 No member strings, byte payloads, or XML DOMs are copied by this cursor.
pub struct IdmlAdmissionCursor {
    source:Option<SourceIdentity>,
    pages:[Option<Box<[u32;PAGE]>>;128],
    page_count:usize,
    page_need:usize,
    mask:usize,
    mode:&'static str,
    closing:bool,
    error:Option<&'static str>,
    index:usize,
    offset:usize,
    bytes:usize,
    segment:usize,
    dots:bool,
    hash:u32,
    slot:usize,
    collision:usize,
    xml:XmlValidationCursor,
    designmap:Option<usize>,
    version_span:Option<XmlSpan>,
    version_offset:usize,
    decoded:usize,
    major:usize,
    minor:usize,
    dot:bool,
    entity:u32,
    scalar:u32,
    digits:usize,
    result:Option<IdmlVersionView>,
}
impl Default for IdmlAdmissionCursor { fn default()->Self { Self::new() } }
impl IdmlAdmissionCursor {
    /// 🌱 Fixed metadata starts cold; source-sized pages are allocated only under explicit grants.
    pub fn new()->Self { Self { source:None,pages:std::array::from_fn(|_|None),page_count:0,page_need:0,mask:0,mode:"init",closing:false,error:None,index:0,offset:0,bytes:0,segment:0,dots:true,hash:2166136261,slot:0,collision:0,xml:XmlValidationCursor::new(),designmap:None,version_span:None,version_offset:0,decoded:0,major:0,minor:0,dot:false,entity:0,scalar:0,digits:0,result:None } }
    fn fail(&mut self,error:&'static str) { self.error=Some(error); }
    /// 🏗 Quotes exactly one fixed numeric name-index page.
    pub fn next_capacity_bytes(&self)->usize { if self.mode=="allocate" && self.page_count<self.page_need { PAGE_BYTES } else { 0 } }
    /// 📝 Quotes conservative stack initialization plus the boxed numeric page copy.
    pub fn next_copy_bytes(&self)->usize { if self.next_capacity_bytes()!=0 { PAGE_COPY_BYTES } else { 0 } }
    /// ♻ Quotes the next exact boxed page layout.
    pub fn next_release_bytes(&self)->usize { if self.page_count!=0 { PAGE_BYTES } else { 0 } }
    /// 🛑 Revokes source loans before any numeric backing is released.
    pub fn begin_close(&mut self) { self.closing=true;self.source=None;self.xml.close(); }
    /// 🪜 Releases at most one admitted page under exact original ownership.
    pub fn close_one(&mut self,maximum_release_bytes:usize)->IdmlAdmissionStep {
        let mut released_bytes=0;
        if !self.closing { return IdmlAdmissionStep { work:0,copied_bytes:0,retained_capacity_bytes:0,released_bytes:0,done:false,error:Some("stdio.idml.admission-close"),version:None }; }
        if self.page_count!=0 && maximum_release_bytes>=PAGE_BYTES { self.page_count-=1;drop(self.pages[self.page_count].take());released_bytes=PAGE_BYTES; }
        IdmlAdmissionStep { work:0,copied_bytes:0,retained_capacity_bytes:0,released_bytes,done:self.page_count==0,error:None,version:None }
    }
    /// 🪹 Confirms the source and every boxed numeric page have been retired.
    pub fn terminal_is_empty(&self)->bool { self.closing && self.page_count==0 && self.source.is_none() && self.xml.terminal_is_empty() }
    fn emit_version(&mut self,c:u32,source:IdmlAdmissionSource<'_>) {
        if c==46 { if self.dot || self.major==0 { self.fail("stdio.idml.dom-version");return; }self.dot=true; }
        else if (48..=57).contains(&c) { if self.dot {self.minor+=1;}else{self.major+=1;} }
        else { self.fail("stdio.idml.dom-version");return; }
        if source.compare_version && source.snapshot.dom_version().unwrap_or("").as_bytes().get(self.decoded).copied().map(u32::from)!=Some(c) { self.fail("stdio.idml.dom-version-mismatch"); }
        self.decoded+=1;
    }
    /// 🚶 Source/name work can pause at every byte; allocation copies need separate authority.
    pub fn advance(&mut self,source:IdmlAdmissionSource<'_>,grant:IdmlAdmissionGrant)->IdmlAdmissionStep {
        if self.closing { return IdmlAdmissionStep { work:0,copied_bytes:0,retained_capacity_bytes:0,released_bytes:0,done:false,error:Some("stdio.idml.admission-closed"),version:None }; }
        let identity=source.identity();
        if let Some(bound)=self.source { if bound!=identity { self.fail("stdio.idml.admission-source"); } } else { self.source=Some(identity); }
        let count=source.snapshot.part_count();
        let mut work=0;let mut copied_bytes=0;let mut retained_capacity_bytes=0;
        while work<grant.maximum_work && self.error.is_none() && self.mode!="done" {
            match self.mode {
                "init" => {
                    if count>16384 { self.fail("stdio.idml.member-limit");break; }
                    if source.replacement.is_some_and(|replacement|replacement.index>=count) { self.fail("stdio.idml.missing-member");break; }
                    if (count==0 || source.snapshot.part_path(0)!="mimetype") || source.data(0).len()!=crate::PACKAGE_MIME.len() { self.fail("stdio.idml.mimetype");break; }
                    self.offset=0;self.mode="mime";work+=1;
                }
                "mime" => {
                    if self.offset==crate::PACKAGE_MIME.len() { let slots=(count*2).next_power_of_two();self.mask=slots-1;self.page_need=slots.div_ceil(PAGE);self.mode="allocate";work+=1;continue; }
                    if source.data(0)[self.offset]!=crate::PACKAGE_MIME.as_bytes()[self.offset] {self.fail("stdio.idml.mimetype");}self.offset+=1;work+=1;
                }
                "allocate" => {
                    if self.page_count==self.page_need {self.mode="part";work+=1;continue;}
                    if grant.maximum_capacity_bytes-retained_capacity_bytes<PAGE_BYTES || grant.maximum_copy_bytes-copied_bytes<PAGE_COPY_BYTES {break;}
                    self.pages[self.page_count]=Some(Box::new([0;PAGE]));self.page_count+=1;retained_capacity_bytes+=PAGE_BYTES;copied_bytes+=PAGE_COPY_BYTES;work+=1;break;
                }
                "part" => {
                    if self.index==count { if self.designmap.is_none() {self.fail("stdio.idml.designmap");}else if self.version_span.is_none(){self.fail("stdio.idml.dom-version");}else{self.version_offset=self.version_span.unwrap().start;self.mode="version";}work+=1;continue; }
                    let name=source.snapshot.part_path(self.index);let data=source.data(self.index);
                    if name.is_empty(){self.fail("stdio.idml.member-path");break;}
                    if name.ends_with('/') && !data.is_empty(){self.fail("stdio.idml.directory-payload");break;}
                    let Some(bytes)=self.bytes.checked_add(data.len()).filter(|bytes|*bytes<=8388608) else {self.fail("stdio.idml.byte-limit");break;};self.bytes=bytes;
                    self.offset=0;self.segment=0;self.dots=true;self.hash=2166136261;self.mode="path";work+=1;
                }
                "path" => {
                    let name=source.snapshot.part_path(self.index).as_bytes();
                    if self.offset==name.len(){if self.segment!=0 && self.dots && self.segment<=2 {self.fail("stdio.idml.member-path");break;}self.slot=(self.hash as usize)&self.mask;self.mode="lookup";work+=1;continue;}
                    let byte=name[self.offset];self.offset+=1;
                    if byte==92 || byte==58 || byte==47 && (self.segment==0 || self.dots && self.segment<=2){self.fail("stdio.idml.member-path");break;}
                    if byte==47{self.segment=0;self.dots=true;}else{self.segment+=1;self.dots=self.dots && byte==46;}
                    self.hash=(self.hash^byte as u32).wrapping_mul(16777619);work+=1;
                }
                "lookup" => {
                    let previous=self.pages[self.slot/PAGE].as_ref().unwrap()[self.slot%PAGE];
                    if previous!=0{self.collision=previous as usize-1;self.offset=0;self.mode="collision";work+=1;continue;}
                    self.pages[self.slot/PAGE].as_mut().unwrap()[self.slot%PAGE]=self.index as u32+1;
                    if source.snapshot.part_path(self.index)=="designmap.xml"{self.designmap=Some(self.index);}
                    if source.snapshot.part_path(self.index).ends_with(".xml"){self.xml.restart();self.mode="xml";}else{self.offset=0;self.mode="bytes";}work+=1;
                }
                "collision" => {
                    let a=source.snapshot.part_path(self.collision).as_bytes();let b=source.snapshot.part_path(self.index).as_bytes();
                    if a.len()!=b.len(){self.slot=(self.slot+1)&self.mask;self.mode="lookup";work+=1;continue;}
                    if self.offset==a.len(){self.fail("stdio.idml.duplicate-member");break;}
                    if a[self.offset]!=b[self.offset]{self.slot=(self.slot+1)&self.mask;self.mode="lookup";}else{self.offset+=1;}work+=1;
                }
                "xml" => {
                    let data=source.data(self.index);let step=self.xml.advance(data,grant.maximum_work-work);work+=step.work;
                    if let Some(error)=step.error{self.fail(error);break;}
                    if self.designmap==Some(self.index){if let Some(event)=step.event{match event{
                        XmlValidationEvent::Element{depth:1,name}=>{if &data[name.start..name.end]!=b"Document"{self.fail("stdio.idml.designmap");}},
                        XmlValidationEvent::Attribute{depth:1,name,value}=>{if &data[name.start..name.end]==b"DOMVersion"{self.version_span=Some(value);}},
                        _=>{}
                    }}}
                    if step.done{self.index+=1;self.mode="part";}
                }
                "bytes" => {let data=source.data(self.index);if self.offset==data.len(){self.index+=1;self.mode="part";}else{self.offset+=1;}work+=1;}
                "version" => {
                    let data=source.data(self.designmap.unwrap());let end=self.version_span.unwrap().end;
                    if self.version_offset==end{
                        if self.entity!=0 || !self.dot || self.major==0 || self.minor==0{self.fail("stdio.idml.dom-version");}
                        else if source.compare_version && source.snapshot.dom_version().map(str::len)!=Some(self.decoded){self.fail("stdio.idml.dom-version-mismatch");}
                        else{self.result=Some(IdmlVersionView{part_index:self.designmap.unwrap(),value:self.version_span.unwrap(),decoded_length:self.decoded});self.mode="done";}work+=1;continue;
                    }
                    let c=data[self.version_offset] as u32;self.version_offset+=1;work+=1;
                    if self.entity==0{if c==38{self.entity=1;self.scalar=0;self.digits=0;}else{self.emit_version(c,source);}continue;}
                    if self.entity==1{if c!=35{self.fail("stdio.idml.dom-version");}else{self.entity=10;}continue;}
                    if self.entity==10 && self.digits==0 && c==120{self.entity=16;continue;}
                    if c==59{if self.digits==0{self.fail("stdio.idml.dom-version");}else{self.emit_version(self.scalar,source);}self.entity=0;continue;}
                    let digit=if(48..=57).contains(&c){Some(c-48)}else if self.entity==16 && (65..=70).contains(&c){Some(c-55)}else if self.entity==16 && (97..=102).contains(&c){Some(c-87)}else{None};
                    if let Some(digit)=digit{if digit>=self.entity || self.scalar>0x10ffff{self.fail("stdio.idml.dom-version");}else{self.scalar=self.scalar*self.entity+digit;self.digits+=1;}}else{self.fail("stdio.idml.dom-version");}
                }
                _=>self.fail("stdio.idml.admission-state"),
            }
        }
        IdmlAdmissionStep{work,copied_bytes,retained_capacity_bytes,released_bytes:0,done:self.mode=="done",error:self.error,version:self.result}
    }
}

/// 🔤 A decoded version byte or one bounded lexical progress turn.
#[derive(Clone,Copy,Debug)]
pub struct IdmlVersionDecodeStep {pub work:usize,pub byte:Option<u8>,pub done:bool,pub error:Option<&'static str>}
/// 🔢 Decodes admitted numeric references without copying the original XML member.
pub struct IdmlVersionDecoder {view:IdmlVersionView,at:usize,entity:u32,value:u32,digits:usize,count:usize,source:Option<(usize,usize)>,error:Option<&'static str>}
impl IdmlVersionDecoder {
    /// 🌱 Captures only the original admitted scalar spans.
    pub fn new(view:IdmlVersionView)->Self {Self{view,at:view.value.start,entity:0,value:0,digits:0,count:0,source:None,error:None}}
    /// 🚶 Emits at most one ASCII byte after bounded original member scanning.
    pub fn advance(&mut self,source:&[u8],maximum_work:usize)->IdmlVersionDecodeStep {
        let identity=(source.as_ptr() as usize,source.len());if let Some(bound)=self.source{if bound!=identity{self.error=Some("stdio.idml.version-source");}}else{self.source=Some(identity);}
        if self.view.value.end>source.len()||self.view.value.end<self.at{self.error=Some("stdio.idml.version-span");}
        let mut work=0;
        while work<maximum_work&&self.error.is_none(){
            if self.at==self.view.value.end{if self.entity!=0||self.count!=self.view.decoded_length{self.error=Some("stdio.idml.dom-version");}return IdmlVersionDecodeStep{work:work+1,byte:None,done:self.error.is_none(),error:self.error};}
            let c=source[self.at] as u32;self.at+=1;work+=1;let mut output=None;
            if self.entity==0{if c==38{self.entity=1;self.value=0;self.digits=0;}else{output=Some(c);}}
            else if self.entity==1{if c!=35{self.error=Some("stdio.idml.dom-version");}else{self.entity=10;}}
            else if self.entity==10&&self.digits==0&&c==120{self.entity=16;}
            else if c==59{if self.digits==0{self.error=Some("stdio.idml.dom-version");}else{output=Some(self.value);}self.entity=0;}
            else{
                let digit=if(48..=57).contains(&c){Some(c-48)}else if self.entity==16&&(65..=70).contains(&c){Some(c-55)}else if self.entity==16&&(97..=102).contains(&c){Some(c-87)}else{None};
                if let Some(digit)=digit{if digit>=self.entity||self.value>0x10ffff{self.error=Some("stdio.idml.dom-version");}else{self.value=self.value*self.entity+digit;self.digits+=1;}}else{self.error=Some("stdio.idml.dom-version");}
            }
            if let Some(output)=output{if !(output==46||(48..=57).contains(&output))||self.count>=self.view.decoded_length{self.error=Some("stdio.idml.dom-version");}else{self.count+=1;return IdmlVersionDecodeStep{work,byte:Some(output as u8),done:false,error:None};}}
        }
        IdmlVersionDecodeStep{work,byte:None,done:false,error:self.error}
    }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

/// 🧳 Genuine immutable source custody for bounded package admission.
#[path="🧳️retained/🦀️.rs"]
pub mod retained;

/// 🔎 Fixed original name hashing/probing/comparison state; it owns no path string.
pub struct IdmlNameLookup {key:Option<(usize,usize)>,phase:u8,offset:usize,hash:u32,slot:usize,candidate:usize,done:bool,index:Option<usize>,error:Option<&'static str>}
impl IdmlNameLookup {
    /// 🔄 Reuses fixed numeric state after its original query settles.
    pub fn restart(&mut self){*self=Self::new();}
    /// 🌱 No original source or metadata page is cloned by this constructor.
    pub fn new()->Self {Self{key:None,phase:0,offset:0,hash:2166136261,slot:0,candidate:0,done:false,index:None,error:None}}
}
impl Default for IdmlNameLookup {fn default()->Self{Self::new()}}
/// 🎯 A missing name is terminal with no index; an unfinished lookup is not a miss.
#[derive(Clone,Copy,Debug)]
pub struct IdmlLookupStep {pub work:usize,pub done:bool,pub index:Option<usize>,pub error:Option<&'static str>}
impl IdmlAdmissionCursor {
    /// 🧷 Admitted evidence belongs to this exact immutable original view.
    pub fn admits_source(&self,source:IdmlAdmissionSource<'_>)->bool {self.mode=="done"&&self.error.is_none()&&!self.closing&&self.source==Some(source.identity())}
    /// 🏷 Returns original version evidence only after complete package admission.
    pub fn version_view(&self)->Option<IdmlVersionView> {if self.mode=="done"&&self.error.is_none()&&!self.closing{self.result}else{None}}
    /// 🔎 Reuses admitted numeric metadata under bounded original key and member reads.
    pub fn lookup(&self,source:IdmlAdmissionSource<'_>,key:&str,cursor:&mut IdmlNameLookup,maximum_work:usize)->IdmlLookupStep {
        if self.mode!="done"||self.error.is_some()||self.closing{return IdmlLookupStep{work:0,done:false,index:None,error:Some("stdio.idml.lookup-admission")};}
        if self.source!=Some(source.identity()){return IdmlLookupStep{work:0,done:false,index:None,error:Some("stdio.idml.lookup-source")};}
        let identity=(key.as_ptr() as usize,key.len());if let Some(bound)=cursor.key{if bound!=identity{cursor.error=Some("stdio.idml.lookup-key");}}else{cursor.key=Some(identity);}
        let key=key.as_bytes();let mut work=0;
        while work<maximum_work&&cursor.error.is_none()&&!cursor.done{
            if cursor.phase==0{if cursor.offset==key.len(){cursor.slot=(cursor.hash as usize)&self.mask;cursor.phase=1;}else{cursor.hash=(cursor.hash^key[cursor.offset] as u32).wrapping_mul(16777619);cursor.offset+=1;}work+=1;continue;}
            if cursor.phase==1{let row=self.pages[cursor.slot/PAGE].as_ref().unwrap()[cursor.slot%PAGE];if row==0{cursor.done=true;work+=1;continue;}cursor.candidate=row as usize-1;cursor.offset=0;cursor.phase=2;work+=1;continue;}
            let candidate=source.snapshot.part_path(cursor.candidate).as_bytes();
            if candidate.len()!=key.len(){cursor.slot=(cursor.slot+1)&self.mask;cursor.phase=1;work+=1;continue;}
            if cursor.offset==candidate.len(){cursor.done=true;cursor.index=Some(cursor.candidate);work+=1;continue;}
            if candidate[cursor.offset]!=key[cursor.offset]{cursor.slot=(cursor.slot+1)&self.mask;cursor.phase=1;}else{cursor.offset+=1;}work+=1;
        }
        IdmlLookupStep{work,done:cursor.done,index:cursor.index,error:cursor.error}
    }
}

/// 🔺 Read-only original coordinate planning for real sparse IDML fields.
#[path="🔺️change/🦀️.rs"]
pub mod change;

/// 🔢 Funded typed version fields preserve their genuine original member source.
#[path="🔢️version/🧳️retained/🦀️.rs"]
pub mod retained_version;
