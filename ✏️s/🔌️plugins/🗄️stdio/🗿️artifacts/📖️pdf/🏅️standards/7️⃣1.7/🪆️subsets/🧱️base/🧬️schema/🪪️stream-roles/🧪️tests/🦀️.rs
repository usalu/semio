use super::*;
use crate::standards::v1_7::subsets::base::{io::{decode_pdf, encode_pdf, text_document}, modules::lift::lift_document_with, schema::graph_source::GraphSource};

fn role() -> PdfAdmittedStreamRole {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let owner = ObjRef { num: fixture["role"]["identity"]["owner"]["num"].as_u64().unwrap() as u32, gen: fixture["role"]["identity"]["owner"]["gen"].as_u64().unwrap() as u16 };
    let dependency = &fixture["role"]["dependencies"][1]["owner"];
    PdfAdmittedStreamRole { identity: PdfGraphIdentity { owner, path: vec![PdfGraphPath::Entry { key: "Contents".into() }] }, dependencies: vec![PdfGraphIdentity { owner, path: vec![PdfGraphPath::Entry { key: "Contents".into() }] }, PdfGraphIdentity { owner: ObjRef { num: dependency["num"].as_u64().unwrap() as u32, gen: dependency["gen"].as_u64().unwrap() as u16 }, path: Vec::new() }], value: PdfStreamRoleValue::Operators { content: vec![PdfOp::Save, PdfOp::Restore] } }
}

#[test]
fn pdf_stream_roles_require_exact_generation_and_explicit_replacement() {
    let role = role();
    let stream = PdfObject::Stream { dict: Vec::new(), data: b"q Q".to_vec(), filters: Vec::new() };
    let mut base = vec![PdfIndirectObject { id: role.dependencies[1].owner, value: stream }, PdfIndirectObject { id: role.identity.owner, value: PdfObject::Dict(vec![crate::standards::v1_7::subsets::base::schema::snapshot::PdfDictEntry::new("Contents", PdfObject::Ref(role.dependencies[1].owner))]) }];
    assert!(resolve_identity(&base, &PdfGraphIdentity { owner: ObjRef { gen: 3, ..role.dependencies[1].owner }, path: Vec::new() }).is_none());
    assert_eq!(page_content(std::slice::from_ref(&role), role.identity.owner), Some([PdfOp::Save, PdfOp::Restore].as_slice()));
    let mut next = base.clone();
    let PdfObject::Stream { data, .. } = &mut next[0].value else { unreachable!() }; data.push(b'q');
    assert!(validate_role_inputs(&base, &next, std::slice::from_ref(&role), &[]).is_err());
    assert!(validate_role_inputs(&base, &next, std::slice::from_ref(&role), std::slice::from_ref(&role)).is_ok());
    let mut without_consumption=base.clone();without_consumption[1].value=PdfObject::Dict(Vec::new());
    assert!(validate_role_transition(&base,&without_consumption,std::slice::from_ref(&role),&[],&[]).is_ok());
    let PdfObject::Stream {data,..}=&mut without_consumption[0].value else {unreachable!()};data.push(0);
    assert!(validate_role_transition(&base,&without_consumption,std::slice::from_ref(&role),&[],&[]).is_err());
    base[0].id.gen += 1;
    assert!(validate_role_inputs(&next, &base, std::slice::from_ref(&role), std::slice::from_ref(&role)).is_err());
    println!("[DEBUG] PDF stream role exact identity, pure operators and replacement refusal confirmed");
}

#[test]
fn pdf_stream_roles_native_page_lift_records_semantic_operators() {
    let snapshot = decode_pdf(&encode_pdf(&text_document(&[(200.0, 300.0, "role")])).unwrap()).unwrap();
    let mut source = GraphSource::new(&snapshot.objects);
    let lifter = lift_document_with(&snapshot.trailer, &snapshot.declared_version, &mut source);
    assert_eq!(page_content(&lifter.admitted_stream_roles, lifter.page_refs[0]), Some(snapshot.pages[0].content.as_slice()));
    assert!(!lifter.admitted_stream_roles[0].dependencies.is_empty());
    println!("[DEBUG] PDF native page lift emitted identity-bound semantic operators");
}

