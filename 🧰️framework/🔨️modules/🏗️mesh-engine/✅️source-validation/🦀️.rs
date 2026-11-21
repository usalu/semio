//! ✅️ Validates owned mesh source samples and material references.

use super::*;

/// 🕸️ Validates material references and owned texture payload limits.
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

/// 🗂️ Validates one indexed channel using its declared domain and interpolation rule.
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

