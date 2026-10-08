//! 🔺️ Pure mesh data, primitive construction, and Obj/Glb/Stl codecs — engine content dissolved
//! out of the framework-module grab-bag it used to share a file with an unrelated DWG codec.
//! Consumed only from artifact facet code (the mesh artifact's own mutation-diff/inference
//! internals) and from engine-to-engine callers such as brep tessellation/mesh-io — never a
//! standalone public surface a plugin app reaches into to bypass the artifact system.

// 🚫️async: R7 — `MeshExporter`/`MeshImporter` are first-party AFIT traits; Send is obtained
// structurally at the concrete-enum call site per R3, never via a `+ Send` bound on the trait
// method, so rustc's `async_fn_in_trait` lint (which would suggest exactly that bound) is
// silenced here rather than resolved by its own suggestion.
#![allow(async_fn_in_trait)]

use semio_framework_pack_json as json;
use std::collections::BTreeMap;
// 🔬️ `serde`/`serde_json` survive ONLY as a `#[cfg(test)]` differential oracle (see
// `mesh_data_json_oracle_tests`/`mesh_data_from_value_oracle_tests` below) now that `MeshData` has
// its own first-party `ToValue`/`FromValue` codec — never a production dependency of this crate.
// Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS.
#[cfg(test)]
use serde::{Deserialize, Serialize};

/// 🎨️ Attribute identities share the existing mesh value and its topology domains.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
#[value(crate = "::pack::value", rename_all = "camelCase")]
pub enum MeshAttributeDomain { Vertex, Corner, Face, Edge }
impl semio_framework_dsl_record::BorrowedDslField for MeshAttributeDomain {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Enum(&[("vertex", 0), ("corner", 1), ("face", 2), ("edge", 3)]);
}

/// 🧭️ Authored channel meaning controls transforms and preview expansion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
#[value(crate = "::pack::value", rename_all = "camelCase")]
pub enum MeshAttributeSemantic { Normal, Uv, Color, Material, Custom }
impl semio_framework_dsl_record::BorrowedDslField for MeshAttributeSemantic {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Enum(&[("normal", 0), ("uv", 1), ("color", 2), ("material", 3), ("custom", 4)]);
}

/// 🧵️ New topology declares how source values combine rather than silently discarding them.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize), serde(rename_all = "camelCase"))]
#[value(crate = "::pack::value", rename_all = "camelCase")]
pub enum MeshAttributeInterpolation { Linear, Nearest, Constant }
impl semio_framework_dsl_record::BorrowedDslField for MeshAttributeInterpolation {
    const SHAPE: semio_framework_dsl_record::BorrowedShape = semio_framework_dsl_record::BorrowedShape::Enum(&[("linear", 0), ("nearest", 1), ("constant", 2)]);
}

/// 📦️ First-party owned values permit numeric channels and structured custom metadata.
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::pack::value")]
pub struct MeshAttribute {
    pub domain: MeshAttributeDomain,
    pub semantic: MeshAttributeSemantic,
    pub interpolation: MeshAttributeInterpolation,
    pub values: Vec<pack::value::DslValue>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    pub indices: Option<Vec<u32>>,
}

impl MeshAttribute {
    /// 🔎️ Resolves the owned sample for one domain element.
    pub fn value_at(&self, index: usize) -> Option<&pack::value::DslValue> {
        let sample = match &self.indices { Some(indices) => *indices.get(index)? as usize, None => index };
        self.values.get(sample)
    }
    /// 📏️ Counts domain elements independently of shared sample storage.
    pub fn domain_len(&self) -> usize { self.indices.as_ref().map_or(self.values.len(), Vec::len) }
}

/// 🖼️ Texture bytes are owned by the same mesh payload as their material references.
#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[value(crate = "::pack::value")]
pub struct MeshTexture { pub mime: String, pub bytes: Vec<u8> }

/// 🥽️ Canonical indexed polygon source shared by artifact edits and inference.
#[derive(Clone,Debug,PartialEq,semio_framework_value_derive::ToValue,semio_framework_value_derive::FromValue)]
#[value(crate = "::pack::value")]
pub struct PolygonMeshSource {
    pub vertices:Vec<[f32;3]>,
    pub faces:Vec<Vec<u32>>,
    #[value(default,skip_serializing_if="BTreeMap::is_empty")]
    pub attributes:BTreeMap<String,MeshAttribute>,
    #[value(default,skip_serializing_if="BTreeMap::is_empty")]
    pub materials:BTreeMap<String,pack::value::DslValue>,
    #[value(default,skip_serializing_if="BTreeMap::is_empty")]
    pub textures:BTreeMap<String,MeshTexture>,
}

impl PolygonMeshSource {
    /// 🎒️ Serializes the same source shape consumed by the named geometry inference.
    pub fn encode(&self)->String {json::from_dsl_value(&pack::value::ToValue::to_value(self)).to_string()}
}

/// 🔎️ Parses bounded indexed polygon source without reconstructing geometry.
pub fn parse_polygon_mesh_source(text:&str)->Result<PolygonMeshSource,String> {
    let mut preparation=PolygonSourcePreparation::new();
    loop {if let Some(source)=preparation.step(text,4096,4096)? {return Ok(source);}}
}

