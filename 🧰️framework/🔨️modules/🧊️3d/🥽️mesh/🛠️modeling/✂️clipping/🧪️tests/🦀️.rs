//! ✂️ Retained clipping geometry, source provenance, and manifold topology laws.
use super::*;
#[test]
fn edge_classification_matches_shared_vectors(){
 let cases:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in cases.as_array().unwrap(){
  let vector=|name:&str|{let a=row[name].as_array().unwrap();Vec3(std::array::from_fn(|i|a[i].as_f64().unwrap() as f32))};
  let epsilon=row["epsilon"].as_f64().unwrap() as f32;
  let(da,_,intersection)=mesh_clip_edge(vector("a"),vector("b"),vector("normal"),row["distance"].as_f64().unwrap() as f32,epsilon);
  assert_eq!(da<=epsilon,row["expected"]["retained"].as_bool().unwrap());
  match intersection{Some((point,t))=>{assert_eq!(t as f64,row["expected"]["parameter"].as_f64().unwrap());let position=row["expected"]["position"].as_array().unwrap();for i in 0..3{assert_eq!(point.0[i] as f64,position[i].as_f64().unwrap());}},None=>assert!(row["expected"]["position"].is_null())}
 }
}

#[test]
fn clipping_provenance_is_complete_and_transferred_once(){
 let mut job=MeshClipJob::new(Vec3([1.0,0.0,0.0]),0.0,0.0001);
 assert!(job.take_source_faces().is_err());assert!(job.vertex_source(0).is_err());
 let mut soup=Soup {positions:vec![[-1.0,-1.0,-1.0],[1.0,-1.0,-1.0],[1.0,1.0,-1.0],[-1.0,1.0,-1.0],[-1.0,-1.0,1.0],[1.0,-1.0,1.0],[1.0,1.0,1.0],[-1.0,1.0,1.0]],faces:vec![vec![0,3,2,1],vec![4,5,6,7],vec![0,1,5,4],vec![1,2,6,5],vec![2,3,7,6],vec![3,0,4,7]],normals:Vec::new()};
 let mut done=false;for _ in 0..4096{if job.advance(&mut soup).unwrap(){done=true;break;}}
 assert!(done);assert!(soup.positions.iter().all(|point|point.iter().all(|value|value.is_finite())));
 assert!(!job.take_source_faces().unwrap().is_empty());assert!(job.take_source_faces().is_err());
 assert!(!job.take_corner_faces().unwrap().is_empty());assert!(job.take_corner_faces().is_err());
 assert!(!job.take_face_weights().unwrap().is_empty());assert!(job.take_face_weights().is_err());
 assert!(job.vertex_source(0).is_ok());assert!(job.advance(&mut soup).is_err());
}

