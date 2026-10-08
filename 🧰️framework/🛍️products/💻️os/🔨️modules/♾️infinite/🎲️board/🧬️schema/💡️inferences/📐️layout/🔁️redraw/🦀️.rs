//! 🔁️ Typed redraw and endpoint handle geometry.
use geometry::Point;
use std::collections::HashMap;
use crate::infinite::board::board_visible_or_true;
use crate::infinite::board::ports::directed::schema::snapshot::*;
use crate::infinite::board::schema::layout::*;
use super::{force,hierarchical};
/// 🔗️ Last visible edge wins for shared handles; coincident centers preserve angles.
pub fn snap_edge_handles(snapshot:&mut BoardSnapshot,control:&mut LayoutControl<'_>)->Result<(),LayoutError>{
 if snapshot.schema.as_str()!="board.ports.directed.v1"{return Err(LayoutError::Schema(snapshot.schema.as_str().into()));}
 let mut locations=HashMap::new();let mut shapes=Vec::new();
 for (ni,node) in snapshot.nodes.iter().enumerate(){control.advance(1)?;if !board_visible_or_true(&node.visibility()){shapes.push(None);continue;}
 let shape=match(node.x,node.y,node.shape){(Some(x),Some(y),Some(BoardNodeShape::Rectangle))=>node.width.zip(node.height).map(|(w,h)|(Point::new(x,y),Some((w,h)))),(Some(x),Some(y),_)=>node.radius.map(|_|(Point::new(x,y),None)),_=>None};shapes.push(shape);
 for (hi,handle) in node.handles.as_deref().unwrap_or(&[]).iter().enumerate(){if board_visible_or_true(&handle.visibility()){locations.insert(handle.id.as_str(),(ni,hi));}}
 }
 let mut angles=HashMap::new();for edge in &snapshot.edges {control.advance(1)?;if !board_visible_or_true(&edge.visibility()){continue;}
 let (Some(source),Some(target))=(edge.source.as_deref(),edge.target.as_deref())else{continue;};let(Some(&(a,ha)),Some(&(b,hb)))=(locations.get(source),locations.get(target))else{continue;};
 let (Some((ca,sa)),Some((cb,sb)))=(shapes[a],shapes[b])else{continue;};if geometry::distance_between(ca,cb)<=1e-9{continue;}
 let angle=|center,toward,shape:Option<(f64,f64)>|match shape{Some((w,h))=>graph::drawing::routing::rectangle_handle_angle_toward(center,w,h,toward),None=>graph::drawing::routing::circle_handle_angle_toward(center,toward)};
 angles.insert((a,ha),angle(ca,cb,sa));angles.insert((b,hb),angle(cb,ca,sb));}
 control.advance(angles.len() as u64)?;for ((ni,hi),angle) in angles{snapshot.nodes[ni].handles.as_mut().expect("admitted handle location")[hi].angle=Some(angle);}Ok(())
}
/// 🔀️ Applies declared options atomically to the original admitted snapshot.
pub fn redraw_layout(snapshot:&mut BoardSnapshot,opts:&RedrawLayoutOptions,control:&mut LayoutControl<'_>)->Result<(),LayoutError>{
 let undirected=matches!(snapshot.schema.as_str(),"board.normal.undirected.v1"|"trinity.graph"|"reasoning.wires.identity.snapshot");
 if undirected&&opts.redraw_handles_after{return Err(LayoutError::InvalidOptions("normal undirected redraw does not support redrawHandlesAfter".into()));}
 control.advance(snapshot.nodes.len() as u64)?;let mut next=snapshot.clone();match opts.mode.as_str(){
 "force-graph"=>{let mut force=opts.force_graph.clone().unwrap_or_default();if opts.center_x.is_some(){force.center_x=opts.center_x;}if opts.center_y.is_some(){force.center_y=opts.center_y;}if let Some(seed)=opts.random_seed{force.random_seed=seed;}for id in &opts.locked_node_ids{if !force.locked_node_ids.contains(id){force.locked_node_ids.push(id.clone());}}if undirected{force::apply_force_layout(&mut next,&force,control)?;}else{force::apply_ported_force_layout(&mut next,&force,control)?;}},
 "hierarchical-tree"=>{let mut tree=opts.hierarchical_tree.clone().unwrap_or_default();if opts.center_x.is_some(){tree.center_x=opts.center_x;}if opts.center_y.is_some(){tree.center_y=opts.center_y;}for id in &opts.locked_node_ids{if !tree.locked_node_ids.contains(id){tree.locked_node_ids.push(id.clone());}}hierarchical::apply_hierarchical_layout(&mut next,&tree,control)?;},
 other=>return Err(LayoutError::InvalidOptions(format!("unknown redraw mode: {other}"))),}
 if opts.redraw_handles_after{snap_edge_handles(&mut next,control)?;}*snapshot=next;Ok(())
}
