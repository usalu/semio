//! 🧪️ Facade test host of `tool-run`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[cfg(test)]
#[path = "🧱️unit-1/🦀️.rs"]
pub(crate) mod tool_run_unit_1;
