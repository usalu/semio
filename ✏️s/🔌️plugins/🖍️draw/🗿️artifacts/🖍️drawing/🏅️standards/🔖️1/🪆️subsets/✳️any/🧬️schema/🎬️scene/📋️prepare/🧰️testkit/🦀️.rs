//! 🧰️ Physical-close macros shared with the drawing writer laws (behind the `testkit` feature).

#[path = "../../../../../../../../../../../../../🧰️framework/🔨️modules/◻️2d/🛤️path/🎟️work/🧪️tests/🦀️.rs"]
mod funding;

pub use funding::Wallet;

use semio_framework_value::{OriginalRetirementPoll, RetainedAllocationEffect, RetainedCloneGrant, RetainedCloneStep, RetainedWorkPoll};

/// 🧮️ Supplies the exact physical grant chosen by a fixture.
pub fn effect_turn<R>(grant: RetainedCloneGrant, run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> R) -> R {
    funding::with_effect(grant, funding::WORK, &mut None, run)
}

/// 🪪️ Identifies the same leased document before and after retirement handoff.
pub fn vector_source_address(owner: &super::DocumentVectorJob<'_>) -> Option<*const super::DrawingSnapshot> {
    owner.source.as_ref().map(|source| source.document() as *const _)
}

/// 📮️ Identifies the leased document while the vector owner retires private work.
pub fn vector_retirement_source_address(owner: &super::DocumentVectorRetirement<'_>) -> Option<*const super::DrawingSnapshot> {
    owner.source.as_ref().map(|source| source.document() as *const _)
}

/// 🪜️ Reports whether the next vector turn retires a completed child.
pub fn vector_child_retirement_pending(owner: &super::DocumentVectorJob<'_>) -> bool {
    owner.closing.is_some()
}

/// 🎒️ Reports private vector work held at a refusing checkpoint.
pub fn vector_has_private_work(owner: &super::DocumentVectorJob<'_>) -> bool {
    owner.preparation.is_some() || owner.traces.is_some() || owner.texts.is_some() || owner.algorithms.is_some()
}

/// 📍️ Identifies the backing of the prepared nodes before publication.
pub fn prepared_node_address(owner: &super::DocumentSceneJob<'_>) -> *const super::DocumentSceneNode {
    owner.cursor.plan.nodes.as_ptr()
}

/// 🧱️ Reports whether a failing preparation already owns unpublished nodes.
pub fn has_unpublished_nodes(owner: &super::DocumentSceneJob<'_>) -> bool {
    !owner.cursor.plan.nodes.is_empty() || owner.cursor.node.is_some()
}

