//! 📝️ Handcrafted CommonMark block, inline and ordered ownership entities.
use super::*;
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
use std::collections::{BTreeMap,BTreeSet};
use super::owned_pack::{OwnedNodes,RetireNode};
use semio_framework_os_kernel::{sqlite_snapshot::{artifact::{NativeEncodingBound,Cell,Projection,reconstruct_text},validate_sqlite_database_schema,SnapshotEncoding,SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase},ArtifactSqliteSnapshot};

#[derive(Clone,Copy)]
enum BlockOwner{Document,Quote(i64),Item(i64)}
#[derive(Clone,Copy)]
enum InlineOwner{Block(i64),Emphasis(i64),Strong(i64),Link(i64)}
enum WriteFrame<'a>{Blocks(&'a [MdBlock],usize,BlockOwner),Inlines(&'a [MdInline],usize,InlineOwner),Items(&'a [Vec<MdBlock>],usize,i64)}

fn ordinal(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"Markdown ordinal exceeds SQLite INTEGER"))}
fn checkpoint(control:&mut SqliteSnapshotControl<'_>,position:usize,total:usize)->Result<(),ValueError>{if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,total)?;}Ok(())}
fn optional_text(value:&Option<String>)->Cell<'_>{value.as_deref().map_or(Cell::Null,Cell::Text)}
fn block_kind(block:&MdBlock)->&'static str{match block{MdBlock::Heading{..}=>"heading",MdBlock::Paragraph{..}=>"paragraph",MdBlock::List{..}=>"list",MdBlock::CodeBlock{..}=>"codeBlock",MdBlock::BlockQuote{..}=>"blockQuote",MdBlock::ThematicBreak=>"thematicBreak",MdBlock::HtmlBlock{..}=>"htmlBlock"}}
fn inline_kind(inline:&MdInline)->&'static str{match inline{MdInline::Text{..}=>"text",MdInline::Emphasis{..}=>"emphasis",MdInline::Strong{..}=>"strong",MdInline::Code{..}=>"code",MdInline::Link{..}=>"link",MdInline::Image{..}=>"image",MdInline::SoftBreak=>"softBreak",MdInline::HardBreak=>"hardBreak",MdInline::HtmlInline{..}=>"htmlInline"}}
fn block_relationship(out:&mut Projection<'_,'_>,owner:BlockOwner,id:i64,index:usize)->Result<(),ValueError>{let(table,parent)=match owner{BlockOwner::Document=>("md_document_block",1),BlockOwner::Quote(id)=>("md_quote_block",id),BlockOwner::Item(id)=>("md_list_item_block",id)};out.insert(table,&[Cell::Integer(parent),Cell::Integer(ordinal(index)?),Cell::Integer(id)])?;Ok(())}
fn inline_relationship(out:&mut Projection<'_,'_>,owner:InlineOwner,id:i64,index:usize)->Result<(),ValueError>{let(table,parent)=match owner{InlineOwner::Block(id)=>("md_block_inline",id),InlineOwner::Emphasis(id)=>("md_emphasis_inline",id),InlineOwner::Strong(id)=>("md_strong_inline",id),InlineOwner::Link(id)=>("md_link_text",id)};out.insert(table,&[Cell::Integer(parent),Cell::Integer(ordinal(index)?),Cell::Integer(id)])?;Ok(())}

fn project(snapshot:&MdSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
 let mut out=Projection::new(<MdSnapshot as ArtifactSqliteSnapshot>::SQLITE_SCHEMA,control)?;out.insert("md_document",&[Cell::Text(&snapshot.schema)])?;let mut frames=vec![WriteFrame::Blocks(&snapshot.blocks,0,BlockOwner::Document)];let mut steps=0;
 while let Some(frame)=frames.pop(){if steps%256==0{out.checkpoint()?;}steps+=1;if frames.len()>out.limits().max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Markdown traversal exceeds row limit"));}match frame{
  WriteFrame::Blocks(blocks,index,owner)=>{let Some(block)=blocks.get(index)else{continue};frames.push(WriteFrame::Blocks(blocks,index+1,owner));let id=out.insert("md_block",&[Cell::Text(block_kind(block))])?;block_relationship(&mut out,owner,id,index)?;match block{
   MdBlock::Heading{level,inlines}=>{out.insert_key("md_heading",id,&[Cell::Integer((*level).into())])?;frames.push(WriteFrame::Inlines(inlines,0,InlineOwner::Block(id)));},
   MdBlock::Paragraph{inlines}=>{out.insert_key("md_paragraph",id,&[])?;frames.push(WriteFrame::Inlines(inlines,0,InlineOwner::Block(id)));},
   MdBlock::List{ordered,start,tight,items}=>{out.insert_key("md_list",id,&[Cell::Integer(i64::from(*ordered)),start.map_or(Cell::Null,|value|Cell::Integer(value.into())),Cell::Integer(i64::from(*tight))])?;frames.push(WriteFrame::Items(items,0,id));},
   MdBlock::CodeBlock{info,literal}=>{out.insert_key("md_code_block",id,&[optional_text(info),Cell::Text(literal)])?;},
   MdBlock::BlockQuote{blocks}=>{out.insert_key("md_block_quote",id,&[])?;frames.push(WriteFrame::Blocks(blocks,0,BlockOwner::Quote(id)));},
   MdBlock::ThematicBreak=>{out.insert_key("md_thematic_break",id,&[])?;},
   MdBlock::HtmlBlock{raw}=>{out.insert_key("md_html_block",id,&[Cell::Text(raw)])?;},
  }},
  WriteFrame::Items(items,index,list)=>{let Some(blocks)=items.get(index)else{continue};frames.push(WriteFrame::Items(items,index+1,list));let id=out.insert("md_list_item",&[Cell::Integer(list),Cell::Integer(ordinal(index)?)])?;frames.push(WriteFrame::Blocks(blocks,0,BlockOwner::Item(id)));},
  WriteFrame::Inlines(inlines,index,owner)=>{let Some(inline)=inlines.get(index)else{continue};frames.push(WriteFrame::Inlines(inlines,index+1,owner));let id=out.insert("md_inline",&[Cell::Text(inline_kind(inline))])?;inline_relationship(&mut out,owner,id,index)?;match inline{
   MdInline::Text{text}=>{out.insert_key("md_text",id,&[Cell::Text(text)])?;},
   MdInline::Emphasis{inlines}=>{out.insert_key("md_emphasis",id,&[])?;frames.push(WriteFrame::Inlines(inlines,0,InlineOwner::Emphasis(id)));},
   MdInline::Strong{inlines}=>{out.insert_key("md_strong",id,&[])?;frames.push(WriteFrame::Inlines(inlines,0,InlineOwner::Strong(id)));},
   MdInline::Code{literal}=>{out.insert_key("md_code_span",id,&[Cell::Text(literal)])?;},
   MdInline::Link{text,url,title}=>{out.insert_key("md_link",id,&[Cell::Text(url),optional_text(title)])?;frames.push(WriteFrame::Inlines(text,0,InlineOwner::Link(id)));},
   MdInline::Image{alt,url,title}=>{out.insert_key("md_image",id,&[Cell::Text(alt),Cell::Text(url),optional_text(title)])?;},
   MdInline::SoftBreak=>{out.insert_key("md_soft_break",id,&[])?;},
   MdInline::HardBreak=>{out.insert_key("md_hard_break",id,&[])?;},
   MdInline::HtmlInline{raw}=>{out.insert_key("md_html_inline",id,&[Cell::Text(raw)])?;},
  }},
 }}out.finish()
}

