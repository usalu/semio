//! 🖨️ Authored Raster layers, masks, intrinsic variants and literal child relationships.
use crate::*;
use crate::standards::v1::subsets::any::schema::snapshot::RasterSnapshot;
use store::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,validate_sqlite_database_schema,artifact::{Projection,Cell,FloatColumn,FloatRow,Reconstruction,insert_ieee754}};
use std::collections::BTreeMap;
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn work(message:&str)->ValueError{ValueError::new(ValueRefusalKind::WorkLimit,message)}

const SCHEMA:&str=include_str!("🗄️.sql");
fn ordinal(value:usize)->Result<i64,ValueError>{i64::try_from(value).map_err(|_|work("Raster ordinal overflow"))}
fn scalar64(p:&mut Projection<'_,'_>,value:f64)->Result<i64,ValueError>{insert_ieee754(p,"raster_scalar64",&[Cell::Real(value)],&[FloatColumn::Binary64(1)])}
fn scalar32(p:&mut Projection<'_,'_>,value:f32)->Result<i64,ValueError>{insert_ieee754(p,"raster_scalar32",&[Cell::Float32(value)],&[FloatColumn::Binary32(1)])}
fn optional_unsigned(value:Option<u32>)->Cell<'static>{value.map(|v|Cell::Integer(i64::from(v))).unwrap_or(Cell::Null)}
fn transform(p:&mut Projection<'_,'_>,table:&str,id:i64,t:&RasterTransform)->Result<(),ValueError>{let mut cells=Vec::new();for value in [t.x,t.y,t.a,t.b,t.c,t.d]{cells.push(Cell::Integer(scalar64(p,value)?));}p.insert_key(table,id,&cells)}
fn intrinsic(p:&mut Projection<'_,'_>,root:&semio_framework_value::DslValue)->Result<i64,ValueError>{
 enum Edge<'a>{Item(i64,usize),Member(i64,usize,&'a str)}
 let mut pending=vec![(root,None)];let mut root_id=0;
 while let Some((v,edge))=pending.pop(){p.checkpoint()?;let kind=match v{semio_framework_value::DslValue::Null=>"null",semio_framework_value::DslValue::Bool(_)=>"boolean",semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(_))=>"signed",semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(_))=>"unsigned",semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(_))=>"float",semio_framework_value::DslValue::String(_)=>"text",semio_framework_value::DslValue::Bytes(_)=>"bytes",semio_framework_value::DslValue::Array(_)=>"array",semio_framework_value::DslValue::Object(_)=>"object"};let id=p.insert("raster_value",&[Cell::Text(kind)])?;
 match edge{None=>root_id=id,Some(Edge::Item(parent,i))=>{p.insert("raster_item",&[Cell::Integer(parent),Cell::Integer(ordinal(i)?),Cell::Integer(id)])?;},Some(Edge::Member(parent,i,name))=>{p.insert("raster_member",&[Cell::Integer(parent),Cell::Integer(ordinal(i)?),Cell::Text(name),Cell::Integer(id)])?;}}
 match v{
 semio_framework_value::DslValue::Null=>{},semio_framework_value::DslValue::Bool(v)=>p.insert_key("raster_boolean",id,&[Cell::Integer(i64::from(*v))])?,
 semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(v))=>p.insert_key("raster_signed",id,&[Cell::Integer(*v)])?,
 semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(v))=>p.insert_key("raster_unsigned",id,&[Cell::Integer((v>>32)as i64),Cell::Integer((v&0xffffffff)as i64)])?,
 semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(v))=>{let scalar=scalar64(p,*v)?;p.insert_key("raster_float",id,&[Cell::Integer(scalar)])?;},
 semio_framework_value::DslValue::String(v)=>p.insert_key("raster_text",id,&[Cell::Text(v)])?,semio_framework_value::DslValue::Bytes(v)=>p.insert_key("raster_bytes",id,&[Cell::Blob(v)])?,
 semio_framework_value::DslValue::Array(v)=>{p.insert_key("raster_array",id,&[])?;p.check_rows(pending.len().checked_add(v.len()).ok_or_else(||work("Raster frontier overflow"))?)?;for(i,child)in v.iter().enumerate().rev(){pending.push((child,Some(Edge::Item(id,i))));}},
 semio_framework_value::DslValue::Object(v)=>{p.insert_key("raster_object",id,&[])?;p.check_rows(pending.len().checked_add(v.len()).ok_or_else(||work("Raster frontier overflow"))?)?;for(i,(name,child))in v.iter().enumerate().rev(){pending.push((child,Some(Edge::Member(id,i,name))));}}
 }
 }Ok(root_id)
}
fn project(s:&RasterSnapshot,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
 let mut p=Projection::new(SCHEMA,control)?;p.insert_key("raster_document",1,&[Cell::Text(&s.schema),Cell::Text(&s.id),s.title.as_deref().map(Cell::Text).unwrap_or(Cell::Null)])?;
 p.check_rows(s.assets.len())?;for(i,(key,child))in s.assets.iter().enumerate(){let target=&child.target;let dialect=&target.dialect;p.insert("raster_asset",&[Cell::Integer(1),Cell::Integer(ordinal(i)?),Cell::Text(key),Cell::Text(&child.child_id),Cell::Text(&dialect.artifact_kind),Cell::Text(&dialect.standard),Cell::Text(&dialect.subset),Cell::Text(&target.artifact_id)])?;}
 p.check_rows(s.layers.len())?;let mut pending=Vec::new();for(i,node)in s.layers.iter().enumerate().rev(){pending.push((node,None,i));}
 while let Some((node,parent,i))=pending.pop(){p.checkpoint()?;let(kind,id,name,visible,locked,opacity,blend,t,mask)=match node{
 RasterLayerNode::Pixel{id,name,visible,locked,opacity,blend_mode,transform,mask,..}=>("pixel",id,name,visible,locked,opacity,blend_mode,transform,mask.as_ref()),
 RasterLayerNode::Group{id,name,visible,locked,opacity,blend_mode,transform,mask,..}=>("group",id,name,visible,locked,opacity,blend_mode,transform,mask.as_ref()),
 RasterLayerNode::Adjustment{id,name,visible,locked,opacity,blend_mode,transform,..}=>("adjustment",id,name,visible,locked,opacity,blend_mode,transform,None)};
 let opacity=scalar32(&mut p,*opacity)?;let key=p.insert("raster_layer",&[if parent.is_none(){Cell::Integer(1)}else{Cell::Null},parent.map(Cell::Integer).unwrap_or(Cell::Null),Cell::Integer(ordinal(i)?),Cell::Text(kind),Cell::Text(id),Cell::Text(name),Cell::Integer(i64::from(*visible)),Cell::Integer(i64::from(*locked)),Cell::Integer(opacity),Cell::Text(blend)])?;transform(&mut p,"raster_transform",key,t)?;
 if let Some(mask)=mask{p.insert_key("raster_mask",key,&[Cell::Integer(i64::from(mask.enabled)),Cell::Integer(i64::from(mask.linked)),Cell::Integer(i64::from(mask.invert)),optional_unsigned(mask.width),optional_unsigned(mask.height),mask.image_key.as_deref().map(Cell::Text).unwrap_or(Cell::Null)])?;transform(&mut p,"raster_mask_transform",key,&mask.transform)?;}
 match node{
 RasterLayerNode::Pixel{width,height,image_key,..}=>p.insert_key("raster_pixel",key,&[optional_unsigned(*width),optional_unsigned(*height),image_key.as_deref().map(Cell::Text).unwrap_or(Cell::Null)])?,
 RasterLayerNode::Group{children,..}=>{p.insert_key("raster_group",key,&[])?;p.check_rows(pending.len().checked_add(children.len()).ok_or_else(||work("Raster frontier overflow"))?)?;for(i,child)in children.iter().enumerate().rev(){pending.push((child,Some(key),i));}},
 RasterLayerNode::Adjustment{adjustment_kind,params,..}=>{p.insert_key("raster_adjustment",key,&[Cell::Text(adjustment_kind)])?;p.check_rows(params.len())?;for(i,(name,value))in params.iter().enumerate(){let root=intrinsic(&mut p,value)?;p.insert("raster_parameter",&[Cell::Integer(key),Cell::Integer(ordinal(i)?),Cell::Text(name),Cell::Integer(root)])?;}}
 }
 }p.finish()
}

