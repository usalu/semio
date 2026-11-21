//! 🧪️ Language-neutral modeling fixtures assert the surface, not just growing arrays.
use semio_framework_3d::mesh::*;
use std::collections::{BTreeMap,HashMap,HashSet};
use semio_framework_value::{ToValue,FromValue};

/// 🏦️ Drives one step under an authoring wallet equal to the granted stock: these laws run without a job turn.
fn funded<R>(grant:semio_framework_value::retained_clone::RetainedCloneGrant,step:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->R)->R{semio_framework_value::AuthoringWallet::new(grant,1<<20).run(grant,1<<16,step).unwrap().expect("work quote admitted").output}

#[test]
fn polygon_capture_keeps_canonical_rows_and_owned_metadata() {
    use semio_framework_value::{AuthoringWallet,DslValue,RetainedCloneGrant,ToValue};
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/📇️polygon-capture.json")).unwrap();
    let points:Vec<[f32;3]>=serde_json::from_value(law["positions"].clone()).unwrap();
    let faces:Vec<Vec<u32>>=serde_json::from_value(law["faces"].clone()).unwrap();
    let mut mesh=HalfedgeMesh::from_faces(&points,&faces).unwrap();
    for (i,face) in mesh.faces.iter_mut().enumerate(){face.flipped=law["flipped"][i].as_bool().unwrap();}
    for vertex in &mut mesh.vertices {vertex.normal=None;}
    mesh.vertices[law["vertexNormal"]["index"].as_u64().unwrap() as usize].normal=Some(serde_json::from_value(law["vertexNormal"]["normal"].clone()).unwrap());
    mesh.faces[law["smoothFace"].as_u64().unwrap() as usize].smooth=true;
    mesh.halfedges[law["cornerUv"]["index"].as_u64().unwrap() as usize].uv=serde_json::from_value(law["cornerUv"]["uv"].clone()).unwrap();
    mesh.mark_uv_seam(&[EdgeId(law["seam"].as_u64().unwrap() as u32)],true);
    let mut materials=mesh.materials().clone();
    materials.insert("paint".into(),DslValue::from(law["material"].clone()));
    let pattern:Vec<u8>=serde_json::from_value(law["texture"]["bytes"].clone()).unwrap();
    let texture=MeshTexture{mime:law["texture"]["mime"].as_str().unwrap().into(),
        bytes:pattern.repeat(law["texture"]["repeat"].as_u64().unwrap() as usize)};
    let mut textures=mesh.textures().clone();
    textures.insert("tex".into(),texture.clone());
    mesh.set_surface_assets(materials,textures).unwrap();
    let custom=DslValue::from(law["customSample"].clone());
    let mut values:Vec<DslValue>=(0..7).map(DslValue::uint).collect();
    values[2]=custom.clone();
    mesh.set_attribute("corners".into(),MeshAttribute{domain:MeshAttributeDomain::Corner,semantic:MeshAttributeSemantic::Custom,
        interpolation:MeshAttributeInterpolation::Nearest,values,
        indices:Some(serde_json::from_value(law["cornerIndex"].clone()).unwrap())}).unwrap();
    mesh.set_attribute("edges".into(),MeshAttribute{domain:MeshAttributeDomain::Edge,semantic:MeshAttributeSemantic::Custom,
        interpolation:MeshAttributeInterpolation::Nearest,values:(10..17).map(DslValue::uint).collect(),indices:None}).unwrap();
    mesh.set_attribute("material".into(),MeshAttribute{domain:MeshAttributeDomain::Face,semantic:MeshAttributeSemantic::Material,
        interpolation:MeshAttributeInterpolation::Constant,values:vec![DslValue::String("paint".into());2],indices:None}).unwrap();
    let original=mesh.to_json().unwrap();
    let stock=RetainedCloneGrant{maximum_items:10000,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<20,maximum_release_bytes:1<<20,maximum_depth:128};
    let mut wallet=AuthoringWallet::new(stock,1<<22);
    let turn=|capture:&mut MeshPolygonSourceCapture,wallet:&mut AuthoringWallet| {
        let d=capture.demand(&mesh,64).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,
            maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let (step,receipt)=wallet.run(grant,32,|effect|capture.step(&mesh,effect)).unwrap().unwrap().output.unwrap();
        assert!(receipt.copied_bytes<=d.copy_bytes && receipt.retained_capacity_bytes<=d.capacity_bytes && receipt.released_bytes<=d.release_bytes);
        wallet.post(receipt).unwrap();
        (step,receipt)
    };
    let mut capture=MeshPolygonSourceCapture::new();
    let denied=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:0,maximum_release_bytes:0,maximum_depth:128};
    let (step,receipt)=wallet.run(denied,32,|effect|capture.step(&mesh,effect)).unwrap().unwrap().output.unwrap();
    assert!(matches!(step,MeshPolygonSourceStep::Working));
    assert_eq!(receipt,Default::default());
    assert_eq!(capture.progress(&mesh).0,0);
    let output=loop {
        match turn(&mut capture,&mut wallet).0 {
            MeshPolygonSourceStep::Working=>{},
            MeshPolygonSourceStep::Done(output)=>break output,
            MeshPolygonSourceStep::Cancelled=>panic!("capture unexpectedly cancelled"),
        }
    };
    let expected=&law["expected"];
    assert_eq!(output.vertices,points);
    assert_eq!(serde_json::to_value(&output.faces).unwrap(),expected["faces"]);
    assert_eq!(output.attributes.get("corners").unwrap().indices,Some(serde_json::from_value(expected["cornerIndex"].clone()).unwrap()));
    assert_eq!(output.attributes.get("corners").unwrap().values[2],custom);
    assert_eq!(output.attributes.get("edges").unwrap().indices,Some(serde_json::from_value(expected["edges"].clone()).unwrap()));
    assert!(output.attributes.get("material").unwrap().indices.is_none());
    assert_eq!(serde_json::Value::from(output.to_value())["surface"],expected["surface"]);
    assert_eq!(output.materials.get("paint"),mesh.materials().get("paint"));
    assert_eq!(output.textures.get("tex"),Some(&texture));
    assert_eq!(mesh.polygon_source().unwrap(),output);

    let mut join=PolygonCombine::new(output.clone(),output).unwrap();
    let combined=loop {
        let d=join.demand(64).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,
            maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let turn=wallet.run(grant,32,|effect|join.step(effect)).unwrap().unwrap().output.unwrap();
        wallet.post(join.take_progress()).unwrap();
        if let Some(combined)=turn{break combined;}
    };
    assert_eq!(combined.vertices.len(),2*points.len());
    assert_eq!(combined.faces[2],vec![11,10,9,8]);
    assert_eq!(combined.surface.len(),2*expected["surface"].as_array().unwrap().len());
    assert_eq!(combined.textures.get("tex"),Some(&texture));
    assert_eq!(combined.materials.len(),1);
    assert_eq!(combined.attributes.get("corners").unwrap().domain_len(),14);

    let mut cancelled=MeshPolygonSourceCapture::new();
    let mut wallet=AuthoringWallet::new(stock,1<<22);
    loop {
        let (step,receipt)=turn(&mut cancelled,&mut wallet);
        assert!(matches!(step,MeshPolygonSourceStep::Working));
        if receipt.retained_capacity_bytes==texture.bytes.len(){break;}
    }
    assert!(cancelled.demand(&mesh.clone(),64).is_err());
    cancelled.cancel();
    assert!(matches!(turn(&mut cancelled,&mut wallet).0,MeshPolygonSourceStep::Cancelled));
    assert_eq!(mesh.to_json().unwrap(),original);
}