#[test]
fn pdf_stream_roles_pure_projection_replaces_operators_and_preserves_inverse() {
    use crate::standards::v1_7::subsets::base::schema::{graph_projection::project_graph, mutations::{PdfMutation, set_object_value::SetObjectValue}, diff::{PdfIndexedDiff, PdfIndexedItem}};
    use protocol::{Mutation, DiffAlgebra};
    let native = encode_pdf(&text_document(&[(200.0, 300.0, "role")])).unwrap();
    let base = decode_pdf(&native).unwrap();
    assert_eq!(semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_page_count(&native).unwrap(), base.pages.len());
    assert_eq!(project_graph(&base).unwrap().pages, base.pages);
    let (index, role) = base.admitted_stream_roles.iter().enumerate().find(|(_, role)| matches!(role.value, PdfStreamRoleValue::Operators { .. })).unwrap();
    let dependency = role.dependencies.iter().find(|dependency| dependency.path.is_empty() && base.objects.iter().any(|object| object.id == dependency.owner && matches!(object.value,PdfObject::Stream { .. }))).unwrap();
    let mut object = base.objects.iter().find(|object| object.id == dependency.owner).unwrap().value.clone();
    let PdfObject::Stream { data, .. } = &mut object else { unreachable!() }; *data = b"q Q".to_vec();
    let refused = PdfMutation::SetObjectValue(SetObjectValue { id: dependency.owner, value: object.clone(), index: None, admitted_stream_roles: None }).diff(&base);
    assert!(protocol::apply_diff(refused.diff(), &base).is_err());
    let mut replacement = role.clone(); replacement.value = PdfStreamRoleValue::Operators { content: vec![PdfOp::Save, PdfOp::Restore] };
    let operation = PdfMutation::SetObjectValue(SetObjectValue { id: dependency.owner, value: object, index: None, admitted_stream_roles: Some(PdfIndexedDiff { modified: vec![PdfIndexedItem { index, value: replacement }], ..PdfIndexedDiff::default() }) });
    let outcome = operation.diff(&base);
    let next = protocol::apply_diff(outcome.diff(), &base).unwrap();
    assert_eq!(next.pages[0].content, vec![PdfOp::Save, PdfOp::Restore]);
    assert_eq!(protocol::apply_diff(&outcome.diff().inverse(&base), &next).unwrap(), base);
    println!("[DEBUG] PDF lopdf page count, pure projection, native-role refusal and exact diff inverse confirmed");
}

#[test]
fn pdf_stream_roles_direct_alias_admission_binds_every_structural_owner() {
    use crate::standards::v1_7::subsets::base::schema::{graph_source::ObjectSource, snapshot::PdfDictEntry};
    let value = PdfObject::Str(vec![0,127,255]);
    let objects: Vec<_> = [ObjRef {num:11,gen:1},ObjRef {num:12,gen:2}].into_iter().map(|id| PdfIndirectObject {id,value:PdfObject::Dict(vec![PdfDictEntry::new("Lookup",value.clone())])}).collect();
    let mut source=GraphSource::new(&objects);
    source.record_role(&value,PdfStreamRoleValue::PaletteComponents {components:vec![0,127,255]});
    assert_eq!(source.admitted_roles().len(),2);
    assert_ne!(source.admitted_roles()[0].identity,source.admitted_roles()[1].identity);
    assert!(validate_role_inputs(&objects,&objects,&[],source.admitted_roles()).is_ok());
    let mut roles=source.admitted_roles().to_vec();
    roles[1].value=PdfStreamRoleValue::PaletteComponents {components:vec![255,127,0]};
    let mut ambiguous=GraphSource::with_roles(&objects,&roles);
    assert!(ambiguous.semantic_role(&value,PdfStreamRoleKind::PaletteComponents).is_none());
    assert_eq!(ambiguous.missing_roles.len(),1);
    println!("[DEBUG] PDF equal direct palettes bind every generation/path and refuse conflicting semantic aliases");
}

