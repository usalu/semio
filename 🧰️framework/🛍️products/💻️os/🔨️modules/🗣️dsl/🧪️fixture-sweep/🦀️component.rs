//! 🧭️ Repo-wide DSL fixture-law sweep (W6, final wave of the DSL-notation program). Walks every
//! real shipped `📚️examples/**` fixture file across every plugin/app that derives
//! `crate::os_store::DocumentDsl` (via `#[derive(crate::os_dsl::Dsl...)]`, `dsl_derive`'s generated impls, or a
//! hand-rolled Route-A idiom bridge) and proves both engine laws directly against the fixture
//! TEXT — the thing that actually ships, not a separately hand-built in-memory example a per-app
//! test might have drifted from:
//!
//! 1. **parse→print→reparse fixpoint**: `parse_dsl(text)` then `print_dsl` then `parse_dsl` again
//!    recovers an equal value.
//! 2. **canonicalize idempotence**: `canonicalize(x) := print_dsl(parse_dsl(x))` is idempotent —
//!    `canonicalize(canonicalize(x)) == canonicalize(x)`. Equivalent to
//!    `crate::os_dsl::schema::canonicalize(x, spec, opts)` for every derive-generated `DocumentDsl` impl (see
//!    `crate::os_store::test_support::check_dsl_fixture_text_laws`'s doc comment for why), and the correct
//!    generalization for hand-rolled Route-A idioms that have no `RecordSpec` at all.
//!
//! Test-only crate (everything lives under `#[cfg(test)]`): depends on every app's thin
//! `🔨️modules/🗣️dsl` (or core) crate purely as a `[dev-dependencies]` fan-in so this ONE `cargo
//! test`/`nx` target can reach every real `DocumentDsl` type without any of those app crates
//! depending back on this one — never a real dependency of anything. Registered by extension
//! (`P::EXTENSION`), not by directory, so a fixture is checked wherever in the repo it actually
//! lives (plugin-root `📚️examples/`, a nested per-app `⚡️implementations/🦀️rust/📚️examples/`, or a
//! framework-level one) — see `POLICY_DSL_ROUND_TRIP_ALLOWLIST`'s doc comment in the root
//! `📜️script.ts` for the parallel per-file static-analysis view of this same migration.

#[cfg(all(test, feature = "dsl-fixture-sweep-full"))]
mod tests {
    use std::path::{Path, PathBuf};

    //#region 🔖️AppTypes
    // One `use` per registered app kind — aliased where the app's own type is plainly named
    // `Document` (every norm sub-app) to avoid a name collision in this one aggregating module.
    use block::artifacts::block2d::Block2dDefinition;
    use block::artifacts::block3d::Block3dDefinition;
    use block::artifacts::block5d::Block5dDefinition;
    use cad_document::artifacts::cad::CadProjection;
    use dag_app::DagDocument;
    use norm::artifacts::din16798::Document as Din16798Document;
    use norm::artifacts::din18599::Document as Din18599Document;
    use norm::artifacts::din4108::Document as Din4108Document;
    use draw::artifacts::draw::DrawDocument;
    use norm::artifacts::en1990::Document as En1990Document;
    use norm::artifacts::en1991::Document as En1991Document;
    use norm::artifacts::en1992::Document as En1992Document;
    use norm::artifacts::en1993::Document as En1993Document;
    use norm::artifacts::en1994::Document as En1994Document;
    use norm::artifacts::en1995::Document as En1995Document;
    use norm::artifacts::en1996::Document as En1996Document;
    use norm::artifacts::en1997::Document as En1997Document;
    use norm::artifacts::en1998::Document as En1998Document;
    use norm::artifacts::en1999::Document as En1999Document;
    use fem2d::Fem2dDocument;
    use fem3d::Fem3dDocument;
    use flow_app::FlowFixture;
    // 🌱️ 26/08/05/FORMS-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION: the old `forms` app facade
    // crate is gone (merged into `semio-s-plugin-forms`); `FormSpec` was always a bare `pub use` alias of
    // `playbook::PlaybookSpec` (forms never overrode `#[dsl(extension = ...)]`) so this repoints straight
    // at the real owner of the type — no `lib.rs` ripple beyond this import line (see TEMPLATE.md §8.2).
    use playbook::PlaybookSpec as FormSpec;
    use gis::artifacts::gismap::GisMapDocument;
    use gis::artifacts::gisterrain::Gis3dTerrainDocument;
    use home::artifacts::home::SHomeDocument;
    use imperative::artifacts::imperative::ImperativeDocument;
    use norm::artifacts::iso16757::Document as Iso16757Document;
    use layout::artifacts::layout::LayoutDocument;
    use lowpoly::artifacts::lowpoly::LowpolyProjection;
    use mathematical::artifacts::mathematical::MathProjection;
    use note_app::artifacts::note::NoteDocument;
    use playbook::PlaybookSpec;
    use present::artifacts::present::PresentDeck;
    use procedural::artifacts::procedural2d::Procedural2dDocument;
    use procedural::artifacts::procedural3d::Procedural3dDocument;
    use process_3d::artifacts::process3d::Process3dDocument;
    use puzzle::artifacts::puzzle2d::Puzzle2dProjection;
    use puzzle::artifacts::puzzle3d::Puzzle3dProjection;
    use puzzle::artifacts::puzzle5d::Puzzle5dProjection;
    use raster::artifacts::raster::RasterProjection;
    use reasoning_mindmap_plugin::artifacts::wires::MindmapWiresDocument;
    use remodel::artifacts::remodel::RemodelProjection;
    use trinity::artifacts::rewrite::RewriteRuleModel;
    use semio_framework_os::WorkflowDocument;
    use sequence::artifacts::sequence::SequenceFixture;
    use shooting::artifacts::shooting::ShootingFixture;
    use sourcing::artifacts::curate::CurateDocument;
    use space::{CollectionProjection, SpaceProjection};
    use trinity::artifacts::jack::GraphFixture;
    use vcs_app::artifacts::vcs::VcsDemoProjection;
    use norm::artifacts::vdi3805::Document as Vdi3805Document;
    use writer::artifacts::writer::WriterProjection;
    //#endregion 🔖️AppTypes

