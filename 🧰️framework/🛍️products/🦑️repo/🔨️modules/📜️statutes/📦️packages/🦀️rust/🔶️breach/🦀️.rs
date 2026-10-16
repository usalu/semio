//! 🔶️ Breach construction: the canonical breach identifier and the breach record.

use super::{Breach, Statute};

//#region 🔶️Breach

/// 🆔️ The canonical identifier of a breach at a scope and position.
pub fn build_breach_id(scope: &str, line: i64, col: i64) -> String {
    if line > 0 && col > 0 {
        format!("repo/breach/{scope}#{line}:{col}")
    } else if line > 0 {
        format!("repo/breach/{scope}#{line}")
    } else {
        format!("repo/breach/{scope}")
    }
}

/// 🔶️ A breach of a statute at a scope and position.
pub fn create_breach(summary: &str, kind: Statute, scope: &str, line: i64, col: i64, excerpt: &str) -> Breach {
    Breach {
        id: build_breach_id(scope, line, col),
        summary: summary.to_string(),
        kind,
        scope: scope.to_string(),
        line,
        column: col,
        excerpt: excerpt.to_string(),
        lint_priority: None,
        lint_autofixable: None,
        reason: String::new(),
        solution: String::new(),
    }
}

//#endregion 🔶️Breach