#[test]
fn pdf_stream_roles_native_colour_admission_uses_components_and_foreign_references() {
    use crate::standards::v1_7::subsets::base::{modules::colour::{lift_colour_space,lift_shading},schema::{graph_source::ObjectSource,snapshot::{PdfDictEntry,PdfColorSpace,PdfShadingKind}}};
    let reference=|num|ObjRef {num,gen:2};
    let palette=vec![0,127,255];
    let profile=include_bytes!("../../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🌈️x-vt-initial/🌈️sRGB2014.icc").to_vec();
    let mesh=vec![0,128,255];
    let objects=vec![
        PdfIndirectObject {id:reference(5),value:PdfObject::Stream {dict:vec![PdfDictEntry::new("N",PdfObject::Int(3))],data:profile.clone(),filters:Vec::new()}},
        PdfIndirectObject {id:reference(6),value:PdfObject::Stream {dict:Vec::new(),data:palette.clone(),filters:Vec::new()}},
        PdfIndirectObject {id:reference(7),value:PdfObject::Stream {dict:vec![PdfDictEntry::new("ShadingType",PdfObject::Int(4)),PdfDictEntry::new("ColorSpace",PdfObject::name("DeviceRGB"))],data:mesh.clone(),filters:Vec::new()}}
    ];
    let mut source=GraphSource::new(&objects);
    let icc=lift_colour_space(&PdfObject::Array(vec![PdfObject::name("ICCBased"),PdfObject::Ref(reference(5))]),&mut source);
    let indexed=lift_colour_space(&PdfObject::Array(vec![PdfObject::name("Indexed"),PdfObject::name("DeviceRGB"),PdfObject::Int(0),PdfObject::Ref(reference(6))]),&mut source);
    let shading=lift_shading("mesh",&PdfObject::Ref(reference(7)),&mut source).unwrap();
    let PdfColorSpace::IccBased {profile:profile_reference,..}=icc.clone() else {panic!("ICC reference")};
    assert_eq!(profile_reference.dialect.artifact_kind,"s.stdio.binary");
    assert!(profile_reference.artifact_id.starts_with("pdf:s.stdio.icc:sha256:"));
    let PdfColorSpace::Indexed {palette:components,..}=indexed else {panic!("logical palette")}; assert_eq!(components,palette);
    let PdfShadingKind::Mesh {reference:mesh_reference,..}=shading.kind else {panic!("mesh reference")};
    assert!(mesh_reference.artifact_id.starts_with("pdf:s.stdio.pdf-mesh:sha256:"));
    let roles=source.admitted_roles(); assert_eq!(roles.len(),3);
    assert!(validate_role_inputs(&objects,&objects,&[],roles).is_ok());
    assert!(roles.iter().any(|role|matches!(&role.value,PdfStreamRoleValue::PaletteComponents {components} if components==&palette)));
    let mut document=text_document(&[(100.0,100.0,"profile")]);
    document.objects=objects.clone();
    document.color_spaces.push(crate::standards::v1_7::subsets::base::schema::snapshot::PdfNamedColorSpace {name:"Profile".into(),color_space:icc});
    let native=encode_pdf(&document).unwrap();
    assert!(semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_profile_streams(&native).unwrap().contains(&profile));
    let retained=crate::standards::v1_7::subsets::base::io::foreign_artifacts::NativePdfArtifactResources::from_objects(&objects);
    use crate::standards::v1_7::subsets::base::io::foreign_artifacts::PdfArtifactResourcePort;
    assert_eq!(retained.resolve(&profile_reference).unwrap(),objects[0].value);
    assert_eq!(retained.resolve(&mesh_reference).unwrap(),objects[2].value);
    println!("[DEBUG] PDF native ICC/mesh reference admission and Indexed components match independent lopdf stream custody");
}

#[test]
fn pdf_stream_roles_role_only_projection_and_insert_remove_preserve_replay() {
    use crate::standards::v1_7::subsets::base::schema::{mutations::{PdfMutation,InsertObject},diff::{PdfDiff,PdfIndexedDiff,PdfIndexedItem}};
    use protocol::{Mutation,DiffAlgebra};
    let base=decode_pdf(&encode_pdf(&text_document(&[(200.0,300.0,"role")])).unwrap()).unwrap();
    let (index,role)=base.admitted_stream_roles.iter().enumerate().find(|(_,role)|matches!(role.value,PdfStreamRoleValue::Operators {..})).unwrap();
    let mut changed=role.clone();changed.value=PdfStreamRoleValue::Operators {content:vec![PdfOp::Save,PdfOp::Restore]};
    let diff=PdfDiff {admitted_stream_roles:Some(PdfIndexedDiff {modified:vec![PdfIndexedItem {index,value:changed}],..Default::default()}),..Default::default()};
    let next=protocol::apply_diff(&diff,&base).unwrap();assert_eq!(next.pages[0].content,vec![PdfOp::Save,PdfOp::Restore]);
    let replay:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🪢️replay/🔣️.json")).unwrap();
    let expected:Vec<String>=serde_json::from_value(replay["nativeOperatorNames"].clone()).unwrap();
    let native=encode_pdf(&next).unwrap();
    assert_eq!(semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_page_operator_names(&native).unwrap(),vec![expected]);
    assert_eq!(decode_pdf(&native).unwrap().pages[0].content,next.pages[0].content);
    assert_eq!(protocol::apply_diff(&diff.inverse(&base),&next).unwrap(),base);
    let owner=ObjRef {num:500,gen:7};
    let identity=PdfGraphIdentity {owner,path:Vec::new()};
    let added=PdfAdmittedStreamRole {identity:identity.clone(),dependencies:vec![identity],value:PdfStreamRoleValue::MetadataText {text:"retained fixture".into()}};
    let insert=PdfMutation::InsertObject(InsertObject {id:owner,value:PdfObject::Stream {dict:Vec::new(),data:b"retained fixture".to_vec(),filters:Vec::new()},index:None,admitted_stream_roles:Some(PdfIndexedDiff {added:vec![PdfIndexedItem {index:base.admitted_stream_roles.len(),value:added}],..Default::default()})});
    let inserted=protocol::apply_diff(insert.diff(&base).diff(),&base).unwrap();
    let mut restored=inserted.clone();
    for inverse in insert.inverse(&base).unwrap() {restored=protocol::apply_diff(inverse.diff(&restored).diff(),&restored).unwrap();}
    assert_eq!(restored,base);
    println!("[DEBUG] PDF role-only pure projection, indexed insertion/removal inverses confirmed");
}

