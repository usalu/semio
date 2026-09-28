use super::*;

#[test]
fn point_references_match_neutral_topology_and_blake3_oracle() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    for row in cases.as_array().unwrap() {
        let segments:Vec<PathSegment>=serde_json::from_value(row["segments"].clone()).unwrap();
        let geometry=geometry_id(&segments).unwrap();
        let bytes=row["bytes"].as_str().unwrap().as_bytes().chunks_exact(2).map(|hex|u8::from_str_radix(std::str::from_utf8(hex).unwrap(),16).unwrap()).collect::<Vec<_>>();
        assert_eq!(geometry,blake3::hash(&bytes).to_hex().to_string());
        let slots=segments.iter().enumerate().flat_map(|(index,segment)|point_slots(segment).iter().map(move |point|serde_json::json!([index,point_name(*point)]))).collect::<Vec<_>>();
        assert_eq!(serde_json::Value::Array(slots),row["points"]);
        for (index,segment) in segments.iter().enumerate() {
            for point in point_slots(segment) {
                let layer_id=row["layerId"].as_str().unwrap();
                let id=point_id(layer_id,&geometry,index,*point).unwrap();
                assert_eq!(parse_point_id(&id),Some(PointSelectionRef {layer_id,geometry:&geometry,index,point:*point}));
            }
        }
        if !segments.is_empty() {
            let mut changed=segments.clone();
            if let PathSegment::Move {to}=&mut changed[0] {to[0]+=1.0;}
            assert_ne!(geometry_id(&changed).unwrap(),geometry);
            changed=segments.clone();changed.push(PathSegment::Move {to:[0.0,0.0]});
            assert_ne!(geometry_id(&changed).unwrap(),geometry);
        }
    }
    assert_eq!(geometry_id(&[PathSegment::Move {to:[-0.0,0.0]}]),geometry_id(&[PathSegment::Move {to:[0.0,-0.0]}]));
    assert!(geometry_id(&[PathSegment::Move {to:[f64::INFINITY,0.0]}]).is_none());
}

#[test]
fn point_reference_parser_rejects_ambiguous_addresses() {
    let geometry="a".repeat(64);
    for id in [format!(":{geometry}:0:anchor"),format!("a:{geometry}:00:anchor"),format!("a:{geometry}:-1:anchor"),format!("a:{geometry}:1.5:anchor"),format!("a:{geometry}:4294967296:anchor"),format!("a:{geometry}:9007199254740992:anchor"),format!("a:{geometry}:1:control3"),format!("a:{}:0:anchor",geometry.to_uppercase()),"a:short:0:anchor".into(),format!("a:{geometry}:0:anchor:extra")] {
        assert!(parse_point_id(&id).is_none(),"{id}");
    }
    assert!(point_id("",&geometry,0,PathPoint::Anchor).is_none());
    assert!(point_id("a","short",0,PathPoint::Anchor).is_none());
    assert!(point_id("a",&format!("prefix:{geometry}"),0,PathPoint::Anchor).is_none());
}

#[test]
fn modified_point_picks_match_neutral_selection_contract() {
    let cases:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🖱️picking/🔣️.json")).unwrap();
    for row in cases.as_array().unwrap() {
        let current:Vec<String>=serde_json::from_value(row["current"].clone()).unwrap();
        let result=pick_point_selection(&current,row["hit"].as_str(),serde_json::from_value(row["mode"].clone()).unwrap());
        assert_eq!(result,serde_json::from_value::<Vec<String>>(row["after"].clone()).unwrap(),"{}",row["name"]);
    }
}

#[test]
fn point_marquee_matches_transformed_anchor_and_merge_fixtures() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/▧️marquee/🔣️.json")).unwrap();
    let segments:Vec<PathSegment>=serde_json::from_value(fixture["segments"].clone()).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let matrix=serde_json::from_value(case["matrix"].clone()).unwrap();let start=serde_json::from_value(case["start"].clone()).unwrap();let end=serde_json::from_value(case["end"].clone()).unwrap();
        let actual=segments.iter().enumerate().filter_map(|(index,segment)|anchor_in_marquee(segment,matrix,start,end).then_some(index)).collect::<Vec<_>>();
        assert_eq!(serde_json::to_value(actual).unwrap(),case["indices"],"{}",case["name"]);
    }
    for case in fixture["merges"].as_array().unwrap() {
        let current:Vec<String>=serde_json::from_value(case["current"].clone()).unwrap();let hits:Vec<String>=serde_json::from_value(case["hits"].clone()).unwrap();let mode=serde_json::from_value(case["mode"].clone()).unwrap();
        assert_eq!(serde_json::to_value(merge_point_selection(&current,&hits,mode)).unwrap(),case["expected"]);
    }
    let mut hasher=geometry_hasher();for segment in &segments {hash_segment(&mut hasher,segment).unwrap();}
    assert_eq!(Some(hasher.finalize().to_hex()),geometry_id(&segments));
    eprintln!("[DEBUG] marquee anchor inclusion, selection set algebra and incremental geometry hash agree with shared fixtures");
}
