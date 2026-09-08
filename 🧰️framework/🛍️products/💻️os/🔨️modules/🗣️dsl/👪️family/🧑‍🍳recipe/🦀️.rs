//! @emoji 🧑‍🍳️ `dsl_family_recipe` — the recipe family notation kit, shared by `process3d`,
//! `playbook`, and `shome`: ordered typed-call steps, `name: target(args)` — e.g.
//! `step-1: state.set(counter 0)`. Deliberately NOT built on `crate::os_dsl::schema::RecordLayout::Call`:
//! that layout fixes both the separator (`=`) and the call target (`RecordSpec.keyword`) at spec-
//! declaration time, which fits a construction chain where every statement calls the same function
//! (`extrude = brep.solid.extrude(...)`) but not a recipe step, where each step's target varies
//! (`state.set`, `state.get`, `math.add`, ...) and the separator is `:`. Built directly on
//! `crate::os_dsl::lex`, matching `dsl_notation`/`dsl_family_graph`/`dsl_family_catalog`'s pattern.

use crate::os_dsl::{lex, Limits, TextError, TextSpan, TokenKind};

//#region 🔖️Step
/// @emoji 🪜️ One recipe step: `name: target(arg1 arg2 ...)`. Arguments are positional only in
/// this v1 (bare `Ident`/`Int`/`Float`/`Text` tokens, printed as their own text) — `key=value`
/// keyed arguments are not yet supported (a real, documented gap, not silently dropped).
#[derive(Clone, Debug, PartialEq)]
pub struct RecipeStep {
    pub name: String,
    pub target: String,
    pub args: Vec<String>,
}

struct Cursor {
    tokens: Vec<crate::os_dsl::SpannedToken>,
    pos: usize,
}

impl Cursor {
    async fn peek(&self) -> &crate::os_dsl::SpannedToken {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    async fn advance(&mut self) -> crate::os_dsl::SpannedToken {
        let token = self.tokens[self.pos.min(self.tokens.len() - 1)].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    async fn expect(&mut self, kind: TokenKind) -> Result<crate::os_dsl::SpannedToken, TextError> {
        if self.peek().await.kind == kind {
            Ok(self.advance().await)
        } else {
            Err(TextError::new(format!("expected {kind:?}, found {:?}", self.peek().await.kind), self.peek().await.span))
        }
    }

    async fn span(&self) -> TextSpan {
        self.peek().await.span
    }
}

async fn arg_text(token: &crate::os_dsl::SpannedToken) -> Result<String, TextError> {
    match token.kind {
        TokenKind::Ident | TokenKind::Int | TokenKind::Float => Ok(token.text.as_str().to_string()),
        TokenKind::Text => Ok(format!("\"{}\"", crate::os_dsl::escape_text(&token.text.as_str()))),
        other => Err(TextError::new(format!("expected an argument, found {other:?}"), token.span)),
    }
}

/// @emoji 🔌️ Parses one standalone recipe step: `name: target(arg1 arg2)`. `target` may be a
/// dotted call path (`state.set`) since `.` is `dsl_core` ident-continue — it lexes as one `Ident`.
pub async fn parse_step_text(text: &str) -> Result<RecipeStep, TextError> {
    let limits = Limits::default();
    // Keeps the `Eof` sentinel (only trivia is filtered) — `Cursor::advance`'s clamp-at-last-index
    // logic needs a real final token to land on and stay at once input is exhausted; dropping it
    // makes `peek()` re-return whatever the last REAL token was forever instead of signaling Eof.
    let tokens: Vec<_> = lex(text, &limits, false)?.into_iter().filter(|t| !t.kind.is_trivia()).collect();
    let mut cursor = Cursor { tokens, pos: 0 };

    let name = cursor.expect(TokenKind::Ident).await?.text.as_str().to_string();
    cursor.expect(TokenKind::Colon).await?;
    let target = cursor.expect(TokenKind::Ident).await?.text.as_str().to_string();
    cursor.expect(TokenKind::LParen).await?;
    let mut args = Vec::new();
    while cursor.peek().await.kind != TokenKind::RParen {
        args.push(arg_text(&cursor.advance().await).await?);
    }
    cursor.expect(TokenKind::RParen).await?;
    if cursor.peek().await.kind != TokenKind::Eof {
        return Err(TextError::new(format!("unexpected trailing {:?} after recipe step", cursor.peek().await.kind), cursor.span().await));
    }
    Ok(RecipeStep { name, target, args })
}

/// @emoji 🖨️ Canonical printer — the inverse of [`parse_step_text`]: `name: target(arg1 arg2)`.
pub async fn print_step(step: &RecipeStep) -> String {
    format!("{}: {}({})", step.name, step.target, step.args.join(" "))
}
//#endregion 🔖️Step

//#region 🔖️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🔖️Tests
