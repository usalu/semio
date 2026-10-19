//! 🥣️ tui cross-crate laws: engine behaviour, overlays, pointer routing, chrome anatomy, render equivalence and the unit corpus.

#[path = "../🔬️text-elements/🦀️.rs"]
#[cfg(test)]
mod text_elements;

#[path = "../🔬️render-equivalence/🦀️.rs"]
#[cfg(test)]
mod render_equivalence;

#[path = "../🔬️engine/🦀️.rs"]
#[cfg(test)]
mod engine_behaviour;

#[path = "../🔬️chrome/🦀️.rs"]
#[cfg(test)]
mod chrome_anatomy;

#[path = "../🔬️overlays/🦀️.rs"]
#[cfg(test)]
mod overlays;

#[path = "../🔬️pointer-routing/🦀️.rs"]
#[cfg(test)]
mod pointer_routing;
