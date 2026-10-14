//! 🪟️ Original generational surface identity shared by layout and execution owners.
pub const UI_LAYOUT_SURFACE_SLOTS: usize = 64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UiSurfaceToken {
    slot: u8,
    generation: u64,
}

impl UiSurfaceToken {
    pub const fn new(slot: u8, generation: u64) -> Self { Self { slot, generation } }
    pub const fn slot(self) -> u8 { self.slot }
    pub const fn generation(self) -> u64 { self.generation }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
