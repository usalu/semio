//! 🧩 Checked source graph access shared by native spatial and analytical projections.
use super::*;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity,IfcTypedValue};
use std::collections::{BTreeMap,HashMap,HashSet};

pub(super) fn a(values:Vec<IfcValue>)->IfcValue {IfcValue::Aggregate(values)}
pub(super) fn s(value:impl Into<String>)->IfcValue {IfcValue::String(value.into())}
pub(super) fn e(value:&str)->IfcValue {IfcValue::Enum(value.into())}
pub(super) fn r(value:f64)->IfcValue {IfcValue::Real(value)}
pub(super) fn typed(name:&str,value:IfcValue)->IfcValue {IfcValue::TypedValue(IfcTypedValue {name:name.into(),items:vec![value]})}
pub(super) fn id(value:&IfcValue)->Result<u64,ExchangeError> {value.as_reference().ok_or_else(||error("exchange.reference","Explicit entity reference required"))}
pub(super) fn list(value:Option<&IfcValue>)->Result<&[IfcValue],ExchangeError> {value.and_then(IfcValue::as_aggregate).ok_or_else(||error("exchange.aggregate","Explicit aggregate required"))}
pub(super) fn number(value:Option<&IfcValue>)->Result<f64,ExchangeError> {
    let n=match value {Some(IfcValue::Integer(n))=>*n as f64,Some(IfcValue::Real(n))=>*n,_=>return Err(error("exchange.number","Explicit numeric value required"))};
    if n.is_finite() {Ok(n)} else {Err(error("exchange.number","Finite value required"))}
}
pub(super) fn text(value:Option<&IfcValue>)->String {value.and_then(IfcValue::as_str).unwrap_or("").into()}

pub(super) struct Builder {pub snapshot:IfcSnapshot,pub context:IfcValue,guid:u64}
impl Builder {
    pub fn new(name:&str)->Self {
        let (mut snapshot,context)=super::projection::graph();
        snapshot.entities.iter_mut().find(|e|e.name=="IFCPROJECT").unwrap().args[2]=s(name);
        Self {snapshot,context,guid:2}
    }
    pub fn add(&mut self,name:&str,args:Vec<IfcValue>)->IfcValue {
        let id=self.snapshot.entities.len() as u64+1;
        self.snapshot.entities.push(IfcEntity {id,name:name.into(),args,complex:vec![]});
        IfcValue::Reference(id)
    }
    pub fn root(&mut self,name:&str)->Vec<IfcValue> {
        let guid=self.guid;self.guid+=1;
        vec![s(format!("{guid:022}")),IfcValue::Unset,s(name),IfcValue::Unset]
    }
    pub fn rooted(&mut self,kind:&str,name:&str,tail:Vec<IfcValue>)->IfcValue {let mut args=self.root(name);args.extend(tail);self.add(kind,args)}
    pub fn relation(&mut self,parent:IfcValue,children:Vec<IfcValue>)->IfcValue {self.rooted("IFCRELAGGREGATES","",vec![parent,a(children)])}
    pub fn placement(&mut self,parent:IfcValue,point:[f64;3],angle:f64)->IfcValue {
        let point=self.add("IFCCARTESIANPOINT",vec![a(point.into_iter().map(r).collect())]);
        let (z,direction)=if angle==0. {(IfcValue::Unset,IfcValue::Unset)} else {(self.add("IFCDIRECTION",vec![a(vec![r(0.),r(0.),r(1.)])]),self.add("IFCDIRECTION",vec![a(vec![r(angle.cos()),r(angle.sin()),r(0.)])]))};
        let axis=self.add("IFCAXIS2PLACEMENT3D",vec![point,z,direction]);
        self.add("IFCLOCALPLACEMENT",vec![parent,axis])
    }
    pub fn properties(&mut self,target:IfcValue,name:&str,values:Vec<(&str,&str,IfcValue)>)->IfcValue {
        let props=values.into_iter().map(|(key,kind,value)|self.add("IFCPROPERTYSINGLEVALUE",vec![s(key),IfcValue::Unset,typed(kind,value),IfcValue::Unset])).collect();
        let pset=self.rooted("IFCPROPERTYSET",name,vec![a(props)]);
        self.rooted("IFCRELDEFINESBYPROPERTIES","",vec![a(vec![target]),pset.clone()]);
        pset
    }
}

