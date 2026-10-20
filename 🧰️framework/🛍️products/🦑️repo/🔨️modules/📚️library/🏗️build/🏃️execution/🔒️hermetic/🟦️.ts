import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from "../🧬️schema/🔒️hermetic-classes.json" with {type: "json"};
import table from "./🔣️.json" with {type: "json"};

if (validateJsonSchemaSubset(schema, table, schema).length) throw new Error("Closed hermetic class table required");

/** 🔒️ One hermetic target class as the table declares it. */
export type HermeticClassV1 = (typeof table.classes)[number];
/** 🧾️ Why a target stays out of the hermetic classes although it is cacheable. */
export type HermeticExceptionReasonV1 = (typeof schema.definitions.Reason.enum)[number];
/** 🎯️ What the class table says of one target: a member of a class, a declared exception or no class. */
export type HermeticSelectionV1 = {kind: "class"; class: HermeticClassV1} | {kind: "excepted"; reason: HermeticExceptionReasonV1} | {kind: "unselected"};
/** 🔬️ What a target contributes to the class selection. */
export type HermeticFactsV1 = {readonly project: string; readonly target: string; readonly root: string};
/** 🧮️ The declared inputs a hermetic action takes from the host besides files: environment values read once when the workspace is generated and third-party packages named by `externalDependencies`. */
export type HermeticInputsV1 = {readonly env: Readonly<Record<string, string>>; readonly packages: readonly string[]};

/** 🧾️ The authored table: classes in selection order, the shared runtime, the closed environment and the strategies per host. */
export const HERMETIC_TABLE_V1 = table;

const expressions = new Map<string, RegExp>();
/** 🧷️ Compiles one class glob: `*` stays inside a path segment, `**` crosses segments, everything else is literal. */
export function hermeticGlobV1(pattern: string): RegExp {
  let known = expressions.get(pattern);
  if (!known) {
    let source = "";
    for (let index = 0; index < pattern.length; index++) {
      const character = pattern[index]!;
      if (character !== "*") source += character.replace(/[.+?^${}()|[\]\\]/gu, "\\$&");
      else if (pattern[index + 1] === "*") {source += ".*"; index++;} else source += "[^/]*";
    }
    expressions.set(pattern, known = new RegExp("^" + source + "$", "u"));
  }
  return known;
}
const matches = (patterns: readonly string[], text: string): boolean => patterns.some(pattern => hermeticGlobV1(pattern).test(text));

/** 🎯️ Selects the first class whose target, project and root rules all hold; a declared exception for the exact target wins over every class. */
export function hermeticSelectV1(facts: HermeticFactsV1, source: {readonly classes: readonly HermeticClassV1[]; readonly exceptions: readonly {readonly project: string; readonly target: string; readonly reason: string}[]} = table): HermeticSelectionV1 {
  const row = source.classes.find(row => matches(row.select.targets, facts.target) && matches(row.select.projects, facts.project) && !matches(row.select.excludeRoots, facts.root));
  if (!row) return {kind: "unselected"};
  const exception = source.exceptions.find(entry => entry.project === facts.project && entry.target === facts.target);
  return exception ? {kind: "excepted", reason: exception.reason as HermeticExceptionReasonV1} : {kind: "class", class: row};
}

/** 🧮️ Reads the environment and package inputs of a target: a named environment value enters as declared, a runtime command the toolchain and platform already determine is dropped, `externalDependencies` names packages; any other host input refuses, as does an environment name the table does not admit. */
export function hermeticInputsV1(inputs: readonly unknown[], environment: Readonly<Record<string, string | undefined>>): HermeticInputsV1 | {readonly refused: "host-inputs"} {
  const env: Record<string, string> = {}, packages = new Set<string>(), admitted = new Set(table.environment.names), covered = new Set(table.runtime.covered);
  for (const input of inputs) {
    if (typeof input !== "object" || input === null) continue;
    const row = input as Record<string, unknown>;
    if (typeof row.env === "string") {
      if (!admitted.has(row.env) || table.environment.denied.some(prefix => (row.env as string).startsWith(prefix))) return {refused: "host-inputs"};
      const value = environment[row.env];
      if (value !== undefined) env[row.env] = value;
    } else if (typeof row.runtime === "string") {
      if (!covered.has(row.runtime)) return {refused: "host-inputs"};
    } else if (Array.isArray(row.externalDependencies)) for (const name of row.externalDependencies) if (typeof name === "string") packages.add(name);
  }
  return {env: Object.fromEntries(Object.entries(env).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0)), packages: [...packages].sort()};
}

/** 🛤️ Names the Bazel spawn strategies one host lists for hermetic actions: `isolated-root` is plain local execution, safe because the runner builds its own root from the declared inputs. */
export function hermeticStrategiesV1(platform: string): readonly string[] {
  const row = (table.strategies as Record<string, readonly string[]>)[platform];
  if (!row) throw new Error(`Hermetic strategies required for platform: ${platform}`);
  return row.map(name => name === "isolated-root" ? "local" : name);
}
