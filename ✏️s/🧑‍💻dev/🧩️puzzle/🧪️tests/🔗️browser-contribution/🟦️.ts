import assert from "node:assert/strict";
import { readFileSync, existsSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import { linkedSessionEngines } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚙️engine/🧭️selection/🟦️.ts";
import { declaredBrowserSessionEnginesV1 } from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚙️engine/🧭️selection/🟨️.mjs";

/** 🧩️ Binds actual Puzzle-owned runtime factory imports, engine producer and package declaration. */
export function testPuzzleBrowserContributionV1(workspace: string): void {
  const owner = "✏️s/🧑‍💻dev/🎭️variants/🧩️puzzle";
  const path = `${owner}/🧬️schema/🔣️.json`;
  const contribution = JSON.parse(readFileSync(join(workspace, path), "utf8"));
  const schema = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️contribution/🧬️schema/🔣️.json"), "utf8"));
  assert.ok(new Ajv({ strict: true }).compile(schema.properties.browserSessionFactories)(contribution.browserSessionFactories));
  const engines = linkedSessionEngines(contribution.browserSessionFactories);
  assert.deepEqual(declaredBrowserSessionEnginesV1(workspace, path), engines);
  assert.deepEqual(engines, ["./🌎️hub/🧩️compositions/🧩️puzzle/📦️packages/🦀️rust"]);
  const entry = ts.createSourceFile("entry.ts", readFileSync(join(workspace, contribution.browserEntry), "utf8"), ts.ScriptTarget.Latest, true);
  const imports = entry.statements.flatMap(node => ts.isImportDeclaration(node) && !node.importClause?.isTypeOnly && ts.isStringLiteral(node.moduleSpecifier) ? [node.moduleSpecifier.text] : []);
  const outward = JSON.parse(readFileSync(join(workspace, "✏️s/🧑‍💻dev/🧩️puzzle/📦️packages/🟦️typescript/package.json"), "utf8"));
  const general = JSON.parse(readFileSync(join(workspace, "🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/package.json"), "utf8"));
  for (const row of contribution.browserSessionFactories) {
    assert.ok(imports.includes(row.module));
    assert.equal(outward.dependencies[row.module], "workspace:*");
    assert.equal(general.dependencies[row.module], undefined);
    assert.ok(existsSync(join(workspace, row.engine, "Cargo.toml")));
  }
  assert.equal(general.semio?.browserSessionFactories, undefined);
  assert.ok(readFileSync(join(workspace, contribution.browserEntry), "utf8").includes("surfaceSessionFactories: PUZZLE_BOARD_SESSION_FACTORIES"));
  console.log("Puzzle browser contribution: actual owner entry, package, engine and independent compiler/schema authority passed");
}
