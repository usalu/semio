//! 🪜️ Layered Buchheim layout of the board schema: the layered Buchheim node, its apportion and walk steps and the layered positions. See [`super`].

use super::*;

#[derive(Debug)]
pub(super) struct LayeredBuchheimNode {
    pub id: String,
    pub(super) parent: Option<usize>,
    pub(super) children: Vec<usize>,
    pub x: f64,
    pub y: f64,
    pub(super) mod_: f64,
    pub thread: Option<usize>,
    pub(super) ancestor: usize,
    pub change: f64,
    pub shift: f64,
    pub number: i32,
    pub(super) synthetic: bool,
}

fn layered_buchheim_left_brother(nodes: &[LayeredBuchheimNode], i: usize) -> Option<usize> {
    let p = nodes[i].parent?;
    let ch = &nodes[p].children;
    let pos = ch.iter().position(|&c| c == i)?;
    if pos == 0 {
        return None;
    }
    Some(ch[pos - 1])
}

fn layered_buchheim_leftmost_sibling(nodes: &[LayeredBuchheimNode], i: usize) -> Option<usize> {
    let p = nodes[i].parent?;
    let ch = &nodes[p].children;
    if ch.first() == Some(&i) {
        return None;
    }
    ch.first().copied()
}

fn layered_buchheim_next_right(nodes: &[LayeredBuchheimNode], i: usize) -> Option<usize> {
    if let Some(t) = nodes[i].thread {
        return Some(t);
    }
    nodes[i].children.last().copied()
}

fn layered_buchheim_next_left(nodes: &[LayeredBuchheimNode], i: usize) -> Option<usize> {
    if let Some(t) = nodes[i].thread {
        return Some(t);
    }
    nodes[i].children.first().copied()
}

fn layered_buchheim_move_subtree(nodes: &mut [LayeredBuchheimNode], wl: usize, wr: usize, shift: f64) {
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

fn layered_buchheim_execute_shifts(nodes: &mut [LayeredBuchheimNode], v: usize) {
    let mut shift = 0.0f64;
    let mut change = 0.0f64;
    for &w in nodes[v].children.iter().rev() {
        nodes[w].x += shift;
        nodes[w].mod_ += shift;
        change += nodes[w].change;
        shift += nodes[w].shift + change;
    }
}

fn layered_buchheim_apportion(nodes: &mut [LayeredBuchheimNode], v: usize, default_ancestor: usize, distance: f64) -> usize {
    let w = match layered_buchheim_left_brother(nodes, v) {
        Some(w) => w,
        None => return default_ancestor,
    };
    let mut vir = v;
    let mut vil = w;
    let mut vol = layered_buchheim_leftmost_sibling(nodes, v).unwrap_or(v);
    let mut vor = v;
    let mut sir = nodes[v].mod_;
    let mut sil = nodes[vil].mod_;
    loop {
        let vil_r = layered_buchheim_next_right(nodes, vil);
        let vir_l = layered_buchheim_next_left(nodes, vir);
        if vil_r.is_none() || vir_l.is_none() {
            break;
        }
        vil = vil_r.expect("checked Some above");
        vir = vir_l.expect("checked Some above");
        let vol_l = layered_buchheim_next_left(nodes, vol);
        let vor_r = layered_buchheim_next_right(nodes, vor);
        if vol_l.is_none() || vor_r.is_none() {
            break;
        }
        vol = vol_l.expect("checked Some above");
        vor = vor_r.expect("checked Some above");
        nodes[vor].ancestor = v;
        let shift = (nodes[vil].x + sil) - (nodes[vir].x + sir) + distance;
        if shift > 0.0 {
            layered_buchheim_move_subtree(nodes, default_ancestor, v, shift);
            sir += shift;
        }
        sil += nodes[vil].mod_;
        sir += nodes[vir].mod_;
    }
    default_ancestor
}

pub(super) fn layered_buchheim_first_walk(nodes: &mut [LayeredBuchheimNode], v: usize, distance: f64,control:&mut LayoutControl<'_>) -> Result<usize,LayoutError> {
    control.advance(1)?;
    if nodes[v].children.is_empty() {
        if let Some(lb) = layered_buchheim_left_brother(nodes, v) {
            nodes[v].x = nodes[lb].x + distance;
        } else {
            nodes[v].x = 0.0;
        }
        return Ok(v);
    }
    let mut default_ancestor = nodes[v].children[0];
    for &w in nodes[v].children.clone().iter() {
        layered_buchheim_first_walk(nodes, w, distance,control)?;
        default_ancestor = layered_buchheim_apportion(nodes, w, default_ancestor, distance);
    }
    layered_buchheim_execute_shifts(nodes, v);
    let c0 = nodes[v].children[0];
    let c1 = *nodes[v].children.last().expect("children non-empty per the is_empty check above");
    let mid = (nodes[c0].x + nodes[c1].x) * 0.5;
    if let Some(w) = layered_buchheim_left_brother(nodes, v) {
        nodes[v].x = nodes[w].x + distance;
        nodes[v].mod_ = nodes[v].x - mid;
    } else {
        nodes[v].x = mid;
    }
    Ok(v)
}

pub(super) fn layered_buchheim_second_walk(nodes: &mut [LayeredBuchheimNode], v: usize, m: f64, depth: i32, min_x: f64,control:&mut LayoutControl<'_>) -> Result<f64,LayoutError> {
    control.advance(1)?;
    nodes[v].x += m;
    nodes[v].y = depth as f64;
    let mut min_x = min_x.min(nodes[v].x);
    for &w in nodes[v].children.clone().iter() {
        min_x = layered_buchheim_second_walk(nodes, w, m + nodes[v].mod_, depth + 1, min_x,control)?;
    }
    Ok(min_x)
}

pub(super) fn layered_buchheim_third_walk(nodes: &mut [LayeredBuchheimNode], v: usize, n: f64,control:&mut LayoutControl<'_>)->Result<(),LayoutError> {
    control.advance(1)?;
    nodes[v].x += n;
    for &c in nodes[v].children.clone().iter() {
        layered_buchheim_third_walk(nodes, c, n,control)?;
    }
    Ok(())
}

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
