//! 🧾️ Cooperative MIME-qualified encoded bytes with private publication.
use semio_framework_io_base64::decode_standard_quad;
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
#[derive(Clone,Copy,Debug)]
pub struct BinarySourceInput<'a>{pub mime:&'a str,pub data:&'a str,pub min_bytes:usize,pub max_source_bytes:usize,pub max_bytes:usize,pub max_work:u64}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct BinarySourceProgress{pub phase:&'static str,pub source_completed:usize,pub source_total:usize,pub bytes:usize,pub total_bytes:usize,pub work:u64,pub done:bool}
#[derive(Clone,Debug,PartialEq,Eq,semio_framework_value::RetireOwned)]
pub enum BinarySourceError{Invalid(&'static str),Incomplete,Cancelled}
impl std::fmt::Display for BinarySourceError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Invalid(v)=>f.write_str(v),Self::Incomplete=>f.write_str("Binary source is incomplete"),Self::Cancelled=>f.write_str("Binary source cancelled")}}}
impl std::error::Error for BinarySourceError{}
fn invalid(v:&'static str)->BinarySourceError{BinarySourceError::Invalid(v)}
fn hex(v:u8)->Option<u8>{match v{b'0'..=b'9'=>Some(v-b'0'),b'A'..=b'F'=>Some(v-b'A'+10),b'a'..=b'f'=>Some(v-b'a'+10),_=>None}}
fn safe(v:u8)->bool{v.is_ascii_alphanumeric()||b";/?:@&=+$,-_.!~*'()".contains(&v)}
fn mime(v:&str)->bool{v.split_once('/').is_some_and(|(a,b)|!a.is_empty()&&!b.is_empty()&&a.bytes().chain(b.bytes()).all(|c|c.is_ascii_alphanumeric()||b"!#$&^_.+~-".contains(&c)))}
/// 🧩️ Complete validation and byte/work admission precede output allocation.
#[derive(semio_framework_value::RetireOwned)]
pub struct BinarySourceJob{
 source_identity:usize,source_length:usize,mime:[u8;128],mime_length:usize,min_bytes:usize,max_bytes:usize,max_work:u64,phase:&'static str,at:usize,body_start:usize,read:usize,source_total:usize,base64:bool,url:bool,
 work:u64,bytes:usize,total_bytes:usize,quad:[u8;4],quad_at:usize,binary:Vec<u8>,output:Vec<u8>,cancelled:bool,failed:Option<BinarySourceError>,
}
semio_framework_value::artifact_retire_leaf!(BinarySourceProgress);
impl BinarySourceJob{
 pub fn new(input:BinarySourceInput<'_>)->Result<Self,BinarySourceError>{
  if !(1..=268439552).contains(&input.max_source_bytes)||input.data.len()>input.max_source_bytes||input.mime.len()>128||!mime(&input.mime)||!(1..=67108864).contains(&input.max_bytes)||input.min_bytes>input.max_bytes||!(1..=1000000000).contains(&input.max_work){return Err(invalid("Invalid binary source contract"));}
  let source_length=input.data.len();let url=input.data.as_bytes().get(..5).is_some_and(|v|v.eq_ignore_ascii_case(b"data:"));let at=if url{5}else{0};
  let mut mime=[0;128];mime[..input.mime.len()].copy_from_slice(input.mime.as_bytes());
  Ok(Self{source_identity:input.data.as_ptr()as usize,source_length,mime,mime_length:input.mime.len(),min_bytes:input.min_bytes,max_bytes:input.max_bytes,max_work:input.max_work,phase:if url{"header"}else{"validate"},at,body_start:0,read:at,source_total:source_length*2,base64:true,url,work:0,bytes:0,total_bytes:0,quad:[0;4],quad_at:0,binary:Vec::new(),output:Vec::new(),cancelled:false,failed:None})
 }
 fn check(&self)->Result<(),BinarySourceError>{if self.cancelled{return Err(BinarySourceError::Cancelled);}if self.phase=="transferred"{return Err(BinarySourceError::Incomplete);}if let Some(e)=&self.failed{return Err(e.clone());}Ok(())}
 fn header_step(&mut self,text:&str)->Result<(),BinarySourceError>{
  if self.at>=self.source_length||self.at>=4096{return Err(invalid("Missing or excessive data URL header"));}
  let value=text.as_bytes()[self.at];self.at+=1;self.read+=1;
  if value!=b','{if !(33..=126).contains(&value)||value==b'#'{return Err(invalid("Invalid data URL header"));}return Ok(());}
  let mut parts=text[5..self.at-1].split(';').peekable();
  if !parts.next().unwrap().as_bytes().eq_ignore_ascii_case(&self.mime[..self.mime_length]){return Err(invalid("Data URL media type does not match its declared MIME"));}
  self.base64=false;
  while let Some(part)=parts.next(){
   if part.eq_ignore_ascii_case("base64"){if self.base64||parts.peek().is_some(){return Err(invalid("Invalid base64 flag placement"));}self.base64=true;continue;}
   let Some((key,value))=part.split_once('=')else{return Err(invalid("Invalid data URL parameter"));};
   if key.is_empty()||value.is_empty()||!key.bytes().all(|v|v.is_ascii_alphanumeric()||v==96||b"!$&'*+.^_|~-".contains(&v)){return Err(invalid("Invalid data URL parameter"));}
   let bytes=value.as_bytes();let mut at=0;
   while at<bytes.len(){if bytes[at]==b'%'{if at+2>=bytes.len()||hex(bytes[at+1]).is_none()||hex(bytes[at+2]).is_none(){return Err(invalid("Invalid parameter escape"));}at+=3;}else{if !safe(bytes[at]){return Err(invalid("Invalid parameter value"));}at+=1;}}
  }
  self.body_start=self.at;self.source_total=self.source_length*2-self.body_start;self.phase="validate";Ok(())
 }
 fn token(&mut self,text:&str)->Result<u8,BinarySourceError>{
  let source=text.as_bytes();let mut value=source[self.at];self.at+=1;self.read+=1;
  if self.url&&value==b'%'{
   if self.at+2>self.source_length{return Err(invalid("Truncated source escape"));}
   let (Some(a),Some(b))=(hex(source[self.at]),hex(source[self.at+1]))else{return Err(invalid("Invalid source escape"));};self.at+=2;self.read+=2;value=a*16+b;
  }else if value>127||!self.base64&&!safe(value){return Err(invalid("Invalid source byte"));}
  Ok(value)
 }
 fn source_step(&mut self,text:&str)->Result<(),BinarySourceError>{
  let validating=self.phase=="validate";
  if self.at==self.source_length{
   if self.quad_at!=0{return Err(invalid("Invalid base64 length"));}
   if validating{if self.total_bytes<self.min_bytes{return Err(invalid("Encoded source is too short"));}self.binary.try_reserve_exact(self.total_bytes).map_err(|_|invalid("Encoded source allocation refused"))?;self.at=self.body_start;self.phase="decode";}
   else{self.output=std::mem::take(&mut self.binary);self.phase="complete";}
   return Ok(());
  }
  let value=self.token(text)?;
  if self.base64{
   if value>127{return Err(invalid("Invalid base64 byte"));}
   self.quad[self.quad_at]=value;self.quad_at+=1;
   if self.quad_at==4{let (bytes,count)=decode_standard_quad(self.quad,self.at-4,self.at==self.source_length).map_err(|_|invalid("Invalid base64 quartet"))?;self.quad_at=0;if validating{self.total_bytes+=count;}else{self.binary.extend_from_slice(&bytes[..count]);self.bytes+=count;}}
  }else if validating{self.total_bytes+=1;}else{self.binary.push(value);self.bytes+=1;}
  if self.total_bytes>self.max_bytes{return Err(invalid("Encoded source byte limit exceeded"));}Ok(())
 }
 pub fn progress(&self)->BinarySourceProgress{BinarySourceProgress{phase:self.phase,source_completed:self.read,source_total:self.source_total,bytes:self.bytes,total_bytes:self.total_bytes,work:self.work,done:self.phase=="complete"}}
 pub fn next_copy_byte_demand(&self)->usize{if matches!(self.phase,"complete"|"transferred"){0}else{32}}
 pub fn next_capacity_byte_demand(&self)->usize{if self.phase=="validate"&&self.at==self.source_length&&self.quad_at==0{self.total_bytes}else{0}}
 pub fn advance(&mut self,source:&str,grant:RetainedCloneGrant)->Result<(BinarySourceProgress,RetainedCloneProgress),BinarySourceError>{
  self.check()?;if source.len()!=self.source_length||source.as_ptr()as usize!=self.source_identity{return Err(invalid("Original binary source changed"));}
  let mut receipt=RetainedCloneProgress::default();
  for _ in 0..grant.maximum_items{if self.phase=="complete"{break;}let copy=self.next_copy_byte_demand();let capacity=self.next_capacity_byte_demand();if grant.maximum_depth<1||copy>grant.maximum_copy_bytes.saturating_sub(receipt.copied_bytes)||capacity>grant.maximum_capacity_bytes.saturating_sub(receipt.retained_capacity_bytes){break;}
   let result=if self.work>=self.max_work{Err(invalid("Binary source work limit exceeded"))}else if self.phase=="header"{self.header_step(source)}else{self.source_step(source)};if let Err(e)=result{self.failed=Some(e.clone());return Err(e);}self.work+=1;receipt.copied_items+=1;receipt.copied_bytes+=copy;receipt.retained_capacity_bytes+=capacity;
  }
  Ok((self.progress(),receipt))
 }
 pub fn cancel(&mut self){self.cancelled=true;}
 pub fn result(&self)->Result<&[u8],BinarySourceError>{self.check()?;if self.phase!="complete"{return Err(BinarySourceError::Incomplete);}Ok(&self.output)}
 pub fn take_result(&mut self,grant:RetainedCloneGrant)->Result<Option<(Vec<u8>,RetainedCloneProgress)>,BinarySourceError>{self.check()?;if self.phase!="complete"{return Err(BinarySourceError::Incomplete);}if grant.maximum_items==0||grant.maximum_copy_bytes<std::mem::size_of::<Vec<u8>>()||grant.maximum_depth==0{return Ok(None);}self.phase="transferred";Ok(Some((std::mem::take(&mut self.output),RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Vec<u8>>(),..Default::default()})))}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
