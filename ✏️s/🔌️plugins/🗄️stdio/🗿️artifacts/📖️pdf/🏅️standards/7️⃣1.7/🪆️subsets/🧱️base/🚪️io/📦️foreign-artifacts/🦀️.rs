//! 📦️ Native PDF foreign artifacts preserve encoded custody behind first-party typed references.
use crate::standards::v1_7::subsets::base::{schema::snapshot::*,modules::lexer::{PResult,PdfEngineError}};
use semio_framework_artifact_reference::{ArtifactRef,ArtifactDialect};
use semio_framework_value::ToValue;

/// 🛂️ Native admission and resolution are the only access to foreign encoded bodies.
pub trait PdfArtifactResourcePort {
    fn admit(&mut self,kind:&str,object:PdfObject)->PResult<ArtifactRef>;
    fn resolve(&self,reference:&ArtifactRef)->PResult<PdfObject>;
}

/// 📚️ Owns foreign native resources independently of every semantic role body.
#[derive(Default)]
pub struct NativePdfArtifactResources<'a> {resources:Vec<(ArtifactRef,PdfObject)>,graph:&'a [PdfIndirectObject]}
fn identity(kind:&str,object:&PdfObject)->PResult<ArtifactRef> {
    let PdfObject::Stream {dict,data,filters}=object else{return Err(PdfEngineError::Unsupported("foreign PDF artifact requires a native stream".into()));};
    let mut hash=semio_framework_hash::Sha256::new();hash.update(kind.as_bytes());hash.update(data);
    let value=semio_framework_pack_json::from_dsl_value(&filters.to_value());hash.update(semio_framework_pack_json::to_string(&value).as_bytes());
    if kind=="s.stdio.font.type1" {for name in ["Length1","Length2","Length3"]{let entry=dict.iter().find(|entry|entry.key==name).map(|entry|&entry.value).unwrap_or(&PdfObject::Null);hash.update(semio_framework_pack_json::to_string(&semio_framework_pack_json::from_dsl_value(&entry.to_value())).as_bytes());}}
    let digest=hash.finalize().iter().map(|byte|format!("{byte:02x}")).collect::<String>();
    Ok(ArtifactRef {artifact_id:format!("pdf:{kind}:sha256:{digest}"),dialect:ArtifactDialect {artifact_kind:"s.stdio.binary".into(),standard:"raw".into(),subset:"*".into()}})
}
impl<'a> NativePdfArtifactResources<'a> {
    /// 📚️ Admits the native graph's existing foreign resources without semantic decoding.
    pub fn into_objects(self)->Vec<PdfIndirectObject>{self.resources.into_iter().enumerate().map(|(index,(_,value))|PdfIndirectObject{id:ObjRef{num:index as u32+1,gen:0},value}).collect()}
    pub fn from_objects(objects:&'a [PdfIndirectObject])->Self {Self {resources:Vec::new(),graph:objects}}

}
impl PdfArtifactResourcePort for NativePdfArtifactResources<'_> {
    fn admit(&mut self,kind:&str,object:PdfObject)->PResult<ArtifactRef> {let reference=identity(kind,&object)?;if !self.resources.iter().any(|(current,_)|current==&reference){self.resources.push((reference.clone(),object));}Ok(reference)}
    fn resolve(&self,reference:&ArtifactRef)->PResult<PdfObject> {
        if let Some((_,object))=self.resources.iter().find(|(current,_)|current==reference){return Ok(object.clone());}
        let kind=reference.artifact_id.strip_prefix("pdf:").and_then(|id|id.split_once(":sha256:")).map(|(kind,_)|kind).ok_or_else(||PdfEngineError::Unsupported("foreign PDF artifact identity is invalid".into()))?;
        for object in self.graph {if matches!(object.value,PdfObject::Stream {..})&&identity(kind,&object.value)?==*reference{return Ok(object.value.clone());}}
        Err(PdfEngineError::Unsupported(format!("foreign PDF artifact {} has not been admitted",reference.artifact_id)))
    }

}

/// 🎟️ Admits a detached native resource and retains its original representation in the native graph.
pub fn admit_pdf_artifact(snapshot:&mut PdfSnapshot,kind:&str,object:PdfObject)->PResult<ArtifactRef> {
    let reference=identity(kind,&object)?;let number=snapshot.objects.iter().map(|object|object.id.num).max().unwrap_or(0).checked_add(1).ok_or_else(||PdfEngineError::Unsupported("PDF native resource identity exhausted".into()))?;
    snapshot.objects.push(PdfIndirectObject {id:ObjRef {num:number,gen:0},value:object});Ok(reference)
}

/// 🛂️ Refuses a native resource reference outside the requested physical artifact kind.
pub fn require_pdf_artifact_kind(reference:&ArtifactRef,kind:&str)->PResult<()> {if reference.dialect.artifact_kind!="s.stdio.binary"||reference.dialect.standard!="raw"||reference.dialect.subset!="*"||!reference.artifact_id.starts_with(&format!("pdf:{kind}:sha256:")){return Err(PdfEngineError::Unsupported("foreign PDF artifact kind does not match its semantic resource".into()));}Ok(())}
/// 🔤️ Binds a semantic font variant to its actual independent native resource kind.
pub fn require_font_artifact_kind(program:&PdfFontProgram)->PResult<()> {let kind=match program {PdfFontProgram::Type1 {..}=>"type1",PdfFontProgram::TrueType {..}=>"truetype",PdfFontProgram::Cff {..}=>"cff",PdfFontProgram::CidCff {..}=>"cid-cff",PdfFontProgram::OpenType {..}=>"opentype"};require_pdf_artifact_kind(program.reference(),&format!("s.stdio.font.{kind}"))}