#[test]
fn partitioned_difference_projects_authored_cutter_faces() {
    use semio_framework_mesh_engine::PolygonMeshSource;
    use semio_framework_value::{AuthoringWallet, DslValue, RetainedCloneGrant};

    let law: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🔺️prismatic-difference.json")).unwrap();
    let nested: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🕳️nested-prismatic-difference.json")).unwrap();
    let tuple = |values: &[f64]| DslValue::Array(values.iter().copied().map(DslValue::float).collect());
    let source = |vertices: Vec<[f32; 3]>, faces: Vec<Vec<u32>>, prefix: &str, bias: f64| {
        let names: Vec<_> = (0..faces.len()).map(|i| format!("{prefix}-{i}")).collect();
        let mut materials = HistoryFoldIndex::new();
        for name in &names {
            materials.insert(name.clone(), DslValue::from(serde_json::json!({"baseColor":[0.25,0.5,0.75,1]})));
        }
        let mut attributes = HistoryFoldIndex::new();
        attributes.insert("material".into(), MeshAttribute {
            domain: MeshAttributeDomain::Face, semantic: MeshAttributeSemantic::Material,
            interpolation: MeshAttributeInterpolation::Constant,
            values: names.into_iter().map(DslValue::String).collect(), indices: None,
        });
        attributes.insert("uv".into(), MeshAttribute {
            domain: MeshAttributeDomain::Corner, semantic: MeshAttributeSemantic::Uv,
            interpolation: MeshAttributeInterpolation::Linear,
            values: faces.iter().enumerate().flat_map(|(face,ring)| {
                let vertices=&vertices;
                ring.iter().map(move |&id| {
                    let [x,y,z]=vertices[id as usize].map(f64::from);
                    tuple(&[bias+16.0*face as f64+x, y+z])
                })
            }).collect(), indices: None,
        });
        PolygonMeshSource { vertices, faces, surface: Vec::new(), attributes, materials, textures: HistoryFoldIndex::new() }
    };
    let wallet_stock = RetainedCloneGrant {
        maximum_items: 100_000, maximum_copy_bytes: 1<<27, maximum_capacity_bytes: 1<<27,
        maximum_release_bytes: 1<<27, maximum_depth: 256,
    };
    macro_rules! run_authored {
        ($job:expr) => {{
            let mut job=$job;
            let mut wallet=AuthoringWallet::new(wallet_stock,1<<28);
            (0..100_000).find_map(|_|{
                let d=job.demand(256).unwrap();
                let grant=RetainedCloneGrant {maximum_items:1,maximum_copy_bytes:d.copy_bytes,
                    maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
                let output=wallet.run(grant,32,|effect|job.step(effect)).unwrap().unwrap().output.unwrap();
                let receipt=job.take_progress();
                assert!(receipt.fits(grant));
                wallet.post(receipt).unwrap();
                output
            }).expect("authored Boolean publishes its complete polygon source")
        }};
    }
    let reimport=|projected|{
        let mut import = HalfedgeMesh::polygon_source_job(projected).unwrap();
        let mut wallet = AuthoringWallet::new(wallet_stock, 1<<28);
        (0..100_000).find_map(|_| {
            let mut receipt = Default::default();
            let grant = RetainedCloneGrant { maximum_items: 1, ..wallet.remaining() };
            let step = wallet.run_invoked(grant, 1<<16, |effect| import.step(1, effect, &mut receipt))
                .unwrap().unwrap().output.unwrap();
            wallet.post(receipt).unwrap();
            if let MeshModelingStep::Done(mesh) = step { Some(mesh) } else { None }
        }).expect("authored Boolean can be reimported")
    };
    let verify=|row:&serde_json::Value,expected_caps:Option<&serde_json::Value>,stock,cutter|{
        let components=row["components"].as_array();
        let uv_tolerance=if components.is_some(){1e-5}else{1e-7};
        let projected=run_authored!(MeshAuthoredDifference::new(stock,cutter));

        let mut caps = Vec::new();
        let mut stock_faces = std::collections::BTreeSet::new();
        let samples=row["edgeSamples"].as_array().map_or(&[][..], Vec::as_slice);
        let mut witnessed=vec![false;samples.len()];
        let mut corner = 0;
        let materials = projected.attributes.get("material").unwrap();
        let uv = projected.attributes.get("uv").unwrap();
        for (face, ring) in projected.faces.iter().enumerate() {
            let material = materials.value_at(face).unwrap().as_str().unwrap();
            let (operand, face) = material.split_once('-').unwrap();
            let owner = face.parse::<u32>().unwrap();
            let bias = if operand == "cutter" { caps.push(owner); 8.0 } else { stock_faces.insert(owner); 0.0 };
            let bias = bias + 16.0*f64::from(owner);
            for &id in ring {
                let [x, y, z] = projected.vertices[id as usize].map(f64::from);
                let second = if operand == "cutter" && row["footprint"].is_array() { (y + z).powi(2) }
                    else if operand == "stock" && !samples.is_empty() {
                        let [low,high]:[f64;2]=serde_json::from_value(nested["height"].clone()).unwrap();
                        let t=(z-low)/(high-low);
                        (1.-t)*(y+low).powi(2)+t*(y+high).powi(2)
                    } else { y + z };
                let expected = [bias + x, second];
                let actual = uv.value_at(corner).unwrap().as_array().unwrap();
                assert!(actual.iter().zip(expected).all(|(value, expected)| (value.as_f64().unwrap() - expected).abs() < uv_tolerance));
                if operand=="stock"{
                    for (index,sample) in samples.iter().enumerate(){
                        let position:[f64;3]=serde_json::from_value(sample["position"].clone()).unwrap();
                        if sample["face"].as_u64()==Some(u64::from(owner)) && position==[x,y,z]{
                            let expected:[f64;2]=serde_json::from_value(sample["uv"].clone()).unwrap();
                            assert!(actual.iter().zip(expected).all(|(value, expected)| (value.as_f64().unwrap()-expected).abs()<uv_tolerance));
                            let raw=projected.surface.iter().find(|row|row.kind==semio_framework_mesh_engine::PolygonSurfaceKind::CornerUv&&row.index as usize==corner).unwrap().uv.unwrap();
                            assert!(raw.into_iter().zip(expected).all(|(value,expected)|(f64::from(value)-expected-0.125).abs()<uv_tolerance));
                            witnessed[index]=true;
                        }
                    }
                }
                corner += 1;
            }
        }
        assert!(witnessed.into_iter().all(|matched|matched));
        caps.sort_unstable();
        if let Some(expected_caps) = expected_caps {
            assert_eq!(&serde_json::to_value(caps).unwrap(), expected_caps);
        } else {
            assert!(!caps.is_empty() && !stock_faces.is_empty());
        }
        if let Some(split)=row["splitStockOpening"].as_u64() {
            assert!(stock_faces.contains(&(split as u32)) && stock_faces.contains(&(split as u32+1)));
        }
        let mesh=reimport(projected);
        assert_eq!(mesh.attributes()["uv"].domain, MeshAttributeDomain::Corner);
        let total = |key: &str| components.map(|members| members.iter().map(|member| member[key].as_f64().unwrap()).sum())
            .or_else(|| row[key].as_f64());
        assert_edit_surface(&mesh, total("area"), total("volume"));
    };
    for row in law["cases"].as_array().unwrap() {
        let spec = &law["profiles"][row["profile"].as_str().unwrap()];
        let footprint: Vec<[f32; 2]> = serde_json::from_value(spec["footprint"].clone()).unwrap();
        let stock = source(serde_json::from_value(law["stock"]["positions"].clone()).unwrap(),
            serde_json::from_value(law["stock"]["faces"].clone()).unwrap(), "stock", 0.0);
        let heights: [f32; 2] = serde_json::from_value(row["height"].clone()).unwrap();
        let vertices = heights.into_iter().flat_map(|z| footprint.iter().map(move |&[x,y]| [x,y,z])).collect();
        let cutter = source(vertices, serde_json::from_value(spec["cutterFaces"].clone()).unwrap(), "cutter", 8.0);
        verify(row, Some(&row["capFaces"]), stock, cutter);
    }
    let profile: Vec<[f32; 2]> = serde_json::from_value(nested["footprint"].clone()).unwrap();
    let heights: [f32; 2] = serde_json::from_value(nested["height"].clone()).unwrap();
    let positions = heights.into_iter().flat_map(|z| profile.iter().map(move |&[x,y]| [x,y,z])).collect();
    let stock = source(serde_json::from_value(nested["stock"]["positions"].clone()).unwrap(),
        serde_json::from_value(nested["stock"]["faces"].clone()).unwrap(), "stock", 0.0);
    let mut cutter = source(positions, serde_json::from_value(nested["faces"].clone()).unwrap(), "cutter", 8.0);
    for value in &mut cutter.attributes.get_mut("uv").unwrap().values {
        let pair = value.as_array().unwrap();
        *value = tuple(&[pair[0].as_f64().unwrap(), pair[1].as_f64().unwrap().powi(2)]);
    }
    let source_faces = serde_json::json!((0..cutter.faces.len()).collect::<Vec<_>>());
    verify(&nested, Some(&source_faces), stock, cutter);

    let pocket: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/➖️convex-difference.json")).unwrap();
    let cube = |bounds: &serde_json::Value| {
        let [[x,y,z],[a,b,c]]: [[f32;3];2] = serde_json::from_value(bounds.clone()).unwrap();
        vec![[x,y,z],[a,y,z],[a,b,z],[x,b,z],[x,y,c],[a,y,c],[a,b,c],[x,b,c]]
    };
    let faces: Vec<Vec<u32>>=serde_json::from_value(pocket["faces"].clone()).unwrap();
    let mut stock=source(heights.into_iter().flat_map(|z|profile.iter().map(move |&[x,y]|[x,y,z])).collect(),
        serde_json::from_value(nested["faces"].clone()).unwrap(),"stock",0.);
    for (index,value) in stock.attributes.get_mut("uv").unwrap().values.iter_mut().enumerate(){
        let pair=value.as_array().unwrap();
        let x=pair[0].as_f64().unwrap();
        let y=pair[1].as_f64().unwrap().powi(2);
        *value=tuple(&[x,y]);
        stock.surface.push(semio_framework_mesh_engine::PolygonSurfaceSample{
            kind:semio_framework_mesh_engine::PolygonSurfaceKind::CornerUv,index:index as u32,
            normal:None,uv:Some([x as f32+0.125,y as f32+0.125])});
    }
    let split=&nested["split"];
    verify(split,None,stock,source(cube(&split["bounds"]),faces.clone(),"cutter",8.));
    let prism=||{
        let mut mesh=source(heights.into_iter().flat_map(|z|profile.iter().map(move |&[x,y]|[x,y,z])).collect(),
            serde_json::from_value(nested["faces"].clone()).unwrap(),"stock",0.);
        for (index,(face,vertex)) in mesh.faces.iter().enumerate().flat_map(|(face,ring)|ring.iter().map(move |&id|(face,id))).enumerate(){
            let [x,y,z]=mesh.vertices[vertex as usize].map(f64::from);
            let uv=[16.*face as f64+x*x,y+z];
            mesh.attributes.get_mut("uv").unwrap().values[index]=tuple(&uv);
            mesh.surface.push(semio_framework_mesh_engine::PolygonSurfaceSample{
                kind:semio_framework_mesh_engine::PolygonSurfaceKind::CornerUv,index:index as u32,
                normal:None,uv:Some(uv.map(|axis|axis as f32+0.125))});
        }
        mesh
    };
    for lateral in [&nested["lateral"],&nested["longitudinalSlab"],&nested["cornerSlab"]]{
        let tool=||{
            let mut vertices=cube(&lateral["bounds"]);
            if lateral["frame"]=="diagonal" {
                for point in &mut vertices {
                    let [u,v,z]=*point;
                    *point=[(u+v)/2.,(v-u)/2.,z];
                }
            }
            source(vertices,faces.clone(),"cutter",8.)
        };
        for (reversed,subtract) in [(false,false),(true,false),(false,true)]{
            let out=if subtract{run_authored!(MeshAuthoredDifference::new(prism(),tool()))}
                else if reversed{run_authored!(MeshAuthoredIntersection::new(tool(),prism()))}
                else{run_authored!(MeshAuthoredIntersection::new(prism(),tool()))};
            for sample in lateral["edgeSamples"].as_array().unwrap(){
                let material=format!("stock-{}",sample["face"].as_u64().unwrap());
                let point:[f32;3]=serde_json::from_value(sample["position"].clone()).unwrap();
                let expected:[f32;2]=serde_json::from_value(sample["uv"].clone()).unwrap();
                let mut offset=0;
                let mut found=None;
                for (face,ring) in out.faces.iter().enumerate(){
                    if out.attributes.get("material").unwrap().value_at(face).unwrap().as_str()==Some(material.as_str()){
                        found=ring.iter().position(|&vertex|out.vertices[vertex as usize]==point).map(|corner|offset+corner);
                        if found.is_some(){break;}
                    }
                    offset+=ring.len();
                }
                let corner=found.unwrap();
                assert_eq!(out.attributes.get("uv").unwrap().value_at(corner),Some(&tuple(&expected.map(f64::from))));
                let raw=out.surface.iter().find(|row|row.kind==semio_framework_mesh_engine::PolygonSurfaceKind::CornerUv
                    &&row.index as usize==corner).unwrap().uv.unwrap();
                assert_eq!(raw,expected.map(|axis|axis+0.125));
            }
            let expected=&lateral[if subtract{"difference"}else{"intersection"}];
            assert_edit_surface(&reimport(out),expected["area"].as_f64(),expected["volume"].as_f64());
        }
    }
    let with_raw_uv = |mut mesh: PolygonMeshSource| {
        mesh.surface = mesh.attributes.get("uv").unwrap().values.iter().enumerate().map(|(index, value)| {
            let values = value.as_array().unwrap();
            semio_framework_mesh_engine::PolygonSurfaceSample {
                kind: semio_framework_mesh_engine::PolygonSurfaceKind::CornerUv, index: index as u32,
                normal: None, uv: Some([0, 1].map(|axis| (values[axis].as_f64().unwrap() + 0.125) as f32)),
            }
        }).collect();
        mesh
    };
    for kind in ["mixedSlab", "mixedOpenSlab", "independentWalls", "fourArmWalls"] {
        let mixed = &nested[kind];
        let spec = if mixed["footprint"].is_array() { mixed } else { &nested };
        let domain: Vec<[f32; 2]> = serde_json::from_value(spec["footprint"].clone()).unwrap();
        let simple_prism = || with_raw_uv(source(heights.into_iter().flat_map(|z| domain.iter().map(move |&[x,y]| [x,y,z])).collect(),
            serde_json::from_value(spec["faces"].clone()).unwrap(), "stock", 0.));
        let slab=||with_raw_uv(source(cube(&mixed["bounds"]),faces.clone(),"cutter",8.));
        let cutter_faces=mixed["walls"].as_array().unwrap().iter().chain(mixed["horizontalWalls"].as_array().unwrap())
            .map(|wall|format!("cutter-{}",wall["face"].as_u64().unwrap())).collect::<std::collections::BTreeSet<_>>();
        for (reversed, subtract) in [(false,false),(true,false),(false,true)]{
            let out=if subtract{run_authored!(MeshAuthoredDifference::new(simple_prism(),slab()))}
                else if reversed{run_authored!(MeshAuthoredIntersection::new(slab(),simple_prism()))}
                else{run_authored!(MeshAuthoredIntersection::new(simple_prism(),slab()))};
            let mut observed=std::collections::BTreeSet::new();
            let corner_uvs = out.surface.iter().filter(|row| row.kind == semio_framework_mesh_engine::PolygonSurfaceKind::CornerUv)
                .map(|row| (row.index as usize, row.uv.unwrap())).collect::<std::collections::BTreeMap<_, _>>();
            let mut corner=0;
            for (face,ring) in out.faces.iter().enumerate(){
                let name=out.attributes.get("material").unwrap().value_at(face).unwrap().as_str().unwrap();
                let (side,row)=name.split_once('-').unwrap();
                let bias=16.*row.parse::<f64>().unwrap()+if side=="cutter"{observed.insert(name.to_owned());8.}else{0.};
                for &id in ring{
                    let [x,y,z]=out.vertices[id as usize].map(f64::from);
                    let uv=out.attributes.get("uv").unwrap().value_at(corner).unwrap().as_array().unwrap();
                    let expected = [bias + x, y + z];
                    assert!(uv.iter().zip(expected).all(|(value, want)| (value.as_f64().unwrap() - want).abs() < 1e-7));
                    assert_eq!(corner_uvs[&corner], expected.map(|value| (value + 0.125) as f32));
                    corner+=1;
                }
            }
            assert_eq!(observed,cutter_faces);
            let expected=&mixed[if subtract{"difference"}else{"intersection"}];
            assert_edit_surface(&reimport(out),expected["area"].as_f64(),expected["volume"].as_f64());
        }
    }
    for row in pocket["cases"].as_array().unwrap().iter().filter(|row|row["splitStockOpening"].is_u64()||row["recess"]==true){
        let mut stock_faces=faces.clone();
        if let Some(split)=row["splitStockOpening"].as_u64(){
            let split=split as usize;
            let [a,b,c,d]=stock_faces[split].as_slice().try_into().unwrap();
            stock_faces.splice(split..=split,[vec![a,b,c],vec![a,c,d]]);
        }
        verify(row,Some(&row["cutPlanes"]),
            source(cube(&pocket["left"]),stock_faces,"stock",0.0),
            source(cube(&row["right"]),faces.clone(),"cutter",8.0));
    }
    let disconnected: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🧩️disconnected-difference.json")).unwrap();
    let faces: Vec<Vec<u32>> = serde_json::from_value(disconnected["faces"].clone()).unwrap();
    for row in disconnected["cases"].as_array().unwrap() {
        let mut stock_faces = faces.clone();
        if row["leftReflected"] == true { for face in &mut stock_faces { face.reverse(); } }
        verify(row, None,
            source(serde_json::from_value(row["left"].clone()).unwrap(), stock_faces, "stock", 0.0),
            source(serde_json::from_value(row["right"].clone()).unwrap(), faces.clone(), "cutter", 8.0));
    }
}

#[test]
fn authored_convex_intersection_projects_both_uv_color_and_material_domains() {
    use semio_framework_mesh_engine::{PolygonMeshSource,PolygonSurfaceKind as Kind,PolygonSurfaceSample as Sample};
    use semio_framework_value::{AuthoringWallet,DslValue,RetainedCloneGrant,ValueRefusalKind};
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/⚗️authored-intersection.json")).unwrap();
    let tuple=|row:&[f64]|DslValue::Array(row.iter().copied().map(DslValue::float).collect());
    let cube=|spec:&serde_json::Value,split:bool|{
        let [lo,hi]:[[f32;3];2]=serde_json::from_value(spec["bounds"].clone()).unwrap();
        let vertices=vec![[lo[0],lo[1],lo[2]],[hi[0],lo[1],lo[2]],[hi[0],hi[1],lo[2]],[lo[0],hi[1],lo[2]],
            [lo[0],lo[1],hi[2]],[hi[0],lo[1],hi[2]],[hi[0],hi[1],hi[2]],[lo[0],hi[1],hi[2]]];
        let mut faces:Vec<Vec<u32>>=serde_json::from_value(law["faces"].clone()).unwrap();
        if split{
            let i=law["splitFace"].as_u64().unwrap() as usize;
            let ring=faces[i].clone();faces[i]=vec![ring[0],ring[1],ring[2]];
            faces.insert(i+1,vec![ring[0],ring[2],ring[3]]);
        }
        let bias=spec["uvBias"].as_f64().unwrap();
        let tone=spec["tone"].as_f64().unwrap();
        let material=spec["material"].as_str().unwrap();
        let normal:[f32;3]=serde_json::from_value(spec["normal"].clone()).unwrap();
        let mut attributes=HistoryFoldIndex::new();
        attributes.insert("color".into(),MeshAttribute{domain:MeshAttributeDomain::Vertex,semantic:MeshAttributeSemantic::Color,
            interpolation:MeshAttributeInterpolation::Linear,values:vertices.iter().map(|&[x,y,z]|
                tuple(&[tone+f64::from(x)/8.0,tone+f64::from(y)/8.0,tone+f64::from(z)/8.0,1.0])).collect(),indices:Some((0..8).collect())});
        attributes.insert("uv".into(),MeshAttribute{domain:MeshAttributeDomain::Corner,semantic:MeshAttributeSemantic::Uv,
            interpolation:MeshAttributeInterpolation::Linear,values:vertices.iter().map(|&[x,y,_]|tuple(&[bias+f64::from(x),bias+f64::from(y)])).collect(),
            indices:Some(faces.iter().flatten().copied().collect())});
        attributes.insert("material".into(),MeshAttribute{domain:MeshAttributeDomain::Face,semantic:MeshAttributeSemantic::Material,
            interpolation:MeshAttributeInterpolation::Constant,values:vec![DslValue::String(material.into())],indices:Some(vec![0;faces.len()])});
        let mut materials=HistoryFoldIndex::new();
        materials.insert(material.into(),DslValue::from(serde_json::json!({"baseColor":[tone,0.5,0.5,1]})));
        let mut surface=Vec::new();
        for (index,&[x,y,_]) in faces.iter().flatten().map(|&id|&vertices[id as usize]).enumerate(){
            surface.push(Sample{kind:Kind::CornerUv,index:index as u32,normal:None,uv:Some([x+bias as f32+0.125,y+bias as f32+0.125])});
        }
        for index in 0..vertices.len(){surface.push(Sample{kind:Kind::VertexNormal,index:index as u32,normal:Some(normal),uv:None});}
        surface.push(Sample{kind:Kind::SmoothFace,index:5,normal:None,uv:None});
        surface.push(Sample{kind:Kind::SeamEdge,index:16,normal:None,uv:None});
        PolygonMeshSource{vertices,faces,surface,attributes,materials,textures:HistoryFoldIndex::new()}
    };
    let stock=RetainedCloneGrant{maximum_items:100000,maximum_copy_bytes:1<<27,maximum_capacity_bytes:1<<27,
        maximum_release_bytes:1<<27,maximum_depth:256};
    let turn=|job:&mut MeshAuthoredIntersection,wallet:&mut AuthoringWallet|{
        let d=job.demand(256).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,
            maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let result=wallet.run(grant,32,|effect|job.step(effect)).unwrap().unwrap().output;
        let receipt=job.take_progress();
        assert!(receipt.fits(grant));wallet.post(receipt).unwrap();
        result
    };
    let run=|mut job:MeshAuthoredIntersection|{
        let mut wallet=AuthoringWallet::new(stock,1<<28);
        for _ in 0..100000{
            match turn(&mut job,&mut wallet){
                Ok(None)=>{},
                Ok(Some(value))=>return Ok(value),
                Err(error)=>return Err(error),
            }
        }
        panic!("authored intersection did not settle")
    };
    let diff:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/➖️convex-difference.json")).unwrap();
    let mut left=law["left"].clone();left["bounds"]=diff["left"].clone();
    let mut right=law["right"].clone();right["bounds"]=diff["cases"][0]["right"].clone();
    let with_normal=|spec:&serde_json::Value|{
        let mut source=cube(spec,false);
        let normal:Vec<f64>=serde_json::from_value(spec["normal"].clone()).unwrap();
        source.attributes.insert("authoredNormal".into(),MeshAttribute{domain:MeshAttributeDomain::Face,semantic:MeshAttributeSemantic::Normal,
            interpolation:MeshAttributeInterpolation::Constant,values:vec![tuple(&normal)],indices:Some(vec![0;source.faces.len()])});
        source
    };
    let mut difference=MeshAuthoredDifference::new(with_normal(&left),with_normal(&right));
    let mut wallet=AuthoringWallet::new(stock,1<<28);
    let sliced=loop {
        let d=difference.demand(256).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,
            maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let result=wallet.run(grant,32,|effect|difference.step(effect)).unwrap().unwrap().output.unwrap();
        let receipt=difference.take_progress();
        assert!(receipt.fits(grant));wallet.post(receipt).unwrap();
        if let Some(source)=result{break source;}
    };
    let mut caps=0;
    let mut offset=0;
    for (face,ring) in sliced.faces.iter().enumerate(){
        let is_cap=sliced.attributes.get("material").unwrap().value_at(face).unwrap().as_str()==right["material"].as_str();
        if is_cap{
            caps+=1;
            let normal:Vec<f64>=serde_json::from_value(right["normal"].clone()).unwrap();
            let normal=tuple(&normal.into_iter().map(|n|-n).collect::<Vec<_>>());
            let matches_normal=|value:&DslValue|value.as_array().unwrap().iter().zip(normal.as_array().unwrap()).all(|(a,b)|a.as_f64()==b.as_f64());
            assert!(matches_normal(sliced.attributes.get("authoredNormal").unwrap().value_at(face).unwrap()));
            for (i,&point) in ring.iter().enumerate(){
                let [x,y,_]=sliced.vertices[point as usize].map(f64::from);
                assert_eq!(x,1.0);
                let bias=right["uvBias"].as_f64().unwrap();
                assert_eq!(sliced.attributes.get("uv").unwrap().value_at(offset+i),Some(&tuple(&[bias+x,bias+y])));
                assert!(matches_normal(sliced.attributes.get("intersectionRawNormal").unwrap().value_at(offset+i).unwrap()));
            }
        }
        offset+=ring.len();
    }
    assert_eq!(caps,1);
    let sliced=HalfedgeMesh::from_faces(&sliced.vertices,&sliced.faces).unwrap();
    assert_edit_surface(&sliced,Some(16.0),Some(4.0));

    let mut corner_cutter=right.clone();corner_cutter["bounds"]=diff["cases"][6]["right"].clone();
    let mut cutter=with_normal(&corner_cutter);
    let second_cap="second-corner-cap";
    cutter.materials.insert(second_cap.into(),DslValue::from(serde_json::json!({"baseColor":[0.125,0.375,0.5,1]})));
    let binding=cutter.attributes.get_mut("material").unwrap();
    binding.values.push(DslValue::String(second_cap.into()));
    binding.indices.as_mut().unwrap()[4]=1;
    let mut corner=MeshAuthoredDifference::new(with_normal(&left),cutter);
    let mut wallet=AuthoringWallet::new(stock,1<<28);
    let cut=loop{
        let d=corner.demand(256).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,
            maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let result=wallet.run(grant,32,|effect|corner.step(effect)).unwrap().unwrap().output.unwrap();
        let receipt=corner.take_progress();assert!(receipt.fits(grant));wallet.post(receipt).unwrap();
        if let Some(output)=result{break output;}
    };
    assert_eq!((cut.vertices.len(),cut.faces.len(),cut.materials.len()),(14,11,3));
    let owned_caps=[corner_cutter["material"].as_str().unwrap(),second_cap];
    let mut seen=[false;2];
    let mut authored_crease=false;
    let mut start=0;
    for (face,ring) in cut.faces.iter().enumerate(){
        let material=cut.attributes.get("material").unwrap().value_at(face).unwrap().as_str().unwrap();
        if let Some(axis)=owned_caps.iter().rev().position(|name|*name==material){
            assert!(!seen[axis]);seen[axis]=true;
            let normal=cut.attributes.get("authoredNormal").unwrap().value_at(face).unwrap().as_array().unwrap();
            let authored=corner_cutter["normal"].as_array().unwrap();
            assert!(normal.iter().zip(authored).all(|(a,b)|a.as_f64().unwrap()==-b.as_f64().unwrap()));
            for (i,&vertex) in ring.iter().enumerate(){
                let [x,y,z]=cut.vertices[vertex as usize].map(f64::from);
                assert_eq!([x,y,z][axis],1.0);
                let bias=corner_cutter["uvBias"].as_f64().unwrap();
                assert_eq!(cut.attributes.get("uv").unwrap().value_at(start+i),Some(&tuple(&[bias+x,bias+y])));
                let next=cut.vertices[ring[(i+1)%ring.len()] as usize];
                if axis==0&&x==1.0&&y==1.0&&next[..2]==[1.0,1.0]{
                    authored_crease=cut.surface.iter().any(|row|row.kind==Kind::SeamEdge&&row.index as usize==start+i);
                }
            }
        }
        start+=ring.len();
    }
    assert_eq!(seen,[true;2]);
    assert!(authored_crease);
    let result=HalfedgeMesh::from_faces(&cut.vertices,&cut.faces).unwrap();
    assert_edit_surface(&result,Some(22.0),Some(6.0));

    let mut slab_cutter=right.clone();slab_cutter["bounds"]=diff["cases"][7]["right"].clone();
    let mut cutter=with_normal(&slab_cutter);
    cutter.materials.insert(second_cap.into(),DslValue::from(serde_json::json!({"baseColor":[0.125,0.375,0.5,1]})));
    let material=cutter.attributes.get_mut("material").unwrap();
    material.values.push(DslValue::String(second_cap.into()));
    material.indices.as_mut().unwrap()[5]=1;
    let normal=cutter.attributes.get_mut("authoredNormal").unwrap();
    normal.values.push(tuple(&[0.0,0.0,1.0]));
    normal.indices.as_mut().unwrap()[5]=1;
    let mut slab=MeshAuthoredDifference::new(with_normal(&left),cutter);
    let mut wallet=AuthoringWallet::new(stock,1<<28);
    let slab=loop{
        let d=slab.demand(256).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,
            maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let result=wallet.run(grant,32,|effect|slab.step(effect)).unwrap().unwrap().output.unwrap();
        let receipt=slab.take_progress();assert!(receipt.fits(grant));wallet.post(receipt).unwrap();
        if let Some(output)=result{break output;}
    };
    assert_eq!((slab.vertices.len(),slab.faces.len(),slab.materials.len()),(16,12,3));
    let caps=[slab_cutter["material"].as_str().unwrap(),second_cap];
    let mut seen=[false;2];
    let mut start=0;
    for (face,ring) in slab.faces.iter().enumerate(){
        let shell=usize::from(slab.vertices[ring[0] as usize][0]>=1.5);
        let interval=[0.0..=0.5,1.5..=2.0];
        assert!(ring.iter().all(|&id|interval[shell].contains(&slab.vertices[id as usize][0])));
        let material=slab.attributes.get("material").unwrap().value_at(face).unwrap().as_str().unwrap();
        if let Some(cap)=caps.iter().position(|name|*name==material){
            assert_eq!(cap,shell);
            assert!(!seen[cap]);seen[cap]=true;
            let expected=if cap==0{serde_json::from_value::<Vec<f64>>(slab_cutter["normal"].clone()).unwrap()}else{vec![0.0,0.0,1.0]};
            let normal=slab.attributes.get("authoredNormal").unwrap().value_at(face).unwrap().as_array().unwrap();
            assert!(normal.iter().zip(expected).all(|(actual,source)|actual.as_f64().unwrap()==-source));
            for (i,&id) in ring.iter().enumerate(){
                let [x,y,_]=slab.vertices[id as usize].map(f64::from);
                assert_eq!(x,[0.5,1.5][shell]);
                let bias=slab_cutter["uvBias"].as_f64().unwrap();
                assert_eq!(slab.attributes.get("uv").unwrap().value_at(start+i),Some(&tuple(&[bias+x,bias+y])));
            }
        }
        start+=ring.len();
    }
    assert_eq!(seen,[true;2]);
    let result=HalfedgeMesh::from_faces(&slab.vertices,&slab.faces).unwrap();
    assert_edit_surface(&result,Some(24.0),Some(4.0));

    let mut octant_cutter=right.clone();octant_cutter["bounds"]=diff["cases"][12]["right"].clone();
    let mut cutter=with_normal(&octant_cutter);
    let third_cap="third-octant-cap";
    for (name,face,normal) in [(second_cap,2,[0.0,0.0,1.0]),(third_cap,4,[1.0,0.0,0.0])]{
        cutter.materials.insert(name.into(),DslValue::from(serde_json::json!({"baseColor":[0.25,0.375,0.5,1]})));
        let material=cutter.attributes.get_mut("material").unwrap();
        material.indices.as_mut().unwrap()[face]=material.values.len() as u32;
        material.values.push(DslValue::String(name.into()));
        let named=cutter.attributes.get_mut("authoredNormal").unwrap();
        named.indices.as_mut().unwrap()[face]=named.values.len() as u32;
        named.values.push(tuple(&normal));
    }
    let mut octant=MeshAuthoredDifference::new(with_normal(&left),cutter);
    let mut wallet=AuthoringWallet::new(stock,1<<28);
    let octant=loop{
        let d=octant.demand(256).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,
            maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
        let result=wallet.run(grant,32,|effect|octant.step(effect)).unwrap().unwrap().output.unwrap();
        let receipt=octant.take_progress();assert!(receipt.fits(grant));wallet.post(receipt).unwrap();
        if let Some(output)=result{break output;}
    };
    assert_eq!((octant.vertices.len(),octant.faces.len(),octant.materials.len()),(19,15,4));
    let cap_names=[third_cap,second_cap,octant_cutter["material"].as_str().unwrap()];
    let mut shared=[None;3];
    let mut start=0;
    for (face,ring) in octant.faces.iter().enumerate(){
        let material=octant.attributes.get("material").unwrap().value_at(face).unwrap().as_str().unwrap();
        if let Some(axis)=cap_names.iter().position(|name|*name==material){
            let expected=match axis{0=>vec![1.0,0.0,0.0],1=>vec![0.0,0.0,1.0],_=>serde_json::from_value(octant_cutter["normal"].clone()).unwrap()};
            let normal=octant.attributes.get("authoredNormal").unwrap().value_at(face).unwrap().as_array().unwrap();
            assert!(normal.iter().zip(expected).all(|(actual,source)|actual.as_f64().unwrap()==-source));
            for (i,&id) in ring.iter().enumerate(){
                let [x,y,z]=octant.vertices[id as usize].map(f64::from);
                assert_eq!([x,y,z][axis],1.0);
                let bias=octant_cutter["uvBias"].as_f64().unwrap();
                assert_eq!(octant.attributes.get("uv").unwrap().value_at(start+i),Some(&tuple(&[bias+x,bias+y])));
                if [x,y,z]==[1.0;3]{assert!(shared[axis].replace(id).is_none());}
            }
        }
        start+=ring.len();
    }
    assert!(shared[0].is_some()&&shared==[shared[0];3]);
    let result=HalfedgeMesh::from_faces(&octant.vertices,&octant.faces).unwrap();
    assert_edit_surface(&result,Some(24.0),Some(7.0));

    let run_cut_shells=|index:usize|{
        let law=&diff["cases"][index];
        let pocket=law["pocket"].is_object();
        let cavity=law["cavity"].is_object();
        let mut tool=right.clone();tool["bounds"]=law["right"].clone();
        let mut cutter=with_normal(&tool);
        let mut surfaces=vec![(2,"wall-y-low",[0.0,-1.0,0.0]),(3,"wall-y-high",[0.0,1.0,0.0]),(4,"wall-x-low",[-1.0,0.0,0.0]),(5,"wall-x-high",[1.0,0.0,0.0])];
        if pocket||cavity{surfaces.push((1,"cutter-floor",[0.0,0.0,1.0]));}
        if cavity{surfaces.push((0,"cutter-ceiling",[0.0,0.0,-1.0]));}
        let uv=cutter.attributes.get_mut("uv").unwrap();
        uv.values=cutter.faces.iter().enumerate().flat_map(|(face,ring)|ring.iter().map({
            let vertices=&cutter.vertices;
            move |&id|{
                let [x,y,_]=vertices[id as usize].map(f64::from);
                tuple(&[face as f64*10.0+x,face as f64*20.0+y])
            }
        })).collect();
        uv.indices=Some((0..uv.values.len() as u32).collect());
        for &(face,name,normal) in &surfaces{
            cutter.materials.insert(name.into(),DslValue::from(serde_json::json!({"baseColor":[0.25,0.375,0.5,1]})));
            let material=cutter.attributes.get_mut("material").unwrap();
            material.indices.as_mut().unwrap()[face]=material.values.len() as u32;
            material.values.push(DslValue::String(name.into()));
            let named=cutter.attributes.get_mut("authoredNormal").unwrap();
            named.indices.as_mut().unwrap()[face]=named.values.len() as u32;
            named.values.push(tuple(&normal));
        }
        let mut stock_surface=with_normal(&left);
        let mut stock_input=with_normal(&left);
        if cavity{
            for surface in [&mut stock_surface,&mut stock_input,&mut cutter]{
                surface.attributes.insert("vertexAuthoredNormal".into(),MeshAttribute{domain:MeshAttributeDomain::Vertex,semantic:MeshAttributeSemantic::Normal,
                    interpolation:MeshAttributeInterpolation::Linear,values:vec![tuple(&[0.0,0.0,1.0])],indices:Some(vec![0;surface.vertices.len()])});
            }
        }
        let mut tunnel=MeshAuthoredDifference::new(stock_input,cutter);
        let mut wallet=AuthoringWallet::new(stock,1<<28);
        let tunnel=loop{
            let d=tunnel.demand(256).unwrap();
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,
                maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
            let result=wallet.run(grant,32,|effect|tunnel.step(effect)).unwrap().unwrap().output.unwrap();
            let receipt=tunnel.take_progress();assert!(receipt.fits(grant));wallet.post(receipt).unwrap();
            if let Some(output)=result{break output;}
        };
        assert_eq!(tunnel.vertices.len() as u64,law["vertices"].as_u64().unwrap());
        assert_eq!(tunnel.faces.len() as u64,law["faces"].as_u64().unwrap());
        assert_eq!(tunnel.materials.len(),surfaces.len()+2);
        if cavity{
            assert_eq!(tunnel.attributes.get("color").unwrap().domain,MeshAttributeDomain::Vertex);
            assert_eq!(tunnel.attributes.get("vertexAuthoredNormal").unwrap().domain,MeshAttributeDomain::Vertex);
        }
        let mut covered=vec![false;surfaces.len()];
        let mut wall_receipts=std::collections::BTreeMap::new();
        let mut start=0;
        for (face,ring) in tunnel.faces.iter().enumerate(){
            let material=tunnel.attributes.get("material").unwrap().value_at(face).unwrap().as_str().unwrap();
            if let Some(wall)=surfaces.iter().position(|(_,name,_)|*name==material){
                assert!(!covered[wall]);covered[wall]=true;
                let (chart,_,normal)=surfaces[wall];
                let chart=chart as f64;
                let named=tunnel.attributes.get("authoredNormal").unwrap().value_at(face).unwrap().as_array().unwrap();
                assert!(named.iter().zip(normal).all(|(actual,source)|actual.as_f64().unwrap()==-source));
                for (i,&id) in ring.iter().enumerate(){
                    let [x,y,z]=tunnel.vertices[id as usize].map(f64::from);
                    if pocket&&material=="cutter-floor"{assert_eq!(z,law["pocket"]["bottom"].as_f64().unwrap());}
                    let sample=[chart*10.0+x,chart*20.0+y];
                    assert_eq!(tunnel.attributes.get("uv").unwrap().value_at(start+i),Some(&tuple(&sample)));
                    if cavity{
                        let values=tunnel.attributes.get("vertexAuthoredNormal").unwrap().value_at(id as usize).unwrap().as_array().unwrap();
                        assert!(values.iter().zip([0.0,0.0,-1.0]).all(|(a,b)|a.as_f64()==Some(b)));
                    }
                    wall_receipts.entry(id).or_insert_with(Vec::new).push(sample);
                }
            }
            else if cavity{
                let original=&stock_surface.faces[face];
                assert_eq!(ring.len(),original.len());
                for (i,(&vertex,&source)) in ring.iter().zip(original).enumerate(){
                    assert_eq!(tunnel.vertices[vertex as usize],stock_surface.vertices[source as usize]);
                    assert_eq!(tunnel.attributes.get("vertexAuthoredNormal").unwrap().value_at(vertex as usize),Some(&tuple(&[0.0,0.0,1.0])));
                    assert_eq!(tunnel.attributes.get("uv").unwrap().value_at(start+i),stock_surface.attributes.get("uv").unwrap().value_at(start+i));
                }
            }
            start+=ring.len();
        }
        assert!(covered.into_iter().all(|present|present));
        assert_eq!(wall_receipts.len(),8);
        for (&vertex,uv) in &wall_receipts{
            let floor=cavity||pocket&&tunnel.vertices[vertex as usize][2]==law["pocket"]["bottom"].as_f64().unwrap() as f32;
            assert_eq!(uv.len(),if floor{3}else{2});
            for (i,sample) in uv.iter().enumerate(){assert!(!uv[..i].contains(sample));}
        }
        let result=HalfedgeMesh::from_faces(&tunnel.vertices,&tunnel.faces).unwrap();
        assert_edit_surface(&result,law["area"].as_f64(),law["volume"].as_f64());
        tunnel
    };
    let tunnel=run_cut_shells(16);
    let pocket=run_cut_shells(20);
    let cavity=run_cut_shells(23);

    let joint:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/➕️convex-union.json")).unwrap();
    left["bounds"]=joint["left"].clone();right["bounds"]=joint["cases"][0]["right"].clone();
    let run_union=|mut union:MeshAuthoredUnion|{
        let mut wallet=AuthoringWallet::new(stock,1<<28);
        loop{
            let d=union.demand(256).unwrap();
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,
                maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};
            let result=wallet.run(grant,32,|effect|union.step(effect)).unwrap().unwrap().output.unwrap();
            let receipt=union.take_progress();
            assert!(receipt.fits(grant));wallet.post(receipt).unwrap();
            if let Some(source)=result{break source;}
        }
    };
    let joined=run_union(MeshAuthoredUnion::new(with_normal(&left),with_normal(&right)));
    let mut separate=right.clone();separate["bounds"]=joint["cases"][5]["right"].clone();
    let separate=run_union(MeshAuthoredUnion::new(with_normal(&left),with_normal(&separate)));
    assert_eq!((separate.vertices.len(),separate.faces.len()),(16,12));
    assert_eq!(separate.attributes.get("color").unwrap().domain,MeshAttributeDomain::Vertex);
    assert_eq!(joined.vertices.len(),12);
    assert_eq!(joined.faces.len(),10);
    assert_eq!(joined.materials.len(),2);
    let mut at=0;
    let mut seam_domains=std::collections::BTreeMap::new();
    for (face,ring) in joined.faces.iter().enumerate(){
        let name=joined.attributes.get("material").unwrap().value_at(face).unwrap().as_str().unwrap();
        let side=usize::from(name==right["material"].as_str().unwrap());
        let spec=[&left,&right][side];
        let bias=spec["uvBias"].as_f64().unwrap();
        let tone=spec["tone"].as_f64().unwrap();
        let normal:Vec<f64>=serde_json::from_value(spec["normal"].clone()).unwrap();
        assert_eq!(joined.attributes.get("authoredNormal").unwrap().value_at(face),Some(&tuple(&normal)));
        for &id in ring{
            let [x,y,z]=joined.vertices[id as usize].map(f64::from);
            assert!(if side==0{x<=1.0}else{x>=1.0});
            assert_eq!(joined.attributes.get("uv").unwrap().value_at(at),Some(&tuple(&[bias+x,bias+y])));
            let color=joined.attributes.get("color").unwrap().value_at(at).unwrap().as_array().unwrap();
            assert!(color.iter().zip([tone+x/8.0,tone+y/8.0,tone+z/8.0,1.0]).all(|(a,b)|(a.as_f64().unwrap()-b).abs()<1e-6));
            if x==1.0{
                seam_domains.entry(id).or_insert([false;2])[side]=true;
                assert_eq!(joined.attributes.get("intersectionRawNormal").unwrap().value_at(at),Some(&tuple(&normal)));
            }
            at+=1;
        }
        assert!(ring.iter().any(|&id|joined.vertices[id as usize][0]!=1.0));
    }
    assert_eq!(seam_domains.len(),4);
    assert!(seam_domains.values().all(|domains|*domains==[true,true]));
    let joined=HalfedgeMesh::from_faces(&joined.vertices,&joined.faces).unwrap();
    assert_edit_surface(&joined,Some(32.0),Some(12.0));

    let mut shifted=right.clone();shifted["bounds"]=joint["cases"][7]["right"].clone();
    let partial=run_union(MeshAuthoredUnion::new(with_normal(&left),with_normal(&shifted)));
    assert_eq!((partial.vertices.len(),partial.faces.len(),partial.materials.len()),(24,20,2));
    assert_eq!(partial.attributes.get("color").unwrap().domain,MeshAttributeDomain::Corner);
    let mut overlap_faces=0;
    let mut at=0;
    for (face,ring) in partial.faces.iter().enumerate(){
        let material=partial.attributes.get("material").unwrap().value_at(face).unwrap().as_str().unwrap();
        let primary=material==left["material"].as_str().unwrap();
        let source=if primary{&left}else{&shifted};
        let bias=source["uvBias"].as_f64().unwrap();
        let mut center=[0.0;3];
        for &id in ring{
            let point=partial.vertices[id as usize].map(f64::from);
            assert_eq!(partial.attributes.get("uv").unwrap().value_at(at),Some(&tuple(&[bias+point[0],bias+point[1]])));
            for (sum,x) in center.iter_mut().zip(point){*sum+=x/ring.len() as f64;}
            at+=1;
        }
        if (center[2]==0.0||center[2]==2.0)&&[center[0],center[1]].into_iter().all(|p|p>1.0&&p<2.0){
            assert!(primary);overlap_faces+=1;
        }
    }
    assert_eq!(overlap_faces,2);
    let result=HalfedgeMesh::from_faces(&partial.vertices,&partial.faces).unwrap();
    assert_edit_surface(&result,Some(38.0),Some(14.0));

    let placed:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🧩️placed-union.json")).unwrap();
    let source_at=|spec:&serde_json::Value,points:&serde_json::Value|{
        let mut source=with_normal(spec);
        source.vertices=serde_json::from_value(points.clone()).unwrap();
        let bias=spec["uvBias"].as_f64().unwrap();
        let tone=spec["tone"].as_f64().unwrap();
        source.attributes.get_mut("uv").unwrap().values=source.vertices.iter().map(|&[x,y,_]|
            tuple(&[bias+f64::from(x),bias+f64::from(y)])).collect();
        source.attributes.get_mut("color").unwrap().values=source.vertices.iter().map(|&[x,y,z]|
            tuple(&[tone+f64::from(x)/8.,tone+f64::from(y)/8.,tone+f64::from(z)/8.,1.])).collect();
        for sample in &mut source.surface {
            if sample.kind!=Kind::CornerUv {continue;}
            let original=source.faces.iter().flatten().nth(sample.index as usize).unwrap();
            let [x,y,_]=source.vertices[*original as usize];
            sample.uv=Some([x+bias as f32+0.125,y+bias as f32+0.125]);
        }
        source
    };
    for row in placed["cases"].as_array().unwrap(){
        for order in [[0,1],[1,0]] {
            let operands=[(&left,&row["left"]),(&right,&row["right"])];
            let [first,second]=order.map(|side|source_at(operands[side].0,operands[side].1));
            let mesh=run_union(MeshAuthoredUnion::new(first,second));
            let mut corners=0;
            let mut owners=[false;2];
            for (face,ring) in mesh.faces.iter().enumerate(){
                let material=mesh.attributes.get("material").unwrap().value_at(face).unwrap().as_str().unwrap();
                let side=usize::from(material==right["material"].as_str().unwrap());
                let spec=operands[side].0;
                assert_eq!(Some(material),spec["material"].as_str());
                owners[side]=true;
                let (bias,tone)=(spec["uvBias"].as_f64().unwrap(),spec["tone"].as_f64().unwrap());
                for &vertex in ring{
                    let [x,y,z]=mesh.vertices[vertex as usize].map(f64::from);
                    let uv=mesh.attributes.get("uv").unwrap().value_at(corners).unwrap().as_array().unwrap();
                    let color=mesh.attributes.get("color").unwrap().value_at(corners).unwrap().as_array().unwrap();
                    assert!(uv.iter().zip([bias+x,bias+y]).all(|(a,b)|(a.as_f64().unwrap()-b).abs()<1e-5));
                    assert!(color.iter().zip([tone+x/8.,tone+y/8.,tone+z/8.,1.]).all(|(a,b)|(a.as_f64().unwrap()-b).abs()<1e-6));
                    corners+=1;
                }
            }
            assert_eq!(owners,[true;2]);
            let mesh=HalfedgeMesh::from_faces(&mesh.vertices,&mesh.faces).unwrap();
            assert_edit_surface(&mesh,row["area"].as_f64(),row["volume"].as_f64());
        }
    }

    let create=||MeshAuthoredIntersection::new(cube(&law["left"],false),cube(&law["right"],false));
    let out=run(create()).unwrap();
    let expected=&law["expected"];
    assert_eq!(out.vertices.len() as u64,expected["vertices"].as_u64().unwrap());
    assert_eq!(out.faces.len() as u64,expected["faces"].as_u64().unwrap());
    assert_eq!(out.materials.len(),2);
    assert_eq!(out.attributes.get("color").unwrap().domain,MeshAttributeDomain::Corner);
    let mut corner=0;
    let mut left_faces=0;
    for (face,ring) in out.faces.iter().enumerate(){
        let name=out.attributes.get("material").unwrap().value_at(face).unwrap().as_str().unwrap();
        let from_left=name==law["left"]["material"].as_str().unwrap();
        let source=if from_left{&law["left"]}else{&law["right"]};
        left_faces+=usize::from(from_left);
        let bias=source["uvBias"].as_f64().unwrap();
        let tone=source["tone"].as_f64().unwrap();
        for &id in ring{
            let [x,y,z]=out.vertices[id as usize].map(f64::from);
            for (key,expected) in [("uv",vec![bias+x,bias+y]),("color",vec![tone+x/8.0,tone+y/8.0,tone+z/8.0,1.0])]{
                let value=out.attributes.get(key).unwrap().value_at(corner).unwrap().as_array().unwrap();
                assert!(value.iter().zip(expected).all(|(a,b)|(a.as_f64().unwrap()-b).abs()<1e-6));
            }
            let authored_normal=out.attributes.get("intersectionRawNormal").unwrap().value_at(corner).unwrap().as_array().unwrap();
            assert!(authored_normal.iter().zip(source["normal"].as_array().unwrap()).all(|(a,b)|a.as_f64()==b.as_f64()));
            let raw=out.surface.iter().find(|row|row.kind==Kind::CornerUv&&row.index as usize==corner).unwrap().uv.unwrap();
            assert!((f64::from(raw[0])-bias-x-0.125).abs()<1e-6 && (f64::from(raw[1])-bias-y-0.125).abs()<1e-6);
            corner+=1;
        }
    }
    assert_eq!(left_faces as u64,expected["leftFaces"].as_u64().unwrap());
    assert_eq!((out.faces.len()-left_faces) as u64,expected["rightFaces"].as_u64().unwrap());
    assert!(out.surface.iter().any(|row|row.kind==Kind::SmoothFace));
    assert!(out.surface.iter().any(|row|row.kind==Kind::SeamEdge));
    assert!(out.attributes.values().any(|a|a.semantic==MeshAttributeSemantic::Normal));
    let mesh=HalfedgeMesh::from_faces(&out.vertices,&out.faces).unwrap();
    assert_edit_surface(&mesh,expected["area"].as_f64(),expected["volume"].as_f64());

    let reconstruct=|source|{
        let mut job=HalfedgeMesh::polygon_source_job(source).unwrap();
        let mut wallet=AuthoringWallet::new(stock,1<<28);
        for _ in 0..100000{
            let mut receipt=Default::default();
            let grant=RetainedCloneGrant{maximum_items:1,..wallet.remaining()};
            let turn=wallet.run_invoked(grant,1<<16,|effect|job.step(1,effect,&mut receipt)).unwrap().unwrap().output.unwrap();
            wallet.post(receipt).unwrap();
            if let MeshModelingStep::Done(mesh)=turn{return mesh;}
        }
        panic!("polygon reconstruction did not settle")
    };
    let imported_partial=reconstruct(partial);
    assert_eq!(imported_partial.materials().len(),2);
    assert_eq!(imported_partial.attributes()["uv"].domain,MeshAttributeDomain::Corner);
    let imported_cavity=reconstruct(cavity);
    assert_eq!(imported_cavity.materials().len(),8);
    assert_eq!(imported_cavity.attributes()["uv"].domain,MeshAttributeDomain::Corner);
    assert_eq!(imported_cavity.attributes()["vertexAuthoredNormal"].domain,MeshAttributeDomain::Vertex);
    assert_edit_surface(&imported_cavity,Some(30.0),Some(7.0));
    let imported_pocket=reconstruct(pocket);
    assert_eq!(imported_pocket.materials().len(),7);
    assert_eq!(imported_pocket.attributes()["uv"].domain,MeshAttributeDomain::Corner);
    let imported_tunnel=reconstruct(tunnel);
    assert_eq!(imported_tunnel.materials().len(),6);
    assert_eq!(imported_tunnel.attributes()["uv"].domain,MeshAttributeDomain::Corner);
    let imported_octant=reconstruct(octant);
    assert_eq!(imported_octant.materials().len(),4);
    assert_eq!(imported_octant.attributes()["uv"].domain,MeshAttributeDomain::Corner);
    let imported_slab=reconstruct(slab);
    assert_eq!(imported_slab.materials().len(),3);
    assert_eq!(imported_slab.attributes()["uv"].domain,MeshAttributeDomain::Corner);
    let reconstructed=reconstruct(out);
    assert_eq!(reconstructed.attributes()["uv"].domain,MeshAttributeDomain::Corner);
    assert_eq!(reconstructed.materials().len(),2);
    assert!(reconstructed.attributes().contains_key("intersectionRawNormal"));

    let mut contained=law["right"].clone();
    contained["bounds"]=serde_json::json!([[0.25,0.5,0.75],[1.25,1.5,1.75]]);
    let mut right=cube(&contained,false);
    let raw=right.attributes.get_mut("uv").unwrap();
    raw.values.push(tuple(&[99.0,99.0]));raw.indices.as_mut().unwrap()[22]=8;
    let inner=run(MeshAuthoredIntersection::new(cube(&law["left"],false),right)).unwrap();
    assert_eq!(inner.materials.len(),1);
    assert!(inner.attributes.get("uv").unwrap().values.contains(&tuple(&[99.0,99.0])));

    let rejected=MeshAuthoredIntersection::new(cube(&law["left"],false),cube(&law["right"],true));
    assert!(matches!(run(rejected),Err(e) if e.to_string().contains("coplanar authored")));
    let mut bad=cube(&law["left"],false);
    let a=bad.attributes.get_mut("uv").unwrap();
    a.values.push(tuple(&[99.0,99.0]));a.indices.as_mut().unwrap()[22]=8;
    assert!(matches!(run(MeshAuthoredIntersection::new(bad,cube(&law["right"],false))),Err(e) if e.to_string().contains("nonaffine")));
    let mut contact=law["right"].clone();contact["bounds"]=law["contact"].clone();
    let empty=run(MeshAuthoredIntersection::new(cube(&law["left"],false),cube(&contact,false))).unwrap();
    assert!(empty.vertices.is_empty()&&empty.faces.is_empty()&&empty.attributes.is_empty());
    let empty_mesh=reconstruct(empty);
    assert_eq!(empty_mesh.vertex_count(),0);
    assert_eq!(empty_mesh.face_count(),0);

    let mut cancelled=create();
    let mut wallet=AuthoringWallet::new(stock,1<<28);
    while cancelled.progress()<=0.7 {assert!(turn(&mut cancelled,&mut wallet).unwrap().is_none());}
    cancelled.cancel();
    assert!(matches!(turn(&mut cancelled,&mut wallet),Err(e) if e.kind==ValueRefusalKind::Canceled));
}

