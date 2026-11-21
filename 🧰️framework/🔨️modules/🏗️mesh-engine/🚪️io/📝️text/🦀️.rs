//! 📝️ Mesh JSON and OBJ representation owners.
use crate::*;
use semio_framework_pack_json as json;

/// 🧲️ Encodes canonical polygon values at the native JSON boundary.
pub fn encode_polygon_mesh_source(source:&PolygonMeshSource)->String {json::from_dsl_value(&pack::value::ToValue::to_value(source)).to_string()}

#[path="🧵️obj-source/🦀️.rs"]
mod obj_source;
pub use obj_source::{ObjSourceCursor,polygon_mesh_source_from_obj};

#[path="📤️polygon-json/🦀️.rs"]
mod polygon_json;
pub use polygon_json::PolygonJsonExport;

#[path="🧵️polygon-source/🦀️.rs"]
mod polygon_source;
pub use polygon_source::{PolygonSourceGrant,PolygonSourcePreparation,PolygonSourceStep,parse_polygon_mesh_source};

/// 🎒️ Retained metadata cursor shared by mesh JSON and preview packing.
#[derive(Default,semio_framework_value_derive::RetireOwned)]
pub struct MeshMetadataCursor {
    metadata_section:u8,metadata_stage:u8,metadata_name:Option<String>,metadata_records:usize,metadata_value:usize,metadata_stack:Vec<MeshJsonTask>,
}
impl MeshMetadataCursor {
    /// 👣️ Appends one bounded metadata transition using borrowed canonical values.
    pub fn step(&mut self, attributes:&HistoryFoldIndex<String,MeshAttribute>,materials:&HistoryFoldIndex<String,pack::value::DslValue>,textures:&HistoryFoldIndex<String,MeshTexture>,references:Option<&ComponentReferenceTable>,corners:Option<&[u32]>,edges:Option<&[u32]>,output:&mut String)->Result<bool,String> {
        use std::ops::Bound::{Excluded,Unbounded};
        if attributes.len()>64 || materials.len()>10_000 || textures.len()>256 {return Err("mesh metadata declaration limit exceeded".into());}
        if references.is_some_and(|values|values.len()>3) {return Err("mesh component reference domain limit exceeded".into());}
        if self.metadata_section==4 {return Ok(true); }
        if self.metadata_stage==0 {
            let (field,empty)=match self.metadata_section {0=>("attributes",attributes.is_empty()),1=>("materials",materials.is_empty()),2=>("textures",textures.is_empty()),_=>("componentReferences",references.is_none_or(ComponentReferenceTable::is_empty))};
            if empty {self.metadata_section+=1;return Ok(false);}
            output.push_str(&format!("{}\"{field}\":{{",if output.ends_with('{') {""}else {","}));self.metadata_stage=1;self.metadata_name=None;self.metadata_records=0;
            return Ok(false);
        }
        if self.metadata_stage==1 {
            let bounds=self.metadata_name.as_ref().map_or((Unbounded,Unbounded),|name|(Excluded(name.clone()),Unbounded));
            let name=match self.metadata_section {0=>attributes.range(bounds).next().map(|(name,_)|name.clone()),1=>materials.range(bounds).next().map(|(name,_)|name.clone()),2=>textures.range(bounds).next().map(|(name,_)|name.clone()),_=>references.and_then(|values|values.iter().find(|(name,_)|self.metadata_name.as_ref().is_none_or(|previous|name>previous)).map(|(name,_)|name.clone()))};
            let Some(name)=name else {output.push('}');self.metadata_section+=1;self.metadata_stage=0;return Ok(false);};
            if name.is_empty() || name.chars().take(129).count()>128 {return Err("mesh metadata name limit exceeded".into());}
            if self.metadata_records>0 {output.push(',');}self.metadata_records+=1;
            output.push_str(&json::Value::from(name.as_str()).to_string());output.push(':');
            self.metadata_value=0;
            match self.metadata_section {
                0=>{
                    let attribute=attributes.get(&name).unwrap();
                    if attribute.values.len()>600_000 || attribute.domain_len()>600_000 {return Err("mesh metadata attribute capacity exceeded".into());}
                    let domain=match attribute.domain {MeshAttributeDomain::Vertex=>"vertex",MeshAttributeDomain::Corner=>"corner",MeshAttributeDomain::Face=>"face",MeshAttributeDomain::Edge=>"edge"};
                    let semantic=match attribute.semantic {MeshAttributeSemantic::Normal=>"normal",MeshAttributeSemantic::Uv=>"uv",MeshAttributeSemantic::Color=>"color",MeshAttributeSemantic::Material=>"material",MeshAttributeSemantic::Custom=>"custom"};
                    let interpolation=match attribute.interpolation {MeshAttributeInterpolation::Linear=>"linear",MeshAttributeInterpolation::Nearest=>"nearest",MeshAttributeInterpolation::Constant=>"constant"};
                    output.push_str(&format!("{{\"domain\":\"{domain}\",\"semantic\":\"{semantic}\",\"interpolation\":\"{interpolation}\",\"values\":["));
                },
                1=>self.metadata_stack.push(MeshJsonTask::Node(Vec::new())),
                2=>{
                    let texture=textures.get(&name).unwrap();
                    if texture.mime.is_empty() || texture.mime.chars().take(129).count()>128 || texture.bytes.len()>16_000_000 {return Err("mesh metadata texture capacity exceeded".into());}
                    output.push_str(&format!("{{\"mime\":{},\"bytes\":[",json::Value::from(texture.mime.as_str())));
                },
                _=>{
                    if !["face","edge","vertex"].contains(&name.as_str()) || references.unwrap().get(&name).unwrap().len()>600_000 {return Err("mesh component reference domain or capacity is invalid".into());}
                    output.push('[');
                },
            }
            self.metadata_name=Some(name);self.metadata_stage=2;return Ok(false);
        }
        let name=self.metadata_name.as_ref().unwrap();
        if self.metadata_section==3 {
            let labels=references.unwrap().get(name).unwrap();
            if let Some(label)=labels.get(self.metadata_value) {
                if !mesh_name_valid(label) {return Err("mesh component label limit exceeded".into());}
                if self.metadata_value>0 {output.push(',');}output.push_str(&json::Value::from(label.as_str()).to_string());self.metadata_value+=1;
            }else {output.push(']');self.metadata_stage=1;}
            return Ok(false);
        }
        if self.metadata_section==2 {
            let texture=textures.get(name).unwrap();
            if self.metadata_value<texture.bytes.len() {
                if self.metadata_value>0 {output.push(',');}output.push_str(&texture.bytes[self.metadata_value].to_string());self.metadata_value+=1;
            } else {output.push_str("]}");self.metadata_stage=1;}
            return Ok(false);
        }
        if self.metadata_section==0 {
            let attribute=attributes.get(name).unwrap();
            let remap=match attribute.domain {MeshAttributeDomain::Corner=>corners,MeshAttributeDomain::Edge=>edges,_=>None};
            if self.metadata_stage==2 {
                if self.metadata_value==attribute.values.len() {
                    output.push(']');self.metadata_value=0;
                    if attribute.indices.is_some() || match attribute.domain {MeshAttributeDomain::Corner=>corners.is_some(),MeshAttributeDomain::Edge=>edges.is_some(),_=>false} {output.push_str(",\"indices\":[");self.metadata_stage=3;}else {output.push('}');self.metadata_stage=1;}
                    return Ok(false);
                }
                if self.metadata_stack.is_empty() {if self.metadata_value>0 {output.push(',');}self.metadata_stack.push(MeshJsonTask::Node(Vec::new()));}
                mesh_json_step(&attribute.values[self.metadata_value],&mut self.metadata_stack,output)?;
                if self.metadata_stack.is_empty() {self.metadata_value+=1;}
            }else {
                if self.metadata_value==attribute.domain_len() {output.push_str("]}");self.metadata_stage=1;return Ok(false);}
                let source=match remap {Some(ids)=>*ids.get(self.metadata_value).ok_or("mesh metadata remap cardinality mismatch")? as usize,None=>self.metadata_value};
                let index=match &attribute.indices {Some(indices)=>*indices.get(source).ok_or("mesh metadata index cardinality mismatch")?,None=>source as u32};
                if index as usize>=attribute.values.len() {return Err("mesh metadata sample index out of range".into());}
                if self.metadata_value>0 {output.push(',');}output.push_str(&index.to_string());self.metadata_value+=1;
            }
        } else {
            mesh_json_step(materials.get(name).unwrap(),&mut self.metadata_stack,output)?;
            if self.metadata_stack.is_empty() {self.metadata_stage=1;}
        }
        Ok(false)
    }
}
#[derive(semio_framework_value_derive::RetireOwned)]
enum MeshJsonTask { Node(Vec<usize>),Array(Vec<usize>,usize),Object(Vec<usize>,usize),Text(Vec<usize>,usize,Option<usize>) }

