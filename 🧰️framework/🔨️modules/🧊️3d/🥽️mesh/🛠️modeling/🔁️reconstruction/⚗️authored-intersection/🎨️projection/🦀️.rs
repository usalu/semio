//! 🎨️ Transfers complete authored rings or projects clipped patches through their supporting polygons.

use super::*;
use semio_framework_3d_mesh_contract::MeshSourceVertex;
use semio_framework_mesh_engine::{PolygonSurfaceKind as Kind,PolygonSurfaceSample as Sample};
use semio_framework_value::DslValue;

type Point=[f64;3];

fn difference(a:Point,b:Point)->Point{std::array::from_fn(|i|a[i]-b[i])}
fn dot(a:Point,b:Point)->f64{(0..3).map(|i|a[i]*b[i]).sum()}
fn cross([a,b,c]:Point,[d,e,f]:Point)->Point{[b*f-c*e,c*d-a*f,a*e-b*d]}

#[derive(Clone,Copy,semio_framework_value::RetireOwned)]
struct Chart {source:usize,start:usize,basis:[usize;3],origin:Point,u:Point,v:Point,normal:Point,gram:[f64;3],epsilon:f64,uncut:bool,needs_affine:bool,profile_cap:bool,reversed:bool,exact_ring_start:Option<usize>,exact_ring_checked:bool}

impl Chart {
    fn new(source:&PolygonMeshSource,face:usize,start:usize,index:usize)->Option<Self>{
        let ring=&source.faces[face];
        let [origin,p,q]=[0,1,index].map(|i|source.vertices[ring[i] as usize].map(f64::from));
        let (u,v)=(difference(p,origin),difference(q,origin));
        let cross=cross(u,v);let length=dot(cross,cross).sqrt();
        let (uu,vv,uv)=(dot(u,u),dot(v,v),dot(u,v));
        let epsilon=uu.sqrt().max(vv.sqrt())*1e-6;
        if length<=epsilon*epsilon{return None;}
        Some(Self{source:face,start,basis:[0,1,index],origin,u,v,normal:cross.map(|n|n/length),
            gram:[uu,vv,uv],epsilon,uncut:true,needs_affine:false,profile_cap:false,reversed:false,exact_ring_start:None,exact_ring_checked:false})
    }

    fn ring_corner(&self,start:usize,step:usize,len:usize)->usize{
        if self.reversed{(start+len-step)%len}else{(start+step)%len}
    }

    fn weights(&self,p:Point)->[f64;3]{
        let [uu,vv,uv]=self.gram;let det=uu*vv-uv*uv;
        let d=difference(p,self.origin);let (u,v)=(dot(d,self.u),dot(d,self.v));
        let (a,b)=((vv*u-uv*v)/det,(uu*v-uv*u)/det);
        [1.0-a-b,a,b]
    }

    fn face_blend(&self,p:Point)->Blend{Blend{corners:self.basis,weights:self.weights(p)}}

    fn basis_point(&self,source:&PolygonMeshSource,i:usize)->Point{
        source.vertices[source.faces[self.source][i] as usize].map(f64::from)
    }

    fn edge_parameter(&self,a:Point,b:Point,q:Point)->(f64,Option<f64>){
        let edge=difference(b,a);let delta=difference(q,a);let length=dot(edge,edge).sqrt();
        let side=dot(cross(edge,delta),self.normal)/length;
        let t=dot(edge,delta)/length;
        let parameter=(side.abs()<=self.epsilon&&t>=-self.epsilon&&t<=length+self.epsilon).then(||(t/length).clamp(0.,1.));
        (side,parameter)
    }

    fn index(&self,source:&PolygonMeshSource,domain:MeshAttributeDomain,corner:usize)->usize{
        match domain{
            MeshAttributeDomain::Vertex=>source.faces[self.source][corner] as usize,
            MeshAttributeDomain::Face=>self.source,
            _=>self.start+corner,
        }
    }
}

#[derive(Clone,Copy,semio_framework_value::RetireOwned)]
struct Blend {corners:[usize;3],weights:[f64;3]}

