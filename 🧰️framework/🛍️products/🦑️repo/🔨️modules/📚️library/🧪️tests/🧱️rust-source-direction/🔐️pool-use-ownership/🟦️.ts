import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import ts from "typescript";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

interface Fixture { readonly schemaVersion: 1; readonly neutralFixture: string; readonly neutralRouter: string; readonly mountedFixture: string; readonly mountedRouter: string; readonly neutralLaws: readonly string[]; readonly mountedLaws: readonly string[]; readonly mountedCaseIds: readonly string[]; readonly neutralCaseIds: readonly string[] }
const root = resolve(import.meta.dir, "../../../../../../../..");
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧱️rust-source-direction/🔐️pool-use-ownership/🔣️.json", import.meta.url), "utf8")) as Fixture;

const source = (path: string): string => readFileSync(resolve(root, path), "utf8");
const json = (path: string): any => JSON.parse(source(path));

function delegatedOwner(path: string, name: string): { tree: ts.SourceFile; node: ts.Node } {
  const tree = ts.createSourceFile(path, readModuleTreeSource(resolve(root, path), { tests: true }), ts.ScriptTarget.Latest, true);
  const declared = tree.statements.find(node => ts.isClassDeclaration(node) && node.name?.text === name)!;
  const callee = /await\s+(\w+)\(/u.exec(declared.getText(tree))?.[1];
  const delegate = callee === undefined ? undefined : tree.statements.find(node => ts.isFunctionDeclaration(node) && node.name?.text === callee);
  return { tree, node: delegate ?? declared };
}

function nativeRosters(path: string, name: string, qualified: readonly string[] = []): string[][] {
  const { tree, node: owner } = delegatedOwner(path, name);
  const rosters: string[][] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isArrayLiteralExpression(node) && node.elements.length === 2 && ts.isStringLiteral(node.elements[0]!) && node.elements[0]!.text.endsWith("🦀️.rs") && ts.isArrayLiteralExpression(node.elements[1]!)) rosters.push(node.elements[1]!.elements.map(element => ts.isStringLiteral(element) ? qualified.find(full => full.endsWith("::" + element.text)) ?? element.text : ""));
    if (ts.isPropertyAssignment(node) && node.name.getText(tree) === "laws" && ts.isArrayLiteralExpression(node.initializer)) rosters.push(node.initializer.elements.map(element => ts.isStringLiteral(element) ? element.text : ""));
    ts.forEachChild(node, visit);
  };
  visit(owner);
  return rosters;
}

test("closed pool use ownership corpus retains neutral and mounted original laws", () => {
  
  expect(fixture["schemaVersion"]).toEqual(1);expect(fixture["neutralFixture"]).toEqual("🧰️framework/🔨️modules/⏳️async/🔐️use/🧪️tests/🧫️fixtures/🔣️.json");expect(fixture["neutralRouter"]).toEqual("🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust/📜️script.ts");expect(fixture["mountedFixture"]).toEqual("🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/⚙️engine/🚪️document-mount/🧪️tests/🧫️fixtures/🔐️pool-use/🔣️.json");expect(fixture["mountedRouter"]).toEqual("🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands/📄️document-checks/🟦️.ts");expect(fixture["neutralLaws"]).toEqual(["native_pool::tests::worker_pool_use_native_busy_keeps_executor_running_until_final_release","native_pool::tests::worker_pool_use_acquire_and_shutdown_linearize_exactly_once","wasm_pool::cooperative_tests::worker_pool_use_cooperative_busy_keeps_executor_running_until_final_release"]);expect(fixture["mountedLaws"]).toEqual(["db_engine::tests::database_worker_pool_use_blocks_early_shutdown_and_releases_at_terminal_ack","db_engine::tests::database_worker_pool_use_is_admitted_before_the_first_storage_probe","db_engine::tests::database_document_mount_hard_scheduler_fault_retains_nonrunnable_job_without_retry_timer"]);expect(fixture["mountedCaseIds"]).toEqual(["database-retains-one-clone-shared-use","terminal-database-releases-exact-use","stopped-pool-refuses-before-storage-probe","hard-scheduler-fault-retains-job","terminal-database-fences-activity-while-unrelated-use-runs"]);expect(fixture["neutralCaseIds"]).toEqual(["held-use-refuses-shutdown","arc-clones-count-one-use-cell","final-drop-enables-shutdown","closing-fences-racing-acquire","stopped-shutdown-is-idempotent"]);
  
  for (const candidate of [{ ...fixture, unknown: true }, { ...fixture, neutralLaws: fixture.neutralLaws.slice(1) }, { ...fixture, mountedCaseIds: [] }]) {
    
    
  }
});

test("neutral pool and mounted database have separate closed authority and original native clients", () => {
  const mounted = json(fixture.mountedFixture), neutral = json(fixture.neutralFixture);
  expect(mounted.cases.map((row: { id: string }) => row.id)).toEqual([...fixture.mountedCaseIds]);
  expect(neutral.cases).toHaveLength(5);
  expect(neutral.cases.map((row: { id: string }) => row.id)).toEqual([...fixture.neutralCaseIds]);
  expect(Object.hasOwn(neutral, "mountedCases")).toBe(false);
  const path = fixture.neutralRouter, delegated = delegatedOwner(path, "WorkerPoolUseCheckScript");
  const owner = delegated.node.getText(delegated.tree);
  expect(owner).not.toContain("🛍️products");
  expect(owner).not.toContain("mountedCases");
  expect([...new Set(nativeRosters(path, "WorkerPoolUseCheckScript", fixture.neutralLaws).flat())]).toEqual([...fixture.neutralLaws]);
  expect(nativeRosters(fixture.mountedRouter, "DocumentMountSingleFlightCheckScript", fixture.mountedLaws)[0]).toEqual(expect.arrayContaining([...fixture.mountedLaws]));
  const higherOwner = delegatedOwner(fixture.mountedRouter, "DocumentMountSingleFlightCheckScript");
  const higher = higherOwner.node.getText(higherOwner.tree);
  expect(higher).toContain('"🧪️tests/🧫️fixtures/🔐️pool-use/🔣️.json"');
  expect(higher).toContain("mountedPoolUse.cases");
  for (const marker of ["missing mounted pool-use marker", "missing authority pool-use marker", "missing sync-hello pool-use marker", "missing mounted pool-use law"]) expect(higher).toContain(marker);
});
