//! 🌳️ SVG document ownership with decoded native attributes.
use super::{ViewBox,TransformOp,PathCommand};
use semio_framework_value::FromValue;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDeclaration,XmlDoctype};
#[derive(Clone,Debug,PartialEq,value_derive::ToValue,value_derive::FromValue)]
#[value(tag="kind",content="value",rename_all="camelCase")]
pub enum SvgAttributeValue {
    Text(String),
    LocalReference(String),
    Number(f64),
    Length(SvgLength),
    ViewBox(ViewBox),
    Transform(Vec<TransformOp>),
    Points(Vec<SvgPoint>),
    PathData(Vec<PathCommand>),
}
#[derive(Clone,Debug,PartialEq,value_derive::ToValue,value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct SvgLength { pub magnitude:f64,pub unit:String }
#[derive(Clone,Copy,Debug,PartialEq,value_derive::ToValue,value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct SvgPoint { pub x:f64,pub y:f64 }
#[derive(Clone,Debug,PartialEq,value_derive::ToValue)]
#[value(rename_all="camelCase")]
pub struct SvgAttr {pub name:String,pub value:SvgAttributeValue}
#[derive(value_derive::FromValue)]
#[value(rename_all="camelCase")]
struct SvgAttributeFields{name:String,value:SvgAttributeValue}
impl semio_framework_value::FromValue for SvgAttr{
    fn from_value(value:semio_framework_value::DslValue)->Result<Self,semio_framework_value::ValueError>{Self::admit(SvgAttributeFields::from_value(value)?)}
    fn from_value_controlled(value:&semio_framework_value::DslValue,control:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<Self,semio_framework_value::ValueError>{Self::admit(SvgAttributeFields::from_value_controlled(value,control)?)}
    fn edit_value_at_path(&mut self,path:&[&str],edit:semio_framework_value::ValueEdit)->Result<(),semio_framework_value::ValueError>{semio_framework_value::edit_through_value(self,path,edit)}
}
impl SvgAttr{
    fn admit(fields:SvgAttributeFields)->Result<Self,semio_framework_value::ValueError>{validate_svg_attribute_owner(&fields.name,&fields.value).map_err(|detail|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,detail))?;Ok(Self{name:fields.name,value:fields.value})}
}
#[derive(Clone,Debug,PartialEq,value_derive::ToValue,value_derive::FromValue)]
#[value(tag="kind",rename_all="camelCase")]
pub enum SvgNode {
    Element {name:String,#[value(default)] attrs:Vec<SvgAttr>,#[value(default)] children:Vec<SvgNode>},
    Text {text:String},
    CData {text:String},
    Comment {text:String},
    ProcessingInstruction {target:String,data:String},
}
#[derive(Clone,Debug,Default,PartialEq,value_derive::ToValue,value_derive::FromValue)]
#[value(rename_all="camelCase")]
pub struct SvgDocument {
    #[value(default)] pub declaration:Option<XmlDeclaration>,#[value(default)] pub doctype:Option<XmlDoctype>,#[value(default)] pub prolog:Vec<SvgNode>,#[value(default)] pub root:Option<SvgNode>,#[value(default)] pub epilog:Vec<SvgNode>,
}
impl SvgDocument {
    pub fn validate_boundaries(&self)->Result<(),String> {
        semio_s_artifact_stdio_xml::schema::snapshot::validate_xml_declaration_boundary(self.declaration.as_ref())?;
        if self.prolog.iter().chain(&self.epilog).any(|node|!matches!(node,SvgNode::Comment{..}|SvgNode::ProcessingInstruction{..})) {return Err("SVG boundaries permit only comments and processing instructions".into());}
        if self.doctype.as_ref().is_some_and(|doctype|doctype.prolog_position>self.prolog.len()as u64) {return Err("SVG doctype position exceeds prolog".into());}
        if self.root.as_ref().is_some_and(|node|!matches!(node,SvgNode::Element{..})) {return Err("SVG root requires element".into());}
        Ok(())
    }
}

/// ♻️ Retires recursive SVG owners through an explicit stack.
pub fn retire_svg_document(mut document:SvgDocument) {
    let mut nodes=Vec::new();nodes.append(&mut document.prolog);nodes.append(&mut document.epilog);if let Some(root)=document.root.take(){nodes.push(root);}
    while let Some(node)=nodes.pop(){if let SvgNode::Element{mut children,..}=node{nodes.append(&mut children);}}
}

impl SvgAttributeValue { pub fn text(&self)->Option<&str>{if let Self::Text(value)=self {Some(value)}else{None}} pub fn owned_size(&self)->usize{match self{Self::Text(value)|Self::LocalReference(value)=>value.len(),Self::Transform(value)=>value.len()*64,Self::Points(value)=>value.len()*16,Self::PathData(value)=>value.len()*64,_=>64}} }

/// 🛡️ Known SVG attribute positions require their decoded semantic owner.
pub fn validate_svg_attribute_owner(name:&str,value:&SvgAttributeValue)->Result<(),String>{
    let valid=match name{
        "viewBox"=>matches!(value,SvgAttributeValue::ViewBox(_)),"transform"=>matches!(value,SvgAttributeValue::Transform(_)),"points"=>matches!(value,SvgAttributeValue::Points(_)),"d"=>matches!(value,SvgAttributeValue::PathData(_)),
        "width"|"height"|"x"|"y"|"x1"|"y1"|"x2"|"y2"|"cx"|"cy"|"r"|"rx"|"ry"|"fx"|"fy"=>matches!(value,SvgAttributeValue::Length(_)),
        "opacity"|"fill-opacity"|"stroke-opacity"=>matches!(value,SvgAttributeValue::Number(_)),
        "clip-path"=>matches!(value,SvgAttributeValue::LocalReference(_)|SvgAttributeValue::Text(_)),
        _=>matches!(value,SvgAttributeValue::Text(_)),
    };if valid{Ok(())}else{Err(format!("SVG attribute {name} requires its decoded owner"))}
}
impl SvgDocument{
    pub fn validate_attribute_owners(&self)->Result<(),String>{let mut pending:Vec<_>=self.prolog.iter().chain(&self.epilog).chain(self.root.iter()).collect();while let Some(node)=pending.pop(){if let SvgNode::Element{attrs,children,..}=node{for attr in attrs{validate_svg_attribute_owner(&attr.name,&attr.value)?;}pending.extend(children);}}Ok(())}
}
