//! 🔌️ Schema-owned stdio library plugin assembly.

use semio_framework_dispatch_macros::dyn_enum_close;
use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp, PluginAssemblyError};

// 🗃️ Closed runtime app fleet for every stdio editor and viewer surface.
dyn_enum_close! {
    pub enum StdioApps: PluginApp {
        PngEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_png::editor::png::PngEditor>>),
        PngViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_png::viewer::png::PngViewer>>),
        JpgAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_jpg::editor::jpg_any::JpgAnyEditor>>),
        JpgAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_jpg::viewer::jpg_any::JpgAnyViewer>>),
        JpgBaselineEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_jpg::editor::jpg_baseline::JpgBaselineEditor>>),
        JpgBaselineViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_jpg::viewer::jpg_baseline::JpgBaselineViewer>>),
        BmpEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_bmp::editor::bmp::BmpEditor>>),
        BmpViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_bmp::viewer::bmp::BmpViewer>>),
        TiffAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_tiff::editor::tiff_any::TiffAnyEditor>>),
        TiffAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_tiff::viewer::tiff_any::TiffAnyViewer>>),
        TiffBaselineEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_tiff::editor::tiff_baseline::TiffBaselineEditor>>),
        TiffBaselineViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_tiff::viewer::tiff_baseline::TiffBaselineViewer>>),
        Gif87aEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_gif::editor::gif_87a::Gif87aEditor>>),
        Gif87aViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_gif::viewer::gif_87a::Gif87aViewer>>),
        Gif89aEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_gif::editor::gif_89a::Gif89aEditor>>),
        Gif89aViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_gif::viewer::gif_89a::Gif89aViewer>>),
        SvgAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_svg::editor::svg_any::SvgAnyEditor>>),
        SvgAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_svg::viewer::svg_any::SvgAnyViewer>>),
        SvgBasicEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_svg::editor::svg_basic::SvgBasicEditor>>),
        SvgBasicViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_svg::viewer::svg_basic::SvgBasicViewer>>),
        SvgTinyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_svg::editor::svg_tiny::SvgTinyEditor>>),
        SvgTinyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_svg::viewer::svg_tiny::SvgTinyViewer>>),
        Mp4Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_mp4::editor::mp4::Mp4Editor>>),
        Mp4Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_mp4::viewer::mp4::Mp4Viewer>>),
        Mp3Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_mp3::editor::mp3::Mp3Editor>>),
        Mp3Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_mp3::viewer::mp3::Mp3Viewer>>),
        WavEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_wav::editor::wav::WavEditor>>),
        WavViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_wav::viewer::wav::WavViewer>>),
        AviEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_avi::editor::avi::AviEditor>>),
        AviViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_avi::viewer::avi::AviViewer>>),
        HtmlEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_html::editor::html::HtmlEditor>>),
        HtmlViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_html::viewer::html::HtmlViewer>>),
        MdEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_md::editor::md::MdEditor>>),
        MdViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_md::viewer::md::MdViewer>>),
        SemioBrepEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_brep::SemioBrepEditor>>),
        SemioBrepViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_brep::SemioBrepViewer>>),
        SemioDrawingEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_drawing::SemioDrawingEditor>>),
        SemioDrawingViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_drawing::SemioDrawingViewer>>),
        SemioGraphEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_graph::SemioGraphEditor>>),
        SemioGraphViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_graph::SemioGraphViewer>>),
        SemioKitEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_kit::SemioKitEditor>>),
        SemioKitViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_kit::SemioKitViewer>>),
        SemioMeshEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_mesh::SemioMeshEditor>>),
        SemioMeshViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_mesh::SemioMeshViewer>>),
        SemioObjectEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_object::SemioObjectEditor>>),
        SemioObjectViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_object::SemioObjectViewer>>),
        SemioTableEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_table::SemioTableEditor>>),
        SemioTableViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_table::SemioTableViewer>>),
        SemioTextEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_text::SemioTextEditor>>),
        SemioTextViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_text::SemioTextViewer>>),
        SemioAnimationEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_animation::SemioAnimationEditor>>),
        SemioAnimationViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_animation::SemioAnimationViewer>>),
        SemioAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_base::SemioAnyEditor>>),
        SemioAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_base::SemioAnyViewer>>),
        SemioAudioEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_audio::SemioAudioEditor>>),
        SemioAudioViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_audio::SemioAudioViewer>>),
        SemioCadEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_cad::SemioCadEditor>>),
        SemioCadViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_cad::SemioCadViewer>>),
        SemioDocumentEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_document::SemioDocumentEditor>>),
        SemioDocumentViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_document::SemioDocumentViewer>>),
        SemioFlowEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_flow::SemioFlowEditor>>),
        SemioFlowViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_flow::SemioFlowViewer>>),
        SemioImageEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_image::SemioImageEditor>>),
        SemioImageViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_image::SemioImageViewer>>),
        SemioModelEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_model::SemioModelEditor>>),
        SemioModelViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_model::SemioModelViewer>>),
        SemioPresentationEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_presentation::SemioPresentationEditor>>),
        SemioPresentationViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_presentation::SemioPresentationViewer>>),
        SemioValueEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_value::SemioValueEditor>>),
        SemioValueViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_value::SemioValueViewer>>),
        SemioVideoEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_semio::editor::semio_video::SemioVideoEditor>>),
        SemioVideoViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_semio::viewer::semio_video::SemioVideoViewer>>),
        StepAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_step::editor::step_any::StepAnyEditor>>),
        StepAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_step::viewer::step_any::StepAnyViewer>>),
        StepCc1Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_step::editor::step_cc1::StepCc1Editor>>),
        StepCc1Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_step::viewer::step_cc1::StepCc1Viewer>>),
        StepCc2Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_step::editor::step_cc2::StepCc2Editor>>),
        StepCc2Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_step::viewer::step_cc2::StepCc2Viewer>>),
        StepCc3Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_step::editor::step_cc3::StepCc3Editor>>),
        StepCc3Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_step::viewer::step_cc3::StepCc3Viewer>>),
        StepCc4Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_step::editor::step_cc4::StepCc4Editor>>),
        StepCc4Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_step::viewer::step_cc4::StepCc4Viewer>>),
        StepCc5Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_step::editor::step_cc5::StepCc5Editor>>),
        StepCc5Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_step::viewer::step_cc5::StepCc5Viewer>>),
        StepCc6Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_step::editor::step_cc6::StepCc6Editor>>),
        StepCc6Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_step::viewer::step_cc6::StepCc6Viewer>>),
        Ifc2x3AnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_ifc::editor::ifc2x3_any::Ifc2x3AnyEditor>>),
        Ifc2x3AnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_ifc::viewer::ifc2x3_any::Ifc2x3AnyViewer>>),
        Ifc2x3CobieEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_ifc::editor::ifc2x3_cobie::Ifc2x3CobieEditor>>),
        Ifc2x3CobieViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_ifc::viewer::ifc2x3_cobie::Ifc2x3CobieViewer>>),
        Ifc2x3Cv20Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_ifc::editor::ifc2x3_cv20::Ifc2x3Cv20Editor>>),
        Ifc2x3Cv20Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_ifc::viewer::ifc2x3_cv20::Ifc2x3Cv20Viewer>>),
        Ifc2x3SavEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_ifc::editor::ifc2x3_sav::Ifc2x3SavEditor>>),
        Ifc2x3SavViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_ifc::viewer::ifc2x3_sav::Ifc2x3SavViewer>>),
        Ifc4AnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_ifc::editor::ifc4_any::Ifc4AnyEditor>>),
        Ifc4AnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_ifc::viewer::ifc4_any::Ifc4AnyViewer>>),
        DwgAc1018Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_dwg::editor::dwg_ac1018::DwgAc1018Editor>>),
        DwgAc1018Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_dwg::viewer::dwg_ac1018::DwgAc1018Viewer>>),
        DwgAc1024Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_dwg::editor::dwg_ac1024::DwgAc1024Editor>>),
        DwgAc1024Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_dwg::viewer::dwg_ac1024::DwgAc1024Viewer>>),
        DxfAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_dxf::editor::dxf::DxfAnyEditor>>),
        DxfAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_dxf::viewer::dxf::DxfAnyViewer>>),
        GltfAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_gltf::editor::gltf::GltfAnyEditor>>),
        GltfAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_gltf::viewer::gltf::GltfAnyViewer>>),
        ObjAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_obj::editor::obj::ObjAnyEditor>>),
        ObjAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_obj::viewer::obj::ObjAnyViewer>>),
        StlAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_stl::editor::stl::StlAnyEditor>>),
        StlAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_stl::viewer::stl::StlAnyViewer>>),
        PlyAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_ply::editor::ply::PlyAnyEditor>>),
        PlyAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_ply::viewer::ply::PlyAnyViewer>>),
        LasAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_las::editor::las::LasAnyEditor>>),
        LasAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_las::viewer::las::LasAnyViewer>>),
        BcfAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_bcf::editor::bcf::BcfAnyEditor>>),
        BcfAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_bcf::viewer::bcf::BcfAnyViewer>>),
        CsvEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_csv::editor::csv::CsvEditor>>),
        CsvViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_csv::viewer::csv::CsvViewer>>),
        TsvEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_tsv::editor::tsv::TsvEditor>>),
        TsvViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_tsv::viewer::tsv::TsvViewer>>),
        TxtEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_txt::editor::txt::TxtEditor>>),
        TxtViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_txt::viewer::txt::TxtViewer>>),
        JsonAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_json::editor::json_any::JsonAnyEditor>>),
        JsonAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_json::viewer::json_any::JsonAnyViewer>>),
        JsonIJsonEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_json::editor::json_i_json::JsonIJsonEditor>>),
        JsonIJsonViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_json::viewer::json_i_json::JsonIJsonViewer>>),
        XmlAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xml::editor::xml_any::XmlAnyEditor>>),
        XmlAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xml::viewer::xml_any::XmlAnyViewer>>),
        XmlValidEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xml::editor::xml_valid::XmlValidEditor>>),
        XmlValidViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xml::viewer::xml_valid::XmlValidViewer>>),
        Pdf14AEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf14a::Pdf14AEditor>>),
        Pdf14AViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf14a::Pdf14AViewer>>),
        Pdf14Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf14::Pdf14Editor>>),
        Pdf14Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf14::Pdf14Viewer>>),
        Pdf14XEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf14x::Pdf14XEditor>>),
        Pdf14XViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf14x::Pdf14XViewer>>),
        Pdf17AEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17a::Pdf17AEditor>>),
        Pdf17AViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17a::Pdf17AViewer>>),
        Pdf17Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17::Pdf17Editor>>),
        Pdf17Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17::Pdf17Viewer>>),
        Pdf17EEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17e::Pdf17EEditor>>),
        Pdf17EViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17e::Pdf17EViewer>>),
        Pdf17HEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17h::Pdf17HEditor>>),
        Pdf17HViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17h::Pdf17HViewer>>),
        Pdf17UaEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17ua::Pdf17UaEditor>>),
        Pdf17UaViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17ua::Pdf17UaViewer>>),
        Pdf17VtEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17vt::Pdf17VtEditor>>),
        Pdf17VtViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17vt::Pdf17VtViewer>>),
        Pdf17XEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pdf::editor::pdf17x::Pdf17XEditor>>),
        Pdf17XViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pdf::viewer::pdf17x::Pdf17XViewer>>),
        DocxEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::base::DocxEditor>>),
        DocxViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::base::DocxViewer>>),
        DocxStrictEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::strict::DocxStrictEditor>>),
        DocxStrictViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::strict::DocxStrictViewer>>),
        DocxTransitionalEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalEditor>>),
        DocxTransitionalViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalViewer>>),
        PptxEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::base::PptxEditor>>),
        PptxViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::base::PptxViewer>>),
        PptxStrictEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::strict::PptxStrictEditor>>),
        PptxStrictViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::strict::PptxStrictViewer>>),
        PptxTransitionalEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalEditor>>),
        PptxTransitionalViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalViewer>>),
        XlsxEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::base::XlsxEditor>>),
        XlsxViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::base::XlsxViewer>>),
        XlsxStrictEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictEditor>>),
        XlsxStrictViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictViewer>>),
        XlsxTransitionalEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalEditor>>),
        XlsxTransitionalViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalViewer>>),
        EpwEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_epw::editor::epw::EpwEditor>>),
        EpwViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_epw::viewer::epw::EpwViewer>>),
        ZipAnyEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_zip::editor::zip::base::ZipAnyEditor>>),
        ZipAnyViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_zip::viewer::zip::base::ZipAnyViewer>>),
        ZipIso21320Editor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_zip::editor::zip::iso21320::ZipIso21320Editor>>),
        ZipIso21320Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_zip::viewer::zip::iso21320::ZipIso21320Viewer>>),
        DeflateEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_deflate::editor::deflate::DeflateEditor>>),
        DeflateViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_deflate::viewer::deflate::DeflateViewer>>),
        BinaryEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_binary::editor::binary::BinaryEditor>>),
        BinaryViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_binary::viewer::binary::BinaryViewer>>),
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
    let mut builder = Plugin::builder("stdio").label("Stdio").version("0.1.0").package_id(crate::registry::component_package_id()?);
    let assemblies = crate::registry::artifact_assemblies()?;
    let catalog = crate::registry::artifact_catalog_contribution(&assemblies)?;
    for assembly in assemblies {
        builder = match assembly {
            crate::registry::ArtifactAssembly::Definition(definition) => builder.artifact_definition(definition),
            crate::registry::ArtifactAssembly::Runtime(declaration) => builder.artifact(declaration),
        };
    }
    for artifact_kind in crate::registry::native_codec_artifact_kinds() {
        builder = builder.artifact_kind(artifact_kind);
    }
    //#region 👁️✏️SurfacesP1StdioMedia
    // 🧵 W2 packet P1-stdio-media (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET):
    // png/jpg/bmp/tiff/gif/svg/mp4/mp3/wav/avi/html/md, 17 subsets × {editor, viewer}.
    builder = builder.editor::<semio_s_artifact_stdio_png::editor::png::PngEditor>(semio_s_artifact_stdio_png::editor::png::create_png_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_png::viewer::png::PngViewer>(semio_s_artifact_stdio_png::viewer::png::create_png_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_jpg::editor::jpg_any::JpgAnyEditor>(semio_s_artifact_stdio_jpg::editor::jpg_any::create_jpg_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_jpg::viewer::jpg_any::JpgAnyViewer>(semio_s_artifact_stdio_jpg::viewer::jpg_any::create_jpg_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_jpg::editor::jpg_baseline::JpgBaselineEditor>(semio_s_artifact_stdio_jpg::editor::jpg_baseline::create_jpg_baseline_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_jpg::viewer::jpg_baseline::JpgBaselineViewer>(semio_s_artifact_stdio_jpg::viewer::jpg_baseline::create_jpg_baseline_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_bmp::editor::bmp::BmpEditor>(semio_s_artifact_stdio_bmp::editor::bmp::create_bmp_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_bmp::viewer::bmp::BmpViewer>(semio_s_artifact_stdio_bmp::viewer::bmp::create_bmp_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_tiff::editor::tiff_any::TiffAnyEditor>(semio_s_artifact_stdio_tiff::editor::tiff_any::create_tiff_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_tiff::viewer::tiff_any::TiffAnyViewer>(semio_s_artifact_stdio_tiff::viewer::tiff_any::create_tiff_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_tiff::editor::tiff_baseline::TiffBaselineEditor>(semio_s_artifact_stdio_tiff::editor::tiff_baseline::create_tiff_baseline_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_tiff::viewer::tiff_baseline::TiffBaselineViewer>(semio_s_artifact_stdio_tiff::viewer::tiff_baseline::create_tiff_baseline_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_gif::editor::gif_87a::Gif87aEditor>(semio_s_artifact_stdio_gif::editor::gif_87a::create_gif_87a_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_gif::viewer::gif_87a::Gif87aViewer>(semio_s_artifact_stdio_gif::viewer::gif_87a::create_gif_87a_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_gif::editor::gif_89a::Gif89aEditor>(semio_s_artifact_stdio_gif::editor::gif_89a::create_gif_89a_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_gif::viewer::gif_89a::Gif89aViewer>(semio_s_artifact_stdio_gif::viewer::gif_89a::create_gif_89a_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_svg::editor::svg_any::SvgAnyEditor>(semio_s_artifact_stdio_svg::editor::svg_any::create_svg_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_svg::viewer::svg_any::SvgAnyViewer>(semio_s_artifact_stdio_svg::viewer::svg_any::create_svg_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_svg::editor::svg_basic::SvgBasicEditor>(semio_s_artifact_stdio_svg::editor::svg_basic::create_svg_basic_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_svg::viewer::svg_basic::SvgBasicViewer>(semio_s_artifact_stdio_svg::viewer::svg_basic::create_svg_basic_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_svg::editor::svg_tiny::SvgTinyEditor>(semio_s_artifact_stdio_svg::editor::svg_tiny::create_svg_tiny_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_svg::viewer::svg_tiny::SvgTinyViewer>(semio_s_artifact_stdio_svg::viewer::svg_tiny::create_svg_tiny_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_mp4::editor::mp4::Mp4Editor>(semio_s_artifact_stdio_mp4::editor::mp4::create_mp4_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_mp4::viewer::mp4::Mp4Viewer>(semio_s_artifact_stdio_mp4::viewer::mp4::create_mp4_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_mp3::editor::mp3::Mp3Editor>(semio_s_artifact_stdio_mp3::editor::mp3::create_mp3_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_mp3::viewer::mp3::Mp3Viewer>(semio_s_artifact_stdio_mp3::viewer::mp3::create_mp3_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_wav::editor::wav::WavEditor>(semio_s_artifact_stdio_wav::editor::wav::create_wav_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_wav::viewer::wav::WavViewer>(semio_s_artifact_stdio_wav::viewer::wav::create_wav_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_avi::editor::avi::AviEditor>(semio_s_artifact_stdio_avi::editor::avi::create_avi_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_avi::viewer::avi::AviViewer>(semio_s_artifact_stdio_avi::viewer::avi::create_avi_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_html::editor::html::HtmlEditor>(semio_s_artifact_stdio_html::editor::html::create_html_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_html::viewer::html::HtmlViewer>(semio_s_artifact_stdio_html::viewer::html::create_html_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_md::editor::md::MdEditor>(semio_s_artifact_stdio_md::editor::md::create_md_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_md::viewer::md::MdViewer>(semio_s_artifact_stdio_md::viewer::md::create_md_viewer());
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
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_brep::SemioBrepEditor>(semio_s_artifact_stdio_semio::editor::semio_brep::create_semio_brep_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_brep::SemioBrepEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_brep::SemioBrepViewer>(semio_s_artifact_stdio_semio::viewer::semio_brep::create_semio_brep_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_brep::SemioBrepViewer>();
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_drawing::SemioDrawingEditor>(semio_s_artifact_stdio_semio::editor::semio_drawing::create_semio_drawing_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_drawing::SemioDrawingEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_drawing::SemioDrawingViewer>(semio_s_artifact_stdio_semio::viewer::semio_drawing::create_semio_drawing_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_drawing::SemioDrawingViewer>();
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_graph::SemioGraphEditor>(semio_s_artifact_stdio_semio::editor::semio_graph::create_semio_graph_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_graph::SemioGraphEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_graph::SemioGraphViewer>(semio_s_artifact_stdio_semio::viewer::semio_graph::create_semio_graph_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_graph::SemioGraphViewer>();
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_kit::SemioKitEditor>(semio_s_artifact_stdio_semio::editor::semio_kit::create_semio_kit_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_kit::SemioKitEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_kit::SemioKitViewer>(semio_s_artifact_stdio_semio::viewer::semio_kit::create_semio_kit_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_kit::SemioKitViewer>();
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_mesh::SemioMeshEditor>(semio_s_artifact_stdio_semio::editor::semio_mesh::create_semio_mesh_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_mesh::SemioMeshEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_mesh::SemioMeshViewer>(semio_s_artifact_stdio_semio::viewer::semio_mesh::create_semio_mesh_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_mesh::SemioMeshViewer>();
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_object::SemioObjectEditor>(semio_s_artifact_stdio_semio::editor::semio_object::create_semio_object_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_object::SemioObjectEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_object::SemioObjectViewer>(semio_s_artifact_stdio_semio::viewer::semio_object::create_semio_object_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_object::SemioObjectViewer>();
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_table::SemioTableEditor>(semio_s_artifact_stdio_semio::editor::semio_table::create_semio_table_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_table::SemioTableEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_table::SemioTableViewer>(semio_s_artifact_stdio_semio::viewer::semio_table::create_semio_table_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_table::SemioTableViewer>();
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_text::SemioTextEditor>(semio_s_artifact_stdio_semio::editor::semio_text::create_semio_text_editor());
    builder = builder.editor_mutation_roster::<semio_s_artifact_stdio_semio::editor::semio_text::SemioTextEditor>();
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_text::SemioTextViewer>(semio_s_artifact_stdio_semio::viewer::semio_text::create_semio_text_viewer());
    builder = builder.viewer_mutation_roster::<semio_s_artifact_stdio_semio::viewer::semio_text::SemioTextViewer>();
    // 🧵 The 32 previously-unwired subsets — `Mutation` is hand-rolled (no `SemanticMutation`), so
    // no `_mutation_roster` call: they register and route, they just do not contribute a roster row.
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_animation::SemioAnimationEditor>(semio_s_artifact_stdio_semio::editor::semio_animation::create_semio_animation_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_animation::SemioAnimationViewer>(semio_s_artifact_stdio_semio::viewer::semio_animation::create_semio_animation_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_base::SemioAnyEditor>(semio_s_artifact_stdio_semio::editor::semio_base::create_semio_base_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_base::SemioAnyViewer>(semio_s_artifact_stdio_semio::viewer::semio_base::create_semio_base_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_audio::SemioAudioEditor>(semio_s_artifact_stdio_semio::editor::semio_audio::create_semio_audio_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_audio::SemioAudioViewer>(semio_s_artifact_stdio_semio::viewer::semio_audio::create_semio_audio_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_cad::SemioCadEditor>(semio_s_artifact_stdio_semio::editor::semio_cad::create_semio_cad_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_cad::SemioCadViewer>(semio_s_artifact_stdio_semio::viewer::semio_cad::create_semio_cad_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_document::SemioDocumentEditor>(semio_s_artifact_stdio_semio::editor::semio_document::create_semio_document_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_document::SemioDocumentViewer>(semio_s_artifact_stdio_semio::viewer::semio_document::create_semio_document_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_flow::SemioFlowEditor>(semio_s_artifact_stdio_semio::editor::semio_flow::create_semio_flow_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_flow::SemioFlowViewer>(semio_s_artifact_stdio_semio::viewer::semio_flow::create_semio_flow_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_image::SemioImageEditor>(semio_s_artifact_stdio_semio::editor::semio_image::create_semio_image_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_image::SemioImageViewer>(semio_s_artifact_stdio_semio::viewer::semio_image::create_semio_image_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_model::SemioModelEditor>(semio_s_artifact_stdio_semio::editor::semio_model::create_semio_model_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_model::SemioModelViewer>(semio_s_artifact_stdio_semio::viewer::semio_model::create_semio_model_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_presentation::SemioPresentationEditor>(semio_s_artifact_stdio_semio::editor::semio_presentation::create_semio_presentation_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_presentation::SemioPresentationViewer>(semio_s_artifact_stdio_semio::viewer::semio_presentation::create_semio_presentation_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_value::SemioValueEditor>(semio_s_artifact_stdio_semio::editor::semio_value::create_semio_value_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_value::SemioValueViewer>(semio_s_artifact_stdio_semio::viewer::semio_value::create_semio_value_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_semio::editor::semio_video::SemioVideoEditor>(semio_s_artifact_stdio_semio::editor::semio_video::create_semio_video_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_semio::viewer::semio_video::SemioVideoViewer>(semio_s_artifact_stdio_semio::viewer::semio_video::create_semio_video_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_step::editor::step_any::StepAnyEditor>(semio_s_artifact_stdio_step::editor::step_any::create_step_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_step::viewer::step_any::StepAnyViewer>(semio_s_artifact_stdio_step::viewer::step_any::create_step_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_step::editor::step_cc1::StepCc1Editor>(semio_s_artifact_stdio_step::editor::step_cc1::create_step_cc1_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_step::viewer::step_cc1::StepCc1Viewer>(semio_s_artifact_stdio_step::viewer::step_cc1::create_step_cc1_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_step::editor::step_cc2::StepCc2Editor>(semio_s_artifact_stdio_step::editor::step_cc2::create_step_cc2_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_step::viewer::step_cc2::StepCc2Viewer>(semio_s_artifact_stdio_step::viewer::step_cc2::create_step_cc2_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_step::editor::step_cc3::StepCc3Editor>(semio_s_artifact_stdio_step::editor::step_cc3::create_step_cc3_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_step::viewer::step_cc3::StepCc3Viewer>(semio_s_artifact_stdio_step::viewer::step_cc3::create_step_cc3_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_step::editor::step_cc4::StepCc4Editor>(semio_s_artifact_stdio_step::editor::step_cc4::create_step_cc4_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_step::viewer::step_cc4::StepCc4Viewer>(semio_s_artifact_stdio_step::viewer::step_cc4::create_step_cc4_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_step::editor::step_cc5::StepCc5Editor>(semio_s_artifact_stdio_step::editor::step_cc5::create_step_cc5_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_step::viewer::step_cc5::StepCc5Viewer>(semio_s_artifact_stdio_step::viewer::step_cc5::create_step_cc5_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_step::editor::step_cc6::StepCc6Editor>(semio_s_artifact_stdio_step::editor::step_cc6::create_step_cc6_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_step::viewer::step_cc6::StepCc6Viewer>(semio_s_artifact_stdio_step::viewer::step_cc6::create_step_cc6_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_ifc::editor::ifc2x3_any::Ifc2x3AnyEditor>(semio_s_artifact_stdio_ifc::editor::ifc2x3_any::create_ifc2x3_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_ifc::viewer::ifc2x3_any::Ifc2x3AnyViewer>(semio_s_artifact_stdio_ifc::viewer::ifc2x3_any::create_ifc2x3_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_ifc::editor::ifc2x3_cobie::Ifc2x3CobieEditor>(semio_s_artifact_stdio_ifc::editor::ifc2x3_cobie::create_ifc2x3_cobie_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_ifc::viewer::ifc2x3_cobie::Ifc2x3CobieViewer>(semio_s_artifact_stdio_ifc::viewer::ifc2x3_cobie::create_ifc2x3_cobie_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_ifc::editor::ifc2x3_cv20::Ifc2x3Cv20Editor>(semio_s_artifact_stdio_ifc::editor::ifc2x3_cv20::create_ifc2x3_cv20_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_ifc::viewer::ifc2x3_cv20::Ifc2x3Cv20Viewer>(semio_s_artifact_stdio_ifc::viewer::ifc2x3_cv20::create_ifc2x3_cv20_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_ifc::editor::ifc2x3_sav::Ifc2x3SavEditor>(semio_s_artifact_stdio_ifc::editor::ifc2x3_sav::create_ifc2x3_sav_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_ifc::viewer::ifc2x3_sav::Ifc2x3SavViewer>(semio_s_artifact_stdio_ifc::viewer::ifc2x3_sav::create_ifc2x3_sav_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_ifc::editor::ifc4_any::Ifc4AnyEditor>(semio_s_artifact_stdio_ifc::editor::ifc4_any::create_ifc4_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_ifc::viewer::ifc4_any::Ifc4AnyViewer>(semio_s_artifact_stdio_ifc::viewer::ifc4_any::create_ifc4_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_dwg::editor::dwg_ac1018::DwgAc1018Editor>(semio_s_artifact_stdio_dwg::editor::dwg_ac1018::create_dwg_ac1018_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_dwg::viewer::dwg_ac1018::DwgAc1018Viewer>(semio_s_artifact_stdio_dwg::viewer::dwg_ac1018::create_dwg_ac1018_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_dwg::editor::dwg_ac1024::DwgAc1024Editor>(semio_s_artifact_stdio_dwg::editor::dwg_ac1024::create_dwg_ac1024_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_dwg::viewer::dwg_ac1024::DwgAc1024Viewer>(semio_s_artifact_stdio_dwg::viewer::dwg_ac1024::create_dwg_ac1024_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_dxf::editor::dxf::DxfAnyEditor>(semio_s_artifact_stdio_dxf::editor::dxf::create_dxf_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_dxf::viewer::dxf::DxfAnyViewer>(semio_s_artifact_stdio_dxf::viewer::dxf::create_dxf_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_gltf::editor::gltf::GltfAnyEditor>(semio_s_artifact_stdio_gltf::editor::gltf::create_gltf_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_gltf::viewer::gltf::GltfAnyViewer>(semio_s_artifact_stdio_gltf::viewer::gltf::create_gltf_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_obj::editor::obj::ObjAnyEditor>(semio_s_artifact_stdio_obj::editor::obj::create_obj_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_obj::viewer::obj::ObjAnyViewer>(semio_s_artifact_stdio_obj::viewer::obj::create_obj_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_stl::editor::stl::StlAnyEditor>(semio_s_artifact_stdio_stl::editor::stl::create_stl_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_stl::viewer::stl::StlAnyViewer>(semio_s_artifact_stdio_stl::viewer::stl::create_stl_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_ply::editor::ply::PlyAnyEditor>(semio_s_artifact_stdio_ply::editor::ply::create_ply_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_ply::viewer::ply::PlyAnyViewer>(semio_s_artifact_stdio_ply::viewer::ply::create_ply_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_las::editor::las::LasAnyEditor>(semio_s_artifact_stdio_las::editor::las::create_las_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_las::viewer::las::LasAnyViewer>(semio_s_artifact_stdio_las::viewer::las::create_las_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_bcf::editor::bcf::BcfAnyEditor>(semio_s_artifact_stdio_bcf::editor::bcf::create_bcf_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_bcf::viewer::bcf::BcfAnyViewer>(semio_s_artifact_stdio_bcf::viewer::bcf::create_bcf_any_viewer());
    //#endregion 👁️✏️SurfacesP3StdioGeometry

    //#region 👁️✏️SurfacesP2StdioData
    // 🧵 W2 packet P2-stdio-data (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET):
    // csv/tsv/txt/json(2)/xml(2), 7 subsets × {editor, viewer}. pdf(10)/docx(3)/pptx(3)/xlsx(3)/
    // epw/zip(2)/deflate/binary are mounted separately below, same packet.
    builder = builder.editor::<semio_s_artifact_stdio_csv::editor::csv::CsvEditor>(semio_s_artifact_stdio_csv::editor::csv::create_csv_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_csv::viewer::csv::CsvViewer>(semio_s_artifact_stdio_csv::viewer::csv::create_csv_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_tsv::editor::tsv::TsvEditor>(semio_s_artifact_stdio_tsv::editor::tsv::create_tsv_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_tsv::viewer::tsv::TsvViewer>(semio_s_artifact_stdio_tsv::viewer::tsv::create_tsv_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_txt::editor::txt::TxtEditor>(semio_s_artifact_stdio_txt::editor::txt::create_txt_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_txt::viewer::txt::TxtViewer>(semio_s_artifact_stdio_txt::viewer::txt::create_txt_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_json::editor::json_any::JsonAnyEditor>(semio_s_artifact_stdio_json::editor::json_any::create_json_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_json::viewer::json_any::JsonAnyViewer>(semio_s_artifact_stdio_json::viewer::json_any::create_json_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_json::editor::json_i_json::JsonIJsonEditor>(semio_s_artifact_stdio_json::editor::json_i_json::create_json_i_json_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_json::viewer::json_i_json::JsonIJsonViewer>(semio_s_artifact_stdio_json::viewer::json_i_json::create_json_i_json_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_xml::editor::xml_any::XmlAnyEditor>(semio_s_artifact_stdio_xml::editor::xml_any::create_xml_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_xml::viewer::xml_any::XmlAnyViewer>(semio_s_artifact_stdio_xml::viewer::xml_any::create_xml_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_xml::editor::xml_valid::XmlValidEditor>(semio_s_artifact_stdio_xml::editor::xml_valid::create_xml_valid_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_xml::viewer::xml_valid::XmlValidViewer>(semio_s_artifact_stdio_xml::viewer::xml_valid::create_xml_valid_viewer());
    //#endregion 👁️✏️SurfacesP2StdioData

    //#region 👁️✏️SurfacesP2StdioDataDocuments
    // 🧵 W2 packet P2-stdio-data (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET):
    // pdf(10)/docx(3)/pptx(3)/xlsx(3), 19 subsets × {editor, viewer}.
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf14a::Pdf14AEditor>(semio_s_artifact_stdio_pdf::editor::pdf14a::create_pdf14_a_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf14a::Pdf14AViewer>(semio_s_artifact_stdio_pdf::viewer::pdf14a::create_pdf14_a_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf14::Pdf14Editor>(semio_s_artifact_stdio_pdf::editor::pdf14::create_pdf14_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf14::Pdf14Viewer>(semio_s_artifact_stdio_pdf::viewer::pdf14::create_pdf14_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf14x::Pdf14XEditor>(semio_s_artifact_stdio_pdf::editor::pdf14x::create_pdf14_x_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf14x::Pdf14XViewer>(semio_s_artifact_stdio_pdf::viewer::pdf14x::create_pdf14_x_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17a::Pdf17AEditor>(semio_s_artifact_stdio_pdf::editor::pdf17a::create_pdf17_a_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17a::Pdf17AViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17a::create_pdf17_a_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17::Pdf17Editor>(semio_s_artifact_stdio_pdf::editor::pdf17::create_pdf17_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17::Pdf17Viewer>(semio_s_artifact_stdio_pdf::viewer::pdf17::create_pdf17_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17e::Pdf17EEditor>(semio_s_artifact_stdio_pdf::editor::pdf17e::create_pdf17_e_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17e::Pdf17EViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17e::create_pdf17_e_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17h::Pdf17HEditor>(semio_s_artifact_stdio_pdf::editor::pdf17h::create_pdf17_h_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17h::Pdf17HViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17h::create_pdf17_h_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17ua::Pdf17UaEditor>(semio_s_artifact_stdio_pdf::editor::pdf17ua::create_pdf17_ua_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17ua::Pdf17UaViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17ua::create_pdf17_ua_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17vt::Pdf17VtEditor>(semio_s_artifact_stdio_pdf::editor::pdf17vt::create_pdf17_vt_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17vt::Pdf17VtViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17vt::create_pdf17_vt_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pdf::editor::pdf17x::Pdf17XEditor>(semio_s_artifact_stdio_pdf::editor::pdf17x::create_pdf17_x_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pdf::viewer::pdf17x::Pdf17XViewer>(semio_s_artifact_stdio_pdf::viewer::pdf17x::create_pdf17_x_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::base::DocxEditor>(semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::base::create_docx_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::base::DocxViewer>(semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::base::create_docx_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::strict::DocxStrictEditor>(semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::strict::create_docx_strict_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::strict::DocxStrictViewer>(semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::strict::create_docx_strict_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalEditor>(
        semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::transitional::create_docx_transitional_editor(),
    );
    builder = builder.viewer::<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalViewer>(
        semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::transitional::create_docx_transitional_viewer(),
    );
    builder = builder.editor::<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::base::PptxEditor>(semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::base::create_pptx_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::base::PptxViewer>(semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::base::create_pptx_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::strict::PptxStrictEditor>(semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::strict::create_pptx_strict_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::strict::PptxStrictViewer>(semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::strict::create_pptx_strict_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalEditor>(
        semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::transitional::create_pptx_transitional_editor(),
    );
    builder = builder.viewer::<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalViewer>(
        semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::transitional::create_pptx_transitional_viewer(),
    );
    builder = builder.editor::<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::base::XlsxEditor>(semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::base::create_xlsx_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::base::XlsxViewer>(semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::base::create_xlsx_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictEditor>(semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::strict::create_xlsx_strict_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictViewer>(semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::strict::create_xlsx_strict_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalEditor>(
        semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::transitional::create_xlsx_transitional_editor(),
    );
    builder = builder.viewer::<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalViewer>(
        semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::transitional::create_xlsx_transitional_viewer(),
    );
    //#endregion 👁️✏️SurfacesP2StdioDataDocuments

    //#region 👁️✏️SurfacesP2StdioDataMisc
    // 🧵 W2 packet P2-stdio-data (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET):
    // epw/zip(2)/deflate/binary, 5 subsets × {editor, viewer}.
    builder = builder.editor::<semio_s_artifact_stdio_epw::editor::epw::EpwEditor>(semio_s_artifact_stdio_epw::editor::epw::create_epw_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_epw::viewer::epw::EpwViewer>(semio_s_artifact_stdio_epw::viewer::epw::create_epw_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_zip::editor::zip::base::ZipAnyEditor>(semio_s_artifact_stdio_zip::editor::zip::base::create_zip_any_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_zip::viewer::zip::base::ZipAnyViewer>(semio_s_artifact_stdio_zip::viewer::zip::base::create_zip_any_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_zip::editor::zip::iso21320::ZipIso21320Editor>(semio_s_artifact_stdio_zip::editor::zip::iso21320::create_zip_iso21320_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_zip::viewer::zip::iso21320::ZipIso21320Viewer>(semio_s_artifact_stdio_zip::viewer::zip::iso21320::create_zip_iso21320_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_deflate::editor::deflate::DeflateEditor>(semio_s_artifact_stdio_deflate::editor::deflate::create_deflate_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_deflate::viewer::deflate::DeflateViewer>(semio_s_artifact_stdio_deflate::viewer::deflate::create_deflate_viewer());
    builder = builder.editor::<semio_s_artifact_stdio_binary::editor::binary::BinaryEditor>(semio_s_artifact_stdio_binary::editor::binary::create_binary_editor());
    builder = builder.viewer::<semio_s_artifact_stdio_binary::viewer::binary::BinaryViewer>(semio_s_artifact_stdio_binary::viewer::binary::create_binary_viewer());
    //#endregion 👁️✏️SurfacesP2StdioDataMisc

    //#region 🔖️Descriptor
    // 🚀 Ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M0 (`📓️design-abi.md` §3/§6): one
    // `on-artifact-kind:` activation event per artifact kind this crate genuinely owns — every
    // package-local `semio_s_artifact_stdio_<fmt>::artifact_kind()` function (36 formats: image/
    // audio/video/text/data/document/geometry), each read via its own function rather than a
    // hardcoded string so this list can never silently drift from the real declarations above.
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_binary::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_txt::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_json::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_xml::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_csv::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_md::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_deflate::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_zip::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_step::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_ifc::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_las::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_gltf::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_obj::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_ply::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_dxf::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_stl::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_svg::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_bmp::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_dwg::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_png::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_pdf::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_jpg::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_gif::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_tiff::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_docx::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_pptx::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_xlsx::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_bcf::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_semio::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_mp4::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_avi::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_mp3::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_wav::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_epw::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_tsv::artifact_kind().id });
    builder = builder.activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_html::artifact_kind().id });
    builder = builder.execution(ExecutionMode::Isolated);
    builder = builder.requests(CapabilityRequest {
        id: CapabilityId("documents.write".into()),
        scope: "plugin".into(),
        reason: "persist editor mutations back to whichever of stdio's 36 owned file-format artifacts (image/audio/video/text/data/document/geometry) is currently open".into(),
        optional: false,
    });
    //#endregion 🔖️Descriptor

    builder.contributes_topic(catalog).try_library()
}