/// 🪙️ Drives one original close turn under the exact physical grant chosen by a test.
pub fn physical_turn(
    grant: RetainedCloneGrant,
    run: impl FnOnce(&mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll,
) -> RetainedCloneStep {
    let poll = effect_turn(grant, run);
    match poll {
        RetainedWorkPoll::Pending { .. } => RetainedCloneStep::Progress(Default::default()),
        RetainedWorkPoll::Complete(turn) => turn.result.expect("physical close must pay its original receipt"),
    }
}

#[macro_export]
macro_rules! draw_scene_physical_turn {
    ($owner:expr, $items:expr) => {{
        let owner = &mut $owner;
        let copy = owner.next_copy_byte_demand().unwrap();
        let release = owner.next_release_byte_demand().unwrap();
        let grant = semio_framework_value::RetainedCloneGrant {
            maximum_items: $items,
            maximum_copy_bytes: copy,
            maximum_capacity_bytes: owner.next_capacity_byte_demand(if copy > 0 { copy } else { release }).unwrap(),
            maximum_release_bytes: release,
            maximum_depth: owner.next_depth_demand().unwrap(),
        };
        let denied = semio_framework_value::RetainedCloneGrant { maximum_items: 0, ..grant };
        let empty = $crate::preparation::testkit::physical_turn(denied, |effect| owner.close_step(copy, effect));
        assert_eq!(empty.progress(), Default::default());
        let step = $crate::preparation::testkit::physical_turn(grant, |effect| owner.close_step(copy, effect));
        assert!(step.progress().fits(grant));
        step
    }};
}

#[macro_export]
macro_rules! draw_scene_physical_close {
    ($owner:expr, $items:expr) => {{
        let mut turns = 0;
        while !$owner.terminal_is_empty() {
            $crate::draw_scene_physical_turn!($owner, $items);
            turns += 1;
            assert!(turns < 2_000_000);
        }
        turns
    }};
}

/// 🚦️ Finite fixture authority is never renewed when a real producer cannot perform another turn.
#[macro_export]
macro_rules! draw_scene_funded_turn {
    ($job:expr, $items:expr, $wallet:expr) => {{
        let job = &mut $job;
        $wallet.turn_effect($items, |work, effect| {
            let before = work.receipt();
            let progress = job.advance($items, work, effect).unwrap();
            assert!(progress.done || work.receipt() != before || job.last_turn_work() > 0, "finite scene fixture exhausted at {:?}", job.next_work_demand());
            progress
        })
    }};
}

/// 🧾️ The exact same finite fixture stock pays the original close.
#[macro_export]
macro_rules! draw_scene_funded_close {
    ($owner:expr, $wallet:expr, $items:expr) => {{
        let owner = &mut $owner;
        let wallet = &mut $wallet;
        let mut turns = 0;
        let mut work = 0;
        while !owner.terminal_is_empty() {
            for _ in 0..$items {
                if owner.terminal_is_empty() {
                    break;
                }
                let body = wallet.remaining().maximum_copy_bytes;
                let demand = owner.next_demand(body).unwrap();
                let progress = wallet.close_turn(demand, |effect| owner.close_step(body, effect)).unwrap().progress();
                assert_ne!(progress, Default::default());
                work += progress.copied_items as u64;
                turns += 1;
                assert!(turns < 2_000_000);
            }
        }
        work
    }};
}

/// 🧪️ Drives the real document vector job to its completed plan under a finite stock.
pub fn scene_plan(document: &semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot) -> crate::preparation::DocumentScenePlan {
    let mut wallet = scene_wallet();
    let mut job = crate::preparation::DocumentVectorJob::new(document, Default::default(), Default::default()).unwrap();
    while !draw_scene_funded_turn!(job, 4096, wallet).done {}
    let (mut closing, plan) = job.into_retirement();
    draw_scene_funded_close!(closing, wallet, 1);
    plan.unwrap()
}

/// 🧧️ The finite authority for one fixture scene and retirement.
pub fn scene_wallet() -> Wallet {
    const STOCK: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1 << 22, maximum_copy_bytes: 1 << 28, maximum_capacity_bytes: 1 << 25, maximum_release_bytes: 1 << 26, maximum_depth: 256 };
    Wallet::from_grant(STOCK, 2_000_000)
}

/// 🧪️ Projects a completed plan onto the flat node records tests assert on: visible paths, glyph text and images.
pub fn scene_nodes_of_plan(plan:&crate::preparation::DocumentScenePlan)->Vec<semio_s_artifact_draw_drawing_model::DrawingSceneNode>{
 use crate::preparation::DocumentSceneContent as C;
 use semio_s_artifact_draw_drawing_model::{DrawingSceneNode,DrawingSceneText,DrawingSceneImage};
 let rule=|rule:&semio_s_artifact_draw_drawing_model::fill_rule::FillRule|match rule{semio_s_artifact_draw_drawing_model::fill_rule::FillRule::Evenodd=>"evenodd",semio_s_artifact_draw_drawing_model::fill_rule::FillRule::Nonzero=>"nonzero"};
 plan.nodes.iter().filter(|node|node.visible).filter_map(|node|{
  let mut out=DrawingSceneNode{id:node.id.clone(),groups:node.groups.clone(),transform:node.transform,segments:Default::default(),fill:None,stroke:None,opacity:node.opacity,blend_mode:node.blend_mode.clone().into(),visible:node.visible,fill_rule:None,text:None,image:None};
  match &node.content{
   C::Path{segments,fill_rule,fill,stroke}=>{out.segments=segments.clone().into();out.fill=fill.clone();out.stroke=stroke.clone();out.fill_rule=Some(rule(fill_rule).into());},
   C::Glyphs{content,x,y,size,font_family,fill,stroke,..}=>{out.transform=semio_s_artifact_draw_drawing_geometry::multiply(node.transform,[1.0,0.0,0.0,1.0,*x,*y]);out.fill=fill.clone();out.stroke=stroke.clone();out.text=Some(DrawingSceneText{content:content.clone().into(),size:*size,font_family:*font_family});},
   C::Image{asset,width,height}=>{out.image=Some(DrawingSceneImage{asset_id:asset.clone().into(),width:*width,height:*height});},
   _=>return None,
  }
  Some(out)
 }).collect()
}

/// 🧪️ Completed visible scene nodes of one document, produced by the real vector job.
pub fn scene_nodes(document:&semio_s_artifact_draw_drawing_model::snapshot::DrawingSnapshot)->Vec<semio_s_artifact_draw_drawing_model::DrawingSceneNode>{scene_nodes_of_plan(&scene_plan(document))}
