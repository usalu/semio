/** 🔑️ Contract: a test that reads a workspace file by its full path key that file. The ui-styling test target declares its out-of-region data reads as closure resources; this law harvests every full-path string literal of the test tree (sources and fixture JSON), keeps the existing data files (module sources join through the closure) outside the regions the target already keys (`ui/**`, `assets/**`), and requires exactly those files as the declared resources, so a new literal read without a declaration fails and a stale declaration fails too. */
import {expect, test} from "bun:test";
import {existsSync, readdirSync, readFileSync, statSync} from "node:fs";
import {dirname, join} from "node:path";

const root = (() => { let at = import.meta.dir; while (!existsSync(join(at, "nx.json"))) at = dirname(at); return at; })();
const project = "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🟦️typescript", tests = "🧰️framework/🔨️modules/🖱️ui/🎨️styling/🧪️tests";
const regions = ["🧰️framework/🔨️modules/🖱️ui/", "🧰️framework/🔨️modules/🖼️assets/"], prefixes = ["🧰️framework/", "🎓️teaching/", "✏️s/", "🌎️hub/", "♻️mit-bestand/"];
const walk = (directory: string): string[] => readdirSync(join(root, directory), {withFileTypes: true}).flatMap((entry) => entry.isDirectory() ? (entry.name === "node_modules" ? [] : walk(`${directory}/${entry.name}`)) : [`${directory}/${entry.name}`]);

test("the ui-styling test target keys exactly the files its tests name by full path outside its keyed regions", () => {
  const harvested = new Set<string>();
  for (const path of walk(tests)) {
    if (!/\.(ts|tsx|json)$/u.test(path)) continue;
    for (const match of readFileSync(join(root, path), "utf8").matchAll(/["'`]([^"'`\n$\\]{8,300})["'`]/gu)) {
      const value = match[1]!;
      if (prefixes.some((prefix) => value.startsWith(prefix)) && !/[*?{}]/u.test(value) && existsSync(join(root, value)) && statSync(join(root, value)).isFile() && !/\.[cm]?[jt]sx?$/u.test(value) && !regions.some((region) => value.startsWith(region))) harvested.add(value);
    }
  }
  const target = JSON.parse(readFileSync(join(root, project, "📋️project.json"), "utf8")).targets.test;
  const declared = (target.metadata?.semio?.closure?.resources ?? []).map((entry: string) => entry.replace("{workspaceRoot}/", ""));
  expect(declared.length, "the declaration exists").toBeGreaterThan(0);
  expect([...harvested].sort(), "declared resources equal the harvested data reads").toEqual([...declared].sort());
  for (const path of declared) expect(statSync(join(root, path)).isFile(), path).toBe(true);
});
