import { describe, expect, test } from "bun:test";
import { existsSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { repoRootFromHere, testTaxonomy } from "../../📦️packages/🟦️typescript/🟦️.ts";

/** 🦀️ Independent oracle for `rustSutCratePath`: the same walk-up, written from scratch against raw
 * `fs` calls instead of importing the plugin's own helper, so the two can disagree if either is wrong. */
function oracleRustSutCratePath(workspaceRoot: string, ownerRel: string): string | null {
  const segments = ownerRel.split("/");
  for (let depth = segments.length; depth >= 0; depth -= 1) {
    const dir = segments.slice(0, depth).join("/");
    const manifest = join(workspaceRoot, dir, "📦️packages", "🦀️rust", "Cargo.toml");
    if (!existsSync(manifest)) continue;
    const name = readFileSync(manifest, "utf8").match(/^\s*name\s*=\s*"([^"]+)"/m)?.[1];
    if (name === "semio-repo-test-host") return null;
    if (name !== undefined) return `${dir}/📦️packages/🦀️rust`;
  }
  return null;
}

/** 🔮️ Independent oracle for `oracleContributionPaths`: same ancestor walk, own JSON reads. */
function oracleContributionPaths(workspaceRoot: string, ownerRel: string): string[] {
  const segments = ownerRel === "" ? [] : ownerRel.split("/");
  const paths: string[] = [];
  for (let depth = 0; depth <= segments.length; depth += 1) {
    const dir = segments.slice(0, depth).join("/");
    const manifest = join(workspaceRoot, dir, "🔮️oracles", "🔣️.json");
    if (!existsSync(manifest)) continue;
    const parsed = JSON.parse(readFileSync(manifest, "utf8")) as { oracleHostPackages?: readonly { path?: string }[] };
    for (const entry of parsed.oracleHostPackages ?? []) if (typeof entry.path === "string") paths.push(entry.path);
  }
  return paths;
}

describe("⚡️ test-exhaustive cache inputs cover the owner's real dependency closure", () => {
  const workspaceRoot = repoRootFromHere();

  test("rustSutCratePath finds a crate rooted well above a deeply nested owner, agreeing with an independent walk", async () => {
    const { internals } = await import("../../🟨️.mjs");
    const owner = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit";
    const observed = internals.rustSutCratePath(workspaceRoot, owner);
    expect(observed).toBe("✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust");
    expect(observed).toBe(oracleRustSutCratePath(workspaceRoot, owner));
    expect(existsSync(join(workspaceRoot, observed!, "Cargo.toml"))).toBe(true);
  });

  test("rustSutCratePath never links the generated host crate itself", async () => {
    const { internals } = await import("../../🟨️.mjs");
    const taxonomy = testTaxonomy(workspaceRoot);
    const owner = taxonomy.testDomainPath;
    expect(internals.rustSutCratePath(workspaceRoot, owner)).toBeNull();
    expect(oracleRustSutCratePath(workspaceRoot, owner)).toBeNull();
  });

  test("oracleContributionPaths resolves every ancestor's path-based oracle host package, agreeing with an independent walk", async () => {
    const { internals } = await import("../../🟨️.mjs");
    const vocabulary = internals.taxonomy(workspaceRoot);
    for (const owner of ["✏️s/🔌️plugins/🌍️gis", "✏️s/🔌️plugins/📕️norm", "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6"]) {
      const observed = internals.oracleContributionPaths(workspaceRoot, vocabulary, owner);
      expect([...observed].sort()).toEqual([...oracleContributionPaths(workspaceRoot, owner)].sort());
      expect(observed.length).toBeGreaterThan(0);
      for (const path of observed) expect(existsSync(join(workspaceRoot, path))).toBe(true);
    }
  });

  test("oracleContributionPaths returns nothing for an owner with no ancestor contribution", async () => {
    const { internals } = await import("../../🟨️.mjs");
    const vocabulary = internals.taxonomy(workspaceRoot);
    expect(internals.oracleContributionPaths(workspaceRoot, vocabulary, "🧰️framework/🔨️modules/🎠️kernel")).toEqual([]);
  });

  test("inputsFor threads both the crate root and the oracle contribution paths into a real case's cache inputs", async () => {
    const { internals } = await import("../../🟨️.mjs");
    const vocabulary = internals.taxonomy(workspaceRoot);
    const owner = "✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain";
    const caseRel = `${owner}/🧪️tests/🏔️mutate-gisterrain-1`;
    const inputs: string[] = internals.inputsFor(workspaceRoot, vocabulary, owner, caseRel, [`${caseRel}/🦀️.rs`]);
    expect(inputs).toContain(`{workspaceRoot}/${owner}/📦️packages/🦀️rust/**/*`);
    expect(inputs).toContain(`{workspaceRoot}/✏️s/🔌️plugins/🗄️stdio/🔮️oracles/📦️packages/🦀️rust/**/*`);
  });

  test("inputsFor adds no rust crate root when the case has no rust adapter", async () => {
    const { internals } = await import("../../🟨️.mjs");
    const vocabulary = internals.taxonomy(workspaceRoot);
    const owner = "🧰️framework/🔨️modules/🎠️kernel";
    const caseRel = `${owner}/🧪️tests/✅️satisfy-version-requirements`;
    const inputs: string[] = internals.inputsFor(workspaceRoot, vocabulary, owner, caseRel, [`${caseRel}/🟦️.ts`]);
    expect(inputs.some((input) => input.includes("📦️packages/🦀️rust"))).toBe(false);
  });

  test("every generated level target is cached alike, exhaustive included", async () => {
    const { default: plugin } = await import("../../🟨️.mjs");
    const featurePath = "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/✅️satisfy-version-requirements/🥒️.feature";
    const [, graph] = (await plugin.createNodesV2[1]([featurePath], {}, { workspaceRoot }))[0];
    const project = Object.values(graph.projects)[0] as { targets: Record<string, { cache?: boolean; outputs?: readonly string[] }> };
    for (const level of ["quick", "long", "exhaustive"]) {
      const target = project.targets[`test-${level}`];
      expect(target.cache, level).toBe(true);
      expect(target.outputs, level).toEqual(expect.arrayContaining([expect.stringContaining("/results"), expect.stringContaining("/reports"), expect.stringContaining("/diffs")]));
    }
  });
});