#[test]
fn knife_cut_fixtures_preserve_surfaces_and_shared_boundaries() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/✂️knife-cut/🔣️.json")).unwrap();
    for case in fixtures["cases"].as_array().unwrap() {
        let positions: Vec<[f32; 3]> = serde_json::from_value(case["mesh"]["vertices"].clone()).unwrap();
        let faces: Vec<Vec<u32>> = serde_json::from_value(case["mesh"]["faces"].clone()).unwrap();
        let start: [f32; 3] = serde_json::from_value(case["cut"]["start"].clone()).unwrap();
        let end: [f32; 3] = serde_json::from_value(case["cut"]["end"].clone()).unwrap();
        let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
        let label = case["name"].as_str().unwrap();
        mesh.knife_cut(FaceId(case["cut"]["face"].as_u64().unwrap() as u32), Vec3(start), Vec3(end)).unwrap();
        let expected = &case["expected"];
        if let Some(count) = expected["vertices"].as_u64() { assert_eq!(mesh.vertex_count(), count as usize, "{label}"); }
        if let Some(count) = expected["faces"].as_u64() { assert_eq!(mesh.face_count(), count as usize, "{label}"); }
        if let Some(count) = expected["minimumFaces"].as_u64() { assert!(mesh.face_count() >= count as usize, "{label}"); }
        let (actual_positions, actual_faces) = mesh.polygon_soup();
        assert_eq!(&actual_positions[..positions.len()], &positions, "{label}: original vertices changed");
        let selected = &faces[case["cut"]["face"].as_u64().unwrap() as usize];
        let point = |p: [f32; 3]| (p[0] as f64, p[1] as f64, p[2] as f64);
        let normal = newell_normal(&selected.iter().map(|&id| Vec3(positions[id as usize])).collect::<Vec<_>>());
        let plane = cross3(sub3(point(end), point(start)), point(normal.0));
        let extent = selected.iter().map(|&id| length3(sub3(point(positions[id as usize]), point(positions[selected[0] as usize])))).fold(0.0f64, f64::max);
        for face in &actual_faces {
            if !face.iter().all(|id| *id as usize >= positions.len() || selected.contains(id)) { continue; }
            let distances = face.iter().map(|&id| dot3(sub3(point(actual_positions[id as usize]), point(start)), plane) / length3(plane)).collect::<Vec<_>>();
            assert!(!(distances.iter().any(|&d| d > extent * 1e-6) && distances.iter().any(|&d| d < -extent * 1e-6)), "{label}: a cut piece still crosses the knife plane");
        }
        for point in case["boundaryPoints"].as_array().unwrap() {
            let point: [f32; 3] = serde_json::from_value(point.clone()).unwrap();
            assert!(actual_positions.contains(&point), "{label}: missing boundary point {point:?}");
        }
        if let Some(unchanged) = case["unchangedFaces"].as_array() {
            for face in unchanged {
                let face: Vec<u32> = serde_json::from_value(face.clone()).unwrap();
                assert!(actual_faces.contains(&face), "{label}: unrelated face changed");
            }
        }
        let mut incidence = HashMap::<(u32, u32), (usize, i32)>::new();
        let mut used = HashSet::new();
        for face in &actual_faces {
            for i in 0..face.len() {
                let (a, b) = (face[i], face[(i + 1) % face.len()]);
                used.insert(a);
                let entry = incidence.entry((a.min(b), a.max(b))).or_default();
                entry.0 += 1; entry.1 += if a < b { 1 } else { -1 };
            }
        }
        assert_eq!(used.len(), mesh.vertex_count(), "{label}: unused vertices");
        assert!(incidence.values().all(|edge| edge.0 == 1 || *edge == (2, 0)), "{label}: broken adjacency");
        assert_eq!(incidence.values().filter(|edge| edge.0 == 1).count(), expected["boundaryEdges"].as_u64().unwrap() as usize, "{label}");
        let transfer = mesh.tessellate().unwrap();
        let mut area = 0.0;
        let mut volume = 0.0;
        for triangle in transfer.indices.chunks_exact(3) {
            let point = |id: u32| { let i = id as usize * 3; (transfer.positions[i] as f64, transfer.positions[i + 1] as f64, transfer.positions[i + 2] as f64) };
            let [a, b, c] = [point(triangle[0]), point(triangle[1]), point(triangle[2])];
            let normal = cross3(sub3(b, a), sub3(c, a));
            let triangle_area = length3(normal) / 2.0;
            assert!(triangle_area > 0.0, "{label}: degenerate triangle");
            if let Some(n) = case["normal"].as_array() { assert!(dot3(normal, (n[0].as_f64().unwrap(), n[1].as_f64().unwrap(), n[2].as_f64().unwrap())) > 0.0, "{label}: reversed winding"); }
            area += triangle_area; volume += dot3(a, cross3(b, c)) / 6.0;
        }
        assert!((area / expected["area"].as_f64().unwrap() - 1.0).abs() < 1e-6, "{label}: area={area}");
        if let Some(expected) = expected["volume"].as_f64() { assert!((volume / expected - 1.0).abs() < 1e-6, "{label}: volume={volume}"); }
    }
}

