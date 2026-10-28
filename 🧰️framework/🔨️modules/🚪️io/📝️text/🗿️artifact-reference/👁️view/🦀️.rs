//! 👁️ Canonical artifact URI bytes borrow the original four fields without constructing text.
use super::ArtifactRef;
/// 🧵️ An immutable byte projection supports bounded hashing, comparison and native wire cursors.
pub struct ArtifactReferenceUriView<'a>{parts:[&'a str;7]}
impl<'a> ArtifactReferenceUriView<'a>{
 /// 📌️ Preserves the exact canonical field order and separators of the defining text codec.
 pub fn new(reference:&'a ArtifactRef)->Self{Self{parts:[reference.artifact_id.as_str(),"!",reference.dialect.artifact_kind.as_str(),"@",reference.dialect.standard.as_str(),"/",reference.dialect.subset.as_str()]}}
 /// 🪢️ Borrows one canonical text segment directly for native fragmented wire encoders.
 pub fn part(&self,index:usize)->Option<&'a str>{self.parts.get(index).copied()}
 /// 📏️ Refuses arithmetic overflow without allocating or scanning the field bodies.
 pub fn byte_len(&self)->Option<usize>{self.parts.iter().try_fold(0usize,|length,part|length.checked_add(part.len()))}
 /// 📤️ Copies at most the caller's output extent, including fragments within UTF8 code points.
 pub fn read_bytes(&self,mut offset:usize,output:&mut[u8])->usize{let mut written=0;for part in self.parts{let part=part.as_bytes();if offset>=part.len(){offset-=part.len();continue}let count=(part.len()-offset).min(output.len()-written);output[written..written+count].copy_from_slice(&part[offset..offset+count]);written+=count;offset=0;if written==output.len(){break}}written}
}
#[cfg(test)]
#[path="tests.rs"]
mod tests;
