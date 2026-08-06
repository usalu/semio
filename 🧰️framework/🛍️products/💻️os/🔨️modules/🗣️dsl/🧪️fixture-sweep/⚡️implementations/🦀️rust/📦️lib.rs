//! 🧭️ Repo-wide DSL fixture-law sweep (W6, final wave of the DSL-notation program). Walks every
//! real shipped `📚️examples/**` fixture file across every plugin/app that derives
//! `store::DocumentDsl` (via `#[derive(dsl::Dsl...)]`, `dsl_derive`'s generated impls, or a
//! hand-rolled Route-A idiom bridge) and proves both engine laws directly against the fixture
//! TEXT — the thing that actually ships, not a separately hand-built in-memory example a per-app
//! test might have drifted from:
//!
//! 1. **parse→print→reparse fixpoint**: `parse_dsl(text)` then `print_dsl` then `parse_dsl` again
//!    recovers an equal value.
//! 2. **canonicalize idempotence**: `canonicalize(x) := print_dsl(parse_dsl(x))` is idempotent —
//!    `canonicalize(canonicalize(x)) == canonicalize(x)`. Equivalent to
//!    `dsl_schema::canonicalize(x, spec, opts)` for every derive-generated `DocumentDsl` impl (see
//!    `store::test_support::check_dsl_fixture_text_laws`'s doc comment for why), and the correct
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

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    //#region 🔖️AppTypes
    // One `use` per registered app kind — aliased where the app's own type is plainly named
    // `Document` (every norm sub-app) to avoid a name collision in this one aggregating module.
    use block::artifacts::block2d::Block2dDefinition;
    use block::artifacts::block3d::Block3dDefinition;
    use block::artifacts::block5d::Block5dDefinition;
    use cad_document::artifacts::cad::CadScene;
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
    use remodel::artifacts::remodel::RemodelScene;
    use trinity::artifacts::rewrite::RewriteRuleState;
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
    /// @emoji 🧭️ `(app label, P::EXTENSION, check fn)` — the check fn is `P`'s own monomorphized
    /// `store::test_support::check_dsl_fixture_text_laws::<P>`, a genuine zero-capture `fn` pointer.
    type CheckFn = fn(&str) -> Result<(), String>;

    fn registry() -> Vec<(&'static str, &'static str, CheckFn)> {
        vec![
            ("writer", <WriterProjection as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<WriterProjection>),
            ("mathematical", <MathProjection as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<MathProjection>),
            ("procedural_2d", <Procedural2dDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Procedural2dDocument>),
            ("procedural_3d", <Procedural3dDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Procedural3dDocument>),
            ("flow_app", <FlowFixture as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<FlowFixture>),
            ("gis2d", <GisMapDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<GisMapDocument>),
            ("gis3d", <Gis3dTerrainDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Gis3dTerrainDocument>),
            ("vcs_app", <VcsDemoProjection as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<VcsDemoProjection>),
            ("present", <PresentDeck as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<PresentDeck>),
            ("shooting", <ShootingFixture as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<ShootingFixture>),
            ("sequence", <SequenceFixture as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<SequenceFixture>),
            ("fem2d", <Fem2dDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Fem2dDocument>),
            ("fem3d", <Fem3dDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Fem3dDocument>),
            ("process_3d", <Process3dDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Process3dDocument>),
            ("lowpoly", <LowpolyProjection as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<LowpolyProjection>),
            ("reasoning_wires", <MindmapWiresDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<MindmapWiresDocument>),
            ("layout", <LayoutDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<LayoutDocument>),
            ("cad_document", <CadScene as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<CadScene>),
            ("iso16757", <Iso16757Document as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Iso16757Document>),
            ("vdi3805", <Vdi3805Document as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Vdi3805Document>),
            ("din4108", <Din4108Document as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Din4108Document>),
            ("din16798", <Din16798Document as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Din16798Document>),
            ("en1990", <En1990Document as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<En1990Document>),
            ("en1991", <En1991Document as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<En1991Document>),
            ("en1992", <En1992Document as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<En1992Document>),
            ("en1993", <En1993Document as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<En1993Document>),
            ("en1994", <En1994Document as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<En1994Document>),
            ("en1995", <En1995Document as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<En1995Document>),
            ("en1996", <En1996Document as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<En1996Document>),
            ("en1997", <En1997Document as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<En1997Document>),
            ("en1998", <En1998Document as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<En1998Document>),
            ("en1999", <En1999Document as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<En1999Document>),
            ("din18599", <Din18599Document as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Din18599Document>),
            ("playbook", <PlaybookSpec as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<PlaybookSpec>),
            ("imperative", <ImperativeDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<ImperativeDocument>),
            ("remodel", <RemodelScene as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<RemodelScene>),
            ("rewrite", <RewriteRuleState as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<RewriteRuleState>),
            ("trinity_ram", <GraphFixture as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<GraphFixture>),
            ("dag_app", <DagDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<DagDocument>),
            ("draw", <DrawDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<DrawDocument>),
            ("raster", <RasterProjection as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<RasterProjection>),
            ("note_app", <NoteDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<NoteDocument>),
            ("puzzle_2d", <Puzzle2dProjection as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Puzzle2dProjection>),
            ("puzzle_5d", <Puzzle5dProjection as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Puzzle5dProjection>),
            ("puzzle_3d", <Puzzle3dProjection as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Puzzle3dProjection>),
            ("block_2d", <Block2dDefinition as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Block2dDefinition>),
            ("block_5d", <Block5dDefinition as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Block5dDefinition>),
            ("block_3d", <Block3dDefinition as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<Block3dDefinition>),
            ("home", <SHomeDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<SHomeDocument>),
            ("semio_framework_os", <WorkflowDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<WorkflowDocument>),
            ("sourcing", <CurateDocument as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<CurateDocument>),
            // 🌱️ `forms` app fixtures ship as `*.forms`, but `FormSpec` is a bare `pub use` alias of
            // `playbook::PlaybookSpec` (forms never overrode `#[dsl(extension = ...)]`), so
            // `<FormSpec as store::DocumentDsl>::EXTENSION` is actually `"playbook"`, not `"forms"` —
            // registered here under the file's real suffix too since `parse_dsl`/`print_dsl` only
            // care about the grammar's field shape, never the extension string.
            ("forms", "forms", store::test_support::check_dsl_fixture_text_laws::<FormSpec>),
            ("space", <SpaceProjection as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<SpaceProjection>),
            ("space", <CollectionProjection as store::DocumentDsl>::EXTENSION, store::test_support::check_dsl_fixture_text_laws::<CollectionProjection>),
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
    //#endregion 🔖️Sweep
}
