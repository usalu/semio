//! 👓️ Borrowed JSON octets feed the existing retained grammar without source materialization.
use super::JsonError;

/// 🪪️ Binds the retained grammar to one exact immutable source borrow until result or retirement handoff.
pub type JsonBorrowedParseCursor<'source,S> = JsonBorrowedCursor<'source,S,super::Value>;
pub type JsonBorrowedDslCursor<'source,S> = JsonBorrowedCursor<'source,S,semio_framework_value::DslValue>;
pub type JsonBorrowedCursor<'source,S,V>=JsonSourceCursor<&'source S,V>;
/// 🪟️ Retains an immutable first-party source view by value alongside its original grammar owner.
pub struct JsonSourceCursor<S:JsonReadSource+Copy,V:JsonParsedValue>{source:S,parser:super::JsonGrammarCursor<V>}
impl<S:JsonReadSource+Copy,V:JsonParsedValue> JsonSourceCursor<S,V>{
    /// 🛂️ Captures the complete original caller limits before any source read or destination allocation.
    pub fn new(source:S,policy:super::JsonMemberPolicy,limits:JsonReadLimits)->Result<Self,semio_framework_value::ValueError>{
        if source.byte_len()as u128>u128::from(limits.maximum_bytes){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::WorkLimit,"JSON complete source exceeds caller byte limit"));}
        let mut parser=super::JsonGrammarCursor::new(policy);parser.limits=limits;Ok(Self{source,parser})
    }
    pub fn source_ref(&self)->&S{&self.source}
    /// 🪪️ Transfers the original immutable source identity with its native grammar owner.
    pub fn into_grammar(self)->(S,super::JsonGrammarCursor<V>){(self.source,self.parser)}
    /// 🪆️ Restores the same source and grammar after a refused ownership handoff.
    pub fn from_grammar(source:S,parser:super::JsonGrammarCursor<V>)->Self{Self{source,parser}}
    pub fn position(&self)->usize{self.parser.position()}
    pub fn phase(&self)->&'static str{self.parser.phase()}
    pub fn normal_step_progress(&self)->super::RetainedCloneProgress{self.parser.normal_step_progress()}
    pub fn normal_step_demands(&self)->Result<super::RetirementDemand,JsonError>{self.parser.normal_step_demands(&self.source)}
    pub fn step(&mut self,maximum_units:usize,control:&mut semio_framework_value::NativeDecodeControl<'_>,grant:super::RetainedCloneGrant)->Result<Option<V>,JsonError>{self.parser.step_source(&self.source,maximum_units,control,grant,false)}
    /// 🎟️ Admits the native grammar retirement frame before transferring its original owner.
    pub fn retirement_birth_bytes(&self)->usize{semio_framework_value::owned_retirement_birth_bytes::<super::JsonGrammarCursor<V>>()}
    pub fn into_retirement(self,grant:semio_framework_value::retained_clone::RetainedCloneGrant)->Result<(Box<dyn semio_framework_value::ErasedSnapshotRetirement>,semio_framework_value::retained_clone::RetainedCloneProgress),(semio_framework_value::ValueError,Self)>{
        let Self{source,parser}=self;
        semio_framework_value::admit_owned_retirement(parser,grant).map_err(|(error,parser)|(error,Self{source,parser}))
    }
}