type Entities<'a>=BTreeMap<i64,&'a SqliteRow>;
type Children=BTreeMap<i64,Vec<i64>>;
fn entities<'a>(database:&'a SqliteDatabase,table:&str,columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Entities<'a>,ValueError>{let rows=&database.table(table)?.rows;let mut entities=BTreeMap::new();for(position,row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;if row.values.len()!=columns||row.rowid<=0||row.integer(0)?!=row.rowid||entities.insert(row.rowid,row).is_some(){return Err(invalid(format!("{table} requires unique positive aliased identities and exact columns")));}}Ok(entities)}
fn select_kind<'a>(owners:&Entities<'a>,kind:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Entities<'a>,ValueError>{let mut selected=BTreeMap::new();for(position,(&id,&row))in owners.iter().enumerate(){checkpoint(control,position,owners.len())?;if row.text(1)?==kind{selected.insert(id,row);}}Ok(selected)}
fn subtypes<'a>(database:&'a SqliteDatabase,owners:&Entities<'a>,tables:&[(&str,&str,usize)],control:&mut SqliteSnapshotControl<'_>)->Result<Entities<'a>,ValueError>{let mut bodies=BTreeMap::new();for&(table,kind,columns)in tables{for(position,(&id,&row))in entities(database,table,columns,control)?.iter().enumerate(){checkpoint(control,position,owners.len())?;if owners.get(&id).ok_or_else(||invalid("Markdown subtype has an unknown entity"))?.text(1)?!=kind||bodies.insert(id,row).is_some(){return Err(invalid("Markdown entity requires exactly its declared subtype"));}}}if bodies.len()!=owners.len(){return Err(invalid("Markdown entity is missing its declared subtype"));}Ok(bodies)}
fn relationships(database:&SqliteDatabase,table:&str,parents:&Entities<'_>,targets:Option<&Entities<'_>>,owned:&mut BTreeSet<i64>,control:&mut SqliteSnapshotControl<'_>)->Result<Children,ValueError>{
 let rows=entities(database,table,if targets.is_some(){4}else{3},control)?;let mut groups=BTreeMap::<i64,Vec<&SqliteRow>>::new();for(position,row)in rows.values().enumerate(){checkpoint(control,position,rows.len())?;let parent=row.integer(1)?;let target=if let Some(targets)=targets{let target=row.integer(3)?;if !targets.contains_key(&target){return Err(invalid("Markdown ownership has an unknown child"));}target}else{row.rowid};if !parents.contains_key(&parent)||!owned.insert(target){return Err(invalid("Markdown entity must have one known owner"));}groups.entry(parent).or_default().push(row);}
 let mut result=BTreeMap::new();for(parent,rows)in groups{let mut ordered=vec![None;rows.len()];for(position,row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;let ordinal=usize::try_from(row.integer(2)?).map_err(|error|invalid(error.to_string()))?;let slot=ordered.get_mut(ordinal).ok_or_else(||invalid("Markdown child ordinals must be dense"))?;if slot.replace(if targets.is_some(){row.integer(3)?}else{row.rowid}).is_some(){return Err(invalid("Markdown child ordinals must be unique"));}}let mut children=Vec::new();for(position,id)in ordered.into_iter().enumerate(){checkpoint(control,position,rows.len())?;children.push(id.ok_or_else(||invalid("Markdown child ordinals must be dense"))?);}result.insert(parent,children);}Ok(result)
}