#[test]
fn knife_cut_rejects_invalid_requests_atomically() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/✂️knife-cut/🔣️.json")).unwrap();
    let positions: Vec<[f32; 3]> = serde_json::from_value(fixtures["cases"][0]["mesh"]["vertices"].clone()).unwrap();
    let faces: Vec<Vec<u32>> = serde_json::from_value(fixtures["cases"][0]["mesh"]["faces"].clone()).unwrap();
    let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
    let before = mesh.to_obj().unwrap();
    for case in fixtures["invalid"].as_array().unwrap() {
        let start: [f32; 3] = serde_json::from_value(case["cut"]["start"].clone()).unwrap();
        let end: [f32; 3] = serde_json::from_value(case["cut"]["end"].clone()).unwrap();
        assert!(mesh.knife_cut(FaceId(case["cut"]["face"].as_u64().unwrap() as u32), Vec3(start), Vec3(end)).is_err(), "{}", case["name"]);
        assert_eq!(mesh.to_obj().unwrap(), before);
    }
    for start in [[f32::INFINITY, 0.0, 0.0], [f32::NAN, 0.0, 0.0]] {
        assert!(mesh.knife_cut(FaceId(0), Vec3(start), Vec3::new(1.0, 3.0, 0.0)).is_err());
        assert_eq!(mesh.to_obj().unwrap(), before);
    }
}