/// 🧵️ Captures, projects, and admits the existing polygon payload under retained work grants.
pub struct PolygonSourcePreparation {
    parser:Option<json::JsonParseCursor>,projection:Option<json::JsonValueProjection>,decoding:Option<pack::value::native_decoding::NativeDecodeContinuation>,stage:u8,
    raw:Option<pack::value::DslValue>,vertices:std::vec::IntoIter<pack::value::DslValue>,faces:std::vec::IntoIter<pack::value::DslValue>,
    attributes:std::vec::IntoIter<(String,pack::value::DslValue)>,materials:std::vec::IntoIter<(String,pack::value::DslValue)>,textures:std::vec::IntoIter<(String,pack::value::DslValue)>,
    source:PolygonMeshSource,face:Vec<u32>,indices:std::vec::IntoIter<pack::value::DslValue>,seen:std::collections::HashSet<u32>,corners:usize,
    current:Option<(String,pack::value::DslValue)>,attribute:Option<(String,MeshAttribute)>,texture:Option<(String,MeshTexture)>,
    garbage:Vec<pack::value::DslValue>,retirement:Option<Box<dyn pack::value::ErasedSnapshotRetirement>>,name:Option<String>,index:usize,field:usize,texture_bytes:usize,
}
impl PolygonSourcePreparation {
    /// 🌱️ Starts without reading or cloning source text.
    pub fn new()->Self {Self {parser:Some(json::JsonParseCursor::new(json::JsonMemberPolicy::Reject)),projection:None,decoding:None,stage:0,raw:None,vertices:Vec::new().into_iter(),faces:Vec::new().into_iter(),attributes:Vec::new().into_iter(),materials:Vec::new().into_iter(),textures:Vec::new().into_iter(),source:PolygonMeshSource {vertices:Vec::new(),faces:Vec::new(),attributes:BTreeMap::new(),materials:BTreeMap::new(),textures:BTreeMap::new()},face:Vec::new(),indices:Vec::new().into_iter(),seen:Default::default(),corners:0,current:None,attribute:None,texture:None,garbage:Vec::new(),retirement:None,name:None,index:0,field:0,texture_bytes:0}}
    /// 📍️ Exposes the current existing source preparation phase.
    pub fn phase(&self)->&'static str {match self.stage {0=>"mesh-source-parse",1..=7=>"mesh-source-project",_=>"mesh-source-admit"}}
    /// ⏱️ Advances at most the granted candidate transitions using borrowed source text.
    pub fn step(&mut self,text:&str,maximum_units:usize,maximum_bytes:usize)->Result<Option<PolygonMeshSource>,String> {
        if text.len()>16_000_000 {return Err("mesh input exceeds 16 MB".into());}
        if maximum_bytes==0 {return Ok(None);}
        for _ in 0..maximum_units {
            if let Some(retirement)=&mut self.retirement {retirement.close_step(1,maximum_bytes).map_err(|error|error.to_string())?;if retirement.terminal_is_empty() {self.retirement=None;}continue;}
            match self.stage {
                0=>{let mut accepted=|_|true;let mut control=match self.decoding.take(){Some(receipt)=>pack::value::NativeDecodeControl::resume(receipt,&mut accepted),None=>Ok(pack::value::NativeDecodeControl::new(256*1024*1024,&mut accepted))}.map_err(|error|error.to_string())?;let parsed=self.parser.as_mut().unwrap().step(text,1,&mut control);self.decoding=Some(control.pause().map_err(|error|error.to_string())?);if let Some(value)=parsed.map_err(|error|error.to_string())? {self.projection=Some(json::JsonValueProjection::new(value));self.retirement=Some(pack::value::retirement::owned_retirement(self.parser.take().unwrap()));self.stage=1;}},
                1=>{let mut accepted=|_|true;let mut control=pack::value::NativeDecodeControl::resume(self.decoding.take().unwrap(),&mut accepted).map_err(|error|error.to_string())?;let projected=self.projection.as_mut().unwrap().step(1,maximum_bytes,&mut control);self.decoding=Some(control.pause().map_err(|error|error.to_string())?);if let Some(value)=projected.map_err(|error|error.to_string())? {self.raw=Some(value);self.retirement=Some(pack::value::retirement::owned_retirement(self.projection.take().unwrap()));self.stage=2;}},
                2=>self.admit_root()?,
                3=>{if let Some(value)=self.vertices.next() {self.raw=Some(value);let values=self.raw.as_ref().unwrap().as_array().filter(|values|values.len()==3).ok_or("vertex must have three coordinates")?;let mut point=[0.0;3];for axis in 0..3 {point[axis]=values[axis].as_f64().filter(|number|number.is_finite() && number.abs()<=f32::MAX as f64).ok_or("coordinate must be finite")? as f32;}self.source.vertices.push(point);self.garbage.push(self.raw.take().unwrap());}else {self.stage=4;}},
                4=>self.project_face()?,
                5=>self.project_attribute()?,
                6=>{if let Some((name,value))=self.materials.next() {self.current=Some((name,value));let (name,value)=self.current.as_ref().unwrap();if !mesh_name_valid(name) || value.as_object().is_none() {return Err("invalid owned mesh material".into());}let (name,value)=self.current.take().unwrap();self.source.materials.insert(name,value);}else {self.stage=7;}},
                7=>self.project_texture()?,
                8=>self.admit_attribute()?,
                9=>self.admit_material()?,
                10=>{if !self.garbage.is_empty() {self.retirement=Some(pack::value::retirement::owned_retirement(std::mem::take(&mut self.garbage)));}else {self.stage=11;}},
                _=>return Ok(Some(std::mem::replace(&mut self.source,PolygonMeshSource {vertices:Vec::new(),faces:Vec::new(),attributes:BTreeMap::new(),materials:BTreeMap::new(),textures:BTreeMap::new()}))),
            }
        }
        Ok(None)
    }
    fn admit_root(&mut self)->Result<(),String> {
        let fields=self.raw.as_ref().unwrap().as_object().filter(|fields|fields.len()<=5 && fields.iter().all(|(name,_)|["vertices","faces","attributes","materials","textures"].contains(&name.as_str()))).ok_or("unknown mesh field")?;
        for (field,minimum,maximum) in [("vertices",3,100_000),("faces",1,100_000)] {let values=fields.iter().find(|(name,_)|name==field).and_then(|(_,value)|value.as_array()).ok_or_else(||format!("{field} must be an array"))?;if !(minimum..=maximum).contains(&values.len()) {return Err("mesh requires 3..100000 vertices and 1..100000 faces".into());}}
        for (field,maximum) in [("attributes",64),("materials",10_000),("textures",256)] {if let Some((_,value))=fields.iter().find(|(name,_)|name==field) {if value.as_object().is_none_or(|fields|fields.len()>maximum) {return Err("mesh asset declaration limit exceeded".into());}}}
        let Some(pack::value::DslValue::Object(fields))=self.raw.take() else {unreachable!()};
        for (name,value) in fields {match (name.as_str(),value) {("vertices",pack::value::DslValue::Array(values))=>self.vertices=values.into_iter(),("faces",pack::value::DslValue::Array(values))=>self.faces=values.into_iter(),("attributes",pack::value::DslValue::Object(values))=>self.attributes=values.into_iter(),("materials",pack::value::DslValue::Object(values))=>self.materials=values.into_iter(),("textures",pack::value::DslValue::Object(values))=>self.textures=values.into_iter(),_=>unreachable!()}}
        self.stage=3;Ok(())
    }
    fn project_face(&mut self)->Result<(),String> {
        if let Some(value)=self.indices.next() {self.raw=Some(value);let id=self.raw.as_ref().unwrap().as_u64().filter(|id|*id<self.source.vertices.len() as u64).ok_or("face index out of range")? as u32;if !self.seen.insert(id) {return Err("face contains a repeated vertex".into());}self.face.push(id);self.garbage.push(self.raw.take().unwrap());return Ok(());}
        if !self.face.is_empty() {self.source.faces.push(std::mem::take(&mut self.face));self.retirement=Some(pack::value::retirement::owned_retirement(std::mem::take(&mut self.seen)));return Ok(());}
        if let Some(value)=self.faces.next() {self.raw=Some(value);let values=self.raw.as_ref().unwrap().as_array().filter(|values|values.len()>=3).ok_or("face requires at least three indices")?;self.corners=self.corners.saturating_add(values.len());if self.corners>600_000 {return Err("mesh exceeds 600000 polygon corners".into());}let Some(pack::value::DslValue::Array(values))=self.raw.take() else {unreachable!()};self.indices=values.into_iter();}else {self.stage=5;}Ok(())
    }
    fn project_attribute(&mut self)->Result<(),String> {
        if let Some((_,attribute))=&mut self.attribute {if let Some(value)=self.indices.next() {self.raw=Some(value);attribute.indices.as_mut().unwrap().push(u32::try_from(self.raw.as_ref().unwrap().as_u64().ok_or("invalid mesh attribute index")?).map_err(|_|"invalid mesh attribute index")?);self.garbage.push(self.raw.take().unwrap());return Ok(());}let (name,attribute)=self.attribute.take().unwrap();self.source.attributes.insert(name,attribute);return Ok(());}
        let Some(value)=self.attributes.next() else {self.stage=6;return Ok(())};self.current=Some(value);
        let (name,value)=self.current.as_ref().unwrap();if !mesh_name_valid(name) {return Err("mesh attribute declaration limit exceeded".into());}
        let fields=value.as_object().filter(|fields|fields.len()<=5 && fields.iter().all(|(name,_)|["domain","semantic","interpolation","values","indices"].contains(&name.as_str()))).ok_or("invalid mesh attribute declaration")?;
        let get=|key:&str|fields.iter().find(|(name,_)|name==key).map(|(_,value)|value);
        let domain=match get("domain").and_then(|value|value.as_str()) {Some("vertex")=>MeshAttributeDomain::Vertex,Some("corner")=>MeshAttributeDomain::Corner,Some("face")=>MeshAttributeDomain::Face,Some("edge")=>MeshAttributeDomain::Edge,_=>return Err("invalid mesh attribute domain".into())};
        let semantic=match get("semantic").and_then(|value|value.as_str()) {Some("normal")=>MeshAttributeSemantic::Normal,Some("uv")=>MeshAttributeSemantic::Uv,Some("color")=>MeshAttributeSemantic::Color,Some("material")=>MeshAttributeSemantic::Material,Some("custom")=>MeshAttributeSemantic::Custom,_=>return Err("invalid mesh attribute semantic".into())};
        let interpolation=match get("interpolation").and_then(|value|value.as_str()) {Some("linear")=>MeshAttributeInterpolation::Linear,Some("nearest")=>MeshAttributeInterpolation::Nearest,Some("constant")=>MeshAttributeInterpolation::Constant,_=>return Err("invalid mesh attribute interpolation".into())};
        if get("values").and_then(|value|value.as_array()).is_none_or(|values|values.len()>600_000) {return Err("invalid mesh attribute values".into());}
        if get("indices").is_some_and(|value|!matches!(value,pack::value::DslValue::Null) && value.as_array().is_none_or(|values|values.len()>600_000)) {return Err("invalid mesh attribute indices".into());}
        let (name,pack::value::DslValue::Object(fields))=self.current.take().unwrap() else {unreachable!()};let mut attribute=MeshAttribute {domain,semantic,interpolation,values:Vec::new(),indices:None};
        for (key,value) in fields {match (key.as_str(),value) {("values",pack::value::DslValue::Array(values))=>attribute.values=values,("indices",pack::value::DslValue::Array(values))=>{self.indices=values.into_iter();attribute.indices=Some(Vec::new());},(_,value)=>self.garbage.push(value)}}
        self.attribute=Some((name,attribute));Ok(())
    }
    fn project_texture(&mut self)->Result<(),String> {
        if let Some((_,texture))=&mut self.texture {if let Some(value)=self.indices.next() {self.raw=Some(value);texture.bytes.push(u8::try_from(self.raw.as_ref().unwrap().as_u64().ok_or("invalid mesh texture byte")?).map_err(|_|"invalid mesh texture byte")?);self.garbage.push(self.raw.take().unwrap());return Ok(());}let (name,texture)=self.texture.take().unwrap();self.source.textures.insert(name,texture);return Ok(());}
        let Some(value)=self.textures.next() else {self.stage=8;self.name=None;return Ok(())};self.current=Some(value);let (name,value)=self.current.as_ref().unwrap();if !mesh_name_valid(name) {return Err("mesh asset declaration limit exceeded".into());}
        let fields=value.as_object().filter(|fields|fields.len()==2 && fields.iter().all(|(name,_)|["mime","bytes"].contains(&name.as_str()))).ok_or("invalid owned mesh texture")?;
        let mime=fields.iter().find(|(name,_)|name=="mime").and_then(|(_,value)|value.as_str()).filter(|mime|mesh_name_valid(mime)).ok_or("invalid owned mesh texture")?;
        let _=mime;let bytes=fields.iter().find(|(name,_)|name=="bytes").and_then(|(_,value)|value.as_array()).ok_or("invalid owned mesh texture")?;self.texture_bytes=self.texture_bytes.saturating_add(bytes.len());if self.texture_bytes>16_000_000 {return Err("invalid owned mesh texture".into());}
        let (name,pack::value::DslValue::Object(fields))=self.current.take().unwrap() else {unreachable!()};let mut texture=MeshTexture {mime:String::new(),bytes:Vec::new()};for (key,value) in fields {match (key.as_str(),value) {("mime",pack::value::DslValue::String(value))=>texture.mime=value,("bytes",pack::value::DslValue::Array(values))=>self.indices=values.into_iter(),_=>unreachable!()}}self.texture=Some((name,texture));Ok(())
    }
    fn admit_attribute(&mut self)->Result<(),String> {
        use std::ops::Bound::{Excluded,Unbounded};
        if self.index==0 {let bounds=self.name.as_ref().map_or((Unbounded,Unbounded),|name|(Excluded(name.clone()),Unbounded));let Some((name,_))=self.source.attributes.range(bounds).next() else {self.stage=9;self.name=None;return Ok(())};self.name=Some(name.clone());self.index=1;}
        let name=self.name.as_ref().unwrap();let attribute=&self.source.attributes[name];let count=match attribute.domain {MeshAttributeDomain::Vertex=>self.source.vertices.len(),MeshAttributeDomain::Face=>self.source.faces.len(),_=>self.corners};
        if attribute.domain_len()!=count {return Err(format!("mesh attribute '{name}' cardinality does not match its domain"));}
        let position=self.index-1;
        if let Some(value)=attribute.indices.as_ref().and_then(|values|values.get(position)) {if *value as usize>=attribute.values.len() {return Err(format!("mesh attribute '{name}' cardinality does not match its domain"));}}
        if let Some(value)=attribute.values.get(position) {validate_mesh_attribute_sample(name,attribute,value,&self.source.materials)?;}
        if position+1>=attribute.values.len().max(attribute.indices.as_ref().map_or(0,Vec::len)) {self.index=0;}else {self.index+=1;}Ok(())
    }
    fn admit_material(&mut self)->Result<(),String> {
        use std::ops::Bound::{Excluded,Unbounded};
        if self.field==0 {let bounds=self.name.as_ref().map_or((Unbounded,Unbounded),|name|(Excluded(name.clone()),Unbounded));let Some((name,_))=self.source.materials.range(bounds).next() else {self.stage=10;return Ok(())};self.name=Some(name.clone());self.field=1;}
        let name=self.name.as_ref().unwrap();let fields=self.source.materials[name].as_object().unwrap();if let Some((key,value))=fields.get(self.field-1) {validate_mesh_material_field(name,key,value,&self.source.textures)?;self.field+=1;}else {self.field=0;}Ok(())
    }
}
impl pack::value::retirement::RetireOwned for PolygonSourcePreparation {fn retirement(self)->Box<dyn pack::value::retirement::RetirementCursor>{let fields=pack::value::artifact_retirement_sequence!(self.parser,self.projection,self.raw,self.vertices,self.faces,self.attributes,self.materials,self.textures,self.source,self.face,self.indices,self.seen,self.current,self.attribute,self.texture,self.garbage,self.name);if let Some(retirement)=self.retirement {pack::value::retirement::sequence(vec![pack::value::retirement::erased_cursor(retirement),fields])}else {fields}}}

/// ✅️ Validates owned polygon channels independently of geometry evaluation.
pub fn validate_polygon_mesh_attributes(vertex_count:usize,face_count:usize,halfedge_count:usize,attributes:&BTreeMap<String,MeshAttribute>,materials:&BTreeMap<String,pack::value::DslValue>,textures:&BTreeMap<String,MeshTexture>)->Result<(),String> {
    if vertex_count>100_000 || face_count>100_000 || halfedge_count>600_000 || attributes.len()>64 {return Err("mesh declaration capacity exceeded".into());}
    validate_mesh_surface_assets(materials,textures)?;
    for (name,attribute) in attributes {validate_mesh_attribute(name,attribute,vertex_count,face_count,halfedge_count,materials)?;}
    Ok(())
}

