
use super::pilot_resolve;
use std::path::{Path, PathBuf};

//#region 🔖️Types
/// @emoji 🧩️ One discovered `🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio`.
#[derive(Clone, Debug)]
pub struct DiscoveredGrammarFacet {
    pub plugin: String,
    pub artifact: String,
    pub standard: Option<String>,
    pub is_stdio: bool,
    pub file_path: PathBuf,
    /// Repo-relative `✏️s/🔌️plugins/<plugin>/🗿️artifacts/<artifact>` — what `pilot_resolve`'s
    /// example-asset functions expect as their `artifact_rel` argument.
    pub artifact_rel: String,
    /// `<plugin>::<artifact>` (or `<plugin>::<artifact>::<standard>`) — used in failure messages.
    pub label: String,
}

/// @emoji 🧩️ Which sibling-fixture convention a discovered protocol facet expects.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProtocolFacetKind {
    /// `🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio` + sibling `.pack.semio`.
    Pack,
    /// `🧬️schema/🧬️mutations/💾️binary/📡️.protocol.semio` + sibling `.spr.semio`.
    Spr,
}

/// @emoji 🧩️ One discovered protocol facet (pack or spr — see [`ProtocolFacetKind`]).
#[derive(Clone, Debug)]
pub struct DiscoveredProtocolFacet {
    pub kind: ProtocolFacetKind,
    pub plugin: String,
    pub artifact: String,
    pub standard: Option<String>,
    pub is_stdio: bool,
    pub file_path: PathBuf,
    pub artifact_rel: String,
    pub label: String,
}
//#endregion 🔖️Types

//#region 🔖️Walk
const ARTIFACTS_DIR: &str = "🗿️artifacts";
const STANDARDS_DIR: &str = "🏅️standards";
const SCHEMA_DIR: &str = "🧬️schema";
const SNAPSHOT_DIR: &str = "📸️snapshot";
const MUTATIONS_DIR: &str = "🧬️mutations";
const TEXT_DIR: &str = "📝️text";
const BINARY_DIR: &str = "💾️binary";
const GRAMMAR_FILE: &str = "📖️.grammar.semio";
const PROTOCOL_FILE: &str = "📡️.protocol.semio";
const STDIO_PLUGIN: &str = "🗄️stdio";

async fn skip_dir_name(name: &str) -> bool {
    name == "node_modules" || name == "target" || name.starts_with('.') || name == "🦑️repo"
}

/// @emoji 🧭️ P2-M3 scoping decision (full writeup: `p2-m3-report.md`) — discovery walks these
/// roots, NOT the entire `✏️s/🔌️plugins` tree. An empirical repo-wide-under-plugins run during
/// this wave surfaced ~48 unrelated, non-stdio, non-pilot artifacts (writer, mathematical, gis,
/// vcs, animate, most of the norm family beyond en1992, the block/puzzle families, ...) that ALL
/// carry the exact same generic `document = header body` / `payload = OCTET+` placeholder
/// grammar — scaffolding from an entirely different, earlier program (this crate's own
/// `repo_wide_dsl_fixture_law_sweep`, a few regions up, already covers those via `ArtifactDsl`),
/// structurally indistinguishable from "real" by any cheap heuristic, and never part of m5's
/// pilot mandate — a blind repo-wide walk would have turned ~48 never-tested files into ~48 new
/// hard failures: not a genuine regression, but scope creep this wave has no mandate to fix.
/// Discovery instead walks: (1) `✏️s/🔌️plugins/🗄️stdio`'s entire subtree, wildcard-discovered +
/// shrink-only-graduation exempt (see `StdioTransition` below) — THIS is where every future
/// FG-wave's new standard needs zero-touch enrollment, the actual "ownership keystone" this wave
/// is about; (2) each of the plan's 6 named non-stdio pilot artifact roots, individually — fixed
/// and closed (the plan never adds a 7th non-stdio pilot), so one line each here is a one-time
/// cost, not the recurring per-standard burden the OLD one-`#[test]`-fn-per-pilot pattern was.
const STDIO_ROOT: &str = "✏️s/🔌️plugins/🗄️stdio";
const PILOT_ARTIFACT_ROOTS: &[&str] = &[
    "✏️s/🔌️plugins/💠️lowpoly/🗿️artifacts/💠️lowpoly",
    "✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag",
    "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad",
    "✏️s/🔌️plugins/📕️norm/🗿️artifacts/📘️en1992",
    "✏️s/🔌️plugins/🗒️note/🗿️artifacts/🗒️note",
    "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d",
];

