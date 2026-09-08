//! 🏭️ Writes the `pdf@1.7/vt` fixture corpus: for each declared kind, a `base.pdf` (seed, arranged so
//! the kind's precondition holds) and a `mutated.pdf` (the forward mutation applied THROUGH `lopdf`).
//!
//! Generation and execution are separate operations: this is the only command that writes into
//! `../🧫️fixtures/`, and its output is reviewed and committed before any test reads it.
//!
//! Every pair is checked for OBSERVABILITY before it is written — a mutation whose projection does not
//! move is not evidence of anything, and silently committing one would manufacture a passing test.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use pdf_vt_lopdf::{apply, arrange, build_seed, project, KINDS};

fn fixture_directory(kind: &str) -> &'static str {
    match kind {
        "set-output-intent" => "🏳️set-output-intent",
        "remove-output-intent" => "🧽️remove-output-intent",
        "set-trim-box" => "📐️set-trim-box",
        "remove-trim-box" => "✂️remove-trim-box",
        "embed-font-file" => "🔤️embed-font-file",
        "remove-font-file" => "🧺️remove-font-file",
        "insert-javascript-action" => "📜️insert-javascript-action",
        "remove-javascript-action" => "🚫️remove-javascript-action",
        "insert-launch-action" => "🚀️insert-launch-action",
        "remove-launch-action" => "🛬️remove-launch-action",
        "insert-media-annotation" => "🎬️insert-media-annotation",
        "remove-media-annotation" => "⏹️remove-media-annotation",
        "set-dpart-root" => "🗂️set-dpart-root",
        "remove-dpart-root" => "🧹️remove-dpart-root",
        "set-dpart-metadata" => "🏷️set-dpart-metadata",
        "remove-dpart-metadata" => "🗑️remove-dpart-metadata",
        _ => panic!("undeclared fixture kind: {kind}"),
    }
}

fn main() {
    let out_root = env::args().nth(1).unwrap_or_else(|| "../../🧫️fixtures".to_string());
    let seed = build_seed();
    let mut failures: Vec<String> = Vec::new();
    let mut written = 0usize;

    for kind in KINDS {
        let base = arrange(kind, &seed);
        let mutated = match apply(kind, &base) {
            Ok(bytes) => bytes,
            Err(error) => {
                failures.push(format!("{kind}: apply: {error}"));
                continue;
            }
        };
        let (before, after) = match (project(&base), project(&mutated)) {
            (Ok(before), Ok(after)) => (before, after),
            (Err(error), _) | (_, Err(error)) => {
                failures.push(format!("{kind}: project: {error}"));
                continue;
            }
        };
        if before == after {
            failures.push(format!("{kind}: not observable in the conformance projection"));
            continue;
        }
        let dir = PathBuf::from(&out_root).join(fixture_directory(kind));
        fs::create_dir_all(&dir).expect("mkdir");
        fs::write(dir.join("⬅️before.pdf"), &base).expect("write before PDF");
        fs::write(dir.join("➡️after.pdf"), &mutated).expect("write after PDF");
        written += 1;
        println!("{kind}: observable base={}B mutated={}B", base.len(), mutated.len());
    }

    println!("[generate] wrote {written}/{} fixture pair(s) into {out_root}", KINDS.len());
    if !failures.is_empty() {
        for failure in &failures {
            eprintln!("[generate] {failure}");
        }
        exit(1);
    }
}
