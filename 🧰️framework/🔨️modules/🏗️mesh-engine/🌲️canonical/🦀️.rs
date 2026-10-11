//! 🌲️ Canonical native JSON trees of mesh payloads keep their camelCase wire shape and skip empty optional members.
use super::{ComponentReferenceTable,MeshAttribute,MeshAttributeDomain,MeshAttributeInterpolation,MeshAttributeSemantic,MeshData,MeshTexture};
use semio_framework_pack_json::{ArtifactCanonicalJsonNode as Node,ArtifactCanonicalJsonText as Text,ArtifactCanonicalJsonTree as Tree};
use pack::value::{ValueError,ValueRefusalKind};
fn absent(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,reason)}
impl Tree for MeshAttributeDomain {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(match self{Self::Vertex=>"vertex",Self::Corner=>"corner",Self::Face=>"face",Self::Edge=>"edge"}))}}
impl Tree for MeshAttributeSemantic {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(match self{Self::Normal=>"normal",Self::Uv=>"uv",Self::Color=>"color",Self::Material=>"material",Self::Custom=>"custom"}))}}
impl Tree for MeshAttributeInterpolation {fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::String(match self{Self::Linear=>"linear",Self::Nearest=>"nearest",Self::Constant=>"constant"}))}}
impl MeshAttribute {
 fn canonical_member(&self,ordinal:usize)->Option<(&'static str,&dyn Tree)>{[Some(("domain",&self.domain as&dyn Tree)),Some(("semantic",&self.semantic as&dyn Tree)),Some(("interpolation",&self.interpolation as&dyn Tree)),Some(("values",&self.values as&dyn Tree)),self.indices.as_ref().map(|indices|("indices",indices as&dyn Tree))].into_iter().flatten().nth(ordinal)}
}
impl Tree for MeshAttribute {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(4+usize::from(self.indices.is_some())))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{self.canonical_member(ordinal).map(|(_,value)|value).ok_or_else(||absent("canonical mesh attribute ordinal is absent"))}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{self.canonical_member(ordinal).map(|(key,_)|Text::from(key)).ok_or_else(||absent("canonical mesh attribute key is absent"))}
}
impl Tree for MeshTexture {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(2))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{match ordinal{0=>Ok(&self.mime),1=>Ok(&self.bytes),_=>Err(absent("canonical mesh texture ordinal is absent"))}}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{match ordinal{0=>Ok("mime".into()),1=>Ok("bytes".into()),_=>Err(absent("canonical mesh texture key is absent"))}}
}
impl Tree for ComponentReferenceTable {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object(self.len()))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{self.iter().nth(ordinal).map(|(_,labels)|labels as&dyn Tree).ok_or_else(||absent("canonical component reference ordinal is absent"))}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{self.iter().nth(ordinal).map(|(key,_)|Text::from(key.as_str())).ok_or_else(||absent("canonical component reference key is absent"))}
}
impl MeshData {
 fn canonical_member(&self,ordinal:usize)->Option<(&'static str,&dyn Tree)>{
  fn non_empty<'a,T>(key:&'static str,values:&'a Vec<T>)->Option<(&'static str,&'a dyn Tree)> where Vec<T>:Tree {(!values.is_empty()).then_some((key,values as&dyn Tree))}
  [Some(("positions",&self.positions as&dyn Tree)),Some(("normals",&self.normals as&dyn Tree)),Some(("colors",&self.colors as&dyn Tree)),Some(("indices",&self.indices as&dyn Tree)),non_empty("uvs",&self.uvs),non_empty("faceIds",&self.face_ids),non_empty("vertexIds",&self.vertex_ids),non_empty("edgePositions",&self.edge_positions),non_empty("edgeIds",&self.edge_ids),non_empty("edgeUvs",&self.edge_uvs),non_empty("edgeIsSeam",&self.edge_is_seam),self.paint_texture_base64.as_ref().map(|texture|("paintTextureBase64",texture as&dyn Tree)),(!self.attributes.is_empty()).then_some(("attributes",&self.attributes as&dyn Tree)),(!self.materials.is_empty()).then_some(("materials",&self.materials as&dyn Tree)),(!self.textures.is_empty()).then_some(("textures",&self.textures as&dyn Tree)),(!self.component_references.is_empty()).then_some(("componentReferences",&self.component_references as&dyn Tree))].into_iter().flatten().nth(ordinal)
 }
}
impl Tree for MeshData {
 fn canonical_tree_node(&self)->Result<Node<'_>,ValueError>{Ok(Node::Object((0..16).take_while(|ordinal|self.canonical_member(*ordinal).is_some()).count()))}
 fn canonical_tree_child(&self,ordinal:usize)->Result<&dyn Tree,ValueError>{self.canonical_member(ordinal).map(|(_,value)|value).ok_or_else(||absent("canonical mesh member ordinal is absent"))}
 fn canonical_tree_key(&self,ordinal:usize)->Result<Text<'_>,ValueError>{self.canonical_member(ordinal).map(|(key,_)|Text::from(key)).ok_or_else(||absent("canonical mesh member key is absent"))}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
