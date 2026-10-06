//! 📏️ Actual DAG literal fields bound encoded output without creating owned mirrors.
use crate::*;
use graph::manifest::PropertyValue;
use semio_framework_value::{DslValue,ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,SqliteSnapshotPhase};
type Result<T>=std::result::Result<T,ValueError>;
fn overflow()->ValueError{ValueError::new(ValueRefusalKind::OwnershipLimit,"DAG native ceiling overflow")}
struct Bound<'a,'p>{control:&'a mut SqliteSnapshotControl<'p>,bytes:usize,semantic:usize,items:usize,maximum_depth:usize}
impl Bound<'_,'_>{
 fn add(&mut self,bytes:usize)->Result<()>{self.bytes=self.bytes.checked_add(bytes).ok_or_else(overflow)?;if self.bytes>self.control.limits().max_file_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"DAG native encoded ceiling exceeds file limit"))}Ok(())}
 fn item(&mut self)->Result<()>{self.items=self.items.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"DAG native entity count overflow"))?;self.control.check_rows(self.items)?;if self.items%256==0{self.control.checkpoint(SqliteSnapshotPhase::EncodeNative,self.items,0)?;}Ok(())}
 fn scalar(&mut self)->Result<()>{self.item()?;self.semantic=self.semantic.checked_add(8).ok_or_else(overflow)?;self.control.check_value_bytes(self.semantic)?;self.add(64)}
 fn record(&mut self)->Result<()>{self.item()?;self.add(4096)}
 fn text(&mut self,text:&str)->Result<()>{self.octets(text.as_bytes())}
 fn octets(&mut self,bytes:&[u8])->Result<()>{self.item()?;self.semantic=self.semantic.checked_add(bytes.len()).ok_or_else(overflow)?;self.control.check_value_bytes(self.semantic)?;self.add(bytes.len().checked_mul(6).and_then(|value|value.checked_add(64)).ok_or_else(overflow)?)?;self.control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,bytes.len())?;let mut completed=0;for piece in bytes.chunks(256){completed+=piece.len();self.control.checkpoint(SqliteSnapshotPhase::EncodeNative,completed,bytes.len())?;}Ok(())}
 fn depth(&self,depth:usize)->Result<()>{if depth>self.maximum_depth{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"DAG native value nesting exceeds encoding depth"))}Ok(())}
 fn property(&mut self,value:&PropertyValue,depth:usize)->Result<()>{self.depth(depth)?;match value{PropertyValue::Null|PropertyValue::Bool(_)|PropertyValue::Number(_)=>self.scalar(),PropertyValue::String(value)=>self.text(value),PropertyValue::Array(values)=>{self.scalar()?;for value in values{self.property(value,depth+1)?;}Ok(())},PropertyValue::Object(values)=>{self.scalar()?;for(key,value)in values{self.text(key)?;self.property(value,depth+1)?;}Ok(())}}}
 fn intrinsic(&mut self,value:&DslValue,depth:usize)->Result<()>{self.depth(depth)?;match value{DslValue::Null|DslValue::Bool(_)|DslValue::Number(_)=>self.scalar(),DslValue::String(value)=>self.text(value),DslValue::Bytes(value)=>self.octets(value),DslValue::Array(values)=>{self.scalar()?;for value in values{self.intrinsic(value,depth+1)?;}Ok(())},DslValue::Object(values)=>{self.scalar()?;for(key,value)in values{self.text(key)?;self.intrinsic(value,depth+1)?;}Ok(())}}}
 fn port(&mut self,value:&IoPortSpec)->Result<()>{self.record()?;for text in [&value.id,&value.label,&value.code,&value.abbreviation,&value.full_name,&value.cardinality]{self.text(text)?;}for text in [&value.value_type,&value.artifact_kind].into_iter().flatten(){self.text(text)?;}for value in [&value.default,&value.value].into_iter().flatten(){self.intrinsic(value,0)?;}self.scalar()?;self.scalar()?;self.scalar()?;Ok(())}
 fn ports(&mut self,values:&[IoPortSpec])->Result<()>{for value in values{self.port(value)?;}Ok(())}
 fn node(&mut self,value:&DagNodeSpec)->Result<()>{self.record()?;for text in [&value.id,&value.name,&value.abbreviation,&value.icon]{self.text(text)?;}for _ in 0..4{self.scalar()?;}if let Some(value)=&value.operator_kind{self.text(value)?;}for(key,value)in &value.properties{self.text(key)?;self.property(value,0)?;}self.record()?;match &value.kind{
  DagNodeKind::Computation{inputs,outputs,..}=>{self.ports(inputs)?;self.ports(outputs)?;self.scalar()?;self.scalar()?;},
  DagNodeKind::Cluster{inputs,outputs}=>{self.ports(inputs)?;self.ports(outputs)?;},
  DagNodeKind::AppInstance{instance_id,plugin_id,app_id,icon,inputs,outputs}=>{for text in [instance_id,plugin_id,app_id,icon]{self.text(text)?;}self.ports(inputs)?;self.ports(outputs)?;},
  DagNodeKind::Slider{output,..}=>{for _ in 0..4{self.scalar()?;}self.port(output)?;},
  DagNodeKind::Select{options,output,..}=>{self.scalar()?;for text in options{self.text(text)?;}self.port(output)?;},
  DagNodeKind::Screen{media,input}=>{if let Some(media)=media{self.record()?;self.scalar()?;self.text(&media.src)?;}self.port(input)?;},
  DagNodeKind::Note{text,output}=>{self.text(text)?;self.port(output)?;},
  DagNodeKind::Image{src,output}=>{self.text(src)?;self.port(output)?;},
  DagNodeKind::Preview{content,expanded,input}=>{self.record()?;match content{DagPreviewContent::Empty=>{},DagPreviewContent::Scalar{text}=>self.text(text)?,DagPreviewContent::Image{src}=>self.text(src)?,DagPreviewContent::Tree{json}=>self.intrinsic(json,0)?,}for text in expanded{self.text(text)?;}self.port(input)?;},
  DagNodeKind::Action{label,input}=>{self.text(label)?;self.port(input)?;},
  DagNodeKind::Export{label,format,input}=>{self.text(label)?;self.text(format)?;self.port(input)?;},
 }Ok(())}
}
pub(super) fn preflight(snapshot:&DagSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<()>{
 control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;
 let maximum_depth=match encoding{SnapshotEncoding::Binary=>usize::from(store::PackEncodeOptions::default().limits.max_depth),SnapshotEncoding::Text=>semio_framework_diagnostic::Limits::default().max_depth};
 let mut bound=Bound{control,bytes:0,semantic:0,items:0,maximum_depth};bound.item()?;bound.add(65536)?;bound.record()?;bound.text(&snapshot.schema)?;for node in &snapshot.nodes{bound.node(node)?;}for edge in &snapshot.edges{bound.record()?;for text in [&edge.id,&edge.source,&edge.target]{bound.text(text)?;}bound.scalar()?;for(key,value)in &edge.properties{bound.text(key)?;bound.property(value,0)?;}}bound.control.checkpoint(SqliteSnapshotPhase::EncodeNative,bound.items,bound.items)
}
