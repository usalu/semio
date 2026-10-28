//! 📽️ Deterministic XY edge projection through firstparty Part21 entities and placement axes.
use semio_s_artifact_stdio_contract::part21::{Part21Document,Part21Instance,Part21Value};
pub(super) struct ProjectionGraph<'a>{doc:&'a Part21Document,instances:std::collections::HashMap<u64,&'a Part21Instance>}
impl<'a> ProjectionGraph<'a>{pub(super) fn new(doc:&'a Part21Document)->Self{Self{doc,instances:doc.instances.iter().map(|instance|(instance.id,instance)).collect()}}pub(super) fn instance(&self,id:u64)->Option<&'a Part21Instance>{self.instances.get(&id).copied()}pub(super) fn by_type<'b>(&'b self,kind:&'b str)->impl Iterator<Item=&'a Part21Instance>+'b{self.doc.instances.iter().filter(move|instance|instance.is_type(kind))}}
#[derive(Clone,Debug,PartialEq)]
pub struct ProjectedCurve{pub source_id:u64,pub points:Vec<[f64;3]>}
#[derive(Clone,Debug,PartialEq)]
pub struct EdgeProjection{pub curves:Vec<ProjectedCurve>,pub unit_to_points:f64,pub issues:Vec<String>}
fn coordinates(doc:&ProjectionGraph<'_>,id:u64,kind:&str)->Option<[f64;3]>{let args=doc.instance(id)?.entity(kind)?;let values=args.get(1)?.as_list()?;Some([values.first()?.as_real()?,values.get(1)?.as_real()?,values.get(2).and_then(Part21Value::as_real).unwrap_or(0.0)])}
fn reference(args:&[Part21Value],index:usize)->Option<u64>{args.get(index)?.as_ref_id()}
fn vertex(doc:&ProjectionGraph<'_>,id:u64)->Option<[f64;3]>{coordinates(doc,reference(doc.instance(id)?.entity("VERTEX_POINT")?,1)?,"CARTESIAN_POINT")}
fn dot(a:[f64;3],b:[f64;3])->f64{a[0]*b[0]+a[1]*b[1]+a[2]*b[2]}
fn norm(a:[f64;3])->Option<[f64;3]>{let length=dot(a,a).sqrt();(length>1e-12&&length.is_finite()).then(||a.map(|value|value/length))}
fn cross(a:[f64;3],b:[f64;3])->[f64;3]{[a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]]}
pub(super) fn placement(doc:&ProjectionGraph<'_>,id:u64)->Option<([f64;3],[f64;3],[f64;3])>{
    let instance=doc.instance(id)?;let two=instance.entity("AXIS2_PLACEMENT_3D").is_none();let args=instance.entity("AXIS2_PLACEMENT_3D").or_else(||instance.entity("AXIS2_PLACEMENT_2D"))?;
    let center=coordinates(doc,reference(args,1)?,"CARTESIAN_POINT")?;
    let normal=if two{[0.0,0.0,1.0]}else{reference(args,2).and_then(|id|coordinates(doc,id,"DIRECTION")).unwrap_or([0.0,0.0,1.0])};
    let z=norm(normal)?;let axis=reference(args,if two{2}else{3}).and_then(|id|coordinates(doc,id,"DIRECTION")).unwrap_or([1.0,0.0,0.0]);
    let component=dot(axis,z);let x=norm([axis[0]-component*z[0],axis[1]-component*z[1],axis[2]-component*z[2]])?;let y=cross(z,x);Some((center,x,y))
}
pub(super) fn length_unit(doc:&ProjectionGraph<'_>)->Result<f64,String>{
    let mut scale=None;
    for instance in doc.by_type("LENGTH_UNIT"){
        let Some(args)=instance.entity("SI_UNIT")else{return Err("step.projection.length-unit".into());};
        if !matches!(args.get(1),Some(Part21Value::Enum(value))if value=="METRE"){return Err("step.projection.length-unit".into());}
        let factor=match args.first(){Some(Part21Value::Enum(value))=>match value.as_str(){"MILLI"=>0.001,"CENTI"=>0.01,"DECI"=>0.1,"MICRO"=>0.000001,"KILO"=>1000.0,_=>return Err("step.projection.length-prefix".into())},_=>1.0};
        if scale.is_some_and(|previous:f64|(previous-factor).abs()>1e-12){return Err("step.projection.mixed-units".into());}scale=Some(factor);
    }
    Ok(scale.unwrap_or(0.0254/72.0)*72.0/0.0254)
}
fn edge(doc:&ProjectionGraph<'_>,id:u64)->Result<ProjectedCurve,String>{
    let args=doc.instance(id).and_then(|instance|instance.entity("EDGE_CURVE")).ok_or("step.projection.edge")?;
    let start=vertex(doc,reference(args,1).ok_or("step.projection.vertex")?).ok_or("step.projection.vertex")?;
    let end=vertex(doc,reference(args,2).ok_or("step.projection.vertex")?).ok_or("step.projection.vertex")?;
    let curve=doc.instance(reference(args,3).ok_or("step.projection.curve")?).ok_or("step.projection.curve")?;
    let points=if curve.entity("LINE").is_some(){vec![start,end]}else if let Some(circle)=curve.entity("CIRCLE"){
        let(center,x,y)=placement(doc,reference(circle,1).ok_or("step.projection.placement")?).ok_or("step.projection.placement")?;
        let radius=circle.get(2).and_then(Part21Value::as_real).filter(|radius|radius.is_finite()&&*radius>0.0).ok_or("step.projection.radius")?;
        let angle=|point:[f64;3]|{let relative=[point[0]-center[0],point[1]-center[1],point[2]-center[2]];dot(relative,y).atan2(dot(relative,x))};
        let first=angle(start);let last=angle(end);let forward=matches!(args.get(4),Some(Part21Value::Enum(value))if value=="T");
        let distance=start.iter().zip(end).map(|(a,b)|(a-b)*(a-b)).sum::<f64>().sqrt();
        let sweep=if distance<=radius*1e-9{if forward{std::f64::consts::TAU}else{-std::f64::consts::TAU}}else if forward{(last-first).rem_euclid(std::f64::consts::TAU)}else{-(first-last).rem_euclid(std::f64::consts::TAU)};
        let segments=(sweep.abs()/(std::f64::consts::PI/32.0)).ceil().clamp(1.0,64.0)as usize;
        (0..=segments).map(|index|{let a=first+sweep*index as f64/segments as f64;let(sin,cos)=a.sin_cos();[center[0]+radius*(cos*x[0]+sin*y[0]),center[1]+radius*(cos*x[1]+sin*y[1]),center[2]+radius*(cos*x[2]+sin*y[2])]}).collect()
    }else{return Err(format!("step.projection.unsupported-curve:{}",curve.primary().map_or("?",|(kind,_)|kind)));};
    if points.iter().flatten().any(|value|!value.is_finite()){return Err("step.projection.non-finite".into());}Ok(ProjectedCurve{source_id:id,points})
}
fn geometric_curve(doc:&ProjectionGraph<'_>,id:u64)->Result<ProjectedCurve,String>{let instance=doc.instance(id).ok_or("step.projection.curve")?;let points=if let Some(args)=instance.entity("POLYLINE"){let refs=args.get(1).and_then(Part21Value::as_list).ok_or("step.projection.polyline")?;if refs.len()<2||refs.len()>1025{return Err("step.projection.polyline-limit".into());}refs.iter().map(|value|value.as_ref_id().and_then(|id|coordinates(doc,id,"CARTESIAN_POINT")).ok_or_else(||"step.projection.vertex".to_string())).collect::<Result<Vec<_>,_>>()?}else if let Some(args)=instance.entity("CIRCLE"){let(center,x,y)=placement(doc,reference(args,1).ok_or("step.projection.placement")?).ok_or("step.projection.placement")?;let radius=args.get(2).and_then(Part21Value::as_real).filter(|radius|radius.is_finite()&&*radius>0.0).ok_or("step.projection.radius")?;(0..=64).map(|index|{let angle=std::f64::consts::TAU*index as f64/64.0;let(sin,cos)=angle.sin_cos();[center[0]+radius*(cos*x[0]+sin*y[0]),center[1]+radius*(cos*x[1]+sin*y[1]),center[2]+radius*(cos*x[2]+sin*y[2])]}).collect()}else{return Err(format!("step.projection.unsupported-curve:{}",instance.primary().map_or("?",|(kind,_)|kind)));};if points.iter().flatten().any(|value|!value.is_finite()){return Err("step.projection.non-finite".into());}Ok(ProjectedCurve{source_id:id,points})}
/// 🧭️ Straight and circular trim edges are sampled by at most64segments peredge; solids and PMI remain owned by STEP.
pub fn project_edges(doc:&Part21Document)->Result<EdgeProjection,String>{
    let indexed=ProjectionGraph::new(doc);let doc=&indexed;
    let mut output=EdgeProjection{curves:Vec::new(),unit_to_points:length_unit(doc)?,issues:Vec::new()};let mut point_count=0usize;
    for instance in doc.by_type("EDGE_CURVE"){
        if output.curves.len()+output.issues.len()>=16_384{return Err("step.projection.edge-limit".into());}
        match edge(doc,instance.id){Ok(mut curve)=>{point_count+=curve.points.len();if point_count>131_072{return Err("step.projection.point-limit".into());}for point in &mut curve.points{for value in point{*value*=output.unit_to_points;}}output.curves.push(curve);},Err(issue)=>output.issues.push(format!("#{}:{issue}",instance.id))}
    }
    let mut seen=std::collections::HashSet::new();for set in doc.by_type("GEOMETRIC_CURVE_SET"){let refs=set.entity("GEOMETRIC_CURVE_SET").and_then(|args|args.get(1)).and_then(Part21Value::as_list).ok_or("step.projection.curve-set")?;for value in refs{let id=value.as_ref_id().ok_or("step.projection.curve")?;if !seen.insert(id){continue;}if output.curves.len()+output.issues.len()>=16_384{return Err("step.projection.edge-limit".into());}match geometric_curve(doc,id){Ok(mut curve)=>{point_count+=curve.points.len();if point_count>131_072{return Err("step.projection.point-limit".into());}for point in &mut curve.points{for value in point{*value*=output.unit_to_points;}}output.curves.push(curve);},Err(issue)=>output.issues.push(format!("#{id}:{issue}"))}}}
    if doc.by_type("MAPPED_ITEM").next().is_some()||doc.by_type("ITEM_DEFINED_TRANSFORMATION").next().is_some(){output.issues.push("step.projection.assembly-occurrence-transforms-not-applied".into());}
    if doc.by_type("LENGTH_UNIT").next().is_none(){output.issues.push("step.projection.unspecified-unit:one-coordinate-unit-is-one-point".into());}
    Ok(output)
}
