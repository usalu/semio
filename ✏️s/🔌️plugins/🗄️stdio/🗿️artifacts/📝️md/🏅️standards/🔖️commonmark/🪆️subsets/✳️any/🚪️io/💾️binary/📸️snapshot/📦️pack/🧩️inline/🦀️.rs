//! 🧩️ Explicit nine-kind CommonMark inline records with exclusive ordered child identities.
use super::*;
#[derive(semio_framework_dsl_record_derive::DslScalar)]
enum Kind{Text,Emphasis,Strong,Code,Link,Image,SoftBreak,HardBreak,HtmlInline}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(super) struct Inline{kind:Kind,text:Option<String>,literal:Option<String>,url:Option<String>,title:Option<String>,alt:Option<String>,raw:Option<String>,inlines:Vec<u64>}
pub(super) fn project<'a>(value:&'a MdInline,pending:&mut std::collections::VecDeque<&'a MdInline>,next:&mut u64)->Inline{let mut row=Inline{kind:Kind::SoftBreak,text:None,literal:None,url:None,title:None,alt:None,raw:None,inlines:Vec::new()};match value{
 MdInline::Text{text}=>{row.kind=Kind::Text;row.text=Some(text.clone())},MdInline::Emphasis{inlines}=>{row.kind=Kind::Emphasis;row.inlines=indices(inlines,pending,next)},MdInline::Strong{inlines}=>{row.kind=Kind::Strong;row.inlines=indices(inlines,pending,next)},MdInline::Code{literal}=>{row.kind=Kind::Code;row.literal=Some(literal.clone())},MdInline::Link{text,url,title}=>{row.kind=Kind::Link;row.inlines=indices(text,pending,next);row.url=Some(url.clone());row.title=title.clone()},MdInline::Image{alt,url,title}=>{row.kind=Kind::Image;row.alt=Some(alt.clone());row.url=Some(url.clone());row.title=title.clone()},MdInline::SoftBreak=>{},MdInline::HardBreak=>row.kind=Kind::HardBreak,MdInline::HtmlInline{raw}=>{row.kind=Kind::HtmlInline;row.raw=Some(raw.clone())}}row}
pub(super) fn reconstruct(rows:Vec<Inline>)->Result<Vec<Option<MdInline>>,String>{let mut values=(0..rows.len()).map(|_|None).collect::<Vec<_>>();for(index,row)in rows.into_iter().enumerate().rev(){let Inline{kind,mut text,mut literal,mut url,mut title,mut alt,mut raw,mut inlines}=row;let inline=match kind{
 Kind::Text=>MdInline::Text{text:text.take().ok_or("CommonMark text is missing")?},Kind::Emphasis=>MdInline::Emphasis{inlines:children(&mut values,Some(index),std::mem::take(&mut inlines))?},Kind::Strong=>MdInline::Strong{inlines:children(&mut values,Some(index),std::mem::take(&mut inlines))?},Kind::Code=>MdInline::Code{literal:literal.take().ok_or("CommonMark inline code literal is missing")?},Kind::Link=>MdInline::Link{text:children(&mut values,Some(index),std::mem::take(&mut inlines))?,url:url.take().ok_or("CommonMark link URL is missing")?,title:title.take()},Kind::Image=>MdInline::Image{alt:alt.take().ok_or("CommonMark image alternative text is missing")?,url:url.take().ok_or("CommonMark image URL is missing")?,title:title.take()},Kind::SoftBreak=>MdInline::SoftBreak,Kind::HardBreak=>MdInline::HardBreak,Kind::HtmlInline=>MdInline::HtmlInline{raw:raw.take().ok_or("CommonMark inline HTML raw text is missing")?}};
 if text.is_some()||literal.is_some()||url.is_some()||title.is_some()||alt.is_some()||raw.is_some()||!inlines.is_empty(){return Err("CommonMark inline contains unrelated variant fields".into())}values[index]=Some(inline)}Ok(values)}

