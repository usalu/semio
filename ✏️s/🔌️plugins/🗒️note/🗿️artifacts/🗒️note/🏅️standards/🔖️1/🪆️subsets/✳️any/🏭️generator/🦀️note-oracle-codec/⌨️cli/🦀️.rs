//! 🚪️ Entry point — two roles, one binary, exactly `…✳️cad/🔬️probes/🦀️oracle-probe`'s shape:
//!
//!   note-oracle-codec generate --out <dir> [--only <recipe-id>]*
//!   note-oracle-codec dxf-project|svg-project|pdf-project --input <path>
//!   note-oracle-codec dxf-compare|svg-compare|pdf-compare --input <expected> --input <actual>
//!
//! `generate` is called by `../📜️script.ts` (the fixture generator) and, via `fixture reproduce`, by
//! the repository's own test harness. `*-project`/`*-compare` are called by `../../🔬️probes/📜️script.ts`.

use crate::dxf_codec;
use crate::pdf_codec;
use crate::recipes::recipes;
use crate::svg_codec;
use crate::{s, Json, Report};
use std::fs;
use std::path::Path;

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

fn ext_for(carrier: &str) -> &'static str {
    match carrier {
        "dxf" => "dxf",
        "svg" => "svg",
        "pdf" => "pdf",
        other => panic!("unknown carrier {other}"),
    }
}

fn encode(carrier: &str, doc: &crate::NoteDoc) -> Result<Vec<u8>, String> {
    match carrier {
        "dxf" => dxf_codec::write_dxf(doc),
        "svg" => svg_codec::write_svg(doc),
        "pdf" => pdf_codec::write_pdf(doc),
        other => Err(format!("unknown carrier {other}")),
    }
}

fn cmd_generate(args: &[String]) -> i32 {
    let out_index = args.iter().position(|a| a == "--out");
    let out = match out_index.and_then(|i| args.get(i + 1)) {
        Some(v) => v.clone(),
        None => {
            eprintln!("[generate] --out <dir> is required");
            return 2;
        }
    };
    let only = values(args, "--only");
    let all = recipes();
    let selected: Vec<_> = if only.is_empty() { all.iter().collect() } else { all.iter().filter(|r| only.iter().any(|o| o == r.id)).collect() };
    if selected.is_empty() {
        eprintln!("[generate] no recipe matches {only:?} — known: {}", all.iter().map(|r| r.id).collect::<Vec<_>>().join(", "));
        return 2;
    }
    let mut failed = 0;
    for recipe in selected {
        let dir = Path::new(&out).join(recipe.id);
        if let Err(e) = fs::create_dir_all(&dir) {
            eprintln!("[generate] {}: mkdir failed: {e}", recipe.id);
            failed += 1;
            continue;
        }
        let mut ok = true;
        for carrier in recipe.carriers {
            for (label, state) in [("before", &recipe.before), ("after", &recipe.after)] {
                match encode(carrier, state) {
                    Ok(bytes) => {
                        let path = dir.join(format!("{label}.{}", ext_for(carrier)));
                        if let Err(e) = fs::write(&path, &bytes) {
                            eprintln!("[generate] {}: write {path:?} failed: {e}", recipe.id);
                            ok = false;
                        }
                    }
                    Err(e) => {
                        eprintln!("[generate] {}: encode {carrier} {label} failed: {e}", recipe.id);
                        ok = false;
                    }
                }
            }
        }
        if ok {
            eprintln!("[generate] {} — {} ({})", recipe.id, recipe.mutation, recipe.carriers.join("+"));
        } else {
            failed += 1;
        }
    }
    if failed > 0 {
        1
    } else {
        0
    }
}

fn read_input(path: &str) -> Result<Vec<u8>, String> {
    fs::read(path).map_err(|e| format!("read {path}: {e}"))
}