fn mesh_json_node<'a>(root:&'a pack::value::DslValue,path:&[usize])->Result<&'a pack::value::DslValue,String> {
    let mut value=root;
    for index in path {value=match value {pack::value::DslValue::Array(values)=>values.get(*index),pack::value::DslValue::Object(values)=>values.get(*index).map(|(_,value)|value),_=>None}.ok_or_else(||String::from("mesh metadata cursor is invalid"))?;}
    Ok(value)
}

fn mesh_json_step(root:&pack::value::DslValue,stack:&mut Vec<MeshJsonTask>,output:&mut String)->Result<(),String> {
    use pack::value::DslValue;
    if stack.len()>256 {return Err(String::from("mesh metadata exceeds nesting limit"));}
    let task=stack.pop().ok_or_else(||String::from("mesh metadata cursor is retired"))?;
    match task {
        MeshJsonTask::Node(path)=>{
            if path.len()>64 {return Err(String::from("mesh metadata exceeds nesting limit"));}
            let value=mesh_json_node(root,&path)?;
            match value {
                DslValue::String(_)=>{output.push('"');stack.push(MeshJsonTask::Text(path,0,None));},
                DslValue::Array(_)|DslValue::Bytes(_)=>{output.push('[');stack.push(MeshJsonTask::Array(path,0));},
                DslValue::Object(_)=>{output.push('{');stack.push(MeshJsonTask::Object(path,0));},
                _=>{if value.as_f64().is_some_and(|value|!value.is_finite()) {return Err(String::from("mesh metadata number must be finite"));}output.push_str(&semio_framework_pack_json::from_dsl_value(value).to_string());},
            }
        },
        MeshJsonTask::Array(path,index)=>{
            let value=mesh_json_node(root,&path)?;
            let length=match value {DslValue::Array(values)=>values.len(),DslValue::Bytes(values)=>values.len(),_=>return Err(String::from("mesh metadata array cursor is invalid"))};
            if index==length {output.push(']');}
            else {
                if index>0 {output.push(',');}stack.push(MeshJsonTask::Array(path.clone(),index+1));
                if let DslValue::Bytes(values)=value {output.push_str(&values[index].to_string());}
                else {let mut child=path;child.push(index);stack.push(MeshJsonTask::Node(child));}
            }
        },
        MeshJsonTask::Object(path,index)=>{
            let values=mesh_json_node(root,&path)?.as_object().ok_or_else(||String::from("mesh metadata object cursor is invalid"))?;
            if index==values.len() {output.push('}');}
            else {
                if index>0 {output.push(',');}stack.push(MeshJsonTask::Object(path.clone(),index+1));
                let mut child=path.clone();child.push(index);stack.push(MeshJsonTask::Node(child));
                output.push('"');stack.push(MeshJsonTask::Text(path,0,Some(index)));
            }
        },
        MeshJsonTask::Text(path,offset,key)=>{
            let value=mesh_json_node(root,&path)?;
            let text=match key {Some(index)=>value.as_object().unwrap()[index].0.as_str(),None=>value.as_str().unwrap()};
            if offset==text.len() {output.push('"');if key.is_some() {output.push(':');}}
            else {
                let character=text[offset..].chars().next().unwrap();let encoded=semio_framework_pack_json::Value::from(character.to_string()).to_string();
                output.push_str(&encoded[1..encoded.len()-1]);stack.push(MeshJsonTask::Text(path,offset+character.len_utf8(),key));
            }
        },
    }
    Ok(())
}

