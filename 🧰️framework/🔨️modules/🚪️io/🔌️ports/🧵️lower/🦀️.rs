//! 🧵️ Borrowed vector events retain only funded numeric ancestry, never child strings or octets.
use super::*;
use semio_framework_value::ValueRefusalKind;

/// 🎟️ Caller-authored work, depth and physical backing authority for one source turn.
#[derive(Clone,Copy,Debug,Default)]
pub struct VectorSceneGrant { pub maximum_source_items:usize, pub maximum_depth:usize, pub maximum_copy_bytes:usize, pub maximum_capacity_bytes:usize, pub maximum_release_bytes:usize }
/// 🧾️ Actual source visits and numeric ancestry backing effects, including refused turns.
#[derive(Clone,Copy,Debug,Default)]
pub struct VectorSceneReceipt { pub source_items:usize, pub copied_bytes:usize, pub retained_capacity_bytes:usize, pub released_bytes:usize }
/// 🎨️ One style borrows its name while carrying only fixed-size presentation values.
#[derive(Clone,Copy,Debug)]
pub struct VectorStyleRef<'a> { pub name:&'a str, pub fill:Option<VectorRgba>, pub stroke:Option<VectorRgba>, pub stroke_width:Option<f64>, pub opacity:Option<f32> }
/// 🖼️ Source-backed image octets support intrinsic binary and native JSON array storage.
#[derive(Clone,Copy,Debug)]
pub enum VectorImageBytes<'a> { Bytes(&'a [u8]), Values(&'a [DslValue]) }
impl VectorImageBytes<'_> {
    /// 📏️ Returns the borrowed source extent without traversing image pixels.
    pub fn len(&self)->usize { match self { Self::Bytes(bytes)=>bytes.len(), Self::Values(values)=>values.len() } }
    /// 🫙️ Reports an empty borrowed source.
    pub fn is_empty(&self)->bool { self.len()==0 }
    /// 🔎️ Reads one requested octet; callers fund and page their own pixel processing.
    pub fn get(&self,index:usize)->Result<Option<u8>,ValueError> { match self { Self::Bytes(bytes)=>Ok(bytes.get(index).copied()), Self::Values(values)=>values.get(index).map(|value| match value { DslValue::Number(number)=>number.as_u64().and_then(|value|u8::try_from(value).ok()).ok_or_else(invalid), _=>Err(invalid()) }).transpose() } }
}
/// 🎬️ Exactly one bounded descriptor; text, identifiers and image payloads borrow the retained source.
#[derive(Clone,Copy,Debug)]
pub enum VectorSceneEvent<'a> {
    Canvas { schema:&'a str, canvas:VectorCanvas }, Style(VectorStyleRef<'a>),
    BeginLayer { id:&'a str, name:&'a str, visible:bool }, BeginGroup { transform:VectorTransform }, EndGroup,
    BeginPath { style:Option<&'a str> }, Segment(VectorSegment), EndPath,
    Text { value:&'a str, at:VectorPoint2, style:Option<&'a str> },
    Image { at:VectorPoint2, width:f64, height:f64, mime:&'a str, bytes:VectorImageBytes<'a> }, EndLayer, Done,
}
/// 🧾️ A source descriptor and the actual work/backing receipt of this turn.
#[derive(Clone,Copy,Debug)]
pub struct VectorSceneStep<'a> { pub event:VectorSceneEvent<'a>, pub receipt:VectorSceneReceipt }
/// 🪜️ Stable numeric ancestry can cross source loans without self references or scene materialization.
#[derive(Debug,Default)]
pub struct VectorSceneCursor { stage:u8, row:usize, path:Vec<usize>, segment:Option<usize>, pending_finish:bool, last:VectorSceneReceipt }
struct Budget { grant:VectorSceneGrant, receipt:VectorSceneReceipt }
fn invalid()->ValueError { ValueError::literal(ValueRefusalKind::InvalidValue,"borrowed vector scene has an invalid source record") }
impl Budget {
    fn visit(&mut self,count:usize)->Result<(),ValueError> { if count>self.grant.maximum_source_items.saturating_sub(self.receipt.source_items) { return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"borrowed vector scene source grant exhausted")); } self.receipt.source_items+=count;Ok(()) }
}
fn record<'a>(value:&'a DslValue,allowed:&[&str],budget:&mut Budget)->Result<&'a [(String,DslValue)],ValueError> {
    budget.visit(1)?;let DslValue::Object(rows)=value else{return Err(invalid())};if rows.len()>allowed.len(){return Err(invalid())}budget.visit(rows.len())?;
    for (index,(key,_)) in rows.iter().enumerate(){if !allowed.contains(&key.as_str())||rows[..index].iter().any(|(previous,_)|previous==key){return Err(invalid())}}Ok(rows)
}
fn field<'a>(rows:&'a [(String,DslValue)],key:&str)->Option<&'a DslValue> { rows.iter().find(|(name,_)|name==key).map(|(_,value)|value) }
fn required<'a>(rows:&'a [(String,DslValue)],key:&str)->Result<&'a DslValue,ValueError> { field(rows,key).ok_or_else(invalid) }
fn text(value:&DslValue)->Result<&str,ValueError> { if let DslValue::String(value)=value { Ok(value) } else { Err(invalid()) } }
fn number(value:&DslValue)->Result<f64,ValueError> { if let DslValue::Number(value)=value { let value=value.as_f64();if value.is_finite(){Ok(value)}else{Err(invalid())} } else { Err(invalid()) } }
fn boolean(value:&DslValue)->Result<bool,ValueError> { if let DslValue::Bool(value)=value {Ok(*value)}else{Err(invalid())} }
fn array(value:&DslValue)->Result<&[DslValue],ValueError> { if let DslValue::Array(value)=value {Ok(value)}else{Err(invalid())} }
fn rows_or_empty<'a>(rows:&'a [(String,DslValue)],key:&str)->Result<&'a [DslValue],ValueError> {field(rows,key).map_or(Ok(&[]),array)}
fn optional_text<'a>(rows:&'a [(String,DslValue)],key:&str)->Result<Option<&'a str>,ValueError> {match field(rows,key){None|Some(DslValue::Null)=>Ok(None),Some(value)=>text(value).map(Some)}}
fn optional_number(rows:&[(String,DslValue)],key:&str)->Result<Option<f64>,ValueError> {match field(rows,key){None|Some(DslValue::Null)=>Ok(None),Some(value)=>number(value).map(Some)}}
fn point(value:&DslValue,budget:&mut Budget)->Result<VectorPoint2,ValueError> {let rows=record(value,&["x","y"],budget)?;Ok(VectorPoint2{x:number(required(rows,"x")?)?,y:number(required(rows,"y")?)?})}
fn point3(value:&DslValue,budget:&mut Budget)->Result<VectorPoint3,ValueError> {let rows=record(value,&["x","y","z"],budget)?;Ok(VectorPoint3{x:number(required(rows,"x")?)?,y:number(required(rows,"y")?)?,z:number(required(rows,"z")?)?})}
fn color(value:&DslValue,budget:&mut Budget)->Result<VectorRgba,ValueError> {let rows=record(value,&["r","g","b","a"],budget)?;let(r,g,b,a)=(number(required(rows,"r")?)? as f32,number(required(rows,"g")?)? as f32,number(required(rows,"b")?)? as f32,number(required(rows,"a")?)? as f32);if [r,g,b,a].iter().any(|value|!value.is_finite()){return Err(invalid())}Ok(VectorRgba{r,g,b,a})}
fn optional_color(rows:&[(String,DslValue)],key:&str,budget:&mut Budget)->Result<Option<VectorRgba>,ValueError> {match field(rows,key){None|Some(DslValue::Null)=>Ok(None),Some(value)=>color(value,budget).map(Some)}}
fn transform(value:&DslValue,budget:&mut Budget)->Result<VectorTransform,ValueError> {let rows=record(value,&["translation","rotation","scale"],budget)?;let rotation=record(required(rows,"rotation")?,&["x","y","z","w"],budget)?;Ok(VectorTransform{translation:point3(required(rows,"translation")?,budget)?,rotation:VectorQuaternion{x:number(required(rotation,"x")?)?,y:number(required(rotation,"y")?)?,z:number(required(rotation,"z")?)?,w:number(required(rotation,"w")?)?},scale:point3(required(rows,"scale")?,budget)?})}
fn segment(value:&DslValue,budget:&mut Budget)->Result<VectorSegment,ValueError> {
let header=record(value,&["kind","to","c1","c2","c","rx","ry","xRotation","largeArc","sweep"],budget)?;Ok(match text(required(header,"kind")?)?{
"moveTo"=>{let rows=record(value,&["kind","to"],budget)?;VectorSegment::MoveTo{to:point(required(rows,"to")?,budget)?}},
"lineTo"=>{let rows=record(value,&["kind","to"],budget)?;VectorSegment::LineTo{to:point(required(rows,"to")?,budget)?}},
"cubicTo"=>{let rows=record(value,&["kind","c1","c2","to"],budget)?;VectorSegment::CubicTo{c1:point(required(rows,"c1")?,budget)?,c2:point(required(rows,"c2")?,budget)?,to:point(required(rows,"to")?,budget)?}},
"quadTo"=>{let rows=record(value,&["kind","c","to"],budget)?;VectorSegment::QuadTo{c:point(required(rows,"c")?,budget)?,to:point(required(rows,"to")?,budget)?}},
"arcTo"=>{let rows=record(value,&["kind","rx","ry","xRotation","largeArc","sweep","to"],budget)?;VectorSegment::ArcTo{rx:number(required(rows,"rx")?)?,ry:number(required(rows,"ry")?)?,x_rotation:number(required(rows,"xRotation")?)?,large_arc:boolean(required(rows,"largeArc")?)?,sweep:boolean(required(rows,"sweep")?)?,to:point(required(rows,"to")?,budget)?}},
"close"=>{record(value,&["kind"],budget)?;VectorSegment::Close},_=>return Err(invalid())})}

