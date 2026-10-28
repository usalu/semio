use super::*;
use pack::snapshot::ArtifactPack;

#[test]
fn official_add2_exports_metres_y_up_and_native_pack() {
    let source = crate::io::parse_ifc4x3_step(include_str!("triangulated.ifc")).unwrap();
    let projected = ifc_to_gltf_controlled(&source, &mut |_| true).unwrap();
    let gltf = projected.snapshot;
    assert_eq!(gltf.document.meshes.len(), 1);
    let position = gltf.document.accessors.iter().find(|a| a.kind == GltfAccessorType::Vec3).unwrap();
    assert_eq!(position.min.as_deref(), Some(&[0.5, 0.0, -0.5][..]));
    assert_eq!(position.max.as_deref(), Some(&[1.5, 2.0, 0.5][..]));
    let restored = GltfSnapshot::decode_pack(&gltf.encode_pack()).unwrap();
    assert_eq!(restored.document, gltf.document);
    assert!(!projected.diagnostics.is_empty());
}

#[test]
fn khronos_box_imports_actual_add2_tessellation_and_roundtrips_glb() {
    let box_model = gltf_io::import::decode_glb(include_bytes!("box.glb")).unwrap();
    let source = gltf_to_ifc_controlled(&box_model, &mut |_| true).unwrap().snapshot;
    assert!(crate::io::write_ifc4x3_step(&source).unwrap().contains("IFC4X3_ADD2"));
    assert!(source.entities.iter().any(|e| e.name == "IFCTRIANGULATEDFACESET"));
    let target = ifc_to_gltf_controlled(&source, &mut |_| true).unwrap().snapshot;
    let bytes = gltf_io::export::encode_glb(&target).unwrap();
    assert_eq!(gltf_io::import::decode_glb(&bytes).unwrap().document.meshes.len(), 1);
}

#[test]
fn missing_units_foreign_schema_and_cancellation_are_atomic() {
    let mut source = crate::io::parse_ifc4x3_step(include_str!("triangulated.ifc")).unwrap();
    assert_eq!(ifc_to_gltf_controlled(&source, &mut |_| false).unwrap_err().code, "exchange.cancelled");
    source.entities.retain(|e| e.name != "IFCPROJECT");
    assert_eq!(ifc_to_gltf_controlled(&source, &mut |_| true).unwrap_err().code, "exchange.units");
    source.header.file_schema = vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4".into())])];
    assert_eq!(ifc_to_gltf_controlled(&source, &mut |_| true).unwrap_err().code, "exchange.schema");
}

#[test]
fn actual_composer_entries_read_only_their_declared_source_dialects() {
    let entries = entries();
    assert_eq!(entries.len(), 2);
    assert_eq!(<IfcToGltf as ArtifactComposer>::reads(), &[crate::io::IFC4X3_DIALECT]);
    assert_eq!(<GltfToIfc as ArtifactComposer>::reads(), &[GLTF_DIALECT]);
    let text=crate::io::TXT_DIALECT;
    assert!(io_entries().iter().all(|entry| entry.fidelity == if entry.from==text||entry.into==text {IoFidelity::Lossless} else {IoFidelity::Lossy}));
}

#[test]
fn erased_composers_publish_target_owned_native_packs_in_both_directions() {
    use semio_framework_plugin::io::{ErasedComposeSource,IoPayload};
    let source=crate::io::parse_ifc4x3_step(include_str!("triangulated.ifc")).unwrap();
    let export=&entries()[0];
    let target=semio_framework_async::poll::resolve_ready((export.compose)(&[ErasedComposeSource {dialect:crate::io::IFC4X3_DIALECT,payload:IoPayload::Binary(source.encode_pack())}])).unwrap();
    assert_eq!(target.dialect,GLTF_DIALECT);
    let IoPayload::Binary(pack)=target.payload else {panic!("native glTF pack required")};
    let gltf=GltfSnapshot::decode_pack(&pack).unwrap();
    let glb=gltf_io::export::encode_glb(&gltf).unwrap();
    let target=semio_framework_async::poll::resolve_ready((entries()[1].compose)(&[ErasedComposeSource {dialect:GLTF_DIALECT,payload:IoPayload::Binary(glb)}])).unwrap();
    assert_eq!(target.dialect,crate::io::IFC4X3_DIALECT);
    let IoPayload::Binary(pack)=target.payload else {panic!("native IFC pack required")};
    let ifc=IfcSnapshot::decode_pack(&pack).unwrap();
    assert_eq!(ifc.header.file_schema,source.header.file_schema);
    assert!(ifc.entities.iter().any(|e|e.name=="IFCTRIANGULATEDFACESET"));
    if let Ok(folder)=std::env::var("SEMIO_IFC_NATIVE_ORACLE_OUTPUT") {
        let folder=std::path::Path::new(&folder);
        std::fs::create_dir_all(folder).unwrap();
        std::fs::write(folder.join("native-triangulated.glb"),gltf_io::export::encode_glb(&gltf).unwrap()).unwrap();
        std::fs::write(folder.join("native-triangulated.ifc"),crate::io::write_ifc4x3_step(&ifc).unwrap()).unwrap();
    }
}

