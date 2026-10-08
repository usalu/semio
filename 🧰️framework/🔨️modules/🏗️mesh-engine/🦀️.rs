//! 🔺️ Canonical mesh values, channel semantics, and geometry construction.

// 🚫️async: R7 — `MeshExporter`/`MeshImporter` are first-party AFIT traits; Send is obtained
// structurally at the concrete-enum call site per R3, never via a `+ Send` bound on the trait
// method, so rustc's `async_fn_in_trait` lint (which would suggest exactly that bound) is
// silenced here rather than resolved by its own suggestion.
#![allow(async_fn_in_trait)]

pub use protocol::causal::{HistoryFoldIndex,HistoryFoldIndexIntoIter};

#[path="🎯️component-references/🦀️.rs"]
mod component_references;
pub use component_references::ComponentReferenceTable;

#[cfg(test)]
#[path="🎯️component-references/🧪️tests/🦀️.rs"]
mod component_reference_custody_tests;
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
    #[value(default,skip_serializing_if="HistoryFoldIndex::is_empty")]
    pub attributes:HistoryFoldIndex<String,MeshAttribute>,
    #[value(default,skip_serializing_if="HistoryFoldIndex::is_empty")]
    pub materials:HistoryFoldIndex<String,pack::value::DslValue>,
    #[value(default,skip_serializing_if="HistoryFoldIndex::is_empty")]
    pub textures:HistoryFoldIndex<String,MeshTexture>,
}

impl PolygonMeshSource {
    /// 🌐️ Closed latitude-longitude sphere around the Y axis: pole fans plus quad bands, wound outward, with shared vertices.
    pub fn uv_sphere(radius:f32,segments:u32,rings:u32)->Result<Self,String> {
        if !radius.is_finite() || radius<=0.0 || !(3..=1024).contains(&segments) || !(2..=1024).contains(&rings) || segments as usize*rings as usize>PARAMETRIC_FACE_LIMIT {return Err("invalid uv sphere dimensions".into());}
        let (segments,rings)=(segments as usize,rings as usize);
        let mut vertices=vec![[0.0,radius,0.0]];
        for ring in 1..rings {let phi=ring as f64/rings as f64*std::f64::consts::PI;for segment in 0..segments {let theta=segment as f64/segments as f64*std::f64::consts::TAU;let r=radius as f64;vertices.push([(r*phi.sin()*theta.cos()) as f32,(r*phi.cos()) as f32,(r*phi.sin()*theta.sin()) as f32]);}}
        vertices.push([0.0,-radius,0.0]);
        let at=|ring:usize,segment:usize|(1+(ring-1)*segments+segment%segments) as u32;
        let south=(vertices.len()-1) as u32;
        let mut faces=Vec::with_capacity(segments*rings);
        for segment in 0..segments {faces.push(vec![0,at(1,segment+1),at(1,segment)]);}
        for ring in 1..rings-1 {for segment in 0..segments {faces.push(vec![at(ring,segment),at(ring,segment+1),at(ring+1,segment+1),at(ring+1,segment)]);}}
        for segment in 0..segments {faces.push(vec![south,at(rings-1,segment),at(rings-1,segment+1)]);}
        Ok(Self {vertices,faces,attributes:HistoryFoldIndex::new(),materials:HistoryFoldIndex::new(),textures:HistoryFoldIndex::new()})
    }