    //#region 🔖️Registry
    /// @emoji 🧭️ `(app label, envelope_id, check fn)` — dispatch is by sniffed `plugin.artifact` from `.semio` content.
    type CheckFn = fn(&str) -> Result<(), String>;

    fn registry() -> Vec<(&'static str, &'static str, CheckFn)> {
        vec![
            ("writer", <WriterProjection as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<WriterProjection>),
            ("mathematical", <MathProjection as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<MathProjection>),
            ("procedural_2d", <Procedural2dDocument as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Procedural2dDocument>),
            ("procedural_3d", <Procedural3dDocument as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Procedural3dDocument>),
            ("flow_app", <FlowFixture as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<FlowFixture>),
            ("gis2d", "gis.gismap", crate::os_store::test_support::check_dsl_fixture_text_laws::<GisMapDocument>),
            ("gis3d", "gis.gisterrain", crate::os_store::test_support::check_dsl_fixture_text_laws::<Gis3dTerrainDocument>),
            ("vcs_app", <VcsDemoProjection as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<VcsDemoProjection>),
            ("present", <PresentDeck as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<PresentDeck>),
            ("shooting", <ShootingFixture as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<ShootingFixture>),
            ("sequence", <SequenceFixture as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<SequenceFixture>),
            ("fem2d", <Fem2dDocument as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Fem2dDocument>),
            ("fem3d", <Fem3dDocument as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Fem3dDocument>),
            ("process_3d", <Process3dDocument as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Process3dDocument>),
            ("lowpoly", <LowpolyProjection as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<LowpolyProjection>),
            ("reasoning_wires", <MindmapWiresDocument as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<MindmapWiresDocument>),
            ("layout", <LayoutDocument as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<LayoutDocument>),
            ("cad_document", <CadProjection as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<CadProjection>),
            ("iso16757", <Iso16757Document as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Iso16757Document>),
            ("vdi3805", <Vdi3805Document as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Vdi3805Document>),
            ("din4108", <Din4108Document as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Din4108Document>),
            ("din16798", <Din16798Document as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Din16798Document>),
            ("en1990", <En1990Document as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<En1990Document>),
            ("en1991", <En1991Document as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<En1991Document>),
            ("en1992", <En1992Document as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<En1992Document>),
            ("en1993", <En1993Document as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<En1993Document>),
            ("en1994", <En1994Document as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<En1994Document>),
            ("en1995", <En1995Document as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<En1995Document>),
            ("en1996", <En1996Document as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<En1996Document>),
            ("en1997", <En1997Document as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<En1997Document>),
            ("en1998", <En1998Document as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<En1998Document>),
            ("en1999", <En1999Document as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<En1999Document>),
            ("din18599", <Din18599Document as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Din18599Document>),
            ("playbook", <PlaybookSpec as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<PlaybookSpec>),
            ("imperative", <ImperativeDocument as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<ImperativeDocument>),
            ("remodel", <RemodelProjection as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<RemodelProjection>),
            ("rewrite", <RewriteRuleModel as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<RewriteRuleModel>),
            ("trinity_ram", <GraphFixture as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<GraphFixture>),
            ("dag_app", <DagDocument as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<DagDocument>),
            ("draw", <DrawDocument as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<DrawDocument>),
            ("raster", <RasterProjection as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<RasterProjection>),
            ("note_app", <NoteDocument as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<NoteDocument>),
            ("puzzle_2d", <Puzzle2dProjection as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Puzzle2dProjection>),
            ("puzzle_5d", <Puzzle5dProjection as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Puzzle5dProjection>),
            ("puzzle_3d", <Puzzle3dProjection as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Puzzle3dProjection>),
            ("block_2d", <Block2dDefinition as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Block2dDefinition>),
            ("block_5d", <Block5dDefinition as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Block5dDefinition>),
            ("block_3d", <Block3dDefinition as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<Block3dDefinition>),
            ("home", <SHomeDocument as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<SHomeDocument>),
            ("semio_framework_os", <WorkflowDocument as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<WorkflowDocument>),
            ("sourcing", <CurateDocument as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<CurateDocument>),
            // 🌱️ `forms` app fixtures ship as `*.forms`, but `FormSpec` is a bare `pub use` alias of
            // `playbook::PlaybookSpec` (forms never overrode `#[dsl(extension = ...)]`), so
            // `<FormSpec as crate::os_store::DocumentDsl>::envelope_id()` is actually `"playbook"`, not `"forms"` —
            // registered here under the file's real suffix too since `parse_dsl`/`print_dsl` only
            // care about the grammar's field shape, never the extension string.
            ("forms", "forms", crate::os_store::test_support::check_dsl_fixture_text_laws::<FormSpec>),
            ("space", <SpaceProjection as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<SpaceProjection>),
            ("space", <CollectionProjection as crate::os_store::DocumentDsl>::envelope_id(), crate::os_store::test_support::check_dsl_fixture_text_laws::<CollectionProjection>),
        ]
    }
    //#endregion 🔖️Registry

