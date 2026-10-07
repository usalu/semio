//! 🚦️ Native SVG projection under physical allocation and cancellation authority.
use crate::schema::snapshot::{SvgAttributeValue as V,SvgNode as N,SvgDocument,TransformOp,PathCommand};
use semio_framework_dsl_record::NativeSchemaControl;
use semio_framework_value::{ValueError,ValueRefusalKind};
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlNode,XmlAttr,XmlDocument};
use std::fmt::Write;
fn grow<T,C:NativeSchemaControl>(values:&mut Vec<T>,count:usize,control:&mut C)->Result<(),ValueError>{
    let total=values.len().checked_add(count).filter(|count|*count<=isize::MAX as usize).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"SVG native frontier overflow"))?;
    if total>values.capacity(){let total=total.max(values.capacity().saturating_mul(2));control.charge(total.checked_mul(std::mem::size_of::<T>()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"SVG native frontier size overflow"))?)?;values.try_reserve_exact(total-values.len()).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"SVG native frontier allocation failed"))?;}Ok(())
}
struct Text<'a,C>{bytes:Vec<u8>,control:&'a mut C,failure:Option<ValueError>}
impl<C:NativeSchemaControl>Write for Text<'_,C>{
    fn write_str(&mut self,text:&str)->std::fmt::Result{
        if self.failure.is_some(){return Err(std::fmt::Error)}
        for piece in text.as_bytes().chunks(65536){let result=(||{self.control.checkpoint()?;grow(&mut self.bytes,piece.len(),self.control)?;self.bytes.extend_from_slice(piece);self.control.advance(piece.len())})()
            ;if let Err(error)=result{self.failure=Some(error);return Err(std::fmt::Error)}}Ok(())
    }
}
/// 📝️ Emits every native attribute token directly into a credited physical buffer.
pub fn print_svg_attribute_controlled<C:NativeSchemaControl>(value:&V,control:&mut C)->Result<String,ValueError>{
    control.scoped_stage(|control|{control.begin_stage(0)?;let mut output=Text{bytes:Vec::new(),control,failure:None};
        macro_rules! out{($($arg:tt)*)=>{write!(&mut output,$($arg)*).map_err(|_|output.failure.take().unwrap_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"SVG native formatting failed")))?};}
        match value{
            V::Text(text)=>out!("{text}"),V::LocalReference(id)=>out!("url(#{id})"),V::Number(number)=>out!("{number}"),V::Length(length)=>out!("{}{}",length.magnitude,length.unit),V::ViewBox(v)=>out!("{} {} {} {}",v.min_x,v.min_y,v.width,v.height),
            V::Points(points)=>for(index,p)in points.iter().enumerate(){if index>0{out!(" ")}out!("{},{}",p.x,p.y)},
            V::Transform(ops)=>for(index,op)in ops.iter().enumerate(){if index>0{out!(" ")}match op{
                TransformOp::Matrix{a,b,c,d,e,f}=>out!("matrix({a},{b},{c},{d},{e},{f})"),TransformOp::Translate{x,y:None}=>out!("translate({x})"),TransformOp::Translate{x,y:Some(y)}=>out!("translate({x},{y})"),
                TransformOp::Scale{x,y:None}=>out!("scale({x})"),TransformOp::Scale{x,y:Some(y)}=>out!("scale({x},{y})"),TransformOp::Rotate{angle,center:None}=>out!("rotate({angle})"),TransformOp::Rotate{angle,center:Some((x,y))}=>out!("rotate({angle},{x},{y})"),TransformOp::SkewX{angle}=>out!("skewX({angle})"),TransformOp::SkewY{angle}=>out!("skewY({angle})"),
            }},
            V::PathData(commands)=>for(index,op)in commands.iter().enumerate(){if index>0{out!(" ")}let letter=|c:char,relative:bool|if relative{c.to_ascii_lowercase()}else{c};match op{
                PathCommand::MoveTo{x,y,relative}=>out!("{} {x} {y}",letter('M',*relative)),PathCommand::LineTo{x,y,relative}=>out!("{} {x} {y}",letter('L',*relative)),PathCommand::HorizontalLineTo{x,relative}=>out!("{} {x}",letter('H',*relative)),PathCommand::VerticalLineTo{y,relative}=>out!("{} {y}",letter('V',*relative)),
                PathCommand::CurveTo{x1,y1,x2,y2,x,y,relative}=>out!("{} {x1} {y1} {x2} {y2} {x} {y}",letter('C',*relative)),PathCommand::SmoothCurveTo{x2,y2,x,y,relative}=>out!("{} {x2} {y2} {x} {y}",letter('S',*relative)),PathCommand::QuadraticCurveTo{x1,y1,x,y,relative}=>out!("{} {x1} {y1} {x} {y}",letter('Q',*relative)),PathCommand::SmoothQuadraticCurveTo{x,y,relative}=>out!("{} {x} {y}",letter('T',*relative)),PathCommand::Arc{rx,ry,x_axis_rotation,large_arc,sweep,x,y,relative}=>out!("{} {rx} {ry} {x_axis_rotation} {} {} {x} {y}",letter('A',*relative),*large_arc as u8,*sweep as u8),PathCommand::ClosePath=>out!("Z"),
            }}
        }String::from_utf8(output.bytes).map_err(|_|ValueError::new(ValueRefusalKind::InvariantViolated,"SVG native attribute output is not UTF8"))
    })
}
struct Nodes(Vec<XmlNode>);
impl Drop for Nodes{fn drop(&mut self){semio_s_artifact_stdio_xml::schema::snapshot::ownership::retire_xml_document(XmlDocument{prolog:std::mem::take(&mut self.0),..Default::default()});}}
fn node<C:NativeSchemaControl>(root:&N,control:&mut C)->Result<XmlNode,ValueError>{
    enum Frame<'a>{Visit(&'a N),Finish(String,Vec<XmlAttr>,usize)}
    let mut frames=control.allocate_vec(1)?;frames.push(Frame::Visit(root));let mut nodes=Nodes(Vec::new());
    while let Some(frame)=frames.pop(){control.checkpoint()?;match frame{
        Frame::Finish(name,attrs,count)=>{grow(&mut nodes.0,1,control)?;let mut children=control.allocate_vec(count)?;let start=nodes.0.len()-count;children.extend(nodes.0.drain(start..));nodes.0.push(XmlNode::Element{name,attrs,children});},
        Frame::Visit(value)=>{grow(&mut nodes.0,1,control)?;match value{
            N::Element{name,attrs,children}=>{let mut native=control.allocate_vec(attrs.len())?;for attr in attrs{control.checkpoint()?;crate::schema::snapshot::validate_svg_attribute_owner(&attr.name,&attr.value).map_err(|detail|ValueError::new(ValueRefusalKind::InvalidValue,detail))?;native.push(XmlAttr{name:control.copy_text(&attr.name)?,value:print_svg_attribute_controlled(&attr.value,control)?});}
                grow(&mut frames,children.len()+1,control)?;frames.push(Frame::Finish(control.copy_text(name)?,native,children.len()));frames.extend(children.iter().rev().map(Frame::Visit));},
            N::Text{text}=>nodes.0.push(XmlNode::Text{text:control.copy_text(text)?}),N::CData{text}=>nodes.0.push(XmlNode::CData{text:control.copy_text(text)?}),N::Comment{text}=>nodes.0.push(XmlNode::Comment{text:control.copy_text(text)?}),N::ProcessingInstruction{target,data}=>nodes.0.push(XmlNode::ProcessingInstruction{target:control.copy_text(target)?,data:control.copy_text(data)?}),
        }}
    }control.step()?;}nodes.0.pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"SVG native projection has no root"))
}
/// 🌳️ Builds and owns every temporary XML lane before existing native XML emission.
pub fn native_svg_document_controlled<C:NativeSchemaControl>(value:&SvgDocument,control:&mut C)->Result<XmlDocument,ValueError>{
    use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDeclaration,XmlDoctype,XmlExternalId,XmlDtdDeclaration};
    use semio_framework_dsl_record::__rt::DecodedFieldOwner;
    control.scoped_stage(|control|{control.begin_stage(0)?;let mut owner=DecodedFieldOwner::new(XmlDocument::default(),semio_s_artifact_stdio_xml::schema::snapshot::ownership::retire_xml_document);
        owner.as_mut().declaration=value.declaration.as_ref().map(|v|->Result<XmlDeclaration,ValueError>{Ok(XmlDeclaration{version:control.copy_text(&v.version)?,encoding:v.encoding.as_deref().map(|v|control.copy_text(v)).transpose()?,standalone:v.standalone,quote:v.quote})}).transpose()?;
        owner.as_mut().doctype=value.doctype.as_ref().map(|v|->Result<XmlDoctype,ValueError>{let external_id=v.external_id.as_ref().map(|e|->Result<XmlExternalId,ValueError>{match e{XmlExternalId::System{system_id}=>Ok(XmlExternalId::System{system_id:control.copy_text(system_id)?}),XmlExternalId::Public{public_id,system_id}=>Ok(XmlExternalId::Public{public_id:control.copy_text(public_id)?,system_id:control.copy_text(system_id)?})}}).transpose()?;let mut declarations=control.allocate_vec(v.declarations.len())?;for declaration in &v.declarations{control.checkpoint()?;match declaration{XmlDtdDeclaration::Entity{parameter,name,value}=>declarations.push(XmlDtdDeclaration::Entity{parameter:*parameter,name:control.copy_text(name)?,value:control.copy_text(value)?})}}Ok(XmlDoctype{prolog_position:v.prolog_position,name:control.copy_text(&v.name)?,external_id,declarations})}).transpose()?;
        owner.as_mut().prolog=control.allocate_vec(value.prolog.len())?;for v in &value.prolog{owner.as_mut().prolog.push(node(v,control)?);}
        owner.as_mut().root=value.root.as_ref().map(|v|node(v,control)).transpose()?;
        owner.as_mut().epilog=control.allocate_vec(value.epilog.len())?;for v in &value.epilog{owner.as_mut().epilog.push(node(v,control)?);}Ok(owner.take())
    })
}

