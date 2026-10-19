//! 🧬️ Commit encoder and payload of the board fill: the commit layout constants, the encode stage, the commit encoder, the commit payload and the commit candidate impl. See [`super`].

use super::*;

const BOARD_FILL_COMMIT_MAGIC: [u8; 4] = *b"P7BF";

const BOARD_FILL_COMMIT_VERSION: u8 = 1;

const BOARD_FILL_COMMIT_HEADER_BYTES: usize = 20;

const BOARD_FILL_COMMIT_TEXT_SLOT_BYTES: usize = 2 + BOARD_FILL_TEXT_BYTES;

const BOARD_FILL_COMMIT_NODE_ID_OFFSET: usize = BOARD_FILL_COMMIT_HEADER_BYTES;

const BOARD_FILL_COMMIT_EDGE_ID_OFFSET: usize = BOARD_FILL_COMMIT_NODE_ID_OFFSET + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;

const BOARD_FILL_COMMIT_EDGE_KIND_OFFSET: usize = BOARD_FILL_COMMIT_EDGE_ID_OFFSET + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;

const BOARD_FILL_COMMIT_NODE_KIND_OFFSET: usize = BOARD_FILL_COMMIT_EDGE_KIND_OFFSET + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;

const BOARD_FILL_COMMIT_SOURCE_OFFSET: usize = BOARD_FILL_COMMIT_NODE_KIND_OFFSET + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;

const BOARD_FILL_COMMIT_TARGET_OFFSET: usize = BOARD_FILL_COMMIT_SOURCE_OFFSET + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;

const BOARD_FILL_COMMIT_TARGET_INDEX_OFFSET: usize = BOARD_FILL_COMMIT_TARGET_OFFSET + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;

const BOARD_FILL_COMMIT_X_OFFSET: usize = BOARD_FILL_COMMIT_TARGET_INDEX_OFFSET + 8;

const BOARD_FILL_COMMIT_Y_OFFSET: usize = BOARD_FILL_COMMIT_X_OFFSET + 8;

const BOARD_FILL_COMMIT_SHAPE_OFFSET: usize = BOARD_FILL_COMMIT_Y_OFFSET + 8;

const BOARD_FILL_COMMIT_RADIUS_OFFSET: usize = BOARD_FILL_COMMIT_SHAPE_OFFSET + 1;

const BOARD_FILL_COMMIT_WIDTH_OFFSET: usize = BOARD_FILL_COMMIT_RADIUS_OFFSET + 8;

const BOARD_FILL_COMMIT_HEIGHT_OFFSET: usize = BOARD_FILL_COMMIT_WIDTH_OFFSET + 8;

const BOARD_FILL_COMMIT_ICON_PRESENT_OFFSET: usize = BOARD_FILL_COMMIT_HEIGHT_OFFSET + 8;

const BOARD_FILL_COMMIT_ICON_OFFSET: usize = BOARD_FILL_COMMIT_ICON_PRESENT_OFFSET + 1;

const BOARD_FILL_COMMIT_HANDLE_COUNT_OFFSET: usize = BOARD_FILL_COMMIT_ICON_OFFSET + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;

const BOARD_FILL_COMMIT_HANDLE_OFFSET: usize = BOARD_FILL_COMMIT_HANDLE_COUNT_OFFSET + 2;

const BOARD_FILL_COMMIT_HANDLE_BYTES: usize = BOARD_FILL_COMMIT_TEXT_SLOT_BYTES * 2 + 8 + 1 + 8;

pub const BOARD_FILL_COMMIT_BYTES: usize = BOARD_FILL_COMMIT_HANDLE_OFFSET + BOARD_FILL_KIND_HANDLE_CAPACITY * BOARD_FILL_COMMIT_HANDLE_BYTES;

