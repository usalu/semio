use crate::standards::v1_4::subsets::base::schema::diff::PdfDiff;
use crate::standards::v1_4::subsets::base::schema::snapshot::{PageDoc, PdfSnapshot};
use semio_framework_test_fixtures::{fixture_path, fixture_text};
use semio_framework_value::DslValue;
use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;
#[allow(unused_imports)]
use semio_plugin::{Fault, FaultCode, FaultOrigin};
#[allow(unused_imports)]
use semio_framework_value::FromValue;
#[allow(unused_imports)]
use semio_s_artifact_stdio_contract::editing::{Carried, EditRules, EntityRule, InsertRule, RemoveRule, Selector};
use crate::standards::v1_4::subsets::base::schema::mutations::*;
use protocol::ColdDiffAlgebra;

#[test]
fn direct_descriptor_and_catalog_bijection() {
    let kinds: Vec<_> = <PdfMutation as protocol::SemanticMutation<PdfSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
    let source = fixture_path!("../../../../🧱️base/🧬️schema/🧬️mutations");
    let catalog: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("../../🔮️oracles/🔣️.json")).unwrap()).unwrap();
    assert_eq!(catalog["mutationCatalogs"][0]["kinds"], serde_json::json!(["insert-page","remove-page","move-page","resize-page","replace-page-text"]));
    let declared:Vec<_>=catalog["mutationCatalogs"].as_array().unwrap().iter().flat_map(|catalog|catalog["kinds"].as_array().unwrap().iter().map(|kind|kind.as_str().unwrap())).collect();
    assert_eq!(declared,kinds);
    let unique:std::collections::BTreeSet<_>=declared.iter().copied().collect();assert_eq!(unique.len(),declared.len());
    {
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("📥️insert-page").join("🔣️.json")).unwrap()).unwrap();
        assert_eq!(descriptor["semanticKind"], kinds[0]);
        assert!(source.join("📥️insert-page").join("🦀️.rs").is_file());
    }
    {
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("🗑️remove-page").join("🔣️.json")).unwrap()).unwrap();
        assert_eq!(descriptor["semanticKind"], kinds[1]);
        assert!(source.join("🗑️remove-page").join("🦀️.rs").is_file());
    }
    {
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("🔀️move-page").join("🔣️.json")).unwrap()).unwrap();
        assert_eq!(descriptor["semanticKind"], kinds[2]);
        assert!(source.join("🔀️move-page").join("🦀️.rs").is_file());
    }
    {
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("📐️resize-page").join("🔣️.json")).unwrap()).unwrap();
        assert_eq!(descriptor["semanticKind"], kinds[3]);
        assert!(source.join("📐️resize-page").join("🦀️.rs").is_file());
    }
    {
        let descriptor: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("♻️replace-page-text").join("🔣️.json")).unwrap()).unwrap();
        assert_eq!(descriptor["semanticKind"], kinds[4]);
        assert!(source.join("♻️replace-page-text").join("🦀️.rs").is_file());
    }
}

