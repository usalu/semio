//! 🏛️ Original verified catalog declarations retain provenance without inventing executables.
use crate::{os_store as store,os_directory::{DocumentOpenPackageV1,DocumentOpenArtifactV1,DocumentOpenSurfaceV1,DocumentOpenGrantV1}};
use crate::os_directory::schema::{DocumentOpenBrowserActorV1,DocumentBrowserActorSourceV1};
use semio_framework_artifact_reference::{ArtifactDialect,io::text::artifact_reference::DialectCoordinateText as _};
use std::{borrow::Cow,collections::BTreeMap,sync::{OnceLock,RwLock}};

/// 🔌️ Distinguishes genuine linked capabilities from declarations awaiting Guest installation.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum ArtifactCatalogCapability {
 UnlinkedGuest,
 Linked{native_identity:store::ArtifactNativeSnapshotIdentity,sqlite_schema:Option<Cow<'static,str>>,factory:Option<String>},
}
impl ArtifactCatalogCapability {
 /// 📥️ Captures exact executable metadata from the original verified native factory binding.
 pub fn linked(codec:&store::ArtifactCodec,factory:Option<String>)->Self{Self::Linked{native_identity:codec.native_identity.clone(),sqlite_schema:codec.snapshot_sqlite.as_ref().map(|provider|provider.schema.clone()),factory}}
}
/// 🪟️ Retains the complete original selected surface and its effective authority.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ArtifactCatalogTarget {pub parent_dialect:ArtifactDialect,pub surface:DocumentOpenSurfaceV1,pub grant:DocumentOpenGrantV1,pub browser_actor:DocumentOpenBrowserActorV1}
/// 🧾️ Retains host and codec-owner provenance independently before SQL capability filtering.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ArtifactCatalogBinding {pub contributor:DocumentOpenPackageV1,pub owner:DocumentOpenPackageV1,pub artifact:DocumentOpenArtifactV1,pub target:Option<ArtifactCatalogTarget>,pub capability:ArtifactCatalogCapability}
fn digest(value:&str)->bool{value.len()==64&&value.bytes().all(|byte|byte.is_ascii_digit()||(b'a'..=b'f').contains(&byte))&&value.bytes().any(|byte|byte!=b'0')}
fn package(value:&DocumentOpenPackageV1)->bool{[value.plugin_id.as_str(),value.package_id.as_str(),value.version.as_str()].into_iter().all(|value|!value.is_empty()&&!value.chars().any(char::is_control))&&[value.component_sha256.as_str(),value.component_blake3.as_str(),value.descriptor_byte_sha256.as_str()].into_iter().all(digest)&&value.execution_protocol.app_channel_version==crate::os_spr::CHANNEL_VERSION}
impl ArtifactCatalogBinding {
 pub(crate) fn key(&self)->Vec<String>{
  let p=&self.contributor;let mut key=vec![p.plugin_id.clone(),p.package_id.clone(),p.version.clone(),self.artifact.kind.clone(),self.artifact.schema.clone()];
  if let Some(target)=&self.target{let s=&target.surface;key.extend([target.parent_dialect.to_coordinate(),s.surface_id.clone(),s.app_id.clone(),s.window_kind_id.clone(),match s.role{crate::os_directory::DocumentOpenSurfaceRoleV1::Viewer=>"viewer",crate::os_directory::DocumentOpenSurfaceRoleV1::Editor=>"editor"}.into(),s.renderer_target.as_str().into()]);}
  key
 }
 fn validates(&self,codecs:&[store::ArtifactCodec])->bool{
  package(&self.contributor)&&package(&self.owner)&&!self.artifact.kind.is_empty()&&!self.artifact.schema.is_empty()&&digest(&self.artifact.pack_schema_hash)
   &&self.target.as_ref().is_none_or(|target|{let s=&target.surface;let coordinate=target.parent_dialect.to_coordinate();ArtifactDialect::parse_coordinate(&coordinate).as_ref()==Ok(&target.parent_dialect)&&[s.surface_id.as_str(),s.app_id.as_str(),s.window_kind_id.as_str()].into_iter().all(|value|!value.is_empty()&&!value.chars().any(char::is_control))&&s.surface_id==s.app_id&&target.browser_actor.validate(DocumentBrowserActorSourceV1{component_sha256:&self.contributor.component_sha256,descriptor_byte_sha256:&self.contributor.descriptor_byte_sha256},s.renderer_target.as_str()).is_ok()})
   &&match &self.capability{ArtifactCatalogCapability::UnlinkedGuest=>true,ArtifactCatalogCapability::Linked{native_identity,sqlite_schema,factory}=>factory.as_ref().is_none_or(|factory|!factory.is_empty())&&match native_identity{store::ArtifactNativeSnapshotIdentity::Typed{..}=>true,store::ArtifactNativeSnapshotIdentity::Guest{plugin_id,package_hash,schema}=>plugin_id==&self.owner.plugin_id&&crate::os_directory::io::binary::artifact_hash::hex_lower(package_hash)==self.owner.component_blake3&&schema==&self.artifact.schema}&&codecs.iter().any(|codec|codec.schema==self.artifact.schema&&codec.native_identity==*native_identity&&codec.snapshot_sqlite.as_ref().map(|provider|&provider.schema)==sqlite_schema.as_ref()&&codec.native_identity.validates_provider(&codec.schema,codec.snapshot_sqlite.as_ref())&&crate::os_directory::io::binary::artifact_hash::hex_lower(&codec.pack_schema_hash)==self.artifact.pack_schema_hash)}
 }
}
pub(crate) type CatalogMap=BTreeMap<Vec<String>,ArtifactCatalogBinding>;
static CATALOG:OnceLock<RwLock<CatalogMap>>=OnceLock::new();
pub(crate) fn registry()->&'static RwLock<CatalogMap>{CATALOG.get_or_init(||RwLock::new(BTreeMap::new()))}
/// 🚫️ A declaration is invalid, duplicated, or changes previously installed provenance.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum ArtifactCatalogBindingError{Unavailable,Duplicate{schema:String},Invalid{schema:String},Conflict{schema:String}}
impl std::fmt::Display for ArtifactCatalogBindingError{fn fmt(&self,f:&mut std::fmt::Formatter<'_>)->std::fmt::Result{match self{Self::Unavailable=>f.write_str("catalog declaration registry unavailable"),Self::Duplicate{schema}=>write!(f,"duplicate original catalog declaration for {schema}"),Self::Invalid{schema}=>write!(f,"invalid original catalog declaration for {schema}"),Self::Conflict{schema}=>write!(f,"conflicting original catalog provenance for {schema}")}}}
impl std::error::Error for ArtifactCatalogBindingError{}
pub(crate) fn propose(existing:&CatalogMap,rows:&[ArtifactCatalogBinding],codecs:&[store::ArtifactCodec])->Result<CatalogMap,ArtifactCatalogBindingError>{
 let mut proposed=CatalogMap::new();
 for row in rows{if !row.validates(codecs){return Err(ArtifactCatalogBindingError::Invalid{schema:row.artifact.schema.clone()})}let key=row.key();if proposed.contains_key(&key){return Err(ArtifactCatalogBindingError::Duplicate{schema:row.artifact.schema.clone()})}if existing.get(&key).is_some_and(|installed|installed!=row){return Err(ArtifactCatalogBindingError::Conflict{schema:row.artifact.schema.clone()})}proposed.insert(key,row.clone());}
 Ok(proposed)
}
/// 🔐️ Validates declarations under the caller's original assembly barrier.
pub fn preflight_artifact_catalog_bindings_in_assembly(_assembly:&semio_framework_schema_registry::assembly::Transaction,rows:&[ArtifactCatalogBinding],codecs:&[store::ArtifactCodec])->Result<(),ArtifactCatalogBindingError>{let registry=registry().read().map_err(|_|ArtifactCatalogBindingError::Unavailable)?;propose(&registry,rows,codecs).map(|_|())}
/// 📚️ Lists all published declarations, including authentic unlinked Guest rows.
pub fn artifact_catalog_bindings()->Result<Vec<ArtifactCatalogBinding>,ArtifactCatalogBindingError>{let _assembly=semio_framework_schema_registry::assembly::begin().map_err(|_|ArtifactCatalogBindingError::Unavailable)?;let registry=registry().read().map_err(|_|ArtifactCatalogBindingError::Unavailable)?;Ok(registry.values().cloned().collect())}
