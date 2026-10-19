//! 🔬️ Third-party readers for `s.stdio.semio@v1/📐️cad`.
//!
//! Every number this binary emits comes out of a third-party library reading a file. `ruststep`
//! parses Part-21 and resolves the entity graph; the `dxf` crate parses ASCII DXF and also WRITES
//! the DXF fixtures. Nothing here predicts what a mutation ought to produce — it reads two artifacts
//! and reports how they differ, which is what keeps the reference external.
//!
//! ruststep is READ-ONLY by measurement, not by choice: 0.4.0 ships no Part-21 writer at all (no AST
//! type implements `Display`, and `ast::ser::to_record` stops at an in-memory `Record` whose nesting
//! disagrees with its own parser). STEP fixtures are therefore emitted here as deterministic Part-21
//! text and then VERIFIED by ruststep's parser before they are ever hashed — honestly `handcrafted`,
//! never claimed as third-party-generated.
//!
//! @see ../../../../../../../../../../../.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️27/SUBSET-SCOPED-EXTERNAL-ORACLE-MUTATION-TESTING/📓️cad-subset-oracle.md

use dxf::entities::*;
use dxf::{Drawing, Point};
use ruststep::ast::{EntityInstance, Exchange, Parameter, Record};
use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::str::FromStr;

#[path = "🧾️json-writer/🦀️.rs"]
mod json_writer;
use json_writer::*;

#[path = "📋️report-and-readings/🦀️.rs"]
mod report_and_readings;
use report_and_readings::*;

#[path = "🖊️dxf-and-step-readers/🦀️.rs"]
mod dxf_and_step_readers;
use dxf_and_step_readers::*;

#[path = "⚖️compare-readings/🦀️.rs"]
mod compare_readings;
use compare_readings::*;

#[path = "🧱️fixture-entity-builders/🦀️.rs"]
mod fixture_entity_builders;
use fixture_entity_builders::*;

#[path = "🖼️base-and-after-drawings/🦀️.rs"]
mod base_and_after_drawings;
use base_and_after_drawings::*;

#[path = "♻️rebuilds-and-step-writer/🦀️.rs"]
mod rebuilds_and_step_writer;
use rebuilds_and_step_writer::*;

#[path = "🏭️recipes-generate-and-verify/🦀️.rs"]
mod recipes_generate_and_verify;
use recipes_generate_and_verify::*;

//#region 🚪️Entry
// 🚫️async: pure argument helper.
fn values(args: &[String], flag: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == flag && i + 1 < args.len() {
            out.push(args[i + 1].clone());
            i += 2;
        } else {
            i += 1;
        }
    }
    out
}

/// 🧭️ A carrier that produced the SAME reading for both sides of a real mutation did not encode it.
///
/// This rule belongs to the WITNESS question ("did the carrier record the change at all?") and to
/// that question only. It is derived from the data rather than from a table: DXF drops the layer for
/// `Ellipse` and `Dimension`, and STEP drops layers, blocks and seven of the nine entity shapes
/// outright. In every one of those cases the two readings come back identical, and reporting that as
/// a passing `ok` would let the mutation clear a gate standing on the ABSENCE of the evidence.
///
/// 🚨️It must NEVER be applied to the AGREEMENT question ("does the oracle's answer match ours?"),
/// where two identical readings are precisely what success looks like. Wiring one rule into both
/// questions made the gate report `unsupported` for a known-GOOD pair — caught by validating the
/// gate in both directions, which is the whole reason that is mandatory.
// 🚫️async: pure predicate helper.
fn unwitnessed(mutation: &str, before: &Reading, after: &Reading) -> bool {
    !mutation.is_empty() && before == after
}

