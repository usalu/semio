/** 🧪️ Concrete extension compositions preserve exact authored law suites. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { parse } from "@iarna/toml";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";

const root = fileURLToPath(new URL("../..", import.meta.url));
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const fixture = JSON.parse(read("🧪️tests/🧫️fixtures/🔣️.json")) as { package: string; suites: { target: string; path: string; targetPath: string; previousPath: string; sha256: string; tree?: boolean; laws: string[] }[]; retainedArtifactUnits: RetainedArtifactUnit[] };
/** 🧩️ One retained artifact test file: the family package and Cargo target that compile it and its exact laws. */
type RetainedArtifactUnit = { path: string; package: string; target: { kind: "lib" | "test"; name: string }; sha256: string; laws: string[] };
/** 🏷️ The selection that names every retained artifact unit group at once. */
export const RETAINED_ARTIFACT_UNITS = "retained-artifact-units";
const unitKey = (unit: RetainedArtifactUnit) => `${unit.package} ${unit.target.kind} ${unit.target.name}`;

/** 🧩️ Each retained artifact unit runs in the family package and Cargo target that compile it. */
export function retainedLawGroups() {
  return [...new Map(fixture.retainedArtifactUnits.map(unit => [unitKey(unit), unit])).values()].map(owner => ({ package: owner.package, target: owner.target, laws: fixture.retainedArtifactUnits.filter(unit => unitKey(unit) === unitKey(owner)).flatMap(unit => unit.laws) }));
}

/** 🧬️ A selected target name addresses its own target and every target named `<selection>-<slice>`, so one family can span several Cargo targets. */
export const inTargetFamily = (name: string, selection: string | undefined) => selection === undefined || name === selection || name.startsWith(`${selection}-`);

/** 🧾️ Each Cargo target executes the exact preserved laws in its own process. */
export function compositionLawGroups() {
  return [
    ...[...new Set(fixture.suites.map(suite => suite.target))].map(target => ({ package: fixture.package, target: { kind: "test" as const, name: target }, laws: fixture.suites.filter(suite => suite.target === target).flatMap(suite => suite.laws) })),
    ...retainedLawGroups(),
  ];
}

/** 🔍️ Independent TOML parsing validates authored ownership and preserved source. */
export function testCompositionOwnership(target?: string): void {
  const authored = read("📦️packages/🦀️rust/Cargo.toml");
  const manifest = Bun.TOML.parse(authored);
  assert.deepEqual(manifest, parse(authored));
  const packageRow=manifest.package;assert(packageRow&&typeof packageRow==="object"&&!Array.isArray(packageRow),"Actual composition package required");assert.equal(Reflect.get(packageRow,"name"),fixture.package);
  const metadata:unknown=Reflect.get(packageRow,"metadata");assert(metadata&&typeof metadata==="object"&&!Array.isArray(metadata),"Actual package metadata required");const semio:unknown=Reflect.get(metadata,"semio");assert(semio&&typeof semio==="object"&&!Array.isArray(semio),"Actual Semio metadata required");assert.equal(Reflect.get(semio,"role"),"test");
  const targets=manifest.test;assert(Array.isArray(targets),"Actual declared Cargo test targets required");
  for (const suite of fixture.suites.filter(suite => inTargetFamily(suite.target, target))) {
    const source = suite.tree === true ? readModuleTreeSource(resolve(root, suite.path)) : read(suite.path);
    assert.equal(createHash("sha256").update(source).digest("hex"), suite.sha256, suite.target);
    const previous = resolve(root, "../../..", suite.previousPath);
    for (const law of suite.laws) if (existsSync(previous)) assert(!readFileSync(previous, "utf8").includes(`fn ${law.split("::").at(-1)}(`), law);
    assert(targets.some(target=>target!==null&&typeof target==="object"&&!Array.isArray(target)&&Reflect.get(target,"name")===suite.target&&typeof Reflect.get(target,"path")==="string"&&resolve(root,"📦️packages/🦀️rust",String(Reflect.get(target,"path")))===resolve(root,suite.targetPath)), suite.target);
    for (const law of suite.laws) assert(source.includes(`fn ${law.split("::").at(-1)}(`), law);
  }
  for (const suite of fixture.retainedArtifactUnits.filter(unit => target === undefined || target === RETAINED_ARTIFACT_UNITS || unit.target.name === target)) {
    const source = readFileSync(resolve(root, "../../..", suite.path), "utf8");
    assert.equal(createHash("sha256").update(source).digest("hex"), suite.sha256, suite.path);
    for (const law of suite.laws) assert(source.includes(`fn ${law.split("::").at(-1)}(`), law);
  }
  const project = JSON.parse(read("📦️packages/🦀️rust/📋️project.json")) as { targets: Record<string, { options?: { command?: string } }> };
  const targetNames = new Set(compositionLawGroups().map(group => group.target.name));
  for (const [name, declaration] of Object.entries(project.targets)) {
    const selected = declaration.options?.command?.match(/(?:^|\s)--test\s+(\S+)/)?.[1];
    if (selected !== undefined) assert([...targetNames].some(name => inTargetFamily(name, selected)) || selected === RETAINED_ARTIFACT_UNITS, `${name}: unknown native law target ${selected}`);
  }
  const family = "../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d";
  const members = (parse(read(`${family}/Cargo.toml`)) as { workspace: { members: string[] } }).workspace.members.map(member => parse(read(`${family}/${member}/Cargo.toml`)) as { package: { name: string }; lib?: { name?: string }; test?: { name: string }[] });
  for (const unit of fixture.retainedArtifactUnits) {
    const owner = members.find(member => member.package.name === unit.package);
    assert(owner, unit.package);
    assert(unit.target.kind === "lib" ? unit.target.name === (owner.lib?.name ?? owner.package.name.replaceAll("-", "_")) : (owner.test ?? []).some(test => test.name === unit.target.name), `${unit.package} ${unit.target.name}`);
  }
}
