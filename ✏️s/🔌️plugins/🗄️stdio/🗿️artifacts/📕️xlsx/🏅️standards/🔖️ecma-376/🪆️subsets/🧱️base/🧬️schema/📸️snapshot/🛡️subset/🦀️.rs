//! 🛡️ Borrowed XLSX profile checks with controlled paths and diagnostic ownership.
use super::XlsxSnapshot;
use semio_framework_os_kernel::{NativeEncodeControl,sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase}};
use semio_s_artifact_stdio_xml::schema::snapshot::XmlNode;
use dsl::{Diagnostic,FaultCode,FaultScope,Severity,TextSpan};
use std::fmt::{Arguments,Write};
const WORKSHEET:&str="application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml";
const VML:&str="application/vnd.openxmlformats-officedocument.vmlDrawing";
const OFFICE:&str="http://schemas.openxmlformats.org/officeDocument/2006/relationships/officeDocument";
const OFFICE_STRICT:&str="http://purl.oclc.org/ooxml/officeDocument/relationships/officeDocument";
const SHEET:&str="http://schemas.openxmlformats.org/officeDocument/2006/relationships/worksheet";
const SHEET_STRICT:&str="http://purl.oclc.org/ooxml/officeDocument/relationships/worksheet";
fn equal(left:&str,right:&str,control:&mut NativeEncodeControl<'_>)->Result<bool,String>{
 if left.len()!=right.len(){return Ok(false)}control.scoped_stage(|control|{control.begin_stage(left.len())?;for(a,b)in left.as_bytes().chunks(256).zip(right.as_bytes().chunks(256)){let same=a==b;control.advance(a.len())?;if !same{return Ok(false)}}Ok(true)})
}
fn scan(text:&str,control:&mut NativeEncodeControl<'_>)->Result<Option<usize>,String>{
 control.scoped_stage(|control|{control.begin_stage(text.len())?;let mut slash=None;for(position,byte)in text.bytes().enumerate(){if byte==b'/'{slash=Some(position)}control.step()?;}Ok(slash)})
}
fn path(owner:&str,target:&str,control:&mut NativeEncodeControl<'_>)->Result<String,String>{
 if let Some(target)=target.strip_prefix('/'){return control.copy_text(target)}
 let base=scan(owner,control)?.map_or("",|position|&owner[..=position]);
 let mut parts:Vec<&str>=Vec::new();
 for text in[base,target]{control.scoped_stage(|control|->Result<(),String>{control.begin_stage(text.len())?;let mut start=0;for(position,byte)in text.bytes().enumerate(){if byte==b'/'{segment(&text[start..position],&mut parts,control)?;start=position+1;}control.step()?;}segment(&text[start..],&mut parts,control)})?;}
 let length=control.scoped_stage(|control|->Result<usize,String>{control.begin_stage(parts.len())?;let mut length=parts.len().saturating_sub(1);for part in &parts{length=length.checked_add(part.len()).ok_or("XLSX relationship path size overflow")?;control.step()?;}Ok(length)})?;
 control.charge(length)?;let mut output=String::new();output.try_reserve_exact(length).map_err(|_|"XLSX relationship path allocation")?;
 control.scoped_stage(|control|->Result<(),String>{control.begin_stage(length)?;for(position,part)in parts.iter().enumerate(){if position!=0{output.push('/');control.step()?;}append(&mut output,part,control)?;}Ok(())})?;Ok(output)
}
fn segment<'a>(text:&'a str,parts:&mut Vec<&'a str>,control:&mut NativeEncodeControl<'_>)->Result<(),String>{
 match text{""|"."=>{},".."=>{parts.pop();},_=>{if parts.len()==parts.capacity(){control.charge(64*std::mem::size_of::<&str>())?;parts.try_reserve_exact(64).map_err(|_|"XLSX relationship path frontier allocation")?;}parts.push(text);}}Ok(())
}
fn append(output:&mut String,text:&str,control:&mut NativeEncodeControl<'_>)->Result<(),String>{
 let mut start=0;while start<text.len(){let mut end=start.saturating_add(256).min(text.len());while !text.is_char_boundary(end){end-=1;}output.push_str(&text[start..end]);control.advance(end-start)?;start=end;}Ok(())
}
struct Message<'a,'p,'o>{control:&'a mut NativeEncodeControl<'p>,output:Option<&'o mut String>,length:usize,error:Option<String>}
impl Write for Message<'_, '_, '_>{fn write_str(&mut self,text:&str)->std::fmt::Result{
 let result=(||->Result<(),String>{self.length=self.length.checked_add(text.len()).ok_or("XLSX diagnostic size overflow")?;if self.length>self.control.maximum_bytes(){return Err("XLSX diagnostic exceeds caller limit".into())}if let Some(output)=self.output.as_mut(){append(output,text,self.control)}else{for chunk in text.as_bytes().chunks(256){self.control.advance(chunk.len())?;}Ok(())}})();if let Err(error)=result{self.error=Some(error);Err(std::fmt::Error)}else{Ok(())}
}}
fn message(arguments:Arguments<'_>,control:&mut NativeEncodeControl<'_>)->Result<String,String>{
 control.scoped_stage(|control|{control.begin_stage(0)?;let mut measure=Message{control,output:None,length:0,error:None};if measure.write_fmt(arguments).is_err(){return Err(measure.error.unwrap_or_else(||"XLSX diagnostic formatting failed".into()))}let length=measure.length;control.begin_stage(length)?;control.charge(length)?;let mut output=String::new();output.try_reserve_exact(length).map_err(|_|"XLSX diagnostic allocation")?;let mut writer=Message{control,output:Some(&mut output),length:0,error:None};if writer.write_fmt(arguments).is_err(){return Err(writer.error.unwrap_or_else(||"XLSX diagnostic formatting failed".into()))}Ok(output)})
}
fn diagnostic(output:&mut Vec<Diagnostic>,subset:&str,suffix:&str,severity:Severity,arguments:Arguments<'_>,control:&mut NativeEncodeControl<'_>)->Result<(),String>{
 control.charge(std::mem::size_of::<Diagnostic>())?;output.try_reserve_exact(1).map_err(|_|"XLSX diagnostic frontier allocation")?;let code=message(format_args!("stdio.xlsx.{subset}.{suffix}"),control)?;let message=message(arguments,control)?;output.push(Diagnostic{code:FaultCode::new(code),severity,span:TextSpan::at(1,1),message,expected:None,scope:FaultScope::default()});Ok(())
}
fn content_type<'a>(snapshot:&'a XlsxSnapshot,target:&str,control:&mut NativeEncodeControl<'_>)->Result<Option<&'a str>,String>{
 let key=target.trim_start_matches('/');let mut dot=None;
 control.scoped_stage(|control|->Result<(),String>{control.begin_stage(key.len())?;for(position,byte)in key.bytes().enumerate(){if byte==b'.'{dot=Some(position)}control.step()?;}Ok(())})?;
 for(name,value)in &snapshot.opc.content_types.overrides{control.checkpoint()?;if name.starts_with('/')&&equal(&name[1..],key,control)?{return Ok(Some(value))}}
 let extension=dot.map_or(key,|position|&key[position+1..]);
 for(name,value)in &snapshot.opc.content_types.defaults{control.checkpoint()?;if name.len()==extension.len(){let same=control.scoped_stage(|control|->Result<bool,String>{control.begin_stage(name.len())?;for(a,b)in name.as_bytes().chunks(256).zip(extension.as_bytes().chunks(256)){let same=a.eq_ignore_ascii_case(b);control.advance(a.len())?;if !same{return Ok(false)}}Ok(true)})?;if same{return Ok(Some(value))}}}
 Ok(None)
}
fn check(snapshot:&XlsxSnapshot,subset:&str,control:&mut NativeEncodeControl<'_>)->Result<Vec<Diagnostic>,String>{
 let strict=subset=="strict";let(ns,rns,title,iso)=if strict{("http://purl.oclc.org/ooxml/spreadsheetml/main","http://purl.oclc.org/ooxml/officeDocument/relationships","Strict","ISO/IEC 29500-1")}else{("http://schemas.openxmlformats.org/spreadsheetml/2006/main","http://schemas.openxmlformats.org/officeDocument/2006/relationships","Transitional","ISO/IEC 29500-4")};
 let mut output=Vec::new();let mut main=None;
 for expected in[OFFICE,OFFICE_STRICT]{for(owner,relationships)in &snapshot.opc.relationships{control.checkpoint()?;if !owner.is_empty(){continue}for relationship in relationships{control.checkpoint()?;if equal(&relationship.rel_type,expected,control)?{main=Some(path("",&relationship.target,control)?);break}}}if main.is_some(){break}}
 let mut root=None;
 if let Some(main)=main.as_deref(){for part in &snapshot.xml_parts{control.checkpoint()?;if equal(&part.path,main.trim_start_matches('/'),control)?{root=part.document.root.as_ref();break}}}
 let Some(XmlNode::Element{name,attrs,..})=root else{diagnostic(&mut output,subset,"namespace-mismatch",Severity::Error,format_args!("xl/workbook.xml is missing or unparsable as XML -- cannot verify {iso} {title} conformance"),control)?;return Ok(output)};
 if name!="workbook"&&!name.ends_with(":workbook"){diagnostic(&mut output,subset,"namespace-mismatch",Severity::Error,format_args!("xl/workbook.xml is missing or unparsable as XML -- cannot verify {iso} {title} conformance"),control)?;return Ok(output)}
 let(mut xmlns,mut xmlns_r,mut conformance)=(None,None,None);
 control.scoped_stage(|control|->Result<(),String>{control.begin_stage(attrs.len())?;for attr in attrs{match attr.name.as_str(){"xmlns" if xmlns.is_none()=>xmlns=Some(attr.value.as_str()),"xmlns:r" if xmlns_r.is_none()=>xmlns_r=Some(attr.value.as_str()),"conformance" if conformance.is_none()=>conformance=Some(attr.value.as_str()),_=>{}}control.step()?;}Ok(())})?;
 if !matches!(xmlns,Some(value)if equal(value,ns,control)?){diagnostic(&mut output,subset,"namespace-mismatch",Severity::Error,format_args!("xl/workbook.xml root xmlns is {xmlns:?}, expected the {title} SpreadsheetML namespace {ns:?} ({iso})"),control)?;}
 if !matches!(xmlns_r,Some(value)if equal(value,rns,control)?){diagnostic(&mut output,subset,"relationships-namespace-mismatch",Severity::Error,format_args!("xl/workbook.xml root xmlns:r is {xmlns_r:?}, expected the {title} officeDocument relationships namespace {rns:?}"),control)?;}
 if strict&&conformance!=Some("strict"){diagnostic(&mut output,subset,"conformance-attribute",Severity::Error,format_args!("xl/workbook.xml workbook@conformance is {conformance:?}, expected \"strict\" (ISO/IEC 29500-1 §12.3.24)"),control)?;}
 if !strict&&conformance==Some("strict"){diagnostic(&mut output,subset,"conformance-attribute",Severity::Error,format_args!("xl/workbook.xml workbook@conformance is \"strict\" -- a document that declares Strict conformance cannot be honestly stamped Transitional"),control)?;}
 if strict{for(path,content_type)in snapshot.opc.parts.iter().map(|part|(&part.path,&part.content_type)).chain(snapshot.xml_parts.iter().map(|part|(&part.path,&part.content_type))){control.checkpoint()?;if equal(content_type,VML,control)?{diagnostic(&mut output,subset,"vml-forbidden",Severity::Error,format_args!("part {path} declares legacy VML drawing content type {VML:?} -- ISO/IEC 29500-1 Strict removes VML support entirely"),control)?;}}}
 if let Some(main)=main{for(owner,relationships)in &snapshot.opc.relationships{control.checkpoint()?;if !equal(owner,&main,control)?{continue}for relationship in relationships{control.checkpoint()?;if equal(&relationship.rel_type,SHEET,control)?||equal(&relationship.rel_type,SHEET_STRICT,control)?{let path=path(&main,&relationship.target,control)?;let content_type=content_type(snapshot,&path,control)?;if !matches!(content_type,Some(value)if equal(value,WORKSHEET,control)?){diagnostic(&mut output,subset,"worksheet-content-type-missing",Severity::Warning,format_args!("worksheet part {path} resolves content type {content_type:?}, expected {WORKSHEET:?} (ECMA-376 Part 1 §12.3.24)"),control)?;}}}}}
 Ok(output)
}
pub(super) fn validate(snapshot:&XlsxSnapshot,subset:&str,control:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{
 let limits=control.limits();let mut callback=|event:semio_framework_value::native_encoding::NativeEncodeProgress|control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,event.completed,event.total).is_ok();let mut native=NativeEncodeControl::new(limits.max_value_bytes,&mut callback);
 let diagnostics=check(snapshot,subset,&mut native).map_err(store::io_schema::IoError::from)?;if diagnostics.iter().any(|diagnostic|matches!(diagnostic.severity,Severity::Error|Severity::Fatal)){Err(store::io_schema::IoError{message:"XLSX owned snapshot violates its exact profile".into(),diagnostics})}else{Ok(store::io_schema::IoOutcome{value:(),diagnostics})}
}

