use crate::apply_mutation;
use crate::schema::diff::TiffDiff;
use crate::schema::mutations::*;
use crate::schema::snapshot::{TiffIfd,TiffSampleBlock,TiffWord64,TiffTag,TiffValues};
use crate::TiffSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{Mutation,MutationDiff};

fn tag(tag:u16,value:u16)->TiffTag{TiffTag{tag,values:TiffValues::Short(vec![value])}}
fn snapshot()->TiffSnapshot{TiffSnapshot{ifds:vec![TiffIfd{entries:vec![tag(256,2),tag(257,1),tag(258,8),tag(262,2),tag(277,3),tag(500,1)],blocks:vec![TiffSampleBlock{x:0,y:0,width:2,height:1,channels:3,samples:(1..=6).map(TiffWord64::from_word).collect()}]}],..Default::default()}}
fn variants(base:&TiffSnapshot)->Vec<TiffMutation>{
 vec![TiffMutation::InsertIfd(InsertIfdMutation{index:1,ifd:TiffIfd{entries:vec![tag(500,3)],blocks:vec![]}}),TiffMutation::RemoveIfd(RemoveIfdMutation{index:0}),TiffMutation::ReplaceTag(ReplaceTagMutation{ifd_index:0,tag:500,values:TiffValues::Long(vec![4])}),TiffMutation::RemoveTag(RemoveTagMutation{ifd_index:0,tag:500}),TiffMutation::PaintRegion(PaintRegionMutation{revision:crate::schema::mutations::paint_region::samples::tiff_revision(base),ifd_index:0,x:0,y:0,width:1,height:1,red:8,green:9,blue:10,alpha:255}),TiffMutation::ReplaceSamples(ReplaceSamplesMutation{ifd_index:0,block:0,offset:2,samples:vec![TiffWord64::from_word(77),TiffWord64::from_word(78)]})]
}
#[test]fn every_mounted_mutation_diff_matches_imperative_apply_and_inverse(){let base=snapshot();for mutation in variants(&base){let expected=mutation.diff(&base);let mut imperative=base.clone();assert_eq!(apply_mutation(&mut imperative,&mutation),expected);let applied=protocol::apply_diff(expected.diff(), &base).expect("diff applies");assert_eq!(applied,imperative);assert_eq!(protocol::apply_diff(&expected.diff().inverse(&base), &applied).expect("inverse applies"),base);}}
#[test]fn absorbed_tag_changes_equal_sequential_application(){let base=snapshot();let first=TiffMutation::ReplaceTag(ReplaceTagMutation{ifd_index:0,tag:500,values:TiffValues::Long(vec![4])}).diff(&base);let middle=protocol::apply_diff(first.diff(), &base).unwrap();let second=TiffMutation::ReplaceTag(ReplaceTagMutation{ifd_index:0,tag:500,values:TiffValues::Long(vec![8])}).diff(&middle);let expected=protocol::apply_diff(second.diff(), &middle).unwrap();let mut absorbed=first.diff().clone();absorbed.absorb(second.diff().clone());assert_eq!(protocol::apply_diff(&absorbed, &base).unwrap(),expected);}

#[semio_framework_async_macros::async_test]
async fn tiff_mutation_inverse_sum_law_holds_for_every_leaf() {
    let single = snapshot();
    let mut base = single.clone();
    base.ifds.push(TiffIfd { entries: vec![tag(500, 5), tag(502, 6)], blocks: vec![] });
    base.ifds.push(single.ifds[0].clone());
    let revision = crate::schema::mutations::paint_region::samples::tiff_revision(&base);
    let words = |values: &[u64]| values.iter().copied().map(TiffWord64::from_word).collect::<Vec<_>>();
    for mutation in [
        TiffMutation::InsertIfd(InsertIfdMutation { index: 1, ifd: TiffIfd { entries: vec![tag(500, 3)], blocks: vec![] } }),
        TiffMutation::InsertIfd(InsertIfdMutation { index: 3, ifd: TiffIfd { entries: vec![tag(501, 3)], blocks: vec![] } }),
        TiffMutation::RemoveIfd(RemoveIfdMutation { index: 1 }),
        TiffMutation::RemoveIfd(RemoveIfdMutation { index: 0 }),
        TiffMutation::ReplaceTag(ReplaceTagMutation { ifd_index: 1, tag: 500, values: TiffValues::Long(vec![4]) }),
        TiffMutation::ReplaceTag(ReplaceTagMutation { ifd_index: 1, tag: 501, values: TiffValues::Long(vec![4]) }),
        TiffMutation::RemoveTag(RemoveTagMutation { ifd_index: 1, tag: 500 }),
        TiffMutation::PaintRegion(PaintRegionMutation { revision: revision.clone(), ifd_index: 2, x: 0, y: 0, width: 1, height: 1, red: 8, green: 9, blue: 10, alpha: 255 }),
        TiffMutation::ReplaceSamples(ReplaceSamplesMutation { ifd_index: 2, block: 0, offset: 1, samples: words(&[41, 42, 43]) }),
        TiffMutation::ReplaceSamples(ReplaceSamplesMutation { ifd_index: 0, block: 0, offset: 0, samples: words(&[7]) }),
    ] {
        protocol::protocol_laws::assert_mutation_inverse_sum_law(&mutation, &base).await;
    }
}
#[test]fn explicit_leaf_sequences_edit_the_snapshot_and_their_reversed_inverses_restore_it(){
 let base=snapshot();
 let leaves=vec![TiffMutation::ReplaceSamples(ReplaceSamplesMutation{ifd_index:0,block:0,offset:1,samples:vec![TiffWord64{lo:99,hi:0}]}),TiffMutation::RemoveTag(RemoveTagMutation{ifd_index:0,tag:500}),TiffMutation::ReplaceTag(ReplaceTagMutation{ifd_index:0,tag:501,values:TiffValues::Short(vec![7])}),TiffMutation::InsertIfd(InsertIfdMutation{index:1,ifd:TiffIfd{entries:vec![tag(500,3)],blocks:vec![]}})];
 let mut state=base.clone();let mut inverses=Vec::new();
 for leaf in &leaves{inverses.push(leaf.inverse(&state).unwrap());assert!(apply_mutation(&mut state,leaf).is_applicable(Default::default()));}
 assert_ne!(state,base);assert_eq!(state.ifds[0].blocks[0].samples[1].lo,99);
 for inverse in inverses.iter().rev(){for row in inverse.iter().rev(){assert!(apply_mutation(&mut state,row).is_applicable(Default::default()));}}
 assert_eq!(state,base);
}