#[test]
fn cancellation_during_vertex_work_and_invalid_view_extent_publish_nothing() {
    let source=crate::io::parse_ifc4x3_step(include_str!("triangulated.ifc")).unwrap();
    assert_eq!(ifc_to_gltf_controlled(&source,&mut |p|p.phase!="vertices").unwrap_err().code,"exchange.cancelled");
    let mut box_model=decode_glb_checked(include_bytes!("box.glb")).unwrap();
    box_model.document.buffer_views[0].byte_length=1;
    assert_eq!(gltf_to_ifc_controlled(&box_model,&mut |_|true).unwrap_err().code,"exchange.accessor");
}

#[test]
fn original_io_authority_can_cancel_before_natural_glb_materialization() {
    let mut receive=|_|false;
    let mut publish=|_|true;
    let mut decode=semio_framework_value::NativeDecodeControl::new(1_000_000,&mut receive);
    let mut encode=semio_framework_value::NativeEncodeControl::new(1_000_000,&mut publish);
    let mut dr=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();
    let mut er=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    decode.install_retirement_recipient(&mut dr).unwrap();
    encode.install_retirement_recipient(&mut er).unwrap();
    let grant=semio_framework_value::RetainedCloneGrant {maximum_items:16_000_000,maximum_copy_bytes:1024*1024*1024,maximum_capacity_bytes:128*1024*1024,maximum_release_bytes:128*1024*1024,maximum_depth:512};
    let mut control=IoRunControl::new(&mut decode,&mut encode,grant);
    let error=semio_framework_async::poll::resolve_ready(GlbIntoIfcNative::deserialize(&IoPayload::Binary(include_bytes!("box.glb").to_vec()),&mut control)).unwrap_err();
    assert_eq!(error.cause.kind,semio_framework_value::ValueRefusalKind::Canceled);
    drop(control);
    while decode.has_retirement_owner() {decode.with_retirement(grant,|loan|loan.close_retirement_recipient(grant)).unwrap();}
    while encode.has_retirement_owner() {encode.close_retirement_recipient(grant).unwrap();}
}

#[test]
fn unresolved_georeference_and_extended_node_semantics_are_refused() {
    let mut source=crate::io::parse_ifc4x3_step(include_str!("triangulated.ifc")).unwrap();
    source.entities.push(semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcEntity {id:999999,name:"IFCMAPCONVERSION".into(),args:vec![],complex:vec![]});
    assert_eq!(ifc_to_gltf_controlled(&source,&mut |_|true).unwrap_err().code,"exchange.georeference");
    let mut gltf=decode_glb_checked(include_bytes!("box.glb")).unwrap();
    gltf.document.nodes[0].extensions=Some(GltfJson::Object(vec![]));
    assert_eq!(gltf_to_ifc_controlled(&gltf,&mut |_|true).unwrap_err().code,"exchange.deformation");
}

#[test]
fn source_validation_and_unit_iteration_retain_original_cancellation() {
    let source=crate::io::parse_ifc4x3_step(include_str!("triangulated.ifc")).unwrap();
    for phase in ["unit-assignment","validate"] {
        assert_eq!(ifc_to_gltf_controlled(&source,&mut |p|p.phase!=phase).unwrap_err().code,"exchange.cancelled");
    }
    crate::io::validate_ifc4x3_graph_controlled(&source,&mut |_|true).unwrap();
}

#[test]
fn malformed_node_graphs_and_accessor_alignment_are_atomic() {
    let source=decode_glb_checked(include_bytes!("box.glb")).unwrap();
    let mut cycle=source.clone();
    cycle.document.nodes[0].children.push(0);
    assert_eq!(gltf_to_ifc_controlled(&cycle,&mut |_|true).unwrap_err().code,"exchange.cycle");
    let mut multiple=source.clone();
    multiple.document.nodes.push(GltfNode {children:vec![0],..Default::default()});
    multiple.document.nodes.push(GltfNode {children:vec![0],..Default::default()});
    assert_eq!(gltf_to_ifc_controlled(&multiple,&mut |_|true).unwrap_err().code,"exchange.node-parent");
    let mut alignment=source.clone();
    let position=alignment.document.meshes[0].primitives[0].attributes.iter().find(|(key,_)|key=="POSITION").unwrap().1;
    alignment.document.accessors[position].byte_offset=1;
    assert_eq!(gltf_to_ifc_controlled(&alignment,&mut |_|true).unwrap_err().code,"exchange.accessor");
    let mut stride=source;
    let position=stride.document.meshes[0].primitives[0].attributes.iter().find(|(key,_)|key=="POSITION").unwrap().1;
    let view=stride.document.accessors[position].buffer_view.unwrap();
    stride.document.buffer_views[view].byte_stride=Some(13);
    assert_eq!(gltf_to_ifc_controlled(&stride,&mut |_|true).unwrap_err().code,"exchange.accessor");
}

