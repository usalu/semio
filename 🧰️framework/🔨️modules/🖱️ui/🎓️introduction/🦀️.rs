/// 🎓️ Required caller-owned facts for offering an introduction in the current session.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IntroductionEligibility<'a> {
    pub app_id: &'a str,
    pub has_introduction: bool,
    pub tutorial_active: bool,
    pub suppressed: bool,
    pub replay_on_load: bool,
    pub seen_on_device: bool,
    pub dismissed_in_session: bool,
}

/// 🌱️ Offers an authored introduction when its host and session facts permit it.
pub fn should_start_introduction(input: &IntroductionEligibility<'_>) -> bool {
    !input.app_id.is_empty() && input.has_introduction && !input.tutorial_active && !input.suppressed && !input.dismissed_in_session && (input.replay_on_load || !input.seen_on_device)
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
