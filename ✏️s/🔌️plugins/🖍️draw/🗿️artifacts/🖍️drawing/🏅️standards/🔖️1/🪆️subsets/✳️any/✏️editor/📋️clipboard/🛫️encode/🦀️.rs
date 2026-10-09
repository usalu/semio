//! 🛫️ Clipboard JSON ordinals borrow original native fields without projecting a value tree.
use super::*;
use semio_framework_pack_json::{JsonWriteNode,JsonWriteSource};
use semio_framework_value::{ValueError,ValueRefusalKind,paged::PagedUtf8,list::PagedList};
fn absent()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Drawing clipboard JSON ordinal is absent")}
#[derive(Clone,Copy)]
enum View<'a>{Packet(&'a DrawingClipboard),Layer(&'a DrawingLayerNode),Transform(&'a crate::DrawingTransform),Attributes(&'a crate::DrawingAttributes),Fill(&'a crate::FillStyle),Stroke(&'a crate::StrokeStyle),Stop(&'a crate::GradientStop),Segment(&'a crate::PathSegment),Rect(&'a crate::DrawingRect),Ellipse(&'a crate::DrawingEllipse),Circle(&'a crate::DrawingCircle),Line(&'a crate::DrawingLine),Polygon(&'a crate::DrawingPolygon),Trace(&'a crate::DrawingTraceParams),Asset(&'a DrawingImageAsset),Layers(&'a[DrawingLayerNode]),Children(&'a PagedList<DrawingLayerNode,{usize::MAX}>),Segments(&'a PagedList<crate::PathSegment,{usize::MAX}>),Points(&'a PagedList<[f64;2],{usize::MAX}>),Stops(&'a PagedList<crate::GradientStop,{usize::MAX}>),Numbers(&'a PagedList<f64,{usize::MAX}>),Identities(&'a PagedList<PagedUtf8<{usize::MAX}>,{usize::MAX}>),Samples(&'a PagedList<[u8;4],{usize::MAX}>),Assets(&'a DrawingClipboardAssets),Strings(&'a[String]),Vector(&'a[f64]),Bytes(&'a[u8]),Native(&'a PagedUtf8<{usize::MAX}>),Text(&'a str),Number(f64),Unsigned(u64),Bool(bool)}
struct Fields<'a>{items:[Option<(&'static str,View<'a>)>;20],len:usize}
impl<'a> Fields<'a>{fn new()->Self{Self{items:[None;20],len:0}}fn push(&mut self,key:&'static str,value:View<'a>){self.items[self.len]=Some((key,value));self.len+=1;}fn at(&self,index:usize)->Result<(&'static str,View<'a>),ValueError>{self.items.get(index).copied().flatten().ok_or_else(absent)}}
macro_rules! fields{($($key:literal=>$value:expr),*$(,)?)=>{{let mut fields=Fields::new();$(fields.push($key,$value);)*fields}}}
impl<'a> View<'a>{
    fn fields(self)->Option<Fields<'a>>{use View::*;Some(match self{
        Packet(packet)=>fields!["schema"=>Text(&packet.schema),"roots"=>Layers(&packet.roots),"selected"=>Strings(&packet.selected),"assets"=>Assets(&packet.assets)],
        Layer(layer)=>{
            let base=layer_base(layer);let kind=match layer{DrawingLayerNode::Shape(_)=>"shape",DrawingLayerNode::Path(_)=>"path",DrawingLayerNode::Text(_)=>"text",DrawingLayerNode::Image(_)=>"image",DrawingLayerNode::Group(_)=>"group",DrawingLayerNode::Boolean(_)=>"boolean",DrawingLayerNode::Trace(_)=>"trace"};
            let mut fields=fields!["kind"=>Text(kind),"id"=>Native(&base.id),"name"=>Native(&base.name),"visible"=>Bool(base.visible),"locked"=>Bool(base.locked),"opacity"=>Number(base.opacity),"blendMode"=>Native(&base.blend_mode),"transform"=>Transform(&base.transform),"attributes"=>Attributes(&base.attributes)];
            match layer{
                DrawingLayerNode::Shape(value)=>{fields.push("shapeKind",Native(&value.shape_kind));if let Some(value)=&value.rect{fields.push("rect",Rect(value));}if let Some(value)=&value.ellipse{fields.push("ellipse",Ellipse(value));}if let Some(value)=&value.circle{fields.push("circle",Circle(value));}if let Some(value)=&value.line{fields.push("line",Line(value));}if let Some(value)=&value.polygon{fields.push("polygon",Polygon(value));}},
                DrawingLayerNode::Path(value)=>fields.push("segments",Segments(&value.segments)),
                DrawingLayerNode::Text(value)=>{fields.push("x",Number(value.x));fields.push("y",Number(value.y));fields.push("content",Native(&value.content));fields.push("size",Number(value.size));},
                DrawingLayerNode::Image(value)=>{fields.push("imageKey",Native(&value.image_key));fields.push("width",Number(value.width));fields.push("height",Number(value.height));},
                DrawingLayerNode::Group(value)=>{if value.isolation{fields.push("isolation",Bool(true));}fields.push("children",Children(&value.children));},
                DrawingLayerNode::Boolean(value)=>{fields.push("operation",Native(&value.operation));fields.push("children",Identities(&value.children));},
                DrawingLayerNode::Trace(value)=>{fields.push("sourceKey",Native(&value.source_key));fields.push("params",Trace(&value.params));}
            };fields
        },
        Transform(value)=>fields!["x"=>Number(value.x),"y"=>Number(value.y),"scaleX"=>Number(value.scale_x),"scaleY"=>Number(value.scale_y),"rotation"=>Number(value.rotation),"shear"=>Number(value.shear)],
        Attributes(value)=>{let mut fields=fields!["fillRule"=>Text(match value.fill_rule{crate::FillRule::Nonzero=>"nonzero",crate::FillRule::Evenodd=>"evenodd"})];if let Some(value)=&value.fill{fields.push("fill",Fill(value));}if let Some(value)=&value.stroke{fields.push("stroke",Stroke(value));}fields},
        Fill(value)=>match value{
            crate::FillStyle::Solid{color}=>fields!["kind"=>Text("solid"),"color"=>Vector(color)],
            crate::FillStyle::LinearGradient{x1,y1,x2,y2,stops}=>fields!["kind"=>Text("linearGradient"),"x1"=>Number(*x1),"y1"=>Number(*y1),"x2"=>Number(*x2),"y2"=>Number(*y2),"stops"=>Stops(stops)],
            crate::FillStyle::RadialGradient{cx,cy,r,stops}=>fields!["kind"=>Text("radialGradient"),"cx"=>Number(*cx),"cy"=>Number(*cy),"r"=>Number(*r),"stops"=>Stops(stops)]},
        Stroke(value)=>{let mut fields=fields!["color"=>Vector(&value.color),"width"=>Number(value.width),"cap"=>Text(match value.cap{crate::StrokeCap::Butt=>"butt",crate::StrokeCap::Round=>"round",crate::StrokeCap::Square=>"square"}),"join"=>Text(match value.join{crate::StrokeJoin::Miter=>"miter",crate::StrokeJoin::Round=>"round",crate::StrokeJoin::Bevel=>"bevel"})];if let Some(value)=&value.dash{fields.push("dash",Numbers(value));}fields},
        Stop(value)=>fields!["offset"=>Number(value.offset),"color"=>Vector(&value.color)],
        Segment(value)=>match value{
            crate::PathSegment::Move{to}=>fields!["kind"=>Text("move"),"to"=>Vector(to)],
            crate::PathSegment::Line{to}=>fields!["kind"=>Text("line"),"to"=>Vector(to)],
            crate::PathSegment::Quad{ctrl,to}=>fields!["kind"=>Text("quad"),"ctrl"=>Vector(ctrl),"to"=>Vector(to)],
            crate::PathSegment::Cubic{ctrl1,ctrl2,to}=>fields!["kind"=>Text("cubic"),"ctrl1"=>Vector(ctrl1),"ctrl2"=>Vector(ctrl2),"to"=>Vector(to)],
            crate::PathSegment::Arc{rx,ry,rotation,large_arc,sweep,to}=>fields!["kind"=>Text("arc"),"rx"=>Number(*rx),"ry"=>Number(*ry),"rotation"=>Number(*rotation),"largeArc"=>Bool(*large_arc),"sweep"=>Bool(*sweep),"to"=>Vector(to)],
            crate::PathSegment::Close=>fields!["kind"=>Text("close")]},
        Rect(value)=>fields!["x"=>Number(value.x),"y"=>Number(value.y),"width"=>Number(value.width),"height"=>Number(value.height)],
        Ellipse(value)=>fields!["cx"=>Number(value.cx),"cy"=>Number(value.cy),"rx"=>Number(value.rx),"ry"=>Number(value.ry)],
        Circle(value)=>fields!["cx"=>Number(value.cx),"cy"=>Number(value.cy),"r"=>Number(value.r)],
        Line(value)=>fields!["x1"=>Number(value.x1),"y1"=>Number(value.y1),"x2"=>Number(value.x2),"y2"=>Number(value.y2)],
        Polygon(value)=>fields!["points"=>Points(&value.points)],
        Trace(value)=>fields!["threshold"=>Number(value.threshold),"simplifyEpsilon"=>Number(value.simplify_epsilon)],
        Asset(value)=>fields!["width"=>Unsigned(u64::from(value.width)),"height"=>Unsigned(u64::from(value.height)),"samples"=>Samples(&value.samples)],
        _=>return None
    })}
    fn node(self)->Result<JsonWriteNode<'a>,ValueError>{use View::*;Ok(match self{
        Text(value)=>JsonWriteNode::String(value),Native(value)=>JsonWriteNode::NativeString(value),Number(value)=>{if !value.is_finite(){return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Drawing clipboard number is not finite"));}JsonWriteNode::Number(semio_framework_value::Number::Float(value))},Unsigned(value)=>JsonWriteNode::Number(semio_framework_value::Number::UInt(value)),Bool(value)=>JsonWriteNode::Bool(value),
        Layers(value)=>JsonWriteNode::Array(value.len()),Children(value)=>JsonWriteNode::Array(value.len()),Segments(value)=>JsonWriteNode::Array(value.len()),Points(value)=>JsonWriteNode::Array(value.len()),Stops(value)=>JsonWriteNode::Array(value.len()),Numbers(value)=>JsonWriteNode::Array(value.len()),Identities(value)=>JsonWriteNode::Array(value.len()),Samples(value)=>JsonWriteNode::Array(value.len()),Strings(value)=>JsonWriteNode::Array(value.len()),Vector(value)=>JsonWriteNode::Array(value.len()),Bytes(value)=>JsonWriteNode::Array(value.len()),Assets(value)=>JsonWriteNode::Object(value.len()),_=>JsonWriteNode::Object(self.fields().ok_or_else(absent)?.len)
    })}
    fn child(self,index:usize)->Result<Self,ValueError>{use View::*;Ok(match self{
        Layers(value)=>Layer(value.get(index).ok_or_else(absent)?),Children(value)=>Layer(value.get(index).ok_or_else(absent)?),Segments(value)=>Segment(value.get(index).ok_or_else(absent)?),Points(value)=>Vector(value.get(index).ok_or_else(absent)?),Stops(value)=>Stop(value.get(index).ok_or_else(absent)?),Numbers(value)=>Number(*value.get(index).ok_or_else(absent)?),Identities(value)=>Native(value.get(index).ok_or_else(absent)?),Samples(value)=>Bytes(value.get(index).ok_or_else(absent)?),Strings(value)=>Text(value.get(index).ok_or_else(absent)?),Vector(value)=>Number(*value.get(index).ok_or_else(absent)?),Bytes(value)=>Unsigned(u64::from(*value.get(index).ok_or_else(absent)?)),Assets(value)=>Asset(value.entry_at(index).ok_or_else(absent)?.1),_=>self.fields().ok_or_else(absent)?.at(index)?.1
    })}
    fn key(self,index:usize)->Result<&'a str,ValueError>{if let Self::Assets(value)=self{return value.entry_at(index).map(|(key,_)|key.as_str()).ok_or_else(absent);}Ok(self.fields().ok_or_else(absent)?.at(index)?.0)}
}
#[derive(semio_framework_value::RetireOwned)]
pub(super) struct ClipboardJsonSource{pub(super) packet:DrawingClipboard}
impl ClipboardJsonSource{fn at(&self,path:&[usize])->Result<View<'_>,ValueError>{let mut value=View::Packet(&self.packet);for index in path{value=value.child(*index)?;}Ok(value)}}
impl JsonWriteSource for ClipboardJsonSource{
    fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{self.at(path)?.node()}
    fn object_key_at_path(&self,path:&[usize],index:usize)->Result<&str,ValueError>{self.at(path)?.key(index)}
}
