//! 🏭️ Production mutation bridge for `✏️s/🔌️plugins/🗄️stdio`.
//!
//! `test inventory` runs this and compares what it prints against each owner manifest and its claimed test
//! catalog. Every row is read out of a production aggregate's `DESCRIPTORS`, which the `dsl::Mutations` derive
//! generates from the mutation leaves themselves; a subset's inventory is the descriptors whose leaf `owner` lies
//! inside that subset's owner, so a verb dispatched in production and absent from the manifest is a breach.
//!
//! @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🏭️inventory/📋️orchestration/🟦️.ts — the caller.

extern crate semio_framework_os_kernel as protocol;

use protocol::{Mutation, MutationLeafDescriptor, MutationOutcomeClass};

/// 🧬️ One aggregate's descriptors, for the snapshot its `#[mutations(snapshot = …)]` names.
fn descriptors<S, M: Mutation<S>>() -> &'static [MutationLeafDescriptor] {
    M::DESCRIPTORS
}

/// 🧭️ Every mutation aggregate the artifact crates below mount.
const AGGREGATES: &[fn() -> &'static [MutationLeafDescriptor]] = &[
    descriptors::<semio_s_artifact_stdio_avi::standards::v1_0::subsets::any::schema::snapshot::AviSnapshot, semio_s_artifact_stdio_avi::standards::v1_0::subsets::any::schema::mutations::AviMutation>,
    descriptors::<semio_s_artifact_stdio_bcf::standards::v2_1::subsets::any::schema::snapshot::BcfSnapshot, semio_s_artifact_stdio_bcf::standards::v2_1::subsets::any::schema::mutations::BcfMutation>,
    descriptors::<semio_s_artifact_stdio_binary::standards::v_raw::subsets::any::schema::snapshot::BinarySnapshot, semio_s_artifact_stdio_binary::standards::v_raw::subsets::any::schema::mutations::BinaryMutation>,
    descriptors::<semio_s_artifact_stdio_bmp::standards::v_v3::subsets::any::schema::snapshot::BmpSnapshot, semio_s_artifact_stdio_bmp::standards::v_v3::subsets::any::schema::mutations::BmpMutation>,
    descriptors::<semio_s_artifact_stdio_csv::standards::v_rfc4180::subsets::any::schema::snapshot::CsvSnapshot, semio_s_artifact_stdio_csv::standards::v_rfc4180::subsets::any::schema::mutations::CsvMutation>,
    descriptors::<semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::schema::snapshot::DeflateSnapshot, semio_s_artifact_stdio_deflate::standards::v_rfc1950::subsets::any::schema::mutations::DeflateMutation>,
    descriptors::<semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::base::schema::snapshot::DocxSnapshot, semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::base::schema::mutations::DocxMutation>,
    descriptors::<semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::base::schema::snapshot::DocxSnapshot, semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::strict::schema::mutations::DocxStrictMutation>,
    descriptors::<semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::base::schema::snapshot::DocxSnapshot, semio_s_artifact_stdio_docx::standards::v_ecma_376::subsets::transitional::schema::mutations::DocxTransitionalMutation>,
    descriptors::<semio_s_artifact_stdio_dwg::standards::v_ac1024::subsets::any::schema::snapshot::DwgSnapshot, semio_s_artifact_stdio_dwg::standards::v_ac1024::subsets::any::schema::mutations::DwgMutation>,
    descriptors::<semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::schema::snapshot::DxfSnapshot, semio_s_artifact_stdio_dxf::standards::v_r12::subsets::any::schema::mutations::DxfMutation>,
    descriptors::<semio_s_artifact_stdio_epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot, semio_s_artifact_stdio_epw::standards::energyplus::subsets::any::schema::mutations::EpwMutation>,
    descriptors::<semio_s_artifact_stdio_gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot, semio_s_artifact_stdio_gif::standards::v87a::subsets::any::schema::mutations::GifMutation>,
    descriptors::<semio_s_artifact_stdio_gif::standards::v87a::subsets::any::schema::snapshot::GifSnapshot, semio_s_artifact_stdio_gif::standards::v89a::subsets::any::schema::mutations::GifMutation>,
    descriptors::<semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot, semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::schema::mutations::GltfMutation>,
    descriptors::<semio_s_artifact_stdio_html::standards::v5::subsets::any::schema::snapshot::HtmlSnapshot, semio_s_artifact_stdio_html::standards::v5::subsets::any::schema::mutations::HtmlMutation>,
    descriptors::<semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot, semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation>,
    descriptors::<semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot, semio_s_artifact_stdio_ifc::standards::v2x3::subsets::cobie::schema::mutations::Ifc2x3CobieMutation>,
    descriptors::<semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot, semio_s_artifact_stdio_ifc::standards::v2x3::subsets::cv20::schema::mutations::Ifc2x3Cv20Mutation>,
    descriptors::<semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot, semio_s_artifact_stdio_ifc::standards::v2x3::subsets::sav::schema::mutations::Ifc2x3SavMutation>,
    descriptors::<semio_s_artifact_stdio_ifc::standards::v4::subsets::any::schema::snapshot::IfcSnapshot, semio_s_artifact_stdio_ifc::standards::v4::subsets::any::schema::mutations::IfcMutation>,
    descriptors::<semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::document::schema::snapshot::JpgSnapshot, semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::baseline::schema::mutations::JpgBaselineMutation>,
    descriptors::<semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::document::schema::snapshot::JpgSnapshot, semio_s_artifact_stdio_jpg::standards::v_jfif_1_01::subsets::document::schema::mutations::JpgMutation>,
    descriptors::<semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::schema::snapshot::JsonSnapshot, semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::schema::mutations::JsonMutation>,
    descriptors::<semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::base::schema::snapshot::JsonSnapshot, semio_s_artifact_stdio_json::standards::v_rfc8259::subsets::i_json::schema::mutations::JsonIJsonMutation>,
    descriptors::<semio_s_artifact_stdio_las::standards::v1_0::subsets::any::schema::snapshot::LasSnapshot, semio_s_artifact_stdio_las::standards::v1_0::subsets::any::schema::mutations::LasMutation>,
    descriptors::<semio_s_artifact_stdio_md::standards::v_commonmark::subsets::any::schema::snapshot::MdSnapshot, semio_s_artifact_stdio_md::standards::v_commonmark::subsets::any::schema::mutations::MdMutation>,
    descriptors::<semio_s_artifact_stdio_mp3::standards::mpeg1_layer3::subsets::any::schema::snapshot::Mp3Snapshot, semio_s_artifact_stdio_mp3::standards::mpeg1_layer3::subsets::any::schema::mutations::Mp3Mutation>,
    descriptors::<semio_s_artifact_stdio_mp4::standards::isobmff::subsets::any::schema::snapshot::Mp4Snapshot, semio_s_artifact_stdio_mp4::standards::isobmff::subsets::any::schema::mutations::Mp4Mutation>,
    descriptors::<semio_s_artifact_stdio_obj::standards::v3_0::subsets::any::schema::snapshot::ObjSnapshot, semio_s_artifact_stdio_obj::standards::v3_0::subsets::any::schema::mutations::ObjMutation>,
    descriptors::<semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, semio_s_artifact_stdio_pdf::standards::v1_4::subsets::a::schema::mutations::PdfA1Mutation>,
    descriptors::<semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::mutations::PdfMutation>,
    descriptors::<semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, semio_s_artifact_stdio_pdf::standards::v1_4::subsets::x::schema::mutations::PdfX1Mutation>,
    descriptors::<semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, semio_s_artifact_stdio_pdf::standards::v1_7::subsets::a::schema::mutations::PdfAMutation>,
    descriptors::<semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, semio_s_artifact_stdio_pdf::standards::v1_7::subsets::base::schema::mutations::PdfMutation>,
    descriptors::<semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, semio_s_artifact_stdio_pdf::standards::v1_7::subsets::e::schema::mutations::PdfEMutation>,
    descriptors::<semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, semio_s_artifact_stdio_pdf::standards::v1_7::subsets::h::schema::mutations::PdfHMutation>,
    descriptors::<semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, semio_s_artifact_stdio_pdf::standards::v1_7::subsets::ua::schema::mutations::PdfUaMutation>,
    descriptors::<semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, semio_s_artifact_stdio_pdf::standards::v1_7::subsets::vt::schema::mutations::PdfVtMutation>,
    descriptors::<semio_s_artifact_stdio_pdf::standards::v1_4::subsets::base::schema::snapshot::PdfSnapshot, semio_s_artifact_stdio_pdf::standards::v1_7::subsets::x::schema::mutations::PdfXMutation>,
    descriptors::<semio_s_artifact_stdio_ply::standards::v1_0::subsets::any::schema::snapshot::PlySnapshot, semio_s_artifact_stdio_ply::standards::v1_0::subsets::any::schema::mutations::PlyMutation>,
    descriptors::<semio_s_artifact_stdio_png::standards::v1_2::subsets::any::schema::snapshot::PngSnapshot, semio_s_artifact_stdio_png::standards::v1_2::subsets::any::schema::mutations::PngMutation>,
    descriptors::<semio_s_artifact_stdio_pptx::standards::v_ecma_376::subsets::base::schema::snapshot::PptxSnapshot, semio_s_artifact_stdio_pptx::standards::v_ecma_376::subsets::base::schema::mutations::PptxMutation>,
    descriptors::<semio_s_artifact_stdio_pptx::standards::v_ecma_376::subsets::base::schema::snapshot::PptxSnapshot, semio_s_artifact_stdio_pptx::standards::v_ecma_376::subsets::strict::schema::mutations::PptxStrictMutation>,
    descriptors::<semio_s_artifact_stdio_pptx::standards::v_ecma_376::subsets::base::schema::snapshot::PptxSnapshot, semio_s_artifact_stdio_pptx::standards::v_ecma_376::subsets::transitional::schema::mutations::PptxTransitionalMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::animation::schema::snapshot::SemioAnimationSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::animation::schema::mutations::SemioAnimationMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::audio::schema::snapshot::SemioAudioSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::audio::schema::mutations::SemioAudioMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::snapshot::SemioSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::mutations::SemioMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::mutations::SemioBrepMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::cad::schema::snapshot::SemioCadSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::cad::schema::mutations::SemioCadMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::document::schema::snapshot::SemioDocumentSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::document::schema::mutations::SemioDocumentMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::snapshot::SemioDrawingSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::drawing::schema::mutations::SemioDrawingMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::mutations::SemioFlowMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::mutations::SemioGraphMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::mutations::SemioKitMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::snapshot::SemioMeshSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::schema::mutations::SemioMeshMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::model::schema::snapshot::SemioModelSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::model::schema::mutations::SemioModelMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::object::schema::snapshot::SemioObjectSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::object::schema::mutations::SemioObjectMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::presentation::schema::snapshot::SemioPresentationSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::presentation::schema::mutations::SemioPresentationMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::snapshot::SemioTableSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::table::schema::mutations::SemioTableMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::text::schema::snapshot::SemioTextSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::text::schema::mutations::SemioTextMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::snapshot::SemioValueSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::value::schema::mutations::SemioValueMutation>,
    descriptors::<semio_s_artifact_stdio_semio::standards::v1::subsets::video::schema::snapshot::SemioVideoSnapshot, semio_s_artifact_stdio_semio::standards::v1::subsets::video::schema::mutations::SemioVideoMutation>,
    descriptors::<semio_s_artifact_stdio_step::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot, semio_s_artifact_stdio_step::standards::v_ap214::subsets::base::schema::mutations::StepMutation>,
    descriptors::<semio_s_artifact_stdio_step::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot, semio_s_artifact_stdio_step::standards::v_ap214::subsets::cc1::schema::mutations::StepCc1Mutation>,
    descriptors::<semio_s_artifact_stdio_step::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot, semio_s_artifact_stdio_step::standards::v_ap214::subsets::cc2::schema::mutations::StepCc2Mutation>,
    descriptors::<semio_s_artifact_stdio_step::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot, semio_s_artifact_stdio_step::standards::v_ap214::subsets::cc3::schema::mutations::StepCc3Mutation>,
    descriptors::<semio_s_artifact_stdio_step::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot, semio_s_artifact_stdio_step::standards::v_ap214::subsets::cc4::schema::mutations::StepCc4Mutation>,
    descriptors::<semio_s_artifact_stdio_step::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot, semio_s_artifact_stdio_step::standards::v_ap214::subsets::cc5::schema::mutations::StepCc5Mutation>,
    descriptors::<semio_s_artifact_stdio_step::standards::v_ap214::subsets::base::schema::snapshot::StepSnapshot, semio_s_artifact_stdio_step::standards::v_ap214::subsets::cc6::schema::mutations::StepCc6Mutation>,
    descriptors::<semio_s_artifact_stdio_stl::standards::v_ascii::subsets::any::schema::snapshot::StlSnapshot, semio_s_artifact_stdio_stl::standards::v_ascii::subsets::any::schema::mutations::StlMutation>,
    descriptors::<semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::schema::snapshot::SvgSnapshot, semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::schema::mutations::SvgMutation>,
    descriptors::<semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::schema::snapshot::SvgSnapshot, semio_s_artifact_stdio_svg::standards::v1_1::subsets::basic::schema::mutations::SvgBasicMutation>,
    descriptors::<semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::schema::snapshot::SvgSnapshot, semio_s_artifact_stdio_svg::standards::v1_1::subsets::tiny::schema::mutations::SvgTinyMutation>,
    descriptors::<semio_s_artifact_stdio_tiff::standards::v6_0::subsets::document::schema::snapshot::TiffSnapshot, semio_s_artifact_stdio_tiff::standards::v6_0::subsets::document::schema::mutations::TiffMutation>,
    descriptors::<semio_s_artifact_stdio_tsv::standards::iana::subsets::any::schema::snapshot::TsvSnapshot, semio_s_artifact_stdio_tsv::standards::iana::subsets::any::schema::mutations::TsvMutation>,
    descriptors::<semio_s_artifact_stdio_txt::standards::v_utf_8::subsets::any::schema::snapshot::TxtSnapshot, semio_s_artifact_stdio_txt::standards::v_utf_8::subsets::any::schema::mutations::TxtMutation>,
    descriptors::<semio_s_artifact_stdio_wav::standards::riff_pcm::subsets::any::schema::snapshot::WavSnapshot, semio_s_artifact_stdio_wav::standards::riff_pcm::subsets::any::schema::mutations::WavMutation>,
    descriptors::<semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot, semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::base::schema::mutations::XlsxMutation>,
    descriptors::<semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot, semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::strict::schema::mutations::XlsxStrictMutation>,
    descriptors::<semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::base::schema::snapshot::XlsxSnapshot, semio_s_artifact_stdio_xlsx::standards::v_ecma_376::subsets::transitional::schema::mutations::XlsxTransitionalMutation>,
    descriptors::<semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::schema::snapshot::XmlSnapshot, semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::schema::mutations::XmlMutation>,
    descriptors::<semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::schema::snapshot::XmlSnapshot, semio_s_artifact_stdio_xml::standards::v1_0::subsets::valid::schema::valid_mutations::XmlValidMutation>,
    descriptors::<semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::schema::snapshot::ZipSnapshot, semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::schema::mutations::ZipMutation>,
    descriptors::<semio_s_artifact_stdio_zip::standards::v2_0::subsets::base::schema::snapshot::ZipSnapshot, semio_s_artifact_stdio_zip::standards::v2_0::subsets::iso21320::schema::mutations::ZipIso21320Mutation>,
];

