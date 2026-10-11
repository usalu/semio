//! 🌲️ Canonical native JSON trees of graph manifests keep their `ToValue` wire shapes and field order.
use super::{PortDirection,PropertyBag,PropertyDef,PropertyKind,PropertyValue};
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node,ArtifactCanonicalJsonText as Text,ArtifactCanonicalJsonTree as Tree};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn absent(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
impl Tree for PropertyKind {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(match self{Self::Data=>"data",Self::Derived=>"derived"}))}
}
impl Tree for PortDirection {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(match self{Self::In=>"in",Self::Out=>"out"}))}
}
impl Tree for PropertyValue {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(match self{Self::Null=>Node::Null,Self::Bool(value)=>Node::Bool(*value),Self::Number(value)=>Node::F64(*value),Self::String(value)=>Node::String(value),Self::Array(values)=>Node::Array(values.len()),Self::Object(values)=>return values.canonical_tree_node()})}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{match self{Self::Array(values)=>values.get(ordinal).map(|value|value as&dyn Tree).ok_or_else(||absent("canonical graph property array ordinal is absent")),Self::Object(values)=>values.canonical_tree_child(ordinal),_=>Err(absent("canonical graph property scalar has no child"))}}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{match self{Self::Object(values)=>values.canonical_tree_key(ordinal),_=>Err(absent("canonical graph property node has no key"))}}
}
impl Tree for PropertyBag {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(self.len()))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{self.iter().nth(ordinal).map(|(_,value)|value as&dyn Tree).ok_or_else(||absent("canonical graph property member ordinal is absent"))}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{self.iter().nth(ordinal).map(|(key,_)|Text::from(key.as_str())).ok_or_else(||absent("canonical graph property key is absent"))}
}
impl Tree for PropertyDef {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(if self.expr.is_some(){4}else{3}))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{match(ordinal,&self.expr){(0,_)=>Ok(&self.name),(1,_)=>Ok(&self.kind),(2,_)=>Ok(&self.value_type),(3,Some(expression))=>Ok(expression),_=>Err(absent("canonical graph property definition ordinal is absent"))}}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{match(ordinal,&self.expr){(0,_)=>Ok("name".into()),(1,_)=>Ok("kind".into()),(2,_)=>Ok("valueType".into()),(3,Some(_))=>Ok("expr".into()),_=>Err(absent("canonical graph property definition key is absent"))}}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
