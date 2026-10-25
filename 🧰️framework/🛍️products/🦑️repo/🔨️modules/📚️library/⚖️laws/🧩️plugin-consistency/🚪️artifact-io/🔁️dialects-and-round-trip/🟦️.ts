import { join } from "node:path";
import { existsSync } from "node:fs";
import type { BreachRecord } from "../../../../🚨️policy/🔖️breach/🟦️.ts";
import { POLICY_STDIO_ARTIFACTS_REL, POLICY_STDIO_PLUGIN_REL, policyLoadStdioOwnerTable } from "../📋️owner-table/🟦️.ts";
import { loadTaxonomy } from "../../../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { policyAllRustFiles, policyNormalizeRelPath } from "../../🏛️foundation/🟦️.ts";
import { subsetIdForDirectoryName } from "../../../../🔍️discovery/🗂️file-kind-resolution/🟦️.ts";
import { policyReadFileSafe } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF } from "../../✍️handcrafted-mutation/🟦️.ts";
import { policyStripRustCommentsAndStrings } from "../../../../🔍️discovery/📖️source-access/🦀️rust-evidence/🟦️.ts";

/**
 * 🎫 Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 2: a
 * SELF-referential `Dialect { artifact_kind: "s.stdio.<X>", standard: StandardId("<S>"), subset:
 * SubsetId("<U>") }` literal (one whose `artifact_kind` names the SAME stdio artifact the file
 * lives under -- e.g. `🎞️gif`'s own `DIALECT`/`WRITES` const, never a cross-artifact `DEP_*`/`READS`
 * dependency literal naming some OTHER artifact) must agree with the `🏅️standards/🔖️<S>/…
 * 🪆️subsets/✳️<U>/` directory the file actually lives under. Cross-artifact references are skipped
 * entirely -- they correctly point at a directory this file doesn't live inside, so there's nothing
 * to verify from here. Subset checking honors the `SubsetId::ANY = SubsetId("*")` sentinel
 * convention (the literal is `"*"`, the directory is named `✳️any`) rather than naive string
 * equality -- moot today (every subset is `any`), meaningful once D5's PDF/A-2b pilot lands.
 */
// 🐛 Scoped to `const DIALECT`/`const WRITES` specifically (the two canonical self-identity const
// names used throughout this codebase's composers) — earlier versions matched ANY `Dialect{...}`
// literal in the file, which also caught legitimate same-artifact-different-subset dependency
// consts (e.g. a subset composer's own `DIALECT_ANY` fallback reference into its standard's `any`
// subset) and flagged them as if they claimed self-identity. Indirection (`const WRITES: Dialect =
// SOME_OTHER_CONST;`) is a false negative here (nothing to check), which is the safe direction —
// never a false positive.
const POLICY_DIALECT_LITERAL_RE = /const\s+(?:DIALECT|WRITES)\s*:\s*Dialect\s*=\s*Dialect\s*\{\s*artifact_kind:\s*"([^"]+)"\s*,\s*standard:\s*StandardId\("([^"]+)"\)\s*,\s*subset:\s*SubsetId\("([^"]+)"\)\s*\}/g;

