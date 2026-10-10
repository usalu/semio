//! 🪄️ Bounded actual OpenType feature records select borrowed lookup authority in original table order.
use super::{FontFace,FontProgress,FontReader,reserve_slot};
use semio_framework_value::{list::PagedList,retirement::{RetireOwned,RetirementCursor}};
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct FontLookup{pub index:u16,pub kind:u16,pub flags:u16,pub offset:usize,pub subtables:usize}
semio_framework_value::artifact_retire_leaf!(FontLookup);
#[derive(semio_framework_value::RetireOwned)]
struct Owners{output:Option<PagedList<FontLookup,{usize::MAX}>>,failure:Option<String>}
pub struct FontLookupPlanCursor{owners:Owners,face:FontFace,substitution:bool,script:u32,features:[u32;128],feature_count:usize,max_work:u64,phase:u8,script_list:usize,feature_list:usize,lookup_list:usize,selected_script:usize,default_script:usize,language:usize,count:usize,at:usize,required:u16,active:[u16;128],active_count:usize,indices:[u16;128],index_count:usize,feature:usize,feature_at:usize,sort_at:usize,sort_probe:usize,work:u64,cancelled:bool}
impl RetireOwned for FontLookupPlanCursor{fn retirement(self)->Box<dyn RetirementCursor>{self.owners.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.owners.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}}
impl FontLookupPlanCursor{
 pub fn new(face:FontFace,substitution:bool,script:[u8;4],features:&[[u8;4]],max_work:u64)->Self{let invalid=features.len()>128||!(1..=1000000000).contains(&max_work);let mut tags=[0;128];for(index,feature)in features.iter().take(128).enumerate(){tags[index]=u32::from_be_bytes(*feature);}Self{owners:Owners{output:Some(PagedList::new()),failure:invalid.then(||"Invalid font lookup policy".into())},face,substitution,script:u32::from_be_bytes(script),features:tags,feature_count:features.len().min(128),max_work,phase:0,script_list:0,feature_list:0,lookup_list:0,selected_script:0,default_script:0,language:0,count:0,at:0,required:u16::MAX,active:[0;128],active_count:0,indices:[0;128],index_count:0,feature:0,feature_at:0,sort_at:1,sort_probe:1,work:0,cancelled:false}}
 fn step(&mut self,bytes:&[u8])->Result<(),String>{if bytes.len()!=self.face.byte_length{return Err("Captured font source changed".into());}let table=if self.substitution{self.face.gsub}else{self.face.gpos};let Some(table)=table else{self.phase=7;return Ok(());};let r=FontReader::new(bytes,table)?;match self.phase{
  0=>{if r.u16(table.offset)?!=1{return Err("Unsupported font layout version".into());}self.script_list=table.offset+usize::from(r.u16(table.offset+4)?);self.feature_list=table.offset+usize::from(r.u16(table.offset+6)?);self.lookup_list=table.offset+usize::from(r.u16(table.offset+8)?);self.count=usize::from(r.u16(self.script_list)?);if self.count>128{return Err("Font script limit exceeded".into());}r.range(self.script_list+2,self.count*6)?;self.phase=1;},
  1=>{if self.at<self.count{let at=self.script_list+2+self.at*6;self.at+=1;let tag=r.u32(at)?;let offset=self.script_list+usize::from(r.u16(at+4)?);if tag==self.script{self.selected_script=offset;}if tag==u32::from_be_bytes(*b"DFLT"){self.default_script=offset;}return Ok(());}if self.selected_script==0{self.selected_script=self.default_script;}if self.selected_script==0{self.phase=7;return Ok(());}let offset=usize::from(r.u16(self.selected_script)?);if offset==0{self.phase=7;return Ok(());}self.language=self.selected_script+offset;self.required=r.u16(self.language+2)?;self.count=usize::from(r.u16(self.language+4)?);if self.count>128{return Err("Font language feature limit exceeded".into());}r.range(self.language+6,self.count*2)?;self.at=0;if self.required!=u16::MAX{self.active[0]=self.required;self.active_count=1;}self.phase=2;},
  2=>{if self.at<self.count{let index=r.u16(self.language+6+self.at*2)?;self.at+=1;if !self.active[..self.active_count].contains(&index){if self.active_count==128{return Err("Font active feature limit exceeded".into());}self.active[self.active_count]=index;self.active_count+=1;}return Ok(());}self.at=0;self.phase=3;},
  3=>{if self.at==self.active_count{self.phase=5;return Ok(());}let index=self.active[self.at];self.at+=1;if index>=r.u16(self.feature_list)?{return Err("Font feature index exceeds authority".into());}let at=self.feature_list+2+usize::from(index)*6;let tag=r.u32(at)?;if index!=self.required&&!self.features[..self.feature_count].contains(&tag){return Ok(());}self.feature=self.feature_list+usize::from(r.u16(at+4)?);self.count=usize::from(r.u16(self.feature+2)?);if self.count>128{return Err("Font feature lookup limit exceeded".into());}r.range(self.feature+4,self.count*2)?;self.feature_at=0;self.phase=4;},
  4=>{if self.feature_at<self.count{let index=r.u16(self.feature+4+self.feature_at*2)?;self.feature_at+=1;if !self.indices[..self.index_count].contains(&index){if self.index_count==128{return Err("Font combined lookup limit exceeded".into());}self.indices[self.index_count]=index;self.index_count+=1;}return Ok(());}self.phase=3;},
  5=>{if self.sort_at>=self.index_count{self.at=0;self.phase=6;return Ok(());}if self.sort_probe>0&&self.indices[self.sort_probe-1]>self.indices[self.sort_probe]{self.indices.swap(self.sort_probe-1,self.sort_probe);self.sort_probe-=1;return Ok(());}self.sort_at+=1;self.sort_probe=self.sort_at;},
  6=>{if self.at==self.index_count{self.phase=7;return Ok(());}if !reserve_slot(self.owners.output.as_mut().unwrap())?{return Ok(());}let index=self.indices[self.at];if index>=r.u16(self.lookup_list)?{return Err("Font lookup index exceeds authority".into());}let offset=self.lookup_list+usize::from(r.u16(self.lookup_list+2+usize::from(index)*2)?);let kind=r.u16(offset)?;let flags=r.u16(offset+2)?;let subtables=usize::from(r.u16(offset+4)?);if subtables>128{return Err("Font lookup subtable limit exceeded".into());}r.range(offset+6,subtables*2+if flags&16!=0{2}else{0})?;self.owners.output.as_mut().unwrap().push_reserved(FontLookup{index,kind,flags,offset,subtables}).map_err(|_|"Font lookup reservation lost")?;self.at+=1;},
  _=>return Err("Font lookup selection stage invalid".into()),
 }Ok(())}
 pub fn advance(&mut self,bytes:&[u8],grant:usize)->Result<FontProgress,String>{if grant==0{return Err("Invalid font lookup work grant".into());}if self.cancelled{return Err("Font lookup selection cancelled".into());}if let Some(error)=&self.owners.failure{return Err(error.clone());}for _ in 0..grant{if self.phase==7{break;}if self.work==self.max_work{self.owners.failure=Some("Font lookup selection work limit exceeded".into());return Err(self.owners.failure.as_ref().unwrap().clone());}if let Err(error)=self.step(bytes){self.owners.failure=Some(error.clone());return Err(error);}self.work+=1;}Ok(FontProgress{phase:match self.phase{0=>"header",1=>"scripts",2=>"language",3=>"features",4=>"featureLookups",5=>"sort",6=>"lookups",_=>"complete"},work:self.work,done:self.phase==7})}
 pub fn result(&self)->Result<&PagedList<FontLookup,{usize::MAX}>,String>{if self.cancelled||self.phase!=7||self.owners.failure.is_some(){return Err("Font lookup selection incomplete".into());}self.owners.output.as_ref().ok_or_else(||"Font lookup selection ownership transferred".into())}
 pub fn take_result(&mut self)->Result<PagedList<FontLookup,{usize::MAX}>,String>{self.result()?;Ok(self.owners.output.take().unwrap())}
 pub fn cancel(&mut self){self.cancelled=true;}
}
#[path="🔗️ligature/🦀️.rs"]
pub mod ligature;
#[path="📍️mark/🦀️.rs"]
pub mod mark;
#[path="🌐️unicode/🦀️.rs"]
pub mod unicode;
#[path="📇️face/🦀️.rs"]
pub mod face;
#[path="🪄️substitute/🦀️.rs"]
pub mod substitute;
#[path="🏷️class/🦀️.rs"]
pub mod class;
#[path="🧵️layout/🦀️.rs"]
pub mod layout;
#[path="🧩️context/🦀️.rs"]
pub mod context;
