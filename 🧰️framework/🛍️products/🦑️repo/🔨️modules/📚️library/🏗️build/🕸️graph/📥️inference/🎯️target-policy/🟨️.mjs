/** 🎯️ Admits target execution contracts and decides which generated targets are cached, continuous or side-effecting. */
import { conformsOwnerContractV1 } from "../../../📤️arguments/🟨️.mjs";

/** 🏷️ Names this component as its roster entry does. */
export const component = "target-policy";
let POLICY, targetExecutionSchema, targetCacheSchema, DEFAULT_EXECUTOR;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ POLICY, targetExecutionSchema, targetCacheSchema, DEFAULT_EXECUTOR } = registry.coordinates);
}

const matchesCommand = (name, commands) => commands.some((command) => name === command || name.startsWith(`${command}-`));
export const matchesUncached = (name, policy) => policy.uncachedExact.includes(name) || matchesCommand(name, policy.uncached);
export const mutatingName = (name) => /(?:^|-)(?:clean|gc|prune|setup|fuzz)(?:-|$)/.test(name);
export const liveName = (name) => /(?:^|-)(?:e2e|live)(?:-|$)/.test(name);
export const verifyCommand = (target) => /(?:^|[\s"'])verify(?:[\s"']|$)/.test(target?.options?.command ?? "");
export const cacheableFamily = (name) => /^(test(?:-|$)|check(?:-|$)|lint(?:-|$)|typecheck(?:-|$)|format-check(?:-|$)|generate(?:-|$)|schema(?:-|$)|verify(?:-|$)|stdio(?:-|$)|build(?:-|$)|wasm$|native-build$|package$|extension-package$|cpp-(?:configure|build|test)$|graph-check$|policy-check$|artifact-check$|artifact-package-contract$|toolchain$)/.test(name) || /(?:^|-)contract(?:-|$)/.test(name) || /^generator(?:-|$)/.test(name) && !name.includes("generator-inputs");

const cacheDecisions = new Map();

/** 💾️ Admits one cache decision against the closed schema once and shares the frozen record between every target that carries it.
 * @see ../../../../🏗️build/🏃️execution/🧬️schema/💾️cache.json */
export function cacheDecisionV1(decisionClass, reason) {
  const key = decisionClass + "\0" + reason, known = cacheDecisions.get(key);
  if (known) return known;
  const value = Object.freeze({ version: 1, class: decisionClass, reason }), schema = Object.values(targetCacheSchema.definitions).find(definition => definition.properties.class.const === decisionClass);
  if (!schema || !conformsOwnerContractV1(schema, value)) throw Error(`Repository target cache decision refused: ${decisionClass}/${reason}`);
  cacheDecisions.set(key, value);
  return value;
}

/** 🔖️ Records why a target replays or always executes next to its execution contract. */
export function withCacheDecision(target, decisionClass, reason) {
  return { ...target, metadata: { ...target.metadata, semio: { ...target.metadata?.semio, cache: cacheDecisionV1(decisionClass, reason) } } };
}

/** 🧭️ Names the physical fact behind a workspace-policy refusal to replay. */
export function uncachedDecisionV1(name) {
  if (/(?:^|-)(?:e2e|live)(?:-|$)/.test(name) || name === "os" || name === "semio") return ["live", "drives-running-process"];
  if (/^(?:audit|bench|cache-report|cache-verify|ci-baseline|disk-report|doctor|perf-baseline|test-doctor|test-inventory|parity)(?:-|$)/.test(name)) return ["diagnostic", "measures-environment"];
  if (/^(?:publish|deploy)(?:-|$)/.test(name)) return ["side-effecting", "publishes-remote"];
  return ["side-effecting", "mutates-workspace"];
}

/** 🛡️ Side effects and live processes cannot be replayed as completed task results. */
export function targetPolicy(name, target, policy = POLICY) {
  if (matchesCommand(name, policy.continuous)) return withCacheDecision({ ...target, cache: false, continuous: true }, "continuous", "serves-until-stopped");
  if (target.cache === false) return { inputs: ["default"], outputs: [], ...target, cache: false };
  if (matchesUncached(name, policy)) return withCacheDecision({ ...target, cache: false }, ...uncachedDecisionV1(name));
  if (policy.cachedExact.includes(name)) return { inputs: ["default", "^default"], outputs: [], ...target, cache: true };
  if (mutatingName(name) || liveName(name)) return withCacheDecision({ ...target, cache: false }, ...uncachedDecisionV1(name));
  if (cacheableFamily(name) || verifyCommand(target)) return { inputs: ["default", "^default"], outputs: [], ...target, cache: true };
  return { ...target, cache: target.cache !== false };
}

/** 🏃️ Admits one declared source or native command owner against its closed production contract. */
export function targetExecutionKindV1(target, identity) {
  const value = target.metadata?.semio?.execution;
  if (value === undefined) return undefined;
  const schema = Object.values(targetExecutionSchema.definitions).find(schema => schema.properties.kind.const === value?.kind);
  if (!schema || !conformsOwnerContractV1(schema,value)) throw Error(`Repository target execution contract refused${identity ? `: ${identity}` : ""}`);
  return value.kind;
}

/** 🎯️ Declares the execution service owned by one generated command factory. */
export function executionTargetV1(kind, target, scope) {
  const execution = { version: 1, kind, ...(scope === undefined ? {} : { scope }) }, declared = { ...target, metadata: { ...target.metadata, semio: { ...target.metadata?.semio, ...(kind === "orchestration" ? { orchestrator: true } : {}), execution } } };
  targetExecutionKindV1(declared);
  return declared;
}

/**
 * 🎚️ Applies the workspace target convention: every target runs through
 * {@link DEFAULT_EXECUTOR}, and a project that owns a `📜️script.ts` runs from
 * its own root, so a `📋️project.json` only spells out what actually differs.
 * Projects without one keep the workspace-root default and thereby delegate to
 * the root `📜️script.ts`.
 * @param {Record<string, any>} target
 * @param {string} root
 * @param {boolean} ownsScript
 */
export function targetWithDefaults(target, root, ownsScript) {
  if(target.command!==undefined){const {command,...rest}=target;if(typeof command!=="string"||target.options?.command!==undefined||target.options?.commands!==undefined)throw Error("Build command shorthand requires one command form");target={...rest,options:{...target.options,command}};}
  const executor = target.executor ?? DEFAULT_EXECUTOR;
  if (executor !== DEFAULT_EXECUTOR) return { ...target, executor };
  if (!ownsScript) return { ...target, executor };
  return { ...target, executor, options: { cwd: root, ...(target.options ?? {}) } };
}

/** 🛠️ Exposes wasm-pack's immutable optimizer preparation to Nx before compiler execution. */
export function withWasmTooling(targets, required) {
  if (!targets.wasm || !required) return targets;
  return { ...targets, wasm: { ...targets.wasm, options: { ...targets.wasm.options, env: { ...targets.wasm.options?.env, SEMIO_WASM_BUILD_REQUIRED: "1" } }, dependsOn: [...new Set([...(targets.wasm.dependsOn ?? []), "workspace:deps-wasm-opt"])] } };
}
