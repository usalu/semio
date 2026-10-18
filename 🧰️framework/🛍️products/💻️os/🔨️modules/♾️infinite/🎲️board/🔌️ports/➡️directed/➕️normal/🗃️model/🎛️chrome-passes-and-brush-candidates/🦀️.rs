//! 🎛️ Chrome passes and brush candidates of the board model: the style chrome pass, the handle paint layer, brush candidates, the candidate page and the brush preview snapshot. See [`super`].

use super::*;

/// 🎨️ Whether drawable style resolves committed selection chrome or neutral cached geometry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StyleChromePass {
    CachedBase,
    InteractionOverlay,
}

/// 🎨️ Which node/handle primitives to paint in a layered draw pass (fills behind icons/text).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NodeHandlePaintLayer {
    Full,
    Fill,
    Stroke,
    Icons,
}

#[derive(Clone, Copy, Debug)]
struct BrushCandidateEntry {
    pub node_kind_id: BoardPointerSpan,
    pub target_handle_index: u16,
    sort_delta: f64,
}

#[derive(Clone, Copy)]
pub struct BrushCandidateRef<'a> {
    pub node_kind_id: &'a str,
    pub target_handle_index: usize,
}

#[derive(Clone)]
pub struct BrushCandidatePage {
    pub bytes: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
    pub byte_len: u16,
    pub entries: Box<[Option<BrushCandidateEntry>; BOARD_POINTER_ITEM_CAPACITY]>,
    pub len: u16,
}

impl Default for BrushCandidatePage {
    fn default() -> Self {
        Self { bytes: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]), byte_len: 0, entries: Box::new([None; BOARD_POINTER_ITEM_CAPACITY]), len: 0 }
    }
}

impl BrushCandidatePage {
    pub fn push(&mut self, node_kind_id: &str, target_handle_index: usize, sort_delta: f64) -> Result<(), BoardEventFault> {
        let index = usize::from(self.len);
        if index == BOARD_POINTER_ITEM_CAPACITY || target_handle_index > u16::MAX as usize {
            return Err(BoardEventFault::ItemCredits);
        }
        let start = usize::from(self.byte_len);
        let end = start.checked_add(node_kind_id.len()).ok_or(BoardEventFault::ByteCredits)?;
        if end > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        self.bytes[start..end].copy_from_slice(node_kind_id.as_bytes());
        self.byte_len = end as u16;
        self.entries[index] = Some(BrushCandidateEntry { node_kind_id: BoardPointerSpan { start: start as u16, len: node_kind_id.len() as u16 }, target_handle_index: target_handle_index as u16, sort_delta });
        self.len += 1;
        Ok(())
    }

    fn id_from(bytes: &[u8], span: BoardPointerSpan) -> &str {
        let start = usize::from(span.start);
        let end = start + usize::from(span.len);
        std::str::from_utf8(&bytes[start..end]).expect("brush candidate ids originate from UTF-8 strings")
    }

    pub fn sort(&mut self) {
        let bytes = &self.bytes[..usize::from(self.byte_len)];
        self.entries[..usize::from(self.len)].sort_by(|left, right| {
            let left = left.expect("candidate entry");
            let right = right.expect("candidate entry");
            left.sort_delta
                .partial_cmp(&right.sort_delta)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| Self::id_from(bytes, left.node_kind_id).cmp(Self::id_from(bytes, right.node_kind_id)))
                .then_with(|| left.target_handle_index.cmp(&right.target_handle_index))
        });
    }

    pub fn get(&self, index: usize) -> Option<BrushCandidateRef<'_>> {
        let entry = self.entries.get(index)?.as_ref()?;
        Some(BrushCandidateRef { node_kind_id: Self::id_from(self.bytes.as_ref(), entry.node_kind_id), target_handle_index: usize::from(entry.target_handle_index) })
    }

    pub fn first(&self) -> Option<BrushCandidateRef<'_>> {
        self.get(0)
    }

    pub fn iter(&self) -> impl Iterator<Item = BrushCandidateRef<'_>> {
        self.entries[..usize::from(self.len)].iter().flatten().map(|entry| BrushCandidateRef { node_kind_id: Self::id_from(self.bytes.as_ref(), entry.node_kind_id), target_handle_index: usize::from(entry.target_handle_index) })
    }

    pub fn len(&self) -> usize {
        usize::from(self.len)
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    pub fn clear(&mut self) {
        self.byte_len = 0;
        self.len = 0;
    }
}

#[derive(Clone, Debug)]
pub struct BrushPreviewSnapshot {
    pub source_handle_id: String,
    pub node_kind_id: String,
    pub x: f64,
    pub y: f64,
    pub shape: NodeShape,
    pub radius: f64,
    pub width: f64,
    pub height: f64,
    pub handles: Vec<NodeKindHandleTemplate>,
    pub target_handle_index: usize,
    pub icon_kind: Option<String>,
}