impl<'source,S:JsonReadSource+?Sized,V:JsonParsedValue> JsonSourceCursor<&'source S,V>{pub fn source(&self)->&'source S{self.source}}

/// 📏️ One immutable finite source supplies original octets for every retained parse step.
pub trait JsonReadSource {
    fn byte_len(&self)->usize;
    fn byte_at(&self,index:usize)->Option<u8>;
}
impl<S:JsonReadSource+?Sized> JsonReadSource for &S{fn byte_len(&self)->usize{(**self).byte_len()}fn byte_at(&self,index:usize)->Option<u8>{(**self).byte_at(index)}}
impl JsonReadSource for str {
    fn byte_len(&self)->usize{self.len()}
    fn byte_at(&self,index:usize)->Option<u8>{self.as_bytes().get(index).copied()}
}
impl JsonReadSource for [u8] {
    fn byte_len(&self)->usize{self.len()}
    fn byte_at(&self,index:usize)->Option<u8>{self.get(index).copied()}
}
impl<const N:usize> JsonReadSource for semio_framework_value::list::PagedList<u8,N> {
    fn byte_len(&self)->usize{self.len()}
    fn byte_at(&self,index:usize)->Option<u8>{self.get(index).copied()}
}
pub(super) fn starts_with<S:JsonReadSource+?Sized>(source:&S,position:usize,text:&str)->bool{
    text.bytes().enumerate().all(|(index,byte)|position.checked_add(index).and_then(|offset|source.byte_at(offset))==Some(byte))
}
pub(super) fn character<S:JsonReadSource+?Sized>(source:&S,position:usize)->Result<char,JsonError>{
    let first=source.byte_at(position).ok_or(JsonError::UnexpectedEof)?;
    let length=match first{0..=0x7f=>1,0xc2..=0xdf=>2,0xe0..=0xef=>3,0xf0..=0xf4=>4,_=>return Err(JsonError::InvalidUtf8)};
    let mut bytes=[0;4];
    for(index,byte)in bytes[..length].iter_mut().enumerate(){*byte=position.checked_add(index).and_then(|offset|source.byte_at(offset)).ok_or(JsonError::InvalidUtf8)?;}
    std::str::from_utf8(&bytes[..length]).map_err(|_|JsonError::InvalidUtf8)?.chars().next().ok_or(JsonError::InvalidUtf8)
}
fn hex4<S:JsonReadSource+?Sized>(source:&S,position:&mut usize,start:usize)->Result<u32,JsonError>{
    if position.checked_add(4).is_none_or(|end|end>source.byte_len()){return Err(JsonError::InvalidUnicodeEscape(start));}
    let mut value=0;
    for index in 0..4{
        let byte=source.byte_at(*position+index).ok_or(JsonError::InvalidUnicodeEscape(start))?;
        let digit=match byte{b'0'..=b'9'=>u32::from(byte-b'0'),b'a'..=b'f'=>u32::from(byte-b'a')+10,b'A'..=b'F'=>u32::from(byte-b'A')+10,_=>return Err(JsonError::InvalidUnicodeEscape(start))};
        value=value*16+digit;
    }
    *position+=4;
    Ok(value)
}
pub(super) fn json_character<S:JsonReadSource+?Sized>(source:&S,position:&mut usize)->Result<Option<char>,JsonError>{
    let next=character(source,*position)?;
    match next{
        '"'=>{*position+=1;Ok(None)},
        '\\'=>{
            let start=*position;*position+=1;let escaped=character(source,*position)?;*position+=escaped.len_utf8();
            let next=match escaped{
                '"'=>'"','\\'=>'\\','/'=>'/','b'=>'\u{0008}','f'=>'\u{000c}','n'=>'\n','r'=>'\r','t'=>'\t',
                'u'=>{
                    let unit=hex4(source,position,start)?;
                    if(0xd800..=0xdbff).contains(&unit){
                        if source.byte_at(*position)!=Some(b'\\'){return Err(JsonError::UnpairedSurrogate(start));}*position+=1;
                        if source.byte_at(*position)!=Some(b'u'){return Err(JsonError::UnpairedSurrogate(start));}*position+=1;
                        let low=hex4(source,position,start)?;if !(0xdc00..=0xdfff).contains(&low){return Err(JsonError::UnpairedSurrogate(start));}
                        char::from_u32(0x10000+((unit-0xd800)<<10)+low-0xdc00).ok_or(JsonError::UnpairedSurrogate(start))?
                    }else if(0xdc00..=0xdfff).contains(&unit){return Err(JsonError::UnpairedSurrogate(start));}
                    else{char::from_u32(unit).ok_or(JsonError::InvalidUnicodeEscape(start))?}
                },
                _=>return Err(JsonError::InvalidEscape(start)),
            };
            Ok(Some(next))
        },
        next if(next as u32)<0x20=>Err(JsonError::ControlCharacterInString{byte:next as u8,offset:*position}),
        next=>{*position+=next.len_utf8();Ok(Some(next))},
    }
}
pub(super) struct NumberText {bytes:[u8;1200],length:usize}
impl NumberText{
    pub(super) fn new()->Self{Self{bytes:[0;1200],length:0}}
    pub(super) fn push(&mut self,byte:u8)->Result<(),JsonError>{
        let cell=self.bytes.get_mut(self.length).ok_or(JsonError::InvalidNumber(0))?;*cell=byte;self.length+=1;Ok(())
    }
    pub(super) fn text(&self)->Result<&str,JsonError>{std::str::from_utf8(&self.bytes[..self.length]).map_err(|_|JsonError::InvalidNumber(0))}
}
impl std::fmt::Write for NumberText{
    fn write_str(&mut self,text:&str)->std::fmt::Result{for byte in text.bytes(){self.push(byte).map_err(|_|std::fmt::Error)?;}Ok(())}
}

mod parsed_value_authority{
    pub trait Sealed{}
    impl Sealed for super::super::Value{}
    impl Sealed for semio_framework_value::DslValue{}
}
/// 🧬️ Closed first-party semantic destinations move each original admitted JSON cell exactly once.
pub trait JsonParsedValue:parsed_value_authority::Sealed+semio_framework_value::retirement::RetireOwned{
    fn json_null()->Self;
    fn json_bool(value:bool)->Self;
    fn json_number(value:super::Number)->Self;
    fn json_string(value:String)->Self;
    fn json_array(values:Vec<Self>)->Self where Self:Sized;
    fn json_object(values:Vec<(String,Self)>)->Self where Self:Sized;
}
impl JsonParsedValue for super::Value{
    fn json_null()->Self{Self::Null}
    fn json_bool(value:bool)->Self{Self::Bool(value)}
    fn json_number(value:super::Number)->Self{Self::Number(value)}
    fn json_string(value:String)->Self{Self::String(value)}
    fn json_array(values:Vec<Self>)->Self{Self::Array(values)}
    fn json_object(values:Vec<(String,Self)>)->Self{Self::Object(super::Object(values))}
}
impl JsonParsedValue for semio_framework_value::DslValue{
    fn json_null()->Self{Self::Null}
    fn json_bool(value:bool)->Self{Self::Bool(value)}
    fn json_number(value:super::Number)->Self{Self::Number(match value{super::Number::UInt(value)=>semio_framework_value::Number::UInt(value),super::Number::Int(value)=>semio_framework_value::Number::Int(value),super::Number::Float(value)=>semio_framework_value::Number::Float(value)})}
    fn json_string(value:String)->Self{Self::String(value)}
    fn json_array(values:Vec<Self>)->Self{Self::Array(values)}
    fn json_object(values:Vec<(String,Self)>)->Self{Self::Object(values)}
}

/// 🛂️ Original caller policy bounds the complete source, cumulative storage and each declared collection extent.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct JsonReadLimits{
    pub maximum_bytes:u64,
    pub maximum_allocation_bytes:usize,
    pub maximum_depth:usize,
    pub maximum_items:u64,
}
