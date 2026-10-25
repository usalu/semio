/** 🫧️ Wave 4 plugin consistency policy: dissolved kernels and composition rules.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/🧩️plugin-consistency/🧭️breaches/🟦️.ts
 */
import { type BreachRecord } from "../../../🚨️policy/🔖️breach/🟦️.ts";
import { inspectRustChildKindMetadata } from "../../../🔍️discovery/🏛️rust-structure/🟦️.ts";
import { policyReadFileSafe } from "../../../🔍️discovery/📖️source-access/🟦️.ts";
import { policyWalkRelFiles } from "../../../🔍️discovery/🚶️file-walk/🟦️.ts";
import { policyAllRustFiles, policyNormalizeRelPath } from "../🏛️foundation/🟦️.ts";

//#region 🔧️PolicyRuleDissolvedKernels
/** 🧊️ Doctrine tier (d) ephemeral working representations — legal only as locals inside a diff
 * constructor or an `InferredField::{plan,dep_input,compute}` body, never as durable state. */
const POLICY_DISSOLVED_EPHEMERAL_REP_TYPES = ["HalfedgeMesh", "BrepEngineHost", "DrawingStore", "DrawingEngine", "EngineCache"] as const;

/** ⚙️ Canonical compute ownership and its retained plugin host constructor authority. */
const POLICY_DISSOLVED_ENGINE_CACHE_ALLOWED_DIRS = ["🧰️framework/🔨️modules/◻️2d/🧮️compute", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"] as const;

/** 🧭️ Constructor authority follows complete workspace-relative directory segments. */
function policyEngineCacheConstructorOwner(relPath: string): boolean {
  const normalized = relPath.replaceAll("\\", "/");
  if (normalized.startsWith("/") || /^[A-Za-z]:/.test(normalized) || normalized.split("/").some((segment) => !segment || segment === "." || segment === "..")) return false;
  return POLICY_DISSOLVED_ENGINE_CACHE_ALLOWED_DIRS.some((dir) => normalized === dir || normalized.startsWith(`${dir}/`));
}

/** 📉️ Shrink-only. Every entry is a known tier-(d) escape awaiting its dissolution wave. */
const POLICY_DISSOLVED_REP_ESCAPE_ALLOWLIST = new Set<string>([
  "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️.rs",
  "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️.rs",
  "🧰️framework/🔨️modules/◻️2d/🗄️store/🦀️.rs",
  "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🖥️host/🦀️.rs",
]);

/** ⏸️ A durable field or static holding an ephemeral representation — measures REACH, not mutability.
 *
 * A `static HOST: OnceLock<BrepEngineHost>` is write-once, so every mutability-based rule passes it,
 * and it is still a plugin holding a handle to host-owned engine state for the process lifetime.
 * Reach is the property that matters; a separate rule from any `&mut`/`static mut` check. */
function policyDissolvedRepEscapeBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    const normalized = policyNormalizeRelPath(relPath);
    const content = policyReadFileSafe(repoRoot, relPath);
    const held: string[] = [];
    for (const repType of POLICY_DISSOLVED_EPHEMERAL_REP_TYPES) {
      const fieldHit = new RegExp(`^\\s*(pub\\s+)?[a-z_][a-z0-9_]*\\s*:\\s*[^,;]*\\b${repType}\\b`, "m").test(content);
      const staticHit = new RegExp(`static\\s+[A-Z0-9_]+\\s*:\\s*[^=;]*\\b${repType}\\b`).test(content);
      if (fieldHit || staticHit) held.push(repType);
    }
    const allowlisted = POLICY_DISSOLVED_REP_ESCAPE_ALLOWLIST.has(normalized);
    if (held.length > 0) {
      if (allowlisted) continue;
      breaches.push({
        id: `dissolved-rep-escape-${relPath}`,
        summary: `"${relPath}" holds an ephemeral working representation (${held.join(", ")}) in a durable field or static`,
        kind: "dissolved-kernels/rep-escape",
        scope: relPath,
        priority: "medium",
        reason:
          "Doctrine tier (d): a working representation (halfedge adjacency, BVH, brep arena, tessellation buffer, engine cache) may exist only as a local inside a 🔺️diff constructor or an InferredField::{plan,dep_input,compute} body. Held durably it becomes authoritative state living outside the ArtifactStore — and a write-once wrapper such as OnceLock does not help, because the violation is ambient REACH, not ambient mutability.",
        solution: `Rebuild the representation from the snapshot via EngineRep::build(&base) at each use in ${relPath}, or if this file's dissolution wave has not landed yet, add "${normalized}" to POLICY_DISSOLVED_REP_ESCAPE_ALLOWLIST citing ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS.`,
      });
    } else if (allowlisted) {
      breaches.push({
        id: `dissolved-rep-escape-stale-${relPath}`,
        summary: `"${relPath}" is allowlisted in POLICY_DISSOLVED_REP_ESCAPE_ALLOWLIST but no longer holds an ephemeral representation`,
        kind: "dissolved-kernels/rep-escape",
        scope: relPath,
        priority: "low",
        reason: "Shrink-only allowlists must be pruned as soon as the underlying file is dissolved.",
        solution: `Remove "${normalized}" from POLICY_DISSOLVED_REP_ESCAPE_ALLOWLIST.`,
      });
    }
  }
  return breaches;
}

