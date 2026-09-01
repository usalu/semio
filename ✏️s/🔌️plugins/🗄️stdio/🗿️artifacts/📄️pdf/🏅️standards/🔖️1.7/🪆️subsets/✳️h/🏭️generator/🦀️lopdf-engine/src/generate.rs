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

use pdf_h_lopdf::{apply, arrange, build_seed, project, KINDS};

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
        let dir = PathBuf::from(&out_root).join(kind);
        fs::create_dir_all(&dir).expect("mkdir");
        fs::write(dir.join("base.pdf"), &base).expect("write base.pdf");
        fs::write(dir.join("mutated.pdf"), &mutated).expect("write mutated.pdf");
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
