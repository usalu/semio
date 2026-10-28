//! 📐 Owned geometry conversion over the shared IFC projector and native glTF accessor reader.
use super::*;
use semio_s_artifact_stdio_ifc_4_model::standards::v4::subsets::any::schema::snapshot::{IfcEntity, IfcHeader};
use std::collections::{HashMap, HashSet};

const VERTICES: usize = 200_000;
const TRIANGLES: usize = 400_000;
const NODES: usize = 64_000;
const BYTES: usize = 64 * 1024 * 1024;
type Matrix = [[f64; 4]; 4];
const IDENTITY: Matrix = [[1.,0.,0.,0.],[0.,1.,0.,0.],[0.,0.,1.,0.],[0.,0.,0.,1.]];

pub(super) fn length_scale(snapshot: &IfcSnapshot, control: &mut dyn FnMut(ExchangeProgress) -> bool) -> Result<f64, ExchangeError> {
    let mut entities = HashMap::new();
    let mut projects = Vec::new();
    for (i, entity) in snapshot.entities.iter().enumerate() {
        if i % 256 == 0 { checkpoint(control, "units", i, snapshot.entities.len())?; }
        entities.insert(entity.id, entity);
        if !entity.complex.is_empty() {return Err(error("exchange.unsupported-geometry","Complex entity geometry requires component-aware evaluation"));}
        if entity.name == "IFCPROJECT" { projects.push(entity); }
        if matches!(entity.name.as_str(), "IFCMAPCONVERSION" | "IFCMAPCONVERSIONSCALED" | "IFCRIGIDOPERATION" | "IFCPROJECTEDCRS" | "IFCGEOGRAPHICCRS") {
            return Err(error("exchange.georeference", "Coordinate reference operations require an explicit evaluated transform"));
        }
    }
    let [project] = projects.as_slice() else { return Err(error("exchange.units", "Exactly one project is required to resolve length units")); };
    let assignment = project.args.get(8).and_then(IfcValue::as_reference).and_then(|id| entities.get(&id)).filter(|e| e.name == "IFCUNITASSIGNMENT").ok_or_else(|| error("exchange.units", "Missing project unit assignment"))?;
    let units = assignment.args.first().and_then(IfcValue::as_aggregate).ok_or_else(|| error("exchange.units", "Invalid unit assignment"))?;
    let mut scale = None;
    for (index,unit) in units.iter().enumerate() {
        checkpoint(control,"unit-assignment",index,units.len())?;
        let unit = unit.as_reference().and_then(|id| entities.get(&id)).ok_or_else(|| error("exchange.units", "Missing assigned unit"))?;
        if unit.args.get(1).and_then(IfcValue::as_enum) != Some("LENGTHUNIT") { continue; }
        if scale.is_some() || unit.name != "IFCSIUNIT" || unit.args.first()!=Some(&IfcValue::Derived) || unit.args.get(3).and_then(IfcValue::as_enum) != Some("METRE") {
            return Err(error("exchange.units", "Exactly one SI metre length unit is supported; conversion-based units require evaluation"));
        }
        scale = Some(match unit.args.get(2) {
            Some(IfcValue::Unset) => 1.,
            Some(IfcValue::Enum(prefix)) => match prefix.as_str() {
                "EXA"=>1e18,"PETA"=>1e15,"TERA"=>1e12,"GIGA"=>1e9,"MEGA"=>1e6,"KILO"=>1e3,"HECTO"=>1e2,"DECA"=>1e1,
                "DECI"=>1e-1,"CENTI"=>1e-2,"MILLI"=>1e-3,"MICRO"=>1e-6,"NANO"=>1e-9,"PICO"=>1e-12,"FEMTO"=>1e-15,"ATTO"=>1e-18,
                _=>return Err(error("exchange.units", "Unsupported SI prefix")),
            },
            _=>return Err(error("exchange.units", "Invalid SI prefix")),
        });
    }
    scale.ok_or_else(|| error("exchange.units", "Project has no length unit"))
}

pub(super) fn validate(snapshot:&IfcSnapshot,control:&mut dyn FnMut(ExchangeProgress)->bool)->Result<(),ExchangeError> {
    let mut limited=false;
    let result=crate::io::validate_ifc4x3_graph_controlled(snapshot,&mut |p| {
        if p.phase=="values"&&p.completed>4_000_000 {limited=true;return false;}
        control(ExchangeProgress {phase:"validate",completed:p.completed,total:p.total})
    });
    if limited {return Err(error("exchange.limit","IFC graph value budget exceeded"));}
    result.map_err(|e|error(if e.code=="io.cancelled" {"exchange.cancelled"} else {e.code},e.message))
}

