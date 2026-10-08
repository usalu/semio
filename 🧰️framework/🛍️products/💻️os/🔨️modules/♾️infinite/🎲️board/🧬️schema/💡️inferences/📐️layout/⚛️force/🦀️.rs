//! 🕸️ Typed force graph layout preserves visibility, endpoint resolution and original physics.
use geometry::Vec2;
use graph::drawing::force::{self,ForceLayoutOptions as CoreForceLayoutOptions};
use std::collections::{HashMap,HashSet};
use crate::infinite::board::board_visible_or_true;
use crate::infinite::board::schema::layout::*;
use crate::infinite::board::ports::directed::schema::snapshot::*;
fn node_repulsion_radius(node:&BoardNodeSnapshot)->f64 {
 if node.shape==Some(BoardNodeShape::Rectangle){let w=node.width.unwrap_or(40.0);let h=node.height.unwrap_or(40.0);return ((w*w+h*h).sqrt()*0.5).max(8.0);}
 node.radius.filter(|r|r.is_finite()&&*r>0.0).unwrap_or(32.0)
}
    fn core_opts(opts: &ForceGraphLayoutOptions, gx: f64, gy: f64) -> CoreForceLayoutOptions {
        CoreForceLayoutOptions {
            iterations: opts.iterations,
            ideal_edge_length: opts.ideal_edge_length,
            repulsion_strength: opts.repulsion_strength,
            spring_strength: opts.spring_strength,
            gravity: opts.gravity,
            center_x: gx,
            center_y: gy,
            time_step: opts.time_step,
            velocity_damping: opts.velocity_damping,
            max_speed: opts.max_speed,
            random_seed: opts.random_seed,
            barnes_hut_theta: opts.barnes_hut_theta,
            pairwise_repulsion_max_bodies: opts.pairwise_repulsion_max_bodies,
        }
    }

    /// ↔ Resolves edge endpoint strings to node ids (node-id graphs only).
    pub fn resolve_node_id_endpoint(endpoint_id: &str, id_to_index: &HashMap<String, usize>) -> Option<String> {
        id_to_index.contains_key(endpoint_id).then(|| endpoint_id.to_string())
    }

    /// 🕸️ Runs undirected force layout on a mindmap or puzzle 2d snapshot with node-id edges.
    pub fn apply_force_layout(snapshot: &mut BoardSnapshot, opts: &ForceGraphLayoutOptions, control: &mut LayoutControl<'_>) -> Result<(), LayoutError> {
        apply_force_layout_resolved(snapshot, opts, control, resolve_node_id_endpoint)
    }

    /// 🕸️ Force layout with a custom endpoint→node-id resolver (ported graphs pass handle lookup here).
    pub fn apply_force_layout_resolved(snapshot: &mut BoardSnapshot, opts: &ForceGraphLayoutOptions, control: &mut LayoutControl<'_>, resolve_node_id: impl Fn(&str, &HashMap<String, usize>) -> Option<String>) -> Result<(), LayoutError> {
        if !["board.ports.directed.v1","board.normal.undirected.v1","trinity.graph","reasoning.wires.identity.snapshot"].contains(&snapshot.schema.as_str()) {return Err(LayoutError::Schema(snapshot.schema.as_str().into()));}
        control.advance(snapshot.nodes.len() as u64 + snapshot.edges.len() as u64)?;
        let edges=&snapshot.edges;
        let nodes=&mut snapshot.nodes;
        if nodes.is_empty() {
            return Ok(());
        }
        let locked_ids: HashSet<String> = opts.locked_node_ids.iter().cloned().collect();
        let mut id_to_index: HashMap<String, usize> = HashMap::new();
        let mut visible_node_indices: Vec<usize> = Vec::new();
        let mut optional_xy: Vec<Option<(f64, f64)>> = Vec::new();
        let mut is_locked: Vec<bool> = Vec::new();
        let mut positions: Vec<Vec2> = Vec::new();
        let mut radii: Vec<f64> = Vec::new();
        for (raw_idx, node) in nodes.iter().enumerate() {
            if !board_visible_or_true(&node.visibility()) {continue;}
            let nid=&node.id;
            let x_opt=node.x;
            let y_opt=node.y;
            let xy = match (x_opt, y_opt) {
                (Some(x), Some(y)) if x.is_finite() && y.is_finite() => Some((x, y)),
                _ => None,
            };
            id_to_index.insert(nid.to_string(), positions.len());
            visible_node_indices.push(raw_idx);
            optional_xy.push(xy);
            is_locked.push(locked_ids.contains(nid));
            positions.push(Vec2::ZERO);
            radii.push(node_repulsion_radius(node));
        }
        let n = positions.len();
        if n == 0 {
            return Ok(());
        }
        let mut sum = Vec2::ZERO;
        let mut finite_ct: u32 = 0;
        for (x, y) in optional_xy.iter().flatten() {
            sum += Vec2::new(*x, *y);
            finite_ct += 1;
        }
        let anchor = if finite_ct > 0 { sum / (finite_ct as f64) } else { Vec2::new(opts.center_x.unwrap_or(0.0), opts.center_y.unwrap_or(0.0)) };
        for i in 0..n {
            positions[i] = if let Some((x, y)) = optional_xy[i] { Vec2::new(x, y) } else { Vec2::ZERO };
        }
        let pin: Vec<Option<Vec2>> = (0..n).map(|i| if is_locked[i] { Some(positions[i]) } else { None }).collect();
        force::seed_positions(&mut positions, &pin, anchor, opts.random_seed);
        let mut edge_pairs: Vec<(usize, usize)> = Vec::new();
        let mut seen: HashSet<(usize, usize)> = HashSet::new();
        for e in edges {
            if !board_visible_or_true(&e.visibility()) {continue;}
            let (Some(src),Some(tgt))=(e.source.as_deref(),e.target.as_deref()) else {continue;};
            let Some(a) = resolve_node_id(src, &id_to_index) else {
                continue;
            };
            let Some(b) = resolve_node_id(tgt, &id_to_index) else {
                continue;
            };
            if a == b {
                continue;
            }
            let Some(&ia) = id_to_index.get(&a) else {
                continue;
            };
            let Some(&ib) = id_to_index.get(&b) else {
                continue;
            };
            let lo = ia.min(ib);
            let hi = ia.max(ib);
            if seen.insert((lo, hi)) {
                edge_pairs.push((lo, hi));
            }
        }
        let mut cx = 0.0f64;
        let mut cy = 0.0f64;
        for p in &positions {
            cx += p.x;
            cy += p.y;
        }
        cx /= n as f64;
        cy /= n as f64;
        let gx = opts.center_x.unwrap_or(cx);
        let gy = opts.center_y.unwrap_or(cy);
        let work=(n as u64).saturating_mul(n as u64).saturating_add(edge_pairs.len() as u64).saturating_add(n as u64);
        let mut failure=None;
        let completed=force::run_force_layout_controlled(&mut positions,&radii,&edge_pairs,&pin,&core_opts(opts,gx,gy),&mut |_|match control.advance(work){Ok(())=>true,Err(error)=>{failure=Some(error);false}});
        if !completed {return Err(failure.expect("refused layout work"));}
        control.advance(n as u64)?;
        for (idx, raw_idx) in visible_node_indices.into_iter().enumerate() {
            let Some(node) = nodes.get_mut(raw_idx) else {
                continue;
            };
            node.x=Some(positions[idx].x);
            node.y=Some(positions[idx].y);
        }
        Ok(())
    }


/// 🔌️ Resolves visible handle endpoints before the shared node geometry core.
pub fn apply_ported_force_layout(snapshot:&mut BoardSnapshot,opts:&ForceGraphLayoutOptions,control:&mut LayoutControl<'_>)->Result<(),LayoutError>{
 control.advance(snapshot.nodes.len() as u64)?;let mut handles=HashMap::new();for node in &snapshot.nodes {control.advance(1)?;if !board_visible_or_true(&node.visibility()){continue;}for h in node.handles.as_deref().unwrap_or(&[]){control.advance(1)?;if board_visible_or_true(&h.visibility()){handles.insert(h.id.clone(),node.id.clone());}}}
 apply_force_layout_resolved(snapshot,opts,control,|endpoint,ids|{let node=handles.get(endpoint).map(String::as_str).unwrap_or(endpoint);ids.contains_key(node).then(||node.to_owned())})
}
