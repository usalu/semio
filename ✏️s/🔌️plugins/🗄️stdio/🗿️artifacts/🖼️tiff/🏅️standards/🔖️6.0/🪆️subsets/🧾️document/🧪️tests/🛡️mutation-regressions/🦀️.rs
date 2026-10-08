use crate::schema::diff::TiffDiff;
use crate::schema::mutations::*;
use crate::schema::mutations::{set_snapshot::SetSnapshot,patch_snapshot::PatchSnapshot};
use crate::schema::snapshot::{TiffIfd,TiffSampleBlock,TiffWord64,TiffTag,TiffValues};
use crate::TiffSnapshot;
use protocol::command::DiffAlgebra;
use protocol::{Mutation,MutationDiff};

fn tag(tag:u16,value:u16)->TiffTag{TiffTag{tag,values:TiffValues::Short(vec![value])}}
fn snapshot()->TiffSnapshot{TiffSnapshot{ifds:vec![TiffIfd{entries:vec![tag(256,2),tag(257,1),tag(258,8),tag(262,2),tag(277,3),tag(500,1)],blocks:vec![TiffSampleBlock{x:0,y:0,width:2,height:1,channels:3,samples:(1..=6).map(TiffWord64::from_word).collect()}]}],..Default::default()}}
fn variants(base:&TiffSnapshot)->Vec<TiffMutation>{
 let event=semio_s_artifact_stdio_contract::editing::SnapshotEditEvent::SetValue{path:"/ifds/0/blocks/0/samples/0/lo".into(),value:semio_framework_value::DslValue::Number(semio_framework_value::Number::UInt(9))};
 let mut replacement=base.clone();replacement.ifds[0].blocks[0].samples[1].lo=99;
 vec![TiffMutation::SetSnapshot(SetSnapshot{snapshot:replacement}),TiffMutation::PatchSnapshot(PatchSnapshot{patch:semio_s_artifact_stdio_contract::editing::prepare_snapshot_patch(base,&event).unwrap()}),TiffMutation::InsertIfd(InsertIfdMutation{index:1,ifd:TiffIfd{entries:vec![tag(500,3)],blocks:vec![]}}),TiffMutation::RemoveIfd(RemoveIfdMutation{index:0}),TiffMutation::ReplaceTag(ReplaceTagMutation{ifd_index:0,tag:500,values:TiffValues::Long(vec![4])}),TiffMutation::RemoveTag(RemoveTagMutation{ifd_index:0,tag:500}),TiffMutation::PaintRegion(PaintRegionMutation{revision:crate::schema::mutations::paint_region::samples::tiff_revision(base),ifd_index:0,x:0,y:0,width:1,height:1,red:8,green:9,blue:10,alpha:255})]
}
#[test]fn every_mounted_mutation_diff_matches_imperative_apply_and_inverse(){let base=snapshot();for mutation in variants(&base){let expected=mutation.diff(&base);let mut imperative=base.clone();assert_eq!(apply_tiff_mutation(&mut imperative,&mutation),expected);let applied=expected.diff().apply(&base).expect("diff applies");assert_eq!(applied,imperative);assert_eq!(expected.diff().inverse(&base).apply(&applied).expect("inverse applies"),base);}}
#[test]fn between_tracks_exact_owned_sample_words(){let a=snapshot();let mut b=a.clone();b.ifds[0].blocks[0].samples[2].lo=99;assert_eq!(TiffDiff::between(&a,&b).apply(&a).unwrap(),b);assert_eq!(TiffDiff::between(&b,&a).apply(&b).unwrap(),a);}
#[test]fn absorbed_tag_changes_equal_sequential_application(){let base=snapshot();let first=TiffMutation::ReplaceTag(ReplaceTagMutation{ifd_index:0,tag:500,values:TiffValues::Long(vec![4])}).diff(&base);let middle=first.diff().apply(&base).unwrap();let second=TiffMutation::ReplaceTag(ReplaceTagMutation{ifd_index:0,tag:500,values:TiffValues::Long(vec![8])}).diff(&middle);let expected=second.diff().apply(&middle).unwrap();let mut absorbed=first.diff().clone();absorbed.absorb(second.diff().clone());assert_eq!(absorbed.apply(&base).unwrap(),expected);}