    /// 🍩️ Closed torus around the Y axis from quads wound outward: `segments` around the ring, `rings` around the tube.
    pub fn torus(major:f32,minor:f32,segments:u32,rings:u32)->Result<Self,String> {
        if !major.is_finite() || !minor.is_finite() || minor<=0.0 || major<=minor || !(3..=1024).contains(&segments) || !(3..=1024).contains(&rings) || segments as usize*rings as usize>PARAMETRIC_FACE_LIMIT {return Err("invalid torus dimensions".into());}
        let (segments,rings)=(segments as usize,rings as usize);
        let mut vertices=Vec::with_capacity(segments*rings);
        for ring in 0..rings {let phi=ring as f64/rings as f64*std::f64::consts::TAU;for segment in 0..segments {let theta=segment as f64/segments as f64*std::f64::consts::TAU;let distance=major as f64+minor as f64*phi.cos();vertices.push([(distance*theta.cos()) as f32,(minor as f64*phi.sin()) as f32,(distance*theta.sin()) as f32]);}}
        let at=|ring:usize,segment:usize|((ring%rings)*segments+segment%segments) as u32;
        let mut faces=Vec::with_capacity(segments*rings);
        for ring in 0..rings {for segment in 0..segments {faces.push(vec![at(ring,segment),at(ring+1,segment),at(ring+1,segment+1),at(ring,segment+1)]);}}
        Ok(Self {vertices,faces,attributes:HistoryFoldIndex::new(),materials:HistoryFoldIndex::new(),textures:HistoryFoldIndex::new()})
    }
}

const PARAMETRIC_FACE_LIMIT:usize=100_000;

/// ✅️ Validates owned polygon channels independently of geometry evaluation.
pub fn validate_polygon_mesh_attributes(vertex_count:usize,face_count:usize,halfedge_count:usize,attributes:&HistoryFoldIndex<String,MeshAttribute>,materials:&HistoryFoldIndex<String,pack::value::DslValue>,textures:&HistoryFoldIndex<String,MeshTexture>)->Result<(),String> {
    if vertex_count>100_000 || face_count>100_000 || halfedge_count>600_000 || attributes.len()>64 {return Err("mesh declaration capacity exceeded".into());}
    validate_mesh_surface_assets(materials,textures)?;
    for (name,attribute) in attributes {validate_mesh_attribute(name,attribute,vertex_count,face_count,halfedge_count,materials)?;}
    Ok(())
}

/// 🎨️ Validates material references and owned texture payload limits.
pub fn validate_mesh_surface_assets(materials:&HistoryFoldIndex<String,pack::value::DslValue>,textures:&HistoryFoldIndex<String,MeshTexture>)->Result<(),String> {
    if materials.len()>10_000 || textures.len()>256 || materials.keys().chain(textures.keys()).any(|name|!mesh_name_valid(name)) {return Err("mesh asset declaration limit exceeded".into());}
    let mut bytes=0usize;
    for texture in textures.values() {bytes=bytes.saturating_add(texture.bytes.len());if !mesh_name_valid(&texture.mime) || bytes>16_000_000 {return Err("invalid owned mesh texture".into());}}
    for (id,material) in materials {
        let fields=material.as_object().ok_or_else(||format!("mesh material '{id}' must be an owned object"))?;
        for (name,value) in fields {validate_mesh_material_field(id,name,value,textures)?;}
    }
    Ok(())
}

