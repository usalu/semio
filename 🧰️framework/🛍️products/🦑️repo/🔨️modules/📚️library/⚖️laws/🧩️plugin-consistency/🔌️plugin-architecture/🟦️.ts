/** 📐️ Wave 4 plugin consistency policy: artifacts-only plugin architecture rules and the APA ratchet.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */
import { type PolicyModSpan, policyMaskLiterals, policyTestModSpans, policyLineInTestMod } from "../../../🔍️discovery/📋️policy/🔎️source/🟦️.ts";
import { loadTaxonomy, type BreachRecord } from "../../../📦️packages/🟦️typescript/🟦️.ts";
import { POLICY_SKIP_DIRS, policyReadFileSafe, policyReaddirSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyLineOfIndex } from "../../../🔍️discovery/📍️source-coordinate/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF_NAME } from "../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { existsSync, readFileSync, readdirSync, type Dirent } from "node:fs";
import { join } from "node:path";
import { policyDiscoverCargoTomlFiles } from "../🚧️authority-bans/🟦️.ts";
import { policyAllRustFiles } from "../🏛️foundation/🟦️.ts";
import { taxonomyContractFilenames, taxonomyMappedFilename } from "../../../🧹️normalization/🔣️taxonomy/🏷️filenames/🟦️.ts";

//#region 🔧️PolicyRuleArtifactsOnlyPluginArchitecture
/**
 * 🏛️ APA (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE) — five report-mode census rules enforcing
 * the target plugin shape: a plugin is EXACTLY 🎛️apps + 🗿️artifacts + root 🦀️.rs/AGENTS.md/
 * README.md + 📦️packages wiring; all io, state changes, registration, and side effects belong to
 * artifacts — never apps, never a setup facet. REPORT MODE IS LOAD-BEARING: every breach below carries
 * `priority: "medium"`, never `"high"` — `VerifyScript.runGate`'s `dissolveBreaches` block (where these
 * are also wired in) filters to `priority === "high"` before throwing, so these five rules census the
 * fleet without failing the gate for the ~14 concurrent agents already running against it
 * (📌️important.md). Wave 3 (mass plugin migration) is what fixes what these rules find; Wave 5 flips
 * them to `"high"` once migration lands, the same staging `policyPluginRootShapeBreaches` itself used.
 */
const POLICY_APA_PLUGINS_ROOT = "✏️s/🔌️plugins";

//#region 🔧️PolicyRulePluginClosedShape
/** 🎫️Ticket-cited exceptions to the plugin-root closed-shape rule — empty; an entry requires a ticket citation in a comment beside it. */
const POLICY_PLUGIN_CLOSED_SHAPE_ALLOWLIST = new Set<string>();

/** 🗑️Filenames/dirnames that are always stray tooling junk at a plugin root, never legitimate content, regardless of plugin. */
const POLICY_PLUGIN_CLOSED_SHAPE_JUNK = new Set(["node_modules", ".DS_Store"]);

/**
 * 🗺️extra-entry → proposed APA destination, keyed `"<pluginOwnerRel>/<entryName>"` — transcribed from
 * 📓️w0-b-plugin-shape.md §5 / 📓️w0-census.md §3 §6 (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE).
 * Every plugin-root entry this table doesn't name falls back to a generic relocate-under-artifacts-or-apps
 * solution in `policyPluginClosedShapeBreaches` below.
 */
const POLICY_PLUGIN_CLOSED_SHAPE_DESTINATIONS: Readonly<Record<string, string>> = {
  "✏️s/🔌️plugins/🌀️procedural/🎮️play": "Fold into 🗿️artifacts/<kind>/📚️examples/ once confirmed non-placeholder (dir currently holds only AGENTS.md) — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🌊️flow/🔨️modules": "Move 🔨️modules/🧮️compute/🟦️.ts into 🗿️artifacts/<flowcompute-kind>/🏅️standards/🔖️1/⚙️engine/compute/ (artifact-kind name needs an owner decision) — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🌊️flow/🧩️extensions": "Extension-crate axis (role=extension, extends=flow, 9 crates) — sanctioned third axis pending the §6 ruling in 📓️w0-census.md; not auto-relocatable.",
  "✏️s/🔌️plugins/🌍️gis/🔨️modules": "Move 🔨️modules/🏔️terrain/🦀️.rs into 🗿️artifacts/gismap/🏅️standards/🔖️1/⚙️engine/terrain/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/➗️formulation": "Fold into 🗿️artifacts/fem/🏅️standards/🔖️1/⚙️engine/formulation/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/🏗️model": "Fold into 🗿️artifacts/fem/🏅️standards/🔖️1/⚙️engine/model/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/📏️elements2d": "Fold into 🗿️artifacts/fem/🏅️standards/🔖️1/⚙️engine/elements2d/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/🔢️sparse": "Fold into 🗿️artifacts/fem/🏅️standards/🔖️1/⚙️engine/sparse/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/🕸️mesh": "Fold into 🗿️artifacts/fem/🏅️standards/🔖️1/⚙️engine/mesh/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/🧊️elements3d": "Fold into 🗿️artifacts/fem/🏅️standards/🔖️1/⚙️engine/elements3d/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/🧮️analyses": "Fold into 🗿️artifacts/fem/🏅️standards/🔖️1/⚙️engine/analyses/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🏗️fem/🖥️app-surface": "CANNOT CLASSIFY under a current APA slot — shared by fem2d_ui and fem3d_ui, needs an explicit cross-app-shared-code ruling before relocation (bannedNameStems forbids the obvious core/shared/common fallback) — 📓️w0-census.md §6.",
  "✏️s/🔌️plugins/🏭️process/🧩️extensions": "Extension-crate axis (role=extension, extends=process, 4 crates) — pending the §6 ruling in 📓️w0-census.md.",
  "✏️s/🔌️plugins/📐️cad/🔨️modules": "Needs per-subdir inspection (14 files, not enumerated) — likely folds into 🗿️artifacts/<cad-artifact>/🏅️standards/…/⚙️engine/ per the gis/puzzle pattern — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/📐️cad/🧩️extensions": "Extension-crate axis (role=extension, extends=cad, 4 crates) — pending the §6 ruling in 📓️w0-census.md.",
  "✏️s/🔌️plugins/📕️norm/⚖️compliance": "Feeds all 15 norm standard artifacts — needs a cross-artifact-shared-engine ruling (candidate: 🗿️artifacts/norm/🏅️standards/🔖️shared/⚙️engine/core/) — 📓️w0-census.md §6.",
  "✏️s/🔌️plugins/📕️norm/🖥️app-surface": "Same cross-app-shared ruling as fem's 🖥️app-surface — 📓️w0-census.md §6.",
  "✏️s/🔌️plugins/📖️playbook/🧩️extensions": "Extension-crate axis (role=extension, extends=playbook, 1 crate) — pending the §6 ruling in 📓️w0-census.md.",
  "✏️s/🔌️plugins/📜️imperative/🧩️extensions": "Extension-crate axis (role=extension, extends=imperative, 5 crates) — pending the §6 ruling in 📓️w0-census.md.",
  "✏️s/🔌️plugins/🔋️energy/⚙️engine": "Single largest violation in the census — a 50-submodule headless HVAC engine sitting at plugin root → 🗿️artifacts/energy/🏅️standards/🔖️1/⚙️engine/<module>/ for each submodule — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🔱️trinity/🌳️ast": "Fold into 🗿️artifacts/<trinity-kind>/🏅️standards/🔖️1/⚙️engine/ast/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🔱️trinity/🔤️lexer": "Fold into 🗿️artifacts/<trinity-kind>/🏅️standards/🔖️1/⚙️engine/lexer/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🔱️trinity/🔨️modules": "23 files incl. jack/shell+jack/lsp → 🗿️artifacts/<kind>/🏅️standards/…/⚙️engine/jack/{shell,lsp}/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🔱️trinity/🗣️language-service": "Fold into 🗿️artifacts/<trinity-kind>/🏅️standards/🔖️1/⚙️engine/language-service/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🔱️trinity/🧮️executor": "Fold into 🗿️artifacts/<trinity-kind>/🏅️standards/🔖️1/⚙️engine/executor/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🖍️draw/🔄️fsm": "→ 🗿️artifacts/draw/🏅️standards/🔖️1/⚙️engine/fsm/; the nested ✨️macros sub-crate needs a crate-boundary specialist, not a plain directory move — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🧩️puzzle/🔨️modules": "Move 🔨️modules/🎲️board-2d/🦀️.rs into 🗿️artifacts/puzzle2d/🏅️standards/🔖️1/⚙️engine/board-2d/ — 📓️w0-b-plugin-shape.md §5.",
  "✏️s/🔌️plugins/🪵️sourcing/🧩️extensions": "Extension-crate axis (role=extension, extends=sourcing, 3 crates) — pending the §6 ruling in 📓️w0-census.md.",
  "🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions": "Nine Hub-owned deployment components compose the 176 Stdio artifact apps through the neutral contribution interface.",
  "✏️s/🔌️plugins/📐️cad/🔣️machine.json": "210KB root data file — CANNOT CLASSIFY without reading contents (likely a generated/vendored CAD-kernel data file) — 📓️w0-b-plugin-shape.md §5.",
};

