use super::*;

#[test]
fn table_loads_every_registered_language() {
    let t = table();
    assert_eq!(t.schema_version, 1);
    assert_eq!(t.registry.len(), 12);
    for name in &t.registry {
        assert!(language_by_name(name).is_some(), "{name} is missing from the table");
    }
    assert!(language_for_extension_unregistered(".json").is_some());
}

/// 🔗️ The table's kind map and the model crate's hand-written classifier are two statements of the
/// same rule; a divergence here means one of them was edited alone.
#[test]
fn definition_kind_map_agrees_with_the_model_crate() {
    for raw in table().definition_kind_map.keys() {
        assert_eq!(derive_definition_kind(raw), semio_framework_repo_model::derive_definition_kind(raw), "{raw}");
    }
    for raw in ["func", "fn", "class", "struct", "impl", "definition", ""] {
        assert_eq!(derive_definition_kind(raw), semio_framework_repo_model::derive_definition_kind(raw), "{raw}");
    }
}

#[test]
fn extension_lookup_follows_registry_order() {
    assert_eq!(language_for_path("a/b/🐹️.go").map(|l| l.name.as_str()), Some("go"));
    assert_eq!(language_for_path("🟦️.TSX").map(|l| l.name.as_str()), Some("typescript"));
    assert!(language_for_path("x.unknown").is_none());
}

#[test]
fn marker_sections_nest_and_close() {
    let src = "// #region 🔖️Outer\nlet a = 1;\n// #region 🔖️Inner\nlet b = 2;\n// #endregion 🔖️Inner\n// #endregion 🔖️Outer\n";
    let sections = parse_sections(src, "x.ts");
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0].name, "Outer");
    assert_eq!(sections[0].emoji, "🔖️");
    assert_eq!((sections[0].start_line, sections[0].end_line), (1, 6));
    assert_eq!(sections[0].children.len(), 1);
    assert_eq!(sections[0].children[0].name, "Inner");
    assert_eq!((sections[0].children[0].start_line, sections[0].children[0].end_line), (3, 5));
}

#[test]
fn unclosed_region_runs_to_the_end_of_the_file() {
    let src = "// #region 🔖️Open\nlet a = 1;\n";
    let sections = parse_sections(src, "x.ts");
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0].end_line, 3);
    assert_eq!(sections[0].end_index, src.len() as i64);
}

#[test]
fn stray_endregion_is_ignored() {
    let src = "// #endregion 🔖️Nothing\nlet a = 1;\n";
    assert!(parse_sections(src, "x.ts").is_empty());
}

#[test]
fn markdown_sections_nest_by_level() {
    let src = "# Title\ntext\n## Child\nmore\n# Second\n";
    let sections = parse_markdown_sections(src);
    assert_eq!(sections.len(), 2);
    assert_eq!(sections[0].name, "Title");
    assert_eq!(sections[0].children.len(), 1);
    assert_eq!(sections[0].children[0].name, "Child");
    assert_eq!(sections[1].name, "Second");
}

#[test]
fn json_sections_span_key_through_value() {
    let src = "{\n  \"a\": 1,\n  \"b\": { \"c\": \"x\" }\n}\n";
    let sections = parse_json_sections(src);
    assert_eq!(sections.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(), ["a", "b"]);
    assert_eq!(sections[1].children.iter().map(|s| s.name.as_str()).collect::<Vec<_>>(), ["c"]);
    assert!(sections.iter().all(|s| s.end_line > 0));
}

#[test]
fn typescript_definitions_carry_kinds() {
    let src = "export function alpha() {\n  return 1;\n}\nexport const beta = () => 2;\nexport interface Gamma {\n  a: number;\n}\n";
    let ranges = parse_definition_ranges(language_by_name("typescript").expect("registered"), &split_lines(src));
    let defs = parse_definitions(src, "x.ts");
    let seen: Vec<(&str, &str, i64, i64)> = ranges.iter().map(|r| (r.name.as_str(), r.kind.as_str(), r.start, r.end)).collect();
    assert_eq!(seen[0], ("alpha", "function", 1, 3));
    assert_eq!(seen[1].0, "beta");
    assert_eq!(seen[1].1, "function");
    assert_eq!(seen[2], ("Gamma", "interface", 5, 7));
    assert_eq!(defs[2].kind, DefinitionKind::Interface);
}

#[test]
fn go_method_receivers_do_not_become_the_name() {
    let src = "func (l *BaseLanguage) Name() string {\n\treturn l.name\n}\n";
    let defs = parse_definitions(src, "x.go");
    assert_eq!(defs.len(), 1);
    assert_eq!(defs[0].name, "Name");
    assert_eq!(parse_definition_ranges(language_by_name("go").expect("registered"), &split_lines(src))[0].kind, "func");
}

#[test]
fn python_definitions_close_on_dedent() {
    let src = "def outer():\n    x = 1\n    return x\n\ndef other():\n    pass\n";
    let defs = parse_definitions(src, "x.py");
    assert_eq!(defs[0].name, "outer");
    assert_eq!((defs[0].start_line, defs[0].end_line), (1, 4));
    assert_eq!(defs[1].name, "other");
}

#[test]
fn sql_definitions_use_the_multi_word_keyword() {
    let src = "CREATE TABLE IF NOT EXISTS public.items (\n  id INT\n);\n";
    let defs = parse_definitions(src, "x.sql");
    assert_eq!(defs[0].name, "public.items");
    assert_eq!(parse_definition_ranges(language_by_name("sql").expect("registered"), &split_lines(src))[0].kind, "CREATE TABLE");
}

