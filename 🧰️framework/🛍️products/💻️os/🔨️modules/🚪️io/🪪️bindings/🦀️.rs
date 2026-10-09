//! 🪪️ Immutable codec publication rows retain original provenance before capability filtering.
use crate::os_store as store;
use semio_framework_artifact_reference::ArtifactDialect;
use semio_framework_artifact_reference::io::text::artifact_reference::DialectCoordinateText as _;
use std::{borrow::Cow,collections::BTreeMap,sync::{OnceLock,RwLock}};
#[path="🏛️catalog/🦀️.rs"]
pub mod catalog;

/// 🧭️ Names the actual authored publication boundary independently from SQL availability.
#[derive(Clone,Copy,Debug,PartialEq,Eq,PartialOrd,Ord)]
pub enum ArtifactCodecBindingChannel { OwnedDeclaration, HostedDeclaration, ForeignApp, TreeSubset, DirectNative, DirectDocumentNative, SchemaOnlyDocument, Guest }

/// 🧾️ Retains one original binding and every optional authored join without inventing owners.
#[derive(Clone,Debug,PartialEq,Eq)]
pub struct ArtifactCodecBinding {
 pub channel:ArtifactCodecBindingChannel,
 pub contributor:String,
 pub artifact_kind:Option<String>,
 pub apps:Vec<String>,
 pub schema:String,
 pub dialect:Option<ArtifactDialect>,
 pub native_identity:store::ArtifactNativeSnapshotIdentity,
 pub sqlite_schema:Option<Cow<'static,str>>,
 pub factory:Option<String>,
}

impl ArtifactCodecBinding {
 /// 📥️ Captures exact owner and capability metadata before an original publisher filters it.
 pub fn from_codec(channel:ArtifactCodecBindingChannel,contributor:impl Into<String>,artifact_kind:Option<String>,apps:Vec<String>,dialect:Option<ArtifactDialect>,factory:Option<String>,codec:&store::ArtifactCodec)->Self {
  Self{channel,contributor:contributor.into(),artifact_kind,apps,schema:codec.schema.clone(),dialect,native_identity:codec.native_identity.clone(),sqlite_schema:codec.snapshot_sqlite.as_ref().map(|provider|provider.schema.clone()),factory}
 }
 /// 👁️ Retains an original schema-only publication without assigning it a dialect or app.
 pub fn schema_only(codec:&store::ArtifactCodec)->Self {
  let contributor=match &codec.native_identity{store::ArtifactNativeSnapshotIdentity::Typed{owner,..}=>owner.to_string(),store::ArtifactNativeSnapshotIdentity::Guest{plugin_id,..}=>plugin_id.clone()};
  Self::from_codec(ArtifactCodecBindingChannel::SchemaOnlyDocument,contributor,None,Vec::new(),None,None,codec)
 }
 fn key(&self)->BindingKey{let mut apps=self.apps.clone();apps.sort();BindingKey{channel:self.channel,contributor:self.contributor.clone(),artifact_kind:self.artifact_kind.clone(),apps,schema:self.schema.clone(),dialect:self.dialect.clone()}}
 pub(crate) fn validates(&self,codec:&store::ArtifactCodec)->bool {
  let shape=match self.channel {
   ArtifactCodecBindingChannel::SchemaOnlyDocument=>self.artifact_kind.is_none()&&self.dialect.is_none()&&self.apps.is_empty()&&self.factory.is_none()&&self.contributor==match &codec.native_identity{store::ArtifactNativeSnapshotIdentity::Typed{owner,..}=>*owner,store::ArtifactNativeSnapshotIdentity::Guest{plugin_id,..}=>plugin_id.as_str()},
   ArtifactCodecBindingChannel::ForeignApp=>self.artifact_kind.is_none()&&self.dialect.is_some()&&self.apps.len()==1,
   ArtifactCodecBindingChannel::Guest=>self.artifact_kind.is_some()&&self.dialect.is_some()&&matches!(&codec.native_identity,store::ArtifactNativeSnapshotIdentity::Guest{plugin_id,..}if plugin_id==&self.contributor),
   _=>self.artifact_kind.is_some()&&self.dialect.is_some(),
  };
  shape&&!self.contributor.is_empty()&&!self.schema.is_empty()&&self.apps.iter().all(|app|!app.is_empty())&&self.apps.iter().enumerate().all(|(index,app)|!self.apps[..index].contains(app))
   &&self.artifact_kind.as_ref().is_none_or(|kind|!kind.is_empty()&&self.dialect.as_ref().is_none_or(|dialect|dialect.artifact_kind==*kind))
   &&self.dialect.as_ref().is_none_or(|dialect|{let coordinate=dialect.to_coordinate();ArtifactDialect::parse_coordinate(&coordinate).as_ref()==Ok(dialect)&&!coordinate.chars().any(char::is_control)})
   &&self.factory.as_ref().is_none_or(|factory|!factory.is_empty())
   &&self.sqlite_schema.as_ref().is_none_or(|schema|!schema.is_empty())
   &&self.schema==codec.schema&&self.native_identity==codec.native_identity
   &&self.sqlite_schema==codec.snapshot_sqlite.as_ref().map(|provider|provider.schema.clone())
   &&codec.native_identity.validates_provider(&codec.schema,codec.snapshot_sqlite.as_ref())
 }
}

