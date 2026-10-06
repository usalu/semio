//! 🧱️ Explicit seven-kind CommonMark block records and ordered list-item ownership.
use super::*;
#[derive(semio_framework_dsl_record_derive::DslScalar)]
enum Kind{Heading,Paragraph,List,CodeBlock,BlockQuote,ThematicBreak,HtmlBlock}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
struct ListItem{blocks:Vec<u64>}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
pub(super) struct Block{kind:Kind,level:Option<u8>,inlines:Vec<u64>,ordered:Option<bool>,start:Option<u32>,tight:Option<bool>,items:Vec<ListItem>,blocks:Vec<u64>,info:Option<String>,literal:Option<String>,raw:Option<String>}
pub(super) fn project<'a>(value:&'a MdBlock,pending:&mut std::collections::VecDeque<&'a MdBlock>,next:&mut u64,inline_pending:&mut std::collections::VecDeque<&'a MdInline>,next_inline:&mut u64)->Block{let mut row=Block{kind:Kind::ThematicBreak,level:None,inlines:Vec::new(),ordered:None,start:None,tight:None,items:Vec::new(),blocks:Vec::new(),info:None,literal:None,raw:None};match value{
 MdBlock::Heading{level,inlines}=>{row.kind=Kind::Heading;row.level=Some(*level);row.inlines=indices(inlines,inline_pending,next_inline)},
 MdBlock::Paragraph{inlines}=>{row.kind=Kind::Paragraph;row.inlines=indices(inlines,inline_pending,next_inline)},
 MdBlock::List{ordered,start,tight,items}=>{row.kind=Kind::List;row.ordered=Some(*ordered);row.start=*start;row.tight=Some(*tight);row.items=items.iter().map(|blocks|ListItem{blocks:indices(blocks,pending,next)}).collect()},
 MdBlock::CodeBlock{info,literal}=>{row.kind=Kind::CodeBlock;row.info=info.clone();row.literal=Some(literal.clone())},
 MdBlock::BlockQuote{blocks}=>{row.kind=Kind::BlockQuote;row.blocks=indices(blocks,pending,next)},
 MdBlock::ThematicBreak=>{},MdBlock::HtmlBlock{raw}=>{row.kind=Kind::HtmlBlock;row.raw=Some(raw.clone())}}row}
pub(super) fn reconstruct(rows:Vec<Block>,inlines:&mut[Option<MdInline>])->Result<Vec<Option<MdBlock>>,String>{let mut values=(0..rows.len()).map(|_|None).collect::<Vec<_>>();for(index,row)in rows.into_iter().enumerate().rev(){let Block{kind,mut level,inlines:inline_ids,mut ordered,mut start,mut tight,items,blocks:block_ids,mut info,mut literal,mut raw}=row;let(mut inline_ids,mut items,mut block_ids)=(inline_ids,items,block_ids);let block=match kind{
 Kind::Heading=>MdBlock::Heading{level:level.take().ok_or("CommonMark heading level is missing")?,inlines:children(inlines,None,std::mem::take(&mut inline_ids))?},
 Kind::Paragraph=>MdBlock::Paragraph{inlines:children(inlines,None,std::mem::take(&mut inline_ids))?},
 Kind::List=>MdBlock::List{ordered:ordered.take().ok_or("CommonMark list ordered flag is missing")?,start:start.take(),tight:tight.take().ok_or("CommonMark list tight flag is missing")?,items:std::mem::take(&mut items).into_iter().map(|item|children(&mut values,Some(index),item.blocks)).collect::<Result<Vec<_>,_>>()?},
 Kind::CodeBlock=>MdBlock::CodeBlock{info:info.take(),literal:literal.take().ok_or("CommonMark code literal is missing")?},
 Kind::BlockQuote=>MdBlock::BlockQuote{blocks:children(&mut values,Some(index),std::mem::take(&mut block_ids))?},
 Kind::ThematicBreak=>MdBlock::ThematicBreak,Kind::HtmlBlock=>MdBlock::HtmlBlock{raw:raw.take().ok_or("CommonMark HTML block raw text is missing")?}};
 if level.is_some()||!inline_ids.is_empty()||ordered.is_some()||start.is_some()||tight.is_some()||!items.is_empty()||!block_ids.is_empty()||info.is_some()||literal.is_some()||raw.is_some(){return Err("CommonMark block contains unrelated variant fields".into())}values[index]=Some(block)}Ok(values)}

