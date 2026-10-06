import { expect, test } from "bun:test";
import { cpSync, existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync } from "node:fs";
import { join, resolve } from "node:path";

import emojiRegex from "emoji-regex";
import ts from "typescript";
import { leadingEmojiIdentity } from "../🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json" with { type: "json" };

test("neutral grapheme vectors preserve exact leading emoji, first grapheme and original rest", () => {
  expect(new Set(fixture.cases.map((row) => row.id)).size).toBe(fixture.cases.length);
  for (const row of fixture.cases) {
    expect(leadingEmojiIdentity(row.value), row.id).toEqual(row.expected);
    let prefix = "";
    for (const match of row.value.matchAll(emojiRegex())) {
      if (match.index !== prefix.length) break;
      prefix += match[0];
    }
    expect(prefix, row.id).toBe(row.oracleEmoji);
  }
});

test("independent TypeScript runtime keeps the neutral implementation", () => {
  const source = readFileSync(resolve(import.meta.dir, "../🟦️.ts"), "utf8");
  const javascript = ts.transpileModule(source, { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 } }).outputText;
  const exports: { leadingEmojiIdentity?: typeof leadingEmojiIdentity } = {};
  new Function("exports", javascript)(exports);
  for (const row of fixture.cases) expect(exports.leadingEmojiIdentity!(row.value), row.id).toEqual(row.expected);

});


test("the complete grapheme implementation and portable laws run with every product absent", () => {
  const root = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!root) throw Error("Grapheme removal laws require caller-owned SEMIO_TEST_ARTIFACT_DIR");
  mkdirSync(root, { recursive: true });
  const sandbox = mkdtempSync(join(root, "grapheme-removal-"));
  try {
    const owner = join(sandbox, fixture.removal.owner);
    mkdirSync(resolve(owner, ".."), { recursive: true });
    cpSync(resolve(import.meta.dir, ".."), owner, { recursive: true });
    symlinkSync(resolve(import.meta.dir, "../../../../../node_modules"), join(sandbox, "node_modules"), "junction");
    expect(existsSync(join(sandbox, "🧰️framework/🛍️products"))).toBe(false);
    const observed = Bun.spawnSync([process.execPath, "test", join(owner, "🧪️tests/🟦️.ts"), "--test-name-pattern", fixture.removal.selectedLaws.map(name => "^" + name + "$").join("|")], { cwd: sandbox, stdout: "pipe", stderr: "pipe" });
    expect(observed.exitCode, Buffer.from(observed.stderr).toString()).toBe(0);
    expect(Buffer.from(observed.stderr).toString()).toContain("2 pass");
  } finally { rmSync(sandbox, { recursive: true, force: true }); }
}, 15_000);