#[derive(Clone,Debug,PartialEq,Eq,PartialOrd,Ord)]
pub(crate) struct BindingKey { channel:ArtifactCodecBindingChannel,contributor:String,artifact_kind:Option<String>,apps:Vec<String>,schema:String,dialect:Option<ArtifactDialect> }
pub(crate) type BindingMap=BTreeMap<BindingKey,ArtifactCodecBinding>;
static BINDINGS:OnceLock<RwLock<BindingMap>>=OnceLock::new();
pub(crate) fn registry()->&'static RwLock<BindingMap>{BINDINGS.get_or_init(||RwLock::new(BTreeMap::new()))}

/// 🚫️ A proposed row is duplicate, inconsistent with its actual codec, or conflicts with installed provenance.
#[derive(Clone,Debug,PartialEq,Eq)]
pub enum ArtifactCodecBindingError { Unavailable, Duplicate{schema:String}, Invalid{schema:String}, Conflict{schema:String} }
impl std::fmt::Display for ArtifactCodecBindingError {
 fn fmt(&self,formatter:&mut std::fmt::Formatter<'_>)->std::fmt::Result {
  match self{Self::Unavailable=>formatter.write_str("codec binding registry unavailable"),Self::Duplicate{schema}=>write!(formatter,"duplicate original codec binding for {schema}"),Self::Invalid{schema}=>write!(formatter,"invalid original codec binding for {schema}"),Self::Conflict{schema}=>write!(formatter,"conflicting original codec binding for {schema}")}
 }
}
impl std::error::Error for ArtifactCodecBindingError {}

pub(crate) fn propose(existing:&BindingMap,rows:&[ArtifactCodecBinding],codecs:&[store::ArtifactCodec])->Result<BindingMap,ArtifactCodecBindingError>{
 let mut proposed=BindingMap::new();
 for row in rows {
  if !codecs.iter().any(|codec|row.validates(codec)){return Err(ArtifactCodecBindingError::Invalid{schema:row.schema.clone()})}
  let key=row.key();
  if proposed.contains_key(&key){return Err(ArtifactCodecBindingError::Duplicate{schema:row.schema.clone()})}
  let mut canonical=row.clone();canonical.apps.sort();
  if existing.get(&key).is_some_and(|existing|existing!=&canonical){return Err(ArtifactCodecBindingError::Conflict{schema:row.schema.clone()})}
  proposed.insert(key,canonical);
 }
 Ok(proposed)
}

/// 🔬️ Checks every original prefilter row without mutating any installed registry.
pub fn preflight_artifact_codec_bindings(rows:&[ArtifactCodecBinding],codecs:&[store::ArtifactCodec])->Result<(),ArtifactCodecBindingError>{
 let _assembly=semio_framework_schema_registry::assembly::begin().map_err(|_|ArtifactCodecBindingError::Unavailable)?;
 preflight_artifact_codec_bindings_in_assembly(&_assembly,rows,codecs)
}

/// 🔐️ Checks original rows while the caller retains the same assembly publication barrier.
pub fn preflight_artifact_codec_bindings_in_assembly(_assembly:&semio_framework_schema_registry::assembly::Transaction,rows:&[ArtifactCodecBinding],codecs:&[store::ArtifactCodec])->Result<(),ArtifactCodecBindingError>{
 let existing=registry().read().map_err(|_|ArtifactCodecBindingError::Unavailable)?;
 propose(&existing,rows,codecs).map(|_|())
}

/// 📚️ Lists every installed original binding, including owners with no SQL capability.
pub fn artifact_codec_bindings()->Result<Vec<ArtifactCodecBinding>,ArtifactCodecBindingError>{
 let _assembly=semio_framework_schema_registry::assembly::begin().map_err(|_|ArtifactCodecBindingError::Unavailable)?;
 let registry=registry().read().map_err(|_|ArtifactCodecBindingError::Unavailable)?;
 Ok(registry.values().cloned().collect())
}
