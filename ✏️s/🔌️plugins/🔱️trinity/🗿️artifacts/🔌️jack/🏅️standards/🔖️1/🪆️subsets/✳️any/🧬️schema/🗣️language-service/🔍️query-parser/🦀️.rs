//! 🔍️ Jack query parser: the recursive descent `Parser` of the semantic `Query` AST.

use super::*;

// #endregion 🔖️ExampleFixture
struct Parser {
    pub(super) tokens: Vec<Token>,
    pub(super) pos: usize,
}

impl Parser {
    pub(super) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    pub(super) fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    pub(super) fn bump(&mut self) -> Token {
        let t = self.peek().clone();
        if !matches!(t, Token::Eof) {
            self.pos += 1;
        }
        t
    }

    pub(super) fn expect_ident(&mut self) -> Result<String, String> {
        match self.bump() {
            Token::Ident(s) => Ok(s),
            other => Err(format!("expected ident, got {other:?}")),
        }
    }

    pub(super) fn parse_query(&mut self) -> Result<Query, String> {
        let mut clauses = Vec::new();
        while !matches!(self.peek(), Token::Eof) {
            clauses.push(self.parse_clause()?);
        }
        Ok(Query { clauses })
    }

    pub(super) fn parse_clause(&mut self) -> Result<Clause, String> {
        match self.peek() {
            Token::KwMatch => {
                self.bump();
                let mut patterns = vec![self.parse_pattern()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    patterns.push(self.parse_pattern()?);
                }
                Ok(Clause::Match(patterns))
            }
            Token::KwWhere => {
                self.bump();
                Ok(Clause::Where(self.parse_expr()?))
            }
            Token::KwReturn => {
                self.bump();
                let mut items = vec![self.parse_return_item()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_return_item()?);
                }
                Ok(Clause::Return(items))
            }
            Token::KwCreate => {
                self.bump();
                Ok(Clause::Create(self.parse_pattern()?))
            }
            Token::KwDelete => {
                self.bump();
                let mut vars = vec![self.expect_ident()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    vars.push(self.expect_ident()?);
                }
                Ok(Clause::Delete(vars))
            }
            Token::KwSet => {
                self.bump();
                let mut items = vec![self.parse_assignment()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_assignment()?);
                }
                Ok(Clause::Set(items))
            }
            Token::KwMerge => {
                self.bump();
                Ok(Clause::Merge(self.parse_pattern()?))
            }
            other => Err(format!("unexpected clause start {other:?}")),
        }
    }

    pub(super) fn parse_pattern(&mut self) -> Result<Pattern, String> {
        self.expect(&Token::LParen)?;
        let left = self.parse_pattern_node()?;
        self.expect(&Token::RParen)?;
        if matches!(self.peek(), Token::Dash) {
            self.bump();
            self.expect(&Token::LBracket)?;
            let edge_var = if matches!(self.peek(), Token::Ident(_)) { Some(self.expect_ident()?) } else { None };
            let edge_kind = if matches!(self.peek(), Token::Colon) {
                self.bump();
                Some(self.expect_ident()?)
            } else {
                None
            };
            self.expect(&Token::RBracket)?;
            self.expect(&Token::Arrow)?;
            self.expect(&Token::LParen)?;
            let right = self.parse_pattern_node()?;
            self.expect(&Token::RParen)?;
            Ok(Pattern { nodes: vec![left], edge: Some(PatternEdge { var: edge_var, kind: edge_kind, directed: true, right }) })
        } else {
            Ok(Pattern { nodes: vec![left], edge: None })
        }
    }

    pub(super) fn parse_pattern_node(&mut self) -> Result<PatternNode, String> {
        let var = self.expect_ident()?;
        self.expect(&Token::Colon)?;
        let kind = self.expect_ident()?;
        Ok(PatternNode { var, kind })
    }

    pub(super) fn parse_return_item(&mut self) -> Result<ReturnItem, String> {
        let var = self.expect_ident()?;
        if matches!(self.peek(), Token::Dot) {
            self.bump();
            let prop = self.expect_ident()?;
            Ok(ReturnItem::Property { var, prop })
        } else {
            Ok(ReturnItem::Var(var))
        }
    }

    pub(super) fn parse_assignment(&mut self) -> Result<Assignment, String> {
        let var = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let prop = self.expect_ident()?;
        self.expect(&Token::Eq)?;
        let value = self.parse_value()?;
        Ok(Assignment { var, prop, value })
    }

    pub(super) fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_or_expr()
    }

    pub(super) fn parse_or_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_and_expr()?;
        while matches!(self.peek(), Token::Or) {
            self.bump();
            let right = self.parse_and_expr()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    pub(super) fn parse_and_expr(&mut self) -> Result<Expr, String> {
        let mut left = self.parse_cmp_expr()?;
        while matches!(self.peek(), Token::And) {
            self.bump();
            let right = self.parse_cmp_expr()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    pub(super) fn parse_cmp_expr(&mut self) -> Result<Expr, String> {
        let var = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let prop = self.expect_ident()?;
        match self.bump() {
            Token::Eq => Ok(Expr::Eq { var, prop, value: self.parse_value()? }),
            Token::Ne => Ok(Expr::Ne { var, prop, value: self.parse_value()? }),
            other => Err(format!("expected = or !=, got {other:?}")),
        }
    }

    pub(super) fn parse_value(&mut self) -> Result<PropertyValue, String> {
        match self.bump() {
            Token::Number(n) => Ok(PropertyValue::Number(n)),
            Token::StringLit(s) => Ok(PropertyValue::String(s)),
            Token::Ident(s) if s.eq_ignore_ascii_case("true") => Ok(PropertyValue::Bool(true)),
            Token::Ident(s) if s.eq_ignore_ascii_case("false") => Ok(PropertyValue::Bool(false)),
            Token::Ident(s) if s.eq_ignore_ascii_case("null") => Ok(PropertyValue::Null),
            other => Err(format!("expected value, got {other:?}")),
        }
    }

    pub(super) fn expect(&mut self, want: &Token) -> Result<(), String> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(want) {
            self.bump();
            Ok(())
        } else {
            Err(format!("expected {want:?}, got {:?}", self.peek()))
        }
    }
}

/// 🔍️ Parse a jack query string.
pub fn parse(query: &str) -> Result<Query, String> {
    let tokens = lex(query)?;
    Parser::new(tokens).parse_query()
}