/** 🚧️ `EngineCache` construction belongs to canonical compute or the retained plugin host.
 * Derived values outside those owners belong in a 💡️inference facet keyed by DepHash;
 * ephemeral working representations are rebuilt through EngineRep. */
function policyDissolvedEngineCacheScopeBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (policyEngineCacheConstructorOwner(relPath)) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!/\bEngineCache::new\b/.test(content)) continue;
    breaches.push({
      id: `dissolved-engine-cache-scope-${relPath}`,
      summary: `"${relPath}" constructs an engine cache outside canonical compute or the retained plugin host`,
      kind: "dissolved-kernels/engine-cache-scope",
      scope: relPath,
      priority: "medium",
      reason:
        "EngineCache construction belongs to canonical compute or the retained plugin host. Elsewhere derived values belong in a 💡️inference facet with a real DepHash chain, and ephemeral representations are rebuilt through EngineRep from the snapshot.",
      solution: `Move the derived value in ${relPath} into a 💡️inference facet, or rebuild it per call via EngineRep::build(&base).`,
    });
  }
  return breaches;
}

/** 📁️ A whole-document-replace triad directory — banned vocabulary with no replacement.
 *
 * Whole-document replace is not expressible as an in-history mutation; it goes through
 * `ArtifactStore::reset`, outside history. A directory-level check catches what an identifier grep
 * misses, because the dispatch arm and the triad dir fail independently. */
function policyDissolvedWholeDocumentReplaceBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.includes("🧬️mutations/")) continue;
    if (!/\/🟤️set-snapshot\//.test(relPath)) continue;
    const dir = relPath.slice(0, relPath.indexOf("/🟤️set-snapshot/") + "/🟤️set-snapshot".length);
    breaches.push({
      id: `dissolved-whole-document-replace-${dir}`,
      summary: `"${dir}" is a whole-document-replace triad directory`,
      kind: "dissolved-kernels/whole-document-replace",
      scope: dir,
      priority: "medium",
      reason:
        "Whole-document replace is not an in-history mutation at all — it records what the document became, never what the user did, and has no meaningful inverse. It goes through ArtifactStore::reset (a non-undoable rebase used for file-open/import), entirely outside the mutation vocabulary.",
      solution: `Delete ${dir}, its dispatch arm, and its 🦀️.rs #[path] mount in the SAME change — a mount pointing at a removed directory aborts the build before compilation, for every crate in the workspace.`,
    });
  }
  return Array.from(new Map(breaches.map((b) => [b.id, b])).values());
}

/** ⚖️ Aggregates the dissolved-kernels doctrine rules (ticket 26/08/12/DISSOLVE-KERNELS…). */
export function policyDissolvedKernelsBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyDissolvedRepEscapeBreaches(repoRoot),
    ...policyDissolvedEngineCacheScopeBreaches(repoRoot),
    ...policyDissolvedWholeDocumentReplaceBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleDissolvedKernels

//#region 🔧️PolicyRuleComposition
/** 🪪️ Canonical artifact-kind grammar mirror of `ArtifactKindId::parse`/`is_canonical_artifact_kind`
 * (🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🦀️.rs:101-153, ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM
 * W1): exactly three dot-separated ASCII segments, the first literally `s`, the remaining two
 * lowercase-kebab (`[a-z0-9-]`, no leading/trailing/doubled hyphen). */
function policyIsCanonicalArtifactKind(kind: string): boolean {
  const segments = kind.split(".");
  if (segments.length !== 3 || segments[0] !== "s") return false;
  const isKebabSegment = (segment: string): boolean =>
    segment.length > 0 && !segment.startsWith("-") && !segment.endsWith("-") && !segment.includes("--") && /^[a-z0-9-]+$/.test(segment);
  return isKebabSegment(segments[1]!) && isKebabSegment(segments[2]!);
}

