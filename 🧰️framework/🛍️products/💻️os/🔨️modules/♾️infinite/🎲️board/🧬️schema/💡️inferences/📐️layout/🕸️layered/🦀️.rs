//! 🕸️ Original layered DAG geometry over declared node and endpoint identities.
use std::collections::{HashMap,HashSet};
use crate::infinite::board::schema::layout::*;
/// 🌲️ Buchheim tidy-tree on string-labeled directed edges.
fn buchheim_positions(roots: &[String], directed: &[(String, String)], depth: &HashMap<String, i32>,control:&mut LayoutControl<'_>) -> Result<HashMap<String, (f64, f64)>,LayoutError> {
    control.advance(roots.len() as u64+directed.len() as u64)?;
    let roots_set: HashSet<String> = roots.iter().cloned().collect();
    let mut incoming: HashMap<String, Vec<String>> = HashMap::new();
    for (u, v) in directed {
        incoming.entry(v.clone()).or_default().push(u.clone());
    }
    for v in incoming.values_mut() {
        v.sort();
        v.dedup();
    }
    let mut chosen_parent: HashMap<String, String> = HashMap::new();
    let mut all_ids: HashSet<String> = HashSet::new();
    for (u, v) in directed {
        all_ids.insert(u.clone());
        all_ids.insert(v.clone());
    }
    for r in roots {
        all_ids.insert(r.clone());
    }
    for id in &all_ids {
        if roots_set.contains(id) {
            continue;
        }
        let ps = incoming.get(id).cloned().unwrap_or_default();
        if ps.is_empty() {
            continue;
        }
        let best = ps
            .iter()
            .min_by_key(|p| {
                let dp = depth.get(*p).copied().unwrap_or(0);
                (dp, (*p).clone())
            })
            .expect("non-empty ps")
            .clone();
        chosen_parent.insert(id.clone(), best);
    }
    let mut ordered_ids: Vec<String> = all_ids.into_iter().collect();
    ordered_ids.sort();
    if ordered_ids.is_empty() {
        return Ok(HashMap::new());
    }
    let id_to_idx: HashMap<String, usize> = ordered_ids.iter().enumerate().map(|(i, s)| (s.clone(), i)).collect();
    let super_idx = ordered_ids.len();
    let mut nodes: Vec<BuchheimNode> =
        ordered_ids.iter().map(|id| BuchheimNode { ancestor: 0, change: 0.0, children: vec![], id: id.clone(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: false, thread: None, x: -1.0, y: 0.0 }).collect();
    nodes.push(BuchheimNode { ancestor: super_idx, change: 0.0, children: vec![], id: "__tree_super__".into(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: true, thread: None, x: -1.0, y: 0.0 });
    for (i, oid) in ordered_ids.iter().enumerate() {
        let pidx = if roots_set.contains(oid) {
            super_idx
        } else {
            match chosen_parent.get(oid) {
                Some(p) => *id_to_idx.get(p).unwrap_or(&super_idx),
                None => super_idx,
            }
        };
        nodes[i].parent = Some(pidx);
    }
    for node in nodes.iter_mut() {
        node.children.clear();
    }
    for i in 0..super_idx {
        let pi = nodes[i].parent.expect("parent set for every non-super node in the loop above");
        nodes[pi].children.push(i);
    }
    for p in 0..=super_idx {
        let mut ch = nodes[p].children.clone();
        ch.sort_by_key(|&c| nodes[c].id.clone());
        nodes[p].children = ch;
    }
    for p in 0..=super_idx {
        if nodes[p].children.is_empty() {
            continue;
        }
        let ch = nodes[p].children.clone();
        for (k, &c) in ch.iter().enumerate() {
            nodes[c].number = (k + 1) as i32;
            nodes[c].ancestor = c;
        }
    }
    buchheim_first_walk(&mut nodes, super_idx, 1.0,control)?;
    let min_x = buchheim_second_walk(&mut nodes, super_idx, 0.0, 0, f64::INFINITY,control)?;
    if min_x.is_finite() && min_x < 0.0 {
        buchheim_third_walk(&mut nodes, super_idx, -min_x,control)?;
    }
    let mut out = HashMap::new();
    for (i, n) in nodes.iter().enumerate() {
        if i == super_idx || n.synthetic {
            continue;
        }
        out.insert(n.id.clone(), (n.x, n.y));
    }
    Ok(out)
}

#[derive(Debug)]
struct BuchheimNode {
    id: String,
    parent: Option<usize>,
    children: Vec<usize>,
    x: f64,
    y: f64,
    mod_: f64,
    thread: Option<usize>,
    ancestor: usize,
    change: f64,
    shift: f64,
    number: i32,
    synthetic: bool,
}

fn buchheim_left_brother(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
    let p = nodes[i].parent?;
    let ch = &nodes[p].children;
    let pos = ch.iter().position(|&c| c == i)?;
    if pos == 0 {
        return None;
    }
    Some(ch[pos - 1])
}

fn buchheim_leftmost_sibling(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
    let p = nodes[i].parent?;
    let ch = &nodes[p].children;
    if ch.first() == Some(&i) {
        return None;
    }
    ch.first().copied()
}

fn buchheim_next_right(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
    if let Some(t) = nodes[i].thread {
        return Some(t);
    }
    nodes[i].children.last().copied()
}

fn buchheim_next_left(nodes: &[BuchheimNode], i: usize) -> Option<usize> {
    if let Some(t) = nodes[i].thread {
        return Some(t);
    }
    nodes[i].children.first().copied()
}

fn buchheim_move_subtree(nodes: &mut [BuchheimNode], wl: usize, wr: usize, shift: f64) {
    let subtrees = (nodes[wr].number - nodes[wl].number) as f64;
    if subtrees <= 0.0 {
        return;
    }
    nodes[wr].change -= shift / subtrees;
    nodes[wr].shift += shift;
    nodes[wl].change += shift / subtrees;
    nodes[wr].x += shift;
    nodes[wr].mod_ += shift;
}

fn buchheim_execute_shifts(nodes: &mut [BuchheimNode], v: usize) {
    let mut shift = 0.0f64;
    let mut change = 0.0f64;
    for &w in nodes[v].children.iter().rev() {
        nodes[w].x += shift;
        nodes[w].mod_ += shift;
        change += nodes[w].change;
        shift += nodes[w].shift + change;
    }
}

fn buchheim_apportion(nodes: &mut [BuchheimNode], v: usize, default_ancestor: usize, distance: f64) -> usize {
    let w = match buchheim_left_brother(nodes, v) {
        Some(w) => w,
        None => return default_ancestor,
    };
    let mut vir = v;
    let mut vil = w;
    let mut vol = buchheim_leftmost_sibling(nodes, v).unwrap_or(v);
    let mut vor = v;
    let mut sir = nodes[v].mod_;
    let mut sil = nodes[vil].mod_;
    loop {
        let vil_r = buchheim_next_right(nodes, vil);
        let vir_l = buchheim_next_left(nodes, vir);
        if vil_r.is_none() || vir_l.is_none() {
            break;
        }
        vil = vil_r.expect("checked Some above");
        vir = vir_l.expect("checked Some above");
        let vol_l = buchheim_next_left(nodes, vol);
        let vor_r = buchheim_next_right(nodes, vor);
        if vol_l.is_none() || vor_r.is_none() {
            break;
        }
        vol = vol_l.expect("checked Some above");
        vor = vor_r.expect("checked Some above");
        nodes[vor].ancestor = v;
        let shift = (nodes[vil].x + sil) - (nodes[vir].x + sir) + distance;
        if shift > 0.0 {
            buchheim_move_subtree(nodes, default_ancestor, v, shift);
            sir += shift;
        }
        sil += nodes[vil].mod_;
        sir += nodes[vir].mod_;
    }
    default_ancestor
}

fn buchheim_first_walk(nodes: &mut [BuchheimNode], v: usize, distance: f64,control:&mut LayoutControl<'_>) -> Result<usize,LayoutError> {
    control.advance(1)?;
    if nodes[v].children.is_empty() {
        if let Some(lb) = buchheim_left_brother(nodes, v) {
            nodes[v].x = nodes[lb].x + distance;
        } else {
            nodes[v].x = 0.0;
        }
        return Ok(v);
    }
    let mut default_ancestor = nodes[v].children[0];
    for &w in nodes[v].children.clone().iter() {
        buchheim_first_walk(nodes, w, distance,control)?;
        default_ancestor = buchheim_apportion(nodes, w, default_ancestor, distance);
    }
    buchheim_execute_shifts(nodes, v);
    let c0 = nodes[v].children[0];
    let c1 = *nodes[v].children.last().expect("children non-empty per the is_empty check above");
    let mid = (nodes[c0].x + nodes[c1].x) * 0.5;
    if let Some(w) = buchheim_left_brother(nodes, v) {
        nodes[v].x = nodes[w].x + distance;
        nodes[v].mod_ = nodes[v].x - mid;
    } else {
        nodes[v].x = mid;
    }
    Ok(v)
}

fn buchheim_second_walk(nodes: &mut [BuchheimNode], v: usize, m: f64, depth: i32, min_x: f64,control:&mut LayoutControl<'_>) -> Result<f64,LayoutError> {
    control.advance(1)?;
    nodes[v].x += m;
    nodes[v].y = depth as f64;
    let mut min_x = min_x.min(nodes[v].x);
    for &w in nodes[v].children.clone().iter() {
        min_x = buchheim_second_walk(nodes, w, m + nodes[v].mod_, depth + 1, min_x,control)?;
    }
    Ok(min_x)
}

fn buchheim_third_walk(nodes: &mut [BuchheimNode], v: usize, n: f64,control:&mut LayoutControl<'_>)->Result<(),LayoutError> {
    control.advance(1)?;
    nodes[v].x += n;
    for &c in nodes[v].children.clone().iter() {
        buchheim_third_walk(nodes, c, n,control)?;
    }
    Ok(())
}

#[cfg(test)]
#[path = "🧪️tests/🔬️tidy-tree/🦀️.rs"]
mod tidy_tree_tests;

/// 🌲️ Returns positions before a typed snapshot owner commits any changed coordinates.
pub fn layered_positions(node_ids:&HashSet<String>,directed:&[(String,String)],opts:&DagLayoutOptions,control:&mut LayoutControl<'_>)->Result<HashMap<String,(f64,f64)>,LayoutError>{
 control.advance(node_ids.len() as u64 +directed.len() as u64)?;if node_ids.is_empty(){return Ok(HashMap::new());}
    let mut incoming: HashMap<String, u32> = HashMap::new();
    for id in node_ids {
        incoming.insert(id.clone(), 0);
    }
    for (_, v) in directed {
        *incoming.entry(v.clone()).or_insert(0) += 1;
    }
    let roots: Vec<String> = node_ids.iter().filter(|id| incoming.get(*id).copied().unwrap_or(0) == 0).cloned().collect();
    let roots = if roots.is_empty() { node_ids.iter().cloned().collect() } else { roots };
    let mut depth: HashMap<String, i32> = HashMap::new();
    for r in &roots {
        depth.insert(r.clone(), 0);
    }
    for _ in 0..directed.len().saturating_add(node_ids.len()).saturating_add(4) {
        control.advance(directed.len() as u64)?;
        let mut changed = false;
        for (u, v) in directed {
            let Some(&du) = depth.get(u) else {
                continue;
            };
            let nd = du + 1;
            if depth.get(v).copied().unwrap_or(-1) < nd {
                depth.insert(v.clone(), nd);
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    let pos = buchheim_positions(&roots, directed, &depth,control)?;
    let mut minx = f64::INFINITY;
    let mut maxx = f64::NEG_INFINITY;
    let mut miny = f64::INFINITY;
    let mut maxy = f64::NEG_INFINITY;
    for (x, y) in pos.values() {
        minx = minx.min(*x);
        maxx = maxx.max(*x);
        miny = miny.min(*y);
        maxy = maxy.max(*y);
    }
    let cx = (minx + maxx) * 0.5;
    let cy = (miny + maxy) * 0.5;
    let gx = opts.center_x.unwrap_or(0.0);
    let gy = opts.center_y.unwrap_or(0.0);
    let (dx, dy) = match opts.orientation {
        DagLayoutOrientation::LeftRight => (gx - cy * opts.layer_spacing, gy - cx * opts.sibling_gap),
        DagLayoutOrientation::TopBottom => (gx - cx * opts.sibling_gap, gy - cy * opts.layer_spacing),
    };
    let mut result=HashMap::new();
    for (id,(bx,by)) in pos {control.advance(1)?;let (x,y)=match opts.orientation {DagLayoutOrientation::LeftRight=>(by*opts.layer_spacing+dx,bx*opts.sibling_gap+dy),DagLayoutOrientation::TopBottom=>(bx*opts.sibling_gap+dx,by*opts.layer_spacing+dy)};result.insert(id,(x,y));}Ok(result)
}