#[derive(Clone,Copy,semio_framework_value::RetireOwned)]
struct Corner {face:usize,vertex:u32,blend:Blend,exact:Option<usize>,edge:Option<usize>}

#[derive(Default,semio_framework_value::RetireOwned)]
struct Surface {
    uv:HistoryFoldIndex<usize,[f32;2]>,normals:HistoryFoldIndex<u32,[f32;3]>,
    smooth:HistoryFoldSet<usize>,seams:HistoryFoldSet<usize>,
    mapped_normals:HistoryFoldIndex<u32,[f32;3]>,conflicting:bool,
}

impl Surface{
    fn add(&mut self,sample:Sample){
        match sample.kind {
            Kind::CornerUv=>{if let Some(uv)=sample.uv{self.uv.insert(sample.index as usize,uv);}},
            Kind::VertexNormal=>{if let Some(normal)=sample.normal{self.normals.insert(sample.index,normal);}},
            Kind::SmoothFace=>{self.smooth.insert(sample.index as usize);},
            Kind::SeamEdge=>{self.seams.insert(sample.index as usize);},
        }
    }

    fn uvs(&self,chart:Chart,blend:Blend)->[f32;2]{
        let values=blend.corners.map(|i|self.uv.get(&(chart.start+i)).copied().unwrap_or([0.0;2]));
        std::array::from_fn(|axis|(0..3).map(|i|blend.weights[i]*f64::from(values[i][axis])).sum::<f64>() as f32)
    }

    fn normal(&self,source:&PolygonMeshSource,chart:Chart,row:Corner)->Option<[f32;3]>{
        let get=|i:usize|self.normals.get(&source.faces[chart.source][i]).map(|n|n.map(|axis|if chart.reversed{-axis}else{axis}));
        if let Some(i)=row.exact{return get(i);}
        let [a,b,c]=row.blend.corners.map(get);
        let samples=[a?,b?,c?];
        let vector:Point=std::array::from_fn(|axis|(0..3).map(|i|row.blend.weights[i]*f64::from(samples[i][axis])).sum());
        let length=dot(vector,vector).sqrt();
        (length>0.0).then(||vector.map(|v|(v/length) as f32))
    }
}

#[derive(semio_framework_value::RetireOwned)]
struct Attribute {
    name:String,record:MeshAttribute,phase:u8,cursor:usize,face:usize,corner:usize,
}

fn sample_index(a:&MeshAttribute,idx:usize)->Result<u32,ValueError>{
    let i=a.indices.as_ref().map_or(Some(idx as u32),|ids|ids.get(idx).copied()).ok_or_else(||refuse("source attribute index is absent"))?;
    if i as usize>=a.values.len(){return Err(refuse("source attribute value is absent"));}
    Ok(i)
}

fn numbers(value:&DslValue)->Result<(&[DslValue],usize),ValueError>{
    if let Some(rows)=value.as_array(){
        if rows.is_empty()||rows.len()>16{return Err(refuse("intersection attribute tuple width is unsupported"));}
        Ok((rows,rows.len()))
    }else{Ok((std::slice::from_ref(value),0))}
}

fn linear_values(source:&PolygonMeshSource,a:&MeshAttribute,chart:Chart,blend:Blend)->Result<([f64;16],usize),ValueError>{
    let indices=blend.corners.map(|i|chart.index(source,a.domain,i));
    let get=|i:usize|a.value_at(indices[i]).ok_or_else(||refuse("intersection lost an affine source value")).and_then(numbers);
    let [(a,aw),(b,bw),(c,cw)]=[get(0)?,get(1)?,get(2)?];
    if aw!=bw||aw!=cw{return Err(refuse("intersection attribute tuple widths differ"));}
    let mut values=[0.0;16];
    for axis in 0..aw.max(1){
        let vals=[a[axis].as_f64(),b[axis].as_f64(),c[axis].as_f64()];
        let [Some(x),Some(y),Some(z)]=vals else{return Err(refuse("intersection attribute is not affine numeric data"));};
        values[axis]=x*blend.weights[0]+y*blend.weights[1]+z*blend.weights[2];
    }
    Ok((values,aw))
}

