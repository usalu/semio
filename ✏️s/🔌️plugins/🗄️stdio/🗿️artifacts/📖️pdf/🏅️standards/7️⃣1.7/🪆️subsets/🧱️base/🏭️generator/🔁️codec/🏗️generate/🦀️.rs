//! 🏭️ Writes the `pdf@1.7/base` fixture corpus: for each declared kind, a `base.pdf` (seed, arranged so
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

use pdf_base_lopdf::{apply, arrange, build_seed, project, KINDS};

fn fixture_directory(kind: &str) -> &'static str {
    match kind {
        "insert-page" => "📥️insert-page",
        "remove-page" => "🗑️remove-page",
        "move-page" => "🔀️move-page",
        "set-page-media-box" => "📐️set-page-media-box",
        "set-page-crop-box" => "✂️set-page-crop-box",
        "set-page-rotation" => "🔄️set-page-rotation",
        "set-page-content" => "✏️set-page-content",
        "append-page-content" => "➕️append-page-content",
        "set-info" => "ℹ️set-info",
        "insert-object" => "📦️insert-object",
        "remove-object" => "🧹️remove-object",
        "set-object-value" => "🔧️set-object-value",
        "set-dict-entry" => "🔑️set-dict-entry",
        "remove-dict-entry" => "🚫️remove-dict-entry",
        "set-trailer-entry" => "🧳️set-trailer-entry",
        "remove-trailer-entry" => "🧽️remove-trailer-entry",
        "set-page-box" => "🖼️set-page-box",
        "set-page-user-unit" => "📏️set-page-user-unit",
        "insert-content" => "🖋️insert-content",
        "remove-content" => "🧻️remove-content",
        "replace-content" => "🔁️replace-content",
        "insert-annotation" => "📌️insert-annotation",
        "remove-annotation" => "📍️remove-annotation",
        "set-annotation" => "📝️set-annotation",
        "set-font" => "🔤️set-font",
        "remove-font" => "🅾️remove-font",
        "set-image" => "🏞️set-image",
        "remove-image" => "🌫️remove-image",
        "set-form" => "📄️set-form",
        "remove-form" => "🗞️remove-form",
        "set-ext-g-state" => "🎛️set-ext-g-state",
        "remove-ext-g-state" => "🎚️remove-ext-g-state",
        "set-shading" => "🌅️set-shading",
        "remove-shading" => "🌄️remove-shading",
        "set-pattern" => "🧩️set-pattern",
        "remove-pattern" => "🪡️remove-pattern",
        "set-color-space" => "🌈️set-color-space",
        "remove-color-space" => "🎨️remove-color-space",
        "set-properties" => "🏷️set-properties",
        "remove-properties" => "🔖️remove-properties",
        "set-embedded-file" => "📎️set-embedded-file",
        "remove-embedded-file" => "🗃️remove-embedded-file",
        "set-outlines" => "📑️set-outlines",
        "set-named-destination" => "🎯️set-named-destination",
        "remove-named-destination" => "🎪️remove-named-destination",
        "set-page-labels" => "🔢️set-page-labels",
        "set-output-intents" => "🏳️set-output-intents",
        "set-acro-form" => "📋️set-acro-form",
        "set-optional-content" => "👁️set-optional-content",
        "set-page-layout" => "📖️set-page-layout",
        "set-page-mode" => "🖥️set-page-mode",
        "set-viewer-preferences" => "🛠️set-viewer-preferences",
        "set-open-action" => "🚪️set-open-action",
        "set-language" => "🗣️set-language",
        "set-mark-info" => "🔏️set-mark-info",
        "set-metadata" => "🧾️set-metadata",
        "set-document-id" => "🆔️set-document-id",
        "set-encryption" => "🔐️set-encryption",
        "set-catalog-entry" => "🗂️set-catalog-entry",
        "remove-catalog-entry" => "🧺️remove-catalog-entry",
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
