//! 🧪️ Facade test host of `reactor-surfaces`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[cfg(test)]
#[path = "🧱️unit-1/🦀️.rs"]
pub(crate) mod reactor_surfaces_unit_1;