/** 🔎️True when a regex match's own source line (from the preceding newline up to the match) already
 * contains a `//` — i.e. the "hit" is doc-comment prose (`///`/`//!` quoting the pattern as an
 * example), not real code. Cheap same-line guard against the exact false-positive class flagged in
 * this ticket's `📌️important.md` ("the sweep rule… strip comments, check the target"). */
function policyMatchIsCommentedOut(content: string, matchIndex: number): boolean {
  const lineStart = content.lastIndexOf("\n", matchIndex - 1) + 1;
  return content.slice(lineStart, matchIndex).includes("//");
}

const POLICY_ARTIFACT_KIND_SPEC_ID_RE = /ArtifactKindSpec\s*\{\s*id:\s*"((?:[^"\\]|\\.)*)"/g;

/** 🗿️Checks literal artifact-kind declarations against the same grammar as child references. */
export function policyCanonicalArtifactKindBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    const content = policyReadFileSafe(repoRoot, relPath);
    if (!content.includes("ArtifactKindSpec")) continue;
    POLICY_ARTIFACT_KIND_SPEC_ID_RE.lastIndex = 0;
    let m: RegExpExecArray | null;
    while ((m = POLICY_ARTIFACT_KIND_SPEC_ID_RE.exec(content))) {
      if (policyMatchIsCommentedOut(content, m.index)) continue;
      const kind = m[1]!;
      if (policyIsCanonicalArtifactKind(kind)) continue;
      const line = content.slice(0, m.index).split("\n").length;
      breaches.push({
        id: `canonical-artifact-kind-${relPath}-${line}`,
        summary: `"${relPath}:${line}" declares ArtifactKindSpec.id "${kind}", not canonical grammar s.<plugin>.<artifact>`,
        kind: "composition/canonical-artifact-kind",
        scope: relPath,
        line,
        priority: "medium",
        reason:
          "ArtifactKindId::parse requires exactly three ASCII segments: s.<plugin>.<artifact>, with lowercase kebab names. Standard and subset are separate dialect fields.",
        solution: "Use a canonical s.<plugin>.<artifact> kind and keep standard/subset in their own dialect fields.",
      });
    }
  }
  return breaches;
}

const POLICY_CHILD_KIND_PATTERNS: Readonly<Record<string, RegExp>> = {
  ts: /@child\s+kind\s*=\s*([^\s*/]+)/g,
  json: /"x-semio-child"\s*:\s*"([^"]*)"/g,
  graphql: /@child\s*\(\s*kind\s*:\s*"([^"]*)"\s*\)/g,
  proto: /@child\s+kind\s*=\s*([^\s*/]+)/g,
};

/** 🧩️Checks child-kind metadata grammar across schema facets; concrete ownership forests belong to Store instance validation. */
export function policyCanonicalChildKindBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const files = policyWalkRelFiles(repoRoot, ["✏️s/🔌️plugins"], (path, name) =>
    path.includes("/🧬️schema/") && !path.includes("/🧪️tests/") && !path.includes("/🧫️fixtures/") && (name.endsWith(".rs") || Object.hasOwn(POLICY_CHILD_KIND_PATTERNS, name.split(".").at(-1)!)),
  );
  for (const relPath of files) {
    const content = policyReadFileSafe(repoRoot, relPath);
    const format = relPath.split(".").at(-1)!;
    const metadata = format === "rs" ? inspectRustChildKindMetadata(content) : [...content.matchAll(POLICY_CHILD_KIND_PATTERNS[format]!)].filter(match => {
      const lineStart = content.lastIndexOf("\n", match.index - 1) + 1;
      return format !== "graphql" || !content.slice(lineStart, match.index).includes("#");
    }).map(match => ({ kind: match[1]!, line: content.slice(0, match.index).split("\n").length }));
    for (const { kind, line } of metadata) {
      if (policyIsCanonicalArtifactKind(kind)) continue;
      breaches.push({
        id: `canonical-child-kind-${relPath}-${line}`,
        summary: `"${relPath}:${line}" declares child kind "${kind}", not canonical grammar s.<plugin>.<artifact>`,
        kind: "composition/canonical-child-kind",
        scope: relPath,
        line,
        priority: "high",
        reason: "A child kind identifies an artifact family. Standard and subset are separate dialect fields; ownership cycles depend on concrete parent and member identities.",
        solution: "Use the canonical artifact kind and keep standard/subset constraints in the child target and payload validators.",
      });
    }
  }
  return breaches;
}

/** 🔏️ Shrink-only. Confirmed, justified exceptions to the dissolved-kind redefinition ban below, keyed
 * `<relPath>:<TypeName>` (mirrors `POLICY_DISSOLVED_REP_ESCAPE_ALLOWLIST`'s shape). Empty at seed time —
 * the ~25-plugin migration wave that authored this ticket's frozen stdio roster already eliminated
 * every known duplicate of these 18 shapes; this allowlist exists only for a future, deliberately-
 * justified exception, never as a place to launder new debt. */
