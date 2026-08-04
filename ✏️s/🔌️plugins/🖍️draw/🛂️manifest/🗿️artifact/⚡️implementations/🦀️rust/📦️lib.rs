//! ✏️ Draw plugin — declarative draw app bundled as a hot-swappable WASM plugin.

fn register_draw_exports() {
    // 🗂️ Registers `DrawDocument`'s pack<->dsl codec under its real `document_schema()` string so
    // `framework/sync`'s `FolderEndpoint::Pack` (and any other schema-keyed caller) can print/parse
    // draw documents without depending on this crate's concrete `Projection`/`Operation` types.
    semio_framework_plugin::plugin_runtime::register_document_codec_for_app::<draw_ui::DrawPlayApp>(draw::DRAW_DOCUMENT_SCHEMA);
    semio_framework_os::register_2d_export_handlers("2d.drawing", "draw", draw_engine::draw_document_json_to_svg);
    semio_framework_os::register_os_media_export_handler("2d.drawing", semio_framework_os::OsMediaFormat::Dwg, |doc| {
        let bytes = draw_engine::draw_document_json_to_dwg_bytes(doc)?;
        Ok(semio_framework_os::OsMediaExportResult {
            data: {
                use base64::Engine;
                base64::engine::general_purpose::STANDARD.encode(bytes)
            },
            mime_type: semio_framework_os::OsMediaFormat::Dwg.mime_type().into(),
            file_name: "draw.dwg".into(),
            encoding: Some("base64".into()),
        })
    });
    semio_framework_os::register_dwg_import_handler("2d.drawing", draw_engine::draw_document_json_from_dwg);
}

semio_framework_plugin::semio_plugin! {
    id: "draw",
    label: "Draw",
    version: "0.1.0",
    setup: register_draw_exports,
    apps: [ draw_ui::create_draw_app => draw_ui::DrawPlayApp ],
}
