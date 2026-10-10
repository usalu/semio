//! 🧬️ Universal `.semio` container: content-derived envelope for every OS artifact encoding.

use std::collections::HashMap;
use semio_framework_value::{ValueError, ValueRefusalKind};
use std::sync::{Mutex, OnceLock};

//#region 🔖️Errors
/// ⚠️ Envelope parse or registry lookup failure.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SemioError {
    InvalidPreamble(String),
    InvalidBinaryHeader(String),
    UnknownEnvelope(String),
    AmbiguousEnvelope,
    DecodingControl(semio_framework_value::ValueError),
}

impl std::fmt::Display for SemioError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidPreamble(detail) => write!(formatter, "invalid semio preamble: {detail}"),
            Self::InvalidBinaryHeader(detail) => write!(formatter, "invalid binary semio header: {detail}"),
            Self::UnknownEnvelope(detail) => write!(formatter, "unknown semio envelope: {detail}"),
            Self::AmbiguousEnvelope => formatter.write_str("ambiguous semio envelope match"),
            Self::DecodingControl(detail) => write!(formatter,"{detail}"),
        }
    }
}

impl std::error::Error for SemioError {}

impl SemioError {
    /// 🧭️ Preserves envelope controller refusals at the first-party value boundary.
    pub fn into_value_error(self) -> semio_framework_value::ValueError {
        match self {
            Self::DecodingControl(error) => error,
            error @ Self::UnknownEnvelope(_) => semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, error.to_string()),
            error @ (Self::InvalidPreamble(_) | Self::InvalidBinaryHeader(_) | Self::AmbiguousEnvelope) => semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()),
        }
    }
}

pub type SemioResult<T> = Result<T, SemioError>;
//#endregion 🔖️Errors

//#region 🔖️Component
/// 🧩 Which constitutional encoding a `.semio` file carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Component {
    Dsl,
    Pack,
    Op,
    Spr,
    Cmd,
}

impl Component {
    /// 🏷️ Wire token in the preamble and filename segment.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Dsl => "dsl",
            Self::Pack => "pack",
            Self::Op => "op",
            Self::Spr => "spr",
            Self::Cmd => "cmd",
        }
    }

    /// 📖️ Parses a component token from preamble or filename.
    pub fn parse(token: &str) -> Option<Self> {
        match token {
            "dsl" => Some(Self::Dsl),
            "pack" => Some(Self::Pack),
            "op" => Some(Self::Op),
            "spr" => Some(Self::Spr),
            "cmd" => Some(Self::Cmd),
            _ => None,
        }
    }

    /// 📝 Whether this component uses a text preamble rather than a binary header.
    pub const fn is_text(self) -> bool {
        matches!(self, Self::Dsl | Self::Op | Self::Cmd)
    }
}
//#endregion 🔖️Component

//#region 🔖️Envelope
/// 📨 Identity of a `.semio` payload — derived from content, not from the filename.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SemioEnvelope {
    pub plugin: String,
    pub artifact: String,
    pub component: Component,
    pub version: u16,
}

impl SemioEnvelope {
    /// 🪪️ Matches the complete container identity before a domain codec reads its body.
    pub fn matches_identity(&self, envelope_id: &str, component: Component, version: u16) -> bool {
        envelope_id.strip_prefix(&self.plugin).and_then(|rest| rest.strip_prefix('.')) == Some(self.artifact.as_str()) && self.component == component && self.version == version
    }

    /// 🪪️ Dotted artifact id (`plugin.artifact`) used in `ArtifactDsl::ENVELOPE_ID`.
    pub fn envelope_id(&self) -> String {
        format!("{}.{}", self.plugin, self.artifact)
    }

    /// 📜️ Full preamble line for text encodings, e.g. `semio gis.gismap.dsl v1`.
    pub fn preamble_line(&self) -> String {
        format!("semio {}.{}.{} v{}", self.plugin, self.artifact, self.component.as_str(), self.version)
    }

