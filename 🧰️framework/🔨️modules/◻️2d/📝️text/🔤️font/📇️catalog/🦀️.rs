//! 📇️ Explicit shipped families retain their actual immutable bytes through progressive admission.
use super::{AdmittedFont,StaticFontBytes,FontFaceAdmissionJob,FontOutlineLimits,FontProgress,reserve_slot};
use semio_framework_value::{list::PagedList,retirement::{RetireOwned,RetirementCursor}};
pub const DEFAULT_FAMILY:&str="Anta";
pub struct FontSourceLocation{pub family:&'static str,pub id:&'static str,pub bytes:StaticFontBytes}
macro_rules! source{($family:literal,$id:literal,$path:literal)=>{FontSourceLocation{family:$family,id:$id,bytes:StaticFontBytes(include_bytes!($path))}};}
pub static FONT_SOURCES:[FontSourceLocation;20]=[
 source!("Anta","latin","../../../../🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf"),
 source!("Anta","latin-ext","../../../../🖼️assets/🔤️fonts/🚀️anta/➕️latin-ext/📖️regular/🔤️outline.ttf"),
 source!("Anta","math","../../../../🖼️assets/🔤️fonts/🚀️anta/🧮️math/📖️regular/🔤️outline.ttf"),
 source!("Anta","symbols","../../../../🖼️assets/🔤️fonts/🚀️anta/🔣️symbols/📖️regular/🔤️outline.ttf"),
 source!("Kelly Slab","latin","../../../../🖼️assets/🔤️fonts/🧱️kelly-slab/🏛️latin/📖️regular/🔤️outline.ttf"),
 source!("Kelly Slab","latin-ext","../../../../🖼️assets/🔤️fonts/🧱️kelly-slab/➕️latin-ext/📖️regular/🔤️outline.ttf"),
 source!("Kelly Slab","cyrillic","../../../../🖼️assets/🔤️fonts/🧱️kelly-slab/🪆️cyrillic/📖️regular/🔤️outline.ttf"),
 source!("Share Tech Mono","latin","../../../../🖼️assets/🔤️fonts/⌨️share-tech-mono/🏛️latin/📖️regular/🔤️outline.ttf"),
 source!("Noto Emoji","regions","../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🌍️regions/📖️regular/🔤️outline.ttf"),
 source!("Noto Emoji","flags","../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🚩️flags/📖️regular/🔤️outline.ttf"),
 source!("Noto Emoji","symbols","../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔣️symbols/📖️regular/🔤️outline.ttf"),
 source!("Noto Emoji","objects","../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🧰️objects/📖️regular/🔤️outline.ttf"),
 source!("Noto Emoji","activities","../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🎯️activities/📖️regular/🔤️outline.ttf"),
 source!("Noto Emoji","travel","../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🧳️travel/📖️regular/🔤️outline.ttf"),
 source!("Noto Emoji","food","../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🍽️food/📖️regular/🔤️outline.ttf"),
 source!("Noto Emoji","nature","../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🌿️nature/📖️regular/🔤️outline.ttf"),
 source!("Noto Emoji","people","../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🧑️people/📖️regular/🔤️outline.ttf"),
 source!("Noto Emoji","faces","../../../../🖼️assets/🔤️fonts/😀️noto-emoji/😀️faces/📖️regular/🔤️outline.ttf"),
 source!("Noto Emoji","joined-forms","../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🔗️joined-forms/📖️regular/🔤️outline.ttf"),
 source!("Noto Emoji","supplement","../../../../🖼️assets/🔤️fonts/😀️noto-emoji/🪉️supplement/📖️regular/🔤️outline.ttf"),
];
#[derive(semio_framework_value::RetireOwned)]
struct Owners{child:Option<FontFaceAdmissionJob<StaticFontBytes>>,children:PagedList<FontFaceAdmissionJob<StaticFontBytes>,{usize::MAX}>,output:Option<Vec<AdmittedFont<StaticFontBytes>>>,failure:Option<String>}
pub struct FontCatalogAdmissionJob{owners:Owners,limits:FontOutlineLimits,indices:[usize;32],count:usize,at:usize,work:u64,cancelled:bool,reserved:bool}
impl RetireOwned for FontCatalogAdmissionJob{fn retirement(self)->Box<dyn RetirementCursor>{self.owners.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.owners.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}}
impl FontCatalogAdmissionJob{
 pub fn new(family:&str,weight:u16,limits:FontOutlineLimits)->Self{let mut indices=[0;32];let mut count=0;for(index,source)in FONT_SOURCES.iter().enumerate(){if source.family==family{indices[count]=index;count+=1;}}let failure=if count==0{Some("Unsupported authored font family".into())}else if weight!=400{Some("Unsupported authored font weight".into())}else{None};if count>0&&family!="Noto Emoji"{for(index,source)in FONT_SOURCES.iter().enumerate(){if source.family=="Noto Emoji"{indices[count]=index;count+=1;}}}Self{owners:Owners{child:None,children:PagedList::new(),output:Some(Vec::new()),failure},limits,indices,count,at:0,work:0,cancelled:false,reserved:false}}
 fn step(&mut self)->Result<(),String>{if !self.reserved{self.owners.output.as_mut().unwrap().try_reserve_exact(self.count).map_err(|_|"Font catalog output admission failed")?;self.reserved=true;return Ok(());}if self.owners.child.is_none(){let source=&FONT_SOURCES[self.indices[self.at]];self.owners.child=Some(FontFaceAdmissionJob::new(source.bytes,self.limits));return Ok(());}if !self.owners.child.as_mut().unwrap().advance(1)?.done{return Ok(());}if !reserve_slot(&mut self.owners.children)?{return Ok(());}let output=self.owners.child.as_mut().unwrap().take_result()?;self.owners.output.as_mut().unwrap().push(output);self.owners.children.push_reserved(self.owners.child.take().unwrap()).map_err(|_|"Font catalog child reservation lost")?;self.at+=1;Ok(())}
 pub fn advance(&mut self,grant:usize)->Result<FontProgress,String>{if grant==0{return Err("Invalid font catalog grant".into());}if self.cancelled{return Err("Font catalog cancelled".into());}if let Some(error)=&self.owners.failure{return Err(error.clone());}for _ in 0..grant{if self.at==self.count{break;}if self.work==self.limits.max_work{self.owners.failure=Some("Font catalog work limit exceeded".into());return Err(self.owners.failure.as_ref().unwrap().clone());}if let Err(error)=self.step(){self.owners.failure=Some(error.clone());return Err(error);}self.work+=1;}Ok(FontProgress{phase:if self.at==self.count{"complete"}else{"admitting"},work:self.work,done:self.at==self.count})}
 pub fn result(&self)->Result<&[AdmittedFont<StaticFontBytes>],String>{if self.cancelled||self.owners.failure.is_some()||self.at!=self.count{return Err("Font catalog incomplete".into());}self.owners.output.as_deref().ok_or_else(||"Font catalog ownership transferred".into())}
 pub fn take_result(&mut self)->Result<Vec<AdmittedFont<StaticFontBytes>>,String>{self.result()?;Ok(self.owners.output.take().unwrap())}
 pub fn cancel(&mut self){self.cancelled=true;}
}