fn data_value(values:[f64;16],width:usize)->DslValue{
    if width==0{DslValue::float(values[0])}
    else{DslValue::Array(values[..width].iter().copied().map(DslValue::float).collect())}
}

fn varying(a:&MeshAttribute)->bool{matches!(a.domain,MeshAttributeDomain::Vertex|MeshAttributeDomain::Corner)}
fn linear(a:&MeshAttribute)->bool{varying(a)&&a.interpolation==MeshAttributeInterpolation::Linear}

#[derive(semio_framework_value::RetireOwned)]
pub(super) struct Projection {
    solid:MeshIntersectionSolid,source:PolygonMeshSource,output:PolygonMeshSource,offset:usize,vertex_offset:u32,
    offsets:Vec<usize>,vertex_ids:HistoryFoldIndex<u32,u32>,charts:Vec<Chart>,rows:Vec<Corner>,surface:Surface,
    keep_vertex_domain:bool,vertex_rows:Vec<usize>,
    normals:Vec<DslValue>,name:Option<String>,named_normal:bool,has_reversed:bool,
    attr:Option<Attribute>,face:usize,corner:usize,edge:usize,slot:usize,cursor:usize,
    start:usize,exact:Option<usize>,edge_origin:Option<usize>,edge_sample:Option<(usize,f64)>,profile_inside:bool,phase:u8,work:usize,
}

impl Projection {
    pub(super) fn new(solid:MeshIntersectionSolid,source:PolygonMeshSource,offset:usize,vertex_offset:u32)->Self{
        let keep_vertex_domain=solid.right_interpolations.is_empty()&&!solid.right_points.is_empty()
            &&solid.points.len()+solid.right_points.len()==solid.geometry.positions.len();
        Self{solid,source,output:empty(),offset,vertex_offset,offsets:Vec::new(),vertex_ids:HistoryFoldIndex::new(),
            keep_vertex_domain,vertex_rows:Vec::new(),
            charts:Vec::new(),rows:Vec::new(),surface:Default::default(),normals:Vec::new(),name:None,named_normal:false,has_reversed:false,
            attr:None,face:0,corner:0,edge:0,slot:0,cursor:0,start:0,exact:None,edge_origin:None,edge_sample:None,profile_inside:false,phase:0,work:0}
    }

    pub(super) fn progress(&self)->f32{f32::from(self.phase)/9.0}
    fn next(&mut self){self.cursor=0;self.face=0;self.corner=0;self.edge=0;self.phase+=1;}

    fn source_face(&self)->usize{
        let (operand,face)=self.solid.faces[self.face].source_face();
        operand*self.offset+face as usize
    }

    fn chart(&mut self)->Result<(),ValueError>{
        let source=self.source_face();
        let ring=self.source.faces.get(source).ok_or_else(||refuse("intersection supporting face is absent"))?;
        let i=self.edge+2;
        if i>=ring.len(){return Err(refuse("intersection supporting face has no affine chart"));}
        if let Some(mut chart)=Chart::new(&self.source,source,self.offsets[source],i){
            chart.reversed=matches!(self.solid.faces[self.face],MeshIntersectionFace::ReversedRightPlane(_));
            chart.profile_cap=matches!(self.solid.faces[self.face],MeshIntersectionFace::LeftBoundary(_)|MeshIntersectionFace::RightBoundary(_));
            self.has_reversed|=chart.reversed;
            chart.uncut=ring.len()==self.solid.geometry.faces[self.face].len();
            self.charts.push(chart);self.output.faces.push(Vec::new());self.edge=0;
        }else{self.edge+=1;}
        Ok(())
    }

