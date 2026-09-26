/** 💬️ TypeScript twin of the Rust description law (`🗂️catalog/🦀️.rs` `🔖️DescriptionLaw`): the
 * manifest's `CapabilityDescription` contract (`🛂️manifest/🧬️schema/🔣️.json`) validated by AJV — the
 * third-party oracle for the repo's owned validator — plus the three cross-field rules the schema
 * states in prose. It reads the committed descriptors and the fixture, never the Rust implementation.
 *
 * - {@link capabilityDescriptionProblems} — one app's verbs → `(verb, problem)` in contract order;
 * - {@link proveCapabilityDescriptionFixture} — replays `🧫️fixtures/💬️capability-description.json`;
 * - {@link capabilityDescriptionCensus} — every agent-published plugin verb of every committed
 *   descriptor, the set `capability-audit-check` holds against the Rust census. */
import Ajv from "ajv";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";

const MANIFEST_SCHEMA = "🧰️framework/🔨️modules/🛂️manifest/🧬️schema/🔣️.json";
const FIXTURE = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗂️catalog/🧫️fixtures/💬️capability-description.json";
const REGISTRY = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json";
const FRAMEWORK_VIEW_SHELL_ACTION_IDS = new Set(["setActiveUtility", "setActiveTool", "startIntroduction", "setHistoryCommandFilter", "noteShellCommand"]);

export type DescriptionProblem = "missing" | "schema" | "untranslated" | "repeatsTitle" | "sharedWithinApp";
export type DescribedVerb = { id: string; title: { en: string; de: string }; description: unknown };
export type DescriptionFinding = { capabilityId: string; problem: DescriptionProblem };

type Label = Record<string, Record<string, string>>;
type Declaration = { id: string; label: Label; kind: string; inPalette?: boolean; semantics?: { audience?: string; description?: Label } };
type App = { id: string; windowKinds?: { actions?: Declaration[] }[]; actions?: Declaration[]; commands?: Declaration[]; modes?: { id: string; commands?: Declaration[] }[] };
type Descriptor = { manifest: { pluginId: string; apps?: App[]; commands?: Declaration[] } };

const read = (repoRoot: string, path: string): unknown => JSON.parse(readFileSync(resolve(repoRoot, path), "utf8"));

const validators = new Map<string, ReturnType<Ajv["compile"]>>();

/** 📐️ AJV over the manifest schema, compiled once per `(repo, def)`. */
const descriptionValidator = (repoRoot: string, def: string): ReturnType<Ajv["compile"]> => {
  const key = `${repoRoot}#${def}`;
  const cached = validators.get(key);
  if (cached) return cached;
  const schema = read(repoRoot, MANIFEST_SCHEMA) as { $id: string };
  const validate = new Ajv({ strict: true, allErrors: true }).addSchema(schema).compile({ $ref: `${schema.$id}#/$defs/${def}` });
  validators.set(key, validate);
  return validate;
};

const cell = (description: unknown, terminology: string, locale: string): string => {
  const row = (description as Record<string, unknown> | null)?.[terminology] as Record<string, unknown> | undefined;
  const text = row?.[locale];
  return typeof text === "string" ? text.trim() : "";
};

const repeatsTitle = (text: string, title: string): boolean => text !== "" && text.replace(/[.!?]+$/u, "").trim().toLowerCase() === title.trim().toLowerCase();

/** ⚖️ The contract over one app's agent-published verbs — the Rust `description_problems`, re-stated. */
export function capabilityDescriptionProblems(repoRoot: string, verbs: DescribedVerb[]): { verb: string; problem: DescriptionProblem }[] {
  const validate = descriptionValidator(repoRoot, "CapabilityDescription");
  const english = new Map<string, number>();
  for (const verb of verbs) {
    const text = verb.description === null || verb.description === undefined ? "" : cell(verb.description, "native", "en");
    if (text !== "") english.set(text, (english.get(text) ?? 0) + 1);
  }
  const problems: { verb: string; problem: DescriptionProblem }[] = [];
  for (const verb of verbs) {
    const description = verb.description;
    if (description === null || description === undefined) {
      problems.push({ verb: verb.id, problem: "missing" });
      continue;
    }
    if (!validate(description)) problems.push({ verb: verb.id, problem: "schema" });
    if (["native", "reuse"].some((terminology) => cell(description, terminology, "en") !== "" && cell(description, terminology, "en") === cell(description, terminology, "de"))) problems.push({ verb: verb.id, problem: "untranslated" });
    if (repeatsTitle(cell(description, "native", "en"), verb.title.en) || repeatsTitle(cell(description, "native", "de"), verb.title.de)) problems.push({ verb: verb.id, problem: "repeatsTitle" });
    if ((english.get(cell(description, "native", "en")) ?? 0) > 1) problems.push({ verb: verb.id, problem: "sharedWithinApp" });
  }
  return problems;
}

