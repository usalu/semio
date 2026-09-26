//! ⌨️ Laws of the text-input model over the language-agnostic fixture `🧫️fixtures/⌨️text-input/🔣️.json`: every typing
//! sequence ends at the fixture's text and selection under BOTH token schedules — `fast` (the initial tokens stay for the
//! whole sequence) and `slow` (the word-run tokenizer re-tokenizes after every step, as the guest's tokens do when they
//! arrive between keystrokes). Chromium's native `<textarea>` editing answers the same fixture (the renderer's
//! `⌨️text-input-oracle` suite), so the expectations are pinned by a third-party editing engine.

use super::*;
use serde_json::Value;

const FIXTURE: &str = include_str!("../../🧫️fixtures/⌨️text-input/🔣️.json");

/// 🔤️ The fixture's `word-runs` tokenizer: every maximal run of ASCII letters, digits and `_` is one `ident` token.
fn word_run_tokens(text: &str) -> String {
    let mut tokens = Vec::new();
    let mut start: Option<usize> = None;
    for (index, ch) in text.char_indices().chain(std::iter::once((text.len(), ' '))) {
        let word = ch.is_ascii_alphanumeric() || ch == '_';
        match (word, start) {
            (true, None) => start = Some(index),
            (false, Some(from)) => {
                tokens.push(format!(r#"{{"start":{from},"end":{index},"class":"ident"}}"#));
                start = None;
            }
            _ => {}
        }
    }
    format!("[{}]", tokens.join(","))
}

fn byte_offset(text: &str, chars: usize) -> usize {
    text.char_indices().nth(chars).map_or(text.len(), |(index, _)| index)
}

fn char_offset(text: &str, bytes: usize) -> usize {
    text[..bytes].chars().count()
}

fn apply_step(host: &mut EditorHost, step: &Value) {
    if let Some(typed) = step["type"].as_str() {
        for ch in typed.chars() {
            host.insert_text(&ch.to_string());
        }
    } else if let Some(key) = step["key"].as_str() {
        match key {
            "Backspace" => host.backspace(),
            "Delete" => host.delete_forward(),
            "ArrowLeft" => host.move_left(false),
            "ArrowRight" => host.move_right(false),
            "ArrowUp" => host.move_up(false),
            "ArrowDown" => host.move_down(false),
            "Home" => host.move_line_start(false),
            "End" => host.move_line_end(false),
            other => panic!("unknown key {other}"),
        }
    } else if let Some(selection) = step["select"].as_array() {
        let text = host.text().to_string();
        host.set_selection_range(byte_offset(&text, selection[0].as_u64().expect("anchor") as usize), byte_offset(&text, selection[1].as_u64().expect("caret") as usize));
    } else if let Some(pasted) = step["paste"].as_str() {
        host.replace_selection(pasted);
    } else if let Some(composed) = step["compose"].as_str() {
        host.insert_text(composed);
    } else {
        panic!("unknown step {step}");
    }
}

/// ⌨️ Runs one sequence under one schedule and answers the final text and selection in Unicode scalar values.
fn run(sequence: &Value, slow: bool) -> (String, [usize; 2]) {
    let mut host = EditorHost::new();
    host.set_size(800, 400, 1.0);
    let text = sequence["text"].as_str().expect("text").to_string();
    host.set_text(text.clone());
    host.set_semantic_tokens_json(&word_run_tokens(&text));
    let selection = sequence["selection"].as_array().expect("selection");
    host.set_selection_range(byte_offset(&text, selection[0].as_u64().expect("anchor") as usize), byte_offset(&text, selection[1].as_u64().expect("caret") as usize));
    for step in sequence["steps"].as_array().expect("steps") {
        if slow && step["type"].is_string() {
            for ch in step["type"].as_str().expect("typed").chars() {
                apply_step(&mut host, &serde_json::json!({ "type": ch.to_string() }));
                let current = host.text().to_string();
                host.set_semantic_tokens_json(&word_run_tokens(&current));
            }
            continue;
        }
        apply_step(&mut host, step);
        if slow {
            let current = host.text().to_string();
            host.set_semantic_tokens_json(&word_run_tokens(&current));
        }
    }
    let text = host.text().to_string();
    (text.clone(), [char_offset(&text, host.anchor()), char_offset(&text, host.caret())])
}

#[test]
fn every_typing_sequence_ends_where_the_fixture_says_under_both_token_schedules() {
    let fixture: Value = serde_json::from_str(FIXTURE).expect("text-input fixture");
    let sequences = fixture["sequences"].as_array().expect("sequences");
    assert!(sequences.len() >= 12, "the text-input law needs more than a happy path");
    for sequence in sequences {
        let id = sequence["id"].as_str().expect("id");
        let expected_text = sequence["expect"]["text"].as_str().expect("expected text");
        let expected_selection = sequence["expect"]["selection"].as_array().expect("expected selection").iter().map(|offset| offset.as_u64().expect("offset") as usize).collect::<Vec<_>>();
        for slow in [false, true] {
            let (text, selection) = run(sequence, slow);
            let schedule = if slow { "slow" } else { "fast" };
            assert_eq!(text, expected_text, "{id} ({schedule}): text");
            assert_eq!(selection.to_vec(), expected_selection, "{id} ({schedule}): selection");
        }
    }
}

#[test]
fn the_word_run_tokenizer_is_the_fixture_rule() {
    assert_eq!(word_run_tokens("MATCH (a1) x_y"), r#"[{"start":0,"end":5,"class":"ident"},{"start":7,"end":9,"class":"ident"},{"start":11,"end":14,"class":"ident"}]"#);
    assert_eq!(word_run_tokens(""), "[]");
}