#[test]
fn convex_intersection_keeps_halfspaces_and_two_source_provenance() {
    use std::collections::{BTreeMap,BTreeSet};
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🛠️modeling/⚗️convex-intersection.json")).unwrap();
    let faces:Vec<Vec<u32>>=serde_json::from_value(law["faces"].clone()).unwrap();
    let cube=|bounds:&serde_json::Value,angle:f64|{
        let [a,b]:[[f32;3];2]=serde_json::from_value(bounds.clone()).unwrap();
        let center=[f64::from(a[0]+b[0])/2.0,f64::from(a[1]+b[1])/2.0];
        let (sin,cos)=angle.to_radians().sin_cos();
        let positions=[[a[0],a[1],a[2]],[b[0],a[1],a[2]],[b[0],b[1],a[2]],[a[0],b[1],a[2]],
            [a[0],a[1],b[2]],[b[0],a[1],b[2]],[b[0],b[1],b[2]],[a[0],b[1],b[2]]].map(|[x,y,z]|{
                let (x,y)=(f64::from(x)-center[0],f64::from(y)-center[1]);
                [(center[0]+x*cos-y*sin) as f32,(center[1]+x*sin+y*cos) as f32,z]
            }).to_vec();
        Soup{positions,faces:faces.clone(),normals:Vec::new()}
    };
    let dot=|a:[f64;3],b:[f64;3]|(0..3).map(|k|a[k]*b[k]).sum::<f64>();
    let cross=|[a,b,c]:[f64;3],[d,e,f]:[f64;3]|[b*f-c*e,c*d-a*f,a*e-b*d];
    let minus=|a:[f64;3],b:[f64;3]|std::array::from_fn(|k|a[k]-b[k]);
    for row in law["cases"].as_array().unwrap(){
        let left_bounds=row.get("left").unwrap_or(&law["left"]);
        let left=cube(left_bounds,0.0);
        let reference=left.positions[0].map(f64::from);
        let right=cube(&row["right"],row["rotateZ"].as_f64().unwrap_or(0.0));
        let mut job=MeshConvexIntersection::new(cube(left_bounds,0.0),cube(&row["right"],row["rotateZ"].as_f64().unwrap_or(0.0))).unwrap();
        let result=loop {
            let phase=job.progress();
            assert!(phase.1<20_000);
            match job.advance().unwrap_or_else(|err|panic!("{}: {err:?}",row["name"])) {
                MeshConvexIntersectionStep::Working=>assert_eq!(job.progress().1,phase.1+1),
                output=>break output,
            }
        };
        if row["empty"]==true {
            assert!(matches!(result,MeshConvexIntersectionStep::Empty),"{}",row["name"]);
            continue;
        }
        let MeshConvexIntersectionStep::Solid(out)=result else{panic!("{} should have volume",row["name"]);};
        let geom=&out.geometry;
        assert_eq!(out.faces.len(),geom.faces.len());
        assert_eq!(out.points.len(),geom.positions.len());
        let mut found_left=Vec::new();
        let mut found_right=Vec::new();
        let mut uses=BTreeMap::new();
        let mut live=BTreeSet::new();
        let (mut area,mut volume)=(0.0,0.0);
        let point=|id:u32|geom.positions[id as usize].map(f64::from);
        for (ring,origin) in geom.faces.iter().zip(&out.faces) {
            let (source,face)=match *origin {
                MeshIntersectionFace::Left(i)=>{found_left.push(i);(&left,i)},
                MeshIntersectionFace::RightPlane(i)=>{found_right.push(i);(&right,i)},
                _=>panic!("convex intersection emitted an unexpected supporting face"),
            };
            let basis=&source.faces[face as usize];
            let anchor=source.positions[basis[0] as usize].map(f64::from);
            let normal=cross(minus(source.positions[basis[1] as usize].map(f64::from),anchor),
                minus(source.positions[basis[2] as usize].map(f64::from),anchor));
            for (i,&a) in ring.iter().enumerate() {
                assert!(dot(minus(point(a),anchor),normal).abs()<1e-5);
                live.insert(a);
                let b=ring[(i+1)%ring.len()];
                let seen=uses.entry((a.min(b),a.max(b))).or_insert([0;2]);
                seen[usize::from(a<b)]+=1;
            }
            for i in 1..ring.len()-1{
                let [a,b,c]=[ring[0],ring[i],ring[i+1]].map(point);
                let n=cross(minus(b,a),minus(c,a));
                area+=dot(n,n).sqrt()/2.0;
                volume+=dot(minus(a,reference),cross(minus(b,reference),minus(c,reference)))/6.0;
            }
        }
        let [low,high]:[[f32;3];2]=serde_json::from_value(row["bounds"].clone()).unwrap();
        for axis in 0..3{
            let min=live.iter().map(|&id|geom.positions[id as usize][axis]).fold(f32::INFINITY,f32::min);
            let max=live.iter().map(|&id|geom.positions[id as usize][axis]).fold(f32::NEG_INFINITY,f32::max);
            assert!((min-low[axis]).abs()<1e-6 && (max-high[axis]).abs()<1e-6);
        }
        assert!(uses.values().all(|uses|*uses==[1,1]),"{}",row["name"]);
        assert_eq!(live.len() as isize-uses.len() as isize+geom.faces.len() as isize,2);
        found_left.sort_unstable();found_right.sort_unstable();
        assert_eq!(serde_json::to_value(found_left).unwrap(),row["leftFaces"]);
        assert_eq!(serde_json::to_value(found_right).unwrap(),row["rightPlanes"]);
        assert!((area-row["area"].as_f64().unwrap()).abs()<1e-5,"{}: {area}",row["name"]);
        assert!((volume-row["volume"].as_f64().unwrap()).abs()<1e-5,"{}: {volume}",row["name"]);
        for (i,source) in out.points.into_iter().enumerate(){
            let actual=point(i as u32);
            let expected=match source {
                SourceVertex::Original(id)=>left.positions[id as usize].map(f64::from),
                SourceVertex::Edge{a,b,t}=>{
                    assert!((a as usize)<i && (b as usize)<i && (0.0..1.0).contains(&t));
                    let (a,b)=(point(a),point(b));
                    std::array::from_fn(|axis|a[axis]+(b[axis]-a[axis])*t)
                },
                _=>panic!("unexpected intersection vertex lineage"),
            };
            assert!(expected.into_iter().zip(actual).all(|(a,b)|(a-b).abs()<1e-6));
        }
    }

    let run=|mut job:MeshConvexIntersection|{
        loop{match job.advance(){
            Ok(MeshConvexIntersectionStep::Working)=>continue,
            out=>{
                assert!(job.advance().is_err());
                assert_eq!(job.progress().0,"intersection-refused");
                break out;
            },
        }}
    };
    let mut open=cube(&law["left"],0.0);open.faces.pop();
    assert!(run(MeshConvexIntersection::new(open,cube(&law["left"],0.0)).unwrap()).is_err());
    let mut inverted=cube(&law["left"],0.0);
    for ring in &mut inverted.faces{ring.reverse();}
    assert!(run(MeshConvexIntersection::new(cube(&law["left"],0.0),inverted).unwrap()).is_err());
    let bevel:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🛠️modeling/🪚️bevel.json")).unwrap();
    let concave=Soup{positions:serde_json::from_value(bevel["concave"]["positions"].clone()).unwrap(),
        faces:serde_json::from_value(bevel["concave"]["faces"].clone()).unwrap(),normals:Vec::new()};
    let cutter = cube(&law["overlappingWallCutter"], 0.0);
    assert!(run(MeshConvexIntersection::new(concave, cutter).unwrap()).is_err());

    let mut cancelled=MeshConvexIntersection::new(cube(&law["left"],0.0),cube(&law["cases"][0]["right"],0.0)).unwrap();
    while cancelled.progress().0!="intersection-clip"{assert!(matches!(cancelled.advance().unwrap(),MeshConvexIntersectionStep::Working));}
    cancelled.cancel();
    let before=cancelled.progress();
    assert!(matches!(cancelled.advance().unwrap(),MeshConvexIntersectionStep::Cancelled));
    assert_eq!(cancelled.progress(),before);
}

