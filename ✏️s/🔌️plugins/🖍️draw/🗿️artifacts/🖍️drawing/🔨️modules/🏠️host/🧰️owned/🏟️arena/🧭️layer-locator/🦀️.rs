//! 🧭️ Drawing layer locator: the layer address and the locator that finds a layer by identifier and rebuilds its container.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq,semio_framework_value::RetireOwned)]
pub struct DrawingLayerAddress {
    pub length: usize,
    pub indices: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
}

impl DrawingLayerAddress {
    pub fn parent(self) -> Option<Self> {
        (self.length > 1).then(|| Self { length: self.length - 1, indices: self.indices })
    }

    pub fn index(self) -> usize {
        self.indices[self.length - 1]
    }
}

pub struct DrawingLayerLocator {
    pub asset: bool,
    pub root: usize,
    pub depth: usize,
    pub path: [usize; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub frames: [DrawingTraversalFrame; DRAWING_MAXIMUM_LAYER_DEPTH],
    pub found: Option<DrawingLayerAddress>,
    pub equality: native_text_footprint::DrawingTextEqualityCursor,
    pub terminal: bool,
}

impl DrawingLayerLocator {
    pub fn new() -> Self {
        Self { asset:false, root: 0, depth: 0, path: [0; DRAWING_MAXIMUM_LAYER_DEPTH], frames: [DrawingTraversalFrame::EMPTY; DRAWING_MAXIMUM_LAYER_DEPTH], found: None, equality: native_text_footprint::DrawingTextEqualityCursor::default(), terminal: false }
    }
    pub fn asset_reference()->Self{let mut cursor=Self::new();cursor.asset=true;cursor}

    pub fn node_at(snapshot: &DrawingSnapshot, address: DrawingLayerAddress) -> Option<&DrawingLayerNode> {
        let mut value = snapshot.layers.get(address.indices[0])?;
        for index in &address.indices[1..address.length] {
            let DrawingLayerNode::Group(group) = value else { return None };
            value = group.children.get(*index)?;
        }
        Some(value)
    }

    pub fn node_at_mut(snapshot: &mut DrawingSnapshot, address: DrawingLayerAddress) -> Option<&mut DrawingLayerNode> {
        fn descend<'a>(value: &'a mut DrawingLayerNode, path: &[usize]) -> Option<&'a mut DrawingLayerNode> {
            let Some((head, tail)) = path.split_first() else { return Some(value) };
            let DrawingLayerNode::Group(group) = value else { return None };
            descend(group.children.get_mut(*head)?, tail)
        }
        let value = snapshot.layers.get_mut(address.indices[0])?;
        descend(value, &address.indices[1..address.length])
    }

    pub fn container_mut(snapshot: &mut DrawingSnapshot, parent: Option<DrawingLayerAddress>) -> Option<&mut DrawingNativeLayers> {
        match parent {
            None => Some(&mut snapshot.layers),
            Some(address) => match Self::node_at_mut(snapshot, address)? {
                DrawingLayerNode::Group(group) => Some(&mut group.children),
                _ => None,
            },
        }
    }

    pub fn step(&mut self, snapshot: &DrawingSnapshot, target: &DrawingNativeText, cx: &mut semio_framework_job::StepContext<'_>) -> Result<bool, &'static str> {
        if self.terminal {
            return Ok(true);
        }
        let Some(root) = snapshot.layers.get(self.root) else {
            self.terminal = true;
            return Ok(true);
        };
        let node = DrawingSnapshotBoundsAuthority::layer_at(root, &self.path[..self.depth]).ok_or("drawing-store.mutation-locator-path")?;
        if self.frames[self.depth].phase == 0 {
            let key=if self.asset{match node{DrawingLayerNode::Image(image)=>Some(&image.image_key),DrawingLayerNode::Trace(trace)=>Some(&trace.source_key),_=>None}}else{Some(semio_s_artifact_draw_drawing_model::layer_id(node))};
            let Some(key)=key else{self.frames[self.depth].phase=1;cx.consume_fuel(1);return Ok(false);};
            let compared = self.equality.step(key, target, 1, DRAWING_OWNED_FIELD_BYTES)?;
            cx.consume_fuel(compared.compared_bytes.max(1) as u64);
            if !compared.complete { return Ok(false); }
            let equal = self.equality.result().ok_or("drawing-store.mutation-locator-equality-incomplete")?;
            self.equality.close_step(1);
            self.equality = native_text_footprint::DrawingTextEqualityCursor::default();
            self.frames[self.depth].phase = 1;
            if equal {
                let mut indices = [0; DRAWING_MAXIMUM_LAYER_DEPTH];
                indices[0] = self.root;
                if self.depth > 0 {
                    indices[1..self.depth + 1].copy_from_slice(&self.path[..self.depth]);
                }
                self.found = Some(DrawingLayerAddress { length: self.depth + 1, indices });
                self.terminal = true;
            }
            cx.consume_fuel(1);
            return Ok(self.terminal);
        }
        if let DrawingLayerNode::Group(group) = node {
            let child = self.frames[self.depth].child;
            if child < group.children.len() {
                if self.depth + 1 >= DRAWING_MAXIMUM_LAYER_DEPTH {
                    return Err("drawing-store.mutation-locator-depth");
                }
                self.frames[self.depth].child += 1;
                self.path[self.depth] = child;
                self.depth += 1;
                self.frames[self.depth] = DrawingTraversalFrame::EMPTY;
                cx.consume_fuel(1);
                return Ok(false);
            }
        }
        if self.depth == 0 {
            self.root += 1;
            self.frames[0] = DrawingTraversalFrame::EMPTY;
        } else {
            self.depth -= 1;
        }
        cx.consume_fuel(1);
        Ok(false)
    }

    pub fn found(&self) -> Option<DrawingLayerAddress> {
        self.found
    }
}

#[path="🫴️original/🦀️.rs"]mod original;
pub use original::DrawingOriginalLookup;
