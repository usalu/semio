//! 🧬️ Intrinsic octets have one owned value node and a canonical padded base64 literal.
use super::{DslValue,FromValue,ValueError};
const ALPHABET:&[u8;64]=b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
/// 🧬️ Explicit field mapping for an intrinsic octet sequence.
pub fn to_value(bytes:&[u8])->DslValue{DslValue::Bytes(bytes.to_vec())}
/// 🧬️ Accepts owned native octets or their canonical JSON array projection.
pub fn from_value(value:DslValue)->Result<Vec<u8>,ValueError>{match value{DslValue::Bytes(bytes)=>Ok(bytes),value@DslValue::Array(_)=>Vec::<u8>::from_value(value),_=>Err(ValueError::new("expected intrinsic octets or their JSON array projection"))}}
/// 🔤️ Encodes an intrinsic octet sequence using canonical RFC4648 padding.
pub fn encode_base64(bytes:&[u8])->String{
    let mut out=String::with_capacity(bytes.len().div_ceil(3).checked_mul(4).expect("base64 encoded length overflow"));
    for chunk in bytes.chunks(3){let a=chunk[0];let b=chunk.get(1).copied().unwrap_or(0);let c=chunk.get(2).copied().unwrap_or(0);out.push(ALPHABET[usize::from(a>>2)]as char);out.push(ALPHABET[usize::from(((a&3)<<4)|(b>>4))]as char);out.push(if chunk.len()>1{ALPHABET[usize::from(((b&15)<<2)|(c>>6))]as char}else{'='});out.push(if chunk.len()>2{ALPHABET[usize::from(c&63)]as char}else{'='});}out
}
/// 🔤️ Rejects misplaced padding and nonzero unused bits before returning owned octets.
pub fn decode_base64(text:&str)->Result<Vec<u8>,String>{
    if text.len()%4!=0{return Err("base64 length must be divisible by four".into());}
    let mut out=Vec::with_capacity(text.len()/4*3);
    let decode=|byte:u8|match byte{b'A'..=b'Z'=>Ok(byte-b'A'),b'a'..=b'z'=>Ok(byte-b'a'+26),b'0'..=b'9'=>Ok(byte-b'0'+52),b'+'=>Ok(62),b'/'=>Ok(63),_=>Err("base64 contains an invalid alphabet byte".to_string())};
    for(index,chunk)in text.as_bytes().chunks_exact(4).enumerate(){let last=index+1==text.len()/4;let a=decode(chunk[0])?;let b=decode(chunk[1])?;let pad2=chunk[2]==b'=';let pad1=chunk[3]==b'=';
        if(pad1||pad2)&&!last||pad2&&!pad1{return Err("base64 padding must terminate the final quartet".into());}
        let c=if pad2{0}else{decode(chunk[2])?};let d=if pad1{0}else{decode(chunk[3])?};if pad2&&b&15!=0||pad1&&!pad2&&c&3!=0{return Err("base64 unused bits must be zero".into());}
        out.push((a<<2)|(b>>4));if !pad2{out.push((b<<4)|(c>>2));}if !pad1{out.push((c<<6)|d);}
    }Ok(out)
}
