use super::*;

#[test]
fn neutral_dense_layout_edits_refuse_before_ifc_publication() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("gltf-accessor-cases.json")).unwrap();
    for row in cases["invalid"].as_array().unwrap() {
        let mut source=gltf_io::import::decode_glb(include_bytes!("box.glb")).unwrap();
        let position=source.document.meshes[0].primitives[0].attributes.iter().find(|(name,_)|name=="POSITION").unwrap().1;
        let view=source.document.accessors[position].buffer_view.unwrap();
        match row["edit"].as_str().unwrap() {
            "misalignedViews"=>{source.buffers[0].insert(0,0);source.document.buffers[0].byte_length+=1;for item in &mut source.document.buffer_views {item.byte_offset+=1;}},
            "shortDeclaredBuffer"=>source.document.buffers[0].byte_length=1,
            "wrongPositionTarget"=>source.document.buffer_views[view].target=Some(34963),
            "nonbyteBufferValue"=>continue,
            _=>panic!("unknown neutral edit"),
        }
        assert!(gltf_to_ifc_controlled(&source,&mut |_|true).is_err(),"{}",row["name"]);
    }
}

#[test]
fn original_cancel_reaches_dense_accessor_rows() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("gltf-accessor-cases.json")).unwrap();
    let mut source=gltf_io::import::decode_glb(include_bytes!("box.glb")).unwrap();
    let position=source.document.meshes[0].primitives[0].attributes.iter().find(|(name,_)|name=="POSITION").unwrap().1;
    let count=cases["cancellation"]["count"].as_u64().unwrap() as usize;
    let minimum=cases["cancellation"]["minimumCompleted"].as_u64().unwrap() as usize;
    let accessor=&mut source.document.accessors[position];
    accessor.count=count;accessor.byte_offset=0;
    let view=&mut source.document.buffer_views[accessor.buffer_view.unwrap()];
    view.buffer=0;view.byte_offset=0;view.byte_length=count*12;view.byte_stride=None;
    source.buffers[0]=vec![0;count*12];source.document.buffers[0].byte_length=count*12;
    let mut seen=false;
    let failure=gltf_to_ifc_controlled(&source,&mut |progress|{
        if progress.phase=="gltf-accessor"&&progress.completed>=minimum {seen=true;false} else {true}
    }).unwrap_err();
    assert!(seen);assert_eq!(failure.code,"exchange.cancelled");
}
