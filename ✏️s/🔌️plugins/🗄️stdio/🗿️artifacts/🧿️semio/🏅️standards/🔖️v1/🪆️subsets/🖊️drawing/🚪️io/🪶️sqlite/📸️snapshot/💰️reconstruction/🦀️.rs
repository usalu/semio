//! 💰️ Original Drawing fields and scalar relational frontiers survive every cancellation boundary.
use super::*;
use semio_framework_os_kernel::sqlite_snapshot::{transfer,artifact::{reconstruct_text_into,reconstruct_blob_into}};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::workspace;
#[derive(semio_framework_value::RetireOwned)]
enum Task{Node(i64),Group{position:usize,start:usize,end:usize}}
#[derive(semio_framework_value::RetireOwned)]
struct Prefix{snapshot:Option<SemioDrawingSnapshot>,validation:transfer::SchemaValidationStorage,rows:indexed::Storage,styles:Vec<usize>,layers:Vec<usize>,names:Vec<usize>,links:Vec<usize>,built:Vec<Option<DrawNode>>,pending:Vec<Task>}
impl Prefix{fn empty()->Self{Self{snapshot:Some(native_decoding::empty()),validation:transfer::SchemaValidationStorage::empty(),rows:indexed::Storage::empty(),styles:Vec::new(),layers:Vec::new(),names:Vec::new(),links:Vec::new(),built:Vec::new(),pending:Vec::new()}}}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn extent(left:usize,right:usize)->Result<usize,ValueError>{left.checked_add(right).ok_or_else(||ValueError::literal(ValueRefusalKind::OwnershipLimit,"drawing reconstruction frontier overflow"))}
fn push(values:&mut Vec<Task>,value:Task)->Result<(),ValueError>{if values.len()==values.capacity(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"drawing task backing was not admitted"))}values.push(value);Ok(())}
fn style_into(output:&mut Option<String>,row:SqliteRow<'_>,column:usize,rows:&indexed::Rows<'_,'_>,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 if row.is_null(column)?{return Ok(())}let position=rows.position("semio_drawing_style",row.integer(column)?)?;let name=rows.get("semio_drawing_style",position)?.text(3)?;*output=Some(String::new());reconstruct_text_into(output.as_mut().unwrap(),control,name)
}
fn unique_names(rows:&indexed::Rows<'_,'_>,table:&str,order:&[usize],scratch:&mut Vec<usize>,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
 scratch.clear();if scratch.capacity()<order.len(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"drawing name backing was not admitted"))}scratch.extend_from_slice(order);
 transfer::heap_sort(scratch,SqliteSnapshotPhase::ReconstructSnapshot,control,|left,right,control|{let left=rows.get(table,*left)?.text(3)?;let right=rows.get(table,*right)?.text(3)?;let order=transfer::compare_text(left,right,SqliteSnapshotPhase::ReconstructSnapshot,control)?;if order.is_eq(){return Err(invalid("duplicate drawing name identity"))}Ok(order)})
}
fn segment(rows:&mut indexed::Rows<'_,'_>,row:SqliteRow<'_>)->Result<PathSegment,ValueError>{
 let id=row.rowid;Ok(match row.text(3)?{
  "move"=>PathSegment::MoveTo{to:point(rows.take("semio_drawing_move",id)?,1)?},
  "line"=>PathSegment::LineTo{to:point(rows.take("semio_drawing_line",id)?,1)?},
  "cubic"=>{let row=rows.take("semio_drawing_cubic",id)?;PathSegment::CubicTo{c1:point(row,1)?,c2:point(row,3)?,to:point(row,5)?}},
  "quad"=>{let row=rows.take("semio_drawing_quad",id)?;PathSegment::QuadTo{c:point(row,1)?,to:point(row,3)?}},
  "arc"=>{let row=rows.take("semio_drawing_arc",id)?;PathSegment::ArcTo{rx:row.real(1)?,ry:row.real(2)?,x_rotation:row.real(3)?,large_arc:boolean(row.integer(4)?)?,sweep:boolean(row.integer(5)?)?,to:point(row,6)?}},
  "close"=>{rows.take("semio_drawing_close",id)?;PathSegment::Close},
  _=>return Err(invalid("unknown drawing segment kind"))
 })
}
pub(super)fn reconstruct(db:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,schema:&str)->Result<SemioDrawingSnapshot,ValueError>{
 workspace::reconstruct_projected(control,Prefix::empty,|prefix,control|reconstruct_into(prefix,db,control,schema),|prefix|prefix.snapshot.take().expect("complete Drawing snapshot"))
}
fn reconstruct_into(prefix:&mut Prefix,db:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>,schema:&str)->Result<(),ValueError>{
 control.check_database(db,SqliteSnapshotPhase::ReconstructSnapshot)?;
 transfer::validate_database_into(&mut prefix.validation,db,schema,SqliteSnapshotPhase::ReconstructSnapshot,control)?;
 transfer::validate_component_into(&mut prefix.validation,db,<SemioDrawingSnapshot as ArtifactSqliteSnapshot>::SQLITE_SCHEMA,SqliteSnapshotPhase::ReconstructSnapshot,control)?;
 let Prefix{snapshot,validation:_,rows:storage,styles,layers,names,links,built,pending}=prefix;
 let snapshot=snapshot.as_mut().expect("original Drawing snapshot");let mut rows=indexed::Rows::new(db,storage,control)?;
 if rows.len("semio_drawing_document")?!=1{return Err(invalid("drawing document must be a singleton"))}let doc=rows.take("semio_drawing_document",1)?;
 let style_count=rows.len("semio_drawing_style")?;let layer_count=rows.len("semio_drawing_layer")?;let count=rows.len("semio_drawing_node")?;let child_count=rows.len("semio_drawing_child")?;let segment_count=rows.len("semio_drawing_segment")?;
 *styles=transfer::reserve(style_count,control)?;*layers=transfer::reserve(layer_count,control)?;*names=transfer::reserve(style_count.max(layer_count),control)?;
 rows.children_into("semio_drawing_style",1,styles,control)?;rows.children_into("semio_drawing_layer",1,layers,control)?;
 unique_names(&rows,"semio_drawing_style",styles,names,control)?;unique_names(&rows,"semio_drawing_layer",layers,names,control)?;
 *links=transfer::reserve(extent(child_count,segment_count)?,control)?;*built=transfer::reserve(count,control)?;for _ in 0..count{built.push(None);}
 *pending=transfer::reserve(extent(extent(count,child_count)?,layer_count)?,control)?;for position in layers.iter().rev(){push(pending,Task::Node(rows.get("semio_drawing_layer",*position)?.integer(6)?))?;}
 while let Some(task)=pending.pop(){
  control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,count)?;
  match task{
   Task::Node(id)=>{
    let position=rows.position("semio_drawing_node",id)?;let node=rows.take("semio_drawing_node",id)?;if built[position].is_some(){return Err(invalid("drawing node reconstructed twice"))}
    match node.text(1)?{
     "path"=>{
      built[position]=Some(DrawNode::Path{segments:Vec::new(),style:None});
      let path=rows.take("semio_drawing_path",id)?;let(start,end)=rows.children_into("semio_drawing_segment",id,links,control)?;
      let DrawNode::Path{segments,style}=built[position].as_mut().unwrap()else{unreachable!()};
      *segments=transfer::reserve(end-start,control)?;
      for index in start..end{let row=rows.get("semio_drawing_segment",links[index])?;segments.push(segment(&mut rows,row)?);control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index-start,end-start)?;}
      style_into(style,path,1,&rows,control)?;
     },
     "text"=>{
      built[position]=Some(DrawNode::Text{value:String::new(),at:SemioPoint2::default(),style:None});
      let row=rows.take("semio_drawing_text",id)?;let DrawNode::Text{value,at,style}=built[position].as_mut().unwrap()else{unreachable!()};
      reconstruct_text_into(value,control,row.text(1)?)?;*at=point(row,2)?;style_into(style,row,4,&rows,control)?;
     },
     "image"=>{
      built[position]=Some(DrawNode::Image{at:SemioPoint2::default(),width:0.0,height:0.0,mime:String::new(),bytes:Vec::new()});
      let row=rows.take("semio_drawing_image",id)?;let DrawNode::Image{at,width,height,mime,bytes}=built[position].as_mut().unwrap()else{unreachable!()};
      *at=point(row,1)?;*width=row.real(3)?;*height=row.real(4)?;reconstruct_text_into(mime,control,row.text(5)?)?;reconstruct_blob_into(bytes,control,row.blob(6)?)?;
     },
     "group"=>{
      built[position]=Some(DrawNode::Group{transform:SemioTransform::identity(),children:Vec::new()});
      let row=rows.take("semio_drawing_group",id)?;let(start,end)=rows.children_into("semio_drawing_child",id,links,control)?;
      let DrawNode::Group{transform:target,children}=built[position].as_mut().unwrap()else{unreachable!()};*target=transform(row)?;*children=transfer::reserve(end-start,control)?;
      push(pending,Task::Group{position,start,end})?;
      for index in(start..end).rev(){push(pending,Task::Node(rows.get("semio_drawing_child",links[index])?.integer(3)?))?;}
     },
     _=>return Err(invalid("unknown drawing node kind"))
    }
   },
   Task::Group{position,start,end}=>{
    for index in start..end{
     control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index-start,end-start)?;
     let child=rows.position("semio_drawing_node",rows.get("semio_drawing_child",links[index])?.integer(3)?)?;
     if child==position{return Err(invalid("cyclic drawing child"))}
     let node=built[child].take().ok_or_else(||invalid("missing owned drawing child"))?;
     let DrawNode::Group{children,..}=built[position].as_mut().expect("original Drawing group")else{unreachable!()};children.push(node);
    }
   }
  }
 }
 snapshot.styles=transfer::reserve(style_count,control)?;
 for position in styles.iter(){
  let row=rows.get("semio_drawing_style",*position)?;snapshot.styles.push(DrawStyle{name:String::new(),fill:None,stroke:None,stroke_width:None,opacity:None});let target=snapshot.styles.last_mut().unwrap();
  reconstruct_text_into(&mut target.name,control,row.text(3)?)?;target.fill=rgba(row,4)?;target.stroke=rgba(row,8)?;target.stroke_width=optional_real(row,12)?;target.opacity=if row.is_null(13)?{None}else{Some(row.binary32(13)?)};
  control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,*position,style_count)?;
 }
 snapshot.layers=transfer::reserve(layer_count,control)?;
 for position in layers.iter(){
  let row=rows.get("semio_drawing_layer",*position)?;let root=rows.position("semio_drawing_node",row.integer(6)?)?;
  snapshot.layers.push(DrawLayer{id:String::new(),name:String::new(),visible:false,root:DrawNode::Group{transform:SemioTransform::identity(),children:Vec::new()}});
  let target=snapshot.layers.last_mut().unwrap();reconstruct_text_into(&mut target.id,control,row.text(3)?)?;reconstruct_text_into(&mut target.name,control,row.text(4)?)?;target.visible=boolean(row.integer(5)?)?;
  target.root=built[root].take().ok_or_else(||invalid("missing drawing layer root"))?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,*position,layer_count)?;
 }
 if built.iter().any(Option::is_some){return Err(invalid("orphan reconstructed drawing node"))}rows.finish(control)?;
 reconstruct_text_into(&mut snapshot.schema,control,doc.text(1)?)?;snapshot.canvas=DrawCanvas{width:doc.real(2)?,height:doc.real(3)?,background:rgba(doc,4)?};
 control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,count)
}
                                                                                                                        