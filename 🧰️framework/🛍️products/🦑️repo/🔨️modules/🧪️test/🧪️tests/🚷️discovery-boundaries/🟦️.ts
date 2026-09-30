import { expect, test } from "bun:test";
import { lstatSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join, resolve } from "node:path";
import { pathToFileURL } from "node:url";
import Ajv from "ajv";
import glob from "fast-glob";

type Vector = Readonly<{ schemaVersion: number; features: readonly string[]; links: readonly { path: string; target: string; file?: boolean }[]; expected: readonly string[] }>;
const owner = resolve(import.meta.dir, "../.."), repo = resolve(owner, "../../../../..");
const fixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🚷️discovery-boundaries/🔣️.json"), "utf8")) as Vector;
const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🚷️discovery-boundaries/🔣️.json"), "utf8"));
const taxonomyPath = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";
const vocabulary = JSON.parse(readFileSync(join(repo, taxonomyPath), "utf8"));
const plugin = await import(pathToFileURL(join(owner, "🟨️.mjs")).href) as { discoverCaseDirs: (root: string) => string[] };
const kind = vocabulary.fileKinds[vocabulary.testFeatureFileKindId];
const feature = `${kind.emoji}${kind.extensionChains[0]}`;

test("portable no-follow discovery fixtures satisfy their schema", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
});

test("Nx test discovery rejects loop, case and feature links and skips generated/opaque trees", () => {
  const output = resolve(process.env.SEMIO_TEST_ARTIFACT_DIR || tmpdir());
  mkdirSync(output, { recursive: true });
  const root = mkdtempSync(join(output, "case-discovery-"));
  try {
    mkdirSync(dirname(join(root, taxonomyPath)), { recursive: true });
    writeFileSync(join(root, taxonomyPath), JSON.stringify(vocabulary));
    for (const path of fixture.features) {
      mkdirSync(join(root, path), { recursive: true });
      writeFileSync(join(root, path, feature), "Feature: Fixture\n");
    }
    for (const link of fixture.links) {
      mkdirSync(dirname(join(root, link.path)), { recursive: true });
      const junction = process.platform === "win32";
      const target = junction && link.file ? dirname(join(root, link.target)) : join(root, link.target);
      symlinkSync(target, join(root, link.path), junction ? "junction" : link.file ? "file" : "dir");
    }
    const ignored = [
      ...vocabulary.implementationLeafPolicy.ignoredPathPatterns.map((pattern: string) => `${pattern}/**`),
      ...vocabulary.pathEmojiPolicy.reservedSubtreeDirectoryNames.map((segment: string) => `**/${segment}/**`),
      ...Object.values(vocabulary.pathExclusions).map((entry) => `${(entry as { path: string }).path}**`),
    ];
    const oracle = glob.sync(`**/${vocabulary.testsDirName}/*/${feature}`, { cwd: root, dot: true, followSymbolicLinks: false, ignore: ignored }).filter((path) => !lstatSync(join(root, path)).isSymbolicLink()).map((path) => dirname(path).replaceAll("\\", "/")).sort();
    expect(oracle).toEqual([...fixture.expected]);
    expect(plugin.discoverCaseDirs(root)).toEqual(oracle);
  } finally { rmSync(root, { recursive: true, force: true }); }
});
