//! 🔢️ Original numeric entity keys format into a fixed caller-owned decimal fragment.
use semio_framework_value::{ValueError,ValueRefusalKind};
pub fn unsigned_len(value:u32,index:usize)->Result<usize,ValueError>{if index!=0{return Err(invalid());}Ok(if value==0{1}else{value.ilog10()as usize+1})}
pub fn read_unsigned(mut value:u32,index:usize,offset:usize,output:&mut[u8])->Result<usize,ValueError>{if index!=0{return Err(invalid());}let mut digits=[0u8;10];let mut start=10;loop{start-=1;digits[start]=b'0'+(value%10)as u8;value/=10;if value==0{break;}}let source=digits[start..].get(offset..).ok_or_else(invalid)?;let count=source.len().min(output.len());output[..count].copy_from_slice(&source[..count]);Ok(count)}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"original Energy entity target segment or offset is absent")}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