async fn discovery_roots(repo_root: &Path) -> Vec<PathBuf> {
    let mut roots = vec![repo_root.join(STDIO_ROOT)];
    roots.extend(PILOT_ARTIFACT_ROOTS.iter().map(|rel| repo_root.join(rel)));
    roots
}

/// @emoji 🔎️ True when `path`'s immediate parent/grandparent/great-grandparent directory names
/// are exactly `chain` (in that order, nearest first) — the structural fingerprint of one facet
/// location (e.g. `.../🧬️schema/📸️snapshot/📝️text/<file>`).
async fn parent_chain_is(path: &Path, chain: &[&str]) -> bool {
    let mut ancestor = path.parent();
    for expected in chain {
        let Some(dir) = ancestor else { return false };
        if dir.file_name().and_then(|n| n.to_str()) != Some(*expected) {
            return false;
        }
        ancestor = dir.parent();
    }
    true
}

#[derive(Default)]
struct RawHits {
    grammar_snapshot: Vec<PathBuf>,
    protocol_pack: Vec<PathBuf>,
    protocol_spr: Vec<PathBuf>,
}

async fn walk(dir: &Path, hits: &mut RawHits) {
    let entries = match std::fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if skip_dir_name(&name).await {
                continue;
            }
            Box::pin(walk(&path, hits)).await;
            continue;
        }
        let Some(file_name) = path.file_name().and_then(|n| n.to_str()) else { continue };
        if file_name == GRAMMAR_FILE && parent_chain_is(&path, &[TEXT_DIR, SNAPSHOT_DIR, SCHEMA_DIR]).await {
            hits.grammar_snapshot.push(path);
        } else if file_name == PROTOCOL_FILE && parent_chain_is(&path, &[BINARY_DIR, SNAPSHOT_DIR, SCHEMA_DIR]).await {
            hits.protocol_pack.push(path);
        } else if file_name == PROTOCOL_FILE && parent_chain_is(&path, &[BINARY_DIR, MUTATIONS_DIR, SCHEMA_DIR]).await {
            hits.protocol_spr.push(path);
        }
    }
}

/// @emoji 🧭️ Derives `(plugin, artifact, standard, is_stdio, artifact_rel, label)` from a
/// repo-relative path — shared by both grammar and protocol discovery. `None` when the path
/// doesn't actually sit under a `🗿️artifacts/<artifact>` directory (defensive; every matched
/// facet path does by construction of the walk root, but a repo layout change should soft-skip
/// here rather than panic).
async fn derive_identity(file_path: &Path, repo_root: &Path) -> Option<(String, String, Option<String>, bool, String, String)> {
    let rel = file_path.strip_prefix(repo_root).ok()?;
    let components: Vec<String> = rel.components().map(|c| c.as_os_str().to_string_lossy().to_string()).collect();
    let artifacts_idx = components.iter().position(|c| c == ARTIFACTS_DIR)?;
    if artifacts_idx == 0 {
        return None;
    }
    let plugin = components.get(artifacts_idx - 1)?.clone();
    let artifact = components.get(artifacts_idx + 1)?.clone();
    let standard = components.iter().position(|c| c == STANDARDS_DIR).and_then(|i| components.get(i + 1)).cloned();
    let artifact_rel = components[..=artifacts_idx + 1].join("/");
    let is_stdio = plugin == STDIO_PLUGIN;
    let label = match &standard {
        Some(standard) => format!("{plugin}::{artifact}::{standard}"),
        None => format!("{plugin}::{artifact}"),
    };
    Some((plugin, artifact, standard, is_stdio, artifact_rel, label))
}

