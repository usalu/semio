//! 🌳️ Immutable native counterpart of the canonical language-neutral EXPRESS program.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpressExpressionKind {
    Integer,
    Real,
    String,
    Logical,
    Unknown,
    Identifier,
    Unary,
    Binary,
    Attribute,
    Qualify,
    Call,
    Index,
    Slice,
    Aggregate,
    Query,
    Interval,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExpressExpression {
    pub kind: ExpressExpressionKind,
    pub offset: usize,
    pub text: Option<&'static str>,
    pub operator: Option<&'static str>,
    pub name: Option<&'static str>,
    pub operands: &'static [ExpressExpression],
    pub entries: &'static [ExpressAggregateEntry],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExpressAggregateEntry {
    pub value: &'static ExpressExpression,
    pub repeat: Option<&'static ExpressExpression>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExpressStatementKind {
    Assign,
    Return,
    If,
    Case,
    Repeat,
    Block,
    Escape,
    Skip,
    Call,
    Null,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExpressStatement {
    pub kind: ExpressStatementKind,
    pub offset: usize,
    pub name: Option<&'static str>,
    pub expressions: &'static [ExpressExpression],
    pub body: &'static [ExpressStatement],
    pub otherwise: &'static [ExpressStatement],
    pub cases: &'static [ExpressCase],
    pub while_condition: Option<&'static ExpressExpression>,
    pub until_condition: Option<&'static ExpressExpression>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExpressCase {
    pub labels: &'static [ExpressExpression],
    pub body: &'static [ExpressStatement],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExpressVariable {
    pub names: &'static [&'static str],
    pub type_source: &'static str,
    pub initial: Option<&'static ExpressExpression>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExpressFunction {
    pub name: &'static str,
    pub parameters: &'static [ExpressVariable],
    pub result_type: &'static str,
    pub locals: &'static [ExpressVariable],
    pub body: &'static [ExpressStatement],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExpressRule {
    pub owner: &'static str,
    pub label: &'static str,
    pub expression: &'static ExpressExpression,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExpressGlobalRule {
    pub name: &'static str,
    pub populations: &'static [&'static str],
    pub locals: &'static [ExpressVariable],
    pub body: &'static [ExpressStatement],
    pub rules: &'static [ExpressRule],
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExpressInverse {
    pub owner: &'static str,
    pub name: &'static str,
    pub entity_type: &'static str,
    pub attribute: &'static str,
    pub aggregate: bool,
    pub lower: Option<&'static ExpressExpression>,
    pub upper: Option<&'static ExpressExpression>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ExpressProgram {
    pub schema: &'static str,
    pub functions: &'static [ExpressFunction],
    pub global_rules: &'static [ExpressGlobalRule],
    pub rules: &'static [ExpressRule],
    pub derived: &'static [ExpressRule],
    pub unique: &'static [ExpressRule],
    pub bounds: &'static [ExpressRule],
    pub inverse: &'static [ExpressInverse],
}
