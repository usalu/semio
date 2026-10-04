use semio_framework_dsl::{LexOptions,lex};
use super::*;


/// 🌾️ Walks every handcrafted `📖️.grammar.semio` shipped under `✏️s/🔌️plugins` and
/// asserts it parses and compiles into a `Recognizer`. This is the runtime guard for the
/// normative grammar sources: it needs only `parse_grammar`/`Recognizer` from this crate, so
/// unlike `🧹️fixture-sweep`'s `m5_handcrafted_grammar_conformance` it does not pull the plugin
/// crates in as dev-dependencies and therefore stays runnable while those are mid-migration.
#[semio_framework_async_macros::async_test]
async fn every_shipped_grammar_semio_parses_and_compiles() {
    fn collect(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        let Ok(entries) = std::fs::read_dir(dir) else { return };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect(&path, out);
            } else if path.file_name().and_then(|n| n.to_str()).is_some_and(|name| name.ends_with(".grammar.semio")) {
                out.push(path);
            }
        }
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../..");
    let plugins = root.join("\u{270f}\u{fe0f}s/\u{1f50c}\u{fe0f}plugins");
    let mut files = Vec::new();
    collect(&plugins, &mut files);
    assert!(!files.is_empty(), "found zero *.grammar.semio under {}", plugins.display());

    let mut failures: Vec<String> = Vec::new();
    for file in &files {
        let text = std::fs::read_to_string(file).unwrap_or_else(|e| panic!("read {}: {e}", file.display()));
        match parse_grammar(&text) {
            Err(error) => failures.push(format!("{}: parse: {error:?}", file.display())),
            Ok(grammar) => {
                if grammar.dialect != SemioDialect::Grammar {
                    failures.push(format!("{}: dialect {:?}, expected Grammar", file.display(), grammar.dialect));
                    continue;
                }
                let _ = Recognizer::compile(&grammar, &crate::os_dsl::grammar::family_fragments().expect("OS family grammar"), crate::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
            }
        }
    }
    assert!(failures.is_empty(), "{} of {} grammars failed:\n{}", failures.len(), files.len(), failures.join("\n"));
    println!("[grammar-sweep] {} handcrafted grammars parsed and compiled", files.len());
}


#[semio_framework_async_macros::async_test]
async fn recognizer_matches_plain_arrow_via_registered_edge_macro() {
    let grammar = parse_grammar("grammar demo\nstart doc\ndoc = edge\n").expect("parse_grammar");
    let recognizer = Recognizer::compile(&grammar, &crate::os_dsl::grammar::family_fragments().expect("OS family grammar"), crate::os_dsl::grammar::product_macros()).expect("selected grammar fragments");
    assert!(recognizer.recognize("a->b").expect("recognize"));
    assert!(recognizer.recognize("a -[e1:Connection]->b").expect("recognize"));
    assert!(!recognizer.recognize("a-> ->").expect("recognize"));
}


#[semio_framework_async_macros::async_test]
async fn walk_protocol_shape_a_spk_like_buffer() {
    let source = r#"dialect protocol
protocol demo.pack
version 1
schema demo
start frame
framing magic 0x8953504B0D0A1A0A
header fixed 12
field format_major u16
field format_minor u16
field flags u32
field header_crc32 u32
segment kind u8
segment flags u8
segment payload varint bytes
footer fixed 84
"#;
    let spec = parse_protocol(source).expect("parse");
    let mut bytes = vec![0x89, b'S', b'P', b'K', 0x0D, 0x0A, 0x1A, 0x0A];
    bytes.extend_from_slice(&1u16.to_le_bytes());
    bytes.extend_from_slice(&0u16.to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.extend_from_slice(&0u32.to_le_bytes());
    bytes.push(1);
    bytes.push(0);
    bytes.push(0);
    bytes.extend(std::iter::repeat_n(0u8, 84));
    let trace = walk_protocol(&spec, &bytes).expect("walk Shape A");
    assert_eq!(trace.consumed, bytes.len());
    verify_protocol_bytes(&parse_grammar(&print_protocol(&spec)).expect("project protocol"), &bytes).expect("shallow verify");
    verify_protocol_source(source, &bytes).expect("deep verify");
    let mut bad = bytes.clone();
    bad[0] = 0x00;
    assert!(walk_protocol(&spec, &bad).is_err());
}


#[semio_framework_async_macros::async_test]
async fn verify_protocol_bytes_accepts_any_0x89_magic() {
    let g = GrammarFile { dialect: SemioDialect::Protocol, id: "demo.pack".into(), extension: None, uses: vec![], start: "frame".into(), productions: vec![], lex: LexOptions::default() };
    let mut bytes = vec![0x89];
    bytes.extend(std::iter::repeat_n(0u8, 31));
    verify_protocol_bytes(&g, &bytes).expect("any 0x89");
    bytes[0] = 0x00;
    assert!(verify_protocol_bytes(&g, &bytes).is_err());
    let spr = GrammarFile { dialect: SemioDialect::Protocol, id: "demo.spr".into(), extension: None, uses: vec![], start: "record".into(), productions: vec![], lex: LexOptions::default() };
    verify_protocol_bytes(&spr, &[1u8]).expect("spr non-empty");
    assert!(verify_protocol_bytes(&spr, &[]).is_err());
}
