/** 🧪️ Regeneration commands: 2 tests covering neutral regeneration commands round-trip with independent schema and canonical JSON oracles; neutral regeneration gate has declared script and typed dashboard entry points. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import stableStringify from "fast-json-stable-stringify";
import ts from "typescript";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { parseTaxonomyPlan } from "../../🧹️normalization/📃️plan-document/🟦️.ts";
import { taxonomyPlanDigest } from "../../🧹️normalization/🗃️affected-state/🟦️.ts";

/** 🏗️ Serialized generator plans retain the neutral launcher in both execution and verification vectors. */
test("neutral regeneration commands round-trip with independent schema and canonical JSON oracles", () => {
  const library = resolve(import.meta.dir, "../.."), fixture = JSON.parse(readFileSync(join(library, "🧪️tests/🧫️fixtures/🏗️regeneration-commands/🔣️.json"), "utf8")), schema = JSON.parse(readFileSync(join(library, "🧹️normalization/🧬️schema/🔣️.json"), "utf8")), ajv = new Ajv({ strict: false });
  ajv.addSchema(schema);
  const valid = ajv.compile({ $ref: schema.$id + "#/$defs/RegenerationCommand" }), hash = (value: unknown) => createHash("sha256").update(stableStringify(value)).digest("hex");
  for (const field of ["command", "verifyCommand"]) for (const vector of fixture.commands) {
    expect(valid(vector.command)).toBe(vector.valid);
    const command = ["repo", "run", "fixture:generate"], provisional = { contractId: fixture.contractId, cwd: fixture.cwd, command, verifyCommand: ["repo", "run", "fixture:check"], outputRoots: [], inputs: [], preOutputs: [], outputs: [], preview: { contractId: fixture.contractId, nodes: [], schemaVersion: 1, staleRemovals: [] }, previewManifestDigest: "4".repeat(64), staleRemovals: [], [field]: vector.command }, regeneration = { id: hash(provisional).slice(0, 24), ...provisional };
    const body = { schemaVersion: 2, taxonomySchemaVersion: 7, baselineCommit: "1".repeat(40), sourceTreeDigest: "2".repeat(64), excludedTreeDigests: [], moves: [], embeddedTicketRoots: [], embeddedTicketRootRelocations: [], symlinkTargetEdits: [], evidenceRemovals: [], destinationAncestorPreimages: [], edits: [], regenerations: [regeneration], unresolved: [], expectedAffectedPreStateDigest: "3".repeat(64), expectedPostStateDigest: "4".repeat(64) }, plan = { ...body, planDigest: hash(body) };
    if (vector.valid) {
      const parsed = parseTaxonomyPlan(JSON.parse(JSON.stringify(plan)));
      expect(JSON.parse(JSON.stringify(parsed))).toEqual(plan);
      expect(taxonomyPlanDigest(parsed)).toBe(hash(body));
    } else expect(() => parseTaxonomyPlan(plan)).toThrow(field + " is invalid");
  }
});

/** 🎛️ The authored project and typed dashboard entry points select the same neutral owner. */
test("neutral regeneration gate has declared script and typed dashboard entry points", () => {
  const library = resolve(import.meta.dir, "../.."), root = resolve(library, "../../../../.."), fixture = JSON.parse(readFileSync(join(library, "🧪️tests/🧫️fixtures/🏗️regeneration-commands/🔣️.json"), "utf8")), expected = fixture.registration, project = JSON.parse(readFileSync(join(library, "📦️packages/🟦️typescript/📋️project.json"), "utf8")), router = readFileSync(join(library, "📦️packages/🟦️typescript/📜️script.ts"), "utf8");
  expect(project.targets[expected.target]).toMatchObject({ executor: "repo:script", options: { cwd: "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript", command: "repo ./📜️script.ts test " + expected.route } });
  const source = ts.createSourceFile("script.ts", router, ts.ScriptTarget.Latest, true), routes: ts.IfStatement[] = [];
  const visit = (node: ts.Node): void => { if (ts.isIfStatement(node) && node.expression.getText(source) === `segments[0] === "${expected.route}"`) routes.push(node); ts.forEachChild(node, visit); };
  visit(source);
  expect(routes).toHaveLength(1);
  expect(routes[0]!.thenStatement.getText(source)).toContain("🧪️tests/🏗️regeneration-commands/🟦️.ts");
  {const dashboard=JSON.parse(readFileSync(process.cwd()+"/📋️project.json","utf8")).metadata.semio.dashboard,tool=dashboard.tools.find((row:any)=>row.id==="run-script");expect(tool.command).toEqual(["repo","exec","--projects={project}","--dry-run={dry-run}","--","bun","{workspace}/{script}"]);expect(tool.parameters.find((row:any)=>row.id==="script")).toMatchObject({kind:"text",required:true});}
});
