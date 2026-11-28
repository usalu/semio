//! 🧪️ Structural laws of the split link over the language-agnostic corpus `🔣️.json`; execution and the independent dynamic-loader oracle live in `🟦️.ts`.

use super::*;

struct Case {
    name: String,
    framework: String,
    plugin: String,
    exports: Vec<String>,
    error: Option<String>,
    missing: Vec<String>,
}

fn corpus() -> Vec<Case> {
    let root: serde_json::Value = serde_json::from_str(include_str!("🔣️.json")).unwrap();
    let strings = |v: &serde_json::Value| v.as_array().map(|a| a.iter().map(|s| s.as_str().unwrap().to_owned()).collect()).unwrap_or_default();
    root["cases"].as_array().unwrap().iter().map(|c| Case { name: c["name"].as_str().unwrap().to_owned(), framework: c["framework"].as_str().unwrap().to_owned(), plugin: c["plugin"].as_str().unwrap().to_owned(), exports: strings(&c["exports"]), error: c["error"].as_str().map(str::to_owned), missing: strings(&c["missing"]) }).collect()
}

#[test]
fn every_corpus_case_merges_or_fails_as_declared() {
    for case in corpus() {
        let framework = wat::parse_str(&case.framework).unwrap_or_else(|e| panic!("{}: framework wat: {e}", case.name));
        let plugin = wat::parse_str(&case.plugin).unwrap_or_else(|e| panic!("{}: plugin wat: {e}", case.name));
        match (merge(&framework, &plugin), &case.error) {
            (Ok((core, _)), None) => {
                let exports: Vec<String> = Parser::new(0).parse_all(&core).filter_map(|p| match p.unwrap() { Payload::ExportSection(r) => Some(r.into_iter().map(|e| e.unwrap().name.to_owned()).collect::<Vec<_>>()), _ => None }).flatten().collect();
                assert_eq!(exports, case.exports, "{}", case.name);
                assert!(core.starts_with(&framework[..8]), "{}", case.name);
            }
            (Err(error), Some(expected)) => {
                assert!(format!("{error:#}").contains(expected.as_str()), "{}: {error:#}", case.name);
                if !case.missing.is_empty() { assert_eq!(error.downcast_ref::<ImageLacksSymbols>().map(|e| e.missing.clone()), Some(case.missing.clone()), "{}", case.name); }
            }
            (Ok(_), Some(expected)) => panic!("{}: merged although it must fail with {expected}", case.name),
            (Err(error), None) => panic!("{}: {error:#}", case.name),
        }
    }
}

#[test]
fn framework_code_is_copied_byte_for_byte() {
    let case = &corpus()[0];
    let framework = wat::parse_str(&case.framework).unwrap();
    let (core, _) = merge(&framework, &wat::parse_str(&case.plugin).unwrap()).unwrap();
    let bodies = |bytes: &[u8]| Parser::new(0).parse_all(bytes).filter_map(|p| match p.unwrap() { Payload::CodeSectionEntry(b) => Some(bytes[span(b.range())].to_vec()), _ => None }).collect::<Vec<_>>();
    let (before, after) = (bodies(&framework), bodies(&core));
    assert_eq!(&after[..before.len()], &before[..]);
}

#[test]
fn image_exports_lists_every_bound_symbol_once() {
    let case = &corpus()[0];
    assert_eq!(image_exports(&wat::parse_str(&case.plugin).unwrap()).unwrap(), ["--export=_RNvCs0_2fw3add", "--export=_RNvCs0_2fw6double", "--export=_RNvCs0_2fw7COUNTER", "--export=__stack_pointer", "--export=cabi_realloc"]);
}

#[test]
fn image_imports_rust_declares_every_non_env_import_once() {
    let case = corpus().into_iter().find(|c| c.name.starts_with("binds canonical intrinsic imports")).unwrap();
    let rust = image_imports_rust(&wat::parse_str(&case.plugin).unwrap()).unwrap();
    assert!(rust.contains("#[link(wasm_import_module = \"[export]semio:x/y@1.0.0\")]\nunsafe extern \"C\" { #[link_name = \"[task-return]r\"] fn __semio_image_import_0(p0: i32); }"), "{rust}");
    assert!(rust.contains("pub extern \"C\" fn __semio_image_imports() -> usize { __semio_image_import_0 as usize }"), "{rust}");
}