export function policyDialectLiteralPathBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches; // owner-table-missing is already flagged by policyStdioCatalogBreaches
  const taxonomy = loadTaxonomy();
  const subsetsDirName = taxonomy.subsetsDirName;
  const dirToKey = new Map<string, string>();
  for (const [key, entry] of Object.entries(table.artifacts)) {
    dirToKey.set(entry.dir, key);
  }
  const artifactsPrefix = `${POLICY_STDIO_ARTIFACTS_REL}/`;
  for (const relPath of policyAllRustFiles(repoRoot)) {
    const cut = relPath.indexOf(artifactsPrefix);
    if (cut === -1) continue;
    const afterArtifacts = relPath.slice(cut + artifactsPrefix.length);
    const ownDir = afterArtifacts.split("/")[0];
    const ownKey = ownDir ? dirToKey.get(ownDir) : undefined;
    if (!ownKey) continue;
    const ownArtifactKind = `s.stdio.${ownKey}`;
    const standardMatch = relPath.match(/🏅️standards\/🔖️([^/]+)\//);
    const pathSegments = relPath.split("/");
    const subsetsIndex = pathSegments.lastIndexOf(subsetsDirName);
    const subsetsOwner = subsetsIndex >= 0 ? pathSegments.slice(0, subsetsIndex + 1).join("/") : null;
    const physicalSubsetDirectory = subsetsIndex >= 0 ? pathSegments[subsetsIndex + 1] ?? null : null;
    const pathSubsetId = subsetsOwner && physicalSubsetDirectory ? subsetIdForDirectoryName(subsetsOwner, physicalSubsetDirectory, taxonomy) : null;
    const content = policyReadFileSafe(repoRoot, relPath);
    POLICY_DIALECT_LITERAL_RE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = POLICY_DIALECT_LITERAL_RE.exec(content))) {
      const [, litKind, litStandard, litSubset] = m!;
      if (litKind !== ownArtifactKind) continue;
      if (standardMatch && litStandard !== standardMatch[1]) {
        breaches.push({
          id: `dialect-literal-standard-${relPath}-${litStandard}`,
          summary: `"${relPath}" declares StandardId("${litStandard}") but lives under 🏅️standards/🔖️${standardMatch[1]}/`,
          kind: "artifact-io/dialect-literal-path",
          scope: relPath,
          priority: "high",
          reason: "A self-referential Dialect literal's standard must match the 🔖️<standard> directory the file physically lives under, or the two will silently drift apart the next time either one changes alone.",
          solution: `Fix StandardId("${litStandard}") in ${relPath} to "${standardMatch[1]}", or move the file to 🏅️standards/🔖️${litStandard}/ if that's the one that's wrong.`,
        });
      }
      if (pathSubsetId !== null && litSubset !== pathSubsetId) {
        breaches.push({
          id: `dialect-literal-subset-${relPath}-${litSubset}`,
          summary: `"${relPath}" declares SubsetId("${litSubset}") but lives under ${subsetsOwner}/${physicalSubsetDirectory}/`,
          kind: "artifact-io/dialect-literal-path",
          scope: relPath,
          priority: "high",
          reason: "A self-referential Dialect literal's logical subset id must match the exact owner-scoped semantic directory it physically lives under.",
          solution: `Fix SubsetId("${litSubset}") in ${relPath} to "${pathSubsetId}", or move the file if the directory is wrong.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 🪪️ Ticket 26/08/11/SEMIO-ARTIFACT-UNIFIED-IMPORT-EXPORT-AND-MEDIA-FORMAT-RETIREMENT, W1 Task 4
 * blocking finding: `store::os_store::codec_registry::register_document_codec` (🧰️framework/…/🏪️store/🦀️.rs) is a
 * plain `HashMap::insert` keyed by the codec's schema id string — registering two DIFFERENT codecs
 * under the SAME id does not panic, it silently overwrites (last-registered-wins), with zero
 * runtime diagnostic (empirically confirmed by W1's own regression test). This is a real risk once
 * many parallel agents each mint their own schema-id constant (13 semio subsets + 7 new formats +
 * 28 pre-existing artifacts, all sharing one process-wide registry) — a typo'd copy-paste collision
 * would silently load the wrong codec at runtime with zero build or test signal. This rule
 * statically resolves every `register_document_codec(store::artifact_codec::<…>(<id-expr>))`
 * call site's id expression — a string literal, or a `const _: &str` referenced bare or fully
 * path-qualified (both forms are used across the crate) — across the WHOLE stdio crate, and flags
 * any resolved id string claimed by more than one call site. Mirrors `policyDialectLiteralPathBreaches`
 * above: same two-pass grep-then-cross-reference shape, same `policyAllRustFiles` file source.
 */
export function policyStdioCodecIdUniquenessBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const pluginRoot = join(repoRoot, POLICY_STDIO_PLUGIN_REL);
  if (!existsSync(pluginRoot)) return breaches;
  const artifactsPrefix = `${POLICY_STDIO_ARTIFACTS_REL}/`;
  const files = policyAllRustFiles(repoRoot).filter((relPath) => relPath.startsWith(artifactsPrefix));

  // Pass 1: crate-wide `const NAME: &str = "value";` map, keyed by the identifier's *last path
  // segment* — call sites reference a schema-id const either bare or fully `crate::…::`-qualified.
  const constValue = new Map<string, string>();
  const CONST_RE = /(?:pub\s+)?const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*"([^"]*)"\s*;/g;
  for (const relPath of files) {
    const content = policyReadFileSafe(repoRoot, relPath);
    CONST_RE.lastIndex = 0;
    let cm: RegExpExecArray | null;
    while ((cm = CONST_RE.exec(content))) {
      const [, name, value] = cm;
      if (name !== undefined && value !== undefined) constValue.set(name, value);
    }
  }

  // Pass 2: every register_document_codec call site's id expression, resolved to a literal value.
  const CALL_RE = /register_document_codec\(\s*store::artifact_codec::<[\s\S]*?>\(\s*([\w:]+|"[^"]*")\s*\)\s*\)/g;
  const claims = new Map<string, { relPath: string; line: number }[]>();
  for (const relPath of files) {
    const content = policyReadFileSafe(repoRoot, relPath);
    CALL_RE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = CALL_RE.exec(content))) {
      const raw = m[1]!;
      const value = raw.startsWith('"') ? raw.slice(1, -1) : constValue.get(raw.split("::").pop()!);
      if (!value) continue; // unresolved id expression — not guessed at, silently skipped
      const line = content.slice(0, m.index).split("\n").length;
      const list = claims.get(value) ?? [];
      list.push({ relPath, line });
      claims.set(value, list);
    }
  }

  for (const [id, sites] of claims) {
    if (sites.length <= 1) continue;
    const locations = sites.map((s) => `${s.relPath}:${s.line}`).join(", ");
    for (const site of sites) {
      breaches.push({
        id: `stdio-codec-id-duplicate-${id}-${site.relPath}-${site.line}`,
        summary: `schema id "${id}" is claimed by ${sites.length} register_document_codec call sites: ${locations}`,
        kind: "stdio-artifacts/codec-id-uniqueness",
        scope: site.relPath,
        priority: "high",
        reason:
          "store::os_store::codec_registry::register_document_codec silently overwrites on a duplicate id (plain HashMap::insert, no panic) — two codecs sharing an id means the second-registered one silently and invisibly wins at runtime, with zero build or test signal.",
        solution: `Give each of ${locations} its own distinct schema id constant/value.`,
      });
    }
  }
  return breaches;
}

/**
 * 🔁️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 8:
 * replaces the old `policyCodecFidelityBreaches` (a 5-string banned-marker grep — "does this file
 * avoid saying SRAS/IFCCARTOONMESH", a purely negative signal) with a positive one: every
 * standard-level `⚙️engine/🦀️.rs` must contain a real decode→encode→decode (or
 * equivalent lossless) round-trip test. Detected via a deliberately generous name/body heuristic
 * (`round_trip`/`roundtrip`/`decode_encode`/`encode_decode`/`lossless`, case-insensitive) rather
 * than requiring one exact test name — every codec this session actually delivered uses one of
 * these, under varying names (`codec_round_trip`, `zip_deflate_round_trip`,
 * `real_decode_stays_lossless_on_reencode`, ...). Old rule NOT deleted yet (still catches literal
 * stub markers, a cheap orthogonal check) — the plan calls for deleting it once this one's
 * allowlist is fully burned down; premature while 43 files remain.
 */
const POLICY_ROUND_TRIP_TEST_ALLOWLIST = new Set<string>([
  // 🐛 Same lesson as the other two allowlists in this file: keys are policyNormalizeRelPath's
  // canonical short form, computed programmatically against a real repo scan, never hand-typed —
  // see the sniff-reality allowlist's own comment for the full story of what happens otherwise.
  "trinity/jack/standards#1-engine-component",
  "trinity/rewriting/standards#1-engine-component",
  "raster/standards#1-engine-component",
  "flow/standards#1-engine-component",
  "process/process3d/standards#1-engine-component",
  "norm/din4108/standards#1-engine-component",
  "norm/din18599/standards#1-engine-component",
  "norm/din16798/standards#1-engine-component",
  "norm/en1990/standards#1-engine-component",
  "norm/en1991/standards#1-engine-component",
  "norm/en1992/standards#1-engine-component",
  "norm/en1993/standards#1-engine-component",
  "norm/en1994/standards#1-engine-component",
  "norm/en1995/standards#1-engine-component",
  "norm/en1996/standards#1-engine-component",
  "norm/en1997/standards#1-engine-component",
  "norm/en1998/standards#1-engine-component",
  "norm/en1999/standards#1-engine-component",
  "cad/standards#1-engine-component",
  "demonstrator/playground/standards#1-engine-component",
  "block/2d/standards#1-engine-component",
  "block/3d/standards#1-engine-component",
  "block/5d/standards#1-engine-component",
  "dag/standards#1-engine-component",
  "stdio/pdf/standards#1.4-engine-component", // deliberately untouched; real vocabulary is on 1.7
  "reasoning/wires/standards#1-engine-component",
  "writer/standards#1-engine-component",
  "animate/presentation/standards#1-engine-component",
  "space/home/standards#1-engine-component",
  "procedural/generation2d/standards#1-engine-component",
  "vcs/standards#1-engine-component",
  "gis/gismap/standards#1-engine-component",
  "gis/gisterrain/standards#1-engine-component",
  "note/standards#1-engine-component",
  "architect/program/standards#1-engine-component",
  "shooting/standards#1-engine-component",
  "puzzle/2d/standards#1-engine-component",
  "puzzle/3d/standards#1-engine-component",
  "puzzle/5d/standards#1-engine-component",
  "fem/2d/standards#1-engine-component",
  "fem/3d/standards#1-engine-component",
  "playbook/standards#1-engine-component",
  "energy/model/standards#1-engine-component",
  // seeded by W1b scaffold, burn down as W3 rewires each format's ArtifactPack off JSON-passthrough
  // onto its own engine's real encoder (see w1b-scaffold-manifest.md §6 — sniff/minimal-parse is
  // already real+tested for all 7, this is specifically the missing decode→encode→decode test) and
  // as W2 adds semio v1's own round-trip test alongside geometry/triples' existing 9. Keys computed
  // via policyNormalizeRelPath.
  // W3 closer removed epw/mp4/mp3/avi/wav — each `⚙️engine/🦀️.rs` now has a real
  // `#[cfg(test)]` round-trip test (grep-verified, policy re-run green after). html stays: its
  // real codec_retention_law round-trip test lives in `📸️snapshot/🦀️.rs`, not
  // `⚙️engine/🦀️.rs` — this rule is scoped to the engine file specifically, so html's
  // `⚙️engine/🦀️.rs` (just `sniff_real_bytes` + a small sniff test) genuinely still has
  // no round-trip signal under this rule's own detection method; a real architectural mismatch
  // between where html's parser lives and where this rule looks, not a fixed-and-forgotten entry
  // — documented here as a follow-up rather than moved, since relocating html's parser is a
  // design decision outside this closer's scope.
  "stdio/html/standards#5-engine-component",
  "stdio/semio/standards#v1-engine-component",
]);

const POLICY_ROUND_TRIP_SIGNAL_RE = /round_trip|roundtrip|decode_encode|encode_decode|lossless/i;

export function policyRoundTripTestBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.startsWith("✏️s/🔌️plugins/")) continue;
    if (!relPath.endsWith(`⚙️engine/${POLICY_RS_COMPONENT_LEAF}`)) continue;
    if (!relPath.includes("🏅️standards/🔖️")) continue;
    const normalized = policyNormalizeRelPath(relPath);
    const allowlisted = POLICY_ROUND_TRIP_TEST_ALLOWLIST.has(normalized);
    const content = policyReadFileSafe(repoRoot, relPath);
    const hasSignal = content.includes("#[cfg(test)]") && POLICY_ROUND_TRIP_SIGNAL_RE.test(content);
    if (!hasSignal) {
      if (allowlisted) continue;
      breaches.push({
        id: `round-trip-test-${relPath}`,
        summary: `"${relPath}" has no real decode→encode→decode round-trip test`,
        kind: "artifact-io/round-trip-test",
        scope: relPath,
        priority: "high",
        reason: "Every standard-level engine must prove its codec is lossless with a real round-trip test, not just avoid a list of known-stub markers.",
        solution: `Add a #[test] round-trip (decode→encode→decode) test to ${relPath}, or if this standard hasn't been reached by the codec uplift yet, add "${normalized}" to POLICY_ROUND_TRIP_TEST_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `round-trip-test-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_ROUND_TRIP_TEST_ALLOWLIST but already has a round-trip test`,
        kind: "artifact-io/round-trip-test",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${normalized}" from POLICY_ROUND_TRIP_TEST_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/**
 * 🗣️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 6
 * (scoped): every derived composition hook declares its cross-artifact dependencies as `const DEP_<NAME>:
 * Dialect = Dialect { artifact_kind: "s.stdio.<dep>", standard: StandardId("<std>"), subset:
 * SubsetId(...) }` (confirmed the universal pattern across every stdio composer this session
 * touched). This rule verifies each declared dependency's (artifact, standard) pair actually
 * exists on disk — `🗿️artifacts/<dep-dir>/🏅️standards/🔖️<std>/` — catching a phantom dependency
 * (typo'd standard, or a standard that got renamed/deleted out from under a composer that still
 * references the old name, e.g. exactly the `ac1018`→`ac1024` kind of rename this session did
 * several times) at policy-check time instead of a runtime compose failure. This is the "no
 * phantoms" half of the plan's fuller "composer dialect consts ↔ io leaf dirs bijection" — the "no
 * orphans" half (an io leaf dir nobody's composer ever references) needs a second pass once rule 3
 * (io-matrix migrated-leaves) traces the legacy early-continue branch it currently shares
 * plumbing with; left for a future session rather than rushed.
 */
const POLICY_DEP_DIALECT_RE = /const\s+DEP_\w+\s*:\s*Dialect\s*=\s*Dialect\s*\{\s*artifact_kind:\s*"s\.stdio\.([^"]+)"\s*,\s*standard:\s*StandardId\("([^"]+)"\)/g;

export function policyComposerDependencyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches;
  const dirByKey = new Map<string, string>();
  for (const [key, entry] of Object.entries(table.artifacts)) {
    dirByKey.set(key, entry.dir);
  }
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.endsWith(`🚪️io/${POLICY_RS_COMPONENT_LEAF}`)) continue;
    if (!relPath.startsWith(`${POLICY_STDIO_ARTIFACTS_REL}/`)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    POLICY_DEP_DIALECT_RE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = POLICY_DEP_DIALECT_RE.exec(content))) {
      const [, depKey, depStandard] = m!;
      const depDir = dirByKey.get(depKey!);
      if (!depDir) {
        breaches.push({
          id: `composer-dep-unknown-artifact-${relPath}-${depKey}`,
          summary: `"${relPath}" depends on unknown stdio artifact "${depKey}" (not in the catalog roster)`,
          kind: "artifact-io/composer-dependency",
          scope: relPath,
          priority: "high",
          reason: "A composer's DEP_* dialect must name a real, currently-cataloged stdio artifact — an unknown one is either a typo or a stale reference to a deleted/renamed artifact.",
          solution: `Fix the artifact_kind in ${relPath}'s DEP_* constant, or add "${depKey}" to the catalog roster if it's genuinely new.`,
        });
        continue;
      }
      const depStandardDir = join(repoRoot, POLICY_STDIO_ARTIFACTS_REL, depDir, "🏅️standards", `🔖️${depStandard}`);
      if (!existsSync(depStandardDir)) {
        breaches.push({
          id: `composer-dep-unknown-standard-${relPath}-${depKey}-${depStandard}`,
          summary: `"${relPath}" depends on ${depKey}@${depStandard}, but no such standard directory exists`,
          kind: "artifact-io/composer-dependency",
          scope: relPath,
          priority: "high",
          reason: "A composer's DEP_* dialect must name a standard that actually exists on disk — a phantom reference (typo, or a stale reference to a renamed/deleted standard) fails at compose time instead of at policy-check time.",
          solution: `Fix the StandardId in ${relPath}'s DEP_* constant to match an existing 🏅️standards/🔖️<std>/ directory under ${depDir}, or restore that standard if it was deleted in error.`,
        });
      }
    }
  }
  return breaches;
}

