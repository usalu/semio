/** 🔁️ The one registered all-plugin rebuild: the build-input gates (no cargo unit built from outside this tree, a fresh
 * mutation-authority projection, the guest-linked framework crates compiling for every wasm target), every component built ONCE for both its descriptor and its dev staging, the generated registry
 * and its check, the restaged `s` guests and their convergence proof, the flow-core browser bindings, then the catalog preflight
 * (seconds) and the trusted catalog of every package, from one tree, in the order `🔣️.json` declares. The whole span runs inside ONE queued exclusive `wasm-build`
 * lease, so no other all-plugin wasm build lands between a descriptor and the staging it describes. Each step is an
 * existing product verb run through nx; a failing step stops the chain. `--from <step>` resumes, `--to <step>` stops. */

import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { getWorkspaceRoot, orchestratorBudgetOpts, runCmd } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { BundleScript } from "../../../../../../🔨️modules/🏃️process/🧭️routing/🟦️.ts";
import { acquireQueuedResourceLease } from "../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import { repoCacheDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { developmentRuntimeRoot, pluginModulesRoot, readActivationReceipt } from "../../../🧑‍💻dev/♻️activation/🟦️.ts";
import { moduleDirectoryName } from "../📦️deployment/🟦️.ts";
import { REGISTRY_DIAGNOSTICS_FILE, parseRegistryChannelDiagnosticsV1 } from "../🔎️discovery/🟦️.ts";
import { readGeneratedCatalogProjection, registryModuleDirectories } from "../📖️catalog-view/🟦️.ts";
import rebuildSchema from "./🧬️schema/🔣️.json";

const guestCheckSchema = rebuildSchema.$defs.GuestFrameworkCheckV1;
const guestCheckKeys = [...guestCheckSchema.required].sort().join("\0");
const guestCrate = new RegExp(guestCheckSchema.properties.packages.items.pattern, "u");
const guestFeature = new RegExp(guestCheckSchema.properties.features.items.pattern, "u");
const guestWorkspace = new RegExp(guestCheckSchema.properties.workspace.pattern, "u");

/** 🧱️ Where a step sits in the dependency order: the input gates guard every build, descriptors feed the registry, the registry feeds the guests, the guests feed the catalog. */
export const REBUILD_STAGES = ["inputs", "descriptors", "registry", "guests", "catalog"] as const;

/** 🚦️ The repository resource every all-plugin wasm build serializes on (`lease exclusive wasm-build …` for shell callers). */
export const WASM_BUILD_LEASE = "wasm-build";

/** 🪜️ One declared step of the chain. */
export type RebuildStepV1 = Readonly<{ id: string; stage: (typeof REBUILD_STAGES)[number]; command: readonly string[] }>;

/** 📜️ Reads and validates the declared chain: known schema, unique ids, nx commands, stages in dependency order. */
export function readRebuildChain(path = join(dirname(fileURLToPath(import.meta.url)), "🔣️.json")): readonly RebuildStepV1[] {
  const document = JSON.parse(readFileSync(path, "utf8")) as { schema?: unknown; steps?: unknown };
  if (document.schema !== "semio.plugin-registry.rebuild-chain/v1" || !Array.isArray(document.steps) || document.steps.length === 0) throw new Error("rebuild chain is not a semio.plugin-registry.rebuild-chain/v1 document");
  const ids = new Set<string>();
  let stage = 0;
  return Object.freeze(
    document.steps.map((value: any) => {
      const next = REBUILD_STAGES.indexOf(value?.stage);
      if (typeof value?.id !== "string" || ids.has(value.id) || next < stage || !Array.isArray(value.command) || value.command[0] !== "nx" || value.command.some((part: unknown) => typeof part !== "string"))
        throw new Error(`rebuild chain step ${String(value?.id)} is duplicated, out of stage order, or not an nx command`);
      ids.add(value.id);
      stage = next;
      return Object.freeze({ id: value.id, stage: value.stage, command: Object.freeze([...value.command]) }) as RebuildStepV1;
    }),
  );
}

/** 🧊️ One fail-fast `cargo check --lib` of guest-linked framework crates for one wasm target inside the one cargo workspace
 * (`workspace`, repo-relative `Cargo.toml`) that owns them, as `🔣️.json` declares it. */
export type GuestFrameworkCheckV1 = Readonly<{ target: "wasm32-wasip2" | "wasm32-unknown-unknown"; workspace: string; packages: readonly string[]; features: readonly string[] }>;

/** 📜️ Reads the declared guest-framework checks against `🧬️schema/🔣️.json` `GuestFrameworkCheckV1`: exact keys, known targets, a
 * portable repo-relative workspace manifest (`/` segments only, no `.`/`..`, `\` or `:`), distinct crate names, features naming a checked crate. */
