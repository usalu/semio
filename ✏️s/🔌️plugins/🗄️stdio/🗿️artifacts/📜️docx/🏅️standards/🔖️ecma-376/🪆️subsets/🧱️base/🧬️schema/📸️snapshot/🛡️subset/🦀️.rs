//! 🛡️ Exact DOCX namespace declarations and relationship policies over the owned XML graph.
use super::DocxSnapshot;
use semio_s_artifact_stdio_xml::schema::snapshot::{XmlDocument,XmlNode,XmlAttr};

use semio_framework_value::NativeEncodeControl;
use semio_framework_diagnostic::Diagnostic;
use semio_framework_diagnostic::FaultCode;
use semio_framework_diagnostic::Severity;
use semio_framework_diagnostic::TextSpan;
use semio_framework_diagnostic::FaultScope;
const STRICT:&str="http://purl.oclc.org/ooxml/wordprocessingml/main";
const TRANS:&str="http://schemas.openxmlformats.org/wordprocessingml/2006/main";
const STRICT_REL:&str="http://purl.oclc.org/ooxml/officeDocument/relationships";
const TRANS_REL:&str="http://schemas.openxmlformats.org/officeDocument/2006/relationships";
const VML:&str="urn:schemas-microsoft-com:vml";
const MC:&str="http://schemas.openxmlformats.org/markup-compatibility/2006";
fn equal(left:&str,right:&str,control:&mut NativeEncodeControl<'_>)->Result<bool, ValueError>{
 if left.len()!=right.len(){return Ok(false)}control.scoped_stage(|control|->Result<bool, ValueError>{control.begin_stage(left.len())?;for(a,b)in left.as_bytes().chunks(256).zip(right.as_bytes().chunks(256)){let same=a==b;control.advance(a.len())?;if !same{return Ok(false)}}Ok(true)})
}
fn text(parts:&[&str],control:&mut NativeEncodeControl<'_>)->Result<String, ValueError>{
 let length=parts.iter().try_fold(0usize,|total,part|total.checked_add(part.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "DOCX profile text overflow")))?;
 if length>control.maximum_bytes(){return Err(ValueError::new(ValueRefusalKind::OwnershipLimit, "DOCX profile text exceeds caller limit"))}
 control.scoped_stage(|control|->Result<String, ValueError>{control.begin_stage(length)?;control.charge(length)?;let mut output=String::new();output.try_reserve_exact(length).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "DOCX profile text allocation"))?;
 for part in parts{let mut start=0;while start<part.len(){let mut end=start.saturating_add(256).min(part.len());while !part.is_char_boundary(end){end-=1;}output.push_str(&part[start..end]);control.advance(end-start)?;start=end;}}Ok(output)})
}
fn diagnostic(output:&mut Vec<Diagnostic>,subset:&str,suffix:&str,severity:Severity,parts:&[&str],control:&mut NativeEncodeControl<'_>)->Result<(), ValueError>{
 control.charge(std::mem::size_of::<Diagnostic>())?;output.try_reserve_exact(1).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "DOCX diagnostic frontier allocation"))?;
 output.push(Diagnostic{code:FaultCode::new(text(&["stdio.docx.",subset,".",suffix],control)?),severity,span:TextSpan::at(1,1),message:text(parts,control)?,expected:None,scope:FaultScope::default()});Ok(())
}
#[derive(Default)]
struct Flags{main_strict:bool,main_trans:bool,trans:bool,vml:bool,family:bool,alternate:bool,conformance:bool}
struct Frame<'a>{nodes:&'a[XmlNode],position:usize,attrs:&'a[XmlAttr]}
fn push<'a>(frames:&mut Vec<Frame<'a>>,nodes:&'a[XmlNode],attrs:&'a[XmlAttr],control:&mut NativeEncodeControl<'_>)->Result<(), ValueError>{
 if frames.len()==frames.capacity(){control.charge(64*std::mem::size_of::<Frame<'a>>())?;frames.try_reserve_exact(64).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "DOCX namespace frontier allocation"))?;}frames.push(Frame{nodes,position:0,attrs});Ok(())
}
fn colon(name:&str,control:&mut NativeEncodeControl<'_>)->Result<Option<usize>, ValueError>{
 control.scoped_stage(|control|->Result<Option<usize>, ValueError>{control.begin_stage(name.len())?;for(position,chunk)in name.as_bytes().chunks(256).enumerate(){let found=chunk.iter().position(|byte|*byte==b':');control.advance(chunk.len())?;if let Some(index)=found{return Ok(Some(position*256+index))}}Ok(None)})
}
fn namespace<'a>(attrs:&'a[XmlAttr],prefix:Option<&str>,control:&mut NativeEncodeControl<'_>)->Result<Option<&'a str>, ValueError>{
 for attr in attrs{control.checkpoint()?;let matches=match prefix{None=>attr.name=="xmlns",Some(prefix)=>match attr.name.strip_prefix("xmlns:"){Some(name)=>equal(name,prefix,control)?,None=>false}};if matches{return Ok(Some(&attr.value))}}Ok(None)
}
fn inspect(document:&XmlDocument,control:&mut NativeEncodeControl<'_>)->Result<Flags, ValueError>{
 let mut flags=Flags::default();let mut frames=Vec::new();
 for forest in[document.prolog.as_slice(),document.root.as_ref().map_or(&[][..],std::slice::from_ref),document.epilog.as_slice()]{
 push(&mut frames,forest,&[],control)?;
 while !frames.is_empty(){let frame=frames.last_mut().unwrap();let Some(node)=frame.nodes.get(frame.position)else{frames.pop();continue};frame.position+=1;control.checkpoint()?;
 let XmlNode::Element{name,attrs,children}=node else{continue};
 for attr in attrs{control.checkpoint()?;if attr.name=="xmlns"||attr.name.starts_with("xmlns:"){flags.trans|=equal(&attr.value,TRANS,control)?;flags.vml|=equal(&attr.value,VML,control)?;flags.family|=attr.value.starts_with("http://purl.oclc.org/ooxml/");}}
 if document.root.as_ref().is_some_and(|root|std::ptr::eq(root,node)){for attr in attrs{control.checkpoint()?;if attr.name=="conformance"&&attr.value=="strict"{flags.conformance=true;}}}
 let separator=colon(name,control)?;let local=separator.map_or(name.as_str(),|position|&name[position+1..]);
 if document.root.as_ref().is_some_and(|root|std::ptr::eq(root,node))&&equal(local,"document",control)?{if let Some(value)=namespace(attrs,separator.map(|position|&name[..position]),control)?{flags.main_strict=equal(value,STRICT,control)?;flags.main_trans=equal(value,TRANS,control)?;}}
 if equal(local,"AlternateContent",control)?{let prefix=separator.map(|position|&name[..position]);let mut value=namespace(attrs,prefix,control)?;if value.is_none(){for frame in frames.iter().rev(){value=namespace(frame.attrs,prefix,control)?;if value.is_some(){break}}}if let Some(value)=value{flags.alternate|=equal(value,MC,control)?;}}
 push(&mut frames,children,attrs,control)?;
 }}Ok(flags)
}
fn path(target:&str,control:&mut NativeEncodeControl<'_>)->Result<String, ValueError>{
 if let Some(path)=target.strip_prefix('/'){return text(&[path],control)}
 let mut parts=Vec::new();
 control.scoped_stage(|control|->Result<(), ValueError>{control.begin_stage(target.len())?;let mut start=0;for(position,byte)in target.bytes().enumerate(){if byte==b'/'{segment(&target[start..position],&mut parts,control)?;start=position+1;}control.step()?;}segment(&target[start..],&mut parts,control)})?;
 let length=parts.iter().try_fold(parts.len().saturating_sub(1),|total,part|total.checked_add(part.len()).ok_or_else(|| ValueError::new(ValueRefusalKind::OwnershipLimit, "DOCX path width overflow")))?;
 control.scoped_stage(|control|->Result<String, ValueError>{control.begin_stage(length)?;control.charge(length)?;let mut output=String::new();output.try_reserve_exact(length).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "DOCX path allocation"))?;
 for(position,part)in parts.iter().enumerate(){if position!=0{output.push('/');control.step()?;}let mut start=0;while start<part.len(){let mut end=(start+256).min(part.len());while !part.is_char_boundary(end){end-=1;}output.push_str(&part[start..end]);control.advance(end-start)?;start=end;}}Ok(output)})
}
fn segment<'a>(part:&'a str,parts:&mut Vec<&'a str>,control:&mut NativeEncodeControl<'_>)->Result<(), ValueError>{match part{""|"."=>{},".."=>{parts.pop();},_=>{if parts.len()==parts.capacity(){control.charge(64*std::mem::size_of::<&str>())?;parts.try_reserve_exact(64).map_err(|_| ValueError::new(ValueRefusalKind::AllocationFailed, "DOCX relationship path frontier allocation"))?;}parts.push(part);}}Ok(())}
pub(crate) fn check(snapshot:&DocxSnapshot,subset:&str,control:&mut NativeEncodeControl<'_>)->Result<Vec<Diagnostic>, ValueError>{
 let strict=subset=="strict";let mut output=Vec::new();let mut main=None;
 for relationship in snapshot.opc.relationships.get("").into_iter().flat_map(|relationships|relationships.iter()){control.checkpoint()?;if relationship.rel_type.to_string_owner().ends_with("/officeDocument"){main=Some(path(&relationship.target.to_string_owner(),control)?);break}}
 let mut main_part=None;if let Some(path)=main.as_deref(){for part in &snapshot.xml_parts{control.checkpoint()?;if equal(&part.path,path,control)?{main_part=Some(part);break}}}
 if main_part.is_none(){diagnostic(&mut output,subset,"main-ns-missing",Severity::Error,&["package has no root officeDocument relationship -- cannot locate the main document part to check the ",if strict{"strict"}else{"transitional"}," namespace on"],control)?;}
 for part in &snapshot.xml_parts{control.checkpoint()?;let document=part.materialize_document(control)?;let flags=inspect(&document,control)?;
 if main_part.is_some_and(|main|std::ptr::eq(main,part)){let path=main.as_deref().unwrap();if !(if strict{flags.main_strict}else{flags.main_trans}){diagnostic(&mut output,subset,"main-ns-missing",Severity::Error,&["main document part ",path," does not declare the ",if strict{"strict"}else{"transitional"}," WordprocessingML namespace ",if strict{STRICT}else{TRANS}],control)?;}
 if strict&&!flags.conformance{diagnostic(&mut output,subset,"conformance-attr-missing",Severity::Warning,&["main document part ",path," root element does not declare conformance=\"strict\""],control)?;}
 if !strict&&flags.conformance{diagnostic(&mut output,subset,"conformance-attr-invalid",Severity::Warning,&["main document part ",path," root element declares conformance=\"strict\" -- transitional documents must leave it absent or =\"transitional\""],control)?;}}
 if strict{if flags.trans{diagnostic(&mut output,subset,"transitional-ns-present",Severity::Error,&["part ",&part.path," contains the transitional WordprocessingML namespace ",TRANS," -- strict conformance forbids mixed namespaces"],control)?;}
 if flags.vml{diagnostic(&mut output,subset,"vml-present",Severity::Error,&["part ",&part.path," contains the VML namespace ",VML," -- VML is transitional-only markup, forbidden under strict conformance"],control)?;}
 if flags.alternate{diagnostic(&mut output,subset,"alternate-content-present",Severity::Warning,&["part ",&part.path," contains mc:AlternateContent compatibility markup"],control)?;}}
 else if flags.family{diagnostic(&mut output,subset,"strict-ns-present",Severity::Error,&["part ",&part.path," contains a strict-family namespace (purl.oclc.org/ooxml) -- transitional conformance forbids mixed namespaces"],control)?;}}
 for relationships in snapshot.opc.relationships.values(){for relationship in relationships.iter(){control.checkpoint()?;let rel_type=relationship.rel_type.to_string_owner();let id=relationship.id.to_string_owner();if strict&&rel_type.starts_with(TRANS_REL){diagnostic(&mut output,subset,"non-strict-relationship-base",Severity::Error,&["relationship ",&id," uses the transitional relationship base ",TRANS_REL," -- strict conformance requires ",STRICT_REL],control)?;}
 if !strict&&rel_type.starts_with(STRICT_REL){diagnostic(&mut output,subset,"strict-ns-present",Severity::Error,&["relationship ",&id," uses a strict-family relationship base (purl.oclc.org/ooxml) -- transitional conformance forbids it"],control)?;}}}Ok(output)
}


use semio_framework_value::{ValueError, ValueRefusalKind};
