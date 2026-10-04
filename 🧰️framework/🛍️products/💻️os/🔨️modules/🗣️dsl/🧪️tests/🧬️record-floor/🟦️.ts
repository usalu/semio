/** 🧬️ Preserves complete JSON caller and native-law sources across canonical ownership changes. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { inspectRustRunnableTests } from "../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import { rustTokens } from "../../../../../../🔨️modules/📚️compiler/📖️syntax/🦀️rust/🟦️.ts";
import { applyJsonSourceEdits, jsonConcurrentCuts, jsonGenericCuts, jsonMountCuts, jsonPathCuts, jsonPolicyCuts, jsonSourceHash, undoJsonOwnershipCuts } from "./📑️receipt/🟦️.ts";

type Input = Readonly<{path: string; source: string; sha256: string; bytes: number}>;
const root = resolve(import.meta.dir, "../../../../../../.."), fixture = resolve(root, ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🔬️record-floor-contract/🧫️fixtures");
const inputs = (JSON.parse(readFileSync(resolve(fixture, "🧬️json-cut-before.json"), "utf8")) as {rows: readonly Input[]}).rows;
const byPath = new Map(inputs.map(row => [row.path, row]));
const read = (path: string): string => readFileSync(resolve(root, path), "utf8");

test("every JSON client source reconstructs its full captured bytes through owned cuts and separately observed concurrent deltas", () => {
  const selected = new Set([...jsonPathCuts, ...jsonPolicyCuts, ...jsonMountCuts, ...jsonGenericCuts].map(row => row.path));
  expect(jsonPathCuts).toHaveLength(2113);
  expect(jsonPolicyCuts).toHaveLength(1618);
  for (const path of selected) {
    const current = read(path), original = undoJsonOwnershipCuts(path, current), captured = byPath.get(path);
    if (captured) { expect(original, path).toBe(captured.source); expect(jsonSourceHash(original), path).toBe(captured.sha256); }
    else { expect([...jsonMountCuts, ...jsonGenericCuts].some(row => row.path === path && row.beforeSource === original), path).toBe(true); }
  }
});

test("every original JSON native law retains its full source and explicit normalization selection", () => {
  const path = "🧰️framework/🔨️modules/🎒️pack/🔤️json/🧪️tests/🔬️unit/🦀️.rs", current = read(path), original = undoJsonOwnershipCuts(path, current), captured = byPath.get(path)!;
  expect(original).toBe(captured.source);
  expect(jsonSourceHash(original)).toBe(captured.sha256);
  const sourceLaws = inspectRustRunnableTests(original), currentLaws = inspectRustRunnableTests(current);
  expect(currentLaws.runnableTests.map(row => row.name)).toEqual(sourceLaws.runnableTests.map(row => row.name));
  expect(sourceLaws.runnableTests).toHaveLength(44);
  const cut = jsonPolicyCuts.find(row => row.path === path)!;
  expect(cut.edits.length).toBeGreaterThan(0);
  for (const edit of cut.edits) expect(edit.to).toBe(", semio_framework_pack_json::JsonMemberPolicy::Replace");
  for (const edit of jsonGenericCuts.find(row => row.path === path)!.policyEdits) expect(edit.to).toBe(", semio_framework_pack_json::JsonMemberPolicy::Replace");
});

test("all other admitted reader argument bindings explicitly reject decoded duplicate members", () => {
  for (const cut of jsonPolicyCuts) {
    if (cut.path.endsWith("🎒️pack/🔤️json/🧪️tests/🔬️unit/🦀️.rs")) continue;
    expect(cut.beforeSource, cut.path).toBeDefined();
    expect(jsonSourceHash(applyJsonSourceEdits(cut.beforeSource!, cut.edits)), cut.path).toBe(cut.afterSha256);
    for (const edit of cut.edits) if (edit.from === "") expect(edit.to, cut.path).toBe(", semio_framework_pack_json::JsonMemberPolicy::Reject");
  }
  for (const cut of jsonGenericCuts) {
    if (cut.path.endsWith("🎒️pack/🔤️json/🧪️tests/🔬️unit/🦀️.rs")) continue;
    const namespace = applyJsonSourceEdits(cut.beforeSource, cut.namespaceEdits);
    expect(jsonSourceHash(namespace), cut.path).toBe(cut.namespaceAfterSha256);
    expect(jsonSourceHash(applyJsonSourceEdits(namespace, cut.policyEdits)), cut.path).toBe(cut.afterSha256);
    for (const edit of cut.policyEdits) expect(edit.to, cut.path).toBe(", semio_framework_pack_json::JsonMemberPolicy::Reject");
  }
  expect(jsonConcurrentCuts.length).toBe(25);
});

test("Pack and OS no longer mount or forward the actual JSON primitive owner", () => {
  for (const path of ["🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust/🦀️.rs", "🧰️framework/🔨️modules/🎒️pack/🦀️.rs", "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs"]) {
    const tokens = rustTokens(read(path)).map(token => token.text);
    expect(tokens.join(" "), path).not.toContain("pub mod json");
    expect(tokens.join(" "), path).not.toContain("pub use pack :: json");
    expect(tokens.join(" "), path).not.toContain("pub use crate :: json");
  }
});