struct Reader<'a>{document:&'a SqliteRow,blocks:Entities<'a>,inlines:Entities<'a>,block_bodies:Entities<'a>,inline_bodies:Entities<'a>,roots:Vec<i64>,quote_blocks:Children,list_items:Children,item_blocks:Children,block_inlines:Children,emphasis_inlines:Children,strong_inlines:Children,link_text:Children}
impl<'a> Reader<'a>{
 fn new(database:&'a SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  let documents=entities(database,"md_document",2,control)?;if documents.len()!=1{return Err(invalid("Markdown requires one document"));}let (&document_id,&document)=documents.first_key_value().ok_or_else(||invalid("Markdown document is missing"))?;let blocks=entities(database,"md_block",2,control)?;let inlines=entities(database,"md_inline",2,control)?;
  let block_bodies=subtypes(database,&blocks,&[("md_heading","heading",2),("md_paragraph","paragraph",1),("md_list","list",4),("md_code_block","codeBlock",3),("md_block_quote","blockQuote",1),("md_thematic_break","thematicBreak",1),("md_html_block","htmlBlock",2)],control)?;
  let inline_bodies=subtypes(database,&inlines,&[("md_text","text",2),("md_emphasis","emphasis",1),("md_strong","strong",1),("md_code_span","code",2),("md_link","link",3),("md_image","image",4),("md_soft_break","softBreak",1),("md_hard_break","hardBreak",1),("md_html_inline","htmlInline",2)],control)?;
  let lists=select_kind(&blocks,"list",control)?;let quotes=select_kind(&blocks,"blockQuote",control)?;let items=entities(database,"md_list_item",3,control)?;let mut owned_blocks=BTreeSet::new();let mut owned_items=BTreeSet::new();let roots=relationships(database,"md_document_block",&documents,Some(&blocks),&mut owned_blocks,control)?.remove(&document_id).unwrap_or_default();let quote_blocks=relationships(database,"md_quote_block",&quotes,Some(&blocks),&mut owned_blocks,control)?;let list_items=relationships(database,"md_list_item",&lists,None,&mut owned_items,control)?;let item_blocks=relationships(database,"md_list_item_block",&items,Some(&blocks),&mut owned_blocks,control)?;if owned_blocks.len()!=blocks.len(){return Err(invalid("Markdown block is missing its unique owner"));}
  let mut inline_blocks=select_kind(&blocks,"heading",control)?;for(position,(&id,&row))in select_kind(&blocks,"paragraph",control)?.iter().enumerate(){checkpoint(control,position,blocks.len())?;inline_blocks.insert(id,row);}let emphasis=select_kind(&inlines,"emphasis",control)?;let strong=select_kind(&inlines,"strong",control)?;let links=select_kind(&inlines,"link",control)?;let mut owned_inlines=BTreeSet::new();let block_inlines=relationships(database,"md_block_inline",&inline_blocks,Some(&inlines),&mut owned_inlines,control)?;let emphasis_inlines=relationships(database,"md_emphasis_inline",&emphasis,Some(&inlines),&mut owned_inlines,control)?;let strong_inlines=relationships(database,"md_strong_inline",&strong,Some(&inlines),&mut owned_inlines,control)?;let link_text=relationships(database,"md_link_text",&links,Some(&inlines),&mut owned_inlines,control)?;if owned_inlines.len()!=inlines.len(){return Err(invalid("Markdown inline is missing its unique owner"));}
  Ok(Self{document,blocks,inlines,block_bodies,inline_bodies,roots,quote_blocks,list_items,item_blocks,block_inlines,emphasis_inlines,strong_inlines,link_text})
 }
}
fn children(groups:&Children,id:i64)->&[i64]{groups.get(&id).map(Vec::as_slice).unwrap_or_default()}
fn boolean(row:&SqliteRow,column:usize)->Result<bool,ValueError>{match row.integer(column)?{0=>Ok(false),1=>Ok(true),_=>Err(invalid("Markdown flag must be boolean"))}}
fn start(row:&SqliteRow)->Result<Option<u32>,ValueError>{if matches!(row.values.get(2),Some(SqliteValue::Null)){Ok(None)}else{Ok(Some(u32::try_from(row.integer(2)?).map_err(|error|invalid(error.to_string()))?))}}
fn text_option(row:&SqliteRow,column:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Option<String>,ValueError>{if matches!(row.values.get(column),Some(SqliteValue::Null)){Ok(None)}else{Ok(Some(reconstruct_text(control,row.text(column)?)?))}}
fn take_tail<T:RetireNode>(values:&mut Vec<T>,start:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<T>,ValueError>{let total=values.len().checked_sub(start).ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Markdown reconstruction frame is invalid"))?;let mut result=OwnedNodes(Vec::new());result.0.try_reserve_exact(total).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Markdown child storage allocation failed"))?;while values.len()>start{result.0.push(values.pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"Markdown child storage is missing"))?);checkpoint(control,result.0.len()-1,total)?;}result.0.reverse();Ok(std::mem::take(&mut result.0))}
enum ReadTask<'a>{Blocks(&'a [i64],usize),Inlines(&'a [i64],usize),Items(&'a [i64],usize),Block(i64),Inline(i64),FinishBlock(i64,usize),FinishInline(i64,usize),FinishItem(usize)}