fn accessor(view: usize, count: usize, kind: GltfAccessorType, component_type: GltfComponentType) -> GltfAccessor {
    GltfAccessor { buffer_view: Some(view), byte_offset: 0, component_type, normalized: false, count, kind, min: None, max: None, sparse: None, name: None, extensions: None, extras: None }
}
fn view(offset: usize, length: usize, target: u64) -> GltfBufferView {
    GltfBufferView { buffer: 0, byte_offset: offset, byte_length: length, byte_stride: None, target: Some(target), name: None, extensions: None, extras: None }
}

/// 📤 Project supported IFC geometry to metres/Y-up; issues, limits and cancellation refuse atomically.
pub fn ifc_to_gltf_controlled(snapshot: &IfcSnapshot, control: &mut dyn FnMut(ExchangeProgress) -> bool) -> Result<Composition<GltfSnapshot>, ExchangeError> {
    checkpoint(control, "begin", 0, snapshot.entities.len())?;
    if snapshot.entities.len()>1_000_000 { return Err(error("exchange.limit","Entity graph exceeds budget")); }
    if snapshot.schema != "stdio.ifc" || snapshot.header.file_schema != vec![IfcValue::Aggregate(vec![IfcValue::String("IFC4X3_ADD2".into())])] {
        return Err(error("exchange.schema", "IFC4X3_ADD2 graph required"));
    }
    let scale = length_scale(snapshot, control)?;
    validate(snapshot,control)?;
    let scene = geometry::project_controlled(snapshot, geometry::GeometryLimits::default(), &mut |p| if control(ExchangeProgress { phase: "geometry", completed: p.work, total: p.budget }) { geometry::GeometryControl::Continue } else { geometry::GeometryControl::Cancel });
    match scene.status {
        geometry::GeometryStatus::Cancelled => return Err(error("exchange.cancelled", "Geometry exchange cancelled")),
        geometry::GeometryStatus::Limited => return Err(error("exchange.limit", "Geometry exceeds projection budget")),
        geometry::GeometryStatus::Complete => {},
    }
    if !scene.issues.is_empty() { return Err(error("exchange.unsupported-geometry", scene.issues.iter().map(|i| format!("#{}: {}", i.entity, i.reason)).collect::<Vec<_>>().join("; "))); }
    if scene.meshes.is_empty() { return Err(error("exchange.empty", "No supported product geometry")); }
    let mut output = GltfSnapshot::default();
    output.source_form = GltfSourceForm::Glb;
    output.document.scene = Some(0);
    output.document.scenes.push(GltfScene::default());
    let mut bytes = Vec::new();
    for (index, mesh) in scene.meshes.iter().enumerate() {
        checkpoint(control, "gltf", index, scene.meshes.len())?;
        if mesh.vertices.is_empty()||mesh.triangles.is_empty() { return Err(error("exchange.empty","Projected mesh has no triangles")); }
        let start = bytes.len();
        let mut min = vec![f64::INFINITY;3];
        let mut max = vec![f64::NEG_INFINITY;3];
        for (i, point) in mesh.vertices.iter().enumerate() {
            if i % 1024 == 0 { checkpoint(control, "vertices", i, mesh.vertices.len())?; }
            for (axis, value) in [point[0]*scale, point[2]*scale, -point[1]*scale].into_iter().enumerate() {
                let value = value as f32;
                if !value.is_finite() { return Err(error("exchange.range", "Position exceeds finite float32 range")); }
                bytes.extend_from_slice(&value.to_le_bytes());
                min[axis] = min[axis].min(value as f64);
                max[axis] = max[axis].max(value as f64);
            }
        }
        let indices = bytes.len();
        for (i, triangle) in mesh.triangles.iter().enumerate() {
            if i % 1024 == 0 { checkpoint(control, "triangles", i, mesh.triangles.len())?; }
            for &vertex in triangle {
                if vertex as usize >= mesh.vertices.len() { return Err(error("exchange.index", "Projected triangle index exceeds positions")); }
                bytes.extend_from_slice(&vertex.to_le_bytes());
            }
        }
        if bytes.len() > BYTES { return Err(error("exchange.limit", "Embedded geometry buffer exceeds budget")); }
        let v = output.document.buffer_views.len();
        let a = output.document.accessors.len();
        output.document.buffer_views.extend([view(start, indices-start,34962),view(indices,bytes.len()-indices,34963)]);
        let mut positions = accessor(v,mesh.vertices.len(),GltfAccessorType::Vec3,GltfComponentType::Float);
        positions.min=Some(min); positions.max=Some(max);
        output.document.accessors.extend([positions,accessor(v+1,mesh.triangles.len()*3,GltfAccessorType::Scalar,GltfComponentType::UnsignedInt)]);
        output.document.meshes.push(GltfMesh { name: Some(mesh.label.clone()), primitives: vec![GltfPrimitive { attributes: vec![("POSITION".into(),a)], indices: Some(a+1), mode: Some(4), ..Default::default() }], ..Default::default() });
        output.document.nodes.push(GltfNode { mesh: Some(index), name: Some(mesh.label.clone()), extras: Some(GltfJson::Object(vec![("ifcProductId".into(),GltfJson::String(mesh.product.to_string())),("ifcRepresentationId".into(),GltfJson::String(mesh.representation.to_string()))])), ..Default::default() });
        output.document.scenes[0].nodes.push(index);
    }
    output.document.buffers.push(GltfBuffer { byte_length: bytes.len(), uri: None, name: None, extensions: None, extras: None });
    output.buffers.push(bytes);
    checkpoint(control, "complete", scene.meshes.len(), scene.meshes.len())?;
    Ok(Composition { snapshot: output, confidence: Confidence::High, diagnostics: vec![loss("Geometry projection preserves represented product triangles and identities; IFC properties, relationships, materials, geographic metadata, authored parameters and analysis semantics are omitted; positions use float32")] })
}