pub(crate) fn validate_mesh_material_field(id:&str,name:&str,value:&pack::value::DslValue,textures:&HistoryFoldIndex<String,MeshTexture>)->Result<(),String> {
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
pub fn validate_mesh_attribute(name:&str,attribute:&MeshAttribute,vertex_count:usize,face_count:usize,halfedge_count:usize,materials:&HistoryFoldIndex<String,pack::value::DslValue>)->Result<(),String> {
    if !mesh_name_valid(name) {return Err("mesh attribute declaration limit exceeded".into());}
    let count=match attribute.domain {MeshAttributeDomain::Vertex=>vertex_count,MeshAttributeDomain::Face=>face_count,_=>halfedge_count};
    if attribute.values.len()>600_000 || attribute.domain_len()!=count || attribute.indices.as_ref().is_some_and(|indices|indices.iter().any(|index|*index as usize>=attribute.values.len())) {return Err(format!("mesh attribute '{name}' cardinality does not match its domain"));}
    if attribute.values.is_empty() && attribute.interpolation==MeshAttributeInterpolation::Linear {return Err(format!("linear mesh attribute '{name}' requires finite numeric values"));}
    for value in &attribute.values {validate_mesh_attribute_sample(name,attribute,value,materials)?;}
    Ok(())
}

pub(crate) fn mesh_name_valid(name:&str)->bool {!name.is_empty() && name.chars().take(129).count()<=128}

fn mesh_attribute_numeric(value:&pack::value::DslValue)->Option<usize> {
    if value.as_f64().is_some_and(|number|number.is_finite() && number.abs()<=f32::MAX as f64) {return Some(0);}
    let tuple=value.as_array()?;(!tuple.is_empty() && tuple.len()<=16 && tuple.iter().all(|value|value.as_f64().is_some_and(|number|number.is_finite() && number.abs()<=f32::MAX as f64))).then_some(tuple.len())
}
pub(crate) fn validate_mesh_attribute_sample(name:&str,attribute:&MeshAttribute,value:&pack::value::DslValue,materials:&HistoryFoldIndex<String,pack::value::DslValue>)->Result<(),String> {
    let numeric=mesh_attribute_numeric;
    if attribute.interpolation==MeshAttributeInterpolation::Linear {let width=attribute.values.first().and_then(numeric).ok_or_else(||format!("linear mesh attribute '{name}' requires finite numeric values"))?;if numeric(value)!=Some(width) {return Err(format!("linear mesh attribute '{name}' requires compatible numeric dimensions"));}}
    let width=match attribute.semantic {MeshAttributeSemantic::Normal=>3,MeshAttributeSemantic::Uv=>2,MeshAttributeSemantic::Color=>4,_=>0};
    if width>0 && (!matches!(attribute.domain,MeshAttributeDomain::Vertex|MeshAttributeDomain::Corner|MeshAttributeDomain::Face) || numeric(value)!=Some(width)) {return Err(format!("mesh attribute '{name}' has an invalid semantic domain or dimensions"));}
    if name=="tangent" && (attribute.semantic!=MeshAttributeSemantic::Custom || !matches!(attribute.domain,MeshAttributeDomain::Vertex|MeshAttributeDomain::Corner|MeshAttributeDomain::Face) || numeric(value)!=Some(4) || value.as_array().is_none_or(|values|values[..3].iter().all(|value|value.as_f64()==Some(0.0)) || values[3].as_f64().is_none_or(|value|value!=1.0 && value!= -1.0))) {return Err("canonical tangent requires a nonzero finite four-component direction and handedness +/-1".into());}
    if attribute.semantic==MeshAttributeSemantic::Normal && value.as_array().unwrap().iter().all(|value|value.as_f64()==Some(0.0)) {return Err(format!("mesh normal '{name}' cannot be zero"));}
    if attribute.semantic==MeshAttributeSemantic::Material && (attribute.domain!=MeshAttributeDomain::Face || attribute.interpolation==MeshAttributeInterpolation::Linear || value.as_str().is_none_or(|name|!materials.contains_key(name))) {return Err(format!("mesh attribute '{name}' references an undefined face material"));}
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
    #[cfg_attr(test, serde(default, skip_serializing_if = "HistoryFoldIndex::is_empty"))]
    pub attributes: HistoryFoldIndex<String,MeshAttribute>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "HistoryFoldIndex::is_empty"))]
    pub materials: HistoryFoldIndex<String,pack::value::DslValue>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "HistoryFoldIndex::is_empty"))]
    pub textures: HistoryFoldIndex<String,MeshTexture>,
    #[cfg_attr(test, serde(default, skip_serializing_if = "ComponentReferenceTable::is_empty"))]
    pub component_references: ComponentReferenceTable,
}

