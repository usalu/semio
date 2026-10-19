//! 🌲️ Diagnostic publication lends original native fields to the shared canonical JSON producer.
use semio_framework_diagnostic::{Diagnostic,ExpectedSet,FaultCode,FaultScope,Severity,TextSpan};
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node,ArtifactCanonicalJsonText as Text,ArtifactCanonicalJsonTree as Tree};
use semio_framework_value::{ValueError,ValueRefusalKind};

fn absent()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"original diagnostic canonical field is absent")}
macro_rules! view{
 ($name:ident,$body:ty)=>{
  #[repr(transparent)]
  pub(super) struct $name($body);
  impl $name{pub(super) fn original(body:&$body)->&Self{unsafe{&*(body as*const $body).cast::<Self>()}}}
 };
}
view!(DiagnosticTree,Diagnostic);
view!(ExpectedTree,ExpectedSet);
view!(ScopeTree,FaultScope);
view!(SpanTree,TextSpan);
view!(CodeTree,FaultCode);
view!(SeverityTree,Severity);

impl Tree for CodeTree{fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Text(Text::Contiguous(&self.0.0)))}}
impl Tree for SeverityTree{fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Text(Text::Contiguous(match self.0{Severity::Info=>"info",Severity::Warning=>"warning",Severity::Error=>"error",Severity::Fatal=>"fatal"})))}}
impl Tree for SpanTree{
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(3))}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{["column","length","line"].get(ordinal).map(|key|Text::Contiguous(key)).ok_or_else(absent)}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{Ok(match ordinal{0=>&self.0.column,1=>&self.0.length,2=>&self.0.line,_=>return Err(absent())})}
}
impl Tree for ExpectedTree{
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(3))}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{["keys","keywords","tokens"].get(ordinal).map(|key|Text::Contiguous(key)).ok_or_else(absent)}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{Ok(match ordinal{0=>&self.0.keys,1=>&self.0.keywords,2=>&self.0.tokens,_=>return Err(absent())})}
}
impl ScopeTree{fn fields(&self)->[(&'static str,Option<&String>);5]{[("appId",self.0.app_id.as_ref()),("bodyKey",self.0.body_key.as_ref()),("instanceId",self.0.instance_id.as_ref()),("module",self.0.module.as_ref()),("pluginId",self.0.plugin_id.as_ref())]}}
impl Tree for ScopeTree{
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(self.fields().iter().filter(|(_,value)|value.is_some()).count()))}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{self.fields().into_iter().filter(|(_,value)|value.is_some()).nth(ordinal).map(|(key,_)|Text::Contiguous(key)).ok_or_else(absent)}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{self.fields().into_iter().filter_map(|(_,value)|value).nth(ordinal).map(|value|value as&dyn Tree).ok_or_else(absent)}
}
impl DiagnosticTree{fn field(&self,ordinal:usize)->Result<usize,ValueError>{if ordinal>=5+usize::from(self.0.expected.is_some()){return Err(absent())}Ok(ordinal+usize::from(self.0.expected.is_none()&&ordinal>0))}}
impl Tree for DiagnosticTree{
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(5+usize::from(self.0.expected.is_some())))}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{Ok(Text::Contiguous(["code","expected","message","scope","severity","span"][self.field(ordinal)?]))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{Ok(match self.field(ordinal)?{0=>CodeTree::original(&self.0.code),1=>ExpectedTree::original(self.0.expected.as_ref().ok_or_else(absent)?),2=>&self.0.message,3=>ScopeTree::original(&self.0.scope),4=>SeverityTree::original(&self.0.severity),5=>SpanTree::original(&self.0.span),_=>return Err(absent())})}
}
