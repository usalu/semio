/** 🛫️ @vitest-environment node */
import { readFileSync } from "node:fs";
import Ajv from "ajv";
import { describe, expect, it } from "vitest";

type Package = { pluginId: string; componentPackageId: string; outputName: string; linkedCodecRegistry: string | null; linkedCodecRegistryPresent: boolean; descriptor: Record<string, unknown> };
type Case = { id: string; profileId: string; packages: Package[]; findings: string[] };
type Findings = (profileId: string, packages: readonly Package[]) => readonly string[];
const fixture = JSON.parse(readFileSync(new URL("../../🗿️artifact-authority/🔏️trusted-catalog/🧫️fixtures/🛫️catalog-selection-preflight/🔣️.json", import.meta.url), "utf8")) as { schema: string; cases: Case[] };

/** 🧬️ The exact `trustedBootstrapSelectionFindingsV1` (and the `trustedBootstrapDescriptorKindsV1` it reads kinds through) of the
 * hub's `📜️script.ts`, lifted out of its source and transpiled here: importing the script would load Bun-only modules the Vitest
 * node environment cannot run. */
async function publisherFindings(): Promise<Findings> {
  const ts = await import("typescript");
  const path = new URL("../../📦️packages/🦀️rust/📜️script.ts", import.meta.url);
  const syntax = ts.createSourceFile(path.pathname, readFileSync(path, "utf8"), ts.ScriptTarget.Latest, true, ts.ScriptKind.TS);
  const lift = (name: string): string => {
    const declaration = syntax.statements.find((node) => ts.isFunctionDeclaration(node) && node.name?.text === name);
    if (!declaration) throw new Error(`the hub publisher no longer defines ${name}`);
    return ts.transpileModule(declaration.getText(syntax).replace(/^export /u, ""), { compilerOptions: { target: ts.ScriptTarget.ES2022, module: ts.ModuleKind.None } }).outputText;
  };
  return new Function(`${lift("trustedBootstrapDescriptorKindsV1")}\n${lift("trustedBootstrapSelectionFindingsV1")}\nreturn trustedBootstrapSelectionFindingsV1;`)() as Findings;
}

/** 🛫️ LAW: a trusted-catalog selection is refused before any build when its profile id is not a bounded `local-…-open-v1` name,
 * a package identity or declared kind is unbounded, a package without linked codecs declares no artifact kind (hosted-only), or a
 * linked package's codec registry is missing — every finding at once, in package order. Oracle: Ajv judges the profile-id rule
 * independently (pattern + 256-byte bound; the fixture's ids are ASCII, so code points = bytes). */
describe("catalog selection preflight", async () => {
  const findings = await publisherFindings();
  const profileIdOracle = new Ajv().compile({ type: "string", minLength: 1, maxLength: 256, pattern: "^local-(?:[a-z0-9]+-)+open-v1$" });
  it("reads the language-agnostic fixture", () => {
    expect(fixture.schema).toBe("semio.hub.catalog-selection-preflight/v1");
    expect(fixture.cases.length).toBeGreaterThan(8);
  });
  for (const row of fixture.cases) {
    it(row.id, () => {
      const actual = findings(row.profileId, row.packages);
      expect(actual.length, actual.join("\n")).toBe(row.findings.length);
      row.findings.forEach((prefix, index) => expect(actual[index]!.startsWith(prefix), actual[index]).toBe(true));
      expect(profileIdOracle(row.profileId)).toBe(!actual.some((finding) => finding.startsWith("profile id")));
    });
  }
});