pub const BOARD_FILL_COMMIT_PAGE_COUNT: usize = BOARD_FILL_COMMIT_BYTES.div_ceil(semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum BoardFillCommitEncodeStage {
    Header,
    Version,
    Presence,
    Stalled,
    Accepted,
    Search,
    NodeId,
    EdgeId,
    EdgeKind,
    NodeKind,
    Source,
    Target,
    TargetIndex,
    X,
    Y,
    Shape,
    Radius,
    Width,
    Height,
    Icon,
    IconText,
    HandleCount,
    HandleId,
    HandleKind,
    HandleAngle,
    HandleRadius,
    HandleRadiusValue,
    Complete,
}

pub struct BoardFillCommitEncoder {
    pub bytes: [u8; BOARD_FILL_COMMIT_BYTES],
    pub stage: BoardFillCommitEncodeStage,
    pub handle: usize,
    pub text_byte: usize,
    text_length_written: bool,
    pub output_cursor: usize,
}

impl BoardFillCommitEncoder {
    pub fn new() -> Self {
        Self { bytes: [0; BOARD_FILL_COMMIT_BYTES], stage: BoardFillCommitEncodeStage::Header, handle: 0, text_byte: 0, text_length_written: false, output_cursor: 0 }
    }

    fn step_text(&mut self, offset: usize, text: &BoardFillText) -> bool {
        if !self.text_length_written {
            self.bytes[offset..offset + 2].copy_from_slice(&text.len.to_le_bytes());
            self.text_length_written = true;
            return false;
        }
        if self.text_byte < usize::from(text.len) {
            self.bytes[offset + 2 + self.text_byte] = text.bytes[self.text_byte];
            self.text_byte += 1;
            if self.text_byte < usize::from(text.len) {
                return false;
            }
        }
        self.text_byte = 0;
        self.text_length_written = false;
        true
    }

    pub fn step(&mut self, state: &BoardFillJobState) -> Result<bool, &'static str> {
        let placement = state.pending_placement.as_ref();
        match self.stage {
            BoardFillCommitEncodeStage::Header => {
                self.bytes[..4].copy_from_slice(&BOARD_FILL_COMMIT_MAGIC);
                self.stage = BoardFillCommitEncodeStage::Version;
            }
            BoardFillCommitEncodeStage::Version => {
                self.bytes[4] = BOARD_FILL_COMMIT_VERSION;
                self.stage = BoardFillCommitEncodeStage::Presence;
            }
            BoardFillCommitEncodeStage::Presence => {
                self.bytes[5] = u8::from(placement.is_some());
                self.stage = BoardFillCommitEncodeStage::Stalled;
            }
            BoardFillCommitEncodeStage::Stalled => {
                self.bytes[6] = u8::from(state.stalled);
                self.stage = BoardFillCommitEncodeStage::Accepted;
            }
            BoardFillCommitEncodeStage::Accepted => {
                self.bytes[8..12].copy_from_slice(&state.accepted_count.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::Search;
            }
            BoardFillCommitEncodeStage::Search => {
                self.bytes[12..20].copy_from_slice(&state.search_count.to_le_bytes());
                self.stage = if placement.is_some() { BoardFillCommitEncodeStage::NodeId } else { BoardFillCommitEncodeStage::Complete };
            }
            BoardFillCommitEncodeStage::NodeId => {
                if self.step_text(BOARD_FILL_COMMIT_NODE_ID_OFFSET, &placement.ok_or("fill-commit-placement")?.node_id) {
                    self.stage = BoardFillCommitEncodeStage::EdgeId;
                }
            }
            BoardFillCommitEncodeStage::EdgeId => {
                if self.step_text(BOARD_FILL_COMMIT_EDGE_ID_OFFSET, &placement.ok_or("fill-commit-placement")?.edge_id) {
                    self.stage = BoardFillCommitEncodeStage::EdgeKind;
                }
            }
            BoardFillCommitEncodeStage::EdgeKind => {
                if self.step_text(BOARD_FILL_COMMIT_EDGE_KIND_OFFSET, &placement.ok_or("fill-commit-placement")?.edge_kind) {
                    self.stage = BoardFillCommitEncodeStage::NodeKind;
                }
            }
            BoardFillCommitEncodeStage::NodeKind => {
                if self.step_text(BOARD_FILL_COMMIT_NODE_KIND_OFFSET, &placement.ok_or("fill-commit-placement")?.node_kind) {
                    self.stage = BoardFillCommitEncodeStage::Source;
                }
            }
            BoardFillCommitEncodeStage::Source => {
                if self.step_text(BOARD_FILL_COMMIT_SOURCE_OFFSET, &placement.ok_or("fill-commit-placement")?.source_handle_id) {
                    self.stage = BoardFillCommitEncodeStage::Target;
                }
            }
            BoardFillCommitEncodeStage::Target => {
                if self.step_text(BOARD_FILL_COMMIT_TARGET_OFFSET, &placement.ok_or("fill-commit-placement")?.target_handle_id) {
                    self.stage = BoardFillCommitEncodeStage::TargetIndex;
                }
            }
            BoardFillCommitEncodeStage::TargetIndex => {
                let index = u64::try_from(placement.ok_or("fill-commit-placement")?.target_handle_index).map_err(|_| "fill-commit-target-index")?;
                self.bytes[BOARD_FILL_COMMIT_TARGET_INDEX_OFFSET..BOARD_FILL_COMMIT_X_OFFSET].copy_from_slice(&index.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::X;
            }
            BoardFillCommitEncodeStage::X => {
                self.bytes[BOARD_FILL_COMMIT_X_OFFSET..BOARD_FILL_COMMIT_Y_OFFSET].copy_from_slice(&placement.ok_or("fill-commit-placement")?.x.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::Y;
            }
            BoardFillCommitEncodeStage::Y => {
                self.bytes[BOARD_FILL_COMMIT_Y_OFFSET..BOARD_FILL_COMMIT_SHAPE_OFFSET].copy_from_slice(&placement.ok_or("fill-commit-placement")?.y.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::Shape;
            }
            BoardFillCommitEncodeStage::Shape => {
                self.bytes[BOARD_FILL_COMMIT_SHAPE_OFFSET] = match placement.ok_or("fill-commit-placement")?.shape {
                    "circle" => 0,
                    "rectangle" => 1,
                    _ => return Err("fill-commit-shape"),
                };
                self.stage = BoardFillCommitEncodeStage::Radius;
            }
            BoardFillCommitEncodeStage::Radius => {
                self.bytes[BOARD_FILL_COMMIT_RADIUS_OFFSET..BOARD_FILL_COMMIT_WIDTH_OFFSET].copy_from_slice(&placement.ok_or("fill-commit-placement")?.radius.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::Width;
            }
            BoardFillCommitEncodeStage::Width => {
                self.bytes[BOARD_FILL_COMMIT_WIDTH_OFFSET..BOARD_FILL_COMMIT_HEIGHT_OFFSET].copy_from_slice(&placement.ok_or("fill-commit-placement")?.width.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::Height;
            }
            BoardFillCommitEncodeStage::Height => {
                self.bytes[BOARD_FILL_COMMIT_HEIGHT_OFFSET..BOARD_FILL_COMMIT_ICON_PRESENT_OFFSET].copy_from_slice(&placement.ok_or("fill-commit-placement")?.height.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::Icon;
            }
            BoardFillCommitEncodeStage::Icon => {
                if placement.ok_or("fill-commit-placement")?.icon_kind.as_ref().is_some() {
                    self.bytes[BOARD_FILL_COMMIT_ICON_PRESENT_OFFSET] = 1;
                    self.stage = BoardFillCommitEncodeStage::IconText;
                } else {
                    self.stage = BoardFillCommitEncodeStage::HandleCount;
                }
            }
            BoardFillCommitEncodeStage::IconText => {
                if self.step_text(BOARD_FILL_COMMIT_ICON_OFFSET, placement.ok_or("fill-commit-placement")?.icon_kind.as_ref().ok_or("fill-commit-icon")?) {
                    self.stage = BoardFillCommitEncodeStage::HandleCount;
                }
            }
            BoardFillCommitEncodeStage::HandleCount => {
                let count = placement.ok_or("fill-commit-placement")?.handle_count();
                let count = u16::try_from(count).map_err(|_| "fill-commit-handle-count")?;
                self.bytes[BOARD_FILL_COMMIT_HANDLE_COUNT_OFFSET..BOARD_FILL_COMMIT_HANDLE_OFFSET].copy_from_slice(&count.to_le_bytes());
                self.handle = 0;
                self.stage = BoardFillCommitEncodeStage::HandleId;
            }
            BoardFillCommitEncodeStage::HandleId => {
                let placement = placement.ok_or("fill-commit-placement")?;
                let Some(handle) = placement.handles.get(self.handle) else {
                    self.stage = BoardFillCommitEncodeStage::Complete;
                    return Ok(false);
                };
                let offset = BOARD_FILL_COMMIT_HANDLE_OFFSET + self.handle * BOARD_FILL_COMMIT_HANDLE_BYTES;
                if self.step_text(offset, &handle.id) {
                    self.stage = BoardFillCommitEncodeStage::HandleKind;
                }
            }
            BoardFillCommitEncodeStage::HandleKind => {
                let handle = placement.ok_or("fill-commit-placement")?.handles.get(self.handle).ok_or("fill-commit-handle")?;
                let offset = BOARD_FILL_COMMIT_HANDLE_OFFSET + self.handle * BOARD_FILL_COMMIT_HANDLE_BYTES + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES;
                if self.step_text(offset, &handle.template.handle_kind) {
                    self.stage = BoardFillCommitEncodeStage::HandleAngle;
                }
            }
            BoardFillCommitEncodeStage::HandleAngle => {
                let handle = placement.ok_or("fill-commit-placement")?.handles.get(self.handle).ok_or("fill-commit-handle")?;
                let offset = BOARD_FILL_COMMIT_HANDLE_OFFSET + self.handle * BOARD_FILL_COMMIT_HANDLE_BYTES + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES * 2;
                self.bytes[offset..offset + 8].copy_from_slice(&handle.template.angle.to_le_bytes());
                self.stage = BoardFillCommitEncodeStage::HandleRadius;
            }
            BoardFillCommitEncodeStage::HandleRadius => {
                let handle = placement.ok_or("fill-commit-placement")?.handles.get(self.handle).ok_or("fill-commit-handle")?;
                let offset = BOARD_FILL_COMMIT_HANDLE_OFFSET + self.handle * BOARD_FILL_COMMIT_HANDLE_BYTES + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES * 2 + 8;
                if let Some(radius) = handle.template.radius {
                    self.bytes[offset] = 1;
                    let _ = radius;
                    self.stage = BoardFillCommitEncodeStage::HandleRadiusValue;
                } else {
                    self.handle += 1;
                    self.stage = BoardFillCommitEncodeStage::HandleId;
                }
            }
            BoardFillCommitEncodeStage::HandleRadiusValue => {
                let handle = placement.ok_or("fill-commit-placement")?.handles.get(self.handle).ok_or("fill-commit-handle")?;
                let offset = BOARD_FILL_COMMIT_HANDLE_OFFSET + self.handle * BOARD_FILL_COMMIT_HANDLE_BYTES + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES * 2 + 9;
                self.bytes[offset..offset + 8].copy_from_slice(&handle.template.radius.ok_or("fill-commit-handle-radius")?.to_le_bytes());
                self.handle += 1;
                self.stage = BoardFillCommitEncodeStage::HandleId;
            }
            BoardFillCommitEncodeStage::Complete => return Ok(true),
        }
        Ok(false)
    }
}

struct BoardFillCommitPayload<'a> {
    pub payload: &'a semio_framework_job_payload::RetainedJobPayload,
}

impl<'a> BoardFillCommitPayload<'a> {
    pub fn new(payload: &'a semio_framework_job_payload::RetainedJobPayload) -> Option<Self> {
        (payload.len() == BOARD_FILL_COMMIT_BYTES && payload.page_count() == BOARD_FILL_COMMIT_PAGE_COUNT && payload.page(0)?.len() == BOARD_FILL_COMMIT_BYTES).then_some(Self { payload })
    }

    pub fn read<const N: usize>(&self, offset: usize) -> Option<[u8; N]> {
        let end = offset.checked_add(N)?;
        if end > BOARD_FILL_COMMIT_BYTES {
            return None;
        }
        let mut output = [0; N];
        let mut copied = 0usize;
        while copied < N {
            let absolute = offset + copied;
            let page_index = absolute / semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
            let page_offset = absolute % semio_framework_job_payload::JOB_PAYLOAD_PAGE_BYTES;
            let page = self.payload.page(page_index)?;
            let count = (N - copied).min(page.len().checked_sub(page_offset)?);
            if count == 0 {
                return None;
            }
            output[copied..copied + count].copy_from_slice(&page[page_offset..page_offset + count]);
            copied += count;
        }
        Some(output)
    }

    pub fn byte(&self, offset: usize) -> Option<u8> {
        Some(self.read::<1>(offset)?[0])
    }
}

impl BoardFillCommitCandidate {
    fn read_text(payload: &BoardFillCommitPayload<'_>, offset: usize) -> Option<BoardFillText> {
        let length = u16::from_le_bytes(payload.read::<2>(offset)?);
        if usize::from(length) > BOARD_FILL_TEXT_BYTES {
            return None;
        }
        let mut text = BoardFillText::empty();
        text.bytes = payload.read::<BOARD_FILL_TEXT_BYTES>(offset + 2)?;
        text.len = length;
        std::str::from_utf8(&text.bytes[..usize::from(length)]).ok()?;
        Some(text)
    }

    fn read_f64(payload: &BoardFillCommitPayload<'_>, offset: usize) -> Option<f64> {
        let value = f64::from_le_bytes(payload.read::<8>(offset)?);
        value.is_finite().then_some(value)
    }

    pub fn from_complete(state: Option<&semio_framework_job_payload::RetainedJobPayload>, output: Option<&semio_framework_job_payload::RetainedJobPayload>) -> Option<Self> {
        let payload = BoardFillCommitPayload::new(output?)?;
        if state.is_some_and(|state| !state.is_empty()) || payload.read::<4>(0)? != BOARD_FILL_COMMIT_MAGIC || payload.byte(4)? != BOARD_FILL_COMMIT_VERSION {
            return None;
        }
        let result = BoardFillResult {
            accepted_count: u32::from_le_bytes(payload.read::<4>(8)?),
            stalled: match payload.byte(6)? {
                0 => false,
                1 => true,
                _ => return None,
            },
            search_count: u64::from_le_bytes(payload.read::<8>(12)?),
        };
        let placement = match payload.byte(5)? {
            0 => None,
            1 => {
                let handle_count = usize::from(u16::from_le_bytes(payload.read::<2>(BOARD_FILL_COMMIT_HANDLE_COUNT_OFFSET)?));
                if handle_count > BOARD_FILL_KIND_HANDLE_CAPACITY {
                    return None;
                }
                let handles = std::array::from_fn(|index| {
                    if index >= handle_count {
                        return None;
                    }
                    let offset = BOARD_FILL_COMMIT_HANDLE_OFFSET + index * BOARD_FILL_COMMIT_HANDLE_BYTES;
                    let radius_offset = offset + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES * 2 + 8;
                    let radius = match payload.byte(radius_offset)? {
                        0 => None,
                        1 => Some(Self::read_f64(&payload, radius_offset + 1)?),
                        _ => return None,
                    };
                    Some(BoardFillCommitHandle {
                        id: Self::read_text(&payload, offset)?,
                        handle_kind: Self::read_text(&payload, offset + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES)?,
                        angle: Self::read_f64(&payload, offset + BOARD_FILL_COMMIT_TEXT_SLOT_BYTES * 2)?,
                        radius,
                    })
                });
                if handles[..handle_count].iter().any(Option::is_none) {
                    return None;
                }
                let target_handle_index = usize::try_from(u64::from_le_bytes(payload.read::<8>(BOARD_FILL_COMMIT_TARGET_INDEX_OFFSET)?)).ok()?;
                if target_handle_index >= handle_count {
                    return None;
                }
                let shape = match payload.byte(BOARD_FILL_COMMIT_SHAPE_OFFSET)? {
                    0 => BoardFillCommitShape::Circle,
                    1 => BoardFillCommitShape::Rectangle,
                    _ => return None,
                };
                let icon_kind = match payload.byte(BOARD_FILL_COMMIT_ICON_PRESENT_OFFSET)? {
                    0 => None,
                    1 => Some(Self::read_text(&payload, BOARD_FILL_COMMIT_ICON_OFFSET)?),
                    _ => return None,
                };
                Some(BoardFillCommitPlacement {
                    node_id: Self::read_text(&payload, BOARD_FILL_COMMIT_NODE_ID_OFFSET)?,
                    edge_id: Self::read_text(&payload, BOARD_FILL_COMMIT_EDGE_ID_OFFSET)?,
                    edge_kind: Self::read_text(&payload, BOARD_FILL_COMMIT_EDGE_KIND_OFFSET)?,
                    node_kind: Self::read_text(&payload, BOARD_FILL_COMMIT_NODE_KIND_OFFSET)?,
                    source_handle_id: Self::read_text(&payload, BOARD_FILL_COMMIT_SOURCE_OFFSET)?,
                    target_handle_id: Self::read_text(&payload, BOARD_FILL_COMMIT_TARGET_OFFSET)?,
                    target_handle_index,
                    x: Self::read_f64(&payload, BOARD_FILL_COMMIT_X_OFFSET)?,
                    y: Self::read_f64(&payload, BOARD_FILL_COMMIT_Y_OFFSET)?,
                    shape,
                    radius: Self::read_f64(&payload, BOARD_FILL_COMMIT_RADIUS_OFFSET)?,
                    width: Self::read_f64(&payload, BOARD_FILL_COMMIT_WIDTH_OFFSET)?,
                    height: Self::read_f64(&payload, BOARD_FILL_COMMIT_HEIGHT_OFFSET)?,
                    icon_kind,
                    handles,
                    handle_count,
                })
            }
            _ => return None,
        };
        Some(Self { result, placement })
    }
}