    fn certify_exact_ring(&mut self){
        let chart=&mut self.charts[self.face];
        let result=&self.solid.geometry.faces[self.face];
        let source=&self.source.faces[chart.source];
        if source.len()!=result.len(){
            chart.exact_ring_checked=true;
            return;
        }
        let matches_at=|source_corner:usize,result_corner:usize|{
            self.source.vertices[source[source_corner] as usize]==self.solid.geometry.positions[result[result_corner] as usize]
        };
        if let Some(anchor)=chart.exact_ring_start{
            let candidate=chart.ring_corner(anchor,self.edge,source.len());
            if !matches_at(candidate,self.edge){
                chart.exact_ring_start=None;
                chart.exact_ring_checked=true;
            }else{
                self.edge+=1;
                chart.exact_ring_checked=self.edge==source.len();
            }
        }else if matches_at(self.edge,0){
            chart.exact_ring_start=Some(self.edge);
            self.edge=0;
        }else{
            self.edge+=1;
            chart.exact_ring_checked=self.edge==source.len();
        }
        if chart.exact_ring_checked{self.edge=0;}
    }

    fn profile_blend(&mut self,chart:Chart,id:u32,point:Point)->Option<Blend>{
        if !self.profile_inside{return None;}
        let (side,_) = self.solid.faces[self.face].source_face();
        let recipe = if side == 0 {
            if let Some(recipe) = self.solid.points.get(id as usize) { recipe }
            else {
                let start=self.solid.points.len() as u32;
                if self.solid.right_points.first().copied()!=Some(start)
                    || self.solid.right_points.get(id.checked_sub(start)? as usize).copied()!=Some(id) { return None; }
                self.keep_vertex_domain=false;
                return Some(chart.face_blend(point));
            }
        } else {
            let samples = &self.solid.right_interpolations;
            &samples[samples.binary_search_by_key(&id, |(vertex, _)| *vertex).ok()?].1
        };
        let MeshSourceVertex::Weighted{vertices,weights}=recipe else{return None;};
        let ring = &self.source.faces[chart.source];
        let mut blend = Blend { corners:[0;3], weights:[0.;3] };
        let mut count = 0;
        for (&vertex,&weight) in vertices.iter().zip(weights) {
            if !weight.is_finite() || weight < 0. {return None;}
            if weight == 0. {continue;}
            let source = vertex.checked_add(self.vertex_offset * side as u32)?;
            *blend.corners.get_mut(count)? = ring.iter().position(|&value|value == source)?;
            blend.weights[count] = weight;
            count += 1;
        }
        if (blend.weights.iter().sum::<f64>() - 1.).abs() > 1e-6 {return None;}
        let samples = blend.corners.map(|corner| chart.basis_point(&self.source,corner));
        for (axis,coordinate) in point.into_iter().enumerate() {
            let sample: f64 = samples.iter().zip(blend.weights).map(|(p,w)|p[axis]*w).sum();
            if (sample - coordinate).abs() > chart.epsilon {return None;}
        }
        Some(blend)
    }