/// 🌉️ `pack::json!` leaf conversion, mirroring this type's own serde attributes exactly: keys are
/// camelCase, `positions`/`normals`/`colors`/`indices` are always emitted, and the remaining fields
/// carry `skip_serializing_if` so they appear only when non-empty/`Some`. Widening `f32` to `f64`
/// matches what `serde_json` itself does for `f32`, so the emitted numbers are unchanged.
/// Needed because the viewer/editor main-window template moved from `serde_json::json!` (which
/// reached this type through `Serialize`) to `pack::json!` (which reaches leaves through `From`).
impl From<MeshData> for json::Value {
    fn from(mesh: MeshData) -> Self {
        fn floats(values: Vec<f32>) -> json::Value {
            json::Value::Array(values.into_iter().map(|v| json::Value::from(v as f64)).collect())
        }
        fn ints(values: Vec<u32>) -> json::Value {
            json::Value::Array(values.into_iter().map(json::Value::from).collect())
        }
        fn bytes(values: Vec<u8>) -> json::Value {
            json::Value::Array(values.into_iter().map(|v| json::Value::from(u32::from(v))).collect())
        }
        let mut object = json::Object::new();
        object.insert("positions", floats(mesh.positions));
        object.insert("normals", floats(mesh.normals));
        object.insert("colors", floats(mesh.colors));
        object.insert("indices", ints(mesh.indices));
        if !mesh.uvs.is_empty() { object.insert("uvs", floats(mesh.uvs)); }
        if !mesh.face_ids.is_empty() { object.insert("faceIds", ints(mesh.face_ids)); }
        if !mesh.vertex_ids.is_empty() { object.insert("vertexIds", ints(mesh.vertex_ids)); }
        if !mesh.edge_positions.is_empty() { object.insert("edgePositions", floats(mesh.edge_positions)); }
        if !mesh.edge_ids.is_empty() { object.insert("edgeIds", ints(mesh.edge_ids)); }
        if !mesh.edge_uvs.is_empty() { object.insert("edgeUvs", floats(mesh.edge_uvs)); }
        if !mesh.edge_is_seam.is_empty() { object.insert("edgeIsSeam", bytes(mesh.edge_is_seam)); }
        if let Some(texture) = mesh.paint_texture_base64 { object.insert("paintTextureBase64", json::Value::from(texture)); }
        use pack::value::ToValue;
        if !mesh.attributes.is_empty() { object.insert("attributes", json::from_dsl_value(&mesh.attributes.to_value())); }
        if !mesh.materials.is_empty() { object.insert("materials", json::from_dsl_value(&mesh.materials.to_value())); }
        if !mesh.textures.is_empty() { object.insert("textures", json::from_dsl_value(&mesh.textures.to_value())); }
        if !mesh.component_references.is_empty() { object.insert("componentReferences", json::from_dsl_value(&mesh.component_references.to_value())); }
        json::Value::Object(object)
    }
}

