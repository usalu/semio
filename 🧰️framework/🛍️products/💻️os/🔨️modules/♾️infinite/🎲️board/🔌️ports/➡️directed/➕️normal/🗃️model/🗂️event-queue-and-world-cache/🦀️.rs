//! 🗂️ Event queue and world cache of the board model: the event queue, node icon spans, the world content cache, property retirement credits, pointer and descriptor capacities and the descriptor admission. See [`super`].

use super::*;

pub struct BoardEventQueue {
    pub slots: Box<[Option<BoardOwnedEvent>; BOARD_EVENT_ITEM_CAPACITY]>,
    pub(super) head: u16,
    pub len: u16,
    pub bytes: usize,
    pub claimed_items: u16,
    pub claimed_bytes: usize,
    pub closing: bool,
}

impl Default for BoardEventQueue {
    fn default() -> Self {
        Self { slots: semio_framework_async::boxed_fixed_slots(|| None), head: 0, len: 0, bytes: 0, claimed_items: 0, claimed_bytes: 0, closing: false }
    }
}

impl BoardEventQueue {
    pub fn reserve(&self, items: usize, bytes: usize) -> Result<(), BoardEventFault> {
        if self.closing || usize::from(self.len).checked_add(usize::from(self.claimed_items)).and_then(|value| value.checked_add(items)).is_none_or(|value| value > BOARD_EVENT_ITEM_CAPACITY) {
            return Err(BoardEventFault::ItemCredits);
        }
        if self.bytes.checked_add(self.claimed_bytes).and_then(|value| value.checked_add(bytes)).is_none_or(|value| value > BOARD_EVENT_BYTE_CAPACITY) {
            return Err(BoardEventFault::ByteCredits);
        }
        Ok(())
    }