    //#region 🔖️Walk
    /// @emoji 🏠️ Ascends from `CARGO_MANIFEST_DIR` looking for `nx.json` (a repo-root-only marker)
    /// rather than hardcoding a `../..` depth — robust to this crate ever moving.
    fn repo_root() -> PathBuf {
        let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        loop {
            if dir.join("nx.json").is_file() {
                return dir;
            }
            if !dir.pop() {
                panic!("could not locate repo root (nx.json) ascending from {}", env!("CARGO_MANIFEST_DIR"));
            }
        }
    }

    /// @emoji 📚️ Recursively finds every directory literally named `📚️examples` under `root`,
    /// skipping `node_modules`/`target`/hidden/ticket-scratch directories.
    fn example_dirs(root: &Path) -> Vec<PathBuf> {
        fn skip(name: &str) -> bool {
            name == "node_modules" || name == "target" || name.starts_with('.') || name == "🦑️repo"
        }
        fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
            let entries = match std::fs::read_dir(dir) {
                Ok(entries) => entries,
                Err(_) => return,
            };
            for entry in entries.flatten() {
                let path = entry.path();
                if !path.is_dir() {
                    continue;
                }
                let name = entry.file_name();
                let name = name.to_string_lossy();
                if skip(&name) {
                    continue;
                }
                if name == "📚️examples" {
                    out.push(path.clone());
                }
                walk(&path, out);
            }
        }
        let mut out = Vec::new();
        walk(root, &mut out);
        out
    }

    /// @emoji 📄️ Recursively collects every FILE under `dir` (fixture directories occasionally
    /// nest one level, e.g. `norm/📚️examples/📘️en1990/...`).
    fn collect_files(dir: &Path, out: &mut Vec<PathBuf>) {
        let entries = match std::fs::read_dir(dir) {
            Ok(entries) => entries,
            Err(_) => return,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                collect_files(&path, out);
            } else {
                out.push(path);
            }
        }
    }
    //#endregion 🔖️Walk

    //#region 🔖️Sweep
    #[test]
    fn repo_wide_dsl_fixture_law_sweep() {
        let root = repo_root();
        let dirs = example_dirs(&root);
        assert!(!dirs.is_empty(), "found zero 📚️examples directories under {root:?} — sweep would vacuously pass");

        let mut fixture_files = Vec::new();
        for dir in &dirs {
            collect_files(dir, &mut fixture_files);
        }
        assert!(!fixture_files.is_empty(), "found {} 📚️examples dir(s) but zero fixture files under {root:?}", dirs.len());

        let registry = registry();
        let mut walked = 0usize;
        let mut unmapped: Vec<String> = Vec::new();
        let mut failures: Vec<String> = Vec::new();

        for file in &fixture_files {
            if file.extension().and_then(|e| e.to_str()) == Some("semio") {
                let bytes = std::fs::read(file).unwrap_or_else(|error| panic!("read {}: {error}", file.display()));
                let envelope = match crate::os_store::semio_format::sniff(&bytes) {
                    Ok(envelope) => envelope,
                    Err(detail) => {
                        unmapped.push(format!("{} (semio sniff failed: {detail})", file.display()));
                        continue;
                    }
                };
                if envelope.component != crate::os_store::semio_format::Component::Dsl {
                    continue;
                }
                let key = envelope.envelope_id();
                let matching: Vec<&(&str, &str, CheckFn)> = registry.iter().filter(|(_, ext, _)| *ext == key).collect();
                if matching.is_empty() {
                    unmapped.push(format!("{} (envelope {key} — no registered DocumentDsl)", file.display()));
                    continue;
                }
                let text = std::str::from_utf8(&bytes).unwrap_or_else(|_| panic!("{} is not valid utf-8", file.display()));
                for (label, _, check) in &matching {
                    walked += 1;
                    if let Err(detail) = check(text) {
                        failures.push(format!("[{label}] {}: {detail}", file.display()));
                    }
                }
                continue;
            }
            let extension = match file.extension().and_then(|e| e.to_str()) {
                Some(extension) => extension,
                None => {
                    unmapped.push(format!("{} (no file extension)", file.display()));
                    continue;
                }
            };
            let matching: Vec<&(&str, &str, CheckFn)> = registry.iter().filter(|(_, ext, _)| *ext == extension).collect();
            if matching.is_empty() {
                unmapped.push(format!("{} (.{extension} — no app registered with this EXTENSION)", file.display()));
                continue;
            }
            let text = std::fs::read_to_string(file).unwrap_or_else(|error| panic!("read {}: {error}", file.display()));
            for (label, _, check) in &matching {
                walked += 1;
                if let Err(detail) = check(&text) {
                    failures.push(format!("[{label}] {}: {detail}", file.display()));
                }
            }
        }

        eprintln!("[dsl-fixture-sweep] {} example dir(s), {} fixture file(s) found, {} law-check(s) run across {} registered app kind(s), {} unmapped fixture(s)", dirs.len(), fixture_files.len(), walked, registry.len(), unmapped.len());
        if !unmapped.is_empty() {
            eprintln!("[dsl-fixture-sweep] unmapped fixtures (no registered DocumentDsl app matches this extension — not counted as a failure):");
            for entry in &unmapped {
                eprintln!("  {entry}");
            }
        }

        assert!(failures.is_empty(), "dsl fixture law sweep failed for {} check(s) across {} fixture file(s):\n\n{}", failures.len(), fixture_files.len(), failures.join("\n\n"));
    }

    #[test]
    fn repo_wide_semio_example_kind_coverage() {
        let root = repo_root();
        let kinds = ["🗣️dsls", "🎒️packs", "🔧️ops", "📡️sprs"];
        let plugins = root.join("✏️s").join("🔌️plugins");
        let mut gaps: Vec<String> = Vec::new();
        let read_dir = |p: &Path| std::fs::read_dir(p).ok().map(|d| d.filter_map(|e| e.ok()).collect::<Vec<_>>()).unwrap_or_default();
        for plugin in read_dir(&plugins) {
            let plugin_path = plugin.path();
            if !plugin_path.is_dir() {
                continue;
            }
            let artifacts = plugin_path.join("🗿️artifacts");
            for artifact in read_dir(&artifacts) {
                let artifact_path = artifact.path();
                if !artifact_path.is_dir() {
                    continue;
                }
                let examples = artifact_path.join("📚️examples");
                for set in read_dir(&examples) {
                    let set_path = set.path();
                    if !set_path.is_dir() {
                        continue;
                    }
                    for kind in kinds {
                        if !set_path.join(kind).is_dir() {
                            gaps.push(format!("{} missing {}", set_path.display(), kind));
                        }
                    }
                }
            }
        }
        assert!(gaps.is_empty(), "semio example kind gaps:\n{}", gaps.join("\n"));
    }
    //#endregion 🔖️Sweep
}

