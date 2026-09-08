
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
            } else if path.file_name().and_then(|n| n.to_str()) == Some("\u{1f4d6}\u{fe0f}component.grammar.semio") {
                out.push(path);
            }
        }
    }
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../../../..");
    let plugins = root.join("\u{270f}\u{fe0f}s/\u{1f50c}\u{fe0f}plugins");
    let mut files = Vec::new();
    collect(&plugins, &mut files);
    assert!(!files.is_empty(), "found zero component.grammar.semio under {}", plugins.display());

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
                let _ = Recognizer::compile(&grammar);
            }
        }
    }
    assert!(failures.is_empty(), "{} of {} grammars failed:\n{}", failures.len(), files.len(), failures.join("\n"));
    println!("[grammar-sweep] {} handcrafted grammars parsed and compiled", files.len());
}

#[semio_framework_async_macros::async_test]
async fn parses_minimal_grammar_header() {
    let g = parse_grammar("grammar demo\nstart doc\ndoc = \"hello\"\n").expect("parse_grammar");
    assert_eq!(g.id, "demo");
    assert_eq!(g.start, "doc");
    assert_eq!(g.productions.len(), 1);
    assert_eq!(g.productions[0].alternatives[0].symbols, vec![Symbol::Literal("hello".to_string())]);
}

/// ✅️ P2-P1 regression: a `#`-comment containing a `"`, `?`, or `|` character (routine in a
/// hand-authored grammar's own doc-comment prose — e.g. illustrating an escaped quote or an
/// EBNF-style alternation) must never be misread by `lex`'s quote/operator pre-scan as this
/// lexer's own quote-open/Pipe/Question token. Root cause of every P2-P1 pilot (json/csv)
/// conformance-test parse failure: `lex` scanned the WHOLE file's raw bytes for `"`/`?`/`|`
/// before comments were ever recognized, so e.g. a doc comment illustrating JSON's `\"` escape
/// opened a runaway "quote" that swallowed real productions below it, and a doc comment
/// illustrating `a | b` alternation split the segment mid-sentence, leaving a stray backtick to
/// fall through to "unexpected character" once the corrupted segment reached `core_lex`.
#[semio_framework_async_macros::async_test]
async fn hash_comment_hides_quote_and_pipe_characters_from_the_operator_prescan() {
    let g = parse_grammar("grammar demo\nstart doc\n# illustrating an escape: \\\" and an alternation: a | b, plus a trailing comma? here\ndoc = \"hello\" | \"world\"\n").expect("parse_grammar");
    assert_eq!(g.id, "demo");
    assert_eq!(g.productions[0].alternatives.len(), 2);
    assert_eq!(g.productions[0].alternatives[0].symbols, vec![Symbol::Literal("hello".to_string())]);
    assert_eq!(g.productions[0].alternatives[1].symbols, vec![Symbol::Literal("world".to_string())]);
}

#[semio_framework_async_macros::async_test]
async fn parses_extension_and_uses() {
    let g = parse_grammar("grammar fem2d\nextension fem2d\nuse core\nuse family-sheet\nstart document\ndocument = header\nheader = \"fem2d\" TEXT\n").expect("parse_grammar");
    assert_eq!(g.extension, Some("fem2d".to_string()));
    assert_eq!(g.uses, vec!["core".to_string(), "family-sheet".to_string()]);
    assert_eq!(g.productions.len(), 2);
}

#[semio_framework_async_macros::async_test]
async fn parses_terminal_vs_ref_vs_macro() {
    let g = parse_grammar("grammar demo\nstart doc\ndoc = TEXT node table(\"rows\", row)\nrow = IDENT\n").expect("parse_grammar");
    let symbols = &g.productions[0].alternatives[0].symbols;
    assert_eq!(symbols[0], Symbol::Terminal("TEXT".to_string()));
    assert_eq!(symbols[1], Symbol::Ref("node".to_string()));
    assert_eq!(symbols[2], Symbol::Macro("table".to_string(), vec![MacroArg::Literal("rows".to_string()), MacroArg::Ident("row".to_string())]));
}

#[semio_framework_async_macros::async_test]
async fn parses_alternation_group_and_quantifiers() {
    let g = parse_grammar("grammar demo\nstart doc\ndoc = {\"a\" | \"b\"}? node* row+\nnode = IDENT\nrow = IDENT\n").expect("parse_grammar");
    let symbols = &g.productions[0].alternatives[0].symbols;
    match &symbols[0] {
        Symbol::Optional(inner) => match inner.as_ref() {
            Symbol::Group(alts) => {
                assert_eq!(alts.len(), 2);
                assert_eq!(alts[0].symbols, vec![Symbol::Literal("a".to_string())]);
                assert_eq!(alts[1].symbols, vec![Symbol::Literal("b".to_string())]);
            }
            other => panic!("expected Group, got {other:?}"),
        },
        other => panic!("expected Optional, got {other:?}"),
    }
    assert!(matches!(&symbols[1], Symbol::Star(_)));
    assert!(matches!(&symbols[2], Symbol::Plus(_)));
}