    /// 🧬️ Binary envelope token without the `semio` keyword.
    pub fn binary_token(&self) -> String {
        format!("{}.{}.{} v{}", self.plugin, self.artifact, self.component.as_str(), self.version)
    }

    /// 📖️ Parses `plugin.artifact` from a document type id.
    pub fn from_envelope_id(envelope_id: &str, component: Component, version: u16) -> SemioResult<Self> {
        let (plugin, artifact) = envelope_id.split_once('.').ok_or_else(|| SemioError::InvalidPreamble(format!("envelope id must be plugin.artifact, got {envelope_id}")))?;
        Ok(Self { plugin: plugin.to_string(), artifact: artifact.to_string(), component, version })
    }
}
//#endregion 🔖️Envelope

//#region 🔖️Binary
/// 🧲️ Magic prefix for binary `.semio` files (`0x89` keeps them non-UTF-8).
pub const BINARY_MAGIC: [u8; 8] = [0x89, b'S', b'E', b'M', 0x0D, 0x0A, 0x1A, 0x0A];

const BINARY_HEADER_PREFIX_LEN: usize = 8 + 4;

/// 📏️ Counts the exact declared framing bytes without allocating an identity token.
pub fn declared_envelope_prefix_len(envelope_id:&str,component:Component,version:u16)->Result<usize,ValueError>{
    if !envelope_id.contains('.') { return Err(ValueError::new(ValueRefusalKind::InvalidValue, "invalid declared envelope identity")); }
    let mut digits=1usize;let mut remaining=version;while remaining>=10{digits+=1;remaining/=10;}
    let token=envelope_id.len().checked_add(component.as_str().len()).and_then(|length|length.checked_add(3+digits)).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit, "native envelope identity length overflow"))?;
    token.checked_add(if component.is_text(){7}else{BINARY_HEADER_PREFIX_LEN}).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit, "native envelope prefix length overflow"))
}

/// 📦️ Wraps a binary payload with the semio binary header.
pub fn wrap_binary(envelope: &SemioEnvelope, payload: &[u8]) -> Vec<u8> {
    let token = envelope.binary_token();
    let token_bytes = token.as_bytes();
    let mut out = Vec::with_capacity(BINARY_HEADER_PREFIX_LEN + token_bytes.len() + payload.len());
    out.extend_from_slice(&BINARY_MAGIC);
    out.extend_from_slice(&(token_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(token_bytes);
    out.extend_from_slice(payload);
    out
}

/// 🛫️ Emits an exact declared binary envelope without allocating an unchecked identity token.
pub fn wrap_binary_controlled(envelope_id:&str,component:Component,version:u16,payload:&[u8],control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<Vec<u8>,ValueError>{
    control.scoped_stage(|control|{control.checkpoint()?;if !envelope_id.contains('.')||component.is_text(){return Err(ValueError::new(ValueRefusalKind::InvalidValue, "invalid declared binary envelope identity"))}let mut digits=[0u8;5];let mut start=digits.len();let mut value=version;loop{start-=1;digits[start]=b'0'+(value%10)as u8;value/=10;if value==0{break;}}let digits=&digits[start..];let length=envelope_id.len().checked_add(component.as_str().len()+3+digits.len()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit, "native envelope identity length overflow"))?;let token=u32::try_from(length).map_err(|_|ValueError::new(ValueRefusalKind::OwnershipLimit, "native envelope identity exceeds u32"))?;let total=BINARY_HEADER_PREFIX_LEN.checked_add(length).and_then(|length|length.checked_add(payload.len())).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit, "native binary envelope length overflow"))?;let mut output=control.allocate_vec::<u8>(total)?;output.extend_from_slice(&BINARY_MAGIC);output.extend_from_slice(&token.to_le_bytes());for bytes in [envelope_id.as_bytes(),b".",component.as_str().as_bytes(),b" v",digits,payload]{control.scoped_stage(|control|{control.begin_stage(bytes.len())?;for fragment in bytes.chunks(65536){output.extend_from_slice(fragment);control.advance(fragment.len())?;}Ok::<_,ValueError>(())})?;}Ok(output)})
}

