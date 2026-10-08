//! 🔤️ Strict RFC 4648 base64 codecs — §4 standard-alphabet padded and §5 URL-safe unpadded (base64url) — the sole runtime encoding any
//! s-plugin needs, replacing the third-party `base64` crate. See
//! <https://www.rfc-editor.org/rfc/rfc4648#section-4>. Relocated verbatim (algorithm unchanged)
//! from `🧰️framework/🔨️modules/📡️replication/⚙️codec/🦀️.rs`'s `🔤️Base64` region — a
//! product-neutral byte codec has no business living inside the replication wire contract, and
//! seven unrelated s-plugins needed it without pulling in replication's mutation/causal/conflict
//! vocabulary. `📡️replication` now re-exports `base64_standard_encode`/`base64_standard_decode`/
//! `Base64Error` from here so its own `crate::base64_standard_*` callers are untouched.

const BASE64_STANDARD_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
const BASE64_URL_ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

#[path = "🎮️cursor/🦀️.rs"]
mod cursor;
pub use cursor::{Base64DecodeCursor, Base64DecodeOutcome, Base64DecodeParts, Base64DecodeState, Base64Input, Base64InputRange};

#[cfg(test)]
#[path = "../../⏱️trace/🧮️memory/🧪️testing/📥️requests/🦀️.rs"]
mod test_allocation;
#[cfg(test)]
#[global_allocator]
static TEST_ALLOCATION_OBSERVER: test_allocation::RequestedAllocator = test_allocation::RequestedAllocator;

/// 📍️ Bounded base64 work uses bytes for Encode and ASCII characters for Validate or Decode.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum Base64Phase{Encode,Validate,Decode}
/// 📍️ Monotonic input units within one base64 phase.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct Base64Progress{pub phase:Base64Phase,pub completed:usize,pub total:usize}
/// 🎟️ Caller-owned output allowance and bounded cancellation callback.
pub struct Base64Control<'a>{pub maximum_output_bytes:usize,pub progress:&'a mut dyn FnMut(Base64Progress)->bool}
/// 🚨️ Exact format failures stay separate from caller resource refusal or cancellation.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum Base64ControlError{Codec(Base64Error),InputRange,OutputLimit,Cancelled}
impl std::fmt::Display for Base64ControlError{
    fn fmt(&self,out:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Codec(error)=>std::fmt::Display::fmt(error,out),Self::InputRange=>out.write_str("base64 input range is invalid"),Self::OutputLimit=>out.write_str("base64 output allocation or limit exceeded"),Self::Cancelled=>out.write_str("base64 cancelled")}}
}
impl std::error::Error for Base64ControlError{}
impl From<Base64Error> for Base64ControlError{fn from(error:Base64Error)->Self{Self::Codec(error)}}
impl Base64Control<'_>{
    fn admit(&self,length:usize)->Result<(),Base64ControlError>{if length>self.maximum_output_bytes{Err(Base64ControlError::OutputLimit)}else{Ok(())}}
    fn checkpoint(&mut self,phase:Base64Phase,completed:usize,total:usize)->Result<(),Base64ControlError>{if(self.progress)(Base64Progress{phase,completed,total}){Ok(())}else{Err(Base64ControlError::Cancelled)}}
}
/// 🔤️ Bounds output before allocation and cancellation before each at-most4095-byte encoding chunk.
pub fn base64_standard_encode_controlled(bytes:&[u8],control:&mut Base64Control<'_>)->Result<String,Base64ControlError>{
    let length=bytes.len().div_ceil(3).checked_mul(4).ok_or(Base64ControlError::OutputLimit)?;control.admit(length)?;control.checkpoint(Base64Phase::Encode,0,bytes.len())?;
    let mut output=String::new();output.try_reserve_exact(length).map_err(|_|Base64ControlError::OutputLimit)?;
    for(offset,chunk)in bytes.chunks(4095).enumerate(){append_encoded_bytes(chunk,BASE64_STANDARD_ALPHABET,true,&mut output);control.checkpoint(Base64Phase::Encode,(offset*4095+chunk.len()).min(bytes.len()),bytes.len())?;}Ok(output)
}
/// 🔤️ Validates strict padding before allocating the result; each pass checks at most4096 input bytes.
pub fn base64_standard_decode_controlled(encoded:&[u8],control:&mut Base64Control<'_>)->Result<Vec<u8>,Base64ControlError>{
    let mut cursor=Base64DecodeCursor::new(Base64Input::Borrowed(encoded));
    loop{if cursor.step(cursor.next_work_demand(),control)?==Base64DecodeState::Complete{return Ok(cursor.take_output().unwrap());}}
}

/// 🔤️ Strict RFC 4648 base64 decoding failure, shared by the standard and URL-safe codecs.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Base64Error {
    InvalidLength,
    InvalidByte { index: usize, byte: u8 },
    InvalidPadding,
    NonCanonicalTrailingBits,
}

impl std::fmt::Display for Base64Error {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLength => formatter.write_str("base64 length must be a multiple of four"),
            Self::InvalidByte { index, byte } => write!(formatter, "invalid base64 byte {byte:#04x} at index {index}"),
            Self::InvalidPadding => formatter.write_str("invalid base64 padding"),
            Self::NonCanonicalTrailingBits => formatter.write_str("non-canonical base64 trailing bits"),
        }
    }
}

impl std::error::Error for Base64Error {}