/**
 * 📏️The three legacy per-plugin facets `taxonomy.pluginChildDirs` dropped (it now reads `["🎮️commands"]`
 * only, per the W3 dissolution) — every one of the 33 plugins still carries doc-only (or, for
 * gis/lowpoly/norm/stdio, real) content under these directory names; this is the "missing absence
 * check" `policyPluginRootShapeBreaches` never performed (it only ever checks presence of required
 * facets, never flags an extra one).
 */
const POLICY_PLUGIN_CLOSED_SHAPE_LEGACY_FACETS: Readonly<Record<string, string>> = {
  "🎟️capabilities": "Doc-only stub in all 33 plugins (zero real .capability() calls repo-wide) — delete. The one real capability call (🪐️space's .local_backbone_storage()) already lives at plugin root, not this facet.",
  "🔧️setup": "30/33 are 1-line doc-only stubs — delete. 🌍️gis/💠️lowpoly/📕️norm carry real fan-out code (register_gis_exports/register_lowpoly_exports/register_norm_exports) — fold each into the owning artifact's own ⚙️engine registration path (or the M1 declarative ArtifactDeclaration mechanism once it lands), not a standalone plugin-root facet.",
  "🛂️manifest": "32/33 are 1-line doc-only stubs — delete. 🗄️stdio's 344-line stdio_format_descriptors() catalog (region 🔖️FormatCatalog) is real declared data and belongs under 🗿️artifacts/<kind>/…, not this facet.",
};

/**
 * 📏️Plugin-root closed-shape rule: for every owner under `✏️s/🔌️plugins/`, every direct child entry NOT
 * in the taxonomy-derived allowed set (`pluginChildDirs` ∪ `artifactsDirName` ∪ `packagesDirName` ∪ the
 * root component leaf ∪ `rootDocFileNames` ∪ `rootDataDirNames` ∪ `rootDataFileNames` — the last two are
 * legal ONLY at an owner root per taxonomy.json's own `_treePurityComment`, which is why 🗄️stdio's
 * `📇️registry`, 📐️cad's `🖼️assets`/`🧫️fixtures`, and every plugin's root `🛂️manifest.json` are excluded
 * below rather than flagged) is a shape breach. The allowed set is derived from `🔣️taxonomy.json` via
 * `loadTaxonomy()` rather than hardcoded, per the ticket's instruction — a dedicated `pluginRootAllowedEntries`
 * taxonomy key was deliberately NOT added: every field this rule needs already exists on `Taxonomy`
 * (`pluginChildDirs`/`artifactsDirName`/`packagesDirName`/`rootDocFileNames`/`rootDataDirNames`/
 * `rootDataFileNames`), and `🔣️taxonomy.json`/`🔍️discovery/🟦️.ts`/`🧪️index.test.ts` are outside
 * this session's single-writer boundary on `📜️script.ts` — adding a key there risks colliding with W1's
 * concurrent taxonomy work for no expressive gain.
 */