#[test]
fn component_transforms_apply_duplicate_vertices_once_and_reject_partial_mutation() {
    for operation in 0..3 {
        let apply = |mesh: &mut HalfedgeMesh, ids: &[VertexId]| match operation {
            0 => mesh.move_vertices(ids, Vec3::new(1.0, 2.0, 3.0)),
            1 => mesh.rotate_vertices(ids, Vec3::new(0.0, 0.0, 1.0), 0.5, Vec3::ZERO),
            _ => mesh.scale_vertices(ids, Vec3::new(2.0, 3.0, 4.0), Vec3::ZERO),
        };
        let mut once = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        let mut repeated = once.clone();
        let before = once.to_obj().unwrap();
        assert!(apply(&mut once, &[VertexId(0), VertexId(999)]).is_err());
        assert_eq!(once.to_obj().unwrap(), before);
        apply(&mut once, &[VertexId(0)]).unwrap();
        apply(&mut repeated, &[VertexId(0), VertexId(0)]).unwrap();
        assert_eq!(once.to_obj().unwrap(), repeated.to_obj().unwrap());
    }
    let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
    let before = mesh.to_obj().unwrap();
    assert!(mesh.move_vertices(&[VertexId(0)], Vec3::new(f32::INFINITY, 0.0, 0.0)).is_err());
    assert!(mesh.rotate_vertices(&[VertexId(0)], Vec3::ZERO, 1.0, Vec3::ZERO).is_err());
    assert!(mesh.scale_vertices(&[VertexId(0)], Vec3::new(1.0, 0.0, 1.0), Vec3::ZERO).is_err());
    assert_eq!(mesh.to_obj().unwrap(), before);
}

#[test]
fn loop_cut_fixtures_preserve_connected_surfaces() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for case in fixtures["loopCuts"].as_array().unwrap() {
        let positions: Vec<[f32; 3]> = serde_json::from_value(case["mesh"]["vertices"].clone()).unwrap();
        let faces: Vec<Vec<u32>> = serde_json::from_value(case["mesh"]["faces"].clone()).unwrap();
        let seeds: Vec<[u32; 2]> = serde_json::from_value(case["edges"].clone()).unwrap();
        let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
        let selected: Vec<_> = seeds.iter().map(|&[a, b]| {
            let find = |a, b| (0..mesh.halfedges.len()).find(|&id| mesh.edge_endpoints(EdgeId(id as u32)).unwrap() == (VertexId(a), VertexId(b)));
            EdgeId(find(a, b).or_else(|| find(b, a)).unwrap() as u32)
        }).collect();
        mesh.loop_cut(&selected, case["cuts"].as_u64().unwrap() as u32).unwrap();
        let expected = &case["expected"];
        let label = case["name"].as_str().unwrap();
        assert_eq!(mesh.vertex_count(), expected["vertices"].as_u64().unwrap() as usize, "{label}");
        assert_eq!(mesh.face_count(), expected["faces"].as_u64().unwrap() as usize, "{label}");
        if let Some(expected) = case.get("addedPositions") {
            let expected: Vec<[f32; 3]> = serde_json::from_value(expected.clone()).unwrap();
            let actual = (positions.len()..mesh.vertex_count()).map(|id| mesh.vertex_position(VertexId(id as u32)).unwrap().0).collect::<Vec<_>>();
            assert_eq!(actual.len(), expected.len(), "{label}");
            for point in expected { assert!(actual.contains(&point), "{label}: missing cut point {point:?}"); }
        }
        if let Some(expected) = case.get("unchangedFaces") {
            let expected: Vec<Vec<u32>> = serde_json::from_value(expected.clone()).unwrap();
            let (_, actual) = mesh.polygon_soup();
            for face in expected { assert!(actual.contains(&face), "{label}: untouched face changed"); }
        }
        let mut incidence = HashMap::<(u32, u32), (usize, i32)>::new();
        let mut used = HashSet::new();
        for face in 0..mesh.face_count() {
            let ids = mesh.face_vertex_ids(FaceId(face as u32)).unwrap();
            for i in 0..ids.len() {
                let (a, b) = (ids[i].0, ids[(i + 1) % ids.len()].0);
                used.insert(a);
                let entry = incidence.entry((a.min(b), a.max(b))).or_default();
                entry.0 += 1; entry.1 += if a < b { 1 } else { -1 };
            }
        }
        assert_eq!(used.len(), mesh.vertex_count(), "{label}: disconnected vertices");
        assert_eq!(incidence.len(), expected["edges"].as_u64().unwrap() as usize, "{label}");
        assert!(incidence.values().all(|edge| edge.0 == 1 || *edge == (2, 0)), "{label}: nonmanifold or inverted edge");
        assert_eq!(incidence.values().filter(|edge| edge.0 == 1).count(), expected["boundaryEdges"].as_u64().unwrap() as usize, "{label}");
        let transfer = mesh.tessellate().unwrap();
        let mut area = 0.0;
        let mut volume = 0.0;
        for triangle in transfer.indices.chunks_exact(3) {
            let point = |id: u32| { let offset = id as usize * 3; (transfer.positions[offset] as f64, transfer.positions[offset + 1] as f64, transfer.positions[offset + 2] as f64) };
            let [a, b, c] = [point(triangle[0]), point(triangle[1]), point(triangle[2])];
            let normal = cross3(sub3(b, a), sub3(c, a));
            let triangle_area = normal.0.hypot(normal.1).hypot(normal.2) / 2.0;
            assert!(triangle_area > 0.0, "{label}: degenerate triangle");
            area += triangle_area; volume += dot3(a, cross3(b, c)) / 6.0;
        }
        assert!((area - expected["area"].as_f64().unwrap()).abs() < 1e-6, "{label}: area={area}");
        if let Some(expected) = expected["volume"].as_f64() { assert!((volume - expected).abs() < 1e-6, "{label}: volume={volume}"); }
    }
}

#[test]
fn loop_cut_rejects_invalid_requests_atomically() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for case in fixtures["invalidLoopCuts"].as_array().unwrap() {
        let positions: Vec<[f32; 3]> = serde_json::from_value(case["mesh"]["vertices"].clone()).unwrap();
        let faces: Vec<Vec<u32>> = serde_json::from_value(case["mesh"]["faces"].clone()).unwrap();
        let edges = case["edges"].as_array().unwrap().iter().map(|id| EdgeId(id.as_u64().unwrap() as u32)).collect::<Vec<_>>();
        let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
        let before = mesh.to_obj().unwrap();
        assert!(mesh.loop_cut(&edges, case["cuts"].as_u64().unwrap() as u32).is_err(), "{}", case["name"]);
        assert_eq!(mesh.to_obj().unwrap(), before);
    }
    for (edges, cuts) in [(vec![], 1), (vec![EdgeId(0)], 0), (vec![EdgeId(0)], 257), (vec![EdgeId(0), EdgeId(999)], 1), (vec![EdgeId(0), EdgeId(1), EdgeId(8)], 256)] {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        let before = mesh.to_obj().unwrap();
        assert!(mesh.loop_cut(&edges, cuts).is_err());
        assert_eq!(mesh.to_obj().unwrap(), before);
    }
    let mut triangle = HalfedgeMesh::from_faces(&[[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]], &[vec![0, 1, 2]]).unwrap();
    let before = triangle.to_obj().unwrap();
    assert!(triangle.loop_cut(&[EdgeId(0)], 1).is_err());
    assert_eq!(triangle.to_obj().unwrap(), before);
}

#[test]
fn vector_normalization_preserves_direction_across_scales() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for direction in fixtures["directions"].as_array().unwrap() {
        for scale in fixtures["scales"].as_array().unwrap() {
            let scale = scale.as_f64().unwrap();
            let vector: [f64; 3] = serde_json::from_value(direction["vector"].clone()).unwrap();
            let expected: [f32; 3] = serde_json::from_value(direction["unit"].clone()).unwrap();
            let actual = Vec3(vector.map(|coordinate| (coordinate * scale) as f32)).normalize();
            for axis in 0..3 { assert!((actual.0[axis] - expected[axis]).abs() < 1e-6, "scale={scale}: {actual:?}"); }
        }
    }
}

#[test]
fn sphere_refinement_has_no_missing_center_faces() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for case in fixtures["spheres"].as_array().unwrap() {
        let radius = case["radius"].as_f64().unwrap();
        let level = case["subdivisions"].as_u64().unwrap() as u32;
        let mesh = HalfedgeMesh::ico_sphere_prim(radius as f32, level).unwrap();
        assert_eq!(mesh.vertex_count(), case["vertices"].as_u64().unwrap() as usize);
        assert_eq!(mesh.face_count(), case["faces"].as_u64().unwrap() as usize);
        let mut edges = HashMap::<(u32, u32), (usize, i32)>::new();
        for face in 0..mesh.face_count() {
            let ids = mesh.face_vertex_ids(FaceId(face as u32)).unwrap();
            let points: Vec<_> = ids.iter().map(|id| mesh.vertex_position(*id).unwrap().0.map(|coordinate| coordinate as f64 / radius)).collect();
            let [a, b, c] = [points[0], points[1], points[2]].map(|p| (p[0], p[1], p[2]));
            assert!(dot3(a, cross3(sub3(b, a), sub3(c, a))) > 0.0);
            for i in 0..3 {
                let (a, b) = (ids[i].0, ids[(i + 1) % 3].0);
                let entry = edges.entry((a.min(b), a.max(b))).or_default();
                entry.0 += 1; entry.1 += if a < b { 1 } else { -1 };
            }
        }
        assert!(edges.values().all(|edge| *edge == (2, 0)));
        for vertex in 0..mesh.vertex_count() {
            let point = mesh.vertex_position(VertexId(vertex as u32)).unwrap().0.map(|coordinate| coordinate as f64 / radius);
            assert!((point[0].hypot(point[1]).hypot(point[2]) - 1.0).abs() < 1e-6);
        }
    }
    for (radius, level) in [(0.0, 0), (-1.0, 1), (f32::NAN, 1), (1.0, 6)] { assert!(HalfedgeMesh::ico_sphere_prim(radius, level).is_err()); }
}

#[test]
fn polygon_tessellation_preserves_shape_at_all_fixture_scales() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for case in fixtures["polygons"].as_array().unwrap() {
        for scale in fixtures["scales"].as_array().unwrap() {
            let scale = scale.as_f64().unwrap();
            let source: Vec<[f64; 3]> = serde_json::from_value(case["vertices"].clone()).unwrap();
            let positions: Vec<_> = source.iter().map(|point| point.map(|coordinate| (coordinate * scale) as f32)).collect();
            let faces: Vec<Vec<u32>> = serde_json::from_value(case["faces"].clone()).unwrap();
            let mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
            let transfer = mesh.tessellate().unwrap();
            let mut area = 0.0;
            for triangle in transfer.indices.chunks_exact(3) {
                let point = |index: u32| (transfer.positions[index as usize * 3] as f64 / scale, transfer.positions[index as usize * 3 + 1] as f64 / scale, transfer.positions[index as usize * 3 + 2] as f64 / scale);
                let [a, b, c] = [point(triangle[0]), point(triangle[1]), point(triangle[2])];
                let signed_area = cross3(sub3(b, a), sub3(c, a)).2 / 2.0;
                assert!(signed_area > 0.0, "scale={scale}: inverted or collapsed triangle");
                area += signed_area;
            }
            assert!((area - case["area"].as_f64().unwrap()).abs() < 1e-6, "scale={scale}: area={area}");
            for normal in transfer.normals.chunks_exact(3) { assert!((normal[2] - 1.0).abs() < 1e-6, "scale={scale}: normal={normal:?}"); }
        }
    }
}

#[test]
fn modeling_fixtures_preserve_closed_oriented_surfaces() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for case in fixtures["cases"].as_array().unwrap() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        let selected: Vec<FaceId> = case["selection"].as_array().unwrap().iter().map(|id| FaceId(id.as_u64().unwrap() as u32)).collect();
        let amount = case["amount"].as_f64().unwrap() as f32;
        match case["operation"].as_str().unwrap() {
            "extrude" => mesh.extrude_faces(&selected, amount).unwrap(),
            "inset" => mesh.inset_faces(&selected, amount).unwrap(),
            "subdivide" => mesh.subdivide_faces(&selected).unwrap(),
            _ => panic!("unknown fixture operation"),
        }
        let label = case["name"].as_str().unwrap();
        assert_eq!(mesh.vertex_count(), case["vertices"].as_u64().unwrap() as usize, "{label}");
        assert_eq!(mesh.face_count(), case["faces"].as_u64().unwrap() as usize, "{label}");
        let mut incidence = HashMap::<(u32, u32), Vec<(u32, u32)>>::new();
        for face in 0..mesh.face_count() {
            let vertices = mesh.face_vertex_ids(FaceId(face as u32)).unwrap();
            for i in 0..vertices.len() {
                let a = vertices[i].0;
                let b = vertices[(i + 1) % vertices.len()].0;
                incidence.entry((a.min(b), a.max(b))).or_default().push((a, b));
            }
        }
        assert_eq!(incidence.values().filter(|uses| uses.len() == 1).count(), case["boundaryEdges"].as_u64().unwrap() as usize, "{label}");
        assert!(incidence.values().all(|uses| uses.len() == 2 && uses[0] == (uses[1].1, uses[1].0)), "{label}: winding or nonmanifold edge");
        let transfer = mesh.tessellate().unwrap();
        let mut area = 0.0f64;
        let mut volume = 0.0f64;
        for triangle in transfer.indices.chunks_exact(3) {
            let point = |index: u32| Vec3::new(transfer.positions[index as usize * 3], transfer.positions[index as usize * 3 + 1], transfer.positions[index as usize * 3 + 2]);
            let [a, b, c] = [point(triangle[0]), point(triangle[1]), point(triangle[2])];
            area += b.sub(a).cross(c.sub(a)).length() as f64 / 2.0;
            volume += a.dot(b.cross(c)) as f64 / 6.0;
        }
        assert!((area - case["area"].as_f64().unwrap()).abs() < 1e-5, "{label}: area {area}");
        assert!((volume - case["volume"].as_f64().unwrap()).abs() < 1e-5, "{label}: signed volume {volume}");
    }
}

#[test]
fn modeling_rejects_invalid_selection_without_mutation() {
    for operation in [HalfedgeMesh::extrude_faces, HalfedgeMesh::inset_faces] {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        let before = mesh.to_obj().unwrap();
        assert!(operation(&mut mesh, &[FaceId(0), FaceId(999)], 0.1).is_err());
        assert_eq!(mesh.to_obj().unwrap(), before);
        assert!(operation(&mut mesh, &[FaceId(0)], f32::NAN).is_err());
        assert_eq!(mesh.to_obj().unwrap(), before);
    }
}