#[test]
fn convex_difference_keeps_reversed_cut_provenance() {
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🛠️modeling/➖️convex-difference.json")).unwrap();
    let faces:Vec<Vec<u32>>=serde_json::from_value(law["faces"].clone()).unwrap();
    let cube=|bounds:&serde_json::Value|{
        let [[x,y,z],[a,b,c]]:[[f32;3];2]=serde_json::from_value(bounds.clone()).unwrap();
        Soup{positions:vec![[x,y,z],[a,y,z],[a,b,z],[x,b,z],[x,y,c],[a,y,c],[a,b,c],[x,b,c]],faces:faces.clone(),normals:Vec::new()}
    };
    for row in law["cases"].as_array().unwrap(){
        let left_source=||{
            let mut left=cube(&law["left"]);
            if let Some(index)=row["splitStockOpening"].as_u64(){
                let index=index as usize;
                let ring=left.faces[index].clone();
                left.faces[index]=vec![ring[0],ring[1],ring[2]];
                left.faces.insert(index+1,vec![ring[0],ring[2],ring[3]]);
            }
            left
        };
        let left=left_source();
        let right=cube(&row["right"]);
        let mut job=MeshConvexIntersection::difference(left_source(),cube(&row["right"])).unwrap();
        let result=loop{
            let previous=job.progress().1;
            match job.advance(){
                Ok(MeshConvexIntersectionStep::Working)=>assert_eq!(job.progress().1,previous+1),
                out=>break out,
            }
        };
        if row["refused"]==true{
            assert!(result.is_err());
            continue;
        }
        if row["volume"]==0{assert!(matches!(result.unwrap(),MeshConvexIntersectionStep::Empty));continue;}
        let name = row["name"].as_str().unwrap();
        let MeshConvexIntersectionStep::Solid(out)=result.unwrap_or_else(|err|panic!("{name}: {err:?}"))else{panic!("{name} must retain positive volume");};
        if row["unchanged"]==true{
            assert_eq!(out.geometry.positions,left.positions);
            assert_eq!(out.geometry.faces,left.faces);
        }
        let mut cuts=Vec::new();
        let junction:Option<[f32;3]>=serde_json::from_value(row["junction"].clone()).unwrap();
        let mut shell_volumes=[0.0;2];
        let cavity=row["cavity"].is_object();
        if cavity{
            assert_eq!(&out.geometry.positions[..left.positions.len()],left.positions.as_slice());
            assert_eq!(&out.geometry.faces[..left.faces.len()],left.faces.as_slice());
            assert_eq!(out.right_points.len(),right.positions.len());
            for (id,point) in out.right_points.iter().zip(&right.positions){assert_eq!(out.geometry.positions[*id as usize],*point);}
        }
        let tunnel=row["tunnel"].as_object().map(|_|{
            let axis=row["tunnel"]["axis"].as_u64().unwrap() as usize;
            let opening:[f32;2]=serde_json::from_value(row["tunnel"]["opening"].clone()).unwrap();
            (axis,opening)
        });
        let pocket=row["pocket"].as_object().map(|_|{
            let axis=row["pocket"]["axis"].as_u64().unwrap() as usize;
            let bottom=row["pocket"]["bottomPlane"].as_u64().unwrap() as u32;
            let depths=[row["pocket"]["opening"].as_f64().unwrap() as f32,row["pocket"]["bottom"].as_f64().unwrap() as f32];
            (axis,bottom,depths)
        });
        let slab=row["slab"].as_object().map(|_|{
            let axis=row["slab"]["axis"].as_u64().unwrap() as usize;
            let ranges:[[f32;2];2]=serde_json::from_value(row["slab"]["intervals"].clone()).unwrap();
            (axis,ranges)
        });
        let mut edges=std::collections::BTreeMap::new();
        let (mut area,mut volume)=(0.0,0.0);
        let split=row["splitStockOpening"].as_u64().map(|i|i as u32);
        let mut split_seen=[false;2];
        let mut rim_sources=std::collections::BTreeMap::new();
        let point=|i:u32|Vec3(out.geometry.positions[i as usize]);
        for (ring,source) in out.geometry.faces.iter().zip(out.faces){
            let mut shell=usize::from(cavity&&matches!(source,MeshIntersectionFace::ReversedRightPlane(_)));
            if let Some((axis,ranges))=slab{
                shell=usize::from(point(ring[0]).0[axis]>=ranges[1][0]);
                assert!(ring.iter().all(|&id|(ranges[shell][0]..=ranges[shell][1]).contains(&point(id).0[axis])));
            }
            for (i,&a) in ring.iter().enumerate(){
                let b=ring[(i+1)%ring.len()];
                let key=(a.min(b),a.max(b));
                edges.entry(key).or_insert([0;2])[usize::from(a<b)]+=1;
                if row["openStockFaces"].is_array(){
                    let (stock,cutter)=rim_sources.entry(key).or_insert((None,false));
                    match source{
                        MeshIntersectionFace::Left(id)=>*stock=Some(id),
                        MeshIntersectionFace::ReversedRightPlane(_)=>*cutter=true,
                        _=>panic!("unowned recess surface"),
                    }
                }
            }
            for i in 1..ring.len()-1{
                let [a,b,c]=[ring[0],ring[i],ring[i+1]].map(point);
                let normal=b.sub(a).cross(c.sub(a));
                area+=f64::from(normal.dot(normal)).sqrt()/2.0;
                let measure=f64::from(a.dot(b.cross(c)))/6.0;
                volume+=measure;shell_volumes[shell]+=measure;
                if let MeshIntersectionFace::ReversedRightPlane(id)=source {
                    let [p,q,r]=[0,1,2].map(|j|Vec3(right.positions[right.faces[id as usize][j] as usize]));
                    assert!(normal.dot(q.sub(p).cross(r.sub(p)))<0.0);
                }
            }
            if let MeshIntersectionFace::ReversedRightPlane(id)=source{
                if cavity{
                    assert!(ring.iter().copied().eq(right.faces[id as usize].iter().rev().map(|&id|out.right_points[id as usize])));
                }
                if let Some((axis,opening))=tunnel{
                    assert!(opening.into_iter().all(|height|ring.iter().any(|&id|point(id).0[axis]==height)));
                }
                if let Some((axis,bottom,depths))=pocket{
                    if id==bottom{assert!(ring.iter().all(|&id|point(id).0[axis]==depths[1]));}
                    else{assert!(depths.into_iter().all(|depth|ring.iter().any(|&id|point(id).0[axis]==depth)));}
                }
                if let Some(point_at_corner)=junction{assert!(ring.iter().any(|&vertex|point(vertex).0==point_at_corner));}
                cuts.push(id);
            }
            else if let MeshIntersectionFace::Left(id)=source {
                if let Some(start)=split.filter(|&start|(start..=start+1).contains(&id)){
                    split_seen[(id-start) as usize]=true;
                    let original=&left.faces[id as usize];
                    let [a,b,c]=[0,1,2].map(|i|Vec3(left.positions[original[i] as usize]));
                    let normal=b.sub(a).cross(c.sub(a));
                    for &vertex in ring {
                        let p=point(vertex);
                        assert!(p.sub(a).dot(normal).abs()<1e-6);
                        assert!([(a,b),(b,c),(c,a)].into_iter().all(|(from,to)|
                            to.sub(from).cross(p.sub(from)).dot(normal)>=-1e-6));
                    }
                }
            }else{panic!("{name} must retain authored surface ownership");}
        }
        if split.is_some(){assert_eq!(split_seen,[true;2],"{name}");}
        if row["openStockFaces"].is_array(){
            let opening=rim_sources.values().filter_map(|&(stock,cutter)|stock.filter(|_|cutter))
                .collect::<std::collections::BTreeSet<_>>();
            assert_eq!(serde_json::to_value(opening).unwrap(),row["openStockFaces"],"{name}");
        }
        let mut expected=row["cutPlane"].as_u64().map(|p|p as u32).into_iter().collect::<Vec<_>>();
        if let Some(planes)=row["cutPlanes"].as_array(){
            expected.extend(planes.iter().map(|plane|plane.as_u64().unwrap() as u32));
            let live=edges.keys().flat_map(|&(a,b)|[a,b]).collect::<std::collections::BTreeSet<_>>();
            let counts = [live.len(), edges.len(), out.geometry.faces.len()].map(|n| n as u64);
            assert_eq!(counts, ["vertices", "edges", "faces"].map(|field| row[field].as_u64().unwrap()), "{name}");
            if tunnel.is_some(){assert_eq!(live.len()+out.geometry.faces.len(),edges.len());}
            if pocket.is_some()||row["recess"]==true{assert_eq!(live.len()+out.geometry.faces.len(),edges.len()+2);}
            if cavity{assert_eq!(live.len()+out.geometry.faces.len(),edges.len()+4);}
        }
        cuts.sort_unstable();assert_eq!(cuts,expected);
        assert!(edges.values().all(|c|*c==[1,1]));
        assert!((area-row["area"].as_f64().unwrap()).abs()<1e-6);
        assert!((volume-row["volume"].as_f64().unwrap()).abs()<1e-6);
        if slab.is_some()||cavity{
            let volumes:[f64;2]=serde_json::from_value(row[if cavity{"cavity"}else{"slab"}]["volumes"].clone()).unwrap();
            for (actual,expected) in shell_volumes.into_iter().zip(volumes){assert!((actual-expected).abs()<1e-6);}
        }
    }
    let mut cancelled=MeshConvexIntersection::difference(cube(&law["left"]),cube(&law["cases"][0]["right"])).unwrap();
    while cancelled.progress().0!="intersection-clip"{assert!(matches!(cancelled.advance().unwrap(),MeshConvexIntersectionStep::Working));}
    cancelled.cancel();
    assert!(matches!(cancelled.advance().unwrap(),MeshConvexIntersectionStep::Cancelled));
    for &index in &[6,7,12,15,16,19,20]{
        let mut cancelled=MeshConvexIntersection::difference(cube(&law["left"]),cube(&law["cases"][index]["right"])).unwrap();
        while cancelled.progress().0!="difference-partition"{assert!(matches!(cancelled.advance().unwrap(),MeshConvexIntersectionStep::Working));}
        for _ in 0..100{assert!(matches!(cancelled.advance().unwrap(),MeshConvexIntersectionStep::Working));}
        cancelled.cancel();
        assert!(matches!(cancelled.advance().unwrap(),MeshConvexIntersectionStep::Cancelled));
    }
    let mut contained=MeshConvexIntersection::difference(cube(&law["left"]),cube(&law["cases"][23]["right"])).unwrap();
    while contained.progress().0!="difference-contained-shells"{assert!(matches!(contained.advance().unwrap(),MeshConvexIntersectionStep::Working));}
    for _ in 0..15{assert!(matches!(contained.advance().unwrap(),MeshConvexIntersectionStep::Working));}
    contained.cancel();
    assert!(matches!(contained.advance().unwrap(),MeshConvexIntersectionStep::Cancelled));

}

