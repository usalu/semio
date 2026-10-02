//! 🧾️ Every committed `.dsl.semio` carrier lexes strictly and braces each record of a record list (`[ { k=v } ]`):
//! the decoder has no bare-record fallback, so `[` directly followed by `ident =` can never parse.

use super::*;
use std::path::PathBuf;

fn repository_root() -> PathBuf {
    let mut directory = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    while !directory.join("nx.json").is_file() {
        assert!(directory.pop(), "repository root (nx.json) above {}", env!("CARGO_MANIFEST_DIR"));
    }
    directory
}

#[test]
fn committed_dsl_carriers_brace_every_record_list_item() {
    let root = repository_root();
    let listed = std::process::Command::new("git").args(["ls-files", "-co", "--exclude-standard", "-z", "--", "*.dsl.semio"]).current_dir(&root).output().expect("git lists committed carriers");
    assert!(listed.status.success(), "{}", String::from_utf8_lossy(&listed.stderr));
    let limits = Limits { max_bytes: usize::MAX, max_tokens: usize::MAX, max_depth: usize::MAX, max_nodes: usize::MAX };
    let (mut carriers, mut bare) = (0usize, Vec::new());
    for path in listed.stdout.split(|byte| *byte == 0).filter(|path| !path.is_empty()) {
        let path = std::str::from_utf8(path).expect("utf-8 carrier path");
        if path.contains("/🎫️tickets/") {
            continue;
        }
        let text = std::fs::read_to_string(root.join(path)).unwrap_or_else(|error| panic!("{path}: {error}"));
        let tokens: Vec<_> = lex(&text, &limits, false).unwrap_or_else(|error| panic!("{path}: {error}")).into_iter().filter(|token| !token.kind.is_trivia()).collect();
        carriers += 1;
        for window in tokens.windows(3) {
            if window[0].kind == TokenKind::LBracket && window[1].kind == TokenKind::Ident && window[2].kind == TokenKind::Equals {
                bare.push(format!("{path}:{}", text[..window[0].byte_range.0 as usize].matches('\n').count() + 1));
            }
        }
    }
    assert!(carriers > 200, "expected the committed carrier corpus, found {carriers}");
    assert!(bare.is_empty(), "{} bare record lists:\n{}", bare.len(), bare.join("\n"));
}