#[test]
fn graphql_extend_type_is_an_interface() {
    let src = "extend type Query {\n  a: Int\n}\n";
    let defs = parse_definitions(src, "x.graphql");
    assert_eq!(defs[0].name, "Query");
    assert_eq!(defs[0].kind, DefinitionKind::Interface);
}

#[test]
fn ruby_definitions_close_on_end() {
    let src = "class Alpha\ndef beta\n  1\nend\nend\n";
    let defs = parse_definitions(src, "x.rb");
    assert_eq!(defs.iter().map(|d| (d.name.as_str(), d.start_line, d.end_line)).collect::<Vec<_>>(), [("Alpha", 1, 5), ("beta", 2, 4)]);
}

/// 🪤️ The Ruby definition pattern is anchored with no leading whitespace, exactly as the Go
/// original is, so an indented `def` is invisible to it. Pinned so a silent "fix" cannot slip in.
#[test]
fn ruby_ignores_indented_definitions() {
    let src = "class Alpha\n  def beta\n    1\n  end\nend\n";
    let defs = parse_definitions(src, "x.rb");
    assert_eq!(defs.iter().map(|d| (d.name.as_str(), d.start_line, d.end_line)).collect::<Vec<_>>(), [("Alpha", 1, 4)]);
}

#[test]
fn scope_ids_follow_the_grammar() {
    assert_eq!(build_scope_id("file", "a/b.ts", "", ""), "file:a/b.ts");
    assert_eq!(build_scope_id("section", "a/b.ts", "Outer.Inner", ""), "section:a/b.ts#Outer.Inner");
    assert_eq!(build_scope_id("definition", "a/b.ts", "Outer", "alpha"), "def:a/b.ts#Outer::alpha");
    assert_eq!(build_scope_id("definition", "a/b.ts", "", "alpha"), "def:a/b.ts#alpha");
}

#[test]
fn scopes_for_a_file_cover_sections_and_definitions() {
    let src = "// #region 🔖️Outer\nexport function alpha() {}\n// #endregion 🔖️Outer\n";
    let scopes = build_scopes_for_file("a/b.ts", src);
    let ids: Vec<&str> = scopes.iter().map(|s| s.id.as_str()).collect();
    assert_eq!(ids, ["file:a/b.ts", "section:a/b.ts#Outer", "def:a/b.ts#Outer::alpha"]);
}

#[test]
fn hydration_puts_definitions_on_the_deepest_section() {
    let src = "// #region 🔖️Outer\n// #region 🔖️Inner\nexport function alpha() {}\n// #endregion 🔖️Inner\nexport function beta() {}\n// #endregion 🔖️Outer\n";
    let sections = parse_sections(src, "x.ts");
    let defs = parse_definitions(src, "x.ts");
    let hydrated = hydrate_sections_with_definitions(&sections, &defs);
    assert_eq!(hydrated[0].definitions.iter().map(|d| d.name.as_str()).collect::<Vec<_>>(), ["beta"]);
    assert_eq!(hydrated[0].children[0].definitions.iter().map(|d| d.name.as_str()).collect::<Vec<_>>(), ["alpha"]);
}

#[test]
fn normalize_section_path_splits_on_both_separators() {
    assert_eq!(normalize_section_path("a/b#c//d"), ["a", "b", "c", "d"]);
    assert_eq!(normalize_section_path(""), Vec::<String>::new());
}

#[test]
fn header_formatting_round_trips_through_parsing() {
    let header = Header {
        file_id: "💻️test/file.ts".to_string(),
        file_uri: "repo://file/💻️test".to_string(),
        summary: "A test file".to_string(),
        contributors: "2025 Test User <test@test.com>".to_string(),
        license: "AGPL license text here".to_string(),
        requirements: "Some requirements".to_string(),
    };
    for name in ["typescript", "go", "python", "csharp", "rust", "ruby", "shell", "sql", "graphql"] {
        let lang = language_by_name(name).expect("registered");
        let text = format_header(lang, &header);
        assert!(text.contains("[💻️test/file.ts](repo://file/💻️test)"), "{name}");
        let parsed = parse_header(lang, &text).expect("header parses");
        assert_eq!(format_header(lang, &parsed), text, "{name}");
        assert_eq!(parsed, header, "{name}");
    }
    for name in ["markdown", "toml", "yaml"] {
        let lang = language_by_name(name).expect("registered");
        assert_eq!(format_header(lang, &header), "");
    }
}

#[test]
fn rust_sections_wrap_a_module() {
    let lang = language_by_name("rust").expect("registered");
    assert_eq!(format_section_start(lang, "Header"), "mod header { // 🔖️Header");
    assert_eq!(format_section_end(lang, "Header"), "} // 🔖️Header");
    assert_eq!(section_name_to_mod_name("Missing Hook Functions"), "missing_hook_functions");
    assert_eq!(section_name_to_mod_name("🔖️"), "section");
}

#[test]
fn case_insensitive_markers_are_accepted() {
    let src = "// #REGION 🔖️Loud\n// #EndRegion 🔖️Loud\n";
    let sections = parse_sections(src, "x.ts");
    assert_eq!(sections.len(), 1);
    assert_eq!(sections[0].name, "Loud");
}