const TABLES:[(&str,usize);23]=[("raster_document",4),("raster_scalar64",4),("raster_scalar32",4),("raster_asset",9),("raster_layer",11),("raster_transform",7),("raster_mask",7),("raster_mask_transform",7),("raster_pixel",4),("raster_group",1),("raster_adjustment",2),("raster_parameter",5),("raster_value",2),("raster_boolean",2),("raster_signed",2),("raster_unsigned",3),("raster_float",2),("raster_text",2),("raster_bytes",2),("raster_array",1),("raster_item",4),("raster_object",1),("raster_member",5)];
struct Reader<'a,'c,'p>{rows:BTreeMap<&'static str,BTreeMap<i64,&'a SqliteRow>>,groups:BTreeMap<(&'static str,usize),BTreeMap<i64,Vec<&'a SqliteRow>>>,control:&'c mut SqliteSnapshotControl<'p>}
impl<'a,'c,'p> Reader<'a,'c,'p>{
 fn new(database:&'a SqliteDatabase,control:&'c mut SqliteSnapshotControl<'p>)->Result<Self,ValueError>{control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,SCHEMA,control.limits())?;let mut rows=BTreeMap::new();for(name,width)in TABLES{let mut values=BTreeMap::new();for(i,row)in database.table(name)?.rows.iter().enumerate(){if row.rowid<=0||row.values.len()!=width||row.integer(0)?!=row.rowid||values.insert(row.rowid,row).is_some(){return Err(invalid("Raster row identity or fields"));}if i%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,i,0)?;}}rows.insert(name,values);}Ok(Self{rows,groups:BTreeMap::new(),control})}
 fn has(&self,name:&str,id:i64)->bool{self.rows.get(name).is_some_and(|v|v.contains_key(&id))}
 fn take(&mut self,name:&str,id:i64)->Result<&'a SqliteRow,ValueError>{self.rows.get_mut(name).ok_or_else(||invalid("Raster table"))?.remove(&id).ok_or_else(||invalid("Raster dangling or multiply owned entity"))}
 fn null(row:&SqliteRow,index:usize)->Result<bool,ValueError>{Ok(row.values.get(index).ok_or_else(||invalid("Raster column"))?==&SqliteValue::Null)}
 fn text(&mut self,row:&SqliteRow,index:usize)->Result<String,ValueError>{Reconstruction::new(self.control)?.text(row.text(index)?)}
 fn optional_text(&mut self,row:&SqliteRow,index:usize)->Result<Option<String>,ValueError>{if Self::null(row,index)?{Ok(None)}else{self.text(row,index).map(Some)}}
 fn boolean(row:&SqliteRow,index:usize)->Result<bool,ValueError>{match row.integer(index)?{0=>Ok(false),1=>Ok(true),_=>Err(invalid("Raster boolean"))}}
 fn unsigned(row:&SqliteRow,index:usize)->Result<u32,ValueError>{u32::try_from(row.integer(index)?).map_err(|_|invalid("Raster scalar exceeds unsigned32"))}
 fn optional_unsigned(row:&SqliteRow,index:usize)->Result<Option<u32>,ValueError>{if Self::null(row,index)?{Ok(None)}else{Self::unsigned(row,index).map(Some)}}
 fn real(&mut self,row:&SqliteRow,index:usize)->Result<f64,ValueError>{let v=self.take("raster_scalar64",row.integer(index)?)?;Reconstruction::new(self.control)?.scalar()?;FloatRow::new(v,&[FloatColumn::Binary64(1)])?.real(1)}
 fn opacity(&mut self,row:&SqliteRow)->Result<f32,ValueError>{let v=self.take("raster_scalar32",row.integer(9)?)?;Reconstruction::new(self.control)?.scalar()?;FloatRow::new(v,&[FloatColumn::Binary32(1)])?.binary32(1)}
 fn transform(&mut self,name:&str,id:i64)->Result<RasterTransform,ValueError>{let v=self.take(name,id)?;Ok(RasterTransform{x:self.real(v,1)?,y:self.real(v,2)?,a:self.real(v,3)?,b:self.real(v,4)?,c:self.real(v,5)?,d:self.real(v,6)?})}
 fn list(&mut self,name:&'static str,parent:i64,index:usize,order:usize)->Result<Vec<&'a SqliteRow>,ValueError>{let key=(name,index);if !self.groups.contains_key(&key){let source=self.rows.get(name).ok_or_else(||invalid("Raster table"))?;self.control.check_rows(source.len())?;let mut groups=BTreeMap::<i64,Vec<&SqliteRow>>::new();for(i,row)in source.values().enumerate(){if !Self::null(row,index)?{groups.entry(row.integer(index)?).or_default().push(*row);}if i%256==0{self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,i,source.len())?;}}self.groups.insert(key,groups);}let source=self.groups.get_mut(&key).unwrap().remove(&parent).unwrap_or_default();let mut ordered=BTreeMap::new();for row in source{let ordinal=row.integer(order)?;if ordinal<0||ordered.insert(ordinal,row).is_some(){return Err(invalid("Raster duplicate or negative ordinal"));}}let mut rows=Vec::new();for(i,(order,row))in ordered.into_iter().enumerate(){if order!=ordinal(i)?{return Err(invalid("Raster relationship order"));}self.take(name,row.rowid)?;rows.push(row);if i%256==0{self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,i,0)?;}}Ok(rows)}
 fn mask(&mut self,id:i64)->Result<Option<RasterLayerMask>,ValueError>{if !self.has("raster_mask",id){return Ok(None)}let v=self.take("raster_mask",id)?;Ok(Some(RasterLayerMask{enabled:Self::boolean(v,1)?,linked:Self::boolean(v,2)?,invert:Self::boolean(v,3)?,width:Self::optional_unsigned(v,4)?,height:Self::optional_unsigned(v,5)?,image_key:self.optional_text(v,6)?,transform:self.transform("raster_mask_transform",id)?}))}
 fn intrinsic(&mut self,root:i64)->Result<semio_framework_value::DslValue,ValueError>{
 enum Task<'a>{Node(i64),Array(i64,Vec<i64>),Object(i64,Vec<(&'a SqliteRow,i64)>)}
 let mut pending=vec![Task::Node(root)];let mut forest=Values(BTreeMap::new());
 while let Some(task)=pending.pop(){self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;match task{
 Task::Node(id)=>{let row=self.take("raster_value",id)?;let value=match row.text(1)?{
 "null"=>semio_framework_value::DslValue::Null,"boolean"=>semio_framework_value::DslValue::Bool(Self::boolean(self.take("raster_boolean",id)?,1)?),
 "signed"=>semio_framework_value::DslValue::Number(semio_framework_value::Number::Int(self.take("raster_signed",id)?.integer(1)?)),
 "unsigned"=>{let v=self.take("raster_unsigned",id)?;semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt((u64::from(Self::unsigned(v,1)?)<<32)|u64::from(Self::unsigned(v,2)?)))},
 "float"=>{let v=self.take("raster_float",id)?;semio_framework_value::DslValue::Number(semio_framework_value::Number::Float(self.real(v,1)?))},
 "text"=>{let v=self.take("raster_text",id)?;semio_framework_value::DslValue::String(self.text(v,1)?)},
 "bytes"=>{let v=self.take("raster_bytes",id)?;semio_framework_value::DslValue::Bytes(Reconstruction::new(self.control)?.blob(v.blob(1)?)?)},
 "array"=>{self.take("raster_array",id)?;let rows=self.list("raster_item",id,1,2)?;self.control.check_rows(pending.len().checked_add(rows.len()).ok_or_else(||work("Raster intrinsic frontier overflow"))?)?;let ids=rows.iter().map(|v|v.integer(3)).collect::<Result<Vec<_>,_>>()?;pending.push(Task::Array(id,ids));for child in rows.iter().rev(){pending.push(Task::Node(child.integer(3)?));}continue;},
 "object"=>{self.take("raster_object",id)?;let rows=self.list("raster_member",id,1,2)?;self.control.check_rows(pending.len().checked_add(rows.len()).ok_or_else(||work("Raster intrinsic frontier overflow"))?)?;let mut items=Vec::new();for row in &rows{items.push((*row,row.integer(4)?));}pending.push(Task::Object(id,items));for row in rows.into_iter().rev(){pending.push(Task::Node(row.integer(4)?));}continue;},_=>return Err(invalid("Raster intrinsic variant"))};forest.0.insert(id,value);},
 Task::Array(id,children)=>{let mut values=Items(Vec::new());for child in children{values.0.push(forest.0.remove(&child).ok_or_else(||invalid("Raster intrinsic item"))?);}forest.0.insert(id,semio_framework_value::DslValue::Array(std::mem::take(&mut values.0)));},
 Task::Object(id,children)=>{let mut values=Members(Vec::new());for(row,child)in children{let name=self.text(row,3)?;values.0.push((name,forest.0.remove(&child).ok_or_else(||invalid("Raster intrinsic member"))?));}forest.0.insert(id,semio_framework_value::DslValue::Object(std::mem::take(&mut values.0)));}
 }}forest.0.remove(&root).ok_or_else(||invalid("Raster intrinsic root"))
 }
}
struct Values(BTreeMap<i64,semio_framework_value::DslValue>);
impl Drop for Values{fn drop(&mut self){while let Some((_,value))=self.0.pop_last(){<semio_framework_value::DslValue as semio_framework_value::FromValue>::retire_decoded(value);}}}
struct Members(Vec<(String,semio_framework_value::DslValue)>);
impl Drop for Members{fn drop(&mut self){while let Some((_,value))=self.0.pop(){<semio_framework_value::DslValue as semio_framework_value::FromValue>::retire_decoded(value);}}}

