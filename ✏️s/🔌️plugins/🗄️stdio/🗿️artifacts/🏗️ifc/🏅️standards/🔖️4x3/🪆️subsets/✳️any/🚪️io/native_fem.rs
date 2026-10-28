//! 🏗 Native unloaded analytical members with explicit canonical engineering definitions.
use super::*;
use super::graph::*;
use semio_s_artifact_fem_3d_model::standards::v1::subsets::any::schema::{snapshot::{Fem3dSnapshot,Fem3dSnapshotValue},structure::{FemDof,FemElement,FemSection,FemSupport},FemNodeValue,FemMaterialValue};
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::IfcEntity;
use std::collections::{BTreeMap,HashMap,HashSet};
const DOFS:[FemDof;6]=[FemDof::Tx,FemDof::Ty,FemDof::Tz,FemDof::Rx,FemDof::Ry,FemDof::Rz];

fn z_axis(start:[f64;3],end:[f64;3])->Result<[f64;3],ExchangeError> {
    let mut x=std::array::from_fn::<_,3,_>(|i|end[i]-start[i]);let length=x[0].hypot(x[1]).hypot(x[2]);
    if !length.is_finite()||length==0. {return Err(error("fem.length","Finite nonzero member length required"));}
    for n in &mut x {*n/=length;}
    let reference=if x[2].abs()>0.99 {[1.,0.,0.]} else {[0.,0.,1.]};
    let mut y=[reference[1]*x[2]-reference[2]*x[1],reference[2]*x[0]-reference[0]*x[2],reference[0]*x[1]-reference[1]*x[0]];
    let length=y[0].hypot(y[1]).hypot(y[2]);for n in &mut y {*n/=length;}
    Ok([x[1]*y[2]-x[2]*y[1],x[2]*y[0]-x[0]*y[2],x[0]*y[1]-x[1]*y[0]])
}
fn si(graph:&mut Builder,tag:&str,name:&str,prefix:IfcValue)->IfcValue {graph.add("IFCSIUNIT",vec![IfcValue::Derived,e(tag),prefix,e(name)])}
fn derived(graph:&mut Builder,tag:&str,parts:Vec<(IfcValue,i64)>)->IfcValue {
    let parts=parts.into_iter().map(|(unit,power)|graph.add("IFCDERIVEDUNITELEMENT",vec![unit,IfcValue::Integer(power)])).collect();
    graph.add("IFCDERIVEDUNIT",vec![a(parts),e(tag),IfcValue::Unset,IfcValue::Unset])
}
fn properties(graph:&mut Builder,values:Vec<(&str,&str,f64,IfcValue)>)->Vec<IfcValue> {values.into_iter().map(|(name,kind,value,unit)|graph.add("IFCPROPERTYSINGLEVALUE",vec![s(name),IfcValue::Unset,typed(kind,r(value)),unit])).collect()}
fn topology(graph:&mut Builder,item:IfcValue,kind:&str)->IfcValue {
    let representation=graph.add("IFCTOPOLOGYREPRESENTATION",vec![graph.context.clone(),s("Reference"),s(kind),a(vec![item])]);
    graph.add("IFCPRODUCTDEFINITIONSHAPE",vec![IfcValue::Unset,IfcValue::Unset,a(vec![representation])])
}

