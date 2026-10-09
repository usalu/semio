//! 🌳️ The closed expression tree, its literal units, operators and the values it evaluates to.
//!
//! Literals keep the unit they were authored in (`90 mm` stays `Length(90, Millimetre)`) so printing is lossless; every [`Value`] is
//! in SI base units: metres, radians, square metres and cubic metres. The JSON contract of the tree is `🧬️schema/🔣️.json`.

use crate::kinds::Kind;

/// 📏️ A length unit of a literal; the SI value is `value * numerator / denominator` metres.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LengthUnit {
    Millimetre,
    Centimetre,
    Metre,
    Kilometre,
    Inch,
    Foot,
}

/// 📐️ An angle unit of a literal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AngleUnit {
    Degree,
    Radian,
}

/// ⬜️ An area unit of a literal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AreaUnit {
    SquareMillimetre,
    SquareCentimetre,
    SquareMetre,
}

/// 🧊️ A volume unit of a literal.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum VolumeUnit {
    CubicMillimetre,
    CubicCentimetre,
    CubicMetre,
    Litre,
}

/// 🔢️ Reads a unit table: the canonical symbol, the accepted aliases and the exact `(numerator, denominator)` ratio to the SI base unit.
macro_rules! unit_table {
    ($unit:ident, $si:literal, [$(($variant:ident, $symbol:literal, [$($alias:literal),*], $num:expr, $den:expr)),+ $(,)?]) => {
        impl $unit {
            /// 📋️ Every unit in canonical order.
            pub const ALL: &'static [$unit] = &[$($unit::$variant),+];

            /// 🏷️ The canonical symbol the printer writes.
            pub fn symbol(self) -> &'static str {
                match self {
                    $($unit::$variant => $symbol),+
                }
            }

            /// 🔎️ Resolves a canonical symbol or an accepted alias.
            pub fn from_symbol(text: &str) -> Option<Self> {
                match text {
                    $($symbol $(| $alias)* => Some($unit::$variant),)+
                    _ => None,
                }
            }

            /// 🔁️ Converts a literal to the SI base unit.
            pub fn to_si(self, value: f64) -> f64 {
                let (num, den): (f64, f64) = match self {
                    $($unit::$variant => ($num, $den)),+
                };
                value * num / den
            }

            /// 🧭️ The SI base unit this table converts to.
            pub const SI: &'static str = $si;
        }
    };
}

unit_table!(LengthUnit, "m", [
    (Millimetre, "mm", [], 1.0, 1000.0),
    (Centimetre, "cm", [], 1.0, 100.0),
    (Metre, "m", [], 1.0, 1.0),
    (Kilometre, "km", [], 1000.0, 1.0),
    (Inch, "in", [], 254.0, 10000.0),
    (Foot, "ft", [], 3048.0, 10000.0),
]);

unit_table!(AreaUnit, "m2", [
    (SquareMillimetre, "mm2", ["mm²"], 1.0, 1_000_000.0),
    (SquareCentimetre, "cm2", ["cm²"], 1.0, 10_000.0),
    (SquareMetre, "m2", ["m²"], 1.0, 1.0),
]);

unit_table!(VolumeUnit, "m3", [
    (CubicMillimetre, "mm3", ["mm³"], 1.0, 1_000_000_000.0),
    (CubicCentimetre, "cm3", ["cm³"], 1.0, 1_000_000.0),
    (CubicMetre, "m3", ["m³"], 1.0, 1.0),
    (Litre, "l", [], 1.0, 1000.0),
]);

impl AngleUnit {
    /// 📋️ Every unit in canonical order.
    pub const ALL: &'static [AngleUnit] = &[AngleUnit::Degree, AngleUnit::Radian];

    /// 🧭️ The SI base unit values are stored in.
    pub const SI: &'static str = "rad";

    /// 🏷️ The canonical symbol the printer writes.
    pub fn symbol(self) -> &'static str {
        match self {
            AngleUnit::Degree => "deg",
            AngleUnit::Radian => "rad",
        }
    }

    /// 🔎️ Resolves a canonical symbol or an accepted alias.
    pub fn from_symbol(text: &str) -> Option<Self> {
        match text {
            "deg" | "°" => Some(AngleUnit::Degree),
            "rad" => Some(AngleUnit::Radian),
            _ => None,
        }
    }

    /// 🔁️ Converts a literal to radians.
    pub fn to_si(self, value: f64) -> f64 {
        match self {
            AngleUnit::Degree => value * std::f64::consts::PI / 180.0,
            AngleUnit::Radian => value,
        }
    }
}