/// 📖️ Strips the semio binary header and returns envelope + inner payload.
pub fn unwrap_binary(bytes: &[u8]) -> SemioResult<(SemioEnvelope, Vec<u8>)> {
    if bytes.len() < BINARY_HEADER_PREFIX_LEN {
        return Err(SemioError::InvalidBinaryHeader("truncated".into()));
    }
    if bytes[0..8] != BINARY_MAGIC {
        return Err(SemioError::InvalidBinaryHeader("bad magic".into()));
    }
    let token_len = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let token_end = BINARY_HEADER_PREFIX_LEN + token_len;
    if bytes.len() < token_end {
        return Err(SemioError::InvalidBinaryHeader("truncated token".into()));
    }
    let token = std::str::from_utf8(&bytes[BINARY_HEADER_PREFIX_LEN..token_end]).map_err(|_| SemioError::InvalidBinaryHeader("token not utf-8".into()))?;
    let envelope = parse_binary_token(token)?;
    let payload = bytes[token_end..].to_vec();
    Ok((envelope, payload))
}

/// 🚦️ Admits the exact declared envelope and borrows its binary body without a payload copy.
pub fn unwrap_binary_controlled<'input>(bytes:&'input[u8],envelope_id:&str,component:Component,version:u16,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<&'input[u8],ValueError>{
    control.checkpoint()?;
    if bytes.len()<BINARY_HEADER_PREFIX_LEN||bytes[..8]!=BINARY_MAGIC{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"invalid binary semio header: invalid binary envelope prefix"));}
    let length=u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let end=BINARY_HEADER_PREFIX_LEN.checked_add(length).filter(|end|*end<=bytes.len()).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"invalid binary semio header: truncated envelope token"))?;
    if !matches_declared_token(&bytes[BINARY_HEADER_PREFIX_LEN..end],envelope_id,component,version,control)?{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"invalid binary semio header: declared envelope identity mismatch"));}
    Ok(&bytes[end..])
}

fn matches_declared_token(token:&[u8],envelope_id:&str,component:Component,version:u16,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<bool,ValueError>{
    let mut digits=[0u8;5];let mut start=digits.len();let mut remaining=version;
    loop{start-=1;digits[start]=b'0'+(remaining%10) as u8;remaining/=10;if remaining==0{break;}}
    let pieces=[envelope_id.as_bytes(),b".".as_slice(),component.as_str().as_bytes(),b" v".as_slice(),&digits[start..]];
    let expected=pieces.iter().try_fold(0usize,|length,piece|length.checked_add(piece.len())).ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"invalid binary semio header: declared envelope size overflow"))?;
    if token.len()!=expected{return Ok(false);}
    control.scoped_stage(|control|{control.begin_stage(expected)?;let mut position=0;for piece in pieces{for chunk in piece.chunks(256){if token[position..position+chunk.len()]!=*chunk{return Ok(false);}position+=chunk.len();control.advance(chunk.len())?;}}Ok(true)})
}

fn parse_binary_token(token: &str) -> SemioResult<SemioEnvelope> {
    let (body, version_str) = token.rsplit_once(" v").ok_or_else(|| SemioError::InvalidBinaryHeader(format!("missing version in {token}")))?;
    let version: u16 = version_str.parse().map_err(|_| SemioError::InvalidBinaryHeader(format!("bad version in {token}")))?;
    let parts: Vec<&str> = body.split('.').collect();
    if parts.len() < 3 {
        return Err(SemioError::InvalidBinaryHeader(format!("expected plugin.artifact.component, got {body}")));
    }
    let component = Component::parse(parts[parts.len() - 1]).ok_or_else(|| SemioError::InvalidBinaryHeader(format!("unknown component in {body}")))?;
    let artifact = parts[parts.len() - 2].to_string();
    let plugin = parts[..parts.len() - 2].join(".");
    Ok(SemioEnvelope { plugin, artifact, component, version })
}
//#endregion 🔖️Binary

