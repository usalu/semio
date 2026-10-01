/** 🪪️ Portable installation admission and sibling collision laws with independent schema and emoji oracles. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import emojiRegex from "emoji-regex";
import { parseInstallationDirectoryV1, installationDirectoryEmoji, installationDirectoryCollision } from "../🟦️.ts";

const owner = resolve(import.meta.dir, "..");
const corpus = JSON.parse(readFileSync(resolve(owner, "🧫️fixtures/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(resolve(owner, "🧬️schema/🔣️.json"), "utf8"));
const ajv = new Ajv({ strict: true }).addSchema(schema);
const validate = ajv.getSchema(schema.$id)!;

function actual(): unknown {
  return { directories: corpus.directories.map((row: { value: unknown }) => {
    try { const value = parseInstallationDirectoryV1(row.value); return { accepted: true, value, emoji: installationDirectoryEmoji(value) }; }
    catch { return { accepted: false }; }
  }), collisions: corpus.collisions.map((row: { directoryName: unknown; siblings: string[] }) => installationDirectoryCollision(parseInstallationDirectoryV1(row.directoryName), row.siblings) ?? null) };
}

test("the closed corpus covers codepoint bounds, graphemes, NFC, metacharacters and sibling files", () => {
  expect(ajv.compile(JSON.parse(readFileSync(resolve(owner, "🧫️fixtures/🧬️schema/🔣️.json"), "utf8")))(corpus)).toBe(true);
  for (const row of corpus.directories) {
    const accepted = validate(row.value) && row.value === row.value.normalize("NFC");
    expect(Boolean(accepted), row.name).toBe(row.accepted);
    if (accepted) {
      expect(parseInstallationDirectoryV1(row.value)).toBe(row.value);
      const emoji = [...row.value.replaceAll("\uFE0F", "").matchAll(emojiRegex())][0]?.[0];
      expect(installationDirectoryEmoji(row.value), row.name).toBe(emoji);
      expect(emoji).toBe(row.emoji);
    } else expect(() => parseInstallationDirectoryV1(row.value), row.name).toThrow();
  }
  for (const row of corpus.collisions) {
    const name = parseInstallationDirectoryV1(row.directoryName);
    const emoji = installationDirectoryEmoji(name);
    const oracle = row.siblings.find((value: string) => { const match = emojiRegex().exec(value.normalize("NFC").replaceAll("\uFE0F", "").replaceAll("\uFE0E", "")); return match?.index === 0 && match[0] === emoji; }) ?? null;
    expect(installationDirectoryCollision(name, row.siblings) ?? null).toBe(row.conflict);
    expect(oracle).toBe(row.conflict);
  }
});

test("native Node and independent esbuild execute the exact first-party entrypoint and corpus", async () => {
  const expected = actual();
  const executable = `import { parseInstallationDirectoryV1, installationDirectoryEmoji, installationDirectoryCollision } from ${JSON.stringify(pathToFileURL(resolve(owner, "🟦️.ts")).href)}; const corpus=${JSON.stringify(corpus)}; ${actual.toString()} process.stdout.write(JSON.stringify(actual()));`;
  const native = Bun.spawnSync(["node", "--experimental-strip-types", "--input-type=module", "--eval", executable], { stdout: "pipe", stderr: "pipe" });
  expect(native.exitCode, Buffer.from(native.stderr).toString()).toBe(0);
  expect(JSON.parse(Buffer.from(native.stdout).toString())).toEqual(expected);
  const { build } = await import("esbuild");
  const output = await build({ stdin: { contents: executable.replace(pathToFileURL(resolve(owner, "🟦️.ts")).href, resolve(owner, "🟦️.ts")), resolveDir: owner, loader: "ts" }, bundle: true, platform: "node", format: "esm", write: false });
  const child = Bun.spawnSync(["node", "--input-type=module", "--eval", output.outputFiles![0]!.text], { stdout: "pipe", stderr: "pipe" });
  expect(child.exitCode, Buffer.from(child.stderr).toString()).toBe(0);
  expect(JSON.parse(Buffer.from(child.stdout).toString())).toEqual(expected);
});
