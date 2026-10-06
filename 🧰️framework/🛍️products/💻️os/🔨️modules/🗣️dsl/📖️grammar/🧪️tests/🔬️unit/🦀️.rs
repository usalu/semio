#[path = "../../../../../../../🔨️modules/📁️filesystem/🔎️discovery/🦀️.rs"]
mod directory_discovery;
use semio_framework_dsl::{LexOptions,lex};
use super::*;


/// 🌾️ Compiles every General-owned grammar; extension facets own their conformance laws.
#[semio_framework_async_macros::async_test]
async fn every_shipped_grammar_semio_parses_and_compiles() {
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../..");
    let grammar_root = root.join("🧰️framework");
    let mut reported = usize::MAX;
    let discovery = directory_discovery::discover(
        grammar_root.clone(),
        |directory| std::fs::canonicalize(directory),
        |directory| std::fs::read_dir(directory)?.map(|entry| entry.map(|entry| { let path = entry.path(); directory_discovery::DiscoveryEntry { directory: path.is_dir(), value: path } })).collect(),
        |path| path.file_name().and_then(|name| name.to_str()).is_some_and(|name| name.ends_with(".grammar.semio")),
        |count, directory| { if count != reported && count % 256 == 0 { println!("[DEBUG] grammar discovery: {count} physical directories, {}", directory.display()); reported = count; } true },
    ).expect("grammar source discovery");
    assert!(!discovery.cancelled, "grammar source discovery cancelled");
    let files = discovery.files;
    assert!(!files.is_empty(), "found zero *.grammar.semio under {}", grammar_root.display());

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