//#region 🔖️Text
/// 📜️ Prepends the mandatory preamble to DSL/op/cmd body text.
pub fn wrap_text(envelope: &SemioEnvelope, body: &str) -> String {
    let mut body_trimmed = body.trim_start_matches('\u{feff}');
    if body_trimmed.starts_with("semio ") {
        if let Ok((_, rest)) = split_text_preamble(body_trimmed) {
            body_trimmed = rest;
        }
    }
    format!("{}\n{}", envelope.preamble_line(), body_trimmed.trim_start())
}

/// 🛫️ Emits canonical declared Text and its exact body under cumulative ownership admission.
pub fn wrap_text_controlled(envelope_id:&str,component:Component,version:u16,body:&str,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<String,ValueError>{
    control.scoped_stage(|control|{control.checkpoint()?;if !envelope_id.contains('.')||!component.is_text(){return Err(ValueError::new(ValueRefusalKind::InvalidValue, "invalid declared text envelope identity"))}let mut digits=[0u8;5];let mut start=digits.len();let mut value=version;loop{start-=1;digits[start]=b'0'+(value%10)as u8;value/=10;if value==0{break;}}let digits=std::str::from_utf8(&digits[start..]).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated, "native envelope version is not ASCII"))?;let total=6usize.checked_add(envelope_id.len()).and_then(|length|length.checked_add(component.as_str().len()+4+digits.len())).and_then(|length|length.checked_add(body.len())).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit, "native text envelope length overflow"))?;control.charge(total)?;let mut output=String::new();output.try_reserve_exact(total).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed, "native text envelope allocation failed"))?;for text in ["semio ",envelope_id,".",component.as_str()," v",digits,"\n",body]{control.scoped_stage(|control|{control.begin_stage(text.len())?;let mut start=0;while start<text.len(){let mut end=(start+65536).min(text.len());while !text.is_char_boundary(end){end-=1;}output.push_str(&text[start..end]);control.advance(end-start)?;start=end;}Ok::<_,ValueError>(())})?;}Ok(output)})
}

/// 🧵️ Owns one declared Text body until physical bytes and source retirement finish.
struct RetainedTextEnvelopeSources { identity: Option<String>, body: Option<String> }
semio_framework_value::artifact_retire_struct!(RetainedTextEnvelopeSources { identity, body });

