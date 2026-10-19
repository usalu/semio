//! 📏️ Exact Layout native capability before the owned relational implementation, with the real Semio drawing codec bound to the layout slot.
use semio_framework_test_original_snapshot_owners as original_owners;
use ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot as _;
use semio_framework_value::{FromValue, ToValue};
use semio_s_artifact_layout_layout_model::io::sqlite::snapshot::*;
use semio_s_artifact_layout_layout_model::snapshot::LayoutSnapshot;
use semio_s_artifact_layout_layout_model::{CharacterStyle, Frame, GridSettings, ImageLink, LAYOUT_DOCUMENT_SCHEMA, Layer, LayoutBounds, LayoutDrawingChild, LayoutRect, Page, PageColumns, PageMargins, PageOverride, ParagraphStyle, ParentPage, Spread, TextStory, TextStyleRun};
use semio_s_artifact_stdio_semio_drawing::schema::snapshot::SemioDrawingSnapshot;

fn typed(child: &LayoutDrawingChild) -> SemioDrawingSnapshot {
    <SemioDrawingSnapshot as FromValue>::from_value(child.content.clone()).expect("the bound codec admits the fixture drawing")
}
#[semio_framework_async_macros::async_test]
async fn sqlite_snapshot_layout_actual_declaration_publishes_exact_native_codec(){crate::bind_layout_background_drawing().unwrap();
    semio_framework_plugin::PluginBuilder::<semio_framework_plugin::NeedsLabel, semio_framework_plugin::app::NoPluginApp>::new("layout").label("Layout owned SQLite").version("0.0.1").package_id("semio:layout").artifact(semio_s_artifact_layout_layout::declaration().unwrap()).try_build().unwrap();
    let codec=store::os_store::codec_registry::document_codec(LAYOUT_DOCUMENT_SCHEMA).await.unwrap().unwrap();
    let provider=codec.snapshot_sqlite.expect("actual Layout declaration must publish semantic SQLite capability");
    assert_eq!(provider.snapshot_type,Some(std::any::TypeId::of::<LayoutSnapshot>()));
    let snapshot=semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();
    let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.layout.layout".into(),standard:"1".into(),subset:"*".into()};
    let payload=store::io::IoPayload::Binary(pack::snapshot::ArtifactPack::encode_pack(&snapshot));
    let database=original_owners::with_decoding(|native_owner|(provider.export)(LAYOUT_DOCUMENT_SCHEMA,&dialect,&payload,&mut store::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,store::sqlite_snapshot::SqliteDatabaseLimits::default()), native_owner)).unwrap().value;
    assert!(database.table("layout_document").is_ok());
}
#[test]
fn sqlite_snapshot_layout_declared_owner_capability(){crate::bind_layout_background_drawing().unwrap();let snapshot=semio_s_artifact_layout_layout_model::io::text::snapshot::snapshot_wire_codec::default_document();let provider=<LayoutSnapshot as pack::snapshot::ArtifactPack>::sqlite_snapshot_codec().expect("Layout must publish its complete typed semantic SQLite capability");let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.layout.layout".into(),standard:"1".into(),subset:"*".into()};let payload=store::io::IoPayload::Binary(pack::snapshot::ArtifactPack::encode_pack(&snapshot));let result=original_owners::with_decoding(|native_owner|(provider.export)(LAYOUT_DOCUMENT_SCHEMA,&dialect,&payload,&mut store::sqlite_snapshot::SqliteSnapshotControl::new(&mut |_|true,store::sqlite_snapshot::SqliteDatabaseLimits::default()), native_owner)).unwrap();assert!(result.value.table("layout_document").is_ok());}

fn component_fixture()->LayoutDrawingChild {
    use semio_s_artifact_stdio_semio_base::schema::geometry::SemioPoint2;
    use semio_s_artifact_stdio_semio_drawing::schema::snapshot::{DrawCanvas, DrawLayer, DrawNode};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let identity=fixture["childIdentity"].as_array().unwrap();
    let target=semio_framework_artifact_reference::ArtifactRef{dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:identity[1].as_str().unwrap().into(),standard:identity[2].as_str().unwrap().into(),subset:identity[3].as_str().unwrap().into()},artifact_id:identity[4].as_str().unwrap().into()};
    LayoutDrawingChild{
        handle:store::os_store::artifact_child::ArtifactChild::new(identity[0].as_str().unwrap().into(),target),
        content:SemioDrawingSnapshot{schema:fixture["drawingSchema"].as_str().unwrap().into(),canvas:DrawCanvas{width:-0.0,height:1.0,background:None},styles:vec![],layers:vec![DrawLayer{id:String::new(),name:String::new(),visible:false,root:DrawNode::Image{at:SemioPoint2{x:0.0,y:-0.0},width:1.0,height:2.0,mime:"image/private".into(),bytes:vec![0,127,255]}}]}.to_value(),
    }
}
#[test]
fn sqlite_snapshot_layout_inline_drawing_controlled_metadata(){crate::bind_layout_background_drawing().unwrap();
    let mut callback=|_|true;
    let mut decode=semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1024*1024,&mut callback);
    let shape=<LayoutDrawingChild as semio_framework_dsl_record::DslField>::shape_controlled(&mut decode).unwrap();
    let semio_framework_dsl_record::Shape::Record(producer)=shape else{panic!("inline Drawing record")};
    let spec=producer.decode(&mut decode).unwrap();
    assert_eq!(spec.fields.len(),2);assert_eq!(spec.fields[0].key,"handle");assert_eq!(spec.fields[1].key,"content");
    let mut encode_callback=|_|true;let mut encode=semio_framework_value::NativeEncodeControl::new(1024*1024,&mut encode_callback);
    assert_eq!(producer.encode(&mut encode).unwrap().fields.len(),2);
    assert!(producer.decode(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1,&mut |_|true)).is_err());
    assert!(producer.encode(&mut semio_framework_value::NativeEncodeControl::new(1,&mut |_|true)).is_err());
}
#[test]
fn sqlite_snapshot_layout_inline_drawing_owned_fields_controlled_input_output(){crate::bind_layout_background_drawing().unwrap();
    let expected=component_fixture();let value=<LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value(&expected);
    let restored=<LayoutDrawingChild as semio_framework_dsl_record::DslField>::from_value_controlled(&value,&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(8*1024*1024,&mut |_|true)).unwrap();
    assert_eq!(restored,expected);assert_eq!(restored.content.canvas.width.to_bits(),(-0.0f64).to_bits());
    let output=<LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value_controlled(&expected,&mut semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut |_|true)).unwrap();
    assert_eq!(output,value);
    assert!(<LayoutDrawingChild as semio_framework_dsl_record::DslField>::from_value_controlled(&value,&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(1,&mut |_|true)).is_err());
    assert!(<LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value_controlled(&expected,&mut semio_framework_value::NativeEncodeControl::new(1,&mut |_|true)).is_err());
    assert!(<LayoutDrawingChild as semio_framework_dsl_record::DslField>::from_value_controlled(&value,&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(8*1024*1024,&mut |_|false)).is_err());
    assert!(<LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value_controlled(&expected,&mut semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut |_|false)).is_err());
}
#[test]
fn sqlite_snapshot_layout_inline_drawing_long_native_copy_is_interior_controlled(){crate::bind_layout_background_drawing().unwrap();
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let total=fixture["intrinsicBytes"].as_u64().unwrap() as usize;
    let mut expected=component_fixture();let mut drawing=typed(&expected);drawing.schema="x".repeat(total);expected.content=drawing.to_value();
    let value=<LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value(&expected);
    let mut input=false;let mut input_callback=|event:semio_framework_value_native_owned::native_decoding::NativeDecodeProgress|{if event.total==total&&event.completed>=65536&&event.completed<total{input=true;false}else{true}};
    assert!(<LayoutDrawingChild as semio_framework_dsl_record::DslField>::from_value_controlled(&value,&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(8*1024*1024,&mut input_callback)).is_err());
    assert!(input,"must cancel during literal inline Drawing ownership copy");
    let mut output=false;let mut output_callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{if event.total==total&&event.completed>=65536&&event.completed<total{output=true;false}else{true}};
    assert!(<LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value_controlled(&expected,&mut semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut output_callback)).is_err());
    assert!(output,"must cancel during literal inline Drawing projection copy");
}