/// 📥️ Parses known native attributes directly into credited semantic collections.
pub fn bind_svg_attribute_controlled<C:NativeSchemaControl>(name:&str,text:&str,control:&mut C)->Result<V,ValueError>{
    use crate::schema::snapshot::{SvgPoint,SvgLength,ViewBox};
    let invalid=|detail:String|ValueError::new(ValueRefusalKind::InvalidValue,detail);
    control.scoped_stage(|control|{control.begin_stage(0)?;Ok(match name{
        "transform"=>{let mut values=Vec::new();let mut failure=None;let result=super::read_transform_list(text,|value|{let admitted=(||{control.checkpoint()?;grow(&mut values,1,control)?;values.push(value);control.step()})();admitted.map_err(|error|{failure=Some(error);String::new()})});if let Err(error)=result{return Err(failure.unwrap_or_else(||invalid(error)))}V::Transform(values)},
        "d"=>{let mut values=Vec::new();let mut failure=None;let result=super::read_path_data(text,|value|{let admitted=(||{control.checkpoint()?;grow(&mut values,1,control)?;values.push(value);control.step()})();admitted.map_err(|error|{failure=Some(error);String::new()})});if let Err(error)=result{return Err(failure.unwrap_or_else(||invalid(error)))}V::PathData(values)},
        "points"=>{let mut cursor=super::NumCursor::new(text);let mut values=Vec::new();loop{control.checkpoint()?;cursor.skip_wsp_comma();if cursor.is_eof(){break}let x=cursor.parse_number().map_err(&invalid)?;let y=cursor.parse_number().map_err(&invalid)?;grow(&mut values,1,control)?;values.push(SvgPoint{x,y});control.step()?;}V::Points(values)},
        "viewBox"=>{let mut cursor=super::NumCursor::new(text);let mut values=[0.0;4];for value in &mut values{control.checkpoint()?;*value=cursor.parse_number().map_err(&invalid)?;}cursor.skip_wsp_comma();if !cursor.is_eof(){return Err(invalid("SVG viewBox requires four numbers".into()))}V::ViewBox(ViewBox{min_x:values[0],min_y:values[1],width:values[2],height:values[3]})},
        "width"|"height"|"x"|"y"|"x1"|"y1"|"x2"|"y2"|"cx"|"cy"|"r"|"rx"|"ry"|"fx"|"fy"=>{let text=text.trim();let mut cursor=super::NumCursor::new(text);let magnitude=cursor.parse_number().map_err(&invalid)?;V::Length(SvgLength{magnitude,unit:control.copy_text(text[cursor.pos..].trim())?})},
        "opacity"|"fill-opacity"|"stroke-opacity"=>V::Number(text.trim().parse().map_err(|_|invalid("SVG opacity requires number".into()))?),
        "clip-path" if text.trim().starts_with("url(")=>{let text=text.trim();let reference=text.strip_prefix("url(").and_then(|v|v.strip_suffix(')')).ok_or_else(||invalid("invalid SVG clip-path reference".into()))?.trim().trim_matches(|c|c=='\''||c=='"');if let Some(id)=reference.strip_prefix('#'){V::LocalReference(control.copy_text(id)?)}else{V::Text(control.copy_text(text)?)}},
        _=>V::Text(control.copy_text(text)?),
    })})
}
enum DecodeFrame{Visit(XmlNode),Finish(String,Vec<crate::schema::snapshot::SvgAttr>,usize)}
struct DecodeFrames(Vec<DecodeFrame>);
impl Drop for DecodeFrames{fn drop(&mut self){while let Some(frame)=self.0.pop(){if let DecodeFrame::Visit(node)=frame{semio_s_artifact_stdio_xml::schema::snapshot::ownership::retire_xml_document(XmlDocument{root:Some(node),..Default::default()});}}}}
struct SvgNodes(Vec<N>);
impl Drop for SvgNodes{fn drop(&mut self){crate::schema::snapshot::retire_svg_document(SvgDocument{prolog:std::mem::take(&mut self.0),..Default::default()});}}
fn bind_node<C:NativeSchemaControl>(node:XmlNode,control:&mut C)->Result<N,ValueError>{
    let mut native=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(XmlDocument{root:Some(node),..Default::default()},semio_s_artifact_stdio_xml::schema::snapshot::ownership::retire_xml_document);
    let mut frames=DecodeFrames(control.allocate_vec(1)?);frames.0.push(DecodeFrame::Visit(native.as_mut().root.take().expect("SVG admission root")));let mut nodes=SvgNodes(Vec::new());
    while let Some(frame)=frames.0.pop(){match frame{
        DecodeFrame::Finish(name,attrs,count)=>{grow(&mut nodes.0,1,control)?;let mut children=control.allocate_vec(count)?;let start=nodes.0.len()-count;children.extend(nodes.0.drain(start..));nodes.0.push(N::Element{name,attrs,children});},
        DecodeFrame::Visit(node)=>{let mut native=semio_framework_dsl_record::__rt::DecodedFieldOwner::new(XmlDocument{root:Some(node),..Default::default()},semio_s_artifact_stdio_xml::schema::snapshot::ownership::retire_xml_document);control.checkpoint()?;grow(&mut nodes.0,1,control)?;
            if let Some(XmlNode::Element{attrs,children,..})=native.as_mut().root.as_ref(){grow(&mut frames.0,children.len()+1,control)?;let mut owned=control.allocate_vec(attrs.len())?;for attr in attrs{owned.push(crate::schema::snapshot::SvgAttr{name:control.copy_text(&attr.name)?,value:bind_svg_attribute_controlled(&attr.name,&attr.value,control)?});}
                if let XmlNode::Element{name,children,..}=native.as_mut().root.take().unwrap(){frames.0.push(DecodeFrame::Finish(name,owned,children.len()));frames.0.extend(children.into_iter().rev().map(DecodeFrame::Visit));}
            }else{let node=native.as_mut().root.take().unwrap();nodes.0.push(match node{XmlNode::Text{text}=>N::Text{text},XmlNode::CData{text}=>N::CData{text},XmlNode::Comment{text}=>N::Comment{text},XmlNode::ProcessingInstruction{target,data}=>N::ProcessingInstruction{target,data},XmlNode::Element{..}=>unreachable!()});}
        }
    }control.step()?;}nodes.0.pop().ok_or_else(||ValueError::new(ValueRefusalKind::InvariantViolated,"SVG typed admission has no root"))
}
/// 🌳️ Admits each native document lane under its caller's cumulative physical budget.
pub fn bind_svg_document_controlled<C:NativeSchemaControl>(document:XmlDocument,control:&mut C)->Result<SvgDocument,ValueError>{
    use semio_framework_dsl_record::__rt::DecodedFieldOwner;
    let mut native=DecodedFieldOwner::new(document,semio_s_artifact_stdio_xml::schema::snapshot::ownership::retire_xml_document);
    control.scoped_stage(|control|{control.begin_stage(0)?;let mut owner=DecodedFieldOwner::new(SvgDocument::default(),crate::schema::snapshot::retire_svg_document);owner.as_mut().declaration=native.as_mut().declaration.take();owner.as_mut().doctype=native.as_mut().doctype.take();owner.as_mut().prolog=control.allocate_vec(native.as_mut().prolog.len())?;owner.as_mut().epilog=control.allocate_vec(native.as_mut().epilog.len())?;
        while let Some(node)=native.as_mut().prolog.pop(){owner.as_mut().prolog.push(bind_node(node,control)?);}owner.as_mut().prolog.reverse();if let Some(node)=native.as_mut().root.take(){owner.as_mut().root=Some(bind_node(node,control)?);}while let Some(node)=native.as_mut().epilog.pop(){owner.as_mut().epilog.push(bind_node(node,control)?);}owner.as_mut().epilog.reverse();Ok(owner.take())
    })
}