/// 🎨️ Validates material references and owned texture payload limits.
pub fn validate_mesh_surface_assets(materials:&BTreeMap<String,pack::value::DslValue>,textures:&BTreeMap<String,MeshTexture>)->Result<(),String> {
    if materials.len()>10_000 || textures.len()>256 || materials.keys().chain(textures.keys()).any(|name|!mesh_name_valid(name)) {return Err("mesh asset declaration limit exceeded".into());}
    let mut bytes=0usize;
    for texture in textures.values() {bytes=bytes.saturating_add(texture.bytes.len());if !mesh_name_valid(&texture.mime) || bytes>16_000_000 {return Err("invalid owned mesh texture".into());}}
    for (id,material) in materials {
        let fields=material.as_object().ok_or_else(||format!("mesh material '{id}' must be an owned object"))?;
        for (name,value) in fields {validate_mesh_material_field(id,name,value,textures)?;}
    }
    Ok(())
}

fn validate_mesh_material_field(id:&str,name:&str,value:&pack::value::DslValue,textures:&BTreeMap<String,MeshTexture>)->Result<(),String> {
            if name.ends_with("Texture") && value.as_str().is_none_or(|name|!textures.contains_key(name)) {return Err(format!("mesh material '{id}' references an undefined texture"));}
            let range=|value:&pack::value::DslValue|value.as_f64().is_some_and(|number|number.is_finite() && (0.0..=1.0).contains(&number));
            let valid=match name {
                "baseColor"=>value.as_array().is_some_and(|values|values.len()==4 && values.iter().all(range)),
                "metallic"|"roughness"|"occlusionStrength"=>range(value),
                "alphaCutoff"=>value.as_f64().is_some_and(|number|number.is_finite() && number>=0.0 && number<=f32::MAX as f64),
                "normalScale"=>value.as_f64().is_some_and(|number|number.is_finite() && number.abs()<=f32::MAX as f64) || value.as_array().is_some_and(|values|values.len()==2 && values.iter().all(|value|value.as_f64().is_some_and(|number|number.is_finite() && number.abs()<=f32::MAX as f64))),
                "textureCoordinates"=>value.as_object().is_some_and(|fields|fields.iter().all(|(key,value)|["baseColorTexture","metallicRoughnessTexture","normalTexture","occlusionTexture","emissiveTexture"].contains(&key.as_str()) && value.as_u64().is_some_and(|set|set<64))),
                "textureSamplers"=>value.as_object().is_some_and(|fields|fields.iter().all(|(key,value)|["baseColorTexture","metallicRoughnessTexture","normalTexture","occlusionTexture","emissiveTexture"].contains(&key.as_str()) && value.as_object().is_some_and(|fields|fields.iter().all(|(key,value)|value.as_u64().is_some_and(|value|match key.as_str(){"wrapS"|"wrapT"=>matches!(value,33071|33648|10497),"magFilter"=>matches!(value,9728|9729),"minFilter"=>matches!(value,9728|9729|9984|9985|9986|9987),_=>false}))))),
                "emissive"=>value.as_array().is_some_and(|values|values.len()==3 && values.iter().all(|value|value.as_f64().is_some_and(|number|number.is_finite() && (0.0..=f32::MAX as f64).contains(&number)))),
                "alphaMode"=>value.as_str().is_some_and(|mode|matches!(mode,"OPAQUE"|"MASK"|"BLEND")),
                "doubleSided"=>value.as_bool().is_some(),
                _=>true,
            };
            if !valid {return Err(format!("mesh material '{id}' has an invalid '{name}' field"));}
        
    Ok(())
}

/// 🧭️ Validates one indexed channel using its declared domain and interpolation rule.
pub fn validate_mesh_attribute(name:&str,attribute:&MeshAttribute,vertex_count:usize,face_count:usize,halfedge_count:usize,materials:&BTreeMap<String,pack::value::DslValue>)->Result<(),String> {
    if !mesh_name_valid(name) {return Err("mesh attribute declaration limit exceeded".into());}
    let count=match attribute.domain {MeshAttributeDomain::Vertex=>vertex_count,MeshAttributeDomain::Face=>face_count,_=>halfedge_count};
    if attribute.values.len()>600_000 || attribute.domain_len()!=count || attribute.indices.as_ref().is_some_and(|indices|indices.iter().any(|index|*index as usize>=attribute.values.len())) {return Err(format!("mesh attribute '{name}' cardinality does not match its domain"));}
    if attribute.values.is_empty() && attribute.interpolation==MeshAttributeInterpolation::Linear {return Err(format!("linear mesh attribute '{name}' requires finite numeric values"));}
    for value in &attribute.values {validate_mesh_attribute_sample(name,attribute,value,materials)?;}
    Ok(())
}

fn mesh_name_valid(name:&str)->bool {!name.is_empty() && name.chars().take(129).count()<=128}

fn mesh_attribute_numeric(value:&pack::value::DslValue)->Option<usize> {
    if value.as_f64().is_some_and(|number|number.is_finite() && number.abs()<=f32::MAX as f64) {return Some(0);}
    let tuple=value.as_array()?;(!tuple.is_empty() && tuple.len()<=16 && tuple.iter().all(|value|value.as_f64().is_some_and(|number|number.is_finite() && number.abs()<=f32::MAX as f64))).then_some(tuple.len())
}
fn validate_mesh_attribute_sample(name:&str,attribute:&MeshAttribute,value:&pack::value::DslValue,materials:&BTreeMap<String,pack::value::DslValue>)->Result<(),String> {
    let numeric=mesh_attribute_numeric;
    if attribute.interpolation==MeshAttributeInterpolation::Linear {let width=attribute.values.first().and_then(numeric).ok_or_else(||format!("linear mesh attribute '{name}' requires finite numeric values"))?;if numeric(value)!=Some(width) {return Err(format!("linear mesh attribute '{name}' requires compatible numeric dimensions"));}}
    let width=match attribute.semantic {MeshAttributeSemantic::Normal=>3,MeshAttributeSemantic::Uv=>2,MeshAttributeSemantic::Color=>4,_=>0};
    if width>0 && (!matches!(attribute.domain,MeshAttributeDomain::Vertex|MeshAttributeDomain::Corner|MeshAttributeDomain::Face) || numeric(value)!=Some(width)) {return Err(format!("mesh attribute '{name}' has an invalid semantic domain or dimensions"));}
    if name=="tangent" && (attribute.semantic!=MeshAttributeSemantic::Custom || !matches!(attribute.domain,MeshAttributeDomain::Vertex|MeshAttributeDomain::Corner|MeshAttributeDomain::Face) || numeric(value)!=Some(4) || value.as_array().is_none_or(|values|values[..3].iter().all(|value|value.as_f64()==Some(0.0)) || values[3].as_f64().is_none_or(|value|value!=1.0 && value!= -1.0))) {return Err("canonical tangent requires a nonzero finite four-component direction and handedness +/-1".into());}
    if attribute.semantic==MeshAttributeSemantic::Normal && value.as_array().unwrap().iter().all(|value|value.as_f64()==Some(0.0)) {return Err(format!("mesh normal '{name}' cannot be zero"));}
    if attribute.semantic==MeshAttributeSemantic::Material && (attribute.domain!=MeshAttributeDomain::Face || attribute.interpolation==MeshAttributeInterpolation::Linear || value.as_str().is_none_or(|name|!materials.contains_key(name))) {return Err(format!("mesh attribute '{name}' references an undefined face material"));}
    Ok(())
}

/// 🎒️ Retained metadata cursor shared by mesh JSON and preview packing.
#[derive(Default)]
pub struct MeshMetadataCursor {
    metadata_section:u8,metadata_stage:u8,metadata_name:Option<String>,metadata_records:usize,metadata_value:usize,metadata_stack:Vec<MeshJsonTask>,
}
impl MeshMetadataCursor {
    /// 🧵️ Appends one bounded metadata transition using borrowed canonical values.
    pub fn step(&mut self, attributes:&BTreeMap<String,MeshAttribute>,materials:&BTreeMap<String,pack::value::DslValue>,textures:&BTreeMap<String,MeshTexture>,references:Option<&BTreeMap<String,Vec<String>>>,corners:Option<&[u32]>,edges:Option<&[u32]>,output:&mut String)->Result<bool,String> {
        use std::ops::Bound::{Excluded,Unbounded};
        if attributes.len()>64 || materials.len()>10_000 || textures.len()>256 {return Err("mesh metadata declaration limit exceeded".into());}
        if references.is_some_and(|values|values.len()>3) {return Err("mesh component reference domain limit exceeded".into());}
        if self.metadata_section==4 {return Ok(true); }
        if self.metadata_stage==0 {
            let (field,empty)=match self.metadata_section {0=>("attributes",attributes.is_empty()),1=>("materials",materials.is_empty()),2=>("textures",textures.is_empty()),_=>("componentReferences",references.is_none_or(BTreeMap::is_empty))};
            if empty {self.metadata_section+=1;return Ok(false);}
            output.push_str(&format!("{}\"{field}\":{{",if output.ends_with('{') {""}else {","}));self.metadata_stage=1;self.metadata_name=None;self.metadata_records=0;
            return Ok(false);
        }
        if self.metadata_stage==1 {
            let bounds=self.metadata_name.as_ref().map_or((Unbounded,Unbounded),|name|(Excluded(name.clone()),Unbounded));
            let name=match self.metadata_section {0=>attributes.range(bounds).next().map(|(name,_)|name.clone()),1=>materials.range(bounds).next().map(|(name,_)|name.clone()),2=>textures.range(bounds).next().map(|(name,_)|name.clone()),_=>references.and_then(|values|values.range(bounds).next().map(|(name,_)|name.clone()))};
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

//#region MeshData
#[derive(Clone, Debug, PartialEq, Default)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
pub struct MeshData {
    #[cfg_attr(test, serde(default))]
    pub positions: Vec<f32>,
    #[cfg_attr(test, serde(default))]
    pub normals: Vec<f32>,
    #[cfg_attr(test, serde(default))]
    pub colors: Vec<f32>,
    #[cfg_attr(test, serde(default))]
    pub indices: Vec<u32>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub uvs: Vec<f32>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub face_ids: Vec<u32>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub vertex_ids: Vec<u32>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub edge_positions: Vec<f32>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub edge_ids: Vec<u32>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub edge_uvs: Vec<f32>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Vec::is_empty"))]
    pub edge_is_seam: Vec<u8>,
    #[cfg_attr(test, serde(skip_serializing_if = "Option::is_none"))]
    pub paint_texture_base64: Option<String>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "BTreeMap::is_empty"))]
    pub attributes: BTreeMap<String, MeshAttribute>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "BTreeMap::is_empty"))]
    pub materials: BTreeMap<String, pack::value::DslValue>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "BTreeMap::is_empty"))]
    pub textures: BTreeMap<String, MeshTexture>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "BTreeMap::is_empty"))]
    pub component_references: BTreeMap<String, Vec<String>>,
}

