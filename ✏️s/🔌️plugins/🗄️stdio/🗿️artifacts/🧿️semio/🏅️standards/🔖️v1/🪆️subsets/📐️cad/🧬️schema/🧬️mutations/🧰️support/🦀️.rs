//! 🧰️ `SemioCadMutation` support: the diff builders and base accessors its mutation leaves call, hosted below the leaves so that no leaf has to import the aggregate module.

use crate::schema::snapshot::CadBlock;
use crate::schema::snapshot::CadEntityRecord;
use crate::schema::snapshot::CadLayer;
use crate::schema::snapshot::SemioCadSnapshot;

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn find_layer<'a>(base: &'a SemioCadSnapshot, name: &str) -> Option<&'a CadLayer> {
    base.layers.iter().find(|l| l.name == name)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn find_block<'a>(base: &'a SemioCadSnapshot, name: &str) -> Option<&'a CadBlock> {
    base.blocks.iter().find(|b| b.name == name)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn find_entity<'a>(base: &'a SemioCadSnapshot, handle: &str) -> Option<&'a CadEntityRecord> {
    base.entities.iter().find(|e| e.handle == handle)
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn find_block_entity<'a>(base: &'a SemioCadSnapshot, block_name: &str, handle: &str) -> Option<&'a CadEntityRecord> {
    find_block(base, block_name)?.entities.iter().find(|e| e.handle == handle)
}