fn append_encoded_bytes(bytes: &[u8], alphabet: &[u8; 64], padded: bool, encoded: &mut String) {
    for chunk in bytes.chunks(3) {
        let first = chunk[0];
        let second = chunk.get(1).copied().unwrap_or(0);
        let third = chunk.get(2).copied().unwrap_or(0);
        encoded.push(alphabet[(first >> 2) as usize] as char);
        encoded.push(alphabet[(((first & 0x03) << 4) | (second >> 4)) as usize] as char);
        if chunk.len() >= 2 {
            encoded.push(alphabet[(((second & 0x0f) << 2) | (third >> 6)) as usize] as char);
        } else if padded {
            encoded.push('=');
        }
        if chunk.len() == 3 {
            encoded.push(alphabet[(third & 0x3f) as usize] as char);
        } else if padded {
            encoded.push('=');
        }
    }
}

fn encode_bytes(bytes: &[u8], alphabet: &[u8; 64], padded: bool) -> String {
    let mut encoded = String::with_capacity(bytes.len().div_ceil(3) * 4);
    append_encoded_bytes(bytes, alphabet, padded, &mut encoded);
    encoded
}

/// 🔤️ Encodes bytes with the padded RFC 4648 standard alphabet. Generic over `AsRef<[u8]>` (so a
/// bare `&str` works too) purely as an ergonomic front door onto [`encode_bytes`].
pub fn base64_standard_encode(bytes: impl AsRef<[u8]>) -> String {
    encode_bytes(bytes.as_ref(), BASE64_STANDARD_ALPHABET, true)
}

/// 🔗️ Encodes bytes with the unpadded RFC 4648 §5 URL-safe alphabet (base64url), the text form opaque
/// binary carriers use inside UI text leaves. See <https://www.rfc-editor.org/rfc/rfc4648#section-5>.
pub fn base64_url_encode(bytes: impl AsRef<[u8]>) -> String {
    encode_bytes(bytes.as_ref(), BASE64_URL_ALPHABET, false)
}

fn sextet(byte: u8, index: usize, url: bool) -> Result<u8, Base64Error> {
    match (byte, url) {
        (b'A'..=b'Z', _) => Ok(byte - b'A'),
        (b'a'..=b'z', _) => Ok(byte - b'a' + 26),
        (b'0'..=b'9', _) => Ok(byte - b'0' + 52),
        (b'+', false) | (b'-', true) => Ok(62),
        (b'/', false) | (b'_', true) => Ok(63),
        _ => Err(Base64Error::InvalidByte { index, byte }),
    }
}

/// 🧮️ Validates one standard-alphabet quartet and returns at most three stack-owned bytes.
pub fn decode_standard_quad(quad:[u8;4],index:usize,last:bool)->Result<([u8;3],usize),Base64Error>{
    if quad[0]==b'='||quad[1]==b'='{return Err(Base64Error::InvalidPadding);}
    let a=sextet(quad[0],index,false)?;let b=sextet(quad[1],index+1,false)?;let pad2=quad[2]==b'=';let pad1=quad[3]==b'=';
    if (pad1||pad2)&&!last||pad2&&!pad1{return Err(Base64Error::InvalidPadding);}
    let c=if pad2{0}else{sextet(quad[2],index+2,false)?};let d=if pad1{0}else{sextet(quad[3],index+3,false)?};
    if pad2&&b&15!=0||pad1&&!pad2&&c&3!=0{return Err(Base64Error::NonCanonicalTrailingBits);}
    Ok(([(a<<2)|(b>>4),(b<<4)|(c>>2),(c<<6)|d],if pad2{1}else if pad1{2}else{3}))
}

fn decode_bytes(encoded: &[u8]) -> Result<Vec<u8>, Base64Error> {
    if !encoded.len().is_multiple_of(4) {return Err(Base64Error::InvalidLength);}
    let mut decoded = Vec::with_capacity(encoded.len() / 4 * 3);
    for (group_index, quad) in encoded.as_chunks::<4>().0.iter().enumerate() {
        let offset=group_index*4;
        let (bytes,count)=decode_standard_quad(*quad,offset,offset+4==encoded.len())?;
        decoded.extend_from_slice(&bytes[..count]);
    }
    Ok(decoded)
}

/// 🔤️ Decodes padded RFC 4648 standard base64 and rejects whitespace, misplaced padding, and
/// non-canonical unused bits. Generic over `AsRef<[u8]>` for the same reason as
/// [`base64_standard_encode`] — one algorithm, [`decode_bytes`], behind an ergonomic front door.
pub fn base64_standard_decode(encoded: impl AsRef<[u8]>) -> Result<Vec<u8>, Base64Error> {
    decode_bytes(encoded.as_ref())
}

/// 🔗️ Decodes unpadded RFC 4648 §5 base64url and rejects padding, whitespace, the standard-only `+`/`/`,
/// a dangling single sextet, and non-canonical unused bits.
pub fn base64_url_decode(encoded: impl AsRef<[u8]>) -> Result<Vec<u8>, Base64Error> {
    let encoded = encoded.as_ref();
    if encoded.len() % 4 == 1 {
        return Err(Base64Error::InvalidLength);
    }
    let mut decoded = Vec::with_capacity(encoded.len() * 3 / 4);
    for (group_index, chunk) in encoded.chunks(4).enumerate() {
        let offset = group_index * 4;
        let sextets = chunk.iter().enumerate().map(|(index, byte)| sextet(*byte, offset + index, true)).collect::<Result<Vec<u8>, Base64Error>>()?;
        decoded.push((sextets[0] << 2) | (sextets[1] >> 4));
        match sextets.len() {
            2 if sextets[1] & 0x0f != 0 => return Err(Base64Error::NonCanonicalTrailingBits),
            3 if sextets[2] & 0x03 != 0 => return Err(Base64Error::NonCanonicalTrailingBits),
            _ => {}
        }
        if sextets.len() >= 3 {
            decoded.push((sextets[1] << 4) | (sextets[2] >> 2));
        }
        if sextets.len() == 4 {
            decoded.push((sextets[2] << 6) | sextets[3]);
        }
    }
    Ok(decoded)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
