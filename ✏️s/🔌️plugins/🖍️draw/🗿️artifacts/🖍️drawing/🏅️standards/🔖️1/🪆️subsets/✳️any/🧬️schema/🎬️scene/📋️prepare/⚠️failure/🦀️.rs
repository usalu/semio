//! 🧬️ Scene failure projection lends each original typed payload before paid Job erasure.
use super::{DocumentSceneError as E,DocumentSceneMessage as M};
use semio_framework_job::{OriginalDomainFailure,OriginalDomainProjection,OriginalDomainView as V};
use semio_framework_value::{RetainedCloneProgress,ValueError,ValueRefusalKind};
use semio_s_artifact_draw_drawing_geometry::raster::PathRasterError as R;
use semio_framework_2d::{flatten::PathFlattenError as F,stroke::StrokeOutlineError as S};
use semio_framework_pixels::editing::PixelEditError as P;
use semio_framework_pixels::affine_sampling::AffineImageError as A;

impl E{
 /// 📢️ Borrows original literal or shared cause prose without formatting or payload copies.
 pub fn message(&self)->&str{match self{Self::Invalid(message)=>message.as_ref(),Self::PathRaster(error)=>error.message(),Self::AffineImage(error)=>error.message(),Self::Incomplete=>"Document scene preparation incomplete",Self::Cancelled=>"Document scene preparation cancelled"}}
}
impl OriginalDomainFailure for E{
 fn domain_code(&self)->&str{"drawing.scene"}
 fn domain_message(&self)->&str{self.message()}
 fn domain_retained_progress(&self)->RetainedCloneProgress{Default::default()}
}
#[derive(Clone,Copy)]
enum Node<'a>{Scene(&'a E),Message(&'a M),Raster(&'a R),Affine(&'a A),Leaf(&'static str,&'a str),Retirement(&'a str,&'a str,Option<&'a str>),Text(&'a str)}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"scene original failure projection ordinal changed")}
impl<'a> Node<'a>{
 fn kind(self)->Result<&'static str,ValueError>{Ok(match self{Self::Scene(error)=>match error{E::Invalid(_)=>"invalid",E::PathRaster(_)=>"pathRaster",E::AffineImage(_)=>"affineImage",E::Incomplete=>"incomplete",E::Cancelled=>"cancelled"},Self::Message(message)=>match message{M::Literal(_)=>"literal",M::Owned(_)=>"owned"},Self::Raster(error)=>match error{R::Invalid(_)=>"invalid",R::Flatten(_)=>"flatten",R::Stroke(_)=>"stroke",R::Pixels(_)=>"pixels",R::Retirement{..}=>"retirement",R::OwnedRetirement(_)=>"ownedRetirement",R::Incomplete=>"incomplete",R::Cancelled=>"cancelled"},Self::Affine(error)=>match error{A::Invalid(_)=>"invalid",A::Pixels(_)=>"pixels",A::Incomplete=>"incomplete",A::Cancelled=>"cancelled"},Self::Leaf(kind,_)=>kind,_=>return Err(invalid())})}
 fn original(self)->Option<Self>{match self{
  Self::Scene(E::Invalid(message))=>Some(Self::Message(message)),Self::Scene(E::PathRaster(error))=>Some(Self::Raster(error)),Self::Scene(E::AffineImage(error))=>Some(Self::Affine(error)),
  Self::Affine(A::Pixels(error))=>Some(Self::Leaf(match error{P::Invalid(_)=>"invalid",P::Incomplete=>"incomplete",P::Cancelled=>"cancelled"},error.message())),
  Self::Raster(R::Flatten(error))=>Some(Self::Leaf(match error{F::Invalid(_)=>"invalid",F::Incomplete=>"incomplete",F::Cancelled=>"cancelled"},error.message())),
  Self::Raster(R::Stroke(error))=>Some(Self::Leaf(match error{S::Invalid(_)=>"invalid",S::Incomplete=>"incomplete",S::Cancelled=>"cancelled"},error.message())),
  Self::Raster(R::Pixels(error))=>Some(Self::Leaf(match error{P::Invalid(_)=>"invalid",P::Incomplete=>"incomplete",P::Cancelled=>"cancelled"},error.message())),
  Self::Raster(R::Retirement{kind,message,posting_refusal})=>Some(Self::Retirement(kind,message,*posting_refusal)),
  Self::Raster(R::OwnedRetirement(cause))=>Some(Self::Retirement(cause.kind,&cause.message,cause.posting_refusal)),_=>None
 }}
 fn len(self)->Result<usize,ValueError>{match self{Self::Scene(_)|Self::Raster(_)|Self::Affine(_)=>Ok(2+usize::from(self.original().is_some())),Self::Message(_)|Self::Leaf(..)=>Ok(2),Self::Retirement(_,_,posting)=>Ok(2+usize::from(posting.is_some())),Self::Text(_)=>Err(invalid())}}
 fn key(self,index:usize)->Result<&'static str,ValueError>{if index>=self.len()?{return Err(invalid())}match self{Self::Message(_)=>Ok(["kind","text"][index]),Self::Retirement(..)=>Ok(["kind","message","postingRefusal"][index]),_=>Ok(["kind","message","original"][index])}}
 fn child(self,index:usize)->Result<Self,ValueError>{match self.key(index)?{
  "kind"=>Ok(Self::Text(match self{Self::Retirement(kind,..)=>kind,_=>self.kind()?})),
  "message"=>Ok(Self::Text(match self{Self::Scene(error)=>error.message(),Self::Raster(error)=>error.message(),Self::Affine(error)=>error.message(),Self::Leaf(_,message)|Self::Retirement(_,message,_)=>message,_=>return Err(invalid())})),
  "text"=>match self{Self::Message(message)=>Ok(Self::Text(message.as_ref())),_=>Err(invalid())},
  "postingRefusal"=>match self{Self::Retirement(_,_,Some(message))=>Ok(Self::Text(message)),_=>Err(invalid())},
  "original"=>self.original().ok_or_else(invalid),_=>Err(invalid())
 }}
 fn at(mut self,path:&[usize])->Result<Self,ValueError>{for &index in path{self=self.child(index)?}Ok(self)}
}
impl OriginalDomainProjection for E{
 fn projection_identity(&self)->usize{self as*const Self as usize}
 fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{let node=Node::Scene(self).at(path)?;match node{Node::Text(text)=>Ok(V::IntrinsicText(text)),node=>Ok(V::IntrinsicObject(node.len()?))}}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{Node::Scene(self).at(path)?.key(index)}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