#[test]
fn convex_union_welds_matching_faces_and_preserves_source_ownership() {
    let law:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🛠️modeling/➕️convex-union.json")).unwrap();
    let faces:Vec<Vec<u32>>=serde_json::from_value(law["faces"].clone()).unwrap();
    let cube=|bounds:&serde_json::Value|{
        let [[x,y,z],[a,b,c]]:[[f32;3];2]=serde_json::from_value(bounds.clone()).unwrap();
        Soup{positions:vec![[x,y,z],[a,y,z],[a,b,z],[x,b,z],[x,y,c],[a,y,c],[a,b,c],[x,b,c]],faces:faces.clone(),normals:Vec::new()}
    };
    for row in law["cases"].as_array().unwrap(){
        let left_bounds=row.get("left").unwrap_or(&law["left"]);
        let left=cube(left_bounds);
        let right=cube(&row["right"]);
        let mut job=MeshConvexUnion::new(cube(left_bounds),cube(&row["right"])).unwrap();
        let result=loop{
            let previous=job.progress().0;
            match job.advance(){
                Ok(MeshConvexIntersectionStep::Working)=>assert_eq!(job.progress().0,previous+1),
                out=>break out,
            }
        };
        if row["refused"]==true{assert!(result.is_err(),"{}",row["name"]);assert!(job.advance().is_err());continue;}
        let MeshConvexIntersectionStep::Solid(out)=result.unwrap_or_else(|err|panic!("{}: {err:?}",row["name"]))else{panic!("expected union boundary");};
        assert_eq!(out.retained_operand,row["retained"].as_u64().map(|i|i as u8));
        if let Some(faces)=row["faces"].as_u64(){assert_eq!(out.faces.len() as u64,faces);}
        for (&id,&point) in out.right_points.iter().zip(&right.positions){assert_eq!(out.geometry.positions[id as usize],point);}
        let mut edges=std::collections::BTreeMap::new();
        let mut volume=0.0;
        let mut area=0.0;
        let point=|id:u32|Vec3(out.geometry.positions[id as usize]);
        for (ring,origin) in out.geometry.faces.iter().zip(&out.faces){
            let (source,id)=match *origin{
                MeshIntersectionFace::Left(id)=>(&left,id),
                MeshIntersectionFace::RightPlane(id)=>(&right,id),
                _=>panic!("interior supporting face leaked into union"),
            };
            let basis=&source.faces[id as usize];
            let [a,b,c]=[0,1,2].map(|i|Vec3(source.positions[basis[i] as usize]));
            let normal=b.sub(a).cross(c.sub(a));
            for (i,&p) in ring.iter().enumerate(){
                assert!(normal.dot(point(p).sub(a)).abs()<1e-6);
                let q=ring[(i+1)%ring.len()];
                edges.entry((p.min(q),p.max(q))).or_insert([0;2])[usize::from(p<q)]+=1;
            }
            for i in 1..ring.len()-1{
                let [a,b,c]=[ring[0],ring[i],ring[i+1]].map(point);
                let cross=b.sub(a).cross(c.sub(a));
                area+=f64::from(cross.dot(cross)).sqrt()/2.0;
                volume+=f64::from(a.dot(b.cross(c)))/6.0;
            }
        }
        assert!(edges.values().all(|uses|*uses==[1,1]));
        if row["partial"]==true{
            let live=edges.keys().flat_map(|&(a,b)|[a,b]).collect::<std::collections::BTreeSet<_>>();
            assert_eq!(live.len()+out.geometry.faces.len(),edges.len()+2);
            if let Some(vertices)=row["vertices"].as_u64(){assert_eq!(live.len() as u64,vertices);}
            if let Some(count)=row["edges"].as_u64(){assert_eq!(edges.len() as u64,count);}
            assert!(!out.right_interpolations.is_empty());
            assert!(out.right_interpolations.iter().any(|&(id,_)|(id as usize)<out.points.len()));
            for &(id,recipe) in &out.right_interpolations{
                let SourceVertex::Edge{a,b,t}=recipe else{panic!("partial union generated point lacks edge provenance");};
                let expected=point(a).lerp(point(b),t as f32);
                assert!(expected.0.into_iter().zip(point(id).0).all(|(a,b)|(a-b).abs()<1e-6));
            }
        }
        assert!((area-row["area"].as_f64().unwrap()).abs()<1e-6);
        assert!((volume-row["volume"].as_f64().unwrap()).abs()<1e-6);
    }
    let mut job=MeshConvexUnion::new(cube(&law["left"]),cube(&law["cases"][0]["right"])).unwrap();
    for _ in 0..500 {assert!(matches!(job.advance().unwrap(),MeshConvexIntersectionStep::Working));}
    job.cancel();
    let before=job.progress();
    assert!(matches!(job.advance().unwrap(),MeshConvexIntersectionStep::Cancelled));
    assert_eq!(job.progress(),before);
    let partial=||MeshConvexUnion::new(cube(&law["left"]),cube(&law["cases"][7]["right"])).unwrap();
    let mut measured=partial();
    while matches!(measured.advance().unwrap(),MeshConvexIntersectionStep::Working){}
    let mut cancelled=partial();
    for _ in 0..measured.progress().0-16{assert!(matches!(cancelled.advance().unwrap(),MeshConvexIntersectionStep::Working));}
    cancelled.cancel();
    let before=cancelled.progress();
    assert!(matches!(cancelled.advance().unwrap(),MeshConvexIntersectionStep::Cancelled));
    assert_eq!(cancelled.progress(),before);

}