/// 📜️ Carries the original Text framing and its admitted source frontier.
pub struct RetainedTextEnvelope {
    identity: Option<String>, body: Option<String>, component: Component,
    digits: [u8; 5], digit_start: usize, segment: usize, offset: usize,
    position: usize, output: Option<String>, admitted: bool, complete: bool,
    grant: semio_framework_value::RetainedCloneGrant,
    progress: semio_framework_value::RetainedCloneProgress,
    retirement: Option<semio_framework_value::retirement::controlled::ControlledRetirement<RetainedTextEnvelopeSources>>,
}
impl RetainedTextEnvelope {
    /// 🌱️ Keeps original fields beside independently supplied physical authority.
    pub fn new(identity: String, component: Component, version: u16, body: String, grant: semio_framework_value::RetainedCloneGrant) -> Self {
        let mut digits=[0u8;5]; let mut digit_start=digits.len(); let mut remaining=version;
        loop { digit_start-=1; digits[digit_start]=b'0'+(remaining%10) as u8; remaining/=10; if remaining==0 { break; } }
        Self { identity:Some(identity),body:Some(body),component,digits,digit_start,segment:0,offset:0,position:0,output:None,admitted:false,complete:false,grant,progress:Default::default(),retirement:None }
    }
    /// 🔎️ Borrows the exact source while the operation owns its backing.
    pub fn source_body(&self) -> Option<&str> { self.body.as_deref() }
    /// 📍️ Returns the cumulative physical byte position.
    pub fn position(&self) -> usize { self.position }
    /// 🧾️ Returns every accepted actual producer receipt without resetting authority.
    pub fn progress(&self)->semio_framework_value::RetainedCloneProgress{self.progress}
    /// 🎟️ Keeps all unspent currencies from the independently declared caller grant.
    pub fn remaining_grant(&self)->semio_framework_value::RetainedCloneGrant{semio_framework_value::RetainedCloneGrant{maximum_items:self.grant.maximum_items-self.progress.copied_items,maximum_copy_bytes:self.grant.maximum_copy_bytes-self.progress.copied_bytes,maximum_capacity_bytes:self.grant.maximum_capacity_bytes-self.progress.retained_capacity_bytes,maximum_release_bytes:self.grant.maximum_release_bytes-self.progress.released_bytes,maximum_depth:self.grant.maximum_depth}}
    fn admit(&self,items:usize,demand:semio_framework_value::RetirementDemand)->Result<(),ValueError>{let grant=self.remaining_grant();if items>grant.maximum_items{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"text envelope exhausted original work"))}if demand.depth>grant.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"text envelope exceeds original depth"))}if demand.copy_bytes>grant.maximum_copy_bytes||demand.capacity_bytes>grant.maximum_capacity_bytes||demand.release_bytes>grant.maximum_release_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"text envelope exceeds original physical grant"))}Ok(())}
    fn record(&mut self,progress:semio_framework_value::RetainedCloneProgress)->Result<(),ValueError>{if !progress.fits(self.remaining_grant()){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"text envelope performed an ungranted receipt").with_retained_progress(progress))}self.progress=self.progress.checked_add(progress)?;Ok(())}
    /// ⏱️ Uses body quanta and atomic headers only within the same original grant.
    pub fn step(&mut self,maximum_units:usize,maximum_copy_bytes:usize,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<Option<String>,ValueError>{let before=self.progress;let result=self.advance(maximum_units,maximum_copy_bytes,control);let actual=semio_framework_value::RetainedCloneProgress{copied_items:self.progress.copied_items-before.copied_items,copied_bytes:self.progress.copied_bytes-before.copied_bytes,retained_capacity_bytes:self.progress.retained_capacity_bytes-before.retained_capacity_bytes,released_bytes:self.progress.released_bytes-before.released_bytes};result.map_err(|error|error.with_retained_progress(actual))}
    fn advance(&mut self,maximum_units:usize,maximum_copy_bytes:usize,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<Option<String>,ValueError>{
        use semio_framework_value::{RetirementDemand,RetainedCloneProgress};
        if maximum_units==0||maximum_copy_bytes==0||self.complete{return Ok(None)}
        for _ in 0..maximum_units{
            control.checkpoint()?;
            if !self.admitted{
                let identity=self.identity.as_deref().expect("declared identity retained");
                if !identity.contains('.')||!self.component.is_text(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"invalid declared text envelope identity"))}
                let length=6usize.checked_add(identity.len()).and_then(|length|length.checked_add(self.component.as_str().len()+4+self.digits.len()-self.digit_start)).and_then(|length|length.checked_add(self.body.as_ref().expect("declared body retained").len())).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"native text envelope length overflow"))?;
                self.admit(1,RetirementDemand{capacity_bytes:length,depth:1,..Default::default()})?;
                control.charge(length)?;
                let mut output=String::new();output.try_reserve_exact(length).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"native text envelope allocation failed"))?;
                let capacity=output.capacity();self.output=Some(output);self.admitted=true;
                self.record(RetainedCloneProgress{copied_items:1,retained_capacity_bytes:capacity,..Default::default()})?;
            }else if self.retirement.is_some(){
                if self.retirement.as_ref().unwrap().terminal_is_empty(){
                    self.admit(1,RetirementDemand{copy_bytes:std::mem::size_of::<Option<String>>(),depth:1,..Default::default()})?;
                    control.step()?;
                    self.retirement.take();let output=self.output.take();self.complete=true;
                    self.record(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Option<String>>(),..Default::default()})?;
                    return Ok(output)
                }
                let original=self.remaining_grant();let owner=self.retirement.as_ref().unwrap();let copy=owner.next_copy_byte_demand()?;let quantum=maximum_copy_bytes.max(copy).min(original.maximum_copy_bytes);
                let demand=RetirementDemand{copy_bytes:copy,capacity_bytes:owner.next_capacity_byte_demand(quantum)?,release_bytes:owner.next_release_byte_demand()?,depth:owner.next_depth_demand()?};
                self.admit(1,demand)?;control.charge(demand.capacity_bytes)?;
                let grant=semio_framework_value::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:quantum,..original};
                match self.retirement.as_mut().unwrap().step(grant){Ok(step)=>self.record(step.progress())?,Err(error)=>{self.record(error.retained_progress())?;return Err(error)}}
            }else if self.segment==8{
                let copy=std::mem::size_of::<RetainedTextEnvelopeSources>();self.admit(1,RetirementDemand{copy_bytes:copy,depth:1,..Default::default()})?;
                let sources=RetainedTextEnvelopeSources{identity:self.identity.take(),body:self.body.take()};
                let owner=semio_framework_value::retirement::controlled::ControlledRetirement::new(sources).map_err(|(error,sources)|{self.identity=sources.identity;self.body=sources.body;error})?;self.retirement=Some(owner);
                self.record(RetainedCloneProgress{copied_items:1,copied_bytes:copy,..Default::default()})?;
            }else{
                let text=match self.segment{0=>"semio ",1=>self.identity.as_deref().expect("identity retained"),2=>".",3=>self.component.as_str(),4=>" v",5=>std::str::from_utf8(&self.digits[self.digit_start..]).map_err(|_|ValueError::literal(ValueRefusalKind::InvariantViolated,"native version is not ASCII"))?,6=>"\n",7=>self.body.as_deref().expect("body retained"),_=>unreachable!()};
                let character=text[self.offset..].chars().next();let bytes=character.map_or(0,char::len_utf8);self.admit(1,RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()})?;
                if let Some(character)=character{self.output.as_mut().expect("output admitted").push(character);self.offset+=bytes;self.position+=bytes;}else{self.segment+=1;self.offset=0;}
                self.record(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
            }
            control.step()?;
        }
        Ok(None)
    }
}
impl semio_framework_value::retirement::RetireOwned for RetainedTextEnvelope {
    fn retirement(self) -> Box<dyn semio_framework_value::retirement::RetirementCursor> {
        use semio_framework_value::retirement::{sequence,deferred};sequence(vec![deferred(self.identity),deferred(self.body),deferred(self.output),deferred(self.retirement)])
    }
    fn retirement_birth_bytes(&self) -> Option<usize> { use semio_framework_value::retirement::{sequence_birth_bytes,deferred_birth_bytes_for};sequence_birth_bytes(&[deferred_birth_bytes_for(&self.identity),deferred_birth_bytes_for(&self.body),deferred_birth_bytes_for(&self.output),deferred_birth_bytes_for(&self.retirement)]) }
    fn controlled_retirement_supported() -> bool { true }
}