impl pack::value::retirement::RetireOwned for MeshData {
    fn retirement(self) -> Box<dyn pack::value::retirement::RetirementCursor> {
        pack::value::artifact_retirement_sequence![self.positions,self.normals,self.colors,self.indices,self.uvs,self.face_ids,self.vertex_ids,self.edge_positions,self.edge_ids,self.edge_uvs,self.edge_is_seam,self.paint_texture_base64,self.attributes,self.materials,self.textures,self.component_references]
    }
}
impl pack::value::retirement::RetireOwned for MeshAttribute {
    fn retirement(self) -> Box<dyn pack::value::retirement::RetirementCursor> {pack::value::artifact_retirement_sequence![self.values,self.indices]}
}
impl pack::value::retirement::RetireOwned for MeshTexture {
    fn retirement(self) -> Box<dyn pack::value::retirement::RetirementCursor> {pack::value::artifact_retirement_sequence![self.mime,self.bytes]}
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

/// 🌱️ First-party value encoding, delegating to the `From<MeshData> for pack::json::Value` impl
/// directly above so the two paths cannot drift — the viewer/editor window templates reach this type
/// through both `pack::json!` (which uses `From`) and `ToValue::to_value`, and they must agree.
impl pack::value::ToValue for MeshData {
    fn to_value(&self) -> pack::value::DslValue {
        json::to_dsl_value(&json::Value::from(self.clone()))
    }
}

/// 🔀️ First-party value decoding — the exact inverse of `ToValue` above, hand-written rather than
/// `#[derive(FromValue)]` because that derive's expansion hardcodes `::semio_framework_os_kernel::…`
/// paths (`🌱️value/✨️derive/🦀️.rs`), and this crate sits BELOW `os-kernel` in the dependency graph
/// (its own doc comment: "consumed only from artifact facet code... or engine-to-engine callers"),
/// so taking that dependency here would invert the layering for a "pure mesh data" leaf crate. Reads
/// the same camelCase object shape `ToValue`/`From<MeshData> for pack::json::Value` emit —
/// `positions`/`normals`/`colors`/`indices` default to empty when absent (mirroring `#[serde(default)]`),
/// every other field defaults to empty/`None` when its key is missing (mirroring
/// `skip_serializing_if`). Indices/counts decode through `u32`'s `FromValue` (accepts `UInt`/`Int`/
/// `Float` DslValue numbers but the encoder only ever emits `UInt` for them — see the sibling
/// `ToValue` impl's `ints` helper), positions/normals/colors/uvs through `f32`'s — never crossed, so
/// a mesh index can never silently decode as a float.
impl pack::value::FromValue for MeshData {
    fn from_value(value: pack::value::DslValue) -> Result<Self, pack::value::ValueError> {
        use pack::value::FromValue;
        let entries = value.into_object()?;
        let field = |key: &str| entries.iter().find(|(k, _)| *k == key).map(|(_, v)| v.clone());
        fn decode_vec<T: FromValue>(field: Option<pack::value::DslValue>, key: &str) -> Result<Vec<T>, pack::value::ValueError> {
            match field {
                Some(value) => Vec::<T>::from_value(value).map_err(|error| error.under(key)),
                None => Ok(Vec::new()),
            }
        }
        let mesh=MeshData {
            positions: decode_vec(field("positions"), "positions")?,
            normals: decode_vec(field("normals"), "normals")?,
            colors: decode_vec(field("colors"), "colors")?,
            indices: decode_vec(field("indices"), "indices")?,
            uvs: decode_vec(field("uvs"), "uvs")?,
            face_ids: decode_vec(field("faceIds"), "faceIds")?,
            vertex_ids: decode_vec(field("vertexIds"), "vertexIds")?,
            edge_positions: decode_vec(field("edgePositions"), "edgePositions")?,
            edge_ids: decode_vec(field("edgeIds"), "edgeIds")?,
            edge_uvs: decode_vec(field("edgeUvs"), "edgeUvs")?,
            edge_is_seam: decode_vec(field("edgeIsSeam"), "edgeIsSeam")?,
            attributes: field("attributes").map(BTreeMap::<String, MeshAttribute>::from_value).transpose()?.unwrap_or_default(),
            materials: field("materials").map(BTreeMap::<String, pack::value::DslValue>::from_value).transpose()?.unwrap_or_default(),
            textures: field("textures").map(BTreeMap::<String, MeshTexture>::from_value).transpose()?.unwrap_or_default(),
            component_references: field("componentReferences").map(BTreeMap::<String, Vec<String>>::from_value).transpose()?.unwrap_or_default(),
            paint_texture_base64: match field("paintTextureBase64") {
                None | Some(pack::value::DslValue::Null) => None,
                Some(value) => Some(String::from_value(value).map_err(|error| error.under("paintTextureBase64"))?),
            },
        };
        mesh.validate_component_references().map_err(|message|pack::value::ValueError::new(pack::value::ValueRefusalKind::InvalidValue,message))?;
        Ok(mesh)
    }
}

impl MeshData {
    /// 🎯️ Validates lossless component labels against their numeric picking buffers.
    pub fn validate_component_references(&self)->Result<(),String> {
        for(domain,labels)in &self.component_references {
            let(ids,count)=match domain.as_str() {"face"=>(&self.face_ids,self.indices.len()/3),"edge"=>(&self.edge_ids,self.edge_positions.len()/6),"vertex"=>(&self.vertex_ids,self.positions.len()/3),_=>return Err("analytic component references contain an unknown domain".into())};
            if labels.len()>600_000 || labels.len()>count || ids.len()!=count || ids.iter().any(|id|*id as usize>=labels.len() && !(domain=="vertex" && *id==u32::MAX)) || labels.iter().any(|label|!mesh_name_valid(label)) {return Err("analytic component references require complete picking buffers and bounded full labels".into());}
        }
        Ok(())
    }

    pub fn vertex_count(&self) -> usize {
        self.positions.len() / 3
    }

    pub fn triangle_count(&self) -> usize {
        self.indices.len() / 3
    }

    pub fn compute_normals(&mut self) {
        let count = self.vertex_count();
        self.normals = vec![0.0; count * 3];
        for tri in self.indices.as_chunks::<3>().0 {
            let i0 = tri[0] as usize;
            let i1 = tri[1] as usize;
            let i2 = tri[2] as usize;
            let p0 = [self.positions[i0 * 3], self.positions[i0 * 3 + 1], self.positions[i0 * 3 + 2]];
            let p1 = [self.positions[i1 * 3], self.positions[i1 * 3 + 1], self.positions[i1 * 3 + 2]];
            let p2 = [self.positions[i2 * 3], self.positions[i2 * 3 + 1], self.positions[i2 * 3 + 2]];
            let e0 = [p1[0] - p0[0], p1[1] - p0[1], p1[2] - p0[2]];
            let e1 = [p2[0] - p0[0], p2[1] - p0[1], p2[2] - p0[2]];
            let n = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
            for &idx in tri {
                let i = idx as usize * 3;
                self.normals[i] += n[0];
                self.normals[i + 1] += n[1];
                self.normals[i + 2] += n[2];
            }
        }
        for chunk in self.normals.as_chunks_mut::<3>().0 {
            let len = (chunk[0] * chunk[0] + chunk[1] * chunk[1] + chunk[2] * chunk[2]).sqrt();
            if len > 1e-8 {
                chunk[0] /= len;
                chunk[1] /= len;
                chunk[2] /= len;
            }
        }
    }

    pub fn aabb(&self) -> ([f32; 3], [f32; 3]) {
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];
        for chunk in self.positions.as_chunks::<3>().0 {
            for axis in 0..3 {
                min[axis] = min[axis].min(chunk[axis]);
                max[axis] = max[axis].max(chunk[axis]);
            }
        }
        (min, max)
    }

    pub fn merge(&mut self, other: &MeshData) {
        let base = self.vertex_count() as u32;
        self.positions.extend_from_slice(&other.positions);
        self.normals.extend_from_slice(&other.normals);
        self.colors.extend_from_slice(&other.colors);
        self.indices.extend(other.indices.iter().map(|index| index + base));
    }
}
//#endregion MeshData

//#region Primitives
fn push_triangle(mesh: &mut MeshData, a: [f32; 3], b: [f32; 3], c: [f32; 3]) {
    let base = mesh.vertex_count() as u32;
    mesh.positions.extend_from_slice(&[a[0], a[1], a[2], b[0], b[1], b[2], c[0], c[1], c[2]]);
    mesh.indices.extend_from_slice(&[base, base + 1, base + 2]);
}

pub fn mesh_box(width: f32, height: f32, depth: f32) -> MeshData {
    let hw = width * 0.5;
    let hh = height * 0.5;
    let hd = depth * 0.5;
    let mut mesh = MeshData::default();
    let faces = [
        ([-hw, -hh, hd], [hw, -hh, hd], [hw, hh, hd], [-hw, hh, hd]),
        ([hw, -hh, -hd], [-hw, -hh, -hd], [-hw, hh, -hd], [hw, hh, -hd]),
        ([-hw, hh, hd], [hw, hh, hd], [hw, hh, -hd], [-hw, hh, -hd]),
        ([-hw, -hh, -hd], [hw, -hh, -hd], [hw, -hh, hd], [-hw, -hh, hd]),
        ([hw, -hh, hd], [hw, -hh, -hd], [hw, hh, -hd], [hw, hh, hd]),
        ([-hw, -hh, -hd], [-hw, -hh, hd], [-hw, hh, hd], [-hw, hh, -hd]),
    ];
    for (a, b, c, d) in faces {
        push_triangle(&mut mesh, a, b, c);
        push_triangle(&mut mesh, a, c, d);
    }
    mesh.compute_normals();
    mesh
}

pub fn mesh_plane(width: f32, depth: f32) -> MeshData {
    let hw = width * 0.5;
    let hd = depth * 0.5;
    let mut mesh = MeshData::default();
    push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, -hd], [hw, 0.0, hd]);
    push_triangle(&mut mesh, [-hw, 0.0, -hd], [hw, 0.0, hd], [-hw, 0.0, hd]);
    mesh.compute_normals();
    mesh
}

pub fn mesh_uv_sphere(radius: f32, segments: u32, rings: u32) -> MeshData {
    let mut mesh = MeshData::default();
    for ring in 0..rings {
        let v0 = ring as f32 / rings as f32;
        let v1 = (ring + 1) as f32 / rings as f32;
        let phi0 = v0 * std::f32::consts::PI;
        let phi1 = v1 * std::f32::consts::PI;
        for seg in 0..segments {
            let u0 = seg as f32 / segments as f32;
            let u1 = (seg + 1) as f32 / segments as f32;
            let theta0 = u0 * std::f32::consts::TAU;
            let theta1 = u1 * std::f32::consts::TAU;
            let p00 = sphere_point(radius, phi0, theta0);
            let p10 = sphere_point(radius, phi0, theta1);
            let p01 = sphere_point(radius, phi1, theta0);
            let p11 = sphere_point(radius, phi1, theta1);
            if ring > 0 {
                push_triangle(&mut mesh, p00, p10, p11);
            }
            if ring + 1 < rings {
                push_triangle(&mut mesh, p00, p11, p01);
            }
        }
    }
    mesh.compute_normals();
    mesh
}

fn sphere_point(radius: f32, phi: f32, theta: f32) -> [f32; 3] {
    let sin_phi = phi.sin();
    [radius * sin_phi * theta.cos(), radius * phi.cos(), radius * sin_phi * theta.sin()]
}