struct Items(Vec<semio_framework_value::DslValue>);
impl Drop for Items{fn drop(&mut self){while let Some(value)=self.0.pop(){<semio_framework_value::DslValue as semio_framework_value::FromValue>::retire_decoded(value);}}}
pub(super) fn retire_snapshot(snapshot:RasterSnapshot){
 let mut cursor=store::ArtifactOwnedValueRetirementFactory::retire_owned(&crate::host::owned::RasterSnapshotRetirementFactory,snapshot);
 let mut stalled=0usize;
 loop{match cursor.close_step(256,usize::MAX).expect("Raster owned snapshot retirement"){
 store::SnapshotRetirementStep::Pending{released_items,released_bytes}=>{if released_items==0&&released_bytes==0{stalled+=1;assert!(stalled<256,"Raster cold retirement made no progress");}else{stalled=0;}},
 store::SnapshotRetirementStep::Blocked=>panic!("Raster owned snapshot retirement blocked"),
 store::SnapshotRetirementStep::Complete=>break,
 }}assert!(cursor.terminal_is_empty());
}
pub(super) fn retire_layer(root:RasterLayerNode){retire_snapshot(RasterSnapshot{schema:String::new(),id:String::new(),title:None,layers:vec![root],assets:RasterOwnedMap::new()});}
struct LayerOwned(Option<RasterLayerNode>);
impl Drop for LayerOwned{fn drop(&mut self){if let Some(node)=self.0.take(){retire_layer(node);}}}
struct Forest(BTreeMap<i64,RasterLayerNode>);
impl Drop for Forest{fn drop(&mut self){while let Some((_,node))=self.0.pop_last(){retire_layer(node);}}}
struct Parameters(RasterOwnedMap<semio_framework_value::DslValue>);
impl Drop for Parameters{fn drop(&mut self){while let Some((_,value))=self.0.take_last_entry(){<semio_framework_value::DslValue as semio_framework_value::FromValue>::retire_decoded(value);}self.0.retire();}}
struct SnapshotOwned(Option<RasterSnapshot>);
impl Drop for SnapshotOwned{fn drop(&mut self){if let Some(value)=self.0.take(){retire_snapshot(value);}}}
fn ordered_keys(a:&str,b:&str,control:&mut SqliteSnapshotControl<'_>)->Result<bool,ValueError>{
 let left=a.as_bytes();let right=b.as_bytes();let limit=left.len().min(right.len());let mut position=0;
 control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,limit)?;
 while position<limit{let end=position.saturating_add(65536).min(limit);let order=left[position..end].cmp(&right[position..end]);if order!=std::cmp::Ordering::Equal{return Ok(order==std::cmp::Ordering::Less)}position=end;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,position,limit)?;}
 Ok(left.len()<right.len())
}
impl Reader<'_,'_,'_>{
 fn leaf(&mut self,row:&SqliteRow)->Result<RasterLayerNode,ValueError>{
 if Self::null(row,1)?==Self::null(row,2)?{return Err(invalid("Raster ambiguous forest owner"))}
 let key=row.rowid;let id=self.text(row,5)?;let name=self.text(row,6)?;let visible=Self::boolean(row,7)?;let locked=Self::boolean(row,8)?;let opacity=self.opacity(row)?;let blend_mode=self.text(row,10)?;let transform=self.transform("raster_transform",key)?;let mask=self.mask(key)?;
 Ok(match row.text(4)?{
 "pixel"=>{let v=self.take("raster_pixel",key)?;RasterLayerNode::Pixel{id,name,visible,locked,opacity,blend_mode,transform,mask,width:Self::optional_unsigned(v,1)?,height:Self::optional_unsigned(v,2)?,image_key:self.optional_text(v,3)?}},
 "group"=>{self.take("raster_group",key)?;RasterLayerNode::Group{id,name,visible,locked,opacity,blend_mode,transform,mask,children:Vec::new()}},
 "adjustment"=>{if mask.is_some(){return Err(invalid("Raster adjustment cannot own mask"))}let v=self.take("raster_adjustment",key)?;let adjustment_kind=self.text(v,1)?;let rows=self.list("raster_parameter",key,1,2)?;if rows.len()>64{return Err(invalid("Raster map capacity"))}let mut params=Parameters(RasterOwnedMap::new());let mut previous=None;for row in rows{let key=self.text(row,3)?;if let Some(previous)=previous{if !ordered_keys(previous,row.text(3)?,self.control)?{return Err(invalid("Raster parameter key order"))}}previous=Some(row.text(3)?);let value=self.intrinsic(row.integer(4)?)?;if let Err(rejected)=params.0.insert(key,value){<semio_framework_value::DslValue as semio_framework_value::FromValue>::retire_decoded(rejected.value);return Err(invalid(rejected.reason))}}RasterLayerNode::Adjustment{id,name,visible,locked,opacity,blend_mode,transform,adjustment_kind,params:std::mem::take(&mut params.0)}},
 _=>return Err(invalid("Raster layer variant"))})
 }
}
fn reconstruct(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<RasterSnapshot,ValueError>{
 let mut r=Reader::new(database,control)?;let document=r.take("raster_document",1)?;let mut owner=SnapshotOwned(Some(RasterSnapshot{schema:r.text(document,1)?,id:r.text(document,2)?,title:r.optional_text(document,3)?,layers:Vec::new(),assets:RasterOwnedMap::new()}));
 let assets=r.list("raster_asset",1,1,2)?;if assets.len()>64{return Err(invalid("Raster map capacity"))}let mut previous=None;for row in assets{let key=r.text(row,3)?;if let Some(previous)=previous{if !ordered_keys(previous,row.text(3)?,r.control)?{return Err(invalid("Raster asset key order"))}}previous=Some(row.text(3)?);let child=store::ArtifactChild::new(r.text(row,4)?,semio_framework_artifact_reference::ArtifactRef{dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:r.text(row,5)?,standard:r.text(row,6)?,subset:r.text(row,7)?},artifact_id:r.text(row,8)?});owner.0.as_mut().unwrap().assets.insert(key,child).map_err(|e|invalid(e.reason))?;}
 enum Task<'a>{Node(&'a SqliteRow),Group(i64,Vec<i64>)}
 let roots=r.list("raster_layer",1,1,3)?;let root_ids=roots.iter().map(|v|v.rowid).collect::<Vec<_>>();let mut pending=Vec::new();for row in roots.into_iter().rev(){pending.push(Task::Node(row));}let mut forest=Forest(BTreeMap::new());
 while let Some(task)=pending.pop(){r.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,0)?;match task{
 Task::Node(row)=>{forest.0.insert(row.rowid,r.leaf(row)?);if row.text(4)?=="group"{let children=r.list("raster_layer",row.rowid,2,3)?;r.control.check_rows(pending.len().checked_add(children.len()).ok_or_else(||work("Raster frontier overflow"))?)?;let ids=children.iter().map(|v|v.rowid).collect();pending.push(Task::Group(row.rowid,ids));for row in children.into_iter().rev(){pending.push(Task::Node(row));}}},
 Task::Group(id,ids)=>{let mut node=LayerOwned(Some(forest.0.remove(&id).ok_or_else(||invalid("Raster group owner"))?));let Some(RasterLayerNode::Group{children,..})=node.0.as_mut()else{return Err(invalid("Raster group discriminant"))};for id in ids{children.push(forest.0.remove(&id).ok_or_else(||invalid("Raster child owner"))?);}forest.0.insert(id,node.0.take().unwrap());}
 }}if r.rows.values().any(|v|!v.is_empty()){return Err(invalid("orphan Raster entities"))}r.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)?;for id in root_ids{owner.0.as_mut().unwrap().layers.push(forest.0.remove(&id).ok_or_else(||invalid("Raster root owner"))?);}if !forest.0.is_empty(){return Err(invalid("unowned Raster layer"))}Ok(owner.0.take().unwrap())
}

