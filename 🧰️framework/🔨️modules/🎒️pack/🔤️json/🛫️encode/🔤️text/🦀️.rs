//! 🔤️ Borrowed native UTF8 roles retain their original chunk owners during canonical encoding.
use semio_framework_value::paged::Utf8Text;

#[derive(Clone, Copy)]
pub enum ArtifactCanonicalJsonText<'a> {
    Contiguous(&'a str),
    Native(&'a (dyn Utf8Text + Sync)),
}

impl std::fmt::Debug for ArtifactCanonicalJsonText<'_> {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Contiguous(text) => formatter.debug_tuple("Contiguous").field(text).finish(),
            Self::Native(text) => formatter.debug_struct("Native").field("bytes", &text.text_bytes()).field("chunks", &text.text_chunk_count()).finish(),
        }
    }
}

impl<'a> From<&'a str> for ArtifactCanonicalJsonText<'a> {
    fn from(text: &'a str) -> Self { Self::Contiguous(text) }
}

impl ArtifactCanonicalJsonText<'_> {
    pub fn next_byte(self, chunk: &mut usize, offset: &mut usize) -> Result<Option<u8>, &'static str> {
        match self {
            Self::Contiguous(text) => { let Some(byte) = text.as_bytes().get(*offset) else { return Ok(None); }; *offset += 1; Ok(Some(*byte)) }
            Self::Native(text) => {
                if text.text_bytes() == 0 || *chunk >= text.text_chunk_count() { return Ok(None); }
                let body = text.text_chunk(*chunk).ok_or("canonical-edit.native-text-chunk-missing")?;
                if body.is_empty() { return Err("canonical-edit.native-text-empty-chunk"); }
                if let Some(byte) = body.as_bytes().get(*offset) { *offset += 1; return Ok(Some(*byte)); }
                if *offset != body.len() { return Err("canonical-edit.native-text-offset"); }
                *chunk += 1; *offset = 0;
                if *chunk == text.text_chunk_count() { return Ok(None); }
                let body = text.text_chunk(*chunk).ok_or("canonical-edit.native-text-chunk-missing")?;
                let byte = *body.as_bytes().first().ok_or("canonical-edit.native-text-empty-chunk")?;
                *offset = 1;
                Ok(Some(byte))
            }
        }
    }
}

/// 🔡️ Projects one canonical JSON escape byte without materializing an escaped string.
pub fn canonical_escaped_byte(byte:u8,index:usize)->Option<u8> {
    match byte {
        b'"'|b'\\'=>[b'\\',byte].get(index).copied(),
        8|9|10|12|13=>[b'\\',match byte{8=>b'b',9=>b't',10=>b'n',12=>b'f',_=>b'r'}].get(index).copied(),
        0..=31=>match index{0=>Some(b'\\'),1=>Some(b'u'),2|3=>Some(b'0'),4=>Some(b"0123456789abcdef"[(byte>>4)as usize]),5=>Some(b"0123456789abcdef"[(byte&15)as usize]),_=>None},
        _=>(index==0).then_some(byte),
    }
}
/// 🔣️ Writes the same escape projection into a caller-owned six-byte scratch cell.
pub fn canonical_escape(byte:u8,escape:&mut[u8;6])->usize {
    let mut count=0;
    while let Some(value)=canonical_escaped_byte(byte,count){escape[count]=value;count+=1;}
    count
}

use semio_framework_value::{RetirementDemand,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};
#[derive(Debug)]
pub struct ArtifactCanonicalJsonTextCursor<'a> {
    text:Option<ArtifactCanonicalJsonText<'a>>,
    chunk:usize,
    offset:usize,
    pending:Option<u8>,
    escape_index:usize,
    phase:u8,
    closing:bool,
}
#[derive(Clone,Copy,Debug)]
pub struct ArtifactCanonicalJsonTextStep {pub step:RetainedCloneStep,pub written_bytes:usize}
impl<'a> ArtifactCanonicalJsonTextCursor<'a> {
    pub fn constructor_demand()->RetirementDemand {RetirementDemand{copy_bytes:std::mem::size_of::<Self>(),depth:1,..RetirementDemand::default()}}
    pub fn admit(text:ArtifactCanonicalJsonText<'a>,grant:RetainedCloneGrant)->Result<(Self,RetainedCloneProgress),(&'static str,ArtifactCanonicalJsonText<'a>)> {
        let demand=Self::constructor_demand();
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Err(("pack-json.text-birth-grant",text));}
        Ok((Self{text:Some(text),chunk:0,offset:0,pending:None,escape_index:0,phase:0,closing:false},RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..RetainedCloneProgress::default()}))
    }
    pub fn is_complete(&self)->bool {self.phase==3}
    pub fn terminal_is_empty(&self)->bool {self.text.is_none()}
    pub fn next_demand(&self)->RetirementDemand {
        if self.terminal_is_empty(){return RetirementDemand::default();}
        RetirementDemand{copy_bytes:if self.closing{std::mem::size_of::<Option<ArtifactCanonicalJsonText<'a>>>()}else if self.is_complete(){0}else{1},depth:1,..RetirementDemand::default()}
    }
    pub fn advance(&mut self,output:&mut[u8],grant:RetainedCloneGrant)->Result<ArtifactCanonicalJsonTextStep,&'static str> {
        if self.closing{return Err("pack-json.text-closing");}
        let empty=ArtifactCanonicalJsonTextStep{step:RetainedCloneStep::Progress(RetainedCloneProgress::default()),written_bytes:0};
        if self.is_complete()||grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes==0{return Ok(empty);}
        let mut written=0;
        match self.phase {
            0|2=>{if output.is_empty(){return Ok(empty);}output[0]=b'"';written=1;self.phase=if self.phase==0{1}else{3};},
            1=>if let Some(byte)=self.pending {
                if output.is_empty(){return Ok(empty);}
                output[0]=canonical_escaped_byte(byte,self.escape_index).ok_or("pack-json.text-escape-offset")?;written=1;self.escape_index+=1;
                if canonical_escaped_byte(byte,self.escape_index).is_none(){self.pending=None;self.escape_index=0;}
            }else{
                self.pending=self.text.ok_or("pack-json.text-owner-missing")?.next_byte(&mut self.chunk,&mut self.offset)?;
                if self.pending.is_none(){self.phase=2;}
            },
            _=>return Err("pack-json.text-phase"),
        }
        let progress=RetainedCloneProgress{copied_items:1,copied_bytes:1,..RetainedCloneProgress::default()};
        Ok(ArtifactCanonicalJsonTextStep{step:if self.is_complete(){RetainedCloneStep::Complete(progress)}else{RetainedCloneStep::Progress(progress)},written_bytes:written})
    }
    pub fn begin_close(&mut self){self.closing=true;}
    pub fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,&'static str> {
        if !self.closing{return Err("pack-json.text-close-not-started");}
        if self.terminal_is_empty(){return Ok(RetainedCloneStep::Complete(RetainedCloneProgress::default()));}
        let demand=self.next_demand();
        if grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Ok(RetainedCloneStep::Progress(RetainedCloneProgress::default()));}
        self.text=None;self.pending=None;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..RetainedCloneProgress::default()}))
    }
}
impl Drop for ArtifactCanonicalJsonTextCursor<'_> {fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"native JSON text cursor requires terminal close");}}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
