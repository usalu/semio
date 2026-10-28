//! ⚠️ Original command Pack refusal retains its complete native failure and borrowed projection.
use semio_framework_os_kernel as store;
use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress};
#[derive(Debug,semio_framework_value::RetireOwned)]
pub struct OriginalCommandPackFailure{pub original:store::os_pack::PackRefusal}
impl semio_framework_job::OriginalDomainFailure for OriginalCommandPackFailure{
 fn domain_code(&self)->&str{"plugin.command-pack"}
 fn domain_message(&self)->&str{match &self.original{store::os_pack::PackRefusal::Malformed{detail,..}=>detail,store::os_pack::PackRefusal::RetainedMalformed{detail,..}|store::os_pack::PackRefusal::RetainedAllocation{detail,..}=>detail,store::os_pack::PackRefusal::ValueRefusal(error)|store::os_pack::PackRefusal::Io{error,..}=>&error.message,store::os_pack::PackRefusal::TextRefusal(error)=>&error.message,_=>"original intrinsic command refusal"}}
 fn domain_retained_progress(&self)->RetainedCloneProgress{match &self.original{store::os_pack::PackRefusal::ValueRefusal(error)|store::os_pack::PackRefusal::Io{error,..}=>error.retained_progress(),_=>Default::default()}}
}
impl semio_framework_job::OriginalDomainProjection for OriginalCommandPackFailure{
 fn projection_identity(&self)->usize{std::ptr::from_ref(&self.original)as usize}
 fn projection_view(&self,path:&[usize])->Result<semio_framework_job::OriginalDomainView<'_>,ValueError>{
  use semio_framework_job::OriginalDomainView as V;use store::os_pack::PackRefusal as E;use semio_framework_value::Number as N;
  let bad=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original command Pack projection changed");
  if path.is_empty(){return Ok(V::IntrinsicObject(2))}if path==[0]{return Ok(V::IntrinsicText(match &self.original{E::BadMagic=>"badMagic",E::UnsupportedVersion{..}=>"unsupportedVersion",E::UnknownRequiredFlags(_)=>"unknownRequiredFlags",E::Truncated(_)=>"truncated",E::ChecksumMismatch{..}=>"checksumMismatch",E::ContentHashMismatch=>"contentHashMismatch",E::LimitExceeded{..}=>"limitExceeded",E::RetainedMalformed{..}=>"retainedMalformed",E::RetainedAllocation{..}=>"retainedAllocation",E::Malformed{..}=>"malformed",E::NonCanonical(_)=>"nonCanonical",E::UnsupportedCodec(_)=>"unsupportedCodec",E::ValueRefusal(_)=>"valueRefusal",E::TextRefusal(_)=>"textRefusal",E::Io{..}=>"io",E::TransportAdmission{..}=>"transportAdmission"}))}
  let [1,rest @ ..]=path else{return Err(bad())};let number=|value:u64|V::IntrinsicNumber(N::UInt(value));
  Ok(match (&self.original,rest){
   (E::BadMagic|E::ContentHashMismatch,[])=>V::IntrinsicNull,
   (E::UnsupportedVersion{..},[])=>V::IntrinsicObject(2),(E::UnsupportedVersion{major,..},[0])=>number(u64::from(*major)),(E::UnsupportedVersion{minor,..},[1])=>number(u64::from(*minor)),
   (E::UnknownRequiredFlags(value),[])=>number(u64::from(*value)),(E::Truncated(value),[])=>number(*value),(E::UnsupportedCodec(value),[])=>number(u64::from(*value)),(E::NonCanonical(value),[])=>V::IntrinsicText(value),
   (E::ChecksumMismatch{..},[])=>V::IntrinsicObject(2),(E::ChecksumMismatch{segment,..},[0])=>V::IntrinsicText(segment),(E::ChecksumMismatch{offset,..},[1])=>number(*offset),
   (E::LimitExceeded{..},[])=>V::IntrinsicObject(2),(E::LimitExceeded{kind,..},[0])=>V::Enum(*kind as u32),(E::LimitExceeded{limit,..},[1])=>V::IntrinsicText(limit),
   (E::Malformed{..}|E::RetainedMalformed{..},[])=>V::IntrinsicObject(4),(E::RetainedAllocation{..},[])=>V::IntrinsicObject(5),
   (E::Malformed{kind,..}|E::RetainedMalformed{kind,..}|E::RetainedAllocation{kind,..},[0])=>V::Enum(*kind as u32),
   (E::Malformed{what,..}|E::RetainedMalformed{what,..}|E::RetainedAllocation{what,..},[1])=>V::IntrinsicText(what),
   (E::Malformed{offset,..}|E::RetainedMalformed{offset,..}|E::RetainedAllocation{offset,..},[2])=>number(*offset),
   (E::Malformed{detail,..},[3])=>V::IntrinsicText(detail),(E::RetainedMalformed{detail,..}|E::RetainedAllocation{detail,..},[3])=>V::IntrinsicText(detail),(E::RetainedAllocation{allocated_bytes,..},[4])=>number(*allocated_bytes as u64),
   (E::ValueRefusal(error),rest)=>return semio_framework_job::project_original_value_error(error,rest),
   (E::Io{..},[])=>V::IntrinsicObject(2),(E::Io{error,..},[0,rest @ ..])=>return semio_framework_job::project_original_value_error(error,rest),(E::Io{retry,..},[1])=>V::Enum(*retry as u32),
   (E::TextRefusal(_),[])=>V::IntrinsicObject(4),(E::TextRefusal(error),[0])=>V::Enum(error.kind as u32),(E::TextRefusal(error),[1])=>V::IntrinsicText(&error.message),(E::TextRefusal(_),[2])=>V::IntrinsicObject(3),(E::TextRefusal(error),[2,index])if *index<3=>number(u64::from([error.span.line,error.span.column,error.span.length][*index])),(E::TextRefusal(error),[3])=>error.expected.as_deref().map_or(V::IntrinsicNull,V::IntrinsicText),
   (E::TransportAdmission{..},[])=>V::IntrinsicObject(2),(E::TransportAdmission{category,..},[0])=>V::Enum(*category as u32),(E::TransportAdmission{..},[1])=>V::IntrinsicObject(4),(E::TransportAdmission{refusal,..},[1,0])=>V::Enum(refusal.kind as u32),(E::TransportAdmission{refusal,..},[1,1])=>V::IntrinsicText(refusal.reason),(E::TransportAdmission{refusal,..},[1,2])=>number(refusal.allocated_bytes as u64),(E::TransportAdmission{refusal,..},[1,3])=>number(refusal.physical_bytes as u64),_=>return Err(bad())
  })
 }
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{use store::os_pack::PackRefusal as E;let bad=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original command Pack projection key changed");let keys:&[&str]=match(path,&self.original){([],_)=>&["kind","original"],([1],E::UnsupportedVersion{..})=>&["major","minor"],([1],E::ChecksumMismatch{..})=>&["segment","offset"],([1],E::LimitExceeded{..})=>&["kind","limit"],([1],E::Malformed{..}|E::RetainedMalformed{..})=>&["kind","what","offset","detail"],([1],E::RetainedAllocation{..})=>&["kind","what","offset","detail","allocatedBytes"],([1,rest @ ..],E::ValueRefusal(error))=>return semio_framework_job::key_original_value_error(error,rest,index),([1],E::Io{..})=>&["error","retry"],([1,0,rest @ ..],E::Io{error,..})=>return semio_framework_job::key_original_value_error(error,rest,index),([1],E::TextRefusal(_))=>&["kind","message","span","expected"],([1,2],E::TextRefusal(_))=>&["line","column","length"],([1],E::TransportAdmission{..})=>&["category","refusal"],([1,1],E::TransportAdmission{..})=>&["kind","reason","allocatedBytes","physicalBytes"],_=>return Err(bad())};keys.get(index).copied().ok_or_else(bad)}
}