function policyPluginClosedShapeBreaches(repoRoot: string): BreachRecord[] {
  const taxonomy = loadTaxonomy();
  const allowedDirs = new Set<string>([...taxonomy.pluginChildDirs, taxonomy.artifactsDirName, taxonomy.packagesDirName, ...taxonomy.rootDataDirNames]);
  const allowedFiles = new Set<string>([
    taxonomyMappedFilename(taxonomy, taxonomy.componentFileKinds, "🦀️rust"),
    ...taxonomyContractFilenames(taxonomy, taxonomy.rootDocumentContractIds),
    ...taxonomyContractFilenames(taxonomy, taxonomy.rootDataContractIds),
  ]);
  const breaches: BreachRecord[] = [];
  let owners: Dirent[];
  try {
    owners = readdirSync(join(repoRoot, POLICY_APA_PLUGINS_ROOT), { withFileTypes: true });
  } catch {
    return breaches;
  }
  for (const owner of owners) {
    if (!owner.isDirectory()) continue;
    const ownerRel = `${POLICY_APA_PLUGINS_ROOT}/${owner.name}`;
    let children: Dirent[];
    try {
      children = readdirSync(join(repoRoot, ownerRel), { withFileTypes: true });
    } catch {
      continue;
    }
    for (const child of children) {
      const childRel = `${ownerRel}/${child.name}`;
      if (POLICY_PLUGIN_CLOSED_SHAPE_ALLOWLIST.has(childRel)) continue;
      if (child.isDirectory()) {
        if (allowedDirs.has(child.name)) continue;
        if (POLICY_PLUGIN_CLOSED_SHAPE_JUNK.has(child.name)) {
          breaches.push({
            id: `plugin-closed-shape-junk-${childRel}`,
            summary: `"${childRel}" is stray tooling junk at a plugin root`,
            kind: "taxonomy/plugin-closed-shape",
            scope: ownerRel,
            priority: "medium",
            reason: "APA: a plugin is EXACTLY 🗿️artifacts + root wiring — build caches and OS junk never belong at plugin root.",
            solution: `Delete ${childRel}; add "${child.name}" to the repo .gitignore if it keeps recurring.`,
          });
          continue;
        }
        const legacy = POLICY_PLUGIN_CLOSED_SHAPE_LEGACY_FACETS[child.name];
        const solution =
          legacy ??
          POLICY_PLUGIN_CLOSED_SHAPE_DESTINATIONS[childRel] ??
          `Relocate ${childRel} under ${ownerRel}/${taxonomy.artifactsDirName}/<kind>/<standard>/🪆️subsets/<subset>/{👁️viewer,✏️editor}/ — see 📓️w0-b-plugin-shape.md §5 for the nearest analogous mapping, or file a "needs ruling" note if none fits.`;
        breaches.push({
          id: `plugin-closed-shape-dir-${childRel}`,
          summary: `"${childRel}" is a plugin-root entry outside the closed artifacts shape`,
          kind: "taxonomy/plugin-closed-shape",
          scope: ownerRel,
          priority: "medium",
          reason: ("APA: a plugin is EXACTLY 🗿️artifacts + root " + POLICY_RS_COMPONENT_LEAF_NAME + "/AGENTS.md/README.md + 📦️packages wiring — every other direct child is a shape violation, not merely a missing-facet gap (which is all policyPluginRootShapeBreaches ever checked)."),
          solution,
        });
        continue;
      }
      if (allowedFiles.has(child.name)) continue;
      breaches.push({
        id: `plugin-closed-shape-file-${childRel}`,
        summary: `"${childRel}" is a plugin-root file outside the closed apps+artifacts shape`,
        kind: "taxonomy/plugin-closed-shape",
        scope: ownerRel,
        priority: "medium",
        reason: "APA: plugin-root files are limited to the leaf component, taxonomy.rootDocFileNames, and taxonomy.rootDataFileNames — a stray data file at root is a shape violation.",
        solution: POLICY_PLUGIN_CLOSED_SHAPE_DESTINATIONS[childRel] ?? `Classify and relocate ${childRel} under 🗿️artifacts or 🎛️apps, or add it to taxonomy.rootDataFileNames if it is genuinely owner-root data.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRulePluginClosedShape

//#region 🔧️PolicyRulePluginPurity
/** 🎫️Ticket-cited exceptions — empty; an entry requires a ticket citation in a comment beside it. */
const POLICY_PLUGIN_PURITY_ALLOWLIST = new Set<string>();

/** 🗂️`fs::`-word-bounded catches both `std::fs::x` and the common `use std::fs; fs::x` idiom this tree actually uses (📓️w0-c-purity.md §1), plus tokio::fs/read_dir/read_to_string/File::. */
const POLICY_PLUGIN_PURITY_FS_RE = /\b(?:std::fs::|tokio::fs::|fs::|read_dir\s*\(|read_to_string\s*\(|File::)/;
const POLICY_PLUGIN_PURITY_ENV_RE = /\b(?:std::env::|std::process::|Command::new\s*\(|temp_dir\s*\()/;
const POLICY_PLUGIN_PURITY_NET_RE = /\b(?:std::net::|reqwest::|ureq::|hyper::|TcpStream\b)/;
const POLICY_PLUGIN_PURITY_THREAD_LOCAL_RE = /\bthread_local!\s*(?:\{|\()/;
const POLICY_PLUGIN_PURITY_STATIC_MUT_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?static\s+mut\b/;
/** 🔓️A `static` item's own declaration line is item-scope even when the item is lexically nested inside a `fn` body (the `fn next_id() { static COUNTER: AtomicU64 = …; }` monotonic-id-counter idiom this tree uses 15+ times) — `static` denotes a persistent 'static-duration item in Rust regardless of lexical nesting, so this overrides the `!inFn` gate for that one line. */
const POLICY_PLUGIN_PURITY_STATIC_ITEM_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?static\s+(?:mut\s+)?\w+\s*:/;
const POLICY_PLUGIN_PURITY_LAZY_STATIC_RE = /\blazy_static!\s*\{/;
/** 🔒️Item-scope interior-mutability types that are genuinely mutated after construction — deliberately excludes bare OnceLock/OnceCell/LazyLock (the sanctioned write-once lazy-table pattern every artifact io_registry uses); a nested `OnceLock<Mutex<...>>` still matches because `Mutex<` is present in the same line. */
const POLICY_PLUGIN_PURITY_INTERIOR_MUT_RE = /\b(Mutex|RwLock|RefCell|Cell)\s*<|\b(Atomic(?:Bool|I8|I16|I32|I64|Isize|U8|U16|U32|U64|Usize))\b/;
const POLICY_PLUGIN_PURITY_TS_RE = /\b(fetch\s*\(|XMLHttpRequest\b|WebSocket\b|localStorage\b|sessionStorage\b|indexedDB\b)/;
const POLICY_PLUGIN_PURITY_FN_ITEM_RE = /^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:const\s+)?fn\s+\w/;

/** 🧪️Brace-span of a `#[test]`/`#[tokio::test]`/`#[async_std::test]`/`#[wasm_bindgen_test]`-attributed fn body — sanctioned test-only IO the purity rule must skip in addition to `#[cfg(test)] mod …` spans (`policyTestModSpans`, reused as-is). */
function policyPluginPurityTestFnSpans(lines: readonly string[]): PolicyModSpan[] {
  const spans: PolicyModSpan[] = [];
  let depth = 0;
  let pendingTestAttr = false;
  let recording = false;
  let recordDepth = 0;
  let openLine = -1;
  lines.forEach((raw, i) => {
    const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
    if (!recording) {
      if (/^\s*#\[(?:test|tokio::test|async_std::test|wasm_bindgen_test)\b/.test(raw)) {
        pendingTestAttr = true;
      } else if (pendingTestAttr && /^\s*#\[/.test(raw)) {
        // stacked attribute — keep waiting for the fn line
      } else if (pendingTestAttr && /\bfn\s+\w/.test(raw)) {
        if (/\{/.test(codeOnly)) {
          recording = true;
          recordDepth = depth;
          openLine = i + 1;
        }
        pendingTestAttr = false;
      } else if (pendingTestAttr) {
        pendingTestAttr = false;
      }
    }
    depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
    if (recording && depth <= recordDepth) {
      spans.push({ name: "test-fn", startLine: openLine, endLine: i + 1 });
      recording = false;
    }
  });
  return spans;
}

/** 🔎️Every `.ts`/`.tsx` file under `rootRel`, excluding the repo's sole TS test convention (`🧪️tests/` dirs, `🟦️.ts` files) — the same split 📓️w0-c-purity.md's scout used, confirmed a 100%-clean split there. */
function policyPluginPurityTsFiles(repoRoot: string, rootRel: string): string[] {
  const found: string[] = [];
  const walk = (relDir: string): void => {
    const abs = join(repoRoot, relDir);
    let entries: Dirent[];
    try {
      entries = readdirSync(abs, { withFileTypes: true });
    } catch {
      return;
    }
    for (const ent of entries) {
      const childRel = `${relDir}/${ent.name}`;
      if (ent.isDirectory()) {
        if (POLICY_SKIP_DIRS.has(ent.name) || ent.name === "🧪️tests") continue;
        walk(childRel);
        continue;
      }
      if (ent.name === "🟦️.ts") continue;
      if (ent.name.endsWith(".ts") || ent.name.endsWith(".tsx")) found.push(childRel);
    }
  };
  walk(rootRel);
  return found.sort();
}

//#region 🔖️StateLaneExhaustiveness
/**
 * 🫧️Files that may touch browser/native storage directly, because they ARE a sanctioned storage owner:
 * - `🖥️platform` is the config lane's persistence adapter (`StoragePort`).
 * - `🏪️store/👷️worker/🟦️.ts` backs `BlobStore` — content-addressed ARTIFACT content (what a
 *   `LinkPin::Snapshot` escrows), which belongs to the artifact mechanism, not to config. Flagging it
 *   would be a category error: it is not local-only UI state routing around a lane.
 */
const POLICY_STATE_LANE_CONFIG_ADAPTER_PREFIXES = ["🧰️framework/🔨️modules/🖥️platform/", "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👷️worker/🟦️.ts"];

/** 🫧️Trees excluded from the lane rules: the parallel `compose/` stack and product/site code that is not an app. */
const POLICY_STATE_LANE_EXCLUDED_PREFIXES = ["compose/", "♻️mit-bestand/", "🌎️hub/", "🧰️framework/🛍️products/🦑️repo/"];

/** 🫧️Direct browser/native persistence — legal only inside the config-lane adapter. */
const POLICY_STATE_LANE_STORAGE_RE = /\b(?:localStorage|sessionStorage|indexedDB)\b/;

/** 🫧️The TS process-local ephemeral boxes the Transient lane replaces. */
const POLICY_STATE_LANE_EPHEMERAL_BOX_RE = /\bephemeral(?:Box|Map|Set|WeakMap)\b/;

/**
 * 📏️State-lane exhaustiveness: exactly FOUR state mechanisms exist, and nothing may route around them —
 * **artifacts** (persisted + shared), **config** (persisted + local-only), **presence** (ephemeral + shared),
 * **transient** (ephemeral + local-only UI). Ticket `26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION`.
 *
 * 🎯️ The lanes became reachable in wave A1 (`PresenceStore`/`TransientStore`, `EphemeralEmit`,
 * `ArtifactApp::ephemeral`). This rule is the other half of the mandate — "enforced by api AND policies":
 * an available lane that nothing forces you to use just becomes a fifth option beside the ad-hoc ones.
 *
 * Two sub-kinds, both REPORT-MODE at medium priority (they measure a real, large existing surface —
 * ~158 `ephemeralBox*` uses and the shell's direct `localStorage` writes — which wave A4 retires):
 * - `storage-outside-config-lane`: direct `localStorage`/`sessionStorage`/`indexedDB` outside the
 *   `🖥️platform` adapter. Persisted local-only state IS the config lane; the medium is an adapter
 *   detail, so bypassing the lane is what is banned, never the API itself.
 * - `ephemeral-box`: the `ephemeralBox`/`ephemeralMap`/`ephemeralSet`/`ephemeralWeakMap` helpers in
 *   `🎠️kernel/🟦️.ts`, whose own docstring calls them the "sole lane until the OS draft
 *   snapshot owns these keys". The Transient lane now owns them.
 */
export function policyStateLaneExhaustivenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const roots = ["🧰️framework", "✏️s"];
  const seen = new Set<string>();

  for (const root of roots) {
    for (const relPath of policyPluginPurityTsFiles(repoRoot, root)) {
      if (seen.has(relPath)) continue;
      seen.add(relPath);
      if (POLICY_STATE_LANE_EXCLUDED_PREFIXES.some((prefix) => relPath.startsWith(prefix))) continue;
      // 🧪️ Test files are exempt: a test that drives the storage adapter, or asserts that an
      // ephemeral box was retired, must be able to NAME the thing it is testing.
      if (/\.test\.tsx?$/.test(relPath) || relPath.includes("/🧪️")) continue;
      const isConfigAdapter = POLICY_STATE_LANE_CONFIG_ADAPTER_PREFIXES.some((prefix) => relPath.startsWith(prefix));
      const content = policyReadFileSafe(repoRoot, relPath);
      if (!content) continue;

      content.split(/\r?\n/).forEach((raw, i) => {
        const lineNo = i + 1;
        // 🎯️ Mask literals, then strip BOTH comment forms: this rule greps raw source, and prose
        // explaining that a helper was retired must not itself trip the rule — a hazard this repo's
        // vocabulary policies have already been bitten by. `//` alone is not enough; the block-comment
        // continuation (` * …`) is exactly where a docstring mentioning `localStorage` lives.
        const trimmed = raw.trimStart();
        if (trimmed.startsWith("*") || trimmed.startsWith("/*") || trimmed.startsWith("//")) return;
        const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "").replace(/\/\*.*?\*\//g, "");

        if (!isConfigAdapter && POLICY_STATE_LANE_STORAGE_RE.test(codeOnly)) {
          breaches.push({
            id: `state-lane-storage-${relPath}-${lineNo}`,
            summary: `"${relPath}:${lineNo}" persists local state outside the config lane`,
            kind: "taxonomy/state-lane-storage-outside-config-lane",
            scope: relPath,
            line: lineNo,
            priority: "medium",
            reason: "Only four state mechanisms exist. Persisted local-only state IS the config lane; a direct localStorage/sessionStorage/indexedDB write is a fifth, untyped, unvalidated one that no schema facet describes and no policy can check the shape of.",
            solution: `Move the state at ${relPath}:${lineNo} into the app's (or the OS shell's) config record and let the 🖥️platform StoragePort adapter persist it. The storage MEDIUM stays legal — bypassing the lane is what does not.`,
          });
        }

        if (POLICY_STATE_LANE_EPHEMERAL_BOX_RE.test(codeOnly)) {
          breaches.push({
            id: `state-lane-ephemeral-box-${relPath}-${lineNo}`,
            summary: `"${relPath}:${lineNo}" uses a process-local ephemeral box`,
            kind: "taxonomy/state-lane-ephemeral-box",
            scope: relPath,
            line: lineNo,
            priority: "medium",
            reason: "The ephemeralBox/Map/Set/WeakMap helpers are untyped process-local state keyed by string — their own docstring calls them the sole lane 'until the OS draft snapshot owns these keys'. The Transient lane (ephemeral + local-only) now owns exactly that role, typed and dispatched.",
            solution: `Replace the ephemeral box at ${relPath}:${lineNo} with the app's Transient lane (ArtifactApp::Transient + ArtifactApp::ephemeral), or — if the value is genuinely render-internal (no plugin render depends on it, no second surface reads it, losing it on re-render breaks only a visual nicety, and it is never persisted) — keep it as ordinary component-local state instead of a module-level box.`,
          });
        }
      });
    }
  }

  return breaches;
}
//#endregion 🔖️StateLaneExhaustiveness

/**
 * 📏️Plugin purity: every `.rs` file under `✏️s/🔌️plugins/` may not perform filesystem, env/process, or
 * network IO, nor own item-scope ambient mutable state (`thread_local!`/`static mut`/`lazy_static!`/
 * item-scope `Mutex`/`RwLock`/`RefCell`/`Cell`/`Atomic*`) — skips `#[cfg(test)]` modules and `#[test]`
 * functions (test-only IO is currently sanctioned, same skip 📓️w0-c-purity.md's census applied). Also
 * scans `.ts`/`.tsx` under the plugin tree for direct browser-side IO (`fetch(`/`XMLHttpRequest`/
 * `WebSocket`/`localStorage`/`sessionStorage`/`indexedDB`).
 */
function policyPluginPurityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];

  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.startsWith(`${POLICY_APA_PLUGINS_ROOT}/`)) continue;
    if (POLICY_PLUGIN_PURITY_ALLOWLIST.has(relPath)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    const lines = content.split(/\r?\n/);
    const skipSpans = [...policyTestModSpans(lines), ...policyPluginPurityTestFnSpans(lines)];
    let depth = 0;
    const fnBodyDepths: number[] = [];

    lines.forEach((raw, i) => {
      const lineNo = i + 1;
      const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
      if (policyLineInTestMod(skipSpans, lineNo)) {
        depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
        while (fnBodyDepths.length > 0 && depth < fnBodyDepths[fnBodyDepths.length - 1]!) fnBodyDepths.pop();
        return;
      }
      const inFn = fnBodyDepths.some((d) => depth >= d);
      const opensFn = !inFn && POLICY_PLUGIN_PURITY_FN_ITEM_RE.test(raw) && /\{/.test(codeOnly);

      const emit = (subKind: string, summaryVerb: string, reason: string, solution: string) => {
        breaches.push({
          id: `plugin-purity-${subKind}-${relPath}-${lineNo}`,
          summary: `"${relPath}:${lineNo}" ${summaryVerb} inside a plugin tree`,
          kind: `taxonomy/plugin-purity-${subKind}`,
          scope: relPath,
          line: lineNo,
          priority: "medium",
          reason,
          solution,
        });
      };

      if (POLICY_PLUGIN_PURITY_FS_RE.test(codeOnly)) {
        emit(
          "filesystem-io",
          "performs filesystem IO",
          "APA: all io/state changes belong to artifacts, never apps/setup facets or non-taxonomy subtrees — filesystem access outside an artifact's own ⚙️engine is a purity violation.",
          `Move the filesystem call at ${relPath}:${lineNo} behind an artifact-owned ⚙️engine, or (for a build.rs/bin.rs sibling) confirm with the ticket owner whether APA's shape rule reaches compile-time/CLI code at all — 📓️w0-c-purity.md §1 flags this as UNVERIFIED.`,
        );
      }
      if (POLICY_PLUGIN_PURITY_ENV_RE.test(codeOnly)) {
        emit(
          "env-process-io",
          "reads env/process state",
          "APA: environment/process access is a side effect that must be routed through the OS host, not read directly by plugin code.",
          `Move the env/process call at ${relPath}:${lineNo} behind an artifact-owned config path or an OS host capability.`,
        );
      }
      if (POLICY_PLUGIN_PURITY_NET_RE.test(codeOnly)) {
        emit(
          "network-io",
          "performs network IO",
          "APA: network IO must be routed through the OS host's capability-gated network-fetch effect, never called directly from plugin code.",
          `Replace the direct network call at ${relPath}:${lineNo} with a capability-gated HostEffect (e.g. OpenExternalUrl/DownloadMediaExport), or route it through an artifact's own io.`,
        );
      }

      if (!inFn || POLICY_PLUGIN_PURITY_STATIC_ITEM_RE.test(raw)) {
        if (POLICY_PLUGIN_PURITY_THREAD_LOCAL_RE.test(codeOnly)) {
          emit(
            "thread-local-state",
            "declares thread_local! ambient state",
            "APA: thread_local! has no framework-sanctioned owner in a plugin — every app that needed session state independently reinvented this because ArtifactApp gave it none. That gap is now closed: the four state mechanisms (artifact/config/presence/transient) are all typed lanes on ArtifactApp.",
            `Replace thread_local! at ${relPath}:${lineNo} with the lane that matches what the state actually IS — ephemeral local UI state → the Transient lane (ArtifactApp::Transient, emitted via ArtifactApp::ephemeral); ephemeral shared state → Presence; uncommitted document content → the Draft lane; a cached view of an owned CHILD's content → do not cache at all, read it through ArtifactView.children, which cannot go stale (see 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION/📓️cw2-child-cache-finding.md for the one case still blocked: codecs take no context, so a child-content cache cannot be deleted until the parent's serialized form drops resolved child content in favour of the handle).`,
          );
        }
        if (POLICY_PLUGIN_PURITY_STATIC_MUT_RE.test(raw)) {
          emit(
            "static-mut-state",
            "declares item-scope static mut",
            "APA: item-scope static mut is forbidden ambient state in a plugin tree.",
            `Delete the static mut at ${relPath}:${lineNo} and route ownership through the artifact's own engine or the OS host.`,
          );
        }
        if (POLICY_PLUGIN_PURITY_LAZY_STATIC_RE.test(codeOnly)) {
          emit(
            "lazy-static-state",
            "declares lazy_static! ambient state",
            "APA: lazy_static! process globals are forbidden outside an artifact-owned write-once table.",
            `Replace lazy_static! at ${relPath}:${lineNo} with std OnceLock/LazyLock if genuinely write-once, or route real mutation through the artifact engine.`,
          );
        }
        const interior = codeOnly.match(POLICY_PLUGIN_PURITY_INTERIOR_MUT_RE);
        if (interior) {
          const ty = (interior[1] ?? interior[2])!;
          const tyLower = ty.toLowerCase();
          emit(
            `interior-mutability-${tyLower}`,
            `declares item-scope ${ty} ambient state`,
            "APA: item-scope Mutex/RwLock/RefCell/Cell/Atomic* is real mutable ambient state, not the sanctioned write-once OnceLock/OnceCell/LazyLock lazy-table pattern (which is exempt on its own — only nested Mutex/RwLock/RefCell/Cell inside one still counts).",
            `Move the ${ty} state at ${relPath}:${lineNo} behind the artifact's own dispatch path (Mutex<Vec<...>> contribution registries) or a typed Draft lane (per-app session/scratch state) — see 📓️w0-c-purity.md §4 for the sanctioned-vs-real classification this rule mirrors.`,
          );
        }
      }

      if (opensFn) fnBodyDepths.push(depth + 1);
      depth += (codeOnly.match(/\{/g) ?? []).length - (codeOnly.match(/\}/g) ?? []).length;
      while (fnBodyDepths.length > 0 && depth < fnBodyDepths[fnBodyDepths.length - 1]!) fnBodyDepths.pop();
    });
  }

  for (const relPath of policyPluginPurityTsFiles(repoRoot, POLICY_APA_PLUGINS_ROOT)) {
    if (POLICY_PLUGIN_PURITY_ALLOWLIST.has(relPath)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    const lines = content.split(/\r?\n/);
    lines.forEach((raw, i) => {
      const lineNo = i + 1;
      const codeOnly = raw.replace(/\/\/.*$/, "");
      const m = codeOnly.match(POLICY_PLUGIN_PURITY_TS_RE);
      if (!m) return;
      breaches.push({
        id: `plugin-purity-ts-side-effect-${relPath}-${lineNo}`,
        summary: `"${relPath}:${lineNo}" calls ${m[1]} directly from a plugin app-tree file`,
        kind: "taxonomy/plugin-purity-ts-side-effect",
        scope: relPath,
        line: lineNo,
        priority: "medium",
        reason: "APA: browser-side IO/ambient-storage APIs (fetch/XMLHttpRequest/WebSocket/localStorage/sessionStorage/indexedDB) must be routed through an artifact, never called directly from an app-tree component — 📓️w0-c-purity.md §3 calls animate's two fetch() sites the cleanest violation in the whole census.",
        solution: `Route the call at ${relPath}:${lineNo} through an artifact-owned io path instead of calling it directly from the app component.`,
      });
    });
  }

  return breaches;
}
//#endregion 🔧️PolicyRulePluginPurity

//#region 🔧️PolicyRuleDeclarativeRegistration
/** 🎫️Ticket-cited exceptions — empty; an entry requires a ticket citation in a comment beside it. */
const POLICY_DECLARATIVE_REGISTRATION_ALLOWLIST = new Set<string>();

/**
 * 📇️Exhaustive OS-host global-registry-mutator family — union of the assignment's given list, the
 * `register_*` family from 📓️w0-a-escape-hatch.md §1 (`register_os_fixture_json`, and the
 * `register_artifact_descriptor(s)` pair used by `register_app_io`/hot-swap), per §6 of
 * 📓️w0-d-sdk-surface.md. Deliberately excludes `register_studio_port` (confirmed plugin-local to
 * 🪐️space, not a framework SDK fn — w0-d §6) and plain `register_app` (`&mut self` method on
 * `PluginRegistry`, not a free-function global-static mutator — w0-a §1).
 */
const POLICY_REGISTRATION_FAMILY_FNS = [
  "register_mesh_exporter",
  "register_mesh_importer",
  "register_mesh_dwg_export_handler",
  "register_mesh_dwg_import_handler",
  "register_solid_exporter",
  "register_solid_importer",
  "register_2d_export_handlers",
  "register_dwg_import_handler",
  "register_app_io",
  "register_os_media_export_handler_kind",
  "register_os_media_import_handler_kind",
  "register_composer_entries",
  "set_io_fallback_dispatcher",
  "register_subset_validator",
  "register_format_descriptors",
  "register_artifact_schema_descriptor",
  "register_artifact_inference_descriptor",
  "register_app_schema_descriptor",
  "register_language",
  "register_document_codec",
  "register_document_codec_for_app",
  "register_dialect_migration",
  "register_os_fixture_json",
  "register_artifact_descriptors",
  "register_artifact_descriptor",
] as const;
const POLICY_REGISTRATION_FAMILY_RE = new RegExp(String.raw`\b(${POLICY_REGISTRATION_FAMILY_FNS.join("|")})\b`);
const POLICY_REGISTRATION_OS_PATH_RE = /\bsemio_framework_os::/;

/** 🧭️True when `relPath` sits inside a `🗿️artifacts/<kind>/…/⚙️engine/…` subtree — the sole currently-compliant interim registration site per 📓️w0-a-escape-hatch.md §2a. */
function policyRegistrationIsEngineSite(relPath: string): boolean {
  const artifactsIdx = relPath.indexOf("/🗿️artifacts/");
  if (artifactsIdx === -1) return false;
  return relPath.indexOf("/⚙️engine/", artifactsIdx) !== -1;
}

/** 🏗️Builds the (identical-shape) engine-backlog vs violation BreachRecord for a registration-family/os-path hit. */
function policyRegistrationBreach(relPath: string, lineNo: number, isEngine: boolean, subKind: string, what: string): BreachRecord {
  if (isEngine) {
    return {
      id: `plugin-registration-engine-backlog-${subKind}-${relPath}-${lineNo}`,
      summary: `"${relPath}:${lineNo}" calls ${what} from inside its own artifact ⚙️engine — currently-compliant interim shape, migration backlog`,
      kind: "taxonomy/plugin-registration-engine-backlog",
      scope: relPath,
      line: lineNo,
      priority: "medium",
      reason: "APA (w0-a §2a): a registration call inside the owning artifact's own ⚙️engine is the sanctioned interim shape until the M1/M2 declarative ArtifactDeclaration/Registrar mechanism lands — tracked as migration backlog, not a live architecture violation.",
      solution: `Once M1's ArtifactDeclaration/Registrar mechanism lands, convert ${relPath}:${lineNo} from an imperative ${what} call into a declarative .artifact(...) entry.`,
    };
  }
  return {
    id: `plugin-registration-violation-${subKind}-${relPath}-${lineNo}`,
    summary: `"${relPath}:${lineNo}" calls ${what} outside its owning artifact's ⚙️engine — wrong layer for a registration call`,
    kind: "taxonomy/plugin-registration-violation",
    scope: relPath,
    line: lineNo,
    priority: "medium",
    reason: 'APA: registration/IO for a kind belongs to that kind\'s own artifact ⚙️engine — a 🔧️setup facet, app file, pane, panel, command handler, or plugin root calling the global registration family (or reaching into semio_framework_os::) mutates OS-host state from the wrong layer, and can register a kind the caller doesn\'t even own (💠️lowpoly registering "3d.mesh" — w0-a §2b-§2d; the 🎪️demonstrator half of that exemplar is RESOLVED, ticket 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION D2 moved its process/procedural/gis/cad registrations to those owners and dropped its semio-framework-os dependency, so a demonstrator hit here is now a regression, not a known backlog).',
    solution: `Delete the ${what} call at ${relPath}:${lineNo} and move the registration into the owning artifact's own 🗿️artifacts/<kind>/…/⚙️engine/ (or, for register_app_io/register_os_fixture_json, into the app's own declarative registration path once M1 lands).`,
  };
}

/**
 * 📏️Declarative registration: under `✏️s/🔌️plugins/`, flags (a) `.setup(` inside a `Plugin::builder(…)`
 * chain — registration must be declarative data, not an imperative callback (M1 retires `PluginBuilder`'s
 * `setup: Option<fn()>`); (b) any call to the exhaustive global registration family; (c) any
 * `semio_framework_os::` path reference (the OS host crate is forbidden to plugins). (b)/(c) are split into
 * two separable kinds: a call inside the owning artifact's own `⚙️engine` is the currently-compliant
 * interim shape (migration backlog); the same call anywhere else (`🔧️setup/`, `🎛️apps/`, `📌️panels/`,
 * `🎮️commands/`, or the plugin root) is a live architecture violation. (`🎪️panes/` used to head that
 * list; the repo's only pane facet, 🎪️demonstrator's, was dissolved into `🎛️apps` by ticket
 * 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION D3.)
 */
function policyDeclarativeRegistrationBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];

  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.startsWith(`${POLICY_APA_PLUGINS_ROOT}/`)) continue;
    if (POLICY_DECLARATIVE_REGISTRATION_ALLOWLIST.has(relPath)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);

    const builderRe = /Plugin::builder\s*\(/g;
    let bm: RegExpExecArray | null;
    while ((bm = builderRe.exec(content))) {
      const buildIdx = content.indexOf(".build()", bm.index);
      const chainEnd = buildIdx === -1 ? Math.min(content.length, bm.index + 4000) : buildIdx;
      const chain = content.slice(bm.index, chainEnd);
      const setupMatch = chain.match(/\.setup\s*\(/);
      if (!setupMatch) continue;
      const idx = bm.index + setupMatch.index!;
      const lineNo = policyLineOfIndex(content, idx);
      breaches.push({
        id: `plugin-registration-setup-callback-${relPath}-${lineNo}`,
        summary: `"${relPath}:${lineNo}" registers via .setup(...) callback inside Plugin::builder — registration must be declarative data, not an imperative hook`,
        kind: "taxonomy/plugin-registration-setup-callback",
        scope: relPath,
        line: lineNo,
        priority: "medium",
        reason: "APA/W1 (📓️w1-mechanism-design.md M1): PluginBuilder's setup: Option<fn()> + .setup(...) + the `if let Some(setup) = self.setup { setup(); }` call are retired in favor of declarative ArtifactDeclaration data walked by build() in a fixed order — an imperative callback can register anything in any order, defeating the ownership check the declarative form enforces structurally.",
        solution: `Replace .setup(...) at ${relPath}:${lineNo} with .artifact(ArtifactDeclaration { .. }) entries once the M1 declarative registration mechanism lands (W1); until then this is tracked, not blocking.`,
      });
    }

    const lines = content.split(/\r?\n/);
    const isEngine = policyRegistrationIsEngineSite(relPath);
    lines.forEach((raw, i) => {
      const lineNo = i + 1;
      const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
      const fnMatch = codeOnly.match(POLICY_REGISTRATION_FAMILY_RE);
      if (fnMatch) breaches.push(policyRegistrationBreach(relPath, lineNo, isEngine, "family-call", `${fnMatch[1]}(...)`));
      if (POLICY_REGISTRATION_OS_PATH_RE.test(codeOnly)) breaches.push(policyRegistrationBreach(relPath, lineNo, isEngine, "os-host-path", "semio_framework_os::"));
    });
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleDeclarativeRegistration

//#region 🔧️PolicyRulePluginDependencyAllowlist
/** 🎫️Ticket-cited exceptions — empty; an entry requires a ticket citation in a comment beside it. */
const POLICY_PLUGIN_DEP_ALLOWLIST = new Set<string>();

/** ✅️What a plugin legitimately needs per 📓️w0-d-sdk-surface.md §3.1: the plugin SDK, the os-kernel crate, the 3d crates (base plus the four brep crates of the wave E4 cut) where genuinely used, and schema. Everything else pointing into 🧰️framework/ is a bypass of the curated SDK surface. */
const POLICY_PLUGIN_DEP_FRAMEWORK_ALLOWLIST = new Set(["semio-framework-plugin", "semio-framework-os-kernel", "semio-framework-3d", "semio-framework-3d-brep-polynomial", "semio-framework-3d-brep-vector", "semio-framework-3d-brep-tolerance", "semio-framework-3d-brep-curve", "semio-framework-3d-brep-surface", "semio-framework-3d-brep-representation", "semio-framework-3d-brep-intersect", "semio-framework-3d-brep-queries", "semio-framework-3d-brep-tessellation", "semio-framework-3d-brep-validation", "semio-framework-3d-brep-analysis", "semio-framework-3d-brep-operations", "semio-framework-3d-brep-engine", "semio-framework-schema"]);
/** ✅️serde-family third-party crates are always allowed regardless of how they're declared (version-pinned, not path-pointed at 🧰️framework, so they never reach this rule's path filter anyway — kept for defense in depth). */
const POLICY_PLUGIN_DEP_SERDE_RE = /^serde(_json|-wasm-bindgen)?$/;

/**
 * 📇️Exact `semio_framework_os::` symbols each plugin's own `.rs` files use (📓️w0-d-sdk-surface.md §3.2)
 * — keyed by plugin owner dir name, folded into the breach `solution` so the SDK re-export fix is
 * obvious from the breach alone. An empty array means the dependency is declared but 0 uses were found
 * (UNVERIFIED why — w0-d flags ✒️writer/📐️cad/🔱️trinity/🖍️draw as likely stale/unused declarations).
 */
const POLICY_PLUGIN_DEP_OS_SYMBOLS: Readonly<Record<string, readonly string[]>> = {
  "✒️writer": [],
  "🌀️procedural": ["register_mesh_dwg_import_handler"],
  "🌍️gis": ["DwgColor", "DwgEntity", "register_2d_export_handlers", "register_dwg_import_handler"],
  // 🎪️ BANNED, not downgraded (ticket 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION
  // D2): the demonstrator's twelve foreign-kind registrations moved to their owners (📐️cad, 🌍️gis),
  // and `semio-framework-os` is gone from its Cargo.toml. The empty array is the ratchet — if the
  // dependency is ever reintroduced, `policyPluginDependencyAllowlistBreaches` flags it with no
  // carve-out to soften the message.
  "🎪️demonstrator": [],
  "🏭️process": ["register_mesh_dwg_import_handler"],
  "📏️layout": ["DwgColor", "DwgDrawing", "DwgEntity", "DwgGeometry"],
  "📐️cad": ["register_dwg_import_handler", "register_mesh_dwg_export_handler", "register_mesh_exporter", "register_mesh_importer", "register_solid_exporter", "register_solid_importer"],
  "🎥️shooting": ["rasterize_svg_to_png_base64"],
  "🎞️animate": ["dwg_drawing_to_svg", "rasterize_svg_to_png_base64", "title_card_svg"],
  "💠️lowpoly": ["register_mesh_dwg_export_handler", "register_mesh_dwg_import_handler", "register_mesh_exporter", "register_mesh_importer"],
  "📸️remodel": ["OsMediaExportResult"],
  "🗒️note": ["svg_to_dwg_bytes"],
  "🔱️trinity": [],
  "🖍️draw": [],
  "🧩️puzzle": ["register_2d_export_handlers", "register_dwg_import_handler", "register_mesh_dwg_export_handler", "register_mesh_dwg_import_handler", "register_mesh_exporter", "register_mesh_importer"],
  "🪐️space": ["APP_REGISTRATIONS", "DwgDrawing", "OS_HOME_VFS_ROOT_ID", "OS_SPACE_SCHEMA", "OsBackbonePort", "OsMediaCapability", "OsMediaExportResult", "OsParameter", "OsParameterFieldBinding", "OsParameterType", "OsSpaceCatalogEntry", "OsWorkflowCamera", "SpaceKind", "SpaceVisibility", "VcsError", "Workflow", "WorkflowNode", "WorkflowSnapshot", "delete_os_space", "dwg_to_bytes", "empty_space_snapshot", "empty_workflow_snapshot", "export_os_app_instance_media_kind", "host", "import_os_app_instance_media_kind", "import_os_space_from_dsl", "list_os_space_catalog_entries", "media_accept_filter_kinds", "open_file_space_backbone", "open_folder_space_backbone", "os_parameter_types_compatible", "os_workflow_to_flow_fixture", "register_app_io", "register_dwg_import_handler", "validate_workflow", "workflow"],
  "🖨️raster": ["DwgColor", "DwgDrawing", "DwgEntity", "DwgGeometry", "rasterize_svg_to_png_base64"],
};

/** 🔑Owner dir name (with emoji) for a repo-relative path under `✏️s/🔌️plugins/<owner>/…` — `""` if not under the plugins tree. */
function policyPluginOwnerFromApaPath(relPath: string): string {
  if (!relPath.startsWith(`${POLICY_APA_PLUGINS_ROOT}/`)) return "";
  return relPath.slice(POLICY_APA_PLUGINS_ROOT.length + 1).split("/")[0] ?? "";
}

/** 🔎️Every `[dependencies]` / `[target.'cfg(...)'.dependencies]` section header's byte offset (start of the section body) in `content`, matching the shape 🧩️puzzle's cfg-gated `semio-framework-os` dependency actually uses (📓️w0-d-sdk-surface.md §3.2). Deliberately excludes `[dev-dependencies]` — dev/test-time deps are out of this rule's runtime-purity scope. */
const POLICY_PLUGIN_DEP_SECTION_RE = /^\[(?:dependencies|target\.[^\]]*\.dependencies)\]\s*$/gm;
const POLICY_PLUGIN_DEP_ANY_SECTION_RE = /^\[[^\]]*\]\s*$/gm;
const POLICY_PLUGIN_DEP_LINE_RE = /^([A-Za-z0-9_.-]+)\s*=\s*\{([^}]*)\}\s*$/gm;

/**
 * 📏️Plugin dependency allowlist: parses every `[dependencies]`/`[target.'cfg(...)'.dependencies]` block
 * in every `Cargo.toml` under `✏️s/🔌️plugins/<owner>/.../📦️packages/🦀️rust/` and flags every dependency whose
 * `path =` points into `🧰️framework/` and whose crate name (the `package = "..."` override, or the TOML
 * key) is outside `POLICY_PLUGIN_DEP_FRAMEWORK_ALLOWLIST`. `semio-framework-os` (the HOST crate) is the
 * headline forbidden one and gets its own kind + the exact offending symbol list in `solution`.
 */
function policyPluginDependencyAllowlistBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyDiscoverCargoTomlFiles(repoRoot)) {
    if (!relPath.startsWith(`${POLICY_APA_PLUGINS_ROOT}/`)) continue;
    if (!relPath.includes("/📦️packages/🦀️rust/")) continue;
    if (POLICY_PLUGIN_DEP_ALLOWLIST.has(relPath)) continue;
    const owner = policyPluginOwnerFromApaPath(relPath);
    const content = policyReadFileSafe(repoRoot, relPath);

    const sectionStarts: number[] = [];
    POLICY_PLUGIN_DEP_SECTION_RE.lastIndex = 0;
    let sm: RegExpExecArray | null;
    while ((sm = POLICY_PLUGIN_DEP_SECTION_RE.exec(content))) sectionStarts.push(sm.index + sm[0].length);
    const allStarts: number[] = [];
    POLICY_PLUGIN_DEP_ANY_SECTION_RE.lastIndex = 0;
    let am: RegExpExecArray | null;
    while ((am = POLICY_PLUGIN_DEP_ANY_SECTION_RE.exec(content))) allStarts.push(am.index);

    for (const start of sectionStarts) {
      const end = allStarts.find((s) => s > start) ?? content.length;
      const block = content.slice(start, end);
      POLICY_PLUGIN_DEP_LINE_RE.lastIndex = 0;
      let m: RegExpExecArray | null;
      while ((m = POLICY_PLUGIN_DEP_LINE_RE.exec(block))) {
        const key = m[1]!;
        const body = m[2]!;
        const pathMatch = body.match(/path\s*=\s*"([^"]*)"/);
        if (!pathMatch || !pathMatch[1]!.includes("🧰️framework")) continue;
        const pkgMatch = body.match(/package\s*=\s*"([^"]*)"/);
        const crateName = pkgMatch ? pkgMatch[1]! : key;
        if (POLICY_PLUGIN_DEP_FRAMEWORK_ALLOWLIST.has(crateName) || POLICY_PLUGIN_DEP_SERDE_RE.test(crateName)) continue;
        const lineNo = policyLineOfIndex(content, start + m.index);
        const isOsHost = crateName === "semio-framework-os";
        const symbols = isOsHost ? (POLICY_PLUGIN_DEP_OS_SYMBOLS[owner] ?? []) : [];
        breaches.push({
          id: `plugin-dependency-allowlist-${relPath}-${crateName}`,
          summary: isOsHost
            ? `"${relPath}" depends on the forbidden HOST crate "semio-framework-os"${symbols.length ? ` (uses: ${symbols.join(", ")})` : " (0 semio_framework_os:: uses found — likely a stale/unused dependency)"}`
            : `"${relPath}" depends on framework crate "${crateName}", outside the plugin dependency allowlist`,
          kind: isOsHost ? "taxonomy/plugin-dependency-os-host" : "taxonomy/plugin-dependency-allowlist",
          scope: relPath,
          line: lineNo,
          priority: "medium",
          reason: isOsHost
            ? "APA/M3: semio-framework-os is the OS HOST crate — plugins may depend only on semio-framework-plugin, the os-kernel crate, semio-framework-3d and its semio-framework-3d-brep-* crates, semio-framework-schema, and serde-family third-party crates; the host crate's types/fns must be re-exported through the SDK instead (📓️w0-d-sdk-surface.md §3.2)."
            : "APA/M3: the plugin dependency allowlist is semio-framework-plugin + the os-kernel crate + semio-framework-3d and semio-framework-3d-brep-* (where genuinely used) + semio-framework-schema + serde-family — every other framework crate is a bypass of the curated SDK surface (📓️w0-d-sdk-surface.md §3.1).",
          solution: isOsHost
            ? symbols.length > 0
              ? `Remove "${crateName}" from ${relPath} and add explicit re-exports for [${symbols.join(", ")}] to semio_framework_plugin's tail block instead (M3).`
              : `Remove the unused "${crateName}" dependency from ${relPath} — 0 semio_framework_os:: uses found in this crate's own .rs files (double-check build.rs/feature-gated code before deleting).`
            : `Remove "${crateName}" from ${relPath}, or add an explicit re-export for whatever it provides to semio_framework_plugin's SDK surface (M3) instead of depending on the framework crate directly.`,
        });
      }
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRulePluginDependencyAllowlist

//#region 🔧️PolicyRuleEffectCapabilityParity
/** 🎫️Ticket-cited exceptions — empty; an entry requires a ticket citation in a comment beside it. */
const POLICY_EFFECT_CAPABILITY_ALLOWLIST = new Set<string>();

/**
 * 📇️`HostEffect` variant → required capability `ArtifactKind`, machine-readable form of the effect→
 * capability contract (📓️w0-d-sdk-surface.md §5, `🎠️kernel/🦀️.rs:247-387`). Rights/Scope are
 * noted per-line for provenance but the parity check below matches at `ArtifactKind` granularity only:
 * no plugin has ever called `.capability(...)` for real yet (0 hits repo-wide) to pin the exact
 * Rights/Scope argument shape the eventual `has_capability(ArtifactKind, Rights)` gate (M5) will take, so
 * matching finer than ArtifactKind would be guessing at an unimplemented signature. `ArtifactKind` itself
 * has no "Shell"/"UI" member yet, so every window-chrome effect below maps onto the closest existing fit
 * (`Window`) per w0-d's own proposal — flagged there as needing W1/W2 confirmation before this becomes
 * policy, not yet resolved by this ticket.
 */
const POLICY_HOST_EFFECT_CAPABILITY: Readonly<Record<string, string>> = {
  OpenWindow: "Window", // Rights::Open, Scope::Instance
  CloseWindow: "Window", // Rights::Write, Scope::Instance
  Notify: "Window", // Rights::Write, Scope::Instance
  ClipboardWrite: "Document", // Rights::Write, Scope::Global
  RequestSync: "Backbone", // Rights::Invoke, Scope::Instance
  Navigate: "Window", // Rights::Write, Scope::Global
  LoadDocument: "Document", // Rights::Write, Scope::Instance
  OpenExternalUrl: "Network", // Rights::Open, Scope::Global
  SetPanel: "Window", // Rights::Write, Scope::Instance
  DownloadMediaExport: "Asset", // Rights::Write, Scope::Instance
  IconRenderExport: "Asset", // Rights::Write, Scope::Instance
  VideoRenderExport: "Asset", // Rights::Write, Scope::Instance
  RequestFileOpen: "Asset", // Rights::Open, Scope::Instance
  RequestMediaFrames: "Asset", // Rights::Open, Scope::Instance
  SpawnPluginInstance: "Window", // Rights::Open, Scope::Global
  OpenPluginInstance: "Window", // Rights::Open, Scope::Global
  SetActiveUtility: "Window", // Rights::Write, Scope::Instance
  SetActiveTool: "Window", // Rights::Write, Scope::Instance
  OpenDialog: "Window", // Rights::Open, Scope::Instance
  DispatchAction: "Window", // Rights::Invoke, Scope::Instance
  ReplayShellCommand: "Window", // Rights::Invoke, Scope::Global
  InvokeExtension: "Engine", // Rights::Invoke, Scope::Plugin
};

