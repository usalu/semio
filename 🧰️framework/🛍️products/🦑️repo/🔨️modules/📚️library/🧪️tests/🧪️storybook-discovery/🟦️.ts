import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative, resolve, sep } from "node:path";

import { describe, expect, test } from "bun:test";
import Ajv2020 from "ajv/dist/2020.js";
import fg from "fast-glob";
import { loadCsf } from "storybook/internal/csf-tools";

import { buildScopeStoryGlobs, resolveActiveScopes, STORY_SCOPES } from "../../../../../../../.storybook/📖️stories/🧭️coordination/🟦️.ts";

type StoryIdentity = Readonly<{
  path: string;
  title: string;
  stories: readonly Readonly<{ exportName: string; id: string; name: string }>[];
}>;
type Fixture = Readonly<{ version: 1; stories: readonly StoryIdentity[] }>;

const repoRoot = resolve(import.meta.dir, "../../../../../../..");
const fixturePath = resolve(import.meta.dir, "../../🧫️fixtures/🧫️storybook-discovery/🔣️.json");
const schemaPath = resolve(import.meta.dir, "../../🧬️schema/🔣️storybook-discovery/🔣️.json");
const fixture = JSON.parse(readFileSync(fixturePath, "utf8")) as Fixture;
const schema = JSON.parse(readFileSync(schemaPath, "utf8"));
const normalize = (path: string): string => path.split(sep).join("/");

describe("Storybook discovery identity", () => {
  test("the portable authority has the closed schema", () => {
    const validate = new Ajv2020({ allErrors: true, strict: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(new Set(fixture.stories.map((row) => row.path)).size).toBe(fixture.stories.length);
  });

  test("scope globs discover every and only canonical story leaf", async () => {
    const storybookRoot = join(repoRoot, ".storybook");
    const matches = await fg(buildScopeStoryGlobs(resolveActiveScopes("")), { cwd: storybookRoot, onlyFiles: true, unique: true });
    const actual = matches.map((path) => normalize(relative(repoRoot, resolve(storybookRoot, path)))).sort();
    const expected = fixture.stories.map((row) => row.path).sort();
    expect(actual).toEqual(expected);
    expect(actual.every((path) => path.endsWith("/🧪️.story.tsx"))).toBe(true);
  });

  test("every configured source root and filesystem alias resolves", () => {
    for (const scope of STORY_SCOPES) {
      for (const path of scope.sourceRoots) expect(existsSync(join(repoRoot, path)), `${scope.id} source root ${path}`).toBe(true);
      for (const path of Object.values(scope.aliases ?? {})) expect(existsSync(join(repoRoot, path)), `${scope.id} alias ${path}`).toBe(true);
    }
  });

  test("the native Storybook parser preserves title, export and derived id identity", () => {
    for (const expected of fixture.stories) {
      const source = readFileSync(join(repoRoot, expected.path), "utf8");
      const parsed = loadCsf(source, { fileName: expected.path, makeTitle: (title) => title }).parse();
      const stories = Object.entries(parsed._stories).map(([exportName, story]) => ({ exportName, id: story.id, name: story.name }));
      expect(parsed.meta?.title, expected.path).toBe(expected.title);
      expect(stories, expected.path).toEqual(expected.stories);
    }
  });

  test("inactive Coda provenance cannot enter the executable story graph", () => {
    expect(() => resolveActiveScopes("coda")).toThrow("unknown scope");
    expect(fixture.stories.some((row) => row.path.includes("coda") || readFileSync(join(repoRoot, row.path), "utf8").includes("@semio-tech/coda-desktop"))).toBe(false);
  });
});