impl VectorSceneCursor {
    /// 🌱️ Begins without allocating source or ancestry ownership.
    pub fn new()->Self { Self::default() }
    /// 📊️ Reports performed effects even when the preceding turn was refused.
    pub fn last_receipt(&self)->VectorSceneReceipt {self.last}
    /// 🎟️ Reads the next physical ancestry demand without advancing or allocating its source.
    pub fn next_demand(&self,content:&DslValue)->Result<semio_framework_value::RetirementDemand,ValueError>{
        use semio_framework_value::RetirementDemand;
        let mut demand=RetirementDemand{depth:1,..Default::default()};let mut stage=self.stage;let mut row=self.row;
        if stage==5{return Ok(demand)}if self.pending_finish&&self.path.is_empty(){stage=4;}
        let mut budget=Budget{grant:VectorSceneGrant{maximum_source_items:usize::MAX,maximum_depth:usize::MAX,..Default::default()},receipt:Default::default()};
        let root=record(content,&["schema","canvas","styles","layers"],&mut budget)?;
        if stage==0{return Ok(demand)}if stage==1{if rows_or_empty(root,"styles")?.get(row).is_some(){return Ok(demand)}stage=2;row=0;}
        let layers=rows_or_empty(root,"layers")?;let Some(layer)=layers.get(row)else{return Ok(demand)};
        if stage==2||stage==4{return Ok(demand)}
        let layer=record(layer,&["id","name","visible","root"],&mut budget)?;let mut node=required(layer,"root")?;
        demand.depth=self.path.len().max(1);
        for depth in 0..self.path.len(){let index=self.path[depth]+usize::from(self.pending_finish&&depth+1==self.path.len());let rows=record(node,&["kind","transform","children"],&mut budget)?;let children=rows_or_empty(rows,"children")?;if index==children.len()&&depth+1==self.path.len(){return Ok(demand)}node=children.get(index).ok_or_else(invalid)?;}
        let rows=record(node,&["kind","segments","style","value","at","transform","children","width","height","mime","bytes"],&mut budget)?;
        if text(required(rows,"kind")?)?=="group"{demand.depth=self.path.len().checked_add(1).ok_or_else(invalid)?;if self.path.len()==self.path.capacity(){let size=std::mem::size_of::<usize>();demand.capacity_bytes=demand.depth.checked_mul(size).ok_or_else(invalid)?;demand.copy_bytes=self.path.len()*size;demand.release_bytes=self.path.capacity()*size;}}
        Ok(demand)
    }
    /// 🔎️ Reborrowing a retained leaf has no new backing obligation.
    pub fn current_leaf_demand(&self)->semio_framework_value::RetirementDemand{semio_framework_value::RetirementDemand{depth:self.path.len().max(1),..Default::default()}}
    /// 🧭️ Borrows one event, charging every revisited source record and funded ancestry growth.
    pub fn advance<'a>(&mut self,content:&'a DslValue,grant:VectorSceneGrant,expected_schema:Option<&str>)->Result<VectorSceneStep<'a>,ValueError> {
        if self.pending_finish { self.finish_node(); self.pending_finish=false; }
        let mut budget=Budget{grant,receipt:VectorSceneReceipt::default()};let result=self.advance_inner(content,expected_schema,&mut budget);self.last=budget.receipt;result.map(|event|VectorSceneStep{event,receipt:self.last})
    }
    fn finish_node(&mut self){self.segment=None;if let Some(index)=self.path.last_mut(){*index+=1}else{self.stage=4}}
    fn advance_inner<'a>(&mut self,content:&'a DslValue,expected_schema:Option<&str>,budget:&mut Budget)->Result<VectorSceneEvent<'a>,ValueError> {
        if self.stage==5{return Ok(VectorSceneEvent::Done)}
        let root=record(content,&["schema","canvas","styles","layers"],budget)?;
        if self.stage==0 {let schema=text(required(root,"schema")?)?;if expected_schema.is_some_and(|expected|schema!=expected){return Err(invalid())}let rows=record(required(root,"canvas")?,&["width","height","background"],budget)?;let canvas=VectorCanvas{width:number(required(rows,"width")?)?,height:number(required(rows,"height")?)?,background:optional_color(rows,"background",budget)?};self.stage=1;return Ok(VectorSceneEvent::Canvas{schema,canvas})}
        if self.stage==1 {let styles=rows_or_empty(root,"styles")?;if let Some(value)=styles.get(self.row){budget.visit(1)?;let rows=record(value,&["name","fill","stroke","strokeWidth","opacity"],budget)?;let style=VectorStyleRef{name:text(required(rows,"name")?)?,fill:optional_color(rows,"fill",budget)?,stroke:optional_color(rows,"stroke",budget)?,stroke_width:optional_number(rows,"strokeWidth")?,opacity:optional_number(rows,"opacity")?.map(|value|value as f32)};if style.opacity.is_some_and(|value|!value.is_finite()){return Err(invalid())}self.row+=1;return Ok(VectorSceneEvent::Style(style))}self.stage=2;self.row=0;}
        let layers=rows_or_empty(root,"layers")?;let Some(layer)=layers.get(self.row)else{self.stage=5;return Ok(VectorSceneEvent::Done)};budget.visit(1)?;let layer=record(layer,&["id","name","visible","root"],budget)?;
        if self.stage==2 {let event=VectorSceneEvent::BeginLayer{id:text(required(layer,"id")?)?,name:text(required(layer,"name")?)?,visible:boolean(required(layer,"visible")?)?};self.stage=3;return Ok(event)}
        if self.stage==4 {self.row+=1;self.stage=2;return Ok(VectorSceneEvent::EndLayer)}
        if self.path.len()>budget.grant.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"borrowed vector scene ancestry exceeds original depth grant"))}
        let mut node=required(layer,"root")?;
        for depth in 0..self.path.len(){let index=self.path[depth];let rows=record(node,&["kind","transform","children"],budget)?;if text(required(rows,"kind")?)?!="group"{return Err(invalid())}let children=rows_or_empty(rows,"children")?;budget.visit(1)?;if index==children.len()&&depth+1==self.path.len(){self.path.pop();self.finish_node();return Ok(VectorSceneEvent::EndGroup)}node=children.get(index).ok_or_else(invalid)?;}
        let rows=record(node,&["kind","segments","style","value","at","transform","children","width","height","mime","bytes"],budget)?;
        match text(required(rows,"kind")?)? {
            "group"=>{let rows=record(node,&["kind","transform","children"],budget)?;let transform=transform(required(rows,"transform")?,budget)?;if self.path.len()>=budget.grant.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"borrowed vector scene group exceeds original depth grant"))}if self.path.len()==self.path.capacity(){let bytes=std::mem::size_of::<usize>();let old=self.path.capacity()*bytes;let copied=self.path.len()*bytes;let next=(self.path.len()+1)*bytes;if next>budget.grant.maximum_capacity_bytes||old>budget.grant.maximum_release_bytes||copied>budget.grant.maximum_copy_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"borrowed vector ancestry growth is unfunded"))}budget.visit(self.path.len())?;let mut backing=Vec::new();backing.try_reserve_exact(self.path.len()+1).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"borrowed vector ancestry allocation failed"))?;backing.extend_from_slice(&self.path);let previous=std::mem::replace(&mut self.path,backing);drop(previous);let retained=self.path.capacity()*bytes;budget.receipt.retained_capacity_bytes+=retained;budget.receipt.released_bytes+=old;budget.receipt.copied_bytes+=copied;if retained>budget.grant.maximum_capacity_bytes{return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit,"borrowed vector ancestry exceeded capacity grant"))}}self.path.push(0);Ok(VectorSceneEvent::BeginGroup{transform})},
            "path"=>{let rows=record(node,&["kind","segments","style"],budget)?;let segments=array(required(rows,"segments")?)?;if let Some(index)=self.segment{if let Some(value)=segments.get(index){budget.visit(1)?;let value=segment(value,budget)?;self.segment=Some(index+1);Ok(VectorSceneEvent::Segment(value))}else{self.finish_node();Ok(VectorSceneEvent::EndPath)}}else{let style=optional_text(rows,"style")?;self.segment=Some(0);Ok(VectorSceneEvent::BeginPath{style})}},
            "text"=>{let rows=record(node,&["kind","value","at","style"],budget)?;let event=VectorSceneEvent::Text{value:text(required(rows,"value")?)?,at:point(required(rows,"at")?,budget)?,style:optional_text(rows,"style")?};self.pending_finish=true;Ok(event)},
            "image"=>{let rows=record(node,&["kind","at","width","height","mime","bytes"],budget)?;let bytes=match required(rows,"bytes")?{DslValue::Bytes(bytes)=>VectorImageBytes::Bytes(bytes),DslValue::Array(values)=>VectorImageBytes::Values(values),_=>return Err(invalid())};let event=VectorSceneEvent::Image{at:point(required(rows,"at")?,budget)?,width:number(required(rows,"width")?)?,height:number(required(rows,"height")?)?,mime:text(required(rows,"mime")?)?,bytes};self.pending_finish=true;Ok(event)},_=>Err(invalid()),
        }
    }
    /// 🔎️ Reborrows the active leaf during style lookup or text pagination without advancing its address.
    pub fn current_leaf<'a>(&mut self,content:&'a DslValue,grant:VectorSceneGrant)->Result<VectorSceneStep<'a>,ValueError> {
        let mut budget=Budget{grant,receipt:Default::default()};let result=self.leaf_inner(content,&mut budget);self.last=budget.receipt;result.map(|event|VectorSceneStep{event,receipt:self.last})
    }
    fn leaf_inner<'a>(&self,content:&'a DslValue,budget:&mut Budget)->Result<VectorSceneEvent<'a>,ValueError> {
        if self.stage!=3{return Err(invalid())}if self.path.len()>budget.grant.maximum_depth{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"borrowed vector leaf exceeds original depth grant"))}
        let root=record(content,&["schema","canvas","styles","layers"],budget)?;let layers=rows_or_empty(root,"layers")?;budget.visit(1)?;let layer=record(layers.get(self.row).ok_or_else(invalid)?,&["id","name","visible","root"],budget)?;let mut node=required(layer,"root")?;
        for index in &self.path{let rows=record(node,&["kind","transform","children"],budget)?;if text(required(rows,"kind")?)?!="group"{return Err(invalid())}budget.visit(1)?;node=rows_or_empty(rows,"children")?.get(*index).ok_or_else(invalid)?;}
        let rows=record(node,&["kind","segments","style","value","at","width","height","mime","bytes"],budget)?;
        match text(required(rows,"kind")?)?{
            "path"=>{let rows=record(node,&["kind","segments","style"],budget)?;Ok(VectorSceneEvent::BeginPath{style:optional_text(rows,"style")?})},
            "text"=>{let rows=record(node,&["kind","value","at","style"],budget)?;Ok(VectorSceneEvent::Text{value:text(required(rows,"value")?)?,at:point(required(rows,"at")?,budget)?,style:optional_text(rows,"style")?})},
            "image"=>{let rows=record(node,&["kind","at","width","height","mime","bytes"],budget)?;let bytes=match required(rows,"bytes")?{DslValue::Bytes(bytes)=>VectorImageBytes::Bytes(bytes),DslValue::Array(values)=>VectorImageBytes::Values(values),_=>return Err(invalid())};Ok(VectorSceneEvent::Image{at:point(required(rows,"at")?,budget)?,width:number(required(rows,"width")?)?,height:number(required(rows,"height")?)?,mime:text(required(rows,"mime")?)?,bytes})},_=>Err(invalid())
        }
    }
    /// 🎨️ Reborrows the last emitted style during bounded identifier comparison.
    pub fn current_style<'a>(&mut self,content:&'a DslValue,grant:VectorSceneGrant)->Result<VectorSceneStep<'a>,ValueError> {
        let mut budget=Budget{grant,receipt:Default::default()};let result=(||{if self.stage!=1||self.row==0{return Err(invalid())}let root=record(content,&["schema","canvas","styles","layers"],&mut budget)?;let styles=rows_or_empty(root,"styles")?;budget.visit(1)?;let rows=record(styles.get(self.row-1).ok_or_else(invalid)?,&["name","fill","stroke","strokeWidth","opacity"],&mut budget)?;let style=VectorStyleRef{name:text(required(rows,"name")?)?,fill:optional_color(rows,"fill",&mut budget)?,stroke:optional_color(rows,"stroke",&mut budget)?,stroke_width:optional_number(rows,"strokeWidth")?,opacity:optional_number(rows,"opacity")?.map(|value|value as f32)};if style.opacity.is_some_and(|value|!value.is_finite()){return Err(invalid())}Ok(VectorSceneEvent::Style(style))})();self.last=budget.receipt;result.map(|event|VectorSceneStep{event,receipt:self.last})
    }
    /// 🪙️ Quotes the only owned backing; borrowed descriptors carry no release obligation.
    pub fn next_release_bytes(&self)->usize {self.path.capacity()*std::mem::size_of::<usize>()}
    /// ♻️ Releases numeric ancestry only when the caller funds its actual backing.
    pub fn close_one(&mut self,maximum_bytes:usize)->bool {let bytes=self.next_release_bytes();if bytes==0||bytes>maximum_bytes{return false}self.path=Vec::new();self.segment=None;self.pending_finish=false;self.stage=5;self.last=VectorSceneReceipt{released_bytes:bytes,..Default::default()};true}
    /// 🫙️ Reports no remaining owned heap, including after cancellation before completion.
    pub fn terminal_is_empty(&self)->bool {self.path.capacity()==0}
}
