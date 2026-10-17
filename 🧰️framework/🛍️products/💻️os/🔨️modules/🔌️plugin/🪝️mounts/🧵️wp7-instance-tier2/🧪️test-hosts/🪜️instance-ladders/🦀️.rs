//! 🧪️ Facade test host of `instance-ladders`: the cfg(test) units that name higher layers or facade fixtures and therefore cannot run inside the crate.

#[cfg(test)]
#[path = "🧱️unit-1/🦀️.rs"]
pub(crate) mod instance_ladders_unit_1;
