//! 🧩️ Borrowed XML components append to one enclosing paid domain projection.
use super::*;
/// 📰️ Writes all authored XML document fields into the caller's fresh relation set.
pub(super) fn append<'a>(documents:impl ExactSizeIterator<Item=(&'a str,XmlDocumentView<'a>)>,tables:XmlSqliteTables,output:&mut Projection<'_,'_>)->Result<(),ValueError>{
 let mut node_id=0i64;
 for(schema,doc)in documents{
  let root=node_id.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"XML node identity overflow"))?;
  let document=output.insert(tables.document,&[Cell::Text(schema),if doc.root.is_some(){Cell::Integer(root)}else{Cell::Null}])?;
  if let Some(value)=doc.declaration{output.insert(tables.declaration,&[Cell::Integer(document),Cell::Text(&value.version),optional(value.encoding.as_deref()),value.standalone.map_or(Cell::Null,|value|Cell::Integer(i64::from(value))),Cell::Text(if value.quote.is_double(){"double"}else{"single"})])?;}
  if let Some(value)=doc.doctype{
   let(external,public,system)=match &value.external_id{None=>(None,None,None),Some(XmlExternalId::System{system_id})=>(Some("system"),None,Some(system_id.as_str())),Some(XmlExternalId::Public{public_id,system_id})=>(Some("public"),Some(public_id.as_str()),Some(system_id.as_str()))};
   let mut position=[0;20];let id=output.insert(tables.doctype,&[Cell::Integer(document),Cell::Text(decimal(value.prolog_position,&mut position)),Cell::Text(&value.name),optional(external),optional(public),optional(system)])?;
   for(ordinal,value)in value.declarations.iter().enumerate(){let XmlDtdDeclaration::Entity{parameter,name,value}=value;output.insert(tables.entity,&[Cell::Integer(id),Cell::Integer(integer(ordinal)?),Cell::Integer(i64::from(*parameter)),Cell::Text(name),Cell::Text(value)])?;}
  }
  let mut pending=output.allocate_frontier(0)?;
  for(ordinal,node)in doc.epilog.iter().enumerate().rev(){output.push_frontier(&mut pending,(node,Parent::Misc("epilog",ordinal)))?;}
  for(ordinal,node)in doc.prolog.iter().enumerate().rev(){output.push_frontier(&mut pending,(node,Parent::Misc("prolog",ordinal)))?;}
  if let Some(root)=doc.root{output.push_frontier(&mut pending,(root,Parent::Root))?;}
  while let Some((node,parent))=pending.pop(){
   node_id=output.insert(tables.node,&[Cell::Text(kind(node))])?;
   match node{
    XmlNode::Element{name,attrs,children}=>{
     output.insert_key(tables.element,node_id,&[Cell::Text(name)])?;
     for(ordinal,attr)in attrs.iter().enumerate(){output.insert(tables.attribute,&[Cell::Integer(node_id),Cell::Integer(integer(ordinal)?),Cell::Text(&attr.name),Cell::Text(&attr.value)])?;}
     for(ordinal,child)in children.iter().enumerate().rev(){output.push_frontier(&mut pending,(child,Parent::Child(node_id,ordinal)))?;}
    },
    XmlNode::Text{text}=>output.insert_key(tables.text,node_id,&[Cell::Text(text)])?,
    XmlNode::CData{text}=>output.insert_key(tables.cdata,node_id,&[Cell::Text(text)])?,
    XmlNode::Comment{text}=>output.insert_key(tables.comment,node_id,&[Cell::Text(text)])?,
    XmlNode::ProcessingInstruction{target,data}=>output.insert_key(tables.processing_instruction,node_id,&[Cell::Text(target),Cell::Text(data)])?
   }
   match parent{Parent::Root=>{},Parent::Misc(position,ordinal)=>{output.insert(tables.document_misc,&[Cell::Integer(document),Cell::Text(position),Cell::Integer(integer(ordinal)?),Cell::Integer(node_id)])?;},Parent::Child(parent,ordinal)=>{output.insert(tables.child,&[Cell::Integer(parent),Cell::Integer(integer(ordinal)?),Cell::Integer(node_id)])?;}}
  }
 }
 Ok(())
}