#[test]
fn subdivision_preserves_concave_polygon_area() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for case in fixtures["polygons"].as_array().unwrap() {
        let positions: Vec<[f32; 3]> = serde_json::from_value(case["vertices"].clone()).unwrap();
        let faces: Vec<Vec<u32>> = serde_json::from_value(case["faces"].clone()).unwrap();
        let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
        mesh.subdivide_faces(&[FaceId(0)]).unwrap();
        assert_eq!(mesh.vertex_count(), case["expectedVertices"].as_u64().unwrap() as usize);
        assert_eq!(mesh.face_count(), case["expectedFaces"].as_u64().unwrap() as usize);
        let transfer = mesh.tessellate().unwrap();
        let mut area = 0.0;
        for triangle in transfer.indices.chunks_exact(3) {
            let point = |index: u32| Vec3::new(transfer.positions[index as usize * 3], transfer.positions[index as usize * 3 + 1], transfer.positions[index as usize * 3 + 2]);
            let [a, b, c] = [point(triangle[0]), point(triangle[1]), point(triangle[2])];
            let signed_area = b.sub(a).cross(c.sub(a)).z() / 2.0;
            assert!(signed_area > 0.0);
            area += signed_area;
        }
        assert!((area as f64 - case["area"].as_f64().unwrap()).abs() < 1e-5);
    }
}

fn assert_edit_surface(mesh: &HalfedgeMesh, expected_area: Option<f64>, expected_volume: Option<f64>) {
    let (positions, faces) = mesh.polygon_soup();
    let mut used = HashSet::new();
    let mut incidence = HashMap::<(u32, u32), (usize, i32)>::new();
    for face in faces {
        assert_eq!(face.iter().copied().collect::<HashSet<_>>().len(), face.len());
        for i in 0..face.len() {
            let (a, b) = (face[i], face[(i + 1) % face.len()]);
            used.insert(a);
            let entry = incidence.entry((a.min(b), a.max(b))).or_default();
            entry.0 += 1; entry.1 += if a < b { 1 } else { -1 };
        }
    }
    assert_eq!(used.len(), positions.len());
    assert!(incidence.values().all(|edge| edge.0 == 1 || *edge == (2, 0)));
    if expected_volume.is_some() { assert!(incidence.values().all(|edge| *edge == (2, 0))); }
    let transfer = mesh.tessellate().unwrap();
    let mut area = 0.0;
    let mut volume = 0.0;
    for indices in transfer.indices.chunks_exact(3) {
        let points: [_; 3] = std::array::from_fn(|i| {
            let offset = indices[i] as usize * 3;
            parry3d::na::Point3::new(transfer.positions[offset], transfer.positions[offset + 1], transfer.positions[offset + 2])
        });
        let triangle = parry3d::shape::Triangle::new(points[0], points[1], points[2]);
        let measured = triangle.area() as f64;
        assert!(measured > 0.0);
        area += measured;
        volume += points[0].coords.dot(&points[1].coords.cross(&points[2].coords)) as f64 / 6.0;
    }
    if let Some(expected) = expected_area { assert!((area - expected).abs() < 1e-5, "area={area}"); }
    if let Some(expected) = expected_volume { assert!((volume - expected).abs() < 1e-5, "volume={volume}, expected={expected}"); }
}

#[test]
fn bevel_fixtures_have_segments_and_closed_geometry() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for case in fixtures["bevels"].as_array().unwrap() {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        let pairs: Vec<[u32; 2]> = serde_json::from_value(case["edges"].clone()).unwrap();
        let edges = pairs.iter().map(|&[a,b]| EdgeId((0..mesh.halfedges.len()).find(|&id| mesh.edge_endpoints(EdgeId(id as u32)).unwrap() == (VertexId(a),VertexId(b))).unwrap() as u32)).collect::<Vec<_>>();
        mesh.bevel_edges(&edges, case["amount"].as_f64().unwrap() as f32, case["segments"].as_u64().unwrap() as u32).unwrap();
        assert_eq!(mesh.vertex_count(), case["vertices"].as_u64().unwrap() as usize);
        assert_eq!(mesh.face_count(), case["faces"].as_u64().unwrap() as usize);
        assert_edit_surface(&mesh, None, case["volume"].as_f64());
    }
}

#[test]
fn selected_vertex_weld_preserves_authored_surface() {
    use semio_framework_value::{AuthoringWallet,DslValue,retained_clone::{RetainedCloneGrant,RetainedCloneProgress}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🧲️selected-weld.json")).unwrap();
    let positions:Vec<[f32;3]>=serde_json::from_value(fixture["positions"].clone()).unwrap();
    let faces:Vec<Vec<u32>>=serde_json::from_value(fixture["faces"].clone()).unwrap();
    let normals:Vec<[f32;3]>=serde_json::from_value(fixture["normals"].clone()).unwrap();
    let uvs:Vec<[f32;2]>=serde_json::from_value(fixture["uvs"].clone()).unwrap();
    let selection:Vec<u32>=serde_json::from_value(fixture["selection"].clone()).unwrap();
    let face_indices:Vec<u32>=serde_json::from_value(fixture["faceIndices"].clone()).unwrap();
    let [a,b]:[u32;2]=serde_json::from_value(fixture["seam"].clone()).unwrap();
    let smooth=fixture["smoothFace"].as_u64().unwrap() as usize;
    let mut mesh=HalfedgeMesh::from_faces(&positions,&faces).unwrap();
    mesh.faces[smooth].smooth=true;
    for (vertex,normal) in mesh.vertices.iter_mut().zip(normals) {vertex.normal=Some(normal);}
    for (corner,uv) in mesh.halfedges.iter_mut().zip(uvs) {corner.uv=uv;}
    let edge=(0..mesh.halfedge_count()).map(|id|EdgeId(id as u32))
        .find(|&id|mesh.edge_endpoints(id).unwrap()==(VertexId(a),VertexId(b))).unwrap();
    mesh.mark_uv_seam(&[edge],true);
    mesh.set_attribute("linear".into(),MeshAttribute{domain:MeshAttributeDomain::Vertex,
        semantic:MeshAttributeSemantic::Custom,interpolation:MeshAttributeInterpolation::Linear,
        values:fixture["pointValues"].as_array().unwrap().iter().map(|v|DslValue::float(v.as_f64().unwrap())).collect(),indices:None}).unwrap();
    mesh.set_attribute("face".into(),MeshAttribute{domain:MeshAttributeDomain::Face,
        semantic:MeshAttributeSemantic::Custom,interpolation:MeshAttributeInterpolation::Constant,
        values:fixture["faceValues"].as_array().unwrap().iter().map(|v|DslValue::String(v.as_str().unwrap().into())).collect(),indices:Some(face_indices.clone())}).unwrap();

    let selected=selection.into_iter().map(VertexId).collect::<Vec<_>>();
    let mut job=mesh.merge_vertices_job_owned(&selected,WeldMode::ByDistance,fixture["threshold"].as_f64().unwrap() as f32).unwrap();
    let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:128,maximum_capacity_bytes:1<<24,maximum_release_bytes:1<<24,maximum_depth:128};
    let step=|job:&mut MeshModelingJob,grant|{
        let mut receipt=RetainedCloneProgress::default();
        let result=AuthoringWallet::new(grant,1<<20).run(grant,1,|effect|job.step(1,effect,&mut receipt)).unwrap().unwrap().output.unwrap();
        (result,receipt)
    };
    let mut copied=0;
    let mut refused=false;
    let mut output=None;
    for _ in 0..4096 {
        let demand=job.refined_surface_demand(grant.maximum_copy_bytes).unwrap();
        if let Some(demand)=demand.filter(|demand|demand.copy_bytes>0 && !refused) {
            let before=job.progress();
            let short=RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant};
            let (result,receipt)=step(&mut job,short);
            assert!(matches!(result,MeshModelingStep::Working(_)));
            assert_eq!(receipt,RetainedCloneProgress::default());
            assert_eq!(job.progress(),before);
            refused=true;
        }
        let (result,receipt)=step(&mut job,grant);
        if demand.is_some() {copied+=receipt.copied_bytes;}
        if let MeshModelingStep::Done(mesh)=result {output=Some(mesh);break;}
    }
    let mesh=output.expect("selected weld should finish");
    assert!(refused && copied>0,"no funded surface transfer");
    let expected=&fixture["expected"];
    assert_eq!(mesh.vertex_count() as u64,expected["vertexCount"].as_u64().unwrap());
    assert_eq!(mesh.face_count() as u64,expected["faceCount"].as_u64().unwrap());
    assert_eq!(mesh.faces.iter().map(|face|face.smooth).collect::<Vec<_>>(),vec![true,false]);
    assert_eq!(mesh.attributes()["face"].indices.as_ref(),Some(&face_indices));
    for sample in expected["points"].as_array().unwrap() {
        let position:[f32;3]=serde_json::from_value(sample["position"].clone()).unwrap();
        let normal:[f32;3]=serde_json::from_value(sample["normal"].clone()).unwrap();
        let uv:Vec<[f32;2]>=serde_json::from_value(sample["uvs"].clone()).unwrap();
        let vertex=mesh.vertices.iter().position(|vertex|vertex.position==position).unwrap();
        assert_eq!(mesh.vertices[vertex].normal,Some(normal));
        assert_eq!(mesh.attributes()["linear"].value_at(vertex).unwrap().as_f64(),sample["linear"].as_f64());
        assert_eq!(mesh.halfedges.iter().filter(|corner|corner.vertex==vertex as u32).map(|corner|corner.uv).collect::<Vec<_>>(),uv);
    }
    let seam=mesh.halfedges.iter().enumerate().filter(|(_,corner)|corner.twin.is_some()).collect::<Vec<_>>();
    assert_eq!(seam.len(),2);
    assert!(seam.iter().all(|(id,_)|mesh.uv_seams.contains(&(*id as u32))));
}

#[test]
fn bevel_preserves_authored_surface_through_all_profile_planes() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🪚️bevel.json")).unwrap();
    let positions: Vec<[f32;3]> = serde_json::from_value(fixture["positions"].clone()).unwrap();
    let faces: Vec<Vec<u32>> = serde_json::from_value(fixture["faces"].clone()).unwrap();
    let normals: Vec<[f32;3]> = serde_json::from_value(fixture["vertexNormals"].clone()).unwrap();
    let uvs: Vec<[f32;2]> = serde_json::from_value(fixture["uvs"].clone()).unwrap();
    let [a,b]: [u32;2] = serde_json::from_value(fixture["edge"].clone()).unwrap();
    let smooth = fixture["smoothFace"].as_u64().unwrap() as usize;
    let mut source = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
    source.faces[smooth].smooth = true;
    for (corner,uv) in source.halfedges.iter_mut().filter(|edge| edge.face == Some(smooth as u32)).zip(uvs) {
        corner.uv = uv;
    }
    for (vertex,normal) in source.vertices.iter_mut().zip(normals) {
        vertex.normal = Some(normal);
    }
    let edge = (0..source.halfedge_count()).map(|id| EdgeId(id as u32))
        .find(|&id| source.edge_endpoints(id).unwrap() == (VertexId(a),VertexId(b))).unwrap();
    for segments in [1,2] {
        let mut mesh = source.clone();
        mesh.bevel_edges(&[edge], fixture["amount"].as_f64().unwrap() as f32, segments).unwrap();
        assert_eq!(mesh.face_count(), faces.len()+segments as usize);
        assert_eq!(mesh.vertex_count(), positions.len()+segments as usize*2);
        assert_edit_surface(&mesh, None, (segments == 1).then(||fixture["volume"].as_f64().unwrap()));
        assert_eq!(mesh.faces.iter().filter(|face|face.smooth).count(), 1);
        for sample in ["retained","intersection"] {
            if sample == "intersection" && segments != 1 {continue;}
            let expected = &fixture[sample];
            let position: [f32;3] = serde_json::from_value(expected["position"].clone()).unwrap();
            let uv: [f32;2] = serde_json::from_value(expected["uv"].clone()).unwrap();
            let normal: [f32;3] = serde_json::from_value(expected["normal"].clone()).unwrap();
            let corner = mesh.halfedges.iter().find(|corner| mesh.faces[corner.face.unwrap() as usize].smooth &&
                mesh.vertices[corner.vertex as usize].position.iter().zip(position).all(|(a,b)|(a-b).abs()<1e-6)).unwrap();
            assert!(corner.uv.iter().zip(uv).all(|(a,b)|(a-b).abs()<1e-6));
            assert!(mesh.vertices[corner.vertex as usize].normal.unwrap().iter().zip(normal).all(|(a,b)|(a-b).abs()<1e-6));
        }
        for face in faces.len()..mesh.face_count() {
            assert!(!mesh.faces[face].smooth);
            let cap = mesh.halfedges.iter().enumerate().filter(|(_,edge)|edge.face==Some(face as u32)).collect::<Vec<_>>();
            assert!(cap.iter().all(|(id,edge)| mesh.uv_seams.contains(&(*id as u32)) &&
                mesh.uv_seams.contains(&edge.twin.unwrap())));
            assert!(cap.iter().any(|(_,edge)|edge.uv!=[0.0;2]));
        }
    }
}

#[test]
fn isolated_convex_bevel_preserves_a_concave_closed_prism() {
    use semio_framework_value::DslValue;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🪚️bevel.json")).unwrap();
    let case=&fixture["concave"];
    let points:Vec<[f32;3]>=serde_json::from_value(case["positions"].clone()).unwrap();
    let faces:Vec<Vec<u32>>=serde_json::from_value(case["faces"].clone()).unwrap();
    let find_edge=|mesh:&HalfedgeMesh,key:&str| {
        let [a,b]:[u32;2]=serde_json::from_value(case[key].clone()).unwrap();
        (0..mesh.halfedge_count()).map(|i|EdgeId(i as u32)).find(|&edge|
            mesh.edge_endpoints(edge).unwrap()==(VertexId(a),VertexId(b))).unwrap()
    };
    let mut source=HalfedgeMesh::from_faces(&points,&faces).unwrap();
    let amount=case["amount"].as_f64().unwrap() as f32;
    let selected=find_edge(&source,"edge");
    assert_eq!(selected.0 as u64,case["selection"][0].as_u64().unwrap());
    let expected=&case["expected"];
    let mut bare=source.clone();
    let twin=EdgeId(bare.halfedges[selected.0 as usize].twin.unwrap());
    bare.bevel_edges(&[twin],amount,1).unwrap();
    assert_edit_surface(&bare,expected["area"].as_f64(),expected["volume"].as_f64());
    source.faces[0].smooth=true;
    for v in &mut source.vertices {v.normal=Some([0.0,0.0,1.0]);}
    for edge in source.halfedges.iter_mut().filter(|edge|edge.face==Some(0)) {
        let [x,y,_]=points[edge.vertex as usize];edge.uv=[x/4.0,y/4.0];
    }
    let retained=find_edge(&source,"retainedSeam");
    source.mark_uv_seam(&[retained],true);
    let mut materials=source.materials().clone();
    materials.insert("paint".into(),DslValue::from(case["material"].clone()));
    materials.insert("trim".into(),DslValue::from(case["material"].clone()));
    source.set_surface_assets(materials,Default::default()).unwrap();
    source.set_attribute("material".into(),MeshAttribute{domain:MeshAttributeDomain::Face,
        semantic:MeshAttributeSemantic::Material,interpolation:MeshAttributeInterpolation::Constant,
        values:vec![DslValue::String("paint".into()),DslValue::String("trim".into())],
        indices:Some((0..faces.len()).map(|face|u32::from(face==7)).collect())}).unwrap();
    let before=source.to_json().unwrap();
    for (edge,width,reason) in [(find_edge(&source,"reflexEdge"),amount,"convex selected edge"),
        (find_edge(&source,"remoteCutEdge"),amount,"crosses its endpoint neighborhood"),
        (selected,case["tooWide"].as_f64().unwrap() as f32,"half the neighboring face clearance")] {
        assert!(matches!(source.bevel_edges(&[edge],width,1),Err(MeshKernelError::InvalidInput(message))
            if message.contains(reason)),"{reason}");
        assert_eq!(source.to_json().unwrap(),before);
    }
    assert!(matches!(source.bevel_edges_with_progress(&[selected],amount,2,|progress|progress==0.0),
        Err(MeshKernelError::InvalidInput(message)) if message.contains("cancelled")));
    assert_eq!(source.to_json().unwrap(),before);
    for segments in [1,2] {
        let mut result=source.clone();
        result.bevel_edges(&[selected],amount,segments).unwrap();
        assert_eq!(result.face_count() as u64,expected["faces"].as_u64().unwrap()+segments as u64-1);
        assert_eq!(result.vertex_count() as u64,expected["vertices"].as_u64().unwrap()+2*(segments as u64-1));
        assert_edit_surface(&result,(segments==1).then(||expected["area"].as_f64().unwrap()),
            (segments==1).then(||expected["volume"].as_f64().unwrap()));
        assert_eq!(result.materials().get("paint"),source.materials().get("paint"));
        assert_eq!(result.materials().get("trim"),source.materials().get("trim"));
        for face in 0..result.face_count(){
            assert_eq!(result.attributes()["material"].value_at(face).unwrap().as_str(),Some(if face==7 {"trim"} else {"paint"}));
        }
        assert!(result.faces[0].smooth && result.faces[faces.len()..].iter().all(|face|!face.smooth));
        let seam=result.halfedges.iter().enumerate().find(|(_,he)| {
            let a=result.vertices[he.vertex as usize].position;
            let b=result.vertices[result.halfedges[he.next as usize].vertex as usize].position;
            a==[0.0,3.0,0.0] && b==[0.0,3.0,2.0]
        }).unwrap();
        assert!(result.is_uv_seam(EdgeId(seam.0 as u32)) && result.is_uv_seam(EdgeId(seam.1.twin.unwrap())));
        for edge in result.halfedges.iter().filter(|he|he.face==Some(0)) {
            let vertex=&result.vertices[edge.vertex as usize];
            let [x,y,_]=vertex.position;
            assert!((edge.uv[0]-x/4.0).abs()<1e-5 && (edge.uv[1]-y/4.0).abs()<1e-5);
            assert_eq!(vertex.normal,Some([0.0,0.0,1.0]));
        }
    }
}

