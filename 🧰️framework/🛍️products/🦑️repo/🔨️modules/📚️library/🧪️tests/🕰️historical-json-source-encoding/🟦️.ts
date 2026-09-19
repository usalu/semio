import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { existsSync, lstatSync, readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import { findNodeAtLocation, getNodeValue, parseTree } from "jsonc-parser";
import { frozenCoordinateEvidenceSeal, loadCatalogTaxonomy, validateFrozenCoordinateEvidenceContracts } from "../../🔍️discovery/🟦️.ts";
import { canonicalJson, frozenCoordinateEvidenceCoordinates } from "../../🧹️normalization/🟦️.ts";

const vector = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🕰️historical-json-source-encoding/🔣️.json"), "utf8"));
const historical = JSON.parse(readFileSync(join(import.meta.dir, "../../🧫️fixtures/🕰️historical-json-source-encoding/🧬️energy-source-coordinates/🔣️.json"), "utf8"));
const sha = (value: string) => createHash("sha256").update(value).digest("hex");
const libraryRoot = resolve(import.meta.dir, "../.."), root = resolve(libraryRoot, "../../../../..");

test("historical escaped-source vectors bind one JSON string layer and an explicit root", () => {
  const validate = new Ajv().compile({ type: "object", required: ["schemaVersion", "contract", "semantics", "cases"], properties: { schemaVersion: { const: 1 }, contract: { const: "historical-json-escaped-source-coordinates-v1" }, cases: { type: "array", minItems: 15, items: { type: "object", required: ["id", "source", "pointer", "accepted"] } } } });
  expect(validate(vector), JSON.stringify(validate.errors)).toBe(true);
  for (const row of vector.cases.filter((row: any) => row.accepted)) {
    const tree = parseTree(row.source)!;
    expect(getNodeValue(tree)).toEqual(JSON.parse(row.source));
    const node = findNodeAtLocation(tree, row.pointer.slice(1).split("/").map((part: string) => /^\d+$/.test(part) ? Number(part) : part))!;
    expect(node.type).toBe("string");
    const raw = row.source.slice(node.offset + 1, node.offset + node.length - 1);
    expect(JSON.parse('"' + raw + '"')).toBe(node.value);
    expect(raw).not.toBe(node.value);
  }
});

for (const row of vector.cases) test("historical escaped-source authority: " + row.id, () => {
  const contract = { path: "🧪️tests/🔣️history.json", sha256: sha(row.source), schemaVersion: null, ...(row.rootKind ? { rootKind: row.rootKind } : {}), coordinates: [{ pointer: row.pointer, kind: "source", representation: "json-escaped-source-path" }] };
  const run = () => frozenCoordinateEvidenceCoordinates(contract.path, Buffer.from(row.source), { history: contract } as never);
  if (!row.accepted) expect(run).toThrow(/frozen-coordinate-evidence-invalid/u);
  else {
    const node = findNodeAtLocation(parseTree(row.source)!, row.pointer.slice(1).split("/").map((part: string) => /^\d+$/.test(part) ? Number(part) : part))!;
    expect(run()).toEqual([{ pointer: row.pointer, start: node.offset + 1, end: node.offset + node.length - 1, value: row.source.slice(node.offset + 1, node.offset + node.length - 1), kind: "source" }]);
  }
});

test("escaped-source authority retains exact representation root digest and selector boundaries", () => {
  const source = vector.cases[0].source, bytes = Buffer.from(source), path = "🧪️tests/🔣️history.json";
  const contract = { path, sha256: sha(source), schemaVersion: null, rootKind: "array", coordinates: [{ pointer: "/0/path", kind: "source", representation: "json-escaped-source-path" }] };
  const run = (value: any, input = bytes) => frozenCoordinateEvidenceCoordinates(value.path, input, { history: value });
  expect(validateFrozenCoordinateEvidenceContracts({ history: contract })).toEqual([]);
  expect(() => run(contract, Buffer.concat([bytes, Buffer.from("\n")]))).toThrow(/digest/u);
  for (const alter of [
    (value: any) => { value.rootKind = "object"; },
    (value: any) => { value.rootKind = null; },
    (value: any) => { value.schemaVersion = 1; },
    (value: any) => { value.extra = true; },
    (value: any) => { value.coordinates[0].kind = "destination"; },
    (value: any) => { value.coordinates[0].representation = "json"; },
    (value: any) => { value.coordinates[0].recordedRepositoryRoot = "/recorded"; },
    (value: any) => { value.coordinates[0].pointer = "/00/path"; },
    (value: any) => { value.coordinates.push({ ...value.coordinates[0], pointer: "/*/path" }); },
    (value: any) => { delete value.coordinates[0].representation; },
  ]) {
    const changed = structuredClone(contract);
    alter(changed);
    expect(() => run(changed)).toThrow(/frozen-coordinate-evidence-invalid/u);
  }
  const invalid = Buffer.concat([bytes, Buffer.from([0xff])]);
  expect(() => run({ ...contract, sha256: createHash("sha256").update(invalid).digest("hex") }, invalid)).toThrow(/UTF-8/u);
  const object = '{"path":"tests/source.rs"}', plain = { path, sha256: sha(object), schemaVersion: null, coordinates: [{ pointer: "/path", kind: "source" as const }] };
  const node = findNodeAtLocation(parseTree(object)!, ["path"])!;
  expect(frozenCoordinateEvidenceCoordinates(path, Buffer.from(object), { history: plain })).toEqual([{ pointer: "/path", start: node.offset + 1, end: node.offset + node.length - 1, value: node.value, kind: "source" }]);
  expect(() => run({ ...contract, sha256: sha(object) }, Buffer.from(object))).toThrow(/root/u);
});

test("one exact encoded historical source is registered without changing the previous JSON contracts", () => {
  const contracts = loadCatalogTaxonomy().frozenCoordinateEvidenceContracts;
  expect(frozenCoordinateEvidenceSeal(contracts)[historical.id]).toEqual(historical.contract);
  expect(validateFrozenCoordinateEvidenceContracts(contracts)).toEqual([]);
  const original = Object.fromEntries(Object.entries(frozenCoordinateEvidenceSeal(contracts)).filter(([id]) => id !== historical.id));
  expect(Object.keys(original)).toHaveLength(historical.originalContracts.count);
  expect(sha(canonicalJson(original))).toBe(historical.originalContracts.canonicalSha256);
});

test("retirement is recorded evidence: every retired contract names its ticket and is genuinely gone", () => {
  const contracts = loadCatalogTaxonomy().frozenCoordinateEvidenceContracts;
  const rows = Object.entries(contracts);
  expect(rows.length).toBeGreaterThan(0);
  const retired = rows.filter(([, contract]) => contract.retired !== undefined);
  expect(retired.length).toBeGreaterThan(0);
  for (const [id, contract] of rows) {
    const present = existsSync(join(root, contract.path));
    expect([id, contract.retired === undefined]).toEqual([id, present]);
    if (!contract.retired) continue;
    expect(contract.retired.reason).toBe("ticket-close-generated-output-removed");
    expect(contract.path.startsWith(`.🧬semio/🦑️repo/🎫️tickets/🎆️${contract.retired.ticket.slice(2, 4)}/🌙️${contract.retired.ticket.slice(5, 7)}/☀️${contract.retired.ticket.slice(8, 10)}/${contract.retired.ticket.slice(11)}/`)).toBe(true);
  }
});

test("a retirement record can never move the evidence seal", () => {
  const contracts = loadCatalogTaxonomy().frozenCoordinateEvidenceContracts;
  const sealed = sha(canonicalJson(frozenCoordinateEvidenceSeal(contracts)));
  const rebranded = Object.fromEntries(Object.entries(contracts).map(([id, contract]) => [id, { ...contract, retired: { ticket: "2026/01/01/SEAL-PROBE", reason: "ticket-close-generated-output-removed" as const } }]));
  expect(sha(canonicalJson(frozenCoordinateEvidenceSeal(rebranded)))).toBe(sealed);
  const [firstId, firstContract] = Object.entries(contracts)[0]!;
  const edited = { ...contracts, [firstId]: { ...firstContract, sha256: firstContract.sha256.replace(/^./u, (character) => (character === "0" ? "1" : "0")) } };
  expect(sha(canonicalJson(frozenCoordinateEvidenceSeal(edited)))).not.toBe(sealed);
});

test("the genuine 164-entry snapshot preserves the exact escaped source span and physical bytes", () => {
  let physical = root;
  const parts = historical.contract.path.split("/");
  expect(/^(?:compose|temp\/compose)(?:\/|$)/u.test(historical.contract.path)).toBe(false);
  // 🪦️ This contract is RETIRED: the ticket that produced the snapshot deleted it at close, as
  // AGENTS.md requires. The frozen evidence — digest, size, coordinate spans — is what this suite
  // preserves, and it lives in the fixture beside this file; the physical bytes cannot be re-read and
  // asserting them would only re-report the deletion the retirement already records. What is asserted
  // instead is that the deletion is REAL, so "retired" can never be used to excuse a live document.
  const retirement = loadCatalogTaxonomy().frozenCoordinateEvidenceContracts[historical.id]?.retired;
  if (retirement) {
    expect(existsSync(join(root, historical.contract.path))).toBe(false);
    expect(retirement.reason).toBe("ticket-close-generated-output-removed");
    expect(sha(canonicalJson(historical.contract))).toBe(sha(canonicalJson(frozenCoordinateEvidenceSeal(loadCatalogTaxonomy().frozenCoordinateEvidenceContracts)[historical.id]!)));
    expect(historical.coordinate.end).toBeGreaterThan(historical.coordinate.start);
    return;
  }
  for (const [index, part] of parts.entries()) {
    expect(part !== "" && part !== "." && part !== "..").toBe(true);
    physical = join(physical, part);
    const stat = lstatSync(physical);
    expect(stat.isSymbolicLink()).toBe(false);
    expect(index === parts.length - 1 ? stat.isFile() : stat.isDirectory()).toBe(true);
  }
  const stat = lstatSync(physical), bytes = readFileSync(physical), source = bytes.toString("utf8"), tree = parseTree(source)!;
  expect(createHash("sha256").update(bytes).digest("hex")).toBe(historical.contract.sha256);
  expect(bytes.length).toBe(historical.size);
  expect(stat.mode & 0o7777).toBe(historical.mode);
  expect(JSON.parse(source)).toHaveLength(historical.rootEntries);
  expect(getNodeValue(tree)).toEqual(JSON.parse(source));
  const node = findNodeAtLocation(tree, [16, "fixtures", 0, "path"])!, raw = source.slice(node.offset + 1, node.offset + node.length - 1);
  expect(node.type).toBe("string");
  expect(JSON.parse('"' + raw + '"')).toBe(node.value);
  expect({ start: node.offset + 1, end: node.offset + node.length - 1, rawSha256: sha(raw), valueSha256: sha(node.value), rawSize: Buffer.byteLength(raw), valueSize: Buffer.byteLength(node.value) }).toEqual(historical.coordinate);
  expect(frozenCoordinateEvidenceCoordinates(historical.contract.path, bytes, { [historical.id]: historical.contract })).toEqual([{ pointer: historical.contract.coordinates[0].pointer, start: historical.coordinate.start, end: historical.coordinate.end, value: raw, kind: "source" }]);
  expect(readFileSync(physical)).toEqual(bytes);
  expect(lstatSync(physical).mode).toBe(stat.mode);
});

test("both historical source gates are mounted through Nx and exact launch registrations", () => {
  const projectSource = readFileSync(join(libraryRoot, "📦️packages/🟦️typescript/📋️project.json"), "utf8"), project = JSON.parse(projectSource);
  expect(getNodeValue(parseTree(projectSource)!)).toEqual(project);
  for (const row of vector.execution) {
    expect(project.targets["test-" + row.id]?.options.command).toBe("bun ./📜️script.ts test " + row.id);
    for (const path of [".vscode/🧩️launch.seed.jsonc", ".vscode/launch.json"]) {
      const document = getNodeValue(parseTree(readFileSync(join(root, path), "utf8"))!), matches = document.configurations.filter((entry: any) => entry.name === row.name);
      expect(matches).toEqual([{ name: row.name, type: "node-terminal", request: "launch", command: "bun nx run @semio-tech/repo-lib:test-" + row.id + " --skip-nx-cache", cwd: "${workspaceFolder}", presentation: { group: "4_gate", order: row.order } }]);
    }
  }
});
