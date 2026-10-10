//! 📝️ Text operation representation contracts.

//#region 🔖️OpText
/// ⚡️ Text I/O for semantic operations. Printing yields one line; parsing that line recovers the operation.
pub trait OpText: Sized {
    fn print_op(&self) -> String;
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError>;
}
//#endregion 🔖️OpText

/// 🔺️ Text I/O for semantic diffs.
pub trait DiffText: Sized {
    fn print_diff(&self) -> String;
    fn parse_diff(line: &str) -> Result<Self, semio_framework_diagnostic::TextError>;
}

#[path = "🧵️canonical/🦀️.rs"]
pub mod canonical;

#[path="🔗️causal/🦀️.rs"]
mod causal;