/// 📤 Export unloaded bars and zero-roll frames with canonical engineering references and boolean supports.
pub fn fem_to_ifc_controlled(source:&Fem3dSnapshot,control:&mut dyn FnMut(ExchangeProgress)->bool)->Result<Composition<IfcSnapshot>,ExchangeError> {
    checkpoint(control,"fem-begin",0,source.nodes.len()+source.elements.len())?;
    if source.nodes.is_empty()||!source.solids.is_empty()||!source.combinations.is_empty()||!source.load_cases.is_empty()||source.nodes.len()+source.elements.len()+source.materials.len()+source.sections.len()+source.supports.len()>64_000 {return Err(error("fem.unsupported","Native admission requires bounded nonempty unloaded members; solids, cases, combinations and self-weight are unsupported"));}
    source.validate_interaction_identities().map_err(|e|error("fem.identity",e.to_string()))?;
    let mut support_rows=HashMap::new();let mut section_rows=HashMap::new();
    for (index,row) in source.supports.iter().enumerate(){checkpoint(control,"fem-support-index",index,source.supports.len())?;if support_rows.insert(row.node_id.as_str(),row).is_some(){return Err(error("fem.support","At most one support per node is admitted"));}}
    for (index,row) in source.sections.iter().enumerate(){checkpoint(control,"fem-section-index",index,source.sections.len())?;section_rows.insert(row.id.as_str(),row);}
    let mut graph=Builder::new("FEM3d analytical projection");
    let metre=IfcValue::Reference(graph.snapshot.entities.iter().find(|e|e.name=="IFCSIUNIT").unwrap().id);
    let area=si(&mut graph,"AREAUNIT","SQUARE_METRE",IfcValue::Unset);let pressure=si(&mut graph,"PRESSUREUNIT","PASCAL",IfcValue::Unset);let mass=si(&mut graph,"MASSUNIT","GRAM",e("KILO"));
    let density=derived(&mut graph,"MASSDENSITYUNIT",vec![(mass.clone(),1),(metre.clone(),-3)]);let inertia=derived(&mut graph,"MOMENTOFINERTIAUNIT",vec![(metre.clone(),4)]);
    graph.snapshot.entities.iter_mut().find(|e|e.name=="IFCUNITASSIGNMENT").unwrap().args[0]=a(vec![metre,area.clone(),pressure.clone(),mass,density.clone(),inertia.clone()]);
    let mut materials:BTreeMap<&str,IfcValue>=BTreeMap::new();let mut nodes:BTreeMap<&str,(IfcValue,IfcValue,[f64;3])>=BTreeMap::new();let mut sections:BTreeMap<String,IfcValue>=BTreeMap::new();let mut members=Vec::new();
    for (index,material) in source.materials.iter().enumerate() {
        checkpoint(control,"fem-materials",index,source.materials.len())?;
        let entity=graph.add("IFCMATERIAL",vec![s(material.name.as_str()),IfcValue::Unset,IfcValue::Unset]);
        let props=properties(&mut graph,vec![("YoungModulus","IFCMODULUSOFELASTICITYMEASURE",material.e,pressure.clone()),("ShearModulus","IFCMODULUSOFELASTICITYMEASURE",material.g,pressure.clone()),("PoissonRatio","IFCRATIOMEASURE",material.nu,IfcValue::Unset),("MassDensity","IFCMASSDENSITYMEASURE",material.rho,density.clone())]);
        graph.add("IFCMATERIALPROPERTIES",vec![s("semio_FemMaterial"),IfcValue::Unset,a(props),entity.clone()]);materials.insert(material.id.as_str(),entity);
    }
    for (index,node) in source.nodes.iter().enumerate() {
        checkpoint(control,"fem-nodes",index,source.nodes.len())?;
        let xyz=[node.x,node.y,node.z];if xyz.iter().any(|n|!n.is_finite()){return Err(error("fem.node","Finite coordinates required"));}
        let point=graph.add("IFCCARTESIANPOINT",vec![a(xyz.into_iter().map(r).collect())]);let vertex=graph.add("IFCVERTEXPOINT",vec![point]);let shape=topology(&mut graph,vertex.clone(),"Vertex");
        let condition=if let Some(support)=support_rows.get(node.id.as_str()) {
            if support.fixed.iter().enumerate().any(|(i,d)|support.fixed[..i].contains(d)) {return Err(error("fem.support","Duplicate restraint DOF"));}
            let mut args=vec![s(&support.id)];args.extend(DOFS.iter().map(|d|typed("IFCBOOLEAN",e(if support.fixed.contains(d){"T"}else{"F"}))));graph.add("IFCBOUNDARYNODECONDITION",args)
        } else {IfcValue::Unset};
        let entity=graph.rooted("IFCSTRUCTURALPOINTCONNECTION",node.id.as_str(),vec![IfcValue::Unset,IfcValue::Unset,shape,condition,IfcValue::Unset]);nodes.insert(node.id.as_str(),(entity,vertex,xyz));
    }
    for support in &source.supports {if !nodes.contains_key(support.node_id.as_str()){return Err(error("fem.support","Support references missing node"));}}
    for (index,member) in source.elements.iter().enumerate() {
        checkpoint(control,"fem-members",index,source.elements.len())?;
        let (name,start,end,material,section,pinned)=match member {
            FemElement::Bar {id,start,end,material_id,section_id}=>(id,start,end,material_id,section_id,true),
            FemElement::Frame {id,start,end,material_id,section_id,roll} if *roll==0.=>(id,start,end,material_id,section_id,false),
            _=>return Err(error("fem.roll","Native frame admission currently requires zero roll")),
        };
        let (start_entity,start_vertex,start_xyz)=nodes.get(start.as_str()).ok_or_else(||error("fem.reference","Missing start node"))?;
        let (end_entity,end_vertex,end_xyz)=nodes.get(end.as_str()).ok_or_else(||error("fem.reference","Missing end node"))?;
        let local_z=z_axis(*start_xyz,*end_xyz)?;let axis=graph.add("IFCDIRECTION",vec![a(local_z.into_iter().map(r).collect())]);
        let edge=graph.add("IFCEDGE",vec![start_vertex.clone(),end_vertex.clone()]);let shape=topology(&mut graph,edge,"Edge");
        let entity=graph.rooted("IFCSTRUCTURALCURVEMEMBER",name,vec![IfcValue::Unset,IfcValue::Unset,shape,e(if pinned{"PIN_JOINED_MEMBER"}else{"RIGID_JOINED_MEMBER"}),axis]);
        graph.rooted("IFCRELASSOCIATESMATERIAL","",vec![a(vec![entity.clone()]),materials.get(material.as_str()).ok_or_else(||error("fem.material","Missing member material"))?.clone()]);
        let pset=if let Some(pset)=sections.get(section) {pset.clone()} else {
            let section_data=section_rows.get(section.as_str()).ok_or_else(||error("fem.section","Missing section"))?;
            if [section_data.area,section_data.iy,section_data.iz,section_data.j].iter().any(|n|!n.is_finite()||*n<=0.) {return Err(error("fem.section","Finite positive section properties required"));}
            let mut props=properties(&mut graph,vec![("CrossSectionArea","IFCAREAMEASURE",section_data.area,area.clone()),("MomentOfInertiaY","IFCMOMENTOFINERTIAMEASURE",section_data.iy,inertia.clone()),("MomentOfInertiaZ","IFCMOMENTOFINERTIAMEASURE",section_data.iz,inertia.clone()),("TorsionalConstant","IFCMOMENTOFINERTIAMEASURE",section_data.j,inertia.clone())]);
            props.push(graph.add("IFCPROPERTYSINGLEVALUE",vec![s("SectionName"),IfcValue::Unset,typed("IFCLABEL",s(&section_data.name)),IfcValue::Unset]));
            let pset=graph.rooted("IFCPROPERTYSET","semio_FemSection",vec![a(props)]);sections.insert(section.clone(),pset.clone());pset
        };
        graph.rooted("IFCRELDEFINESBYPROPERTIES","",vec![a(vec![entity.clone()]),pset]);
        for node in [start_entity,end_entity] {graph.rooted("IFCRELCONNECTSSTRUCTURALMEMBER","",vec![entity.clone(),node.clone(),IfcValue::Unset,IfcValue::Unset,IfcValue::Unset,IfcValue::Unset]);}
        members.push(entity);
    }
    let analysis=graph.rooted("IFCSTRUCTURALANALYSISMODEL","FEM3d",vec![IfcValue::Unset,e("LOADING_3D"),IfcValue::Unset,IfcValue::Unset,IfcValue::Unset,IfcValue::Unset]);
    let objects=nodes.values().map(|(entity,_,_)|entity.clone()).chain(members).collect::<Vec<_>>();
    if !objects.is_empty(){graph.rooted("IFCRELASSIGNSTOGROUP","",vec![a(objects),IfcValue::Unset,analysis.clone()]);}
    let project=IfcValue::Reference(graph.snapshot.entities.iter().find(|e|e.name=="IFCPROJECT").unwrap().id);graph.rooted("IFCRELDECLARES","",vec![project,a(vec![analysis])]);
    super::projection::validate(&graph.snapshot,control)?;
    Ok(Composition {snapshot:graph.snapshot,confidence:Confidence::High,diagnostics:vec![loss("Native unloaded analytical projection retains nodes, bars/zero-roll frames, canonical materials/sections and boolean supports; source identities, unused definitions and solver/display settings are omitted; load cases, combinations and solids are refused")]})
}

