//! 📚️ Canonical IDML package identity and strict ZIP/XML container admission.
use semio_s_artifact_stdio_zip::{ZipSnapshot,schema::snapshot::ZipEntry,standards::v2_0::subsets::base::io::{decode_zip,encode_zip}};
use semio_s_artifact_stdio_xml::{schema::snapshot::XmlDocument,standards::v1_0::subsets::base::io::text::snapshot::xml_document_from_text};
pub const ARTIFACT_KIND:&str="s.stdio.idml";
pub const STANDARD:&str="8.0";
pub const PACKAGE_MIME:&str="application/vnd.adobe.indesign-idml-package";
pub const MAX_PACKAGE_BYTES:usize=8<<20;
pub const MAX_PACKAGE_PARTS:usize=16_384;
#[derive(Clone,Debug,PartialEq)]
pub struct IdmlPackage{dom_version:String,archive:ZipSnapshot}
fn path(name:&str)->Result<(),String>{if name.is_empty()||name.starts_with('/')||name.contains('\\')||name.contains(':')||name.split('/').any(|part|part.is_empty()||part=="."||part==".."){Err("stdio.idml.member-path".into())}else{Ok(())}}
fn document(bytes:&[u8])->Result<XmlDocument,String>{let doc=xml_document_from_text(std::str::from_utf8(bytes).map_err(|error|error.to_string())?)?;if doc.doctype.is_some(){return Err("stdio.idml.doctype".into());}Ok(doc)}
fn decoded_version(bytes:&[u8],view:io::admission::IdmlVersionView)->Result<String,String>{
    let mut text=String::with_capacity(view.decoded_length);let mut decoder=io::admission::IdmlVersionDecoder::new(view);
    loop{let step=decoder.advance(bytes,65536);if let Some(error)=step.error{return Err(error.into());}if let Some(byte)=step.byte{text.push(byte as char);}if step.done{return Ok(text);}}
}

fn validate(archive:ZipSnapshot)->Result<IdmlPackage,String>{
    use io::admission::{IdmlAdmissionCursor,IdmlAdmissionGrant,IdmlAdmissionSource};
    if archive.entries.first().is_none_or(|entry|entry.metadata.compression_method!=0){return Err("stdio.idml.mimetype".into());}
    let mut cursor=IdmlAdmissionCursor::new();
    let (error,version)=loop{
        let step=cursor.advance(IdmlAdmissionSource::parts(&archive),IdmlAdmissionGrant{maximum_work:65536,maximum_copy_bytes:8192,maximum_capacity_bytes:4096,maximum_release_bytes:0});
        if step.done||step.error.is_some(){break(step.error,step.version);}
    };
    cursor.begin_close();while cursor.next_release_bytes()!=0{let quote=cursor.next_release_bytes();cursor.close_one(quote);}
    if let Some(error)=error{return Err(error.into());}
    let version=version.ok_or("stdio.idml.dom-version")?;
    let dom_version=decoded_version(&archive.entries[version.part_index].data,version)?;
    Ok(IdmlPackage{dom_version,archive})
}

impl IdmlPackage{
    pub fn dom_version(&self)->&str{&self.dom_version}
    pub fn into_archive(self)->ZipSnapshot{self.archive}
    pub fn from_archive(archive:ZipSnapshot)->Result<Self,String>{validate(archive)}
    pub fn parts(&self)->impl Iterator<Item=(&str,&[u8])>{self.archive.entries.iter().map(|entry|(entry.name.as_str(),entry.data.as_slice()))}
    pub fn part(&self,name:&str)->Option<&[u8]>{self.archive.entries.iter().find(|entry|entry.name==name).map(|entry|entry.data.as_slice())}
    pub fn xml_part(&self,name:&str)->Result<XmlDocument,String>{path(name)?;document(self.part(name).ok_or("stdio.idml.missing-member")?)}
    pub fn from_parts(parts:Vec<(String,Vec<u8>)>)->Result<Self,String>{let mut archive=ZipSnapshot::default();archive.entries=parts.into_iter().map(|(name,data)|{let mut entry=ZipEntry{name,data,..ZipEntry::default()};if entry.name=="mimetype"{entry.metadata.compression_method=0;}entry}).collect();validate(archive)}
    pub fn decode(bytes:&[u8])->Result<Self,String>{if bytes.len()>MAX_PACKAGE_BYTES{return Err("stdio.idml.byte-limit".into());}validate(decode_zip(bytes).map_err(|error|error.to_string())?)}
    pub fn encode(&self)->Result<Vec<u8>,String>{let bytes=encode_zip(&self.archive).map_err(|error|error.to_string())?;if bytes.len()>MAX_PACKAGE_BYTES{return Err("stdio.idml.byte-limit".into());}Ok(bytes)}
}

#[path = "🧬️schema/🦀️.rs"]
pub mod schema;


pub const DOCUMENT_SCHEMA: &str = "stdio.idml";
pub use schema::{snapshot::IdmlSnapshot, diff::IdmlDiff, mutations::IdmlMutation};
#[path = "🚪️io/🦀️.rs"]
pub mod io;
pub use io::{native_record_spec, schema_hash};

#[path = "🧾️artifact-declaration/🦀️.rs"]
mod artifact_declaration;
pub use artifact_declaration::{ARTIFACT_DEFINITION_SCHEMA, definition, declaration, artifact_kind, formats, native_codecs, assembly, contribution};

#[path="🏭️issuer/🦀️.rs"]pub mod issuer;
