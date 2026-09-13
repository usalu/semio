import { readSelectors } from "../../🔍️discovery/🎛️selection/🟦️.ts";
import {
  type MutationManifest,
  type OracleRegistry,
  isQualifyingOracleKind,
  leafDescriptorCoverage,
  loadOracleRegistry,
  manifestFromLeafDescriptors,
  readRuntimeInventory,
  resolvePayloadSchemas,
  scaffoldOwnerDescriptors,
  subsetCoordinatesOfOwner,
  testFilenameForKind,
  testTaxonomy,
} from "../../📦️packages/🟦️typescript/🟦️.ts";
import { Script } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { existsSync, readFileSync, writeFileSync } from "node:fs";
import { join } from "node:path";

/**
 * 🕳️ What each owner still owes before its mutations are externally oracled at subset level.
 *
 * The target is that EVERY mutation of every artifact is predicted by a third-party library, scoped to
 * the smallest semantic subset. `matrix` measures what is covered; this answers the complementary and
 * more actionable question — what is missing, per owner, and which of four things it is. Without it the
 * shortfall is a single percentage, and a percentage tells nobody what to do on Monday.
 */
/**
 * 🧬️ Reports which mutation leaves DECLARE a payload contract, and what defeats the rest.
 *
 * It reads; it never derives. A payload schema used to be reconstructed by grep-scanning every Rust
 * file in the repository for a same-named struct and picking the definition whose directory sat
 * closest to the leaf — so a contract's content depended on which same-named type happened to be
 * nearest, and nobody had declared any of it. The leaf is the declared authority
 * (`mutationPayloadSchemaAuthority`), its descriptor names the one path, and a leaf that declares
 * nothing is REPORTED as declaring nothing rather than having a contract invented for it.
 *
 *   bun 📜️script.ts manifest payload-schema        # who declares a payload contract, and who does not
 */
