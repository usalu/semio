//! 📄️ Incremental SVG hierarchy construction with retained progress and cancellation.
use std::collections::BTreeMap;
use crate::{DrawingSnapshot,DrawingLayerNode,DrawingLayerBase,DrawingGroupBody,DrawingPathBody,DrawingTextBody,DrawingAttributes,FillStyle,FillRule,StrokeStyle,StrokeCap,StrokeJoin,PathSegment};
use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::schema::snapshot::{SvgNode,SvgAttr,SvgAttributeValue};
use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::io::text::snapshot::parse_svg_xml;
use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::io::text::snapshot::attributes::bind_svg_attribute;
type Style=BTreeMap<String,SvgAttributeValue>;
const INHERITED:&[&str]=&["fill","fill-rule","fill-opacity","stroke","stroke-width","stroke-opacity","stroke-linecap","stroke-linejoin","stroke-dasharray","visibility","font-size","font-family","color"];
const UNSUPPORTED:&[&str]=&["clip-path","mask","filter","vector-effect","stroke-dashoffset","font-weight","font-style","text-anchor"];
fn attr<'a>(attrs:&'a [SvgAttr],key:&str)->Option<&'a SvgAttributeValue> {attrs.iter().find(|item|item.name==key).map(|item|&item.value)}
fn text_attr<'a>(attrs:&'a [SvgAttr],key:&str)->Option<&'a str> {attr(attrs,key).and_then(SvgAttributeValue::text)}
fn local(name:&str)->&str {name.rsplit(':').next().unwrap_or(name)}
fn scalar(value:&str)->Result<f64,String> {let value=value.trim().parse::<f64>().map_err(|_|"Invalid SVG number".to_owned())?;if value.is_finite(){Ok(value)}else{Err("Nonfinite SVG number".into())}}
fn length(value:Option<&SvgAttributeValue>,fallback:f64)->Result<f64,String> {
    let Some(value)=value else{return Ok(fallback)};
    if let SvgAttributeValue::Length(value)=value {
        let factor=match value.unit.as_str(){""|"px"=>1.0,"pt"=>96.0/72.0,"pc"=>16.0,"mm"=>96.0/25.4,"cm"=>96.0/2.54,"in"=>96.0,_=>return Err("Unsupported SVG length unit".into())};
        let result=value.magnitude*factor;return if result.is_finite(){Ok(result)}else{Err("Nonfinite SVG dimension".into())};
    }
    let SvgAttributeValue::Text(value)=value else{return Err("SVG dimension requires length".into())};text_length(value)
}
fn text_length(value:&str)->Result<f64,String> {
    let value=value.trim();
    for (unit,factor) in [("px",1.0),("pt",96.0/72.0),("pc",16.0),("mm",96.0/25.4),("cm",96.0/2.54),("in",96.0)] {if let Some(number)=value.strip_suffix(unit){return scalar(number).map(|number|number*factor);}}
    scalar(value)
}
fn nonnegative(value:f64)->Result<f64,String> {if value.is_finite()&&value>=0.0 {Ok(value)}else{Err("Invalid SVG dimension".into())}}
fn fraction(value:Option<&SvgAttributeValue>)->Result<f64,String> {let number=match value {None=>1.0,Some(SvgAttributeValue::Number(value)) if value.is_finite()=>*value,Some(SvgAttributeValue::Text(value))=>scalar(value)?,_=>return Err("SVG opacity requires finite number".into())};Ok(number.clamp(0.0,1.0))}
fn color(source:&str,alpha:f64)->Result<[f64;4],String> {
    if source=="transparent" {return Ok([0.0;4]);}
    let lower=source.to_ascii_lowercase();
    let named=match lower.as_str(){"black"=>"000000","white"=>"ffffff","red"=>"ff0000","green"=>"008000","blue"=>"0000ff","yellow"=>"ffff00","gray"|"grey"=>"808080","silver"=>"c0c0c0","maroon"=>"800000","purple"=>"800080","fuchsia"=>"ff00ff","lime"=>"00ff00","olive"=>"808000","navy"=>"000080","teal"=>"008080","aqua"=>"00ffff","orange"=>"ffa500",_=>""};
    let hex=source.strip_prefix('#').unwrap_or(named);
    if [3,4,6,8].contains(&hex.len())&&hex.bytes().all(|byte|byte.is_ascii_hexdigit()) {
        let full=if hex.len()<5 {hex.chars().flat_map(|ch|[ch,ch]).collect::<String>()}else{hex.to_owned()};
        let channel=|offset|u8::from_str_radix(&full[offset..offset+2],16).unwrap() as f64/255.0;
        return Ok([channel(0),channel(2),channel(4),alpha*if full.len()==8{channel(6)}else{1.0}]);
    }
    if let Some(values)=source.strip_prefix("rgb(").and_then(|value|value.strip_suffix(')')) {
        let values=values.split(',').map(|value|{let value=value.trim();if let Some(number)=value.strip_suffix('%'){scalar(number).map(|number|(number/100.0).clamp(0.0,1.0))}else{scalar(value).map(|number|(number/255.0).clamp(0.0,1.0))}}).collect::<Result<Vec<_>,_>>()?;
        if values.len()==3{return Ok([values[0],values[1],values[2],alpha]);}
    }
    Err(format!("Unsupported SVG paint: {source}"))
}
fn properties(attrs:&[SvgAttr],parent:&Style)->Result<Style,String> {
    let mut style:Style=parent.iter().filter(|(key,_)|INHERITED.contains(&key.as_str())).map(|(key,value)|(key.clone(),value.clone())).collect();
    let supported=|key:&str|INHERITED.contains(&key)||UNSUPPORTED.contains(&key)||["opacity","display","mix-blend-mode","isolation"].contains(&key);
    for item in attrs {if supported(&item.name){style.insert(item.name.clone(),item.value.clone());}}
    for declaration in text_attr(attrs,"style").unwrap_or("").split(';').filter(|value|!value.trim().is_empty()) {
        let (key,value)=declaration.split_once(':').ok_or("Invalid SVG style declaration")?;let key=key.trim();
        if !supported(key){return Err(format!("Unsupported SVG style: {key}"));}let value=value.trim();style.insert(key.into(),if value=="inherit"{SvgAttributeValue::Text(value.into())}else{bind_svg_attribute(key,value)?});
    }
    for key in style.keys().cloned().collect::<Vec<_>>() {if style.get(&key).is_some_and(|value|value.text()==Some("inherit")){if let Some(value)=parent.get(&key){style.insert(key,value.clone());}else{style.remove(&key);}}}
    for key in UNSUPPORTED {if style.get(*key).is_some_and(|value|value.text()!=Some("none")){return Err(format!("Unsupported SVG property: {key}"));}}
    Ok(style)
}
fn blend_mode(value:Option<&str>)->Result<String,String> {
    Ok(match value.unwrap_or("normal") {
        "color-dodge"=>"colorDodge","color-burn"=>"colorBurn","hard-light"=>"hardLight","soft-light"=>"softLight",
        mode @ ("normal"|"multiply"|"screen"|"overlay"|"darken"|"lighten"|"difference"|"exclusion"|"hue"|"saturation"|"color"|"luminosity")=>mode,
        mode=>return Err(format!("Unsupported SVG blend mode: {mode}")),
    }.into())
}
fn isolation(value:Option<&str>)->Result<bool,String> {
    if !matches!(value,None|Some("auto"|"isolate")){return Err("Invalid SVG isolation".into());}
    Ok(value==Some("isolate"))
}
fn attributes(style:&Style)->Result<DrawingAttributes,String> {
    let field=|key:&str|style.get(key);let value=|key:&str|field(key).and_then(SvgAttributeValue::text);
    let resolve=|paint:&str|if paint=="currentColor"{value("color").unwrap_or("black").to_owned()}else{paint.to_owned()};
    let fill_rule=FillRule::parse(value("fill-rule").unwrap_or("nonzero")).map_err(str::to_owned)?;
    let fill=match value("fill").unwrap_or("black") {"none"=>None,paint=>Some(FillStyle::Solid {color:color(&resolve(paint),fraction(field("fill-opacity"))?)?})};
    let stroke=match value("stroke") {None|Some("none")=>None,Some(paint)=>{
        let dash=match value("stroke-dasharray"){None|Some("none")=>None,Some(source)=>Some(source.split(|ch:char|ch==','||ch.is_ascii_whitespace()).filter(|value|!value.is_empty()).map(|value|nonnegative(text_length(value)?)).collect::<Result<semio_framework_value::list::PagedList<f64,{usize::MAX}>,_>>()?)};
        Some(StrokeStyle {color:color(&resolve(paint),fraction(field("stroke-opacity"))?)?,width:nonnegative(length(field("stroke-width"),1.0)?)?,cap:StrokeCap::parse(value("stroke-linecap").unwrap_or("butt")).map_err(str::to_owned)?,join:StrokeJoin::parse(value("stroke-linejoin").unwrap_or("miter")).map_err(str::to_owned)?,dash})
    }};
    Ok(DrawingAttributes {fill_rule,fill,stroke})
}
fn geometry(tag:&str,attrs:&[SvgAttr])->Result<Vec<PathSegment>,String> {
    let n=|key|length(attr(attrs,key),0.0);let m=|x,y|PathSegment::Move {to:[x,y]};let l=|x,y|PathSegment::Line {to:[x,y]};
    match tag {
        "path"=>match attr(attrs,"d"){None=>Ok(Vec::new()),Some(SvgAttributeValue::PathData(commands))=>super::path::editable_svg_path_commands(commands),_=>Err("SVG path requires decoded commands".into())},
        "line"=>Ok(vec![m(n("x1")?,n("y1")?),l(n("x2")?,n("y2")?)]),
        "rect"=>{
            let (x,y,w,h)=(n("x")?,n("y")?,nonnegative(n("width")?)?,nonnegative(n("height")?)?);
            if w==0.0||h==0.0{return Ok(Vec::new());}
            let rx=nonnegative(length(attr(attrs,"rx"),n("ry")?)?)?.min(w/2.0);let ry=nonnegative(length(attr(attrs,"ry"),n("rx")?)?)?.min(h/2.0);
            if rx==0.0||ry==0.0{return Ok(vec![m(x,y),l(x+w,y),l(x+w,y+h),l(x,y+h),PathSegment::Close]);}
            let a=|x,y|PathSegment::Arc {rx,ry,rotation:0.0,large_arc:false,sweep:true,to:[x,y]};
            Ok(vec![m(x+rx,y),l(x+w-rx,y),a(x+w,y+ry),l(x+w,y+h-ry),a(x+w-rx,y+h),l(x+rx,y+h),a(x,y+h-ry),l(x,y+ry),a(x+rx,y),PathSegment::Close])
        },
        "circle"|"ellipse"=>{
            let (x,y)=(n("cx")?,n("cy")?);let rx=nonnegative(n(if tag=="circle"{"r"}else{"rx"})?)?;let ry=nonnegative(n(if tag=="circle"{"r"}else{"ry"})?)?;
            if rx==0.0||ry==0.0{return Ok(Vec::new());}
            let a=|x|PathSegment::Arc {rx,ry,rotation:0.0,large_arc:false,sweep:true,to:[x,y]};Ok(vec![m(x+rx,y),a(x-rx),a(x+rx),PathSegment::Close])
        },
        "polyline"|"polygon"=>{
            let points=match attr(attrs,"points"){None=>&[][..],Some(SvgAttributeValue::Points(points))=>points.as_slice(),_=>return Err("SVG polygon requires decoded points".into())};
            if points.iter().any(|point|!point.x.is_finite()||!point.y.is_finite()){return Err("Nonfinite SVG points".into());}
            let mut segments=points.iter().enumerate().map(|(index,point)|if index==0{m(point.x,point.y)}else{l(point.x,point.y)}).collect::<Vec<_>>();
            if tag=="polygon"&&!segments.is_empty(){segments.push(PathSegment::Close);}Ok(segments)
        },
        _=>Err(format!("Unsupported SVG element: {tag}"))
    }
}
fn viewport(attrs:&[SvgAttr])->Result<(f64,f64,[f64;6]),String> {
    let bounds=match attr(attrs,"viewBox"){None=>None,Some(SvgAttributeValue::ViewBox(value))=>Some([value.min_x,value.min_y,value.width,value.height]),_=>return Err("SVG viewport requires decoded view box".into())};
    if bounds.as_ref().is_some_and(|value|value.len()!=4||value[2]<=0.0||value[3]<=0.0){return Err("SVG viewBox needs positive width and height".into());}
    let width=nonnegative(length(attr(attrs,"width"),bounds.as_ref().map_or(300.0,|v|v[2]))?)?;
    let height=nonnegative(length(attr(attrs,"height"),bounds.as_ref().map_or(150.0,|v|v[3]))?)?;
    let Some(bounds)=bounds else{return Ok((width,height,[1.0,0.0,0.0,1.0,0.0,0.0]));};
    let (mut sx,mut sy,mut dx,mut dy)=(width/bounds[2],height/bounds[3],0.0,0.0);
    let parts=text_attr(attrs,"preserveAspectRatio").unwrap_or("xMidYMid meet").split_ascii_whitespace().collect::<Vec<_>>();
    let align=parts.first().copied().unwrap_or("xMidYMid");let mode=parts.get(1).copied().unwrap_or("meet");
    if parts.len()>2||!["meet","slice"].contains(&mode){return Err("Invalid SVG aspect ratio".into());}
    if align!="none" {
        let valid=["xMinYMin","xMidYMin","xMaxYMin","xMinYMid","xMidYMid","xMaxYMid","xMinYMax","xMidYMax","xMaxYMax"];
        if !valid.contains(&align){return Err("Invalid SVG aspect ratio".into());}
        sx=if mode=="slice"{sx.max(sy)}else{sx.min(sy)};sy=sx;
        let part=|value|match value{"Min"=>0.0,"Mid"=>0.5,_=>1.0};dx=(width-bounds[2]*sx)*part(&align[1..4]);dy=(height-bounds[3]*sy)*part(&align[5..]);
    }
    Ok((width,height,[sx,0.0,0.0,sy,dx-bounds[0]*sx,dy-bounds[1]*sy]))
}
fn gradient_fill(reference:&str,definitions:&BTreeMap<String,SvgNode>,segments:&[PathSegment],viewport:[f64;2],alpha:f64)->Result<Option<FillStyle>,String> {
    let id=reference.strip_prefix("url(").and_then(|value|value.strip_suffix(')')).map(str::trim).map(|value|value.trim_matches(['\"','\''])).and_then(|value|value.strip_prefix('#')).ok_or("SVG paint needs a local gradient reference")?;
    let mut current=id.to_owned();let mut seen=std::collections::BTreeSet::new();let mut values=Style::new();let mut source_stops=Vec::new();
    let Some(SvgNode::Element {name,..})=definitions.get(id) else{return Err("Missing SVG gradient".into());};
    if local(name)!="linearGradient"{return Err("Radial SVG gradient import requires an authored gradient coordinate system".into());}
    loop {
        if !seen.insert(current.clone()){return Err("Cyclic SVG gradient reference".into());}
        let Some(SvgNode::Element {attrs,children,..})=definitions.get(&current) else{return Err("Missing SVG gradient reference".into());};
        let mut interpolation=text_attr(attrs,"color-interpolation").unwrap_or("sRGB");
        for declaration in text_attr(attrs,"style").unwrap_or("").split(';'){if let Some((key,value))=declaration.split_once(':'){if key.trim()=="color-interpolation"{interpolation=value.trim();}}}
        if !["sRGB","auto"].contains(&interpolation){return Err("Unsupported SVG gradient color interpolation".into());}
        for item in attrs {values.entry(item.name.clone()).or_insert_with(||item.value.clone());}
        if source_stops.is_empty(){source_stops=children.iter().filter_map(|node|match node{SvgNode::Element {name,attrs,..} if local(name)=="stop"=>Some(attrs.as_slice()),_=>None}).collect();}
        let Some(href)=text_attr(attrs,"href").or_else(||text_attr(attrs,"xlink:href")) else{break;};
        current=href.strip_prefix('#').ok_or("SVG gradient references must be local")?.to_owned();
    }
    if values.get("spreadMethod").is_some_and(|value|value.text()!=Some("pad")){return Err("Unsupported SVG gradient spread".into());}
    let mut stops=semio_framework_value::list::PagedList::new();let mut last:f64=0.0;
    for attrs in source_stops {
        let mut style=Style::new();for declaration in text_attr(attrs,"style").unwrap_or("").split(';'){if let Some((key,value))=declaration.split_once(':'){style.insert(key.trim().into(),bind_svg_attribute(key.trim(),value.trim())?);}}
        let source=text_attr(attrs,"offset").unwrap_or("0");let offset=if let Some(value)=source.strip_suffix('%'){scalar(value)?/100.0}else{scalar(source)?};last=last.max(offset.clamp(0.0,1.0));
        let paint=style.get("stop-color").and_then(SvgAttributeValue::text).or_else(||text_attr(attrs,"stop-color")).unwrap_or("black");
        let opacity=style.get("stop-opacity").or_else(||attr(attrs,"stop-opacity"));
        stops.push(crate::GradientStop {offset:last,color:color(paint,alpha*fraction(opacity)?)?});
    }
    if stops.is_empty(){return Ok(None);}if stops.len()==1{return Ok(Some(FillStyle::Solid {color:stops[0].color}));}
    let unit=values.get("gradientUnits").and_then(SvgAttributeValue::text).unwrap_or("objectBoundingBox");
    if !["objectBoundingBox","userSpaceOnUse"].contains(&unit){return Err("Invalid SVG gradient units".into());}
    let (mut current,mut start,mut min,mut max)=([0.0;2],[0.0;2],[f64::INFINITY;2],[f64::NEG_INFINITY;2]);
    for segment in segments {
        if let PathSegment::Move {to}=segment {current=*to;start=*to;continue;}
        let bounds=crate::schema::geometry::segment_bounds(segment,current,start,[1.0,0.0,0.0,1.0,0.0,0.0]);
        for axis in 0..2{min[axis]=min[axis].min(bounds[axis]);max[axis]=max[axis].max(bounds[axis]+bounds[axis+2]);}
        current=match segment{PathSegment::Move {to}|PathSegment::Line {to}|PathSegment::Quad {to,..}|PathSegment::Cubic {to,..}|PathSegment::Arc {to,..}=>*to,PathSegment::Close=>start};
    }
    let mut matrix=crate::schema::drawing_transform_to_matrix(&super::transform::parse_editable_svg_transform(values.get("gradientTransform").and_then(SvgAttributeValue::text).unwrap_or(""))?);
    if unit=="objectBoundingBox"{let (w,h)=(max[0]-min[0],max[1]-min[1]);if !(w>0.0&&h>0.0){return Ok(None);}matrix=crate::schema::geometry::multiply([w,0.0,0.0,h,min[0],min[1]],matrix);}
    let coordinate=|key:&str,fallback:f64,axis:usize|->Result<f64,String>{let scale=if unit=="objectBoundingBox"{1.0}else{viewport[axis]};match values.get(key){None=>Ok(fallback*scale),Some(SvgAttributeValue::Length(value)) if value.unit=="%"=>Ok(value.magnitude/100.0*scale),Some(SvgAttributeValue::Length(value)) if unit=="objectBoundingBox"&&value.unit.is_empty()=>Ok(value.magnitude),Some(value) if unit=="userSpaceOnUse"=>length(Some(value),0.0),_=>Err("SVG gradient coordinate requires decoded length".into())}};
    let (x,y)=(coordinate("x1",0.0,0)?,coordinate("y1",0.0,1)?);let (vx,vy)=(coordinate("x2",1.0,0)?-x,coordinate("y2",0.0,1)?-y);
    if vx==0.0&&vy==0.0{return Ok(Some(FillStyle::Solid {color:stops.last().unwrap().color}));}
    let [a,b,c,d,e,f]=matrix;let det=a*d-b*c;let len=vx*vx+vy*vy;if det==0.0{return Err("Singular SVG gradient transform".into());}
    let (nx,ny)=((d*vx-b*vy)/det/len,(-c*vx+a*vy)/det/len);let norm=nx*nx+ny*ny;
    let (x1,y1)=(a*x+c*y+e,b*x+d*y+f);let (x2,y2)=(x1+nx/norm,y1+ny/norm);
    if ![x1,y1,x2,y2].iter().all(|value|value.is_finite()){return Err("SVG gradient exceeds finite coordinates".into());}
    Ok(Some(FillStyle::LinearGradient {x1,y1,x2,y2,stops}))
}
enum Work {Element {node:SvgNode,style:Style,id:String},FinishGroup}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct SvgImportProgress {pub completed:usize,pub pending:usize,pub done:bool}
pub struct SvgImportJob {gradients:BTreeMap<String,SvgNode>,user_viewport:[f64;2],pending:Vec<Work>,groups:Vec<DrawingGroupBody>,completed:usize,document:Option<DrawingSnapshot>,cancelled:bool,failed:bool}
impl SvgImportJob {
    pub fn new(source:&str,id:&str)->Result<Self,String> {
        if id.is_empty(){return Err("SVG import needs a document id".into());}
        let root=parse_svg_xml(source)?.root.ok_or("Invalid SVG document")?;
        let SvgNode::Element {name,attrs,children}=&root else{return Err("Invalid SVG document".into());};
        if local(name)!="svg"{return Err("Invalid SVG document".into());}
        let (width,height,_)=viewport(attrs)?;
        let title=children.iter().find_map(|child|match child{SvgNode::Element {name,children,..} if local(name)=="title"=>Some(children.iter().filter_map(|child|match child{SvgNode::Text {text}|SvgNode::CData {text}=>Some(text.as_str()),_=>None}).collect::<String>()),_=>None}).filter(|value|!value.trim().is_empty()).unwrap_or_else(||"SVG".into());
        let document=DrawingSnapshot {id:id.into(),title:Some(title.trim().into()),layers:semio_framework_value::list::PagedList::new(),artboard:Some(crate::schema::DrawingArtboard {width,height}),..Default::default()};
        let user_viewport=if let Some(SvgAttributeValue::ViewBox(value))=attr(attrs,"viewBox"){[value.width,value.height]}else{[width,height]};
        let mut gradients=BTreeMap::new();let mut nodes=vec![&root];
        while let Some(node)=nodes.pop(){if let SvgNode::Element {name,attrs,children}=node{
            if ["linearGradient","radialGradient"].contains(&local(name)){if let Some(id)=text_attr(attrs,"id"){if gradients.insert(id.to_owned(),node.clone()).is_some(){return Err("Duplicate SVG gradient id".into());}}}
            nodes.extend(children.iter());
        }}
        Ok(Self {gradients,user_viewport,pending:vec![Work::Element {node:root,style:Style::new(),id:"svg".into()}],groups:Vec::new(),completed:0,document:Some(document),cancelled:false,failed:false})
    }
    pub fn cancel(&mut self){self.cancelled=true;self.pending.clear();self.groups.clear();self.gradients.clear();self.document=None;}
    pub fn step(&mut self,budget:usize)->Result<SvgImportProgress,String> {
        if self.cancelled||self.failed||self.document.is_none(){return Err("SVG import is not available".into());}
        if budget==0{return Err("SVG import budget must be positive".into());}
        for _ in 0..budget {
            let Some(work)=self.pending.pop() else{break;};
            if let Err(error)=self.visit(work){self.failed=true;self.pending.clear();self.groups.clear();self.gradients.clear();self.document=None;return Err(error);}
        }
        Ok(SvgImportProgress {completed:self.completed,pending:self.pending.len(),done:self.pending.is_empty()})
    }
    pub fn take(&mut self)->Result<DrawingSnapshot,String> {
        if self.cancelled||self.failed||!self.pending.is_empty(){return Err("SVG import is incomplete".into());}
        self.gradients.clear();
        self.document.take().ok_or_else(||"SVG import is incomplete".into())
    }
    fn append(&mut self,layer:DrawingLayerNode){if let Some(group)=self.groups.last_mut(){group.children.push(layer);}else{self.document.as_mut().unwrap().layers.push(layer);}}
    fn visit(&mut self,work:Work)->Result<(),String> {
        let Work::Element {node,style:parent,id}=work else{let group=self.groups.pop().ok_or("Missing SVG group")?;self.append(DrawingLayerNode::Group(group));return Ok(());};
        self.completed+=1;
        let SvgNode::Element {name,attrs,children}=node else{return Ok(());};let tag=local(&name);
        if ["title","desc","metadata","defs","linearGradient","radialGradient","stop"].contains(&tag){return Ok(());}
        let style=properties(&attrs,&parent)?;let field=|key:&str|style.get(key);let value=|key:&str|field(key).and_then(SvgAttributeValue::text);
        let name=match text_attr(&attrs,"inkscape:label").or_else(||text_attr(&attrs,"id")){Some(value)=>value.into(),None if id=="svg"=>self.document.as_ref().unwrap().title.clone().unwrap_or_else(||tag.into()),None=>tag.into()};
        let mut transform=match attr(&attrs,"transform"){None=>super::transform::editable_svg_transform_operations(&[])?,Some(SvgAttributeValue::Transform(operations))=>super::transform::editable_svg_transform_operations(operations)?,_=>return Err("SVG transform requires decoded operations".into())};
        if id=="svg"{transform=crate::schema::drawing_matrix_to_transform(crate::schema::geometry::multiply(crate::schema::drawing_transform_to_matrix(&transform),viewport(&attrs)?.2));}
        let mut solid_style=style.clone();if value("fill").is_some_and(|paint|paint.starts_with("url(")){solid_style.insert("fill".into(),SvgAttributeValue::Text("none".into()));}
        let mut base=DrawingLayerBase {id:id.as_str().into(),name,visible:value("display")!=Some("none")&&(["g","svg"].contains(&tag)||!matches!(value("visibility"),Some("hidden"|"collapse"))),locked:false,opacity:fraction(field("opacity"))?,blend_mode:blend_mode(value("mix-blend-mode"))?.into(),transform,attributes:attributes(&solid_style)?};
        let isolated=isolation(value("isolation"))?;
        if tag=="g"||id=="svg" {
            self.groups.push(DrawingGroupBody {isolation:isolated,base,children:semio_framework_value::list::PagedList::new()});self.pending.push(Work::FinishGroup);
            let children=children.into_iter().filter(|node|matches!(node,SvgNode::Element {..})).collect::<Vec<_>>();
            for (index,node) in children.into_iter().enumerate().rev(){self.pending.push(Work::Element {node,style:style.clone(),id:format!("{id}.{index}")});}
        }else if tag=="text" {
            if value("fill").is_some_and(|paint|paint.starts_with("url(")){return Err("SVG text gradients need exact text bounds".into());}
            if children.iter().any(|node|matches!(node,SvgNode::Element {..})){return Err("Positioned SVG text spans are not supported yet".into());}
            let size=nonnegative(length(field("font-size"),16.0)?)?;
            let mut content=semio_framework_value::paged::PagedUtf8::default();for node in children{if let SvgNode::Text{text}|SvgNode::CData{text}=node{content.try_push_str(&text).map_err(|error|error.into_message())?;}}
            self.append(DrawingLayerNode::Text(DrawingTextBody {font_family:match value("font-family"){Some(family)=>crate::DrawingFontFamily::from_catalog_family(family).map_err(str::to_owned)?,None=>crate::DrawingFontFamily::Anta},base,x:length(attr(&attrs,"x"),0.0)?,y:length(attr(&attrs,"y"),0.0)?-size,content,size}));
        }else{
            let segments=geometry(tag,&attrs)?;
            if let Some(paint)=value("fill").filter(|paint|paint.starts_with("url(")){base.attributes.fill=gradient_fill(paint,&self.gradients,&segments,self.user_viewport,fraction(field("fill-opacity"))?)?;}
            self.append(DrawingLayerNode::Path(DrawingPathBody {base,segments:segments.into()}));
        }
        Ok(())
    }
}
#[cfg(test)]
#[path="🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