    fn geometry(&mut self)->Result<(),ValueError>{
        if self.face==self.solid.geometry.faces.len(){self.next();return Ok(());}
        if self.charts.len()==self.face{return self.chart();}
        let chart=self.charts[self.face];
        if !chart.exact_ring_checked{self.certify_exact_ring();return Ok(());}
        let ring=&self.solid.geometry.faces[self.face];
        let id=ring[self.corner];
        let p=self.solid.geometry.positions[id as usize].map(f64::from);
        let source_len=self.source.faces[chart.source].len();
        if let Some(start)=chart.exact_ring_start{
            let corner=chart.ring_corner(start,self.corner,source_len);
            self.exact=Some(corner);
            self.edge_origin=Some(if chart.reversed{(corner+source_len-1)%source_len}else{corner});
            self.finish_corner(chart,id,p,None);
            return Ok(());
        }
        let q=self.solid.geometry.positions[ring[(self.corner+1)%ring.len()] as usize].map(f64::from);
        let j=self.edge;
        let (a,b)=(chart.basis_point(&self.source,j),chart.basis_point(&self.source,(j+1)%source_len));
        let (side,parameter)=chart.edge_parameter(a,b,p);
        if (!chart.profile_cap&&side< -chart.epsilon)||dot(difference(p,chart.origin),chart.normal).abs()>chart.epsilon{
            return Err(refuse("intersection crosses a coplanar authored supporting-face boundary"));
        }
        if let Some(t)=parameter{
            self.edge_sample.get_or_insert((j,t));
            if chart.edge_parameter(a,b,q).1.is_some(){self.edge_origin=Some(j);}
        }
        if p==a{self.exact=Some(j);}
        if chart.profile_cap && self.exact.is_none() && self.edge_sample.is_none() {
            let axis = (0..3).max_by(|&a,&b|chart.normal[a].abs().total_cmp(&chart.normal[b].abs())).unwrap();
            let [u,v] = [(axis+1)%3,(axis+2)%3];
            if (a[v]>p[v])!=(b[v]>p[v]) && p[u]<a[u]+(p[v]-a[v])*(b[u]-a[u])/(b[v]-a[v]){
                self.profile_inside = !self.profile_inside;
            }
        }
        self.edge+=1;
        if self.edge==source_len{
            let interior = if chart.profile_cap&&self.exact.is_none()&&self.edge_sample.is_none(){
                Some(self.profile_blend(chart,id,p).ok_or_else(||refuse("certified profile cut has no authored interior sample"))?)
            } else {None};
            self.finish_corner(chart,id,p,interior);
        }
        Ok(())
    }

    fn finish_corner(&mut self,chart:Chart,id:u32,point:Point,interior:Option<Blend>){
        let vertex=*self.vertex_ids.entry(id).or_insert_with(||{
            let next=self.output.vertices.len() as u32;
            self.output.vertices.push(self.solid.geometry.positions[id as usize]);
            if self.keep_vertex_domain{self.vertex_rows.push(self.rows.len());}
            next
        });
        self.output.faces[self.face].push(vertex);
        let exact=self.exact.take();
        let edge_sample=self.edge_sample.take();
        self.charts[self.face].uncut&=exact.is_some();
        self.charts[self.face].needs_affine|=exact.is_none()&&edge_sample.is_none();
        let blend=edge_sample.map_or_else(||interior.unwrap_or_else(||chart.face_blend(point)),|(a,t)|{
            let b=(a+1)%self.source.faces[chart.source].len();
            Blend{corners:[a,b,a],weights:[1.-t,t,0.]}
        });
        self.rows.push(Corner{face:self.face,vertex,blend,exact,edge:self.edge_origin.take()});
        self.edge=0;self.corner+=1;self.profile_inside=false;
        if self.corner==self.solid.geometry.faces[self.face].len(){
            if self.surface.smooth.contains(&chart.source){
                self.output.surface.push(Sample{kind:Kind::SmoothFace,index:self.face as u32,normal:None,uv:None});
            }
            self.face+=1;self.corner=0;
        }
    }

    fn verify_uv(&mut self)->Result<(),ValueError>{
        if self.surface.uv.is_empty()||self.face==self.charts.len(){self.next();return Ok(());}
        let chart=self.charts[self.face];
        if !chart.needs_affine{self.face+=1;return Ok(());}
        let expected=self.surface.uv.get(&(chart.start+self.corner)).copied().unwrap_or([0.0;2]);
        let actual=self.surface.uvs(chart,chart.face_blend(chart.basis_point(&self.source,self.corner)));
        if expected.into_iter().zip(actual).any(|(a,b)|(a-b).abs()>1e-5*(1.0+a.abs())){
            return Err(refuse("intersection cannot transfer a nonaffine corner UV patch"));
        }
        self.corner+=1;
        if self.corner==self.source.faces[chart.source].len(){self.corner=0;self.face+=1;}
        Ok(())
    }