fn item<'a>(graph:&Reader<'a>,entity:&IfcEntity)->Result<&'a IfcEntity,ExchangeError> {
    if entity.args.get(5)!=Some(&IfcValue::Unset) {return Err(error("fem.placement","Native unloaded admission requires explicit global structural placement"));}
    let shape=graph.reference(entity.args.get(6))?;let reps=list(shape.args.get(2))?;
    if shape.name!="IFCPRODUCTDEFINITIONSHAPE"||reps.len()!=1{return Err(error("fem.topology","One analytical shape representation required"));}
    let rep=graph.reference(reps.first())?;let items=list(rep.args.get(3))?;
    if rep.name!="IFCTOPOLOGYREPRESENTATION"||items.len()!=1{return Err(error("fem.topology","One reference topology item required"));}
    let context=graph.reference(rep.args.first())?;
    if context.name!="IFCGEOMETRICREPRESENTATIONCONTEXT"||context.args.get(2)!=Some(&IfcValue::Integer(3)){return Err(error("fem.context","Direct 3D geometric context required"));}
    graph.reference(items.first())
}
fn engineering(graph:&Reader<'_>,property:&IfcEntity,kind:&str,dimensions:[i32;7],control:&mut dyn FnMut(ExchangeProgress)->bool)->Result<f64,ExchangeError> {
    let Some(IfcValue::TypedValue(nominal))=property.args.get(2) else{return Err(error("fem.measure","Typed engineering property required"));};
    if nominal.name!=kind||nominal.items.len()!=1{return Err(error("fem.measure","Wrong engineering measure type"));}
    if dimensions==[0;7] {if property.args.get(3)!=Some(&IfcValue::Unset){return Err(error("fem.unit","Dimensionless ratio cannot carry physical unit"));}} else {
        let resolved=graph.unit(graph.reference(property.args.get(3))?,control)?;if resolved!=(dimensions,1.){return Err(error("fem.unit","Explicit matching SI engineering dimensions required"));}
    }
    number(nominal.items.first())
}