//#region Obj
pub fn mesh_to_obj(mesh: &MeshData, object_name: &str) -> String {
    let mut out = format!("o {object_name}\n");
    for chunk in mesh.positions.as_chunks::<3>().0 {
        out.push_str(&format!("v {} {} {}\n", chunk[0], chunk[1], chunk[2]));
    }
    if mesh.normals.len() == mesh.positions.len() {
        for chunk in mesh.normals.as_chunks::<3>().0 {
            out.push_str(&format!("vn {} {} {}\n", chunk[0], chunk[1], chunk[2]));
        }
    }
    for tri in mesh.indices.as_chunks::<3>().0 {
        let a = tri[0] + 1;
        let b = tri[1] + 1;
        let c = tri[2] + 1;
        if mesh.normals.len() == mesh.positions.len() {
            out.push_str(&format!("f {a}//{a} {b}//{b} {c}//{c}\n"));
        } else {
            out.push_str(&format!("f {a} {b} {c}\n"));
        }
    }
    out
}

/// 🔤️ Hand-parses OBJ text (`v`/`vn`/`f` lines) back into `MeshData`; fan-triangulates n-gon faces and falls back to computed normals when the file has no `vn` lines or a mismatched vertex/normal count. Round-trips `mesh_to_obj`'s own output losslessly; general third-party OBJ interop is unvalidated.
pub fn mesh_from_obj(text: &str) -> Result<MeshData, String> {
    let mut positions: Vec<f32> = Vec::new();
    let mut normals: Vec<f32> = Vec::new();
    let mut indices: Vec<u32> = Vec::new();
    let mut vertex_count = 0usize;
    let mut normal_count = 0usize;
    for line in text.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let mut parts = line.split_whitespace();
        let tag = match parts.next() {
            Some(tag) => tag,
            None => continue,
        };
        match tag {
            "v" => {
                let coords: Vec<f32> = parts.filter_map(|value| value.parse().ok()).collect();
                if coords.len() < 3 {
                    return Err("obj: malformed v line".into());
                }
                positions.extend_from_slice(&coords[..3]);
                vertex_count += 1;
            }
            "vn" => {
                let coords: Vec<f32> = parts.filter_map(|value| value.parse().ok()).collect();
                if coords.len() < 3 {
                    return Err("obj: malformed vn line".into());
                }
                normals.extend_from_slice(&coords[..3]);
                normal_count += 1;
            }
            "f" => {
                let mut face: Vec<usize> = Vec::new();
                for token in parts {
                    let raw_index = token.split('/').next().ok_or_else(|| "obj: malformed face token".to_string())?;
                    let raw: i64 = raw_index.parse().map_err(|_| "obj: malformed face index".to_string())?;
                    face.push(obj_resolve_index(raw, vertex_count)?);
                }
                if face.len() < 3 {
                    continue;
                }
                for i in 1..face.len() - 1 {
                    indices.push(face[0] as u32);
                    indices.push(face[i] as u32);
                    indices.push(face[i + 1] as u32);
                }
            }
            _ => {}
        }
    }
    let mut mesh = MeshData { positions, indices, ..MeshData::default() };
    if normal_count == vertex_count && normal_count > 0 {
        mesh.normals = normals;
    } else {
        mesh.compute_normals();
    }
    Ok(mesh)
}

fn obj_resolve_index(raw: i64, count: usize) -> Result<usize, String> {
    if raw > 0 {
        usize::try_from(raw - 1).map_err(|_| "obj: face index out of range".into())
    } else if raw < 0 {
        let index = count as i64 + raw;
        if index < 0 {
            Err("obj: negative vertex index out of range".into())
        } else {
            Ok(index as usize)
        }
    } else {
        Err("obj: zero vertex index".into())
    }
}
//#endregion Obj



#[path="🧬️fields/🦀️.rs"]
mod fields;

#[cfg(test)]
#[path="🧪️tests/🧬️ownership/🦀️.rs"]
mod neutral_ownership_tests;
