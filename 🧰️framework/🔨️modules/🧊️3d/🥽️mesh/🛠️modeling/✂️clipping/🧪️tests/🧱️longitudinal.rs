//! 🧱️ Verifies retained longitudinal clipping through the public Mesh geometry API.

use semio_framework_3d_mesh_clipping::{MeshConvexIntersection,MeshConvexIntersectionStep,MeshConvexUnion,MeshIntersectionFace};
use semio_framework_3d_mesh_contract::{MeshPolygonSoup as Soup,MeshSourceVertex as SourceVertex,Vec3};

#[test]
fn difference_keeps_a_strictly_nested_concave_prism() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🛠️modeling/🕳️nested-prismatic-difference.json")).unwrap();
    let stock = || Soup {
        positions: serde_json::from_value(law["stock"]["positions"].clone()).unwrap(),
        faces: serde_json::from_value(law["stock"]["faces"].clone()).unwrap(),
        normals: Vec::new(),
    };
    let prism_from = |spec: &serde_json::Value, shift: f32| {
        let profile: Vec<[f32; 2]> = serde_json::from_value(spec["footprint"].clone()).unwrap();
        let heights: [f32; 2] = serde_json::from_value(law["height"].clone()).unwrap();
        let positions = heights.into_iter().flat_map(|z| profile.iter().map(move |&[x, y]| [x + shift, y, z])).collect();
        Soup { positions, faces: serde_json::from_value(spec["faces"].clone()).unwrap(), normals: Vec::new() }
    };
    let cutter = |shift| prism_from(&law, shift);
    let tool = cutter(0.0);
    let mut job = MeshConvexIntersection::difference(stock(), cutter(0.0)).unwrap();
    let result = loop {
        assert!(job.progress().1 < 25_000);
        match job.advance().unwrap() {
            MeshConvexIntersectionStep::Working => continue,
            MeshConvexIntersectionStep::Solid(result) => break result,
            _ => panic!("expected the outer boundary and original reversed cutter"),
        }
    };
    assert_eq!(result.geometry.positions.len() as u64, law["vertices"].as_u64().unwrap());
    assert_eq!(result.faces.len(), stock().faces.len() + tool.faces.len());
    let mut edges = std::collections::BTreeMap::new();
    let mut volume = 0.0;
    for (ring, source) in result.geometry.faces.iter().zip(result.faces.iter()) {
        for (&a, &b) in ring.iter().zip(ring.iter().cycle().skip(1)) {
            edges.entry((a.min(b), a.max(b))).or_insert([0; 2])[usize::from(a < b)] += 1;
        }
        let vertices = |id: u32| Vec3(result.geometry.positions[id as usize]);
        for i in 1..ring.len() - 1 {
            let [a, b, c] = [ring[0], ring[i], ring[i + 1]].map(vertices);
            volume += f64::from(a.dot(b.cross(c))) / 6.0;
        }
        if let MeshIntersectionFace::ReversedRightPlane(face) = source {
            let expected = tool.faces[*face as usize].iter().rev().map(|&index| tool.positions[index as usize]).collect::<Vec<_>>();
            let actual = ring.iter().map(|&index| result.geometry.positions[index as usize]).collect::<Vec<_>>();
            assert_eq!(actual, expected);
        } else {
            assert!(matches!(source, MeshIntersectionFace::Left(_)));
        }
    }
    assert!(edges.values().all(|uses| *uses == [1, 1]));
    assert_eq!(edges.len() as u64, law["edges"].as_u64().unwrap());
    assert_eq!(result.geometry.positions.len() + result.faces.len() - edges.len(), 2 * law["boundaries"].as_u64().unwrap() as usize);
    assert!((volume - law["volume"].as_f64().unwrap()).abs() < 1e-6);
    for tool_first in [false, true] {
        let operands = || if tool_first { (cutter(0.0), stock()) } else { (stock(), cutter(0.0)) };
        let (left, right) = operands();
        let mut intersection = MeshConvexIntersection::new(left, right).unwrap();
        let inside = loop {
            match intersection.advance().unwrap() {
                MeshConvexIntersectionStep::Working => continue,
                MeshConvexIntersectionStep::Solid(result) => break result,
                _ => panic!("expected the original concave operand"),
            }
        };
        assert_eq!(inside.retained_operand, Some(u8::from(!tool_first)));
        assert_eq!(inside.geometry.positions, tool.positions);
        assert_eq!(inside.geometry.faces, tool.faces);
        let (left, right) = operands();
        let mut union = MeshConvexUnion::new(left, right).unwrap();
        let outside = loop {
            match union.advance().unwrap() {
                MeshConvexIntersectionStep::Working => continue,
                MeshConvexIntersectionStep::Solid(result) => break result,
                _ => panic!("expected the original convex operand"),
            }
        };
        assert_eq!(outside.retained_operand, Some(u8::from(tool_first)));
        assert_eq!(outside.geometry.positions, stock().positions);
        assert_eq!(outside.geometry.faces, stock().faces);
    }
    let mut removed = MeshConvexIntersection::difference(cutter(0.0), stock()).unwrap();
    loop {
        match removed.advance().unwrap() {
            MeshConvexIntersectionStep::Working => continue,
            MeshConvexIntersectionStep::Empty => break,
            _ => panic!("the containing solid removes the original prism"),
        }
    }
    let mut placed = cutter(0.0);
    let rotation: [f64; 3] = serde_json::from_value(law["placed"]["angles"].clone()).unwrap();
    let origin: [f64; 3] = serde_json::from_value(law["placed"]["origin"].clone()).unwrap();
    let target: [f64; 3] = serde_json::from_value(law["placed"]["target"].clone()).unwrap();
    for point in &mut placed.positions {
        let mut translated = std::array::from_fn::<_, 3, _>(|axis| f64::from(point[axis]) - origin[axis]);
        for (axis, angle) in rotation.into_iter().enumerate() {
            let (u, v) = ((axis + 1) % 3, (axis + 2) % 3);
            let (sine, cosine) = angle.to_radians().sin_cos();
            (translated[u], translated[v]) = (cosine * translated[u] - sine * translated[v], sine * translated[u] + cosine * translated[v]);
        }
        *point = std::array::from_fn(|axis| (translated[axis] + target[axis]) as f32);
    }
    let placed_points = placed.positions.clone();
    let placed_faces = placed.faces.len();
    let mut placed_cut = MeshConvexIntersection::difference(stock(), placed).unwrap();
    let result = loop {
        match placed_cut.advance().unwrap() {
            MeshConvexIntersectionStep::Working => continue,
            MeshConvexIntersectionStep::Solid(result) => break result,
            _ => panic!("expected the original placed cut boundary"),
        }
    };
    assert_eq!(&result.geometry.positions[stock().positions.len()..], placed_points.as_slice());
    assert_eq!(result.faces.iter().filter(|face| matches!(face, MeshIntersectionFace::ReversedRightPlane(_))).count(), placed_faces);
    let mut slab=stock();
    let [[a,b,c],[d,e,f]]:[[f32;3];2]=serde_json::from_value(law["clipped"]["bounds"].clone()).unwrap();
    slab.positions=vec![[a,b,c],[d,b,c],[d,e,c],[a,e,c],[a,b,f],[d,b,f],[d,e,f],[a,e,f]];
    for prism_first in [false,true] {
        let slab=Soup{positions:slab.positions.clone(),faces:slab.faces.clone(),normals:Vec::new()};
        let (left,right)=if prism_first{(cutter(0.0),slab)}else{(slab,cutter(0.0))};
        let mut clipped=MeshConvexIntersection::new(left,right).unwrap();
        let output=loop{
            match clipped.advance().unwrap(){
                MeshConvexIntersectionStep::Working=>continue,
                MeshConvexIntersectionStep::Solid(output)=>break output,
                _=>panic!("a transverse cap retains part of the concave prism"),
            }
        };
        let [stock_faces,prism_faces]=output.faces.iter().fold([0,0],|mut counts,face|{
            let right=matches!(face,MeshIntersectionFace::RightPlane(_));
            counts[usize::from(right!=prism_first)]+=1;
            counts
        });
        assert_eq!([stock_faces,prism_faces],[1,7]);
        let min=output.geometry.faces.iter().flatten().map(|&id|output.geometry.positions[id as usize][2]).fold(f32::INFINITY,f32::min);
        assert_eq!(f64::from(min),law["clipped"]["capHeight"].as_f64().unwrap());
        let mut volume=0.0;
        let mut area=0.0;
        for ring in &output.geometry.faces{
            let point=|id:u32|Vec3(output.geometry.positions[id as usize]);
            let first=point(ring[0]);
            let mut normal=Vec3([0.;3]);
            for pair in ring[1..].windows(2){
                let (a,b)=(point(pair[0]),point(pair[1]));
                normal=normal.add(a.sub(first).cross(b.sub(first)));
                volume+=f64::from(first.dot(a.cross(b)))/6.0;
            }
            area+=f64::from(normal.dot(normal)).sqrt()/2.0;
        }
        assert!((volume-law["clipped"]["volume"].as_f64().unwrap()).abs()<1e-6);
        assert!((area-law["clipped"]["area"].as_f64().unwrap()).abs()<1e-6);
    }
    let mut slab=stock();
    let [[a,b,c],[d,e,f]]:[[f32;3];2]=serde_json::from_value(law["split"]["bounds"].clone()).unwrap();
    slab.positions=vec![[a,b,c],[d,b,c],[d,e,c],[a,e,c],[a,b,f],[d,b,f],[d,e,f],[a,e,f]];
    let mut split=MeshConvexIntersection::difference(cutter(0.0),slab).unwrap();
    let output=loop{
        match split.advance().unwrap(){
            MeshConvexIntersectionStep::Working=>continue,
            MeshConvexIntersectionStep::Solid(output)=>break output,
            _=>panic!("a transverse cutter leaves two separated prism pieces"),
        }
    };
    let planes=output.faces.iter().filter_map(|face| match face{MeshIntersectionFace::ReversedRightPlane(id)=>Some(*id),_=>None}).collect::<Vec<_>>();
    assert_eq!(planes.len(),2);
    assert_ne!(planes[0],planes[1]);
    assert_eq!(output.geometry.faces.len(),2*tool.faces.len());
    let all_edges=output.geometry.faces.iter().flat_map(|ring|ring.iter().copied().zip(ring.iter().copied().cycle().skip(1)));
    let mut edges=std::collections::BTreeMap::new();
    for (a,b) in all_edges{edges.entry((a.min(b),a.max(b))).or_insert([0;2])[usize::from(a<b)]+=1;}
    assert!(edges.values().all(|uses|*uses==[1,1]));
    let vertices=edges.keys().flat_map(|&(a,b)|[a,b]).collect::<std::collections::BTreeSet<_>>();
    assert_eq!(vertices.len()+output.geometry.faces.len()-edges.len(),4);
    for name in ["lateral","longitudinalSlab","mixedSlab","mixedOpenSlab","independentWalls","fourArmWalls","disjointSupport","disjointSupportInverse","disjointSupportWithCaps", "disjointAndSingleSupports", "grazingSupport", "grazingEdge", "reflexSupport", "reflexSupportInverse", "reflexSupportWithCaps", "reflexSupportOpenTail", "reflexJoinsBothTails", "reflexJoinsOneTail", "multipleReflexContacts", "reflexOpposedBaseBand", "reflexOpposedTipBand", "reflexOpposedBaseTails", "reflexOpposedTipTails", "reflexNonparallelSupports", "reflexOnBothSupports", "reflexOnBothSupportsWithTails", "joinedReflexRight", "joinedReflexLeft", "joinedReflexDiagonal", "joinedReflexRightSlab", "joinedReflexDiagonalTail", "joinedReflexLeftTail", "joinedInteriorSlab", "joinedInteriorDiagonalSlab", "joinedInteriorStationaryCap", "joinedInteriorStockCaps", "joinedThreeSupportSlab", "joinedThreeSupportStockCaps", "threeNonparallelSlab", "threeNonparallelStockCaps", "threeNonparallelStationaryCap", "fourInteriorSupportsSlab", "fourInteriorSupportsStockCaps", "fourInteriorSupportsRotatedPocket", "pentagonalInteriorSlab", "pentagonalInteriorStockCaps", "hexagonalInteriorPocket", "maskedReflexHexagonalPocket", "maskedCoincidentTriangle","cornerSlab"]{
        let lateral=&law[name];
        let walls=lateral["walls"].as_array().unwrap();
        let diagonal=lateral["frame"]=="diagonal";
        let [[x,y,z],[a,b,c]]:[[f32;3];2]=serde_json::from_value(lateral["bounds"].clone()).unwrap();
        let lateral_tool=|| {
            if lateral["cutterProfile"].is_array() {
                let profile:Vec<[f32;2]>=serde_json::from_value(lateral["cutterProfile"].clone()).unwrap();
                return Soup { positions:[z,c].into_iter().flat_map(|z| profile.iter().map(move |&[x,y]| [x,y,z])).collect(),
                    faces:serde_json::from_value(lateral["cutterFaces"].clone()).unwrap(), normals:Vec::new() };
            }
            Soup {
                positions:[[x,y,z],[a,y,z],[a,b,z],[x,b,z],[x,y,c],[a,y,c],[a,b,c],[x,b,c]].into_iter()
                    .map(|[u,v,w]|if diagonal{[(u+v)/2.,(v-u)/2.,w]}else{[u,v,w]}).collect(),
                faces:stock().faces,normals:Vec::new(),
            }
        };
        for (reversed,subtract) in [(false,false),(true,false),(false,true)]{
            if subtract && lateral["difference"].is_null(){continue;}
            let profile = if lateral["footprint"].is_array() { lateral } else { &law };
            let prism = prism_from(profile, 0.);
            let (left,right)=if reversed{(lateral_tool(),prism)}else{(prism,lateral_tool())};
            let mut job=if subtract{MeshConvexIntersection::difference(left,right)}else{MeshConvexIntersection::new(left,right)}.unwrap();
            let result=loop{
                assert!(job.progress().1<25_000);
                let step=job.advance().unwrap_or_else(|error| panic!("{name} (reversed {reversed}, subtract {subtract}): {error:?}"));
                match step {
                    MeshConvexIntersectionStep::Working=>continue,
                    MeshConvexIntersectionStep::Solid(result)=>break result,
                    _=>panic!("opposing longitudinal supports keep closed prism boundaries"),
                }
            };
            let mut uses=std::collections::BTreeMap::new();
            let mut used=std::collections::BTreeSet::new();
            let (mut area,mut volume,mut caps,mut horizontal_caps)=(0.,0.,0,0);
            let grazing_face=lateral["grazingFace"].as_u64().map(|owner|MeshIntersectionFace::Left(owner as u32));
            let mut grazes=0;
            let point=|id:u32|Vec3(result.geometry.positions[id as usize]);
            for (face,ring) in result.geometry.faces.iter().enumerate(){
                for (&p,&q) in ring.iter().zip(ring.iter().cycle().skip(1)){
                    if lateral["generatedContact"] == true {
                        assert_ne!(point(p).0,point(q).0,"{name} has a zero-length boundary edge");
                    }
                    used.insert(p);
                    uses.entry((p.min(q),p.max(q))).or_insert([0;2])[usize::from(p<q)]+=1;
                }
                let first=point(ring[0]);
                let mut normal=Vec3([0.;3]);
                for pair in ring[1..].windows(2){
                    let (p,q)=(point(pair[0]),point(pair[1]));
                    normal=normal.add(p.sub(first).cross(q.sub(first)));
                    volume+=f64::from(first.dot(p.cross(q)))/6.;
                }
                area+=f64::from(normal.dot(normal)).sqrt()/2.;
                if let Some(wall)=walls.iter().find(|wall|ring.iter().all(|&id|{
                    let xyz = point(id).0;
                    let axis = wall["axis"].as_u64().unwrap_or(0) as usize;
                    let coordinate=if let Some(axes)=wall["coefficients"].as_array() {
                        xyz.into_iter().zip(axes).map(|(value,weight)| f64::from(value)*weight.as_f64().unwrap()).sum()
                    } else if diagonal { f64::from(xyz[0]+if axis==0 {-xyz[1]} else {xyz[1]}) }
                    else { f64::from(xyz[axis]) };
                    coordinate==wall["coordinate"].as_f64().unwrap()
                })){
                    if subtract && lateral["maskedStockFace"].as_u64()
                        .is_some_and(|id|result.faces[face] == MeshIntersectionFace::Left(id as u32)) {
                        continue;
                    }
                    if Some(result.faces[face])==grazing_face {
                        grazes+=1;
                        assert!(subtract && normal.0[1]>0.);
                        continue;
                    }
                    caps+=1;
                    let index=wall["face"].as_u64().unwrap() as u32;
                    assert_eq!(result.faces[face],if subtract{MeshIntersectionFace::ReversedRightPlane(index)}
                        else if reversed{MeshIntersectionFace::Left(index)}else{MeshIntersectionFace::RightPlane(index)});
                    let axis = wall["axis"].as_u64().unwrap_or(0) as usize;
                    assert_eq!(normal.0[axis].is_sign_negative(),(wall["normalSign"].as_i64().unwrap()<0)!=subtract);
                }
                if let Some(wall)=lateral["horizontalWalls"].as_array().into_iter().flatten()
                    .find(|wall|ring.iter().all(|&id|f64::from(point(id).0[2])==wall["z"].as_f64().unwrap())){
                    horizontal_caps+=1;
                    let index=wall["face"].as_u64().unwrap() as u32;
                    assert_eq!(result.faces[face],if subtract{MeshIntersectionFace::ReversedRightPlane(index)}
                        else if reversed{MeshIntersectionFace::Left(index)}else{MeshIntersectionFace::RightPlane(index)});
                    assert_eq!(normal.0[2].is_sign_negative(),(index==0)!=subtract);
                }
            }
            let expected=&lateral[if subtract{"difference"}else{"intersection"}];
            for opening in expected["stockCapOpenings"].as_array().into_iter().flatten() {
                let z = opening["z"].as_f64().unwrap() as f32;
                let source = opening["face"].as_u64().unwrap() as u32;
                let stock_planes: Vec<_> = result.geometry.faces.iter().enumerate()
                    .filter(|(_,ring)| ring.iter().all(|&id| point(id).0[2] == z)).map(|(face,_)| result.faces[face]).collect();
                assert_eq!(stock_planes, [MeshIntersectionFace::LeftBoundary(source);2]);
            }
            assert!((volume-expected["volume"].as_f64().unwrap()).abs()<1e-6);
            assert!((area-expected["area"].as_f64().unwrap()).abs()<1e-6);
            assert_eq!(caps,expected["wallSegments"].as_u64().unwrap_or_else(|| walls.iter()
                .map(|wall|wall[if subtract {"differenceSegments"} else {"intersectionSegments"}].as_u64()
                    .or_else(||wall["segments"].as_u64()).unwrap_or(1)).sum::<u64>()));
            for raw in lateral["pinches"].as_array().into_iter().flatten() {
                let [x,y]:[f32;2]=serde_json::from_value(raw.clone()).unwrap();
                let bounds=expected.get("contactHeights").unwrap_or(&law["height"]);
                let heights:Vec<f32>=serde_json::from_value(bounds.clone()).unwrap();
                for (level,z) in heights.into_iter().enumerate() {
                    let touching=used.iter().copied().filter(|&id|point(id).0==[x,y,z]).collect::<Vec<_>>();
                    let copies=&expected["contactCopies"];
                    assert_eq!(touching.len() as u64,copies.as_u64().or_else(||copies[level].as_u64()).unwrap());
                    for id in touching {
                        if lateral["generatedContact"] == true {
                            let recipe = if reversed {
                                &result.right_interpolations.iter().find(|(vertex, _)| *vertex == id).unwrap().1
                            } else {
                                &result.points[id as usize]
                            };
                            let SourceVertex::Weighted { vertices, weights } = recipe else {
                                panic!("interior support crossing needs stock interpolation");
                            };
                            assert!(weights.iter().all(|weight| *weight >= 0.));
                            assert!((weights.iter().sum::<f64>() - 1.).abs() < 1e-6);
                            let stock = prism_from(profile, 0.);
                            for (axis, expected) in [x, y, z].map(f64::from).into_iter().enumerate() {
                                let actual: f64 = vertices.iter().zip(weights)
                                    .map(|(&vertex, weight)| f64::from(stock.positions[vertex as usize][axis]) * weight).sum();
                                assert!((actual - expected).abs() < 1e-6);
                            }
                        }
                        let link:std::collections::BTreeMap<_,_>=result.geometry.faces.iter().filter_map(|ring|{
                            let corner=ring.iter().position(|&vertex|vertex==id)?;
                            Some((ring[(corner+ring.len()-1)%ring.len()],ring[(corner+1)%ring.len()]))
                        }).collect();
                        let first=*link.keys().next().unwrap();
                        let mut current=first;
                        for visited in 0..link.len() {
                            current=link[&current];
                            assert_eq!(current==first,visited+1==link.len());
                        }
                    }
                }
            }
            if let Ok([x,y])=serde_json::from_value::<[f32;2]>(lateral["existingCorner"].clone()) {
                let heights:[f32;2]=serde_json::from_value(law["height"].clone()).unwrap();
                for z in heights {
                    let mut matching=result.geometry.positions.iter().enumerate().filter(|(_,point)|**point==[x,y,z]);
                    let (id,_)=matching.next().unwrap();
                    assert!(matching.next().is_none()&&used.contains(&(id as u32)));
                }
            }
            if let Ok([x,y]) = serde_json::from_value::<[f32;2]>(lateral["grazingCorner"].clone()) {
                let heights:[f32;2]=serde_json::from_value(law["height"].clone()).unwrap();
                for z in heights {
                    let touching=used.iter().filter(|&&id|point(id).0==[x,y,z]).count();
                    assert_eq!(touching,usize::from(subtract));
                }
            }
            assert_eq!(grazes,usize::from(subtract && grazing_face.is_some()));
            assert_eq!(horizontal_caps,lateral["horizontalWalls"].as_array().into_iter().flatten().map(|wall|wall["segments"].as_u64().unwrap_or(1)).sum::<u64>());
            assert!(uses.values().all(|&pair|pair==[1,1]));
            let euler=used.len() as i64+result.geometry.faces.len() as i64-uses.len() as i64;
            assert_eq!(euler,2*(expected["components"].as_i64().unwrap()-expected["genus"].as_i64().unwrap_or(0)), "{name}");
        }
    }
    let mut contact = MeshConvexIntersection::difference(stock(), cutter(-1.0)).unwrap();
    loop {
        match contact.advance() {
            Ok(MeshConvexIntersectionStep::Working) => continue,
            Err(_) => break,
            _ => panic!("the concave tool touching the outer boundary must refuse"),
        }
    }
    let mut cancelled = MeshConvexIntersection::difference(stock(), cutter(0.0)).unwrap();
    for _ in 0..50 { assert!(matches!(cancelled.advance().unwrap(), MeshConvexIntersectionStep::Working)); }
    cancelled.cancel();
    assert!(matches!(cancelled.advance().unwrap(), MeshConvexIntersectionStep::Cancelled));
}

