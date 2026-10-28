//#region 🧲️Header

// 2026 Ueli Saluz <ueli@semio-tech.com>

//#endregion 🧲️Header

//#region 🔌️Adapters
import { readFileSync } from "node:fs";
import { createRequire } from "node:module";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
//#endregion 🔌️Adapters

//#region 🧭️Oracle

/**
 * 🧭️ Independent TypeScript derivation of the build surface from the neutral catalogue and the authored policy, validated by Ajv.
 * It shares no code with the Go implementation: both read the same two authored files, and the golden vectors decide.
 */
const modules = join(import.meta.dirname, "../../..");
const read = (...segments: string[]): any => JSON.parse(readFileSync(join(modules, ...segments), "utf8"));
const backendIds: string[] = read("📚️library", "🏗️build", "🔌️backends", "📋️registry", "🔣️.json").backends.map((row: any) => row.id);
const catalogue = JSON.parse(readFileSync(join(modules, "📚️library", "🏗️build", "⚙️operations", "🔣️catalog.json"), "utf8").replaceAll("{backends}", backendIds.join("|")));
const policy = read("🔌️mcp", "🖼️assets", "🏗️build-surface.json");
const golden = read("🔌️mcp", "🧪️tests", "🧫️fixtures", "🏗️build-surface.json");
const AjvConstructor = createRequire(import.meta.url)("ajv") as new (options: Record<string, unknown>) => { compile(schema: unknown): ((data: unknown) => boolean) & { errors?: unknown } };

const closedValues = (valueName?: string): string[] | undefined => {
  if (!valueName || !valueName.includes("|")) return undefined;
  const values = valueName.split("|");
  return values.some((value) => value === "" || /[\[\]<>=, ]/.test(value)) ? undefined : values;
};
const property = (flag: string): string => flag.replace(/^--/, "").replaceAll("-", "_");
const both = (text: { en: string; de: string }): string => `EN: ${text.en}\nDE: ${text.de}`;

function toolName(spec: any): string {
  return spec.name ?? [spec.operation, spec.subcommand].filter(Boolean).join("_").replaceAll("-", "_");
}

function flagsOf(spec: any): string[] {
  const operation = catalogue.operations.find((row: any) => row.name === spec.operation);
  if (spec.flags) return spec.flags;
  if (spec.subcommand) {
    const sub = operation.subcommands.find((row: any) => row.name === spec.subcommand);
    if (sub.flags) return sub.flags;
  }
  return operation.flags;
}

function inputSchema(spec: any): Record<string, unknown> {
  const properties: Record<string, unknown> = {};
  const required: string[] = [];
  for (const positional of spec.positional ?? []) {
    if (positional.argv) properties[positional.name] = { type: "array", items: { type: "string", minLength: 1 }, minItems: 1, maxItems: policy.limits.maxArguments };
    else if (positional.enum) properties[positional.name] = { type: "string", enum: positional.enum };
    else properties[positional.name] = { type: "string", minLength: 1, ...(positional.pattern ? { pattern: positional.pattern } : {}) };
    if (!positional.optional) required.push(positional.name);
  }
  const bound = Object.keys(spec.bind ?? {});
  const omitted = policy.omittedFlags.map((row: any) => row.flag);
  for (const flag of flagsOf(spec).filter((name) => !bound.includes(name) && !omitted.includes(name))) {
    const row = catalogue.flags.find((candidate: any) => candidate.name === flag && (candidate.operations ?? []).includes(spec.operation)) ?? catalogue.flags.find((candidate: any) => candidate.name === flag);
    const override = policy.flagTypes[flag];
    let schema: Record<string, unknown>;
    if (row.value === "none") schema = { type: "boolean" };
    else if (override?.type === "integer") schema = { type: "integer", minimum: override.minimum, maximum: override.maximum };
    else if (override?.type === "list") schema = { type: "array", minItems: 1, uniqueItems: true, items: { type: "string", minLength: 1, ...(override.closed ? { enum: closedValues(row.valueName) } : override.items ? { enum: override.items } : {}) } };
    else if (closedValues(row.valueName)) schema = { type: "string", enum: closedValues(row.valueName) };
    else schema = { type: "string", minLength: 1 };
    properties[property(flag)] = schema;
    if ((spec.required ?? []).includes(flag)) required.push(property(flag));
  }
  for (const name of spec.environment ?? []) properties[name] = { type: "string", enum: policy.environment[name].enum };
  for (const global of policy.globals) properties[global.input] = { type: "string", enum: closedValues(catalogue.flags.find((row: any) => row.name === global.flag).valueName) };
  properties[policy.identity.argument] = { type: "string", minLength: 1, pattern: policy.identity.pattern };
  return { type: "object", properties, required, additionalProperties: false, ...(spec.anyOf ? { anyOf: spec.anyOf.map((group: string[]) => ({ required: group.map(property) })) } : {}) };
}

