//! 🖐️ The audience rule of the gumball verbs the OS interaction hosts implement (the world, canvas and board gumballs),
//! enforced on every app the builder assembles through [`RESERVED`]: the handle toggle is window chrome. A gumball
//! gesture is one self-contained pose delta (a one-shot tool transaction, or the `phase`-streamed ticks of one), so no
//! bracket verb exists to classify.
use semio_framework::manifest::{CapabilityAudience, ReservedContribution};

/// 🎛️ The gumball's handle toggle — which move/rotate/scale handles one window shows.
pub(crate) const GUMBALL_CHROME_ACTION_IDS: [&str; 1] = ["setTransformGumballFlag"];

/// 🧭️ The one classification rule for the gumball verbs, stated once for every app that uses the gumball (lowpoly,
/// puzzle, fem, block, …): the handle toggle is [`CapabilityAudience::Chrome`], every other id is untouched. Held
/// against `🧪️tests/🧫️fixtures/🖐️gumball-verb-audience.json`.
pub(crate) fn gumball_fixed_audience(action_id: &str) -> Option<CapabilityAudience> {
    GUMBALL_CHROME_ACTION_IDS.contains(&action_id).then_some(CapabilityAudience::Chrome)
}

/// 🧩️ The gumball contribution: it reserves no action and fixes the audience of the handle toggle.
pub(crate) const RESERVED: ReservedContribution = ReservedContribution::new("gumball").with_fixed_audience(gumball_fixed_audience);

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
