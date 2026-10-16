//! 🧪️ Facade test host of `reactor-turn`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[cfg(test)]
#[path = "🧱️unit-1/🦀️.rs"]
pub(crate) mod reactor_turn_unit_1;

#[cfg(test)]
#[path = "🧱️unit-2/🦀️.rs"]
pub(crate) mod reactor_turn_unit_2;

#[cfg(test)]
#[path = "🧱️unit-3/🦀️.rs"]
pub(crate) mod reactor_turn_unit_3;