#[test]
fn boundary_vertex_dissolve_protects_both_authored_seam_segments() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🧵️dissolve-seam.json")).unwrap();
    let positions:Vec<[f32;3]>=serde_json::from_value(fixture["positions"].clone()).unwrap();
    let face:Vec<u32>=serde_json::from_value(fixture["face"].clone()).unwrap();
    let uvs:Vec<[f32;2]>=serde_json::from_value(fixture["cornerUvs"].clone()).unwrap();
    let normal:[f32;3]=serde_json::from_value(fixture["normal"].clone()).unwrap();
    let selected=VertexId(fixture["selection"].as_u64().unwrap() as u32);
    let mut source=HalfedgeMesh::from_faces(&positions,&[face]).unwrap();
    source.faces[0].smooth=true;
    for vertex in &mut source.vertices {vertex.normal=Some(normal);}
    for (corner,uv) in source.halfedges.iter_mut().zip(&uvs) {corner.uv=*uv;}

    let mark=|mesh:&mut HalfedgeMesh,key:&str|{
        let [a,b]:[u32;2]=serde_json::from_value(fixture[key].clone()).unwrap();
        let edge=(0..mesh.halfedge_count()).map(|id|EdgeId(id as u32))
            .find(|&id|mesh.edge_endpoints(id).unwrap()==(VertexId(a),VertexId(b))).unwrap();
        mesh.mark_uv_seam(&[edge],true);
    };
    for key in ["incomingSeam","outgoingSeam"] {
        let mut protected=source.clone();
        mark(&mut protected,key);
        let before=protected.to_json().unwrap();
        assert!(matches!(protected.dissolve_vertices(&[selected]),Err(MeshKernelError::InvalidInput(message))
            if message.contains("authored removed edge")),"{key} must survive exactly");
        assert_eq!(protected.to_json().unwrap(),before);
    }

    mark(&mut source,"unaffectedSeam");
    source.dissolve_vertices(&[selected]).unwrap();
    let retained:Vec<usize>=serde_json::from_value(fixture["expected"]["face"].clone()).unwrap();
    let expected_positions=retained.iter().map(|&id|positions[id]).collect::<Vec<_>>();
    assert_eq!(source.polygon_soup().0,expected_positions);
    assert!(source.faces[0].smooth);
    assert!(source.vertices.iter().all(|vertex|vertex.normal==Some(normal)));
    assert_eq!(source.halfedges.iter().map(|corner|corner.uv).collect::<Vec<_>>(),retained.iter().map(|&id|uvs[id]).collect::<Vec<_>>());
    let [a,b]:[usize;2]=serde_json::from_value(fixture["unaffectedSeam"].clone()).unwrap();
    let marked:Vec<_>=source.halfedges.iter().enumerate().filter(|&(id,_)|source.is_uv_seam(EdgeId(id as u32)))
        .map(|(_,edge)|[source.vertices[edge.vertex as usize].position,source.vertices[source.halfedges[edge.next as usize].vertex as usize].position]).collect();
    assert_eq!(marked,vec![[positions[a],positions[b]]]);
}

#[test]
fn bridge_joins_two_anchored_rims_and_preserves_authored_surface() {
    use semio_framework_value::DslValue;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🌉️bridge.json")).unwrap();
    let positions:Vec<[f32;3]>=serde_json::from_value(fixture["positions"].clone()).unwrap();
    let rims:Vec<Vec<u32>>=serde_json::from_value(fixture["rims"].clone()).unwrap();
    let anchors=|label:&str|serde_json::from_value::<[u32;2]>(fixture[label].clone()).unwrap().map(EdgeId);
    let uvs:Vec<[f32;2]>=serde_json::from_value(fixture["rimUvs"].clone()).unwrap();
    let mut mesh=HalfedgeMesh::from_faces(&positions,&rims).unwrap();
    mesh.faces[0].smooth=true;
    mesh.mark_uv_seam(&[EdgeId(0)],true);
    for vertex in &mut mesh.vertices {vertex.normal=Some([0.0,0.0,1.0]);}
    for (corner,uv) in mesh.halfedges.iter_mut().zip(&uvs) {corner.uv=*uv;}
    let old_uv=DslValue::Array(vec![DslValue::float(0.25),DslValue::float(0.5)]);
    mesh.set_attribute("uv".into(),MeshAttribute{domain:MeshAttributeDomain::Vertex,
        semantic:MeshAttributeSemantic::Uv,interpolation:MeshAttributeInterpolation::Linear,
        values:vec![old_uv.clone()],indices:Some(vec![0;positions.len()])}).unwrap();
    mesh.set_attribute("rim".into(),MeshAttribute{domain:MeshAttributeDomain::Face,
        semantic:MeshAttributeSemantic::Custom,interpolation:MeshAttributeInterpolation::Nearest,
        values:vec![DslValue::String("first".into()),DslValue::String("second".into())],indices:None}).unwrap();
    for label in ["sameLoop","misaligned"] {
        let before=mesh.to_json().unwrap();
        assert!(mesh.bridge_boundary_loops(anchors(label)).is_err(),"{label}");
        assert_eq!(mesh.to_json().unwrap(),before);
    }
    let expected=&fixture["expected"];
    let ambiguous=&fixture["ambiguous"];
    let extra:Vec<[f32;3]>=serde_json::from_value(ambiguous["positions"].clone()).unwrap();
    let branch:Vec<u32>=serde_json::from_value(ambiguous["face"].clone()).unwrap();
    let mut branched=HalfedgeMesh::from_faces(&[positions.clone(),extra].concat(),&[rims.clone(),vec![branch]].concat()).unwrap();
    let before=branched.to_json().unwrap();
    assert!(branched.bridge_boundary_loops(anchors("selection")).is_err());
    assert_eq!(branched.to_json().unwrap(),before);
    let walls:Vec<Vec<u32>>=serde_json::from_value(expected["walls"].clone()).unwrap();
    let tile:Vec<[f32;2]>=serde_json::from_value(expected["wallUv"].clone()).unwrap();
    mesh.bridge_boundary_loops(anchors("selection")).unwrap();
    assert_eq!(mesh.polygon_soup(),(positions,[rims,walls].concat()));
    assert_eq!(mesh.face_count() as u64,expected["faces"].as_u64().unwrap());
    assert_eq!(mesh.vertex_count() as u64,expected["vertices"].as_u64().unwrap());
    assert_edit_surface(&mesh,expected["area"].as_f64(),expected["volume"].as_f64());
    assert!(mesh.halfedges.iter().all(|edge|edge.twin.is_some()));
    assert!(mesh.vertices.iter().all(|vertex|vertex.normal==Some([0.0,0.0,1.0])));
    assert!(mesh.faces[0].smooth && mesh.faces[1..].iter().all(|face|!face.smooth));
    assert_eq!(mesh.halfedges[..uvs.len()].iter().map(|corner|corner.uv).collect::<Vec<_>>(),uvs);
    let active=&mesh.attributes()["uv"];
    assert_eq!(active.domain,MeshAttributeDomain::Corner);
    for corner in 0..8 {assert_eq!(active.value_at(corner),Some(&old_uv));}
    for (i,wall) in mesh.halfedges[8..].chunks_exact(4).enumerate(){
        assert_eq!(wall.iter().map(|corner|corner.uv).collect::<Vec<_>>(),tile);
        assert_eq!(mesh.attributes()["rim"].value_at(i+2).unwrap().as_str(),Some("first"));
        for (j,uv) in tile.iter().enumerate(){assert_eq!(mesh.corner_uv(EdgeId((8+i*4+j) as u32)).unwrap(),*uv);}
    }
    assert!(mesh.is_uv_seam(EdgeId(0)));
    assert!(mesh.is_uv_seam(EdgeId(mesh.halfedges[0].twin.unwrap())));

    let case=&fixture["unequal"];
    let points:Vec<[f32;3]>=serde_json::from_value(case["positions"].clone()).unwrap();
    let rims:Vec<Vec<u32>>=serde_json::from_value(case["rims"].clone()).unwrap();
    let walls:Vec<Vec<u32>>=serde_json::from_value(case["expected"]["walls"].clone()).unwrap();
    let anchors=serde_json::from_value::<[u32;2]>(case["selection"].clone()).unwrap().map(EdgeId);
    let tile:Vec<[f32;2]>=serde_json::from_value(case["wallUv"].clone()).unwrap();
    let mut unequal=HalfedgeMesh::from_faces(&points,&rims).unwrap();
    unequal.set_attribute("uv".into(),MeshAttribute{domain:MeshAttributeDomain::Vertex,
        semantic:MeshAttributeSemantic::Uv,interpolation:MeshAttributeInterpolation::Linear,
        values:vec![old_uv.clone()],indices:Some(vec![0;points.len()])}).unwrap();
    unequal.set_attribute("edgeOrigin".into(),MeshAttribute{domain:MeshAttributeDomain::Edge,
        semantic:MeshAttributeSemantic::Custom,interpolation:MeshAttributeInterpolation::Nearest,
        values:(0..7).map(|edge|DslValue::String(edge.to_string().into())).collect(),indices:None}).unwrap();
    unequal.mark_uv_seam(&[anchors[1]],true);
    unequal.bridge_boundary_loops(anchors).unwrap();
    assert_eq!(unequal.polygon_soup(),(points,[rims,walls.clone()].concat()));
    assert_edit_surface(&unequal,case["expected"]["area"].as_f64(),case["expected"]["volume"].as_f64());
    assert!(unequal.is_uv_seam(anchors[1]));
    assert!(unequal.is_uv_seam(EdgeId(unequal.halfedges[anchors[1].0 as usize].twin.unwrap())));
    let provenance=&unequal.attributes()["edgeOrigin"];
    let edge_origin=|corner:usize|provenance.value_at(corner).unwrap().as_str().unwrap().parse::<u32>().unwrap();
    for i in 0..7 {assert_eq!(edge_origin(i),i as u32);}
    let expected_edges:Vec<Vec<u32>>=serde_json::from_value(case["expected"]["edgeOrigins"].clone()).unwrap();
    let mut offset=7;
    for (wall,origins) in walls.iter().zip(expected_edges){
        for (i,uv) in tile.iter().take(wall.len()).enumerate(){
            assert_eq!(unequal.corner_uv(EdgeId((offset+i) as u32)).unwrap(),*uv);
            assert_eq!(edge_origin(offset+i),origins[i]);
        }
        offset+=wall.len();
    }
}

#[test]
fn boundary_extrusion_preserves_a_connected_authored_strip() {
    use semio_framework_value::{AuthoringWallet,DslValue,RetainedCloneGrant};
    let law:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/📏️boundary-extrude.json")).unwrap();
    let points:Vec<[f32;3]>=serde_json::from_value(law["positions"].clone()).unwrap();
    let faces:Vec<Vec<u32>>=serde_json::from_value(law["faces"].clone()).unwrap();
    let input=|row:&serde_json::Value|(
        serde_json::from_value::<Vec<u32>>(row["edges"].clone()).unwrap().into_iter().map(EdgeId).collect::<Vec<_>>(),
        Vec3(serde_json::from_value(row["offset"].clone()).unwrap()));
    let mut patch=HalfedgeMesh::from_faces(&points,&faces).unwrap();
    patch.faces[0].smooth=true;
    for v in &mut patch.vertices {v.normal=Some([0.0,0.0,1.0]);}
    let uvs:Vec<[f32;2]>=serde_json::from_value(law["sourceUvs"].clone()).unwrap();
    for (edge,uv) in patch.halfedges.iter_mut().zip(&uvs){edge.uv=*uv;}
    let seam=EdgeId(law["seam"].as_u64().unwrap() as u32);
    patch.mark_uv_seam(&[seam],true);
    let mut assets=patch.materials().clone();
    assets.insert("paint".into(),DslValue::from(serde_json::json!({"baseColor":[0.1,0.4,0.6,1.0]})));
    patch.set_surface_assets(assets,Default::default()).unwrap();
    patch.set_attribute("material".into(),MeshAttribute{domain:MeshAttributeDomain::Face,
        semantic:MeshAttributeSemantic::Material,interpolation:MeshAttributeInterpolation::Constant,
        values:vec![DslValue::String("paint".into())],indices:None}).unwrap();
    patch.set_attribute("selections".into(),MeshAttribute{domain:MeshAttributeDomain::Edge,
        semantic:MeshAttributeSemantic::Custom,interpolation:MeshAttributeInterpolation::Nearest,
        values:(0..4).map(DslValue::uint).collect(),indices:None}).unwrap();
    let authored=DslValue::Array(vec![DslValue::float(0.25),DslValue::float(0.5)]);
    patch.set_attribute("uv".into(),MeshAttribute{domain:MeshAttributeDomain::Vertex,
        semantic:MeshAttributeSemantic::Uv,interpolation:MeshAttributeInterpolation::Linear,
        values:vec![authored.clone()],indices:Some(vec![0;points.len()])}).unwrap();
    let before=patch.to_json().unwrap();
    for row in law["invalid"].as_array().unwrap(){
        let (ids,offset)=input(row);
        assert!(patch.extrude_boundary(&ids,offset).is_err(),"{row}");
        assert_eq!(patch.to_json().unwrap(),before);
    }
    let folded=&law["folded"];
    let folded_points:Vec<[f32;3]>=serde_json::from_value(folded["positions"].clone()).unwrap();
    let folded_face:Vec<u32>=serde_json::from_value(folded["face"].clone()).unwrap();
    assert!(matches!(HalfedgeMesh::from_faces(&folded_points,&[folded_face]).unwrap().extrude_boundary(&[EdgeId(0),EdgeId(1)],Vec3([0.0,0.0,1.0])),
        Err(MeshKernelError::InvalidInput(message)) if message.contains("folds")));
    let crossed=&law["crossed"];
    let crossed_points:Vec<[f32;3]>=serde_json::from_value(crossed["positions"].clone()).unwrap();
    let crossed_face:Vec<u32>=serde_json::from_value(crossed["face"].clone()).unwrap();
    assert!(matches!(HalfedgeMesh::from_faces(&crossed_points,&[crossed_face]).unwrap().extrude_boundary(&[EdgeId(0),EdgeId(1),EdgeId(2)],Vec3([0.0,0.0,1.0])),
        Err(MeshKernelError::InvalidInput(message)) if message.contains("crosses")));
    let interior=&law["interior"];
    let interior_faces:Vec<Vec<u32>>=serde_json::from_value(interior["faces"].clone()).unwrap();
    let interior_edge=EdgeId(interior["edge"].as_u64().unwrap() as u32);
    assert!(HalfedgeMesh::from_faces(&points,&interior_faces).unwrap().extrude_boundary(&[interior_edge],Vec3([0.0,0.0,1.0])).is_err());
    let branch=&law["branched"];
    let branch_points:Vec<[f32;3]>=serde_json::from_value(branch["points"].clone()).unwrap();
    let branch_face:Vec<u32>=serde_json::from_value(branch["face"].clone()).unwrap();
    let mut branched=HalfedgeMesh::from_faces(&[points.clone(),branch_points].concat(),&[faces.clone(),vec![branch_face]].concat()).unwrap();
    assert!(matches!(branched.extrude_boundary(&[EdgeId(0)],Vec3([0.0,0.0,1.0])),
        Err(MeshKernelError::InvalidInput(message)) if message.contains("ambiguous")));

    let tile:Vec<[f32;2]>=serde_json::from_value(law["wallUv"].clone()).unwrap();
    for case in law["cases"].as_array().unwrap(){
        let (ids,offset)=input(&case["request"]);
        let added:Vec<[f32;3]>=serde_json::from_value(case["addedPoints"].clone()).unwrap();
        let walls:Vec<Vec<u32>>=serde_json::from_value(case["walls"].clone()).unwrap();
        let origins:Vec<Vec<u64>>=serde_json::from_value(case["wallEdgeOrigins"].clone()).unwrap();
        let mut result=patch.clone();
        result.extrude_boundary(&ids,offset).unwrap();
        assert_eq!(result.polygon_soup(),([points.clone(),added].concat(),[faces.clone(),walls.clone()].concat()));
        assert_edit_surface(&result,case["area"].as_f64(),None);
        assert_eq!(result.halfedges.iter().filter(|edge|edge.twin.is_none()).count(),6);
        assert!(ids.iter().all(|id|result.halfedges[id.0 as usize].twin.is_some()));
        assert!(result.faces[0].smooth && result.faces[1..].iter().all(|face|!face.smooth));
        assert!(result.vertices.iter().all(|v|v.normal==Some([0.0,0.0,1.0])));
        assert_eq!(result.materials().get("paint"),patch.materials().get("paint"));
        assert!(result.is_uv_seam(seam));
        if let Some(twin)=result.halfedges[seam.0 as usize].twin{assert!(result.is_uv_seam(EdgeId(twin)));}
        for (i,uv) in uvs.iter().enumerate(){
            assert_eq!(result.halfedges[i].uv,*uv);
            assert_eq!(result.attributes()["uv"].value_at(i),Some(&authored));
        }
        for (face,row) in origins.iter().enumerate(){
            assert_eq!(result.attributes()["material"].value_at(face+1).unwrap().as_str(),Some("paint"));
            for (corner,origin) in row.iter().enumerate(){
                let vertex=4+face*4+corner;
                assert_eq!(result.corner_uv(EdgeId(vertex as u32)).unwrap(),tile[corner]);
                assert_eq!(result.attributes()["selections"].value_at(vertex).unwrap().as_u64(),Some(*origin));
            }
        }
    }

    let (ids,offset)=input(&law["cases"][0]["request"]);
    let mut job=patch.extrude_boundary_job(&ids,offset).unwrap();
    let stock=RetainedCloneGrant{maximum_items:2000,maximum_copy_bytes:1<<18,maximum_capacity_bytes:1<<22,maximum_release_bytes:1<<22,maximum_depth:256};
    let mut wallet=AuthoringWallet::new(stock,1<<24);
    let grant=RetainedCloneGrant{maximum_items:1,..stock};
    let step=|job:&mut MeshModelingJob,wallet:&mut AuthoringWallet| {
        let mut receipt=Default::default();
        let turn=wallet.run_invoked(grant.restricted_by(wallet.remaining()),1<<16,|effect|job.step(1,effect,&mut receipt)).unwrap().unwrap().output.unwrap();
        wallet.post(receipt).unwrap();turn
    };
    loop{
        match step(&mut job,&mut wallet){
            MeshModelingStep::Working(progress) if progress.phase=="boundary-extrude-walls"=>break,
            MeshModelingStep::Working(_)=>{},
            _=>panic!("cancel fixture must reach the retained wall phase"),
        }
    }
    job.cancel();
    assert!(matches!(step(&mut job,&mut wallet),MeshModelingStep::Cancelled(_)));
    assert_eq!(patch.to_json().unwrap(),before);
}