export function payloadSchemaCommand(repoRoot: string, registry: OracleRegistry, selectors: ReturnType<typeof readSelectors>): void {
  const owners = [...new Set(registry.contributions.map((entry) => entry.owner))].filter((owner) => selectors.subset === null || subsetCoordinatesOfOwner(owner)?.subset === selectors.subset);
  let leaves = 0;
  let declared = 0;
  const refusals = new Map<string, number>();
  const undeclared: string[] = [];
  for (const owner of owners) {
    for (const row of resolvePayloadSchemas(repoRoot, owner)) {
      leaves += 1;
      if (row.schema !== null) {
        declared += 1;
        continue;
      }
      undeclared.push(row.leaf);
      // 🧾️Group by the KIND of refusal, not by the path it names, so the report says how many leaves
      // have no descriptor at all versus how many name a contract file that is not there.
      for (const why of row.refused) {
        const kind = why
          .replace(/`[^`]*`/g, "…")
          .replace(/\S*🧬️schema\S*/gu, "the declared path")
          .replace(/\S+\/🔣️\.json/gu, "the declared path");
        refusals.set(kind, (refusals.get(kind) ?? 0) + 1);
      }
    }
  }
  console.log(`[manifest payload-schema] ${declared}/${leaves} leaves declare a payload contract at the taxonomy location (${((declared / Math.max(leaves, 1)) * 100).toFixed(1)}%)`);
  for (const [why, count] of [...refusals].sort((a, b) => b[1] - a[1]).slice(0, 15)) console.log(`[manifest payload-schema]   ${String(count).padStart(4)} × ${why.slice(0, 96)}`);
  for (const leaf of undeclared.slice(0, 20)) console.log(`[manifest payload-schema]   undeclared: ${leaf}`);
  if (undeclared.length > 20) console.log(`[manifest payload-schema]   … and ${undeclared.length - 20} more`);
}

/** 🏗️ Derives leaf descriptors from the leaves themselves, refusing every field it cannot cite. */
export function scaffoldCommand(repoRoot: string, registry: OracleRegistry, selectors: ReturnType<typeof readSelectors>, write: boolean): void {
  const owners = [...new Set(registry.contributions.map((entry) => entry.owner))].filter((owner) => selectors.subset === null || subsetCoordinatesOfOwner(owner)?.subset === selectors.subset);
  let leaves = 0;
  let derived = 0;
  let written = 0;
  const refusals = new Map<string, number>();
  const ready: string[] = [];
  const taxonomy = testTaxonomy(repoRoot);
  const filename = testFilenameForKind(taxonomy, taxonomy.testContributionFileKindId);
  for (const owner of owners) {
    const rows = scaffoldOwnerDescriptors(repoRoot, owner);
    if (rows.length === 0) continue;
    leaves += rows.length;
    derived += rows.filter((row) => row.descriptor !== null).length;
    for (const row of rows) for (const why of row.refused) refusals.set(why.split(":")[0]!, (refusals.get(why.split(":")[0]!) ?? 0) + 1);
    // 🏗️An owner is written ALL-OR-NOTHING. A partial descriptor set would let a manifest be generated
    // over a denominator that silently omits the undescribed leaves, which reads as coverage of a
    // smaller vocabulary rather than as the gap it is.
    if (rows.some((row) => row.descriptor === null)) continue;
    ready.push(`${owner} (${rows.length} leaves)`);
    if (!write) continue;
    for (const row of rows) {
      const path = join(repoRoot, row.leaf, filename);
      if (existsSync(path)) continue;
      writeFileSync(path, `${JSON.stringify(row.descriptor, null, 2)}\n`);
      written += 1;
    }
  }
  console.log(`[manifest scaffold] ${derived}/${leaves} leaves derivable with full evidence (${((derived / Math.max(leaves, 1)) * 100).toFixed(1)}%)`);
  for (const [field, count] of [...refusals].sort((a, b) => b[1] - a[1])) console.log(`[manifest scaffold] refused ${String(count).padStart(5)} × ${field}`);
  console.log(`[manifest scaffold] ${ready.length} owner(s) fully derivable:`);
  for (const owner of ready) console.log(`[manifest scaffold]   ${owner}`);
  console.log(write ? `[manifest scaffold] ${written} descriptor(s) written` : "[manifest scaffold] dry run — pass --write to emit descriptors for the fully derivable owners");
}

export class GapScript extends Script {
  run(segments: string[]): void {
    const registry = loadOracleRegistry(this.repoRoot);
    const selectors = readSelectors(segments);
    const manifested = new Set(registry.mutationManifests.flatMap((manifest) => manifest.mutations.map((mutation) => mutation.capability)));
    // 🧫️A capability is only genuinely covered once fixtures exist for it too. Counting a manifest as
    // sufficient would repeat the error this whole protocol exists to remove.
    const fixtured = new Set(registry.contributions.flatMap((contribution) => contribution.fixtureManifests.map((fixture) => `${fixture.target.artifact}@${fixture.target.standard}/${fixture.target.subset}`)));
    const fixtureCountFor = (artifact: string, standard: string, subset: string): number =>
      registry.contributions.flatMap((contribution) => contribution.fixtureManifests).filter((fixture) => fixture.target.artifact === artifact && fixture.target.standard === standard && fixture.target.subset === subset).length;

    type Row = { owner: string; catalog: string; capability: string; kinds: number; subset: string; state: string; oracles: string; owed: string; fixtures: number };
    const rows: Row[] = [];
    for (const contribution of registry.contributions) {
      for (const catalog of contribution.mutationCatalogs) {
        const coordinates = subsetCoordinatesOfOwner(contribution.owner);
        const subset = coordinates?.subset ?? "";
        if (selectors.subset !== null && subset !== selectors.subset) continue;
        const supplying = registry.oracles.filter((oracle) => oracle.capabilities.includes(catalog.capability));
        const qualifying = supplying.filter((oracle) => isQualifyingOracleKind(oracle.kind));
        const state = qualifying.length > 0 ? (manifested.has(catalog.capability) ? "covered" : "manifestable") : supplying.length > 0 ? "supplemental-only" : "un-oracled";
        if (selectors.status !== null && state !== selectors.status) continue;
        // 🕳️What is OWED, stated in full. An earlier version said "only a manifest" for the qualifying
        // group, and that was an understatement of exactly the kind this protocol exists to remove: a
        // manifest also needs the OUTCOME CLASSES each mutation can reach, which nothing can state
        // honestly until the production bridge has been run, and every mutation needs a fixture whose
        // expected result that oracle actually produced. A qualifying oracle is the PREREQUISITE, not
        // the remaining work.
        const owed =
          state === "covered"
            ? "—"
            : state === "manifestable"
              ? "a manifest (needs each mutation's OUTCOME CLASSES, which only the production bridge can state), a runtime inventory, and a fixture per mutation × outcome — the oracle is in place"
              : state === "supplemental-only"
                ? `a QUALIFYING third-party oracle before anything else; today only ${supplying.map((oracle) => `${oracle.id}(${oracle.kind ?? "unclassified"})`).join(", ")}`
                : "a qualifying third-party oracle, and nothing supplies this capability at all";
        const fixtures = coordinates === null ? 0 : fixtureCountFor(catalog.capability.startsWith("step-") ? "s.stdio.step" : "", coordinates.standard, coordinates.subset);
        rows.push({ owner: contribution.owner, catalog: catalog.id, capability: catalog.capability, kinds: catalog.kinds.length, subset, state, oracles: qualifying.map((oracle) => oracle.id).join(",") || "—", owed, fixtures });
      }
    }

    if (segments.includes("--json")) {
      console.log(JSON.stringify(rows, null, 2));
      return;
    }

    const byState = new Map<string, Row[]>();
    for (const row of rows) byState.set(row.state, [...(byState.get(row.state) ?? []), row]);
    const mutationsIn = (state: string): number => (byState.get(state) ?? []).reduce((total, row) => total + row.kinds, 0);
    for (const state of ["covered", "manifestable", "supplemental-only", "un-oracled"]) {
      const group = byState.get(state) ?? [];
      console.log(`\n[gap] ${state.toUpperCase()} — ${group.length} catalog(s), ${mutationsIn(state)} mutation kind(s)`);
      for (const row of group.slice(0, segments.includes("--all") ? group.length : 12)) {
        console.log(`[gap]   ${row.capability.padEnd(34)} ${String(row.kinds).padStart(4)} kinds  ${String(row.fixtures).padStart(4)} fixtures  ${row.subset.padEnd(10)} ${row.owed}`);
      }
      if (!segments.includes("--all") && group.length > 12) console.log(`[gap]   … and ${group.length - 12} more (--all to list, --json for the full record)`);
    }

    const total = rows.reduce((sum, row) => sum + row.kinds, 0);
    const covered = mutationsIn("covered");
    console.log(`\n[gap] ${covered}/${total} mutation kind(s) are externally oracled and manifested — ${((covered / Math.max(total, 1)) * 100).toFixed(1)}%`);
    console.log(`[gap] ${mutationsIn("manifestable")} kind(s) have a qualifying oracle and still need a manifest, a runtime inventory and fixtures`);
    console.log(`[gap] ${mutationsIn("supplemental-only") + mutationsIn("un-oracled")} kind(s) need a qualifying third-party oracle BEFORE any of that`);
    // 🏭️Nothing above can be completed while the production bridge cannot run: a manifest must declare
    // the outcome classes each mutation reaches, and only dispatch can state them.
    const inventories = registry.mutationManifests.filter((manifest) => readRuntimeInventory(this.repoRoot, manifest) !== null).length;
    console.log(`[gap] ${inventories}/${registry.mutationManifests.length} manifest(s) have a runtime inventory — a manifest's outcome classes cannot be stated honestly without one`);
    // 🕳️A cross-semio implementation is the single largest category, and naming it as such is the point:
    // it reads as an oracle in the registry and discharges nothing.
    const semioDerived = registry.oracles.filter((oracle) => oracle.kind === "cross-semio-implementation").length;
    console.log(`[gap] ${semioDerived} of ${registry.oracles.length} registered oracles are second implementations written inside this repository, and none of them discharges a mutation's requirement`);
  }
}

/**
 * 🧬️ Generates each owner's v2 mutation manifest FROM ITS LEAF DESCRIPTORS, and reports who cannot yet
 * have one.
 *
 * A manifest must state the OUTCOME CLASSES each mutation can reach, and that is the one field nobody
 * can honestly invent from outside the implementation. The `dsl::Mutations` derive already reads it
 * from a declarative per-leaf JSON descriptor at expansion time — so a manifest built from the same
 * file is generated from production's own record rather than restated beside it, and it needs no
 * compiler, no running bridge and no guess.
 *
 *   bun 📜️script.ts manifest --dry            # who is ready, who is blocked, and on what
 *   bun 📜️script.ts manifest --write          # write manifests for every ready owner
 */
export class ManifestScript extends Script {
  run(segments: string[]): void {
    const registry = loadOracleRegistry(this.repoRoot);
    const selectors = readSelectors(segments);
    const write = segments.includes("--write");
    if (segments.includes("scaffold")) return scaffoldCommand(this.repoRoot, registry, selectors, write);
    if (segments.includes("payload-schema")) return payloadSchemaCommand(this.repoRoot, registry, selectors);

    type Row = { owner: string; capability: string; leaves: number; described: number; ready: boolean; reason: string; manifest: MutationManifest | null };
    const rows: Row[] = [];
    for (const contribution of registry.contributions) {
      for (const catalog of contribution.mutationCatalogs) {
        if (selectors.subset !== null && subsetCoordinatesOfOwner(contribution.owner)?.subset !== selectors.subset) continue;
        const coverage = leafDescriptorCoverage(this.repoRoot, contribution.owner);
        const manifest = manifestFromLeafDescriptors(this.repoRoot, contribution.owner, catalog.capability);
        const qualifying = registry.oracles.filter((oracle) => oracle.capabilities.includes(catalog.capability) && isQualifyingOracleKind(oracle.kind));
        const reason =
          coverage.leaves === 0
            ? "no mutation leaves on disk"
            : coverage.missing.length > 0
              ? `${coverage.missing.length}/${coverage.leaves} leaves carry no descriptor — a manifest whose outcome classes were guessed for even one mutation is worse than none`
              : manifest === null
                ? "leaves are described but the owner path or artifact id could not be resolved"
                : qualifying.length === 0
                  ? "described, but no QUALIFYING third-party oracle supplies this capability — the manifest would declare a requirement nothing can discharge"
                  : "ready";
        rows.push({ owner: contribution.owner, capability: catalog.capability, leaves: coverage.leaves, described: coverage.described, ready: reason === "ready", reason, manifest });
      }
    }

    if (segments.includes("--json")) {
      console.log(
        JSON.stringify(
          rows.map(({ manifest, ...rest }) => ({ ...rest, mutations: manifest?.mutations.length ?? 0 })),
          null,
          2,
        ),
      );
      return;
    }

    const ready = rows.filter((row) => row.ready);
    const described = rows.filter((row) => row.leaves > 0 && row.described === row.leaves);
    const totalLeaves = rows.reduce((sum, row) => sum + row.leaves, 0);
    const totalDescribed = rows.reduce((sum, row) => sum + row.described, 0);

    console.log(`[manifest] ${totalDescribed}/${totalLeaves} mutation leaves carry a descriptor across ${rows.length} catalog(s)`);
    console.log(`[manifest] ${described.length} owner(s) fully described; ${ready.length} of those also have a qualifying oracle and are READY`);
    for (const row of ready) console.log(`[manifest] READY   ${row.capability.padEnd(32)} ${String(row.manifest?.mutations.length).padStart(4)} mutations  ${row.owner}`);
    const blocked = new Map<string, number>();
    for (const row of rows.filter((candidate) => !candidate.ready)) blocked.set(row.reason.split("—")[0]!.trim(), (blocked.get(row.reason.split("—")[0]!.trim()) ?? 0) + 1);
    for (const [reason, count] of [...blocked].sort((a, b) => b[1] - a[1])) console.log(`[manifest] BLOCKED ${String(count).padStart(4)} catalog(s): ${reason}`);

    if (!write) {
      console.log(`[manifest] dry run — pass --write to emit manifests for the ${ready.length} ready owner(s)`);
      return;
    }
    let written = 0;
    for (const row of ready) {
      const contribution = registry.contributions.find((entry) => entry.owner === row.owner);
      if (contribution === undefined || row.manifest === null) continue;
      const path = join(this.repoRoot, contribution.manifestPath);
      const parsed = JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
      const all = (parsed.mutationManifests as MutationManifest[] | undefined) ?? [];
      const prior = all.find((entry) => entry.artifact === row.manifest!.artifact && entry.standard === row.manifest!.standard && entry.subset === row.manifest!.subset);
      const existing = all.filter((entry) => entry !== prior);
      // 🤝️MERGE, NEVER REPLACE. The generator derives STRUCTURE from the leaf descriptors — payload
      // schema, outcome classes, dispatch variant — but it knows nothing about SCOPE: which specific
      // oracle discharges a mutation, and which mutations the carrier provably cannot witness. That is
      // registration work, and a wholesale replace silently undid it: re-running this command flattened
      // `sequence`'s hand-scoped 4-carried/4-uncarried split back to eight undifferentiated mutations,
      // turning an honest partial into a claim of blanket coverage. Refined fields win over derived ones.
      const carried = new Map((prior?.mutations ?? []).map((mutation) => [mutation.id, mutation] as const));
      const merged = {
        ...row.manifest,
        mutations: row.manifest.mutations.map((mutation) => {
          const before = carried.get(mutation.id);
          if (before === undefined) return mutation;
          return {
            ...mutation,
            ...(before.oracleRequirements !== undefined ? { oracleRequirements: before.oracleRequirements } : {}),
            ...((before as { invariants?: unknown }).invariants !== undefined ? { invariants: (before as { invariants?: unknown }).invariants } : {}),
            ...((before as { carriers?: unknown }).carriers !== undefined ? { carriers: (before as { carriers?: unknown }).carriers } : {}),
            ...((before as { comparisonPipeline?: unknown }).comparisonPipeline !== undefined ? { comparisonPipeline: (before as { comparisonPipeline?: unknown }).comparisonPipeline } : {}),
          };
        }),
      };
      parsed.mutationManifests = [...existing, merged];
      parsed.schemaVersion = 2;
      writeFileSync(path, `${JSON.stringify(parsed, null, 2)}\n`);
      written += 1;
      console.log(`[manifest] wrote ${row.manifest.mutations.length} mutation(s) into ${contribution.manifestPath}`);
    }
    console.log(`[manifest] ${written} manifest(s) written`);
  }
}
