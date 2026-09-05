//! 🔣️ Third-party JSON fixture generator for `s.stdio.json@rfc8259/🧱️base`.
//!
//! Every fixture is a `(⬅️before.json, ➡️after.json)` PAIR for exactly one mutation kind. Both halves are
//! built as `serde_json::Value` trees and serialized by serde_json itself, so the expectation never
//! passes through our codec — `json-rust-rfc8259-mutate` reads both halves and the difference it
//! reports is what the mutation must produce. The `after` tree is authored, never computed by applying
//! one of our mutations, which is what keeps this a READER oracle rather than a predicting one.
//!
//! `preserve_order` is on deliberately: `set-member` and `remove-member` are only observable as
//! MEMBER-SET changes if member order survives the round trip.

use serde_json::{json, Value};

/// 🧪️ `(kind, before, after)` — one authored pair per manifested mutation kind.
fn corpus() -> Vec<(&'static str, Value, Value)> {
    let base = || json!({ "name": "semio", "tags": ["a", "b", "c"], "count": 3, "nested": { "keep": true } });

    let mut set_member = base();
    set_member["name"] = json!("semio-drawing");

    let mut remove_member = base();
    remove_member.as_object_mut().unwrap().remove("count");

    let mut insert_array_element = base();
    insert_array_element["tags"].as_array_mut().unwrap().insert(1, json!("inserted"));

    let mut remove_array_element = base();
    remove_array_element["tags"].as_array_mut().unwrap().remove(0);

    let mut set_scalar = base();
    set_scalar["nested"]["keep"] = json!(false);

    vec![
        ("✏️set-member", base(), set_member),
        ("🗑️remove-member", base(), remove_member),
        ("📥️insert-array-element", base(), insert_array_element),
        ("📤️remove-array-element", base(), remove_array_element),
        ("🔢️set-scalar", base(), set_scalar),
    ]
}

fn main() {
    let out = std::env::args().nth(1).unwrap_or_else(|| { eprintln!("usage: generate <out-dir>"); std::process::exit(2) });
    let mut written = 0usize;
    for (kind, before, after) in corpus() {
        assert_ne!(before, after, "{kind}: a fixture pair whose halves are equal proves nothing");
        let dir = std::path::Path::new(&out).join(kind);
        std::fs::create_dir_all(&dir).expect("fixture directory");
        for (name, value) in [("⬅️before.json", &before), ("➡️after.json", &after)] {
            let mut bytes = serde_json::to_vec_pretty(value).expect("serialize");
            bytes.push(b'\n');
            std::fs::write(dir.join(name), bytes).expect("write");
            written += 1;
        }
        println!("{kind}");
    }
    eprintln!("{written} file(s)");
}