fn multiply(a: Matrix,b: Matrix)->Matrix {
    let mut output=[[0.;4];4];
    for i in 0..4 { for j in 0..4 { for k in 0..4 { output[i][j]+=a[i][k]*b[k][j]; } } }
    output
}
fn matrix(node: &GltfNode)->Result<Matrix,ExchangeError> {
    let result=if let Some(m)=node.matrix {
        if node.translation.is_some()||node.rotation.is_some()||node.scale.is_some() { return Err(error("exchange.transform","Matrix and TRS are exclusive")); }
        std::array::from_fn(|r|std::array::from_fn(|c|m[c*4+r]))
    } else {
        let [x,y,z,w]=node.rotation.unwrap_or([0.,0.,0.,1.]);
        if ((x*x+y*y+z*z+w*w).sqrt()-1.).abs()>1e-5 { return Err(error("exchange.transform","Unit quaternion required")); }
        let [a,b,c]=node.scale.unwrap_or([1.;3]);
        let [u,v,t]=node.translation.unwrap_or([0.;3]);
        [[(1.-2.*(y*y+z*z))*a,2.*(x*y-z*w)*b,2.*(x*z+y*w)*c,u],[2.*(x*y+z*w)*a,(1.-2.*(x*x+z*z))*b,2.*(y*z-x*w)*c,v],[2.*(x*z-y*w)*a,2.*(y*z+x*w)*b,(1.-2.*(x*x+y*y))*c,t],[0.,0.,0.,1.]]
    };
    if result.iter().flatten().any(|v|!v.is_finite()) || result[3]!=[0.,0.,0.,1.] { return Err(error("exchange.transform","Finite affine transform required")); }
    Ok(result)
}
fn orientation(matrix:Matrix)->Result<i8,ExchangeError> {
    if matrix.iter().flatten().any(|value|!value.is_finite()) {return Err(error("exchange.transform","Transform composition is not finite"));}
    Ok(finite_orientation(matrix))
}
pub(super) fn finite_orientation(matrix:Matrix)->i8 {
    let mut positive=[0u64;100];
    let mut negative=[0u64;100];
    for (columns,subtract) in [([0,1,2],false),([1,2,0],false),([2,0,1],false),([2,1,0],true),([1,0,2],true),([0,2,1],true)] {
        let mut product=[1u64,0,0];
        let mut exponent=3222i32;
        let mut minus=subtract;
        for row in 0..3 {
            let bits=matrix[row][columns[row]].to_bits();
            minus^=bits>>63!=0;
            let encoded=((bits>>52)&0x7ff) as i32;
            let mantissa=(bits&0x000f_ffff_ffff_ffff)|if encoded==0 {0} else {1u64<<52};
            exponent+=if encoded==0 {-1074} else {encoded-1075};
            let mut carry=0u128;
            for limb in &mut product {
                let multiplied=(*limb as u128)*(mantissa as u128)+carry;
                *limb=multiplied as u64;
                carry=multiplied>>64;
            }
        }
        let mut term=[0u64;100];
        let word=exponent as usize/64;
        let shift=exponent as usize%64;
        for (index,limb) in product.into_iter().enumerate() {
            term[word+index]|=limb<<shift;
            if shift!=0 {term[word+index+1]|=limb>>(64-shift);}
        }
        let accumulator=if minus {&mut negative} else {&mut positive};
        let mut carry=0u128;
        for index in 0..100 {
            let sum=accumulator[index] as u128+term[index] as u128+carry;
            accumulator[index]=sum as u64;
            carry=sum>>64;
        }
    }
    match positive.iter().rev().cmp(negative.iter().rev()) {std::cmp::Ordering::Less=>-1,std::cmp::Ordering::Equal=>0,std::cmp::Ordering::Greater=>1}
}

