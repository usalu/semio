//! 📝️ Mesh JSON and OBJ representation owners.
use crate::*;
use semio_framework_pack_json as json;
use std::collections::BTreeMap;

/// 🎒️ Encodes canonical polygon values at the native JSON boundary.
pub fn encode_polygon_mesh_source(source:&PolygonMeshSource)->String {json::from_dsl_value(&pack::value::ToValue::to_value(source)).to_string()}

/// 📜️ Reads polygon-preserving OBJ through the cooperative source cursor.
pub fn polygon_mesh_source_from_obj(text:&str)->Result<PolygonMeshSource,String> {let mut cursor=ObjSourceCursor::new();loop {if let Some(source)=cursor.step(text,4096)?{return Ok(source);}}}

/// 📜️ Incremental form of [`polygon_mesh_source_from_obj`]: reads at most the granted number of lines per step so a host can stay inside an interactive step ceiling.
pub struct ObjSourceCursor {offset:usize,lines:usize,vertices:Vec<[f32;3]>,faces:Vec<Vec<u32>>}

impl Default for ObjSourceCursor {fn default()->Self {Self::new()}}

impl ObjSourceCursor {
    pub fn new()->Self {Self {offset:0,lines:0,vertices:Vec::new(),faces:Vec::new()}}

    /// 📊 Bytes consumed and the byte length of `text`.
    pub fn progress(&self,text:&str)->(usize,usize) {(self.offset,text.len())}

    /// ⏱️ Reads up to `maximum_lines` lines of `text`; the source once the whole text is read and every face index is checked.
    pub fn step(&mut self,text:&str,maximum_lines:usize)->Result<Option<PolygonMeshSource>,String> {
        if text.len()>16_000_000 {return Err("obj: input exceeds 16 MB".into());}
        for _ in 0..maximum_lines.max(1) {
            if self.offset>=text.len() {
                if self.faces.iter().flatten().any(|index|*index as usize>=self.vertices.len()) {return Err("obj: face index out of range".into());}
                return Ok(Some(PolygonMeshSource {vertices:std::mem::take(&mut self.vertices),faces:std::mem::take(&mut self.faces),attributes:HistoryFoldIndex::new(),materials:HistoryFoldIndex::new(),textures:HistoryFoldIndex::new()}));
            }
            let rest=&text[self.offset..];
            let end=rest.find('\n').unwrap_or(rest.len());
            let line=&rest[..end];
            self.offset+=end+1;
            self.lines+=1;
            let mut parts=line.split_whitespace();
            match parts.next() {
                Some("v")=>{
                    let coordinates=parts.take(3).map(|value|value.parse::<f32>().ok().filter(|number|number.is_finite())).collect::<Option<Vec<_>>>().filter(|coordinates|coordinates.len()==3).ok_or_else(||format!("obj: malformed v line {}",self.lines))?;
                    self.vertices.push([coordinates[0],coordinates[1],coordinates[2]]);
                }
                Some("f")=>{
                    let mut face=Vec::new();
                    for token in parts {
                        let raw:i64=token.split('/').next().unwrap_or_default().parse().map_err(|_|format!("obj: malformed face index on line {}",self.lines))?;
                        face.push(obj_resolve_index(raw,self.vertices.len())? as u32);
                    }
                    if face.len()>=3 {self.faces.push(face);}
                }
                _=>{}
            }
        }
        Ok(None)
    }
}

/// 🔎️ Parses bounded indexed polygon source without reconstructing geometry.
pub fn parse_polygon_mesh_source(text:&str)->Result<PolygonMeshSource,String> {
    let mut preparation=PolygonSourcePreparation::new();
    loop {let grant=preparation.cold_grant(4096,4096)?;if let Some(source)=preparation.step(text,grant)?.source{return Ok(source);}}
}

