//! 📇️ Explicit native application contributions, independent of any installed application owner.

use std::collections::BTreeSet;
use std::path::Path;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct NativeHostV1 { pub landing_app_id:String, pub host_app_id:String }
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct NativePluginV1 { pub plugin_id:String, pub crate_path:String, pub wasm_out:String, pub role:String, pub extends:Option<String>, pub capabilities:Vec<String>, pub activation_events:Vec<String>, pub host:Option<NativeHostV1> }
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all="camelCase",deny_unknown_fields)]
pub struct NativePlaygroundV1 { pub variant:String, pub plugin_id:String, pub app:Option<String>, pub aliases:Vec<String> }
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(deny_unknown_fields)]
pub struct NativeCatalogV1 { pub version:u32, pub plugins:Vec<NativePluginV1>, pub playgrounds:Vec<NativePlaygroundV1> }
impl Default for NativeCatalogV1 { fn default()->Self { Self {version:1,plugins:Vec::new(),playgrounds:Vec::new()} } }
impl NativeCatalogV1 {
    /// 🧭️ Resolves only aliases supplied by the calling application.
    pub fn registry_id<'a>(&'a self,variant:&'a str)->&'a str { self.playgrounds.iter().find(|row|row.variant==variant||row.aliases.iter().any(|alias|alias==variant)).map(|row|row.plugin_id.as_str()).unwrap_or(variant) }
    /// 🪟️ Projects the explicitly contributed app selection.
    pub fn app_id(&self,variant:&str)->Option<&str> { self.playgrounds.iter().find(|row|row.variant==variant||row.aliases.iter().any(|alias|alias==variant)).and_then(|row|row.app.as_deref()) }
    /// 🧩️ Enumerates explicit extension ownership without a compiled roster.
    pub fn extensions_of<'a>(&'a self,parent:&'a str)->impl Iterator<Item=&'a NativePluginV1> { self.plugins.iter().filter(move|row|row.role=="extension"&&row.extends.as_deref()==Some(parent)) }
}

/// 🔐️ Admits the closed schema and ownership references before exposing any contributed entry.
pub fn read_native_catalog_v1(text:&str)->Result<NativeCatalogV1,String> {
    if text.len()>8_388_608 {return Err("Native catalog exceeds its byte authority".into());}
    let raw:serde_json::Value=serde_json::from_str(text).map_err(|error|format!("Native catalog JSON refused: {error}"))?;
    let object=raw.as_object().ok_or("Native catalog must be an object")?;
    for name in ["plugins","playgrounds"] {let rows=object.get(name).and_then(|rows|rows.as_array()).ok_or("Native catalog rows required")?;if rows.len()>4096{return Err("Native catalog row authority exceeded".into());}for row in rows {let row=row.as_object().ok_or("Native catalog row must be an object")?;for key in if name=="plugins" {&["extends","host"][..]} else {&["app"][..]} {if !row.contains_key(*key){return Err(format!("Explicit native catalog member {key} required"));}}}}
    let catalog:NativeCatalogV1=serde_json::from_value(raw).map_err(|error|format!("Native catalog schema refused: {error}"))?;
    if catalog.version!=1{return Err("Native catalog version refused".into());}
    let mut ids=BTreeSet::new();let mut claims=BTreeSet::new();let valid=|text:&str|!text.is_empty()&&!text.contains('\0');
    for row in &catalog.plugins {if !valid(&row.plugin_id)||!valid(&row.crate_path)||!valid(&row.wasm_out)||!matches!(row.role.as_str(),"plugin"|"extension")||!ids.insert(row.plugin_id.as_str()) {return Err("Native plugin identity refused".into());}if Path::new(&row.crate_path).is_absolute()||Path::new(&row.wasm_out).is_absolute()||row.crate_path.split(['/', '\\']).chain(row.wasm_out.split(['/', '\\'])).any(|part|part=="..") {return Err("Native plugin artifact path escapes its owner".into());}for values in [&row.capabilities,&row.activation_events] {if values.len()>4096||values.iter().any(|value|!valid(value)){return Err("Native plugin member authority refused".into());}}if let Some(host)=&row.host {if !valid(&host.landing_app_id)||!valid(&host.host_app_id){return Err("Native host apps required".into());}}for event in &row.activation_events {if let Some(kind)=event.strip_prefix("on-artifact-kind:"){if !valid(kind)||!claims.insert(kind){return Err("Native artifact activation ownership refused".into());}}}}
    for row in &catalog.plugins {if row.extends.as_ref().is_some_and(|parent|!ids.contains(parent.as_str()))||row.role=="extension"&&row.extends.is_none(){return Err("Native extension parent refused".into());}}
    let mut variants=BTreeSet::new();for row in &catalog.playgrounds {if !ids.contains(row.plugin_id.as_str())||row.aliases.len()>4096||row.app.as_ref().is_some_and(|app|!valid(app)){return Err("Native playground owner refused".into());}for variant in std::iter::once(&row.variant).chain(row.aliases.iter()) {if !valid(variant)||!variants.insert(variant){return Err("Native playground alias refused".into());}}}
    Ok(catalog)
}

/// 📥️ No supplied publication means an empty roster; a supplied publication is mandatory and fails closed.
pub fn load_native_catalog_v1(publication:Option<&Path>)->Result<NativeCatalogV1,String> {
    let Some(root)=publication else {return Ok(NativeCatalogV1::default());};
    if !root.is_absolute() {return Err("Native catalog publication path must be absolute".into());}
    let path=root.join("🦀️catalog.json");let metadata=std::fs::symlink_metadata(&path).map_err(|error|format!("Configured native catalog {} unavailable: {error}",path.display()))?;
    if !metadata.is_file()||metadata.file_type().is_symlink()||metadata.len()>8_388_608 {return Err("Configured native catalog file authority refused".into());}
    let text=std::fs::read_to_string(&path).map_err(|error|format!("Configured native catalog could not be read: {error}"))?;read_native_catalog_v1(&text)
}
/// 🌿️ Receives the caller-authored publication location with no repository inference.
pub fn configured_native_catalog_v1()->Result<NativeCatalogV1,String>{if let Some(catalog)=INSTALLED_NATIVE_CATALOG.get(){return Ok(catalog.clone());}match std::env::var("SEMIO_PLUGIN_CATALOG") {Ok(value) if value.is_empty()=>Err("Configured native catalog path is empty".into()),Ok(value)=>load_native_catalog_v1(Some(Path::new(&value))),Err(std::env::VarError::NotPresent)=>load_native_catalog_v1(None),Err(_)=>Err("Configured native catalog path is not Unicode".into())}}
/// 🎚️ Resolves only the two authored component modes.
pub fn plugin_component_directory(mode:&str)->Result<&'static str,&'static str>{match mode {"prototype"=>Ok("dist/components/prototype"),"release"=>Ok("dist/components/release"),_=>Err("Native component mode must be prototype or release")}}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;

static INSTALLED_NATIVE_CATALOG:std::sync::OnceLock<NativeCatalogV1>=std::sync::OnceLock::new();
/// 📇️ Receives one admitted application contribution for native service startup.
pub fn install_native_catalog_v1(catalog:NativeCatalogV1)->Result<(),String>{let text=serde_json::to_string(&catalog).map_err(|error|error.to_string())?;let catalog=read_native_catalog_v1(&text)?;INSTALLED_NATIVE_CATALOG.set(catalog).map_err(|_|"Native application catalog already installed".into())}
