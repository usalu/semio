//! 🧭️ Full-fleet example laws; public kernel APIs and production providers only.

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    //#region 🔖️AppTypes
    // One `use` per registered app kind — aliased where the app's own type is plainly named
    // `Document` (every norm sub-app) to avoid a name collision in this one aggregating module.
    use block::artifacts::block2d::Block2dSnapshot as Block2dDefinition;
    use block::artifacts::block3d::Block3dSnapshot as Block3dDefinition;
    use block::artifacts::block5d::Block5dSnapshot as Block5dDefinition;
    use cad_document::artifacts::cad::CadSnapshot;
    use dag_app::artifacts::dag::DagSnapshot;
    use draw::artifacts::draw::DrawSnapshot as DrawDocument;
    use fem::artifacts::fem2d::Fem2dSnapshot as Fem2dDocument;
    use fem::artifacts::fem3d::Fem3dSnapshot as Fem3dDocument;
    use flow_app::FlowFixture;
    use norm::artifacts::din16798::Din16798Snapshot as Din16798Document;
    use norm::artifacts::din18599::Din18599Snapshot as Din18599Document;
    use norm::artifacts::din4108::Din4108Snapshot as Din4108Document;
    use norm::artifacts::en1990::En1990Snapshot as En1990Document;
    use norm::artifacts::en1991::En1991Snapshot as En1991Document;
    use norm::artifacts::en1992::En1992Snapshot as En1992Document;
    use norm::artifacts::en1993::En1993Snapshot as En1993Document;
    use norm::artifacts::en1994::En1994Snapshot as En1994Document;
    use norm::artifacts::en1995::En1995Snapshot as En1995Document;
    use norm::artifacts::en1996::En1996Snapshot as En1996Document;
    use norm::artifacts::en1997::En1997Snapshot as En1997Document;
    use norm::artifacts::en1998::En1998Snapshot as En1998Document;
    use norm::artifacts::en1999::En1999Snapshot as En1999Document;
    // 🌱️ 26/08/05/FORMS-PLUGIN-MIGRATION-TO-CRATE-AND-TAXONOMY-CONSOLIDATION: the old `forms` app facade
    // crate is gone (merged into `semio-s-plugin-forms`); `FormSpec` was always a bare `pub use` alias of
    // `playbook::PlaybookSpec` (forms never overrode `#[dsl(extension = ...)]`) so this repoints straight
    // at the real owner of the type — no `lib.rs` ripple beyond this import line (see TEMPLATE.md §8.2).
    use gis::artifacts::gismap::GisMapSnapshot as GisMapDocument;
    use gis::artifacts::gisterrain::GisTerrainSnapshot as Gis3dTerrainDocument;
    use home::artifacts::home::SHomeSnapshot as SHomeDocument;
    use imperative::artifacts::procedure::ProcedureSnapshot as ImperativeDocument;
    use layout::artifacts::layout::LayoutSnapshot as LayoutDocument;
    use lowpoly::artifacts::lowpoly::LowpolySnapshot;
    use mathematical::artifacts::equation::EquationSnapshot;
    use norm::artifacts::iso16757::Iso16757Snapshot as Iso16757Document;
    use norm::artifacts::vdi3805::Vdi3805Snapshot as Vdi3805Document;
    use note_app::artifacts::note::NoteSnapshot as NoteDocument;
    // 📖️ `playbook::PlaybookSpec` is the FRAMEWORK kernel's playbook domain type, mounted inside
    // `flow_app` (`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust`'s glue re-exports
    // `../../../📖️playbook/🦀️.rs` as `flow_app::playbook`) — not a standalone `playbook` crate.
    use flow_app::playbook::PlaybookSpec as FormSpec;
    use flow_app::playbook::PlaybookSpec;
    use presentation::artifacts::presentation::PresentationSnapshot as PresentationDeck;
    use procedural::artifacts::generation2d::Generation2dSnapshot as Generation2dDocument;
    use procedural::artifacts::generation3d::Generation3dSnapshot as Generation3dDocument;
    use process_3d::artifacts::process3d::Process3dSnapshot as Process3dDocument;
    use puzzle::artifacts::puzzle2d::Puzzle2dSnapshot;
    use puzzle::artifacts::puzzle3d::Puzzle3dSnapshot;
    use puzzle::artifacts::puzzle5d::Puzzle5dSnapshot;
    use raster::artifacts::raster::RasterSnapshot;
    use reasoning_mindmap_plugin::artifacts::wires::WiresSnapshot as MindmapWiresDocument;
    use remodel::artifacts::remodel::RemodelSnapshot;
    use semio_framework_os::WorkflowSnapshot;
    use sequence::artifacts::sequence::SequenceFixture;
    use shooting::artifacts::shooting::ShootingSnapshot as ShootingFixture;
    use sourcing::artifacts::curation::CurationSnapshot as CurationDocument;
    // 🪐️ `semio_framework_os::space` (framework OS product, NOT the `space` plugin `home` is
    // aliased to above) — `SpaceSnapshot`/`CollectionSnapshot` live at
    // `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🦀️.rs`, mounted by that crate's glue.
    use semio_framework_os::space::{CollectionSnapshot, SpaceSnapshot};
    use trinity::artifacts::jack::JackSnapshot as GraphFixture;
    use trinity::artifacts::rewriting::RewritingSnapshot as RewriteRuleModel;
    use vcs_app::artifacts::vcs::VcsSnapshot;
    use writer::artifacts::writer::WriterSnapshot;
    //#endregion 🔖️AppTypes

    //#region 🔖️Registry
    /// @emoji 🧭️ `(app label, envelope_id, check fn)` — dispatch is by sniffed `plugin.artifact` from `.semio` content.
    type CheckFn = fn(&str) -> Result<(), String>;

    fn registry() -> Vec<(&'static str, &'static str, CheckFn)> {
        vec![
            ("writer", <WriterSnapshot as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<WriterSnapshot>),
            ("equation", <EquationSnapshot as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<EquationSnapshot>),
            ("generation_2d", <Generation2dDocument as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Generation2dDocument>),
            ("generation_3d", <Generation3dDocument as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Generation3dDocument>),
            ("flow_app", <FlowFixture as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<FlowFixture>),
            ("gis2d", "gis.gismap", semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<GisMapDocument>),
            ("gis3d", "gis.gisterrain", semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Gis3dTerrainDocument>),
            ("vcs_app", <VcsSnapshot as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<VcsSnapshot>),
            ("presentation", <PresentationDeck as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<PresentationDeck>),
            ("shooting", <ShootingFixture as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<ShootingFixture>),
            ("sequence", <SequenceFixture as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<SequenceFixture>),
            ("fem2d", <Fem2dDocument as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Fem2dDocument>),
            ("fem3d", <Fem3dDocument as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Fem3dDocument>),
            ("process_3d", <Process3dDocument as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Process3dDocument>),
            ("lowpoly", <LowpolySnapshot as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<LowpolySnapshot>),
            ("reasoning_wires", <MindmapWiresDocument as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<MindmapWiresDocument>),
            ("layout", <LayoutDocument as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<LayoutDocument>),
            ("cad_document", <CadSnapshot as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<CadSnapshot>),
            ("iso16757", <Iso16757Document as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Iso16757Document>),
            ("vdi3805", <Vdi3805Document as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Vdi3805Document>),
            ("din4108", <Din4108Document as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Din4108Document>),
            ("din16798", <Din16798Document as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Din16798Document>),
            ("en1990", <En1990Document as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<En1990Document>),
            ("en1991", <En1991Document as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<En1991Document>),
            ("en1992", <En1992Document as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<En1992Document>),
            ("en1993", <En1993Document as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<En1993Document>),
            ("en1994", <En1994Document as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<En1994Document>),
            ("en1995", <En1995Document as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<En1995Document>),
            ("en1996", <En1996Document as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<En1996Document>),
            ("en1997", <En1997Document as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<En1997Document>),
            ("en1998", <En1998Document as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<En1998Document>),
            ("en1999", <En1999Document as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<En1999Document>),
            ("din18599", <Din18599Document as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Din18599Document>),
            ("playbook", <PlaybookSpec as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<PlaybookSpec>),
            ("imperative", <ImperativeDocument as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<ImperativeDocument>),
            ("remodel", <RemodelSnapshot as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<RemodelSnapshot>),
            ("rewrite", <RewriteRuleModel as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<RewriteRuleModel>),
            ("trinity_ram", <GraphFixture as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<GraphFixture>),
            ("dag_app", <DagSnapshot as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<DagSnapshot>),
            ("draw", <DrawDocument as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<DrawDocument>),
            ("raster", <RasterSnapshot as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<RasterSnapshot>),
            ("note_app", <NoteDocument as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<NoteDocument>),
            ("puzzle_2d", <Puzzle2dSnapshot as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Puzzle2dSnapshot>),
            ("puzzle_5d", <Puzzle5dSnapshot as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Puzzle5dSnapshot>),
            ("puzzle_3d", <Puzzle3dSnapshot as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Puzzle3dSnapshot>),
            ("block_2d", <Block2dDefinition as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Block2dDefinition>),
            ("block_5d", <Block5dDefinition as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Block5dDefinition>),
            ("block_3d", <Block3dDefinition as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<Block3dDefinition>),
            ("home", <SHomeDocument as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<SHomeDocument>),
            ("semio_framework_os", <WorkflowSnapshot as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<WorkflowSnapshot>),
            ("sourcing", <CurationDocument as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<CurationDocument>),
            // 🌱️ `forms` app fixtures ship as `*.forms`, but `FormSpec` is a bare `pub use` alias of
            // `playbook::PlaybookSpec` (forms never overrode `#[dsl(extension = ...)]`), so
            // `<FormSpec as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id()` is actually `"playbook"`, not `"forms"` —
            // registered here under the file's real suffix too since `parse_dsl`/`print_dsl` only
            // care about the grammar's field shape, never the extension string.
            ("forms", "forms", semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<FormSpec>),
            ("space", <SpaceSnapshot as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<SpaceSnapshot>),
            ("space", <CollectionSnapshot as semio_framework_os_kernel::os_store::ArtifactDsl>::envelope_id(), semio_framework_os_kernel::os_store::test_support::check_dsl_fixture_text_laws::<CollectionSnapshot>),
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

    const EXAMPLES_DIR_NAME: &str = "📚️examples";
    const ASSETS_DIR_NAME: &str = "🖼️assets";
    const LEGACY_KIND_DIRS: &[&str] = &["🗣️dsls", "🎒️packs", "🔧️ops", "📡️sprs"];

    fn skip_dir_name(name: &str) -> bool {
        name == "node_modules" || name == "target" || name.starts_with('.') || name == "🦑️repo"
    }

    /// @emoji 📚️ Recursively finds every directory literally named `📚️examples` under `root`,
    /// skipping `node_modules`/`target`/hidden/ticket-scratch directories.
    fn example_dirs(root: &Path) -> Vec<PathBuf> {
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
                if skip_dir_name(&name) {
                    continue;
                }
                if name == EXAMPLES_DIR_NAME {
                    out.push(path.clone());
                }
                walk(&path, out);
            }
        }
        let mut out = Vec::new();
        walk(root, &mut out);
        out
    }

    /// @emoji 🏷️ Direct child directories of a `📚️examples` root — one per example slug.
    fn example_slug_dirs(examples_dir: &Path) -> Vec<PathBuf> {
        let mut out = Vec::new();
        let entries = match std::fs::read_dir(examples_dir) {
            Ok(entries) => entries,
            Err(_) => return out,
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                out.push(path);
            }
        }
        out.sort();
        out
    }

    /// @emoji 📄️ Recursively collects every FILE under `dir`.
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

    /// @emoji 🖼️ Collects `.semio` assets for one example slug.
    /// Prefers `🖼️assets/` (new layout); soft-migrates by walking the slug tree when assets are absent.
    fn collect_slug_semio_files(slug_dir: &Path) -> Vec<PathBuf> {
        let assets = slug_dir.join(ASSETS_DIR_NAME);
        let mut files = Vec::new();
        if assets.is_dir() {
            collect_files(&assets, &mut files);
        } else {
            collect_files(slug_dir, &mut files);
        }
        files.retain(|path| path.extension().and_then(|e| e.to_str()) == Some("semio"));
        files.sort();
        files
    }

    /// @emoji 📚️ Repo-wide `.semio` example assets under every `📚️examples/<slug>/` (assets-first).
    fn collect_example_semio_files(root: &Path) -> Vec<PathBuf> {
        let mut out = Vec::new();
        for examples in example_dirs(root) {
            for slug in example_slug_dirs(&examples) {
                out.extend(collect_slug_semio_files(&slug));
            }
        }
        out
    }

    fn has_semio_under(dir: &Path) -> bool {
        let mut files = Vec::new();
        collect_files(dir, &mut files);
        files.iter().any(|path| path.extension().and_then(|e| e.to_str()) == Some("semio"))
    }

    fn slug_has_legacy_kind_dirs(slug_dir: &Path) -> bool {
        LEGACY_KIND_DIRS.iter().any(|kind| slug_dir.join(kind).is_dir())
    }
    //#endregion 🔖️Walk

    //#region 🔖️Sweep
    #[semio_framework_async_macros::async_test]
    async fn repo_wide_dsl_fixture_law_sweep() {
        let root = repo_root();
        let dirs = example_dirs(&root);
        assert!(!dirs.is_empty(), "found zero 📚️examples directories under {root:?} — sweep would vacuously pass");

        let fixture_files = collect_example_semio_files(&root);
        assert!(!fixture_files.is_empty(), "found {} 📚️examples dir(s) but zero .semio fixture files under {root:?}", dirs.len());

        let registry = registry();
        let mut walked = 0usize;
        let mut unmapped: Vec<String> = Vec::new();
        let mut failures: Vec<String> = Vec::new();

        for file in &fixture_files {
            let bytes = std::fs::read(file).unwrap_or_else(|error| panic!("read {}: {error}", file.display()));
            let envelope = match semio_framework_os_kernel::os_store::semio_format::sniff(&bytes) {
                Ok(envelope) => envelope,
                Err(detail) => {
                    unmapped.push(format!("{} (semio sniff failed: {detail})", file.display()));
                    continue;
                }
            };
            if envelope.component != semio_framework_os_kernel::os_store::semio_format::Component::Dsl {
                continue;
            }
            let key = envelope.envelope_id();
            let matching: Vec<&(&str, &str, CheckFn)> = registry.iter().filter(|(_, ext, _)| *ext == key).collect();
            if matching.is_empty() {
                unmapped.push(format!("{} (envelope {key} — no registered ArtifactDsl)", file.display()));
                continue;
            }
            let text = std::str::from_utf8(&bytes).unwrap_or_else(|_| panic!("{} is not valid utf-8", file.display()));
            for (label, _, check) in &matching {
                walked += 1;
                if let Err(detail) = check(text) {
                    failures.push(format!("[{label}] {}: {detail}", file.display()));
                }
            }
        }

        eprintln!("[dsl-fixture-sweep] {} example dir(s), {} .semio fixture file(s) found, {} law-check(s) run across {} registered app kind(s), {} unmapped fixture(s)", dirs.len(), fixture_files.len(), walked, registry.len(), unmapped.len());
        if !unmapped.is_empty() {
            eprintln!("[dsl-fixture-sweep] unmapped fixtures (no registered ArtifactDsl app matches this extension — not counted as a failure):");
            for entry in &unmapped {
                eprintln!("  {entry}");
            }
        }

        assert!(failures.is_empty(), "dsl fixture law sweep failed for {} check(s) across {} fixture file(s):\n\n{}", failures.len(), fixture_files.len(), failures.join("\n\n"));
    }

    #[semio_framework_async_macros::async_test]
    async fn repo_wide_semio_example_kind_coverage() {
        // Target: each artifact `📚️examples/<slug>/` has `🖼️assets/` with ≥1 `.semio`.
        // Mid-migration (W1b→W3): soft-skip slugs that still lack `🖼️assets/` with a clear message.
        // Empty `🖼️assets/` after the dir exists is a hard gap.
        let root = repo_root();
        let plugins = root.join("✏️s").join("🔌️plugins");
        let mut gaps: Vec<String> = Vec::new();
        let mut migrated = 0usize;
        let mut soft_skipped = 0usize;
        let read_dir = |p: &Path| std::fs::read_dir(p).ok().map(|d| d.filter_map(|e| e.ok()).collect::<Vec<_>>()).unwrap_or_default();
        for plugin in read_dir(&plugins) {
            let artifacts = plugin.path().join("🗿️artifacts");
            for artifact in read_dir(&artifacts) {
                let artifact_path = artifact.path();
                if !artifact_path.is_dir() {
                    continue;
                }
                let examples = artifact_path.join(EXAMPLES_DIR_NAME);
                if !examples.is_dir() {
                    continue;
                }
                for slug in example_slug_dirs(&examples) {
                    let assets = slug.join(ASSETS_DIR_NAME);
                    if assets.is_dir() {
                        if has_semio_under(&assets) {
                            migrated += 1;
                        } else {
                            gaps.push(format!("{}: {}/ present but has zero .semio files", slug.display(), ASSETS_DIR_NAME));
                        }
                    } else {
                        soft_skipped += 1;
                        let legacy_hint = if slug_has_legacy_kind_dirs(&slug) { "legacy plural kind dirs still present" } else { "no legacy kind dirs either" };
                        eprintln!("[DEBUG] soft-skip example coverage {}: missing {}/ with ≥1 .semio — mid-migration ({})", slug.display(), ASSETS_DIR_NAME, legacy_hint);
                    }
                }
            }
        }
        eprintln!("[dsl-fixture-sweep] example asset coverage: {migrated} slug(s) on new 🖼️assets layout, {soft_skipped} soft-skipped mid-migration");
        assert!(gaps.is_empty(), "semio example asset gaps:\n{}", gaps.join("\n"));
    }
    //#endregion 🔖️Sweep
}
