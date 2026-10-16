//! 🧮️ Jack parser: tokens to the query abstract syntax tree.

use super::*;

// #region 🔖️Parser
struct Parser {
    pub(crate) tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    pub(crate) fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn bump(&mut self) -> Token {
        let t = self.peek().clone();
        if !matches!(t, Token::Eof) {
            self.pos += 1;
        }
        t
    }

    fn expect_ident(&mut self) -> Result<String, GraphDslError> {
        match self.bump() {
            Token::Ident(s) => Ok(s),
            other => Err(GraphDslError::UnexpectedToken { expected: "ident".into(), found: format!("{other:?}") }),
        }
    }

    fn parse_query(&mut self) -> Result<Query, GraphDslError> {
        let mut clauses = Vec::new();
        while !matches!(self.peek(), Token::Eof) {
            clauses.push(self.parse_clause()?);
        }
        Ok(Query { clauses })
    }

    fn parse_clause(&mut self) -> Result<Clause, GraphDslError> {
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
            Token::KwWith => {
                self.bump();
                let mut items = vec![self.parse_return_item()?];
                while matches!(self.peek(), Token::Comma) {
                    self.bump();
                    items.push(self.parse_return_item()?);
                }
                Ok(Clause::With(items))
            }
            Token::KwUnwind => {
                self.bump();
                let source = self.parse_return_item()?;
                self.expect(&Token::KwAs)?;
                let var = self.expect_ident()?;
                Ok(Clause::Unwind(UnwindClause { source, var }))
            }
            Token::KwCall => {
                self.bump();
                let name = self.expect_ident()?;
                self.expect(&Token::LParen)?;
                let mut args = Vec::new();
                if !matches!(self.peek(), Token::RParen) {
                    args.push(self.parse_value()?);
                    while matches!(self.peek(), Token::Comma) {
                        self.bump();
                        args.push(self.parse_value()?);
                    }
                }
                self.expect(&Token::RParen)?;
                Ok(Clause::Call(CallClause { name, args }))
            }
            other => Err(GraphDslError::UnexpectedToken { expected: "clause start (MATCH/WHERE/RETURN/CREATE/DELETE/SET/MERGE/WITH/UNWIND/CALL)".into(), found: format!("{other:?}") }),
        }
    }

    /// 🕸️ Pattern grammar over the unified token alphabet — `dsl_core` has no standalone `-`
    /// token (only `->`/`--`/`<-`), so the leading connector before a bracketed edge label is
    /// always `--` or `<-`, never a bare dash (real Cypher's `-[r]->`/`<-[r]-` shape, adapted to
    /// this repo's alphabet). `<-` at the front means the edge points INTO `left`; represented by
    /// swapping which parsed node plays "left" so the stored `PatternEdge.right` is always the
    /// forward-direction target, mirroring `dsl_schema`'s own wire `<-` normalization.
    fn parse_pattern(&mut self) -> Result<Pattern, GraphDslError> {
        self.expect(&Token::LParen)?;
        let left = self.parse_pattern_node()?;
        self.expect(&Token::RParen)?;
        match self.peek().clone() {
            Token::Arrow => {
                self.bump();
                let right = self.parse_bracketed_pattern_node()?;
                Ok(Pattern { nodes: vec![left], edge: Some(PatternEdge { var: None, kind: None, directed: true, right }) })
            }
            // ➖️ `-` and `--` are the SAME connector. Cypher (and this dialect's own executor parser,
            // and every committed example query in the repo) writes `(a)-[r:Kind]->(b)`; `--` is this
            // grammar's own equal-status spelling. Accepting only `--` made every real query fail
            // `unexpected character '-'` in lint/complete/hover/format while the executor ran it fine.
            Token::Dash | Token::DashArrow => {
                self.bump();
                if matches!(self.peek(), Token::LBracket) {
                    let (edge_var, edge_kind) = self.parse_edge_label()?;
                    let directed = match self.peek() {
                        Token::Arrow => {
                            self.bump();
                            true
                        }
                        Token::Dash | Token::DashArrow => {
                            self.bump();
                            false
                        }
                        other => return Err(GraphDslError::UnexpectedToken { expected: "->, - or --".into(), found: format!("{other:?}") }),
                    };
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![left], edge: Some(PatternEdge { var: edge_var, kind: edge_kind, directed, right }) })
                } else {
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![left], edge: Some(PatternEdge { var: None, kind: None, directed: false, right }) })
                }
            }
            Token::BackArrow => {
                self.bump();
                if matches!(self.peek(), Token::LBracket) {
                    let (edge_var, edge_kind) = self.parse_edge_label()?;
                    match self.peek() {
                        Token::Dash | Token::DashArrow => {
                            self.bump();
                        }
                        other => return Err(GraphDslError::UnexpectedToken { expected: "- or --".into(), found: format!("{other:?}") }),
                    }
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![right], edge: Some(PatternEdge { var: edge_var, kind: edge_kind, directed: true, right: left }) })
                } else {
                    let right = self.parse_bracketed_pattern_node()?;
                    Ok(Pattern { nodes: vec![right], edge: Some(PatternEdge { var: None, kind: None, directed: true, right: left }) })
                }
            }
            _ => Ok(Pattern { nodes: vec![left], edge: None }),
        }
    }

    fn parse_bracketed_pattern_node(&mut self) -> Result<PatternNode, GraphDslError> {
        self.expect(&Token::LParen)?;
        let node = self.parse_pattern_node()?;
        self.expect(&Token::RParen)?;
        Ok(node)
    }

    fn parse_edge_label(&mut self) -> Result<(Option<String>, Option<String>), GraphDslError> {
        self.expect(&Token::LBracket)?;
        let edge_var = if matches!(self.peek(), Token::Ident(_)) { Some(self.expect_ident()?) } else { None };
        let edge_kind = if matches!(self.peek(), Token::Colon) {
            self.bump();
            Some(self.expect_ident()?)
        } else {
            None
        };
        self.expect(&Token::RBracket)?;
        Ok((edge_var, edge_kind))
    }

    fn parse_pattern_node(&mut self) -> Result<PatternNode, GraphDslError> {
        let var = self.expect_ident()?;
        self.expect(&Token::Colon)?;
        let kind = self.expect_ident()?;
        let port = if matches!(self.peek(), Token::At) {
            self.bump();
            Some(self.expect_ident()?)
        } else {
            None
        };
        Ok(PatternNode { var, kind, port })
    }

    fn parse_return_item(&mut self) -> Result<ReturnItem, GraphDslError> {
        let var = self.expect_ident()?;
        if matches!(self.peek(), Token::Dot) {
            self.bump();
            let prop = self.expect_ident()?;
            Ok(ReturnItem::Property { var, prop })
        } else {
            Ok(ReturnItem::Var(var))
        }
    }

    fn parse_assignment(&mut self) -> Result<Assignment, GraphDslError> {
        let var = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let prop = self.expect_ident()?;
        self.expect(&Token::Eq)?;
        let value = self.parse_value()?;
        Ok(Assignment { var, prop, value })
    }

    fn parse_expr(&mut self) -> Result<Expr, GraphDslError> {
        self.parse_or_expr()
    }

    fn parse_or_expr(&mut self) -> Result<Expr, GraphDslError> {
        let mut left = self.parse_and_expr()?;
        while matches!(self.peek(), Token::Or) {
            self.bump();
            let right = self.parse_and_expr()?;
            left = Expr::Or(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_and_expr(&mut self) -> Result<Expr, GraphDslError> {
        let mut left = self.parse_cmp_expr()?;
        while matches!(self.peek(), Token::And) {
            self.bump();
            let right = self.parse_cmp_expr()?;
            left = Expr::And(Box::new(left), Box::new(right));
        }
        Ok(left)
    }

    fn parse_cmp_expr(&mut self) -> Result<Expr, GraphDslError> {
        let var = self.expect_ident()?;
        self.expect(&Token::Dot)?;
        let prop = self.expect_ident()?;
        match self.bump() {
            Token::Eq => Ok(Expr::Eq { var, prop, value: self.parse_value()? }),
            Token::Ne => Ok(Expr::Ne { var, prop, value: self.parse_value()? }),
            other => Err(GraphDslError::UnexpectedToken { expected: "= or !=".into(), found: format!("{other:?}") }),
        }
    }

    fn parse_value(&mut self) -> Result<PropertyValue, GraphDslError> {
        match self.bump() {
            Token::Number(n) => Ok(PropertyValue::Number(n)),
            Token::StringLit(s) => Ok(PropertyValue::String(s)),
            Token::Ident(s) if s.eq_ignore_ascii_case("true") => Ok(PropertyValue::Bool(true)),
            Token::Ident(s) if s.eq_ignore_ascii_case("false") => Ok(PropertyValue::Bool(false)),
            Token::Ident(s) if s.eq_ignore_ascii_case("null") => Ok(PropertyValue::Null),
            other => Err(GraphDslError::UnexpectedToken { expected: "value".into(), found: format!("{other:?}") }),
        }
    }

    pub(crate) fn expect(&mut self, want: &Token) -> Result<(), GraphDslError> {
        if std::mem::discriminant(self.peek()) == std::mem::discriminant(want) {
            self.bump();
            Ok(())
        } else {
            Err(GraphDslError::UnexpectedToken { expected: format!("{want:?}"), found: format!("{:?}", self.peek()) })
        }
    }
}

/// 🔍️ Parse a jack query string.
pub fn parse(query: &str) -> Result<Query, GraphDslError> {
    let tokens = lex(query)?;
    Parser::new(tokens).parse_query()
}
// #endregion 🔖️Parser
