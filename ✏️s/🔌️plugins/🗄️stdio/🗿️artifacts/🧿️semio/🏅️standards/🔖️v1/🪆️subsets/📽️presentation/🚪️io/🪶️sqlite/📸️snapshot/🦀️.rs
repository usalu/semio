//! 📽️ Master/layout/slide relationships, authored shapes and embedded typed document blocks.
use semio_framework_os_kernel::sqlite_snapshot::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native::Bound;
use semio_framework_os_kernel::sqlite_snapshot::artifact::{FloatColumn,FloatRow as SqliteRow,RowIndex,reconstruct_text,reconstruct_blob};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned;
use crate::standards::v1::subsets::presentation::schema::snapshot::{SemioPresentationSnapshot,SlideMaster,SlideLayout,Slide,SlideShape,SlideFrame,SlidePictureImage,SlideTableRow,SlideTableCell,PlaceholderKind};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2};
use crate::standards::v1::subsets::document::io::sqlite::snapshot::{DocSqliteTables,project_block_collection,reconstruct_block_collections};
use semio_framework_os_kernel::{ArtifactSqliteSnapshot,sqlite_snapshot::{artifact::{Cell,RowWriter,Reconstruction},validate_sqlite_database_schema,SqliteDatabase,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase}};
const BLOCKS:DocSqliteTables=DocSqliteTables{collection:"semio_presentation_collection",member:"semio_presentation_member",block:"semio_presentation_block",paragraph:"semio_presentation_paragraph",heading:"semio_presentation_heading",run:"semio_presentation_run",list:"semio_presentation_list",list_item:"semio_presentation_list_item",table:"semio_presentation_table",table_row:"semio_presentation_table_row",table_cell:"semio_presentation_table_cell",code:"semio_presentation_code",quote:"semio_presentation_quote",image_block:"semio_presentation_image_block",page_break:"semio_presentation_page_break"};
#[path="🧮️semantic/🦀️.rs"]
pub(crate)mod semantic;
/// 🫳️ Visits real Presentation owner partitions, shape frames and embedded DocBlock fields.
pub(crate)fn visit_rows(snapshot:&SemioPresentationSnapshot,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
 use crate::standards::v1::subsets::document::io::sqlite::snapshot::{doc_frontier,doc_identity};
 let masters=doc_frontier(snapshot.masters.iter().map(|master|master.id.as_str()),snapshot.masters.len(),out)?;let layouts=doc_frontier(snapshot.layouts.iter().map(|layout|layout.id.as_str()),snapshot.layouts.len(),out)?;let _slides=doc_frontier(snapshot.slides.iter().map(|slide|slide.id.as_str()),snapshot.slides.len(),out)?;
 out.insert_key("semio_presentation_document",1,&[Cell::Text(&snapshot.schema)])?;
 for(ordinal,master)in snapshot.masters.iter().enumerate(){let id=out.insert("semio_presentation_master",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&master.id)])?;project_shapes(&master.shapes,(Some(id),None,None),out)?;}
 for(ordinal,layout)in snapshot.layouts.iter().enumerate(){let master=doc_identity(&masters,&layout.master_id,out)?;let id=out.insert("semio_presentation_layout",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&layout.id),Cell::Integer(master)])?;project_shapes(&layout.shapes,(None,Some(id),None),out)?;}
 for(ordinal,slide)in snapshot.slides.iter().enumerate(){let layout=slide.layout_id.as_deref().map(|id|doc_identity(&layouts,id,out)).transpose()?.map(Cell::Integer).unwrap_or(Cell::Null);let notes=project_block_collection(&slide.notes,BLOCKS,None,None,out)?;let id=out.insert("semio_presentation_slide",&[Cell::Integer(1),Cell::Integer(number(ordinal)?),Cell::Text(&slide.id),layout,Cell::Integer(notes)])?;project_shapes(&slide.shapes,(None,None,Some(id)),out)?;}Ok(())
}
/// 🖼️ Projects four real shape variants through their actual exact frame column roles.
fn project_shapes(shapes:&[SlideShape],owner:(Option<i64>,Option<i64>,Option<i64>),p:&mut RowWriter<'_,'_>)->Result<(),ValueError>{for(ordinal,shape)in shapes.iter().enumerate(){let(kind,frame)=match shape{SlideShape::TextBox{frame,..}=>("textBox",frame),SlideShape::Picture{frame,..}=>("picture",frame),SlideShape::Table{frame,..}=>("table",frame),SlideShape::Placeholder{frame,..}=>("placeholder",frame)};let id=p.insert_float("semio_presentation_shape",&[owner.0.map(Cell::Integer).unwrap_or(Cell::Null),owner.1.map(Cell::Integer).unwrap_or(Cell::Null),owner.2.map(Cell::Integer).unwrap_or(Cell::Null),Cell::Integer(number(ordinal)?),Cell::Text(kind),Cell::Real(frame.origin.x),Cell::Real(frame.origin.y),Cell::Real(frame.width),Cell::Real(frame.height)],float_columns("semio_presentation_shape"))?;match shape{
SlideShape::TextBox{blocks,..}=>{let collection=project_block_collection(blocks,BLOCKS,None,None,p)?;p.insert_key_float("semio_presentation_text_box",id,&[Cell::Integer(collection)],float_columns("semio_presentation_text_box"))?;},
SlideShape::Picture{image,..}=>p.insert_key_float("semio_presentation_picture",id,&[Cell::Text(&image.asset_id),Cell::Text(&image.mime),Cell::Blob(&image.bytes)],float_columns("semio_presentation_picture"))?,
SlideShape::Table{rows,..}=>{p.insert_key_float("semio_presentation_shape_table",id,&[],float_columns("semio_presentation_shape_table"))?;for(ordinal,row)in rows.iter().enumerate(){let rid=p.insert_float("semio_presentation_shape_table_row",&[Cell::Integer(id),Cell::Integer(number(ordinal)?)],float_columns("semio_presentation_shape_table_row"))?;for(ordinal,cell)in row.cells.iter().enumerate(){let collection=project_block_collection(&cell.blocks,BLOCKS,None,None,p)?;p.insert_float("semio_presentation_shape_table_cell",&[Cell::Integer(rid),Cell::Integer(number(ordinal)?),Cell::Integer(collection)],float_columns("semio_presentation_shape_table_cell"))?;}}},
SlideShape::Placeholder{kind,..}=>{let(tag,other)=match kind{PlaceholderKind::Title=>("title",None),PlaceholderKind::Subtitle=>("subtitle",None),PlaceholderKind::Body=>("body",None),PlaceholderKind::Footer=>("footer",None),PlaceholderKind::SlideNumber=>("slideNumber",None),PlaceholderKind::DateTime=>("dateTime",None),PlaceholderKind::Other{value}=>("other",Some(value.as_str()))};p.insert_key_float("semio_presentation_placeholder",id,&[Cell::Text(tag),other.map(Cell::Text).unwrap_or(Cell::Null)],float_columns("semio_presentation_placeholder"))?;}}}Ok(())}/// 🎟️ Admits all typed Presentation cells before native forecasting or materialization.
pub(crate)fn admit_values(snapshot:&SemioPresentationSnapshot,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{semantic::layout(control.limits())?;let mut out=RowWriter::borrowed(control,phase)?;visit_rows(snapshot,&mut out)?;out.finish_borrowed()}
/// 🏛️ Admits the exact authored table and column layout before native ownership.
pub(crate)fn admit_layout(limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::layout(limits)}
/// 📦️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_binary(body:&[u8],control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::binary(body,control,limits)}
/// 📝️ Counts the actual native primitive cells before typed ownership.
pub(crate)fn admit_document(body:&str,control:&mut semio_framework_value::NativeDecodeControl<'_>,limits:store::sqlite_snapshot::SqliteDatabaseLimits)->Result<(),ValueError>{semantic::document(body,control,limits)}

fn number(n:usize)->Result<i64,ValueError>{i64::try_from(n).map_err(|e|ValueError::new(ValueRefusalKind::WorkLimit,e.to_string()))}
fn identity<'a>(row:impl std::borrow::Borrow<SqliteRow<'a>>,n:usize)->Result<(),ValueError>{let row=*row.borrow();if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=n{Err(ValueError::new(ValueRefusalKind::InvalidValue,"invalid presentation row identity or columns"))}else{Ok(())}}
fn optional_integer(row:SqliteRow<'_>,i:usize)->Result<Option<i64>,ValueError>{if row.is_null(i)?{Ok(None)}else{row.integer(i).map(Some)}}
/// 🚫️ Reports an authored Presentation relationship refusal.
fn presentation_invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
/// 🗃️ Binds a declared Presentation family to the shared paid row index.
fn details<'a>(db:&'a SqliteDatabase,name:&str,columns:usize,c:&mut SqliteSnapshotControl<'_>)->Result<RowIndex<'a>,ValueError>{RowIndex::new(db,name,columns,float_columns(name),c,"invalid presentation row identity or columns")}
/// 🪪️ Validates complete literal identities and document ownership through paid positions.
fn names(rows:&RowIndex<'_>,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{rows.unique_text(rows.indices(),3,c,"invalid presentation native identity or owner")?;for &index in rows.indices(){if rows.row(index)?.integer(1)?!=1{return Err(presentation_invalid("invalid presentation native identity or owner"))}c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,rows.len())?;}Ok(())}
/// 🎯️ Selects the sole actual master, layout or slide owner.
fn presentation_owner(row:SqliteRow<'_>)->Result<(u8,Option<i64>),ValueError>{match(optional_integer(row,1)?,optional_integer(row,2)?,optional_integer(row,3)?){(Some(id),None,None)=>Ok((0,Some(id))),(None,Some(id),None)=>Ok((1,Some(id))),(None,None,Some(id))=>Ok((2,Some(id))),_=>Err(presentation_invalid("presentation shape requires one declared owner"))}}
/// 👪️ Pays one declared scalar parent ordering after validating every foreign key.
fn presentation_group(rows:&RowIndex<'_>,parents:&RowIndex<'_>,c:&mut SqliteSnapshotControl<'_>)->Result<Vec<usize>,ValueError>{for &index in rows.indices(){if parents.get(rows.row(index)?.integer(1)?,c)?.is_none(){return Err(presentation_invalid("dangling presentation relationship owner"))}}rows.grouped_by(2,c,"presentation order must be contiguous",|row|Ok((0,Some(row.integer(1)?))))}
/// 🛡️ Retains completed block collections inside their existing typed retirement guard.
struct PresentationReconstruction<'a>{
 shapes:RowIndex<'a>,shape_order:Vec<usize>,text_boxes:RowIndex<'a>,pictures:RowIndex<'a>,tables:RowIndex<'a>,placeholders:RowIndex<'a>,
 rows:RowIndex<'a>,row_order:Vec<usize>,cells:RowIndex<'a>,cell_order:Vec<usize>,
 roots:Vec<i64>,root_order:Vec<usize>,root_used:Vec<u8>,blocks:Owned<Vec<Vec<crate::standards::v1::subsets::document::schema::snapshot::DocBlock>>>
}
impl PresentationReconstruction<'_>{
 /// 🫴️ Transfers one root exactly once from its guarded source-order collection.
 fn take_blocks(&mut self,id:i64,c:&mut SqliteSnapshotControl<'_>)->Result<Vec<crate::standards::v1::subsets::document::schema::snapshot::DocBlock>,ValueError>{
  let(mut low,mut high)=(0,self.root_order.len());while low<high{c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,low,self.root_order.len())?;let middle=low+(high-low)/2;let index=self.root_order[middle];match self.roots[index].cmp(&id){std::cmp::Ordering::Less=>low=middle+1,std::cmp::Ordering::Greater=>high=middle,std::cmp::Ordering::Equal=>{if self.root_used[index]!=0{return Err(presentation_invalid("missing presentation block collection"))}self.root_used[index]=1;return Ok(std::mem::take(&mut self.blocks.get_mut()[index]))}}}Err(presentation_invalid("missing presentation block collection"))
 }
 /// 🖼️ Reconstructs one owner's ordered shapes with guarded picture, table and placeholder fields.
 fn shapes(&mut self,owner:(u8,Option<i64>),c:&mut SqliteSnapshotControl<'_>)->Result<Vec<SlideShape>,ValueError>{
  let range=self.shapes.range_by(&self.shape_order,owner,c,presentation_owner)?;let mut native=Owned::new(semio_framework_os_kernel::sqlite_snapshot::transfer::reserve::<SlideShape>(range.len(),c)?);
  for ordinal in range{let row=self.shapes.take_index(self.shape_order[ordinal],c)?.ok_or_else(||presentation_invalid("duplicate presentation shape ownership"))?;let id=row.rowid;let frame=SlideFrame{origin:SemioPoint2{x:row.real(6)?,y:row.real(7)?},width:row.real(8)?,height:row.real(9)?};
   let mut shape=match row.text(5)?{
    "textBox"=>{let detail=self.text_boxes.take(id,c)?.ok_or_else(||presentation_invalid("missing presentation text box fields"))?;Owned::new(SlideShape::TextBox{frame,blocks:self.take_blocks(detail.integer(1)?,c)?})},
    "picture"=>{let detail=self.pictures.take(id,c)?.ok_or_else(||presentation_invalid("missing presentation picture fields"))?;let mut shape=Owned::new(SlideShape::Picture{frame,image:SlidePictureImage::default()});if let SlideShape::Picture{image,..}=shape.get_mut(){image.asset_id=reconstruct_text(c,detail.text(1)?)?;image.mime=reconstruct_text(c,detail.text(2)?)?;image.bytes=reconstruct_blob(c,detail.blob(3)?)?;}shape},
    "table"=>{self.tables.take(id,c)?.ok_or_else(||presentation_invalid("missing presentation table fields"))?;let range=self.rows.range_by(&self.row_order,(0,Some(id)),c,|row|Ok((0,Some(row.integer(1)?))))?;let mut shape=Owned::new(SlideShape::Table{frame,rows:Vec::new()});if let SlideShape::Table{rows,..}=shape.get_mut(){*rows=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(range.len(),c)?;for ordinal in range{let row=self.rows.take_index(self.row_order[ordinal],c)?.ok_or_else(||presentation_invalid("duplicate presentation table row"))?;let range=self.cells.range_by(&self.cell_order,(0,Some(row.rowid)),c,|row|Ok((0,Some(row.integer(1)?))))?;let mut native_row=Owned::new(SlideTableRow::default());native_row.get_mut().cells=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(range.len(),c)?;for ordinal in range{let cell=self.cells.take_index(self.cell_order[ordinal],c)?.ok_or_else(||presentation_invalid("duplicate presentation table cell"))?;native_row.get_mut().cells.push(SlideTableCell{blocks:self.take_blocks(cell.integer(3)?,c)?});}rows.push(native_row.take());}}shape},
    "placeholder"=>{let detail=self.placeholders.take(id,c)?.ok_or_else(||presentation_invalid("missing presentation placeholder fields"))?;let other=detail.optional_text(2)?;let mut shape=Owned::new(SlideShape::Placeholder{frame,kind:PlaceholderKind::Title});if let SlideShape::Placeholder{kind,..}=shape.get_mut(){*kind=match(detail.text(1)?,other){("title",None)=>PlaceholderKind::Title,("subtitle",None)=>PlaceholderKind::Subtitle,("body",None)=>PlaceholderKind::Body,("footer",None)=>PlaceholderKind::Footer,("slideNumber",None)=>PlaceholderKind::SlideNumber,("dateTime",None)=>PlaceholderKind::DateTime,("other",Some(value))=>PlaceholderKind::Other{value:reconstruct_text(c,value)?},_=>return Err(presentation_invalid("invalid presentation placeholder shape"))};}shape},
    _=>return Err(presentation_invalid("unknown presentation shape kind"))
   };c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,native.get_mut().len(),self.shapes.len())?;native.get_mut().push(shape.take());
  }Ok(native.take())
 }
}
impl ArtifactSqliteSnapshot for SemioPresentationSnapshot{
fn retire_sqlite_snapshot(self){drop(crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::Owned::new(self));}

fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io::IoPayload,ValueError>{crate::standards::v1::subsets::presentation::io::sqlite::snapshot::native_encoding::encode(self,encoding,control)}
fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{crate::standards::v1::subsets::presentation::io::sqlite::snapshot::native_decoding::decode(payload,control)}
fn preflight_sqlite_snapshot_encoding(&self,_encoding:semio_framework_os_kernel::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{(|| -> Result<(),ValueError>{admit_values(self,SqliteSnapshotPhase::EncodeNative,control)?;let mut b=Bound::file_only("",control)?;self.native_fields(&mut b)?;b.finish()})()}

fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->semio_framework_os_kernel::io_schema::IoResult<()>{(|| -> Result<semio_framework_os_kernel::io_schema::IoOutcome<()>,ValueError>{
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
if dialect.artifact_kind!="s.stdio.semio"||dialect.standard!="v1"||(dialect.subset!="*"&&dialect.subset!="presentation"){return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned snapshot dialect differs from its dedicated semantic subset"));}
let row=database.table("semio_presentation_document")?.single_row()?;
if row.rowid!=1||row.integer(0)?!=1||row.text(1)?!=self.schema{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"Semio owned document identity differs from projected semantic fields"));}
control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(semio_framework_os_kernel::io_schema::IoOutcome::clean(()))})().map_err(semio_framework_os_kernel::io_schema::IoError::from_value_error)}

const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.project_sqlite_database(c)}
fn from_sqlite_database(db:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError> {semantic::layout(c.limits())?;Self::reconstruct_sqlite_database(db, c, Self::SQLITE_SCHEMA)}
}

impl SemioPresentationSnapshot {
    /// 🧩️ Projects owned semantic fields with typed relational refusals.
    pub fn project_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
semantic::layout(c.limits())?;let mut out=RowWriter::new(Self::SQLITE_SCHEMA,c)?;visit_rows(self,&mut out)?;out.finish()}

    /// 🧩️ Restores the owned typed subset inside its independently declared relational composition.
    pub fn reconstruct_sqlite_database(db:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>, declared_schema: &str)->Result<Self,ValueError> {
c.check_database(db,SqliteSnapshotPhase::ReconstructSnapshot)?;semio_framework_os_kernel::sqlite_snapshot::validate_sqlite_database_schema_controlled(db,declared_schema,SqliteSnapshotPhase::ReconstructSnapshot,c)?;let doc=single_float_row(db,"semio_presentation_document")?;identity(doc,2)?;if doc.rowid!=1{return Err(presentation_invalid("invalid presentation document identity"))}
let masters=details(db,"semio_presentation_master",4,c)?;let layouts=details(db,"semio_presentation_layout",5,c)?;let slides=details(db,"semio_presentation_slide",6,c)?;names(&masters,c)?;names(&layouts,c)?;names(&slides,c)?;
let master_order=masters.ordered(2,c,"presentation order must be contiguous")?;let layout_order=layouts.ordered(2,c,"presentation order must be contiguous")?;let slide_order=slides.ordered(2,c,"presentation order must be contiguous")?;
let shapes=details(db,"semio_presentation_shape",10,c)?;for &index in shapes.indices(){let owner=presentation_owner(shapes.row(index)?)?;let rows=match owner.0{0=>&masters,1=>&layouts,_=>&slides};if rows.get(owner.1.ok_or_else(||presentation_invalid("missing presentation shape owner"))?,c)?.is_none(){return Err(presentation_invalid("presentation shape requires one declared owner"))}}
let shape_order=shapes.grouped_by(4,c,"presentation order must be contiguous",presentation_owner)?;
let text_boxes=details(db,"semio_presentation_text_box",2,c)?;let pictures=details(db,"semio_presentation_picture",4,c)?;let tables=details(db,"semio_presentation_shape_table",1,c)?;let placeholders=details(db,"semio_presentation_placeholder",3,c)?;let rows=details(db,"semio_presentation_shape_table_row",3,c)?;let row_order=presentation_group(&rows,&tables,c)?;let cells=details(db,"semio_presentation_shape_table_cell",4,c)?;let cell_order=presentation_group(&cells,&rows,c)?;
let total=text_boxes.len().checked_add(cells.len()).and_then(|count|count.checked_add(slides.len())).ok_or_else(||presentation_invalid("presentation root extent overflow"))?;
let mut roots=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(total,c)?;
for &index in text_boxes.indices(){roots.push(text_boxes.row(index)?.integer(1)?);c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,roots.len(),total)?;}
for &index in cells.indices(){roots.push(cells.row(index)?.integer(3)?);c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,roots.len(),total)?;}
for &index in &slide_order{roots.push(slides.row(index)?.integer(5)?);c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,roots.len(),total)?;}
let mut root_order=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(total,c)?;let mut root_used=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(total,c)?;for index in 0..total{root_order.push(index);root_used.push(0u8);c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,total)?;}
semio_framework_os_kernel::sqlite_snapshot::transfer::heap_sort(&mut root_order,SqliteSnapshotPhase::ReconstructSnapshot,c,|a,b,_|Ok(roots[*a].cmp(&roots[*b])))?;
let mut snapshot=Owned::new(Self{schema:String::new(),masters:Vec::new(),layouts:Vec::new(),slides:Vec::new()});
let blocks=Owned::new(reconstruct_block_collections(db,BLOCKS,&roots,None,None,c)?);
let mut content=PresentationReconstruction{shapes,shape_order,text_boxes,pictures,tables,placeholders,rows,row_order,cells,cell_order,roots,root_order,root_used,blocks};
snapshot.get_mut().masters=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(master_order.len(),c)?;
for index in master_order{let row=masters.row(index)?;let mut master=Owned::new(SlideMaster::default());master.get_mut().id=reconstruct_text(c,row.text(3)?)?;master.get_mut().shapes=content.shapes((0,Some(row.rowid)),c)?;snapshot.get_mut().masters.push(master.take());}
snapshot.get_mut().layouts=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(layout_order.len(),c)?;
for index in layout_order{let row=layouts.row(index)?;let mut layout=Owned::new(SlideLayout::default());layout.get_mut().id=reconstruct_text(c,row.text(3)?)?;let master=masters.get(row.integer(4)?,c)?.ok_or_else(||presentation_invalid("dangling presentation layout master"))?;layout.get_mut().master_id=reconstruct_text(c,master.text(3)?)?;layout.get_mut().shapes=content.shapes((1,Some(row.rowid)),c)?;snapshot.get_mut().layouts.push(layout.take());}
snapshot.get_mut().slides=semio_framework_os_kernel::sqlite_snapshot::transfer::reserve(slide_order.len(),c)?;
for index in slide_order{let row=slides.row(index)?;let mut slide=Owned::new(Slide::default());slide.get_mut().id=reconstruct_text(c,row.text(3)?)?;slide.get_mut().layout_id=optional_integer(row,4)?.map(|id|{let layout=layouts.get(id,c)?.ok_or_else(||presentation_invalid("dangling presentation slide layout"))?;reconstruct_text(c,layout.text(3)?)}).transpose()?;slide.get_mut().shapes=content.shapes((2,Some(row.rowid)),c)?;slide.get_mut().notes=content.take_blocks(row.integer(5)?,c)?;snapshot.get_mut().slides.push(slide.take());}
if [content.shapes.remaining(),content.text_boxes.remaining(),content.pictures.remaining(),content.tables.remaining(),content.placeholders.remaining(),content.rows.remaining(),content.cells.remaining()].iter().any(|count|*count!=0){return Err(presentation_invalid("orphan or contradictory presentation shape detail"))}
for(index,used)in content.root_used.iter().enumerate(){c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index,content.root_used.len())?;if *used==0{return Err(presentation_invalid("orphan presentation content"))}}
snapshot.get_mut().schema=reconstruct_text(c,doc.text(1)?)?;c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;Ok(snapshot.take())

    }
}

