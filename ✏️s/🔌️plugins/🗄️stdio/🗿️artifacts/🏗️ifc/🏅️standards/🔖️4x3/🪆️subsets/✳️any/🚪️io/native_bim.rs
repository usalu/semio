//! 🏛 Native source-owned BIM spatial projection without inferred element families.
use super::*;
use super::graph::*;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::{ModelSnapshot,Site,Building,Storey};
use semio_s_artifact_bim_model_schema::standards::v1::subsets::any::schema::snapshot::values::Point2;
use std::collections::{BTreeMap,HashMap};

fn compound(angle:f64)->Result<IfcValue,ExchangeError> {
    if !angle.is_finite()||angle.abs()>180. {return Err(error("bim.angle","Finite geographic angle within 180 degrees required"));}
    let mut micro=(angle.abs()*3_600_000_000.).round() as i64;
    let sign=if angle<0. {-1} else {1};
    let degrees=micro/3_600_000_000;micro%=3_600_000_000;
    let minutes=micro/60_000_000;micro%=60_000_000;
    Ok(a(vec![degrees,minutes,micro/1_000_000,micro%1_000_000].into_iter().map(|n|IfcValue::Integer(n*sign)).collect()))
}
fn angle(value:Option<&IfcValue>)->Result<f64,ExchangeError> {
    let parts=list(value)?;
    if !(3..=4).contains(&parts.len())||parts.iter().any(|v|!matches!(v,IfcValue::Integer(_))) {return Err(error("bim.angle","Explicit compound integer angle required"));}
    let p=parts.iter().map(|v|number(Some(v))).collect::<Result<Vec<_>,_>>()?;
    if p[1].abs()>=60.||p[2].abs()>=60.||p.get(3).is_some_and(|n|n.abs()>=1_000_000.)||p.iter().any(|n|*n<0.)&&p.iter().any(|n|*n>0.) {return Err(error("bim.angle","Invalid signed compound angle"));}
    Ok(p[0]+p[1]/60.+p[2]/3600.+p.get(3).copied().unwrap_or(0.)/3_600_000_000.)
}
fn elevations<'a>(storeys:&'a BTreeMap<String,Storey>,control:&mut dyn FnMut(ExchangeProgress)->bool)->Result<HashMap<&'a str,f64>,ExchangeError> {
    let mut groups:BTreeMap<&str,BTreeMap<i32,(&str,f64)>>=BTreeMap::new();
    for (index,(key,storey)) in storeys.iter().enumerate() {
        checkpoint(control,"bim-levels",index,storeys.len())?;
        if !storey.height.is_finite()||storey.height<=0.||storey.cut_height.is_some_and(|v|!v.is_finite())||groups.entry(&storey.building).or_default().insert(storey.level,(key,storey.height)).is_some(){return Err(error("bim.storey","Unique levels and finite positive storey heights required"));}
    }
    let mut output=HashMap::new();
    for group in groups.values() {
        let mut above=0.0f64;let mut below=0.0f64;
        for (_,entry) in group.range(0..) {let (key,height)=*entry;checkpoint(control,"bim-levels-positive",output.len(),storeys.len())?;output.insert(key,above);above+=height;if !above.is_finite(){return Err(error("bim.height","Cumulative height exceeds finite range"));}}
        for (_,entry) in group.range(..0).rev() {let (key,height)=*entry;checkpoint(control,"bim-levels-negative",output.len(),storeys.len())?;below-=height;if !below.is_finite(){return Err(error("bim.height","Cumulative height exceeds finite range"));}output.insert(key,below);}
    }
    Ok(output)
}
fn length(value:f64,scale:f64)->Result<f64,ExchangeError>{let result=value*scale;if result.is_finite(){Ok(result)}else{Err(error("bim.unit","Spatial length exceeds finite SI range"))}}

