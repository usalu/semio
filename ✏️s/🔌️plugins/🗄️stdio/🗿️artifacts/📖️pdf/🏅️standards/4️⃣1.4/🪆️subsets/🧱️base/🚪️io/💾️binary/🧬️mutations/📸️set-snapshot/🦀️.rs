//! 💾️ Own14 structured replacement pages with exact binary64 words.
use crate::standards::v1_4::subsets::base::schema::mutations::SetSnapshot;
use crate::standards::v1_4::subsets::base::schema::mutations::PdfMutation;
use crate::standards::v1_4::subsets::base::schema::snapshot::{PageDoc,PdfSnapshot};
pub const TAG:u8=dsl::protocol_record::tag_u8(include_str!("../📡️.protocol.semio"),"set-snapshot");
fn text(out:&mut Vec<u8>,value:&str)->Result<(),String>{out.extend_from_slice(&u64::try_from(value.len()).map_err(|error|error.to_string())?.to_le_bytes());out.extend_from_slice(value.as_bytes());Ok(())}
pub fn encode(mutation:&PdfMutation)->Option<Result<Vec<u8>,String>>{let PdfMutation::SetSnapshot(payload)=mutation else{return None};Some((||{let mut out=Vec::new();text(&mut out,&payload.snapshot.schema)?;out.extend_from_slice(&u64::try_from(payload.snapshot.pages.len()).map_err(|error|error.to_string())?.to_le_bytes());for page in &payload.snapshot.pages{out.extend_from_slice(&page.width.to_bits().to_le_bytes());out.extend_from_slice(&page.height.to_bits().to_le_bytes());text(&mut out,&page.text)?;}Ok(out)})())}
struct Reader<'a>{bytes:&'a[u8],at:usize}
impl<'a> Reader<'a>{
    fn take(&mut self,count:usize)->Result<&'a[u8],String>{let end=self.at.checked_add(count).ok_or("Replacement payload overflow")?;let value=self.bytes.get(self.at..end).ok_or("Truncated replacement payload")?;self.at=end;Ok(value)}
    fn word(&mut self)->Result<u64,String>{Ok(u64::from_le_bytes(self.take(8)?.try_into().unwrap()))}
    fn text(&mut self)->Result<String,String>{let count=usize::try_from(self.word()?).map_err(|error|error.to_string())?;String::from_utf8(self.take(count)?.to_vec()).map_err(|error|error.to_string())}
}
pub fn decode(bytes:&[u8])->Result<PdfMutation,String>{let mut reader=Reader{bytes,at:0};let schema=reader.text()?;let count=usize::try_from(reader.word()?).map_err(|error|error.to_string())?;if count>(bytes.len()-reader.at)/24{return Err("Truncated replacement page collection".into())}let mut pages=Vec::new();pages.try_reserve_exact(count).map_err(|error|error.to_string())?;for _ in 0..count{pages.push(PageDoc{width:f64::from_bits(reader.word()?),height:f64::from_bits(reader.word()?),text:reader.text()?});}if reader.at!=bytes.len(){return Err("Trailing replacement payload bytes".into())}Ok(PdfMutation::SetSnapshot(SetSnapshot{snapshot:PdfSnapshot{schema,pages}}))}
