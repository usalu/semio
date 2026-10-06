import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";

import ts from "typescript";


interface Case { readonly id: string; readonly before: string | null; readonly after: string | null; readonly accepted: boolean }
interface Fixture { readonly schemaVersion: 1; readonly hostSource: string; readonly nativeOwner: string; readonly module: string; readonly nativeLaw: string; readonly neutralSources: readonly string[]; readonly neutralRouter: string; readonly neutralLaws: readonly string[]; readonly cases: readonly Case[] }
const root = resolve(import.meta.dir, "../../../../../../../");
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🤝️cooperative-pump/🔣️.json", import.meta.url), "utf8")) as Fixture;
const source = (path: string): string => readFileSync(resolve(root, path), "utf8");

test("closed cooperative host ownership corpus retains all five original hostile witnesses", () => {
  expect(fixture.cases.filter(row => row.accepted)).toHaveLength(1);
  expect(fixture.cases.filter(row => !row.accepted)).toHaveLength(5);
  
});

test("original cooperative host law has one higher owner and all twelve neutral laws remain", async () => {
  const native = source(fixture.nativeOwner), host = source(fixture.hostSource);
  const bytes = new Uint8Array(await Bun.file(resolve(root, fixture.nativeOwner)).arrayBuffer());
  expect(new Bun.CryptoHasher("sha256").update(bytes).digest("hex")).toBe(createHash("sha256").update(native).digest("hex"));
  expect(native.match(/fn exact_live_pump_binding\(/gu)).toHaveLength(1);
  expect(native.match(/fn cooperative_maintenance_live_host_revisits_queued_owner\(/gu)).toHaveLength(1);
  expect(native).toContain('include_str!("../../🦀️.rs")');
  for (const row of fixture.cases.filter(row => !row.accepted)) {
    expect(host).toContain(row.before!);
    expect(native).toContain("source.replace(" + JSON.stringify(row.before) + ", " + JSON.stringify(row.after) + ")");
  }
  expect(host).toContain('#[cfg(test)]\n#[path = "🧪️tests/🤝️cooperative-pump/🦀️.rs"]\nmod ' + fixture.module + ";");
  for (const path of fixture.neutralSources) {
    const neutral = source(path);
    expect(neutral).not.toContain("exact_live_pump_binding");
    expect(neutral).not.toContain("cooperative_maintenance_live_host_revisits_queued_owner");
    expect(neutral).not.toContain("🛍️products");
  }
  const router = source(fixture.neutralRouter), tree = ts.createSourceFile(fixture.neutralRouter, router, ts.ScriptTarget.Latest, true);
  const owner = tree.statements.find(node => ts.isClassDeclaration(node) && node.name?.text === "WorkerMaintenanceCheckScript")!;
  const rosters: string[][] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isPropertyAssignment(node) && node.name.getText(tree) === "laws" && ts.isArrayLiteralExpression(node.initializer)) rosters.push(node.initializer.elements.map(element => ts.isStringLiteral(element) ? element.text : ""));
    ts.forEachChild(node, visit);
  };
  visit(owner);
  expect(rosters).toHaveLength(1);
  expect(rosters[0]).toEqual([...fixture.neutralLaws]);
  expect(rosters[0]).not.toContain("cooperative_maintenance_live_host_revisits_queued_owner");
  expect(rosters[0]).toContain("cooperative_maintenance_retains_deficit_until_later_host_turn");
  expect(rosters[0]).toContain("cooperative_maintenance_snapshot_contention_preserves_queued_job");
  console.log("[DEBUG] cooperative-host-owner native=1 neutral=12 hostile=5 physical-provider=Bun+Node");
});
