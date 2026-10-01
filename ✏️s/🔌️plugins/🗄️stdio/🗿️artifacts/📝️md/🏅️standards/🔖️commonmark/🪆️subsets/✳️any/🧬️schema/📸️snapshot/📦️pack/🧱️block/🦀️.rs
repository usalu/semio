//! 🧱️ Explicit seven-kind CommonMark block records and ordered list-item ownership.
use super::*;
#[derive(dsl::DslScalar)]
enum Kind{Heading,Paragraph,List,CodeBlock,BlockQuote,ThematicBreak,HtmlBlock}
#[derive(dsl::DslRecord)]
struct ListItem{blocks:Vec<u64>}
#[derive(dsl::DslRecord)]
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
