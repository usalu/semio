//! 🧬️ PptxSnapshot schema (ecma-376/🔒️strict) — reuses the 🧱️base subset's `PptxSnapshot` verbatim
//! (the SAME Rust type, same `s.stdio.pptx` schema id). ISO/IEC 29500-1:2016 Strict is a
//! validation-gated dialect STAMP on top of that existing schema, not a new one -- see D4's
//! Tier-1 "same snapshot type, subset moves" semantics (`ArtifactCommand::MigrateDialect`). This
//! leaf exists so `🪆️subsets/🔒️strict/🧬️schema/` is present per `🔣️taxonomy.json`'s
//! `subsetChildDirs`, without duplicating the schema definition.
//!
//! Ticket 26/08/11/ARTIFACT-STANDARD-SUBSETS-REAL-VOCABULARIES: real ISO/IEC 29500-1 Strict
//! conformance-class subset, same shared pattern as `📜️docx`/`📕️xlsx` ecma-376 🔒️strict.

pub use crate::standards::v_ecma_376::subsets::base::schema::*;

/// 🏛️ Authors the Strict presentation, master, layout, theme and their complete OPC relationships.
pub fn blank_strict_pptx_snapshot()->crate::PptxSnapshot{
 use crate::standards::v_ecma_376::subsets::base::io::{PRESENTATION_CONTENT_TYPE,SLIDE_MASTER_CONTENT_TYPE,SLIDE_LAYOUT_CONTENT_TYPE,THEME_CONTENT_TYPE};
 use semio_s_artifact_stdio_zip::opc::{OpcPackage,RELS_CONTENT_TYPE};
 use semio_framework_plugin::ArtifactBuilder;
 let mut opc=OpcPackage::empty();
 opc.content_types.set_default("rels",RELS_CONTENT_TYPE);
 opc.content_types.set_default("xml","application/xml");
 for(path,content_type,xml)in[
  ("ppt/slideMasters/slideMaster1.xml",SLIDE_MASTER_CONTENT_TYPE,r#"<p:sldMaster xmlns:a="http://purl.oclc.org/ooxml/drawingml/main" xmlns:p="http://purl.oclc.org/ooxml/presentationml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships"><p:cSld><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/></p:spTree></p:cSld><p:clrMap bg1="lt1" tx1="dk1" bg2="lt2" tx2="dk2" accent1="accent1" accent2="accent2" accent3="accent3" accent4="accent4" accent5="accent5" accent6="accent6" hlink="hlink" folHlink="folHlink"/><p:sldLayoutIdLst><p:sldLayoutId id="2147483649" r:id="rId1"/></p:sldLayoutIdLst></p:sldMaster>"#),
  ("ppt/slideLayouts/slideLayout1.xml",SLIDE_LAYOUT_CONTENT_TYPE,r#"<p:sldLayout xmlns:a="http://purl.oclc.org/ooxml/drawingml/main" xmlns:p="http://purl.oclc.org/ooxml/presentationml/main" type="blank" preserve="1"><p:cSld><p:spTree><p:nvGrpSpPr><p:cNvPr id="1" name=""/><p:cNvGrpSpPr/><p:nvPr/></p:nvGrpSpPr><p:grpSpPr/></p:spTree></p:cSld><p:clrMapOvr><a:masterClrMapping/></p:clrMapOvr></p:sldLayout>"#),
  ("ppt/theme/theme1.xml",THEME_CONTENT_TYPE,r#"<a:theme xmlns:a="http://purl.oclc.org/ooxml/drawingml/main" name="Minimal"><a:themeElements><a:clrScheme name="Minimal"><a:dk1><a:sysClr val="windowText" lastClr="000000"/></a:dk1><a:lt1><a:sysClr val="window" lastClr="FFFFFF"/></a:lt1><a:dk2><a:srgbClr val="1F497D"/></a:dk2><a:lt2><a:srgbClr val="EEECE1"/></a:lt2><a:accent1><a:srgbClr val="4F81BD"/></a:accent1><a:accent2><a:srgbClr val="C0504D"/></a:accent2><a:accent3><a:srgbClr val="9BBB59"/></a:accent3><a:accent4><a:srgbClr val="8064A2"/></a:accent4><a:accent5><a:srgbClr val="4BACC6"/></a:accent5><a:accent6><a:srgbClr val="F79646"/></a:accent6><a:hlink><a:srgbClr val="0000FF"/></a:hlink><a:folHlink><a:srgbClr val="800080"/></a:folHlink></a:clrScheme><a:fontScheme name="Minimal"><a:majorFont><a:latin typeface="Calibri"/><a:ea typeface=""/><a:cs typeface=""/></a:majorFont><a:minorFont><a:latin typeface="Calibri"/><a:ea typeface=""/><a:cs typeface=""/></a:minorFont></a:fontScheme><a:fmtScheme name="Minimal"><a:fillStyleLst><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:fillStyleLst><a:lnStyleLst><a:ln><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:ln><a:ln><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:ln><a:ln><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:ln></a:lnStyleLst><a:effectStyleLst><a:effectStyle><a:effectLst/></a:effectStyle><a:effectStyle><a:effectLst/></a:effectStyle><a:effectStyle><a:effectLst/></a:effectStyle></a:effectStyleLst><a:bgFillStyleLst><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:solidFill><a:schemeClr val="phClr"/></a:solidFill><a:solidFill><a:schemeClr val="phClr"/></a:solidFill></a:bgFillStyleLst></a:fmtScheme></a:themeElements></a:theme>"#),
  ("ppt/presentation.xml",PRESENTATION_CONTENT_TYPE,r#"<p:presentation xmlns:a="http://purl.oclc.org/ooxml/drawingml/main" xmlns:p="http://purl.oclc.org/ooxml/presentationml/main" xmlns:r="http://purl.oclc.org/ooxml/officeDocument/relationships" conformance="strict"><p:sldMasterIdLst><p:sldMasterId id="2147483648" r:id="rId1"/></p:sldMasterIdLst><p:sldIdLst/></p:presentation>"#),
 ]{opc.set_part(path,content_type,xml.as_bytes().to_vec());}
 for(owner,id,kind,target)in[
  ("ppt/slideMasters/slideMaster1.xml","rId1","http://purl.oclc.org/ooxml/officeDocument/relationships/slideLayout","../slideLayouts/slideLayout1.xml"),
  ("ppt/slideMasters/slideMaster1.xml","rId2","http://purl.oclc.org/ooxml/officeDocument/relationships/theme","../theme/theme1.xml"),
  ("ppt/slideLayouts/slideLayout1.xml","rId1","http://purl.oclc.org/ooxml/officeDocument/relationships/slideMaster","../slideMasters/slideMaster1.xml"),
  ("ppt/presentation.xml","rId1","http://purl.oclc.org/ooxml/officeDocument/relationships/slideMaster","slideMasters/slideMaster1.xml"),
  ("","rId1","http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument","ppt/presentation.xml"),
 ]{opc.add_relationship(owner,id,kind,target);}
 let bytes=semio_s_artifact_stdio_zip::opc::encode_opc_with_package_order(&opc).expect("authored Strict PPTX package");
 let snapshot=crate::standards::v_ecma_376::subsets::base::io::import::deserializers::decode_pptx(&bytes).expect("authored Strict PPTX owner");
 PptxStrictBuilderConstruction::from_snapshot(snapshot).build().expect("valid authored Strict PPTX initial owner")
}

//#region 🧬️Mutations
// 🧬️ This subset's OWN conformance-class vocabulary, mounted here rather than in the crate's shared
// `🦀️.rs`: that file is one wiring file for every stdio artifact at once, and the rationale the
// 🧱️base subset already records for its own test mount — leave the shared file alone, let an artifact
// own the subtree it owns — applies to a production leaf of this subset just as well. `#[path]` on a
// non-inline module resolves against this file's own directory. The explicit declaration shadows the
// glob re-export of 🧱️base's `mutations` above, which is what puts this subset's own vocabulary at
// `subsets::strict::schema::mutations` while 🧱️base's document vocabulary stays reachable at its own
// address.
#[path = "🧬️mutations/🦀️.rs"]
pub mod mutations;
//#endregion 🧬️Mutations

//#region 🏗️DerivedConstruction

//#endregion 🏗️DerivedConstruction

//#region 🧐️DerivedAnalysis

//#endregion 🧐️DerivedAnalysis

//#region 🧬️DerivedArtifactFacets

//#endregion 🧬️DerivedArtifactFacets
