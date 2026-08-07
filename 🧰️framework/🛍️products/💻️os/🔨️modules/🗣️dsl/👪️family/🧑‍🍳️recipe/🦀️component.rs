//! @emoji 🧑‍🍳️ `dsl_family_recipe` — the recipe family notation kit, shared by `process3d`,
//! `playbook`, and `shome`: ordered typed-call steps, `name: target(args)` — e.g.
//! `step-1: state.set(counter 0)`. Deliberately NOT built on `crate::os_dsl::schema::RecordLayout::Call`:
//! that layout fixes both the separator (`=`) and the call target (`RecordSpec.keyword`) at spec-
//! declaration time, which fits a construction chain where every statement calls the same function
//! (`extrude = brep.solid.extrude(...)`) but not a recipe step, where each step's target varies
//! (`state.set`, `state.get`, `math.add`, ...) and the separator is `:`. Built directly on
//! `crate::os_dsl::core::lex`, matching `dsl_notation`/`dsl_family_graph`/`dsl_family_catalog`'s pattern.

use crate::os_dsl::core::{lex, Limits, TextError, TextSpan, TokenKind};

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
    tokens: Vec<crate::os_dsl::core::SpannedToken>,
    pos: usize,
}

impl Cursor {
    fn peek(&self) -> &crate::os_dsl::core::SpannedToken {
        &self.tokens[self.pos.min(self.tokens.len() - 1)]
    }

    fn advance(&mut self) -> crate::os_dsl::core::SpannedToken {
        let token = self.tokens[self.pos.min(self.tokens.len() - 1)].clone();
        if self.pos < self.tokens.len() - 1 {
            self.pos += 1;
        }
        token
    }

    fn expect(&mut self, kind: TokenKind) -> Result<crate::os_dsl::core::SpannedToken, TextError> {
        if self.peek().kind == kind {
            Ok(self.advance())
        } else {
            Err(TextError::new(format!("expected {kind:?}, found {:?}", self.peek().kind), self.peek().span))
        }
    }

    fn span(&self) -> TextSpan {
        self.peek().span
    }
}

fn arg_text(token: &crate::os_dsl::core::SpannedToken) -> Result<String, TextError> {
    match token.kind {
        TokenKind::Ident | TokenKind::Int | TokenKind::Float => Ok(token.text.as_str().to_string()),
        TokenKind::Text => Ok(format!("\"{}\"", crate::os_dsl::core::escape_text(&token.text.as_str()))),
        other => Err(TextError::new(format!("expected an argument, found {other:?}"), token.span)),
    }
}

/// @emoji 🔌️ Parses one standalone recipe step: `name: target(arg1 arg2)`. `target` may be a
/// dotted call path (`state.set`) since `.` is `dsl_core` ident-continue — it lexes as one `Ident`.
pub fn parse_step_text(text: &str) -> Result<RecipeStep, TextError> {
    let limits = Limits::default();
    // Keeps the `Eof` sentinel (only trivia is filtered) — `Cursor::advance`'s clamp-at-last-index
    // logic needs a real final token to land on and stay at once input is exhausted; dropping it
    // makes `peek()` re-return whatever the last REAL token was forever instead of signaling Eof.
    let tokens: Vec<_> = lex(text, &limits, false)?.into_iter().filter(|t| !t.kind.is_trivia()).collect();
    let mut cursor = Cursor { tokens, pos: 0 };

    let name = cursor.expect(TokenKind::Ident)?.text.as_str().to_string();
    cursor.expect(TokenKind::Colon)?;
    let target = cursor.expect(TokenKind::Ident)?.text.as_str().to_string();
    cursor.expect(TokenKind::LParen)?;
    let mut args = Vec::new();
    while cursor.peek().kind != TokenKind::RParen {
        args.push(arg_text(&cursor.advance())?);
    }
    cursor.expect(TokenKind::RParen)?;
    if cursor.peek().kind != TokenKind::Eof {
        return Err(TextError::new(format!("unexpected trailing {:?} after recipe step", cursor.peek().kind), cursor.span()));
    }
    Ok(RecipeStep { name, target, args })
}

/// @emoji 🖨️ Canonical printer — the inverse of [`parse_step_text`]: `name: target(arg1 arg2)`.
pub fn print_step(step: &RecipeStep) -> String {
    format!("{}: {}({})", step.name, step.target, step.args.join(" "))
}
//#endregion 🔖️Step

//#region 🔖️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_a_typed_call_step_with_a_dotted_target() {
        let step = parse_step_text("step-1: state.set(counter 0)").expect("parse_step_text");
        assert_eq!(step, RecipeStep { name: "step-1".to_string(), target: "state.set".to_string(), args: vec!["counter".to_string(), "0".to_string()] });
        assert_eq!(print_step(&step), "step-1: state.set(counter 0)");
    }

    #[test]
    fn parses_a_step_with_no_args() {
        let step = parse_step_text("step-2: state.reset()").expect("parse_step_text");
        assert_eq!(step.args, Vec::<String>::new());
        assert_eq!(print_step(&step), "step-2: state.reset()");
    }

    #[test]
    fn parses_a_step_with_a_text_argument() {
        let step = parse_step_text("step-3: log.write(\"hello world\")").expect("parse_step_text");
        assert_eq!(step.args, vec!["\"hello world\"".to_string()]);
    }

    #[test]
    fn rejects_a_missing_colon() {
        let err = parse_step_text("step-1 state.set(counter 0)").unwrap_err();
        assert!(err.message.contains("Colon"), "unexpected message: {}", err.message);
    }

    #[test]
    fn rejects_trailing_content() {
        assert!(parse_step_text("step-1: state.set(counter 0) extra").is_err());
    }

    #[test]
    fn round_trip_matrix() {
        let sources = vec!["step-1: state.set(counter 0)", "step-2: state.reset()", "step-3: math.add(1 2 3)"];
        for source in sources {
            let step = parse_step_text(source).unwrap_or_else(|e| panic!("parse of {source:?} failed: {e:?}"));
            let printed = print_step(&step);
            assert_eq!(printed, source);
            let reparsed = parse_step_text(&printed).unwrap_or_else(|e| panic!("reparse of {printed:?} failed: {e:?}"));
            assert_eq!(reparsed, step);
        }
    }

    /// @emoji 📖️ The fragment's `.grammar` file must at least parse under `dsl_grammar`'s parser.
    #[test]
    fn grammar_file_is_syntactically_valid() {
        let source = include_str!("📖️family-recipe.grammar.semio");
        let grammar = crate::os_dsl::grammar::parse_grammar(source).expect("family-recipe.grammar must parse");
        assert_eq!(grammar.id, "family-recipe");
        assert!(grammar.productions.len() > 4, "family-recipe should cover named and positional args");
    }
}
//#endregion 🔖️Tests