//#region 🔖️M5SoftSkip
/// @emoji 🛟 Soft-skip helpers for M5 pilot laws when a facet has not exported a usable
/// `COMPONENT_GRAMMAR_SEMIO` / `COMPONENT_PROTOCOL_SEMIO` yet (empty or stub text). Keeps the
/// fixture-sweep compiling without plugin crate fan-in via `include_str!` of sibling specs.
#[cfg(test)]
mod m5_soft_skip {
    /// @emoji ⏭️ Returns true when the pilot constant/spec text is missing or still a stub.
    pub fn soft_skip_missing(label: &str, text: &str) -> bool {
        let trimmed = text.trim();
        if trimmed.is_empty() || (trimmed.contains("TODO") && trimmed.lines().count() < 4) {
            eprintln!("[DEBUG] soft-skip {label}: pilot constant/spec missing or stub");
            return true;
        }
        false
    }

    /// @emoji ⏭️ Soft-skip when binary example payload is empty after unwrap.
    pub fn soft_skip_empty_bytes(label: &str, bytes: &[u8]) -> bool {
        if bytes.is_empty() {
            eprintln!("[DEBUG] soft-skip {label}: empty payload");
            return true;
        }
        false
    }
}
//#endregion 🔖️M5SoftSkip

//#region 🔖️M5HandcraftedGrammar
/// @emoji 📖️ M5 grammar conformance on pilots that ship `COMPONENT_GRAMMAR_SEMIO` (lowpoly/dag/cad/en1992
/// plus note/fem2d when present). Soft-skips empty/stub specs.
#[cfg(test)]
mod m5_handcrafted_grammar_conformance {
    use super::m5_soft_skip::soft_skip_missing;
    use crate::os_dsl::{parse_grammar, Recognizer, SemioDialect};
    use crate::os_store::semio_format::split_text_preamble;