pack::value::artifact_retire_struct!(MeshData {positions,normals,colors,indices,uvs,face_ids,vertex_ids,edge_positions,edge_ids,edge_uvs,edge_is_seam,paint_texture_base64,attributes,materials,textures,component_references});
impl pack::value::retirement::RetireOwned for MeshAttribute {
    fn retirement(self)->Box<dyn pack::value::retirement::RetirementCursor> {pack::value::retirement::sequence(vec![pack::value::retirement::deferred(self.values),pack::value::retirement::deferred(self.indices)])}
    fn retirement_birth_bytes(&self)->Option<usize> {pack::value::retirement::sequence_birth_bytes(&[pack::value::retirement::deferred_birth_bytes_for(&self.values),pack::value::retirement::deferred_birth_bytes_for(&self.indices)])}
    fn controlled_retirement_supported()->bool {true}
}
impl pack::value::retirement::RetireOwned for MeshTexture {
    fn retirement(self)->Box<dyn pack::value::retirement::RetirementCursor> {pack::value::retirement::sequence(vec![pack::value::retirement::deferred(self.mime),pack::value::retirement::deferred(self.bytes)])}
    fn retirement_birth_bytes(&self)->Option<usize> {pack::value::retirement::sequence_birth_bytes(&[pack::value::retirement::deferred_birth_bytes_for(&self.mime),pack::value::retirement::deferred_birth_bytes_for(&self.bytes)])}
    fn controlled_retirement_supported()->bool {true}
}

/// 🌱️ Projects canonical mesh members without any wire representation.
impl pack::value::ToValue for MeshData {
    fn to_value(&self)->pack::value::DslValue {
        use pack::value::{DslValue,ToValue};
        let mut fields=vec![("positions".into(),self.positions.to_value()),("normals".into(),self.normals.to_value()),("colors".into(),self.colors.to_value()),("indices".into(),self.indices.to_value())];
        macro_rules! sparse {($name:literal,$field:ident)=>{if !self.$field.is_empty(){fields.push(($name.into(),self.$field.to_value()));}};}
        sparse!("uvs",uvs);sparse!("faceIds",face_ids);sparse!("vertexIds",vertex_ids);sparse!("edgePositions",edge_positions);sparse!("edgeIds",edge_ids);sparse!("edgeUvs",edge_uvs);sparse!("edgeIsSeam",edge_is_seam);
        if let Some(texture)=&self.paint_texture_base64{fields.push(("paintTextureBase64".into(),texture.to_value()));}
        sparse!("attributes",attributes);sparse!("materials",materials);sparse!("textures",textures);sparse!("componentReferences",component_references);
        DslValue::Object(fields)
    }
}

/// 🔀️ Admits canonical mesh members and validates component references.
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
            attributes: field("attributes").map(HistoryFoldIndex::<String,MeshAttribute>::from_value).transpose()?.unwrap_or_default(),
            materials: field("materials").map(HistoryFoldIndex::<String,pack::value::DslValue>::from_value).transpose()?.unwrap_or_default(),
            textures: field("textures").map(HistoryFoldIndex::<String,MeshTexture>::from_value).transpose()?.unwrap_or_default(),
            component_references: field("componentReferences").map(ComponentReferenceTable::from_value).transpose()?.unwrap_or_default(),
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

impl pack::value::retirement::RetireOwned for PolygonMeshSource {
    fn retirement(self)->Box<dyn pack::value::retirement::RetirementCursor> {pack::value::retirement::sequence(vec![pack::value::retirement::deferred(self.vertices),pack::value::retirement::deferred(self.faces),pack::value::retirement::deferred(self.attributes),pack::value::retirement::deferred(self.materials),pack::value::retirement::deferred(self.textures)])}
    fn retirement_birth_bytes(&self)->Option<usize> {pack::value::retirement::sequence_birth_bytes(&[pack::value::retirement::deferred_birth_bytes_for(&self.vertices),pack::value::retirement::deferred_birth_bytes_for(&self.faces),pack::value::retirement::deferred_birth_bytes_for(&self.attributes),pack::value::retirement::deferred_birth_bytes_for(&self.materials),pack::value::retirement::deferred_birth_bytes_for(&self.textures)])}
    fn controlled_retirement_supported()->bool {true}
}
#[path="🚪️io/🦀️.rs"]
pub mod io;