#[cfg(test)]
#[path="🧪️tests/native_orientation.rs"]
mod orientation_tests;

#[cfg(test)]
#[path="🧪️tests/native_accessor.rs"]
mod accessor_tests;

#[cfg(test)]
#[path="🧪️tests/native_gltf_samples.rs"]
mod sample_tests;
fn read_accessor(source:&GltfSnapshot,index:usize,kind:GltfAccessorType,control:&mut dyn FnMut(ExchangeProgress)->bool)->Result<Vec<f64>,ExchangeError> {
    let a=source.document.accessors.get(index).ok_or_else(||error("exchange.accessor","Missing accessor"))?;
    let allowed=if kind==GltfAccessorType::Vec3 { a.component_type==GltfComponentType::Float } else { matches!(a.component_type,GltfComponentType::UnsignedByte|GltfComponentType::UnsignedShort|GltfComponentType::UnsignedInt) };
    if a.kind!=kind||!allowed||a.sparse.is_some()||a.extensions.is_some()||a.normalized||a.buffer_view.is_none()||a.count>if kind==GltfAccessorType::Vec3 {VERTICES} else {TRIANGLES*3} { return Err(error("exchange.accessor","Bounded dense float32 positions and unsigned integer indices required")); }
    let view=source.document.buffer_views.get(a.buffer_view.unwrap()).ok_or_else(||error("exchange.accessor","Missing buffer view"))?;
    let bytes=source.buffers.get(view.buffer).ok_or_else(||error("exchange.accessor","Missing buffer"))?;
    if view.extensions.is_some() { return Err(error("exchange.accessor","Buffer view extensions require evaluation")); }
    let declared=source.document.buffers.get(view.buffer).ok_or_else(||error("exchange.accessor","Missing declared buffer"))?;
    let width=if kind==GltfAccessorType::Vec3 {12} else {match a.component_type {GltfComponentType::UnsignedByte=>1,GltfComponentType::UnsignedShort=>2,_=>4}};
    let component=if kind==GltfAccessorType::Vec3 {4} else {width};
    let stride=view.byte_stride.unwrap_or(width);
    let extent=if a.count==0 {Some(0)} else {(a.count-1).checked_mul(stride).and_then(|n|n.checked_add(width))};
    if a.count==0||a.byte_offset%component!=0||view.byte_offset%component!=0||stride<width||view.target.is_some_and(|target|target!=if kind==GltfAccessorType::Vec3 {34962} else {34963})||(kind==GltfAccessorType::Vec3&&view.byte_stride.is_some_and(|stride|stride%4!=0||stride>252))||(kind==GltfAccessorType::Scalar&&view.byte_stride.is_some())||a.byte_offset.checked_add(extent.ok_or_else(||error("exchange.accessor","Accessor extent overflows"))?).is_none_or(|n|n>view.byte_length)||view.byte_offset.checked_add(view.byte_length).is_none_or(|n|n>bytes.len()||n>declared.byte_length) { return Err(error("exchange.accessor","Accessor alignment, stride or extent is invalid")); }
    let components=if kind==GltfAccessorType::Vec3 {3} else {1};
    let mut values=Vec::with_capacity(a.count*components);
    for row in 0..a.count {
        if row%256==0 {checkpoint(control,"gltf-accessor",row,a.count)?;}
        let start=view.byte_offset+a.byte_offset+row*stride;
        for column in 0..components {
            let at=start+column*component;
            let value=match a.component_type {
                GltfComponentType::Float=>f32::from_le_bytes(bytes[at..at+4].try_into().unwrap()) as f64,
                GltfComponentType::UnsignedInt=>u32::from_le_bytes(bytes[at..at+4].try_into().unwrap()) as f64,
                GltfComponentType::UnsignedShort=>u16::from_le_bytes(bytes[at..at+2].try_into().unwrap()) as f64,
                GltfComponentType::UnsignedByte=>bytes[at] as f64,
                _=>return Err(error("exchange.accessor","Unsupported dense accessor component")),
            };
            if !value.is_finite() {return Err(error("exchange.accessor","Non-finite accessor"));}
            values.push(value);
        }
    }
    checkpoint(control,"gltf-accessor",a.count,a.count)?;
    Ok(values)
}