#[derive(Clone,Copy)]
pub(super) struct Frame {pub parent:Option<u64>,pub position:[f64;3],pub angle:f64}
pub(super) struct Reader<'a> {pub entities:HashMap<u64,&'a IfcEntity>,pub scale:f64,pub definitions:HashMap<u64,Vec<&'a IfcEntity>>,pub materials:HashMap<u64,Vec<&'a IfcEntity>>,pub connections:HashMap<u64,Vec<&'a IfcEntity>>,pub material_properties:HashMap<u64,Vec<&'a IfcEntity>>}
impl<'a> Reader<'a> {
    pub fn unit(&self,entity:&IfcEntity,control:&mut dyn FnMut(ExchangeProgress)->bool)->Result<([i32;7],f64),ExchangeError> {
        self.unit_inner(entity,&mut HashSet::new(),&mut 0,control)
    }
    fn unit_inner(&self,entity:&IfcEntity,path:&mut HashSet<u64>,work:&mut usize,control:&mut dyn FnMut(ExchangeProgress)->bool)->Result<([i32;7],f64),ExchangeError> {
        checkpoint(control,"engineering-units",path.len(),64)?;
        *work+=1;if *work>20_000{return Err(error("exchange.limit","Engineering unit evaluation budget exceeded"));}
        if path.len()>=64||!path.insert(entity.id) {return Err(error("exchange.unit","Cyclic or overly deep unit graph"));}
        let result=if entity.name=="IFCSIUNIT" {
            if entity.args.first()!=Some(&IfcValue::Derived) {return Err(error("exchange.unit","SI unit dimensions must be derived"));}
            let (tag,dimensions,base)=match entity.args.get(3).and_then(IfcValue::as_enum) {
                Some("METRE")=>("LENGTHUNIT",[1,0,0,0,0,0,0],1.),Some("SQUARE_METRE")=>("AREAUNIT",[2,0,0,0,0,0,0],1.),
                Some("GRAM")=>("MASSUNIT",[0,1,0,0,0,0,0],1e-3),Some("PASCAL")=>("PRESSUREUNIT",[-1,1,-2,0,0,0,0],1.),
                Some("NEWTON")=>("FORCEUNIT",[1,1,-2,0,0,0,0],1.),_=>return Err(error("exchange.unit","SI engineering unit is outside native admission")),
            };
            if entity.args.get(1).and_then(IfcValue::as_enum)!=Some(tag) {return Err(error("exchange.unit","SI name/type mismatch"));}
            let prefix=match entity.args.get(2) {Some(IfcValue::Unset)=>1.,Some(IfcValue::Enum(p))=>match p.as_str(){"KILO"=>1e3,"MILLI"=>1e-3,"CENTI"=>1e-2,_=>return Err(error("exchange.unit","Unsupported engineering prefix"))},_=>return Err(error("exchange.unit","Invalid engineering prefix"))};
            (dimensions,base*if tag=="AREAUNIT" {prefix*prefix} else {prefix})
        } else if entity.name=="IFCDERIVEDUNIT" {
            let mut dimensions=[0i32;7];let mut scale=1.;
            let elements=list(entity.args.first())?;
            if elements.is_empty() {return Err(error("exchange.unit","Empty derived unit"));}
            for element in elements {
                let element=self.reference(Some(element))?;
                let Some(IfcValue::Integer(power))=element.args.get(1) else{return Err(error("exchange.unit","Integral derived-unit exponent required"));};
                if element.name!="IFCDERIVEDUNITELEMENT"||!(-16..=16).contains(power) {return Err(error("exchange.unit","Unsupported derived unit element"));}
                let (component,factor)=self.unit_inner(self.reference(element.args.first())?,path,work,control)?;
                for axis in 0..7 {dimensions[axis]+=component[axis]*(*power as i32);}
                scale*=factor.powi(*power as i32);
            }
            let expected=match entity.args.get(1).and_then(IfcValue::as_enum) {Some("MASSDENSITYUNIT")=>[-3,1,0,0,0,0,0],Some("MOMENTOFINERTIAUNIT")=>[4,0,0,0,0,0,0],Some("LINEARFORCEUNIT")=>[0,1,-2,0,0,0,0],Some("TORQUEUNIT")=>[2,1,-2,0,0,0,0],_=>return Err(error("exchange.unit","Derived engineering tag outside admission"))};
            if dimensions!=expected||!scale.is_finite()||scale<=0. {return Err(error("exchange.unit","Derived engineering dimensions contradict declaration"));}
            (dimensions,scale)
        } else {return Err(error("exchange.unit","Only explicit SI and supported derived engineering units are admitted"));};
        path.remove(&entity.id);
        Ok(result)
    }
    pub fn new(snapshot:&'a IfcSnapshot,control:&mut dyn FnMut(ExchangeProgress)->bool)->Result<Self,ExchangeError> {
        if snapshot.entities.len()>1_000_000 {return Err(error("exchange.limit","Source entity budget exceeded"));}
        super::projection::validate(snapshot,control)?;
        let scale=super::projection::length_scale(snapshot,control)?;
        let mut output=Self {entities:HashMap::new(),scale,definitions:HashMap::new(),materials:HashMap::new(),connections:HashMap::new(),material_properties:HashMap::new()};
        for (index,entity) in snapshot.entities.iter().enumerate(){checkpoint(control,"graph-entities",index,snapshot.entities.len())?;output.entities.insert(entity.id,entity);}
        let mut work=0;
        for (index,entity) in snapshot.entities.iter().enumerate() {
            if index%128==0 {checkpoint(control,"graph-index",index,snapshot.entities.len())?;}
            let target=match entity.name.as_str(){"IFCRELDEFINESBYPROPERTIES"=>Some(&mut output.definitions),"IFCRELASSOCIATESMATERIAL"=>Some(&mut output.materials),_=>None};
            if let Some(target)=target {for value in list(entity.args.get(4))? {work+=1;if work>2_000_000{return Err(error("exchange.limit","Relationship index budget exceeded"));}checkpoint(control,"graph-relations",work,2_000_000)?;target.entry(id(value)?).or_default().push(entity);}}
            if entity.name=="IFCRELCONNECTSSTRUCTURALMEMBER" {output.connections.entry(id(entity.args.get(4).ok_or_else(||error("exchange.reference","Missing structural member"))?)?).or_default().push(entity);}
            if entity.name=="IFCMATERIALPROPERTIES" {output.material_properties.entry(id(entity.args.get(3).ok_or_else(||error("exchange.reference","Missing material property owner"))?)?).or_default().push(entity);}
        }
        Ok(output)
    }
    pub fn entity(&self,id:u64)->Result<&'a IfcEntity,ExchangeError> {self.entities.get(&id).copied().ok_or_else(||error("exchange.reference","Missing entity"))}
    pub fn reference(&self,value:Option<&IfcValue>)->Result<&'a IfcEntity,ExchangeError> {self.entity(id(value.ok_or_else(||error("exchange.reference","Missing reference"))?)?)}
    pub fn vector(&self,value:Option<&IfcValue>,name:&str)->Result<Vec<f64>,ExchangeError> {
        let entity=self.reference(value)?;
        if entity.name!=name {return Err(error("exchange.vector","Wrong coordinate entity"));}
        let values=list(entity.args.first())?;
        if !(2..=3).contains(&values.len()) {return Err(error("exchange.vector","Two or three coordinates required"));}
        values.iter().map(|v|number(Some(v))).collect()
    }
    pub fn axis(&self,value:Option<&IfcValue>)->Result<Frame,ExchangeError> {
        let axis=self.reference(value)?;
        if axis.name!="IFCAXIS2PLACEMENT3D"||axis.args.len()!=3 {return Err(error("exchange.placement","Exact 3D axis placement required"));}
        if matches!(axis.args.get(1),Some(IfcValue::Unset))!=matches!(axis.args.get(2),Some(IfcValue::Unset)){return Err(error("exchange.placement","3D Axis and RefDirection must be supplied together or both omitted"));}
        let p=self.vector(axis.args.first(),"IFCCARTESIANPOINT")?;
        if p.len()!=3 {return Err(error("exchange.placement","Three-dimensional origin required"));}
        if !matches!(axis.args.get(1),Some(IfcValue::Unset)) {
            let z=self.vector(axis.args.get(1),"IFCDIRECTION")?;
            if z.len()!=3||z[0]!=0.||z[1]!=0.||z[2]<=0. {return Err(error("exchange.placement","Only horizontal right-handed frames are admitted"));}
        }
        let angle=if matches!(axis.args.get(2),Some(IfcValue::Unset)) {0.} else {
            let x=self.vector(axis.args.get(2),"IFCDIRECTION")?;
            if x.len()!=3||x[2]!=0.||x[0].hypot(x[1])==0. {return Err(error("exchange.placement","Finite nonzero planar reference direction required"));}
            x[1].atan2(x[0])
        };
        Ok(Frame {parent:None,position:[p[0],p[1],p[2]],angle})
    }
    pub fn local(&self,value:Option<&IfcValue>)->Result<Frame,ExchangeError> {
        let place=self.reference(value)?;
        if place.name!="IFCLOCALPLACEMENT" {return Err(error("exchange.placement","Direct local placement required"));}
        let mut frame=self.axis(place.args.get(1))?;
        frame.parent=match place.args.first() {Some(IfcValue::Unset)=>None,Some(value)=>Some(id(value)?),None=>return Err(error("exchange.placement","Missing placement parent"))};
        Ok(frame)
    }
    pub fn properties(&self,target:u64,name:&str,control:&mut dyn FnMut(ExchangeProgress)->bool)->Result<(u64,BTreeMap<String,&'a IfcEntity>),ExchangeError> {
        let mut psets=HashSet::new();
        for (index,row) in self.definitions.get(&target).into_iter().flatten().enumerate() {
            checkpoint(control,"properties",index,0)?;
            let pset=self.reference(row.args.get(5))?;
            if pset.name=="IFCPROPERTYSET"&&pset.args.get(2).and_then(IfcValue::as_str)==Some(name) {psets.insert(pset.id);}
        }
        if psets.len()!=1 {return Err(error("exchange.properties","Exactly one authored property set is required"));}
        let pset=psets.into_iter().next().unwrap();
        let mut values=BTreeMap::new();
        for (index,value) in list(self.entity(pset)?.args.get(4))?.iter().enumerate() {
            checkpoint(control,"property-values",index,0)?;
            let property=self.reference(Some(value))?;
            let key=text(property.args.first());
            if property.name!="IFCPROPERTYSINGLEVALUE"||values.insert(key,property).is_some() {return Err(error("exchange.properties","Distinct authored single-value properties required"));}
        }
        Ok((pset,values))
    }
    pub fn length_property(&self,property:&IfcEntity)->Result<f64,ExchangeError> {
        let IfcValue::TypedValue(value)=property.args.get(2).ok_or_else(||error("exchange.property","Missing nominal property"))? else {return Err(error("exchange.property","Typed length measure required"));};
        if !matches!(value.name.as_str(),"IFCLENGTHMEASURE"|"IFCPOSITIVELENGTHMEASURE"|"IFCNONNEGATIVELENGTHMEASURE")||value.items.len()!=1 {return Err(error("exchange.property","Single typed length measure required"));}
        let scale=match property.args.get(3) {
            Some(IfcValue::Unset)=>self.scale,
            Some(unit)=>{
                let unit=self.reference(Some(unit))?;
                if unit.name!="IFCSIUNIT"||unit.args.first()!=Some(&IfcValue::Derived)||unit.args.get(1).and_then(IfcValue::as_enum)!=Some("LENGTHUNIT")||unit.args.get(3).and_then(IfcValue::as_enum)!=Some("METRE") {return Err(error("exchange.unit","Explicit property requires a supported SI length unit"));}
                match unit.args.get(2) {Some(IfcValue::Unset)=>1.,Some(IfcValue::Enum(p))=>match p.as_str(){"MILLI"=>1e-3,"CENTI"=>1e-2,"DECI"=>1e-1,"KILO"=>1e3,_=>return Err(error("exchange.unit","Explicit length prefix is outside this admitted subset"))},_=>return Err(error("exchange.unit","Invalid explicit length prefix"))}
            },
            None=>return Err(error("exchange.unit","Explicit property Unit attribute required")),
        };
        let result=number(value.items.first())?*scale;
        if result.is_finite() {Ok(result)} else {Err(error("exchange.unit","Length exceeds finite SI range"))}
    }
}
