use super::*;
use crate::standards::v1_4::subsets::base::io::binary::mutations as binary;

#[test]
fn direct_descriptor_and_catalog_bijection() {
    let kinds: Vec<_> = <PdfMutation as protocol::SemanticMutation<PdfSnapshot>>::kinds().iter().map(|descriptor| descriptor.kind).collect();
    let source = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/4️⃣1.4/🪆️subsets/🧱️base/🧬️schema/🧬️mutations");
    let catalog: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(source.join("../../🔮️oracles/🔣️.json")).unwrap()).unwrap();
    assert_eq!(catalog["mutationCatalogs"][0]["kinds"], serde_json::json!(["insert-page","remove-page","move-page","resize-page","replace-page-text"]));
    assert_eq!(catalog["mutationCatalogs"][1]["kinds"], serde_json::json!(["set-snapshot","patch-snapshot"]));
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
    for (ordinal,directory) in [(5,"📸️set-snapshot"),(6,"🩹️patch-snapshot")] {
        let descriptor:serde_json::Value=serde_json::from_str(&std::fs::read_to_string(source.join(directory).join("🔣️.json")).unwrap()).unwrap();assert_eq!(descriptor["semanticKind"],kinds[ordinal]);assert!(source.join(directory).join("🦀️.rs").is_file());
    }
}