fn cmd_project(carrier: &str, args: &[String]) -> String {
    let started = std::time::Instant::now();
    let engine = match carrier {
        "dxf" => ("dxf-rs", "dxf crate ascii reader/writer", "0.6"),
        "svg" => ("quick-xml", "quick-xml pull-parser/writer", "0.42"),
        "pdf" => ("lopdf", "lopdf content-stream reader/writer", "0.44"),
        _ => unreachable!(),
    };
    let mut report = Report::new(Box::leak(format!("note-{carrier}-project").into_boxed_str()), engine);
    let inputs = values(args, "--input");
    match inputs.first() {
        None => report.fail("no --input given".into()),
        Some(path) => match read_input(path) {
            Err(e) => report.fail(e),
            Ok(bytes) => {
                let projected = match carrier {
                    "dxf" => dxf_codec::project_dxf_json(&bytes),
                    "svg" => svg_codec::project_svg_json(&bytes),
                    "pdf" => pdf_codec::project_pdf_json(&bytes),
                    _ => unreachable!(),
                };
                match projected {
                    Ok(json) => {
                        report.put("bytes", Json::Int(bytes.len() as i64));
                        report.put("projection", json);
                    }
                    Err(e) => report.fail(e),
                }
            }
        },
    }
    report.emit(started.elapsed().as_millis())
}

fn cmd_compare(carrier: &str, args: &[String]) -> String {
    let started = std::time::Instant::now();
    let engine = match carrier {
        "dxf" => ("dxf-rs", "dxf crate ascii reader/writer", "0.6"),
        "svg" => ("quick-xml", "quick-xml pull-parser/writer", "0.42"),
        "pdf" => ("lopdf", "lopdf content-stream reader/writer", "0.44"),
        _ => unreachable!(),
    };
    let mut report = Report::new(Box::leak(format!("note-{carrier}-compare").into_boxed_str()), engine);
    let inputs = values(args, "--input");
    if inputs.len() != 2 {
        report.fail(format!("expected exactly two --input (expected, actual), got {}", inputs.len()));
        return report.emit(started.elapsed().as_millis());
    }
    let (expected, actual) = match (read_input(&inputs[0]), read_input(&inputs[1])) {
        (Ok(e), Ok(a)) => (e, a),
        (Err(e), _) | (_, Err(e)) => {
            report.fail(e);
            return report.emit(started.elapsed().as_millis());
        }
    };
    let outcome = match carrier {
        "dxf" => dxf_codec::compare_dxf(&expected, &actual),
        "svg" => svg_codec::compare_svg(&expected, &actual),
        "pdf" => pdf_codec::compare_pdf(&expected, &actual),
        _ => unreachable!(),
    };
    match outcome {
        Ok((equal, problems)) => {
            report.put("agree", Json::Bool(equal));
            report.put("differenceCount", Json::Int(problems.len() as i64));
            report.put("differences", Json::Arr(problems.into_iter().map(|p| s(&p)).collect()));
            if !equal {
                // 🔎️Not a probe FAILURE — a probe that ran cleanly and found a real disagreement is
                // reporting exactly what it was asked to. `status` stays "ok"; the orchestrator reads
                // `agree`.
            }
        }
        Err(e) => report.fail(e),
    }
    report.emit(started.elapsed().as_millis())
}

pub fn run() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let command = args.first().cloned().unwrap_or_default();
    let rest = if args.is_empty() { &args[..] } else { &args[1..] };
    let code = match command.as_str() {
        "generate" => cmd_generate(rest),
        "dxf-project" => {
            println!("{}", cmd_project("dxf", rest));
            0
        }
        "svg-project" => {
            println!("{}", cmd_project("svg", rest));
            0
        }
        "pdf-project" => {
            println!("{}", cmd_project("pdf", rest));
            0
        }
        "dxf-compare" => {
            println!("{}", cmd_compare("dxf", rest));
            0
        }
        "svg-compare" => {
            println!("{}", cmd_compare("svg", rest));
            0
        }
        "pdf-compare" => {
            println!("{}", cmd_compare("pdf", rest));
            0
        }
        other => {
            eprintln!("[note-oracle-codec] unknown command {other:?} — expected generate | dxf-project | svg-project | pdf-project | dxf-compare | svg-compare | pdf-compare");
            2
        }
    };
    std::process::exit(code);
}
