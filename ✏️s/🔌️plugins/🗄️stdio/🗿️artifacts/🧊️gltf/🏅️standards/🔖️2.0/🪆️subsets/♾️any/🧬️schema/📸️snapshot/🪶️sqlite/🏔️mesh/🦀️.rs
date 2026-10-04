//! 🏔️ Explicit meshes, primitives, attribute pairs and morph targets preserve every occurrence.
use super::*;
const VALUE:&[FloatColumn]=&[FloatColumn::Binary64(3)];
fn attributes(write:&mut Write<'_,'_>,table:&'static str,owner:i64,values:&[(String,usize)])->Result<(), ValueError>{write.check(values.len())?;for(position,(name,accessor))in values.iter().enumerate(){let [high,low]=index(*accessor);write.insert(table,&[Cell::Integer(owner),ordinal(position)?,Cell::Text(name),high,low])?;}Ok(())}
fn read_attributes(read:&mut Read<'_,'_,'_>,table:&'static str,owner:i64)->Result<Vec<(String,usize)>, ValueError>{let rows=read.rows(table,1,owner,2)?;let mut result=Vec::with_capacity(rows.len());for row in rows{result.push((read.text(row,3)?,read.index(row,4)?));}Ok(result)}
pub(super) fn project(write:&mut Write<'_,'_>,meshes:&[GltfMesh])->Result<(), ValueError>{
 write.check(meshes.len())?;for(position,mesh)in meshes.iter().enumerate(){let [extension,extra]=write.extras(&mesh.extensions,&mesh.extras)?;let owner=write.insert("gltf_mesh",&[Cell::Integer(1),ordinal(position)?,text(&mesh.name),extension,extra])?;
  write.check(mesh.weights.len())?;for(position,value)in mesh.weights.iter().enumerate(){write.floats("gltf_mesh_weight",&[Cell::Integer(owner),ordinal(position)?,Cell::Real(*value)],VALUE)?;}
  write.check(mesh.primitives.len())?;for(position,primitive)in mesh.primitives.iter().enumerate(){let [indices_high,indices_low]=optional_index(primitive.indices);let [material_high,material_low]=optional_index(primitive.material);let [mode_high,mode_low]=optional_word(primitive.mode);let [extension,extra]=write.extras(&primitive.extensions,&primitive.extras)?;let key=write.insert("gltf_primitive",&[Cell::Integer(owner),ordinal(position)?,indices_high,indices_low,material_high,material_low,mode_high,mode_low,extension,extra])?;
   attributes(write,"gltf_primitive_attribute",key,&primitive.attributes)?;
   write.check(primitive.targets.len())?;for(position,target)in primitive.targets.iter().enumerate(){let target_id=write.insert("gltf_morph_target",&[Cell::Integer(key),ordinal(position)?])?;attributes(write,"gltf_morph_attribute",target_id,&target.0)?;}
  }
 }Ok(())
}
pub(super) fn reconstruct(read:&mut Read<'_,'_,'_>)->Result<Vec<GltfMesh>, ValueError>{
 let rows=read.rows("gltf_mesh",1,1,2)?;let mut meshes=owned(Vec::with_capacity(rows.len()));for row in rows{let key=row.rowid;let mut mesh=owned(GltfMesh::default());let value=mesh.as_mut();value.name=read.optional_text(row,3)?;value.extensions=read.json(row,4)?;value.extras=read.json(row,5)?;
  let rows=read.rows("gltf_mesh_weight",1,key,2)?;value.weights.reserve(rows.len());for row in rows{read.scalar()?;value.weights.push(FloatRow::new(row,VALUE)?.real(3)?);}
  let rows=read.rows("gltf_primitive",1,key,2)?;value.primitives.reserve(rows.len());for row in rows{let key=row.rowid;let mut primitive=owned(GltfPrimitive::default());let current=primitive.as_mut();current.indices=read.optional_index(row,3)?;current.material=read.optional_index(row,5)?;current.mode=read.optional_word(row,7)?;current.extensions=read.json(row,9)?;current.extras=read.json(row,10)?;current.attributes=read_attributes(read,"gltf_primitive_attribute",key)?;let rows=read.rows("gltf_morph_target",1,key,2)?;current.targets.reserve(rows.len());for row in rows{current.targets.push(GltfMorphTarget(read_attributes(read,"gltf_morph_attribute",row.rowid)?));}value.primitives.push(primitive.take());}
  meshes.as_mut().push(mesh.take());
 }Ok(meshes.take())
}