/// 📖️ Parses a text `.semio` file into envelope and body (without preamble line).
pub fn split_text_preamble(text: &str) -> SemioResult<(SemioEnvelope, &str)> {
    let mut lines = text.lines();
    let first = lines.next().ok_or_else(|| SemioError::InvalidPreamble("empty file".into()))?.trim();
    let envelope = parse_preamble_line(first)?;
    let rest = text[first.len()..].trim_start_matches(['\r', '\n']);
    Ok((envelope, rest))
}

/// 🛂️ Borrows canonical text after exact owner, component and version admission.
pub fn split_text_preamble_controlled<'input>(text:&'input str,envelope_id:&str,component:Component,version:u16,control:&mut semio_framework_value::NativeDecodeControl<'_>)->SemioResult<&'input str>{
    control.checkpoint().map_err(SemioError::DecodingControl)?;
    let bytes=text.as_bytes();let token_start=6usize;
    if !bytes.starts_with(b"semio "){return Err(SemioError::InvalidPreamble("missing canonical envelope prefix".into()));}
    let digits=if version>=10000{5}else if version>=1000{4}else if version>=100{3}else if version>=10{2}else{1};
    let end=token_start.checked_add(envelope_id.len()).and_then(|n|n.checked_add(component.as_str().len()+3+digits)).filter(|end|*end<=bytes.len()).ok_or_else(||SemioError::InvalidPreamble("truncated envelope token".into()))?;
    if !matches_declared_token(&bytes[token_start..end],envelope_id,component,version,control).map_err(SemioError::DecodingControl)?{return Err(SemioError::InvalidPreamble("declared envelope identity mismatch".into()));}
    let body=if end==bytes.len(){end}else if bytes[end]==b'\n'{end+1}else if bytes.get(end..end+2)==Some(b"\r\n"){end+2}else{return Err(SemioError::InvalidPreamble("canonical preamble requires a line boundary".into()));};
    Ok(&text[body..])
}