#[test]
fn host_glb_delivery_uses_original_output_allocation_budget() {
    let source=crate::io::parse_ifc4x3_step(include_str!("triangulated.ifc")).unwrap();
    for maximum in [0,1_000_000] {
        let mut receive=|_|true;
        let mut publish=|_|true;
        let mut decode=semio_framework_value::NativeDecodeControl::new(1_000_000,&mut receive);
        let mut encode=semio_framework_value::NativeEncodeControl::new(maximum,&mut publish);
        let mut dr=semio_framework_value_native_owned::native_decoding::NativeDecodeRetirementRecipient::new();
        let mut er=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
        decode.install_retirement_recipient(&mut dr).unwrap();
        encode.install_retirement_recipient(&mut er).unwrap();
        let grant=semio_framework_value::RetainedCloneGrant {maximum_items:16_000_000,maximum_copy_bytes:1024*1024*1024,maximum_capacity_bytes:128*1024*1024,maximum_release_bytes:128*1024*1024,maximum_depth:512};
        let mut control=IoRunControl::new(&mut decode,&mut encode,grant);
        let outcome=semio_framework_async::poll::resolve_ready(IfcIntoGlb::serialize(&source,&ArchiveChildren::empty(),&mut control));
        if maximum==0 {assert!(outcome.is_err());} else {
            let IoPayload::Binary(bytes)=outcome.unwrap().value else {panic!("GLB bytes required")};
            assert_eq!(decode_glb_checked(&bytes).unwrap().document.meshes.len(),1);
        }
        drop(control);
        while decode.has_retirement_owner() {decode.with_retirement(grant,|loan|loan.close_retirement_recipient(grant)).unwrap();}
        while encode.has_retirement_owner() {encode.close_retirement_recipient(grant).unwrap();}
    }
}

#[test]
fn nonsingular_small_scales_do_not_become_singular_from_determinant_underflow() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("gltf-framing-cases.json")).unwrap();
    for scale in cases["scales"].as_array().unwrap().iter().map(|value|value.as_f64().unwrap()) {
        let mut source=decode_glb_checked(include_bytes!("box.glb")).unwrap();
        source.document.nodes[0].matrix=None;
        source.document.nodes[0].scale=Some([scale;3]);
        let output=gltf_to_ifc_controlled(&source,&mut |_|true).unwrap().snapshot;
        assert!(output.entities.iter().any(|e|e.name=="IFCTRIANGULATEDFACESET"));
    }
}

fn minimal_glb(json_padding:u8,bin_padding:u8)->Vec<u8> {
    let json=b"{\"asset\":{\"version\":\"2.0\"},\"buffers\":[{\"byteLength\":1}]}";
    let size=(json.len()+4)/4*4;
    let mut bytes=vec![0;28+size+4];
    bytes[..4].copy_from_slice(b"glTF");
    bytes[4..8].copy_from_slice(&2u32.to_le_bytes());
    let length=bytes.len() as u32;
    bytes[8..12].copy_from_slice(&length.to_le_bytes());
    bytes[12..16].copy_from_slice(&(size as u32).to_le_bytes());
    bytes[16..20].copy_from_slice(b"JSON");
    bytes[20..20+size].fill(json_padding);
    bytes[20..20+json.len()].copy_from_slice(json);
    bytes[20+size..24+size].copy_from_slice(&4u32.to_le_bytes());
    bytes[24+size..28+size].copy_from_slice(b"BIN\0");
    bytes[28+size]=7;
    bytes[29+size..].fill(bin_padding);
    bytes
}

#[test]
fn raw_glb_json_and_bin_padding_must_be_canonical() {
    decode_glb_checked(&minimal_glb(32,0)).unwrap();
    assert_eq!(decode_glb_checked(&minimal_glb(0,0)).unwrap_err().code,"exchange.glb");
    assert_eq!(decode_glb_checked(&minimal_glb(32,1)).unwrap_err().code,"exchange.glb");
}

#[test]
fn neutral_glb_framing_preserves_legal_json_text_and_ignores_future_chunks() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("gltf-framing-cases.json")).unwrap();
    for row in cases["cases"].as_array().unwrap() {
        let tail=row["jsonTail"].as_str().unwrap().as_bytes();
        let mut bytes=minimal_glb(tail[0],row["binPadding"].as_u64().unwrap() as u8);
        if row["unknownChunk"].as_bool().unwrap() {
            bytes.extend_from_slice(&4u32.to_le_bytes());
            bytes.extend_from_slice(b"EXTR");
            bytes.extend_from_slice(&[1,2,3,4]);
            let length=bytes.len() as u32;
            bytes[8..12].copy_from_slice(&length.to_le_bytes());
        }
        let outcome=decode_glb_checked(&bytes);
        assert_eq!(outcome.is_ok(),row["accepted"].as_bool().unwrap(),"{}",row["name"]);
        if let Ok(snapshot)=outcome {assert_eq!(snapshot.buffers,vec![vec![7]]);}
    }
}