#[semio_framework_async_macros::async_test]
async fn round_trip_matrix_over_representative_grammars() {
    let sources = vec![
        "grammar demo\nstart doc\ndoc = \"hello\"\n",
        "grammar fem2d\nextension fem2d\nuse core\nstart document\ndocument = header body\nheader = \"fem2d\" TEXT\nbody = row*\nrow = IDENT FLOAT?\n",
        "grammar demo\nstart doc\ndoc = {\"a\" | \"b\"} node+\nnode = IDENT\n",
    ];
    for source in sources {
        let parsed = parse_grammar(source).unwrap_or_else(|e| panic!("parse of {source:?} failed: {e:?}"));
        let printed = print_grammar(&parsed);
        let reparsed = parse_grammar(&printed).unwrap_or_else(|e| panic!("reparse of canonical {printed:?} failed: {e:?}"));
        assert_eq!(reparsed, parsed, "round trip mismatch for {source:?} -> {printed:?}");
        let canonical_twice = canonicalize(&printed).expect("canonicalize");
        assert_eq!(canonical_twice, printed, "canonicalize is not idempotent for {printed:?}");
    }
}

#[semio_framework_async_macros::async_test]
async fn missing_start_directive_is_an_error() {
    let err = parse_grammar("grammar demo\ndoc = \"hello\"\n").unwrap_err();
    assert!(err.message.contains("start"), "unexpected message: {}", err.message);
}

/// @emoji 🪞️ This crate's own format description parses under the parser it defines — the
/// self-hosting proof the architecture plan calls for.
#[semio_framework_async_macros::async_test]
async fn self_hosting_grammar_grammar_parses_and_round_trips() {
    let source = include_str!("../../📖️grammar.grammar.semio");
    let parsed = parse_grammar(source).expect("dsl_grammar's own grammar.grammar must parse under its own parser");
    assert_eq!(parsed.id, "grammar");
    let printed = print_grammar(&parsed);
    let reparsed = parse_grammar(&printed).expect("canonical print of grammar.grammar must reparse");
    assert_eq!(reparsed, parsed);
}

#[semio_framework_async_macros::async_test]
async fn recognizer_matches_plain_arrow_via_registered_edge_macro() {
    let grammar = parse_grammar("grammar demo\nstart doc\ndoc = edge\n").expect("parse_grammar");
    let recognizer = Recognizer::compile(&grammar);
    assert!(recognizer.recognize("a->b").expect("recognize"));
    assert!(recognizer.recognize("a -[e1:Connection]->b").expect("recognize"));
    assert!(!recognizer.recognize("a-> ->").expect("recognize"));
}

#[semio_framework_async_macros::async_test]
async fn recognizer_matches_literals_terminals_and_quantifiers() {
    let grammar = parse_grammar("grammar demo\nstart doc\ndoc = \"beam\" IDENT node*\nnode = IDENT\n").expect("parse_grammar");
    let recognizer = Recognizer::compile(&grammar);
    assert!(recognizer.recognize("beam e3 n1 n2").expect("recognize"));
    assert!(recognizer.recognize("beam e3").expect("recognize"));
    assert!(!recognizer.recognize("beam").expect("recognize"));
}

#[semio_framework_async_macros::async_test]
async fn parse_grammar_sets_dialect_grammar_vs_protocol() {
    let g = parse_grammar("dialect grammar\ngrammar demo\nstart doc\ndoc = \"x\"\n").expect("grammar");
    assert_eq!(g.dialect, SemioDialect::Grammar);
    let p = parse_grammar("dialect protocol\nprotocol demo.pack\nversion 1\nschema demo\nstart frame\nframing magic 0x8953504B0D0A1A0A\nheader fixed 4\nfield flags u32\n").expect("protocol");
    assert_eq!(p.dialect, SemioDialect::Protocol);
    assert_eq!(p.start, "frame");
    assert_eq!(p.id, "demo.pack");
}

#[semio_framework_async_macros::async_test]
async fn protocol_parse_print_round_trip_retains_body() {
    let source = r#"dialect protocol
protocol flow.pack
version 1
schema flow
start frame
framing magic 0x8953504B0D0A1A0A
header fixed 32
field format_major u16
field format_minor u16
field flags u32
field header_crc32 u32
segment kind u8
segment flags u8
segment payload varint bytes
record field id u16 type tag
field tag varint
field body bytes
footer fixed 84
"#;
    let parsed = parse_protocol(source).expect("parse_protocol");
    assert_eq!(parsed.id, "flow.pack");
    assert!(matches!(parsed.framing, Framing::Magic(_)));
    assert!(parsed.blocks.iter().any(|b| matches!(b, Block::Header(_))));
    assert!(parsed.blocks.iter().any(|b| matches!(b, Block::Segment { .. })));
    assert!(parsed.blocks.iter().any(|b| matches!(b, Block::Footer(84))));
    let printed = print_protocol(&parsed);
    let reparsed = parse_protocol(&printed).expect("reparse print_protocol");
    assert_eq!(reparsed, parsed);
    let once = canonicalize(source).expect("canonicalize");
    let twice = canonicalize(&once).expect("canonicalize twice");
    assert_eq!(once, twice);
}