/// 📤 Preserve authored project/site/building/storey values through actual ADD2 spatial records.
pub fn bim_to_ifc_controlled(source:&ModelSnapshot,control:&mut dyn FnMut(ExchangeProgress)->bool)->Result<Composition<IfcSnapshot>,ExchangeError> {
    checkpoint(control,"bim-begin",0,source.sites.len()+source.buildings.len()+source.storeys.len())?;
    if source.schema!="s.bim.model@1"||source.sites.len()+source.buildings.len()+source.storeys.len()>64_000 {return Err(error("bim.schema","Expected a bounded native BIM model"));}
    if source.project.phase_names.iter().any(|p|p.is_empty()||p.contains(';')) {return Err(error("bim.phases","Native phase labels require nonempty semicolon-free names for IFC Phase projection"));}
    let levels=elevations(&source.storeys,control)?;
    let mut graph=Builder::new(&source.project.name);
    let project=graph.snapshot.entities.iter_mut().find(|e|e.name=="IFCPROJECT").unwrap();
    project.args[3]=s(&source.project.description);project.args[6]=s(source.project.phase_names.join(";"));
    let project=IfcValue::Reference(project.id);
    graph.snapshot.header.file_name[2]=a(vec![s(&source.project.author)]);
    graph.snapshot.header.file_name[3]=a(vec![s(&source.project.organization)]);
    let north=source.sites.values().next().map(|s|s.true_north).unwrap_or(0.);
    if !north.is_finite()||source.sites.values().any(|s|s.true_north!=north) {return Err(error("bim.north","Sites require one finite shared true north"));}
    let direction=graph.add("IFCDIRECTION",vec![a(vec![r(-north.sin()),r(north.cos())])]);
    graph.snapshot.entities.iter_mut().find(|e|e.name=="IFCGEOMETRICREPRESENTATIONCONTEXT").unwrap().args[5]=direction;
    let mut sites=BTreeMap::new();
    let mut buildings=BTreeMap::new();
    for (index,(key,site)) in source.sites.iter().enumerate() {
        checkpoint(control,"bim-sites",index,source.sites.len())?;
        if !site.elevation.is_finite()||site.latitude.abs()>90. {return Err(error("bim.site","Finite site elevation and valid latitude required"));}
        let placement=graph.placement(IfcValue::Unset,[0.;3],0.);
        let entity=graph.rooted("IFCSITE",&site.name,vec![s(key),placement.clone(),IfcValue::Unset,IfcValue::Unset,e("ELEMENT"),compound(site.latitude)?,compound(site.longitude)?,r(site.elevation),IfcValue::Unset,IfcValue::Unset]);
        sites.insert(key.clone(),(entity,placement));
    }
    if !sites.is_empty() {graph.relation(project,sites.values().map(|(entity,_)|entity.clone()).collect());}
    for (index,(key,building)) in source.buildings.iter().enumerate() {
        checkpoint(control,"bim-buildings",index,source.buildings.len())?;
        if [building.origin.x,building.origin.y,building.rotation,building.elevation].iter().any(|v|!v.is_finite()) {return Err(error("bim.building","Finite building frame required"));}
        let (parent,place)=sites.get(&building.site).ok_or_else(||error("bim.site","Building references missing site"))?;
        let placement=graph.placement(place.clone(),[building.origin.x,building.origin.y,building.elevation],building.rotation);
        let entity=graph.rooted("IFCBUILDING",&building.name,vec![s(key),placement.clone(),IfcValue::Unset,IfcValue::Unset,e("ELEMENT"),r(building.elevation),IfcValue::Unset,IfcValue::Unset]);
        graph.relation(parent.clone(),vec![entity.clone()]);
        buildings.insert(key.clone(),(entity,placement));
    }
    for (index,(key,storey)) in source.storeys.iter().enumerate() {
        checkpoint(control,"bim-storeys",index,source.storeys.len())?;
        let (parent,place)=buildings.get(&storey.building).ok_or_else(||error("bim.building","Storey references missing building"))?;
        let level=levels[key.as_str()];
        let placement=graph.placement(place.clone(),[0.,0.,level],0.);
        let entity=graph.rooted("IFCBUILDINGSTOREY",&storey.name,vec![s(key),placement,IfcValue::Unset,IfcValue::Unset,e("ELEMENT"),r(level)]);
        graph.relation(parent.clone(),vec![entity.clone()]);
        let mut values=vec![("Level","IFCINTEGER",IfcValue::Integer(storey.level as i64)),("Height","IFCLENGTHMEASURE",r(storey.height))];
        if let Some(cut)=storey.cut_height {values.push(("CutHeight","IFCLENGTHMEASURE",r(cut)));}
        graph.properties(entity,"semio_BimStorey",values);
    }
    super::projection::validate(&graph.snapshot,control)?;
    checkpoint(control,"bim-complete",source.storeys.len(),source.storeys.len())?;
    Ok(Composition {snapshot:graph.snapshot,confidence:Confidence::High,diagnostics:vec![loss("BIM spatial projection retains project metadata, sites, building frames and authored storey parameters; site boundaries, every other BIM family/collection and original artifact identities are omitted")]})
}

