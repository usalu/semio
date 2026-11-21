//! 🧵️ Admits canonical polygon payloads and retains their original vector backing.

use super::*;
use pack::value::{DslValue,retirement::RetireOwned};

#[derive(semio_framework_value_derive::RetireOwned)]
#[value(send_only)]
struct SourceRows<T:RetireOwned> {values:Vec<T>,cursor:usize}
impl<T:RetireOwned> Default for SourceRows<T> {fn default()->Self {Self::new(Vec::new())}}
impl<T:RetireOwned> SourceRows<T> {
    fn new(values:Vec<T>)->Self {Self {values,cursor:0}}
    fn shift(&mut self,empty:T)->Option<T> {
        let value=self.values.get_mut(self.cursor)?;
        self.cursor+=1;
        Some(std::mem::replace(value,empty))
    }
}
impl SourceRows<DslValue> {fn next(&mut self)->Option<DslValue>{self.shift(DslValue::Null)}}
impl SourceRows<(String,DslValue)> {fn next(&mut self)->Option<(String,DslValue)>{self.shift((String::new(),DslValue::Null))}}

/// 🔎️ Parses bounded indexed polygon source without reconstructing geometry.
pub fn parse_polygon_mesh_source(text:&str)->Result<PolygonMeshSource,String> {
    let mut preparation=PolygonSourcePreparation::new();
    let mut idle=0usize;
    let mut turns=0usize;
    loop {
        turns+=1;if turns>text.len().saturating_mul(4).saturating_add(100_000){return Err("mesh source preparation exceeded its turn bound".into())}
        let grant=preparation.cold_grant(text,4096,4096)?;
        let mut wallet=pack::value::AuthoringWallet::new(grant.retirement,COLD_WORK);
        let Some(done)=wallet.run_invoked(grant.retirement,COLD_STEP_WORK,|effect|preparation.step(text,grant,effect)).map_err(|error|error.to_string())? else {idle+=1;if idle>=COLD_IDLE_TURNS{return Err("mesh source preparation was denied its quoted work".into())}continue};
        let step=done.output?;
        if let Some(source)=step.source {return Ok(source);}
        idle=0;
    }
}

const COLD_WORK:u128=1_000_000_000;
const COLD_STEP_WORK:u64=1<<20;
const COLD_IDLE_TURNS:usize=64;

/// 🎛️ Keeps projection work and retirement currencies independent.
#[derive(Clone,Copy,Debug)]
pub struct PolygonSourceGrant {pub maximum_units:usize,pub maximum_projection_bytes:usize,pub retirement:pack::value::retained_clone::RetainedCloneGrant}

/// 🪜️ Returns physical closure receipts alongside the admitted canonical source.
pub struct PolygonSourceStep {pub source:Option<PolygonMeshSource>,pub retirement:pack::value::retained_clone::RetainedCloneProgress}

fn admit_source_retirement<T:pack::value::retirement::RetireOwned>(slot:&mut Option<Box<dyn pack::value::ErasedSnapshotRetirement>>,value:&mut Option<T>,effect:&mut pack::value::RetainedAllocationEffect<'_>)->Result<pack::value::retained_clone::RetainedCloneProgress,String>{
    use pack::value::retained_clone::RetainedCloneProgress;
    let grant=effect.grant();let bytes=pack::value::retirement::owned_retirement_birth_bytes::<T>();
    if grant.maximum_items==0||grant.maximum_capacity_bytes<bytes{return Ok(RetainedCloneProgress::default());}
    if value.is_none(){return Err("mesh retirement lost its original owner".into());}
    match pack::value::retirement::admit_owned_retirement(value,effect).result{Ok(Some((owner,progress)))=>{*slot=Some(owner);Ok(progress)},Ok(None)=>Ok(RetainedCloneProgress::default()),Err(error)=>Err(error.to_string())}
}

