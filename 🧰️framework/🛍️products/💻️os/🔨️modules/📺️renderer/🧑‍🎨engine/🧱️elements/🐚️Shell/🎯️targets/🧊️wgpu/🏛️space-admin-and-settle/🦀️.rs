//! 🏛️ Space administration, directory queue, hub tasks and the settle pump.
//!
//! Space administration operations, the directory command queue, hub workspace tasks, boot program selection,
//! the settle pump and its verdicts, pointer capture and the presented-input geometry.
//!
//! One chunk of the former `shell` monolith (ticket CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT); [`ShellState`] and the
//! shared imports stay in the parent module and are reached through `use super::*;`.

use super::*;

//#region ShellTypes
//#region 🎮️DirectoryCommandQueue
/// 🎮️ Bounded (contract §C6 "bounded, in-memory") native directory-command FIFO depth.
pub(crate) const MAX_PENDING_DIRECTORY_COMMANDS: usize = 64;
/// 🧾️ Bounded transient command-result slot depth.
pub(crate) const MAX_DIRECTORY_COMMAND_RESULTS: usize = 64;

//#endregion 🎮️DirectoryCommandQueue



/// 📏️ Items one retirement step may return. A page, never one item — the unit the rest of this
/// shell's close ladder is already priced in (`📓️close-ladder-budget-2026-09-12.md` §3).
pub(crate) const SHELL_DOCUMENT_RETIREMENT_ITEMS: usize = 1_024;

/// 📏️ Bytes one retirement step may return. Kept under the guest contiguous request ceiling so a
/// grant never asks an arena for a block `dlmalloc` cannot serve once fragmented.
pub(crate) const SHELL_DOCUMENT_RETIREMENT_BYTES: usize = 32 * 1_024;

/// 🎯️🖱️ The layer that owns the pointer SEQUENCE in flight — the DOM's implicit pointer capture,
/// which the wgpu target has to keep itself because it has one canvas and no event targets.
///
/// ⚖️ A press captures the layer it landed on and every move and the release belong to it until the
/// release ends the sequence. Both directions matter: a press on chrome keeps the whole gesture away
/// from the engine surface under it (React's `pointerdown` on a `<button>` means the canvas never
/// sees that `pointerup`), and a press on the surface keeps an orbit drag on the surface even where
/// it sweeps over a panel (three's `OrbitControls` calls `setPointerCapture` on the canvas).
///
/// 🩸️ Without it every chrome click ALSO reached the world pane painted under it — the reference
/// journals one `noteShellCommand` for a navbar panel tab where wgpu journalled
/// `noteShellCommand` + `interactionHover` + `interactionSelect`, and the stray
/// `interactionSelect` carried empty `targets`, i.e. pressing a panel tab CLEARED the world
/// selection (ticket 26/09/17/WGPU-RENDERER-REACT-PARITY,
/// `📓️w11a-prepared-world-mesh-missing.md` §6 family A).
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PointerCapture {
    pub(crate) slots: [Option<PointerCaptureSlot>; crate::element_contracts::SCENE_POINTER_OWNER_CAPACITY],
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct PointerCaptureSlot {
    pub(crate) pointer_id: ui_render::PointerId,
    pub(crate) owner: PointerHitOwner,
    pub(crate) position: [f32; 2],
}

impl PointerCapture {
    pub fn press(&mut self, pointer_id: ui_render::PointerId, owner: PointerHitOwner, x: f32, y: f32) -> Option<PointerHitOwner> {
        let index = self.slots.iter().position(|slot| slot.is_some_and(|slot| slot.pointer_id == pointer_id)).or_else(|| self.slots.iter().position(Option::is_none))?;
        self.slots[index] = Some(PointerCaptureSlot { pointer_id, owner, position: [x, y] });
        Some(owner)
    }

    /// 🖱️⬆️ Ends the sequence and answers the layer that owned it. A release with no press before it
    /// — the pointer entered the canvas already down — is the surface's, as it is in the DOM.
    pub fn release(&mut self, pointer_id: ui_render::PointerId) -> PointerHitOwner {
        self.slots.iter_mut().find(|slot| slot.is_some_and(|slot| slot.pointer_id == pointer_id)).and_then(Option::take).map_or(PointerHitOwner::Surface, |slot| slot.owner)
    }

    pub fn owner_of_move(&self, pointer_id: ui_render::PointerId, under_pointer: PointerHitOwner) -> PointerHitOwner {
        self.holder(pointer_id).unwrap_or(under_pointer)
    }

    pub fn holder(&self, pointer_id: ui_render::PointerId) -> Option<PointerHitOwner> {
        self.slots.iter().flatten().find(|slot| slot.pointer_id == pointer_id).map(|slot| slot.owner)
    }

    pub fn position(&self, pointer_id: ui_render::PointerId) -> Option<[f32; 2]> {
        self.slots.iter().flatten().find(|slot| slot.pointer_id == pointer_id).map(|slot| slot.position)
    }

    pub fn advance(&mut self, pointer_id: ui_render::PointerId, x: f32, y: f32) -> Option<(f32, f32)> {
        let slot = self.slots.iter_mut().flatten().find(|slot| slot.pointer_id == pointer_id)?;
        let delta = (x - slot.position[0], y - slot.position[1]);
        slot.position = [x, y];
        Some(delta)
    }

    pub fn any_active(&self) -> bool {
        self.slots.iter().any(Option::is_some)
    }
}

//#endregion ShellTypes