/// 🎛️ Keeps projection work and retirement currencies independent.
#[derive(Clone,Copy,Debug)]
pub struct PolygonSourceGrant {pub maximum_units:usize,pub maximum_projection_bytes:usize,pub retirement:pack::value::retained_clone::RetainedCloneGrant}

/// 📊️ Returns physical closure receipts alongside the admitted canonical source.
pub struct PolygonSourceStep {pub source:Option<PolygonMeshSource>,pub retirement:pack::value::retained_clone::RetainedCloneProgress}

fn admit_source_retirement<T:pack::value::retirement::RetireOwned>(slot:&mut Option<Box<dyn pack::value::ErasedSnapshotRetirement>>,value:&mut Option<T>,grant:pack::value::retained_clone::RetainedCloneGrant)->Result<pack::value::retained_clone::RetainedCloneProgress,String>{
    use pack::value::retained_clone::RetainedCloneProgress;
    let bytes=pack::value::retirement::owned_retirement_birth_bytes::<T>();
    if grant.maximum_items==0||grant.maximum_capacity_bytes<bytes{return Ok(RetainedCloneProgress::default());}
    let original=value.take().ok_or("mesh retirement lost its original owner")?;
    match pack::value::retirement::admit_owned_retirement(original,grant){Ok((owner,progress))=>{*slot=Some(owner);Ok(progress)},Err((error,original))=>{*value=Some(original);Err(error.to_string())}}
}