export function readGuestFrameworkChecks(path = join(dirname(fileURLToPath(import.meta.url)), "🔣️.json")): readonly GuestFrameworkCheckV1[] {
  const document = JSON.parse(readFileSync(path, "utf8")) as { guestFrameworkChecks?: unknown };
  if (!Array.isArray(document.guestFrameworkChecks) || document.guestFrameworkChecks.length === 0) throw new Error("rebuild chain declares no guestFrameworkChecks");
  return Object.freeze(
    document.guestFrameworkChecks.map((value: any) => {
      const packages = value?.packages, features = value?.features, workspace = value?.workspace;
      if (!guestCheckSchema.properties.target.enum.includes(value?.target) || !Array.isArray(packages) || packages.length === 0 || new Set(packages).size !== packages.length || !packages.every((name: unknown) => typeof name === "string" && guestCrate.test(name))
        || !Array.isArray(features) || new Set(features).size !== features.length || !features.every((feature: unknown) => typeof feature === "string" && guestFeature.test(feature) && packages.includes(feature.split("/")[0])))
        throw new Error(`rebuild chain guest-framework check ${JSON.stringify(value)} is not a known target with distinct crates and crate-qualified features`);
      if (typeof workspace !== "string" || workspace.length > guestCheckSchema.properties.workspace.maxLength || !guestWorkspace.test(workspace))
        throw new Error(`rebuild chain guest-framework check ${JSON.stringify(value)} names no repo-relative workspace Cargo.toml`);
      if (Object.keys(value).sort().join("\0") !== guestCheckKeys) throw new Error(`rebuild chain guest-framework check ${JSON.stringify(value)} declares unknown keys`);
      return Object.freeze({ target: value.target, workspace, packages: Object.freeze([...packages]), features: Object.freeze([...features]) }) as GuestFrameworkCheckV1;
    }),
  );
}

/** 🧊️ The cargo argument vector of one declared guest-framework check, scoped to its declared workspace. */
export function guestFrameworkCheckArgs(check: GuestFrameworkCheckV1): readonly string[] {
  return Object.freeze(["check", "--manifest-path", check.workspace, "--lib", "--target", check.target, ...check.packages.flatMap((name) => ["-p", name]), ...(check.features.length ? ["--features", check.features.join(",")] : [])]);
}

/** 🧊️ `guest-framework-check`: the declared checks in order; cargo stops at the first crate that fails, so a broken guest-linked framework
 * crate refuses the rebuild in minutes instead of inside the 60-component build. */
export class GuestFrameworkCheckScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length) throw new Error("usage: guest-framework-check");
    const repoRoot = getWorkspaceRoot();
    for (const check of readGuestFrameworkChecks()) {
      const started = Date.now();
      console.log(`guest-framework-check ${check.target} (${check.workspace}): ${check.packages.join(", ")}`);
      runCmd("cargo", [...guestFrameworkCheckArgs(check)], { cwd: repoRoot, ...orchestratorBudgetOpts() });
      console.log(`guest-framework-check ${check.target} done in ${Math.round((Date.now() - started) / 1000)} s`);
    }
  }
}

/** 🧭️ The contiguous step range `--from`/`--to` select, refusing an unknown or inverted bound. */
export function selectRebuildSteps(chain: readonly RebuildStepV1[], segments: readonly string[]): readonly RebuildStepV1[] {
  const bound = (flag: string, fallback: number): number => {
    const index = segments.indexOf(flag);
    if (index === -1) return fallback;
    const position = chain.findIndex((step) => step.id === segments[index + 1]);
    if (position === -1) throw new Error(`${flag} names no step (${chain.map((step) => step.id).join("|")})`);
    return position;
  };
  const flags = segments.filter((_, index) => index % 2 === 0);
  if (segments.length % 2 !== 0 || flags.some((flag) => flag !== "--from" && flag !== "--to") || new Set(flags).size !== flags.length) throw new Error("usage: rebuild-all [--from <step>] [--to <step>]");
  const from = bound("--from", 0);
  const to = bound("--to", chain.length - 1);
  if (from > to) throw new Error("rebuild-all --from comes after --to");
  return chain.slice(from, to + 1);
}

/** 🏁️ `rebuild-all [--from <step>] [--to <step>]`: runs the selected steps in order under one queued exclusive
 * `wasm-build` lease, with a numbered progress line per step; cancellation (SIGINT/SIGTERM) releases the lease. */
export class RebuildAllScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const chain = readRebuildChain();
    const steps = selectRebuildSteps(chain, segments);
    const repoRoot = getWorkspaceRoot();
    const controller = new AbortController(), abort = () => controller.abort();
    process.once("SIGINT", abort);
    process.once("SIGTERM", abort);
    const waited = Date.now();
    const lease = await acquireQueuedResourceLease({ directory: repoCacheDirectory(repoRoot, "agents", "resource-leases"), resource: WASM_BUILD_LEASE, mode: "exclusive", owner: "rebuild-all", signal: controller.signal });
    console.log(`rebuild-all holds ${WASM_BUILD_LEASE} after ${Math.round((Date.now() - waited) / 1000)} s`);
    try {
      for (const step of steps) {
        const index = chain.indexOf(step);
        const started = Date.now();
        console.log(`rebuild-all ${index + 1}/${chain.length} ${step.id} (${step.stage}) started`);
        runCmd("bun", [...step.command], { cwd: repoRoot, ...orchestratorBudgetOpts() });
        console.log(`rebuild-all ${index + 1}/${chain.length} ${step.id} done in ${Math.round((Date.now() - started) / 1000)} s`);
      }
    } finally {
      lease.release();
    }
  }
}