#[test]
fn pdf_stream_roles_sampled_words_match_independent_native_extents() {
    use crate::standards::v1_7::subsets::base::schema::{snapshot::{PdfFunction,PdfShading,PdfShadingKind,PdfColorSpace},graph_projection::project_graph};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🚪️io/🪶️sqlite/📸️snapshot/🌈️color/🧫️fixtures/🔣️.json")).unwrap();
    for case in fixture["sampleWidths"].as_array().unwrap().iter().filter(|case|[12,32].contains(&case["bitsPerSample"].as_u64().unwrap())) {
        let bits=case["bitsPerSample"].as_u64().unwrap() as u32;
        let words:Vec<u32>=serde_json::from_value(case["sampleWords"].clone()).unwrap();
        let bytes:Vec<u8>=serde_json::from_value(case["nativeBytes"].clone()).unwrap();
        let mut document=text_document(&[(100.0,100.0,"sample")]);
        document.shadings.push(PdfShading {id:"Sample".into(),color_space:PdfColorSpace::DeviceGray,kind:PdfShadingKind::Axial {coords:[0.0,0.0,1.0,1.0],domain:None,function:PdfFunction::Sampled {domain:vec![0.0,1.0],range:vec![0.0,1.0],size:vec![words.len() as u32],bits_per_sample:bits,order:None,encode:None,decode:None,samples:words.clone()},extend:[false,false]},background:None,bbox:None,anti_alias:false,extra:Vec::new()});
        let native=encode_pdf(&document).unwrap();
        let independent=semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_sampled_function_streams(&native).unwrap();
        assert!(independent.contains(&(bits,bytes)));
        let admitted=decode_pdf(&native).unwrap();
        assert!(admitted.admitted_stream_roles.iter().any(|role|matches!(&role.value,PdfStreamRoleValue::SampledWords {samples} if samples==&words)));
        assert_eq!(project_graph(&admitted).unwrap().shadings,admitted.shadings);
        println!("[DEBUG] PDF sampled semantic words width={bits} match independent native extent and pure role projection");
    }
}