    fn surface_row(&mut self){
        let Some(&row)=self.rows.get(self.cursor)else{self.next();return;};
        let chart=self.charts[row.face];
        let uv=row.exact.map_or_else(||self.surface.uvs(chart,row.blend),|i|self.surface.uv.get(&(chart.start+i)).copied().unwrap_or([0.0;2]));
        if uv!=[0.0;2]{self.output.surface.push(Sample{kind:Kind::CornerUv,index:self.cursor as u32,normal:None,uv:Some(uv)});}
        if row.edge.is_some_and(|edge|self.surface.seams.contains(&(chart.start+edge))){
            self.output.surface.push(Sample{kind:Kind::SeamEdge,index:self.cursor as u32,normal:None,uv:None});
        }
        if let Some(normal)=self.surface.normal(&self.source,chart,row){
            if let Some(previous)=self.surface.mapped_normals.get(&row.vertex){self.surface.conflicting|=*previous!=normal;}
            else{self.surface.mapped_normals.insert(row.vertex,normal);}
        }
        self.cursor+=1;
    }

    fn normal_row(&mut self){
        if self.cursor==self.output.vertices.len(){self.next();return;}
        if let Some(&normal)=self.surface.mapped_normals.get(&(self.cursor as u32)){
            self.output.surface.push(Sample{kind:Kind::VertexNormal,index:self.cursor as u32,normal:Some(normal),uv:None});
        }
        self.cursor+=1;
    }

    fn verify_attribute(&self,a:&MeshAttribute,chart:Chart,index:usize)->Result<(),ValueError>{
        if linear(a){
            let sample=a.value_at(chart.index(&self.source,a.domain,index)).ok_or_else(||refuse("intersection source sample is absent"))?;
            let (actual,actual_width)=numbers(sample)?;
            let (expected,width)=linear_values(&self.source,a,chart,chart.face_blend(chart.basis_point(&self.source,index)))?;
            if width!=actual_width||actual.iter().zip(expected).any(|(a,b)|a.as_f64().is_none_or(|a|(a-b).abs()>1e-5*(1.0+a.abs()))){
                return Err(refuse("intersection cannot transfer a nonaffine named channel"));
            }
        }else if varying(a)&&sample_index(a,chart.index(&self.source,a.domain,index))?!=sample_index(a,chart.index(&self.source,a.domain,0))?{
            return Err(refuse("intersection cannot transfer a discontinuous categorical face channel"));
        }
        Ok(())
    }

    pub(super) fn demand(&self,copy:usize)->Result<RetirementDemand,ValueError>{
        let mut d=RetirementDemand{depth:1,..Default::default()};
        if self.phase==6{
            if let Some(attr)=&self.attr{
                let count=self.attribute_count(attr.record.domain);
                match attr.phase{
                    1=>d.capacity_bytes=count*if attr.record.indices.is_none(){size_of::<DslValue>()}else{size_of::<u32>()},
                    3=>{d.capacity_bytes=self.output.attributes.next_insert_capacity_byte_demand(&attr.name,copy)?;
                        d.depth=self.output.attributes.next_insert_depth_demand(&attr.name)?;},
                    _=>{},
                }
            }
        }
        if matches!(self.phase,7|8){
            if let Some(name)=&self.name{
                if self.cursor==self.rows.len(){
                    d.capacity_bytes=self.output.attributes.next_insert_capacity_byte_demand(name,copy)?;
                    d.depth=self.output.attributes.next_insert_depth_demand(name)?;
                }
            }
        }
        Ok(d)
    }

    fn insert(out:&mut PolygonMeshSource,key:&String,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)
        ->Result<bool,ValueError>{
        let grant=effect.grant();
        if out.attributes.next_insert_capacity_byte_demand(key,grant.maximum_copy_bytes)?==0{return Ok(true);}
        *receipt=out.attributes.reserve_insert_step(key,grant).map_err(|(error,progress)|error.with_retained_progress(progress))?;
        Ok(false)
    }

    fn attribute_count(&self,domain:MeshAttributeDomain)->usize{
        match domain{
            MeshAttributeDomain::Face=>self.charts.len(),
            MeshAttributeDomain::Vertex=>self.vertex_rows.len(),
            _=>self.rows.len(),
        }
    }