const POLICY_HOST_EFFECT_CONSTRUCT_RE = /\bEffect::(\w+)\b/g;
const POLICY_CAPABILITY_CALL_RE = /\.capability\s*\(\s*ArtifactKind::(\w+)/g;
const POLICY_BACKBONE_STORAGE_RE = /\.local_backbone_storage\s*\(/;

/**
 * 📏️Effect/capability parity: for each plugin, collects every `HostEffect::<Variant>` constructed
 * anywhere in its tree (skipping `#[cfg(test)]` spans) and the capabilities it declares in its root
 * `🦀️.rs` (`.capability(ArtifactKind::X, …)` and `.local_backbone_storage()`, which stands in
 * for `ArtifactKind::Backbone`) — flags every constructed variant whose required `ArtifactKind` isn't
 * declared. Expected to fire on nearly every effect-constructing plugin: today only 🪐️space declares any
 * capability at all, and its lone `.local_backbone_storage()` doesn't satisfy any of the `Window`/
 * `Document`/`Asset` requirements its own 6 constructed variants need either.
 */
function policyEffectCapabilityParityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const owner of policyReaddirSafe(repoRoot, POLICY_APA_PLUGINS_ROOT)) {
    if (!owner.isDirectory) continue;
    const ownerRel = `${POLICY_APA_PLUGINS_ROOT}/${owner.name}`;
    if (POLICY_EFFECT_CAPABILITY_ALLOWLIST.has(ownerRel)) continue;

    const declared = new Set<string>();
    const rootLeaf = join(repoRoot, ownerRel, (POLICY_RS_COMPONENT_LEAF_NAME));
    if (existsSync(rootLeaf)) {
      const rootContent = readFileSync(rootLeaf, "utf8");
      POLICY_CAPABILITY_CALL_RE.lastIndex = 0;
      let cm: RegExpExecArray | null;
      while ((cm = POLICY_CAPABILITY_CALL_RE.exec(rootContent))) declared.add(cm[1]!);
      if (POLICY_BACKBONE_STORAGE_RE.test(rootContent)) declared.add("Backbone");
    }

    const constructedByVariant = new Map<string, { relPath: string; line: number }[]>();
    const walk = (relDir: string): void => {
      for (const entry of policyReaddirSafe(repoRoot, relDir)) {
        const childRel = `${relDir}/${entry.name}`;
        if (entry.isDirectory) {
          walk(childRel);
          continue;
        }
        if (!entry.name.endsWith(".rs")) continue;
        const content = policyReadFileSafe(repoRoot, childRel);
        const lines = content.split(/\r?\n/);
        const testSpans = policyTestModSpans(lines);
        lines.forEach((raw, i) => {
          const lineNo = i + 1;
          if (policyLineInTestMod(testSpans, lineNo)) return;
          const codeOnly = policyMaskLiterals(raw).replace(/\/\/.*$/, "");
          POLICY_HOST_EFFECT_CONSTRUCT_RE.lastIndex = 0;
          let vm: RegExpExecArray | null;
          while ((vm = POLICY_HOST_EFFECT_CONSTRUCT_RE.exec(codeOnly))) {
            const variant = vm[1]!;
            if (!(variant in POLICY_HOST_EFFECT_CAPABILITY)) continue;
            const list = constructedByVariant.get(variant) ?? [];
            list.push({ relPath: childRel, line: lineNo });
            constructedByVariant.set(variant, list);
          }
        });
      }
    };
    walk(ownerRel);

    for (const [variant, sites] of constructedByVariant) {
      const requiredKind = POLICY_HOST_EFFECT_CAPABILITY[variant]!;
      if (declared.has(requiredKind)) continue;
      const first = sites[0]!;
      breaches.push({
        id: `effect-capability-parity-${ownerRel}-${variant}`,
        summary: `"${ownerRel}" constructs HostEffect::${variant} (${sites.length} site${sites.length === 1 ? "" : "s"}, first at ${first.relPath}:${first.line}) without declaring the ${requiredKind} capability it requires`,
        kind: "taxonomy/effect-capability-parity",
        scope: ownerRel,
        line: first.line,
        priority: "medium",
        reason: "APA/M5: every HostEffect variant maps to a CapabilityRequirement (📓️w0-d-sdk-surface.md §5) — a plugin that constructs the effect without declaring the matching capability makes today's capability system decorative; only 🪐️space declares any capability repo-wide, and it doesn't cover any of the effects plugins actually construct.",
        solution: `Add .capability(ArtifactKind::${requiredKind}, ...) to ${ownerRel}/${POLICY_RS_COMPONENT_LEAF_NAME}'s Plugin::builder chain once M5's has_capability(ArtifactKind, Rights) gate lands, or remove the HostEffect::${variant} construction if the plugin shouldn't have this capability.`,
      });
    }
  }
  return breaches;
}
//#endregion 🔧️PolicyRuleEffectCapabilityParity