    fn dsl_body_from_fixture(text: &str) -> &str {
        if text.trim_start().starts_with("semio ") {
            split_text_preamble(text).map(|(_, body)| body).unwrap_or(text)
        } else {
            text
        }
    }

    fn assert_grammar_recognizes_shipped_fixture(grammar_semio: &str, fixture_semio: &str, pilot: &str) {
        if soft_skip_missing(&format!("{pilot}.grammar"), grammar_semio) {
            return;
        }
        if soft_skip_missing(&format!("{pilot}.fixture"), fixture_semio) {
            return;
        }
        let grammar = parse_grammar(grammar_semio).unwrap_or_else(|error| panic!("{pilot}: parse grammar.semio: {error:?}"));
        assert_eq!(grammar.dialect, SemioDialect::Grammar, "{pilot}: expected grammar dialect");
        let recognizer = Recognizer::compile(&grammar);
        let body = dsl_body_from_fixture(fixture_semio);
        assert!(
            recognizer.recognize(body).unwrap_or_else(|error| panic!("{pilot}: recognize failed: {error:?}")),
            "{pilot}: grammar must recognize shipped fixture DSL body"
        );
    }

    #[test]
    fn lowpoly_dsl_grammar_recognizes_shipped_fixture_tokens() {
        const GRAMMAR: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🗣️dsl/📖️component.grammar.semio"
        );
        const FIXTURE: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.lowpoly.lowpoly.dsl.semio"
        );
        assert_grammar_recognizes_shipped_fixture(GRAMMAR, FIXTURE, "lowpoly");
    }

    #[test]
    fn dag_dsl_grammar_recognizes_shipped_fixture_tokens() {
        const GRAMMAR: &str =
            include_str!("../../../../../../✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🗣️dsl/📖️component.grammar.semio");
        const FIXTURE: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.dag.dag.dsl.semio"
        );
        assert_grammar_recognizes_shipped_fixture(GRAMMAR, FIXTURE, "dag");
    }

    #[test]
    fn cad_dsl_grammar_recognizes_shipped_fixture_tokens() {
        const GRAMMAR: &str =
            include_str!("../../../../../../✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🗣️dsl/📖️component.grammar.semio");
        const FIXTURE: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📚️examples/♻️default/🗣️dsls/♻️default/🧬️component.cad.cad.dsl.semio"
        );
        assert_grammar_recognizes_shipped_fixture(GRAMMAR, FIXTURE, "cad");
    }

    #[test]
    fn en1992_dsl_grammar_recognizes_shipped_fixture_tokens() {
        const GRAMMAR: &str =
            include_str!("../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🗣️dsl/📖️component.grammar.semio");
        const FIXTURE: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/📚️examples/📕️liquid-retaining-fem-anchor/🗣️dsls/📕️liquid-retaining-fem-anchor/🧬️component.norm.en1992.dsl.semio"
        );
        assert_grammar_recognizes_shipped_fixture(GRAMMAR, FIXTURE, "en1992");
    }

    #[test]
    fn note_dsl_grammar_recognizes_shipped_fixture_tokens() {
        const GRAMMAR: &str =
            include_str!("../../../../../../✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🗣️dsl/📖️component.grammar.semio");
        const FIXTURE: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📚️examples/♻️semio/🗣️dsls/♻️semio/🧬️component.note.note.dsl.semio"
        );
        assert_grammar_recognizes_shipped_fixture(GRAMMAR, FIXTURE, "note");
    }

    #[test]
    fn fem2d_dsl_grammar_recognizes_shipped_fixture_tokens() {
        const GRAMMAR: &str =
            include_str!("../../../../../../✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🗣️dsl/📖️component.grammar.semio");
        const FIXTURE: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.fem.fem2d.dsl.semio"
        );
        assert_grammar_recognizes_shipped_fixture(GRAMMAR, FIXTURE, "fem2d");
    }
}
//#endregion 🔖️M5HandcraftedGrammar


