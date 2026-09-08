//! @emoji 📎️ `dsl_family_embed` — embed/island family kit for host-language fences.

pub use crate::os_dsl::notation::{print_edge, EdgeLabel, EdgeLink, EdgeNode, EdgeValue};

/// @emoji 🏷️ Parses `lang` id from a fence header line (` ```jack `).
pub async fn parse_fence_lang(header: &str) -> Option<String> {
    let trimmed = header.trim();
    let rest = trimmed.strip_prefix("```")?;
    let lang = rest.split_whitespace().next()?;
    (!lang.is_empty()).then(|| lang.to_string())
}

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