fn validate_nodes(document:&GltfDocument,control:&mut dyn FnMut(ExchangeProgress)->bool)->Result<(),ExchangeError> {
    let mut parents=vec![0u8;document.nodes.len()];
    for (index,node) in document.nodes.iter().enumerate() {
        checkpoint(control,"node-graph",index,document.nodes.len())?;
        for (edge,&child) in node.children.iter().enumerate() {
            if edge%256==0 {checkpoint(control,"node-edges",edge,node.children.len())?;}
            let parent=parents.get_mut(child).ok_or_else(||error("exchange.node","Missing child node"))?;
            if *parent!=0 {return Err(error("exchange.node-parent","glTF nodes must have one parent at most"));}
            *parent=1;
        }
    }
    let mut state=vec![0u8;document.nodes.len()];
    for root in 0..document.nodes.len() {
        if state[root]!=0 {continue;}
        let mut stack=vec![(root,false,0usize)];
        while let Some((index,exit,depth))=stack.pop() {
            checkpoint(control,"node-topology",index,document.nodes.len())?;
            if exit {state[index]=2;continue;}
            if state[index]==1 {return Err(error("exchange.cycle","Cyclic node graph"));}
            if state[index]==2 {continue;}
            if depth>=64 {return Err(error("exchange.limit","Node depth exceeds budget"));}
            state[index]=1;
            stack.push((index,true,depth));
            for &child in document.nodes[index].children.iter().rev() {stack.push((child,false,depth+1));}
        }
    }
    for scene in &document.scenes {
        let mut seen=HashSet::new();
        for (index,&root) in scene.nodes.iter().enumerate() {
            checkpoint(control,"scene-roots",index,scene.nodes.len())?;
            if parents.get(root)!=Some(&0)||!seen.insert(root) {return Err(error("exchange.scene","Scene roots must exist, be unique and have no parent"));}
        }
    }
    Ok(())
}

fn aggregate(values:Vec<IfcValue>)->IfcValue { IfcValue::Aggregate(values) }
fn text(value:impl Into<String>)->IfcValue { IfcValue::String(value.into()) }
fn enumeration(value:&str)->IfcValue { IfcValue::Enum(value.into()) }
fn add(snapshot:&mut IfcSnapshot,name:&str,args:Vec<IfcValue>)->IfcValue {
    let id=snapshot.entities.len() as u64+1;
    snapshot.entities.push(IfcEntity {id,name:name.into(),args,complex:vec![]});
    IfcValue::Reference(id)
}
pub(super) fn graph()->(IfcSnapshot,IfcValue) {
    let mut snapshot=IfcSnapshot {schema:"stdio.ifc".into(),header:IfcHeader::default(),entities:vec![]};
    snapshot.header.file_schema=vec![aggregate(vec![text("IFC4X3_ADD2")])];
    snapshot.header.file_description=vec![aggregate(vec![text("ViewDefinition [notYetAssigned]")]),text("2;1")];
    let origin=add(&mut snapshot,"IFCCARTESIANPOINT",vec![aggregate(vec![IfcValue::Real(0.);3])]);
    let axis=add(&mut snapshot,"IFCAXIS2PLACEMENT3D",vec![origin,IfcValue::Unset,IfcValue::Unset]);
    let context=add(&mut snapshot,"IFCGEOMETRICREPRESENTATIONCONTEXT",vec![IfcValue::Unset,text("Model"),IfcValue::Integer(3),IfcValue::Real(1e-6),axis,IfcValue::Unset]);
    let unit=add(&mut snapshot,"IFCSIUNIT",vec![IfcValue::Derived,enumeration("LENGTHUNIT"),IfcValue::Unset,enumeration("METRE")]);
    let units=add(&mut snapshot,"IFCUNITASSIGNMENT",vec![aggregate(vec![unit])]);
    add(&mut snapshot,"IFCPROJECT",vec![text("0000000000000000000001"),IfcValue::Unset,text("glTF geometry"),IfcValue::Unset,IfcValue::Unset,IfcValue::Unset,IfcValue::Unset,aggregate(vec![context.clone()]),units]);
    (snapshot,context)
}