/**
 * 🚩️ Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 5:
 * flagship artifacts (svg/gltf/pdf/gif — the ones D2 explicitly names as getting real mutation
 * vocabularies, not just `{NoMutation, SetSnapshot}`) must have a mutation enum with more than
 * those two variants. Shrink-only allowlist for the flagships this hasn't reached yet.
 */
const POLICY_FLAGSHIP_MUTATION_ALLOWLIST = new Set<string>([
  // 🐛 Keys are policyNormalizeRelPath's canonical <pluginId>/<component>#<tail> short form, NOT
  // the raw file path — a full-path entry silently never matches (found the hard way: this
  // allowlist, and the sibling POLICY_SNIFF_REALITY_ALLOWLIST, both shipped with raw-path keys for
  // most of a session, during which `repo ./📜️script.ts verify` (a DIFFERENT, narrower gate
  // pipeline than `repo ./📜️script.ts policy`) was mistakenly used to "confirm" zero breaches —
  // `policy` is the command that actually runs these rules; always verify against `policy`, not
  // `verify`, when touching anything under //#region 🔧️PolicyRule*).
  "stdio/gltf/standards#2.0-subsets-any-schema-mutations-component", // gltf-internal's steps 1-2 scope didn't include mutations; not yet reached
  "stdio/pdf/standards#1.4-subsets-any-schema-mutations-component", // 1.4 deliberately left untouched; the real vocabulary landed on 1.7 instead
  "stdio/gif/standards#87a-subsets-any-schema-mutations-component", // permanent, not a todo: 87a has no GCE/animation/transparency concept to mutate: 89a carries the real vocabulary
]);