pub fn mesh_ico_sphere(radius: f32, subdivisions: u32) -> MeshData {
    let t = (1.0 + 5.0_f32.sqrt()) * 0.5;
    let mut verts = vec![
        normalize3([-1.0, t, 0.0]),
        normalize3([1.0, t, 0.0]),
        normalize3([-1.0, -t, 0.0]),
        normalize3([1.0, -t, 0.0]),
        normalize3([0.0, -1.0, t]),
        normalize3([0.0, 1.0, t]),
        normalize3([0.0, -1.0, -t]),
        normalize3([0.0, 1.0, -t]),
        normalize3([t, 0.0, -1.0]),
        normalize3([t, 0.0, 1.0]),
        normalize3([-t, 0.0, -1.0]),
        normalize3([-t, 0.0, 1.0]),
    ];
    let mut faces =
        vec![[0, 11, 5], [0, 5, 1], [0, 1, 7], [0, 7, 10], [0, 10, 11], [1, 5, 9], [5, 11, 4], [11, 10, 2], [10, 7, 6], [7, 1, 8], [3, 9, 4], [3, 4, 2], [3, 2, 6], [3, 6, 8], [3, 8, 9], [4, 9, 5], [2, 4, 11], [6, 2, 10], [8, 6, 7], [9, 8, 1]];
    for _ in 0..subdivisions {
        let mut next = Vec::new();
        let mut midpoint_cache = std::collections::HashMap::new();
        for face in &faces {
            let a = midpoint(&mut verts, &mut midpoint_cache, face[0], face[1]);
            let b = midpoint(&mut verts, &mut midpoint_cache, face[1], face[2]);
            let c = midpoint(&mut verts, &mut midpoint_cache, face[2], face[0]);
            next.extend_from_slice(&[[face[0], a, c], [face[1], b, a], [face[2], c, b], [a, b, c]]);
        }
        faces = next;
    }
    let mut mesh = MeshData::default();
    for face in faces {
        let a = scale3(verts[face[0] as usize], radius);
        let b = scale3(verts[face[1] as usize], radius);
        let c = scale3(verts[face[2] as usize], radius);
        push_triangle(&mut mesh, a, b, c);
    }
    mesh.compute_normals();
    mesh
}

fn normalize3(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    [v[0] / len, v[1] / len, v[2] / len]
}

fn scale3(v: [f32; 3], s: f32) -> [f32; 3] {
    [v[0] * s, v[1] * s, v[2] * s]
}

fn midpoint(verts: &mut Vec<[f32; 3]>, cache: &mut std::collections::HashMap<(u32, u32), u32>, a: u32, b: u32) -> u32 {
    let key = if a < b { (a, b) } else { (b, a) };
    if let Some(index) = cache.get(&key) {
        return *index;
    }
    let mid = normalize3([(verts[a as usize][0] + verts[b as usize][0]) * 0.5, (verts[a as usize][1] + verts[b as usize][1]) * 0.5, (verts[a as usize][2] + verts[b as usize][2]) * 0.5]);
    let index = verts.len() as u32;
    verts.push(mid);
    cache.insert(key, index);
    index
}

pub fn mesh_cylinder(radius: f32, height: f32, segments: u32) -> MeshData {
    let mut mesh = MeshData::default();
    let half = height * 0.5;
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p00 = [radius * a0.cos(), -half, radius * a0.sin()];
        let p01 = [radius * a1.cos(), -half, radius * a1.sin()];
        let p10 = [radius * a0.cos(), half, radius * a0.sin()];
        let p11 = [radius * a1.cos(), half, radius * a1.sin()];
        push_triangle(&mut mesh, p00, p01, p11);
        push_triangle(&mut mesh, p00, p11, p10);
        push_triangle(&mut mesh, [0.0, -half, 0.0], p01, p00);
        push_triangle(&mut mesh, [0.0, half, 0.0], p10, p11);
    }
    mesh.compute_normals();
    mesh
}

pub fn mesh_cone(radius: f32, height: f32, segments: u32) -> MeshData {
    let mut mesh = MeshData::default();
    let apex = [0.0, height, 0.0];
    for seg in 0..segments {
        let u0 = seg as f32 / segments as f32;
        let u1 = (seg + 1) as f32 / segments as f32;
        let a0 = u0 * std::f32::consts::TAU;
        let a1 = u1 * std::f32::consts::TAU;
        let p0 = [radius * a0.cos(), 0.0, radius * a0.sin()];
        let p1 = [radius * a1.cos(), 0.0, radius * a1.sin()];
        push_triangle(&mut mesh, apex, p1, p0);
        push_triangle(&mut mesh, [0.0, 0.0, 0.0], p0, p1);
    }
    mesh.compute_normals();
    mesh
}

pub fn mesh_torus(major_radius: f32, minor_radius: f32, segments: u32, rings: u32) -> MeshData {
    let mut mesh = MeshData::default();
    for ring in 0..rings {
        let v0 = ring as f32 / rings as f32;
        let v1 = (ring + 1) as f32 / rings as f32;
        let phi0 = v0 * std::f32::consts::TAU;
        let phi1 = v1 * std::f32::consts::TAU;
        for seg in 0..segments {
            let u0 = seg as f32 / segments as f32;
            let u1 = (seg + 1) as f32 / segments as f32;
            let theta0 = u0 * std::f32::consts::TAU;
            let theta1 = u1 * std::f32::consts::TAU;
            let p00 = torus_point(major_radius, minor_radius, phi0, theta0);
            let p10 = torus_point(major_radius, minor_radius, phi0, theta1);
            let p01 = torus_point(major_radius, minor_radius, phi1, theta0);
            let p11 = torus_point(major_radius, minor_radius, phi1, theta1);
            push_triangle(&mut mesh, p00, p10, p11);
            push_triangle(&mut mesh, p00, p11, p01);
        }
    }
    mesh.compute_normals();
    mesh
}

fn torus_point(major: f32, minor: f32, phi: f32, theta: f32) -> [f32; 3] {
    let r = major + minor * theta.cos();
    [r * phi.cos(), minor * theta.sin(), r * phi.sin()]
}

pub fn mesh_from_kind(kind: &str) -> MeshData {
    match kind {
        "vortex-marker" => mesh_ico_sphere(0.12, 1),
        "vertex-marker" => mesh_ico_sphere(1.0, 1),
        "sphere" | "uvSphere" => mesh_uv_sphere(0.5, 16, 12),
        "icoSphere" => mesh_ico_sphere(0.5, 1),
        "plane" => mesh_plane(1.0, 1.0),
        "cylinder" => mesh_cylinder(0.5, 1.0, 16),
        "cone" => mesh_cone(0.5, 1.0, 16),
        "torus" => mesh_torus(0.5, 0.15, 16, 12),
        _ => mesh_box(1.0, 1.0, 1.0),
    }
}

/** 🔩️ Builds mesh data from indexed brep tessellation buffers. */
pub fn mesh_from_indexed(positions: &[f32], normals: &[f32], indices: &[u32]) -> MeshData {
    let mut mesh = MeshData { positions: positions.to_vec(), normals: normals.to_vec(), indices: indices.to_vec(), ..MeshData::default() };
    if mesh.normals.is_empty() && !mesh.positions.is_empty() {
        mesh.compute_normals();
    }
    mesh
}

/** 🧩️ Like `mesh_from_indexed`, but also stamps `face_ids` per triangle from `(face id, triangle start, triangle count)`
 * groups — lets a picked triangle resolve back to the brep face it came from. Plain tuples (not the kernel's `FaceGroup`)
 * so this crate doesn't need to depend on the kernel engine crate; callers convert their own group type. */
pub fn mesh_from_indexed_with_face_groups(positions: &[f32], normals: &[f32], indices: &[u32], face_groups: &[(u32, u32, u32)]) -> MeshData {
    let mut mesh = mesh_from_indexed(positions, normals, indices);
    if !face_groups.is_empty() {
        let triangle_count = indices.len() / 3;
        let mut face_ids = vec![0u32; triangle_count];
        for &(face_id, start, count) in face_groups {
            let start_tri = (start / 3) as usize;
            let count_tri = (count / 3) as usize;
            for slot in face_ids.iter_mut().take((start_tri + count_tri).min(triangle_count)).skip(start_tri) {
                *slot = face_id;
            }
        }
        mesh.face_ids = face_ids;
    }
    mesh
}
//#endregion Primitives

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
        Ok((raw - 1) as usize)
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

//#region Glb
pub fn mesh_to_glb(mesh: &MeshData) -> Vec<u8> {
    let positions = f32_slice_to_bytes(&mesh.positions);
    let normals = if mesh.normals.len() == mesh.positions.len() {
        f32_slice_to_bytes(&mesh.normals)
    } else {
        let mut copy = mesh.clone();
        copy.compute_normals();
        f32_slice_to_bytes(&copy.normals)
    };
    let indices = u32_slice_to_bytes(&mesh.indices);
    let bin = [positions.as_slice(), normals.as_slice(), indices.as_slice()].concat();
    let padded_bin = pad_to_4(bin);
    let positions_len = positions.len();
    let normals_len = normals.len();
    let indices_len = indices.len();
    let positions_offset = 0usize;
    let normals_offset = positions_offset + positions_len;
    let indices_offset = normals_offset + normals_len;
    let json = format!(
        r#"{{
  "asset": {{"version": "2.0"}},
  "scene": 0,
  "scenes": [{{"nodes": [0]}}],
  "nodes": [{{"mesh": 0}}],
  "meshes": [{{
    "primitives": [{{
      "attributes": {{"POSITION": 0, "NORMAL": 1}},
      "indices": 2,
      "mode": 4
    }}]
  }}],
  "accessors": [
    {{"bufferView": 0, "componentType": 5126, "count": {}, "type": "VEC3", "min": {}, "max": {}}},
    {{"bufferView": 1, "componentType": 5126, "count": {}, "type": "VEC3"}},
    {{"bufferView": 2, "componentType": 5125, "count": {}, "type": "SCALAR"}}
  ],
  "bufferViews": [
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}},
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}},
    {{"buffer": 0, "byteOffset": {}, "byteLength": {}}}
  ],
  "buffers": [{{"byteLength": {}}}]
}}"#,
        mesh.vertex_count(),
        json_vec3_min(&mesh.positions),
        json_vec3_max(&mesh.positions),
        mesh.vertex_count(),
        mesh.indices.len(),
        positions_offset,
        positions_len,
        normals_offset,
        normals_len,
        indices_offset,
        indices_len,
        padded_bin.len()
    );
    let mut json_bytes = json.into_bytes();
    while json_bytes.len() % 4 != 0 {
        json_bytes.push(b' ');
    }
    let total_len = 12 + 8 + json_bytes.len() + 8 + padded_bin.len();
    let mut out = Vec::with_capacity(total_len);
    out.extend_from_slice(b"glTF");
    out.extend_from_slice(&(2u32).to_le_bytes());
    out.extend_from_slice(&(total_len as u32).to_le_bytes());
    out.extend_from_slice(&(json_bytes.len() as u32).to_le_bytes());
    out.extend_from_slice(b"JSON");
    out.extend_from_slice(&json_bytes);
    out.extend_from_slice(&(padded_bin.len() as u32).to_le_bytes());
    out.extend_from_slice(b"BIN\x00");
    out.extend_from_slice(&padded_bin);
    out
}

type GlbMatrix = [[f32; 4]; 4];

fn glb_identity() -> GlbMatrix {
    [[1.0, 0.0, 0.0, 0.0], [0.0, 1.0, 0.0, 0.0], [0.0, 0.0, 1.0, 0.0], [0.0, 0.0, 0.0, 1.0]]
}