pub(super) fn project_controlled<'a>(value:&'a MdInline,pending:&mut std::collections::VecDeque<&'a MdInline>,next:&mut u64,control:&mut NativeEncodeControl<'_>)->Result<Inline,ValueError>{
 let mut row=Inline{kind:Kind::SoftBreak,text:None,literal:None,url:None,title:None,alt:None,raw:None,inlines:Vec::new()};match value{
 MdInline::Text{text}=>{row.kind=Kind::Text;row.text=Some(control.copy_text(text)?);},MdInline::Emphasis{inlines}=>{row.kind=Kind::Emphasis;row.inlines=paid_indices(inlines,pending,next,control)?;},MdInline::Strong{inlines}=>{row.kind=Kind::Strong;row.inlines=paid_indices(inlines,pending,next,control)?;},MdInline::Code{literal}=>{row.kind=Kind::Code;row.literal=Some(control.copy_text(literal)?);},
 MdInline::Link{text,url,title}=>{row.kind=Kind::Link;row.inlines=paid_indices(text,pending,next,control)?;row.url=Some(control.copy_text(url)?);row.title=paid_option(title,control)?;},MdInline::Image{alt,url,title}=>{row.kind=Kind::Image;row.alt=Some(control.copy_text(alt)?);row.url=Some(control.copy_text(url)?);row.title=paid_option(title,control)?;},MdInline::SoftBreak=>{},MdInline::HardBreak=>row.kind=Kind::HardBreak,MdInline::HtmlInline{raw}=>{row.kind=Kind::HtmlInline;row.raw=Some(control.copy_text(raw)?);}}
 Ok(row)
}
pub(super) fn reconstruct_controlled(rows:Vec<Inline>,control:&mut NativeDecodeControl<'_>)->Result<OwnedSlots<MdInline>,ValueError>{
 let mut values=OwnedSlots(control.allocate_vec(rows.len())?);values.0.resize_with(rows.len(),||None);control.begin_stage(rows.len())?;
 for(index,row)in rows.into_iter().enumerate().rev(){let Inline{kind,mut text,mut literal,mut url,mut title,mut alt,mut raw,mut inlines}=row;let fields_allowed=match kind{
 Kind::Text=>literal.is_none()&&url.is_none()&&title.is_none()&&alt.is_none()&&raw.is_none()&&inlines.is_empty(),
 Kind::Emphasis|Kind::Strong=>text.is_none()&&literal.is_none()&&url.is_none()&&title.is_none()&&alt.is_none()&&raw.is_none(),
 Kind::Code=>text.is_none()&&url.is_none()&&title.is_none()&&alt.is_none()&&raw.is_none()&&inlines.is_empty(),
 Kind::Link=>text.is_none()&&literal.is_none()&&alt.is_none()&&raw.is_none(),
 Kind::Image=>text.is_none()&&literal.is_none()&&raw.is_none()&&inlines.is_empty(),
 Kind::SoftBreak|Kind::HardBreak=>text.is_none()&&literal.is_none()&&url.is_none()&&title.is_none()&&alt.is_none()&&raw.is_none()&&inlines.is_empty(),
 Kind::HtmlInline=>text.is_none()&&literal.is_none()&&url.is_none()&&title.is_none()&&alt.is_none()&&inlines.is_empty()};if !fields_allowed{return Err(invalid("CommonMark inline contains unrelated variant fields"))}
 let inline=match kind{
 Kind::Text=>MdInline::Text{text:text.take().ok_or_else(||invalid("CommonMark text is missing"))?},Kind::Emphasis=>MdInline::Emphasis{inlines:paid_children(&mut values.0,Some(index),std::mem::take(&mut inlines),control)?},Kind::Strong=>MdInline::Strong{inlines:paid_children(&mut values.0,Some(index),std::mem::take(&mut inlines),control)?},Kind::Code=>MdInline::Code{literal:literal.take().ok_or_else(||invalid("CommonMark inline code literal is missing"))?},
 Kind::Link=>{let url=url.take().ok_or_else(||invalid("CommonMark link URL is missing"))?;MdInline::Link{text:paid_children(&mut values.0,Some(index),std::mem::take(&mut inlines),control)?,url,title:title.take()}},Kind::Image=>MdInline::Image{alt:alt.take().ok_or_else(||invalid("CommonMark image alternative text is missing"))?,url:url.take().ok_or_else(||invalid("CommonMark image URL is missing"))?,title:title.take()},Kind::SoftBreak=>MdInline::SoftBreak,Kind::HardBreak=>MdInline::HardBreak,Kind::HtmlInline=>MdInline::HtmlInline{raw:raw.take().ok_or_else(||invalid("CommonMark inline HTML raw text is missing"))?}};
 values.0[index]=Some(inline);control.step()?;
 }Ok(values)
}