/// 🗺️ Manifest coordinate → the owner directory whose leaves it measures.
const COORDINATES: &[(&str, &str, &str, &str)] = &[
    ("s.stdio.avi", "1.0", "hdrl", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl"),
    ("s.stdio.avi", "1.0", "idx1", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/📇️idx1"),
    ("s.stdio.avi", "1.0", "movi", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎞️movi"),
    ("s.stdio.bcf", "2.1", "markup", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/🖊️markup"),
    ("s.stdio.bcf", "2.1", "snapshot", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/📸️snapshot"),
    ("s.stdio.bcf", "2.1", "viewpoint", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🏅️standards/🔖️2.1/🪆️subsets/👁️viewpoint"),
    ("s.stdio.binary", "raw", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any"),
    ("s.stdio.bmp", "v3", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any"),
    ("s.stdio.csv", "rfc4180", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🏅️standards/🔖️rfc4180/🪆️subsets/✳️any"),
    ("s.stdio.deflate", "rfc1950", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🏅️standards/🔖️rfc1950/🪆️subsets/✳️any"),
    ("s.stdio.docx", "ecma-376", "base", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base"),
    ("s.stdio.docx", "ecma-376", "strict", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/📏️strict"),
    ("s.stdio.docx", "ecma-376", "transitional", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🔄️transitional"),
    ("s.stdio.dwg", "ac1018", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/4️⃣ac1018/🪆️subsets/✳️any"),
    ("s.stdio.dwg", "ac1024", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🏅️standards/🔟ac1024/🪆️subsets/✳️any"),
    ("s.stdio.dxf", "r12", "blocks", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/🧱️blocks"),
    ("s.stdio.dxf", "r12", "entities", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/🧩️entities"),
    ("s.stdio.dxf", "r12", "header", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📰️header"),
    ("s.stdio.dxf", "r12", "tables", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🏅️standards/🔖️r12/🪆️subsets/📊️tables"),
    ("s.stdio.epw", "energyplus", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any"),
    ("s.stdio.gif", "87a", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any"),
    ("s.stdio.gif", "89a", "base", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base"),
    ("s.stdio.gltf", "2.0", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🏅️standards/🔖️2.0/🪆️subsets/♾️any"),
    ("s.stdio.html", "5", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any"),
    ("s.stdio.ifc", "2x3", "base", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base"),
    ("s.stdio.ifc", "2x3", "cobie", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🏢️cobie"),
    ("s.stdio.ifc", "2x3", "cv20", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🤝️cv20"),
    ("s.stdio.ifc", "2x3", "sav", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧮️sav"),
    ("s.stdio.ifc", "4", "base", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any"),
    ("s.stdio.jpg", "jfif-1.01", "baseline", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧱️baseline"),
    ("s.stdio.jpg", "jfif-1.01", "document", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🏅️standards/🔖️jfif-1.01/🪆️subsets/🧾️document"),
    ("s.stdio.json", "rfc8259", "base", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base"),
    ("s.stdio.json", "rfc8259", "i-json", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🛜️i-json"),
    ("s.stdio.las", "1.0", "header", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header"),
    ("s.stdio.las", "1.0", "points", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/📍️points"),
    ("s.stdio.las", "1.0", "vlr", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🏅️standards/🔖️1.0/🪆️subsets/📼️vlr"),
    ("s.stdio.md", "commonmark", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🏅️standards/🔖️commonmark/🪆️subsets/✳️any"),
    ("s.stdio.mp3", "mpeg1-layer3", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any"),
    ("s.stdio.mp4", "isobmff", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any"),
    ("s.stdio.obj", "3.0", "geometry", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/📐️geometry"),
    ("s.stdio.obj", "3.0", "material", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🏅️standards/🔖️3.0/🪆️subsets/🎨️material"),
    ("s.stdio.pdf", "1.4", "a", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🗄️a"),
    ("s.stdio.pdf", "1.4", "base", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🧱️base"),
    ("s.stdio.pdf", "1.4", "x", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/4️⃣1.4/🪆️subsets/🖨️x"),
    ("s.stdio.pdf", "1.7", "a", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🗄️a"),
    ("s.stdio.pdf", "1.7", "base", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base"),
    ("s.stdio.pdf", "1.7", "e", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/📐️e"),
    ("s.stdio.pdf", "1.7", "h", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/⚕️h"),
    ("s.stdio.pdf", "1.7", "ua", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/♿️ua"),
    ("s.stdio.pdf", "1.7", "vt", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧾️vt"),
    ("s.stdio.pdf", "1.7", "x", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🖨️x"),
    ("s.stdio.ply", "1.0", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🏅️standards/🔖️1.0/🪆️subsets/✳️any"),
    ("s.stdio.png", "1.2", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🏅️standards/🔖️1.2/🪆️subsets/✳️any"),
    ("s.stdio.pptx", "ecma-376", "base", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base"),
    ("s.stdio.pptx", "ecma-376", "strict", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict"),
    ("s.stdio.pptx", "ecma-376", "transitional", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional"),
    ("s.stdio.semio", "v1", "animation", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation"),
    ("s.stdio.semio", "v1", "audio", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio"),
    ("s.stdio.semio", "v1", "base", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base"),
    ("s.stdio.semio", "v1", "brep", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep"),
    ("s.stdio.semio", "v1", "cad", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad"),
    ("s.stdio.semio", "v1", "document", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document"),
    ("s.stdio.semio", "v1", "drawing", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing"),
    ("s.stdio.semio", "v1", "flow", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow"),
    ("s.stdio.semio", "v1", "graph", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph"),
    ("s.stdio.semio", "v1", "image", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image"),
    ("s.stdio.semio", "v1", "kit", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit"),
    ("s.stdio.semio", "v1", "mesh", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh"),
    ("s.stdio.semio", "v1", "model", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model"),
    ("s.stdio.semio", "v1", "object", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object"),
    ("s.stdio.semio", "v1", "presentation", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation"),
    ("s.stdio.semio", "v1", "table", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table"),
    ("s.stdio.semio", "v1", "text", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text"),
    ("s.stdio.semio", "v1", "value", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value"),
    ("s.stdio.semio", "v1", "video", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video"),
    ("s.stdio.step", "ap214", "base", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base"),
    ("s.stdio.step", "ap214", "cc1", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1"),
    ("s.stdio.step", "ap214", "cc2", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2"),
    ("s.stdio.step", "ap214", "cc3", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3"),
    ("s.stdio.step", "ap214", "cc4", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4"),
    ("s.stdio.step", "ap214", "cc5", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5"),
    ("s.stdio.step", "ap214", "cc6", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6"),
    ("s.stdio.stl", "ascii", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🏅️standards/🔖️ascii/🪆️subsets/✳️any"),
    ("s.stdio.svg", "1.1", "base", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🧱️base"),
    ("s.stdio.svg", "1.1", "basic", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔰️basic"),
    ("s.stdio.svg", "1.1", "tiny", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🏅️standards/🔖️1.1/🪆️subsets/🔬️tiny"),
    ("s.stdio.tiff", "6.0", "baseline", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧱️baseline"),
    ("s.stdio.tiff", "6.0", "document", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🏅️standards/🔖️6.0/🪆️subsets/🧾️document"),
    ("s.stdio.tsv", "iana", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any"),
    ("s.stdio.txt", "utf-8", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any"),
    ("s.stdio.wav", "riff-pcm", "any", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any"),
    ("s.stdio.xlsx", "ecma-376", "base", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base"),
    ("s.stdio.xlsx", "ecma-376", "strict", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🔒️strict"),
    ("s.stdio.xlsx", "ecma-376", "transitional", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🏅️standards/🔖️ecma-376/🪆️subsets/🌉️transitional"),
    ("s.stdio.xml", "1.0", "base", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base"),
    ("s.stdio.xml", "1.0", "valid", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/✅️valid"),
    ("s.stdio.zip", "2.0", "base", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🧱️base"),
    ("s.stdio.zip", "2.0", "iso21320", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🏅️standards/🔖️2.0/🪆️subsets/🌐️iso21320"),
];

/// 🎯️ Production outcome severities as protocol outcome classes: `Info`/`Warning` ride on an applied outcome.
fn protocol_outcomes(classes: &[MutationOutcomeClass]) -> Vec<&'static str> {
    let mut seen: Vec<&'static str> = Vec::new();
    for outcome in classes {
        let mapped = match outcome {
            MutationOutcomeClass::Applied | MutationOutcomeClass::Info | MutationOutcomeClass::Warning => "applied",
            MutationOutcomeClass::Error | MutationOutcomeClass::Fatal => "rejected",
        };
        if !seen.contains(&mapped) {
            seen.push(mapped);
        }
    }
    if seen.is_empty() {
        seen.push("applied");
    }
    seen
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let [command, artifact, standard, subset] = args.as_slice() else {
        eprintln!("usage: list-mutations <artifact> <standard> <subset>");
        std::process::exit(2);
    };
    let Some((_, _, _, owner)) = COORDINATES.iter().find(|(a, s, u, _)| command == "list-mutations" && a == artifact && s == standard && u == subset) else {
        eprintln!("this bridge does not answer {command} {artifact} {standard} {subset}");
        std::process::exit(2);
    };
    let prefix = format!("{owner}/");
    let mut rows: Vec<pack::JsonValue> = Vec::new();
    let mut seen: Vec<&str> = Vec::new();
    for descriptor in AGGREGATES.iter().flat_map(|aggregate| aggregate().iter()) {
        if !descriptor.owner.starts_with(&prefix) || seen.contains(&descriptor.semantic_kind) {
            continue;
        }
        seen.push(descriptor.semantic_kind);
        rows.push(pack::json_object([
            ("id".to_string(), pack::JsonValue::from(descriptor.semantic_kind)),
            ("variant".to_string(), pack::JsonValue::from(descriptor.aggregate_variant)),
            ("outcomes".to_string(), pack::json_array(protocol_outcomes(descriptor.outcome_classes).into_iter().map(pack::JsonValue::from))),
        ]));
    }
    let out = pack::json_object([
        ("schema".to_string(), pack::JsonValue::from("semio.repository-test.runtime-inventory/v2")),
        ("artifact".to_string(), pack::JsonValue::from(artifact.as_str())),
        ("standard".to_string(), pack::JsonValue::from(standard.as_str())),
        ("subset".to_string(), pack::JsonValue::from(subset.as_str())),
        ("bridgeVersion".to_string(), pack::JsonValue::from(1_i64)),
        ("producedBy".to_string(), pack::JsonValue::from("semio-stdio-mutation-bridge")),
        ("mutations".to_string(), pack::json_array(rows)),
    ]);
    println!("{}", pack::json_to_string(&out));
}
