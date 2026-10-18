//! 🔬️ Interaction records and note edit of the DAG model: the note edit state, the interaction capacities, the pointer phase, intent, move, offset, projection, snapshot and plan records and the graph edit enum. See [`super`].

use super::*;

// #endregion 🔖️Lod

// #region 🔖️NoteEdit

#[derive(Clone, Debug)]
pub struct NoteEditState {
    pub node_id: String,
    pub caret: usize,
    pub anchor: usize,
}

// #endregion 🔖️NoteEdit

// #region 🔖️DagHost

pub const DAG_INTERACTION_NODE_CAPACITY: usize = 256;

pub const DAG_INTERACTION_WORD_CAPACITY: usize = DAG_INTERACTION_NODE_CAPACITY / 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DagInteractionPlanFault {
    NodeCredits,
    StringCredits,
    Unsupported,
}

#[derive(Clone, Copy, Debug)]
pub enum DagPointerPhase {
    Down,
    Move,
    Up,
    Leave,
}

#[derive(Clone, Copy, Debug)]
pub struct DagPointerIntent {
    pub phase: DagPointerPhase,
    pub x: f64,
    pub y: f64,
    pub button: u8,
    pub shift: bool,
    pub ctrl_or_meta: bool,
    pub alt: bool,
    pub pan: bool,
}

#[derive(Clone, Copy, Debug)]
pub struct DagNodeMove {
    pub index: u16,
    pub x: f64,
    pub y: f64,
}

/// ✋️ How far a drag moved its nodes, and the press (the projection revision it began at) that names the gesture.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DagDragOffset {
    pub press: u64,
    pub dx: f64,
    pub dy: f64,
}

#[derive(Clone, Copy, Debug)]
#[expect(clippy::large_enum_variant, reason = "Pointer plans copy the complete fixed-capacity gesture snapshot before atomic admission; boxing drag state would allocate and break Copy.")]
pub enum DagProjectionGesture {
    Idle,
    Pan { start_x: f64, start_y: f64, camera: [f64; 3] },
    Drag { press: u64, start_x: f64, start_y: f64, starts: [Option<DagNodeMove>; DAG_INTERACTION_NODE_CAPACITY], len: u16 },
    Select { start_x: f64, start_y: f64, initial: [u64; DAG_INTERACTION_WORD_CAPACITY] },
}

#[derive(Clone, Copy, Debug)]
pub struct DagInteractionProjection {
    pub revision: u64,
    pub camera: [f64; 3],
    pub selected: [u64; DAG_INTERACTION_WORD_CAPACITY],
    pub hover: Option<u16>,
    pub gesture: DagProjectionGesture,
}

/// 📍️ Owned selection, hover and camera values from an admitted pointer projection.
pub struct DagPointerSnapshot {
    pub node_ids: Vec<String>,
    pub hovered_id: Option<String>,
    pub camera: [f64; 3],
}

#[derive(Clone, Copy, Debug)]
pub struct DagPointerPlan {
    pub expected_revision: u64,
    pub previous_active: bool,
    pub next: DagInteractionProjection,
    pub moves: [Option<DagNodeMove>; DAG_INTERACTION_NODE_CAPACITY],
    pub move_len: u16,
    /// 🫃️ The offset this plan's pointer moved the drag by, from its press — `None` while it never left the drag's own
    /// origin. A press-and-release that never moved derives a ZERO-delta drag whose `moves` are the positions the nodes
    /// already hold, and journalling those would spend a guest mutation on a plain click.
    pub drag: Option<DagDragOffset>,
}

pub fn dag_bit_contains(bits: &[u64; DAG_INTERACTION_WORD_CAPACITY], index: usize) -> bool {
    bits.get(index / 64).is_some_and(|word| word & (1_u64 << (index % 64)) != 0)
}

pub fn dag_bit_set(bits: &mut [u64; DAG_INTERACTION_WORD_CAPACITY], index: usize, selected: bool) {
    if let Some(word) = bits.get_mut(index / 64) {
        let mask = 1_u64 << (index % 64);
        if selected {
            *word |= mask;
        } else {
            *word &= !mask;
        }
    }
}

impl DagInteractionProjection {
    pub fn revision(&self) -> u64 {
        self.revision
    }

    pub fn camera(&self) -> [f64; 3] {
        self.camera
    }

    pub fn selected(&self, index: usize) -> bool {
        dag_bit_contains(&self.selected, index)
    }

    pub fn hover(&self) -> Option<usize> {
        self.hover.map(usize::from)
    }

    /// 🫳️ Whether a BOUNDED gesture — a node drag, a marquee, a pan — is in flight in this projection.
    /// A gesture belongs to the path that STARTED it: without this, a drag whose pointer crossed a port
    /// or an inline widget was handed to the screen path mid-flight, which resynced the projection and
    /// erased the drag (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    pub fn gesture_active(&self) -> bool {
        !matches!(self.gesture, DagProjectionGesture::Idle)
    }
}

impl DagPointerPlan {
    pub fn projection(&self) -> &DagInteractionProjection {
        &self.next
    }

    pub fn expected_revision(&self) -> u64 {
        self.expected_revision
    }

    pub fn gesture_active(&self) -> bool {
        !matches!(self.next.gesture, DagProjectionGesture::Idle)
    }

    pub fn previous_gesture_active(&self) -> bool {
        self.previous_active
    }

    pub fn move_len(&self) -> usize {
        usize::from(self.move_len)
    }
}

/// 🔗️ One graph edit a completed gesture performed on this host's own graph, in the guest's own `nodeGraphEdit` row
/// vocabulary (design §13.3) rather than in engine handle/edge ids: `connect`, `disconnect` by synapse id, `move` (the
/// press, the moved node ids and their ONE relative offset), `setSlider` (the value an inline slider was released on) and
/// `insertPort` (a variadic port inserted at `index`). Every renderer — React and wgpu alike — drains these after a
/// gesture and dispatches exactly them ([`dag_graph_edit_rows_json`]); no renderer ever publishes the whole snapshot.
#[derive(Clone, Debug, PartialEq)]
pub enum DagGraphEdit {
    Connect { source_node_id: String, source_port_id: String, target_node_id: String, target_port_id: String },
    Disconnect { synapse_id: String },
    Move { gesture_id: String, node_ids: Vec<String>, dx: f64, dy: f64 },
    SetSlider { node_id: String, value: f64 },
    InsertPort { node_id: String, side: DagPortSide, index: usize },
}
