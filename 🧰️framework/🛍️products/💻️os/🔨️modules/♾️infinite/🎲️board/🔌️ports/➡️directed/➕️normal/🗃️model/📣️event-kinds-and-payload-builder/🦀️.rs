//! 📣️ Event kinds and payload builder of the board model: the event capacities, event kinds, pointer offsets, gesture motion, event faults, owned events, the payload builder and the event reservations. See [`super`].

use super::*;

pub const BOARD_EVENT_ITEM_CAPACITY: usize = 256;

pub const BOARD_EVENT_BYTE_CAPACITY: usize = 256 * 1024;

pub const BOARD_EVENT_PAYLOAD_BYTE_CAPACITY: usize = 16 * 1024;

pub const BOARD_EVENT_KEY_BYTE_CAPACITY: usize = 256;

/// 🏷️ Every row kind the board engine publishes. `NodeMove` and `TransformPreview` are live-preview
/// frames hosts mirror into sibling panes and never dispatch; a finished pointer gesture is ONE `Gesture`
/// record (drag, rotate), riding one batch with the `select` row its press staged.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BoardEventKind {
    Camera,
    NodeMove,
    TransformPreview,
    Gesture,
    Select,
    Preselect,
    PreselectCancel,
    Hover,
    BrushPreview,
    BrushCandidates,
    BrushPlace,
    LinkCompatibleNodes,
    LinkTargetRing,
    EdgeCreate,
    EdgeDelete,
    NodeDelete,
    IndirectConnect,
    ProximityConnect,
    RegionCreate,
    RegionResize,
}

impl BoardEventKind {
    pub const ALL: [Self; 20] = [
        Self::Camera,
        Self::NodeMove,
        Self::TransformPreview,
        Self::Gesture,
        Self::Select,
        Self::Preselect,
        Self::PreselectCancel,
        Self::Hover,
        Self::BrushPreview,
        Self::BrushCandidates,
        Self::BrushPlace,
        Self::LinkCompatibleNodes,
        Self::LinkTargetRing,
        Self::EdgeCreate,
        Self::EdgeDelete,
        Self::NodeDelete,
        Self::IndirectConnect,
        Self::ProximityConnect,
        Self::RegionCreate,
        Self::RegionResize,
    ];

    pub fn name(self) -> &'static str {
        match self {
            Self::Camera => "camera",
            Self::NodeMove => "nodeMove",
            Self::TransformPreview => "transformPreview",
            Self::Gesture => "gesture",
            Self::Select => "select",
            Self::Preselect => "preselect",
            Self::PreselectCancel => "preselectCancel",
            Self::Hover => "hover",
            Self::BrushPreview => "brushPreview",
            Self::BrushCandidates => "brushCandidates",
            Self::BrushPlace => "brushPlace",
            Self::LinkCompatibleNodes => "linkCompatibleNodes",
            Self::LinkTargetRing => "linkTargetRing",
            Self::EdgeCreate => "edgeCreate",
            Self::EdgeDelete => "edgeDelete",
            Self::NodeDelete => "nodeDelete",
            Self::IndirectConnect => "indirectConnect",
            Self::ProximityConnect => "proximityConnect",
            Self::RegionCreate => "regionCreate",
            Self::RegionResize => "regionResize",
        }
    }

    /// 🔎️ The kind a row `name` spells, `None` for a name the engine never publishes.
    pub fn parse(name: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.name() == name)
    }
}

/// ✂️ A drag offset measured from pointer input, as the shortest decimal of its f32 value. Pointer coordinates
/// arrive with f32 precision, so every f64 digit below it is representation noise — an 80 px drag reading
/// `80.00003051757813` — that must never become a stored mutation input. A rotation keeps its f64 angle: a snapped
/// angle is an exact multiple whose narrowing would move every orbit off its exact pose.
pub fn board_pointer_offset(value: f64) -> f64 {
    let narrowed = value as f32;
    if !narrowed.is_finite() {
        return value;
    }
    narrowed.to_string().parse().unwrap_or(value)
}

/// 🎬️ What one released pointer gesture did, in its record's own terms: a drag by one offset, or a
/// rotation by `angle` radians (counter-clockwise) about `pivot`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BoardGestureMotion {
    Drag { dx: f64, dy: f64 },
    Rotate { pivot: Point, angle: f64 },
}

#[derive(Debug, PartialEq, Eq)]
pub enum BoardEventFault {
    ItemCredits,
    ByteCredits,
    KeyCredits,
    Schema,
}

#[derive(Debug)]
pub struct BoardOwnedEvent {
    pub kind: BoardEventKind,
    pub payload: Box<[u8; BOARD_EVENT_PAYLOAD_BYTE_CAPACITY]>,
    pub(super) payload_len: u16,
    pub key: [u8; BOARD_EVENT_KEY_BYTE_CAPACITY],
    pub(super) key_len: u16,
}

pub struct BoardPayloadBuilder {
    pub bytes: Box<[u8; BOARD_EVENT_PAYLOAD_BYTE_CAPACITY]>,
    pub len: u16,
}

