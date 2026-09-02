import { readFileSync } from "node:fs";
import { join } from "node:path";
import { describe, expect, it } from "vitest";
import { getWorkspaceRoot } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";

type GeneratorContract = { readonly previewTarget?: string };
type LaunchEntry = {
  readonly name?: string;
  readonly type?: string;
  readonly request?: string;
  readonly command?: string;
  readonly cwd?: string;
  readonly presentation?: { readonly group?: string; readonly order?: number };
};

describe("plugin registry generated preview launchers", () => {
  it("exposes every owned generator preview exactly once in contract order", () => {
    const repoRoot = getWorkspaceRoot();
    const taxonomy = JSON.parse(readFileSync(join(repoRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"), "utf8")) as {
      readonly generatorContracts: Readonly<Record<string, GeneratorContract>>;
    };
    const launch = Bun.JSONC.parse(readFileSync(join(repoRoot, ".vscode/launch.json"), "utf8")) as { readonly configurations: readonly LaunchEntry[] };
    const expected = Object.entries(taxonomy.generatorContracts).filter((entry): entry is [string, GeneratorContract & { readonly previewTarget: string }] => typeof entry[1].previewTarget === "string");

    expect(expected).toHaveLength(14);
    expected.forEach(([contractId, contract], index) => {
      const name = `📦️preview🤖️${contractId}`;
      const matches = launch.configurations.filter((entry) => entry.name === name);
      expect(matches, name).toEqual([
        {
          name,
          type: "node-terminal",
          request: "launch",
          command: `bun nx run ${contract.previewTarget}`,
          cwd: "${workspaceFolder}",
          presentation: { group: "4_build", order: Math.round((206.01 + index * 0.01) * 100) / 100 },
        },
      ]);
    });
  });
});