#[test]
fn own14_diff_signed_zero_and_sparse_nan_text_are_exact() {
    use protocol::{MutationDiff,DiffBinary,DiffCodec,DiffText};use protocol::command::DiffAlgebra;
    use crate::standards::v1_4::subsets::base::schema::snapshot::PageDoc;
    let base=PdfSnapshot{schema:"stdio.pdf".into(),pages:vec![PageDoc{width:0.0,height:1.0,text:"same".into()}]};let mut next=base.clone();next.pages[0].width=-0.0;
    assert_ne!(base,next,"own14 publication equality must distinguish owner words");
    let width_diff=|width:f64|PdfDiff {pages:Some(crate::standards::v1_4::subsets::base::schema::diff::PdfPagesDiff {modified:vec![crate::standards::v1_4::subsets::base::schema::diff::PdfPageModified {index:0,diff:crate::standards::v1_4::subsets::base::schema::diff::PdfPageDiff {width:Some(width),..Default::default()}}],..Default::default()})};
    let diff=width_diff(-0.0);assert!(!diff.is_empty());let applied=protocol::apply_diff_cold(&diff, &base).unwrap();assert_eq!(applied.pages[0].width.to_bits(),(-0.0f64).to_bits());let restored=protocol::apply_diff_cold(&diff.inverse(&base), &applied).unwrap();assert_eq!(restored.pages[0].width.to_bits(),0);
    next.pages[0].width=f64::from_bits(0x7ff8000000000042);let diff=width_diff(f64::from_bits(0x7ff8000000000042));let text=diff.print_diff();assert!(text.contains("nan64_7ff8000000000042"),"{text}");let parsed=PdfDiff::parse_diff(&text).unwrap();assert_eq!(protocol::apply_diff_cold(&parsed, &base).unwrap().pages[0].width.to_bits(),0x7ff8000000000042);
    assert!(PdfDiff::default().is_empty(),"unchanged NaN owner word is no mutation");
}
#[test]
fn pdf14_operations_round_trip_through_the_pack_document_with_exact_float_words(){
    use protocol::OpBinary;
    use crate::standards::v1_4::subsets::base::schema::snapshot::PageDoc;
    let neutral:serde_json::Value=serde_json::from_str(fixture_text!("../../../../🧱️base/🧪️tests/🧫️fixtures/🧬️mutations/🫳️operation-source/🔣️.json")).unwrap();
    let index=|case:&serde_json::Value,key:&str|usize::try_from(case[key].as_u64().unwrap()).unwrap();
    let word=|case:&serde_json::Value,key:&str|f64::from_bits(case[key].as_str().unwrap().parse().unwrap());
    let mut count=0;
    for case in neutral["cases"].as_array().unwrap(){
        let operation=match case["kind"].as_str().unwrap(){
            "insert-page"=>PdfMutation::InsertPage(InsertPage{index:index(case,"index"),page:PageDoc{width:word(case,"widthBits"),height:word(case,"heightBits"),text:case["text"].as_str().unwrap().into()}}),
            "remove-page"=>PdfMutation::RemovePage(RemovePage{index:index(case,"index")}),
            "move-page"=>PdfMutation::MovePage(MovePage{from:index(case,"from"),to:index(case,"to")}),
            "resize-page"=>PdfMutation::ResizePage(ResizePage{index:index(case,"index"),width:word(case,"widthBits"),height:word(case,"heightBits")}),
            "replace-page-text"=>PdfMutation::ReplacePageText(ReplacePageText{index:index(case,"index"),text:case["text"].as_str().unwrap().into()}),
            kind=>panic!("undeclared literal kind {kind}"),
        };
        let bytes=operation.encode_op().unwrap();
        let decoded=PdfMutation::decode_op(&bytes).unwrap();
        assert_eq!(decoded.encode_op().unwrap(),bytes);
        match(&operation,&decoded){
            (PdfMutation::InsertPage(left),PdfMutation::InsertPage(right))=>{assert_eq!(left.page.width.to_bits(),right.page.width.to_bits());assert_eq!(left.page.height.to_bits(),right.page.height.to_bits());}
            (PdfMutation::ResizePage(left),PdfMutation::ResizePage(right))=>{assert_eq!(left.width.to_bits(),right.width.to_bits());assert_eq!(left.height.to_bits(),right.height.to_bits());}
            _=>assert_eq!(operation,decoded),
        }
        count+=1;
    }
    assert_eq!(count,5);
}

fn resolved(event: &semio_s_artifact_stdio_contract::editing::SnapshotEditEvent, base: &PdfSnapshot) -> Result<Vec<PdfMutation>, String> {
    if let Some(leaves) = special_edit(event, base).map_err(|fault| format!("{fault:?}"))? {
        return Ok(leaves);
    }
    EDIT_RULES.resolve::<PdfSnapshot, PdfMutation>(base, event).map_err(|error| error.to_string())
}

#[test]
fn a_details_edit_dispatches_the_concrete_page_kind_it_names() {
    use crate::standards::v1_4::subsets::base::schema::snapshot::PageDoc;
    use semio_framework_value::{DslValue, ToValue};
    use semio_s_artifact_stdio_contract::editing::SnapshotEditEvent;
    let base = PdfSnapshot { pages: vec![PageDoc { width: 612.0, height: 792.0, text: "first".to_string() }, PageDoc { width: 100.0, height: 200.0, text: "middle".to_string() }, PageDoc { width: 300.0, height: 400.0, text: "last".to_string() }], ..Default::default() };
    let set = |path: &str, value: DslValue| resolved(&SnapshotEditEvent::SetValue { path: path.to_string(), value }, &base).expect("the edit names a page kind");
    assert_eq!(set("/pages/1/text", "edited".to_string().to_value()), vec![PdfMutation::ReplacePageText(ReplacePageText { index: 1, text: "edited".to_string() })]);
    assert_eq!(set("/pages/2/width", 10.0f64.to_value()), vec![PdfMutation::ResizePage(ResizePage { index: 2, width: 10.0, height: 400.0 })]);
    assert_eq!(set("/pages/2", PageDoc { width: 10.0, height: 20.0, text: "last".to_string() }.to_value()), vec![PdfMutation::ResizePage(ResizePage { index: 2, width: 10.0, height: 20.0 })]);
    let page = PageDoc::new(1.0, 2.0);
    assert_eq!(resolved(&SnapshotEditEvent::InsertValue { path: "/pages/1".to_string(), value: page.to_value() }, &base).unwrap(), vec![PdfMutation::InsertPage(InsertPage { index: 1, page })]);
    assert_eq!(resolved(&SnapshotEditEvent::RemoveValue { path: "/pages/1".to_string() }, &base).unwrap(), vec![PdfMutation::RemovePage(RemovePage { index: 1 })]);
    assert_eq!(resolved(&SnapshotEditEvent::MoveValue { from: "/pages/0".to_string(), path: "/pages/2".to_string() }, &base).unwrap(), vec![PdfMutation::MovePage(MovePage { from: 0, to: 2 })]);
    assert!(resolved(&SnapshotEditEvent::SetValue { path: "/schema".to_string(), value: "x".to_string().to_value() }, &base).is_err());
}
