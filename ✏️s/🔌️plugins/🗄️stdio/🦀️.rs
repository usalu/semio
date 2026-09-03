//! 🔌️ Schema-owned stdio library plugin assembly.

use semio_framework_dispatch_macros::dyn_enum_close;
use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

// 🗃️ Closed runtime app fleet for every stdio editor and viewer surface.
dyn_enum_close! {
    pub enum StdioApps: PluginApp {
        PngEditor(VcsArtifactApp<EditorApp<crate::editor::png::PngEditor>>),
        PngViewer(VcsArtifactApp<ViewerApp<crate::viewer::png::PngViewer>>),
        JpgAnyEditor(VcsArtifactApp<EditorApp<crate::editor::jpg_any::JpgAnyEditor>>),
        JpgAnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::jpg_any::JpgAnyViewer>>),
        JpgBaselineEditor(VcsArtifactApp<EditorApp<crate::editor::jpg_baseline::JpgBaselineEditor>>),
        JpgBaselineViewer(VcsArtifactApp<ViewerApp<crate::viewer::jpg_baseline::JpgBaselineViewer>>),
        BmpEditor(VcsArtifactApp<EditorApp<crate::editor::bmp::BmpEditor>>),
        BmpViewer(VcsArtifactApp<ViewerApp<crate::viewer::bmp::BmpViewer>>),
        TiffAnyEditor(VcsArtifactApp<EditorApp<crate::editor::tiff_any::TiffAnyEditor>>),
        TiffAnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::tiff_any::TiffAnyViewer>>),
        TiffBaselineEditor(VcsArtifactApp<EditorApp<crate::editor::tiff_baseline::TiffBaselineEditor>>),
        TiffBaselineViewer(VcsArtifactApp<ViewerApp<crate::viewer::tiff_baseline::TiffBaselineViewer>>),
        Gif87aEditor(VcsArtifactApp<EditorApp<crate::editor::gif_87a::Gif87aEditor>>),
        Gif87aViewer(VcsArtifactApp<ViewerApp<crate::viewer::gif_87a::Gif87aViewer>>),
        Gif89aEditor(VcsArtifactApp<EditorApp<crate::editor::gif_89a::Gif89aEditor>>),
        Gif89aViewer(VcsArtifactApp<ViewerApp<crate::viewer::gif_89a::Gif89aViewer>>),
        SvgAnyEditor(VcsArtifactApp<EditorApp<crate::editor::svg_any::SvgAnyEditor>>),
        SvgAnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::svg_any::SvgAnyViewer>>),
        SvgBasicEditor(VcsArtifactApp<EditorApp<crate::editor::svg_basic::SvgBasicEditor>>),
        SvgBasicViewer(VcsArtifactApp<ViewerApp<crate::viewer::svg_basic::SvgBasicViewer>>),
        SvgTinyEditor(VcsArtifactApp<EditorApp<crate::editor::svg_tiny::SvgTinyEditor>>),
        SvgTinyViewer(VcsArtifactApp<ViewerApp<crate::viewer::svg_tiny::SvgTinyViewer>>),
        Mp4Editor(VcsArtifactApp<EditorApp<crate::editor::mp4::Mp4Editor>>),
        Mp4Viewer(VcsArtifactApp<ViewerApp<crate::viewer::mp4::Mp4Viewer>>),
        Mp3Editor(VcsArtifactApp<EditorApp<crate::editor::mp3::Mp3Editor>>),
        Mp3Viewer(VcsArtifactApp<ViewerApp<crate::viewer::mp3::Mp3Viewer>>),
        WavEditor(VcsArtifactApp<EditorApp<crate::editor::wav::WavEditor>>),
        WavViewer(VcsArtifactApp<ViewerApp<crate::viewer::wav::WavViewer>>),
        AviEditor(VcsArtifactApp<EditorApp<crate::editor::avi::AviEditor>>),
        AviViewer(VcsArtifactApp<ViewerApp<crate::viewer::avi::AviViewer>>),
        HtmlEditor(VcsArtifactApp<EditorApp<crate::editor::html::HtmlEditor>>),
        HtmlViewer(VcsArtifactApp<ViewerApp<crate::viewer::html::HtmlViewer>>),
        MdEditor(VcsArtifactApp<EditorApp<crate::editor::md::MdEditor>>),
        MdViewer(VcsArtifactApp<ViewerApp<crate::viewer::md::MdViewer>>),
        SemioBrepEditor(VcsArtifactApp<EditorApp<crate::editor::semio_brep::SemioBrepEditor>>),
        SemioBrepViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_brep::SemioBrepViewer>>),
        SemioDrawingEditor(VcsArtifactApp<EditorApp<crate::editor::semio_drawing::SemioDrawingEditor>>),
        SemioDrawingViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_drawing::SemioDrawingViewer>>),
        SemioGraphEditor(VcsArtifactApp<EditorApp<crate::editor::semio_graph::SemioGraphEditor>>),
        SemioGraphViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_graph::SemioGraphViewer>>),
        SemioKitEditor(VcsArtifactApp<EditorApp<crate::editor::semio_kit::SemioKitEditor>>),
        SemioKitViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_kit::SemioKitViewer>>),
        SemioMeshEditor(VcsArtifactApp<EditorApp<crate::editor::semio_mesh::SemioMeshEditor>>),
        SemioMeshViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_mesh::SemioMeshViewer>>),
        SemioObjectEditor(VcsArtifactApp<EditorApp<crate::editor::semio_object::SemioObjectEditor>>),
        SemioObjectViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_object::SemioObjectViewer>>),
        SemioTableEditor(VcsArtifactApp<EditorApp<crate::editor::semio_table::SemioTableEditor>>),
        SemioTableViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_table::SemioTableViewer>>),
        SemioTextEditor(VcsArtifactApp<EditorApp<crate::editor::semio_text::SemioTextEditor>>),
        SemioTextViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_text::SemioTextViewer>>),
        SemioAnimationEditor(VcsArtifactApp<EditorApp<crate::editor::semio_animation::SemioAnimationEditor>>),
        SemioAnimationViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_animation::SemioAnimationViewer>>),
        SemioAnyEditor(VcsArtifactApp<EditorApp<crate::editor::semio_base::SemioAnyEditor>>),
        SemioAnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_base::SemioAnyViewer>>),
        SemioAudioEditor(VcsArtifactApp<EditorApp<crate::editor::semio_audio::SemioAudioEditor>>),
        SemioAudioViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_audio::SemioAudioViewer>>),
        SemioCadEditor(VcsArtifactApp<EditorApp<crate::editor::semio_cad::SemioCadEditor>>),
        SemioCadViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_cad::SemioCadViewer>>),
        SemioDocumentEditor(VcsArtifactApp<EditorApp<crate::editor::semio_document::SemioDocumentEditor>>),
        SemioDocumentViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_document::SemioDocumentViewer>>),
        SemioFlowEditor(VcsArtifactApp<EditorApp<crate::editor::semio_flow::SemioFlowEditor>>),
        SemioFlowViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_flow::SemioFlowViewer>>),
        SemioImageEditor(VcsArtifactApp<EditorApp<crate::editor::semio_image::SemioImageEditor>>),
        SemioImageViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_image::SemioImageViewer>>),
        SemioModelEditor(VcsArtifactApp<EditorApp<crate::editor::semio_model::SemioModelEditor>>),
        SemioModelViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_model::SemioModelViewer>>),
        SemioPresentationEditor(VcsArtifactApp<EditorApp<crate::editor::semio_presentation::SemioPresentationEditor>>),
        SemioPresentationViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_presentation::SemioPresentationViewer>>),
        SemioValueEditor(VcsArtifactApp<EditorApp<crate::editor::semio_value::SemioValueEditor>>),
        SemioValueViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_value::SemioValueViewer>>),
        SemioVideoEditor(VcsArtifactApp<EditorApp<crate::editor::semio_video::SemioVideoEditor>>),
        SemioVideoViewer(VcsArtifactApp<ViewerApp<crate::viewer::semio_video::SemioVideoViewer>>),
        StepAnyEditor(VcsArtifactApp<EditorApp<crate::editor::step_any::StepAnyEditor>>),
        StepAnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::step_any::StepAnyViewer>>),
        StepCc1Editor(VcsArtifactApp<EditorApp<crate::editor::step_cc1::StepCc1Editor>>),
        StepCc1Viewer(VcsArtifactApp<ViewerApp<crate::viewer::step_cc1::StepCc1Viewer>>),
        StepCc2Editor(VcsArtifactApp<EditorApp<crate::editor::step_cc2::StepCc2Editor>>),
        StepCc2Viewer(VcsArtifactApp<ViewerApp<crate::viewer::step_cc2::StepCc2Viewer>>),
        StepCc3Editor(VcsArtifactApp<EditorApp<crate::editor::step_cc3::StepCc3Editor>>),
        StepCc3Viewer(VcsArtifactApp<ViewerApp<crate::viewer::step_cc3::StepCc3Viewer>>),
        StepCc4Editor(VcsArtifactApp<EditorApp<crate::editor::step_cc4::StepCc4Editor>>),
        StepCc4Viewer(VcsArtifactApp<ViewerApp<crate::viewer::step_cc4::StepCc4Viewer>>),
        StepCc5Editor(VcsArtifactApp<EditorApp<crate::editor::step_cc5::StepCc5Editor>>),
        StepCc5Viewer(VcsArtifactApp<ViewerApp<crate::viewer::step_cc5::StepCc5Viewer>>),
        StepCc6Editor(VcsArtifactApp<EditorApp<crate::editor::step_cc6::StepCc6Editor>>),
        StepCc6Viewer(VcsArtifactApp<ViewerApp<crate::viewer::step_cc6::StepCc6Viewer>>),
        Ifc2x3AnyEditor(VcsArtifactApp<EditorApp<crate::editor::ifc2x3_any::Ifc2x3AnyEditor>>),
        Ifc2x3AnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::ifc2x3_any::Ifc2x3AnyViewer>>),
        Ifc2x3CobieEditor(VcsArtifactApp<EditorApp<crate::editor::ifc2x3_cobie::Ifc2x3CobieEditor>>),
        Ifc2x3CobieViewer(VcsArtifactApp<ViewerApp<crate::viewer::ifc2x3_cobie::Ifc2x3CobieViewer>>),
        Ifc2x3Cv20Editor(VcsArtifactApp<EditorApp<crate::editor::ifc2x3_cv20::Ifc2x3Cv20Editor>>),
        Ifc2x3Cv20Viewer(VcsArtifactApp<ViewerApp<crate::viewer::ifc2x3_cv20::Ifc2x3Cv20Viewer>>),
        Ifc2x3SavEditor(VcsArtifactApp<EditorApp<crate::editor::ifc2x3_sav::Ifc2x3SavEditor>>),
        Ifc2x3SavViewer(VcsArtifactApp<ViewerApp<crate::viewer::ifc2x3_sav::Ifc2x3SavViewer>>),
        Ifc4AnyEditor(VcsArtifactApp<EditorApp<crate::editor::ifc4_any::Ifc4AnyEditor>>),
        Ifc4AnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::ifc4_any::Ifc4AnyViewer>>),
        DwgAc1018Editor(VcsArtifactApp<EditorApp<crate::editor::dwg_ac1018::DwgAc1018Editor>>),
        DwgAc1018Viewer(VcsArtifactApp<ViewerApp<crate::viewer::dwg_ac1018::DwgAc1018Viewer>>),
        DwgAc1024Editor(VcsArtifactApp<EditorApp<crate::editor::dwg_ac1024::DwgAc1024Editor>>),
        DwgAc1024Viewer(VcsArtifactApp<ViewerApp<crate::viewer::dwg_ac1024::DwgAc1024Viewer>>),
        DxfAnyEditor(VcsArtifactApp<EditorApp<crate::editor::dxf::DxfAnyEditor>>),
        DxfAnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::dxf::DxfAnyViewer>>),
        GltfAnyEditor(VcsArtifactApp<EditorApp<crate::editor::gltf::GltfAnyEditor>>),
        GltfAnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::gltf::GltfAnyViewer>>),
        ObjAnyEditor(VcsArtifactApp<EditorApp<crate::editor::obj::ObjAnyEditor>>),
        ObjAnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::obj::ObjAnyViewer>>),
        StlAnyEditor(VcsArtifactApp<EditorApp<crate::editor::stl::StlAnyEditor>>),
        StlAnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::stl::StlAnyViewer>>),
        PlyAnyEditor(VcsArtifactApp<EditorApp<crate::editor::ply::PlyAnyEditor>>),
        PlyAnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::ply::PlyAnyViewer>>),
        LasAnyEditor(VcsArtifactApp<EditorApp<crate::editor::las::LasAnyEditor>>),
        LasAnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::las::LasAnyViewer>>),
        BcfAnyEditor(VcsArtifactApp<EditorApp<crate::editor::bcf::BcfAnyEditor>>),
        BcfAnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::bcf::BcfAnyViewer>>),
        CsvEditor(VcsArtifactApp<EditorApp<crate::editor::csv::CsvEditor>>),
        CsvViewer(VcsArtifactApp<ViewerApp<crate::viewer::csv::CsvViewer>>),
        TsvEditor(VcsArtifactApp<EditorApp<crate::editor::tsv::TsvEditor>>),
        TsvViewer(VcsArtifactApp<ViewerApp<crate::viewer::tsv::TsvViewer>>),
        TxtEditor(VcsArtifactApp<EditorApp<crate::editor::txt::TxtEditor>>),
        TxtViewer(VcsArtifactApp<ViewerApp<crate::viewer::txt::TxtViewer>>),
        JsonAnyEditor(VcsArtifactApp<EditorApp<crate::editor::json_any::JsonAnyEditor>>),
        JsonAnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::json_any::JsonAnyViewer>>),
        JsonIJsonEditor(VcsArtifactApp<EditorApp<crate::editor::json_i_json::JsonIJsonEditor>>),
        JsonIJsonViewer(VcsArtifactApp<ViewerApp<crate::viewer::json_i_json::JsonIJsonViewer>>),
        XmlAnyEditor(VcsArtifactApp<EditorApp<crate::editor::xml_any::XmlAnyEditor>>),
        XmlAnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::xml_any::XmlAnyViewer>>),
        XmlValidEditor(VcsArtifactApp<EditorApp<crate::editor::xml_valid::XmlValidEditor>>),
        XmlValidViewer(VcsArtifactApp<ViewerApp<crate::viewer::xml_valid::XmlValidViewer>>),
        Pdf14AEditor(VcsArtifactApp<EditorApp<crate::editor::pdf14a::Pdf14AEditor>>),
        Pdf14AViewer(VcsArtifactApp<ViewerApp<crate::viewer::pdf14a::Pdf14AViewer>>),
        Pdf14Editor(VcsArtifactApp<EditorApp<crate::editor::pdf14::Pdf14Editor>>),
        Pdf14Viewer(VcsArtifactApp<ViewerApp<crate::viewer::pdf14::Pdf14Viewer>>),
        Pdf14XEditor(VcsArtifactApp<EditorApp<crate::editor::pdf14x::Pdf14XEditor>>),
        Pdf14XViewer(VcsArtifactApp<ViewerApp<crate::viewer::pdf14x::Pdf14XViewer>>),
        Pdf17AEditor(VcsArtifactApp<EditorApp<crate::editor::pdf17a::Pdf17AEditor>>),
        Pdf17AViewer(VcsArtifactApp<ViewerApp<crate::viewer::pdf17a::Pdf17AViewer>>),
        Pdf17Editor(VcsArtifactApp<EditorApp<crate::editor::pdf17::Pdf17Editor>>),
        Pdf17Viewer(VcsArtifactApp<ViewerApp<crate::viewer::pdf17::Pdf17Viewer>>),
        Pdf17EEditor(VcsArtifactApp<EditorApp<crate::editor::pdf17e::Pdf17EEditor>>),
        Pdf17EViewer(VcsArtifactApp<ViewerApp<crate::viewer::pdf17e::Pdf17EViewer>>),
        Pdf17HEditor(VcsArtifactApp<EditorApp<crate::editor::pdf17h::Pdf17HEditor>>),
        Pdf17HViewer(VcsArtifactApp<ViewerApp<crate::viewer::pdf17h::Pdf17HViewer>>),
        Pdf17UaEditor(VcsArtifactApp<EditorApp<crate::editor::pdf17ua::Pdf17UaEditor>>),
        Pdf17UaViewer(VcsArtifactApp<ViewerApp<crate::viewer::pdf17ua::Pdf17UaViewer>>),
        Pdf17VtEditor(VcsArtifactApp<EditorApp<crate::editor::pdf17vt::Pdf17VtEditor>>),
        Pdf17VtViewer(VcsArtifactApp<ViewerApp<crate::viewer::pdf17vt::Pdf17VtViewer>>),
        Pdf17XEditor(VcsArtifactApp<EditorApp<crate::editor::pdf17x::Pdf17XEditor>>),
        Pdf17XViewer(VcsArtifactApp<ViewerApp<crate::viewer::pdf17x::Pdf17XViewer>>),
        DocxEditor(VcsArtifactApp<EditorApp<crate::editor::docx::standards::v_ecma_376::subsets::base::DocxEditor>>),
        DocxViewer(VcsArtifactApp<ViewerApp<crate::viewer::docx::standards::v_ecma_376::subsets::base::DocxViewer>>),
        DocxStrictEditor(VcsArtifactApp<EditorApp<crate::editor::docx::standards::v_ecma_376::subsets::strict::DocxStrictEditor>>),
        DocxStrictViewer(VcsArtifactApp<ViewerApp<crate::viewer::docx::standards::v_ecma_376::subsets::strict::DocxStrictViewer>>),
        DocxTransitionalEditor(VcsArtifactApp<EditorApp<crate::editor::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalEditor>>),
        DocxTransitionalViewer(VcsArtifactApp<ViewerApp<crate::viewer::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalViewer>>),
        PptxEditor(VcsArtifactApp<EditorApp<crate::editor::pptx::standards::v_ecma_376::subsets::base::PptxEditor>>),
        PptxViewer(VcsArtifactApp<ViewerApp<crate::viewer::pptx::standards::v_ecma_376::subsets::base::PptxViewer>>),
        PptxStrictEditor(VcsArtifactApp<EditorApp<crate::editor::pptx::standards::v_ecma_376::subsets::strict::PptxStrictEditor>>),
        PptxStrictViewer(VcsArtifactApp<ViewerApp<crate::viewer::pptx::standards::v_ecma_376::subsets::strict::PptxStrictViewer>>),
        PptxTransitionalEditor(VcsArtifactApp<EditorApp<crate::editor::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalEditor>>),
        PptxTransitionalViewer(VcsArtifactApp<ViewerApp<crate::viewer::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalViewer>>),
        XlsxEditor(VcsArtifactApp<EditorApp<crate::editor::xlsx::standards::v_ecma_376::subsets::base::XlsxEditor>>),
        XlsxViewer(VcsArtifactApp<ViewerApp<crate::viewer::xlsx::standards::v_ecma_376::subsets::base::XlsxViewer>>),
        XlsxStrictEditor(VcsArtifactApp<EditorApp<crate::editor::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictEditor>>),
        XlsxStrictViewer(VcsArtifactApp<ViewerApp<crate::viewer::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictViewer>>),
        XlsxTransitionalEditor(VcsArtifactApp<EditorApp<crate::editor::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalEditor>>),
        XlsxTransitionalViewer(VcsArtifactApp<ViewerApp<crate::viewer::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalViewer>>),
        EpwEditor(VcsArtifactApp<EditorApp<crate::editor::epw::EpwEditor>>),
        EpwViewer(VcsArtifactApp<ViewerApp<crate::viewer::epw::EpwViewer>>),
        ZipAnyEditor(VcsArtifactApp<EditorApp<crate::editor::zip::base::ZipAnyEditor>>),
        ZipAnyViewer(VcsArtifactApp<ViewerApp<crate::viewer::zip::base::ZipAnyViewer>>),
        ZipIso21320Editor(VcsArtifactApp<EditorApp<crate::editor::zip::iso21320::ZipIso21320Editor>>),
        ZipIso21320Viewer(VcsArtifactApp<ViewerApp<crate::viewer::zip::iso21320::ZipIso21320Viewer>>),
        DeflateEditor(VcsArtifactApp<EditorApp<crate::editor::deflate::DeflateEditor>>),
        DeflateViewer(VcsArtifactApp<ViewerApp<crate::viewer::deflate::DeflateViewer>>),
        BinaryEditor(VcsArtifactApp<EditorApp<crate::editor::binary::BinaryEditor>>),
        BinaryViewer(VcsArtifactApp<ViewerApp<crate::viewer::binary::BinaryViewer>>),
    }
}