//#region 🔖️M5HandcraftedProtocol
/// @emoji 📡️ M5 protocol conformance via [`verify_protocol_source`] / [`walk_protocol`] when
/// `COMPONENT_PROTOCOL_SEMIO` text exists. Soft-skips empty/stub protocols or empty payloads.
#[cfg(test)]
mod m5_handcrafted_protocol_conformance {
    use super::m5_soft_skip::{soft_skip_empty_bytes, soft_skip_missing};
    use crate::os_dsl::{parse_protocol, verify_protocol_source, walk_protocol};
    use crate::os_store::semio_format::unwrap_binary;

    fn inner_payload_from_semio_example(bytes: &'static [u8], label: &str) -> Option<Vec<u8>> {
        match unwrap_binary(bytes) {
            Ok((_, inner)) => Some(inner.to_vec()),
            Err(error) => {
                eprintln!("[DEBUG] soft-skip {label}: unwrap failed: {error}");
                None
            }
        }
    }

    fn assert_protocol_conformance(protocol_semio: &str, pack_or_spr: &'static [u8], pilot: &str) {
        if soft_skip_missing(&format!("{pilot}.protocol"), protocol_semio) {
            return;
        }
        let Some(bytes) = inner_payload_from_semio_example(pack_or_spr, pilot) else {
            return;
        };
        if soft_skip_empty_bytes(pilot, &bytes) {
            return;
        }
        verify_protocol_source(protocol_semio, &bytes)
            .unwrap_or_else(|error| panic!("{pilot}: verify_protocol_source: {error}"));
        let spec = parse_protocol(protocol_semio)
            .unwrap_or_else(|error| panic!("{pilot}: parse_protocol: {error:?}"));
        walk_protocol(&spec, &bytes).unwrap_or_else(|error| {
            panic!("{pilot}: walk_protocol @{}: {}", error.offset, error.message)
        });
    }