struct Importer<'a,'b> {
    source:&'a GltfSnapshot,
    control:&'b mut dyn FnMut(ExchangeProgress)->bool,
    output:IfcSnapshot,
    context:IfcValue,
    active:HashSet<usize>,
    nodes:usize,
    vertices:usize,
    triangles:usize,
    primitives:usize,
}
impl Importer<'_,'_> {
    fn visit(&mut self,index:usize,parent:Matrix)->Result<(),ExchangeError> {
        checkpoint(self.control,"nodes",self.nodes,self.source.document.nodes.len())?;
        self.nodes+=1;
        if self.nodes>NODES||self.active.len()>=64 { return Err(error("exchange.limit","Node graph exceeds depth/work budget")); }
        if !self.active.insert(index) { return Err(error("exchange.cycle","Cyclic node hierarchy")); }
        let source=self.source;
        let node=source.document.nodes.get(index).ok_or_else(||error("exchange.node","Missing node"))?;
        if node.skin.is_some()||!node.weights.is_empty()||node.extensions.is_some() { return Err(error("exchange.deformation","Skinned/morphed or extended nodes require evaluation")); }
        let transform=multiply(parent,matrix(node)?);
        let orientation=orientation(transform)?;
        if orientation==0 { return Err(error("exchange.transform","Singular transform")); }
        if let Some(mesh)=node.mesh {
            let mesh=source.document.meshes.get(mesh).ok_or_else(||error("exchange.mesh","Missing mesh"))?;
            if !mesh.weights.is_empty()||mesh.extensions.is_some() { return Err(error("exchange.deformation","Morph weights and mesh extensions require evaluation")); }
            for primitive in &mesh.primitives {
                checkpoint(self.control,"primitives",self.primitives,0)?;
                if primitive.mode.unwrap_or(4)!=4||!primitive.targets.is_empty()||primitive.extensions.is_some() { return Err(error("exchange.topology","Only static triangle primitives without extensions are supported")); }
                let position=primitive.attributes.iter().find(|(key,_)|key=="POSITION").map(|(_,index)|*index).ok_or_else(||error("exchange.position","Missing POSITION"))?;
                let positions=read_accessor(self.source,position,GltfAccessorType::Vec3,self.control)?;
                let count=positions.len()/3;
                self.vertices+=count;
                if count==0||self.vertices>VERTICES { return Err(error("exchange.limit","Position budget exceeded")); }
                let indices=match primitive.indices {Some(index)=>read_accessor(self.source,index,GltfAccessorType::Scalar,self.control)?,None=>(0..count).map(|i|i as f64).collect()};
                self.triangles+=indices.len()/3;
                if indices.is_empty()||indices.len()%3!=0||self.triangles>TRIANGLES { return Err(error("exchange.topology","Invalid triangle count or triangle budget exceeded")); }
                let mut points=Vec::with_capacity(count);
                for (i,p) in positions.chunks_exact(3).enumerate() {
                    if i%1024==0 { checkpoint(self.control,"vertices",i,count)?; }
                    let q: [f64;3]=std::array::from_fn(|r|transform[r][0]*p[0]+transform[r][1]*p[1]+transform[r][2]*p[2]+transform[r][3]);
                    if q.iter().any(|v|!v.is_finite()) { return Err(error("exchange.range","Transformed position is not finite")); }
                    points.push(aggregate(vec![IfcValue::Real(q[0]),IfcValue::Real(-q[2]),IfcValue::Real(q[1])]));
                }
                let mut faces=Vec::with_capacity(indices.len()/3);
                for (i,triangle) in indices.chunks_exact(3).enumerate() {
                    if i%1024==0 { checkpoint(self.control,"triangles",i,indices.len()/3)?; }
                    if triangle.iter().any(|v|*v<0.||*v>=count as f64||v.fract()!=0.) { return Err(error("exchange.index","Triangle index exceeds positions")); }
                    let mut triangle=[triangle[0] as i64+1,triangle[1] as i64+1,triangle[2] as i64+1];
                    if orientation<0 { triangle.swap(1,2); }
                    faces.push(aggregate(triangle.into_iter().map(IfcValue::Integer).collect()));
                }
                let points=add(&mut self.output,"IFCCARTESIANPOINTLIST3D",vec![aggregate(points),IfcValue::Unset]);
                let shape=add(&mut self.output,"IFCTRIANGULATEDFACESET",vec![points,IfcValue::Unset,enumeration("F"),aggregate(faces),IfcValue::Unset]);
                let shape=add(&mut self.output,"IFCSHAPEREPRESENTATION",vec![self.context.clone(),text("Body"),text("Tessellation"),aggregate(vec![shape])]);
                let shape=add(&mut self.output,"IFCPRODUCTDEFINITIONSHAPE",vec![IfcValue::Unset,IfcValue::Unset,aggregate(vec![shape])]);
                self.primitives+=1;
                let name=node.name.clone().or_else(||mesh.name.clone()).unwrap_or_else(||format!("glTF primitive {}",self.primitives));
                add(&mut self.output,"IFCBUILDINGELEMENTPROXY",vec![text(format!("{:022}",self.primitives+1)),IfcValue::Unset,text(name),IfcValue::Unset,IfcValue::Unset,IfcValue::Unset,shape,IfcValue::Unset,enumeration("NOTDEFINED")]);
            }
        }
        for &child in &node.children { self.visit(child,transform)?; }
        self.active.remove(&index);
        Ok(())
    }
}

