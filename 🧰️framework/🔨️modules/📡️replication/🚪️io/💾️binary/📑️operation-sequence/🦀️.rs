//! 📑️ Controlled count-and-length operation sequence framing over original octet slices.
use crate::value::{NativeEncodeControl,ValueError,ValueRefusalKind};
fn word(value:usize,buffer:&mut[u8;10])->usize{let mut value=value as u64;let mut length=0;loop{buffer[length]=(value&127)as u8;value>>=7;length+=1;if value==0{return length;}buffer[length-1]|=128;}}
/// ✍️ Admits exact output before framing the original operation octets under caller cancellation.
pub fn encode(operations:&[Vec<u8>],control:&mut NativeEncodeControl<'_>)->Result<Vec<u8>,ValueError>{control.scoped_stage(|control|{
    control.begin_stage(operations.len())?;let mut buffer=[0;10];let mut length=word(operations.len(),&mut buffer);
    for operation in operations{control.step()?;length=length.checked_add(word(operation.len(),&mut buffer)).and_then(|length|length.checked_add(operation.len())).ok_or_else(||ValueError::literal(ValueRefusalKind::WorkLimit,"operation sequence size overflow"))?;}
    let mut output=control.allocate_vec(length)?;control.begin_stage(length)?;let count=word(operations.len(),&mut buffer);control.advance(count)?;output.extend_from_slice(&buffer[..count]);
    for operation in operations{let count=word(operation.len(),&mut buffer);control.advance(count)?;output.extend_from_slice(&buffer[..count]);for chunk in operation.chunks(256){control.advance(chunk.len())?;output.extend_from_slice(chunk);}}
    Ok(output)
})}
