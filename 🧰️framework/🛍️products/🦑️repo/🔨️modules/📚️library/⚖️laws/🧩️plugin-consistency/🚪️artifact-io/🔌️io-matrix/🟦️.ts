import { join } from "node:path";
import { existsSync } from "node:fs";
import { policyReaddirSafe, policyReadFileSafe } from "../../../../🔍️discovery/📖️source-access/🟦️.ts";
import { POLICY_RS_COMPONENT_LEAF } from "../../✍️handcrafted-mutation/🟦️.ts";
import type { BreachRecord } from "../../../../🚨️policy/🔖️breach/🟦️.ts";
import { POLICY_STDIO_CODEC_BANNED_MARKERS, POLICY_STDIO_FACET_DESERIALIZERS, POLICY_STDIO_FACET_SERIALIZERS, POLICY_STDIO_IO_EXPORT, POLICY_STDIO_IO_IMPORT, POLICY_STDIO_OWNER_TABLE_REL, POLICY_STDIO_PLUGIN_REL, policyLoadStdioOwnerTable, policyStdioArtifactsDirName, policyStdioFormatDir } from "../📋️owner-table/🟦️.ts";
import { type PolicyArtifactDialect, policyListArtifactDialectDirs } from "../../../../🔍️discovery/🗿️artifact/🗣️dialects/🟦️.ts";
import { policyArtifactIsMigrated } from "../🗂️stdio-catalog/🟦️.ts";
import { POLICY_TS_COMPONENT_LEAF } from "../../../../🧹️normalization/🧬️mutation/🪪️identity/🟦️.ts";
import { policyWalkRelFiles } from "../../../../🔍️discovery/🚶️file-walk/🟦️.ts";

/** 🔎️True if any `🦀️.rs` exists anywhere under `rootRel` (any depth) — used to check a
 * migrated io leaf exists without having to guess which of a stdio format's own (standard,
 * subset) pairs a given domain artifact happens to bridge to. */
function policyHasComponentUnder(repoRoot: string, rootRel: string): boolean {
  const abs = join(repoRoot, rootRel);
  if (!existsSync(abs)) return false;
  const stack = [rootRel];
  while (stack.length > 0) {
    const dir = stack.pop()!;
    for (const entry of policyReaddirSafe(repoRoot, dir)) {
      if (entry.isDirectory) {
        stack.push(`${dir}/${entry.name}`);
      } else if (entry.name === POLICY_RS_COMPONENT_LEAF) {
        return true;
      }
    }
  }
  return false;
}

/**
 * 🎫 Ticket 26/08/10/ARTIFACT-SYSTEM-OVERHAUL-REAL-CODECS-RUNTIME-REUSE-EVOLUTION, D6 rule 3: the
 * REAL replacement for `policyIoSerializerMatrixBreaches`'s dead early-continue (see that
 * function's own comment — every owner is migrated now, so it has been a permanent no-op since
 * this session's Phase 1 completed the migration; confirmed via zero `stdio-artifacts/io-matrix`
 * breaches anywhere in a real `repo ./📜️script.ts policy` run). This checks the MIGRATED shape
 * instead: for each catalog owner's curated import/export format list, at least one of that domain
 * artifact's own (standard, subset) dirs must carry a real io leaf for that format — searched
 * existence-anywhere-under its `🚪️io/<direction>/<facet>/🗿️artifacts/<format-dir>/` tree (not a
 * specific (format-standard, format-subset) pair, since a domain artifact may reasonably bridge to
 * any one of a format's now-multiple standards, e.g. pdf 1.4 vs 1.7 — picking one to require would
 * be guessing, and this rule shouldn't invent a policy the codebase hasn't actually adopted yet).
 */
export function policyIoMatrixMigratedBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches;
  const roster = table.artifacts;
  const artifactsDir = policyStdioArtifactsDirName();
  const dialectsByArt = new Map<string, PolicyArtifactDialect[]>();
  for (const d of policyListArtifactDialectDirs(repoRoot)) {
    if (!dialectsByArt.has(d.artRel)) dialectsByArt.set(d.artRel, []);
    dialectsByArt.get(d.artRel)!.push(d);
  }
  for (const owner of table.owners ?? []) {
    const scope = owner.path;
    const dialects = dialectsByArt.get(scope);
    if (!dialects || dialects.length === 0) continue; // not migrated (yet) -- the legacy flat-path rule below still owns it
    const checkDirection = (direction: string, childFacet: string, formatIds: string[], label: string) => {
      for (const formatId of formatIds) {
        const formatDir = policyStdioFormatDir(roster, formatId);
        if (!formatDir) continue; // unknown-format-id is already reported by the legacy rule for unmigrated owners; a migrated owner referencing an unknown id is vanishingly unlikely and not worth a second breach shape here
        const found = dialects.some((d) => policyHasComponentUnder(repoRoot, `${d.subsetRel}/🚪️io/${direction}/${childFacet}/${artifactsDir}/${formatDir}`));
        if (!found) {
          breaches.push({
            id: `io-matrix-migrated-${scope}-${direction}-${formatId}`,
            summary: `"${scope}" has no migrated ${label} leaf for stdio format "${formatId}" in any of its own (standard, subset) dirs`,
            kind: "artifact-io/io-matrix-migrated",
            scope,
            priority: "high",
            reason: `The catalog's curated ${label} list for this artifact names "${formatId}", but no 🚪️io/${direction}/${childFacet}/${artifactsDir}/${formatDir}/ leaf exists under any of this artifact's migrated standard/subset dirs.`,
            solution: `Add a ${direction}/${childFacet} leaf for ${formatId} under one of "${scope}"'s 🏅️standards/🔖️.../🪆️subsets/✳️.../🚪️io/ dirs, or remove "${formatId}" from the ${label} list in ${POLICY_STDIO_OWNER_TABLE_REL} if it's no longer curated.`,
          });
        }
      }
    };
    checkDirection(POLICY_STDIO_IO_IMPORT, POLICY_STDIO_FACET_DESERIALIZERS, owner.import ?? [], "import");
    checkDirection(POLICY_STDIO_IO_EXPORT, POLICY_STDIO_FACET_SERIALIZERS, owner.export ?? [], "export");
  }
  return breaches;
}

/** ⚖️Curated import/export deserializer and serializer matrix from 🧪owner-table.json. */
export function policyIoSerializerMatrixBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches;
  const roster = table.artifacts;
  const artifactsDir = policyStdioArtifactsDirName();
  for (const owner of table.owners ?? []) {
    const scope = owner.path;
    if (policyArtifactIsMigrated(repoRoot, scope)) {
      // Migrated owner: its io leaves moved under 🏅️standards/.../🚪️io/ with target-qualified
      // (standard, subset) leaf paths -- a dedicated migrated io-matrix rule lands with the
      // domain fan-out (W14); until then this flat-path rule simply doesn't apply to it.
      continue;
    }
    const ioRoot = `${scope}/🚪️io`;
    if (!existsSync(join(repoRoot, ioRoot))) {
      breaches.push({
        id: `stdio-io-matrix-io-${scope}`,
        summary: `"${scope}" is missing 🚪️io/`,
        kind: "stdio-artifacts/io-matrix",
        scope,
        priority: "high",
        reason: "Curated stdio pairs are wired through the io facet deserializer/serializer tree.",
        solution: `Create ${ioRoot}/ with import/deserializers and export/serializers.`,
      });
      continue;
    }
    const checkLeaves = (direction: string, childFacet: string, formatIds: string[], label: string) => {
      for (const formatId of formatIds) {
        const formatDir = policyStdioFormatDir(roster, formatId);
        if (!formatDir) {
          breaches.push({
            id: `stdio-io-matrix-unknown-${scope}-${formatId}`,
            summary: `unknown stdio format id "${formatId}" on ${scope}`,
            kind: "stdio-artifacts/io-matrix",
            scope,
            priority: "high",
            reason: "Matrix format ids must exist in the schema-owned definition set.",
            solution: `Fix ${label} list for ${scope} in ${POLICY_STDIO_OWNER_TABLE_REL}.`,
          });
          continue;
        }
        const leafBase = `${ioRoot}/${direction}/${childFacet}/${artifactsDir}/${formatDir}`;
        for (const leaf of [POLICY_RS_COMPONENT_LEAF, POLICY_TS_COMPONENT_LEAF] as const) {
          const leafRel = `${leafBase}/${leaf}`;
          if (existsSync(join(repoRoot, leafRel))) continue;
          breaches.push({
            id: `stdio-io-matrix-${leafRel}`,
            summary: `missing ${label} ${leaf} for ${formatId} under ${scope}`,
            kind: "stdio-artifacts/io-matrix",
            scope,
            priority: "high",
            reason: "Each curated pair needs both Rust and TypeScript codec leaves under 🗿️artifacts/<stdio-dir>/.",
            solution: `Add ${leafRel}.`,
          });
        }
      }
    };
    checkLeaves(POLICY_STDIO_IO_IMPORT, POLICY_STDIO_FACET_DESERIALIZERS, owner.import ?? [], "import");
    checkLeaves(POLICY_STDIO_IO_EXPORT, POLICY_STDIO_FACET_SERIALIZERS, owner.export ?? [], "export");
  }
  return breaches;
}

