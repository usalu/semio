import assert from "node:assert/strict";
import { readFileSync, existsSync } from "node:fs";
import { join, resolve } from "node:path";
import Ajv from "ajv";
import ts from "typescript";

/** 🌉️ Proves physical service composition ownership with independent schema, AST and module-build oracles. */
export async function serviceMcpCompositionOwnership(repoRoot: string): Promise<void> {
  const owner = resolve(import.meta.dir, "../..");
  const read = (path: string): any => JSON.parse(readFileSync(path, "utf8"));
  const fixture = read(join(owner, "🧫️fixtures/🌉️mcp-composition/🔣️.json"));
  const validate = new Ajv({ strict: true, allErrors: true }).compile(read(join(owner, "🧬️schema/🌉️mcp-composition/🔣️.json")));
  assert(validate(fixture), JSON.stringify(validate.errors));
  const project = read(join(repoRoot, fixture.packageRoot, "📋️project.json"));
  const generic = read(join(repoRoot, fixture.genericPackageRoot, "📋️project.json"));
  const router = ts.createSourceFile("script.ts", readFileSync(join(repoRoot, fixture.packageRoot, "📜️script.ts"), "utf8"), ts.ScriptTarget.Latest, true);
  const registrations: string[] = [];
  const visit = (node: ts.Node): void => {
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "register" && node.arguments[0] && ts.isStringLiteral(node.arguments[0])) registrations.push(node.arguments[0].text);
    ts.forEachChild(node, visit);
  };
  visit(router);
  assert.equal(project.name, fixture.project);
  for (const row of fixture.cases) {
    assert(existsSync(join(repoRoot, row.source)), row.source);
    assert(!existsSync(join(repoRoot, row.removedSource)), row.removedSource);
    assert.equal(project.targets[row.target].options.cwd, fixture.packageRoot);
    assert.equal(project.targets[row.target].options.command, row.command);
    assert.equal(generic.targets[row.target], undefined);
    assert(registrations.includes(row.target));
    const built = await Bun.build({ entrypoints: [join(repoRoot, row.source)], target: "bun", packages: "external" });
    assert(built.success, built.logs.map(String).join("\n"));
  }
  const recipe = read(join(owner, "🧫️fixtures/🧷️untrusted-content/🔣️.json"));
  const recipeSchema = new Ajv({ strict: true, allErrors: true }).compile(read(join(owner, "🧬️schema/🧷️untrusted-content/🔣️.json")));
  assert(recipeSchema(recipe), JSON.stringify(recipeSchema.errors));
  const law = read(join(repoRoot, recipe.lawPath));
  assert.equal(law.plants, undefined);
  const neutralSchema = read(join(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🗿️artifact/🧬️schema/🔣️.json"));
  const ajv = new Ajv({ strict: true, allErrors: true });
  ajv.addSchema(neutralSchema);
  const neutralLaw = ajv.getSchema(neutralSchema.$id + "#/$defs/UntrustedContentLawV1")!;
  assert(neutralLaw(law), JSON.stringify(neutralLaw.errors));
  assert(!neutralLaw({ ...law, plants: recipe.plants }));
  console.log(`services-mcp-composition: physical=${fixture.cases.length} independentAst=true independentAjv=true modules=${fixture.cases.length} neutral-law=true`);
}
