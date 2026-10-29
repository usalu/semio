//! 🌱️ Native surface declarations commit real shared definition and registration producers before mounted admission.
use super::*;
use semio_framework_plugin_decl_surface::{ArtifactEditor,ArtifactViewer,OriginalArtifactEditorBirth,OriginalArtifactViewerBirth,EditorApp,ViewerApp};
use semio_framework_plugin_instance_construction::{OriginalVcsApp,factory::original_instance_factory};
use semio_framework_plugin_instance_initialization::services::OriginalInstanceRegistration;

impl<PA:PluginApp> PluginBuilder<Ready,PA>{
 /// ✏️ Declares only an original retained editor factory and its authentic immutable registration source.
 pub fn native_editor<E,R>(mut self,mut definition:semio_framework::AppDefinition)->Result<Self,PluginAssemblyError>
 where E:OriginalArtifactEditorBirth,R:OriginalInstanceRegistration<EditorApp<E>,E::Members>,PA:From<OriginalVcsApp<EditorApp<E>,E::Members>>{
  if definition.io.artifact_schema.is_empty(){definition.io.artifact_schema=E::DOCUMENT_SCHEMA.into();}
  if let Some(codec)=E::natural_file_codec(){semio_framework_plugin_decl_app::mount_natural_file_actions(&mut definition,codec);}
  semio_framework_plugin_decl_registry::declarations::stamp_fault_notices::<EditorApp<E>>(&mut definition);
  self.install_native_surface::<EditorApp<E>,E::Members,R>(definition,E::examples())?;
  use semio_framework::kernel::{ArtifactKind,Rights,Scope};
  Ok(self.capability(CapabilityRequirement{artifact:ArtifactKind::Document,rights:Rights::Read,scope:Scope::App}).capability(CapabilityRequirement{artifact:ArtifactKind::Document,rights:Rights::Write,scope:Scope::App}))
 }
 /// 👁️ Declares a read-only original retained viewer factory without constructing an instance at declaration time.
 pub fn native_viewer<V,R>(mut self,mut definition:semio_framework::AppDefinition)->Result<Self,PluginAssemblyError>
 where V:OriginalArtifactViewerBirth,R:OriginalInstanceRegistration<ViewerApp<V>,V::Members>,PA:From<OriginalVcsApp<ViewerApp<V>,V::Members>>{
  if definition.io.artifact_schema.is_empty(){definition.io.artifact_schema=V::DOCUMENT_SCHEMA.into();}
  semio_framework_plugin_decl_registry::declarations::stamp_fault_notices::<ViewerApp<V>>(&mut definition);
  self.install_native_surface::<ViewerApp<V>,V::Members,R>(definition,Vec::new())?;
  use semio_framework::kernel::{ArtifactKind,Rights,Scope};
  Ok(self.capability(CapabilityRequirement{artifact:ArtifactKind::Document,rights:Rights::Read,scope:Scope::App}))
 }
 fn install_native_surface<A,M,R>(&mut self,mut definition:semio_framework::AppDefinition,examples:Vec<semio_framework_plugin_app_builder::ExampleSource>)->Result<(),PluginAssemblyError>
 where A:semio_framework_plugin_decl_app::OriginalArtifactAppBirth,M:semio_framework_os_kernel::ReferenceSpaceMember+semio_framework_os_kernel::MemberFactory+Send+'static,R:OriginalInstanceRegistration<A,M>,PA:From<OriginalVcsApp<A,M>>{
  semio_framework_plugin_job::reserved_jobs::join_framework_shared_action_dispositions(&mut definition);
  let app=App{definition:definition.clone(),examples};
  let definition=semio_framework_plugin_window_kits::RegisteredAppDefinition::declared(definition,true);
  let original_factory=original_instance_factory::<A,M,PA,R>();
  let registration=(original_factory.registration)(definition.original_source().expect("native declaration retains its original shared producer")).map_err(PluginAssemblyError::from_fault)?;
  self.app_defs.push((app,semio_framework_plugin_decl_registry::plugin_program::AppFactory{definition,registration:Some(registration),original_factory:Some(original_factory),create:None,document_schema:A::DOCUMENT_SCHEMA,codec:semio_framework_plugin_decl_surface::artifact_codec_table::<A>()}));
  fn app_schema<A:semio_framework_plugin_decl_app::ArtifactApp>()->Option<::semio_framework_schema_registry::AppSchemaDescriptor>{::semio_framework_async::poll::resolve_ready(A::app_schema())}
  self.app_schema_descriptors.push(app_schema::<A>);
  Ok(())
 }
}
