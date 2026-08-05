//! 🎛️ S Studio plugin — designer OS shell bundled as a hot-swappable WASM plugin.
//!
//! 🪶️ Thin bundle after the constitutional split: the Home launcher (`home_ui::HomeApp`) and Studio
//! (`space_ui::SpaceApp`) apps, their document entities, headless compute, and manifests all live in
//! `s/plugin/space/app/{home,space}/{rs,engine,dsl,op,pack,protocol,ui}` — see
//! `s/plugin/space/shared/rs` for the fixture/document helpers both apps share. This file keeps only
//! the pack↔dsl codec registration and the manual `PluginBundle` builder (not the `semio_plugin!`
//! macro — `s` is the OS host plugin, not a typical document-app plugin).

//#region 🔖️DocumentCodecs
/// 🗂️ Registers `s.home`/`s.space`'s pack<->dsl codecs under their real `document_schema()` strings
/// so `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse
/// these documents without depending on this crate's concrete `Projection`/`Operation` types.
fn register_s_exports() {
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<home_ui::HomeApp>("s.home");
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<space_ui::SpaceApp>(semio_framework_os::OS_SPACE_SCHEMA);
}
//#endregion 🔖️DocumentCodecs

//#region 🔖️Manifest
fn bundle() -> semio_framework_plugin::PluginBundle {
    register_s_exports();
    semio_framework_plugin::PluginBundle::new("s", "S Studio", "0.1.0").local_backbone_storage().register_document_app(home_ui::create_home_app(), || home_ui::HomeApp).register_document_app(space_ui::create_space_app(), || space_ui::SpaceApp)
}
semio_framework_plugin::plugin_exports!(bundle);
//#endregion 🔖️Manifest
