//! 🛬️ Borrowed Semio native primitives admit explicit owner fields before materialization.
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_framework_value::native_decoding::NativeDecodeControl;
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,SqliteDatabaseLimits};

/// 📥️ Borrowed native payload body after envelope unwrapping, in the encoding the caller supplied.
pub enum Body<'a>{Binary(&'a[u8]),Text(&'a str)}

/// 🧶️ Admits file size, SQL allocation ledger and progress, then lends the unwrapped body to the typed reader.
pub fn with_body<T>(payload:&store::io::IoPayload,id:&str,control:&mut SqliteSnapshotControl<'_>,native_control:&mut NativeDecodeControl<'_>,read:impl FnOnce(Body<'_>,&mut NativeDecodeControl<'_>,SqliteDatabaseLimits)->Result<T,ValueError>)->Result<T,ValueError>{
 let limits=control.limits();
 let size=match payload{store::io::IoPayload::Binary(v)=>v.len(),store::io::IoPayload::Text(v)=>v.len()};if size>limits.max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio native input exceeds file limit"))}
 control.allocation_stage_native(SqliteSnapshotPhase::DecodeNative,|remaining,checkpoint|{
  let native_before=native_control.owned_bytes();
  let Some(allowance)=native_before.checked_add(remaining) else{return (Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"native snapshot allowance overflow")),0)};
  let result=native_control.scoped_maximum(allowance,|native|{native.scoped_observer(&mut |event:semio_framework_value::native_decoding::NativeDecodeProgress|checkpoint(event.completed,event.total),|native|{
   (||->Result<T,ValueError>{match payload{
    store::io::IoPayload::Binary(value)=>{let body=store::semio_format::unwrap_binary_controlled(value,id,store::semio_format::Component::Pack,1,native)?;read(Body::Binary(body),native,limits)},
    store::io::IoPayload::Text(value)=>{let body=store::semio_format::split_text_preamble_controlled(value,id,store::semio_format::Component::Dsl,1,native).map_err(store::semio_format::SemioError::into_value_error)?;read(Body::Text(body),native,limits)}
   }})()
  })});
  (result,native_control.owned_bytes().saturating_sub(native_before))
 })?
}
pub fn length(reader:&mut store::ByteReader<'_>)->Result<usize,ValueError>{usize::try_from(reader.read_varint_u64().map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))?).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"Semio native length exceeds address space"))}
pub fn bytes<'a>(reader:&mut store::ByteReader<'a>)->Result<&'a[u8],ValueError>{let length=length(reader)?;reader.read_bytes(length).map_err(|e|ValueError::new(ValueRefusalKind::InvalidValue,e.to_string()))}
pub fn entities(total:&mut usize,count:usize,limits:SqliteDatabaseLimits)->Result<(),ValueError>{let next=total.checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Semio native entity count overflow"))?;if next>limits.max_rows{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio native entities exceed caller row limit"))}*total=next;Ok(())}
pub fn fields<'a,const N:usize>(body:&'a str,names:[&str;N],control:&mut NativeDecodeControl<'_>)->Result<[Option<&'a str>;N],ValueError>{
 control.scoped_stage(|control|{control.begin_stage(body.len())?;for _ in body.bytes(){control.step()?;}let mut fields=[None;N];for line in body.lines(){let line=line.trim();if line.is_empty(){continue}let(name,value)=line.split_once('=').ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native field requires equals"))?;let index=names.iter().position(|expected|*expected==name).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"unknown Semio native field"))?;if fields[index].replace(value).is_some(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate Semio native field"))}}Ok(fields)})
}
#[derive(Clone)]
pub struct Items<'a>{remaining:&'a str}
impl<'a> Items<'a>{
 pub fn new(value:&'a str)->Result<Self,ValueError>{let value=value.trim();Ok(Self{remaining:value.strip_prefix('[').and_then(|v|v.strip_suffix(']')).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native list requires brackets"))?})}
 pub fn next(&mut self,control:&mut NativeDecodeControl<'_>)->Result<Option<&'a str>,ValueError>{
 if self.remaining.is_empty(){return Ok(None)}let mut depth=0usize;let mut end=self.remaining.len();
 for(index,byte)in self.remaining.bytes().enumerate(){if index%256==0{control.checkpoint()?;}match byte{b'['=>{depth+=1;if depth>64{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"Semio native list depth exceeds limit"))}},b']'=>{depth=depth.checked_sub(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native unmatched bracket"))?},b',' if depth==0=>{end=index;break},_=>{}}}
 if depth!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native unmatched bracket"))}let item=self.remaining[..end].trim();self.remaining=if end<self.remaining.len(){let rest=&self.remaining[end+1..];if rest.trim().is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native trailing list separator"))}rest}else{""};if item.is_empty(){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native empty list item"))}Ok(Some(item))
 }
 pub fn count(&self,control:&mut NativeDecodeControl<'_>,maximum:usize)->Result<usize,ValueError>{let mut items=self.clone();let mut count=0usize;while items.next(control)?.is_some(){count=count.checked_add(1).filter(|count|*count<=maximum).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"Semio native list exceeds entity limit"))?;}Ok(count)}
}
pub fn record<'a,const N:usize>(value:&'a str,control:&mut NativeDecodeControl<'_>)->Result<[&'a str;N],ValueError>{
 let value=value.trim();let inner=value.strip_prefix('[').and_then(|v|v.strip_suffix(']')).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native record requires brackets"))?;let mut fields=["";N];if N==0{return if inner.is_empty(){Ok(fields)}else{Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native record has extra fields"))}}
 let mut depth=0usize;let mut field=0usize;let mut start=0;
 for(index,byte)in inner.bytes().enumerate(){if index%256==0{control.checkpoint()?;}match byte{b'['=>{depth+=1;if depth>64{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"Semio native record exceeds nesting limit"))}},b']'=>{depth=depth.checked_sub(1).ok_or_else(||ValueError::new(ValueRefusalKind::InvalidValue,"Semio native unmatched bracket"))?},b',' if depth==0=>{if field>=N-1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native record has extra fields"))}fields[field]=inner[start..index].trim();field+=1;start=index+1},_=>{}}}
 if depth!=0||field!=N-1{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native record field shape differs"))}fields[field]=inner[start..].trim();Ok(fields)
}

pub fn float(value:&str,control:&mut NativeDecodeControl<'_>)->Result<f64,ValueError>{
 control.scoped_stage(|control|{control.begin_stage(value.len())?;for _ in value.bytes(){control.step()?;}if let Some(word)=value.strip_prefix("nan64_"){if word.len()!=16||!word.bytes().all(|byte|byte.is_ascii_hexdigit()){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native NaN word"))}let bits=u64::from_str_radix(word,16).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native NaN word"))?;if bits&0x7ff0000000000000!=0x7ff0000000000000||bits&0xfffffffffffff==0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native NaN word has non-NaN class"))}return Ok(f64::from_bits(bits))}value.parse::<f64>().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native floating number"))})
}