/// ➖️ A prefix operator: arithmetic negation or boolean not.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UnaryOp {
    Negate,
    Not,
}

/// ➕️ An arithmetic infix operator or the two-argument `min` and `max`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Min,
    Max,
}

/// ⚖️ A comparison; equality is tolerant, see [`crate::evaluation`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum CompareOp {
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

/// 🧰️ A built-in function.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Function {
    Sqrt,
    Abs,
    Round,
    Floor,
    Ceil,
    Sin,
    Cos,
    Tan,
    Atan2,
}

impl UnaryOp {
    /// 🏷️ The canonical symbol the printer writes.
    pub fn symbol(self) -> &'static str {
        match self {
            UnaryOp::Negate => "-",
            UnaryOp::Not => "not",
        }
    }
}

impl BinaryOp {
    /// 📋️ Every operator in canonical order.
    pub const ALL: &'static [BinaryOp] = &[BinaryOp::Add, BinaryOp::Subtract, BinaryOp::Multiply, BinaryOp::Divide, BinaryOp::Power, BinaryOp::Min, BinaryOp::Max];

    /// 🏷️ The canonical symbol the printer writes; `min` and `max` are written as calls.
    pub fn symbol(self) -> &'static str {
        match self {
            BinaryOp::Add => "+",
            BinaryOp::Subtract => "-",
            BinaryOp::Multiply => "*",
            BinaryOp::Divide => "/",
            BinaryOp::Power => "^",
            BinaryOp::Min => "min",
            BinaryOp::Max => "max",
        }
    }
}

impl CompareOp {
    /// 📋️ Every comparison in canonical order.
    pub const ALL: &'static [CompareOp] = &[CompareOp::Equal, CompareOp::NotEqual, CompareOp::Less, CompareOp::LessEqual, CompareOp::Greater, CompareOp::GreaterEqual];

    /// 🏷️ The canonical symbol the printer writes.
    pub fn symbol(self) -> &'static str {
        match self {
            CompareOp::Equal => "=",
            CompareOp::NotEqual => "!=",
            CompareOp::Less => "<",
            CompareOp::LessEqual => "<=",
            CompareOp::Greater => ">",
            CompareOp::GreaterEqual => ">=",
        }
    }

    /// 🟰️ Whether the comparison is an equality test rather than an ordering.
    pub fn is_equality(self) -> bool {
        matches!(self, CompareOp::Equal | CompareOp::NotEqual)
    }
}

impl Function {
    /// 📋️ Every function in canonical order.
    pub const ALL: &'static [Function] = &[Function::Sqrt, Function::Abs, Function::Round, Function::Floor, Function::Ceil, Function::Sin, Function::Cos, Function::Tan, Function::Atan2];

    /// 🏷️ The name the printer writes and the parser reads.
    pub fn name(self) -> &'static str {
        match self {
            Function::Sqrt => "sqrt",
            Function::Abs => "abs",
            Function::Round => "round",
            Function::Floor => "floor",
            Function::Ceil => "ceil",
            Function::Sin => "sin",
            Function::Cos => "cos",
            Function::Tan => "tan",
            Function::Atan2 => "atan2",
        }
    }

    /// 🔎️ Resolves a function by name.
    pub fn from_name(name: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|f| f.name() == name)
    }

    /// 🔢️ The exact argument count.
    pub fn arity(self) -> usize {
        match self {
            Function::Atan2 => 2,
            _ => 1,
        }
    }
}

/// 🌳️ A typed expression; the tree is closed, immutable data with no evaluation state.
#[derive(Clone, Debug, PartialEq)]
pub enum Expr {
    Number(f64),
    Length(f64, LengthUnit),
    Angle(f64, AngleUnit),
    Area(f64, AreaUnit),
    Volume(f64, VolumeUnit),
    Bool(bool),
    Text(String),
    Param(String),
    Unary(UnaryOp, Box<Expr>),
    Binary(BinaryOp, Box<Expr>, Box<Expr>),
    Compare(CompareOp, Box<Expr>, Box<Expr>),
    And(Box<Expr>, Box<Expr>),
    Or(Box<Expr>, Box<Expr>),
    If(Box<Expr>, Box<Expr>, Box<Expr>),
    Call(Function, Vec<Expr>),
}

