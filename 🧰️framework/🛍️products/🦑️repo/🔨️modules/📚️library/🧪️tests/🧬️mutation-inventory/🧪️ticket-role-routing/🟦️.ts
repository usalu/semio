import { expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { relative, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import mapKeys from "lodash/mapKeys.js";
import pick from "lodash/pick.js";

//#region 🧭️Inputs
type Origin = "tracked" | "nonignored-untracked" | "ignored-generator";
type Case = {
  readonly id: string;
  readonly invocation: "mutation-cli" | "direct-api";
  readonly operation: "inventory" | "plan" | "apply" | "verify";
  readonly outputTicket: string | null;
  readonly explicitTicketDir: string | null;
  readonly outputCandidates: readonly { readonly sourcePath: string; readonly independentOrigins: readonly Origin[] }[];
  readonly authoredInputs: readonly string[];
  readonly expected: {
    readonly nTicketDir: string | null;
    readonly outputDestination: string | null;
    readonly assignmentLedgerPath: string | null;
    readonly explicitTicketPaths: readonly string[];
    readonly independentOutputPaths: readonly string[];
    readonly accepted: boolean;
    readonly error: string | null;
  };
};

type Vectors = { readonly schemaVersion: 1; readonly cases: readonly Case[] };
const root = resolve(import.meta.dir, "../../../../../../../../"), schemaPath = resolve(import.meta.dir, "🧬️schema/🔣️.json"), vectorsPath = resolve(import.meta.dir, "🔣️.json"), rootScriptPath = resolve(root, "📜️script.ts"), normalizationPath = resolve(import.meta.dir, "../../../🧹️normalization/🟦️.ts");
const schema = JSON.parse(readFileSync(schemaPath, "utf8")), vectors = JSON.parse(readFileSync(vectorsPath, "utf8")) as Vectors;

/** 🧪️ Projects supplied role facts only; it does not construct a source roster. */
function roleReference(row: Case): Case["expected"] {
  const picked = pick(row, ["invocation", "outputTicket", "explicitTicketDir", "outputCandidates", "authoredInputs"]), renamed = mapKeys(picked, (_value, key) => key === "explicitTicketDir" ? "nTicketDir" : key);
  const { invocation, outputTicket, outputCandidates, authoredInputs, nTicketDir } = renamed;
  if ((invocation !== "mutation-cli" && invocation !== "direct-api") || (outputTicket !== null && typeof outputTicket !== "string") || !Array.isArray(outputCandidates) || !Array.isArray(authoredInputs) || (nTicketDir !== null && typeof nTicketDir !== "string")) throw new Error(`invalid role facts ${row.id}`);
  const independentOutputPaths: string[] = [];
  for (const candidate of outputCandidates) {
    const candidateFacts = pick(candidate, ["sourcePath", "independentOrigins"]);
    const origins: Origin[] = [];
    for (const origin of candidateFacts.independentOrigins) origins.push(origin);
    if (origins.length > 0) independentOutputPaths.push(candidateFacts.sourcePath);
  }
  if (invocation === "mutation-cli") {
    if (outputTicket === null || nTicketDir !== null || authoredInputs.length !== 0) throw new Error(`invalid mutation CLI role case ${row.id}`);
    return { nTicketDir: null, outputDestination: outputTicket, assignmentLedgerPath: `${outputTicket}/📋️mutation-assignments.json`, explicitTicketPaths: [], independentOutputPaths, accepted: true, error: null };
  }
  if (outputTicket !== null || nTicketDir === null) throw new Error(`invalid direct API role case ${row.id}`);
  return { nTicketDir, outputDestination: null, assignmentLedgerPath: null, explicitTicketPaths: [...authoredInputs], independentOutputPaths, accepted: true, error: null };
}

function withoutKey(value: Record<string, unknown>, key: string): Record<string, unknown> {
  const copy = { ...value };
  delete copy[key];
  return copy;
}

function replacingCase(index: number, replacement: Record<string, unknown>): Record<string, unknown> {
  return { ...vectors, cases: vectors.cases.map((row, current) => current === index ? replacement : row) };
}

function expectSchemaError(validate: ReturnType<Ajv2020["compile"]>, value: unknown, instancePath: string, keyword: string, property?: string): void {
  expect(validate(value)).toBe(false);
  expect(validate.errors).toEqual(expect.arrayContaining([expect.objectContaining({ instancePath, keyword, ...(property === undefined ? {} : { params: expect.objectContaining({ missingProperty: property }) }) })]));
}
//#endregion 🧭️Inputs

//#region 🧪️Reference
test("mutation ticket role routing vectors are closed and every field participates", () => {
  const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
  expect(validate(vectors), JSON.stringify(validate.errors)).toBe(true);
  expect(new Set(vectors.cases.map((row) => row.id)).size).toBe(vectors.cases.length);
  for (const field of ["schemaVersion", "cases"]) expectSchemaError(validate, withoutKey(vectors as unknown as Record<string, unknown>, field), "", "required", field);
  for (const [index, row] of vectors.cases.entries()) {
    expect(roleReference(row), row.id).toEqual(row.expected);
    const unknownCase = { ...(row as unknown as Record<string, unknown>), unknown: true };
    expectSchemaError(validate, replacingCase(index, unknownCase), `/cases/${index}`, "additionalProperties");
    for (const field of ["id", "invocation", "operation", "outputTicket", "explicitTicketDir", "outputCandidates", "authoredInputs", "expected"]) expectSchemaError(validate, replacingCase(index, withoutKey(row as unknown as Record<string, unknown>, field)), `/cases/${index}`, "required", field);
    for (const field of ["nTicketDir", "outputDestination", "assignmentLedgerPath", "explicitTicketPaths", "independentOutputPaths", "accepted", "error"]) expectSchemaError(validate, replacingCase(index, { ...row, expected: withoutKey(row.expected as unknown as Record<string, unknown>, field) }), `/cases/${index}/expected`, "required", field);
    for (const candidate of row.outputCandidates) for (const field of ["sourcePath", "independentOrigins"]) expectSchemaError(validate, replacingCase(index, { ...row, outputCandidates: [withoutKey(candidate as unknown as Record<string, unknown>, field)] }), `/cases/${index}/outputCandidates/0`, "required", field);
  }
});
//#endregion 🧪️Reference

//#region 🧪️Subject
test("mutation ticket role routing reaches only the mocked N admission boundary", () => {
  const rootBefore = createHash("sha256").update(readFileSync(rootScriptPath)).digest("hex"), normalizationBefore = createHash("sha256").update(readFileSync(normalizationPath)).digest("hex");
  const marker = "__MUTATION_TICKET_ROLE_ROUTING__";
  const source = [
    'import { mock } from "bun:test";',
    `const normalizerUrl = ${JSON.stringify(pathToFileURL(normalizationPath).href)};`,
    `const rootUrl = ${JSON.stringify(pathToFileURL(rootScriptPath).href)};`,
    `const cases = ${JSON.stringify(vectors.cases)};`,
    'const family = await import(normalizerUrl);',
    'const calls = []; let armed = false;',
    'class Stop extends Error { constructor() { super("ticket-role-routing-stop"); this.name = "TicketRoleRoutingStop"; } }',
    'const sentinel = (options) => { const id = globalThis.__ticketRoleCase; calls.push({ id, options }); throw new Stop(); };',
    'mock.module(normalizerUrl, () => { armed = true; return { ...family, inventoryTaxonomySources: sentinel }; });',
    'const rebound = await import(normalizerUrl);',
    'if (!armed || rebound.inventoryTaxonomySources !== sentinel) throw new Error("N sentinel identity was not armed before S import");',
    'const subject = await import(`${rootUrl}?ticket-role-routing=${Date.now()}`);',
    'for (const row of cases) {',
    '  globalThis.__ticketRoleCase = row.id;',
    '  try {',
    '    if (row.invocation === "mutation-cli") subject.runMutationTaxonomyCli("/virtual/workspace", row.operation, { failOnWarning: false, format: "json", kind: "mutation", ...(row.operation === "plan" ? { baseline: "a".repeat(40) } : {}) }, `/virtual/workspace/${row.outputTicket}`);',
    '    else subject.mutationTaxonomySourceIndex("/virtual/workspace", { explicitTicketDir: `/virtual/workspace/${row.explicitTicketDir}` });',
    '    throw new Error(`sentinel was not reached for ${row.id}`);',
    '  } catch (error) { if (!(error instanceof Stop)) throw error; }',
    '}',
    `console.log(${JSON.stringify(marker)} + JSON.stringify({ calls, exportCount: Object.keys(family).length }));`,
  ].join("\n");
  const child = Bun.spawnSync([process.execPath, "-e", source], { cwd: root, stdout: "pipe", stderr: "pipe", timeout: 10_000 });
  const stdout = new TextDecoder().decode(child.stdout), stderr = new TextDecoder().decode(child.stderr);
  console.error(`[DEBUG] taxonomy ticket role routing child stdout=${JSON.stringify(stdout)} stderr=${JSON.stringify(stderr)}`);
  expect(child.signal ?? null).toBeNull();
  expect(child.exitCode).toBe(0);
  const line = stdout.split("\n").find((value) => value.startsWith(marker));
  expect(line).toBeDefined();
  const receipt = JSON.parse(line!.slice(marker.length)) as { readonly calls: readonly { readonly id: string; readonly options: { readonly ticketDir?: string } }[]; readonly exportCount: number };
  expect(receipt.exportCount).toBeGreaterThan(10);
  expect(receipt.calls).toHaveLength(vectors.cases.length);
  const observed = new Map(receipt.calls.map((entry) => [entry.id, entry.options.ticketDir === undefined ? null : relative("/virtual/workspace", entry.options.ticketDir).replaceAll("\\", "/")]));
  for (const row of vectors.cases) expect(observed.get(row.id), row.id).toBe(row.expected.nTicketDir);
  expect(createHash("sha256").update(readFileSync(rootScriptPath)).digest("hex")).toBe(rootBefore);
  expect(createHash("sha256").update(readFileSync(normalizationPath)).digest("hex")).toBe(normalizationBefore);
});
//#endregion 🧪️Subject
