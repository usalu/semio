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
const hasHead = (name, head) => name === head || name.startsWith(`${head}-`);
const hasTail = (name, tail) => name === tail || name.endsWith(`-${tail}`);
const hasToken = (name, token) => hasHead(name, token) || hasTail(name, token) || name.includes(`-${token}-`);

/** 🔤️ Strips the leading symbols (an emoji prefix) of a target name, so every name rule reads `🏭️generator-build` exactly as `generator-build`. */
export const normalizedTargetName = (name) => name.replace(/^[^A-Za-z0-9]+/u, "");

/** 🗂️ Resolves the cache class the workspace policy declares for one target name: a row that names the name exactly decides first, then the first row with a matching head, tail or token; a name no row classifies has none and never replays.
 * @see ../../../../⚡️caching/🧬️schema/🔣️.json */
export function targetClassV1(name, rows = POLICY.targetClasses) {
  const row = targetRowV1(normalizedTargetName(name), rows);
  return row && cacheDecisionV1(row.decision.class, row.decision.reason);
}

const targetRowV1 = (bare, rows) => rows.find((row) => row.exact?.includes(bare)) ?? rows.find((row) => row.heads?.some((head) => hasHead(bare, head)) || row.tails?.some((tail) => hasTail(bare, tail)) || row.tokens?.some((token) => hasToken(bare, token)));

/** 🧪️ Names the input whose negations the row of a target appends to its inputs, so the target reads production sources only; the row is found exactly as its cache class is.
 * @see ../../../../⚡️caching/🧬️schema/🔣️.json */
export const targetExclusionsV1 = (name, rows = POLICY.targetClasses) => targetRowV1(normalizedTargetName(name), rows)?.exclusions;

/** 🛡️ Whether one explicit input of a target (a file or glob it names itself, neither a named input nor a dependency group nor a negation) selects a path the exclusions remove: such a target reads that path on purpose, so the exclusions would hide a real read from its key.
 * @see ../../../../🏗️build/🔣️policy.json */
export function excludedInputConflictV1(inputs, namedInputs, patterns, matches, projectRoot) {
  const root = projectRoot === "." ? "" : projectRoot + "/";
  return inputs.some((input) => typeof input === "string" && !input.startsWith("^") && !input.startsWith("!") && !Object.hasOwn(namedInputs, input) && patterns.some((pattern) => matches(input.replaceAll("{projectRoot}/", root).replaceAll("{workspaceRoot}/", ""), pattern)));
}

export const cacheableFamily = (name, rows) => targetClassV1(name, rows)?.class === "deterministic";

/** 📌️ Admits the cache decision a target declares itself in `metadata.semio.cache`; a malformed or open record is refused instead of ignored. */
function declaredCacheDecisionV1(value) {
  if (value === undefined) return undefined;
  const decision = cacheDecisionV1(value?.class, value?.reason);
  if (Object.keys(value).length !== Object.keys(decision).length || value.version !== decision.version) throw Error(`Repository target cache decision refused: ${value?.class}/${value?.reason}`);
  return decision;
}

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

/** 🛡️ Side effects and live processes cannot be replayed as completed task results, and neither can a target nothing classifies: a target replays only when the owner declared it deterministic, the policy classes its name as deterministic or its command verifies. */
export function targetPolicy(name, target, policy = POLICY) {
  const bare = normalizedTargetName(name);
  if (matchesCommand(bare, policy.continuous)) return withCacheDecision({ ...target, cache: false, continuous: true }, "continuous", "serves-until-stopped");
  if (target.cache === false) { declaredCacheDecisionV1(target.metadata?.semio?.cache); return { inputs: ["default"], outputs: [], ...target, cache: false }; }
  if (matchesUncached(bare, policy)) return withCacheDecision({ ...target, cache: false }, ...uncachedDecisionV1(bare));
  const exact = policy.cachedExact.includes(bare);
  if (!exact && (mutatingName(bare) || liveName(bare))) return withCacheDecision({ ...target, cache: false }, ...uncachedDecisionV1(bare));
  const decision = declaredCacheDecisionV1(target.metadata?.semio?.cache) ?? targetClassV1(bare, policy.targetClasses) ?? (exact || verifyCommand(target) ? cacheDecisionV1("deterministic", "check-without-outputs") : undefined);
  if (!decision) return withCacheDecision({ ...target, cache: false }, "unclassified", "no-cache-classification");
  if (decision.class !== "deterministic") return withCacheDecision({ ...target, cache: false }, decision.class, decision.reason);
  return { inputs: ["default", "^default"], outputs: [], ...withCacheDecision(target, decision.class, decision.reason), cache: true };
}

/** 🚧️ A replayed producer must declare what it writes: a target classed `pure-function-of-inputs` that declares no output (and is neither a Rust build intent, a host-platform output declaration nor a pure ordering task) would replay as an empty success on a clean checkout, so it stops being replayable instead. */
export function cachedOutputsAdmittedV1(target) {
  const decision = target.metadata?.semio?.cache;
  if (target.cache !== true || decision?.class !== "deterministic" || decision.reason !== "pure-function-of-inputs") return target;
  const semio = target.metadata.semio;
  if ((Array.isArray(target.outputs) && target.outputs.length > 0) || semio.rustBuildIntent !== undefined || semio.rustUnitAggregate !== undefined || semio.hostPlatformOutputsV1 !== undefined || target.executor === "repo:noop") return target;
  return withCacheDecision({ ...target, cache: false }, "side-effecting", "mutates-workspace");
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

/** 🛠️ Exposes the pinned wasm toolchain (wasm-pack, wasm-bindgen and the immutable Binaryen optimizer preparation) as a target dependency before compiler execution. */
export function withWasmTooling(targets, required) {
  if (!targets.wasm || !required) return targets;
  return { ...targets, wasm: { ...targets.wasm, options: { ...targets.wasm.options, env: { ...targets.wasm.options?.env, SEMIO_WASM_BUILD_REQUIRED: "1" } }, dependsOn: [...new Set([...(targets.wasm.dependsOn ?? []), "workspace:deps-wasm"])] } };
}