//#region 🔧️PolicyRuleApaRatchet
/**
 * 🚦W5 shrink-only ceiling table for APA's five report-mode rules. A rule's kind at or under its ceiling
 * stays entirely at `priority: "medium"` (census, non-gating); only breaches ABOVE the ceiling — the
 * measured regression — are promoted to `priority: "high"`, which `VerifyScript.runGate`'s
 * `dissolveBreaches` block already filters on. Ceilings MAY be lowered freely as debt burns down; RAISING
 * one requires a ticket citation in a comment beside the changed entry, or the ratchet is meaningless.
 * Measured `bun ./📜️script.ts policy` against `.🧬semio/🦑️repo/⚡️cache/breaches/compose.json`, three runs
 * 2026-08-13 ~00:12/00:14/00:16 (~90-100s apart) — see `📓️w5-ratchet-report.md`.
 *
 * Four of eight APA ratchet keys are intentionally ABSENT below — no ceiling, permanently `medium` — not
 * from a stale prior estimate but because the three-run measurement itself showed each one moving:
 * - `plugin-registration-engine-backlog` — coordinator-flagged: a concurrent peer wave (9 agents) is
 *   actively dissolving artifact ⚙️engine/ dirs and relocating register*() calls out of them right now; a
 *   move leaves code transiently in both the old and new location, which can legitimately INCREASE this
 *   count for a few minutes. A shrink-only ceiling here would gate the shared tree on the peer's own
 *   in-flight progress.
 * - `plugin-registration-violation` — measured 570 → 580 → 600 across the three runs; the same dual-write
 *   window reaches this kind too (a register*() call briefly lands outside any ⚙️engine mid-move).
 * - `plugin-registration-setup-callback` — measured 14 → 14 → 15, in the same window `📓️status.md`'s
 *   "FINAL STATE" section describes the `.setup()` 33→11 conversion landing in.
 * - `plugin-purity` (all `taxonomy/plugin-purity-*` sub-kinds summed) — measured 116 → 118 → 125
 *   (filesystem-io alone: 35 → 35 → 42, interior-mutability-mutex: 20 → 22 → 22). Despite being
 *   inventory-only by APA's own original design (nothing in this ticket ever tried to reduce it), the
 *   underlying file set is not static tonight, so it gets the same exemption on measured evidence rather
 *   than on the design intent alone.
 */
