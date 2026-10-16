//! 🔀️ `shell-actions` staging of `panel-records`: RENDERER-WP4 steps S0 and T1, the part of the former chunk that plan Appendix B and C (re-homed by the call graph) assign to this crate.
//!
//! Members of tier-1 crates are methods of their component struct with the lower components as parameters; use-case members stay `impl ShellState` and address the components as `self.<component>.<field>`; a private item that crossed a module boundary was widened to `pub(super)`.

use super::*;

//#region ShellLifecycle
//#region 🧾️PanelRecordsAssembler
/// 📝️ The value one staged-arg dispatch stages: a segmented button's `option`, a colour part's colour, a vector axis
/// spliced into the tuple at `index` (the tuple already staged, else the `tuple` the row painted), else the editor's
/// own `value`.
pub fn staged_arg_value(args: &Value, current: Option<&Value>) -> Value {
    if let Some(option) = args.get("option") {
        return option.clone();
    }
    if let Some(color) = args.get("color") {
        return staged_color_value(color, args.get("value"), current);
    }
    if let (Some(index), Some(dims)) = (args.get("index").and_then(Value::as_u64), args.get("dims").and_then(Value::as_u64)) {
        let mut tuple: Vec<Value> = current.or_else(|| args.get("tuple")).and_then(Value::as_array).cloned().unwrap_or_default();
        tuple.resize(usize::try_from(dims).unwrap_or(0), Value::from(0.0));
        if let (Some(slot), Some(number)) = (usize::try_from(index).ok().and_then(|index| tuple.get_mut(index)), args.get("value").and_then(staged_number)) {
            *slot = Value::from(number);
        }
        return Value::Array(tuple);
    }
    args.get("value").cloned().unwrap_or(Value::Null)
}

//#endregion 🧾️PanelRecordsAssembler
//#endregion ShellLifecycle