/** 🔎️Top-level variant names of the first `pub enum ...Mutation { ... }` found in `content`, or `null` if none found. Brace-depth-tracked (variants carry struct-style bodies with their own braces). */
function policyRustMutationEnumVariants(content: string): string[] | null {
  const stripped = policyStripRustCommentsAndStrings(content);
  const headerMatch = stripped.match(/pub\s+enum\s+\w*Mutation\w*\s*\{/);
  if (!headerMatch) return null;
  const start = headerMatch.index! + headerMatch[0].length;
  let depth = 1;
  let i = start;
  let segStart = start;
  const variants: string[] = [];
  const pushSeg = (raw: string) => {
    const m = raw.match(/(?:#\[[^\]]*\]\s*)*(\w+)/);
    if (m) variants.push(m[1]!);
  };
  while (i < stripped.length && depth > 0) {
    const ch = stripped[i];
    if (ch === "{" || ch === "(") depth++;
    else if (ch === "}" || ch === ")") {
      depth--;
      if (depth === 0) {
        pushSeg(stripped.slice(segStart, i));
        break;
      }
    } else if (ch === "," && depth === 1) {
      pushSeg(stripped.slice(segStart, i));
      segStart = i + 1;
    }
    i++;
  }
  return variants;
}

export function policyMutationVocabularyBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const flagshipDirs = [
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌳️pdf",
    "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif",
  ];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.endsWith(`🧬️mutations/${POLICY_RS_COMPONENT_LEAF}`)) continue;
    if (!flagshipDirs.some((dir) => relPath.startsWith(`${dir}/`))) continue;
    const normalized = policyNormalizeRelPath(relPath);
    const content = policyReadFileSafe(repoRoot, relPath);
    const variants = policyRustMutationEnumVariants(content);
    const realVariantCount = variants ? variants.filter((v) => v !== "NoMutation" && v !== "SetSnapshot").length : 0;
    const allowlisted = POLICY_FLAGSHIP_MUTATION_ALLOWLIST.has(normalized);
    if (realVariantCount === 0) {
      if (allowlisted) continue;
      breaches.push({
        id: `mutation-vocabulary-${relPath}`,
        summary: `"${relPath}" is a flagship mutation enum with only NoMutation/SetSnapshot`,
        kind: "artifact-io/mutation-vocabulary",
        scope: relPath,
        priority: "high",
        reason: "D2 names this artifact as a flagship that must expose a real mutation vocabulary (targeted insert/remove/set-field variants with real apply+inverse), not just the universal NoMutation/SetSnapshot pair every artifact starts with.",
        solution: `Add real, targeted mutation variants (with real apply+inverse) to ${relPath}, or if not reached yet, add "${normalized}" to POLICY_FLAGSHIP_MUTATION_ALLOWLIST citing this ticket.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `mutation-vocabulary-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_FLAGSHIP_MUTATION_ALLOWLIST but already has ${realVariantCount} real mutation variant(s)`,
        kind: "artifact-io/mutation-vocabulary",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is fixed.",
        solution: `Remove "${normalized}" from POLICY_FLAGSHIP_MUTATION_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}
