//! 👆️ Pointer plan of the board scene: the pointer plan kinds and record, its methods, the JSON string byte count, the admitted pointer id and the wheel plan impl. See [`super`].

use super::*;

#[derive(Clone, Copy, Debug)]
pub enum BoardPointerPlanKind {
    Idle,
    Pan {
        camera: [f64; 3],
    },
    DragMove,
    FinishPan {
        camera: [f64; 3],
    },
    FinishDrag,
    SelectionPreview {
        start: Point,
        start_screen: Point,
    },
    SelectionCommit,
    LinkMove {
        source: BoardPointerSpan,
        target: Option<BoardPointerSpan>,
        hover: Option<BoardPointerSpan>,
        compat_key: BoardPointerSpan,
        ring_key: BoardPointerSpan,
        end_world: Point,
        activated: bool,
        start_screen: Point,
    },
    LinkFinish {
        source: BoardPointerSpan,
        target: Option<BoardPointerSpan>,
        edge: Option<BoardPointerSpan>,
        target_node: Option<BoardPointerSpan>,
        hover: Option<BoardPointerSpan>,
        compat_key: Option<BoardPointerSpan>,
        ring_key: Option<BoardPointerSpan>,
    },
    LinkRetain {
        hover: Option<BoardPointerSpan>,
    },
    Hover {
        hover: Option<BoardPointerSpan>,
    },
    Brush {
        source: Option<BoardPointerSpan>,
        hover: Option<BoardPointerSpan>,
        alt: bool,
        commit_old: bool,
    },
}

#[derive(Debug)]
pub struct BoardPointerPlan {
    pub revision: u64,
    pub kind: BoardPointerPlanKind,
    pub bytes: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
    pub byte_len: u16,
    pub deltas: Box<[Option<BoardPointerDelta>; BOARD_POINTER_ITEM_CAPACITY]>,
    pub delta_len: u16,
    pub points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]>,
    pub screen_points: Box<[Point; BOARD_POINTER_ITEM_CAPACITY]>,
    pub point_len: u16,
    pub output: Box<[u8; BOARD_POINTER_BYTE_CAPACITY]>,
    pub output_len: u16,
}

