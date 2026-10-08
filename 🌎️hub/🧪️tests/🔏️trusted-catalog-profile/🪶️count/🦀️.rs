//! 🪶️ Held Count profile uses the actual component descriptor and the production trusted loader.
use std::{path::{Path,PathBuf},sync::Arc};
use super::{TrustedCatalogLoader,VerifiedTrustedCatalog,VerifiedDocumentOpenSelectionV1};
use crate::artifact_authority::{AuthorityError,AuthorityLimits,AuthorityOperationControl,AuthorityProgress,OperationContext};
use crate::artifact_authority::native_openable_provider::NativeCodecProviderSetV1;
use semio_framework_plugin_host::{GuestRuntime,OwnedRuntime,CompiledHandle,PackageRef,PackageId,PackageHash,Budget};
use semio_framework_hash::{Hasher,Sha256};
use directory::{os_directory::io::binary::artifact_hash::hex_lower,os_store};
const KIND:&str="fixture.neutral-host-fixture.counter";
const PROFILE:&str="count-real-component-lease";
const ACTOR:&[u8]=b"test-owned-never-executed-count-browser-actor";
struct Control;
impl AuthorityOperationControl for Control{fn now_ms(&self)->u64{1000}fn is_cancelled(&self)->bool{false}fn report(&self,_:AuthorityProgress){}}
pub struct VerifiedCountProfile{root:PathBuf,catalog:Arc<VerifiedTrustedCatalog>,selection:VerifiedDocumentOpenSelectionV1,pub pack:Vec<u8>,pub spr:Vec<u8>,pub codec_runtime:Arc<OwnedRuntime>,pub compiled:CompiledHandle}
impl VerifiedCountProfile{pub fn catalog(&self)->&Arc<VerifiedTrustedCatalog>{&self.catalog}pub fn selection(&self)->&VerifiedDocumentOpenSelectionV1{&self.selection}}
impl Drop for VerifiedCountProfile{fn drop(&mut self){let _=std::fs::remove_dir_all(&self.root);}}
pub async fn verified_count_profile(root:&Path,component:&[u8],document_id:&str)->Result<VerifiedCountProfile,AuthorityError>{
 let fail=|error:String|AuthorityError::Catalog(error);
 let runtime=Arc::new(OwnedRuntime::new());let package=PackageRef{package:PackageId("semio:neutral-host-fixture".into()),hash:PackageHash(*semio_framework_hash::hash(component).as_bytes())};
 let compiled=runtime.compile(&package,component).await.map_err(|e|fail(format!("Count component compile: {e}")))?;
 let budget=||Budget{fuel:8_000_000_000,deadline_ms:120000,max_effects:0,max_patch_bytes:0,max_frames:0};
 let emitted=runtime.describe_observed(&compiled,budget(),|_,_|{}).await.map_err(|e|fail(format!("actual Count descriptor: {e}")))?;
 let mut descriptor=super::decode_package_descriptor(&emitted)?;
 if descriptor.manifest.plugin_id!="neutral-host-fixture"||descriptor.package_id!="semio:neutral-host-fixture"||!descriptor.manifest.dependencies.is_empty(){return Err(fail("Count test profile requires its exact declared dependency-free package".into()))}
 let component_sha=hex_lower(&Sha256::digest(component));let component_blake=hex_lower(Hasher::new().update(component).finalize().as_bytes());
 descriptor.hashes.wasm_sha256=component_sha.clone();descriptor.hashes.core_wasm_sha256=component_sha.clone();descriptor.hashes.descriptor_sha256.clear();
 descriptor.hashes.descriptor_sha256=hex_lower(&Sha256::digest(&os_store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&descriptor))));
 let descriptor_bytes=os_store::pack_rt::encode_wire_value(&semio_framework_value::ToValue::to_value(&descriptor));
 let pack_hash=runtime.codec_pack_schema_hash(&compiled,KIND,budget()).await.map_err(|e|fail(format!("actual Count pack identity: {e}")))?;
 if pack_hash==[0;32]{return Err(fail("Count guest structural identity is absent".into()))}
 let pair=runtime.codec_genesis(&compiled,KIND,document_id,budget()).await.map_err(|e|fail(format!("actual Count genesis: {e}")))?;
 let editor=descriptor.manifest.apps.iter().find(|app|app.role==semio_framework::AppRole::Editor&&app.dialect.artifact_kind==KIND).ok_or_else(||fail("actual Count editor absent".into()))?;
 let package=serde_json::json!({"pluginId":descriptor.manifest.plugin_id,"packageId":descriptor.package_id,"version":descriptor.manifest.version});
 let target=serde_json::json!({"artifactKind":KIND,"artifactSchema":KIND,"packSchemaHash":hex_lower(&pack_hash),"surfaceId":editor.id,"appId":editor.id,"windowKindId":editor.window_kinds.first().id,"role":"editor","rendererTarget":"wasm","parentDialect":{"artifactKind":KIND,"standard":"1","subset":"*"},"grant":{"read":true,"write":true,"observe":true}});
 std::fs::create_dir_all(root).map_err(|e|fail(e.to_string()))?;
 for(name,bytes)in[("component.wasm",component),("descriptor.semio",descriptor_bytes.as_slice()),("closed-actor.mjs",ACTOR)]{std::fs::write(root.join(name),bytes).map_err(|e|fail(e.to_string()))?}
 let plugin_module=super::write_fixture_plugin_module(root,&descriptor.manifest.plugin_id,&descriptor.package_id,&descriptor.manifest.version,&component_sha,&descriptor_bytes)?;
 let mut bundle=serde_json::json!({"schemaVersion":3,"profiles":[{"id":PROFILE,"selectedClosure":[package.clone()],"selectedClosureSha256":"01".repeat(32),"openTargets":[{"package":package.clone(),"target":target.clone()}],"generationId":"02".repeat(32)}],"packages":[{"pluginId":package["pluginId"],"packageId":package["packageId"],"version":package["version"],"role":"plugin","dependencies":[],"executionProtocol":{"appChannelVersion":descriptor.execution_protocol.app_channel_version},"component":{"path":"component.wasm","byteLength":component.len(),"sha256":component_sha,"blake3":component_blake},"descriptor":{"path":"descriptor.semio","byteLength":descriptor_bytes.len(),"sha256":hex_lower(&Sha256::digest(&descriptor_bytes))},"browserActor":{"kind":"closed-browser-actor","schema":"semio.os.closed-browser-actor.v1","codegenPolicy":"semio.os.browser-jco-1.34.0-jspi.v1","path":"closed-actor.mjs","byteLength":ACTOR.len(),"sha256":hex_lower(&Sha256::digest(ACTOR)),"sourceComponentSha256":component_sha,"sourceDescriptorByteSha256":hex_lower(&Sha256::digest(&descriptor_bytes)),"policySha256":"41".repeat(32),"importInterfaces":[]},"pluginModule":plugin_module,"nativeCodecs":[{"artifactKind":KIND,"artifactSchema":KIND,"packSchemaHash":hex_lower(&pack_hash)}],"openTargets":[target]}]});
 let decoded:super::TrustedBundleV1=serde_json::from_value(bundle.clone()).map_err(|e|fail(e.to_string()))?;bundle["profiles"][0]["selectedClosureSha256"]=hex_lower(&super::selected_closure_digest(&decoded.profiles[0].selected_closure)?).into();
 let decoded:super::TrustedBundleV1=serde_json::from_value(bundle.clone()).map_err(|e|fail(e.to_string()))?;bundle["profiles"][0]["generationId"]=super::trusted_profile_generation(&decoded,&decoded.profiles[0])?.into();
 let path=root.join("trusted-catalog.json");std::fs::write(&path,serde_json::to_vec_pretty(&bundle).map_err(|e|fail(e.to_string()))?).map_err(|e|fail(e.to_string()))?;
 let context=OperationContext::new(u64::MAX,AuthorityLimits::maximum(),&Control);let catalog=Arc::new(TrustedCatalogLoader::load_fixture(&path,PROFILE,&NativeCodecProviderSetV1::linked(),&context).await?);
 let selection=catalog.artifact_creation_selection(KIND).cloned().ok_or_else(||fail("verified Count owner absent".into()))?;
 Ok(VerifiedCountProfile{root:root.into(),catalog,selection,pack:pair.pack,spr:pair.spr,codec_runtime:runtime,compiled})
}