#[test]
fn own14_aggregate_literal_set_applies_and_inverts_every_owner_word() {
    use protocol::{Mutation,MutationDiff,OpBinary};
    use crate::standards::v1_4::subsets::base::schema::snapshot::PageDoc;
    let neutral:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📸️set-patch-exact-words/🔣️.json")).unwrap();
    let base=PdfSnapshot{schema:neutral["schema"].as_str().unwrap().into(),pages:vec![PageDoc{width:0.0,height:1.0,text:"base".into()};8]};
    let expected=PdfSnapshot{schema:base.schema.clone(),pages:neutral["pages"].as_array().unwrap().iter().map(|page|PageDoc{width:f64::from_bits(page["widthBits"].as_str().unwrap().parse().unwrap()),height:f64::from_bits(page["heightBits"].as_str().unwrap().parse().unwrap()),text:page["text"].as_str().unwrap().into()}).collect()};
    let frame=|owner:&PdfSnapshot|{let mut bytes=vec![store::pack_rt::OP_BINARY_FORMAT,5];let text=|bytes:&mut Vec<u8>,text:&str|{bytes.extend_from_slice(&(text.len() as u64).to_le_bytes());bytes.extend_from_slice(text.as_bytes());};text(&mut bytes,&owner.schema);bytes.extend_from_slice(&(owner.pages.len() as u64).to_le_bytes());for p in &owner.pages{bytes.extend_from_slice(&p.width.to_bits().to_le_bytes());bytes.extend_from_slice(&p.height.to_bits().to_le_bytes());text(&mut bytes,&p.text);}bytes};
    let same=|a:&PdfSnapshot,b:&PdfSnapshot|{assert_eq!(a.schema,b.schema);assert_eq!(a.pages.len(),b.pages.len());for(a,b)in a.pages.iter().zip(&b.pages){assert_eq!(a.width.to_bits(),b.width.to_bits());assert_eq!(a.height.to_bits(),b.height.to_bits());assert_eq!(a.text,b.text);}};
    let hex=neutral["binaryFrameHex"].as_str().unwrap();let literal=(0..hex.len()).step_by(2).map(|i|u8::from_str_radix(&hex[i..i+2],16).unwrap()).collect::<Vec<_>>();assert_eq!(frame(&expected),literal);
    let decoded=PdfMutation::decode_op(&literal);assert!(decoded.is_ok(),"literal own14 tag5 must decode through existing aggregate: {decoded:?}");let op=decoded.unwrap();
    let outcome=op.diff(&base);assert!(outcome.messages().is_empty());let next=outcome.diff().apply(&base).unwrap();same(&next,&expected);
    let mut restored=next;for inverse in op.inverse(&base).unwrap(){let outcome=inverse.diff(&restored);assert!(outcome.messages().is_empty());restored=outcome.diff().apply(&restored).unwrap();}same(&restored,&base);
    let mut foreign=expected;foreign.schema="foreign".into();let rejected=PdfMutation::decode_op(&frame(&foreign)).unwrap().diff(&base);assert!(!rejected.messages().is_empty());same(&rejected.diff().apply(&base).unwrap(),&base);
    eprintln!("[DEBUG] own14 aggregate literal tag5 exact application inverse identity");
}
#[test]
fn own14_diff_signed_zero_and_sparse_nan_text_are_exact() {
    use protocol::{MutationDiff,DiffBinary,DiffCodec,DiffText};use protocol::command::DiffAlgebra;
    use crate::standards::v1_4::subsets::base::schema::snapshot::PageDoc;
    let base=PdfSnapshot{schema:"stdio.pdf".into(),pages:vec![PageDoc{width:0.0,height:1.0,text:"same".into()}]};let mut next=base.clone();next.pages[0].width=-0.0;
    assert_ne!(base,next,"own14 publication equality must distinguish owner words");
    let diff=PdfDiff::between(&base,&next);assert!(!diff.is_empty());let applied=diff.apply(&base).unwrap();assert_eq!(applied.pages[0].width.to_bits(),(-0.0f64).to_bits());let restored=diff.inverse(&base).apply(&applied).unwrap();assert_eq!(restored.pages[0].width.to_bits(),0);
    next.pages[0].width=f64::from_bits(0x7ff8000000000042);let diff=PdfDiff::between(&base,&next);let text=diff.print_diff();assert!(text.contains("nan64_7ff8000000000042"),"{text}");let parsed=PdfDiff::parse_diff(&text).unwrap();assert_eq!(parsed.apply(&base).unwrap().pages[0].width.to_bits(),0x7ff8000000000042);
    assert!(PdfDiff::between(&next,&next).is_empty(),"unchanged NaN owner word is no mutation");
    eprintln!("[DEBUG] own14 signed-zero publication equality sparse diff and payload NaN text");
}
#[test]
fn own14_aggregate_patch_applies_inverse_and_freezes_identity() {
    use protocol::{Mutation,MutationDiff,OpText};
    use crate::standards::v1_4::subsets::base::schema::snapshot::PageDoc;
    let neutral:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📸️set-patch-exact-words/🔣️.json")).unwrap();
    let same=|a:&PdfSnapshot,b:&PdfSnapshot|{assert_eq!(a.schema,b.schema);assert_eq!(a.pages.len(),b.pages.len());for(a,b)in a.pages.iter().zip(&b.pages){assert_eq!(a.width.to_bits(),b.width.to_bits());assert_eq!(a.height.to_bits(),b.height.to_bits());assert_eq!(a.text,b.text);}};
    for identity in ["stdio.pdf","pdf14.custom-schema"] {
        let base=PdfSnapshot{schema:identity.into(),pages:neutral["pages"].as_array().unwrap().iter().map(|page|PageDoc{width:f64::from_bits(page["widthBits"].as_str().unwrap().parse().unwrap()),height:f64::from_bits(page["heightBits"].as_str().unwrap().parse().unwrap()),text:page["text"].as_str().unwrap().into()}).collect()};
        let original=base.clone();let op=PdfMutation::parse_op(r#"patch-snapshot payload={"operation":"set","path":"/pages/0/text","value":"edited Ω"}"#);assert!(op.is_ok(),"existing aggregate must admit literal path patch: {op:?}");let op=op.unwrap();let outcome=op.diff(&base);assert!(outcome.messages().is_empty());let mut next=outcome.diff().apply(&base).unwrap();let mut expected=base.clone();expected.pages[0].text="edited Ω".into();same(&next,&expected);same(&base,&original);
        for inverse in op.inverse(&base).unwrap(){let outcome=inverse.diff(&next);assert!(outcome.messages().is_empty());next=outcome.diff().apply(&next).unwrap();}same(&next,&base);
        for literal in [r#"patch-snapshot payload={"operation":"set","path":"/pages/0/width","value":"wrong typed field"}"#,r#"patch-snapshot payload={"operation":"set","path":"/schema","value":"foreign"}"#] {
            let invalid=PdfMutation::parse_op(literal).unwrap();let refused=invalid.diff(&base);assert!(!refused.messages().is_empty());same(&refused.diff().apply(&base).unwrap(),&base);assert!(invalid.inverse(&base).is_err());
        }
    }
    eprintln!("[DEBUG] own14 neutral path patch preserves eight words canonical/custom identity typed refusal and inverse");
}

#[test]
fn own14_replacement_and_patch_preserve_literal_binary64_words_and_frozen_identity() {
    use protocol::{MutationKind,OpBinary,OpText};
    use crate::standards::v1_4::subsets::base::schema::snapshot::PageDoc;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📸️set-patch-exact-words/🔣️.json")).unwrap();
    let source=PdfSnapshot{schema:fixture["schema"].as_str().unwrap().into(),pages:fixture["pages"].as_array().unwrap().iter().map(|row|PageDoc{width:f64::from_bits(row["widthBits"].as_str().unwrap().parse().unwrap()),height:f64::from_bits(row["heightBits"].as_str().unwrap().parse().unwrap()),text:row["text"].as_str().unwrap().into()}).collect()};
    let same=|left:&PdfSnapshot,right:&PdfSnapshot|{assert_eq!(left.schema,right.schema);assert_eq!(left.pages.len(),right.pages.len());for(a,b)in left.pages.iter().zip(&right.pages){assert_eq!(a.width.to_bits(),b.width.to_bits());assert_eq!(a.height.to_bits(),b.height.to_bits());assert_eq!(a.text,b.text);}};
    let mutation=PdfMutation::SetSnapshot(SetSnapshot{snapshot:source.clone()});let bytes=mutation.encode_op().unwrap();let hex=bytes.iter().map(|byte|format!("{byte:02x}")).collect::<String>();assert_eq!(hex,fixture["binaryFrameHex"].as_str().unwrap());
    let PdfMutation::SetSnapshot(restored)=PdfMutation::decode_op(&bytes).unwrap()else{panic!("set payload")};same(&restored.snapshot,&source);
    let text=mutation.print_op();assert!(text.contains("nan64_7ff8000000000042"));assert!(text.contains("nan64_7ff0000000000001"));let PdfMutation::SetSnapshot(restored)=PdfMutation::parse_op(&text).unwrap()else{panic!("set payload")};same(&restored.snapshot,&source);
    let base=PdfSnapshot{schema:source.schema.clone(),pages:vec![PageDoc{width:1.0,height:1.0,text:"base".into()}]};let inverse=SetSnapshot{snapshot:source.clone()}.inverse(&base).unwrap();assert_eq!(inverse.len(),1);let PdfMutation::SetSnapshot(original)=&inverse[0]else{panic!("inverse payload")};same(&original.snapshot,&base);
    let patch=PatchSnapshot{patch:semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set{path:"/pages/0/text".into(),value:semio_framework_value::DslValue::String("edited Ω\0".into())}};
    let next=semio_s_artifact_stdio_contract::editing::apply_snapshot_patch(&source,&patch.patch).unwrap();assert_eq!(next.pages[0].text,"edited Ω\0");for(a,b)in next.pages.iter().zip(&source.pages){assert_eq!(a.width.to_bits(),b.width.to_bits());assert_eq!(a.height.to_bits(),b.height.to_bits());}
    let refused=PatchSnapshot{patch:semio_s_artifact_stdio_contract::editing::SnapshotPatch::Set{path:"/schema".into(),value:semio_framework_value::DslValue::String("foreign\0schema".into())}};assert!(refused.inverse(&source).is_err());
    assert_eq!(binary::set_snapshot::TAG,5);assert_eq!(binary::patch_snapshot::TAG,6);
    eprintln!("[DEBUG] own14 full raw-word Set/patch domain inverse identity and exact literal frame");
}

#[test]
fn paged_pdf14_original_source_preserves_seven_frames_raw_words_and_fixed_page_grants(){
    use protocol::{OpBinary,operation_bytes::{OwnedOperationBytes,OperationByteMeasurement,OperationBytePreparation,OperationByteCloseStep}};
    use semio_framework_value::{NativeEncodeControl,ValueRefusalKind};
    use crate::standards::v1_4::subsets::base::schema::snapshot::PageDoc;
    let neutral:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🫳️operation-source/🔣️.json")).unwrap();
    let index=|case:&serde_json::Value,key:&str|usize::try_from(case[key].as_u64().unwrap()).unwrap();
    let word=|case:&serde_json::Value,key:&str|f64::from_bits(case[key].as_str().unwrap().parse().unwrap());
    let mut cases=Vec::new();
    for case in neutral["cases"].as_array().unwrap(){
        let operation=match case["kind"].as_str().unwrap(){
            "insert-page"=>PdfMutation::InsertPage(InsertPage{index:index(case,"index"),page:PageDoc{width:word(case,"widthBits"),height:word(case,"heightBits"),text:case["text"].as_str().unwrap().into()}}),
            "remove-page"=>PdfMutation::RemovePage(RemovePage{index:index(case,"index")}),
            "move-page"=>PdfMutation::MovePage(MovePage{from:index(case,"from"),to:index(case,"to")}),
            "resize-page"=>PdfMutation::ResizePage(ResizePage{index:index(case,"index"),width:word(case,"widthBits"),height:word(case,"heightBits")}),
            "replace-page-text"=>PdfMutation::ReplacePageText(ReplacePageText{index:index(case,"index"),text:case["text"].as_str().unwrap().into()}),
            "set-snapshot"=>PdfMutation::SetSnapshot(SetSnapshot{snapshot:PdfSnapshot{schema:case["identity"].as_str().unwrap().into(),pages:case["pages"].as_array().unwrap().iter().map(|page|PageDoc{width:word(page,"widthBits"),height:word(page,"heightBits"),text:page["text"].as_str().unwrap().into()}).collect()}}),
            "patch-snapshot"=>PdfMutation::PatchSnapshot(PatchSnapshot{patch:semio_s_artifact_stdio_contract::editing::SnapshotPatch::Remove{path:case["patch"]["path"].as_str().unwrap().into()}}),
            kind=>panic!("undeclared literal kind {kind}"),
        };
        let hex=case["frameHex"].as_str().unwrap();let literal=(0..hex.len()).step_by(2).map(|index|u8::from_str_radix(&hex[index..index+2],16).unwrap()).collect::<Vec<_>>();
        assert_eq!(literal[1],case["tag"].as_u64().unwrap()as u8);assert_eq!(operation.encode_op().unwrap(),literal);cases.push((operation,literal));
    }
    assert_eq!(cases.len(),binary::REGISTRY.len());
    let large=PdfMutation::ReplacePageText(ReplacePageText{index:0,text:"x".repeat(neutral["payloadBytes"].as_u64().unwrap()as usize)});let literal=large.encode_op().unwrap();cases.push((large,literal));
    let allocation=neutral["allocationBytes"].as_u64().unwrap()as usize;let items=neutral["maximumCloseItems"].as_u64().unwrap()as usize;let bytes=neutral["maximumCloseBytes"].as_u64().unwrap()as usize;
    let close=|owner:&mut OwnedOperationBytes|{let allocation=owner.allocated_bytes();let turns=owner.len()+256;let mut released=0;for _ in 0..turns{match owner.close_one(items,bytes).unwrap(){OperationByteCloseStep::Complete=>break,OperationByteCloseStep::Pending{released_items,released_bytes}=>{assert!(released_items<=items);assert!(released_bytes<=bytes);released+=released_bytes;}}}assert!(owner.terminal_is_empty());assert_eq!(owner.allocated_bytes(),0);assert_eq!(released,allocation);};
    let refusal=|error:protocol::ProtocolError|match error{protocol::ProtocolError::Pack(protocol::PackError::Refusal(refusal))=>refusal.kind(),other=>panic!("expected exact typed policy refusal: {other:?}")};
    for(operation,literal)in &cases{
        let mut options=protocol::codec::PackEncodeOptions::default();options.limits.max_file_len=literal.len()as u64;
        let mut measure=OperationByteMeasurement::new(options.limits.max_file_len);let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);operation.encode_op_into(&options,&mut measure,&mut control).unwrap();assert_eq!(control.owned_bytes(),0);assert_eq!(measure.exact_length().unwrap(),literal.len());
        let mut preparation=OperationBytePreparation::try_new(literal.len(),allocation).unwrap();let mut accept=|_|true;let mut control=NativeEncodeControl::new(allocation,&mut accept);for _ in 0..literal.len()+256{preparation.fund_one(items,bytes,&mut control).unwrap();if preparation.is_funded(){break;}}assert!(preparation.is_funded());let charged=control.owned_bytes();let backing=preparation.allocated_bytes();operation.encode_op_into(&options,&mut preparation,&mut control).unwrap();assert_eq!(charged,control.owned_bytes());assert_eq!(backing,preparation.allocated_bytes());let mut owner=preparation.take_ready().unwrap();assert!(owner.iter().eq(literal.iter().copied()));assert_eq!(serde_json::to_value(&owner).unwrap(),serde_json::to_value(literal).unwrap());assert_eq!(owner.close_one(0,bytes).unwrap(),OperationByteCloseStep::Pending{released_items:0,released_bytes:0});assert_eq!(owner.len(),literal.len());close(&mut owner);
        options.limits.max_file_len-=1;let mut prefix=OwnedOperationBytes::try_new(literal.len(),allocation).unwrap();let mut accept=|_|true;let mut control=NativeEncodeControl::new(allocation,&mut accept);assert_eq!(refusal(operation.encode_op_into(&options,&mut prefix,&mut control).unwrap_err()),ValueRefusalKind::OwnershipLimit);assert!(prefix.len()<literal.len());assert!(prefix.iter().eq(literal[..prefix.len()].iter().copied()));close(&mut prefix);
    }
    let(operation,literal)=cases.last().unwrap();let options=protocol::codec::PackEncodeOptions::default();let mut prefix=OwnedOperationBytes::try_new(literal.len(),allocation).unwrap();let mut cancel=|progress:semio_framework_value::native_encoding::NativeEncodeProgress|progress.completed<neutral["cancelAt"].as_u64().unwrap()as usize;let mut control=NativeEncodeControl::new(allocation,&mut cancel);assert_eq!(refusal(operation.encode_op_into(&options,&mut prefix,&mut control).unwrap_err()),ValueRefusalKind::Canceled);assert!(prefix.iter().eq(literal[..prefix.len()].iter().copied()));close(&mut prefix);
    for key in ["max_items","max_depth"]{let mut options=protocol::codec::PackEncodeOptions::default();if key=="max_items"{options.limits.max_items=0;}else{options.limits.max_depth=0;}let mut measure=OperationByteMeasurement::new(options.limits.max_file_len);let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);let kind=refusal(cases[5].0.encode_op_into(&options,&mut measure,&mut control).unwrap_err());assert_eq!(kind,if key=="max_items"{ValueRefusalKind::WorkLimit}else{ValueRefusalKind::DepthLimit});}
    let mut options=protocol::codec::PackEncodeOptions::default();options.limits.max_total_alloc=0;let mut owner=OwnedOperationBytes::try_new(literal.len(),allocation).unwrap();let mut accept=|_|true;let mut control=NativeEncodeControl::new(allocation,&mut accept);assert_eq!(refusal(operation.encode_op_into(&options,&mut owner,&mut control).unwrap_err()),ValueRefusalKind::OwnershipLimit);assert_eq!(owner.len(),0);assert_eq!(control.owned_bytes(),0);assert_eq!(control.maximum_bytes(),allocation);close(&mut owner);
    let invalid=PdfMutation::InsertPage(InsertPage{index:0,page:PageDoc{width:f64::NAN,height:1.0,text:String::new()}});assert!(invalid.encode_op().is_err());let options=protocol::codec::PackEncodeOptions::default();let mut measure=OperationByteMeasurement::new(options.limits.max_file_len);let mut accept=|_|true;let mut control=NativeEncodeControl::new(0,&mut accept);assert!(matches!(invalid.encode_op_into(&options,&mut measure,&mut control),Err(protocol::ProtocolError::Malformed{what:"PDF 1.4 mutation",..})));
    eprintln!("[DEBUG] PDF1.4 seven neutral literal frames, NaN payload/signed-zero words, actual8194 text, same-control caller ceilings and exact physical4096 page retirement");
}
