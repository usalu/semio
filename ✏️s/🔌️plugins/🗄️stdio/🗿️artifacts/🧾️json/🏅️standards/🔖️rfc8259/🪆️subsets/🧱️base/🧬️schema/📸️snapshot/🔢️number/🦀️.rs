//! 🔢️ Borrowed number meaning never normalizes the owned original lexeme.
use semio_framework_os_kernel::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase};
pub(crate) struct NumberMeaning{pub(crate) valid:bool,pub(crate) numeric:Option<f64>}
fn digits(bytes:&[u8],cursor:&mut usize,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase,completed:usize,total:usize)->Result<bool, ValueError>{let start=*cursor;while bytes.get(*cursor).is_some_and(u8::is_ascii_digit){*cursor+=1;if *cursor%65536==0{control.checkpoint(phase,completed,total)?;}}Ok(*cursor>start)}
pub(crate) fn meaning(lexeme:&str,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase,completed:usize,total:usize)->Result<NumberMeaning, ValueError>{
 let bytes=lexeme.as_bytes();let mut cursor=0;if bytes.len()>65536{control.checkpoint(phase,completed,total)?;}
 if bytes.get(cursor)==Some(&b'-'){cursor+=1;}
 if bytes.get(cursor)==Some(&b'0'){cursor+=1;}else if bytes.get(cursor).is_some_and(|byte|(b'1'..=b'9').contains(byte)){if !digits(bytes,&mut cursor,control,phase,completed,total)?{return Ok(NumberMeaning{valid:false,numeric:None})}}else{return Ok(NumberMeaning{valid:false,numeric:None})}
 if bytes.get(cursor)==Some(&b'.'){cursor+=1;if !digits(bytes,&mut cursor,control,phase,completed,total)?{return Ok(NumberMeaning{valid:false,numeric:None})}}
 if bytes.get(cursor).is_some_and(|byte|*byte==b'e'||*byte==b'E'){cursor+=1;if bytes.get(cursor).is_some_and(|byte|*byte==b'+'||*byte==b'-'){cursor+=1;}if !digits(bytes,&mut cursor,control,phase,completed,total)?{return Ok(NumberMeaning{valid:false,numeric:None})}}
 if cursor!=bytes.len(){return Ok(NumberMeaning{valid:false,numeric:None})}Ok(NumberMeaning{valid:true,numeric:lexeme.parse::<f64>().ok().filter(|value|value.is_finite()).map(|value|if value==0.0{0.0}else{value})})
}


use semio_framework_os_kernel::sqlite_snapshot::ValueError;
