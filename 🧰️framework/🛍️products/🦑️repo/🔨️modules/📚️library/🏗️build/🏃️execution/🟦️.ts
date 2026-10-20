import coverageSchema from "./🧬️schema/🎯️coverage.json" with { type: "json" };

const definitions = coverageSchema.definitions;

/** 🧬️ The tool family that performs the work of a task. */
export type TaskFamilyV1 = (typeof definitions.Family.enum)[number];

/** 🚦️ What the neutral contract assigns a task: replay an action, replay a test result, run until stopped, always execute, or refuse to cache. */
export type TaskDispositionV1 = (typeof definitions.Disposition.enum)[number];

/** 🧾️ The uncached reasons a build tool records. */
export type TaskReasonV1 = (typeof definitions.Reason.enum)[number];

/** 📋️ One ordered family rule as the schema declares it. */
export interface TaskFamilyRuleV1 { id: string; family: TaskFamilyV1; noop?: true; target?: string; root?: string; command?: string; script?: string }

/** 🔬️ What the classifier reads of one task. */
export interface TaskCoverageInputV1 { project?: string; target: string; root: string; kind: "command" | "noop"; command?: string; script?: string; outputs?: readonly string[]; cache: { cache: true } | { cache: false; reason: TaskReasonV1 } }

/** 🎯️ The family, disposition and, for a refused or uncached task, the reason. */
export interface TaskCoverageV1 { family: TaskFamilyV1; disposition: TaskDispositionV1; reason?: TaskReasonV1 }

/** 📊️ Task counts by disposition and by family and disposition. */
export interface TaskCoverageReportV1 { version: 1; tasks: number; dispositions: Record<string, number>; families: Record<string, Record<string, number>> }

const rules = definitions.Rules.const as readonly TaskFamilyRuleV1[];
const policy = definitions.Policy.const as { version: 1; test: { target: string }; interactive: readonly string[]; refused: readonly string[] };
const compiled = new Map<string, RegExp>();
const search = (source: string, text: string): boolean => {
  let expression = compiled.get(source);
  if (!expression) compiled.set(source, expression = new RegExp(source, "u"));
  return expression.test(text);
};

/** 🧩️ Names the first rule whose every stated matcher holds; a matcher on an absent `command` or `script` never holds. */
export function taskFamilyV1(input: TaskCoverageInputV1): TaskFamilyV1 {
  const rule = rules.find(row => (row.noop === undefined || input.kind === "noop") && (row.target === undefined || search(row.target, input.target)) && (row.root === undefined || search(row.root, input.root)) && (row.command === undefined || input.command !== undefined && search(row.command, input.command)) && (row.script === undefined || input.script !== undefined && search(row.script, input.script)));
  if (!rule) throw new Error(`Task family rule required: ${input.target}`);
  return rule.family;
}

/** 🧭️ Assigns the disposition: a pure ordering task and a cached task replay, a cached task named `test` that declares no outputs replays as a test result, a continuous task is interactive, an undeclared cache is refused and every other uncached task always executes. */
export function taskDispositionV1(input: TaskCoverageInputV1): { disposition: TaskDispositionV1; reason?: TaskReasonV1 } {
  if (input.kind === "noop") return { disposition: "action" };
  if (input.cache.cache) return { disposition: search(policy.test.target, input.target) && Array.isArray(input.outputs) && input.outputs.length === 0 ? "test" : "action" };
  const reason = input.cache.reason;
  return { disposition: policy.interactive.includes(reason) ? "interactive" : policy.refused.includes(reason) ? "refused" : "uncached", reason };
}

/** 🏷️ Classifies one task by tool family and disposition. */
export function taskCoverageV1(input: TaskCoverageInputV1): TaskCoverageV1 {
  return { family: taskFamilyV1(input), ...taskDispositionV1(input) };
}

/** 🧮️ Counts classified tasks by disposition and by family and disposition, in a stable key order. */
export function taskCoverageReportV1(rows: Iterable<TaskCoverageV1>): TaskCoverageReportV1 {
  const dispositions: Record<string, number> = {}, families: Record<string, Record<string, number>> = {};
  let tasks = 0;
  for (const row of rows) {
    tasks++;
    dispositions[row.disposition] = (dispositions[row.disposition] ?? 0) + 1;
    const family = families[row.family] ??= {};
    family[row.disposition] = (family[row.disposition] ?? 0) + 1;
  }
  const sorted = <T>(value: Record<string, T>): Record<string, T> => Object.fromEntries(Object.entries(value).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0));
  return { version: 1, tasks, dispositions: sorted(dispositions), families: Object.fromEntries(Object.entries(families).sort(([left], [right]) => left < right ? -1 : left > right ? 1 : 0).map(([name, counts]) => [name, sorted(counts)])) };
}