/** ⚖️ Every catalogue row is exposed or excluded with a reason in both languages. */
function uncovered(): string[] {
  const accounted = new Set<string>([...policy.tools, ...policy.excluded].map((row: any) => `${row.operation} ${row.subcommand ?? ""}`));
  const missing: string[] = [];
  for (const operation of catalogue.operations) {
    for (const sub of operation.subcommands?.length ? operation.subcommands : [{ name: "" }]) {
      if (!accounted.has(`${operation.name} ${sub.name}`) && !accounted.has(`${operation.name} `)) missing.push(`${operation.name} ${sub.name}`.trim());
    }
  }
  return missing;
}

//#endregion 🧭️Oracle

//#region 🧪️Cases

describe("build surface oracle", () => {
  it("derives exactly the golden tool names", () => {
    expect(policy.tools.map(toolName).sort()).toEqual(golden.tools);
  });

  it("declares a lease priority per entry point: fan-out tools are background, every valid golden vector carries exactly the declared class, nothing is interactive", () => {
    const declared = new Map<string, string>(policy.tools.filter((row: any) => row.priority !== undefined).map((row: any) => [toolName(row), row.priority]));
    expect(policy.priority.variable).toBe("SEMIO_LEASE_PRIORITY");
    expect([...declared.keys()].sort()).toEqual(["affected", "bench_run", "concurrency_run", "conformance_run", "incremental_run", "run_many", "test", "units_run"]);
    for (const value of declared.values()) expect(value).toBe("background");
    const vectors = golden.vectors.filter((vector: any) => vector.environment !== null && vector.environment !== undefined);
    expect(vectors.length).toBeGreaterThan(10);
    for (const vector of vectors) expect([vector.id, vector.environment[policy.priority.variable]]).toEqual([vector.id, declared.get(vector.tool)]);
    expect(declared.size).toBeLessThan(policy.tools.length);
  });

  it("leaves no catalogue row neither exposed nor excluded", () => {
    expect(uncovered()).toEqual([]);
    for (const row of policy.excluded) {
      expect(row.reason.en.length).toBeGreaterThan(0);
      expect(row.reason.de.length).toBeGreaterThan(0);
    }
  });

  it("states every authored text in both languages", () => {
    const texts: { en?: string; de?: string }[] = [policy.identity.description, ...Object.values<any>(policy.environment).map((row) => row.description), ...Object.values<any>(policy.kinds).map((row) => row.note), ...policy.tools.flatMap((spec: any) => [...(spec.summary ? [spec.summary] : []), ...(spec.positional ?? []).map((positional: any) => positional.description)])];
    for (const text of texts) {
      expect(text.en && text.de).toBeTruthy();
    }
    expect(both({ en: "a", de: "b" })).toBe("EN: a\nDE: b");
  });

  it("accepts and refuses exactly the golden argument vectors under a JSON Schema validator", () => {
    const ajv = new AjvConstructor({ strict: false });
    const validators = new Map<string, ((data: unknown) => boolean) & { errors?: unknown }>(policy.tools.map((spec: any) => [toolName(spec), ajv.compile(inputSchema(spec))]));
    for (const vector of golden.vectors) {
      const validate = validators.get(vector.tool)!;
      expect(validate(vector.arguments), `${vector.id}: ${JSON.stringify(validate.errors)}`).toBe(vector.valid);
    }
  });

  it("binds the sealed graph to every graph query by default", () => {
    for (const name of ["show_projects", "show_project", "show_targets", "graph"]) {
      const spec = policy.tools.find((row: any) => toolName(row) === name);
      expect(spec.defaults["--graph-policy"]).toBe("current");
      expect(spec.bind["--json"]).toBe(true);
    }
  });
});

//#endregion 🧪️Cases