const POLICY_DISSOLVED_KIND_REDEFINITION_ALLOWLIST = new Set<string>([]);

/** 📸️ The 18 canonical stdio `🧿️semio` subset snapshot types (ticket
 * 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM's frozen roster, `📓️status.md` "ROSTER FROZEN") — the
 * ONE shape each of these content kinds may take repo-wide. */
const POLICY_DISSOLVED_KIND_CANONICAL_TYPES = [
  "SemioAnimationSnapshot",
  "SemioAudioSnapshot",
  "SemioBrepSnapshot",
  "SemioCadSnapshot",
  "SemioDocumentSnapshot",
  "SemioDrawingSnapshot",
  "SemioFlowSnapshot",
  "SemioGraphSnapshot",
  "SemioImageSnapshot",
  "SemioKitSnapshot",
  "SemioMeshSnapshot",
  "SemioModelSnapshot",
  "SemioObjectSnapshot",
  "SemioPresentationSnapshot",
  "SemioTableSnapshot",
  "SemioTextSnapshot",
  "SemioValueSnapshot",
  "SemioVideoSnapshot",
] as const;

/** 🚫️ Dissolved-kind redefinition ban — seeded allowlist ratchet (design doc "W6 ratchet" scope,
 * corrigendum-narrowed to composition-specific predicates only). A plugin outside `🗄️stdio` redeclaring
 * one of the 18 frozen `🧿️semio` subset snapshot types by name is exactly the duplicated-content-type
 * failure this ticket's migration wave exists to eliminate (design doc: "9+ mesh types, 4 brep
 * topologies… duplicate kind ids"). Deliberately narrow — an EXACT canonical struct/enum name
 * collision only, never a fuzzy shape-similarity heuristic — because the ~25-plugin migration already
 * burned this down to zero known duplicates: this rule's job is to catch a NEW one being reintroduced,
 * not to re-litigate settled debt (a broad heuristic here would drown real signal, exactly the trap
 * `📌️important.md` warns against). */
export function policyDissolvedKindRedefinitionBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  for (const relPath of policyAllRustFiles(repoRoot)) {
    if (!relPath.includes("🔌️plugins/") || relPath.includes("🔌️plugins/🗄️stdio/")) continue;
    const content = policyReadFileSafe(repoRoot, relPath);
    for (const typeName of POLICY_DISSOLVED_KIND_CANONICAL_TYPES) {
      if (!content.includes(typeName)) continue;
      const re = new RegExp(`^[ \\t]*(?:pub(?:\\([^)]*\\))?\\s+)?(?:struct|enum)\\s+${typeName}\\b`, "m");
      const match = re.exec(content);
      if (!match) continue;
      if (policyMatchIsCommentedOut(content, match.index)) continue;
      const line = content.slice(0, match.index).split("\n").length;
      const key = `${relPath}:${typeName}`;
      if (POLICY_DISSOLVED_KIND_REDEFINITION_ALLOWLIST.has(key)) continue;
      breaches.push({
        id: `dissolved-kind-redefinition-${relPath}-${typeName}`,
        summary: `"${relPath}:${line}" redefines "${typeName}", one of the 18 frozen stdio 🧿️semio subset snapshot types, outside 🗄️stdio`,
        kind: "composition/dissolved-kind-redefinition",
        scope: relPath,
        line,
        priority: "high",
        reason:
          "Ticket 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM dissolved every plugin-local duplicate of these 18 neutral content shapes into stdio's frozen 🧿️semio roster; a plugin redeclaring the same type name is the exact regression the migration wave eliminated (design doc: '9+ mesh types, 4 brep topologies… duplicate kind ids').",
        solution: `Import store::os_store::artifact_child::ArtifactChild<${typeName}> from stdio instead of redefining ${typeName} in ${relPath}, or if this is a deliberate, justified exception, add "${key}" to POLICY_DISSOLVED_KIND_REDEFINITION_ALLOWLIST citing why.`,
      });
    }
  }
  return breaches;
}

/** ✅️Checks canonical artifact and child kinds and prevents duplicate shared content declarations. */
export function policyCompositionBreaches(repoRoot: string): BreachRecord[] {
  return [
    ...policyCanonicalArtifactKindBreaches(repoRoot),
    ...policyCanonicalChildKindBreaches(repoRoot),
    ...policyDissolvedKindRedefinitionBreaches(repoRoot),
  ];
}
//#endregion 🔧️PolicyRuleComposition
