import { expect, test } from "bun:test";
import { readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { resolve, join } from "node:path";
import { spawnSync } from "node:child_process";
import Ajv from "ajv";
import TOML from "@iarna/toml";

type Law = { name: string; sha256: string; owner: "portable" | "engine" };
type Group = { id: string; portableSource: string; engineSource: string; productionSource: string; sharedDeclarations: string[]; helpers: { name: string; sha256: string }[]; laws: Law[] };
const ui = resolve(import.meta.dir, "../.."), native = join(ui, "📦️packages/🦀️rust");
const corpus = JSON.parse(readFileSync(join(ui, "🧫️fixtures/🧊️feature-ownership/🔣️.json"), "utf8")) as { groups: Group[]; targets: { name: string; features: string[] }[] };
const validate = new Ajv({ strict: true }).compile(JSON.parse(readFileSync(join(ui, "🧬️schema/🧊️feature-ownership/🔣️.json"), "utf8")));
function body(source: string, name: string): string {
  const start = source.search(new RegExp("fn " + name + "[<(]"));
  if (start < 0) throw Error("Missing retained law " + name);
  const firstLine = source.slice(start, source.indexOf("\n", start));
  if (firstLine.endsWith("{}")) return firstLine;
  const end = source.indexOf("\n}", start);
  if (end < 0) throw Error("Unclosed retained law " + name);
  return source.slice(start, end + 2);
}

test("the exact closed native law corpus refuses substitution", () => {
  expect(validate(corpus), JSON.stringify(validate.errors)).toBe(true);
  const hostile = structuredClone(corpus);
  hostile.groups[0]!.laws[0]!.owner = "engine";
  expect(validate(hostile)).toBe(false);
  expect(corpus.groups.reduce((count, row) => count + row.laws.length, 0)).toBe(73);
  expect(corpus.groups.flatMap(row => row.laws).filter(row => row.owner === "engine").length).toBe(10);
});

for (const group of corpus.groups) test(group.id + " retains each original function once at its real feature owner", () => {
  const portable = readFileSync(join(ui, group.portableSource), "utf8"), engine = readFileSync(join(ui, group.engineSource), "utf8");
  const file = group.engineSource.split("/").slice(-2).join("/");
  expect(portable).toContain('#[cfg(feature = "wgpu-engine")]\n#[path = "../' + file + '"]\nmod engine_tests;');
  expect(engine.startsWith("use super::*;\n")).toBe(true);
  const production = readFileSync(join(ui, group.productionSource), "utf8");
  expect(production).toContain('#[cfg(test)]\n#[path = "../../../' + group.portableSource + '"]\nmod tests;');
  for (const declaration of group.sharedDeclarations) {
    expect(portable.split(declaration).length - 1).toBe(1);
    expect(engine.includes(declaration)).toBe(false);
  }
  for (const helper of group.helpers) {
    expect(new RegExp("fn " + helper.name + "[<(]").test(engine)).toBe(false);
    expect(new Bun.CryptoHasher("sha256").update(body(portable, helper.name)).digest("hex")).toBe(helper.sha256);
  }
  const observed = [...portable.matchAll(/^#\[(?:test|semio_framework_async_macros::test)\]\n(?:async )?fn (\w+)\(/gm), ...engine.matchAll(/^#\[(?:test|semio_framework_async_macros::test)\]\n(?:async )?fn (\w+)\(/gm)].map(row => row[1]).sort();
  expect(observed).toEqual(group.laws.map(row => row.name).sort());
  for (const law of group.laws) {
    const selected = law.owner === "engine" ? engine : portable, other = law.owner === "engine" ? portable : engine;
    expect(other.includes("fn " + law.name + "(")).toBe(false);
    expect(new Bun.CryptoHasher("sha256").update(body(selected, law.name)).digest("hex")).toBe(law.sha256);
  }
});

test("independent Node hashing witnesses all original law bodies", () => {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  const program = `const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');const rows=JSON.parse(fs.readFileSync(process.argv[1],'utf8')).groups;const ui=process.argv[2];const result=rows.flatMap(row=>row.laws.map(law=>{const source=fs.readFileSync(path.join(ui,law.owner==='engine'?row.engineSource:row.portableSource),'utf8');const match=new RegExp('^fn '+law.name+'\\\\([\\\\s\\\\S]*?^}', 'm').exec(source);if(!match)throw Error(law.name);return {name:law.name,sha256:crypto.createHash('sha256').update(match[0]).digest('hex')};}));console.log(JSON.stringify(result));`;
  const child = spawnSync("node", ["--eval", program, join(ui, "🧫️fixtures/🧊️feature-ownership/🔣️.json"), ui], { encoding: "utf8" });
  expect(child.status, child.stderr).toBe(0);
  expect(JSON.parse(child.stdout)).toEqual(corpus.groups.flatMap(row => row.laws.map(({ name, sha256 }) => ({ name, sha256 }))));
  mkdirSync(artifacts, { recursive: true });
  writeFileSync(join(artifacts, "ui-feature-ownership-node-receipt.json"), child.stdout);
});

test("owned targets preserve features and test providers select real neutral capabilities", () => {
  const manifest = TOML.parse(readFileSync(join(native, "Cargo.toml"), "utf8"));
  const dev = manifest["dev-dependencies"] as Record<string, { path: string; features?: string[] }>;
  expect(dev["semio-framework-async"]!.features).toEqual(["entrypoint"]);
  expect(TOML.parse(readFileSync(resolve(native, dev["semio-framework-async"]!.path, "Cargo.toml"), "utf8")).package).toMatchObject({ name: "semio-framework-async" });
  expect(TOML.parse(readFileSync(resolve(native, dev["semio-framework-ui-viewport"]!.path, "Cargo.toml"), "utf8")).package).toMatchObject({ name: "semio-framework-ui-viewport" });
  const script = readFileSync(join(native, "📜️script.ts"), "utf8");
  expect(script).toContain('extraArgs: ["--features", "tui-terminal,wgpu", ...rest]');
  expect(script).toContain('extraArgs: ["--features", "wgpu-engine", "--lib", ...rest]');
  expect(corpus.targets).toEqual([{ name: "test", features: ["tui-terminal", "wgpu"] }, { name: "test-wgpu-engine", features: ["wgpu-engine"] }]);
});