fn reconstruct(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<MdSnapshot,ValueError>{
 let reader=Reader::new(database,control)?;let mut tasks=vec![ReadTask::Blocks(&reader.roots,0)];let mut blocks=OwnedNodes(Vec::new());let mut inlines=OwnedNodes(Vec::new());let mut items=OwnedNodes(Vec::new());let mut seen_blocks=BTreeSet::new();let mut seen_inlines=BTreeSet::new();let mut steps=0;
 while let Some(task)=tasks.pop(){checkpoint(control,steps,0)?;steps+=1;match task{
  ReadTask::Blocks(ids,index)=>{if let Some(&id)=ids.get(index){tasks.push(ReadTask::Blocks(ids,index+1));tasks.push(ReadTask::Block(id));}},
  ReadTask::Inlines(ids,index)=>{if let Some(&id)=ids.get(index){tasks.push(ReadTask::Inlines(ids,index+1));tasks.push(ReadTask::Inline(id));}},
  ReadTask::Items(ids,index)=>{if let Some(&id)=ids.get(index){tasks.push(ReadTask::Items(ids,index+1));tasks.push(ReadTask::FinishItem(blocks.len()));tasks.push(ReadTask::Blocks(children(&reader.item_blocks,id),0));}},
  ReadTask::FinishItem(start)=>{items.push(take_tail(&mut blocks,start,control)?);},
  ReadTask::Block(id)=>{if !seen_blocks.insert(id){return Err(invalid("Markdown block graph contains a cycle or repeated entity"));}let row=reader.block_bodies[&id];match reader.blocks[&id].text(1)?{
   "heading"|"paragraph"=>{tasks.push(ReadTask::FinishBlock(id,inlines.len()));tasks.push(ReadTask::Inlines(children(&reader.block_inlines,id),0));},
   "list"=>{tasks.push(ReadTask::FinishBlock(id,items.len()));tasks.push(ReadTask::Items(children(&reader.list_items,id),0));},
   "blockQuote"=>{tasks.push(ReadTask::FinishBlock(id,blocks.len()));tasks.push(ReadTask::Blocks(children(&reader.quote_blocks,id),0));},
   "codeBlock"=>blocks.push(MdBlock::CodeBlock{info:text_option(row,1,control)?,literal:reconstruct_text(control,row.text(2)?)?}),
   "thematicBreak"=>blocks.push(MdBlock::ThematicBreak),
   "htmlBlock"=>blocks.push(MdBlock::HtmlBlock{raw:reconstruct_text(control,row.text(1)?)?}),
   _=>return Err(invalid("Markdown block kind is unknown")),
  }},
  ReadTask::FinishBlock(id,start_index)=>{let row=reader.block_bodies[&id];let block=match reader.blocks[&id].text(1)?{
   "heading"=>MdBlock::Heading{level:u8::try_from(row.integer(1)?).map_err(|error|invalid(error.to_string()))?,inlines:take_tail(&mut inlines,start_index,control)?},
   "paragraph"=>MdBlock::Paragraph{inlines:take_tail(&mut inlines,start_index,control)?},
   "list"=>MdBlock::List{ordered:boolean(row,1)?,start:start(row)?,tight:boolean(row,3)?,items:take_tail(&mut items,start_index,control)?},
   "blockQuote"=>MdBlock::BlockQuote{blocks:take_tail(&mut blocks,start_index,control)?},
   _=>return Err(invalid("Markdown block completion kind is invalid")),
  };blocks.push(block);},
  ReadTask::Inline(id)=>{if !seen_inlines.insert(id){return Err(invalid("Markdown inline graph contains a cycle or repeated entity"));}let row=reader.inline_bodies[&id];match reader.inlines[&id].text(1)?{
   "emphasis"=>{tasks.push(ReadTask::FinishInline(id,inlines.len()));tasks.push(ReadTask::Inlines(children(&reader.emphasis_inlines,id),0));},
   "strong"=>{tasks.push(ReadTask::FinishInline(id,inlines.len()));tasks.push(ReadTask::Inlines(children(&reader.strong_inlines,id),0));},
   "link"=>{tasks.push(ReadTask::FinishInline(id,inlines.len()));tasks.push(ReadTask::Inlines(children(&reader.link_text,id),0));},
   "text"=>inlines.push(MdInline::Text{text:reconstruct_text(control,row.text(1)?)?}),
   "code"=>inlines.push(MdInline::Code{literal:reconstruct_text(control,row.text(1)?)?}),
   "image"=>inlines.push(MdInline::Image{alt:reconstruct_text(control,row.text(1)?)?,url:reconstruct_text(control,row.text(2)?)?,title:text_option(row,3,control)?}),
   "softBreak"=>inlines.push(MdInline::SoftBreak),"hardBreak"=>inlines.push(MdInline::HardBreak),
   "htmlInline"=>inlines.push(MdInline::HtmlInline{raw:reconstruct_text(control,row.text(1)?)?}),
   _=>return Err(invalid("Markdown inline kind is unknown")),
  }},
  ReadTask::FinishInline(id,start)=>{let row=reader.inline_bodies[&id];let mut children=OwnedNodes(take_tail(&mut inlines,start,control)?);let url=if reader.inlines[&id].text(1)?=="link"{Some(reconstruct_text(control,row.text(1)?)?)}else{None};let title=if url.is_some(){text_option(row,2,control)?}else{None};let inline=match reader.inlines[&id].text(1)?{"emphasis"=>MdInline::Emphasis{inlines:std::mem::take(&mut children.0)},"strong"=>MdInline::Strong{inlines:std::mem::take(&mut children.0)},"link"=>MdInline::Link{text:std::mem::take(&mut children.0),url:url.ok_or_else(||invalid("Markdown link URL is missing"))?,title},_=>return Err(invalid("Markdown inline completion kind is invalid"))};inlines.push(inline);},
 }}
 if seen_blocks.len()!=reader.blocks.len()||seen_inlines.len()!=reader.inlines.len()||!inlines.is_empty()||!items.is_empty(){return Err(invalid("Markdown tree contains unreachable entities"));}Ok(MdSnapshot{schema:reconstruct_text(control,reader.document.text(1)?)?,blocks:std::mem::take(&mut blocks.0)})
}

impl ArtifactSqliteSnapshot for MdSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn preflight_sqlite_snapshot_encoding(&self, _encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {
  let result=(||->Result<(),ValueError>{
  let mut bound = NativeEncodingBound::new(control)?;
  bound.add(32768)?;
  bound.repeated(self.schema.len(), 24)?;
  let mut frames = vec![WriteFrame::Blocks(&self.blocks, 0, BlockOwner::Document)];
  while let Some(frame) = frames.pop() {
   match frame {
    WriteFrame::Blocks(blocks, index, _) if index < blocks.len() => {
     bound.add(4096)?;
     frames.push(WriteFrame::Blocks(blocks, index + 1, BlockOwner::Document));
     match &blocks[index] {
      MdBlock::Heading { inlines, .. } | MdBlock::Paragraph { inlines } => frames.push(WriteFrame::Inlines(inlines, 0, InlineOwner::Block(0))),
      MdBlock::List { items, .. } => frames.push(WriteFrame::Items(items, 0, 0)),
      MdBlock::CodeBlock { info, literal } => { if let Some(info) = info { bound.repeated(info.len(), 24)?; } bound.repeated(literal.len(), 24)?; }
      MdBlock::BlockQuote { blocks } => frames.push(WriteFrame::Blocks(blocks, 0, BlockOwner::Document)),
      MdBlock::HtmlBlock { raw } => bound.repeated(raw.len(), 24)?,
      MdBlock::ThematicBreak => {},
     }
    }
    WriteFrame::Inlines(inlines, index, _) if index < inlines.len() => {
     bound.add(4096)?;
     frames.push(WriteFrame::Inlines(inlines, index + 1, InlineOwner::Block(0)));
     match &inlines[index] {
      MdInline::Text { text } => bound.repeated(text.len(), 24)?,
      MdInline::Emphasis { inlines } | MdInline::Strong { inlines } => frames.push(WriteFrame::Inlines(inlines, 0, InlineOwner::Block(0))),
      MdInline::Code { literal } => bound.repeated(literal.len(), 24)?,
      MdInline::Link { text, url, title } => { bound.repeated(url.len(), 24)?; if let Some(title) = title { bound.repeated(title.len(), 24)?; } frames.push(WriteFrame::Inlines(text, 0, InlineOwner::Block(0))); }
      MdInline::Image { alt, url, title } => { bound.repeated(alt.len(), 24)?; bound.repeated(url.len(), 24)?; if let Some(title) = title { bound.repeated(title.len(), 24)?; } }
      MdInline::HtmlInline { raw } => bound.repeated(raw.len(), 24)?,
      MdInline::SoftBreak | MdInline::HardBreak => {},
     }
    }
    WriteFrame::Items(items, index, _) if index < items.len() => {
     bound.add(2048)?;
     frames.push(WriteFrame::Items(items, index + 1, 0));
     frames.push(WriteFrame::Blocks(&items[index], 0, BlockOwner::Document));
    }
    WriteFrame::Blocks(_, _, _) | WriteFrame::Inlines(_, _, _) | WriteFrame::Items(_, _, _) => {},
   }
  }
  bound.finish()
  })();result
 }

 fn validate_sqlite_snapshot_subset(&self,dialect:&store::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{validate_owned(self,dialect,database,control)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{decode_hook(payload,control)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{encode_hook(self,encoding,control)}
 fn retire_sqlite_snapshot(self){retire_hook(self)}
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;let mut snapshot=OwnedSnapshot(Some(reconstruct(database,control)?));control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;snapshot.0.take().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"CommonMark completed snapshot is missing"))}
}

struct OwnedSnapshot(Option<MdSnapshot>);
impl Drop for OwnedSnapshot{fn drop(&mut self){if let Some(value)=self.0.take(){super::owned_pack::retire_owned(value)}}}

fn validate_owned(snapshot:&MdSnapshot,dialect:&store::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{
 let result=(||->Result<(),ValueError>{
 control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.md"||dialect.standard!="commonmark"||dialect.subset!="*"{return Err(ValueError::new(ValueRefusalKind::UnsupportedOwner,format!("CommonMark does not own semantic subset {}",dialect.to_coordinate())))}
 let restored=OwnedSnapshot(Some(<MdSnapshot as ArtifactSqliteSnapshot>::from_sqlite_database(database,control)?));let expected=project(snapshot,control)?;let candidate=project(restored.0.as_ref().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"CommonMark retained candidate is missing"))?,control)?;let total=expected.tables.iter().try_fold(0usize,|total,table|total.checked_add(table.rows.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"CommonMark comparison work overflow")))?;let mut completed=0;
 if expected.tables.len()!=candidate.tables.len(){return Err(invalid("CommonMark document identity disagrees with its snapshot"))}for(wanted,actual)in expected.tables.iter().zip(&candidate.tables){if wanted.name!=actual.name||wanted.rows.len()!=actual.rows.len(){return Err(invalid("CommonMark document identity disagrees with its snapshot"))}for(row,other)in wanted.rows.iter().zip(&actual.rows){if completed%256==0{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,completed,total)?;}if row.rowid!=other.rowid||row.values!=other.values{return Err(invalid("CommonMark document identity disagrees with its snapshot"))}completed+=1;}}
 control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,completed,total)?;Ok(())
 })();result.map(|()|store::io_schema::IoOutcome::clean(())).map_err(store::io_schema::IoError::from_value_error)
}

fn decode_hook(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<MdSnapshot,ValueError>{super::owned_pack::decode_owned(payload,control)}
fn encode_hook(value:&MdSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{super::owned_pack::encode_owned(value,encoding,control)}
fn retire_hook(value:MdSnapshot){super::owned_pack::retire_owned(value)}
