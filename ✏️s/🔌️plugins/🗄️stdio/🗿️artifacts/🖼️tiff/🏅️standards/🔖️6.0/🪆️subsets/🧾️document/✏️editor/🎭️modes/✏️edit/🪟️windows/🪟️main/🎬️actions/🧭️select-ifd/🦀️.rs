//! 🧭️ User action payload for selecting a TIFF image page.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SelectIfdAction {
    pub ifd_index: usize,
}