#[test]
fn convex_difference_retains_prismatic_openings() {
    use std::collections::{BTreeMap, BTreeSet};
    let law: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🛠️modeling/🔺️prismatic-difference.json")).unwrap();
    let stock = || Soup {
        positions: serde_json::from_value(law["stock"]["positions"].clone()).unwrap(),
        faces: serde_json::from_value(law["stock"]["faces"].clone()).unwrap(),
        normals: Vec::new(),
    };
    for row in law["cases"].as_array().unwrap() {
        let spec = &law["profiles"][row["profile"].as_str().unwrap()];
        let profile: Vec<[f32; 2]> = serde_json::from_value(spec["footprint"].clone()).unwrap();
        let heights: [f32; 2] = serde_json::from_value(row["height"].clone()).unwrap();
        let cutter: Vec<[f32; 3]> = heights.into_iter().flat_map(|z| profile.iter().map(move |&[x, y]| [x, y, z])).collect();
        let faces: Vec<Vec<u32>> = serde_json::from_value(spec["cutterFaces"].clone()).unwrap();
        let mut job = MeshConvexIntersection::difference(
            stock(), Soup { positions: cutter.clone(), faces: faces.clone(), normals: Vec::new() },
        ).unwrap();
        let name = row["name"].as_str().unwrap();
        let result = loop {
            assert!(job.progress().1 < 25_000, "{name}");
            match job.advance().unwrap_or_else(|error| panic!("{name}: {error:?}")) {
                MeshConvexIntersectionStep::Working => continue,
                MeshConvexIntersectionStep::Solid(result) => break result,
                _ => panic!("{name} must publish one closed solid"),
            }
        };
        let mut cap_ids = Vec::new();
        let mut vertices = BTreeSet::new();
        let mut edges = BTreeMap::new();
        let (mut area, mut volume) = (0.0, 0.0);
        let point = |i: u32| Vec3(result.geometry.positions[i as usize]);
        for (ring, owner) in result.geometry.faces.iter().zip(result.faces) {
            let cap = if let MeshIntersectionFace::ReversedRightPlane(id) = owner {
                cap_ids.push(id);
                let [a, b, c] = [0, 1, 2].map(|i| Vec3(cutter[faces[id as usize][i] as usize]));
                let normal = b.sub(a).cross(c.sub(a));
                assert!(ring.iter().all(|&id| point(id).sub(a).dot(normal).abs() < 1e-6));
                Some(normal)
            } else {
                assert!(matches!(owner, MeshIntersectionFace::Left(_)));
                None
            };
            for (i, &a) in ring.iter().enumerate() {
                vertices.insert(a);
                let b = ring[(i + 1) % ring.len()];
                edges.entry((a.min(b), a.max(b))).or_insert([0; 2])[usize::from(a < b)] += 1;
            }
            for i in 1..ring.len() - 1 {
                let [a, b, c] = [ring[0], ring[i], ring[i + 1]].map(point);
                let normal = b.sub(a).cross(c.sub(a));
                if let Some(cap) = cap { assert!(normal.dot(cap) <= 0.0); }
                area += f64::from(normal.dot(normal)).sqrt() / 2.0;
                volume += f64::from(a.dot(b.cross(c))) / 6.0;
            }
        }
        cap_ids.sort_unstable();
        assert_eq!(serde_json::to_value(cap_ids).unwrap(), row["capFaces"]);
        assert!(edges.values().all(|uses| *uses == [1, 1]), "{name}");
        let counts = [vertices.len(), edges.len(), result.geometry.faces.len()].map(|n| n as u64);
        assert_eq!(counts, ["vertices", "edges", "faces"].map(|field| row[field].as_u64().unwrap()), "{name}");
        let euler = vertices.len() as i64 + result.geometry.faces.len() as i64 - edges.len() as i64;
        assert_eq!(euler, row["euler"].as_i64().unwrap(), "{name}");
        assert!((volume - row["volume"].as_f64().unwrap()).abs() < 1e-6, "{name}: {volume}");
        assert!((area - row["area"].as_f64().unwrap()).abs() < 1e-6, "{name}: {area}");
    }
}


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
    for name in ["lateral","longitudinalSlab","mixedSlab","mixedOpenSlab","independentWalls","fourArmWalls","cornerSlab"]{
        let lateral=&law[name];
        let walls=lateral["walls"].as_array().unwrap();
        let diagonal=lateral["frame"]=="diagonal";
        let [[x,y,z],[a,b,c]]:[[f32;3];2]=serde_json::from_value(lateral["bounds"].clone()).unwrap();
        let lateral_tool=||Soup{
            positions:[[x,y,z],[a,y,z],[a,b,z],[x,b,z],[x,y,c],[a,y,c],[a,b,c],[x,b,c]].into_iter()
                .map(|[u,v,w]|if diagonal{[(u+v)/2.,(v-u)/2.,w]}else{[u,v,w]}).collect(),
            faces:stock().faces,normals:Vec::new(),
        };
        for (reversed,subtract) in [(false,false),(true,false),(false,true)]{
            if subtract && lateral["difference"].is_null(){continue;}
            let profile = if lateral["footprint"].is_array() { lateral } else { &law };
            let prism = prism_from(profile, 0.);
            let (left,right)=if reversed{(lateral_tool(),prism)}else{(prism,lateral_tool())};
            let mut job=if subtract{MeshConvexIntersection::difference(left,right)}else{MeshConvexIntersection::new(left,right)}.unwrap();
            let result=loop{
                assert!(job.progress().1<25_000);
                match job.advance().unwrap(){
                    MeshConvexIntersectionStep::Working=>continue,
                    MeshConvexIntersectionStep::Solid(result)=>break result,
                    _=>panic!("opposing longitudinal supports keep closed prism boundaries"),
                }
            };
            let mut uses=std::collections::BTreeMap::new();
            let mut used=std::collections::BTreeSet::new();
            let (mut area,mut volume,mut caps,mut horizontal_caps)=(0.,0.,0,0);
            let point=|id:u32|Vec3(result.geometry.positions[id as usize]);
            for (face,ring) in result.geometry.faces.iter().enumerate(){
                for (&p,&q) in ring.iter().zip(ring.iter().cycle().skip(1)){
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
                    f64::from(if diagonal { xyz[0] - xyz[1] } else { xyz[axis] }) == wall["coordinate"].as_f64().unwrap()
                })){
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
            assert!((volume-expected["volume"].as_f64().unwrap()).abs()<1e-6);
            assert!((area-expected["area"].as_f64().unwrap()).abs()<1e-6);
            assert_eq!(caps,walls.len());
            if diagonal {
                let [x,y]:[f32;2]=serde_json::from_value(lateral["existingCorner"].clone()).unwrap();
                let heights:[f32;2]=serde_json::from_value(law["height"].clone()).unwrap();
                for z in heights {
                    let mut matching=result.geometry.positions.iter().enumerate().filter(|(_,point)|**point==[x,y,z]);
                    let (id,_)=matching.next().unwrap();
                    assert!(matching.next().is_none()&&used.contains(&(id as u32)));
                }
            }
            assert_eq!(horizontal_caps,lateral["horizontalWalls"].as_array().map_or(0,Vec::len));
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
fn convex_difference_keeps_disconnected_manifold_components() {
    use std::collections::{BTreeMap, BTreeSet};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🛠️modeling/🧩️disconnected-difference.json")).unwrap();
    let faces:Vec<Vec<u32>>=serde_json::from_value(fixture["faces"].clone()).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let soup=|side:&str|{
            let mut faces=faces.clone();
            if side=="left"&&row["leftReflected"]==true{for face in &mut faces{face.reverse();}}
            Soup{positions:serde_json::from_value(row[side].clone()).unwrap(),faces,normals:Vec::new()}
        };
        let mut job=MeshConvexIntersection::difference(soup("left"),soup("right")).unwrap();
        let output=loop{
            match job.advance().unwrap_or_else(|err|panic!("{}: {err:?}",row["name"])){
                MeshConvexIntersectionStep::Working=>{},
                MeshConvexIntersectionStep::Solid(output)=>break output,
                _=>panic!("{} lost material",row["name"]),
            }
        };
        let mut edges=BTreeMap::<_,Vec<_>>::new();
        for (face,ring) in output.geometry.faces.iter().enumerate(){
            for (a,b) in ring.iter().copied().zip(ring.iter().copied().cycle().skip(1)){
                edges.entry((a.min(b),a.max(b))).or_default().push((face,a<b));
            }
        }
        assert!(edges.values().all(|uses|uses.len()==2&&uses[0].1!=uses[1].1));
        let mut neighbors=vec![Vec::new();output.geometry.faces.len()];
        for used in edges.values(){
            let (a,b)=(used[0].0,used[1].0);
            neighbors[a].push(b);
            neighbors[b].push(a);
        }
        let mut remaining=(0..neighbors.len()).collect::<BTreeSet<_>>();
        let mut metrics=Vec::new();
        let point=|index:u32|Vec3(output.geometry.positions[index as usize]);
        while let Some(start)=remaining.pop_first(){
            let mut stack=vec![start];
            let anchor=point(output.geometry.faces[start][0]);
            let mut vertices=BTreeSet::new();
            let mut incidence=0;
            let mut faces=0;
            let (mut area,mut volume)=(0.,0.);
            let mut sides=[false;2];
            while let Some(face)=stack.pop(){
                sides[usize::from(matches!(output.faces[face],MeshIntersectionFace::ReversedRightPlane(_)))]=true;
                let ring=&output.geometry.faces[face];
                let first=point(ring[0]);
                for pair in ring[1..].windows(2){
                    let [a,b]=[point(pair[0]),point(pair[1])];
                    let normal=a.sub(first).cross(b.sub(first));
                    area+=f64::from(normal.dot(normal)).sqrt()/2.;
                    volume+=f64::from(first.sub(anchor).dot(a.sub(anchor).cross(b.sub(anchor))))/6.;
                }
                vertices.extend(ring.iter().copied());
                incidence+=ring.len();
                faces+=1;
                stack.extend(neighbors[face].iter().copied().filter(|near|remaining.remove(near)));
            }
            assert_eq!(sides,[true,true]);
            metrics.push((volume,area,vertices.len() as i64-(incidence/2) as i64+faces));
        }
        metrics.sort_unstable_by(|a,b|a.0.total_cmp(&b.0));
        let expected=row["components"].as_array().unwrap();
        assert_eq!(metrics.len(),expected.len());
        for ((volume,area,euler),want) in metrics.into_iter().zip(expected){
            assert!((volume-want["volume"].as_f64().unwrap()).abs()<1e-4,"{} volume {volume}",row["name"]);
            assert!((area-want["area"].as_f64().unwrap()).abs()<1e-4,"{} area {area}",row["name"]);
            assert_eq!(euler,want["euler"].as_i64().unwrap());
        }
    }
}


#[test]
fn convex_union_keeps_both_authored_boundaries_for_placed_stock() {
    use std::collections::{BTreeMap,BTreeSet};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/🛠️modeling/🧩️placed-union.json")).unwrap();
    let source_faces:Vec<Vec<u32>>=serde_json::from_value(fixture["faces"].clone()).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        let source=|slot:&str|Soup{positions:serde_json::from_value(row[slot].clone()).unwrap(),faces:source_faces.clone(),normals:Vec::new()};
        for [first,second] in [["left","right"],["right","left"]]{
            let left=source(first);
            let right=source(second);
            let mut job=MeshConvexUnion::new(source(first),source(second)).unwrap();
            let output=loop{
                match job.advance().unwrap_or_else(|err|panic!("{} ({first} first): {err:?}",row["name"])){
                    MeshConvexIntersectionStep::Working=>{},
                    MeshConvexIntersectionStep::Solid(output)=>break output,
                    _=>panic!("{} lost material",row["name"]),
                }
            };
            let mut edges=BTreeMap::<_,[u32;2]>::new();
            let mut live=BTreeSet::new();
            let mut owner_seen=[false;2];
            let (mut area,mut volume)=(0.,0.);
            let point=|index:u32|Vec3(output.geometry.positions[index as usize]);
            for (index,ring) in output.geometry.faces.iter().enumerate(){
                let (operand,face)=match output.faces[index]{
                    MeshIntersectionFace::Left(face)=>(0,face),
                    MeshIntersectionFace::RightPlane(face)=>(1,face),
                    _=>panic!("Union needs an outward authored face"),
                };
                owner_seen[operand]=true;
                let source=[&left,&right][operand];
                let [a,b,c]=[0,1,2].map(|corner|Vec3(source.positions[source.faces[face as usize][corner] as usize]));
                let normal=b.sub(a).cross(c.sub(a));
                let normal=normal.scale(1./normal.dot(normal).sqrt());
                for (a_id,b_id) in ring.iter().copied().zip(ring.iter().copied().cycle().skip(1)){
                    assert!(point(a_id).sub(a).dot(normal).abs()<1e-5);
                    live.insert(a_id);
                    edges.entry((a_id.min(b_id),a_id.max(b_id))).or_insert([0;2])[usize::from(a_id<b_id)]+=1;
                }
                let first=point(ring[0]);
                for pair in ring[1..].windows(2){
                    let [a,b]=[point(pair[0]),point(pair[1])];
                    let triangle=a.sub(first).cross(b.sub(first)).dot(normal);
                    assert!(triangle>=-1e-6);
                    area+=f64::from(triangle)/2.;
                    volume+=f64::from(first.dot(a.cross(b)))/6.;
                }
            }
            assert_eq!(owner_seen,[true;2]);
            assert!(edges.values().all(|uses|*uses==[1,1]));
            assert_eq!(live.len()+output.geometry.faces.len(),edges.len()+2);
            assert!((area-row["area"].as_f64().unwrap()).abs()<1e-4);
            assert!((volume-row["volume"].as_f64().unwrap()).abs()<1e-4);
        }
    }
}