impl BoardPayloadBuilder {
    pub fn new() -> Self {
        Self { bytes: Box::new([0; BOARD_EVENT_PAYLOAD_BYTE_CAPACITY]), len: 0 }
    }

    pub fn raw(&mut self, value: &str) -> Result<(), BoardEventFault> {
        let start = usize::from(self.len);
        let end = start.checked_add(value.len()).ok_or(BoardEventFault::ByteCredits)?;
        if end > BOARD_EVENT_PAYLOAD_BYTE_CAPACITY {
            return Err(BoardEventFault::ByteCredits);
        }
        self.bytes[start..end].copy_from_slice(value.as_bytes());
        self.len = end as u16;
        Ok(())
    }

    pub fn string(&mut self, value: &str) -> Result<(), BoardEventFault> {
        self.raw("\"")?;
        for character in value.chars() {
            match character {
                '"' => self.raw("\\\"")?,
                '\\' => self.raw("\\\\")?,
                '\n' => self.raw("\\n")?,
                '\r' => self.raw("\\r")?,
                '\t' => self.raw("\\t")?,
                character if character <= '\u{1f}' => {
                    let escaped = format!("\\u{:04x}", character as u32);
                    self.raw(&escaped)?;
                }
                character => {
                    let mut encoded = [0; 4];
                    self.raw(character.encode_utf8(&mut encoded))?;
                }
            }
        }
        self.raw("\"")
    }

    pub(super) fn string_array<'a>(&mut self, values: impl IntoIterator<Item = &'a str>) -> Result<(), BoardEventFault> {
        self.raw("[")?;
        let mut first = true;
        for value in values {
            if !first {
                self.raw(",")?;
            }
            first = false;
            self.string(value)?;
        }
        self.raw("]")
    }

    pub fn number(&mut self, value: f64) -> Result<(), BoardEventFault> {
        if !value.is_finite() {
            return Err(BoardEventFault::Schema);
        }
        self.raw(&value.to_string())
    }

    pub fn usize(&mut self, value: usize) -> Result<(), BoardEventFault> {
        self.raw(&value.to_string())
    }

    pub(super) fn boolean(&mut self, value: bool) -> Result<(), BoardEventFault> {
        self.raw(if value { "true" } else { "false" })
    }

    pub fn finish(self, kind: BoardEventKind, key: Option<&str>) -> Result<BoardOwnedEvent, BoardEventFault> {
        let payload = std::str::from_utf8(&self.bytes[..usize::from(self.len)]).map_err(|_| BoardEventFault::Schema)?;
        BoardOwnedEvent::from_payload(kind, payload, key)
    }

    pub fn finish_owned(self, kind: BoardEventKind) -> BoardOwnedEvent {
        BoardOwnedEvent { kind, payload: self.bytes, payload_len: self.len, key: [0; BOARD_EVENT_KEY_BYTE_CAPACITY], key_len: 0 }
    }
}

pub struct BoardEventReservation {
    pub(super) expected_len: u16,
    pub(super) expected_bytes: usize,
    pub event: Option<BoardOwnedEvent>,
}

const BOARD_EVENT_BATCH_CAPACITY: usize = 4;

pub struct BoardEventBatchReservation {
    pub(super) expected_len: u16,
    pub(super) expected_bytes: usize,
    pub events: [Option<BoardOwnedEvent>; BOARD_EVENT_BATCH_CAPACITY],
    pub len: u8,
    pub cursor: u8,
}

impl BoardEventBatchReservation {
    pub fn one(event: BoardOwnedEvent) -> Self {
        let mut events = std::array::from_fn(|_| None);
        events[0] = Some(event);
        Self { expected_len: 0, expected_bytes: 0, events, len: 1, cursor: 0 }
    }

    #[expect(clippy::result_large_err, reason = "Refusal returns the exact event, reservation, placement or checkpoint without allocating outside the bounded operation.")]
    pub fn push(&mut self, event: BoardOwnedEvent) -> Result<(), BoardOwnedEvent> {
        let index = usize::from(self.len);
        if index == BOARD_EVENT_BATCH_CAPACITY {
            return Err(event);
        }
        self.events[index] = Some(event);
        self.len += 1;
        Ok(())
    }

    pub fn owned_bytes(&self) -> Option<usize> {
        self.events[usize::from(self.cursor)..usize::from(self.len)].iter().try_fold(0usize, |bytes, event| bytes.checked_add(event.as_ref()?.owned_bytes()))
    }

    pub fn peek(&self) -> Option<&BoardOwnedEvent> {
        self.events.get(usize::from(self.cursor))?.as_ref()
    }

    pub fn pop(&mut self) -> Option<BoardOwnedEvent> {
        if self.cursor == self.len {
            return None;
        }
        let event = self.events[usize::from(self.cursor)].take();
        self.cursor += 1;
        event
    }

    pub fn is_empty(&self) -> bool {
        self.cursor == self.len
    }
}
