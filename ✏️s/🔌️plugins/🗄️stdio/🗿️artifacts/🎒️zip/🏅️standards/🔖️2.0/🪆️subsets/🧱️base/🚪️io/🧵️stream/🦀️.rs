//! 🧵️ Bounded stored-entry ZIP emission with data descriptors and no retained file bodies.
const LIMIT:usize=4096;
#[derive(Debug)]
struct Entry{name:String,crc:u32,size:u32,offset:u32,flags:u16}
#[derive(Debug,Default)]
pub struct StoredZipStream{entries:Vec<Entry>,active:Option<Entry>,offset:u32,central_offset:Option<u32>,central_index:usize,finished:bool}
fn word(out:&mut Vec<u8>,value:u16){out.extend_from_slice(&value.to_le_bytes());}
fn wide(out:&mut Vec<u8>,value:u32){out.extend_from_slice(&value.to_le_bytes());}
fn crc(mut value:u32,bytes:&[u8])->u32{for &byte in bytes{value^=u32::from(byte);for _ in 0..8{value=(value>>1)^(0xedb8_8320u32.wrapping_mul(value&1));}}value}
impl StoredZipStream{
    pub const MAXIMUM_CHUNK_BYTES:usize=LIMIT;
    pub fn new()->Self{Self::default()}
    pub fn entry_is_open(&self)->bool{self.active.is_some()}
    fn admit_name(&self,name:&str)->Result<(),String>{if self.finished||self.central_offset.is_some()||self.active.is_some(){return Err("zip.stream.phase".into());}if self.entries.len()>=1024||name.is_empty()||name.len()>256||self.entries.iter().any(|entry|entry.name==name){return Err("zip.stream.entry".into());}Ok(())}
    fn publish(&mut self,bytes:Vec<u8>)->Result<Vec<u8>,String>{if bytes.len()>LIMIT{return Err("zip.stream.chunk-limit".into());}self.offset=self.offset.checked_add(u32::try_from(bytes.len()).map_err(|_|"zip.stream.size")?).ok_or("zip.stream.size")?;Ok(bytes)}
    fn header(name:&str,flags:u16,crc:u32,size:u32)->Vec<u8>{let mut out=Vec::with_capacity(30+name.len());wide(&mut out,0x04034b50);word(&mut out,20);word(&mut out,flags);word(&mut out,0);word(&mut out,0);word(&mut out,0x21);wide(&mut out,crc);wide(&mut out,size);wide(&mut out,size);word(&mut out,name.len()as u16);word(&mut out,0);out.extend_from_slice(name.as_bytes());out}
    pub fn begin_entry(&mut self,name:&str)->Result<Vec<u8>,String>{self.admit_name(name)?;let out=Self::header(name,0x0808,0,0);self.active=Some(Entry{name:name.into(),crc:u32::MAX,size:0,offset:self.offset,flags:0x0808});self.publish(out)}
    pub fn write_chunk(&mut self,bytes:&[u8])->Result<Vec<u8>,String>{if bytes.len()>LIMIT{return Err("zip.stream.chunk-limit".into());}let entry=self.active.as_mut().ok_or("zip.stream.no-entry")?;entry.crc=crc(entry.crc,bytes);entry.size=entry.size.checked_add(bytes.len()as u32).ok_or("zip.stream.size")?;self.publish(bytes.to_vec())}
    pub fn end_entry(&mut self)->Result<Vec<u8>,String>{let mut entry=self.active.take().ok_or("zip.stream.no-entry")?;entry.crc^=u32::MAX;let mut out=Vec::with_capacity(16);wide(&mut out,0x08074b50);wide(&mut out,entry.crc);wide(&mut out,entry.size);wide(&mut out,entry.size);self.entries.push(entry);self.publish(out)}
    pub fn known_entry(&mut self,name:&str,bytes:&[u8])->Result<Vec<u8>,String>{self.admit_name(name)?;if 30+name.len()+bytes.len()>LIMIT{return Err("zip.stream.chunk-limit".into());}let checksum=crc(u32::MAX,bytes)^u32::MAX;let mut out=Self::header(name,0x0800,checksum,bytes.len()as u32);out.extend_from_slice(bytes);self.entries.push(Entry{name:name.into(),crc:checksum,size:bytes.len()as u32,offset:self.offset,flags:0x0800});self.publish(out)}
    pub fn finish_one(&mut self)->Result<Option<Vec<u8>>,String>{
        if self.active.is_some(){return Err("zip.stream.open-entry".into());}if self.finished{return Ok(None);}let start=*self.central_offset.get_or_insert(self.offset);
        if let Some(entry)=self.entries.get(self.central_index){let mut out=Vec::with_capacity(46+entry.name.len());wide(&mut out,0x02014b50);word(&mut out,20);word(&mut out,20);word(&mut out,entry.flags);word(&mut out,0);word(&mut out,0);word(&mut out,0x21);wide(&mut out,entry.crc);wide(&mut out,entry.size);wide(&mut out,entry.size);word(&mut out,entry.name.len()as u16);word(&mut out,0);word(&mut out,0);word(&mut out,0);word(&mut out,0);wide(&mut out,0);wide(&mut out,entry.offset);out.extend_from_slice(entry.name.as_bytes());self.central_index+=1;return self.publish(out).map(Some);}
        let mut out=Vec::with_capacity(22);wide(&mut out,0x06054b50);word(&mut out,0);word(&mut out,0);word(&mut out,self.entries.len()as u16);word(&mut out,self.entries.len()as u16);wide(&mut out,self.offset-start);wide(&mut out,start);word(&mut out,0);self.finished=true;self.publish(out).map(Some)
    }
    pub fn next_release_bytes(&self)->usize{if let Some(entry)=&self.active{return entry.name.capacity();}if let Some(entry)=self.entries.last(){return entry.name.capacity();}self.entries.capacity()*std::mem::size_of::<Entry>()}
    pub fn close_one(&mut self,maximum_bytes:usize)->bool{if self.next_release_bytes()>maximum_bytes{return false;}if self.active.take().is_some(){return true;}if self.entries.pop().is_some(){return true;}if self.entries.capacity()!=0{self.entries=Vec::new();return true;}false}
    pub fn terminal_is_empty(&self)->bool{self.active.is_none()&&self.entries.capacity()==0}
}