#[test]
fn concave_prisms_certify_axial_and_profile_relations() {
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🛠️modeling/🕳️nested-prismatic-difference.json")).unwrap();
    let profile: Vec<[f32; 2]> = serde_json::from_value(law["footprint"].clone()).unwrap();
    for row in ["axialPairs","nestedConcavePairs","separatedConcavePairs","capCoincidentConcavePairs"].into_iter().flat_map(|key| law[key].as_array().unwrap()) {
        let axis = row["axis"].as_u64().unwrap() as usize;
        let rotation = row["cutterRotation"].as_u64().unwrap() as usize;
        let prism = |name: &str, shift| {
            let levels: [f32; 2] = serde_json::from_value(row[name].clone()).unwrap();
            let (profile_key, faces_key) = if name=="cutter" {("cutterProfile","cutterFaces")} else {("stockProfile","stockFaces")};
            let own_profile = row[profile_key].is_array();
            let authored_profile: Vec<[f32;2]> = if own_profile {
                serde_json::from_value(row[profile_key].clone()).unwrap()
            } else { Vec::new() };
            let profile = if own_profile { &authored_profile } else { &profile };
            let positions = levels.into_iter().flat_map(|z| profile.iter().cycle().skip(shift).take(profile.len())
                .map(move |&[x,y]| [0,1,2].map(|i| [x,y,z][(i+2+3-axis)%3]))).collect();
            let spec = if own_profile { &row[faces_key] } else { &law["faces"] };
            let mut faces: Vec<Vec<u32>> = serde_json::from_value(spec.clone()).unwrap();
            if name=="cutter" && row["reverseCutterCaps"]==true { faces.swap(0,1); }
            Soup { positions, faces, normals: Vec::new() }
        };
        if row["crossingProfile"].is_object() {
            let mut cancellable=MeshConvexIntersection::new(prism("stock",0),prism("cutter",0)).unwrap();
            while cancellable.progress().0!="mesh-prism-profile-relation" {
                assert!(matches!(cancellable.advance().unwrap(),MeshConvexIntersectionStep::Working));
            }
            assert!(matches!(cancellable.advance().unwrap(),MeshConvexIntersectionStep::Working));
            let progress=cancellable.progress();
            cancellable.cancel();
            assert!(matches!(cancellable.advance().unwrap(),MeshConvexIntersectionStep::Cancelled));
            assert_eq!(cancellable.progress(),progress);

            let outline: Vec<[f32;2]>=serde_json::from_value(row["crossingProfile"]["footprint"].clone()).unwrap();
            let mut crossing=prism("cutter",0);
            for (i,position) in crossing.positions.iter_mut().enumerate() {
                let [x,y]=outline[i%outline.len()];
                let z=position[axis];
                *position=[0,1,2].map(|i|[x,y,z][(i+2+3-axis)%3]);
            }
            let mut job=MeshConvexIntersection::new(prism("stock",0),crossing).unwrap();
            loop {
                match job.advance() {
                    Ok(MeshConvexIntersectionStep::Working)=>continue,
                    Err(error)=>{assert!(error.to_string().contains("strict axial containment"));break;}
                    _=>panic!("an edge crossing the outer profile must refuse containment"),
                }
            }
        }
        for (operation,reversed,subtract,union) in [("intersection",false,false,false),
            ("reversedIntersection",true,false,false),("difference",false,true,false),
            ("reversedDifference",true,true,false),
            ("union",false,false,true),("reversedUnion",true,false,true)] {
            if row[operation].is_null() { continue; }
            let first = prism("stock",0);
            let second = prism("cutter",rotation);
            let (left,right) = if reversed { (second,first) } else { (first,second) };
            let (mut intersection,mut union_job) = if union {
                (None,Some(MeshConvexUnion::new(left,right).unwrap()))
            } else {
                let job = if subtract { MeshConvexIntersection::difference(left,right) }
                    else { MeshConvexIntersection::new(left,right) }.unwrap();
                (Some(job),None)
            };
            let expected=&row[operation];
            let result = loop {
                let step = if let Some(job) = &mut union_job { job.advance() }
                    else { intersection.as_mut().unwrap().advance() };
                match step.unwrap_or_else(|e| panic!("{} {operation}: {e:?}",row["name"])) {
                    MeshConvexIntersectionStep::Working => continue,
                    MeshConvexIntersectionStep::Empty => break None,
                    MeshConvexIntersectionStep::Solid(out) => break Some(out),
                    _ => panic!("axial clipping settles"),
                }
            };
            let Some(result) = result else {
                assert_eq!(expected["components"],0);
                continue;
            };
            let mut uses=std::collections::BTreeMap::new();
            let mut vertices=std::collections::BTreeSet::new();
            let mut cap_owners=Vec::new();
            let mut left_owners=Vec::new();
            let mut incidence=std::collections::BTreeMap::new();
            let (mut area,mut volume)=(0.,0.);
            let point=|id:u32|Vec3(result.geometry.positions[id as usize]);
            for (ring,owner) in result.geometry.faces.iter().zip(result.faces) {
                if matches!(owner,MeshIntersectionFace::RightPlane(_) | MeshIntersectionFace::ReversedRightPlane(_)) {
                    assert_eq!(matches!(owner,MeshIntersectionFace::ReversedRightPlane(_)),subtract);
                    cap_owners.push(owner.source_face().1);
                }
                let (side,face) = owner.source_face();
                if side == 0 { left_owners.push(face); }
                for (&a,&b) in ring.iter().zip(ring.iter().cycle().skip(1)) {
                    vertices.insert(a);
                    incidence.entry(a).or_insert([false;2])[side]=true;
                    uses.entry((a.min(b),a.max(b))).or_insert([0;2])[usize::from(a<b)]+=1;
                }
                let anchor=point(ring[0]);
                let mut normal=Vec3([0.;3]);
                for pair in ring[1..].windows(2) {
                    let (a,b)=(point(pair[0]),point(pair[1]));
                    normal=normal.add(a.sub(anchor).cross(b.sub(anchor)));
                    volume+=f64::from(anchor.dot(a.cross(b)))/6.;
                }
                area+=f64::from(normal.length())/2.;
            }
            for face in expected["openStockFaces"].as_array().into_iter().flatten() {
                assert_eq!(left_owners.iter().filter(|&&owner|Some(u64::from(owner))==face.as_u64()).count(),2);
            }
            cap_owners.sort_unstable();
            assert_eq!(serde_json::to_value(cap_owners).unwrap(), expected[if union {"cutterFaces"} else {"cutterCaps"}]);
            if union {
                left_owners.sort_unstable();
                assert_eq!(serde_json::to_value(left_owners).unwrap(),expected["stockFaces"]);
                let shared: Vec<_> = incidence.iter().filter(|(_,owners)| **owners==[true;2]).map(|(&id,_)|id).collect();
                if let Some(seam)=expected["seam"].as_f64() {
                    assert_eq!(shared.len(),profile.len());
                    for id in shared {
                        let position=point(id).0;
                        assert_eq!(f64::from(position[axis]),seam);
                        assert!(result.right_points.contains(&id));
                        let provenance=&result.points[id as usize];
                        if expected["interpolatedSeam"]==true {
                            let SourceVertex::Edge{a,b,t}=*provenance else {panic!("overlap needs a stock-edge sample")};
                            assert!(t>0. && t<1.);
                            for ((before,after),sample) in point(a).0.into_iter().zip(point(b).0).zip(position) {
                                assert!((f64::from(before)+(f64::from(after)-f64::from(before))*t-f64::from(sample)).abs()<1e-6);
                            }
                        } else { assert!(matches!(*provenance,SourceVertex::Original(source) if source==id)); }
                    }
                } else { assert!(shared.is_empty()); }
            }

            assert!(uses.values().all(|count| *count==[1,1]));
            assert_eq!(vertices.len() as i64+result.geometry.faces.len() as i64-uses.len() as i64,2*(expected["components"].as_i64().unwrap()-expected["genus"].as_i64().unwrap_or(0)));
            assert!((area-expected["area"].as_f64().unwrap()).abs()<1e-6);
            assert!((volume-expected["volume"].as_f64().unwrap()).abs()<1e-6);
        }
    }
}
