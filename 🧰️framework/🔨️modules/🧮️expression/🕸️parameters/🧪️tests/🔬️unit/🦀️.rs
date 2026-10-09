use super::*;
use crate::syntax::parse;

fn set(rows: &[(&str, &str)]) -> BTreeMap<String, Expr> {
    rows.iter().map(|(name, src)| (name.to_string(), parse(src).unwrap())).collect()
}

fn names(rows: &[&str]) -> Vec<String> {
    rows.iter().map(|s| s.to_string()).collect()
}

fn graph(rows: &[(&str, &[&str])]) -> BTreeMap<String, BTreeSet<String>> {
    rows.iter().map(|(n, d)| (n.to_string(), d.iter().map(|s| s.to_string()).collect())).collect()
}

#[test]
fn dependencies_include_untaken_branches_and_deduplicate() {
    let e = parse("if a > 0 then b + b else c * sqrt(d)").unwrap();
    assert_eq!(dependencies(&e), ["a", "b", "c", "d"].into_iter().map(String::from).collect());
    assert!(dependencies(&parse("1 + 2 m").unwrap()).is_empty());
    assert_eq!(dependencies(&parse("`frame width` * 2").unwrap()), ["frame width"].into_iter().map(String::from).collect());
}

#[test]
fn the_plan_is_layered_and_sorted_by_name() {
    let p = plan(&graph(&[("d", &["b", "c"]), ("c", &["a"]), ("b", &["a"]), ("a", &[]), ("z", &[]), ("e", &["d"])]));
    assert_eq!(p.order, names(&["a", "z", "b", "c", "d", "e"]));
    assert!(p.cycles.is_empty() && p.blocked.is_empty());
}

#[test]
fn dependencies_on_absent_names_do_not_block() {
    let p = plan(&graph(&[("a", &["ghost"]), ("b", &["a"])]));
    assert_eq!(p.order, names(&["a", "b"]));
}

#[test]
fn cycles_are_reported_as_sorted_components_with_their_blocked_dependants() {
    let p = plan(&graph(&[("a", &["b"]), ("b", &["c"]), ("c", &["a"]), ("d", &["c"]), ("e", &["e"]), ("f", &[]), ("g", &["d"])]));
    assert_eq!(p.order, names(&["f"]));
    assert_eq!(p.cycles, vec![names(&["a", "b", "c"]), names(&["e"])]);
    assert_eq!(p.blocked, names(&["d", "g"]));
}

#[test]
fn a_chain_into_two_overlapping_cycles_forms_one_component() {
    let p = plan(&graph(&[("a", &["b"]), ("b", &["a", "c"]), ("c", &["b"])]));
    assert_eq!(p.cycles, vec![names(&["a", "b", "c"])]);
}

#[test]
fn values_resolve_in_dependency_order() {
    let params = set(&[("width", "2.4 m"), ("height", "width / 2"), ("area", "width * height"), ("half", "area / 2"), ("ok", "area > 2 m2")]);
    let r = evaluate_all(&params, &BTreeMap::new());
    assert!(r.errors.is_empty());
    assert_eq!(r.order, names(&["width", "height", "area", "half", "ok"]));
    assert_eq!(r.values["height"], Value::Length(1.2));
    assert!(matches!(r.values["area"], Value::Area(v) if (v - 2.88).abs() < 1e-12));
    assert_eq!(r.values["ok"], Value::Bool(true));
}

#[test]
fn overrides_replace_formulas_and_flow_to_dependants() {
    let params = set(&[("width", "2 m"), ("double", "width * 2")]);
    let overrides = set(&[("width", "3 m")]);
    let r = evaluate_all(&params, &overrides);
    assert_eq!(r.values["double"], Value::Length(6.0));
    assert_eq!(evaluate_all(&params, &BTreeMap::new()).values["double"], Value::Length(4.0));
}

#[test]
fn an_override_of_a_missing_parameter_is_an_error_and_changes_nothing() {
    let params = set(&[("a", "1")]);
    let r = evaluate_all(&params, &set(&[("b", "2")]));
    assert_eq!(r.errors["b"].kind, ErrorKind::UnknownOverride);
    assert_eq!(r.values.len(), 1);
    assert_eq!(r.order, names(&["a"]));
}