/** 🧪️ Replays the language-agnostic law cases; returns the number of rows proven. */
export function proveCapabilityDescriptionFixture(repoRoot: string): number {
  const rows = read(repoRoot, FIXTURE) as { name: string; verbs: DescribedVerb[]; problems: { verb: string; problem: DescriptionProblem }[] }[];
  const validateFixture = descriptionValidator(repoRoot, "CapabilityDescriptionFixture");
  if (!validateFixture(rows)) throw new Error(`capability-description fixture violates CapabilityDescriptionFixture: ${JSON.stringify(validateFixture.errors)}`);
  for (const row of rows) {
    const actual = JSON.stringify(capabilityDescriptionProblems(repoRoot, row.verbs));
    if (actual !== JSON.stringify(row.problems)) throw new Error(`capability-description fixture "${row.name}": expected ${JSON.stringify(row.problems)}, got ${actual}`);
  }
  return rows.length;
}

const audience = (declaration: Declaration): string => declaration.semantics?.audience ?? (declaration.kind === "interaction" ? "input" : declaration.kind === "view" && !declaration.inPalette ? "chrome" : "agent");

const frameworkInjected = (action: Declaration): boolean => ["history", "clipboard", "interaction"].includes(action.kind) || FRAMEWORK_VIEW_SHELL_ACTION_IDS.has(action.id);

const described = (id: string, declaration: Declaration): DescribedVerb => ({ id, title: { en: declaration.label.native?.en ?? "", de: declaration.label.native?.de ?? "" }, description: declaration.semantics?.description ?? null });

/** 🧾️ Every agent-published plugin verb of every committed descriptor — window-kind actions first, then
 * app-scope actions (first declaration of an id wins), app and mode commands, plugin commands — judged
 * per app. Framework-injected verbs are the gateway's own definitions, not descriptor data, and stay
 * with the Rust census. Sorted by capability id, then problem. */
export function capabilityDescriptionCensus(repoRoot: string): DescriptionFinding[] {
  const findings: DescriptionFinding[] = [];
  const judge = (prefix: string, verbs: DescribedVerb[]) => {
    for (const { verb, problem } of capabilityDescriptionProblems(repoRoot, verbs)) findings.push({ capabilityId: `${prefix}${verb}`, problem });
  };
  for (const entry of read(repoRoot, REGISTRY) as { cratePath: string }[]) {
    const descriptor = read(repoRoot, resolve(repoRoot, dirname(dirname(entry.cratePath)), "🔣️.json")) as Descriptor;
    const pluginId = descriptor.manifest.pluginId;
    for (const app of descriptor.manifest.apps ?? []) {
      const seen = new Set<string>();
      const actions: Declaration[] = [];
      for (const action of [...(app.windowKinds ?? []).flatMap((windowKind) => windowKind.actions ?? []), ...(app.actions ?? [])]) {
        if (seen.has(action.id)) continue;
        seen.add(action.id);
        actions.push(action);
      }
      const verbs = actions.filter((action) => !frameworkInjected(action) && audience(action) === "agent").map((action) => described(action.id, action));
      verbs.push(...(app.commands ?? []).filter((command) => audience(command) === "agent").map((command) => described(`cmd.${command.id}`, command)));
      for (const mode of app.modes ?? []) verbs.push(...(mode.commands ?? []).filter((command) => audience(command) === "agent").map((command) => described(`mode.${mode.id}.${command.id}`, command)));
      judge(`${pluginId}.${app.id}.`, verbs);
    }
    judge(`${pluginId}.cmd.`, (descriptor.manifest.commands ?? []).filter((command) => audience(command) === "agent").map((command) => described(command.id, command)));
  }
  const order: DescriptionProblem[] = ["missing", "schema", "untranslated", "repeatsTitle", "sharedWithinApp"];
  return findings.sort((left, right) => (left.capabilityId < right.capabilityId ? -1 : left.capabilityId > right.capabilityId ? 1 : order.indexOf(left.problem) - order.indexOf(right.problem)));
}