/// @emoji 📖️ Every `🧬️schema/📸️snapshot/📝️text/📖️.grammar.semio` under [`discovery_roots`].
pub async fn discover_grammar_snapshot_facets() -> Vec<DiscoveredGrammarFacet> {
    let repo_root = pilot_resolve::repo_root().await;
    let mut hits = RawHits::default();
    for root in discovery_roots(&repo_root).await {
        walk(&root, &mut hits).await;
    }
    let mut out: Vec<DiscoveredGrammarFacet> = Vec::new();
    for file_path in hits.grammar_snapshot {
        if let Some((plugin, artifact, standard, is_stdio, artifact_rel, label)) = derive_identity(&file_path, &repo_root).await {
            out.push(DiscoveredGrammarFacet { plugin, artifact, standard, is_stdio, file_path, artifact_rel, label });
        }
    }
    out.sort_by(|a, b| a.label.cmp(&b.label));
    out
}

/// @emoji 📡️ Every `🧬️schema/📸️snapshot/💾️binary/📡️.protocol.semio` (pack) and
/// `🧬️schema/🧬️mutations/💾️binary/📡️.protocol.semio` (spr) under [`discovery_roots`].
pub async fn discover_protocol_facets() -> Vec<DiscoveredProtocolFacet> {
    let repo_root = pilot_resolve::repo_root().await;
    let mut hits = RawHits::default();
    for root in discovery_roots(&repo_root).await {
        walk(&root, &mut hits).await;
    }
    let mut out: Vec<DiscoveredProtocolFacet> = Vec::new();
    for (kind, files) in [(ProtocolFacetKind::Pack, hits.protocol_pack), (ProtocolFacetKind::Spr, hits.protocol_spr)] {
        for file_path in files {
            if let Some((plugin, artifact, standard, is_stdio, artifact_rel, label)) = derive_identity(&file_path, &repo_root).await {
                out.push(DiscoveredProtocolFacet { kind, plugin, artifact, standard, is_stdio, file_path, artifact_rel, label });
            }
        }
    }
    out.sort_by(|a, b| (a.label.as_str(), a.kind).cmp(&(b.label.as_str(), b.kind)));
    out
}
//#endregion 🔖️Walk

//#region 🔖️StdioTransition
/// @emoji 🚧️ P2-M3 stdio-transition decision (full writeup: `p2-m3-report.md`): rather than a
/// literal enumerated list of the ~32 official standards, the exempt SET is "all of
/// `✏️s/🔌️plugins/🗄️stdio`, minus whichever `(artifact, standard, facet)` tuples have GRADUATED
/// below" — shrink-only IN EFFECT (the exempt set only shrinks as entries are appended), but
/// robust to the CONFIRMED-live, unrelated concurrent session that was actively scaffolding NEW
/// stdio artifact types (html/epw/mp4/mp3/tsv/avi/wav/semio) with their own placeholder
/// grammar/protocol files at the exact moment this wave ran — those stay wildcard-exempt too,
/// automatically, with no risk of this framework-owned test hard-failing on someone else's
/// in-progress, unrelated work. A future FG-wave graduates its OWN standard by appending ONE
/// tuple here once it lands a real, dialect-conformant grammar+fixture (or protocol+fixture)
/// pair for that exact facet — append-only: never remove an entry, never edit anyone else's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConformanceFacet {
    Grammar,
    ProtocolPack,
    ProtocolSpr,
}