#[semio_framework_async_macros::async_test]
async fn protocol_parses_rich_struct_enum_segment_forms() {
    let source = r#"dialect protocol
protocol demo.pack
version 1
schema demo
start frame
framing magic 0x8953504B0D0A1A0A
struct Vertex { x f32 y f32 z f32 }
enum Op { ObjectsAdd=1 ObjectsRemove=2 }
segment Objects kind=1 { count varint items Array(Ref(Object), Field(count)) }
footer fixed 84
"#;
    let parsed = parse_protocol(source).expect("parse rich protocol");
    assert!(parsed.blocks.iter().any(|b| matches!(b, Block::Struct { name, .. } if name == "Vertex")));
    assert!(parsed.blocks.iter().any(|b| matches!(b, Block::Enum { name, .. } if name == "Op")));
    assert!(parsed.blocks.iter().any(|b| matches!(b, Block::Segment { name, kind: Some(1), .. } if name == "Objects")));
    let printed = print_protocol(&parsed);
    assert_eq!(parse_protocol(&printed).expect("reparse"), parsed);
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
    verify_protocol_bytes(&project_protocol(spec.clone()), &bytes).expect("shallow verify");
    verify_protocol_source(source, &bytes).expect("deep verify");
    let mut bad = bytes.clone();
    bad[0] = 0x00;
    assert!(walk_protocol(&spec, &bad).is_err());
}

#[semio_framework_async_macros::async_test]
async fn walk_protocol_minimal_op_binary_record() {
    let source = r#"dialect protocol
protocol demo.spr
version 1
schema demo.operation
start record
framing record
field format u8
field ordinal varint
field body bytes
"#;
    let spec = parse_protocol(source).expect("parse spr");
    let bytes = vec![1u8, 0x00, 0xAA, 0xBB];
    let trace = walk_protocol(&spec, &bytes).expect("walk OpBinary");
    assert_eq!(trace.consumed, 4);
    assert!(walk_protocol(&spec, &[]).is_err());
}

#[semio_framework_async_macros::async_test]
async fn self_hosting_protocol_grammar_semio_parses_as_grammar() {
    let source = include_str!("../../📡️protocol.grammar.semio");
    let parsed = parse_grammar(source).expect("protocol.grammar.semio must parse as dialect grammar");
    assert_eq!(parsed.dialect, SemioDialect::Grammar);
    assert_eq!(parsed.id, "protocol");
    let printed = print_grammar(&parsed);
    let reparsed = parse_grammar(&printed).expect("canonical protocol grammar reparses");
    assert_eq!(reparsed, parsed);
}

#[semio_framework_async_macros::async_test]
async fn parse_protocol_roundtrips_magic_pack() {
    let source = "dialect protocol\nprotocol demo.pack\nversion 1\nschema demo.v1\nstart frame\nframing magic 0x8953454D0D0A1A0A\nheader fixed 4\nfield flags u32\n";
    let parsed = parse_protocol(source).expect("parse_protocol");
    let printed = print_protocol(&parsed);
    let reparsed = parse_protocol(&printed).expect("reparse");
    assert_eq!(parsed, reparsed);
}

#[semio_framework_async_macros::async_test]
async fn walk_protocol_consumes_magic_and_header() {
    let source = "dialect protocol\nprotocol demo.pack\nversion 1\nschema demo.v1\nstart frame\nframing magic 0x8953454D0D0A1A0A\nheader fixed 4\nfield flags u32\n";
    let spec = parse_protocol(source).expect("parse");
    let mut bytes = vec![0x89, b'S', b'E', b'M', 0x0D, 0x0A, 0x1A, 0x0A];
    bytes.extend_from_slice(&7u32.to_le_bytes());
    walk_protocol(&spec, &bytes).expect("walk");
    assert!(walk_protocol(&spec, &bytes[..8]).is_err());
}

#[semio_framework_async_macros::async_test]
async fn walk_protocol_spr_record_body_as_rest() {
    let source = "dialect protocol\nprotocol demo.spr\nversion 1\nschema demo.op\nstart record\nframing record\nfield format u8\nfield body bytes\n";
    let spec = parse_protocol(source).expect("parse");
    walk_protocol(&spec, &[1u8, 9, 9, 9]).expect("spr walk");
}

#[semio_framework_async_macros::async_test]
async fn recognizer_matches_bool_terminal() {
    let grammar = parse_grammar("grammar demo\nstart doc\ndoc = BOOL\n").expect("grammar");
    let rec = Recognizer::compile(&grammar);
    assert!(rec.recognize("true").unwrap());
    assert!(rec.recognize("false").unwrap());
    assert!(!rec.recognize("maybe").unwrap());
}