/// 📥 Static glTF triangles become real IFC ADD2 tessellated proxies in SI metres/Z-up.
pub fn gltf_to_ifc_controlled(source:&GltfSnapshot,control:&mut dyn FnMut(ExchangeProgress)->bool)->Result<Composition<IfcSnapshot>,ExchangeError> {
    checkpoint(control,"begin",0,source.document.nodes.len())?;
    let d=&source.document;
    if source.schema!="stdio.gltf"||d.asset.version!="2.0"||!d.extensions_required.is_empty()||!d.extensions_used.is_empty()||d.extensions.is_some()||!d.animations.is_empty()||!d.skins.is_empty()||d.scenes.iter().any(|s|s.extensions.is_some()) { return Err(error("exchange.unsupported","Static glTF 2.0 without extensions, animation or skinning is required")); }
    if d.nodes.len()>NODES||source.buffers.iter().try_fold(0usize,|n,b|n.checked_add(b.len())).is_none_or(|n|n>BYTES) { return Err(error("exchange.limit","Source exceeds node/buffer bounds")); }
    gltf_io::validate_document(d).map_err(|e|error("exchange.gltf",e))?;
    validate_nodes(d,control)?;
    let roots=if let Some(scene)=d.scene { d.scenes.get(scene).ok_or_else(||error("exchange.scene","Missing default scene"))?.nodes.clone() } else if d.scenes.len()==1 { d.scenes[0].nodes.clone() } else if d.scenes.is_empty() {
        let children:HashSet<usize>=d.nodes.iter().flat_map(|n|n.children.iter().copied()).collect();
        (0..d.nodes.len()).filter(|i|!children.contains(i)).collect()
    } else { return Err(error("exchange.scene","Ambiguous scene selection")); };
    let (output,context)=graph();
    let mut importer=Importer {source,control,output,context,active:HashSet::new(),nodes:0,vertices:0,triangles:0,primitives:0};
    for root in roots { importer.visit(root,IDENTITY)?; }
    if importer.primitives==0 { return Err(error("exchange.empty","No static triangle primitives")); }
    validate(&importer.output,importer.control)?;
    checkpoint(importer.control,"complete",importer.primitives,importer.primitives)?;
    Ok(Composition {snapshot:importer.output,confidence:Confidence::High,diagnostics:vec![loss("Static geometry is baked in metres/Z-up into new IFC proxies; glTF hierarchy, source identities, materials, normals, cameras and extras are omitted; no authored BIM or analysis semantics are inferred")]})
}
