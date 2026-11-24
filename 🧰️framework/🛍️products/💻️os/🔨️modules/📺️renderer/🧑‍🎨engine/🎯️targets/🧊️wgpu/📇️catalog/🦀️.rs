//! 📇️ Native host and activation views derived only from the caller-supplied catalog.
use semio_os_kernel::native_catalog::{NativeCatalogV1,read_native_catalog_v1,configured_native_catalog_v1};
use std::cell::Cell;
use std::sync::OnceLock;
static CATALOG:OnceLock<NativeCatalogV1>=OnceLock::new();

/// 📥️ Installs one immutable application publication before renderer startup.
pub fn install_plugin_catalog_v1(catalog:NativeCatalogV1)->Result<(),String>{let text=serde_json::to_string(&catalog).map_err(|error|error.to_string())?;let catalog=read_native_catalog_v1(&text)?;CATALOG.set(catalog).map_err(|_|"Renderer catalog was already installed".into())}
/// 🧾️ Admits the authored native document before installing host contributions.
pub fn install_plugin_catalog_json_v1(text:&str)->Result<(),String>{install_plugin_catalog_v1(read_native_catalog_v1(text)?)}
/// 🌿️ Receives explicit native publication; unconfigured startup owns an empty roster.
pub fn install_configured_plugin_catalog_v1()->Result<(),String>{install_plugin_catalog_v1(configured_native_catalog_v1()?)}
/// 📇️ Borrows the immutable caller catalog, with an empty unconfigured host contract.
pub fn plugin_catalog_v1()->&'static NativeCatalogV1 {match SCOPED.with(Cell::get){Some(scoped)=>scoped.catalog,None=>CATALOG.get_or_init(NativeCatalogV1::default)}}

struct ScopedCatalog{catalog:&'static NativeCatalogV1,activations:Vec<(&'static str,&'static str)>}
thread_local!{static SCOPED:Cell<Option<&'static ScopedCatalog>>=const{Cell::new(None)}}
/// 🧵️ Restores the thread's previous catalog when dropped.
pub struct PluginCatalogScope(Option<&'static ScopedCatalog>);
impl Drop for PluginCatalogScope{fn drop(&mut self){SCOPED.with(|scoped|scoped.set(self.0))}}
/// 🧵️ Binds `catalog` to the calling thread only, so concurrent shells of one process each read their own roster; the process-wide publication stays untouched.
pub fn scope_plugin_catalog_v1(catalog:NativeCatalogV1)->PluginCatalogScope{let catalog:&'static NativeCatalogV1=Box::leak(Box::new(catalog));let scoped:&'static ScopedCatalog=Box::leak(Box::new(ScopedCatalog{catalog,activations:activations_of(catalog)}));PluginCatalogScope(SCOPED.with(|cell|cell.replace(Some(scoped))))}
fn activations_of(catalog:&'static NativeCatalogV1)->Vec<(&'static str,&'static str)>{catalog.plugins.iter().flat_map(|row|row.activation_events.iter().filter_map(move|event|event.strip_prefix("on-artifact-kind:").map(|kind|(kind,row.plugin_id.as_str())))).collect()}

pub struct PluginHostConfig {pub plugin_id:&'static str,pub landing_app_id:&'static str,pub host_app_id:&'static str}
/// 🧭️ Resolves the supplied variant and alias inventory.
pub fn resolve_registry_plugin_id(plugin_filter:&str)->&str{plugin_catalog_v1().registry_id(plugin_filter)}
/// 🪟️ Resolves the supplied native app selection.
pub fn resolve_playground_app_id(plugin_filter:&str)->Option<&'static str>{plugin_catalog_v1().app_id(plugin_filter)}
/// 🏠️ Projects each native host from its contributed owner.
pub fn resolve_plugin_host_config(plugin_filter:&str)->Option<&'static PluginHostConfig>{static HOSTS:OnceLock<Vec<PluginHostConfig>>=OnceLock::new();let hosts=HOSTS.get_or_init(||plugin_catalog_v1().plugins.iter().filter_map(|row|row.host.as_ref().map(|host|PluginHostConfig{plugin_id:&row.plugin_id,landing_app_id:&host.landing_app_id,host_app_id:&host.host_app_id})).collect());let registry=resolve_registry_plugin_id(plugin_filter);hosts.iter().find(|host|host.plugin_id==registry)}
/// 🪟️ Recognizes only the explicit host contribution.
pub fn is_space_mode(plugin_filter:&str)->bool{resolve_plugin_host_config(plugin_filter).is_some()}
/// 🎬️ Returns validated artifact activation ownership without a generated table.
pub fn plugin_artifact_kind_activations()->&'static [(&'static str,&'static str)] {static ACTIVATIONS:OnceLock<Vec<(&str,&str)>>=OnceLock::new();match SCOPED.with(Cell::get){Some(scoped)=>scoped.activations.as_slice(),None=>ACTIVATIONS.get_or_init(||activations_of(plugin_catalog_v1()))}}
