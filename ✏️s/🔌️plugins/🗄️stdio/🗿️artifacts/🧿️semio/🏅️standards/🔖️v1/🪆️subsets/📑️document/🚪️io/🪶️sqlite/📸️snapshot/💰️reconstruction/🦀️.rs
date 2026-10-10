//! 🧩️ Original document block fields and scalar forest traversal stay in their caller workspace.
use super::*;
use semio_framework_os_kernel::sqlite_snapshot::{transfer,artifact::{RowIndexStorage,reconstruct_text_into,reconstruct_blob_into}};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::workspace;

/// 🪆️ Stores every partial block and all relational scratch in the original outer prefix.
#[derive(semio_framework_value::RetireOwned)]
pub(crate)struct BlockReconstructionStorage{
 indices:[RowIndexStorage;15],member_order:Vec<usize>,run_order:Vec<usize>,item_order:Vec<usize>,row_order:Vec<usize>,cell_order:Vec<usize>,
 visited:Vec<u8>,schedule:Vec<i64>,pending:Vec<(i64,bool)>,built:Vec<Option<Vec<DocBlock>>>,blocks:Vec<Option<DocBlock>>,pub(crate)output:Vec<Vec<DocBlock>>
}
impl BlockReconstructionStorage{
 pub(crate)fn empty()->Self{Self{indices:std::array::from_fn(|_|RowIndexStorage::empty()),member_order:Vec::new(),run_order:Vec::new(),item_order:Vec::new(),row_order:Vec::new(),cell_order:Vec::new(),visited:Vec::new(),schedule:Vec::new(),pending:Vec::new(),built:Vec::new(),blocks:Vec::new(),output:Vec::new()}}
}
/// 🛂️ Binds borrowed source rows to the actual original scalar index storage.
pub(super)fn details<'a>(storage:&'a mut RowIndexStorage,db:&'a SqliteDatabase,name:&str,columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<RowIndex<'a>,ValueError>{RowIndex::new(storage,db,name,columns,float_columns(name),control,"invalid document entity identity or columns")}
/// 🔗️ Copies a nullable reference directly into its original selected field.
pub(super)fn reference_into(output:&mut Option<String>,row:SqliteRow<'_>,column:usize,names:Option<&RowIndex<'_>>,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 if row.is_null(column)?{return Ok(())}let value=match names{Some(rows)=>rows.get(row.integer(column)?,control)?.ok_or_else(||doc_invalid("dangling document named reference"))?.text(3)?,None=>row.text(column)?};*output=Some(String::new());reconstruct_text_into(output.as_mut().unwrap(),control,value)
}
/// 🔤️ Initializes a nullable original text slot before copying any bytes.
fn optional_text_into(output:&mut Option<String>,row:SqliteRow<'_>,column:usize,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{if let Some(value)=row.optional_text(column)?{*output=Some(String::new());reconstruct_text_into(output.as_mut().unwrap(),control,value)?;}Ok(())}
/// 👪️ Checks authored owners before sorting positions into the original relationship backing.
fn groups_into(order:&mut Vec<usize>,rows:&RowIndex<'_>,parents:&RowIndex<'_>,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{for &index in rows.indices(){if parents.get(rows.row(index)?.integer(1)?,control)?.is_none(){return Err(doc_invalid("invalid document relationship owner or identity"))}}rows.grouped_by_into(order,2,control,"document relationship order must be contiguous",|row|Ok((0,Some(row.integer(1)?))))}
/// 🔎️ Borrows the scalar range belonging to one actual collection or nested owner.
fn range(rows:&RowIndex<'_>,order:&[usize],owner:i64,control:&mut SqliteSnapshotControl<'_>)->Result<std::ops::Range<usize>,ValueError>{rows.range_by(order,(0,Some(owner)),control,|row|Ok((0,Some(row.integer(1)?))))}
/// 📏️ Refuses an overflow before any forest backing is admitted.
fn extent(left:usize,right:usize)->Result<usize,ValueError>{left.checked_add(right).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"document forest extent overflow"))}
/// ➕️ Inserts one scalar task into previously admitted original backing.
fn push(values:&mut Vec<(i64,bool)>,value:(i64,bool))->Result<(),ValueError>{if values.len()==values.capacity(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"document traversal backing was not admitted"))}values.push(value);Ok(())}
/// 🫴️ Moves a completed original child collection after its last fallible identity lookup.
fn take_collection(collections:&RowIndex<'_>,built:&mut[Option<Vec<DocBlock>>],id:i64,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<DocBlock>,ValueError>{let position=collections.position(id,control)?.ok_or_else(||doc_invalid("missing owned document collection"))?;built[position].take().ok_or_else(||doc_invalid("missing owned document collection"))}
/// 🏗️ Fills the original block forest without creating local owned field or traversal buffers.
pub(crate)fn reconstruct_block_collections_into(storage:&mut BlockReconstructionStorage,db:&SqliteDatabase,t:DocSqliteTables,roots:&[i64],styles:Option<&RowIndex<'_>>,images:Option<&RowIndex<'_>>,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 let BlockReconstructionStorage{indices,member_order,run_order,item_order,row_order,cell_order,visited,schedule,pending,built,blocks:native_blocks,output}=storage;
 let [collection_store,block_store,member_store,paragraph_store,heading_store,run_store,list_store,item_store,table_store,row_store,cell_store,code_store,quote_store,image_store,page_store]=indices;
 let mut collections=details(collection_store,db,t.collection,1,control)?;let mut blocks=details(block_store,db,t.block,2,control)?;let mut members=details(member_store,db,t.member,4,control)?;
 let mut paragraphs=details(paragraph_store,db,t.paragraph,2,control)?;let mut headings=details(heading_store,db,t.heading,3,control)?;let mut runs=details(run_store,db,t.run,11,control)?;
 let mut lists=details(list_store,db,t.list,2,control)?;let mut items=details(item_store,db,t.list_item,4,control)?;let mut tables=details(table_store,db,t.table,1,control)?;let mut rows=details(row_store,db,t.table_row,3,control)?;let mut cells=details(cell_store,db,t.table_cell,4,control)?;
 let mut codes=details(code_store,db,t.code,3,control)?;let mut quotes=details(quote_store,db,t.quote,2,control)?;let mut image_blocks=details(image_store,db,t.image_block,5,control)?;let mut page_breaks=details(page_store,db,t.page_break,1,control)?;
 groups_into(member_order,&members,&collections,control)?;groups_into(item_order,&items,&lists,control)?;groups_into(row_order,&rows,&tables,control)?;groups_into(cell_order,&cells,&rows,control)?;
 for &index in paragraphs.indices(){if headings.get(paragraphs.row(index)?.rowid,control)?.is_some(){return Err(doc_invalid("contradictory paragraph and heading fields"))}}
 for &index in runs.indices(){let owner=runs.row(index)?.integer(1)?;if paragraphs.get(owner,control)?.is_none()&&headings.get(owner,control)?.is_none(){return Err(doc_invalid("invalid document relationship owner or identity"))}}
 runs.grouped_by_into(run_order,2,control,"document relationship order must be contiguous",|row|Ok((0,Some(row.integer(1)?))))?;
 *visited=transfer::reserve(blocks.len(),control)?;*native_blocks=transfer::reserve(blocks.len(),control)?;for index in 0..blocks.len(){visited.push(0);native_blocks.push(None);control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,blocks.len())?;}
 *schedule=transfer::reserve(collections.len(),control)?;*built=transfer::reserve(collections.len(),control)?;for index in 0..collections.len(){built.push(Some(Vec::new()));control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,collections.len())?;}
 let pending_count=extent(extent(extent(extent(roots.len(),collections.len())?,items.len())?,cells.len())?,quotes.len())?;*pending=transfer::reserve(pending_count,control)?;
 for root in roots.iter().rev(){push(pending,(*root,false))?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,pending.len(),roots.len())?;}
 while let Some((id,done))=pending.pop(){
  control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,schedule.len(),collections.len())?;if done{schedule.push(id);continue}
  if collections.take(id,control)?.is_none(){return Err(doc_invalid("cyclic, multiply owned or dangling document block collection"))}push(pending,(id,true))?;
  for &position in &member_order[range(&members,member_order,id,control)?]{let member=members.row(position)?;let block_id=member.integer(3)?;let position=blocks.position(block_id,control)?.ok_or_else(||doc_invalid("dangling document block member"))?;if visited[position]!=0{return Err(doc_invalid("multiply owned document block"))}visited[position]=1;let block=blocks.row(position)?;
   match block.text(1)?{
    "list"=>{for &position in &item_order[range(&items,item_order,block_id,control)?]{push(pending,(items.row(position)?.integer(3)?,false))?;}},
    "table"=>{for &position in &row_order[range(&rows,row_order,block_id,control)?]{let row=rows.row(position)?;for &position in &cell_order[range(&cells,cell_order,row.rowid,control)?]{push(pending,(cells.row(position)?.integer(3)?,false))?;}}},
    "quote"=>push(pending,(quotes.get(block_id,control)?.ok_or_else(||doc_invalid("missing quote fields"))?.integer(1)?,false))?,
    _=>{}
   }
  }
 }
 if collections.remaining()!=0{return Err(doc_invalid("orphan document block or collection"))}for(index,mark)in visited.iter().enumerate(){control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,visited.len())?;if *mark==0{return Err(doc_invalid("orphan document block or collection"))}}
 for &collection in schedule.iter(){
  let collection_position=collections.position(collection,control)?.ok_or_else(||doc_invalid("missing collection identity"))?;let member_range=range(&members,member_order,collection,control)?;
  *built[collection_position].as_mut().unwrap()=transfer::reserve(member_range.len(),control)?;
  for &position in &member_order[member_range]{
   let id=members.take_index(position,control)?.ok_or_else(||doc_invalid("duplicate document member ownership"))?.integer(3)?;let block_position=blocks.position(id,control)?.ok_or_else(||doc_invalid("missing document block identity"))?;let row=blocks.take_index(block_position,control)?.ok_or_else(||doc_invalid("duplicate document block ownership"))?;
   let slot=&mut native_blocks[block_position];
   match row.text(1)?{
    "paragraph"|"heading"=>{
     let heading=row.text(1)?=="heading";let detail=if heading{headings.take(id,control)?.ok_or_else(||doc_invalid("missing heading fields"))?}else{paragraphs.take(id,control)?.ok_or_else(||doc_invalid("missing paragraph fields"))?};
     let level=if heading{u8::try_from(detail.integer(1)?).map_err(|_|doc_invalid("invalid document heading level"))?}else{0};
     *slot=Some(if heading{DocBlock::Heading{level,style_id:None,runs:Vec::new()}}else{DocBlock::Paragraph{style_id:None,runs:Vec::new()}});
     let run_range=range(&runs,run_order,id,control)?;
     if let DocBlock::Paragraph{style_id,runs:native_runs}|DocBlock::Heading{style_id,runs:native_runs,..}=slot.as_mut().unwrap(){
      reference_into(style_id,detail,if heading{2}else{1},styles,control)?;*native_runs=transfer::reserve(run_range.len(),control)?;
      for &position in &run_order[run_range]{let run=runs.take_index(position,control)?.ok_or_else(||doc_invalid("duplicate document run ownership"))?;native_runs.push(DocRun::default());let native_run=native_runs.last_mut().unwrap();reconstruct_text_into(&mut native_run.text,control,run.text(3)?)?;
       native_run.style.bold=boolean(run.integer(4)?)?;native_run.style.italic=boolean(run.integer(5)?)?;native_run.style.underline=boolean(run.integer(6)?)?;native_run.style.size=optional_real(run,7)?;
       optional_text_into(&mut native_run.style.font,run,8,control)?;optional_text_into(&mut native_run.style.color,run,9,control)?;optional_text_into(&mut native_run.style.link,run,10,control)?;
      }
     }
    },
    "list"=>{let detail=lists.take(id,control)?.ok_or_else(||doc_invalid("missing list fields"))?;*slot=Some(DocBlock::List{ordered:boolean(detail.integer(1)?)?,items:Vec::new()});let item_range=range(&items,item_order,id,control)?;
     if let DocBlock::List{items:native_items,..}=slot.as_mut().unwrap(){*native_items=transfer::reserve(item_range.len(),control)?;for &position in &item_order[item_range]{let item=items.take_index(position,control)?.ok_or_else(||doc_invalid("duplicate list item ownership"))?;native_items.push(DocListItem{blocks:take_collection(&collections,built,item.integer(3)?,control)?});}}
    },
    "table"=>{tables.take(id,control)?.ok_or_else(||doc_invalid("missing table fields"))?;*slot=Some(DocBlock::Table{rows:Vec::new()});let row_range=range(&rows,row_order,id,control)?;
     if let DocBlock::Table{rows:native_rows}=slot.as_mut().unwrap(){*native_rows=transfer::reserve(row_range.len(),control)?;for &position in &row_order[row_range]{let row=rows.take_index(position,control)?.ok_or_else(||doc_invalid("duplicate table row ownership"))?;let cell_range=range(&cells,cell_order,row.rowid,control)?;native_rows.push(DocTableRow::default());let native_row=native_rows.last_mut().unwrap();native_row.cells=transfer::reserve(cell_range.len(),control)?;for &position in &cell_order[cell_range]{let cell=cells.take_index(position,control)?.ok_or_else(||doc_invalid("duplicate table cell ownership"))?;native_row.cells.push(DocTableCell{blocks:take_collection(&collections,built,cell.integer(3)?,control)?});}}}
    },
    "code"=>{let detail=codes.take(id,control)?.ok_or_else(||doc_invalid("missing code fields"))?;*slot=Some(DocBlock::Code{language:None,text:String::new()});if let DocBlock::Code{language,text}=slot.as_mut().unwrap(){optional_text_into(language,detail,1,control)?;reconstruct_text_into(text,control,detail.text(2)?)?;}},
    "quote"=>{let detail=quotes.take(id,control)?.ok_or_else(||doc_invalid("missing quote fields"))?;*slot=Some(DocBlock::Quote{blocks:Vec::new()});if let DocBlock::Quote{blocks}=slot.as_mut().unwrap(){*blocks=take_collection(&collections,built,detail.integer(1)?,control)?;}},
    "image"=>{let detail=image_blocks.take(id,control)?.ok_or_else(||doc_invalid("missing image block fields"))?;*slot=Some(DocBlock::Image{image_id:String::new(),alt:String::new(),width:None,height:None});if let DocBlock::Image{image_id,alt,width,height}=slot.as_mut().unwrap(){if detail.is_null(1)?{return Err(doc_invalid("null document image reference"))}let value=match images{Some(images)=>images.get(detail.integer(1)?,control)?.ok_or_else(||doc_invalid("dangling document image reference"))?.text(3)?,None=>detail.text(1)?};reconstruct_text_into(image_id,control,value)?;reconstruct_text_into(alt,control,detail.text(2)?)?;*width=optional_real(detail,3)?;*height=optional_real(detail,4)?;}},
    "pageBreak"=>{page_breaks.take(id,control)?.ok_or_else(||doc_invalid("missing page break fields"))?;*slot=Some(DocBlock::PageBreak);},
    _=>return Err(doc_invalid("unknown document block kind"))
   }
   control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,built[collection_position].as_ref().unwrap().len(),members.len())?;built[collection_position].as_mut().unwrap().push(slot.take().unwrap());
  }
 }
 if [members.remaining(),blocks.remaining(),paragraphs.remaining(),headings.remaining(),lists.remaining(),tables.remaining(),codes.remaining(),quotes.remaining(),image_blocks.remaining(),page_breaks.remaining(),runs.remaining(),items.remaining(),rows.remaining(),cells.remaining()].iter().any(|remaining|*remaining!=0){return Err(doc_invalid("orphan or contradictory document block detail"))}
 *output=transfer::reserve(roots.len(),control)?;for root in roots{output.push(take_collection(&collections,built,*root,control)?);}
 for(index,value)in built.iter().enumerate(){control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,collections.len())?;if value.is_some(){return Err(doc_invalid("orphan native block collection"))}}
 control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,roots.len(),roots.len())
}

