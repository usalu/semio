//! 🧬️ The committed-vector laws every state-lane and document mutation case asserts over one report of its owner's
//! `*_mutation_report_json` bridge (`store::test_support::mutation_report_json`): the applied snapshot is the committed
//! after-snapshot, the produced delta is the committed `🔺️diff`, the diagnostics are the ones the committed `🎯️outcome`
//! declares, a kind the vector shows moving really moved, and the mutation's own computed inverse restores the
//! committed before-snapshot.
//!
//! @see ../🦀️.rs — `divergence`, `mutation_is_observable`, `inverse_restores`.

use super::{divergence, inverse_restores, mutation_is_observable};
use semio_repo_test_host::{parse_json, Context, Json};

//#region 🔖️Vector
/// 🧫️ One kind's committed `(before, mutation, after, diff, outcome)` specification vector, read literally.
pub struct Vector<'a> {
    pub before: &'a str,
    pub mutation: &'a str,
    pub after: &'a str,
    pub diff: &'a str,
    pub outcome: &'a str,
    /// 👁️ Whether the committed after-snapshot differs from the before-snapshot; `false` only for a record with no
    /// observable field (named, with its reason, in the case's feature).
    pub observable: bool,
}

/// 📜️ The five committed leaves a scenario's doc string addresses by URI (`before`, `mutation`, `after`, `diff`,
/// `outcome`), read through the plan's declared fixtures — so the feature stays the one place a vector path is written.
pub struct Leaves {
    pub before: String,
    pub mutation: String,
    pub after: String,
    pub diff: String,
    pub outcome: String,
}

impl Leaves {
    /// 📥️ Reads every leaf the scenario's doc string names.
    pub fn read(ctx: &Context) -> Result<Leaves, String> {
        let spec = ctx.doc_json()?;
        let text = |member: &str| -> Result<String, String> {
            let uri = spec.str(member);
            String::from_utf8(ctx.fixture_bytes(&uri)?).map_err(|error| format!("{uri}: {error}"))
        };
        Ok(Leaves { before: text("before")?, mutation: text("mutation")?, after: text("after")?, diff: text("diff")?, outcome: text("outcome")? })
    }

    /// 🧫️ The leaves as the vector the laws judge.
    pub fn vector(&self, observable: bool) -> Vector<'_> {
        Vector { before: &self.before, mutation: &self.mutation, after: &self.after, diff: &self.diff, outcome: &self.outcome, observable }
    }
}
//#endregion 🔖️Vector

//#region 🔖️Report
fn member<'a>(report: &'a Json, key: &str) -> Result<&'a Json, String> {
    report.get(key).ok_or_else(|| format!("the report carries no {key:?} member"))
}

fn members(report: &Json, key: &str) -> Result<Vec<Json>, String> {
    match member(report, key)? {
        Json::Array(items) => Ok(items.clone()),
        other => Err(format!("the report's {key:?} member is {}, not an array", other.to_string())),
    }
}

fn declared_outcome_holds(kind: &str, produced: &[Json], outcome: &Json) -> Result<(), String> {
    let codes: Vec<String> = produced.iter().map(|message| message.str("code")).collect();
    let levels: Vec<String> = produced.iter().map(|message| message.str("level")).collect();
    let status = outcome.str("status");
    let expected: Vec<String> = outcome.array("messages").iter().map(|message| message.str("code")).collect();
    if codes != expected {
        return Err(format!("mutate-{kind}: the vector declares the diagnostics {expected:?}, the implementation raised {codes:?}"));
    }
    let refused = levels.iter().any(|level| level == "error" || level == "fatal");
    match status.as_str() {
        "rejected" if !refused => Err(format!("mutate-{kind}: the vector declares a rejection, the implementation raised only {levels:?}")),
        "rejected" => Ok(()),
        _ if refused => Err(format!("mutate-{kind}: the vector declares {status:?}, the implementation refused at {levels:?}")),
        _ => Ok(()),
    }
}
//#endregion 🔖️Report

//#region 🔖️Laws
/// 🎯️ The forward law over one report: after-snapshot, delta and diagnostics are exactly what the vector commits, and
/// an `applied` vector really moved the snapshot. Returns the applied snapshot for the outcome payload.
pub fn mutate(kind: &str, report: &str, committed: &Vector<'_>) -> Result<Json, String> {
    let report = parse_json(report)?;
    let applied = member(&report, "snapshot")?;
    if let Some(first) = divergence(applied, member(&report, "expectedSnapshot")?) {
        return Err(format!("mutate-{kind}: the applied snapshot is not the committed after-snapshot — {first}"));
    }
    if let Some(first) = divergence(member(&report, "diff")?, &parse_json(committed.diff)?) {
        return Err(format!("mutate-{kind}: the produced delta is not the committed 🔺️diff — {first}"));
    }
    let outcome = parse_json(committed.outcome)?;
    declared_outcome_holds(kind, &members(&report, "messages")?, &outcome)?;
    let observed = committed.observable && outcome.str("status") == "applied";
    mutation_is_observable(kind, applied, member(&report, "base")?, if observed { &[] } else { std::slice::from_ref(&kind) })?;
    Ok(applied.clone())
}

/// ↩️ The inverse law over one report: every inverse step applies without refusal and restores the committed
/// before-snapshot exactly. Returns the restored snapshot for the outcome payload.
pub fn inverse(kind: &str, report: &str) -> Result<Json, String> {
    let report = parse_json(report)?;
    let faults: Vec<String> = members(&report, "inverseMessages")?.iter().filter(|message| matches!(message.str("level").as_str(), "error" | "fatal")).map(|message| message.str("code")).collect();
    if !faults.is_empty() {
        return Err(format!("inverse-{kind}: an inverse step was refused with {faults:?}"));
    }
    let restored = member(&report, "inverseSnapshot")?;
    inverse_restores(kind, restored, member(&report, "base")?)?;
    Ok(restored.clone())
}
//#endregion 🔖️Laws

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