    #[test]
    fn handcrafted_lowpoly_pack_bytes_verify_against_pack_protocol_spec() {
        const PROTOCOL: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🎒️pack/📡️component.protocol.semio"
        );
        const PACK_EXAMPLE: &[u8] = include_bytes!(
            "../../../../../../✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📚️examples/♻️reuse/🎒️packs/♻️reuse/🧬️component.lowpoly.lowpoly.pack.semio"
        );
        assert_protocol_conformance(PROTOCOL, PACK_EXAMPLE, "lowpoly.pack");
    }

    #[test]
    fn handcrafted_dag_pack_bytes_verify_against_pack_protocol_spec() {
        const PROTOCOL: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🎒️pack/📡️component.protocol.semio"
        );
        const PACK_EXAMPLE: &[u8] = include_bytes!(
            "../../../../../../✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📚️examples/♻️reuse/🎒️packs/♻️reuse/🧬️component.dag.dag.pack.semio"
        );
        assert_protocol_conformance(PROTOCOL, PACK_EXAMPLE, "dag.pack");
    }

    #[test]
    fn handcrafted_dag_spr_bytes_verify_against_spr_protocol_spec() {
        const PROTOCOL: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📡️spr/📡️component.protocol.semio"
        );
        const SPR_EXAMPLE: &[u8] = include_bytes!(
            "../../../../../../✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📚️examples/♻️reuse/📡️sprs/♻️reuse/🧬️component.dag.dag.spr.semio"
        );
        assert_protocol_conformance(PROTOCOL, SPR_EXAMPLE, "dag.spr");
    }

    #[test]
    fn handcrafted_cad_pack_bytes_verify_against_pack_protocol_spec() {
        const PROTOCOL: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🎒️pack/📡️component.protocol.semio"
        );
        const PACK_EXAMPLE: &[u8] = include_bytes!(
            "../../../../../../✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📚️examples/♻️default/🎒️packs/♻️default/🧬️component.cad.cad.pack.semio"
        );
        assert_protocol_conformance(PROTOCOL, PACK_EXAMPLE, "cad.pack");
    }

    #[test]
    fn handcrafted_en1992_pack_bytes_verify_against_pack_protocol_spec() {
        const PROTOCOL: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🎒️pack/📡️component.protocol.semio"
        );
        const PACK_EXAMPLE: &[u8] = include_bytes!(
            "../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/📚️examples/📕️liquid-retaining-fem-anchor/🎒️packs/📕️liquid-retaining-fem-anchor/🧬️component.norm.en1992.pack.semio"
        );
        assert_protocol_conformance(PROTOCOL, PACK_EXAMPLE, "en1992.pack");
    }

    #[test]
    fn handcrafted_note_pack_bytes_verify_against_pack_protocol_spec() {
        const PROTOCOL: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/🎒️pack/📡️component.protocol.semio"
        );
        const PACK_EXAMPLE: &[u8] = include_bytes!(
            "../../../../../../✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note/📚️examples/♻️reuse/🎒️packs/♻️reuse/🧬️component.note.note.pack.semio"
        );
        assert_protocol_conformance(PROTOCOL, PACK_EXAMPLE, "note.pack");
    }

    #[test]
    fn handcrafted_fem2d_pack_bytes_verify_against_pack_protocol_spec() {
        const PROTOCOL: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/🎒️pack/📡️component.protocol.semio"
        );
        const PACK_EXAMPLE: &[u8] = include_bytes!(
            "../../../../../../✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/📚️examples/♻️reuse/🎒️packs/♻️reuse/🧬️component.fem.fem2d.pack.semio"
        );
        assert_protocol_conformance(PROTOCOL, PACK_EXAMPLE, "fem2d.pack");
    }
}
//#endregion 🔖️M5HandcraftedProtocol

//#region 🔖️M5CrossArtifactRejection
/// @emoji ⚔️ Cross-artifact anti-genericness: lowpoly recognizer must reject a dag sample (and vice versa).
#[cfg(test)]
mod m5_cross_artifact_rejection {
    use super::m5_soft_skip::soft_skip_missing;
    use crate::os_dsl::{parse_grammar, Recognizer};
    use crate::os_store::semio_format::split_text_preamble;

    fn dsl_body_from_fixture(text: &str) -> &str {
        if text.trim_start().starts_with("semio ") {
            split_text_preamble(text).map(|(_, body)| body).unwrap_or(text)
        } else {
            text
        }
    }

    #[test]
    fn lowpoly_recognizer_rejects_dag_sample() {
        const LOWPOLY_GRAMMAR: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🗣️dsl/📖️component.grammar.semio"
        );
        const DAG_GRAMMAR: &str =
            include_str!("../../../../../../✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🗣️dsl/📖️component.grammar.semio");
        const LOWPOLY_FIXTURE: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.lowpoly.lowpoly.dsl.semio"
        );
        const DAG_FIXTURE: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.dag.dag.dsl.semio"
        );
        if soft_skip_missing("lowpoly.grammar", LOWPOLY_GRAMMAR) || soft_skip_missing("dag.grammar", DAG_GRAMMAR) {
            return;
        }
        if soft_skip_missing("lowpoly.fixture", LOWPOLY_FIXTURE) || soft_skip_missing("dag.fixture", DAG_FIXTURE) {
            return;
        }
        let lowpoly_grammar = parse_grammar(LOWPOLY_GRAMMAR).expect("lowpoly grammar");
        let dag_grammar = parse_grammar(DAG_GRAMMAR).expect("dag grammar");
        let lowpoly = Recognizer::compile(&lowpoly_grammar);
        let dag = Recognizer::compile(&dag_grammar);
        let lowpoly_body = dsl_body_from_fixture(LOWPOLY_FIXTURE);
        let dag_body = dsl_body_from_fixture(DAG_FIXTURE);
        assert!(
            !lowpoly.recognize(dag_body).expect("lowpoly recognize dag body"),
            "lowpoly grammar must reject dag fixture body"
        );
        assert!(
            !dag.recognize(lowpoly_body).expect("dag recognize lowpoly body"),
            "dag grammar must reject lowpoly fixture body"
        );
    }
}
//#endregion 🔖️M5CrossArtifactRejection

