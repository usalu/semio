//! 🕸️ Named parameter sets: dependency extraction, a deterministic evaluation plan with cycle detection, and evaluation of the whole set.
//!
//! The plan is layered: every round takes all parameters whose dependencies are done, sorted by name, so the order is a pure function of
//! the names and formulas. Parameters that remain are either on a cycle (reported with their strongly connected component, sorted) or
//! blocked behind one. References to names outside the set are ignored by the plan and reported by evaluation as unknown parameters.

use crate::errors::{ErrorKind, ExprError};
use crate::evaluation::evaluate_with;
use crate::kinds::Kind;
use crate::tree::{Expr, Value};
use std::collections::{BTreeMap, BTreeSet};

/// 🔗️ Every parameter the expression names, including those in branches that never run.
pub fn dependencies(expr: &Expr) -> BTreeSet<String> {
    let mut names = BTreeSet::new();
    collect(expr, &mut names);
    names
}

fn collect(expr: &Expr, names: &mut BTreeSet<String>) {
    if let Expr::Param(name) = expr {
        names.insert(name.clone());
    }
    for child in expr.children() {
        collect(child, names);
    }
}

/// 🗺️ The deterministic order in which a dependency graph can be evaluated, and what cannot be.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Plan {
    pub order: Vec<String>,
    pub cycles: Vec<Vec<String>>,
    pub blocked: Vec<String>,
}

/// 🧭️ Plans the evaluation of a graph given as `name -> names it depends on`; dependencies on absent names are ignored.
pub fn plan(graph: &BTreeMap<String, BTreeSet<String>>) -> Plan {
    let edges: BTreeMap<&String, Vec<&String>> = graph.iter().map(|(name, deps)| (name, deps.iter().filter(|d| graph.contains_key(*d)).collect())).collect();
    let mut remaining: BTreeSet<&String> = graph.keys().collect();
    let mut order = Vec::new();
    loop {
        let ready: Vec<&String> = remaining.iter().copied().filter(|name| edges[name].iter().all(|d| !remaining.contains(*d))).collect();
        if ready.is_empty() {
            break;
        }
        for name in ready {
            remaining.remove(name);
            order.push(name.clone());
        }
    }
    let reach: BTreeMap<&String, BTreeSet<&String>> = remaining
        .iter()
        .map(|&start| {
            let mut seen = BTreeSet::new();
            let mut stack: Vec<&String> = edges[start].iter().copied().filter(|d| remaining.contains(*d)).collect();
            while let Some(next) = stack.pop() {
                if seen.insert(next) {
                    stack.extend(edges[next].iter().copied().filter(|d| remaining.contains(*d)));
                }
            }
            (start, seen)
        })
        .collect();
    let mut cycles: Vec<Vec<String>> = Vec::new();
    let mut cyclic: BTreeSet<&String> = BTreeSet::new();
    for &name in &remaining {
        if reach[name].contains(name) && !cyclic.contains(name) {
            let component: Vec<&String> = reach[name].iter().copied().filter(|other| reach[*other].contains(name)).collect();
            cyclic.extend(component.iter().copied());
            cycles.push(component.into_iter().cloned().collect());
        }
    }
    let blocked = remaining.iter().copied().filter(|name| !cyclic.contains(*name)).cloned().collect();
    Plan { order, cycles, blocked }
}

/// 🎁️ The outcome of evaluating a parameter set: the order tried, the values that resolved and the error of every parameter that did not.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Resolved {
    pub order: Vec<String>,
    pub values: BTreeMap<String, Value>,
    pub errors: BTreeMap<String, ExprError>,
}

/// ▶️ Evaluates every parameter, with `overrides` replacing the formulas of existing parameters; an override of an absent name is an error.
pub fn evaluate_all(params: &BTreeMap<String, Expr>, overrides: &BTreeMap<String, Expr>) -> Resolved {
    evaluate_all_declared(params, overrides, &BTreeMap::new())
}

/// ▶️ [`evaluate_all`] that also rejects a parameter whose computed kind differs from the kind declared for it.
pub fn evaluate_all_declared(params: &BTreeMap<String, Expr>, overrides: &BTreeMap<String, Expr>, declared: &BTreeMap<String, Kind>) -> Resolved {
    let mut formulas: BTreeMap<&String, &Expr> = params.iter().collect();
    let mut resolved = Resolved::default();
    for (name, formula) in overrides {
        if formulas.contains_key(name) {
            formulas.insert(name, formula);
        } else {
            resolved.errors.insert(name.clone(), ExprError::parameter(ErrorKind::UnknownOverride));
        }
    }
    let graph: BTreeMap<String, BTreeSet<String>> = formulas.iter().map(|(name, expr)| ((*name).clone(), dependencies(expr))).collect();
    let plan = plan(&graph);
    let mut kinds: BTreeMap<String, Kind> = BTreeMap::new();
    let failed_dependency = |name: &String, errors: &BTreeMap<String, ExprError>, extra: &BTreeSet<&String>| {
        graph[name].iter().find(|d| graph.contains_key(*d) && (errors.contains_key(*d) || extra.contains(d))).cloned()
    };
    let blocked_or_cyclic: BTreeSet<&String> = plan.cycles.iter().flatten().chain(plan.blocked.iter()).collect();
    for name in &plan.order {
        let outcome = match failed_dependency(name, &resolved.errors, &BTreeSet::new()) {
            Some(dependency) => Err(ExprError::parameter(ErrorKind::FailedDependency { name: dependency })),
            None => evaluate_with(formulas[name], &resolved.values, &kinds).and_then(|value| match declared.get(name) {
                Some(want) if *want != value.kind() => Err(ExprError::parameter(ErrorKind::DeclaredKind { declared: *want, found: value.kind() })),
                _ => Ok(value),
            }),
        };
        match outcome {
            Ok(value) => {
                kinds.insert(name.clone(), value.kind());
                resolved.values.insert(name.clone(), value);
            }
            Err(error) => {
                resolved.errors.insert(name.clone(), error);
            }
        }
    }
    resolved.order = plan.order.clone();
    for members in &plan.cycles {
        for member in members {
            resolved.errors.insert(member.clone(), ExprError::parameter(ErrorKind::Cycle { members: members.clone() }));
        }
    }
    for name in &plan.blocked {
        let dependency = failed_dependency(name, &resolved.errors, &blocked_or_cyclic).unwrap_or_default();
        resolved.errors.insert(name.clone(), ExprError::parameter(ErrorKind::FailedDependency { name: dependency }));
    }
    resolved.order.extend(blocked_or_cyclic.iter().map(|name| (*name).clone()));
    resolved
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