/** 🧾️ One component's convergence row: its committed descriptor, its `dist/component-dev` deliverable, its dev staging
 * and the activation receipt must name ONE build. */
export type StagedConvergenceRowV1 = Readonly<{ pluginId: string; committed: string; dist: string; staged: string; activated: boolean; missing: readonly string[]; ok: boolean }>;

/** 🔍️ Proves `committed == dist == staged` and full staging for every registry component of one dev variant; every plugin the dev catalog withholds (§21.4 stale channel) is a refused row. */
export function stagedConvergence(repoRoot: string, variant: string): Readonly<{ rows: readonly StagedConvergenceRowV1[]; receiptPlugins: number }> {
  const sha = (path: string): string => (existsSync(path) ? createHash("sha256").update(readFileSync(path)).digest("hex") : "-");
  const registry = readGeneratedCatalogProjection(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated")).entries;
  const inventory = registryModuleDirectories(registry);
  const receipt = readActivationReceipt(join(developmentRuntimeRoot(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript"), variant, "dev", "react"), "activation"));
  const activated = new Set(receipt.plugins.map((row) => row.pluginId));
  const modules = pluginModulesRoot("dev");
  const rows = registry.map((entry) => {
    const descriptor = join(repoRoot, entry.cratePath, "..", "..", "🔣️.json");
    const committed = existsSync(descriptor) ? String((JSON.parse(readFileSync(descriptor, "utf8")) as { hashes?: { wasmSha256?: string } }).hashes?.wasmSha256 ?? "-") : "-";
    const dist = sha(join(repoRoot, entry.cratePath, "dist", "component-dev", entry.wasmOut));
    const moduleRoot = join(modules, moduleDirectoryName(entry.pluginId, inventory));
    const stagedDescriptor = join(moduleRoot, "🔣️.json");
    const staged = existsSync(stagedDescriptor) ? String((JSON.parse(readFileSync(stagedDescriptor, "utf8")) as { hashes?: { wasmSha256?: string } }).hashes?.wasmSha256 ?? "-") : "-";
    const manifest = join(moduleRoot, ".nx-artifact.json");
    const files = existsSync(manifest) ? (JSON.parse(readFileSync(manifest, "utf8")) as { files: string[] }).files : [];
    const missing = [...files.filter((file) => !existsSync(join(moduleRoot, file))), ...(files.some((file) => file.endsWith("_component.core.wasm")) ? [] : ["*_component.core.wasm"])];
    const ok = committed !== "-" && committed === dist && dist === staged && activated.has(entry.pluginId) && missing.length === 0;
    return Object.freeze({ pluginId: entry.pluginId, committed, dist, staged, activated: activated.has(entry.pluginId), missing: Object.freeze(missing), ok });
  });
  const diagnostics = join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated", REGISTRY_DIAGNOSTICS_FILE);
  const withheld = existsSync(diagnostics) ? parseRegistryChannelDiagnosticsV1(readFileSync(diagnostics, "utf8")).map((row) => Object.freeze({ pluginId: row.pluginId, committed: row.code, dist: "-", staged: "-", activated: activated.has(row.pluginId), missing: Object.freeze([]), ok: false })) : [];
  return Object.freeze({ rows: Object.freeze([...rows, ...withheld]), receiptPlugins: receipt.plugins.length });
}

/** ✅️ `verify-staged --variant <variant>`: one row per component, the summary line, exit 1 when any component diverged. */
export class VerifyStagedScript extends BundleScript {
  run(segments: string[]): void {
    if (segments.length !== 2 || segments[0] !== "--variant") throw new Error("usage: verify-staged --variant <variant>");
    const { rows, receiptPlugins } = stagedConvergence(getWorkspaceRoot(), segments[1]!);
    for (const row of rows) console.log(`${row.ok ? "OK  " : "DIFF"} ${row.pluginId.padEnd(34)} committed=${row.committed.slice(0, 12)} dist=${row.dist.slice(0, 12)} staged=${row.staged.slice(0, 12)} activated=${row.activated}${row.missing.length ? ` missing=${row.missing.slice(0, 3).join(",")}` : ""}`);
    const diverged = rows.filter((row) => !row.ok).length;
    console.log(`components=${rows.length} consistent=${rows.length - diverged} diverged=${diverged} receiptPlugins=${receiptPlugins}`);
    if (diverged > 0) process.exitCode = 1;
  }
}