/// 📥 Produce the FEM owner's explicit canonical authoring value before its actual controlled native receiving boundary.
pub fn ifc_to_fem_reference_controlled(snapshot:&IfcSnapshot,control:&mut dyn FnMut(ExchangeProgress)->bool)->Result<Composition<Fem3dSnapshotValue>,ExchangeError> {
    let graph=Reader::new(snapshot,control)?;
    if graph.scale!=1. {return Err(error("fem.unit","Native analytical admission requires SI metre project units"));}
    let analyses=snapshot.entities.iter().filter(|e|e.name=="IFCSTRUCTURALANALYSISMODEL").collect::<Vec<_>>();
    let [analysis]=analyses.as_slice() else{return Err(error("fem.analysis","One structural analysis required"));};
    if analysis.args.get(5).and_then(IfcValue::as_enum)!=Some("LOADING_3D")||(6..=9).any(|i|analysis.args.get(i)!=Some(&IfcValue::Unset)){return Err(error("fem.analysis","Only unloaded global 3D structural analysis is admitted"));}
    let items=snapshot.entities.iter().filter(|e|matches!(e.name.as_str(),"IFCSTRUCTURALPOINTCONNECTION"|"IFCSTRUCTURALCURVEMEMBER")).map(|e|e.id).collect::<HashSet<_>>();
    if items.len()>64_000||items.is_empty(){return Err(error("fem.analysis","Bounded nonempty structural items required"));}
    let mut membership=HashSet::new();
    for (index,entity) in snapshot.entities.iter().enumerate() {
        if index%128==0 {checkpoint(control,"fem-admission",index,snapshot.entities.len())?;}
        if entity.name.starts_with("IFCSTRUCTURAL")&&!matches!(entity.name.as_str(),"IFCSTRUCTURALANALYSISMODEL"|"IFCSTRUCTURALPOINTCONNECTION"|"IFCSTRUCTURALCURVEMEMBER"){return Err(error("fem.unsupported","Loaded/specialized structural semantics are outside native unloaded admission"));}
        if entity.name.starts_with("IFCRELCONNECTSSTRUCTURAL")&&entity.name!="IFCRELCONNECTSSTRUCTURALMEMBER"{return Err(error("fem.unsupported","Specialized structural connection semantics require additional mappings"));}
        if entity.name.starts_with("IFCRELASSIGNSTOGROUP")&&entity.args.get(6).and_then(IfcValue::as_reference)==Some(analysis.id){
            if entity.name!="IFCRELASSIGNSTOGROUP" {return Err(error("fem.membership","Factored group membership is unsupported"));}
            let members=list(entity.args.get(4))?;
            for (index,value) in members.iter().enumerate() {checkpoint(control,"fem-membership",index,members.len())?;if !membership.insert(id(value)?){return Err(error("fem.membership","Repeated structural membership"));}}
        }
        if entity.name=="IFCGEOMETRICREPRESENTATIONCONTEXT" {let frame=graph.axis(entity.args.get(4))?;if entity.args.get(2)!=Some(&IfcValue::Integer(3))||frame.position!=[0.;3]||frame.angle!=0. {return Err(error("fem.context","Identity 3D analytical geometry context required"));}}
    }
    if membership!=items{return Err(error("fem.membership","Every admitted structural item must belong to selected analysis"));}
    let mut output=Fem3dSnapshotValue::default();output.analysis.modal_count=0;output.analysis.buckling_count=0;output.analysis.deformation_scale=1.;
    let mut vertices=HashMap::new();let mut node_entities=HashMap::new();let mut positions=HashMap::new();let mut material_ids:HashMap<u64,String>=HashMap::new();let mut section_ids:HashMap<u64,String>=HashMap::new();
    for entity in snapshot.entities.iter().filter(|e|e.name=="IFCSTRUCTURALPOINTCONNECTION") {
        checkpoint(control,"fem-nodes",output.nodes.len(),items.len())?;
        if entity.args.get(8)!=Some(&IfcValue::Unset){return Err(error("fem.support-axes","Local support axes require conversion"));}
        let vertex=item(&graph,entity)?;if vertex.name!="IFCVERTEXPOINT" {return Err(error("fem.topology","Point connection requires vertex"));}
        let xyz=graph.vector(vertex.args.first(),"IFCCARTESIANPOINT")?;if xyz.len()!=3{return Err(error("fem.topology","3D analytical node required"));}
        let key=format!("ifc{}",entity.id);if vertices.insert(vertex.id,key.clone()).is_some(){return Err(error("fem.topology","Shared vertex has ambiguous connection identity"));}
        positions.insert(key.clone(),[xyz[0],xyz[1],xyz[2]]);node_entities.insert(entity.id,key.clone());output.nodes.push(FemNodeValue {id:key.clone(),x:xyz[0],y:xyz[1],z:xyz[2]});
        if entity.args.get(7)!=Some(&IfcValue::Unset){let condition=graph.reference(entity.args.get(7))?;if condition.name!="IFCBOUNDARYNODECONDITION"||condition.args.len()!=7{ return Err(error("fem.support","Unsupported boundary condition"));}
            let mut fixed=Vec::new();for (i,dof) in DOFS.iter().enumerate(){let Some(IfcValue::TypedValue(value))=condition.args.get(i+1) else{return Err(error("fem.support","Explicit boolean stiffness selectors required"));};if value.name!="IFCBOOLEAN"||value.items.len()!=1{return Err(error("fem.support","Spring/select stiffness is unsupported"));}match value.items[0].as_enum(){Some("T")=>fixed.push(*dof),Some("F")=>{},_=>return Err(error("fem.support","Boolean support required"))}}
            output.supports.push(FemSupport {id:format!("ifc{}",entity.id),node_id:key,fixed});}
    }
    for entity in snapshot.entities.iter().filter(|e|e.name=="IFCSTRUCTURALCURVEMEMBER") {
        checkpoint(control,"fem-members",output.elements.len(),items.len())?;
        let edge=item(&graph,entity)?;if edge.name!="IFCEDGE"{return Err(error("fem.topology","Member requires edge"));}
        let start=vertices.get(&id(edge.args.first().ok_or_else(||error("fem.topology","Missing edge start"))?)?).ok_or_else(||error("fem.topology","Missing start connection"))?.clone();let end=vertices.get(&id(edge.args.get(1).ok_or_else(||error("fem.topology","Missing edge end"))?)?).ok_or_else(||error("fem.topology","Missing end connection"))?.clone();
        let expected=z_axis(positions[&start],positions[&end])?;let axis=graph.vector(entity.args.get(8),"IFCDIRECTION")?;
        if axis.len()!=3{return Err(error("fem.axis","3D member axis required"));}let scale=axis.iter().fold(0.0f64,|m,v|m.max(v.abs()));if scale==0.{return Err(error("fem.axis","Nonzero member axis required"));}let norm=(axis[0]/scale).hypot(axis[1]/scale).hypot(axis[2]/scale);if (0..3).any(|i|(axis[i]/scale/norm-expected[i]).abs()>1e-10){return Err(error("fem.roll","Native admission requires reference zero-roll axis"));}
        let mut endpoints=HashSet::new();let mut material=None;
        for relation in graph.connections.get(&entity.id).into_iter().flatten() {
            checkpoint(control,"fem-connections",endpoints.len(),2)?;
            if (6..=9).any(|i|relation.args.get(i)!=Some(&IfcValue::Unset)){return Err(error("fem.release","Member conditions/releases require additional semantics"));}let endpoint=id(relation.args.get(5).ok_or_else(||error("fem.endpoint","Missing endpoint"))?)?;if !endpoints.insert(endpoint){return Err(error("fem.endpoint","Repeated endpoint relation"));}
        }
        for relation in graph.materials.get(&entity.id).into_iter().flatten() {
            checkpoint(control,"fem-material-reference",0,0)?;
            let candidate=id(relation.args.get(5).ok_or_else(||error("fem.material","Missing material reference"))?)?;if material.is_some_and(|m|m!=candidate){return Err(error("fem.material","Ambiguous member material"));}material=Some(candidate);
        }
        if endpoints.len()!=2||endpoints.iter().any(|id|node_entities.get(id).is_none_or(|key|key!=&start&&key!=&end)){return Err(error("fem.endpoint","Edge topology must agree with two unmodified member connections"));}
        let material=material.ok_or_else(||error("fem.material","Explicit material required"))?;
        let material_id=if let Some(key)=material_ids.get(&material){key.clone()}else{
            let row=graph.entity(material)?;if row.name!="IFCMATERIAL"{return Err(error("fem.material","Plain material definition required"));}
            let definitions=graph.material_properties.get(&material).into_iter().flatten().filter(|p|p.args.first().and_then(IfcValue::as_str)==Some("semio_FemMaterial")).collect::<Vec<_>>();let [definition]=definitions.as_slice() else{return Err(error("fem.material","One explicit canonical material property definition required"));};
            let values=list(definition.args.get(2))?;
            let mut properties=BTreeMap::new();for (index,value) in values.iter().enumerate(){checkpoint(control,"fem-material-properties",index,values.len())?;let p=graph.reference(Some(value))?;if p.name!="IFCPROPERTYSINGLEVALUE"||properties.insert(text(p.args.first()),p).is_some(){return Err(error("fem.material","Unambiguous material properties required"));}}
            let mut property=|name:&str,kind:&str,dim|engineering(&graph,properties.get(name).ok_or_else(||error("fem.material",format!("Missing {name}")))?,kind,dim,control);
            let e=property("YoungModulus","IFCMODULUSOFELASTICITYMEASURE",[-1,1,-2,0,0,0,0])?;let g=property("ShearModulus","IFCMODULUSOFELASTICITYMEASURE",[-1,1,-2,0,0,0,0])?;let nu=property("PoissonRatio","IFCRATIOMEASURE",[0;7])?;let rho=property("MassDensity","IFCMASSDENSITYMEASURE",[-3,1,0,0,0,0,0])?;
            if e<=0.||g<=0.||rho<=0.||nu<=-1.||nu>=0.5{return Err(error("fem.material","Physical isotropic engineering values required"));}
            let key=format!("ifc{material}");output.materials.push(FemMaterialValue {id:key.clone(),name:text(row.args.first()),e,g,nu,rho});material_ids.insert(material,key.clone());key
        };
        let (section,properties)=graph.properties(entity.id,"semio_FemSection",control)?;
        let section_id=if let Some(key)=section_ids.get(&section){key.clone()}else{
            let mut property=|name:&str,kind:&str,dim|engineering(&graph,properties.get(name).ok_or_else(||error("fem.section",format!("Missing {name}")))?,kind,dim,control);
            let area=property("CrossSectionArea","IFCAREAMEASURE",[2,0,0,0,0,0,0])?;let iy=property("MomentOfInertiaY","IFCMOMENTOFINERTIAMEASURE",[4,0,0,0,0,0,0])?;let iz=property("MomentOfInertiaZ","IFCMOMENTOFINERTIAMEASURE",[4,0,0,0,0,0,0])?;let j=property("TorsionalConstant","IFCMOMENTOFINERTIAMEASURE",[4,0,0,0,0,0,0])?;
            if [area,iy,iz,j].iter().any(|n|*n<=0.){return Err(error("fem.section","Positive section values required"));}
            let name=properties.get("SectionName").ok_or_else(||error("fem.section","Explicit section name required"))?;let Some(IfcValue::TypedValue(value))=name.args.get(2) else{return Err(error("fem.section","Typed section name required"));};if value.name!="IFCLABEL"||value.items.len()!=1||name.args.get(3)!=Some(&IfcValue::Unset)||value.items[0].as_str().is_none(){return Err(error("fem.section","Dimensionless label section name required"));}
            let key=format!("ifc{section}");output.sections.push(FemSection {id:key.clone(),name:text(value.items.first()),area,iy,iz,j});section_ids.insert(section,key.clone());key
        };
        let id=format!("ifc{}",entity.id);output.elements.push(match entity.args.get(7).and_then(IfcValue::as_enum){Some("PIN_JOINED_MEMBER")=>FemElement::Bar {id,start,end,material_id,section_id},Some("RIGID_JOINED_MEMBER")=>FemElement::Frame {id,start,end,material_id,section_id,roll:0.},_=>return Err(error("fem.member","Explicit pin/rigid member kind required"))});
    }
    output.validate_interaction_identities().map_err(|e|error("fem.identity",e.to_string()))?;
    if output.nodes.len()+output.elements.len()+output.materials.len()+output.sections.len()+output.supports.len()>64_000 {return Err(error("exchange.limit","Native analytical target exceeds retained entity count bound"));}
    checkpoint(control,"fem-complete",output.elements.len(),output.elements.len())?;
    Ok(Composition {snapshot:output,confidence:Confidence::High,diagnostics:vec![loss("Native unloaded analytical projection retains admitted engineering topology, canonical shared definitions and supports; loads, combinations, solids, releases, local frames, nonzero roll and missing units are refused; solver/display settings and source artifact identities are not retained")]})
}

#[cfg(test)]
#[path="🧪️tests/native_fem.rs"]
mod tests;