    pub fn claim(&mut self, items: usize, bytes: usize) -> Result<(), BoardEventFault> {
        self.reserve(items, bytes)?;
        let items = u16::try_from(items).map_err(|_| BoardEventFault::ItemCredits)?;
        self.claimed_items += items;
        self.claimed_bytes += bytes;
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn push_claimed(&mut self, event: BoardOwnedEvent) -> Result<(), BoardOwnedEvent> {
        let bytes = event.owned_bytes();
        if self.closing || self.claimed_items == 0 || self.claimed_bytes < bytes || usize::from(self.len) == BOARD_EVENT_ITEM_CAPACITY {
            return Err(event);
        }
        self.claimed_items -= 1;
        self.claimed_bytes -= bytes;
        let index = (usize::from(self.head) + usize::from(self.len)) % BOARD_EVENT_ITEM_CAPACITY;
        self.bytes += bytes;
        self.slots[index] = Some(event);
        self.len += 1;
        Ok(())
    }

    pub fn release_claim(&mut self, items: usize, bytes: usize) -> bool {
        let Ok(items) = u16::try_from(items) else {
            return false;
        };
        if self.claimed_items < items || self.claimed_bytes < bytes {
            return false;
        }
        self.claimed_items -= items;
        self.claimed_bytes -= bytes;
        true
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn push(&mut self, event: BoardOwnedEvent) -> Result<(), BoardOwnedEvent> {
        if self.reserve(1, event.owned_bytes()).is_err() {
            return Err(event);
        }
        let index = (usize::from(self.head) + usize::from(self.len)) % BOARD_EVENT_ITEM_CAPACITY;
        self.bytes += event.owned_bytes();
        self.slots[index] = Some(event);
        self.len += 1;
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn reserve_event(&self, event: BoardOwnedEvent) -> Result<BoardEventReservation, BoardOwnedEvent> {
        if self.reserve(1, event.owned_bytes()).is_err() {
            return Err(event);
        }
        Ok(BoardEventReservation { expected_len: self.len, expected_bytes: self.bytes, event: Some(event) })
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn publish_reserved(&mut self, mut reservation: BoardEventReservation) -> Result<(), BoardEventReservation> {
        if self.closing || self.len != reservation.expected_len || self.bytes != reservation.expected_bytes {
            return Err(reservation);
        }
        let event = reservation.event.take().expect("board reservation owns one event");
        let index = (usize::from(self.head) + usize::from(self.len)) % BOARD_EVENT_ITEM_CAPACITY;
        self.bytes += event.owned_bytes();
        self.slots[index] = Some(event);
        self.len += 1;
        Ok(())
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn reserve_batch(&self, mut reservation: BoardEventBatchReservation) -> Result<BoardEventBatchReservation, BoardEventBatchReservation> {
        let Some(bytes) = reservation.owned_bytes() else {
            return Err(reservation);
        };
        if self.reserve(usize::from(reservation.len), bytes).is_err() {
            return Err(reservation);
        }
        reservation.expected_len = self.len;
        reservation.expected_bytes = self.bytes;
        Ok(reservation)
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn publish_batch(&mut self, mut reservation: BoardEventBatchReservation) -> Result<(), BoardEventBatchReservation> {
        if self.closing || self.len != reservation.expected_len || self.bytes != reservation.expected_bytes {
            return Err(reservation);
        }
        for index in 0..usize::from(reservation.len) {
            let event = reservation.events[index].take().expect("board batch slot is occupied");
            let slot = (usize::from(self.head) + usize::from(self.len)) % BOARD_EVENT_ITEM_CAPACITY;
            self.bytes += event.owned_bytes();
            self.slots[slot] = Some(event);
            self.len += 1;
        }
        Ok(())
    }

    pub fn pop(&mut self) -> Option<BoardOwnedEvent> {
        if self.len == 0 {
            return None;
        }
        let index = usize::from(self.head);
        let event = self.slots[index].take()?;
        self.bytes -= event.owned_bytes();
        self.head = ((index + 1) % BOARD_EVENT_ITEM_CAPACITY) as u16;
        self.len -= 1;
        Some(event)
    }

    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &BoardOwnedEvent> {
        (0..usize::from(self.len)).filter_map(|offset| self.slots[(usize::from(self.head) + offset) % BOARD_EVENT_ITEM_CAPACITY].as_ref())
    }

    pub fn owned_bytes(&self) -> usize {
        self.bytes
    }

    pub fn close_step(&mut self) -> bool {
        self.closing = true;
        if self.pop().is_some() {
            return false;
        }
        true
    }

    pub fn terminal_is_empty(&self) -> bool {
        self.len == 0 && self.bytes == 0 && self.claimed_items == 0 && self.claimed_bytes == 0
    }
}

/// 🎨️ One node's icon commands inside [`WorldContentCache::scene`], baked at `(x, y)`.
pub struct NodeIconSpan {
    pub id: String,
    pub start: usize,
    pub end: usize,
    pub x: f64,
    pub y: f64,
}

/// 🎨️ World-space node icons reused across frames. Spans stay valid under translation: a drag
/// draws each span at `current - baked` until retirement frees a slot for a fresh bake.
pub struct WorldContentCache {
    pub generation: u64,
    pub lod: BoardDrawLod,
    pub scene: Scene,
    pub spans: Vec<NodeIconSpan>,
}

#[cfg(any(test, feature = "testkit"))]
    pub fn board_property_retirement_credits(properties: &semio_framework_graph_manifest::PropertyBag) -> Result<(usize, usize), BoardEventFault> {
        let mut stack: Box<[Option<(&semio_framework_graph_manifest::PropertyValue, u16)>; BOARD_POINTER_ITEM_CAPACITY]> = Box::new(std::array::from_fn(|_| None));
        let mut stack_len = 0usize;
        let mut nodes = 0usize;
        let mut bytes = 0usize;
        for (key, value) in properties {
            bytes = bytes.checked_add(key.len()).ok_or(BoardEventFault::ByteCredits)?;
            if stack_len == BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            stack[stack_len] = Some((value, 1));
            stack_len += 1;
        }
        while stack_len > 0 {
            stack_len -= 1;
            let (value, depth) = stack[stack_len].take().expect("property pre-admission stack owner");
            nodes += 1;
            if nodes > BOARD_POINTER_ITEM_CAPACITY || usize::from(depth) > BOARD_POINTER_ITEM_CAPACITY {
                return Err(BoardEventFault::ItemCredits);
            }
            match value {
                semio_framework_graph_manifest::PropertyValue::String(value) => {
                    bytes = bytes.checked_add(value.len()).ok_or(BoardEventFault::ByteCredits)?;
                }
                semio_framework_graph_manifest::PropertyValue::Array(values) => {
                    for value in values {
                        if stack_len == BOARD_POINTER_ITEM_CAPACITY {
                            return Err(BoardEventFault::ItemCredits);
                        }
                        stack[stack_len] = Some((value, depth + 1));
                        stack_len += 1;
                    }
                }
                semio_framework_graph_manifest::PropertyValue::Object(values) => {
                    for (key, value) in values {
                        bytes = bytes.checked_add(key.len()).ok_or(BoardEventFault::ByteCredits)?;
                        if stack_len == BOARD_POINTER_ITEM_CAPACITY {
                            return Err(BoardEventFault::ItemCredits);
                        }
                        stack[stack_len] = Some((value, depth + 1));
                        stack_len += 1;
                    }
                }
                semio_framework_graph_manifest::PropertyValue::Null | semio_framework_graph_manifest::PropertyValue::Bool(_) | semio_framework_graph_manifest::PropertyValue::Number(_) => {}
            }
            if bytes > BOARD_POINTER_BYTE_CAPACITY {
                return Err(BoardEventFault::ByteCredits);
            }
        }
        Ok((nodes, bytes))
    }

/// 🧮️ Entities (nodes + handles + edges + wires) one board descriptor, selection or gesture payload
/// may carry. Sized for the largest shipped board example: Nakagin Capsule Tower is 180 nodes
/// + 358 handles + 179 edges = 717 entities — the old 256 refused it silently (`load_board_snapshot_json`
/// → `false`, three empty panes, 2026-09-16).
pub const BOARD_POINTER_ITEM_CAPACITY: usize = 1_024;

/// 🧮️ Entity-id bytes the same payloads may carry — Nakagin's UUID ids total ~29 KiB; kept under the
/// guest's 64 KiB contiguous-request ceiling since every owner here is one boxed array.
pub const BOARD_POINTER_BYTE_CAPACITY: usize = 48 * 1024;

/// 🧮️ Entities one board DESCRIPTOR may carry — the render/hit-test census, decoupled from the
/// gesture payload credits above: a 100-placement fill of an 11-handle kind is 1 200
/// entities, and a board must keep painting past what one pointer payload may name (a whole-board
/// selection above the pointer credits is refused by `select_set`'s own `ItemCredits` law instead).
pub const BOARD_DESCRIPTOR_ITEM_CAPACITY: usize = 8_192;

/// 🧮️ Entity-id bytes one board descriptor may carry (UUID ids: 8 192 × ~45 B).
pub const BOARD_DESCRIPTOR_BYTE_CAPACITY: usize = 384 * 1024;

#[derive(Clone, Copy, Debug)]
pub struct BoardPointerSpan {
    pub start: u16,
    pub len: u16,
}

/// 🛂️ Every refusal a descriptor sync can raise BEFORE it touches the scene: the two fixed
/// descriptor ceilings and the handle colours it would have to parse. A sync that fails halfway
/// leaves a half-built board, and a snapshot parse that clears first leaves an EMPTY one (three
/// blank panes, 2026-09-17), so both preflight through here and mutate only once it passes.
pub fn descriptor_admission(desc: &SceneDescriptor) -> Result<(), NormalPortError> {
    let entity_count = desc
        .nodes
        .len()
        .checked_add(desc.handles.len())
        .and_then(|count| count.checked_add(desc.edges.len()))
        .and_then(|count| count.checked_add(desc.wires.len()))
        .and_then(|count| count.checked_add(desc.regions.len()))
        .ok_or(NormalPortError::EventCredits)?;
    if entity_count > BOARD_DESCRIPTOR_ITEM_CAPACITY {
        return Err(NormalPortError::EventCredits);
    }
    let entity_id_bytes = desc
        .nodes
        .iter()
        .map(|node| node.id.len())
        .chain(desc.handles.iter().map(|handle| handle.id.len()))
        .chain(desc.edges.iter().map(|edge| edge.id.len()))
        .chain(desc.wires.iter().map(|wire| wire.id.len()))
        .chain(desc.regions.iter().map(|region| region.id.len()))
        .try_fold(0usize, usize::checked_add)
        .ok_or(NormalPortError::EventCredits)?;
    if entity_id_bytes > BOARD_DESCRIPTOR_BYTE_CAPACITY {
        return Err(NormalPortError::EventCredits);
    }
    for handle in &desc.handles {
        if let Some(color) = handle.color.as_ref().map(|value| value.trim()).filter(|value| !value.is_empty()) {
            parse_css_color(color).ok_or_else(|| NormalPortError::InvalidHandleColor(handle.id.clone(), color.to_string()))?;
        }
    }
    Ok(())
}

/// 🧾️ The pure half of [`Self::load_board_snapshot_json`]: one snapshot document into one scene
/// descriptor, or `None` for a document this port refuses. Touches no board state.
///
/// 🎯️ Target regions are optional by construction (the artifact omits an empty collection),
/// so a malformed row is skipped rather than refusing the whole document: a board that
/// cannot paint its constraint rectangles must still paint its graph.
pub fn board_snapshot_scene_descriptor(f:BoardSnapshot,has_ports:bool)->Option<SceneDescriptor>{
    let text=|value:Option<String>|value.map(|v|v.trim().to_owned()).filter(|v|!v.is_empty());
    let positive=|value:Option<f64>|value.filter(|v|v.is_finite()&&*v>0.0);
    let mut desc=SceneDescriptor::default();
    for node in f.nodes {
        let(x,y)=(node.x?,node.y?);if !x.is_finite()||!y.is_finite(){return None;}
        let visible=board_visible_option(&node.visibility());let locked=board_locked_option(&node.visibility());
        if has_ports {for h in node.handles? {let angle=h.angle?;if !angle.is_finite(){return None;}let hv=board_visible_option(&h.visibility());let hl=board_locked_option(&h.visibility());desc.handles.push(HandleDescriptor{id:h.id,node_id:node.id.clone(),angle,radius:positive(h.radius),scale:positive(h.scale),selected:None,style:None,handle_kind:Some(text(h.handle_kind).unwrap_or_else(||"port".into())),color:text(h.color),icon_kind:text(h.icon_kind),user_data:None,visible:hv,locked:hl});}}
        else if node.handles.is_some(){return None;}
        let (shape,radius,width,height)=if node.shape==Some(BoardNodeShape::Rectangle){("rectangle",None,Some(positive(node.width)?),Some(positive(node.height)?))}else{("circle",Some(positive(node.radius)?),None,None)};
        desc.nodes.push(NodeDescriptor{id:node.id,x,y,draggable:None,selected:None,style:None,text:text(node.text),icon_kind:text(node.icon_kind),node_kind:text(node.node_kind),user_data:None,visible,locked,root:node.root,shape:Some(shape.into()),radius,width,height,scale:positive(node.scale)});
    }
    for edge in f.edges {let visible=board_visible_option(&edge.visibility());let locked=board_locked_option(&edge.visibility());let(source,target)=(edge.source?,edge.target?);if !has_ports{let ids:BTreeSet<&str>=desc.nodes.iter().map(|n|n.id.as_str()).collect();if !ids.contains(source.as_str())||!ids.contains(target.as_str()){return None;}}
        desc.edges.push(EdgeDescriptor{id:edge.id,source,target,edge_kind:text(edge.edge_kind),source_tip:text(edge.source_tip),target_tip:text(edge.target_tip),selected:None,style:None,user_data:None,visible,locked});}
    for region in f.target_regions {let Some(id)=text(Some(region.id))else{continue;};let finite=|value:Option<f64>|value.filter(|v|v.is_finite()).unwrap_or(0.0);desc.regions.push(RegionDescriptor{id,x:finite(region.x),y:finite(region.y),width:finite(region.width),height:finite(region.height),label:text(region.label),hidden:region.hidden,locked:region.locked,selected:region.selected});}Some(desc)
}