fn glb_matrix_mul(left: GlbMatrix, right: GlbMatrix) -> GlbMatrix {
    let mut result = [[0.0; 4]; 4];
    for column in 0..4 {
        for row in 0..4 {
            result[column][row] = (0..4).map(|axis| left[axis][row] * right[column][axis]).sum();
        }
    }
    result
}

fn glb_transform_point(matrix: GlbMatrix, point: [f32; 3]) -> [f32; 3] {
    [
        matrix[0][0] * point[0] + matrix[1][0] * point[1] + matrix[2][0] * point[2] + matrix[3][0],
        matrix[0][1] * point[0] + matrix[1][1] * point[1] + matrix[2][1] * point[2] + matrix[3][1],
        matrix[0][2] * point[0] + matrix[1][2] * point[1] + matrix[2][2] * point[2] + matrix[3][2],
    ]
}

fn glb_transform_normal(matrix: GlbMatrix, normal: [f32; 3]) -> [f32; 3] {
    let (a00, a01, a02) = (matrix[0][0], matrix[1][0], matrix[2][0]);
    let (a10, a11, a12) = (matrix[0][1], matrix[1][1], matrix[2][1]);
    let (a20, a21, a22) = (matrix[0][2], matrix[1][2], matrix[2][2]);
    let det = a00 * (a11 * a22 - a12 * a21) - a01 * (a10 * a22 - a12 * a20) + a02 * (a10 * a21 - a11 * a20);
    if det.abs() <= f32::EPSILON {
        return normal;
    }
    let inverse_det = det.recip();
    let transformed = [
        ((a11 * a22 - a12 * a21) * normal[0] + (a12 * a20 - a10 * a22) * normal[1] + (a10 * a21 - a11 * a20) * normal[2]) * inverse_det,
        ((a02 * a21 - a01 * a22) * normal[0] + (a00 * a22 - a02 * a20) * normal[1] + (a01 * a20 - a00 * a21) * normal[2]) * inverse_det,
        ((a01 * a12 - a02 * a11) * normal[0] + (a02 * a10 - a00 * a12) * normal[1] + (a00 * a11 - a01 * a10) * normal[2]) * inverse_det,
    ];
    let length = transformed.iter().map(|value| value * value).sum::<f32>().sqrt();
    if length <= f32::EPSILON {
        normal
    } else {
        transformed.map(|value| value / length)
    }
}

//#region 🔖️GltfCodec
/// 🧊️ First-party glTF 2.0 container split: `.glb` binary (magic/version/chunk walk) or bare
/// `.gltf` JSON text (detected by a leading `{`) with no binary chunk. Replaces the `gltf` crate
/// per ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS — mirrors the
/// byte-for-byte semantics of the stdio gltf artifact's own `decode_glb`
/// (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🦀️.rs`),
/// re-expressed against `pack::json` instead of `serde_json` since this is the framework, not
/// that artifact's own mutation-schema codec.
fn gltf_split_container(bytes: &[u8]) -> Result<(Vec<u8>, Option<Vec<u8>>), String> {
    if bytes.first() == Some(&b'{') {
        return Ok((bytes.to_vec(), None));
    }
    if bytes.len() < 12 || &bytes[0..4] != b"glTF" {
        return Err("glb: bad magic, expected 'glTF' or '{'".into());
    }
    let version = u32::from_le_bytes(bytes[4..8].try_into().unwrap());
    if version != 2 {
        return Err(format!("glb: unsupported version {version}, only 2 is supported"));
    }
    let total_len = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let mut pos = 12usize;
    let mut json_chunk: Option<Vec<u8>> = None;
    let mut bin_chunk: Option<Vec<u8>> = None;
    while pos + 8 <= bytes.len() && pos < total_len {
        let chunk_len = u32::from_le_bytes(bytes[pos..pos + 4].try_into().unwrap()) as usize;
        let chunk_type = &bytes[pos + 4..pos + 8];
        let data_start = pos + 8;
        let data_end = data_start + chunk_len;
        if data_end > bytes.len() {
            return Err("glb: chunk length exceeds buffer".into());
        }
        if chunk_type == b"JSON" && json_chunk.is_none() {
            json_chunk = Some(bytes[data_start..data_end].to_vec());
        } else if chunk_type == b"BIN\0" && bin_chunk.is_none() {
            bin_chunk = Some(bytes[data_start..data_end].to_vec());
        }
        pos = data_end;
    }
    Ok((json_chunk.ok_or_else(|| "glb: missing JSON chunk".to_string())?, bin_chunk))
}

/// 🔓️ Decodes a `data:...;base64,...` uri through the framework's own base64 codec — external
/// (file-path) buffer uris are left unresolved (empty bytes), same contract as the stdio gltf
/// artifact's `resolve_document_buffers`: this engine has no filesystem/network access.
fn gltf_decode_data_uri(uri: &str) -> Result<Vec<u8>, String> {
    if !uri.starts_with("data:") {
        return Err("gltf: unsupported external buffer uri (no filesystem access)".into());
    }
    let marker = ";base64,";
    let idx = uri.find(marker).ok_or_else(|| "gltf: unsupported non-base64 data uri".to_string())?;
    semio_framework_io_base64::base64_standard_decode(&uri[idx + marker.len()..]).map_err(|error| error.to_string())
}

/// 📦️ Resolves `document.buffers[i]` to raw bytes, index-aligned with the JSON array. Only
/// `buffers[0]` may omit `uri` and be sourced from the `.glb` BIN chunk, per spec.
fn gltf_resolve_buffers(document: &json::Value, embedded_bin: Option<&[u8]>) -> Vec<Vec<u8>> {
    document
        .get("buffers")
        .and_then(json::Value::as_array)
        .map_or(&[][..], Vec::as_slice)
        .iter()
        .enumerate()
        .map(|(i, buffer)| match buffer.get("uri").and_then(json::Value::as_str) {
            Some(uri) => gltf_decode_data_uri(uri).unwrap_or_default(),
            None if i == 0 => embedded_bin.map(<[u8]>::to_vec).unwrap_or_default(),
            None => Vec::new(),
        })
        .collect()
}

fn gltf_component_byte_size(component_type: u64) -> Result<usize, String> {
    Ok(match component_type {
        5120 | 5121 => 1,
        5122 | 5123 => 2,
        5125 | 5126 => 4,
        other => return Err(format!("gltf: unsupported accessor.componentType {other}")),
    })
}

fn gltf_read_component(component_type: u64, bytes: &[u8], offset: usize) -> Result<f64, String> {
    let size = gltf_component_byte_size(component_type)?;
    if offset + size > bytes.len() {
        return Err("gltf: accessor component read out of buffer bounds".into());
    }
    Ok(match component_type {
        5120 => bytes[offset] as i8 as f64,
        5121 => bytes[offset] as f64,
        5122 => i16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap()) as f64,
        5123 => u16::from_le_bytes(bytes[offset..offset + 2].try_into().unwrap()) as f64,
        5125 => u32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as f64,
        5126 => f32::from_le_bytes(bytes[offset..offset + 4].try_into().unwrap()) as f64,
        other => return Err(format!("gltf: unsupported accessor.componentType {other}")),
    })
}

fn gltf_accessor_type_components(kind: &str) -> Result<usize, String> {
    Ok(match kind {
        "SCALAR" => 1,
        "VEC2" => 2,
        "VEC3" => 3,
        "VEC4" | "MAT2" => 4,
        "MAT3" => 9,
        "MAT4" => 16,
        other => return Err(format!("gltf: unsupported accessor.type {other:?}")),
    })
}

fn gltf_read_elements(bytes: &[u8], base_offset: usize, component_type: u64, accessor_type: &str, count: usize, byte_stride: Option<usize>) -> Result<Vec<f64>, String> {
    let nc = gltf_accessor_type_components(accessor_type)?;
    let component_size = gltf_component_byte_size(component_type)?;
    let stride = byte_stride.unwrap_or(component_size * nc);
    let mut out = Vec::with_capacity(count * nc);
    for i in 0..count {
        let elem_off = base_offset + i * stride;
        for c in 0..nc {
            out.push(gltf_read_component(component_type, bytes, elem_off + c * component_size)?);
        }
    }
    Ok(out)
}

/// 🎚️ Applies glTF 2.0 accessor normalization (§3.6.2.2) after dense/sparse values assembled.
fn gltf_normalize_components(component_type: u64, components: &mut [f64]) -> Result<(), String> {
    let (scale, signed) = match component_type {
        5120 => (127.0, true),
        5121 => (255.0, false),
        5122 => (32_767.0, true),
        5123 => (65_535.0, false),
        5125 => (4_294_967_295.0, false),
        5126 => return Err("gltf: normalized FLOAT accessor is invalid glTF 2.0".into()),
        other => return Err(format!("gltf: unsupported accessor.componentType {other}")),
    };
    for value in components {
        *value = if signed { (*value / scale).max(-1.0) } else { *value / scale };
    }
    Ok(())
}

fn gltf_read_bufferview_elements(document: &json::Value, buffers: &[Vec<u8>], bv_idx: usize, extra_offset: usize, component_type: u64, accessor_type: &str, count: usize) -> Result<Vec<f64>, String> {
    let bv = document.get("bufferViews").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice).get(bv_idx).ok_or_else(|| format!("gltf: bufferView index {bv_idx} out of range"))?;
    let buffer_index = bv.get("buffer").and_then(json::Value::as_u64).unwrap_or(0) as usize;
    let byte_offset = bv.get("byteOffset").and_then(json::Value::as_u64).unwrap_or(0) as usize;
    let byte_stride = bv.get("byteStride").and_then(json::Value::as_u64).map(|value| value as usize);
    let bytes = buffers.get(buffer_index).ok_or_else(|| format!("gltf: buffer index {buffer_index} out of range"))?;
    if bytes.is_empty() {
        return Err(format!("gltf: buffer {buffer_index} bytes unavailable (external uri not resolvable, or empty embedded buffer)"));
    }
    gltf_read_elements(bytes, byte_offset + extra_offset, component_type, accessor_type, count, byte_stride)
}