pub struct Owned<T:semio_framework_value::retirement::RetireOwned>{value:Option<T>}
impl<T:semio_framework_value::retirement::RetireOwned> Owned<T>{
 pub fn new(value:T)->Self{Self{value:Some(value)}}
 pub fn get_mut(&mut self)->&mut T{self.value.as_mut().expect("live Semio native owner")}
 pub fn take(mut self)->T{self.value.take().expect("live Semio native owner")}
}
impl<T:semio_framework_value::retirement::RetireOwned> Drop for Owned<T>{
 fn drop(&mut self){if let Some(value)=self.value.take(){let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items:usize::MAX,maximum_copy_bytes:usize::MAX,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX};let (mut cursor,_)=semio_framework_value::retirement::admit_owned_retirement(value,grant).unwrap_or_else(|(error,_)|panic!("cold Semio native retirement refused: {error:?}"));while !cursor.terminal_is_empty(){cursor.close_step(grant).expect("valid Semio native retirement cursor");}}}
}

pub fn float32(value:&str,control:&mut NativeDecodeControl<'_>)->Result<f32,ValueError>{control.scoped_stage(|control|{control.begin_stage(value.len())?;for _ in value.bytes(){control.step()?;}if let Some(word)=value.strip_prefix("nan32_"){if word.len()!=8||!word.bytes().all(|byte|byte.is_ascii_hexdigit()){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native binary32 NaN word"))}let bits=u32::from_str_radix(word,16).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native binary32 NaN word"))?;if bits&0x7f800000!=0x7f800000||bits&0x7fffff==0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native binary32 word has non-NaN class"))}return Ok(f32::from_bits(bits))}value.parse::<f32>().map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"invalid Semio native binary32 number"))})}


/// 🧷️ Measures exact decoded UTF-8 from borrowed hexadecimal source with fixed scalar storage.
pub fn hex_text_extent(value:&str,control:&mut NativeDecodeControl<'_>)->Result<usize,ValueError>{
 if !value.len().is_multiple_of(2){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native hexadecimal text is invalid"))}
 control.scoped_stage(|control|{
  let mut scalar=[0u8;4];let mut used=0;let mut needed=0;control.begin_stage(value.len()/2)?;
  for pair in value.as_bytes().chunks_exact(2){
   let digit=|value|match value{b'0'..=b'9'=>Ok(value-b'0'),b'a'..=b'f'=>Ok(value-b'a'+10),b'A'..=b'F'=>Ok(value-b'A'+10),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native hexadecimal text is invalid"))};let byte=digit(pair[0])?*16+digit(pair[1])?;
   if used==0{needed=match byte{0..=0x7f=>1,0xc2..=0xdf=>2,0xe0..=0xef=>3,0xf0..=0xf4=>4,_=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native hexadecimal text is invalid"))};}else if byte&0xc0!=0x80{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native hexadecimal text is invalid"))}
   scalar[used]=byte;used+=1;if used==needed{std::str::from_utf8(&scalar[..used]).map_err(|_|ValueError::new(ValueRefusalKind::InvalidValue,"Semio native hexadecimal text is not UTF-8"))?;used=0;}control.step()?;
  }
  if used!=0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio native hexadecimal text is invalid"))}Ok(value.len()/2)
 })
}
pub fn text_into(output:&mut String,text:&str,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{control.copy_text_into(text,output)}

/// 🔗️ Creates a zero-heap artifact reference whose identity fields are filled in place after it is retained.
pub fn empty_reference()->semio_framework_artifact_reference::ArtifactRef{semio_framework_artifact_reference::ArtifactRef{artifact_id:String::new(),dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:String::new(),standard:String::new(),subset:String::new()}}}
/// 🪆️ Creates a zero-heap child handle whose identity fields are filled in place after it is retained.
pub fn empty_child<S>()->store::os_store::artifact_child::ArtifactChild<S>{store::os_store::artifact_child::ArtifactChild::new(String::new(),empty_reference())}
/// 🧵️ Copies one length-prefixed binary text field into its original empty destination.
pub fn reader_text_into(output:&mut String,reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{let text=control.borrow_text(bytes(reader)?)?;control.copy_text_into(text,output)}
/// 🪡️ Copies one length-prefixed binary octet field into its original empty destination.
pub fn reader_bytes_into(output:&mut Vec<u8>,reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{control.copy_bytes_into(bytes(reader)?,output)}
fn digit(value:u8)->Result<u8,ValueError>{match value{b'0'..=b'9'=>Ok(value-b'0'),b'a'..=b'f'=>Ok(value-b'a'+10),b'A'..=b'F'=>Ok(value-b'A'+10),_=>Err(ValueError::literal(ValueRefusalKind::InvalidValue,"invalid Semio native hex digit"))}}
fn not_utf8()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"Semio native text is not UTF-8")}
/// 🧬️ Decodes hexadecimal octets directly into their original empty destination, keeping every cancelled prefix.
pub fn hex_bytes_into(output:&mut Vec<u8>,value:&str,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
 if !value.len().is_multiple_of(2){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Semio native odd hex length"))}
 if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native hexadecimal octet slot must be empty"))}
 control.scoped_stage(|control|{*output=control.allocate_vec::<u8>(value.len()/2)?;control.begin_stage(value.len()/2)?;for pair in value.as_bytes().chunks_exact(2){output.push((digit(pair[0])?<<4)|digit(pair[1])?);control.step()?;}control.checkpoint()?;Ok::<_,ValueError>(())})
}
/// 🔤️ Decodes hexadecimal UTF-8 scalars directly into their original empty destination, keeping every cancelled prefix.
pub fn hex_text_into(output:&mut String,value:&str,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
 if !value.len().is_multiple_of(2){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Semio native odd hex length"))}
 if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native hexadecimal text slot must be empty"))}
 control.scoped_stage(|control|{
  let size=value.len()/2;control.charge(size)?;output.try_reserve_exact(size).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"native decoding text allocation failed"))?;control.begin_stage(size)?;
  let mut scalar=[0u8;4];let mut used=0usize;let mut needed=0usize;
  for pair in value.as_bytes().chunks_exact(2){
   let byte=(digit(pair[0])?<<4)|digit(pair[1])?;
   if used==0{needed=match byte{0..=0x7f=>1,0xc2..=0xdf=>2,0xe0..=0xef=>3,0xf0..=0xf4=>4,_=>return Err(not_utf8())};}else if byte&0xc0!=0x80{return Err(not_utf8())}
   scalar[used]=byte;used+=1;if used==needed{output.push_str(std::str::from_utf8(&scalar[..used]).map_err(|_|not_utf8())?);used=0;}control.step()?;
  }
  if used!=0{return Err(not_utf8())}control.checkpoint()?;Ok::<_,ValueError>(())
 })
}
/// 🔢️ Parses a hexadecimal-encoded numeric word on the stack so no ungranted heap text exists to cancel.
pub fn hex_float(value:&str,control:&mut NativeDecodeControl<'_>)->Result<f64,ValueError>{
 let size=hex_text_extent(value,control)?;let mut word=[0u8;384];if size>word.len(){return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"Semio native scalar exceeds stack word"))}
 for(slot,pair)in word.iter_mut().zip(value.as_bytes().chunks_exact(2)){*slot=(digit(pair[0])?<<4)|digit(pair[1])?;}
 float(std::str::from_utf8(&word[..size]).map_err(|_|not_utf8())?,control)
}
/// 🗂️ Reads a binary list into its original empty vector, inserting each zero-heap element before it is filled.
pub fn binary_list_into<T>(output:&mut Vec<T>,reader:&mut store::ByteReader<'_>,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize,empty:impl Fn()->T,mut read:impl FnMut(&mut T,&mut store::ByteReader<'_>,&mut NativeDecodeControl<'_>,&mut usize)->Result<(),ValueError>)->Result<(),ValueError>{
 let count=length(reader)?;self::entities(entities,count,limits)?;
 if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native list slot must be empty"))}
 *output=control.allocate_vec::<T>(count)?;
 control.scoped_stage(|control|{control.begin_stage(count)?;for _ in 0..count{output.push(empty());read(output.last_mut().unwrap(),reader,control,entities)?;control.step()?;}Ok::<_,ValueError>(())})
}
/// 🗒️ Reads a bracketed text list into its original empty vector, inserting each zero-heap element before it is filled.
pub fn text_list_into<T>(output:&mut Vec<T>,value:&str,control:&mut NativeDecodeControl<'_>,limits:SqliteDatabaseLimits,entities:&mut usize,empty:impl Fn()->T,mut read:impl FnMut(&mut T,&str,&mut NativeDecodeControl<'_>,&mut usize)->Result<(),ValueError>)->Result<(),ValueError>{
 let mut source=Items::new(value)?;let count=source.count(control,limits.max_rows)?;self::entities(entities,count,limits)?;
 if !output.is_empty()||output.capacity()!=0{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"native list slot must be empty"))}
 *output=control.allocate_vec::<T>(count)?;
 control.scoped_stage(|control|{control.begin_stage(count)?;while let Some(value)=source.next(control)?{output.push(empty());read(output.last_mut().unwrap(),value,control,entities)?;control.step()?;}Ok::<_,ValueError>(())})
}

#[path="🫙️workspace/🦀️.rs"]
pub mod workspace;
