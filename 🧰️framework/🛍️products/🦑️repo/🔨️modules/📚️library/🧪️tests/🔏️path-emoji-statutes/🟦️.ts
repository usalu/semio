import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { join } from "node:path";
import Ajv from "ajv";
import emojiRegex from "emoji-regex";
import { fixedFilenameContractIdsForPath, leadingEmojiIdentity, loadCatalogTaxonomy, pathEmojiStatuteFindings, semanticDirectoryKindId, validateTaxonomy } from "../../🔍️discovery/🟦️.ts";

const root = import.meta.dir;
const fixture = JSON.parse(readFileSync(join(root, "🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(root, "🧬️schema/🔣️.json"), "utf8"));

test("path emoji statutes share a language-neutral contract", () => {
  const validate = new Ajv({ strict: true }).compile(schema);
  expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
  for (const scenario of fixture.cases) {
    expect(pathEmojiStatuteFindings(scenario.entries, fixture.genericEmojiIdentities)).toEqual(scenario.expected);
  }
});

test("leading emoji identities agree with the independent Unicode emoji oracle", () => {
  for (const scenario of fixture.cases) for (const entry of scenario.entries) {
    const name = entry.path.split("/").at(-1)!;
    const oracle = emojiRegex().exec(name);
    const observed = leadingEmojiIdentity(name).emoji;
    expect(Boolean(observed)).toBe(Boolean(oracle?.index === 0));
    if (oracle?.index === 0) expect(observed.startsWith(oracle[0]) || oracle[0].startsWith(observed)).toBe(true);
  }
});

test("a sibling discriminator preserves the directory role encoded by the first emoji", () => {
  expect(semanticDirectoryKindId("🎮️🔎️commands")).toBe("commands");
  expect(semanticDirectoryKindId("📦️📦️packages")).toBe("packages");
  expect(semanticDirectoryKindId("🧪️🧪️🏔️🦋️tests")).toBe("tests");
  expect(semanticDirectoryKindId("🧪️✅️tests")).toBe("tests");
});

test("external package basenames remain reserved only in their declared ecosystem scope", () => {
  const taxonomy = loadCatalogTaxonomy();
  expect(validateTaxonomy(taxonomy)).toEqual([]);
  const context = { packageRoot: true, ecosystemId: "🟦️typescript" } as const;
  expect(fixedFilenameContractIdsForPath("owner/README.md", taxonomy, context)).toEqual(["bun-package-readme"]);
  expect(fixedFilenameContractIdsForPath("owner/LICENSE.md", taxonomy, context)).toEqual(["bun-package-license"]);
  expect(fixedFilenameContractIdsForPath("owner/README.md", taxonomy)).toEqual([]);
});