/// 📥 Require authored storey parameters and equivalent admitted spatial frames before creating a BIM artifact.
pub fn ifc_to_bim_controlled(snapshot:&IfcSnapshot,control:&mut dyn FnMut(ExchangeProgress)->bool)->Result<Composition<ModelSnapshot>,ExchangeError> {
    let graph=Reader::new(snapshot,control)?;
    let project=snapshot.entities.iter().find(|e|e.name=="IFCPROJECT").ok_or_else(||error("bim.project","Explicit project required"))?;
    let mut output=ModelSnapshot::default();
    output.project.name=text(project.args.get(2));output.project.description=text(project.args.get(3));
    output.project.author=text(list(snapshot.header.file_name.get(2))?.first());output.project.organization=text(list(snapshot.header.file_name.get(3))?.first());
    let phases=text(project.args.get(6));output.project.phase_names=if phases.is_empty(){vec![]}else{phases.split(';').map(str::to_owned).collect()};
    let mut north=None;
    let contexts=list(project.args.get(7))?;
    if contexts.is_empty(){return Err(error("bim.context","Explicit geometric project context required"));}
    for (index,value) in contexts.iter().enumerate() {
        checkpoint(control,"bim-contexts",index,contexts.len())?;
        let context=graph.reference(Some(value))?;
        let frame=graph.axis(context.args.get(4))?;
        if context.name!="IFCGEOMETRICREPRESENTATIONCONTEXT"||context.args.get(2)!=Some(&IfcValue::Integer(3))||frame.position!=[0.;3]||frame.angle!=0. {return Err(error("bim.context","Identity 3D geometric project context required"));}
        let vector=graph.vector(context.args.get(5),"IFCDIRECTION")?;
        if vector.len()!=2||vector[0].hypot(vector[1])==0. {return Err(error("bim.north","Explicit planar true north required"));}
        let angle=(-vector[0]).atan2(vector[1]);
        if north.is_some_and(|n:f64|(n-angle).abs()>1e-12){return Err(error("bim.north","Project contexts have differing true north"));}north=Some(angle);
    }
    let mut parents=HashMap::new();
    for (index,row) in snapshot.entities.iter().enumerate() {
        if index%128==0 {checkpoint(control,"bim-hierarchy",index,snapshot.entities.len())?;}
        if row.name=="IFCRELNESTS" {return Err(error("bim.hierarchy","Nested decomposition is outside spatial admission"));}
        if row.name!="IFCRELAGGREGATES" {continue;}
        let parent=id(row.args.get(4).ok_or_else(||error("bim.hierarchy","Missing aggregate parent"))?)?;
        let children=list(row.args.get(5))?;
        for (index,child) in children.iter().enumerate() {checkpoint(control,"bim-hierarchy-children",index,children.len())?;if parents.insert(id(child)?,parent).is_some(){return Err(error("bim.hierarchy","Aggregate child has multiple parents"));}}
    }
    let key=|id:u64|format!("ifc{id}");
    let mut spatial_count=0;
    for (index,entity) in snapshot.entities.iter().enumerate() {
        if index%128==0 {checkpoint(control,"bim-import",index,snapshot.entities.len())?;}
        if matches!(entity.name.as_str(),"IFCSITE"|"IFCBUILDING"|"IFCBUILDINGSTOREY"){spatial_count+=1;if spatial_count>64_000{return Err(error("exchange.limit","Native BIM spatial target exceeds entity bound"));}}
        match entity.name.as_str() {
            "IFCSITE"=>{
                let frame=graph.local(entity.args.get(5))?;
                if frame.parent.is_some()||frame.position!=[0.;3]||frame.angle!=0.||parents.get(&entity.id)!=Some(&project.id) {return Err(error("bim.site-placement","Site requires identity absolute placement under project"));}
                let latitude=angle(entity.args.get(9))?;let longitude=angle(entity.args.get(10))?;
                if latitude.abs()>90.||longitude.abs()>180. {return Err(error("bim.angle","Geographic angle out of range"));}
                output.sites.insert(key(entity.id),Site {name:text(entity.args.get(2)),latitude,longitude,elevation:length(number(entity.args.get(11))?,graph.scale)?,true_north:north.unwrap(),boundary:vec![]});
            },
            "IFCBUILDING"=>{
                let parent=graph.entity(*parents.get(&entity.id).ok_or_else(||error("bim.site","Building requires site aggregate parent"))?)?;
                let frame=graph.local(entity.args.get(5))?;
                if parent.name!="IFCSITE"||frame.parent!=parent.args.get(5).and_then(IfcValue::as_reference) {return Err(error("bim.building-placement","Building requires direct site-relative horizontal frame"));}
                output.buildings.insert(key(entity.id),Building {site:key(parent.id),name:text(entity.args.get(2)),origin:Point2 {x:length(frame.position[0],graph.scale)?,y:length(frame.position[1],graph.scale)?},rotation:frame.angle,elevation:length(frame.position[2],graph.scale)?});
            },
            "IFCBUILDINGSTOREY"=>{
                let parent=graph.entity(*parents.get(&entity.id).ok_or_else(||error("bim.building","Storey requires building parent"))?)?;
                let frame=graph.local(entity.args.get(5))?;
                if parent.name!="IFCBUILDING"||frame.parent!=parent.args.get(5).and_then(IfcValue::as_reference)||frame.angle!=0.||frame.position[0]!=0.||frame.position[1]!=0.||(frame.position[2]-number(entity.args.get(9))?).abs()>1e-8 {return Err(error("bim.storey-placement","Storey requires direct unrotated building-relative elevation"));}
                let (_,values)=graph.properties(entity.id,"semio_BimStorey",control)?;
                let level=values.get("Level").ok_or_else(||error("bim.level","Explicit authored level required"))?;
                let Some(IfcValue::TypedValue(level_value))=level.args.get(2) else {return Err(error("bim.level","Typed integer level required"));};
                let [IfcValue::Integer(level)]=level_value.items.as_slice() else {return Err(error("bim.level","Single integer level required"));};
                if level_value.name!="IFCINTEGER"||values["Level"].args.get(3)!=Some(&IfcValue::Unset) {return Err(error("bim.level","Ordinal level cannot carry physical units"));}
                let level=i32::try_from(*level).map_err(|_|error("bim.level","Level exceeds BIM integer range"))?;
                let height=graph.length_property(values.get("Height").ok_or_else(||error("bim.height","Explicit authored height required"))?)?;
                if height<=0. {return Err(error("bim.height","Positive storey height required"));}
                let cut_height=values.get("CutHeight").map(|property|graph.length_property(property)).transpose()?;
                output.storeys.insert(key(entity.id),Storey {building:key(parent.id),name:text(entity.args.get(2)),level,height,cut_height});
            },
            _=>{},
        }
    }
    let levels=elevations(&output.storeys,control)?;
    for entity in snapshot.entities.iter().filter(|e|e.name=="IFCBUILDINGSTOREY") {
        checkpoint(control,"bim-authored-elevation",0,output.storeys.len())?;
        if (levels[key(entity.id).as_str()]-length(number(entity.args.get(9))?,graph.scale)?).abs()>1e-8 {return Err(error("bim.authored-elevation","Storey level/height contradicts elevation"));}
    }
    checkpoint(control,"bim-complete",output.storeys.len(),output.storeys.len())?;
    Ok(Composition {snapshot:output,confidence:Confidence::High,diagnostics:vec![loss("BIM spatial projection retains supported authored parameters only; IFC identities, geometry, properties outside the reserved authored set, families and analytical semantics remain in the source graph")]})
}

#[cfg(test)]
#[path="🧪️tests/native_bim.rs"]
mod tests;
