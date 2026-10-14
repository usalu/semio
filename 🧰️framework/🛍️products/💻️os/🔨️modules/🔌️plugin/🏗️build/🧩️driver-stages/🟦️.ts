import { requireMaterializedPathV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import type { CompilerAdmissionControl } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { buildModeEnvironment } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import { mkdirSync, rmSync } from "node:fs";
import { availableParallelism } from "node:os";
import { join } from "node:path";
import { runNativeBuildDriverV1, type LinkedComponentV1, type NativeBuildOutcomeV1, type NativeBuildStageV1 } from "../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts";
import { repoCacheDirectory } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🟦️.ts";
import { cargoDirectories, componentPassPlanV1 } from "../../../../../🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🟦️.ts";
import { assembleComponentModuleV1, optimizeComponentCoresV1, probeComponentModuleV1, publishComponentModuleV1, transpileComponentModulesV1, type ComponentModuleWorkV1 } from "../../🌐️browser-bundle/🏗️materialization/🚀️commands/🟦️.ts";
import { componentDeliverableV1, SHIPPED_COMPONENT_PROFILE, componentProfileV1, type ComponentReceiptV1 } from "../../🖨️describe/🏷️component-receipt/🟦️.ts";
import { componentStageTargetV1, componentsDescribeArgumentsV1, describeComponentStageV1, describeIterateComponentsV1, settleComponentDescriptorV1, writeComponentStageReceiptV1, type ComponentDescribeModeV1 } from "../../🖨️describe/🧩️component-stage/🟦️.ts";
import type { SemanticDescriptorEmissionV1 } from "../../🖨️describe/🛂️descriptor-emission/🟦️.ts";
import { PLAYGROUND_BUILD_TARGETS } from "../../📇️registry/🤖️generated/🎮️playgrounds/🟦️.ts";
import { EXTENSION_TARGETS, PLUGIN_BUILD_TARGETS } from "../../📇️registry/🤖️generated/🧩️plugins/🟦️.ts";
import { runtimeComponentClosure } from "../../../../../🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs";

/** 🧩️ The post-link stages of one shipped plugin or extension component inside a build driver, each holding one jobserver token and cached by content:
 * `describe` (the semantic descriptor exactly these bytes answer, verified against the committed pair) and `transpile` start the moment the component is
 * staged and run beside the rest of the compilation; `optimize` (`wasm-opt -Oz` per core) follows the transpile; `probe` runs the assembled module's own
 * `describe()`; `publish` waits for the pass's provenance, writes the receipt bound to the Merkle root of the component's unit closure and publishes the
 * module; the `discard` finalizer then removes an assembled module no publish took and the stage work directory, whatever failed or was cancelled. A component
 * without a deployment directory is described and receipted only. `describeMode` `write` replaces a differing committed pair
 * (`components describe`) instead of refusing it. A plan's `stageAfter` adds the jobs of the same driver a stage
 * consumes (the descriptor emitter before `describe`, browser support before `probe`).
 * @see ../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts
 * @see ../../🖨️describe/🏷️component-receipt/🧬️schema/🔣️.json */
export function componentDriverStagesV1(repoRoot: string, describeMode: Extract<ComponentDescribeModeV1, "verify" | "write"> = "verify"): (component: LinkedComponentV1) => readonly NativeBuildStageV1[] {
  return (component) => {
    const target = componentStageTargetV1(repoRoot, component.package.manifest), deliverable = componentDeliverableV1(target.crateRoot, target.packageName, componentProfileV1(component.pass.profile));
    if (deliverable.wasm !== component.staged) throw new Error(`${target.pluginId}: the driver staged ${component.staged}, the deliverable is ${deliverable.wasm}`);
    let emission: SemanticDescriptorEmissionV1 | undefined, transpiled: string | undefined, optimized: string | null = null, module: string | undefined, semantic: Uint8Array | undefined, work: ComponentModuleWorkV1 | undefined;
    const workOf = (control: CompilerAdmissionControl, cache: ComponentModuleWorkV1["cache"], root: string, environment: Readonly<Record<string, string | undefined>>): ComponentModuleWorkV1 => {
      if (!work) { const directory = requireMaterializedPathV1(join(root, "w", target.packageName)); rmSync(directory, { recursive: true, force: true }); mkdirSync(directory, { recursive: true }); work = { repoRoot, target, deliverable, cache, work: directory, control }; }
      work = { ...work, control, cache, environment }; return work;
    };
    const publish: NativeBuildStageV1 = { name: "publish", after: [...(target.directoryName === undefined ? [] : ["probe"]), "describe", `provenance:${component.pass.id}`], run: async (context) => {
      const provenance = await context.provenance();
      const receipt: ComponentReceiptV1 = writeComponentStageReceiptV1(repoRoot, target, deliverable, emission!, { sourceTreeDigest: provenance.sourceTreeDigest, provenanceSha256: provenance.root });
      if (module) await publishComponentModuleV1(workOf(context.control, context.cache, context.generation.root, context.environment), module, semantic!, receipt);
    } };
    const discard: NativeBuildStageV1 = { name: "discard", after: ["publish"], finalizer: true, run: async () => {
      if (module) rmSync(module, { recursive: true, force: true });
      if (work) rmSync(work.work, { recursive: true, force: true });
    } };
    const describe: NativeBuildStageV1 = { name: "describe", after: [], run: async (context) => {
      emission = await describeComponentStageV1(repoRoot, target, deliverable, context.cache, { compiler: context.control, environment: context.environment, cancelled: () => context.signal.aborted });
      const drift = settleComponentDescriptorV1(repoRoot, target, deliverable, emission, describeMode);
      if (drift.length && describeMode === "write") context.log(`[describe] ${target.pluginId}: rewrote the committed pair (${drift.length} path(s))`);
    } };
    if (target.directoryName === undefined) return [describe, publish, discard];
    return [
      describe,
      { name: "transpile", after: [], run: async (context) => { transpiled = await transpileComponentModulesV1(workOf(context.control, context.cache, context.generation.root, context.environment)); } },
      { name: "optimize", after: ["transpile"], run: async (context) => { optimized = await optimizeComponentCoresV1(workOf(context.control, context.cache, context.generation.root, context.environment), transpiled!, context.widen(availableParallelism())); } },
      { name: "probe", after: ["optimize"], run: async (context) => {
        module = assembleComponentModuleV1(workOf(context.control, context.cache, context.generation.root, context.environment), transpiled!, optimized);
        try { semantic = await probeComponentModuleV1(work!, module); }
        catch (error) { rmSync(module, { recursive: true, force: true }); module = undefined; throw error; }
      } },
      publish,
      discard,
    ];
  };
}

/** 🎯️ Builds exactly `pluginIds` — the closure one dev launch serves — through the one build driver over the warm stores: only the passes and packages of
 * those components, each streamed through its cached stages into the one plugin-modules root. A launch never pays for components it does not load.
 * @see ../../../🧑‍💻dev/♻️activation/🧰️preparation/🟦️.ts */
export function buildComponentClosureV1(repoRoot: string, pluginIds: ReadonlySet<string>, options: Readonly<{ control: CompilerAdmissionControl; log?: (line: string) => void }>): Promise<NativeBuildOutcomeV1> {
  return runNativeBuildDriverV1({ compilerConfiguration: join(repoRoot,".cargo/config.toml"), plan: componentPassPlanV1(repoRoot, pluginIds), cwd: repoRoot, cacheRoot: repoCacheDirectory(repoRoot), shared: cargoDirectories(repoRoot), generation: "warm", leaseDirectory: join(repoRoot, ".cache/framework/leases"), environment: process.env, control: options.control, stages: componentDriverStagesV1(repoRoot), ...(options.log ? { log: options.log } : {}) });
}

/** 🧭️ The components one playground variant loads: `boot` is the host closure a launch must serve before anything else (its app scope only), `full` is
 * every component the variant's session lists. */
export function playgroundComponentClosureV1(variant: string, scope: "boot" | "full"): ReadonlySet<string> {
  const row = PLAYGROUND_BUILD_TARGETS.find((target) => target.variant === variant);
  if (!row) throw new Error(`Unknown playground variant: ${variant}`);
  return new Set(runtimeComponentClosure([...PLUGIN_BUILD_TARGETS, ...EXTENSION_TARGETS], [{ id: row.pluginId, appScoped: scope === "boot" || row.app !== undefined }]) as readonly string[]);
}

/** ✍️ `components describe <plugin-id…>|--all [--profile wasm-release|wasm-prototype]`: writes the committed semantic pairs from freshly built bytes. Shipped
 * bytes come from one build driver run over exactly the selected components, each described (writing a differing pair), receipted and materialized as it
 * links; iterate bytes are built and described one component at a time. */
export async function describeComponentsV1(repoRoot: string, segments: readonly string[], control: CompilerAdmissionControl): Promise<void> {
  const { all, pluginIds, profile } = componentsDescribeArgumentsV1(segments);
  if (profile !== SHIPPED_COMPONENT_PROFILE) return describeIterateComponentsV1(repoRoot, segments, control);
  const outcome = await runNativeBuildDriverV1({ compilerConfiguration: join(repoRoot,".cargo/config.toml"), plan: componentPassPlanV1(repoRoot, all ? undefined : new Set(pluginIds), "release"), cwd: repoRoot, cacheRoot: repoCacheDirectory(repoRoot), shared: cargoDirectories(repoRoot), generation: "warm", leaseDirectory: join(repoRoot, ".cache/framework/leases"), environment: buildModeEnvironment("release"), control, stages: componentDriverStagesV1(repoRoot, "write") });
  console.log(`components describe: ${outcome.staged.length} component(s) described from ${profile} bytes; report ${outcome.report}`);
}
