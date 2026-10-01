//! 📝️ Handcrafted CommonMark block, inline and ordered ownership entities.
use super::*;
use std::collections::{BTreeMap,BTreeSet};
use semio_framework_os_kernel::{sqlite_snapshot::{artifact::{NativeEncodingBound,Cell,Projection,reconstruct_text},validate_sqlite_database_schema,SnapshotEncoding,SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase},ArtifactSqliteSnapshot};

#[derive(Clone,Copy)]
enum BlockOwner{Document,Quote(i64),Item(i64)}
#[derive(Clone,Copy)]
enum InlineOwner{Block(i64),Emphasis(i64),Strong(i64),Link(i64)}
enum WriteFrame<'a>{Blocks(&'a [MdBlock],usize,BlockOwner),Inlines(&'a [MdInline],usize,InlineOwner),Items(&'a [Vec<MdBlock>],usize,i64)}

fn ordinal(value:usize)->Result<i64,String>{i64::try_from(value).map_err(|error|error.to_string())}
fn checkpoint(control:&mut SqliteSnapshotControl<'_>,position:usize,total:usize)->Result<(),String>{if position%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,total)?;}Ok(())}
fn optional_text(value:&Option<String>)->Cell<'_>{value.as_deref().map_or(Cell::Null,Cell::Text)}
fn block_kind(block:&MdBlock)->&'static str{match block{MdBlock::Heading{..}=>"heading",MdBlock::Paragraph{..}=>"paragraph",MdBlock::List{..}=>"list",MdBlock::CodeBlock{..}=>"codeBlock",MdBlock::BlockQuote{..}=>"blockQuote",MdBlock::ThematicBreak=>"thematicBreak",MdBlock::HtmlBlock{..}=>"htmlBlock"}}
fn inline_kind(inline:&MdInline)->&'static str{match inline{MdInline::Text{..}=>"text",MdInline::Emphasis{..}=>"emphasis",MdInline::Strong{..}=>"strong",MdInline::Code{..}=>"code",MdInline::Link{..}=>"link",MdInline::Image{..}=>"image",MdInline::SoftBreak=>"softBreak",MdInline::HardBreak=>"hardBreak",MdInline::HtmlInline{..}=>"htmlInline"}}
fn block_relationship(out:&mut Projection<'_,'_>,owner:BlockOwner,id:i64,index:usize)->Result<(),String>{let(table,parent)=match owner{BlockOwner::Document=>("md_document_block",1),BlockOwner::Quote(id)=>("md_quote_block",id),BlockOwner::Item(id)=>("md_list_item_block",id)};out.insert(table,&[Cell::Integer(parent),Cell::Integer(ordinal(index)?),Cell::Integer(id)])?;Ok(())}
fn inline_relationship(out:&mut Projection<'_,'_>,owner:InlineOwner,id:i64,index:usize)->Result<(),String>{let(table,parent)=match owner{InlineOwner::Block(id)=>("md_block_inline",id),InlineOwner::Emphasis(id)=>("md_emphasis_inline",id),InlineOwner::Strong(id)=>("md_strong_inline",id),InlineOwner::Link(id)=>("md_link_text",id)};out.insert(table,&[Cell::Integer(parent),Cell::Integer(ordinal(index)?),Cell::Integer(id)])?;Ok(())}

fn project(snapshot:&MdSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
 let mut out=Projection::new(<MdSnapshot as ArtifactSqliteSnapshot>::SQLITE_SCHEMA,control)?;out.insert("md_document",&[Cell::Text(&snapshot.schema)])?;let mut frames=vec![WriteFrame::Blocks(&snapshot.blocks,0,BlockOwner::Document)];let mut steps=0;
 while let Some(frame)=frames.pop(){if steps%256==0{out.checkpoint()?;}steps+=1;if frames.len()>out.limits().max_rows{return Err("Markdown traversal exceeds row limit".into());}match frame{
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
fn entities<'a>(database:&'a SqliteDatabase,table:&str,columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Entities<'a>,String>{let rows=&database.table(table)?.rows;let mut entities=BTreeMap::new();for(position,row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;if row.values.len()!=columns||row.rowid<=0||row.integer(0)?!=row.rowid||entities.insert(row.rowid,row).is_some(){return Err(format!("{table} requires unique positive aliased identities and exact columns"));}}Ok(entities)}
fn select_kind<'a>(owners:&Entities<'a>,kind:&str,control:&mut SqliteSnapshotControl<'_>)->Result<Entities<'a>,String>{let mut selected=BTreeMap::new();for(position,(&id,&row))in owners.iter().enumerate(){checkpoint(control,position,owners.len())?;if row.text(1)?==kind{selected.insert(id,row);}}Ok(selected)}
fn subtypes<'a>(database:&'a SqliteDatabase,owners:&Entities<'a>,tables:&[(&str,&str,usize)],control:&mut SqliteSnapshotControl<'_>)->Result<Entities<'a>,String>{let mut bodies=BTreeMap::new();for&(table,kind,columns)in tables{for(position,(&id,&row))in entities(database,table,columns,control)?.iter().enumerate(){checkpoint(control,position,owners.len())?;if owners.get(&id).ok_or("Markdown subtype has an unknown entity")?.text(1)?!=kind||bodies.insert(id,row).is_some(){return Err("Markdown entity requires exactly its declared subtype".into());}}}if bodies.len()!=owners.len(){return Err("Markdown entity is missing its declared subtype".into());}Ok(bodies)}
fn relationships(database:&SqliteDatabase,table:&str,parents:&Entities<'_>,targets:Option<&Entities<'_>>,owned:&mut BTreeSet<i64>,control:&mut SqliteSnapshotControl<'_>)->Result<Children,String>{
 let rows=entities(database,table,if targets.is_some(){4}else{3},control)?;let mut groups=BTreeMap::<i64,Vec<&SqliteRow>>::new();for(position,row)in rows.values().enumerate(){checkpoint(control,position,rows.len())?;let parent=row.integer(1)?;let target=if let Some(targets)=targets{let target=row.integer(3)?;if !targets.contains_key(&target){return Err("Markdown ownership has an unknown child".into());}target}else{row.rowid};if !parents.contains_key(&parent)||!owned.insert(target){return Err("Markdown entity must have one known owner".into());}groups.entry(parent).or_default().push(row);}
 let mut result=BTreeMap::new();for(parent,rows)in groups{let mut ordered=vec![None;rows.len()];for(position,row)in rows.iter().enumerate(){checkpoint(control,position,rows.len())?;let ordinal=usize::try_from(row.integer(2)?).map_err(|error|error.to_string())?;let slot=ordered.get_mut(ordinal).ok_or("Markdown child ordinals must be dense")?;if slot.replace(if targets.is_some(){row.integer(3)?}else{row.rowid}).is_some(){return Err("Markdown child ordinals must be unique".into());}}let mut children=Vec::new();for(position,id)in ordered.into_iter().enumerate(){checkpoint(control,position,rows.len())?;children.push(id.ok_or("Markdown child ordinals must be dense")?);}result.insert(parent,children);}Ok(result)
}

struct Reader<'a>{document:&'a SqliteRow,blocks:Entities<'a>,inlines:Entities<'a>,block_bodies:Entities<'a>,inline_bodies:Entities<'a>,roots:Vec<i64>,quote_blocks:Children,list_items:Children,item_blocks:Children,block_inlines:Children,emphasis_inlines:Children,strong_inlines:Children,link_text:Children}
impl<'a> Reader<'a>{
 fn new(database:&'a SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{
  let documents=entities(database,"md_document",2,control)?;if documents.len()!=1||!documents.contains_key(&1){return Err("Markdown document identity must be one".into());}let document=documents[&1];let blocks=entities(database,"md_block",2,control)?;let inlines=entities(database,"md_inline",2,control)?;
  let block_bodies=subtypes(database,&blocks,&[("md_heading","heading",2),("md_paragraph","paragraph",1),("md_list","list",4),("md_code_block","codeBlock",3),("md_block_quote","blockQuote",1),("md_thematic_break","thematicBreak",1),("md_html_block","htmlBlock",2)],control)?;
  let inline_bodies=subtypes(database,&inlines,&[("md_text","text",2),("md_emphasis","emphasis",1),("md_strong","strong",1),("md_code_span","code",2),("md_link","link",3),("md_image","image",4),("md_soft_break","softBreak",1),("md_hard_break","hardBreak",1),("md_html_inline","htmlInline",2)],control)?;
  let lists=select_kind(&blocks,"list",control)?;let quotes=select_kind(&blocks,"blockQuote",control)?;let items=entities(database,"md_list_item",3,control)?;let mut owned_blocks=BTreeSet::new();let mut owned_items=BTreeSet::new();let roots=relationships(database,"md_document_block",&documents,Some(&blocks),&mut owned_blocks,control)?.remove(&1).unwrap_or_default();let quote_blocks=relationships(database,"md_quote_block",&quotes,Some(&blocks),&mut owned_blocks,control)?;let list_items=relationships(database,"md_list_item",&lists,None,&mut owned_items,control)?;let item_blocks=relationships(database,"md_list_item_block",&items,Some(&blocks),&mut owned_blocks,control)?;if owned_blocks.len()!=blocks.len(){return Err("Markdown block is missing its unique owner".into());}
  let mut inline_blocks=select_kind(&blocks,"heading",control)?;for(position,(&id,&row))in select_kind(&blocks,"paragraph",control)?.iter().enumerate(){checkpoint(control,position,blocks.len())?;inline_blocks.insert(id,row);}let emphasis=select_kind(&inlines,"emphasis",control)?;let strong=select_kind(&inlines,"strong",control)?;let links=select_kind(&inlines,"link",control)?;let mut owned_inlines=BTreeSet::new();let block_inlines=relationships(database,"md_block_inline",&inline_blocks,Some(&inlines),&mut owned_inlines,control)?;let emphasis_inlines=relationships(database,"md_emphasis_inline",&emphasis,Some(&inlines),&mut owned_inlines,control)?;let strong_inlines=relationships(database,"md_strong_inline",&strong,Some(&inlines),&mut owned_inlines,control)?;let link_text=relationships(database,"md_link_text",&links,Some(&inlines),&mut owned_inlines,control)?;if owned_inlines.len()!=inlines.len(){return Err("Markdown inline is missing its unique owner".into());}
  Ok(Self{document,blocks,inlines,block_bodies,inline_bodies,roots,quote_blocks,list_items,item_blocks,block_inlines,emphasis_inlines,strong_inlines,link_text})
 }
}
fn children(groups:&Children,id:i64)->&[i64]{groups.get(&id).map(Vec::as_slice).unwrap_or_default()}
fn boolean(row:&SqliteRow,column:usize)->Result<bool,String>{match row.integer(column)?{0=>Ok(false),1=>Ok(true),_=>Err("Markdown flag must be boolean".into())}}
fn start(row:&SqliteRow)->Result<Option<u32>,String>{if matches!(row.values.get(2),Some(SqliteValue::Null)){Ok(None)}else{Ok(Some(u32::try_from(row.integer(2)?).map_err(|error|error.to_string())?))}}
fn text_option(row:&SqliteRow,column:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Option<String>,String>{if matches!(row.values.get(column),Some(SqliteValue::Null)){Ok(None)}else{Ok(Some(reconstruct_text(control,row.text(column)?)?))}}
fn take_tail<T>(values:&mut Vec<T>,start:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<T>,String>{let total=values.len().checked_sub(start).ok_or("Markdown reconstruction frame is invalid")?;let mut result=Vec::new();for(position,value)in values.drain(start..).enumerate(){checkpoint(control,position,total)?;result.push(value);}Ok(result)}
enum ReadTask<'a>{Blocks(&'a [i64],usize),Inlines(&'a [i64],usize),Items(&'a [i64],usize),Block(i64),Inline(i64),FinishBlock(i64,usize),FinishInline(i64,usize),FinishItem(usize)}

fn reconstruct(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<MdSnapshot,String>{
 let reader=Reader::new(database,control)?;let mut tasks=vec![ReadTask::Blocks(&reader.roots,0)];let mut blocks=Vec::new();let mut inlines=Vec::new();let mut items=Vec::new();let mut seen_blocks=BTreeSet::new();let mut seen_inlines=BTreeSet::new();let mut steps=0;
 while let Some(task)=tasks.pop(){checkpoint(control,steps,0)?;steps+=1;match task{
  ReadTask::Blocks(ids,index)=>{if let Some(&id)=ids.get(index){tasks.push(ReadTask::Blocks(ids,index+1));tasks.push(ReadTask::Block(id));}},
  ReadTask::Inlines(ids,index)=>{if let Some(&id)=ids.get(index){tasks.push(ReadTask::Inlines(ids,index+1));tasks.push(ReadTask::Inline(id));}},
  ReadTask::Items(ids,index)=>{if let Some(&id)=ids.get(index){tasks.push(ReadTask::Items(ids,index+1));tasks.push(ReadTask::FinishItem(blocks.len()));tasks.push(ReadTask::Blocks(children(&reader.item_blocks,id),0));}},
  ReadTask::FinishItem(start)=>{items.push(take_tail(&mut blocks,start,control)?);},
  ReadTask::Block(id)=>{if !seen_blocks.insert(id){return Err("Markdown block graph contains a cycle or repeated entity".into());}let row=reader.block_bodies[&id];match reader.blocks[&id].text(1)?{
   "heading"|"paragraph"=>{tasks.push(ReadTask::FinishBlock(id,inlines.len()));tasks.push(ReadTask::Inlines(children(&reader.block_inlines,id),0));},
   "list"=>{tasks.push(ReadTask::FinishBlock(id,items.len()));tasks.push(ReadTask::Items(children(&reader.list_items,id),0));},
   "blockQuote"=>{tasks.push(ReadTask::FinishBlock(id,blocks.len()));tasks.push(ReadTask::Blocks(children(&reader.quote_blocks,id),0));},
   "codeBlock"=>blocks.push(MdBlock::CodeBlock{info:text_option(row,1,control)?,literal:reconstruct_text(control,row.text(2)?)?}),
   "thematicBreak"=>blocks.push(MdBlock::ThematicBreak),
   "htmlBlock"=>blocks.push(MdBlock::HtmlBlock{raw:reconstruct_text(control,row.text(1)?)?}),
   _=>return Err("Markdown block kind is unknown".into()),
  }},
  ReadTask::FinishBlock(id,start_index)=>{let row=reader.block_bodies[&id];let block=match reader.blocks[&id].text(1)?{
   "heading"=>MdBlock::Heading{level:u8::try_from(row.integer(1)?).map_err(|error|error.to_string())?,inlines:take_tail(&mut inlines,start_index,control)?},
   "paragraph"=>MdBlock::Paragraph{inlines:take_tail(&mut inlines,start_index,control)?},
   "list"=>MdBlock::List{ordered:boolean(row,1)?,start:start(row)?,tight:boolean(row,3)?,items:take_tail(&mut items,start_index,control)?},
   "blockQuote"=>MdBlock::BlockQuote{blocks:take_tail(&mut blocks,start_index,control)?},
   _=>return Err("Markdown block completion kind is invalid".into()),
  };blocks.push(block);},
  ReadTask::Inline(id)=>{if !seen_inlines.insert(id){return Err("Markdown inline graph contains a cycle or repeated entity".into());}let row=reader.inline_bodies[&id];match reader.inlines[&id].text(1)?{
   "emphasis"=>{tasks.push(ReadTask::FinishInline(id,inlines.len()));tasks.push(ReadTask::Inlines(children(&reader.emphasis_inlines,id),0));},
   "strong"=>{tasks.push(ReadTask::FinishInline(id,inlines.len()));tasks.push(ReadTask::Inlines(children(&reader.strong_inlines,id),0));},
   "link"=>{tasks.push(ReadTask::FinishInline(id,inlines.len()));tasks.push(ReadTask::Inlines(children(&reader.link_text,id),0));},
   "text"=>inlines.push(MdInline::Text{text:reconstruct_text(control,row.text(1)?)?}),
   "code"=>inlines.push(MdInline::Code{literal:reconstruct_text(control,row.text(1)?)?}),
   "image"=>inlines.push(MdInline::Image{alt:reconstruct_text(control,row.text(1)?)?,url:reconstruct_text(control,row.text(2)?)?,title:text_option(row,3,control)?}),
   "softBreak"=>inlines.push(MdInline::SoftBreak),"hardBreak"=>inlines.push(MdInline::HardBreak),
   "htmlInline"=>inlines.push(MdInline::HtmlInline{raw:reconstruct_text(control,row.text(1)?)?}),
   _=>return Err("Markdown inline kind is unknown".into()),
  }},
  ReadTask::FinishInline(id,start)=>{let row=reader.inline_bodies[&id];let children=take_tail(&mut inlines,start,control)?;let inline=match reader.inlines[&id].text(1)?{"emphasis"=>MdInline::Emphasis{inlines:children},"strong"=>MdInline::Strong{inlines:children},"link"=>MdInline::Link{text:children,url:reconstruct_text(control,row.text(1)?)?,title:text_option(row,2,control)?},_=>return Err("Markdown inline completion kind is invalid".into())};inlines.push(inline);},
 }}
 if seen_blocks.len()!=reader.blocks.len()||seen_inlines.len()!=reader.inlines.len()||!inlines.is_empty()||!items.is_empty(){return Err("Markdown tree contains unreachable entities".into());}Ok(MdSnapshot{schema:reconstruct_text(control,reader.document.text(1)?)?,blocks})
}

impl ArtifactSqliteSnapshot for MdSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn preflight_sqlite_snapshot_encoding(&self, _encoding: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), String> {
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
 }

 fn validate_sqlite_snapshot_subset(&self,dialect:&store::io_schema::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0)?;if dialect.artifact_kind!="s.stdio.md"||dialect.standard!="commonmark"||dialect.subset!="*"{return Err(format!("CommonMark does not own semantic subset {}",dialect.to_coordinate()).into());}let row=database.table("md_document")?.single_row()?;if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err("CommonMark document identity disagrees with its snapshot".to_string().into());}Ok(store::io_schema::IoOutcome::clean(()))}
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{validate_sqlite_database_schema(database,Self::SQLITE_SCHEMA,control.limits()).map_err(|error|error.to_string())?;control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;let snapshot=reconstruct(database,control)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(snapshot)}
}