#[test]
fn pdf_stream_roles_native_known_payloads_persist_and_project_semantically() {
    use crate::standards::v1_7::subsets::base::{schema::{snapshot::*,graph_projection::project_graph},io::foreign_artifacts::admit_pdf_artifact,modules::fonts::truetype::synthesize_truetype};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let payload=|kind:&str|fixture.as_object().unwrap().values().find_map(|value|(value["value"]["kind"]==kind).then(||value["value"].clone())).unwrap();
    let glyphs:Vec<u16>=serde_json::from_value(payload("glyphIds")["glyphs"].clone()).unwrap();
    let attachment:Vec<u8>=serde_json::from_value(payload("attachmentBytes")["bytes"].clone()).unwrap();
    let code=payload("calculatorProgram")["code"].as_str().unwrap().to_string();
    let metadata=payload("metadataText")["text"].as_str().unwrap().to_string();
    let mut document=text_document(&[(100.0,100.0,"roles")]);
    let program=synthesize_truetype(1000,&[(65,600,vec![vec![(0,0),(0,700),(500,700)]])]);
    let reference=admit_pdf_artifact(&mut document,"s.stdio.font.truetype",PdfObject::Stream {dict:Vec::new(),data:program.clone(),filters:Vec::new()}).unwrap();
    let cmap=PdfEmbeddedCMap {name:"Fixture".into(),vertical:false,codespace:vec![PdfCodespaceRange {byte_width:4,low:0,high:u32::MAX}],mappings:vec![PdfCidMapping::Char {code:u32::MAX,cid:513}],use_cmap:None};
    let unicode=PdfToUnicode {byte_width:4,mappings:vec![PdfToUnicodeMapping::Char {code:u32::MAX,text:"Ω😀".into()}]};
    document.fonts.push(PdfFont {id:"RoleFont".into(),kind:PdfFontKind::Type0 {base_font:"Synth".into(),cmap:PdfCMap::Embedded {cmap:cmap.clone()},descendant:PdfCidFont {true_type:true,base_font:"Synth".into(),system_info:Default::default(),descriptor:PdfFontDescriptor {font_name:"Synth".into(),..Default::default()},default_width:1000.0,widths:Vec::new(),default_vertical:None,vertical_metrics:Vec::new(),cid_to_gid:Some(PdfCidToGid::Map {glyphs:glyphs.clone()}),program:Some(PdfFontProgram::TrueType {reference}),extra:Vec::new()}},to_unicode:Some(unicode.clone()),extra:Vec::new()});
    document.images.push(PdfImage::samples("RoleImage",1,1,PdfColorSpace::DeviceRgb,8,vec![0,127,255]));
    document.metadata=Some(metadata.clone());
    document.embedded_files.push(PdfEmbeddedFile {id:"RoleAttachment".into(),file_name:"fixture.bin".into(),description:None,mime_type:None,data:attachment.clone(),creation_date:None,modification_date:None,relationship:None,listed:true});
    document.shadings.push(PdfShading {id:"Calculator".into(),color_space:PdfColorSpace::DeviceGray,kind:PdfShadingKind::Axial {coords:[0.0,0.0,1.0,1.0],domain:None,function:PdfFunction::PostScript {domain:vec![0.0,1.0],range:vec![0.0,1.0],code:code.clone()},extend:[false,false]},background:None,bbox:None,anti_alias:false,extra:Vec::new()});
    let native=encode_pdf(&document).unwrap();
    let independent=semio_s_artifact_stdio_pdf_test_oracle::standards::v1_7::subsets::base::reference_stream_bodies(&native).unwrap();
    for expected in [program,attachment.clone(),metadata.clone().into_bytes(),code.clone().into_bytes(),glyphs.iter().flat_map(|glyph|glyph.to_be_bytes()).collect(),vec![0,127,255]] {assert!(independent.contains(&expected));}
    let admitted=decode_pdf(&native).unwrap();
    let roles=&admitted.admitted_stream_roles;
    for kind in [PdfStreamRoleKind::Operators,PdfStreamRoleKind::CalculatorProgram,PdfStreamRoleKind::CharacterMap,PdfStreamRoleKind::UnicodeMap,PdfStreamRoleKind::FontProgram,PdfStreamRoleKind::Image,PdfStreamRoleKind::MetadataText,PdfStreamRoleKind::AttachmentBytes,PdfStreamRoleKind::GlyphIds] {assert!(roles.iter().any(|role|role.value.kind()==kind));}
    assert!(roles.iter().any(|role|matches!(&role.value,PdfStreamRoleValue::UnicodeMap {mapping} if mapping==&unicode)));
    assert!(roles.iter().any(|role|matches!(&role.value,PdfStreamRoleValue::CharacterMap {cmap:actual} if actual==&cmap)));
    assert!(roles.iter().any(|role|matches!(&role.value,PdfStreamRoleValue::GlyphIds {glyphs:actual} if actual==&glyphs)));
    validate_role_inputs(&admitted.objects,&admitted.objects,&[],roles).unwrap();
    let projected=project_graph(&admitted).unwrap();
    assert_eq!(projected.fonts,admitted.fonts);assert_eq!(projected.images,admitted.images);assert_eq!(projected.metadata,admitted.metadata);assert_eq!(projected.embedded_files,admitted.embedded_files);assert_eq!(projected.shadings,admitted.shadings);
    use semio_framework_value::{FromValue,ToValue};
    assert_eq!(PdfSnapshot::from_value(admitted.to_value()).unwrap(),admitted);
    println!("[DEBUG] PDF nine known native stream roles captured typed semantics, independent stream bodies and exact pure/persisted projection");
}