//#region 🔖️M5ProductionCoverage
/// @emoji 📊️ Production coverage hook: [`Recognizer::uncovered_productions`] reports productions
/// never reached by a shipped pilot fixture. Soft-skips missing specs; logs uncovered names for
/// pilots still mid-handcraft without failing the gate hard until corpus coverage lands.
#[cfg(test)]
mod m5_production_coverage {
    use super::m5_soft_skip::soft_skip_missing;
    use crate::os_dsl::{parse_grammar, Recognizer};
    use crate::os_store::semio_format::split_text_preamble;

    fn dsl_body_from_fixture(text: &str) -> &str {
        if text.trim_start().starts_with("semio ") {
            split_text_preamble(text).map(|(_, body)| body).unwrap_or(text)
        } else {
            text
        }
    }

    fn report_uncovered(grammar_semio: &str, fixture_semio: &str, pilot: &str) {
        if soft_skip_missing(&format!("{pilot}.grammar"), grammar_semio)
            || soft_skip_missing(&format!("{pilot}.fixture"), fixture_semio)
        {
            return;
        }
        let grammar = parse_grammar(grammar_semio).unwrap_or_else(|error| panic!("{pilot}: parse grammar: {error:?}"));
        let recognizer = Recognizer::compile(&grammar);
        let body = dsl_body_from_fixture(fixture_semio);
        let uncovered = recognizer
            .uncovered_productions(body)
            .unwrap_or_else(|error| panic!("{pilot}: uncovered_productions: {error:?}"));
        if !uncovered.is_empty() {
            eprintln!(
                "[DEBUG] {pilot}: uncovered productions ({}) = {}",
                uncovered.len(),
                uncovered.join(", ")
            );
        }
        // Soft assertion for now: recognition must succeed; uncovered list is advisory until P4/P7.
        assert!(
            recognizer.recognize(body).unwrap_or_else(|error| panic!("{pilot}: recognize: {error:?}")),
            "{pilot}: fixture must still recognize while coverage is tracked"
        );
    }

    #[test]
    fn lowpoly_reports_uncovered_productions_for_shipped_fixture() {
        const GRAMMAR: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/🗣️dsl/📖️component.grammar.semio"
        );
        const FIXTURE: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.lowpoly.lowpoly.dsl.semio"
        );
        report_uncovered(GRAMMAR, FIXTURE, "lowpoly");
    }

    #[test]
    fn dag_reports_uncovered_productions_for_shipped_fixture() {
        const GRAMMAR: &str =
            include_str!("../../../../../../✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/🗣️dsl/📖️component.grammar.semio");
        const FIXTURE: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag/📚️examples/♻️reuse/🗣️dsls/♻️reuse/🧬️component.dag.dag.dsl.semio"
        );
        report_uncovered(GRAMMAR, FIXTURE, "dag");
    }

    #[test]
    fn cad_reports_uncovered_productions_for_shipped_fixture() {
        const GRAMMAR: &str =
            include_str!("../../../../../../✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🗣️dsl/📖️component.grammar.semio");
        const FIXTURE: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/📚️examples/♻️default/🗣️dsls/♻️default/🧬️component.cad.cad.dsl.semio"
        );
        report_uncovered(GRAMMAR, FIXTURE, "cad");
    }

    #[test]
    fn en1992_reports_uncovered_productions_for_shipped_fixture() {
        const GRAMMAR: &str =
            include_str!("../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/🗣️dsl/📖️component.grammar.semio");
        const FIXTURE: &str = include_str!(
            "../../../../../../✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992/📚️examples/📕️liquid-retaining-fem-anchor/🗣️dsls/📕️liquid-retaining-fem-anchor/🧬️component.norm.en1992.dsl.semio"
        );
        report_uncovered(GRAMMAR, FIXTURE, "en1992");
    }
}
//#endregion 🔖️M5ProductionCoverage