/// 🧵️ Captures, projects, and admits the existing polygon payload under retained work grants.
pub struct PolygonSourcePreparation {
    parser:Option<json::JsonParseCursor>,projection:Option<json::JsonValueProjection>,decoding:Option<pack::value::native_decoding::NativeDecodeContinuation>,stage:u8,
    raw:Option<pack::value::DslValue>,vertices:std::vec::IntoIter<pack::value::DslValue>,faces:std::vec::IntoIter<pack::value::DslValue>,
    attributes:std::vec::IntoIter<(String,pack::value::DslValue)>,materials:std::vec::IntoIter<(String,pack::value::DslValue)>,textures:std::vec::IntoIter<(String,pack::value::DslValue)>,
    source:PolygonMeshSource,face:Vec<u32>,indices:std::vec::IntoIter<pack::value::DslValue>,seen:Vec<u32>,corners:usize,
    current:Option<(String,pack::value::DslValue)>,attribute:Option<(String,MeshAttribute)>,texture:Option<(String,MeshTexture)>,
    garbage:Vec<pack::value::DslValue>,retirement:Option<Box<dyn pack::value::ErasedSnapshotRetirement>>,name:Option<String>,index:usize,field:usize,texture_bytes:usize,
}
impl PolygonSourcePreparation {
    /// 🌱️ Starts without reading or cloning source text.
    pub fn new()->Self {Self {parser:Some(json::JsonParseCursor::new(json::JsonMemberPolicy::Reject)),projection:None,decoding:None,stage:0,raw:None,vertices:Vec::new().into_iter(),faces:Vec::new().into_iter(),attributes:Vec::new().into_iter(),materials:Vec::new().into_iter(),textures:Vec::new().into_iter(),source:PolygonMeshSource {vertices:Vec::new(),faces:Vec::new(),attributes:HistoryFoldIndex::new(),materials:HistoryFoldIndex::new(),textures:HistoryFoldIndex::new()},face:Vec::new(),indices:Vec::new().into_iter(),seen:Default::default(),corners:0,current:None,attribute:None,texture:None,garbage:Vec::new(),retirement:None,name:None,index:0,field:0,texture_bytes:0}}
    /// 📍️ Exposes the current existing source preparation phase.
    pub fn phase(&self)->&'static str {match self.stage {0=>"mesh-source-parse",1..=7=>"mesh-source-project",_=>"mesh-source-admit"}}
    /// 📏️ Computes exact closure demands for explicit cold IO callers.
    pub fn cold_grant(&self,maximum_units:usize,maximum_projection_bytes:usize)->Result<PolygonSourceGrant,String>{
        use pack::value::{retained_clone::RetainedCloneGrant,retirement::owned_retirement_birth_bytes};
        let demand=match &self.retirement{Some(owner)=>pack::value::factory_ticket_demands(owner,0).map_err(|error|error.to_string())?,None=>pack::value::RetirementDemand{capacity_bytes:if self.stage==1&&self.parser.is_some(){owned_retirement_birth_bytes::<json::JsonParseCursor>()}else if self.stage==2&&self.projection.is_some(){owned_retirement_birth_bytes::<json::JsonValueProjection>()}else if self.stage==10&&!self.garbage.is_empty(){owned_retirement_birth_bytes::<Vec<pack::value::DslValue>>()}else{0},depth:1,..Default::default()}};
        Ok(PolygonSourceGrant{maximum_units,maximum_projection_bytes,retirement:RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)}})
    }
    /// ⏱️ Advances projection work or one independently admitted closure turn.
    pub fn step(&mut self,text:&str,grant:PolygonSourceGrant)->Result<PolygonSourceStep,String>{
        let waiting=|retirement|PolygonSourceStep{source:None,retirement};
        if text.len()>16_000_000{return Err("mesh input exceeds 16 MB".into());}
        if grant.maximum_units==0{return Ok(waiting(Default::default()));}
        if self.retirement.is_some(){let closed=pack::value::close_factory_ticket(&mut self.retirement,grant.retirement).map_err(|error|error.to_string())?;return Ok(waiting(closed.progress()));}
        if self.stage==1&&self.parser.is_some(){return Ok(waiting(admit_source_retirement(&mut self.retirement,&mut self.parser,grant.retirement)?));}
        if self.stage==2&&self.projection.is_some(){return Ok(waiting(admit_source_retirement(&mut self.retirement,&mut self.projection,grant.retirement)?));}
        if self.stage==10&&!self.garbage.is_empty(){let bytes=pack::value::retirement::owned_retirement_birth_bytes::<Vec<pack::value::DslValue>>();if grant.retirement.maximum_items==0||grant.retirement.maximum_capacity_bytes<bytes{return Ok(waiting(Default::default()));}let mut original=Some(std::mem::take(&mut self.garbage));let admitted=admit_source_retirement(&mut self.retirement,&mut original,grant.retirement);if let Some(original)=original{self.garbage=original;}return admitted.map(waiting);}
        let maximum_bytes=grant.maximum_projection_bytes;
        if maximum_bytes==0{return Ok(waiting(Default::default()));}
        for _ in 0..grant.maximum_units {
            match self.stage {
                0=>{let mut accepted=|_|true;let mut control=match self.decoding.take(){Some(receipt)=>pack::value::NativeDecodeControl::resume(receipt,&mut accepted),None=>Ok(pack::value::NativeDecodeControl::new(256*1024*1024,&mut accepted))}.map_err(|error|error.to_string())?;let parsed=self.parser.as_mut().unwrap().step(text,1,&mut control);self.decoding=Some(control.pause().map_err(|error|error.to_string())?);if let Some(value)=parsed.map_err(|error|error.to_string())? {self.projection=Some(json::JsonValueProjection::new(value));self.stage=1;return Ok(waiting(Default::default()));}},
                1=>{let mut accepted=|_|true;let mut control=pack::value::NativeDecodeControl::resume(self.decoding.take().unwrap(),&mut accepted).map_err(|error|error.to_string())?;let projected=self.projection.as_mut().unwrap().step(1,maximum_bytes,&mut control);self.decoding=Some(control.pause().map_err(|error|error.to_string())?);if let Some(value)=projected.map_err(|error|error.to_string())? {self.raw=Some(value);self.stage=2;return Ok(waiting(Default::default()));}},
                2=>self.admit_root()?,
                3=>{if let Some(value)=self.vertices.next() {self.raw=Some(value);let values=self.raw.as_ref().unwrap().as_array().filter(|values|values.len()==3).ok_or("vertex must have three coordinates")?;let mut point=[0.0;3];for axis in 0..3 {point[axis]=values[axis].as_f64().filter(|number|number.is_finite() && number.abs()<=f32::MAX as f64).ok_or("coordinate must be finite")? as f32;}self.source.vertices.push(point);self.seen.push(0);self.garbage.push(self.raw.take().unwrap());}else {self.stage=4;}},
                4=>self.project_face()?,
                5=>self.project_attribute()?,
                6=>{if let Some((name,value))=self.materials.next() {self.current=Some((name,value));let (name,value)=self.current.as_ref().unwrap();if !mesh_name_valid(name) || value.as_object().is_none() {return Err("invalid owned mesh material".into());}let (name,value)=self.current.take().unwrap();self.source.materials.insert(name,value);}else {self.stage=7;}},
                7=>self.project_texture()?,
                8=>self.admit_attribute()?,
                9=>self.admit_material()?,
                10=>{if !self.garbage.is_empty(){return Ok(waiting(Default::default()));}self.stage=11;},
                _=>return Ok(PolygonSourceStep{source:Some(std::mem::replace(&mut self.source,PolygonMeshSource {vertices:Vec::new(),faces:Vec::new(),attributes:HistoryFoldIndex::new(),materials:HistoryFoldIndex::new(),textures:HistoryFoldIndex::new()})),retirement:Default::default()}),
            }
        }
        Ok(waiting(Default::default()))
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
        if let Some(value)=self.indices.next() {self.raw=Some(value);let id=self.raw.as_ref().unwrap().as_u64().filter(|id|*id<self.source.vertices.len() as u64).ok_or("face index out of range")? as u32;let generation=self.source.faces.len() as u32+1;if self.seen[id as usize]==generation{return Err("face contains a repeated vertex".into());}self.seen[id as usize]=generation;self.face.push(id);self.garbage.push(self.raw.take().unwrap());return Ok(());}
        if !self.face.is_empty() {self.source.faces.push(std::mem::take(&mut self.face));return Ok(());}
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
impl pack::value::retirement::RetireOwned for PolygonSourcePreparation {
    fn retirement(self)->Box<dyn pack::value::retirement::RetirementCursor> {let fields=pack::value::retirement::sequence(vec![pack::value::retirement::deferred(self.parser),pack::value::retirement::deferred(self.projection),pack::value::retirement::deferred(self.raw),pack::value::retirement::deferred(self.vertices),pack::value::retirement::deferred(self.faces),pack::value::retirement::deferred(self.attributes),pack::value::retirement::deferred(self.materials),pack::value::retirement::deferred(self.textures),pack::value::retirement::deferred(self.source),pack::value::retirement::deferred(self.face),pack::value::retirement::deferred(self.indices),pack::value::retirement::deferred(self.seen),pack::value::retirement::deferred(self.current),pack::value::retirement::deferred(self.attribute),pack::value::retirement::deferred(self.texture),pack::value::retirement::deferred(self.garbage),pack::value::retirement::deferred(self.name)]);if let Some(retirement)=self.retirement {pack::value::retirement::sequence(vec![pack::value::retirement::erased_cursor(retirement),fields])}else {fields}}
    fn retirement_birth_bytes(&self)->Option<usize> {let fields=pack::value::retirement::sequence_birth_bytes(&[pack::value::retirement::deferred_birth_bytes_for(&self.parser),pack::value::retirement::deferred_birth_bytes_for(&self.projection),pack::value::retirement::deferred_birth_bytes_for(&self.raw),pack::value::retirement::deferred_birth_bytes_for(&self.vertices),pack::value::retirement::deferred_birth_bytes_for(&self.faces),pack::value::retirement::deferred_birth_bytes_for(&self.attributes),pack::value::retirement::deferred_birth_bytes_for(&self.materials),pack::value::retirement::deferred_birth_bytes_for(&self.textures),pack::value::retirement::deferred_birth_bytes_for(&self.source),pack::value::retirement::deferred_birth_bytes_for(&self.face),pack::value::retirement::deferred_birth_bytes_for(&self.indices),pack::value::retirement::deferred_birth_bytes_for(&self.seen),pack::value::retirement::deferred_birth_bytes_for(&self.current),pack::value::retirement::deferred_birth_bytes_for(&self.attribute),pack::value::retirement::deferred_birth_bytes_for(&self.texture),pack::value::retirement::deferred_birth_bytes_for(&self.garbage),pack::value::retirement::deferred_birth_bytes_for(&self.name)])?;if self.retirement.is_some() {pack::value::retirement::sequence_birth_bytes(&[pack::value::retirement::erased_cursor_birth_bytes(),fields])}else {Some(fields)}}
    fn controlled_retirement_supported()->bool {true}
}

/// 🎒️ Retained metadata cursor shared by mesh JSON and preview packing.
#[derive(Default)]
pub struct MeshMetadataCursor {
    metadata_section:u8,metadata_stage:u8,metadata_name:Option<String>,metadata_records:usize,metadata_value:usize,metadata_stack:Vec<MeshJsonTask>,
}
impl MeshMetadataCursor {
    /// 🧵️ Appends one bounded metadata transition using borrowed canonical values.
    pub fn step(&mut self, attributes:&HistoryFoldIndex<String,MeshAttribute>,materials:&HistoryFoldIndex<String,pack::value::DslValue>,textures:&HistoryFoldIndex<String,MeshTexture>,references:Option<&crate::ComponentReferenceTable>,corners:Option<&[u32]>,edges:Option<&[u32]>,output:&mut String)->Result<bool,String> {
        use std::ops::Bound::{Excluded,Unbounded};
        if attributes.len()>64 || materials.len()>10_000 || textures.len()>256 {return Err("mesh metadata declaration limit exceeded".into());}
        if references.is_some_and(|values|values.len()>3) {return Err("mesh component reference domain limit exceeded".into());}
        if self.metadata_section==4 {return Ok(true); }
        if self.metadata_stage==0 {
            let (field,empty)=match self.metadata_section {0=>("attributes",attributes.is_empty()),1=>("materials",materials.is_empty()),2=>("textures",textures.is_empty()),_=>("componentReferences",references.is_none_or(crate::ComponentReferenceTable::is_empty))};
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

impl pack::value::retirement::RetireOwned for MeshMetadataCursor {
    fn retirement(self)->Box<dyn pack::value::retirement::RetirementCursor> {pack::value::retirement::sequence(vec![pack::value::retirement::deferred(self.metadata_name),pack::value::retirement::deferred(self.metadata_stack)])}
    fn retirement_birth_bytes(&self)->Option<usize> {pack::value::retirement::sequence_birth_bytes(&[pack::value::retirement::deferred_birth_bytes_for(&self.metadata_name),pack::value::retirement::deferred_birth_bytes_for(&self.metadata_stack)])}
    fn controlled_retirement_supported()->bool {true}
}
impl pack::value::retirement::RetireOwned for MeshJsonTask {
    fn retirement(self)->Box<dyn pack::value::retirement::RetirementCursor> {let (Self::Node(path)|Self::Array(path,..)|Self::Object(path,..)|Self::Text(path,..))=self;pack::value::retirement::deferred(path)}
    fn retirement_birth_bytes(&self)->Option<usize> {let (Self::Node(path)|Self::Array(path,..)|Self::Object(path,..)|Self::Text(path,..))=self;Some(pack::value::retirement::deferred_birth_bytes_for(path))}
    fn controlled_retirement_supported()->bool {true}
}


#[path="🧬️fields/🦀️.rs"]
mod fields;

#[cfg(test)]
#[path="🧪️tests/🧬️ownership/🦀️.rs"]
mod neutral_ownership_tests;
