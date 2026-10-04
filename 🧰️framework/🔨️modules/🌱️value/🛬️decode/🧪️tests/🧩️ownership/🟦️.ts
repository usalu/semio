/** 🧭️ Proves neutral discovery and exact preservation of the existing decode laws. */
import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import * as toml from "@iarna/toml";
import Ajv from "ajv";
import contract from "../../🧫️fixtures/🧩️ownership/🔣️.json";
import schema from "../../🧬️schema/🧩️ownership/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../..");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const digest = (source: string) => createHash("sha256").update(source).digest("hex");

test("the closed ownership witness agrees with independent schema admission", () => {
  const validate = new Ajv({ strict: true, allErrors: true }).compile(schema);
  const mutants: unknown[] = [contract, { ...contract, unknown: true }, { ...contract, schemaVersion: 2 }, { ...contract, nativeLaws: [] }];
  for (const key of ["originalNative", "originalControl", "ownedImport", "nativeMount", "unchangedInputs", "packageManifest", "referenceBindings"]) {
    const broken: Record<string, unknown> = { ...contract };
    delete broken[key];
    mutants.push(broken);
  }
  mutants.push({ ...contract, packageSource: "" }, { ...contract, packageManifest: "" }, { ...contract, nativeLaws: [contract.nativeLaws[0], contract.nativeLaws[0]] }, { ...contract, nativeLaws: [...contract.nativeLaws, "foreign_law"] }, { ...contract, unchangedInputs: [] }, { ...contract, unchangedInputs: contract.unchangedInputs.map((row, index) => index === 0 ? { ...row, unknown: true } : row) }, { ...contract, unchangedInputs: contract.unchangedInputs.map((row, index) => index === 0 ? { ...row, sha256: "invalid" } : row) });
  mutants.push({ ...contract, referenceBindings: [] }, { ...contract, referenceBindings: contract.referenceBindings.map((row, index) => index === 0 ? { ...row, unknown: true } : row) });
  for (const value of mutants) {
    expect(validate(value)).toBe(value === contract);
    expect(validateJsonSchemaSubset(schema, value).length === 0).toBe(value === contract);
  }
});

test("captured default value roots declare both neutral native laws", () => {
  const packageSource = read(contract.packageSource), valueSource = read(contract.valueSource);
  expect(digest(packageSource)).toBe(contract.packageSha256);
  expect(digest(valueSource)).toBe(contract.valueSha256);
  expect(packageSource.split('#[path = "../../🦀️.rs"]\npub mod value;').length).toBe(2);
  expect(valueSource.split('#[path = "🛬️decode/🦀️.rs"]\npub mod native_decoding;').length).toBe(2);
  const control = read(contract.controlSource);
  expect(control.split(contract.nativeMount).length).toBe(2);
  const native = read(contract.nativeSource);
  expect(native).toContain(contract.ownedImport);
  expect(native).not.toContain("semio_framework_os_kernel");
  expect([...native.matchAll(/^fn ([a-z_]+)\(/gmu)].map((match) => match[1])).toEqual(contract.nativeLaws);
});

test("every original control and law byte survives only its declared ownership edits", () => {
  expect(read(contract.controlSource).replace(contract.nativeMount, "")).toBe(contract.originalControl);
  expect(read(contract.nativeSource).replace(contract.ownedImport, contract.originalImport)).toBe(contract.originalNative);
  for (const input of contract.unchangedInputs) expect(digest(read(input.path)), input.path).toBe(input.sha256);
  const script = read(contract.packageScript);
  const portable = script.slice(script.indexOf("class TestScript"), script.indexOf("class ControlledValueTestScript"));
  expect(portable).toContain('resolve(this.root, "../../🔁️codec/🧪️tests/🛬️controlled/🟦️.ts")');
  expect(portable).toContain('resolve(this.root, "../../🛬️decode/🧪️tests/🟦️.ts")');
});


test("the actual Cargo library root admits the captured neutral native law tree", () => {
  const path = contract.packageManifest;
  expect(typeof path).toBe("string");
  if (!path) throw Error("packageManifest is required");
  const source = read(path), native = Bun.TOML.parse(source);
  expect(native).toEqual(toml.parse(source));
  const manifest = native as {package: {name: string}; lib: {name: string; path: string}};
  expect(manifest.package.name).toBe(contract.package);
  expect(manifest.lib.name).toBe("semio_framework_value");
  expect(resolve(root, dirname(path), manifest.lib.path)).toBe(resolve(root, contract.packageSource));
});

test("independent SQLite references retain every original byte except explicit owned binding arrays", () => {
  for (const row of contract.referenceBindings) {
    const source = read(row.path);
    expect(source.split(row.ownedCall).length).toBe(2);
    expect(source.replace(row.ownedCall, row.originalCall)).toBe(row.originalSource);
  }
});