pub(super) fn project_controlled<'a>(value:&'a MdBlock,pending:&mut std::collections::VecDeque<&'a MdBlock>,next:&mut u64,inline_pending:&mut std::collections::VecDeque<&'a MdInline>,next_inline:&mut u64,control:&mut NativeEncodeControl<'_>)->Result<Block,ValueError>{
 let mut row=Block{kind:Kind::ThematicBreak,level:None,inlines:Vec::new(),ordered:None,start:None,tight:None,items:Vec::new(),blocks:Vec::new(),info:None,literal:None,raw:None};match value{
 MdBlock::Heading{level,inlines}=>{row.kind=Kind::Heading;row.level=Some(*level);row.inlines=paid_indices(inlines,inline_pending,next_inline,control)?;},
 MdBlock::Paragraph{inlines}=>{row.kind=Kind::Paragraph;row.inlines=paid_indices(inlines,inline_pending,next_inline,control)?;},
 MdBlock::List{ordered,start,tight,items}=>{row.kind=Kind::List;row.ordered=Some(*ordered);row.start=*start;row.tight=Some(*tight);row.items=control.allocate_vec(items.len())?;control.scoped_stage(|control|{control.begin_stage(items.len())?;for blocks in items{row.items.push(ListItem{blocks:paid_indices(blocks,pending,next,control)?});control.step()?;}Ok::<_,ValueError>(())})?;},
 MdBlock::CodeBlock{info,literal}=>{row.kind=Kind::CodeBlock;row.info=paid_option(info,control)?;row.literal=Some(control.copy_text(literal)?);},
 MdBlock::BlockQuote{blocks}=>{row.kind=Kind::BlockQuote;row.blocks=paid_indices(blocks,pending,next,control)?;},
 MdBlock::ThematicBreak=>{},MdBlock::HtmlBlock{raw}=>{row.kind=Kind::HtmlBlock;row.raw=Some(control.copy_text(raw)?);}}
 Ok(row)
}
pub(super) fn reconstruct_controlled(rows:Vec<Block>,inlines:&mut[Option<MdInline>],control:&mut NativeDecodeControl<'_>)->Result<OwnedSlots<MdBlock>,ValueError>{
 let mut values=OwnedSlots(control.allocate_vec(rows.len())?);values.0.resize_with(rows.len(),||None);control.begin_stage(rows.len())?;
 for(index,row)in rows.into_iter().enumerate().rev(){let Block{kind,mut level,inlines:inline_ids,mut ordered,mut start,mut tight,items,blocks:block_ids,mut info,mut literal,mut raw}=row;let(mut inline_ids,mut items,mut block_ids)=(inline_ids,items,block_ids);
 let fields_allowed=match kind{
 Kind::Heading=>ordered.is_none()&&start.is_none()&&tight.is_none()&&items.is_empty()&&block_ids.is_empty()&&info.is_none()&&literal.is_none()&&raw.is_none(),
 Kind::Paragraph=>level.is_none()&&ordered.is_none()&&start.is_none()&&tight.is_none()&&items.is_empty()&&block_ids.is_empty()&&info.is_none()&&literal.is_none()&&raw.is_none(),
 Kind::List=>level.is_none()&&inline_ids.is_empty()&&block_ids.is_empty()&&info.is_none()&&literal.is_none()&&raw.is_none(),
 Kind::CodeBlock=>level.is_none()&&inline_ids.is_empty()&&ordered.is_none()&&start.is_none()&&tight.is_none()&&items.is_empty()&&block_ids.is_empty()&&raw.is_none(),
 Kind::BlockQuote=>level.is_none()&&inline_ids.is_empty()&&ordered.is_none()&&start.is_none()&&tight.is_none()&&items.is_empty()&&info.is_none()&&literal.is_none()&&raw.is_none(),
 Kind::ThematicBreak=>level.is_none()&&inline_ids.is_empty()&&ordered.is_none()&&start.is_none()&&tight.is_none()&&items.is_empty()&&block_ids.is_empty()&&info.is_none()&&literal.is_none()&&raw.is_none(),
 Kind::HtmlBlock=>level.is_none()&&inline_ids.is_empty()&&ordered.is_none()&&start.is_none()&&tight.is_none()&&items.is_empty()&&block_ids.is_empty()&&info.is_none()&&literal.is_none()};if !fields_allowed{return Err(invalid("CommonMark block contains unrelated variant fields"))}
 let block=match kind{
 Kind::Heading=>{let level=level.take().ok_or_else(||invalid("CommonMark heading level is missing"))?;MdBlock::Heading{level,inlines:paid_children(inlines,None,std::mem::take(&mut inline_ids),control)?}},
 Kind::Paragraph=>MdBlock::Paragraph{inlines:paid_children(inlines,None,std::mem::take(&mut inline_ids),control)?},
 Kind::List=>{let ordered=ordered.take().ok_or_else(||invalid("CommonMark list ordered flag is missing"))?;let tight=tight.take().ok_or_else(||invalid("CommonMark list tight flag is missing"))?;let mut owned=OwnedItems(control.allocate_vec(items.len())?);control.scoped_stage(|control|{control.begin_stage(items.len())?;for item in std::mem::take(&mut items){owned.0.push(paid_children(&mut values.0,Some(index),item.blocks,control)?);control.step()?;}Ok::<_,ValueError>(())})?;MdBlock::List{ordered,start:start.take(),tight,items:std::mem::take(&mut owned.0)}},
 Kind::CodeBlock=>MdBlock::CodeBlock{info:info.take(),literal:literal.take().ok_or_else(||invalid("CommonMark code literal is missing"))?},
 Kind::BlockQuote=>MdBlock::BlockQuote{blocks:paid_children(&mut values.0,Some(index),std::mem::take(&mut block_ids),control)?},
 Kind::ThematicBreak=>MdBlock::ThematicBreak,Kind::HtmlBlock=>MdBlock::HtmlBlock{raw:raw.take().ok_or_else(||invalid("CommonMark HTML block raw text is missing"))?}};
 values.0[index]=Some(block);control.step()?;
 }Ok(values)
}
