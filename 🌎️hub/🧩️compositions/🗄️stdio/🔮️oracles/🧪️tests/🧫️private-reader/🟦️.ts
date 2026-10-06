/** 🧾️ Proves exact private reader ownership against the original caller and macro inputs. */
import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { lstatSync, readFileSync, mkdirSync, mkdtempSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, resolve, relative, sep } from "node:path";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🧫️private-reader/🔣️.json";

import composition from "../../🧫️fixtures/🧩️composition/🔣️.json";
import { assertPrivateReaderInput, assertPrivateReaderBindings } from "./🧩️preservation/🟦️.ts";
import { inspectRustCompileReferences } from "../../../../../../🧰️framework/🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { validateJsonSchemaSubset } from "../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../..");
const read = (path: string) => readFileSync(resolve(root, path), "utf8");
const digest = (source: string) => createHash("sha256").update(source).digest("hex");
const witness = fixture.cases[0]!;
const row = composition.callers.find(({ source }) => source === witness.source)!;
const frozen = (source: string) => row.rewrites.reduce((text, rewrite) => text.replaceAll(rewrite.current, rewrite.previous), source);
const ports = { read, assertFile: (path: string) => {
  let file = resolve(root, path);
  expect(lstatSync(file).isFile()).toBe(true);
  while (file !== root) {
    expect(lstatSync(file).isSymbolicLink(), path).toBe(false);
    file = dirname(file);
  }
} };

test("the closed witness agrees with the independent schema reference", () => {
  
  expect(fixture.schemaVersion).toBe(1);
  expect(fixture.cases.map(({ id }) => id)).toEqual(["home-editor-transient-vectors"]);
  expect(witness.inputs).toHaveLength(15);
});

test("the original three macro rows and all fifteen include targets are unchanged", () => {
  const originalFields = [...witness.originalRegion.matchAll(/([a-z]+): include_str!\(concat!\("([^"]+)", \$name, "([^"]+)"\)\)/gu)];
  const originalVectors = [...witness.originalRegion.matchAll(/"([^"]+)" => committed!\("([^"]+)", (true|false)\)/gu)];
  expect(originalFields.map((match) => match[1])).toEqual(["before", "mutation", "after", "diff", "outcome"]);
  expect(originalVectors.map((match) => ({ id: match[1], observable: match[3] === "true" }))).toEqual(witness.vectors);
  const targets = originalVectors.flatMap((vector) => originalFields.map((field) => relative(root, resolve(root, dirname(witness.source), field[2]! + vector[2]! + field[3]!)).split(sep).join("/")));
  expect(targets).toEqual(witness.inputs.map(({ path }) => path));
  const helper = read(witness.helper);
  expect(digest(helper)).toBe(witness.helperSha256);
  expect(inspectRustCompileReferences(helper).map(({ kind, path }) => ({ kind, path }))).toEqual(witness.inputs.map(({ include: path }) => ({ kind: "include_str", path })));
  for (const input of witness.inputs) {
    ports.assertFile(input.path);
    expect(digest(read(input.path))).toBe(input.currentSha256);
    expect(() => JSON.parse(read(input.path))).not.toThrow();
  }
  expect(digest(frozen(witness.originalSource))).toBe(row.sha256);
  expect(witness.frozenCallerSha256).toBe(row.sha256);
});

test("the private reader reconstructs every original frozen caller byte", () => {
  const original = assertPrivateReaderBindings(witness.source, read(witness.source), ports);
  expect(original).toBe(read(witness.source));
  expect(inspectRustCompileReferences(original).some(({ path }) => path.endsWith("🧫️fixtures/🦀️.rs"))).toBe(true);
  expect(assertPrivateReaderBindings("unrelated-source.rs", "unchanged", ports)).toBe("unchanged");
});

test("changed reader bindings, flags, assets, and physical identity refuse restoration", () => {
  const source = read(witness.source), helper = read(witness.helper);
  const brokenSources = [source.replace(witness.currentRegion, ""), source.replace("🧫️fixtures/🦀️.rs", "🧫️elsewhere/🦀️.rs"), source.replace(witness.importAnchor, ""), source + "\n" + witness.currentRegion, source.replace(witness.importAnchor, "")];
  for (const broken of brokenSources) expect(() => assertPrivateReaderBindings(witness.source, broken, ports)).toThrow();
  for (const replacement of [helper.replace("observable: true", "observable: false"), helper.replace("no committed vector", "accepted missing vector"), helper.replace("✅️apply", "🟰️apply")]) {
    expect(() => assertPrivateReaderBindings(witness.source, source, { ...ports, read: (path) => path === witness.helper ? replacement : read(path) })).toThrow();
  }
  const first = witness.inputs[0]!.path;
  expect(() => assertPrivateReaderBindings(witness.source, source, { ...ports, read: (path) => path === first ? "{}" : read(path) })).toThrow();
  expect(() => assertPrivateReaderBindings(witness.source, source, { ...ports, assertFile: () => { throw new Error("linked input"); } })).toThrow();
  expect(() => assertPrivateReaderBindings(witness.source, source, { ...ports, assertFile: (path) => { if (path === witness.source) throw new Error("linked caller"); ports.assertFile(path); } })).toThrow();
});

 test("real physical inputs reject linked ancestors, directories, absence, and escapes", () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");
  mkdirSync(output, { recursive: true });
  const workspace = mkdtempSync(resolve(output, "private-reader-"));
  try {
    const owned = resolve(workspace, "owned");
    mkdirSync(owned);
    writeFileSync(resolve(owned, "fixture.json"), "{}");
    symlinkSync(owned, resolve(workspace, "linked"), process.platform === "win32" ? "junction" : "dir");
    expect(() => assertPrivateReaderInput("owned/fixture.json", workspace)).not.toThrow();
    for (const path of ["linked/fixture.json", "owned", "absent.json", "../outside.json"]) {
      expect(() => assertPrivateReaderInput(path, workspace)).toThrow();
    }
  } finally {
    rmSync(workspace, { recursive: true, force: true });
  }
});