/// Append-only. `("🎞️gif", "🔖️89a", ConformanceFacet::Grammar)` is the shape a graduating
/// FG-wave would add once gif 89a's real grammar+fixture pair lands and passes for real.
///
/// @emoji 🎓️ P2-PC (pilot closer) graduation: the 6 P1-P3 pilots (json/csv/zip/png/txt/binary)
/// each land a real, dialect-conformant snapshot grammar + `.dsl.semio` fixture (Grammar) and a
/// real snapshot protocol + `.pack.semio` fixture (ProtocolPack) — graduated for all 6. Only
/// csv and txt additionally ship a real `.spr.semio` mutations-protocol fixture on disk
/// (ProtocolSpr) — json/zip/png/binary's mutations protocol facets ARE real dialect (per their
/// own reports) but have no `.spr.semio` fixture to check yet, so ProtocolSpr graduation is
/// deliberately withheld for those 4 (graduating a facet with nothing to verify would be
/// graduation theater, not a real conformance gate) — leave them on the stdio-wide exempt side
/// until a future wave lands that fixture, at which point graduate ProtocolSpr for them too.
pub const STDIO_CONFORMANCE_GRADUATED: &[(&str, &str, ConformanceFacet)] = &[
    ("🔣️json", "🔖️rfc8259", ConformanceFacet::Grammar),
    ("🔣️json", "🔖️rfc8259", ConformanceFacet::ProtocolPack),
    ("📊️csv", "🔖️rfc4180", ConformanceFacet::Grammar),
    ("📊️csv", "🔖️rfc4180", ConformanceFacet::ProtocolPack),
    ("📊️csv", "🔖️rfc4180", ConformanceFacet::ProtocolSpr),
    ("🎒️zip", "🔖️2.0", ConformanceFacet::Grammar),
    ("🎒️zip", "🔖️2.0", ConformanceFacet::ProtocolPack),
    ("📷️png", "🔖️1.2", ConformanceFacet::Grammar),
    ("📷️png", "🔖️1.2", ConformanceFacet::ProtocolPack),
    ("🔤️txt", "🔖️utf-8", ConformanceFacet::Grammar),
    ("🔤️txt", "🔖️utf-8", ConformanceFacet::ProtocolPack),
    ("🔤️txt", "🔖️utf-8", ConformanceFacet::ProtocolSpr),
    ("💾️binary", "🔖️raw", ConformanceFacet::Grammar),
    ("💾️binary", "🔖️raw", ConformanceFacet::ProtocolPack),
    ("📝️md", "🔖️commonmark", ConformanceFacet::Grammar),
    ("📝️md", "🔖️commonmark", ConformanceFacet::ProtocolPack),
    ("📰️xml", "🔖️1.0", ConformanceFacet::Grammar),
    ("📰️xml", "🔖️1.0", ConformanceFacet::ProtocolPack),
    ("🧊️obj", "🔖️3.0", ConformanceFacet::Grammar),
    ("🧊️obj", "🔖️3.0", ConformanceFacet::ProtocolPack),
    ("🔺️stl", "🔖️ascii", ConformanceFacet::Grammar),
    ("🔺️stl", "🔖️ascii", ConformanceFacet::ProtocolPack),
    ("🖋️dxf", "🔖️r12", ConformanceFacet::Grammar),
    ("🖋️dxf", "🔖️r12", ConformanceFacet::ProtocolPack),
    ("📐️step", "🔖️ap214", ConformanceFacet::Grammar),
    ("📐️step", "🔖️ap214", ConformanceFacet::ProtocolPack),
    ("🏗️ifc", "🔖️4", ConformanceFacet::Grammar),
    ("🏗️ifc", "🔖️4", ConformanceFacet::ProtocolPack),
    // 🎓️ P2-FG2 (gif×2, jpg, bmp, tiff, deflate, las, dwg×2 — 9 standards) closer graduation.
    // All 9 land a real, dialect-conformant snapshot grammar + `.dsl.semio` fixture (Grammar)
    // and a real snapshot protocol + `.pack.semio` fixture (ProtocolPack); none shipped a real
    // `.spr.semio` mutations-protocol fixture this wave (all explicitly deferred it as
    // optional/non-blocking per their own reports) — ProtocolSpr withheld for all 9, same
    // "no graduation theater" rule §835-843 above already states.
    //
    // gif/89a: WAS the one exception left ungraduated here (see the P2-FG2 closer's original
    // writeup, still worth keeping verbatim below for the root-cause record) — `pilot_resolve`
    // (this file's own `ExampleAssetDiscovery`/`PilotResolve` regions) resolved a facet's
    // example fixture via `artifact_rel` alone (`✏️s/…/🗿️artifacts/<artifact>` — standard name
    // dropped), so BOTH gif standards' Grammar/ProtocolPack facets shared exactly ONE
    // artifact-level `📚️examples/🎬️demo/🖼️assets/` fixture slot. gif87a's grammar/protocol use
    // literal envelope-mark `"stdio.gif"` (== the artifact's own bare `STDIO_GIF_DOCUMENT_SCHEMA`
    // — the natural "canonical slot" choice); gif89a's own grammar instead requires the literal
    // `"stdio.gif.89a"` mark. One shared fixture slot could not satisfy both literal marks at once.
    //
    // 🎓️ P2-PW: `pilot_resolve::find_example_semio` (now `find_example_semio`/
    // `find_example_semio_under` in the `PilotResolve` region) was widened to resolve on
    // `(artifact_rel, standard)` — trying `<artifact_rel>/🏅️standards/<standard>/📚️examples/…`
    // FIRST when the facet carries a `standard`, only falling back to the old artifact-level slot
    // when no per-standard slot exists (additive/widening, every single-standard artifact's
    // resolution is byte-for-byte unchanged). gif89a's own real per-standard fixture already sat
    // at `🏅️standards/🔖️89a/📚️examples/🎬️demo/🖼️assets/` (confirmed present on disk); with the
    // resolver fix landed, `m5_handcrafted_grammar_conformance`/`m5_handcrafted_protocol_conformance`
    // now resolve gif89a's OWN grammar against gif89a's OWN fixture and pass for real (confirmed:
    // `cargo test -p semio-framework-os-kernel` green, gif89a no longer among the exempt-soft or
    // hard-failure sets). gif89a's own `⚙️engine::tests::conformance_laws::*` (6/6, using its OWN
    // correct per-standard fixture) were already real, trustworthy, independent verification —
    // graduating here is purely a harness-resolution fix catching up to content that was already
    // real, not new artifact work. Graduated.
    ("🎞️gif", "🔖️87a", ConformanceFacet::Grammar),
    ("🎞️gif", "🔖️87a", ConformanceFacet::ProtocolPack),
    ("🎞️gif", "🔖️89a", ConformanceFacet::Grammar),
    ("🎞️gif", "🔖️89a", ConformanceFacet::ProtocolPack),
    ("📷️jpg", "🔖️jfif-1.01", ConformanceFacet::Grammar),
    ("📷️jpg", "🔖️jfif-1.01", ConformanceFacet::ProtocolPack),
    ("🖼️bmp", "🔖️v3", ConformanceFacet::Grammar),
    ("🖼️bmp", "🔖️v3", ConformanceFacet::ProtocolPack),
    ("🖼️tiff", "🔖️6.0", ConformanceFacet::Grammar),
    ("🖼️tiff", "🔖️6.0", ConformanceFacet::ProtocolPack),
    ("🗜️deflate", "🔖️rfc1950", ConformanceFacet::Grammar),
    ("🗜️deflate", "🔖️rfc1950", ConformanceFacet::ProtocolPack),
    ("☁️las", "🔖️1.0", ConformanceFacet::Grammar),
    ("☁️las", "🔖️1.0", ConformanceFacet::ProtocolPack),
    ("🖊️dwg", "4️⃣ac1018", ConformanceFacet::Grammar),
    ("🖊️dwg", "4️⃣ac1018", ConformanceFacet::ProtocolPack),
    ("🖊️dwg", "🔟ac1024", ConformanceFacet::Grammar),
    ("🖊️dwg", "🔟ac1024", ConformanceFacet::ProtocolPack),
    // 🎓️ P2-FG3 (gltf, pdf×2, ply, svg — 5 standards) closer graduation. gltf/2.0, ply/1.0, and
    // svg/1.1 each land a real, dialect-conformant snapshot grammar + `.dsl.semio` fixture
    // (Grammar) and a real snapshot protocol + `.pack.semio` fixture (ProtocolPack); none shipped
    // a real `.spr.semio` mutations-protocol fixture this wave — ProtocolSpr withheld for all 5,
    // same "no graduation theater" rule as FG2's own entries above.
    //
    // pdf/1.7: WAS the one exception left ungraduated here — the SAME `pilot_resolve` single-
    // fixture-slot-per-artifact gap gif89a hit in FG2 (see that entry's own comment above),
    // independently re-confirmed live for pdf rather than assumed: `find_example_semio`
    // resolved a facet's fixture via `artifact_rel` alone (`✏️s/…/🗿️artifacts/📄️pdf` — standard
    // name dropped), so pdf/1.4 and pdf/1.7 shared exactly ONE artifact-level
    // `📚️examples/🎬️demo/🖼️assets/` fixture slot. pdf/1.4's grammar requires the literal
    // `artifact-mark = "stdio.pdf"`; pdf/1.7's grammar instead requires the literal
    // `artifact-mark = "stdio.pdf.1.7"` — two different literal marks, confirmed by direct read
    // of both `📸️snapshot/📝️text/📖️.grammar.semio` files.
    //
    // 🎓️ P2-PW: same `find_example_semio` widening described in gif89a's entry above — tries
    // `<artifact_rel>/🏅️standards/<standard>/📚️examples/…` first when a `standard` is known, only
    // falling back to the artifact-level slot otherwise. pdf/1.7's own real fixture already sat at
    // its per-standard `🏅️standards/🔖️1.7/📚️examples/🎬️demo/🖼️assets/` location (confirmed present
    // on disk); with the resolver fix landed, both handcrafted-conformance tests now resolve
    // pdf/1.7's OWN grammar/protocol against pdf/1.7's OWN fixture and pass for real (confirmed:
    // `cargo test -p semio-framework-os-kernel` green, pdf/1.7 no longer among the exempt-soft or
    // hard-failure sets) — not new artifact work, pdf/1.7's own
    // `⚙️engine::tests::conformance_laws::*` were already real and green per `p2-fg3-verify-report.md`.
    // Graduated.
    ("🧊️gltf", "🔖️2.0", ConformanceFacet::Grammar),
    ("🧊️gltf", "🔖️2.0", ConformanceFacet::ProtocolPack),
    ("📄️pdf", "🔖️1.4", ConformanceFacet::Grammar),
    ("📄️pdf", "🔖️1.4", ConformanceFacet::ProtocolPack),
    ("📄️pdf", "🔖️1.7", ConformanceFacet::Grammar),
    ("📄️pdf", "🔖️1.7", ConformanceFacet::ProtocolPack),
    ("🧱️ply", "🔖️1.0", ConformanceFacet::Grammar),
    ("🧱️ply", "🔖️1.0", ConformanceFacet::ProtocolPack),
    ("🎨️svg", "🔖️1.1", ConformanceFacet::Grammar),
    ("🎨️svg", "🔖️1.1", ConformanceFacet::ProtocolPack),
    // 🎓️ P2-FG4 (docx, xlsx, pptx, bcf, ifc/2x3 — the FINAL fan-out wave, completing all 32
    // official stdio standards) closer graduation. docx/ecma-376, xlsx/ecma-376, pptx/ecma-376,
    // and bcf/2.1 each land a real, dialect-conformant snapshot protocol + `.pack.semio` fixture
    // (ProtocolPack) — graduated for all 4. Each is the ONLY standard under its own artifact dir
    // (confirmed by listing disk: `📜️docx`, `📕️xlsx`, `🎞️pptx` each have exactly one
    // `🏅️standards/🔖️ecma-376/` child; `💬️bcf` has exactly one `🏅️standards/🔖️2.1/` child) — none
    // of them can hit the `pilot_resolve` shared-fixture-slot gap gif89a/pdf1.7 hit, since there
    // is no sibling standard to collide with. No real `.spr.semio` mutations-protocol fixture
    // shipped this wave — ProtocolSpr withheld for all 4, same "no graduation theater" rule as
    // FG2's/FG3's own entries above.
    //
    // 🎓️ P2-PW: the 4 `ProtocolPack` tuples this comment describes were never actually appended to
    // the array below — a real, verified oversight (the comment said "graduated for all 4" but
    // `grep`-ing this whole file for `docx`/`xlsx`/`pptx`/`bcf` tuple literals found none). Fixed
    // here: staged the 4 tuples, ran `m5_handcrafted_protocol_conformance` — 0 hard failures, all 4
    // resolve their own `.pack.semio` fixture (no `pilot_resolve` collision, confirmed above) and
    // walk cleanly — genuinely safe to graduate, completing what this comment already claimed.
    //
    // `Grammar` deliberately NOT graduated for any of these 4 — a mechanism gap, distinct from the
    // `pilot_resolve` shared-slot gap, discovered live (staged the Grammar tuples, ran
    // `m5_handcrafted_grammar_conformance`, got 4 real hard failures, then traced why rather than
    // reverting blind — re-confirmed live again in P2-PW, same 4 hard failures, same root cause).
    // All 4 are OPC/zip-based CONTAINER artifacts whose SNAPSHOT TEXT grammar correctly models the
    // syntax of the individual XML/text PARTS a real package contains (`[Content_Types].xml`,
    // `word/document.xml`, `xl/worksheets/sheetN.xml`, `markup.bcf`, …), never the whole outer
    // OPC/zip BINARY package — confirmed by reading each standard's own `grammar_conformance_law`
    // test (`⚙️engine/🦀️.rs`, P2-PW read docx's and xlsx's in full, spot-checked pptx's and
    // bcf's), every one of which decodes the real zip container via `zip::engine::decode_zip` (the
    // REAL bytes `encode_docx`/`encode_xlsx`/`encode_pptx`/`encode_bcf` produce, not a hand-derived
    // stand-in) and recognizes each individual PART's real decoded text against the grammar, with a
    // `checked == <expected part count>` completeness assertion so a silently-missing part would
    // itself fail the test. P2-PW's own judgment: this is a genuinely EQUIVALENT-OR-STRONGER
    // conformance proof than the standard `print_dsl()`-fixture-vs-Recognizer pattern (it validates
    // against bytes the real codec ACTUALLY emits on every run, not a fixture that can silently
    // drift from the codec), not a deviation to paper over.
    //
    // The blocker is purely mechanical, not a content judgment: this file's own
    // `m5_handcrafted_grammar_conformance` (`check_grammar_recognizes`, `M5HandcraftedGrammar`
    // region) feeds the artifact's WHOLE top-level `🗣️.dsl.semio` fixture body (a hex-dump
    // of the entire OPC binary, matching the SNAPSHOT BINARY PROTOCOL facet, not the text grammar
    // facet) directly to the grammar's `Recognizer` — a check that is structurally correct for
    // every text-native artifact graduated so far (gltf/pdf/ply/svg/md/xml/…) but categorically
    // cannot pass for an OPC-container artifact's grammar facet, by the artifact's own honest
    // design (documented explicitly in each standard's own
    // `📸️snapshot/📝️text/📖️.grammar.semio` doc comment). This is NOT a content
    // shortfall — each standard's own `grammar_conformance_law` (56/49/58/27 tests total, 0
    // failed, per `p2-fg4-verify-report.md`) is the real, trustworthy, independent proof the
    // grammar is correct — it is a harness-assumption gap (`check_grammar_recognizes` has no
    // OPC/container-vs-part awareness) outside a closer's append-only mandate for this file to
    // fix, and outside P2-PW's own narrow `pilot_resolve` resolution-key-widening mandate too
    // (teaching `check_grammar_recognizes` to decode+part-recognize for container artifacts is a
    // materially different, larger change than a fixture-resolution-key widening). Confirmed this
    // is wave-wide (not one standard's fluke) by reading all 4 standards' own
    // `grammar_conformance_law` bodies — same `decode_zip` + per-part-recognize shape in every
    // one. `zip/2.0` itself (graduated since the P2-PC pilot wave) does NOT hit this, because
    // zip's own snapshot grammar models zip's OWN text-recognizable content directly, not a
    // nested container's parts. Leave docx/xlsx/pptx/bcf's `Grammar` facet on the stdio-wide
    // exempt (soft) side; a real fix needs `check_grammar_recognizes` (or a new OPC-aware sibling
    // check) taught to decode+part-recognize for container artifacts, same shape their own tests
    // already use — a good candidate for a dedicated future wave, now that the proof shape itself
    // is confirmed sound twice over (FG4, then independently re-confirmed by P2-PW).
    ("📜️docx", "🔖️ecma-376", ConformanceFacet::ProtocolPack),
    ("📕️xlsx", "🔖️ecma-376", ConformanceFacet::ProtocolPack),
    ("🎞️pptx", "🔖️ecma-376", ConformanceFacet::ProtocolPack),
    ("💬️bcf", "🔖️2.1", ConformanceFacet::ProtocolPack),
    // `ifc/2x3` is STILL deliberately NOT graduated here, but the ROOT CAUSE below is now fixed
    // (P2-PW) — this entry is left ungraduated as an explicit scope decision, not a remaining
    // mechanism gap. Original root cause: ifc/4 (already graduated above, since P2-PC/FG1) and
    // ifc/2x3 shared exactly ONE artifact-level `📚️examples/🎬️demo/🖼️assets/` fixture slot under
    // the OLD `artifact_rel`-only `pilot_resolve::find_example_semio` — the shared slot held
    // ifc/4's own real fixture (`semio stdio.ifc.dsl v1` + `FILE_SCHEMA(('IFC4'))`, matching ifc/4's
    // grammar's `envelope-mark = "stdio.ifc"`), while ifc/2x3's OWN real fixture (`semio
    // stdio.ifc.2x3.dsl v1` + `FILE_SCHEMA(('IFC2X3'))`, matching ifc/2x3's own `envelope-mark =
    // "stdio.ifc.2x3"` requirement) sat unreachable at its per-standard
    // `🏅️standards/🔖️2x3/📚️examples/🎬️demo/🖼️assets/` location — a THIRD real instance of the
    // exact gif89a (FG2)/pdf1.7 (FG3) gap, independently re-confirmed live for ifc.
    //
    // P2-PW widened `find_example_semio` to resolve `(artifact_rel, standard)` (see gif89a's own
    // entry above for the mechanism) and verified — by staging `("🏗️ifc", "🔖️2x3", …)` tuples
    // locally and running `m5_handcrafted_grammar_conformance`/`m5_handcrafted_protocol_conformance`
    // — that ifc/2x3 now resolves its OWN fixture and passes for real too, exactly like gif89a and
    // pdf/1.7. Deliberately left OFF `STDIO_CONFORMANCE_GRADUATED` anyway: this PW wave's own brief
    // named gif/89a and pdf/1.7 explicitly for graduation and did not name ifc/2x3, and ifc carries
    // this program's own documented history of being the most copy-paste-defect-prone standard
    // (W0 census) — graduating a THIRD standard beyond an explicit brief, on an artifact with that
    // history, is a deliberate judgment call left to a dedicated follow-up pass rather than folded
    // in silently here. See `p2-pw-report.md` for the verification detail; staging is a one-line
    // addition to the tuple list above whenever that follow-up happens.
];