/** 🕸️Stdio dependency DAG is acyclic and every format eventually depends on binary. */
export function policyIoTerminalityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const table = policyLoadStdioOwnerTable(repoRoot);
  if (!table) return breaches;
  const roster = table.artifacts;
  const nodes = new Set(Object.keys(roster));
  const adj = new Map<string, Set<string>>();
  for (const id of nodes) adj.set(id, new Set(roster[id]?.depends ?? []));
  for (const edge of table.dependency_edges) {
    if (!nodes.has(edge.from) || !nodes.has(edge.to)) {
      breaches.push({
        id: `stdio-dag-unknown-${edge.from}-${edge.to}`,
        summary: `schema-derived dependency edge references unknown node (${edge.from} → ${edge.to})`,
        kind: "stdio-artifacts/io-terminality",
        scope: POLICY_STDIO_PLUGIN_REL,
        priority: "high",
        reason: "Dependency edges must only connect schema-defined format ids.",
        solution: `Align schema dependencies in ${POLICY_STDIO_OWNER_TABLE_REL}.`,
      });
      continue;
    }
    adj.get(edge.from)?.add(edge.to);
    const rosterDeps = new Set(roster[edge.from]?.depends ?? []);
    if (!rosterDeps.has(edge.to)) {
      breaches.push({
        id: `stdio-dag-definition-edge-${edge.from}-${edge.to}`,
        summary: `schema-derived dependency edge ${edge.from}→${edge.to} is absent from its definition`,
        kind: "stdio-artifacts/io-terminality",
        scope: POLICY_STDIO_PLUGIN_REL,
        priority: "high",
        reason: "Derived dependency edges must mirror definition dependencies.",
        solution: `Fix dependencies for ${edge.from} in ${POLICY_STDIO_OWNER_TABLE_REL}.`,
      });
    }
  }
  const visitedGlobal = new Set<string>();
  const findCycle = (start: string): string[] | null => {
    const stack: string[] = [];
    const onStack = new Set<string>();
    const dfs = (n: string): string[] | null => {
      if (onStack.has(n)) return [...stack.slice(stack.indexOf(n)), n];
      if (visitedGlobal.has(n)) return null;
      onStack.add(n);
      stack.push(n);
      for (const dep of adj.get(n) ?? []) {
        const cyc = dfs(dep);
        if (cyc) return cyc;
      }
      stack.pop();
      onStack.delete(n);
      visitedGlobal.add(n);
      return null;
    };
    return dfs(start);
  };
  for (const n of nodes) {
    if (visitedGlobal.has(n)) continue;
    const cyc = findCycle(n);
    if (!cyc) continue;
    breaches.push({
      id: `stdio-dag-cycle-${cyc.join("-")}`,
      summary: `stdio DAG cycle: ${cyc.join(" → ")}`,
      kind: "stdio-artifacts/io-terminality",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "Stdio codec dependencies must form a DAG.",
      solution: "Remove cyclic schema definition dependencies.",
    });
    break;
  }
  const memo = new Map<string, boolean>();
  const reachesBinary = (n: string, trail: Set<string>): boolean => {
    if (n === "binary") return true;
    if (memo.has(n)) return memo.get(n)!;
    if (trail.has(n)) return false;
    trail.add(n);
    const deps = adj.get(n);
    if (!deps || deps.size === 0) {
      memo.set(n, false);
      return false;
    }
    let ok = true;
    for (const d of deps) ok = ok && reachesBinary(d, trail);
    memo.set(n, ok);
    return ok;
  };
  for (const n of nodes) {
    if (n === "binary") continue;
    if (reachesBinary(n, new Set())) continue;
    breaches.push({
      id: `stdio-dag-term-${n}`,
      summary: `stdio format "${n}" does not terminate at binary via depends`,
      kind: "stdio-artifacts/io-terminality",
      scope: POLICY_STDIO_PLUGIN_REL,
      priority: "high",
      reason: "Every stdio artifact dependency chain must eventually reach the binary root.",
      solution: `Fix dependencies for "${n}" until binary is reachable.`,
    });
  }
  return breaches;
}

/** 🚨️Banned stub codec markers (SRAS, IFCCARTOONMESH, minimal stubs) must not remain in Rust sources. */
export function policyCodecFidelityBreaches(repoRoot: string): BreachRecord[] {
  const breaches: BreachRecord[] = [];
  const scanRoots = [
    "🧰️framework/🔨️modules/🔺️mesh",
    "🧰️framework/🛍️products/💻️os",
    "🧰️framework/🛍️products/💻️os/🖥️host",
    "✏️s/🔌️plugins",
  ];
  const rsFiles = policyWalkRelFiles(repoRoot, scanRoots, (_p, name) => name.endsWith(".rs"));
  for (const rel of rsFiles) {
    const body = policyReadFileSafe(repoRoot, rel);
    for (const marker of POLICY_STDIO_CODEC_BANNED_MARKERS) {
      if (!body.includes(marker)) continue;
      breaches.push({
        id: `stdio-codec-ban-${rel}-${marker}`,
        summary: `"${rel}" contains banned stub marker ${JSON.stringify(marker)}`,
        kind: "stdio-artifacts/codec-fidelity",
        scope: rel,
        priority: "high",
        reason: "Framework and plugin codecs must be real round-trip implementations, not SRAS/IFCCARTOONMESH/minimal stubs.",
        solution: `Replace the stub in ${rel} with a stdio-owned codec or delete the dead path.`,
      });
    }
  }
  return breaches;
}
