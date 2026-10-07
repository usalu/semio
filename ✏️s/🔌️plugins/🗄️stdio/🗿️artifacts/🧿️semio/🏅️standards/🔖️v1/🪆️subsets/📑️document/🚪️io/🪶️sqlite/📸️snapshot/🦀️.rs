//! 📑️ Authored document blocks, typed collections, formatting, styles and images.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow,RowIndex,reconstruct_text,reconstruct_blob};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
use crate::standards::v1::subsets::document::schema::snapshot::{SemioDocumentSnapshot,DocBlock,DocStyle,DocImage,DocRun,RunStyle,DocListItem,DocTableRow,DocTableCell};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,Reconstruction},validate_sqlite_database_schema,SqliteDatabase,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase}};
/// 📚️ Explicit table names for the actual DocBlock types shared by document and presentation.
#[derive(Clone,Copy)]
pub struct DocSqliteTables{pub collection:&'static str,pub member:&'static str,pub block:&'static str,pub paragraph:&'static str,pub heading:&'static str,pub run:&'static str,pub list:&'static str,pub list_item:&'static str,pub table:&'static str,pub table_row:&'static str,pub table_cell:&'static str,pub code:&'static str,pub quote:&'static str,pub image_block:&'static str,pub page_break:&'static str}
/// 📑️ The document's independently declared block table names.
pub const DOCUMENT_TABLES:DocSqliteTables=DocSqliteTables{collection:"semio_document_collection",member:"semio_document_member",block:"semio_document_block",paragraph:"semio_document_paragraph",heading:"semio_document_heading",run:"semio_document_run",list:"semio_document_list",list_item:"semio_document_list_item",table:"semio_document_table",table_row:"semio_document_table_row",table_cell:"semio_document_table_cell",code:"semio_document_code",quote:"semio_document_quote",image_block:"semio_document_image_block",page_break:"semio_document_page_break"};
#[path="🧮️semantic/🦀️.rs"]
pub(crate)mod semantic;
/// 🪪️ Orders a paid borrowed identity frontier without materializing native Strings.
pub(crate)fn doc_frontier<'a>(ids:impl Iterator<Item=&'a str>,count:usize,out:&mut RowWriter<'_,'_>)->Result<Vec<(&'a str,i64)>,ValueError>{
 let phase=out.phase();let mut names=out.allocate_frontier(count)?;for(ordinal,id)in ids.enumerate(){out.checkpoint()?;names.push((id,number(ordinal.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"document identity extent overflow"))?)?));}
 out.sort_frontier(&mut names,|a,b,control|semio_framework_os_kernel::sqlite_snapshot::transfer::compare_text(a.0,b.0,phase,control))?;
 for pair in names.windows(2){if out.compare_text(pair[0].0,pair[1].0)?==std::cmp::Ordering::Equal{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate document named identity"))}}Ok(names)
}
/// 🔍️ Resolves an authored reference with bounded cancellable UTF8 comparisons.
pub(crate)fn doc_identity(names:&[(&str,i64)],id:&str,out:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
 let mut lo=0;let mut hi=names.len();while lo<hi{let mid=lo+(hi-lo)/2;match out.compare_text(names[mid].0,id)?{std::cmp::Ordering::Less=>lo=mid+1,std::cmp::Ordering::Greater=>hi=mid, std::cmp::Ordering::Equal=>return Ok(names[mid].1)}}Err(ValueError::new(ValueRefusalKind::InvalidValue,"dangling document named reference"))
}
/// 🏷️ Preserves external literal references or resolves the caller's real named partition.
fn doc_reference<'a>(native:Option<&'a str>,names:Option<&[(&str,i64)]>,out:&mut RowWriter<'_,'_>)->Result<Cell<'a>,ValueError>{
 match native{None=>Ok(Cell::Null),Some(name)=>match names{Some(names)=>Ok(Cell::Integer(doc_identity(names,name,out)?)),None=>Ok(Cell::Text(name))}}
}
/// ♻️ Checks style ancestry using paid fixed-width parent marks and cancellable walks.
fn doc_style_cycles(parents:&mut[(Option<usize>,u8)],out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 for root in 0..parents.len(){let mut current=Some(root);while let Some(index)=current{out.checkpoint()?;match parents[index].1{2=>break,1=>return Err(ValueError::new(ValueRefusalKind::InvalidValue,"cyclic document style inheritance")),_=>{parents[index].1=1;current=parents[index].0;}}}
  let mut current=Some(root);while let Some(index)=current{out.checkpoint()?;if parents[index].1!=1{break}parents[index].1=2;current=parents[index].0;}
 }Ok(())
}
/// 🫳️ Visits the complete real document corpus using the actual shared RowWriter.
pub(crate)fn visit_rows(snapshot:&SemioDocumentSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 let styles=doc_frontier(snapshot.styles.iter().map(|style|style.id.as_str()),snapshot.styles.len(),out)?;let images=doc_frontier(snapshot.images.iter().map(|image|image.id.as_str()),snapshot.images.len(),out)?;let mut parents=out.allocate_frontier(snapshot.styles.len())?;
 for style in &snapshot.styles{out.checkpoint()?;let parent=style.based_on.as_deref().map(|id|doc_identity(&styles,id,out).and_then(|row|usize::try_from(row-1).map_err(|error|ValueError::new(ValueRefusalKind::InvalidValue,error.to_string())))).transpose()?;parents.push((parent,0u8));}doc_style_cycles(&mut parents,out)?;
 for(ordinal,style)in snapshot.styles.iter().enumerate(){let parent=parents[ordinal].0.map(|index|number(index+1).map(Cell::Integer)).transpose()?.unwrap_or(Cell::Null);out.insert_key("semio_document_style",number(ordinal+1)?,&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&style.id),Cell::Text(&style.name),parent])?;}
 for(ordinal,image)in snapshot.images.iter().enumerate(){out.insert("semio_document_image",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&image.id),Cell::Text(&image.mime),Cell::Blob(&image.bytes)])?;}
 let root=project_block_collection(&snapshot.blocks,DOCUMENT_TABLES,Some(&styles),Some(&images),out)?;out.insert_key("semio_document_document",1,&[Cell::Text(&snapshot.schema),Cell::Integer(root)])?;Ok(())
}
/// 🧱️ Visits the shared block tree with a paid iterative collection frontier.
pub fn project_block_collection(blocks:&[DocBlock],t:DocSqliteTables,styles:Option<&[(&str,i64)]>,images:Option<&[(&str,i64)]>,p:&mut RowWriter<'_,'_>)->Result<i64,ValueError>{
let root=p.insert_float(t.collection,&[],float_columns(t.collection))?;let mut pending=Vec::new();let mut initial=Some((root,blocks));while let Some((collection,blocks))=initial.take().or_else(||pending.pop()){p.check_rows(blocks.len().checked_add(pending.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"document traversal count overflow"))?)?;for(ordinal,block)in blocks.iter().enumerate(){let kind=match block{DocBlock::Paragraph{..}=>"paragraph",DocBlock::Heading{..}=>"heading",DocBlock::List{..}=>"list",DocBlock::Table{..}=>"table",DocBlock::Code{..}=>"code",DocBlock::Quote{..}=>"quote",DocBlock::Image{..}=>"image",DocBlock::PageBreak=>"pageBreak"};let id=p.insert_float(t.block,&[Cell::Text(kind)],float_columns(t.block))?;p.insert_float(t.member,&[Cell::Integer(collection),Cell::Integer(number(ordinal)?),Cell::Integer(id)],float_columns(t.member))?;
match block{
DocBlock::Paragraph{style_id,..}=>{let style=doc_reference(style_id.as_deref(),styles,p)?;p.insert_key_float(t.paragraph,id,&[style],float_columns(t.paragraph))?;},
DocBlock::Heading{level,style_id,..}=>{let style=doc_reference(style_id.as_deref(),styles,p)?;p.insert_key_float(t.heading,id,&[Cell::Integer(i64::from(*level)),style],float_columns(t.heading))?;},
DocBlock::List{ordered,items}=>{p.insert_key_float(t.list,id,&[Cell::Integer(i64::from(*ordered))],float_columns(t.list))?;p.check_rows(pending.len().checked_add(items.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"document list traversal overflow"))?)?;for(ordinal,item)in items.iter().enumerate(){let child=p.insert_float(t.collection,&[],float_columns(t.collection))?;p.insert_float(t.list_item,&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Integer(child)],float_columns(t.list_item))?;p.push_frontier(&mut pending,(child,item.blocks.as_slice()))?;}},
DocBlock::Table{rows}=>{p.insert_key_float(t.table,id,&[],float_columns(t.table))?;for(ordinal,row)in rows.iter().enumerate(){let row_id=p.insert_float(t.table_row,&[Cell::Integer(id),Cell::Integer(number(ordinal)?)],float_columns(t.table_row))?;p.check_rows(pending.len().checked_add(row.cells.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"document table traversal overflow"))?)?;for(ordinal,cell)in row.cells.iter().enumerate(){let child=p.insert_float(t.collection,&[],float_columns(t.collection))?;p.insert_float(t.table_cell,&[Cell::Integer(row_id),Cell::Integer(number(ordinal)?),Cell::Integer(child)],float_columns(t.table_cell))?;p.push_frontier(&mut pending,(child,cell.blocks.as_slice()))?;}}},
DocBlock::Code{language,text}=>p.insert_key_float(t.code,id,&[language.as_deref().map(Cell::Text).unwrap_or(Cell::Null),Cell::Text(text)],float_columns(t.code))?,
DocBlock::Quote{blocks}=>{let child=p.insert_float(t.collection,&[],float_columns(t.collection))?;p.insert_key_float(t.quote,id,&[Cell::Integer(child)],float_columns(t.quote))?;p.push_frontier(&mut pending,(child,blocks))?;},
DocBlock::Image{image_id,alt,width,height}=>{let image=doc_reference(Some(image_id),images,p)?;p.insert_key_float(t.image_block,id,&[image,Cell::Text(alt),width.map(Cell::Real).unwrap_or(Cell::Null),height.map(Cell::Real).unwrap_or(Cell::Null)],float_columns(t.image_block))?;}
DocBlock::PageBreak=>p.insert_key_float(t.page_break,id,&[],float_columns(t.page_break))?}
if let DocBlock::Paragraph{runs,..}|DocBlock::Heading{runs,..}=block{for(ordinal,run)in runs.iter().enumerate(){let s=&run.style;p.insert_float(t.run,&[Cell::Integer(id),Cell::Integer(number(ordinal)?),Cell::Text(&run.text),Cell::Integer(i64::from(s.bold)),Cell::Integer(i64::from(s.italic)),Cell::Integer(i64::from(s.underline)),s.size.map(Cell::Real).unwrap_or(Cell::Null),s.font.as_deref().map(Cell::Text).unwrap_or(Cell::Null),s.color.as_deref().map(Cell::Text).unwrap_or(Cell::Null),s.link.as_deref().map(Cell::Text).unwrap_or(Cell::Null)],float_columns(t.run))?;}}p.checkpoint()?;}}
Ok(root)}/// 🎟️ Admits all typed Document cells before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioDocumentSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Admits the exact authored table and column layout before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(n:usize)->Result<i64,ValueError>{i64::try_from(n).map_err(|e|ValueError::new(ValueRefusalKind::WorkLimit,e.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,n:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=n{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid document entity identity or columns"))}else{Ok(())}}
fn boolean(n:i64)->Result<bool,ValueError>{match n{0=>Ok(false),1=>Ok(true),_=>Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid document boolean"))}}
fn optional_real(row:SqliteRow<'_>,i:usize)->Result<Option<f64>,ValueError>{if row.is_null(i)?{Ok(None)}else{row.real(i).map(Some)}}
fn optional_integer(row:SqliteRow<'_>,i:usize)->Result<Option<i64>,ValueError>{if row.is_null(i)?{Ok(None)}else{row.integer(i).map(Some)}}
fn optional_text(row:SqliteRow<'_>,i:usize,r:&mut Reconstruction<'_,'_>)->Result<Option<String>,ValueError>{row.optional_text(i)?.map(|s|r.text(s)).transpose()}

/// 🔗️ Resolves the declared style or image identity without allocating a lookup map.
fn restore_reference(row:SqliteRow<'_>,column:usize,names:Option<&RowIndex<'_>>,control:&mut SqliteSnapshotControl<'_>)->Result<Option<String>,ValueError>{
 if row.is_null(column)?{return Ok(None)}let value=match names{Some(rows)=>rows.get(row.integer(column)?,control)?.ok_or_else(||doc_invalid("dangling document named reference"))?.text(3)?,None=>row.text(column)?};reconstruct_text(control,value).map(Some)
}
/// 🚫️ Reports the document's authored relational refusal.
fn doc_invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
/// 🗃️ Binds an actual document row family to paid source positions.
fn details<'a>(db:&'a SqliteDatabase,name:&str,columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<RowIndex<'a>,ValueError>{RowIndex::new(db,name,columns,float_columns(name),control,"invalid document entity identity or columns")}
/// 👪️ Validates each declared parent before paying its owner-ordinal ordering.
fn groups(rows:&RowIndex<'_>,parents:&RowIndex<'_>,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<usize>,ValueError>{
 for &index in rows.indices(){let row=rows.row(index)?;if parents.get(row.integer(1)?,control)?.is_none(){return Err(doc_invalid("invalid document relationship owner or identity"))}}
 rows.grouped_by(2,control,"document relationship order must be contiguous",|row|Ok((0,Some(row.integer(1)?))))
}
/// 🔎️ Borrows a group's contiguous paid scalar positions.
fn doc_range(rows:&RowIndex<'_>,order:&[usize],owner:i64,control:&mut SqliteSnapshotControl<'_>)->Result<std::ops::Range<usize>,ValueError>{rows.range_by(order,(0,Some(owner)),control,|row|Ok((0,Some(row.integer(1)?))))}
/// ➕️ Admits actual replacement backing before extending a scalar traversal stack.
fn doc_push<T>(values:&mut Vec<T>,value:T,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semio_framework_os_kernel::sqlite_snapshot::transfer::grow(values,SqliteSnapshotPhase::ReconstructSnapshot,control)?;values.push(value);Ok(())}
/// 🫴️ Moves one completed child collection directly from its guarded paid slot.
fn doc_take_collection(collections:&RowIndex<'_>,built:&mut[Option<Vec<DocBlock>>],id:i64,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<DocBlock>,ValueError>{
 let index=collections.position(id,control)?.ok_or_else(||doc_invalid("missing owned document collection"))?;built[index].take().ok_or_else(||doc_invalid("missing owned document collection"))
}
/// 🧩️ Reconstructs the actual block forest with paid indices and guarded partial native owners.
pub fn reconstruct_block_collections(db:&SqliteDatabase,t:DocSqliteTables,roots:&[i64],styles:Option<&RowIndex<'_>>,images:Option<&RowIndex<'_>>,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<Vec<DocBlock>>,ValueError>{
 let mut collections=details(db,t.collection,1,control)?;let mut blocks=details(db,t.block,2,control)?;let members=details(db,t.member,4,control)?;let member_order=groups(&members,&collections,control)?;
 let mut paragraphs=details(db,t.paragraph,2,control)?;let mut headings=details(db,t.heading,3,control)?;let mut lists=details(db,t.list,2,control)?;let mut tables=details(db,t.table,1,control)?;let mut codes=details(db,t.code,3,control)?;let mut quotes=details(db,t.quote,2,control)?;let mut image_blocks=details(db,t.image_block,5,control)?;let mut page_breaks=details(db,t.page_break,1,control)?;
 for &index in paragraphs.indices(){if headings.get(paragraphs.row(index)?.rowid,control)?.is_some(){return Err(doc_invalid("contradictory paragraph and heading fields"))}}
 let mut runs=details(db,t.run,11,control)?;for &index in runs.indices(){let owner=runs.row(index)?.integer(1)?;if paragraphs.get(owner,control)?.is_none()&&headings.get(owner,control)?.is_none(){return Err(doc_invalid("invalid document relationship owner or identity"))}}
 let run_order=runs.grouped_by(2,control,"document relationship order must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;
 let mut items=details(db,t.list_item,4,control)?;let item_order=groups(&items,&lists,control)?;
 let mut rows=details(db,t.table_row,3,control)?;let row_order=groups(&rows,&tables,control)?;
 let mut cells=details(db,t.table_cell,4,control)?;let cell_order=groups(&cells,&rows,control)?;
 let mut visited=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(blocks.len(),control)?;for index in 0..blocks.len(){control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,blocks.len())?;visited.push(0u8);}
 let mut schedule=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(collections.len(),control)?;
 let mut stack=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(roots.len(),control)?;for root in roots.iter().rev(){control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,stack.len(),roots.len())?;stack.push((*root,false));}
 while let Some((id,done))=stack.pop(){
  control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,schedule.len(),collections.len())?;if done{schedule.push(id);continue}
  if collections.take(id,control)?.is_none(){return Err(doc_invalid("cyclic, multiply owned or dangling document block collection"))}doc_push(&mut stack,(id,true),control)?;
  let range=doc_range(&members,&member_order,id,control)?;
  for &position in &member_order[range]{let member=members.row(position)?;let block_id=member.integer(3)?;let block_position=blocks.position(block_id,control)?.ok_or_else(||doc_invalid("dangling document block member"))?;if visited[block_position]!=0{return Err(doc_invalid("multiply owned document block"))}visited[block_position]=1;let block=blocks.row(block_position)?;
   match block.text(1)?{
    "list"=>{let range=doc_range(&items,&item_order,block_id,control)?;for &position in &item_order[range]{doc_push(&mut stack,(items.row(position)?.integer(3)?,false),control)?;}},
    "table"=>{let range=doc_range(&rows,&row_order,block_id,control)?;for &position in &row_order[range]{let row=rows.row(position)?;let range=doc_range(&cells,&cell_order,row.rowid,control)?;for &position in &cell_order[range]{doc_push(&mut stack,(cells.row(position)?.integer(3)?,false),control)?;}}},
    "quote"=>doc_push(&mut stack,(quotes.get(block_id,control)?.ok_or_else(||doc_invalid("missing quote fields"))?.integer(1)?,false),control)?,
    _=>{}
   }
  }
 }
 if collections.remaining()!=0{return Err(doc_invalid("orphan document block or collection"))}for(index,mark)in visited.iter().enumerate(){control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,visited.len())?;if *mark==0{return Err(doc_invalid("orphan document block or collection"))}}
 let mut built=Owned::new(semio_framework_os_kernel::sqlite_snapshot::transfer::reserve::<Option<Vec<DocBlock>>>(collections.len(),control)?);for index in 0..collections.len(){control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,collections.len())?;built.get_mut().push(None);}
 for collection in schedule{
  let range=doc_range(&members,&member_order,collection,control)?;let mut native=Owned::new(semio_framework_os_kernel::sqlite_snapshot::transfer::reserve::<DocBlock>(range.len(),control)?);
  for &position in &member_order[range]{let id=members.row(position)?.integer(3)?;let row=blocks.take(id,control)?.ok_or_else(||doc_invalid("duplicate document block ownership"))?;
   let mut block=match row.text(1)?{
    "paragraph"|"heading"=>{
     let heading=row.text(1)?=="heading";let detail=if heading{headings.take(id,control)?.ok_or_else(||doc_invalid("missing heading fields"))?}else{paragraphs.take(id,control)?.ok_or_else(||doc_invalid("missing paragraph fields"))?};
     let mut block=Owned::new(if heading{DocBlock::Heading{level:u8::try_from(detail.integer(1)?).map_err(|error|doc_invalid(&error.to_string()))?,style_id:None,runs:Vec::new()}}else{DocBlock::Paragraph{style_id:None,runs:Vec::new()}});
     let range=doc_range(&runs,&run_order,id,control)?;
     if let DocBlock::Paragraph{style_id,runs:native_runs}|DocBlock::Heading{style_id,runs:native_runs,..}=block.get_mut(){
      *style_id=restore_reference(detail,if heading{2}else{1},styles,control)?;*native_runs=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(range.len(),control)?;
      for &position in &run_order[range]{let run=runs.take_index(position,control)?.ok_or_else(||doc_invalid("duplicate document run ownership"))?;let mut native_run=Owned::new(DocRun::default());native_run.get_mut().text=reconstruct_text(control,run.text(3)?)?;
       native_run.get_mut().style.bold=boolean(run.integer(4)?)?;native_run.get_mut().style.italic=boolean(run.integer(5)?)?;native_run.get_mut().style.underline=boolean(run.integer(6)?)?;native_run.get_mut().style.size=optional_real(run,7)?;
       native_run.get_mut().style.font=run.optional_text(8)?.map(|text|reconstruct_text(control,text)).transpose()?;native_run.get_mut().style.color=run.optional_text(9)?.map(|text|reconstruct_text(control,text)).transpose()?;native_run.get_mut().style.link=run.optional_text(10)?.map(|text|reconstruct_text(control,text)).transpose()?;native_runs.push(native_run.take());
      }
     }block
    },
    "list"=>{let detail=lists.take(id,control)?.ok_or_else(||doc_invalid("missing list fields"))?;let range=doc_range(&items,&item_order,id,control)?;let mut block=Owned::new(DocBlock::List{ordered:boolean(detail.integer(1)?)?,items:Vec::new()});if let DocBlock::List{items:native_items,..}=block.get_mut(){*native_items=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(range.len(),control)?;for &position in &item_order[range]{let item=items.take_index(position,control)?.ok_or_else(||doc_invalid("duplicate list item ownership"))?;native_items.push(DocListItem{blocks:doc_take_collection(&collections,built.get_mut(),item.integer(3)?,control)?});}}block},
    "table"=>{tables.take(id,control)?.ok_or_else(||doc_invalid("missing table fields"))?;let range=doc_range(&rows,&row_order,id,control)?;let mut block=Owned::new(DocBlock::Table{rows:Vec::new()});if let DocBlock::Table{rows:native_rows}=block.get_mut(){*native_rows=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(range.len(),control)?;for &position in &row_order[range]{let row=rows.take_index(position,control)?.ok_or_else(||doc_invalid("duplicate table row ownership"))?;let range=doc_range(&cells,&cell_order,row.rowid,control)?;let mut native_row=Owned::new(DocTableRow::default());native_row.get_mut().cells=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(range.len(),control)?;for &position in &cell_order[range]{let cell=cells.take_index(position,control)?.ok_or_else(||doc_invalid("duplicate table cell ownership"))?;native_row.get_mut().cells.push(DocTableCell{blocks:doc_take_collection(&collections,built.get_mut(),cell.integer(3)?,control)?});}native_rows.push(native_row.take());}}block},
    "code"=>{let detail=codes.take(id,control)?.ok_or_else(||doc_invalid("missing code fields"))?;let mut block=Owned::new(DocBlock::Code{language:None,text:String::new()});if let DocBlock::Code{language,text}=block.get_mut(){*language=detail.optional_text(1)?.map(|text|reconstruct_text(control,text)).transpose()?;*text=reconstruct_text(control,detail.text(2)?)?;}block},
    "quote"=>{let detail=quotes.take(id,control)?.ok_or_else(||doc_invalid("missing quote fields"))?;Owned::new(DocBlock::Quote{blocks:doc_take_collection(&collections,built.get_mut(),detail.integer(1)?,control)?})},
    "image"=>{let detail=image_blocks.take(id,control)?.ok_or_else(||doc_invalid("missing image block fields"))?;let mut block=Owned::new(DocBlock::Image{image_id:String::new(),alt:String::new(),width:None,height:None});if let DocBlock::Image{image_id,alt,width,height}=block.get_mut(){*image_id=restore_reference(detail,1,images,control)?.ok_or_else(||doc_invalid("null document image reference"))?;*alt=reconstruct_text(control,detail.text(2)?)?;*width=optional_real(detail,3)?;*height=optional_real(detail,4)?;}block},
    "pageBreak"=>{page_breaks.take(id,control)?.ok_or_else(||doc_invalid("missing page break fields"))?;Owned::new(DocBlock::PageBreak)},
    _=>return Err(doc_invalid("unknown document block kind"))
   };
   control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,native.get_mut().len(),members.len())?;native.get_mut().push(block.take());
  }
  let position=collections.position(collection,control)?.ok_or_else(||doc_invalid("missing collection identity"))?;built.get_mut()[position]=Some(native.take());
 }
 if [paragraphs.remaining(),headings.remaining(),lists.remaining(),tables.remaining(),codes.remaining(),quotes.remaining(),image_blocks.remaining(),page_breaks.remaining(),runs.remaining(),items.remaining(),rows.remaining(),cells.remaining()].iter().any(|remaining|*remaining!=0){return Err(doc_invalid("orphan or contradictory document block detail"))}
 let mut result=Owned::new(semio_framework_os_kernel::sqlite_snapshot::transfer::reserve::<Vec<DocBlock>>(roots.len(),control)?);for root in roots{result.get_mut().push(doc_take_collection(&collections,built.get_mut(),*root,control)?);}
 for(index,value)in built.get_mut().iter().enumerate(){control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,collections.len())?;if value.is_some(){return Err(doc_invalid("orphan native block collection"))}}
 control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,roots.len(),roots.len())?;Ok(result.take())
}
impl ArtifactSqliteSnapshot for SemioDocumentSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}

fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::os_io::IoPayload,ValueError>{crate::standards::v1::subsets::document::io::sqlite::snapshot::native_encoding::encode(self,encoding,control)}

fn decode_sqlite_snapshot_native(payload:&store::os_io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::document::io::sqlite::snapshot::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{let result=(||->Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})();result}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{let result=(||->Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="document"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_document_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})();result.map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(control)}
fn from_sqlite_database(db:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {let result=(||->Result<Self,ValueError>{ semantic::layout(control.limits())?;Self::reconstruct_sqlite_database(db, control, Self::SQLITE_SCHEMA) })();result}
}

impl SemioDocumentSnapshot {
    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(db:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,ValueError> {
control.check_database(db,SqliteSnapshotPhase::ReconstructSnapshot)?;semio_framework_os_kernel::sqlite_snapshot::validate_sqlite_database_schema_controlled(db,declared_schema,SqliteSnapshotPhase::ReconstructSnapshot,control)?;let doc=single_float_row(db,"semio_document_document")?;identity(doc,3)?;if doc.rowid!=1{return Err(doc_invalid("invalid document identity"))}
let styles=details(db,"semio_document_style",6,control)?;let images=details(db,"semio_document_image",6,control)?;
let style_order=styles.ordered(2,control,"relationship ordinals must be contiguous and unique")?;let image_order=images.ordered(2,control,"relationship ordinals must be contiguous and unique")?;
styles.unique_text(styles.indices(),3,control,"invalid document style identity or owner")?;images.unique_text(images.indices(),3,control,"invalid document image identity or owner")?;
for rows in [&styles,&images]{for &index in rows.indices(){if rows.row(index)?.integer(1)?!=1{return Err(doc_invalid("invalid document named entity owner"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,rows.len())?;}}
let mut parents=styles.parent_positions(5,control,"dangling document style inheritance")?;RowIndex::cycles(&mut parents,control,"cyclic document style inheritance")?;
let mut snapshot=Owned::new(Self{schema:String::new(),styles:Vec::new(),images:Vec::new(),blocks:Vec::new()});
let mut blocks=Owned::new(reconstruct_block_collections(db,DOCUMENT_TABLES,&[doc.integer(2)?],Some(&styles),Some(&images),control)?);snapshot.get_mut().blocks=blocks.get_mut().pop().ok_or_else(||doc_invalid("missing document root collection"))?;
snapshot.get_mut().styles=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(style_order.len(),control)?;
for index in style_order{let row=styles.row(index)?;let mut style=Owned::new(DocStyle::default());style.get_mut().id=reconstruct_text(control,row.text(3)?)?;style.get_mut().name=reconstruct_text(control,row.text(4)?)?;style.get_mut().based_on=restore_reference(row,5,Some(&styles),control)?;snapshot.get_mut().styles.push(style.take());}
snapshot.get_mut().images=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(image_order.len(),control)?;
for index in image_order{let row=images.row(index)?;let mut image=Owned::new(DocImage::default());image.get_mut().id=reconstruct_text(control,row.text(3)?)?;image.get_mut().mime=reconstruct_text(control,row.text(4)?)?;image.get_mut().bytes=reconstruct_blob(control,row.blob(5)?)?;snapshot.get_mut().images.push(image.take());}
snapshot.get_mut().schema=reconstruct_text(control,doc.text(1)?)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(snapshot.take())

}
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_document_run"=>&[FloatColumn::Binary64(7)],"semio_document_image_block"=>&[FloatColumn::Binary64(3),FloatColumn::Binary64(4)],"semio_presentation_run"=>&[FloatColumn::Binary64(7)],"semio_presentation_image_block"=>&[FloatColumn::Binary64(3),FloatColumn::Binary64(4)],_=>&[]}}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioDocumentSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.styles.len())?;for style in &self.styles{b.text(&style.id)?;b.text(&style.name)?;b.optional_text(style.based_on.as_deref())?;}b.entities(self.images.len())?;for image in &self.images{b.text(&image.id)?;b.text(&image.mime)?;b.bytes(&image.bytes)?;}native_blocks(&self.blocks,b)?;Ok(())}
}

/// 📑️ Bounds actual shared DocBlock variants and their typed inline fields iteratively.
pub fn native_blocks(roots:&[DocBlock],b:&mut Bound<'_, '_>)->Result<(),ValueError>{b.entities(roots.len())?;let mut pending:Vec<_>=roots.iter().rev().collect();while let Some(block)=pending.pop(){b.entities(1)?;match block{
DocBlock::Paragraph{style_id,runs}|DocBlock::Heading{style_id,runs,..}=>{b.scalars(1)?;b.optional_text(style_id.as_deref())?;b.entities(runs.len())?;for run in runs{b.text(&run.text)?;b.scalars(4)?;b.optional_text(run.style.font.as_deref())?;b.optional_text(run.style.color.as_deref())?;b.optional_text(run.style.link.as_deref())?;}},
DocBlock::List{items,..}=>{b.scalars(1)?;b.entities(items.len())?;for item in items.iter().rev(){b.entities(item.blocks.len())?;pending.extend(item.blocks.iter().rev());}},
DocBlock::Table{rows}=>{b.entities(rows.len())?;for row in rows.iter().rev(){b.entities(row.cells.len())?;for cell in row.cells.iter().rev(){b.entities(cell.blocks.len())?;pending.extend(cell.blocks.iter().rev());}}},
DocBlock::Code{language,text}=>{b.optional_text(language.as_deref())?;b.text(text)?;},DocBlock::Quote{blocks}=>{b.entities(blocks.len())?;pending.extend(blocks.iter().rev());},
DocBlock::Image{image_id,alt,..}=>{b.text(image_id)?;b.text(alt)?;b.scalars(2)?;},DocBlock::PageBreak=>{}}}Ok(())}

impl SemioDocumentSnapshot {
    /// 🪶️ Projects the owned subset into its declared relational writer while preserving refusals.
    pub fn project_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(control.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;visit_rows(self,&mut out)?;out.finish()}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