fn complete_fixture(word:f64,color:f32)->LayoutSnapshot{
    crate::bind_layout_background_drawing().unwrap();
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let maximum=fixture["maximumU64"].as_str().unwrap().parse::<u64>().unwrap();
    let bounds=LayoutBounds{x:word,y:word,width:word,height:word,rotation:word};let rect=LayoutRect{x:word,y:word,width:word,height:word};
    let layers=vec![Layer{id:String::new(),name:String::new(),visible:false,locked:true,object_ids:vec![String::new(),"!".into()]}];
    let frames=vec![Frame::Rect{id:String::new(),layer_id:String::new(),bounds:bounds.clone(),locked:None,visible:Some(false),fill:Some([color;4]),stroke:None},Frame::Text{id:String::new(),layer_id:String::new(),bounds:bounds.clone(),locked:Some(true),visible:None,story_id:String::new(),thread_next:Some(String::new()),columns:u32::MAX,inset:rect.clone(),wrap_mode:String::new()},Frame::Image{id:String::new(),layer_id:String::new(),bounds:bounds.clone(),locked:Some(false),visible:Some(true),link_id:String::new()}];
    let mut value=semio_s_artifact_layout_layout_model::snapshot::empty_layout_snapshot();value.schema=fixture["schema"].as_str().unwrap().into();value.grid=GridSettings{baseline_grid:word,baseline_offset:word,snap_to_baseline:false};
    value.paragraph_styles=vec![ParagraphStyle{id:String::new(),name:String::new(),font_family:String::new(),font_size:word,font_weight:u32::MAX,leading:word,tracking:word,alignment:String::new()}];
    value.character_styles=vec![CharacterStyle{id:String::new(),name:None,font_family:Some(String::new()),font_size:Some(word),font_weight:None,italic:Some(false),color:Some([color;4]),tracking:None}];
    value.stories=vec![TextStory{id:String::new(),content:"語\0".into(),style_runs:vec![TextStyleRun{start:maximum,end:0,paragraph_style_id:Some(String::new()),character_style_id:None}]}];
    value.links=vec![ImageLink{id:String::new(),path:String::new(),hash:String::new(),width:u32::MAX,height:0,dpi:1,color_profile:None,state:Some(String::new()),proxy_data_url:Some(String::new()),artifact_kind:fixture["imageArtifactKind"].as_str().unwrap().into(),artifact_ref:fixture["imageArtifactRef"].as_str().unwrap().into()}];
    value.parent_pages=vec![ParentPage{id:String::new(),name:String::new(),width:word,height:word,layer_ids:vec![String::new(),String::new()],layers:layers.clone(),frames:frames.clone()}];
    value.spreads=vec![Spread{id:String::new(),name:String::new(),page_ids:vec![String::new(),"!".into()]}];
    value.pages=vec![Page{id:String::new(),name:String::new(),spread_id:String::new(),parent_page_id:Some(String::new()),width:word,height:word,margins:PageMargins{top:word,right:word,bottom:word,left:word},columns:PageColumns{count:u32::MAX,gutter:word},guides:vec![rect],layer_ids:vec![String::new()],layers,frames,overrides:vec![PageOverride{object_id:String::new(),bounds:Some(bounds),visible:Some(false),locked:None},PageOverride{object_id:String::new(),bounds:None,visible:None,locked:Some(true)}]}];
    value.print_target=Some(String::new());value.data_fields=Some(semio_framework_value_dictionary::QuestionDictionary{entries:vec![
 semio_framework_value_dictionary::QuestionDictionaryEntry{question_id:String::new(),value:semio_framework_value::DslValue::String("intrinsic data field\0".into())},
 semio_framework_value_dictionary::QuestionDictionaryEntry{question_id:"null".into(),value:semio_framework_value::DslValue::Null},
 semio_framework_value_dictionary::QuestionDictionaryEntry{question_id:"boolean".into(),value:semio_framework_value::DslValue::Bool(true)},
 semio_framework_value_dictionary::QuestionDictionaryEntry{question_id:"unsigned".into(),value:semio_framework_value::DslValue::uint(u64::MAX)},
 semio_framework_value_dictionary::QuestionDictionaryEntry{question_id:"signed".into(),value:semio_framework_value::DslValue::int(i64::MIN)},
 semio_framework_value_dictionary::QuestionDictionaryEntry{question_id:"float".into(),value:semio_framework_value::DslValue::float(word)},
 semio_framework_value_dictionary::QuestionDictionaryEntry{question_id:"bytes".into(),value:semio_framework_value::DslValue::Bytes(vec![0,127,128,255])},
 semio_framework_value_dictionary::QuestionDictionaryEntry{question_id:"array".into(),value:semio_framework_value::DslValue::Array(vec![semio_framework_value::DslValue::Null,semio_framework_value::DslValue::uint(u64::MAX)])},
 semio_framework_value_dictionary::QuestionDictionaryEntry{question_id:"object".into(),value:semio_framework_value::DslValue::Object(vec![("repeat".into(),semio_framework_value::DslValue::float(word)),(String::new(),semio_framework_value::DslValue::Bytes(vec![0,255])),("repeat".into(),semio_framework_value::DslValue::Null)])},
 ]});value.background_drawing=Some(component_fixture());
    value.referenced_model=Some(store::ArtifactLink{target:semio_framework_artifact_reference::ArtifactRef{dialect:semio_framework_artifact_reference::ArtifactDialect{artifact_kind:String::new(),standard:"!/@".into(),subset:String::new()},artifact_id:String::new()},role:String::new(),pin:store::LinkPin::Snapshot{blob:store::BlobRef{hash:String::new(),size:maximum,media_type:String::new()}}});value
}
#[test]
fn sqlite_snapshot_layout_complete_owned_fields_words_and_literals_cross_both_native_encodings(){crate::bind_layout_background_drawing().unwrap();
    use store::{pack::snapshot::ArtifactDsl,pack::snapshot::ArtifactPack};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    let codec=<LayoutSnapshot as ArtifactPack>::sqlite_snapshot_codec().expect("complete Layout native capability");
    let dialect=semio_framework_artifact_reference::ArtifactDialect{artifact_kind:"s.layout.layout".into(),standard:"1".into(),subset:"*".into()};
    for(index,bits)in fixture["float64Bits"].as_array().unwrap().iter().enumerate(){
        let word=u64::from_str_radix(bits.as_str().unwrap(),16).unwrap();let color=u32::from_str_radix(fixture["float32Bits"][index].as_str().unwrap(),16).unwrap();let expected=complete_fixture(f64::from_bits(word),f32::from_bits(color));
        for encoding in [store::sqlite_snapshot::SnapshotEncoding::Binary,store::sqlite_snapshot::SnapshotEncoding::Text]{
            let payload=match encoding{store::sqlite_snapshot::SnapshotEncoding::Binary=>store::io::IoPayload::Binary(expected.encode_pack()),store::sqlite_snapshot::SnapshotEncoding::Text=>store::io::IoPayload::Text(expected.print_dsl())};
            let mut callback=|_|true;let mut control=store::sqlite_snapshot::SqliteSnapshotControl::new(&mut callback,store::sqlite_snapshot::SqliteDatabaseLimits::default());
            let database=original_owners::with_decoding(|native_owner|(codec.export)(LAYOUT_DOCUMENT_SCHEMA,&dialect,&payload,&mut control, native_owner)).unwrap().value;
            assert_eq!(database.table("layout_frame").unwrap().rows.len(),6);assert_eq!(database.table("layout_image_link").unwrap().rows[0].text(12).unwrap(),fixture["imageArtifactKind"].as_str().unwrap());
            assert_eq!(database.table("layout_scalar64").unwrap().rows[0].integer(2).unwrap() as u64,word);
            let result=original_owners::with_encoding(|native_owner|(codec.import)(LAYOUT_DOCUMENT_SCHEMA,&dialect, &mut Some(database),encoding,&mut control, native_owner)).unwrap().value;
            let restored=match result{store::io::IoPayload::Binary(bytes)=>LayoutSnapshot::decode_pack(&bytes).unwrap(),store::io::IoPayload::Text(text)=>LayoutSnapshot::parse_dsl(&text).unwrap()};
            assert_eq!(restored.grid.baseline_grid.to_bits(),word);assert_eq!(restored.character_styles[0].color.unwrap()[0].to_bits(),color);assert_eq!(restored.stories[0].style_runs[0].start,expected.stories[0].style_runs[0].start);assert_eq!(restored.links,expected.links);assert_eq!(restored.background_drawing.as_ref(),expected.background_drawing.as_ref());assert_eq!(restored.referenced_model,expected.referenced_model);assert_eq!(restored.data_fields,expected.data_fields);
            assert!(restored.encode_pack()==expected.encode_pack(),"complete derived owner fields must survive relational reconstruction");
        }
    }
}
#[test]
fn sqlite_snapshot_layout_deep_inline_drawing_component_has_owned_controlled_boundaries(){crate::bind_layout_background_drawing().unwrap();
    use semio_s_artifact_stdio_semio_drawing::schema::snapshot::DrawNode;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();let depth=fixture["componentDepth"].as_u64().unwrap() as usize;
    let mut expected=component_fixture();let mut drawing=typed(&expected);let mut node=std::mem::take(&mut drawing.layers[0].root);
    for _ in 0..depth{let mut parent=DrawNode::default();let DrawNode::Group{children,..}=&mut parent else{panic!("declared default group")};children.push(node);node=parent;}
    drawing.layers[0].root=node;expected.content=drawing.to_value();<SemioDrawingSnapshot as store::os_store::serialization::ArtifactSqliteSnapshotHost>::retire_sqlite_snapshot(drawing);
    let value=<LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value_controlled(&expected,&mut semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut |_|true)).unwrap();
    let restored=<LayoutDrawingChild as semio_framework_dsl_record::DslField>::from_value_controlled(&value,&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(8*1024*1024,&mut |_|true)).unwrap();
    assert_eq!(restored.handle,expected.handle);let restored_drawing=typed(&restored);let mut current=&restored_drawing.layers[0].root;let mut count=0;
    while let DrawNode::Group{children,..}=current{assert_eq!(children.len(),1);current=&children[0];count+=1;}
    assert_eq!(count,depth);let DrawNode::Image{bytes,..}=current else{panic!("complete image leaf")};assert_eq!(bytes,&[0,127,255]);
    <SemioDrawingSnapshot as store::os_store::serialization::ArtifactSqliteSnapshotHost>::retire_sqlite_snapshot(restored_drawing);
}