impl Expr {
    /// 🔤️ A parameter reference.
    pub fn param(name: &str) -> Expr {
        Expr::Param(name.to_string())
    }

    /// 🧩️ An operator node over two operands.
    pub fn binary(op: BinaryOp, left: Expr, right: Expr) -> Expr {
        Expr::Binary(op, Box::new(left), Box::new(right))
    }

    /// 🧩️ A comparison node over two operands.
    pub fn compare(op: CompareOp, left: Expr, right: Expr) -> Expr {
        Expr::Compare(op, Box::new(left), Box::new(right))
    }

    /// 🧩️ A prefix operator node.
    pub fn unary(op: UnaryOp, operand: Expr) -> Expr {
        Expr::Unary(op, Box::new(operand))
    }

    /// 🔀️ A conditional node.
    pub fn conditional(condition: Expr, then: Expr, otherwise: Expr) -> Expr {
        Expr::If(Box::new(condition), Box::new(then), Box::new(otherwise))
    }

    /// 👶️ The direct children in path order: the operands, the condition then both branches, or the call arguments.
    pub fn children(&self) -> Vec<&Expr> {
        match self {
            Expr::Unary(_, a) => vec![a],
            Expr::Binary(_, a, b) | Expr::Compare(_, a, b) | Expr::And(a, b) | Expr::Or(a, b) => vec![a, b],
            Expr::If(c, t, e) => vec![c, t, e],
            Expr::Call(_, args) => args.iter().collect(),
            _ => Vec::new(),
        }
    }

    /// 🧭️ The node reached by following child indices from this node.
    pub fn at(&self, path: &[usize]) -> Option<&Expr> {
        path.iter().try_fold(self, |node, &index| node.children().get(index).copied())
    }

    /// 🧮️ The number of nodes in the tree.
    pub fn size(&self) -> usize {
        1 + self.children().iter().map(|c| c.size()).sum::<usize>()
    }

    /// ✅️ Whether printing and parsing return this exact tree: every literal is finite and non-negative, every call has its arity
    /// and every parameter name is non-empty.
    pub fn is_canonical(&self) -> bool {
        let literal_ok = |v: f64| v.is_finite() && v.is_sign_positive();
        let own = match self {
            Expr::Number(v) | Expr::Length(v, _) | Expr::Angle(v, _) | Expr::Area(v, _) | Expr::Volume(v, _) => literal_ok(*v),
            Expr::Call(f, args) => args.len() == f.arity(),
            Expr::Param(name) => !name.is_empty(),
            _ => true,
        };
        own && self.children().iter().all(|c| c.is_canonical())
    }
}

/// 🎁️ A computed value in SI base units: metres, radians, square metres, cubic metres.
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Length(f64),
    Angle(f64),
    Area(f64),
    Volume(f64),
    Bool(bool),
    Text(String),
}

impl Value {
    /// 🏷️ The kind of the value.
    pub fn kind(&self) -> Kind {
        match self {
            Value::Number(_) => Kind::Number,
            Value::Length(_) => Kind::Length,
            Value::Angle(_) => Kind::Angle,
            Value::Area(_) => Kind::Area,
            Value::Volume(_) => Kind::Volume,
            Value::Bool(_) => Kind::Bool,
            Value::Text(_) => Kind::Text,
        }
    }

    /// 🔢️ The SI magnitude of a numeric value.
    pub fn magnitude(&self) -> Option<f64> {
        match self {
            Value::Number(v) | Value::Length(v) | Value::Angle(v) | Value::Area(v) | Value::Volume(v) => Some(*v),
            _ => None,
        }
    }

    /// 🏗️ A numeric value of a kind from its SI magnitude; `None` for `Bool` and `Text`.
    pub fn of_kind(kind: Kind, magnitude: f64) -> Option<Value> {
        match kind {
            Kind::Number => Some(Value::Number(magnitude)),
            Kind::Length => Some(Value::Length(magnitude)),
            Kind::Angle => Some(Value::Angle(magnitude)),
            Kind::Area => Some(Value::Area(magnitude)),
            Kind::Volume => Some(Value::Volume(magnitude)),
            Kind::Bool | Kind::Text => None,
        }
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
