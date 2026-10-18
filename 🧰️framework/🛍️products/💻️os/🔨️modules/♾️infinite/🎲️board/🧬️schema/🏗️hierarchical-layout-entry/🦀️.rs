//! 🏗️ Hierarchical layout entry of the board schema: the hierarchical inner layout and the Buchheim positions. See [`super`].

use super::*;

    /// 🌳️ Writes node centers: Buchheim tidy-tree on a spanning forest (min-depth parent tie-break id), synthetic multi-root; super-root not serialized.
    pub fn apply_hierarchical_inner(snapshot: &mut BoardSnapshot, opts: &HierarchicalTreeLayoutOptions, control:&mut LayoutControl<'_>) -> Result<(), LayoutError> {
        let dir = TreeDirection::parse(&opts.direction)?;
        if snapshot.schema.as_str()!="board.ports.directed.v1" {return Err(LayoutError::Schema("schema must be board.ports.directed.v1".into()));}
        control.advance(snapshot.nodes.len() as u64 +snapshot.edges.len() as u64)?;
        let edges_json=&snapshot.edges;let nodes=&mut snapshot.nodes;
        if nodes.is_empty() {
            return Ok(());
        }
        let mut handle_to_node: HashMap<String, String> = HashMap::new();
        let mut id_to_node: BTreeMap<String, BoardNodeSnapshot> = BTreeMap::new();
        for node in nodes.iter() {
            if !board_visible_or_true(&node.visibility()){continue;}
            id_to_node.insert(node.id.clone(),node.clone());
            for handle in node.handles.as_deref().unwrap_or(&[]){if board_visible_or_true(&handle.visibility()){handle_to_node.insert(handle.id.clone(),node.id.clone());}}
        }
        if id_to_node.is_empty() {
            return Ok(());
        }
        let mut directed: Vec<(String, String)> = Vec::new();
        let mut seen_dir: HashSet<(String, String)> = HashSet::new();
        for e in edges_json {
            if !board_visible_or_true(&e.visibility()){continue;}
            let (Some(src_h),Some(tgt_h))=(e.source.as_deref(),e.target.as_deref()) else {continue;};
            let source_node_id=handle_to_node.get(src_h).cloned().unwrap_or_else(||src_h.into());
            let target_node_id=handle_to_node.get(tgt_h).cloned().unwrap_or_else(||tgt_h.into());
            if source_node_id == target_node_id {
                continue;
            }
            if !id_to_node.contains_key(&source_node_id) || !id_to_node.contains_key(&target_node_id) {
                continue;
            }
            if seen_dir.insert((source_node_id.clone(), target_node_id.clone())) {
                directed.push((source_node_id, target_node_id));
            }
        }
        let mut incoming_edge_count_by_node: HashMap<String, u32> = HashMap::new();
        for id in id_to_node.keys() {
            incoming_edge_count_by_node.insert(id.clone(), 0);
        }
        for (_source_nid, target_nid) in &directed {
            *incoming_edge_count_by_node.entry(target_nid.clone()).or_insert(0) += 1;
        }
        let mut roots: Vec<String> = Vec::new();
        for node in nodes.iter(){if board_visible_or_true(&node.visibility())&&node.root==Some(true){roots.push(node.id.clone());}}
        roots.sort();
        roots.dedup();
        if roots.is_empty() {
            for (id, &d) in &incoming_edge_count_by_node {
                if d == 0 {
                    roots.push(id.clone());
                }
            }
            roots.sort();
        }
        if roots.is_empty() {
            roots = id_to_node.keys().cloned().collect();
            roots.sort();
        }
        let mut depth: HashMap<String, i32> = HashMap::new();
        for r in &roots {
            depth.insert(r.clone(), 0);
        }
        let cap = directed.len().saturating_mul(3).saturating_add(nodes.len()).saturating_add(8);
        for _ in 0..cap {
            control.advance(directed.len() as u64)?;
            let mut changed = false;
            for (source_nid, target_nid) in &directed {
                let Some(&dp) = depth.get(source_nid) else {
                    continue;
                };
                let nd = dp + 1;
                let cur = *depth.get(target_nid).unwrap_or(&-1);
                if nd > cur {
                    depth.insert(target_nid.clone(), nd);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
        let max_depth = depth.values().copied().max().unwrap_or(0);
        for id in id_to_node.keys() {
            depth.entry(id.clone()).or_insert(max_depth + 1);
        }
        let raw = run_buchheim_layout(control,&id_to_node, &roots, &directed, &depth)?;
        let mean_half: f64 = id_to_node.values().map(half_extent).sum::<f64>() / id_to_node.len().max(1) as f64;
        let along_scale = (opts.sibling_gap + 2.0 * mean_half).max(8.0);
        let mut pos: HashMap<String, (f64, f64)> = HashMap::new();
        for (id, (bx, by)) in raw {
            let along = bx * along_scale;
            let orth = by * opts.layer_spacing;
            let (lx, ly) = match dir {
                TreeDirection::Downwards => (along, orth),
                TreeDirection::Upwards => (along, -orth),
                TreeDirection::Right => (orth, along),
                TreeDirection::Left => (-orth, along),
            };
            pos.insert(id, (lx, ly));
        }
        let mut minx = f64::INFINITY;
        let mut maxx = f64::NEG_INFINITY;
        let mut miny = f64::INFINITY;
        let mut maxy = f64::NEG_INFINITY;
        for (id, (x, y)) in &pos {
            let h = half_extent(id_to_node.get(id).unwrap());
            minx = minx.min(x - h);
            maxx = maxx.max(x + h);
            miny = miny.min(y - h);
            maxy = maxy.max(y + h);
        }
        if !minx.is_finite() {
            minx = 0.0;
            maxx = 1.0;
            miny = 0.0;
            maxy = 1.0;
        }
        let cx = (minx + maxx) * 0.5;
        let cy = (miny + maxy) * 0.5;
        let gx = opts.center_x.unwrap_or(0.0);
        let gy = opts.center_y.unwrap_or(0.0);
        let dx = gx - cx;
        let dy = gy - cy;
        let locked_set: HashSet<String> = opts.locked_node_ids.iter().cloned().collect();
        let mut pinned_world: HashMap<String, (f64, f64)> = HashMap::new();
        if !locked_set.is_empty() {
            for node in nodes.iter(){if board_visible_or_true(&node.visibility())&&locked_set.contains(&node.id)&&id_to_node.contains_key(&node.id){pinned_world.insert(node.id.clone(),(node.x.unwrap_or(0.0),node.y.unwrap_or(0.0)));}}
        }
        control.advance(pos.len() as u64)?;
        for (id, (x, y)) in pos {
            let (fx, fy) = if let Some(&(px, py)) = pinned_world.get(&id) { (px, py) } else { (x + dx, y + dy) };
            let idx = nodes.iter().position(|n| n.id==id).ok_or_else(|| LayoutError::InvalidOptions(format!("node index {id}")))?;
            nodes[idx].x=Some(fx);nodes[idx].y=Some(fy);
        }
        Ok(())
    }

/// 🌲️ Buchheim tidy-tree on string-labeled directed edges.
pub fn buchheim_positions(roots: &[String], directed: &[(String, String)], depth: &HashMap<String, i32>,control:&mut LayoutControl<'_>) -> Result<HashMap<String, (f64, f64)>,LayoutError> {
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
    let mut nodes: Vec<LayeredBuchheimNode> =
        ordered_ids.iter().map(|id| LayeredBuchheimNode { ancestor: 0, change: 0.0, children: vec![], id: id.clone(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: false, thread: None, x: -1.0, y: 0.0 }).collect();
    nodes.push(LayeredBuchheimNode { ancestor: super_idx, change: 0.0, children: vec![], id: "__tree_super__".into(), mod_: 0.0, number: 0, parent: None, shift: 0.0, synthetic: true, thread: None, x: -1.0, y: 0.0 });
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
    layered_buchheim_first_walk(&mut nodes, super_idx, 1.0,control)?;
    let min_x = layered_buchheim_second_walk(&mut nodes, super_idx, 0.0, 0, f64::INFINITY,control)?;
    if min_x.is_finite() && min_x < 0.0 {
        layered_buchheim_third_walk(&mut nodes, super_idx, -min_x,control)?;
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