#[test]
fn dissolve_and_merge_fixtures_preserve_surface() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for group in ["dissolutions", "merges"] {
        for case in fixtures[group].as_array().unwrap() {
            let positions: Vec<[f32;3]> = serde_json::from_value(case["mesh"]["vertices"].clone()).unwrap();
            let faces: Vec<Vec<u32>> = serde_json::from_value(case["mesh"]["faces"].clone()).unwrap();
            let selected: Vec<u32> = serde_json::from_value(case["selection"].clone()).unwrap();
            let selected = selected.into_iter().map(VertexId).collect::<Vec<_>>();
            let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
            if group == "dissolutions" { mesh.dissolve_vertices(&selected).unwrap(); }
            else { mesh.merge_vertices(&selected, if case["mode"] == "distance" { WeldMode::ByDistance } else { WeldMode::First }, case["threshold"].as_f64().unwrap() as f32).unwrap(); }
            assert_eq!(mesh.vertex_count(), case["vertices"].as_u64().unwrap() as usize);
            assert_eq!(mesh.face_count(), case["faces"].as_u64().unwrap() as usize);
            assert_edit_surface(&mesh, case["area"].as_f64(), None);
        }
    }
}

#[test]
fn proportional_and_snap_fixtures_are_atomic_and_update_normals() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for group in ["proportional", "snapping"] {
        let case = &fixtures[group];
        let positions: Vec<[f32;3]> = serde_json::from_value(case["mesh"]["vertices"].clone()).unwrap();
        let faces: Vec<Vec<u32>> = serde_json::from_value(case["mesh"]["faces"].clone()).unwrap();
        let selected: Vec<u32> = serde_json::from_value(case["selection"].clone()).unwrap();
        let selected = selected.into_iter().map(VertexId).collect::<Vec<_>>();
        let mut mesh = HalfedgeMesh::from_faces(&positions, &faces).unwrap();
        let before = mesh.to_obj().unwrap();
        if group == "snapping" {
            assert!(mesh.snap_vertices_to_grid(&[VertexId(0), VertexId(999)], 1.0).is_err());
            assert_eq!(mesh.to_obj().unwrap(), before);
            assert!(mesh.snap_vertices_to_grid(&selected, f32::NAN).is_err());
            mesh.snap_vertices_to_grid(&selected, case["grid"].as_f64().unwrap() as f32).unwrap();
        } else {
            let delta = Vec3(serde_json::from_value(case["delta"].clone()).unwrap());
            let pivot = Vec3(serde_json::from_value(case["pivot"].clone()).unwrap());
            assert!(mesh.move_vertices_proportional(&[VertexId(0), VertexId(999)], delta, pivot, 1.0).is_err());
            assert_eq!(mesh.to_obj().unwrap(), before);
            mesh.move_vertices_proportional(&selected, delta, pivot, case["radius"].as_f64().unwrap() as f32).unwrap();
        }
        let expected: Vec<[f32;3]> = serde_json::from_value(case["expected"].clone()).unwrap();
        assert_eq!(mesh.polygon_soup().0, expected);
        let rebuilt = HalfedgeMesh::from_faces(&expected, &faces).unwrap();
        assert_eq!(mesh.tessellate().unwrap().normals, rebuilt.tessellate().unwrap().normals);
    }
}

#[test]
fn adjacent_bevels_and_duplicate_selections_preserve_manifold() {
    for edges in [vec![EdgeId(0), EdgeId(1)], vec![EdgeId(0), EdgeId(0)], vec![EdgeId(0), EdgeId(1), EdgeId(2)]] {
        let mut mesh = HalfedgeMesh::box_prim(1.0, 1.0, 1.0).unwrap();
        mesh.bevel_edges(&edges,0.1,3).unwrap();
        assert_edit_surface(&mesh,None,None);
        assert!(mesh.halfedges.iter().all(|edge| edge.twin.is_some()));
    }
}

#[test]
fn bevel_and_decimate_cancel_and_reject_invalid_input_atomically() {
    let mut mesh = HalfedgeMesh::box_prim(1.0,1.0,1.0).unwrap();
    let before = mesh.to_obj().unwrap();
    for (edges,width,segments) in [(vec![EdgeId(999)],0.1,1),(vec![EdgeId(0)],f32::NAN,1),(vec![EdgeId(0)],0.1,0),(vec![EdgeId(0)],0.1,65),(vec![EdgeId(0)],0.9,1)] {
        assert!(mesh.bevel_edges(&edges,width,segments).is_err());
        assert_eq!(mesh.to_obj().unwrap(),before);
    }
    let mut updates = Vec::new();
    assert!(mesh.bevel_edges_with_progress(&[EdgeId(0)],0.1,4,|fraction| { updates.push(fraction); fraction < 0.5 }).is_err());
    assert!(updates.len() > 1);
    assert_eq!(mesh.to_obj().unwrap(),before);
    assert!(mesh.decimate_with_progress(0.5,|_| false).is_err());
    assert_eq!(mesh.to_obj().unwrap(),before);
    assert!(mesh.mirror(MirrorAxis::X,0.0).is_err());
    assert_eq!(mesh.to_obj().unwrap(),before);
    assert!(mesh.dissolve_edges(&[EdgeId(0),EdgeId(999)]).is_err());
    assert_eq!(mesh.to_obj().unwrap(),before);
}

#[test]
fn mirror_and_decimate_fixtures_are_compact_and_oriented() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for case in fixtures["mirrors"].as_array().unwrap() {
        let mut mesh = HalfedgeMesh::box_prim(1.0,1.0,1.0).unwrap();
        mesh.translate(Vec3(serde_json::from_value(case["translation"].clone()).unwrap())).unwrap();
        mesh.mirror(MirrorAxis::X,case["threshold"].as_f64().unwrap() as f32).unwrap();
        assert_eq!(mesh.vertex_count(),case["vertices"].as_u64().unwrap() as usize);
        assert_eq!(mesh.face_count(),case["faces"].as_u64().unwrap() as usize);
        assert_edit_surface(&mesh,case["area"].as_f64(),case["volume"].as_f64());
    }
    for case in fixtures["decimations"].as_array().unwrap() {
        let mut mesh = HalfedgeMesh::box_prim(1.0,1.0,1.0).unwrap();
        mesh.decimate(case["ratio"].as_f64().unwrap() as f32).unwrap();
        assert!(mesh.vertex_count() <= case["maximumVertices"].as_u64().unwrap() as usize);
        assert_edit_surface(&mesh,None,None);
        assert!(mesh.halfedges.iter().all(|edge| edge.twin.is_some()));
        let transfer = mesh.tessellate().unwrap();
        let points = transfer.positions.chunks_exact(3).map(|p| parry3d::na::Point3::new(p[0],p[1],p[2])).collect::<Vec<_>>();
        let indices = transfer.indices.chunks_exact(3).map(|i| [i[0],i[1],i[2]]).collect::<Vec<_>>();
        let oracle = parry3d::shape::TriMesh::new(points,indices);
        use parry3d::shape::Shape;
        assert!(oracle.mass_properties(1.0).mass() > case["minimumVolume"].as_f64().unwrap() as f32);
    }
}

#[test]
fn smooth_normal_fixture_excludes_flat_faces_and_face_order() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    let case = &fixtures["normals"];
    let positions: Vec<[f32;3]> = serde_json::from_value(case["mesh"]["vertices"].clone()).unwrap();
    let faces: Vec<Vec<u32>> = serde_json::from_value(case["mesh"]["faces"].clone()).unwrap();
    let smooth: Vec<usize> = serde_json::from_value(case["smoothFaces"].clone()).unwrap();
    let expected: [f32;3] = serde_json::from_value(case["expected"].clone()).unwrap();
    let mut sum = parry3d::na::Vector3::zeros();
    for &id in &smooth {
        let points = faces[id].iter().map(|&id| parry3d::na::Point3::from(positions[id as usize])).collect::<Vec<_>>();
        sum += (points[1]-points[0]).cross(&(points[2]-points[0])).normalize();
    }
    let oracle = sum.normalize();
    for order in [[0,1,2],[2,1,0],[1,0,2]] {
        let permuted = order.iter().map(|&id| faces[id].clone()).collect::<Vec<_>>();
        let mut mesh = HalfedgeMesh::from_faces(&positions,&permuted).unwrap();
        let selected = order.iter().enumerate().filter_map(|(id,source)| smooth.contains(source).then_some(FaceId(id as u32))).collect::<Vec<_>>();
        mesh.set_shading(&selected,true).unwrap();
        let normal = mesh.vertices[case["vertex"].as_u64().unwrap() as usize].normal.unwrap();
        for axis in 0..3 { assert!((normal[axis]-expected[axis]).abs() < 1e-6); assert!((normal[axis]-oracle[axis]).abs() < 1e-6); }
    }
}

#[test]
fn retained_modeling_jobs_slice_work_and_match_synchronous_geometry() {
    let fixtures: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🛠️modeling/🔣️.json")).unwrap();
    for case in fixtures["jobs"].as_array().unwrap() {
        let source = HalfedgeMesh::box_prim(1.0,1.0,1.0).unwrap();
        let before = source.to_obj().unwrap();
        let make_job = || if case["operation"] == "bevel" { source.bevel_job(&[EdgeId(0)],case["amount"].as_f64().unwrap() as f32,case["segments"].as_u64().unwrap() as u32).unwrap() } else { source.decimate_job(case["ratio"].as_f64().unwrap() as f32).unwrap() };
        let mut synchronous = source.clone();
        if case["operation"] == "bevel" { synchronous.bevel_edges(&[EdgeId(0)],case["amount"].as_f64().unwrap() as f32,case["segments"].as_u64().unwrap() as u32).unwrap(); } else { synchronous.decimate(case["ratio"].as_f64().unwrap() as f32).unwrap(); }
        let mut sliced = make_job();
        let initial = sliced.progress();
        assert!(matches!(funded(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},|effect|sliced.step(0,effect,&mut semio_framework_value::retained_clone::RetainedCloneProgress::default())).unwrap(),MeshModelingStep::Working(_)));
        assert_eq!(sliced.progress(),initial);
        let mut calls = 0;
        let output = loop {
            let before = sliced.progress().units_done;
            calls += 1;
            match funded(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},|effect|sliced.step(1,effect,&mut semio_framework_value::retained_clone::RetainedCloneProgress::default())).unwrap() {
                MeshModelingStep::Working(progress) => {
                    assert_eq!(progress.units_done,before+1);
                    assert!(progress.units_done < progress.units_total);
                }
                MeshModelingStep::Done(mesh) => break mesh,
                MeshModelingStep::Cancelled(_) => panic!("unexpected cancellation"),
            }
            assert!(calls < 10000);
        };
        assert!(calls >= case["minimumSteps"].as_u64().unwrap() as usize);
        assert_eq!(output.to_obj().unwrap(),synchronous.to_obj().unwrap());
        assert_edit_surface(&output,None,case["volume"].as_f64());
        let transfer = output.tessellate().unwrap();
        let (positions,faces) = output.polygon_soup();
        let rebuilt = HalfedgeMesh::from_faces(&positions,&faces).unwrap().tessellate().unwrap();
        assert_eq!(transfer.normals,rebuilt.normals);
        let points = transfer.positions.chunks_exact(3).map(|p| parry3d::na::Point3::new(p[0],p[1],p[2])).collect::<Vec<_>>();
        let indices = transfer.indices.chunks_exact(3).map(|i| [i[0],i[1],i[2]]).collect::<Vec<_>>();
        use parry3d::shape::Shape;
        let volume = parry3d::shape::TriMesh::new(points,indices).mass_properties(1.0).mass() as f64;
        if let Some(expected) = case["volume"].as_f64() { assert!((volume-expected).abs() < 1e-6); }
        if let Some(minimum) = case["minimumVolume"].as_f64() { assert!(volume > minimum); }
        assert_eq!(source.to_obj().unwrap(),before);
        assert_eq!(sliced.progress().units_done,sliced.progress().units_total);
        sliced.cancel();
        assert!(funded(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},|effect|sliced.step(1,effect,&mut semio_framework_value::retained_clone::RetainedCloneProgress::default())).is_err());
        let mut cancelled = make_job();
        assert!(matches!(funded(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},|effect|cancelled.step(1,effect,&mut semio_framework_value::retained_clone::RetainedCloneProgress::default())).unwrap(),MeshModelingStep::Working(_)));
        let progress = cancelled.progress();
        cancelled.cancel();
        assert!(matches!(funded(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},|effect|cancelled.step(1,effect,&mut semio_framework_value::retained_clone::RetainedCloneProgress::default())).unwrap(),MeshModelingStep::Cancelled(value) if value.units_done == progress.units_done));
        assert_eq!(source.to_obj().unwrap(),before);
        let mut batched = make_job();
        let output = loop { match funded(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},|effect|batched.step(3,effect,&mut semio_framework_value::retained_clone::RetainedCloneProgress::default())).unwrap() { MeshModelingStep::Done(mesh) => break mesh, MeshModelingStep::Working(_) => {}, MeshModelingStep::Cancelled(_) => panic!("unexpected cancellation") } };
        assert_eq!(output.to_obj().unwrap(),synchronous.to_obj().unwrap());
        for phase in case["cancelPhases"].as_array().unwrap() {
            let phase = phase.as_str().unwrap();
            let mut job = make_job();
            while job.progress().phase != phase {
                assert!(matches!(funded(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},|effect|job.step(1,effect,&mut semio_framework_value::retained_clone::RetainedCloneProgress::default())).unwrap(),MeshModelingStep::Working(_)),"missing phase {phase}");
            }
            let budget = case["sliceEvidence"]["cancelPhaseUnits"].as_u64().unwrap() as usize;
            assert!(matches!(funded(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},|effect|job.step(budget,effect,&mut semio_framework_value::retained_clone::RetainedCloneProgress::default())).unwrap(), MeshModelingStep::Working(_)));
            let held = job.progress();
            job.cancel();
            assert!(matches!(funded(semio_framework_value::retained_clone::RetainedCloneGrant {maximum_items:usize::MAX,maximum_copy_bytes:128,maximum_capacity_bytes:usize::MAX,maximum_release_bytes:usize::MAX,maximum_depth:usize::MAX},|effect|job.step(1000,effect,&mut semio_framework_value::retained_clone::RetainedCloneProgress::default())).unwrap(),MeshModelingStep::Cancelled(progress) if progress == held));
            assert_eq!(source.to_obj().unwrap(),before);
        }
    }
}
