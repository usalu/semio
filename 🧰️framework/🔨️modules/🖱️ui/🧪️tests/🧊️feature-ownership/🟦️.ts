import { expect, test } from "bun:test";
import { readFileSync, mkdirSync, writeFileSync } from "node:fs";
import { resolve, join } from "node:path";
import { spawnSync } from "node:child_process";

import TOML from "@iarna/toml";

type Law = { name: string; sha256: string; owner: "portable" | "engine" };
type Group = { id: string; portableSource: string; engineSource: string; productionSource: string; sharedDeclarations: string[]; helpers: { name: string; source: string; sha256: string }[]; laws: Law[] };
const ui = resolve(import.meta.dir, "../.."), native = join(ui, "📦️packages/🦀️rust");
const corpus = JSON.parse(readFileSync(join(ui, "🧫️fixtures/🧊️feature-ownership/🔣️.json"), "utf8")) as { groups: Group[]; targets: { name: string; features: string[] }[] };
function body(source: string, name: string): string {
  const start = source.search(new RegExp("fn " + name + "[<(]"));
  if (start < 0) throw Error("Missing retained law " + name);
  const firstLine = source.slice(start, source.indexOf("\n", start));
  if (firstLine.endsWith("{}")) return firstLine;
  const end = source.indexOf("\n}", start);
  if (end < 0) throw Error("Unclosed retained law " + name);
  return source.slice(start, end + 2);
}

test("the current closed native law corpus refuses substitution", () => {
  const hostile = structuredClone(corpus);
  hostile.groups[0]!.laws[0]!.owner = "engine";
  expect(() => auditGroup(hostile.groups[0]!)).toThrow();
  expect(corpus.groups.map(row => row.id)).toEqual(["input", "prepared", "theme"]);
  expect(corpus.groups.reduce((count, row) => count + row.laws.length, 0)).toBe(73);
  expect(corpus.groups.flatMap(row => row.laws).filter(row => row.owner === "engine").length).toBe(10);
});

function auditGroup(group: Group): void {
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
    const source = readFileSync(join(ui, helper.source), "utf8");
    expect([...source.matchAll(new RegExp("^fn " + helper.name + "[<(]", "gm"))].length).toBe(1);
    expect(new Bun.CryptoHasher("sha256").update(body(source, helper.name)).digest("hex")).toBe(helper.sha256);
    if (helper.source !== group.portableSource) {
      expect(new RegExp("fn " + helper.name + "[<(]").test(portable)).toBe(false);
      expect(portable).toContain('#[path = "../../' + helper.source + '"]');
    }
  }
  const observed = [...portable.matchAll(/^#\[(?:test|semio_framework_async_macros::test)\]\n(?:async )?fn (\w+)\(/gm), ...engine.matchAll(/^#\[(?:test|semio_framework_async_macros::test)\]\n(?:async )?fn (\w+)\(/gm)].map(row => row[1]).sort();
  expect(observed).toEqual(group.laws.map(row => row.name).sort());
  for (const law of group.laws) {
    const selected = law.owner === "engine" ? engine : portable, other = law.owner === "engine" ? portable : engine;
    expect(other.includes("fn " + law.name + "(")).toBe(false);
    expect(new Bun.CryptoHasher("sha256").update(body(selected, law.name)).digest("hex")).toBe(law.sha256);
  }
}

for (const group of corpus.groups) test(group.id + " retains each current law once at its declared feature owner", () => auditGroup(group));

test("independent Node hashing witnesses every current law and helper body", () => {
  const artifacts = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifacts) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned ticket output");
  const program = String.raw`
const fs=require('node:fs'),path=require('node:path'),crypto=require('node:crypto');
const rows=JSON.parse(fs.readFileSync(process.argv[1],'utf8')).groups,ui=process.argv[2];
const hash=(source,name)=>{const lines=fs.readFileSync(path.join(ui,source),'utf8').split('\n');const start=lines.findIndex(line=>new RegExp('^fn '+name+'[<(]').test(line));if(start<0)throw Error(name);let end=start;if(!lines[start].endsWith('{}')){end=lines.findIndex((line,index)=>index>start&&line==='}');if(end<0)throw Error(name);}return crypto.createHash('sha256').update(lines.slice(start,end+1).join('\n')).digest('hex');};
console.log(JSON.stringify({laws:rows.flatMap(row=>row.laws.map(law=>({name:law.name,sha256:hash(law.owner==='engine'?row.engineSource:row.portableSource,law.name)}))),helpers:rows.flatMap(row=>row.helpers.map(helper=>({name:helper.name,source:helper.source,sha256:hash(helper.source,helper.name)})))}));`;
  const child = spawnSync("node", ["--eval", program, join(ui, "🧫️fixtures/🧊️feature-ownership/🔣️.json"), ui], { encoding: "utf8" });
  expect(child.status, child.stderr).toBe(0);
  expect(JSON.parse(child.stdout)).toEqual({ laws: corpus.groups.flatMap(row => row.laws.map(({ name, sha256 }) => ({ name, sha256 }))), helpers: corpus.groups.flatMap(row => row.helpers) });
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
