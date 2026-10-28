//! 📇️ Explicit shipped families retain their actual immutable bytes through progressive admission.
use semio_framework_2d_text_font::{AdmittedFont,StaticFontBytes,FontFaceAdmissionJob,FontOutlineLimits,FontProgress};
use semio_framework_value::retirement::{RetireOwned, RetirementCursor};
use semio_framework_value_persistent::list::PagedList;
pub const DEFAULT_FAMILY:&str="Anta";
pub struct FontSourceLocation{pub family:&'static str,pub id:&'static str,pub bytes:StaticFontBytes}
pub static FONT_SOURCES:[FontSourceLocation;20]=[
 FontSourceLocation{family:"Anta",id:"latin",bytes:StaticFontBytes(semio_framework_font_anta_latin_regular::OUTLINE)},
 FontSourceLocation{family:"Anta",id:"latin-ext",bytes:StaticFontBytes(semio_framework_font_anta_latin_ext_regular::OUTLINE)},
 FontSourceLocation{family:"Anta",id:"math",bytes:StaticFontBytes(semio_framework_font_anta_math_regular::OUTLINE)},
 FontSourceLocation{family:"Anta",id:"symbols",bytes:StaticFontBytes(semio_framework_font_anta_symbols_regular::OUTLINE)},
 FontSourceLocation{family:"Kelly Slab",id:"latin",bytes:StaticFontBytes(semio_framework_font_kelly_slab_latin_regular::OUTLINE)},
 FontSourceLocation{family:"Kelly Slab",id:"latin-ext",bytes:StaticFontBytes(semio_framework_font_kelly_slab_latin_ext_regular::OUTLINE)},
 FontSourceLocation{family:"Kelly Slab",id:"cyrillic",bytes:StaticFontBytes(semio_framework_font_kelly_slab_cyrillic_regular::OUTLINE)},
 FontSourceLocation{family:"Share Tech Mono",id:"latin",bytes:StaticFontBytes(semio_framework_font_share_tech_mono_latin_regular::OUTLINE)},
 FontSourceLocation{family:"Noto Emoji",id:"regions",bytes:StaticFontBytes(semio_framework_font_noto_emoji_regions_regular::OUTLINE)},
 FontSourceLocation{family:"Noto Emoji",id:"flags",bytes:StaticFontBytes(semio_framework_font_noto_emoji_flags_regular::OUTLINE)},
 FontSourceLocation{family:"Noto Emoji",id:"symbols",bytes:StaticFontBytes(semio_framework_font_noto_emoji_symbols_regular::OUTLINE)},
 FontSourceLocation{family:"Noto Emoji",id:"objects",bytes:StaticFontBytes(semio_framework_font_noto_emoji_objects_regular::OUTLINE)},
 FontSourceLocation{family:"Noto Emoji",id:"activities",bytes:StaticFontBytes(semio_framework_font_noto_emoji_activities_regular::OUTLINE)},
 FontSourceLocation{family:"Noto Emoji",id:"travel",bytes:StaticFontBytes(semio_framework_font_noto_emoji_travel_regular::OUTLINE)},
 FontSourceLocation{family:"Noto Emoji",id:"food",bytes:StaticFontBytes(semio_framework_font_noto_emoji_food_regular::OUTLINE)},
 FontSourceLocation{family:"Noto Emoji",id:"nature",bytes:StaticFontBytes(semio_framework_font_noto_emoji_nature_regular::OUTLINE)},
 FontSourceLocation{family:"Noto Emoji",id:"people",bytes:StaticFontBytes(semio_framework_font_noto_emoji_people_regular::OUTLINE)},
 FontSourceLocation{family:"Noto Emoji",id:"faces",bytes:StaticFontBytes(semio_framework_font_noto_emoji_faces_regular::OUTLINE)},
 FontSourceLocation{family:"Noto Emoji",id:"joined-forms",bytes:StaticFontBytes(semio_framework_font_noto_emoji_joined_forms_regular::OUTLINE)},
 FontSourceLocation{family:"Noto Emoji",id:"supplement",bytes:StaticFontBytes(semio_framework_font_noto_emoji_supplement_regular::OUTLINE)},
];
#[derive(semio_framework_value::RetireOwned)]
struct Owners{child:Option<FontFaceAdmissionJob<StaticFontBytes>>,children:PagedList<FontFaceAdmissionJob<StaticFontBytes>,{usize::MAX}>,output:Option<Vec<AdmittedFont<StaticFontBytes>>>,failure:Option<&'static str>}
pub struct FontCatalogAdmissionJob{owners:Owners,limits:FontOutlineLimits,indices:[usize;32],count:usize,at:usize,work:u64,cancelled:bool,reserved:bool}
impl RetireOwned for FontCatalogAdmissionJob{fn retirement(self)->Box<dyn RetirementCursor>{self.owners.retirement()}fn retirement_birth_bytes(&self)->Option<usize>{self.owners.retirement_birth_bytes()}fn controlled_retirement_supported()->bool{true}}
impl FontCatalogAdmissionJob{
 pub fn new(family:&str,weight:u16,limits:FontOutlineLimits)->Self{let mut indices=[0;32];let mut count=0;for(index,source)in FONT_SOURCES.iter().enumerate(){if source.family==family{indices[count]=index;count+=1;}}let failure=if count==0{Some("Unsupported authored font family".into())}else if weight!=400{Some("Unsupported authored font weight".into())}else{None};if count>0&&family!="Noto Emoji"{for(index,source)in FONT_SOURCES.iter().enumerate(){if source.family=="Noto Emoji"{indices[count]=index;count+=1;}}}Self{owners:Owners{child:None,children:PagedList::new(),output:Some(Vec::new()),failure},limits,indices,count,at:0,work:0,cancelled:false,reserved:false}}
 fn step(&mut self,work:&mut semio_framework_2d_text_font::work::FontWorkLedger)->Result<(),&'static str>{if !work.can_step(){return Ok(());}if !self.reserved{if !work.contiguous(self.owners.output.as_mut().unwrap(),self.count)?{return Ok(());}self.reserved=true;return Ok(());}if self.owners.child.is_none(){let source=&FONT_SOURCES[self.indices[self.at]];self.owners.child=Some(FontFaceAdmissionJob::new(source.bytes,self.limits));return Ok(());}if !self.owners.child.as_mut().unwrap().advance(1,work)?.done{return Ok(());}if !work.slot(&mut self.owners.children)?||!work.vec_slot(self.owners.output.as_mut().unwrap())?{return Ok(());}let output=self.owners.child.as_mut().unwrap().take_result()?;work.vec_push(self.owners.output.as_mut().unwrap(),output).map_err(|_|"Font catalog result copy authority")?;{let value=self.owners.child.take().unwrap();work.push(&mut self.owners.children,value)}.map_err(|_|"Font catalog child reservation lost")?;self.at+=1;Ok(())}
 pub fn advance(&mut self,grant:usize,work:&mut semio_framework_2d_text_font::work::FontWorkLedger)->Result<FontProgress,&'static str>{let entered=work.enter();let result=(||{if grant==0{return Err("Invalid font catalog grant".into());}if self.cancelled{return Err("Font catalog cancelled".into());}if let Some(error)=&self.owners.failure{return Err(error.clone());}for _ in 0..(if entered.is_some(){grant.min(work.remaining().maximum_items)}else{0}){if self.at==self.count{break;}if self.work==self.limits.max_work{self.owners.failure=Some("Font catalog work limit exceeded".into());return Err(self.owners.failure.as_ref().unwrap().clone());}if let Err(error)=self.step(work){self.owners.failure=Some(error.clone());return Err(error);}if work.blocked(){break;}self.work+=1;work.note_source(entered==Some(true));}Ok(FontProgress{phase:if self.at==self.count{"complete"}else{"admitting"},work:self.work,done:self.at==self.count})})();if entered.is_some(){work.exit();}result}
 pub fn result(&self)->Result<&[AdmittedFont<StaticFontBytes>],&'static str>{if self.cancelled||self.owners.failure.is_some()||self.at!=self.count{return Err("Font catalog incomplete".into());}self.owners.output.as_deref().ok_or_else(||"Font catalog ownership transferred".into())}
 pub fn take_result(&mut self)->Result<Vec<AdmittedFont<StaticFontBytes>>,&'static str>{self.result()?;Ok(self.owners.output.take().unwrap())}
 pub fn cancel(&mut self){self.cancelled=true;}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