    fn attributes(&mut self,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<(),ValueError>{
        if self.slot==self.source.attributes.slot_count(){self.next();return Ok(());}
        let Some((source_name,a))=self.source.attributes.slot_entry(self.slot)else{self.slot+=1;return Ok(());};
        if self.attr.is_none(){
            self.named_normal|=a.semantic==MeshAttributeSemantic::Normal;
            let domain=if a.domain==MeshAttributeDomain::Vertex&&!self.keep_vertex_domain{MeshAttributeDomain::Corner}else{a.domain};
            let record=MeshAttribute{domain,semantic:a.semantic,interpolation:a.interpolation,values:Vec::new(),indices:(!linear(a)&&!(self.has_reversed&&a.semantic==MeshAttributeSemantic::Normal)).then(Vec::new)};
            self.attr=Some(Attribute{name:source_name.clone(),record,phase:0,cursor:0,face:0,corner:0});
            return Ok(());
        }
        let mut attr=self.attr.take().unwrap();
        let result=self.attribute(&mut attr,effect,receipt);
        if attr.phase!=4||result.is_err(){self.attr=Some(attr);}else{self.slot+=1;}
        result
    }

    fn attribute(&mut self,attr:&mut Attribute,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<(),ValueError>{
        let a=self.source.attributes.slot_entry(self.slot).unwrap().1;
        match attr.phase{
            0=>{
                if !varying(a)||attr.face==self.charts.len(){attr.phase=1;return Ok(());}
                let chart=self.charts[attr.face];
                if chart.uncut||(linear(a)&&!chart.needs_affine){attr.face+=1;return Ok(()); }
                self.verify_attribute(a,chart,attr.corner)?;
                attr.corner+=1;
                if attr.corner==self.source.faces[chart.source].len(){attr.corner=0;attr.face+=1;}
            },
            1=>{
                let count=self.attribute_count(attr.record.domain);
                receipt.retained_capacity_bytes=if let Some(indices)=attr.record.indices.as_mut(){reserve_polygon_backing(indices,count,effect)?}
                    else{reserve_polygon_backing(&mut attr.record.values,count,effect)?};
                attr.phase=2;
            },
            2=>{
                let domain=attr.record.domain;
                if attr.cursor==self.attribute_count(domain){attr.phase=3;return Ok(());}
                let row=match domain{
                    MeshAttributeDomain::Face=>None,
                    MeshAttributeDomain::Vertex=>Some(self.rows[self.vertex_rows[attr.cursor]]),
                    _=>Some(self.rows[attr.cursor]),
                };
                let chart=self.charts[row.map_or(attr.cursor,|row|row.face)];
                let index=if let Some(row)=row{
                    if a.domain==MeshAttributeDomain::Edge{
                        chart.index(&self.source,a.domain,row.edge.ok_or_else(||refuse("generated intersection edge lacks authored edge provenance"))?)
                    }else{chart.index(&self.source,a.domain,row.exact.unwrap_or(0))}
                }else{chart.source};
                if let Some(indices)=&mut attr.record.indices{indices.push(sample_index(a,index)?);}
                else{
                    let value=if let Some(row)=row.filter(|row|linear(a)&&row.exact.is_none()){
                        let (values,width)=linear_values(&self.source,a,chart,row.blend)?;
                        data_value(values,width)
                    }else{a.value_at(index).ok_or_else(||refuse("intersection attribute sample is absent"))?.clone()};
                    if chart.reversed&&a.semantic==MeshAttributeSemantic::Normal&&value!=DslValue::Null{
                        let (values,width)=numbers(&value)?;
                        let mut reversed=[0.0;16];
                        for (i,number) in values.iter().enumerate(){reversed[i]=-number.as_f64().ok_or_else(||refuse("difference normal is not numeric"))?;}
                        attr.record.values.push(data_value(reversed,width));
                    }else{attr.record.values.push(value);}
                }
                attr.cursor+=1;
            },
            3=>{
                if !Self::insert(&mut self.output,&attr.name,effect,receipt)?{return Ok(());}
                if attr.record.indices.is_some(){
                    let (_,original)=self.source.attributes.extract_slot_if(self.slot,|_,_|false).ok_or_else(||refuse("intersection lost the source attribute backing"))?;
                    attr.record.values=original.values;
                }
                let name=std::mem::take(&mut attr.name);
                let record=std::mem::replace(&mut attr.record,MeshAttribute{domain:MeshAttributeDomain::Face,
                    semantic:MeshAttributeSemantic::Custom,interpolation:MeshAttributeInterpolation::Constant,values:Vec::new(),indices:None});
                match self.output.attributes.insert_reserved(name,record,effect.grant()){
                    Ok((None,progress))=>{*receipt=progress;attr.phase=4;},
                    Ok((Some(_),_))=>unreachable!(),
                    Err((error,name,record))=>{attr.name=name;attr.record=record;return Err(error);},
                }
            },
            _=>unreachable!(),
        }
        Ok(())
    }

    fn promoted(&mut self,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<bool,ValueError>{
        let raw=self.phase==8;
        if !self.surface.conflicting||(!raw&&(self.named_normal||self.surface.smooth.is_empty())){return Ok(true);}
        if self.name.is_none(){
            if self.output.attributes.len()==64{return Err(refuse("intersection cannot preserve conflicting normals within its attribute limit"));}
            let mut name=if raw{"intersectionRawNormal"}else{"normal"}.to_owned();
            while self.output.attributes.contains_key(&name){name.push('_');}
            self.name=Some(name);return Ok(false);
        }
        if let Some(&row)=self.rows.get(self.cursor){
            let chart=self.charts[row.face];
            let on_source=self.surface.normal(&self.source,chart,row);
            let normal=if raw{on_source}else{
                Some(on_source.filter(|_|self.surface.smooth.contains(&chart.source)).unwrap_or_else(||chart.normal.map(|axis|if chart.reversed{-axis as f32}else{axis as f32})))
            };
            self.normals.push(normal.map_or(DslValue::Null,|normal|DslValue::Array(normal.map(|v|DslValue::float(f64::from(v))).to_vec())));
            self.cursor+=1;return Ok(false);
        }
        if !Self::insert(&mut self.output,self.name.as_ref().unwrap(),effect,receipt)?{return Ok(false);}
        let record=MeshAttribute{domain:MeshAttributeDomain::Corner,
            semantic:if raw{MeshAttributeSemantic::Custom}else{MeshAttributeSemantic::Normal},
            interpolation:if raw{MeshAttributeInterpolation::Nearest}else{MeshAttributeInterpolation::Linear},
            values:std::mem::take(&mut self.normals),indices:None};
        let (_,progress)=self.output.attributes.insert_reserved(self.name.take().unwrap(),record,effect.grant()).map_err(|(error,_,_)|error)?;
        *receipt=progress;
        Ok(true)
    }

    pub(super) fn step(&mut self,effect:&mut RetainedAllocationEffect<'_>,receipt:&mut RetainedCloneProgress)->Result<Option<PolygonMeshSource>,ValueError>{
        if self.work==4_000_000{return Err(refuse("intersection authored projection work budget exhausted"));}
        self.work+=1;
        match self.phase {
            0=>{
                if let Some(face)=self.source.faces.get(self.cursor){
                    self.offsets.push(self.start);self.start+=face.len();self.cursor+=1;
                }else{self.next();}
            },
            1=>{
                if let Some(sample)=self.source.surface.get(self.cursor){self.surface.add(*sample);self.cursor+=1;}
                else{self.next();}
            },
            2=>self.geometry()?,
            3=>self.verify_uv()?,
            4=>self.surface_row(),
            5=>self.normal_row(),
            6=>self.attributes(effect,receipt)?,
            7|8=>if self.promoted(effect,receipt)?{self.next();},
            9=>{
                self.output.materials=std::mem::take(&mut self.source.materials);
                self.output.textures=std::mem::take(&mut self.source.textures);
                return Ok(Some(std::mem::replace(&mut self.output,empty())));
            },
            _=>unreachable!(),
        }
        Ok(None)
    }
}