#[test]
fn sqlite_snapshot_layout_inline_drawing_all_node_fields_preserve_ieee_words(){crate::bind_layout_background_drawing().unwrap();
    use semio_s_artifact_stdio_semio_base::schema::geometry::{SemioPoint2, SemioTransform};
    use semio_s_artifact_stdio_semio_drawing::schema::snapshot::{DrawNode, PathSegment};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
    for bits in fixture["float64Bits"].as_array().unwrap(){
        let word=u64::from_str_radix(bits.as_str().unwrap(),16).unwrap();let number=f64::from_bits(word);let point=SemioPoint2{x:number,y:-0.0};
        let mut transform=SemioTransform::identity();transform.translation.x=number;
        let mut expected=component_fixture();let mut drawing=typed(&expected);drawing.layers[0].root=DrawNode::Group{transform,children:vec![
            DrawNode::Path{segments:vec![PathSegment::MoveTo{to:point},PathSegment::LineTo{to:point},PathSegment::CubicTo{c1:point,c2:point,to:point},PathSegment::QuadTo{c:point,to:point},PathSegment::ArcTo{rx:number,ry:number,x_rotation:number,large_arc:false,sweep:true,to:point},PathSegment::Close],style:Some(String::new())},
            DrawNode::Text{value:"English Deutsch\0文".into(),at:point,style:None},
            DrawNode::Image{at:point,width:number,height:-0.0,mime:String::new(),bytes:vec![0,127,255]},
        ]};expected.content=drawing.to_value();<SemioDrawingSnapshot as store::os_store::serialization::ArtifactSqliteSnapshotHost>::retire_sqlite_snapshot(drawing);
        let output=<LayoutDrawingChild as semio_framework_dsl_record::DslField>::to_value_controlled(&expected,&mut semio_framework_value::NativeEncodeControl::new(8*1024*1024,&mut |_|true)).unwrap();
        let restored=<LayoutDrawingChild as semio_framework_dsl_record::DslField>::from_value_controlled(&output,&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl::new(8*1024*1024,&mut |_|true)).unwrap();
        assert_eq!(restored.handle,expected.handle);
        assert_eq!(restored.content,expected.content,"every authored Drawing node and path field must remain exact");
        let restored_drawing=typed(&restored);let DrawNode::Group{transform,children}=&restored_drawing.layers[0].root else{panic!("literal group")};
        assert_eq!(transform.translation.x.to_bits(),word);
        let DrawNode::Image{width,bytes,..}=&children[2] else{panic!("literal image")};assert_eq!(width.to_bits(),word);assert_eq!(bytes,&[0,127,255]);
        <SemioDrawingSnapshot as store::os_store::serialization::ArtifactSqliteSnapshotHost>::retire_sqlite_snapshot(restored_drawing);
    }
}