fn float_columns(table:&str)->&'static [FloatColumn]{match table{"semio_presentation_shape"=>&[FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9)],"semio_presentation_run"=>&[FloatColumn::Binary64(7)],"semio_presentation_image_block"=>&[FloatColumn::Binary64(3),FloatColumn::Binary64(4)],_=>&[]}}
fn single_float_row<'a>(db:&'a SqliteDatabase,table:&str)->Result<SqliteRow<'a>,ValueError>{SqliteRow::new(db.table(table)?.single_row()?,float_columns(table))}

impl SemioPresentationSnapshot{
/// 📏️ Bounds explicitly owned native fields before encoding.
pub fn native_fields(&self,b:&mut Bound<'_,'_>)->Result<(),ValueError>{b.text(&self.schema)?;b.entities(self.masters.len())?;for master in &self.masters{b.text(&master.id)?;native_shapes(&master.shapes,b)?;}b.entities(self.layouts.len())?;for layout in &self.layouts{b.text(&layout.id)?;b.text(&layout.master_id)?;native_shapes(&layout.shapes,b)?;}b.entities(self.slides.len())?;for slide in &self.slides{b.text(&slide.id)?;b.optional_text(slide.layout_id.as_deref())?;native_shapes(&slide.shapes,b)?;crate::standards::v1::subsets::document::io::sqlite::snapshot::native_blocks(&slide.notes,b)?;}Ok(())}
}

fn native_shapes(shapes:&[SlideShape],b:&mut Bound<'_, '_>)->Result<(),ValueError>{b.entities(shapes.len())?;for shape in shapes{b.scalars(4)?;match shape{SlideShape::TextBox{blocks,..}=>crate::standards::v1::subsets::document::io::sqlite::snapshot::native_blocks(blocks,b)?,SlideShape::Picture{image,..}=>{b.text(&image.asset_id)?;b.text(&image.mime)?;b.bytes(&image.bytes)?;},SlideShape::Table{rows,..}=>{b.entities(rows.len())?;for row in rows{b.entities(row.cells.len())?;for cell in &row.cells{crate::standards::v1::subsets::document::io::sqlite::snapshot::native_blocks(&cell.blocks,b)?;}}},SlideShape::Placeholder{kind,..}=>{b.scalars(1)?;if let PlaceholderKind::Other{value}=kind{b.text(value)?;}}}}Ok(())}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
pub(crate) mod tests;


#[path = "🛫️native/🦀️.rs"]
pub(crate) mod native_encoding;

#[path = "🛬️native/🦀️.rs"]
pub(crate) mod native_decoding;