/// 🔍 Parses `semio plugin.artifact.component vN`.
pub fn parse_preamble_line(line: &str) -> SemioResult<SemioEnvelope> {
    let line = line.trim();
    let rest = line.strip_prefix("semio ").ok_or_else(|| SemioError::InvalidPreamble(format!("expected semio preamble, got {line}")))?;
    let (token, version_str) = rest.rsplit_once(" v").ok_or_else(|| SemioError::InvalidPreamble(format!("missing version in {line}")))?;
    let version: u16 = version_str.parse().map_err(|_| SemioError::InvalidPreamble(format!("bad version in {line}")))?;
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() < 3 {
        return Err(SemioError::InvalidPreamble(format!("expected plugin.artifact.component, got {token}")));
    }
    let component = Component::parse(parts[parts.len() - 1]).ok_or_else(|| SemioError::InvalidPreamble(format!("unknown component in {token}")))?;
    let artifact = parts[parts.len() - 2].to_string();
    let plugin = parts[..parts.len() - 2].join(".");
    Ok(SemioEnvelope { plugin, artifact, component, version })
}
//#endregion 🔖️Text

//#region 🔖️Sniff
/// 👃 Derives format identity from raw bytes alone.
pub fn sniff(bytes: &[u8]) -> SemioResult<SemioEnvelope> {
    if bytes.starts_with(&BINARY_MAGIC) {
        let (envelope, _) = unwrap_binary(bytes)?;
        return Ok(envelope);
    }
    let text = std::str::from_utf8(bytes).map_err(|_| SemioError::InvalidPreamble("binary file without semio magic".into()))?;
    let (envelope, _) = split_text_preamble(text)?;
    Ok(envelope)
}
//#endregion 🔖️Sniff

//#region 🔖️Paths
/// 📁 On-disk filename for a document facet: `<id>.<plugin>.<artifact>.<component>.semio`.
pub fn semio_filename(document_id: &str, envelope_id: &str, component: Component) -> String {
    format!("{document_id}.{envelope_id}.{}.semio", component.as_str())
}

/// 📖️ Infers envelope from a decorative filename (fallback only — content wins in `sniff`).
pub fn envelope_from_filename(name: &str) -> Option<SemioEnvelope> {
    let name = name.strip_suffix(".semio")?;
    let component = name.rsplit_once('.').and_then(|(_, c)| Component::parse(c))?;
    let rest = name.strip_suffix(&format!(".{}", component.as_str()))?;
    let (_doc, envelope_id) = rest.rsplit_once('.')?;
    let version = 1u16;
    SemioEnvelope::from_envelope_id(envelope_id, component, version).ok()
}
//#endregion 🔖️Paths

