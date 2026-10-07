//! 🪪️ Native known string roles bind once into retained semantic text and date values.
use crate::standards::v1_7::subsets::base::{schema::snapshot::*,modules::fonts::decode_text_string};
use std::collections::{HashMap,HashSet};

fn role(key:&str)->Option<u8> {
    if ["CreationDate","ModDate","LastModified","M"].contains(&key) {Some(1)}
    else if key=="URI" {Some(2)}
    else if ["Title","Author","Subject","Keywords","Creator","Producer","Lang","T","TU","TM","V","DV","RC","RV","Contents","Subj","Desc","F","UF","DOS","Mac","Unix","D","Dest","JS","Name","Registry","Ordering","OutputCondition","OutputConditionIdentifier","RegistryName","Info","Names","Opt","Fields","FontFamily"].contains(&key) {Some(0)}else{None}
}
fn admitted(bytes:&[u8],role:u8)->PdfObject {
    let text=if role==2 {String::from_utf8_lossy(bytes).into_owned()}else{decode_text_string(bytes)};
    if role==1 {if let Some(date)=super::date::parse_pdf_date(&text) {return PdfObject::Date(date);}}
    PdfObject::Text(text)
}
fn collect_roles(value:&PdfObject,context:Option<u8>,roles:&mut HashSet<(ObjRef,u8)>,depth:usize) {
    if depth>=64 {return;}
    match value {
        PdfObject::Ref(reference)=>{if let Some(context)=context {roles.insert((*reference,context));}},
        PdfObject::Array(values)=>for value in values {collect_roles(value,context,roles,depth+1)},
        PdfObject::Dict(entries)|PdfObject::Stream{dict:entries,..}=>for entry in entries {collect_roles(&entry.value,role(&entry.key),roles,depth+1)},
        _=>{}
    }
}
fn bind(value:&mut PdfObject,context:Option<u8>,depth:usize) {
    if depth>=64 {return;}
    match value {
        PdfObject::Str(bytes)=>{if let Some(context)=context {*value=admitted(bytes,context);}},
        PdfObject::Array(values)=>for value in values {bind(value,context,depth+1)},
        PdfObject::Dict(entries)|PdfObject::Stream{dict:entries,..}=>for entry in entries {bind(&mut entry.value,role(&entry.key),depth+1)},
        _=>{}
    }
}
/// 🧾️ Admits known roles while retaining one logical object for each indirect identity.
pub fn admit_retained_text(objects:&mut [PdfIndirectObject],trailer:&mut [PdfDictEntry]) {
    let indices:HashMap<ObjRef,usize>=objects.iter().enumerate().map(|(index,object)|(object.id,index)).collect();
    let mut roles=HashSet::new();for object in objects.iter(){collect_roles(&object.value,None,&mut roles,0);}for entry in trailer.iter(){collect_roles(&entry.value,role(&entry.key),&mut roles,0);}
    let mut pending:Vec<_>=roles.iter().copied().collect();
    while let Some((reference,context))=pending.pop() {
        if let Some(index)=indices.get(&reference) {
            let mut children=HashSet::new();collect_roles(&objects[*index].value,Some(context),&mut children,0);
            for child in children {if roles.insert(child){pending.push(child);}}
        }
    }
    let mut ordered:Vec<_>=roles.into_iter().collect();ordered.sort_by_key(|(reference,context)|(reference.num,reference.gen,if *context==1 {0}else{*context+1}));
    for (reference,context) in ordered {if let Some(index)=indices.get(&reference) {bind(&mut objects[*index].value,Some(context),0);}}
    for object in objects.iter_mut(){bind(&mut object.value,None,0);}for entry in trailer.iter_mut(){bind(&mut entry.value,role(&entry.key),0);}
}
