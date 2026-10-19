//! 🌳️ Drawing layer tree: the layer accessors, the lookup, flatten and play row id helpers, the clone, update, remove, extract and insert operations over the layer tree and the layer location.

use super::*;

//#region 🔖️DocumentHelpers
//#region 🔖️Tree
pub fn layer_id(layer: &DrawingLayerNode) -> &semio_framework_value::paged::PagedUtf8<{usize::MAX}> {
    match layer {
        DrawingLayerNode::Shape(shape) => &shape.base.id,
        DrawingLayerNode::Path(path) => &path.base.id,
        DrawingLayerNode::Text(text) => &text.base.id,
        DrawingLayerNode::Image(image) => &image.base.id,
        DrawingLayerNode::Group(group) => &group.base.id,
        DrawingLayerNode::Boolean(boolean) => &boolean.base.id,
        DrawingLayerNode::Trace(trace) => &trace.base.id,
    }
}

pub fn layer_base(layer: &DrawingLayerNode) -> &DrawingLayerBase {
    match layer {
        DrawingLayerNode::Shape(shape) => &shape.base,
        DrawingLayerNode::Path(path) => &path.base,
        DrawingLayerNode::Text(text) => &text.base,
        DrawingLayerNode::Image(image) => &image.base,
        DrawingLayerNode::Group(group) => &group.base,
        DrawingLayerNode::Boolean(boolean) => &boolean.base,
        DrawingLayerNode::Trace(trace) => &trace.base,
    }
}

pub fn layer_kind_label(layer: &DrawingLayerNode) -> String {
    match layer {
        DrawingLayerNode::Shape(shape) => format!("shape:{}", shape.shape_kind),
        DrawingLayerNode::Path(_) => "path".into(),
        DrawingLayerNode::Text(_) => "text".into(),
        DrawingLayerNode::Image(_) => "image".into(),
        DrawingLayerNode::Group(_) => "group".into(),
        DrawingLayerNode::Boolean(_) => "boolean".into(),
        DrawingLayerNode::Trace(_) => "trace".into(),
    }
}

pub fn find_drawing_layer<'a>(doc: &'a DrawingSnapshot, layer_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized)) -> Option<&'a DrawingLayerNode> {
    for layer in &doc.layers {
        if let Some(found) = find_drawing_layer_in_node(layer, layer_id) {
            return Some(found);
        }
    }
    None
}

fn find_drawing_layer_in_node<'a>(node: &'a DrawingLayerNode, target_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized)) -> Option<&'a DrawingLayerNode> {
    if layer_id(node).eq_text(target_id) {
        return Some(node);
    }
    if let DrawingLayerNode::Group(group) = node {
        for child in &group.children {
            if let Some(found) = find_drawing_layer_in_node(child, target_id) {
                return Some(found);
            }
        }
    }
    None
}

pub fn selected_drawing_layers<'a>(document: &'a DrawingSnapshot, ids: &[String]) -> Vec<&'a DrawingLayerNode> {
    flatten_drawing_layers(&document.layers).into_iter().filter(|layer| ids.iter().any(|id| id == &layer_base(layer).id || id == &drawing_play_layers_tree_row_id(layer))).collect()
}

/// 🔒️ A lock on any ancestor protects its complete subtree from interactive edits.
pub fn drawing_layer_is_locked(document: &DrawingSnapshot, id: &(impl semio_framework_value::paged::Utf8Text + ?Sized)) -> bool {
    fn visit(layers: &semio_framework_value_persistent::list::PagedList<DrawingLayerNode, {usize::MAX}>, id: &(impl semio_framework_value::paged::Utf8Text + ?Sized), inherited: bool) -> Option<bool> {
        for layer in layers {
            let locked = inherited || layer_base(layer).locked;
            if layer_id(layer).eq_text(id) { return Some(locked); }
            if let DrawingLayerNode::Group(group) = layer {
                if let Some(found) = visit(&group.children, id, locked) { return Some(found); }
            }
        }
        None
    }
    visit(&document.layers, id, false).unwrap_or(false)
}

pub fn flatten_drawing_layers(layers: &semio_framework_value_persistent::list::PagedList<DrawingLayerNode, {usize::MAX}>) -> Vec<&DrawingLayerNode> {
    let mut out = Vec::new();
    fn walk<'a>(nodes: &'a semio_framework_value_persistent::list::PagedList<DrawingLayerNode, {usize::MAX}>, out: &mut Vec<&'a DrawingLayerNode>) {
        for node in nodes {
            out.push(node);
            if let DrawingLayerNode::Group(group) = node {
                walk(&group.children, out);
            }
        }
    }
    walk(layers, &mut out);
    out
}