/// 🧩️ Decodes `document.accessors[accessor_index]` against `buffers` — dense `bufferView` read,
/// then `accessor.sparse` substitution (base is zero-filled when there's no `bufferView`).
fn gltf_decode_accessor(document: &json::Value, buffers: &[Vec<u8>], accessor_index: usize) -> Result<Vec<f64>, String> {
    let accessor = document.get("accessors").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice).get(accessor_index).ok_or_else(|| format!("gltf: accessor index {accessor_index} out of range"))?;
    let component_type = accessor.get("componentType").and_then(json::Value::as_u64).ok_or_else(|| "gltf: accessor missing componentType".to_string())?;
    let accessor_type = accessor.get("type").and_then(json::Value::as_str).ok_or_else(|| "gltf: accessor missing type".to_string())?;
    let count = accessor.get("count").and_then(json::Value::as_u64).unwrap_or(0) as usize;
    let normalized = accessor.get("normalized").and_then(json::Value::as_bool).unwrap_or(false);
    let nc = gltf_accessor_type_components(accessor_type)?;

    let mut components = vec![0.0f64; count * nc];
    if let Some(bv_idx) = accessor.get("bufferView").and_then(json::Value::as_u64) {
        let extra_offset = accessor.get("byteOffset").and_then(json::Value::as_u64).unwrap_or(0) as usize;
        components = gltf_read_bufferview_elements(document, buffers, bv_idx as usize, extra_offset, component_type, accessor_type, count)?;
    }

    if let Some(sparse) = accessor.get("sparse") {
        let sparse_count = sparse.get("count").and_then(json::Value::as_u64).unwrap_or(0) as usize;
        let indices = sparse.get("indices").ok_or_else(|| "gltf: sparse accessor missing indices".to_string())?;
        let values = sparse.get("values").ok_or_else(|| "gltf: sparse accessor missing values".to_string())?;
        let indices_bv = indices.get("bufferView").and_then(json::Value::as_u64).ok_or_else(|| "gltf: sparse indices missing bufferView".to_string())? as usize;
        let indices_offset = indices.get("byteOffset").and_then(json::Value::as_u64).unwrap_or(0) as usize;
        let indices_component = indices.get("componentType").and_then(json::Value::as_u64).ok_or_else(|| "gltf: sparse indices missing componentType".to_string())?;
        let values_bv = values.get("bufferView").and_then(json::Value::as_u64).ok_or_else(|| "gltf: sparse values missing bufferView".to_string())? as usize;
        let values_offset = values.get("byteOffset").and_then(json::Value::as_u64).unwrap_or(0) as usize;

        let idx_values = gltf_read_bufferview_elements(document, buffers, indices_bv, indices_offset, indices_component, "SCALAR", sparse_count)?;
        let val_values = gltf_read_bufferview_elements(document, buffers, values_bv, values_offset, component_type, accessor_type, sparse_count)?;
        for i in 0..sparse_count {
            let idx = idx_values[i] as usize;
            let dst = idx * nc;
            if dst + nc > components.len() {
                return Err(format!("gltf: sparse accessor index {idx} out of range for count {count}"));
            }
            components[dst..dst + nc].copy_from_slice(&val_values[i * nc..i * nc + nc]);
        }
    }

    if normalized {
        gltf_normalize_components(component_type, &mut components)?;
    }
    Ok(components)
}

fn gltf_node_vec3(node: &json::Value, key: &str, default: [f32; 3]) -> [f32; 3] {
    node.get(key)
        .and_then(json::Value::as_array)
        .filter(|values| values.len() == 3)
        .map_or(default, |values| [values[0].as_f64().unwrap_or(default[0] as f64) as f32, values[1].as_f64().unwrap_or(default[1] as f64) as f32, values[2].as_f64().unwrap_or(default[2] as f64) as f32])
}

fn gltf_node_quat(node: &json::Value) -> [f32; 4] {
    node.get("rotation")
        .and_then(json::Value::as_array)
        .filter(|values| values.len() == 4)
        .map_or([0.0, 0.0, 0.0, 1.0], |values| [values[0].as_f64().unwrap_or(0.0) as f32, values[1].as_f64().unwrap_or(0.0) as f32, values[2].as_f64().unwrap_or(0.0) as f32, values[3].as_f64().unwrap_or(1.0) as f32])
}

/// 🧮️ `T * R * S` node transform per glTF 2.0 §5.25 — quaternion-to-rotation composed with
/// translation/scale, laid out column-major to match this file's `GlbMatrix` convention.
fn gltf_trs_matrix(t: [f32; 3], r: [f32; 4], s: [f32; 3]) -> GlbMatrix {
    let (x, y, z, w) = (r[0], r[1], r[2], r[3]);
    let (x2, y2, z2) = (x + x, y + y, z + z);
    let (xx, xy, xz) = (x * x2, x * y2, x * z2);
    let (yy, yz, zz) = (y * y2, y * z2, z * z2);
    let (wx, wy, wz) = (w * x2, w * y2, w * z2);
    [
        [(1.0 - (yy + zz)) * s[0], (xy + wz) * s[0], (xz - wy) * s[0], 0.0],
        [(xy - wz) * s[1], (1.0 - (xx + zz)) * s[1], (yz + wx) * s[1], 0.0],
        [(xz + wy) * s[2], (yz - wx) * s[2], (1.0 - (xx + yy)) * s[2], 0.0],
        [t[0], t[1], t[2], 1.0],
    ]
}

/// 🧮️ A node's own local matrix: explicit `matrix` (already column-major, 16 floats) takes
/// precedence over `translation`/`rotation`/`scale` per spec.
fn gltf_node_local_matrix(node: &json::Value) -> GlbMatrix {
    if let Some(m) = node.get("matrix").and_then(json::Value::as_array).filter(|values| values.len() == 16) {
        let f: Vec<f32> = m.iter().map(|value| value.as_f64().unwrap_or(0.0) as f32).collect();
        return [[f[0], f[1], f[2], f[3]], [f[4], f[5], f[6], f[7]], [f[8], f[9], f[10], f[11]], [f[12], f[13], f[14], f[15]]];
    }
    gltf_trs_matrix(gltf_node_vec3(node, "translation", [0.0, 0.0, 0.0]), gltf_node_quat(node), gltf_node_vec3(node, "scale", [1.0, 1.0, 1.0]))
}

fn gltf_triangle_indices(mode: u64, source: Vec<u32>) -> Vec<u32> {
    match mode {
        4 => source,
        5 => source.windows(3).enumerate().flat_map(|(index, tri)| if index % 2 == 0 { [tri[0], tri[1], tri[2]] } else { [tri[1], tri[0], tri[2]] }).collect(),
        6 => source.first().map(|first| source[1..].windows(2).flat_map(|pair| [*first, pair[0], pair[1]]).collect()).unwrap_or_default(),
        _ => Vec::new(),
    }
}

fn gltf_append_primitive(mesh: &mut MeshData, document: &json::Value, primitive: &json::Value, buffers: &[Vec<u8>], matrix: GlbMatrix) -> Result<(), String> {
    let mode = primitive.get("mode").and_then(json::Value::as_u64).unwrap_or(4);
    if !matches!(mode, 4..=6) {
        return Ok(());
    }
    let attributes = primitive.get("attributes").ok_or_else(|| "gltf: primitive missing attributes".to_string())?;
    let position_accessor = attributes.get("POSITION").and_then(json::Value::as_u64).ok_or_else(|| "glb triangle primitive missing POSITION".to_string())? as usize;
    let positions: Vec<[f32; 3]> = gltf_decode_accessor(document, buffers, position_accessor)?.as_chunks::<3>().0.iter().map(|c| [c[0] as f32, c[1] as f32, c[2] as f32]).collect();

    let source_indices: Vec<u32> = if let Some(indices_accessor) = primitive.get("indices").and_then(json::Value::as_u64) {
        gltf_decode_accessor(document, buffers, indices_accessor as usize)?.into_iter().map(|value| value as u32).collect()
    } else {
        (0..positions.len() as u32).collect()
    };
    let indices = gltf_triangle_indices(mode, source_indices);
    if indices.iter().any(|index| *index as usize >= positions.len()) {
        return Err("glb triangle index outside POSITION accessor".into());
    }
    let normals: Vec<[f32; 3]> = if let Some(normal_accessor) = attributes.get("NORMAL").and_then(json::Value::as_u64) {
        gltf_decode_accessor(document, buffers, normal_accessor as usize)?.as_chunks::<3>().0.iter().map(|c| [c[0] as f32, c[1] as f32, c[2] as f32]).collect()
    } else {
        let mut local = MeshData { positions: positions.iter().flatten().copied().collect(), indices: indices.clone(), ..Default::default() };
        local.compute_normals();
        local.normals.as_chunks::<3>().0.to_vec()
    };
    if normals.len() != positions.len() {
        return Err("glb NORMAL and POSITION accessor counts differ".into());
    }
    let vertex_offset = mesh.vertex_count() as u32;
    for position in positions {
        mesh.positions.extend(glb_transform_point(matrix, position));
    }
    for normal in normals {
        mesh.normals.extend(glb_transform_normal(matrix, normal));
    }
    mesh.indices.extend(indices.into_iter().map(|index| vertex_offset + index));
    Ok(())
}

fn gltf_append_mesh(mesh: &mut MeshData, document: &json::Value, mesh_index: usize, buffers: &[Vec<u8>], matrix: GlbMatrix) -> Result<(), String> {
    let source = document.get("meshes").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice).get(mesh_index).ok_or_else(|| format!("gltf: mesh index {mesh_index} out of range"))?;
    for primitive in source.get("primitives").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice) {
        gltf_append_primitive(mesh, document, primitive, buffers, matrix)?;
    }
    Ok(())
}

fn gltf_append_node(mesh: &mut MeshData, document: &json::Value, node_index: usize, parent: GlbMatrix, buffers: &[Vec<u8>]) -> Result<(), String> {
    let node = document.get("nodes").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice).get(node_index).ok_or_else(|| format!("gltf: node index {node_index} out of range"))?;
    let matrix = glb_matrix_mul(parent, gltf_node_local_matrix(node));
    if let Some(mesh_index) = node.get("mesh").and_then(json::Value::as_u64) {
        gltf_append_mesh(mesh, document, mesh_index as usize, buffers, matrix)?;
    }
    for child in node.get("children").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice) {
        if let Some(child_index) = child.as_u64() {
            gltf_append_node(mesh, document, child_index as usize, matrix, buffers)?;
        }
    }
    Ok(())
}

/// 🧊️ Decodes every triangle primitive in the active GLB/glTF scene into one renderer-neutral
/// mesh, via this crate's own glTF 2.0 codec (`pack::json` + `semio_framework_io_base64`) —
/// never the `gltf` crate, which survives only as a `[dev-dependencies]` differential-test oracle.
pub fn mesh_from_glb(bytes: &[u8]) -> Result<MeshData, String> {
    let (json_bytes, bin) = gltf_split_container(bytes)?;
    let text = std::str::from_utf8(&json_bytes).map_err(|error| format!("gltf json is not valid utf-8: {error}"))?;
    let document = json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| format!("gltf json parse error: {error}"))?;
    let buffers = gltf_resolve_buffers(&document, bin.as_deref());
    let mut mesh = MeshData::default();

    let scene_index = document.get("scene").and_then(json::Value::as_u64).map(|value| value as usize);
    let scenes = document.get("scenes").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice);
    let scene = scene_index.and_then(|index| scenes.get(index)).or_else(|| scenes.first());

    if let Some(scene) = scene {
        for node in scene.get("nodes").and_then(json::Value::as_array).map_or(&[][..], Vec::as_slice) {
            if let Some(node_index) = node.as_u64() {
                gltf_append_node(&mut mesh, &document, node_index as usize, glb_identity(), &buffers)?;
            }
        }
    } else {
        let mesh_count = document.get("meshes").and_then(json::Value::as_array).map_or(0, Vec::len);
        for mesh_index in 0..mesh_count {
            gltf_append_mesh(&mut mesh, &document, mesh_index, &buffers, glb_identity())?;
        }
    }

    if mesh.indices.is_empty() {
        return Err("glb contains no triangle primitives".into());
    }
    Ok(mesh)
}
//#endregion 🔖️GltfCodec