fn mesh_source_decode_admission(control:&semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,demand:pack::value::RetirementDemand)->Result<(),String>{
 if control.owned_bytes().checked_add(demand.capacity_bytes).is_none_or(|bytes|bytes>256*1024*1024){return Err("mesh native source exceeds cumulative allocation limit".into())}Ok(())
}

/// 🧵️ Captures, projects, and admits the existing polygon payload under retained work grants.
#[derive(semio_framework_value_derive::RetireOwned)]
#[value(send_only)]
pub struct PolygonSourcePreparation {
    parser:Option<json::JsonParseCursor>,projection:Option<json::JsonValueProjection>,decoding:Option<semio_framework_value_native_owned::native_decoding::NativeDecodeContinuation>,stage:u8,
    raw:Option<pack::value::DslValue>,vertices:SourceRows<pack::value::DslValue>,faces:SourceRows<pack::value::DslValue>,
    surface: SourceRows<pack::value::DslValue>,
    surface_seen: HistoryFoldIndex<(u8, u32), ()>,
    normal_receipt: [usize; 4],
    attributes:SourceRows<(String,pack::value::DslValue)>,materials:SourceRows<(String,pack::value::DslValue)>,textures:SourceRows<(String,pack::value::DslValue)>,
    source:PolygonMeshSource,face:Vec<u32>,indices:SourceRows<pack::value::DslValue>,seen:Vec<u32>,corners:usize,
    current:Option<(String,pack::value::DslValue)>,attribute:Option<(String,MeshAttribute)>,texture:Option<(String,MeshTexture)>,
    garbage:Vec<pack::value::DslValue>,index_backing:Option<Vec<pack::value::DslValue>>,retirement:Option<Box<dyn pack::value::ErasedSnapshotRetirement>>,name:Option<String>,index:usize,field:usize,texture_bytes:usize,
}
impl PolygonSourcePreparation {
    /// 🌱️ Starts without reading or cloning source text.
    pub fn new()->Self {Self {parser:Some(json::JsonParseCursor::new(json::JsonMemberPolicy::Reject)),projection:None,decoding:None,stage:0,raw:None,vertices:SourceRows::default(),faces:SourceRows::default(),surface:SourceRows::default(),surface_seen:HistoryFoldIndex::new(),normal_receipt:Default::default(),attributes:SourceRows::default(),materials:SourceRows::default(),textures:SourceRows::default(),source:PolygonMeshSource {vertices:Vec::new(),faces:Vec::new(),surface:Vec::new(),attributes:HistoryFoldIndex::new(),materials:HistoryFoldIndex::new(),textures:HistoryFoldIndex::new()},face:Vec::new(),indices:SourceRows::default(),seen:Default::default(),corners:0,current:None,attribute:None,texture:None,garbage:Vec::new(),index_backing:None,retirement:None,name:None,index:0,field:0,texture_bytes:0}}
    /// 📍️ Exposes the current existing source preparation phase.
    pub fn phase(&self)->&'static str {match self.stage {0=>"mesh-source-parse",1..=7=>"mesh-source-project",_=>"mesh-source-admit"}}
    /// 📏️ Computes exact closure demands for explicit cold IO callers.
    pub fn cold_grant(&self,text:&str,maximum_units:usize,maximum_projection_bytes:usize)->Result<PolygonSourceGrant,String>{
        use pack::value::{retained_clone::RetainedCloneGrant,retirement::controlled::controlled_retirement_birth_demand};
        let demand=match &self.retirement{
            Some(owner)=>pack::value::factory_ticket_demands(owner,0).map_err(|error|error.to_string())?,
            None if self.index_backing.is_some()=>controlled_retirement_birth_demand(self.index_backing.as_ref().unwrap()).map_err(|error|error.to_string())?,
            None if self.stage==1&&self.parser.is_some()=>controlled_retirement_birth_demand(self.parser.as_ref().unwrap()).map_err(|error|error.to_string())?,
            None if self.stage==2&&self.projection.is_some()=>controlled_retirement_birth_demand(self.projection.as_ref().unwrap()).map_err(|error|error.to_string())?,
            None if self.stage==0=>self.parser.as_ref().ok_or("mesh parser lost its original owner")?.normal_step_demands(text).map_err(|error|error.to_string())?,
            None if self.stage==1=>self.projection.as_ref().ok_or("mesh projection lost its original owner")?.normal_step_demands().map_err(|error|error.to_string())?,
            None if self.stage==10&&!self.garbage.is_empty()=>controlled_retirement_birth_demand(&self.garbage).map_err(|error|error.to_string())?,
            None=>pack::value::RetirementDemand{depth:1,..Default::default()},
        };
        Ok(PolygonSourceGrant{maximum_units,maximum_projection_bytes,retirement:RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth.max(1)}})
    }
    /// 🧾️ Carries parser and retirement effects on successful and faulting turns.
    pub fn normal_step_retained_progress(&self) -> pack::value::retained_clone::RetainedCloneProgress {
        let [copied_items, copied_bytes, retained_capacity_bytes, released_bytes] = self.normal_receipt;
        pack::value::retained_clone::RetainedCloneProgress { copied_items, copied_bytes, retained_capacity_bytes, released_bytes }
    }

    fn record_receipt(&mut self, progress: pack::value::retained_clone::RetainedCloneProgress) {
        self.normal_receipt = [progress.copied_items, progress.copied_bytes, progress.retained_capacity_bytes, progress.released_bytes];
    }

    /// 💼️ Advances projection work or one independently admitted closure turn under the calling turn's effect.
    pub fn step(&mut self,text:&str,grant:PolygonSourceGrant,effect:&mut pack::value::RetainedAllocationEffect<'_>)->Result<PolygonSourceStep,String> {
        self.normal_receipt = Default::default();
        let result = self.advance(text, grant, effect);
        if let Ok(step) = &result { self.record_receipt(step.retirement); }
        result
    }

    fn advance(&mut self,text:&str,grant:PolygonSourceGrant,effect:&mut pack::value::RetainedAllocationEffect<'_>)->Result<PolygonSourceStep,String> {
        let waiting=|retirement|PolygonSourceStep{source:None,retirement};
        if text.len()>16_000_000{return Err("mesh input exceeds 16 MB".into());}
        if grant.maximum_units==0{return Ok(waiting(Default::default()));}
        if self.retirement.is_some(){let closed=match pack::value::close_factory_ticket(&mut self.retirement,grant.retirement.maximum_copy_bytes,effect){pack::value::RetainedWorkPoll::Complete(turn)=>turn.result.map_err(|error|error.to_string())?.progress(),pack::value::RetainedWorkPoll::Pending{..}=>Default::default()};return Ok(waiting(closed));}
        if self.index_backing.is_some(){return Ok(waiting(admit_source_retirement(&mut self.retirement,&mut self.index_backing,effect)?));}
        if self.stage==1&&self.parser.is_some(){return Ok(waiting(admit_source_retirement(&mut self.retirement,&mut self.parser,effect)?));}
        if self.stage==2&&self.projection.is_some(){return Ok(waiting(admit_source_retirement(&mut self.retirement,&mut self.projection,effect)?));}
        if self.stage==10&&!self.garbage.is_empty(){let bytes=pack::value::retirement::owned_retirement_birth_bytes::<Vec<pack::value::DslValue>>();if grant.retirement.maximum_items==0||grant.retirement.maximum_capacity_bytes<bytes{return Ok(waiting(Default::default()));}let mut original=Some(std::mem::take(&mut self.garbage));let admitted=admit_source_retirement(&mut self.retirement,&mut original,effect);if let Some(original)=original{self.garbage=original;}return admitted.map(waiting);}
        let maximum_bytes=grant.maximum_projection_bytes;
        if maximum_bytes==0{return Ok(waiting(Default::default()));}
        for _ in 0..grant.maximum_units {
            match self.stage {
                0=>{let mut accepted=|_|true;let mut control=match self.decoding.take(){Some(receipt)=>semio_framework_value_native_owned::native_decoding::NativeDecodeControl::resume(receipt,&mut accepted).map_err(|(error,receipt)|{self.decoding=Some(receipt);error.to_string()})?,None=>semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new_retained(&mut accepted)};let parser=self.parser.as_mut().unwrap();let parsed=(||{mesh_source_decode_admission(&control,parser.normal_step_demands(text).map_err(|error|error.to_string())?)?;parser.step(text,1,&mut control,effect).map_err(|error|error.to_string())})();let progress=parser.normal_step_progress();self.record_receipt(progress);self.decoding=Some(control.pause().map_err(|(error,_)|error.to_string())?);if let Some(value)=parsed? {self.projection=Some(json::JsonValueProjection::new(value));self.stage=1;}if !progress.fits(grant.retirement){return Err("mesh original parser/projection receipt exceeds caller grant".into());}return Ok(waiting(progress));},
                1=>{let mut accepted=|_|true;let receipt=self.decoding.take().ok_or("mesh native decoding continuation is absent")?;let mut control=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::resume(receipt,&mut accepted).map_err(|(error,receipt)|{self.decoding=Some(receipt);error.to_string()})?;let projection=self.projection.as_mut().unwrap();let projected=(||{mesh_source_decode_admission(&control,projection.normal_step_demands().map_err(|error|error.to_string())?)?;projection.step(1,maximum_bytes,&mut control,effect).map_err(|error|error.to_string())})();let progress=projection.normal_step_progress();self.record_receipt(progress);self.decoding=Some(control.pause().map_err(|(error,_)|error.to_string())?);if let Some(value)=projected? {self.raw=Some(value);self.stage=2;}if !progress.fits(grant.retirement){return Err("mesh original parser/projection receipt exceeds caller grant".into());}return Ok(waiting(progress));},
                2=>self.admit_root()?,
                3=>{if let Some(value)=self.vertices.next() {self.raw=Some(value);let values=self.raw.as_ref().unwrap().as_array().filter(|values|values.len()==3).ok_or("vertex must have three coordinates")?;let mut point=[0.0;3];for axis in 0..3 {point[axis]=values[axis].as_f64().filter(|number|number.is_finite() && number.abs()<=f32::MAX as f64).ok_or("coordinate must be finite")? as f32;}self.source.vertices.push(point);self.seen.push(0);self.garbage.push(self.raw.take().unwrap());}else {self.stage=4;}},
                4=>self.project_face()?,
                5=>self.project_attribute()?,
                6=>{if let Some((name,value))=self.materials.next() {self.current=Some((name,value));let (name,value)=self.current.as_ref().unwrap();if !mesh_name_valid(name) || value.as_object().is_none() {return Err("invalid owned mesh material".into());}let (name,value)=self.current.take().unwrap();self.source.materials.insert(name,value);}else {self.stage=7;}},
                7=>self.project_texture()?,
                8=>self.admit_attribute()?,
                9=>self.admit_material()?,
                11=>self.project_surface()?,
                10=>{if !self.garbage.is_empty(){return Ok(waiting(Default::default()));}self.stage=12;},
                _=>return Ok(PolygonSourceStep{source:Some(std::mem::replace(&mut self.source,PolygonMeshSource {vertices:Vec::new(),faces:Vec::new(),surface:Vec::new(),attributes:HistoryFoldIndex::new(),materials:HistoryFoldIndex::new(),textures:HistoryFoldIndex::new()})),retirement:Default::default()}),
            }
        }
        Ok(waiting(Default::default()))
    }
    fn admit_root(&mut self)->Result<(),String> {
        let fields=self.raw.as_ref().unwrap().as_object().filter(|fields|fields.len()<=6 && fields.iter().all(|(name,_)|["vertices","faces","attributes","materials","textures","surface"].contains(&name.as_str()))).ok_or("unknown mesh field")?;
        for (field,minimum,maximum) in [("vertices",3,100_000),("faces",1,100_000)] {let values=fields.iter().find(|(name,_)|name==field).and_then(|(_,value)|value.as_array()).ok_or_else(||format!("{field} must be an array"))?;if !(minimum..=maximum).contains(&values.len()) {return Err("mesh requires 3..100000 vertices and 1..100000 faces".into());}}
        for (field,maximum) in [("attributes",64),("materials",10_000),("textures",256)] {if let Some((_,value))=fields.iter().find(|(name,_)|name==field) {if value.as_object().is_none_or(|fields|fields.len()>maximum) {return Err("mesh asset declaration limit exceeded".into());}}}
        if fields.iter().find(|(name,_)| name == "surface").is_some_and(|(_,value)| value.as_array().is_none_or(|samples| samples.len() > 1_400_000)) { return Err("mesh surface declaration limit exceeded".into()); }
        let Some(pack::value::DslValue::Object(fields))=self.raw.take() else {unreachable!()};
        for (name,value) in fields {match (name.as_str(),value) {("vertices",pack::value::DslValue::Array(values))=>self.vertices=SourceRows::new(values),("faces",pack::value::DslValue::Array(values))=>self.faces=SourceRows::new(values),("surface",pack::value::DslValue::Array(values))=>self.surface=SourceRows::new(values),("attributes",pack::value::DslValue::Object(values))=>self.attributes=SourceRows::new(values),("materials",pack::value::DslValue::Object(values))=>self.materials=SourceRows::new(values),("textures",pack::value::DslValue::Object(values))=>self.textures=SourceRows::new(values),_=>unreachable!()}}
        self.stage=3;Ok(())
    }
    fn replace_indices(&mut self,values:Vec<pack::value::DslValue>) {
        let original=std::mem::replace(&mut self.indices,SourceRows::new(values)).values;
        if original.capacity()!=0 {self.index_backing=Some(original);}
    }
    fn project_face(&mut self)->Result<(),String> {
        if let Some(value)=self.indices.next() {self.raw=Some(value);let id=self.raw.as_ref().unwrap().as_u64().filter(|id|*id<self.source.vertices.len() as u64).ok_or("face index out of range")? as u32;let generation=self.source.faces.len() as u32+1;if self.seen[id as usize]==generation{return Err("face contains a repeated vertex".into());}self.seen[id as usize]=generation;self.face.push(id);self.garbage.push(self.raw.take().unwrap());return Ok(());}
        if !self.face.is_empty() {self.source.faces.push(std::mem::take(&mut self.face));return Ok(());}
        if let Some(value)=self.faces.next() {self.raw=Some(value);let values=self.raw.as_ref().unwrap().as_array().filter(|values|values.len()>=3).ok_or("face requires at least three indices")?;self.corners=self.corners.saturating_add(values.len());if self.corners>600_000 {return Err("mesh exceeds 600000 polygon corners".into());}let Some(pack::value::DslValue::Array(values))=self.raw.take() else {unreachable!()};self.replace_indices(values);}else {self.stage=11;}Ok(())
    }
    fn project_surface(&mut self) -> Result<(), String> {
        fn components<const N: usize>(value: &pack::value::DslValue) -> Option<[f32; N]> {
            let fields = value.as_array().filter(|fields| fields.len() == N)?;
            let mut result = [0.0; N];
            for (component, field) in result.iter_mut().zip(fields) {
                *component = field.as_f64().filter(|value| value.is_finite() && value.abs() <= f32::MAX as f64)? as f32;
            }
            Some(result)
        }

        const INVALID: &str = "invalid mesh surface sample";
        let Some(value) = self.surface.next() else { self.stage = 5; return Ok(()); };
        self.raw = Some(value);
        let value = self.raw.as_ref().unwrap();
        let fields = value.as_object().filter(|fields| (2..=3).contains(&fields.len())).ok_or(INVALID)?;
        let kind = match value.get("kind").and_then(pack::value::DslValue::as_str) {
            Some("smoothFace") => PolygonSurfaceKind::SmoothFace,
            Some("vertexNormal") => PolygonSurfaceKind::VertexNormal,
            Some("cornerUv") => PolygonSurfaceKind::CornerUv,
            Some("seamEdge") => PolygonSurfaceKind::SeamEdge,
            _ => return Err(INVALID.into()),
        };
        let index = value.get("index").and_then(pack::value::DslValue::as_u64).filter(|index| *index < 600_000).ok_or(INVALID)? as u32;
        let normal = value.get("normal").map(|field| components(field).ok_or(INVALID)).transpose()?;
        let uv = value.get("uv").map(|field| components(field).ok_or(INVALID)).transpose()?;
        if fields.len() != 2 + usize::from(normal.is_some()) + usize::from(uv.is_some()) {
            return Err(INVALID.into());
        }
        let sample = PolygonSurfaceSample { kind, index, normal, uv };
        let limit = match (sample.kind, sample.normal, sample.uv) {
            (PolygonSurfaceKind::SmoothFace, None, None) => self.source.faces.len(),
            (PolygonSurfaceKind::VertexNormal, Some(normal), None) if normal != [0.0; 3] => self.source.vertices.len(),
            (PolygonSurfaceKind::CornerUv, None, Some(_)) | (PolygonSurfaceKind::SeamEdge, None, None) => self.corners,
            _ => return Err("mesh surface sample has invalid fields".into()),
        };
        if sample.index as usize >= limit || self.surface_seen.insert((sample.kind as u8, sample.index), ()).is_some() {
            return Err("mesh surface index is duplicate or out of range".into());
        }
        self.source.surface.push(sample);
        self.garbage.push(self.raw.take().unwrap());
        Ok(())
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
        for (key,value) in fields {match (key.as_str(),value) {("values",pack::value::DslValue::Array(values))=>attribute.values=values,("indices",pack::value::DslValue::Array(values))=>{self.replace_indices(values);attribute.indices=Some(Vec::new());},(_,value)=>self.garbage.push(value)}}
        self.attribute=Some((name,attribute));Ok(())
    }
    fn project_texture(&mut self)->Result<(),String> {
        if let Some((_,texture))=&mut self.texture {if let Some(value)=self.indices.next() {self.raw=Some(value);texture.bytes.push(u8::try_from(self.raw.as_ref().unwrap().as_u64().ok_or("invalid mesh texture byte")?).map_err(|_|"invalid mesh texture byte")?);self.garbage.push(self.raw.take().unwrap());return Ok(());}let (name,texture)=self.texture.take().unwrap();self.source.textures.insert(name,texture);return Ok(());}
        let Some(value)=self.textures.next() else {self.stage=8;self.name=None;return Ok(())};self.current=Some(value);let (name,value)=self.current.as_ref().unwrap();if !mesh_name_valid(name) {return Err("mesh asset declaration limit exceeded".into());}
        let fields=value.as_object().filter(|fields|fields.len()==2 && fields.iter().all(|(name,_)|["mime","bytes"].contains(&name.as_str()))).ok_or("invalid owned mesh texture")?;
        let mime=fields.iter().find(|(name,_)|name=="mime").and_then(|(_,value)|value.as_str()).filter(|mime|mesh_name_valid(mime)).ok_or("invalid owned mesh texture")?;
        let _=mime;let bytes=fields.iter().find(|(name,_)|name=="bytes").and_then(|(_,value)|value.as_array()).ok_or("invalid owned mesh texture")?;self.texture_bytes=self.texture_bytes.saturating_add(bytes.len());if self.texture_bytes>16_000_000 {return Err("invalid owned mesh texture".into());}
        let (name,pack::value::DslValue::Object(fields))=self.current.take().unwrap() else {unreachable!()};let mut texture=MeshTexture {mime:String::new(),bytes:Vec::new()};for (key,value) in fields {match (key.as_str(),value) {("mime",pack::value::DslValue::String(value))=>texture.mime=value,("bytes",pack::value::DslValue::Array(values))=>self.replace_indices(values),_=>unreachable!()}}self.texture=Some((name,texture));Ok(())
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

