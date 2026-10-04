//! 🧾️ Cooperative MIME-qualified encoded bytes with private publication.
use semio_framework_io_base64::decode_standard_quad;
use std::sync::Arc;
#[derive(Clone,Debug)]
pub struct BinarySourceInput{pub mime:String,pub data:Arc<String>,pub min_bytes:usize,pub max_source_bytes:usize,pub max_bytes:usize,pub max_work:u64}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct BinarySourceProgress{pub phase:&'static str,pub source_completed:usize,pub source_total:usize,pub bytes:usize,pub total_bytes:usize,pub work:u64,pub done:bool}
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum BinarySourceError{Invalid(String),Incomplete,Cancelled}
impl std::fmt::Display for BinarySourceError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Invalid(v)=>f.write_str(v),Self::Incomplete=>f.write_str("Binary source is incomplete"),Self::Cancelled=>f.write_str("Binary source cancelled")}}}
impl std::error::Error for BinarySourceError{}
fn invalid(v:impl ToString)->BinarySourceError{BinarySourceError::Invalid(v.to_string())}
fn hex(v:u8)->Option<u8>{match v{b'0'..=b'9'=>Some(v-b'0'),b'A'..=b'F'=>Some(v-b'A'+10),b'a'..=b'f'=>Some(v-b'a'+10),_=>None}}
fn safe(v:u8)->bool{v.is_ascii_alphanumeric()||b";/?:@&=+$,-_.!~*'()".contains(&v)}
fn mime(v:&str)->bool{v.split_once('/').is_some_and(|(a,b)|!a.is_empty()&&!b.is_empty()&&a.bytes().chain(b.bytes()).all(|c|c.is_ascii_alphanumeric()||b"!#$&^_.+~-".contains(&c)))}
/// 🧩️ Complete validation and byte/work admission precede output allocation.
pub struct BinarySourceJob{
 source:Option<Arc<String>>,source_length:usize,mime:String,min_bytes:usize,max_bytes:usize,max_work:u64,phase:&'static str,at:usize,body_start:usize,read:usize,source_total:usize,base64:bool,url:bool,
 work:u64,bytes:usize,total_bytes:usize,quad:[u8;4],quad_at:usize,binary:Vec<u8>,output:Vec<u8>,cancelled:bool,failed:Option<BinarySourceError>,
}
impl BinarySourceJob{
 pub fn new(input:BinarySourceInput)->Result<Self,BinarySourceError>{
  if !(1..=268439552).contains(&input.max_source_bytes)||input.data.len()>input.max_source_bytes||input.mime.len()>128||!mime(&input.mime)||!(1..=67108864).contains(&input.max_bytes)||input.min_bytes>input.max_bytes||!(1..=1000000000).contains(&input.max_work){return Err(invalid("Invalid binary source contract"));}
  let source_length=input.data.len();let url=input.data.as_bytes().get(..5).is_some_and(|v|v.eq_ignore_ascii_case(b"data:"));let at=if url{5}else{0};
  Ok(Self{source:Some(input.data),source_length,mime:input.mime,min_bytes:input.min_bytes,max_bytes:input.max_bytes,max_work:input.max_work,phase:if url{"header"}else{"validate"},at,body_start:0,read:at,source_total:source_length*2,base64:true,url,work:0,bytes:0,total_bytes:0,quad:[0;4],quad_at:0,binary:Vec::new(),output:Vec::new(),cancelled:false,failed:None})
 }
 fn check(&self)->Result<(),BinarySourceError>{if self.cancelled{return Err(BinarySourceError::Cancelled);}if let Some(e)=&self.failed{return Err(e.clone());}Ok(())}
 fn release(&mut self){self.source=None;self.binary=Vec::new();self.output=Vec::new();}
 fn header_step(&mut self)->Result<(),BinarySourceError>{
  if self.at>=self.source_length||self.at>=4096{return Err(invalid("Missing or excessive data URL header"));}
  let text=self.source.as_ref().unwrap();let value=text.as_bytes()[self.at];self.at+=1;self.read+=1;
  if value!=b','{if !(33..=126).contains(&value)||value==b'#'{return Err(invalid("Invalid data URL header"));}return Ok(());}
  let mut parts=text[5..self.at-1].split(';').peekable();
  if !parts.next().unwrap().eq_ignore_ascii_case(&self.mime){return Err(invalid(format!("Data URL media type does not match {}",self.mime)));}
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
 fn token(&mut self)->Result<u8,BinarySourceError>{
  let source=self.source.as_ref().unwrap().as_bytes();let mut value=source[self.at];self.at+=1;self.read+=1;
  if self.url&&value==b'%'{
   if self.at+2>self.source_length{return Err(invalid("Truncated source escape"));}
   let (Some(a),Some(b))=(hex(source[self.at]),hex(source[self.at+1]))else{return Err(invalid("Invalid source escape"));};self.at+=2;self.read+=2;value=a*16+b;
  }else if value>127||!self.base64&&!safe(value){return Err(invalid("Invalid source byte"));}
  Ok(value)
 }
 fn source_step(&mut self)->Result<(),BinarySourceError>{
  let validating=self.phase=="validate";
  if self.at==self.source_length{
   if self.quad_at!=0{return Err(invalid("Invalid base64 length"));}
   if validating{if self.total_bytes<self.min_bytes{return Err(invalid("Encoded source is too short"));}self.binary.try_reserve_exact(self.total_bytes).map_err(invalid)?;self.at=self.body_start;self.phase="decode";}
   else{self.output=std::mem::take(&mut self.binary);self.source=None;self.phase="complete";}
   return Ok(());
  }
  let value=self.token()?;
  if self.base64{
   if value>127{return Err(invalid("Invalid base64 byte"));}
   self.quad[self.quad_at]=value;self.quad_at+=1;
   if self.quad_at==4{let (bytes,count)=decode_standard_quad(self.quad,self.at-4,self.at==self.source_length).map_err(invalid)?;self.quad_at=0;if validating{self.total_bytes+=count;}else{self.binary.extend_from_slice(&bytes[..count]);self.bytes+=count;}}
  }else if validating{self.total_bytes+=1;}else{self.binary.push(value);self.bytes+=1;}
  if self.total_bytes>self.max_bytes{return Err(invalid("Encoded source byte limit exceeded"));}Ok(())
 }
 pub fn advance(&mut self,budget:usize)->Result<BinarySourceProgress,BinarySourceError>{
  if budget==0||budget as u64>9007199254740991{return Err(invalid("Binary source work grant must be a positive integer"));}self.check()?;
  for _ in 0..budget{if self.phase=="complete"{break;}let result=if self.work>=self.max_work{Err(invalid("Binary source work limit exceeded"))}else if self.phase=="header"{self.header_step()}else{self.source_step()};if let Err(e)=result{self.failed=Some(e.clone());self.release();return Err(e);}self.work+=1;}
  Ok(BinarySourceProgress{phase:self.phase,source_completed:self.read,source_total:self.source_total,bytes:self.bytes,total_bytes:self.total_bytes,work:self.work,done:self.phase=="complete"})
 }
 pub fn cancel(&mut self){self.cancelled=true;self.release();}
 pub fn result(&self)->Result<&[u8],BinarySourceError>{self.check()?;if self.phase!="complete"{return Err(BinarySourceError::Incomplete);}Ok(&self.output)}
 pub fn into_result(self)->Result<Vec<u8>,BinarySourceError>{self.check()?;if self.phase!="complete"{return Err(BinarySourceError::Incomplete);}Ok(self.output)}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
