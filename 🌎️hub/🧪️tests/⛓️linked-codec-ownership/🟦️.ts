/** ⛓️ @vitest-environment node */
import { readFileSync } from "node:fs";
import { describe, expect, it } from "vitest";

type Codec = { artifactKind: string; artifactSchema: string; packSchemaHash: string };
type Case = { id: string; declared: [string, string][]; linked: Codec[]; unowned?: string[]; refusal?: string };
type Classify = (pairs: readonly (readonly [string, string])[], linked: readonly Codec[]) => ReadonlySet<string>;
const fixture = JSON.parse(readFileSync(new URL("../../🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/⛓️linked-codec-ownership/🔣️.json", import.meta.url), "utf8")) as { schema: string; cases: Case[] };

/** 🧬️ The exact `trustedBootstrapLinkedUnownedKindsV1` of the hub's `📜️script.ts`, lifted out of its source and
 * transpiled here: importing the script would load Bun-only modules the Vitest node environment cannot run. */
async function publisherClassify(): Promise<Classify> {
  const ts = await import("typescript");
  const path = new URL("../../📦️packages/🦀️rust/📜️script.ts", import.meta.url);
  const syntax = ts.createSourceFile(path.pathname, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const declaration = syntax.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === "trustedBootstrapLinkedUnownedKindsV1");
  if (!declaration) throw new Error("the hub publisher no longer defines trustedBootstrapLinkedUnownedKindsV1");
  const javascript = ts.transpileModule(declaration.getText(syntax).replace(/^export /u, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.None } }).outputText;
  return new Function(`${javascript}\nreturn trustedBootstrapLinkedUnownedKindsV1;`)() as Classify;
}

/** ⛓️ LAW: a linked package (Stdio, GIS) owns exactly the kinds its linked native codecs carry; every other declared
 * kind is unowned and never becomes a hub open target, and a schema that disagrees with its linked row is refused. */
describe("linked codec ownership", async () => {
  const classify = await publisherClassify();
  it("reads the language-agnostic fixture", () => {
    expect(fixture.schema).toBe("semio.hub.linked-codec-ownership/v1");
    expect(fixture.cases.length).toBeGreaterThan(2);
  });
  for (const row of fixture.cases) {
    it(row.id, () => {
      const run = () => [...classify(row.declared, row.linked)].sort();
      if (row.refusal) expect(run).toThrow(row.refusal);
      else expect(run()).toEqual(row.unowned);
    });
  }
});