#[test]
fn an_override_of_a_different_kind_surfaces_in_the_dependants() {
    let params = set(&[("width", "2 m"), ("double", "width * 2")]);
    let r = evaluate_all(&params, &set(&[("width", "30 deg")]));
    assert_eq!(r.values["double"], Value::Angle(std::f64::consts::PI / 3.0));
    let r = evaluate_all(&set(&[("width", "2 m"), ("sum", "width + 1 m")]), &set(&[("width", "30 deg")]));
    assert_eq!(r.errors["sum"].code(), "mixed-kinds");
}

#[test]
fn cycles_fail_every_member_and_block_dependants() {
    let params = set(&[("a", "b + 1"), ("b", "a + 1"), ("c", "a * 2"), ("d", "c"), ("self", "self + 1"), ("fine", "1")]);
    let r = evaluate_all(&params, &BTreeMap::new());
    assert_eq!(r.values.len(), 1);
    assert_eq!(r.errors["a"].kind, ErrorKind::Cycle { members: names(&["a", "b"]) });
    assert_eq!(r.errors["b"], r.errors["a"]);
    assert_eq!(r.errors["self"].kind, ErrorKind::Cycle { members: names(&["self"]) });
    assert_eq!(r.errors["c"].kind, ErrorKind::FailedDependency { name: "a".into() });
    assert_eq!(r.errors["d"].kind, ErrorKind::FailedDependency { name: "c".into() });
    assert_eq!(r.order, names(&["fine", "a", "b", "c", "d", "self"]));
}

#[test]
fn a_failing_parameter_blocks_only_its_dependants() {
    let params = set(&[("zero", "0"), ("bad", "1 / zero"), ("later", "bad + 1"), ("indep", "zero + 5"), ("typo", "missing")]);
    let r = evaluate_all(&params, &BTreeMap::new());
    assert_eq!(r.errors["bad"].kind, ErrorKind::DivisionByZero);
    assert_eq!(r.errors["later"].kind, ErrorKind::FailedDependency { name: "bad".into() });
    assert_eq!(r.errors["typo"].kind, ErrorKind::UnknownParam { name: "missing".into() });
    assert_eq!(r.values["indep"], Value::Number(5.0));
}

#[test]
fn the_first_failed_dependency_by_name_is_reported() {
    let params = set(&[("x", "1 / 0"), ("y", "1 / 0"), ("z", "y + x")]);
    assert_eq!(evaluate_all(&params, &BTreeMap::new()).errors["z"].kind, ErrorKind::FailedDependency { name: "x".into() });
}

#[test]
fn declared_kinds_are_enforced() {
    let params = set(&[("w", "2 m"), ("n", "w / 1 m"), ("m", "n * 2")]);
    let declared: BTreeMap<String, Kind> = [("w".to_string(), Kind::Length), ("n".to_string(), Kind::Length)].into_iter().collect();
    let r = evaluate_all_declared(&params, &BTreeMap::new(), &declared);
    assert_eq!(r.errors["n"].kind, ErrorKind::DeclaredKind { declared: Kind::Length, found: Kind::Number });
    assert_eq!(r.errors["m"].kind, ErrorKind::FailedDependency { name: "n".into() });
    assert!(r.values.contains_key("w"));
}

#[test]
fn the_result_is_deterministic_and_independent_of_construction_order() {
    let rows = [("a", "1"), ("b", "a + 1"), ("c", "b + a"), ("d", "c * b"), ("e", "d - a"), ("x", "y"), ("y", "x")];
    let forward = evaluate_all(&set(&rows), &BTreeMap::new());
    let mut reversed = rows;
    reversed.reverse();
    assert_eq!(evaluate_all(&set(&reversed), &BTreeMap::new()), forward);
}

#[test]
fn an_empty_set_resolves_to_nothing() {
    assert_eq!(evaluate_all(&BTreeMap::new(), &BTreeMap::new()), Resolved::default());
}

#[test]
fn long_chains_resolve_without_recursion_limits() {
    let mut rows = vec![("p0".to_string(), "1".to_string())];
    for i in 1..2000 {
        rows.push((format!("p{i}"), format!("p{} + 1", i - 1)));
    }
    let params: BTreeMap<String, Expr> = rows.iter().map(|(n, s)| (n.clone(), parse(s).unwrap())).collect();
    let r = evaluate_all(&params, &BTreeMap::new());
    assert_eq!(r.values["p1999"], Value::Number(2000.0));
    assert_eq!(r.order.len(), 2000);
}
