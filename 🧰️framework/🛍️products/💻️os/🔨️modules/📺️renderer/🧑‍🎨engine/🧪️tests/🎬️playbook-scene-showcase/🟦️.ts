import { readFileSync } from "node:fs";
import { dirname, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import Ajv2020 from "ajv/dist/2020.js";
import { describe, expect, it } from "vitest";

type WindowFixture = { readonly windowId: string; readonly bodyKey: string; readonly surfaceKind: "table" | "diffView" | "eventFeed" | "textEditor" };

const suiteRoot = dirname(fileURLToPath(import.meta.url));
const repoRoot = resolve(suiteRoot, "../../../../../../../..");
const taxonomy = "✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/🏗️builder";
const fixture = JSON.parse(readFileSync(resolve(repoRoot, taxonomy, "🧫️fixtures/🎬️scene-showcase/🔣️.json"), "utf8")) as { readonly windows: readonly WindowFixture[] };
const schema = JSON.parse(readFileSync(resolve(repoRoot, taxonomy, "🧬️schema/🎬️scene-showcase/🔣️.json"), "utf8"));

describe("registered Playbook scene showcase", () => {
  it("validates the authored route corpus with the independent JSON Schema oracle", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.windows.map(({ surfaceKind }) => surfaceKind)).toEqual(["table", "diffView", "eventFeed", "textEditor"]);
    expect(new Set(fixture.windows.map(({ windowId }) => windowId)).size).toBe(fixture.windows.length);
  });
});