/// 🧾️ Builds all stdio definitions before the typed library assembly boundary. `.activation(…)`/
/// `.execution(…)`/`.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M0,
/// `📓️design-abi.md` §3/§6, following the `✏️s/🔌️plugins/🗒️note` E2 proof migration's shape): stdio
/// owns 36 well-known file-format artifact kinds (see `//#region 🔖️Descriptor` below), so the host
/// activates one `stdio` actor instance whenever any one of them is opened; the actor runs
/// `Isolated` (no publisher trust assumed beyond the sandbox default, same as every other
/// migrated plugin so far); and it asks the broker for document write access, because every one
/// of its ~90 registered editors persists mutations back to whichever of these formats is open.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plugin() -> Result<Plugin<StdioApps>, PluginAssemblyError> {
    let mut builder = Plugin::builder("stdio").label("Stdio").version("0.1.0");
    for assembly in crate::registry::artifact_assemblies()? {
        builder = match assembly {
            crate::registry::ArtifactAssembly::Definition(definition) => builder.artifact_definition(definition),
            crate::registry::ArtifactAssembly::Runtime(declaration) => builder.artifact(declaration),
        };
    }
    //#region 👁️✏️SurfacesP1StdioMedia
    // 🧵 W2 packet P1-stdio-media (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET):
    // png/jpg/bmp/tiff/gif/svg/mp4/mp3/wav/avi/html/md, 17 subsets × {editor, viewer}.
    builder = builder.editor::<crate::editor::png::PngEditor>(crate::editor::png::create_png_editor());
    builder = builder.viewer::<crate::viewer::png::PngViewer>(crate::viewer::png::create_png_viewer());
    builder = builder.editor::<crate::editor::jpg_any::JpgAnyEditor>(crate::editor::jpg_any::create_jpg_any_editor());
    builder = builder.viewer::<crate::viewer::jpg_any::JpgAnyViewer>(crate::viewer::jpg_any::create_jpg_any_viewer());
    builder = builder.editor::<crate::editor::jpg_baseline::JpgBaselineEditor>(crate::editor::jpg_baseline::create_jpg_baseline_editor());
    builder = builder.viewer::<crate::viewer::jpg_baseline::JpgBaselineViewer>(crate::viewer::jpg_baseline::create_jpg_baseline_viewer());
    builder = builder.editor::<crate::editor::bmp::BmpEditor>(crate::editor::bmp::create_bmp_editor());
    builder = builder.viewer::<crate::viewer::bmp::BmpViewer>(crate::viewer::bmp::create_bmp_viewer());
    builder = builder.editor::<crate::editor::tiff_any::TiffAnyEditor>(crate::editor::tiff_any::create_tiff_any_editor());
    builder = builder.viewer::<crate::viewer::tiff_any::TiffAnyViewer>(crate::viewer::tiff_any::create_tiff_any_viewer());
    builder = builder.editor::<crate::editor::tiff_baseline::TiffBaselineEditor>(crate::editor::tiff_baseline::create_tiff_baseline_editor());
    builder = builder.viewer::<crate::viewer::tiff_baseline::TiffBaselineViewer>(crate::viewer::tiff_baseline::create_tiff_baseline_viewer());
    builder = builder.editor::<crate::editor::gif_87a::Gif87aEditor>(crate::editor::gif_87a::create_gif_87a_editor());
    builder = builder.viewer::<crate::viewer::gif_87a::Gif87aViewer>(crate::viewer::gif_87a::create_gif_87a_viewer());
    builder = builder.editor::<crate::editor::gif_89a::Gif89aEditor>(crate::editor::gif_89a::create_gif_89a_editor());
    builder = builder.viewer::<crate::viewer::gif_89a::Gif89aViewer>(crate::viewer::gif_89a::create_gif_89a_viewer());
    builder = builder.editor::<crate::editor::svg_any::SvgAnyEditor>(crate::editor::svg_any::create_svg_any_editor());
    builder = builder.viewer::<crate::viewer::svg_any::SvgAnyViewer>(crate::viewer::svg_any::create_svg_any_viewer());
    builder = builder.editor::<crate::editor::svg_basic::SvgBasicEditor>(crate::editor::svg_basic::create_svg_basic_editor());
    builder = builder.viewer::<crate::viewer::svg_basic::SvgBasicViewer>(crate::viewer::svg_basic::create_svg_basic_viewer());
    builder = builder.editor::<crate::editor::svg_tiny::SvgTinyEditor>(crate::editor::svg_tiny::create_svg_tiny_editor());
    builder = builder.viewer::<crate::viewer::svg_tiny::SvgTinyViewer>(crate::viewer::svg_tiny::create_svg_tiny_viewer());
    builder = builder.editor::<crate::editor::mp4::Mp4Editor>(crate::editor::mp4::create_mp4_editor());
    builder = builder.viewer::<crate::viewer::mp4::Mp4Viewer>(crate::viewer::mp4::create_mp4_viewer());
    builder = builder.editor::<crate::editor::mp3::Mp3Editor>(crate::editor::mp3::create_mp3_editor());
    builder = builder.viewer::<crate::viewer::mp3::Mp3Viewer>(crate::viewer::mp3::create_mp3_viewer());
    builder = builder.editor::<crate::editor::wav::WavEditor>(crate::editor::wav::create_wav_editor());
    builder = builder.viewer::<crate::viewer::wav::WavViewer>(crate::viewer::wav::create_wav_viewer());
    builder = builder.editor::<crate::editor::avi::AviEditor>(crate::editor::avi::create_avi_editor());
    builder = builder.viewer::<crate::viewer::avi::AviViewer>(crate::viewer::avi::create_avi_viewer());
    builder = builder.editor::<crate::editor::html::HtmlEditor>(crate::editor::html::create_html_editor());
    builder = builder.viewer::<crate::viewer::html::HtmlViewer>(crate::viewer::html::create_html_viewer());
    builder = builder.editor::<crate::editor::md::MdEditor>(crate::editor::md::create_md_editor());
    builder = builder.viewer::<crate::viewer::md::MdViewer>(crate::viewer::md::create_md_viewer());
    //#endregion 👁️✏️SurfacesP1StdioMedia

    //#region 👁️✏️SurfacesP3StdioGeometry
    // 🧵 W2 packet P3-stdio-geometry (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET):
    // semio(19)/step(7)/ifc(5)/dwg(2)/dxf/gltf/obj/stl/ply/las/bcf, 40 subsets x {editor, viewer},
    // ALL 40 wired below (lane W2-SDK2 closed the SDK gap `📓️w2-stdio-geometry-report.md` found:
    // `PluginBuilder::editor`/`::viewer` used to require `E::Mutation: protocol::SemanticMutation<
    // E::Snapshot>`, a bound `ArtifactEditor`/`ArtifactViewer` themselves never asked for — contract
    // §2.1/§2.2 only require plain `protocol::Mutation`. That bound only ever fed the *optional*
    // `contributor.list-artifact-mutations` roster capability, so `🏗️builder/🦀️.rs` split
    // it into an opt-in `.editor_mutation_roster::<E>()`/`.viewer_mutation_roster::<V>()` call —
    // `.editor::<E>()`/`.viewer::<V>()` themselves now carry no bound at all. `SemanticMutation` is
    // still "implemented only by `#[derive(Mutations)]`, never by hand" (its own doc comment,
    // `📡️spr/🎮️command/🦀️.rs`), so only the 8 semio subsets that already carry that derive
    // on their pre-existing, schema-owned mutation enum chain the roster call; the other 32 (11 semio
    // + step x7 + ifc x5 + dwg x2 + dxf + gltf + obj + stl + ply + las + bcf) carry a pre-existing
    // HAND-ROLLED `impl protocol::Mutation<Snapshot>` (predates this ticket, outside this packet's
    // lease to migrate) and register/route without contributing a roster row — see
    // `📓️w2-sdk2-report.md` for the full trace and the follow-up this implies for every other W2
    // plugin packet that hit the same wall.
    builder = builder.editor::<crate::editor::semio_brep::SemioBrepEditor>(crate::editor::semio_brep::create_semio_brep_editor());
    builder = builder.editor_mutation_roster::<crate::editor::semio_brep::SemioBrepEditor>();
    builder = builder.viewer::<crate::viewer::semio_brep::SemioBrepViewer>(crate::viewer::semio_brep::create_semio_brep_viewer());
    builder = builder.viewer_mutation_roster::<crate::viewer::semio_brep::SemioBrepViewer>();
    builder = builder.editor::<crate::editor::semio_drawing::SemioDrawingEditor>(crate::editor::semio_drawing::create_semio_drawing_editor());
    builder = builder.editor_mutation_roster::<crate::editor::semio_drawing::SemioDrawingEditor>();
    builder = builder.viewer::<crate::viewer::semio_drawing::SemioDrawingViewer>(crate::viewer::semio_drawing::create_semio_drawing_viewer());
    builder = builder.viewer_mutation_roster::<crate::viewer::semio_drawing::SemioDrawingViewer>();
    builder = builder.editor::<crate::editor::semio_graph::SemioGraphEditor>(crate::editor::semio_graph::create_semio_graph_editor());
    builder = builder.editor_mutation_roster::<crate::editor::semio_graph::SemioGraphEditor>();
    builder = builder.viewer::<crate::viewer::semio_graph::SemioGraphViewer>(crate::viewer::semio_graph::create_semio_graph_viewer());
    builder = builder.viewer_mutation_roster::<crate::viewer::semio_graph::SemioGraphViewer>();
    builder = builder.editor::<crate::editor::semio_kit::SemioKitEditor>(crate::editor::semio_kit::create_semio_kit_editor());
    builder = builder.editor_mutation_roster::<crate::editor::semio_kit::SemioKitEditor>();
    builder = builder.viewer::<crate::viewer::semio_kit::SemioKitViewer>(crate::viewer::semio_kit::create_semio_kit_viewer());
    builder = builder.viewer_mutation_roster::<crate::viewer::semio_kit::SemioKitViewer>();
    builder = builder.editor::<crate::editor::semio_mesh::SemioMeshEditor>(crate::editor::semio_mesh::create_semio_mesh_editor());
    builder = builder.editor_mutation_roster::<crate::editor::semio_mesh::SemioMeshEditor>();
    builder = builder.viewer::<crate::viewer::semio_mesh::SemioMeshViewer>(crate::viewer::semio_mesh::create_semio_mesh_viewer());
    builder = builder.viewer_mutation_roster::<crate::viewer::semio_mesh::SemioMeshViewer>();
    builder = builder.editor::<crate::editor::semio_object::SemioObjectEditor>(crate::editor::semio_object::create_semio_object_editor());
    builder = builder.editor_mutation_roster::<crate::editor::semio_object::SemioObjectEditor>();
    builder = builder.viewer::<crate::viewer::semio_object::SemioObjectViewer>(crate::viewer::semio_object::create_semio_object_viewer());
    builder = builder.viewer_mutation_roster::<crate::viewer::semio_object::SemioObjectViewer>();
    builder = builder.editor::<crate::editor::semio_table::SemioTableEditor>(crate::editor::semio_table::create_semio_table_editor());
    builder = builder.editor_mutation_roster::<crate::editor::semio_table::SemioTableEditor>();
    builder = builder.viewer::<crate::viewer::semio_table::SemioTableViewer>(crate::viewer::semio_table::create_semio_table_viewer());
    builder = builder.viewer_mutation_roster::<crate::viewer::semio_table::SemioTableViewer>();
    builder = builder.editor::<crate::editor::semio_text::SemioTextEditor>(crate::editor::semio_text::create_semio_text_editor());
    builder = builder.editor_mutation_roster::<crate::editor::semio_text::SemioTextEditor>();
    builder = builder.viewer::<crate::viewer::semio_text::SemioTextViewer>(crate::viewer::semio_text::create_semio_text_viewer());
    builder = builder.viewer_mutation_roster::<crate::viewer::semio_text::SemioTextViewer>();
    // 🧵 The 32 previously-unwired subsets — `Mutation` is hand-rolled (no `SemanticMutation`), so
    // no `_mutation_roster` call: they register and route, they just do not contribute a roster row.
    builder = builder.editor::<crate::editor::semio_animation::SemioAnimationEditor>(crate::editor::semio_animation::create_semio_animation_editor());
    builder = builder.viewer::<crate::viewer::semio_animation::SemioAnimationViewer>(crate::viewer::semio_animation::create_semio_animation_viewer());
    builder = builder.editor::<crate::editor::semio_base::SemioAnyEditor>(crate::editor::semio_base::create_semio_base_editor());
    builder = builder.viewer::<crate::viewer::semio_base::SemioAnyViewer>(crate::viewer::semio_base::create_semio_base_viewer());
    builder = builder.editor::<crate::editor::semio_audio::SemioAudioEditor>(crate::editor::semio_audio::create_semio_audio_editor());
    builder = builder.viewer::<crate::viewer::semio_audio::SemioAudioViewer>(crate::viewer::semio_audio::create_semio_audio_viewer());
    builder = builder.editor::<crate::editor::semio_cad::SemioCadEditor>(crate::editor::semio_cad::create_semio_cad_editor());
    builder = builder.viewer::<crate::viewer::semio_cad::SemioCadViewer>(crate::viewer::semio_cad::create_semio_cad_viewer());
    builder = builder.editor::<crate::editor::semio_document::SemioDocumentEditor>(crate::editor::semio_document::create_semio_document_editor());
    builder = builder.viewer::<crate::viewer::semio_document::SemioDocumentViewer>(crate::viewer::semio_document::create_semio_document_viewer());
    builder = builder.editor::<crate::editor::semio_flow::SemioFlowEditor>(crate::editor::semio_flow::create_semio_flow_editor());
    builder = builder.viewer::<crate::viewer::semio_flow::SemioFlowViewer>(crate::viewer::semio_flow::create_semio_flow_viewer());
    builder = builder.editor::<crate::editor::semio_image::SemioImageEditor>(crate::editor::semio_image::create_semio_image_editor());
    builder = builder.viewer::<crate::viewer::semio_image::SemioImageViewer>(crate::viewer::semio_image::create_semio_image_viewer());
    builder = builder.editor::<crate::editor::semio_model::SemioModelEditor>(crate::editor::semio_model::create_semio_model_editor());
    builder = builder.viewer::<crate::viewer::semio_model::SemioModelViewer>(crate::viewer::semio_model::create_semio_model_viewer());
    builder = builder.editor::<crate::editor::semio_presentation::SemioPresentationEditor>(crate::editor::semio_presentation::create_semio_presentation_editor());
    builder = builder.viewer::<crate::viewer::semio_presentation::SemioPresentationViewer>(crate::viewer::semio_presentation::create_semio_presentation_viewer());
    builder = builder.editor::<crate::editor::semio_value::SemioValueEditor>(crate::editor::semio_value::create_semio_value_editor());
    builder = builder.viewer::<crate::viewer::semio_value::SemioValueViewer>(crate::viewer::semio_value::create_semio_value_viewer());
    builder = builder.editor::<crate::editor::semio_video::SemioVideoEditor>(crate::editor::semio_video::create_semio_video_editor());
    builder = builder.viewer::<crate::viewer::semio_video::SemioVideoViewer>(crate::viewer::semio_video::create_semio_video_viewer());
    builder = builder.editor::<crate::editor::step_any::StepAnyEditor>(crate::editor::step_any::create_step_any_editor());
    builder = builder.viewer::<crate::viewer::step_any::StepAnyViewer>(crate::viewer::step_any::create_step_any_viewer());
    builder = builder.editor::<crate::editor::step_cc1::StepCc1Editor>(crate::editor::step_cc1::create_step_cc1_editor());
    builder = builder.viewer::<crate::viewer::step_cc1::StepCc1Viewer>(crate::viewer::step_cc1::create_step_cc1_viewer());
    builder = builder.editor::<crate::editor::step_cc2::StepCc2Editor>(crate::editor::step_cc2::create_step_cc2_editor());
    builder = builder.viewer::<crate::viewer::step_cc2::StepCc2Viewer>(crate::viewer::step_cc2::create_step_cc2_viewer());
    builder = builder.editor::<crate::editor::step_cc3::StepCc3Editor>(crate::editor::step_cc3::create_step_cc3_editor());
    builder = builder.viewer::<crate::viewer::step_cc3::StepCc3Viewer>(crate::viewer::step_cc3::create_step_cc3_viewer());
    builder = builder.editor::<crate::editor::step_cc4::StepCc4Editor>(crate::editor::step_cc4::create_step_cc4_editor());
    builder = builder.viewer::<crate::viewer::step_cc4::StepCc4Viewer>(crate::viewer::step_cc4::create_step_cc4_viewer());
    builder = builder.editor::<crate::editor::step_cc5::StepCc5Editor>(crate::editor::step_cc5::create_step_cc5_editor());
    builder = builder.viewer::<crate::viewer::step_cc5::StepCc5Viewer>(crate::viewer::step_cc5::create_step_cc5_viewer());
    builder = builder.editor::<crate::editor::step_cc6::StepCc6Editor>(crate::editor::step_cc6::create_step_cc6_editor());
    builder = builder.viewer::<crate::viewer::step_cc6::StepCc6Viewer>(crate::viewer::step_cc6::create_step_cc6_viewer());
    builder = builder.editor::<crate::editor::ifc2x3_any::Ifc2x3AnyEditor>(crate::editor::ifc2x3_any::create_ifc2x3_any_editor());
    builder = builder.viewer::<crate::viewer::ifc2x3_any::Ifc2x3AnyViewer>(crate::viewer::ifc2x3_any::create_ifc2x3_any_viewer());
    builder = builder.editor::<crate::editor::ifc2x3_cobie::Ifc2x3CobieEditor>(crate::editor::ifc2x3_cobie::create_ifc2x3_cobie_editor());
    builder = builder.viewer::<crate::viewer::ifc2x3_cobie::Ifc2x3CobieViewer>(crate::viewer::ifc2x3_cobie::create_ifc2x3_cobie_viewer());
    builder = builder.editor::<crate::editor::ifc2x3_cv20::Ifc2x3Cv20Editor>(crate::editor::ifc2x3_cv20::create_ifc2x3_cv20_editor());
    builder = builder.viewer::<crate::viewer::ifc2x3_cv20::Ifc2x3Cv20Viewer>(crate::viewer::ifc2x3_cv20::create_ifc2x3_cv20_viewer());
    builder = builder.editor::<crate::editor::ifc2x3_sav::Ifc2x3SavEditor>(crate::editor::ifc2x3_sav::create_ifc2x3_sav_editor());
    builder = builder.viewer::<crate::viewer::ifc2x3_sav::Ifc2x3SavViewer>(crate::viewer::ifc2x3_sav::create_ifc2x3_sav_viewer());
    builder = builder.editor::<crate::editor::ifc4_any::Ifc4AnyEditor>(crate::editor::ifc4_any::create_ifc4_any_editor());
    builder = builder.viewer::<crate::viewer::ifc4_any::Ifc4AnyViewer>(crate::viewer::ifc4_any::create_ifc4_any_viewer());
    builder = builder.editor::<crate::editor::dwg_ac1018::DwgAc1018Editor>(crate::editor::dwg_ac1018::create_dwg_ac1018_editor());
    builder = builder.viewer::<crate::viewer::dwg_ac1018::DwgAc1018Viewer>(crate::viewer::dwg_ac1018::create_dwg_ac1018_viewer());
    builder = builder.editor::<crate::editor::dwg_ac1024::DwgAc1024Editor>(crate::editor::dwg_ac1024::create_dwg_ac1024_editor());
    builder = builder.viewer::<crate::viewer::dwg_ac1024::DwgAc1024Viewer>(crate::viewer::dwg_ac1024::create_dwg_ac1024_viewer());
    builder = builder.editor::<crate::editor::dxf::DxfAnyEditor>(crate::editor::dxf::create_dxf_any_editor());
    builder = builder.viewer::<crate::viewer::dxf::DxfAnyViewer>(crate::viewer::dxf::create_dxf_any_viewer());
    builder = builder.editor::<crate::editor::gltf::GltfAnyEditor>(crate::editor::gltf::create_gltf_any_editor());
    builder = builder.viewer::<crate::viewer::gltf::GltfAnyViewer>(crate::viewer::gltf::create_gltf_any_viewer());
    builder = builder.editor::<crate::editor::obj::ObjAnyEditor>(crate::editor::obj::create_obj_any_editor());
    builder = builder.viewer::<crate::viewer::obj::ObjAnyViewer>(crate::viewer::obj::create_obj_any_viewer());
    builder = builder.editor::<crate::editor::stl::StlAnyEditor>(crate::editor::stl::create_stl_any_editor());
    builder = builder.viewer::<crate::viewer::stl::StlAnyViewer>(crate::viewer::stl::create_stl_any_viewer());
    builder = builder.editor::<crate::editor::ply::PlyAnyEditor>(crate::editor::ply::create_ply_any_editor());
    builder = builder.viewer::<crate::viewer::ply::PlyAnyViewer>(crate::viewer::ply::create_ply_any_viewer());
    builder = builder.editor::<crate::editor::las::LasAnyEditor>(crate::editor::las::create_las_any_editor());
    builder = builder.viewer::<crate::viewer::las::LasAnyViewer>(crate::viewer::las::create_las_any_viewer());
    builder = builder.editor::<crate::editor::bcf::BcfAnyEditor>(crate::editor::bcf::create_bcf_any_editor());
    builder = builder.viewer::<crate::viewer::bcf::BcfAnyViewer>(crate::viewer::bcf::create_bcf_any_viewer());
    //#endregion 👁️✏️SurfacesP3StdioGeometry

    //#region 👁️✏️SurfacesP2StdioData
    // 🧵 W2 packet P2-stdio-data (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET):
    // csv/tsv/txt/json(2)/xml(2), 7 subsets × {editor, viewer}. pdf(10)/docx(3)/pptx(3)/xlsx(3)/
    // epw/zip(2)/deflate/binary are mounted separately below, same packet.
    builder = builder.editor::<crate::editor::csv::CsvEditor>(crate::editor::csv::create_csv_editor());
    builder = builder.viewer::<crate::viewer::csv::CsvViewer>(crate::viewer::csv::create_csv_viewer());
    builder = builder.editor::<crate::editor::tsv::TsvEditor>(crate::editor::tsv::create_tsv_editor());
    builder = builder.viewer::<crate::viewer::tsv::TsvViewer>(crate::viewer::tsv::create_tsv_viewer());
    builder = builder.editor::<crate::editor::txt::TxtEditor>(crate::editor::txt::create_txt_editor());
    builder = builder.viewer::<crate::viewer::txt::TxtViewer>(crate::viewer::txt::create_txt_viewer());
    builder = builder.editor::<crate::editor::json_any::JsonAnyEditor>(crate::editor::json_any::create_json_editor());
    builder = builder.viewer::<crate::viewer::json_any::JsonAnyViewer>(crate::viewer::json_any::create_json_viewer());
    builder = builder.editor::<crate::editor::json_i_json::JsonIJsonEditor>(crate::editor::json_i_json::create_json_i_json_editor());
    builder = builder.viewer::<crate::viewer::json_i_json::JsonIJsonViewer>(crate::viewer::json_i_json::create_json_i_json_viewer());
    builder = builder.editor::<crate::editor::xml_any::XmlAnyEditor>(crate::editor::xml_any::create_xml_editor());
    builder = builder.viewer::<crate::viewer::xml_any::XmlAnyViewer>(crate::viewer::xml_any::create_xml_viewer());
    builder = builder.editor::<crate::editor::xml_valid::XmlValidEditor>(crate::editor::xml_valid::create_xml_valid_editor());
    builder = builder.viewer::<crate::viewer::xml_valid::XmlValidViewer>(crate::viewer::xml_valid::create_xml_valid_viewer());
    //#endregion 👁️✏️SurfacesP2StdioData

    //#region 👁️✏️SurfacesP2StdioDataDocuments
    // 🧵 W2 packet P2-stdio-data (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET):
    // pdf(10)/docx(3)/pptx(3)/xlsx(3), 19 subsets × {editor, viewer}.
    builder = builder.editor::<crate::editor::pdf14a::Pdf14AEditor>(crate::editor::pdf14a::create_pdf14_a_editor());
    builder = builder.viewer::<crate::viewer::pdf14a::Pdf14AViewer>(crate::viewer::pdf14a::create_pdf14_a_viewer());
    builder = builder.editor::<crate::editor::pdf14::Pdf14Editor>(crate::editor::pdf14::create_pdf14_editor());
    builder = builder.viewer::<crate::viewer::pdf14::Pdf14Viewer>(crate::viewer::pdf14::create_pdf14_viewer());
    builder = builder.editor::<crate::editor::pdf14x::Pdf14XEditor>(crate::editor::pdf14x::create_pdf14_x_editor());
    builder = builder.viewer::<crate::viewer::pdf14x::Pdf14XViewer>(crate::viewer::pdf14x::create_pdf14_x_viewer());
    builder = builder.editor::<crate::editor::pdf17a::Pdf17AEditor>(crate::editor::pdf17a::create_pdf17_a_editor());
    builder = builder.viewer::<crate::viewer::pdf17a::Pdf17AViewer>(crate::viewer::pdf17a::create_pdf17_a_viewer());
    builder = builder.editor::<crate::editor::pdf17::Pdf17Editor>(crate::editor::pdf17::create_pdf17_editor());
    builder = builder.viewer::<crate::viewer::pdf17::Pdf17Viewer>(crate::viewer::pdf17::create_pdf17_viewer());
    builder = builder.editor::<crate::editor::pdf17e::Pdf17EEditor>(crate::editor::pdf17e::create_pdf17_e_editor());
    builder = builder.viewer::<crate::viewer::pdf17e::Pdf17EViewer>(crate::viewer::pdf17e::create_pdf17_e_viewer());
    builder = builder.editor::<crate::editor::pdf17h::Pdf17HEditor>(crate::editor::pdf17h::create_pdf17_h_editor());
    builder = builder.viewer::<crate::viewer::pdf17h::Pdf17HViewer>(crate::viewer::pdf17h::create_pdf17_h_viewer());
    builder = builder.editor::<crate::editor::pdf17ua::Pdf17UaEditor>(crate::editor::pdf17ua::create_pdf17_ua_editor());
    builder = builder.viewer::<crate::viewer::pdf17ua::Pdf17UaViewer>(crate::viewer::pdf17ua::create_pdf17_ua_viewer());
    builder = builder.editor::<crate::editor::pdf17vt::Pdf17VtEditor>(crate::editor::pdf17vt::create_pdf17_vt_editor());
    builder = builder.viewer::<crate::viewer::pdf17vt::Pdf17VtViewer>(crate::viewer::pdf17vt::create_pdf17_vt_viewer());
    builder = builder.editor::<crate::editor::pdf17x::Pdf17XEditor>(crate::editor::pdf17x::create_pdf17_x_editor());
    builder = builder.viewer::<crate::viewer::pdf17x::Pdf17XViewer>(crate::viewer::pdf17x::create_pdf17_x_viewer());
    builder = builder.editor::<crate::editor::docx::standards::v_ecma_376::subsets::base::DocxEditor>(crate::editor::docx::standards::v_ecma_376::subsets::base::create_docx_editor());
    builder = builder.viewer::<crate::viewer::docx::standards::v_ecma_376::subsets::base::DocxViewer>(crate::viewer::docx::standards::v_ecma_376::subsets::base::create_docx_viewer());
    builder = builder.editor::<crate::editor::docx::standards::v_ecma_376::subsets::strict::DocxStrictEditor>(crate::editor::docx::standards::v_ecma_376::subsets::strict::create_docx_strict_editor());
    builder = builder.viewer::<crate::viewer::docx::standards::v_ecma_376::subsets::strict::DocxStrictViewer>(crate::viewer::docx::standards::v_ecma_376::subsets::strict::create_docx_strict_viewer());
    builder = builder.editor::<crate::editor::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalEditor>(crate::editor::docx::standards::v_ecma_376::subsets::transitional::create_docx_transitional_editor());
    builder = builder.viewer::<crate::viewer::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalViewer>(crate::viewer::docx::standards::v_ecma_376::subsets::transitional::create_docx_transitional_viewer());
    builder = builder.editor::<crate::editor::pptx::standards::v_ecma_376::subsets::base::PptxEditor>(crate::editor::pptx::standards::v_ecma_376::subsets::base::create_pptx_editor());
    builder = builder.viewer::<crate::viewer::pptx::standards::v_ecma_376::subsets::base::PptxViewer>(crate::viewer::pptx::standards::v_ecma_376::subsets::base::create_pptx_viewer());
    builder = builder.editor::<crate::editor::pptx::standards::v_ecma_376::subsets::strict::PptxStrictEditor>(crate::editor::pptx::standards::v_ecma_376::subsets::strict::create_pptx_strict_editor());
    builder = builder.viewer::<crate::viewer::pptx::standards::v_ecma_376::subsets::strict::PptxStrictViewer>(crate::viewer::pptx::standards::v_ecma_376::subsets::strict::create_pptx_strict_viewer());
    builder = builder.editor::<crate::editor::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalEditor>(crate::editor::pptx::standards::v_ecma_376::subsets::transitional::create_pptx_transitional_editor());
    builder = builder.viewer::<crate::viewer::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalViewer>(crate::viewer::pptx::standards::v_ecma_376::subsets::transitional::create_pptx_transitional_viewer());
    builder = builder.editor::<crate::editor::xlsx::standards::v_ecma_376::subsets::base::XlsxEditor>(crate::editor::xlsx::standards::v_ecma_376::subsets::base::create_xlsx_editor());
    builder = builder.viewer::<crate::viewer::xlsx::standards::v_ecma_376::subsets::base::XlsxViewer>(crate::viewer::xlsx::standards::v_ecma_376::subsets::base::create_xlsx_viewer());
    builder = builder.editor::<crate::editor::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictEditor>(crate::editor::xlsx::standards::v_ecma_376::subsets::strict::create_xlsx_strict_editor());
    builder = builder.viewer::<crate::viewer::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictViewer>(crate::viewer::xlsx::standards::v_ecma_376::subsets::strict::create_xlsx_strict_viewer());
    builder = builder.editor::<crate::editor::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalEditor>(crate::editor::xlsx::standards::v_ecma_376::subsets::transitional::create_xlsx_transitional_editor());
    builder = builder.viewer::<crate::viewer::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalViewer>(crate::viewer::xlsx::standards::v_ecma_376::subsets::transitional::create_xlsx_transitional_viewer());
    //#endregion 👁️✏️SurfacesP2StdioDataDocuments

    //#region 👁️✏️SurfacesP2StdioDataMisc
    // 🧵 W2 packet P2-stdio-data (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET):
    // epw/zip(2)/deflate/binary, 5 subsets × {editor, viewer}.
    builder = builder.editor::<crate::editor::epw::EpwEditor>(crate::editor::epw::create_epw_editor());
    builder = builder.viewer::<crate::viewer::epw::EpwViewer>(crate::viewer::epw::create_epw_viewer());
    builder = builder.editor::<crate::editor::zip::base::ZipAnyEditor>(crate::editor::zip::base::create_zip_any_editor());
    builder = builder.viewer::<crate::viewer::zip::base::ZipAnyViewer>(crate::viewer::zip::base::create_zip_any_viewer());
    builder = builder.editor::<crate::editor::zip::iso21320::ZipIso21320Editor>(crate::editor::zip::iso21320::create_zip_iso21320_editor());
    builder = builder.viewer::<crate::viewer::zip::iso21320::ZipIso21320Viewer>(crate::viewer::zip::iso21320::create_zip_iso21320_viewer());
    builder = builder.editor::<crate::editor::deflate::DeflateEditor>(crate::editor::deflate::create_deflate_editor());
    builder = builder.viewer::<crate::viewer::deflate::DeflateViewer>(crate::viewer::deflate::create_deflate_viewer());
    builder = builder.editor::<crate::editor::binary::BinaryEditor>(crate::editor::binary::create_binary_editor());
    builder = builder.viewer::<crate::viewer::binary::BinaryViewer>(crate::viewer::binary::create_binary_viewer());
    //#endregion 👁️✏️SurfacesP2StdioDataMisc

    //#region 🔖️Descriptor
    // 🚀 Ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M0 (`📓️design-abi.md` §3/§6): one
    // `on-artifact-kind:` activation event per artifact kind this crate genuinely owns — every
    // top-level `crate::artifacts::<fmt>::artifact_kind()` function in the tree (36 formats: image/
    // audio/video/text/data/document/geometry), each read via its own function rather than a
    // hardcoded string so this list can never silently drift from the real declarations above.
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::binary::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::txt::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::json::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::xml::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::csv::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::md::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::deflate::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::zip::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::step::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::ifc::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::las::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::gltf::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::obj::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::ply::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::dxf::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::stl::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::svg::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::bmp::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::dwg::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::png::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::pdf::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::jpg::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::gif::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::tiff::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::docx::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::pptx::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::xlsx::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::bcf::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::semio::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::mp4::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::avi::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::mp3::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::wav::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::epw::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::tsv::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::html::artifact_kind().id });
    builder = builder.execution(ExecutionMode::Isolated);
    builder = builder.requests(CapabilityRequest {
        id: CapabilityId("documents.write".into()),
        scope: "plugin".into(),
        reason: "persist editor mutations back to whichever of stdio's 36 owned file-format artifacts (image/audio/video/text/data/document/geometry) is currently open".into(),
        optional: false,
    });
    //#endregion 🔖️Descriptor

    builder.try_library()
}