//#region 🔖️Registry
/// 🗂️ Handler keyed by full envelope identity.
pub type SemioHandler = fn(&[u8]) -> Result<(), String>;

struct RegistryState {
    by_key: HashMap<String, SemioHandler>,
}

fn registry_state() -> &'static Mutex<RegistryState> {
    static STATE: OnceLock<Mutex<RegistryState>> = OnceLock::new();
    STATE.get_or_init(|| Mutex::new(RegistryState { by_key: HashMap::new() }))
}

fn registry_key(envelope: &SemioEnvelope) -> String {
    format!("{}.{}.{}", envelope.plugin, envelope.artifact, envelope.component.as_str())
}

/// 📝 Registers a verify/parse handler for one envelope.
pub fn register_format(envelope: SemioEnvelope, handler: SemioHandler) {
    let key = registry_key(&envelope);
    registry_state().lock().expect("semio registry").by_key.insert(key, handler);
}

/// 🔎 Resolves a handler from sniffed content.
pub fn resolve(bytes: &[u8]) -> SemioResult<SemioHandler> {
    let envelope = sniff(bytes)?;
    let key = registry_key(&envelope);
    let state = registry_state().lock().expect("semio registry");
    state.by_key.get(&key).copied().ok_or(SemioError::UnknownEnvelope(key))
}

/// ✅ Runs the registered handler for these bytes.
pub fn verify(bytes: &[u8]) -> SemioResult<()> {
    let handler = resolve(bytes)?;
    handler(bytes).map_err(SemioError::InvalidPreamble)
}
//#endregion 🔖️Registry

//#region 🔖️Cli
/// ⌨️ `semio` CLI entry (`inspect`, `open`, `convert`, `verify`).
#[cfg(all(not(target_arch = "wasm32"), feature = "native-bin"))]
pub mod cli {
    use super::*;

    /// 🏃 Dispatches argv; returns process exit code.
    pub fn main_impl(args: &[String]) -> i32 {
        if args.is_empty() || args[0] == "help" || args[0] == "--help" {
            eprintln!("usage: semio <inspect|verify|open|convert> <path> [...]");
            return 0;
        }
        let cmd = args[0].as_str();
        let path = match args.get(1) {
            Some(p) => p,
            None => {
                eprintln!("[semio] missing path argument");
                return 2;
            }
        };
        let bytes = match std::fs::read(path) {
            Ok(b) => b,
            Err(err) => {
                eprintln!("[semio] read {path}: {err}");
                return 2;
            }
        };
        match cmd {
            "inspect" => match sniff(&bytes) {
                Ok(env) => {
                    println!("[TRACE] semio inspect {path}: {}", env.preamble_line());
                    println!("{}", env.preamble_line());
                    0
                }
                Err(err) => {
                    eprintln!("[semio] inspect failed: {err}");
                    1
                }
            },
            "verify" => match verify(&bytes) {
                Ok(()) => {
                    println!("[TRACE] semio verify {path}: ok");
                    0
                }
                Err(err) => {
                    eprintln!("[semio] verify failed: {err}");
                    1
                }
            },
            "open" | "convert" => {
                let env = match sniff(&bytes) {
                    Ok(e) => e,
                    Err(err) => {
                        eprintln!("[semio] {cmd} sniff failed: {err}");
                        return 1;
                    }
                };
                println!("[TRACE] semio {cmd} {path}: identity from content only -> {}", env.preamble_line());
                if let Ok(handler) = resolve(&bytes) {
                    if let Err(detail) = handler(&bytes) {
                        eprintln!("[semio] handler: {detail}");
                        return 1;
                    }
                }
                0
            }
            _ => {
                eprintln!("[semio] unknown command {cmd}");
                2
            }
        }
    }
}
//#endregion 🔖️Cli

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