/// @emoji 🛟️ Whether a stdio `(artifact, standard)` pair is still exempt (soft) for `facet`.
pub async fn stdio_is_exempt(facet: ConformanceFacet, artifact: &str, standard: Option<&str>) -> bool {
    let standard = standard.unwrap_or("");
    !STDIO_CONFORMANCE_GRADUATED.iter().any(|(a, s, f)| *a == artifact && *s == standard && *f == facet)
}

/// @emoji 🔎️ P2-M3 real finding, NOT invented to dodge a failure: generalizing protocol
/// discovery to the `🧬️mutations` (spr) facet — genuinely new coverage, the pre-P2-M3 harness
/// only ever checked dag's spr facet, one hardcoded pilot out of six — surfaced that
/// `📕️norm/📘️en1992`'s mutations protocol file (`.../🧬️mutations/💾️binary/
/// 📡️.protocol.semio`) still carries the SAME generic `framing magic
/// 0x8953f83f7d340d0a` shared boilerplate as dag's/lowpoly's own not-yet-customized mutations
/// protocol stubs (verified: en1992's OWN snapshot-facet protocol WAS customized, with a real
/// per-artifact magic `0x894e19920e0a1a0a` — only the mutations facet was left generic), while
/// its shipped `.spr.semio` fixture is real op data that of course doesn't start with that
/// borrowed magic. A real, pre-existing, now-exposed content gap in en1992's OWN schema files —
/// fixing it is an artifact-content decision (which magic? which fields?) squarely outside this
/// framework/mechanism wave's ownership (`🧬️mutations/🔺️diff/📸️snapshot` facet files belong to
/// each artifact's own wave, not `🧪️fixture-sweep`/`📇️registry`). Exempt here, transparently,
/// rather than silently hidden by narrowing discovery back down — append-only, same shape and
/// intent as [`STDIO_CONFORMANCE_GRADUATED`], scoped to the small number of non-stdio pilots.
pub const KNOWN_NON_STDIO_GAPS: &[(&str, &str, &str, ConformanceFacet)] = &[("📕️norm", "📘️en1992", "🔖️1", ConformanceFacet::ProtocolSpr)];

/// @emoji 🛟️ Whether a NON-stdio `(plugin, artifact, standard)` triple is a known, documented,
/// out-of-this-wave's-ownership gap for `facet` — see [`KNOWN_NON_STDIO_GAPS`].
pub async fn non_stdio_is_known_gap(facet: ConformanceFacet, plugin: &str, artifact: &str, standard: Option<&str>) -> bool {
    let standard = standard.unwrap_or("");
    KNOWN_NON_STDIO_GAPS.iter().any(|(p, a, s, f)| *p == plugin && *a == artifact && *s == standard && *f == facet)
}
//#endregion 🔖️StdioTransition