fn main() {
    let started = std::time::Instant::now();
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().cloned().unwrap_or_default();
    let inputs = values(&args, "--input");
    let mutation = values(&args, "--mutation").first().cloned().unwrap_or_default();
    let engine = if command.starts_with("step") { STEP_ENGINE } else { DXF_ENGINE };
    let probe: &'static str = match command.as_str() {
        "dxf-read" => "dxf-read",
        "dxf-witness" => "dxf-witness",
        "dxf-compare" => "dxf-compare",
        "step-read" => "step-read",
        "step-witness" => "step-witness",
        "step-compare" => "step-compare",
        "generate" => "generate",
        _ => {
            eprintln!("[probe] unknown command {command:?} — expected dxf-read | dxf-witness | dxf-compare | step-read | step-witness | step-compare | generate");
            std::process::exit(2);
        }
    };

    if command == "generate" {
        let out = values(&args, "--out").first().cloned().unwrap_or_default();
        let only = values(&args, "--only");
        let recipes: Vec<String> = if only.is_empty() { DXF_RECIPES.iter().chain(STEP_RECIPES.iter()).map(|r| r.to_string()).collect() } else { only };
        let mut failed = 0;
        for recipe in &recipes {
            match generate(recipe, &out) {
                Ok(files) => println!("[generate] {recipe}: {} file(s)", files.len()),
                Err(e) => {
                    eprintln!("[generate] {recipe}: {e}");
                    failed += 1;
                }
            }
        }
        std::process::exit(if failed > 0 { 1 } else { 0 });
    }

    let mut report = Report::new(probe, engine);
    let read = |path: &str| -> Result<(Reading, usize), String> { if command.starts_with("step") { read_step(path) } else { read_dxf(path).map(|r| (r, 0)) } };

    match probe {
        "dxf-read" | "step-read" => match inputs.first() {
            None => {
                report.status = "failed";
                report.diag("error", "no --input given".into(), None);
            }
            Some(path) => match read(path) {
                Err(e) => {
                    report.status = "failed";
                    report.diag("error", e, None);
                }
                Ok((reading, unresolved)) => {
                    let (l, b, en) = reading.counts();
                    report.put("layerCount", Json::Int(l as i64));
                    report.put("blockCount", Json::Int(b as i64));
                    report.put("entityCount", Json::Int(en as i64));
                    report.put("unresolvedEntities", Json::Int(unresolved as i64));
                    report.put("reading", reading.json());
                }
            },
        },
        "dxf-compare" | "step-compare" | "dxf-witness" | "step-witness" => {
            let witnessing = probe.ends_with("witness");
            if inputs.len() != 2 {
                report.status = "failed";
                report.diag("error", format!("{probe} needs exactly two --input, got {}", inputs.len()), None);
            } else {
                match (read(&inputs[0]), read(&inputs[1])) {
                    (Err(e), _) | (_, Err(e)) => {
                        report.status = "failed";
                        report.diag("error", e, None);
                    }
                    (Ok((left, ul)), Ok((right, ur))) => {
                        let (equal, worst, problems) = compare(&left, &right);
                        report.put("bothRead", Json::Bool(true));
                        report.put("unresolvedEntities", Json::Int((ul + ur) as i64));
                        report.put("readingsEqual", Json::Bool(equal));
                        report.put("maxAbsoluteDelta", Json::Num(worst));
                        report.put("toleranceUsed", Json::Num(NEAR_EXACT));
                        report.put("leftCounts", nums(&[left.counts().0 as f64, left.counts().1 as f64, left.counts().2 as f64]));
                        report.put("rightCounts", nums(&[right.counts().0 as f64, right.counts().1 as f64, right.counts().2 as f64]));
                        report.put("differenceCount", Json::Int(problems.len() as i64));
                        report.put("differences", Json::Arr(problems.iter().take(24).map(|p| Json::Str(p.clone())).collect()));
                        if witnessing {
                            let witnessed = !unwitnessed(&mutation, &left, &right);
                            report.put("carrierWitnessed", Json::Bool(witnessed));
                            if !witnessed {
                                report.status = "unsupported";
                                report.diag(
                                    "warning",
                                    format!("this carrier does not encode mutation {mutation:?}"),
                                    Some("before and after read back identical, so the carrier dropped whatever the mutation changed; reporting unsupported rather than a passing ok, which would stand on the absence of the evidence".into()),
                                );
                            }
                        } else {
                            report.put("agree", Json::Bool(equal));
                        }
                    }
                }
            }
        }
        _ => {}
    }
    println!("{}", report.emit(started.elapsed().as_millis()));
}
//#endregion 🚪️Entry
