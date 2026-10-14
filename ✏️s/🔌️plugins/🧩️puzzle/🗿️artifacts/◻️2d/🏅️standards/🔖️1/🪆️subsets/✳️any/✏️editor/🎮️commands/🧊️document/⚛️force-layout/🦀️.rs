//! 🕸️ Force command admits one typed read and records coordinate mutations directly.
use crate::editor::puzzle2d::Puzzle2dActionCtx;
use semio_framework_value::FromValue;
use semio_framework_os_infinite::board::{ports::directed::BoardSnapshot,schema::layout::{ForceGraphLayoutOptions,LayoutControl}};
/// 🌀️ Layout emits one move-node fact per admitted coordinate without serializing the document.
pub fn force_layout(ctx:&mut Puzzle2dActionCtx<'_>,control:&mut LayoutControl<'_>){
 let Ok(mut snapshot)=BoardSnapshot::from_value(semio_framework_pack_json::to_dsl_value(&ctx.scene.board_snapshot))else{return;};
 if semio_framework_os_infinite::board::schema::layout_inferences::force::apply_ported_force_layout(&mut snapshot,&ForceGraphLayoutOptions::default(),control).is_ok(){ctx.recorder.place_nodes(&snapshot.nodes);}
}