pub fn drawing_play_layers_tree_row_id(layer: &DrawingLayerNode) -> String {
    let segment = match layer {
        DrawingLayerNode::Group(_) => "group",
        DrawingLayerNode::Boolean(_) => "boolean",
        DrawingLayerNode::Trace(_) => "trace",
        DrawingLayerNode::Path(_) => "path",
        DrawingLayerNode::Shape(_) => "shape",
        DrawingLayerNode::Text(_) => "text",
        DrawingLayerNode::Image(_) => "image",
    };
    format!("drawing-play-layers.{segment}.{}", layer_id(layer))
}

pub fn drawing_play_boolean_child_row_id(boolean_id: &(impl semio_framework_value::paged::Utf8Text + std::fmt::Display + ?Sized), child_id: &(impl std::fmt::Display + ?Sized)) -> String {
    format!("drawing-play-layers.boolean-child.{}.{boolean_id}{child_id}", boolean_id.text_bytes())
}

pub fn drawing_play_layer_id_from_tree_row_id(row_id: &str) -> Option<String> {
    let (kind, id) = row_id.strip_prefix("drawing-play-layers.")?.split_once('.')?;
    if kind == "boolean-child" {
        let (length, ids) = id.split_once('.')?;
        let child = ids.get(length.parse::<usize>().ok()?..)?;
        return (!child.is_empty()).then(|| child.to_string());
    }
    if !matches!(kind, "group" | "boolean" | "trace" | "path" | "shape" | "text" | "image") || id.is_empty() { return None; }
    Some(id.to_string())
}

pub fn clone_drawing_layer_node(node: &DrawingLayerNode, name_suffix: &str, identities: &semio_framework_value_persistent::list::PagedList<DrawingIdentityAssignment,{usize::MAX}>) -> Result<DrawingLayerNode,semio_framework_value::ValueError> {
    fn identify(node: &mut DrawingLayerNode, identities: &semio_framework_value_persistent::list::PagedList<DrawingIdentityAssignment,{usize::MAX}>, ids: &mut BTreeMap<semio_framework_value::paged::PagedUtf8<{usize::MAX}>, semio_framework_value::paged::PagedUtf8<{usize::MAX}>>) -> Result<(),semio_framework_value::ValueError> {
        let base=layer_base_mut(node);let old=base.id.clone();
        let mut matches=identities.iter().filter(|assignment|assignment.source==old);
        let assignment=matches.next().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Drawing clone identity missing"))?;
        if matches.next().is_some()||assignment.target.is_empty()||assignment.target==old||ids.contains_key(&old)||ids.values().any(|target|*target==assignment.target){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Drawing clone identity repeated or invalid"));}
        base.id=assignment.target.clone();ids.insert(old,base.id.clone());
        if let DrawingLayerNode::Group(group)=node{for child in &mut group.children{identify(child,identities,ids)?;}}Ok(())
    }
    fn references(node:&mut DrawingLayerNode,ids:&BTreeMap<semio_framework_value::paged::PagedUtf8<{usize::MAX}>,semio_framework_value::paged::PagedUtf8<{usize::MAX}>>){
        match node{DrawingLayerNode::Boolean(boolean)=>{for child in &mut boolean.children{if let Some(id)=ids.get(child){*child=id.clone();}}},DrawingLayerNode::Group(group)=>{for child in &mut group.children{references(child,ids);}},_=>{}}
    }
    let mut cloned=node.clone();let mut ids=BTreeMap::new();identify(&mut cloned,identities,&mut ids)?;
    if identities.len()!=ids.len()||ids.values().any(|target|ids.contains_key(target)){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"Drawing clone identity census differs"));}
    layer_base_mut(&mut cloned).name.push_str(name_suffix);references(&mut cloned,&ids);Ok(cloned)
}

pub fn layer_base_mut(layer: &mut DrawingLayerNode) -> &mut DrawingLayerBase {
    match layer {
        DrawingLayerNode::Shape(shape) => &mut shape.base,
        DrawingLayerNode::Path(path) => &mut path.base,
        DrawingLayerNode::Text(text) => &mut text.base,
        DrawingLayerNode::Image(image) => &mut image.base,
        DrawingLayerNode::Group(group) => &mut group.base,
        DrawingLayerNode::Boolean(boolean) => &mut boolean.base,
        DrawingLayerNode::Trace(trace) => &mut trace.base,
    }
}

pub fn mutate_drawing_layer(doc: &DrawingSnapshot, target_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized), mutator: impl FnMut(&mut DrawingLayerNode)) -> DrawingSnapshot {
    let mut next = doc.clone();
    let mut mutator = mutator;
    update_layer_in_tree(&mut next.layers, target_id, &mut mutator);
    next
}

