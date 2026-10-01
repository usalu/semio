//! 📄️ GLTF document identity, scenes, ordered extension names and independently retained buffers.
use super::*;
pub(super) fn project(write:&mut Write<'_,'_>,snapshot:&GltfSnapshot)->Result<(),String>{
 let document=&snapshot.document;let [extension,extra]=write.extras(&document.extensions,&document.extras)?;let [high,low]=optional_index(document.scene);
 write.insert_key("gltf_document",1,&[Cell::Text(&snapshot.schema),Cell::Text(match snapshot.source_form{GltfSourceForm::Json=>"json",GltfSourceForm::Glb=>"glb"}),high,low,extension,extra])?;
 let asset=&document.asset;let [extension,extra]=write.extras(&asset.extensions,&asset.extras)?;
 write.insert_key("gltf_asset",1,&[Cell::Integer(1),Cell::Text(&asset.version),text(&asset.generator),text(&asset.copyright),text(&asset.min_version),extension,extra])?;
 write.check(document.scenes.len())?;for(position,scene)in document.scenes.iter().enumerate(){let [extension,extra]=write.extras(&scene.extensions,&scene.extras)?;let owner=write.insert("gltf_scene",&[Cell::Integer(1),ordinal(position)?,text(&scene.name),extension,extra])?;write.check(scene.nodes.len())?;for(position,node)in scene.nodes.iter().enumerate(){let [high,low]=index(*node);write.insert("gltf_scene_node",&[Cell::Integer(owner),ordinal(position)?,high,low])?;}}
 write.check(document.extensions_used.len())?;for(position,name)in document.extensions_used.iter().enumerate(){write.insert("gltf_extension_used",&[Cell::Integer(1),ordinal(position)?,Cell::Text(name)])?;}
 write.check(document.extensions_required.len())?;for(position,name)in document.extensions_required.iter().enumerate(){write.insert("gltf_extension_required",&[Cell::Integer(1),ordinal(position)?,Cell::Text(name)])?;}
 write.check(snapshot.buffers.len())?;for(position,bytes)in snapshot.buffers.iter().enumerate(){let owner=write.insert("gltf_resolved_buffer",&[Cell::Integer(1),ordinal(position)?])?;write.check(bytes.len())?;for(position,byte)in bytes.iter().enumerate(){write.insert("gltf_resolved_byte",&[Cell::Integer(owner),ordinal(position)?,Cell::Integer(i64::from(*byte))])?;}}
 Ok(())
}

pub(super) fn reconstruct(read:&mut Read<'_,'_,'_>)->Result<GltfSnapshot,String>{
 let root=read.key("gltf_document",1)?;let schema=read.text(root,1)?;let source_form=match root.text(2)?{"json"=>GltfSourceForm::Json,"glb"=>GltfSourceForm::Glb,_=>return Err("GLTF source form is unknown".into())};let scene=read.optional_index(root,3)?;let extensions=read.json(root,5)?;let extras=read.json(root,6)?;
 let row=read.key("gltf_asset",1)?;if row.integer(1)?!=1{return Err("GLTF asset owner differs".into());}
 let asset=GltfAsset{version:read.text(row,2)?,generator:read.optional_text(row,3)?,copyright:read.optional_text(row,4)?,min_version:read.optional_text(row,5)?,extensions:read.json(row,6)?,extras:read.json(row,7)?};
 let rows=read.rows("gltf_scene",1,1,2)?;let mut scenes=Vec::with_capacity(rows.len());for row in rows{let name=read.optional_text(row,3)?;let extensions=read.json(row,4)?;let extras=read.json(row,5)?;let children=read.rows("gltf_scene_node",1,row.rowid,2)?;let mut nodes=Vec::with_capacity(children.len());for row in children{nodes.push(read.index(row,3)?);}scenes.push(GltfScene{nodes,name,extensions,extras});}
 let rows=read.rows("gltf_extension_used",1,1,2)?;let mut extensions_used=Vec::with_capacity(rows.len());for row in rows{extensions_used.push(read.text(row,3)?);}
 let rows=read.rows("gltf_extension_required",1,1,2)?;let mut extensions_required=Vec::with_capacity(rows.len());for row in rows{extensions_required.push(read.text(row,3)?);}
 let rows=read.rows("gltf_resolved_buffer",1,1,2)?;let mut buffers=Vec::with_capacity(rows.len());for row in rows{let bytes=read.rows("gltf_resolved_byte",1,row.rowid,2)?;let mut buffer=Vec::with_capacity(bytes.len());for row in bytes{read.scalar()?;buffer.push(u8::try_from(row.integer(3)?).map_err(|error|error.to_string())?);}buffers.push(buffer);}
 Ok(GltfSnapshot{schema,document:GltfDocument{asset,scene,scenes,extensions_used,extensions_required,extensions,extras,..GltfDocument::default()},buffers,source_form})
}