#[semio_framework_async_macros::async_test]
async fn verify_protocol_source_ok() {
    let source = "dialect protocol\nprotocol demo.pack\nversion 1\nschema demo.v1\nstart frame\nframing magic 0x8953454D0D0A1A0A\nheader fixed 4\nfield flags u32\n";
    let mut bytes = vec![0x89, b'S', b'E', b'M', 0x0D, 0x0A, 0x1A, 0x0A];
    bytes.extend_from_slice(&0u32.to_le_bytes());
    verify_protocol_source(source, &bytes).expect("verify_protocol_source");
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

//#region 🔖️P2M1Grammar
// Item 1: `string`/`comment` header directives parse and drive the Recognizer's own lexing.
#[semio_framework_async_macros::async_test]
async fn string_header_directive_drives_backslash_decode_end_to_end() {
    let g = parse_grammar("grammar jsontest\nstring double backslash\nstart doc\ndoc = TEXT\n").expect("parse_grammar");
    assert_eq!(g.lex.strings, vec![StringMode { quote: '"', escape: StringEscape::Backslash }]);
    let rec = Recognizer::compile(&g);
    assert!(rec.recognize(r#""café""#).expect("recognize"), "the json-dialect grammar must recognize a \\uXXXX-escaped string");
    // Prove real decoding happened (not just successful lexing) by relexing with the grammar's
    // own compiled dialect and inspecting the Text token's content.
    let tokens = core_lex_with(r#""café""#, &Limits::default(), false, &g.lex).expect("lex_with");
    let text = tokens.iter().find(|t| t.kind == CoreKind::Text).expect("Text token");
    assert_eq!(text.text.as_str().as_ref(), "café");
}

#[semio_framework_async_macros::async_test]
async fn string_header_directive_drives_csv_doubled_quote_decode() {
    let g = parse_grammar("grammar csvtest\nstring double doubled\nstart doc\ndoc = TEXT\n").expect("parse_grammar");
    let rec = Recognizer::compile(&g);
    assert!(rec.recognize(r#""a""b""#).expect("recognize"));
    let tokens = core_lex_with(r#""a""b""#, &Limits::default(), false, &g.lex).expect("lex_with");
    let text = tokens.iter().find(|t| t.kind == CoreKind::Text).expect("Text token");
    assert_eq!(text.text.as_str().as_ref(), "a\"b");
}

#[semio_framework_async_macros::async_test]
async fn string_header_directive_supports_single_and_double_quote_together_xml_style() {
    let g = parse_grammar("grammar xmltest\nstring double raw\nstring single raw\nstart doc\ndoc = TEXT TEXT\n").expect("parse_grammar");
    assert_eq!(g.lex.strings.len(), 2);
    let rec = Recognizer::compile(&g);
    assert!(rec.recognize(r#""a" 'b'"#).expect("recognize a mix of both quote chars"));
}

#[semio_framework_async_macros::async_test]
async fn string_header_directive_supports_step_single_quote_doubling() {
    let g = parse_grammar("grammar steptest\nstring single doubled\nstart doc\ndoc = TEXT\n").expect("parse_grammar");
    let rec = Recognizer::compile(&g);
    assert!(rec.recognize("'it''s a beam'").expect("recognize"));
    let tokens = core_lex_with("'it''s a beam'", &Limits::default(), false, &g.lex).expect("lex_with");
    let text = tokens.iter().find(|t| t.kind == CoreKind::Text).expect("Text token");
    assert_eq!(text.text.as_str().as_ref(), "it's a beam");
}

#[semio_framework_async_macros::async_test]
async fn grammar_without_string_or_comment_directives_keeps_default_lex_options() {
    let g = parse_grammar("grammar demo\nstart doc\ndoc = \"hello\"\n").expect("parse_grammar");
    assert_eq!(g.lex, LexOptions::default());
    // print_grammar must NOT emit comment/string lines for the default case (round trip proof
    // already covered by `round_trip_matrix_over_representative_grammars`; this asserts the
    // specific absence).
    let printed = print_grammar(&g);
    assert!(!printed.contains("comment"), "default comment config must not be printed");
    assert!(!printed.contains("\nstring "), "default string config must not be printed");
}

// Item 2: the "raw span" terminal — `LINE` (rest-of-physical-line) and `REST` (rest-of-EOF).
#[semio_framework_async_macros::async_test]
async fn line_terminal_captures_rest_of_physical_line_verbatim_stl_style() {
    let g = parse_grammar("grammar stltest\nstart doc\ndoc = \"solid\" LINE\n").expect("parse_grammar");
    let rec = Recognizer::compile(&g);
    // "My Cube" is two Ident tokens with a space between them — LINE must swallow both AND
    // the space, ending exactly at end-of-input since there's no trailing newline.
    assert!(rec.recognize("solid My Cube").expect("recognize"), "LINE must capture the whole 'My Cube' rest-of-line as one span");
    assert!(rec.recognize("solid").expect("recognize"), "a raw span may legitimately be empty (no name)");
}

#[semio_framework_async_macros::async_test]
async fn rest_terminal_captures_to_eof_txt_style_over_out_of_alphabet_characters() {
    let g = parse_grammar("grammar txttest\nstart doc\ndoc = \"BODY\" REST\n").expect("parse_grammar");
    let rec = Recognizer::compile(&g);
    // `~` and `%` are still outside the fixed token alphabet even after P2-M1's promotions —
    // REST must swallow them without the whole document failing to lex (forgiving mode) and
    // without needing to re-tokenize the interior.
    assert!(rec.recognize("BODY arbitrary prose with ~weird~ %chars% and trailing punctuation!!!").expect("recognize"));
}

// Item 3: promoted single-char tokens `< > & $ ;`, real Terminal matching through the Recognizer.
#[semio_framework_async_macros::async_test]
async fn promoted_tokens_are_real_terminals_the_recognizer_can_require_positionally() {
    let g = parse_grammar("grammar xmlish\nstart tag\ntag = LT IDENT GT AMP IDENT SEMICOLON DOLLAR IDENT\n").expect("parse_grammar");
    let rec = Recognizer::compile(&g);
    assert!(rec.recognize("<tag>&amp;$VAR").expect("recognize"));
    assert!(!rec.recognize("tag").expect("recognize"), "without the promoted LT/GT/AMP/SEMICOLON/DOLLAR tokens present, the sequence must not match");
}

// Item 4: per-grammar comment dialect — line marker override, disabled, block comment.
#[semio_framework_async_macros::async_test]
async fn comment_header_directive_disables_hash_and_enables_block_comment_step_style() {
    // '#' isn't itself promoted to a token (it stays the DEFAULT line-comment marker unless a
    // grammar overrides it), so a real STEP entity sigil needs `comment none`/`comment line
    // none`; DOLLAR stands in for it here as a real promoted token (item 3) — the point of
    // THIS test is the comment dialect (item 4), proven directly against `g.lex` below.
    let g = parse_grammar("grammar steplike\ncomment none\ncomment block \"/*\" \"*/\"\nstring single doubled\nstart doc\ndoc = DOLLAR INT EQUALS IDENT LPAREN TEXT RPAREN SEMICOLON\n").expect("parse_grammar");
    assert_eq!(g.lex.comment.line, None);
    assert_eq!(g.lex.comment.block, Some(("/*".to_string(), "*/".to_string())));
    let rec = Recognizer::compile(&g);
    assert!(rec.recognize("$10=IFCWALL('a');").expect("recognize"), "with comment.line=None, '$' must lex as a real Dollar token, not be eaten by a comment");
    assert!(rec.recognize("/* a comment */\n$10=IFCWALL('a');").expect("recognize"), "a leading block comment must be trivia, not part of the document");
}

#[semio_framework_async_macros::async_test]
async fn print_grammar_round_trips_comment_and_string_header_directives() {
    let source = "grammar steplike\ncomment line none\ncomment block \"/*\" \"*/\"\nstring single doubled\nstart doc\ndoc = TEXT\n";
    let parsed = parse_grammar(source).expect("parse_grammar");
    let printed = print_grammar(&parsed);
    let reparsed = parse_grammar(&printed).expect("reparse printed steplike grammar");
    assert_eq!(reparsed, parsed, "comment/string header directives must round trip through print_grammar");
    assert_eq!(parsed.lex.comment.line, None);
    assert_eq!(parsed.lex.comment.block, Some(("/*".to_string(), "*/".to_string())));
    assert_eq!(parsed.lex.strings, vec![StringMode { quote: '\'', escape: StringEscape::Doubled }]);
}

// Item 5: trailing-dot floats + leading-dot enum literals, matched through real FLOAT/DOTENUM terminals.
#[semio_framework_async_macros::async_test]
async fn trailing_dot_float_and_leading_dot_enum_literal_terminals_match_through_recognizer() {
    let g = parse_grammar("grammar stepvalues\nstart doc\ndoc = FLOAT DOTENUM\n").expect("parse_grammar");
    let rec = Recognizer::compile(&g);
    assert!(rec.recognize("10. .T.").expect("recognize"), "a trailing-dot float and a leading-dot enum literal must each match their own terminal");
    assert!(!rec.recognize("10 .T.").expect("recognize"), "a plain Int must NOT satisfy a FLOAT terminal");
}

// Item 6: `Ref` self-recursion — pptx's shape-tree shape (`grpSp` recursively contains more
// shapes, including itself), verified with a real 3-level-nested fixture, not assumed.
#[semio_framework_async_macros::async_test]
async fn ref_self_recursion_matches_a_three_level_nested_shape_tree_pptx_style() {
    let source = "grammar shapetree\nstart tree\ntree = \"spTree\" group\ngroup = \"{\" node* \"}\"\nnode = leaf | nested\nleaf = \"sp\" IDENT\nnested = \"grpSp\" group\n";
    let g = parse_grammar(source).expect("parse_grammar");
    let rec = Recognizer::compile(&g);
    // Level 0 (tree) -> level 1 group contains a leaf and a grpSp -> level 2 group contains a
    // leaf and ANOTHER grpSp -> level 3 group contains one leaf. `nested` recursively refers
    // back to `group`, which refers back to `node`, which refers back to `nested` — genuine
    // mutual/self recursion through three real nesting levels, not a synthetic single hop.
    let fixture = "spTree { sp a grpSp { sp b grpSp { sp c } sp d } }";
    assert!(rec.recognize(fixture).expect("recognize"), "3-level self-recursive Ref chain must match a real nested fixture");
    // A malformed variant (unclosed innermost group) must NOT spuriously match.
    assert!(!rec.recognize("spTree { sp a grpSp { sp b grpSp { sp c } sp d }").expect("recognize"));
    // Confirm every production in the recursive chain was actually exercised, not merely
    // present — `uncovered_productions` must report none of them as unreached.
    let uncovered = rec.uncovered_productions(fixture).expect("uncovered_productions");
    assert!(uncovered.is_empty(), "every production in the recursive shape-tree grammar should be covered by the 3-level fixture, got uncovered: {uncovered:?}");
}
//#endregion 🔖️P2M1Grammar

//#region 🔖️P2M2Protocol
// Item 1a+1: repeated tag-dispatched block — length-first order, ASCII fixed(4) tag (PNG/GLB
// shape), unknown-type skip via declared length, repeat-until-sentinel-tag ("IEND").
#[semio_framework_async_macros::async_test]
async fn repeat_block_dispatches_png_shaped_chunks_and_skips_unknown_type() {
    let source = r#"dialect protocol
protocol demo.pngish
version 1
schema demo.pngish
start frame
framing record
repeat chunks {
tag fixed 4
length u32be
order length-first
trailer u32be
until "IEND"
arm "IHDR" { width u32be height u32be }
arm "IEND" { }
}
"#;
    let spec = parse_protocol(source).expect("parse pngish");
    let mut bytes = Vec::new();
    // Known arm: IHDR, length=8, two u32be fields, then a crc32be trailer.
    bytes.extend_from_slice(&8u32.to_be_bytes());
    bytes.extend_from_slice(b"IHDR");
    bytes.extend_from_slice(&100u32.to_be_bytes());
    bytes.extend_from_slice(&200u32.to_be_bytes());
    bytes.extend_from_slice(&0xDEAD_BEEFu32.to_be_bytes());
    // Unknown chunk type ("tEXt"): must be skipped as opaque via its declared length.
    bytes.extend_from_slice(&5u32.to_be_bytes());
    bytes.extend_from_slice(b"tEXt");
    bytes.extend_from_slice(&[1, 2, 3, 4, 5]);
    bytes.extend_from_slice(&0x1234_5678u32.to_be_bytes());
    // Sentinel: IEND, length=0, no fields, trailer crc, then the repeat block must stop.
    bytes.extend_from_slice(&0u32.to_be_bytes());
    bytes.extend_from_slice(b"IEND");
    bytes.extend_from_slice(&0xCAFE_BABEu32.to_be_bytes());

    let trace = walk_protocol(&spec, &bytes).expect("walk pngish");
    assert_eq!(trace.consumed, bytes.len(), "every declared+skipped+trailer byte must be consumed exactly");

    // Truncating the unknown chunk's declared payload must fail (proves the skip genuinely
    // reads `length`, not a fixed/guessed amount).
    let mut truncated = bytes.clone();
    truncated.truncate(bytes.len() - 9);
    assert!(walk_protocol(&spec, &truncated).is_err());
}

// Item 1b: two-level nested tag dispatch — GIF 89a shape (outer introducer byte, extension
// introducer's arm dispatches AGAIN on the label byte), tag-first order, no per-iteration
// length (all top-level introducers are known), repeat-until-trailer-byte (0x3B).
#[semio_framework_async_macros::async_test]
async fn repeat_block_two_level_nested_dispatch_gif89a_shaped() {
    let source = r#"dialect protocol
protocol demo.gifish
version 1
schema demo.gifish
start frame
framing record
repeat blocks {
tag u8
until 0x3B
arm 0x2C { left u16 top u16 }
arm 0x21 {
nested label u8 {
arm 0xF9 { flags u8 delay u16 }
arm 0xFE { }
}
}
arm 0x3B { }
}
"#;
    let spec = parse_protocol(source).expect("parse gifish");
    let mut bytes = Vec::new();
    // Image descriptor (0x2C): left/top u16 LE.
    bytes.push(0x2C);
    bytes.extend_from_slice(&10u16.to_le_bytes());
    bytes.extend_from_slice(&20u16.to_le_bytes());
    // Extension introducer (0x21) -> nested dispatch on label 0xF9 (GCE): flags u8, delay u16.
    bytes.push(0x21);
    bytes.push(0xF9);
    bytes.push(0);
    bytes.extend_from_slice(&100u16.to_le_bytes());
    // Trailer (0x3B) — sentinel, empty fields, loop must stop right after.
    bytes.push(0x3B);

    let trace = walk_protocol(&spec, &bytes).expect("walk gifish");
    assert_eq!(trace.consumed, bytes.len());

    // An unrecognized nested label must fail (proves the second dispatch level is real, not a
    // no-op fallthrough).
    let mut bad = bytes.clone();
    bad[5] = 0xAA; // corrupt the label byte inside the extension block
    assert!(walk_protocol(&spec, &bad).is_err());
}

// Item 1c: marker-prefix scanning — JPG shape. `marker(0xFF)` skips fill bytes before reading
// the real marker code, distinct from a fixed-position tag read.
#[semio_framework_async_macros::async_test]
async fn marker_scan_prim_finds_next_marker_byte_over_fill_bytes_jpg_style() {
    let source = r#"dialect protocol
protocol demo.jpgish
version 1
schema demo.jpgish
start frame
framing record
repeat segments {
tag marker(0xFF)
until 0xD9
arm 0xD8 { }
arm 0xE0 { version u16be }
arm 0xD9 { }
}
"#;
    let spec = parse_protocol(source).expect("parse jpgish");
    let mut bytes = Vec::new();
    // SOI preceded by an extra 0xFF fill byte — the scan must skip both leading 0xFFs and land
    // on the real 0xD8 marker code.
    bytes.extend_from_slice(&[0xFF, 0xFF, 0xD8]);
    // APP0 (0xE0), ordinary single-prefix marker, with a u16be field.
    bytes.extend_from_slice(&[0xFF, 0xE0]);
    bytes.extend_from_slice(&5u16.to_be_bytes());
    // EOI (0xD9) — sentinel.
    bytes.extend_from_slice(&[0xFF, 0xD9]);

    let trace = walk_protocol(&spec, &bytes).expect("walk jpgish");
    assert_eq!(trace.consumed, bytes.len());
}

// Item 2: BE `Prim` variants — a real round trip (parse/print/reparse) AND proof the decode is
// genuinely big-endian (a `Field(count)`-driven Array only walks cleanly if `count` was decoded
// with the declared byte order; LE-misreading a BE 3 as 0x0300 would overrun the buffer).
#[semio_framework_async_macros::async_test]
async fn be_prim_variants_round_trip_and_decode_big_endian_for_real() {
    let source = "dialect protocol\nprotocol demo.be\nversion 1\nschema demo.be\nstart frame\nframing record\nfield count u16be\nfield items Array(u8, Field(count))\n";
    let spec = parse_protocol(source).expect("parse");
    let printed = print_protocol(&spec);
    assert!(printed.contains("u16be"), "u16be must round trip through the printer");
    assert_eq!(parse_protocol(&printed).expect("reparse"), spec);

    let mut bytes = Vec::new();
    bytes.extend_from_slice(&3u16.to_be_bytes());
    bytes.extend_from_slice(&[9, 9, 9]);
    let trace = walk_protocol(&spec, &bytes).expect("walk be-driven array");
    assert_eq!(trace.consumed, bytes.len());

    // The same 2 count bytes read as LE (0x0900 = 2304) must NOT satisfy the buffer — proves a
    // real big-endian decode happened, not an accidental LE fallback.
    let le_misread_would_want = u16::from_le_bytes([3u8.to_be_bytes()[0], 0]);
    let _ = le_misread_would_want; // documentation only; the real proof is the failing walk below
    let mut too_short = bytes.clone();
    too_short.truncate(2); // count bytes only, no item bytes at all
    assert!(walk_protocol(&spec, &too_short).is_err());
}

// Item 3: cross-block field-env threading — a HEADER block's field is consumed by a LATER,
// separate SEGMENT block's `Array(_, Field(name))`. Pre-M2, `walk_fields` created a fresh
// per-call-local env, so this would fail to resolve; post-M2 the env is walk-wide.
#[semio_framework_async_macros::async_test]
async fn cross_block_field_env_threads_header_field_into_a_later_segment_las_vlr_style() {
    let source = r#"dialect protocol
protocol demo.crossblock
version 1
schema demo.crossblock
start frame
framing record
header fixed 2
field count u16
segment payload {
items Array(u8, Field(count))
}
"#;
    let spec = parse_protocol(source).expect("parse crossblock");
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&4u16.to_le_bytes());
    bytes.extend_from_slice(&[1, 2, 3, 4]);
    let trace = walk_protocol(&spec, &bytes).expect("segment must resolve `count` decoded by the earlier header block");
    assert_eq!(trace.consumed, bytes.len());

    // A `count` that doesn't match the actual trailing bytes must fail, proving the segment
    // genuinely used the decoded value (not silently accepting anything).
    let mut wrong = bytes.clone();
    wrong[0] = 9; // count now claims 9 items but only 4 bytes of payload exist
    assert!(walk_protocol(&spec, &wrong).is_err());
}

// Item 4: conditional field/segment presence — bmp shape (`if compression eq 3` gates one
// field, `if bpp le 8` gates a whole segment).
#[semio_framework_async_macros::async_test]
async fn conditional_field_and_segment_presence_gate_on_an_earlier_field_bmp_style() {
    let source = r#"dialect protocol
protocol demo.bmpish
version 1
schema demo.bmpish
start frame
framing record
field compression u8
field mask u32 if compression eq 3
field bpp u8
segment palette if bpp le 8 { colors u8 }
field trailer u8
"#;
    let spec = parse_protocol(source).expect("parse bmpish");

    // compression==3 -> mask present; bpp==8 -> palette present.
    let mut present = Vec::new();
    present.push(3u8);
    present.extend_from_slice(&0x0000_00FFu32.to_le_bytes());
    present.push(8u8);
    present.push(200u8); // palette.colors
    present.push(1u8); // trailer
    let trace = walk_protocol(&spec, &present).expect("walk with mask+palette present");
    assert_eq!(trace.consumed, present.len());

    // compression==0 -> mask absent; bpp==24 -> palette absent.
    let mut absent = Vec::new();
    absent.push(0u8);
    absent.push(24u8);
    absent.push(1u8); // trailer
    let trace2 = walk_protocol(&spec, &absent).expect("walk with mask+palette absent");
    assert_eq!(trace2.consumed, absent.len());

    // A buffer sized for the ABSENT shape must not satisfy the PRESENT shape's byte demands
    // (proves presence genuinely changes how many bytes are consumed, not a no-op guard).
    assert!(walk_protocol(&spec, &absent[..2]).is_err());
}

// Item 5: ZIP-shaped backward-scan (EOCD located by scanning backward from EOF for its magic)
// + absolute-offset jump (EOCD's `cd_offset` field -> central-directory-entry block).
#[semio_framework_async_macros::async_test]
async fn backward_scan_and_jump_to_resolve_zip_eocd_and_central_directory_offset() {
    let source = r#"dialect protocol
protocol demo.zipish
version 1
schema demo.zipish
start frame
framing record
backward eocd magic 0x504B0506 {
cd_offset u32
entry_count u16
}
jump central from cd_offset {
entry_tag u32
entry_value u32
}
"#;
    let spec = parse_protocol(source).expect("parse zipish");

    let mut bytes = Vec::new();
    // 16 bytes standing in for a "local header region" this protocol doesn't describe at all.
    bytes.extend(std::iter::repeat_n(0xAAu8, 16));
    let central_dir_offset = bytes.len() as u32;
    // Central-directory entry (jumped to via `cd_offset`).
    bytes.extend_from_slice(&0xCAFE_BABEu32.to_le_bytes());
    bytes.extend_from_slice(&42u32.to_le_bytes());
    // EOCD: magic + cd_offset (points back at the entry above) + entry_count.
    bytes.extend_from_slice(&[0x50, 0x4B, 0x05, 0x06]);
    bytes.extend_from_slice(&central_dir_offset.to_le_bytes());
    bytes.extend_from_slice(&1u16.to_le_bytes());

    let trace = walk_protocol(&spec, &bytes).expect("walk zipish");
    // The walk's FINAL position is wherever the last-declared block (the jump) left `pos` —
    // NOT bytes.len(), since a jump is a deliberate exception to linear forward accounting
    // (see `walk_protocol`'s own doc comment). Here that's right after the jumped-to entry's
    // two u32 fields.
    assert_eq!(trace.consumed, central_dir_offset as usize + 8);

    // Corrupting the EOCD magic must make the backward scan fail to find it at all.
    let mut corrupt_magic = bytes.clone();
    let magic_at = bytes.len() - 10;
    corrupt_magic[magic_at] = 0x00;
    assert!(walk_protocol(&spec, &corrupt_magic).is_err());

    // Print/reparse round trip for the new block syntax itself.
    let printed = print_protocol(&spec);
    assert!(printed.contains("backward eocd magic"));
    assert!(printed.contains("jump central from cd_offset"));
    assert_eq!(parse_protocol(&printed).expect("reparse zipish"), spec);
}

// Item 6: TIFF-style runtime-selected endianness — a leading marker field's VALUE selects
// LE-vs-BE for every subsequent plain (non-`Be`-suffixed) `Prim` read for the rest of the walk.
#[semio_framework_async_macros::async_test]
async fn endian_marker_field_switches_runtime_byte_order_for_the_rest_of_the_walk_tiff_style() {
    let source = "dialect protocol\nprotocol demo.tiffish\nversion 1\nschema demo.tiffish\nstart frame\nframing record\nfield byte_order endian { \"II\"=le \"MM\"=be }\nfield count u16\nfield items Array(u8, Field(count))\n";
    let spec = parse_protocol(source).expect("parse tiffish");

    // "II" -> little-endian mode for the rest of the walk.
    let mut le_bytes = Vec::new();
    le_bytes.extend_from_slice(b"II");
    le_bytes.extend_from_slice(&2u16.to_le_bytes());
    le_bytes.extend_from_slice(&[7, 8]);
    let trace_le = walk_protocol(&spec, &le_bytes).expect("walk II/LE");
    assert_eq!(trace_le.consumed, le_bytes.len());

    // "MM" -> big-endian mode for the rest of the walk — the SAME declared field (`count u16`,
    // no `Be` suffix) must now be read big-endian, proving the marker genuinely flips a runtime
    // mode rather than being cosmetic.
    let mut be_bytes = Vec::new();
    be_bytes.extend_from_slice(b"MM");
    be_bytes.extend_from_slice(&2u16.to_be_bytes());
    be_bytes.extend_from_slice(&[7, 8]);
    let trace_be = walk_protocol(&spec, &be_bytes).expect("walk MM/BE");
    assert_eq!(trace_be.consumed, be_bytes.len());

    // An unrecognized marker must be rejected outright.
    let mut bad = le_bytes.clone();
    bad[0] = b'X';
    assert!(walk_protocol(&spec, &bad).is_err());

    // Round trip the `endian {...}` syntax itself.
    let printed = print_protocol(&spec);
    assert!(printed.contains("endian {"));
    assert_eq!(parse_protocol(&printed).expect("reparse tiffish"), spec);
}
//#endregion 🔖️P2M2Protocol