/// 📑️ Owns the complete document snapshot, schema validation and every reconstruction frontier.
#[derive(semio_framework_value::RetireOwned)]
struct Prefix{snapshot:Option<SemioDocumentSnapshot>,validation:transfer::SchemaValidationStorage,styles:RowIndexStorage,images:RowIndexStorage,style_order:Vec<usize>,image_order:Vec<usize>,style_names:Vec<usize>,image_names:Vec<usize>,parents:Vec<(Option<usize>,u8)>,blocks:BlockReconstructionStorage}
impl Prefix{fn empty()->Self{Self{snapshot:Some(SemioDocumentSnapshot{schema:String::new(),styles:Vec::new(),images:Vec::new(),blocks:Vec::new()}),validation:transfer::SchemaValidationStorage::empty(),styles:RowIndexStorage::empty(),images:RowIndexStorage::empty(),style_order:Vec::new(),image_order:Vec::new(),style_names:Vec::new(),image_names:Vec::new(),parents:Vec::new(),blocks:BlockReconstructionStorage::empty()}}}
/// 🫙️ Admits the original complete document workspace before any relational materialization.
pub(super)fn reconstruct(db:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,schema:&str)->Result<SemioDocumentSnapshot,ValueError>{workspace::reconstruct_projected(control,Prefix::empty,|prefix,control|reconstruct_into(prefix,db,control,schema),|prefix|prefix.snapshot.take().expect("complete Document snapshot"))}
fn reconstruct_into(prefix:&mut Prefix,db:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,schema:&str)->Result<(),ValueError>{
 control.check_database(db,SqliteSnapshotPhase::ReconstructSnapshot)?;transfer::validate_database_into(&mut prefix.validation,db,schema,SqliteSnapshotPhase::ReconstructSnapshot,control)?;transfer::validate_component_into(&mut prefix.validation,db,<SemioDocumentSnapshot as ArtifactSqliteSnapshot>::SQLITE_SCHEMA,SqliteSnapshotPhase::ReconstructSnapshot,control)?;
 let Prefix{snapshot,validation:_,styles:style_storage,images:image_storage,style_order,image_order,style_names,image_names,parents,blocks}=prefix;
 let snapshot=snapshot.as_mut().unwrap();let doc=single_float_row(db,"semio_document_document")?;identity(doc,3)?;if doc.rowid!=1{return Err(doc_invalid("invalid document identity"))}
 let styles=details(style_storage,db,"semio_document_style",6,control)?;let images=details(image_storage,db,"semio_document_image",6,control)?;
 styles.ordered_into(style_order,2,control,"relationship ordinals must be contiguous and unique")?;images.ordered_into(image_order,2,control,"relationship ordinals must be contiguous and unique")?;
 styles.unique_text(style_names,styles.indices(),3,control,"invalid document style identity or owner")?;images.unique_text(image_names,images.indices(),3,control,"invalid document image identity or owner")?;
 for rows in [&styles,&images]{for &index in rows.indices(){if rows.row(index)?.integer(1)?!=1{return Err(doc_invalid("invalid document named entity owner"))}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,rows.len())?;}}
 styles.parent_positions_into(parents,5,control,"dangling document style inheritance")?;RowIndex::cycles(parents,control,"cyclic document style inheritance")?;
 reconstruct_block_collections_into(blocks,db,DOCUMENT_TABLES,&[doc.integer(2)?],Some(&styles),Some(&images),control)?;
 snapshot.blocks=blocks.output.pop().ok_or_else(||doc_invalid("missing document root collection"))?;
 snapshot.styles=transfer::reserve(style_order.len(),control)?;for &index in style_order.iter(){let row=styles.row(index)?;snapshot.styles.push(DocStyle::default());let style=snapshot.styles.last_mut().unwrap();reconstruct_text_into(&mut style.id,control,row.text(3)?)?;reconstruct_text_into(&mut style.name,control,row.text(4)?)?;reference_into(&mut style.based_on,row,5,Some(&styles),control)?;}
 snapshot.images=transfer::reserve(image_order.len(),control)?;for &index in image_order.iter(){let row=images.row(index)?;snapshot.images.push(DocImage::default());let image=snapshot.images.last_mut().unwrap();reconstruct_text_into(&mut image.id,control,row.text(3)?)?;reconstruct_text_into(&mut image.mime,control,row.text(4)?)?;reconstruct_blob_into(&mut image.bytes,control,row.blob(5)?)?;}
 reconstruct_text_into(&mut snapshot.schema,control,doc.text(1)?)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)
}
