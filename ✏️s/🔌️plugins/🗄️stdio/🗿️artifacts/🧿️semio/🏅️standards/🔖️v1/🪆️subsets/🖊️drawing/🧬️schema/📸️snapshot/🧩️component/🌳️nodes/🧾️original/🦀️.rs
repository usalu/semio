//! 🌳️ Borrowed Drawing fields materialize only in the caller-returned original typed root.
use crate::standards::v1::subsets::drawing::schema::snapshot::{DrawNode,PathSegment,DrawLayer,DrawStyle,DrawCanvas,SemioDrawingSnapshot};
use crate::standards::v1::subsets::base::schema::geometry::{SemioPoint2,SemioPoint3,SemioQuaternion,SemioTransform,SemioRgba};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_decoding::{workspace,text_into};
use semio_framework_value::{DslValue,FromValue,NativeDecodeControl,ValueError,ValueRefusalKind};

fn refusal(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
fn object<'v>(value:&'v DslValue,allowed:&[&str],control:&mut NativeDecodeControl<'_>)->Result<&'v[(String,DslValue)],ValueError>{
 control.checkpoint()?;let DslValue::Object(fields)=value else{return Err(refusal("Drawing value must be an object"))};
 if fields.len()>allowed.len(){return Err(refusal("Drawing object has excess fields"))}
 for (index,(key,_)) in fields.iter().enumerate(){if !allowed.contains(&key.as_str())||fields[..index].iter().any(|(previous,_)|previous==key){return Err(refusal("Drawing object has unknown or duplicate fields"))}control.step()?;}
 Ok(fields)
}
fn field<'v>(fields:&'v[(String,DslValue)],key:&str)->Option<&'v DslValue>{fields.iter().find(|(name,_)|name==key).map(|(_,value)|value)}
fn required<'v>(fields:&'v[(String,DslValue)],key:&str)->Result<&'v DslValue,ValueError>{field(fields,key).ok_or_else(||refusal("Drawing required field is missing"))}
fn text(value:&DslValue)->Result<&str,ValueError>{if let DslValue::String(value)=value{Ok(value)}else{Err(refusal("Drawing text field must be text"))}}
fn array(value:&DslValue)->Result<&[DslValue],ValueError>{if let DslValue::Array(value)=value{Ok(value)}else{Err(refusal("Drawing collection must be an array"))}}
fn number(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<f64,ValueError>{if !matches!(value,DslValue::Number(_)){return Err(refusal("Drawing number field must be numeric"))}f64::from_value_controlled(value,control)}
fn boolean(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<bool,ValueError>{if !matches!(value,DslValue::Bool(_)){return Err(refusal("Drawing flag must be boolean"))}bool::from_value_controlled(value,control)}
fn point2(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<SemioPoint2,ValueError>{let fields=object(value,&["x","y"],control)?;Ok(SemioPoint2{x:number(required(fields,"x")?,control)?,y:number(required(fields,"y")?,control)?})}
fn point3(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<SemioPoint3,ValueError>{let fields=object(value,&["x","y","z"],control)?;Ok(SemioPoint3{x:number(required(fields,"x")?,control)?,y:number(required(fields,"y")?,control)?,z:number(required(fields,"z")?,control)?})}
fn transform(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<SemioTransform,ValueError>{
 let fields=object(value,&["translation","rotation","scale"],control)?;let translation=point3(required(fields,"translation")?,control)?;let rotation=object(required(fields,"rotation")?,&["x","y","z","w"],control)?;
 let rotation=SemioQuaternion{x:number(required(rotation,"x")?,control)?,y:number(required(rotation,"y")?,control)?,z:number(required(rotation,"z")?,control)?,w:number(required(rotation,"w")?,control)?};let scale=point3(required(fields,"scale")?,control)?;Ok(SemioTransform{translation,rotation,scale})
}
fn color(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<SemioRgba,ValueError>{let fields=object(value,&["r","g","b","a"],control)?;let mut scalar=|name|{let value=required(fields,name)?;if !matches!(value,DslValue::Number(_)){return Err(refusal("Drawing color field must be numeric"))}f32::from_value_controlled(value,control)};Ok(SemioRgba{r:scalar("r")?,g:scalar("g")?,b:scalar("b")?,a:scalar("a")?})}
fn optional_text(output:&mut Option<String>,value:Option<&DslValue>,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{if let Some(value)=value.filter(|v|!matches!(v,DslValue::Null)){*output=Some(String::new());text_into(output.as_mut().unwrap(),text(value)?,control)?;}Ok(())}
fn segment(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<PathSegment,ValueError>{
 let DslValue::Object(fields)=value else{return Err(refusal("Drawing segment must be an object"))};if fields.len()>7{return Err(refusal("Drawing segment has excess fields"))}let kind=text(required(fields,"kind")?)?;
 let allowed:&[&str]=match kind{"moveTo"|"lineTo"=>&["kind","to"],"cubicTo"=>&["kind","c1","c2","to"],"quadTo"=>&["kind","c","to"],"arcTo"=>&["kind","rx","ry","xRotation","largeArc","sweep","to"],"close"=>&["kind"],_=>return Err(refusal("unknown Drawing path segment"))};let fields=object(value,allowed,control)?;
 Ok(match kind{"moveTo"=>PathSegment::MoveTo{to:point2(required(fields,"to")?,control)?},"lineTo"=>PathSegment::LineTo{to:point2(required(fields,"to")?,control)?},"cubicTo"=>PathSegment::CubicTo{c1:point2(required(fields,"c1")?,control)?,c2:point2(required(fields,"c2")?,control)?,to:point2(required(fields,"to")?,control)?},"quadTo"=>PathSegment::QuadTo{c:point2(required(fields,"c")?,control)?,to:point2(required(fields,"to")?,control)?},"arcTo"=>PathSegment::ArcTo{rx:number(required(fields,"rx")?,control)?,ry:number(required(fields,"ry")?,control)?,x_rotation:number(required(fields,"xRotation")?,control)?,large_arc:boolean(required(fields,"largeArc")?,control)?,sweep:boolean(required(fields,"sweep")?,control)?,to:point2(required(fields,"to")?,control)?},_=>PathSegment::Close})
}
/// 🪴️ Creates no backing allocation before the caller has admitted the original workspace.
pub(crate)fn empty_node()->DrawNode{DrawNode::Group{transform:SemioTransform::identity(),children:Vec::new()}}
/// 🧩️ Fills one already retained zero-heap node, retaining every sibling and partial field before checkpoints.
pub(crate)fn node_into(output:&mut DrawNode,value:&DslValue,control:&mut NativeDecodeControl<'_>,depth:usize)->Result<(),ValueError>{
 if depth>=64{return Err(ValueError::literal(ValueRefusalKind::DepthLimit,"Drawing node exceeds typed depth limit"))}
 if !matches!(output,DrawNode::Group{children,..}if children.capacity()==0){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"Drawing node target is already owned"))}
 control.checkpoint()?;let DslValue::Object(fields)=value else{return Err(refusal("Drawing node must be an object"))};if fields.len()>7{return Err(refusal("Drawing node has excess fields"))}let kind=text(required(fields,"kind")?)?;
 let allowed:&[&str]=match kind{"path"=>&["kind","segments","style"],"text"=>&["kind","value","at","style"],"image"=>&["kind","at","width","height","mime","bytes"],"group"=>&["kind","transform","children"],_=>return Err(refusal("unknown Drawing node kind"))};let fields=object(value,allowed,control)?;
 match kind{
  "path"=>{*output=DrawNode::Path{segments:Vec::new(),style:None};let DrawNode::Path{segments,style}=output else{unreachable!()};let values=array(required(fields,"segments")?)?;*segments=control.allocate_vec(values.len())?;for value in values{segments.push(segment(value,control)?);control.checkpoint()?;}optional_text(style,field(fields,"style"),control)?;},
  "text"=>{*output=DrawNode::Text{value:String::new(),at:SemioPoint2::default(),style:None};let DrawNode::Text{value,at,style}=output else{unreachable!()};text_into(value,text(required(fields,"value")?)?,control)?;*at=point2(required(fields,"at")?,control)?;optional_text(style,field(fields,"style"),control)?;},
  "image"=>{*output=DrawNode::Image{at:SemioPoint2::default(),width:0.0,height:0.0,mime:String::new(),bytes:Vec::new()};let DrawNode::Image{at,width,height,mime,bytes}=output else{unreachable!()};*at=point2(required(fields,"at")?,control)?;*width=number(required(fields,"width")?,control)?;*height=number(required(fields,"height")?,control)?;text_into(mime,text(required(fields,"mime")?)?,control)?;let values=array(required(fields,"bytes")?)?;*bytes=control.allocate_vec(values.len())?;for value in values{let value=number(value,control)?;if !value.is_finite()||value.fract()!=0.0||!(0.0..=255.0).contains(&value){return Err(refusal("Drawing image octet is outside its exact range"))}bytes.push(value as u8);control.step()?;}},
  "group"=>{*output=empty_node();let DrawNode::Group{transform:original_transform,children}=output else{unreachable!()};*original_transform=transform(required(fields,"transform")?,control)?;let values=field(fields,"children").map(array).transpose()?.unwrap_or(&[]);*children=control.allocate_vec(values.len())?;for value in values{children.push(empty_node());node_into(children.last_mut().unwrap(),value,control,depth+1)?;control.checkpoint()?;}},
  _=>unreachable!(),
 }
 control.checkpoint()
}
/// 🫴️ Returns the actual complete or partial original node through the explicit recipient.
pub fn decode_node(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<DrawNode,ValueError>{workspace::decode(control,empty_node,|output,control|node_into(output,value,control,0))}
fn empty_style()->DrawStyle{DrawStyle{name:String::new(),fill:None,stroke:None,stroke_width:None,opacity:None}}
fn style_into(output:&mut DrawStyle,value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
 let fields=object(value,&["name","fill","stroke","strokeWidth","opacity"],control)?;text_into(&mut output.name,text(required(fields,"name")?)?,control)?;
 output.fill=field(fields,"fill").filter(|v|!matches!(v,DslValue::Null)).map(|v|color(v,control)).transpose()?;output.stroke=field(fields,"stroke").filter(|v|!matches!(v,DslValue::Null)).map(|v|color(v,control)).transpose()?;output.stroke_width=field(fields,"strokeWidth").filter(|v|!matches!(v,DslValue::Null)).map(|v|number(v,control)).transpose()?;
 output.opacity=field(fields,"opacity").filter(|v|!matches!(v,DslValue::Null)).map(|v|{if !matches!(v,DslValue::Number(_)){return Err(refusal("Drawing opacity must be numeric"))}f32::from_value_controlled(v,control)}).transpose()?;control.checkpoint()
}
/// 🎨️ Retains the original named style before any field is materialized.
pub fn decode_style(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<DrawStyle,ValueError>{workspace::decode(control,empty_style,|output,control|style_into(output,value,control))}
fn empty_layer()->DrawLayer{DrawLayer{id:String::new(),name:String::new(),visible:false,root:empty_node()}}
fn layer_into(output:&mut DrawLayer,value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{let fields=object(value,&["id","name","visible","root"],control)?;text_into(&mut output.id,text(required(fields,"id")?)?,control)?;text_into(&mut output.name,text(required(fields,"name")?)?,control)?;output.visible=boolean(required(fields,"visible")?,control)?;node_into(&mut output.root,required(fields,"root")?,control,0)}
/// 🗂️ Retains layer metadata and its original recursive root in one workspace.
pub fn decode_layer(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<DrawLayer,ValueError>{workspace::decode(control,empty_layer,|output,control|layer_into(output,value,control))}
fn canvas(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<DrawCanvas,ValueError>{let fields=object(value,&["width","height","background"],control)?;Ok(DrawCanvas{width:number(required(fields,"width")?,control)?,height:number(required(fields,"height")?,control)?,background:field(fields,"background").filter(|v|!matches!(v,DslValue::Null)).map(|v|color(v,control)).transpose()?})}
/// 📄️ Initializes the actual drawing root without allocating its default schema string.
pub(crate)fn empty_snapshot()->SemioDrawingSnapshot{SemioDrawingSnapshot{schema:String::new(),canvas:DrawCanvas::default(),styles:Vec::new(),layers:Vec::new()}}
fn snapshot_into(output:&mut SemioDrawingSnapshot,value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
 let fields=object(value,&["schema","canvas","styles","layers"],control)?;text_into(&mut output.schema,text(required(fields,"schema")?)?,control)?;output.canvas=canvas(required(fields,"canvas")?,control)?;
 let values=field(fields,"styles").map(array).transpose()?.unwrap_or(&[]);output.styles=control.allocate_vec(values.len())?;for value in values{output.styles.push(empty_style());style_into(output.styles.last_mut().unwrap(),value,control)?;}
 let values=field(fields,"layers").map(array).transpose()?.unwrap_or(&[]);output.layers=control.allocate_vec(values.len())?;for value in values{output.layers.push(empty_layer());layer_into(output.layers.last_mut().unwrap(),value,control)?;}control.checkpoint()
}
/// 🖊️ Preserves the whole original document and every unfinished nested field through one explicit return slot.
pub fn decode_snapshot(value:&DslValue,control:&mut NativeDecodeControl<'_>)->Result<SemioDrawingSnapshot,ValueError>{workspace::decode(control,empty_snapshot,|output,control|snapshot_into(output,value,control))}