pub fn update_layer_in_tree(layers: &mut semio_framework_value_persistent::list::PagedList<DrawingLayerNode, {usize::MAX}>, target_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized), mutator: &mut impl FnMut(&mut DrawingLayerNode)) -> bool {
    for layer in layers.iter_mut() {
        if layer_id(layer).eq_text(target_id) {
            mutator(layer);
            return true;
        }
        if let DrawingLayerNode::Group(group) = layer {
            if update_layer_in_tree(&mut group.children, target_id, mutator) {
                return true;
            }
        }
    }
    false
}

pub fn remove_layer_from_tree(layers: &mut semio_framework_value_persistent::list::PagedList<DrawingLayerNode, {usize::MAX}>, target_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized)) -> bool {
    if let Some(index) = layers.iter().position(|layer| layer_id(layer).eq_text(target_id)) {
        layers.remove(index);
        return true;
    }
    for layer in layers.iter_mut() {
        if let DrawingLayerNode::Group(group) = layer {
            if remove_layer_from_tree(&mut group.children, target_id) {
                return true;
            }
        }
    }
    false
}

pub fn extract_layer_node(layers: &mut semio_framework_value_persistent::list::PagedList<DrawingLayerNode, {usize::MAX}>, target_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized)) -> Option<DrawingLayerNode> {
    if let Some(index) = layers.iter().position(|layer| layer_id(layer).eq_text(target_id)) {
        return Some(layers.remove(index));
    }
    for layer in layers.iter_mut() {
        if let DrawingLayerNode::Group(group) = layer {
            if let Some(node) = extract_layer_node(&mut group.children, target_id) {
                return Some(node);
            }
        }
    }
    None
}

pub fn insert_layer(layers: &mut semio_framework_value_persistent::list::PagedList<DrawingLayerNode, {usize::MAX}>, parent_id: Option<&str>, index: usize, node: DrawingLayerNode) {
    if let Some(parent_id) = parent_id {
        if !insert_layer_in_parent(layers, parent_id, index, node.clone()) {
            layers.push(node);
        }
    } else {
        let at = index.min(layers.len());
        layers.insert(at, node);
    }
}

fn insert_layer_in_parent(layers: &mut semio_framework_value_persistent::list::PagedList<DrawingLayerNode, {usize::MAX}>, parent_id: &str, index: usize, node: DrawingLayerNode) -> bool {
    for layer in layers.iter_mut() {
        if let DrawingLayerNode::Group(group) = layer {
            if group.base.id == parent_id {
                let at = index.min(group.children.len());
                group.children.insert(at, node);
                return true;
            }
            if insert_layer_in_parent(&mut group.children, parent_id, index, node.clone()) {
                return true;
            }
        }
    }
    false
}

#[derive(Clone, Debug, PartialEq, semio_framework_value::ToValue, semio_framework_value::FromValue)]
#[value(rename_all = "camelCase")]
pub struct DrawingLayerLocation {
    #[value(skip_serializing_if = "Option::is_none")]
    pub parent_id: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>,
    pub index: usize,
}

pub fn find_drawing_layer_location(doc: &DrawingSnapshot, target_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized)) -> Option<DrawingLayerLocation> {
    fn search(layers: &semio_framework_value_persistent::list::PagedList<DrawingLayerNode, {usize::MAX}>, parent_id: Option<semio_framework_value::paged::PagedUtf8<{usize::MAX}>>, target_id: &(impl semio_framework_value::paged::Utf8Text + ?Sized)) -> Option<DrawingLayerLocation> {
        for (index, layer) in layers.iter().enumerate() {
            if layer_id(layer).eq_text(target_id) {
                return Some(DrawingLayerLocation { parent_id, index });
            }
            if let DrawingLayerNode::Group(group) = layer {
                if let Some(found) = search(&group.children, Some(group.base.id.clone()), target_id) {
                    return Some(found);
                }
            }
        }
        None
    }
    search(&doc.layers, None, target_id)
}
//#endregion 🔖️Tree

//#region 🔖️SegmentGeometry
pub fn drawing_layer_descendant_leaf_ids(layer: &DrawingLayerNode) -> Vec<String> {
    match layer {
        DrawingLayerNode::Group(group) => group.children.iter().flat_map(drawing_layer_descendant_leaf_ids).collect(),
        _ => vec![layer_id(layer).to_string()],
    }
}
//#endregion 🔖️SegmentGeometry
//#endregion 🔖️DocumentHelpers