const POLICY_APA_RATCHET_CEILINGS: Readonly<Record<string, number>> = {
  "plugin-closed-shape": 41, // flat 41/41/41 across 3 runs ~90-100s apart, 2026-08-13 ~00:12-00:16 — 📓️w5-ratchet-report.md
  "plugin-dependency-allowlist": 105, // flat 105/105/105 across 3 runs ~90-100s apart, 2026-08-13 ~00:12-00:16 — 📓️w5-ratchet-report.md
  "plugin-dependency-os-host": 10, // flat 10/10/10 across 3 runs ~90-100s apart, 2026-08-13 ~00:12-00:16 — 📓️w5-ratchet-report.md
  "effect-capability-parity": 47, // flat 47/47/47 across 3 runs ~90-100s apart, 2026-08-13 ~00:12-00:16 — 📓️w5-ratchet-report.md
};

/** 🔑Ratchet grouping key for a breach `kind` — every `taxonomy/plugin-purity-*` sub-kind (filesystem-io, interior-mutability-*, thread-local-state, ts-side-effect, …) collapses onto one shared `"plugin-purity"` key matching the ceiling table's single combined row; every other APA kind keeps a 1:1 key. */
function policyApaRatchetKey(kind: string): string {
  if (kind.startsWith("taxonomy/plugin-purity-")) return "plugin-purity";
  return kind.startsWith("taxonomy/") ? kind.slice("taxonomy/".length) : kind;
}

