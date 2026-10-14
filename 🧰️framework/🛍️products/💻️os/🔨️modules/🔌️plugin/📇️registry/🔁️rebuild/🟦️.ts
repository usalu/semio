/** 🔁️ Validates owner-authored rebuild contributions and executes one cancellable leased chain. */
import { readFileSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { BundleScript, scriptInvocationBudget, type ScriptInvocation } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { runOwnedCommand } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import { acquireQueuedResourceLease } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { repoCacheDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import rebuildSchema from "./🧬️schema/🔣️.json";

/** 🧱️ Dependency order shared by general and deployment-owned contributions. */
export const REBUILD_STAGES = Object.freeze(rebuildSchema.$defs.RebuildStepV1.properties.stage.enum) as readonly ("inputs" | "descriptors" | "registry" | "guests" | "catalog")[];
/** 🚦️ Serializes descriptor, registry, staging and catalog publication. */
export const WASM_BUILD_LEASE = "wasm-build";
/** 🪜️ One owner-authored Nx operation. */
export type RebuildStepV1 = Readonly<{ id: string; stage: (typeof REBUILD_STAGES)[number]; command: readonly string[] }>;
/** 🧊️ One portable workspace-owned guest compilation check. */
export type GuestFrameworkCheckV1 = Readonly<{ target: "wasm32-wasip2" | "wasm32-unknown-unknown"; workspace: string; packages: readonly string[]; features: readonly string[] }>;
/** 📜️ Explicit contribution input; concrete callers compose their owned stages. */
export type RebuildPlanV1 = Readonly<{ schema: "semio.plugin-registry.rebuild-chain/v1"; steps: readonly RebuildStepV1[]; guestFrameworkChecks: readonly GuestFrameworkCheckV1[] }>;

const keys = (value: object, required: readonly string[]): boolean => Object.keys(value).sort().join("\0") === [...required].sort().join("\0");
const record = (value: unknown): value is Record<string, unknown> => value !== null && typeof value === "object" && !Array.isArray(value);
const matching = (values: unknown, pattern: string, minimum: number): values is string[] => Array.isArray(values) && values.length >= minimum && new Set(values).size === values.length && values.every(value => typeof value === "string" && new RegExp(pattern, "u").test(value));

/** 🔎️ Admits the exact schema and cross-field uniqueness, stage and crate-feature laws. */
export function parseRebuildPlanV1(value: unknown): RebuildPlanV1 {
  if (!record(value) || !keys(value, rebuildSchema.required) || value.schema !== rebuildSchema.properties.schema.const || !Array.isArray(value.steps) || value.steps.length === 0 || !Array.isArray(value.guestFrameworkChecks)) throw Error("rebuild plan is not an exact semio.plugin-registry.rebuild-chain/v1 document");
  const ids = new Set<string>();
  let stage = 0;
  const steps = value.steps.map(value => {
    const schema = rebuildSchema.$defs.RebuildStepV1;
    if (!record(value) || !keys(value, schema.required) || typeof value.id !== "string" || !new RegExp(schema.properties.id.pattern, "u").test(value.id) || !Array.isArray(value.command) || value.command.length < schema.properties.command.minItems || value.command[0] !== "nx" || value.command.some(part => typeof part !== "string" || !part.length)) throw Error("rebuild step requires exact keys, a valid id and an nx command");
    const next = REBUILD_STAGES.indexOf(value.stage as RebuildStepV1["stage"]);
    if (ids.has(value.id) || next < stage) throw Error(`rebuild chain step ${value.id} is duplicated or out of stage order`);
    ids.add(value.id);
    stage = next;
    return Object.freeze({ id: value.id, stage: value.stage, command: Object.freeze([...value.command]) }) as RebuildStepV1;
  });
  const guestFrameworkChecks = value.guestFrameworkChecks.map(value => {
    const schema = rebuildSchema.$defs.GuestFrameworkCheckV1;
    if (!record(value) || !keys(value, schema.required)) throw Error("guest-framework check declares missing or unknown keys");
    if (!schema.properties.target.enum.includes(value.target as string) || !matching(value.packages, schema.properties.packages.items.pattern, 1) || !matching(value.features, schema.properties.features.items.pattern, 0) || !value.features.every(feature => (value.packages as string[]).includes(feature.split("/")[0]!))) throw Error("guest-framework check requires a known target, distinct crates and crate-qualified features");
    if (typeof value.workspace !== "string" || value.workspace.length > schema.properties.workspace.maxLength || !new RegExp(schema.properties.workspace.pattern, "u").test(value.workspace)) throw Error("guest-framework check requires a portable workspace Cargo.toml");
    return Object.freeze({ target: value.target, workspace: value.workspace, packages: Object.freeze([...value.packages]), features: Object.freeze([...value.features]) }) as GuestFrameworkCheckV1;
  });
  return Object.freeze({ schema: "semio.plugin-registry.rebuild-chain/v1", steps: Object.freeze(steps), guestFrameworkChecks: Object.freeze(guestFrameworkChecks) });
}

/** 📖️ Reads the neutral plan by default or an explicit owner-supplied file. */
export function readRebuildPlanV1(path = join(dirname(fileURLToPath(import.meta.url)), "🔣️.json")): RebuildPlanV1 {
  return parseRebuildPlanV1(JSON.parse(readFileSync(path, "utf8")));
}

/** 🧩️ Merges explicit contributions in stage order while preserving owner order within each stage. */
export function composeRebuildPlansV1(plans: readonly RebuildPlanV1[]): RebuildPlanV1 {
  const admitted = plans.map(parseRebuildPlanV1);
  return parseRebuildPlanV1({ schema: "semio.plugin-registry.rebuild-chain/v1", steps: admitted.flatMap(plan => plan.steps).sort((a, b) => REBUILD_STAGES.indexOf(a.stage) - REBUILD_STAGES.indexOf(b.stage)), guestFrameworkChecks: admitted.flatMap(plan => plan.guestFrameworkChecks) });
}

/** 🧊️ Produces portable Cargo arguments from the workspace-owned contract. */
export function guestFrameworkCheckArgs(check: GuestFrameworkCheckV1): readonly string[] {
  return Object.freeze(["check", "--manifest-path", check.workspace, "--lib", "--target", check.target, ...check.packages.flatMap(name => ["-p", name]), ...(check.features.length ? ["--features", check.features.join(",")] : [])]);
}

/** 🧊️ Runs supplied guest checks with progress, bounded execution and process-tree cancellation. */
export class GuestFrameworkCheckScript extends BundleScript {
  constructor(root: string, repoRoot: string, invocation: ScriptInvocation, private readonly plan: RebuildPlanV1 = readRebuildPlanV1()) { super(root, repoRoot, invocation); }
  async run(segments: string[]): Promise<void> {
    if (segments.length) throw Error("usage: guest-framework-check");
    for (const check of parseRebuildPlanV1(this.plan).guestFrameworkChecks) {
      const started = Date.now();
      console.log(`guest-framework-check ${check.target} (${check.workspace}): ${check.packages.join(", ")}`);
      await runOwnedCommand("cargo", [...guestFrameworkCheckArgs(check), "--message-format=json"], this.repoRoot, "guest-framework-check", scriptInvocationBudget(this.invocation, 1200000), { signal: this.invocation.control.signal });
      console.log(`guest-framework-check ${check.target} done in ${Math.round((Date.now() - started) / 1000)} s`);
    }
  }
}

/** 🧭️ Selects contiguous ranges and rejects unknown, repeated or inverted bounds. */
export function selectRebuildSteps(chain: readonly RebuildStepV1[], segments: readonly string[]): readonly RebuildStepV1[] {
  const bound = (flag: string, fallback: number): number => {
    const index = segments.indexOf(flag);
    if (index === -1) return fallback;
    const position = chain.findIndex(step => step.id === segments[index + 1]);
    if (position === -1) throw Error(`${flag} names no step (${chain.map(step => step.id).join("|")})`);
    return position;
  };
  const flags = segments.filter((_, index) => index % 2 === 0);
  if (segments.length % 2 !== 0 || flags.some(flag => flag !== "--from" && flag !== "--to") || new Set(flags).size !== flags.length) throw Error("usage: rebuild-all [--from <step>] [--to <step>]");
  const from = bound("--from", 0), to = bound("--to", chain.length - 1);
  if (from > to) throw Error("rebuild-all --from comes after --to");
  return chain.slice(from, to + 1);
}

/** 🎛️ Owns execution and lease custody behind the general caller-facing boundary. */
export interface RebuildExecutionPortsV1 {
  acquire(signal: AbortSignal): Promise<Readonly<{ release(): void }>>;
  run(step: RebuildStepV1, signal: AbortSignal, budgetMilliseconds: number): Promise<void>;
}

/** 🏁️ Executes an explicit chain under one exclusive wasm-build lease. */
export class RebuildAllScript extends BundleScript {
  constructor(root: string, repoRoot: string, invocation: ScriptInvocation, private readonly plan: RebuildPlanV1 = readRebuildPlanV1(), private readonly execution?: RebuildExecutionPortsV1) { super(root, repoRoot, invocation); }
  async run(segments: string[]): Promise<void> {
    const chain = parseRebuildPlanV1(this.plan).steps, steps = selectRebuildSteps(chain, segments);
    const signal = this.invocation.control.signal, waited = Date.now();
    const execution = this.execution ?? {
      acquire: (signal: AbortSignal) => acquireQueuedResourceLease({ directory: repoCacheDirectory(this.repoRoot, "agents", "resource-leases"), resource: WASM_BUILD_LEASE, mode: "exclusive", owner: "rebuild-all", signal }),
      run: (step: RebuildStepV1, signal: AbortSignal, budgetMilliseconds: number) => runOwnedCommand(process.execPath, [...step.command], this.repoRoot, `rebuild-all:${step.id}`, budgetMilliseconds, { signal }),
    };
    const lease = await execution.acquire(signal);
    console.log(`rebuild-all holds ${WASM_BUILD_LEASE} after ${Math.round((Date.now() - waited) / 1000)} s`);
    try {
      for (const step of steps) {
        const budgetMilliseconds = scriptInvocationBudget(this.invocation, 7200000), index = chain.indexOf(step), started = Date.now();
        console.log(`rebuild-all ${index + 1}/${chain.length} ${step.id} (${step.stage}) started`);
        await execution.run(step, signal, budgetMilliseconds);
        console.log(`rebuild-all ${index + 1}/${chain.length} ${step.id} done in ${Math.round((Date.now() - started) / 1000)} s`);
      }
    } finally { lease.release(); }
  }
}

/** 📋️ Emits the validated plan for observable runtime verification. */
export function printRebuildPlanV1(plan: RebuildPlanV1): void {
  const admitted = parseRebuildPlanV1(plan);
  for (const step of admitted.steps) console.log(`rebuild-plan ${step.id} (${step.stage}): ${step.command.join(" ")}`);
  console.log(`rebuild-plan steps=${admitted.steps.length} guestChecks=${admitted.guestFrameworkChecks.length}`);
}

/** 🔬️ Observes the validated neutral plan without expensive component compilation. */
export class RebuildPlanCheckScript extends BundleScript {
  run(args: string[]): void {
    if (args.length) throw Error("plan-check accepts no arguments");
    printRebuildPlanV1(readRebuildPlanV1());
  }
}
