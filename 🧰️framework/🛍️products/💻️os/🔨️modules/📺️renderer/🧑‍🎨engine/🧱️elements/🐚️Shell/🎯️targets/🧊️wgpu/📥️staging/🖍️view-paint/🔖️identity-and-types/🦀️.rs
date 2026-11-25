//! 🖍 `shell-view-paint` staging of `identity-and-types`: RENDERER-WP5, the part of the former chunk that plan Appendix B and C, the WP4 re-homing by the call graph and the WP5 size spill assign to this crate.

use super::*;

//#region 🔖️IdentityPure
/// 👤️ The name a peer's roster shows for this shell — the twin of React's `presenceClientIdentity`
/// (`🛠️ShellHelpers/🟦️.tsx`): the signed-in person's display name, else a guest named after the actor's tail.
pub fn shell_presence_label(identity: Option<&Identity>, actor: &str) -> String {
    match identity.map(|identity| identity.display_name.trim()).filter(|name| !name.is_empty()) {
        Some(name) => name.to_string(),
        None => format!("Guest {}", actor.chars().rev().take(4).collect::<Vec<_>>().into_iter().rev().collect::<String>().to_uppercase()),
    }
}
//#endregion 🔖️IdentityPure