/**
 * 🚦Applies the shrink-only ratchet: groups `breaches` by `policyApaRatchetKey`, and for any key present in
 * `POLICY_APA_RATCHET_CEILINGS`, keeps the first `ceiling` breaches at their original `medium` priority and
 * promotes only the breaches beyond the ceiling to `high` with a regression message naming the key, the
 * ceiling, and the measured count. Keys absent from the table pass through untouched, always `medium`.
 */
function policyApaRatchetApply(breaches: readonly BreachRecord[]): BreachRecord[] {
  const grouped = new Map<string, BreachRecord[]>();
  for (const b of breaches) {
    const key = policyApaRatchetKey(b.kind);
    const list = grouped.get(key);
    if (list) list.push(b);
    else grouped.set(key, [b]);
  }
  const out: BreachRecord[] = [];
  for (const [key, list] of grouped) {
    const ceiling = POLICY_APA_RATCHET_CEILINGS[key];
    if (ceiling === undefined || list.length <= ceiling) {
      out.push(...list);
      continue;
    }
    list.forEach((b, i) => {
      if (i < ceiling) {
        out.push(b);
        return;
      }
      out.push({
        ...b,
        priority: "high",
        summary: `RATCHET REGRESSION on "${key}": ceiling is ${ceiling}, measured ${list.length} — shrink-only, raising the ceiling requires a ticket citation — ${b.summary}`,
      });
    });
  }
  return out;
}

/**
 * 🎯Combined, ratcheted breach set for all five APA rules — the single call site both `policy`'s
 * aggregator and `VerifyScript.runGate`'s `dissolveBreaches` block use, so census and gate can never
 * disagree on a breach's priority.
 */
export function policyApaBreaches(repoRoot: string): BreachRecord[] {
  return policyApaRatchetApply([
    ...policyPluginClosedShapeBreaches(repoRoot),
    ...policyPluginPurityBreaches(repoRoot),
    ...policyStateLaneExhaustivenessBreaches(repoRoot),
    ...policyDeclarativeRegistrationBreaches(repoRoot),
    ...policyPluginDependencyAllowlistBreaches(repoRoot),
    ...policyEffectCapabilityParityBreaches(repoRoot),
  ]);
}
//#endregion 🔧️PolicyRuleApaRatchet

//#endregion 🔧️PolicyRuleArtifactsOnlyPluginArchitecture
