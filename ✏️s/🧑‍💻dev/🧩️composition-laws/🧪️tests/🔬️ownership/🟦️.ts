/** 🧪️ Concrete extension compositions preserve exact authored law suites. */
import assert from "node:assert/strict";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv from "ajv";
import { parse } from "@iarna/toml";

const root = fileURLToPath(new URL("../../", import.meta.url));
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const fixture = JSON.parse(read("🧫️fixtures/🔣️.json")) as { package: string; suites: { target: string; path: string; targetPath: string; previousPath: string; sha256: string; laws: string[] }[]; retainedArtifactUnits: { path: string; sha256: string; laws: string[] }[] };

/** 🧾️ Each Cargo target executes the exact preserved laws in its own process. */
export function compositionLawGroups() {
  return [
    ...[...new Set(fixture.suites.map(suite => suite.target))].map(target => ({ package: fixture.package, target: { kind: "test" as const, name: target }, laws: fixture.suites.filter(suite => suite.target === target).flatMap(suite => suite.laws) })),
    ...(fixture.retainedArtifactUnits.length ? [{ package: "semio-s-artifact-procedural-generation3d", target: { kind: "lib" as const, name: "semio_s_artifact_procedural_generation3d" }, cargoArgs: ["--features", "component-app-assembly"], laws: fixture.retainedArtifactUnits.flatMap(suite => suite.laws) }] : []),
  ];
}

/** 🔍️ AJV and independent TOML parsing validate authored ownership and preserved source. */
export function testCompositionOwnership(target?: string): void {
  const schema = JSON.parse(read("🧬️schema/🔣️.json"));
  const ajv = new Ajv({ strict: true, allErrors: true }).addKeyword("x-semio-formats").addSchema(schema);
  const validate = ajv.getSchema(`${schema.$id}#/$defs/SCompositionLawsV1`)!;
  assert(validate(fixture), JSON.stringify(validate.errors));
  const authored = read("📦️packages/🦀️rust/Cargo.toml");
  const manifest = Bun.TOML.parse(authored) as any;
  assert.deepEqual(manifest, parse(authored));
  assert.equal(manifest.package.name, fixture.package);
  assert.equal(manifest.package.metadata.semio.role, "test");
  for (const suite of fixture.suites.filter(suite => target === undefined || suite.target === target)) {
    const source = read(suite.path);
    assert.equal(createHash("sha256").update(source).digest("hex"), suite.sha256, suite.target);
    const previous = resolve(root, "../../..", suite.previousPath);
    for (const law of suite.laws) if (existsSync(previous)) assert(!readFileSync(previous, "utf8").includes(`fn ${law.split("::").at(-1)}(`), law);
    assert(manifest.test.some((target: any) => target.name === suite.target && resolve(root, "📦️packages/🦀️rust", target.path) === resolve(root, suite.targetPath)), suite.target);
    for (const law of suite.laws) assert(source.includes(`fn ${law.split("::").at(-1)}(`), law);
  }
  for (const suite of target === undefined || target === "semio_s_artifact_procedural_generation3d" ? fixture.retainedArtifactUnits : []) {
    const source = readFileSync(resolve(root, "../../..", suite.path), "utf8");
    assert.equal(createHash("sha256").update(source).digest("hex"), suite.sha256, suite.path);
    for (const law of suite.laws) assert(source.includes(`fn ${law.split("::").at(-1)}(`), law);
  }
  const project = JSON.parse(read("📦️packages/🦀️rust/📋️project.json")) as { targets: Record<string, { options?: { command?: string } }> };
  const targetNames = new Set(compositionLawGroups().map(group => group.target.name));
  for (const [name, declaration] of Object.entries(project.targets)) {
    const selected = declaration.options?.command?.match(/(?:^|\s)--test\s+(\S+)/)?.[1];
    if (selected !== undefined) assert(targetNames.has(selected), `${name}: unknown native law target ${selected}`);
  }
  const artifact = parse(read("../../🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/📦️packages/🦀️rust/Cargo.toml")) as { package: { name: string }; lib: { name?: string } };
  for (const group of compositionLawGroups().filter(group => group.target.kind === "lib")) {
    assert.equal(group.target.name, artifact.lib.name ?? artifact.package.name.replaceAll("-", "_"));
  }
}
