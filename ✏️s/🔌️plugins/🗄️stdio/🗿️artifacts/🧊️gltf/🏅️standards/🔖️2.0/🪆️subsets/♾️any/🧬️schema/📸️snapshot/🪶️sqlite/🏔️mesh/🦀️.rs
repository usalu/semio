//! 🏔️ Explicit meshes, primitives, attribute pairs and morph targets preserve every occurrence.
use super::*;
const VALUE:&[FloatColumn]=&[FloatColumn::Binary64(3)];
fn attributes(write:&mut Write<'_,'_>,table:&'static str,owner:i64,values:&[(String,usize)])->Result<(),String>{write.check(values.len())?;for(position,(name,accessor))in values.iter().enumerate(){let [high,low]=index(*accessor);write.insert(table,&[Cell::Integer(owner),ordinal(position)?,Cell::Text(name),high,low])?;}Ok(())}
fn read_attributes(read:&mut Read<'_,'_,'_>,table:&'static str,owner:i64)->Result<Vec<(String,usize)>,String>{let rows=read.rows(table,1,owner,2)?;let mut result=Vec::with_capacity(rows.len());for row in rows{result.push((read.text(row,3)?,read.index(row,4)?));}Ok(result)}
pub(super) fn project(write:&mut Write<'_,'_>,meshes:&[GltfMesh])->Result<(),String>{
 write.check(meshes.len())?;for(position,mesh)in meshes.iter().enumerate(){let [extension,extra]=write.extras(&mesh.extensions,&mesh.extras)?;let owner=write.insert("gltf_mesh",&[Cell::Integer(1),ordinal(position)?,text(&mesh.name),extension,extra])?;
  write.check(mesh.weights.len())?;for(position,value)in mesh.weights.iter().enumerate(){write.floats("gltf_mesh_weight",&[Cell::Integer(owner),ordinal(position)?,Cell::Real(*value)],VALUE)?;}
  write.check(mesh.primitives.len())?;for(position,primitive)in mesh.primitives.iter().enumerate(){let [indices_high,indices_low]=optional_index(primitive.indices);let [material_high,material_low]=optional_index(primitive.material);let [mode_high,mode_low]=optional_word(primitive.mode);let [extension,extra]=write.extras(&primitive.extensions,&primitive.extras)?;let key=write.insert("gltf_primitive",&[Cell::Integer(owner),ordinal(position)?,indices_high,indices_low,material_high,material_low,mode_high,mode_low,extension,extra])?;
   attributes(write,"gltf_primitive_attribute",key,&primitive.attributes)?;
   write.check(primitive.targets.len())?;for(position,target)in primitive.targets.iter().enumerate(){let target_id=write.insert("gltf_morph_target",&[Cell::Integer(key),ordinal(position)?])?;attributes(write,"gltf_morph_attribute",target_id,&target.0)?;}
  }
 }Ok(())
}
pub(super) fn reconstruct(read:&mut Read<'_,'_,'_>)->Result<Vec<GltfMesh>,String>{
 let rows=read.rows("gltf_mesh",1,1,2)?;let mut meshes=Vec::with_capacity(rows.len());for row in rows{let owner=row.rowid;let name=read.optional_text(row,3)?;let extensions=read.json(row,4)?;let extras=read.json(row,5)?;
  let rows=read.rows("gltf_mesh_weight",1,owner,2)?;let mut weights=Vec::with_capacity(rows.len());for row in rows{read.scalar()?;weights.push(FloatRow::new(row,VALUE)?.real(3)?);}
  let rows=read.rows("gltf_primitive",1,owner,2)?;let mut primitives=Vec::with_capacity(rows.len());for row in rows{let key=row.rowid;let indices=read.optional_index(row,3)?;let material=read.optional_index(row,5)?;let mode=read.optional_word(row,7)?;let extensions=read.json(row,9)?;let extras=read.json(row,10)?;let attributes=read_attributes(read,"gltf_primitive_attribute",key)?;let rows=read.rows("gltf_morph_target",1,key,2)?;let mut targets=Vec::with_capacity(rows.len());for row in rows{targets.push(GltfMorphTarget(read_attributes(read,"gltf_morph_attribute",row.rowid)?));}primitives.push(GltfPrimitive{attributes,indices,material,mode,targets,extensions,extras});}
  meshes.push(GltfMesh{primitives,weights,name,extensions,extras});
 }Ok(meshes)
}