impl BoardPointerPlan {
    pub fn empty(revision: u64, kind: BoardPointerPlanKind) -> Self {
        Self {
            revision,
            kind,
            bytes: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]),
            byte_len: 0,
            deltas: Box::new([None; BOARD_POINTER_ITEM_CAPACITY]),
            delta_len: 0,
            points: Box::new(std::array::from_fn(|_| Point::new(0.0, 0.0))),
            screen_points: Box::new(std::array::from_fn(|_| Point::new(0.0, 0.0))),
            point_len: 0,
            output: Box::new([0; BOARD_POINTER_BYTE_CAPACITY]),
            output_len: 0,
        }
    }

    pub fn push_id(&mut self, id: &str) -> Result<BoardPointerSpan, BoardPointerPlanFault> {
        if id.len() > u16::MAX as usize {
            return Err(BoardPointerPlanFault::ByteCredits);
        }
        let start = usize::from(self.byte_len);
        let end = start.checked_add(id.len()).ok_or(BoardPointerPlanFault::ByteCredits)?;
        if end > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardPointerPlanFault::ByteCredits);
        }
        self.bytes[start..end].copy_from_slice(id.as_bytes());
        self.byte_len = end as u16;
        Ok(BoardPointerSpan { start: start as u16, len: id.len() as u16 })
    }

    pub fn push_delta(&mut self, id: &str, x: f64, y: f64) -> Result<(), BoardPointerPlanFault> {
        let index = usize::from(self.delta_len);
        if index == BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardPointerPlanFault::ItemCredits);
        }
        let id = self.push_id(id)?;
        self.deltas[index] = Some(BoardPointerDelta { id, x, y });
        self.delta_len += 1;
        Ok(())
    }

    pub fn id(&self, span: BoardPointerSpan) -> &str {
        let start = usize::from(span.start);
        let end = start + usize::from(span.len);
        std::str::from_utf8(&self.bytes[start..end]).expect("board pointer ids originate from UTF-8 strings")
    }

    pub(crate) fn push_point(&mut self, world: Point, screen: Point) -> Result<(), BoardPointerPlanFault> {
        let index = usize::from(self.point_len);
        if index == BOARD_POINTER_ITEM_CAPACITY {
            return Err(BoardPointerPlanFault::ItemCredits);
        }
        self.points[index] = world;
        self.screen_points[index] = screen;
        self.point_len += 1;
        Ok(())
    }

    #[cfg(any(test, feature = "testkit"))]
    pub fn selection_ids(&self) -> impl Iterator<Item = &str> {
        self.deltas[..usize::from(self.delta_len)].iter().flatten().map(|delta| self.id(delta.id))
    }

    /// 🎥️ The camera this plan settles on the view lane: a finished pan's. A camera move is view state, so the host
    /// publishes it as its view verb (`setCamera`) and never as a board event.
    pub fn view_camera(&self) -> Option<[f64; 3]> {
        match self.kind {
            BoardPointerPlanKind::FinishPan { camera } => Some(camera),
            _ => None,
        }
    }

    /// 📮️ Whether committing this plan puts anything on the host's action queue: board events or a view camera.
    pub fn publishes(&self) -> bool {
        self.event_count() > 0 || self.view_camera().is_some()
    }

    pub fn event_count(&self) -> usize {
        match self.kind {
            BoardPointerPlanKind::FinishDrag => usize::from(self.output_len > 2),
            BoardPointerPlanKind::SelectionPreview { .. } | BoardPointerPlanKind::SelectionCommit => 1,
            BoardPointerPlanKind::LinkMove { .. } => usize::from(self.output_len > 2),
            BoardPointerPlanKind::LinkFinish { .. } => usize::from(self.output_len > 2),
            BoardPointerPlanKind::LinkRetain { .. } => usize::from(self.output_len > 2),
            BoardPointerPlanKind::Hover { .. } => usize::from(self.output_len > 2),
            BoardPointerPlanKind::Brush { .. } => usize::from(self.output_len > 2),
            _ => 0,
        }
    }

    pub fn requires_retained_commit(&self) -> bool {
        !matches!(self.kind, BoardPointerPlanKind::Idle)
    }

    /// 🈳️ Seals the empty event page of a hand-built plan that publishes no board event.
    #[cfg(any(test, feature = "testkit"))]
    pub fn seal_events(&mut self) -> Result<(), BoardPointerPlanFault> {
        self.output_len = 0;
        self.output_raw("[]")
    }

    fn output_raw(&mut self, value: &str) -> Result<(), BoardPointerPlanFault> {
        let start = usize::from(self.output_len);
        let end = start.checked_add(value.len()).ok_or(BoardPointerPlanFault::ByteCredits)?;
        if end > BOARD_POINTER_BYTE_CAPACITY {
            return Err(BoardPointerPlanFault::ByteCredits);
        }
        self.output[start..end].copy_from_slice(value.as_bytes());
        self.output_len = end as u16;
        Ok(())
    }

    pub(crate) fn seal_owned_event(&mut self, event: &BoardOwnedEvent) -> Result<(), BoardPointerPlanFault> {
        self.output_len = 0;
        self.output_raw("[{\"name\":\"")?;
        self.output_raw(event.kind().name())?;
        self.output_raw("\",\"payload\":")?;
        self.output_raw(event.payload_json())?;
        self.output_raw("}]")
    }

    pub fn seal_optional_events<const N: usize>(&mut self, events: &[Option<BoardOwnedEvent>; N]) -> Result<(), BoardPointerPlanFault> {
        self.output_len = 0;
        self.output_raw("[")?;
        for (emitted, event) in events.iter().flatten().enumerate() {
            if emitted > 0 {
                self.output_raw(",")?;
            }
            self.output_raw("{\"name\":\"")?;
            self.output_raw(event.kind().name())?;
            self.output_raw("\",\"payload\":")?;
            self.output_raw(event.payload_json())?;
            self.output_raw("}")?;
        }
        self.output_raw("]")
    }

    pub fn events_json(&self) -> &str {
        std::str::from_utf8(&self.output[..usize::from(self.output_len)]).expect("board event page is encoded from UTF-8 schema tokens")
    }

}

pub fn board_json_string_bytes(value: &str) -> usize {
    value
        .chars()
        .map(|character| match character {
            '"' | '\\' | '\n' | '\r' | '\t' => 2,
            character if character <= '\u{1f}' => 6,
            character => character.len_utf8(),
        })
        .try_fold(2usize, usize::checked_add)
        .unwrap_or(usize::MAX)
}

pub fn admitted_board_pointer_id(value: &str) -> Result<String, BoardEventFault> {
    if value.len() > BOARD_POINTER_BYTE_CAPACITY {
        return Err(BoardEventFault::ByteCredits);
    }
    Ok(value.to_owned())
}

impl BoardWheelPlan {
    pub fn camera(&self) -> [f64; 3] {
        [self.next.x, self.next.y, self.next.zoom]
    }
}
