//! 🧬️ Baseline owned image interpretation mutations.
use crate::schema::diff::*;
use crate::schema::snapshot::*;
use protocol::{Mutation,MutationDiff};
#[path="🩹️patch-snapshot/🦀️.rs"] pub mod patch_snapshot;
#[path="📸️set-snapshot/🦀️.rs"] pub mod set_snapshot;
#[path="🔢️set-bits-per-sample/🦀️.rs"] pub mod set_bits_per_sample;
#[path="🌈️set-photometric-interpretation/🦀️.rs"] pub mod set_photometric_interpretation;
#[derive(Clone,Debug,PartialEq,value_derive::ToValue,value_derive::FromValue,dsl::Mutations)]
#[mutations(snapshot=TiffSnapshot,diff=TiffDiff,schema="TiffBaselineMutation")]
#[value(tag="mutation",content="payload",rename_all="kebab-case")]
pub enum TiffBaselineMutation{SetSnapshot(set_snapshot::SetSnapshot),PatchSnapshot(patch_snapshot::PatchSnapshot),SetPhotometricInterpretation(set_photometric_interpretation::SetPhotometricInterpretation),SetBitsPerSample(set_bits_per_sample::SetBitsPerSample)}
pub const KINDS:&[&str]=&["set-snapshot","patch-snapshot","set-photometric-interpretation","set-bits-per-sample"];
crate::impl_serde_op_codec!(TiffBaselineMutation,"tiff-baseline-mutation");
pub fn tiff_baseline_conformance_codes(snapshot:&TiffSnapshot)->Vec<String>{crate::standards::v6_0::subsets::baseline::schema::check_tiff_baseline_conformance(snapshot).into_iter().map(|v|v.code.0).collect()}
pub fn apply_tiff_baseline_mutation(snapshot:&mut TiffSnapshot,mutation:&TiffBaselineMutation)->protocol::MutationOutcome<TiffDiff>{let outcome=mutation.diff(snapshot);match outcome.diff().apply(snapshot){Ok(next)=>{*snapshot=next;outcome},Err(error)=>protocol::MutationOutcome::fatal(error.code,error.message,error.target).absorb_messages(outcome.messages().to_vec())}}
pub(crate) fn agg_diff(this:&TiffBaselineMutation,base:&TiffSnapshot)->protocol::MutationOutcome<TiffDiff>{
 let(tag,values)=match this{TiffBaselineMutation::SetSnapshot(v)=>return protocol::MutationOutcome::new(diff_set_snapshot(base,&v.snapshot)),TiffBaselineMutation::PatchSnapshot(v)=>return <patch_snapshot::PatchSnapshot as protocol::MutationKind<TiffSnapshot,TiffBaselineMutation>>::diff(v,base),TiffBaselineMutation::SetBitsPerSample(v)=>(258,TiffValues::Short(v.bits.clone())),TiffBaselineMutation::SetPhotometricInterpretation(v)=>(262,TiffValues::Short(vec![v.photometric]))};
 let Some(page)=base.ifds.first()else{return protocol::MutationOutcome::new(TiffDiff::default())};let mut entries=TiffTagsDiff::default();match page.entries.iter().find(|v|v.tag==tag){Some(old)if old.values==values=>return protocol::MutationOutcome::new(TiffDiff::default()),Some(_)=>entries.modified.push(TiffTagModified{tag,values}),None=>entries.added.push(TiffTagAdded{tag,values})}
 protocol::MutationOutcome::new(TiffDiff{ifds:Some(TiffIfdsDiff{modified:vec![TiffIfdModified{index:0,diff:TiffIfdDiff{entries,blocks:None}}],..Default::default()})})
}
pub(crate) fn agg_inverse(this:&TiffBaselineMutation,base:&TiffSnapshot)->Result<Vec<TiffBaselineMutation>,semio_framework_value::ValueError>{match this{TiffBaselineMutation::PatchSnapshot(v)=><patch_snapshot::PatchSnapshot as protocol::MutationKind<TiffSnapshot,TiffBaselineMutation>>::inverse(v,base),_=>Ok(vec![TiffBaselineMutation::SetSnapshot(set_snapshot::SetSnapshot{snapshot:base.clone()})])}}