#[path="💰️backing/🦀️.rs"]mod backing;

mod public_populated_owner {
    use super::*;
    use store::{pack::snapshot::ArtifactDsl,pack::snapshot::ArtifactPack,os_store::serialization::ArtifactSqliteSnapshotHost};
    use store::sqlite_snapshot::{SnapshotEncoding,SqliteDatabaseLimits};
    struct Owned(Option<LayoutSnapshot>);
    impl Owned{fn new(value:LayoutSnapshot)->Self{Self(Some(value))}fn get(&self)->&LayoutSnapshot{self.0.as_ref().unwrap()}}
    impl Drop for Owned{fn drop(&mut self){if let Some(value)=self.0.take(){value.retire_sqlite_snapshot();}}}
    fn exact(a:&LayoutSnapshot,b:&LayoutSnapshot){
        use semio_framework_value::{DslValue,Number,ToValue};
        fn same(a:&DslValue,b:&DslValue){match(a,b){
            (DslValue::Number(Number::Float(a)),DslValue::Number(Number::Float(b)))=>assert_eq!(a.to_bits(),b.to_bits()),
            (DslValue::Array(a),DslValue::Array(b))=>{assert_eq!(a.len(),b.len());for(a,b)in a.iter().zip(b){same(a,b);}},
            (DslValue::Object(a),DslValue::Object(b))=>{assert_eq!(a.len(),b.len());for((ka,a),(kb,b))in a.iter().zip(b){assert_eq!(ka,kb);same(a,b);}},
            _=>assert_eq!(a,b),
        }}
        same(&a.to_value(),&b.to_value());assert_eq!(a.encode_pack(),b.encode_pack());assert_eq!(a.print_dsl(),b.print_dsl());
    }
    fn register(){crate::bind_layout_background_drawing().unwrap();semio_framework_plugin::PluginBuilder::<semio_framework_plugin::NeedsLabel, semio_framework_plugin::app::NoPluginApp>::new("layout").label("Layout populated public SQLite").version("0.0.1").package_id("semio:layout").artifact(semio_s_artifact_layout_layout::declaration().unwrap()).try_build().unwrap();}
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_layout_populated_public_binary_text_exact_owner(){crate::bind_layout_background_drawing().unwrap();
        use {semio_framework_artifact_reference::ArtifactDialect,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot,store::io::io_mechanism::io_route};
        use store::io_schema::{IoFidelity,SQLITE_SNAPSHOT};
        register();let dialect=ArtifactDialect{artifact_kind:"s.layout.layout".into(),standard:"1".into(),subset:"*".into()};let sqlite=ArtifactDialect::from(SQLITE_SNAPSHOT);
        for route in[io_route(&dialect,&sqlite,1).await.unwrap().value,io_route(&sqlite,&dialect,1).await.unwrap().value]{assert_eq!(route.hops.len(),1);assert_eq!(route.fidelity,IoFidelity::Exact);}
        let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).unwrap();
        for(index,bits)in fixture["float64Bits"].as_array().unwrap().iter().enumerate(){let word=u64::from_str_radix(bits.as_str().unwrap(),16).unwrap();let color=u32::from_str_radix(fixture["float32Bits"][index].as_str().unwrap(),16).unwrap();let expected=Owned::new(complete_fixture(f64::from_bits(word),f32::from_bits(color)));
            for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=async{let original_file_dialect=&dialect;let original_file_input=expected.get();let original_file_encoding=encoding;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_|true;store::io::testing::with_snapshot_file_decoding(store::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await.unwrap().value;let actual=Owned::new(async{let original_file_dialect=&dialect;let original_file_input=&bytes;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_|true;store::io::testing::with_snapshot_file_encoding(store::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<LayoutSnapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value);exact(actual.get(),expected.get());}
        }
    }
    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_layout_populated_public_independent_sql_edit_retirement(){crate::bind_layout_background_drawing().unwrap();
        use {semio_framework_artifact_reference::ArtifactDialect,store::io::io_mechanism::io_export_sqlite_snapshot,store::io::io_mechanism::io_import_sqlite_snapshot};use std::{io::Write,process::{Command,Stdio}};
        register();let dialect=ArtifactDialect{artifact_kind:"s.layout.layout".into(),standard:"1".into(),subset:"*".into()};let expected=Owned::new(complete_fixture(-0.0,-0.0));
        for encoding in[SnapshotEncoding::Binary,SnapshotEncoding::Text]{let bytes=async{let original_file_dialect=&dialect;let original_file_input=expected.get();let original_file_encoding=encoding;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_|true;store::io::testing::with_snapshot_file_decoding(store::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_export_sqlite_snapshot(original_file_dialect,original_file_input,original_file_encoding,sqlite_control,native_owner).await).await}.await.unwrap().value;
            let script=r#"import{Database}from'bun:sqlite';const d=Database.deserialize(await Bun.stdin.bytes(),{safeIntegers:true});if(d.query('PRAGMA integrity_check').get().integrity_check!=='ok'||d.query('PRAGMA foreign_key_check').all().length)throw Error('integrity');if(d.query('SELECT COUNT(*) AS n FROM sqlite_schema WHERE type=\'table\'').get().n!==60n)throw Error('all43Layout16DrawingPlusMetadata');if(d.query('SELECT COUNT(*) AS n FROM layout_frame').get().n!==6n)throw Error('allFrameKinds');if(d.query('SELECT COUNT(*) AS n FROM layout_dictionary_entry').get().n!==9n)throw Error('allIntrinsicKinds');if(d.query('SELECT COUNT(*) AS n FROM layout_drawing_component').get().n!==1n)throw Error('inlineDrawing');if(d.query('SELECT content FROM layout_story WHERE id=1').get().content!=='語\u0000')throw Error('literalText');const m=d.query('SELECT * FROM semio_snapshot').get();if(JSON.stringify(Object.keys(m))!==JSON.stringify(['id','artifact_kind','standard','subset','schema_version','native_encoding'])||m.id!==1n||m.schema_version!==1n)throw Error('completePublicMetadata');if(m.artifact_kind!=='s.layout.layout'||m.standard!=='1'||m.subset!=='*'||m.native_encoding!==process.argv[1])throw Error('metadata');d.query('UPDATE layout_story SET content=? WHERE id=1').run('independent 日本\u0000');await Bun.write(Bun.stdout,d.serialize());d.close();"#;
            let mut child=Command::new("bun").args(["--eval",script,encoding.as_str()]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(&bytes).unwrap();let edited=child.wait_with_output().unwrap();assert!(edited.status.success(),"{}",String::from_utf8_lossy(&edited.stderr));let actual=Owned::new(async{let original_file_dialect=&dialect;let original_file_input=&edited.stdout;let original_file_limits=SqliteDatabaseLimits::default();let original_file_progress:&mut dyn FnMut(store::sqlite_snapshot::SqliteSnapshotProgress)->bool=&mut |_|true;store::io::testing::with_snapshot_file_encoding(store::io::testing::ORIGINAL_FILE_CALLER,original_file_limits,original_file_progress,async|sqlite_control,native_owner|io_import_sqlite_snapshot::<LayoutSnapshot>(original_file_dialect,original_file_input,sqlite_control,native_owner).await).await}.await.unwrap().value);let mut literal=complete_fixture(-0.0,-0.0);literal.stories[0].content="independent 日本\0".into();let literal=Owned::new(literal);exact(actual.get(),literal.get());
        }
    }
}
