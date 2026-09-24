//! 🏭️ Writes the `semio@v1/brep` JSON-carrier fixture corpus. Every pair is checked for OBSERVABILITY
//! before it is written — a mutation whose projection does not move is refused, not committed.
use std::fs;
use std::path::PathBuf;
use std::process::exit;

use semio_brep_json::{apply, build_seed, project, render, FIXTURE_DIRECTORY_BY_KIND, KINDS};

fn main() {
    let out_root = std::env::args().nth(1).expect("usage: generate <fixtures-dir>");
    let seed = build_seed();
    let mut failures: Vec<String> = Vec::new();
    let mut written = 0usize;
    for kind in KINDS {
        let mutated = match apply(kind, &seed) {
            Ok(value) => value,
            Err(error) => {
                failures.push(format!("{kind}: apply: {error}"));
                continue;
            }
        };
        let base_bytes = render(&seed);
        let mutated_bytes = render(&mutated);
        match (project(base_bytes.as_bytes()), project(mutated_bytes.as_bytes())) {
            (Ok(before), Ok(after)) if before == after => {
                failures.push(format!("{kind}: not observable in the carrier projection"));
                continue;
            }
            (Ok(_), Ok(_)) => {}
            (Err(error), _) | (_, Err(error)) => {
                failures.push(format!("{kind}: project: {error}"));
                continue;
            }
        }
        let directory = FIXTURE_DIRECTORY_BY_KIND.iter().find_map(|(registered, directory)| (registered == kind).then_some(*directory)).expect("every kind has one reviewed fixture directory");
        let dir = PathBuf::from(&out_root).join(directory);
        fs::create_dir_all(&dir).expect("mkdir");
        fs::write(dir.join("⬅️before.json"), &base_bytes).expect("write before carrier");
        fs::write(dir.join("➡️after.json"), &mutated_bytes).expect("write after carrier");
        written += 1;
        println!("{kind}: observable before={}B after={}B", base_bytes.len(), mutated_bytes.len());
    }
    println!("[generate] wrote {written}/{} fixture pair(s) into {out_root}", KINDS.len());
    if !failures.is_empty() {
        for failure in &failures {
            eprintln!("[generate] {failure}");
        }
        exit(1);
    }
}
