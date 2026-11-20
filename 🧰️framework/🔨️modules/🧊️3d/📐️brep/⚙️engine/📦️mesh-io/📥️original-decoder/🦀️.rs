//! 📥️ Common retained triangle-buffer handoff for original exchange formats.

use super::*;
use semio_framework_value::{RetainedAllocationEffect, RetirementDemand};

/// 🧾️ One original geometric exchange decoder.
pub enum MeshDecode { Obj(ObjDecode), Stl(StlDecode) }

macro_rules! decode {
    ($owner:expr, $method:ident($($input:expr),*)) => {
        match $owner { MeshDecode::Obj(obj) => obj.$method($($input),*), MeshDecode::Stl(stl) => stl.$method($($input),*) }
    };
}

impl MeshDecode {
    pub fn ready(&self) -> bool { decode!(self, ready()) }
    pub fn step(&mut self, source: &str, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneProgress, ValueError> {
        decode!(self, step(source, effect))
    }
    pub fn demand(&self) -> RetirementDemand { decode!(self, demand()) }
    pub fn into_cursor(&mut self) -> Result<MeshImportCursor, ValueError> { decode!(self, into_cursor()) }
    pub fn terminal_is_empty(&self) -> bool { decode!(self, terminal_is_empty()) }
    pub fn close_demand(&self) -> RetirementDemand { decode!(self, close_demand()) }
    pub fn close(&mut self, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, ValueError> { decode!(self, close(effect)) }
}
