//! 🌳️ Nodes keep every transform component, source reference and ordered child/weight occurrence.
use super::*;
const VALUE:&[FloatColumn]=&[FloatColumn::Binary64(3)];
fn project_fixed<const N:usize>(write:&mut Write<'_,'_>,table:&'static str,components:&'static str,owner:i64,value:&Option<[f64;N]>)->Result<(),String>{if let Some(values)=value{write.insert_key(table,owner,&[])?;for(position,value)in values.iter().enumerate(){write.floats(components,&[Cell::Integer(owner),ordinal(position)?,Cell::Real(*value)],VALUE)?;}}Ok(())}
fn reconstruct_fixed<const N:usize>(read:&mut Read<'_,'_,'_>,table:&'static str,components:&'static str,owner:i64)->Result<Option<[f64;N]>,String>{if read.optional_key(table,owner)?.is_none(){return Ok(None)}let rows=read.rows(components,1,owner,2)?;if rows.len()!=N{return Err("GLTF fixed transform dimension differs".into());}let mut values=[0.0;N];for(position,row)in rows.into_iter().enumerate(){read.scalar()?;values[position]=FloatRow::new(row,VALUE)?.real(3)?;}Ok(Some(values))}

pub(super) fn project(write:&mut Write<'_,'_>,nodes:&[GltfNode])->Result<(),String>{
 write.check(nodes.len())?;for(position,node)in nodes.iter().enumerate(){let [mesh_high,mesh_low]=optional_index(node.mesh);let [camera_high,camera_low]=optional_index(node.camera);let [skin_high,skin_low]=optional_index(node.skin);let [extension,extra]=write.extras(&node.extensions,&node.extras)?;let owner=write.insert("gltf_node",&[Cell::Integer(1),ordinal(position)?,text(&node.name),mesh_high,mesh_low,camera_high,camera_low,skin_high,skin_low,extension,extra])?;
  write.check(node.children.len())?;for(position,child)in node.children.iter().enumerate(){let [high,low]=index(*child);write.insert("gltf_node_child",&[Cell::Integer(owner),ordinal(position)?,high,low])?;}
  project_fixed(write,"gltf_node_matrix","gltf_node_matrix_component",owner,&node.matrix)?;
  project_fixed(write,"gltf_node_translation","gltf_node_translation_component",owner,&node.translation)?;
  project_fixed(write,"gltf_node_rotation","gltf_node_rotation_component",owner,&node.rotation)?;
  project_fixed(write,"gltf_node_scale","gltf_node_scale_component",owner,&node.scale)?;
  write.check(node.weights.len())?;for(position,value)in node.weights.iter().enumerate(){write.floats("gltf_node_weight",&[Cell::Integer(owner),ordinal(position)?,Cell::Real(*value)],VALUE)?;}
 }Ok(())
}
pub(super) fn reconstruct(read:&mut Read<'_,'_,'_>)->Result<Vec<GltfNode>,String>{
 let rows=read.rows("gltf_node",1,1,2)?;let mut nodes=Vec::with_capacity(rows.len());for row in rows{let owner=row.rowid;let name=read.optional_text(row,3)?;let mesh=read.optional_index(row,4)?;let camera=read.optional_index(row,6)?;let skin=read.optional_index(row,8)?;let extensions=read.json(row,10)?;let extras=read.json(row,11)?;
  let rows=read.rows("gltf_node_child",1,owner,2)?;let mut children=Vec::with_capacity(rows.len());for row in rows{children.push(read.index(row,3)?);}
  let matrix=reconstruct_fixed(read,"gltf_node_matrix","gltf_node_matrix_component",owner)?;
  let translation=reconstruct_fixed(read,"gltf_node_translation","gltf_node_translation_component",owner)?;
  let rotation=reconstruct_fixed(read,"gltf_node_rotation","gltf_node_rotation_component",owner)?;
  let scale=reconstruct_fixed(read,"gltf_node_scale","gltf_node_scale_component",owner)?;
  let rows=read.rows("gltf_node_weight",1,owner,2)?;let mut weights=Vec::with_capacity(rows.len());for row in rows{read.scalar()?;weights.push(FloatRow::new(row,VALUE)?.real(3)?);}
  nodes.push(GltfNode{children,mesh,camera,skin,matrix,translation,rotation,scale,weights,name,extensions,extras});
 }Ok(nodes)
}
