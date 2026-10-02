//! 🧩️ Complete owned provider inventory contributed by 🔮️oracles/🔣️.json.

/// 📇️ One independently owned oracle package in the complete assembly.
pub struct OracleProvider {
    pub owner: &'static str,
    pub package: &'static str,
    pub library: &'static str,
}

const PROVIDERS: &[OracleProvider] = &[
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖊️drawing", package: "semio-s-plugin-stdio-drawing-test-oracle", library: "semio_s_plugin_stdio_drawing_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las", package: "semio-s-artifact-stdio-las-test-oracle", library: "semio_s_artifact_stdio_las_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html", package: "semio-s-artifact-stdio-html-test-oracle", library: "semio_s_artifact_stdio_html_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw", package: "semio-s-artifact-stdio-epw-test-oracle", library: "semio_s_artifact_stdio_epw_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip", package: "semio-s-artifact-stdio-zip-test-oracle", library: "semio_s_artifact_stdio_zip_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif", package: "semio-s-artifact-stdio-gif-test-oracle", library: "semio_s_artifact_stdio_gif_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4", package: "semio-s-artifact-stdio-mp4-test-oracle", library: "semio_s_artifact_stdio_mp4_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg", package: "semio-s-artifact-stdio-svg-test-oracle", library: "semio_s_artifact_stdio_svg_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3", package: "semio-s-artifact-stdio-mp3-test-oracle", library: "semio_s_artifact_stdio_mp3_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc", package: "semio-s-artifact-stdio-ifc-test-oracle", library: "semio_s_artifact_stdio_ifc_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf", package: "semio-s-artifact-stdio-bcf-test-oracle", library: "semio_s_artifact_stdio_bcf_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary", package: "semio-s-artifact-stdio-binary-test-oracle", library: "semio_s_artifact_stdio_binary_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv", package: "semio-s-artifact-stdio-csv-test-oracle", library: "semio_s_artifact_stdio_csv_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step", package: "semio-s-artifact-stdio-step-test-oracle", library: "semio_s_artifact_stdio_step_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv", package: "semio-s-artifact-stdio-tsv-test-oracle", library: "semio_s_artifact_stdio_tsv_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx", package: "semio-s-artifact-stdio-xlsx-test-oracle", library: "semio_s_artifact_stdio_xlsx_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf", package: "semio-s-artifact-stdio-pdf-test-oracle", library: "semio_s_artifact_stdio_pdf_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx", package: "semio-s-artifact-stdio-docx-test-oracle", library: "semio_s_artifact_stdio_docx_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md", package: "semio-s-artifact-stdio-md-test-oracle", library: "semio_s_artifact_stdio_md_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml", package: "semio-s-artifact-stdio-xml-test-oracle", library: "semio_s_artifact_stdio_xml_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png", package: "semio-s-artifact-stdio-png-test-oracle", library: "semio_s_artifact_stdio_png_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg", package: "semio-s-artifact-stdio-jpg-test-oracle", library: "semio_s_artifact_stdio_jpg_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi", package: "semio-s-artifact-stdio-avi-test-oracle", library: "semio_s_artifact_stdio_avi_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx", package: "semio-s-artifact-stdio-pptx-test-oracle", library: "semio_s_artifact_stdio_pptx_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav", package: "semio-s-artifact-stdio-wav-test-oracle", library: "semio_s_artifact_stdio_wav_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt", package: "semio-s-artifact-stdio-txt-test-oracle", library: "semio_s_artifact_stdio_txt_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl", package: "semio-s-artifact-stdio-stl-test-oracle", library: "semio_s_artifact_stdio_stl_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg", package: "semio-s-artifact-stdio-dwg-test-oracle", library: "semio_s_artifact_stdio_dwg_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf", package: "semio-s-artifact-stdio-dxf-test-oracle", library: "semio_s_artifact_stdio_dxf_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff", package: "semio-s-artifact-stdio-tiff-test-oracle", library: "semio_s_artifact_stdio_tiff_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate", package: "semio-s-artifact-stdio-deflate-test-oracle", library: "semio_s_artifact_stdio_deflate_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj", package: "semio-s-artifact-stdio-obj-test-oracle", library: "semio_s_artifact_stdio_obj_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf", package: "semio-s-artifact-stdio-gltf-test-oracle", library: "semio_s_artifact_stdio_gltf_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply", package: "semio-s-artifact-stdio-ply-test-oracle", library: "semio_s_artifact_stdio_ply_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json", package: "semio-s-artifact-stdio-json-test-oracle", library: "semio_s_artifact_stdio_json_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio", package: "semio-s-artifact-stdio-semio-test-oracle", library: "semio_s_artifact_stdio_semio_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp", package: "semio-s-artifact-stdio-bmp-test-oracle", library: "semio_s_artifact_stdio_bmp_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note", package: "semio-s-artifact-note-note-test-oracle", library: "semio_s_artifact_note_note_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🎒️archive", package: "semio-s-plugin-stdio-archive-test-oracle", library: "semio_s_plugin_stdio_archive_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📃️document", package: "semio-s-plugin-stdio-document-test-oracle", library: "semio_s_plugin_stdio_document_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📊️tabular", package: "semio-s-plugin-stdio-tabular-test-oracle", library: "semio_s_plugin_stdio_tabular_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📰markup", package: "semio-s-plugin-stdio-markup-test-oracle", library: "semio_s_plugin_stdio_markup_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔊️audio", package: "semio-s-plugin-stdio-audio-test-oracle", library: "semio_s_plugin_stdio_audio_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🖼️raster", package: "semio-s-plugin-stdio-raster-test-oracle", library: "semio_s_plugin_stdio_raster_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🧊️mesh", package: "semio-s-plugin-stdio-mesh-test-oracle", library: "semio_s_plugin_stdio_mesh_test_oracle" },
    OracleProvider { owner: "✏️s/🔌️plugins/🗄️stdio/🔮️oracles/🔤️part21", package: "semio-s-plugin-stdio-part21-test-oracle", library: "semio_s_plugin_stdio_part21_test_oracle" },
];

/// 🌳️ Exposes every declared provider and family without introducing an ancestor host dependency.
pub fn providers() -> &'static [OracleProvider] {
    PROVIDERS
}