impl store::ArtifactSqliteSnapshot for RasterSnapshot{
 const SQLITE_SCHEMA:&'static str=SCHEMA;
 fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{project(self,control)}
 fn from_sqlite_database(database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{reconstruct(database,control)}
 fn retire_sqlite_snapshot(self){retire_snapshot(self)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{
  let maximum_rows=control.limits().max_rows;
  store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),crate::standards::v1::subsets::any::io::text::snapshot::record::RasterNativeDocument::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, semio_framework_value::ValueError> = (|| {
   let document=crate::standards::v1::subsets::any::io::text::snapshot::record::RasterNativeDocument::__dsl_from_record_controlled(record,native)?;document.into_snapshot(maximum_rows,native)
  })(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)
 }
 fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{
  let maximum_rows=control.limits().max_rows;
  store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),crate::standards::v1::subsets::any::io::text::snapshot::record::RasterNativeDocument::__dsl_spec_producer(),|native|{
   let document=crate::standards::v1::subsets::any::io::text::snapshot::record::RasterNativeDocument::from_snapshot(self,maximum_rows,native)?;
   document.__dsl_to_record_controlled(native)
  },control,native_owner)
 }

 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,database:&SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{
use semio_framework_artifact_reference::io::text::artifact_reference::{DialectCoordinateText as _};

  let io=store::io_schema::IoError::from_value_error;
  control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,0).map_err(io)?;
  if dialect.artifact_kind!="s.raster.raster"||dialect.standard!="1"||dialect.subset!="*"{return Err(io(invalid(format!("unrecognized Raster semantic dialect {}",dialect.to_coordinate()))))}
  validate_sqlite_database_schema(database,SCHEMA,control.limits()).map_err(io)?;
  Ok(store::io_schema::IoOutcome::clean(()))
 }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