fn f32_slice_to_bytes(values: &[f32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn u32_slice_to_bytes(values: &[u32]) -> Vec<u8> {
    values.iter().flat_map(|v| v.to_le_bytes()).collect()
}

fn pad_to_4(mut data: Vec<u8>) -> Vec<u8> {
    while !data.len().is_multiple_of(4) {
        data.push(0);
    }
    data
}

fn json_vec3_min(positions: &[f32]) -> String {
    let (min, _) = MeshData { positions: positions.to_vec(), ..Default::default() }.aabb();
    format!("[{}, {}, {}]", min[0], min[1], min[2])
}

fn json_vec3_max(positions: &[f32]) -> String {
    let (_, max) = MeshData { positions: positions.to_vec(), ..Default::default() }.aabb();
    format!("[{}, {}, {}]", max[0], max[1], max[2])
}
//#endregion Glb

//#region Stl
/// 🧱️ Hand-rolled binary STL: 80-byte header, `u32` little-endian triangle count, then per triangle a `f32x3` facet normal, three `f32x3` vertices, and a `u16` attribute-byte-count (written as 0). No vertex dedupe, matching the binary STL convention of one independent triangle per record.
pub fn mesh_to_stl(mesh: &MeshData) -> Vec<u8> {
    let triangle_count = mesh.triangle_count() as u32;
    let mut out = Vec::with_capacity(80 + 4 + triangle_count as usize * 50);
    out.extend_from_slice(&[0u8; 80]);
    out.extend_from_slice(&triangle_count.to_le_bytes());
    for tri in mesh.indices.as_chunks::<3>().0 {
        let p0 = stl_vertex(&mesh.positions, tri[0]);
        let p1 = stl_vertex(&mesh.positions, tri[1]);
        let p2 = stl_vertex(&mesh.positions, tri[2]);
        let normal = stl_face_normal(p0, p1, p2);
        for component in normal {
            out.extend_from_slice(&component.to_le_bytes());
        }
        for vertex in [p0, p1, p2] {
            for component in vertex {
                out.extend_from_slice(&component.to_le_bytes());
            }
        }
        out.extend_from_slice(&0u16.to_le_bytes());
    }
    out
}

pub fn mesh_from_stl(bytes: &[u8]) -> Result<MeshData, String> {
    if bytes.len() < 84 {
        return Err("stl: truncated header".into());
    }
    let triangle_count = u32::from_le_bytes(bytes[80..84].try_into().unwrap()) as usize;
    let expected_len = 84 + triangle_count * 50;
    if bytes.len() < expected_len {
        return Err("stl: truncated triangle data".into());
    }
    let mut mesh = MeshData::default();
    for triangle in 0..triangle_count {
        let base = 84 + triangle * 50;
        let mut normal = [0f32; 3];
        for axis in 0..3 {
            normal[axis] = f32::from_le_bytes(bytes[base + axis * 4..base + axis * 4 + 4].try_into().unwrap());
        }
        let vertex_base = base + 12;
        for corner in 0..3 {
            let corner_base = vertex_base + corner * 12;
            let mut position = [0f32; 3];
            for axis in 0..3 {
                position[axis] = f32::from_le_bytes(bytes[corner_base + axis * 4..corner_base + axis * 4 + 4].try_into().unwrap());
            }
            let index = (mesh.positions.len() / 3) as u32;
            mesh.positions.extend_from_slice(&position);
            mesh.normals.extend_from_slice(&normal);
            mesh.indices.push(index);
        }
    }
    Ok(mesh)
}

fn stl_vertex(positions: &[f32], index: u32) -> [f32; 3] {
    let base = index as usize * 3;
    [positions[base], positions[base + 1], positions[base + 2]]
}

fn stl_face_normal(a: [f32; 3], b: [f32; 3], c: [f32; 3]) -> [f32; 3] {
    let e0 = [b[0] - a[0], b[1] - a[1], b[2] - a[2]];
    let e1 = [c[0] - a[0], c[1] - a[1], c[2] - a[2]];
    let n = [e0[1] * e1[2] - e0[2] * e1[1], e0[2] * e1[0] - e0[0] * e1[2], e0[0] * e1[1] - e0[1] * e1[0]];
    let len = (n[0] * n[0] + n[1] * n[1] + n[2] * n[2]).sqrt();
    if len > 1e-8 {
        [n[0] / len, n[1] / len, n[2] / len]
    } else {
        [0.0, 0.0, 0.0]
    }
}
//#endregion Stl

//#region MeshCodec
/// 🔌️ Format-keyed mesh export codec; concrete implementations below are zero-dependency
/// (hand-rolled OBJ/GLB/STL). B-Rep apps additionally get `SolidExporter` (kernel/3d/brep/rs) which
/// wraps the real kernel's STEP/STL/OBJ writers, and reuse `GlbExporter`/`GlbImporter` here via a
/// tessellation bridge so GLB is the same codec everywhere. `format_kind` is the short stdio format
/// kind id (the legacy format enum was retired — ticket 26/08/11/
/// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6).
pub trait MeshExporter: Send + Sync {
    fn format_kind(&self) -> &'static str;
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String>;
}

/// 🔌️ Format-keyed mesh import codec; see `MeshExporter`.
pub trait MeshImporter: Send + Sync {
    fn format_kind(&self) -> &'static str;
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String>;
}

pub struct ObjExporter;
impl MeshExporter for ObjExporter {
    fn format_kind(&self) -> &'static str {
        "obj"
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_obj(mesh, "mesh").into_bytes())
    }
}

pub struct ObjImporter;
impl MeshImporter for ObjImporter {
    fn format_kind(&self) -> &'static str {
        "obj"
    }
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        let text = std::str::from_utf8(bytes).map_err(|error| error.to_string())?;
        mesh_from_obj(text)
    }
}

pub struct GlbExporter;
impl MeshExporter for GlbExporter {
    fn format_kind(&self) -> &'static str {
        "glb"
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_glb(mesh))
    }
}

pub struct GlbImporter;
impl MeshImporter for GlbImporter {
    fn format_kind(&self) -> &'static str {
        "glb"
    }
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        mesh_from_glb(bytes)
    }
}

pub struct StlExporter;
impl MeshExporter for StlExporter {
    fn format_kind(&self) -> &'static str {
        "stl"
    }
    fn export(&self, mesh: &MeshData) -> Result<Vec<u8>, String> {
        Ok(mesh_to_stl(mesh))
    }
}

pub struct StlImporter;
impl MeshImporter for StlImporter {
    fn format_kind(&self) -> &'static str {
        "stl"
    }
    fn import(&self, bytes: &[u8]) -> Result<MeshData, String> {
        mesh_from_stl(bytes)
    }
}
//#endregion MeshCodec

//#region IoError
/// ⚠️ Media IO error shared by ArtifactImport/Export and framework codecs. `Unsupported` carries the
/// unsupported format's kind id string (the legacy format enum was retired — ticket 26/08/11/
/// SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT W6).
#[derive(Clone, Debug, PartialEq)]
pub enum IoError {
    Format(String),
    Unsupported(String),
    Payload(String),
}

impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Format(m) => write!(f, "format: {m}"),
            Self::Unsupported(fmt) => write!(f, "unsupported: {fmt}"),
            Self::Payload(m) => write!(f, "payload: {m}"),
        }
    }
}

impl std::error::Error for IoError {}
//#endregion IoError

//#region 🧪️Tests
// 🧪 Relocated verbatim (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS
// G2) from `🧰️framework/🔨️modules/🔺️mesh/🦀️.rs`'s own `#[cfg(test)] mod tests` — that
// file's DOC COMMENT already said its mesh content "now dissolved into semio-framework-mesh-
// engine" (i.e. HERE), but its 20 tests exercising exactly this crate's own public functions
// (`mesh_box`/`mesh_from_obj`/`ObjExporter`/etc.) were left behind, orphaned, testing a module
// they no longer lived in. This region is that overdue move, landing them with the functions they
// actually exercise. The other 9 tests in that same old `mod tests` block exercised the unrelated
// DWG codec that file also held; those moved to `semio-s-plugin-stdio`'s `ac1024`/`🚪️io` instead.
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

//#region 🧪️GltfOracleDifferential
/// 🔬️ Differential test oracle: decodes the SAME `.glb`/`.gltf` bytes through the third-party
/// `gltf` crate (kept ONLY as a `[dev-dependencies]` reference here — never linked into any
/// production target, per ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-
/// ARTIFACTS) and asserts the DECODED STRUCTURE matches this crate's own first-party
/// `mesh_from_glb`. Compares structure, not bytes — glTF exporters are not byte-deterministic.
#[cfg(test)]
#[path = "🧪️tests/🔬️gltf-oracle-differential/🦀️.rs"]
mod gltf_oracle_differential;
//#endregion 🧪️GltfOracleDifferential

//#region 🧪️MeshDataJsonOracleDifferential
/// 🧪️ Validates `From<MeshData> for pack::json::Value` against `serde_json`, the third-party
/// oracle, rather than against a hand-written expectation: both must produce the same JSON for the
/// same mesh. This is what pins the camelCase renaming and the `skip_serializing_if` sparseness —
/// a first-party impl that silently emitted `face_ids`, or emitted `uvs: []` where serde omits the
/// key, would still compile and would still round-trip through our own reader, but would change the
/// bytes the viewer/editor windows put on the wire.
#[cfg(test)]
#[path = "🧪️tests/🔬️mesh-data-json-oracle/🦀️.rs"]
mod mesh_data_json_oracle_tests;
//#endregion 🧪️MeshDataJsonOracleDifferential

//#region 🧪️MeshDataFromValueRoundTrip
/// 🔄️ `FromValue` is the literal inverse of `ToValue`/`From<MeshData> for pack::json::Value` above:
/// `FromValue::from_value(ToValue::to_value(&mesh)) == mesh` for a dense mesh (every always-emitted
/// field only) and for one with every sparse field also populated. A differential oracle test proves
/// the SAME JSON this type's `ToValue` produces decodes through serde_json's own `Deserialize` to
/// the identical `MeshData` our first-party `FromValue` decodes from the equivalent `DslValue` tree
/// — not just that our own encode/decode pair agrees with itself.
#[cfg(test)]
#[path = "🧪️tests/🔬️mesh-data-from-value-round-trip/🦀️.rs"]
mod mesh_data_from_value_round_trip;
//#endregion 🧪️MeshDataFromValueRoundTrip

impl pack::value::retirement::RetireOwned for PolygonMeshSource {
    fn retirement(self)->Box<dyn pack::value::retirement::RetirementCursor> {pack::value::artifact_retirement_sequence![self.vertices,self.faces,self.attributes,self.materials,self.textures]}
}
impl pack::value::retirement::RetireOwned for MeshMetadataCursor {
    fn retirement(self)->Box<dyn pack::value::retirement::RetirementCursor> {pack::value::artifact_retirement_sequence![self.metadata_name,self.metadata_stack]}
}
impl pack::value::retirement::RetireOwned for MeshJsonTask {
    fn retirement(self)->Box<dyn pack::value::retirement::RetirementCursor> {let (Self::Node(path)|Self::Array(path,..)|Self::Object(path,..)|Self::Text(path,..))=self;pack::value::retirement::RetireOwned::retirement(path)}
}

#[path="🪆️binding/🦀️.rs"]
mod native_binding;
